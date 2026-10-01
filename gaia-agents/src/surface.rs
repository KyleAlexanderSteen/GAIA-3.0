//! Jarvis Phase 2 — anticipatory surface layer.
//!
//! Given the current task and a set of candidate resources, rank which ones
//! are worth showing to the human. This layer only **surfaces**. `act` always
//! refuses. The only path into the execute log is `submit_to`, which reads
//! the session halt itself. A caller cannot pass a false halt bit.
//!
//! Scoring is keyword overlap. There is no model behind it.
//! Not yet connected to the Predict-5 honesty layer (#1146).

use std::collections::BTreeSet;

use crate::act::{ActError, ActGate, HumanActReceipt, ProposedAct};
use crate::session::SessionContext;

const STOPWORDS: &[&str] = &[
    "a", "an", "and", "for", "gaia", "in", "into", "of", "on", "or", "the", "to", "with",
];

fn tokens(text: &str) -> BTreeSet<String> {
    text.split(|c: char| !c.is_alphanumeric())
        .filter(|w| w.chars().count() >= 2)
        .map(|w| w.to_lowercase())
        .filter(|w| !STOPWORDS.contains(&w.as_str()))
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceKind {
    Crate,
    Tool,
    Doc,
    Decision,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceCandidate {
    pub id: String,
    pub kind: ResourceKind,
    pub keywords: Vec<String>,
    pub source: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConfidenceBand {
    low: f64,
    point: f64,
    high: f64,
}

impl ConfidenceBand {
    fn from_overlap(score: f64, matched: usize) -> Self {
        let half = 0.5 / (1.0 + matched as f64);
        Self {
            low: (score - half).max(0.0),
            point: score,
            high: (score + half).min(1.0),
        }
    }

    pub fn low(&self) -> f64 {
        self.low
    }

    pub fn point(&self) -> f64 {
        self.point
    }

    pub fn high(&self) -> f64 {
        self.high
    }

    pub fn width(&self) -> f64 {
        self.high - self.low
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SurfaceError {
    SurfaceOnly,
    SessionHalted,
}

impl std::fmt::Display for SurfaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SurfaceOnly => write!(f, "surface events are display-only and cannot be acted on"),
            Self::SessionHalted => write!(f, "session is halted; nothing is surfaced"),
        }
    }
}

impl std::error::Error for SurfaceError {}

#[derive(Debug, Clone, PartialEq)]
pub struct SurfaceEvent {
    resource: String,
    kind: ResourceKind,
    score: f64,
    confidence: ConfidenceBand,
    source_trace: Vec<String>,
}

impl SurfaceEvent {
    pub fn resource(&self) -> &str {
        &self.resource
    }

    pub fn kind(&self) -> ResourceKind {
        self.kind
    }

    pub fn score(&self) -> f64 {
        self.score
    }

    pub fn confidence(&self) -> ConfidenceBand {
        self.confidence
    }

    pub fn source_trace(&self) -> &[String] {
        &self.source_trace
    }

    pub fn act(&self) -> Result<(), SurfaceError> {
        Err(SurfaceError::SurfaceOnly)
    }

    pub fn propose(&self) -> ProposedAct {
        ProposedAct {
            tool: "surface".into(),
            target: self.resource.clone(),
        }
    }

    /// Reads `ctx`. The caller cannot override the halt bit.
    pub fn submit_to(
        &self,
        gate: &mut ActGate,
        ctx: &SessionContext,
        receipt: Option<&HumanActReceipt>,
    ) -> Result<(), ActError> {
        gate.submit(&self.propose(), receipt, ctx.is_halted())
    }
}

pub fn rank_resources(
    task: &str,
    resources: &[ResourceCandidate],
    limit: usize,
) -> Vec<SurfaceEvent> {
    let task_terms = tokens(task);
    if task_terms.is_empty() || limit == 0 {
        return Vec::new();
    }
    let mut events: Vec<SurfaceEvent> = resources
        .iter()
        .filter_map(|r| {
            let mut terms = tokens(&r.id);
            for k in &r.keywords {
                terms.extend(tokens(k));
            }
            let matched: Vec<&String> = task_terms.intersection(&terms).collect();
            if matched.is_empty() {
                return None;
            }
            let score = matched.len() as f64 / task_terms.len() as f64;
            let mut trace: Vec<String> =
                matched.iter().map(|t| format!("matched term: {t}")).collect();
            trace.push(format!("resource source: {}", r.source));
            Some(SurfaceEvent {
                resource: r.id.clone(),
                kind: r.kind,
                score,
                confidence: ConfidenceBand::from_overlap(score, matched.len()),
                source_trace: trace,
            })
        })
        .collect();
    events.sort_by(|a, b| {
        b.score
            .total_cmp(&a.score)
            .then_with(|| a.resource.cmp(&b.resource))
    });
    events.truncate(limit);
    events
}

pub fn surface_for_session(
    ctx: &SessionContext,
    resources: &[ResourceCandidate],
    limit: usize,
) -> Result<Vec<SurfaceEvent>, SurfaceError> {
    if ctx.is_halted() {
        return Err(SurfaceError::SessionHalted);
    }
    Ok(match ctx.task() {
        Some(task) => rank_resources(task, resources, limit),
        None => Vec::new(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::act::{ActDecision, HumanAllowlist};
    use crate::session::{SessionEvent, SessionRecorder};

    fn cand(id: &str, kw: &[&str]) -> ResourceCandidate {
        ResourceCandidate {
            id: id.into(),
            kind: ResourceKind::Crate,
            keywords: kw.iter().map(|s| s.to_string()).collect(),
            source: "test".into(),
        }
    }

    fn sample() -> Vec<ResourceCandidate> {
        vec![
            cand("gaia-sfs", &["semantic", "file"]),
            cand("gaia-orchestrator", &["flow"]),
            cand("gaia-acp", &["flow", "mode"]),
        ]
    }

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    fn gate() -> ActGate {
        ActGate::with_allowlist(HumanAllowlist::developer())
    }

    fn open_ctx() -> SessionContext {
        let (mut r, t) = SessionRecorder::new(0, 1_000);
        r.apply(&t, SessionEvent::TaskOpen { task: "fix flow mode acp".into() }, 1)
            .unwrap();
        r.context(2).unwrap().clone()
    }

    fn halted_ctx() -> SessionContext {
        let (mut r, t) = SessionRecorder::new(0, 1_000);
        r.apply(&t, SessionEvent::TaskOpen { task: "fix flow mode acp".into() }, 1)
            .unwrap();
        r.apply(&t, SessionEvent::HaltTriggered { reason: "stop".into() }, 2)
            .unwrap();
        r.context(3).unwrap().clone()
    }

    fn grant(id: &str) -> HumanActReceipt {
        HumanActReceipt {
            id: id.into(),
            human_id: "human:kyle".into(),
            decision: ActDecision::Grant,
            tool: "surface".into(),
            target: "gaia-acp".into(),
        }
    }

    #[test]
    fn surfaces_correct_resource_first() {
        let out = rank_resources("fix flow mode acp", &sample(), 5);
        assert_eq!(out.len(), 2);
        assert_eq!(out[0].resource(), "gaia-acp");
        assert_eq!(out[1].resource(), "gaia-orchestrator");
        assert!(close(out[0].score(), 0.75));
        assert!(close(out[1].score(), 0.25));
    }

    #[test]
    fn unrelated_resources_are_not_surfaced() {
        let out = rank_resources("fix flow mode acp", &sample(), 5);
        assert!(out.iter().all(|e| e.resource() != "gaia-sfs"));
    }

    #[test]
    fn surface_refuses_to_act() {
        let out = rank_resources("fix flow mode acp", &sample(), 5);
        assert_eq!(out[0].act(), Err(SurfaceError::SurfaceOnly));
    }

    #[test]
    fn suggestion_without_receipt_does_not_execute() {
        let out = rank_resources("fix flow mode acp", &sample(), 5);
        let mut gate = ActGate::default();
        let err = out[0].submit_to(&mut gate, &open_ctx(), None).unwrap_err();
        assert_eq!(err, ActError::ApprovalMissing);
        assert!(gate.executed().is_empty());
        assert_eq!(out[0].propose().target, "gaia-acp");
    }

    #[test]
    fn suggestion_deny_does_not_execute() {
        let out = rank_resources("fix flow mode acp", &sample(), 5);
        let mut gate = gate();
        let mut receipt = grant("r-deny");
        receipt.decision = ActDecision::Deny;
        let err = out[0]
            .submit_to(&mut gate, &open_ctx(), Some(&receipt))
            .unwrap_err();
        assert_eq!(err, ActError::Denied);
        assert!(gate.executed().is_empty());
    }

    #[test]
    fn human_grant_is_the_only_execute_path() {
        let out = rank_resources("fix flow mode acp", &sample(), 5);
        let mut gate = gate();
        let receipt = grant("r-grant");
        out[0]
            .submit_to(&mut gate, &open_ctx(), Some(&receipt))
            .unwrap();
        assert_eq!(gate.executed(), ["surface:gaia-acp".to_string()]);
        assert_eq!(out[0].act(), Err(SurfaceError::SurfaceOnly));
    }

    #[test]
    fn halted_session_blocks_a_valid_grant() {
        let out = rank_resources("fix flow mode acp", &sample(), 5);
        let mut gate = gate();
        let receipt = grant("r-halt");
        let err = out[0]
            .submit_to(&mut gate, &halted_ctx(), Some(&receipt))
            .unwrap_err();
        assert_eq!(err, ActError::Halted);
        assert!(gate.executed().is_empty());
    }

    #[test]
    fn prefixed_agent_cannot_use_a_surface_grant() {
        let out = rank_resources("fix flow mode acp", &sample(), 5);
        let mut gate = ActGate::with_allowlist(HumanAllowlist::new(["human:agent"]));
        let mut receipt = grant("r-agent");
        receipt.human_id = "human:agent".into();
        let err = out[0]
            .submit_to(&mut gate, &open_ctx(), Some(&receipt))
            .unwrap_err();
        assert_eq!(err, ActError::NotHuman);
        assert!(gate.executed().is_empty());
    }

    #[test]
    fn confidence_band_is_honest() {
        let out = rank_resources("fix flow mode acp", &sample(), 5);
        for e in &out {
            let c = e.confidence();
            assert!(c.low() >= 0.0 && c.high() <= 1.0);
            assert!(c.low() <= c.point() && c.point() <= c.high());
        }
        assert!(out[1].confidence().width() > out[0].confidence().width());
    }

    #[test]
    fn source_trace_lists_matches_and_source() {
        let out = rank_resources("fix flow mode acp", &sample(), 5);
        let trace = out[0].source_trace();
        assert!(trace.contains(&"matched term: acp".to_string()));
        assert!(trace.contains(&"matched term: flow".to_string()));
        assert!(trace.contains(&"resource source: test".to_string()));
    }

    #[test]
    fn no_match_and_empty_task_give_nothing() {
        assert!(rank_resources("zzz qqq", &sample(), 5).is_empty());
        assert!(rank_resources("the and of", &sample(), 5).is_empty());
        assert!(rank_resources("", &sample(), 5).is_empty());
    }

    #[test]
    fn limit_is_respected() {
        assert_eq!(rank_resources("fix flow mode acp", &sample(), 1).len(), 1);
        assert!(rank_resources("fix flow mode acp", &sample(), 0).is_empty());
    }

    #[test]
    fn ties_break_by_id() {
        let rs = vec![cand("b-crate", &["flow"]), cand("a-crate", &["flow"])];
        let out = rank_resources("flow", &rs, 5);
        assert_eq!(out[0].resource(), "a-crate");
        assert_eq!(out[1].resource(), "b-crate");
    }

    #[test]
    fn session_task_drives_surfacing() {
        let ctx = open_ctx();
        let out = surface_for_session(&ctx, &sample(), 5).unwrap();
        assert_eq!(out[0].resource(), "gaia-acp");
    }

    #[test]
    fn session_without_task_surfaces_nothing() {
        let (r, _t) = SessionRecorder::new(0, 1_000);
        let ctx = r.context(1).unwrap();
        assert!(surface_for_session(ctx, &sample(), 5).unwrap().is_empty());
    }

    #[test]
    fn halted_session_surfaces_nothing() {
        assert_eq!(
            surface_for_session(&halted_ctx(), &sample(), 5),
            Err(SurfaceError::SessionHalted)
        );
    }
}
