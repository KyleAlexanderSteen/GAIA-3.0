"""Reference energy-aware state machine for GAIA 3.0 (#1692)."""
from __future__ import annotations
from dataclasses import dataclass
from enum import Enum

class EnergyState(str, Enum):
    FULL="FULL"; ACTIVE="ACTIVE"; READY="READY"; LOW_POWER="LOW-POWER"
    SLEEP="SLEEP"; WAKE_CONDITION="WAKE CONDITION"; WAKE="WAKE"; RECOVERY="RECOVERY"

LEGAL_TRANSITIONS={
 EnergyState.FULL:frozenset({EnergyState.ACTIVE,EnergyState.READY}),
 EnergyState.ACTIVE:frozenset({EnergyState.FULL,EnergyState.READY,EnergyState.LOW_POWER,EnergyState.SLEEP,EnergyState.RECOVERY}),
 EnergyState.READY:frozenset({EnergyState.ACTIVE,EnergyState.LOW_POWER,EnergyState.RECOVERY}),
 EnergyState.LOW_POWER:frozenset({EnergyState.READY,EnergyState.SLEEP,EnergyState.RECOVERY}),
 EnergyState.SLEEP:frozenset({EnergyState.WAKE_CONDITION,EnergyState.RECOVERY}),
 EnergyState.WAKE_CONDITION:frozenset({EnergyState.WAKE,EnergyState.RECOVERY}),
 EnergyState.WAKE:frozenset({EnergyState.ACTIVE,EnergyState.RECOVERY}),
 EnergyState.RECOVERY:frozenset({EnergyState.ACTIVE,EnergyState.READY,EnergyState.SLEEP,EnergyState.RECOVERY}),
}

@dataclass(frozen=True)
class AuthorizationSnapshot:
    scope:str
    version:str

@dataclass(frozen=True)
class WakeEvent:
    kind:str
    authorized:bool
    evidence:str
    policy_version:str

@dataclass(frozen=True)
class TransitionRecord:
    previous:EnergyState
    requested:EnergyState
    resulting:EnergyState
    reason:str
    evidence:str
    authorization:AuthorizationSnapshot
    integrity_ok:bool

@dataclass(frozen=True)
class ResourceMetrics:
    state:EnergyState
    energy_units:float
    latency_ms:float
    responsiveness:float
    integrity_ok:bool
    recovery_ok:bool

def transition(current:EnergyState, requested:EnergyState)->EnergyState:
    if requested not in LEGAL_TRANSITIONS[current]:
        raise ValueError(f"illegal energy transition: {current.value} -> {requested.value}")
    return requested

def validate_wake(event:WakeEvent, authorization:AuthorizationSnapshot)->bool:
    return event.authorized and bool(event.evidence) and bool(event.policy_version) and bool(authorization.scope)

def validate_recovery(before:AuthorizationSnapshot, after:AuthorizationSnapshot, integrity_ok:bool)->bool:
    return integrity_ok and before==after

def record_transition(current,requested,reason,evidence,authorization,integrity_ok):
    resulting=transition(current,requested)
    return TransitionRecord(current,requested,resulting,reason,evidence,authorization,integrity_ok)
