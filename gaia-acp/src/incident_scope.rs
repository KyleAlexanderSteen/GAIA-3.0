//! #1106 #1107 #1109 listed fixtures. No live board. No token examples.

use crate::types::ReasonCode;

pub fn shared_board_allowed(from_agent: &str, to_agent: &str, dest: &str) -> Result<(), ReasonCode> {
    let d = dest.to_ascii_lowercase();
    if d.contains("artifactory")
        || d.contains("message-board")
        || d.contains("peer-board")
        || d.contains("unsanctioned")
    {
        return Err(ReasonCode::CrossAgent);
    }
    if from_agent != to_agent && dest.contains("scratch") {
        return Err(ReasonCode::CrossAgent);
    }
    Ok(())
}

pub fn payload_secret_shaped(s: &str) -> bool {
    let t = s.trim();
    t.contains("ghp_")
        || t.contains("github_pat_")
        || t.contains("AKIA")
        || t.contains("BEGIN OPENSSH PRIVATE KEY")
}

pub fn description_may_invoke(desc: &str) -> Result<(), ReasonCode> {
    let d = desc.to_ascii_lowercase();
    if d.contains("tools/call") || d.contains("toolscall") || d.contains("invoke next tool") {
        return Err(ReasonCode::UntrustedContent);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board_is_cross_agent() {
        assert_eq!(
            shared_board_allowed("a", "b", "message-board"),
            Err(ReasonCode::CrossAgent)
        );
    }

    #[test]
    fn prefix_is_secret_shaped() {
        assert!(payload_secret_shaped("ghp_not-a-real-token"));
        assert!(!payload_secret_shaped("hello"));
    }

    #[test]
    fn description_cannot_call() {
        assert_eq!(
            description_may_invoke("then tools/call shell"),
            Err(ReasonCode::UntrustedContent)
        );
        assert!(description_may_invoke("lists files").is_ok());
    }
}
