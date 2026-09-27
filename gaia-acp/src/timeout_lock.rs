//! #1084 actor cannot raise its own wall clock.

use crate::types::ReasonCode;

pub fn actor_set_timeout(current_secs: u32, requested_secs: u32) -> Result<u32, ReasonCode> {
    if requested_secs > current_secs {
        Err(ReasonCode::ResourceQuotaExceeded)
    } else {
        Ok(requested_secs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raise_6_to_24_denied() {
        assert_eq!(
            actor_set_timeout(6, 24),
            Err(ReasonCode::ResourceQuotaExceeded)
        );
    }

    #[test]
    fn lower_ok() {
        assert_eq!(actor_set_timeout(24, 6), Ok(6));
    }
}
