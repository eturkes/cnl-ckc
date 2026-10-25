#!/usr/bin/env python3
"""Validate a SUP2 research deliverable against its cluster: rc 0 = complete, 1 = violations."""
import re, sys
from pathlib import Path

HEAD = 'region\tverdict\tinstrument\teffective\tquote\turl\tpart\tnote'
VERDICTS = ('superseded', 'not-superseded', 'pre-publication', 'unclear')

def main(cluster, path):
    want = [l.split('\t')[0] for l in Path(cluster).read_text().splitlines()[1:]]
    lines = Path(path).read_text().splitlines()
    bad = []
    if not lines or lines[0] != HEAD:
        bad.append('header')
    rows = [l.split('\t') for l in lines[1:]]
    if [r[0] for r in rows] != want:
        bad.append(f'regions: {len(rows)} rows vs {len(want)} cluster rows (order + set must match)')
    unknown = sum(1 for r in rows if 'unknown' in r[1:])
    for i, r in enumerate(rows, 2):
        if len(r) != 8:
            bad.append(f'line {i}: {len(r)} fields'); continue
        reg, v, inst, eff, quote, url, part, note = r
        if v == 'unknown':
            continue
        if v not in VERDICTS:
            bad.append(f'line {i}: verdict {v!r}')
        if v in ('superseded', 'pre-publication'):
            if not re.fullmatch(r'\d{4}-\d{2}-\d{2}', eff):
                bad.append(f'line {i}: effective {eff!r}')
            for name, val in (('instrument', inst), ('quote', quote), ('url', url), ('part', part)):
                if val in ('', '-'):
                    bad.append(f'line {i}: {v} without {name}')
            if not url.startswith('https://'):
                bad.append(f'line {i}: url {url!r}')
        if v in ('not-superseded', 'unclear') and note in ('', '-'):
            bad.append(f'line {i}: {v} without note')
    counts = {v: sum(1 for r in rows if len(r) > 1 and r[1] == v) for v in VERDICTS}
    print(f'rows={len(rows)} unknown={unknown} ' + ' '.join(f'{k}={n}' for k, n in counts.items()) + f' violations={len(bad)}')
    for b in bad[:20]:
        print(' ', b)
    return 0 if not bad and unknown == 0 else 1

sys.exit(main(sys.argv[1], sys.argv[2]))
