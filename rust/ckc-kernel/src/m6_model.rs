#[cfg(verus_keep_ghost)]
use crate::k2_term::arena_ok;
use crate::k2_term::{ENode, ETermArena};
use crate::m6_term::{T, copy};
#[cfg(verus_keep_ghost)]
use crate::m6_term::{models, prefix, prefix_all, valid, valid_all};
use ckc_spec::emit as spec;
use ckc_spec::v1text;
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub enum W {
    Root,
    Antecedent,
    Consequent,
}

impl View for W {
    type V = spec::Where;

    open spec fn view(&self) -> Self::V {
        match self {
            W::Root => spec::Where::Root,
            W::Antecedent => spec::Where::Antecedent,
            W::Consequent => spec::Where::Consequent,
        }
    }
}

pub enum E {
    Top,
    Op,
    Naf,
}

impl View for E {
    type V = spec::Encl;

    open spec fn view(&self) -> Self::V {
        match self {
            E::Top => spec::Encl::Top,
            E::Op => spec::Encl::Op,
            E::Naf => spec::Encl::Naf,
        }
    }
}

impl E {
    pub fn cp(&self) -> (out: E)
        ensures
            out@ == self@,
    {
        match self {
            E::Top => E::Top,
            E::Op => E::Op,
            E::Naf => E::Naf,
        }
    }
}

pub struct Binding {
    pub key: T,
    pub value: T,
}

impl View for Binding {
    type V = (ckc_spec::term::Term, ckc_spec::term::Term);

    open spec fn view(&self) -> Self::V {
        (self.key@, self.value@)
    }
}

pub open spec fn binding_valid(nodes: Seq<ENode>, b: &Binding) -> bool {
    valid(nodes, &b.key) && valid(nodes, &b.value)
}

pub open spec fn map_valid(nodes: Seq<ENode>, bs: Seq<Binding>) -> bool {
    forall|i: int| 0 <= i < bs.len() ==> #[trigger] binding_valid(nodes, &bs[i])
}

pub open spec fn map_model(bs: Seq<Binding>) -> Seq<(ckc_spec::term::Term, ckc_spec::term::Term)> {
    bs.map_values(|b: Binding| b@)
}

pub proof fn map_prefix(before: Seq<ENode>, after: Seq<ENode>, bs: Seq<Binding>)
    requires
        before.is_prefix_of(after),
        map_valid(before, bs),
    ensures
        map_valid(after, bs),
{
    assert forall|i: int| 0 <= i < bs.len() implies #[trigger] binding_valid(after, &bs[i]) by {
        assert(binding_valid(before, &bs[i]));
        prefix(before, after, &bs[i].key);
        prefix(before, after, &bs[i].value);
    }
}

pub fn copy_map(arena: &ETermArena, bs: &Vec<Binding>) -> (out: Vec<Binding>)
    requires
        arena_ok(arena),
        map_valid(arena.nodes@, bs@),
    ensures
        map_valid(arena.nodes@, out@),
        map_model(out@) == map_model(bs@),
{
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < bs.len()
        invariant
            arena_ok(arena),
            map_valid(arena.nodes@, bs@),
            i <= bs.len(),
            map_valid(arena.nodes@, out@),
            out.len() == i,
            map_model(out@) == map_model(bs@).take(i as int),
        decreases bs.len() - i,
    {
        proof {
            assert(binding_valid(arena.nodes@, &bs@[i as int]));
        }
        let next = Binding { key: bs[i].key.cp(), value: bs[i].value.cp() };
        proof {
            assert(next@ == bs@[i as int]@);
        }
        let ghost before = out@;
        out.push(next);
        proof {
            assert forall|j: int| 0 <= j < out.len() implies #[trigger] binding_valid(
                arena.nodes@,
                &out@[j],
            ) by {
                if j < before.len() {
                    assert(out@[j] == before[j]);
                }
            }
            assert_seqs_equal!(map_model(out@) == map_model(bs@).take(i as int+1), j => {
                if j < before.len() {
                    assert(out@[j] == before[j]);
                    assert(map_model(before)[j] == map_model(bs@)[j]);
                } else {
                    assert(j == i);
                    assert(out@[j] == next);
                    assert(map_model(out@)[j] == next@);
                    assert(next@ == bs@[i as int]@);
                    assert(map_model(bs@)[j] == bs@[i as int]@);
                }
            });
        }
        i += 1;
    }
    out
}

pub struct I {
    pub kind: Kind,
    pub model: Ghost<spec::Item>,
}

pub enum Kind {
    Anch(T, T),
    Op { b: usize, outer: T, inner: T, op: Vec<u8> },
    Naf { dom: Vec<T>, payload: Vec<I> },
}

impl View for I {
    type V = spec::Item;

    open spec fn view(&self) -> Self::V {
        self.model@
    }
}

pub open spec fn item_models(items: Seq<I>) -> Seq<spec::Item> {
    items.map_values(|i: I| i@)
}

pub open spec fn item_valid(nodes: Seq<ENode>, it: &I) -> bool
    decreases it, 0int,
{
    match &it.kind {
        Kind::Anch(c, t) => it@ == spec::Item::Anch(c@, t@) && valid(nodes, c) && valid(nodes, t),
        Kind::Op { b, outer, inner, op } => it@ == spec::Item::Op(*b as nat, outer@, inner@, op@)
            && valid(nodes, outer) && valid(nodes, inner),
        Kind::Naf { dom, payload } => it@ == spec::Item::Naf(models(dom@), item_models(payload@))
            && valid_all(nodes, dom@) && items_valid(nodes, payload@),
    }
}

pub open spec fn items_valid(nodes: Seq<ENode>, items: Seq<I>) -> bool
    decreases items, 1int,
{
    if items.len() == 0 {
        true
    } else {
        item_valid(nodes, &items[0]) && items_valid(nodes, items.drop_first())
    }
}

pub proof fn items_at(nodes: Seq<ENode>, items: Seq<I>, i: int)
    requires
        items_valid(nodes, items),
        0 <= i < items.len(),
    ensures
        item_valid(nodes, &items[i]),
    decreases i,
{
    reveal_with_fuel(items_valid, 1);
    if i > 0 {
        assert(items.drop_first()[i - 1] == items[i]);
        items_at(nodes, items.drop_first(), i - 1);
    }
}

pub proof fn items_concat(nodes: Seq<ENode>, a: Seq<I>, b: Seq<I>)
    requires
        items_valid(nodes, a),
        items_valid(nodes, b),
    ensures
        items_valid(nodes, a + b),
        item_models(a + b) == item_models(a) + item_models(b),
    decreases a.len(),
{
    if a.len() > 0 {
        assert((a + b).drop_first() == a.drop_first() + b);
        reveal_with_fuel(items_valid, 1);
        items_concat(nodes, a.drop_first(), b);
    }
    reveal_with_fuel(items_valid, 1);
    assert_seqs_equal!(item_models(a+b) == item_models(a)+item_models(b));
}

pub proof fn item_prefix(before: Seq<ENode>, after: Seq<ENode>, it: &I)
    requires
        before.is_prefix_of(after),
        item_valid(before, it),
    ensures
        item_valid(after, it),
    decreases it, 0int,
{
    reveal(item_valid);
    match &it.kind {
        Kind::Anch(c, t) => {
            prefix(before, after, c);
            prefix(before, after, t);
        },
        Kind::Op { outer, inner, .. } => {
            prefix(before, after, outer);
            prefix(before, after, inner);
        },
        Kind::Naf { dom, payload } => {
            prefix_all(before, after, dom@);
            items_prefix(before, after, payload@);
        },
    }
}

pub proof fn items_prefix(before: Seq<ENode>, after: Seq<ENode>, items: Seq<I>)
    requires
        before.is_prefix_of(after),
        items_valid(before, items),
    ensures
        items_valid(after, items),
    decreases items, 1int,
{
    reveal_with_fuel(items_valid, 1);
    if items.len() > 0 {
        item_prefix(before, after, &items[0]);
        items_prefix(before, after, items.drop_first());
    }
}

pub fn anch(arena: &ETermArena, ctx: &T, inner: &T) -> (out: I)
    requires
        arena_ok(arena),
        valid(arena.nodes@, ctx),
        valid(arena.nodes@, inner),
    ensures
        item_valid(arena.nodes@, &out),
        out@ == spec::Item::Anch(ctx@, inner@),
{
    I { kind: Kind::Anch(ctx.cp(), inner.cp()), model: Ghost(spec::Item::Anch(ctx@, inner@)) }
}

pub fn operator(arena: &ETermArena, b: usize, outer: &T, inner: &T, op: &Vec<u8>) -> (out: I)
    requires
        arena_ok(arena),
        valid(arena.nodes@, outer),
        valid(arena.nodes@, inner),
    ensures
        item_valid(arena.nodes@, &out),
        out@ == spec::Item::Op(b as nat, outer@, inner@, op@),
{
    I {
        kind: Kind::Op { b, outer: outer.cp(), inner: inner.cp(), op: op.clone() },
        model: Ghost(spec::Item::Op(b as nat, outer@, inner@, op@)),
    }
}

pub fn naf(arena: &ETermArena, dom: Vec<T>, payload: Vec<I>) -> (out: I)
    requires
        arena_ok(arena),
        valid_all(arena.nodes@, dom@),
        items_valid(arena.nodes@, payload@),
    ensures
        item_valid(arena.nodes@, &out),
        out@ == spec::Item::Naf(models(dom@), item_models(payload@)),
{
    let ghost model = spec::Item::Naf(models(dom@), item_models(payload@));
    I { kind: Kind::Naf { dom, payload }, model: Ghost(model) }
}

pub fn copy_item(arena: &ETermArena, it: &I) -> (out: I)
    requires
        arena_ok(arena),
        item_valid(arena.nodes@, it),
    ensures
        item_valid(arena.nodes@, &out),
        out@ == it@,
    decreases it, 0int,
{
    proof {
        reveal(item_valid);
    }
    match &it.kind {
        Kind::Anch(ctx, inner) => anch(arena, ctx, inner),
        Kind::Op { b, outer, inner, op } => operator(arena, *b, outer, inner, op),
        Kind::Naf { dom, payload } => naf(arena, copy(dom), copy_items(arena, payload)),
    }
}

pub fn copy_items(arena: &ETermArena, items: &Vec<I>) -> (out: Vec<I>)
    requires
        arena_ok(arena),
        items_valid(arena.nodes@, items@),
    ensures
        items_valid(arena.nodes@, out@),
        item_models(out@) == item_models(items@),
    decreases items@, 1int,
{
    let mut out = Vec::new();
    let mut i = 0usize;
    proof {
        reveal_with_fuel(items_valid, 1);
    }
    while i < items.len()
        invariant
            arena_ok(arena),
            items_valid(arena.nodes@, items@),
            i <= items.len(),
            items_valid(arena.nodes@, out@),
            item_models(out@) == item_models(items@).take(i as int),
        decreases items.len() - i,
    {
        proof {
            items_at(arena.nodes@, items@, i as int);
        }
        let next = copy_item(arena, &items[i]);
        let ghost before = out@;
        proof {
            reveal_with_fuel(items_valid, 2);
            items_concat(arena.nodes@, out@, seq![next]);
        }
        out.push(next);
        proof {
            assert(out@ == before + seq![next]);
            assert_seqs_equal!(item_models(out@) == item_models(items@).take(i as int+1));
        }
        i += 1;
    }
    out
}

pub fn has_anch(arena: &ETermArena, items: &Vec<I>) -> (out: bool)
    requires
        arena_ok(arena),
        items_valid(arena.nodes@, items@),
    ensures
        out == spec::has_anch(item_models(items@)),
{
    let mut i = 0usize;
    while i < items.len()
        invariant
            arena_ok(arena),
            items_valid(arena.nodes@, items@),
            i <= items.len(),
            forall|j: int| 0 <= j < i ==> !(item_models(items@)[j] is Anch),
        decreases items.len() - i,
    {
        proof {
            items_at(arena.nodes@, items@, i as int);
            reveal(item_valid);
        }
        if let Kind::Anch(_, _) = &items[i].kind {
            proof {
                assert(item_models(items@)[i as int] is Anch);
            }
            return true;
        }
        i += 1;
    }
    false
}

pub struct Flat {
    pub items: Vec<I>,
    pub n: usize,
}

impl View for Flat {
    type V = spec::Flat;

    open spec fn view(&self) -> Self::V {
        spec::Flat { items: item_models(self.items@), n: self.n as nat }
    }
}

pub open spec fn flat_valid(nodes: Seq<ENode>, f: &Flat) -> bool {
    items_valid(nodes, f.items@)
}

pub open spec fn flat_result(r: Result<Flat, T>) -> Result<spec::Flat, ckc_spec::term::Term> {
    match r {
        Ok(f) => Ok(f@),
        Err(t) => Err(t@),
    }
}

pub enum Body {
    Pos(T),
    Naf(Vec<T>),
}

impl View for Body {
    type V = v1text::BodyItem;

    open spec fn view(&self) -> Self::V {
        match self {
            Body::Pos(t) => v1text::BodyItem::Pos(t@),
            Body::Naf(ts) => v1text::BodyItem::Naf(models(ts@)),
        }
    }
}

pub open spec fn body_valid(nodes: Seq<ENode>, b: &Body) -> bool {
    match b {
        Body::Pos(t) => valid(nodes, t),
        Body::Naf(ts) => valid_all(nodes, ts@),
    }
}

pub open spec fn bodies_valid(nodes: Seq<ENode>, bs: Seq<Body>) -> bool {
    forall|i: int| 0 <= i < bs.len() ==> #[trigger] body_valid(nodes, &bs[i])
}

pub open spec fn body_models(bs: Seq<Body>) -> Seq<v1text::BodyItem> {
    bs.map_values(|b: Body| b@)
}

pub proof fn bodies_prefix(before: Seq<ENode>, after: Seq<ENode>, bs: Seq<Body>)
    requires
        before.is_prefix_of(after),
        bodies_valid(before, bs),
    ensures
        bodies_valid(after, bs),
{
    assert forall|i: int| 0 <= i < bs.len() implies #[trigger] body_valid(after, &bs[i]) by {
        assert(body_valid(before, &bs[i]));
        match &bs[i] {
            Body::Pos(t) => prefix(before, after, t),
            Body::Naf(ts) => prefix_all(before, after, ts@),
        }
    }
}

pub fn copy_body(arena: &ETermArena, bs: &Vec<Body>) -> (out: Vec<Body>)
    requires
        arena_ok(arena),
        bodies_valid(arena.nodes@, bs@),
    ensures
        bodies_valid(arena.nodes@, out@),
        body_models(out@) == body_models(bs@),
{
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < bs.len()
        invariant
            arena_ok(arena),
            bodies_valid(arena.nodes@, bs@),
            i <= bs.len(),
            bodies_valid(arena.nodes@, out@),
            out.len() == i,
            body_models(out@) == body_models(bs@).take(i as int),
        decreases bs.len() - i,
    {
        proof {
            assert(body_valid(arena.nodes@, &bs@[i as int]));
        }
        let next = match &bs[i] {
            Body::Pos(t) => Body::Pos(t.cp()),
            Body::Naf(ts) => Body::Naf(copy(ts)),
        };
        proof {
            assert(next@ == bs@[i as int]@);
            assert(body_valid(arena.nodes@, &next));
        }
        let ghost before = out@;
        out.push(next);
        proof {
            assert forall|j: int| 0 <= j < out.len() implies #[trigger] body_valid(
                arena.nodes@,
                &out@[j],
            ) by {
                if j < before.len() {
                    assert(out@[j] == before[j]);
                }
            }
            assert_seqs_equal!(body_models(out@) == body_models(bs@).take(i as int+1), j => {
                if j < before.len() {
                    assert(out@[j] == before[j]);
                    assert(body_models(before)[j] == body_models(bs@)[j]);
                } else {
                    assert(j == i);
                    assert(out@[j] == next);
                    assert(body_models(out@)[j] == next@);
                    assert(next@ == bs@[i as int]@);
                    assert(body_models(bs@)[j] == bs@[i as int]@);
                }
            });
        }
        i += 1;
    }
    out
}

pub struct Clause {
    pub head: T,
    pub body: Vec<Body>,
}

impl View for Clause {
    type V = v1text::DocClause;

    open spec fn view(&self) -> Self::V {
        v1text::DocClause { head: self.head@, body: body_models(self.body@) }
    }
}

pub open spec fn clause_valid(nodes: Seq<ENode>, c: &Clause) -> bool {
    valid(nodes, &c.head) && bodies_valid(nodes, c.body@)
}

pub open spec fn clauses_valid(nodes: Seq<ENode>, cs: Seq<Clause>) -> bool {
    forall|i: int| 0 <= i < cs.len() ==> #[trigger] clause_valid(nodes, &cs[i])
}

pub open spec fn clause_models(cs: Seq<Clause>) -> Seq<v1text::DocClause> {
    cs.map_values(|c: Clause| c@)
}

pub proof fn clauses_prefix(before: Seq<ENode>, after: Seq<ENode>, cs: Seq<Clause>)
    requires
        before.is_prefix_of(after),
        clauses_valid(before, cs),
    ensures
        clauses_valid(after, cs),
{
    assert forall|i: int| 0 <= i < cs.len() implies #[trigger] clause_valid(after, &cs[i]) by {
        assert(clause_valid(before, &cs[i]));
        prefix(before, after, &cs[i].head);
        bodies_prefix(before, after, cs[i].body@);
    }
}

pub struct Group {
    pub k: usize,
    pub pairs: Vec<Binding>,
    pub clauses: Vec<Clause>,
}

impl View for Group {
    type V = spec::Group;

    open spec fn view(&self) -> Self::V {
        spec::Group {
            k: self.k as nat,
            pairs: map_model(self.pairs@),
            clauses: clause_models(self.clauses@),
        }
    }
}

pub open spec fn group_valid(nodes: Seq<ENode>, g: &Group) -> bool {
    map_valid(nodes, g.pairs@) && clauses_valid(nodes, g.clauses@)
}

pub open spec fn groups_valid(nodes: Seq<ENode>, gs: Seq<Group>) -> bool {
    forall|i: int| 0 <= i < gs.len() ==> #[trigger] group_valid(nodes, &gs[i])
}

pub open spec fn group_models(gs: Seq<Group>) -> Seq<spec::Group> {
    gs.map_values(|g: Group| g@)
}

pub proof fn groups_prefix(before: Seq<ENode>, after: Seq<ENode>, gs: Seq<Group>)
    requires
        before.is_prefix_of(after),
        groups_valid(before, gs),
    ensures
        groups_valid(after, gs),
{
    assert forall|i: int| 0 <= i < gs.len() implies #[trigger] group_valid(after, &gs[i]) by {
        assert(group_valid(before, &gs[i]));
        map_prefix(before, after, gs[i].pairs@);
        clauses_prefix(before, after, gs[i].clauses@);
    }
}

pub struct Projected {
    pub s: usize,
    pub groups: Vec<Group>,
}

impl View for Projected {
    type V = spec::Projected;

    open spec fn view(&self) -> Self::V {
        spec::Projected { s: self.s as nat, groups: group_models(self.groups@) }
    }
}

pub open spec fn projected_valid(nodes: Seq<ENode>, p: &Projected) -> bool {
    groups_valid(nodes, p.groups@)
}

pub open spec fn projections_valid(nodes: Seq<ENode>, ps: Seq<Projected>) -> bool {
    forall|i: int| 0 <= i < ps.len() ==> #[trigger] projected_valid(nodes, &ps[i])
}

pub open spec fn project_models(ps: Seq<Projected>) -> Seq<spec::Projected> {
    ps.map_values(|p: Projected| p@)
}

pub proof fn projections_prefix(before: Seq<ENode>, after: Seq<ENode>, ps: Seq<Projected>)
    requires
        before.is_prefix_of(after),
        projections_valid(before, ps),
    ensures
        projections_valid(after, ps),
{
    assert forall|i: int| 0 <= i < ps.len() implies #[trigger] projected_valid(after, &ps[i]) by {
        assert(projected_valid(before, &ps[i]));
        groups_prefix(before, after, ps[i].groups@);
    }
}

} // verus!
