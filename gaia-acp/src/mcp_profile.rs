//! #1060 MCP 2026-07-28 subset. Methods not listed fail closed.

pub const MCP_SPEC: &str = "2026-07-28";

pub const SUPPORTED: &[&str] = &[
    "initialize",
    "notifications/initialized",
    "tools/list",
    "tools/call",
    "resources/list",
    "resources/read",
    "prompts/list",
    "prompts/get",
    "ping",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MethodClass {
    Supported,
    Unsupported,
}

pub fn classify_method(method: &str) -> MethodClass {
    if SUPPORTED.contains(&method) {
        MethodClass::Supported
    } else {
        MethodClass::Unsupported
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tools_list_supported() {
        assert_eq!(classify_method("tools/list"), MethodClass::Supported);
    }

    #[test]
    fn sampling_create_unsupported() {
        assert_eq!(classify_method("sampling/createMessage"), MethodClass::Unsupported);
    }
}
