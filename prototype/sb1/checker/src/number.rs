use crate::term::{Arena, Id, Node};
use std::cmp::Ordering;

const BASE: u64 = 1_000_000_000;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Nat(Vec<u32>);

impl Ord for Nat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0
            .len()
            .cmp(&other.0.len())
            .then_with(|| self.0.iter().rev().cmp(other.0.iter().rev()))
    }
}
impl PartialOrd for Nat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Nat {
    fn small(n: u32) -> Self {
        if n == 0 {
            Self(Vec::new())
        } else {
            Self(vec![n])
        }
    }
    fn parse(s: &str) -> Option<Self> {
        if s.is_empty() || !s.bytes().all(|b| b.is_ascii_digit()) {
            return None;
        }
        let mut n = Self::small(0);
        for b in s.bytes() {
            n = n.mul_small(10);
            n.add_small(u32::from(b - b'0'));
        }
        Some(n)
    }
    fn trim(&mut self) {
        while self.0.last() == Some(&0) {
            self.0.pop();
        }
    }
    fn zero(&self) -> bool {
        self.0.is_empty()
    }
    fn add_small(&mut self, n: u32) {
        let mut carry = u64::from(n);
        let mut i = 0;
        while carry > 0 {
            if i == self.0.len() {
                self.0.push(0);
            }
            carry += u64::from(self.0[i]);
            self.0[i] = (carry % BASE) as u32;
            carry /= BASE;
            i += 1;
        }
    }
    fn mul_small(&self, n: u32) -> Self {
        let mut out = Vec::with_capacity(self.0.len() + 1);
        let mut carry = 0;
        for &x in &self.0 {
            let value = u64::from(x) * u64::from(n) + carry;
            out.push((value % BASE) as u32);
            carry = value / BASE;
        }
        if carry > 0 {
            out.push(carry as u32);
        }
        let mut n = Self(out);
        n.trim();
        n
    }
    fn sub(&self, b: &Self) -> Self {
        assert!(self >= b);
        let mut out = self.clone();
        let mut borrow = 0i64;
        for i in 0..out.0.len() {
            let value = i64::from(out.0[i]) - i64::from(*b.0.get(i).unwrap_or(&0)) - borrow;
            borrow = i64::from(value < 0);
            out.0[i] = (value + borrow * BASE as i64) as u32;
        }
        out.trim();
        out
    }
    fn mul(&self, b: &Self) -> Self {
        let mut out = vec![0u32; self.0.len() + b.0.len()];
        for (i, &x) in self.0.iter().enumerate() {
            let mut carry = 0;
            for (j, &y) in b.0.iter().enumerate() {
                let value = u64::from(x) * u64::from(y) + u64::from(out[i + j]) + carry;
                out[i + j] = (value % BASE) as u32;
                carry = value / BASE;
            }
            if !b.0.is_empty() {
                out[i + b.0.len()] = carry as u32;
            }
        }
        let mut n = Self(out);
        n.trim();
        n
    }
    fn divmod(&self, divisor: &Self) -> (Self, Self) {
        assert!(!divisor.zero());
        let mut q = vec![0u32; self.0.len()];
        let mut rem = Self::small(0);
        for i in (0..self.0.len()).rev() {
            rem.0.insert(0, self.0[i]);
            rem.trim();
            let mut low = 0u64;
            let mut high = BASE;
            while low + 1 < high {
                let mid = (low + high) / 2;
                if divisor.mul_small(mid as u32) <= rem {
                    low = mid;
                } else {
                    high = mid;
                }
            }
            q[i] = low as u32;
            rem = rem.sub(&divisor.mul_small(low as u32));
        }
        let mut q = Self(q);
        q.trim();
        (q, rem)
    }
    fn gcd(mut a: Self, mut b: Self) -> Self {
        while !b.zero() {
            let (_, r) = a.divmod(&b);
            a = b;
            b = r;
        }
        a
    }
    fn decimal(&self) -> String {
        let Some(last) = self.0.last() else {
            return "0".into();
        };
        let mut out = last.to_string();
        for n in self.0.iter().rev().skip(1) {
            out.push_str(&format!("{n:09}"));
        }
        out
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Rational {
    negative: bool,
    num: Nat,
    den: Nat,
}

impl Rational {
    fn new(negative: bool, num: Nat, den: Nat) -> Self {
        let gcd = Nat::gcd(num.clone(), den.clone());
        Self {
            negative: negative && !num.zero(),
            num: num.divmod(&gcd).0,
            den: den.divmod(&gcd).0,
        }
    }
    pub fn from_term(arena: &Arena, term: Id) -> Option<Self> {
        match arena.node(term) {
            Node::Integer(n) => Some(Self {
                negative: n.starts_with('-'),
                num: Nat::parse(n.trim_start_matches('-'))?,
                den: Nat::small(1),
            }),
            Node::Compound(name, args) if arena.name(*name) == "rational" && args.len() == 2 => {
                let Node::Integer(p) = arena.node(args[0]) else {
                    return None;
                };
                let Node::Integer(q) = arena.node(args[1]) else {
                    return None;
                };
                let num = Nat::parse(p.trim_start_matches('-'))?;
                let den = Nat::parse(q)?;
                if den < Nat::small(2) || Nat::gcd(num.clone(), den.clone()) != Nat::small(1) {
                    return None;
                }
                Some(Self {
                    negative: p.starts_with('-'),
                    num,
                    den,
                })
            }
            _ => None,
        }
    }
    pub fn term(&self, arena: &mut Arena) -> Id {
        let mut p = self.num.decimal();
        if self.negative {
            p.insert(0, '-');
        }
        let p = arena.put(Node::Integer(p));
        if self.den == Nat::small(1) {
            p
        } else {
            let q = arena.put(Node::Integer(self.den.decimal()));
            arena.compound("rational", vec![p, q])
        }
    }
    fn zero(&self) -> bool {
        self.num.zero()
    }
    fn sign(&self) -> Ordering {
        if self.zero() {
            Ordering::Equal
        } else if self.negative {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
    pub fn compare(&self, b: &Self) -> Ordering {
        self.sign().cmp(&b.sign()).then_with(|| {
            let order = self.num.mul(&b.den).cmp(&b.num.mul(&self.den));
            if self.negative {
                order.reverse()
            } else {
                order
            }
        })
    }
    fn mul(&self, b: &Self) -> Self {
        Self::new(
            self.negative != b.negative,
            self.num.mul(&b.num),
            self.den.mul(&b.den),
        )
    }
    fn inverse(&self) -> Option<Self> {
        (!self.zero()).then(|| Self {
            negative: self.negative,
            num: self.den.clone(),
            den: self.num.clone(),
        })
    }
    fn scaled(&self, scale: u32) -> Self {
        Self::new(self.negative, self.num.mul_small(scale), self.den.clone())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Kind {
    Point,
    Lower,
    Upper,
}
#[derive(Clone)]
struct ValueSet {
    kind: Kind,
    bound: Rational,
    closed: bool,
}

impl ValueSet {
    fn read(arena: &Arena, cmp: Id, n: Id) -> Option<Self> {
        let (kind, closed) = match arena.atom_name(cmp)? {
            "exactly" => (Kind::Point, true),
            "eq" | "geq" => (Kind::Lower, true),
            "greater" => (Kind::Lower, false),
            "leq" => (Kind::Upper, true),
            "less" => (Kind::Upper, false),
            _ => return None,
        };
        Some(Self {
            kind,
            bound: Rational::from_term(arena, n)?,
            closed,
        })
    }
    fn stretch(&self, factor: &Rational) -> Self {
        let bound = self.bound.mul(factor);
        let kind = if factor.zero() {
            Kind::Point
        } else if factor.negative {
            match self.kind {
                Kind::Point => Kind::Point,
                Kind::Lower => Kind::Upper,
                Kind::Upper => Kind::Lower,
            }
        } else {
            self.kind
        };
        Self {
            kind,
            bound,
            closed: factor.zero() || self.closed,
        }
    }
    fn product(&self, b: &Self) -> Option<Self> {
        if self.kind == Kind::Point {
            return Some(b.stretch(&self.bound));
        }
        if b.kind == Kind::Point {
            return Some(self.stretch(&b.bound));
        }
        let nonnegative = |s: &Self| s.kind == Kind::Lower && !s.bound.negative;
        let nonpositive = |s: &Self| s.kind == Kind::Upper && s.bound.sign() != Ordering::Greater;
        let kind = if (nonnegative(self) && nonnegative(b)) || (nonpositive(self) && nonpositive(b))
        {
            Kind::Lower
        } else if (nonnegative(self) && nonpositive(b)) || (nonpositive(self) && nonnegative(b)) {
            Kind::Upper
        } else {
            return None;
        };
        let closed = (self.closed && b.closed)
            || (self.closed && self.bound.zero())
            || (b.closed && b.bound.zero());
        Some(Self {
            kind,
            bound: self.bound.mul(&b.bound),
            closed,
        })
    }
    fn spell(&self, arena: &mut Arena) -> (Id, Id) {
        let cmp = match (self.kind, self.closed) {
            (Kind::Point, _) => "exactly",
            (Kind::Lower, true) => "geq",
            (Kind::Lower, false) => "greater",
            (Kind::Upper, true) => "leq",
            (Kind::Upper, false) => "less",
        };
        let cmp = arena.atom(cmp);
        let count = self.bound.term(arena);
        (cmp, count)
    }
}

fn unit(arena: &Arena, id: Id) -> Option<(u8, u32)> {
    Some(match arena.atom_name(id)? {
        "second" => (0, 1),
        "minute" => (0, 60),
        "hour" => (0, 3600),
        "day" => (0, 86400),
        "week" => (0, 604800),
        "month" => (1, 1),
        "year" => (1, 12),
        _ => return None,
    })
}

pub fn satisfies(arena: &Arena, args: &[Id]) -> bool {
    let (fact_cmp, fact_count, cond_cmp, cond_count, scale_f, scale_c) = match args {
        [s, n, b, m] => (*s, *n, *b, *m, 1, 1),
        [s, n, u, b, m, v] => {
            let Some((fg, fs)) = unit(arena, *u) else {
                return false;
            };
            let Some((cg, cs)) = unit(arena, *v) else {
                return false;
            };
            if fg != cg {
                return false;
            }
            (*s, *n, *b, *m, fs, cs)
        }
        _ => return false,
    };
    let Some(mut fact) = ValueSet::read(arena, fact_cmp, fact_count) else {
        return false;
    };
    let Some(mut cond) = ValueSet::read(arena, cond_cmp, cond_count) else {
        return false;
    };
    if arena.atom_name(cond_cmp) == Some("eq") {
        return false;
    }
    fact.bound = fact.bound.scaled(scale_f);
    cond.bound = cond.bound.scaled(scale_c);
    let cmp = fact.bound.compare(&cond.bound);
    match (fact.kind, cond.kind) {
        (Kind::Point, Kind::Point) => cmp == Ordering::Equal,
        (_, Kind::Point) => false,
        (Kind::Point, Kind::Lower) | (Kind::Lower, Kind::Lower) => {
            cmp == Ordering::Greater || (cmp == Ordering::Equal && (cond.closed || !fact.closed))
        }
        (Kind::Point, Kind::Upper) | (Kind::Upper, Kind::Upper) => {
            cmp == Ordering::Less || (cmp == Ordering::Equal && (cond.closed || !fact.closed))
        }
        _ => false,
    }
}

pub fn arithmetic(arena: &mut Arena, name: &str, args: &[Id]) -> Option<(Id, Id)> {
    if args.len() != 4 {
        return None;
    }
    let left = ValueSet::read(arena, args[0], args[1])?;
    let mut right = ValueSet::read(arena, args[2], args[3])?;
    if name == "guideline_quotient" {
        if right.kind != Kind::Point {
            return None;
        }
        right.bound = right.bound.inverse()?;
    }
    left.product(&right).map(|set| set.spell(arena))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::term::parse_term;
    #[test]
    fn exact_big_rationals_and_ray_endpoints() {
        let mut a = Arena::default();
        let mut eval = |text: &str| {
            let term = parse_term(&mut a, text).unwrap();
            let (_, args) = a.call(term).unwrap();
            let args = args.to_vec();
            let (c, n) = arithmetic(&mut a, "guideline_product", &args[..4]).unwrap();
            (a.render(c), a.render(n))
        };
        assert_eq!(
            eval("x(exactly,123456789012345678901234567890,exactly,rational(3,20))"),
            (
                "exactly".into(),
                "rational(37037036703703703670370370367,2)".into()
            )
        );
        assert_eq!(eval("x(geq,0,greater,3)"), ("geq".into(), "0".into()));
        assert_eq!(eval("x(less,0,less,0)"), ("greater".into(), "0".into()));
    }
    #[test]
    fn nat_operations_match_u128_domain() {
        for x in [
            0u128,
            1,
            2,
            19,
            999_999_999,
            1_000_000_001,
            99_999_999_999_999_999_999,
        ] {
            for y in [1u128, 2, 13, 999_999_999, 1_000_000_007] {
                let a = Nat::parse(&x.to_string()).unwrap();
                let b = Nat::parse(&y.to_string()).unwrap();
                assert_eq!(a.mul(&b).decimal(), (x * y).to_string());
                let (q, r) = a.divmod(&b);
                assert_eq!(
                    (q.decimal(), r.decimal()),
                    ((x / y).to_string(), (x % y).to_string())
                );
            }
        }
    }
}
