use crate::{k5_bytes as b, k5_utf8 as utf};
use ckc_spec::ui as u;
use vstd::prelude::*;
use vstd::utf8::*;
verus! {

pub open spec fn cvs(xs: Seq<u32>) -> Seq<char> {
    xs.map_values(|x: u32| x as char)
}

pub open spec fn scalar_list(xs: Seq<u32>) -> bool {
    forall|i: int| 0 <= i < xs.len() ==> is_scalar(#[trigger] xs[i])
}

pub fn decode(s: &[u8]) -> (out: Vec<u32>)
    ensures
        cvs(out@) == u::chars(s@),
        scalar_list(out@),
{
    if !utf::valid(s) {
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
            scalar_list(out@),
            cvs(out@) + decode_utf8(s@.skip(i as int)) == u::chars(s@),
        decreases s.len() - i,
    {
        let got = utf::first(&s[i..s.len()]);
        let (c, n) = got.unwrap();
        let ghost before = out@;
        out.push(c);
        proof {
            assert(cvs(out@) =~= cvs(before).push(c as char));
            assert(pop_first_scalar(s@.skip(i as int)) =~= s@.skip(i as int + n as int));
        }
        i += n;
    }
    out
}

pub fn scalar_bytes(c: u32) -> (out: Vec<u8>)
    requires
        is_scalar(c),
    ensures
        out@ == encode_scalar(c),
{
    let mut out = Vec::new();
    if c <= 0x7f {
        out.push((c & 0x7f) as u8);
    } else if c <= 0x7ff {
        out.push(0xc0 | ((c >> 6) & 0x1f) as u8);
        out.push(0x80 | (c & 0x3f) as u8);
    } else if c <= 0xffff {
        out.push(0xe0 | ((c >> 12) & 0x0f) as u8);
        out.push(0x80 | ((c >> 6) & 0x3f) as u8);
        out.push(0x80 | (c & 0x3f) as u8);
    } else {
        out.push(0xf0 | ((c >> 18) & 0x07) as u8);
        out.push(0x80 | ((c >> 12) & 0x3f) as u8);
        out.push(0x80 | ((c >> 6) & 0x3f) as u8);
        out.push(0x80 | (c & 0x3f) as u8);
    }
    out
}

pub fn encode(s: &[u32]) -> (out: Vec<u8>)
    requires
        scalar_list(s@),
    ensures
        out@ == encode_utf8(cvs(s@)),
{
    let mut out = Vec::new();
    let mut i = 0;
    proof {
        assert(cvs(s@).skip(0) =~= cvs(s@));
    }
    while i < s.len()
        invariant
            i <= s.len(),
            scalar_list(s@),
            out@ + encode_utf8(cvs(s@).skip(i as int)) == encode_utf8(cvs(s@)),
        decreases s.len() - i,
    {
        let c = s[i];
        let part = scalar_bytes(c);
        b::append(&mut out, &part);
        proof {
            assert((c as char) as u32 == c);
            assert(cvs(s@).skip(i as int).drop_first() =~= cvs(s@).skip(i as int + 1));
        }
        i += 1;
    }
    out
}

} // verus!
