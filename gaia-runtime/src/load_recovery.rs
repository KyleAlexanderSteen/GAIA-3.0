//! Runtime load/recovery integration for #1691.
//!
//! This module adapts measured runtime/resource telemetry into the #1691
//! workload-load contract. It is deliberately a reference integration:
//! thresholds are configuration, not universal truths, and no state transition
//! grants authority or silently expands authorization scope.

use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadState {
    Continue,
    Slow,
    Pause,
    Stop,
    Recover,
    Resume,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LoadThresholds {
    pub slow: f64,
    pub pause: f64,
    pub stop: f64,
    pub uncertainty_margin: f64,
}

impl Default for LoadThresholds {
    fn default() -> Self {
        Self {
            slow: 0.35,
            pause: 0.55,
            stop: 0.80,
            uncertainty_margin: 0.05,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LoadIndicators {
    pub duration: f64,
    pub error_rate: f64,
    pub contradiction_rate: f64,
    pub interruption_rate: f64,
    pub latency_ratio: f64,
    pub resource_pressure: f64,
    pub explicit_stop: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LoadAssessment {
    pub state: LoadState,
    pub score: Option<f64>,
    pub uncertainty: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuntimeLoadTelemetry {
    pub elapsed: Duration,
    pub duration_budget: Duration,
    pub cpu_millis_used: u64,
    pub cpu_millis_budget: u64,
    pub memory_bytes_used: usize,
    pub memory_bytes_budget: usize,
    pub operations: u64,
    pub failures: u64,
    pub contradictions: u64,
    pub interruptions: u64,
    pub explicit_stop: bool,
}

impl RuntimeLoadTelemetry {
    /// Convert measured runtime/resource telemetry into normalized #1691
    /// indicators. Zero-operation samples remain UNKNOWN rather than becoming
    /// an apparently healthy workload.
    pub fn indicators(self) -> LoadIndicators {
        let ratio = |numerator: u64, denominator: u64| {
            if denominator == 0 {
                1.0
            } else {
                (numerator as f64 / denominator as f64).clamp(0.0, 1.0)
            }
        };
        let duration = ratio(
            self.elapsed.as_millis() as u64,
            self.duration_budget.as_millis() as u64,
        );
        let latency_ratio = duration;
        let error_rate = ratio(self.failures, self.operations);
        let contradiction_rate = ratio(self.contradictions, self.operations);
        let interruption_rate = ratio(self.interruptions, self.operations);
        let cpu_pressure = ratio(self.cpu_millis_used, self.cpu_millis_budget);
        let memory_pressure = if self.memory_bytes_budget == 0 {
            1.0
        } else {
            (self.memory_bytes_used as f64 / self.memory_bytes_budget as f64).clamp(0.0, 1.0)
        };

        LoadIndicators {
            duration,
            error_rate,
            contradiction_rate,
            interruption_rate,
            latency_ratio,
            resource_pressure: cpu_pressure.max(memory_pressure),
            explicit_stop: self.explicit_stop,
        }
    }

    pub fn assess(self, thresholds: LoadThresholds) -> LoadAssessment {
        assess(self.indicators(), thresholds)
    }
}

pub fn assess(indicators: LoadIndicators, thresholds: LoadThresholds) -> LoadAssessment {
    if indicators.explicit_stop {
        return LoadAssessment {
            state: LoadState::Stop,
            score: Some(1.0),
            uncertainty: 0.0,
        };
    }

    let values = [
        indicators.duration,
        indicators.error_rate,
        indicators.contradiction_rate,
        indicators.interruption_rate,
        indicators.latency_ratio,
        indicators.resource_pressure,
    ];

    if values.iter().any(|value| !value.is_finite() || !(0.0..=1.0).contains(value)) {
        return LoadAssessment {
            state: LoadState::Unknown,
            score: None,
            uncertainty: 1.0,
        };
    }

    let score = values.iter().sum::<f64>() / values.len() as f64;
    let min = values.iter().copied().fold(f64::INFINITY, f64::min);
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let uncertainty = max - min;

    // A spread by itself is not conflicting evidence: zero error or
    // interruption rates are healthy observations, not disagreement with
    // duration/resource pressure. Preserve UNKNOWN only when a material
    // high-load signal is directly opposed by a low-load signal.
    let materially_conflicting = max >= thresholds.pause
        && min <= thresholds.uncertainty_margin
        && uncertainty > thresholds.uncertainty_margin;

    if materially_conflicting && score < thresholds.stop {
        return LoadAssessment {
            state: LoadState::Unknown,
            score: Some(score),
            uncertainty,
        };
    }

    let state = if score >= thresholds.stop {
        LoadState::Stop
    } else if score >= thresholds.pause {
        LoadState::Pause
    } else if score >= thresholds.slow {
        LoadState::Slow
    } else {
        LoadState::Continue
    };

    LoadAssessment {
        state,
        score: Some(score),
        uncertainty,
    }
}

pub fn transition(current: LoadState, requested: LoadState) -> Result<LoadState, &'static str> {
    let legal = match current {
        LoadState::Continue => matches!(
            requested,
            LoadState::Slow | LoadState::Pause | LoadState::Stop | LoadState::Unknown
        ),
        LoadState::Slow => matches!(
            requested,
            LoadState::Continue | LoadState::Pause | LoadState::Stop | LoadState::Unknown
        ),
        LoadState::Pause => matches!(
            requested,
            LoadState::Recover | LoadState::Stop | LoadState::Unknown
        ),
        LoadState::Stop => matches!(requested, LoadState::Recover | LoadState::Unknown),
        LoadState::Recover => matches!(
            requested,
            LoadState::Resume | LoadState::Stop | LoadState::Unknown
        ),
        LoadState::Resume => matches!(
            requested,
            LoadState::Continue | LoadState::Slow | LoadState::Pause | LoadState::Unknown
        ),
        LoadState::Unknown => matches!(
            requested,
            LoadState::Slow | LoadState::Pause | LoadState::Stop | LoadState::Recover
        ),
    };

    if legal {
        Ok(requested)
    } else {
        Err("illegal workload transition")
    }
}

pub fn continuity_scope_preserved(before: &str, after: &str) -> bool {
    before == after
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepresentativeWorkload {
    Stable,
    RisingLoad,
    SustainedDegradation,
    HighLoad,
    ConflictingEvidence,
    ExplicitStop,
}

impl RepresentativeWorkload {
    pub fn telemetry(self) -> RuntimeLoadTelemetry {
        let base = |ratio: u64| RuntimeLoadTelemetry {
            elapsed: Duration::from_millis(ratio),
            duration_budget: Duration::from_millis(100),
            cpu_millis_used: ratio,
            cpu_millis_budget: 100,
            memory_bytes_used: ratio as usize,
            memory_bytes_budget: 100,
            operations: 100,
            failures: 0,
            contradictions: 0,
            interruptions: 0,
            explicit_stop: false,
        };

        match self {
            Self::Stable => base(10),
            Self::RisingLoad => base(40),
            Self::SustainedDegradation => base(60),
            Self::HighLoad => base(90),
            Self::ConflictingEvidence => RuntimeLoadTelemetry {
                elapsed: Duration::from_millis(100),
                duration_budget: Duration::from_millis(100),
                cpu_millis_used: 0,
                cpu_millis_budget: 100,
                memory_bytes_used: 0,
                memory_bytes_budget: 100,
                operations: 100,
                failures: 0,
                contradictions: 0,
                interruptions: 0,
                explicit_stop: false,
            },
            Self::ExplicitStop => RuntimeLoadTelemetry {
                elapsed: Duration::from_millis(10),
                duration_budget: Duration::from_millis(100),
                cpu_millis_used: 10,
                cpu_millis_budget: 100,
                memory_bytes_used: 10,
                memory_bytes_budget: 100,
                operations: 100,
                failures: 0,
                contradictions: 0,
                interruptions: 0,
                explicit_stop: true,
            },
        }
    }

    pub fn expected_state(self) -> LoadState {
        match self {
            Self::Stable => LoadState::Continue,
            Self::RisingLoad => LoadState::Slow,
            Self::SustainedDegradation => LoadState::Pause,
            Self::HighLoad => LoadState::Stop,
            Self::ConflictingEvidence => LoadState::Unknown,
            Self::ExplicitStop => LoadState::Stop,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn evaluate(workload: RepresentativeWorkload) -> LoadAssessment {
        workload.telemetry().assess(LoadThresholds::default())
    }

    #[test]
    fn runtime_telemetry_maps_to_representative_states() {
        for workload in [
            RepresentativeWorkload::Stable,
            RepresentativeWorkload::RisingLoad,
            RepresentativeWorkload::SustainedDegradation,
            RepresentativeWorkload::HighLoad,
            RepresentativeWorkload::ExplicitStop,
        ] {
            assert_eq!(evaluate(workload).state, workload.expected_state());
        }
    }

    #[test]
    fn conflicting_runtime_evidence_preserves_unknown() {
        let telemetry = RuntimeLoadTelemetry {
            elapsed: Duration::from_millis(100),
            duration_budget: Duration::from_millis(100),
            cpu_millis_used: 0,
            cpu_millis_budget: 100,
            memory_bytes_used: 0,
            memory_bytes_budget: 100,
            operations: 100,
            failures: 0,
            contradictions: 0,
            interruptions: 0,
            explicit_stop: false,
        };
        assert_eq!(
            telemetry.assess(LoadThresholds::default()).state,
            LoadState::Unknown
        );
    }

    #[test]
    fn zero_operation_sample_does_not_look_healthy() {
        let telemetry = RuntimeLoadTelemetry {
            elapsed: Duration::from_millis(0),
            duration_budget: Duration::from_millis(100),
            cpu_millis_used: 0,
            cpu_millis_budget: 100,
            memory_bytes_used: 0,
            memory_bytes_budget: 100,
            operations: 0,
            failures: 0,
            contradictions: 0,
            interruptions: 0,
            explicit_stop: false,
        };
        assert_ne!(telemetry.assess(LoadThresholds::default()).state, LoadState::Continue);
    }

    #[test]
    fn invalid_transition_is_rejected() {
        assert!(transition(LoadState::Continue, LoadState::Resume).is_err());
    }

    #[test]
    fn recovery_does_not_expand_scope() {
        assert!(continuity_scope_preserved("agent:default", "agent:default"));
        assert!(!continuity_scope_preserved("agent:default", "agent:expanded"));
    }

    #[test]
    fn fp_fn_measurement_is_zero_for_reference_workloads() {
        let workloads = [
            RepresentativeWorkload::Stable,
            RepresentativeWorkload::RisingLoad,
            RepresentativeWorkload::SustainedDegradation,
            RepresentativeWorkload::HighLoad,
            RepresentativeWorkload::ExplicitStop,
        ];
        let mut false_positive = 0;
        let mut false_negative = 0;

        for workload in workloads {
            let actual = evaluate(workload).state;
            let expected = workload.expected_state();
            if actual == LoadState::Stop && expected != LoadState::Stop {
                false_positive += 1;
            }
            if actual != LoadState::Stop && expected == LoadState::Stop {
                false_negative += 1;
            }
        }

        assert_eq!(false_positive, 0);
        assert_eq!(false_negative, 0);
    }

    #[test]
    fn bounded_runtime_workload_produces_measured_telemetry() {
        use std::hint::black_box;
        use std::time::Instant;

        let started = Instant::now();
        let mut bytes = Vec::with_capacity(4096);
        for value in 0u8..=255 {
            bytes.push(value);
        }
        black_box(&bytes);
        let elapsed = started.elapsed();

        let telemetry = RuntimeLoadTelemetry {
            elapsed,
            duration_budget: Duration::from_secs(1),
            cpu_millis_used: elapsed.as_millis() as u64,
            cpu_millis_budget: 1_000,
            memory_bytes_used: bytes.len(),
            memory_bytes_budget: 64 * 1024,
            operations: 256,
            failures: 0,
            contradictions: 0,
            interruptions: 0,
            explicit_stop: false,
        };

        let assessment = telemetry.assess(LoadThresholds::default());
        assert!(elapsed <= Duration::from_secs(1));
        assert_eq!(assessment.state, LoadState::Continue);
        assert!(assessment.score.is_some());
    }

    #[test]
    fn thresholds_are_configuration_not_global_truth() {
        let workload = RepresentativeWorkload::RisingLoad.telemetry();
        let conservative = LoadThresholds {
            slow: 0.20,
            pause: 0.35,
            stop: 0.70,
            uncertainty_margin: 0.05,
        };
        assert_eq!(
            workload.assess(conservative).state,
            LoadState::Pause
        );
        assert_eq!(
            workload.assess(LoadThresholds::default()).state,
            LoadState::Slow
        );
    }
}
