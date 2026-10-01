//! Jarvis Phase 1 — session context.
//!
//! `SessionContext` is read-only to everyone. The only mutation path is
//! `SessionRecorder::apply`, which takes a `SessionEvent` and a
//! `SessionToken` held by the session owner.
//!
//! - Writes without the matching token are rejected.
//! - `HaltTriggered` is sticky: every later event is refused until a human
//!   re-authorization call clears it.
//! - Context carries an expiry and is dropped once it passes.
//!
//! Time is passed in as `now_ms` (epoch milliseconds) so behaviour is
//! deterministic and testable. There is no clock access in this module.
//!
//! Not yet integrated with the memory gate or identity anchor. The
//! `HumanReauthorization` check only requires a non-empty approver name; real
//! verification waits on identity integration.

use std::collections::BTreeSet;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_SESSION_ID: AtomicU64 = AtomicU64::new(1);

/// Events that may change session context. Nothing else can.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionEvent {
    /// Open a task (pushed onto the open-thread stack).
    TaskOpen { task: String },
    /// Close the most recently opened task.
    TaskClose,
    /// Record a decision summary.
    DecisionMade { summary: String },
    /// Record a resource (for example a crate name) as active.
    ResourceSurfaced { resource: String },
    /// Halt the session. Sticky until human re-authorization.
    HaltTriggered { reason: String },
}

/// Errors returned by the session recorder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionError {
    /// The token does not belong to this session.
    Unauthorized,
    /// The session is halted; only re-authorization is accepted.
    Halted,
    /// The context passed its retention window and was dropped.
    Expired,
    /// `TaskClose` with no open task.
    NoOpenTask,
    /// A required text field was empty.
    EmptyField(&'static str),
    /// Re-authorization was refused.
    ReauthRejected,
}

impl std::fmt::Display for SessionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unauthorized => write!(f, "session token does not match this session"),
            Self::Halted => write!(f, "session is halted; human re-authorization required"),
            Self::Expired => write!(f, "session context expired and was dropped"),
            Self::NoOpenTask => write!(f, "no open task to close"),
            Self::EmptyField(name) => write!(f, "field `{name}` must not be empty"),
            Self::ReauthRejected => write!(f, "re-authorization rejected"),
        }
    }
}

impl std::error::Error for SessionError {}

/// Proof that the caller owns a session. Not `Clone`; issued once by
/// `SessionRecorder::new`.
#[derive(Debug, PartialEq, Eq)]
pub struct SessionToken {
    session_id: u64,
}

/// A human's re-authorization after a halt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HumanReauthorization {
    /// Name of the person re-authorizing. Must be non-empty.
    pub approver: String,
}

/// Read-only view of session state. Fields are private.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionContext {
    tasks: Vec<String>,
    resources: BTreeSet<String>,
    last_decision: Option<String>,
    halt: Option<String>,
    expires_at_ms: u64,
}

impl SessionContext {
    fn new(expires_at_ms: u64) -> Self {
        Self {
            tasks: Vec::new(),
            resources: BTreeSet::new(),
            last_decision: None,
            halt: None,
            expires_at_ms,
        }
    }

    /// The current (most recently opened) task.
    pub fn task(&self) -> Option<&str> {
        self.tasks.last().map(String::as_str)
    }

    /// All open tasks, oldest first.
    pub fn open_threads(&self) -> &[String] {
        &self.tasks
    }

    /// Resources surfaced so far (for example crate names), sorted.
    pub fn active_crates(&self) -> Vec<&str> {
        self.resources.iter().map(String::as_str).collect()
    }

    /// The most recent decision summary.
    pub fn last_decision(&self) -> Option<&str> {
        self.last_decision.as_deref()
    }

    /// Why the session is halted, if it is.
    pub fn halt_reason(&self) -> Option<&str> {
        self.halt.as_deref()
    }

    /// Whether the session is halted.
    pub fn is_halted(&self) -> bool {
        self.halt.is_some()
    }

    /// Epoch ms at which this context expires.
    pub fn expires_at_ms(&self) -> u64 {
        self.expires_at_ms
    }
}

/// The only writer of a `SessionContext`.
#[derive(Debug)]
pub struct SessionRecorder {
    session_id: u64,
    context: Option<SessionContext>,
}

impl SessionRecorder {
    /// Start a session that expires `retention_ms` after `now_ms`.
    /// Returns the recorder and the only token that can write to it.
    pub fn new(now_ms: u64, retention_ms: u64) -> (Self, SessionToken) {
        let session_id = NEXT_SESSION_ID.fetch_add(1, Ordering::Relaxed);
        let recorder = Self {
            session_id,
            context: Some(SessionContext::new(now_ms.saturating_add(retention_ms))),
        };
        (recorder, SessionToken { session_id })
    }

    /// Read the context. Returns `None` once it has expired.
    pub fn context(&self, now_ms: u64) -> Option<&SessionContext> {
        self.context.as_ref().filter(|c| now_ms < c.expires_at_ms)
    }

    /// Drop the context if its retention window has passed.
    pub fn purge_expired(&mut self, now_ms: u64) {
        if let Some(c) = &self.context {
            if now_ms >= c.expires_at_ms {
                self.context = None;
            }
        }
    }

    fn check(&mut self, token: &SessionToken, now_ms: u64) -> Result<(), SessionError> {
        if token.session_id != self.session_id {
            return Err(SessionError::Unauthorized);
        }
        self.purge_expired(now_ms);
        if self.context.is_none() {
            return Err(SessionError::Expired);
        }
        Ok(())
    }

    /// Apply one event. Checks run in order: token, expiry, halt.
    pub fn apply(
        &mut self,
        token: &SessionToken,
        event: SessionEvent,
        now_ms: u64,
    ) -> Result<(), SessionError> {
        self.check(token, now_ms)?;
        let ctx = self.context.as_mut().ok_or(SessionError::Expired)?;
        if ctx.halt.is_some() {
            return Err(SessionError::Halted);
        }
        match event {
            SessionEvent::TaskOpen { task } => {
                if task.trim().is_empty() {
                    return Err(SessionError::EmptyField("task"));
                }
                ctx.tasks.push(task);
            }
            SessionEvent::TaskClose => {
                if ctx.tasks.pop().is_none() {
                    return Err(SessionError::NoOpenTask);
                }
            }
            SessionEvent::DecisionMade { summary } => {
                if summary.trim().is_empty() {
                    return Err(SessionError::EmptyField("summary"));
                }
                ctx.last_decision = Some(summary);
            }
            SessionEvent::ResourceSurfaced { resource } => {
                if resource.trim().is_empty() {
                    return Err(SessionError::EmptyField("resource"));
                }
                ctx.resources.insert(resource);
            }
            SessionEvent::HaltTriggered { reason } => {
                if reason.trim().is_empty() {
                    return Err(SessionError::EmptyField("reason"));
                }
                ctx.halt = Some(reason);
            }
        }
        Ok(())
    }

    /// Clear a halt. Context is kept. Requires a named human approver.
    pub fn reauthorize(
        &mut self,
        token: &SessionToken,
        auth: HumanReauthorization,
        now_ms: u64,
    ) -> Result<(), SessionError> {
        self.check(token, now_ms)?;
        if auth.approver.trim().is_empty() {
            return Err(SessionError::ReauthRejected);
        }
        let ctx = self.context.as_mut().ok_or(SessionError::Expired)?;
        ctx.halt = None;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn open(r: &mut SessionRecorder, t: &SessionToken, task: &str, now: u64) {
        r.apply(t, SessionEvent::TaskOpen { task: task.into() }, now)
            .unwrap();
    }

    #[test]
    fn records_task_decision_and_resources() {
        let (mut r, t) = SessionRecorder::new(0, 1_000);
        open(&mut r, &t, "phase 1", 1);
        r.apply(&t, SessionEvent::DecisionMade { summary: "new module".into() }, 2)
            .unwrap();
        r.apply(&t, SessionEvent::ResourceSurfaced { resource: "gaia-agents".into() }, 3)
            .unwrap();
        let c = r.context(4).unwrap();
        assert_eq!(c.task(), Some("phase 1"));
        assert_eq!(c.open_threads(), ["phase 1".to_string()]);
        assert_eq!(c.last_decision(), Some("new module"));
        assert_eq!(c.active_crates(), vec!["gaia-agents"]);
    }

    #[test]
    fn context_survives_crate_swap() {
        let (mut r, t) = SessionRecorder::new(0, 1_000);
        open(&mut r, &t, "phase 1", 1);
        r.apply(&t, SessionEvent::ResourceSurfaced { resource: "gaia-agents".into() }, 2)
            .unwrap();
        r.apply(&t, SessionEvent::ResourceSurfaced { resource: "gaia-sfs".into() }, 3)
            .unwrap();
        let c = r.context(4).unwrap();
        assert_eq!(c.task(), Some("phase 1"));
        assert_eq!(c.active_crates(), vec!["gaia-agents", "gaia-sfs"]);
    }

    #[test]
    fn task_close_pops_and_errors_when_empty() {
        let (mut r, t) = SessionRecorder::new(0, 1_000);
        assert_eq!(r.apply(&t, SessionEvent::TaskClose, 1), Err(SessionError::NoOpenTask));
        open(&mut r, &t, "a", 2);
        open(&mut r, &t, "b", 3);
        r.apply(&t, SessionEvent::TaskClose, 4).unwrap();
        assert_eq!(r.context(5).unwrap().task(), Some("a"));
    }

    #[test]
    fn write_with_other_sessions_token_is_rejected() {
        let (mut r, _mine) = SessionRecorder::new(0, 1_000);
        let (_other_r, other_t) = SessionRecorder::new(0, 1_000);
        let res = r.apply(&other_t, SessionEvent::TaskOpen { task: "x".into() }, 1);
        assert_eq!(res, Err(SessionError::Unauthorized));
        assert!(r.context(2).unwrap().task().is_none());
    }

    #[test]
    fn empty_fields_are_rejected() {
        let (mut r, t) = SessionRecorder::new(0, 1_000);
        let res = r.apply(&t, SessionEvent::TaskOpen { task: "  ".into() }, 1);
        assert_eq!(res, Err(SessionError::EmptyField("task")));
    }

    #[test]
    fn halt_is_sticky_and_blocks_all_events() {
        let (mut r, t) = SessionRecorder::new(0, 1_000);
        r.apply(&t, SessionEvent::HaltTriggered { reason: "stop".into() }, 1)
            .unwrap();
        assert!(r.context(2).unwrap().is_halted());
        let res = r.apply(&t, SessionEvent::TaskOpen { task: "x".into() }, 2);
        assert_eq!(res, Err(SessionError::Halted));
        assert_eq!(r.apply(&t, SessionEvent::TaskClose, 3), Err(SessionError::Halted));
    }

    #[test]
    fn reauthorization_clears_halt_and_keeps_context() {
        let (mut r, t) = SessionRecorder::new(0, 1_000);
        open(&mut r, &t, "keep me", 1);
        r.apply(&t, SessionEvent::HaltTriggered { reason: "stop".into() }, 2)
            .unwrap();
        let blank = HumanReauthorization { approver: " ".into() };
        assert_eq!(r.reauthorize(&t, blank, 3), Err(SessionError::ReauthRejected));
        assert!(r.context(3).unwrap().is_halted());
        let ok = HumanReauthorization { approver: "owner".into() };
        r.reauthorize(&t, ok, 4).unwrap();
        let c = r.context(5).unwrap();
        assert!(!c.is_halted());
        assert_eq!(c.task(), Some("keep me"));
        r.apply(&t, SessionEvent::DecisionMade { summary: "resumed".into() }, 6)
            .unwrap();
    }

    #[test]
    fn expired_context_is_dropped() {
        let (mut r, t) = SessionRecorder::new(100, 50);
        open(&mut r, &t, "short lived", 120);
        assert!(r.context(149).is_some());
        assert!(r.context(150).is_none());
        let res = r.apply(&t, SessionEvent::TaskOpen { task: "late".into() }, 150);
        assert_eq!(res, Err(SessionError::Expired));
        assert!(r.context(120).is_none());
    }
}
