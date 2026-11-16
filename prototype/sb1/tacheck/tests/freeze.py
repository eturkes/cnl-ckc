#!/usr/bin/env python3
"""TAS freeze inputs + schedule, without starting any resource acquisition."""
import argparse
from collections import Counter
import csv
import hashlib
import json
from pathlib import Path
import subprocess

MASK = (1 << 64) - 1
INSTANCES = ['F1b-g100-s1', 'F1a-g100-s1', 'F1b-g001-s0']
QUERIES = [('q-all.txt', 'q-all.out'), ('q-one.pl', 'q-one.out'), ('q-patient.pl', 'q-patient.out')]
CHUNKS = [
    dict(id='b1-f1b', instance='F1b-g100-s1', paired=None, blocks=7, seed=90121, queries=['q-all.txt', 'q-one.pl', 'q-patient.pl'], spacing_ns=0),
    dict(id='b2-f1a', instance='F1a-g100-s1', paired=None, blocks=7, seed=90122, queries=['q-all.txt', 'q-one.pl', 'q-patient.pl'], spacing_ns=0),
    dict(id='r1-f1b', instance='F1b-g100-s1', paired='F1b-g001-s0', blocks=7, seed=90123, queries=['q-one.pl'], spacing_ns=30_000_000_000),
]


def sha(path):
    with path.open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()


def table(path, rows):
    with path.open('w', newline='') as stream:
        writer = csv.DictWriter(stream, rows[0].keys(), delimiter='\t', lineterminator='\n')
        writer.writeheader(); writer.writerows(rows)


class Random:
    def __init__(self, seed):
        self.state = seed

    def below_two(self):
        # Rust Random::below(2): rejection threshold = 0, exactly one word.
        self.state = (self.state + 0x9e3779b97f4a7c15) & MASK
        z = self.state
        z = ((z ^ (z >> 30)) * 0xbf58476d1ce4e5b9) & MASK
        z = ((z ^ (z >> 27)) * 0x94d049bb133111eb) & MASK
        return (z ^ (z >> 31)) % 2


def plans(chunk):
    random = Random(chunk['seed'])
    rows = []
    for block in range(1, chunk['blocks'] + 1):
        before = random.state
        arms = [('target', chunk['instance'])]
        draw = None
        if chunk['paired'] is not None:
            arms.append(('baseline', chunk['paired']))
            draw = random.below_two()
            if draw:
                arms.reverse()
        order = 0
        for arm, instance in arms:
            for query in chunk['queries']:
                order += 1
                rows.append(dict(chunk=chunk['id'], chunk_seed=chunk['seed'], block=block, rng_state_before=before, arm_draw='none' if draw is None else draw, rng_state_after=random.state, run=len(rows) + 1, order=order, arm=arm, instance=instance, query=query, earliest_offset_ns=chunk['spacing_ns'] * (block - 1)))
    return rows


def schedule(a):
    a.out.mkdir(parents=True, exist_ok=False)
    # Independent schedule reader rematches the allowed one-pair rehearsal's
    # actual seed/draw, not any confirmation campaign result.
    expected = list(csv.DictReader((a.rehearsal / 'runs/x02/plan.tsv').open(), delimiter='\t'))
    settings = dict((r['key'], r['value']) for r in csv.DictReader((a.rehearsal / 'runs/x02/settings.tsv').open(), delimiter='\t'))
    control = plans(dict(id='rehearsal-x02', instance='F1a-g001-s0', paired='F1b-g001-s0', blocks=1, seed=int(settings['seed']), queries=['q-one.pl'], spacing_ns=30_000_000_000))
    assert len(expected) == len(control) == 2
    for actual, predicted in zip(expected, control):
        assert (int(actual['run']), int(actual['block']), int(actual['order']), actual['arm'], Path(actual['query']).name) == (predicted['run'], predicted['block'], predicted['order'], predicted['arm'], predicted['query'])
    table(a.out / 'rehearsal-plan.tsv', control)
    rows = [r for chunk in CHUNKS for r in plans(chunk)]
    assert len(rows) == 56
    table(a.out / 'schedule.tsv', rows)
    (a.out / 'chunks.json').write_text(json.dumps(CHUNKS, indent=2) + '\n')
    print('schedule: 56 calls frozen; rehearsal arm/order control equal; triple query order fixed, R arm order drawn')


def inputs(a):
    a.out.mkdir(parents=True, exist_ok=False)
    root = a.root.resolve()
    bench = root / 'rust/target/bench'
    allowed = [bench / name for name in ['sb1-frozen', 'sb1-oracles', 'sb1-rehearsal-2']]
    assert a.forbidden is not None
    forbidden = a.forbidden.absolute()

    def permitted(path):
        assert not path.absolute().is_relative_to(forbidden), ('grade-blind boundary', path)
        path = path.resolve(strict=True)
        assert path.is_relative_to(root), ('input leaves the declared frozen root', path)
        assert not path.is_relative_to(forbidden), ('grade-blind boundary', path)
        assert not path.is_relative_to(bench) or any(path.is_relative_to(p) for p in allowed), ('grade-blind boundary', path)
        assert path.is_file()
        return path

    manifest = list(csv.reader(a.manifest.open(), delimiter='\t'))
    rows, summaries = [], []
    signatures = {}
    schema = permitted(root / 'guidelines/schema.pl')
    schema_bytes = schema.read_bytes()
    for instance in INSTANCES:
        directory = permitted(bench / 'sb1-frozen' / instance / 'composition.tsv').parent
        composition = directory / 'composition.tsv'
        values = [line.split('\t')[1] for line in (directory / 'manifest.tsv').read_text().splitlines() if line.startswith('patient_documents\t')]
        assert len(values) == 1
        patients = int(values[0])
        paths = [line for line in composition.read_text().splitlines() if line and not line.startswith('#')]
        assert 0 < patients < len(paths)
        count = len(paths) - patients
        knowledge = hashlib.sha256(b'sb1-ta-knowledge\0dc=1\0')
        knowledge.update(count.to_bytes(8, 'little'))
        knowledge.update(len(schema_bytes).to_bytes(8, 'little')); knowledge.update(schema_bytes)
        frame = hashlib.sha256(f'schema\0{len(schema_bytes)}\0{sha(schema)}\n'.encode())
        signatures[instance] = []
        for kind, position, listed, path in [('schema', 0, 'guidelines/schema.pl', schema)] + [('document', i + 1, p, permitted(root / p)) for i, p in enumerate(paths)]:
            content = path.read_bytes()
            digest = hashlib.sha256(content).hexdigest()
            patient = kind == 'document' and position > count
            rows.append(dict(instance=instance, kind=kind, position=position, listed=listed, canonical=str(path), bytes=len(content), sha256=digest, patient=int(patient)))
            if kind == 'document':
                frame.update(f'file\0{listed}\0{len(content)}\0{digest}\n'.encode())
                signatures[instance].append((len(content), digest))
                if not patient:
                    knowledge.update(len(content).to_bytes(8, 'little')); knowledge.update(content)
        expected = [r[4] for r in manifest if len(r) >= 5 and r[0] == 'instance' and r[1] == instance and r[2] == 'composition']
        assert expected == [frame.hexdigest()], (instance, expected, frame.hexdigest())
        for name in ['composition.tsv', 'manifest.tsv']:
            path = permitted(directory / name)
            rows.append(dict(instance=instance, kind=name, position=0, listed=name, canonical=str(path), bytes=path.stat().st_size, sha256=sha(path), patient=0))
        query_rows = []
        for query, answer in QUERIES:
            qp = permitted(directory / 'queries' / query)
            ap = permitted(bench / 'sb1-oracles' / instance / 'union' / answer)
            query_rows.append(dict(query=query, query_sha256=sha(qp), expected_answer=str(ap), expected_sha256=sha(ap)))
            for kind, path in [('query', qp), ('expected-answer', ap)]:
                rows.append(dict(instance=instance, kind=kind, position=0, listed=query if kind == 'query' else answer, canonical=str(path), bytes=path.stat().st_size, sha256=sha(path), patient=0))
        summaries.append(dict(instance=instance, documents=len(paths), knowledge_documents=count, patient_documents=patients, composition_file_sha256=sha(composition), composition_identity=frame.hexdigest(), kb_identity=knowledge.hexdigest(), queries=query_rows))
        table(a.out / 'members.tsv', rows)
        (a.out / 'identities.json').write_text(json.dumps(summaries, indent=2) + '\n')
    baseline = Counter(signatures['F1b-g001-s0'])
    target = Counter(signatures['F1b-g100-s1'])
    assert not baseline - target, 'R baseline has a document absent from G100 bytes'
    assert len(signatures['F1b-g001-s0']) == 493
    for query, _ in QUERIES:
        assert sha(bench / 'sb1-frozen/F1b-g001-s0/queries' / query) == sha(bench / 'sb1-frozen/F1b-g100-s1/queries' / query)
    answer = bench / 'sb1-oracles/F1b-g001-s0/union/q-one.out'
    target_answer = bench / 'sb1-oracles/F1b-g100-s1/union/q-one.out'
    assert subprocess.run(['cmp', '-s', '--', str(answer), str(target_answer)], check=False).returncode == 0
    (a.out / 'closure.json').write_text(json.dumps(dict(members_rows=len(rows), members_sha256=sha(a.out / 'members.tsv'), identities_sha256=sha(a.out / 'identities.json'), manifest_sha256=sha(a.manifest), r_baseline_documents_byte_matched=493, r_query_files_byte_matched=3, r_q_one_answers_byte_equal=True), indent=2) + '\n')
    print(f'inputs: {len(rows)} full member rows, 3 instance identities; R 493 documents/3 queries byte-matched')


def ratio(a):
    a.out.mkdir(parents=True, exist_ok=False)
    rows = list(csv.DictReader((a.rehearsal / 'runs/x02/ratio.tsv').open(), delimiter='\t'))
    assert len(rows) == 1 and rows[0]['pairs'] == '1'
    expected = (rows[0]['ratio_of_medians'], rows[0]['one_sided_95_upper'])
    report = list(csv.reader(a.grade.open(), delimiter='\t'))
    compared = []
    for row in report:
        if row[0] in ['ratio_harness', 'ratio_recomputed']:
            fields = dict(item.split('=', 1) for item in row[2:] if '=' in item)
            assert (fields['R'], fields['UB95']) == expected
            compared.append(row[:2])
    assert len(compared) == 3
    result = dict(rehearsal='sb1-rehearsal-2/runs/x02', pairs=1, seed=rows[0]['seed'], ratio=expected[0], upper=expected[1], compared=compared, ratio_sha256=sha(a.rehearsal / 'runs/x02/ratio.tsv'), grade_sha256=sha(a.grade), scope='estimator equality control, not TAS/confirmation performance')
    (a.out / 'equality.json').write_text(json.dumps(result, indent=2) + '\n')
    print('R estimator: harness/all/accepted recomputation exactly equal on permitted rehearsal')


if __name__ == '__main__':
    p = argparse.ArgumentParser()
    p.add_argument('mode', choices=['inputs', 'schedule', 'ratio'])
    p.add_argument('--out', type=Path, required=True)
    p.add_argument('--root', type=Path)
    p.add_argument('--manifest', type=Path)
    p.add_argument('--rehearsal', type=Path)
    p.add_argument('--grade', type=Path)
    p.add_argument('--forbidden', type=Path)
    a = p.parse_args()
    {'inputs': inputs, 'schedule': schedule, 'ratio': ratio}[a.mode](a)
