//! #1142 Predict-1 trajectory ensemble + confidence band.
//! #37 What-Now, What-Next, What-If.
//! Fixture distributions. Not a GCM, SSP runner, or tipping cascade.

use crate::SourceKind;

// ── Existing simulation types (unchanged) ─────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimMode {
    WhatNow,
    WhatNext,
    WhatIf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutcomeKind {
    Climate,
    Biodiversity,
    Economy,
    Welfare,
    TippingRisk,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Distribution {
    pub kind: OutcomeKind,
    pub mean: f64,
    pub uncertainty: f64,
    pub source: SourceKind,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScenarioRun {
    pub id: String,
    pub mode: SimMode,
    pub outcomes: Vec<Distribution>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimError {
    UnknownScenario,
    MissingUncertainty,
    EnsembleTooSmall,
}

impl std::fmt::Display for SimError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownScenario => write!(f, "unknown scenario"),
            Self::MissingUncertainty => write!(f, "uncertainty is mandatory"),
            Self::EnsembleTooSmall => write!(f, "ensemble requires at least 2 members"),
        }
    }
}

pub struct ScenarioEngine {
    library: Vec<(&'static str, SimMode)>,
}

impl Default for ScenarioEngine {
    fn default() -> Self {
        Self {
            library: vec![
                ("state-now", SimMode::WhatNow),
                ("seasonal-next", SimMode::WhatNext),
                ("net-zero by 2040", SimMode::WhatIf),
                ("AMOC -30%", SimMode::WhatIf),
            ],
        }
    }
}

impl ScenarioEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn library(&self) -> &[(&'static str, SimMode)] {
        &self.library
    }

    pub fn run(&self, id: &str) -> Result<ScenarioRun, SimError> {
        let mode = self
            .library
            .iter()
            .find(|(name, _)| *name == id)
            .map(|(_, mode)| *mode)
            .ok_or(SimError::UnknownScenario)?;
        Ok(ScenarioRun {
            id: id.into(),
            mode,
            outcomes: stub_outcomes(),
        })
    }
}

fn stub_outcomes() -> Vec<Distribution> {
    [
        OutcomeKind::Climate,
        OutcomeKind::Biodiversity,
        OutcomeKind::Economy,
        OutcomeKind::Welfare,
        OutcomeKind::TippingRisk,
    ]
    .into_iter()
    .map(|kind| Distribution {
        kind,
        mean: 0.0,
        uncertainty: 1.0,
        source: SourceKind::Synthetic,
    })
    .collect()
}

// ── Predict-1: Trajectory ensemble + confidence band ──────────────────────
// A trajectory ensemble runs N perturbed forward projections from the same
// initial state. The spread of member trajectories IS the uncertainty.
// Wider spread = less predictable future = wider confidence band.
// No GCM weights, no GraphCast. Fixture values until live ingest connected.

/// Predictability horizon flag — how reliable is this forecast window?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HorizonFlag {
    /// Forecast is within the reliable window for this domain.
    WithinHorizon,
    /// Forecast is approaching the edge of reliable window; interpret with care.
    ApproachingHorizon,
    /// Forecast is beyond reliable window; treat as low-confidence hypothesis.
    BeyondHorizon,
}

/// A single member trajectory: a sequence of outcome values over time steps.
#[derive(Debug, Clone, PartialEq)]
pub struct TrajectoryMember {
    pub kind: OutcomeKind,
    pub source: SourceKind,
    /// Values at each time step t+1, t+2, … t+n.
    pub steps: Vec<f64>,
}

/// Confidence band derived from an ensemble of trajectory members.
/// mean ± half_width_90 is the 90% confidence interval at each step.
/// uncertainty is mandatory and must be > 0.0.
#[derive(Debug, Clone, PartialEq)]
pub struct ConfidenceBand {
    pub kind: OutcomeKind,
    /// Per-step ensemble mean.
    pub mean: Vec<f64>,
    /// Per-step half-width of the 90% confidence interval.
    pub half_width_90: Vec<f64>,
    /// Propagated uncertainty estimate.
    pub uncertainty: f64,
    /// Horizon flag for this forecast.
    pub horizon: HorizonFlag,
    /// Plain-language horizon statement — mandatory on every output.
    pub horizon_statement: &'static str,
}

impl ConfidenceBand {
    pub fn horizon_statement(&self) -> &'static str {
        self.horizon_statement
    }
}

/// A trajectory ensemble: N members for one OutcomeKind.
/// Call `build_band()` to collapse to a `ConfidenceBand`.
#[derive(Debug, Clone, PartialEq)]
pub struct TrajectoryEnsemble {
    pub kind: OutcomeKind,
    pub members: Vec<TrajectoryMember>,
}

impl TrajectoryEnsemble {
    /// Create a new ensemble. Returns `SimError::EnsembleTooSmall` if < 2 members.
    pub fn new(kind: OutcomeKind, members: Vec<TrajectoryMember>) -> Result<Self, SimError> {
        if members.len() < 2 {
            return Err(SimError::EnsembleTooSmall);
        }
        Ok(Self { kind, members })
    }

    /// Number of ensemble members.
    pub fn size(&self) -> usize {
        self.members.len()
    }

    /// Collapse ensemble to a `ConfidenceBand`.
    /// `uncertainty` must be > 0.0 or returns `SimError::MissingUncertainty`.
    /// `horizon_steps` controls the horizon flag: within ≤ 7, approaching ≤ 30, beyond > 30.
    pub fn build_band(
        &self,
        uncertainty: f64,
        horizon_steps: usize,
    ) -> Result<ConfidenceBand, SimError> {
        if uncertainty <= 0.0 {
            return Err(SimError::MissingUncertainty);
        }

        let n_steps = self.members.iter().map(|m| m.steps.len()).min().unwrap_or(0);
        let n_members = self.members.len() as f64;

        let mean: Vec<f64> = (0..n_steps)
            .map(|t| self.members.iter().map(|m| m.steps[t]).sum::<f64>() / n_members)
            .collect();

        // 90% CI half-width approximation: 1.645 * std_dev / sqrt(n)
        let half_width_90: Vec<f64> = (0..n_steps)
            .map(|t| {
                let m = mean[t];
                let var = self
                    .members
                    .iter()
                    .map(|mb| (mb.steps[t] - m).powi(2))
                    .sum::<f64>()
                    / (n_members - 1.0);
                1.645 * var.sqrt() / n_members.sqrt()
            })
            .collect();

        let (horizon, horizon_statement) = match horizon_steps {
            0..=7 => (
                HorizonFlag::WithinHorizon,
                "Forecast is within the reliable window for this domain.",
            ),
            8..=30 => (
                HorizonFlag::ApproachingHorizon,
                "Forecast is approaching the reliable horizon. Interpret with care and wider uncertainty.",
            ),
            _ => (
                HorizonFlag::BeyondHorizon,
                "Forecast is beyond the reliable horizon. \
                 Treat outputs as low-confidence hypotheses, not forecasts.",
            ),
        };

        Ok(ConfidenceBand {
            kind: self.kind,
            mean,
            half_width_90,
            uncertainty,
            horizon,
            horizon_statement,
        })
    }
}

/// Build a fixture ensemble for `kind` with `n_members` members of `n_steps` steps.
/// All values are synthetic stubs — not GCM output.
pub fn fixture_ensemble(
    kind: OutcomeKind,
    n_members: usize,
    n_steps: usize,
) -> Result<TrajectoryEnsemble, SimError> {
    let members = (0..n_members)
        .map(|i| TrajectoryMember {
            kind,
            source: SourceKind::Synthetic,
            steps: (0..n_steps)
                .map(|t| (t as f64) * 0.1 + (i as f64) * 0.05)
                .collect(),
        })
        .collect();
    TrajectoryEnsemble::new(kind, members)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_member_rejected() {
        let member = TrajectoryMember {
            kind: OutcomeKind::Climate,
            source: SourceKind::Synthetic,
            steps: vec![1.0, 2.0, 3.0],
        };
        assert_eq!(
            TrajectoryEnsemble::new(OutcomeKind::Climate, vec![member]),
            Err(SimError::EnsembleTooSmall)
        );
    }

    #[test]
    fn zero_uncertainty_rejected() {
        let ens = fixture_ensemble(OutcomeKind::Climate, 5, 10).unwrap();
        assert_eq!(ens.build_band(0.0, 7), Err(SimError::MissingUncertainty));
    }

    #[test]
    fn within_horizon_flag() {
        let ens = fixture_ensemble(OutcomeKind::TippingRisk, 10, 7).unwrap();
        let band = ens.build_band(0.1, 7).unwrap();
        assert_eq!(band.horizon, HorizonFlag::WithinHorizon);
        assert_eq!(band.mean.len(), 7);
        assert_eq!(band.half_width_90.len(), 7);
    }

    #[test]
    fn beyond_horizon_flag() {
        let ens = fixture_ensemble(OutcomeKind::Climate, 4, 31).unwrap();
        let band = ens.build_band(0.2, 31).unwrap();
        assert_eq!(band.horizon, HorizonFlag::BeyondHorizon);
        assert!(!band.horizon_statement().is_empty());
    }

    #[test]
    fn fixture_ensemble_smoke() {
        let ens = fixture_ensemble(OutcomeKind::Biodiversity, 8, 10).unwrap();
        assert_eq!(ens.size(), 8);
        let band = ens.build_band(0.05, 10).unwrap();
        assert_eq!(band.kind, OutcomeKind::Biodiversity);
        assert!(band.half_width_90.iter().all(|w| w.is_finite()));
    }

    // Existing ScenarioEngine tests unchanged.
    #[test]
    fn scenario_engine_known_id() {
        let eng = ScenarioEngine::new();
        let run = eng.run("state-now").unwrap();
        assert_eq!(run.mode, SimMode::WhatNow);
        assert_eq!(run.outcomes.len(), 5);
    }

    #[test]
    fn scenario_engine_unknown_id() {
        let eng = ScenarioEngine::new();
        assert_eq!(eng.run("not-a-scenario"), Err(SimError::UnknownScenario));
    }
}
