# Governance (Foundation / TSC / SIG)

Informative. A legal GAIA Foundation entity is **not** required to start implementation. This document is the placeholder so contributors know who decides what.

## Intended structure

```
GAIA Foundation (nonprofit — future)
├── Technical Steering Committee (TSC)
│   ├── Core Maintainers (elected, 2-year terms)
│   ├── Working Groups (per layer L0–L6)
│   └── Security Response Team
├── Community Council
└── Special Interest Groups
    ├── SIG-Security
    ├── SIG-Privacy
    ├── SIG-Hardware
    ├── SIG-Agents
    └── SIG-Accessibility
```

## Interim (now)

- **Owner:** `KyleAlexanderSteen` on this repository.
- **Interim TSC:** owner + anyone listed as a maintainer in a future `MAINTAINERS` file.
- **Decision rules** (from the blueprint):
  - RFC for major changes
  - Lazy consensus for minor changes
  - Supermajority (2/3) of sitting TSC for breaking protocol changes

## SIGs

SIGs may form by RFC. They do not ship code on their own; they produce recommendations the TSC accepts or rejects.

## Related issues

Phase 0 governance work: #11. Design-overlay foundation/v1.0 gate: #200.
