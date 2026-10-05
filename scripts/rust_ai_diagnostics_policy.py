#!/usr/bin/env python3
"""Ownership-aware enforcement policy for Rust AI Diagnostics."""

PASS = "PASS"
PRE_EXISTING = "PRE_EXISTING"
PR_FAILURE = "PR_FAILURE"
UNKNOWN = "UNKNOWN"

def classify_pr_enforcement(*, blocking_diagnostics, fmt_failed, fmt_files, fmt_changed_files, ownership_classification_ok):
    """Return (status, reason) for the PR-specific enforcement gate."""
    if blocking_diagnostics:
        return PR_FAILURE, "blocking compiler diagnostics are present"
    if not fmt_failed:
        return PASS, "no blocking formatter failure"
    if not ownership_classification_ok:
        return UNKNOWN, "formatter ownership could not be established"
    if fmt_changed_files > 0:
        return PR_FAILURE, "formatter failure overlaps the PR changed-file set"
    if fmt_files > 0:
        return PRE_EXISTING, "formatter failure is outside the PR changed-file set"
    return UNKNOWN, "formatter failed but produced no classifiable formatter files"
