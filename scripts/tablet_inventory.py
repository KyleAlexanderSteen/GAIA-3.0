#!/usr/bin/env python3
"""Print docs/tablets extras vs INDEX names vs color-map. Advisory. INDEX wins."""
from __future__ import annotations

import json
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
TABLETS = ROOT / "docs" / "tablets"
INDEX = TABLETS / "INDEX.md"
CMAP = ROOT / "docs" / "color" / "color-map.json"


def main() -> int:
    files = sorted(p.name for p in TABLETS.glob("*_TABLET.md"))
    index = INDEX.read_text(encoding="utf-8")
    names = re.findall(r"\[([A-Za-z]+) Tablet\]", index)
    cmap = json.loads(CMAP.read_text(encoding="utf-8"))
    cmap_files = [t.get("file", "").split("/")[-1] for t in cmap.get("tablets", [])]
    extras = [f for f in files if f.replace("_TABLET.md", "") not in names]
    missing_files = [n + "_TABLET.md" for n in names if n + "_TABLET.md" not in files]
    print("files", len(files))
    print("index_names", len(names), names)
    print("cmap_rows", len(cmap_files))
    print("extras", extras)
    print("missing_files", missing_files)
    sealed_null = [t["name"] for t in cmap["tablets"] if t.get("sealed") is None]
    print("cmap_sealed_null", sealed_null)
    return 0


if __name__ == "__main__":
    sys.exit(main())
