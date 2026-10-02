# CHAOS-003: Early warning indicators of destructive chaos

Issue: #1200 (parent #1175). Related, not duplicated: #1143, #1146, #1127. Evidence standard: #1171.
Status: DRAFT v0.5. Research and specification only; no runtime code; cannot raise autonomy or bypass a human gate.
Scope note: this catalog recommends NO indicator for operational use. Any use needs its own issue, calibration (#1146) and the existing human gates.

## Stage reached

- Read in full text: Jager and Fullsack, PLoS ONE 14(2) e0211072 (2019), PMC6364907; Boettiger and Hastings, 'Early Warning Signals and the Prosecutor's Fallacy' (arXiv 1210.1204, 2012).
- Read in part: Dakos et al., PLoS ONE 7(7) e41010 (2012), PMC3398887. The fetch returned the abstract, introduction, full methods and the start of the datasets section; it was cut off before the results and limitations sections. A second attempt via the PLoS printable-PDF link returned no content. Statements about that paper below are limited to what was returned.
- Fetch failed, not read: Carpenter et al. Science 2011 (PDF fetch returned no content); Diks et al. finance paper (HAL host showed a bot check); power-system paper (ScienceDirect bot check).
- v0.5 climate rows (W14-W21) and the conflict section come from search-result abstracts and excerpts only; no full text was read for them.
- Everything else in the catalog is from search-result abstracts and excerpts.
- Domains with sources collected: ecosystems (lake field experiment), generic statistics and false positives, epidemics, deep-learning detection, agent-based social simulation, finance (two different indicator families), power systems (abstract only), global temperature (as a false-positive test case), climate tipping elements (AMOC, Greenland, paleoclimate, Antarctic glacier model; abstract level), conflict warning (policy-level commentary only).
- NOT covered, no sources collected (NeedVerify): quantitative validation of any early-warning indicator for conflict escalation; empirical flickering cases; Amazon rainforest beyond a one-line mention in a review chapter.
- Lead times below are what the abstracts state; exact figures were not extracted.
- Author names are omitted where the search output did not show them, except for papers read in full or where the source text named the author.

## Evidence labels

- PROSPECTIVE-EXPERIMENT: a transition was deliberately induced and monitored against a reference system. A prospective test of the indicator, but NOT a forecast of an unforced natural event.
- RETROSPECTIVE: indicator fitted or checked on data after the transition is known.
- OBSERVATIONAL: indicator computed on current observations of a system that has not yet transitioned; cannot be scored as right or wrong until the future is known.
- MODEL/SIM: simulation or theory only.
- ABSTRACT-ONLY: described in an abstract; method, data and error rates not checked.
- FULL-TEXT: the paper body was read (see Stage reached for which ones).

## Warning-signal catalog

| # | Indicator | System | Evidence label | Reported lead time | Known failure cases | Source |
|---|-----------|--------|----------------|--------------------|---------------------|--------|
| W1 | Rising variance and lag-1 autocorrelation (critical slowing down) | Whole-lake food web, manipulated vs reference lake | PROSPECTIVE-EXPERIMENT (abstract-level) | Signals evident more than a year before the transition completed | Experimentally forced, not a natural event; one lake pair | Carpenter et al., Science 332 (2011) |
| W2 | Zooplankton variance and autocorrelation | Same lake experiment | PROSPECTIVE-EXPERIMENT | Elevated in year 3 of 4 | Signal returned to reference-lake levels in year 4 as the transition completed | Limnology and Oceanography 58 (2013) |
| W3 | Conditional heteroskedasticity | Same lake experiment | PROSPECTIVE-EXPERIMENT | At least a year before the shift | Single experiment | Ecosystems (2012) |
| W4 | Spatial variance, low-frequency spatial variance | Same lake experiment, prey-fish catch | PROSPECTIVE-EXPERIMENT | Reported as early warning | Single experiment | Ecosphere (2014) |
| W5 | Variance, autocorrelation, skewness, kurtosis | COVID-19 case curves, multiple countries | RETROSPECTIVE | Authors suggest 2-3 weeks | Confounded by interventions and reporting; abstract says increases predict onset for 'most' datasets, so some did not | Frontiers in Public Health (2020) |
| W6 | Second factorial moment, decay time of reported cases | Birth-death-immigration case-report models | MODEL/SIM | n/a | Estimates from a single series have high variance; averaging over an ensemble helps | PMC8455094 |
| W7 | Deep-learning classifier trained on simulated normal forms | Real-world systems incl. paleoclimate and thermoacoustic data | Mixed; see source | Not extracted | Claimed more sensitive with fewer false positives than generic indicators; independent replication NOT checked | PNAS (2021), doi 10.1073/pnas.2106140118 |
| W8 | LSTM on EWS of cooperation collapse | Agent-based public-good game | MODEL/SIM | n/a | Simulation only; no real population | Elsevier (2020) |
| W9 | Critical slowing down statistics | Financial markets: Black Monday 1987, and later crises | RETROSPECTIVE (abstract-level) | Not extracted | Statistical evidence before Black Monday 1987, but results mixed or insignificant for the more recent crises; inconsistent across events | Empirical Economics (2018), 'Critical slowing down as an early warning signal for financial crises?' |
| W10 | Slower recovery from perturbations, rising variance, rising autocorrelation | Power systems approaching critical transitions | ABSTRACT-ONLY | Not extracted | Error rates, thresholds and real-grid validation not extracted | Electric Power Systems Research, 'Early warning signals for critical transitions in power systems' (2015) |
| W11 | Credit and debt-based indicators (debt-service ratios, credit-to-GDP gaps, household and cross-border debt) | Banking distress, cross-country | RETROSPECTIVE | Alert counts as correct if issued at least once in the 12 quarters before a crisis | A different indicator family from critical slowing down; the criterion rewards any alert in a long window, so it can hide false alarms; exact false-alarm rates not extracted | BIS Quarterly Review (March 2018) |
| W12 | Standard deviation and AC(1) on raw (undetrended) data | Global surface temperature change (GISTEMP) | FULL-TEXT, real data, FALSE POSITIVE | n/a | Both rose on raw data, resembling a quadratic-growth system; both increases vanished after moving-average detrending, so the rise was not a valid warning | Jager and Fullsack, PLoS ONE 14(2) e0211072 (2019) |
| W13 | Variance and autocorrelation (Kendall's tau of rolling-window trend) | Simulated stable birth-death (Allee) and predator-prey systems, selected because they collapsed by chance | FULL-TEXT, MODEL/SIM, FALSE POSITIVE | n/a | Chance collapses showed higher tau than non-collapsed replicates, so a historical 'successful detection' can be a false positive | Boettiger and Hastings, arXiv 1210.1204 (2012) |
| W14 | Critical slowing down indicator (variance, autocorrelation, restoring rate) | AMOC, eight observation-based indices from sea-surface temperature and salinity | OBSERVATIONAL, ABSTRACT-ONLY | None; no transition has occurred | Abstract says estimates of the critical transition point remain uncertain; the 2025 AMOC paper (W15) and the ambiguity paper (W19) challenge how far these signals can be trusted | Boers, Nature Climate Change 11 (2021) |
| W15 | Critical slowing down in observations vs Earth system models | AMOC, eastern subpolar North Atlantic | OBSERVATIONAL + MODEL, ABSTRACT-ONLY | None | Models do not consistently show the stability loss seen in observations; observed and modelled signals agree only under warming beyond the Paris goal, which the authors read as models overestimating stability; the abstract itself says it is unclear whether signals are overlooked in models or over-interpreted in observations | 'Reconciled warning signals in observations and models imply approaching AMOC tipping point' (2025, Semantic Scholar record) |
| W16 | Critical slowing down (variance, AC(1)) | Western Greenland Ice Sheet | OBSERVATIONAL, TITLE/EXCERPT ONLY | None | Title says the ice sheet is 'close to a tipping point'; method, data and error rates not extracted | Boers and Rypdal, PNAS (2021), doi 10.1073/pnas.2024192118 |
| W17 | Standard deviation and AC(1) in sliding windows | Cenozoic deep-sea isotope records (CENOGRID), 9 earlier-identified transitions | RETROSPECTIVE, ABSTRACT-ONLY (conference abstract) | Not extracted | Significant signals in at least one of two records for 5 of 9 transitions, so 4 of 9 showed none by that criterion | EGU21 abstract EGU21-2520 (2021) |
| W18 | CSD and wavelet-based indicators | Greenland ice cores, Dansgaard-Oeschger events | RETROSPECTIVE, ABSTRACT-ONLY | Not extracted | Re-evaluation found fewer significant signals than earlier studies; counts not significant for most ice-core records, and for correlation-time estimators on their own; location-specific signals cannot be ruled out | Earth System Dynamics 16 (2025), 'Inconclusive early warning signals for Dansgaard-Oeschger events across Greenland ice cores' |
| W19 | Generic CSD indicators | AMOC (observation and model-based work) | ABSTRACT/EXCERPT ONLY | Not extracted | Excerpts state generic signals may not always be reliable in complex systems like the AMOC, and a 2025 Nature Climate Change PDF is titled 'Ambiguity of early warning signals for climate tipping points'; no results extracted | Geophysical Research Letters (2025) 10.1029/2024GL112415; Nature Climate Change (2025) s41558-025-02328-8 |
| W20 | CSD indicators in a calibrated box model, large ensemble | AMOC box model, bifurcation-, rate- and noise-induced tipping | MODEL/SIM, PREPRINT | Real-time probability per trajectory (CNN) | Under identical forcing some members tip and others do not; the preprint says CSD-based indicators are unreliable in this stochastic regime; the CNN alternative is simulation-only | arXiv 2509.06450 (2025) |
| W21 | Lag-1 autocorrelation, detrending diagnostics | Pine Island Glacier, West Antarctica, ice-flow model with slowly rising melt | MODEL/SIM | Autocorrelation indicator rises before unstable retreat in all three tested cases and approaches 1 near the event | Model-only; forcing was a gradual transient; the paper itself warns EWS need prior independent evidence that the system can tip | The Cryosphere 15 (2021) 1501 |

## Full-text findings (read)

### Jager and Fullsack 2019 (read in full)

- Design: 12 generic growth and decay systems (linear, quadratic, exponential, logarithmic, trigonometric, logistic), each with absolute noise and with relative noise, 22 usable system-noise cases; none can have a critical transition by construction, so every detected signal is a false positive. Series had 10,000 steps with rolling windows of 500, tracking standard deviation, AC(1), skewness and kurtosis.
- Result: 11 of 22 cases showed a clear simultaneous rise in standard deviation and AC(1). Under absolute noise: quadratic growth, exponential growth, logistic growth, quadratic decay, logarithmic decay, trigonometric decay, logistic decay. Under relative noise: quadratic decay, logarithmic decay, trigonometric decay, logistic decay.
- Mitigation tested: local and global scaling did NOT remove the false positives; detrending (local linear fit, global 5th-order polynomial, moving average of 50 steps) removed them in all investigated systems.
- Real-data check: global temperature behaved like the quadratic-growth system, showed the false signal on raw data, and showed none after detrending.
- Authors' stated limits: results rely on model-generated data; not all real systems resemble the 12 abstract systems; fluctuations that are neither absolute nor relative are out of scope. They also note it is unknown how detrending affects TRUE positives, and that publication bias hides failed detections.
- Implication for GAIA: any rolling variance plus autocorrelation monitor run on trending telemetry will raise false alarms unless the trend is removed first, and detrending's effect on real warnings is untested here.

### Boettiger and Hastings 2012 (read in full, arXiv preprint v1)

- Question: does testing warning signals only on systems known to have transitioned bias the result? This is the prosecutor's fallacy: a low probability of the evidence given an innocent system does not mean a low probability that the system is innocent given the evidence.
- Method: a stochastic individual-based birth-death model with an Allee threshold, with all parameters held constant so no bifurcation occurs and no true warning should exist. Runs of 50,000 time units sampled every 50 units. Replicates that collapsed by chance were selected, a window ending just before collapse (still above the threshold) was cut out, and variance and autocorrelation were computed in a moving window of half the series length, with Kendall's tau as the trend measure. A second model (a logistic growth with saturating predation, after May 1977) was used to show the effect is not specific to Allee models.
- Result: chance-collapsed replicates showed higher tau for both indicators than otherwise identical replicates that did not collapse. In a model with no underlying change, selecting collapsed cases made the indicators look like they had detected one.
- Mechanism given: to cross the threshold by chance, the system must move away from the stable state through a fast string of steps. That excursion looks like high autocorrelation and high variance within the window, but it is a chance trajectory, not the slowing return to equilibrium that critical slowing down is supposed to capture.
- Model-based comparison: a model-based estimate (an approximate saddle-node bifurcation model) found no difference from zero in any of the 266 collapsed replicates, so it showed no bias here. The authors say this should not be read as immunity to the prosecutor's fallacy.
- Why truncation does not fix it: the equilibrium location is unknown and may itself be moving in a system nearing a transition, so any rule for cutting off the collapse branch is arbitrary without a model of the process.
- Remedy named: replicated experiments, which generate a complete sample instead of a selected one.
- Limits and inconsistencies noticed: the text says 20,000 replicates were simulated and also says 266 of 1,000 collapsed in the window; I did not resolve which is meant. Only simulations are used; no field data are re-analysed. The preprint's figures were not inspected visually.
- Implication for GAIA: a detector validated only on past incidents that GAIA already knows were failures will look better than it is. A validation set must include stable periods and chance excursions that did not become failures.

### Dakos et al. 2012 (read through methods; results and limitations not returned)

- Provides a toolbox of metric-based indicators (AC(1), return rate, spectral density and ratio, DFA, standard deviation, coefficient of variation, skewness, kurtosis, conditional heteroskedasticity, BDS test) and model-based ones (time-varying AR(p), threshold AR(p), drift-diffusion-jump, potential analysis).
- Applied only to SIMULATED series from a harvested-resource model, because the authors state real series rarely have a clearly defined critical transition and few real series are available. In the CSD dataset the shift happened near step 970 of 1,000.
- States that detection is hard for two reasons: lack of suitable data (high-frequency sampling and designed experiments help but are often impossible; many sampling schemes deliberately avoid autocorrelation, which the indicators need) and no clear framework for applying them.
- DFA needs more than 100 points for robust estimation.
- Threshold AR(p) and potential analysis detect flickering, which the authors say is not strictly an early warning because the system has already switched repeatedly.
- The BDS test is not a leading indicator; it is a diagnostic that lowers the chance of a false detection when another strong indicator is also present.
- NOT read: the paper's own sensitivity and limitation findings for each indicator.

## Climate tipping notes (abstract level)

- This is where the literature is most contested. W14 and W16 report warning signals in current AMOC and Greenland observations; W15 says those signals and Earth system models disagree and that the question of whether they are overlooked or over-interpreted is unresolved; W18 and W17 show that re-analysis and paleoclimate checks give weaker or partial support; W19 and W20 say generic indicators can be unreliable in noisy or complex cases.
- A 2026 preprint on projection, memory and scalar early-warning validity (Semantic Scholar record) states that a single observable can reflect several different mechanisms, and proves failure cases: a critical mode may be invisible or unexcited; an oscillatory crossing need not raise lag-1 correlation; non-normal amplification can raise variance with a fixed stable spectrum. It is a preprint and its claims were not independently checked.
- A 2024 paper on edge states proposes using the unstable edge state to choose which observables carry critical slowing down, because many variables in high-dimensional systems may show no signal or changes unrelated to a tipping point. Simulation and conceptual models only.
- A review chapter on Earth system tipping points (Chapter 1.6, UPC repository) says appropriate use of early warning signals needs prior independent evidence that the system can actually exhibit tipping behaviour, as opposed to losing resilience with no alternative state; it lists western Greenland, AMOC and Amazon as showing loss of resilience consistent with approaching tipping points. Review-level, not primary evidence.
- Practical read for GAIA: even the best-studied, highest-stakes cases have unresolved disputes, and none has been scored against an outcome, since none has tipped.

## Conflict-warning notes (policy-level; no indicator evidence)

- The search for early-warning work on conflict escalation returned policy commentary, not statistical indicator validation. A 2025 NYU Center on International Cooperation piece says warning systems, even robust ones, rarely trigger timely action and describes a 'warning-response gap'.
- A Beyond Intractability essay says conflict early warning is harder than earlier types because it involves human behavior.
- Neither source gives a tested indicator, lead time or error rate. This leaves conflict escalation as an evidence gap, with only one transferable point: a warning that does not connect to a response is not useful, which fits the human-gate design in #1143.
- Not found: any study applying critical slowing down or similar generic statistics to conflict data with out-of-sample scoring.

## Finance notes

- W9 and W11 are not interchangeable. W9 tests whether market time series show critical slowing down. W11 tests whether slow-moving credit and debt aggregates predict banking distress. Evidence for one says nothing about the other.
- BIS describes the useful properties of early warning indicators as timing, stability, real-time availability and interpretability. These are a useful template for the #1146 horizon and regime fields.
- The 12-quarter window in W11 is a lead-time definition, not an observed lead time.

## Infrastructure notes

- W10 names three candidate signals from critical slowing down in power systems: slower recovery (if measurable directly), increased variance, increased autocorrelation. Whether they fire reliably on real grids before a failure is NOT established here.
- The abstract-level result supports treating telemetry as a source of ambiguous resilience-loss signals, not as proof a cascade is imminent.
- Related model-level theory for cascades is in CHAOS-002 M1 and M2 (interdependent networks); those describe how failures propagate, not how to detect them early.

## General failure modes

- Not all critical transitions can be detected (false negatives), and signals do not prove a transition is imminent (false positives). In the simulated cases of Jager and Fullsack 2019, whole classes of systems always show warning signals without featuring critical transitions (FULL-TEXT).
- Systematic false positives from trends: 11 of 22 trend-only cases raised both variance and AC(1). Detrending removed them in that study; scaling did not (FULL-TEXT).
- Prosecutor's fallacy: selecting systems because a transition occurred inflates the false-positive rate, because a chance excursion toward a threshold itself looks like rising variance and autocorrelation (Boettiger and Hastings 2012, FULL-TEXT).
- Truncating the series before the collapse does not cleanly solve it, because the equilibrium is unknown and can move (Boettiger and Hastings 2012, FULL-TEXT).
- Model-based estimation resisted the bias in that test but is not shown to be immune (Boettiger and Hastings 2012, FULL-TEXT).
- Noise-induced and rate-induced tipping: in a stochastic AMOC box model, identical forcing gave both transitions and non-transitions, and CSD indicators were reported unreliable (W20, abstract-level preprint).
- Single-variable ambiguity: one observable can mimic or miss critical slowing down (2026 preprint, abstract-level, unverified).
- Disagreement between data and models: observational signals may not be reproduced by models (W15, abstract-level).
- Weak replication under re-analysis: DO-event signals shrank under a robustness test (W18) and only 5 of 9 Cenozoic transitions showed signals (W17).
- Data requirements: indicators need high-frequency sampling and enough autocorrelation in the data; DFA needs more than 100 points (Dakos 2012, FULL-TEXT for these statements).
- Non-stationarity in the mean can cause spurious signals, especially for rolling-window metrics. A systematic rise in external noise can raise variance indicators but not memory indicators (PLoS ONE 7(7) e41010, 2012; excerpt level).
- Publication bias: studies that fail to find signals are rarely published, so the literature likely overstates success (Jager and Fullsack 2019, FULL-TEXT).
- Threshold choice trades sensitivity for specificity: a higher cut-off gives fewer and later warnings. Complex cases can give AUC below 0.5 (Nonlinear Dynamics, 2024; excerpt).
- The generic indicators do not say what lies beyond the tipping point (PNAS 2021).
- Inconsistent replication across events: W9 worked before one crash and not reliably before others.
- Link to CHAOS-002 M7: warning indicators may fail to announce a true transition, noise-induced transitions are unlikely to be announced, and in most cases indicators are detected only in retrospect (Scheffer and Carpenter 2003, excerpt level; NeedVerify). Boettiger and Hastings 2012 now gives a full-text, simulation-level account of why noise-driven transitions mislead retrospective tests.

## Retrospective vs prospective summary

- Prospective tests found so far: the lake experiments (W1-W4). All are induced transitions in one lake pair, so they test whether indicators respond, not whether a natural collapse can be forecast.
- Boettiger and Hastings 2012 explains why retrospective tests alone are unreliable and names replicated experiments as the clean remedy.
- The AMOC and Greenland signals (W14-W16) are observational and unscored: the transitions have not happened, so they are neither confirmed nor refuted.
- No prospective, in-advance, pre-registered forecast of an unforced natural transition has been collected yet. NeedVerify.
- Dakos 2012 itself uses only simulated data with known transitions, so it tests methods, not field forecasting.
- Epidemic, finance and power-system evidence collected is retrospective, model-based or abstract-only (W5, W6, W9, W10, W11).

## Criticality / edge-of-chaos target (docs/ANTI_CHAOS_CONTROL_PLANE.md)

Assumptions recorded from the doc, treated as HYPOTHESES:

- H1: branching ratio sigma in [0.95, 1.05] is the safe operating band.
- H2: QRC Thouless ratio tau in [0.5, 1.5] and eta in [0.45, 0.52] mark the desired phase.
- H3: overall_phi = 0.35*classical_soc + 0.30*qrc + 0.20*schumann + 0.15*noospheric weights these terms usefully.
- H4: the plane is 'live' (schema applied 2026-09-17), so these are already operating assumptions, not tested ones.

Review so far: the doc cites no literature for H1-H3. The weights in H3 and the Schumann and noospheric terms have NO supporting source collected here; tag NeedVerify. The collected early-warning literature treats loss of resilience near a tipping point as the danger signal, while the plane treats holding near criticality as the target. Whether these are compatible for GAIA's monitored systems is an open question, not a finding.

## Comparison with #1143 (proposed comments, NOT posted)

- Claim: deep-learning EWS 'outperforms statistical EWS in published benchmarks (PNAS 2021)'. Source abstract supports 'more sensitive and fewer false positives than generic indicators'; benchmark conditions and replication not checked. A 2025 AMOC preprint (W20) also reports a CNN approach working where CSD fails, but in simulation only.
- Claim: multi-variable fusion 'reduces false positive rate'. No supporting source collected. NeedVerify. The 2026 scalar-ambiguity preprint and the 2024 edge-state paper both suggest the choice of observable matters, which is a reason to test fusion rather than assume it helps.
- Gap: social stability, institutional trust and 'GAIAN behavioral tipping points' have no prospective evidence collected; only agent-based simulation (W8). Conflict-warning sources found are policy commentary without validated indicators.
- Gap: 'estimated distance to tipping point' - no source collected on whether distance is estimable; the AMOC abstract (W14) says estimates of the critical transition point remain uncertain.
- Gap: #1143 lists infrastructure cascades as a domain; the only infrastructure source collected (W10) is abstract-only, with no error rates.
- Addition: finance is not a #1143 deployment domain. If market stability is in scope, W9's mixed results and the W9/W11 family distinction should be recorded there.
- Addition (from W12): any #1143 variance-plus-autocorrelation monitor on trending data needs a documented detrending step and a test that detrending does not remove true warnings; Jager and Fullsack show 11 of 22 trend-only cases raised false alarms without it.
- Addition (from W13): validate any detector on a set that includes stable periods and chance excursions that did not become failures, not only on past incidents. Boettiger and Hastings show that selecting by outcome inflates apparent detection.
- Addition (from W14-W20): if climate tipping elements are in scope, record that the best-known cases are unscored, disputed between observations and models, and weakened under re-analysis (W17, W18).
- Consistent: #1143 'Honest Limits' (false positives in noisy systems, ambiguity flag, no autonomous action) match the failure modes above. The conflict-warning commentary also supports the no-autonomous-action and human-gate design.

## Fit with #1146 fields

Each catalog row maps to #1146 fields: lead time (horizon), evidence label (regime), failure cases (what this cannot predict), and the failure modes above (EWS ambiguity statement).

## Open items

- [x] Read Jager and Fullsack 2019 in full.
- [x] Read Boettiger and Hastings 2012 (prosecutor's fallacy) in full.
- [~] Read Dakos 2012: methods read; results and limitations still to get (PLoS printable-PDF fetch failed; try a different host).
- [~] Climate tipping: abstract-level rows W14-W21 collected; none read in full. Priority: Boers 2021; the 2025 ambiguity paper (EPIC PDF); the DO-event re-analysis.
- [ ] Read in full: Scheffer 2009; Carpenter 2011 (PDF fetch failed); PNAS 2021; the 2018 finance paper (HAL blocked); the 2015 power-system paper (ScienceDirect blocked, try another host); Boettiger and Hastings 2012 Interface paper on limits to detection.
- [ ] Extract exact lead times and false-positive/false-negative rates where reported.
- [ ] Find quantitative studies of early warning for conflict escalation, or confirm none exist; collect empirical flickering cases; check Amazon rainforest evidence.
- [ ] Find prospective forecasts of unforced natural transitions, or confirm none exist.
- [ ] Test whether detrending removes TRUE warnings (open question raised by Jager and Fullsack).
- [ ] Resolve the 20,000 vs 1,000 replicate count in the Boettiger and Hastings text.
- [ ] Find literature on criticality targets (e.g. self-organized criticality, branching ratio) for H1-H3.
- [ ] Cross-check with #1143 and #1146; post proposed comments only after human approval.
- [ ] Human review.
