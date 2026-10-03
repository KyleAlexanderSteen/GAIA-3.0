# canon-runtime

Machine-readable **listed** layer over the Hermetic Tablet Canon.

Parent epic: [#836](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/836).
Registry source of truth: [`docs/tablets/INDEX.md`](../docs/tablets/INDEX.md).
Canon hygiene epic: [#798](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/798).

This directory is data + a read-only lookup. It is not a crate.

## Non-enforcement notice

Presence of a tablet in `manifest.json` is **not**:

- ACP enforcement
- a syscall
- a honesty flip
- sentience, planetary agency, or a second constitution

Neutral systems language for a future consumer map lives on [#817](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/817). This tree does not do #817's work.

## Regenerate the manifest

From the repository root (Python 3.12 stdlib only):

```bash
python3 scripts/build-canon-runtime.py
python3 scripts/build-canon-runtime.py --check
python3 -m unittest discover -s canon-runtime -p "test_*.py"
```

`--check` is what CI runs. If INDEX, CAP sidecar, or proof filenames change, regenerate and commit `manifest.json` in the same PR.

## Schema + query contract

| Piece | Path | Issue |
| --- | --- | --- |
| JSON Schema | [`schema.json`](schema.json) | #892 |
| Build script | [`scripts/build-canon-runtime.py`](../scripts/build-canon-runtime.py) | #893 |
| CI gate | [`.github/workflows/canon-runtime.yml`](../.github/workflows/canon-runtime.yml) | #894 |
| CAP sidecar | [`cap.json`](cap.json) | #895 |
| Read-only query | [`query.py`](query.py) | #896 |
| This README | [`README.md`](README.md) | #897 |

`query.py` functions (unknown id → `None`, never a panic):

- `getTablet(id)`
- `getConstraints(id)` / `getAffordances(id)` / `getProhibitions(id)`
- `getTabletByColor(hex)`
- `getTabletsByElement(element)` / `getTabletsByStage(stage)`

Lookups that must stay true while INDEX is authoritative:

- Emerald hex `#50C878`
- Terra bistre `#3D2B1F`

Element and stage enums follow `docs/canon/ELEMENT_ONTOLOGY.md` and `docs/canon/STAGE_SEQUENCE.md`. Unassigned INDEX cells (`—`) become JSON `null`.

## Examples

- Sealed: [`examples/emerald.json`](examples/emerald.json)
- Unsealed: [`examples/amber.json`](examples/amber.json)

Invalid element or stage values fail the build script (exit non-zero).

## Related

INDEX · proofs/ · docs/canon/SEALING_CEREMONY.md · #815 · #817 · #835
