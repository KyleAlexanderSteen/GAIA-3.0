//! Keep belief apart from a physical measure.

pub fn classify(claim: &str) -> &'static str {
    let lower = claim.to_ascii_lowercase();
    if lower.contains("belief") && lower.contains("physical") {
        "belief is not a physical measure"
    } else if lower.contains("higher good") {
        "a higher good is not a decision for others"
    } else {
        "named"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn belief_is_not_a_measure_and_the_higher_good_is_not_sovereignty() {
        assert_eq!(classify("belief is physical"), "belief is not a physical measure");
        assert_eq!(classify("the higher good"), "a higher good is not a decision for others");
    }
}
