#[cfg(verus_keep_ghost)]
use crate::k2_term::arena_ok;
use crate::k2_term::{ENode, ETermArena};
use crate::m6_drs::box_parts;
#[cfg(verus_keep_ghost)]
use crate::m6_flat::flat_valid_result;
use crate::m6_flat::{Env, flatten_list};
use crate::m6_group::error;
use crate::m6_model::*;
use crate::m6_symbols::Sym;
#[cfg(verus_keep_ghost)]
use crate::m6_symbols::symbol;
use crate::m6_term::*;
use ckc_spec::emit as spec;
use ckc_spec::term::Term;
use ckc_spec::v1text;
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub struct Seg {
    pub dom: Vec<T>,
    pub conds: T,
}

impl View for Seg {
    type V = spec::Seg;

    open spec fn view(&self) -> Self::V {
        spec::Seg { dom: models(self.dom@), conds: self.conds@ }
    }
}

pub open spec fn seg_valid(nodes: Seq<ENode>, s: &Seg) -> bool {
    valid_all(nodes, s.dom@) && valid(nodes, &s.conds)
}

pub open spec fn segs_valid(nodes: Seq<ENode>, ss: Seq<Seg>) -> bool {
    forall|i: int| 0 <= i < ss.len() ==> #[trigger] seg_valid(nodes, &ss[i])
}

pub open spec fn seg_models(ss: Seq<Seg>) -> Seq<spec::Seg> {
    ss.map_values(|s: Seg| s@)
}

pub struct Curry {
    pub segs: Vec<Seg>,
    pub cons: T,
}

impl View for Curry {
    type V = (Seq<spec::Seg>, Term);

    open spec fn view(&self) -> Self::V {
        (seg_models(self.segs@), self.cons@)
    }
}

pub open spec fn curry_valid(nodes: Seq<ENode>, c: &Curry) -> bool {
    segs_valid(nodes, c.segs@) && valid(nodes, &c.cons)
}

pub open spec fn curry_result(r: Result<Curry, T>) -> Result<(Seq<spec::Seg>, Term), Term> {
    match r {
        Ok(c) => Ok(c@),
        Err(e) => Err(e@),
    }
}

pub open spec fn curry_result_valid(nodes: Seq<ENode>, r: &Result<Curry, T>) -> bool {
    match r {
        Ok(c) => curry_valid(nodes, c),
        Err(e) => valid(nodes, e),
    }
}

pub fn curry_one(arena: &ETermArena, seg: Seg, cons: &T) -> (out: Curry)
    requires
        arena_ok(arena),
        seg_valid(arena.nodes@, &seg),
        valid(arena.nodes@, cons),
    ensures
        curry_valid(arena.nodes@, &out),
        out@ == (seq![seg@], cons@),
{
    let mut segs = Vec::new();
    segs.push(seg);
    proof {
        assert_seqs_equal!(seg_models(segs@) == seq![seg@]);
    }
    Curry { segs, cons: cons.cp() }
}

pub fn curry_prepend(arena: &ETermArena, seg: Seg, mut tail: Curry) -> (out: Curry)
    requires
        arena_ok(arena),
        seg_valid(arena.nodes@, &seg),
        curry_valid(arena.nodes@, &tail),
    ensures
        curry_valid(arena.nodes@, &out),
        out@ == (seq![seg@] + tail@.0, tail@.1),
{
    let ghost rest = tail.segs@;
    let mut segs = Vec::new();
    segs.push(seg);
    segs.append(&mut tail.segs);
    proof {
        assert_seqs_equal!(seg_models(segs@) == seq![seg@] + seg_models(rest));
        assert forall|i: int| 0 <= i < segs.len() implies #[trigger] seg_valid(
            arena.nodes@,
            &segs@[i],
        ) by {
            if i > 0 {
                assert(segs@[i] == rest[i - 1]);
            }
        }
    }
    Curry { segs, cons: tail.cons }
}

pub fn curry(arena: &mut ETermArena, ante: &T, cons: &T, fuel: usize) -> (out: Result<Curry, T>)
    requires
        arena_ok(old(arena)),
        valid(old(arena).nodes@, ante),
        valid(old(arena).nodes@, cons),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        curry_result_valid(final(arena).nodes@, &out),
        out is Ok ==> final(arena).nodes@ == old(arena).nodes@,
        curry_result(out) == spec::curry(ante@, cons@, fuel as nat),
    decreases fuel,
{
    let ghost start = arena.nodes@;
    proof {
        reveal_with_fuel(spec::curry, 1);
    }
    let b = match box_parts(arena, ante) {
        Some(b) => b,
        None => return Err(error(arena, &Sym::InvalidDrsShape, Ghost(start))),
    };
    let seg = Seg { dom: b.dom, conds: b.conds };
    if fuel > 0 && is_comp(arena, cons, &Sym::Drs, 2) {
        let ca = args(arena, cons);
        if is_nil(arena, &ca[0]) && is_comp(arena, &ca[1], &Sym::Cons, 2) {
            let cl = args(arena, &ca[1]);
            if is_nil(arena, &cl[1]) && is_comp(arena, &cl[0], &Sym::Implies, 2) {
                let rule = args(arena, &cl[0]);
                return match curry(arena, &rule[0], &rule[1], fuel - 1) {
                    Err(e) => Err(e),
                    Ok(tail) => Ok(curry_prepend(arena, seg, tail)),
                };
            }
        }
    }
    Ok(curry_one(arena, seg, cons))
}

pub fn seg_domain(arena: &ETermArena, segs: &Vec<Seg>) -> (out: Vec<T>)
    requires
        arena_ok(arena),
        segs_valid(arena.nodes@, segs@),
    ensures
        valid_all(arena.nodes@, out@),
        models(out@) == spec::seg_domain(seg_models(segs@)),
{
    let mut out = Vec::new();
    let mut i = 0usize;
    proof {
        assert(seg_models(segs@).skip(0) == seg_models(segs@));
        assert_seqs_equal!(models(out@) == Seq::<Term>::empty());
    }
    while i < segs.len()
        invariant
            arena_ok(arena),
            segs_valid(arena.nodes@, segs@),
            valid_all(arena.nodes@, out@),
            i <= segs.len(),
            models(out@) + spec::seg_domain(seg_models(segs@).skip(i as int)) == spec::seg_domain(
                seg_models(segs@),
            ),
        decreases segs.len() - i,
    {
        proof {
            assert(seg_valid(arena.nodes@, &segs@[i as int]));
            assert(seg_models(segs@).skip(i as int).drop_first() == seg_models(segs@).skip(
                i as int + 1,
            ));
            reveal_with_fuel(spec::seg_domain, 1);
        }
        out = concat(&out, &segs[i].dom);
        i += 1;
    }
    proof {
        reveal(spec::seg_domain);
    }
    out
}

pub struct Split {
    pub shared: Vec<T>,
    pub arms: Vec<Binding>,
}

impl View for Split {
    type V = (Seq<Term>, Seq<(Term, Term)>);

    open spec fn view(&self) -> Self::V {
        (models(self.shared@), map_model(self.arms@))
    }
}

pub open spec fn split_valid(nodes: Seq<ENode>, s: &Split) -> bool {
    valid_all(nodes, s.shared@) && map_valid(nodes, s.arms@)
}

pub open spec fn split_result(s: Option<Split>) -> Option<(Seq<Term>, Seq<(Term, Term)>)> {
    match s {
        Some(s) => Some(s@),
        None => None,
    }
}

pub fn empty_split(arena: &ETermArena) -> (out: Split)
    requires
        arena_ok(arena),
    ensures
        split_valid(arena.nodes@, &out),
        out@ == (Seq::<Term>::empty(), Seq::<(Term, Term)>::empty()),
{
    let shared = Vec::new();
    let arms = Vec::new();
    proof {
        assert_seqs_equal!(models(shared@) == Seq::<Term>::empty());
        assert_seqs_equal!(map_model(arms@) == Seq::<(Term, Term)>::empty());
    }
    Split { shared, arms }
}

pub fn join_split(arena: &ETermArena, mut a: Split, mut b: Split) -> (out: Split)
    requires
        arena_ok(arena),
        split_valid(arena.nodes@, &a),
        split_valid(arena.nodes@, &b),
    ensures
        split_valid(arena.nodes@, &out),
        out@ == (a@.0 + b@.0, a@.1 + b@.1),
{
    let shared = concat(&a.shared, &b.shared);
    let ghost first = a.arms@;
    let ghost last = b.arms@;
    a.arms.append(&mut b.arms);
    proof {
        assert_seqs_equal!(map_model(a.arms@) == map_model(first) + map_model(last));
        assert forall|i: int| 0 <= i < a.arms.len() implies #[trigger] binding_valid(
            arena.nodes@,
            &a.arms@[i],
        ) by {
            if i < first.len() {
                assert(a.arms@[i] == first[i]);
            } else {
                assert(a.arms@[i] == last[i - first.len()]);
            }
        }
    }
    Split { shared, arms: a.arms }
}

pub fn split_item(arena: &ETermArena, t: &T) -> (out: Split)
    requires
        arena_ok(arena),
        valid(arena.nodes@, t),
    ensures
        split_valid(arena.nodes@, &out),
        out@ == if spec::is_comp(t@, "v"@, 2) {
            (
                Seq::<Term>::empty(),
                seq![(ckc_spec::replay::arg(t@, 0), ckc_spec::replay::arg(t@, 1))],
            )
        } else {
            (seq![t@], Seq::<(Term, Term)>::empty())
        },
{
    let mut shared = Vec::new();
    let mut arms = Vec::new();
    if is_comp(arena, t, &Sym::V, 2) {
        let a = args(arena, t);
        arms.push(Binding { key: a[0].cp(), value: a[1].cp() });
        proof {
            assert_seqs_equal!(models(shared@) == Seq::<Term>::empty());
            assert_seqs_equal!(map_model(arms@) == seq![(a[0]@, a[1]@)]);
        }
    } else {
        shared.push(t.cp());
        proof {
            assert_seqs_equal!(models(shared@) == seq![t@]);
            assert_seqs_equal!(map_model(arms@) == Seq::<(Term, Term)>::empty());
        }
    }
    Split { shared, arms }
}

pub fn split_conds(arena: &ETermArena, l: &T) -> (out: Option<Split>)
    requires
        arena_ok(arena),
        valid(arena.nodes@, l),
    ensures
        out matches Some(s) ==> split_valid(arena.nodes@, &s),
        split_result(out) == spec::split_conds(l@),
    decreases l@,
{
    proof {
        reveal_with_fuel(spec::split_conds, 1);
        reveal_strlit("[|]");
        reveal(v1text::ascii);
        assert_seqs_equal!(symbol(&Sym::Cons) == v1text::cons_name());
    }
    if is_nil(arena, l) {
        return Some(empty_split(arena));
    }
    match parts(arena, l) {
        Some((name, a)) => {
            proof {
                assert(l@ == Term::Comp(name@, models(a@)));
            }
            if has_name(&name, &Sym::Cons) && a.len() == 2 {
                match split_conds(arena, &a[1]) {
                    None => None,
                    Some(rest) => {
                        let head = split_item(arena, &a[0]);
                        Some(join_split(arena, head, rest))
                    },
                }
            } else {
                None
            }
        },
        None => None,
    }
}

pub fn split_scan_from(arena: &ETermArena, segs: &Vec<Seg>, i: usize) -> (out: Option<Split>)
    requires
        arena_ok(arena),
        segs_valid(arena.nodes@, segs@),
        i <= segs.len(),
    ensures
        out matches Some(s) ==> split_valid(arena.nodes@, &s),
        split_result(out) == spec::split_scan(seg_models(segs@).skip(i as int)),
    decreases segs.len() - i,
{
    proof {
        reveal_with_fuel(spec::split_scan, 1);
    }
    if i == segs.len() {
        return Some(empty_split(arena));
    }
    proof {
        assert(seg_valid(arena.nodes@, &segs@[i as int]));
        assert(seg_models(segs@).skip(i as int).drop_first() == seg_models(segs@).skip(
            i as int + 1,
        ));
    }
    let head = match split_conds(arena, &segs[i].conds) {
        Some(s) => s,
        None => return None,
    };
    match split_scan_from(arena, segs, i + 1) {
        None => None,
        Some(rest) => Some(join_split(arena, head, rest)),
    }
}

pub fn flatten_at(arena: &mut ETermArena, conds: &T, w: &W, env: &Env, deps: &T, n: usize) -> (out:
    Result<Flat, T>)
    requires
        arena_ok(old(arena)),
        valid(old(arena).nodes@, conds),
        valid(old(arena).nodes@, deps),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        flat_valid_result(final(arena).nodes@, &out),
        flat_result(out) == spec::flatten_list(
            conds@,
            w@,
            env.s as nat,
            env.docid@,
            deps@,
            spec::actual(),
            spec::Encl::Top,
            n as nat,
            env.base as nat,
        ),
{
    let ghost start = arena.nodes@;
    let actual = named(arena, &Sym::Actual);
    let ghost middle = arena.nodes@;
    proof {
        prefix(start, middle, conds);
        prefix(start, middle, deps);
    }
    let out = flatten_list(arena, conds, w, env, deps, &actual, &E::Top, n);
    proof {
        crate::k2_load::prefix_chain(start, middle, arena.nodes@);
    }
    out
}

pub fn flatten_ante(arena: &mut ETermArena, conds: &T, env: &Env, n: usize) -> (out: Result<
    Flat,
    T,
>)
    requires
        arena_ok(old(arena)),
        valid(old(arena).nodes@, conds),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        flat_valid_result(final(arena).nodes@, &out),
        flat_result(out) == spec::flatten_list(
            conds@,
            spec::Where::Antecedent,
            env.s as nat,
            env.docid@,
            Term::Nil,
            spec::actual(),
            spec::Encl::Top,
            n as nat,
            env.base as nat,
        ),
{
    let ghost start = arena.nodes@;
    let deps = nil(arena);
    let ghost middle = arena.nodes@;
    proof {
        prefix(start, middle, conds);
    }
    let out = flatten_at(arena, conds, &W::Antecedent, env, &deps, n);
    proof {
        crate::k2_load::prefix_chain(start, middle, arena.nodes@);
    }
    out
}

pub fn flatten_seq(arena: &mut ETermArena, conds: &Vec<T>, env: &Env, n: usize) -> (out: Result<
    Flat,
    T,
>)
    requires
        arena_ok(old(arena)),
        valid_all(old(arena).nodes@, conds@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        flat_valid_result(final(arena).nodes@, &out),
        flat_result(out) == spec::flatten_seq(
            models(conds@),
            env.s as nat,
            env.docid@,
            n as nat,
            env.base as nat,
        ),
{
    let ghost start = arena.nodes@;
    let l = list(arena, conds);
    let ghost middle = arena.nodes@;
    let out = flatten_ante(arena, &l, env, n);
    proof {
        crate::k2_load::prefix_chain(start, middle, arena.nodes@);
    }
    out
}

pub fn flatten_cons(arena: &mut ETermArena, conds: &T, env: &Env, deps: &T, n: usize) -> (out:
    Result<Flat, T>)
    requires
        arena_ok(old(arena)),
        valid(old(arena).nodes@, conds),
        valid(old(arena).nodes@, deps),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        flat_valid_result(final(arena).nodes@, &out),
        flat_result(out) == spec::flatten_cons(
            conds@,
            env.s as nat,
            env.docid@,
            deps@,
            n as nat,
            env.base as nat,
        ),
{
    flatten_at(arena, conds, &W::Consequent, env, deps, n)
}

pub fn concat_items(arena: &ETermArena, a: &Vec<I>, b: &Vec<I>) -> (out: Vec<I>)
    requires
        arena_ok(arena),
        items_valid(arena.nodes@, a@),
        items_valid(arena.nodes@, b@),
    ensures
        items_valid(arena.nodes@, out@),
        item_models(out@) == item_models(a@) + item_models(b@),
{
    let mut out = copy_items(arena, a);
    let mut rest = copy_items(arena, b);
    let ghost first = out@;
    let ghost last = rest@;
    proof {
        items_concat(arena.nodes@, first, last);
    }
    out.append(&mut rest);
    proof {
        assert_seqs_equal!(item_models(out@) == item_models(a@) + item_models(b@));
    }
    out
}

} // verus!
