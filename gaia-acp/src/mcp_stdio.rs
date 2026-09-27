//! #1061 local JSON-RPC line frames. No network. Fake adapter stays.

use serde_json::Value;

pub const MAX_FRAME: usize = 64 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StdioError {
    Empty,
    TooLarge,
    NotJson,
    NotObject,
    MissingMethod,
}

pub fn parse_frame(line: &str) -> Result<Value, StdioError> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return Err(StdioError::Empty);
    }
    if trimmed.len() > MAX_FRAME {
        return Err(StdioError::TooLarge);
    }
    let v: Value = serde_json::from_str(trimmed).map_err(|_| StdioError::NotJson)?;
    let obj = v.as_object().ok_or(StdioError::NotObject)?;
    if !obj.contains_key("method") && !obj.contains_key("result") && !obj.contains_key("error") {
        return Err(StdioError::MissingMethod);
    }
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn good_tools_list() {
        let v = parse_frame(r#"{"jsonrpc":"2.0","id":1,"method":"tools/list"}"#).unwrap();
        assert_eq!(v["method"], "tools/list");
    }

    #[test]
    fn oversized_fails() {
        let s = format!("{{\"method\":\"{}\"}}", "x".repeat(MAX_FRAME));
        assert_eq!(parse_frame(&s), Err(StdioError::TooLarge));
    }

    #[test]
    fn junk_fails() {
        assert_eq!(parse_frame("not-json"), Err(StdioError::NotJson));
    }
}
