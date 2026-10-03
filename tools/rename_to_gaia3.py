#!/usr/bin/env python3
"""Rename GAIA 2.0 to GAIA 3.0 across the repository (#1296, #1312).

Usage: python tools/rename_to_gaia3.py [--apply] [--keep-filenames]
Default is a dry run that prints what would change. Run from the repo root.

Rules, applied in order:
  KyleAlexanderSteen/GAIA-2.0 and KyleAlexanderSteen/GAIA-2.0 -> KyleAlexanderSteen/GAIA-3.0
  GAIA-2.0 -> GAIA-3.0, GAIAN 2.0 -> GAIAN 3.0, GAIA 2.0 -> GAIA 3.0

Left alone on purpose:
  - lines that record history (see HISTORY below), so attribution stays true
  - stable identifiers: PROOF-GAIA20, gaia20_, gaia-2-0 (they do not match the rules)
  - this script, tools/agent-skills/, .git, build output, binary files
File names containing the old name are renamed with git mv so links stay consistent.
"""
import re, subprocess, sys, pathlib

RULES = [
    (re.compile(r'Kyle Alexander SteenThe(?:Alchemist|Avatar)/GAIA-2\.0'), 'KyleAlexanderSteen/GAIA-3.0'),
    (re.compile(r'GAIA-2\.0'), 'GAIA-3.0'),
    (re.compile(r'GAIAN 2\.0'), 'GAIAN 3.0'),
    (re.compile(r'GAIA 2\.0'), 'GAIA 3.0'),
]
HISTORY = re.compile(r'Alchemist era|formerly|renamed from|previously|originally|legacy name|2\.0\s*(?:->|\u2192)\s*3\.0|was GAIA 2\.0', re.I)
SKIP_PREFIX = ('.git/', 'target/', 'node_modules/', 'tools/agent-skills/')
SKIP_FILES = {'tools/rename_to_gaia3.py'}


def convert(text):
    out, n = [], 0
    for line in text.split('\n'):
        new = line
        if not HISTORY.search(line):
            for rx, rep in RULES:
                new = rx.sub(rep, new)
        n += new != line
        out.append(new)
    return '\n'.join(out), n


def main(apply, keep_names):
    files = subprocess.run(['git', 'ls-files', '-z'], capture_output=True, text=True, check=True).stdout.split('\0')
    changed = lines = renamed = 0
    for f in filter(None, files):
        if f in SKIP_FILES or f.startswith(SKIP_PREFIX):
            continue
        p = pathlib.Path(f)
        try:
            text = p.read_text(encoding='utf-8')
        except (UnicodeDecodeError, OSError):
            continue
        new, n = convert(text)
        if n:
            changed += 1; lines += n
            if apply:
                p.write_bytes(new.encode('utf-8'))
        newname = f
        if not keep_names:
            for rx, rep in RULES:
                newname = rx.sub(rep, newname)
        if newname != f:
            renamed += 1
            if apply:
                pathlib.Path(newname).parent.mkdir(parents=True, exist_ok=True)
                subprocess.run(['git', 'mv', f, newname], check=True)
    mode = 'applied' if apply else 'dry run'
    print(f'{mode}: {lines} line(s) in {changed} file(s); {renamed} file(s) renamed')


if __name__ == '__main__':
    main('--apply' in sys.argv, '--keep-filenames' in sys.argv)
