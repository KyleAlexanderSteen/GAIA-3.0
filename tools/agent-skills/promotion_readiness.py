#!/usr/bin/env python3
"""Build a deterministic, machine-readable readiness report.

This module does not grant authority, merge code, or close issues. It only
normalizes explicit stage/evidence signals and reports blockers.
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

STAGES = (
    "SPECIFICATION",
    "IMPLEMENTATION",
    "TEST",
    "INTEGRATION",
    "VERIFICATION",
    "OPERATIONAL",
)
CLOSERS = re.compile(r"(?i)\b(?:closes|fixes|resolves)\s+(?:[\w.-]+/[\w.-]+)?#\d+")
ISSUE_REF = re.compile(r"(?:[\w.-]+/[\w.-]+)?#(\d+)")


def checked_stages(body: str) -> list[str]:
    return [s for s in STAGES if re.search(rf"(?im)^- \[[xX]\]\s*{s}\b", body)]


def evidence_lines(body: str) -> list[str]:
    lines = []
    in_evidence = False
    for line in body.splitlines():
        if line.strip().lower() == "## evidence":
            in_evidence = True
            continue
        if in_evidence and line.startswith("## "):
            break
        if in_evidence and line.lstrip().startswith("- "):
            lines.append(line.strip())
    return lines


def build_report(body: str, evidence: dict, commit: str, acceptance_complete: bool = False) -> dict:
    stages = checked_stages(body)
    evidence_lines_found = evidence_lines(body)
    blockers: list[str] = []

    if not stages:
        blockers.append("no lifecycle stage is explicitly checked")
    if not evidence_lines_found:
        blockers.append("no PR Evidence section entries were found")
    if not commit:
        blockers.append("evaluated commit SHA is missing")
    if not acceptance_complete:
        blockers.append("acceptance criteria are not explicitly marked complete")
    if CLOSERS.search(body) and "OPERATIONAL" not in stages:
        blockers.append("closing relationship requires OPERATIONAL stage")

    failed_checks = [
        name for name, result in evidence.get("checks", {}).items()
        if str(result).lower() not in {"success", "passed", "pass", "neutral", "skipped"}
    ]
    blockers.extend(f"validation check not successful: {name}" for name in failed_checks)

    if blockers:
        status = "BLOCKED" if any("check" in b or "missing" in b or "no " in b for b in blockers) else "INCOMPLETE"
    else:
        status = "READY FOR HUMAN APPROVAL"

    return {
        "schema_version": "1.0",
        "kind": "gaia.work-readiness-report",
        "commit": commit,
        "stages": [{"name": s, "explicit": True} for s in stages],
        "evidence": evidence,
        "promotion": {
            "status": status,
            "human_approval_required": True,
            "authority_granted": False,
            "blockers": blockers,
        },
    }


def main() -> int:
    p = argparse.ArgumentParser()
    p.add_argument("--body", required=True)
    p.add_argument("--evidence", required=True)
    p.add_argument("--commit", required=True)
    p.add_argument("--acceptance-complete", action="store_true")
    p.add_argument("--output")
    args = p.parse_args()

    body = Path(args.body).read_text(encoding="utf-8")
    evidence = json.loads(Path(args.evidence).read_text(encoding="utf-8"))
    report = build_report(body, evidence, args.commit, args.acceptance_complete)
    rendered = json.dumps(report, indent=2, sort_keys=True) + "\n"
    if args.output:
        Path(args.output).write_text(rendered, encoding="utf-8")
    print(rendered, end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
