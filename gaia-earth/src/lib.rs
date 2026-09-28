//! Artificial Twin of Earth first cuts (#33–#55 replay).
//! Not Iceberg, Cesium, GraphCast weights, or Twin v1.0.

mod audiences;
mod commons;
mod correlate;
mod drill;
mod ensemble;
mod ews;
mod feeds;
mod guardian;
pub mod honesty;
mod iface;
mod lake;
mod library;
mod living;
mod memory;
mod missions;
mod nervous;
mod policy;
mod portal;
mod profiles;
mod qc;
mod runtime;
mod simulate;
mod stream;
mod tiers;

pub use audiences::{surveillance_endpoints, AgentAnswer, LocalAgent, PolicyUi, ScientistApi};
pub use commons::{Collection, Commons, Domain, QualityTier};
pub use correlate::{BoundaryIndicator, CaseStudy};
pub use drill::{release_checklist, DrillStep, HistoricalDrill};
pub use ensemble::{ModelProduct, RegisteredModel};
pub use ews::{
    compute_ews, BoundaryMonitor, EarlyWarningDetector, EwsAmbiguity, EwsScore, WatchItem,
    WatchState,
};
pub use feeds::{Connector, FeedBus, FeedRecord, RateLimit};
pub use guardian::{Boundary, BoundaryState, BoundaryStatus, Correction, CorrectionStep, Guardian};
pub use honesty::{live_ews_network, twin_v1_tagged};
pub use iface::{AccessTier, CitizenCredit, EarthInterface, PlaceState};
pub use lake::{Lake, LakeRow, LakeZone};
pub use library::{CascadeEdge, Dist, ScenarioLibrary, ScenarioSpec};
pub use living::{LivingKind, LivingRecord};
pub use memory::{CubeSet, MemStoreParams, ModelOutput, PlaceTime, PlanetaryMemCube, PlanetaryMemory};
pub use missions::{CoverageGap, Mission, MissionCatalog, MissionDomain};
pub use nervous::{FeedKind, IngestParams, NervousFabric, QcTier, Sample};
pub use policy::{IngestTicket, LicenseClass};
pub use portal::{demo_globe, GlobeLayer, PortalPin, TimeCursor};
pub use profiles::{ProfileGap, SystemProfile};
pub use qc::{assimilate, CuratedRecord, QualityClass};
pub use runtime::{GridScale, SimJob, SimRun};
pub use simulate::{
    fixture_ensemble, ConfidenceBand, Distribution, HorizonFlag, OutcomeKind, ScenarioEngine,
    ScenarioRun, SimError, SimMode, TrajectoryEnsemble, TrajectoryMember,
};
pub use stream::{DeadLetter, StreamBus, StreamEvent, StreamMetrics};
pub use tiers::{MemoryTier, TierCube, TierStore};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceKind {
    Measured,
    Synthetic,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemTwin {
    Atmosphere,
    Ocean,
    Land,
    Biosphere,
    Cryosphere,
    Lithosphere,
    Anthroposphere,
    Technosphere,
    Magnetosphere,
}

impl SystemTwin {
    pub fn all() -> [SystemTwin; 9] {
        [
            Self::Atmosphere,
            Self::Ocean,
            Self::Land,
            Self::Biosphere,
            Self::Cryosphere,
            Self::Lithosphere,
            Self::Anthroposphere,
            Self::Technosphere,
            Self::Magnetosphere,
        ]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Observation {
    pub system: SystemTwin,
    pub source: SourceKind,
    pub value: f64,
    pub uncertainty: f64,
    pub unit: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TwinError {
    MissingUncertainty,
    UnlabeledPoint,
    WeaponizedUse,
    UnknownCollection,
    SparseRegion,
    ImmutableRaw,
    FeedUnavailable,
}

impl std::fmt::Display for TwinError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingUncertainty => write!(f, "uncertainty is mandatory"),
            Self::UnlabeledPoint => write!(f, "point must be measured or labeled synthetic"),
            Self::WeaponizedUse => write!(f, "weaponization is refused"),
            Self::UnknownCollection => write!(f, "unknown collection"),
            Self::SparseRegion => write!(f, "sparse region flagged; value not invented"),
            Self::ImmutableRaw => write!(f, "raw zone is append-only"),
            Self::FeedUnavailable => write!(f, "feed unavailable; value not invented"),
        }
    }
}

impl Observation {
    pub fn admit(
        system: SystemTwin,
        source: SourceKind,
        value: f64,
        uncertainty: Option<f64>,
        unit: &str,
    ) -> Result<Self, TwinError> {
        let uncertainty = uncertainty.ok_or(TwinError::MissingUncertainty)?;
        if uncertainty < 0.0 {
            return Err(TwinError::MissingUncertainty);
        }
        if unit.trim().is_empty() {
            return Err(TwinError::UnlabeledPoint);
        }
        Ok(Self {
            system,
            source,
            value,
            uncertainty,
            unit: unit.into(),
        })
    }
}

pub fn allow_purpose(purpose: &str) -> Result<(), TwinError> {
    let lower = purpose.to_ascii_lowercase();
    if lower.contains("weapon")
        || lower.contains("target individual")
        || lower.contains("surveillance of person")
    {
        return Err(TwinError::WeaponizedUse);
    }
    Ok(())
}
