# CHAOS-001: Good chaos — when unpredictability helps

Issue: #1204 (parent #1175). Evidence standard: #1171.
Status: DRAFT v0.2. Research and specification only; no runtime code; cannot raise autonomy or bypass a human gate.
Scope note: this covers system-level variation. Nothing here implies that variation benefits human suffering or harm.

## Stage reached

- Read: search-result abstracts and excerpts. Full papers were NOT read.
- v0.2 adds T5 (creativity) and T6 (antifragility). Both are weaker than T1-T4; see their evidence grades.
- NOT covered yet: innovation science and adaptive experimentation (NeedVerify, no sources collected).

## Taxonomy of beneficial chaos

| # | Taxon | Benefit appears when | Benefit disappears when | Evidence grade | Source |
|---|-------|----------------------|-------------------------|----------------|--------|
| T1 | Noise-assisted detection (stochastic resonance) | Bistable or threshold system, weak periodic signal, intermediate noise | Noise too low or too high; monostable systems | Simulation + applied | Bistable-system SR study, IEEE 2025 (excerpt) |
| T2 | Random uphill moves (simulated annealing) | Cooling schedule controlled | Cooling too fast loses the guarantee; guaranteed schedule too slow to use | Theorem | Hajek, Cooling Schedules for Optimal Annealing |
| T3 | Deliberate exploration (bandits) | Exploration budgeted; regret grows only logarithmically | Costly exploration, short horizon | Theorem, per setting | AdaUCB arXiv 1709.04004; CExp2 PMLR v237 |
| T4 | Mutation / variation | Selection removes harmful variants | Rate above error threshold; meltdown | Theory + review | Eigen error catastrophe; PMC7993354 |
| T5 | Environmental noise and creativity | Unclear. Possibly moderate noise for some individuals | Unpredictable, intelligible noise impaired performance in one study | WEAK, mixed | SCIRP 2021 study (excerpt) |
| T6 | Antifragility (gain from volatility) | Payoff is convex in dispersion; system can adapt ('tinker') | Highly dynamic conditions; no adaptation mechanism | Mixed: definition formal, empirical support thin | Taleb et al. arXiv 1208.1189; arXiv 2405.11397; Syst. Res. 2020 |

## Evidence per taxon

### T1-T4 (unchanged from v0.1)

- T1: an intermediate noise intensity maximizes output SNR in bistable systems; Gaussian, pink and impulse noise each have their own optimum; bistability is essential. Simulation results, not field data.
- T2: with T_t = c / log(1 + t), convergence in probability to global minima holds iff c is at least the depth of the deepest non-global local minimum. Logarithmic cooling is described as too slow for practice.
- T3: AdaUCB gets O(log T) regret (O(1) with zero exploration cost in the opportunistic setting). CExp2 gets order-optimal O(c* log T) in collaborative bandits. These are not a universal exploration rate.
- T4: Eigen error threshold: L*q < -log(S) (summary; primary NeedVerify). High deleterious-mutation rates can overwhelm selection even in large populations.

### T5 Creativity and noise (WEAK evidence)

- The one source collected is a 2021 study of young people, and its own excerpt is mixed. It reports that creative performance was impaired by noise, especially unpredictable and intelligible noise, but also that no significant effects on divergent-creativity tasks were found in any group compared with no noise.
- The same source notes some evidence that for highly original individuals a moderate noise level may raise creative performance (a claim about prior work; NeedVerify).
- Boundary: this is environmental noise acting on people. It is NOT evidence that random variation in idea generation produces creative output. The latter is untested here.
- No 'optimal' noise level is given by any source, so none is stated.
- Verdict: creativity remains a hypothesis, not a supported taxon. It is listed so the gap is visible. The taxon must not be used to justify injecting randomness into human work.

### T6 Antifragility (mixed; mostly conceptual)

- Formal definition: Taleb and Douady define fragility and antifragility as negative or positive sensitivity to a semi-measure of dispersion and volatility (a variant of vega), linked to nonlinear effects, with model error integrated.
- Machine-learning extension: arXiv 2405.11397 defines antifragility in online decision making as dynamic regret's strictly concave response to environmental variability, and argues that approaches focused on resisting shifts are limited.
- Boundary, simulation critique: a system-dynamics study of supply chains concluded that antifragility can lose its unambiguous advantage in highly dynamic situations (Systems Research, 2020).
- Biology: an MDPI 2011 paper links antifragility to 'tinkering', i.e. creative response to change, and uses evolution as the example (conceptual).
- Software: a 2014 arXiv paper on antifragile software reaches a hypothesis (antifragile development processes may produce antifragile systems), not a measured result.
- Secondary critique (low grade, blog review): antifragile ignores research on posttraumatic growth and the role of social support. This is a book review, not a primary source. NeedVerify against primary resilience literature.
- Verdict: antifragility has a clear formal definition (positive response to dispersion) but little tested evidence in the sources collected. It does not license 'chaos is good'; it names a property that some systems have and others lack.

## Is 'breakthroughs emerge from disorder' supported?

Partially. In supported cases (T1-T4) randomness is paired with a selection or control mechanism. T5 gives no support. T6 gives a definition of when variation pays (convex response), not evidence that it usually does. Disorder as a minor input beside selection and structure is the better-supported reading.

## Cases where more randomness hurt

- Past the SR optimum (T1); too-fast cooling (T2); mutation above threshold (T4); costly exploration in finite horizons (T3); unpredictable noise impaired creative performance in one study (T5); antifragility advantage lost in highly dynamic supply-chain simulations (T6).

## Open items

- [ ] Read primary papers in full (Hajek 1988; Kirkpatrick 1983; Eigen 1971; Lynch; Taleb and Douady).
- [ ] Add a primary creativity study of variation in idea generation (not only environmental noise).
- [ ] Add innovation science and adaptive experimentation.
- [ ] Add a primary antifragility critique and an empirical test.
- [ ] Second independent source per taxon.
- [ ] Human review of the 'disorder' conclusion.


## Registered failure

A good-chaos claim is not eligible until a failure condition is named before the run. For this draft the condition is: variation past the cited optimum, or noise that impairs the measured task, counts as hurt, not help. No outside witness has run that condition here. This does not close #1204.
