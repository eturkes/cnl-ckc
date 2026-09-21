use crate::k5_bytes as b;
use ckc_spec::ui as u;
use vstd::prelude::*;
use vstd::slice::slice_to_vec;
verus! {

pub fn document(path: &[u8]) -> (out: Option<(Vec<u8>, Vec<u8>)>)
    ensures
        match out {
            Some((g, d)) => u::doc_route(path@) == Some((g@, d@)),
            None => u::doc_route(path@) is None,
        },
{
    let prefix = b::literal("/g/");
    proof {
        reveal_strlit("/g/");
        reveal_strlit(".html");
        reveal_with_fuel(vstd::utf8::encode_utf8, 6);
        assert(u::lit("/g/"@).len() == 3);
        assert(u::lit(".html"@).len() == 5);
    }
    if !b::starts(path, &prefix) {
        return None;
    }
    let xs = b::split(&path[3..path.len()], 47);
    let doc = b::literal("doc");
    let suffix = b::literal(".html");
    if xs.len() == 3 && xs[0].len() > 0 && b::equal(&xs[1], &doc) && b::ends(&xs[2], &suffix)
        && xs[2].len() > 5 {
        Some((slice_to_vec(&xs[0]), slice_to_vec(&xs[2][0..xs[2].len() - 5])))
    } else {
        None
    }
}

} // verus!
