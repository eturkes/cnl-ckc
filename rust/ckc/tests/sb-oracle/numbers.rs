use super::model::{Binding, BuiltinKind, Term, atom};
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rational {
    pub numerator: i128,
    pub denominator: i128,
}

impl Rational {
    pub fn new(mut numerator: i128, mut denominator: i128) -> Self {
        assert_ne!(denominator, 0);
        if denominator < 0 {
            numerator = -numerator;
            denominator = -denominator;
        }
        let (mut a, mut b) = (numerator.abs(), denominator);
        while b != 0 {
            (a, b) = (b, a % b);
        }
        Self {
            numerator: numerator / a,
            denominator: denominator / a,
        }
    }

    pub fn render(self) -> String {
        if self.denominator == 1 {
            self.numerator.to_string()
        } else {
            format!("rational({},{})", self.numerator, self.denominator)
        }
    }

    pub fn plus(self, rhs: Self) -> Self {
        Self::new(
            self.numerator * rhs.denominator + rhs.numerator * self.denominator,
            self.denominator * rhs.denominator,
        )
    }

    pub fn times(self, rhs: Self) -> Self {
        Self::new(
            self.numerator * rhs.numerator,
            self.denominator * rhs.denominator,
        )
    }

    pub fn divided_by(self, rhs: Self) -> Self {
        Self::new(
            self.numerator * rhs.denominator,
            self.denominator * rhs.numerator,
        )
    }

    fn sign(self) -> i128 {
        self.numerator.signum()
    }
}

impl Ord for Rational {
    fn cmp(&self, rhs: &Self) -> Ordering {
        (self.numerator * rhs.denominator).cmp(&(rhs.numerator * self.denominator))
    }
}

impl PartialOrd for Rational {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Extended {
    NegativeInfinity,
    Finite(Rational),
    PositiveInfinity,
}

impl Extended {
    fn times(self, rhs: Self) -> Self {
        use Extended::*;
        match (self, rhs) {
            (Finite(a), Finite(b)) => Finite(a.times(b)),
            (Finite(a), _) | (_, Finite(a)) if a.sign() == 0 => Finite(a),
            (a, b) => {
                let sign = |v| match v {
                    NegativeInfinity => -1,
                    Finite(n) => n.sign(),
                    PositiveInfinity => 1,
                };
                if sign(a) * sign(b) < 0 {
                    NegativeInfinity
                } else {
                    PositiveInfinity
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Interval {
    low: Extended,
    low_closed: bool,
    high: Extended,
    high_closed: bool,
}

impl Interval {
    fn from_pair(comparator: &str, count: Rational) -> Option<Self> {
        use Extended::*;
        let point = Finite(count);
        let (low, low_closed, high, high_closed) = match comparator {
            "exactly" => (point, true, point, true),
            "eq" | "geq" => (point, true, PositiveInfinity, false),
            "greater" => (point, false, PositiveInfinity, false),
            "leq" => (NegativeInfinity, false, point, true),
            "less" => (NegativeInfinity, false, point, false),
            _ => return None,
        };
        Some(Self {
            low,
            low_closed,
            high,
            high_closed,
        })
    }

    fn subset_of(self, other: Self) -> bool {
        (self.low > other.low || (self.low == other.low && (!self.low_closed || other.low_closed)))
            && (self.high < other.high
                || (self.high == other.high && (!self.high_closed || other.high_closed)))
    }

    fn contains_zero(self) -> bool {
        let zero = Extended::Finite(Rational::new(0, 1));
        (self.low < zero || (self.low == zero && self.low_closed))
            && (self.high > zero || (self.high == zero && self.high_closed))
    }

    fn product(self, other: Self) -> Self {
        let mut corners = Vec::with_capacity(4);
        for (a, a_closed) in [(self.low, self.low_closed), (self.high, self.high_closed)] {
            for (b, b_closed) in [
                (other.low, other.low_closed),
                (other.high, other.high_closed),
            ] {
                corners.push((a.times(b), a_closed && b_closed));
            }
        }
        let low = corners.iter().map(|(n, _)| *n).min().unwrap();
        let high = corners.iter().map(|(n, _)| *n).max().unwrap();
        let attained = |bound| {
            if bound == Extended::Finite(Rational::new(0, 1)) {
                self.contains_zero() || other.contains_zero()
            } else {
                matches!(bound, Extended::Finite(_))
                    && corners
                        .iter()
                        .any(|(value, closed)| *value == bound && *closed)
            }
        };
        Self {
            low,
            low_closed: attained(low),
            high,
            high_closed: attained(high),
        }
    }

    fn as_pair(self) -> Option<(&'static str, Rational)> {
        use Extended::*;
        match (self.low, self.high) {
            (Finite(a), Finite(b)) if a == b && self.low_closed && self.high_closed => {
                Some(("exactly", a))
            }
            (Finite(a), PositiveInfinity) => {
                Some((if self.low_closed { "geq" } else { "greater" }, a))
            }
            (NegativeInfinity, Finite(b)) => {
                Some((if self.high_closed { "leq" } else { "less" }, b))
            }
            _ => None,
        }
    }
}

pub fn unit_factor(unit: &str) -> Option<(u8, i128)> {
    Some(match unit {
        "second" => (0, 1),
        "minute" => (0, 60),
        "hour" => (0, 3_600),
        "day" => (0, 86_400),
        "week" => (0, 604_800),
        "month" => (1, 1),
        "year" => (1, 12),
        _ => return None,
    })
}

pub fn conversion(from: &str, to: &str) -> Option<Rational> {
    let (component_a, a) = unit_factor(from)?;
    let (component_b, b) = unit_factor(to)?;
    (component_a == component_b).then(|| Rational::new(a, b))
}

fn atom_value(term: &Term) -> Option<&str> {
    if let Term::Atom(s) = term {
        Some(s)
    } else {
        None
    }
}

fn number_value(term: &Term) -> Option<Rational> {
    if let Term::Number(n) = term {
        Some(*n)
    } else {
        None
    }
}

fn pair(comparator: &Term, count: &Term) -> Option<Interval> {
    Interval::from_pair(atom_value(comparator)?, number_value(count)?)
}

pub fn arithmetic(kind: BuiltinKind, inputs: &[Term]) -> Option<(&'static str, Rational)> {
    let left = pair(&inputs[0], &inputs[1])?;
    let right = if kind == BuiltinKind::Quotient {
        if atom_value(&inputs[2])? != "exactly" {
            return None;
        }
        let divisor = number_value(&inputs[3])?;
        if divisor.sign() == 0 {
            return None;
        }
        Interval::from_pair("exactly", Rational::new(1, 1).divided_by(divisor))?
    } else {
        pair(&inputs[2], &inputs[3])?
    };
    left.product(right).as_pair()
}

pub fn apply(kind: BuiltinKind, args: &[Term], env: &Binding) -> Option<Binding> {
    let input_count = match kind {
        BuiltinKind::Satisfies4 => 4,
        BuiltinKind::Satisfies6 => 6,
        BuiltinKind::Product | BuiltinKind::Quotient => 4,
    };
    let inputs = args[..input_count]
        .iter()
        .map(|t| t.substitute(env))
        .collect::<Option<Vec<_>>>()
        .expect("frozen domain: built-in inputs bound");
    if matches!(kind, BuiltinKind::Product | BuiltinKind::Quotient) {
        let (comparison, value) = arithmetic(kind, &inputs)?;
        let mut result = env.clone();
        if args[4].match_ground(&atom(comparison), &mut result)
            && args[5].match_ground(&Term::Number(value), &mut result)
        {
            Some(result)
        } else {
            None
        }
    } else {
        let (left, right) = if kind == BuiltinKind::Satisfies4 {
            (pair(&inputs[0], &inputs[1])?, pair(&inputs[2], &inputs[3])?)
        } else {
            let multiplier = conversion(atom_value(&inputs[2])?, atom_value(&inputs[5])?)?;
            let source_count = number_value(&inputs[1])?.times(multiplier);
            (
                Interval::from_pair(atom_value(&inputs[0])?, source_count)?,
                pair(&inputs[3], &inputs[4])?,
            )
        };
        left.subset_of(right).then(|| env.clone())
    }
}
