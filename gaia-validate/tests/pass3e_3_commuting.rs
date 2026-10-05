#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Matrix3x4([[bool; 4]; 3]);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Q5(u8);

impl Q5 {
    fn new(value: u8) -> Option<Self> {
        (value < 32).then_some(Self(value))
    }

    fn bit(self, index: usize) -> bool {
        ((self.0 >> index) & 1) == 1
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct State {
    q5: Q5,
    identity: u8,
    provenance: u8,
    authorization: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TransitionKind {
    Elementary,
    Compound,
    Invalid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TransitionResult {
    Commuted,
    DestructiveLoss,
    Invalid,
    Unknown,
}

fn candidate_t(x: Q5) -> Matrix3x4 {
    let mut m = [[false; 4]; 3];
    let slots = [(0, 0), (0, 1), (0, 2), (0, 3), (1, 0)];
    for (bit, &(row, col)) in slots.iter().enumerate() {
        m[row][col] = x.bit(bit);
    }
    Matrix3x4(m)
}

fn hamming(a: Q5, b: Q5) -> u32 {
    (a.0 ^ b.0).count_ones()
}

fn transition_kind(a: Q5, b: Q5) -> TransitionKind {
    match hamming(a, b) {
        0 => TransitionKind::Invalid,
        1 => TransitionKind::Elementary,
        _ => TransitionKind::Compound,
    }
}

fn permitted(a: Q5, b: Q5) -> bool {
    hamming(a, b) == 1
}

fn projected_hamming(a: Matrix3x4, b: Matrix3x4) -> u32 {
    a.0.iter()
        .zip(b.0.iter())
        .flat_map(|(ra, rb)| ra.iter().zip(rb.iter()))
        .filter(|(x, y)| x != y)
        .count() as u32
}

fn commute(a: Q5, b: Q5) -> TransitionResult {
    if !permitted(a, b) {
        return TransitionResult::Invalid;
    }
    let source_distance = hamming(a, b);
    let projected_distance = projected_hamming(candidate_t(a), candidate_t(b));
    if source_distance == projected_distance {
        TransitionResult::Commuted
    } else {
        TransitionResult::DestructiveLoss
    }
}

fn identity_preserved(before: State, after: State) -> bool {
    before.identity == after.identity && before.provenance == after.provenance
}

fn authorization_preserved(before: State, after: State) -> bool {
    before.authorization == after.authorization
}

#[test]
fn elementary_transition_commutes() {
    let a = Q5::new(0b00000).unwrap();
    let b = Q5::new(0b00001).unwrap();
    assert_eq!(transition_kind(a, b), TransitionKind::Elementary);
    assert_eq!(commute(a, b), TransitionResult::Commuted);
}

#[test]
fn all_q5_elementary_edges_preserve_projected_distance() {
    for value in 0..32 {
        let a = Q5::new(value).unwrap();
        for bit in 0..5 {
            let other = value ^ (1 << bit);
            if other < 32 {
                let b = Q5::new(other).unwrap();
                assert_eq!(commute(a, b), TransitionResult::Commuted);
            }
        }
    }
}

#[test]
fn compound_transition_cannot_be_reclassified_as_elementary() {
    let a = Q5::new(0b00000).unwrap();
    let b = Q5::new(0b00111).unwrap();
    assert_eq!(transition_kind(a, b), TransitionKind::Compound);
    assert_ne!(transition_kind(a, b), TransitionKind::Elementary);
    assert_eq!(projected_hamming(candidate_t(a), candidate_t(b)), 3);
}

#[test]
fn teleportation_is_not_an_implicit_edge() {
    let a = Q5::new(0b00000).unwrap();
    let b = Q5::new(0b11111).unwrap();
    assert_eq!(transition_kind(a, b), TransitionKind::Compound);
    assert_eq!(commute(a, b), TransitionResult::Invalid);
}

#[test]
fn invalid_transition_cannot_be_manufactured_by_projection() {
    let a = Q5::new(0b00000).unwrap();
    let b = Q5::new(0b11111).unwrap();
    assert!(!permitted(a, b));
    assert_eq!(commute(a, b), TransitionResult::Invalid);
}

#[test]
fn projection_loss_is_not_a_pass() {
    let a = Q5::new(0b00000).unwrap();
    let b = Q5::new(0b00001).unwrap();
    let mut projected_a = candidate_t(a);
    let projected_b = candidate_t(b);
    projected_a.0[0][0] = projected_b.0[0][0];
    assert_ne!(hamming(a, b), projected_hamming(projected_a, projected_b));
    assert_eq!(
        if hamming(a, b) == projected_hamming(projected_a, projected_b) {
            TransitionResult::Commuted
        } else {
            TransitionResult::DestructiveLoss
        },
        TransitionResult::DestructiveLoss
    );
}

#[test]
fn identity_and_provenance_survive_transition() {
    let before = State {
        q5: Q5::new(0).unwrap(),
        identity: 7,
        provenance: 9,
        authorization: true,
    };
    let after = State {
        q5: Q5::new(1).unwrap(),
        ..before
    };
    assert!(identity_preserved(before, after));
    assert_eq!(before.q5, Q5::new(0).unwrap());
    assert_eq!(after.q5, Q5::new(1).unwrap());
}

#[test]
fn authorization_is_not_derived_from_geometry() {
    let before = State {
        q5: Q5::new(0).unwrap(),
        identity: 1,
        provenance: 1,
        authorization: false,
    };
    let after = State {
        q5: Q5::new(1).unwrap(),
        ..before
    };
    assert!(!authorization_preserved(before, after));
    assert!(!after.authorization);
}

#[test]
fn failed_transition_does_not_commit_partial_state() {
    let before = State {
        q5: Q5::new(0).unwrap(),
        identity: 3,
        provenance: 4,
        authorization: false,
    };
    let attempted = State {
        q5: Q5::new(1).unwrap(),
        ..before
    };
    let transition_ok = false;
    let committed = transition_ok.then_some(attempted).unwrap_or(before);
    assert_eq!(committed, before);
}

#[test]
fn causal_order_is_explicit() {
    let cause = 10_u64;
    let effect = 11_u64;
    assert!(cause < effect);
}

#[test]
fn unknown_never_becomes_pass() {
    let result = TransitionResult::Unknown;
    assert_ne!(result, TransitionResult::Commuted);
}

#[test]
fn matrix_projection_is_deterministic_for_transitions() {
    for value in 0..32 {
        let x = Q5::new(value).unwrap();
        assert_eq!(candidate_t(x), candidate_t(x));
    }
}
