#!/usr/bin/env python3
"""Detached frozen-vector controller. Host-only selection owns replacement/stop."""
import argparse
import csv
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import time

BASE = 'c2cdf9bb3c9b0d141e467a91e452cc690c0c617d'
FREEZE_SHA = '301879c1702e4fbd2d7ec74c87c10da8ae9eeada7cc8bb5253d5833fe6790a41'
SOURCE_SHA = '8673c4f70dc389c5755d99ebf1e3d8ac221378c034a24068f24cf11e6169304d'
ARTIFACT_SHA = 'f9dbfaa1342a02a9a99f10056a92508491417a108ed3f23eed980cd4bef453e3'
BINARY_SHA = '3158c63c230ad3f7f3602e8c1e0a2a5836ad04cf08859506aa6707ff19420bf7'


def sha(path):
    with path.open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()


def save(path, value):
    path.write_text(json.dumps(value, indent=2) + '\n')


def require_pin(path, expected):
    actual = sha(path)
    assert actual == expected, ('pin mismatch', str(path), expected, actual)
    return dict(path=str(path), bytes=path.stat().st_size, sha256=actual)


def selected(selection):
    value = selection['selected_attempt']
    if value is None:
        assert selection['verdict'] == 'hold-invalid-external'
        return False
    assert value in [1, 2] and selection['verdict'] == 'valid-acquisition'
    return True


def controller_memory(status_text=None):
    status = (Path('/proc/self/status').read_text() if status_text is None else status_text)
    values = dict(line.split(':', 1) for line in status.splitlines() if ':' in line)
    stat = Path('/proc/self/stat').read_text().rsplit(')', 1)[1].split()
    return dict(pid=os.getpid(), start_ticks=int(stat[19]), unix_ns=time.time_ns(),
                monotonic_ns=time.monotonic_ns(), rss_bytes=int(values['VmRSS'].split()[0]) * 1024,
                vmhwm_bytes=int(values['VmHWM'].split()[0]) * 1024,
                bound_source='/proc/self/status VmHWM; same resident controller at post-vector boundary')


def preflight(manifest, command, out):
    directory = manifest.parent
    checks = [require_pin(directory / 'FREEZE.md', FREEZE_SHA),
              require_pin(directory / 'source-pins.tsv', SOURCE_SHA),
              require_pin(directory / 'artifact-pins.tsv', ARTIFACT_SHA),
              require_pin(directory / 'binary-pins.tsv', BINARY_SHA)]
    artifacts = list(csv.DictReader((directory / 'artifact-pins.tsv').open(), delimiter='\t'))
    pins = {row['canonical']: row for row in artifacts}
    checks.append(require_pin(manifest, pins[str(manifest)]['sha256']))
    sources = list(csv.DictReader((directory / 'source-pins.tsv').open(), delimiter='\t'))
    assert len(sources) == 417 and len({s['path'] for s in sources}) == 417
    argv = ['git', '-C', command['cwd'], 'diff', '--quiet', BASE, '--'] + [r['path'] for r in sources]
    diff = subprocess.run(argv, check=False).returncode
    save(out / (command['id'] + '-source-diff.json'), dict(argv=argv, rc=diff, positive_inventory=len(sources)))
    assert diff == 0, ('frozen source changed', diff)
    work = Path(command['cwd'])
    for relative in ['prototype/sb1/tacheck/tests/acquire.py', 'prototype/sb1/tacheck/tests/parity.py',
                     'prototype/sb1/tacheck/tests/freeze.py', 'tools/sb-campaign-chunk']:
        path = work / relative
        expected, = [r['sha256'] for r in sources if r['path'] == relative]
        checks.append(require_pin(path, expected))
    binaries = list(csv.DictReader((directory / 'binary-pins.tsv').open(), delimiter='\t'))
    # Small sealed ELFs only; no input/object bulk hashing between acquisitions.
    roles = {'measured-ta-server', 'harness-ckc'}
    if command['id'] == 'T3x':
        roles.add('future-ta-objects')
    matched = [row for row in binaries if row['role'] in roles]
    assert len(matched) == len(roles), ('binary roles', roles, [r['role'] for r in binaries])
    for row in matched:
        checks.append(require_pin(Path(row['canonical']), row['sha256']))
    assert command['already_shaped'] is False and '--already-shaped' not in command['argv']
    save(out / (command['id'] + '-preflight.json'), dict(unix_ns=time.time_ns(),
         source_diff_rc=diff, checked=checks, cwd=command['cwd'], argv=command['argv']))
    return command


def run(a, prelaunch=preflight):
    a.out.mkdir(parents=True, exist_ok=False)
    (a.out / 'pid').write_text(str(os.getpid()) + '\n')
    save(a.out / 'controller.json', dict(pid=os.getpid(), session=os.getsid(0), manifest=str(a.manifest),
         source=str(Path(__file__).resolve()), source_sha256=sha(Path(__file__)), ids=a.ids))
    rc = 1
    try:
        manifest = json.loads(a.manifest.read_text())
        assert manifest['source_commit'] == BASE
        commands = {c['id']: c for c in manifest['commands']}
        assert a.ids in [['T3x'], manifest['execution_order']]
        for position, name in enumerate(a.ids):
            rc = 1
            command = prelaunch(a.manifest, commands[name], a.out)
            if position:
                assert Path(commands[a.ids[position - 1]]['wrapper_rc_file']).is_file()
            state = None
            if name == 'T3x':
                log, marker = a.out / 'T3x.log', a.out / 'T3x.rc'
                metrics = Path(command['argv'][-2])
                assert not metrics.parent.exists(), ('output exists', metrics.parent)
            else:
                log, marker = Path(command['wrapper_log_file']), Path(command['wrapper_rc_file'])
                state = Path(command['argv'][command['argv'].index('--out') + 1])
                assert not state.exists(), ('output exists', state)
            assert not log.exists() and not marker.exists(), ('wrapper output exists', log, marker)
            save(a.out / (name + '-controller-before.json'), controller_memory())
            print(f'launch {name} unix_ns={time.time_ns()}', flush=True)
            with log.open('xb') as stream:
                child = subprocess.Popen(command['argv'], cwd=command['cwd'], stdout=stream,
                                         stderr=subprocess.STDOUT)
                waited, status, usage = os.wait4(child.pid, 0)
                assert waited == child.pid
                child_rc = child.returncode = os.waitstatus_to_exitcode(status)
            save(a.out / (name + '-controller-after.json'), dict(controller_memory(), child_pid=child.pid,
                 child_wait4_peak_bytes=usage.ru_maxrss * 1024,
                 child_wait4_cpu_seconds=usage.ru_utime + usage.ru_stime, child_rc=child_rc))
            with marker.open('x') as stream:
                stream.write(str(child_rc) + '\n'); stream.flush(); os.fsync(stream.fileno())
            print(f'wrapper {name} rc={child_rc}', flush=True)
            if name == 'T3x':
                rc = child_rc
                continue
            assert state is not None
            selection_file = state / 'selection.json'
            if not selection_file.is_file():
                save(a.out / 'stop.json', dict(id=name, reason='wrapper incomplete; no acquisition selection', wrapper_rc=child_rc))
                return 1
            selection = json.loads(selection_file.read_text())
            if not selected(selection):
                save(a.out / 'stop.json', dict(id=name, reason='second invalid-external attempt: HOLD', selection=selection))
                return 1
            assert child_rc == 0, ('selected-valid wrapper failed', child_rc)
            print(f'acquisition {name} selected_attempt={selection["selected_attempt"]}', flush=True)
            rc = 0
        save(a.out / 'result.json', dict(verdict='vectors-returned', ids=a.ids, rc=rc))
        return rc
    except BaseException as error:
        rc = 1
        save(a.out / 'error.json', dict(type=type(error).__name__, error=str(error)))
        raise
    finally:
        (a.out / 'rc').write_text(str(rc) + '\n')


def dispatch_controls(out):
    fixture = ('import json,sys; from pathlib import Path; '
               'p=Path(sys.argv[2]); p.mkdir(); '
               's=sys.argv[3]; '
               '(p/"selection.json").write_text(json.dumps(dict(selected_attempt=1 if s=="valid1" else 2 if s=="valid2" else None, '
               'verdict="hold-invalid-external" if s=="invalid" else "valid-acquisition"))) if s!="missing" else None; '
               'sys.exit(int(sys.argv[4]))')
    order = ['b1-f1b', 'b2-f1a', 'r1-f1b']
    results = []
    cases = [('sequence', ['valid1', 'valid2', 'valid1'], [0, 0, 0], 0),
             ('second-invalid', ['invalid'], [0], 1),
             ('incomplete', ['missing'], [23], 1),
             ('wrapper-failed', ['valid1'], [17], 1),
             ('T3x-green', [], [0], 0), ('T3x-red', [], [17], 17),
             ('preflight-red', ['valid1'], [0], 1)]
    for name, selections, returncodes, wanted in cases:
        directory = out / name; directory.mkdir()
        commands = []
        ids = ['T3x'] if name.startswith('T3x') else order
        for i, identifier in enumerate(ids):
            state = directory / (identifier + '-state')
            if identifier == 'T3x':
                code = 'import sys; sys.exit(' + str(returncodes[0]) + ')'
                argv = [sys.executable, '-I', '-c', code, str(state / 'metrics'), '--graph-only']
            else:
                choice = selections[i] if i < len(selections) else 'valid1'
                code = returncodes[i] if i < len(returncodes) else 0
                argv = [sys.executable, '-I', '-c', fixture, '--out', str(state), choice, str(code)]
            commands.append(dict(id=identifier, cwd=str(directory), argv=argv,
                 wrapper_log_file=str(directory / (identifier + '.log')), wrapper_rc_file=str(directory / (identifier + '.rc'))))
        manifest = directory / 'manifest.json'
        save(manifest, dict(source_commit=BASE, execution_order=order, commands=commands))

        def fixture_prelaunch(_manifest, command, _out):
            if name == 'preflight-red':
                raise AssertionError('planted pin failure')
            return command

        args = argparse.Namespace(manifest=manifest, out=directory / 'controller', ids=ids)
        try:
            rc = run(args, fixture_prelaunch)
        except AssertionError:
            rc = 1
        assert rc == wanted and int((args.out / 'rc').read_text()) == wanted, (name, rc, wanted)
        wrappers = [Path(c['wrapper_rc_file']).exists() for c in commands]
        if name == 'sequence':
            assert wrappers == [True, True, True]
        elif ids != ['T3x']:
            assert wrappers == ([False, False, False] if name == 'preflight-red' else [True, False, False])
        if name == 'second-invalid':
            assert json.loads((args.out / 'stop.json').read_text())['reason'].startswith('second invalid-external')
        if name == 'incomplete':
            assert json.loads((args.out / 'stop.json').read_text())['reason'].startswith('wrapper incomplete')
        results.append(dict(case=name, actual_rc=rc, wanted_rc=wanted, acquisition=False))
    save(out / 'dispatch-controls.json', results)


def selftest(out):
    out.mkdir(parents=True, exist_ok=False)
    path = out / 'pinned'; path.write_bytes(b'pin\n')
    expected = sha(path)
    require_pin(path, expected)
    path.write_bytes(b'plant\n')
    try:
        require_pin(path, expected)
    except AssertionError:
        red = True
    else:
        red = False
    assert red, 'changed-byte plant escaped'
    path.write_bytes(b'pin\n'); require_pin(path, expected)
    assert selected(dict(selected_attempt=1, verdict='valid-acquisition'))
    assert selected(dict(selected_attempt=2, verdict='valid-acquisition'))
    assert not selected(dict(selected_attempt=None, verdict='hold-invalid-external'))
    save(out / 'controls.json', dict(pin_green_planted_red_restored_green=True,
         host_selection_valid1=True, host_selection_valid2=True, second_invalid_stops=True))
    memory = controller_memory()
    assert memory['pid'] == os.getpid() and memory['vmhwm_bytes'] >= memory['rss_bytes'] > 0
    try:
        controller_memory('VmRSS: 1 kB\n')
    except KeyError:
        missing_hwm_red = True
    else:
        missing_hwm_red = False
    assert missing_hwm_red
    save(out / 'memory-controls.json', dict(actual=memory, omitted_hwm_refused=missing_hwm_red))
    dispatch_controls(out)
    for snapshot in out.glob('*/controller/*-controller-after.json'):
        row = json.loads(snapshot.read_text())
        assert row['pid'] == os.getpid() and row['child_wait4_peak_bytes'] > 0
    print('controller controls: pin green/red/green; seven no-acquisition dispatch branches + HWM omission plant pass')


if __name__ == '__main__':
    p = argparse.ArgumentParser()
    modes = p.add_subparsers(dest='mode', required=True)
    s = modes.add_parser('selftest'); s.add_argument('out', type=Path)
    s = modes.add_parser('run')
    s.add_argument('--manifest', type=Path, required=True)
    s.add_argument('--out', type=Path, required=True)
    s.add_argument('ids', nargs='+')
    a = p.parse_args()
    if a.mode == 'selftest':
        selftest(a.out)
    else:
        raise SystemExit(run(a))
