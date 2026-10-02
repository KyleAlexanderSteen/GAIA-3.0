//! OTel metric stubs for the 12-stage execution pipeline.
//!
//! These types record durations only. They are not OpenTelemetry.
//! Trace context lives in `trace.rs` and refuses OTLP export. #734.  All call-sites
//! in `engine.rs` compile and behave correctly today; swapping the bodies
//! for real OTel calls later requires no API changes.
//!
//! Metrics emitted per intent execution:
//! - `gaia_intent_total{status="success|failure"}`          counter
//! - `gaia_intent_duration_ms`                              histogram  (p99 < 2000 ms)
//! - `gaia_stage_duration_ms{stage="1".."12"}`              per-stage latency
//! - `gaia_policy_denials_total{agent, capability}`         counter
//! - `gaia_replan_total{reason}`                            counter
//! - `gaia_audit_write_latency_ms`                          histogram  (p99 < 100 ms)

use std::time::Instant;

/// Lightweight span that records wall-clock duration for a single stage.
pub struct StageSpan {
    pub stage: u8,
    start: Instant,
}

impl StageSpan {
    pub fn begin(stage: u8) -> Self {
        Self { stage, start: Instant::now() }
    }

    /// Returns elapsed milliseconds and "emits" the metric (no-op stub).
    pub fn finish(self) -> u64 {
        let ms = self.start.elapsed().as_millis() as u64;
        // TODO(#734): otel_histogram!("gaia_stage_duration_ms", ms, stage = self.stage);
        ms
    }
}

/// Per-intent telemetry accumulator.
#[derive(Default)]
pub struct IntentSpan {
    start: Option<Instant>,
    pub stage_ms: [u64; 12],
}

impl IntentSpan {
    pub fn new() -> Self {
        Self {
            start: Some(Instant::now()),
            stage_ms: [0u64; 12],
        }
    }

    pub fn record_stage(&mut self, stage: u8, ms: u64) {
        if (1..=12).contains(&stage) {
            self.stage_ms[(stage - 1) as usize] = ms;
        }
    }

    /// Returns total intent duration in ms and "emits" the final counters.
    pub fn finish(self, success: bool) -> u64 {
        let total = self
            .start
            .map(|s| s.elapsed().as_millis() as u64)
            .unwrap_or(0);
        // TODO(#734): otel_counter!("gaia_intent_total", status = if success { "success" } else { "failure" });
        // TODO(#734): otel_histogram!("gaia_intent_duration_ms", total);
        let _ = success;
        total
    }
}
