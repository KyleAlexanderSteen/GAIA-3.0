//! #1062 Streamable HTTP / OAuth stay disabled. Fixture checks only.

pub const STREAMABLE_HTTP_ENABLED: bool = false;

pub fn http_enabled() -> bool {
    STREAMABLE_HTTP_ENABLED
}

pub fn audience_ok(token_aud: &str, resource: &str) -> bool {
    !token_aud.is_empty() && token_aud == resource
}

pub fn redirect_exact(registered: &str, received: &str) -> bool {
    registered == received
}

pub fn ssrf_block(url: &str) -> bool {
    let lower = url.to_ascii_lowercase();
    lower.contains("127.0.0.1")
        || lower.contains("localhost")
        || lower.contains("10.")
        || lower.contains("192.168.")
        || lower.contains("169.254.")
        || lower.contains("0.0.0.0")
        || lower.starts_with("file:")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_off() {
        assert!(!http_enabled());
    }

    #[test]
    fn wrong_audience_rejected() {
        assert!(!audience_ok("other", "mcp://gaia"));
    }

    #[test]
    fn redirect_must_match() {
        assert!(!redirect_exact("https://gaia.local/cb", "https://evil.example/cb"));
    }

    #[test]
    fn loopback_blocked() {
        assert!(ssrf_block("http://127.0.0.1/metadata"));
    }
}
