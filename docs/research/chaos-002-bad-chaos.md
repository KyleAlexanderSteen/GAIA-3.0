# CHAOS-002: Bad chaos — what turns uncertainty into collapse

Issue: #1205 (parent #1175). Related: #1200, #1143, #1099. Evidence standard: #1171.
Status: DRAFT v0.2. Research and specification only; no runtime code; cannot raise autonomy or bypass a human gate.
Scope note: mechanisms and dynamics only. No profiling of real people or groups.

## Stage reached

- Read: search-result abstracts and excerpts. Full papers were NOT read.
- v0.2 adds M5 (misinformation), M6 (addiction dynamics), M7 (ecosystem regime shifts) and a first real same-trigger comparison. It also removes an unsupported 2003 blackout claim that was in v0.1 under M1.
- NOT covered yet (NeedVerify, no sources collected): extremism dynamics, catastrophe theory proper, and resource-competition collapse.

## Chaos-risk model

A small disturbance grows into collapse when a propagation rule exists, the system is close enough to a threshold, and nothing absorbs the spread.

| # | Mechanism | Reported condition | Observed case | Counterexample or limit | Source |
|---|-----------|--------------------|---------------|-------------------------|--------|
| M1 | Load-redistribution cascade | Loads redistribute on failure; heterogeneous loads | Internet and power-grid models | Nonlinear capacity tuning can improve robustness | Motter and Lai, PRE 66, 065102 |
| M2 | Interdependent-network cascade | Nodes depend on nodes in another network | Italy blackout, 28 Sept 2003 | Result is model-specific | Buldyrev et al., Nature 464 (2010) |
| M3 | Financial contagion | Shock size relative to a critical threshold | Interbank network models; Brazilian system | Dense links stabilize small shocks, fragilize large ones | Acemoglu et al., AER |
| M4 | Threshold cascades | Vulnerable cluster percolates | Fads, diffusion | Degree heterogeneity can lower risk | Watts, PNAS 2002 |
| M5 | Misinformation diffusion | Novel, emotionally charged content spread by many users | ~126,000 Twitter cascades, 2006-2017 | One platform; stories chosen via fact-checkers; explanation is a hypothesis | Vosoughi, Roy, Aral, Science 2018 |
| M6 | Addiction relapse as attractor switching | Two stable states with a barrier between them | Post-treatment substance-use data (model fit) | Descriptive model; person-level parameters are not for profiling | PMC11984488; Witkiewitz and Marlatt relapse model |
| M7 | Ecosystem regime shift with hysteresis | Alternative stable states; passing a bifurcation | Shallow-lake eutrophication | Deeper lakes respond smoothly; not every shift has a tipping point | Scheffer and Carpenter 2003; Scheffer et al. 2001 |

## Evidence for M1-M4

Unchanged from v0.1 except the blackout correction. Motter and Lai: attack on a key node can cascade through networks with heterogeneous loads (Internet and power grids are their stated examples). Buldyrev et al.: critical mean degree 2.445 for two interdependent Erdos-Renyi networks versus 1 for one network; the 2003 Italian blackout is their cited case. Acemoglu et al.: connectivity helps for small shocks and hurts past a critical size; the Brazilian-system thesis finds it depends on capital. Watts: threshold heterogeneity raises cascade risk, degree heterogeneity lowers it. All thresholds belong to their models.

## M5 Misinformation (mechanism level)

- Data: about 126,000 rumor cascades tweeted over 4.5 million times by about 3 million people, 2006-2017, classified by six fact-checking organizations.
- Findings: falsehood diffused significantly farther, faster, deeper and more broadly than truth in all categories. False news was 70 percent more likely to be retweeted; truth took about six times as long to reach 1,500 people; false cascades reached depth 10 about 20 times faster. The top 1 percent of false cascades reached 1,000 to 100,000 people, while truth rarely exceeded 1,000.
- Mechanism candidates: removing bots left the differences intact, so humans were the main spreaders. False tweets were more novel than true ones; novelty and emotional reaction are the authors' proposed explanation, not a tested cause.
- Limits: Twitter only; stories were those that fact-checkers investigated, so selection effects are possible; the period ends in 2017. This says nothing about any group or person, only about diffusion patterns.
- Counterexample: not found yet. NeedVerify (look for replications and non-Twitter platforms).

## M6 Addiction dynamics (mechanism level)

- Double-well potential model: post-treatment substance use is described as a dynamical system with two stable equilibria, abstinence and relapse, plus a dominant-equilibrium 'tilt', a 'steepness' (ease of changing wells) and an overall relapse risk. A larger separation between wells means a larger disturbance is needed to switch.
- Relapse-prevention literature: the dynamic model treats relapse as a complex nonlinear process in which many factors act jointly. Stable background factors set a threshold (who is vulnerable); transient factors decide when relapse occurs. Relapse can be sudden and unexpected.
- Limits: this is a model fitted to data, not a measured mechanism. Individual-level parameters exist in the source; this issue does not use them and does not profile individuals. Generalization beyond substance-use treatment data is not supported.
- Counterexample: not found yet. NeedVerify.

## M7 Ecosystems (regime shifts)

- Theory: Scheffer and Carpenter describe alternative equilibria. When a bifurcation is passed, a catastrophic transition occurs, and returning requires going back beyond a different bifurcation point; this difference is hysteresis. Lakes recovering from acidification or eutrophication are cited examples.
- Warning signals: critical slowing down (rising variance and autocorrelation) can announce a nearby tipping point, but indicators may fail to announce a true transition, noise-induced transitions are unlikely to be announced, and in most cases indicators are detected only in retrospect or while the transition unfolds. Slowing down also precedes non-catastrophic transitions, so it is not specific to collapse. These caveats feed #1200.
- Field case: a 64-year record from a subtropical Chinese lake shows a critical transition and hysteresis under combined warming, eutrophication and fish stocking. Early-warning signals were detectable in both collapse and recovery, and full recovery had not yet been observed.

## Same trigger, different outcome

| Trigger | Collapse or persistence | Recovery or smooth response | Differentiating condition | Source |
|---------|-------------------------|-----------------------------|---------------------------|--------|
| Nutrient loading | Shallow lakes show pronounced hysteresis | Deeper lakes respond smoothly | Lake depth | Scheffer et al. 2001 (excerpt) |
| Phosphorus abatement in Lake Erie | Re-eutrophication in the 2000s-2010s driven by agricultural, non-point inputs | Fish communities rehabilitated rapidly after the 1972 abatement programme | Which nutrient sources were controlled; reversal not permanent | Ohio State fish study; Ecology and Society 2023 |

Only model-level contrasts exist for M1-M4 (shock size, capital buffers, single vs interdependent coupling).

## Generalization warnings

- Every threshold belongs to its model and setting. None is a general law.
- No source shows that all collapse is preventable, and none is claimed here.

## Open items

- [ ] Read primary papers in full.
- [ ] Extremism dynamics at mechanism level (patterns only).
- [ ] Catastrophe theory proper; resource-competition collapse.
- [ ] Replications or counterexamples for M5 and M6.
- [ ] Cross-check with #1143 and #1200.
- [ ] Human review.
