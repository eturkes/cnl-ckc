use super::*;

fn world(s: &Scratch, id: &str, clauses: &str, nouns: &[&str]) -> Run {
    let control = fs::read_to_string(root().join("tests/ws-controls/w1.pl.in")).unwrap();
    let header = control.split_once("% S1:").unwrap().0.replace("ws-w1", id);
    let pl = s.file(&format!("{id}.pl"), &format!("{header}{clauses}"));
    let heads: Vec<_> = nouns.iter().enumerate().map(|(i, noun)| format!("guideline_entity(actual,'$guideline_id'(product,'{id}',1,ref({}),[]),{noun},countable)", i + 1)).collect();
    let payload = format!(
        "'$guideline_proof'('{id}',1,variant(1),witness([]),prove([{}])).\n",
        heads.join(",")
    );
    let proof = s.file(&format!("{id}.proof"), &payload);
    let manifest = s.file(
        &format!("{id}.tsv"),
        &format!("{}\t{}\n", pl.display(), proof.display()),
    );
    native(s, &["v1", "aggregate-check", p(&manifest)])
}
pub fn constant_cut(s: &Scratch) -> Run {
    world(s, "ws-p4b-k2", &fixture("naf-depth-cut.pl.in"), &["cat"])
}

fn positive_retry(s: &Scratch, id: &str) -> Run {
    world(
        s,
        id,
        &fixture("positive-retry.pl.in").replace("ws-p4c", id),
        &["bird", "bird"],
    )
}

#[test]
fn p4c_positive_proof_after_pruned_branch_and_fresh_heads() {
    let s = Scratch::new("p4c");
    let mut checks = Checks::default();
    let got = positive_retry(&s, "ws-p4c");
    checks.success(
        &got,
        "positive proof after a pruned recursive branch; both heads fresh",
    );
    checks.check(
        got.out == "ace_to_pl aggregate ok 1 documents 1 obligations\n",
        format!("complete retry control: {}", got.summary()),
    );
    checks.finish();
}

#[test]
fn p6d_failed_tuple_does_not_taint_later_tuple_or_heads() {
    let s = Scratch::new("p6d");
    let c = Compiler::shared();
    let mut checks = Checks::default();
    let ace = fixture("fresh-replay.ace.in");
    let safe = ace
        .replacen("at least 1 cat then", "at least 1 bird then", 1)
        .replace("not provably risky", "not provably blue");
    let case = Case::new(
        &s,
        &c,
        "ws-p6d",
        &ace,
        Some((
            &safe,
            &[
                (",bird,countable)", ",cat,countable)"),
                (",blue,pos)", ",risky,pos)"),
            ],
        )),
    );
    checks.success(&case.emitted, "fresh tuple fork");
    if let Some(w) = checks.walk(&case.call(&s, "walks"), "fresh tuple walks") {
        checks.check(
            w.len() == 1
                && w[0].candidates == [oracle::candidates(Cmp::Some, Q::int(0), &[Q::int(1)])]
                && w[0].winner == Some(vec![2]),
            "two failed tuples leave no bindings; 0 first replays every computed head",
        );
        if let Some(w) = w.first() {
            checks.check(!w.fragment, "own risky derivation outside fragment");
        }
    }
    let proof = case.call(&s, "doc");
    checks.success(&proof, "fresh tuple certifies");
    if proof.ok() {
        checks.check(
            cardinalities(&proof.out, 1, "cat") == [Q::int(0)],
            "first replayable tuple after two failed candidates",
        );
        checks.check(
            head_counts(&proof.out, 1, "dog") == [Q::int(0)]
                && head_counts(&proof.out, 1, "frog") == [Q::int(0)],
            "all selected heads share fresh witness",
        );
        checks.success(&case.replay(&s, &proof.out), "each aggregate head fresh");
        let fork = c.emit(&s, "ws-p6d", &ace, true);
        checks.success(&fork, "fresh tuple fork proof");
        checks.check(fork.out == proof.out, "fresh replay payload parity");
    }
    checks.success(
        &positive_retry(&s, "ws-p6d-positive"),
        "fresh head budgets: each positive branch can prune before its base proof",
    );
    let got = world(
        &s,
        "ws-p6d-k2",
        &fixture("naf-cut-before-base.pl.in"),
        &["cat"],
    );
    checks.check(
        got.rc == Some(1) && got.out.is_empty() && got.err.contains("obligation_failed"),
        format!(
            "NAF cut aborts the head even before a later base: {}",
            got.summary()
        ),
    );
    checks.finish();
}

#[test]
fn p6b_coupled_naf_pins_last_operand_fastest_and_first_winner() {
    let s = Scratch::new("p6-carry");
    let c = Compiler::shared();
    let mut checks = Checks::default();
    for n in 2..=4 {
        let expr = (1..n).fold(Expr::mul(Expr::Var(0), Expr::Lit(Q::int(2))), |e, i| {
            Expr::mul(e, Expr::Var(i))
        });
        let scene = Scene {
            operands: (0..n)
                .map(|_| Operand {
                    cmp: Cmp::Some,
                    bound: Q::int(0),
                })
                .collect(),
            deny: vec![],
            expr,
        };
        let base = scene.ace(false);
        let (_, after) = base.split_once(" then ").unwrap();
        let mut positive = vec![format!("a man contains some cats {}", NOUNS[0].2)];
        for i in 1..n {
            positive.push(format!(
                "the {} contain some {} {}",
                NOUNS[i - 1].1,
                NOUNS[i].1,
                NOUNS[i].2
            ));
        }
        let positive = positive.join(" and ");
        // Relative clauses keep every count inside one NAF; a finite 'and'
        // after 'it is not provable that' leaves that clause in the outer body.
        let negative = (0..n)
            .map(|i| {
                format!(
                    "{}at least 1 {}",
                    if i == 0 { "" } else { " that contains " },
                    NOUNS[i].0
                )
            })
            .collect::<String>();
        let safe_negative = (0..n)
            .map(|i| {
                format!(
                    "{}at least 1 {}",
                    if i == 0 { "" } else { " that contains " },
                    SAFE[i].0
                )
            })
            .collect::<String>();
        let ace =
            format!("If {positive} and the man does not provably contain {negative} then {after}");
        let safe = format!(
            "If {positive} and the man does not provably contain {safe_negative} then {after}"
        );
        let swaps: Vec<_> = (0..n)
            .map(|i| {
                (
                    format!(",{},countable)", SAFE[i].0),
                    format!(",{},countable)", NOUNS[i].0),
                )
            })
            .collect();
        let refs: Vec<_> = swaps
            .iter()
            .map(|(a, b)| (a.as_str(), b.as_str()))
            .collect();
        let id = format!("ws-p6-carry-{n}");
        let case = Case::new(&s, &c, &id, &ace, Some((&safe, &refs)));
        let lists = vec![oracle::candidates(Cmp::Some, Q::int(0), &[Q::int(1)]); n];
        let expected = oracle::first(&lists, |t| {
            scene.expr.eval(t).is_some() && !t.iter().all(|&x| x >= Q::int(1))
        })
        .unwrap();
        let mut digits = vec![0; n];
        digits[n - 1] = 1;
        assert_eq!(expected, digits);
        assert_eq!(oracle::prefix(&lists, Some(&expected)), 2);
        checks.success(&case.emitted, "coupled NAF fork");
        if let Some(w) = checks.walk(&case.call(&s, "walks"), "coupled NAF walks") {
            checks.check(w.len() == 1 && w[0].candidates == lists && w[0].winner == Some(expected.clone()) && w[0].fragment, "coupled NAF: the last operand changes before any earlier operand; stop at the second tuple");
        }
        let proof = case.call(&s, "doc");
        checks.success(&proof, "coupled NAF certifies");
        if proof.ok() {
            let tuple: Vec<_> = lists.iter().zip(&expected).map(|(l, &d)| l[d]).collect();
            for i in 0..n {
                checks.check(
                    cardinalities(&proof.out, 1, NOUNS[i].0) == [tuple[i]],
                    format!("coupled NAF selected operand {i}"),
                );
            }
            checks.check(
                head_counts(&proof.out, 1, "frog") == [scene.expr.eval(&tuple).unwrap()],
                "coupled NAF result",
            );
            checks.success(&case.replay(&s, &proof.out), "coupled NAF aggregate");
            let fork = c.emit(&s, &id, &ace, true);
            checks.success(&fork, "coupled NAF fork proof");
            checks.check(fork.out == proof.out, "coupled NAF payload parity");
        }
    }
    checks.finish();
}
