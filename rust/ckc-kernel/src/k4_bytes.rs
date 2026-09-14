use ckc_spec::check::*;
#[cfg(verus_keep_ghost)]
use ckc_spec::replay::first_byte;
use vstd::assert_seqs_equal;
use vstd::prelude::*;
use vstd::slice::{slice_subrange, slice_to_vec};

verus! {

pub fn copy(s: &[u8]) -> (r: Vec<u8>)
    ensures
        r@ == s@,
{
    slice_to_vec(s)
}

pub fn range(s: &[u8], start: usize, end: usize) -> (r: Vec<u8>)
    requires
        start <= end <= s@.len(),
    ensures
        r@ == s@.subrange(start as int, end as int),
{
    slice_to_vec(slice_subrange(s, start, end))
}

pub fn append(out: &mut Vec<u8>, s: &[u8])
    ensures
        final(out)@ == old(out)@ + s@,
{
    let ghost base = out@;
    let mut i = 0usize;
    while i < s.len()
        invariant
            i <= s@.len(),
            out@ == base + s@.take(i as int),
        decreases s.len() - i,
    {
        out.push(s[i]);
        proof {
            assert_seqs_equal!(s@.take(i as int + 1) == s@.take(i as int).push(s@[i as int]));
        }
        i += 1;
    }
}

pub fn concat(a: &[u8], b: &[u8]) -> (r: Vec<u8>)
    ensures
        r@ == a@ + b@,
{
    let mut r = copy(a);
    append(&mut r, b);
    r
}

pub fn eq(a: &[u8], b: &[u8]) -> (r: bool)
    ensures
        r == (a@ == b@),
{
    if a.len() != b.len() {
        return false;
    }
    let mut i = 0usize;
    while i < a.len()
        invariant
            a@.len() == b@.len(),
            i <= a@.len(),
            forall|j: int| 0 <= j < i ==> a@[j] == b@[j],
        decreases a.len() - i,
    {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(a@ == b@);
    }
    true
}

pub fn starts_with(s: &[u8], p: &[u8]) -> (r: bool)
    ensures
        r == starts(s@, p@),
{
    if s.len() < p.len() {
        false
    } else {
        eq(slice_subrange(s, 0, p.len()), p)
    }
}

pub fn ends_with(s: &[u8], p: &[u8]) -> (r: bool)
    ensures
        r == ends(s@, p@),
{
    if s.len() < p.len() {
        false
    } else {
        eq(slice_subrange(s, s.len() - p.len(), s.len()), p)
    }
}

pub proof fn first_exact(s: Seq<u8>, b: u8, start: nat, end: nat)
    requires
        start <= end <= s.len(),
        forall|j: int| start <= j < end ==> s[j] != b,
        end == s.len() || s[end as int] == b,
    ensures
        first_byte(s, b, start) == end,
    decreases end - start,
{
    reveal_with_fuel(first_byte, 2);
    if start < end {
        first_exact(s, b, start + 1, end);
    }
}

pub fn first(s: &[u8], b: u8, start: usize) -> (r: usize)
    requires
        start <= s@.len(),
    ensures
        start <= r <= s@.len(),
        r as nat == first_byte(s@, b, start as nat),
        forall|j: int| start <= j < r ==> s@[j] != b,
        r < s@.len() ==> s@[r as int] == b,
{
    let mut i = start;
    while i < s.len()
        invariant
            start <= i <= s@.len(),
            forall|j: int| start <= j < i ==> s@[j] != b,
        decreases s.len() - i,
    {
        if s[i] == b {
            proof {
                first_exact(s@, b, start as nat, i as nat);
            }
            return i;
        }
        i += 1;
    }
    proof {
        first_exact(s@, b, start as nat, i as nat);
    }
    i
}

pub fn has(s: &[u8], b: u8) -> (r: bool)
    ensures
        r == has_byte(s@, b),
{
    let i = first(s, b, 0);
    i < s.len()
}

pub proof fn byte_rows_push(v: Seq<Vec<u8>>, x: Vec<u8>)
    ensures
        byte_rows(v.push(x)) == byte_rows(v).push(x@),
{
    assert_seqs_equal!(byte_rows(v.push(x)) == byte_rows(v).push(x@));
}

pub fn split(s: &[u8], b: u8) -> (out: Vec<Vec<u8>>)
    ensures
        byte_rows(out@) == split_on(s@, b),
{
    let mut out = Vec::new();
    let mut start = 0usize;
    proof {
        assert_seqs_equal!(s@.skip(0) == s@);
        assert(byte_rows(out@) == Seq::<Seq<u8>>::empty());
    }
    while start < s.len()
        invariant
            start <= s@.len(),
            split_on(s@, b) == byte_rows(out@) + split_on(s@.skip(start as int), b),
        decreases s.len() - start,
    {
        let rest = slice_subrange(s, start, s.len());
        let n = first(rest, b, 0);
        let field = range(rest, 0, n);
        let ghost before = out@;
        let ghost field_view = field@;
        proof {
            byte_rows_push(before, field);
            reveal_with_fuel(split_on, 2);
        }
        out.push(field);
        if n == rest.len() {
            proof {
                assert(field_view == rest@);
            }
            return out;
        }
        proof {
            assert_seqs_equal!(rest@.skip(n as int + 1) == s@.skip(start as int + n as int + 1));
        }
        start = start + n + 1;
    }
    let empty = Vec::new();
    proof {
        byte_rows_push(out@, empty);
        reveal_with_fuel(split_on, 2);
    }
    out.push(empty);
    out
}

pub fn contains(rows: &Vec<Vec<u8>>, key: &[u8]) -> (found: bool)
    ensures
        found == byte_rows(rows@).contains(key@),
{
    let mut i = 0usize;
    while i < rows.len()
        invariant
            i <= rows@.len(),
            forall|j: int| 0 <= j < i ==> rows@[j]@ != key@,
        decreases rows.len() - i,
    {
        if eq(&rows[i], key) {
            proof {
                assert(byte_rows(rows@)[i as int] == key@);
            }
            return true;
        }
        i += 1;
    }
    false
}

pub fn less(a: &[u8], b: &[u8]) -> (r: bool)
    ensures
        r == ckc_spec::engine::bytes_lt(a@, b@),
{
    let mut i = 0usize;
    proof {
        assert_seqs_equal!(a@.skip(0) == a@);
        assert_seqs_equal!(b@.skip(0) == b@);
    }
    while i < a.len() && i < b.len()
        invariant
            i <= a@.len(),
            i <= b@.len(),
            ckc_spec::engine::bytes_lt(a@, b@) == ckc_spec::engine::bytes_lt(
                a@.skip(i as int),
                b@.skip(i as int),
            ),
        decreases a.len() - i,
    {
        proof {
            reveal_with_fuel(ckc_spec::engine::bytes_lt, 2);
        }
        if a[i] != b[i] {
            return a[i] < b[i];
        }
        proof {
            assert_seqs_equal!(a@.skip(i as int).drop_first() == a@.skip(i as int + 1));
            assert_seqs_equal!(b@.skip(i as int).drop_first() == b@.skip(i as int + 1));
        }
        i += 1;
    }
    proof {
        reveal_with_fuel(ckc_spec::engine::bytes_lt, 2);
    }
    i < b.len()
}

} // verus!
