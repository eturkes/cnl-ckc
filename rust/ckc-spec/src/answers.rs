use vstd::prelude::*;
use crate::term::*;
use crate::v1text::*;
use crate::engine::*;
use crate::replay::*;

verus! {

// Trusted spec: answer mode (contract m5u2 K2; legacy law ace_to_pl.pl
// answer_mode). Pipeline = manifest → query custody → load + records
// (skipped for an empty composition) → solve under the R8 answer bounds →
// one canonical answers file on stdout.

pub open spec fn query_reject(why: Term) -> Out { check_load(Term::Comp(ascii("query_file"@), seq![why])) }

// --- class gate: the legacy term census admits exactly two terms ---

pub open spec fn clause_count(bs: Seq<Bundle>) -> nat
    decreases bs.len(),
{
    if bs.len() == 0 { 0 } else { bs[0].clauses.len() + clause_count(bs.drop_first()) }
}

// Doc = 18 declaration directives + 2 records + clauses; answers/traces = one record.
pub open spec fn term_count(f: V1File) -> nat {
    match f { V1File::Doc(d) => 20 + clause_count(d.bundles), V1File::Query(_) => 2, _ => 1 }
}

// --- goal walk: depth-first, left to right, first offender wins ---

pub open spec fn foreign(name: Term, arity: nat) -> Term {
    Term::Comp(ascii("goal_foreign"@), seq![name, Term::Int(arity as int)])
}

pub open spec fn goal_walk(g: Term) -> Option<Term>
    decreases g,
{
    match g {
        Term::Var(_) => Option::Some(atom("goal_variable"@)),
        Term::Comp(name, args) =>
            if name == comma_name() && args.len() == 2 {
                match goal_walk(args[0]) { Option::Some(w) => Option::Some(w), Option::None => goal_walk(args[1]) }
            } else if is_semantic_pred(name, args.len()) { Option::None }
            else { Option::Some(foreign(Term::Atom(name), args.len())) },
        _ => Option::Some(foreign(g, 0)),   // an atomic leaf is its own functor name
    }
}

// --- answer rows ---

pub open spec fn list_items(t: Term) -> Option<Seq<Term>>
    decreases t,
{
    match t {
        Term::Nil => Option::Some(Seq::empty()),
        Term::Comp(name, args) =>
            if name == cons_name() && args.len() == 2 {
                match list_items(args[1]) { Option::Some(rest) => Option::Some(seq![args[0]] + rest), Option::None => Option::None }
            } else { Option::None },
        _ => Option::None,
    }
}

pub open spec fn desc_ok(d: Term) -> bool {
    match d {
        Term::Comp(name, args) =>
            (name == ascii("noun"@) && args.len() == 2 && args[0] is Atom && args[1] is Atom)
            || (name == ascii("wh"@) && args.len() == 1 && (args[0] == atom("who"@) || args[0] == atom("what"@))),
        _ => false,
    }
}

pub open spec fn answer_var(i: nat, why: Seq<char>) -> Term {
    Term::Comp(ascii("answer_var"@), seq![Term::Int(i as int), atom(why)])
}

// Row i (1-based): shape → variable → occurs in the goal → fresh → descriptor.
pub open spec fn rows_check(rows: Seq<Term>, i: nat, goal: Term, seen: Seq<nat>) -> Option<Term>
    decreases rows.len(),
{
    if rows.len() == 0 { Option::None } else {
        match rows[0] {
            Term::Comp(name, args) =>
                if name == ascii("answer"@) && args.len() == 2 {
                    match args[0] {
                        Term::Var(x) =>
                            if !occurs(x, goal) { Option::Some(answer_var(i, "absent"@)) }
                            else if seen.contains(x) { Option::Some(answer_var(i, "duplicate"@)) }
                            else if !desc_ok(args[1]) { Option::Some(Term::Comp(ascii("answer_desc"@), seq![Term::Int(i as int)])) }
                            else { rows_check(rows.drop_first(), i + 1, goal, seen.push(x)) },
                        _ => Option::Some(answer_var(i, "nonvar"@)),
                    }
                } else { Option::Some(Term::Comp(ascii("answer_shape"@), seq![Term::Int(i as int)])) },
            _ => Option::Some(Term::Comp(ascii("answer_shape"@), seq![Term::Int(i as int)])),
        }
    }
}

// --- query custody ---

pub open spec fn custody(query: Src) -> Result<(QueryFile, Seq<Term>), Out> {
    match query {
        Src::Missing => Result::Err(check_load(atom("unreadable"@))),   // SWI open error = rust class map (t-c1008 shape)
        Src::Bad(off) => Result::Err(utf8_reject(off)),
        Src::Bytes(b) =>
            if !accepts(b) { Result::Err(query_reject(atom("noncanonical"@))) } else {
                match the_v1(b) {
                    V1File::Query(q) => match goal_walk(q.goal) {
                        Option::Some(w) => Result::Err(query_reject(w)),
                        Option::None => match list_items(q.answers) {
                            Option::None => Result::Err(query_reject(atom("answers_list"@))),
                            Option::Some(rows) => match rows_check(rows, 1, q.goal, Seq::empty()) {
                                Option::Some(w) => Result::Err(query_reject(w)),
                                Option::None => Result::Ok((q, rows)),
                            },
                        },
                    },
                    f => Result::Err(query_reject(Term::Comp(ascii("term_count"@), seq![Term::Int(term_count(f) as int)]))),
                }
            },
    }
}

// --- solve + classify ---

pub open spec fn row_ground(r: Seq<Term>) -> bool {
    forall|i: int| 0 <= i < r.len() ==> ground(#[trigger] r[i])
}

pub open spec fn some_nonground(rows: Seq<Seq<Term>>) -> bool {
    exists|i: int| 0 <= i < rows.len() && !row_ground(#[trigger] rows[i])
}

pub open spec fn sol_term(r: Seq<Term>) -> Term { Term::Comp(ascii("sol"@), seq![list_term(r)]) }

pub open spec fn limit() -> Term { Term::Comp(ascii("indeterminate"@), seq![atom("limit"@)]) }

// Yes-no (no rows) = bounded once: the first proof is conclusive; wh = every
// solution under the shared budget, a nonground row outranks the limit fold.
pub open spec fn answer_result(db: Seq<DocClause>, q: QueryFile, rows: Seq<Term>) -> Result<Term, Out> {
    let sol = rows.map_values(|r: Term| arg(r, 0));
    match solve(db, q.goal, answer_depth(), sol, rows.len() > 0, answer_inf()) {
        ROut::Sol => Result::Ok(atom("yes"@)),
        ROut::End { complete, rows: found } =>
            if rows.len() == 0 {
                Result::Ok(if complete { Term::Comp(ascii("no"@), seq![atom("finite_failure"@)]) } else { limit() })
            } else if some_nonground(found) {
                Result::Err(proof_fail(Term::Comp(ascii("nonground_solution"@), seq![Term::Atom(q.qid)])))
            } else if !complete { Result::Ok(limit()) }
            else {
                Result::Ok(Term::Comp(ascii("solutions"@), seq![list_term(sort_unique(found.map_values(|r: Seq<Term>| sol_term(r))))]))
            },
    }
}

pub open spec fn composition(rows: Seq<MRow>, pls: Seq<Src>) -> Result<Seq<DocFile>, Out> {
    if rows.len() == 0 { Result::Ok(Seq::empty()) } else {
        match load_stage(rows, pls) {
            Result::Err(o) => Result::Err(o),
            Result::Ok(docs) => match assertions(rows, docs) { Option::Some(o) => Result::Err(o), Option::None => Result::Ok(docs) },
        }
    }
}

// qsha = lowercase hex sha256 of the query bytes, computed by the shell (digest law).
pub open spec fn answer_output(mpath: Seq<u8>, manifest: Src, pls: Seq<Src>, pys: Seq<Src>, query: Src, qsha: Seq<u8>) -> Out {
    match manifest_stage(mpath, manifest, pls, pys) {
        Result::Err(o) => o,
        Result::Ok(rows) => match custody(query) {
            Result::Err(o) => o,
            Result::Ok((q, arows)) => match composition(rows, pls) {
                Result::Err(o) => o,
                Result::Ok(docs) => match answer_result(db_of(docs), q, arows) {
                    Result::Err(o) => o,
                    Result::Ok(result) => ok(print_answers(AnswersFile { qid: q.qid, qsha, result })),
                },
            },
        },
    }
}

} // verus!
