use crate::k2_term::ETermArena;
#[cfg(verus_keep_ghost)]
use crate::k2_term::arena_ok;
use crate::m6_ante::{Seg, curry};
#[cfg(verus_keep_ghost)]
use crate::m6_ante::{seg_models, seg_valid, segs_valid};
use crate::m6_drs::{ERoot, anchor, atom_name, box_parts, group_leaves};
#[cfg(verus_keep_ghost)]
use crate::m6_drs::{root_models, root_valid, roots_valid, sizes_concat, term_opt};
use crate::m6_model::*;
use crate::m6_refs::contains;
#[cfg(verus_keep_ghost)]
use crate::m6_rules::{groups_result, groups_result_valid};
use crate::m6_symbols::Sym;
#[cfg(verus_keep_ghost)]
use crate::m6_symbols::symbol;
use crate::m6_term::*;
#[cfg(verus_keep_ghost)]
use crate::m7_annotate::tab_view;
use crate::m7_annotate::{arg_at, int_cmp};
use crate::m7_q12::EEnv;
use crate::m7_temporal::EVocabulary;
use ckc_spec::emit as spec;
use ckc_spec::term::Term;
use ckc_spec::v1text::DocClause;
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub fn deontic(name: &Vec<u8>) -> (out: bool)
    ensures
        out == spec::deontic(name@),
{
    has_name(name, &Sym::Should) || has_name(name, &Sym::Must) || has_name(name, &Sym::May)
}

proof fn deontic_concat(a: Seq<Term>, b: Seq<Term>)
    ensures
        spec::has_deontic_all(a + b) == (spec::has_deontic_all(a) || spec::has_deontic_all(b)),
        spec::negated_deontic_all(a + b) == (spec::negated_deontic_all(a)
            || spec::negated_deontic_all(b)),
    decreases a.len(),
{
    if a.len() > 0 {
        assert((a + b).drop_first() == a.drop_first() + b);
        deontic_concat(a.drop_first(), b);
    }
    reveal_with_fuel(spec::has_deontic_all, 2);
    reveal_with_fuel(spec::negated_deontic_all, 2);
}

pub fn has_deontic(arena: &ETermArena, t: &T) -> (out: bool)
    requires
        arena_ok(arena),
        valid(arena.nodes@, t),
    ensures
        out == spec::has_deontic(t@),
{
    let mut todo = Vec::new();
    todo.push(t.cp());
    proof {
        reveal_with_fuel(spec::has_deontic_all, 2);
    }
    while todo.len() > 0
        invariant
            arena_ok(arena),
            valid_all(arena.nodes@, todo@),
            spec::has_deontic_all(models(todo@)) == spec::has_deontic(t@),
        decreases crate::k2_engine::terms_size(models(todo@)),
    {
        let ghost before = todo@;
        let current = todo.remove(0);
        proof {
            assert(current == before[0]);
            assert(models(before)[0] == current@);
            assert(models(before).drop_first() == models(todo@));
            reveal_with_fuel(spec::has_deontic_all, 1);
            reveal_with_fuel(spec::has_deontic, 1);
            reveal_with_fuel(crate::k2_engine::terms_size, 1);
            assert(spec::has_deontic_all(models(before)) == (spec::has_deontic(current@)
                || spec::has_deontic_all(models(todo@))));
            assert(crate::k2_engine::terms_size(models(before)) == crate::k2_engine::term_size(
                current@,
            ) + crate::k2_engine::terms_size(models(todo@)));
        }
        if let Some((name, children)) = parts(arena, &current) {
            proof {
                assert(current@ == Term::Comp(name@, models(children@)));
            }
            if deontic(&name) && children.len() == 1 {
                proof {
                    assert(spec::has_deontic(current@));
                    assert(spec::has_deontic(t@));
                }
                return true;
            }
            proof {
                deontic_concat(models(children@), models(todo@));
                sizes_concat(models(children@), models(todo@));
                reveal(crate::k2_engine::term_size);
            }
            todo = concat(&children, &todo);
        } else {
            proof {
                reveal(crate::k2_engine::term_size);
            }
        }
    }
    proof {
        reveal(spec::has_deontic_all);
    }
    false
}

pub fn negated_deontic(arena: &ETermArena, t: &T) -> (out: bool)
    requires
        arena_ok(arena),
        valid(arena.nodes@, t),
    ensures
        out == spec::negated_deontic(t@),
{
    let mut todo = Vec::new();
    todo.push(t.cp());
    proof {
        reveal_with_fuel(spec::negated_deontic_all, 2);
    }
    while todo.len() > 0
        invariant
            arena_ok(arena),
            valid_all(arena.nodes@, todo@),
            spec::negated_deontic_all(models(todo@)) == spec::negated_deontic(t@),
        decreases crate::k2_engine::terms_size(models(todo@)),
    {
        let ghost before = todo@;
        let current = todo.remove(0);
        proof {
            assert(current == before[0]);
            assert(models(before)[0] == current@);
            assert(models(before).drop_first() == models(todo@));
            reveal_with_fuel(spec::negated_deontic_all, 1);
            reveal_with_fuel(spec::negated_deontic, 1);
            reveal_with_fuel(crate::k2_engine::terms_size, 1);
            assert(spec::negated_deontic_all(models(before)) == (spec::negated_deontic(current@)
                || spec::negated_deontic_all(models(todo@))));
            assert(crate::k2_engine::terms_size(models(before)) == crate::k2_engine::term_size(
                current@,
            ) + crate::k2_engine::terms_size(models(todo@)));
        }
        if let Some((name, children)) = parts(arena, &current) {
            proof {
                assert(current@ == Term::Comp(name@, models(children@)));
            }
            if has_name(&name, &Sym::Minus) && children.len() == 1 {
                if has_deontic(arena, &children[0]) {
                    proof {
                        assert(spec::negated_deontic(current@));
                        assert(spec::negated_deontic(t@));
                    }
                    return true;
                }
                proof {
                    reveal(crate::k2_engine::term_size);
                }
            } else {
                proof {
                    deontic_concat(models(children@), models(todo@));
                    sizes_concat(models(children@), models(todo@));
                    reveal(crate::k2_engine::term_size);
                }
                todo = concat(&children, &todo);
            }
        } else {
            proof {
                reveal(crate::k2_engine::term_size);
            }
        }
    }
    proof {
        reveal(spec::negated_deontic_all);
    }
    false
}

pub fn leaves_of(arena: &ETermArena, conds: &Vec<T>) -> (out: Option<Vec<T>>)
    requires
        arena_ok(arena),
        valid_all(arena.nodes@, conds@),
    ensures
        out matches Some(ts) ==> valid_all(arena.nodes@, ts@),
        option_models(out) == spec::leaves_of(models(conds@)),
{
    let ghost ms = models(conds@);
    let mut out = Vec::new();
    let mut i = 0usize;
    proof {
        assert(ms.skip(0) == ms);
        assert(models(out@) == Seq::<Term>::empty());
    }
    while i < conds.len()
        invariant
            arena_ok(arena),
            valid_all(arena.nodes@, conds@),
            valid_all(arena.nodes@, out@),
            ms == models(conds@),
            i <= conds.len(),
            spec::leaves_of(ms) == match spec::leaves_of(ms.skip(i as int)) {
                Some(rest) => Some(models(out@) + rest),
                None => None,
            },
        decreases conds.len() - i,
    {
        proof {
            assert(ms.skip(i as int)[0] == conds@[i as int]@);
            assert(ms.skip(i as int).drop_first() == ms.skip(i + 1));
            reveal_with_fuel(spec::leaves_of, 1);
        }
        if anchor(arena, &conds[i]).is_none() {
            return None;
        }
        let leaf = arg_at(arena, &conds[i], 0);
        proof {
            extend(arena.nodes@, out@, leaf);
        }
        let ghost previous = out@;
        out.push(leaf);
        proof {
            match spec::leaves_of(ms.skip(i + 1)) {
                Some(rest) => {
                    assert(models(out@) + rest == models(previous) + (seq![leaf@] + rest));
                },
                None => {},
            }
        }
        i += 1;
    }
    proof {
        reveal_with_fuel(spec::leaves_of, 1);
    }
    Some(out)
}

fn atom_named(arena: &ETermArena, t: &T, name: &Vec<u8>) -> (out: bool)
    requires
        arena_ok(arena),
        valid(arena.nodes@, t),
    ensures
        out == (t@ == Term::Atom(name@)),
{
    match atom_name(arena, t) {
        Some(n) => bytes_eq(&n, name),
        None => false,
    }
}

spec fn verb_condition(tab: ckc_spec::vocabulary::Vocabulary, t: Term) -> bool {
    spec::is_pred(t) && ckc_spec::replay::arg(t, 1) == Term::Atom(tab.exclusions[0])
}

fn names_verb_leaf(arena: &ETermArena, tab: &EVocabulary, t: &T) -> (out: bool)
    requires
        arena_ok(arena),
        valid(arena.nodes@, t),
        tab.exclusions.len() > 0,
    ensures
        out == verb_condition(tab@, t@),
{
    if !crate::m7_strength::is_pred(arena, t) {
        return false;
    }
    let name = arg_at(arena, t, 1);
    atom_named(arena, &name, &tab.exclusions[0])
}

pub open spec fn verb_view(t: Option<T>) -> Term {
    match t {
        Some(t) => t@,
        None => Term::Nil,
    }
}

pub fn verb_leaf(arena: &ETermArena, tab: &EVocabulary, ls: &Vec<T>) -> (out: Option<T>)
    requires
        arena_ok(arena),
        valid_all(arena.nodes@, ls@),
        tab.exclusions.len() > 0,
    ensures
        out matches Some(t) ==> valid(arena.nodes@, &t) && spec::is_pred(t@),
        out is Some == spec::names_verb(tab@, models(ls@)),
        verb_view(out) == spec::verb_leaf(tab@, models(ls@)),
{
    let ghost ms = models(ls@);
    let mut i = 0usize;
    proof {
        assert(ms.skip(0) == ms);
    }
    while i < ls.len()
        invariant
            arena_ok(arena),
            valid_all(arena.nodes@, ls@),
            tab.exclusions.len() > 0,
            ms == models(ls@),
            i <= ls.len(),
            spec::verb_leaf(tab@, ms) == spec::verb_leaf(tab@, ms.skip(i as int)),
            forall|j: int| 0 <= j < i ==> !verb_condition(tab@, #[trigger] ms[j]),
        decreases ls.len() - i,
    {
        proof {
            assert(ms.skip(i as int)[0] == ls@[i as int]@);
            assert(ms.skip(i as int).drop_first() == ms.skip(i + 1));
            reveal_with_fuel(spec::verb_leaf, 1);
        }
        if names_verb_leaf(arena, tab, &ls[i]) {
            proof {
                assert(verb_condition(tab@, ms[i as int]));
                assert(spec::names_verb(tab@, ms));
            }
            return Some(ls[i].cp());
        }
        i += 1;
    }
    proof {
        reveal_with_fuel(spec::verb_leaf, 1);
        assert forall|j: int| 0 <= j < ms.len() implies !(spec::is_pred(#[trigger] ms[j])
            && ckc_spec::replay::arg(ms[j], 1) == Term::Atom(tab@.exclusions[0])) by {
            assert(!verb_condition(tab@, ms[j]));
        }
    }
    None
}

pub fn names_verb(arena: &ETermArena, tab: &EVocabulary, ls: &Vec<T>) -> (out: bool)
    requires
        arena_ok(arena),
        valid_all(arena.nodes@, ls@),
        tab.exclusions.len() > 0,
    ensures
        out == spec::names_verb(tab@, models(ls@)),
{
    verb_leaf(arena, tab, ls).is_some()
}

pub fn noun_in(arena: &ETermArena, ls: &Vec<T>, x: &T) -> (out: Option<T>)
    requires
        arena_ok(arena),
        valid_all(arena.nodes@, ls@),
        valid(arena.nodes@, x),
    ensures
        out matches Some(t) ==> valid(arena.nodes@, &t),
        term_opt(out) == spec::noun_in(models(ls@), x@),
{
    let ghost ms = models(ls@);
    let mut i = 0usize;
    proof {
        assert(ms.skip(0) == ms);
    }
    while i < ls.len()
        invariant
            arena_ok(arena),
            valid_all(arena.nodes@, ls@),
            valid(arena.nodes@, x),
            ms == models(ls@),
            i <= ls.len(),
            spec::noun_in(ms, x@) == spec::noun_in(ms.skip(i as int), x@),
        decreases ls.len() - i,
    {
        proof {
            assert(ms.skip(i as int)[0] == ls@[i as int]@);
            assert(ms.skip(i as int).drop_first() == ms.skip(i + 1));
            reveal_with_fuel(spec::noun_in, 1);
        }
        if is_comp(arena, &ls[i], &Sym::Object, 6) {
            let object = arg_at(arena, &ls[i], 0);
            if crate::m6_term::equal(arena, &object, x) {
                return Some(arg_at(arena, &ls[i], 1));
            }
        }
        i += 1;
    }
    proof {
        reveal_with_fuel(spec::noun_in, 1);
    }
    None
}

pub fn pps_ok(arena: &ETermArena, ls: &Vec<T>, e: &T, dom: &Vec<T>) -> (out: bool)
    requires
        arena_ok(arena),
        valid_all(arena.nodes@, ls@),
        valid(arena.nodes@, e),
        valid_all(arena.nodes@, dom@),
    ensures
        out == spec::pps_ok(models(ls@), e@, models(dom@)),
{
    let ghost ms = models(ls@);
    let mut i = 0usize;
    while i < ls.len()
        invariant
            arena_ok(arena),
            valid_all(arena.nodes@, ls@),
            valid(arena.nodes@, e),
            valid_all(arena.nodes@, dom@),
            ms == models(ls@),
            i <= ls.len(),
            forall|j: int|
                0 <= j < i ==> spec::is_comp(#[trigger] ms[j], "modifier_pp"@, 3)
                    && ckc_spec::replay::arg(ms[j], 0) == e@ && !models(dom@).contains(
                    ckc_spec::replay::arg(ms[j], 2),
                ),
        decreases ls.len() - i,
    {
        proof {
            assert(ms[i as int] == ls@[i as int]@);
        }
        if !is_comp(arena, &ls[i], &Sym::ModifierPp, 3) {
            return false;
        }
        let event = arg_at(arena, &ls[i], 0);
        let object = arg_at(arena, &ls[i], 2);
        if !crate::m6_term::equal(arena, &event, e) || contains(arena, dom, &object) {
            return false;
        }
        i += 1;
    }
    true
}

pub fn exclusion_box(arena: &ETermArena, tab: &EVocabulary, dom: &Vec<T>, conds: &Vec<T>) -> (out:
    bool)
    requires
        arena_ok(arena),
        valid_all(arena.nodes@, dom@),
        valid_all(arena.nodes@, conds@),
        tab.scopes.len() > 0,
        tab.exclusions.len() > 0,
    ensures
        out == spec::exclusion_box(tab@, models(dom@), models(conds@)),
{
    let ls = match leaves_of(arena, conds) {
        Some(ls) => ls,
        None => return false,
    };
    if ls.len() < 2 || !is_comp(arena, &ls[0], &Sym::Object, 6) {
        return false;
    }
    let obj = args(arena, &ls[0]);
    if !atom_named(arena, &obj[1], &tab.scopes[0].0) || !is_atom(arena, &obj[2], &Sym::Countable)
        || !is_atom(arena, &obj[3], &Sym::Na) || !is_atom(arena, &obj[4], &Sym::Eq) || !int_cmp(
        arena,
        &obj[5],
        true,
    ) || !is_comp(arena, &ls[1], &Sym::Predicate, 3) {
        return false;
    }
    let event = args(arena, &ls[1]);
    if !atom_named(arena, &event[1], &tab.exclusions[0]) || !crate::m6_term::equal(
        arena,
        &event[2],
        &obj[0],
    ) || !contains(arena, dom, &obj[0]) || !contains(arena, dom, &event[0]) {
        return false;
    }
    let pps = tail(&ls, 2);
    pps_ok(arena, &pps, &event[0], dom)
}

pub open spec fn box_names_verb(tab: ckc_spec::vocabulary::Vocabulary, t: Term) -> bool {
    match spec::box_parts(t) {
        Some((_, l)) => ckc_spec::answers::list_items(l) is Some && spec::names_verb(
            tab,
            spec::group_leaves(l),
        ),
        None => false,
    }
}

pub fn verb_box(arena: &ETermArena, tab: &EVocabulary, cs: &Vec<T>) -> (out: Option<T>)
    requires
        arena_ok(arena),
        valid_all(arena.nodes@, cs@),
        tab.exclusions.len() > 0,
    ensures
        out matches Some(t) ==> valid(arena.nodes@, &t) && box_names_verb(tab@, t@),
        term_opt(out) == spec::verb_box(tab@, models(cs@)),
{
    let ghost ms = models(cs@);
    let mut i = 0usize;
    proof {
        assert(ms.skip(0) == ms);
    }
    while i < cs.len()
        invariant
            arena_ok(arena),
            valid_all(arena.nodes@, cs@),
            tab.exclusions.len() > 0,
            ms == models(cs@),
            i <= cs.len(),
            spec::verb_box(tab@, ms) == spec::verb_box(tab@, ms.skip(i as int)),
        decreases cs.len() - i,
    {
        proof {
            assert(ms.skip(i as int)[0] == cs@[i as int]@);
            assert(ms.skip(i as int).drop_first() == ms.skip(i + 1));
            reveal_with_fuel(spec::verb_box, 1);
        }
        if is_comp(arena, &cs[i], &Sym::Minus, 1) {
            let inner = arg_at(arena, &cs[i], 0);
            if let Some(b) = box_parts(arena, &inner) {
                if items(arena, &b.conds).is_some() {
                    let ls = group_leaves(arena, &b.conds);
                    if names_verb(arena, tab, &ls) {
                        return Some(inner);
                    }
                }
            }
        }
        i += 1;
    }
    proof {
        reveal_with_fuel(spec::verb_box, 1);
    }
    None
}

pub enum Why {
    DeonticCondition,
    NegatedDeontic,
    RootDeontic,
    Root,
    Subject,
    Consequent,
}

impl View for Why {
    type V = Seq<char>;

    open spec fn view(&self) -> Self::V {
        match self {
            Why::DeonticCondition => "deontic_condition"@,
            Why::NegatedDeontic => "negated_deontic"@,
            Why::RootDeontic => "root_deontic"@,
            Why::Root => "root"@,
            Why::Subject => "subject"@,
            Why::Consequent => "consequent"@,
        }
    }
}

impl Why {
    pub fn symbol(&self) -> (out: Sym)
        ensures
            symbol(&out) == ckc_spec::v1text::ascii(self@),
    {
        match self {
            Why::DeonticCondition => Sym::DeonticCondition,
            Why::NegatedDeontic => Sym::NegatedDeontic,
            Why::RootDeontic => Sym::RootDeontic,
            Why::Root => Sym::Root,
            Why::Subject => Sym::Subject,
            Why::Consequent => Sym::Consequent,
        }
    }
}

pub enum Scope {
    Plain,
    Exclusion,
    Reject(Why),
}

impl View for Scope {
    type V = spec::Scope;

    open spec fn view(&self) -> Self::V {
        match self {
            Scope::Plain => spec::Scope::Plain,
            Scope::Exclusion => spec::Scope::Exclusion,
            Scope::Reject(why) => spec::Scope::Reject(why@),
        }
    }
}

fn deontic_segments(arena: &ETermArena, segs: &Vec<Seg>) -> (out: bool)
    requires
        arena_ok(arena),
        segs_valid(arena.nodes@, segs@),
    ensures
        out == (exists|i: int|
            0 <= i < segs.len() && spec::has_deontic(#[trigger] seg_models(segs@)[i].conds)),
{
    let mut i = 0usize;
    while i < segs.len()
        invariant
            arena_ok(arena),
            segs_valid(arena.nodes@, segs@),
            i <= segs.len(),
            forall|j: int| 0 <= j < i ==> !spec::has_deontic(#[trigger] seg_models(segs@)[j].conds),
        decreases segs.len() - i,
    {
        proof {
            assert(seg_valid(arena.nodes@, &segs@[i as int]));
        }
        if has_deontic(arena, &segs[i].conds) {
            proof {
                assert(spec::has_deontic(seg_models(segs@)[i as int].conds));
            }
            return true;
        }
        i += 1;
    }
    false
}

pub fn rule_scope(arena: &mut ETermArena, tab: &EVocabulary, ante: &T, cons: &T) -> (out: Scope)
    requires
        arena_ok(old(arena)),
        valid(old(arena).nodes@, ante),
        valid(old(arena).nodes@, cons),
        tab.scopes.len() > 0,
        tab.exclusions.len() > 0,
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out@ == spec::rule_scope(tab@, ante@, cons@),
{
    hide(spec::curry);
    let c = match curry(arena, ante, cons, 64) {
        Ok(c) => c,
        Err(_) => return Scope::Plain,
    };
    if deontic_segments(arena, &c.segs) {
        return Scope::Reject(Why::DeonticCondition);
    }
    if negated_deontic(arena, &c.cons) {
        return Scope::Reject(Why::NegatedDeontic);
    }
    let final_box = match box_parts(arena, &c.cons) {
        Some(b) => b,
        None => return Scope::Plain,
    };
    let cs = match items(arena, &final_box.conds) {
        Some(cs) => cs,
        None => return Scope::Plain,
    };
    let b = match verb_box(arena, tab, &cs) {
        Some(b) => b,
        None => return Scope::Plain,
    };
    let inner = match box_parts(arena, &b) {
        Some(inner) => inner,
        None => return Scope::Plain,
    };
    let bcs = match items(arena, &inner.conds) {
        Some(bcs) => bcs,
        None => return Scope::Plain,
    };
    let ls = group_leaves(arena, &inner.conds);
    let event = match verb_leaf(arena, tab, &ls) {
        Some(event) => event,
        None => return Scope::Plain,
    };
    let x = arg_at(arena, &event, 2);
    if !contains(arena, &inner.dom, &x) {
        return Scope::Reject(Why::Subject);
    }
    let noun = match noun_in(arena, &ls, &x) {
        Some(noun) => noun,
        None => return Scope::Reject(Why::Subject),
    };
    if !atom_named(arena, &noun, &tab.scopes[0].0) {
        return Scope::Reject(Why::Subject);
    }
    if cs.len() == 1 && exclusion_box(arena, tab, &inner.dom, &bcs) {
        Scope::Exclusion
    } else {
        Scope::Reject(Why::Consequent)
    }
}

spec fn root_has_deontic(r: spec::Root) -> bool {
    match r {
        spec::Root::Anchored(c) | spec::Root::Boxed(c) => spec::has_deontic(c),
        spec::Root::Rule(_, _) => false,
    }
}

spec fn root_has_verb(tab: ckc_spec::vocabulary::Vocabulary, r: spec::Root) -> bool {
    match r {
        spec::Root::Boxed(c) => spec::verb_box(tab, seq![c]) is Some,
        _ => false,
    }
}

pub fn root_scope(arena: &ETermArena, tab: &EVocabulary, roots: &Vec<ERoot>) -> (out: Scope)
    requires
        arena_ok(arena),
        roots_valid(arena.nodes@, roots@),
        tab.exclusions.len() > 0,
    ensures
        out@ == spec::root_scope(tab@, root_models(roots@)),
{
    let ghost ms = root_models(roots@);
    let mut i = 0usize;
    while i < roots.len()
        invariant
            arena_ok(arena),
            roots_valid(arena.nodes@, roots@),
            ms == root_models(roots@),
            i <= roots.len(),
            forall|j: int| 0 <= j < i ==> !root_has_deontic(#[trigger] ms[j]),
        decreases roots.len() - i,
    {
        proof {
            assert(root_valid(arena.nodes@, &roots@[i as int]));
        }
        let found = match &roots[i] {
            ERoot::Anchored(c) | ERoot::Boxed(c) => has_deontic(arena, c),
            ERoot::Rule(_, _) => false,
        };
        if found {
            proof {
                assert(root_has_deontic(ms[i as int]));
            }
            return Scope::Reject(Why::RootDeontic);
        }
        i += 1;
    }
    let mut j = 0usize;
    while j < roots.len()
        invariant
            arena_ok(arena),
            roots_valid(arena.nodes@, roots@),
            tab.exclusions.len() > 0,
            ms == root_models(roots@),
            j <= roots.len(),
            forall|k: int| 0 <= k < ms.len() ==> !root_has_deontic(#[trigger] ms[k]),
            forall|k: int| 0 <= k < j ==> !root_has_verb(tab@, #[trigger] ms[k]),
        decreases roots.len() - j,
    {
        proof {
            assert(root_valid(arena.nodes@, &roots@[j as int]));
        }
        if let ERoot::Boxed(c) = &roots[j] {
            let mut cs = Vec::new();
            cs.push(c.cp());
            proof {
                assert(models(cs@) == seq![c@]);
            }
            if verb_box(arena, tab, &cs).is_some() {
                proof {
                    assert(root_has_verb(tab@, ms[j as int]));
                }
                return Scope::Reject(Why::Root);
            }
        }
        j += 1;
    }
    Scope::Plain
}

pub fn exclusion_record(arena: &mut ETermArena, key: &Vec<u8>) -> (out: T)
    requires
        arena_ok(old(arena)),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid(final(arena).nodes@, &out),
        out@ == spec::exclusion_record(key@),
{
    let ghost start = arena.nodes@;
    let name = atom(arena, key);
    let ghost middle = arena.nodes@;
    let out = c1(arena, &Sym::GuidelineExclusion, &name);
    proof {
        crate::k2_load::prefix_chain(start, middle, arena.nodes@);
    }
    out
}

pub fn deontic_edge(arena: &ETermArena, h: &T) -> (out: bool)
    requires
        arena_ok(arena),
        valid(arena.nodes@, h),
    ensures
        out == spec::deontic_edge(h@),
{
    if !is_comp(arena, h, &Sym::GuidelineOperator, 3) {
        return false;
    }
    let op = arg_at(arena, h, 2);
    match atom_name(arena, &op) {
        Some(name) => deontic(&name),
        None => false,
    }
}

pub fn guard(arena: &ETermArena, c: &Clause, key: &Vec<u8>, record: &T) -> (out: Clause)
    requires
        arena_ok(arena),
        clause_valid(arena.nodes@, c),
        valid(arena.nodes@, record),
        record@ == spec::exclusion_record(key@),
    ensures
        clause_valid(arena.nodes@, &out),
        out@ == spec::guard(c@, key@),
{
    let mut body = copy_body(arena, &c.body);
    if deontic_edge(arena, &c.head) {
        let mut gs = Vec::new();
        gs.push(record.cp());
        proof {
            assert(models(gs@) == seq![record@]);
        }
        let item = Body::Naf(gs);
        let ghost before = body@;
        body.push(item);
        proof {
            assert_seqs_equal!(body_models(body@) == body_models(before).push(item@));
            assert forall|i: int| 0 <= i < body.len() implies #[trigger] body_valid(
                arena.nodes@,
                &body@[i],
            ) by {
                if i < before.len() {
                    assert(body@[i] == before[i]);
                }
            }
        }
    }
    Clause { head: c.head.cp(), body }
}

pub fn scoped_group(arena: &ETermArena, g: &Group, key: &Vec<u8>, record: &T, excl: bool) -> (out:
    Group)
    requires
        arena_ok(arena),
        group_valid(arena.nodes@, g),
        valid(arena.nodes@, record),
        record@ == spec::exclusion_record(key@),
    ensures
        group_valid(arena.nodes@, &out),
        out@ == spec::scoped_group(g@, key@, excl),
{
    let mut clauses = Vec::new();
    let mut i = 0usize;
    while i < g.clauses.len()
        invariant
            arena_ok(arena),
            group_valid(arena.nodes@, g),
            valid(arena.nodes@, record),
            record@ == spec::exclusion_record(key@),
            clauses_valid(arena.nodes@, clauses@),
            i <= g.clauses.len(),
            clauses.len() == i,
            clause_models(clauses@) == clause_models(g.clauses@).take(i as int).map_values(
                |c: DocClause| spec::guard(c, key@),
            ),
        decreases g.clauses.len() - i,
    {
        proof {
            assert(clause_valid(arena.nodes@, &g.clauses@[i as int]));
        }
        let next = guard(arena, &g.clauses[i], key, record);
        let ghost before = clauses@;
        clauses.push(next);
        proof {
            assert_seqs_equal!(clause_models(clauses@) == clause_models(g.clauses@).take(i + 1).map_values(
                |c: DocClause| spec::guard(c, key@)), j => {
                if j < before.len() {
                    assert(clauses@[j] == before[j]);
                    assert(clause_models(before)[j] == spec::guard(clause_models(g.clauses@)[j], key@));
                } else {
                    assert(j == i);
                    assert(clauses@[j] == next);
                    assert(clause_models(g.clauses@)[j] == g.clauses@[j]@);
                }
            });
            assert forall|j: int| 0 <= j < clauses.len() implies #[trigger] clause_valid(
                arena.nodes@,
                &clauses@[j],
            ) by {
                if j < before.len() {
                    assert(clauses@[j] == before[j]);
                }
            }
        }
        i += 1;
    }
    proof {
        assert(clause_models(g.clauses@).take(i as int) == clause_models(g.clauses@));
    }
    if excl && g.clauses.len() > 0 {
        proof {
            assert(clause_valid(arena.nodes@, &g.clauses@[0]));
        }
        let next = Clause { head: record.cp(), body: copy_body(arena, &g.clauses[0].body) };
        let ghost before = clauses@;
        clauses.push(next);
        proof {
            assert_seqs_equal!(clause_models(clauses@) == clause_models(before).push(next@));
            assert forall|j: int| 0 <= j < clauses.len() implies #[trigger] clause_valid(
                arena.nodes@,
                &clauses@[j],
            ) by {
                if j < before.len() {
                    assert(clauses@[j] == before[j]);
                }
            }
        }
    }
    Group { k: g.k, pairs: copy_map(arena, &g.pairs), clauses }
}

pub fn scoped_groups(
    arena: &ETermArena,
    gs: &Vec<Group>,
    key: &Vec<u8>,
    record: &T,
    excl: bool,
) -> (out: Vec<Group>)
    requires
        arena_ok(arena),
        groups_valid(arena.nodes@, gs@),
        valid(arena.nodes@, record),
        record@ == spec::exclusion_record(key@),
    ensures
        groups_valid(arena.nodes@, out@),
        group_models(out@) == group_models(gs@).map_values(
            |g: spec::Group| spec::scoped_group(g, key@, excl),
        ),
{
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < gs.len()
        invariant
            arena_ok(arena),
            groups_valid(arena.nodes@, gs@),
            groups_valid(arena.nodes@, out@),
            valid(arena.nodes@, record),
            record@ == spec::exclusion_record(key@),
            i <= gs.len(),
            out.len() == i,
            group_models(out@) == group_models(gs@).take(i as int).map_values(
                |g: spec::Group| spec::scoped_group(g, key@, excl),
            ),
        decreases gs.len() - i,
    {
        proof {
            assert(group_valid(arena.nodes@, &gs@[i as int]));
        }
        let next = scoped_group(arena, &gs[i], key, record, excl);
        let ghost before = out@;
        out.push(next);
        proof {
            assert_seqs_equal!(group_models(out@) == group_models(gs@).take(i + 1).map_values(
                |g: spec::Group| spec::scoped_group(g, key@, excl)), j => {
                if j < before.len() {
                    assert(out@[j] == before[j]);
                    assert(group_models(before)[j] == spec::scoped_group(group_models(gs@)[j], key@, excl));
                } else {
                    assert(j == i);
                    assert(out@[j] == next);
                    assert(group_models(gs@)[j] == gs@[j]@);
                }
            });
            assert forall|j: int| 0 <= j < out.len() implies #[trigger] group_valid(
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
    out
}

pub fn scope_pass(
    arena: &mut ETermArena,
    tab: &Option<EEnv>,
    roots: &Vec<ERoot>,
    gs: Vec<Group>,
) -> (out: Result<Vec<Group>, T>)
    requires
        arena_ok(old(arena)),
        roots_valid(old(arena).nodes@, roots@),
        groups_valid(old(arena).nodes@, gs@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        groups_result_valid(final(arena).nodes@, &out),
        groups_result(out) == spec::scope_pass(
            tab_view(tab),
            root_models(roots@),
            group_models(gs@),
        ),
{
    let ghost start = arena.nodes@;
    let env = match tab {
        Some(env) => env,
        None => return Ok(gs),
    };
    if env.tab.scopes.len() == 0 || env.tab.exclusions.len() == 0 {
        return Ok(gs);
    }
    let key = &env.tab.scopes[0].1;
    let class = if roots.len() == 1 {
        proof {
            assert(root_valid(arena.nodes@, &roots@[0]));
        }
        match &roots[0] {
            ERoot::Rule(a, c) => rule_scope(arena, &env.tab, a, c),
            _ => root_scope(arena, &env.tab, roots),
        }
    } else {
        root_scope(arena, &env.tab, roots)
    };
    let ghost classified = arena.nodes@;
    proof {
        groups_prefix(start, classified, gs@);
    }
    let excl = match class {
        Scope::Reject(why) => {
            let sym = why.symbol();
            let e = crate::m7_strength::shape(arena, &Sym::ScopeShape, &sym);
            proof {
                crate::k2_load::prefix_chain(start, classified, arena.nodes@);
            }
            return Err(e);
        },
        Scope::Exclusion => true,
        Scope::Plain => false,
    };
    let record = exclusion_record(arena, key);
    proof {
        crate::k2_load::prefix_chain(start, classified, arena.nodes@);
        groups_prefix(classified, arena.nodes@, gs@);
    }
    Ok(scoped_groups(arena, &gs, key, &record, excl))
}

} // verus!
