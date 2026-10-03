//! Honest flags for GAIAN #57–#75.
//!
//! ## `live_whisper`
//!
//! Returns `true` only when the crate is compiled with
//! `--features whisper`, which links `whisper-rs` and activates the real
//! `WhisperAsr` inference path in `asr.rs`.
//!
//! In the **default build** (no feature flag) this returns `false` because:
//! * no GGML model file is present in CI,
//! * the `asr::WhisperAsr::transcribe` path is the compile-time stub, and
//! * the honesty contract test asserts `!live_whisper()` by default.
//!
//! To activate real Whisper inference locally:
//! ```bash
//! export GAIA_WHISPER_MODEL=/path/to/ggml-base.en.bin
//! cargo test --manifest-path gaia-gaian/Cargo.toml --features whisper
//! ```
//!
//! ## Kundalini Principle K1 — Latent, Not Absent
//!
//! Every `pub fn` below that returns `false` in this file is a **K1 coiled
//! serpent**: potential that has not yet been activated, not capability that
//! does not exist.  These flags earn `true` through real, verifiable
//! integration work — they are never given it as a gift.
//!
//! The path from `false` to `true` for each flag is documented in the
//! corresponding epic issue.  The philosophy is captured in full in
//! `Documents/GAIA_Kundalini_Architecture.md` (merged via PR #775, Sep 2026).
//!
//! > *"The elixir of life has always been there within ourselves.
//! > It's called the kundalini."* — Kyle Alexander Steen, 22 Sep 2026

pub fn live_ollama() -> bool { false }
pub fn live_flutter() -> bool { false }
/// `true` only when compiled with `--features whisper` (real whisper.cpp linked).
pub fn live_whisper() -> bool { cfg!(feature = "whisper") }
pub fn live_vrm_runtime() -> bool { false }
pub fn gaian_v1_tagged() -> bool { false }
pub fn medical_product() -> bool { false }
