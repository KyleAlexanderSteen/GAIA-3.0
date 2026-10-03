import json
import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT))

from verification.simulation_gate import run_scenario, stable_event_stream


class SimulationGateTests(unittest.TestCase):
    def test_event_order_is_replayable(self):
        events = [{"kind": "noop", "name": str(i)} for i in range(8)]
        self.assertEqual(stable_event_stream(events, 42), stable_event_stream(events, 42))

    def test_refused_action_invariant_passes(self):
        scenario = {"id": "safe", "seed": 3, "events": [{"kind": "execute", "name": "x", "authorized": False}], "invariants": ["refused_action_never_executes"]}
        self.assertEqual(run_scenario(scenario)["outcome"], "PASS")

    def test_injected_failure_without_recovery_fails(self):
        scenario = {"id": "failure", "seed": 4, "events": [{"kind": "inject_failure", "name": "build"}], "invariants": ["recovery_is_explicit"]}
        result = run_scenario(scenario)
        self.assertEqual(result["outcome"], "FAIL")
        self.assertIn("explicit recovery", result["errors"][0])

    def test_checked_in_scenarios_pass(self):
        scenarios = json.loads((ROOT / "verification" / "scenarios.json").read_text(encoding="utf-8"))
        self.assertTrue(scenarios)
        for scenario in scenarios:
            self.assertEqual(run_scenario(scenario)["outcome"], "PASS")


if __name__ == "__main__":
    unittest.main()
