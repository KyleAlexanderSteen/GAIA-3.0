# Symbology v0 prototype

Status: **prototype, specification plus tested code. Not integrated into the execution path.** Refs #1341.

- `symbology_v0.py`: signed record -> Reed-Solomon -> 3x3-cell glyphs. The corner cell is always filled, borrowing the closed-cell orientation rule described for Kryptonian in *Man of Steel*.
- `symbology_pq.py`: hybrid frame, Ed25519 plus ML-DSA-44 (FIPS 204). Both signatures must verify.
- Security comes only from the signatures. The glyphs add readability and damage tolerance. A glyph must verify cryptographically, never visually.
- Dependencies: see `requirements.txt` (not version-pinned yet).
- Run from this directory: `python test_symbology.py` (about 25 seconds; one test downloads a public-domain book from Project Gutenberg and is skipped if offline) and `python test_symbology_pq.py`.

## Recorded results

Ed25519 only (`results.json`, produced by `test_symbology.py`):
- 107 glyphs carry an 83-byte signed frame plus 24 parity glyphs.
- Round trip, image round trip, and 0/90/180/270 degree rotations verify.
- Up to 12 damaged glyphs (scattered or in a block) always recover; beyond that the glyph is rejected.
- 18,600 damage trials: 0 wrongly accepted. 8,000 forgery attempts: 0 accepted.
- A Wakandan-style one-to-one symbol substitution is fully recovered by a simple frequency attack at 300, 600 and 1,200 letters.

Hybrid with ML-DSA-44 (`results_pq.json`, produced by `test_symbology_pq.py`):
- The frame grows from 107 to 2,767 glyphs (25.9 times), because an ML-DSA-44 signature is 2,420 bytes and the public key is 1,312 bytes. A 48-column sheet is 2200 x 2640 px.
- Forgery cases all rejected, including one where an attacker who has broken Ed25519 re-signs a tampered record: the ML-DSA signature still fails. A v1 frame presented to a v2 verifier is rejected, so downgrade fails.
- Scattered damage up to 48 glyphs recovered in 12 of 12 trials, 96 in 3 of 12. A contiguous block of 12 recovered, 24 not. Burst tolerance did not improve because error correction works per 255-byte chunk. 0 wrongly accepted. These are small samples (12 trials per cell).

## Known limits

- Replay of an old valid glyph is not prevented. Needs a nonce or expiry policy.
- `dilithium-py` is a pure-Python implementation and not constant time: prototype only. Production needs a vetted library.
- At 25.9 times the size, carrying the full ML-DSA signature in glyphs is impractical for small marks. Options to evaluate: carry only a hash and a locator, with the signature held elsewhere; or use the hybrid frame only for high-trust records.
- Not tested: SLH-DSA, ML-DSA-65/87, real camera capture, outside cryptographic review. The image path is tested for Ed25519 only.
- No claim of unbreakability is made anywhere in this directory.
