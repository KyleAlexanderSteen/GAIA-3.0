//! #1076 disagreeing chunks stay NeedVerify. No auto-merge.

use gaia_aikd::tier2::{flag_hit_conflicts, RankedHit};

fn hit(text: &str, score: f32) -> RankedHit {
    RankedHit {
        hex: format!("{:064}", text.len()),
        text: text.into(),
        score,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Outcome {
    NeedVerify,
    Pass,
}

fn conflict_outcome(hits: &[RankedHit]) -> Outcome {
    if flag_hit_conflicts(hits).is_empty() {
        Outcome::Pass
    } else {
        Outcome::NeedVerify
    }
}

#[test]
fn disagreeing_years_need_verify() {
    let hits = vec![
        hit("The treaty was signed in 1992.", 0.8),
        hit("The treaty was signed in 1848.", 0.7),
    ];
    assert_eq!(conflict_outcome(&hits), Outcome::NeedVerify);
}

#[test]
fn single_chunk_no_conflict() {
    let hits = vec![hit("The treaty was signed in 1992.", 0.8)];
    assert_eq!(conflict_outcome(&hits), Outcome::Pass);
}
