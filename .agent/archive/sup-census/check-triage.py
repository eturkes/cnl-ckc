#!/usr/bin/env python3
"""Validate a SUP2 triage file against the coverage ledger: rc 0 = complete, 1 = violations."""
import sys
from pathlib import Path

def main(root, path):
    cov = [l.split('\t') for l in Path(root, 'guidelines/cdc-2022-opioid/coverage.tsv').read_text().splitlines() if l and not l.startswith('#')]
    ids = [r[0] for r in cov]
    lines = Path(path).read_text().splitlines()
    bad = []
    head = 'region\tflag\tdependency\tnote'
    if not lines or lines[0] != head:
        bad.append('header')
    rows = [l.split('\t') for l in lines[1:]]
    if [r[0] for r in rows] != ids:
        bad.append(f'regions: {len(rows)} rows vs {len(ids)} coverage ids (order + set must match)')
    unknown = sum(1 for l in lines[1:] if 'unknown' in l.split('\t')[1:])
    for i, r in enumerate(rows, 2):
        if len(r) != 4:
            bad.append(f'line {i}: {len(r)} fields'); continue
        if r[1] not in ('candidate', 'clear', 'unknown'):
            bad.append(f'line {i}: flag {r[1]!r}')
        if r[1] == 'candidate' and (not r[2] or r[2] == 'unknown'):
            bad.append(f'line {i}: candidate without dependency')
    print(f'rows={len(rows)} unknown={unknown} candidates={sum(1 for r in rows if len(r) > 1 and r[1] == "candidate")} violations={len(bad)}')
    for b in bad[:20]:
        print(' ', b)
    return 0 if not bad and unknown == 0 else 1

sys.exit(main(sys.argv[1], sys.argv[2]))
