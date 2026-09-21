use crate::k4_bytes::{copy, eq, range};
use crate::k4_scalar::number;
use ckc_spec::check::*;
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub proof fn zero_front(s: Seq<u8>)
    requires
        ckc_spec::v1text::all_in(s, |b: u8| ckc_spec::v1text::is_digit_b(b)),
    ensures
        dec_of(seq![0x30u8] + s) == dec_of(s),
    decreases s.len(),
{
    reveal_with_fuel(dec_of, 3);
    if s.len() > 0 {
        zero_front(s.drop_last());
        assert_seqs_equal!((seq![0x30u8] + s).drop_last() == seq![0x30u8] + s.drop_last());
    }
}

pub proof fn canonical_print(s: Seq<u8>)
    requires
        s.len() > 0,
        ckc_spec::v1text::all_in(s, |b: u8| ckc_spec::v1text::is_digit_b(b)),
        s.len() == 1 || s[0] != 0x30,
    ensures
        nat_bytes(dec_of(s)) == s,
    decreases s.len(),
{
    reveal_with_fuel(dec_of, 2);
    if s.len() == 1 {
        reveal_with_fuel(ckc_spec::v1text::udec_bytes, 2);
        reveal(ckc_spec::v1text::digit_byte);
        assert_seqs_equal!(s == seq![s[0]]);
    } else {
        let p = s.drop_last();
        canonical_print(p);
        if dec_of(p) == 0 {
            reveal_with_fuel(ckc_spec::v1text::udec_bytes, 2);
            reveal(ckc_spec::v1text::digit_byte);
            assert(p == seq![0x30u8]);
            assert(false);
        }
        let n = dec_of(s);
        let d = (s.last() - 0x30) as nat;
        assert(n == dec_of(p) * 10 + d);
        assert(n / 10 == dec_of(p));
        assert(n % 10 == d);
        reveal_with_fuel(ckc_spec::v1text::udec_bytes, 2);
        reveal(ckc_spec::v1text::digit_byte);
        assert_seqs_equal!(s == p.push(s.last()));
    }
}

pub proof fn decimal_print_inverse(n: nat)
    ensures
        dec_of(nat_bytes(n)) == n,
        nat_bytes(n).len() > 0,
        ckc_spec::v1text::all_in(nat_bytes(n), |b: u8| ckc_spec::v1text::is_digit_b(b)),
    decreases n,
{
    reveal_with_fuel(ckc_spec::v1text::udec_bytes, 2);
    reveal(ckc_spec::v1text::digit_byte);
    if n < 10 {
        reveal_with_fuel(dec_of, 2);
    } else {
        decimal_print_inverse(n / 10);
        let p = nat_bytes(n / 10);
        let last = ckc_spec::v1text::digit_byte((n % 10) as int);
        assert(nat_bytes(n) == p.push(last));
        assert_seqs_equal!(nat_bytes(n).drop_last() == p);
        reveal_with_fuel(dec_of, 2);
    }
}

pub fn normalized(s: &[u8]) -> (r: Vec<u8>)
    requires
        s@.len() > 0,
        ckc_spec::v1text::all_in(s@, |b: u8| ckc_spec::v1text::is_digit_b(b)),
    ensures
        r@ == nat_bytes(dec_of(s@)),
{
    let mut i = 0usize;
    proof {
        assert_seqs_equal!(s@.skip(0) == s@);
    }
    while i < s.len() && s[i] == 0x30
        invariant
            i <= s@.len(),
            ckc_spec::v1text::all_in(s@, |b: u8| ckc_spec::v1text::is_digit_b(b)),
            dec_of(s@) == dec_of(s@.skip(i as int)),
        decreases s.len() - i,
    {
        proof {
            let rest = s@.skip(i as int + 1);
            assert_seqs_equal!(s@.skip(i as int) == seq![0x30u8] + rest);
            zero_front(rest);
        }
        i += 1;
    }
    if i == s.len() {
        proof {
            reveal_with_fuel(dec_of, 2);
        }
        number(0)
    } else {
        let r = range(s, i, s.len());
        proof {
            canonical_print(r@);
        }
        r
    }
}

pub fn equals_count(digits: &[u8], count: usize) -> (r: bool)
    requires
        digits@.len() > 0,
        ckc_spec::v1text::all_in(digits@, |b: u8| ckc_spec::v1text::is_digit_b(b)),
    ensures
        r == (dec_of(digits@) == count as nat),
{
    let a = normalized(digits);
    let b = number(count);
    let r = eq(&a, &b);
    proof {
        if r {
            decimal_print_inverse(dec_of(digits@));
            decimal_print_inverse(count as nat);
        }
    }
    r
}

} // verus!
