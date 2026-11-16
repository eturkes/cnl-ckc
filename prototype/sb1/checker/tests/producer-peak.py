#!/usr/bin/env python3
"""TAS-P2 M1 controls, not an acquisition. Fixed payload512MiB; small≤64MiB; b2 at+5ms between50ms samples; CPU burn50ms."""
import argparse
import copy
import hashlib
import json
import os
from pathlib import Path
import select
import signal
import struct
import subprocess
import sys
import time

MIB = 1 << 20
ALLOCATION = 512 * MIB


def hwm(pid):
    for line in Path(f'/proc/{pid}/status').read_text().splitlines():
        if line.startswith('VmHWM:'):
            return int(line.split()[1]) * 1024
    return 0


def until(ns):
    while time.monotonic_ns() < ns:
        select.select([], [], [], max(0, (ns - time.monotonic_ns()) / 1e9))


def line(process):
    end = time.monotonic() + 20
    data = bytearray()
    while not data.endswith(b'\n'):
        left = end - time.monotonic()
        assert left > 0 and select.select([process.stdout], [], [], left)[0], 'launcher-report timeout'
        part = os.read(process.stdout.fileno(), 1)
        assert part, 'launcher-report missing'
        data += part
        assert len(data) <= 4096, 'launcher-report long'
    return data[:-1].decode().split('\t')


class Launcher:
    def __init__(self, script, out):
        self.out, self.sequence = out, 0
        self.created = time.monotonic_ns()
        self.terminal = out / 'launcher-terminal.tsv'
        self.process = subprocess.Popen([sys.executable, '-I', '-c', script.read_text(), str(self.terminal)], stdin=subprocess.PIPE,
                                        stdout=subprocess.PIPE, stderr=(out / 'launcher.stderr').open('wb'), bufsize=0)
        self.start = line(self.process)
        assert self.start[:2] == ['ready', '1'] and int(self.start[2]) == self.process.pid
        self.start_hwm, self.pids = hwm(self.process.pid), []

    def begin(self, args):
        self.sequence += 1
        self.identity = hashlib.sha256(b'sb1-call\0' + struct.pack('<Q', self.sequence) + os.fsencode(self.out)).hexdigest()
        fields = [self.identity.encode(), os.fsencode(self.out / f'{self.sequence}.stdout'),
                  os.fsencode(self.out / f'{self.sequence}.stderr'), *map(os.fsencode, args)]
        payload = struct.pack('<QI', self.sequence, len(fields))
        for field in fields:
            payload += struct.pack('<I', len(field)) + field
        self.requested = time.monotonic_ns()
        self.process.stdin.write(payload); self.process.stdin.flush()
        self.started = line(self.process)
        assert self.started[:3] == ['started', str(self.sequence), self.identity], 'launcher-report started'
        pid, ticks = map(int, self.started[3:])
        stat = Path(f'/proc/{pid}/stat').read_text().rsplit(')', 1)[1].split()
        assert int(stat[1]) == self.process.pid and int(stat[19]) == ticks
        self.observed = hwm(pid)
        self.process.stdin.write(f'ack\t{self.sequence}\t{self.identity}\t{pid}\t{ticks}\t{self.observed}\n'.encode())
        self.process.stdin.flush(); self.pids.append(self.process.pid)
        return pid

    def end(self):
        report = line(self.process)
        (self.out / f'{self.sequence}.report.tsv').write_text('\t'.join(report)+'\n')
        assert report[:5] == ['exited', *self.started[1:]], 'launcher-report identity'
        kind, status, wait4_peak, peak, cpu_us, observed = report[5:]
        status, wait4_peak, peak, cpu_us, observed = map(int, [status, wait4_peak, peak, cpu_us, observed])
        assert wait4_peak > 0 and peak % 1024 == 0 and peak == max(wait4_peak, self.observed, observed), 'launcher-report peak'
        assert kind == ('exit' if os.WIFEXITED(status) else 'signal')
        assert line(self.process) == ['complete', str(self.sequence), self.identity], 'launcher-report duplicate/end'
        ended = time.monotonic_ns()
        return dict(pid=int(report[3]), start_ticks=int(report[4]), call_identity=self.identity, kind=kind,
                    raw_status=status, peak_bytes=peak, producer_wait4_maxrss_bytes=wait4_peak, producer_cpu_us=cpu_us, observed_hwm_bytes=max(self.observed, observed),
                    requested_ns=self.requested, ended_ns=ended, duration_ns=ended-self.requested,
                    stdout=(self.out / f'{self.sequence}.stdout').read_text())

    def run(self, args):
        self.begin(args)
        return self.end()

    def children(self):
        return list(map(int, Path(f'/proc/{self.process.pid}/task/{self.process.pid}/children').read_text().split()))

    def sample(self):
        began = time.monotonic_ns(); children = self.children(); processes = []
        for pid in [self.process.pid, *children]:
            values = dict(line.split(':', 1) for line in Path(f'/proc/{pid}/status').read_text().splitlines())
            rss = int(values['VmRSS'].split()[0]) * 1024
            processes.append(dict(pid=pid, rss_bytes=rss, hwm_bytes=int(values['VmHWM'].split()[0])*1024))
        return dict(monotonic_ns=began, end_ns=time.monotonic_ns(), producer_pids=children,
                    processes=processes, summed_rss_bytes=sum(row['rss_bytes'] for row in processes))

    def close(self):
        end_hwm = hwm(self.process.pid); self.process.stdin.close(); rc = self.process.wait(timeout=20); self.process.stdout.close()
        assert rc == 0
        terminal = dict(line.split('\t', 1) for line in self.terminal.read_text().splitlines())
        assert terminal['format'] == 'sb1-launcher-terminal 1' and terminal['resource_scope'] == 'self'
        assert int(terminal['pid']) == self.process.pid and int(terminal['start_ticks']) == int(self.start[4])
        assert terminal['exit_kind'] == 'normal' and terminal['exit_code'] == '0'
        raw, own = int(terminal['ru_maxrss_bytes']), int(terminal['vmhwm_bytes'])
        assert raw > 0 and own > 0 and end_hwm <= max(raw, own) <= 64 * MIB, 'launcher own terminal bound'
        return dict(pid=self.process.pid, start_ticks=int(self.start[4]), pgid=int(self.start[3]), startup_hwm_bytes=int(self.start[5]),
                    before_hwm_bytes=self.start_hwm, after_hwm_bytes=end_hwm, created_ns=self.created, call_launcher_pids=self.pids,
                    terminal_record=terminal, terminal_bound_bytes=max(raw, own), eof_rc=rc, absent_after_eof=not Path(f'/proc/{self.process.pid}').exists())


def direct():
    child = subprocess.Popen(['true']); pid, status, usage = os.wait4(child.pid, 0); child.returncode = os.waitstatus_to_exitcode(status)
    assert pid == child.pid
    return dict(pid=pid, raw_status=status, peak_bytes=int(usage.ru_maxrss)*1024)


def record(args):
    out = args.out.resolve(); out.mkdir(parents=True, exist_ok=False)
    launcher = Launcher(args.script, out)
    result = dict(parent_pid=os.getpid(), parent_pgid=os.getpgrp(), script=str(args.script.resolve()), child=str(args.child.resolve()),
                  small=launcher.run(['true']), direct_before=direct(), protocol='M1-D12; fixed b2 payload512MiB at+5ms between50ms samples')
    result['allocation_begin_ns'] = time.monotonic_ns()
    big = bytearray(2 << 30)
    for at in range(0, len(big), 4096): big[at] = 1
    result['parent_hwm_bytes'] = hwm(os.getpid()); result['large'] = launcher.run(['true']); result['direct_after'] = direct()
    del big
    result['after_free'] = launcher.run(['true']); result['direct_after_free'] = direct()
    cached = out / 'cached.bin'
    try:
        with cached.open('wb') as stream:
            block = b'\x01' * MIB
            for _ in range(512): stream.write(block)
        child = str(args.child.resolve())
        result['allocation'] = launcher.run([child, str(cached), '100', '0'])
        first = launcher.sample(); t0 = first['monotonic_ns']
        until(t0 + 5_000_000)
        result['short'] = launcher.run([child, str(cached), '0', '0'])
        until(t0 + 50_000_000)
        second = launcher.sample()
        result['subperiod_samples'] = [first, second]
        if result['short']['duration_ns'] >= 50_000_000 or result['short']['ended_ns'] >= t0 + 50_000_000:
            result['launcher'] = launcher.close(); result['qualification'] = 'non-diagnostic b2 premise absent; stop'
            (out / 'observations.json').write_text(json.dumps(result, indent=2)+'\n')
            raise RuntimeError(result['qualification'])
        result['normal'] = launcher.run(['true']); result['refused'] = launcher.run(['false'])
        result['cpu'] = launcher.run([child, 'cpu'])
        pid = launcher.begin([child, str(cached), '10000', '0']); target = out / f'{launcher.sequence}.stdout'; end = time.monotonic()+10
        while not target.exists() or not target.stat().st_size:
            assert time.monotonic()<end, 'allocation ready timeout'
            select.select([], [], [], 0.001)
        result['deadline_child_pgid'] = os.getpgid(pid); until(time.monotonic_ns()+100_000_000); os.kill(pid, signal.SIGKILL)
        result['deadline'] = launcher.end(); result['launcher'] = launcher.close()
    finally:
        cached.unlink(missing_ok=True)
        try:
            if launcher.process.poll() is None:
                result['launcher'] = launcher.close()
            elif 'launcher' not in result:
                result['launcher_failure_rc'] = launcher.process.returncode
        except Exception as error:
            result['cleanup_error'] = str(error)
        (out / 'observations.json').write_text(json.dumps(result, indent=2)+'\n')
    print(json.dumps(result, indent=2)); grade(result)


def grade(original, plant=None):
    data = copy.deepcopy(original)
    if plant == 'direct': data['large']['peak_bytes'] = data['direct_after']['peak_bytes']
    elif plant == 'allocation': data['allocation']['peak_bytes'] = ALLOCATION-1
    elif plant == 'short': data['short']['duration_ns'] = 50_000_000
    elif plant == 'sample': data['subperiod_samples'][0]['producer_pids'] = [data['short']['pid']]
    elif plant == 'rss': data['subperiod_samples'][0]['summed_rss_bytes'] = 0
    elif plant == 'status': data['refused']['raw_status'] = 0
    elif plant == 'zero': data['deadline']['peak_bytes'] = 0
    elif plant == 'cpu': data['cpu']['producer_cpu_us'] = 49_999
    elif plant == 'group': data['launcher']['pgid'] += 1
    elif plant == 'lifetime': data['launcher']['call_launcher_pids'][1] += 1
    elif plant == 'startup': data['launcher']['created_ns'] = data['allocation_begin_ns']+1
    elif plant == 'observed': data['small']['observed_hwm_bytes'] = data['small']['peak_bytes']+1024
    elif plant == 'max': data['small']['observed_hwm_bytes'] = data['small']['producer_wait4_maxrss_bytes']+1024; data['small']['peak_bytes'] = data['small']['producer_wait4_maxrss_bytes']
    elif plant == 'missing-terminal': data['launcher']['terminal_record'] = None
    elif plant == 'foreign-terminal': data['launcher']['terminal_record']['start_ticks'] = str(data['launcher']['start_ticks']+1)
    elif plant == 'terminal-child': data['launcher']['terminal_record']['ru_maxrss_bytes'] = str(data['allocation']['producer_wait4_maxrss_bytes'])
    assert data['parent_hwm_bytes'] >= 2 << 30, 'A3 parent allocation'
    for name in ['small', 'large', 'after_free']:
        assert 0 < data[name]['peak_bytes'] <= 64*MIB, f'A3 small bound: {name}'
    for name in ['allocation', 'short', 'deadline']:
        assert data[name]['peak_bytes'] >= ALLOCATION, f'A4/A5 allocation underreported: {name}'
        own = json.loads(data[name]['stdout']); assert own['allocation_bytes'] == ALLOCATION
        assert data[name]['peak_bytes'] >= own['own_hwm_bytes'] >= ALLOCATION, f'A4 same-producer HWM: {name}'
        assert own['pgid'] == data['parent_pgid'], 'A7 producer group'
    for name in ['small', 'large', 'after_free', 'allocation', 'short', 'normal', 'refused', 'cpu', 'deadline']:
        assert data[name]['producer_wait4_maxrss_bytes'] > 0
        assert data[name]['peak_bytes'] == max(data[name]['producer_wait4_maxrss_bytes'], data[name]['observed_hwm_bytes']), f'A4 raw/observed maximum: {name}'
    a,b = data['subperiod_samples']; short = data['short']
    assert short['duration_ns'] < 50_000_000 and a['monotonic_ns'] < short['requested_ns'] < short['ended_ns'] < a['monotonic_ns']+50_000_000 <= b['monotonic_ns'], 'A4 short lifetime/phasing'
    assert not a['producer_pids'] and not b['producer_pids'], 'A4 between-samples premise'
    assert a['end_ns'] < short['requested_ns'] and short['ended_ns'] < b['monotonic_ns']
    for sample in [a, b]:
        assert [row['pid'] for row in sample['processes']] == [data['launcher']['pid']]
        assert sample['summed_rss_bytes'] == sum(row['rss_bytes'] for row in sample['processes']) > 0, 'A4 positive RSS sample'
    for name,status in [('normal',0),('refused',256),('deadline',9)]:
        assert data[name]['raw_status'] == status and data[name]['peak_bytes'] > 0, f'A5 {name}'
    burn = json.loads(data['cpu']['stdout'])['burn_cpu_ns']; assert burn >= 50_000_000 and data['cpu']['producer_cpu_us']*1000 >= burn, 'A5 CPU burn'
    launcher=data['launcher']; assert launcher['created_ns'] < data['allocation_begin_ns'], 'A7 pre-load startup'
    assert 0 < launcher['startup_hwm_bytes'] <= 64*MIB and 0 < launcher['after_hwm_bytes'] <= 64*MIB, 'A7 small launcher'
    assert launcher['call_launcher_pids'] and set(launcher['call_launcher_pids']) == {launcher['pid']}, 'A7 one launcher'
    assert launcher['pgid'] == data['parent_pgid'] == data['deadline_child_pgid'], 'A7 launcher group'
    assert launcher['eof_rc'] == 0 and launcher['absent_after_eof'], 'A7 EOF cleanup'
    terminal = launcher['terminal_record']; assert terminal is not None, 'D12 missing terminal record'
    assert len(terminal) == 8 and terminal['format'] == 'sb1-launcher-terminal 1' and terminal['resource_scope'] == 'self'
    assert int(terminal['pid']) == launcher['pid'] and int(terminal['start_ticks']) == launcher['start_ticks'], 'D12 foreign terminal record'
    raw_self, own_hwm = int(terminal['ru_maxrss_bytes']), int(terminal['vmhwm_bytes'])
    assert raw_self > 0 and own_hwm > 0 and max(raw_self, own_hwm) == launcher['terminal_bound_bytes'], 'D12 own bound source'
    assert launcher['after_hwm_bytes'] <= launcher['terminal_bound_bytes'] <= 64*MIB, 'D12 own bound excludes children'
    assert terminal['exit_kind'] == 'normal' and terminal['exit_code'] == '0'
    print('M1-D12 controls pass A3 A4(b1,b2) A5(status,CPU) A7(own terminal)')


def main():
    parser=argparse.ArgumentParser(description=__doc__); sub=parser.add_subparsers(dest='mode', required=True); run=sub.add_parser('record')
    for name in ['script','child','out']: run.add_argument('--'+name,type=Path,required=True)
    check=sub.add_parser('grade'); check.add_argument('input',type=Path)
    check.add_argument('--plant',choices=['direct','allocation','short','sample','rss','status','zero','cpu','group','lifetime','startup','observed','max','missing-terminal','foreign-terminal','terminal-child'])
    args=parser.parse_args()
    if args.mode=='record': record(args)
    else: grade(json.loads(args.input.read_text()),args.plant)


if __name__=='__main__': main()
