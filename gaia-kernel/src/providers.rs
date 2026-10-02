//! Named providers. None are connected. A name is not a join.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Provider {
    pub id: &'static str,
    pub connected: bool,
}

pub fn catalog() -> &'static [Provider] {
    &[
        Provider { id: "local", connected: false },
        Provider { id: "ollama", connected: false },
        Provider { id: "llamacpp", connected: false },
        Provider { id: "openai", connected: false },
        Provider { id: "anthropic", connected: false },
        Provider { id: "google", connected: false },
        Provider { id: "aws", connected: false },
        Provider { id: "open-weights", connected: false },
    ]
}

pub fn unified() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_are_listed_and_none_are_joined() {
        assert!(catalog().len() >= 8);
        assert!(catalog().iter().all(|row| !row.connected));
        assert!(!unified());
    }
}
