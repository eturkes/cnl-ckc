#!/usr/bin/env python3
"""B4 development: actual warm provider calls vs frozen full-program answers."""
import argparse
import csv
import hashlib
import json
import os
from pathlib import Path
import select
import signal
import subprocess
import time


def sha(path):
    h = hashlib.sha256()
    with path.open('rb') as stream:
        while data := stream.read(1 << 20):
            h.update(data)
    return h.hexdigest()


def table(path, rows):
    with path.open('w', newline='') as stream:
        writer = csv.DictWriter(stream, rows[0].keys(), delimiter='\t', lineterminator='\n')
        writer.writeheader()
        writer.writerows(rows)


def line(process, seconds):
    end = time.monotonic() + seconds
    value = bytearray()
    while True:
        left = end - time.monotonic()
        assert left > 0, 'protocol censor'
        assert select.select([process.stdout], [], [], left)[0], 'protocol censor'
        c = os.read(process.stdout.fileno(), 1)
        assert c, f'server exited {process.poll()}'
        value.extend(c)
        if c == b'\n':
            return value.decode().rstrip('\n')


def run(argv, out):
    out.mkdir(parents=True)
    (out / 'argv.json').write_text(json.dumps(argv, indent=2) + '\n')
    with (out / 'stdout').open('wb') as stdout, (out / 'stderr').open('wb') as stderr:
        result = subprocess.run(argv, stdout=stdout, stderr=stderr, check=False, timeout=43200)
    (out / 'rc').write_text(str(result.returncode) + '\n')
    assert result.returncode == 0, (out, result.returncode)


def main():
    parser = argparse.ArgumentParser()
    for name in ['root', 'worktree', 'server', 'static', 'census', 'out']:
        parser.add_argument('--' + name, type=Path, required=True)
    parser.add_argument('--cases', nargs='+', default=['F1b-g001-s0', 'F1a-g001-s0', 'F1b-g010-s0', 'F1a-g010-s0'])
    args = parser.parse_args()
    args.out.mkdir(parents=True, exist_ok=False)
    (args.out / 'protocol.json').write_text(json.dumps(dict(
        cases=args.cases, server=str(args.server), server_sha256=sha(args.server),
        static=str(args.static), static_sha256=sha(args.static), census=str(args.census), census_sha256=sha(args.census),
        root=str(args.root), already_shaped=False, receipts='none', ta_run=False,
        role='B4 development correctness; no resource grade', censor_seconds=43200,
    ), indent=2) + '\n')
    answers, counts = [], []
    query_names = ['q-all.txt', 'q-one.pl', 'q-patient.pl']
    for case in args.cases:
        instance = args.root / 'rust/target/bench/sb1-frozen' / case
        expected = args.root / 'rust/target/bench/sb1-oracles' / case / 'union'
        queries = [instance / 'queries' / name for name in query_names]
        for mode, binary in [('typed', args.server), ('static', args.static)]:
            out = args.out / case / mode
            out.mkdir(parents=True)
            arm = out / 'arm'
            argv = [str(binary), 'serve', '--provider', 'static', '--ta-receipts', 'none', '--producer', str(args.root / 'prototype/sb1/producer.prolog'), '--root', str(args.root), str(instance / 'composition.tsv'), str(arm)]
            (out / 'argv.json').write_text(json.dumps(argv, indent=2) + '\n')
            with (out / 'stderr').open('wb') as stderr, (out / 'stdout').open('w') as stdout:
                p = subprocess.Popen(argv, stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=stderr, start_new_session=True, bufsize=0)
                try:
                    report = line(p, 43200)
                    stdout.write(report + '\n'); stdout.flush()
                    assert report == 'ready', report
                    for index, query in enumerate(queries, 1):
                        call = out / f'call-{index}'
                        p.stdin.write(f'{index}\t{query}\t{call}\n'.encode()); p.stdin.flush()
                        report = line(p, 43200)
                        stdout.write(report + '\n'); stdout.flush()
                        assert report == f'done\t{index}\t0', (case, mode, report)
                        key = query.stem
                        oracle = expected / (key + '.out')
                        actual = call / 'answers.txt'
                        cmp = subprocess.run(['cmp', '--', str(oracle), str(actual)], capture_output=True, check=False)
                        answers.append(dict(case=case, mode=mode, query=key, query_sha256=sha(query), expected=str(oracle), actual=str(actual), expected_sha256=sha(oracle), actual_sha256=sha(actual), cmp_rc=cmp.returncode))
                        table(args.out / 'answers.tsv', answers)
                        assert cmp.returncode == 0, (case, mode, key, cmp.stdout, cmp.stderr)
                        ids = (call / 'selected-clauses.tsv').read_text().splitlines()
                        counts.append(dict(case=case, mode=mode, query=key, selected=len(ids), identity_file=str(call / 'selected-clauses.tsv'), sha256=sha(call / 'selected-clauses.tsv')))
                        table(args.out / 'counts.tsv', counts)
                    p.stdin.close()
                    rc = p.wait(timeout=120)
                    (out / 'rc').write_text(str(rc) + '\n')
                    assert rc == 0
                    assert (out / 'stderr').stat().st_size == 0
                finally:
                    if p.poll() is None:
                        os.killpg(p.pid, signal.SIGKILL)
                        p.wait()
                    p.stdout.close()
                    if not p.stdin.closed:
                        p.stdin.close()
            print(f'B4\t{case}\t{mode}\tanswers=3 cmp0', flush=True)
        census_out = args.out / case / 'census'
        argv = [str(args.census), '--root', str(args.root), '--composition', str(instance / 'composition.tsv'), '--dc', '--patient', 'last', '--redesign', '--out', str(census_out / 'metrics'), '--dump-analysis']
        for query in queries:
            argv.extend(['--query', str(query)])
        argv.extend(['--slice-dir', str(census_out / 'slices')])
        run(argv, census_out)
        for query in queries:
            name = query.name.replace('.', '_').replace('-', '_')
            path = census_out / 'metrics' / ('slice-' + name + '.tsv')
            count = sum(1 for _ in path.open())
            counts.append(dict(case=case, mode='concrete-census', query=query.stem, selected=count, identity_file=str(path), sha256=sha(path)))
        table(args.out / 'counts.tsv', counts)
        print(f'B4\t{case}\tconcrete-census\tcounts=3', flush=True)
    assert len(answers) == len(args.cases) * 6
    print(f'B4 PASS\tcases={len(args.cases)}\tanswers={len(answers)} cmp0\tcounts={len(counts)}', flush=True)


if __name__ == '__main__':
    main()
