#!/usr/bin/env python3
"""C1 validator controls; these mutate diagnostic data, not C5's implementation."""
import copy
from pathlib import Path
import runpy
import tempfile
import unittest

HERE = Path(__file__).resolve().parent
VALIDATE = runpy.run_path(str(HERE / 'control-matrix'))['validate_witnesses']


class Witnesses(unittest.TestCase):
    def setUp(self):
        parent = HERE / 'target/control-matrix-tests'
        parent.mkdir(parents=True, exist_ok=True)
        self.temp = tempfile.TemporaryDirectory(dir=parent)
        self.root = Path(self.temp.name)
        self.source = self.root / 'knowledge'
        self.source.write_text('clause one\nclause two\n')
        self.allowed = {str(self.source)}
        self.label = lambda n: f'{self.source}:{n}:{n}'
        (self.root / 'front-nodes.tsv').write_text(f'0\t{self.source}\t1\tfalse\n1\t{self.source}\t2\tfalse\n')
        (self.root / 'front-edges.tsv').write_text('0\t1\ttrue\tfalse\n1\t0\tfalse\tfalse\n')
        (self.root / 'flow-base-edges.tsv').write_text('')
        (self.root / 'flow-final-edges.tsv').write_text('')
        (self.root / 'growth-edges.tsv').write_text('from\tto\tclause\tvariable\n')
        self.expected = [dict(kind='negative', component='0', step=str(n),
                              **{'from': self.label(n), 'to': self.label(3-n)},
                              sign='-' if n == 1 else '+', growth='false', goal='goal', matched_features='G1 feature=binding')
                         for n in (1, 2)]

    def tearDown(self):
        self.temp.cleanup()

    def test_original_edge_positive_and_four_bad_diagnostics(self):
        self.assertEqual(VALIDATE(self.expected, self.expected, self.root, self.allowed), 1)
        for label, field, value in (('sign', 'sign', '+'), ('features', 'matched_features', 'invented'),
                                    ('source', 'from', self.label(99)), ('unconfirmed', 'matched_features', 'unconfirmed')):
            planted = copy.deepcopy(self.expected)
            planted[0][field] = value
            with self.subTest(label=label), self.assertRaises((ValueError, KeyError)):
                VALIDATE(planted, self.expected, self.root, self.allowed)
        with self.assertRaises(ValueError):
            VALIDATE(self.expected[:-1], self.expected, self.root, self.allowed)
        self.assertEqual(VALIDATE(self.expected, self.expected, self.root, self.allowed), 1)

    def test_base_and_k1_growth_require_real_supporting_site(self):
        for scope in ('origin-', 'k1-'):
            text = 'a\tb\nb\ta\n'
            (self.root / 'flow-base-edges.tsv').write_text(text)
            (self.root / 'flow-final-edges.tsv').write_text(text)
            (self.root / 'growth-edges.tsv').write_text('from\tto\tclause\tvariable\n'+
                f'a\tb\t{self.label(1)}\tA\nb\ta\t{self.label(2)}\tB\n')
            expected = [dict(kind='growth', component=scope+'0', step=str(n),
                             **{'from': start, 'to': end}, sign='+', growth='true',
                             goal=f'input({var}) @ {self.label(n)}', matched_features='G1 feature=binding')
                        for n, start, end, var in ((1, 'a', 'b', 'A'), (2, 'b', 'a', 'B'))]
            self.assertEqual(VALIDATE(expected, expected, self.root, self.allowed), 1)
            bad = copy.deepcopy(expected)
            bad[0]['goal'] = f'input(Z) @ {self.label(1)}'
            with self.assertRaisesRegex(ValueError, 'supporting clause'):
                VALIDATE(bad, expected, self.root, self.allowed)
            self.assertEqual(VALIDATE(expected, expected, self.root, self.allowed), 1)

    def test_empty_positive_and_extra_witness_refused(self):
        self.assertEqual(VALIDATE([], [], self.root, self.allowed), 0)
        with self.assertRaises(ValueError):
            VALIDATE(self.expected, [], self.root, self.allowed)


if __name__ == '__main__':
    unittest.main()
