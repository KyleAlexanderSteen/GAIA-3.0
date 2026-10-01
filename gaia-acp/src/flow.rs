//! #1148 flow modes. Companion labels. Not autonomy.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlowMode {
    Struggle,
    Release,
    Flow,
    Recovery,
}

impl FlowMode {
    pub fn may_act(self) -> bool {
        matches!(self, FlowMode::Flow)
    }

    pub fn suggest_only(self) -> bool {
        !self.may_act()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn struggle_is_suggest_only() {
        assert!(FlowMode::Struggle.suggest_only());
        assert!(!FlowMode::Flow.suggest_only());
    }
}
