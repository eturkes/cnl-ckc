use ckc_spec::{check as ck, ui as u, v1text as v};
use vstd::prelude::*;
use vstd::utf8::*;
verus! {

broadcast use {vstd::seq::group_seq_axioms, vstd::seq_lib::group_seq_properties};

pub open spec fn span_at(s: u::Bytes, b: u::Bytes, a: int) -> bool {
    0 <= a && a + s.len() <= b.len() && s == b.subrange(a, a + s.len() as int)
}

pub open spec fn span(s: u::Bytes, b: u::Bytes) -> bool {
    exists|a: int| #[trigger] span_at(s, b, a)
}

pub proof fn span_self(s: u::Bytes)
    ensures
        span(s, s),
{
    hide(u::copy_registry);
    assert(s.subrange(0, s.len() as int) =~= s);
    assert(span_at(s, s, 0));
}

pub proof fn span_empty(s: u::Bytes)
    ensures
        span(Seq::empty(), s),
{
    hide(u::copy_registry);
    assert(s.subrange(0, 0) =~= Seq::<u8>::empty());
    assert(span_at(Seq::empty(), s, 0));
}

pub proof fn span_sub(s: u::Bytes, b: u::Bytes, l: int, r: int)
    requires
        span(s, b),
        0 <= l <= r <= s.len(),
    ensures
        span(s.subrange(l, r), b),
{
    hide(u::copy_registry);
    let a = choose|a: int| #[trigger] span_at(s, b, a);
    assert(s.subrange(l, r) =~= b.subrange(a + l, a + r));
    assert(span_at(s.subrange(l, r), b, a + l));
}

pub proof fn span_trans(s: u::Bytes, b: u::Bytes, c: u::Bytes)
    requires
        span(s, b),
        span(b, c),
    ensures
        span(s, c),
{
    hide(u::copy_registry);
    let a = choose|a: int| #[trigger] span_at(s, b, a);
    let z = choose|z: int| #[trigger] span_at(b, c, z);
    assert(s =~= c.subrange(z + a, z + a + s.len() as int));
    assert(span_at(s, c, z + a));
}

pub proof fn first_sub_bound(b: u::Bytes, s: u::Bytes, a: nat, i: nat)
    requires
        i <= a,
        a + s.len() <= b.len(),
        b.subrange(a as int, a + s.len() as int) == s,
    ensures
        ck::first_sub(b, s, i) <= a,
    decreases a - i,
{
    hide(u::copy_registry);
    if i < a && b.subrange(i as int, i + s.len() as int) != s {
        first_sub_bound(b, s, a, i + 1);
    }
}

pub proof fn span_copied(s: u::Bytes, b: u::Bytes, inputs: Seq<u::Bytes>)
    requires
        span(s, b),
        inputs.contains(b),
    ensures
        u::copied_span(s, inputs),
{
    hide(u::copy_registry);
    let a = choose|a: int| #[trigger] span_at(s, b, a);
    let i = choose|i: int| 0 <= i < inputs.len() && inputs[i] == b;
    first_sub_bound(b, s, a as nat, 0);
    assert(ck::first_sub(inputs[i], s, 0) + s.len() <= inputs[i].len());
}

pub proof fn atom(s: u::Bytes, inputs: Seq<u::Bytes>)
    requires
        u::copied_span(s, inputs) || u::copy_registry().contains(s) || (u::digits(s) && (s.len()
            == 1 || s[0] != 48)),
    ensures
        u::copy_derived(s, inputs, u::copy_registry()),
{
    hide(u::copy_registry);
    reveal_with_fuel(u::copy_derived, 2);
    if s.len() > 0 {
        assert(s.take(s.len() as int) =~= s);
        assert(s.skip(s.len() as int) =~= Seq::<u8>::empty());
    }
}

pub proof fn source(s: u::Bytes, b: u::Bytes, inputs: Seq<u::Bytes>)
    requires
        span(s, b),
        inputs.contains(b),
    ensures
        u::copy_derived(s, inputs, u::copy_registry()),
{
    hide(u::copy_registry);
    span_copied(s, b, inputs);
    atom(s, inputs);
}

pub proof fn literal(s: u::Bytes, inputs: Seq<u::Bytes>)
    requires
        u::copy_registry().contains(s),
    ensures
        u::copy_derived(s, inputs, u::copy_registry()),
{
    hide(u::copy_registry);
    atom(s, inputs);
}

pub proof fn cat(a: u::Bytes, b: u::Bytes, inputs: Seq<u::Bytes>)
    requires
        u::copy_derived(a, inputs, u::copy_registry()),
        u::copy_derived(b, inputs, u::copy_registry()),
    ensures
        u::copy_derived(a + b, inputs, u::copy_registry()),
    decreases a.len(),
{
    hide(u::copy_registry);
    if a.len() == 0 {
        assert(a + b =~= b);
    } else {
        let k = choose|k: int|
            0 < k <= a.len() && (u::copied_span(#[trigger] a.take(k), inputs)
                || u::copy_registry().contains(a.take(k)) || (u::digits(a.take(k)) && (k == 1
                || a[0] != 48))) && u::copy_derived(a.skip(k), inputs, u::copy_registry());
        cat(a.skip(k), b, inputs);
        assert((a + b).take(k) =~= a.take(k));
        assert((a + b).skip(k) =~= a.skip(k) + b);
    }
}

pub proof fn decimal(n: nat, inputs: Seq<u::Bytes>)
    ensures
        u::copy_derived(ck::nat_bytes(n), inputs, u::copy_registry()),
{
    hide(u::copy_registry);
    crate::k5_sound_escape::decimal(n);
    if n == 0 {
        assert(ck::nat_bytes(n).len() == 1);
    }
    atom(ck::nat_bytes(n), inputs);
}

pub proof fn join(xs: Seq<u::Bytes>, sep: u::Bytes, inputs: Seq<u::Bytes>)
    requires
        forall|i: int|
            0 <= i < xs.len() ==> u::copy_derived(#[trigger] xs[i], inputs, u::copy_registry()),
        u::copy_derived(sep, inputs, u::copy_registry()),
    ensures
        u::copy_derived(u::join(xs, sep), inputs, u::copy_registry()),
    decreases xs.len(),
{
    hide(u::copy_registry);
    if xs.len() > 1 {
        join(xs.drop_first(), sep, inputs);
        cat(xs[0], sep, inputs);
        cat(xs[0] + sep, u::join(xs.drop_first(), sep), inputs);
    }
}

pub proof fn split(s: u::Bytes, b: u8)
    ensures
        forall|i: int|
            0 <= i < ck::split_on(s, b).len() ==> span(#[trigger] ck::split_on(s, b)[i], s),
    decreases s.len(),
{
    hide(u::copy_registry);
    span_self(s);
    if s.len() == 0 {
        span_empty(s);
    } else {
        let n = ckc_spec::replay::first_byte(s, b, 0);
        if n < s.len() {
            let tail = s.skip(n as int + 1);
            span_sub(s, s, 0, n as int);
            span_sub(s, s, n as int + 1, s.len() as int);
            assert(tail =~= s.subrange(n as int + 1, s.len() as int));
            split(tail, b);
            assert forall|i: int| 0 <= i < ck::split_on(s, b).len() implies span(
                #[trigger] ck::split_on(s, b)[i],
                s,
            ) by {
                if i > 0 {
                    span_trans(ck::split_on(tail, b)[i - 1], tail, s);
                }
            }
        }
    }
}

pub proof fn ws_bounds(s: u::Bytes, i: nat)
    requires
        i <= s.len(),
    ensures
        i + ck::ws_len(s, i) <= s.len(),
{
    hide(u::copy_registry);
}

pub proof fn lead_bounds(s: u::Bytes, i: nat)
    requires
        i <= s.len(),
    ensures
        ck::lead_ws(s, i) <= s.len(),
    decreases s.len() - i,
{
    hide(u::copy_registry);
    ws_bounds(s, i);
    if i < s.len() && ck::ws_len(s, i) > 0 {
        lead_bounds(s, i + ck::ws_len(s, i));
    }
}

pub proof fn last_bounds(s: u::Bytes, i: nat, a: nat)
    requires
        i <= s.len(),
        a <= s.len(),
    ensures
        ck::last_end(s, i, a) <= s.len(),
    decreases s.len() - i,
{
    hide(u::copy_registry);
    ws_bounds(s, i);
    if i < s.len() {
        if ck::ws_len(s, i) > 0 {
            last_bounds(s, i + ck::ws_len(s, i), a);
        } else {
            last_bounds(s, i + 1, i + 1);
        }
    }
}

pub proof fn stripped(s: u::Bytes)
    ensures
        span(ck::strip_ws(s), s),
{
    hide(u::copy_registry);
    lead_bounds(s, 0);
    last_bounds(s, 0, 0);
    if ck::lead_ws(s, 0) < ck::last_end(s, 0, 0) {
        span_self(s);
        span_sub(s, s, ck::lead_ws(s, 0) as int, ck::last_end(s, 0, 0) as int);
    } else {
        span_empty(s);
    }
}

pub proof fn unprefix(s: u::Bytes, p: u::Bytes)
    ensures
        span(u::unprefix(s, p), s),
{
    hide(u::copy_registry);
    span_self(s);
    if ck::starts(s, p) {
        span_sub(s, s, p.len() as int, s.len() as int);
    }
}

pub proof fn unsuffix(s: u::Bytes, p: u::Bytes)
    ensures
        span(u::unsuffix(s, p), s),
{
    hide(u::copy_registry);
    span_self(s);
    if ck::ends(s, p) {
        span_sub(s, s, 0, s.len() - p.len());
    }
}

pub proof fn slice(s: u::Bytes, l: int, r: int)
    ensures
        span(u::slice_text(s, l, r), s),
{
    hide(u::copy_registry);
    let cs = u::chars(s);
    if 0 <= l <= r <= cs.len() && valid_utf8(s) {
        decode_utf8_encode_utf8(s);
        let a = cs.take(l);
        let b = cs.subrange(l, r);
        let c = cs.skip(r);
        assert(cs =~= a + b + c);
        encode_utf8_concat(a, b);
        encode_utf8_concat(a + b, c);
        let start = encode_utf8(a).len();
        assert(encode_utf8(b) =~= s.subrange(start as int, (start + encode_utf8(b).len()) as int));
        assert(span_at(encode_utf8(b), s, start as int));
    } else {
        span_empty(s);
    }
}

} // verus!
