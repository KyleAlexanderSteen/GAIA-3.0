#!/usr/bin/env python3
"""Deterministic pre-PR validation for GAIA 3.0 (#1700)."""
from __future__ import annotations
import argparse, re, sys
from pathlib import Path
import dod_check

PATH_RE = re.compile(r"^(?:\.?\.?/|[A-Za-z0-9_.-]+/|[A-Za-z0-9_.-]+\.[A-Za-z0-9_-]+(?:/.*)?)")
URL_RE = re.compile(r"^(?:https?://|git@|mailto:)", re.IGNORECASE)

def _highest_checked(body: str) -> str | None:
    match = re.search(r"^##\s*Stage reached.*?(?=^##\s|\Z)", body, re.S | re.M)
    if not match: return None
    checked = [s for s in dod_check.STAGES if re.search(r"^- \[[xX]\]\s*" + re.escape(s) + r"\b", match.group(0), re.M)]
    return max(checked, key=dod_check.STAGES.index) if checked else None

def _evidence_values(body: str, stage: str) -> list[str]:
    match = re.search(r"^##\s*Stage reached.*?(?=^##\s|\Z)", body, re.S | re.M)
    stage_section = match.group(0) if match else ""
    evidence = re.search(r"^##\s*Evidence\s*$.*?(?=^##\s|\Z)", body, re.S | re.M)
    text = stage_section + (evidence.group(0) if evidence else "")
    pattern = r"^\s*[-*]\s+" + re.escape(stage) + r"\s*:\s*(\S.*?)\s*$"
    return [m.group(1).strip() for m in re.finditer(pattern, text, re.M)]

def _looks_like_path(value: str) -> bool:
    value = value.strip().strip(chr(96))
    if URL_RE.match(value): return False
    return bool(PATH_RE.match(value))

def check(body: str, repo_root: Path | None = None) -> list[str]:
    problems = list(dod_check.check(body))
    if repo_root is None: return problems
    stage = _highest_checked(body)
    if stage is None: return problems
    root = repo_root.resolve()
    for raw in _evidence_values(body, stage):
        value = raw.strip().strip(chr(96)).rstrip(".,;")
        path = Path(value)
        if path.is_absolute():
            problems.append(f"evidence path must be repository-relative: {raw}")
            continue
        if not _looks_like_path(value): continue
        resolved = (root / path).resolve()
        try: resolved.relative_to(root)
        except ValueError: problems.append(f"evidence path escapes repository: {raw}"); continue
        if not resolved.exists(): problems.append(f"evidence path does not exist: {raw}")
    return problems

def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description="Run deterministic GAIA pre-PR validation.")
    parser.add_argument("--body", help="PR body file; defaults to stdin")
    parser.add_argument("--repo-root", default=None, help="Repository root for evidence-path validation")
    args = parser.parse_args(argv)
    body = Path(args.body).read_text(encoding="utf-8") if args.body else sys.stdin.read()
    problems = check(body, Path(args.repo_root) if args.repo_root else None)
    for problem in problems: print("FAIL:", problem)
    if problems: print(f"{len(problems)} problem(s)"); return 1
    print("READY FOR VALIDATION"); return 0

if __name__ == "__main__": raise SystemExit(main())
