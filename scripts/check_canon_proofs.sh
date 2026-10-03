#!/usr/bin/env bash
# Canon Proof Gate — Author: Kyle Steen (KyleAlexanderSteen)
# C77 / THE ORDER: no canon without a proof artefact.
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$ROOT"

BASE_REF="${GITHUB_BASE_REF:-}"
CHANGED=""

if [[ -n "$BASE_REF" ]] && git rev-parse --verify "origin/${BASE_REF}" >/dev/null 2>&1; then
  CHANGED=$(git diff --name-only "origin/${BASE_REF}...HEAD" -- 'docs/canon/' || true)
elif git rev-parse --verify origin/main >/dev/null 2>&1; then
  CHANGED=$(git diff --name-only origin/main...HEAD -- 'docs/canon/' || true)
fi

if [[ -z "${CHANGED}" ]]; then
  CHANGED=$(git ls-files 'docs/canon/' || true)
fi

if [[ -z "${CHANGED}" ]]; then
  echo "No canon documents to check."
  exit 0
fi

proof_has_method() {
  local proof_path="$1"
  grep -Eqi '(^##[[:space:]]*Method|\*\*Type:\*\*|\*\*Status:\*\*|\*\*Method:\*\*)' "$proof_path"
}

fail=0
while IFS= read -r file; do
  [[ -z "$file" ]] && continue
  [[ -f "$file" ]] || continue
  [[ "$file" == *.md ]] || continue

  echo "Checking $file"
  found_proof=0

  if grep -Eq 'PROOF-[A-Z0-9][A-Z0-9_-]*' "$file"; then
    mapfile -t ids < <(grep -Eo 'PROOF-[A-Z0-9][A-Z0-9_-]*' "$file" | sort -u)
    for id in "${ids[@]}"; do
      proof_path="proofs/${id}.md"
      if [[ -f "$proof_path" ]]; then
        if proof_has_method "$proof_path"; then
          echo "  OK: $id -> $proof_path"
          found_proof=1
        else
          echo "  FAIL: $proof_path missing Type/Status/Method"
          fail=1
        fi
      fi
    done
  fi

  if [[ "$found_proof" -eq 0 ]] && [[ -d proofs ]]; then
    while IFS= read -r proof_path; do
      [[ -z "$proof_path" ]] && continue
      if grep -Fq "$file" "$proof_path" && proof_has_method "$proof_path"; then
        echo "  OK: path cited by $proof_path"
        found_proof=1
      fi
    done < <(git ls-files 'proofs/*.md' || ls proofs/*.md 2>/dev/null || true)
  fi

  if [[ "$found_proof" -eq 0 ]]; then
    echo "  FAIL: no PROOF- id referenced and no proofs/*.md cites this path"
    fail=1
  fi
done <<< "$CHANGED"

if [[ "$fail" -ne 0 ]]; then
  echo "Canon Proof Gate failed. C77: no canon without proof."
  exit 1
fi

echo "Canon Proof Gate passed."
