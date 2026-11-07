#!/usr/bin/env python3
"""Candidate-census validator controls on one generated case; original bytes stay unchanged."""
import argparse
import csv
import hashlib
from pathlib import Path
import runpy
import shutil
import tempfile

HERE = Path(__file__).resolve().parent
COLLECT = runpy.run_path(str(HERE / 'candidate-census'))['collect']


def table(path, rows):
    with path.open('w', newline='') as sink:
        writer = csv.DictWriter(sink, rows[0].keys(), delimiter='\t', lineterminator='\n')
        writer.writeheader(); writer.writerows(rows)


def read(path):
    with path.open() as stream: return list(csv.DictReader(stream, delimiter='\t'))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('case', type=Path)
    parser.add_argument('--out-parent', type=Path, required=True)
    args = parser.parse_args()
    original = {p: hashlib.sha256(p.read_bytes()).hexdigest() for p in args.case.glob('candidate-census/*.tsv')}
    args.out_parent.mkdir(parents=True, exist_ok=True)
    expected = sum(t['calls'] for t in COLLECT(args.case).values())
    assert expected > 0
    print(f'positive control expected calls={expected}', flush=True)
    with tempfile.TemporaryDirectory(dir=args.out_parent) as directory:
        out = Path(directory)
        shutil.copytree(args.case / 'candidate-census', out / 'candidate-census')
        for name in ['base-stats.tsv', 'k1-stats.tsv']: shutil.copyfile(args.case / name, out / name)
        assert sum(t['calls'] for t in COLLECT(out).values()) == expected
        def reject(label):
            try: COLLECT(out)
            except AssertionError: print(label + ': refused', flush=True)
            else: raise AssertionError(label + ': accepted a planted defect')
        calls = sorted((out / 'candidate-census').glob('*-calls.tsv'))[0]
        pristine = calls.read_bytes()
        data = read(calls); data[0]['native'] = str(int(data[0]['native']) + 1); table(calls, data)
        reject('changed native count')
        calls.write_bytes(pristine)
        data = read(calls); assert len(data) > 1; table(calls, data[:-1])
        reject('dropped request')
        calls.write_bytes(pristine)
        keys = calls.with_name(calls.name.replace('-calls.tsv', '-keys.tsv'))
        key_bytes = keys.read_bytes()
        data = read(keys); data[0]['goal'] = str(int(data[0]['goal']) + 1); table(keys, data)
        reject('changed exact key')
        keys.write_bytes(key_bytes)
        stats = out / 'base-stats.tsv'; stat_bytes = stats.read_bytes(); data = read(stats)
        for row in data:
            if row['key'] == 'header_requests': row['value'] = str(int(row['value']) + 1)
        table(stats, data); reject('independent request count')
        stats.write_bytes(stat_bytes)
        assert sum(t['calls'] for t in COLLECT(out).values()) == expected
    assert all(hashlib.sha256(p.read_bytes()).hexdigest() == digest for p, digest in original.items())
    print('candidate-census validator: 4 planted defects refused; restored green; original hashes unchanged')
    return 0


if __name__ == '__main__': raise SystemExit(main())
