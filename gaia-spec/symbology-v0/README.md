# Symbology v0 prototype

Status: **prototype, specification plus tested code. Not integrated into the execution path.** Refs #1341.

- `symbology_v0.py`: signed record -> Reed-Solomon -> 3x3-cell glyphs. The corner cell is always filled, borrowing the closed-cell orientation rule described for Kryptonian in *Man of Steel*.
- Security comes only from the Ed25519 signature. The glyphs add readability and damage tolerance. A glyph must verify cryptographically, never visually.
- Dependencies: `reedsolo`, `cryptography`, `pillow`, `numpy`.
- Run the tests from this directory: `python test_symbology.py` (about 25 seconds; one test downloads a public-domain book from Project Gutenberg and is skipped if offline).

## Recorded results (see results.json)

- 107 glyphs carry an 83-byte signed frame plus 24 parity glyphs.
- Round trip, image round trip, and 0/90/180/270 degree rotations verify.
- Up to 12 damaged glyphs (scattered or in a block) always recover; beyond that the glyph is rejected.
- 18,600 damage trials: 0 wrongly accepted. 8,000 forgery attempts: 0 accepted.
- A Wakandan-style one-to-one symbol substitution is fully recovered by a simple frequency attack at 300, 600 and 1,200 letters. Scripts alone give no secrecy.

## Known limits

- Replay of an old valid glyph is not prevented. Needs a nonce or expiry policy.
- Ed25519 is not quantum-resistant. Not tested: ML-DSA or hybrid signatures, real camera capture, outside cryptographic review.
- No claim of unbreakability is made anywhere in this directory.
