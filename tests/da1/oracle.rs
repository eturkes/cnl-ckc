// D1–D6 reference: endpoint hulls, not the implementation's kind/sign case table.
use super::rational::Rat;
use std::cmp::Ordering;

pub const KINDS: [&str; 6] = ["exactly", "eq", "geq", "greater", "leq", "less"];
pub const BOUNDS: [&str; 8] = [
    "exactly", "geq", "greater", "leq", "less", "eq", "about", "na",
];
pub const UNITS: [(&str, i128, u8); 7] = [
    ("second", 1, 0),
    ("minute", 60, 0),
    ("hour", 3600, 0),
    ("day", 86400, 0),
    ("week", 604800, 0),
    ("month", 1, 1),
    ("year", 12, 1),
];
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum End {
    NegInf,
    Finite(Rat),
    PosInf,
}
#[derive(Clone, Debug)]
struct Edge {
    value: End,
    closed: bool,
}
#[derive(Clone, Debug)]
pub struct Interval {
    low: Edge,
    high: Edge,
}
impl Interval {
    pub fn new(kind: &str, n: Rat) -> Option<Self> {
        let finite = |closed| Edge {
            value: End::Finite(n.clone()),
            closed,
        };
        let low = Edge {
            value: End::NegInf,
            closed: false,
        };
        let high = Edge {
            value: End::PosInf,
            closed: false,
        };
        Some(match kind {
            "exactly" => Self {
                low: finite(true),
                high: finite(true),
            },
            "eq" | "geq" => Self {
                low: finite(true),
                high,
            },
            "greater" => Self {
                low: finite(false),
                high,
            },
            "leq" => Self {
                low,
                high: finite(true),
            },
            "less" => Self {
                low,
                high: finite(false),
            },
            _ => return None,
        })
    }
    fn contains(&self, n: &Rat) -> bool {
        let n = End::Finite(n.clone());
        (n > self.low.value || (n == self.low.value && self.low.closed))
            && (n < self.high.value || (n == self.high.value && self.high.closed))
    }
    fn subset(&self, b: &Self) -> bool {
        (self.low.value > b.low.value
            || (self.low.value == b.low.value && (!self.low.closed || b.low.closed)))
            && (self.high.value < b.high.value
                || (self.high.value == b.high.value && (!self.high.closed || b.high.closed)))
    }
    fn spell(&self) -> Option<(String, String)> {
        match (&self.low.value, &self.high.value) {
            (End::Finite(a), End::Finite(b)) if a == b && self.low.closed && self.high.closed => {
                Some(("exactly".into(), a.term()))
            }
            (End::Finite(a), End::PosInf) => Some((
                if self.low.closed { "geq" } else { "greater" }.into(),
                a.term(),
            )),
            (End::NegInf, End::Finite(a)) => Some((
                if self.high.closed { "leq" } else { "less" }.into(),
                a.term(),
            )),
            _ => None,
        }
    }
    pub fn product(&self, b: &Self) -> Self {
        let zero_attained = self.contains(&Rat::integer(0)) || b.contains(&Rat::integer(0));
        let mut corners = Vec::new();
        for a in [&self.low, &self.high] {
            for b in [&b.low, &b.high] {
                let sign = |e: &End| match e {
                    End::NegInf => -1,
                    End::PosInf => 1,
                    End::Finite(r) => r.sign(),
                };
                let value = match (&a.value, &b.value) {
                    (End::Finite(x), End::Finite(y)) => End::Finite(x.mul(y)),
                    _ if sign(&a.value) * sign(&b.value) == 0 => End::Finite(Rat::integer(0)),
                    _ if sign(&a.value) * sign(&b.value) < 0 => End::NegInf,
                    _ => End::PosInf,
                };
                let closed = match &value {
                    End::Finite(r) => (r.zero() && zero_attained) || (a.closed && b.closed),
                    _ => false,
                };
                corners.push(Edge { value, closed });
            }
        }
        let extreme = |order: Ordering| {
            let value = corners
                .iter()
                .map(|e| &e.value)
                .reduce(|a, b| if a.cmp(b) == order { a } else { b })
                .unwrap()
                .clone();
            let closed = corners.iter().any(|e| e.value == value && e.closed);
            Edge { value, closed }
        };
        Self {
            low: extreme(Ordering::Less),
            high: extreme(Ordering::Greater),
        }
    }
}

pub fn arithmetic(op: &str, args: &[String]) -> Option<(String, String)> {
    let [s, n, t, m] = args else {
        return None;
    };
    let n = Rat::decode(n)?;
    let m = Rat::decode(m)?;
    let a = Interval::new(s, n)?;
    let b = if op == "guideline_quotient" {
        if t != "exactly" || m.zero() {
            return None;
        }
        Interval::new("exactly", m.reciprocal())?
    } else {
        assert_eq!(op, "guideline_product");
        Interval::new(t, m)?
    };
    a.product(&b).spell()
}
pub fn comparison(args: &[String]) -> bool {
    let (s, n, t, m, units) = match args {
        [s, n, t, m] => (s, n, t, m, None),
        [s, n, u, t, m, v] => (s, n, t, m, Some((u, v))),
        _ => return false,
    };
    if !KINDS.contains(&t.as_str()) || t == "eq" {
        return false;
    }
    let (Some(mut n), Some(mut m)) = (Rat::decode(n), Rat::decode(m)) else {
        return false;
    };
    if let Some((u, v)) = units {
        let a = UNITS.iter().find(|(name, _, _)| name == u);
        let b = UNITS.iter().find(|(name, _, _)| name == v);
        let (Some((_, r, g)), Some((_, q, h))) = (a, b) else {
            return false;
        };
        if g != h {
            return false;
        }
        n = n.mul(&Rat::integer(*r));
        m = m.mul(&Rat::integer(*q));
    }
    match (Interval::new(s, n), Interval::new(t, m)) {
        (Some(a), Some(b)) => a.subset(&b),
        _ => false,
    }
}
#[derive(Clone, Debug)]
pub struct Probe {
    pub name: String,
    pub args: Vec<String>,
    pub output: Option<(String, String)>,
    pub truth: bool,
}
impl Probe {
    pub fn arithmetic(name: &str, s: &str, n: &str, t: &str, m: &str) -> Self {
        let args = [s, n, t, m].map(String::from).to_vec();
        let output = arithmetic(name, &args);
        Self {
            name: name.into(),
            args,
            truth: output.is_some(),
            output,
        }
    }
    pub fn comparison(args: &[&str]) -> Self {
        let args = args.iter().map(|s| (*s).into()).collect::<Vec<_>>();
        let truth = comparison(&args);
        Self {
            name: "guideline_satisfies".into(),
            args,
            output: None,
            truth,
        }
    }
    pub fn goal(&self) -> String {
        if self.name == "guideline_satisfies" {
            format!("{}({})", self.name, self.args.join(","))
        } else {
            format!("{}({},A,B)", self.name, self.args.join(","))
        }
    }
    pub fn bound_goal(&self, s: &str, n: &str) -> String {
        assert_ne!(self.name, "guideline_satisfies");
        format!("{}({},{s},{n})", self.name, self.args.join(","))
    }
}
pub fn arithmetic_grid(name: &str) -> Vec<Probe> {
    let values = [
        "-3",
        "rational(-7,3)",
        "rational(-1,2)",
        "0",
        "rational(1,2)",
        "rational(7,3)",
        "3",
    ];
    let mut out = Vec::new();
    for s in KINDS {
        for t in KINDS {
            for n in values {
                for m in values {
                    out.push(Probe::arithmetic(name, s, n, t, m));
                }
            }
        }
    }
    out
}
pub fn comparison_grid(integer_only: bool, with_units: bool) -> Vec<Probe> {
    let mut out = Vec::new();
    let endpoints = if integer_only {
        vec![Rat::integer(-1), Rat::integer(0), Rat::integer(1)]
    } else {
        [(1, 2), (-1, 3), (13, 7), (1234567, 1000000)]
            .iter()
            .map(|&(p, q)| Rat::new(p, q))
            .collect()
    };
    let units = if with_units {
        UNITS
            .iter()
            .flat_map(|&(u, r, _)| UNITS.iter().map(move |&(v, q, _)| Some((u, r, v, q))))
            .collect()
    } else {
        vec![None]
    };
    for unit in units {
        let (r, q) = unit.map_or((1, 1), |(_, r, _, q)| (r, q));
        for y in &endpoints {
            for delta in [-1, 0, 1] {
                let x = y
                    .mul(&Rat::integer(q))
                    .add(&Rat::new(delta, if integer_only { 1 } else { 10000000 }));
                let y = y.mul(&Rat::integer(r));
                for s in KINDS {
                    for b in BOUNDS {
                        let p = match unit {
                            None => Probe::comparison(&[s, &x.term(), b, &y.term()]),
                            Some((u, _, v, _)) => {
                                Probe::comparison(&[s, &x.term(), u, b, &y.term(), v])
                            }
                        };
                        out.push(p);
                    }
                }
            }
        }
    }
    out
}
pub const BAD_NUMBERS: [&str; 17] = [
    "rational(2,4)",
    "rational(0,2)",
    "rational(3,1)",
    "rational(1,0)",
    "rational(1,-2)",
    "rational(a,2)",
    "rational(1,A)",
    "rational(A,2)",
    "rational(1,2,3)",
    "rational(1)",
    "rational",
    "number",
    "A",
    "[]",
    "[1]",
    "value(1)",
    "rational(1,rational(1,2))",
];
pub fn invalid_grid(arithmetic: bool) -> Vec<Probe> {
    let mut out = Vec::new();
    if arithmetic {
        for name in ["guideline_product", "guideline_quotient"] {
            for slot in 0..4 {
                let bad: &[&str] = if slot % 2 == 1 {
                    &BAD_NUMBERS
                } else {
                    &[
                        "A",
                        "about",
                        "na",
                        "not_a_comparison",
                        "0",
                        "[]",
                        "[exactly]",
                        "f(exactly)",
                    ]
                };
                for &bad in bad {
                    for (n, m) in [("rational(1,2)", "3"), ("0", "3"), ("2", "0")] {
                        let mut args = ["exactly", n, "exactly", m];
                        args[slot] = bad;
                        out.push(Probe::arithmetic(name, args[0], args[1], args[2], args[3]));
                    }
                }
            }
        }
    } else {
        for n in [4, 6] {
            let base = if n == 4 {
                vec!["exactly", "rational(1,2)", "geq", "0"]
            } else {
                vec!["exactly", "rational(1,2)", "day", "geq", "0", "hour"]
            };
            for slot in 0..n {
                let numbers = slot == 1 || slot == if n == 4 { 3 } else { 4 };
                let bad: &[&str] = if numbers {
                    &BAD_NUMBERS
                } else {
                    &["A", "bogus", "0", "[]", "[day]", "f(day)"]
                };
                for &bad in bad {
                    let mut args = base.clone();
                    args[slot] = bad;
                    out.push(Probe::comparison(&args));
                }
            }
        }
    }
    assert!(out.iter().all(|p| !p.truth));
    out
}

#[test]
fn oracle_interval_hulls_have_contract_properties_and_plants() {
    let grid = arithmetic_grid("guideline_product");
    let mut successful = 0;
    for p in &grid {
        let reverse = Probe::arithmetic(&p.name, &p.args[2], &p.args[3], &p.args[0], &p.args[1]);
        assert_eq!(
            p.output,
            reverse.output,
            "product commutativity: {}",
            p.goal()
        );
        if let Some((kind, value)) = &p.output {
            successful += 1;
            assert_ne!(kind, "eq");
            assert_eq!(Rat::decode(value).unwrap().term(), *value);
            let image = Interval::new(kind, Rat::decode(value).unwrap()).unwrap();
            let a = Interval::new(&p.args[0], Rat::decode(&p.args[1]).unwrap()).unwrap();
            let b = Interval::new(&p.args[2], Rat::decode(&p.args[3]).unwrap()).unwrap();
            for x in -8..=8 {
                for y in -8..=8 {
                    let x = Rat::new(x, 2);
                    let y = Rat::new(y, 2);
                    if a.contains(&x) && b.contains(&y) {
                        assert!(image.contains(&x.mul(&y)), "{}", p.goal());
                    }
                }
            }
        } else {
            let a = Interval::new(&p.args[0], Rat::decode(&p.args[1]).unwrap()).unwrap();
            let b = Interval::new(&p.args[2], Rat::decode(&p.args[3]).unwrap()).unwrap();
            let image = a.product(&b);
            assert_eq!(image.low.value, End::NegInf);
            assert_eq!(image.high.value, End::PosInf);
        }
    }
    assert!(successful > 500);
    for (s, n, t, m, want) in [
        ("geq", "-1", "geq", "1", None),
        ("greater", "0", "geq", "0", Some(("geq", "0"))),
        ("greater", "0", "greater", "0", Some(("greater", "0"))),
        ("less", "0", "leq", "0", Some(("geq", "0"))),
        (
            "exactly",
            "-3",
            "geq",
            "rational(1,2)",
            Some(("leq", "rational(-3,2)")),
        ),
        (
            "exactly",
            "rational(2,3)",
            "exactly",
            "3",
            Some(("exactly", "2")),
        ),
    ] {
        let got = Probe::arithmetic("guideline_product", s, n, t, m).output;
        assert_eq!(got, want.map(|(s, n)| (s.into(), n.into())));
    }
    // Public witnesses for P10's omitted sign guard and wrong-normalization plants.
    assert!(
        Probe::arithmetic("guideline_product", "geq", "-1", "geq", "1")
            .output
            .is_none()
    );
    assert_eq!(Rat::new(-2, -4).term(), "rational(1,2)");
    assert_eq!(Rat::new(0, 7).term(), "0");
}
