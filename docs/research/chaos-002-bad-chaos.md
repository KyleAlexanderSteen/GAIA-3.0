# CHAOS-002: Bad chaos — what turns uncertainty into collapse

Issue: #1205 (parent #1175). Related: #1200, #1143, #1099. Evidence standard: #1171.
Status: DRAFT v0.4. Research and specification only; no runtime code; cannot raise autonomy or bypass a human gate.
Scope note: mechanisms and dynamics only. No profiling of real people or groups.

## Stage reached

- Read in full (main text only): Vosoughi, Roy and Aral, Science 359 (2018), for M5. The supplementary tables were NOT read.
- Read in full (arXiv preprint v1, 2009, NOT the Nature 2010 version): Buldyrev, Parshani, Paul, Stanley and Havlin, for M2. See the M2 section for what that does and does not confirm.
- Read as abstracts, excerpts or secondary summaries only: M1, M3, M4, M6, M7, M8, M9, M10. Full papers were NOT read. The Scheffer and Carpenter 2003 page was blocked (bot check), so M7 is still excerpt-level.
- v0.4 adds M8 (extremism), M9 (catastrophe theory) and M10 (resource-complexity collapse), and upgrades M2. v0.3 upgraded M5; v0.2 added M5-M7 and removed an unsupported 2003 blackout claim from M1.
- NOT covered: resource competition in ecology or economics beyond Tainter's single-theory view; creativity and innovation science (see #1357).

## Chaos-risk model

A small disturbance grows into collapse when a propagation rule exists, the system is close enough to a threshold, and nothing absorbs the spread.

| # | Mechanism | Reported condition | Observed case | Counterexample or limit | Source |
|---|-----------|--------------------|---------------|-------------------------|--------|
| M1 | Load-redistribution cascade | Loads redistribute on failure; heterogeneous loads | Internet and power-grid models | Nonlinear capacity tuning can improve robustness | Motter and Lai, PRE 66, 065102 |
| M2 | Interdependent-network cascade | Nodes depend on nodes in another network | Simulation and theory on random networks; power-grid and communication networks as the motivating example | Result is model-specific; assumes random, independent coupling and one-to-one dependency | Buldyrev et al., arXiv 0907.1182 (preprint) |
| M3 | Financial contagion | Shock size relative to a critical threshold | Interbank network models; Brazilian system | Dense links stabilize small shocks, fragilize large ones | Acemoglu et al., AER |
| M4 | Threshold cascades | Vulnerable cluster percolates | Fads, diffusion | Degree heterogeneity can lower risk | Watts, PNAS 2002 |
| M5 | Misinformation diffusion | Novel content that people choose to share; spread driven by human behaviour, not bots | ~126,000 Twitter cascades, 2006-2017 | One platform; fact-checked stories only; novelty is correlational; Friggeri et al. found true rumors shared more per rumor | Vosoughi, Roy, Aral, Science 2018 |
| M6 | Addiction relapse as attractor switching | Two stable states with a barrier between them | Post-treatment substance-use data (model fit) | Descriptive model; person-level parameters are not for profiling | PMC11984488; Witkiewitz and Marlatt |
| M7 | Ecosystem regime shift with hysteresis | Alternative stable states; passing a bifurcation | Shallow-lake eutrophication | Deeper lakes respond smoothly; not every shift has a tipping point | Scheffer and Carpenter 2003; Scheffer et al. 2001 |
| M8 | Opinion polarization with extremists | Bounded confidence: agents only update toward opinions within an uncertainty range; extremists with low uncertainty | Agent-based simulations only; no real-population case collected | Fixed large uncertainties give indefinitely fluctuating opinions instead of clusters; outcomes depend on the uncertainty-update rule | Deffuant et al.; JASSS 19(1) 6 (2016) |
| M9 | Cusp catastrophe (sudden jump under smooth change) | Two control variables; hysteresis and bimodality | Mechanical Zeeman machine; fold and cusp geometry in physics and engineering | Social and biological applications were criticized as incorrect reasoning and exaggerated claims; fitting is hard | Zeeman 1972-77; Zahler and Sussmann 1977 (cited second-hand) |
| M10 | Declining returns on complexity | Cost of added complexity rises faster than benefit; no stronger neighbour to fill a vacuum | Roman, Maya and Chacoan societies (author's selection) | One theory among several the author lists, including a 'house of cards' fragility model; case selection by the author | Tainter, The Collapse of Complex Societies (excerpts, secondary summaries) |

## Evidence for M1, M3, M4

Unchanged from v0.1 except the blackout correction. Motter and Lai: attack on a key node can cascade through networks with heterogeneous loads (Internet and power grids are their stated examples). Acemoglu et al.: connectivity helps for small shocks and hurts past a critical size; the Brazilian-system thesis finds it depends on capital. Watts: threshold heterogeneity raises cascade risk, degree heterogeneity lowers it. All thresholds belong to their models. Excerpt level only.

## M2 Interdependent networks (primary text read)

Source status: arXiv preprint v1 (7 Jul 2009) read in full. The published Nature 2010 version was NOT read, so any difference between versions is unchecked.

- Model: two networks A and B of N nodes each; each node depends on one node in the other network. Removing a fraction 1-p of A also removes the matching nodes in B. Only nodes in a mutually connected giant component stay functional. Failures alternate between networks in stages until nothing more fragments.
- Result for two Erdos-Renyi networks: the abstract gives a critical mean degree of 2.445 versus 1 for a single network. The body gives pc = 2.4554/a for equal mean degree a, and a critical mutual giant-component fraction of 1.2564/a. The abstract and body values differ slightly (2.445 vs 2.4554) and the preprint does not explain why. Treat 2.4554 as the body-derived number and flag the discrepancy.
- Scale-free networks: a single scale-free network has pc approaching 0 for exponent 3 or below, but under mutual percolation pc stays above 0 for exponents above 2.
- Counterintuitive result: for a single network a broader degree distribution increases robustness to random failure; for interdependent networks the broader distribution makes them more vulnerable. The stated reason is that hubs in one network can map to low-degree nodes in the other, and low-degree nodes disconnect easily.
- Finite size: near the threshold the number of cascade stages grows with network size; at criticality the authors expect it to scale as N to the power 1/4, supported by their simulations. Simulations with N up to 128,000 match the theory.
- Limits: random independent coupling and one-to-one dependency are assumed. The authors say the model extends to other couplings but do not test them here. The preprint motivates with power stations and communication networks but does NOT mention the Italian 2003 blackout, which confirms that claim was not supported by this source.
- Counterexample: not found in the preprint. M1's finding that capacity tuning can improve robustness is a different model. NeedVerify for real-network data.

## M5 Misinformation (mechanism level)

Source status: main text read in full; supplement not read.

- Data: about 126,000 rumor cascades tweeted over 4.5 million times by about 3 million people, 2006-2017. Six fact-checking organizations classified them and agreed 95 to 98 percent of the time.
- Findings: falsehood diffused significantly farther, faster, deeper and more broadly than truth in all categories, and the effect was strongest for political news. False news was 70 percent more likely to be retweeted, controlling for the original poster's account age, activity, followers, followees and verified status. Truth took about six times as long as falsehood to reach 1,500 people and about 20 times as long to reach depth 10. The top 1 percent of false cascades reached 1,000 to 100,000 people, while truth rarely exceeded 1,000.
- Against the obvious explanations: users who spread falsehood had fewer followers, followed fewer people, were less active, were verified less often and had been on Twitter for less time.
- Mechanism candidates: bots sped up true and false news at roughly the same rate, so humans were the main cause of the difference. False rumors scored as more novel on three novelty measures. The authors state they cannot claim novelty causes the retweets, so novelty is a correlate, tagged NeedVerify as a cause.
- Robustness: a second sample of about 13,240 rumor cascades labelled by three student annotators (90 percent agreement, Fleiss' kappa 0.88) gave nearly identical results.
- Limits: Twitter only; English-language replies; stories investigated by fact-checkers or annotators; period ends in 2017. This says nothing about any group or person.
- Counterexample: the paper cites Friggeri et al. (about 4,000 Facebook rumors), which found little difference in depth and more shares per rumor for true rumors. The authors discount it for smaller sample, an early sample, and because shares per rumor do not equal depth, breadth or speed. Reported second-hand; the Friggeri paper has NOT been read.
- Policy note from the authors: behavioural interventions such as labelling and incentives, not only curbing bots. Their recommendation, not a tested result, and not adopted here.

## M6 Addiction dynamics (mechanism level)

- Double-well potential model: post-treatment substance use is described as a dynamical system with two stable equilibria, abstinence and relapse, plus a dominant-equilibrium 'tilt', a 'steepness' and an overall relapse risk. A larger separation between wells means a larger disturbance is needed to switch.
- Relapse-prevention literature: stable background factors set a threshold; transient factors decide when relapse occurs. Relapse can be sudden and unexpected.
- Limits: a model fitted to data, not a measured mechanism. Individual-level parameters exist in the source; this issue does not use them and does not profile individuals.
- Counterexample: not found yet. NeedVerify.

## M7 Ecosystems (regime shifts)

- Theory: Scheffer and Carpenter describe alternative equilibria. Passing a bifurcation causes a catastrophic transition, and returning requires going back beyond a different bifurcation point; this difference is hysteresis.
- Warning signals: critical slowing down (rising variance and autocorrelation) can announce a nearby tipping point, but indicators may fail to announce a true transition, noise-induced transitions are unlikely to be announced, and in most cases indicators are detected only in retrospect. Slowing down also precedes non-catastrophic transitions. These caveats feed #1200.
- Field case: a 64-year record from a subtropical Chinese lake shows a critical transition and hysteresis under combined warming, eutrophication and fish stocking. Full recovery had not yet been observed.
- Source status: excerpt level. A full-text fetch of the 2003 paper was blocked.

## M8 Extremism and opinion dynamics (mechanism level, models only)

Source status: abstract and search excerpts only. No full paper read. No real-world dataset collected. This section describes how simulated agents behave, not how any real group behaves.

- Bounded confidence: agents talk to peers whose opinions fall within an uncertainty range and ignore the rest. A narrow range means little influence from distant views; a wide range means listening to far-off views (JASSS 17(1) 13 excerpt).
- Attractors: models with moderate and extremist agents show three attractor types, central clusters, double-extreme clusters and single-extreme clusters, when moderates' uncertainty falls after meeting extremists (JASSS 19(1) 6 abstract).
- Same setup, different outcome: when uncertainties are fixed and the moderates' uncertainty is large, a stationary state appears in which moderate opinions keep fluctuating without clustering (same abstract). So the extremist-takeover result depends on the update rule.
- Network structure: a 2026 simulation study reports that rewiring edges in a bounded-confidence model changes how opinions cluster (Frontiers in Physics, excerpt only; its main result was not read).
- Limits: all results are agent-based simulations with chosen parameters. No empirical test collected here. The model labels agents by an opinion number, so it must not be used to label real people or groups.
- Counterexample: the fixed-uncertainty fluctuating state above. Real-world validation: NeedVerify.

## M9 Catastrophe theory proper

Source status: Wikipedia overview, a Zeeman book review (Project Euclid) and a student thesis, all secondary. Zeeman's own papers were NOT read.

- Idea: catastrophe theory is a branch of bifurcation theory. In the cusp catastrophe, smooth change in two control variables can cause a sudden jump, with hysteresis. The Zeeman catastrophe machine shows this: smooth movement of a spring's end causes sudden changes in a wheel's position.
- Where it holds: fold bifurcations and cusp geometry recur in physics, engineering and mathematical modelling. A review of Zeeman's collected papers describes the cusp as the most important example.
- Where it was criticized: from the late 1970s, applications in biology and the social sciences were attacked. Zahler and Sussmann (Nature, 1977) called them characterised by incorrect reasoning, far-fetched assumptions, erroneous consequences and exaggerated claims. The book review notes no justification for the stock exchange model's hypotheses from existing data or theory.
- Why fitting is hard: a thesis summarizes the objections as a deterministic theory misused in a stochastic setting, poorly operationalized ad-hoc control variables and weak quantitative method. Cusp regression on cross-sectional data cannot say anything about time, and stochastic versions remain hard to fit.
- Use in this issue: as a vocabulary for sudden jumps and hysteresis (links to M6 and M7), not as a predictive model for social collapse. Any social use needs data that this issue has not collected.
- Counterexample: the critique above is itself the counterexample. NeedVerify whether recent stochastic-cusp applications are validated.

## M10 Declining returns on complexity (resource-cost collapse)

Source status: book excerpt and secondary summaries only. The book itself was NOT read.

- Claim: Tainter proposes that returns on investment in sociopolitical complexity follow a curve. Past some point, more complexity still adds benefit but at a declining marginal rate, and the society becomes more vulnerable to collapse (book excerpt).
- Conditions: peers competing with each other must keep investing in complexity even when returns are poor, and collapse can only occur where no stronger competitor can fill the vacuum (same excerpt).
- Secondary summaries restate four premises: societies are problem-solving organizations, they need energy, complexity raises per-capita cost, and investment often reaches negative marginal returns.
- Limits: cases were selected by the author (Roman, Maya, Chacoan). 'Complexity' is defined loosely across agriculture, fuel extraction, research, education and politics (a reviewer's remark). One reviewer notes the book itself lists other models, including a 'house of cards' view that complex societies run on thin reserves.
- Counterexample or alternative: the competing models listed in the book. No test against independent cases collected. NeedVerify.

## Same trigger, different outcome

| Trigger | Collapse or persistence | Recovery or smooth response | Differentiating condition | Source |
|---------|-------------------------|-----------------------------|---------------------------|--------|
| Nutrient loading | Shallow lakes show pronounced hysteresis | Deeper lakes respond smoothly | Lake depth | Scheffer et al. 2001 (excerpt) |
| Phosphorus abatement in Lake Erie | Re-eutrophication in the 2000s-2010s driven by agricultural, non-point inputs | Fish communities rehabilitated rapidly after the 1972 abatement programme | Which nutrient sources were controlled; reversal not permanent | Ohio State fish study; Ecology and Society 2023 |
| Extremist agents in a bounded-confidence model | Clusters pulled to the extremes | Moderates keep fluctuating without clustering | Whether moderates' uncertainty shrinks after meeting extremists | JASSS 19(1) 6 (abstract) |
| Random node failure in coupled networks | Broad degree distribution: more vulnerable | Narrow distribution: less vulnerable | Interdependence between networks (reverses the single-network result) | Buldyrev et al. preprint |

## Generalization warnings

- Every threshold belongs to its model and setting. None is a general law.
- No source shows that all collapse is preventable, and none is claimed here.
- M8 and M9 are model-level; neither is evidence about real people or groups.

## Open items

- [x] Read the M5 primary paper (main text). Supplement still open.
- [x] Read the M2 primary text (arXiv preprint). Nature 2010 version still open.
- [ ] Read the primary papers for M1, M3, M4, M6 and M7 in full (Scheffer and Carpenter fetch blocked; try another host).
- [ ] Read Friggeri et al. to check the M5 counterexample first-hand.
- [ ] Read Deffuant et al. on extremism; find an empirical test of bounded-confidence predictions.
- [ ] Read Zeeman and Zahler-Sussmann directly; check recent stochastic-cusp validation.
- [ ] Read Tainter's book chapter; find tests against independent cases.
- [ ] Replications or counterexamples for M6, and non-Twitter replications for M5.
- [ ] Cross-check with #1143 and #1200; coordinate with #1357 on innovation.
- [ ] Human review.
