//! SI shelf. A name is not a capability.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SiRow {
    pub id: &'static str,
    pub demonstrated: bool,
    pub grants: bool,
}

pub fn shelf() -> &'static [SiRow] {
    &[
        SiRow { id: "rename", demonstrated: true, grants: false },
        SiRow { id: "superintelligence", demonstrated: false, grants: false },
        SiRow { id: "synthetic", demonstrated: false, grants: false },
    ]
}

pub fn grants_capability() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shelf_names_three_meanings_and_grants_nothing() {
        assert_eq!(shelf().len(), 3);
        assert!(shelf().iter().all(|row| !row.grants));
        assert!(shelf().iter().any(|row| row.id == "rename" && row.demonstrated));
        assert!(!grants_capability());
    }
}
