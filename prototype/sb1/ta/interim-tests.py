#!/usr/bin/env python3
"""I1 coordinator controls. Fixture numbers are not benchmark measurements."""
from pathlib import Path
import runpy
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

HERE = Path(__file__).resolve().parent
MAIN = runpy.run_path(str(HERE / 'interim-front'))['main']
REAL_RUN = subprocess.run
EDGES = '0\t1\tfalse\tfalse\n1\t0\tfalse\tfalse\n'


class Interim(unittest.TestCase):
    def execute(self, *, censor=None, wrong_graph=False, negative=False, a_graph=20):
        parent = HERE / 'target/interim-tests'
        parent.mkdir(parents=True, exist_ok=True)
        with tempfile.TemporaryDirectory(dir=parent) as temporary:
            root = Path(temporary)
            binary = root / 'candidate'; binary.write_bytes(b'candidate')
            oracle = root / 'oracle'; oracle.write_bytes(b'oracle')
            out = root / 'result'
            def run(command, **kwargs):
                if command[0] == 'cmp':
                    return REAL_RUN(command, **kwargs)
                self.assertEqual(Path(command[2]).name, 'probe.py')
                case = Path(command[command.index('--out') + 1])
                metrics = case / 'metrics'; metrics.mkdir(parents=True)
                name = case.name
                phases = {'parse_shape': 2, 'may_fixpoint': 40, 'graph_scc': 60}
                if name == 'oracle-pair':
                    phases.update(origin_growth=30, k1_growth=20)
                    (metrics / 'analysis.tsv').write_text('verdict\tpeak_rss_bytes\tedges\nadmitted\t1000\t2\n')
                else:
                    if name == 'front-F1b':
                        phases = {'parse_shape': 1, 'may_fixpoint': 10, 'graph_scc': 10}
                    if name == 'front-F1a':
                        phases = {'parse_shape': 1, 'may_fixpoint': 20, 'graph_scc': a_graph}
                    fields = {'peak_rss_bytes': 1000, 'edges': 2, 'body_solves': 2,
                              'header_candidates': 3, 'header_survivors': 2}
                    if name.startswith('front-'):
                        fields['negative_components'] = int(negative)
                    (metrics / 'front-stats.tsv').write_text('key\tvalue\n' + ''.join(f'{k}\t{v}\n' for k, v in fields.items()))
                (metrics / 'phases.tsv').write_text('phase\tseconds\n' + ''.join(f'{k}\t{v}\n' for k, v in phases.items()))
                (case / 'scope-wall.txt').write_text('9999\n')
                (metrics / 'front-edges.tsv').write_text(EDGES.replace('0\t1', '0\t0', 1) if wrong_graph and name == 'front-F1b' else EDGES)
                return subprocess.CompletedProcess(command, 124 if name == censor else 0)
            argv = ['interim-front', '--ta', str(binary), '--oracle', str(oracle), '--root', str(root),
                    '--instances', str(root), '--out', str(out), '--source-commit', 'a' * 40]
            with patch.object(sys, 'argv', argv), patch.object(subprocess, 'run', run):
                rc = MAIN()
            decision = dict(line.split('\t', 1) for line in (out / 'decision.tsv').read_text().splitlines()[1:])
            return rc, decision

    def test_paired_speed_and_both_projections_follow_D4_not_wrapper_wall(self):
        rc, d = self.execute()
        self.assertEqual(rc, 0); self.assertEqual(d['outcome'], 'continue')
        self.assertEqual(float(d['s']), 5)
        self.assertEqual(float(d['projection_F1b']), 31)
        self.assertEqual(float(d['projection_F1a']), 51)
        self.assertEqual(d['cmp_rc'], '0')

    def test_exact_1650_line_passes_and_one_hundredth_over_stops(self):
        rc, d = self.execute(a_graph=1619)
        self.assertEqual(rc, 0); self.assertEqual(float(d['projection_F1a']), 1650)
        self.assertEqual(d['outcome'], 'continue')
        rc, d = self.execute(a_graph=1619.01)
        self.assertEqual(rc, 0); self.assertEqual(d['outcome'], 'stop')

    def test_censor_at_each_position_stops_with_no_projection(self):
        for name in ['oracle-pair', 'front-F1b', 'front-F1a', 'oracle-dump']:
            rc, d = self.execute(censor=name)
            self.assertEqual(rc, 1); self.assertEqual(d['outcome'], 'stop')
            self.assertNotIn('s', d)

    def test_wrong_C3_bytes_stop_even_when_counts_match(self):
        rc, d = self.execute(wrong_graph=True)
        self.assertEqual(rc, 1); self.assertEqual(d['outcome'], 'stop')
        self.assertEqual(d['cmp_rc'], '1')
        self.assertNotEqual(d['oracle_edges_sha256'], d['redesign_edges_sha256'])

    def test_front_refusal_is_not_a_fast_completion(self):
        rc, d = self.execute(negative=True)
        self.assertEqual(rc, 1); self.assertEqual(d['outcome'], 'stop')


if __name__ == '__main__':
    unittest.main()
