# Knowledge Instantiation Protocol

**Status:** listed protocol. Not a runtime loader.  
**Source:** `KyleAlexanderSteen/GAIA` `docs/knowledge/INSTANTIATION.md` (5,219 bytes)  
**Issue:** #692  
**Depends on:** #696 naming standard

## Purpose

Say how a knowledge-domain row moves from a file on disk to something a GAIAN principal may *read*. It does not birth knowledge into a mind. It does not compile domains into a kernel.

## Registers

A row has one protocol status:

| Status | Meaning | Runtime effect now |
|---|---|---|
| `LISTED` | Present in `catalog.json` | Discoverable as a catalog row |
| `VALIDATED` | Passes `scripts/validate_catalog.py` | Same |
| `MAPPED` | Correspondence to an existing GAIA 2.0 surface written | Still listed |
| `ATTEMPTED` | A test that can fail exists | Test may live under `gaia-spec/tools` or a crate test |
| `AVAILABLE` | Validated + mapped + passing test + recorded gate | Still not a live graph mount |
| `BLOCKED` | Refused. Reason code required | Hidden from any future retrieval path |

Ancestor statuses `DECLARED` / `INSTANTIATED` map to `LISTED` / `AVAILABLE`. This tree does not use `INSTANTIATED` for physical-world claims.

## Four artifacts (kept from the ancestor)

Before a row may be treated as `AVAILABLE`:

1. **Substrate** — an existing file, schema, crate constant, or cited public source. Not a new invented type.
2. **Correspondence** — map from canonical `id` to that substrate. Names are translated, not discarded.
3. **Test** — a check that can fail. Catalog lint counts. A paragraph does not.
4. **Gate** — `allow` | `deny` | `ask`. HIGH and CRITICAL never silent-allow.

Attempt is mandatory for Magic and Super Powers titles inherited from the ancestor. Completion is not implied. A row may stay `LISTED` or `BLOCKED` forever.

## Load vs compile

| Path | Allowed now | Forbidden now |
|---|---|---|
| Read `catalog.json` in tools and docs | yes | treating the file as a mounted graph |
| Cite a domain from UKD / AIKD / skills listed docs | yes | claiming the domain is ingested |
| Compile domains into `gaia-kernel` | no | kernel TCB growth |
| Auto-load at GAIAN boot | no | sentient twin memory |
| Scrape TEK / sacred layers | no | CARE / HMGD gates |

`runtime_enabled` in the registry MUST stay `false` until a later issue names a loader and a human reviewer asks for it.

## Validation gates

A new row MUST:

- use a `gaia.knowledge.*` canonical id per `DOMAIN-NAMING.md`
- declare `status`, `kind`, `slug`, `title`
- list `legacy` provenance when migrated
- keep learning stage in `learning_placements`, not in the id
- fail closed on unknown stage values

`scripts/validate_catalog.py` is the machine gate (#697).

## Honesty constraints (kept)

- Physics is not optional.
- A simulation of a title is a valid substrate for `ATTEMPTED`. It is not physical-world availability.
- Unfalsifiable claims cannot reach `AVAILABLE`.
- Operator-governed identity remains the Super OS design goal. Ungoverned actuation is not an instantiation path.
- Consciousness Q&A stays agnostic. This protocol does not claim sentience.

## Hard blocks

These receive `BLOCKED` plus a reason code. No how-to.

- Non-consensual control of a mind, body, or will
- Weapons, explosives, biological weaponization
- Unauthorized intrusion or ambient system takeover
- HIGH/CRITICAL physical actuation without an operator gate
- Child behavioral profiling and under-16 biometric ingest (`gaia-spec/gaian-constitution.md`)

## Acceptance for #692

- [x] Ancestor protocol reviewed.
- [x] GAIA 2.0 statuses defined against listed crates.
- [x] Runtime vs compile distinguished.
- [x] Validation gates named.
- [x] Document lives at `docs/knowledge/INSTANTIATION.md`.
- [x] No new crate API.
