use crate::k4_bytes::{eq, range, starts_with};
use crate::k4_scalar::digit;
use ckc_spec::check::*;
use vstd::assert_seqs_equal;
use vstd::prelude::*;
use vstd::slice::slice_subrange;

verus! {

pub fn sub(s: &[u8], p: &[u8], start: usize) -> (r: usize)
    requires
        start <= s@.len(),
    ensures
        start <= r <= s@.len(),
        r as nat == first_sub(s@, p@, start as nat),
        r < s@.len() ==> r + p@.len() <= s@.len() && s@.subrange(
            r as int,
            r as int + p@.len() as int,
        ) == p@,
{
    let mut i = start;
    while i < s.len()
        invariant
            start <= i <= s@.len(),
            first_sub(s@, p@, start as nat) == first_sub(s@, p@, i as nat),
        decreases s.len() - i,
    {
        if p.len() > s.len() - i {
            proof {
                reveal_with_fuel(first_sub, 2);
            }
            return s.len();
        }
        let hit = eq(slice_subrange(s, i, i + p.len()), p);
        proof {
            reveal_with_fuel(first_sub, 2);
        }
        if hit {
            return i;
        }
        i += 1;
    }
    proof {
        reveal_with_fuel(first_sub, 2);
    }
    i
}

pub fn digits(s: &[u8], start: usize) -> (i: usize)
    requires
        start <= s@.len(),
    ensures
        start <= i <= s@.len(),
        i as nat == lead_digits(s@, start as nat),
        forall|j: int| start <= j < i ==> ckc_spec::v1text::is_digit_b(s@[j]),
{
    let mut i = start;
    while i < s.len()
        invariant
            start <= i <= s@.len(),
            lead_digits(s@, start as nat) == lead_digits(s@, i as nat),
            forall|j: int| start <= j < i ==> ckc_spec::v1text::is_digit_b(s@[j]),
        decreases s.len() - i,
    {
        let d = digit(s[i]);
        proof {
            reveal_with_fuel(lead_digits, 2);
        }
        if !d {
            return i;
        }
        i += 1;
    }
    proof {
        reveal_with_fuel(lead_digits, 2);
    }
    i
}

pub open spec fn census_view(c: Option<Vec<u8>>) -> Option<nat> {
    match c {
        Some(ds) => Some(dec_of(ds@)),
        None => None,
    }
}

pub fn census(text: &[u8]) -> (r: Option<Vec<u8>>)
    ensures
        census_view(r) == census_of(text@),
        r matches Some(ds) ==> ds@.len() > 0 && ckc_spec::v1text::all_in(
            ds@,
            |b: u8| ckc_spec::v1text::is_digit_b(b),
        ),
{
    let p: &[u8] = b"identify the ";
    let q: &[u8] = b" payloads below";
    proof {
        reveal_byteslit(b"identify the ");
        reveal_strlit("identify the ");
        reveal_byteslit(b" payloads below");
        reveal_strlit(" payloads below");
        reveal(ckc_spec::v1text::ascii);
        assert(p@ == ckc_spec::v1text::ascii("identify the "@));
        assert(q@ == ckc_spec::v1text::ascii(" payloads below"@));
    }
    let mut k = 0usize;
    while k < text.len()
        invariant
            k <= text@.len(),
            census_of(text@) == census_at(text@, k as nat),
            p@ == ckc_spec::v1text::ascii("identify the "@),
            q@ == ckc_spec::v1text::ascii(" payloads below"@),
        decreases text.len() - k,
    {
        let hit = sub(text, p, k);
        if hit >= text.len() {
            proof {
                reveal_with_fuel(census_at, 2);
            }
            return None;
        }
        let after = slice_subrange(text, hit + p.len(), text.len());
        let n = digits(after, 0);
        let rest = slice_subrange(after, n, after.len());
        let matched = n > 0 && starts_with(rest, q);
        proof {
            reveal_with_fuel(census_at, 2);
        }
        if matched {
            return Some(range(after, 0, n));
        }
        k += 1;
    }
    proof {
        reveal_with_fuel(census_at, 2);
    }
    None
}

} // verus!
