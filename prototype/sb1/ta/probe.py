#!/usr/bin/env python3
"""One sealed construction census; workstation callers hold the SB1 timing lock."""
import argparse
import csv
import hashlib
import json
import math
import os
from pathlib import Path
import re
import shlex
import subprocess
import time


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def identities(root, composition, out, patient):
    text = composition.read_text()
    paths = [root / line for line in text.splitlines() if line and not line.startswith('#')]
    counts = [int(line.split('\t')[1]) for line in text.splitlines()
              if line.startswith('# patient_documents\t')]
    manifest = composition.with_name('manifest.tsv')
    if manifest.is_file():
        counts += [int(line.split('\t')[1]) for line in manifest.read_text().splitlines()
                   if line.startswith('patient_documents\t')]
    if counts and len(set(counts)) != 1:
        raise ValueError('conflicting patient counts')
    n = (counts[0] if counts else 1) if patient == 'last' else 0
    if n > len(paths):
        raise ValueError('patient suffix')
    knowledge = paths[:-n] if n else paths
    digest = hashlib.sha256(b'sb1-ta-knowledge\0dc=1\0')
    digest.update(len(knowledge).to_bytes(8, 'little'))
    listing = ['ordinal\tpath\tbytes\tsha256\n']
    for i, path in enumerate([root / 'guidelines/schema.pl', *knowledge]):
        data = path.read_bytes()
        digest.update(len(data).to_bytes(8, 'little'))
        digest.update(data)
        listing.append(f'{i}\t{path}\t{len(data)}\t{hashlib.sha256(data).hexdigest()}\n')
    (out / 'knowledge-members.tsv').write_text(''.join(listing))
    (out / 'knowledge-identity.txt').write_text(digest.hexdigest() + '\n')
    return paths[-n:] if n else []


def receipt(out, directory, source_commit, binary_sha, host, command, rc):
    if not re.fullmatch('[0-9a-f]{40}', source_commit):
        raise ValueError('receipt source commit must be a full SHA')
    if host not in ['pool', 'workstation']:
        raise ValueError('receipt host')
    identity = (out / 'knowledge-identity.txt').read_text().strip()
    if not re.fullmatch('[0-9a-f]{64}', identity) or not re.fullmatch('[0-9a-f]{64}', binary_sha):
        raise ValueError('receipt digest')
    with (out / 'knowledge-members.tsv').open() as stream:
        members = list(csv.DictReader(stream, delimiter='\t'))
    if [int(r['ordinal']) for r in members] != list(range(len(members))) or not members:
        raise ValueError('receipt member inventory')
    analysis_file = out / 'metrics/analysis.tsv'
    analysis = {}
    if analysis_file.is_file():
        with analysis_file.open() as stream:
            rows = list(csv.DictReader(stream, delimiter='\t'))
        if len(rows) != 1:
            raise ValueError('receipt analysis row count')
        analysis = rows[0]
    verdict = (analysis.get('verdict') if rc == 0
               else 'refused' if rc == 1 and analysis.get('verdict') == 'refused'
               else 'censored')
    if verdict not in ['admitted', 'refused', 'censored']:
        raise ValueError('receipt has no completed admission')
    phases_file = out / 'metrics/phases.tsv'
    phases = {}
    if phases_file.is_file():
        with phases_file.open() as stream:
            phases = {r['phase']: r['seconds'] for r in csv.DictReader(stream, delimiter='\t')}
    fields = {
        'format': 'sb1-ta-receipt 1',
        'kb_identity': identity,
        'knowledge_documents': str(len(members) - 1),
        'knowledge_bytes': str(sum(int(r['bytes']) for r in members[1:])),
        'verdict': verdict,
        'ta_source_commit': source_commit,
        'ta_binary_sha256': binary_sha,
        'dc': '1',
        'cap_bytes': str(28 * 2**30),
        'vmhwm_bytes': analysis.get('peak_rss_bytes', 'unavailable') if verdict != 'censored' else 'unavailable',
        'parse_seconds': phases.get('parse_shape', 'unavailable'),
        'analysis_seconds': analysis.get('wall_seconds', 'unavailable'),
        'host': host,
        'command': shlex.join(command),
    }
    if verdict == 'admitted':
        if int(fields['vmhwm_bytes']) <= 0:
            raise ValueError('receipt has no completed HWM')
        for key in ['parse_seconds', 'analysis_seconds']:
            value = float(fields[key])
            if not math.isfinite(value) or value < 0:
                raise ValueError('receipt has no completed phase measurement')
    if any('\t' in value or '\n' in value or '\r' in value for value in fields.values()):
        raise ValueError('receipt field delimiter')
    directory.mkdir(parents=True, exist_ok=True)
    destination = directory / (identity + '.tsv')
    temporary = destination.with_suffix('.tmp')
    temporary.write_text(''.join(f'{key}\t{value}\n' for key, value in fields.items()))
    temporary.replace(destination)
    return destination


def derived_fields(basis, identity, documents, byte_count, relation):
    keys = ['format', 'kb_identity', 'knowledge_documents', 'knowledge_bytes',
            'verdict', 'ta_source_commit', 'ta_binary_sha256', 'dc', 'cap_bytes',
            'vmhwm_bytes', 'parse_seconds', 'analysis_seconds', 'host', 'command']
    if basis.get('format') != 'sb1-ta-receipt 1' or basis.get('verdict') != 'admitted':
        raise ValueError('derived receipt requires an admitted basis')
    if relation not in ['renaming', 'subset']:
        raise ValueError('derived receipt relation')
    if not re.fullmatch('[0-9a-f]{64}', identity) or identity == basis['kb_identity']:
        raise ValueError('derived receipt target must be a distinct exact identity')
    if documents < 0 or byte_count < 0:
        raise ValueError('derived receipt target inventory')
    fields = {key: basis[key] for key in keys}
    fields.update(kb_identity=identity, knowledge_documents=str(documents),
                  knowledge_bytes=str(byte_count), basis=relation,
                  basis_identity=basis['kb_identity'])
    return fields


def processes(parent):
    rows = {}
    for path in Path('/proc').iterdir():
        if not path.name.isdecimal():
            continue
        try:
            raw = (path / 'stat').read_text().rsplit(')', 1)[1].split()
            rows[int(path.name)] = (int(raw[1]), raw[19])
        except (OSError, IndexError, ValueError):
            pass
    owned = {parent}
    while True:
        more = {pid for pid, (ppid, _) in rows.items() if ppid in owned}
        if more <= owned:
            break
        owned |= more
    return [(pid, rows[pid][1]) for pid in owned if pid in rows]


def service_cap(unit, membership=None, root=Path('/sys/fs/cgroup')):
    if not re.fullmatch('[A-Za-z0-9_.@-]+', unit):
        raise ValueError('service unit name')
    text = Path('/proc/self/cgroup').read_text() if membership is None else membership
    groups = [line[3:] for line in text.splitlines() if line.startswith('0::')]
    if len(groups) != 1:
        raise ValueError('service requires cgroup v2')
    group = root / groups[0].lstrip('/')
    if group.name != unit + '.service':
        raise ValueError('probe is not in the named service')
    cap = (group / 'memory.max').read_text().strip()
    swap = (group / 'memory.swap.max').read_text().strip()
    if cap != str(28 * 2**30) or swap != '0':
        raise ValueError('service requires MemoryMax=28GiB + swap=0')
    return group


def ta_flags(args):
    return ['--' + key.replace('_', '-') for key in
            ['reference', 'features_only', 'compare_reference', 'front_only',
             'redesign_front', 'redesign', 'dump_graph', 'dump_front', 'dump_analysis']
            if getattr(args, key)]


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('--ta', required=True, type=Path)
    p.add_argument('--root', required=True, type=Path)
    p.add_argument('--composition', required=True, type=Path)
    p.add_argument('--out', required=True, type=Path)
    p.add_argument('--patient', choices=['last', 'none'], default='last')
    p.add_argument('--reference', action='store_true')
    p.add_argument('--features-only', action='store_true')
    p.add_argument('--compare-reference', action='store_true')
    p.add_argument('--front-only', action='store_true')
    p.add_argument('--redesign-front', action='store_true')
    p.add_argument('--redesign', action='store_true')
    p.add_argument('--dump-graph', action='store_true')
    p.add_argument('--dump-front', action='store_true')
    p.add_argument('--dump-analysis', action='store_true')
    p.add_argument('--query', action='append', default=[], type=Path)
    p.add_argument('--emit', type=Path)
    p.add_argument('--seconds', type=int, default=7200)
    p.add_argument('--receipt-dir', type=Path)
    p.add_argument('--ta-source-commit')
    p.add_argument('--receipt-host', choices=['pool', 'workstation'])
    p.add_argument('--service-unit', help='run inside this already-capped detached user service')
    a = p.parse_args()
    if a.redesign and (a.reference or a.features_only or a.compare_reference
                       or a.front_only or a.redesign_front):
        p.error('--redesign requires a full analysis without reference/partial modes')
    if a.receipt_dir and (not a.ta_source_commit or not a.receipt_host
                          or a.reference or a.features_only or a.compare_reference
                          or a.front_only or a.redesign_front or a.dump_graph and not a.redesign):
        p.error('receipts require a refined full analysis, --ta-source-commit + --receipt-host')
    a.out.mkdir(parents=True, exist_ok=False)
    binary_sha = sha(a.ta)
    if a.ta.name != 'ta-' + binary_sha:
        raise ValueError('executable must be an immutable SHA-named copy')
    paths = identities(a.root, a.composition, a.out, a.patient)
    (a.out / 'identity.tsv').write_text('kind\tpath\tbytes\tsha256\n' + ''.join(
        f'{kind}\t{path}\t{path.stat().st_size}\t{sha(path)}\n'
        for kind, path in [('binary', a.ta), ('composition', a.composition)]
        + [('patient', path) for path in paths]
    ))
    unit = 'sb1-b14-' + hashlib.sha256(str(a.out).encode()).hexdigest()[:16]
    cmd = ['timeout', str(a.seconds), str(a.ta), '--root', str(a.root),
           '--composition', str(a.composition), '--patient', a.patient, '--dc',
           '--out', str(a.out / 'metrics')]
    if a.service_unit:
        group = service_cap(a.service_unit)
        (a.out / 'cgroup-cap.tsv').write_text(f'group\t{group}\ncap_bytes\t{28 * 2**30}\nswap_bytes\t0\n')
        journal_unit = a.service_unit + '.service'
    else:
        cmd = ['systemd-run', '--user', '--scope', '--unit', unit, '-q', '-p',
               'MemoryMax=28G', '-p', 'MemorySwapMax=0', '-p', 'OOMPolicy=continue', *cmd]
        journal_unit = unit + '.scope'
    cmd += ta_flags(a)
    for q in a.query:
        cmd += ['--query', str(q)]
    if a.emit:
        cmd += ['--emit', str(a.emit)]
    (a.out / 'command.json').write_text(json.dumps(cmd) + '\n')
    env = os.environ.copy()
    env['XDG_RUNTIME_DIR'] = f'/run/user/{os.getuid()}'
    env['SB1_TA_MEMORY'] = str(a.out / 'memory.tsv')
    started = time.monotonic()
    with (a.out / 'stdout.txt').open('w') as stdout, (a.out / 'stderr.txt').open('w') as stderr, (a.out / 'rss.tsv').open('w') as rss:
        child = subprocess.Popen(cmd, stdout=stdout, stderr=stderr, env=env)
        rss.write('seconds\tpid\tstart_ticks\trss_bytes\thwm_bytes\n')
        while child.poll() is None:
            now = time.monotonic() - started
            for pid, identity in processes(child.pid):
                try:
                    fields = dict(line.split(':', 1) for line in Path(f'/proc/{pid}/status').read_text().splitlines() if ':' in line)
                    resident = int(fields.get('VmRSS', '0').split()[0]) * 1024
                    peak = int(fields.get('VmHWM', '0').split()[0]) * 1024
                    rss.write(f'{now:.6f}\t{pid}\t{identity}\t{resident}\t{peak}\n')
                except (OSError, ValueError, IndexError):
                    pass
            rss.flush()
            time.sleep(0.05)
        rc = child.wait()
    (a.out / 'rc.txt').write_text(str(rc) + '\n')
    (a.out / 'scope-wall.txt').write_text(str(time.monotonic() - started) + '\n')
    with (a.out / 'scope-journal.txt').open('w') as stream:
        subprocess.run(['journalctl', '--user', '-u', journal_unit, '-n', '12', '--no-pager'], stdout=stream, stderr=subprocess.STDOUT, check=False, env=env)
    if a.receipt_dir:
        path = receipt(a.out, a.receipt_dir, a.ta_source_commit, binary_sha,
                       a.receipt_host, cmd, rc)
        print(f'receipt\t{path}', flush=True)
    print(f'probe\t{a.out.name}\trc={rc}\t{binary_sha}', flush=True)
    return rc if rc >= 0 else 128 - rc


if __name__ == '__main__':
    raise SystemExit(main())
