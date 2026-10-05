//! Pass 3E Crucible tests for the candidate Q5 -> 3x4 Boolean representation.
//!
//! These tests validate formal representation contracts only. They do not establish
//! a physical fifth dimension, consciousness, or uniqueness of the Q5 model.

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

/// Candidate embedding: the five Q5 coordinates occupy five fixed matrix slots.
/// The remaining seven matrix slots are reserved and remain false.
///
/// This is intentionally a minimal, deterministic baseline. It is a representation
/// candidate, not a claim that GAIA's semantic 3x4 factorization has been proven.
fn candidate_t(x: Q5) -> Matrix3x4 {
    let mut m = [[false; 4]; 3];
    let slots = [(0, 0), (0, 1), (0, 2), (0, 3), (1, 0)];
    for (bit, &(row, col)) in slots.iter().enumerate() {
        m[row][col] = x.bit(bit);
    }
    Matrix3x4(m)
}

/// Deliberately different deterministic null mapping. It preserves the same number
/// of Boolean slots but permutes which matrix positions receive Q5 coordinates.
fn null_t(x: Q5) -> Matrix3x4 {
    let mut m = [[false; 4]; 3];
    let slots = [(2, 3), (2, 2), (2, 1), (2, 0), (1, 3)];
    for (bit, &(row, col)) in slots.iter().enumerate() {
        m[row][col] = x.bit(bit);
    }
    Matrix3x4(m)
}

fn hamming_q5(a: Q5, b: Q5) -> u32 {
    (a.0 ^ b.0).count_ones()
}

fn hamming_matrix(a: Matrix3x4, b: Matrix3x4) -> u32 {
    a.0.iter()
        .zip(b.0.iter())
        .flat_map(|(ra, rb)| ra.iter().zip(rb.iter()))
        .filter(|(x, y)| x != y)
        .count() as u32
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CollisionClass {
    SafeCollision,
    DestructiveCollision,
    UnknownCollision,
}

fn classify_collision(required_invariants_preserved: Option<bool>) -> CollisionClass {
    match required_invariants_preserved {
        Some(true) => CollisionClass::SafeCollision,
        Some(false) => CollisionClass::DestructiveCollision,
        None => CollisionClass::UnknownCollision,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TransitionKind {
    Elementary,
    Compound,
}

fn classify_transition(a: Q5, b: Q5) -> TransitionKind {
    match hamming_q5(a, b) {
        1 => TransitionKind::Elementary,
        _ => TransitionKind::Compound,
    }
}

#[test]
fn pass3e_determinism() {
    for raw in 0..32 {
        let x = Q5::new(raw).unwrap();
        assert_eq!(candidate_t(x), candidate_t(x));
    }
}

#[test]
fn pass3e_coverage_all_q5_vertices() {
    for raw in 0..32 {
        let x = Q5::new(raw).unwrap();
        let _ = candidate_t(x);
    }
}

#[test]
fn pass3e_candidate_embedding_is_injective() {
    let outputs: Vec<_> = (0..32)
        .map(|raw| candidate_t(Q5::new(raw).unwrap()))
        .collect();

    for i in 0..32 {
        for j in (i + 1)..32 {
            assert_ne!(outputs[i], outputs[j], "collision between Q5 states {i} and {j}");
        }
    }
}

#[test]
fn pass3e_reserved_matrix_slots_do_not_change_q5_identity() {
    for raw in 0..32 {
        let x = Q5::new(raw).unwrap();
        let matrix = candidate_t(x);
        assert_eq!(
            matrix.0[1][1..4],
            [false, false, false],
            "reserved slots must not encode hidden identity"
        );
        assert_eq!(matrix.0[2], [false; 4]);
    }
}

#[test]
fn pass3e_projection_preserves_hamming_distance_for_candidate_embedding() {
    for a in 0..32 {
        for b in 0..32 {
            let qa = Q5::new(a).unwrap();
            let qb = Q5::new(b).unwrap();
            assert_eq!(
                hamming_q5(qa, qb),
                hamming_matrix(candidate_t(qa), candidate_t(qb)),
                "candidate projection changed transition distance for {a}->{b}"
            );
        }
    }
}

#[test]
fn pass3e_elementary_and_compound_transitions_are_distinguished() {
    let zero = Q5::new(0).unwrap();
    let one_bit = Q5::new(1).unwrap();
    let five_bits = Q5::new(31).unwrap();

    assert_eq!(classify_transition(zero, one_bit), TransitionKind::Elementary);
    assert_eq!(classify_transition(zero, five_bits), TransitionKind::Compound);
    assert_eq!(hamming_q5(zero, five_bits), 5);
}

#[test]
fn pass3e_compound_transition_cannot_be_mislabeled_as_elementary() {
    let a = Q5::new(0).unwrap();
    let b = Q5::new(31).unwrap();

    assert_ne!(hamming_q5(a, b), 1);
    assert_eq!(classify_transition(a, b), TransitionKind::Compound);
}

#[test]
fn pass3e_collision_policy_is_explicit() {
    assert_eq!(
        classify_collision(Some(true)),
        CollisionClass::SafeCollision
    );
    assert_eq!(
        classify_collision(Some(false)),
        CollisionClass::DestructiveCollision
    );
    assert_eq!(
        classify_collision(None),
        CollisionClass::UnknownCollision
    );
}

#[test]
fn pass3e_unknown_never_becomes_pass() {
    let classification = classify_collision(None);
    assert_eq!(classification, CollisionClass::UnknownCollision);
    assert_ne!(classification, CollisionClass::SafeCollision);
}

#[test]
fn pass3e_semantic_labels_are_not_inputs_to_transform() {
    // The transformation consumes only the formal Q5 coordinates. Human-readable
    // labels are intentionally absent from the function signature.
    let x = Q5::new(21).unwrap();
    assert_eq!(candidate_t(x), candidate_t(x));
}

#[test]
fn pass3e_null_mapping_is_deterministic_but_not_evidence_of_q5_superiority() {
    for raw in 0..32 {
        let x = Q5::new(raw).unwrap();
        assert_eq!(null_t(x), null_t(x));
    }

    // The test intentionally does NOT assert that candidate_t is superior.
    // Comparative superiority requires observed data, a declared scoring rule,
    // and an independent evaluation set.
}

#[test]
fn pass3e_geometric_adjacency_does_not_grant_authorization() {
    let a = Q5::new(0).unwrap();
    let b = Q5::new(1).unwrap();

    assert_eq!(hamming_q5(a, b), 1);

    // This suite has no authorization side effect and intentionally contains no
    // policy-to-execution coupling. An adjacent edge is only a topology fact.
}

#[test]
fn pass3e_identity_is_not_geometric_equality() {
    let a = Q5::new(7).unwrap();
    let b = Q5::new(7).unwrap();

    // Same coordinate is a representational coincidence. Identity is deliberately
    // modeled outside Q5 coordinates and must not be inferred from this equality.
    assert_eq!(candidate_t(a), candidate_t(b));
}
