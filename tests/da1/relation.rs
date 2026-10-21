use super::common::*;
use super::oracle::*;
use super::rational::Rat;
use std::path::{Path, PathBuf};
use std::process::Command;

// Each chunk is one wh query, so every row is exercised even on a seed-red run.
// Distinct case ids preserve failure rows and avoid answer dedup hiding a verdict.
fn native_grid(row: &'static str, probes: &[Probe], modes: bool) {
    let mut c = Cases::new(row);
    let mut clauses = Vec::new();
    let mut expected = Vec::new();
    for (i, p) in probes.iter().enumerate() {
        let id = format!("c{i:06}");
        if p.name == "guideline_satisfies" {
            clauses.push(format!(
                "guideline_cardinality(actual,{id},na,exactly,1) :- {}",
                p.goal()
            ));
            if p.truth {
                expected.push(format!("sol([{id},exactly,1])"));
            }
        } else {
            clauses.push(format!(
                "guideline_cardinality(actual,{id},na,A,B) :- {}",
                p.goal()
            ));
            if let Some((s, n)) = &p.output {
                expected.push(format!("sol([{id},{s},{n}])"));
            }
        }
    }
    let mut total = 0;
    for (chunk, clauses) in clauses.chunks(128).enumerate() {
        install(&c, &[("grid", clauses.to_vec(), String::new())]);
        let q = query(
            "guideline_cardinality(actual,A,na,B,C)",
            "[answer(A,wh(what)),answer(B,wh(what)),answer(C,wh(what))]",
        );
        c.write("query.pl", &q);
        let lo = format!("sol([c{:06},", chunk * 128);
        let hi = format!("sol([c{:06},", (chunk + 1) * 128);
        let rows: Vec<_> = expected
            .iter()
            .filter(|s| s.as_str() >= lo.as_str() && s.as_str() < hi.as_str())
            .cloned()
            .collect();
        c.expect(
            &format!(
                "{} rows {}..{}; first {}",
                row,
                chunk * 128,
                chunk * 128 + clauses.len(),
                probes[chunk * 128].goal()
            ),
            &["v1", "answer", "m.tsv", "query.pl"],
            Out::ok(answers(&q, &format!("solutions([{}])", rows.join(",")))),
        );
        total += clauses.len();
    }
    assert_eq!(total, probes.len());
    if modes {
        let mut mode_goals = Vec::new();
        for p in probes {
            if let Some((s, n)) = &p.output {
                let n = Rat::decode(n).unwrap();
                for (s, n, truth) in [
                    (s.clone(), n.term(), true),
                    (s.clone(), n.add(&Rat::integer(1)).term(), false),
                    ("eq".into(), n.term(), false),
                    (
                        if s == "exactly" { "geq" } else { "exactly" }.into(),
                        n.term(),
                        false,
                    ),
                    (s.clone(), n.unreduced(), false),
                ] {
                    mode_goals.push((p.bound_goal(&s, &n), truth));
                }
            }
        }
        native_booleans(&mut c, "pre-bound outputs", &mode_goals);
        println!(
            "DA1 {row} oracle inputs={} output-mode rows={}",
            probes.len(),
            mode_goals.len()
        );
    } else {
        println!("DA1 {row} oracle inputs={}", probes.len());
    }
    c.finish();
}
fn native_booleans(c: &mut Cases, label: &str, rows: &[(String, bool)]) {
    for (chunk, rows) in rows.chunks(128).enumerate() {
        let clauses: Vec<_> = rows
            .iter()
            .enumerate()
            .map(|(i, (g, _))| format!("guideline_event(actual,c{i:03},pass) :- {g}"))
            .collect();
        install(c, &[("modes", clauses, String::new())]);
        let q = query("guideline_event(actual,A,pass)", "[answer(A,wh(what))]");
        let result = rows
            .iter()
            .enumerate()
            .filter(|(_, (_, truth))| *truth)
            .map(|(i, _)| format!("sol([c{i:03}])"))
            .collect::<Vec<_>>()
            .join(",");
        c.write("query.pl", &q);
        c.expect(
            &format!("{label} chunk={chunk} first={}", rows[0].0),
            &["v1", "answer", "m.tsv", "query.pl"],
            Out::ok(answers(&q, &format!("solutions([{result}])"))),
        );
    }
}
#[test]
fn p2_native_rational_comparison_boundary_grid() {
    native_grid("P2-rational", &comparison_grid(false, false), false);
}
#[test]
fn p2_native_all_ordered_units_and_groups() {
    native_grid("P2-units", &comparison_grid(false, true), false);
}
#[test]
fn p2_native_integer_grid_unchanged() {
    native_grid("P2-integers", &comparison_grid(true, false), false);
}
#[test]
fn p3_native_product_kind_sign_endpoint_grid_and_output_modes() {
    native_grid("P3-product", &arithmetic_grid("guideline_product"), true);
}
#[test]
fn p4_native_quotient_kind_sign_endpoint_grid_and_output_modes() {
    native_grid("P4-quotient", &arithmetic_grid("guideline_quotient"), true);
}

fn invalid_native(row: &'static str, probes: &[Probe]) {
    let mut c = Cases::new(row);
    c.write("m.tsv", "");
    for p in probes {
        c.answer(&p.goal(), &p.goal(), false);
    }
    c.finish();
}
#[test]
fn p1_native_every_invalid_number_slot_comparisons() {
    invalid_native("P1-compare-invalid", &invalid_grid(false));
}
#[test]
fn p1_native_every_invalid_number_and_comparator_slot_arithmetic() {
    invalid_native("P1-arithmetic-invalid", &invalid_grid(true));
}

#[derive(Clone)]
struct SchemaRow {
    inputs: String,
    template: String,
    goal: String,
    expected: String,
}
impl SchemaRow {
    fn probe(p: &Probe) -> Self {
        let arithmetic = p.name != "guideline_satisfies";
        let expected = if let Some((s, n)) = &p.output {
            format!("[pair({s},{n})]")
        } else if p.truth {
            "[ok]".into()
        } else {
            "[]".into()
        };
        Self {
            inputs: format!("[{}]", p.args.join(",")),
            template: if arithmetic { "pair(A,B)" } else { "ok" }.into(),
            goal: p.goal(),
            expected,
        }
    }
    fn boolean(inputs: &str, goal: &str, truth: bool) -> Self {
        Self {
            inputs: inputs.into(),
            template: "ok".into(),
            goal: goal.into(),
            expected: if truth { "[ok]" } else { "[]" }.into(),
        }
    }
}
fn schema_run(c: &Cases, schema: &Path, rows: &[SchemaRow]) -> Out {
    let text = rows
        .iter()
        .enumerate()
        .map(|(i, r)| format!("probe({i},{},{},{}).\n", r.inputs, r.template, r.goal))
        .collect::<String>();
    c.write("grid.pl", &text);
    execute(
        Command::new("swipl")
            .current_dir(&c.dir)
            .args(["-q", "-f", "none", "-F", "none", "-s"])
            .arg(schema)
            .args([
                "-s",
                "driver.pl",
                "-g",
                "run",
                "-t",
                "halt(9)",
                "--",
                "grid.pl",
            ]),
    )
}
fn schema_grid(row: &'static str, probes: &[Probe], modes: bool) {
    let mut c = Cases::new(row);
    c.write("driver.pl", &fixture("schema-driver.pl.in"));
    let schema = std::env::var_os("CKC_DA1_SCHEMA_FILE")
        .map(PathBuf::from)
        .unwrap_or_else(|| root().join("guidelines/schema.pl"));
    assert!(schema.is_file(), "schema path {}", schema.display());
    let mut rows = Vec::new();
    for p in probes {
        rows.push(SchemaRow::probe(p));
        if modes && let Some((s, n)) = &p.output {
            let n = Rat::decode(n).unwrap();
            for (s, n, truth) in [
                (s.clone(), n.term(), true),
                (s.clone(), n.add(&Rat::integer(1)).term(), false),
                ("eq".into(), n.term(), false),
                (
                    if s == "exactly" { "geq" } else { "exactly" }.into(),
                    n.term(),
                    false,
                ),
                (s.clone(), n.unreduced(), false),
            ] {
                rows.push(SchemaRow::boolean(
                    &format!("[{}]", p.args.join(",")),
                    &p.bound_goal(&s, &n),
                    truth,
                ));
            }
        }
    }
    schema_check(&mut c, &schema, &rows);
    println!(
        "DA1 {row} oracle inputs={} schema calls={}",
        probes.len(),
        rows.len()
    );
    c.finish();
}
fn schema_check(c: &mut Cases, schema: &Path, rows: &[SchemaRow]) {
    let out = schema_run(c, schema, rows);
    c.compare(
        "SWI process/no warnings/no exceptions",
        Out {
            rc: out.rc,
            stdout: String::new(),
            stderr: out.stderr.clone(),
        },
        Out::ok(""),
    );
    let lines: Vec<_> = out.stdout.lines().collect();
    c.compare(
        "every oracle row reported",
        Out::ok(lines.len().to_string()),
        Out::ok(rows.len().to_string()),
    );
    for (i, r) in rows.iter().enumerate() {
        c.compare(
            &r.goal,
            Out::ok(lines.get(i).copied().unwrap_or("<missing>")),
            Out::ok(format!("{i}\t{}", r.expected)),
        );
    }
}
#[test]
fn p2_swi_rational_integer_units_boundary_grid() {
    schema_grid(
        "P2-schema",
        &[
            comparison_grid(false, false),
            comparison_grid(false, true),
            comparison_grid(true, false),
        ]
        .concat(),
        false,
    );
}
#[test]
fn p3_swi_product_grid_canonical_outputs_and_multiplicity() {
    schema_grid("P3-schema", &arithmetic_grid("guideline_product"), true);
}
#[test]
fn p4_swi_quotient_grid_canonical_outputs_and_multiplicity() {
    schema_grid("P4-schema", &arithmetic_grid("guideline_quotient"), true);
}
#[test]
fn p1_swi_invalid_inputs_never_bind_or_raise() {
    let mut probes = [invalid_grid(false), invalid_grid(true)].concat();
    probes.extend([
        Probe::comparison(&["exactly", "1.5", "geq", "1"]),
        Probe::comparison(&["exactly", "2", "geq", "1.5"]),
        Probe::comparison(&["exactly", "1.5", "day", "geq", "1", "hour"]),
        Probe::comparison(&["exactly", "2", "day", "geq", "1.5", "hour"]),
        Probe::arithmetic("guideline_product", "exactly", "1.5", "exactly", "2"),
        Probe::arithmetic("guideline_product", "exactly", "2", "exactly", "1.5"),
        Probe::arithmetic("guideline_quotient", "exactly", "1.5", "exactly", "2"),
        Probe::arithmetic("guideline_quotient", "exactly", "2", "exactly", "1.5"),
    ]);
    schema_grid("P1-schema-invalid", &probes, false);
}
#[test]
fn p1_p5b_swi_output_aliases_partial_terms_and_input_modes() {
    let mut c = Cases::new("P1-P5b-schema-modes");
    c.write("driver.pl", &fixture("schema-driver.pl.in"));
    let schema = std::env::var_os("CKC_DA1_SCHEMA_FILE")
        .map(PathBuf::from)
        .unwrap_or_else(|| root().join("guidelines/schema.pl"));
    let mut rows = Vec::new();
    for name in ["guideline_product", "guideline_quotient"] {
        for (inputs, args) in [
            ("[exactly,2,exactly,3]", "exactly,2,exactly,3,A,A"),
            ("[exactly,A,exactly,3]", "exactly,A,exactly,3,exactly,A"),
            ("[A,2,exactly,3]", "A,2,exactly,3,A,6"),
            (
                "[exactly,rational(1,A),exactly,3]",
                "exactly,rational(1,A),exactly,3,exactly,rational(3,A)",
            ),
            ("[exactly,2,exactly,3]", "exactly,2,exactly,3,eq,A"),
            ("[exactly,0,exactly,3]", "exactly,0,exactly,3,A,f(A)"),
        ] {
            rows.push(SchemaRow::boolean(
                inputs,
                &format!("{name}({args})"),
                false,
            ));
        }
    }
    for (inputs, goal, template, expected) in [
        (
            "[exactly,3,exactly,rational(1,2)]",
            "guideline_product(exactly,3,exactly,rational(1,2),exactly,1.5)",
            "ok",
            "[]",
        ),
        (
            "[exactly,3,exactly,2]",
            "guideline_quotient(exactly,3,exactly,2,exactly,1.5)",
            "ok",
            "[]",
        ),
        (
            "[exactly,2,exactly,3]",
            "guideline_product(exactly,2,exactly,3,exactly,6.0)",
            "ok",
            "[]",
        ),
        (
            "[exactly,3,exactly,rational(1,2)]",
            "guideline_product(exactly,3,exactly,rational(1,2),A,rational(B,2))",
            "pair(A,B)",
            "[pair(exactly,3)]",
        ),
        (
            "[exactly,3,exactly,rational(1,2)]",
            "guideline_product(exactly,3,exactly,rational(1,2),exactly,rational(A,B))",
            "pair(A,B)",
            "[pair(3,2)]",
        ),
        (
            "[exactly,-3,exactly,2]",
            "guideline_quotient(exactly,-3,exactly,2,exactly,rational(A,B))",
            "pair(A,B)",
            "[pair(-3,2)]",
        ),
        (
            "[exactly,3,exactly,rational(1,2)]",
            "guideline_product(exactly,3,exactly,rational(1,2),A,rational(A,2))",
            "ok",
            "[]",
        ),
        (
            "[exactly,2,exactly,3]",
            "guideline_product(exactly,2,exactly,3,exactly,rational(A,B))",
            "ok",
            "[]",
        ),
        (
            "[exactly,2,exactly,3]",
            "guideline_product(exactly,2,exactly,3,exactly,A)",
            "pair(exactly,A)",
            "[pair(exactly,6)]",
        ),
    ] {
        rows.push(SchemaRow {
            inputs: inputs.into(),
            goal: goal.into(),
            template: template.into(),
            expected: expected.into(),
        });
    }
    schema_check(&mut c, &schema, &rows);
    c.finish();
}
#[test]
fn p9_schema_driver_detects_duplicate_exception_input_binding_and_false_result() {
    let c = Cases::new("P9-driver");
    c.write("driver.pl", &fixture("schema-driver.pl.in"));
    let schema = c.dir.join("plant.pl");
    let p = Probe::arithmetic("guideline_product", "exactly", "2", "exactly", "3");
    let row = SchemaRow::probe(&p);
    let clause = "guideline_product(exactly,2,exactly,3,exactly,6).\n";
    c.write("plant.pl", clause);
    assert_eq!(
        schema_run(&c, &schema, std::slice::from_ref(&row)),
        Out::ok("0\t[pair(exactly,6)]\n")
    );
    c.write("plant.pl", &clause.repeat(2));
    assert_eq!(
        schema_run(&c, &schema, std::slice::from_ref(&row)),
        Out::ok("0\t[pair(exactly,6),pair(exactly,6)]\n")
    );
    c.write(
        "plant.pl",
        "guideline_product(_,_,_,_,_,_) :- throw(oracle_exception).\n",
    );
    assert_eq!(
        schema_run(&c, &schema, std::slice::from_ref(&row)),
        Out::err(1, "oracle_exception\n")
    );
    c.write("plant.pl", clause);
    let mut unbound = row.clone();
    unbound.inputs = "[A,2,exactly,3]".into();
    unbound.goal = "guideline_product(A,2,exactly,3,B,C)".into();
    unbound.template = "pair(B,C)".into();
    assert_eq!(
        schema_run(&c, &schema, &[unbound]),
        Out::err(1, "bound_input(0)\n")
    );
    c.write("plant.pl", "guideline_product(_,_,_,_,_,_) :- fail.\n");
    assert_eq!(schema_run(&c, &schema, &[row]), Out::ok("0\t[]\n"));
}

fn huge_probes() -> Vec<Probe> {
    let p = format!("1{}7", "0".repeat(107));
    let q = "9".repeat(113);
    let a = Rat::fraction(&p, &q);
    let b = Rat::fraction(&q, &p);
    let c = Rat::fraction(
        &format!("-{}3", "8".repeat(129)),
        &format!("1{}9", "0".repeat(119)),
    );
    let mut out = Vec::new();
    for name in ["guideline_product", "guideline_quotient"] {
        for s in KINDS {
            for x in [&a, &a.neg(), &c, &Rat::integer(0)] {
                for y in [&b, &b.neg(), &c, &Rat::integer(0)] {
                    out.push(Probe::arithmetic(name, s, &x.term(), "exactly", &y.term()));
                }
            }
        }
    }
    let integer = Rat::fraction(&p, "1");
    for name in ["guideline_product", "guideline_quotient"] {
        for s in KINDS {
            for n in [&integer, &integer.neg()] {
                for m in [q.as_str(), "rational(3,2)"] {
                    out.push(Probe::arithmetic(name, s, &n.term(), "exactly", m));
                }
            }
        }
    }
    for s in KINDS {
        for bound in BOUNDS {
            for delta in [-1, 0, 1] {
                let n = integer.add(&Rat::integer(delta));
                out.push(Probe::comparison(&[s, &n.term(), bound, &integer.term()]));
                out.push(Probe::comparison(&[
                    s,
                    &n.term(),
                    "day",
                    bound,
                    &integer.mul(&Rat::integer(24)).term(),
                    "hour",
                ]));
            }
        }
    }
    // Large common factors cancel; the input fractions themselves stay reduced.
    for name in ["guideline_product", "guideline_quotient"] {
        for (x, y) in [(&a, &b), (&a, &a), (&c, &c), (&a.neg(), &b)] {
            out.push(Probe::arithmetic(
                name,
                "exactly",
                &x.term(),
                "exactly",
                &y.term(),
            ));
        }
    }
    for s in KINDS {
        for bound in BOUNDS {
            for delta in [-1, 0, 1] {
                let x = a.add(&Rat::fraction(
                    &delta.to_string(),
                    &format!("1{}", "0".repeat(125)),
                ));
                out.push(Probe::comparison(&[s, &x.term(), bound, &a.term()]));
                out.push(Probe::comparison(&[
                    s,
                    &x.mul(&Rat::integer(7)).term(),
                    "day",
                    bound,
                    &a.term(),
                    "week",
                ]));
                out.push(Probe::comparison(&[
                    s,
                    &x.term(),
                    "day",
                    bound,
                    &a.term(),
                    "month",
                ]));
            }
        }
    }
    out
}
#[test]
fn p6b_native_hundred_digit_cancellation_sign_and_coprime_results() {
    native_grid("P6b-huge", &huge_probes(), true);
}
#[test]
fn p6b_swi_hundred_digit_cancellation_sign_and_coprime_results() {
    schema_grid("P6b-schema", &huge_probes(), true);
}

struct Random(u64);
impl Random {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn decimal(&mut self, n: usize) -> String {
        let mut text = (1 + self.next() % 9).to_string();
        for _ in 1..n {
            text.push(char::from(b'0' + (self.next() % 10) as u8));
        }
        text
    }
}
fn generated_properties() -> Vec<Probe> {
    let mut rng = Random(0x6461_315f_6f72_6163);
    let mut out = Vec::new();
    for i in 0..48 {
        let len = [1, 9, 19, 37, 107, 141][i % 6];
        let mut p = rng.decimal(len);
        if i % 2 == 1 {
            p.insert(0, '-');
        }
        let q = rng.decimal(if i % 7 == 0 { 1 } else { len + 3 });
        let x = Rat::fraction(&p, &q);
        let y = Rat::fraction(&rng.decimal(len + 1), &rng.decimal(len + 2));
        for (j, s) in KINDS.iter().enumerate() {
            let t = KINDS[(i + j) % KINDS.len()];
            let a = Probe::arithmetic("guideline_product", s, &x.term(), t, &y.term());
            let b = Probe::arithmetic("guideline_product", t, &y.term(), s, &x.term());
            assert_eq!(a.output, b.output, "commutativity property");
            out.extend([a, b]);
            let quotient =
                Probe::arithmetic("guideline_quotient", s, &x.term(), "exactly", &y.term());
            let reciprocal = Probe::arithmetic(
                "guideline_product",
                s,
                &x.term(),
                "exactly",
                &y.reciprocal().term(),
            );
            assert_eq!(
                quotient.output, reciprocal.output,
                "reciprocal scaling property"
            );
            out.extend([quotient, reciprocal]);
            let bound = if *s == "eq" { "geq" } else { s };
            let reflexive = Probe::comparison(&[s, &x.term(), bound, &x.term()]);
            assert!(reflexive.truth, "reflexivity property");
            out.push(reflexive);
        }
        let eps = Rat::new(1, 7);
        for (s, a, t, b) in [
            ("greater", x.add(&eps), "geq", x.clone()),
            ("geq", x.clone(), "greater", x.add(&eps.neg())),
            ("greater", x.add(&eps), "greater", x.add(&eps.neg())),
        ] {
            let p = Probe::comparison(&[s, &a.term(), t, &b.term()]);
            assert!(p.truth, "transitivity witnesses");
            out.push(p);
        }
        for (u, r, group) in UNITS {
            let base = if group == 0 { "second" } else { "month" };
            let p = Probe::comparison(&[
                "exactly",
                &x.term(),
                u,
                "exactly",
                &x.mul(&Rat::integer(r)).term(),
                base,
            ]);
            assert!(p.truth, "unit scaling property");
            out.push(p);
        }
    }
    out
}
#[test]
fn p2_p3_p4_native_generated_commutativity_reciprocal_inclusion_and_scaling() {
    native_grid("P2-P3-P4-generated", &generated_properties(), false);
}
#[test]
fn p2_p3_p4_swi_generated_commutativity_reciprocal_inclusion_and_scaling() {
    schema_grid("P2-P3-P4-schema-generated", &generated_properties(), false);
}

#[test]
fn p10_public_soundness_plants_have_observable_counterexamples() {
    let mut c = Cases::new("P10-public-plants");
    c.write("m.tsv", "");
    for (goal, holds) in [
        ("guideline_product(geq,-1,geq,1,geq,-1)", false),
        (
            "guideline_product(exactly,rational(-1,2),exactly,-3,exactly,rational(3,2))",
            true,
        ),
        (
            "guideline_product(exactly,rational(2,3),exactly,3,exactly,2)",
            true,
        ),
        (
            "guideline_product(exactly,rational(2,3),exactly,3,exactly,rational(6,3))",
            false,
        ),
        (
            "guideline_quotient(exactly,1,exactly,-2,exactly,rational(-1,2))",
            true,
        ),
        ("guideline_product(exactly,0,less,-1,exactly,0)", true),
    ] {
        c.answer(goal, goal, holds);
    }
    c.finish();
}
