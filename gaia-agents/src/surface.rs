//! Jarvis Phase 2 — anticipatory surface layer.
//!
//! Given the current task and a set of candidate resources, rank which ones
//! are worth showing to the human. This layer only **surfaces**. It never
//! acts: `SurfaceEvent::act` always refuses.
//!
//! Scoring is keyword overlap between the task text and each resource's id
//! and keywords. There is no model behind it. The confidence band reflects
//! how much evidence the overlap provides and nothing more: one matched
//! term gives a wide band, several matched terms give a narrower one.
//!
//! Not yet connected to the Predict-5 honesty layer (#1146). `ConfidenceBand`
//! is a local type until that integration lands.

use std::collections::BTreeSet;

use crate::session::SessionContext;

/// Words ignored when scoring. `gaia` is dropped because nearly every crate
/// and task mentions it, so it carries no signal.
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

/// What kind of thing is being surfaced.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceKind {
    /// A workspace crate.
    Crate,
    /// A tool.
    Tool,
    /// A document.
    Doc,
    /// A prior decision.
    Decision,
}

/// Something that could be surfaced.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceCandidate {
    /// Identifier, for example `gaia-acp`.
    pub id: String,
    /// Kind of resource.
    pub kind: ResourceKind,
    /// Extra words describing the resource.
    pub keywords: Vec<String>,
    /// Where this candidate came from (recorded in the source trace).
    pub source: String,
}

/// A low / point / high estimate, all within 0.0..=1.0.
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

    /// Lower bound.
    pub fn low(&self) -> f64 {
        self.low
    }

    /// Point estimate.
    pub fn point(&self) -> f64 {
        self.point
    }

    /// Upper bound.
    pub fn high(&self) -> f64 {
        self.high
    }

    /// Width of the band (`high - low`).
    pub fn width(&self) -> f64 {
        self.high - self.low
    }
}

/// Errors from the surface layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SurfaceError {
    /// Surface events cannot be acted on by this layer.
    SurfaceOnly,
    /// The session is halted; nothing is surfaced.
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

/// A ranked suggestion for the human. Fields are private.
#[derive(Debug, Clone, PartialEq)]
pub struct SurfaceEvent {
    resource: String,
    kind: ResourceKind,
    score: f64,
    confidence: ConfidenceBand,
    source_trace: Vec<String>,
}

impl SurfaceEvent {
    /// Id of the surfaced resource.
    pub fn resource(&self) -> &str {
        &self.resource
    }

    /// Kind of the surfaced resource.
    pub fn kind(&self) -> ResourceKind {
        self.kind
    }

    /// Relevance score in 0.0..=1.0.
    pub fn score(&self) -> f64 {
        self.score
    }

    /// Confidence band for the score.
    pub fn confidence(&self) -> ConfidenceBand {
        self.confidence
    }

    /// Why this was surfaced: matched terms and the resource source.
    pub fn source_trace(&self) -> &[String] {
        &self.source_trace
    }

    /// Always refuses. Surfacing never acts; a human must act.
    pub fn act(&self) -> Result<(), SurfaceError> {
        Err(SurfaceError::SurfaceOnly)
    }
}

/// Rank `resources` against `task`. Resources with no matching term are
/// dropped. Ordering is by score (highest first), then by id, so output is
/// deterministic.
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

/// Surface resources for the task in a session context. Returns an error if
/// the session is halted, and nothing if there is no open task.
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
    fn confidence_band_is_honest() {
        let out = rank_resources("fix flow mode acp", &sample(), 5);
        for e in &out {
            let c = e.confidence();
            assert!(c.low() >= 0.0 && c.high() <= 1.0);
            assert!(c.low() <= c.point() && c.point() <= c.high());
        }
        // One matched term is weaker evidence than three, so a wider band.
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
        let (mut r, t) = SessionRecorder::new(0, 1_000);
        r.apply(&t, SessionEvent::TaskOpen { task: "fix flow mode acp".into() }, 1)
            .unwrap();
        let ctx = r.context(2).unwrap();
        let out = surface_for_session(ctx, &sample(), 5).unwrap();
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
        let (mut r, t) = SessionRecorder::new(0, 1_000);
        r.apply(&t, SessionEvent::TaskOpen { task: "fix flow mode acp".into() }, 1)
            .unwrap();
        r.apply(&t, SessionEvent::HaltTriggered { reason: "stop".into() }, 2)
            .unwrap();
        let ctx = r.context(3).unwrap();
        assert_eq!(
            surface_for_session(ctx, &sample(), 5),
            Err(SurfaceError::SessionHalted)
        );
    }
}
