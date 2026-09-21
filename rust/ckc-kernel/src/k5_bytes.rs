use ckc_spec::check as ck;
use ckc_spec::ui as u;
use vstd::prelude::*;
use vstd::slice::slice_to_vec;
use vstd::string::StringSliceAdditionalSpecFns;

verus! {

pub open spec fn views(xs: Seq<Vec<u8>>) -> Seq<Seq<u8>> {
    xs.map_values(|x: Vec<u8>| x@)
}

pub fn literal(s: &str) -> (out: Vec<u8>)
    ensures
        out@ == u::lit(s@),
{
    slice_to_vec(s.as_bytes())
}

pub fn append(out: &mut Vec<u8>, s: &[u8])
    ensures
        final(out)@ == old(out)@ + s@,
{
    out.extend_from_slice(s);
    assert(final(out)@ =~= old(out)@ + s@);
}

pub fn cat(mut a: Vec<u8>, b: &[u8]) -> (out: Vec<u8>)
    ensures
        out@ == a@ + b@,
{
    append(&mut a, b);
    a
}

pub fn equal(a: &[u8], b: &[u8]) -> (yes: bool)
    ensures
        yes == (a@ == b@),
{
    if a.len() != b.len() {
        return false;
    }
    let mut i = 0;
    while i < a.len()
        invariant
            a.len() == b.len(),
            i <= a.len(),
            forall|j: int| 0 <= j < i ==> a@[j] == b@[j],
        decreases a.len() - i,
    {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    assert(a@ =~= b@);
    true
}

pub fn starts(s: &[u8], p: &[u8]) -> (yes: bool)
    ensures
        yes == ck::starts(s@, p@),
{
    if p.len() > s.len() {
        false
    } else {
        equal(&s[0..p.len()], p)
    }
}

pub fn ends(s: &[u8], p: &[u8]) -> (yes: bool)
    ensures
        yes == ck::ends(s@, p@),
{
    if p.len() > s.len() {
        false
    } else {
        equal(&s[s.len() - p.len()..s.len()], p)
    }
}

pub fn nat_bytes(n: u64) -> (out: Vec<u8>)
    ensures
        out@ == ck::nat_bytes(n as nat),
    decreases n,
{
    if n < 10 {
        let mut out = Vec::new();
        out.push(48 + n as u8);
        proof {
            reveal_with_fuel(ckc_spec::v1text::udec_bytes, 2);
            reveal(ckc_spec::v1text::digit_byte);
        }
        out
    } else {
        let mut out = nat_bytes(n / 10);
        out.push(48 + (n % 10) as u8);
        proof {
            reveal_with_fuel(ckc_spec::v1text::udec_bytes, 2);
            reveal(ckc_spec::v1text::digit_byte);
        }
        out
    }
}

pub fn escape(s: &[u8], attr: bool) -> (out: Vec<u8>)
    ensures
        out@ == u::escape(s@, attr),
{
    let mut out = Vec::new();
    let mut i = 0;
    proof {
        assert(s@.skip(0) =~= s@);
    }
    while i < s.len()
        invariant
            i <= s.len(),
            out@ + u::escape(s@.skip(i as int), attr) == u::escape(s@, attr),
        decreases s.len() - i,
    {
        let ghost tail = s@.skip(i as int);
        let c = s[i];
        if c == 38 {
            let b = literal("&amp;");
            append(&mut out, &b);
        } else if c == 60 {
            let b = literal("&lt;");
            append(&mut out, &b);
        } else if c == 62 {
            let b = literal("&gt;");
            append(&mut out, &b);
        } else if attr && c == 34 {
            let b = literal("&quot;");
            append(&mut out, &b);
        } else if attr && c == 39 {
            let b = literal("&#x27;");
            append(&mut out, &b);
        } else {
            out.push(c);
        }
        proof {
            assert(tail.drop_first() =~= s@.skip(i as int + 1));
        }
        i += 1;
    }
    out
}

pub proof fn first_byte_exact(s: Seq<u8>, b: u8, from: nat, at: nat)
    requires
        from <= at <= s.len(),
        forall|j: int| from <= j < at ==> s[j] != b,
        at == s.len() || s[at as int] == b,
    ensures
        ckc_spec::replay::first_byte(s, b, from) == at,
    decreases at - from,
{
    if from < at {
        first_byte_exact(s, b, from + 1, at);
    }
}

pub fn first_byte(s: &[u8], b: u8, from: usize) -> (at: usize)
    requires
        from <= s.len(),
    ensures
        from <= at <= s.len(),
        at == ckc_spec::replay::first_byte(s@, b, from as nat),
{
    let mut at = from;
    while at < s.len()
        invariant
            from <= at <= s.len(),
            forall|j: int| from <= j < at ==> s@[j] != b,
        decreases s.len() - at,
    {
        if s[at] == b {
            proof {
                first_byte_exact(s@, b, from as nat, at as nat);
            }
            return at;
        }
        at += 1;
    }
    proof {
        first_byte_exact(s@, b, from as nat, at as nat);
    }
    at
}

pub fn split(s: &[u8], b: u8) -> (out: Vec<Vec<u8>>)
    ensures
        views(out@) == ck::split_on(s@, b),
{
    let mut out = Vec::new();
    let mut from = 0;
    proof {
        assert(s@.skip(0) =~= s@);
    }
    loop
        invariant
            from <= s.len(),
            views(out@) + ck::split_on(s@.skip(from as int), b) == ck::split_on(s@, b),
        decreases s.len() - from + 1,
    {
        let tail = &s[from..s.len()];
        let n = first_byte(tail, b, 0);
        let ghost before = views(out@);
        out.push(slice_to_vec(&tail[0..n]));
        proof {
            assert(views(out@) =~= before.push(tail@.take(n as int)));
        }
        if n == tail.len() {
            proof {
                assert(tail@.take(n as int) =~= tail@);
                if tail.len() == 0 {
                    assert(tail@ =~= Seq::<u8>::empty());
                }
                assert(ck::split_on(tail@, b) == seq![tail@]);
                assert(s@.skip(from as int) =~= tail@);
                assert(views(out@) =~= before + seq![tail@]);
            }
            return out;
        }
        proof {
            assert(tail@.skip(n as int + 1) =~= s@.skip(from as int + n as int + 1));
        }
        from = from + n + 1;
    }
}

pub fn at(xs: &Vec<Vec<u8>>, i: usize) -> (out: Vec<u8>)
    ensures
        out@ == u::at(views(xs@), i as int),
{
    if i < xs.len() {
        slice_to_vec(&xs[i])
    } else {
        Vec::new()
    }
}

pub fn join(xs: &Vec<Vec<u8>>, sep: &[u8]) -> (out: Vec<u8>)
    ensures
        out@ == u::join(views(xs@), sep@),
{
    let mut out = Vec::new();
    let mut i = 0;
    proof {
        assert(views(xs@).skip(0) =~= views(xs@));
    }
    while i < xs.len()
        invariant
            i <= xs.len(),
            u::join(views(xs@), sep@) == out@ + (if 0 < i < xs.len() {
                sep@
            } else {
                Seq::empty()
            }) + u::join(views(xs@).skip(i as int), sep@),
        decreases xs.len() - i,
    {
        let ghost tail = views(xs@).skip(i as int);
        if i > 0 {
            append(&mut out, sep);
        }
        append(&mut out, &xs[i]);
        proof {
            assert(tail.drop_first() =~= views(xs@).skip(i as int + 1));
        }
        i += 1;
    }
    out
}

pub fn raw_rows(s: &[u8]) -> (out: Vec<Vec<u8>>)
    ensures
        views(out@) == u::raw_rows(s@),
{
    let xs = split(s, 10);
    let mut out = Vec::new();
    let mut i = 0;
    while i < xs.len()
        invariant
            i <= xs.len(),
            views(xs@) == ck::split_on(s@, 10),
            views(out@) == views(xs@).take(i as int).filter(|r: Seq<u8>| r.len() > 0 && r[0] != 35),
        decreases xs.len() - i,
    {
        let x = &xs[i];
        if x.len() > 0 && x[0] != 35 {
            out.push(slice_to_vec(x));
        }
        proof {
            let ghost before = views(xs@).take(i as int);
            assert(views(xs@).take(i as int + 1) =~= before.push(x@));
            before.lemma_filter_push(x@, |r: Seq<u8>| r.len() > 0 && r[0] != 35);
        }
        i += 1;
    }
    proof {
        assert(views(xs@).take(i as int) =~= views(xs@));
    }
    out
}

} // verus!
