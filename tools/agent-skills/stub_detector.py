#!/usr/bin/env python3
"""Skill: stub detection (AI skills registry: debugging, code review; #1304).

Flags Rust functions that print a success line but contain a TODO and no real work.
Usage: python stub_detector.py [root]   Exit 1 if any stub is found.
"""
import re, sys, pathlib

SUCCESS = re.compile(r'(println!|print!)\s*\(\s*"[^"]*(\u2713|started|ready|success|done|queued)', re.I)
FN = re.compile(r'^\s*(?:pub\s+)?(?:async\s+)?fn\s+(\w+)', re.M)


def functions(src):
    """Yield (name, line, body) using brace matching."""
    for m in FN.finditer(src):
        i = src.find('{', m.end())
        if i < 0:
            continue
        depth, j = 0, i
        while j < len(src):
            depth += (src[j] == '{') - (src[j] == '}')
            j += 1
            if depth == 0:
                break
        yield m.group(1), src.count('\n', 0, m.start()) + 1, src[i:j]


def find_stubs(src):
    out = []
    for name, line, body in functions(src):
        if 'TODO' in body and SUCCESS.search(body) and 'not_implemented' not in body:
            out.append((name, line))
    return out


def scan(root):
    hits = []
    for p in pathlib.Path(root).rglob('*.rs'):
        if any(x in p.parts for x in ('target', '.git')):
            continue
        for name, line in find_stubs(p.read_text(errors='ignore')):
            hits.append((str(p), line, name))
    return hits


if __name__ == '__main__':
    hits = scan(sys.argv[1] if len(sys.argv) > 1 else '.')
    for f, l, n in hits:
        print(f'{f}:{l}: fn {n} prints success but has a TODO')
    print(f'{len(hits)} stub(s) found')
    sys.exit(1 if hits else 0)
