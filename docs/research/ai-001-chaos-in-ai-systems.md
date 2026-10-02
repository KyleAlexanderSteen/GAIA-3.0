# AI-001: Chaos in AI systems, a taxonomy of instability and failure

Issue: #1220. Parent: #1180. Evidence standard: #1171. Classification criteria: #1204 (CHAOS-001), #1205 (CHAOS-002).

Status: DRAFT v0.6. Research and specification only. No runtime code. Cannot raise autonomy or bypass a human gate. No exploitation detail; scope is understanding and defense.

Evidence tags: READ = full text read; PARTIAL = specific sections read, listed in the row; ABSTRACT = abstract only; SECONDARY = third-party description; NeedVerify = not confirmed against a primary source.

## 1. Taxonomy

| ID | Failure mode | Evidence and conditions | Evidence tag | Status | Class (per #1204/#1205) |
|---|---|---|---|---|---|
| A1 | Prompt-format sensitivity | Sclar et al., ICLR 2024 (arXiv 2310.11324): up to 76 accuracy points on LLaMA-2-13B from meaning-preserving format changes; median spread 7.5 points on 10 sampled formats. Hua et al., EMNLP 2025: much measured sensitivity is a scoring artifact; sensitivity is not zero. Mitigation comparison: arXiv 2508.11383. | Sclar: READ. Hua: READ. 2508.11383: PARTIAL | Observed; magnitude depends on scoring method | Harmful to evaluation validity |
| A2 | Run-to-run variance at temperature 0 | Atil et al. (arXiv 2408.04667, preprint): five models, eight tasks, ten runs, temperature 0, top-p 1, fixed seed; typically 5-15 point max-min accuracy spread; worst case Mixtral-8x7B on college math. Serving-side cause unconfirmed. | READ | Observed; preprint | Harmful to reproducibility |
| A3 | Error cascades in multi-agent systems | arXiv 2603.04474 (preprint): directed-graph model; one injected tracer error; six frameworks; five of six reached 100% final infection; hub-node errors infected 100% in two star topologies versus 15.9% and 9.7% for worker nodes. | PARTIAL (setup, results, Table VII) | Observed in experiments; preprint | Harmful |
| H1 | Sampling diversity above temperature 0 | No source read. | none | Hypothesis | Possibly useful only where selection filters variance (#1204) |
| H2 | Distribution shift and brittleness | LENS (arXiv 2604.17650): 192 post-deployment prompt-shift settings. The 73% figure is a head-to-head loss rate versus an in-distribution oracle, not an accuracy drop. | PARTIAL | Observed, bounded to study-trained models | Harmful |
| H3 | Multi-agent failure taxonomy (MAST) | Cemri et al. (arXiv 2503.13657, NeurIPS 2025 D&B): 14-mode taxonomy over 1,600+ traces across 7 frameworks. | PARTIAL | Observed; descriptive taxonomy | Harmful |
| H4 | Hallucination and ungrounded write-back | Hallucination mechanism and measurement sources; no per-model rate adopted. | ABSTRACT / SECONDARY | Hypothesis for GAIA; mechanism documented | Directly maps to #1095 and overlaps #1090/#1093/#1097 |

## 2. A1: prompt-format sensitivity

Source: Hua et al., ACL Anthology 2025.emnlp-main.1006 (arXiv 2509.01790), read in full.

- Seven models; six benchmarks; 12 templates per benchmark; greedy decoding. Main judge: Gemini 2.0 Flash; GPT-4o-mini cross-check on ARC-Challenge.
- Gemma-2 on ARC-Challenge ranged from 0.25 to 0.90 across templates under heuristic scoring (std 0.28), but had a 0.17 range under LLM-as-judge (std 0.005).
- Mean rank correlation across templates on ARC-Challenge went from 0.30 under heuristic scoring (four open models) to 0.92 with the judge; NarrativeQA's table says 0.59 to 0.87, while the text says 0.40 to 0.87.
- Human check: perfect agreement over all 12 templates occurred for 52% of GPQA-Diamond questions and 88% of SimpleQA questions.
- The study does not replicate or refute Sclar: it does not use LLaMA-2-13B, and varies wording and option-label styles rather than separators, spacing, and casing.

Design implication: evaluation must use semantic scoring where possible and report a distribution across templates, not a single rigid-match score.

## 3. H2: LENS distribution shift

Read: abstract, introduction, Table 1 and caption.

- 192 post-deployment prompt-shift settings across time, user group and geography; 81 study-trained models; 4.68M training prompts; 57.6k evaluation prompts. Table 1 uses Llama3-8B, Qwen2.5-7B, and Qwen2.5-14B.
- “73% average loss” means the mean share of head-to-head response comparisons where the shifted-distribution model loses to an oracle trained on the evaluation distribution. It is not a percent accuracy decline.
- This supports monitoring of natural prompt shift, but does not establish a numeric rate for frontier models.

## 4. A3: cascade evaluation

Read: evaluation setup, Table VII, and discussion paragraph of arXiv 2603.04474.

- Tasks: QUANT (UCI-derived data analysis), RIGID (MATH-derived logic/calculation), and MMLU adapted to retrieval. Six frameworks across three interaction topologies.
- Metrics: attack success rate; BICR = 1 - attack success rate; Safe Completion = usable and non-infected final artifact; token and latency cost per safe task.
- The verification layer used a self-built knowledge base and GPT-4o-mini; its NLI module used DeBERTa-v3-small.

| Mode | BICR | Safe Completion | Tokens per safe task | Latency per safe task (s) |
|---|---:|---:|---:|---:|
| Reflection | 0.32 | 0.32 | 12,749 | 91.8 |
| Speed | 0.89 | 0.88 | 21,227 | 149.7 |
| Balanced | 0.93 | 0.91 | 30,844 | 179.6 |
| Strict | 0.94 | 0.93 | 57,610 | 217.9 |
| AGrail baseline | 0.79 | 0.11 | 19,902 | 98.8 |
| CFG baseline | 0.76 | 0.16 | 13,323 | 65.3 |

Interpretation:
- 0.32 to 0.89 compares the paper's Reflection and Speed modes, not undefended versus defended operation. The abstract’s “at least 89%” is the Speed-mode BICR.
- Balanced and Strict improve BICR further but cost more. Strict uses about 4.5 times Reflection's tokens per safe task.
- The results remain controlled-injection preprint results, not a general production guarantee. The secondary “under 15% self-reflection control” claim is not supported and is dropped.

## 5. H3: MAST taxonomy

Read: abstract, introduction, Figure 1, and category-1 discussion.

- MAST-Data contains 1,600+ annotated traces across seven multi-agent frameworks. The taxonomy was built from 150 traces with three human experts, with inter-annotator kappa 0.88. The paper says its LLM annotator achieved kappa 0.77 against human annotations.

| Category | Share | Modes (share) |
|---|---:|---|
| System design issues | 44.2% | Disobey task spec 11.8%; role spec 1.5%; step repetition 15.7%; context loss 2.8%; termination-unaware 12.4% |
| Inter-agent misalignment | 32.3% | Conversation reset 2.2%; no clarification 6.8%; derailment 7.4%; withholding 0.8%; ignored input 1.9%; reasoning-action mismatch 13.2% |
| Task verification | 23.5% | Premature termination 6.2%; incomplete verification 8.2%; incorrect verification 9.1% |

MAST describes observed trace failures; it does not measure A3's controlled propagation. Its verification category is where a cascade-control layer can act.

## 6. A1 mitigation comparison

Read: introduction, method list, frontier-model section, conclusion of arXiv 2508.11383.

- It compares Batch Calibration, Template Ensembles, Sensitivity-Aware Decoding, LoRA with format augmentation, and LoRA with consistency loss, including descriptor, separator and spacing variation.
- The authors conclude that calibration methods are fragile under class imbalance and light supervised fine-tuning with augmentations did not improve robustness.
- GPT-4.1 and DeepSeek V3 were more robust, but some tasks still varied by format. Several methods require logits, which many frontier APIs do not expose.
- No single “best method” is claimed here because the per-method result tables were not used to make one.

## 7. H4: hallucination and write-back

- Kalai et al., “Why Language Models Hallucinate” (arXiv 2509.04664) attributes hallucination in part to training and evaluation incentives that reward guessing rather than calibrated abstention.
- Vectara's FaithJudge work (arXiv 2505.04847) benchmarks faithfulness in retrieval-grounded summarization, QA, and data-to-text; AA-Omniscience has a separate closed-book calibration-oriented definition.
- No universal hallucination rate is recorded: results require a named model/version, task, context, and scorer.
- GAIA implication: grounding tests should reject unsupported factual claims and ungrounded writes, while scoring appropriately calibrated abstention rather than accuracy alone.

## 8. Completed overlap review: #1089-#1099 and #1144

The review below is a scope-and-control comparison, not a claim that the named issues are implemented.

| Issue(s) | Relation to AI-001 | Required treatment in this research file |
|---|---|---|
| #1089 prompt injection | Indirect injection, slow-drift manipulation, and cross-agent propagation can seed A3 cascades. | A3 must treat injected content as a distinct seed source; depend on #1089's `UntrustedContent` handling. |
| #1090 sensitive disclosure | A hallucinated or injected artifact can expose secrets or hidden context. | H4 output/write-back evidence is related, but #1090 owns disclosure and egress controls. |
| #1091 excessive agency | A3 can turn one wrong premise into delegated action chains; #1091 owns authority/delegation limits. | Cascade mitigation needs #1091’s depth and autonomy caps. |
| #1092 supply chain | Poisoned skills/MCP servers/RAG sources can seed H4 or A3. | Treat supply-chain provenance as a source-control dependency, not a chaos taxonomy control. |
| #1093 data/model/memory poisoning | Persistent memory or poisoned RAG can preserve and replay H4 misinformation and cascade seeds. | H4 write-back protection depends on #1093 memory authorization/provenance. |
| #1094 unbounded consumption | A3 amplification can consume tokens, calls, and sub-agent budget. | Cascade circuit breaker needs #1094 quota/call-depth enforcement. |
| #1095 misinformation/grounding | Direct home for H4: grounding gate, citation policy, hallucination fixtures, and ungrounded write prevention. | H4 is mapped here; no duplicate implementation belongs in AI-001. |
| #1096 vector/embedding | Adversarial retrieval ranking and tenant failures are a route for H4 and injected A3 seeds. | Depend on retrieval trust labels and namespace isolation. |
| #1097 improper output handling | Propagating output into another agent is an A3 cross-agent path; ungrounded output can be downstream injection. | Depend on its output sanitization / `UntrustedContent` gate. |
| #1098 rogue deployment | Misconfiguration can remove oversight, quota, or stop controls that limit cascades. | Deployment gate must check circuit-breaker and policy configuration. |
| #1099 multi-agent cascade | Direct operational home for A3 and closest implementation locus for MAST's inter-agent and verification failures. | Add this research result as evidence; #1099 owns trust-chain, breaker, identity, orphan and fixture work. |
| #1144 Black Swan Resilience | Direct conceptual home for H2: OOD flagging and vulnerability mapping. | Add LENS as evidence for prompt-shift monitoring; it does not convert black-swan claims into prediction. |

Conclusion of review:
- A3 belongs operationally in #1099, with dependencies on #1089, #1091, #1094, #1097, and #1098.
- H4 belongs operationally in #1095, with dependencies on #1090, #1093, #1096, and #1097.
- H2 belongs in #1144’s OOD detector and honesty layer.
- A1 and A2 have no direct child in #1089-#1099. They remain evaluation- and reproducibility-harness requirements; do not force them into a security-threat issue.
- Numbering inconsistency found: #1099 is titled Threat #11 but uses fixture path `threat-12`. Preserve the issue’s existing title in citations and correct its fixture path/title before implementation.

## 9. Reading completion ledger

Completed to the evidence level used here:
- Hua et al. EMNLP 2025: full text.
- Cascade paper arXiv 2603.04474: evaluation setup, main safety-cost table, and interpretation.
- MAST: taxonomy, category/mode frequencies, data scale, human and LLM agreement figures.
- LENS: study scope and the precise definition of the 73% value.
- Robustness-methods paper: methods, stated conclusions and frontier-model limits.
- H4: a mechanism paper plus two measurement-framework sources.
- #1144 and every issue #1089 through #1099: individual scope review completed.

Not converted into claimed findings:
- Per-method winner in arXiv 2508.11383.
- Per-framework cascade breakdown or full Reflection-mode mechanics.
- LENS shift-measurement implementation details.
- A universal hallucination rate or GAIA numeric threshold.

Those values are intentionally absent rather than treated as unfinished work. They are not needed to support the taxonomy, mappings, or design implications in this document.

## 10. Corrections log

- 2026-10-02: CHAOS-001 and CHAOS-002 already existed as #1204 and #1205. Duplicate #1353 was filed in error and closed.
- 2026-10-02: early mapping put A1/A2 under #1145 and H4 under #1095 without reading both; corrected after review.
- 2026-10-02: early EMNLP summary was abstract-only; full text does not support “sensitivity is negligible.”
- 2026-10-02: LENS “73% average loss” was initially misread as an accuracy loss. It is a head-to-head loss rate against an oracle. Corrected.
- 2026-10-02: 0.32 and 0.89 were initially described as baseline-versus-defense figures. Table VII makes them Reflection- versus Speed-mode BICR. Corrected.
- 2026-10-02: overlap review expanded from parent-table inspection to individual review of #1089 through #1099.
