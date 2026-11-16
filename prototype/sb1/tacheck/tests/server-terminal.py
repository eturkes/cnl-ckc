#!/usr/bin/env python3
"""TAS-P2 B1: real G1 server EOF / SIGKILL / reaped-512-MiB-child controls, not scored data."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import select
import shlex
import signal
import subprocess
import time


def kv(path):
    rows = [line.split('\t', 1) for line in path.read_text().splitlines()]
    assert len(dict(rows)) == len(rows), f'duplicate row: {path}'
    return dict(rows)


def start(pid):
    return int(Path(f'/proc/{pid}/stat').read_text().rsplit(')', 1)[1].split()[19])


def hwm(pid):
    return int(next(s for s in Path(f'/proc/{pid}/status').read_text().splitlines()
                    if s.startswith('VmHWM:')).split()[1]) * 1024


def line(proc):
    deadline = time.monotonic() + 300
    data = bytearray()
    while not data.endswith(b'\n'):
        left = deadline - time.monotonic()
        assert left > 0 and select.select([proc.stdout], [], [], left)[0], 'server timeout'
        part = os.read(proc.stdout.fileno(), 1)
        assert part, f'premature EOF rc={proc.poll()}'
        data += part
    return data.decode().rstrip('\n')


def control(args, case):
    out = args.out / case
    out.mkdir()
    arm = out / 'arm'
    instance = args.root / 'rust/target/bench/sb1-frozen/F1b-g001-s0'
    swipl = args.swipl
    if case == 'reaped-child':
        swipl = out / 'swipl-with-512m-child'
        swipl.write_text('#!/usr/bin/env bash\nset -e\ndd if=/dev/zero of=/dev/null bs=512M count=1 status=none\nexec '
                         + shlex.quote(str(args.swipl)) + ' "$@"\n')
        swipl.chmod(0o755)
    argv = [str(args.server), 'serve', '--provider', 'static', '--ta-receipts', 'none',
            '--swipl', str(swipl), '--producer', str(args.root / 'prototype/sb1/producer.prolog'),
            '--root', str(args.root), str(instance / 'composition.tsv'), str(arm)]
    (out / 'argv.json').write_text(json.dumps(argv, indent=2) + '\n')
    with (out / 'stderr').open('wb') as stderr:
        proc = subprocess.Popen(argv, stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=stderr,
                                start_new_session=True, bufsize=0)
        try:
            assert line(proc) == 'ready'
            ticks = start(proc.pid)
            observed = hwm(proc.pid)
            producer = None
            if case == 'reaped-child':
                call = out / 'call'
                proc.stdin.write(f'1\t{instance / "queries/q-one.pl"}\t{call}\n'.encode())
                proc.stdin.flush()
                assert line(proc) == 'done\t1\t0'
                producer = kv(call / 'producer-report.tsv')
                assert int(producer['producer_maxrss_bytes']) >= 512 << 20
                assert hashlib.sha256((call / 'answers.txt').read_bytes()).hexdigest() == '6adbe91dba206a09935dc7cbc9e9fab7eba8b9ccee46c8ac82f95e7704d817cc'
                observed = max(observed, hwm(proc.pid))
            if case == 'killed':
                os.killpg(proc.pid, signal.SIGKILL)
                rc = proc.wait(timeout=30)
                assert rc == -signal.SIGKILL
                assert not (arm / 'server-terminal.tsv').exists()
                assert not (arm / 'server-terminal-check.tsv').exists()
                return dict(case=case, pid=proc.pid, start_ticks=ticks, rc=rc, terminal='absent')
            proc.stdin.close()
            rc = proc.wait(timeout=30)
            assert rc == 0
            raw = kv(arm / 'server-terminal.tsv')
            checked = kv(arm / 'server-terminal-check.tsv')
            assert raw['format'] == 'sb1-server-terminal 1' and raw['resource_scope'] == 'self'
            assert raw['exit_kind'] == 'normal' and raw['exit_code'] == '0'
            for rows in (raw, checked):
                assert int(rows['pid']) == proc.pid and int(rows['start_ticks']) == ticks
            bound = int(checked['own_bound_bytes'])
            assert checked['format'] == 'sb1-server-terminal-check 1'
            assert checked['verdict'] == 'bounded'
            assert bound == max(int(raw['ru_maxrss_bytes']), int(raw['vmhwm_bytes']))
            assert observed <= bound <= 384 << 20, (observed, bound)
            assert (out / 'stderr').stat().st_size == 0
            return dict(case=case, pid=proc.pid, start_ticks=ticks, rc=rc, observed_hwm_bytes=observed,
                        raw=raw, checked=checked, producer=producer)
        finally:
            if proc.poll() is None:
                os.killpg(proc.pid, signal.SIGKILL)
                proc.wait()
            proc.stdout.close()
            if not proc.stdin.closed:
                proc.stdin.close()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ('root', 'server', 'swipl', 'out'):
        parser.add_argument('--' + name, type=Path, required=True)
    args = parser.parse_args()
    args.out.mkdir(parents=True, exist_ok=False)
    plan = dict(controls=['normal', 'killed', 'reaped-child'], child_min_bytes=512 << 20,
                server_max_bytes=384 << 20, bound_at_least_last_observation=True,
                kill_has_no_terminal_files=True, scored=False)
    (args.out / 'plan.json').write_text(json.dumps(plan, indent=2) + '\n')
    rows = []
    for case in plan['controls']:
        rows.append(control(args, case))
        (args.out / 'results.json').write_text(json.dumps(rows, indent=2) + '\n')
    print('server-terminal: normal bounded; SIGKILL absent; 512 MiB child excluded; 3/3 controls pass')


if __name__ == '__main__':
    main()
