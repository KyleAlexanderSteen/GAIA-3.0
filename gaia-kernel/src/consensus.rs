//! Local majority. A tie is not a decision.

pub fn majority(votes: &[bool]) -> Result<bool, &'static str> {
    if votes.is_empty() {
        return Err("no votes");
    }
    let yes = votes.iter().filter(|vote| **vote).count();
    let no = votes.len() - yes;
    if yes == no {
        Err("tie is not a decision")
    } else {
        Ok(yes > no)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_of_three_decide_and_a_tie_does_not() {
        assert_eq!(majority(&[true, true, false]), Ok(true));
        assert_eq!(majority(&[true, false]), Err("tie is not a decision"));
    }
}
