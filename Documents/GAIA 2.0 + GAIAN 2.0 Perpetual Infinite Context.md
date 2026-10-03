# GAIA 2.0 + GAIAN 2.0: Perpetual Infinite Context
## Blueprint 59: The Continuity Substrate
### September 9, 2026 — Version 0.6 (canonical)

**Write target:** [KyleAlexanderSteen/GAIA-2.0](https://github.com/KyleAlexanderSteen/GAIA-2.0)
**License:** Apache-2.0
**Issues:** [#215](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/215) (epic), [#219](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/219) (spec)

---

## The three contracts

**Perpetual Infinite Context**
The system continuously captures, processes, and indexes everything the user sees, hears, reads, and types across all sessions without lagging the system.

**Vectorized Episodic Memory**
A semantic memory layer that allows the user to query their entire digital history using vague, natural language.

**Dynamic State Hydration**
The ability to pause any complex workflow, close all windows, and instantly recreate the exact mental state, open files, and AI context weeks later in a single prompt.

Not: unbounded transformer windows, JSON dumps into a vector DB, or replaying weeks of chat. Infinite history, bounded working set.

---

## Placement

L2.5 Continuity (Capture Bus, Episode Index, Snapshot Store) sits between SFS (L2) and MemOS (L3). Complements Blueprint 49 (Mi-Memory) and Blueprint 58 (MemOS). Does not replace them.

```
L3  MemOS
L2.5 CONTINUITY — Capture Bus | Episode Index | Snapshot Store
L2  Semantic File System
```

---

## Architecture

### Capture Bus
Background ring. Drop oldest if full. Never stall the UI. Accessibility tree first; OCR only if empty. Audio/keys/clipboard/files/agent traces opt-in. Pipeline: capture → redact secrets/CARE → segment → embed async → commit. Idle CPU under 3%. Gates: Invariant 0.3, lock-screen halt, Invariant 0.8.

### Episode Index
Episode = time, device, modality, redacted text, embedding, importance, optional snapshot_id. Query: temporal anchor → dense+BM25 → contiguity → graph hop → bounded assemble. Vector store is the index, not the memory architecture.

### Snapshot Store
Immutable: mental_state, workspace W_t, ai_context, resume_contract (current_step, state_delta). Hydrate: resolve → freeze current (undo) → restore files/layout → load agent+MemCubes → apply state_delta **before** inference. Do not replay idle chat. Scale agents to zero while paused.

---

## Academic map

Paper-first. No blogs.

| Contract | Paper | ID | Use |
|----------|-------|----|-----|
| Capture / bounded compute | Infini-attention | arXiv:2404.07143 | Infinite input, bounded memory |
| Capture / bounded compute | StreamingLLM | ICLR 2024 | Attention sinks; streaming |
| Capture / OS metaphor | MemGPT | arXiv:2310.08560 | Virtual context; paging |
| Capture / memory OS | MemOS | arXiv:2507.03724 | Memory as OS resource |
| Episodes | EM-LLM (Fountas et al.) | arXiv:2407.09450 | Event segmentation; 10M-token retrieval |
| Episodes | LMEB | arXiv:2603.12572 | Long-horizon episodic embedding eval |
| Episodes | Mem-α | arXiv:2509.25911 | Timestamped episodic store + RL |
| Episodes | Memento 2 | arXiv:2512.22716 | Stateful reflective episodic memory |
| Working set | MEM1 | arXiv:2506.15841 | Constant-memory long-horizon agents |
| Hydration | Versioned workspace | arXiv:2608.18050 | Hydrate W_t at turn start |
| Hydration / trust | MemTrust | arXiv:2601.07004 | Episodic stream + semantic profile |
| Governance | Mi-Memory | arXiv:2607.18975 | Lifecycle, audit, forget |
| Governance | SuperLocalMemory 4.0 | arXiv:2608.08253 | Verified erasure |

EM-LLM is the episodic spine. MemGPT/MemOS are paging. Versioned workspace hydrates files, not chat.

---

## GitHub map

Home: this repo. Code: `gaia-memos/continuity/` (adapter, not a vendor fork). Stars as of 2026-09-09.

| Layer | Repo | Stars | Role |
|-------|------|------:|------|
| Capture | [screenpipe/screenpipe](https://github.com/screenpipe/screenpipe) | 21,506 | Local computer history |
| Capture | [screenpipe/uniOCR](https://github.com/screenpipe/uniOCR) | 228 | OCR fallback only |
| Memory API | [mem0ai/mem0](https://github.com/mem0ai/mem0) | 65,009 | Fact/preference layer |
| Graph | [getzep/graphiti](https://github.com/getzep/graphiti) | 30,736 | Temporal entity graph |
| Agent OS | [letta-ai/letta](https://github.com/letta-ai/letta) | 24,676 | Virtual context (MemGPT) |
| Hydration format | [letta-ai/agent-file](https://github.com/letta-ai/agent-file) | 1,197 | `.af` serialize |
| Memory OS | [MemTensor/MemOS](https://github.com/MemTensor/MemOS) | 11,249 | MemCubes |
| Screen→memory | [Mirix-AI/MIRIX](https://github.com/Mirix-AI/MIRIX) | 3,440 | On-screen → six memory types |

Kernel stays Apache-2.0. Wrap adapters. Do not mint new epics.

---

## Daemon / MVP

```
remember_life()        # Capture Bus; never blocks UI
ask_history(vague)     # Episode Index; top-k only
pause_world(label)     # freeze; scale to zero
restore_world(prompt)  # hydrate; state_delta before inference
```

| Week | Ship |
|------|------|
| 1 | Accessibility logger + ring + SQLite (text + time) |
| 2 | Local embeddings + FTS5; `ask_history` |
| 3 | Snapshot files/hashes/git HEAD/current_step; `restore_world` |
| 4 | Sleep/wake; export + cryptographic delete of one day |

Success: 4h capture under 3% CPU; vague recall hits the right file; next-day one-prompt restore of files + step.

---

## Important work

Do this next. Track on existing #215 / #219 only.

1. **Consent first** — every capture flag default false; lock-screen halt; CARE freeze before any logger.
2. **Implement `gaia-memos/continuity/`** — the four calls above; drop frames rather than stall.
3. **Map onto MemOS cubes** — Capture → plaintext MemCubes; snapshots → activation; frequent episodes may migrate parametric (#219).
4. **Forget/correct** — Invariant 0.8; forgotten item not retrievable; audit trace (#219 acceptance).
5. **Migration fixture** — identity survives harness A→B without a new UUID (#215 acceptance).
6. **Hydration without hallucination** — `state_delta` before inference; never infer `current_step` from chat.
7. **Adapters, not forks** — Screenpipe/Mem0/Graphiti/Letta/MIRIX behind Apache-2.0 interfaces.

---

*Blueprint 59 v0.6 canonical — tables restored. No new issues.*
