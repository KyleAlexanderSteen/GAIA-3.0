//! Meta band for knowledge, skills, and powers. #1363.
//! Magic is not a meta row. Chaos and order stay outside the table.
//! Ascendence cannot be self-claimed. See docs/canon/ALCHEMICAL-ARC.md stage 5.
//! Witness rules: #1231, #1257. A report is not a finding.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Band {
    Normal,
    Super,
    Meta,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BandRow {
    pub domain: &'static str,
    pub band: Band,
    pub shelf: &'static str,
    pub grants: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Witness {
    pub outside_system: bool,
    pub registered_failure: bool,
    pub report_only: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Eligibility {
    Refused(&'static str),
    EligibleForReview,
}

pub fn catalog() -> &'static [BandRow] {
    &[
        BandRow { domain: "knowledge", band: Band::Normal, shelf: "gaia-ukd", grants: false },
        BandRow { domain: "knowledge", band: Band::Super, shelf: "gaia-aikd", grants: false },
        BandRow { domain: "knowledge", band: Band::Meta, shelf: "meta:knowledge", grants: false },
        BandRow { domain: "skills", band: Band::Normal, shelf: "gaia-skills", grants: false },
        BandRow { domain: "skills", band: Band::Super, shelf: "gaia-aisd", grants: false },
        BandRow { domain: "skills", band: Band::Meta, shelf: "meta:skills", grants: false },
        BandRow { domain: "powers", band: Band::Normal, shelf: "skill-capability", grants: false },
        BandRow { domain: "powers", band: Band::Super, shelf: "gaia-hspd", grants: false },
        BandRow { domain: "powers", band: Band::Meta, shelf: "meta:powers", grants: false },
    ]
}

pub fn meta_rows() -> impl Iterator<Item = &'static BandRow> {
    catalog().iter().filter(|row| row.band == Band::Meta)
}

pub fn grants_anything() -> bool {
    catalog().iter().any(|row| row.grants)
}

pub fn magic_is_meta_band() -> bool {
    false
}

pub fn ascendence_self_claim() -> bool {
    false
}

/// Knowing the shelf is not having the skill. #1231.
pub fn knowing_is_having() -> bool {
    false
}

/// Declaring a power is not holding it.
pub fn declare_is_holding() -> bool {
    false
}

/// A challenge to a constraint is not a grant. Insurgence is not success.
pub fn challenge_is_grant() -> bool {
    false
}

/// Entanglement is physics or relation, never both at once, and never consciousness.
pub fn entanglement_is_both() -> bool {
    false
}

/// Closing chaos into a magic band is the alchemy mistake. Refuse it.
pub fn synthesis_into_magic_band() -> Eligibility {
    Eligibility::Refused("magic is not a band")
}

/// Eligible for review is not a consciousness finding. #1257.
pub fn ascendence_eligible(witness: Witness) -> Eligibility {
    if ascendence_self_claim() {
        return Eligibility::Refused("self-claim forbidden");
    }
    if !witness.outside_system {
        return Eligibility::Refused("witness must be outside the system");
    }
    if !witness.registered_failure {
        return Eligibility::Refused("divergence needs a registered failure");
    }
    if !witness.report_only {
        return Eligibility::Refused("a report is not a finding");
    }
    Eligibility::EligibleForReview
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_domains_have_a_meta_row() {
        let domains: Vec<_> = meta_rows().map(|row| row.domain).collect();
        assert_eq!(domains, ["knowledge", "skills", "powers"]);
    }

    #[test]
    fn magic_is_not_a_meta_band() {
        assert!(!magic_is_meta_band());
        assert!(catalog().iter().all(|row| row.domain != "magic"));
        assert_eq!(synthesis_into_magic_band(), Eligibility::Refused("magic is not a band"));
    }

    #[test]
    fn limits_do_not_grant() {
        assert!(!grants_anything());
        assert!(!knowing_is_having());
        assert!(!declare_is_holding());
        assert!(!challenge_is_grant());
        assert!(!entanglement_is_both());
        assert!(!ascendence_self_claim());
    }

    #[test]
    fn ascendence_needs_an_outside_witness() {
        let missing = Witness { outside_system: false, registered_failure: true, report_only: true };
        assert_eq!(
            ascendence_eligible(missing),
            Eligibility::Refused("witness must be outside the system")
        );
        let ready = Witness { outside_system: true, registered_failure: true, report_only: true };
        assert_eq!(ascendence_eligible(ready), Eligibility::EligibleForReview);
    }
}
