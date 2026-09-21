use ckc_spec::{check as ck, ui as u};
use vstd::prelude::*;
verus! {

pub proof fn first_sub_bound(s: u::Bytes, p: u::Bytes, from: nat, at: nat)
    requires
        from <= at,
        at + p.len() <= s.len(),
        s.subrange(at as int, (at + p.len()) as int) == p,
    ensures
        ck::first_sub(s, p, from) <= at,
    decreases at - from,
{
    if s.subrange(from as int, (from + p.len()) as int) != p {
        assert(from < at);
        first_sub_bound(s, p, from + 1, at);
    }
}

pub proof fn first_sub_match(s: u::Bytes, p: u::Bytes, from: nat)
    requires
        from <= s.len(),
        ck::first_sub(s, p, from) + p.len() <= s.len(),
    ensures
        s.subrange(ck::first_sub(s, p, from) as int, (ck::first_sub(s, p, from) + p.len()) as int)
            == p,
    decreases s.len() + 1 - from,
{
    if from + p.len() <= s.len() && s.subrange(from as int, (from + p.len()) as int) != p {
        if from == s.len() {
            assert(p =~= Seq::<u8>::empty());
            assert(s.subrange(from as int, (from + p.len()) as int) =~= p);
        }
        assert(from < s.len());
        first_sub_match(s, p, from + 1);
    }
}

pub proof fn copied_at(inputs: Seq<u::Bytes>, i: int, a: int, b: int)
    requires
        0 <= i < inputs.len(),
        0 <= a <= b <= inputs[i].len(),
    ensures
        u::copied_span(inputs[i].subrange(a, b), inputs),
{
    let p = inputs[i].subrange(a, b);
    first_sub_bound(inputs[i], p, 0, a as nat);
    assert(ck::first_sub(inputs[i], p, 0) + p.len() <= inputs[i].len());
}

pub proof fn copied_member(s: u::Bytes, inputs: Seq<u::Bytes>)
    requires
        inputs.contains(s),
    ensures
        u::copied_span(s, inputs),
{
    let i = choose|i: int| 0 <= i < inputs.len() && inputs[i] == s;
    copied_at(inputs, i, 0, s.len() as int);
    assert(inputs[i].subrange(0, s.len() as int) =~= s);
}

pub proof fn copied_subspan(s: u::Bytes, inputs: Seq<u::Bytes>, a: int, b: int)
    requires
        u::copied_span(s, inputs),
        0 <= a <= b <= s.len(),
    ensures
        u::copied_span(s.subrange(a, b), inputs),
{
    let i = choose|i: int|
        0 <= i < inputs.len() && ck::first_sub(inputs[i], s, 0) + s.len() <= inputs[i].len();
    let n = ck::first_sub(inputs[i], s, 0);
    first_sub_match(inputs[i], s, 0);
    copied_at(inputs, i, n as int + a, n as int + b);
    assert(inputs[i].subrange(n as int + a, n as int + b) =~= s.subrange(a, b));
}

pub open spec fn eligible(s: u::Bytes, inputs: Seq<u::Bytes>, registry: Seq<u::Bytes>) -> bool {
    u::copied_span(s, inputs) || registry.contains(s) || (u::digits(s) && (s.len() == 1 || s[0]
        != 48))
}

pub proof fn derived_one(s: u::Bytes, inputs: Seq<u::Bytes>, registry: Seq<u::Bytes>)
    requires
        s.len() == 0 || eligible(s, inputs, registry),
    ensures
        u::copy_derived(s, inputs, registry),
{
    if s.len() > 0 {
        let k = s.len() as int;
        assert(s.take(k) =~= s);
        assert(s.skip(k) =~= Seq::<u8>::empty());
        assert(u::copy_derived(s.skip(k), inputs, registry));
        assert(0 < k <= s.len() && (u::copied_span(s.take(k), inputs) || registry.contains(
            s.take(k),
        ) || (u::digits(s.take(k)) && (k == 1 || s[0] != 48))) && u::copy_derived(
            s.skip(k),
            inputs,
            registry,
        ));
    }
}

pub proof fn derived_append(
    a: u::Bytes,
    b: u::Bytes,
    inputs: Seq<u::Bytes>,
    registry: Seq<u::Bytes>,
)
    requires
        u::copy_derived(a, inputs, registry),
        u::copy_derived(b, inputs, registry),
    ensures
        u::copy_derived(a + b, inputs, registry),
    decreases a.len(),
{
    if a.len() == 0 {
        assert(a + b =~= b);
    } else {
        let k = choose|k: int|
            0 < k <= a.len() && (u::copied_span(a.take(k), inputs) || registry.contains(a.take(k))
                || (u::digits(a.take(k)) && (k == 1 || a[0] != 48))) && u::copy_derived(
                a.skip(k),
                inputs,
                registry,
            );
        derived_append(a.skip(k), b, inputs, registry);
        assert((a + b).take(k) =~= a.take(k));
        assert((a + b).skip(k) =~= a.skip(k) + b);
        assert((a + b)[0] == a[0]);
        assert(0 < k <= (a + b).len() && (u::copied_span((a + b).take(k), inputs)
            || registry.contains((a + b).take(k)) || (u::digits((a + b).take(k)) && (k == 1 || (a
            + b)[0] != 48))) && u::copy_derived((a + b).skip(k), inputs, registry));
    }
}

pub proof fn derived_join(
    xs: Seq<u::Bytes>,
    sep: u::Bytes,
    inputs: Seq<u::Bytes>,
    registry: Seq<u::Bytes>,
)
    requires
        u::copy_derived(sep, inputs, registry),
        forall|i: int| 0 <= i < xs.len() ==> u::copy_derived(#[trigger] xs[i], inputs, registry),
    ensures
        u::copy_derived(u::join(xs, sep), inputs, registry),
    decreases xs.len(),
{
    if xs.len() > 1 {
        assert forall|i: int| 0 <= i < xs.drop_first().len() implies u::copy_derived(
            #[trigger] xs.drop_first()[i],
            inputs,
            registry,
        ) by {
            assert(xs.drop_first()[i] == xs[i + 1]);
        }
        derived_join(xs.drop_first(), sep, inputs, registry);
        derived_append(xs[0], sep, inputs, registry);
        derived_append(xs[0] + sep, u::join(xs.drop_first(), sep), inputs, registry);
    }
}

pub proof fn derived_number(n: nat, inputs: Seq<u::Bytes>, registry: Seq<u::Bytes>)
    ensures
        u::copy_derived(ck::nat_bytes(n), inputs, registry),
{
    crate::k5_sound_escape::decimal_good(n);
    derived_one(ck::nat_bytes(n), inputs, registry);
}

} // verus!
