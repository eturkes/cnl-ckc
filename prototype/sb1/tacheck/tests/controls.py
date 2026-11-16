#!/usr/bin/env python3
"""T1: frozen expected outcomes + independent original-edge witnesses."""
import argparse
import csv
import hashlib
import json
from pathlib import Path
import subprocess


OBJECTS = ['front-' + n + '.tsv' for n in ['values', 'unary', 'links', 'nodes', 'edges', 'scc']]
OBJECTS += ['flow-' + n + '.tsv' for n in ['base-nodes', 'base-edges', 'final-nodes', 'final-edges', 'outcome']]


def sha(path):
    h = hashlib.sha256()
    with path.open('rb') as f:
        while data := f.read(1 << 20):
            h.update(data)
    return h.hexdigest()


def rows(path):
    with path.open() as f:
        return list(csv.DictReader(f, delimiter='\t'))


def raw(path):
    with path.open() as f:
        return list(csv.reader(f, delimiter='\t'))


def table(path, values):
    with path.open('w', newline='') as f:
        w = csv.DictWriter(f, values[0].keys(), delimiter='\t', lineterminator='\n')
        w.writeheader()
        w.writerows(values)


def invoke(cmd, out):
    out.mkdir(parents=True, exist_ok=False)
    (out / 'argv.json').write_text(json.dumps(cmd, indent=2) + '\n')
    with (out / 'stdout').open('wb') as stdout, (out / 'stderr').open('wb') as stderr:
        try:
            rc = subprocess.run(cmd, stdout=stdout, stderr=stderr, check=False, timeout=43200).returncode
        except subprocess.TimeoutExpired:
            rc = 124
    (out / 'rc').write_text(str(rc) + '\n')
    return rc


def label(text):
    path, line, ordinal = text.rsplit(':', 2)
    assert int(line) > 0 and int(ordinal) > 0
    return path, int(ordinal)


def witness(candidate, oracle, expected, allowed):
    lines = (candidate / 'stderr').read_text().splitlines()
    assert lines[0] == f'refused ({expected} cycle)', lines[:1]
    cycle = [line.split('\t') for line in lines[1:]]
    assert cycle, 'empty program-cycle claim'
    if expected == 'negative':
        nodes = {(r[1], int(r[2])): int(r[0]) for r in raw(oracle / 'front-nodes.tsv')}
        edges = {(int(r[0]), int(r[1]), r[2]) for r in raw(oracle / 'front-edges.tsv')}
        for r in cycle:
            assert len(r) == 3 and r[2] in ['+', '-'], r
            a, b = label(r[0]), label(r[1])
            assert a[0] in allowed and b[0] in allowed
            assert (nodes[a], nodes[b], 'true' if r[2] == '-' else 'false') in edges, r
        assert any(r[2] == '-' for r in cycle)
    else:
        edges = {tuple(r) for name in ['flow-base-edges.tsv', 'flow-final-edges.tsv'] for r in raw(oracle / name)}
        supports = {(r['from'], r['to'], r['clause'], r['variable']) for r in rows(oracle / 'growth-edges.tsv')}
        for r in cycle:
            assert len(r) == 4 and label(r[2])[0] in allowed, r
            assert (r[0], r[1]) in edges, r
            assert tuple(r) in supports, r
    assert all(a[1] == b[0] for a, b in zip(cycle, cycle[1:] + cycle[:1])), cycle
    return len(cycle)


def main():
    p = argparse.ArgumentParser()
    for name in ['checker', 'oracle', 'root', 'out']:
        p.add_argument('--' + name, type=Path, required=True)
    p.add_argument('--only-independent', action='store_true')
    a = p.parse_args()
    a.out.mkdir(parents=True, exist_ok=False)
    local = Path(__file__).resolve().parents[1]
    expected_path = local / 'fixtures/controls.tsv'
    (a.out / 'expected.tsv').write_bytes(expected_path.read_bytes())
    (a.out / 'protocol.json').write_text(json.dumps(dict(expected_sha256=sha(expected_path), checker_sha256=sha(a.checker), oracle_sha256=sha(a.oracle), already_shaped=True, only_independent=a.only_independent, censor_seconds=43200), indent=2) + '\n')
    expected = rows(expected_path)
    assert len(expected) == 20 and len({r['case'] for r in expected}) == 20
    inputs = a.out / 'inputs'
    inputs.mkdir()
    cdc = sorted((a.root / 'guidelines/cdc-2022-opioid/pl').glob('*.pl'))
    assert len(cdc) == 492
    if not a.only_independent:
        original = inputs / 'cdc-original.tsv'
        original.write_text('# patient_documents\t0\n' + ''.join(str(p) + '\n' for p in cdc))
        cmd = [str(a.oracle), '--root', str(a.root), '--composition', str(original), '--patient', 'none', '--dc', '--shape-only', '--emit', str(inputs / 'cdc-shaped'), '--out', str(a.out / 'shape-metrics')]
        assert invoke(cmd, a.out / 'shape-command') == 0
        cdc = [Path(p) for p in (inputs / 'cdc-shaped/composition.tsv').read_text().splitlines() if p and not p.startswith('#')]
        assert len(cdc) == 492
    commands, outcomes, members = [], [], []
    for row in expected:
        name, wanted = row['case'], row['expected']
        independent = name.endswith(('c17', 'c18'))
        if a.only_independent and not independent:
            continue
        paths, patient = list(cdc), 0
        if independent:
            paths = [local / f'fixtures/{name}.pl.in']
        elif name in ['adult', 'minor', 'cancer']:
            paths += [a.root / f'tests/consumer/{name}.pl.in']
            patient = 1
        elif name == 'dose':
            paths += [a.root / 'tests/queries/green/dose-arithmetic/tree/guidelines/fx/pl/world.pl']
            patient = 1
        elif name != 'cdc':
            plant = a.root / 'prototype/sb1/ta/plants'
            paths += [plant / (name + '-' + side + '.pl.in') for side in ['A', 'B']] if name in ['L5', 'P5'] else [plant / (name + '.pl.in')]
        composition = inputs / (name + '.tsv')
        composition.write_text(f'# patient_documents\t{patient}\n' + ''.join(str(p) + '\n' for p in paths))
        for i, path in enumerate(paths):
            members.append(dict(case=name, position=i + 1, path=str(path), bytes=path.stat().st_size, sha256=sha(path), patient=int(i >= len(paths) - patient)))
        table(a.out / 'input-members.tsv', members)
        oracle = a.out / name / 'oracle'
        oracle_cmd = [str(a.oracle), '--root', str(a.root), '--composition', str(composition), '--patient', 'last', '--out', str(oracle), '--dump-front', '--dump-analysis', '--edges']
        # invoke owns its log directory; the census creates metrics separately.
        logs = a.out / name / 'oracle-command'
        oracle_rc = invoke(oracle_cmd, logs)
        assert oracle_rc == (0 if wanted == 'admitted' else 1), (name, 'oracle', oracle_rc)
        candidate = a.out / name / 'candidate'
        cmd = [str(a.checker), str(a.root), str(composition), '--already-shaped']
        candidate_rc = invoke(cmd, candidate)
        assert candidate_rc == (0 if wanted == 'admitted' else 1), (name, 'checker', candidate_rc)
        for mode, binary, argv, rc in [('oracle', a.oracle, oracle_cmd, oracle_rc), ('checker', a.checker, cmd, candidate_rc)]:
            commands.append(dict(case=name, mode=mode, binary_sha256=sha(binary), composition_sha256=sha(composition), rc=rc, argv=json.dumps(argv)))
        table(a.out / 'commands.tsv', commands)
        if wanted == 'admitted':
            assert (candidate / 'stderr').read_bytes() == b''
            report = (candidate / 'stdout').read_text()
            count = len(raw(oracle / 'front-nodes.tsv'))
            edges = len(raw(oracle / 'front-edges.tsv'))
            assert report == f'admitted\tclauses={count}\tedges={edges}\n', (name, report, count, edges)
            steps = 0
        else:
            assert (candidate / 'stdout').read_bytes() == b''
            steps = witness(candidate, oracle, wanted, {str(p) for p in paths})
        outcomes.append(dict(case=name, expected=wanted, oracle_rc=oracle_rc, checker_rc=candidate_rc, witness_steps=steps, verdict='pass'))
        table(a.out / 'outcomes.tsv', outcomes)
        print(f'T1\t{name}\t{wanted}\tPASS\twitness_steps={steps}', flush=True)
    print(f'T1 PASS\tcases={len(outcomes)}', flush=True)


if __name__ == '__main__':
    main()
