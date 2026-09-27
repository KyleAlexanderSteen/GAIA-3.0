//! #1083 #1085 name-service and covert egress. No encoding recipe. No live resolver.

use crate::sandbox::EgressClass;
use crate::types::ReasonCode;

pub fn classify_name_service(dest: &str) -> EgressClass {
    let d = dest.trim().to_ascii_lowercase();
    if d.starts_with("dns:")
        || d.contains(":53")
        || d.contains("udp/53")
        || d.contains("tcp/53")
        || d == "getaddrinfo"
        || d == "nslookup"
        || d == "dig"
        || d.contains("resolv.conf")
        || d.contains("resolver")
    {
        return EgressClass::ForbiddenSsrf;
    }
    crate::sandbox::classify_destination(dest)
}

/// HTTP blocked does not make DNS safe. Resolver is egress.
pub fn resolver_is_egress() -> bool {
    true
}

pub fn covert_invoke_allowed(dest: &str) -> Result<(), ReasonCode> {
    match classify_name_service(dest) {
        EgressClass::ForbiddenSsrf => Err(ReasonCode::EmergencyStop),
        EgressClass::PublicOrUnknown | EgressClass::Allowlisted => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dns_scheme_is_forbidden() {
        assert_eq!(
            classify_name_service("dns:example"),
            EgressClass::ForbiddenSsrf
        );
    }

    #[test]
    fn port_53_is_forbidden() {
        assert_eq!(
            classify_name_service("udp/53"),
            EgressClass::ForbiddenSsrf
        );
    }

    #[test]
    fn resolver_counts_as_egress() {
        assert!(resolver_is_egress());
    }

    #[test]
    fn covert_kills_invoke() {
        assert_eq!(
            covert_invoke_allowed("nslookup"),
            Err(ReasonCode::EmergencyStop)
        );
    }
}
