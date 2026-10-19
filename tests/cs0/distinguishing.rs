use super::common::{Cases, Out, answer, query};
use super::model::{LIMIT, allow, chain, inner};

fn check(c: &mut Cases, label: &str, clauses: &[String], wh: bool, result: &str) {
    c.install(clauses);
    let q = query(&allow(if wh { "A" } else { "candidate" }), wh);
    c.write("query.pl", &q);
    c.expect(
        label,
        &["v1", "answer", "manifest.tsv", "query.pl"],
        Out::ok(answer(&q, result)),
    );
}

#[test]
fn t12_entry_prune_is_restored_for_clean_and_refuted_negations() {
    let mut c = Cases::new("T12");
    for refuted in [false, true] {
        let mut clauses = vec![format!("{} :- {}", allow("candidate"), inner("prior0"))];
        clauses.extend(chain("prior", 105, false));
        if refuted {
            clauses.push(format!("{} :- \\+ {}", allow("candidate"), inner("n0")));
            clauses.extend(chain("n", 105, false));
            clauses.push(inner("n0"));
        } else {
            clauses.push(format!(
                "{} :- \\+ {}, {}",
                allow("candidate"),
                inner("absent1"),
                inner("absent2")
            ));
        }
        clauses.push(allow("candidate"));
        for wh in [false, true] {
            check(
                &mut c,
                &format!("refuted={refuted} wh={wh}"),
                &clauses,
                wh,
                if wh { LIMIT } else { "yes" },
            );
        }
    }
    c.finish();
}
#[test]
fn t13_outer_rescue_and_prior_ground_rows_cannot_complete_a_cut_run() {
    let mut c = Cases::new("T13");
    for terminal in [false, true] {
        let mut clauses = vec![
            allow("alpha"),
            format!("{} :- \\+ {}", allow("candidate"), inner("n0")),
        ];
        clauses.extend(chain("n", 105, terminal));
        clauses.push(allow("candidate"));
        for wh in [false, true] {
            check(
                &mut c,
                &format!("terminal={terminal} wh={wh}"),
                &clauses,
                wh,
                LIMIT,
            );
        }
    }
    c.finish();
}
#[test]
fn t14_nested_clause_frames_stop_globally_and_absorb_local_positive_cuts() {
    let mut c = Cases::new("T14");
    for rescue in [false, true] {
        let mut clauses = vec![
            format!("{} :- \\+ {}", allow("candidate"), inner("wrapper")),
            format!(
                "{} :- \\+ {}, {}",
                inner("wrapper"),
                inner("n0"),
                inner("absent")
            ),
            inner("wrapper"),
        ];
        clauses.extend(chain("n", 105, true));
        if rescue {
            clauses.push(allow("candidate"));
        }
        check(
            &mut c,
            &format!("inner-alternative rescue={rescue}"),
            &clauses,
            false,
            LIMIT,
        );
    }
    let mut clauses = vec![
        format!("{} :- \\+ {}", allow("candidate"), inner("wrapper")),
        format!("{} :- \\+ {}", inner("wrapper"), inner("n0")),
    ];
    clauses.extend(chain("n", 105, false));
    clauses.push(inner("n0"));
    check(&mut c, "nested-absorption yes/no", &clauses, false, "yes");
    check(
        &mut c,
        "nested-absorption wh",
        &clauses,
        true,
        "solutions([sol([candidate])])",
    );
    c.finish();
}
#[test]
fn t15_three_layers_turn_seed_yes_into_limit_and_two_layers_stay_limit() {
    let mut c = Cases::new("T15");
    for layers in [2, 3] {
        for terminal in [false, true] {
            let mut clauses = vec![format!("{} :- \\+ {}", allow("candidate"), inner("w0"))];
            for i in 0..layers - 1 {
                let goal = if i == layers - 2 {
                    inner("n0")
                } else {
                    inner(&format!("w{}", i + 1))
                };
                clauses.push(format!("{} :- \\+ {goal}", inner(&format!("w{i}"))));
            }
            clauses.extend(chain("n", 105, terminal));
            check(
                &mut c,
                &format!("layers={layers} terminal={terminal}"),
                &clauses,
                false,
                LIMIT,
            );
        }
    }
    c.finish();
}
#[test]
fn t16_nonground_rows_only_outrank_limits_when_reached_before_the_stop() {
    let mut c = Cases::new("T16");
    for earlier in [false, true] {
        for terminal in [false, true] {
            let mut clauses = vec![allow("alpha")];
            if earlier {
                clauses.push(allow("A"));
            }
            clauses.push(format!("{} :- \\+ {}", allow("candidate"), inner("n0")));
            clauses.extend(chain("n", 105, terminal));
            if !earlier {
                clauses.push(allow("A"));
            }
            c.install(&clauses);
            let q = query(&allow("A"), true);
            c.write("query.pl", &q);
            c.expect(
                &format!("nonground-before-cut={earlier} terminal={terminal}"),
                &["v1", "answer", "manifest.tsv", "query.pl"],
                if earlier {
                    Out::error("ace_to_pl_error(proof,nonground_solution(cs0)).\n")
                } else {
                    Out::ok(answer(&q, LIMIT))
                },
            );
        }
    }
    c.finish();
}

fn balanced(first: &str, rest: &str, leaves: usize) -> String {
    if leaves == 1 {
        return first.into();
    }
    let left = leaves / 2;
    format!(
        "','({},{})",
        balanced(first, rest, left),
        balanced(rest, rest, leaves - left)
    )
}
#[test]
fn t17_clean_and_refuted_frames_keep_the_exact_transition_boundary() {
    let mut c = Cases::new("T17");
    for refuted in [false, true] {
        let goal = inner(if refuted { "token" } else { "absent" });
        let mut clauses = vec![
            inner("token"),
            format!("{} :- \\+ {goal}", allow("candidate")),
        ];
        if refuted {
            clauses.push(allow("candidate"));
        }
        // N leaves cost 2N steps. This first leaf adds 2 (clean NAF) or
        // 4 (refuted NAF + retry) steps without a resumed depth-cut frame.
        let boundary = if refuted { 49998 } else { 49999 };
        for leaves in [boundary - 1, boundary, boundary + 1] {
            c.install(&clauses);
            let q = query(
                &balanced(&allow("candidate"), &inner("token"), leaves),
                false,
            );
            c.write("query.pl", &q);
            c.expect(
                &format!("refuted={refuted} leaves={leaves}"),
                &["v1", "answer", "manifest.tsv", "query.pl"],
                Out::ok(answer(&q, if leaves <= boundary { "yes" } else { LIMIT })),
            );
        }
    }
    c.finish();
}
