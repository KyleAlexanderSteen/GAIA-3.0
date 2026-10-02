from __future__ import annotations

from .errors import InvalidArgument, NotImplementedCapability
from .types import AgentSpec, MemCube, ResourceSpec, TaskHandle, new_id


class GaiaClient:
    """Local stub client. No kernel transport in Phase 0."""

    def intent(self, goal: str) -> TaskHandle:
        if not goal or not goal.strip():
            raise InvalidArgument("goal must not be empty")
        raise NotImplementedCapability("intent has no kernel transport; nothing was admitted")

    def context(self, query: str) -> MemCube:
        if not query or not str(query).strip():
            raise InvalidArgument("query must not be empty")
        raise NotImplementedCapability("context has no MemOS transport; nothing was stored")

    def invoke(self, agent: AgentSpec) -> str:
        if not agent.agent_id:
            raise InvalidArgument("agent_id required")
        raise NotImplementedCapability(f"invoke has no registered agent: {agent.name}")

    def observe(self, sensor: str) -> str:
        if not sensor:
            raise InvalidArgument("sensor required")
        raise NotImplementedCapability(f"observe has no sensor bus: {sensor}")

    def sign(self, payload: bytes) -> bytes:
        if not payload:
            raise InvalidArgument("payload required")
        raise NotImplementedCapability("Ed25519 signing lands with Phase 1 (#14/#19)")

    def verify(self, payload: bytes, signature: bytes) -> bool:
        if not payload or not signature:
            raise InvalidArgument("payload and signature required")
        raise NotImplementedCapability("Ed25519 verify lands with Phase 1 (#14/#19)")

    def declare(self, resource: ResourceSpec) -> str:
        if not resource.name:
            raise InvalidArgument("resource name required")
        raise NotImplementedCapability(f"declare has no resource registry: {resource.name}")
