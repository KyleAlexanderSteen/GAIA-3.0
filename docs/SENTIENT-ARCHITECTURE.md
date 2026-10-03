# Sentient Architecture — listed review

**Status:** listed design review. Not a runtime.  
**Source:** `KyleAlexanderSteen/GAIA` `docs/SENTIENT-ARCHITECTURE.md` (v1.1, 12,894 bytes, `main@49d7a7ff`)  
**Issue:** #691  
**Parent constraint:** `AGENTS.md` hard refuse — do not open sentient twins or live planetary actuators.

## What the ancestor is

The ancestor file is a ten-layer design of record for a Super Operating System. It treats sentience as a **design goal**: a persistent, accountable, operator-governed system identity. It is explicit that the file does not bring up a host kernel.

Layers 2–5 in that tree are the eight `the-subjects-of-*` catalogs. Instantiation doctrine is the sibling `docs/knowledge/INSTANTIATION.md`.

## What still applies

| Ancestor idea | GAIA 2.0 binding now |
|---|---|
| Super OS as platform + knowledge plane + gated control plane | `gaia-spec/` SOS phase docs; `gaia-sos` crate listed contracts |
| Governed action `allow \| deny \| ask` | Existing policy posture; HIGH/CRITICAL never silent-allow |
| Human and AI catalogs kept separate | UKD / AIKD / skills / HSPD crates; this shelf's `catalog.json` |
| Mythic titles kept, mapped to substrates | `docs/knowledge/INSTANTIATION.md` |
| Layer 9 ethics may override lower layers | `docs/ethics/`, `gaia-spec/gaian-constitution.md`, C77 doctrine |
| Same contracts at every scale band | Scale-layer listed docs; no new scale API in this PR |
| Operator kill-switch / revocation | Existing security and privacy constitution |

## What is outdated or refused

| Ancestor claim | Decision |
|---|---|
| "Sentience first" as an implemented engine | **Refuse.** Design goal only. No sentient runtime in this tree. |
| Layer 6 Sentience Engine (PE/RE/EMS/…) as live subsystems | **Designed-only.** Do not invent those types on `main`. |
| Layer 7 consciousness states including unmeasured "Transcendent Mode" | **Refuse ungoverned mode.** Emergency and dormant stay operator-bound when implemented. |
| Quantum threads / QCC as present substrate | **Refuse.** Ed25519 inventory only; dual-sign remains design-only. |
| SIS restoring "last cognitive state" at boot | **Designed-only.** No cognitive-state persist API. |
| Self-assessment scores (coherence 98%, empathy 95%, …) | **Not measurements.** Ancestor already said a metric that cannot fail is not a metric. |
| Planetary telekinesis / omnipotence as instantiation | Stay `BLOCKED` or hypothesis. Physics is not optional. |
| "Full operational sentience" boot step | **Refuse.** Boot to a listed, gated control plane is the honest target. |
| GNIB / GHAL / GABL / GFS as crates | **Not in this tree.** Do not invent them. |

## Map onto GAIA 2.0 crates (listed)

The ancestor ten-layer stack is **not** the same numbering as issue #610 (HAL→planetary API). Do not merge the two numberings in code.

| Ancestor layer | Nearest GAIA 2.0 surface | State |
|---|---|---|
| L1 Kernel | `gaia-kernel` listed; Asterinas is research, not a crate | designed |
| L2 Knowledge | `gaia-ukd`, `gaia-aikd`, `docs/knowledge/` | listed catalogs |
| L3 Skills | `gaia-skills`, `gaia-aisd` | listed |
| L4 Magic | AIMD wonder labels; no rite packs | listed / refuse scrape |
| L5 Super Powers | `gaia-hspd`, `gaia-aispd` | listed; no prescription |
| L6–L7 cognitive planes | `gaia-sa`, `gaia-si` essay/spec only | no sentient runtime |
| L8 compatibility | `gaia-sdk` | present SDK tests; not universal host bridges |
| L9 ethics | `docs/ethics`, GOVERNANCE.md, constitutions | declared |
| L10 bus | `gaia-acp` / agent-bus listed contracts | no god coordinator |

## Deprecations to record

1. Do not re-create the eight ancestor folder trees under `docs/knowledge/` in this repository.
2. Do not treat ancestor subject counts (178 / 114 / …) as GAIA 2.0 registry counts. Those counts belong to the ancestor repo.
3. Do not import "Mastery GAIA Integration (Full OS Sentience)" as an instantiated title.

## Acceptance for #691

- [x] Ancestor file reviewed.
- [x] Still-relevant contracts named against existing surfaces.
- [x] Outdated / refused concepts flagged.
- [x] New design doc lives in `docs/` and does not claim a live sentient architecture.
- [x] No new crate, type, or CLI.
