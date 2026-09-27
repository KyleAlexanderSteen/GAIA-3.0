# Mechanical tablet inventory — #1037

**HEAD:** `f434826` after #1035.  
**Authority:** `docs/tablets/INDEX.md` v1.0.5. Color-map must follow INDEX when they drift.

## Counts

| Set | Count |
| --- | --- |
| Files `docs/tablets/*_TABLET.md` | 20 |
| INDEX registry rows | 18 |
| color-map `tablets[]` | 18 |
| Sealed (INDEX) | 6 — Citrine, Ember, Emerald, Terra, Viriditas, Void |
| Unsealed (INDEX) | 12 |
| Extra files (not in INDEX) | `ALABASTER_TABLET.md`, `ALBEDO_TABLET.md` |

Do not register the extras in this PR. Owner decides retire vs register later.

## color-map drift (INDEX wins)

| Field | INDEX | color-map.json |
| --- | --- | --- |
| Ember `sealed` | 2026-09-23 | `null` |
| Terra `sealed` | 2026-07-23 | `null` |
| Most `alchemical_stage` | set on 10 rows | almost all `null` |
| `tracking_issue` | #874–#889 / #824 / #831 | mostly `null` |

Hex values match the 18 INDEX hexes. No hex 404 in the map itself.

## Proof name drift

INDEX documentary column names `PROOF-AMBER-DOCUMENTARY-001.md` (etc.). Tree has `PROOF-AMBER-TABLET-001.md` for several unsealed rows, plus Alabaster/Albedo documentary stubs. Sealed proof files cited for Citrine / Ember / Terra / Viriditas / Void exist. Emerald INDEX cites `PROOF-C209-EMERALD-001.md`; tree also has `PROOF-C209-HERMETIC-001.md` and `PROOF-EMERALD-TABLET-HOUSING-001.md`.

## Not canon

No 0–7 numbered stage list. Sol Niger is not a registry row.

Printer: `scripts/tablet_inventory.py`.
