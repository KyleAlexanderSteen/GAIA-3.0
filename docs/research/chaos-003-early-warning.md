# CHAOS-003: Early warning indicators of destructive chaos

Issue: #1200 (parent #1175). Related, not duplicated: #1143, #1146, #1127. Evidence standard: #1171.
Status: DRAFT v0.2. Research and specification only; no runtime code; cannot raise autonomy or bypass a human gate.
Scope note: this catalog recommends NO indicator for operational use. Any use needs its own issue, calibration (#1146) and the existing human gates.

## Stage reached

- Read: search-result abstracts and excerpts only. NO full paper was read. Full-text fetches of the power-system paper (ScienceDirect, bot check) and the financial-crisis paper (Springer, truncated) did not return usable text.
- Domains with sources collected: ecosystems (field experiment), generic statistics and false positives, epidemics, deep-learning detection, agent-based social simulation, finance (two different indicator families), power systems (abstract only).
- v0.2 adds W9-W11 and the finance and infrastructure sections.
- NOT covered, no sources collected (NeedVerify): conflict escalation, flickering, climate tipping elements (AMOC, ice sheets). A search for conflict-escalation early-warning work returned only financial and power-system results, so that gap is open.
- Lead times below are what the abstracts state; exact figures were not extracted.
- Author names are omitted where the search output did not show them.

## Evidence labels

- PROSPECTIVE-EXPERIMENT: a transition was deliberately induced and monitored against a reference system. A prospective test of the indicator, but NOT a forecast of an unforced natural event.
- RETROSPECTIVE: indicator fitted or checked on data after the transition is known.
- MODEL/SIM: simulation or theory only.
- ABSTRACT-ONLY: described in an abstract; method, data and error rates not checked.

## Warning-signal catalog

| # | Indicator | System | Evidence label | Reported lead time | Known failure cases | Source |
|---|-----------|--------|----------------|--------------------|---------------------|--------|
| W1 | Rising variance and lag-1 autocorrelation (critical slowing down) | Whole-lake food web, manipulated vs reference lake | PROSPECTIVE-EXPERIMENT | Signals evident more than a year before the transition completed | Experimentally forced, not a natural event; one lake pair | Carpenter et al., Science 332 (2011) |
| W2 | Zooplankton variance and autocorrelation | Same lake experiment | PROSPECTIVE-EXPERIMENT | Elevated in year 3 of 4 | Signal returned to reference-lake levels in year 4 as the transition completed | Limnology and Oceanography 58 (2013) |
| W3 | Conditional heteroskedasticity | Same lake experiment | PROSPECTIVE-EXPERIMENT | At least a year before the shift | Single experiment | Ecosystems (2012) |
| W4 | Spatial variance, low-frequency spatial variance | Same lake experiment, prey-fish catch | PROSPECTIVE-EXPERIMENT | Reported as early warning | Single experiment | Ecosphere (2014) |
| W5 | Variance, autocorrelation, skewness, kurtosis | COVID-19 case curves, multiple countries | RETROSPECTIVE | Authors suggest 2-3 weeks | Confounded by interventions and reporting; abstract says increases predict onset for 'most' datasets, so some did not | Frontiers in Public Health (2020) |
| W6 | Second factorial moment, decay time of reported cases | Birth-death-immigration case-report models | MODEL/SIM | n/a | Estimates from a single series have high variance; averaging over an ensemble helps | PMC8455094 |
| W7 | Deep-learning classifier trained on simulated normal forms | Real-world systems incl. paleoclimate and thermoacoustic data | Mixed; see source | Not extracted | Claimed more sensitive with fewer false positives than generic indicators; independent replication NOT checked | PNAS (2021), doi 10.1073/pnas.2106140118 |
| W8 | LSTM on EWS of cooperation collapse | Agent-based public-good game | MODEL/SIM | n/a | Simulation only; no real population | Elsevier (2020) |
| W9 | Critical slowing down statistics | Financial markets: Black Monday 1987, and later crises | RETROSPECTIVE | Not extracted | Statistical evidence before Black Monday 1987, but results mixed or insignificant for the more recent crises; inconsistent across events | Empirical Economics (2018), 'Critical slowing down as an early warning signal for financial crises?' |
| W10 | Slower recovery from perturbations, rising variance, rising autocorrelation | Power systems approaching critical transitions | ABSTRACT-ONLY | Not extracted | Error rates, thresholds and real-grid validation not extracted | Electric Power Systems Research, 'Early warning signals for critical transitions in power systems' (2015) |
| W11 | Credit and debt-based indicators (debt-service ratios, credit-to-GDP gaps, household and cross-border debt) | Banking distress, cross-country | RETROSPECTIVE | Alert counts as correct if issued at least once in the 12 quarters before a crisis | A different indicator family from critical slowing down; the criterion rewards any alert in a long window, so it can hide false alarms; exact false-alarm rates not extracted | BIS Quarterly Review (March 2018) |

## Finance notes

- W9 and W11 are not interchangeable. W9 tests whether market time series show critical slowing down. W11 tests whether slow-moving credit and debt aggregates predict banking distress. Evidence for one says nothing about the other.
- BIS describes the useful properties of early warning indicators as timing, stability, real-time availability and interpretability. These are a useful template for the #1146 horizon and regime fields.
- The 12-quarter window in W11 is a lead-time definition, not an observed lead time.

## Infrastructure notes

- W10 names three candidate signals from critical slowing down in power systems: slower recovery (if measurable directly), increased variance, increased autocorrelation. Whether they fire reliably on real grids before a failure is NOT established here.
- The abstract-level result supports treating telemetry as a source of ambiguous resilience-loss signals, not as proof a cascade is imminent.
- Related model-level theory for cascades is in CHAOS-002 M1 and M2 (interdependent networks); those describe how failures propagate, not how to detect them early.

## General failure modes (abstract and excerpt level)

- Not all critical transitions can be detected (false negatives), and signals do not prove a transition is imminent (false positives). Some classes of systems always show warning signals without featuring critical transitions (PMC6364907).
- Prosecutor's fallacy: selecting systems because a transition occurred inflates false-positive rates. Simulated systems that transitioned purely by chance showed elevated false positives in common statistics. Experiments with replicates avoid this pitfall (Proc. R. Soc. B 2012; arXiv 1210.1204).
- Non-stationarity in the mean can cause spurious signals, especially for rolling-window metrics. A systematic rise in external noise can raise variance indicators but not memory indicators (PLoS ONE 7(7) e41010, 2012).
- Threshold choice trades sensitivity for specificity: a higher cut-off gives fewer and later warnings. Complex cases can give AUC below 0.5 (Nonlinear Dynamics, 2024; excerpt).
- The generic indicators do not say what lies beyond the tipping point (PNAS 2021).
- Inconsistent replication across events: W9 worked before one crash and not reliably before others.
- Link to CHAOS-002 M7: warning indicators may fail to announce a true transition, noise-induced transitions are unlikely to be announced, and in most cases indicators are detected only in retrospect (Scheffer and Carpenter 2003, excerpt level; NeedVerify).

## Retrospective vs prospective summary

- Prospective tests found so far: the lake experiments (W1-W4). All are induced transitions in one lake pair, so they test whether indicators respond, not whether a natural collapse can be forecast.
- No prospective, in-advance, pre-registered forecast of an unforced natural transition has been collected yet. NeedVerify.
- Epidemic, finance and power-system evidence collected is retrospective, model-based or abstract-only (W5, W6, W9, W10, W11).

## Criticality / edge-of-chaos target (docs/ANTI_CHAOS_CONTROL_PLANE.md)

Assumptions recorded from the doc, treated as HYPOTHESES:

- H1: branching ratio sigma in [0.95, 1.05] is the safe operating band.
- H2: QRC Thouless ratio tau in [0.5, 1.5] and eta in [0.45, 0.52] mark the desired phase.
- H3: overall_phi = 0.35*classical_soc + 0.30*qrc + 0.20*schumann + 0.15*noospheric weights these terms usefully.
- H4: the plane is 'live' (schema applied 2026-09-17), so these are already operating assumptions, not tested ones.

Review so far: the doc cites no literature for H1-H3. The weights in H3 and the Schumann and noospheric terms have NO supporting source collected here; tag NeedVerify. The collected early-warning literature treats loss of resilience near a tipping point as the danger signal, while the plane treats holding near criticality as the target. Whether these are compatible for GAIA's monitored systems is an open question, not a finding.

## Comparison with #1143 (proposed comments, NOT posted)

- Claim: deep-learning EWS 'outperforms statistical EWS in published benchmarks (PNAS 2021)'. Source abstract supports 'more sensitive and fewer false positives than generic indicators'; benchmark conditions and replication not checked.
- Claim: multi-variable fusion 'reduces false positive rate'. No supporting source collected. NeedVerify.
- Gap: social stability, institutional trust and 'GAIAN behavioral tipping points' have no prospective evidence collected; only agent-based simulation (W8).
- Gap: 'estimated distance to tipping point' - no source collected on whether distance is estimable.
- Gap: #1143 lists infrastructure cascades as a domain; the only infrastructure source collected (W10) is abstract-only, with no error rates.
- Addition: finance is not a #1143 deployment domain. If market stability is in scope, W9's mixed results and the W9/W11 family distinction should be recorded there.
- Consistent: #1143 'Honest Limits' (false positives in noisy systems, ambiguity flag, no autonomous action) match the failure modes above.

## Fit with #1146 fields

Each catalog row maps to #1146 fields: lead time (horizon), evidence label (regime), failure cases (what this cannot predict), and the failure modes above (EWS ambiguity statement).

## Open items

- [ ] Read primary papers in full: Scheffer 2009; Dakos 2012; Carpenter 2011; Boettiger and Hastings 2012; PNAS 2021; the 2018 finance paper; the 2015 power-system paper (try another host).
- [ ] Extract exact lead times and false-positive/false-negative rates where reported.
- [ ] Collect sources for conflict escalation, flickering, climate tipping elements.
- [ ] Find prospective forecasts of unforced natural transitions, or confirm none exist.
- [ ] Find literature on criticality targets (e.g. self-organized criticality, branching ratio) for H1-H3.
- [ ] Cross-check with #1143 and #1146; post proposed comments only after human approval.
- [ ] Human review.
