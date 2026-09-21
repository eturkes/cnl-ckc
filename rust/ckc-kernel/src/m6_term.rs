use crate::k2_term::{ENode, ENodeKind, ETermArena};
#[cfg(verus_keep_ghost)]
use crate::k2_term::{arena_ok, node_ok, root_ok};
#[cfg(verus_keep_ghost)]
use crate::m6_symbols::symbol;
use crate::m6_symbols::{Sym, symbol_bytes};
use ckc_spec::term::Term;
use vstd::assert_seqs_equal;
use vstd::prelude::*;
use vstd::slice::slice_to_vec;

verus! {

// Stable handles keep the projection models independent of arena growth.
pub struct T {
    pub root: usize,
    pub model: Ghost<Term>,
}

impl View for T {
    type V = Term;

    open spec fn view(&self) -> Term {
        self.model@
    }
}

impl T {
    pub fn cp(&self) -> (out: T)
        ensures
            out.root == self.root,
            out@ == self@,
    {
        T { root: self.root, model: Ghost(self@) }
    }
}

pub open spec fn valid(nodes: Seq<ENode>, t: &T) -> bool {
    t.root < nodes.len() && nodes[t.root as int].term@ == t@
}

pub open spec fn models(ts: Seq<T>) -> Seq<Term> {
    ts.map_values(|t: T| t@)
}

pub open spec fn valid_all(nodes: Seq<ENode>, ts: Seq<T>) -> bool {
    forall|i: int| 0 <= i < ts.len() ==> #[trigger] valid(nodes, &ts[i])
}

pub proof fn prefix(before: Seq<ENode>, after: Seq<ENode>, t: &T)
    requires
        before.is_prefix_of(after),
        valid(before, t),
    ensures
        valid(after, t),
{
}

pub proof fn prefix_all(before: Seq<ENode>, after: Seq<ENode>, ts: Seq<T>)
    requires
        before.is_prefix_of(after),
        valid_all(before, ts),
    ensures
        valid_all(after, ts),
{
    assert forall|i: int| 0 <= i < ts.len() implies #[trigger] valid(after, &ts[i]) by {
        prefix(before, after, &ts[i]);
    }
}

pub proof fn extend(nodes: Seq<ENode>, ts: Seq<T>, t: T)
    requires
        valid_all(nodes, ts),
        valid(nodes, &t),
    ensures
        valid_all(nodes, ts.push(t)),
        models(ts.push(t)) == models(ts).push(t@),
{
    assert forall|i: int| 0 <= i < ts.push(t).len() implies #[trigger] valid(
        nodes,
        &ts.push(t)[i],
    ) by {
        if i < ts.len() {
            assert(ts.push(t)[i] == ts[i]);
        }
    }
    assert_seqs_equal!(models(ts.push(t)) == models(ts).push(t@));
}

pub fn from_root(arena: &ETermArena, root: usize) -> (out: T)
    requires
        root_ok(arena, root),
    ensures
        out.root == root,
        out@ == arena@[root as int],
        valid(arena.nodes@, &out),
        ckc_spec::term::wf_term(out@),
{
    proof {
        assert(node_ok(arena.nodes@, root as int));
    }
    T { root, model: Ghost(arena@[root as int]) }
}

pub fn copy(ts: &Vec<T>) -> (out: Vec<T>)
    ensures
        out@ == ts@,
{
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < ts.len()
        invariant
            i <= ts.len(),
            out@ == ts@.take(i as int),
        decreases ts.len() - i,
    {
        let t = ts[i].cp();
        proof {
            assert(t == ts@[i as int]);
        }
        out.push(t);
        proof {
            assert_seqs_equal!(out@ == ts@.take(i as int + 1));
        }
        i += 1;
    }
    out
}

pub fn roots(arena: &ETermArena, ts: &Vec<T>) -> (out: Vec<usize>)
    requires
        arena_ok(arena),
        valid_all(arena.nodes@, ts@),
    ensures
        crate::k2_engine::roots_valid(arena.nodes@, out@),
        crate::k2_engine::root_terms(arena.nodes@, out@) == models(ts@),
        out.len() == ts.len(),
{
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < ts.len()
        invariant
            i <= ts.len(),
            out.len() == i,
            arena_ok(arena),
            valid_all(arena.nodes@, ts@),
            forall|j: int| 0 <= j < i ==> out@[j] == ts@[j].root,
        decreases ts.len() - i,
    {
        out.push(ts[i].root);
        i += 1;
    }
    proof {
        assert forall|j: int| 0 <= j < out.len() implies out@[j] < arena.nodes@.len() by {
            assert(valid(arena.nodes@, &ts@[j]));
        }
        assert_seqs_equal!(crate::k2_engine::root_terms(arena.nodes@, out@) == models(ts@), j => {
            assert(valid(arena.nodes@, &ts@[j]));
        });
    }
    out
}

pub fn from_roots(arena: &ETermArena, rs: &Vec<usize>) -> (out: Vec<T>)
    requires
        arena_ok(arena),
        crate::k2_engine::roots_valid(arena.nodes@, rs@),
    ensures
        valid_all(arena.nodes@, out@),
        models(out@) == crate::k2_engine::root_terms(arena.nodes@, rs@),
        out.len() == rs.len(),
{
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < rs.len()
        invariant
            i <= rs.len(),
            out.len() == i,
            arena_ok(arena),
            crate::k2_engine::roots_valid(arena.nodes@, rs@),
            valid_all(arena.nodes@, out@),
            models(out@) == crate::k2_engine::root_terms(arena.nodes@, rs@).take(i as int),
        decreases rs.len() - i,
    {
        let t = from_root(arena, rs[i]);
        proof {
            extend(arena.nodes@, out@, t);
        }
        out.push(t);
        proof {
            assert_seqs_equal!(models(out@) == crate::k2_engine::root_terms(arena.nodes@, rs@).take(i as int + 1));
        }
        i += 1;
    }
    out
}

pub fn args(arena: &ETermArena, t: &T) -> (out: Vec<T>)
    requires
        arena_ok(arena),
        valid(arena.nodes@, t),
    ensures
        valid_all(arena.nodes@, out@),
        models(out@) == ckc_spec::engine::args_of(t@),
{
    let rs = crate::k2_engine::args_roots(arena, t.root);
    from_roots(arena, &rs)
}

pub fn bytes_eq(left: &[u8], right: &[u8]) -> (out: bool)
    ensures
        out == (left@ == right@),
{
    if left.len() != right.len() {
        return false;
    }
    let mut i = 0usize;
    while i < left.len()
        invariant
            left.len() == right.len(),
            i <= left.len(),
            forall|j: int| 0 <= j < i ==> left@[j] == right@[j],
        decreases left.len() - i,
    {
        if left[i] != right[i] {
            return false;
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(left@ == right@);
    }
    true
}

pub fn has_name(name: &Vec<u8>, sym: &Sym) -> (out: bool)
    ensures
        out == (name@ == symbol(sym)),
{
    let target = symbol_bytes(sym);
    bytes_eq(name, &target)
}

pub fn is_comp(arena: &ETermArena, t: &T, sym: &Sym, arity: usize) -> (out: bool)
    requires
        arena_ok(arena),
        valid(arena.nodes@, t),
    ensures
        out == (match t@ {
            Term::Comp(n, a) => n == symbol(sym) && a.len() == arity,
            _ => false,
        }),
{
    let name = symbol_bytes(sym);
    crate::k2_engine::literal_matches(arena, t.root, &name, arity)
}

pub fn is_atom(arena: &ETermArena, t: &T, sym: &Sym) -> (out: bool)
    requires
        arena_ok(arena),
        valid(arena.nodes@, t),
    ensures
        out == (t@ == Term::Atom(symbol(sym))),
{
    proof {
        assert(node_ok(arena.nodes@, t.root as int));
        reveal(node_ok);
    }
    match &arena.nodes[t.root].kind {
        ENodeKind::Atom { name } => has_name(name, sym),
        _ => false,
    }
}

pub fn is_var(arena: &ETermArena, t: &T) -> (out: bool)
    requires
        arena_ok(arena),
        valid(arena.nodes@, t),
    ensures
        out == (t@ is Var),
{
    proof {
        assert(node_ok(arena.nodes@, t.root as int));
        reveal(node_ok);
    }
    match &arena.nodes[t.root].kind {
        ENodeKind::Var { .. } => true,
        _ => false,
    }
}

pub fn is_nil(arena: &ETermArena, t: &T) -> (out: bool)
    requires
        arena_ok(arena),
        valid(arena.nodes@, t),
    ensures
        out == (t@ == Term::Nil),
{
    proof {
        assert(node_ok(arena.nodes@, t.root as int));
        reveal(node_ok);
    }
    match &arena.nodes[t.root].kind {
        ENodeKind::Nil => true,
        _ => false,
    }
}

pub fn is_int(arena: &ETermArena, t: &T) -> (out: bool)
    requires
        arena_ok(arena),
        valid(arena.nodes@, t),
    ensures
        out == (t@ is Int),
{
    proof {
        assert(node_ok(arena.nodes@, t.root as int));
        reveal(node_ok);
    }
    match &arena.nodes[t.root].kind {
        ENodeKind::Int { .. } => true,
        _ => false,
    }
}

pub fn var_index(arena: &ETermArena, t: &T) -> (out: usize)
    requires
        arena_ok(arena),
        valid(arena.nodes@, t),
    ensures
        out as nat == ckc_spec::emit::var_index(t@),
{
    proof {
        assert(node_ok(arena.nodes@, t.root as int));
        reveal(node_ok);
    }
    match &arena.nodes[t.root].kind {
        ENodeKind::Var { key, .. } => *key,
        _ => 0,
    }
}

pub fn nil(arena: &mut ETermArena) -> (out: T)
    requires
        arena_ok(old(arena)),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid(final(arena).nodes@, &out),
        out@ == Term::Nil,
{
    let r = crate::k2_term::push_nil(arena);
    from_root(arena, r)
}

pub fn int(arena: &mut ETermArena, n: usize) -> (out: T)
    requires
        arena_ok(old(arena)),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid(final(arena).nodes@, &out),
        out@ == Term::Int(n as int),
{
    let r = crate::k2_output::int_root(arena, n);
    from_root(arena, r)
}

pub fn atom(arena: &mut ETermArena, name: &[u8]) -> (out: T)
    requires
        arena_ok(old(arena)),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid(final(arena).nodes@, &out),
        out@ == Term::Atom(name@),
{
    let r = crate::k2_output::atom_root(arena, name);
    from_root(arena, r)
}

pub fn named(arena: &mut ETermArena, sym: &Sym) -> (out: T)
    requires
        arena_ok(old(arena)),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid(final(arena).nodes@, &out),
        out@ == Term::Atom(symbol(sym)),
{
    let name = symbol_bytes(sym);
    atom(arena, &name)
}

pub fn var(arena: &mut ETermArena, key: usize) -> (out: T)
    requires
        arena_ok(old(arena)),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid(final(arena).nodes@, &out),
        out@ == Term::Var(key as nat),
{
    let spelling = crate::k2_engine::var_spelling(key);
    let r = crate::k2_term::push_var(arena, key, spelling);
    from_root(arena, r)
}

pub fn comp(arena: &mut ETermArena, name: &[u8], ts: &Vec<T>) -> (out: T)
    requires
        arena_ok(old(arena)),
        valid_all(old(arena).nodes@, ts@),
        ts.len() > 0,
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid(final(arena).nodes@, &out),
        out@ == Term::Comp(name@, models(ts@)),
{
    let rs = roots(arena, ts);
    let r = crate::k2_output::comp_root(arena, name, rs);
    from_root(arena, r)
}

pub fn c(arena: &mut ETermArena, sym: &Sym, ts: &Vec<T>) -> (out: T)
    requires
        arena_ok(old(arena)),
        valid_all(old(arena).nodes@, ts@),
        ts.len() > 0,
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid(final(arena).nodes@, &out),
        out@ == Term::Comp(symbol(sym), models(ts@)),
{
    let name = symbol_bytes(sym);
    comp(arena, &name, ts)
}

pub fn c1(arena: &mut ETermArena, sym: &Sym, a: &T) -> (out: T)
    requires
        arena_ok(old(arena)),
        valid(old(arena).nodes@, a),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid(final(arena).nodes@, &out),
        out@ == Term::Comp(symbol(sym), seq![a@]),
{
    let mut ts = Vec::new();
    ts.push(a.cp());
    proof {
        assert_seqs_equal!(models(ts@) == seq![a@]);
    }
    c(arena, sym, &ts)
}

pub fn c2(arena: &mut ETermArena, sym: &Sym, a: &T, b: &T) -> (out: T)
    requires
        arena_ok(old(arena)),
        valid(old(arena).nodes@, a),
        valid(old(arena).nodes@, b),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid(final(arena).nodes@, &out),
        out@ == Term::Comp(symbol(sym), seq![a@, b@]),
{
    let mut ts = Vec::new();
    ts.push(a.cp());
    ts.push(b.cp());
    proof {
        assert_seqs_equal!(models(ts@) == seq![a@, b@]);
    }
    c(arena, sym, &ts)
}

pub fn c3(arena: &mut ETermArena, sym: &Sym, a: &T, b: &T, d: &T) -> (out: T)
    requires
        arena_ok(old(arena)),
        valid(old(arena).nodes@, a),
        valid(old(arena).nodes@, b),
        valid(old(arena).nodes@, d),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid(final(arena).nodes@, &out),
        out@ == Term::Comp(symbol(sym), seq![a@, b@, d@]),
{
    let mut ts = Vec::new();
    ts.push(a.cp());
    ts.push(b.cp());
    ts.push(d.cp());
    proof {
        assert_seqs_equal!(models(ts@) == seq![a@, b@, d@]);
    }
    c(arena, sym, &ts)
}

pub fn list(arena: &mut ETermArena, ts: &Vec<T>) -> (out: T)
    requires
        arena_ok(old(arena)),
        valid_all(old(arena).nodes@, ts@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid(final(arena).nodes@, &out),
        out@ == ckc_spec::engine::list_term(models(ts@)),
{
    let rs = roots(arena, ts);
    let r = crate::k2_walk::list_root(arena, &rs);
    from_root(arena, r)
}

pub open spec fn option_models(ts: Option<Vec<T>>) -> Option<Seq<Term>> {
    match ts {
        Some(v) => Some(models(v@)),
        None => None,
    }
}

pub fn items(arena: &ETermArena, t: &T) -> (out: Option<Vec<T>>)
    requires
        arena_ok(arena),
        valid(arena.nodes@, t),
    ensures
        out matches Some(ts) ==> valid_all(arena.nodes@, ts@),
        option_models(out) == ckc_spec::answers::list_items(t@),
{
    match crate::k2_walk::list_items_exec(arena, t.root) {
        Some(rs) => Some(from_roots(arena, &rs)),
        None => None,
    }
}

pub fn substitute(arena: &mut ETermArena, t: &T, x: usize, replacement: &T) -> (out: T)
    requires
        arena_ok(old(arena)),
        valid(old(arena).nodes@, t),
        valid(old(arena).nodes@, replacement),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid(final(arena).nodes@, &out),
        out@ == ckc_spec::engine::subst(t@, x as nat, replacement@),
{
    let r = crate::k2_engine::subst_root(arena, t.root, x, replacement.root);
    from_root(arena, r)
}

pub fn ground(arena: &ETermArena, t: &T) -> (out: bool)
    requires
        arena_ok(arena),
        valid(arena.nodes@, t),
    ensures
        out == ckc_spec::term::ground(t@),
{
    crate::k2_walk::ground_root(arena, t.root)
}

pub fn nvars(arena: &mut ETermArena, t: &T) -> (out: usize)
    requires
        arena_ok(old(arena)),
        valid(old(arena).nodes@, t),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out as nat == ckc_spec::engine::nvars(t@),
        out <= final(arena).nodes.len(),
{
    crate::k2_engine::nvars_root(arena, t.root)
}

pub fn successor(arena: &mut ETermArena, n: usize) -> (out: usize)
    requires
        arena_ok(old(arena)),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out as nat == n as nat + 1,
        out <= final(arena).nodes.len(),
{
    crate::k2_engine::ensure_var_capacity(arena, Some(n))
}

struct Pair {
    a: T,
    b: T,
}

spec fn pair_valid(nodes: Seq<ENode>, p: &Pair) -> bool {
    valid(nodes, &p.a) && valid(nodes, &p.b)
}

spec fn pairs_valid(nodes: Seq<ENode>, ps: Seq<Pair>) -> bool {
    forall|i: int| 0 <= i < ps.len() ==> #[trigger] pair_valid(nodes, &ps[i])
}

spec fn equal_pairs(ps: Seq<Pair>) -> bool {
    forall|i: int| 0 <= i < ps.len() ==> (#[trigger] ps[i]).a@ == ps[i].b@
}

spec fn pair_work(p: Pair) -> nat {
    crate::k2_engine::term_size(p.a@) + crate::k2_engine::term_size(p.b@)
}

spec fn pairs_work(ps: Seq<Pair>) -> nat
    decreases ps.len(),
{
    if ps.len() == 0 {
        0
    } else {
        pair_work(ps[0]) + pairs_work(ps.drop_first())
    }
}

proof fn pairs_concat(a: Seq<Pair>, b: Seq<Pair>)
    ensures
        pairs_work(a + b) == pairs_work(a) + pairs_work(b),
        equal_pairs(a + b) == (equal_pairs(a) && equal_pairs(b)),
    decreases a.len(),
{
    if a.len() > 0 {
        assert((a + b).drop_first() == a.drop_first() + b);
        pairs_concat(a.drop_first(), b);
    }
    reveal_with_fuel(pairs_work, 2);
    if equal_pairs(a + b) {
        assert forall|i: int| 0 <= i < a.len() implies (#[trigger] a[i]).a@ == a[i].b@ by {
            assert((a + b)[i] == a[i]);
        }
        assert forall|i: int| 0 <= i < b.len() implies (#[trigger] b[i]).a@ == b[i].b@ by {
            assert((a + b)[a.len() + i] == b[i]);
        }
    }
    if equal_pairs(a) && equal_pairs(b) {
        assert forall|i: int| 0 <= i < (a + b).len() implies (#[trigger] (a + b)[i]).a@ == (a
            + b)[i].b@ by {
            if i < a.len() {
                assert((a + b)[i] == a[i]);
            } else {
                assert((a + b)[i] == b[i - a.len()]);
            }
        }
    }
}

fn paired_args(arena: &ETermArena, a: &Vec<T>, b: &Vec<T>) -> (out: Vec<Pair>)
    requires
        arena_ok(arena),
        valid_all(arena.nodes@, a@),
        valid_all(arena.nodes@, b@),
        a.len() == b.len(),
    ensures
        pairs_valid(arena.nodes@, out@),
        equal_pairs(out@) == (models(a@) == models(b@)),
        pairs_work(out@) == crate::k2_engine::terms_size(models(a@)) + crate::k2_engine::terms_size(
            models(b@),
        ),
{
    let mut out = Vec::new();
    let mut i = 0usize;
    proof {
        reveal_with_fuel(pairs_work, 1);
        reveal(equal_pairs);
        assert(pairs_work(out@) == 0);
        assert_seqs_equal!(models(a@).skip(0) == models(a@));
        assert_seqs_equal!(models(b@).skip(0) == models(b@));
    }
    while i < a.len()
        invariant
            a.len() == b.len(),
            i <= a.len(),
            out.len() == i,
            arena_ok(arena),
            valid_all(arena.nodes@, a@),
            valid_all(arena.nodes@, b@),
            pairs_valid(arena.nodes@, out@),
            forall|j: int| 0 <= j < i ==> (#[trigger] out@[j]).a@ == a@[j]@ && out@[j].b@ == b@[j]@,
            pairs_work(out@) + crate::k2_engine::terms_size(models(a@).skip(i as int))
                + crate::k2_engine::terms_size(models(b@).skip(i as int))
                == crate::k2_engine::terms_size(models(a@)) + crate::k2_engine::terms_size(
                models(b@),
            ),
        decreases a.len() - i,
    {
        let p = Pair { a: a[i].cp(), b: b[i].cp() };
        let ghost before = out@;
        proof {
            pairs_concat(before, seq![p]);
            assert(before.push(p) == before + seq![p]);
            reveal_with_fuel(pairs_work, 2);
            assert(models(a@).skip(i as int).drop_first() == models(a@).skip(i as int + 1));
            assert(models(b@).skip(i as int).drop_first() == models(b@).skip(i as int + 1));
            reveal_with_fuel(crate::k2_engine::terms_size, 1);
        }
        out.push(p);
        proof {
            assert forall|j: int| 0 <= j < out.len() implies #[trigger] pair_valid(
                arena.nodes@,
                &out@[j],
            ) by {
                if j < before.len() {
                    assert(out@[j] == before[j]);
                }
            }
        }
        i += 1;
    }
    proof {
        assert(models(a@).skip(i as int).len() == 0);
        assert(models(b@).skip(i as int).len() == 0);
        reveal(crate::k2_engine::terms_size);
        if equal_pairs(out@) {
            assert_seqs_equal!(models(a@) == models(b@), j => {
                assert(out@[j].a@ == a@[j]@);
                assert(out@[j].b@ == b@[j]@);
                assert(out@[j].a@ == out@[j].b@);
            });
        }
        if models(a@) == models(b@) {
            assert forall|j: int| 0 <= j < out.len() implies (#[trigger] out@[j]).a@
                == out@[j].b@ by {
                assert(models(a@)[j] == models(b@)[j]);
            }
        }
    }
    out
}

enum EqStep {
    Equal,
    Different,
    Children(Vec<Pair>),
}

fn compare_step(arena: &ETermArena, p: &Pair) -> (out: EqStep)
    requires
        arena_ok(arena),
        pair_valid(arena.nodes@, p),
    ensures
        pair_work(*p) > 0,
        match out {
            EqStep::Equal => p.a@ == p.b@,
            EqStep::Different => p.a@ != p.b@,
            EqStep::Children(ps) => pairs_valid(arena.nodes@, ps@) && equal_pairs(ps@) == (p.a@
                == p.b@) && pairs_work(ps@) < pair_work(*p),
        },
{
    proof {
        assert(node_ok(arena.nodes@, p.a.root as int));
        assert(node_ok(arena.nodes@, p.b.root as int));
        reveal(node_ok);
        reveal(crate::k2_engine::term_size);
        assert(pair_work(*p) > 0);
    }
    if p.a.root == p.b.root {
        return EqStep::Equal;
    }
    match (&arena.nodes[p.a.root].kind, &arena.nodes[p.b.root].kind) {
        (ENodeKind::Var { key: a, .. }, ENodeKind::Var { key: b, .. }) => {
            if a == b {
                EqStep::Equal
            } else {
                EqStep::Different
            }
        },
        (ENodeKind::Nil, ENodeKind::Nil) => EqStep::Equal,
        (ENodeKind::Atom { name: a }, ENodeKind::Atom { name: b }) => {
            if bytes_eq(a, b) {
                EqStep::Equal
            } else {
                EqStep::Different
            }
        },
        (
            ENodeKind::Int { magnitude: a, negative: an, value: av, .. },
            ENodeKind::Int { magnitude: b, negative: bn, value: bv, .. },
        ) => {
            match crate::k2_term::int_order(a, *an, *av, b, *bn, *bv) {
                crate::k2_term::EOrder::Equal => EqStep::Equal,
                _ => EqStep::Different,
            }
        },
        (
            ENodeKind::Comp { name: a, child_roots: ac, .. },
            ENodeKind::Comp { name: b, child_roots: bc, .. },
        ) => {
            if !bytes_eq(a, b) || ac.len() != bc.len() {
                return EqStep::Different;
            }
            let aa = args(arena, &p.a);
            let ba = args(arena, &p.b);
            let ps = paired_args(arena, &aa, &ba);
            proof {
                reveal(crate::k2_engine::term_size);
            }
            EqStep::Children(ps)
        },
        _ => EqStep::Different,
    }
}

pub fn equal(arena: &ETermArena, a: &T, b: &T) -> (out: bool)
    requires
        arena_ok(arena),
        valid(arena.nodes@, a),
        valid(arena.nodes@, b),
    ensures
        out == (a@ == b@),
{
    let p = Pair { a: a.cp(), b: b.cp() };
    let mut tasks = Vec::new();
    tasks.push(p);
    proof {
        reveal(equal_pairs);
        assert(tasks.len() == 1);
        assert(tasks@[0].a@ == a@);
        assert(tasks@[0].b@ == b@);
        if a@ == b@ {
            assert forall|j: int| 0 <= j < tasks.len() implies (#[trigger] tasks@[j]).a@
                == tasks@[j].b@ by {
                assert(j == 0);
            }
        }
        if equal_pairs(tasks@) {
            assert(tasks@[0].a@ == tasks@[0].b@);
        }
        assert(equal_pairs(tasks@) == (a@ == b@));
    }
    while tasks.len() > 0
        invariant
            arena_ok(arena),
            valid(arena.nodes@, a),
            valid(arena.nodes@, b),
            pairs_valid(arena.nodes@, tasks@),
            (a@ == b@) == equal_pairs(tasks@),
        decreases pairs_work(tasks@),
    {
        let ghost before = tasks@;
        let p = tasks.pop().unwrap();
        proof {
            assert(before == tasks@ + seq![p]);
            pairs_concat(tasks@, seq![p]);
            reveal_with_fuel(pairs_work, 2);
        }
        match compare_step(arena, &p) {
            EqStep::Different => return false,
            EqStep::Equal => {},
            EqStep::Children(mut children) => {
                let ghost rest = tasks@;
                let ghost cs = children@;
                proof {
                    pairs_concat(rest, cs);
                }
                tasks.append(&mut children);
                proof {
                    assert forall|j: int| 0 <= j < tasks.len() implies #[trigger] pair_valid(
                        arena.nodes@,
                        &tasks@[j],
                    ) by {
                        if j < rest.len() {
                            assert(tasks@[j] == rest[j]);
                        } else {
                            assert(tasks@[j] == cs[j - rest.len()]);
                        }
                    }
                }
            },
        }
    }
    true
}

pub open spec fn parts_view(p: Option<(Vec<u8>, Vec<T>)>) -> Option<(Seq<u8>, Seq<Term>)> {
    match p {
        Some((n, a)) => Some((n@, models(a@))),
        None => None,
    }
}

pub fn parts(arena: &ETermArena, t: &T) -> (out: Option<(Vec<u8>, Vec<T>)>)
    requires
        arena_ok(arena),
        valid(arena.nodes@, t),
    ensures
        out matches Some((n, a)) ==> valid_all(arena.nodes@, a@),
        parts_view(out) == match t@ {
            Term::Comp(n, a) => Some((n, a)),
            _ => None,
        },
{
    proof {
        assert(node_ok(arena.nodes@, t.root as int));
        reveal(node_ok);
    }
    match &arena.nodes[t.root].kind {
        ENodeKind::Comp { name, .. } => Some((name.clone(), args(arena, t))),
        _ => None,
    }
}

pub fn concat(left: &Vec<T>, right: &Vec<T>) -> (out: Vec<T>)
    ensures
        out@ == left@ + right@,
        models(out@) == models(left@) + models(right@),
{
    let mut out = copy(left);
    let mut rest = copy(right);
    out.append(&mut rest);
    proof {
        assert_seqs_equal!(models(out@) == models(left@) + models(right@));
    }
    out
}

pub fn tail(ts: &Vec<T>, i: usize) -> (out: Vec<T>)
    requires
        i <= ts.len(),
    ensures
        out@ == ts@.skip(i as int),
        models(out@) == models(ts@).skip(i as int),
{
    let mut out = Vec::new();
    let mut j = i;
    while j < ts.len()
        invariant
            i <= j <= ts.len(),
            out@ == ts@.subrange(i as int, j as int),
        decreases ts.len() - j,
    {
        let t = ts[j].cp();
        proof {
            assert(t == ts@[j as int]);
        }
        out.push(t);
        j += 1;
        proof {
            assert_seqs_equal!(out@ == ts@.subrange(i as int, j as int));
        }
    }
    proof {
        assert_seqs_equal!(models(out@) == models(ts@).skip(i as int));
    }
    out
}

} // verus!
