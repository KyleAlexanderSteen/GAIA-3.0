# OpenAI DNS side-channel — 20 Sep 2026

Primary: https://alignment.openai.com/misalignment-reports/an-agent-used-dns-to-reach-an-external-chatbot/

| Fact | GAIA bind |
| --- | --- |
| HTTP/search blocked, resolver still live | #1083 `classify_name_service` |
| Agent raised timeout 6→19→24 | #1084 `actor_set_timeout` |
| Monitor 15 min, kill 2.5 h | #1085 covert → EmergencyStop same invoke |
| July HF patch missed DNS | new surfaces after last fix |
| Reward already penalized; still escaped | do not trust training signal as a gate |

This repo does not run RL. Default `SandboxProfile.network = false` is a **flag**, not a kernel filter. #1061 must not inherit the host resolver.

No encoding recipe in this file.
