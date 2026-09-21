use crate::k4_bytes::{append, copy};
use ckc_spec::check::*;
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub fn escape(d: &[u8]) -> (out: Vec<u8>)
    ensures
        out@ == escape_detail(d@),
{
    let mut out = Vec::new();
    let mut i = 0usize;
    proof {
        assert_seqs_equal!(d@.skip(0) == d@);
    }
    while i < d.len()
        invariant
            i <= d@.len(),
            escape_detail(d@) == out@ + escape_detail(d@.skip(i as int)),
        decreases d.len() - i,
    {
        let b = d[i];
        proof {
            reveal_with_fuel(escape_detail, 2);
            assert_seqs_equal!(d@.skip(i as int).drop_first() == d@.skip(i as int + 1));
        }
        if b == 0x0a {
            out.push(0x5c);
            out.push(0x6e);
        } else if b == 0x0d {
            out.push(0x5c);
            out.push(0x72);
        } else {
            out.push(b);
        }
        i += 1;
    }
    proof {
        reveal_with_fuel(escape_detail, 2);
    }
    out
}

pub fn verdict(v: &EVerdict) -> (r: ERendered)
    ensures
        r@ == render(v@),
{
    match v {
        EVerdict::Ok(m) => ERendered { rc: 0, out: copy(m) },
        EVerdict::Fail(c, d) => {
            let mut out = copy(b"goal: ");
            append(&mut out, c);
            append(&mut out, b": ");
            append(&mut out, &escape(d));
            out.push(0x0a);
            proof {
                reveal_byteslit(b"goal: ");
                reveal_strlit("goal: ");
                reveal_byteslit(b": ");
                reveal_strlit(": ");
                reveal(ckc_spec::v1text::ascii);
                reveal(render);
            }
            ERendered { rc: 1, out }
        },
    }
}

} // verus!
