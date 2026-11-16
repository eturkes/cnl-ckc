#!/usr/bin/env python3
"""Prospective TAS acquisition wrapper. Run inside the fixed scope+lock+heavy.
Only host records decide validity/replacement; resource/correctness outcomes stay.
"""
import argparse
import csv
import json
import os
from pathlib import Path
import select
import signal
import subprocess
import time

GIB28 = 28 * (1 << 30)
MEM_AVAILABLE = 34_000_000_000
PERIOD_NS = 1_000_000_000
CENSOR_NS = 43_200_000_000_000


def pressure(path, kind):
    for line in path.read_text().splitlines():
        fields = line.split()
        if fields[0] == kind:
            return float(dict(item.split('=') for item in fields[1:])['avg10'])
    raise ValueError(f'missing PSI {path} {kind}')


def fields(path):
    rows = (line.split(maxsplit=1) for line in path.read_text().splitlines())
    return {row[0]: row[1] if len(row) == 2 else '' for row in rows if row}


def cgroup():
    rows = Path('/proc/self/cgroup').read_text().splitlines()
    row = [line.removeprefix('0::') for line in rows if line.startswith('0::')]
    assert len(row) == 1
    cg = Path('/sys/fs/cgroup') / row[0].lstrip('/')
    assert int((cg / 'memory.max').read_text()) == GIB28, 'wrong construction fence'
    assert (cg / 'memory.swap.max').read_text().strip() == '0', 'swap fence absent'
    return cg


def sample(cg):
    values = [int(v) for v in Path('/proc/stat').read_text().splitlines()[0].split()[1:9]]
    vm = fields(Path('/proc/vmstat'))
    available = [line.split()[1] for line in Path('/proc/meminfo').read_text().splitlines() if line.startswith('MemAvailable:')]
    assert len(values) == 8 and len(available) == 1
    return dict(monotonic_ns=time.monotonic_ns(), unix_ns=time.time_ns(), total_jiffies=sum(values), idle_jiffies=values[3] + values[4], own_usage_usec=int(fields(cg / 'cpu.stat')['usage_usec']), cpu_some_avg10=pressure(Path('/proc/pressure/cpu'), 'some'), memory_full_avg10=pressure(Path('/proc/pressure/memory'), 'full'), mem_available_bytes=int(available[0]) * 1024, pswpin=int(vm['pswpin']), pswpout=int(vm['pswpout']))


class Validity:
    def __init__(self):
        self.previous = None
        self.duration = 0
        self.foreign_integral = 0.0
        self.host_integral = 0.0
        self.own_integral = 0.0
        self.samples = 0
        self.memory_max = 0.0
        self.available_last = None

    def add(self, row):
        self.samples += 1
        self.memory_max = max(self.memory_max, row['memory_full_avg10'])
        self.available_last = row['mem_available_bytes']
        host = own = foreign = None
        if self.previous is not None:
            old = self.previous
            dt = row['monotonic_ns'] - old['monotonic_ns']
            total = row['total_jiffies'] - old['total_jiffies']
            idle = row['idle_jiffies'] - old['idle_jiffies']
            usage = row['own_usage_usec'] - old['own_usage_usec']
            assert dt > 0 and total > 0 and 0 <= idle <= total and usage >= 0, 'nonmonotone host counters'
            host = 8.0 * (1.0 - idle / total)
            own = usage * 1000.0 / dt
            foreign = host - own
            self.duration += dt
            self.foreign_integral += foreign * dt
            self.host_integral += host * dt
            self.own_integral += own * dt
        self.previous = row
        return dict(row, host_busy_threads=host, own_threads=own, foreign_threads=foreign)

    def result(self, pre=False):
        mean = self.foreign_integral / self.duration if self.duration else None
        complete = self.samples >= 2 and self.duration > 0
        if pre:
            complete = complete and self.duration >= 60 * PERIOD_NS
        valid = complete and mean <= 3.0 and self.memory_max <= 1.0
        if pre:
            valid = valid and self.available_last >= MEM_AVAILABLE
        return dict(samples=self.samples, duration_ns=self.duration, foreign_mean_threads=mean, host_mean_threads=self.host_integral / self.duration if self.duration else None, own_mean_threads=self.own_integral / self.duration if self.duration else None, memory_full_max_avg10=self.memory_max, mem_available_last_bytes=self.available_last, verdict='valid' if valid else 'invalid-external')


def roster(cg, stream, label):
    now = time.monotonic_ns()
    for pid in sorted(set((cg / 'cgroup.procs').read_text().split())):
        try:
            root = Path('/proc') / pid
            stat = (root / 'stat').read_text().rsplit(')', 1)[1].split()
            status = fields(root / 'status')
            rss = int(status['VmRSS:'].split()[0]) * 1024
            hwm = int(status['VmHWM:'].split()[0]) * 1024
            command = (root / 'cmdline').read_bytes().replace(b'\0', b' ').decode(errors='replace').strip()
            stream.write(f'{label}\t{now}\t{pid}\t{stat[19]}\t{rss}\t{hwm}\t{command}\n')
        except (FileNotFoundError, ProcessLookupError, KeyError):
            stream.write(f'{label}\t{now}\t{pid}\tgone\tNA\tNA\tNA\n')
    stream.flush()


def status(out, name):
    with (out / (name + '.stdout')).open('wb') as stdout, (out / (name + '.stderr')).open('wb') as stderr:
        child = subprocess.Popen(['heavy', '--status'], stdout=stdout, stderr=stderr)
        _, code, usage = os.wait4(child.pid, 0)
        child.returncode = os.waitstatus_to_exitcode(code)
        result = dict(rc=child.returncode, pid=child.pid, wait4_peak_bytes=usage.ru_maxrss * 1024)
        assert child.returncode == 0, ('heavy status failed', result)
    (out / (name + '.json')).write_text(json.dumps(result, indent=2) + '\n')


def tick(cg, writer, aggregate, process_stream, phase):
    row = aggregate.add(sample(cg))
    writer.writerow(dict(phase=phase, **row))
    roster(cg, process_stream, phase)


def wait_tick(next_tick, pidfd=None):
    left = max(0.0, (next_tick - time.monotonic_ns()) / 1e9)
    select.select([] if pidfd is None else [pidfd], [], [], left)


def run(a):
    assert os.cpu_count() == 8, 'TAS shared-host regime requires eight threads'
    cg = cgroup()
    a.out.mkdir(parents=True, exist_ok=False)
    (a.out / 'protocol.json').write_text(json.dumps(dict(argv=a.command, cgroup=str(cg), memory_max_bytes=GIB28, memory_swap_max_bytes=0, sample_period_ns=PERIOD_NS, observation_censor_ns=CENSOR_NS, idle='idle+iowait; first eight /proc/stat counters', foreign='signed host minus own; monotonic-time-weighted mean', replacements=1, launcher_pid=os.getpid()), indent=2) + '\n')
    status(a.out, 'heavy-start')
    attempts = []
    for attempt in [1, 2]:
        out = a.out / f'attempt-{attempt}'
        out.mkdir()
        with (out / 'host-samples.tsv').open('w', newline='') as stream, (out / 'scope-processes.tsv').open('w') as processes:
            header = ['phase', 'monotonic_ns', 'unix_ns', 'total_jiffies', 'idle_jiffies', 'own_usage_usec', 'cpu_some_avg10', 'memory_full_avg10', 'mem_available_bytes', 'pswpin', 'pswpout', 'host_busy_threads', 'own_threads', 'foreign_threads']
            writer = csv.DictWriter(stream, header, delimiter='\t', lineterminator='\n')
            writer.writeheader()
            processes.write('phase\tmonotonic_ns\tpid\tstart_ticks\trss_bytes\tvmhwm_bytes\tcommand\n')
            windows = []
            while True:
                pre = Validity()
                next_tick = time.monotonic_ns()
                while pre.duration < 60 * PERIOD_NS:
                    wait_tick(next_tick)
                    tick(cg, writer, pre, processes, f'pre-{len(windows) + 1}')
                    stream.flush()
                    next_tick = pre.previous['monotonic_ns'] + PERIOD_NS if pre.samples == 1 else next_tick + PERIOD_NS
                result = pre.result(pre=True); windows.append(result)
                (out / 'prewindows.json').write_text(json.dumps(windows, indent=2) + '\n')
                if result['verdict'] == 'valid':
                    break
            argv = [arg.replace('@RUN@', str(out / 'run')) for arg in a.command]
            assert '@RUN@' in a.command, 'command must name the run destination placeholder'
            (out / 'argv.json').write_text(json.dumps(argv, indent=2) + '\n')
            before = fields(cg / 'memory.events')
            active = Validity()
            tick(cg, writer, active, processes, 'attempt')
            started = time.monotonic_ns(); next_tick = started + PERIOD_NS
            censored = False
            with (out / 'command.stdout').open('wb') as stdout, (out / 'command.stderr').open('wb') as stderr:
                child = subprocess.Popen(argv, stdout=stdout, stderr=stderr, start_new_session=True)
                fd = os.pidfd_open(child.pid)
                try:
                    while child.poll() is None:
                        if time.monotonic_ns() - started >= CENSOR_NS:
                            censored = True
                            os.killpg(child.pid, signal.SIGKILL)
                            break
                        wait_tick(next_tick, fd)
                        if time.monotonic_ns() >= next_tick:
                            tick(cg, writer, active, processes, 'attempt')
                            stream.flush(); next_tick += PERIOD_NS
                    rc = child.wait()
                finally:
                    os.close(fd)
                    if child.poll() is None:
                        os.killpg(child.pid, signal.SIGKILL); child.wait()
            tick(cg, writer, active, processes, 'attempt')
            stream.flush()
            result = active.result()
            result.update(attempt=attempt, command_rc=rc, observation_censored=censored, oom_kill_delta=int(fields(cg / 'memory.events')['oom_kill']) - int(before['oom_kill']), memory_peak_bytes=int((cg / 'memory.peak').read_text()), memory_scope_note='kernel physical/cgroup peak, not a substitute for summed RSS or the T7 bound')
            (out / 'validity.json').write_text(json.dumps(result, indent=2) + '\n')
            attempts.append(result)
            (a.out / 'attempts.json').write_text(json.dumps(attempts, indent=2) + '\n')
        if result['verdict'] == 'valid':
            break
    status(a.out, 'heavy-end')
    # Only validity selects the attempt. Valid slowness, OOM, refusal or censor
    # stays a result. The grading rows read their actual resource/output files.
    selected = next((r['attempt'] for r in attempts if r['verdict'] == 'valid'), None)
    (a.out / 'selection.json').write_text(json.dumps(dict(selected_attempt=selected, verdict='valid-acquisition' if selected is not None else 'hold-invalid-external'), indent=2) + '\n')
    print(json.dumps(dict(selected_attempt=selected, attempts=attempts)))
    return 0 if selected is not None else 1


def scope_control(a):
    import sys
    assert os.cpu_count() == 8
    cg = cgroup()
    a.out.mkdir(parents=True, exist_ok=False)
    control = 'two-thread-attribution' if a.attribution_control else 'precheck-positive-red'
    (a.out / 'protocol.json').write_text(json.dumps(dict(control=control, cgroup=str(cg), memory_max_bytes=GIB28, memory_swap_max_bytes=0, idle='idle+iowait; guests excluded', sample_period_ns=PERIOD_NS), indent=2) + '\n')
    status(a.out, 'heavy-start')
    burners = []
    usage = []
    aggregate = Validity()
    try:
        if a.attribution_control:
            burners = [subprocess.Popen([sys.executable, '-I', '-c', 'while True: pass'], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL) for _ in range(2)]
        with (a.out / 'host-samples.tsv').open('w', newline='') as stream, (a.out / 'scope-processes.tsv').open('w') as processes:
            writer = csv.DictWriter(stream, ['phase', 'monotonic_ns', 'unix_ns', 'total_jiffies', 'idle_jiffies', 'own_usage_usec', 'cpu_some_avg10', 'memory_full_avg10', 'mem_available_bytes', 'pswpin', 'pswpout', 'host_busy_threads', 'own_threads', 'foreign_threads'], delimiter='\t', lineterminator='\n')
            writer.writeheader()
            processes.write('phase\tmonotonic_ns\tpid\tstart_ticks\trss_bytes\tvmhwm_bytes\tcommand\n')
            target = (30 if a.attribution_control else 60) * PERIOD_NS
            next_tick = time.monotonic_ns()
            while aggregate.duration < target:
                wait_tick(next_tick)
                tick(cg, writer, aggregate, processes, control)
                stream.flush()
                next_tick = aggregate.previous['monotonic_ns'] + PERIOD_NS if aggregate.samples == 1 else next_tick + PERIOD_NS
    finally:
        for child in burners:
            child.terminate()
        for child in burners:
            _, status_code, resource = os.wait4(child.pid, 0)
            child.returncode = os.waitstatus_to_exitcode(status_code)
            usage.append(dict(pid=child.pid, rc=child.returncode, cpu_seconds=resource.ru_utime + resource.ru_stime, peak_bytes=resource.ru_maxrss * 1024))
    result = aggregate.result(pre=not a.attribution_control)
    if a.attribution_control:
        passed = aggregate.duration >= 30 * PERIOD_NS and 1.9 <= result['own_mean_threads'] <= 2.1
    else:
        passed = result['verdict'] == 'invalid-external' and result['foreign_mean_threads'] > 3.0
    result.update(control=control, control_verdict='pass' if passed else 'red', burners=usage, sample_cpuweight=(cg / 'cpu.weight').read_text().strip())
    (a.out / 'control.json').write_text(json.dumps(result, indent=2) + '\n')
    status(a.out, 'heavy-end')
    print(json.dumps(result))
    return 0 if passed else 1


def selftest():
    def grade(foreign=3.0, memory=1.0, available=MEM_AVAILABLE):
        a = Validity()
        a.add(dict(monotonic_ns=0, total_jiffies=0, idle_jiffies=0, own_usage_usec=0, memory_full_avg10=memory, mem_available_bytes=available))
        # 800 total jiffies / s, 100 Hz, eight threads; one owned thread.
        busy = foreign + 1
        a.add(dict(monotonic_ns=60 * PERIOD_NS, total_jiffies=48000, idle_jiffies=48000 * (1 - busy / 8), own_usage_usec=60_000_000, memory_full_avg10=memory, mem_available_bytes=available))
        return a.result(pre=True)['verdict']
    assert grade() == 'valid'
    assert grade(foreign=3.0001) == 'invalid-external'
    assert grade(memory=1.0001) == 'invalid-external'
    assert grade(available=MEM_AVAILABLE - 1) == 'invalid-external'
    assert grade(foreign=0.0, memory=0.0, available=MEM_AVAILABLE + 1) == 'valid'
    print('validity selftest: 5/5 boundary/control cases')


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--selftest', action='store_true')
    parser.add_argument('--precheck-once', action='store_true')
    parser.add_argument('--attribution-control', action='store_true')
    parser.add_argument('--out', type=Path)
    parser.add_argument('command', nargs=argparse.REMAINDER)
    args = parser.parse_args()
    if args.selftest:
        selftest()
    elif args.precheck_once or args.attribution_control:
        assert args.out is not None and args.precheck_once != args.attribution_control and not args.command
        raise SystemExit(scope_control(args))
    else:
        assert args.out is not None and args.command[:1] == ['--']
        args.command = args.command[1:]
        raise SystemExit(run(args))
