//! Issue 64 privacy constitution and age-gate.
//! Author: Kyle Steen / Kyle Alexander Steen the Alchemist (immutable attribution).

const SCOPES: [&str; 7] = [
    "face",
    "body",
    "voice",
    "health",
    "memory",
    "agent",
    "earth_twin_share",
];

const PRINCIPLES: [&str; 10] = [
    "consent",
    "local_default",
    "biometric_sovereignty",
    "user_held_keys",
    "deletion",
    "transparency",
    "purpose_limitation",
    "non_weaponization",
    "equity",
    "child_protection",
];

#[derive(Clone, Debug)]
struct Subject {
    age: u8,
    guardian_consent: bool,
}

#[derive(Clone, Debug)]
struct Enable {
    phase3_personality_learning: bool,
    behavioral_profiling: bool,
    health_twin_learning: bool,
    third_party_likeness: bool,
    hidden_copy: bool,
    insurer_or_employer_export: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Decision {
    Allow,
    Refuse(&'static str),
}

fn is_child(s: &Subject) -> bool {
    s.age < 16
}

fn decide(s: &Subject, e: &Enable) -> Decision {
    if is_child(s) && !s.guardian_consent {
        return Decision::Refuse("guardian_consent_required");
    }
    if is_child(s) && e.phase3_personality_learning {
        return Decision::Refuse("child_phase3_personality_learning");
    }
    if is_child(s) && e.behavioral_profiling {
        return Decision::Refuse("child_behavioral_profiling");
    }
    if is_child(s) && e.health_twin_learning {
        return Decision::Refuse("child_health_twin_learning");
    }
    if e.third_party_likeness {
        return Decision::Refuse("third_party_likeness");
    }
    if e.hidden_copy {
        return Decision::Refuse("hidden_copies");
    }
    if e.insurer_or_employer_export {
        return Decision::Refuse("insurer_employer_health_export");
    }
    Decision::Allow
}

#[test]
fn ten_principles_and_seven_scopes() {
    assert_eq!(PRINCIPLES.len(), 10);
    assert_eq!(SCOPES.len(), 7);
    assert!(SCOPES.contains(&"health"));
    assert!(SCOPES.contains(&"earth_twin_share"));
}

#[test]
fn under_16_needs_verifiable_guardian_consent() {
    let child = Subject {
        age: 15,
        guardian_consent: false,
    };
    let ok_child = Subject {
        age: 15,
        guardian_consent: true,
    };
    let adult = Subject {
        age: 16,
        guardian_consent: false,
    };
    let none = Enable {
        phase3_personality_learning: false,
        behavioral_profiling: false,
        health_twin_learning: false,
        third_party_likeness: false,
        hidden_copy: false,
        insurer_or_employer_export: false,
    };
    assert_eq!(
        decide(&child, &none),
        Decision::Refuse("guardian_consent_required")
    );
    assert_eq!(decide(&ok_child, &none), Decision::Allow);
    assert_eq!(decide(&adult, &none), Decision::Allow);
}

#[test]
fn child_path_cannot_enable_phase3_personality_learning() {
    let child = Subject {
        age: 12,
        guardian_consent: true,
    };
    let e = Enable {
        phase3_personality_learning: true,
        behavioral_profiling: false,
        health_twin_learning: false,
        third_party_likeness: false,
        hidden_copy: false,
        insurer_or_employer_export: false,
    };
    assert_eq!(
        decide(&child, &e),
        Decision::Refuse("child_phase3_personality_learning")
    );
}

#[test]
fn child_blocks_profiling_and_health_twin_learning() {
    let child = Subject {
        age: 10,
        guardian_consent: true,
    };
    let profile = Enable {
        phase3_personality_learning: false,
        behavioral_profiling: true,
        health_twin_learning: false,
        third_party_likeness: false,
        hidden_copy: false,
        insurer_or_employer_export: false,
    };
    let health = Enable {
        phase3_personality_learning: false,
        behavioral_profiling: false,
        health_twin_learning: true,
        third_party_likeness: false,
        hidden_copy: false,
        insurer_or_employer_export: false,
    };
    assert_eq!(
        decide(&child, &profile),
        Decision::Refuse("child_behavioral_profiling")
    );
    assert_eq!(
        decide(&child, &health),
        Decision::Refuse("child_health_twin_learning")
    );
}

#[test]
fn forbids_likeness_hidden_copies_and_health_export() {
    let adult = Subject {
        age: 40,
        guardian_consent: false,
    };
    let likeness = Enable {
        phase3_personality_learning: false,
        behavioral_profiling: false,
        health_twin_learning: false,
        third_party_likeness: true,
        hidden_copy: false,
        insurer_or_employer_export: false,
    };
    let hidden = Enable {
        phase3_personality_learning: false,
        behavioral_profiling: false,
        health_twin_learning: false,
        third_party_likeness: false,
        hidden_copy: true,
        insurer_or_employer_export: false,
    };
    let export = Enable {
        phase3_personality_learning: false,
        behavioral_profiling: false,
        health_twin_learning: false,
        third_party_likeness: false,
        hidden_copy: false,
        insurer_or_employer_export: true,
    };
    assert_eq!(
        decide(&adult, &likeness),
        Decision::Refuse("third_party_likeness")
    );
    assert_eq!(decide(&adult, &hidden), Decision::Refuse("hidden_copies"));
    assert_eq!(
        decide(&adult, &export),
        Decision::Refuse("insurer_employer_health_export")
    );
}
