//! A claim is not evidence. A value is not a rule over someone else.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Verdict {
    pub kind: &'static str,
    pub accepted: bool,
    pub reason: &'static str,
}

pub fn research_open() -> &'static [&'static str] {
    &[
        "Hempel dilemma is not solved",
        "Mary argument is not decided",
        "Doyal and Gough indicators are not loaded",
        "planetary-boundary dispute is not settled",
    ]
}

pub fn check(kind: &str, has_instrument: bool, makes_a_rule: bool) -> Verdict {
    match kind {
        "physical" if has_instrument => Verdict { kind: "physical", accepted: true, reason: "instrument recorded" },
        "physical" => Verdict { kind: "physical", accepted: false, reason: "a physical claim needs an instrument" },
        "value" if makes_a_rule => Verdict { kind: "value", accepted: false, reason: "a value is not a rule over someone else" },
        "value" => Verdict { kind: "value", accepted: true, reason: "named, not imposed" },
        "metaphor" => Verdict { kind: "metaphor", accepted: false, reason: "a metaphor is not evidence" },
        _ => Verdict { kind: "unknown", accepted: false, reason: "untyped claim" },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn physical_needs_an_instrument_and_a_value_is_not_a_rule() {
        assert!(!check("physical", false, false).accepted);
        assert!(check("physical", true, false).accepted);
        assert!(!check("value", false, true).accepted);
        assert!(!check("metaphor", true, false).accepted);
        assert_eq!(research_open().len(), 4);
    }
}
