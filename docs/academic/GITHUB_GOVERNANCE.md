# GAIA 2.0 GitHub Governance

**Filed:** 2026-09-17  
**Account:** [KyleAlexanderSteen](https://github.com/KyleAlexanderSteen)  
**Canonical repo:** [KyleAlexanderSteen/GAIA-2.0](https://github.com/KyleAlexanderSteen/GAIA-2.0)

## Repository map

| Repository | Visibility | Language | Role |
|---|---|---|---|
| `GAIA-2.0` | Public | Rust | Canonical Super OS; laws, crates, proofs, supabase/, huggingface/ |
| `GAIA` | Private | Python | Immediate predecessor; keep as reference, do not fork canon forward blindly |
| `GAIA-Old-Repository` | Private | JavaScript | Archive |
| `NEXUS-Old-Repository` | Private | Python | Archive |
| `The-Nexus-of-the-Alchemist-Old-Repository` | Private | — | Archive / alchemy origin |

`GAIA-2.0` description: *Universal open-source Super Operating System. Artificial Twin of Earth. Home of GAIAN 2.0, the Artificial Twins of Humans.* Created 2026-09-09. Last push 2026-09-17. **202 open issues.** Default branch: `main`.

## In-repo layout (live)

Top-level crates and planes already present:

- Runtime: `gaia-kernel`, `gaia-agents`, `gaia-orchestrator`, `gaia-sdk`, `gaia-interface`, `gaia-skills`
- Memory / identity: `gaia-memos`, `gaia-gaian`, `gaia-sfs`
- Knowledge: `gaia-aikd`, `gaia-aimd`, `gaia-aisd`, `gaia-aispd`, `gaia-ukd`, `gaia-hmgd`, `gaia-hspd`
- Earth / geometry: `gaia-earth`, `gaia-geometry`
- Spec / docs: `gaia-spec`, `gaia-docs`, `docs/academic`, `rfcs`, `proofs`
- Platform mirrors: `supabase/migrations`, `huggingface/gaia-elements`
- Governance files: `README.md`, `GOVERNANCE.md`, `CODE_OF_CONDUCT.md`, `CONTRIBUTING.md`, `SECURITY.md`, dual licenses, `NOTICE`
- CI: `.github/workflows/ci.yml`, issue templates, PR template

Do not split into `gaia-codex` / `gaia-runtime` / `gaia-data` unless the monorepo becomes unworkable. The monorepo already encodes those planes as directories.

## Proof law

Existing proof artefacts:

- `proofs/PROOF-GAIA20-MEMORY-001.md`
- `proofs/PROOF-GAIA20-MEMORY-002.md`
- `proofs/PROOF-GAIA20-MEMORY-003.md`
- `proofs/PROOF-GAIA20-AGENT-001.md`

Required Proof block on every canon document:

```markdown
## Proof
- Type: simulation | formal | empirical | emergent_confirmation
- Status: proven | partial | in_progress
- Method: ...
- Results: ...
- Artefacts: path or SHA
```

CI must fail any PR that:

1. Touches canon without a Proof block
2. Breaks JSON schema for elements / minerals / crystals
3. Changes Supabase migrations without a matching proof or migration note
4. Publishes a Hugging Face card that lacks intended-use and limitations

Academic issue rule: every issue that is research must end with a self-contained deliverable (markdown, simulation output, or schema), not a comment thread.

## Issue topology (open, sampled 2026-09-17)

Highest-number open issues are GAIAN design and Sentient Architecture:

| Range | Theme |
|---|---|
| #213–#221 | GAIAN identity, Telos, memory continuity, autonomy, constitution, child rules |
| #202–#212 | Sentient Architecture Phases 0–4 (TEK, biophilia, regenerative systems, pattern library) |

Older specs still in the attached corpus map to earlier issues (#119–#173: personhood, consent ledger, memory hierarchy, canon graph, telemetry).

When filing new academic work, attach it to an existing epic rather than creating a parallel universe of issues.

## Canon graph (Issue #169)

Canon is a DAG, not a folder of essays.

- Front-matter: `id`, `title`, `version`, `status`, `requires`, `supersedes`, `upstream`, `tags`
- Status: `active | deprecated | draft`
- Safety gates: cycle warning, 3-tag conflict check, impact report before deprecation
- Trace: `CANON_LOAD` event after graph build

GitHub is the source of truth for canon files; Supabase `proofs` table is the runtime registry of proof status.

## Branch and PR policy

- `main` — protected; proofs required for canon paths
- `research/*` — studies, surveys, pre-canon
- `sim/*` — simulation harnesses
- `hotfix/*` — runtime safety only

PR template should require: canon refs, proof type, Supabase migration impact, Hugging Face impact, ethics/privacy impact.

## What GitHub is not

GitHub is not the personal-memory store, not the consent ledger, and not the place for raw HRV, GSR, or child-study data. Those belong in Supabase with RLS, or never leave the lab.
