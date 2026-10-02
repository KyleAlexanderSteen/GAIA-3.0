//! Meta band for knowledge, skills, and powers. #1363.
//! Magic is not a meta row. It is the totality the bands sit inside:
//! chaos and order, not a third shelf that grants consciousness.
//! Ascendence cannot be self-claimed. See docs/canon/ALCHEMICAL-ARC.md stage 5.

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

/// Magic is outside the band table. Chaos (#1175, #1204, #1205) and order
/// (#1176, #1207, #1208) are the research that keeps it there.
pub fn magic_is_meta_band() -> bool {
    false
}

/// Ascendence is independent confirmation. This runtime must not claim it.
pub fn ascendence_self_claim() -> bool {
    false
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
    }

    #[test]
    fn meta_band_grants_nothing_and_cannot_self_claim_ascendence() {
        assert!(!grants_anything());
        assert!(!ascendence_self_claim());
        for row in meta_rows() {
            assert!(!row.grants, "{}", row.domain);
        }
    }
}
