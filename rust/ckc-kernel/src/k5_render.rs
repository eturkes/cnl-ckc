use crate::k5_bytes as b;
use ckc_spec::ui::{self as u, EPage, EPiece};
use vstd::prelude::*;
use vstd::slice::slice_to_vec;
verus! {

pub proof fn render_cons(p: u::Html)
    requires
        p.len() > 0,
    ensures
        u::render_page(p) == u::emit_piece(p[0]) + u::render_page(p.drop_first()),
{
    assert(p.map_values(|x: u::Piece| u::emit_piece(x)).drop_first() =~= p.drop_first().map_values(
        |x: u::Piece| u::emit_piece(x),
    ));
}

pub fn piece(p: &EPiece) -> (out: Vec<u8>)
    ensures
        out@ == u::emit_piece(p@),
{
    match p {
        EPiece::Fixed(x) => slice_to_vec(x),
        EPiece::Text(x) => b::escape(x, false),
        EPiece::Attr(x) => b::escape(x, true),
        EPiece::Decimal(n) => b::nat_bytes(*n),
        EPiece::Comment(x) => crate::k5_utf8::comment(x),
    }
}

pub fn page(p: &EPage) -> (out: Vec<u8>)
    ensures
        out@ == u::render_page(p@),
{
    let mut out = Vec::new();
    let mut i = 0;
    proof {
        assert(p@.skip(0) =~= p@);
    }
    while i < p.parts.len()
        invariant
            i <= p.parts.len(),
            out@ + u::render_page(p@.skip(i as int)) == u::render_page(p@),
        decreases p.parts.len() - i,
    {
        proof {
            render_cons(p@.skip(i as int));
        }
        let x = piece(&p.parts[i]);
        b::append(&mut out, &x);
        proof {
            assert(p@.skip(i as int).drop_first() =~= p@.skip(i as int + 1));
        }
        i += 1;
    }
    out
}

} // verus!
