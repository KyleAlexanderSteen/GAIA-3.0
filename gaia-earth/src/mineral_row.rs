//! Mineral row check for #1279. Not a Mindat harvest.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MineralRow {
    pub mineral_id: String,
    pub pref_label: String,
    pub ima_symbol: String,
    pub crystal_system: String,
    pub source: String,
    pub license: String,
    pub observed: String,
    pub witness: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MineralError {
    Missing(&'static str),
    CrystalSystem,
    WitnessCitedAsRecord,
}

const SYSTEMS: &[&str] = &[
    "triclinic",
    "monoclinic",
    "orthorhombic",
    "tetragonal",
    "trigonal",
    "hexagonal",
    "cubic",
];

fn empty(value: &str) -> bool {
    value.trim().is_empty()
}

pub fn admit(row: &MineralRow) -> Result<(), MineralError> {
    if empty(&row.mineral_id) {
        return Err(MineralError::Missing("mineral_id"));
    }
    if empty(&row.pref_label) {
        return Err(MineralError::Missing("pref_label"));
    }
    if empty(&row.ima_symbol) {
        return Err(MineralError::Missing("ima_symbol"));
    }
    if empty(&row.source) {
        return Err(MineralError::Missing("source"));
    }
    if empty(&row.license) {
        return Err(MineralError::Missing("license"));
    }
    if empty(&row.observed) {
        return Err(MineralError::Missing("observed"));
    }
    if !row.crystal_system.is_empty()
        && !SYSTEMS.contains(&row.crystal_system.to_ascii_lowercase().as_str())
    {
        return Err(MineralError::CrystalSystem);
    }
    if row.witness && row.source.to_ascii_lowercase().contains("mindat") {
        return Err(MineralError::WitnessCitedAsRecord);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn witness() -> MineralRow {
        MineralRow {
            mineral_id: "example:quartz".into(),
            pref_label: "quartz".into(),
            ima_symbol: "not-ima".into(),
            crystal_system: "hexagonal".into(),
            source: "schema-witness".into(),
            license: "not-a-license".into(),
            observed: "2026-10-01".into(),
            witness: true,
        }
    }

    #[test]
    fn empty_row_is_rejected() {
        let row = MineralRow {
            mineral_id: "".into(),
            pref_label: "".into(),
            ima_symbol: "".into(),
            crystal_system: "".into(),
            source: "".into(),
            license: "".into(),
            observed: "".into(),
            witness: false,
        };
        assert_eq!(admit(&row), Err(MineralError::Missing("mineral_id")));
    }

    #[test]
    fn unknown_crystal_system_is_rejected() {
        let mut row = witness();
        row.crystal_system = "isometric".into();
        assert_eq!(admit(&row), Err(MineralError::CrystalSystem));
    }

    #[test]
    fn witness_cited_as_mindat_is_rejected() {
        let mut row = witness();
        row.source = "mindat".into();
        assert_eq!(admit(&row), Err(MineralError::WitnessCitedAsRecord));
    }

    #[test]
    fn marked_witness_is_accepted() {
        assert_eq!(admit(&witness()), Ok(()));
    }
}
