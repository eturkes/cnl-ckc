use std::cmp::Ordering;
use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Level {
    Small(u64),
    Big(String),
}
impl Default for Level {
    fn default() -> Self {
        Self::Small(0)
    }
}
impl Level {
    pub fn parse(text: &str) -> Option<Self> {
        if text.is_empty() || !text.bytes().all(|b| b.is_ascii_digit()) {
            return None;
        }
        let digits = text.trim_start_matches('0');
        if digits.is_empty() {
            return Some(Self::Small(0));
        }
        Some(match digits.parse() {
            Ok(n) => Self::Small(n),
            Err(_) => Self::Big(digits.into()),
        })
    }
    pub fn is_zero(&self) -> bool {
        matches!(self, Self::Small(0))
    }
}
impl Ord for Level {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Small(a), Self::Small(b)) => a.cmp(b),
            (Self::Small(_), Self::Big(_)) => Ordering::Less,
            (Self::Big(_), Self::Small(_)) => Ordering::Greater,
            (Self::Big(a), Self::Big(b)) => a.len().cmp(&b.len()).then_with(|| a.cmp(b)),
        }
    }
}
impl PartialOrd for Level {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Small(n) => n.fmt(f),
            Self::Big(n) => f.write_str(n),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Level;

    #[test]
    fn arbitrary_naturals_keep_numeric_order_across_the_machine_boundary() {
        let values = [
            "0",
            "0001",
            "18446744073709551615",
            "18446744073709551616",
            "9999999999999999999999999999999999999999999",
        ];
        let levels = values.map(|n| Level::parse(n).unwrap());
        assert!(levels.windows(2).all(|w| w[0] < w[1]));
        assert_eq!(levels[1].to_string(), "1");
        assert_eq!(Level::parse("000"), Some(Level::Small(0)));
        for bad in ["", "-1", "+1", "1 ", "١"] {
            assert_eq!(Level::parse(bad), None);
        }
    }
}
