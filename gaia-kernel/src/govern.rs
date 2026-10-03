//! A run needs a risk note and a human record.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Oversight {
    pub person: String,
    pub decision: String,
}

pub fn allow_run(risk: &str, person: &str) -> Result<Oversight, &'static str> {
    if risk.trim().is_empty() {
        return Err("a run needs a risk note");
    }
    if person.trim().is_empty() {
        return Err("a run needs a human record");
    }
    Ok(Oversight { person: person.to_string(), decision: "recorded".into() })
}

pub fn vendor_leaves(kind: &str) -> bool {
    kind != "local-file" && kind != "ollama-localhost"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_run_needs_a_risk_and_a_person() {
        assert_eq!(allow_run("", "ada"), Err("a run needs a risk note"));
        assert_eq!(allow_run("local file", "").unwrap_err(), "a run needs a human record");
        assert_eq!(allow_run("local file", "ada").unwrap().decision, "recorded");
        assert!(vendor_leaves("huggingface"));
        assert!(!vendor_leaves("local-file"));
    }
}
