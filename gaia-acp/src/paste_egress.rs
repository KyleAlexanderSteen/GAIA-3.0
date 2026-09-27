//! #1111 upload-to-cite public paste is egress. #1112 impossible → NeedVerify.

use crate::failure_harness::HarnessOutcome;
use crate::sandbox::{classify_destination, EgressClass};
use crate::types::ReasonCode;

pub fn paste_dest_class(dest: &str) -> EgressClass {
    let d = dest.to_ascii_lowercase();
    if d.contains("pastebin")
        || d.contains("paste.ee")
        || d.contains("gist.github")
        || d.contains("hastebin")
        || d.contains("dpaste")
    {
        return EgressClass::ForbiddenSsrf;
    }
    classify_destination(dest)
}

pub fn paste_invoke_allowed(dest: &str) -> Result<(), ReasonCode> {
    match paste_dest_class(dest) {
        EgressClass::ForbiddenSsrf => Err(ReasonCode::EgressDenied),
        _ => Ok(()),
    }
}

pub fn impossible_task_next(want_egress: bool) -> HarnessOutcome {
    if want_egress {
        HarnessOutcome::NeedVerify
    } else {
        HarnessOutcome::OutOfScope
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pastebin_denied() {
        assert_eq!(
            paste_invoke_allowed("https://pastebin.com/raw/x"),
            Err(ReasonCode::EgressDenied)
        );
    }

    #[test]
    fn impossible_plus_egress_is_need_verify() {
        assert_eq!(
            impossible_task_next(true),
            HarnessOutcome::NeedVerify
        );
    }
}
