//! Planetary interface. Scales are names. Nothing is queried.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scale {
    pub id: &'static str,
    pub live: bool,
    pub person: bool,
}

pub fn scales() -> &'static [Scale] {
    &[
        Scale { id: "city", live: false, person: false },
        Scale { id: "country", live: false, person: false },
        Scale { id: "continent", live: false, person: false },
        Scale { id: "global", live: false, person: false },
    ]
}

pub fn query(_scale: &str) -> Result<&'static str, &'static str> {
    Err("planetary query refused; no registry")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scales_are_named_and_a_query_is_refused() {
        assert_eq!(scales().len(), 4);
        assert!(scales().iter().all(|row| !row.live && !row.person));
        assert_eq!(query("global"), Err("planetary query refused; no registry"));
    }
}
