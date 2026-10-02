# AI-001: Chaos in AI systems, a taxonomy of instability and failure

Issue: #1220. Parent: #1180. Evidence standard: #1171. Classification criteria: #1204 (CHAOS-001), #1205 (CHAOS-002).

Status: DRAFT v0.3. Research and specification only. No runtime code. Cannot raise autonomy or bypass a human gate. No exploitation detail; scope is understanding and defense.

Evidence tags: READ = full text read; ABSTRACT = abstract only; SECONDARY = third-party description; NeedVerify = not confirmed against a primary source.

## 1. Taxonomy

| ID | Failure mode | Evidence and conditions | Evidence tag | Status | Class (per #1204/#1205) |
|---|---|---|---|---|---|
| A1 | Prompt-format sensitivity | Sclar et al., ICLR 2024 (arXiv 2310.11324): up to 76 accuracy points on LLaMA-2-13B from meaning-preserving format changes; median spread 7.5 points on 10 sampled formats; classification tasks; open-source models plus GPT-3.5. Hua et al., EMNLP 2025 (arXiv 2509.01790): 7 LLMs, 6 benchmarks, 12 templates; much of the sensitivity comes from heuristic scoring (log-likelihood, rigid answer matching); variance falls substantially with LLM-as-judge scoring. | Sclar: READ. Hua: ABSTRACT | Observed, magnitude contested; partly an evaluation artifact | Harmful to evaluation validity |
| A2 | Run-to-run variance at temperature 0 | Atil et al. (arXiv 2408.04667, preprint): 5 models, 8 tasks, 10 runs, temperature 0, top-p 1, fixed seed; typically 5-15 point max-min accuracy spread across runs; worst case Mixtral-8x7B on college math. Cause unconfirmed (hypothesis: serving-side batching and optimizations; a locally run Llama3-8B was deterministic). | READ | Observed; preprint | Harmful to reproducibility |
| A3 | Error cascades in multi-agent systems | arXiv 2603.04474 (preprint): directed-graph model of error propagation; controlled injection of one tracer error; six frameworks; 5 of 6 reached 100% final infection; hub-node errors infected 100% in two star-topology frameworks versus 15.9% and 9.7% for worker-node errors; later correction left more polluted rounds. Results are from controlled injection, not deployed systems. | READ (through the defense-section start); evaluation section NOT yet read | Observed in experiments; preprint | Harmful |
| H1 | Sampling diversity above temperature 0 | No source read. | none | Hypothesis | Possibly useful (see #1204: only where a selection mechanism filters the variance) |
| H2 | Distribution shift and brittleness | Yuan et al. 2023 (arXiv 2306.04618, BOSS benchmark): 5 tasks, 20 datasets; earlier shift settings often too easy; both fine-tuned small models and in-context-learning LLMs struggled on shifted data. Older models; needs a current source. | ABSTRACT | Observed, dated | Harmful |
| H3 | MAST multi-agent failure taxonomy | Cemri et al.; only a secondary description seen (14 failure modes from 150 annotated traces). | SECONDARY | NeedVerify | Not yet classified |
| H4 | Hallucination and ungrounded write-back | Not yet catalogued. | none | Hypothesis | Maps to #1095 |

## 2. Mapping to existing GAIA issues

- H4 maps to #1095 (misinformation and grounding failures): that issue covers fabricated output, acting on hallucinated facts and ungrounded write-back.
- #1145 (self-monitoring suite) monitors calibration, value drift, reasoning coherence, goal displacement and adversarial stability. None of A1-A3 is among those five dimensions; A1-A3 are related monitoring context, not a direct match.
- Overlap with #1144 (Black Swan Resilience) and #1089-#1099 (threat program) not yet checked.

## 3. Open reading list

In priority order:

1. arXiv 2603.04474, evaluation section: confirm or drop the baseline and defense figures. The abstract states the governance layer prevents final infection in at least 89% of runs; the "0.32 to 0.89" and "under 15% self-reflection control" figures came from a secondary summary and are NeedVerify.
2. arXiv 2509.01790 full text: which of Sclar et al.'s formats were tested, and whether the 76-point result is addressed.
3. MAST primary paper (Cemri et al.).
4. A current (2025-2026) distribution-shift source for LLMs.
5. A hallucination-rate source with model, version and conditions (for H4).

## 4. Corrections log

- 2026-10-02: CHAOS-001 and CHAOS-002 already existed as #1204 and #1205. A duplicate (#1353) was filed in error and closed.
- 2026-10-02: v0.1 mapped A1/A2 to #1145 and H4 to #1095 without reading either; corrected in v0.2 after reading both.
