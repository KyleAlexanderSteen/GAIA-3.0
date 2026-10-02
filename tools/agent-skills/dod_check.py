#!/usr/bin/env python3
"""Skill: definition-of-done check for PR bodies (#1319).

Rules: a '## Stage reached' section exists with at least one checked stage;
checked stages are contiguous from SPECIFICATION; the highest checked stage has
an evidence bullet ('- STAGE: link or test name') under 'Evidence:' in the stage
section or in a '## Evidence' section; GitHub closing keywords (close/fix/resolve
and their forms) require OPERATIONAL. Reads a PR body on stdin or from a file.
"""
import re, sys

STAGES = ['SPECIFICATION', 'IMPLEMENTATION', 'TEST', 'INTEGRATION', 'VERIFICATION', 'OPERATIONAL']
CLOSING_WORDS = r'close(?:s|d)?|fix(?:es|ed)?|resolve(?:s|d)?'


def _evidence_text(body, sec):
    m = re.search(r'^##\s*Evidence\s*$.*?(?=^##\s|\Z)', body, re.S | re.M)
    return sec + (m.group(0) if m else '')


def check(body):
    problems = []
    m = re.search(r'^##\s*Stage reached.*?(?=^##\s|\Z)', body, re.S | re.M)
    if not m:
        return ['missing "## Stage reached" section']
    sec = m.group(0)
    checked = {s for s in STAGES if re.search(r'^- \[[xX]\]\s*' + s + r'\b', sec, re.M)}
    if not checked:
        return ['no stage is checked']

    highest = max(STAGES.index(s) for s in checked)
    missing = [s for s in STAGES[:highest + 1] if s not in checked]
    if missing:
        problems.append('checked stages must be contiguous from SPECIFICATION; missing: ' + ', '.join(missing))

    top = STAGES[highest]
    if not re.search(r'^\s*[-*]\s+' + top + r'\s*:\s*\S', _evidence_text(body, sec), re.M):
        problems.append('missing evidence bullet "- ' + top + ': <link or test name>" under "Evidence:" or in a "## Evidence" section')

    if re.search(r'(?i)(?:^|\s)(?:' + CLOSING_WORDS + r')\s+#\d+\b', body) and 'OPERATIONAL' not in checked:
        problems.append('uses a GitHub closing keyword but OPERATIONAL is not checked; use "Refs #"')
    return problems


if __name__ == '__main__':
    text = open(sys.argv[1]).read() if len(sys.argv) > 1 else sys.stdin.read()
    probs = check(text)
    for p in probs:
        print('FAIL:', p)
    print('ok' if not probs else f'{len(probs)} problem(s)')
    sys.exit(1 if probs else 0)
