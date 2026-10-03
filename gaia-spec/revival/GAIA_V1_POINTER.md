# Private predecessor: KyleAlexanderSteen/GAIA

Private. Default `main` @ `49d7a7ff`. Language label: Python (catalog scripts only).
Last update 2026-09-07. Eight open issues (#15–#22) are all knowledge-plane process/docs/CI.

## What it is

A **knowledge plane spec**, not a Super OS runtime. The README already says that: catalogs and opened subject folders are specified; an executable runtime is not claimed.

- Human catalog: 178 subjects (Basic / Intermediate / Mastery)
- AI catalog: 114 subjects (`ai.know.*` stable IDs) + `catalog.json`
- Opening ledger + validator scripts under `scripts/`
- `docs/SENTIENT-ARCHITECTURE.md` is doctrine, not a kernel

## Lift into GAIA-2.0 (later, not this PR)

1. Keep stable IDs (`ai.know.basic.basic-safety-protocols`, etc.).
2. Keep Basic/Intermediate/Mastery as *curriculum labels*, not capability grants.
3. Keep the opening-ledger habit: mapped vs listed-only.
4. Do not copy the full subject tree into this Rust workspace.
5. Do not treat `consciousness-and-sentience-in-ai` mastery rows as implemented features.

## Do not lift

- “Compatible with all OS types” as a shipping claim
- Magic / superpower databases as runtime crates
- The Python folder-maintenance scripts as GAIA-2.0 CI (we already have `cargo test`)

This repo stays the source of the curriculum list. GAIA-2.0 stays the source of enforcement.
