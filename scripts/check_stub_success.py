#!/usr/bin/env python3
"""Fail if crate code reports success next to an unwired TODO. #1304."""
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
ROOTS = [p / "src" for p in ROOT.glob("gaia-*") if (p / "src").is_dir()]
ROOTS.append(ROOT / "gaia-sdk" / "rust" / "src")
TOKENS = ("queued", "Runtime started", '"created"', '"deploying"', 'status: "done"', "mcp-ok:", 'state: "admitted"', 'state="admitted"')
bad = []
for root in ROOTS:
    if not root.exists():
        continue
    for path in root.rglob("*.rs"):
        lines = path.read_text(encoding="utf-8").splitlines()
        for i, line in enumerate(lines):
            if "TODO" not in line and "todo!" not in line:
                continue
            window = "\n".join(lines[max(0, i - 2) : i + 3])
            for token in TOKENS:
                if token in window:
                    bad.append(f"{path.relative_to(ROOT)}:{i + 1} TODO near {token}")
sdk = ROOT / "gaia-sdk"
for path in list(sdk.rglob("*.rs")) + list(sdk.rglob("*.py")):
    text = path.read_text(encoding="utf-8")
    if 'state: "admitted"' in text or 'state="admitted"' in text or "mcp-ok:" in text:
        bad.append(f"{path.relative_to(ROOT)} still reports admitted or mcp-ok")
if bad:
    print("stub success guard failed:")
    print("\n".join(bad))
    sys.exit(1)
print("stub success guard ok")
