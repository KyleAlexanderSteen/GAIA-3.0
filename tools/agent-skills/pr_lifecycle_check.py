#!/usr/bin/env python3
"""Validate GAIA PR issue-lifecycle metadata.

The checker is intentionally deterministic and has no network dependency.
It enforces:
- at least one explicit issue relationship (Refs/Closes/Fixes/Resolves);
- all closing keywords require OPERATIONAL in the DoD stage list;
- closing references are positive issue numbers;
- active PR collisions are reported when two open PRs claim the same issue;
- Refs is the default partial-work relationship.

Optional --active-prs accepts a JSON array of objects with number, title, body.
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from collections import defaultdict
from typing import Any

ISSUE_RELATION = re.compile(
    r"(?im)^\s*(?P<kind>refs?|closes?|fixes?|resolves?)\s*:??\s*"
    r"(?P<refs>(?:[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+)?#\d+(?:\s*,\s*)?)+\s*$"
)
ANY_CLOSER = re.compile(
    r"(?i)(?<![A-Za-z])(?:close|closes|closed|fix|fixes|fixed|resolve|resolves|resolved)"
    r"\s*:??\s*(?:[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+)?#\d+"
)
ISSUE_REF = re.compile(r"(?:[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+)?#(\d+)")

STAGES = (
    "SPECIFICATION",
    "IMPLEMENTATION",
    "TEST",
    "INTEGRATION",
    "VERIFICATION",
    "OPERATIONAL",
)


def checked_stages(body: str) -> set[str]:
    return {
        stage
        for stage in STAGES
        if re.search(r"(?m)^- \[[xX]\]\s*" + stage + r"\b", body)
    }


def relationships(body: str) -> list[tuple[str, list[int]]]:
    found: list[tuple[str, list[int]]] = []
    for match in ISSUE_RELATION.finditer(body):
        refs = [int(n) for n in ISSUE_REF.findall(match.group("refs"))]
        if refs:
            found.append((match.group("kind").lower(), refs))
    return found


def check(body: str, active_prs: list[dict[str, Any]] | None = None) -> list[str]:
    problems: list[str] = []
    rels = relationships(body)
    if not rels:
        problems.append(
            "missing explicit issue relationship; use a line such as 'Refs #123' "
            "or 'Closes #123'"
        )

    stages = checked_stages(body)
    closing_refs = [
        n
        for kind, refs in rels
        if kind.startswith(("close", "fix", "resolve"))
        for n in refs
    ]

    # Also catch closing keywords written inline, which GitHub accepts.
    inline_closers = ANY_CLOSER.findall(body)
    if inline_closers:
        closing_refs.extend(int(n) for n in inline_closers)

    if closing_refs and "OPERATIONAL" not in stages:
        problems.append(
            "uses a GitHub closing keyword but OPERATIONAL is not checked; "
            "use 'Refs #N' for partial work"
        )

    for number in closing_refs:
        if number <= 0:
            problems.append(f"invalid issue number: #{number}")

    if active_prs is not None:
        mine = set(n for _, refs in rels for n in refs)
        for pr in active_prs:
            number = pr.get("number")
            if not isinstance(number, int):
                continue
            if pr.get("current") or (args.current_pr is not None and pr.get("number") == args.current_pr):
                continue
            other = set(n for _, refs in relationships(pr.get("body", "")) for n in refs)
            overlap = sorted(mine & other)
            if overlap:
                problems.append(
                    "active implementation collision: PR #"
                    + str(number)
                    + " references the same issue(s): "
                    + ", ".join(f"#{n}" for n in overlap)
                )

    return problems


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--body", help="PR body file; otherwise stdin")
    parser.add_argument("--active-prs", help="JSON array of open PR metadata")
    parser.add_argument("--current-pr", type=int, help="PR number being checked")
    args = parser.parse_args()

    body = open(args.body, encoding="utf-8").read() if args.body else sys.stdin.read()
    active: list[dict[str, Any]] | None = None
    if args.active_prs:
        with open(args.active_prs, encoding="utf-8") as fh:
            active = json.load(fh)

    problems = check(body, active)
    for problem in problems:
        print(f"FAIL: {problem}")
    print("ok" if not problems else f"{len(problems)} problem(s)")
    return 1 if problems else 0


if __name__ == "__main__":
    raise SystemExit(main())
