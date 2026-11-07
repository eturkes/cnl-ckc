#!/usr/bin/env python3
"""Receipt structure + absent-evidence controls; fixture values are not census results."""
import csv
from pathlib import Path
import runpy
import subprocess
import sys
import tempfile
import unittest
from types import SimpleNamespace

HERE = Path(__file__).resolve().parent
MODULE = runpy.run_path(str(HERE / 'probe.py'))
RECEIPT = MODULE['receipt']
SERVICE_CAP = MODULE['service_cap']
DERIVED = MODULE['derived_fields']
KEYS = {'format', 'kb_identity', 'knowledge_documents', 'knowledge_bytes',
        'verdict', 'ta_source_commit', 'ta_binary_sha256', 'dc', 'cap_bytes',
        'vmhwm_bytes', 'parse_seconds', 'analysis_seconds', 'host', 'command'}


class Receipts(unittest.TestCase):
    def setUp(self):
        parent = HERE / 'target/probe-tests'
        parent.mkdir(parents=True, exist_ok=True)
        self.temporary = tempfile.TemporaryDirectory(dir=parent)
        self.root = Path(self.temporary.name)
        self.out = self.root / 'case'
        self.out.mkdir()
        (self.out / 'knowledge-identity.txt').write_text('b' * 64 + '\n')
        (self.out / 'knowledge-members.tsv').write_text(
            'ordinal\tpath\tbytes\tsha256\n0\tschema\t5\t' + 'c' * 64 + '\n'
            '1\tfirst\t7\t' + 'd' * 64 + '\n2\tsecond\t11\t' + 'e' * 64 + '\n')
        (self.out / 'metrics').mkdir()
        (self.out / 'metrics/analysis.tsv').write_text(
            'verdict\twall_seconds\tpeak_rss_bytes\nadmitted\t12.5\t2048\n')
        (self.out / 'metrics/phases.tsv').write_text('phase\tseconds\nparse_shape\t3.25\n')

    def tearDown(self):
        self.temporary.cleanup()

    def test_detached_service_cap_is_observed_not_trusted(self):
        root = self.root / 'cgroups'
        group = root / 'user.slice/sb1-test.service'
        group.mkdir(parents=True)
        (group / 'memory.max').write_text(str(28 * 2**30))
        (group / 'memory.swap.max').write_text('0')
        membership = '0::/user.slice/sb1-test.service\n'
        self.assertEqual(SERVICE_CAP('sb1-test', membership, root), group)
        for cap, swap in [('max', '0'), (str(28 * 2**30 + 1), '0'), (str(28 * 2**30), '1')]:
            (group / 'memory.max').write_text(cap)
            (group / 'memory.swap.max').write_text(swap)
            with self.assertRaises(ValueError):
                SERVICE_CAP('sb1-test', membership, root)
        with self.assertRaises(ValueError):
            SERVICE_CAP('another', membership, root)
        with self.assertRaises(ValueError):
            SERVICE_CAP('sb1-test', '1:memory:/user.slice/sb1-test.service\n', root)

    def write(self, rc=0, source='a' * 40):
        path = RECEIPT(self.out, self.root / 'receipts', source, 'f' * 64,
                       'pool', ['ta', '--root', 'path with spaces'], rc)
        with path.open() as stream:
            rows = list(csv.reader(stream, delimiter='\t'))
        self.assertEqual(len(rows), len(KEYS))
        fields = dict(rows)
        self.assertEqual(set(fields), KEYS)
        self.assertEqual(path.name, 'b' * 64 + '.tsv')
        self.assertEqual(list((self.root / 'receipts').glob('*.tmp')), [])
        return fields

    def test_derived_records_label_basis_and_copy_only_measured_provenance(self):
        basis = self.write()
        for relation in ['renaming', 'subset']:
            row = DERIVED(basis, 'c' * 64, 1, 7, relation)
            self.assertEqual(set(row), KEYS | {'basis', 'basis_identity'})
            self.assertEqual(row['kb_identity'], 'c' * 64)
            self.assertEqual(row['knowledge_documents'], '1')
            self.assertEqual(row['knowledge_bytes'], '7')
            self.assertEqual(row['basis'], relation)
            self.assertEqual(row['basis_identity'], basis['kb_identity'])
            for key in KEYS - {'kb_identity', 'knowledge_documents', 'knowledge_bytes'}:
                self.assertEqual(row[key], basis[key])
            child = DERIVED(row, 'd' * 64, 1, 5, 'subset')
            self.assertEqual(child['basis_identity'], row['kb_identity'])
        for verdict in ['refused', 'censored']:
            with self.assertRaises(ValueError):
                DERIVED({**basis, 'verdict': verdict}, 'c' * 64, 1, 7, 'subset')
        with self.assertRaises(ValueError):
            DERIVED(basis, basis['kb_identity'], 1, 7, 'renaming')

    def test_complete_admission_binds_exact_inventory_and_evidence(self):
        row = self.write()
        self.assertEqual(row['format'], 'sb1-ta-receipt 1')
        self.assertEqual(row['verdict'], 'admitted')
        self.assertEqual(row['knowledge_documents'], '2')
        self.assertEqual(row['knowledge_bytes'], '18')
        self.assertEqual(row['dc'], '1')
        self.assertEqual(row['cap_bytes'], str(28 * 2**30))
        self.assertEqual(row['vmhwm_bytes'], '2048')
        self.assertEqual(row['parse_seconds'], '3.25')
        self.assertEqual(row['analysis_seconds'], '12.5')
        self.assertEqual(row['command'], "ta --root 'path with spaces'")

    def test_refusal_and_censor_never_become_admitted(self):
        self.assertEqual(self.write(1)['verdict'], 'censored')
        (self.out / 'metrics/analysis.tsv').write_text(
            'verdict\twall_seconds\tpeak_rss_bytes\nrefused\t12.5\t2048\n')
        self.assertEqual(self.write(1)['verdict'], 'refused')
        (self.out / 'metrics/analysis.tsv').unlink()
        for rc in [-9, 124, 2]:
            row = self.write(rc)
            self.assertEqual(row['verdict'], 'censored')
            self.assertEqual(row['vmhwm_bytes'], 'unavailable')
            self.assertEqual(row['analysis_seconds'], 'unavailable')
            self.assertEqual(row['parse_seconds'], '3.25')

    def test_missing_or_ambiguous_success_evidence_fails(self):
        (self.out / 'metrics/phases.tsv').unlink()
        with self.assertRaises(ValueError):
            self.write()
        (self.out / 'metrics/analysis.tsv').unlink()
        with self.assertRaises(ValueError):
            self.write()
        with self.assertRaises(ValueError):
            self.write(124, source='bad-source')
        (self.out / 'metrics/analysis.tsv').write_text(
            'verdict\twall_seconds\tpeak_rss_bytes\nadmitted\t1\t1\nadmitted\t2\t2\n')
        with self.assertRaises(ValueError):
            self.write()

    def test_complete_redesign_flags_are_forwarded(self):
        flags = ['reference', 'features_only', 'compare_reference', 'front_only',
                 'redesign_front', 'redesign', 'dump_graph', 'dump_front', 'dump_analysis']
        forward = MODULE['ta_flags']
        for mask in range(1 << len(flags)):
            args = SimpleNamespace(**{key: bool(mask & (1 << i)) for i, key in enumerate(flags)})
            expected = ['--' + key.replace('_', '-') for i, key in enumerate(flags) if mask & (1 << i)]
            self.assertEqual(forward(args), expected)
        self.assertEqual(forward(SimpleNamespace(**{key: key == 'redesign' for key in flags})), ['--redesign'])

    def test_redesign_rejects_partial_or_reference_modes_before_io(self):
        for mode in ['--reference', '--features-only', '--compare-reference', '--front-only', '--redesign-front']:
            out = self.root / ('bad-' + mode[2:])
            result = subprocess.run([sys.executable, '-I', str(HERE / 'probe.py'),
                                     '--ta', '/missing', '--root', str(self.root),
                                     '--composition', '/missing', '--out', str(out),
                                     '--redesign', mode], capture_output=True, check=False)
            self.assertEqual(result.returncode, 2)
            self.assertIn(b'--redesign requires a full analysis', result.stderr)
            self.assertFalse(out.exists())

    def test_reference_and_partial_modes_cannot_issue_receipts(self):
        for mode in ['--reference', '--features-only', '--compare-reference',
                     '--front-only', '--redesign-front', '--dump-graph']:
            result = subprocess.run([sys.executable, '-I', str(HERE / 'probe.py'),
                                     '--ta', '/missing', '--root', str(self.root),
                                     '--composition', '/missing', '--out', str(self.root / 'unused'),
                                     '--receipt-dir', str(self.root / 'receipts'),
                                     '--ta-source-commit', 'a' * 40,
                                     '--receipt-host', 'pool', mode],
                                    capture_output=True, check=False)
            self.assertEqual(result.returncode, 2)
            self.assertIn(b'receipts require a refined full analysis', result.stderr)
        self.assertFalse((self.root / 'unused').exists())


if __name__ == '__main__':
    unittest.main()
