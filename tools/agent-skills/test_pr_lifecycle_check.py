import unittest

from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "tools" / "agent-skills"))

from pr_lifecycle_check import check


def body(rel="Refs #1624", stages=("SPECIFICATION",)):
    lines = [rel, "", "## Stage reached"]
    for stage in ("SPECIFICATION", "IMPLEMENTATION", "TEST", "INTEGRATION", "VERIFICATION", "OPERATIONAL"):
        mark = "x" if stage in stages else " "
        lines.append(f"- [{mark}] {stage}")
    return "\n".join(lines)


class PrLifecycleCheckTests(unittest.TestCase):
    def test_refs_is_valid_for_partial_work(self):
        self.assertEqual(check(body()), [])

    def test_missing_relationship_fails(self):
        errors = check(body(rel="No issue here"))
        self.assertTrue(any("missing explicit issue relationship" in e for e in errors))

    def test_closes_requires_operational(self):
        errors = check(body(rel="Closes #1624"))
        self.assertTrue(any("OPERATIONAL" in e for e in errors))

    def test_closes_passes_at_operational(self):
        self.assertEqual(check(body(rel="Closes #1624", stages=("SPECIFICATION", "IMPLEMENTATION", "TEST", "INTEGRATION", "VERIFICATION", "OPERATIONAL"))), [])

    def test_fixes_and_resolves_are_also_closing_relationships(self):
        for relation in ("Fixes #1624", "Resolves #1624"):
            errors = check(body(rel=relation))
            self.assertTrue(any("OPERATIONAL" in e for e in errors), relation)

    def test_inline_closer_is_caught(self):
        errors = check(body(rel="This PR closes #1624"))
        self.assertTrue(any("OPERATIONAL" in e for e in errors))

    def test_multiple_refs_are_supported(self):
        self.assertEqual(check(body(rel="Refs #1624, #1537")), [])

    def test_active_duplicate_work_is_reported(self):
        active = [
            {"number": 1700, "body": body(rel="Refs #1624"), "current": False},
        ]
        errors = check(body(rel="Refs #1624"), active)
        self.assertTrue(any("active implementation collision" in e for e in errors))

    def test_other_issue_does_not_collide(self):
        active = [
            {"number": 1700, "body": body(rel="Refs #1539"), "current": False},
        ]
        self.assertEqual(check(body(rel="Refs #1624"), active), [])


if __name__ == "__main__":
    unittest.main()
