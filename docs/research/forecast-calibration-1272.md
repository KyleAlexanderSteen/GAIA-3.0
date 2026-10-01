# Forecast calibration, listed

Partial for #1272 and #1171. No trainer. No 2027 forecast. Does not close Predict-1 through Predict-6.

Checked 2026-10-01.

## Score

For a binary event, a forecast is a probability \(p\) in \([0, 1]\) and the outcome \(o\) is 1 if the event happens and 0 if it does not. The Brier score is the mean squared error

\[
BS = \frac{1}{N}\sum_{t=1}^{N}(p_t - o_t)^2.
\]

Lower is better. On this two-outcome form the score lies in \([0, 1]\). Brier's 1950 multi-category sum can reach 2. A note must say which form it uses. A score without a named horizon and a resolved outcome set is NeedVerify.

## Decomposition

Murphy, Journal of Applied Meteorology, 1973, partitions the probability score into three terms. The form used in later notes is

\[
BS = REL - RES + UNC.
\]

Reliability \(REL\) is the squared gap between the forecast bin and the observed frequency in that bin. Resolution \(RES\) is how far those observed frequencies sit from the base rate. Uncertainty \(UNC\) is the base rate itself, \(\bar{o}(1-\bar{o})\), and does not depend on the forecaster. A good headline score can hide a bad reliability term. The calibration check is the reliability diagram: forecast probability on one axis, observed frequency on the other, with counts per bin. A calibrated forecast sits on the diagonal.

Proper scoring rules reward reporting the probability you actually hold. Brier and the logarithmic score are both proper. They punish differently. Log score punishes a near-zero probability on an event that happens more sharply. A ledger that reports only Brier must not be read as a log-score result.

## Ledger row

| Field | Required |
| --- | --- |
| question | yes |
| horizon, open and resolve dates | yes |
| forecast probability | yes |
| outcome, once resolved | yes |
| score form | yes, binary or multi-category |
| bin counts, if a reliability claim is made | yes |
| source of the resolution | yes |

No row in this cut is a live forecast.

## External numbers, not GAIA results

ForecastBench, as reported on 4 March 2026, listed superforecasters at a Brier score of 0.086 and a Brier Index of 70.6 percent. That index is \((1 - \sqrt{BS}) \times 100\). It is not percent correct. Those numbers are theirs. GAIA has not run that benchmark.

Good Judgment Project practice, as summarized in the secondary literature, treats calibration as average confidence against percent correct, and resolution as whether correct calls were made away from 50 percent. That is a reading aid. It is not a second score.

## Refusal

This file does not train a model, score a GAIA prediction, or claim a 2027 path. The earth ensemble on main remains a fixture.
