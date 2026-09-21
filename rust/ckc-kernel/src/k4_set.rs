#[cfg(verus_keep_ghost)]
use crate::k4_bytes::byte_rows_push;
use crate::k4_bytes::{contains, copy};
#[cfg(verus_keep_ghost)]
use ckc_spec::check::byte_rows;
use std::collections::HashMap;
use vstd::assert_seqs_equal;
use vstd::assert_sets_equal;
use vstd::prelude::*;

verus! {

broadcast use vstd::std_specs::hash::group_hash_axioms;

pub open spec fn code_step(h: u64, b: u8) -> u64 {
    h.wrapping_mul(1099511628211).wrapping_add(b as u64)
}

pub open spec fn code(s: Seq<u8>) -> u64 {
    s.fold_left(14695981039346656037u64, |h: u64, b: u8| code_step(h, b))
}

pub fn step(h: u64, b: u8) -> (r: u64)
    ensures
        r == code_step(h, b),
{
    let p = h.wrapping_mul(1099511628211);
    p.wrapping_add(b as u64)
}

pub proof fn code_push(s: Seq<u8>, b: u8)
    ensures
        code(s.push(b)) == code_step(code(s), b),
{
    assert_seqs_equal!(s.push(b).drop_last() == s);
    reveal_with_fuel(Seq::fold_left, 2);
}

pub fn bucket_code(s: &[u8]) -> (h: u64)
    ensures
        h == code(s@),
{
    hide(code_step);
    let mut h = 14695981039346656037u64;
    let mut i = 0usize;
    proof {
        assert_seqs_equal!(s@.take(0) == Seq::<u8>::empty());
    }
    while i < s.len()
        invariant
            i <= s@.len(),
            h == code(s@.take(i as int)),
        decreases s.len() - i,
    {
        let next = step(h, s[i]);
        proof {
            code_push(s@.take(i as int), s@[i as int]);
            assert_seqs_equal!(s@.take(i as int + 1) == s@.take(i as int).push(s@[i as int]));
        }
        h = next;
        i += 1;
    }
    proof {
        assert_seqs_equal!(s@.take(i as int) == s@);
    }
    h
}

// vstd models integer keys; each shared bucket compares complete byte strings.
pub struct ByteSet {
    pub buckets: HashMap<u64, Vec<Vec<u8>>>,
    pub size: usize,
}

pub closed spec fn members(m: Map<u64, Vec<Vec<u8>>>) -> Set<Seq<u8>> {
    m.dom().map_flatten_by(
        |h: u64| byte_rows(m[h]@).to_set().filter(|s: Seq<u8>| code(s) == h),
        |s: Seq<u8>| code(s),
    )
}

pub proof fn members_contains(m: Map<u64, Vec<Vec<u8>>>, value: Seq<u8>)
    ensures
        members(m).contains(value) == (m.dom().contains(code(value)) && byte_rows(
            m[code(value)]@,
        ).contains(value)),
{
    hide(code);
    hide(code_step);
    let fwd = |h: u64| byte_rows(m[h]@).to_set().filter(|s: Seq<u8>| code(s) == h);
    let rev = |s: Seq<u8>| code(s);
    assert forall|h: u64, s: Seq<u8>|
        m.dom().contains(h) && #[trigger] fwd(h).contains(s) implies rev(s) == h by {};
    m.dom().lemma_map_flatten_by_contains(fwd, rev, value);
    reveal(members);
}

impl View for ByteSet {
    type V = Set<Seq<u8>>;

    open spec fn view(&self) -> Self::V {
        members(self.buckets@)
    }
}

pub open spec fn valid(s: &ByteSet) -> bool {
    s@.finite() && s.size as nat == s@.len()
}

pub fn empty() -> (s: ByteSet)
    ensures
        valid(&s),
        s@ == Set::<Seq<u8>>::empty(),
        s.size == 0,
{
    let s = ByteSet { buckets: HashMap::new(), size: 0 };
    proof {
        assert forall|x: Seq<u8>| !#[trigger] members(s.buckets@).contains(x) by {
            members_contains(s.buckets@, x);
        }
        assert_sets_equal!(s@ == Set::<Seq<u8>>::empty());
    }
    s
}

pub fn has(s: &ByteSet, value: &[u8]) -> (r: bool)
    ensures
        r == s@.contains(value@),
{
    let h = bucket_code(value);
    let r = match s.buckets.get(&h) {
        Some(bucket) => contains(bucket, value),
        None => false,
    };
    proof {
        members_contains(s.buckets@, value@);
    }
    r
}

pub fn copy_rows(input: &Vec<Vec<u8>>) -> (out: Vec<Vec<u8>>)
    ensures
        out@.len() == input@.len(),
        byte_rows(out@) == byte_rows(input@),
{
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < input.len()
        invariant
            i <= input@.len(),
            out@.len() == i,
            byte_rows(out@) == byte_rows(input@).take(i as int),
        decreases input.len() - i,
    {
        let row = copy(&input[i]);
        proof {
            byte_rows_push(out@, row);
            assert_seqs_equal!(byte_rows(input@).take(i as int + 1) == byte_rows(input@).take(i as int).push(row@));
        }
        out.push(row);
        i += 1;
    }
    proof {
        assert_seqs_equal!(byte_rows(input@).take(i as int) == byte_rows(input@));
    }
    out
}

pub fn insert(s: &mut ByteSet, value: &[u8]) -> (added: bool)
    requires
        valid(old(s)),
        old(s)@.len() < usize::MAX,
    ensures
        valid(final(s)),
        final(s)@ == old(s)@.insert(value@),
        added == !old(s)@.contains(value@),
        final(s)@.len() == old(s)@.len() + if added {
            1nat
        } else {
            0nat
        },
        final(s).size == old(s).size + if added {
            1usize
        } else {
            0usize
        },
{
    hide(code);
    hide(code_step);
    let ghost before = s.buckets@;
    let ghost old_set = s@;
    proof {
        assert(old_set == members(before));
    }
    if has(s, value) {
        proof {
            assert_sets_equal!(s@.insert(value@) == s@);
        }
        return false;
    }
    let h = bucket_code(value);
    let mut bucket = match s.buckets.get(&h) {
        Some(v) => copy_rows(v),
        None => Vec::new(),
    };
    let ghost prior = byte_rows(bucket@);
    proof {
        assert(prior == if before.dom().contains(h) {
            byte_rows(before[h]@)
        } else {
            Seq::<Seq<u8>>::empty()
        });
    }
    let row = copy(value);
    proof {
        byte_rows_push(bucket@, row);
    }
    bucket.push(row);
    let ghost after_bucket = bucket@;
    let ignored = s.buckets.insert(h, bucket);
    proof {
        let after = s.buckets@;
        assert forall|x: Seq<u8>| #[trigger]
            members(after).contains(x) <==> old_set.insert(value@).contains(x) by {
            members_contains(after, x);
            members_contains(before, x);
            assert(old_set.insert(value@).contains(x) == (x == value@ || old_set.contains(x)));
            vstd::seq_lib::lemma_seq_contains_after_push(prior, value@, x);
            assert(prior.push(value@).contains(x) == (x == value@ || prior.contains(x)));
            if code(x) == h {
                assert(after.dom().contains(h));
                assert(byte_rows(after[h]@) == prior.push(value@));
                assert(members(after).contains(x) == prior.push(value@).contains(x));
                assert(members(before).contains(x) == prior.contains(x));
            } else {
                assert(x != value@);
                assert(after.dom().contains(code(x)) == before.dom().contains(code(x)));
                if before.dom().contains(code(x)) {
                    assert(after[code(x)] == before[code(x)]);
                }
                assert(members(after).contains(x) == members(before).contains(x));
            }
            assert(members(after).contains(x) == old_set.insert(value@).contains(x));
        }
        assert_sets_equal!(members(after) == old_set.insert(value@));
        assert(!old_set.contains(value@));
        assert(old_set.insert(value@).len() == old_set.len() + 1);
    }
    s.size += 1;
    true
}

pub fn len(s: &ByteSet) -> (n: usize)
    requires
        valid(s),
    ensures
        n as nat == s@.len(),
{
    s.size
}

} // verus!
