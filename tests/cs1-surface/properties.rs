use super::compiler::Compiler;
use super::model::{self, KEY, Product, quote};
use super::support::*;

#[test]
fn a4_scope_noun_and_exclusion_verb_are_table_parameters() {
    let scratch = Scratch::new("lemma-property");
    let compiler = Compiler::new(&scratch.0);
    let mut checks = Checks::default();
    for (i, (noun, verb, finite)) in [
        ("recommendation", "apply", "applies"),
        ("dog", "sleep", "sleeps"),
        ("cat", "wait", "waits"),
    ]
    .iter()
    .enumerate()
    {
        let table = format!(
            "{}{}scope\t{noun}\t{KEY}\nexclusion\t{verb}\tscope\n",
            model::HEADER,
            model::BASE
        );
        for (shape, source_noun, source_verb, record, reject) in [
            ("match", *noun, *finite, true, false),
            ("other-verb", *noun, "works", false, false),
            ("other-noun", "woman", *finite, false, true),
        ] {
            let id = format!("cs-lemma-{i}-{shape}");
            let ace =
                format!("If a man works then it is false that a {source_noun} {source_verb}.\n");
            let dump = compiler.dump(&scratch.0.join("dump"), &ace);
            dump.require_ok(&id);
            assert!(
                dump.stdout.contains("messages([])."),
                "{id} parser {}",
                dump.stdout
            );
            let got = compiler.run(&scratch.0.join("doc"), &id, "doc", &ace, Some(&table));
            if reject {
                checks.require(
                    got.rc == Some(1)
                        && got.stdout.is_empty()
                        && got.stderr == "ace_to_pl_error(unsupported,scope_shape(subject,1)).\n",
                    format!("{id} subject boundary: {}", got.describe()),
                );
            } else {
                let model = Product::from_dump(&id, &ace, &dump.stdout, true);
                let old = format!(
                    "vocabulary(sha256({}))",
                    quote(&sha(model::table(Some(KEY))))
                );
                let new = format!("vocabulary(sha256({}))", quote(&sha(&table)));
                let expected = model.document(Some(KEY), record).replacen(&old, &new, 1);
                checks.require(got.ok(), format!("{id}: {}", got.describe()));
                if got.ok() {
                    checks.equal(&got.stdout, &expected, &id);
                }
            }
        }
    }
    checks.finish("A4 named scope/exclusion lemmas: parameter property");
}
#[test]
fn a5_plain_indefinite_subject_count_boundary_property() {
    let scratch = Scratch::new("subject-count");
    let compiler = Compiler::new(&scratch.0);
    let mut checks = Checks::default();
    for (cmp, words) in [
        ("eq", ""),
        ("geq", "at least "),
        ("greater", "more than "),
        ("leq", "at most "),
        ("less", "less than "),
        ("exactly", "exactly "),
    ] {
        for count in [0, 1, 2, 18] {
            let id = format!("cs-subject-{cmp}-{count}");
            let noun = if count == 1 {
                "recommendation"
            } else {
                "recommendations"
            };
            let verb = if count == 1 { "applies" } else { "apply" };
            let ace = format!(
                "If a patient sleeps then it is false that {words}{count} {noun} {verb} to the patient.\n"
            );
            let dump = compiler.dump(&scratch.0.join("dump"), &ace);
            dump.require_ok(&id);
            assert!(
                dump.stdout.contains("messages([])."),
                "{id} parser {}",
                dump.stdout
            );
            let got = compiler.run(
                &scratch.0.join("doc"),
                &id,
                "doc",
                &ace,
                Some(&model::table(Some(KEY))),
            );
            if cmp == "eq" && count == 1 {
                let expected =
                    Product::from_dump(&id, &ace, &dump.stdout, true).document(Some(KEY), true);
                checks.require(got.ok(), format!("{id}: {}", got.describe()));
                if got.ok() {
                    checks.equal(&got.stdout, &expected, &id);
                }
            } else {
                checks.require(
                    got.rc == Some(1)
                        && got.stdout.is_empty()
                        && got.stderr
                            == "ace_to_pl_error(unsupported,scope_shape(consequent,1)).\n",
                    format!("{id} expected consequent: {}", got.describe()),
                );
            }
        }
    }
    checks.finish("A5 subject object must be countable/na/eq/1: 24-case property");
}

#[test]
fn a5_duplicate_grade_precedes_root_deontic_scope_reject() {
    let scratch = Scratch::new("grade-precedence");
    let compiler = Compiler::new(&scratch.0);
    let mut checks = Checks::default();
    let base = format!(
        "{}strength\tdog\tstrong\nstrength\tcat\tconditional\n",
        model::HEADER
    );
    let scoped = format!("{base}scope\trecommendation\t{KEY}\nexclusion\tapply\tscope\n");
    let ace = "A recommendation is a dog and is a cat and should sleep.\n";
    for (label, table) in [("unscoped control", &base), ("scoped precedence", &scoped)] {
        let got = compiler.run(
            &scratch.0.join("work"),
            "cs-grade-precedence",
            "doc",
            ace,
            Some(table),
        );
        checks.require(
            got.rc == Some(1)
                && got.stdout.is_empty()
                && got.stderr
                    == "ace_to_pl_error(unsupported,grade_shape(duplicate_dimension,1)).\n",
            format!(
                "{label}: expected duplicate_dimension before root_deontic; {}",
                got.describe()
            ),
        );
    }
    checks.finish("A5 duplicate-grade barrier before scope pass");
}
