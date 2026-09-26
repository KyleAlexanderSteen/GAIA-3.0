# Canon field audit — listed

No INDEX rewrite. No seal ceremony. #798 stays open.

Color-map (`docs/color/color-map.json` v1.0.1) has 18 rows. Hex present on every row. `sealed` set on Citrine, Emerald, Viriditas, Void only. Ember/Terra and the rest stay `null` — deferred to tablet trackers `#874–#889`.

| Issue | Finding |
| --- | --- |
| #820 dates | Audit only. Do not invent seal dates. |
| #821 hex | 18 hex values present. Path 404s belong to missing companion color pages, not the hex field. |
| #823 schema | Enforced by `tests/canon/test_canon_integrity.py` + `tablet-validation.yml`. |
| #825 xrefs | Broken-link sweep not executed as a rewrite. Live gate is canon-integrity. |
| #826 proofs | `scripts/check_canon_proofs.sh` already on main. |
| #827 semver tables | Missing tables stay missing. No tablet body edit. |
| #828 element | Map field mostly empty (`alchemical_stage` null except Amber). |
| #829 stage | Same. |
| #830 author | Map authors string is shared; per-tablet Author field not rewritten. |
| #834 validator | `scripts/validate-tablets.sh` wraps the existing suite. |
