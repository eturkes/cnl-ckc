use super::common::*;
use super::oracle::*;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn differential(row: &'static str, probes: &[Probe]) {
    reference_self_checks();
    let mut c = Cases::new(row);
    c.write("m.tsv", "");
    for p in probes {
        c.answer(&p.goal(), &p.goal(), p.expected());
    }
    c.finish();
}
#[test]
fn p2_unitless_comparison_grid() {
    differential("P2-unitless", &unitless_grid());
}
#[test]
fn p2_every_ordered_unit_pair_and_dense_boundary() {
    differential("P2-units", &unit_grid());
}
#[test]
fn p2_hundred_digit_values_and_scaled_equalities() {
    differential("P2-huge", &huge_grid());
}
#[test]
fn p2_ill_typed_arguments_are_false_not_errors() {
    differential("P2-types", &invalid_grid());
}
#[test]
fn p2_generated_arbitrary_precision_differential() {
    differential("P2-generated", &generated_grid());
}

#[test]
fn p2_inclusion_and_unit_scaling_properties() {
    differential("P2-properties", &property_grid());
}

fn schema_run(c: &Cases, schema: &std::path::Path, probes: &[Probe]) -> Out {
    let cases: String = probes
        .iter()
        .enumerate()
        .map(|(i, p)| format!("probe({i},{}).\n", p.goal()))
        .collect();
    c.write("grid.pl", &cases);
    let mut command = Command::new("swipl");
    command
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
        ]);
    let out = execute(&mut command);
    assert_eq!(fs::read_to_string(c.dir.join("grid.pl")).unwrap(), cases);
    out
}
#[test]
fn p2_schema_matches_the_same_oracle_without_errors_or_duplicates() {
    reference_self_checks();
    let c = Cases::new("P2-schema");
    c.write("driver.pl", &fixture("schema-driver.pl.in"));
    let positive = [Probe::new(&["exactly", "60", "geq", "50"])];
    let plant = c.dir.join("schema-control.pl");
    c.write(
        "schema-control.pl",
        "guideline_satisfies(exactly,60,geq,50).\n",
    );
    assert_eq!(schema_run(&c, &plant, &positive), Out::ok("0\t1\n"));
    c.write(
        "schema-control.pl",
        &"guideline_satisfies(exactly,60,geq,50).\n".repeat(2),
    );
    assert_eq!(
        schema_run(&c, &plant, &positive),
        Out::ok("0\t2\n"),
        "duplicate-solution plant fires"
    );
    c.write(
        "schema-control.pl",
        "guideline_satisfies(_,_,_,_) :- fail.\n",
    );
    assert_eq!(
        schema_run(&c, &plant, &positive),
        Out::ok("0\t0\n"),
        "false-verdict plant fires"
    );
    c.write(
        "schema-control.pl",
        "guideline_satisfies(_,_,_,_) :- throw(probe_exception).\n",
    );
    assert_eq!(
        schema_run(&c, &plant, &positive),
        Out::err(1, "probe_exception\n"),
        "exception plant fires"
    );
    c.write(
        "schema-control.pl",
        "guideline_satisfies(exactly,60,geq,50).\n",
    );
    assert_eq!(
        schema_run(&c, &plant, &[Probe::new(&["A", "60", "geq", "50"])]),
        Out::err(1, "bound_argument(0)\n"),
        "binding plant fires"
    );
    println!(
        "NT1 P2-schema driver: positive control + duplicate/verdict/exception/binding plants fired"
    );
    let schema = std::env::var_os("CKC_NT1_SCHEMA_FILE")
        .map(PathBuf::from)
        .unwrap_or_else(|| root().join("guidelines/schema.pl"));
    assert!(
        schema.is_file(),
        "D10 schema payload absent: {}",
        schema.display()
    );
    let mut probes = all_probes();
    // R21 excludes floats from native input; the public Prolog predicate still guards them.
    probes.push(Probe::new(&["exactly", "60.0", "geq", "50"]));
    probes.push(Probe::new(&["exactly", "60", "day", "geq", "50.0", "day"]));
    let expected: Vec<_> = probes
        .iter()
        .enumerate()
        .map(|(i, p)| format!("{i}\t{}", usize::from(p.expected())))
        .collect();
    let out = schema_run(&c, &schema, &probes);
    assert_eq!(out.rc, 0, "schema grid stderr: {}", out.stderr);
    assert_eq!(out.stderr, "", "schema grid stderr");
    let actual: Vec<_> = out.stdout.lines().collect();
    assert_eq!(
        actual.len(),
        expected.len(),
        "every generated input gets one verdict"
    );
    let differences: Vec<_> = actual
        .iter()
        .zip(&expected)
        .enumerate()
        .filter(|(_, (a, b))| **a != b.as_str())
        .map(|(i, (a, b))| (i, *a, b.as_str()))
        .collect();
    assert!(
        differences.is_empty(),
        "schema differential: {} mismatches, first 8 = {:?}",
        differences.len(),
        &differences[..differences.len().min(8)]
    );
    assert!(out.stdout.ends_with('\n'));
    println!(
        "NT1 P2-schema probes={} native-grid+float-guards",
        probes.len()
    );
}
