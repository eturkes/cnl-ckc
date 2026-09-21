use crate::k5_utf8;
use ckc_spec::ui as u;
use vstd::prelude::*;
use vstd::string::StringSliceAdditionalSpecFns;
verus! {

pub fn chars(bytes: &[u8]) -> (out: Vec<char>)
    ensures
        out@ == u::chars(bytes@),
{
    if !k5_utf8::valid(bytes) {
        return Vec::new();
    }
    // R89: valid() proves the pinned valid_utf8 precondition.

    let text = unsafe { str::from_utf8_unchecked(bytes) };
    proof {
        vstd::utf8::encode_utf8_decode_utf8(text@);
        assert(text.spec_bytes() == bytes@);
    }
    text.chars().collect()
}

} // verus!
