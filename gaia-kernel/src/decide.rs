//! Auditable choice. A tie is not a winner.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Choice {
    pub winner: &'static str,
    pub kept: &'static str,
    pub not_taken: &'static str,
}

pub fn choose(left_name: &'static str, right_name: &'static str, left: u32, right: u32) -> Result<Choice, &'static str> {
    if left == right {
        return Err("tie is not a decision");
    }
    if left > right {
        Ok(Choice { winner: left_name, kept: left_name, not_taken: right_name })
    } else {
        Ok(Choice { winner: right_name, kept: right_name, not_taken: left_name })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn higher_count_wins_and_names_the_loss() {
        let choice = choose("list-24", "tree-21", 3, 2).unwrap();
        assert_eq!(choice.winner, "list-24");
        assert_eq!(choice.not_taken, "tree-21");
        assert_eq!(choose("list-24", "tree-21", 1, 1), Err("tie is not a decision"));
    }
}
