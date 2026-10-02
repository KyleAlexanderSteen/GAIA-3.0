# CHAOS-001: Good chaos — when unpredictability helps

Issue: #1204 (parent #1175). Evidence standard: #1171.
Status: DRAFT v0.1. Research and specification only; no runtime code; cannot raise autonomy or bypass a human gate.
Scope note: this covers system-level variation. Nothing here implies that variation benefits human suffering or harm.

## Stage reached

- Read: search-result abstracts and excerpts for the sources below. Full papers were NOT read.
- Therefore every quantitative claim is tagged by what the excerpt supports, and anything beyond it is NeedVerify.
- NOT covered yet: creativity research, innovation science, adaptive experimentation, antifragility (all NeedVerify, no sources collected).

## Taxonomy of beneficial chaos

Each taxon states where the benefit appears and where it disappears.

| # | Taxon | Benefit appears when | Benefit disappears when | Source |
|---|-------|----------------------|-------------------------|--------|
| T1 | Noise-assisted detection (stochastic resonance) | Bistable or threshold system with a weak periodic signal; noise at an intermediate intensity | Noise too low or too high; monostable systems do not show it | Bistable-system simulation study, IEEE 2025 (excerpt); sensory review, Clin. Neurophysiol. (excerpt) |
| T2 | Random uphill moves in optimization (simulated annealing) | Temperature falls on a suitable schedule | Cooling too fast loses the guarantee; the guaranteed schedule is too slow to use in practice | Hajek, Cooling Schedules for Optimal Annealing; Liang lecture notes (excerpt) |
| T3 | Deliberate exploration (bandits) | Exploration is budgeted so regret grows only logarithmically in time T | Exploration costs are high or the horizon is short; results are per-setting | AdaUCB paper, arXiv 1709.04004 (excerpt); CExp2, PMLR v237 (excerpt) |
| T4 | Mutation / variation in evolution | Mutation supplies variants while selection removes harmful ones | Mutation rate exceeds the error threshold; mutational meltdown | Error catastrophe (Eigen); Lynch-based review, PMC7993354 (excerpt) |

## Evidence per taxon

### T1 Stochastic resonance

- Mechanism: in bistable systems, an optimal noise intensity maximizes output signal-to-noise ratio. In a simulation study, each of Gaussian white, pink and impulse noise had its own optimal intensity, and bistability was essential for the effect.
- Boundary (too much noise): the same study shows SNR is maximized at an intermediate intensity, so other intensities perform worse. Exact curves are simulation results, not field data.
- Related: a 2014 Physical Review E paper on forbidden-interval theorems for threshold detectors indicates that some noise-benefit conditions are provably excluded (title only; NeedVerify).
- Applied claims (neural prosthetics, bearing-fault detection) are in the sources' own abstracts; clinical effectiveness is NeedVerify.

### T2 Simulated annealing

- Hajek's necessary and sufficient condition: with cooling schedule T_t = c / log(1 + t), convergence in probability to global minima holds iff c is at least the depth of the deepest non-global local minimum.
- Boundary: logarithmic cooling is described as so slow that nobody can afford the running time; faster schedules (for example square-root) need a different algorithm (stochastic approximation annealing) for guarantees.
- Lesson: random moves help only while the level of randomness is controlled and reduced. The 'optimal' schedule is problem-specific (depth of the landscape).

### T3 Exploration vs exploitation

- Result: AdaUCB achieves O(log T) regret, and O(1) regret if exploration cost is zero below a load threshold (opportunistic bandits). CExp2 achieves order-optimal O(c* log T) regret in collaborative bandits.
- Conditions: these bounds hold for the stated bandit variants (stochastic rewards, specific load and communication models). They are NOT a universal optimal exploration rate.
- No single 'optimal randomness percentage' is given by these sources, and none is claimed here.

### T4 Mutation and error threshold

- Eigen's error threshold: a genome survives unchanged only if L*q < -log(S) (Wikipedia summary of the model; primary paper NeedVerify).
- Boundary: mutational meltdown. Excerpt: deleterious mutation rates on the order of 1 per individual per generation mark the transition between drift-driven and mutation-driven loss, and high rates can overwhelm selection even in large populations.
- Lesson: variation is a resource only below a threshold set by selection strength.

## Is 'breakthroughs emerge from disorder' supported?

Partially, and only with a qualifier. In every supported case above, randomness is paired with a selection or control mechanism: a signal and a threshold (T1), a cooling schedule and acceptance rule (T2), a reward signal (T3), natural selection (T4). The sources support 'bounded randomness plus selection helps'. They do not support 'disorder alone produces breakthroughs'. Claims about human creativity and innovation remain NeedVerify (not researched).

## Cases where more randomness hurt

- Too much noise past the stochastic-resonance optimum (T1).
- Too-fast cooling or no cooling control in annealing (T2).
- Mutation rate above the error threshold; extinction by meltdown (T4).
- Costly exploration in finite horizons (T3), per the opportunistic-bandit setting.

## Open items before this issue can close

- [ ] Read primary papers in full (Hajek 1988; Kirkpatrick et al. 1983; Eigen 1971; Lynch et al.; a stochastic-resonance review).
- [ ] Add creativity, innovation science and adaptive experimentation sources.
- [ ] Add antifragility with a critical source (NeedVerify).
- [ ] Add a second independent source per taxon.
- [ ] Human review of the wording of the 'disorder' conclusion.
