# GAIA 2.0 Hugging Face Artefacts

**Filed:** 2026-09-17  
**Account:** [KyleAlexanderSteen](https://huggingface.co/KyleAlexanderSteen)  
**Org:** none yet  
**Published repos:** none  

GitHub already has a stub at `huggingface/gaia-elements/README.md`. Nothing has been pushed to the Hub.

## Role of the Hub

Hugging Face is the public, citable, versioned plane. Use it when visitors should **download, inspect, or reproduce**. Keep live personal state in Supabase.

| Want people to... | Put it in |
|---|---|
| Run or fine-tune a model | Model repo + model card |
| Cite a versioned table, eval set, or anonymized log | Dataset repo + dataset card |
| Try a demo | Space (later) |
| Read law | Stay on GitHub |

Official GitHub-to-Hub sync exists. Mirror from `GAIA-2.0/huggingface/*` so Hub files never drift from canon.

## Publication order

1. `gaia-elements` — from Supabase `elements` + element schema (safest, no PII)
2. `gaia-minerals` — minerals with GAIA layer alignment, RRUFF/IMA/Mindat provenance
3. `gaia-crystals` — crystals with septagram nodes; quantum-bridge fields marked `[NEEDS PROOF]` unless proven
4. `gaia-alscn-gan` — Canon C67 interface spec + known limitations (already honest about cluster truncation)
5. `gaia-memory-access-log` — **only after** RESEARCH_USE consent + anonymization
6. `gaia-gfi-instrument` — 35-item Flourishing Index instrument, not population microdata
7. Models: relevance scorer, then agent-policy, each with a GitHub proof ID

Do not publish Coherence Gap or Elemental Learning participant data. Publish protocols and synthetic/public benchmarks only until ethics gates pass.

## Dataset card template

Every dataset README must include YAML metadata and these sections:

```yaml
---
license: cc-by-4.0
task_categories: [tabular-classification]
language: [en]
tags: [gaia, gaia-2.0, lithic]
pretty_name: GAIA Elements
---
```

1. Summary (one paragraph)
2. Intended use
3. Out-of-scope use
4. Schema / splits
5. Source and provenance (GitHub SHA, Supabase export timestamp, proof_id)
6. Known limitations
7. Ethics and PII statement
8. License
9. Citation

Model cards additionally require: training data identifiers, evaluation metrics, biases, environment actually tested.

## First dataset: `gaia-elements`

Source tables: `public.elements` (currently 3 rows — publication waits until the lithic ingest from RRUFF/IMA is loaded).

Minimum columns:

- `z`, `symbol`, `name`
- physical fields from `element_schema.json`
- `gaia_layer_alignment`
- `penteract_substrate` (if present)
- `source` / `notes`

License recommendation: data facts (atomic numbers) are not copyrightable; GAIA alignment fields use CC-BY-4.0 to match repo `LICENSE-CC-BY-4.0`. Code remains Apache-2.0 / MIT as in the monorepo.

## Org recommendation

Create Hugging Face org `gaia-os` (or `gaia-2-0`) and transfer datasets there so they survive account changes. Current OAuth is user-scoped with `contribute-repos`. Org membership was not visible to the current credential.

## Proof binding

No Hub repo is canon until:

1. A GitHub file exists under `huggingface/<name>/`
2. A row exists in Supabase `proofs`
3. The card links both

Example: `proof_id = HF-ELEMENTS-001`, `canon_path = huggingface/gaia-elements/README.md`.
