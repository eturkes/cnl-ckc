#!/usr/bin/env python3
"""Frozen T3x byte comparison; canonicalization stays in ta-objects diagnostics."""
import argparse
import importlib.util
import itertools
import json
from pathlib import Path
import subprocess
import sys
from typing import Any

sys.dont_write_bytecode = True
SPEC = importlib.util.spec_from_file_location('tas_frozen_parity', Path(__file__).with_name('parity.py'))
assert SPEC is not None and SPEC.loader is not None
PARITY = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(PARITY)
NORMALIZATION_SHA = '49aa4cb84ab9398daa2df8fe4d5f0095417f0d621dbaf992cf9538e40b4ab251'
FRONT = dict(rows=54_164_700, bytes=1_440_496_243, sha256='03c8466f1719993572e73763cd02c79e5b4a1e02ee96d8bfb5faa4a57f285bc1')


def save(path, value):
    path.write_text(json.dumps(value, indent=2) + '\n')


def pair(left: Path, right: Path, out: Path) -> tuple[int, dict[str, Any]]:
    out.mkdir(parents=True, exist_ok=False)
    expected, actual = PARITY.digest(left), PARITY.digest(right)
    rc = subprocess.run(['cmp', '-s', '--', str(left), str(right)], check=False).returncode
    assert rc in [0, 1], ('cmp tool failure', rc, left, right)
    result = dict(left=str(left), right=str(right), expected=expected, actual=actual,
                  cmp_rc=rc, row_delta=actual['rows'] - expected['rows'],
                  byte_delta=actual['bytes'] - expected['bytes'])
    save(out / 'comparison.json', result)
    if rc:
        with left.open('rb') as ls, right.open('rb') as rs:
            for line, (lrow, rrow) in enumerate(itertools.zip_longest(ls, rs), 1):
                if lrow != rrow:
                    save(out / 'first-delta.json', dict(line=line, expected=None if lrow is None else lrow.decode(),
                         actual=None if rrow is None else rrow.decode(), left=str(left), right=str(right),
                         expected_rows=expected['rows'], actual_rows=actual['rows']))
                    break
    return rc, result


def controls(out):
    out.mkdir(parents=True, exist_ok=False)
    expected = out / 'expected'; expected.write_bytes(b'a\nb\n')
    rows = []
    for name, data, wanted in [('equal', b'a\nb\n', 0), ('missing', b'a\n', 1), ('extra', b'a\nb\nc\n', 1)]:
        actual = out / name; actual.write_bytes(data)
        for direction, left, right in [('forward', expected, actual), ('reverse', actual, expected)]:
            target = out / (name + '-' + direction)
            argv = [sys.executable, '-I', str(Path(__file__).resolve()), 'pair', str(left), str(right), str(target)]
            with (out / (name + '-' + direction + '.log')).open('wb') as log:
                rc = subprocess.run(argv, stdout=log, stderr=subprocess.STDOUT, check=False).returncode
            assert rc == wanted, (name, direction, rc, wanted)
            result = json.loads((target / 'comparison.json').read_text())
            assert result['expected']['rows'] > 0
            assert result['cmp_rc'] == wanted
            assert (target / 'first-delta.json').exists() == bool(wanted)
            rows.append(dict(control=name, direction=direction, expected_rows=result['expected']['rows'],
                             actual_rows=result['actual']['rows'], rc=rc, wanted=wanted))
    PARITY.table(out / 'verdicts.tsv', rows)
    print('T3x comparator controls: 6/6 equal/missing/extra, both directions', flush=True)


def compare(a):
    a.out.mkdir(parents=True, exist_ok=False)
    controls(a.out / 'controls')
    assert PARITY.sha(a.normalization) == NORMALIZATION_SHA
    manifest = json.loads(a.manifest.read_text())
    command, = [c for c in manifest['commands'] if c['id'] == 'T3x']
    oracle = Path(command['oracle'])
    assert len(command['compares']) == 8
    # D1d rematch precedes any oracle/candidate comparison. The final diagnostic
    # ELF applies the same OWN-base projection already graded at G <= 10.
    front = PARITY.digest(oracle / 'front-edges.tsv')
    save(a.out / 'oracle-front-rematch.json', front)
    assert all(front[key] == value for key, value in FRONT.items()), ('D1d object identity', front)
    save(a.out / 'protocol.json', dict(manifest=str(a.manifest), manifest_sha256=PARITY.sha(a.manifest),
         normalization=str(a.normalization), normalization_sha256=NORMALIZATION_SHA,
         normalization_owner='frozen diagnostics.rs canonical printers + OWN-base D1f(i) projection; no Python rewrite',
         parity_sha256=PARITY.sha(Path(__file__).with_name('parity.py')), objects=command['compares']))
    rows = []
    for name in command['compares']:
        rc, result = pair(oracle / name, a.metrics / name, a.out / name)
        rows.append(dict(object=name, expected_rows=result['expected']['rows'], actual_rows=result['actual']['rows'],
                         expected_bytes=result['expected']['bytes'], actual_bytes=result['actual']['bytes'],
                         expected_sha256=result['expected']['sha256'], actual_sha256=result['actual']['sha256'], cmp_rc=rc))
        PARITY.table(a.out / 'comparisons.tsv', rows)
        print(f'T3x\t{name}\tcmp={rc}\trows={result["expected"]["rows"]}/{result["actual"]["rows"]}', flush=True)
        if rc:
            save(a.out / 'result.json', dict(verdict='mismatch', compared=len(rows), first_object=name,
                 first_delta=str(a.out / name / 'first-delta.json')))
            return 1
    save(a.out / 'result.json', dict(verdict='equal', compared=len(rows), required=8))
    return 0


if __name__ == '__main__':
    p = argparse.ArgumentParser()
    modes = p.add_subparsers(dest='mode', required=True)
    s = modes.add_parser('selftest'); s.add_argument('out', type=Path)
    s = modes.add_parser('pair')
    for key in ['left', 'right', 'out']:
        s.add_argument(key, type=Path)
    s = modes.add_parser('compare')
    for key in ['manifest', 'normalization', 'metrics', 'out']:
        s.add_argument('--' + key, type=Path, required=True)
    a = p.parse_args()
    if a.mode == 'selftest':
        controls(a.out)
    elif a.mode == 'pair':
        raise SystemExit(pair(a.left, a.right, a.out)[0])
    else:
        raise SystemExit(compare(a))
