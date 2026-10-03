# Security pillars — listed (#868)

**Points at:** `gaia-security`, `gaia-acp`, `gaia-runtime`, `gaia-gateway`.
**Implementation epic stays open:** [#905](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/905).

| Pillar | Existing surface | Status |
| --- | --- | --- |
| Input | gateway health + listed HSPD/AISPD | no injection classifier |
| Retrieval | chunk store + citations + faithfulness | no poison detector |
| Output | `enforce_grounding`, NeedVerify | lexical only |
| Prompt | listed AISPD/HSPD | no extraction detector |
| Tool | `gaia-acp` authorize + audit | no wallet / no sandbox host |
| Runtime | boot/cli tests | no circuit breaker |

## Refuse

No new `gaia-security` APIs. No threat-intel feed. No Security Tablet seal.
