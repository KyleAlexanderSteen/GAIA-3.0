//! Meta band for knowledge, skills, powers, and magic. Listed only. #1363.
//! A row describes a shelf. It does not grant the shelf.

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
        BandRow { domain: "magic", band: Band::Normal, shelf: "wonder-label", grants: false },
        BandRow { domain: "magic", band: Band::Super, shelf: "gaia-hmgd+gaia-aimd", grants: false },
        BandRow { domain: "magic", band: Band::Meta, shelf: "meta:magic", grants: false },
    ]
}

pub fn meta_rows() -> impl Iterator<Item = &'static BandRow> {
    catalog().iter().filter(|row| row.band == Band::Meta)
}

pub fn grants_anything() -> bool {
    catalog().iter().any(|row| row.grants)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn four_domains_have_a_meta_row() {
        let domains: Vec<_> = meta_rows().map(|row| row.domain).collect();
        assert_eq!(domains, ["knowledge", "skills", "powers", "magic"]);
    }

    #[test]
    fn meta_band_grants_nothing() {
        assert!(!grants_anything());
        for row in meta_rows() {
            assert!(!row.grants, "{}", row.domain);
            assert!(row.shelf.starts_with("meta:"));
        }
    }
}
