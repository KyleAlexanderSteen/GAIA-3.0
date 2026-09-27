# Super OS claim vs compile — #1041

Status key: `implemented` = tests exist on main. `partial` = crate or helper exists, thin. `listed` = spec/docs only. `absent` = slogan without path. `refused` = must stay off (#1043).

| Claim | Path | Test / gate | Status |
| --- | --- | --- | --- |
| Workspace Rust crates | `Cargo.toml` members | `cargo test --workspace --lib` | partial |
| HAL | `gaia-hal` | crate tests | partial |
| Kernel | `gaia-kernel` | crate tests | partial |
| Asterinas framekernel | — | — | listed / not a crate |
| Identity / DID / revoke | #725 #736 | — | listed |
| WASM 3.0 + WASI 0.3 | #614 | — | absent |
| ACP authorize + audit | `gaia-acp` | crate tests | partial |
| Real tool plane | #726 | — | listed |
| MemOS 5-tier | #616 | — | listed |
| File chunk store + hybrid RAG | `gaia-aikd` | crate tests | partial |
| Grounding / NeedVerify | aikd helpers | crate tests | partial |
| Gateway health | `gaia-gateway` | crate tests | partial |
| CLI | `gaia-cli` | crate tests | partial |
| Boot | `gaia-boot` | crate tests | partial |
| Canon integrity | `tests/canon/` | `canon-integrity.yml` | implemented |
| Proof gate | `scripts/check_canon_proofs.sh` | `canon-proof-gate.yml` | implemented |
| Knowledge catalog | `docs/knowledge/catalog.json` | `validate_catalog.py` | implemented (`runtime_enabled` false) |
| Tablet inventory | `scripts/tablet_inventory.py` | advisory | partial |
| Qdrant / Graphiti | #723 | — | refused |
| Live Earth ingest | #949 #652 | — | refused |
| Trainer / `gaia-learn` | #870 #1053 | — | refused |
| `gaia-economy` / token | #871 | — | refused |
| `gaia-net` daemon | #872 | — | refused |
| `gaia-xr` | #873 | — | refused |
| Sentient / planetary consciousness | #641 #972 | catalog flag | refused |
| 1,000-language engine | #651 | — | listed vision; v0 is #1046 |
