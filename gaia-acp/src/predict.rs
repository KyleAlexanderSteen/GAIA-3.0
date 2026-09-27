//! #1129 Predict-5 honesty band. Not a forecast engine.

use crate::failure_harness::HarnessOutcome;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ForecastKind {
    ListedEnvelope,
    OutOfDistribution,
    NamedProphecy,
    DestinEIngest,
    TrueBlackSwan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HonestyBand {
    pub outcome: HarnessOutcome,
    pub cannot: &'static str,
}

pub fn honesty(kind: ForecastKind) -> HonestyBand {
    match kind {
        ForecastKind::ListedEnvelope => HonestyBand {
            outcome: HarnessOutcome::NeedVerify,
            cannot: "no live Earth or social ingest",
        },
        ForecastKind::OutOfDistribution => HonestyBand {
            outcome: HarnessOutcome::NeedVerify,
            cannot: "out of distribution; treat outputs as low confidence",
        },
        ForecastKind::NamedProphecy => HonestyBand {
            outcome: HarnessOutcome::OutOfScope,
            cannot: "cannot predict the future",
        },
        ForecastKind::DestinEIngest => HonestyBand {
            outcome: HarnessOutcome::OutOfScope,
            cannot: "DestinE / GraphCast ingest refused",
        },
        ForecastKind::TrueBlackSwan => HonestyBand {
            outcome: HarnessOutcome::OutOfScope,
            cannot: "true black swans are undefined by construction",
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prophecy_is_out_of_scope() {
        let h = honesty(ForecastKind::NamedProphecy);
        assert_eq!(h.outcome, HarnessOutcome::OutOfScope);
    }

    #[test]
    fn ood_is_need_verify() {
        let h = honesty(ForecastKind::OutOfDistribution);
        assert_eq!(h.outcome, HarnessOutcome::NeedVerify);
    }

    #[test]
    fn destine_is_out_of_scope() {
        let h = honesty(ForecastKind::DestinEIngest);
        assert_eq!(h.outcome, HarnessOutcome::OutOfScope);
    }
}
