// Oracle arithmetic: binary limbs + restoring division + binary GCD.
// Independent of the contract's decimal-digit / Euclidean implementation.
use std::cmp::Ordering;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Nat(Vec<u32>);
impl Nat {
    fn small(n: u32) -> Self {
        Self(if n == 0 { vec![] } else { vec![n] })
    }
    fn trim(mut self) -> Self {
        while self.0.last() == Some(&0) {
            self.0.pop();
        }
        self
    }
    fn parse(s: &str) -> Self {
        assert!(!s.is_empty() && s.bytes().all(|b| b.is_ascii_digit()));
        s.bytes().fold(Self::small(0), |n, b| {
            n.mul(&Self::small(10))
                .add(&Self::small(u32::from(b - b'0')))
        })
    }
    fn zero(&self) -> bool {
        self.0.is_empty()
    }
    fn bits(&self) -> usize {
        self.0.last().map_or(0, |x| {
            (self.0.len() - 1) * 32 + (32 - x.leading_zeros()) as usize
        })
    }
    fn bit(&self, n: usize) -> bool {
        self.0.get(n / 32).is_some_and(|x| x & (1 << (n % 32)) != 0)
    }
    fn set(&mut self, n: usize) {
        self.0.resize(self.0.len().max(n / 32 + 1), 0);
        self.0[n / 32] |= 1 << (n % 32);
    }
    fn add(&self, b: &Self) -> Self {
        let mut out = Vec::new();
        let mut carry = 0u64;
        for i in 0..self.0.len().max(b.0.len()) {
            carry += u64::from(*self.0.get(i).unwrap_or(&0)) + u64::from(*b.0.get(i).unwrap_or(&0));
            out.push(carry as u32);
            carry >>= 32;
        }
        if carry != 0 {
            out.push(carry as u32);
        }
        Self(out)
    }
    fn sub(&self, b: &Self) -> Self {
        assert!(self >= b);
        let mut out = Vec::new();
        let mut borrow = 0i64;
        for (i, &a) in self.0.iter().enumerate() {
            let n = i64::from(a) - i64::from(*b.0.get(i).unwrap_or(&0)) - borrow;
            borrow = i64::from(n < 0);
            out.push((n + (borrow << 32)) as u32);
        }
        assert_eq!(borrow, 0);
        Self(out).trim()
    }
    fn mul(&self, b: &Self) -> Self {
        let mut out = vec![0u32; self.0.len() + b.0.len()];
        for (i, &a) in self.0.iter().enumerate() {
            let mut carry = 0u64;
            for (j, &b) in b.0.iter().enumerate() {
                let n = u64::from(a) * u64::from(b) + u64::from(out[i + j]) + carry;
                out[i + j] = n as u32;
                carry = n >> 32;
            }
            if !b.zero() {
                out[i + b.0.len()] = carry as u32;
            }
        }
        Self(out).trim()
    }
    fn shl(&self, n: usize) -> Self {
        let mut out = Self::small(0);
        for i in 0..self.bits() {
            if self.bit(i) {
                out.set(i + n);
            }
        }
        out
    }
    fn shr(&self, n: usize) -> Self {
        let mut out = Self::small(0);
        for i in n..self.bits() {
            if self.bit(i) {
                out.set(i - n);
            }
        }
        out
    }
    fn trailing(&self) -> usize {
        (0..self.bits()).find(|&i| self.bit(i)).expect("nonzero")
    }
    fn div_rem(&self, b: &Self) -> (Self, Self) {
        assert!(!b.zero());
        let (mut q, mut r) = (Self::small(0), Self::small(0));
        for i in (0..self.bits()).rev() {
            r = r.shl(1);
            if self.bit(i) {
                r.set(0);
            }
            if &r >= b {
                r = r.sub(b);
                q.set(i);
            }
        }
        (q, r)
    }
    fn exact_div(&self, b: &Self) -> Self {
        let (q, r) = self.div_rem(b);
        assert!(r.zero());
        q
    }
    fn gcd(mut a: Self, mut b: Self) -> Self {
        if a.zero() {
            return b;
        }
        if b.zero() {
            return a;
        }
        let shift = a.trailing().min(b.trailing());
        a = a.shr(a.trailing());
        b = b.shr(b.trailing());
        while a != b {
            if a > b {
                a = a.sub(&b);
                a = a.shr(a.trailing());
            } else {
                b = b.sub(&a);
                b = b.shr(b.trailing());
            }
        }
        a.shl(shift)
    }
    fn decimal(&self) -> String {
        if self.zero() {
            return "0".into();
        }
        let mut value = self.clone();
        let mut digits = Vec::new();
        while !value.zero() {
            let mut carry = 0u64;
            for digit in value.0.iter_mut().rev() {
                let n = (carry << 32) + u64::from(*digit);
                *digit = (n / 10) as u32;
                carry = n % 10;
            }
            digits.push(char::from(b'0' + carry as u8));
            value = value.trim();
        }
        digits.iter().rev().collect()
    }
}
impl Ord for Nat {
    fn cmp(&self, b: &Self) -> Ordering {
        self.0
            .len()
            .cmp(&b.0.len())
            .then_with(|| self.0.iter().rev().cmp(b.0.iter().rev()))
    }
}
impl PartialOrd for Nat {
    fn partial_cmp(&self, b: &Self) -> Option<Ordering> {
        Some(self.cmp(b))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Rat {
    negative: bool,
    n: Nat,
    d: Nat,
}
impl Rat {
    fn normalized(negative: bool, n: Nat, d: Nat) -> Self {
        assert!(!d.zero());
        let gcd = Nat::gcd(n.clone(), d.clone());
        Self {
            negative: negative && !n.zero(),
            n: n.exact_div(&gcd),
            d: d.exact_div(&gcd),
        }
    }
    pub fn fraction(p: &str, q: &str) -> Self {
        Self::normalized(
            p.starts_with('-') != q.starts_with('-'),
            Nat::parse(p.strip_prefix('-').unwrap_or(p)),
            Nat::parse(q.strip_prefix('-').unwrap_or(q)),
        )
    }
    pub fn new(p: i128, q: i128) -> Self {
        Self::fraction(&p.to_string(), &q.to_string())
    }
    pub fn integer(n: i128) -> Self {
        Self::new(n, 1)
    }
    pub fn zero(&self) -> bool {
        self.n.zero()
    }
    pub fn sign(&self) -> i8 {
        if self.zero() {
            0
        } else if self.negative {
            -1
        } else {
            1
        }
    }
    pub fn neg(&self) -> Self {
        Self {
            negative: !self.negative && !self.zero(),
            n: self.n.clone(),
            d: self.d.clone(),
        }
    }
    pub fn mul(&self, b: &Self) -> Self {
        Self::normalized(
            self.negative != b.negative,
            self.n.mul(&b.n),
            self.d.mul(&b.d),
        )
    }
    pub fn reciprocal(&self) -> Self {
        assert!(!self.zero());
        Self {
            negative: self.negative,
            n: self.d.clone(),
            d: self.n.clone(),
        }
    }
    pub fn add(&self, b: &Self) -> Self {
        let a = self.n.mul(&b.d);
        let c = b.n.mul(&self.d);
        let (negative, n) = if self.negative == b.negative {
            (self.negative, a.add(&c))
        } else if a >= c {
            (self.negative, a.sub(&c))
        } else {
            (b.negative, c.sub(&a))
        };
        Self::normalized(negative, n, self.d.mul(&b.d))
    }
    pub fn term(&self) -> String {
        let p = format!(
            "{}{}",
            if self.negative { "-" } else { "" },
            self.n.decimal()
        );
        if self.d == Nat::small(1) {
            p
        } else {
            format!("rational({p},{})", self.d.decimal())
        }
    }
    pub fn unreduced(&self) -> String {
        format!(
            "rational({}{},{})",
            if self.negative { "-" } else { "" },
            self.n.mul(&Nat::small(2)).decimal(),
            self.d.mul(&Nat::small(2)).decimal()
        )
    }
    pub fn decode(term: &str) -> Option<Self> {
        let integer = |s: &str| {
            let s = s.strip_prefix('-').unwrap_or(s);
            !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit())
        };
        if integer(term) {
            return Some(Self::fraction(term, "1"));
        }
        let body = term.strip_prefix("rational(")?.strip_suffix(')')?;
        let (p, q) = body.split_once(',')?;
        if !integer(p) || !integer(q) || q.starts_with('-') {
            return None;
        }
        let denominator = Nat::parse(q);
        if denominator < Nat::small(2) {
            return None;
        }
        let r = Self::fraction(p, q);
        (r.term() == term).then_some(r)
    }
}
impl Ord for Rat {
    fn cmp(&self, b: &Self) -> Ordering {
        if self.negative != b.negative {
            return b.negative.cmp(&self.negative);
        }
        let order = self.n.mul(&b.d).cmp(&b.n.mul(&self.d));
        if self.negative {
            order.reverse()
        } else {
            order
        }
    }
}
impl PartialOrd for Rat {
    fn partial_cmp(&self, b: &Self) -> Option<Ordering> {
        Some(self.cmp(b))
    }
}

#[test]
fn oracle_binary_arithmetic_matches_machine_integer_controls() {
    for a in [
        0u128,
        1,
        2,
        3,
        7,
        31,
        32,
        33,
        65535,
        (1 << 32) - 1,
        1 << 32,
        (1 << 53) + 7,
    ] {
        for b in [1u128, 2, 3, 7, 31, 65537, (1 << 32) + 1] {
            let x = Nat::parse(&a.to_string());
            let y = Nat::parse(&b.to_string());
            assert_eq!(x.add(&y).decimal(), (a + b).to_string());
            assert_eq!(x.mul(&y).decimal(), (a * b).to_string());
            let (q, r) = x.div_rem(&y);
            assert_eq!(q.decimal(), (a / b).to_string());
            assert_eq!(r.decimal(), (a % b).to_string());
            if a >= b {
                assert_eq!(x.sub(&y).decimal(), (a - b).to_string());
            }
        }
    }
    for a in -13i128..=13 {
        for b in 1i128..=11 {
            for c in -7i128..=7 {
                let x = Rat::new(a, b);
                let y = Rat::new(c, 7);
                assert_eq!(x.mul(&y), Rat::new(a * c, b * 7));
                assert_eq!(x.add(&y), Rat::new(a * 7 + b * c, b * 7));
                assert_eq!(x.cmp(&y), (a * 7).cmp(&(b * c)));
                assert_eq!(Rat::decode(&x.term()), Some(x));
            }
        }
    }
    let a = format!("1{}7", "0".repeat(127));
    let b = "9".repeat(133);
    let x = Rat::fraction(&a, &b);
    assert_eq!(x.mul(&x.reciprocal()), Rat::integer(1));
    assert_eq!(x.add(&x.neg()), Rat::integer(0));
}

#[test]
fn oracle_large_limbs_follow_decimal_identities_not_self_cancellation_alone() {
    for n in [100, 127, 256] {
        let below = "9".repeat(n);
        let above = format!("1{}1", "0".repeat(n - 1));
        let a = Nat::parse(&below);
        let b = Nat::parse(&above);
        assert_eq!(
            a.mul(&a).decimal(),
            format!("{}8{}1", "9".repeat(n - 1), "0".repeat(n - 1))
        );
        assert_eq!(a.mul(&b).decimal(), "9".repeat(2 * n));
        let (q, r) = Nat::parse(&format!("1{}7", "0".repeat(2 * n - 1)))
            .div_rem(&Nat::parse(&format!("1{}3", "0".repeat(n - 1))));
        assert_eq!(q.decimal(), format!("{}7", "9".repeat(n - 1)));
        assert_eq!(r.decimal(), "16");
        assert_eq!(Nat::gcd(a.mul(&Nat::small(3)), a.mul(&Nat::small(7))), a);
        let sum = Rat::fraction("1", &below).add(&Rat::fraction("1", &above));
        assert_eq!(
            sum.term(),
            format!("rational(2{},{})", "0".repeat(n), "9".repeat(2 * n))
        );
    }
}
