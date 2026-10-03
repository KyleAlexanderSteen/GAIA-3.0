//! Local MCP frame. HTTP and OAuth stay off.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tool {
    pub name: String,
    pub schema_hash: u64,
}

pub fn accept_transport(kind: &str) -> Result<(), &'static str> {
    match kind {
        "stdio" => Ok(()),
        "http" | "oauth" => Err("remote transport is not enabled"),
        _ => Err("unknown transport"),
    }
}

pub fn parse_frame(line: &str) -> Result<&'static str, &'static str> {
    if line.contains("\"method\":\"initialize\"") {
        Ok("initialize")
    } else if line.contains("\"method\":\"tools/list\"") {
        Ok("tools/list")
    } else {
        Err("frame was not recognized")
    }
}

pub fn select(tools: &[Tool], budget: usize) -> Vec<&Tool> {
    tools.iter().take(budget).collect()
}

pub fn drifted(seen: u64, current: u64) -> bool {
    seen != current
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stdio_is_local_and_a_schema_change_is_drift() {
        assert!(accept_transport("stdio").is_ok());
        assert_eq!(accept_transport("http"), Err("remote transport is not enabled"));
        assert_eq!(parse_frame("{\"method\":\"initialize\"}"), Ok("initialize"));
        let tools = [Tool { name: "read".into(), schema_hash: 1 }, Tool { name: "write".into(), schema_hash: 2 }];
        assert_eq!(select(&tools, 1).len(), 1);
        assert!(drifted(1, 2));
    }
}
