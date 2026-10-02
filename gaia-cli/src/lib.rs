//! Thin lib surface so `cargo test --lib -p gaia-cli` has something to run.
//! The binary stays `src/main.rs`. No live gateway calls.

pub fn bin_name() -> &'static str {
    "gaia-cli"
}

pub fn listed_commands() -> &'static [&'static str] {
    &["init", "start", "agent", "intent", "memory", "audit", "revoke", "bands"]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bin_name_is_gaia_cli() {
        assert_eq!(bin_name(), "gaia-cli");
    }

    #[test]
    fn eight_listed_commands() {
        assert_eq!(listed_commands().len(), 8);
        assert!(listed_commands().contains(&"init"));
        assert!(listed_commands().contains(&"revoke"));
        assert!(listed_commands().contains(&"bands"));
    }

    #[test]
    fn listed_commands_are_unique() {
        let mut v = listed_commands().to_vec();
        v.sort();
        v.dedup();
        assert_eq!(v.len(), listed_commands().len());
    }
}
