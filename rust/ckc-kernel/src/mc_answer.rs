use crate::k2_engine::{EBodyItem, EClause};
#[cfg(verus_keep_ghost)]
use crate::k2_engine::{root_terms, roots_valid};
use crate::k2_term::{ENode, ENodeKind, ETermArena};
use crate::mc_model::EMcModel;
use ckc_spec::model::{EMcItem, Question};
use ckc_spec::term::Term;
use ckc_spec::v1text::{BodyItem, DocClause};
#[cfg(verus_keep_ghost)]
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

// Contract mc D3 + D7 MC-T3: a question = the clause `sol([Row…]) :- Goals` over the checked model;
// its answers = one DFS over the goals by joins over M (no negation: custody admits schema
// literals + built-ins alone).
// --- custody facts ---
pub proof fn custody_walk(
    src: ckc_spec::replay::Src,
    qf: ckc_spec::v1text::QueryFile,
    rows: Seq<Term>,
)
    requires
        ckc_spec::answers::custody(src) == Result::<
            (ckc_spec::v1text::QueryFile, Seq<Term>),
            ckc_spec::replay::Out,
        >::Ok((qf, rows)),
    ensures
        ckc_spec::answers::goal_walk(qf.goal) is None,
{
    match src {
        ckc_spec::replay::Src::Bytes(b) => {
            match ckc_spec::replay::the_v1(b) {
                ckc_spec::v1text::V1File::Query(q) => {
                    assert(q == qf);
                },
                _ => {},
            }
        },
        _ => {},
    }
}

// Custody's goal walk admits compound leaves alone (no variable, no foreign leaf).
pub proof fn walk_comp(g: Term)
    requires
        ckc_spec::answers::goal_walk(g) is None,
    ensures
        forall|i: int|
            0 <= i < ckc_spec::model::conj_goals(g).len() ==> (
            #[trigger] ckc_spec::model::conj_goals(g)[i]) is Comp,
    decreases g,
{
    match g {
        Term::Comp(f, a) => {
            if f == ckc_spec::engine::comma_name() && a.len() == 2 {
                walk_comp(a[0]);
                walk_comp(a[1]);
                let l = ckc_spec::model::conj_goals(a[0]);
                let r = ckc_spec::model::conj_goals(a[1]);
                assert(ckc_spec::model::conj_goals(g) == l + r);
                assert forall|i: int| 0 <= i < ckc_spec::model::conj_goals(g).len() implies (
                #[trigger] ckc_spec::model::conj_goals(g)[i]) is Comp by {
                    if i < l.len() {
                        assert(ckc_spec::model::conj_goals(g)[i] == l[i]);
                    } else {
                        assert(ckc_spec::model::conj_goals(g)[i] == r[i - l.len()]);
                    }
                }
            }
        },
        _ => {},
    }
}

// --- the question as arena roots ---
// The two conjuncts of a ','/2 goal, if the term at root is one.
fn conj_split(arena: &ETermArena, root: usize) -> (out: Option<(usize, usize)>)
    requires
        crate::k2_term::root_ok(arena, root),
    ensures
        out matches Some((l, r)) ==> l < root && r < root && ckc_spec::model::conj_goals(
            arena@[root as int],
        ) == ckc_spec::model::conj_goals(arena@[l as int]) + ckc_spec::model::conj_goals(
            arena@[r as int],
        ),
        out is None ==> ckc_spec::model::conj_goals(arena@[root as int]) == seq![
            arena@[root as int],
        ],
{
    proof {
        assert(crate::k2_term::node_ok(arena.nodes@, root as int));
    }
    match &arena.nodes[root].kind {
        ENodeKind::Comp { name, child_roots, .. } => {
            let ghost args = crate::k2_term::child_terms(arena.nodes@, child_roots@);
            let comma: &[u8] = b",";
            proof {
                assert(arena@[root as int] == Term::Comp(name@, args));
                reveal_byteslit(b",");
                reveal_strlit(",");
                reveal(ckc_spec::v1text::ascii);
                assert(comma@ == ckc_spec::engine::comma_name());
            }
            if child_roots.len() == 2 && crate::k3_front::bytes_equal(name, comma) {
                let l = child_roots[0];
                let r = child_roots[1];
                proof {
                    assert(args[0] == arena@[l as int]);
                    assert(args[1] == arena@[r as int]);
                }
                Some((l, r))
            } else {
                None
            }
        },
        _ => None,
    }
}

// The ','/2 tree at root, flattened left to right onto out (conj_goals).
fn conj_roots(arena: &ETermArena, root: usize, out: &mut Vec<usize>)
    requires
        crate::k2_term::root_ok(arena, root),
        roots_valid(arena.nodes@, old(out)@),
    ensures
        roots_valid(arena.nodes@, final(out)@),
        root_terms(arena.nodes@, final(out)@) == root_terms(arena.nodes@, old(out)@)
            + ckc_spec::model::conj_goals(arena@[root as int]),
    decreases root,
{
    let ghost nodes = arena.nodes@;
    let ghost o0 = out@;
    match conj_split(arena, root) {
        Some((l, r)) => {
            conj_roots(arena, l, out);
            let ghost o1 = out@;
            conj_roots(arena, r, out);
            proof {
                let gl = ckc_spec::model::conj_goals(arena@[l as int]);
                let gr = ckc_spec::model::conj_goals(arena@[r as int]);
                assert(root_terms(nodes, o0) + gl + gr =~= root_terms(nodes, o0) + (gl + gr));
            }
        },
        None => {
            out.push(root);
            proof {
                assert(root_terms(nodes, out@) =~= root_terms(nodes, o0) + seq![
                    arena@[root as int],
                ]);
            }
        },
    }
}

// The first argument's root (replay::arg(t, 0)), None where it is Nil.
fn first_arg(arena: &ETermArena, root: usize) -> (out: Option<usize>)
    requires
        crate::k2_term::root_ok(arena, root),
    ensures
        out matches Some(a) ==> a < arena.nodes@.len() && arena@[a as int] == ckc_spec::replay::arg(
            arena@[root as int],
            0,
        ),
        out is None ==> ckc_spec::replay::arg(arena@[root as int], 0) == Term::Nil,
{
    proof {
        assert(crate::k2_term::node_ok(arena.nodes@, root as int));
    }
    match &arena.nodes[root].kind {
        ENodeKind::Comp { name, child_roots, .. } => {
            let ghost args = crate::k2_term::child_terms(arena.nodes@, child_roots@);
            proof {
                assert(arena@[root as int] == Term::Comp(name@, args));
            }
            if child_roots.len() > 0 {
                proof {
                    assert(args[0] == arena@[child_roots@[0] as int]);
                }
                Some(child_roots[0])
            } else {
                None
            }
        },
        _ => None,
    }
}

pub struct EQuestion {
    pub goals: Vec<usize>,
    pub proj: Vec<usize>,
    pub clause: EClause,
}

pub open spec fn question_ok(nodes: Seq<ENode>, eq: &EQuestion, q: Question) -> bool {
    &&& roots_valid(nodes, eq.goals@)
    &&& root_terms(nodes, eq.goals@) == q.goals
    &&& roots_valid(nodes, eq.proj@)
    &&& root_terms(nodes, eq.proj@) == q.projection
    &&& crate::k2_engine::clause_valid(nodes, &eq.clause)
    &&& crate::k2_engine::clause_view(nodes, &eq.clause) == ckc_spec::model::question_clause(q)
}

pub proof fn question_ok_prefix(before: Seq<ENode>, after: Seq<ENode>, eq: &EQuestion, q: Question)
    requires
        before.is_prefix_of(after),
        question_ok(before, eq, q),
    ensures
        question_ok(after, eq, q),
{
    crate::k2_engine::roots_models_prefix(before, after, eq.goals@);
    crate::k2_engine::roots_models_prefix(before, after, eq.proj@);
    crate::k2_engine::clause_models_prefix(before, after, &eq.clause);
}

// question_of(custody's query, its rows) as roots: goals, projection, the clause `sol(…) :- goals`.
pub fn build_question(arena: &mut ETermArena, query: &crate::k2_answers::EQuery) -> (out: EQuestion)
    requires
        crate::k2_term::arena_ok(old(arena)),
        crate::k2_answers::query_ok(old(arena).nodes@, query),
    ensures
        crate::k2_term::arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        question_ok(
            final(arena).nodes@,
            &out,
            ckc_spec::model::question_of(query.file@, root_terms(old(arena).nodes@, query.rows@)),
        ),
{
    let ghost a0 = arena.nodes@;
    let ghost rows = root_terms(a0, query.rows@);
    let ghost q = ckc_spec::model::question_of(query.file@, rows);
    let mut goals: Vec<usize> = Vec::new();
    proof {
        assert(root_terms(a0, goals@) =~= Seq::<Term>::empty());
    }
    conj_roots(arena, query.goal, &mut goals);
    proof {
        assert(root_terms(a0, goals@) =~= q.goals);
        assert(a0.is_prefix_of(a0)) by {
            assert(a0.take(a0.len() as int) =~= a0);
        }
    }
    let mut proj: Vec<usize> = Vec::new();
    let mut i = 0usize;
    while i < query.rows.len()
        invariant
            crate::k2_term::arena_ok(arena),
            a0 == old(arena).nodes@,
            a0.is_prefix_of(arena.nodes@),
            crate::k2_answers::query_ok(a0, query),
            rows == root_terms(a0, query.rows@),
            q == ckc_spec::model::question_of(query.file@, rows),
            roots_valid(arena.nodes@, goals@),
            root_terms(arena.nodes@, goals@) == q.goals,
            i <= query.rows@.len(),
            proj@.len() == i,
            roots_valid(arena.nodes@, proj@),
            forall|k: int|
                0 <= k < i ==> #[trigger] arena.nodes@[proj@[k] as int].term@
                    == ckc_spec::replay::arg(rows[k], 0),
        decreases query.rows.len() - i,
    {
        let ghost before = arena.nodes@;
        let row = query.rows[i];
        proof {
            assert(row < a0.len());
            assert(before[row as int] == a0[row as int]);
            assert(rows[i as int] == a0[row as int].term@);
        }
        let a = match first_arg(arena, row) {
            Some(a) => a,
            None => crate::k2_term::push_nil(arena),
        };
        proof {
            crate::k2_load::prefix_chain(a0, before, arena.nodes@);
            crate::k2_engine::roots_models_prefix(before, arena.nodes@, goals@);
            crate::k2_engine::roots_models_prefix(before, arena.nodes@, proj@);
        }
        proj.push(a);
        proof {
            assert forall|k: int|
                0 <= k < i + 1 implies #[trigger] arena.nodes@[proj@[k] as int].term@
                == ckc_spec::replay::arg(rows[k], 0) by {
                if k < i {
                    assert(arena.nodes@[proj@[k] as int].term@ == before[proj@[k] as int].term@);
                }
            }
        }
        i += 1;
    }
    let ghost a1 = arena.nodes@;
    proof {
        assert(root_terms(a1, proj@) =~= q.projection);
    }
    let list = crate::k2_walk::list_root(arena, &proj);
    let name: &[u8] = b"sol";
    proof {
        reveal_byteslit(b"sol");
        reveal_strlit("sol");
        reveal(ckc_spec::v1text::ascii);
        assert(name@ == ckc_spec::v1text::ascii("sol"@));
    }
    let head = crate::k2_output::comp1(arena, name, list);
    let ghost a2 = arena.nodes@;
    proof {
        crate::k2_load::prefix_chain(a0, a1, a2);
        crate::k2_engine::roots_models_prefix(a1, a2, goals@);
        crate::k2_engine::roots_models_prefix(a1, a2, proj@);
        assert(a2[head as int].term@ == ckc_spec::answers::sol_term(q.projection));
    }
    let mut body: Vec<EBodyItem> = Vec::new();
    let mut k = 0usize;
    while k < goals.len()
        invariant
            k <= goals@.len(),
            body@.len() == k,
            forall|j: int|
                0 <= j < k ==> #[trigger] body@[j] == (EBodyItem::Pos { root: goals@[j] }),
        decreases goals.len() - k,
    {
        body.push(EBodyItem::Pos { root: goals[k] });
        k += 1;
    }
    let clause = EClause { head, body };
    proof {
        assert forall|j: int|
            0 <= j < clause.body@.len() implies #[trigger] crate::k2_engine::body_item_valid(
            a2,
            &clause.body@[j],
        ) by {
            assert(clause.body@[j] == (EBodyItem::Pos { root: goals@[j] }));
        }
        assert(crate::k2_engine::clause_valid(a2, &clause));
        assert(crate::k2_engine::body_items_view(a2, clause.body@) =~= q.goals.map_values(
            |g: Term| BodyItem::Pos(g),
        )) by {
            assert forall|j: int|
                0 <= j < clause.body@.len() implies crate::k2_engine::body_item_view(
                a2,
                &clause.body@[j],
            ) == BodyItem::Pos(q.goals[j]) by {
                assert(clause.body@[j] == (EBodyItem::Pos { root: goals@[j] }));
                assert(root_terms(a2, goals@)[j] == a2[goals@[j] as int].term@);
            }
        }
        assert(crate::k2_engine::clause_view(a2, &clause) == ckc_spec::model::question_clause(q));
    }
    EQuestion { goals, proj, clause }
}

// --- question admission (contract mc D3; D9 items 1-based) ---
proof fn list_occurs(ps: Seq<Term>, x: nat)
    ensures
        ckc_spec::engine::occurs(x, ckc_spec::engine::list_term(ps)) == exists|r: int|
            0 <= r < ps.len() && #[trigger] ckc_spec::engine::occurs(x, ps[r]),
    decreases ps.len(),
{
    if ps.len() > 0 {
        let rest = ps.drop_first();
        list_occurs(rest, x);
        let lt = ckc_spec::engine::list_term(rest);
        let args = seq![ps[0], lt];
        assert(ckc_spec::engine::list_term(ps) == Term::Comp(ckc_spec::v1text::cons_name(), args));
        crate::mc_safe::occurs_all_any(x, args);
        if ckc_spec::engine::occurs(x, ckc_spec::engine::list_term(ps))
            && !ckc_spec::engine::occurs(x, ps[0]) {
            let k = choose|k: int|
                0 <= k < args.len() && #[trigger] ckc_spec::engine::occurs(x, args[k]);
            assert(k == 1);
            let r = choose|r: int|
                0 <= r < rest.len() && #[trigger] ckc_spec::engine::occurs(x, rest[r]);
            assert(rest[r] == ps[r + 1]);
        }
        if exists|r: int| 0 <= r < ps.len() && #[trigger] ckc_spec::engine::occurs(x, ps[r]) {
            let r = choose|r: int|
                0 <= r < ps.len() && #[trigger] ckc_spec::engine::occurs(x, ps[r]);
            if r == 0 {
                assert(ckc_spec::engine::occurs(x, args[0]));
            } else {
                assert(rest[r - 1] == ps[r]);
                assert(ckc_spec::engine::occurs(x, args[1]));
            }
        }
    }
}

// The answer head's variables = the projection's.
pub proof fn sol_vars(ps: Seq<Term>, x: nat)
    ensures
        ckc_spec::model::vars(ckc_spec::answers::sol_term(ps)).contains(x)
            == ckc_spec::model::vars_all(ps).contains(x),
{
    let l = ckc_spec::engine::list_term(ps);
    let args = seq![l];
    crate::mc_safe::vars_occurs(ckc_spec::answers::sol_term(ps), x);
    crate::mc_safe::occurs_all_any(x, args);
    list_occurs(ps, x);
    crate::mc_safe::vars_all_any(ps, x);
    if ckc_spec::engine::occurs(x, ckc_spec::answers::sol_term(ps)) {
        let k = choose|k: int|
            0 <= k < args.len() && #[trigger] ckc_spec::engine::occurs(x, args[k]);
        assert(k == 0);
    }
    if ckc_spec::engine::occurs(x, l) {
        assert(ckc_spec::engine::occurs(x, args[0]));
    }
}

// The first goal that is not left-to-right safe, else the first answer row whose variable no goal
// binds; Ok ⇒ safe_question + the question clause's safe_clause.
pub fn question_check(
    arena: &ETermArena,
    eq: &EQuestion,
    Ghost(q): Ghost<Question>,
    n: usize,
) -> (out: Result<(), EMcItem>)
    requires
        crate::k2_term::arena_ok(arena),
        question_ok(arena.nodes@, eq, q),
        crate::mc_safe::vars_below(ckc_spec::model::question_clause(q), n as nat),
    ensures
        out is Ok ==> ckc_spec::model::safe_question(q) && ckc_spec::model::safe_clause(
            ckc_spec::model::question_clause(q),
        ),
{
    let ghost qc = ckc_spec::model::question_clause(q);
    let mut bv = crate::mc_safe::none_flags(n);
    let ghost mut b: Set<nat> = Set::empty();
    let mut binders = crate::mc_safe::none_flags(n);
    let ghost mut bs: Set<nat> = Set::empty();
    let mut j = 0usize;
    proof {
        assert(q.goals.skip(0) =~= q.goals);
    }
    while j < eq.goals.len()
        invariant
            crate::k2_term::arena_ok(arena),
            question_ok(arena.nodes@, eq, q),
            qc == ckc_spec::model::question_clause(q),
            crate::mc_safe::vars_below(qc, n as nat),
            j <= eq.goals@.len(),
            bv@.len() == n,
            crate::mc_safe::set_of(bv@, b),
            binders@.len() == n,
            crate::mc_safe::set_of(binders@, bs),
            forall|x: nat| #[trigger]
                bs.contains(x) == exists|q2: int|
                    0 <= q2 < j && #[trigger] ckc_spec::model::item_binds(qc.body[q2]).contains(x),
            ckc_spec::model::items_safe(qc, 0, Set::empty()) == ckc_spec::model::items_safe(
                qc,
                j as nat,
                b,
            ),
            ckc_spec::model::conj_safe(q.goals, Set::empty()) == ckc_spec::model::conj_safe(
                q.goals.skip(j as int),
                b,
            ),
        decreases eq.goals.len() - j,
    {
        let root = eq.goals[j];
        let ghost l = arena@[root as int];
        proof {
            assert(root_terms(arena.nodes@, eq.goals@)[j as int] == l);
            assert(qc.body[j as int] == BodyItem::Pos(l));
            assert forall|x: nat| #[trigger] ckc_spec::model::vars(l).contains(x) implies x < n by {
                assert(ckc_spec::model::item_vars(qc.body[j as int]).contains(x));
            }
            assert(q.goals.skip(j as int)[0] == l);
            assert(q.goals.skip(j as int).drop_first() =~= q.goals.skip(j as int + 1));
        }
        let next = match crate::mc_safe::pos_step(arena, root, &bv, Ghost(b), n) {
            None => return Err(EMcItem::Goal(j + 1)),
            Some(next) => next,
        };
        let ghost lb = ckc_spec::model::binds(l);
        proof {
            assert(ckc_spec::model::item_binds(qc.body[j as int]) == lb);
            assert forall|x: nat| #[trigger] lb.contains(x) implies x < n by {
                assert(b.union(lb).contains(x));
            }
        }
        let lflags = crate::mc_safe::pos_binds(arena, root, n);
        binders = crate::mc_safe::union_flags(&binders, &lflags, Ghost(bs), Ghost(lb));
        proof {
            bs = bs.union(lb);
            assert forall|x: nat| #[trigger]
                bs.contains(x) == exists|q2: int|
                    0 <= q2 < j + 1 && #[trigger] ckc_spec::model::item_binds(qc.body[q2]).contains(
                        x,
                    ) by {
                if lb.contains(x) {
                    assert(ckc_spec::model::item_binds(qc.body[j as int]).contains(x));
                }
            }
            b = b.union(lb);
        }
        bv = next;
        j += 1;
    }
    proof {
        assert forall|x: nat| #[trigger]
            bs.contains(x) == ckc_spec::model::binders(qc.body).contains(x) by {
            crate::mc_safe::binders_any(qc.body, x);
        }
        assert(bs =~= ckc_spec::model::binders(qc.body));
        assert(q.goals.skip(j as int).len() == 0);
    }
    let mut r = 0usize;
    while r < eq.proj.len()
        invariant
            crate::k2_term::arena_ok(arena),
            question_ok(arena.nodes@, eq, q),
            qc == ckc_spec::model::question_clause(q),
            crate::mc_safe::vars_below(qc, n as nat),
            binders@.len() == n,
            crate::mc_safe::set_of(binders@, bs),
            bs == ckc_spec::model::binders(qc.body),
            ckc_spec::model::items_safe(qc, 0, Set::empty()),
            ckc_spec::model::conj_safe(q.goals, Set::empty()),
            r <= eq.proj@.len(),
            forall|r2: int, x: nat|
                0 <= r2 < r && #[trigger] ckc_spec::engine::occurs(x, q.projection[r2])
                    ==> bs.contains(x),
        decreases eq.proj.len() - r,
    {
        let p = eq.proj[r];
        proof {
            assert(root_terms(arena.nodes@, eq.proj@)[r as int] == arena@[p as int]);
            assert(arena@[p as int] == q.projection[r as int]);
            assert forall|x: nat| #[trigger]
                ckc_spec::model::vars(arena@[p as int]).contains(x) implies x < n by {
                crate::mc_safe::vars_occurs(arena@[p as int], x);
                crate::mc_safe::vars_all_any(q.projection, x);
                sol_vars(q.projection, x);
            }
        }
        let pf = crate::mc_safe::term_flags(arena, p, n);
        if !crate::mc_safe::subset_flags(
            &pf,
            &binders,
            Ghost(ckc_spec::model::vars(arena@[p as int])),
            Ghost(bs),
        ) {
            return Err(EMcItem::Row(r + 1));
        }
        proof {
            assert forall|r2: int, x: nat|
                0 <= r2 < r + 1 && #[trigger] ckc_spec::engine::occurs(
                    x,
                    q.projection[r2],
                ) implies bs.contains(x) by {
                if r2 == r {
                    crate::mc_safe::vars_occurs(arena@[p as int], x);
                }
            }
        }
        r += 1;
    }
    proof {
        assert(ckc_spec::model::vars_all(q.projection).subset_of(ckc_spec::model::binders(qc.body)))
            by {
            assert forall|x: nat|
                ckc_spec::model::vars_all(q.projection).contains(
                    x,
                ) implies ckc_spec::model::binders(qc.body).contains(x) by {
                crate::mc_safe::vars_all_any(q.projection, x);
            }
        }
        assert(ckc_spec::model::vars(qc.head).subset_of(ckc_spec::model::binders(qc.body))) by {
            assert forall|x: nat|
                ckc_spec::model::vars(qc.head).contains(x) implies ckc_spec::model::binders(
                qc.body,
            ).contains(x) by {
                sol_vars(q.projection, x);
                crate::mc_safe::vars_all_any(q.projection, x);
            }
        }
    }
    Ok(())
}

// --- answers: one DFS over the goals (contract mc D7 MC-T3) ---
// Every body item a positive compound literal.
pub open spec fn pos_comp(c: DocClause) -> bool {
    forall|q: int|
        0 <= q < c.body.len() ==> (#[trigger] c.body[q]) is Pos && c.body[q]->Pos_0 is Comp
}

// t = the head of a valuation of c whose body holds in mm.
pub open spec fn answer_of(mm: ISet<Term>, c: DocClause, t: Term) -> bool {
    exists|th: Map<nat, Term>| #[trigger]
        ckc_spec::model::valuation(c, th) && ckc_spec::model::body_holds(mm, mm, c, th) && t
            == ckc_spec::model::apply(c.head, th)
}

// Every collected root: a ground answer.
pub open spec fn outs_ok(nodes: Seq<ENode>, out: Seq<usize>, mm: ISet<Term>, c: DocClause) -> bool {
    &&& roots_valid(nodes, out)
    &&& forall|i: int|
        0 <= i < out.len() ==> {
            &&& ckc_spec::term::ground(#[trigger] nodes[out[i] as int].term@)
            &&& answer_of(mm, c, nodes[out[i] as int].term@)
        }
}

pub open spec fn collected(nodes: Seq<ENode>, out: Seq<usize>, t: Term) -> bool {
    exists|i: int| 0 <= i < out.len() && #[trigger] nodes[out[i] as int].term@ == t
}

// The head of every valuation extending th that satisfies items j.. is collected.
pub open spec fn heads_collected(
    nodes: Seq<ENode>,
    out: Seq<usize>,
    mm: ISet<Term>,
    c: DocClause,
    j: int,
    th: Map<nat, Term>,
) -> bool {
    forall|t3: Map<nat, Term>|
        ckc_spec::model::valuation(c, t3) && crate::mc_subst::extends(t3, th)
            && crate::mc_closure::items_from(mm, c, j, t3) ==> collected(
            nodes,
            out,
            ckc_spec::model::apply(c.head, t3),
        )
}

proof fn seq_prefix_refl<A>(a: Seq<A>)
    ensures
        a.is_prefix_of(a),
{
    assert(a.take(a.len() as int) =~= a);
}

proof fn seq_prefix_chain<A>(a: Seq<A>, b: Seq<A>, c: Seq<A>)
    requires
        a.is_prefix_of(b),
        b.is_prefix_of(c),
    ensures
        a.is_prefix_of(c),
{
    assert_seqs_equal!(a == c.take(a.len() as int), i => {
        assert(a[i] == b[i]);
        assert(b[i] == c[i]);
    });
}

// What was collected stays collected as the arena + the output vector grow.
proof fn collected_grow(n1: Seq<ENode>, o1: Seq<usize>, n2: Seq<ENode>, o2: Seq<usize>, t: Term)
    requires
        n1.is_prefix_of(n2),
        o1.is_prefix_of(o2),
        roots_valid(n1, o1),
        collected(n1, o1, t),
    ensures
        collected(n2, o2, t),
{
    let i = choose|i: int| 0 <= i < o1.len() && #[trigger] n1[o1[i] as int].term@ == t;
    assert(o2[i] == o1[i]);
    assert(n2[o1[i] as int] == n1[o1[i] as int]);
    assert(n2[o2[i] as int].term@ == t);
}

proof fn outs_ok_prefix(n1: Seq<ENode>, n2: Seq<ENode>, o: Seq<usize>, mm: ISet<Term>, c: DocClause)
    requires
        n1.is_prefix_of(n2),
        outs_ok(n1, o, mm, c),
    ensures
        outs_ok(n2, o, mm, c),
{
    crate::k2_engine::roots_models_prefix(n1, n2, o);
}

// A goal holding under th holds under every extension of th (its instance is already ground).
proof fn goal_ok_extend(mm: ISet<Term>, g: Term, th: Map<nat, Term>, th2: Map<nat, Term>)
    requires
        crate::mc_model::goal_ok(mm, g, th),
        crate::mc_subst::extends(th2, th),
    ensures
        crate::mc_model::goal_ok(mm, g, th2),
{
    assert forall|x: nat| #[trigger] ckc_spec::engine::occurs(x, g) implies (th.contains_key(x)
        == th2.contains_key(x) && (th.contains_key(x) ==> th[x] == th2[x])) by {
        crate::mc_subst::ground_apply_binds(g, th, x);
    }
    crate::mc_subst::apply_agree(g, th, th2);
}

// Sound + complete: every root it appends is an answer; every answer's head is collected.
pub fn collect_items(
    arena: &mut ETermArena,
    clause: &EClause,
    Ghost(cc): Ghost<DocClause>,
    n: usize,
    j: usize,
    sub: &Vec<Option<usize>>,
    Ghost(th): Ghost<Map<nat, Term>>,
    m: &EMcModel,
    out: &mut Vec<usize>,
)
    requires
        crate::k2_term::arena_ok(old(arena)),
        crate::k2_engine::clause_valid(old(arena).nodes@, clause),
        cc == crate::k2_engine::clause_view(old(arena).nodes@, clause),
        pos_comp(cc),
        ckc_spec::model::safe_clause(cc),
        crate::mc_safe::vars_below(cc, n as nat),
        j <= clause.body@.len(),
        sub@.len() == n,
        crate::mc_subst::sub_ok(old(arena).nodes@, sub@, th),
        th.dom() == crate::mc_lemma::binds_upto(cc, j as int),
        crate::mc_model::model_ok(old(arena).nodes@, m),
        forall|q: int|
            0 <= q < j ==> #[trigger] crate::mc_model::goal_ok(
                m.ms@.to_iset(),
                cc.body[q]->Pos_0,
                th,
            ),
        outs_ok(old(arena).nodes@, old(out)@, m.ms@.to_iset(), cc),
    ensures
        crate::k2_term::arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        old(out)@.is_prefix_of(final(out)@),
        outs_ok(final(arena).nodes@, final(out)@, m.ms@.to_iset(), cc),
        heads_collected(final(arena).nodes@, final(out)@, m.ms@.to_iset(), cc, j as int, th),
    decreases clause.body@.len() - j,
{
    let ghost origin = arena.nodes@;
    let ghost o0 = out@;
    let ghost mm = m.ms@.to_iset();
    proof {
        seq_prefix_refl(origin);
        seq_prefix_refl(o0);
        crate::mc_model::sub_vals_wf(arena, sub@, th);
    }
    if j == clause.body.len() {
        proof {
            assert forall|x: nat| #[trigger]
                ckc_spec::engine::occurs(
                    x,
                    origin[clause.head as int].term@,
                ) implies th.contains_key(x) by {
                crate::mc_safe::vars_occurs(cc.head, x);
                crate::mc_closure::binders_are_bound(cc, x);
            }
        }
        let inst = crate::mc_subst::instantiate(arena, clause.head, sub, Ghost(th));
        proof {
            crate::mc_subst::apply_ground(cc.head, th);
            assert forall|x: nat| #[trigger]
                th.contains_key(x) <==> ckc_spec::model::nonlocal(cc, x) by {
                if th.contains_key(x) {
                    crate::mc_lemma::binds_upto_nonlocal(cc, j as int, x);
                }
                if ckc_spec::model::nonlocal(cc, x) {
                    if ckc_spec::model::vars(cc.head).contains(x) {
                        crate::mc_closure::binders_are_bound(cc, x);
                    } else {
                        let q = choose|q: int|
                            0 <= q < cc.body.len() && #[trigger] ckc_spec::model::item_vars(
                                cc.body[q],
                            ).contains(x) && (cc.body[q] is Pos || ckc_spec::model::outside(
                                cc,
                                q,
                                x,
                            ));
                        let l = cc.body[q]->Pos_0;
                        assert(cc.body[q] is Pos);
                        assert(crate::mc_model::goal_ok(mm, l, th));
                        crate::mc_safe::vars_occurs(l, x);
                        crate::mc_subst::ground_apply_binds(l, th, x);
                    }
                }
            }
            assert(ckc_spec::model::valuation(cc, th));
            assert(ckc_spec::model::body_holds(mm, mm, cc, th)) by {
                assert forall|q: int|
                    0 <= q < cc.body.len() implies #[trigger] ckc_spec::model::item_holds(
                    mm,
                    mm,
                    cc.body[q],
                    th,
                ) by {
                    assert(crate::mc_model::goal_ok(mm, cc.body[q]->Pos_0, th));
                }
            }
            assert(answer_of(mm, cc, arena@[inst as int]));
            outs_ok_prefix(origin, arena.nodes@, o0, mm, cc);
        }
        out.push(inst);
        proof {
            assert(out@.take(o0.len() as int) =~= o0);
            assert forall|i: int| 0 <= i < out@.len() implies {
                &&& ckc_spec::term::ground(#[trigger] arena.nodes@[out@[i] as int].term@)
                &&& answer_of(mm, cc, arena.nodes@[out@[i] as int].term@)
            } by {
                if i < o0.len() {
                    assert(out@[i] == o0[i]);
                } else {
                    assert(out@[i] == inst);
                }
            }
            assert forall|t3: Map<nat, Term>|
                ckc_spec::model::valuation(cc, t3) && crate::mc_subst::extends(t3, th)
                    && crate::mc_closure::items_from(mm, cc, j as int, t3) implies collected(
                arena.nodes@,
                out@,
                ckc_spec::model::apply(cc.head, t3),
            ) by {
                assert forall|x: nat| #[trigger] ckc_spec::engine::occurs(x, cc.head) implies (
                t3.contains_key(x) == th.contains_key(x) && (t3.contains_key(x) ==> t3[x]
                    == th[x])) by {
                    crate::mc_safe::vars_occurs(cc.head, x);
                    crate::mc_closure::binders_are_bound(cc, x);
                }
                crate::mc_subst::apply_agree(cc.head, t3, th);
                assert(arena.nodes@[out@[o0.len() as int] as int].term@ == ckc_spec::model::apply(
                    cc.head,
                    t3,
                ));
            }
        }
        return;
    }
    let j1 = j + 1;
    proof {
        assert(crate::k2_engine::body_item_valid(origin, &clause.body@[j as int]));
        assert(cc.body[j as int] == crate::k2_engine::body_item_view(
            origin,
            &clause.body@[j as int],
        ));
        crate::mc_lemma::items_safe_at(cc, j as nat);
        assert(crate::mc_lemma::binds_upto(cc, j as int + 1) == crate::mc_lemma::binds_upto(
            cc,
            j as int,
        ).union(ckc_spec::model::item_binds(cc.body[j as int])));
    }
    let root = match &clause.body[j] {
        EBodyItem::Pos { root } => *root,
        EBodyItem::Naf { .. } => {
            proof {
                assert(cc.body[j as int] is Pos);
            }
            return;
        },
    };
    let ghost l = origin[root as int].term@;
    proof {
        assert(cc.body[j as int] == BodyItem::Pos(l));
        assert(l is Comp);
        assert(crate::k2_term::node_ok(origin, root as int));
        assert(ckc_spec::term::wf_term(l));
        assert forall|x: nat| #[trigger]
            ckc_spec::engine::occurs(x, origin[root as int].term@) implies x < sub@.len() by {
            crate::mc_safe::vars_occurs(l, x);
            assert(ckc_spec::model::item_vars(cc.body[j as int]).contains(x));
        }
    }
    let (builtin, _kids, _k_in) = crate::mc_safe::literal_parts(arena, root);
    if builtin {
        proof {
            assert(ckc_spec::model::lit_safe(l, th.dom()));
        }
        match crate::mc_join::builtin_step(arena, root, sub, Ghost(th), Ghost(mm)) {
            None => {
                proof {
                    outs_ok_prefix(origin, arena.nodes@, o0, mm, cc);
                    assert forall|t3: Map<nat, Term>|
                        ckc_spec::model::valuation(cc, t3) && crate::mc_subst::extends(t3, th)
                            && crate::mc_closure::items_from(
                            mm,
                            cc,
                            j as int,
                            t3,
                        ) implies collected(
                        arena.nodes@,
                        out@,
                        ckc_spec::model::apply(cc.head, t3),
                    ) by {
                        assert(ckc_spec::model::item_holds(mm, mm, cc.body[j as int], t3));
                        crate::mc_closure::pos_goal_ok(cc, j as int, t3, mm);
                    }
                }
                return;
            },
            Some((s2, th2)) => {
                let ghost after = arena.nodes@;
                proof {
                    crate::mc_model::model_ok_prefix(origin, after, m);
                    crate::k2_engine::clause_models_prefix(origin, after, clause);
                    outs_ok_prefix(origin, after, o0, mm, cc);
                    assert forall|q: int|
                        0 <= q < j + 1 implies #[trigger] crate::mc_model::goal_ok(
                        mm,
                        cc.body[q]->Pos_0,
                        th2@,
                    ) by {
                        if q < j {
                            goal_ok_extend(mm, cc.body[q]->Pos_0, th, th2@);
                        }
                    }
                }
                collect_items(arena, clause, Ghost(cc), n, j1, &s2, th2, m, out);
                proof {
                    crate::k2_load::prefix_chain(origin, after, arena.nodes@);
                    assert forall|t3: Map<nat, Term>|
                        ckc_spec::model::valuation(cc, t3) && crate::mc_subst::extends(t3, th)
                            && crate::mc_closure::items_from(
                            mm,
                            cc,
                            j as int,
                            t3,
                        ) implies collected(
                        arena.nodes@,
                        out@,
                        ckc_spec::model::apply(cc.head, t3),
                    ) by {
                        assert(ckc_spec::model::item_holds(mm, mm, cc.body[j as int], t3));
                        crate::mc_closure::pos_goal_ok(cc, j as int, t3, mm);
                        assert(crate::mc_subst::extends(t3, th2@));
                        assert(crate::mc_closure::items_from(mm, cc, j as int + 1, t3));
                    }
                }
                return;
            },
        }
    }
    let mut i = 0usize;
    while i < m.facts.len()
        invariant
            crate::k2_term::arena_ok(arena),
            origin == old(arena).nodes@,
            o0 == old(out)@,
            origin.is_prefix_of(arena.nodes@),
            o0.is_prefix_of(out@),
            crate::k2_engine::clause_valid(origin, clause),
            cc == crate::k2_engine::clause_view(origin, clause),
            pos_comp(cc),
            ckc_spec::model::safe_clause(cc),
            crate::mc_safe::vars_below(cc, n as nat),
            j < clause.body@.len(),
            j1 == j + 1,
            cc.body[j as int] == BodyItem::Pos(l),
            l == origin[root as int].term@,
            l is Comp,
            ckc_spec::term::wf_term(l),
            !ckc_spec::model::builtin(l),
            root < origin.len(),
            sub@.len() == n,
            crate::mc_subst::sub_ok(arena.nodes@, sub@, th),
            th.dom() == crate::mc_lemma::binds_upto(cc, j as int),
            crate::mc_lemma::binds_upto(cc, j as int + 1) == crate::mc_lemma::binds_upto(
                cc,
                j as int,
            ).union(ckc_spec::model::binds(l)),
            crate::mc_model::model_ok(arena.nodes@, m),
            mm == m.ms@.to_iset(),
            forall|q: int|
                0 <= q < j ==> #[trigger] crate::mc_model::goal_ok(mm, cc.body[q]->Pos_0, th),
            outs_ok(arena.nodes@, out@, mm, cc),
            i <= m.facts@.len(),
            forall|t3: Map<nat, Term>|
                ckc_spec::model::valuation(cc, t3) && crate::mc_subst::extends(t3, th)
                    && crate::mc_closure::items_from(mm, cc, j as int, t3) && (exists|i2: int|
                    0 <= i2 < i && #[trigger] m.ms@[i2] == ckc_spec::model::apply(l, t3))
                    ==> collected(arena.nodes@, out@, ckc_spec::model::apply(cc.head, t3)),
        decreases m.facts.len() - i,
    {
        let ghost before = arena.nodes@;
        let ghost ob = out@;
        let mut s2 = sub.clone();
        proof {
            assert(before[root as int] == origin[root as int]);
            assert forall|x: nat| #[trigger]
                ckc_spec::engine::occurs(x, before[root as int].term@) implies x < s2@.len() by {
                crate::mc_safe::vars_occurs(l, x);
                assert(ckc_spec::model::item_vars(cc.body[j as int]).contains(x));
            }
            assert(m.facts@[i as int] < before.len());
        }
        let (ok, th2) = crate::mc_subst::match_term(arena, root, m.facts[i], &mut s2, Ghost(th));
        if ok {
            proof {
                assert(th2@.dom() =~= crate::mc_lemma::binds_upto(cc, j as int + 1)) by {
                    assert forall|x: nat|
                        th2@.dom().contains(x) == crate::mc_lemma::binds_upto(
                            cc,
                            j as int + 1,
                        ).contains(x) by {
                        crate::mc_safe::vars_occurs(l, x);
                    }
                }
                crate::k2_engine::clause_models_prefix(origin, before, clause);
                assert(crate::k2_engine::clause_valid(before, clause));
                assert(cc == crate::k2_engine::clause_view(before, clause));
                let f = m.ms@[i as int];
                assert(before[m.facts@[i as int] as int].term@ == f);
                assert(ckc_spec::model::apply(l, th2@) == f);
                assert(crate::k2_term::node_ok(before, m.facts@[i as int] as int));
                assert(ckc_spec::term::wf_term(f));
                crate::mc_subst::builtin_apply_iff(l, th2@);
                crate::mc_model::iset_of_seq(m.ms@, f);
                assert(m.ms@.contains(f));
                assert(crate::mc_model::goal_ok(mm, l, th2@));
                assert forall|q: int| 0 <= q < j + 1 implies #[trigger] crate::mc_model::goal_ok(
                    mm,
                    cc.body[q]->Pos_0,
                    th2@,
                ) by {
                    if q < j {
                        goal_ok_extend(mm, cc.body[q]->Pos_0, th, th2@);
                    }
                }
            }
            collect_items(arena, clause, Ghost(cc), n, j1, &s2, th2, m, out);
            proof {
                crate::k2_load::prefix_chain(origin, before, arena.nodes@);
                seq_prefix_chain(o0, ob, out@);
                crate::mc_subst::sub_ok_prefix(before, arena.nodes@, sub@, th);
                crate::mc_model::model_ok_prefix(before, arena.nodes@, m);
                assert forall|t3: Map<nat, Term>|
                    ckc_spec::model::valuation(cc, t3) && crate::mc_subst::extends(t3, th)
                        && crate::mc_closure::items_from(mm, cc, j as int, t3) && (exists|i2: int|
                        0 <= i2 < i + 1 && #[trigger] m.ms@[i2] == ckc_spec::model::apply(
                            l,
                            t3,
                        )) implies collected(
                    arena.nodes@,
                    out@,
                    ckc_spec::model::apply(cc.head, t3),
                ) by {
                    if m.ms@[i as int] == ckc_spec::model::apply(l, t3) {
                        assert(before[m.facts@[i as int] as int].term@ == m.ms@[i as int]);
                        assert(crate::mc_subst::extends(t3, th2@));
                        assert(crate::mc_closure::items_from(mm, cc, j as int + 1, t3));
                    } else {
                        collected_grow(
                            before,
                            ob,
                            arena.nodes@,
                            out@,
                            ckc_spec::model::apply(cc.head, t3),
                        );
                    }
                }
            }
        } else {
            proof {
                assert forall|t3: Map<nat, Term>|
                    ckc_spec::model::valuation(cc, t3) && crate::mc_subst::extends(t3, th)
                        && crate::mc_closure::items_from(mm, cc, j as int, t3) && (exists|i2: int|
                        0 <= i2 < i + 1 && #[trigger] m.ms@[i2] == ckc_spec::model::apply(
                            l,
                            t3,
                        )) implies collected(
                    arena.nodes@,
                    out@,
                    ckc_spec::model::apply(cc.head, t3),
                ) by {
                    assert(before[m.facts@[i as int] as int].term@ == m.ms@[i as int]);
                }
            }
        }
        i += 1;
    }
    proof {
        assert forall|t3: Map<nat, Term>|
            ckc_spec::model::valuation(cc, t3) && crate::mc_subst::extends(t3, th)
                && crate::mc_closure::items_from(mm, cc, j as int, t3) implies collected(
            arena.nodes@,
            out@,
            ckc_spec::model::apply(cc.head, t3),
        ) by {
            assert(ckc_spec::model::item_holds(mm, mm, cc.body[j as int], t3));
            crate::mc_subst::builtin_apply_iff(l, t3);
            crate::mc_model::iset_of_seq(m.ms@, ckc_spec::model::apply(l, t3));
            let i2 = choose|i2: int|
                0 <= i2 < m.ms@.len() && m.ms@[i2] == ckc_spec::model::apply(l, t3);
        }
    }
}

// --- the result term (MC-T3: the answers-file vocabulary) ---
pub fn result_root(arena: &mut ETermArena, sorted: &Vec<usize>, rows: bool) -> (out: usize)
    requires
        crate::k2_term::arena_ok(old(arena)),
        roots_valid(old(arena).nodes@, sorted@),
    ensures
        crate::k2_term::arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        crate::k2_term::root_ok(final(arena), out),
        final(arena)@[out as int] == if rows {
            Term::Comp(
                ckc_spec::v1text::ascii("solutions"@),
                seq![ckc_spec::engine::list_term(root_terms(old(arena).nodes@, sorted@))],
            )
        } else if sorted@.len() > 0 {
            ckc_spec::replay::atom("yes"@)
        } else {
            Term::Comp(
                ckc_spec::v1text::ascii("no"@),
                seq![ckc_spec::replay::atom("finite_failure"@)],
            )
        },
{
    if rows {
        let list = crate::k2_walk::list_root(arena, sorted);
        let name: &[u8] = b"solutions";
        proof {
            reveal_byteslit(b"solutions");
            reveal_strlit("solutions");
            reveal(ckc_spec::v1text::ascii);
            assert(name@ == ckc_spec::v1text::ascii("solutions"@));
        }
        crate::k2_output::comp1(arena, name, list)
    } else if sorted.len() > 0 {
        let name: &[u8] = b"yes";
        proof {
            reveal_byteslit(b"yes");
            reveal_strlit("yes");
            reveal(ckc_spec::v1text::ascii);
            assert(name@ == ckc_spec::v1text::ascii("yes"@));
        }
        crate::k2_output::atom_root(arena, name)
    } else {
        let name: &[u8] = b"no";
        let why: &[u8] = b"finite_failure";
        proof {
            reveal_byteslit(b"no");
            reveal_strlit("no");
            reveal_byteslit(b"finite_failure");
            reveal_strlit("finite_failure");
            reveal(ckc_spec::v1text::ascii);
            assert(name@ == ckc_spec::v1text::ascii("no"@));
            assert(why@ == ckc_spec::v1text::ascii("finite_failure"@));
        }
        let detail = crate::k2_output::atom_root(arena, why);
        crate::k2_output::comp1(arena, name, detail)
    }
}

} // verus!
