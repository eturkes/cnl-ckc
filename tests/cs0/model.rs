use super::common::{node, proved};

pub const LIMIT: &str = "indeterminate(limit)";
pub const NO: &str = "no(finite_failure)";

pub fn allow(name: &str) -> String {
    format!("guideline_event(actual,{name},allow)")
}
pub fn inner(name: &str) -> String {
    format!("guideline_event(actual,{name},blocked)")
}
pub fn chain(prefix: &str, links: usize, terminal: bool) -> Vec<String> {
    let mut clauses: Vec<_> = (0..links)
        .map(|i| {
            format!(
                "{} :- {}",
                inner(&format!("{prefix}{i}")),
                inner(&format!("{prefix}{}", i + 1))
            )
        })
        .collect();
    if terminal {
        clauses.push(inner(&format!("{prefix}{links}")));
    }
    clauses
}
fn negated(body: &str) -> String {
    format!("{} :- \\+ {body}", allow("candidate"))
}
fn naf_proof(rule: &str, goal: &str) -> String {
    proved(rule, &[format!("naf({goal})")])
}

pub struct Scenario {
    pub label: String,
    pub clauses: Vec<String>,
    pub wh: bool,
    pub result: String,
    pub trace_result: String,
}
impl Scenario {
    fn new(label: String, clauses: Vec<String>, result: &str, trace_result: String) -> Self {
        Self {
            label,
            clauses,
            wh: false,
            result: result.into(),
            trace_result,
        }
    }
    pub fn goal(&self) -> String {
        allow(if self.wh { "A" } else { "candidate" })
    }
    fn wh(mut self, names: &[&str]) -> Self {
        self.wh = true;
        if self.result != LIMIT {
            let mut names = names.to_vec();
            names.sort_unstable();
            names.dedup();
            self.result = format!(
                "solutions([{}])",
                names
                    .iter()
                    .map(|n| format!("sol([{n}])"))
                    .collect::<Vec<_>>()
                    .join(",")
            );
            self.trace_result = format!(
                "solutions([{}])",
                names
                    .iter()
                    .map(|n| format!("sol([{n}],proved([{}]))", node(&allow(n), &[])))
                    .collect::<Vec<_>>()
                    .join(",")
            );
        }
        self
    }
}

pub fn t1() -> Vec<Scenario> {
    let mut cases = Vec::new();
    // The enclosing rule + NAF consume two of the answer's 100 depth units.
    for links in [98, 99, 100, 105] {
        for terminal in [false, true] {
            for fallback in [false, true] {
                for grouped in [false, true] {
                    let goal = if grouped {
                        format!("({}, {})", inner("n0"), inner("absent"))
                    } else {
                        inner("n0")
                    };
                    let mut clauses = vec![negated(&goal)];
                    clauses.extend(chain("n", links, terminal));
                    if fallback {
                        clauses.push(allow("candidate"));
                    }
                    cases.push(Scenario::new(
                        format!("links={links} terminal={terminal} outer-fallback={fallback} grouped={grouped}"),
                        clauses, LIMIT, LIMIT.into(),
                    ));
                }
            }
        }
    }
    for links in [98, 105] {
        let mut clauses = vec![negated(&format!("({}, {})", inner("n0"), inner("absent")))];
        clauses.extend(chain("n", links, false));
        clauses.push(inner("n0"));
        cases.push(Scenario::new(
            format!("links={links} inner-subgoal-proved-but-conjunction-fails"),
            clauses,
            LIMIT,
            LIMIT.into(),
        ));
    }
    cases
}
pub fn t2() -> Vec<Scenario> {
    let mut cases = Vec::new();
    for links in [0, 1, 2, 96, 97] {
        for terminal in [false, true] {
            let rule = negated(&inner("n0"));
            let mut clauses = vec![rule.clone()];
            clauses.extend(chain("n", links, terminal));
            cases.push(Scenario::new(
                format!("links={links} terminal={terminal}"),
                clauses,
                if terminal { NO } else { "yes" },
                if terminal {
                    NO.into()
                } else {
                    naf_proof(&rule, &inner("n0"))
                },
            ));
        }
    }
    let rule = format!(
        "{} :- \\+ {}, \\+ {}",
        allow("candidate"),
        inner("absent1"),
        inner("absent2")
    );
    cases.push(Scenario::new(
        "two-clean-negations".into(),
        vec![rule.clone()],
        "yes",
        proved(
            &rule,
            &[
                format!("naf({})", inner("absent1")),
                format!("naf({})", inner("absent2")),
            ],
        ),
    ));
    let rule = format!(
        "{} :- \\+ {}, {}",
        allow("candidate"),
        inner("absent1"),
        inner("absent2")
    );
    cases.push(Scenario::new(
        "clean-negation-failed-continuation".into(),
        vec![rule],
        NO,
        NO.into(),
    ));
    cases
}
pub fn t3() -> Vec<Scenario> {
    let mut cases = Vec::new();
    for links in [98, 105] {
        for terminal in [false, true] {
            for first in [false, true] {
                let mut clauses = vec![negated(&inner("n0"))];
                if first {
                    clauses.push(inner("n0"));
                }
                clauses.extend(chain("n", links, terminal));
                if !first {
                    clauses.push(inner("n0"));
                }
                cases.push(Scenario::new(
                    format!("links={links} terminal={terminal} proof-before-cut={first}"),
                    clauses,
                    NO,
                    NO.into(),
                ));
            }
        }
    }
    for links in [98, 105] {
        let mut clauses = vec![negated(&format!("({}, {})", inner("n0"), inner("token")))];
        clauses.extend(chain("n", links, false));
        clauses.extend([inner("n0"), inner("token")]);
        cases.push(Scenario::new(
            format!("links={links} entire-inner-conjunction-proved"),
            clauses,
            NO,
            NO.into(),
        ));
    }
    cases
}
pub fn t4() -> Vec<Scenario> {
    let mut cases = Vec::new();
    for mut case in t3() {
        case.label.push_str(" empty");
        cases.push(case.wh(&[]));
    }
    for before in [false, true] {
        for after in [false, true] {
            for terminal in [false, true] {
                let mut clauses = Vec::new();
                let mut names = Vec::new();
                if before {
                    clauses.push(allow("alpha"));
                    names.push("alpha");
                }
                clauses.push(negated(&inner("n0")));
                clauses.extend(chain("n", 105, terminal));
                clauses.push(inner("n0"));
                if after {
                    clauses.extend([
                        allow("omega"),
                        format!("{} :- {}", allow("omega"), inner("token")),
                        inner("token"),
                    ]);
                    names.push("omega");
                }
                cases.push(
                    Scenario::new(
                        format!("before={before} after={after} terminal={terminal}"),
                        clauses,
                        NO,
                        NO.into(),
                    )
                    .wh(&names),
                );
            }
        }
    }
    let rule = negated(&inner("w0"));
    let mut clauses = vec![
        rule.clone(),
        format!("{} :- \\+ {}", inner("w0"), inner("n0")),
    ];
    clauses.extend(chain("n", 105, false));
    clauses.push(inner("n0"));
    cases.push(Scenario {
        label: "nested-absorption-proves-outer-negation".into(),
        clauses,
        wh: true,
        result: "solutions([sol([candidate])])".into(),
        trace_result: format!(
            "solutions([sol([candidate],proved([{}]))])",
            node(&rule, &[format!("naf({})", inner("w0"))])
        ),
    });
    cases
}
pub fn t5() -> Vec<Scenario> {
    let mut cases = Vec::new();
    for nesting in [2, 3, 4] {
        for terminal in [false, true] {
            for fallback in [false, true] {
                let mut clauses = vec![negated(&inner("w0"))];
                for i in 0..nesting - 1 {
                    let last = i == nesting - 2;
                    let goal = if last {
                        inner("n0")
                    } else {
                        inner(&format!("w{}", i + 1))
                    };
                    let tail = if last && nesting % 2 == 0 {
                        format!(", {}", inner("absent"))
                    } else {
                        String::new()
                    };
                    clauses.push(format!("{} :- \\+ {goal}{tail}", inner(&format!("w{i}"))));
                }
                clauses.extend(chain("n", 105, terminal));
                if fallback {
                    clauses.push(allow("candidate"));
                }
                cases.push(Scenario::new(
                    format!("nesting={nesting} terminal={terminal} outer-fallback={fallback}"),
                    clauses,
                    LIMIT,
                    LIMIT.into(),
                ));
            }
        }
    }
    cases
}
pub fn t6() -> Vec<Scenario> {
    let mut cases = Vec::new();
    for links in [98, 105] {
        for terminal in [false, true] {
            for before in [false, true] {
                for after in [false, true] {
                    let mut clauses = Vec::new();
                    if before {
                        clauses.push(allow("alpha"));
                    }
                    clauses.push(negated(&inner("n0")));
                    clauses.extend(chain("n", links, terminal));
                    if after {
                        clauses.push(allow("omega"));
                    }
                    cases.push(
                        Scenario::new(
                            format!(
                                "links={links} terminal={terminal} before={before} after={after}"
                            ),
                            clauses,
                            LIMIT,
                            LIMIT.into(),
                        )
                        .wh(&[]),
                    );
                }
            }
        }
    }
    cases
}
pub fn t7() -> Vec<Scenario> {
    let mut cases = Vec::new();
    for continuation in ["fact", "clean-naf", "refuted-naf", "failed-continuation"] {
        let mut clauses = vec![format!("{} :- {}", allow("candidate"), inner("prior0"))];
        clauses.extend(chain("prior", 105, false));
        let (result, trace_result) = match continuation {
            "fact" => {
                let fact = allow("candidate");
                clauses.push(fact.clone());
                ("yes", proved(&fact, &[]))
            }
            "clean-naf" => {
                let rule = negated(&inner("absent"));
                clauses.push(rule.clone());
                ("yes", naf_proof(&rule, &inner("absent")))
            }
            "refuted-naf" => {
                clauses.push(negated(&inner("n0")));
                clauses.extend(chain("n", 105, false));
                clauses.push(inner("n0"));
                (LIMIT, LIMIT.into())
            }
            "failed-continuation" => {
                clauses.push(format!(
                    "{} :- \\+ {}, {}",
                    allow("candidate"),
                    inner("absent1"),
                    inner("absent2")
                ));
                (LIMIT, LIMIT.into())
            }
            _ => unreachable!(),
        };
        cases.push(Scenario::new(
            continuation.into(),
            clauses.clone(),
            result,
            trace_result,
        ));
        cases.push(
            Scenario::new(format!("{continuation}-wh"), clauses, LIMIT, LIMIT.into()).wh(&[]),
        );
    }
    cases
}
pub fn t8() -> Vec<Scenario> {
    let mut cases = Vec::new();
    // Two clauses per level enumerate a finite binary tree; depth stays below 20.
    for levels in [13, 14, 15, 16] {
        for proof_inside in [false, true] {
            let rule = negated(&inner("n0"));
            let mut clauses = vec![rule.clone()];
            for clause in chain("n", levels, false) {
                clauses.extend([clause.clone(), clause]);
            }
            if proof_inside {
                clauses.push(inner("n0"));
            }
            let result = if levels >= 15 {
                LIMIT
            } else if proof_inside {
                NO
            } else {
                "yes"
            };
            let trace_result = if result == "yes" {
                naf_proof(&rule, &inner("n0"))
            } else {
                result.into()
            };
            cases.push(Scenario::new(
                format!("levels={levels} inner-fallback={proof_inside}"),
                clauses,
                result,
                trace_result,
            ));
        }
    }
    cases
}

pub fn all() -> Vec<Scenario> {
    let mut result = Vec::new();
    for (row, cases) in [t1(), t2(), t3(), t4(), t5(), t6(), t7(), t8()]
        .into_iter()
        .enumerate()
    {
        for mut case in cases {
            case.label = format!("T{} {}", row + 1, case.label);
            result.push(case);
        }
    }
    result
}
