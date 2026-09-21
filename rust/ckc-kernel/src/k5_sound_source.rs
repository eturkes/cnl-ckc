use crate::{k5_sound_escape as e, k5_sound_provenance as p};
use ckc_spec::{check as ck, ui as u};
use vstd::prelude::*;
verus! {

pub proof fn first_byte_bound(s: u::Bytes, b: u8, i: nat)
    requires
        i <= s.len(),
    ensures
        i <= ckc_spec::replay::first_byte(s, b, i) <= s.len(),
    decreases s.len() - i,
{
    if i < s.len() && s[i as int] != b {
        first_byte_bound(s, b, i + 1);
    }
}

pub proof fn split_spans(s: u::Bytes, b: u8, inputs: Seq<u::Bytes>)
    requires
        u::copied_span(s, inputs),
    ensures
        forall|i: int|
            0 <= i < ck::split_on(s, b).len() ==> u::copied_span(
                #[trigger] ck::split_on(s, b)[i],
                inputs,
            ),
    decreases s.len(),
{
    if s.len() == 0 {
        assert(s =~= Seq::<u8>::empty());
        assert(ck::split_on(s, b) =~= seq![s]);
    } else {
        let n = ckc_spec::replay::first_byte(s, b, 0);
        first_byte_bound(s, b, 0);
        if n < s.len() {
            p::copied_subspan(s, inputs, 0, n as int);
            p::copied_subspan(s, inputs, n as int + 1, s.len() as int);
            assert(s.take(n as int) =~= s.subrange(0, n as int));
            assert(s.skip(n as int + 1) =~= s.subrange(n as int + 1, s.len() as int));
            split_spans(s.skip(n as int + 1), b, inputs);
            assert forall|i: int| 0 <= i < ck::split_on(s, b).len() implies u::copied_span(
                #[trigger] ck::split_on(s, b)[i],
                inputs,
            ) by {
                if i > 0 {
                    assert(ck::split_on(s, b)[i] == ck::split_on(s.skip(n as int + 1), b)[i - 1]);
                }
            }
        }
    }
}

pub proof fn width_bound(s: u::Bytes, i: nat)
    requires
        i <= s.len(),
    ensures
        ck::ws_len(s, i) <= s.len() - i,
{
}

pub proof fn end_bound(s: u::Bytes, i: nat, acc: nat)
    requires
        i <= s.len(),
        acc <= s.len(),
    ensures
        ck::last_end(s, i, acc) <= s.len(),
    decreases s.len() - i,
{
    if i < s.len() {
        width_bound(s, i);
        if ck::ws_len(s, i) > 0 {
            end_bound(s, i + ck::ws_len(s, i), acc);
        } else {
            end_bound(s, i + 1, i + 1);
        }
    }
}

pub proof fn stripped_span(s: u::Bytes, inputs: Seq<u::Bytes>)
    requires
        u::copied_span(s, inputs),
    ensures
        u::copied_span(ck::strip_ws(s), inputs),
{
    end_bound(s, 0, 0);
    let l = ck::lead_ws(s, 0);
    let end = ck::last_end(s, 0, 0);
    if l < end {
        p::copied_subspan(s, inputs, l as int, end as int);
    } else {
        p::copied_subspan(s, inputs, 0, 0);
        assert(s.subrange(0, 0) =~= Seq::<u8>::empty());
    }
}

pub proof fn unprefix_span(s: u::Bytes, prefix: u::Bytes, inputs: Seq<u::Bytes>)
    requires
        u::copied_span(s, inputs),
    ensures
        u::copied_span(u::unprefix(s, prefix), inputs),
{
    if ck::starts(s, prefix) {
        p::copied_subspan(s, inputs, prefix.len() as int, s.len() as int);
        assert(s.skip(prefix.len() as int) =~= s.subrange(prefix.len() as int, s.len() as int));
    }
}

pub proof fn unsuffix_span(s: u::Bytes, suffix: u::Bytes, inputs: Seq<u::Bytes>)
    requires
        u::copied_span(s, inputs),
    ensures
        u::copied_span(u::unsuffix(s, suffix), inputs),
{
    if ck::ends(s, suffix) {
        p::copied_subspan(s, inputs, 0, s.len() - suffix.len());
        assert(s.take(s.len() - suffix.len()) =~= s.subrange(0, s.len() - suffix.len()));
    }
}

pub proof fn at_span(xs: Seq<u::Bytes>, i: int, inputs: Seq<u::Bytes>)
    requires
        inputs.len() > 0,
        forall|j: int| 0 <= j < xs.len() ==> u::copied_span(#[trigger] xs[j], inputs),
    ensures
        u::copied_span(u::at(xs, i), inputs),
{
    if !(0 <= i < xs.len()) {
        p::copied_at(inputs, 0, 0, 0);
        assert(inputs[0].subrange(0, 0) =~= Seq::<u8>::empty());
    }
}

pub proof fn first_title_span(xs: Seq<u::Bytes>, fallback: u::Bytes, inputs: Seq<u::Bytes>)
    requires
        u::copied_span(fallback, inputs),
        forall|i: int| 0 <= i < xs.len() ==> u::copied_span(#[trigger] xs[i], inputs),
    ensures
        u::copied_span(u::first_title(xs, fallback), inputs),
    decreases xs.len(),
{
    reveal_strlit("# ");
    e::ascii_literal("# "@);
    assert(u::lit("# "@).len() == 2);
    if xs.len() > 0 {
        if ck::starts(xs[0], u::lit("# "@)) && ck::strip_ws(xs[0].skip(2)).len() > 0 {
            p::copied_subspan(xs[0], inputs, 2, xs[0].len() as int);
            assert(xs[0].skip(2) =~= xs[0].subrange(2, xs[0].len() as int));
            stripped_span(xs[0].skip(2), inputs);
        } else {
            assert forall|i: int| 0 <= i < xs.drop_first().len() implies u::copied_span(
                #[trigger] xs.drop_first()[i],
                inputs,
            ) by {
                assert(xs.drop_first()[i] == xs[i + 1]);
            }
            first_title_span(xs.drop_first(), fallback, inputs);
        }
    }
}

pub proof fn append_member<A>(a: Seq<A>, b: Seq<A>, x: A)
    requires
        a.contains(x) || b.contains(x),
    ensures
        (a + b).contains(x),
{
    if a.contains(x) {
        let i = choose|i: int| 0 <= i < a.len() && #[trigger] a[i] == x;
        assert((a + b)[i] == x);
    } else {
        let i = choose|i: int| 0 <= i < b.len() && #[trigger] b[i] == x;
        assert((a + b)[a.len() as int + i] == x);
    }
}

pub proof fn flat_member<A>(xs: Seq<Seq<A>>, i: int, j: int)
    requires
        0 <= i < xs.len(),
        0 <= j < xs[i].len(),
    ensures
        xs.flatten().contains(xs[i][j]),
    decreases i,
{
    if i == 0 {
        assert(xs[0].contains(xs[0][j]));
        append_member(xs[0], xs.drop_first().flatten(), xs[0][j]);
    } else {
        assert(xs.drop_first()[i - 1] == xs[i]);
        flat_member(xs.drop_first(), i - 1, j);
        append_member(xs[0], xs.drop_first().flatten(), xs[i][j]);
    }
}

} // verus!
