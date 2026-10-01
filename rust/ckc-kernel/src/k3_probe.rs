use crate::k2_term::{ETermArena, push_atom, push_comp, push_nil, push_var};
#[cfg(verus_keep_ghost)]
use crate::k2_term::{arena_ok, child_terms, root_ok};
#[cfg(verus_keep_ghost)]
use ckc_spec::engine::*;
#[cfg(verus_keep_ghost)]
use ckc_spec::replay::{atom, gid_name};
#[cfg(verus_keep_ghost)]
use ckc_spec::term::{Term, ground, ground_all};
use ckc_spec::trace::*;
#[cfg(verus_keep_ghost)]
use ckc_spec::v1text::{BodyItem, DocClause, ascii};
use vstd::prelude::*;
use vstd::slice::slice_to_vec;

verus! {

broadcast use {vstd::seq::group_seq_axioms, vstd::seq_lib::group_seq_properties};

proof fn shift_ground(t: Term, off: nat)
    requires
        ground(t),
    ensures
        shift(t, off) == t,
    decreases t,
{
    if let Term::Comp(name, args) = t {
        shift_all_ground(args, off);
    }
}

proof fn shift_all_ground(ts: Seq<Term>, off: nat)
    requires
        ground_all(ts),
    ensures
        shift_all(ts, off) == ts,
    decreases ts,
{
    if ts.len() > 0 {
        shift_ground(ts[0], off);
        shift_all_ground(ts.drop_first(), off);
        assert(seq![ts[0]] + ts.drop_first() =~= ts);
    }
}

proof fn subst_ground(t: Term, x: nat, v: Term)
    requires
        ground(t),
    ensures
        subst(t, x, v) == t,
    decreases t,
{
    if let Term::Comp(name, args) = t {
        subst_all_ground(args, x, v);
    }
}

proof fn subst_all_ground(ts: Seq<Term>, x: nat, v: Term)
    requires
        ground_all(ts),
    ensures
        subst_all(ts, x, v) == ts,
    decreases ts,
{
    if ts.len() > 0 {
        subst_ground(ts[0], x, v);
        subst_all_ground(ts.drop_first(), x, v);
        assert(seq![ts[0]] + ts.drop_first() =~= ts);
    }
}

proof fn apply_ground(t: Term, th: Seq<(nat, Term)>)
    requires
        ground(t),
    ensures
        apply(t, th) == t,
    decreases th.len(),
{
    if th.len() > 0 {
        subst_ground(t, th[0].0, th[0].1);
        apply_ground(t, th.drop_first());
    }
}

// A ground site and a nonground payload can never be the site as called.
proof fn counterexample_rejected(site: Term, payload: Term)
    requires
        site == cx_lit(atom("a"@), "q"@),
        payload == cx_lit(Term::Var(0), "q"@),
        ground(site),
        !ground(payload),
    ensures
        !forest_valid(cx_db(), cx_lit(atom("c"@), "r"@), cx_forest()),
{
    let db = cx_db();
    let goal = cx_lit(atom("c"@), "r"@);
    let kids = seq![PNode::Naf(payload)];
    assert(cx_forest() == seq![PNode::Clause(2, kids)]);
    assert(conj_leaves(goal) == seq![goal]);
    assert forall|th: Seq<(nat, Term)>|
        !#[trigger] kids_valid(db, th, conj_leaves(goal), cx_forest()) by {
        assert forall|k: nat|
            #![trigger resolves(db, th, goal, 2, k, kids)]
            #![trigger shift(db[2int].head, k)]
            !resolves(db, th, goal, 2, k, kids) by {
            let it = BodyItem::Naf(seq![site]);
            assert(db[2].body == seq![it]);
            assert(seq![site].drop_first() =~= Seq::<Term>::empty());
            reveal_with_fuel(ground_all, 2);
            assert(ground_all(seq![site]));
            shift_all_ground(seq![site], k);
            assert(conj_term(seq![site]) == site);
            assert(item_term(it, k) == Term::Comp(naf_name(), seq![site]));
            assert(body_terms(db[2].body, k) == seq![Term::Comp(naf_name(), seq![site])]);
            apply_ground(site, th);
            assert(payload != apply(site, th));
            assert(!node_valid(db, th, Term::Comp(naf_name(), seq![site]), PNode::Naf(payload)));
        }
        assert(!node_valid(db, th, goal, PNode::Clause(2, kids)));
        assert(!kids_valid(db, th, seq![goal], cx_forest()));
    }
}

// guideline_entity('$guideline_id'(context, probe, 1, box(1), []), x, noun, countable)
fn cx_lit_exec(arena: &mut ETermArena, x: usize, noun: &[u8], Ghost(n): Ghost<Seq<char>>) -> (root:
    usize)
    requires
        root_ok(old(arena), x),
        noun@ == ascii(n),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        root_ok(final(arena), root),
        final(arena)@[root as int] == cx_lit(old(arena)@[x as int], n),
{
    let ghost s0 = arena.nodes@;
    let ghost tx = arena@[x as int];
    let context: &[u8] = b"context";
    let probe: &[u8] = b"probe";
    let boxed: &[u8] = b"box";
    let gid: &[u8] = b"$guideline_id";
    let entity: &[u8] = b"guideline_entity";
    let countable: &[u8] = b"countable";
    proof {
        reveal_byteslit(b"context");
        reveal_strlit("context");
        reveal_byteslit(b"probe");
        reveal_strlit("probe");
        reveal_byteslit(b"box");
        reveal_strlit("box");
        reveal_byteslit(b"$guideline_id");
        reveal_strlit("$guideline_id");
        reveal_byteslit(b"guideline_entity");
        reveal_strlit("guideline_entity");
        reveal_byteslit(b"countable");
        reveal_strlit("countable");
        reveal(ascii);
        assert(context@ == ascii("context"@));
        assert(probe@ == ascii("probe"@));
        assert(boxed@ == ascii("box"@));
        assert(gid@ == gid_name());
        assert(entity@ == ascii("guideline_entity"@));
        assert(countable@ == ascii("countable"@));
    }
    let a0 = push_atom(arena, slice_to_vec(context));
    let a1 = push_atom(arena, slice_to_vec(probe));
    let one = crate::k2_reject::push_usize_int(arena, 1);
    let one_b = crate::k2_reject::push_usize_int(arena, 1);
    let mut bv = Vec::new();
    bv.push(one_b);
    let ghost s_bx = arena.nodes@;
    let bx = push_comp(arena, slice_to_vec(boxed), bv);
    proof {
        assert(child_terms(s_bx, bv@) =~= seq![Term::Int(1)]);
    }
    let nil = push_nil(arena);
    let mut roots = Vec::new();
    roots.push(a0);
    roots.push(a1);
    roots.push(one);
    roots.push(bx);
    roots.push(nil);
    let ghost s_ctx = arena.nodes@;
    let ctx = push_comp(arena, slice_to_vec(gid), roots);
    proof {
        assert(child_terms(s_ctx, roots@) =~= seq![
            atom("context"@),
            atom("probe"@),
            Term::Int(1),
            Term::Comp(ascii("box"@), seq![Term::Int(1)]),
            Term::Nil,
        ]);
    }
    let nr = push_atom(arena, slice_to_vec(noun));
    let cr = push_atom(arena, slice_to_vec(countable));
    let mut parts = Vec::new();
    parts.push(ctx);
    parts.push(x);
    parts.push(nr);
    parts.push(cr);
    let ghost s_e = arena.nodes@;
    let root = push_comp(arena, slice_to_vec(entity), parts);
    proof {
        assert(s_e[x as int] == s0[x as int]);
        assert(child_terms(s_e, parts@) =~= seq![
            Term::Comp(
                gid_name(),
                seq![
                    atom("context"@),
                    atom("probe"@),
                    Term::Int(1),
                    Term::Comp(ascii("box"@), seq![Term::Int(1)]),
                    Term::Nil,
                ],
            ),
            tx,
            atom(n),
            atom("countable"@),
        ]);
        assert(s0.is_prefix_of(arena.nodes@));
    }
    root
}

// Builds the counterexample's site goal q(a) and naf payload q(X) and checks
// the payload against the site: a ground site never equals a nonground payload.
pub fn naf_counterexample_impl() -> (rejected: bool)
    ensures
        rejected ==> !forest_valid(cx_db(), cx_lit(atom("c"@), "r"@), cx_forest()),
{
    let mut arena = crate::k2_reject::empty_arena();
    let a: &[u8] = b"a";
    let q: &[u8] = b"q";
    proof {
        reveal_byteslit(b"a");
        reveal_strlit("a");
        reveal_byteslit(b"q");
        reveal_strlit("q");
        reveal(ascii);
        assert(a@ == ascii("a"@));
        assert(q@ == ascii("q"@));
    }
    let ar = push_atom(&mut arena, slice_to_vec(a));
    let site = cx_lit_exec(&mut arena, ar, q, Ghost("q"@));
    let ghost st = arena@[site as int];
    let site_ground = crate::k2_walk::ground_root(&arena, site);
    let var_spelling = crate::k2_engine::var_spelling(0);
    let xr = push_var(&mut arena, 0, var_spelling);
    let payload = cx_lit_exec(&mut arena, xr, q, Ghost("q"@));
    let ghost pt = arena@[payload as int];
    let payload_ground = crate::k2_walk::ground_root(&arena, payload);
    let rejected = site_ground && !payload_ground;
    proof {
        if rejected {
            counterexample_rejected(st, pt);
        }
    }
    rejected
}

} // verus!
