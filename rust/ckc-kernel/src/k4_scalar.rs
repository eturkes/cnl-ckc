use crate::k4_bytes::range;
use ckc_spec::check::*;
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub fn digit(b: u8) -> (r: bool)
    ensures
        r == ckc_spec::v1text::is_digit_b(b),
{
    b >= 0x30 && b <= 0x39
}

pub fn docid(s: &[u8]) -> (r: bool)
    ensures
        r == docid_ok(s@),
{
    if s.len() == 0 || s.len() > 250 {
        return false;
    }
    if s[0] == 0x2d {
        return false;
    }
    let mut i = 0usize;
    while i < s.len()
        invariant
            0 < s@.len() <= 250,
            s@[0] != 0x2d,
            i <= s@.len(),
            forall|j: int|
                0 <= j < i ==> (ckc_spec::v1text::is_lower_b(s@[j]) || ckc_spec::v1text::is_digit_b(
                    s@[j],
                ) || s@[j] == 0x2d),
        decreases s.len() - i,
    {
        let b = s[i];
        if !((b >= 0x61 && b <= 0x7a) || digit(b) || b == 0x2d) {
            return false;
        }
        i += 1;
    }
    true
}

pub fn hex(s: &[u8], n: usize) -> (r: bool)
    ensures
        r == (s@.len() == n && ckc_spec::v1text::all_in(
            s@,
            |b: u8| ckc_spec::v1text::is_hex_lower_b(b),
        )),
{
    if s.len() != n {
        return false;
    }
    let mut i = 0usize;
    while i < s.len()
        invariant
            s@.len() == n,
            i <= s@.len(),
            forall|j: int| 0 <= j < i ==> ckc_spec::v1text::is_hex_lower_b(s@[j]),
        decreases s.len() - i,
    {
        let b = s[i];
        if !(digit(b) || (b >= 0x61 && b <= 0x66)) {
            return false;
        }
        i += 1;
    }
    true
}

pub fn clean(s: &[u8]) -> (r: bool)
    ensures
        r == text_clean(s@),
{
    let mut i = 0usize;
    while i < s.len()
        invariant
            i <= s@.len(),
            forall|j: int| 0 <= j < i ==> s@[j] >= 0x20 && s@[j] != 0x7f,
        decreases s.len() - i,
    {
        if s[i] < 0x20 || s[i] == 0x7f {
            return false;
        }
        i += 1;
    }
    true
}

pub fn n_plus(n: usize, plus: u8) -> (r: Vec<u8>)
    requires
        plus <= 9,
    ensures
        r@ == nat_bytes(n as nat + plus as nat),
    decreases n,
{
    let d = (n % 10) as u8 + plus;
    let q = n / 10;
    if q == 0 && d < 10 {
        let mut r = Vec::new();
        r.push(0x30 + d);
        proof {
            reveal_with_fuel(ckc_spec::v1text::udec_bytes, 2);
            reveal(ckc_spec::v1text::digit_byte);
        }
        r
    } else {
        let mut r = n_plus(q, d / 10);
        r.push(0x30 + d % 10);
        proof {
            assert((n as nat + plus as nat) / 10 == q as nat + (d / 10) as nat);
            assert((n as nat + plus as nat) % 10 == (d % 10) as nat);
            reveal_with_fuel(ckc_spec::v1text::udec_bytes, 2);
            reveal(ckc_spec::v1text::digit_byte);
        }
        r
    }
}

pub fn number(n: usize) -> (r: Vec<u8>)
    ensures
        r@ == nat_bytes(n as nat),
{
    n_plus(n, 0)
}

pub fn decimal(s: &[u8]) -> (n: usize)
    requires
        s@.len() <= 4,
        ckc_spec::v1text::all_in(s@, |b: u8| ckc_spec::v1text::is_digit_b(b)),
    ensures
        n as nat == dec_of(s@),
        n <= 9999,
{
    let mut n = 0usize;
    let mut i = 0usize;
    proof {
        assert_seqs_equal!(s@.take(0) == Seq::<u8>::empty());
    }
    while i < s.len()
        invariant
            i <= s@.len() <= 4,
            ckc_spec::v1text::all_in(s@, |b: u8| ckc_spec::v1text::is_digit_b(b)),
            n as nat == dec_of(s@.take(i as int)),
            i == 0 ==> n == 0,
            i == 1 ==> n <= 9,
            i == 2 ==> n <= 99,
            i == 3 ==> n <= 999,
            i == 4 ==> n <= 9999,
        decreases s.len() - i,
    {
        let d = s[i] - 0x30;
        proof {
            assert_seqs_equal!(s@.take(i as int + 1).drop_last() == s@.take(i as int));
            reveal_with_fuel(dec_of, 2);
        }
        n = n * 10 + d as usize;
        i += 1;
    }
    proof {
        assert_seqs_equal!(s@.take(s@.len() as int) == s@);
    }
    n
}

pub fn date(s: &[u8]) -> (r: bool)
    ensures
        r == date_ok(s@),
{
    if s.len() != 20 {
        return false;
    }
    if s[4] != 0x2d || s[7] != 0x2d || s[10] != 0x54 || s[13] != 0x3a || s[16] != 0x3a || s[19]
        != 0x5a {
        return false;
    }
    let mut i = 0usize;
    while i < 20
        invariant
            s@.len() == 20,
            i <= 20,
            s@[4] == 0x2d,
            s@[7] == 0x2d,
            s@[10] == 0x54,
            s@[13] == 0x3a,
            s@[16] == 0x3a,
            s@[19] == 0x5a,
            forall|j: int|
                0 <= j < i && j != 4 && j != 7 && j != 10 && j != 13 && j != 16 && j != 19
                    ==> ckc_spec::v1text::is_digit_b(s@[j]),
        decreases 20 - i,
    {
        if i != 4 && i != 7 && i != 10 && i != 13 && i != 16 && i != 19 && !digit(s[i]) {
            return false;
        }
        i += 1;
    }
    let y = decimal(&range(s, 0, 4));
    let mo = decimal(&range(s, 5, 7));
    let d = decimal(&range(s, 8, 10));
    let h = decimal(&range(s, 11, 13));
    let mi = decimal(&range(s, 14, 16));
    let se = decimal(&range(s, 17, 19));
    let leap_year = (y.is_multiple_of(4) && !y.is_multiple_of(100)) || y.is_multiple_of(400);
    let days = if mo == 2 {
        if leap_year {
            29usize
        } else {
            28usize
        }
    } else if mo == 4 || mo == 6 || mo == 9 || mo == 11 {
        30usize
    } else {
        31usize
    };
    y >= 1 && mo >= 1 && mo <= 12 && d >= 1 && d <= days && h <= 23 && mi <= 59 && se <= 59
}

} // verus!
