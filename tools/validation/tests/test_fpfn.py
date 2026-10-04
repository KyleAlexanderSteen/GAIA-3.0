import pathlib
import sys
import unittest

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))

from fpfn import (
    ConformanceResult,
    ExpectedOutcome,
    Fixture,
    assert_conforming,
    evaluate,
    require_categories,
)


class FpFnConformance(unittest.TestCase):
    def test_positive_condition_detected_is_true_positive(self):
        fixture = Fixture(
            "real-condition-detected",
            ExpectedOutcome.POSITIVE,
            ExpectedOutcome.POSITIVE,
            category="positive",
        )
        self.assertEqual(fixture.evaluate(), ConformanceResult.TRUE_POSITIVE)

    def test_real_condition_missed_is_false_negative(self):
        fixture = Fixture(
            "real-condition-missed",
            ExpectedOutcome.POSITIVE,
            ExpectedOutcome.NEGATIVE,
            category="positive",
        )
        self.assertEqual(fixture.evaluate(), ConformanceResult.FALSE_NEGATIVE)

    def test_no_condition_rejected_is_true_negative(self):
        fixture = Fixture(
            "no-condition-rejected",
            ExpectedOutcome.NEGATIVE,
            ExpectedOutcome.NEGATIVE,
            category="negative",
        )
        self.assertEqual(fixture.evaluate(), ConformanceResult.TRUE_NEGATIVE)

    def test_no_condition_reported_is_false_positive(self):
        fixture = Fixture(
            "no-condition-reported",
            ExpectedOutcome.NEGATIVE,
            ExpectedOutcome.POSITIVE,
            category="negative",
        )
        self.assertEqual(fixture.evaluate(), ConformanceResult.FALSE_POSITIVE)

    def test_unknown_remains_unknown(self):
        fixture = Fixture(
            "insufficient-evidence",
            ExpectedOutcome.UNKNOWN,
            ExpectedOutcome.UNKNOWN,
            category="unknown",
        )
        self.assertEqual(fixture.evaluate(), ConformanceResult.UNKNOWN_CORRECT)

    def test_unknown_forced_positive_is_mismatch(self):
        fixture = Fixture(
            "unknown-forced-positive",
            ExpectedOutcome.UNKNOWN,
            ExpectedOutcome.POSITIVE,
            category="unknown",
        )
        self.assertEqual(fixture.evaluate(), ConformanceResult.UNKNOWN_MISMATCH)

    def test_unknown_forced_negative_is_mismatch(self):
        fixture = Fixture(
            "unknown-forced-negative",
            ExpectedOutcome.UNKNOWN,
            ExpectedOutcome.NEGATIVE,
            category="unknown",
        )
        self.assertEqual(fixture.evaluate(), ConformanceResult.UNKNOWN_MISMATCH)

    def test_assert_conforming_accepts_both_directions_and_unknown(self):
        fixtures = [
            Fixture("positive", ExpectedOutcome.POSITIVE, ExpectedOutcome.POSITIVE),
            Fixture("negative", ExpectedOutcome.NEGATIVE, ExpectedOutcome.NEGATIVE),
            Fixture("unknown", ExpectedOutcome.UNKNOWN, ExpectedOutcome.UNKNOWN),
        ]
        assert_conforming(fixtures)

    def test_assert_conforming_rejects_false_positive(self):
        fixtures = [
            Fixture("false-positive", ExpectedOutcome.NEGATIVE, ExpectedOutcome.POSITIVE)
        ]
        with self.assertRaises(AssertionError):
            assert_conforming(fixtures)

    def test_evaluate_preserves_one_result_per_fixture(self):
        fixtures = [
            Fixture("a", ExpectedOutcome.POSITIVE, ExpectedOutcome.POSITIVE),
            Fixture("b", ExpectedOutcome.NEGATIVE, ExpectedOutcome.POSITIVE),
            Fixture("c", ExpectedOutcome.UNKNOWN, ExpectedOutcome.UNKNOWN),
        ]
        self.assertEqual(
            evaluate(fixtures),
            [
                ConformanceResult.TRUE_POSITIVE,
                ConformanceResult.FALSE_POSITIVE,
                ConformanceResult.UNKNOWN_CORRECT,
            ],
        )

    def test_required_fixture_categories_are_explicit(self):
        fixtures = [
            Fixture("positive", ExpectedOutcome.POSITIVE, ExpectedOutcome.POSITIVE, "positive"),
            Fixture("negative", ExpectedOutcome.NEGATIVE, ExpectedOutcome.NEGATIVE, "negative"),
            Fixture("boundary", ExpectedOutcome.UNKNOWN, ExpectedOutcome.UNKNOWN, "boundary"),
            Fixture("conflicting", ExpectedOutcome.UNKNOWN, ExpectedOutcome.UNKNOWN, "conflicting"),
        ]
        require_categories(fixtures, "positive", "negative", "boundary", "conflicting")

    def test_missing_required_category_is_rejected(self):
        fixtures = [
            Fixture("positive", ExpectedOutcome.POSITIVE, ExpectedOutcome.POSITIVE, "positive"),
            Fixture("negative", ExpectedOutcome.NEGATIVE, ExpectedOutcome.NEGATIVE, "negative"),
        ]
        with self.assertRaises(AssertionError):
            require_categories(fixtures, "positive", "negative", "boundary")


if __name__ == "__main__":
    unittest.main()
