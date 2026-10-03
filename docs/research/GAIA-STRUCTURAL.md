# GAIA Structural

**Status:** listed parallel formulation.  
**Issue:** [#817](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/817)

## Abstract

This note states a reusable method for comparing knowledge systems as layered architectures. A source is read in its own context. Stable relations are extracted as units (hierarchy, recursion, polarity, transformation, scale coupling, mediation, circulation). Units are encoded in a versioned schema. Units may map to an implementation surface, or they may map to nothing. Empty maps are published. Negative controls (unrelated text) must not produce accepted maps. The method is useful only if it fails on some inputs.

## Terminology

| Classical register (repo) | Structural term |
| --- | --- |
| numbered law file | core protocol spec |
| above / below | inter-scale coupling |
| open / navigable / withheld layers | public interface / documentation / protected internals |
| color navigation | access-layer model |
| seal / proof | versioned acceptance artefact |

Concepts that lose meaning if forced into this table stay in the classical register only.

## Extraction pipeline

1. Identify the source, edition, and locator.
2. Extract a structural unit with no target attached.
3. Propose a target in a second pass, or record `no_mapping`.
4. Attach one competing reading and one disconfirmation condition.
5. Run the same process on negative-control text.

Schema: `gaia-spec/mapping/schema.yaml`.

## Access layers

Public interface, documentation, and protected internals are three views of one protocol. They are not sensors.

## Falsification

See `tests/falsification/` and `gaia-spec/mapping/PROTOCOL.md`.

## Predictions (listed, untested)

1. A protocol that skips the substrate layer will fail integration tests more often than one that does not.
2. Maps accepted on lorem-class controls indicate over-fit.
3. Inter-scale coupling claims that ignore provenance will not replicate across editions.
4. Stable configurations keep an explicit uncertainty band; configurations that drop the band drift.
5. Patterns that appear in only one corpus stay local and are not generalized.

Minimum viable tests already designed: (2) is `tests/falsification/test_negative_controls.py`. (1) is the existing workspace `cargo test` gate.

## Convergence

Where the two registers agree: empty maps are valid; negative controls must fail; confidence is a band. Where they diverge: the classical register keeps names this document does not use. Divergence is a finding.

## Publication pathway (listed)

Candidate venues only, no submission: systems science journals, complexity venues, knowledge-organization venues; preprint class cs.AI or nlin.AO. Not filed.

## Refuse

No corpus ingest. No claim that both registers have been peer-reviewed.
