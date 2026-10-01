//! #1144 #1132 black-swan resilience. Not a prophecy detector.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shift {
    InDistribution,
    OutOfDistribution,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraySwan {
    pub name: &'static str,
    pub rare: bool,
    pub structurally_novel: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resilience {
    NeedVerify,
    OutOfScope,
    SurvivePlan,
}

pub fn shift_of(distance: f64, threshold: f64) -> Shift {
    if distance > threshold {
        Shift::OutOfDistribution
    } else {
        Shift::InDistribution
    }
}

pub fn classify_event(novel: bool) -> Resilience {
    if novel {
        Resilience::OutOfScope
    } else {
        Resilience::NeedVerify
    }
}

pub fn gray_library() -> [GraySwan; 3] {
    [
        GraySwan {
            name: "regional grid loss",
            rare: true,
            structurally_novel: false,
        },
        GraySwan {
            name: "dns covert channel",
            rare: true,
            structurally_novel: false,
        },
        GraySwan {
            name: "true black swan",
            rare: true,
            structurally_novel: true,
        },
    ]
}

pub fn stress(category: &str) -> Resilience {
    if category.trim().is_empty() {
        Resilience::OutOfScope
    } else {
        Resilience::SurvivePlan
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn far_point_is_ood() {
        assert_eq!(shift_of(3.0, 1.0), Shift::OutOfDistribution);
    }

    #[test]
    fn novel_is_out_of_scope() {
        assert_eq!(classify_event(true), Resilience::OutOfScope);
    }

    #[test]
    fn library_marks_true_swan_novel() {
        assert!(gray_library().iter().any(|g| g.structurally_novel));
    }
}
