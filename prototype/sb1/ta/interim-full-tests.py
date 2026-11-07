#!/usr/bin/env python3
"""I2 frozen-rule controls; synthetic fixture values are not measurements."""
import csv
from decimal import Decimal
import hashlib
import json
from pathlib import Path
import runpy
import subprocess
import sys
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import patch

HERE = Path(__file__).resolve().parent
MODULE = runpy.run_path(str(HERE / 'interim-full'))
PHASES = MODULE['PHASES']
GRADE = MODULE['decision']
COMPLETE = MODULE['complete_seconds']
COMPARE = MODULE['compare_graph']


class I2(unittest.TestCase):
    def setUp(self):
        parent = HERE / 'target/interim-full-tests'
        parent.mkdir(parents=True, exist_ok=True)
        self.temporary = tempfile.TemporaryDirectory(dir=parent)
        self.root = Path(self.temporary.name)
        self.oracle = self.root / 'oracle/metrics'
        self.oracle.mkdir(parents=True)
        (self.oracle.parent / 'knowledge-identity.txt').write_text('b' * 64 + '\n')
        self.graph = {'front-nodes.tsv': 'node1\nnode2\n', 'front-edges.tsv': '0\t1\tfalse\tfalse\n', 'front-scc.tsv': '0\n1\n'}
        for name, text in self.graph.items():
            (self.oracle / name).write_text(text)
        self.binary_sha = hashlib.sha256(b'fixture binary').hexdigest()
        self.binary = self.root / ('ta-' + self.binary_sha)
        self.binary.write_bytes(b'fixture binary')
        self.calls = []
        self.codes = {}
        self.verdicts = {}
        self.costs = {}
        self.bad_graph = False
        self.bad_analysis = False

    def tearDown(self):
        self.temporary.cleanup()

    @staticmethod
    def row(name, seconds='1650'):
        row = dict(run=name, rc=0, verdict='admitted', **{p: '0' for p in PHASES})
        row['parse_shape'] = seconds
        return row

    def test_rule_includes_parse_at_the_frozen_boundary(self):
        pair = [self.row('F1b'), self.row('F1a')]
        self.assertEqual(GRADE(pair, True), 'continue')
        for name in ['F1b', 'F1a']:
            planted = [dict(r) for r in pair]
            planted[['F1b', 'F1a'].index(name)]['parse_shape'] = '1650.000000001'
            self.assertEqual(GRADE(planted, True), 'stop')
        self.assertEqual(GRADE(pair, False), 'stop')
        self.assertEqual(GRADE(pair[::-1], True), 'stop')
        self.assertEqual(GRADE(pair[:1], True), 'stop')
        phases = dict(zip(PHASES, ['30', '60', '600', '500', '460']))
        self.assertEqual(COMPLETE(phases), Decimal(1650))

    def test_missing_and_invalid_phase_evidence_stops(self):
        row = self.row('F1b')
        for value in ['NaN', 'Infinity', '-0.1', 'bad']:
            self.assertEqual(GRADE([{**row, 'k1_growth': value}, self.row('F1a')], True), 'stop')
        for p in PHASES:
            phases = {k: '1' for k in PHASES if k != p}
            with self.assertRaises(ValueError): COMPLETE(phases)
        for code in [1, 2, 124, -9]:
            self.assertEqual(GRADE([{**row, 'rc': code}, self.row('F1a')], True), 'stop')
        self.assertEqual(GRADE([{**row, 'verdict': 'refused'}, self.row('F1a')], True), 'stop')

    def test_graph_missing_and_changed_bytes_are_detected(self):
        actual = self.root / 'graph'
        actual.mkdir()
        for name, text in self.graph.items(): (actual / name).write_text(text)
        self.assertTrue(all(r['cmp_rc'] == 0 for r in COMPARE(self.oracle, actual)))
        for text in ['', '1\t0\tfalse\tfalse\n']:
            (actual / 'front-edges.tsv').write_text(text)
            result = COMPARE(self.oracle, actual)
            self.assertTrue(any(r['cmp_rc'] != 0 and r['oracle_sha256'] != r['candidate_sha256'] for r in result))
        (actual / 'front-edges.tsv').write_text(self.graph['front-edges.tsv'])
        self.assertTrue(all(r['cmp_rc'] == 0 for r in COMPARE(self.oracle, actual)))

    def invoke(self, label):
        out = self.root / label
        argv = ['interim-full', '--ta', str(self.binary), '--root', str(self.root),
                '--instances', str(self.root / 'instances'), '--oracle-dumps', str(self.oracle),
                '--out', str(out), '--source-commit', 'a' * 40]
        real_run = subprocess.run
        def fake_run(cmd, **kwargs):
            if len(cmd) < 3 or not str(cmd[2]).endswith('probe.py'):
                return real_run(cmd, **kwargs)
            output = Path(cmd[cmd.index('--out') + 1])
            family = output.name
            self.calls.append(family)
            self.assertIn('--redesign', cmd)
            self.assertEqual('--dump-graph' in cmd, family == 'F1b')
            self.assertEqual(cmd[cmd.index('--seconds') + 1], '43200')
            metrics = output / 'metrics'; metrics.mkdir(parents=True)
            phases = {p: '1' for p in PHASES}
            phases['parse_shape'] = self.costs.get(family, '1')
            analysis = sum((Decimal(phases[p]) for p in PHASES[1:]), Decimal(0))
            report_wall = str(analysis + (1 if self.bad_analysis else 0))
            verdict = self.verdicts.get(family, 'admitted')
            (metrics / 'phases.tsv').write_text('phase\tseconds\n' + ''.join(f'{p}\t{s}\n' for p, s in phases.items()))
            (metrics / 'analysis.tsv').write_text(f'verdict\twall_seconds\tpeak_rss_bytes\n{verdict}\t{report_wall}\t2048\n')
            identity = ('b' if family == 'F1b' else 'c') * 64
            (output / 'knowledge-identity.txt').write_text(identity + '\n')
            (output / 'scope-wall.txt').write_text('100\n')
            for name, text in self.graph.items():
                (metrics / name).write_text(text if not self.bad_graph or name != 'front-edges.tsv' else 'wrong\n')
            receipt = dict(format='sb1-ta-receipt 1', kb_identity=identity, ta_source_commit='a' * 40,
                           ta_binary_sha256=self.binary_sha, verdict=verdict, dc='1',
                           cap_bytes=str(28 * 2**30), host='workstation', vmhwm_bytes='2048',
                           parse_seconds=phases['parse_shape'], analysis_seconds=report_wall)
            receipts = out / 'private-receipts'; receipts.mkdir(exist_ok=True)
            (receipts / (identity + '.tsv')).write_text(''.join(f'{k}\t{v}\n' for k, v in receipt.items()))
            return SimpleNamespace(returncode=self.codes.get(family, 0))
        def fake_git(cmd, **kwargs):
            return b'' if 'status' in cmd else ('a' * 40 + '\n').encode()
        with patch.object(sys, 'argv', argv), patch.object(subprocess, 'check_output', fake_git), patch.object(subprocess, 'run', fake_run):
            rc = MODULE['main']()
        with (out / 'decision.tsv').open() as stream:
            fields = dict(csv.reader(stream, delimiter='\t'))
        return rc, fields, out

    def test_end_to_end_order_caps_receipts_and_continue(self):
        rc, verdict, out = self.invoke('positive')
        self.assertEqual(rc, 0)
        self.assertEqual(verdict['outcome'], 'continue')
        self.assertEqual(self.calls, ['F1b', 'F1a'])
        self.assertTrue((out / 'rule.txt').is_file())
        self.assertEqual(len(json.loads((out / 'commands.json').read_text())), 2)

    def test_each_censor_refusal_and_graph_mismatch_stops(self):
        self.codes['F1b'] = 124
        rc, result, _ = self.invoke('censor')
        self.assertEqual((rc, result['outcome'], self.calls), (1, 'stop', ['F1b']))
        self.calls.clear(); self.codes.clear(); self.verdicts['F1b'] = 'refused'
        rc, result, _ = self.invoke('refusal')
        self.assertEqual((rc, result['outcome'], self.calls), (1, 'stop', ['F1b']))
        self.calls.clear(); self.verdicts.clear(); self.bad_graph = True
        rc, result, _ = self.invoke('bad-graph')
        self.assertEqual((rc, result['outcome'], self.calls), (1, 'stop', ['F1b']))

    def test_over_budget_first_run_keeps_both_measurements_then_stops(self):
        self.costs['F1b'] = '1650'
        rc, result, _ = self.invoke('slow')
        self.assertEqual((rc, result['outcome'], self.calls), (1, 'stop', ['F1b', 'F1a']))

    def test_missing_graph_evidence_cannot_leave_a_pending_decision(self):
        (self.oracle / 'front-edges.tsv').unlink()
        rc, result, _ = self.invoke('missing-graph')
        self.assertEqual((rc, result['outcome'], self.calls), (1, 'stop', ['F1b']))

    def test_inconsistent_phase_sum_cannot_continue(self):
        self.bad_analysis = True
        rc, result, _ = self.invoke('sum')
        self.assertEqual((rc, result['outcome'], self.calls), (1, 'stop', ['F1b']))


if __name__ == '__main__':
    unittest.main()
