#!/usr/bin/env python3
"""REV-P census validator: every recorded projection loss of the CDC guideline classed, every typable loss re-authored.
usage: check.py <repo-root> [<notes-rev>] → rc 0 = pass. <notes-rev> = the revision whose ledger is graded (default: the working tree).
Checks: census shape + classes (typable carrier | gap class of docs/m7-gap-taxonomy.md | no-loss `-`); the census segments of each
document = the dropped segments (`; `-split) of its row at the census base BASE; at the graded ledger a typable segment is gone
from its row's dropped column and the row's kept column records `REV-P:`, every other segment is still listed."""
import subprocess, sys
from collections import Counter
from pathlib import Path
BASE = '6761187c'
NOTES = 'guidelines/cdc-2022-opioid/audit/projection-notes.tsv'
HEAD = 'id\tdocid\tregion\tsegment\tclass\tcarrier\tbasis\truled_by'
TYPABLE = {'example-fact', 'event', 'condition', 'pp', 'anaphor', 'property', 'modal', 'order', 'negation', 'qualifier',
           'coordination', 'comparison', 'relative-clause'}
QUAL = {'usuality', 'ideality', 'focus', 'manner', 'force', 'possibility', 'applicability'}
GAPS = {'temporal', 'strength', 'regimen-choice', 'numeric-threshold', 'dose-arithmetic', 'derived-measure', 'probability',
        'condition-scope', 'quantification', 'relation-structure', 'supersession'}
root = Path(sys.argv[1]); rev = sys.argv[2] if len(sys.argv) > 2 else None
show = lambda r, p: subprocess.run(['git', '-C', str(root), 'show', f'{r}:{p}'], capture_output=True, text=True, check=True).stdout
def ledger(text):
    out = {}
    for l in text.split('\n'):
        f = l.split('\t')
        if l and not l.startswith('#') and len(f) == 4: out[f[0]] = (f[2], f[3].split('; '))
    return out
lines = (Path(__file__).parent / 'census.tsv').read_text(encoding='utf-8').splitlines()
bad = []
if not lines or lines[0] != HEAD: bad.append('census header')
rows = [l.split('\t') for l in lines[1:]]
ids = Counter(r[0] for r in rows)
bad += [f'duplicate id {i}' for i, n in ids.items() if n > 1]
for r in rows:
    if len(r) != 8: bad.append(f'{r[0]}: {len(r)} fields'); continue
    c, car = r[4], r[5]
    ok = (c == 'typable' and (car in TYPABLE or (car.startswith('qualifier:') and car[10:] in QUAL))) or \
         (c == 'carrier-less' and car in GAPS) or (c == 'no-loss' and car == '-')
    if not ok: bad.append(f'{r[0]}: class {c} carrier {car}')
    if r[7] not in ('readers', 'main') or not r[6].strip(): bad.append(f'{r[0]}: basis / ruled_by')
base = ledger(show(BASE, NOTES))
now = ledger(show(rev, NOTES) if rev else (root / NOTES).read_text(encoding='utf-8'))
by_doc = {}
for r in rows: by_doc.setdefault(r[1], []).append(r)
for d, (_, segs) in base.items():
    got = [r[3] for r in by_doc.get(d, [])]
    if Counter(got) != Counter(segs): bad.append(f'{d}: census segments ≠ base dropped segments')
bad += [f'{d}: census document absent from the base ledger' for d in by_doc if d not in base]
for d, rs in by_doc.items():
    if d not in now: bad.append(f'{d}: absent from the graded ledger'); continue
    kept, segs = now[d]
    for r in rs:
        if r[4] == 'typable':
            if r[3] in segs: bad.append(f'{r[0]}: typable segment still listed as dropped')
            if 'REV-P:' not in kept: bad.append(f'{r[0]}: kept column records no REV-P re-authoring')
        elif r[3] not in segs: bad.append(f'{r[0]}: {r[4]} segment no longer listed')
cls = Counter(r[4] for r in rows); gaps = Counter(r[5] for r in rows if r[4] == 'carrier-less')
print(f'revp-census: {len(rows)} segments over {len(by_doc)} documents; ' + ', '.join(f'{k} {v}' for k, v in sorted(cls.items())))
print('  carrier-less by gap class: ' + ', '.join(f'{k} {v}' for k, v in gaps.most_common()))
for b in bad[:40]: print('  FAIL', b)
print(f'revp-census: {"ok" if not bad else f"{len(bad)} failures"}')
sys.exit(1 if bad else 0)
