#!/usr/bin/env python3
"""Deterministically surface possible overlapping implementation work.

It reports candidates; it never declares unrelated issues duplicates.
"""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path

REF = re.compile(r"(?:[\w.-]+/[\w.-]+)?#(\d+)")


def issue_refs(body: str) -> set[int]:
    return {int(n) for n in REF.findall(body or "")}


def overlap(current_files: set[str], other_files: set[str]) -> float:
    if not current_files or not other_files:
        return 0.0
    return len(current_files & other_files) / len(current_files | other_files)


def find_candidates(current_number: int, current_body: str, current_files: set[str],
                    active_prs: list[dict]) -> list[dict]:
    current_issues = issue_refs(current_body)
    candidates = []
    for pr in active_prs:
        if int(pr.get("number", -1)) == current_number:
            continue
        other_issues = issue_refs(pr.get("body", ""))
        other_files = set(pr.get("changed_files", []))
        shared_issues = sorted(current_issues & other_issues)
        shared_files = sorted(current_files & other_files)
        score = overlap(current_files, other_files)
        if shared_issues or shared_files:
            candidates.append({
                "pr": int(pr["number"]),
                "shared_issues": shared_issues,
                "shared_files": shared_files,
                "file_overlap_ratio": round(score, 4),
                "review_required": True,
            })
    return candidates


def main() -> int:
    p = argparse.ArgumentParser()
    p.add_argument("--current-pr", type=int, required=True)
    p.add_argument("--body", required=True)
    p.add_argument("--files", required=True)
    p.add_argument("--active-prs", required=True)
    p.add_argument("--output")
    args = p.parse_args()

    body = Path(args.body).read_text(encoding="utf-8")
    current_files = set(json.loads(Path(args.files).read_text(encoding="utf-8")))
    active = json.loads(Path(args.active_prs).read_text(encoding="utf-8"))
    report = {
        "schema_version": "1.0",
        "kind": "gaia.work-overlap-report",
        "candidates": find_candidates(args.current_pr, body, current_files, active),
        "automatic_duplicate_decision": False,
    }
    rendered = json.dumps(report, indent=2, sort_keys=True) + "\n"
    if args.output:
        Path(args.output).write_text(rendered, encoding="utf-8")
    print(rendered, end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
