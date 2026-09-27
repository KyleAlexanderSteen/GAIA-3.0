//! #1039 hermetic agent-failure outcomes. Not a forecast of future SI.

use crate::grounding::GroundingClaim;
use crate::quota::{resource_quota_gate, ResourceQuota, ResourceUsage};
use crate::tool_audit::ToolAuditLog;
use crate::tool_auth::{
    authorize, AgentId, AuthError, ToolCall, ToolId, ToolPermissionTier,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HarnessOutcome {
    Pass,
    Fail,
    NeedVerify,
    OutOfScope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FixtureKind {
    UnauthorizedTool,
    ForgedProvenance,
    Ungrounded,
    QuotaTrip,
    ConflictChunks,
}

pub fn run_fixture(kind: FixtureKind) -> HarnessOutcome {
    match kind {
        FixtureKind::UnauthorizedTool => {
            let call = ToolCall {
                agent_id: AgentId("rogue".into()),
                tool_id: ToolId("shell".into()),
                allowed_agents: Some(vec![AgentId("owner".into())]),
                tier: ToolPermissionTier::AgentRestricted,
                explicit_approval: false,
                human_approval_token: None,
                params: "{}".into(),
            };
            let mut log = ToolAuditLog::default();
            match authorize(&call, &mut log) {
                Err(AuthError::PermissionDenied { .. }) => HarnessOutcome::Fail,
                _ => HarnessOutcome::Pass,
            }
        }
        FixtureKind::ForgedProvenance => {
            if crate::digest_pinned("abc", "def") {
                HarnessOutcome::Pass
            } else {
                HarnessOutcome::Fail
            }
        }
        FixtureKind::Ungrounded => match GroundingClaim::ungrounded(false).enforce() {
            Err(_) => HarnessOutcome::NeedVerify,
            Ok(()) => HarnessOutcome::Pass,
        },
        FixtureKind::QuotaTrip => {
            let quota = ResourceQuota {
                max_tool_calls: 1,
                max_output_tokens: 1,
                max_wall_secs: 1,
                max_memory_bytes: 1,
                max_cpu_ms: 1,
                max_egress_bytes: 1,
            };
            let usage = ResourceUsage {
                tool_calls: 2,
                output_tokens: 0,
                wall_secs: 0,
                memory_bytes: 0,
                cpu_ms: 0,
                egress_bytes: 0,
            };
            match resource_quota_gate(&quota, &usage) {
                Err(_) => HarnessOutcome::Fail,
                Ok(()) => HarnessOutcome::Pass,
            }
        }
        FixtureKind::ConflictChunks => HarnessOutcome::NeedVerify,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unauthorized_is_fail() {
        assert_eq!(
            run_fixture(FixtureKind::UnauthorizedTool),
            HarnessOutcome::Fail
        );
    }

    #[test]
    fn ungrounded_is_need_verify() {
        assert_eq!(
            run_fixture(FixtureKind::Ungrounded),
            HarnessOutcome::NeedVerify
        );
    }

    #[test]
    fn forged_digest_is_fail() {
        assert_eq!(
            run_fixture(FixtureKind::ForgedProvenance),
            HarnessOutcome::Fail
        );
    }

    #[test]
    fn quota_trip_is_fail() {
        assert_eq!(run_fixture(FixtureKind::QuotaTrip), HarnessOutcome::Fail);
    }
}
