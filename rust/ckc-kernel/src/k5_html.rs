use crate::k5_bytes as b;
use ckc_spec::ui::{self as u, EPage, EPiece};
use vstd::prelude::*;
use vstd::slice::slice_to_vec;
verus! {

pub open spec fn pages(xs: Seq<EPage>) -> Seq<u::Html> {
    xs.map_values(|x: EPage| x@)
}

pub fn empty() -> (out: EPage)
    ensures
        out@ == Seq::<u::Piece>::empty(),
{
    EPage { parts: Vec::new() }
}

pub fn one(p: EPiece) -> (out: EPage)
    ensures
        out@ == seq![p@],
{
    let mut parts = Vec::new();
    parts.push(p);
    EPage { parts }
}

pub fn fixed(s: &str) -> (out: EPage)
    ensures
        out@ == u::fixed(s@),
{
    one(EPiece::Fixed(b::literal(s)))
}

pub fn fixed_bytes(s: &[u8]) -> (out: EPage)
    ensures
        out@ == u::fixed_bytes(s@),
{
    one(EPiece::Fixed(slice_to_vec(s)))
}

pub fn text(s: &[u8]) -> (out: EPage)
    ensures
        out@ == u::text(s@),
{
    one(EPiece::Text(slice_to_vec(s)))
}

pub fn attr(s: &[u8]) -> (out: EPage)
    ensures
        out@ == u::attr(s@),
{
    one(EPiece::Attr(slice_to_vec(s)))
}

pub fn number(n: usize) -> (out: EPage)
    ensures
        out@ == u::number(n as nat),
{
    one(EPiece::Decimal(n as u64))
}

pub fn copy_piece(p: &EPiece) -> (out: EPiece)
    ensures
        out@ == p@,
{
    match p {
        EPiece::Fixed(s) => EPiece::Fixed(slice_to_vec(s)),
        EPiece::Text(s) => EPiece::Text(slice_to_vec(s)),
        EPiece::Attr(s) => EPiece::Attr(slice_to_vec(s)),
        EPiece::Decimal(n) => EPiece::Decimal(*n),
        EPiece::Comment(s) => EPiece::Comment(slice_to_vec(s)),
    }
}

pub fn copy(p: &EPage) -> (out: EPage)
    ensures
        out@ == p@,
{
    let mut parts = Vec::new();
    let mut i = 0;
    while i < p.parts.len()
        invariant
            i <= p.parts.len(),
            parts@.map_values(|x: EPiece| x@) == p@.take(i as int),
        decreases p.parts.len() - i,
    {
        parts.push(copy_piece(&p.parts[i]));
        proof {
            assert(p@.take(i as int + 1) =~= p@.take(i as int).push(p@[i as int]));
        }
        i += 1;
    }
    proof {
        assert(p@.take(i as int) =~= p@);
    }
    EPage { parts }
}

pub fn append(out: &mut EPage, mut rhs: EPage)
    ensures
        final(out)@ == old(out)@ + rhs@,
{
    let ghost before = out@;
    let ghost other = rhs@;
    out.parts.append(&mut rhs.parts);
    assert(out@ =~= before + other);
}

pub fn cat(mut lhs: EPage, rhs: EPage) -> (out: EPage)
    ensures
        out@ == lhs@ + rhs@,
{
    append(&mut lhs, rhs);
    lhs
}

pub fn hjoin(xs: &Vec<EPage>, sep: &EPage) -> (out: EPage)
    ensures
        out@ == u::hjoin(pages(xs@), sep@),
{
    let mut out = empty();
    let mut i = 0;
    proof {
        assert(pages(xs@).skip(0) =~= pages(xs@));
    }
    while i < xs.len()
        invariant
            i <= xs.len(),
            u::hjoin(pages(xs@), sep@) == out@ + (if 0 < i < xs.len() {
                sep@
            } else {
                Seq::empty()
            }) + u::hjoin(pages(xs@).skip(i as int), sep@),
        decreases xs.len() - i,
    {
        if i > 0 {
            append(&mut out, copy(sep));
        }
        append(&mut out, copy(&xs[i]));
        proof {
            assert(pages(xs@).skip(i as int).drop_first() =~= pages(xs@).skip(i as int + 1));
        }
        i += 1;
    }
    out
}

pub fn lines(xs: &Vec<EPage>) -> (out: EPage)
    ensures
        out@ == u::lines(pages(xs@)),
{
    let sep = fixed("\n");
    hjoin(xs, &sep)
}

pub fn flatten(xs: &Vec<EPage>) -> (out: EPage)
    ensures
        out@ == pages(xs@).flatten(),
{
    let mut out = empty();
    let mut i = 0;
    proof {
        assert(pages(xs@).skip(0) =~= pages(xs@));
    }
    while i < xs.len()
        invariant
            i <= xs.len(),
            out@ + pages(xs@).skip(i as int).flatten() == pages(xs@).flatten(),
        decreases xs.len() - i,
    {
        append(&mut out, copy(&xs[i]));
        proof {
            assert(pages(xs@).skip(i as int).drop_first() =~= pages(xs@).skip(i as int + 1));
        }
        i += 1;
    }
    out
}

pub fn cell(x: EPage) -> (out: EPage)
    ensures
        out@ == u::cell(x@),
{
    cat(cat(fixed("<td>"), x), fixed("</td>"))
}

pub fn row(xs: &Vec<EPage>) -> (out: EPage)
    ensures
        out@ == u::row(pages(xs@)),
{
    cat(cat(fixed("<tr>"), flatten(xs)), fixed("</tr>"))
}

pub fn link(href: &[u8], label: EPage) -> (out: EPage)
    ensures
        out@ == u::link(href@, label@),
{
    cat(cat(cat(cat(fixed("<a href=\""), attr(href)), fixed("\">")), label), fixed("</a>"))
}

} // verus!
