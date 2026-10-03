import unittest

from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "tools" / "agent-skills"))

from pr_lifecycle_postmerge_check import verify


class PrLifecyclePostMergeTests(unittest.TestCase):
    def test_refs_requires_issue_to_remain_open(self):
        self.assertEqual(verify("Refs #1624", {"1624": "open"}), [])

    def test_refs_detects_unexpected_closure(self):
        errors = verify("Refs #1624", {"1624": "closed"})
        self.assertTrue(any("expects open" in e for e in errors))

    def test_closes_requires_issue_to_be_closed(self):
        self.assertEqual(verify("Closes #1624", {"1624": "closed"}), [])

    def test_closes_detects_missing_closure(self):
        errors = verify("Closes #1624", {"1624": "open"})
        self.assertTrue(any("expects closed" in e for e in errors))

    def test_fixes_and_resolves_are_completion_relationships(self):
        for relation in ("Fixes #1624", "Resolves #1624"):
            self.assertEqual(verify(relation, {"1624": "closed"}), [])

    def test_multiple_relationships_are_checked(self):
        errors = verify(
            "Refs #1624\nCloses #1539",
            {"1624": "open", "1539": "open"},
        )
        self.assertEqual(len(errors), 1)
        self.assertIn("#1539", errors[0])

    def test_missing_state_is_failure(self):
        errors = verify("Closes #1624", {})
        self.assertTrue(any("missing post-merge state" in e for e in errors))


if __name__ == "__main__":
    unittest.main()
