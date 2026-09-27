#!/usr/bin/env python3
"""Fail if refused crates appear in the workspace or catalog runtime is enabled."""
from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CARGO = ROOT / "Cargo.toml"
CATALOG = ROOT / "docs" / "knowledge" / "catalog.json"
REFUSE = ROOT / "docs" / "governance" / "REFUSE.md"
BANNED = ("gaia-learn", "gaia-economy", "gaia-net", "gaia-xr", "gaia-studio", "gaia-forge")


def main() -> int:
    errors = 0
    if not REFUSE.is_file():
        print("error: missing docs/governance/REFUSE.md")
        return 1
    cargo = CARGO.read_text(encoding="utf-8")
    members = set(re.findall(r'"([^"]+)"', cargo.split("[workspace.package]")[0]))
    # members paths may be gaia-sdk/rust — compare last segment and full path
    flat = set()
    for m in members:
        flat.add(m)
        flat.add(m.split("/")[0])
    for name in BANNED:
        if name in flat:
            print(f"error: refused crate in workspace: {name}")
            errors += 1
    if CATALOG.is_file():
        data = json.loads(CATALOG.read_text(encoding="utf-8"))
        if data.get("runtime_enabled") is True:
            print("error: catalog runtime_enabled must stay false")
            errors += 1
    if errors:
        print(f"refuse list failed ({errors})")
        return 1
    print("refuse list clean")
    return 0


if __name__ == "__main__":
    sys.exit(main())
