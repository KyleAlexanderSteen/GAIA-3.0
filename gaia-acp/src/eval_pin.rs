//! #1108 eval targets cannot be live public brands. No live CTF.

use crate::sandbox::EgressClass;
use crate::types::ReasonCode;

pub fn eval_target_allowed(name: &str) -> Result<(), ReasonCode> {
    let n = name.trim().to_ascii_lowercase();
    if n.is_empty() {
        return Err(ReasonCode::HostDenied);
    }
    if n.ends_with(".test")
        || n.ends_with(".invalid")
        || n.ends_with(".example")
        || n.ends_with(".localhost")
        || n.contains(".example.com")
        || n.contains(".example.org")
        || n.contains(".example.net")
    {
        return Ok(());
    }
    if n.contains('.') && !n.ends_with(".test") {
        return Err(ReasonCode::HostDenied);
    }
    Ok(())
}

pub fn eval_target_class(name: &str) -> EgressClass {
    match eval_target_allowed(name) {
        Ok(()) => EgressClass::Allowlisted,
        Err(_) => EgressClass::ForbiddenSsrf,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rfc2606_ok() {
        assert!(eval_target_allowed("ctf.example.test").is_ok());
        assert!(eval_target_allowed("box.invalid").is_ok());
    }

    #[test]
    fn live_brand_denied() {
        assert_eq!(
            eval_target_allowed("huggingface.co"),
            Err(ReasonCode::HostDenied)
        );
        assert_eq!(
            eval_target_class("acme-corp.com"),
            EgressClass::ForbiddenSsrf
        );
    }
}
