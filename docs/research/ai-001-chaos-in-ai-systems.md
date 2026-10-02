# AI-001: Chaos in AI systems, a taxonomy of instability and failure

Issue: #1220. Parent: #1180. Evidence standard: #1171. Classification criteria: #1204 (CHAOS-001), #1205 (CHAOS-002).

Status: DRAFT v0.5. Research and specification only. No runtime code. Cannot raise autonomy or bypass a human gate. No exploitation detail; scope is understanding and defense.

Evidence tags: READ = full text read; PARTIAL = specific sections read, listed in the row; ABSTRACT = abstract only; SECONDARY = third-party description; NeedVerify = not confirmed against a primary source.

## 1. Taxonomy

| ID | Failure mode | Evidence and conditions | Evidence tag | Status | Class (per #1204/#1205) |
|---|---|---|---|---|---|
| A1 | Prompt-format sensitivity | Section 2. Sclar et al., ICLR 2024 (arXiv 2310.11324): up to 76 accuracy points on LLaMA-2-13B from meaning-preserving format changes; median spread 7.5 points on 10 sampled formats. Hua et al., EMNLP 2025: much measured sensitivity is an artifact of heuristic scoring; not zero. Mitigation comparison: arXiv 2508.11383 (section 6). | Sclar: READ. Hua: READ. 2508.11383: PARTIAL | Observed; magnitude depends on scoring method | Harmful to evaluation validity |
| A2 | Run-to-run variance at temperature 0 | Atil et al. (arXiv 2408.04667, preprint): 5 models, 8 tasks, 10 runs, temperature 0, top-p 1, fixed seed; typically 5-15 point max-min accuracy spread; worst case Mixtral-8x7B on college math. Cause unconfirmed (hypothesis: serving-side batching; a locally run Llama3-8B was deterministic). | READ | Observed; preprint | Harmful to reproducibility |
| A3 | Error cascades in multi-agent systems | arXiv 2603.04474 (preprint): directed-graph model; one injected tracer error; six frameworks; 5 of 6 reached 100% final infection; hub-node errors infected 100% in two star-topology frameworks versus 15.9% and 9.7% for worker-node errors. Controlled injection, not deployed systems. Defense results: section 4. | PARTIAL (setup, results, Table VII) | Observed in experiments; preprint | Harmful |
| H1 | Sampling diversity above temperature 0 | No source read. | none | Hypothesis | Possibly useful only where a selection mechanism filters the variance (#1204) |
| H2 | Distribution shift and brittleness | LENS (arXiv 2604.17650, Apr 2026), section 3. Older: Yuan et al. 2023 (arXiv 2306.04618, BOSS). | LENS: PARTIAL. BOSS: ABSTRACT | Observed; LENS 73% is a head-to-head loss rate, not an accuracy drop | Harmful |
| H3 | Multi-agent failure taxonomy (MAST) | Cemri et al. (arXiv 2503.13657, NeurIPS 2025 D&B): section 5. | PARTIAL (taxonomy figure, FC1 text, intro) | Observed; descriptive taxonomy | Harmful (failure taxonomy) |
| H4 | Hallucination and ungrounded write-back | Section 7. Mechanism sources found; no per-model rate adopted. | ABSTRACT / SECONDARY | Hypothesis for GAIA; mechanism well documented | Maps to #1095 |

## 2. A1 detail: Hua et al. (EMNLP 2025) read in full

Source: ACL Anthology 2025.emnlp-main.1006 (arXiv 2509.01790).

Setup:
- 7 models: LLaMA-3.1-8B-Instruct, Qwen2-7B-Instruct, Gemma-2-9B-it, Ministral-8B-Instruct, GPT-4o-mini, GPT-4.1-mini, Gemini 2.0 Flash.
- 6 benchmarks: ARC-Challenge, GPQA-diamond, OpenbookQA (multiple choice, heuristic = log-likelihood over options); NarrativeQA, MATH, SimpleQA (open-ended).
- 12 prompt templates per benchmark, paraphrased with GPT-4o. Greedy decoding. Main judge: Gemini 2.0 Flash; GPT-4o-mini cross-check on ARC-Challenge only.

Findings:
- Gemma-2 on ARC-Challenge: accuracy ranged 0.25 to 0.90 across templates (std 0.28) under heuristic scoring, versus a 0.17 range (std 0.005) under LLM-as-judge.
- Mean Spearman rank correlation of model rankings across templates: ARC-Challenge 0.30 (heuristic, 4 open models) to 0.92 (judge, same 4 models) and 0.95 (all 7). NarrativeQA 0.59 to 0.87 in Table 1 (the text gives 0.40 to 0.87; the two differ in the paper).
- MATH: a well-built heuristic gave stability comparable to the judge (0.9593 vs 0.9647).
- Human check: 50 questions per dataset, Fleiss kappa above 0.6; perfect agreement across all 12 templates on 52% (GPQA-Diamond) to 88% (SimpleQA) of questions.

What it does NOT show:
- Sensitivity is not zero. Even under the judge, Gemma-2's ARC accuracy spans 0.17, and GPQA-Diamond agreement across templates is only 52%.
- It does not address the 76-point result. Sclar et al. appear only in the introduction and related work.
- Templates vary instruction wording and option-label style. From Appendix A, no systematic variation of separators, spacing or casing was seen; those are the feature classes Sclar et al. sampled. The two studies measure different things.
- Models are 7-9B open models plus three small proprietary ones; no LLaMA-2-13B. The judge is itself an LLM.

Working conclusion for A1: keep as Observed, and record that reported magnitudes depend on how answers are scored. A GAIA evaluation harness should not rely on log-likelihood or rigid string matching alone, and should report variance across several templates.

## 3. H2 detail: LENS (arXiv 2604.17650)

Read: abstract, introduction, Table 1 and its caption. Not read: the shift-measurement method in full, appendices.

- Scale: 192 post-deployment prompt-shift settings over time, user group and geography; 81 models trained on 4.68M prompts; evaluated on 57.6k prompts. Models in Table 1: Llama3-8B, Qwen2.5-7B, Qwen2.5-14B, fine-tuned by the authors.
- What the 73% is: the Table 1 caption defines Loss Rate as the share of head-to-head response comparisons in which the model trained on shifted-distribution prompts does worse than an oracle model trained on prompts from the evaluation distribution. The 73% is the average loss rate across the three shift axes. It is NOT a 73% drop in accuracy. The abstract wording ("73% average loss") is easy to misread, and v0.4 of this file misread it as a performance loss.
- Caveat: these are models fine-tuned for the study and compared against same-distribution counterparts. The figure does not transfer to off-the-shelf frontier models.
- Use: supports the claim that natural prompt shift measurably degrades instruction-following in fine-tuned models, and that monitoring is warranted. Do not quote 73% without the loss-rate definition.

## 4. A3 detail: arXiv 2603.04474 evaluation (resolves earlier open item)

Read: evaluation setup (section VII.A), Table VII, and the paragraph discussing it. Not read: appendices, Table VI ablation in detail.

Setup:
- Three scenarios: QUANT (data analysis from UCI tasks), RIGID (multi-step logic and calculation from MATH), MMLU (retrieval-style knowledge questions). Six MAS frameworks across three topologies.
- Metrics: Attack Success Rate (final artifact labeled infected); Benign Infection Control Rate, BICR = 1 - ASR; Safe Completion (usable and non-infected artifact); Token/Safe; Latency/Safe.
- Verification layer: self-built knowledge base and GPT-4o-mini; NLI module based on DeBERTa-v3-small.

Table VII (averaged safety-cost trade-off):

| Mode | BICR | Safe Completion | Tokens per safe task | Latency per safe task (s) |
|---|---|---|---|---|
| Reflection | 0.32 | 0.32 | 12,749 | 91.8 |
| Speed | 0.89 | 0.88 | 21,227 | 149.7 |
| Balanced | 0.93 | 0.91 | 30,844 | 179.6 |
| Strict | 0.94 | 0.93 | 57,610 | 217.9 |
| AGrail (baseline) | 0.79 | 0.11 | 19,902 | 98.8 |
| CFG (baseline) | 0.76 | 0.16 | 13,323 | 65.3 |

What this means:
- The 0.32 to 0.89 figures are BICR for the lowest-cost Reflection mode versus the Speed mode of the authors' own governance layer. They are not a comparison with an undefended system, and not the strongest alternative: the two external baselines reach BICR 0.79 and 0.76 but complete safely only 11% and 16% of the time.
- Stronger modes cost more: Strict roughly quadruples tokens per safe task relative to Reflection (57,610 vs 12,749).
- The abstract's "at least 89% of runs" matches the Speed row. Results are averages over scenarios and frameworks; per-framework variation was not read.
- The Reflection-mode definition was not read in detail. The earlier "under 15% self-reflection control" figure from a secondary summary does not appear in anything read; drop it.
- Still a preprint, still controlled injection in research frameworks. The figures are usable as "one published result, with its conditions", not as a general defense success rate.

## 5. H3 detail: MAST (arXiv 2503.13657)

Read: abstract, introduction, taxonomy figure (Figure 1), and the failure-category-1 discussion. Not read: LLM-annotator validation details, appendices, intervention studies.

- Data and method: MAST-Data of 1600+ annotated traces across 7 frameworks; taxonomy built from 150 traces with three expert annotators and inter-annotator kappa 0.88; an LLM annotator reached kappa 0.77 against human experts (stated in the introduction).
- Share of failures by category and mode, as printed in Figure 1 and the text:

| Category | Share | Modes (share) |
|---|---|---|
| FC1 System design issues | 44.2% | Disobey task spec 11.8%; disobey role spec 1.5%; step repetition 15.7%; loss of conversation history 2.8%; unaware of termination conditions 12.4% |
| FC2 Inter-agent misalignment | 32.3% | Conversation reset 2.2%; fail to ask for clarification 6.8%; task derailment 7.4%; information withholding 0.8%; ignored other agent's input 1.9%; reasoning-action mismatch 13.2% |
| FC3 Task verification | 23.5% | Premature termination 6.2%; no or incomplete verification 8.2%; incorrect verification 9.1% |

- The mode shares within each category sum to the category share, which checks the figure extraction.
- The authors argue many FC1 failures reflect system design, not only base-model limits, and that better role specification alone does not remove them (intervention studies in their Appendix H, not read).
- Link to A3: MAST is descriptive of failures seen in traces; A3 is a controlled propagation experiment. Verification failures (FC3, 23.5%) are the category where a cascade defense would act.

## 6. A1 mitigation: arXiv 2508.11383

Read: introduction, method list, frontier-model section, conclusion. Not read: per-method result tables.

- Compares prompt-robustness methods (Batch Calibration, Template Ensembles, Sensitivity-Aware Decoding, LoRA with format augmentation, LoRA with consistency loss) against standard few-shot and fine-tuning, on classification and multiple-choice tasks.
- Format components include descriptors, separators and spacing, the same feature classes Sclar et al. varied.
- Conclusion states: calibration-based methods are fragile to class imbalance; light supervised fine-tuning with augmentations did not improve robustness; frontier models (GPT-4.1, DeepSeek V3, tested on 10 of 52 tasks) are substantially more robust, yet some tasks still show spread across formats.
- Limits stated by the authors: classification and multiple choice only; several methods need logits that frontier APIs often do not expose.
- Which single method works best was not determined from the sections read.

## 7. H4: hallucination sources

- Mechanism: Kalai et al., "Why Language Models Hallucinate" (arXiv 2509.04664; OpenAI summary Sep 2025) argue hallucination persists because training and evaluation reward guessing over acknowledging uncertainty; accuracy-only scoring favors a guessing model. Proposed fix: penalize confident errors more than abstention and give partial credit for expressed uncertainty. Tag: ABSTRACT plus vendor summary.
- Measurement, RAG setting: Vectara leaderboard and FaithJudge (arXiv 2505.04847) track hallucination rates in summarization, question answering and data-to-text with retrieval context. Tag: ABSTRACT.
- Measurement, closed-book: Artificial Analysis AA-Omniscience defines hallucination rate as incorrect / (incorrect + partial + not attempted) across 6,000 questions. Tag: SECONDARY (benchmark page).
- No per-model rate is adopted here: rates change by model version, task and scorer, and none was read from a primary table. Any GAIA threshold needs a named model, version and benchmark.
- Design implication for #1095: a GAIA grounding check should score abstention explicitly rather than accuracy alone.

## 8. Mapping to existing GAIA issues

- H4 maps to #1095 (misinformation and grounding failures).
- #1144 (Predict-3, Black Swan Resilience): proposes an out-of-distribution detector (Mahalanobis distance, density-based methods, rolling baseline) that emits an explicit low-confidence flag. This is the closest existing home for H2. LENS gives evidence that shift in user prompts degrades fine-tuned models; #1144 does not yet cite it. Design-stage issue, no implementation.
- #1088 (AI Threat Hardening Program, parent) lists a child for "Cross-agent trust exploitation and multi-agent cascades".
- #1099 (Threat #11): that child. It names cascade amplification and trust-chain inflation, notes a missing cascade circuit breaker and trust-chain depth limit, and requires a design note for the breaker. Direct overlap with A3 and H3. A3's hub-versus-worker result (100% vs 15.9% and 9.7%) is relevant evidence for where a breaker or identity check matters most. Housekeeping: #1099 is titled Threat #11 but its fixture path is `threat-12`; flag to the issue owner.
- #1089-#1098 were not individually read; only #1095 (via earlier mapping) and the parent's table of attack classes were checked. Other children are listed in #1088 by class, none of which names chaos, variance or prompt-format sensitivity. Not a substitute for reading each.
- #1145 (self-monitoring suite) monitors calibration, value drift, reasoning coherence, goal displacement and adversarial stability. None of A1-A3 is among those; MAST categories are a plausible input to reasoning coherence and goal displacement (an inference from category names).

## 9. Open reading list

1. MAST: LLM-annotator validation and the intervention studies (Appendix H).
2. LENS: the shift-measurement method and appendices.
3. arXiv 2508.11383: per-method result tables.
4. arXiv 2603.04474: per-framework breakdown and the Reflection-mode definition.
5. A primary-table hallucination rate with model, version and conditions, if GAIA needs a numeric threshold.
6. Read #1089-#1098 individually for overlap.
7. Done: Hua et al. (section 2); 2603.04474 evaluation (section 4); MAST taxonomy (section 5); LENS (section 3); 2508.11383 (section 6); H4 sources (section 7); #1144 and #1099 overlap (section 8).

## 10. Corrections log

- 2026-10-02: CHAOS-001 and CHAOS-002 already existed as #1204 and #1205. A duplicate (#1353) was filed in error and closed.
- 2026-10-02: v0.1 mapped A1/A2 to #1145 and H4 to #1095 without reading either; corrected in v0.2.
- 2026-10-02: v0.2 described the EMNLP paper from its abstract only. v0.3 reflects the full text; it does not support the claim that sensitivity is negligible.
- 2026-10-02: earlier text described MAST as "14 failure modes from 150 annotated traces" from a secondary source. Primary abstract: 150 traces built the taxonomy; the dataset has 1600+ annotated traces.
- 2026-10-02: v0.4 and earlier read LENS "73% average performance loss" as an accuracy drop. Table 1 defines it as a head-to-head loss rate against an oracle model. Corrected in v0.5.
- 2026-10-02: v0.4 treated 0.32 and 0.89 as a baseline versus defense comparison of unknown conditions. Table VII shows they are BICR for Reflection and Speed modes of the authors' own system; external baselines scored 0.79 and 0.76. Corrected in v0.5.
