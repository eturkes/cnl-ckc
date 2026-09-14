#[cfg(verus_keep_ghost)]
use crate::k2_term::arena_ok;
use crate::k2_term::{ENode, ETermArena};
use crate::m6_symbols::Sym;
use crate::m6_term::*;
use ckc_spec::emit as spec;
use ckc_spec::term::Term;
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub struct BoxParts {
    pub dom: Vec<T>,
    pub conds: T,
}

impl View for BoxParts {
    type V = (Seq<Term>, Term);

    open spec fn view(&self) -> Self::V {
        (models(self.dom@), self.conds@)
    }
}

pub open spec fn box_valid(nodes: Seq<ENode>, b: &BoxParts) -> bool {
    valid_all(nodes, b.dom@) && valid(nodes, &b.conds)
}

pub open spec fn box_view(b: Option<BoxParts>) -> Option<(Seq<Term>, Term)> {
    match b {
        Some(b) => Some(b@),
        None => None,
    }
}

pub fn box_parts(arena: &ETermArena, t: &T) -> (out: Option<BoxParts>)
    requires
        arena_ok(arena),
        valid(arena.nodes@, t),
    ensures
        out matches Some(b) ==> box_valid(arena.nodes@, &b),
        box_view(out) == spec::box_parts(t@),
{
    if !is_comp(arena, t, &Sym::Drs, 2) {
        return None;
    }
    let a = args(arena, t);
    let dom = match items(arena, &a[0]) {
        Some(dom) => dom,
        None => return None,
    };
    if items(arena, &a[1]).is_none() {
        return None;
    }
    Some(BoxParts { dom, conds: a[1].cp() })
}

pub open spec fn int_value(t: Term) -> int {
    match t {
        Term::Int(n) => n,
        _ => 0,
    }
}

pub open spec fn int_refs(ts: Seq<T>) -> bool {
    forall|i: int| 0 <= i < ts.len() ==> (#[trigger] ts[i])@ is Int
}

pub open spec fn ints(ts: Seq<T>) -> Seq<int> {
    ts.map_values(|t: T| int_value(t@))
}

pub open spec fn option_int(t: Option<T>) -> Option<int> {
    match t {
        Some(t) => Some(int_value(t@)),
        None => None,
    }
}

pub fn anchor(arena: &ETermArena, t: &T) -> (out: Option<T>)
    requires
        arena_ok(arena),
        valid(arena.nodes@, t),
    ensures
        out matches Some(s) ==> valid(arena.nodes@, &s) && s@ is Int,
        option_int(out) == spec::anchor_s(t@),
{
    if !is_comp(arena, t, &Sym::Minus, 2) {
        return None;
    }
    let a = args(arena, t);
    if !is_comp(arena, &a[1], &Sym::Slash, 2) {
        return None;
    }
    let at = args(arena, &a[1]);
    if !is_int(arena, &at[0]) || !is_int(arena, &at[1]) {
        return None;
    }
    Some(at[0].cp())
}

pub fn modal(name: &Vec<u8>) -> (out: bool)
    ensures
        out == spec::is_modal(name@),
{
    has_name(name, &Sym::Should) || has_name(name, &Sym::Must) || has_name(name, &Sym::Can)
        || has_name(name, &Sym::May)
}

pub fn op(name: &Vec<u8>) -> (out: bool)
    ensures
        out == spec::is_op(name@),
{
    modal(name) || has_name(name, &Sym::Minus)
}

pub fn carrier(name: &Vec<u8>, arity: usize) -> (out: bool)
    ensures
        out == spec::is_carrier(name@, arity as nat),
{
    (has_name(name, &Sym::Drs) && arity == 2) || (has_name(name, &Sym::Question) && arity == 1) || (
    op(name) && arity == 1) || (has_name(name, &Sym::Naf) && arity == 1) || (has_name(
        name,
        &Sym::Implies,
    ) && arity == 2) || (has_name(name, &Sym::V) && arity == 2) || (has_name(name, &Sym::Cons)
        && arity == 2)
}

pub proof fn size_in(ts: Seq<Term>, i: int)
    requires
        0 <= i < ts.len(),
    ensures
        crate::k2_engine::term_size(ts[i]) <= crate::k2_engine::terms_size(ts),
    decreases i,
{
    reveal(crate::k2_engine::terms_size);
    if i > 0 {
        assert(ts.drop_first()[i - 1] == ts[i]);
        size_in(ts.drop_first(), i - 1);
    }
}

pub proof fn child_size(t: Term, i: int)
    requires
        0 <= i < ckc_spec::engine::args_of(t).len(),
    ensures
        crate::k2_engine::term_size(ckc_spec::engine::args_of(t)[i]) < crate::k2_engine::term_size(
            t,
        ),
{
    size_in(ckc_spec::engine::args_of(t), i);
    reveal(crate::k2_engine::term_size);
}

pub proof fn sizes_concat(a: Seq<Term>, b: Seq<Term>)
    ensures
        crate::k2_engine::terms_size(a + b) == crate::k2_engine::terms_size(a)
            + crate::k2_engine::terms_size(b),
    decreases a.len(),
{
    if a.len() > 0 {
        assert((a + b).drop_first() == a.drop_first() + b);
        sizes_concat(a.drop_first(), b);
    }
    reveal_with_fuel(crate::k2_engine::terms_size, 2);
}

pub proof fn anchors_concat(a: Seq<Term>, b: Seq<Term>)
    ensures
        spec::anchors_all(a + b) == spec::anchors_all(a) + spec::anchors_all(b),
    decreases a.len(),
{
    if a.len() > 0 {
        assert((a + b).drop_first() == a.drop_first() + b);
        anchors_concat(a.drop_first(), b);
    }
    reveal_with_fuel(spec::anchors_all, 2);
}

pub fn anchors(arena: &ETermArena, t: &T) -> (out: Vec<T>)
    requires
        arena_ok(arena),
        valid(arena.nodes@, t),
    ensures
        valid_all(arena.nodes@, out@),
        int_refs(out@),
        ints(out@) == spec::anchors(t@),
{
    let mut todo = Vec::new();
    todo.push(t.cp());
    let mut out = Vec::new();
    proof {
        reveal_with_fuel(spec::anchors_all, 2);
    }
    while todo.len() > 0
        invariant
            arena_ok(arena),
            valid(arena.nodes@, t),
            valid_all(arena.nodes@, todo@),
            valid_all(arena.nodes@, out@),
            int_refs(out@),
            ints(out@) + spec::anchors_all(models(todo@)) == spec::anchors(t@),
        decreases crate::k2_engine::terms_size(models(todo@)),
    {
        let ghost before = todo@;
        let current = todo.remove(0);
        proof {
            assert(todo@ == before.drop_first());
            assert(models(before).drop_first() == models(todo@));
            reveal_with_fuel(spec::anchors_all, 1);
            reveal_with_fuel(crate::k2_engine::terms_size, 1);
        }
        match anchor(arena, &current) {
            Some(s) => {
                let a = args(arena, &current);
                let mut children = Vec::new();
                children.push(a[0].cp());
                let ghost previous = out@;
                proof {
                    extend(arena.nodes@, out@, s);
                }
                out.push(s);
                proof {
                    assert_seqs_equal!(ints(out@) == ints(previous).push(int_value(s@)));
                    assert forall|j: int| 0 <= j < out.len() implies (
                    #[trigger] out@[j])@ is Int by {
                        if j < previous.len() {
                            assert(out@[j] == previous[j]);
                        }
                    }
                    anchors_concat(models(children@), models(todo@));
                    sizes_concat(models(children@), models(todo@));
                    child_size(current@, 0);
                    reveal_with_fuel(spec::anchors, 1);
                    reveal_with_fuel(spec::anchors_all, 2);
                    reveal_with_fuel(crate::k2_engine::terms_size, 2);
                }
                todo = concat(&children, &todo);
            },
            None => {
                match parts(arena, &current) {
                    Some((name, children)) => {
                        if carrier(&name, children.len()) {
                            proof {
                                anchors_concat(models(children@), models(todo@));
                                sizes_concat(models(children@), models(todo@));
                                reveal_with_fuel(spec::anchors, 1);
                                reveal(crate::k2_engine::term_size);
                            }
                            todo = concat(&children, &todo);
                        } else {
                            proof {
                                reveal_with_fuel(spec::anchors, 1);
                                reveal(crate::k2_engine::term_size);
                            }
                        }
                    },
                    None => {
                        proof {
                            reveal_with_fuel(spec::anchors, 1);
                            reveal(crate::k2_engine::term_size);
                        }
                    },
                }
            },
        }
    }
    proof {
        reveal(spec::anchors_all);
    }
    out
}

pub fn inner_sentence(arena: &ETermArena, t: &T) -> (out: Option<T>)
    requires
        arena_ok(arena),
        valid(arena.nodes@, t),
    ensures
        out matches Some(s) ==> valid(arena.nodes@, &s) && s@ is Int,
        option_int(out) == spec::inner_sentence(t@),
{
    let ss = anchors(arena, t);
    if ss.len() == 0 {
        return None;
    }
    let mut i = 1usize;
    while i < ss.len()
        invariant
            arena_ok(arena),
            valid_all(arena.nodes@, ss@),
            int_refs(ss@),
            ints(ss@) == spec::anchors(t@),
            1 <= i <= ss.len(),
            forall|j: int| 0 <= j < i ==> #[trigger] ints(ss@)[j] == ints(ss@)[0],
        decreases ss.len() - i,
    {
        if !crate::m6_term::equal(arena, &ss[0], &ss[i]) {
            proof {
                assert(ints(ss@)[i as int] != ints(ss@)[0]);
                assert(!(forall|j: int| 0 <= j < ints(ss@).len() ==> #[trigger] ints(ss@)[j] == ints(ss@)[0]));
            }
            return None;
        }
        i += 1;
    }
    Some(ss[0].cp())
}

pub enum ERoot {
    Anchored(T),
    Rule(T, T),
    Boxed(T),
}

impl View for ERoot {
    type V = spec::Root;

    open spec fn view(&self) -> spec::Root {
        match self {
            ERoot::Anchored(t) => spec::Root::Anchored(t@),
            ERoot::Rule(a, b) => spec::Root::Rule(a@, b@),
            ERoot::Boxed(t) => spec::Root::Boxed(t@),
        }
    }
}

pub open spec fn root_valid(nodes: Seq<ENode>, r: &ERoot) -> bool {
    match r {
        ERoot::Anchored(t) | ERoot::Boxed(t) => valid(nodes, t),
        ERoot::Rule(a, b) => valid(nodes, a) && valid(nodes, b),
    }
}

pub open spec fn roots_valid(nodes: Seq<ENode>, rs: Seq<ERoot>) -> bool {
    forall|i: int| 0 <= i < rs.len() ==> #[trigger] root_valid(nodes, &rs[i])
}

pub open spec fn root_models(rs: Seq<ERoot>) -> Seq<spec::Root> {
    rs.map_values(|r: ERoot| r@)
}

pub fn clone_root(arena: &ETermArena, r: &ERoot) -> (out: ERoot)
    requires
        arena_ok(arena),
        root_valid(arena.nodes@, r),
    ensures
        root_valid(arena.nodes@, &out),
        out@ == r@,
{
    match r {
        ERoot::Anchored(t) => ERoot::Anchored(t.cp()),
        ERoot::Rule(a, b) => ERoot::Rule(a.cp(), b.cp()),
        ERoot::Boxed(t) => ERoot::Boxed(t.cp()),
    }
}

pub struct ETag {
    pub sentence: T,
    pub root: ERoot,
}

impl View for ETag {
    type V = (int, spec::Root);

    open spec fn view(&self) -> Self::V {
        (int_value(self.sentence@), self.root@)
    }
}

pub open spec fn tag_valid(nodes: Seq<ENode>, t: &ETag) -> bool {
    valid(nodes, &t.sentence) && t.sentence@ is Int && root_valid(nodes, &t.root)
}

pub open spec fn tagged_valid(nodes: Seq<ENode>, ts: Seq<ETag>) -> bool {
    forall|i: int| 0 <= i < ts.len() ==> #[trigger] tag_valid(nodes, &ts[i])
}

pub open spec fn tag_models(ts: Seq<ETag>) -> Seq<(int, spec::Root)> {
    ts.map_values(|t: ETag| t@)
}

pub open spec fn tag_view(t: Option<ETag>) -> Option<(int, spec::Root)> {
    match t {
        Some(t) => Some(t@),
        None => None,
    }
}

pub fn tag(arena: &ETermArena, t: &T) -> (out: Option<ETag>)
    requires
        arena_ok(arena),
        valid(arena.nodes@, t),
    ensures
        out matches Some(t) ==> tag_valid(arena.nodes@, &t),
        tag_view(out) == spec::tag(t@),
{
    if let Some(sentence) = anchor(arena, t) {
        let a = args(arena, t);
        return Some(ETag { sentence, root: ERoot::Anchored(a[0].cp()) });
    }
    match parts(arena, t) {
        Some((name, a)) => {
            if has_name(&name, &Sym::Implies) && a.len() == 2 {
                match inner_sentence(arena, t) {
                    Some(sentence) => Some(
                        ETag { sentence, root: ERoot::Rule(a[0].cp(), a[1].cp()) },
                    ),
                    None => None,
                }
            } else if op(&name) && a.len() == 1 {
                match inner_sentence(arena, t) {
                    Some(sentence) => Some(ETag { sentence, root: ERoot::Boxed(t.cp()) }),
                    None => None,
                }
            } else {
                None
            }
        },
        None => None,
    }
}

pub open spec fn tagged_view(t: Option<Vec<ETag>>) -> Option<Seq<(int, spec::Root)>> {
    match t {
        Some(t) => Some(tag_models(t@)),
        None => None,
    }
}

pub open spec fn tag_prefix(a: Seq<(int, spec::Root)>, b: Option<Seq<(int, spec::Root)>>) -> Option<
    Seq<(int, spec::Root)>,
> {
    match b {
        Some(b) => Some(a + b),
        None => None,
    }
}

pub fn tags(arena: &ETermArena, ts: &Vec<T>) -> (out: Option<Vec<ETag>>)
    requires
        arena_ok(arena),
        valid_all(arena.nodes@, ts@),
    ensures
        out matches Some(ts) ==> tagged_valid(arena.nodes@, ts@),
        tagged_view(out) == spec::tags(models(ts@)),
{
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < ts.len()
        invariant
            arena_ok(arena),
            valid_all(arena.nodes@, ts@),
            i <= ts.len(),
            tagged_valid(arena.nodes@, out@),
            spec::tags(models(ts@)) == tag_prefix(
                tag_models(out@),
                spec::tags(models(ts@).skip(i as int)),
            ),
        decreases ts.len() - i,
    {
        proof {
            assert(models(ts@).skip(i as int).drop_first() == models(ts@).skip(i as int + 1));
            reveal_with_fuel(spec::tags, 1);
        }
        let next = match tag(arena, &ts[i]) {
            Some(next) => next,
            None => return None,
        };
        let ghost before = out@;
        out.push(next);
        proof {
            assert_seqs_equal!(tag_models(out@) == tag_models(before).push(next@));
            assert forall|j: int| 0 <= j < out.len() implies #[trigger] tag_valid(
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
        reveal(spec::tags);
    }
    Some(out)
}

pub proof fn root_prefix(before: Seq<ENode>, after: Seq<ENode>, r: &ERoot)
    requires
        before.is_prefix_of(after),
        root_valid(before, r),
    ensures
        root_valid(after, r),
{
    match r {
        ERoot::Anchored(t) | ERoot::Boxed(t) => prefix(before, after, t),
        ERoot::Rule(a, b) => {
            prefix(before, after, a);
            prefix(before, after, b);
        },
    }
}

pub proof fn roots_prefix(before: Seq<ENode>, after: Seq<ENode>, rs: Seq<ERoot>)
    requires
        before.is_prefix_of(after),
        roots_valid(before, rs),
    ensures
        roots_valid(after, rs),
{
    assert forall|i: int| 0 <= i < rs.len() implies #[trigger] root_valid(after, &rs[i]) by {
        root_prefix(before, after, &rs[i]);
    }
}

pub proof fn tags_prefix(before: Seq<ENode>, after: Seq<ENode>, ts: Seq<ETag>)
    requires
        before.is_prefix_of(after),
        tagged_valid(before, ts),
    ensures
        tagged_valid(after, ts),
{
    assert forall|i: int| 0 <= i < ts.len() implies #[trigger] tag_valid(after, &ts[i]) by {
        prefix(before, after, &ts[i].sentence);
        root_prefix(before, after, &ts[i].root);
    }
}

fn int_le(arena: &ETermArena, a: &T, b: &T) -> (out: bool)
    requires
        arena_ok(arena),
        valid(arena.nodes@, a),
        valid(arena.nodes@, b),
        a@ is Int,
        b@ is Int,
    ensures
        out == (int_value(a@) <= int_value(b@)),
{
    proof {
        assert(crate::k2_term::node_ok(arena.nodes@, a.root as int));
        assert(crate::k2_term::node_ok(arena.nodes@, b.root as int));
        reveal(crate::k2_term::node_ok);
    }
    match (&arena.nodes[a.root].kind, &arena.nodes[b.root].kind) {
        (
            crate::k2_term::ENodeKind::Int { magnitude: am, negative: an, value: av, .. },
            crate::k2_term::ENodeKind::Int { magnitude: bm, negative: bn, value: bv, .. },
        ) => {
            match crate::k2_term::int_order(am, *an, *av, bm, *bn, *bv) {
                crate::k2_term::EOrder::Greater => false,
                _ => true,
            }
        },
        _ => {
            proof {
                assert(false);
            }
            false
        },
    }
}

fn range_ints(arena: &ETermArena, ts: &Vec<ETag>, low: &T, high: &T, count: usize) -> (out: bool)
    requires
        arena_ok(arena),
        tagged_valid(arena.nodes@, ts@),
        valid(arena.nodes@, low),
        valid(arena.nodes@, high),
        low@ == Term::Int(1),
        high@ == Term::Int(count as int),
    ensures
        out == spec::in_range(tag_models(ts@), count as nat),
{
    let mut i = 0usize;
    while i < ts.len()
        invariant
            arena_ok(arena),
            tagged_valid(arena.nodes@, ts@),
            valid(arena.nodes@, low),
            valid(arena.nodes@, high),
            low@ == Term::Int(1),
            high@ == Term::Int(count as int),
            i <= ts.len(),
            forall|j: int| 0 <= j < i ==> 1 <= (#[trigger] tag_models(ts@)[j]).0 <= count,
        decreases ts.len() - i,
    {
        if !int_le(arena, low, &ts[i].sentence) || !int_le(arena, &ts[i].sentence, high) {
            return false;
        }
        i += 1;
    }
    true
}

pub fn in_range(arena: &mut ETermArena, ts: &Vec<ETag>, count: usize) -> (out: bool)
    requires
        arena_ok(old(arena)),
        tagged_valid(old(arena).nodes@, ts@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out == spec::in_range(tag_models(ts@), count as nat),
{
    let ghost base = arena.nodes@;
    let low = crate::m6_term::int(arena, 1);
    let ghost middle = arena.nodes@;
    let high = crate::m6_term::int(arena, count);
    proof {
        crate::k2_load::prefix_chain(base, middle, arena.nodes@);
        tags_prefix(base, arena.nodes@, ts@);
        prefix(middle, arena.nodes@, &low);
    }
    range_ints(arena, ts, &low, &high, count)
}

pub fn of_sentence(arena: &ETermArena, ts: &Vec<ETag>, s: &T) -> (out: Vec<ERoot>)
    requires
        arena_ok(arena),
        tagged_valid(arena.nodes@, ts@),
        valid(arena.nodes@, s),
        s@ is Int,
    ensures
        roots_valid(arena.nodes@, out@),
        root_models(out@) == spec::of_sentence(tag_models(ts@), int_value(s@)),
{
    let mut out = Vec::new();
    let mut i = 0usize;
    let ghost tagged = tag_models(ts@);
    let ghost pred = |p: (int, spec::Root)| p.0 == int_value(s@);
    while i < ts.len()
        invariant
            arena_ok(arena),
            tagged_valid(arena.nodes@, ts@),
            valid(arena.nodes@, s),
            s@ is Int,
            tagged == tag_models(ts@),
            pred == (|p: (int, spec::Root)| p.0 == int_value(s@)),
            i <= ts.len(),
            roots_valid(arena.nodes@, out@),
            root_models(out@) == tagged.take(i as int).filter(pred).map_values(
                |p: (int, spec::Root)| p.1,
            ),
        decreases ts.len() - i,
    {
        let matched = crate::m6_term::equal(arena, &ts[i].sentence, s);
        proof {
            assert(matched == pred(tagged[i as int]));
            assert(tagged.take(i as int + 1) == tagged.take(i as int).push(tagged[i as int]));
            tagged.take(i as int).lemma_filter_push(tagged[i as int], pred);
        }
        if matched {
            let root = clone_root(arena, &ts[i].root);
            let ghost before = out@;
            out.push(root);
            proof {
                assert forall|j: int| 0 <= j < out.len() implies #[trigger] root_valid(
                    arena.nodes@,
                    &out@[j],
                ) by {
                    if j < before.len() {
                        assert(out@[j] == before[j]);
                    }
                }
                assert_seqs_equal!(root_models(out@) == root_models(before).push(root@));
                assert_seqs_equal!(tagged.take(i as int+1).filter(pred).map_values(|p: (int, spec::Root)| p.1) == root_models(out@));
            }
        }
        i += 1;
    }
    out
}

pub fn reserved(name: &Vec<u8>) -> (out: bool)
    ensures
        out == spec::reserved_atom(name@),
{
    let plain = crate::m6_symbols::symbol_bytes(&Sym::Guideline);
    let dollar = crate::m6_symbols::symbol_bytes(&Sym::DollarGuideline);
    proof {
        reveal_strlit("guideline_");
        reveal_strlit("$guideline_");
        reveal(ckc_spec::v1text::ascii);
    }
    (name.len() >= 10 && bytes_eq(&name[0..10], &plain)) || (name.len() >= 11 && bytes_eq(
        &name[0..11],
        &dollar,
    ))
}

pub open spec fn term_opt(t: Option<T>) -> Option<Term> {
    match t {
        Some(t) => Some(t@),
        None => None,
    }
}

pub fn lemma_ref(arena: &ETermArena, t: &T) -> (out: Option<T>)
    requires
        arena_ok(arena),
        valid(arena.nodes@, t),
    ensures
        out matches Some(t) ==> valid(arena.nodes@, &t),
        term_opt(out) == spec::lemma_of(t@),
{
    match parts(arena, t) {
        Some((name, a)) => {
            if (has_name(&name, &Sym::Object) && a.len() == 6) || (has_name(&name, &Sym::Predicate)
                && a.len() >= 3) || (has_name(&name, &Sym::Property) && a.len() == 3) || (has_name(
                &name,
                &Sym::ModifierPp,
            ) && a.len() == 3) {
                Some(a[1].cp())
            } else {
                None
            }
        },
        None => None,
    }
}

pub open spec fn bytes_opt(v: Option<Vec<u8>>) -> Option<Seq<u8>> {
    match v {
        Some(v) => Some(v@),
        None => None,
    }
}

pub fn atom_name(arena: &ETermArena, t: &T) -> (out: Option<Vec<u8>>)
    requires
        arena_ok(arena),
        valid(arena.nodes@, t),
    ensures
        bytes_opt(out) == match t@ {
            Term::Atom(n) => Some(n),
            _ => None,
        },
{
    proof {
        assert(crate::k2_term::node_ok(arena.nodes@, t.root as int));
        reveal(crate::k2_term::node_ok);
    }
    match &arena.nodes[t.root].kind {
        crate::k2_term::ENodeKind::Atom { name } => Some(name.clone()),
        _ => None,
    }
}

proof fn collides_concat(a: Seq<Term>, b: Seq<Term>)
    ensures
        spec::collides_all(a + b) == (spec::collides_all(a) || spec::collides_all(b)),
    decreases a.len(),
{
    if a.len() > 0 {
        assert((a + b).drop_first() == a.drop_first() + b);
        collides_concat(a.drop_first(), b);
    }
    reveal_with_fuel(spec::collides_all, 2);
}

pub fn collides(arena: &ETermArena, t: &T) -> (out: bool)
    requires
        arena_ok(arena),
        valid(arena.nodes@, t),
    ensures
        out == spec::collides(t@),
{
    let mut todo = Vec::new();
    todo.push(t.cp());
    proof {
        reveal_with_fuel(spec::collides_all, 2);
    }
    while todo.len() > 0
        invariant
            arena_ok(arena),
            valid(arena.nodes@, t),
            valid_all(arena.nodes@, todo@),
            spec::collides_all(models(todo@)) == spec::collides(t@),
        decreases crate::k2_engine::terms_size(models(todo@)),
    {
        let ghost before = todo@;
        let current = todo.remove(0);
        proof {
            assert(models(before).drop_first() == models(todo@));
            reveal_with_fuel(spec::collides_all, 1);
            reveal_with_fuel(spec::collides, 1);
            reveal_with_fuel(crate::k2_engine::terms_size, 1);
        }
        if let Some((name, children)) = parts(arena, &current) {
            if reserved(&name) {
                return true;
            }
            if let Some(lemma) = lemma_ref(arena, &current) {
                if let Some(name) = atom_name(arena, &lemma) {
                    if reserved(&name) {
                        return true;
                    }
                }
            }
            proof {
                collides_concat(models(children@), models(todo@));
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
        reveal(spec::collides_all);
    }
    false
}

} // verus!
