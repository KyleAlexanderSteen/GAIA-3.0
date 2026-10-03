import json
import tempfile
import unittest
from pathlib import Path

from promotion_readiness import build_report
from work_overlap_check import find_candidates
from work_history_audit import classify


class GovernanceTests(unittest.TestCase):
    def test_readiness_blocks_without_acceptance(self):
        body = """Refs #1626
## Stage reached
- [x] SPECIFICATION
- [x] IMPLEMENTATION
## Evidence
- IMPLEMENTATION: test
"""
        report = build_report(body, {"checks": {"unit": "success"}}, "abc", False)
        self.assertEqual(report["promotion"]["status"], "INCOMPLETE")
        self.assertTrue(report["promotion"]["human_approval_required"])
        self.assertFalse(report["promotion"]["authority_granted"])

    def test_readiness_requires_operational_for_closer(self):
        body = """Closes #1626
## Stage reached
- [x] SPECIFICATION
## Evidence
- SPECIFICATION: docs
"""
        report = build_report(body, {}, "abc", True)
        self.assertIn("closing relationship requires OPERATIONAL stage", report["promotion"]["blockers"])

    def test_readiness_can_reach_human_gate(self):
        body = """Refs #1626
## Stage reached
- [x] SPECIFICATION
- [x] IMPLEMENTATION
- [x] TEST
- [x] INTEGRATION
- [x] VERIFICATION
- [x] OPERATIONAL
## Evidence
- IMPLEMENTATION: test
"""
        report = build_report(body, {"checks": {"unit": "success"}}, "abc", True)
        self.assertEqual(report["promotion"]["status"], "READY FOR HUMAN APPROVAL")
        self.assertFalse(report["promotion"]["authority_granted"])

    def test_overlap_surfaces_same_issue_and_files(self):
        candidates = find_candidates(
            10, "Refs #20", {"a.py", "b.py"},
            [{"number": 11, "body": "Refs #20", "changed_files": ["b.py", "c.py"]}],
        )
        self.assertEqual(candidates[0]["shared_issues"], [20])
        self.assertEqual(candidates[0]["shared_files"], ["b.py"])
        self.assertTrue(candidates[0]["review_required"])

    def test_history_is_conservative(self):
        self.assertEqual(classify({"number": 1, "merged": True, "issue_number": 2})["classification"], "PARTIAL")
        self.assertEqual(classify({"number": 2, "merged": True, "issue_closed": True, "stages": ["OPERATIONAL"]})["classification"], "COMPLETE")
        self.assertFalse(classify({"number": 3, "merged": True, "issue_number": 4})["recovery"]["reopen_historical_pr"])


if __name__ == "__main__":
    unittest.main()
