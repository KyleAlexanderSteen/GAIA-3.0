#!/usr/bin/env bash
set -uo pipefail
BASE_REF="main"
SKIP_TESTS=false
while [[ $# -gt 0 ]]; do
  case "$1" in
    --base) BASE_REF="${2:-main}"; shift 2;;
    --skip-tests) SKIP_TESTS=true; shift;;
    -h|--help) echo "Usage: $0 [--base REF] [--skip-tests]"; exit 0;;
    *) echo "Unknown argument: $1" >&2; exit 2;;
  esac
done
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
WARN=0
BLOCK=0
pass(){ printf '[PASS] %s\n' "$1"; }
warn(){ WARN=$((WARN+1)); printf '[WARN] %s\n' "$1"; }
block(){ BLOCK=$((BLOCK+1)); printf '[BLOCK] %s\n' "$1"; }
section(){ printf '\n%s\n%s\n' "$1" '----------------------------------------'; }
section 'GAIA PREFLIGHT'
printf 'Branch: '; git branch --show-current
printf 'HEAD:   '; git rev-parse --short HEAD
printf 'Base:   %s\n' "$BASE_REF"
if [[ -n "$(git status --porcelain)" ]]; then block 'Working tree is not clean.'; else pass 'Working tree clean.'; fi
if ! git fetch --quiet origin "$BASE_REF"; then block "Unable to fetch origin/$BASE_REF."; exit 1; fi
BASE="origin/$BASE_REF"
HEAD_SHA="$(git rev-parse HEAD)"
BASE_SHA="$(git rev-parse "$BASE")"
MERGE_BASE="$(git merge-base HEAD "$BASE" 2>/dev/null || true)"
if [[ "$HEAD_SHA" == "$BASE_SHA" ]]; then warn "Branch has no commits ahead of $BASE_REF.";
elif [[ "$MERGE_BASE" == "$BASE_SHA" ]]; then ahead="$(git rev-list --count "$BASE..HEAD")"; pass "Branch is synchronized: $ahead commit(s) ahead, 0 behind.";
else ahead="$(git rev-list --count "$BASE..HEAD")"; behind="$(git rev-list --count "$HEAD..$BASE")"; block "Branch diverges from $BASE_REF: ahead $ahead, behind $behind."; fi
section 'MERGE HOTSPOTS'
for path in gaia-kernel/src/lib.rs gaia-cli/src/main.rs gaia-cli/src/commands/mod.rs; do
  if git diff --name-only "$BASE...HEAD" -- "$path" | grep -q .; then warn "Changed merge hotspot: $path"; else pass "Hotspot untouched: $path"; fi
done
section 'RUST MODULE REGISTRY'
MODULE_ERRORS=0
for file in gaia-kernel/src/lib.rs gaia-cli/src/commands/mod.rs; do
  [[ -f "$file" ]] || continue
  while IFS= read -r module; do
    [[ -z "$module" ]] && continue
    dir="$(dirname "$file")"
    if [[ ! -f "$dir/$module.rs" && ! -f "$dir/$module/mod.rs" ]]; then
      block "$file declares missing module: $module"; MODULE_ERRORS=$((MODULE_ERRORS+1))
    fi
  done < <(grep -E '^pub mod [A-Za-z0-9_]+;' "$file" | awk '{print $3}' | tr -d ';')
done
[[ "$MODULE_ERRORS" -eq 0 ]] && pass 'Declared kernel/CLI modules resolve to source files.'
section 'FORMAT / BUILD VALIDATION'
mapfile -t FMT_FILES < <(git diff --name-only "$BASE...HEAD" -- '*.rs')
if [[ ${#FMT_FILES[@]} -eq 0 ]]; then pass 'No changed Rust files; format check skipped.'
elif rustfmt --edition 2021 --check "${FMT_FILES[@]}" >/tmp/gaia-preflight-fmt.out 2>&1; then pass 'rustfmt passed.'
else block 'rustfmt failed; see /tmp/gaia-preflight-fmt.out.'; fi
if cargo clippy --workspace --exclude gaia-cli --exclude gaia-agents --message-format=short -- -D warnings >/tmp/gaia-preflight-clippy.out 2>&1; then pass 'cargo clippy passed.'; else block 'cargo clippy failed; see /tmp/gaia-preflight-clippy.out.'; fi
if [[ "$SKIP_TESTS" == true ]]; then warn 'cargo test skipped by request.'
elif cargo test --workspace --exclude gaia-cli --exclude gaia-agents -- --test-threads=4 >/tmp/gaia-preflight-test.out 2>&1; then pass 'cargo test passed.'
else block 'cargo test failed; see /tmp/gaia-preflight-test.out.'; fi
section 'RESULT'
if [[ "$BLOCK" -eq 0 ]]; then [[ "$WARN" -eq 0 ]] && echo 'RESULT: READY' || echo 'RESULT: READY WITH WARNINGS'; exit 0; fi
echo "RESULT: BLOCKED ($BLOCK blocking check(s), $WARN warning(s))"
exit 1
