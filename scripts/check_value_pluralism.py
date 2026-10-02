#!/usr/bin/env python3
"""Value pluralism stays listed. No row grants. No row collapses to one reward."""
import csv
from pathlib import Path
import sys

path = Path(__file__).resolve().parents[1] / "gaia-spec" / "knowledge" / "pluralism.csv"
rows = list(csv.DictReader(path.open()))
required = {
    "truthfulness",
    "neighbor-dignity",
    "care-for-least",
    "non-domination",
    "no-compulsion",
    "non-harm",
    "stewardship",
    "registered-failure",
    "mercy-with-record",
    "collision-is-real",
    "no-final-harmony",
    "liberty-not-master",
    "loss-stays-loss",
}
bad = []
if {row["id"] for row in rows} != required:
    bad.append(f"missing or extra values: { {row['id'] for row in rows} ^ required }")
if any(row["grants"] != "false" for row in rows):
    bad.append("a value grants")
if any(not row["does_not_collapse_into"] for row in rows):
    bad.append("a value collapses")
if bad:
    print("value pluralism check failed:")
    print("\n".join(bad))
    sys.exit(1)
print(f"value pluralism ok rows={len(rows)}")
