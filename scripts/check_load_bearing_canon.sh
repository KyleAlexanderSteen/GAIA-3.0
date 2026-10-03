#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
fail=0

require_file() {
  local path="$1"
  if [[ ! -s "$path" ]]; then
    echo "FAIL: missing or empty: $path"
    fail=1
  else
    echo "OK: $path"
  fi
}

require_text() {
  local path="$1"
  local pattern="$2"
  local label="$3"
  if [[ ! -f "$path" ]]; then
    echo "FAIL: cannot search ${label}; missing ${path}"
    fail=1
    return
  fi
  if grep -qiE "$pattern" "$path"; then
    echo "OK: $path contains ${label}"
  else
    echo "FAIL: $path missing required ${label}"
    fail=1
  fi
}

require_file "docs/canon/C77_LOVE_LED_STEWARDSHIP_DOCTRINE.md"
require_file "docs/ethics/ISSUE_64_PRIVACY_CONSTITUTION.md"
require_file "docs/ethics/ISSUE_221_CONSTITUTION.md"
require_file "NOTICE"
require_file "SECURITY.md"
require_file "gaia-gaian/src/constitution.rs"
require_file "proofs/PROOF-C77-LOVE-001.md"
require_file "proofs/PROOF_ISSUE_64.md"
require_file "proofs/PROOF_ISSUE_221.md"

require_text "docs/canon/C77_LOVE_LED_STEWARDSHIP_DOCTRINE.md" "Love-Led|stewardship" "Love-Led Stewardship"
require_text "docs/ethics/ISSUE_64_PRIVACY_CONSTITUTION.md" "privacy|age" "privacy/age-gate language"
require_text "docs/ethics/ISSUE_221_CONSTITUTION.md" "refus|consent|sovereign" "constitution refusals"
require_text "gaia-gaian/src/constitution.rs" "age|privacy|consent" "runtime constitution"
require_text "NOTICE" "Kyle|Steen|Kyle Alexander Steen|Alchemist|GAIA" "authorship notice"

if [[ "$fail" -ne 0 ]]; then
  echo "Load-bearing canon gate failed."
  exit 1
fi

echo "Load-bearing canon gate passed."
