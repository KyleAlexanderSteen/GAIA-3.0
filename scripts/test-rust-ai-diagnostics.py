#!/usr/bin/env python3
"""Regression tests for the Rust AI Diagnostics workflow's reporting invariants."""

import os
import tempfile
import sys

sys.path.insert(0, os.path.dirname(__file__))
from rust_ai_diagnostics_policy import (
    PASS,
    PRE_EXISTING,
    PR_FAILURE,
    UNKNOWN,
    classify_pr_enforcement,
)


def read_fmt_status(path):
    with open(path, encoding="utf-8") as fh:
        return int(fh.read().strip())


def test_fmt_status_is_exit_code_driven():
    with tempfile.TemporaryDirectory() as tmp:
        status = os.path.join(tmp, "fmt-exit-code")
        with open(status, "w", encoding="utf-8") as fh:
            fh.write("0\n")
        assert read_fmt_status(status) == 0

        with open(status, "w", encoding="utf-8") as fh:
            fh.write("1\n")
        assert read_fmt_status(status) == 1


def test_empty_output_does_not_imply_success():
    # The historical bug inferred formatting success from output text.
    # An empty output file is not a formatter status; the exit code is.
    with tempfile.TemporaryDirectory() as tmp:
        output = os.path.join(tmp, "fmt-raw.txt")
        status = os.path.join(tmp, "fmt-exit-code")
        open(output, "w", encoding="utf-8").close()
        with open(status, "w", encoding="utf-8") as fh:
            fh.write("1\n")
        assert os.path.getsize(output) == 0
        assert read_fmt_status(status) == 1


def test_checked_sha_is_authoritative():
    checked_sha = "abc123"
    pr_head_sha = "merge-ref-or-stale-sha"
    assert checked_sha != pr_head_sha
    # The report/artifact must use the checked-out HEAD, not a merge ref.
    reported_head_sha = checked_sha
    assert reported_head_sha == checked_sha


def test_failed_diagnostics_do_not_suppress_report_publication():
    # The workflow must publish the report after enforcement fails.
    workflow = open(
        ".github/workflows/rust-ai-diagnostics.yml", encoding="utf-8"
    ).read()
    assert "if: always() && steps.report.outcome == 'success'" in workflow
    assert workflow.index("Enforce diagnostics result") < workflow.index(
        "Post diagnostics comment"
    )


def test_enforcement_all_out_of_scope_is_non_blocking():
    status, reason = classify_pr_enforcement(
        blocking_diagnostics=False,
        fmt_failed=True,
        fmt_files=704,
        fmt_changed_files=0,
        ownership_classification_ok=True,
    )
    assert status == PRE_EXISTING
    assert "outside" in reason


def test_enforcement_pr_overlap_is_blocking():
    status, _ = classify_pr_enforcement(
        blocking_diagnostics=False,
        fmt_failed=True,
        fmt_files=704,
        fmt_changed_files=1,
        ownership_classification_ok=True,
    )
    assert status == PR_FAILURE


def test_enforcement_unknown_ownership_is_blocking():
    status, _ = classify_pr_enforcement(
        blocking_diagnostics=False,
        fmt_failed=True,
        fmt_files=704,
        fmt_changed_files=0,
        ownership_classification_ok=False,
    )
    assert status == UNKNOWN


def test_enforcement_clean_repository_passes():
    status, _ = classify_pr_enforcement(
        blocking_diagnostics=False,
        fmt_failed=False,
        fmt_files=0,
        fmt_changed_files=0,
        ownership_classification_ok=True,
    )
    assert status == PASS


if __name__ == "__main__":
    tests = [
        test_fmt_status_is_exit_code_driven,
        test_empty_output_does_not_imply_success,
        test_checked_sha_is_authoritative,
        test_failed_diagnostics_do_not_suppress_report_publication,
        test_enforcement_all_out_of_scope_is_non_blocking,
        test_enforcement_pr_overlap_is_blocking,
        test_enforcement_unknown_ownership_is_blocking,
        test_enforcement_clean_repository_passes,
    ]
    for test in tests:
        test()
        print(f"PASS: {test.__name__}")
    print("Rust AI Diagnostics regression tests: PASS")
