# AI System Inventory — #1047

Living list. Status is compile/test honesty, not marketing. Last review: 2026-09-27.

| System | Owner crate | Purpose | Build / buy / model | Risk | Data in | Status |
| --- | --- | --- | --- | --- | --- | --- |
| ACP gateway | `gaia-acp` | authorize, audit, sandbox listed | build | High | tool names, policy | Experimental |
| Hybrid RAG | `gaia-aikd` | retrieve, cite, ground | build | Medium | local chunks | Experimental |
| Gateway HTTP | `gaia-gateway` | health + agent routes | build | Medium | HTTP | Experimental |
| CLI | `gaia-cli` | operator commands | build | Low | argv | Experimental |
| Boot | `gaia-boot` | process start | build | Medium | config | Experimental |
| Knowledge catalog | `docs/knowledge/catalog.json` | listed domains | build | Low | json | listed; runtime_enabled false |
| Ollama | #646 | local LLM | buy/local | High | prompts | Pending; not wired default |
| Hugging Face cards | — | model cards | buy | Medium | public cards | Pending |
| Whisper ASR | feature gate | speech | buy | Medium | audio | feature-gated off |
