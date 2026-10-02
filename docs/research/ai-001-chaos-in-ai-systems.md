# AI-001: Chaos in AI systems, a taxonomy of instability and failure

Issue: #1220. Parent: #1180. Evidence standard: #1171. Classification criteria: #1204 (CHAOS-001), #1205 (CHAOS-002).

Status: DRAFT v0.3. Research and specification only. No runtime code. Cannot raise autonomy or bypass a human gate. No exploitation detail; scope is understanding and defense.

Evidence tags: READ = full text read; ABSTRACT = abstract only; SECONDARY = third-party description; NeedVerify = not confirmed against a primary source.

## 1. Taxonomy

| ID | Failure mode | Evidence and conditions | Evidence tag | Status | Class (per #1204/#1205) |
|---|---|---|---|---|---|
| A1 | Prompt-format sensitivity | See section 2. Sclar et al., ICLR 2024 (arXiv 2310.11324): up to 76 accuracy points on LLaMA-2-13B from meaning-preserving format changes; median spread 7.5 points on 10 sampled formats; classification tasks. Hua et al., EMNLP 2025: much of the measured sensitivity is an artifact of heuristic scoring; not a claim that sensitivity is zero. | Sclar: READ. Hua: READ | Observed; magnitude depends on the scoring method | Harmful to evaluation validity |
| A2 | Run-to-run variance at temperature 0 | Atil et al. (arXiv 2408.04667, preprint): 5 models, 8 tasks, 10 runs, temperature 0, top-p 1, fixed seed; typically 5-15 point max-min accuracy spread across runs; worst case Mixtral-8x7B on college math. Cause unconfirmed (hypothesis: serving-side batching and optimizations; a locally run Llama3-8B was deterministic). | READ | Observed; preprint | Harmful to reproducibility |
| A3 | Error cascades in multi-agent systems | arXiv 2603.04474 (preprint): directed-graph model; controlled injection of one tracer error; six frameworks; 5 of 6 reached 100% final infection; hub-node errors infected 100% in two star-topology frameworks versus 15.9% and 9.7% for worker-node errors; later correction left more polluted rounds. Controlled injection, not deployed systems. Evaluation section (defense figures) NOT yet read. | READ (partial) | Observed in experiments; preprint | Harmful |
| H1 | Sampling diversity above temperature 0 | No source read. | none | Hypothesis | Possibly useful (see #1204: only where a selection mechanism filters the variance) |
| H2 | Distribution shift and brittleness | Yuan et al. 2023 (arXiv 2306.04618, BOSS): 5 tasks, 20 datasets; both fine-tuned small models and in-context-learning LLMs struggled on shifted data. Older models; needs a current source. | ABSTRACT | Observed, dated | Harmful |
| H3 | MAST multi-agent failure taxonomy | Cemri et al.; only a secondary description seen (14 failure modes from 150 annotated traces). | SECONDARY | NeedVerify | Not yet classified |
| H4 | Hallucination and ungrounded write-back | Not yet catalogued. | none | Hypothesis | Maps to #1095 |

## 2. A1 detail: Hua et al. (EMNLP 2025) read in full

Source: ACL Anthology 2025.emnlp-main.1006 (arXiv 2509.01790).

Setup:
- 7 models: LLaMA-3.1-8B-Instruct, Qwen2-7B-Instruct, Gemma-2-9B-it, Ministral-8B-Instruct, GPT-4o-mini, GPT-4.1-mini, Gemini 2.0 Flash.
- 6 benchmarks: ARC-Challenge, GPQA-diamond, OpenbookQA (multiple choice, heuristic = log-likelihood over options); NarrativeQA, MATH, SimpleQA (open-ended).
- 12 prompt templates per benchmark, paraphrased with GPT-4o. Greedy decoding. Main judge: Gemini 2.0 Flash; GPT-4o-mini cross-check on ARC-Challenge only.

Findings:
- Gemma-2 on ARC-Challenge: accuracy ranged 0.25 to 0.90 across templates (std 0.28) under heuristic scoring, versus a 0.17 range (std 0.005) under LLM-as-judge.
- Mean Spearman rank correlation of model rankings across templates: ARC-Challenge 0.30 (heuristic, 4 open models) to 0.92 (judge, same 4 models) and 0.95 (all 7). NarrativeQA 0.59 to 0.87 in Table 1 (the text gives 0.40 to 0.87; the two differ in the paper).
- MATH: a well-built heuristic (symbolic simplification, equivalence checking) gave stability comparable to the judge (0.9593 vs 0.9647).
- Human check: 50 questions per dataset, Fleiss kappa above 0.6; perfect agreement across all 12 templates on 52% (GPQA-Diamond) to 88% (SimpleQA) of questions.

What it does NOT show:
- Sensitivity is not zero. Even under the judge, Gemma-2's ARC accuracy still spans 0.17, and GPQA-Diamond agreement across templates is only 52%.
- It does not address the 76-point result. Sclar et al. (cited as arXiv 2310.11324) appear only in the introduction and related work as one of several studies reporting prompt sensitivity.
- Templates vary instruction wording and option-label style (A/B/C/D, 1/2/3/4, [A], Option A). From Appendix A, I saw no systematic variation of separators, spacing or casing, which are the feature classes Sclar et al. sampled. Treat the two studies as measuring different things, not as a direct replication or refutation.
- Models are 7-9B open models plus three small proprietary models; no LLaMA-2-13B. The judge is itself an LLM.

Working conclusion for A1: keep it as Observed, but record that reported magnitudes depend on how answers are scored. A GAIA evaluation harness should not rely on log-likelihood or rigid string matching alone, and should report variance across several templates.

## 3. Mapping to existing GAIA issues

- H4 maps to #1095 (misinformation and grounding failures).
- #1145 (self-monitoring suite) monitors calibration, value drift, reasoning coherence, goal displacement and adversarial stability. None of A1-A3 is among those five dimensions; related context, not a direct match.
- Overlap with #1144 (Black Swan Resilience) and #1089-#1099 (threat program) not yet checked.

## 4. Open reading list

1. arXiv 2603.04474, evaluation section: confirm or drop the baseline and defense figures. The abstract states the governance layer prevents final infection in at least 89% of runs; the "0.32 to 0.89" and "under 15% self-reflection control" figures came from a secondary summary and are NeedVerify.
2. MAST primary paper (Cemri et al.).
3. A current (2025-2026) distribution-shift source for LLMs.
4. A hallucination-rate source with model, version and conditions (for H4).
5. Done: Hua et al., EMNLP 2025 (section 2).

## 5. Corrections log

- 2026-10-02: CHAOS-001 and CHAOS-002 already existed as #1204 and #1205. A duplicate (#1353) was filed in error and closed.
- 2026-10-02: v0.1 mapped A1/A2 to #1145 and H4 to #1095 without reading either; corrected in v0.2.
- 2026-10-02: v0.2 described the EMNLP paper from its abstract only. v0.3 reflects the full text; it does not support the claim that sensitivity is negligible.
