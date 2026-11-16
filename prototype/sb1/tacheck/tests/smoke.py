#!/usr/bin/env python3
"""Actual CLI controls; no latency/memory or production claim."""
import argparse
import csv
import hashlib
import json
from pathlib import Path
import subprocess


def sha(path):
    h = hashlib.sha256()
    with path.open('rb') as f:
        while data := f.read(1 << 20):
            h.update(data)
    return h.hexdigest()


def main():
    p = argparse.ArgumentParser()
    for name in ['checker', 'root', 'out']:
        p.add_argument('--' + name, type=Path, required=True)
    p.add_argument('--g1', type=Path)
    a = p.parse_args()
    a.out.mkdir(parents=True, exist_ok=False)
    fixtures = Path(__file__).resolve().parents[1] / 'fixtures'
    records = []
    for name in ['inactive-c17', 'active-c17', 'inactive-c18', 'active-c18']:
        case = a.out / name
        case.mkdir()
        source = fixtures / (name + '.pl.in')
        composition = case / 'composition.tsv'
        composition.write_text('# patient_documents\t0\n' + str(source) + '\n')
        cmd = [str(a.checker), str(a.root), str(composition)]
        (case / 'command.json').write_text(json.dumps(dict(argv=cmd, checker_sha256=sha(a.checker), source_sha256=sha(source), composition_sha256=sha(composition)), indent=2) + '\n')
        result = subprocess.run(cmd, capture_output=True, check=False)
        (case / 'stdout').write_bytes(result.stdout)
        (case / 'stderr').write_bytes(result.stderr)
        (case / 'rc').write_text(str(result.returncode) + '\n')
        active = name.startswith('active')
        if active:
            assert result.returncode == 1 and result.stdout == b'', (name, result.returncode)
            lines = result.stderr.decode().splitlines()
            assert lines[0] == 'refused (negative cycle)' and len(lines) == 2, (name, lines)
            head = 3 if name.endswith('c17') else 1
            label = f'{source}:{head}:{head}'
            assert lines[1] == label + '\t' + label + '\t-', (name, lines)
        else:
            assert result.returncode == 0 and result.stderr == b'', (name, result.returncode, result.stderr)
            assert result.stdout == b'admitted\tclauses=2\tedges=0\n', (name, result.stdout)
        records.append(dict(case=name, checker_sha256=sha(a.checker), source_sha256=sha(source), composition_sha256=sha(composition), rc=result.returncode, argv=json.dumps(cmd)))
        print(name + '\tPASS', flush=True)
    if a.g1:
        cmd = [str(a.checker), str(a.root), str(a.g1)]
        case = a.out / 'F1b-g001-s0'
        case.mkdir()
        (case / 'command.json').write_text(json.dumps(dict(argv=cmd, checker_sha256=sha(a.checker), composition_sha256=sha(a.g1)), indent=2) + '\n')
        with (case / 'stdout').open('wb') as out, (case / 'stderr').open('wb') as err:
            rc = subprocess.run(cmd, stdout=out, stderr=err, check=False).returncode
        (case / 'rc').write_text(str(rc) + '\n')
        assert rc == 0 and (case / 'stderr').read_bytes() == b'', rc
        assert (case / 'stdout').read_bytes() == b'admitted\tclauses=18873\tedges=151587\n'
        records.append(dict(case='F1b-g001-s0', checker_sha256=sha(a.checker), source_sha256=sha(a.g1), composition_sha256=sha(a.g1), rc=rc, argv=json.dumps(cmd)))
        print('F1b-g001-s0\tPASS', flush=True)
    with (a.out / 'commands.tsv').open('w', newline='') as stream:
        w = csv.DictWriter(stream, records[0].keys(), delimiter='\t', lineterminator='\n')
        w.writeheader()
        w.writerows(records)
    print(f'smoke PASS\tcases={len(records)}', flush=True)


if __name__ == '__main__':
    main()
