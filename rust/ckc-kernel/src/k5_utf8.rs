use ckc_spec::ui as u;
use vstd::prelude::*;
use vstd::utf8::*;

verus! {

pub fn first(s: &[u8]) -> (r: Option<(u32, usize)>)
    ensures
        r.is_some() == valid_first_scalar(s@),
        r matches Some((c, n)) ==> c == decode_first_scalar(s@) && n == length_of_first_scalar(s@)
            && 0 < n <= s.len() && is_scalar(c),
{
    if s.len() == 0 {
        return None;
    }
    let b = s[0];
    let (c, n): (u32, usize) = if b <= 0x7f {
        proof {
            assert((b & 0x7fu8) == b) by (bit_vector)
                requires
                    b <= 0x7f,
            ;
        }
        (b as u32, 1)
    } else if 0xc0 <= b && b <= 0xdf {
        if s.len() < 2 || s[1] < 0x80 || s[1] > 0xbf {
            return None;
        }
        let b1 = s[1];
        proof {
            assert((((b & 0x1fu8) as u32) << 6 | ((b1 & 0x3fu8) as u32)) <= 0x7ffu32)
                by (bit_vector);
        }
        (((b & 0x1f) as u32) << 6 | ((b1 & 0x3f) as u32), 2)
    } else if 0xe0 <= b && b <= 0xef {
        if s.len() < 3 || s[1] < 0x80 || s[1] > 0xbf || s[2] < 0x80 || s[2] > 0xbf {
            return None;
        }
        let b1 = s[1];
        let b2 = s[2];
        proof {
            assert((((b & 0x0fu8) as u32) << 12 | ((b1 & 0x3fu8) as u32) << 6 | ((b2
                & 0x3fu8) as u32)) <= 0xffffu32) by (bit_vector);
        }
        (((b & 0x0f) as u32) << 12 | ((b1 & 0x3f) as u32) << 6 | ((b2 & 0x3f) as u32), 3)
    } else if 0xf0 <= b && b <= 0xf7 {
        if s.len() < 4 || s[1] < 0x80 || s[1] > 0xbf || s[2] < 0x80 || s[2] > 0xbf || s[3] < 0x80
            || s[3] > 0xbf {
            return None;
        }
        (
            ((b & 0x07) as u32) << 18 | ((s[1] & 0x3f) as u32) << 12 | ((s[2] & 0x3f) as u32) << 6
                | ((s[3] & 0x3f) as u32),
            4,
        )
    } else {
        return None;
    };
    if (n == 2 && c < 0x80) || (n == 3 && c < 0x800) || (n == 4 && (c < 0x10000 || c > 0x10ffff))
        || (0xd800 <= c && c <= 0xdfff) {
        None
    } else {
        Some((c, n))
    }
}

pub fn valid(s: &[u8]) -> (yes: bool)
    ensures
        yes == valid_utf8(s@),
{
    let mut i = 0;
    proof {
        assert(s@.skip(0) =~= s@);
    }
    while i < s.len()
        invariant
            i <= s.len(),
            valid_utf8(s@) == valid_utf8(s@.skip(i as int)),
        decreases s.len() - i,
    {
        match first(&s[i..s.len()]) {
            None => {
                return false;
            },
            Some((_, n)) => {
                proof {
                    assert(pop_first_scalar(s@.skip(i as int)) =~= s@.skip(i as int + n as int));
                }
                i += n;
            },
        }
    }
    true
}

pub fn comment(s: &[u8]) -> (out: Vec<u8>)
    ensures
        out@ == u::comment_safe(s@),
{
    if !valid(s) {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut i = 0;
    proof {
        assert(s@.skip(0) =~= s@);
    }
    while i < s.len()
        invariant
            i <= s.len(),
            valid_utf8(s@),
            valid_utf8(s@.skip(i as int)),
            out@ + u::comment_safe(s@.skip(i as int)) == u::comment_safe(s@),
        decreases s.len() - i,
    {
        let got = first(&s[i..s.len()]);
        let (c, n) = got.unwrap();
        let x = if (48 <= c && c <= 57) || (65 <= c && c <= 90) || (97 <= c && c <= 122) || c == 32
            || c == 58 || c == 95 || c == 46 {
            c as u8
        } else {
            95u8
        };
        out.push(x);
        proof {
            let tail = s@.skip(i as int);
            assert(is_scalar(c));
            assert(pop_first_scalar(tail) =~= s@.skip(i as int + n as int));
            assert(decode_utf8(tail) =~= seq![c as char] + decode_utf8(pop_first_scalar(tail)));
            assert((c as char) as u32 == c);
        }
        i += n;
    }
    out
}

} // verus!
