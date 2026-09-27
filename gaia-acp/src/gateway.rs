use crate::adapter::{FakeAdapter, RecordingAdapter};
use crate::approval::HumanApprovalReceipt;
use crate::audit::{ActionReceipt, AuditChain, AuditPushInput, PlaneEvent, PlaneState};
use crate::autonomy::{gate, AutonomyLevel};
use crate::grounding::GroundingClaim;
use crate::manifest::{CapabilityManifest, RevocationList};
use crate::policy::{PolicyDecision, PolicyEngine, PolicyEvaluationContext};
use crate::trace::{
    from_invoke, ClaimClass, InvokeTraceInput, MemoryTraceSink, TraceKind, TraceSink,
};
use crate::types::{ProposedAction, ReasonCode, SignedIntent, UntrustedContent};

#[derive(Debug)]
pub struct ControlPlane {
    pub now: u64,
    pub state: PlaneState,
    pub intent: SignedIntent,
    pub revoked: RevocationList,
    pub consumed_approvals: Vec<String>,
    pub emergency_stop: bool,
    pub audit: AuditChain,
    pub traces: MemoryTraceSink,
    pub autonomy: AutonomyLevel,
    deny_streak: u32,
    pending_grounding: Option<GroundingClaim>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvokeResult {
    pub allowed: bool,
    pub executed: bool,
    pub reason: ReasonCode,
    pub receipt: ActionReceipt,
}

impl ControlPlane {
    pub fn start(now: u64, agent_id: &str) -> Result<Self, ReasonCode> {
        let mut plane = Self {
            now,
            state: PlaneState::Unregistered,
            intent: SignedIntent {
                intent_id: format!("intent-{agent_id}"),
                goal_class: "local-research".into(),
                issued_at: now,
                expires_at: now + 3600,
            },
            revoked: RevocationList::default(),
            consumed_approvals: Vec::new(),
            emergency_stop: false,
            audit: AuditChain::default(),
            traces: MemoryTraceSink::default(),
            autonomy: AutonomyLevel::BoundedRemediate,
            deny_streak: 0,
            pending_grounding: None,
        };
        plane.transition(PlaneState::Registered, agent_id)?;
        plane.transition(PlaneState::Verified, agent_id)?;
        plane.transition(PlaneState::SessionStarted, agent_id)?;
        plane.transition(PlaneState::ManifestIssued, agent_id)?;
        Ok(plane)
    }

    fn emit(&mut self, kind: TraceKind, agent_id: &str, reason: ReasonCode, request_hash: &str) {
        let intent_id = self.intent.intent_id.clone();
        let ev = from_invoke(InvokeTraceInput {
            kind,
            ts: self.now,
            actor_id: agent_id,
            intent_id: &intent_id,
            correlation_id: &intent_id,
            reason,
            request_hash,
            claim_class: ClaimClass::Established,
        });
        self.traces.emit(ev);
    }

    pub fn transition(&mut self, next: PlaneState, agent_id: &str) -> Result<(), ReasonCode> {
        if !self.state.can_transition(next) {
            return Err(ReasonCode::StateInvalid);
        }
        self.state = next;
        let ev = match next {
            PlaneState::Registered => PlaneEvent::ServerRegistered,
            PlaneState::Verified => PlaneEvent::ServerVerified,
            PlaneState::SessionStarted => PlaneEvent::SessionStarted,
            PlaneState::ManifestIssued => PlaneEvent::ManifestIssued,
            PlaneState::Stopped => PlaneEvent::ServerStopped,
            PlaneState::Killed => PlaneEvent::ServerKilled,
            PlaneState::Unregistered => PlaneEvent::AnomalyDetected,
        };
        self.audit.push(AuditPushInput {
            event: ev,
            agent_id,
            tool: "-",
            action_class: "lifecycle",
            request_hash: "0",
            reason: ReasonCode::Allow,
            outcome: "state",
        });
        Ok(())
    }

    pub fn revoke(&mut self, id: &str) {
        self.revoked.revoke(id);
        self.audit.push(AuditPushInput {
            event: PlaneEvent::CredentialRevoked,
            agent_id: id,
            tool: "-",
            action_class: "revoke",
            request_hash: "0",
            reason: ReasonCode::Revoked,
            outcome: "revoked",
        });
    }

    pub fn pause(&mut self, agent_id: &str) -> Result<(), ReasonCode> {
        self.transition(PlaneState::Stopped, agent_id)
    }

    pub fn resume(&mut self, agent_id: &str) -> Result<(), ReasonCode> {
        if self.emergency_stop || self.state == PlaneState::Killed {
            return Err(ReasonCode::EmergencyStop);
        }
        self.transition(PlaneState::ManifestIssued, agent_id)
    }

    pub fn kill(&mut self, agent_id: &str) {
        self.emergency_stop = true;
        let _ = self.transition(PlaneState::Killed, agent_id);
        self.audit.push(AuditPushInput {
            event: PlaneEvent::EmergencyStop,
            agent_id,
            tool: "-",
            action_class: "stop",
            request_hash: "0",
            reason: ReasonCode::EmergencyStop,
            outcome: "killed",
        });
        self.emit(TraceKind::Kill, agent_id, ReasonCode::EmergencyStop, "0");
    }

    pub fn invoke(
        &mut self,
        manifest: &mut CapabilityManifest,
        action: &ProposedAction,
        approval: Option<&HumanApprovalReceipt>,
        untrusted: Option<&UntrustedContent>,
    ) -> InvokeResult {
        let mut adapter = RecordingAdapter::default();
        self.invoke_with_adapter(manifest, action, approval, untrusted, &mut adapter)
    }

    pub fn invoke_grounded(
        &mut self,
        manifest: &mut CapabilityManifest,
        action: &ProposedAction,
        approval: Option<&HumanApprovalReceipt>,
        untrusted: Option<&UntrustedContent>,
        claim: GroundingClaim,
    ) -> InvokeResult {
        self.pending_grounding = Some(claim);
        self.invoke(manifest, action, approval, untrusted)
    }

    pub fn invoke_with_adapter<A: FakeAdapter>(
        &mut self,
        manifest: &mut CapabilityManifest,
        action: &ProposedAction,
        approval: Option<&HumanApprovalReceipt>,
        untrusted: Option<&UntrustedContent>,
        adapter: &mut A,
    ) -> InvokeResult {
        self.audit.push(AuditPushInput {
            event: PlaneEvent::ToolProposed,
            agent_id: &action.agent_id,
            tool: &action.tool,
            action_class: &format!("{:?}", action.action_class),
            request_hash: &action.request_hash(),
            reason: ReasonCode::Allow,
            outcome: "proposed",
        });

        let claim = self.pending_grounding.take();

        if !self.state.allows_calls() {
            let receipt = self.deny(action, ReasonCode::StateInvalid);
            return InvokeResult {
                allowed: false,
                executed: false,
                reason: ReasonCode::StateInvalid,
                receipt,
            };
        }

        if let Some(claim) = claim {
            if let Err(reason) = claim.enforce() {
                let receipt = self.deny(action, reason);
                return InvokeResult {
                    allowed: false,
                    executed: false,
                    reason,
                    receipt,
                };
            }
        }

        if let Err(reason) = gate(self.autonomy, action, approval) {
            let receipt = self.deny(action, reason);
            return InvokeResult {
                allowed: false,
                executed: false,
                reason,
                receipt,
            };
        }

        let decision = PolicyEngine::evaluate(PolicyEvaluationContext {
            now: self.now,
            intent: &self.intent,
            manifest,
            action,
            approval,
            revoked: &self.revoked,
            emergency_stop: self.emergency_stop,
            consumed_approvals: &self.consumed_approvals,
            untrusted,
        });
        self.audit.push(AuditPushInput {
            event: PlaneEvent::PolicyEvaluated,
            agent_id: &action.agent_id,
            tool: &action.tool,
            action_class: &format!("{:?}", action.action_class),
            request_hash: &action.request_hash(),
            reason: decision.reason(),
            outcome: "evaluated",
        });

        match decision {
            PolicyDecision::Allow { reason } => {
                if let Some(a) = approval {
                    if a.single_use {
                        self.consumed_approvals.push(a.id.clone());
                    }
                }
                manifest.actions_used = manifest.actions_used.saturating_add(1);
                self.deny_streak = 0;
                let executed = adapter.execute(action).is_ok();
                let action_class_s = format!("{:?}", action.action_class);
                let request_hash_s = action.request_hash();
                let receipt = self.audit.push(AuditPushInput {
                    event: if executed {
                        PlaneEvent::ExecutionCompleted
                    } else {
                        PlaneEvent::ExecutionFailed
                    },
                    agent_id: &action.agent_id,
                    tool: &action.tool,
                    action_class: &action_class_s,
                    request_hash: &request_hash_s,
                    reason,
                    outcome: if executed { "executed" } else { "adapter-fail" },
                });
                self.emit(
                    if executed {
                        TraceKind::Allow
                    } else {
                        TraceKind::ExecutionFailure
                    },
                    &action.agent_id,
                    reason,
                    &action.request_hash(),
                );
                InvokeResult {
                    allowed: true,
                    executed,
                    reason,
                    receipt,
                }
            }
            PolicyDecision::Deny { reason } | PolicyDecision::RequireApproval { reason } => {
                InvokeResult {
                    allowed: false,
                    executed: false,
                    reason,
                    receipt: self.deny(action, reason),
                }
            }
        }
    }

    fn deny(&mut self, action: &ProposedAction, reason: ReasonCode) -> ActionReceipt {
        self.deny_streak += 1;
        if self.deny_streak >= 5 {
            self.audit.push(AuditPushInput {
                event: PlaneEvent::AnomalyDetected,
                agent_id: &action.agent_id,
                tool: &action.tool,
                action_class: "anomaly",
                request_hash: &action.request_hash(),
                reason,
                outcome: "repeated-deny",
            });
        }
        let ev = if matches!(reason, ReasonCode::EgressDenied | ReasonCode::SsrfDenied) {
            PlaneEvent::EgressDenied
        } else {
            PlaneEvent::CallDenied
        };
        let request_hash_s = action.request_hash();
        let action_class_s = format!("{:?}", action.action_class);
        let receipt = self.audit.push(AuditPushInput {
            event: ev,
            agent_id: &action.agent_id,
            tool: &action.tool,
            action_class: &action_class_s,
            request_hash: &request_hash_s,
            reason,
            outcome: "denied",
        });
        let kind = if matches!(reason, ReasonCode::ApprovalReplay) {
            TraceKind::Replay
        } else {
            TraceKind::Deny
        };
        self.emit(kind, &action.agent_id, reason, &action.request_hash());
        receipt
    }
}
