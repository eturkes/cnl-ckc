// Independent D1 model: intervals with open/closed endpoints, then containment.
// Decimal arithmetic only multiplies by a positive unit scale; no production code.
use std::cmp::Ordering;
use std::fmt;

pub const STATED: [&str; 6] = ["eq", "exactly", "geq", "greater", "leq", "less"];
pub const BOUNDS: [&str; 5] = ["exactly", "geq", "greater", "leq", "less"];
pub const UNITS: [(&str, u32, u8); 7] = [
    ("second", 1, 0),
    ("minute", 60, 0),
    ("hour", 3600, 0),
    ("day", 86400, 0),
    ("week", 604800, 0),
    ("month", 1, 1),
    ("year", 12, 1),
];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Integer {
    negative: bool,
    digits: Vec<u8>,
}
impl Integer {
    pub fn parse(text: &str) -> Option<Self> {
        let negative = text.starts_with('-');
        let text = text.strip_prefix('-').unwrap_or(text);
        if text.is_empty() || !text.bytes().all(|b| b.is_ascii_digit()) {
            return None;
        }
        let text = text.trim_start_matches('0');
        let digits = if text.is_empty() {
            vec![0]
        } else {
            text.bytes().rev().map(|b| b - b'0').collect()
        };
        Some(Self {
            negative: negative && digits != [0],
            digits,
        })
    }
    pub fn mul(&self, factor: u32) -> Self {
        let mut digits = Vec::new();
        let mut carry = 0u64;
        for &digit in &self.digits {
            let n = u64::from(digit) * u64::from(factor) + carry;
            digits.push((n % 10) as u8);
            carry = n / 10;
        }
        while carry != 0 {
            digits.push((carry % 10) as u8);
            carry /= 10;
        }
        while digits.len() > 1 && digits.last() == Some(&0) {
            digits.pop();
        }
        Self {
            negative: self.negative && digits != [0],
            digits,
        }
    }
    pub fn offset(&self, delta: i8) -> Self {
        assert!((-1..=1).contains(&delta));
        if delta == 0 {
            return self.clone();
        }
        if self.digits == [0] {
            return Self::parse(&delta.to_string()).unwrap();
        }
        let mut out = self.clone();
        if (delta < 0) == self.negative {
            let mut i = 0;
            while i < out.digits.len() && out.digits[i] == 9 {
                out.digits[i] = 0;
                i += 1;
            }
            if i == out.digits.len() {
                out.digits.push(1);
            } else {
                out.digits[i] += 1;
            }
        } else {
            let mut i = 0;
            while out.digits[i] == 0 {
                out.digits[i] = 9;
                i += 1;
            }
            out.digits[i] -= 1;
            while out.digits.len() > 1 && out.digits.last() == Some(&0) {
                out.digits.pop();
            }
            if out.digits == [0] {
                out.negative = false;
            }
        }
        out
    }
}
impl Ord for Integer {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.negative != other.negative {
            return other.negative.cmp(&self.negative);
        }
        let absolute = self
            .digits
            .len()
            .cmp(&other.digits.len())
            .then_with(|| self.digits.iter().rev().cmp(other.digits.iter().rev()));
        if self.negative {
            absolute.reverse()
        } else {
            absolute
        }
    }
}
impl PartialOrd for Integer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.negative {
            write!(f, "-")?;
        }
        for digit in self.digits.iter().rev() {
            write!(f, "{digit}")?;
        }
        Ok(())
    }
}

struct Interval {
    lower: Option<(Integer, bool)>,
    upper: Option<(Integer, bool)>,
}
impl Interval {
    fn new(kind: &str, x: Integer) -> Option<Self> {
        let (lower, upper) = match kind {
            "eq" | "geq" => (Some((x, true)), None),
            "greater" => (Some((x, false)), None),
            "leq" => (None, Some((x, true))),
            "less" => (None, Some((x, false))),
            "exactly" => (Some((x.clone(), true)), Some((x, true))),
            _ => return None,
        };
        Some(Self { lower, upper })
    }
    fn subset_of(&self, outer: &Self) -> bool {
        let lower = match (&self.lower, &outer.lower) {
            (_, None) => true,
            (None, Some(_)) => false,
            (Some((x, closed)), Some((y, outer_closed))) => {
                x > y || (x == y && (!closed || *outer_closed))
            }
        };
        let upper = match (&self.upper, &outer.upper) {
            (_, None) => true,
            (None, Some(_)) => false,
            (Some((x, closed)), Some((y, outer_closed))) => {
                x < y || (x == y && (!closed || *outer_closed))
            }
        };
        lower && upper
    }
}
pub fn within(stated: &str, x: &Integer, bound: &str, y: &Integer) -> bool {
    if !BOUNDS.contains(&bound) {
        return false;
    }
    match (
        Interval::new(stated, x.clone()),
        Interval::new(bound, y.clone()),
    ) {
        (Some(inner), Some(outer)) => inner.subset_of(&outer),
        _ => false,
    }
}
pub fn holds(args: &[String]) -> bool {
    let (s, n, b, m, units) = match args {
        [s, n, b, m] => (s, n, b, m, None),
        [s, n, u, b, m, v] => (s, n, b, m, Some((u, v))),
        _ => return false,
    };
    let (Some(mut n), Some(mut m)) = (Integer::parse(n), Integer::parse(m)) else {
        return false;
    };
    if let Some((u, v)) = units {
        let first = UNITS.iter().find(|(name, _, _)| name == u);
        let second = UNITS.iter().find(|(name, _, _)| name == v);
        let (Some((_, r, g)), Some((_, q, h))) = (first, second) else {
            return false;
        };
        if g != h {
            return false;
        }
        n = n.mul(*r);
        m = m.mul(*q);
    }
    within(s, &n, b, &m)
}
#[derive(Clone)]
pub struct Probe {
    pub args: Vec<String>,
}
impl Probe {
    pub fn new(args: &[&str]) -> Self {
        Self {
            args: args.iter().map(|s| (*s).into()).collect(),
        }
    }
    pub fn goal(&self) -> String {
        format!("guideline_satisfies({})", self.args.join(","))
    }
    pub fn expected(&self) -> bool {
        holds(&self.args)
    }
}
fn boundary_grid(units: Option<(&str, u32, &str, u32)>, endpoints: &[String]) -> Vec<Probe> {
    let mut out = Vec::new();
    for endpoint in endpoints {
        let y = Integer::parse(endpoint).unwrap();
        let (rx, ry) = units.map_or((1, 1), |(_, r, _, q)| (r, q));
        for delta in [-1, 0, 1] {
            let x = y.mul(ry).offset(delta).to_string();
            let y = y.mul(rx).to_string();
            for s in STATED {
                for b in BOUNDS {
                    out.push(match units {
                        None => Probe::new(&[s, &x, b, &y]),
                        Some((u, _, v, _)) => Probe::new(&[s, &x, u, b, &y, v]),
                    });
                }
            }
        }
    }
    out
}
pub fn unitless_grid() -> Vec<Probe> {
    boundary_grid(None, &["-1".into(), "0".into(), "1".into()])
}
pub fn unit_grid() -> Vec<Probe> {
    let mut out = Vec::new();
    for (u, r, _) in UNITS {
        for (v, q, _) in UNITS {
            out.extend(boundary_grid(
                Some((u, r, v, q)),
                &["-1".into(), "0".into(), "1".into()],
            ));
        }
    }
    out
}
pub fn huge_grid() -> Vec<Probe> {
    let huge = format!("1{}", "0".repeat(99));
    let nines = "9".repeat(100);
    let mut out = boundary_grid(
        None,
        &[
            huge.clone(),
            format!("-{huge}"),
            nines.clone(),
            format!("-{nines}"),
        ],
    );
    for (u, r, _) in UNITS {
        for (v, q, _) in UNITS {
            out.extend(boundary_grid(
                Some((u, r, v, q)),
                &[huge.clone(), format!("-{huge}")],
            ));
        }
    }
    out
}
pub fn invalid_grid() -> Vec<Probe> {
    let mut out = Vec::new();
    for args in [
        vec!["exactly", "60", "geq", "50"],
        vec!["exactly", "60", "day", "geq", "50", "day"],
    ] {
        for slot in 0..args.len() {
            for bad in ["A", "[]", "value(x)", "[x]", "na", "about", "'two words'"] {
                let mut p = Probe::new(&args);
                p.args[slot] = bad.into();
                assert!(!p.expected());
                out.push(p);
            }
        }
        let count_slots: &[usize] = if args.len() == 4 { &[1, 3] } else { &[1, 4] };
        for &slot in count_slots {
            for bad in ["fifty", "'50'", "'1.5'", "n(50)"] {
                let mut p = Probe::new(&args);
                p.args[slot] = bad.into();
                out.push(p);
            }
        }
        let (s, b) = if args.len() == 4 { (0, 2) } else { (0, 3) };
        for (slot, values) in [
            (s, vec!["unknown", "true", "-1"]),
            (b, vec!["eq", "unknown", "true", "-1"]),
        ] {
            for bad in values {
                let mut p = Probe::new(&args);
                p.args[slot] = bad.into();
                out.push(p);
            }
        }
        if args.len() == 6 {
            for slot in [2, 5] {
                for bad in ["fortnight", "days", "millisecond", "mg", "0"] {
                    let mut p = Probe::new(&args);
                    p.args[slot] = bad.into();
                    out.push(p);
                }
            }
        }
    }
    out.extend([
        Probe::new(&["A", "A", "geq", "50"]),
        Probe::new(&["A", "A", "A", "A"]),
        Probe::new(&["A", "A", "A", "A", "A", "A"]),
        Probe::new(&["exactly", "n(A)", "geq", "50"]),
        Probe::new(&["exactly", "60", "unit(A)", "geq", "50", "day"]),
    ]);
    out
}
pub struct Random(u64);
impl Random {
    pub fn new() -> Self {
        Self(0x6e74_315f_6f72_6163)
    }
    pub fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    pub fn integer(&mut self) -> Integer {
        let len = 1 + (self.next() % 100) as usize;
        let sign = if self.next().is_multiple_of(2) {
            ""
        } else {
            "-"
        };
        let first = 1 + self.next() % 9;
        let mut text = format!("{sign}{first}");
        for _ in 1..len {
            text.push(char::from(b'0' + (self.next() % 10) as u8));
        }
        Integer::parse(&text).unwrap()
    }
}
pub fn generated_grid() -> Vec<Probe> {
    let mut random = Random::new();
    let mut out = Vec::new();
    for i in 0..512 {
        let x = random.integer();
        let y = if i % 3 == 0 {
            x.offset((random.next() % 3) as i8 - 1)
        } else {
            random.integer()
        };
        let s = STATED[(random.next() % STATED.len() as u64) as usize];
        let b = BOUNDS[(random.next() % BOUNDS.len() as u64) as usize];
        let x = x.to_string();
        let y = y.to_string();
        if i % 2 == 0 {
            out.push(Probe::new(&[s, &x, b, &y]));
        } else {
            let u = UNITS[(random.next() % 7) as usize].0;
            let v = UNITS[(random.next() % 7) as usize].0;
            out.push(Probe::new(&[s, &x, u, b, &y, v]));
        }
    }
    out
}
pub fn property_grid() -> Vec<Probe> {
    let mut probes = Vec::new();
    let mut random = Random::new();
    let mut transitive = 0;
    for _ in 0..16 {
        let x = random.integer();
        let y = x.offset(1);
        let z = y.offset(1);
        for s in STATED {
            // Reflexivity; eq and geq denote the same half-line.
            let bound = if s == "eq" { "geq" } else { s };
            let p = Probe::new(&[s, &x.to_string(), bound, &x.to_string()]);
            assert!(p.expected());
            probes.push(p);
        }
        for b in BOUNDS {
            let eq = Probe::new(&["eq", &x.to_string(), b, &y.to_string()]);
            let geq = Probe::new(&["geq", &x.to_string(), b, &y.to_string()]);
            assert_eq!(eq.expected(), geq.expected());
            probes.extend([eq, geq]);
        }
        for a in BOUNDS {
            for b in BOUNDS {
                for d in BOUNDS {
                    let ab = Probe::new(&[a, &z.to_string(), b, &y.to_string()]);
                    let bc = Probe::new(&[b, &y.to_string(), d, &x.to_string()]);
                    if ab.expected() && bc.expected() {
                        let ac = Probe::new(&[a, &z.to_string(), d, &x.to_string()]);
                        assert!(ac.expected(), "interval containment is transitive");
                        transitive += 1;
                        probes.extend([ab, bc, ac]);
                    }
                }
            }
        }
        for (u, scale, _) in UNITS {
            for s in STATED {
                let b = BOUNDS[(random.next() % 5) as usize];
                let plain = Probe::new(&[s, &x.to_string(), b, &y.to_string()]);
                let same = Probe::new(&[s, &x.to_string(), u, b, &y.to_string(), u]);
                let scaled =
                    Probe::new(&[s, &x.mul(scale).to_string(), b, &y.mul(scale).to_string()]);
                assert_eq!(plain.expected(), same.expected());
                assert_eq!(plain.expected(), scaled.expected());
                probes.extend([plain, same, scaled]);
            }
        }
    }
    assert!(transitive >= 16, "nonvacuous transitivity census");
    probes
}

pub fn all_probes() -> Vec<Probe> {
    [
        unitless_grid(),
        unit_grid(),
        huge_grid(),
        invalid_grid(),
        generated_grid(),
        property_grid(),
    ]
    .concat()
}
pub fn reference_self_checks() {
    for x in -200i128..=200 {
        let n = Integer::parse(&x.to_string()).unwrap();
        for d in [-1, 0, 1] {
            assert_eq!(n.offset(d).to_string(), (x + i128::from(d)).to_string());
        }
        for (_, scale, _) in UNITS {
            assert_eq!(
                n.mul(scale).to_string(),
                (x * i128::from(scale)).to_string()
            );
        }
        for y in [-201, -1, 0, 1, 201] {
            assert_eq!(n.cmp(&Integer::parse(&y.to_string()).unwrap()), x.cmp(&y));
        }
    }
    for (args, want) in [
        (vec!["eq", "60", "geq", "50"], true),
        (vec!["eq", "60", "exactly", "60"], false),
        (vec!["greater", "49", "geq", "50"], false),
        (vec!["greater", "23", "hour", "geq", "1", "day"], false),
        (vec!["less", "0", "leq", "0"], true),
        (vec!["leq", "0", "less", "0"], false),
        (vec!["exactly", "18", "month", "geq", "1", "year"], true),
        (vec!["exactly", "45", "day", "geq", "1", "month"], false),
    ] {
        assert_eq!(Probe::new(&args).expected(), want, "{args:?}");
    }
}
