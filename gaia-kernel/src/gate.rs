//! Local allow or deny before a tool call. Not Dogwood. Not a cloud policy.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
    Allow,
    Deny(&'static str),
}

pub fn tool_call(text: &str) -> Verdict {
    let trimmed = text.trim();
    if trimmed.starts_with("echo:") && trimmed.len() > "echo:".len() {
        return Verdict::Allow;
    }
    Verdict::Deny("plain intent is not a tool call")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn echo_is_allowed_and_plain_is_denied() {
        assert_eq!(tool_call("echo: hi"), Verdict::Allow);
        assert_eq!(tool_call("  echo: hi"), Verdict::Allow);
        assert_eq!(tool_call("echo:"), Verdict::Deny("plain intent is not a tool call"));
        assert_eq!(tool_call("build the world"), Verdict::Deny("plain intent is not a tool call"));
    }
}
