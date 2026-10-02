#!/usr/bin/env python3
"""Fail if CLI or gateway code reports success next to an unwired TODO. #1304."""
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
ROOTS = [ROOT / "gaia-cli" / "src", ROOT / "gaia-gateway" / "src"]
TOKENS = ("queued", "Runtime started", '"created"', '"deploying"', "status: \"done\"")
bad = []
for root in ROOTS:
    for path in root.rglob("*.rs"):
        lines = path.read_text(encoding="utf-8").splitlines()
        for i, line in enumerate(lines):
            if "TODO" not in line:
                continue
            window = "\n".join(lines[max(0, i - 2) : i + 3])
            for token in TOKENS:
                if token in window:
                    bad.append(f"{path.relative_to(ROOT)}:{i + 1} TODO near {token}")
if bad:
    print("stub success guard failed:")
    print("\n".join(bad))
    sys.exit(1)
print("stub success guard ok")
