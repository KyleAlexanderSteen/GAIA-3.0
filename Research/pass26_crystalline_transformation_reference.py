"""Pass 26 reference simulation: crystalline transformation grammar.

This is intentionally deterministic and dependency-free. It is a semantic
model, not a physical crystal simulator.
"""

from dataclasses import dataclass, replace
from math import sqrt
from typing import Dict, List, Tuple


@dataclass(frozen=True)
class Node:
    identity: str
    value: float
    coherence: float = 1.0
    memory: Tuple[float, ...] = ()
    stable: bool = True


@dataclass(frozen=True)
class State:
    nodes: Dict[str, Node]
    phase: str
    step: int

    def coherence(self) -> float:
        if not self.nodes:
            return 0.0
        return sum(n.coherence for n in self.nodes.values()) / len(self.nodes)


def divergence(state: State) -> State:
    nodes = {}
    for key, node in state.nodes.items():
        nodes[key + "_A"] = replace(node, identity=key + "_A", value=node.value - 0.1)
        nodes[key + "_B"] = replace(node, identity=key + "_B", value=node.value + 0.1)
    return State(nodes, "DIVERGENCE", state.step + 1)


def insurgence(state: State, noise: float = 0.05) -> State:
    nodes = {}
    for key, node in state.nodes.items():
        shifted = node.value + (noise if key.endswith("_B") else -noise)
        coherence = max(0.0, node.coherence - abs(noise))
        nodes[key] = replace(node, value=shifted, coherence=coherence)
    return State(nodes, "INSURGENCE", state.step + 1)


def allegiance(state: State) -> State:
    # Preserve identity, history, and safety-relevant state while allowing
    # the value estimate to remain revisable.
    nodes = {
        key: replace(node, memory=node.memory + (node.value,))
        for key, node in state.nodes.items()
    }
    return State(nodes, "ALLEGIANCE", state.step + 1)


def convergence(state: State) -> State:
    # Pair A/B branches by original prefix and average only after recording
    # both observations. This models reconciliation without erasing history.
    grouped: Dict[str, List[Node]] = {}
    for key, node in state.nodes.items():
        base = key.rsplit("_", 1)[0]
        grouped.setdefault(base, []).append(node)

    nodes = {}
    for base, members in grouped.items():
        value = sum(n.value for n in members) / len(members)
        coherence = sum(n.coherence for n in members) / len(members)
        history = tuple(x for n in members for x in n.memory) + (value,)
        nodes[base] = Node(base, value, coherence, history)
    return State(nodes, "CONVERGENCE", state.step + 1)


def ascendence(state: State) -> State:
    # A verified new state: preserve identity and history, improve stability
    # only as a bounded transformation.
    nodes = {
        key: replace(node, stable=True, coherence=min(1.0, node.coherence + 0.05))
        for key, node in state.nodes.items()
    }
    return State(nodes, "ASCENDENCE", state.step + 1)


def avatar_state(state: State) -> State:
    # Higher-order integration, deliberately bounded. No consciousness claim.
    mean = sum(n.value for n in state.nodes.values()) / max(1, len(state.nodes))
    nodes = {
        key: replace(
            node,
            value=(node.value + mean) / 2,
            coherence=min(1.0, node.coherence + 0.02),
            memory=node.memory + (mean,),
        )
        for key, node in state.nodes.items()
    }
    return State(nodes, "AVATAR_STATE", state.step + 1)


def return_state(state: State) -> State:
    # Controlled exit: retain integrated memory while returning to a stable
    # ordinary operating phase.
    return State(state.nodes, "RETURN", state.step + 1)


def coherence_integrity(initial: State, final: State) -> float:
    """Small reference metric, not an IQ or consciousness measure."""
    identity_continuity = len(
        set(initial.nodes).intersection(final.nodes)
    ) / max(1, len(initial.nodes))
    memory_retention = sum(
        bool(n.memory) for n in final.nodes.values()
    ) / max(1, len(final.nodes))
    coherence = final.coherence()
    return (identity_continuity + memory_retention + coherence) / 3


def run() -> State:
    initial = State(
        {"C0": Node("C0", 1.0)},
        "BASELINE",
        0,
    )
    state = divergence(initial)
    state = insurgence(state)
    state = allegiance(state)
    state = convergence(state)
    state = ascendence(state)
    state = avatar_state(state)
    state = return_state(state)
    print("Final phase:", state.phase)
    print("Steps:", state.step)
    print("Coherence:", round(state.coherence(), 4))
    print("CI:", round(coherence_integrity(initial, state), 4))
    return state


if __name__ == "__main__":
    run()
