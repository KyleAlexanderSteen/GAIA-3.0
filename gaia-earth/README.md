# gaia-earth

Tracking crate for the Artificial Twin of Earth ([#33](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/33)).

This crate is the observation contract. It is not a data lake, STAC catalog,
Kafka cluster, or live planetary model.

Hard rules encoded here:

- Every point is `Measured` or labeled `Synthetic`
- Uncertainty is required
- Weaponization purposes are refused

Child work stays on its own issues: Data Commons #34 / #40–#43, profiles #56.
