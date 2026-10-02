# ALIGNMENT

Claim class: experimental (methods), established only where already gated in code (Suggest default, receipt, kill, refuse live Supabase).
Evoke: copy the math. Do not invoke a rater as law.

## RICE (map, do not fork a new module)

Robustness, Interpretability, Controllability, Ethicality.
GAIA already owns C as pause / kill / receipt. R is evals and shift. I is traces, not a reasoner-soul. E is the human spec, not RLHF.

## RLHF then DPO

KL-regularized RLHF has a closed-form policy. Invert the reward. Bradley-Terry cancels Z. DPO is logistic loss on the implicit reward

    r-hat(x,y) = beta log(pi_theta(y|x) / pi_ref(y|x))

No separate reward net. Spec is still whoever labeled the pairs.

## IPO and cousins

IPO squares the log-ratio gap to a finite target 1/(2 beta). Stops unbounded DPO margins.
Offline / online / IPO-MD (geometric mix of pi and pi_ref when sampling).
KTO unpaired. ORPO / SimPO drop or free the reference. Loss swap is ~1 point after correction in one 2026 bake-off; scale and online vs offline move more.

## NLHF / Nash-MD / Nash-MP / Nash Prox

Preference model P(y > y'|x) defines a two-player game.
Regularized P_beta bills both sides with KL to pi_ref. Unique symmetric Nash pi*_beta.
Nash-MD: mirror step against a geometric mixture opponent.
Nash-MP: two Bregman proxes (lookahead + correction), last-iterate linear in KL under beta-strong geometry.
Nash Prox: EMA target as cheap second anchor; mix of KL-to-ref and KL-to-target in an IPO-shaped square loss. kappa ~ 1/T_k.

Refuse: Nash of a sycophantic judge is not a conscience. Human pause stays outside the game.

## Mirror geometry (reading)

Bregman D_h(x,y) = h(x)-h(y)-<grad h(y), x-y>. Entropy -> KL. Three-point identity is the descent lemma.
Mirror Descent: one prox. Dual averaging / FTRL safer with shrinking steps.
Mirror Prox / extra-gradient: two calls. Monotone Lip gap of the average is O(L Omega / K). Strongly monotone last-iterate contracts.

## Extrapolation family

Korpelevich EG: two F, two projections, gamma < 1/L sharp.
Popov / OGDA: reuse last lookahead, one F per step after warmup.
EG and OGDA approximate proximal point; they split on some time-varying games.
EAG: EG + Halpern anchor epsilon_k (x - v), last ||F|| = O(1/k), ||F||^2 = O(1/k^2) optimal on monotone Lip equations.
FEG: EG+ two-time-scale plus anchor; same fast residual under negative comonotonicity.
S-FEG: fast rate plus a floor unless Var(xi) decays like 1/k. Not SVRG.
SVRG-EG: finite-sum snapshot estimator in both EG legs. Last-iterate linear under an error bound (bilinear games included), not only mu-strong monotonicity. Needs a frozen finite sum.

## GAIA bind

| Idea | Bind | Refuse |
| --- | --- | --- |
| Labels / P | audit who wrote them | treat as constitution |
| beta / KL to ref | stay near issued manifest | disappear the human |
| EMA / anchor | stabilize last iterate | replace kill |
| SVRG | only on a frozen pool | live RM + fresh rollouts as finite-sum |
| last-iterate rates | eval discipline | proof of virtue |

Parent shelf. Does not implement a trainer. Does not open #341 MCP zero-trust.

## Value pluralism

More than one good can be real. They do not collapse into one reward. See `VALUE-PLURALISM.md`. A conflict stays visible.

A preference loss fits labeled pairs. It does not discover a rate at which dignity converts into helpfulness. Ethicality stays the human spec. The witness has to stand outside the optimizer. See `VALUE-PLURALISM.md`.
