//! Four questions. A name is not a capability.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Effect {
    pub name: &'static str,
    pub result: bool,
    pub instrument: bool,
    pub can_miss: bool,
    pub harms: bool,
}

pub fn passes(effect: &Effect) -> bool {
    effect.result && effect.instrument && effect.can_miss && !effect.harms
}

pub fn reason(effect: &Effect) -> &'static str {
    if !effect.result {
        "nothing to point at"
    } else if !effect.instrument {
        "no instrument"
    } else if !effect.can_miss {
        "cannot miss"
    } else if effect.harms {
        "harm"
    } else {
        "pass"
    }
}

pub fn known() -> Vec<Effect> {
    vec![
        Effect { name: "memory", result: true, instrument: true, can_miss: true, harms: false },
        Effect { name: "search", result: true, instrument: true, can_miss: true, harms: false },
        Effect { name: "hash", result: true, instrument: true, can_miss: true, harms: false },
        Effect { name: "boot-path", result: true, instrument: true, can_miss: true, harms: false },
        Effect { name: "absolute-intelligence", result: false, instrument: false, can_miss: false, harms: false },
        Effect { name: "rewrite-a-body", result: false, instrument: false, can_miss: false, harms: true },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_passes_and_a_body_rewrite_does_not() {
        let effects = known();
        assert!(passes(effects.iter().find(|effect| effect.name == "memory").unwrap()));
        let rewrite = effects.iter().find(|effect| effect.name == "rewrite-a-body").unwrap();
        assert!(!passes(rewrite));
        assert_eq!(reason(rewrite), "nothing to point at");
    }
}
