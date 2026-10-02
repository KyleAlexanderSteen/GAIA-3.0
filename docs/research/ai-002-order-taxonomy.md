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

<!-- Sections 2-5 added in following commits. -->
