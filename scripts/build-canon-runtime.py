#!/usr/bin/env python3
"""Build canon-runtime/manifest.json from INDEX + CAP sidecar (#893)."""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
INDEX = ROOT / "docs" / "tablets" / "INDEX.md"
CAP_PATH = ROOT / "canon-runtime" / "cap.json"
SCHEMA_PATH = ROOT / "canon-runtime" / "schema.json"
MANIFEST_PATH = ROOT / "canon-runtime" / "manifest.json"
PROOFS = ROOT / "proofs"

ELEMENTS = {
    "Earth", "Water", "Fire", "Air", "Aether", "Spirit", "Light",
    "Shadow", "Sound", "Wood", "Metal", "Void", "Cosmos", "None",
}
STAGES = {
    "Prima Materia", "Calcination", "Dissolution", "Separation",
    "Conjunction", "Fermentation", "Distillation", "Coagulation", "Full Sequence",
}

ROW_RE = re.compile(
    r"^\|\s*(?P<n>\d+)\s*\|\s*\[(?P<label>[^\]]+)\]\((?P<path>[^)]+)\)\s*\|"
    r"\s*(?P<hexname>[^|]+)\|\s*(?P<color>[^|]+)\|\s*(?P<element>[^|]+)\|"
    r"\s*(?P<stage>[^|]+)\|\s*(?P<sealed>[^|]+)\|\s*(?P<tracker>[^|]+)\|\s*$"
)
HEX_RE = re.compile(r"`(#[0-9A-Fa-f]{6})`")
ISSUE_RE = re.compile(r"#(\d+)")
DATE_RE = re.compile(r"(\d{4}-\d{2}-\d{2})")


def slug_from_label(label: str) -> str:
    return label.replace(" Tablet", "").strip().lower().replace(" ", "-")


def parse_dash(value: str):
    text = value.strip().replace("\u2014", "-").replace("\u2013", "-")
    if text in {"", "-"}:
        return None
    return value.strip()


def parse_index(text: str) -> list[dict]:
    rows = []
    for line in text.splitlines():
        match = ROW_RE.match(line)
        if not match:
            continue
        label = match.group("label")
        path = match.group("path")
        source_path = path if path.startswith("docs/") else f"docs/tablets/{Path(path).name}"
        hex_match = HEX_RE.search(match.group("hexname"))
        if not hex_match:
            raise SystemExit(f"missing backtick hex in INDEX row: {label}")
        sealed_cell = match.group("sealed")
        sealed = None
        if "\u2705" in sealed_cell:
            date_match = DATE_RE.search(sealed_cell)
            sealed = date_match.group(1) if date_match else None
            if sealed is None:
                raise SystemExit(f"sealed row missing ISO date: {label}")
        tracker_match = ISSUE_RE.search(match.group("tracker"))
        if not tracker_match:
            raise SystemExit(f"missing tracker issue: {label}")
        rows.append({
            "id": slug_from_label(label),
            "name": label.strip(),
            "color": {"name": match.group("color").strip(), "hex": hex_match.group(1)},
            "element": parse_dash(match.group("element")),
            "stage": parse_dash(match.group("stage")),
            "sealed": sealed,
            "source_path": source_path,
            "tracker_issue": int(tracker_match.group(1)),
        })
    if len(rows) != 18:
        raise SystemExit(f"expected 18 INDEX tablet rows, found {len(rows)}")
    return rows


PROOF_HINTS = {
    "citrine": "PROOF-C210-CITRINE-001.md",
    "ember": "PROOF-EMBER-TABLET-001.md",
    "emerald": "PROOF-EMERALD-TABLET-HOUSING-001.md",
    "terra": "PROOF-TERRA-TABLET-001.md",
    "viriditas": "PROOF-VIRIDITAS-TABLET-001.md",
    "void": "PROOF-VOID-TABLET-001.md",
    "amber": "PROOF-AMBER-TABLET-001.md",
    "amethyst": "PROOF-AMETHYST-TABLET-001.md",
    "aqua": "PROOF-AQUA-TABLET-001.md",
    "celestial": "PROOF-CELESTIAL-TABLET-001.md",
    "lapis": "PROOF-LAPIS-TABLET-001.md",
    "obsidian": "PROOF-OBSIDIAN-TABLET-001.md",
    "rose": "PROOF-ROSE-TABLET-001.md",
    "ruby": "PROOF-RUBY-TABLET-001.md",
    "sapphire": "PROOF-SAPPHIRE-TABLET-001.md",
    "shadow": "PROOF-SHADOW-TABLET-001.md",
    "silver": "PROOF-SILVER-TABLET-001.md",
    "solar": "PROOF-SOLAR-TABLET-001.md",
}

LAWS = {
    "citrine": "The Law of Calibrated Light — calibrated confidence, not plausible certainty.",
    "emerald": "As above, so below — the law from which all transformation in this system derives.",
    "ember": "Calcination: the first fire that burns false structure.",
    "terra": "Earth holds the full sequence; the ground is not optional.",
    "viriditas": "The greening force: living coherence without extraction.",
    "void": "Prima materia: named emptiness, contained rather than denied.",
}


def validate_row(row: dict) -> None:
    if row["element"] is not None and row["element"] not in ELEMENTS:
        raise SystemExit(f"invalid element {row['element']!r} on {row['id']}")
    if row["stage"] is not None and row["stage"] not in STAGES:
        raise SystemExit(f"invalid stage {row['stage']!r} on {row['id']}")
    row["color"]["hex"] = row["color"]["hex"].upper()
    if not re.fullmatch(r"#[0-9A-Fa-f]{6}", row["color"]["hex"]):
        raise SystemExit(f"invalid hex on {row['id']}")
    if not (ROOT / row["source_path"]).is_file():
        raise SystemExit(f"missing source_path {row['source_path']}")
    proof = PROOFS / f"{row['proof_id']}.md"
    if not proof.is_file():
        raise SystemExit(f"broken proof path for {row['id']}: {proof.name}")
    if row["sealed"] and not any(row[k] for k in ("constraints", "affordances", "prohibitions")):
        raise SystemExit(f"sealed tablet {row['id']} has empty CAP lists")


def build() -> dict:
    index_rows = parse_index(INDEX.read_text(encoding="utf-8"))
    cap = json.loads(CAP_PATH.read_text(encoding="utf-8"))
    tablets = []
    for row in index_rows:
        slug = row["id"]
        if slug not in cap:
            raise SystemExit(f"CAP sidecar missing key {slug}")
        lists = cap[slug]
        tablet = {
            "id": slug,
            "name": row["name"],
            "color": {"name": row["color"]["name"], "hex": row["color"]["hex"].upper()},
            "governing_law": LAWS.get(slug, ""),
            "element": row["element"],
            "stage": row["stage"],
            "sealed": row["sealed"],
            "author": "Kyle Alexander Steen the Alchemist & GAIA",
            "version": "1.0.0",
            "constraints": list(lists.get("constraints") or []),
            "affordances": list(lists.get("affordances") or []),
            "prohibitions": list(lists.get("prohibitions") or []),
            "proof_id": Path(PROOF_HINTS[slug]).stem,
            "source_path": row["source_path"],
            "tracker_issue": row["tracker_issue"],
        }
        validate_row(tablet)
        tablets.append(tablet)
    tablets.sort(key=lambda t: t["id"])
    return {
        "schema": "canon-runtime/schema.json",
        "generated_from": "docs/tablets/INDEX.md",
        "enforcement": "none",
        "tablets": tablets,
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    if not SCHEMA_PATH.is_file():
        print("missing canon-runtime/schema.json", file=sys.stderr)
        return 1
    manifest = build()
    rendered = json.dumps(manifest, indent=2, sort_keys=False) + "\n"
    if args.check:
        if not MANIFEST_PATH.is_file():
            print("missing canon-runtime/manifest.json", file=sys.stderr)
            return 1
        current = json.loads(MANIFEST_PATH.read_text(encoding="utf-8"))
        if current != manifest:
            print("canon-runtime/manifest.json is stale; run scripts/build-canon-runtime.py", file=sys.stderr)
            return 1
        print(f"manifest ok ({len(manifest['tablets'])} tablets)")
        return 0
    MANIFEST_PATH.write_text(rendered, encoding="utf-8")
    print(f"wrote {MANIFEST_PATH.relative_to(ROOT)} ({len(manifest['tablets'])} tablets)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
