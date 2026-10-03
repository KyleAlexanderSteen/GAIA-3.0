#!/usr/bin/env python3
"""Verify that a merged PR produced the issue state it declared.

The workflow supplies issue states collected from the GitHub API. The core
verification is deterministic and network-free so it can also be unit-tested.
"""

from __future__ import annotations

import argparse
import json
import re
import sys

ISSUE_RELATION = re.compile(
    r"(?im)^\s*(?P<kind>refs?|closes?|fixes?|resolves?)\s*:??\s*"
    r"(?P<refs>(?:[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+)?#\d+(?:\s*,\s*)?)+\s*$"
)
ISSUE_REF = re.compile(r"(?:[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+)?#(\d+)")


def relationships(body: str) -> list[tuple[str, list[int]]]:
    return [
        (m.group("kind").lower(), [int(n) for n in ISSUE_REF.findall(m.group("refs"))])
        for m in ISSUE_RELATION.finditer(body)
    ]


def verify(body: str, issue_states: dict[str, str]) -> list[str]:
    problems: list[str] = []
    rels = relationships(body)

    for kind, refs in rels:
        if kind.startswith(("close", "fix", "resolve")):
            expected = "closed"
        elif kind.startswith("ref"):
            expected = "open"
        else:
            continue

        for number in refs:
            actual = issue_states.get(str(number))
            if actual is None:
                problems.append(f"missing post-merge state for #{number}")
            elif actual != expected:
                problems.append(
                    f"issue state mismatch for #{number}: PR relationship "
                    f"{kind} expects {expected}, observed {actual}"
                )

    return problems


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--body", required=True)
    parser.add_argument("--states", required=True, help="JSON object: issue number -> state")
    args = parser.parse_args()

    body = open(args.body, encoding="utf-8").read()
    with open(args.states, encoding="utf-8") as fh:
        states = json.load(fh)

    problems = verify(body, states)
    for problem in problems:
        print(f"FAIL: {problem}")
    print("ok" if not problems else f"{len(problems)} problem(s)")
    return 1 if problems else 0


if __name__ == "__main__":
    raise SystemExit(main())
