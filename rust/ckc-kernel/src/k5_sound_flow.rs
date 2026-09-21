use crate::k5_sound_escape as escaped;
use ckc_spec::{ui as u, v1text as v};
use vstd::prelude::*;
verus! {

pub open spec fn next(p: u::Piece, c: int) -> int {
    match p {
        u::Piece::Fixed(b) => u::fixed_context(b, c),
        _ => c,
    }
}

pub open spec fn slot_ok(p: u::Piece, c: int) -> bool {
    match p {
        u::Piece::Fixed(b) => u::copy_registry().contains(b),
        u::Piece::Text(b) => c == 0 && u::escaped(u::escape_text(b), false),
        u::Piece::Attr(b) => (c == 2 || c == 3) && u::escaped(u::escape_attr(b), true),
        u::Piece::Decimal(n) => (c == 0 || c == 2 || c == 3) && u::digits(
            ckc_spec::check::nat_bytes(n),
        ),
        u::Piece::Comment(b) => c == 4 && v::all_in(
            u::comment_safe(b),
            |x: u8| v::is_alnum_b(x) || x == 32 || x == 58 || x == 46,
        ),
    }
}

pub open spec fn flow(h: u::Html, c: int, d: int) -> bool
    decreases h.len(),
{
    if h.len() == 0 {
        c == d
    } else {
        slot_ok(h[0], c) && flow(h.drop_first(), next(h[0], c), d)
    }
}

pub proof fn flow_equiv(h: u::Html, c: int)
    ensures
        flow(h, c, 0) == u::escaped_slots(h, c),
    decreases h.len(),
{
    if h.len() > 0 {
        flow_equiv(h.drop_first(), next(h[0], c));
    }
}

pub proof fn flow_append(a: u::Html, b: u::Html, c: int, m: int, d: int)
    requires
        flow(a, c, m),
        flow(b, m, d),
    ensures
        flow(a + b, c, d),
    decreases a.len(),
{
    if a.len() == 0 {
        assert(a + b =~= b);
    } else {
        assert((a + b).drop_first() =~= a.drop_first() + b);
        flow_append(a.drop_first(), b, next(a[0], c), m, d);
    }
}

pub proof fn flow_fixed(b: u::Bytes, c: int, d: int)
    requires
        u::copy_registry().contains(b),
        u::fixed_context(b, c) == d,
    ensures
        flow(u::fixed_bytes(b), c, d),
{
    hide(u::copy_registry);
    hide(u::fixed_context);
    reveal_with_fuel(flow, 2);
}

pub proof fn flow_text(b: u::Bytes)
    ensures
        flow(u::text(b), 0, 0),
{
    reveal_with_fuel(flow, 2);
    escaped::escape_good(b, false);
}

pub proof fn flow_attr(b: u::Bytes, c: int)
    requires
        c == 2 || c == 3,
    ensures
        flow(u::attr(b), c, c),
{
    reveal_with_fuel(flow, 2);
    escaped::escape_good(b, true);
}

pub proof fn flow_number(n: nat, c: int)
    requires
        c == 0 || c == 2 || c == 3,
    ensures
        flow(u::number(n), c, c),
{
    reveal_with_fuel(flow, 2);
    escaped::decimal_good(n);
}

pub proof fn flow_comment(b: u::Bytes)
    ensures
        flow(seq![u::Piece::Comment(b)], 4, 4),
{
    reveal_with_fuel(flow, 2);
    escaped::comment_good(b);
}

pub proof fn flow_join(xs: Seq<u::Html>, sep: u::Html, c: int)
    requires
        flow(sep, c, c),
        forall|i: int| 0 <= i < xs.len() ==> flow(#[trigger] xs[i], c, c),
    ensures
        flow(u::hjoin(xs, sep), c, c),
    decreases xs.len(),
{
    if xs.len() > 1 {
        assert forall|i: int| 0 <= i < xs.drop_first().len() implies flow(
            #[trigger] xs.drop_first()[i],
            c,
            c,
        ) by {
            assert(xs.drop_first()[i] == xs[i + 1]);
        }
        flow_join(xs.drop_first(), sep, c);
        flow_append(xs[0], sep, c, c, c);
        flow_append(xs[0] + sep, u::hjoin(xs.drop_first(), sep), c, c, c);
    }
}

pub proof fn flow_path(xs: Seq<u::Html>, sep: u::Html, states: Seq<int>)
    requires
        states.len() == xs.len() + 1,
        forall|i: int| 0 <= i < xs.len() ==> flow(#[trigger] xs[i], states[i], states[i + 1]),
        forall|i: int| 0 < i < xs.len() ==> flow(sep, #[trigger] states[i], states[i]),
    ensures
        flow(u::hjoin(xs, sep), states[0], states.last()),
    decreases xs.len(),
{
    if xs.len() > 1 {
        assert forall|i: int| 0 <= i < xs.drop_first().len() implies flow(
            #[trigger] xs.drop_first()[i],
            states.drop_first()[i],
            states.drop_first()[i + 1],
        ) by {
            assert(xs.drop_first()[i] == xs[i + 1]);
        }
        assert forall|i: int| 0 < i < xs.drop_first().len() implies flow(
            sep,
            #[trigger] states.drop_first()[i],
            states.drop_first()[i],
        ) by {
            assert(states.drop_first()[i] == states[i + 1]);
        }
        flow_path(xs.drop_first(), sep, states.drop_first());
        flow_append(xs[0], sep, states[0], states[1], states[1]);
        assert(states.drop_first().last() == states.last());
        flow_append(
            xs[0] + sep,
            u::hjoin(xs.drop_first(), sep),
            states[0],
            states[1],
            states.last(),
        );
    }
}

pub proof fn visible_append(a: u::Html, b: u::Html, c: u::Corpus, r: Seq<u::Bytes>)
    requires
        u::visible_bytes_from(a, c, r),
        u::visible_bytes_from(b, c, r),
    ensures
        u::visible_bytes_from(a + b, c, r),
{
    assert forall|i: int| 0 <= i < (a + b).len() implies match #[trigger] (a + b)[i] {
        u::Piece::Fixed(x) => r.contains(x),
        u::Piece::Text(x) => u::copy_derived(x, u::corpus_bytes(c), r),
        _ => true,
    } by {
        if i < a.len() {
            assert((a + b)[i] == a[i]);
        } else {
            assert((a + b)[i] == b[i - a.len() as int]);
        }
    }
}

pub proof fn visible_fixed(b: u::Bytes, c: u::Corpus, r: Seq<u::Bytes>)
    requires
        r.contains(b),
    ensures
        u::visible_bytes_from(u::fixed_bytes(b), c, r),
{
}

pub proof fn visible_text(b: u::Bytes, c: u::Corpus, r: Seq<u::Bytes>)
    requires
        u::copy_derived(b, u::corpus_bytes(c), r),
    ensures
        u::visible_bytes_from(u::text(b), c, r),
{
}

pub proof fn visible_other(p: u::Piece, c: u::Corpus, r: Seq<u::Bytes>)
    requires
        p is Attr || p is Decimal || p is Comment,
    ensures
        u::visible_bytes_from(seq![p], c, r),
{
}

pub proof fn visible_flat(xs: Seq<u::Html>, c: u::Corpus, r: Seq<u::Bytes>)
    requires
        forall|i: int| 0 <= i < xs.len() ==> u::visible_bytes_from(#[trigger] xs[i], c, r),
    ensures
        u::visible_bytes_from(xs.flatten(), c, r),
    decreases xs.len(),
{
    if xs.len() > 0 {
        assert forall|i: int| 0 <= i < xs.drop_first().len() implies u::visible_bytes_from(
            #[trigger] xs.drop_first()[i],
            c,
            r,
        ) by {
            assert(xs.drop_first()[i] == xs[i + 1]);
        }
        visible_flat(xs.drop_first(), c, r);
        visible_append(xs[0], xs.drop_first().flatten(), c, r);
    }
}

pub proof fn visible_join(xs: Seq<u::Html>, sep: u::Html, c: u::Corpus, r: Seq<u::Bytes>)
    requires
        u::visible_bytes_from(sep, c, r),
        forall|i: int| 0 <= i < xs.len() ==> u::visible_bytes_from(#[trigger] xs[i], c, r),
    ensures
        u::visible_bytes_from(u::hjoin(xs, sep), c, r),
    decreases xs.len(),
{
    if xs.len() > 1 {
        assert forall|i: int| 0 <= i < xs.drop_first().len() implies u::visible_bytes_from(
            #[trigger] xs.drop_first()[i],
            c,
            r,
        ) by {
            assert(xs.drop_first()[i] == xs[i + 1]);
        }
        visible_join(xs.drop_first(), sep, c, r);
        visible_append(xs[0], sep, c, r);
        visible_append(xs[0] + sep, u::hjoin(xs.drop_first(), sep), c, r);
    }
}

} // verus!
