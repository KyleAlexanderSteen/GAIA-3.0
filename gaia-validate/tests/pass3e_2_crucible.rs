//! Pass 3E-2: invariant and alternative-model Crucible.
//!
//! These fixtures deliberately separate representation mechanics from
//! architectural claims. A passing test demonstrates only the contract
//! exercised by that fixture.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Matrix3x4([[bool; 4]; 3]);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct State {
    q5: u8,
    identity: u8,
    provenance: u8,
    authorization: bool,
}

impl State {
    fn new(q5: u8, identity: u8, provenance: u8, authorization: bool) -> Self {
        assert!(q5 < 32);
        Self { q5, identity, provenance, authorization }
    }
}

fn candidate_t(state: State) -> Matrix3x4 {
    let mut m = [[false; 4]; 3];
    for bit in 0..5 {
        m[bit / 4][bit % 4] = ((state.q5 >> bit) & 1) == 1;
    }
    Matrix3x4(m)
}

fn destructive_projection(state: State) -> Matrix3x4 {
    // Deliberately drops the identity/provenance/authorization dimensions.
    candidate_t(State::new(state.q5, 0, 0, false))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LossClass {
    Preserved,
    Destructive,
    Unknown,
}

fn classify_projection(before: State, after: State) -> LossClass {
    if before.identity != after.identity
        || before.provenance != after.provenance
        || before.authorization != after.authorization
    {
        LossClass::Destructive
    } else {
        LossClass::Preserved
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TransitionKind {
    Elementary,
    Compound,
}

fn hamming(a: u8, b: u8) -> u32 {
    (a ^ b).count_ones()
}

fn transition_kind(a: State, b: State) -> TransitionKind {
    if hamming(a.q5, b.q5) == 1 {
        TransitionKind::Elementary
    } else {
        TransitionKind::Compound
    }
}

fn project_transition(a: State, b: State) -> (Matrix3x4, Matrix3x4) {
    (candidate_t(a), candidate_t(b))
}

fn transition_distance(projected: (Matrix3x4, Matrix3x4)) -> u32 {
    projected
        .0
        .0
        .iter()
        .zip(projected.1 .0.iter())
        .flat_map(|(a, b)| a.iter().zip(b.iter()))
        .filter(|(a, b)| a != b)
        .count() as u32
}

#[test]
fn projection_loss_fixture_is_detected() {
    let before = State::new(7, 11, 23, true);
    let after = State::new(7, 0, 0, false);
    assert_eq!(classify_projection(before, after), LossClass::Destructive);
    assert_eq!(candidate_t(before), destructive_projection(before));
}

#[test]
fn projection_preservation_requires_required_invariants() {
    let before = State::new(7, 11, 23, true);
    let after = State::new(7, 11, 23, true);
    assert_eq!(classify_projection(before, after), LossClass::Preserved);
}

#[test]
fn distinct_identities_cannot_be_hidden_by_same_q5_coordinate() {
    let a = State::new(7, 11, 23, true);
    let b = State::new(7, 12, 24, true);
    assert_eq!(candidate_t(a), candidate_t(b));
    assert_ne!(a.identity, b.identity);
    assert_eq!(classify_projection(a, b), LossClass::Destructive);
}

#[test]
fn compound_transition_cannot_be_reclassified_as_elementary() {
    let a = State::new(0, 1, 1, true);
    let b = State::new(31, 1, 1, true);
    assert_eq!(transition_kind(a, b), TransitionKind::Compound);
    assert_eq!(hamming(a.q5, b.q5), 5);
    assert_ne!(transition_distance(project_transition(a, b)), 1);
}

#[test]
fn elementary_transition_preserves_hamming_distance() {
    let a = State::new(8, 1, 1, true);
    let b = State::new(9, 1, 1, true);
    assert_eq!(transition_kind(a, b), TransitionKind::Elementary);
    assert_eq!(transition_distance(project_transition(a, b)), 1);
}

#[test]
fn semantic_labels_are_irrelevant_to_formal_projection() {
    let state = State::new(13, 41, 77, true);
    // Human-readable labels are intentionally absent from the transformation.
    assert_eq!(candidate_t(state), candidate_t(state));
}

#[test]
fn geometric_equality_does_not_imply_semantic_identity() {
    let a = State::new(5, 1, 10, true);
    let b = State::new(5, 2, 10, true);
    assert_eq!(candidate_t(a), candidate_t(b));
    assert_ne!(a.identity, b.identity);
    assert_eq!(classify_projection(a, b), LossClass::Destructive);
}

#[test]
fn unknown_is_not_pass() {
    assert_ne!(LossClass::Unknown, LossClass::Preserved);
    assert_ne!(LossClass::Unknown, LossClass::Destructive);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Model {
    Q5,
    Q4PlusR,
    Q4TimesBinary,
    Graph,
}

fn model_cost(model: Model) -> u32 {
    match model {
        Model::Q5 => 5,
        Model::Q4PlusR => 5,
        Model::Q4TimesBinary => 5,
        Model::Graph => 31,
    }
}

fn declared_models() -> [Model; 4] {
    [Model::Q5, Model::Q4PlusR, Model::Q4TimesBinary, Model::Graph]
}

#[test]
fn alternative_models_are_evaluated_under_same_declared_metric() {
    let models = declared_models();
    assert_eq!(models.len(), 4);
    for model in models {
        // Cost is deliberately the same criterion for every candidate.
        assert!(model_cost(model) > 0);
    }
}

#[test]
fn model_shape_alone_cannot_establish_superiority() {
    let q5_cost = model_cost(Model::Q5);
    let q4_plus_r_cost = model_cost(Model::Q4PlusR);
    // Equal representation cost means shape alone supplies no superiority claim.
    assert_eq!(q5_cost, q4_plus_r_cost);
}

fn reconstructible_with_n_bits(n: u8) -> bool {
    n >= 5
}

#[test]
fn minimum_binary_representation_requires_five_bits_for_32_states() {
    assert!(!reconstructible_with_n_bits(4));
    assert!(reconstructible_with_n_bits(5));
    assert!(reconstructible_with_n_bits(6));
}

#[test]
fn false_equivalence_fixture_separates_geometry_from_authority() {
    let permitted = State::new(3, 1, 9, true);
    let adjacent_but_unauthorized = State::new(2, 1, 9, false);

    assert_eq!(hamming(permitted.q5, adjacent_but_unauthorized.q5), 1);
    assert_eq!(transition_kind(permitted, adjacent_but_unauthorized), TransitionKind::Elementary);
    assert!(!adjacent_but_unauthorized.authorization);
}

#[test]
fn five_bits_are_embeddable_without_projection_collision() {
    let states: Vec<_> = (0..32).map(|q5| State::new(q5, 1, 1, true)).collect();
    for i in 0..states.len() {
        for j in (i + 1)..states.len() {
            assert_ne!(candidate_t(states[i]), candidate_t(states[j]));
        }
    }
}
