#!/usr/bin/env python3
"""Classify historical PR records without reopening or changing anything.

The classifier is deliberately conservative: insufficient evidence becomes
UNCLEAR rather than an invented completion claim.
"""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path

STAGES = ("SPECIFICATION", "IMPLEMENTATION", "TEST", "INTEGRATION", "VERIFICATION", "OPERATIONAL")


def classify(pr: dict) -> dict:
    if pr.get("duplicate"):
        classification = "DUPLICATE"
    elif pr.get("superseded"):
        classification = "SUPERSEDED"
    elif pr.get("documentation_only"):
        classification = "DOCUMENTATION_ONLY"
    elif not pr.get("merged", False):
        classification = "FAILED_OR_UNMERGED"
    elif pr.get("issue_closed") and "OPERATIONAL" in pr.get("stages", []):
        classification = "COMPLETE"
    elif pr.get("merged"):
        classification = "PARTIAL" if pr.get("issue_number") else "UNCLEAR"
    else:
        classification = "UNCLEAR"

    recovery = None
    if classification in {"PARTIAL", "FAILED_OR_UNMERGED", "UNCLEAR"}:
        recovery = {
            "action": "REVIEW_CANONICAL_ISSUE",
            "reopen_historical_pr": False,
            "preserve_issue_linkage": True,
        }

    return {
        "pr": pr.get("number"),
        "issue": pr.get("issue_number"),
        "classification": classification,
        "recovery": recovery,
    }


def main() -> int:
    p = argparse.ArgumentParser()
    p.add_argument("--input", required=True)
    p.add_argument("--output")
    args = p.parse_args()
    records = json.loads(Path(args.input).read_text(encoding="utf-8"))
    report = {
        "schema_version": "1.0",
        "kind": "gaia.historical-work-audit",
        "records": [classify(r) for r in records],
        "mutations_performed": False,
    }
    rendered = json.dumps(report, indent=2, sort_keys=True) + "\n"
    if args.output:
        Path(args.output).write_text(rendered, encoding="utf-8")
    print(rendered, end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
