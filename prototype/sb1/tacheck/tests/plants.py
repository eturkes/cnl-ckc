#!/usr/bin/env python3
"""T2: sealed real test paths, green → planted red → restored green."""
import argparse
import csv
import hashlib
import json
import os
from pathlib import Path
import subprocess

CERTIFICATE = ['missing-seed', 'missing-transfer', 'rank-zero', 'rank-missing',
               'weak-positive-rank', 'strict-negative-rank', 'missing-forwarding-source',
               'missing-realizable-tag-tuple', 'missing-realizable-clause-variant',
               'inherited-base-rank', 'non-cyclic-growth-rank']
OMISSION = ['missing-variable-head', 'missing-top-compatible-head', 'dropped-signed-edge',
            'wrong-sign', 'csr-drop', 'query-root-omitted', 'query-payload-omitted',
            'query-naf-omitted', 'exhaustion-as-no-edge', 'complement-growth-edge']


def sha(path):
    return hashlib.file_digest(path.open('rb'), 'sha256').hexdigest()


def main():
    p = argparse.ArgumentParser()
    p.add_argument('--tests', type=Path, required=True)
    p.add_argument('--out', type=Path, required=True)
    a = p.parse_args()
    a.out.mkdir(parents=True, exist_ok=False)
    plants = CERTIFICATE + OMISSION
    assert len(plants) == len(set(plants)) == 21
    argv = [str(a.tests), 'plant_tests::t2_probe', '--exact', '--nocapture']
    (a.out / 'protocol.json').write_text(json.dumps(dict(tests_sha256=sha(a.tests), argv=argv, plants=plants, censor_seconds=120), indent=2) + '\n')
    results = []

    def run(name, plant):
        out = a.out / name
        out.mkdir()
        env = os.environ.copy()
        env.pop('SB1_TAS_T2_PLANT', None)
        if plant:
            env['SB1_TAS_T2_PLANT'] = plant
        env['SB1_TAS_T2_DIR'] = str(out)
        with (out / 'stdout').open('wb') as stdout, (out / 'stderr').open('wb') as stderr:
            rc = subprocess.run(argv, stdout=stdout, stderr=stderr, env=env, timeout=120, check=False).returncode
        (out / 'rc').write_text(str(rc) + '\n')
        return rc, (out / 'stdout').read_text() + (out / 'stderr').read_text()

    for index, plant in enumerate(plants, 1):
        before, _ = run(f'{index:02}-green', '')
        assert before == 0, (plant, 'unplanted control', before)
        red, text = run(f'{index:02}-{plant}', plant)
        assert red == 101 and 'test result: FAILED.' in text, (plant, 'plant failed to fire', red, text)
        if plant in CERTIFICATE:
            assert 'Certificate(' in text and 'Cycle {' not in text and 'Indeterminate(' not in text, (plant, text)
            outcome = 'refused-certificate'
        else:
            assert 'nonempty' in text or 'exhausted may-match read as no-edge/admitted' in text, (plant, text)
            outcome = 'independent-object-or-semantic-delta'
        restored, _ = run(f'{index:02}-restored', '')
        assert restored == 0, (plant, 'restored control', restored)
        results.append(dict(plant=plant, green_rc=before, planted_rc=red, restored_rc=restored, outcome=outcome, verdict='pass'))
        with (a.out / 'verdicts.tsv').open('w', newline='') as stream:
            writer = csv.DictWriter(stream, results[0].keys(), delimiter='\t', lineterminator='\n')
            writer.writeheader(); writer.writerows(results)
        print(f'T2\t{plant}\tgreen=0 planted=101 restored=0\t{outcome}', flush=True)
    print(f'T2 PASS plants={len(results)}', flush=True)


if __name__ == '__main__':
    main()
