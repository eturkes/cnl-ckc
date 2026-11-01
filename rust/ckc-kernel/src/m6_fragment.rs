use crate::k2_engine::{EBodyItem, EClause};
#[cfg(verus_keep_ghost)]
use crate::k2_engine::{db_valid, db_view};
use crate::k2_term::{ENode, ETermArena};
#[cfg(verus_keep_ghost)]
use crate::k2_term::{arena_ok, root_ok};
#[cfg(verus_keep_ghost)]
use crate::m6_cands::{seqs_models, seqs_valid};
use crate::m6_model::*;
use crate::m6_symbols::Sym;
use crate::m6_term::*;
use ckc_spec::emit as spec;
#[cfg(verus_keep_ghost)]
use ckc_spec::replay::arg;
use ckc_spec::term::Term;
use ckc_spec::v1text::{BodyItem, DocClause};
use ckc_spec::witness as ws;
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

// The completeness fragment (contract ws D7): `ws::in_fragment`, decided.
pub fn is_arith_lit(arena: &ETermArena, t: &T) -> (out: bool)
    requires
        arena_ok(arena),
        valid(arena.nodes@, t),
    ensures
        out == ws::is_arith_lit(t@),
{
    match parts(arena, t) {
        Some((name, xs)) => crate::compare_impl::is_arith_exec(&name, xs.len()),
        None => false,
    }
}

pub fn is_naf_name(name: &Vec<u8>) -> (out: bool)
    ensures
        out == (name@ == ckc_spec::engine::naf_name()),
{
    let mut naf: Vec<u8> = Vec::new();
    naf.push(b'\\');
    naf.push(b'+');
    proof {
        reveal_strlit("\\+");
        reveal(ckc_spec::engine::naf_name);
        reveal(ckc_spec::v1text::ascii);
        assert_seqs_equal!(naf@ == ckc_spec::engine::naf_name());
    }
    let mut i = 0usize;
    if name.len() != naf.len() {
        return false;
    }
    while i < name.len()
        invariant
            name.len() == naf.len(),
            naf@ == ckc_spec::engine::naf_name(),
            i <= name.len(),
            forall|j: int| 0 <= j < i ==> name@[j] == naf@[j],
        decreases name.len() - i,
    {
        if name[i] != naf[i] {
            proof {
                assert(name@[i as int] != ckc_spec::engine::naf_name()[i as int]);
            }
            return false;
        }
        i += 1;
    }
    proof {
        assert(name@ =~= naf@);
    }
    true
}

pub fn is_comma_name(name: &Vec<u8>) -> (out: bool)
    ensures
        out == (name@ == ckc_spec::engine::comma_name()),
{
    proof {
        reveal_strlit(",");
        reveal(ckc_spec::engine::comma_name);
        reveal(ckc_spec::v1text::ascii);
        assert(crate::m6_symbols::symbol(&Sym::Comma) =~= ckc_spec::engine::comma_name());
    }
    has_name(name, &Sym::Comma)
}

proof fn arg_at(t: Term, xs: Seq<T>, k: int)
    requires
        models(xs) == ckc_spec::engine::args_of(t),
        0 <= k < xs.len(),
    ensures
        xs[k]@ == arg(t, k),
{
    assert(models(xs)[k] == xs[k]@);
    match t {
        Term::Comp(_, a) => {
            assert(a.len() == xs.len());
        },
        _ => {
            assert(ckc_spec::engine::args_of(t) == Seq::<Term>::empty());
            assert(models(xs).len() == xs.len());
        },
    }
}

// --- ground default pairs ---
pub fn ground_pairs(arena: &ETermArena, g: &Group) -> (out: bool)
    requires
        arena_ok(arena),
        group_valid(arena.nodes@, g),
    ensures
        out == (forall|k: int|
            0 <= k < g@.pairs.len() ==> ckc_spec::term::ground((#[trigger] g@.pairs[k]).1)),
{
    let mut i = 0usize;
    while i < g.pairs.len()
        invariant
            arena_ok(arena),
            group_valid(arena.nodes@, g),
            i <= g.pairs.len(),
            forall|k: int| 0 <= k < i ==> ckc_spec::term::ground((#[trigger] g@.pairs[k]).1),
        decreases g.pairs.len() - i,
    {
        proof {
            assert(binding_valid(arena.nodes@, &g.pairs@[i as int]));
            assert(g@.pairs[i as int] == g.pairs@[i as int]@);
        }
        if !ground(arena, &g.pairs[i].value) {
            return false;
        }
        i += 1;
    }
    true
}

// --- the chain's shape ---
// `ws::outputs`: each chain node's output pair, in order.
pub fn outputs(arena: &ETermArena, sel: &ESel) -> (out: Vec<T>)
    requires
        arena_ok(arena),
        valid_all(arena.nodes@, sel.chain@),
        arith_goals(sel.chain@),
    ensures
        valid_all(arena.nodes@, out@),
        models(out@) == ws::outputs(sel@),
{
    let ghost rows = sel@.chain.map_values(|c: Term| seq![arg(c, 4), arg(c, 5)]);
    let mut out: Vec<T> = Vec::new();
    let mut k = 0usize;
    proof {
        assert(rows.take(0) =~= Seq::<Seq<Term>>::empty());
        assert_seqs_equal!(models(out@) == rows.take(0).flatten());
    }
    while k < sel.chain.len()
        invariant
            arena_ok(arena),
            valid_all(arena.nodes@, sel.chain@),
            arith_goals(sel.chain@),
            rows == sel@.chain.map_values(|c: Term| seq![arg(c, 4), arg(c, 5)]),
            k <= sel.chain.len(),
            valid_all(arena.nodes@, out@),
            models(out@) == rows.take(k as int).flatten(),
        decreases sel.chain.len() - k,
    {
        proof {
            assert(valid(arena.nodes@, &sel.chain@[k as int]));
            assert(sel@.chain[k as int] == sel.chain@[k as int]@);
            assert(rows.take(k as int + 1) =~= rows.take(k as int).push(rows[k as int]));
            rows.take(k as int).lemma_flatten_push(rows[k as int]);
        }
        let xs = args(arena, &sel.chain[k]);
        proof {
            let t = sel.chain@[k as int]@;
            assert(xs.len() == 6) by {
                reveal(ckc_spec::arith::is_arith);
            }
            assert(models(xs@)[4] == xs@[4]@);
            assert(models(xs@)[5] == xs@[5]@);
            assert(xs@[4]@ == arg(t, 4));
            assert(xs@[5]@ == arg(t, 5));
            assert(valid(arena.nodes@, &xs@[4]));
            assert(valid(arena.nodes@, &xs@[5]));
        }
        let ghost before = out@;
        out.push(xs[4].cp());
        out.push(xs[5].cp());
        proof {
            assert_seqs_equal!(models(out@) == models(before) + rows[k as int]);
            assert forall|j: int| 0 <= j < out.len() implies #[trigger] valid(
                arena.nodes@,
                &out@[j],
            ) by {
                if j < before.len() {
                    assert(out@[j] == before[j]);
                }
            }
        }
        k += 1;
    }
    proof {
        assert(rows.take(k as int) =~= rows);
    }
    out
}

// `ws::chain_input` for node k's input pair (s, n).
fn chain_input(arena: &ETermArena, sel: &ESel, k: usize, s: &T, n: &T) -> (out: bool)
    requires
        arena_ok(arena),
        crate::m6_witness::ops_valid(arena.nodes@, sel),
        valid_all(arena.nodes@, sel.chain@),
        arith_goals(sel.chain@),
        k <= sel.chain.len(),
        valid(arena.nodes@, s),
        valid(arena.nodes@, n),
    ensures
        out == ws::chain_input(sel@, k as int, s@, n@),
{
    let mut i = 0usize;
    while i < sel.ops.len()
        invariant
            arena_ok(arena),
            crate::m6_witness::ops_valid(arena.nodes@, sel),
            valid(arena.nodes@, s),
            valid(arena.nodes@, n),
            i <= sel.ops.len(),
            forall|j: int|
                0 <= j < i ==> !(s@ == (#[trigger] sel@.ops[j]).s && n@ == sel@.ops[j].n),
        decreases sel.ops.len() - i,
    {
        proof {
            assert(opnd_valid(arena.nodes@, &sel.ops@[i as int]));
            assert(sel@.ops[i as int] == sel.ops@[i as int]@);
        }
        if crate::m6_term::equal(arena, s, &sel.ops[i].s) && crate::m6_term::equal(
            arena,
            n,
            &sel.ops[i].n,
        ) {
            proof {
                assert(s@ == sel@.ops[i as int].s && n@ == sel@.ops[i as int].n);
            }
            return true;
        }
        i += 1;
    }
    if is_atom(arena, s, &Sym::Exactly) && crate::m6_cands::number_of(arena, n).is_some() {
        proof {
            reveal_strlit("exactly");
            assert(s@ == ws::exactly());
        }
        return true;
    }
    proof {
        reveal_strlit("exactly");
        assert(!(s@ == ws::exactly() && ckc_spec::arith::number(n@) is Some));
    }
    let mut m = 0usize;
    while m < k
        invariant
            arena_ok(arena),
            valid_all(arena.nodes@, sel.chain@),
            arith_goals(sel.chain@),
            k <= sel.chain.len(),
            valid(arena.nodes@, s),
            valid(arena.nodes@, n),
            m <= k,
            forall|j: int|
                0 <= j < m ==> !(s@ == arg(#[trigger] sel@.chain[j], 4) && n@ == arg(
                    sel@.chain[j],
                    5,
                )),
        decreases k - m,
    {
        proof {
            assert(valid(arena.nodes@, &sel.chain@[m as int]));
            assert(sel@.chain[m as int] == sel.chain@[m as int]@);
        }
        let xs = args(arena, &sel.chain[m]);
        proof {
            let t = sel.chain@[m as int]@;
            assert(xs.len() == 6) by {
                reveal(ckc_spec::arith::is_arith);
            }
            assert(models(xs@)[4] == xs@[4]@);
            assert(models(xs@)[5] == xs@[5]@);
            assert(xs@[4]@ == arg(t, 4));
            assert(xs@[5]@ == arg(t, 5));
        }
        if crate::m6_term::equal(arena, s, &xs[4]) && crate::m6_term::equal(arena, n, &xs[5]) {
            return true;
        }
        m += 1;
    }
    false
}

// No two equal terms (`Seq::no_duplicates`).
pub fn distinct(arena: &ETermArena, ts: &Vec<T>) -> (out: bool)
    requires
        arena_ok(arena),
        valid_all(arena.nodes@, ts@),
    ensures
        out == models(ts@).no_duplicates(),
{
    let mut a = 0usize;
    while a < ts.len()
        invariant
            arena_ok(arena),
            valid_all(arena.nodes@, ts@),
            a <= ts.len(),
            forall|x: int, y: int|
                0 <= x < a && x < y < ts.len() ==> #[trigger] models(ts@)[x] != #[trigger] models(
                    ts@,
                )[y],
        decreases ts.len() - a,
    {
        let mut b = a + 1;
        while b < ts.len()
            invariant
                arena_ok(arena),
                valid_all(arena.nodes@, ts@),
                a < ts.len(),
                a + 1 <= b <= ts.len(),
                forall|x: int, y: int|
                    0 <= x < a && x < y < ts.len() ==> #[trigger] models(ts@)[x]
                        != #[trigger] models(ts@)[y],
                forall|y: int| a < y < b ==> models(ts@)[a as int] != #[trigger] models(ts@)[y],
            decreases ts.len() - b,
        {
            proof {
                assert(valid(arena.nodes@, &ts@[a as int]));
                assert(valid(arena.nodes@, &ts@[b as int]));
            }
            if crate::m6_term::equal(arena, &ts[a], &ts[b]) {
                proof {
                    assert(models(ts@)[a as int] == models(ts@)[b as int]);
                }
                return false;
            }
            b += 1;
        }
        a += 1;
    }
    proof {
        assert forall|x: int, y: int|
            0 <= x < models(ts@).len() && 0 <= y < models(ts@).len() && x != y implies models(
            ts@,
        )[x] != models(ts@)[y] by {
            if x < y {
                assert(models(ts@)[x] != models(ts@)[y]);
            } else {
                assert(models(ts@)[y] != models(ts@)[x]);
            }
        }
    }
    true
}

// `ws::chain_shape`.
pub fn chain_shape(arena: &ETermArena, sel: &ESel) -> (out: bool)
    requires
        arena_ok(arena),
        crate::m6_witness::ops_valid(arena.nodes@, sel),
        valid_all(arena.nodes@, sel.chain@),
        arith_goals(sel.chain@),
    ensures
        out == ws::chain_shape(sel@),
{
    let mut k = 0usize;
    while k < sel.chain.len()
        invariant
            arena_ok(arena),
            crate::m6_witness::ops_valid(arena.nodes@, sel),
            valid_all(arena.nodes@, sel.chain@),
            arith_goals(sel.chain@),
            k <= sel.chain.len(),
            forall|j: int|
                0 <= j < k ==> ws::is_arith_lit(#[trigger] sel@.chain[j]) && ws::chain_input(
                    sel@,
                    j,
                    arg(sel@.chain[j], 0),
                    arg(sel@.chain[j], 1),
                ) && ws::chain_input(sel@, j, arg(sel@.chain[j], 2), arg(sel@.chain[j], 3)),
        decreases sel.chain.len() - k,
    {
        proof {
            assert(valid(arena.nodes@, &sel.chain@[k as int]));
            assert(sel@.chain[k as int] == sel.chain@[k as int]@);
        }
        let c = &sel.chain[k];
        if !is_arith_lit(arena, c) {
            return false;
        }
        let xs = args(arena, c);
        proof {
            let t = sel.chain@[k as int]@;
            assert(xs.len() == 6) by {
                reveal(ckc_spec::arith::is_arith);
            }
            arg_at(t, xs@, 0);
            arg_at(t, xs@, 1);
            arg_at(t, xs@, 2);
            arg_at(t, xs@, 3);
            assert(valid(arena.nodes@, &xs@[0]));
            assert(valid(arena.nodes@, &xs@[1]));
            assert(valid(arena.nodes@, &xs@[2]));
            assert(valid(arena.nodes@, &xs@[3]));
        }
        if !chain_input(arena, sel, k, &xs[0], &xs[1]) || !chain_input(
            arena,
            sel,
            k,
            &xs[2],
            &xs[3],
        ) {
            return false;
        }
        k += 1;
    }
    let mut i = 0usize;
    while i < sel.ops.len()
        invariant
            arena_ok(arena),
            crate::m6_witness::ops_valid(arena.nodes@, sel),
            i <= sel.ops.len(),
            forall|j: int|
                0 <= j < sel.chain.len() ==> ws::is_arith_lit(#[trigger] sel@.chain[j])
                    && ws::chain_input(sel@, j, arg(sel@.chain[j], 0), arg(sel@.chain[j], 1))
                    && ws::chain_input(sel@, j, arg(sel@.chain[j], 2), arg(sel@.chain[j], 3)),
            forall|j: int| 0 <= j < i ==> (#[trigger] sel@.ops[j]).s is Var && sel@.ops[j].n is Var,
        decreases sel.ops.len() - i,
    {
        proof {
            assert(opnd_valid(arena.nodes@, &sel.ops@[i as int]));
            assert(sel@.ops[i as int] == sel.ops@[i as int]@);
        }
        if !is_var(arena, &sel.ops[i].s) || !is_var(arena, &sel.ops[i].n) {
            return false;
        }
        i += 1;
    }
    let outs = outputs(arena, sel);
    let mut m = 0usize;
    while m < outs.len()
        invariant
            arena_ok(arena),
            valid_all(arena.nodes@, outs@),
            models(outs@) == ws::outputs(sel@),
            forall|j: int|
                0 <= j < sel.chain.len() ==> ws::is_arith_lit(#[trigger] sel@.chain[j])
                    && ws::chain_input(sel@, j, arg(sel@.chain[j], 0), arg(sel@.chain[j], 1))
                    && ws::chain_input(sel@, j, arg(sel@.chain[j], 2), arg(sel@.chain[j], 3)),
            forall|j: int|
                0 <= j < sel.ops.len() ==> (#[trigger] sel@.ops[j]).s is Var
                    && sel@.ops[j].n is Var,
            m <= outs.len(),
            forall|j: int| 0 <= j < m ==> (#[trigger] models(outs@)[j]) is Var,
        decreases outs.len() - m,
    {
        proof {
            assert(valid(arena.nodes@, &outs@[m as int]));
            assert(models(outs@)[m as int] == outs@[m as int]@);
        }
        if !is_var(arena, &outs[m]) {
            proof {
                assert(!(ws::outputs(sel@)[m as int] is Var));
            }
            return false;
        }
        m += 1;
    }
    let mut ss: Vec<T> = Vec::new();
    let mut q = 0usize;
    while q < sel.ops.len()
        invariant
            arena_ok(arena),
            crate::m6_witness::ops_valid(arena.nodes@, sel),
            valid_all(arena.nodes@, outs@),
            models(outs@) == ws::outputs(sel@),
            forall|j: int|
                0 <= j < sel.chain.len() ==> ws::is_arith_lit(#[trigger] sel@.chain[j])
                    && ws::chain_input(sel@, j, arg(sel@.chain[j], 0), arg(sel@.chain[j], 1))
                    && ws::chain_input(sel@, j, arg(sel@.chain[j], 2), arg(sel@.chain[j], 3)),
            forall|j: int|
                0 <= j < sel.ops.len() ==> (#[trigger] sel@.ops[j]).s is Var
                    && sel@.ops[j].n is Var,
            forall|j: int| 0 <= j < outs.len() ==> (#[trigger] models(outs@)[j]) is Var,
            q <= sel.ops.len(),
            ss.len() == q,
            valid_all(arena.nodes@, ss@),
            forall|j: int| 0 <= j < q ==> (#[trigger] ss@[j])@ == sel@.ops[j].s,
        decreases sel.ops.len() - q,
    {
        proof {
            assert(opnd_valid(arena.nodes@, &sel.ops@[q as int]));
            assert(sel@.ops[q as int] == sel.ops@[q as int]@);
        }
        let x = sel.ops[q].s.cp();
        proof {
            extend(arena.nodes@, ss@, x);
        }
        ss.push(x);
        q += 1;
    }
    let cs = crate::m6_cands::counts(sel);
    proof {
        assert forall|j: int| 0 <= j < cs.len() implies #[trigger] valid(arena.nodes@, &cs@[j]) by {
            assert(opnd_valid(arena.nodes@, &sel.ops@[j]));
        }
    }
    let all = concat(&concat(&ss, &cs), &outs);
    proof {
        assert forall|j: int| 0 <= j < all.len() implies #[trigger] valid(
            arena.nodes@,
            &all@[j],
        ) by {
            if j < ss.len() {
                assert(all@[j] == ss@[j]);
            } else if j < ss.len() + cs.len() {
                assert(all@[j] == cs@[j - ss.len()]);
            } else {
                assert(all@[j] == outs@[j - ss.len() - cs.len()]);
            }
        }
        assert(models(ss@) =~= sel@.ops.map_values(|o: ws::Opnd| o.s));
        assert(models(all@) == sel@.ops.map_values(|o: ws::Opnd| o.s) + ws::counts(sel@)
            + ws::outputs(sel@));
    }
    distinct(arena, &all)
}

// --- the default pairs ---
pub fn exact_comparators(arena: &ETermArena, g: &Group, sel: &ESel) -> (out: bool)
    requires
        arena_ok(arena),
        group_valid(arena.nodes@, g),
        crate::m6_witness::ops_valid(arena.nodes@, sel),
    ensures
        out == ws::exact_comparators(g@, sel@),
{
    let mut i = 0usize;
    while i < sel.ops.len()
        invariant
            arena_ok(arena),
            group_valid(arena.nodes@, g),
            crate::m6_witness::ops_valid(arena.nodes@, sel),
            i <= sel.ops.len(),
            forall|j: int|
                0 <= j < i ==> spec::lookup(g@.pairs, (#[trigger] sel@.ops[j]).s) == Some(
                    ws::exactly(),
                ) && spec::lookup(g@.pairs, sel@.ops[j].n) is Some,
        decreases sel.ops.len() - i,
    {
        proof {
            assert(opnd_valid(arena.nodes@, &sel.ops@[i as int]));
            assert(sel@.ops[i as int] == sel.ops@[i as int]@);
            reveal_strlit("exactly");
        }
        match crate::m6_refs::lookup(arena, &g.pairs, &sel.ops[i].s) {
            Some(v) => {
                if !is_atom(arena, &v, &Sym::Exactly) {
                    return false;
                }
            },
            None => {
                return false;
            },
        }
        if crate::m6_refs::lookup(arena, &g.pairs, &sel.ops[i].n).is_none() {
            return false;
        }
        i += 1;
    }
    true
}

// --- the skeleton holds each slot once, nothing else ---
pub fn fixed_skeleton(arena: &ETermArena, g: &Group, sel: &ESel, sk: &Vec<T>) -> (out: bool)
    requires
        arena_ok(arena),
        crate::m6_witness::ops_valid(arena.nodes@, sel),
        valid_all(arena.nodes@, sk@),
        models(sk@) == ws::skeleton(g@, sel@),
    ensures
        out == ws::fixed_skeleton(g@, sel@),
{
    let cs = crate::m6_cands::counts(sel);
    proof {
        assert forall|j: int| 0 <= j < cs.len() implies #[trigger] valid(arena.nodes@, &cs@[j]) by {
            assert(opnd_valid(arena.nodes@, &sel.ops@[j]));
        }
    }
    if !distinct(arena, &cs) {
        return false;
    }
    let st = crate::m6_vars::stream_all(arena, sk);
    let ghost skm = models(sk@);
    let mut i = 0usize;
    while i < sel.ops.len()
        invariant
            arena_ok(arena),
            crate::m6_witness::ops_valid(arena.nodes@, sel),
            skm == models(sk@),
            skm == ws::skeleton(g@, sel@),
            crate::m6_vars::nums(st@) == ckc_spec::term::var_stream_all(skm),
            i <= sel.ops.len(),
            forall|j: int|
                0 <= j < i ==> match (#[trigger] sel@.ops[j]).n {
                    Term::Var(x) => spec::occ_all(x, skm) == 1,
                    _ => false,
                },
        decreases sel.ops.len() - i,
    {
        proof {
            assert(opnd_valid(arena.nodes@, &sel.ops@[i as int]));
            assert(sel@.ops[i as int] == sel.ops@[i as int]@);
        }
        if !is_var(arena, &sel.ops[i].n) {
            return false;
        }
        let x = var_index(arena, &sel.ops[i].n);
        proof {
            crate::m6_safety::occ_all_bridge(skm, x as nat);
            reveal(spec::var_index);
        }
        if crate::m6_vars::count(&st, x) != 1 {
            return false;
        }
        i += 1;
    }
    let mut q = 0usize;
    while q < st.len()
        invariant
            arena_ok(arena),
            crate::m6_witness::ops_valid(arena.nodes@, sel),
            skm == models(sk@),
            skm == ws::skeleton(g@, sel@),
            crate::m6_vars::nums(st@) == ckc_spec::term::var_stream_all(skm),
            models(cs@) == ws::counts(sel@),
            cs.len() == sel.ops.len(),
            q <= st.len(),
            forall|j: int|
                0 <= j < q ==> ws::counts(sel@).contains(
                    Term::Var(#[trigger] crate::m6_vars::nums(st@)[j]),
                ),
        decreases st.len() - q,
    {
        let x = st[q];
        let mut found = false;
        let mut i = 0usize;
        while i < sel.ops.len()
            invariant
                arena_ok(arena),
                crate::m6_witness::ops_valid(arena.nodes@, sel),
                models(cs@) == ws::counts(sel@),
                cs.len() == sel.ops.len(),
                i <= sel.ops.len(),
                found ==> ws::counts(sel@).contains(Term::Var(x as nat)),
                !found ==> forall|j: int|
                    0 <= j < i ==> (#[trigger] ws::counts(sel@)[j]) != Term::Var(x as nat),
            decreases sel.ops.len() - i,
        {
            proof {
                assert(opnd_valid(arena.nodes@, &sel.ops@[i as int]));
                assert(sel@.ops[i as int] == sel.ops@[i as int]@);
                assert(ws::counts(sel@)[i as int] == sel@.ops[i as int].n);
                reveal(spec::var_index);
            }
            if !found && is_var(arena, &sel.ops[i].n) && var_index(arena, &sel.ops[i].n) == x {
                found = true;
            }
            i += 1;
        }
        if !found {
            proof {
                assert(!ws::counts(sel@).contains(Term::Var(x as nat)));
                assert(crate::m6_vars::nums(st@)[q as int] == x as nat);
                assert(ckc_spec::term::var_stream_all(skm).contains(x as nat));
                assert(ckc_spec::term::var_stream_all(skm).to_set().contains(x as nat));
                assert(spec::vars_all(skm).contains(x as nat));
            }
            return false;
        }
        q += 1;
    }
    proof {
        assert forall|x: nat| #[trigger] spec::vars_all(skm).contains(x) implies ws::counts(
            sel@,
        ).contains(Term::Var(x)) by {
            assert(ckc_spec::term::var_stream_all(skm).to_set().contains(x));
            assert(ckc_spec::term::var_stream_all(skm).contains(x));
            let j = choose|j: int|
                0 <= j < ckc_spec::term::var_stream_all(skm).len()
                    && ckc_spec::term::var_stream_all(skm)[j] == x;
            assert(crate::m6_vars::nums(st@)[j] == x);
        }
    }
    true
}

// --- body literals ---
pub fn primitive(arena: &ETermArena, t: &T) -> (out: bool)
    requires
        arena_ok(arena),
        valid(arena.nodes@, t),
    ensures
        out == ws::primitive(t@),
{
    match parts(arena, t) {
        Some((name, xs)) => !(is_comma_name(&name) && xs.len() == 2) && !(is_naf_name(&name)
            && xs.len() == 1),
        None => false,
    }
}

// `ws::head_call`: K2 calls the head as a predicate.
fn head_call(arena: &ETermArena, h: &T) -> (out: bool)
    requires
        arena_ok(arena),
        valid(arena.nodes@, h),
    ensures
        out == ws::head_call(h@),
{
    if !primitive(arena, h) {
        return false;
    }
    match parts(arena, h) {
        Some((name, xs)) => !crate::compare_impl::is_builtin_exec(&name, xs.len()),
        None => false,
    }
}

pub fn primitive_body(arena: &ETermArena, body: &Vec<Body>) -> (out: bool)
    requires
        arena_ok(arena),
        bodies_valid(arena.nodes@, body@),
    ensures
        out == ws::primitive_body(body_models(body@)),
{
    let ghost bm = body_models(body@);
    let mut k = 0usize;
    while k < body.len()
        invariant
            arena_ok(arena),
            bodies_valid(arena.nodes@, body@),
            bm == body_models(body@),
            k <= body.len(),
            forall|j: int|
                0 <= j < k ==> match #[trigger] bm[j] {
                    BodyItem::Pos(t) => ws::primitive(t),
                    BodyItem::Naf(gs) => gs.len() > 0 && forall|m: int|
                        0 <= m < gs.len() ==> ws::primitive(#[trigger] gs[m]),
                },
        decreases body.len() - k,
    {
        proof {
            assert(body_valid(arena.nodes@, &body@[k as int]));
            assert(bm[k as int] == body@[k as int]@);
        }
        match &body[k] {
            Body::Pos(t) => {
                if !primitive(arena, t) {
                    return false;
                }
            },
            Body::Naf(gs) => {
                proof {
                    assert(bm[k as int] == BodyItem::Naf(models(gs@)));
                }
                if gs.len() == 0 {
                    proof {
                        assert(!ws::primitive_body(bm)) by {
                            if ws::primitive_body(bm) {
                                match bm[k as int] {
                                    BodyItem::Naf(g2) => {
                                        assert(g2 == models(gs@));
                                    },
                                    _ => {},
                                }
                            }
                        }
                    }
                    return false;
                }
                let mut m = 0usize;
                while m < gs.len()
                    invariant
                        arena_ok(arena),
                        valid_all(arena.nodes@, gs@),
                        bm == body_models(body@),
                        k < body.len(),
                        bm[k as int] == BodyItem::Naf(models(gs@)),
                        m <= gs.len(),
                        forall|q: int| 0 <= q < m ==> ws::primitive(#[trigger] models(gs@)[q]),
                    decreases gs.len() - m,
                {
                    proof {
                        assert(valid(arena.nodes@, &gs@[m as int]));
                    }
                    if !primitive(arena, &gs[m]) {
                        proof {
                            assert(models(gs@)[m as int] == gs@[m as int]@);
                            assert(!ws::primitive_body(bm)) by {
                                if ws::primitive_body(bm) {
                                    assert(match bm[k as int] {
                                        BodyItem::Pos(t) => ws::primitive(t),
                                        BodyItem::Naf(g2) => g2.len() > 0 && forall|q: int|
                                            0 <= q < g2.len() ==> ws::primitive(#[trigger] g2[q]),
                                    });
                                    match bm[k as int] {
                                        BodyItem::Naf(g2) => {
                                            assert(g2 == models(gs@));
                                            assert(ws::primitive(g2[m as int]));
                                        },
                                        _ => {},
                                    }
                                }
                            }
                        }
                        return false;
                    }
                    m += 1;
                }
            },
        }
        k += 1;
    }
    true
}

// `ws::mentions`: a variable slot occurs in t.
pub fn mentions(arena: &ETermArena, t: &T, slot: &T) -> (out: bool)
    requires
        arena_ok(arena),
        valid(arena.nodes@, t),
        valid(arena.nodes@, slot),
    ensures
        out == ws::mentions(t@, slot@),
{
    if is_var(arena, slot) {
        let x = var_index(arena, slot);
        proof {
            reveal(spec::var_index);
            assert(slot@ == Term::Var(x as nat));
            assert(arena@[t.root as int] == t@);
        }
        crate::k2_engine::occurs_root(arena, x, t.root)
    } else {
        false
    }
}

// The NAF goals of a body, in order (`ws::naf_goals`).
pub fn naf_goals(arena: &ETermArena, body: &Vec<Body>) -> (out: Vec<T>)
    requires
        arena_ok(arena),
        bodies_valid(arena.nodes@, body@),
    ensures
        valid_all(arena.nodes@, out@),
        models(out@) == ws::naf_goals(body_models(body@)),
{
    let ghost bm = body_models(body@);
    let ghost rows = bm.map_values(
        |it: BodyItem|
            match it {
                BodyItem::Naf(gs) => gs,
                BodyItem::Pos(_) => Seq::empty(),
            },
    );
    let mut out: Vec<T> = Vec::new();
    let mut k = 0usize;
    proof {
        assert(rows.take(0) =~= Seq::<Seq<Term>>::empty());
        assert_seqs_equal!(models(out@) == rows.take(0).flatten());
    }
    while k < body.len()
        invariant
            arena_ok(arena),
            bodies_valid(arena.nodes@, body@),
            bm == body_models(body@),
            rows == bm.map_values(
                |it: BodyItem|
                    match it {
                        BodyItem::Naf(gs) => gs,
                        BodyItem::Pos(_) => Seq::empty(),
                    },
            ),
            k <= body.len(),
            valid_all(arena.nodes@, out@),
            models(out@) == rows.take(k as int).flatten(),
        decreases body.len() - k,
    {
        proof {
            assert(body_valid(arena.nodes@, &body@[k as int]));
            assert(bm[k as int] == body@[k as int]@);
            assert(rows.take(k as int + 1) =~= rows.take(k as int).push(rows[k as int]));
            rows.take(k as int).lemma_flatten_push(rows[k as int]);
        }
        let ghost before = out@;
        match &body[k] {
            Body::Naf(gs) => {
                out = concat(&out, gs);
                proof {
                    assert forall|j: int| 0 <= j < out.len() implies #[trigger] valid(
                        arena.nodes@,
                        &out@[j],
                    ) by {
                        if j < before.len() {
                            assert(out@[j] == before[j]);
                        } else {
                            assert(out@[j] == gs@[j - before.len()]);
                        }
                    }
                }
            },
            Body::Pos(_) => {
                proof {
                    assert(rows[k as int] == Seq::<Term>::empty());
                    assert(models(out@) =~= models(before) + rows[k as int]);
                }
            },
        }
        k += 1;
    }
    proof {
        assert(rows.take(k as int) =~= rows);
    }
    out
}

// `ws::chain_body`.
pub fn chain_body(arena: &ETermArena, sel: &ESel, outs: &Vec<T>, body: &Vec<Body>) -> (out: bool)
    requires
        arena_ok(arena),
        valid_all(arena.nodes@, sel.chain@),
        valid_all(arena.nodes@, outs@),
        models(outs@) == ws::outputs(sel@),
        bodies_valid(arena.nodes@, body@),
    ensures
        out == ws::chain_body(sel@, body_models(body@)),
{
    let ghost bm = body_models(body@);
    let pos = crate::m6_safety::positive_terms(arena, body);
    let ghost pm = models(pos@);
    let mut ar: Vec<T> = Vec::new();
    let mut ot: Vec<T> = Vec::new();
    let mut i = 0usize;
    proof {
        reveal(Seq::filter);
        assert(pm.take(0) =~= Seq::<Term>::empty());
        assert_seqs_equal!(models(ar@) == Seq::<Term>::empty());
        assert_seqs_equal!(models(ot@) == Seq::<Term>::empty());
    }
    while i < pos.len()
        invariant
            arena_ok(arena),
            valid_all(arena.nodes@, pos@),
            pm == models(pos@),
            i <= pos.len(),
            valid_all(arena.nodes@, ar@),
            valid_all(arena.nodes@, ot@),
            models(ar@) == pm.take(i as int).filter(|t: Term| ws::is_arith_lit(t)),
            models(ot@) == pm.take(i as int).filter(|t: Term| !ws::is_arith_lit(t)),
        decreases pos.len() - i,
    {
        proof {
            assert(valid(arena.nodes@, &pos@[i as int]));
            assert(pm[i as int] == pos@[i as int]@);
            assert(pm.take(i as int + 1) == pm.take(i as int).push(pm[i as int]));
            pm.take(i as int).lemma_filter_push(pm[i as int], |t: Term| ws::is_arith_lit(t));
            pm.take(i as int).lemma_filter_push(pm[i as int], |t: Term| !ws::is_arith_lit(t));
        }
        let x = pos[i].cp();
        if is_arith_lit(arena, &x) {
            proof {
                extend(arena.nodes@, ar@, x);
            }
            ar.push(x);
        } else {
            proof {
                extend(arena.nodes@, ot@, x);
            }
            ot.push(x);
        }
        i += 1;
    }
    let ghost spos = spec::pos_terms(bm.filter(|it: BodyItem| it is Pos));
    proof {
        assert(pm.take(i as int) =~= pm);
        assert(pm == crate::m6_safety::positive_terms_model(bm));
        assert(pm == spos);
        assert(models(ar@) == spos.filter(|t: Term| ws::is_arith_lit(t)));
        assert(models(ot@) == spos.filter(|t: Term| !ws::is_arith_lit(t)));
    }
    if ar.len() != sel.chain.len() {
        proof {
            assert(models(ar@).len() != sel@.chain.len());
            assert(!ws::chain_body(sel@, bm));
        }
        return false;
    }
    let mut q = 0usize;
    while q < ar.len()
        invariant
            arena_ok(arena),
            valid_all(arena.nodes@, ar@),
            valid_all(arena.nodes@, sel.chain@),
            ar.len() == sel.chain.len(),
            bm == body_models(body@),
            spos == spec::pos_terms(bm.filter(|it: BodyItem| it is Pos)),
            models(ar@) == spos.filter(|t: Term| ws::is_arith_lit(t)),
            q <= ar.len(),
            forall|j: int| 0 <= j < q ==> models(ar@)[j] == #[trigger] sel@.chain[j],
        decreases ar.len() - q,
    {
        proof {
            assert(valid(arena.nodes@, &ar@[q as int]));
            assert(valid(arena.nodes@, &sel.chain@[q as int]));
        }
        if !crate::m6_term::equal(arena, &ar[q], &sel.chain[q]) {
            proof {
                assert(models(ar@)[q as int] != sel@.chain[q as int]);
                assert(models(ar@) != sel@.chain);
                assert(!ws::chain_body(sel@, bm));
            }
            return false;
        }
        q += 1;
    }
    proof {
        assert(models(ar@) =~= sel@.chain);
    }
    let nafs = naf_goals(arena, body);
    let mut k = 0usize;
    while k < nafs.len()
        invariant
            arena_ok(arena),
            valid_all(arena.nodes@, nafs@),
            bm == body_models(body@),
            models(nafs@) == ws::naf_goals(bm),
            k <= nafs.len(),
            forall|j: int| 0 <= j < k ==> !ws::is_arith_lit(#[trigger] models(nafs@)[j]),
        decreases nafs.len() - k,
    {
        proof {
            assert(valid(arena.nodes@, &nafs@[k as int]));
            assert(models(nafs@)[k as int] == nafs@[k as int]@);
        }
        if is_arith_lit(arena, &nafs[k]) {
            return false;
        }
        k += 1;
    }
    let others = concat(&ot, &nafs);
    proof {
        assert forall|j: int| 0 <= j < others.len() implies #[trigger] valid(
            arena.nodes@,
            &others@[j],
        ) by {
            if j < ot.len() {
                assert(others@[j] == ot@[j]);
            } else {
                assert(others@[j] == nafs@[j - ot.len()]);
            }
        }
    }
    proof {
        assert(models(others@) == spos.filter(|t: Term| !ws::is_arith_lit(t)) + ws::naf_goals(bm));
    }
    let mut a = 0usize;
    while a < others.len()
        invariant
            arena_ok(arena),
            valid_all(arena.nodes@, others@),
            valid_all(arena.nodes@, outs@),
            models(outs@) == ws::outputs(sel@),
            bm == body_models(body@),
            models(others@) == spos.filter(|t: Term| !ws::is_arith_lit(t)) + ws::naf_goals(bm),
            spos == spec::pos_terms(bm.filter(|it: BodyItem| it is Pos)),
            a <= others.len(),
            forall|x: int, y: int|
                0 <= x < a && 0 <= y < outs.len() ==> !ws::mentions(
                    #[trigger] models(others@)[x],
                    #[trigger] models(outs@)[y],
                ),
        decreases others.len() - a,
    {
        let mut b = 0usize;
        while b < outs.len()
            invariant
                arena_ok(arena),
                valid_all(arena.nodes@, others@),
                valid_all(arena.nodes@, outs@),
                a < others.len(),
                models(outs@) == ws::outputs(sel@),
                bm == body_models(body@),
                models(others@) == spos.filter(|t: Term| !ws::is_arith_lit(t)) + ws::naf_goals(bm),
                spos == spec::pos_terms(bm.filter(|it: BodyItem| it is Pos)),
                b <= outs.len(),
                forall|x: int, y: int|
                    0 <= x < a && 0 <= y < outs.len() ==> !ws::mentions(
                        #[trigger] models(others@)[x],
                        #[trigger] models(outs@)[y],
                    ),
                forall|y: int|
                    0 <= y < b ==> !ws::mentions(
                        models(others@)[a as int],
                        #[trigger] models(outs@)[y],
                    ),
            decreases outs.len() - b,
        {
            proof {
                assert(valid(arena.nodes@, &others@[a as int]));
                assert(valid(arena.nodes@, &outs@[b as int]));
                assert(models(others@)[a as int] == others@[a as int]@);
                assert(models(outs@)[b as int] == outs@[b as int]@);
            }
            if mentions(arena, &others[a], &outs[b]) {
                proof {
                    assert(ws::mentions(models(others@)[a as int], ws::outputs(sel@)[b as int]));
                    assert(!ws::chain_body(sel@, bm));
                }
                return false;
            }
            b += 1;
        }
        a += 1;
    }
    true
}

// --- the walk (`ws::walk_ok`) ---
// What the walk reads beside the program (`ws::WalkEnv`).
pub struct EWalk {
    pub facts: Vec<T>,
    pub slots: Vec<T>,
    pub bounds: Vec<Vec<T>>,
    pub chain: Vec<T>,
    pub off: usize,
}

pub open spec fn walk_model(e: &EWalk, p: Seq<DocClause>) -> ws::WalkEnv {
    ws::WalkEnv {
        p,
        facts: models(e.facts@),
        slots: models(e.slots@),
        bounds: seqs_models(e.bounds@),
        chain: models(e.chain@),
        off: e.off as nat,
    }
}

pub open spec fn walk_valid(nodes: Seq<ENode>, e: &EWalk, db: Seq<EClause>) -> bool {
    &&& valid_all(nodes, e.facts@)
    &&& valid_all(nodes, e.slots@)
    &&& seqs_valid(nodes, e.bounds@)
    &&& e.bounds.len() == e.slots.len()
    &&& valid_all(nodes, e.chain@)
    &&& arith_goals(e.chain@)
    &&& db_valid(nodes, db)
    &&& e.off <= nodes.len()
}

proof fn walk_prefix(before: Seq<ENode>, after: Seq<ENode>, e: &EWalk, db: Seq<EClause>)
    requires
        before.is_prefix_of(after),
        walk_valid(before, e, db),
    ensures
        walk_valid(after, e, db),
        db_view(before, db) == db_view(after, db),
{
    prefix_all(before, after, e.facts@);
    prefix_all(before, after, e.slots@);
    crate::m6_cands::seqs_prefix(before, after, e.bounds@);
    prefix_all(before, after, e.chain@);
    crate::k2_engine::db_models_prefix(before, after, db);
}

pub fn slot_free(arena: &ETermArena, slots: &Vec<T>, t: &T) -> (out: bool)
    requires
        arena_ok(arena),
        valid_all(arena.nodes@, slots@),
        valid(arena.nodes@, t),
    ensures
        out == (forall|i: int|
            0 <= i < slots.len() ==> !ws::mentions(t@, #[trigger] models(slots@)[i])),
{
    let mut i = 0usize;
    while i < slots.len()
        invariant
            arena_ok(arena),
            valid_all(arena.nodes@, slots@),
            valid(arena.nodes@, t),
            i <= slots.len(),
            forall|j: int| 0 <= j < i ==> !ws::mentions(t@, #[trigger] models(slots@)[j]),
        decreases slots.len() - i,
    {
        proof {
            assert(valid(arena.nodes@, &slots@[i as int]));
            assert(models(slots@)[i as int] == slots@[i as int]@);
        }
        if mentions(arena, t, &slots[i]) {
            return false;
        }
        i += 1;
    }
    true
}

fn holds(arena: &ETermArena, bs: &Vec<T>, x: &T) -> (out: bool)
    requires
        arena_ok(arena),
        valid_all(arena.nodes@, bs@),
        valid(arena.nodes@, x),
    ensures
        out == models(bs@).contains(x@),
{
    let mut i = 0usize;
    while i < bs.len()
        invariant
            arena_ok(arena),
            valid_all(arena.nodes@, bs@),
            valid(arena.nodes@, x),
            i <= bs.len(),
            forall|j: int| 0 <= j < i ==> #[trigger] models(bs@)[j] != x@,
        decreases bs.len() - i,
    {
        proof {
            assert(valid(arena.nodes@, &bs@[i as int]));
            assert(models(bs@)[i as int] == bs@[i as int]@);
        }
        if crate::m6_term::equal(arena, &bs[i], x) {
            return true;
        }
        i += 1;
    }
    false
}

// `ws::test_ok`: a built-in reads a slot only as a comparison's count input
// against a number of that slot's B.
fn test_ok(arena: &ETermArena, e: &EWalk, c: &T, Ghost(we): Ghost<ws::WalkEnv>) -> (out: bool)
    requires
        arena_ok(arena),
        valid_all(arena.nodes@, e.slots@),
        seqs_valid(arena.nodes@, e.bounds@),
        e.bounds.len() == e.slots.len(),
        valid(arena.nodes@, c),
        we.slots == models(e.slots@),
        we.bounds == seqs_models(e.bounds@),
    ensures
        out == ws::test_ok(we, c@),
{
    if slot_free(arena, &e.slots, c) {
        proof {
            assert forall|i: int| 0 <= i < we.slots.len() implies !ws::mentions(
                c@,
                #[trigger] we.slots[i],
            ) by {
                assert(we.slots[i] == models(e.slots@)[i]);
            }
            assert(ws::slot_free(we, c@));
        }
        return true;
    }
    proof {
        reveal_strlit("guideline_satisfies");
    }
    if !is_comp(arena, c, &Sym::GuidelineSatisfies, 4) {
        return false;
    }
    let xs = args(arena, c);
    proof {
        arg_at(c@, xs@, 0);
        arg_at(c@, xs@, 1);
        arg_at(c@, xs@, 2);
        arg_at(c@, xs@, 3);
        assert(valid(arena.nodes@, &xs@[0]));
        assert(valid(arena.nodes@, &xs@[1]));
        assert(valid(arena.nodes@, &xs@[2]));
        assert(valid(arena.nodes@, &xs@[3]));
    }
    if !slot_free(arena, &e.slots, &xs[0]) || !slot_free(arena, &e.slots, &xs[2]) {
        proof {
            assert(!ws::slot_free(we, arg(c@, 0)) || !ws::slot_free(we, arg(c@, 2)));
        }
        return false;
    }
    proof {
        assert(spec::is_comp(c@, "guideline_satisfies"@, 4));
        assert forall|i: int| 0 <= i < we.slots.len() implies !ws::mentions(
            arg(c@, 0),
            #[trigger] we.slots[i],
        ) && !ws::mentions(arg(c@, 2), we.slots[i]) by {
            assert(we.slots[i] == models(e.slots@)[i]);
        }
        assert(ws::slot_free(we, arg(c@, 0)) && ws::slot_free(we, arg(c@, 2)));
    }
    let mut i = 0usize;
    while i < e.slots.len()
        invariant
            arena_ok(arena),
            valid_all(arena.nodes@, e.slots@),
            seqs_valid(arena.nodes@, e.bounds@),
            e.bounds.len() == e.slots.len(),
            valid_all(arena.nodes@, xs@),
            xs.len() == 4,
            we.slots == models(e.slots@),
            we.bounds == seqs_models(e.bounds@),
            xs@[1]@ == arg(c@, 1),
            xs@[3]@ == arg(c@, 3),
            spec::is_comp(c@, "guideline_satisfies"@, 4),
            ws::slot_free(we, arg(c@, 0)),
            ws::slot_free(we, arg(c@, 2)),
            i <= e.slots.len(),
            forall|j: int|
                0 <= j < i ==> !(arg(c@, 1) == #[trigger] we.slots[j] && we.bounds[j].contains(
                    arg(c@, 3),
                )),
        decreases e.slots.len() - i,
    {
        proof {
            assert(valid(arena.nodes@, &e.slots@[i as int]));
            assert(models(e.slots@)[i as int] == e.slots@[i as int]@);
            assert(valid_all(arena.nodes@, e.bounds@[i as int]@));
            assert(seqs_models(e.bounds@)[i as int] == models(e.bounds@[i as int]@));
        }
        if crate::m6_term::equal(arena, &xs[1], &e.slots[i]) && holds(arena, &e.bounds[i], &xs[3]) {
            proof {
                assert(arg(c@, 1) == we.slots[i as int]);
                assert(we.bounds[i as int].contains(arg(c@, 3)));
            }
            return true;
        }
        i += 1;
    }
    false
}

// `ws::input_ok` for input pair q (comparator q, count q + 1) of the walked
// arithmetic goal g (arguments gx) against its chain node c (arguments cx).
fn input_ok(
    arena: &ETermArena,
    e: &EWalk,
    gx: &Vec<T>,
    cx: &Vec<T>,
    q: usize,
    Ghost(we): Ghost<ws::WalkEnv>,
    Ghost(g): Ghost<Term>,
    Ghost(c): Ghost<Term>,
) -> (out: bool)
    requires
        arena_ok(arena),
        valid_all(arena.nodes@, e.slots@),
        valid_all(arena.nodes@, gx@),
        valid_all(arena.nodes@, cx@),
        models(gx@) == ckc_spec::engine::args_of(g),
        models(cx@) == ckc_spec::engine::args_of(c),
        gx.len() == 6,
        cx.len() == 6,
        q == 0 || q == 2,
        we.slots == models(e.slots@),
    ensures
        out == ws::input_ok(we, g, c, q as int),
{
    proof {
        arg_at(g, gx@, q as int);
        arg_at(g, gx@, q as int + 1);
        arg_at(c, cx@, q as int);
        arg_at(c, cx@, q as int + 1);
        assert(valid(arena.nodes@, &gx@[q as int]));
        assert(valid(arena.nodes@, &gx@[q as int + 1]));
        assert(valid(arena.nodes@, &cx@[q as int]));
        assert(valid(arena.nodes@, &cx@[q as int + 1]));
        reveal_strlit("exactly");
    }
    if holds(arena, &e.slots, &cx[q + 1]) {
        crate::m6_term::equal(arena, &gx[q + 1], &cx[q + 1]) && is_atom(
            arena,
            &gx[q],
            &Sym::Exactly,
        )
    } else if is_var(arena, &cx[q + 1]) {
        is_var(arena, &gx[q]) && is_var(arena, &gx[q + 1])
    } else {
        crate::m6_term::equal(arena, &gx[q], &cx[q]) && crate::m6_term::equal(
            arena,
            &gx[q + 1],
            &cx[q + 1],
        )
    }
}

// `ws::arith_ok`: the walk's arithmetic goal g = its chain node c.
fn arith_ok(arena: &ETermArena, e: &EWalk, g: &T, c: &T, Ghost(we): Ghost<ws::WalkEnv>) -> (out:
    bool)
    requires
        arena_ok(arena),
        valid_all(arena.nodes@, e.slots@),
        valid(arena.nodes@, g),
        valid(arena.nodes@, c),
        ws::is_arith_lit(g@),
        ws::is_arith_lit(c@),
        we.slots == models(e.slots@),
    ensures
        out == ws::arith_ok(we, g@, c@),
{
    let (gn, gx) = match parts(arena, g) {
        Some(p) => p,
        None => {
            return false;
        },
    };
    let (cn, cx) = match parts(arena, c) {
        Some(p) => p,
        None => {
            return false;
        },
    };
    proof {
        reveal(ckc_spec::arith::is_arith);
        assert(gx.len() == 6 && cx.len() == 6);
    }
    if !crate::m6_term::bytes_eq(&gn, &cn) {
        return false;
    }
    if !input_ok(arena, e, &gx, &cx, 0, Ghost(we), Ghost(g@), Ghost(c@)) {
        return false;
    }
    if !input_ok(arena, e, &gx, &cx, 2, Ghost(we), Ghost(g@), Ghost(c@)) {
        return false;
    }
    proof {
        arg_at(g@, gx@, 4);
        arg_at(g@, gx@, 5);
        assert(valid(arena.nodes@, &gx@[4]));
        assert(valid(arena.nodes@, &gx@[5]));
    }
    is_var(arena, &gx[4]) && is_var(arena, &gx[5])
}

fn unifiable(arena: &mut ETermArena, a: usize, b: usize) -> (out: bool)
    requires
        arena_ok(old(arena)),
        a < old(arena).nodes@.len(),
        b < old(arena).nodes@.len(),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out == ws::unifiable(old(arena)@[a as int], old(arena)@[b as int]),
{
    let mut pairs: Vec<crate::k2_engine::EPair> = Vec::new();
    pairs.push(crate::k2_engine::EPair { left: a, right: b });
    let st = crate::k2_engine::EBoundState { pairs, stack: Vec::new(), sol: Vec::new() };
    let ghost u = ckc_spec::engine::UState {
        pairs: seq![(arena@[a as int], arena@[b as int])],
        stack: Seq::empty(),
        sol: Seq::empty(),
    };
    proof {
        assert(crate::k2_engine::bound_state_valid(arena.nodes@, &st)) by {
            reveal(crate::k2_engine::roots_valid);
        }
        assert(crate::k2_engine::pairs_view(arena.nodes@, st.pairs@) =~= seq![
            (arena@[a as int], arena@[b as int]),
        ]);
        assert(crate::k2_engine::goals_view(arena.nodes@, st.stack@) =~= Seq::<
            ckc_spec::engine::Goal,
        >::empty());
        assert(crate::k2_engine::root_terms(arena.nodes@, st.sol@) =~= Seq::<Term>::empty());
        assert(crate::k2_engine::bound_state_view(arena.nodes@, &st) == u);
    }
    match crate::k2_engine::unify(arena, st) {
        crate::k2_engine::EUResult::Ok { .. } => true,
        crate::k2_engine::EUResult::Fail => false,
    }
}

// --- goals the walk keeps: control nodes over compound literals ---
// Unification only fills variables, so it keeps this shape and each goal's node
// count; the walk's fuel (1 + the body's nodes) then never runs out.
pub open spec fn structured(t: Term) -> bool
    decreases t,
{
    match t {
        Term::Comp(name, args) => if name == ckc_spec::engine::comma_name() && args.len() == 2 {
            structured(args[0]) && structured(args[1])
        } else if name == ckc_spec::engine::naf_name() && args.len() == 1 {
            structured(args[0])
        } else {
            true
        },
        _ => false,
    }
}

pub open spec fn structured_upto(ts: Seq<Term>, m: int) -> bool {
    forall|i: int| 0 <= i < m ==> structured(#[trigger] ts[i])
}

proof fn subst_structured(t: Term, x: nat, v: Term)
    requires
        structured(t),
    ensures
        structured(ckc_spec::engine::subst(t, x, v)),
        ws::goal_nodes(ckc_spec::engine::subst(t, x, v)) == ws::goal_nodes(t),
    decreases t,
{
    reveal_with_fuel(ws::goal_nodes, 1);
    match t {
        Term::Comp(name, args) => {
            crate::k3_sound::naf::subst_all_index(args, x, v);
            let a2 = ckc_spec::engine::subst_all(args, x, v);
            assert(ckc_spec::engine::subst(t, x, v) == Term::Comp(name, a2));
            if name == ckc_spec::engine::comma_name() && args.len() == 2 {
                subst_structured(args[0], x, v);
                subst_structured(args[1], x, v);
                assert(a2[0] == ckc_spec::engine::subst(args[0], x, v));
                assert(a2[1] == ckc_spec::engine::subst(args[1], x, v));
            } else if name == ckc_spec::engine::naf_name() && args.len() == 1 {
                subst_structured(args[0], x, v);
                assert(a2[0] == ckc_spec::engine::subst(args[0], x, v));
            }
        },
        _ => {},
    }
}

proof fn subst_all_structured(ts: Seq<Term>, x: nat, v: Term, m: int)
    requires
        0 <= m <= ts.len(),
        structured_upto(ts, m),
    ensures
        ckc_spec::engine::subst_all(ts, x, v).len() == ts.len(),
        structured_upto(ckc_spec::engine::subst_all(ts, x, v), m),
        forall|i: int|
            0 <= i < m ==> ws::goal_nodes(#[trigger] ckc_spec::engine::subst_all(ts, x, v)[i])
                == ws::goal_nodes(ts[i]),
{
    crate::k3_sound::naf::subst_all_index(ts, x, v);
    assert forall|i: int| 0 <= i < m implies structured(
        #[trigger] ckc_spec::engine::subst_all(ts, x, v)[i],
    ) && ws::goal_nodes(ckc_spec::engine::subst_all(ts, x, v)[i]) == ws::goal_nodes(ts[i]) by {
        assert(structured(ts[i]));
        subst_structured(ts[i], x, v);
    }
}

proof fn unify_n_structured(u: ckc_spec::engine::UState, f: nat, m: int)
    requires
        0 <= m <= u.sol.len(),
        structured_upto(u.sol, m),
    ensures
        ckc_spec::engine::unify_n(u, f) matches ckc_spec::engine::UOut::Ok(_, s) ==> {
            &&& s.len() == u.sol.len()
            &&& structured_upto(s, m)
            &&& forall|i: int|
                0 <= i < m ==> ws::goal_nodes(#[trigger] s[i]) == ws::goal_nodes(u.sol[i])
        },
    decreases f,
{
    reveal_with_fuel(ckc_spec::engine::unify_n, 1);
    if f == 0 || u.pairs.len() == 0 {
        return;
    }
    let a = u.pairs[0].0;
    let b = u.pairs[0].1;
    let rest = u.pairs.drop_first();
    let f1 = (f - 1) as nat;
    match (a, b) {
        (Term::Var(x), _) => {
            if a == b {
                unify_n_structured(ckc_spec::engine::UState { pairs: rest, ..u }, f1, m);
            } else if !ckc_spec::engine::occurs(x, b) {
                let u2 = ckc_spec::engine::u_bind(u, rest, x, b);
                subst_all_structured(u.sol, x, b, m);
                unify_n_structured(u2, f1, m);
            }
        },
        (_, Term::Var(y)) => {
            if !ckc_spec::engine::occurs(y, a) {
                let u2 = ckc_spec::engine::u_bind(u, rest, y, a);
                subst_all_structured(u.sol, y, a, m);
                unify_n_structured(u2, f1, m);
            }
        },
        (Term::Comp(n1, xs), Term::Comp(n2, ys)) => {
            if n1 == n2 && xs.len() == ys.len() {
                unify_n_structured(
                    ckc_spec::engine::UState { pairs: ckc_spec::engine::zip(xs, ys) + rest, ..u },
                    f1,
                    m,
                );
            }
        },
        _ => {
            if a == b {
                unify_n_structured(ckc_spec::engine::UState { pairs: rest, ..u }, f1, m);
            }
        },
    }
}

pub proof fn unify_structured(u: ckc_spec::engine::UState, m: int)
    requires
        0 <= m <= u.sol.len(),
        structured_upto(u.sol, m),
    ensures
        ckc_spec::engine::unify(u) matches ckc_spec::engine::UOut::Ok(_, s) ==> {
            &&& s.len() == u.sol.len()
            &&& structured_upto(s, m)
            &&& forall|i: int|
                0 <= i < m ==> ws::goal_nodes(#[trigger] s[i]) == ws::goal_nodes(u.sol[i])
        },
{
    if exists|f: nat| !(ckc_spec::engine::unify_n(u, f) is Out) {
        let f = choose|f: nat| !(ckc_spec::engine::unify_n(u, f) is Out);
        unify_n_structured(u, f, m);
    }
}

proof fn goal_nodes_pos(t: Term)
    ensures
        ws::goal_nodes(t) >= 1,
{
    reveal_with_fuel(ws::goal_nodes, 1);
}

proof fn nodes_all_cons(t: Term, ts: Seq<Term>)
    ensures
        ws::nodes_all(seq![t] + ts) == ws::goal_nodes(t) + ws::nodes_all(ts),
{
    reveal_with_fuel(ws::nodes_all, 1);
    assert((seq![t] + ts).drop_first() =~= ts);
    assert((seq![t] + ts)[0] == t);
}

proof fn nodes_all_eq(a: Seq<Term>, b: Seq<Term>)
    requires
        a.len() == b.len(),
        forall|i: int| 0 <= i < a.len() ==> ws::goal_nodes(#[trigger] a[i]) == ws::goal_nodes(b[i]),
    ensures
        ws::nodes_all(a) == ws::nodes_all(b),
    decreases a.len(),
{
    reveal_with_fuel(ws::nodes_all, 1);
    if a.len() > 0 {
        assert forall|i: int| 0 <= i < a.drop_first().len() implies ws::goal_nodes(
            #[trigger] a.drop_first()[i],
        ) == ws::goal_nodes(b.drop_first()[i]) by {
            assert(a.drop_first()[i] == a[i + 1]);
            assert(b.drop_first()[i] == b[i + 1]);
        }
        nodes_all_eq(a.drop_first(), b.drop_first());
    }
}

fn take(ts: &Vec<T>, m: usize) -> (out: Vec<T>)
    requires
        m <= ts.len(),
    ensures
        out@ == ts@.take(m as int),
        models(out@) == models(ts@).take(m as int),
{
    let mut out: Vec<T> = Vec::new();
    let mut i = 0usize;
    while i < m
        invariant
            i <= m,
            m <= ts.len(),
            out@ == ts@.take(i as int),
        decreases m - i,
    {
        out.push(ts[i].cp());
        proof {
            assert(ts@.take(i as int + 1) == ts@.take(i as int).push(ts@[i as int]));
        }
        i += 1;
    }
    proof {
        assert(models(out@) =~= models(ts@).take(m as int));
    }
    out
}

// The terms of hs from m on equal ts (`hs.skip(m) == ts`).
fn same_tail(arena: &ETermArena, hs: &Vec<T>, m: usize, ts: &Vec<T>) -> (out: bool)
    requires
        arena_ok(arena),
        valid_all(arena.nodes@, hs@),
        valid_all(arena.nodes@, ts@),
        m <= hs.len(),
    ensures
        out == (models(hs@).skip(m as int) == models(ts@)),
{
    if hs.len() - m != ts.len() {
        proof {
            assert(models(hs@).skip(m as int).len() != models(ts@).len());
        }
        return false;
    }
    let mut q = 0usize;
    while q < ts.len()
        invariant
            arena_ok(arena),
            valid_all(arena.nodes@, hs@),
            valid_all(arena.nodes@, ts@),
            m <= hs.len(),
            hs.len() - m == ts.len(),
            q <= ts.len(),
            forall|i: int| 0 <= i < q ==> models(hs@)[m + i] == #[trigger] models(ts@)[i],
        decreases ts.len() - q,
    {
        proof {
            assert(valid(arena.nodes@, &hs@[m + q]));
            assert(valid(arena.nodes@, &ts@[q as int]));
            assert(models(hs@)[m + q] == hs@[m + q]@);
            assert(models(ts@)[q as int] == ts@[q as int]@);
        }
        if !crate::m6_term::equal(arena, &hs[m + q], &ts[q]) {
            proof {
                assert(models(hs@).skip(m as int)[q as int] != models(ts@)[q as int]);
            }
            return false;
        }
        q += 1;
    }
    proof {
        assert(models(hs@).skip(m as int) =~= models(ts@));
    }
    true
}

pub fn walk_ok(
    arena: &mut ETermArena,
    e: &EWalk,
    db: &Vec<EClause>,
    gs: &Vec<T>,
    k: usize,
    Ghost(n): Ghost<nat>,
    Ghost(we): Ghost<ws::WalkEnv>,
) -> (out: bool)
    requires
        arena_ok(old(arena)),
        walk_valid(old(arena).nodes@, e, db@),
        we == walk_model(e, db_view(old(arena).nodes@, db@)),
        valid_all(old(arena).nodes@, gs@),
        structured_upto(models(gs@), gs.len() as int),
        ws::nodes_all(models(gs@)) <= n,
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out == ws::walk_ok(we, models(gs@), n, k as nat),
    decreases n, 0nat,
{
    let ghost start = arena.nodes@;
    let ghost gm = models(gs@);
    proof {
        reveal_with_fuel(ws::walk_ok, 1);
    }
    if gs.len() == 0 {
        return true;
    }
    proof {
        nodes_all_cons(gm[0], gm.drop_first());
        assert(gm == seq![gm[0]] + gm.drop_first());
        goal_nodes_pos(gm[0]);
        assert(structured(gm[0]));
        assert(valid(arena.nodes@, &gs@[0]));
        assert(gm[0] == gs@[0]@);
    }
    let g = gs[0].cp();
    let rest = tail(gs, 1);
    proof {
        assert(models(rest@) == gm.drop_first());
        assert forall|i: int| 0 <= i < rest.len() implies #[trigger] valid(
            arena.nodes@,
            &rest@[i],
        ) by {
            assert(rest@[i] == gs@[i + 1]);
        }
        assert forall|i: int| 0 <= i < rest.len() implies structured(
            #[trigger] models(rest@)[i],
        ) by {
            assert(models(rest@)[i] == gm[i + 1]);
        }
    }
    match parts(arena, &g) {
        None => true,
        Some((name, xs)) => {
            proof {
                assert(g@ == Term::Comp(name@, models(xs@)));
                reveal_with_fuel(ws::goal_nodes, 1);
            }
            if is_comma_name(&name) && xs.len() == 2 {
                let mut next: Vec<T> = Vec::new();
                next.push(xs[0].cp());
                next.push(xs[1].cp());
                let next = concat(&next, &rest);
                proof {
                    let a0 = models(xs@)[0];
                    let a1 = models(xs@)[1];
                    assert(xs@[0]@ == a0 && xs@[1]@ == a1);
                    assert(models(next@) =~= seq![a0, a1] + gm.drop_first());
                    assert(seq![a0, a1] + gm.drop_first() =~= seq![a0] + (seq![a1]
                        + gm.drop_first()));
                    nodes_all_cons(a0, seq![a1] + gm.drop_first());
                    nodes_all_cons(a1, gm.drop_first());
                    assert forall|i: int| 0 <= i < next.len() implies #[trigger] valid(
                        arena.nodes@,
                        &next@[i],
                    ) by {
                        if i == 0 {
                            assert(next@[0] == xs@[0]);
                        } else if i == 1 {
                            assert(next@[1] == xs@[1]);
                        } else {
                            assert(next@[i] == rest@[i - 2]);
                        }
                    }
                    assert forall|i: int| 0 <= i < next.len() implies structured(
                        #[trigger] models(next@)[i],
                    ) by {
                        if i >= 2 {
                            assert(models(next@)[i] == models(rest@)[i - 2]);
                        }
                    }
                }
                walk_ok(arena, e, db, &next, k, Ghost((n - 1) as nat), Ghost(we))
            } else if is_naf_name(&name) && xs.len() == 1 {
                let mut inner: Vec<T> = Vec::new();
                inner.push(xs[0].cp());
                proof {
                    let a0 = models(xs@)[0];
                    assert(xs@[0]@ == a0);
                    assert(models(inner@) =~= seq![a0]);
                    nodes_all_cons(a0, Seq::<Term>::empty());
                    assert(seq![a0] + Seq::<Term>::empty() =~= seq![a0]);
                    reveal_with_fuel(ws::nodes_all, 1);
                }
                let ghost before = arena.nodes@;
                let a = walk_ok(arena, e, db, &inner, k, Ghost((n - 1) as nat), Ghost(we));
                if !a {
                    return false;
                }
                proof {
                    walk_prefix(before, arena.nodes@, e, db@);
                    prefix_all(before, arena.nodes@, rest@);
                }
                walk_ok(arena, e, db, &rest, k, Ghost((n - 1) as nat), Ghost(we))
            } else if crate::compare_impl::is_arith_exec(&name, xs.len()) {
                if k >= e.chain.len() {
                    return false;
                }
                proof {
                    assert(valid(arena.nodes@, &e.chain@[k as int]));
                    assert(we.chain[k as int] == e.chain@[k as int]@);
                    assert(ws::is_arith_lit(g@));
                }
                if !arith_ok(arena, e, &g, &e.chain[k], Ghost(we)) {
                    return false;
                }
                walk_ok(arena, e, db, &rest, k + 1, Ghost((n - 1) as nat), Ghost(we))
            } else if crate::compare_impl::is_builtin_exec(&name, xs.len()) {
                if !test_ok(arena, e, &g, Ghost(we)) {
                    return false;
                }
                walk_ok(arena, e, db, &rest, k, Ghost((n - 1) as nat), Ghost(we))
            } else if has_name(&name, &Sym::GuidelineOperator) {
                false
            } else {
                call_ok(arena, e, db, &g, &rest, k, Ghost((n - 1) as nat), Ghost(we))
            }
        },
    }
}

// `ws::call_ok`: no rule of the program resolves the call; every fact match
// keeps the slots and continues the walk.
fn call_ok(
    arena: &mut ETermArena,
    e: &EWalk,
    db: &Vec<EClause>,
    g: &T,
    rest: &Vec<T>,
    k: usize,
    Ghost(n): Ghost<nat>,
    Ghost(we): Ghost<ws::WalkEnv>,
) -> (out: bool)
    requires
        arena_ok(old(arena)),
        walk_valid(old(arena).nodes@, e, db@),
        we == walk_model(e, db_view(old(arena).nodes@, db@)),
        valid(old(arena).nodes@, g),
        valid_all(old(arena).nodes@, rest@),
        structured_upto(models(rest@), rest.len() as int),
        ws::nodes_all(models(rest@)) <= n,
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out == ws::call_ok(we, g@, models(rest@), n, k as nat),
    decreases n, e.facts.len() + 2,
{
    let ghost start = arena.nodes@;
    let ghost p = db_view(arena.nodes@, db@);
    let mut ci = 0usize;
    while ci < db.len()
        invariant
            arena_ok(arena),
            start == old(arena).nodes@,
            start.is_prefix_of(arena.nodes@),
            walk_valid(arena.nodes@, e, db@),
            p == db_view(start, db@),
            p == db_view(arena.nodes@, db@),
            we == walk_model(e, p),
            valid(arena.nodes@, g),
            valid_all(arena.nodes@, rest@),
            ci <= db.len(),
            forall|j: int|
                0 <= j < ci && !ws::ground_fact(#[trigger] p[j]) ==> !ws::unifiable(
                    g@,
                    ckc_spec::engine::shift(p[j].head, we.off),
                ),
        decreases db.len() - ci,
    {
        let c = &db[ci];
        proof {
            assert(crate::k2_engine::clause_valid(arena.nodes@, &db@[ci as int]));
            assert(p[ci as int] == crate::k2_engine::clause_view(arena.nodes@, &db@[ci as int]));
            assert(arena@[c.head as int] == p[ci as int].head);
            assert(p[ci as int].body.len() == c.body.len());
        }
        let fact = c.body.len() == 0 && crate::k2_walk::ground_root(arena, c.head);
        if !fact {
            let ghost here = arena.nodes@;
            let sh = crate::k2_engine::shift_root(arena, c.head, e.off);
            proof {
                walk_prefix(here, arena.nodes@, e, db@);
                prefix(here, arena.nodes@, g);
                prefix_all(here, arena.nodes@, rest@);
                crate::k2_load::prefix_chain(start, here, arena.nodes@);
                assert(arena@[g.root as int] == g@);
            }
            let ghost here2 = arena.nodes@;
            let u = unifiable(arena, g.root, sh);
            proof {
                walk_prefix(here2, arena.nodes@, e, db@);
                prefix(here2, arena.nodes@, g);
                prefix_all(here2, arena.nodes@, rest@);
                crate::k2_load::prefix_chain(start, here2, arena.nodes@);
            }
            if u {
                return false;
            }
        }
        ci += 1;
    }
    facts_ok(arena, e, db, g, rest, k, Ghost(n), Ghost(we))
}

fn facts_ok(
    arena: &mut ETermArena,
    e: &EWalk,
    db: &Vec<EClause>,
    g: &T,
    rest: &Vec<T>,
    k: usize,
    Ghost(n): Ghost<nat>,
    Ghost(we): Ghost<ws::WalkEnv>,
) -> (out: bool)
    requires
        arena_ok(old(arena)),
        walk_valid(old(arena).nodes@, e, db@),
        we == walk_model(e, db_view(old(arena).nodes@, db@)),
        valid(old(arena).nodes@, g),
        valid_all(old(arena).nodes@, rest@),
        structured_upto(models(rest@), rest.len() as int),
        ws::nodes_all(models(rest@)) <= n,
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out == ws::facts_ok(we, g@, models(rest@), n, k as nat, 0),
    decreases n, e.facts.len() + 1,
{
    let ghost start = arena.nodes@;
    let ghost p = db_view(arena.nodes@, db@);
    let ghost rm = models(rest@);
    let all = concat(rest, &e.slots);
    proof {
        assert forall|i: int| 0 <= i < all.len() implies #[trigger] valid(
            arena.nodes@,
            &all@[i],
        ) by {
            if i < rest.len() {
                assert(all@[i] == rest@[i]);
            } else {
                assert(all@[i] == e.slots@[i - rest.len()]);
            }
        }
    }
    let mut j = 0usize;
    while j < e.facts.len()
        invariant
            arena_ok(arena),
            start == old(arena).nodes@,
            start.is_prefix_of(arena.nodes@),
            walk_valid(arena.nodes@, e, db@),
            p == db_view(start, db@),
            p == db_view(arena.nodes@, db@),
            we == walk_model(e, p),
            valid(arena.nodes@, g),
            valid_all(arena.nodes@, rest@),
            valid_all(arena.nodes@, all@),
            rm == models(rest@),
            models(all@) == rm + models(e.slots@),
            structured_upto(rm, rest.len() as int),
            ws::nodes_all(rm) <= n,
            j <= e.facts.len(),
            ws::facts_ok(we, g@, rm, n, k as nat, 0) == ws::facts_ok(
                we,
                g@,
                rm,
                n,
                k as nat,
                j as nat,
            ),
        decreases e.facts.len() - j,
    {
        proof {
            reveal_with_fuel(ws::facts_ok, 1);
            assert(valid(arena.nodes@, &e.facts@[j as int]));
            assert(we.facts[j as int] == e.facts@[j as int]@);
        }
        let ghost here = arena.nodes@;
        let sol = roots(arena, &all);
        let mut pairs: Vec<crate::k2_engine::EPair> = Vec::new();
        pairs.push(crate::k2_engine::EPair { left: g.root, right: e.facts[j].root });
        let st = crate::k2_engine::EBoundState { pairs, stack: Vec::new(), sol };
        let ghost u = ckc_spec::engine::UState {
            pairs: seq![(g@, we.facts[j as int])],
            stack: Seq::empty(),
            sol: rm + we.slots,
        };
        proof {
            assert(crate::k2_engine::bound_state_valid(arena.nodes@, &st)) by {
                reveal(crate::k2_engine::roots_valid);
            }
            assert(crate::k2_engine::pairs_view(arena.nodes@, st.pairs@) =~= seq![
                (g@, we.facts[j as int]),
            ]);
            assert(crate::k2_engine::goals_view(arena.nodes@, st.stack@) =~= Seq::<
                ckc_spec::engine::Goal,
            >::empty());
            assert(crate::k2_engine::bound_state_view(arena.nodes@, &st) == u);
            crate::k3_sound::naf::unify_output_len(u);
            assert forall|i: int| 0 <= i < rest.len() implies structured(#[trigger] u.sol[i]) by {
                assert(u.sol[i] == rm[i]);
            }
            unify_structured(u, rest.len() as int);
        }
        let res = crate::k2_engine::unify(arena, st);
        proof {
            walk_prefix(here, arena.nodes@, e, db@);
            prefix(here, arena.nodes@, g);
            prefix_all(here, arena.nodes@, rest@);
            prefix_all(here, arena.nodes@, all@);
            crate::k2_load::prefix_chain(start, here, arena.nodes@);
        }
        match res {
            crate::k2_engine::EUResult::Ok { stack: _, sol: s } => {
                let hs = from_roots(arena, &s);
                let ghost sm = models(hs@);
                proof {
                    assert(sm.len() == rm.len() + we.slots.len());
                }
                if !same_tail(arena, &hs, rest.len(), &e.slots) {
                    return false;
                }
                let r2 = take(&hs, rest.len());
                proof {
                    assert forall|i: int| 0 <= i < r2.len() implies #[trigger] valid(
                        arena.nodes@,
                        &r2@[i],
                    ) by {
                        assert(r2@[i] == hs@[i]);
                    }
                    assert forall|i: int| 0 <= i < r2.len() implies structured(
                        #[trigger] models(r2@)[i],
                    ) && ws::goal_nodes(models(r2@)[i]) == ws::goal_nodes(rm[i]) by {
                        assert(models(r2@)[i] == sm[i]);
                        assert(u.sol[i] == rm[i]);
                    }
                    nodes_all_eq(models(r2@), rm);
                }
                let ghost here3 = arena.nodes@;
                let w = walk_ok(arena, e, db, &r2, k, Ghost(n), Ghost(we));
                proof {
                    walk_prefix(here3, arena.nodes@, e, db@);
                    prefix(here3, arena.nodes@, g);
                    prefix_all(here3, arena.nodes@, rest@);
                    prefix_all(here3, arena.nodes@, all@);
                    crate::k2_load::prefix_chain(start, here3, arena.nodes@);
                }
                if !w {
                    return false;
                }
            },
            crate::k2_engine::EUResult::Fail => {},
        }
        j += 1;
    }
    proof {
        reveal_with_fuel(ws::facts_ok, 1);
    }
    true
}

// --- the clause walk (`ws::clause_ok`) ---
proof fn renumber_all_index(ts: Seq<Term>, fs: Seq<nat>)
    ensures
        spec::renumber_all(ts, fs).len() == ts.len(),
        forall|i: int|
            0 <= i < ts.len() ==> #[trigger] spec::renumber_all(ts, fs)[i] == spec::renumber(
                ts[i],
                fs,
            ),
    decreases ts.len(),
{
    reveal_with_fuel(spec::renumber_all, 1);
    if ts.len() > 0 {
        renumber_all_index(ts.drop_first(), fs);
        assert forall|i: int| 0 <= i < ts.len() implies #[trigger] spec::renumber_all(ts, fs)[i]
            == spec::renumber(ts[i], fs) by {
            if i > 0 {
                assert(ts.drop_first()[i - 1] == ts[i]);
            }
        }
    }
}

proof fn renumber_primitive(t: Term, fs: Seq<nat>)
    requires
        ws::primitive(t),
    ensures
        ws::primitive(spec::renumber(t, fs)),
{
    reveal_with_fuel(spec::renumber, 1);
    match t {
        Term::Comp(_, args) => {
            renumber_all_index(args, fs);
        },
        _ => {},
    }
}

proof fn canon_primitive(c: DocClause)
    requires
        ws::primitive_body(c.body),
    ensures
        ws::primitive_body(spec::canon_clause(c).body),
{
    let fs = ckc_spec::term::firsts(
        ckc_spec::term::var_stream(c.head) + ckc_spec::v1text::body_var_stream(c.body),
        Set::empty(),
    );
    let cb = spec::canon_clause(c).body;
    assert(cb == c.body.map_values(|it: BodyItem| spec::renumber_item(it, fs)));
    assert forall|k: int| 0 <= k < cb.len() implies match #[trigger] cb[k] {
        BodyItem::Pos(t) => ws::primitive(t),
        BodyItem::Naf(gs) => forall|m: int| 0 <= m < gs.len() ==> ws::primitive(#[trigger] gs[m]),
    } by {
        assert(cb[k] == spec::renumber_item(c.body[k], fs));
        match c.body[k] {
            BodyItem::Pos(t) => {
                renumber_primitive(t, fs);
            },
            BodyItem::Naf(gs) => {
                renumber_all_index(gs, fs);
                assert forall|m: int| 0 <= m < gs.len() implies ws::primitive(
                    #[trigger] spec::renumber_all(gs, fs)[m],
                ) by {
                    renumber_primitive(gs[m], fs);
                }
            },
        }
    }
}

proof fn shift_primitive(t: Term, off: nat)
    requires
        ws::primitive(t),
    ensures
        ws::primitive(ckc_spec::engine::shift(t, off)),
        structured(ckc_spec::engine::shift(t, off)),
{
    reveal_with_fuel(ckc_spec::engine::shift, 1);
    match t {
        Term::Comp(_, args) => {
            crate::k3_sound::naf::shift_all_len(args, off);
        },
        _ => {},
    }
}

proof fn primitive_structured(t: Term)
    requires
        ws::primitive(t),
    ensures
        structured(t),
{
    reveal_with_fuel(structured, 1);
}

proof fn conj_structured(gs: Seq<Term>)
    requires
        gs.len() > 0,
        forall|m: int| 0 <= m < gs.len() ==> ws::primitive(#[trigger] gs[m]),
    ensures
        structured(ckc_spec::engine::conj_term(gs)),
    decreases gs.len(),
{
    reveal_with_fuel(ckc_spec::engine::conj_term, 1);
    assert(ws::primitive(gs[0]));
    primitive_structured(gs[0]);
    if gs.len() > 1 {
        assert forall|m: int| 0 <= m < gs.drop_first().len() implies ws::primitive(
            #[trigger] gs.drop_first()[m],
        ) by {
            assert(gs.drop_first()[m] == gs[m + 1]);
        }
        conj_structured(gs.drop_first());
        let t = ckc_spec::engine::conj_term(gs);
        assert(t == Term::Comp(
            ckc_spec::engine::comma_name(),
            seq![gs[0], ckc_spec::engine::conj_term(gs.drop_first())],
        ));
        reveal_with_fuel(structured, 1);
    }
}

// A primitive item shifted is a structured goal.
proof fn item_structured(it: BodyItem, off: nat)
    requires
        match it {
            BodyItem::Pos(t) => ws::primitive(t),
            BodyItem::Naf(gs) => gs.len() > 0 && forall|m: int|
                0 <= m < gs.len() ==> ws::primitive(#[trigger] gs[m]),
        },
    ensures
        structured(ckc_spec::engine::item_term(it, off)),
{
    match it {
        BodyItem::Pos(t) => {
            shift_primitive(t, off);
        },
        BodyItem::Naf(gs) => {
            let sh = ckc_spec::engine::shift_all(gs, off);
            crate::k3_sound::shapes::shift_all_map(gs, off);
            assert forall|m: int| 0 <= m < sh.len() implies ws::primitive(#[trigger] sh[m]) by {
                shift_primitive(gs[m], off);
            }
            conj_structured(sh);
        },
    }
}

proof fn items_nvars_bound(items: Seq<BodyItem>, b: nat)
    requires
        forall|i: int|
            0 <= i < items.len() ==> crate::k2_engine::item_nvars(#[trigger] items[i]) <= b,
    ensures
        ckc_spec::engine::items_nvars(items) <= b,
    decreases items.len(),
{
    reveal_with_fuel(ckc_spec::engine::items_nvars, 1);
    if items.len() > 0 {
        assert(crate::k2_engine::item_nvars(items[0]) <= b);
        assert forall|i: int|
            0 <= i < items.drop_first().len() implies crate::k2_engine::item_nvars(
            #[trigger] items.drop_first()[i],
        ) <= b by {
            assert(items.drop_first()[i] == items[i + 1]);
        }
        items_nvars_bound(items.drop_first(), b);
    }
}

proof fn clauses_prefix_one(before: Seq<ENode>, after: Seq<ENode>, c: &Clause)
    requires
        before.is_prefix_of(after),
        clause_valid(before, c),
    ensures
        clause_valid(after, c),
{
    crate::m6_model::clauses_prefix(before, after, seq![*c]);
    assert(seq![*c][0] == *c);
}

// `clause_view(db[k]) == c`.
fn clause_eq(arena: &ETermArena, e: &EClause, c: &Clause) -> (out: bool)
    requires
        arena_ok(arena),
        crate::k2_engine::clause_valid(arena.nodes@, e),
        clause_valid(arena.nodes@, c),
    ensures
        out == (crate::k2_engine::clause_view(arena.nodes@, e) == c@),
{
    let ghost ev = crate::k2_engine::clause_view(arena.nodes@, e);
    let head = from_root(arena, e.head);
    if !crate::m6_term::equal(arena, &head, &c.head) {
        return false;
    }
    if e.body.len() != c.body.len() {
        proof {
            assert(ev.body.len() != c@.body.len());
        }
        return false;
    }
    let mut i = 0usize;
    while i < e.body.len()
        invariant
            arena_ok(arena),
            crate::k2_engine::clause_valid(arena.nodes@, e),
            clause_valid(arena.nodes@, c),
            ev == crate::k2_engine::clause_view(arena.nodes@, e),
            e.body.len() == c.body.len(),
            ev.head == c@.head,
            i <= e.body.len(),
            forall|j: int| 0 <= j < i ==> #[trigger] ev.body[j] == c@.body[j],
        decreases e.body.len() - i,
    {
        proof {
            assert(crate::k2_engine::body_item_valid(arena.nodes@, &e.body@[i as int]));
            assert(body_valid(arena.nodes@, &c.body@[i as int]));
            assert(ev.body[i as int] == crate::k2_engine::body_item_view(
                arena.nodes@,
                &e.body@[i as int],
            ));
            assert(c@.body[i as int] == c.body@[i as int]@);
        }
        let same = match (&e.body[i], &c.body[i]) {
            (EBodyItem::Pos { root }, Body::Pos(t)) => {
                let x = from_root(arena, *root);
                crate::m6_term::equal(arena, &x, t)
            },
            (EBodyItem::Naf { roots }, Body::Naf(ts)) => {
                let xs = from_roots(arena, roots);
                if xs.len() != ts.len() {
                    proof {
                        assert(models(xs@).len() != models(ts@).len());
                    }
                    false
                } else {
                    let mut q = 0usize;
                    let mut ok = true;
                    while q < xs.len()
                        invariant
                            arena_ok(arena),
                            valid_all(arena.nodes@, xs@),
                            valid_all(arena.nodes@, ts@),
                            xs.len() == ts.len(),
                            q <= xs.len(),
                            ok ==> forall|m: int|
                                0 <= m < q ==> models(xs@)[m] == #[trigger] models(ts@)[m],
                            !ok ==> models(xs@) != models(ts@),
                        decreases xs.len() - q,
                    {
                        proof {
                            assert(valid(arena.nodes@, &xs@[q as int]));
                            assert(valid(arena.nodes@, &ts@[q as int]));
                            assert(models(xs@)[q as int] == xs@[q as int]@);
                            assert(models(ts@)[q as int] == ts@[q as int]@);
                        }
                        if ok && !crate::m6_term::equal(arena, &xs[q], &ts[q]) {
                            ok = false;
                        }
                        q += 1;
                    }
                    proof {
                        if ok {
                            assert(models(xs@) =~= models(ts@));
                        }
                    }
                    ok
                }
            },
            _ => false,
        };
        if !same {
            proof {
                assert(ev.body[i as int] != c@.body[i as int]);
                assert(ev.body != c@.body);
            }
            return false;
        }
        i += 1;
    }
    proof {
        assert(ev.body =~= c@.body);
    }
    true
}

// The clause of db equal to c, if any.
fn find_clause(arena: &ETermArena, db: &Vec<EClause>, c: &Clause) -> (out: Option<usize>)
    requires
        arena_ok(arena),
        db_valid(arena.nodes@, db@),
        clause_valid(arena.nodes@, c),
    ensures
        match out {
            Some(k) => k < db.len() && db_view(arena.nodes@, db@)[k as int] == c@,
            None => !db_view(arena.nodes@, db@).contains(c@),
        },
{
    let ghost p = db_view(arena.nodes@, db@);
    let mut k = 0usize;
    while k < db.len()
        invariant
            arena_ok(arena),
            db_valid(arena.nodes@, db@),
            clause_valid(arena.nodes@, c),
            p == db_view(arena.nodes@, db@),
            k <= db.len(),
            forall|j: int| 0 <= j < k ==> #[trigger] p[j] != c@,
        decreases db.len() - k,
    {
        proof {
            assert(crate::k2_engine::clause_valid(arena.nodes@, &db@[k as int]));
            assert(p[k as int] == crate::k2_engine::clause_view(arena.nodes@, &db@[k as int]));
        }
        if clause_eq(arena, &db[k], c) {
            return Some(k);
        }
        k += 1;
    }
    None
}

// `ws::open_head`: head j under the witness pairs, counts + chain outputs open.
fn open_head(
    arena: &mut ETermArena,
    g: &Group,
    sel: &ESel,
    j: usize,
    cs: &Vec<T>,
    outs: &Vec<T>,
) -> (out: T)
    requires
        arena_ok(old(arena)),
        group_valid(old(arena).nodes@, g),
        j < g.clauses.len(),
        valid_all(old(arena).nodes@, cs@),
        models(cs@) == ws::counts(sel@),
        valid_all(old(arena).nodes@, outs@),
        models(outs@) == ws::outputs(sel@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid(final(arena).nodes@, &out),
        out@ == ws::open_head(g@, sel@, j as int),
{
    let ghost keep = |p: (Term, Term)|
        !ws::counts(sel@).contains(p.0) && !ws::outputs(sel@).contains(p.0);
    let ghost pm = map_model(g.pairs@);
    let mut fp: Vec<Binding> = Vec::new();
    let mut i = 0usize;
    proof {
        reveal(Seq::filter);
        assert(pm.take(0) =~= Seq::<(Term, Term)>::empty());
        assert_seqs_equal!(map_model(fp@) == Seq::<(Term, Term)>::empty());
    }
    while i < g.pairs.len()
        invariant
            arena_ok(arena),
            group_valid(arena.nodes@, g),
            valid_all(arena.nodes@, cs@),
            models(cs@) == ws::counts(sel@),
            valid_all(arena.nodes@, outs@),
            models(outs@) == ws::outputs(sel@),
            pm == map_model(g.pairs@),
            keep == (|p: (Term, Term)|
                !ws::counts(sel@).contains(p.0) && !ws::outputs(sel@).contains(p.0)),
            i <= g.pairs.len(),
            map_valid(arena.nodes@, fp@),
            map_model(fp@) == pm.take(i as int).filter(keep),
        decreases g.pairs.len() - i,
    {
        proof {
            assert(binding_valid(arena.nodes@, &g.pairs@[i as int]));
            assert(pm.take(i as int + 1) == pm.take(i as int).push(pm[i as int]));
            pm.take(i as int).lemma_filter_push(pm[i as int], keep);
            assert(pm[i as int].0 == g.pairs@[i as int].key@);
        }
        if !crate::m6_refs::contains(arena, cs, &g.pairs[i].key) && !crate::m6_refs::contains(
            arena,
            outs,
            &g.pairs[i].key,
        ) {
            let b = Binding { key: g.pairs[i].key.cp(), value: g.pairs[i].value.cp() };
            let ghost before = fp@;
            fp.push(b);
            proof {
                assert_seqs_equal!(map_model(fp@) == map_model(before).push(b@));
                assert forall|q: int| 0 <= q < fp.len() implies #[trigger] binding_valid(
                    arena.nodes@,
                    &fp@[q],
                ) by {
                    if q < before.len() {
                        assert(fp@[q] == before[q]);
                    }
                }
            }
        }
        i += 1;
    }
    proof {
        assert(pm.take(i as int) =~= pm);
        assert(clause_valid(arena.nodes@, &g.clauses@[j as int]));
        assert(g@.clauses[j as int] == g.clauses@[j as int]@);
    }
    crate::m6_bind::bind_all(arena, &g.clauses[j].head, &fp)
}

fn nvars_all(arena: &mut ETermArena, ts: &Vec<T>) -> (out: usize)
    requires
        arena_ok(old(arena)),
        valid_all(old(arena).nodes@, ts@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out as nat == ckc_spec::engine::nvars_all(models(ts@)),
        out <= final(arena).nodes@.len(),
{
    let ghost start = arena.nodes@;
    let ghost tm = models(ts@);
    let mut out = 0usize;
    let mut i = ts.len();
    proof {
        reveal_with_fuel(ckc_spec::engine::nvars_all, 1);
        assert(tm.skip(i as int) =~= Seq::<Term>::empty());
    }
    while i > 0
        invariant
            arena_ok(arena),
            start == old(arena).nodes@,
            start.is_prefix_of(arena.nodes@),
            valid_all(arena.nodes@, ts@),
            tm == models(ts@),
            i <= ts.len(),
            out as nat == ckc_spec::engine::nvars_all(tm.skip(i as int)),
            out <= arena.nodes@.len(),
        decreases i,
    {
        i -= 1;
        proof {
            assert(valid(arena.nodes@, &ts@[i as int]));
            assert(tm[i as int] == ts@[i as int]@);
            assert(tm.skip(i as int).drop_first() =~= tm.skip(i as int + 1));
            assert(tm.skip(i as int)[0] == tm[i as int]);
            reveal_with_fuel(ckc_spec::engine::nvars_all, 1);
        }
        let ghost here = arena.nodes@;
        let n = nvars(arena, &ts[i]);
        proof {
            prefix_all(here, arena.nodes@, ts@);
            crate::k2_load::prefix_chain(start, here, arena.nodes@);
        }
        if n > out {
            out = n;
        }
    }
    proof {
        assert(tm.skip(0) =~= tm);
    }
    out
}

fn copy_lists(xs: &Vec<Vec<T>>) -> (out: Vec<Vec<T>>)
    ensures
        out@.len() == xs@.len(),
        forall|i: int| 0 <= i < xs.len() ==> (#[trigger] out@[i])@ == xs@[i]@,
{
    let mut out: Vec<Vec<T>> = Vec::new();
    let mut i = 0usize;
    while i < xs.len()
        invariant
            i <= xs.len(),
            out.len() == i,
            forall|j: int| 0 <= j < i ==> (#[trigger] out@[j])@ == xs@[j]@,
        decreases xs.len() - i,
    {
        out.push(copy(&xs[i]));
        i += 1;
    }
    out
}

pub open spec fn env_facts(g: spec::Group, sel: ws::Sel, p: Seq<DocClause>) -> Seq<Term> {
    ws::skeleton(g, sel) + p.filter(|d: DocClause| ws::ground_fact(d)).map_values(
        |d: DocClause| d.head,
    )
}

pub open spec fn env_bounds(g: spec::Group, sel: ws::Sel) -> Seq<Seq<Term>> {
    Seq::new(sel.ops.len(), |i: int| ws::boundaries(g, sel, i))
}

fn clause_ok(
    arena: &mut ETermArena,
    db: &Vec<EClause>,
    g: &Group,
    sel: &ESel,
    j: usize,
    sk: &Vec<T>,
    cs: &Vec<T>,
    outs: &Vec<T>,
    facts: &Vec<T>,
    bounds: &Vec<Vec<T>>,
    Ghost(p): Ghost<Seq<DocClause>>,
) -> (out: bool)
    requires
        arena_ok(old(arena)),
        group_valid(old(arena).nodes@, g),
        j < g.clauses.len(),
        db_valid(old(arena).nodes@, db@),
        db_view(old(arena).nodes@, db@) == p,
        valid_all(old(arena).nodes@, sk@),
        models(sk@) == ws::skeleton(g@, sel@),
        valid_all(old(arena).nodes@, cs@),
        models(cs@) == ws::counts(sel@),
        valid_all(old(arena).nodes@, outs@),
        models(outs@) == ws::outputs(sel@),
        valid_all(old(arena).nodes@, facts@),
        models(facts@) == env_facts(g@, sel@, p),
        seqs_valid(old(arena).nodes@, bounds@),
        seqs_models(bounds@) == env_bounds(g@, sel@),
        bounds.len() == cs.len(),
        ws::primitive_body(g@.clauses[j as int].body),
        valid_all(old(arena).nodes@, sel.chain@),
        arith_goals(sel.chain@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out == ws::clause_ok(p, g@, sel@, j as int),
{
    let ghost start = arena.nodes@;
    proof {
        assert(clause_valid(arena.nodes@, &g.clauses@[j as int]));
        assert(g@.clauses[j as int] == g.clauses@[j as int]@);
    }
    let c = crate::m6_normal::canon_clause(arena, &g.clauses[j]);
    let ghost n1 = arena.nodes@;
    proof {
        crate::k2_engine::db_models_prefix(start, n1, db@);
    }
    let k = match find_clause(arena, db, &c) {
        Some(k) => k,
        None => {
            return false;
        },
    };
    proof {
        crate::m6_witness::group_prefix(start, n1, g);
        prefix_all(start, n1, cs@);
        prefix_all(start, n1, outs@);
    }
    let h = open_head(arena, g, sel, j, cs, outs);
    let ghost n2 = arena.nodes@;
    proof {
        crate::k2_engine::db_models_prefix(n1, n2, db@);
        prefix_all(start, n2, sk@);
        prefix_all(n1, n2, cs@);
        crate::k2_load::prefix_chain(start, n1, n2);
    }
    let a = nvars_all(arena, sk);
    let ghost n3 = arena.nodes@;
    proof {
        prefix(n2, n3, &h);
    }
    let b = nvars(arena, &h);
    let off = if a >= b {
        a
    } else {
        b
    };
    let ghost n4 = arena.nodes@;
    proof {
        crate::k2_load::prefix_chain(n2, n3, n4);
        crate::k2_load::prefix_chain(start, n2, n4);
        crate::k2_engine::db_models_prefix(n2, n4, db@);
        prefix_all(n2, n4, cs@);
        prefix_all(start, n4, sk@);
        prefix(n3, n4, &h);
        assert(off as nat == ckc_spec::engine::max_nat(
            ckc_spec::engine::nvars_all(ws::skeleton(g@, sel@)),
            ckc_spec::engine::nvars(ws::open_head(g@, sel@, j as int)),
        ));
    }
    if !head_call(arena, &h) {
        return false;
    }
    if !slot_free(arena, cs, &h) {
        proof {
            let i = choose|i: int| 0 <= i < cs.len() && ws::mentions(h@, #[trigger] models(cs@)[i]);
            assert(ws::mentions(h@, ws::counts(sel@)[i]));
            assert forall|e: ws::WalkEnv|
                e.slots == ws::counts(sel@) implies !#[trigger] ws::slot_free(e, h@) by {
                assert(ws::mentions(h@, e.slots[i]));
            }
        }
        return false;
    }
    let ghost offs = ckc_spec::engine::max_nat(
        ckc_spec::engine::nvars_all(ws::skeleton(g@, sel@)),
        ckc_spec::engine::nvars(ws::open_head(g@, sel@, j as int)),
    );
    proof {
        clauses_prefix_one(n1, arena.nodes@, &c);
    }
    // every program clause reachable from the open head is c
    let mut k2 = 0usize;
    while k2 < db.len()
        invariant
            arena_ok(arena),
            start == old(arena).nodes@,
            j < g.clauses.len(),
            g@.clauses.len() == g.clauses.len(),
            c@ == spec::canon_clause(g@.clauses[j as int]),
            h@ == ws::open_head(g@, sel@, j as int),
            off as nat == offs,
            offs == ckc_spec::engine::max_nat(
                ckc_spec::engine::nvars_all(ws::skeleton(g@, sel@)),
                ckc_spec::engine::nvars(ws::open_head(g@, sel@, j as int)),
            ),
            valid_all(arena.nodes@, sk@),
            models(sk@) == ws::skeleton(g@, sel@),
            n4.is_prefix_of(arena.nodes@),
            start.is_prefix_of(arena.nodes@),
            db_valid(arena.nodes@, db@),
            db_view(arena.nodes@, db@) == p,
            clause_valid(arena.nodes@, &c),
            valid(arena.nodes@, &h),
            off <= arena.nodes@.len(),
            k2 <= db.len(),
            forall|q: int|
                0 <= q < k2 && ws::unifiable(
                    h@,
                    ckc_spec::engine::shift((#[trigger] p[q]).head, off as nat),
                ) ==> p[q] == c@,
        decreases db.len() - k2,
    {
        proof {
            assert(crate::k2_engine::clause_valid(arena.nodes@, &db@[k2 as int]));
            assert(p[k2 as int] == crate::k2_engine::clause_view(arena.nodes@, &db@[k2 as int]));
        }
        let ghost here = arena.nodes@;
        let sh = crate::k2_engine::shift_root(arena, db[k2].head, off);
        proof {
            crate::k2_engine::db_models_prefix(here, arena.nodes@, db@);
            clauses_prefix_one(here, arena.nodes@, &c);
            prefix(here, arena.nodes@, &h);
            prefix_all(here, arena.nodes@, sk@);
            crate::k2_load::prefix_chain(n4, here, arena.nodes@);
            crate::k2_load::prefix_chain(start, here, arena.nodes@);
            assert(arena@[h.root as int] == h@);
        }
        let ghost here2 = arena.nodes@;
        let u = unifiable(arena, h.root, sh);
        proof {
            crate::k2_engine::db_models_prefix(here2, arena.nodes@, db@);
            clauses_prefix_one(here2, arena.nodes@, &c);
            prefix(here2, arena.nodes@, &h);
            prefix_all(here2, arena.nodes@, sk@);
            crate::k2_load::prefix_chain(n4, here2, arena.nodes@);
            crate::k2_load::prefix_chain(start, here2, arena.nodes@);
            assert(crate::k2_engine::clause_valid(arena.nodes@, &db@[k2 as int]));
            assert(p[k2 as int] == crate::k2_engine::clause_view(arena.nodes@, &db@[k2 as int]));
        }
        if u && !clause_eq(arena, &db[k2], &c) {
            proof {
                assert(ws::unifiable(h@, ckc_spec::engine::shift(p[k2 as int].head, offs)));
                assert(p[k2 as int] != c@);
            }
            return false;
        }
        k2 += 1;
    }
    // no skeleton fact matches the open head
    let mut m = 0usize;
    while m < sk.len()
        invariant
            arena_ok(arena),
            start == old(arena).nodes@,
            j < g.clauses.len(),
            g@.clauses.len() == g.clauses.len(),
            c@ == spec::canon_clause(g@.clauses[j as int]),
            h@ == ws::open_head(g@, sel@, j as int),
            off as nat == offs,
            offs == ckc_spec::engine::max_nat(
                ckc_spec::engine::nvars_all(ws::skeleton(g@, sel@)),
                ckc_spec::engine::nvars(ws::open_head(g@, sel@, j as int)),
            ),
            valid_all(arena.nodes@, sk@),
            models(sk@) == ws::skeleton(g@, sel@),
            forall|q: int|
                0 <= q < db.len() && ws::unifiable(
                    h@,
                    ckc_spec::engine::shift((#[trigger] p[q]).head, off as nat),
                ) ==> p[q] == c@,
            n4.is_prefix_of(arena.nodes@),
            start.is_prefix_of(arena.nodes@),
            db_valid(arena.nodes@, db@),
            db_view(arena.nodes@, db@) == p,
            clause_valid(arena.nodes@, &c),
            valid(arena.nodes@, &h),
            valid_all(arena.nodes@, sk@),
            off <= arena.nodes@.len(),
            m <= sk.len(),
            forall|q: int| 0 <= q < m ==> !ws::unifiable(h@, #[trigger] models(sk@)[q]),
        decreases sk.len() - m,
    {
        proof {
            assert(valid(arena.nodes@, &sk@[m as int]));
            assert(models(sk@)[m as int] == sk@[m as int]@);
            assert(arena@[h.root as int] == h@);
            assert(arena@[sk@[m as int].root as int] == sk@[m as int]@);
        }
        let ghost here = arena.nodes@;
        let u = unifiable(arena, h.root, sk[m].root);
        proof {
            crate::k2_engine::db_models_prefix(here, arena.nodes@, db@);
            clauses_prefix_one(here, arena.nodes@, &c);
            prefix(here, arena.nodes@, &h);
            prefix_all(here, arena.nodes@, sk@);
            crate::k2_load::prefix_chain(n4, here, arena.nodes@);
            crate::k2_load::prefix_chain(start, here, arena.nodes@);
        }
        if u {
            proof {
                assert(ws::unifiable(h@, ws::skeleton(g@, sel@)[m as int]));
            }
            return false;
        }
        m += 1;
    }
    // the body from the head unification
    let ghost cm = c@;
    let ghost nm = arena.nodes@;
    proof {
        assert(crate::k2_engine::clause_valid(arena.nodes@, &db@[k as int]));
        assert(p[k as int] == crate::k2_engine::clause_view(arena.nodes@, &db@[k as int]));
        assert(cm == p[k as int]);
        crate::k2_load::prefix_chain(start, n4, nm);
    }
    let cn = crate::k2_engine::clause_nvars(arena, &db[k]);
    let ghost n5 = arena.nodes@;
    proof {
        assert(cn as nat == ckc_spec::engine::clause_nvars(cm));
        crate::k2_engine::db_models_prefix(nm, n5, db@);
        prefix(nm, n5, &h);
        crate::k2_load::prefix_chain(start, nm, n5);
    }
    let hh = crate::k2_engine::shift_root(arena, db[k].head, off);
    let ghost n6 = arena.nodes@;
    proof {
        crate::k2_engine::db_models_prefix(n5, n6, db@);
        prefix(n5, n6, &h);
        crate::k2_load::prefix_chain(start, n5, n6);
        assert(arena@[hh as int] == ckc_spec::engine::shift(cm.head, off as nat));
    }
    let mut items: Vec<usize> = Vec::new();
    let mut i = 0usize;
    proof {
        assert(crate::k2_engine::roots_valid(arena.nodes@, items@)) by {
            reveal(crate::k2_engine::roots_valid);
        }
    }
    while i < db[k].body.len()
        invariant
            arena_ok(arena),
            start.is_prefix_of(arena.nodes@),
            n6.is_prefix_of(arena.nodes@),
            db_valid(arena.nodes@, db@),
            db_view(arena.nodes@, db@) == p,
            valid(arena.nodes@, &h),
            k < db.len(),
            cm == p[k as int],
            hh < arena.nodes@.len(),
            arena@[hh as int] == ckc_spec::engine::shift(cm.head, off as nat),
            off as nat + ckc_spec::engine::nvars(cm.head) <= arena.nodes@.len(),
            off <= arena.nodes@.len(),
            i <= db@[k as int].body.len(),
            items.len() == i,
            crate::k2_engine::roots_valid(arena.nodes@, items@),
            forall|q: int|
                0 <= q < i ==> crate::k2_engine::root_terms(arena.nodes@, items@)[q]
                    == ckc_spec::engine::item_term(#[trigger] cm.body[q], off as nat),
            forall|q: int|
                0 <= q < i ==> off as nat + crate::k2_engine::item_nvars(#[trigger] cm.body[q])
                    <= arena.nodes@.len(),
        decreases db@[k as int].body.len() - i,
    {
        proof {
            assert(crate::k2_engine::clause_valid(arena.nodes@, &db@[k as int]));
            assert(crate::k2_engine::body_item_valid(arena.nodes@, &db@[k as int].body@[i as int]));
            assert(cm.body[i as int] == crate::k2_engine::body_item_view(
                arena.nodes@,
                &db@[k as int].body@[i as int],
            ));
        }
        let ghost here = arena.nodes@;
        let r = crate::k2_engine::shift_item(arena, &db[k].body[i], off);
        proof {
            crate::k2_engine::db_models_prefix(here, arena.nodes@, db@);
            crate::k2_load::prefix_chain(n6, here, arena.nodes@);
            crate::k2_load::prefix_chain(start, here, arena.nodes@);
            prefix(here, arena.nodes@, &h);
            assert forall|q: int| 0 <= q < i implies crate::k2_engine::root_terms(
                arena.nodes@,
                items@,
            )[q] == crate::k2_engine::root_terms(here, items@)[q] by {
                assert(here[items@[q] as int] == arena.nodes@[items@[q] as int]);
            }
            assert(arena@[hh as int] == here[hh as int].term@);
        }
        let ghost before = items@;
        items.push(r);
        proof {
            assert forall|q: int| 0 <= q < i + 1 implies crate::k2_engine::root_terms(
                arena.nodes@,
                items@,
            )[q] == ckc_spec::engine::item_term(#[trigger] cm.body[q], off as nat) by {
                if q < i {
                    assert(items@[q] == before[q]);
                }
            }
            assert(crate::k2_engine::roots_valid(arena.nodes@, items@)) by {
                reveal(crate::k2_engine::roots_valid);
            }
        }
        i += 1;
    }
    let ghost body = cm.body.map_values(|it: BodyItem| ckc_spec::engine::item_term(it, off as nat));
    let total = arena.nodes.len();
    proof {
        assert(crate::k2_engine::clause_valid(arena.nodes@, &db@[k as int]));
        assert(i == cm.body.len());
        assert(crate::k2_engine::root_terms(arena.nodes@, items@) =~= body);
        items_nvars_bound(cm.body, (arena.nodes@.len() - off) as nat);
        assert(off as nat + cn as nat <= total);
        prefix_all(start, arena.nodes@, facts@);
        prefix_all(start, arena.nodes@, cs@);
        prefix_all(start, arena.nodes@, sel.chain@);
        crate::m6_cands::seqs_prefix(start, arena.nodes@, bounds@);
    }
    let e = EWalk {
        facts: copy(facts),
        slots: copy(cs),
        bounds: copy_lists(bounds),
        chain: copy(&sel.chain),
        off: off + cn,
    };
    let ghost we = walk_model(&e, p);
    proof {
        assert(seqs_models(e.bounds@) =~= seqs_models(bounds@));
        assert(models(e.facts@) =~= models(facts@));
        assert(models(e.slots@) =~= models(cs@));
        assert(models(e.chain@) =~= sel@.chain);
        assert(crate::m6_cands::seqs_valid(arena.nodes@, e.bounds@)) by {
            assert forall|q: int| 0 <= q < e.bounds.len() implies valid_all(
                arena.nodes@,
                (#[trigger] e.bounds@[q])@,
            ) by {
                assert(e.bounds@[q]@ == bounds@[q]@);
            }
        }
    }
    let mut pairs: Vec<crate::k2_engine::EPair> = Vec::new();
    pairs.push(crate::k2_engine::EPair { left: h.root, right: hh });
    let st = crate::k2_engine::EBoundState { pairs, stack: Vec::new(), sol: items };
    let ghost u = ckc_spec::engine::UState {
        pairs: seq![(h@, ckc_spec::engine::shift(cm.head, off as nat))],
        stack: Seq::empty(),
        sol: body,
    };
    proof {
        assert(arena@[h.root as int] == h@);
        assert(crate::k2_engine::bound_state_valid(arena.nodes@, &st)) by {
            reveal(crate::k2_engine::roots_valid);
        }
        assert(crate::k2_engine::pairs_view(arena.nodes@, st.pairs@) =~= seq![
            (h@, ckc_spec::engine::shift(cm.head, off as nat)),
        ]);
        assert(crate::k2_engine::goals_view(arena.nodes@, st.stack@) =~= Seq::<
            ckc_spec::engine::Goal,
        >::empty());
        assert(crate::k2_engine::root_terms(arena.nodes@, st.sol@) == body);
        assert(crate::k2_engine::bound_state_view(arena.nodes@, &st) == u);
        canon_primitive(g@.clauses[j as int]);
        assert forall|q: int| 0 <= q < body.len() implies structured(#[trigger] body[q]) by {
            assert(crate::k2_engine::body_item_valid(arena.nodes@, &db@[k as int].body@[q]));
            assert(cm.body[q] == crate::k2_engine::body_item_view(
                arena.nodes@,
                &db@[k as int].body@[q],
            ));
            item_structured(cm.body[q], off as nat);
        }
        crate::k3_sound::naf::unify_output_len(u);
        unify_structured(u, body.len() as int);
    }
    let ghost n7 = arena.nodes@;
    match crate::k2_engine::unify(arena, st) {
        crate::k2_engine::EUResult::Ok { stack: _, sol: bs } => {
            let gs = from_roots(arena, &bs);
            proof {
                walk_prefix(n7, arena.nodes@, &e, db@);
                nodes_all_eq(models(gs@), body);
            }
            walk_ok(arena, &e, db, &gs, 0, Ghost((1 + ws::nodes_all(body)) as nat), Ghost(we))
        },
        crate::k2_engine::EUResult::Fail => false,
    }
}

// The facts a walk call may match: the skeleton + the program's ground facts.
fn env_facts_exec(
    arena: &ETermArena,
    sk: &Vec<T>,
    db: &Vec<EClause>,
    Ghost(p): Ghost<Seq<DocClause>>,
) -> (out: Vec<T>)
    requires
        arena_ok(arena),
        db_valid(arena.nodes@, db@),
        db_view(arena.nodes@, db@) == p,
        valid_all(arena.nodes@, sk@),
    ensures
        valid_all(arena.nodes@, out@),
        models(out@) == models(sk@) + p.filter(|d: DocClause| ws::ground_fact(d)).map_values(
            |d: DocClause| d.head,
        ),
{
    let ghost keep = |d: DocClause| ws::ground_fact(d);
    let ghost head = |d: DocClause| d.head;
    let mut gf: Vec<T> = Vec::new();
    let mut k = 0usize;
    proof {
        reveal(Seq::filter);
        assert(p.take(0) =~= Seq::<DocClause>::empty());
        assert_seqs_equal!(models(gf@) == p.take(0).filter(keep).map_values(head));
    }
    while k < db.len()
        invariant
            arena_ok(arena),
            db_valid(arena.nodes@, db@),
            db_view(arena.nodes@, db@) == p,
            keep == (|d: DocClause| ws::ground_fact(d)),
            head == (|d: DocClause| d.head),
            k <= db.len(),
            valid_all(arena.nodes@, gf@),
            models(gf@) == p.take(k as int).filter(keep).map_values(head),
        decreases db.len() - k,
    {
        proof {
            assert(crate::k2_engine::clause_valid(arena.nodes@, &db@[k as int]));
            assert(p[k as int] == crate::k2_engine::clause_view(arena.nodes@, &db@[k as int]));
            assert(p.take(k as int + 1) == p.take(k as int).push(p[k as int]));
            p.take(k as int).lemma_filter_push(p[k as int], keep);
            assert(arena@[db@[k as int].head as int] == p[k as int].head);
        }
        let c = &db[k];
        if c.body.len() == 0 && crate::k2_walk::ground_root(arena, c.head) {
            let t = from_root(arena, c.head);
            let ghost before = gf@;
            proof {
                extend(arena.nodes@, gf@, t);
            }
            gf.push(t);
            proof {
                let f0 = p.take(k as int).filter(keep);
                assert(f0.push(p[k as int]).map_values(head) =~= f0.map_values(head).push(
                    p[k as int].head,
                ));
                assert_seqs_equal!(models(gf@) == models(before).push(t@));
            }
        }
        k += 1;
    }
    proof {
        assert(p.take(k as int) =~= p);
    }
    concat(sk, &gf)
}

// Each operand's B (`ws::boundaries`).
fn env_bounds_exec(
    arena: &mut ETermArena,
    g: &Group,
    sel: &ESel,
    sk: &Vec<T>,
    nafs: &Vec<Vec<T>>,
) -> (out: Vec<Vec<T>>)
    requires
        arena_ok(old(arena)),
        crate::m6_witness::ops_valid(old(arena).nodes@, sel),
        valid_all(old(arena).nodes@, sk@),
        models(sk@) == ws::skeleton(g@, sel@),
        seqs_valid(old(arena).nodes@, nafs@),
        seqs_models(nafs@) == ws::naf_items(g@, sel@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        seqs_valid(final(arena).nodes@, out@),
        seqs_models(out@) == env_bounds(g@, sel@),
        out.len() == sel.ops.len(),
{
    let ghost start = arena.nodes@;
    let mut out: Vec<Vec<T>> = Vec::new();
    let mut i = 0usize;
    while i < sel.ops.len()
        invariant
            arena_ok(arena),
            start == old(arena).nodes@,
            start.is_prefix_of(arena.nodes@),
            crate::m6_witness::ops_valid(arena.nodes@, sel),
            valid_all(arena.nodes@, sk@),
            models(sk@) == ws::skeleton(g@, sel@),
            seqs_valid(arena.nodes@, nafs@),
            seqs_models(nafs@) == ws::naf_items(g@, sel@),
            i <= sel.ops.len(),
            out.len() == i,
            seqs_valid(arena.nodes@, out@),
            forall|q: int|
                0 <= q < i ==> models((#[trigger] out@[q])@) == ws::boundaries(g@, sel@, q),
        decreases sel.ops.len() - i,
    {
        let o = &sel.ops[i];
        proof {
            assert(opnd_valid(arena.nodes@, &sel.ops@[i as int]));
            assert(sel@.ops[i as int] == o@);
            assert(crate::m6_cands::cond_model(o.cond) == sel@.ops[i as int].cond);
        }
        let ghost here = arena.nodes@;
        let bs = crate::m6_cands::boundaries(arena, &o.cond, nafs, sk, &o.n);
        proof {
            crate::m6_witness::ops_prefix_pub(here, arena.nodes@, sel);
            prefix_all(here, arena.nodes@, sk@);
            crate::m6_cands::seqs_prefix(here, arena.nodes@, nafs@);
            crate::m6_cands::seqs_prefix(here, arena.nodes@, out@);
            crate::k2_load::prefix_chain(start, here, arena.nodes@);
            assert(models(bs@) == ws::boundaries(g@, sel@, i as int));
        }
        let ghost before = out@;
        out.push(bs);
        proof {
            assert forall|q: int| 0 <= q < out.len() implies valid_all(
                arena.nodes@,
                (#[trigger] out@[q])@,
            ) by {
                if q < i {
                    assert(out@[q] == before[q]);
                }
            }
            assert forall|q: int| 0 <= q < i + 1 implies models((#[trigger] out@[q])@)
                == ws::boundaries(g@, sel@, q) by {
                if q < i {
                    assert(out@[q] == before[q]);
                }
            }
        }
        i += 1;
    }
    proof {
        assert(seqs_models(out@) =~= env_bounds(g@, sel@));
    }
    out
}

// `ws::in_fragment`: the completeness premise, decided.
pub fn in_fragment(
    arena: &mut ETermArena,
    prog: &crate::m6_witness::DocProgram,
    g: &Group,
    sel: &ESel,
    Ghost(p): Ghost<Seq<DocClause>>,
) -> (out: bool)
    requires
        arena_ok(old(arena)),
        crate::m6_witness::program_ok(old(arena).nodes@, prog, p),
        group_valid(old(arena).nodes@, g),
        crate::m6_witness::ops_valid(old(arena).nodes@, sel),
        valid_all(old(arena).nodes@, sel.chain@),
        arith_goals(sel.chain@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out == ws::in_fragment(p, g@, sel@),
{
    let ghost start = arena.nodes@;
    proof {
        reveal(crate::m6_witness::program_ok);
    }
    let db = &prog.db;
    if g.clauses.len() == 0 {
        return false;
    }
    if !ground_pairs(arena, g) {
        return false;
    }
    if !chain_shape(arena, sel) {
        return false;
    }
    if !exact_comparators(arena, g, sel) {
        return false;
    }
    let cs = crate::m6_cands::counts(sel);
    proof {
        assert forall|q: int| 0 <= q < cs.len() implies #[trigger] valid(arena.nodes@, &cs@[q]) by {
            assert(opnd_valid(arena.nodes@, &sel.ops@[q]));
        }
    }
    let op = crate::m6_cands::open_pairs(arena, g, sel, &cs);
    let sk = crate::m6_cands::skeleton(arena, g, sel, &op);
    let ghost n1 = arena.nodes@;
    proof {
        crate::m6_witness::group_prefix(start, n1, g);
        crate::m6_witness::ops_prefix_pub(start, n1, sel);
        prefix_all(start, n1, sel.chain@);
        prefix_all(start, n1, cs@);
        map_prefix(start, n1, op@);
        crate::k2_engine::db_models_prefix(start, n1, db@);
    }
    if !fixed_skeleton(arena, g, sel, &sk) {
        return false;
    }
    let outs = outputs(arena, sel);
    let mut j = 0usize;
    while j < g.clauses.len()
        invariant
            arena_ok(arena),
            start == old(arena).nodes@,
            start.is_prefix_of(arena.nodes@),
            group_valid(arena.nodes@, g),
            valid_all(arena.nodes@, sel.chain@),
            valid_all(arena.nodes@, outs@),
            models(outs@) == ws::outputs(sel@),
            j <= g.clauses.len(),
            forall|q: int|
                0 <= q < j ==> ws::primitive_body((#[trigger] g@.clauses[q]).body)
                    && ws::chain_body(sel@, g@.clauses[q].body),
        decreases g.clauses.len() - j,
    {
        proof {
            assert(clause_valid(arena.nodes@, &g.clauses@[j as int]));
            assert(g@.clauses[j as int] == g.clauses@[j as int]@);
        }
        if !primitive_body(arena, &g.clauses[j].body) || !chain_body(
            arena,
            sel,
            &outs,
            &g.clauses[j].body,
        ) {
            return false;
        }
        j += 1;
    }
    let facts = env_facts_exec(arena, &sk, db, Ghost(p));
    let nafs = crate::m6_cands::naf_items(arena, g, sel, &op);
    let ghost n2 = arena.nodes@;
    proof {
        crate::m6_witness::ops_prefix_pub(n1, n2, sel);
        prefix_all(n1, n2, sk@);
        prefix_all(n1, n2, facts@);
    }
    let bounds = env_bounds_exec(arena, g, sel, &sk, &nafs);
    let ghost n3 = arena.nodes@;
    proof {
        crate::m6_witness::group_prefix(start, n3, g);
        crate::k2_load::prefix_chain(start, n1, n2);
        crate::k2_load::prefix_chain(start, n2, n3);
        crate::m6_witness::group_prefix(n1, n3, g);
        prefix_all(n2, n3, sk@);
        prefix_all(n1, n3, cs@);
        prefix_all(n1, n3, outs@);
        prefix_all(n1, n3, sel.chain@);
        prefix_all(n2, n3, facts@);
        crate::k2_engine::db_models_prefix(n1, n3, db@);
        assert(models(facts@) == env_facts(g@, sel@, p));
    }
    let mut j = 0usize;
    while j < g.clauses.len()
        invariant
            arena_ok(arena),
            start == old(arena).nodes@,
            start.is_prefix_of(arena.nodes@),
            group_valid(arena.nodes@, g),
            db_valid(arena.nodes@, db@),
            db_view(arena.nodes@, db@) == p,
            valid_all(arena.nodes@, sk@),
            models(sk@) == ws::skeleton(g@, sel@),
            valid_all(arena.nodes@, cs@),
            models(cs@) == ws::counts(sel@),
            valid_all(arena.nodes@, outs@),
            models(outs@) == ws::outputs(sel@),
            valid_all(arena.nodes@, facts@),
            models(facts@) == env_facts(g@, sel@, p),
            seqs_valid(arena.nodes@, bounds@),
            seqs_models(bounds@) == env_bounds(g@, sel@),
            bounds.len() == cs.len(),
            valid_all(arena.nodes@, sel.chain@),
            arith_goals(sel.chain@),
            forall|q: int|
                0 <= q < g.clauses.len() ==> ws::primitive_body((#[trigger] g@.clauses[q]).body)
                    && ws::chain_body(sel@, g@.clauses[q].body),
            j <= g.clauses.len(),
            forall|q: int| 0 <= q < j ==> #[trigger] ws::clause_ok(p, g@, sel@, q),
        decreases g.clauses.len() - j,
    {
        let ghost here = arena.nodes@;
        let ok = clause_ok(arena, db, g, sel, j, &sk, &cs, &outs, &facts, &bounds, Ghost(p));
        proof {
            crate::m6_witness::group_prefix(here, arena.nodes@, g);
            crate::k2_engine::db_models_prefix(here, arena.nodes@, db@);
            prefix_all(here, arena.nodes@, sk@);
            prefix_all(here, arena.nodes@, cs@);
            prefix_all(here, arena.nodes@, outs@);
            prefix_all(here, arena.nodes@, facts@);
            prefix_all(here, arena.nodes@, sel.chain@);
            crate::m6_cands::seqs_prefix(here, arena.nodes@, bounds@);
            crate::k2_load::prefix_chain(start, here, arena.nodes@);
        }
        if !ok {
            return false;
        }
        j += 1;
    }
    true
}

} // verus!
