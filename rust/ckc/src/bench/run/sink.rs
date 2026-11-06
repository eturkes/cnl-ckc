use super::{Result, ns, report};
use std::fs;
use std::path::Path;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

const ROUNDING_NS: u64 = 1_000;

pub(super) struct Complete {
    pub at: u64,
    pub bytes: u64,
    pub sha256: String,
    pub cpu_ticks: u64,
    pub uncertainty: u64,
}

pub(super) struct Clock {
    unix: u128,
    before: u64,
    after: u64,
}
fn unix_ns() -> Result<u128> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .map_err(|e| e.to_string())
}
impl Clock {
    pub fn capture(origin: Instant) -> Result<Self> {
        let before = ns(origin.elapsed());
        let unix = unix_ns()?;
        let after = ns(origin.elapsed());
        Ok(Self {
            unix,
            before,
            after,
        })
    }
    // UNIX ns of a controller-relative instant at or after the capture.
    pub fn unix_at(&self, rel: u64) -> u128 {
        self.unix
            .saturating_add(u128::from(rel.saturating_sub(self.before)))
    }
    fn validate(&self, unix: u128, before: u64, after: u64) -> Result<()> {
        let delta = u64::try_from(
            unix.checked_sub(self.unix)
                .ok_or("sink clock moved backwards")?,
        )
        .map_err(|_| "sink clock overflow")?;
        let predicted_before = self
            .before
            .checked_add(delta)
            .ok_or("sink clock overflow")?;
        let predicted_after = self.after.checked_add(delta).ok_or("sink clock overflow")?;
        // Brackets include scheduler preemption; one microsecond covers clock-read rounding.
        if predicted_after.saturating_add(ROUNDING_NS) < before
            || after.saturating_add(ROUNDING_NS) < predicted_before
        {
            return Err("sink clock discontinuity".to_owned());
        }
        Ok(())
    }
    pub fn complete(
        &self,
        work: &Path,
        origin: Instant,
        start: u64,
        end: u64,
        final_cpu: u64,
    ) -> Result<Complete> {
        let bytes =
            fs::read(work.join("sink-complete.tsv")).map_err(|e| format!("sink-complete: {e}"))?;
        let text = std::str::from_utf8(&bytes).map_err(|_| "sink-complete UTF-8")?;
        if !text.ends_with('\n') || text.contains('\r') {
            return Err("sink-complete LF framing".to_owned());
        }
        let mut lines = text.lines();
        if lines.next() != Some("sb1-sink 1") {
            return Err("sink-complete version".to_owned());
        }
        let mut field = |name: &str| -> Result<&str> {
            let (key, value) = lines
                .next()
                .and_then(|line| line.split_once('\t'))
                .ok_or("sink-complete fields")?;
            if key != name || value.is_empty() {
                return Err("sink-complete field order".to_owned());
            }
            Ok(value)
        };
        let closed = field("closed_unix_ns")?
            .parse::<u128>()
            .map_err(|_| "sink-complete clock")?;
        let size = field("bytes")?
            .parse::<u64>()
            .map_err(|_| "sink-complete bytes")?;
        let digest = field("sha256")?.to_owned();
        let cpu_ticks = field("cpu_ticks")?
            .parse::<u64>()
            .map_err(|_| "sink-complete CPU")?;
        if lines.next().is_some()
            || digest.len() != 64
            || !digest
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        {
            return Err("sink-complete shape".to_owned());
        }
        let now_before = ns(origin.elapsed());
        let now_unix = unix_ns()?;
        let now_after = ns(origin.elapsed());
        self.validate(now_unix, now_before, now_after)?;
        let delta = u64::try_from(
            closed
                .checked_sub(self.unix)
                .ok_or("sink predates clock origin")?,
        )
        .map_err(|_| "sink clock overflow")?;
        let at = self.before + (self.after - self.before) / 2;
        let at = at.checked_add(delta).ok_or("sink clock overflow")?;
        let uncertainty = self.after - self.before + ROUNDING_NS;
        if at < start || at > end || cpu_ticks > final_cpu {
            return Err("sink-complete outside owner interval".to_owned());
        }
        let (actual_size, actual_hash) = report::digest(&work.join("answers.txt"))?;
        if size != actual_size || digest != actual_hash {
            return Err("sink-complete answer digest/bytes mismatch".to_owned());
        }
        Ok(Complete {
            at,
            bytes: size,
            sha256: digest,
            cpu_ticks,
            uncertainty,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn clock_brackets_keep_preemption_and_refuse_steps() {
        let clock = Clock {
            unix: 1_000_000_000,
            before: 100,
            after: 200,
        };
        assert!(
            clock
                .validate(1_100_000_000, 100_000_100, 100_000_200)
                .is_ok()
        );
        assert!(
            clock
                .validate(1_100_000_000, 99_000_000, 101_000_000)
                .is_ok()
        );
        assert_eq!(
            clock
                .validate(1_200_000_000, 100_000_100, 100_000_200)
                .unwrap_err(),
            "sink clock discontinuity"
        );
        assert_eq!(
            clock
                .validate(1_050_000_000, 100_000_100, 100_000_200)
                .unwrap_err(),
            "sink clock discontinuity"
        );
        assert_eq!(
            clock
                .validate(999_999_999, 100_000_100, 100_000_200)
                .unwrap_err(),
            "sink clock moved backwards"
        );
    }
}
