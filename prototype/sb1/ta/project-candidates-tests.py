#!/usr/bin/env python3
"""Frozen-law arithmetic/eligibility controls; synthetic counts, never timings."""
from fractions import Fraction as F
from pathlib import Path
import runpy
import unittest

M = runpy.run_path(str(Path(__file__).with_name('project-candidates')))


class FrozenLaw(unittest.TestCase):
    def records(self, savings):
        return [dict(family=f, lever=m, saving_seconds=str(v), status='estimated', reason='')
                for f in ('F1a', 'F1b', 'F3', 'F4') for m, v in savings.items()]

    def test_polynomial_fit_and_independent_residuals(self):
        for a in (-17, 0, 19):
            for b in (0, 3, 21):
                values = {g: a*g + b*g*g for g in (1, 2, 4, 10)}
                self.assertEqual(M['fit'](values), (a, b, a*100+b*10000, 0, 0))
                values[1] += 2
                values[2] -= 3
                self.assertEqual(M['fit'](values), (a, b, a*100+b*10000, 2, -3))

    def test_negative_prediction_is_not_clamped(self):
        values = {g: 80*g - g*g for g in (1, 2, 4, 10)}
        self.assertEqual(M['fit'](values)[2], -2000)
        model = M['Projection'].__new__(M['Projection'])
        model.predicted = {'F1a': None, ('F1a', 'graph', 'calls'): F(-2000)}
        with self.assertRaisesRegex(ValueError, 'negative extrapolation'):
            model.p('F1a', 'graph', 'calls')

    def test_subset_violation_stops(self):
        model = M['Projection'].__new__(M['Projection'])
        model.predicted = {('F1a', 'graph', 'calls'): F(3), ('F1a', 'graph', 'full_hits'): F(4)}
        with self.assertRaisesRegex(ValueError, 'subset violation'):
            model.check_subsets('F1a', 'graph', [('full_hits', 'calls')])

    def test_combinations_require_every_single_below_f1a_threshold(self):
        records = self.records({'a1': 400, 'a2': F('602.4488'), 'b': 601, 'a1+b': 1000, 'a2+b': 2000})
        result = M['select'](records)
        self.assertEqual(result['eligible'], ['a1', 'a2', 'b'])
        self.assertEqual(result['lever'], 'a2')
        records = self.records({'a1': 400, 'a2': F('602.4487'), 'b': 601, 'a1+b': 1000, 'a2+b': 2000})
        self.assertEqual(M['select'](records)['lever'], 'a2+b')

    def test_within_ten_percent_prefers_nonrepeat(self):
        records = self.records({'a1': 899, 'a2': 900, 'b': 1000, 'a1+b': 1001, 'a2+b': 2000})
        self.assertEqual(M['select'](records)['lever'], 'a2')
        records = self.records({'a1': 899, 'a2': 899, 'b': 1000, 'a1+b': 1001, 'a2+b': 2000})
        self.assertEqual(M['select'](records)['lever'], 'b')

    def test_nonestimable_winner_is_not_fabricated(self):
        records = self.records({'a1': 700, 'a2': 800, 'b': 1000, 'a1+b': 1001, 'a2+b': 2000})
        records[2]['status'] = 'non-estimable'
        self.assertEqual(M['select'](records)['decision'], 'stop')

    def cache_model(self):
        model = M['Projection'].__new__(M['Projection'])
        counts = dict(native=6, point_checks=12, point_pass=4, survivors=3, forced=3,
                      calls=10, native_union=6, composite_points=8, candidate_hits=4,
                      full_hits=2, full_skip_forced=1, key_cells=100, cache_store_units=18,
                      candidate_hit_same=2, candidate_hit_cross=2, candidate_hit_unclassified=0,
                      full_hit_same=1, full_hit_cross=1, full_hit_unclassified=0)
        for work, skipped in (('native_union', 2), ('point_checks', 4), ('point_pass', 2), ('calls', 4)):
            counts['candidate_skip_'+work] = skipped
            counts['candidate_miss_'+work] = counts[work] - skipped
        model.predicted = {('F1a', 'graph', k): F(v) for k, v in counts.items()}
        rates = {k: F(1) for k in ('skip_native', 'skip_point', 'skip_prefix', 'skip_wrapper',
                                  'skip_forced', 'key', 'store', 'hit')}
        model.costs = {('F1a', 'graph'): rates}
        model.cost_units = {('F1a', 'graph'): dict(rates)}
        return model

    def test_two_cache_returns_and_uncalibrated_cost_are_charged(self):
        model = self.cache_model()
        self.assertEqual(model.saving('F1a', 'graph', None, True), -111)
        model.cost_units['F1a', 'graph']['skip_forced'] = 0
        with self.assertRaisesRegex(ValueError, 'no G10 calibration units'):
            model.saving('F1a', 'graph', None, True)

    def test_repeat_subclasses_are_nonnegative_subsets_and_a_partition(self):
        model = self.cache_model()
        model.predicted['F1a', 'graph', 'candidate_hit_same'] = -1
        with self.assertRaisesRegex(ValueError, 'negative extrapolation'):
            model.saving('F1a', 'graph', None, True)
        model = self.cache_model()
        model.predicted['F1a', 'graph', 'candidate_hit_cross'] = 5
        with self.assertRaisesRegex(ValueError, 'subset violation'):
            model.saving('F1a', 'graph', None, True)
        model = self.cache_model()
        model.predicted['F1a', 'graph', 'full_hit_cross'] = 0
        with self.assertRaisesRegex(ValueError, 'hit subclass partition'):
            model.saving('F1a', 'graph', None, True)

    def test_zero_work_does_not_invent_a_rate(self):
        self.assertEqual(M['ratio'](F(0), F(0)), 0)
        with self.assertRaises(ValueError):
            M['ratio'](F(1), F(0))


if __name__ == '__main__':
    unittest.main()
