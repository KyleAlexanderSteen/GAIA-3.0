//! Pass 3 conformance: provenance -> evidence -> verification.
//!
//! This test deliberately verifies the existing provenance contract without
//! introducing a second provenance implementation. The verification step is
//! independent: it recomputes SHA-256 from the original raw bytes and checks
//! the recorded provenance receipt before the epistemic evidence is accepted
//! as grounded metadata.

use sha2::Digest;

use gaia_ingest::{
    DataSource, EpistemicStateBuilder, EvidenceKind, ProvenanceBuilder,
};

#[test]
fn provenance_evidence_chain_is_integrity_bound() {
    let raw = b"NOAA observation payload";
    let receipt = ProvenanceBuilder::new(
        DataSource::Noaa,
        "https://api.noaa.gov/obs/test",
        "OBS-CONFORMANCE-001",
        1_700_000_001,
        1_700_000_000,
    )
    .seal(raw)
    .expect("valid provenance receipt");

    // Independent verification of the receipt's cryptographic binding.
    let recomputed = hex::encode(sha2::Sha256::digest(raw));
    assert_eq!(receipt.sha256, recomputed);
    assert!(receipt.is_valid());

    // Evidence remains epistemic metadata; it does not replace provenance.
    let evidence = EpistemicStateBuilder::new()
        .claim_status(gaia_ingest::ClaimStatus::Corroborated)
        .evidence_kind(EvidenceKind::DirectObservation)
        .confidence(0.95)
        .grounding_uri(&receipt.source_url)
        .rationale("Evidence is grounded in a provenance-bound source receipt.")
        .build()
        .expect("valid epistemic state");

    assert_eq!(evidence.evidence_kind, EvidenceKind::DirectObservation);
    assert_eq!(evidence.grounding_uris, vec![receipt.source_url.clone()]);
    assert_eq!(evidence.confidence.value(), 0.95);
}

#[test]
fn provenance_verification_detects_changed_source_bytes() {
    let raw = b"NOAA observation payload";
    let changed = b"NOAA observation payload changed";

    let receipt = ProvenanceBuilder::new(
        DataSource::Noaa,
        "https://api.noaa.gov/obs/test",
        "OBS-CONFORMANCE-002",
        1_700_000_001,
        1_700_000_000,
    )
    .seal(raw)
    .expect("valid provenance receipt");

    let changed_hash = hex::encode(sha2::Sha256::digest(changed));
    assert_ne!(
        receipt.sha256, changed_hash,
        "changed source bytes must not verify against the original receipt"
    );
}
