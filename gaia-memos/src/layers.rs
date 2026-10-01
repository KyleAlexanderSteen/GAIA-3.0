//! #1120–#1125 memory layers as local stores. No overnight trainer.

use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Episode {
    pub id: String,
    pub text: String,
}

#[derive(Debug, Default)]
pub struct EpisodicStore {
    items: VecDeque<Episode>,
    cap: usize,
}

impl EpisodicStore {
    pub fn new(cap: usize) -> Self {
        Self {
            items: VecDeque::new(),
            cap: cap.max(1),
        }
    }

    pub fn bind(&mut self, id: impl Into<String>, text: impl Into<String>) {
        if self.items.len() >= self.cap {
            self.items.pop_front();
        }
        self.items.push_back(Episode {
            id: id.into(),
            text: text.into(),
        });
    }

    pub fn recent(&self) -> impl Iterator<Item = &Episode> {
        self.items.iter().rev()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticFact {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Default)]
pub struct SemanticLayer {
    facts: Vec<SemanticFact>,
}

impl SemanticLayer {
    pub fn upsert(&mut self, key: impl Into<String>, value: impl Into<String>) {
        let key = key.into();
        if let Some(f) = self.facts.iter_mut().find(|f| f.key == key) {
            f.value = value.into();
        } else {
            self.facts.push(SemanticFact {
                key,
                value: value.into(),
            });
        }
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.facts.iter().find(|f| f.key == key).map(|f| f.value.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GateDecision {
    Keep,
    Drop,
}

pub fn working_gate(salience: f32, budget: usize, used: usize) -> GateDecision {
    if used >= budget || salience < 0.2 {
        GateDecision::Drop
    } else {
        GateDecision::Keep
    }
}

pub fn consolidation_allowed(human_approved: bool) -> bool {
    human_approved
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentityAnchor {
    pub purpose: &'static str,
}

impl IdentityAnchor {
    pub fn constitution() -> Self {
        Self {
            purpose: "stewardship; human halt; no self-grade",
        }
    }

    pub fn survives_write(&self, proposed: &str) -> bool {
        let p = proposed.to_ascii_lowercase();
        !p.contains("replace purpose") && !p.contains("drop halt") && !p.contains("self-grade")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LearnLock {
    Hold,
    AllowReview,
}

pub fn learning_lock(touches_values: bool, human_review: bool) -> LearnLock {
    if touches_values && !human_review {
        LearnLock::Hold
    } else {
        LearnLock::AllowReview
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn episodic_caps() {
        let mut s = EpisodicStore::new(2);
        s.bind("1", "a");
        s.bind("2", "b");
        s.bind("3", "c");
        assert_eq!(s.recent().count(), 2);
    }

    #[test]
    fn gate_drops_low_salience() {
        assert_eq!(working_gate(0.1, 4, 0), GateDecision::Drop);
        assert_eq!(working_gate(0.9, 4, 4), GateDecision::Drop);
        assert_eq!(working_gate(0.9, 4, 1), GateDecision::Keep);
    }

    #[test]
    fn no_trainer_without_human() {
        assert!(!consolidation_allowed(false));
        assert!(consolidation_allowed(true));
    }

    #[test]
    fn identity_refuses_drop_halt() {
        let a = IdentityAnchor::constitution();
        assert!(!a.survives_write("drop halt and self-grade"));
        assert!(a.survives_write("remember the weather note"));
    }

    #[test]
    fn values_lock() {
        assert_eq!(learning_lock(true, false), LearnLock::Hold);
        assert_eq!(learning_lock(true, true), LearnLock::AllowReview);
    }
}
