"""Reference workload-load/recovery state model for GAIA 3.0 (#1691)."""
from __future__ import annotations
from dataclasses import dataclass
from enum import Enum

class LoadState(str, Enum):
    CONTINUE="CONTINUE"; SLOW="SLOW"; PAUSE="PAUSE"; STOP="STOP"; RECOVER="RECOVER"; RESUME="RESUME"; UNKNOWN="UNKNOWN"

@dataclass(frozen=True)
class LoadIndicators:
    duration: float=0.0
    error_rate: float=0.0
    contradiction_rate: float=0.0
    interruption_rate: float=0.0
    latency_ratio: float=0.0
    resource_pressure: float=0.0
    explicit_stop: bool=False

@dataclass(frozen=True)
class LoadThresholds:
    slow: float=0.35
    pause: float=0.55
    stop: float=0.80
    uncertainty_margin: float=0.05

@dataclass(frozen=True)
class LoadAssessment:
    state: LoadState
    score: float|None
    uncertainty: float
    reason: str

def assess(indicators: LoadIndicators, thresholds: LoadThresholds=LoadThresholds()) -> LoadAssessment:
    if indicators.explicit_stop:
        return LoadAssessment(LoadState.STOP,1.0,0.0,"explicit_stop")
    values=(indicators.duration,indicators.error_rate,indicators.contradiction_rate,
            indicators.interruption_rate,indicators.latency_ratio,indicators.resource_pressure)
    if any(value<0 or value>1 for value in values):
        return LoadAssessment(LoadState.UNKNOWN,None,1.0,"indicator_out_of_range")
    score=sum(values)/len(values)
    uncertainty=min(1.0,max(values)-min(values))
    if uncertainty>thresholds.uncertainty_margin and score<thresholds.stop:
        return LoadAssessment(LoadState.UNKNOWN,score,uncertainty,"conflicting_indicators")
    if score>=thresholds.stop: return LoadAssessment(LoadState.STOP,score,uncertainty,"high_load")
    if score>=thresholds.pause: return LoadAssessment(LoadState.PAUSE,score,uncertainty,"sustained_degradation")
    if score>=thresholds.slow: return LoadAssessment(LoadState.SLOW,score,uncertainty,"rising_load")
    return LoadAssessment(LoadState.CONTINUE,score,uncertainty,"stable_workload")

LEGAL_TRANSITIONS={
 LoadState.CONTINUE:frozenset({LoadState.SLOW,LoadState.PAUSE,LoadState.STOP,LoadState.UNKNOWN}),
 LoadState.SLOW:frozenset({LoadState.CONTINUE,LoadState.PAUSE,LoadState.STOP,LoadState.UNKNOWN}),
 LoadState.PAUSE:frozenset({LoadState.RECOVER,LoadState.STOP,LoadState.UNKNOWN}),
 LoadState.STOP:frozenset({LoadState.RECOVER,LoadState.UNKNOWN}),
 LoadState.RECOVER:frozenset({LoadState.RESUME,LoadState.STOP,LoadState.UNKNOWN}),
 LoadState.RESUME:frozenset({LoadState.CONTINUE,LoadState.SLOW,LoadState.PAUSE,LoadState.UNKNOWN}),
 LoadState.UNKNOWN:frozenset({LoadState.SLOW,LoadState.PAUSE,LoadState.STOP,LoadState.RECOVER}),
}

def transition(current:LoadState, requested:LoadState)->LoadState:
    if requested not in LEGAL_TRANSITIONS[current]:
        raise ValueError(f"illegal workload transition: {current.value} -> {requested.value}")
    return requested

def continuity_scope(before:str, after:str)->bool:
    return before==after
