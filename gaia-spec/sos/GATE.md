# SOS v1.0 gate

A gate list. Not a tag.
Crate already on main: `gaia-sos::{sos_v1_tagged, five_nines_claimed, formal_verify_done, threats}`.
RFC template already on main: `gaia-spec/sos/RFC.md`.
Issue this slice: #529 / #200. Stay on KyleAlexanderSteen/GAIA-2.0 until a TSC exists.

## Checklist that stays unchecked

- `sos_v1_tagged()` → false
- `five_nines_claimed()` → false
- `formal_verify_done()` → false (plan, not a crate)
- ABI freeze is an RFC field, not a completed freeze
- threat model exists (`threats()`); it is not a closed P0 audit

Measured scheduler metrics are not published. Do not invent them.

## Refuse

- SOS v1.0 release tag
- five-nines marketing
- empty gaia-os org
