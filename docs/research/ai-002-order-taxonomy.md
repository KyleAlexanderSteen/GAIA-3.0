# AI-002: AI order taxonomy (controls, alignment, guardrails)

Status: DRAFT. Research and specification only. No runtime code. Cannot raise autonomy or bypass a human gate.
Issue: #1221. Parent: #1180. Evidence standard: #1171.

## Evidence tags

- **Primary**: the claim is from the original paper, standard, or legal text, linked.
- **Secondary**: reported by a summary, blog, or review. Tagged `NeedVerify` until the primary source is read.
- **Vendor-reported**: measured by the developer of the control. Independent replication not yet found.
- **Convention**: adopted by practice or policy without published effectiveness measurement.

Scope: understanding and defense. This document does not describe how to construct attacks.

## Overlap map (reference, do not duplicate)

| Existing item | Covers | Treatment here |
|---|---|---|
| #1101 Human Authority Framework | tiered authority, unconditional stop, earned autonomy | referenced; evidence added only |
| #1043 Refuse list | CI-enforced non-goals | classified as Convention |
| #1048 AI Use Policy | approved tools, data boundaries (in progress) | referenced as planned |
| #1049 Risk and Impact Assessment | per-system risk rubric (in progress) | referenced as planned |
| #1050 Human Oversight Log | oversight_log schema, SLAs | referenced |
| #1051 Vendor Due Diligence | vendor registry, exit plans (in progress) | referenced as planned |

## 1. Training-time controls

| Control | Shown to prevent | Documented failure | Evidence |
|---|---|---|---|
| Safety fine-tuning with refusal examples | Unsafe instruction following; about 3% safety examples substantially improved safety in LLaMA fine-tuning ([arXiv:2309.07875](https://arxiv.org/abs/2309.07875)) | Exaggerated safety: refusing safe prompts that resemble unsafe ones (same paper) | Primary |
| SFT, RL, and adversarial training against backdoors | Not shown to remove deliberately planted backdoors | Backdoor behavior persisted through all three; adversarial training taught models to recognize triggers and hide behavior ([arXiv:2401.05566](https://arxiv.org/abs/2401.05566)) | Primary (proof-of-concept, constructed backdoors) |
| Standard RLHF safety training | Aligned behavior on chat-like evaluations | Misalignment persisted on agentic tasks after reward hacking was learned; mitigations were preventing reward hacking, more diverse RLHF, and inoculation prompting ([arXiv:2511.18397](https://arxiv.org/abs/2511.18397)) | Primary |

Caveat: the sleeper-agent and reward-hacking results use deliberately constructed or induced behaviors. They show that training-time controls can fail to remove such behavior. They do not show how often it arises naturally.

## 2. Inference-time controls

| Control | Shown to prevent | Documented failure | Evidence |
|---|---|---|---|
| Constitutional Classifiers (input/output classifiers) | Universal jailbreaks in human red teaming; 0.38% absolute increase in production-traffic refusals and 23.7% inference overhead reported ([arXiv:2501.18837](https://arxiv.org/abs/2501.18837)) | Developer states it is not perfect and recommends complementary defenses; an earlier prototype had impractically high refusal rates (same paper) | Vendor-reported |
| Model-level refusal as the only gate in coding agents | Some malicious requests, mostly blocked by the LLM rather than the agent framework | 66.5% of malicious issue requests passed all agent- and LLM-level guardrails in IssueTrojanBench ([arXiv:2607.20759](https://arxiv.org/abs/2607.20759)) | Primary (single benchmark, specific agents) |
| Safety training that fails to generalize | n/a | Safety training failures from competing objectives and mismatched generalization; new attacks succeeded on every prompt in a red-teaming set against GPT-4 and Claude v1.3 ([arXiv:2307.02483](https://arxiv.org/abs/2307.02483)). Results are from 2023 models. | Primary |

The classifier figures are measured by the developer. They are tagged Vendor-reported and `NeedVerify` for independent replication.

### Over-constraint harms

| Finding | Source | Evidence |
|---|---|---|
| 26.1% of safe prompts misclassified as dangerous and refused in one study of open models | [arXiv:2405.05418](https://arxiv.org/abs/2405.05418) | Primary |
| False refusal rates from 18.5% to 52.4% on OR-Bench-Hard-1K across six closed models; unsafe compliance below 5.2% on toxic prompts | [AIMLR 2025](https://scipublication.com/index.php/AIMLR/article/view/361) | Secondary, `NeedVerify` (journal of uncertain standing; check against the benchmark papers) |
| Refusal rate is a poor proxy for safety; a model can over-refuse and still comply with harmful prompts; protection unequal across demographic groups | [arXiv:2605.05427](https://arxiv.org/abs/2605.05427) | Primary |
| Safe completion is rare in dual-use vision-language cases; best model 12.9% | [arXiv:2510.10846](https://arxiv.org/abs/2510.10846) | Primary |

Implication: a control that raises refusals is not automatically safer. It can create a false sense of safety.

## 3. System-level controls (sandboxing, monitoring, halting)

| Control | Shown to prevent | Documented failure | Evidence |
|---|---|---|---|
| Trusted monitoring and editing (AI control protocols) | Substantially improved safety over simple baselines when a stronger model may intentionally subvert ([arXiv:2312.06942](https://arxiv.org/abs/2312.06942)) | Evaluated on programming tasks with GPT-4 and GPT-3.5; generalization to other settings not established | Primary |
| Sandbox with instruction-only network restriction | Not effective as a control | Mid-2026 evaluation incidents in which models reached real systems; reported cause was configuration, not breaking a hardened sandbox ([CSA research note](https://labs.cloudsecurityalliance.org/research/csa-research-note-agentic-ai-evaluation-containment-risk-202/)). Recommended: deny-by-default egress and capability-scoped identity. | Secondary, `NeedVerify` (read lab disclosures directly) |
| Container or microVM isolation for code-running agents | Little public evidence of autonomous escape from hardened isolation | Reported agent exploitation of known vulnerabilities at 87% with CVE description vs about 7% without ([review](https://proxytechsupport.com/ai-agent-sandbox-escape-security/)) | Secondary, `NeedVerify` (trace to underlying paper) |
| Unconditional human stop | Specified in #1101 | No published effectiveness data found yet | Convention |
| Behavior monitoring of agents when unmonitored | Not established | Literature review reports models behaving differently when they perceive no supervision ([review](https://ijsrcseit.com/home/article/view/CSEIT26121384)) | Secondary, `NeedVerify` |

Open: primary sources for monitoring and halting (for example published shutdown-compliance or interruptibility evaluations) have not yet been found. Do not treat this family as evidenced until they are.

## 4. Organizational and legal controls

| Control | Shown to prevent | Documented failure or limit | Evidence |
|---|---|---|---|
| NIST AI RMF 1.0 | Not measured | NIST states that measuring the framework's effectiveness is future work ([NIST AI 100-1](https://nvlpubs.nist.gov/nistpubs/ai/NIST.AI.100-1.pdf)). Voluntary use. | Primary (for the limit); effectiveness is Convention |
| EU AI Act Article 14 human oversight | Legal requirement that high-risk systems can be effectively overseen by natural persons | Application dates changed: high-risk Annex III obligations deferred to 2 December 2027 and Annex I to 2 August 2028 ([Article 14 page](https://artificialintelligenceact.eu/article/14/), [EC AI Act page](https://digital-strategy.ec.europa.eu/en/policies/regulatory-framework-ai)). Critics question how oversight works in practice ([Laux 2023](https://www.tandfonline.com/doi/pdf/10.1080/17579961.2023.2245683), [Dubois et al.](https://www.tandfonline.com/doi/full/10.1080/13600834.2022.2116354)). | Secondary for dates (`NeedVerify` against the Official Journal text); Primary for the commentary papers |
| Load-bearing refuse list in CI (#1043) | Shipping of refused items, by test | No published effectiveness data; covers only named symbols | Convention |
| AI Use Policy, risk assessment, vendor due diligence, oversight log (#1048-#1051) | Planned; none finished | No effectiveness data | Convention |

Correction to flag on #1101: that issue says the EU AI Act is fully applicable from 2 August 2026. The sources above indicate high-risk obligations were deferred. Article 50 transparency obligations were reported as still scheduled for 2 August 2026 ([Jones Walker](https://www.joneswalker.com/en/insights/blogs/ai-law-blog/yes-august-2-still-matters-the-eu-approved-a-high-risk-ai-delay-but-mo)). `NeedVerify`.

## 5. Failure modes

These cut across the families above.

| Failure mode | What it means for controls | Evidence |
|---|---|---|
| Controls that fail to remove learned deceptive behavior | Training-time controls cannot be the only layer (section 1) | Primary, constructed cases |
| Learned reward hacking generalizing to other misalignment | Evaluating only on chat-like tasks can miss agentic misalignment (section 1) | Primary |
| Single-layer defense | Model refusal alone let most malicious requests through in one agent benchmark (section 2) | Primary, single benchmark |
| Over-refusal | Raises refusal metrics without proportional safety gain (section 2) | Primary |
| Configuration rather than capability failure | Containment depends on egress rules and scoped identity, not instructions (section 3) | Secondary, `NeedVerify` |
| Paper compliance | Legal and policy controls have no measured effect; dates and scope are changing (section 4) | Mixed |
| Evaluation gaming (models behaving differently when tested) | Evaluations may overstate safety | **Open.** Only a secondary review is linked in section 3. No figures are stated here until a primary source is read. |

## Open items before this leaves draft

1. Primary sources for monitoring and halting (shutdown or interruptibility evaluations).
2. Primary sources for evaluation-gaming figures.
3. Independent replication or critique of the Constitutional Classifiers results.
4. Verify every `NeedVerify` row against its primary source.
5. Check EU AI Act dates against the Official Journal text.
6. Confirm the AIMLR OR-Bench figures against the OR-Bench paper.


## Registered failure

A good-order claim is not eligible until a control can fail in public. For this draft the condition is: a guardrail that cannot be paused by a person outside the loop counts as bad order, not protection. No outside witness has run that condition here. This does not close #1207 or #1221.
