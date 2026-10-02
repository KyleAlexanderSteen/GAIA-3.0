# CHAOS-002: Bad chaos — what turns uncertainty into collapse

Issue: #1205 (parent #1175). Related: #1200, #1143, #1099. Evidence standard: #1171.
Status: DRAFT v0.3. Research and specification only; no runtime code; cannot raise autonomy or bypass a human gate.
Scope note: mechanisms and dynamics only. No profiling of real people or groups.

## Stage reached

- Read in full (main text only): Vosoughi, Roy and Aral, Science 359 (2018), for M5. The supplementary tables were NOT read, so per-test statistics from the supplement are unchecked.
- Read as abstracts and excerpts only: everything else (M1-M4, M6, M7). Full papers were NOT read.
- v0.3 upgrades M5 and fills its counterexample slot from the paper's own footnote 22. v0.2 had added M5-M7 and a first same-trigger comparison, and removed an unsupported 2003 blackout claim from M1.
- NOT covered yet (NeedVerify, no sources collected): extremism dynamics, catastrophe theory proper, and resource-competition collapse.

## Chaos-risk model

A small disturbance grows into collapse when a propagation rule exists, the system is close enough to a threshold, and nothing absorbs the spread.

| # | Mechanism | Reported condition | Observed case | Counterexample or limit | Source |
|---|-----------|--------------------|---------------|-------------------------|--------|
| M1 | Load-redistribution cascade | Loads redistribute on failure; heterogeneous loads | Internet and power-grid models | Nonlinear capacity tuning can improve robustness | Motter and Lai, PRE 66, 065102 |
| M2 | Interdependent-network cascade | Nodes depend on nodes in another network | Italy blackout, 28 Sept 2003 | Result is model-specific | Buldyrev et al., Nature 464 (2010) |
| M3 | Financial contagion | Shock size relative to a critical threshold | Interbank network models; Brazilian system | Dense links stabilize small shocks, fragilize large ones | Acemoglu et al., AER |
| M4 | Threshold cascades | Vulnerable cluster percolates | Fads, diffusion | Degree heterogeneity can lower risk | Watts, PNAS 2002 |
| M5 | Misinformation diffusion | Novel content that people choose to share; spread driven by human behaviour, not bots | ~126,000 Twitter cascades, 2006-2017 | One platform; fact-checked stories only; novelty is correlational; an earlier smaller study (Friggeri et al.) found true rumors shared more per rumor | Vosoughi, Roy, Aral, Science 2018 |
| M6 | Addiction relapse as attractor switching | Two stable states with a barrier between them | Post-treatment substance-use data (model fit) | Descriptive model; person-level parameters are not for profiling | PMC11984488; Witkiewitz and Marlatt relapse model |
| M7 | Ecosystem regime shift with hysteresis | Alternative stable states; passing a bifurcation | Shallow-lake eutrophication | Deeper lakes respond smoothly; not every shift has a tipping point | Scheffer and Carpenter 2003; Scheffer et al. 2001 |

## Evidence for M1-M4

Unchanged from v0.1 except the blackout correction. Motter and Lai: attack on a key node can cascade through networks with heterogeneous loads (Internet and power grids are their stated examples). Buldyrev et al.: critical mean degree 2.445 for two interdependent Erdos-Renyi networks versus 1 for one network; the 2003 Italian blackout is their cited case. Acemoglu et al.: connectivity helps for small shocks and hurts past a critical size; the Brazilian-system thesis finds it depends on capital. Watts: threshold heterogeneity raises cascade risk, degree heterogeneity lowers it. All thresholds belong to their models.

## M5 Misinformation (mechanism level)

Source status: main text read in full; supplement not read.

- Data: about 126,000 rumor cascades tweeted over 4.5 million times by about 3 million people, 2006-2017. Six fact-checking organizations classified them and agreed 95 to 98 percent of the time.
- Findings: falsehood diffused significantly farther, faster, deeper and more broadly than truth in all categories, and the effect was strongest for political news. False news was 70 percent more likely to be retweeted, controlling for the original poster's account age, activity, followers, followees and verified status. Truth took about six times as long as falsehood to reach 1,500 people and about 20 times as long to reach depth 10. The top 1 percent of false cascades reached 1,000 to 100,000 people, while truth rarely exceeded 1,000.
- Against the obvious explanations: users who spread falsehood had fewer followers, followed fewer people, were less active, were verified less often and had been on Twitter for less time. The spread happened despite those differences, not because of them.
- Mechanism candidates: bots sped up true and false news at roughly the same rate, so humans were the main cause of the difference. False rumors scored as more novel than true ones on three novelty measures, and replies to false rumors showed more surprise and disgust. The authors state they cannot claim novelty causes the retweets, so novelty is a correlate, tagged NeedVerify as a cause.
- Robustness: the authors tested selection bias with a second sample of about 13,240 rumor cascades labelled by three student annotators (90 percent agreement, Fleiss' kappa 0.88) and found nearly identical results.
- Limits: Twitter only; English-language replies; stories were those that fact-checkers or the annotators investigated; the period ends in 2017. This says nothing about any group or person, only about diffusion patterns.
- Counterexample: the paper itself cites Friggeri et al. (about 4,000 Facebook rumors), which found little difference in depth and more shares per rumor for true rumors. The authors discount it for smaller sample size, an early sample that misses the rise of false news after 2013, and because shares per rumor do not equal depth, breadth or speed. This is a conflicting result from a single platform, reported second-hand; the Friggeri paper itself has NOT been read. Independent replications on other platforms are still NeedVerify.
- Policy note from the authors: containment should emphasise behavioural interventions such as labelling and incentives, not only curbing bots. This is their recommendation, not a tested result, and it is not adopted here.

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

- [x] Read the M5 primary paper (main text). Supplement still open.
- [ ] Read the primary papers for M1-M4, M6 and M7 in full.
- [ ] Read Friggeri et al. to check the M5 counterexample first-hand.
- [ ] Extremism dynamics at mechanism level (patterns only).
- [ ] Catastrophe theory proper; resource-competition collapse.
- [ ] Replications or counterexamples for M6, and non-Twitter replications for M5.
- [ ] Cross-check with #1143 and #1200.
- [ ] Human review.
