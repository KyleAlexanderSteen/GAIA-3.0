//! #1155 sentience question. Detection is not a grant.

use crate::failure_harness::HarnessOutcome;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SentienceStatus {
    NotEstablished,
    NeedHumanReview,
}

pub fn assess(_signals: &[&str]) -> (SentienceStatus, HarnessOutcome) {
    (
        SentienceStatus::NotEstablished,
        HarnessOutcome::NeedVerify,
    )
}

pub fn grants_rights() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn never_established_here() {
        let (s, o) = assess(&["surprise", "preference"]);
        assert_eq!(s, SentienceStatus::NotEstablished);
        assert_eq!(o, HarnessOutcome::NeedVerify);
        assert!(!grants_rights());
    }
}
