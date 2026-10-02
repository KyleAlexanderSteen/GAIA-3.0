# Multi-agent circuit breaker note (#1099)

**Status:** design note. Nothing here is implemented beyond what the existing budget and kill controls already do.

## Current mechanism

The action budget (`max_actions`) and `kill` are the only brakes today. Both are per agent.

## Gap

No control trips across agents. If agent A feeds agent B bad output, B's own budget does not know. A cross-agent breaker would need shared state across planes, which `ControlPlane` does not have.

## Open question

Whether the breaker belongs in the control plane or in the gateway. Left for a decision in #1099.
