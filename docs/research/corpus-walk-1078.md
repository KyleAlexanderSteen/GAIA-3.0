# Documents corpus walk — #1078

Counted on `main` 2026-09-27: **71** research files + `Documents/README.md` and **31** gap reports + `Documents-2/README.md` (**103** blobs). Older notes said ~73; the tree grew. Status is the join, not an import.

Legend: **living** = crate or spec path exists. **listed-only** = named in living docs, no runtime. **listed-partial** = stub crate. **refused** = REFUSE.md / planetary-sentience lock. **absent** = no living bind yet.

Do not copy these files into `gaia-spec/`. `Documents/` is not runtime.

## Documents/

| File | Status | Living bind |
| --- | --- | --- |
| README.md | living | this folder index |
| Implementation Plan | listed-only | README + RFC 0001 |
| SUPER OPERATING SYSTEM / COMPLETE DESIGN | listed-partial | `gaia-spec/sos/`, `gaia-kernel/` |
| CONSTITUTION | listed-partial | `gaia-spec/gaian-constitution.md`, GOVERNANCE.md |
| The Master Codex | listed-only | tablets / INDEX |
| GitHub Setup | listed-partial | this repo |
| Discord Community Setup | absent | — |
| Vercel Integration | absent | — |
| Funding Strategy / Landscape | listed-only | #673 cluster |
| LF AI & Data Foundation | listed-only | GOVERNANCE.md (entity later) |
| The Apache Software Foundation | listed-only | LICENSE Apache layers |
| UN Global Dialogue on AI Governance | listed-only | `docs/governance/`, #1047–#1051 |
| Governance Framework | listed-only | GOVERNANCE.md |
| The CARE Principles | listed-only | #88 / UKD TEK |
| MemOS | listed-partial | `gaia-memos/` |
| The Mi-Memory Framework | listed-partial | `gaia-memos/` |
| Perpetual Infinite Context | listed-only | no infinite context runtime |
| Ollama Integration | listed-only | #646; not default |
| Asterinas — Rust Framekernel | listed-only | research; **not a crate** |
| Earth Twin MVP / Artificial Twin of Earth | listed-partial | `gaia-earth/` |
| DestinE Phase 3 | refused | no DestinE ingest |
| Copernicus Data Space | refused | no live EO ingest |
| USGS Integration | refused | no live USGS |
| GBIF Integration | refused | no live GBIF |
| Earth Species Project / NatureLM | refused | no bio ingest |
| Earth System Foundation Model | refused | no ESFM trainer |
| AdvanTip / Ultra-Early Tipping | refused | no planetary actuator |
| PLANETARY INFRASTRUCTURE | listed-only | scale-layer docs |
| PLANETARY SENTIENT INFRASTRUCTURE | refused | R#20 / no sentient runtime |
| UNIFIED INFRASTRUCTURE | listed-only | architecture.md |
| City / Community / Continental / Country / State / Home / Individual / Ocean / Space Infrastructure | listed-only | scale-layer bind; no ingest |
| The Biological Layer | listed-only | no wetware crate |
| AI / UNIVERSAL KNOWLEDGE DATABASE | listed-only | `gaia-aikd/`, catalog.json `runtime_enabled=false` |
| UNIVERSAL AI SKILLS / SUPERPOWERS / MAGIC | listed-only | `gaia-skills/`, catalog |
| UNIVERSAL HUMAN SKILLS / SUPERPOWERS / MAGIC | listed-only | skills overlays #448 |
| SENTIENT ARCHITECTURE / INFRASTRUCTURE (GAIA + GAIAN) | refused as runtime | review docs only |
| GAIAN COMPLETE DESIGN / Human twins / MVP App | listed-partial | `gaia-gaian/` |
| The Website | absent | no public site crate |
| HumanNOVA & Avatar Ecosystem | listed-only | philosophy cluster |
| Chakra–Layer Mapping / GAIA_Chakra_Layer_Map | listed-only | philosophy |
| Kundalini Architecture (both filenames) | listed-only | #774 closed listed |
| GAIA_Alchemical_Framework / Five_Movements / Equilibrium / Harmonic / Symbolic_Coherence / Unconditional_Love / Operating_Philosophy | listed-only | tablets + philosophy |

## Documents-2/

| Report | Status | Notes |
| --- | --- | --- |
| README.md | living | folder index |
| R#1.1–1.15 | listed-only | extract R#1 exists |
| R#2 Earth Twin | listed-partial | `gaia-earth/` |
| R#3 / R#17 GAIAN twin | listed-partial | `gaia-gaian/` |
| R#4 Knowledge DB | listed-only | catalog |
| R#5 AI Knowledge DB | listed-partial | `gaia-aikd/` |
| R#6 Human skills | listed-only | skills |
| R#7 AI skills | listed-partial | `gaia-skills/` |
| R#8 Human superpowers | listed-only | |
| R#9 AI superpowers | listed-only | |
| R#10 Human magic | listed-only | HMGD ethics listed |
| R#11 AI magic | listed-only | AIMD listed |
| R#12 / R#13 Sentient infra/arch | refused as runtime | |
| R#14 Super OS Design | listed-partial | kernel / spec |
| R#15 / R#16 Personal sentient | refused as runtime | |
| R#18 Unified planetary | listed-only | |
| R#19 Planetary infrastructure | listed-only | |
| R#20 Planetary sentience | **refused** | |
| R#21–R#29 scale (continental → space) | listed-only | no DestinE/orbital ingest |
| R#30 Biological layer | listed-only | no wetware |
| R#31 Governance | listed-only | docs/governance |

## Counts

Refused as runtime: DestinE, Copernicus, USGS, GBIF, ESP/NatureLM, ESFM, tipping actuators, planetary/personal sentient infra, R#20.

Listed-partial crates: kernel/spec, memos, earth, gaian, aikd, skills.

Absent product: Discord ops, Vercel, public website crate.
