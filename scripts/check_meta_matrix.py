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
with (root / "ways.csv").open() as f:
    ways = list(csv.DictReader(f))
way_ids = {row["way"] for row in ways}
if not {"science", "technology", "philosophy", "unspecified"} <= way_ids:
    bad.append(f"ways {way_ids}")
if any(row["operational"] != "false" or row["certifies_consciousness"] != "false" for row in ways):
    bad.append("a way is operational or certifies consciousness")
if not any(row["id"] == "und" for row in ways):
    bad.append("missing und way")
with (root / "chaos-order.csv").open() as f:
    chaos_rows = list(csv.DictReader(f))
seen = {(row["domain"], row["band"]) for row in chaos_rows}
expect = {(d, b) for d in required_domains for b in ("normal", "super")}
if seen != expect:
    bad.append(f"chaos-order coverage {seen}")
if any(row["grants"] != "false" or row["live"] != "false" for row in chaos_rows):
    bad.append("chaos-order grants or is live")
if bad:
    print("meta matrix check failed:")
    print("\n".join(bad))
    sys.exit(1)
print(f"meta matrix ok rows={len(rows)} traditions={len(traditions)} ways={len(ways)}")
