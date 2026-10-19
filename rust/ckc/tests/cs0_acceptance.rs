#[path = "../../../tests/cs0/common.rs"]
mod common;
#[path = "../../../tests/cs0/distinguishing.rs"]
mod distinguishing;
#[path = "../../../tests/cs0/model.rs"]
mod model;

use common::{Cases, Out, answer, document, proof, query, trace};
use model::{Scenario, allow, chain, inner};

fn answers(row: &'static str, scenarios: Vec<Scenario>) {
    let mut c = Cases::new(row);
    for case in scenarios {
        c.install(&case.clauses);
        let q = query(&case.goal(), case.wh);
        c.write("query.pl", &q);
        c.expect(
            &case.label,
            &["v1", "answer", "manifest.tsv", "query.pl"],
            Out::ok(answer(&q, &case.result)),
        );
    }
    c.finish();
}

#[test]
fn t01_resumed_depth_cut_ends_the_run_before_any_outer_fallback() {
    answers("T1", model::t1());
}
#[test]
fn t02_finite_negations_keep_their_truth_and_continuations() {
    answers("T2", model::t2());
}
#[test]
fn t03_inner_proof_absorbs_earlier_inner_depth_cuts() {
    answers("T3", model::t3());
}
#[test]
fn t04_absorption_keeps_wh_enumeration_complete_and_rows_intact() {
    answers("T4", model::t4());
}
#[test]
fn t05_nested_resumed_cut_cannot_be_negated_away() {
    answers("T5", model::t5());
}
#[test]
fn t06_wh_search_crossing_a_resumed_cut_remains_incomplete() {
    answers("T6", model::t6());
}
#[test]
fn t07_outer_prunes_survive_negation_but_do_not_disprove_positive_proofs() {
    answers("T7", model::t7());
}
#[test]
fn t08_fuel_exhaustion_inside_negation_stays_a_limit() {
    answers("T8", model::t8());
}

fn replay(c: &Cases, links: usize, terminal: bool, reverse: bool, guarded: bool) {
    let fact = "guideline_entity(actual,'$guideline_id'(product,cs0,1,ref(1),[]),probe,countable)"
        .to_string();
    let clauses = [
        fact.clone(),
        format!("{} :- \\+ {}", allow("candidate"), inner("n0")),
    ];
    c.write("doc.pl", &document("cs0", &clauses));
    c.write(
        "support.pl",
        &document("support", &chain("n", links, terminal)),
    );
    let mut heads = vec![fact];
    if guarded {
        heads.push(allow("candidate"));
    }
    c.write("payload.pl", &proof(&heads));
    c.write("support.proof", "");
    c.write(
        "manifest.tsv",
        if reverse {
            "support.pl\tsupport.proof\ndoc.pl\tpayload.pl\n"
        } else {
            "doc.pl\tpayload.pl\nsupport.pl\tsupport.proof\n"
        },
    );
}
#[test]
fn t09_aggregate_replay_cannot_prove_a_head_through_a_cut_negation() {
    let mut c = Cases::new("T9");
    for links in [3997, 3998, 4000] {
        for terminal in [false, true] {
            for reverse in [false, true] {
                replay(&c, links, terminal, reverse, true);
                c.expect(
                    &format!("links={links} terminal={terminal} reverse={reverse}"),
                    &["v1", "aggregate-check", "manifest.tsv"],
                    if links >= 3998 || terminal {
                        Out::error("ace_to_pl_error(proof,obligation_failed(cs0,1,variant(1))).\n")
                    } else {
                        Out::ok("ace_to_pl aggregate ok 2 documents 1 obligations\n")
                    },
                );
            }
        }
    }
    c.finish();
}
#[test]
fn t10_same_knowledge_with_an_ordinary_fact_obligation_still_replays() {
    let mut c = Cases::new("T10");
    for links in [3997, 3998, 4000] {
        for terminal in [false, true] {
            for reverse in [false, true] {
                replay(&c, links, terminal, reverse, true);
                let files = ["doc.pl", "support.pl", "manifest.tsv", "payload.pl"];
                let guarded = files.map(|name| std::fs::read(c.dir.join(name)).unwrap());
                replay(&c, links, terminal, reverse, false);
                for (name, before) in files.into_iter().zip(guarded) {
                    let after = std::fs::read(c.dir.join(name)).unwrap();
                    if name == "payload.pl" {
                        assert_ne!(before, after, "the guarded-head obligation is removed");
                    } else {
                        assert_eq!(before, after, "knowledge-only control keeps {name}");
                    }
                }
                c.expect(
                    &format!("links={links} terminal={terminal} reverse={reverse}"),
                    &["v1", "aggregate-check", "manifest.tsv"],
                    Out::ok("ace_to_pl aggregate ok 2 documents 1 obligations\n"),
                );
            }
        }
    }
    c.finish();
}
#[test]
fn t11_traces_over_the_contract_answers_keep_the_demonstration_boundary() {
    let mut c = Cases::new("T11");
    for case in model::all() {
        c.install(&case.clauses);
        let q = query(&case.goal(), case.wh);
        let a = answer(&q, &case.result);
        let t = trace(&q, &a, &case.trace_result);
        c.write("query.pl", &q);
        c.write("answers.pl", &a);
        c.write("traces.pl", &t);
        c.expect(
            &format!("{}/trace", case.label),
            &["v1", "trace", "manifest.tsv", "query.pl", "answers.pl"],
            Out::ok(&t),
        );
        let demo = (case.trace_result.starts_with("yes(proved([")
            || case.trace_result.starts_with("solutions([sol("))
            && !case.trace_result.contains("unproved(");
        c.expect(
            &format!("{}/trace-check", case.label),
            &[
                "v1",
                "trace-check",
                "manifest.tsv",
                "query.pl",
                "answers.pl",
                "traces.pl",
            ],
            if demo {
                Out::ok(format!(
                    "ckc: trace-check ok cs0 nodes={}\n",
                    case.trace_result.matches("clause(").count()
                ))
            } else {
                Out::error("ace_to_pl_error(proof,trace_check(non_demo)).\n")
            },
        );
    }
    c.finish();
}
