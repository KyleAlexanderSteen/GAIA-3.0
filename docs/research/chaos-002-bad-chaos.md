# CHAOS-002: Bad chaos — what turns uncertainty into collapse

Issue: #1205 (parent #1175). Related: #1200, #1143, #1099. Evidence standard: #1171.
Status: DRAFT v0.1. Research and specification only; no runtime code; cannot raise autonomy or bypass a human gate.
Scope note: mechanisms and dynamics only. No profiling of real people or groups.

## Stage reached

- Read: search-result abstracts and excerpts for the sources below. Full papers were NOT read.
- Covered: load-redistribution cascades, interdependent networks, financial contagion, threshold cascades.
- NOT covered yet (NeedVerify, no sources collected): misinformation spread, extremism dynamics, addiction dynamics, ecosystem failure, catastrophe theory, and the same-trigger-different-outcome comparison across real cases.

## Chaos-risk model (v0.1)

A small disturbance grows into collapse when a propagation rule exists, the system is close enough to a threshold, and nothing absorbs the spread. The mechanisms below each list observed cases, a reported condition, and counterexamples.

| # | Mechanism | Reported condition | Observed case | Counterexample or limit | Source |
|---|-----------|--------------------|---------------|-------------------------|--------|
| M1 | Load redistribution (overload cascade) | Loads redistribute when a node fails; heterogeneous loads | Internet and power-grid models; 2003 US-Canada style blackouts are cited in the grid literature | Tuning capacity nonlinearly with centrality can improve robustness | Motter and Lai, Phys. Rev. E 66, 065102; Sci. Open 2024 (excerpt) |
| M2 | Interdependent-network cascade | Nodes in one network depend on nodes in another | Italy blackout, 28 Sept 2003 (power stations and internet nodes) | Result is for the model's assumptions (for example two Erdos-Renyi networks) | Buldyrev et al., Nature 464, 1025 (2010) |
| M3 | Financial contagion | Shock size and number relative to a critical threshold | Interbank networks (models; Brazilian system study) | Dense links stabilize for small shocks, fragilize for large ones | Acemoglu, Ozdaglar, Tahbaz-Salehi, AER; Columbia Contagion Index thesis |
| M4 | Threshold cascades on networks | Vulnerable cluster percolates through the network | Fads, norm and innovation diffusion; infrastructure failures (as motivation) | Higher degree heterogeneity can reduce vulnerability while threshold heterogeneity raises it | Watts, PNAS 2002 |

## Evidence per mechanism

### M1 Load redistribution

- Motter and Lai: in networks where load can shift, intentional attack on a key node can trigger a cascade of overload failures that collapses all or a substantial part of the network. Heterogeneous load distributions (Internet, power grids) are especially vulnerable.
- Limit: the model concerns intentional attack on one key node. Random failure behavior differs and is not established by this source alone (NeedVerify).
- Counterexample: a 2024 study of grid models reports that fine-tuning the nonlinear relation between capacity and centrality significantly improves robustness against attack (excerpt). A 2026 model with temporal network evolution reports a higher collapse threshold (12.41 percent of nodes removed) than a static scale-free baseline (7.85 percent). These numbers are model-specific and must not be generalized.

### M2 Interdependence

- Buldyrev et al.: failure of a small fraction of nodes in one network can fragment several interdependent networks completely. For two interdependent Erdos-Renyi networks the critical mean degree is 2.445, versus 1 for a single network.
- Surprising reversal: a broader degree distribution increases vulnerability of interdependent networks to random failure, the opposite of single networks.
- Real case cited by the authors: the 28 September 2003 Italian blackout, where power-station shutdowns disabled internet nodes, which disabled more stations.
- Limit: 2.445 applies to the specific model, not to real infrastructure.

### M3 Financial contagion

- Acemoglu et al.: contagion shows a phase-transition pattern. For small shocks, a more densely connected network (diversified liabilities) improves stability; beyond a critical size or number of shocks, the same connectivity propagates shocks and increases fragility.
- The complete network is least prone to contagious defaults and the ring network the most fragile, within the model and below the critical shock size.
- Empirical check (Brazilian system, doctoral thesis): in well-capitalized networks, higher connectivity helps only if initial connectivity is already high; in undercapitalized networks, more connectivity increases contagion severity.
- Limit: results depend on capital levels and shock size. 'Connectivity is good' or 'bad' alone is not supported.

### M4 Threshold cascades

- Watts: rare global cascades arise from small initial shocks when the subnetwork of vulnerable vertices percolates. Cascade sizes follow a power law in sparse regimes and are bimodal in dense ones.
- Heterogeneity is ambiguous: heterogeneous thresholds raise cascade risk, heterogeneous degree lowers it.

## Same trigger, different outcome: differentiating conditions

Only model-level contrasts are sourced so far, not real-case comparisons:

- Shock size relative to a critical threshold (M3).
- Capital or capacity buffers (M3, M1 tuning).
- Single versus interdependent coupling (M2).
- Threshold versus degree heterogeneity (M4).

Real paired cases (same trigger, one collapse, one recovery) are NOT yet collected.

## Generalization warnings

- Every threshold above belongs to its model and setting. None is a general law.
- No source here shows that all collapse is preventable, and the model makes no such claim.

## Open items before this issue can close

- [ ] Read primary papers in full.
- [ ] Add misinformation, extremism and addiction dynamics at mechanism level only.
- [ ] Add ecosystem and catastrophe-theory sources, with failure cases.
- [ ] Add real paired cases for the same-trigger comparison.
- [ ] Cross-check with #1143 and #1200 for overlap.
- [ ] Human review.
