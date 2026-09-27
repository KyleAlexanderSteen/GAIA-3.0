# Agent-failure simulation index — #1036 / #1039

Not a crystal ball. Offline gates only.

| Surface | Issue | Outcome language |
| --- | --- | --- |
| `run_fixture` | #1039 | Pass / Fail / NeedVerify / OutOfScope |
| Eval-awareness catalog | #1040 | GatePresent / Absent |
| Human halt | #1045 | actor cannot resume |
| Independent verdict | #1042 | no self-grade |
| Refuse list CI | #1043 | crate names + catalog flag |
| Conflict chunks | #1039 leftover | NeedVerify only; no auto-resolve |

Run: `cargo test -p gaia-acp run_fixture -- --nocapture` and `cargo test -p gaia-acp scenario_gate`.

Does not predict 2027 rogue SI.
