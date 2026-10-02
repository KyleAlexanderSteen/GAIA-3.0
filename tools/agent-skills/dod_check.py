#!/usr/bin/env python3
"""Skill: definition-of-done check for PR bodies (#1319).

Rules: a '## Stage reached' section exists with at least one checked stage;
'Closes #N' requires OPERATIONAL checked. Reads the PR body on stdin or a file.
"""
import re, sys

STAGES = ['SPECIFICATION', 'IMPLEMENTATION', 'TEST', 'INTEGRATION', 'VERIFICATION', 'OPERATIONAL']


def check(body):
    problems = []
    m = re.search(r'^##\s*Stage reached.*?(?=^##\s|\Z)', body, re.S | re.M)
    if not m:
        return ['missing "## Stage reached" section']
    sec = m.group(0)
    checked = {s for s in STAGES if re.search(r'- \[[xX]\]\s*' + s, sec)}
    if not checked:
        problems.append('no stage is checked')
    if re.search(r'^\s*Closes\s+#\d+', body, re.M | re.I) and 'OPERATIONAL' not in checked:
        problems.append('uses "Closes #" but OPERATIONAL is not checked; use "Refs #"')
    return problems


if __name__ == '__main__':
    text = open(sys.argv[1]).read() if len(sys.argv) > 1 else sys.stdin.read()
    probs = check(text)
    for p in probs:
        print('FAIL:', p)
    print('ok' if not probs else f'{len(probs)} problem(s)')
    sys.exit(1 if probs else 0)
