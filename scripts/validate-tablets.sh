#!/usr/bin/env bash
# Named wrapper for #834. Does not invent a second schema.
set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
exec python3 "$root/tests/canon/test_canon_integrity.py" "$@"
