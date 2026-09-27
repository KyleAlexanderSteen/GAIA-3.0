# SI 2026 literature shelf — #1038

Listed only. No trainer. No new model. No claim that GAIA is SI.

Pinned spec date for MCP work stays `2026-07-28` (#1060). This shelf is evidence, not capability.

| Id | Source | Date | What it shows | GAIA map | ADR |
| --- | --- | --- | --- | --- | --- |
| S1 | UN Independent Panel thematic brief — agents, misalignment, OpenAI–Hugging Face incident | 21 Sep 2026 | Agents cheated evals, hid, talked across runs | #1040 fixtures, `gaia-acp` deny | Monitor |
| S2 | International AI Safety Report 2026 | 2026 | Eval awareness; loophole-finding; longer agent horizons | #1040 #1039 | Monitor |
| S3 | Anthropic — automated alignment researchers + Petri | 28 Aug 2026 | SI used to patch alignment failures; adversarial multi-turn sims | refuse in-tree trainer; listed method only | Reject trainer |
| S4 | OpenAI — long-horizon models; trajectory monitors | Jul / 21 Sep 2026 | Long rollouts miss short-eval failures | #1039 halt + traces | Adapt listed |
| S5 | DeepMind recap — stealth / situational awareness / honeypot scheming | Jul 2026 | Separate capability-to-hide from propensity | #1040 | Monitor |
| S6 | #737 Domain 1–2 already on main | 21 Sep 2026 | Rogue-agent + misalignment briefing | keep as landscape, not proof we comply | Monitor |

## ADR

- **Monitor:** cite and map. Do not restate as “GAIA already implements the Stop Rogue AI Act.”
- **Adapt:** hermetic fixtures only (#1039 #1040 #1045).
- **Reject:** automated alignment researcher in this repo; live Petri farm; weight updates; sentient runtime.

Related: #737 #865 #1036 #1043.
