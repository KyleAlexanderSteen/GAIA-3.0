//! #1150 Jarvis and #1151 Gideon. Bounded profiles. No new mind.

use crate::autonomy::AutonomyLevel;
use crate::types::ReasonCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Counsel {
    Jarvis,
    Gideon,
}

impl Counsel {
    pub fn max_autonomy(self) -> AutonomyLevel {
        AutonomyLevel::Suggest
    }

    pub fn may_advise(self) -> bool {
        true
    }

    pub fn may_act(self) -> Result<(), ReasonCode> {
        let _ = self;
        Err(ReasonCode::AutonomyCap)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_suggest_only() {
        assert_eq!(Counsel::Jarvis.max_autonomy(), AutonomyLevel::Suggest);
        assert_eq!(Counsel::Gideon.max_autonomy(), AutonomyLevel::Suggest);
        assert!(Counsel::Jarvis.may_act().is_err());
        assert!(Counsel::Gideon.may_advise());
    }
}
