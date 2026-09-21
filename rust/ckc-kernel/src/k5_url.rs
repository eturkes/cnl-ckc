use crate::{k5_copy, k5_highlight as bytes};
use ckc_spec::ui as u;
use vstd::prelude::*;
verus! {

pub fn segment(s: &[u8]) -> (out: Vec<u8>)
    ensures
        out@ == u::url_seg(s@),
{
    let mut out = Vec::new();
    let mut i = 0;
    proof {
        assert(s@.skip(0) =~= s@);
    }
    while i < s.len()
        invariant
            i <= s.len(),
            out@ + u::url_seg(s@.skip(i as int)) == u::url_seg(s@),
        decreases s.len() - i,
    {
        let x = s[i];
        if bytes::token_byte(x) || x == 95 || x == 45 || x == 46 || x == 126 {
            out.push(x);
        } else {
            out.push(37);
            out.push(k5_copy::hex_digit(x as u32 / 16));
            out.push(k5_copy::hex_digit(x as u32 % 16));
        }
        proof {
            assert(s@.skip(i as int).drop_first() =~= s@.skip(i as int + 1));
        }
        i += 1;
    }
    out
}

} // verus!
