# CHAOS-003: Early warning indicators of destructive chaos

Issue: #1200 (parent #1175). Related, not duplicated: #1143, #1146, #1127. Evidence standard: #1171.
Status: DRAFT v0.1. Research and specification only; no runtime code; cannot raise autonomy or bypass a human gate.
Scope note: this catalog recommends NO indicator for operational use. Any use needs its own issue, calibration (#1146) and the existing human gates.

## Stage reached

- Read: search-result abstracts and excerpts only. NO full paper was read for this draft.
- Domains with sources collected: ecosystems (field experiment), generic statistics and false positives, epidemics, deep-learning detection, agent-based social simulation.
- NOT covered, no sources collected (NeedVerify): financial and market crashes, conflict escalation, infrastructure cascades, flickering, climate tipping elements (AMOC, ice sheets).
- Lead times below are what the abstracts state; exact figures were not extracted.

## Evidence labels

- PROSPECTIVE-EXPERIMENT: a transition was deliberately induced and monitored against a reference system. This is a prospective test of the indicator, but NOT a forecast of an unforced natural event.
- RETROSPECTIVE: indicator fitted or checked on data after the transition is known.
- MODEL/SIM: simulation or theory only.

## Warning-signal catalog

| # | Indicator | System | Evidence label | Reported lead time | Known failure cases | Source |
|---|-----------|--------|----------------|--------------------|---------------------|--------|
| W1 | Rising variance and lag-1 autocorrelation (critical slowing down) | Whole-lake food web, manipulated vs reference lake | PROSPECTIVE-EXPERIMENT | Signals evident more than a year before the transition completed | Experimentally forced, not a natural event; one lake pair | Carpenter et al., Science 332 (2011) |
| W2 | Zooplankton variance and autocorrelation | Same lake experiment | PROSPECTIVE-EXPERIMENT | Elevated in year 3 of 4 | Signal returned to reference-lake levels in year 4 as the transition completed | Limnology and Oceanography 58 (2013) |
| W3 | Conditional heteroskedasticity | Same lake experiment | PROSPECTIVE-EXPERIMENT | At least a year before the shift | Single experiment | Ecosystems (2012) |
| W4 | Spatial variance, low-frequency spatial variance | Same lake experiment, prey-fish catch | PROSPECTIVE-EXPERIMENT | Reported as early warning | Single experiment | Ecosphere (2014) |
| W5 | Variance, autocorrelation, skewness, kurtosis | COVID-19 case curves, multiple countries | RETROSPECTIVE | Authors suggest 2-3 weeks | Confounded by interventions and reporting; abstract says increases predict onset for 'most' datasets, so some did not | Frontiers in Public Health (2020) |
| W6 | Second factorial moment, decay time of reported cases | Birth-death-immigration case-report models | MODEL/SIM | n/a | Estimates from a single series have high variance; averaging over an ensemble helps | PMC8455094 |
| W7 | Deep-learning classifier trained on simulated normal forms | Real-world systems incl. paleoclimate and thermoacoustic data | Mixed; see source | Not extracted | Claimed more sensitive with fewer false positives than generic indicators; independent replication NOT checked | Bury et al., PNAS (2021) |
| W8 | LSTM on EWS of cooperation collapse | Agent-based public-good game | MODEL/SIM | n/a | Simulation only; no real population | Elsevier (2020) |

## General failure modes (primary-source level, abstracts only)

- Not all critical transitions can be detected (false negatives), and signals do not prove a transition is imminent (false positives). Some classes of systems always show warning signals without featuring critical transitions (PMC6364907).
- Prosecutor's fallacy: selecting systems because a transition occurred inflates false-positive rates. Simulated systems that transitioned purely by chance showed elevated false positives in common statistics. Experiments with replicates avoid this pitfall (Boettiger and Hastings, Proc. R. Soc. B 2012; arXiv 1210.1204).
- Non-stationarity in the mean can cause spurious signals, especially for rolling-window metrics. A systematic rise in external noise can raise variance indicators but not memory indicators (Dakos et al., PLoS ONE 7(7) e41010, 2012).
- Threshold choice trades sensitivity for specificity: a higher cut-off gives fewer and later warnings. Complex cases can give AUC below 0.5 (Springer, Nonlinear Dynamics, 2024; excerpt).
- The generic indicators do not say what lies beyond the tipping point (Bury et al. 2021).
- Link to CHAOS-002 M7: warning indicators may fail to announce a true transition, noise-induced transitions are unlikely to be announced, and in most cases indicators are detected only in retrospect (Scheffer and Carpenter 2003, excerpt level; NeedVerify).

## Retrospective vs prospective summary

- Prospective tests found so far: the Carpenter lake experiments (W1-W4). All are induced transitions in one lake pair, so they test whether indicators respond, not whether a natural collapse can be forecast.
- No prospective, in-advance, pre-registered forecast of an unforced natural transition has been collected yet. NeedVerify.
- Epidemic evidence collected is retrospective (W5) or model-based (W6).

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
- Consistent: #1143 'Honest Limits' (false positives in noisy systems, ambiguity flag, no autonomous action) match the failure modes above.

## Fit with #1146 fields

Each catalog row maps to #1146 fields: lead time (horizon), evidence label (regime), failure cases (what this cannot predict), and the failure modes above (EWS ambiguity statement).

## Open items

- [ ] Read primary papers in full: Scheffer 2009; Dakos 2012; Carpenter 2011; Boettiger and Hastings 2012; Bury 2021.
- [ ] Extract exact lead times and false-positive/false-negative rates where reported.
- [ ] Collect sources for finance, conflict, infrastructure, flickering, climate tipping elements.
- [ ] Find prospective forecasts of unforced natural transitions, or confirm none exist.
- [ ] Find literature on criticality targets (e.g. self-organized criticality, branching ratio) for H1-H3.
- [ ] Cross-check with #1143 and #1146; post proposed comments only after human approval.
- [ ] Human review.
