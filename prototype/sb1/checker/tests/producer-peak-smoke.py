#!/usr/bin/env python3
"""M1 real-server G1 correctness + lifetime smoke; no scored timing or acquisition."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import select
import signal
import subprocess
import time


def line(process, seconds=180):
    end = time.monotonic() + seconds
    data = bytearray()
    while not data.endswith(b'\n'):
        left = end - time.monotonic()
        assert left > 0 and select.select([process.stdout], [], [], left)[0], 'server protocol timeout'
        part = os.read(process.stdout.fileno(), 1)
        assert part, f'server EOF rc={process.poll()}'
        data += part
    return data.decode().rstrip('\n')


def table(path):
    return dict(line.split('\t', 1) for line in path.read_text().splitlines())


def hwm(pid):
    return int(next(line for line in Path(f'/proc/{pid}/status').read_text().splitlines()
                    if line.startswith('VmHWM:')).split()[1]) * 1024


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def run(server, label, args):
    out = args.out / label
    out.mkdir()
    instance = args.root / 'rust/target/bench/sb1-frozen/F1b-g001-s0'
    oracle = args.root / 'rust/target/bench/sb1-oracles/F1b-g001-s0/union'
    arm = out / 'arm'
    argv = [str(server), 'serve', '--provider', 'static', '--ta-receipts', 'none',
            '--producer', str(args.root / 'prototype/sb1/producer.prolog'),
            '--root', str(args.root), str(instance / 'composition.tsv'), str(arm)]
    (out / 'argv.json').write_text(json.dumps(argv, indent=2) + '\n')
    with (out / 'stderr').open('wb') as stderr:
        process = subprocess.Popen(argv, stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                   stderr=stderr, bufsize=0, start_new_session=True)
        reports, calls, startup = [], [], None
        try:
            reports.append(line(process))
            assert reports[-1] == 'ready'
            if label == 'after':
                startup = table(arm / 'producer-launcher.tsv')
                pid = int(startup['pid'])
                assert int(startup['pgid']) == process.pid == os.getpgid(pid)
                assert int(startup['vmhwm_bytes']) < 64 << 20
                assert int(startup['parent_vmhwm_bytes']) < 64 << 20
                assert startup['python_path'] == str(Path(f'/proc/{pid}/exe').resolve())
                assert startup['python_sha256'] == digest(Path(f'/proc/{pid}/exe'))
                script = args.worktree / 'prototype/sb1/checker/src/producer_launcher.py'
                assert startup['script_sha256'] == digest(script)
            for sequence, query in enumerate(['q-all.txt', 'q-one.pl', 'q-patient.pl'], 1):
                directory = out / f'call-{sequence}'
                process.stdin.write(f'{sequence}\t{instance / "queries" / query}\t{directory}\n'.encode())
                process.stdin.flush()
                reports.append(line(process))
                assert reports[-1] == f'done\t{sequence}\t0', reports[-1]
                expected = oracle / (Path(query).stem + '.out')
                actual = directory / 'answers.txt'
                cmp = subprocess.run(['cmp', '--', str(expected), str(actual)], capture_output=True)
                assert cmp.returncode == 0, (query, cmp.stdout, cmp.stderr)
                counters = [row.split('\t') for row in (directory / 'checker-counters.tsv').read_text().splitlines()[1:]]
                runtime = {name: int(value) for kind, name, value in counters if kind == 'runtime'}
                peak = runtime['producer_maxrss_bytes']
                row = dict(query=query, cmp_rc=cmp.returncode, expected_sha256=digest(expected),
                           actual_sha256=digest(actual), producer_pid=runtime['producer_pid'],
                           producer_maxrss_bytes=peak, server_vmhwm_bytes=hwm(process.pid),
                           selected_clauses=sum(1 for _ in (directory / 'selected-clauses.tsv').open()))
                if label == 'after':
                    report = table(directory / 'producer-report.tsv')
                    assert int(report['launcher_pid']) == int(startup['pid'])
                    assert int(report['sequence']) == sequence
                    assert int(report['producer_pid']) == runtime['producer_pid']
                    assert int(report['producer_start_ticks']) == runtime['producer_start_ticks'] > 0
                    assert int(report['producer_cpu_us']) == runtime['producer_cpu_us']
                    assert int(report['producer_maxrss_bytes']) == peak > 0
                    assert int(report['producer_wait4_maxrss_bytes']) == runtime['producer_wait4_maxrss_bytes'] > 0
                    assert int(report['producer_observed_hwm_bytes']) == runtime['producer_observed_hwm_bytes']
                    assert peak == max(int(report['producer_wait4_maxrss_bytes']), int(report['producer_observed_hwm_bytes']))
                    frame = b'sb1-call\0' + sequence.to_bytes(8, 'little') + os.fsencode(directory)
                    assert report['call_identity'] == hashlib.sha256(frame).hexdigest()
                    assert report['producer_exit_kind'] == 'exit' and int(report['producer_exit_status']) == 0
                    row['producer_start_ticks'] = runtime['producer_start_ticks']
                    row['producer_cpu_us'] = runtime['producer_cpu_us']
                    row['launcher_vmhwm_bytes'] = hwm(int(startup['pid']))
                    if query == 'q-one.pl':
                        assert peak < row['server_vmhwm_bytes'] and peak < 64 << 20
                calls.append(row)
            if startup:
                os.kill(int(startup['pid']), signal.SIGKILL)
                failure = out / 'call-launcher-killed'
                process.stdin.write(f'4\t{instance / "queries/q-one.pl"}\t{failure}\n'.encode()); process.stdin.flush()
                reports.append(line(process)); assert reports[-1] == 'done\t4\t2'
                assert (failure / 'call.stderr').read_text().startswith('launcher-report ')
                metrics = (failure / 'checker-counters.tsv').read_text()
                assert 'runtime\tlauncher_report_failure\t1\n' in metrics
                assert 'runtime\tproducer_maxrss_bytes\t' not in metrics
                assert 'runtime\tproducer_pid\t' not in metrics
            process.stdin.close()
            rc = process.wait(timeout=20)
            assert rc == 0 and (out / 'stderr').stat().st_size == 0
            if startup:
                assert not Path(f'/proc/{startup["pid"]}').exists(), 'launcher leaked past EOF'
                terminal_check = table(arm / 'launcher-terminal-check.tsv')
                assert terminal_check['verdict'] == 'unbounded' and terminal_check['own_bound_bytes'] == 'NA'
                assert not (arm / 'launcher-terminal.tsv').exists()  # Deliberate SIGKILL, not an EOF self-report.
        finally:
            if process.poll() is None:
                os.killpg(process.pid, signal.SIGKILL)
                process.wait()
            process.stdout.close()
    (out / 'protocol.txt').write_text('\n'.join(reports) + '\n')
    return dict(label=label, server=str(server), server_sha256=digest(server),
                calls=calls, startup=startup, eof_rc=rc)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ['server', 'before', 'root', 'worktree', 'out']:
        parser.add_argument('--' + name, type=Path, required=True)
    args = parser.parse_args()
    args.out.mkdir(parents=True, exist_ok=False)
    result = [run(args.before, 'before', args), run(args.server, 'after', args)]
    (args.out / 'summary.json').write_text(json.dumps(result, indent=2) + '\n')
    print(json.dumps(result, indent=2))
    print('M1 G1 smoke pass: 6/6 frozen oracle comparisons; one pre-load launcher; EOF reaped')


if __name__ == '__main__':
    main()
