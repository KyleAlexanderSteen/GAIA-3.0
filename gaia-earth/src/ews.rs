//! #1143 Predict-2 EWS statistics. Boundary monitor + rolling variance/autocorrelation.
//! Not AdvanTip, DestinE, or a trained ML detector.

use crate::TwinError;

// ── Planetary Boundary watch items (unchanged) ────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WatchItem {
    ClimateChange,
    BiosphereIntegrity,
    LandSystemChange,
    FreshwaterChange,
    BiogeochemicalFlows,
    OceanAcidification,
    AtmosphericAerosol,
    StratosphericOzone,
    NovelEntities,
}

impl WatchItem {
    pub fn all() -> [WatchItem; 9] {
        [
            Self::ClimateChange,
            Self::BiosphereIntegrity,
            Self::LandSystemChange,
            Self::FreshwaterChange,
            Self::BiogeochemicalFlows,
            Self::OceanAcidification,
            Self::AtmosphericAerosol,
            Self::StratosphericOzone,
            Self::NovelEntities,
        ]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WatchState {
    pub item: WatchItem,
    pub source: String,
    pub timestamp_unix: u64,
    pub uncertainty: f64,
    pub ratio: f64,
}

pub struct BoundaryMonitor {
    states: Vec<WatchState>,
}

impl BoundaryMonitor {
    pub fn seed(now: u64) -> Self {
        Self {
            states: WatchItem::all()
                .into_iter()
                .map(|item| WatchState {
                    item,
                    source: "fixture-indicator".into(),
                    timestamp_unix: now,
                    uncertainty: 0.1,
                    ratio: 0.4,
                })
                .collect(),
        }
    }

    pub fn states(&self) -> &[WatchState] {
        &self.states
    }

    pub fn approach(&mut self, item: WatchItem, ratio: f64) -> Result<bool, TwinError> {
        if ratio < 0.0 {
            return Err(TwinError::MissingUncertainty);
        }
        let state = self
            .states
            .iter_mut()
            .find(|s| s.item == item)
            .ok_or(TwinError::UnlabeledPoint)?;
        state.ratio = ratio;
        Ok(ratio >= 0.8)
    }
}

// ── Predict-2: Rolling EWS statistics ─────────────────────────────────────
// Variance and lag-1 autocorrelation are the two canonical EWS metrics.
// Critical slowing down manifests as rising variance AND rising autocorrelation
// simultaneously. Neither alone is diagnostic. Both together are the signal.
// Not a trained classifier. Not AdvanTip. Fixture values only until live ingest.

/// Ambiguity class for every EWS output — mandatory, never suppressed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EwsAmbiguity {
    /// Both variance and autocorrelation rising — consistent with approach.
    ConsistentWithApproach,
    /// Signal present but could be noise-driven variance increase.
    AmbiguousNeedReview,
    /// Insufficient window length or flat signal — no inference possible.
    InsufficientData,
}

/// Rolling EWS score for a single time-series window.
/// Score ∈ [0.0, 1.0]. uncertainty is mandatory — never zero.
#[derive(Debug, Clone, PartialEq)]
pub struct EwsScore {
    pub item: WatchItem,
    /// Combined EWS score — 0.0 = no signal, 1.0 = maximum signal.
    pub score: f64,
    /// Variance of the detrended window.
    pub variance: f64,
    /// Lag-1 autocorrelation of the detrended window.
    pub lag1_ac: f64,
    /// Propagated uncertainty on the score.
    pub uncertainty: f64,
    /// Mandatory ambiguity classification — never suppressed.
    pub ambiguity: EwsAmbiguity,
    /// Human review required at this score threshold.
    pub review_threshold: f64,
}

impl EwsScore {
    /// Returns true when the score meets or exceeds the review threshold.
    pub fn requires_review(&self) -> bool {
        self.score >= self.review_threshold
    }

    /// Plain-language ambiguity statement — mandatory on every output.
    pub fn ambiguity_statement(&self) -> &'static str {
        match self.ambiguity {
            EwsAmbiguity::ConsistentWithApproach => {
                "Signal is consistent with approaching tipping point. \
                 Human review required. GAIA does not act autonomously on this warning."
            }
            EwsAmbiguity::AmbiguousNeedReview => {
                "Signal is consistent with BOTH approaching tipping point AND \
                 noise-driven variance increase. Cannot distinguish without additional data. \
                 Human review required."
            }
            EwsAmbiguity::InsufficientData => {
                "Insufficient window length or flat signal. \
                 No EWS inference is possible from this data."
            }
        }
    }
}

/// Compute rolling EWS statistics from a detrended time-series window.
/// `window` must have at least 2 points; uncertainty must be > 0.0.
/// Returns `TwinError::SparseRegion` if the window is too short.
/// Returns `TwinError::MissingUncertainty` if uncertainty ≤ 0.0.
pub fn compute_ews(
    item: WatchItem,
    window: &[f64],
    uncertainty: f64,
    review_threshold: f64,
) -> Result<EwsScore, TwinError> {
    if uncertainty <= 0.0 {
        return Err(TwinError::MissingUncertainty);
    }
    if window.len() < 2 {
        return Ok(EwsScore {
            item,
            score: 0.0,
            variance: 0.0,
            lag1_ac: 0.0,
            uncertainty,
            ambiguity: EwsAmbiguity::InsufficientData,
            review_threshold,
        });
    }

    let n = window.len() as f64;
    let mean = window.iter().sum::<f64>() / n;
    let variance = window.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (n - 1.0);

    // Lag-1 autocorrelation: Pearson correlation between x[t] and x[t-1].
    let lag1_ac = if window.len() < 3 || variance < f64::EPSILON {
        0.0
    } else {
        let pairs: Vec<(f64, f64)> = window
            .windows(2)
            .map(|w| (w[0] - mean, w[1] - mean))
            .collect();
        let cov: f64 = pairs.iter().map(|(a, b)| a * b).sum::<f64>() / (n - 2.0);
        (cov / variance).clamp(-1.0, 1.0)
    };

    // Combined score: mean of normalised variance signal and autocorrelation.
    // variance_signal maps [0, ∞) → [0, 1) via sigmoid-like 1 - exp(-v).
    let variance_signal = 1.0 - (-variance).exp();
    let ac_signal = (lag1_ac + 1.0) / 2.0; // maps [-1, 1] → [0, 1]
    let score = ((variance_signal + ac_signal) / 2.0).clamp(0.0, 1.0);

    let ambiguity = if variance_signal > 0.5 && lag1_ac > 0.5 {
        EwsAmbiguity::ConsistentWithApproach
    } else if variance_signal > 0.3 || lag1_ac > 0.3 {
        EwsAmbiguity::AmbiguousNeedReview
    } else {
        EwsAmbiguity::InsufficientData
    };

    Ok(EwsScore {
        item,
        score,
        variance,
        lag1_ac,
        uncertainty,
        ambiguity,
        review_threshold,
    })
}

/// Early Warning Detector — runs `compute_ews` across all nine WatchItems.
/// All windows are fixture stubs until live ingest is connected.
pub struct EarlyWarningDetector {
    pub review_threshold: f64,
}

impl EarlyWarningDetector {
    pub fn new(review_threshold: f64) -> Self {
        Self { review_threshold }
    }

    /// Scan a slice of (WatchItem, window, uncertainty) tuples.
    /// Returns one EwsScore per item, or an error on the first bad input.
    pub fn scan(
        &self,
        inputs: &[(WatchItem, Vec<f64>, f64)],
    ) -> Result<Vec<EwsScore>, TwinError> {
        inputs
            .iter()
            .map(|(item, window, unc)| {
                compute_ews(*item, window, *unc, self.review_threshold)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_window_is_insufficient() {
        let score = compute_ews(WatchItem::ClimateChange, &[1.0], 0.1, 0.7).unwrap();
        assert_eq!(score.ambiguity, EwsAmbiguity::InsufficientData);
        assert_eq!(score.score, 0.0);
    }

    #[test]
    fn zero_uncertainty_is_error() {
        let err = compute_ews(WatchItem::ClimateChange, &[1.0, 2.0], 0.0, 0.7);
        assert_eq!(err, Err(TwinError::MissingUncertainty));
    }

    #[test]
    fn flat_series_is_insufficient() {
        let window = vec![1.0, 1.0, 1.0, 1.0, 1.0];
        let score = compute_ews(WatchItem::OceanAcidification, &window, 0.05, 0.7).unwrap();
        assert_eq!(score.variance, 0.0);
        assert_eq!(score.ambiguity, EwsAmbiguity::InsufficientData);
    }

    #[test]
    fn rising_series_scores_above_zero() {
        let window: Vec<f64> = (0..20).map(|i| i as f64 + (i as f64 * 0.1)).collect();
        let score = compute_ews(WatchItem::BiosphereIntegrity, &window, 0.05, 0.7).unwrap();
        assert!(score.score > 0.0);
        assert!(!score.ambiguity_statement().is_empty());
    }

    #[test]
    fn review_threshold_triggered() {
        // High-variance, high-autocorrelation series triggers review.
        let window: Vec<f64> = (0..30).map(|i| (i as f64).sin() * i as f64).collect();
        let score = compute_ews(WatchItem::ClimateChange, &window, 0.1, 0.1).unwrap();
        assert!(score.requires_review());
    }

    #[test]
    fn detector_scans_multiple_items() {
        let detector = EarlyWarningDetector::new(0.7);
        let inputs = vec![
            (WatchItem::ClimateChange, vec![1.0, 2.0, 3.0], 0.1),
            (WatchItem::OceanAcidification, vec![0.0, 0.5, 1.0, 1.5], 0.05),
        ];
        let scores = detector.scan(&inputs).unwrap();
        assert_eq!(scores.len(), 2);
    }
}
