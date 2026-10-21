use super::model::{Goal, Product};
use super::preservation::decimal_envelope as decimal_dump;
use super::support::*;
use super::surface::results;
use super::term::*;

fn anchored<'a>(dump: &'a str, prefix: &str) -> &'a str {
    let start = dump.find(prefix).expect("anchored condition");
    let (mut depth, mut quoted, mut escaped) = (0, false, false);
    for (i, ch) in dump[start..].char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if quoted && ch == '\\' {
            escaped = true;
            continue;
        }
        if ch == '\'' {
            quoted = !quoted;
            continue;
        }
        if quoted {
            continue;
        }
        if ch == '(' {
            depth += 1;
        }
        if ch == ')' {
            depth -= 1;
            if depth == 0 {
                return &dump[start..start + i + 1];
            }
        }
    }
    panic!("unterminated anchored condition {prefix}");
}
fn mutate(mut product: Product, kind: &str) -> Product {
    let group = &mut product.groups[0];
    let at = group.body.iter().position(|g| matches!(g, Goal::Pos(t) if ["guideline_product","guideline_quotient"].contains(&t.name()))).unwrap();
    match kind {
        "drop" => {
            group.body.remove(at);
        }
        "extra" => {
            group.body.insert(at, group.body[at].clone());
        }
        "move" => {
            let node = group.body.remove(at);
            group.body.insert(0, node);
        }
        "head-constant" | "head-geq-two" => {
            let h = group
                .heads
                .iter_mut()
                .find(|h| h.name() == "guideline_cardinality")
                .unwrap();
            let Term::Call(_, xs) = h else { unreachable!() };
            xs[3] = a(if kind == "head-constant" {
                "exactly"
            } else {
                "geq"
            });
            xs[4] = a(if kind == "head-constant" { 7 } else { 2 });
        }
        "some-filter" => {
            let Goal::Pos(t) = &group.body[at] else {
                unreachable!()
            };
            let xs = t.args();
            group.body.insert(
                at,
                Goal::Pos(c(
                    "guideline_satisfies",
                    vec![xs[0].clone(), xs[1].clone(), a("geq"), a(2)],
                )),
            );
        }
        _ => {
            let Goal::Pos(Term::Call(name, xs)) = &mut group.body[at] else {
                unreachable!()
            };
            match kind {
                "literal" => xs[3] = c("rational", vec![a(5), a(2)]),
                "swap-operands" => {
                    xs.swap(0, 2);
                    xs.swap(1, 3);
                }
                "operator" => {
                    *name = if name == "guideline_product" {
                        "guideline_quotient"
                    } else {
                        "guideline_product"
                    }
                    .into()
                }
                "relink" => xs[1] = xs[4].clone(),
                _ => panic!("mutation {kind}"),
            }
        }
    }
    product
}
#[test]
fn p5_clause_tampering_rejects_each_chain_head_and_link_change() {
    let mut checks = Checks::default();
    let work = Scratch::new("mutants");
    for kind in [
        "literal",
        "swap-operands",
        "operator",
        "drop",
        "extra",
        "move",
        "head-constant",
        "head-geq-two",
        "some-filter",
        "relink",
    ] {
        let id = if kind == "swap-operands" {
            "da2-two-operands"
        } else {
            "da2-product"
        };
        let obs = &results()[id];
        let changed = mutate(obs.model.clone(), kind).document();
        assert_ne!(
            changed,
            obs.model.document(),
            "{kind} mutation must change bytes"
        );
        let got = certify(
            &work.0,
            id,
            &obs.case.ace,
            &obs.dump.stdout,
            &changed,
            obs.case.table(),
        );
        checks.output(
            &got,
            &Run::err(1, format!("ckc: certify: {id}: clauses(1).\n")),
            kind,
        );
    }
    checks.finish("P5/mutants");
}
#[test]
fn p2_synthetic_dump_leaf_must_match_token_and_source() {
    let work = Scratch::new("leaf-mismatch");
    let mut checks = Checks::default();
    for (id, from, to) in [
        (
            "da2-product",
            "real('$decimal'('1.5'))",
            "real('$decimal'('2.5'))",
        ),
        ("da2-right-chain", "int(3)", "int(5)"),
        ("da2-right-chain", "int(2)", "int(5)"),
        (
            "da2-right-chain",
            "real('$decimal'('4.7'))",
            "real('$decimal'('5.7'))",
        ),
    ] {
        let obs = &results()[id];
        let dump = decimal_dump(&obs.dump.stdout);
        assert_eq!(dump.matches(from).count(), 1, "synthetic leaf anchor");
        let tampered = dump.replacen(from, to, 1);
        let got = certify(
            &work.0,
            id,
            &obs.case.ace,
            &tampered,
            &obs.model.document(),
            obs.case.table(),
        );
        checks.output(
            &got,
            &Run::err(
                1,
                format!("ckc: certify: {id}: unsupported(sentence(1,formula_shape(literal))).\n"),
            ),
            &format!("{id}: {from} -> {to}; token/source unchanged"),
        );
    }
    let obs = &results()["da2-product"];
    let dump = decimal_dump(&obs.dump.stdout);
    let anchor = ",'$decimal'('1.5'),'.']";
    assert_eq!(dump.matches(anchor).count(), 1);
    let extra_token = dump.replacen(anchor, ",'$decimal'('1.5'),99,'.']", 1);
    let expected = Run::err(
        1,
        "ckc: certify: da2-product: unsupported(sentence(1,formula_shape(literal))).\n",
    );
    let got = certify(
        &work.0,
        &obs.case.id,
        &obs.case.ace,
        &extra_token,
        &obs.model.document(),
        None,
    );
    checks.output(
        &got,
        &expected,
        "numeric token after the matching prefix has no source lexeme",
    );
    let mut source = obs.model.clone();
    source.case.ace = source
        .case
        .ace
        .replace("1.5.", "1.5 and the opioid-dosage sees 99 cats.");
    assert_ne!(source.case.ace, obs.case.ace);
    let got = certify(
        &work.0,
        &source.case.id,
        &source.case.ace,
        &dump,
        &source.document(),
        None,
    );
    checks.output(
        &got,
        &expected,
        "source lexeme after the matching prefix has no numeric token",
    );
    checks.finish("P2/leaf-token");
}
#[test]
fn p3_formula_positions_inside_boxes_and_naf_are_rejected() {
    let obs = &results()["da2-product"];
    let mut checks = Checks::default();
    let work = Scratch::new("synthetic-position");
    let dump = decimal_dump(&obs.dump.stdout);
    let raw_formula = anchored(&dump, "-(formula(");
    // A NAF box in a rule consequent rejects while the rule flattens, before the
    // formula pass (D3 pass order), formula or not.
    for (op, why) in [
        ("should", "formula_shape(position)"),
        ("~", "naf_placement"),
    ] {
        let replacement = format!("{op}(drs([],[{raw_formula}]))");
        let changed = dump.replacen(raw_formula, &replacement, 1);
        let got = certify(
            &work.0,
            &obs.case.id,
            &obs.case.ace,
            &changed,
            &obs.model.document(),
            obs.case.table(),
        );
        checks.output(
            &got,
            &Run::err(
                1,
                format!("ckc: certify: da2-product: unsupported(sentence(1,{why})).\n"),
            ),
            op,
        );
    }
    checks.finish("P3/synthetic-position");
}
#[test]
fn p3_operand_scope_bound_and_missing_target_dump_boundaries() {
    let obs = &results()["da2-product"];
    let dump = decimal_dump(&obs.dump.stdout);
    let object = anchored(&dump, "-(object(B,");
    let target = anchored(&dump, "-(object(D,");
    let mut cases = Vec::new();
    for op in ["should", "~"] {
        cases.push((
            op,
            "operand",
            dump.replacen(object, &format!("{op}(drs([],[{object}]))"), 1),
        ));
    }
    cases.push((
        "disjunction arm",
        "operand",
        dump.replacen(object, &format!("v(drs([],[{object}]),drs([],[]))"), 1),
    ));
    for cmp in ["about", "na", "eq"] {
        let changed = object.replace(",geq,2)", &format!(",{cmp},2)"));
        assert_ne!(changed, object);
        cases.push((cmp, "operand_bound", dump.replacen(object, &changed, 1)));
    }
    cases.push((
        "missing target",
        "target",
        dump.replacen(&format!("{target},"), "", 1),
    ));
    let work = Scratch::new("synthetic-operands");
    let mut checks = Checks::default();
    for (label, why, changed) in cases {
        assert_ne!(changed, dump, "plant {label}");
        let got = certify(
            &work.0,
            &obs.case.id,
            &obs.case.ace,
            &changed,
            &obs.model.document(),
            obs.case.table(),
        );
        checks.output(
            &got,
            &Run::err(
                1,
                format!(
                    "ckc: certify: da2-product: unsupported(sentence(1,formula_shape({why}))).\n"
                ),
            ),
            label,
        );
    }
    checks.finish("P3/synthetic-operands");
}
#[test]
fn p3_question_formula_keeps_query_leaf_refusal() {
    let compiler = compiler();
    let work = Scratch::new("formula-question");
    let ace = "Is it true that 6 = 2 * 3?\n";
    let dump = compiler.dump(&work.0, ace.as_bytes(), None, None);
    dump.require_ok("question syntax");
    assert!(dump.stdout.contains("messages([])."));
    let got = compiler.compile(
        &work.0,
        "formula-question",
        ace.as_bytes(),
        None,
        None,
        "question",
    );
    let mut checks = Checks::default();
    checks.output(
        &got,
        &Run::err(
            1,
            "ace_to_pl_error(unsupported,query_unsupported(leaf(/(formula,3)),1)).\n",
        ),
        "question formula",
    );
    checks.finish("P3/question preservation");
}
#[test]
fn p4_kernel_negative_lower_bound_chooses_nonzero_witness() {
    let compiler = compiler();
    let work = Scratch::new("negative-witness");
    let ulex = work.0.join("lexicon.ulex");
    write(&ulex, super::cases::ULEX);
    let case = super::cases::simple(
        "da2-kernel-greater-negative",
        "more than 2",
        "2 / X",
        "kernel-witness",
    );
    let parsed = compiler.dump(&work.0, case.ace.as_bytes(), Some(&ulex), None);
    parsed.require_ok("kernel witness source scene");
    let anchor = ",countable,na,greater,2)";
    assert_eq!(parsed.stdout.matches(anchor).count(), 1);
    // D4's kernel-only input: APE cannot emit a negative cardinality; the supplied DRS can.
    let dump = parsed
        .stdout
        .replacen(anchor, ",countable,na,greater,-1)", 1);
    let model = Product::from_dump(&case, &dump);
    let expected = model.proof();
    assert!(expected.contains(",na,exactly,1)"));
    assert!(expected.contains(",na,exactly,2)"));
    let got = certify(&work.0, &case.id, &case.ace, &dump, &model.document(), None);
    let mut checks = Checks::default();
    checks.output(
        &got,
        &Run::ok(expected),
        "greater -1: witness 1 makes 2 / X derive 2",
    );
    checks.finish("P4/kernel negative bound");
}
