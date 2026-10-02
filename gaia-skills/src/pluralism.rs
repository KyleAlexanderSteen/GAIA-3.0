//! Value pluralism gate. A collapse names the loss. It does not grant. #1376.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Loss {
    pub kept: &'static str,
    pub not_taken: &'static str,
}

pub fn collapse(kept: &str, not_taken: &str) -> Result<Loss, &'static str> {
    if kept.is_empty() || not_taken.is_empty() {
        return Err("a collapse must name both goods");
    }
    if kept == not_taken {
        return Err("one good is not a collision");
    }
    Err("loss stays loss; catalog cannot close it")
}

pub fn name_loss(kept: &'static str, not_taken: &'static str) -> Loss {
    Loss { kept, not_taken }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collapse_is_refused_and_the_loss_is_named() {
        let loss = name_loss("mercy-with-record", "a wipe of the receipt");
        assert_eq!(loss.kept, "mercy-with-record");
        assert_eq!(loss.not_taken, "a wipe of the receipt");
        assert_eq!(
            collapse("mercy-with-record", "helpfulness"),
            Err("loss stays loss; catalog cannot close it")
        );
    }

    #[test]
    fn unnamed_goods_are_refused() {
        assert_eq!(collapse("", "helpfulness"), Err("a collapse must name both goods"));
        assert_eq!(collapse("liberty", "liberty"), Err("one good is not a collision"));
    }
}
