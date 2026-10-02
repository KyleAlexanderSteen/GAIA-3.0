# Multi-agent trust policy (#1099)

**Status:** TEST stage. Fixtures exist in `gaia-acp/tests/multi_agent.rs`; they have not been run locally by the author, so CI is the first execution.

## Rules the fixtures enforce

1. An action is only valid under the manifest issued to its own `agent_id`. A different agent reusing it is denied with `CrossAgent`.
2. Output from another agent is `UntrustedContent`. Claims of approval or authority inside it are denied with `UntrustedAuthority`.
3. Delegation requests (`wants_delegation`) are denied with `DelegationDenied`.
4. A killed agent cannot act (`EmergencyStop` or `StateInvalid`).
5. A manifest whose action budget is used up is denied with `BudgetExceeded`, which bounds how far an error can cascade.
6. Every denial is written to the audit chain, and the chain must stay verifiable.

## Not covered yet

- Cascade behaviour across several real agents. These tests use one plane and one manifest.
- Any measured defense effectiveness. No numbers are claimed here.
