#!/usr/bin/env python3
"""SUP2 census validator: triage ⊇ coverage, union = candidates, clusters partition it,
research rows complete, superseded set = rows.tsv. rc 0 = consistent."""
import subprocess, sys
from pathlib import Path

root = Path(sys.argv[1])
here = root / '.agent/archive/sup-census'
bad = []

def run(script, *args):
    r = subprocess.run([sys.executable, '-I', str(here / script), *map(str, args)], capture_output=True, text=True)
    if r.returncode != 0:
        bad.append(f'{script} {args[-1]}: {r.stdout.strip()}')

for n in (1, 2):
    run('check-triage.py', root, here / f'triage-{n}.tsv')
tri = [{l.split('\t')[0]: l.split('\t')[1] for l in (here / f'triage-{n}.tsv').read_text().splitlines()[1:]} for n in (1, 2)]
union = [r for r in tri[0] if tri[0][r] == 'candidate' or tri[1][r] == 'candidate']
cand = [l.split('\t')[0] for l in (here / 'candidates.tsv').read_text().splitlines()[1:]]
if cand != union:
    bad.append(f'candidates.tsv: {len(cand)} rows vs union {len(union)}')
clusters = {}
for k in ('law', 'fda', 'federal', 'society'):
    rows = [l.split('\t')[0] for l in (here / f'cluster-{k}.tsv').read_text().splitlines()[1:]]
    clusters[k] = rows
    run('check-research.py', here / f'cluster-{k}.tsv', here / f'research-{k}.tsv')
joined = sorted(r for rs in clusters.values() for r in rs)
if joined != sorted(cand):
    bad.append(f'clusters: {len(joined)} rows vs {len(cand)} candidates (partition)')
superseded = sorted(l.split('\t')[0] for k in clusters for l in (here / f'research-{k}.tsv').read_text().splitlines()[1:] if l.split('\t')[1] == 'superseded')
rows = [l.split('\t') for l in (here / 'rows.tsv').read_text().splitlines()[1:]]
if sorted(r[0] for r in rows) != superseded:
    bad.append(f'rows.tsv regions {sorted(r[0] for r in rows)} vs research superseded {superseded}')
cov = {l.split('\t')[0]: l.split('\t')[4] for l in (root / 'guidelines/cdc-2022-opioid/coverage.tsv').read_text().splitlines() if l and not l.startswith('#')}
for r in rows:
    region, ruling, kind, key = r[0], r[1], r[2], r[3]
    status = cov.get(region, '')
    if kind == 'supersession' and status != f'ace({key})':
        bad.append(f'{region}: supersession row names {key}, coverage {status!r}')
    if kind == 'superseded' and (key != region or status.startswith('ace(')):
        bad.append(f'{region}: superseded row on {status!r}')
print(f'census: regions={len(tri[0])} candidates={len(cand)} superseded={len(rows)} violations={len(bad)}')
for b in bad:
    print(' ', b)
sys.exit(1 if bad else 0)
