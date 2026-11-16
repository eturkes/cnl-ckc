#!/usr/bin/env python3
"""T3 canonical parity. Stop on the first required nonempty delta, keep all bytes."""
import argparse
import csv
import hashlib
import itertools
import json
from pathlib import Path
import subprocess

NAMES = ['front-' + n + '.tsv' for n in ['values', 'unary', 'links', 'nodes', 'edges', 'scc']]
NAMES += ['flow-' + n + '.tsv' for n in ['base-nodes', 'base-edges', 'final-nodes', 'final-edges', 'outcome']]


def sha(path):
    with path.open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()


def table(path, rows):
    with path.open('w', newline='') as stream:
        writer = csv.DictWriter(stream, rows[0].keys(), delimiter='\t', lineterminator='\n')
        writer.writeheader(); writer.writerows(rows)


def digest(path):
    h = hashlib.sha256(); rows = size = 0; chunks = []
    with path.open('rb') as stream:
        while data := stream.read(64 << 20):
            chunks.append(hashlib.sha256(data).hexdigest())
            h.update(data); rows += data.count(b'\n'); size += len(data)
    return dict(sha256=h.hexdigest(), rows=rows, bytes=size, chunks=chunks)


def invoke(argv, out):
    out.mkdir(parents=True, exist_ok=False)
    (out / 'argv.json').write_text(json.dumps(argv, indent=2) + '\n')
    with (out / 'stdout').open('wb') as stdout, (out / 'stderr').open('wb') as stderr:
        try:
            rc = subprocess.run(argv, stdout=stdout, stderr=stderr, timeout=43200, check=False).returncode
        except subprocess.TimeoutExpired:
            rc = 124
    (out / 'rc').write_text(str(rc) + '\n')
    assert rc == 0, (out, rc)


def main():
    p = argparse.ArgumentParser()
    for key in ['root', 'oracle', 'census', 'checker', 'normalization', 'out']:
        p.add_argument('--' + key, type=Path, required=True)
    p.add_argument('--cases', nargs='+')
    a = p.parse_args()
    a.out.mkdir(parents=True, exist_ok=False)
    cases = a.cases or [f'{family}-g{g:03}-s0' for family in ['F1a', 'F1b', 'F2', 'F3', 'F4', 'MX'] for g in [1, 10]] + ['F1a-g010-s0-permuted']
    protocol = dict(cases=cases, oracle=str(a.oracle), oracle_sha256=sha(a.oracle), census=str(a.census), census_sha256=sha(a.census), checker=str(a.checker), checker_sha256=sha(a.checker), normalization=str(a.normalization), normalization_sha256=sha(a.normalization), already_shaped=False, censor_seconds=43200)
    (a.out / 'protocol.json').write_text(json.dumps(protocol, indent=2) + '\n')
    control = a.out / 'comparator-controls'; control.mkdir()
    left = control / 'expected'; left.write_bytes(b'a\nb\n')
    results = []
    for name, data, wanted in [('equal', b'a\nb\n', 0), ('missing', b'a\n', 1), ('extra', b'a\nb\nc\n', 1)]:
        path = control / name; path.write_bytes(data)
        rc = subprocess.run(['cmp', '-s', '--', str(left), str(path)], check=False).returncode
        results.append(dict(control=name, expected_rows=2, actual_rows=data.count(b'\n'), rc=rc, wanted=wanted))
        assert rc == wanted
    table(control / 'verdicts.tsv', results)
    comparisons, members, complements = [], [], []

    def compare(case, mode, name, left, right, required=True):
        l, r = digest(left), digest(right)
        rc = subprocess.run(['cmp', '-s', '--', str(left), str(right)], check=False).returncode
        assert rc in [0, 1], (case, mode, name, rc)
        row = dict(case=case, mode=mode, object=name, left=str(left), right=str(right), left_sha256=l['sha256'], right_sha256=r['sha256'], left_rows=l['rows'], right_rows=r['rows'], left_bytes=l['bytes'], right_bytes=r['bytes'], cmp_rc=rc, required=int(required))
        comparisons.append(row); table(a.out / 'comparisons.tsv', comparisons)
        chunk = a.out / case / (mode + '-' + name + '.chunks.json')
        chunk.write_text(json.dumps(dict(expected=l, actual=r), indent=2) + '\n')
        if rc and required:
            with left.open('rb') as ls, right.open('rb') as rs:
                for line, (expected, actual) in enumerate(itertools.zip_longest(ls, rs), 1):
                    if expected != actual:
                        (a.out / case / 'first-delta.json').write_text(json.dumps(dict(mode=mode, object=name, line=line, expected=None if expected is None else expected.decode(), actual=None if actual is None else actual.decode(), expected_rows=l['rows'], actual_rows=r['rows']), indent=2) + '\n')
                        break
            raise AssertionError((case, mode, name, 'first-delta.json'))

    for case in cases:
        permuted = case.endswith('-permuted')
        instance_name = case.removesuffix('-permuted')
        instance = a.root / 'rust/target/bench/sb1-frozen' / instance_name
        out = a.out / case; out.mkdir()
        composition = instance / 'composition.tsv'
        paths = [line for line in composition.read_text().splitlines() if line and not line.startswith('#')]
        patient = [int(line.split('\t')[1]) for line in (instance / 'manifest.tsv').read_text().splitlines() if line.startswith('patient_documents\t')]
        assert len(patient) == 1 and 0 < patient[0] < len(paths)
        if permuted:
            paths = list(reversed(paths[:-patient[0]])) + paths[-patient[0]:]
            composition = out / 'composition.tsv'
            composition.write_text(f'# patient_documents\t{patient[0]}\n' + ''.join(path + '\n' for path in paths))
        queries = [instance / 'queries' / name for name in ['q-all.txt', 'q-one.pl', 'q-patient.pl']]
        for position, path in enumerate([a.root / 'guidelines/schema.pl'] + [a.root / path for path in paths] + queries):
            members.append(dict(case=case, position=position, path=str(path), bytes=path.stat().st_size, sha256=sha(path)))
        table(a.out / 'input-members.tsv', members)
        for mode, binary, extra in [('seal', a.oracle, []), ('default', a.census, []), ('full', a.census, ['--dump-full-mixed'])]:
            target = out / mode / 'metrics'
            argv = [str(binary), '--root', str(a.root), '--composition', str(composition), '--out', str(target), '--dc', '--patient', 'last', '--redesign', '--dump-front', '--dump-analysis'] + extra
            invoke(argv, out / mode)
        argv = [str(a.checker), str(a.root), str(composition), str(out / 'checked' / 'metrics')]
        for query in queries:
            argv.extend(['--query', str(query)])
        invoke(argv, out / 'checked')
        for name in NAMES:
            compare(case, 'default-seal', name, out / 'seal/metrics' / name, out / 'default/metrics' / name)
            compare(case, 'independent-seal', name, out / 'seal/metrics' / name, out / 'checked/metrics' / name)
        compare(case, 'full-mixed', 'edges', out / 'full/metrics/flow-final-edges.tsv', out / 'checked/metrics/flow-final-full-edges.tsv')
        compare(case, 'full-mixed', 'nodes', out / 'full/metrics/flow-final-nodes.tsv', out / 'checked/metrics/flow-final-full-nodes.tsv')
        hint = out / 'checked/metrics/hint-final-full-edges.tsv'
        if hint.exists():
            compare(case, 'hint-diagnostic', 'edges', out / 'full/metrics/flow-final-edges.tsv', hint, False)
        values = dict(line.split('\t') for line in (out / 'checked/metrics/flow-complement.tsv').read_text().splitlines())
        complements.append(dict(case=case, projected=values['projected'], full=values['full'], complement=values['complement']))
        table(a.out / 'complements.tsv', complements)
        print(f'T3\t{case}\tC3=11 default+independent cmp0\tfull-mixed=2 cmp0\tcomplement={values["complement"]}\treach=3 equal', flush=True)
    print(f'T3 PASS cases={len(cases)} required_comparisons={sum(row["required"] for row in comparisons)}', flush=True)


if __name__ == '__main__':
    main()
