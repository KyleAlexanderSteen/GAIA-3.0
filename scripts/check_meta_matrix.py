#!/usr/bin/env python3
"""Listed international matrix. Grants nothing. Not live."""
import csv
from pathlib import Path
import sys

root = Path(__file__).resolve().parents[1] / "gaia-spec" / "bands"
required_domains = {"knowledge", "skills", "capabilities", "powers", "magic"}
required_scales = {"city", "county", "state", "country", "continent", "global"}
bad = []
with (root / "matrix.csv").open() as f:
    rows = list(csv.DictReader(f))
domains = {row["domain"] for row in rows}
scales = {row["scale"] for row in rows}
if domains != required_domains:
    bad.append(f"domains {domains}")
if scales != required_scales:
    bad.append(f"scales {scales}")
if len(rows) != 5 * 3 * 6:
    bad.append(f"expected 90 rows, got {len(rows)}")
for row in rows:
    if row["grants"] != "false" or row["live"] != "false":
        bad.append(f"grant or live row {row}")
with (root / "traditions.csv").open() as f:
    traditions = list(csv.DictReader(f))
if not any(row["id"] == "und" for row in traditions):
    bad.append("missing und tradition")
if any(row["operational"] != "false" for row in traditions):
    bad.append("a tradition is operational")
if bad:
    print("meta matrix check failed:")
    print("\n".join(bad))
    sys.exit(1)
print(f"meta matrix ok rows={len(rows)} traditions={len(traditions)}")
