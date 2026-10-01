# Forecast calibration, listed

Partial for #1272 and #1171. No trainer. No 2027 forecast. Does not close Predict-1 through Predict-6.

Checked 2026-10-01.

## Score

For a binary event, a forecast is a probability \(p\) in \([0, 1]\) and the outcome \(o\) is 1 if the event happens and 0 if it does not. The Brier score is the mean squared error

\[
BS = \frac{1}{N}\sum_{t=1}^{N}(p_t - o_t)^2.
\]

Lower is better. On this two-outcome form the score lies in \([0, 1]\). Brier's 1950 multi-category sum can reach 2. A note must say which form it uses. A score without a named horizon and a resolved outcome set is NeedVerify.

The score mixes three things. Calibration: events called 80 percent happen about 80 percent of the time. Resolution: the forecast separates events that happen from events that do not. Uncertainty: the base rate of the events. A good headline score is not proof of calibration. The calibration check is a reliability diagram, forecast probability against observed frequency. A calibrated forecast sits on the diagonal.

## What a ledger row needs

| Field | Required |
| --- | --- |
| question | yes |
| horizon | yes |
| forecast probability | yes |
| outcome, once resolved | yes |
| score form | yes, binary or multi-category |
| source of the resolution | yes |

No row in this cut is a live forecast.

## External numbers, not GAIA results

ForecastBench, as reported on 4 March 2026, listed superforecasters at a Brier score of 0.086 and a Brier Index of 70.6 percent. That index is \((1 - \sqrt{BS}) \times 100\). It is not percent correct. Those numbers are theirs. GAIA has not run that benchmark.

## Refusal

This file does not train a model, score a GAIA prediction, or claim a 2027 path. The earth ensemble on main remains a fixture.
