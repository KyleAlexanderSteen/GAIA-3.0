//! #1148 flow modes. Companion labels. Not autonomy.
//!
//! A flow mode describes the human's state. It never grants the system
//! permission to act: every variant is suggest-only.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlowMode {
    Struggle,
    Release,
    Flow,
    Recovery,
}

impl FlowMode {
    pub const ALL: [FlowMode; 4] = [
        FlowMode::Struggle,
        FlowMode::Release,
        FlowMode::Flow,
        FlowMode::Recovery,
    ];

    /// Flow classification can never raise autonomy.
    pub fn may_act(self) -> bool {
        false
    }

    pub fn suggest_only(self) -> bool {
        !self.may_act()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_flow_mode_is_suggest_only() {
        for mode in FlowMode::ALL {
            assert!(mode.suggest_only(), "{:?} must be suggest-only", mode);
            assert!(!mode.may_act(), "{:?} must not act", mode);
        }
    }

    #[test]
    fn flow_does_not_raise_autonomy() {
        assert!(FlowMode::Flow.suggest_only());
        assert!(!FlowMode::Flow.may_act());
    }
}
