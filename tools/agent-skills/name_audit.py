#!/usr/bin/env python3
"""Skill: naming audit for the GAIA 3.0 rename (#1296).

Counts leftover old names, ignoring an allowlist of stable identifiers.
Usage: python name_audit.py [root]   Exit 1 if non-allowlisted leftovers exist.
"""
import re, subprocess, sys

PATTERN = re.compile(r'GAIAN? 2\.0|GAIA-2\.0|KyleAlexanderSteen/GAIA|GAIAN?_(?:GAIAN_)?2\.0')
ALLOW = [re.compile(p) for p in (r'PROOF-GAIA20', r'gaia20_', r'gaia-2-0')]


def audit(root='.'):
    files = subprocess.run(['git', '-C', root, 'ls-files', '-z'], capture_output=True, text=True).stdout.split('\0')
    hits = []
    for f in filter(None, files):
        try:
            text = open(f'{root}/{f}', encoding='utf-8').read()
        except (UnicodeDecodeError, OSError):
            continue
        for n, line in enumerate(text.splitlines(), 1):
            if PATTERN.search(line) and not any(a.search(line) for a in ALLOW):
                hits.append((f, n))
    return hits


if __name__ == '__main__':
    hits = audit(sys.argv[1] if len(sys.argv) > 1 else '.')
    by = {}
    for f, _ in hits:
        by[f] = by.get(f, 0) + 1
    for f, c in sorted(by.items(), key=lambda x: -x[1])[:15]:
        print(f'{c:5d}  {f}')
    print(f'{len(hits)} leftover line(s) in {len(by)} file(s)')
    sys.exit(1 if hits else 0)
