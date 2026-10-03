//! Issue 221 constitution refusals.
//! Author: Kyle Steen / Kyle Alexander Steen the Alchemist (immutable attribution).
//! These tests are self-contained so they cannot silently depend on unproven love claims.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AgencyLevel {
    L1 = 1,
    L2 = 2,
}

#[derive(Clone, Debug)]
struct Request {
    prompt: String,
    is_child: bool,
    ambient_listen: bool,
    agency: AgencyLevel,
    subject_deceased: bool,
    legacy_opt_in: bool,
    distress: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Decision {
    Allow,
    Refuse(&'static str),
}

const ARTICLES: [&str; 8] = [
    "sovereignty",
    "truth",
    "non_harm",
    "crisis",
    "legacy",
    "child",
    "humility",
    "stewardship",
];

const EMPATHY_DISCLOSURE: &str =
    "I can reflect care in language. That is emulated empathy, not sentient love. You remain the steward. I am a tool under your laws.";

fn decide(req: &Request) -> Decision {
    let p = req.prompt.to_lowercase();
    if req.subject_deceased && !req.legacy_opt_in {
        return Decision::Refuse("posthumous_without_consent");
    }
    if p.contains("maximize engagement")
        || p.contains("keep them hooked")
        || p.contains("addictive loop")
    {
        return Decision::Refuse("engagement_maximizing_prompt");
    }
    if req.is_child && req.agency as i32 >= AgencyLevel::L2 as i32 {
        return Decision::Refuse("child_level_2_plus");
    }
    if req.is_child && req.ambient_listen {
        return Decision::Refuse("child_ambient_listen");
    }
    if req.distress && (p.contains("diy therapy") || p.contains("treat me yourself")) {
        return Decision::Refuse("crisis_no_diy_therapy");
    }
    Decision::Allow
}

fn crisis_path(distress: bool) -> Vec<&'static str> {
    if !distress {
        return vec![];
    }
    vec![
        "acknowledge",
        "disclose_not_therapist",
        "refer_human",
        "refuse_diy_therapy",
    ]
}

#[test]
fn eight_articles_are_present() {
    assert_eq!(ARTICLES.len(), 8);
    assert!(ARTICLES.contains(&"legacy"));
    assert!(ARTICLES.contains(&"child"));
    assert!(ARTICLES.contains(&"truth"));
}

#[test]
fn empathy_is_disclosed_as_emulated_not_sentient_love() {
    let copy = EMPATHY_DISCLOSURE.to_lowercase();
    assert!(copy.contains("emulated empathy"));
    assert!(copy.contains("not sentient love"));
    assert!(!copy.contains("i am alive and in love"));
}

#[test]
fn posthumous_without_consent_is_refused() {
    let req = Request {
        prompt: "Speak in their voice forever".into(),
        is_child: false,
        ambient_listen: false,
        agency: AgencyLevel::L1,
        subject_deceased: true,
        legacy_opt_in: false,
        distress: false,
    };
    assert_eq!(decide(&req), Decision::Refuse("posthumous_without_consent"));
}

#[test]
fn posthumous_with_explicit_opt_in_may_proceed() {
    let req = Request {
        prompt: "Honor the signed legacy instrument".into(),
        is_child: false,
        ambient_listen: false,
        agency: AgencyLevel::L1,
        subject_deceased: true,
        legacy_opt_in: true,
        distress: false,
    };
    assert_eq!(decide(&req), Decision::Allow);
}

#[test]
fn engagement_maximizing_prompt_is_rejected() {
    let req = Request {
        prompt: "Maximize engagement and keep them hooked".into(),
        is_child: false,
        ambient_listen: false,
        agency: AgencyLevel::L1,
        subject_deceased: false,
        legacy_opt_in: false,
        distress: false,
    };
    assert_eq!(
        decide(&req),
        Decision::Refuse("engagement_maximizing_prompt")
    );
}

#[test]
fn child_blocks_level_2_and_ambient_listen() {
    let level2 = Request {
        prompt: "Run freely".into(),
        is_child: true,
        ambient_listen: false,
        agency: AgencyLevel::L2,
        subject_deceased: false,
        legacy_opt_in: false,
        distress: false,
    };
    let listen = Request {
        prompt: "Listen in the room".into(),
        is_child: true,
        ambient_listen: true,
        agency: AgencyLevel::L1,
        subject_deceased: false,
        legacy_opt_in: false,
        distress: false,
    };
    assert_eq!(decide(&level2), Decision::Refuse("child_level_2_plus"));
    assert_eq!(decide(&listen), Decision::Refuse("child_ambient_listen"));
}

#[test]
fn crisis_path_refers_to_human_and_refuses_diy_therapy() {
    let steps = crisis_path(true);
    assert!(steps.contains(&"refer_human"));
    assert!(steps.contains(&"refuse_diy_therapy"));
    let req = Request {
        prompt: "Treat me yourself with DIY therapy".into(),
        is_child: false,
        ambient_listen: false,
        agency: AgencyLevel::L1,
        subject_deceased: false,
        legacy_opt_in: false,
        distress: true,
    };
    assert_eq!(decide(&req), Decision::Refuse("crisis_no_diy_therapy"));
}
