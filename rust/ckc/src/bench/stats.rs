// Statistics shared by the harness report (`ckc bench run`) + `ckc bench grade` (contract sb D1/D6):
// one definition each, so a grade's recomputed R equals the harness's on the same pairs.
pub(super) struct Random(u64);
impl Random {
    pub fn new(seed: u64) -> Self {
        Self(seed)
    }
    pub fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut value = self.0;
        value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        value ^ (value >> 31)
    }
    pub fn below(&mut self, bound: usize) -> usize {
        let bound = bound as u64;
        let threshold = bound.wrapping_neg() % bound;
        loop {
            let value = self.next();
            if value >= threshold {
                return (value % bound) as usize;
            }
        }
    }
}
fn percentile(values: &[f64], quantile: f64) -> f64 {
    let rank = (values.len() - 1) as f64 * quantile;
    let lower = rank.floor() as usize;
    let upper = rank.ceil() as usize;
    values[lower] + (values[upper] - values[lower]) * rank.fract()
}
/// (median, IQR, max) of a nonempty sample, linear-interpolated percentiles.
pub(super) fn stats(mut values: Vec<f64>) -> (f64, f64, f64) {
    values.sort_by(f64::total_cmp);
    (
        percentile(&values, 0.5),
        percentile(&values, 0.75) - percentile(&values, 0.25),
        values[values.len() - 1],
    )
}
/// R = median(target) / median(baseline) over nonempty (baseline, target) pairs + its one-sided 95 %
/// upper bound from 10,000 whole-pair bootstrap resamples drawn by `Random::new(seed)`.
pub(super) fn ratio(pairs: &[(f64, f64)], seed: u64) -> (f64, f64) {
    let median = |pairs: &[(f64, f64)]| {
        stats(pairs.iter().map(|p| p.1).collect()).0 / stats(pairs.iter().map(|p| p.0).collect()).0
    };
    let mut random = Random::new(seed);
    let mut estimates = Vec::with_capacity(10000);
    for _ in 0..10000 {
        let selected: Vec<_> = (0..pairs.len())
            .map(|_| pairs[random.below(pairs.len())])
            .collect();
        estimates.push(median(&selected));
    }
    estimates.sort_by(f64::total_cmp);
    (
        median(pairs),
        estimates[(0.95 * estimates.len() as f64).ceil() as usize - 1],
    )
}
