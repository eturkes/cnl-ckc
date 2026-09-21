use crate::{
    k4_bytes as order, k5_bytes as b, k5_highlight as hl, k5_html as h, k5_model as model,
    k5_records as r, k5_text as text,
};
use ckc_spec::align::{ECheck, EModel};
use ckc_spec::check as ck;
use ckc_spec::ui::{self as u, EDocument, EGuideline, EPage, ERecord};
use vstd::prelude::*;
use vstd::slice::slice_to_vec;
verus! {

pub open spec fn alignment_view(m: Option<EModel>) -> Option<ckc_spec::align::AlignModel> {
    match m {
        Some(m) => Some(m@),
        None => None,
    }
}

pub fn alignment(g: &EGuideline, d: &EDocument, source: &[u8]) -> (out: Option<EModel>)
    requires
        source@ == u::payload(g@, d@.bundle.docid),
    ensures
        alignment_view(out) == u::alignment(g@, d@),
{
    match &d.alignment {
        None => None,
        Some(data) => {
            let a = text::chars(data);
            let s = text::chars(source);
            let ace = text::chars(&d.ace);
            match crate::align_impl::align_check_impl(&a, &s, &ace) {
                ECheck::Ok(m) => Some(m),
                ECheck::Err(_) => None,
            }
        },
    }
}

pub fn aligned(g: &EGuideline, d: &EDocument, source: &[u8], m: &Option<EModel>, ace: bool) -> (out:
    EPage)
    requires
        source@ == u::payload(g@, d@.bundle.docid),
        alignment_view(*m) == u::alignment(g@, d@),
    ensures
        out@ == u::aligned_text(g@, d@, ace),
{
    let s = if ace {
        d.ace.as_slice()
    } else {
        source
    };
    match m {
        Some(m) => {
            let ps = if ace {
                &m.ace
            } else {
                &m.src
            };
            hl::marked(s, ps, ace)
        },
        None => if ace {
            hl::keyword(s)
        } else {
            h::text(s)
        },
    }
}

pub fn reviewers(rs: &Vec<ERecord>) -> (out: Vec<Vec<u8>>)
    ensures
        b::views(out@) == r::records(rs@).map_values(|r: u::Record| r.reviewer).filter(
            |x: u::Bytes| x.len() > 0,
        ),
{
    let mut out = Vec::new();
    let mut i = 0;
    while i < rs.len()
        invariant
            i <= rs.len(),
            b::views(out@) == r::records(rs@).take(i as int).map_values(
                |r: u::Record| r.reviewer,
            ).filter(|x: u::Bytes| x.len() > 0),
        decreases rs.len() - i,
    {
        let name = &rs[i].reviewer;
        if name.len() > 0 {
            let ghost before = b::views(out@);
            out.push(slice_to_vec(name));
            proof {
                assert(b::views(out@) =~= before.push(name@));
            }
        }
        proof {
            let before = r::records(rs@).take(i as int).map_values(|r: u::Record| r.reviewer);
            assert(r::records(rs@).take(i as int + 1).map_values(|r: u::Record| r.reviewer)
                =~= before.push(name@));
            before.lemma_filter_push(name@, |x: u::Bytes| x.len() > 0);
        }
        i += 1;
    }
    proof {
        assert(r::records(rs@).take(i as int) =~= r::records(rs@));
    }
    out
}

pub proof fn dedup_push(s: Seq<u::Bytes>, x: u::Bytes)
    ensures
        ck::dedup_bytes(s.push(x)) == if ck::dedup_bytes(s).contains(x) {
            ck::dedup_bytes(s)
        } else {
            ck::dedup_bytes(s).push(x)
        },
{
    reveal_with_fuel(Seq::fold_left, 2);
    assert(s.push(x).drop_last() =~= s);
}

pub fn dedup(xs: &Vec<Vec<u8>>) -> (out: Vec<Vec<u8>>)
    ensures
        b::views(out@) == ck::dedup_bytes(b::views(xs@)),
{
    let mut out = Vec::new();
    let mut i = 0;
    while i < xs.len()
        invariant
            i <= xs.len(),
            b::views(out@) == ck::dedup_bytes(b::views(xs@).take(i as int)),
        decreases xs.len() - i,
    {
        let x = &xs[i];
        let ghost prior = b::views(out@);
        if !hl::contains(&out, x) {
            out.push(slice_to_vec(x));
            proof {
                assert(b::views(out@) =~= prior.push(x@));
            }
        }
        proof {
            assert(b::views(xs@).take(i as int + 1) =~= b::views(xs@).take(i as int).push(x@));
            dedup_push(b::views(xs@).take(i as int), x@);
        }
        i += 1;
    }
    proof {
        assert(b::views(xs@).take(i as int) =~= b::views(xs@));
    }
    out
}

pub fn insert(x: &[u8], xs: &Vec<Vec<u8>>) -> (out: Vec<Vec<u8>>)
    ensures
        b::views(out@) == ck::insert_bytes(x@, b::views(xs@)),
{
    let mut pos = 0;
    proof {
        assert(b::views(xs@).skip(0) =~= b::views(xs@));
    }
    while pos < xs.len() && order::less(&xs[pos], x)
        invariant
            pos <= xs.len(),
            ck::insert_bytes(x@, b::views(xs@)) == b::views(xs@).take(pos as int)
                + ck::insert_bytes(x@, b::views(xs@).skip(pos as int)),
        decreases xs.len() - pos,
    {
        proof {
            assert(b::views(xs@).take(pos as int + 1) =~= b::views(xs@).take(pos as int).push(
                xs@[pos as int]@,
            ));
            assert(b::views(xs@).skip(pos as int).drop_first() =~= b::views(xs@).skip(
                pos as int + 1,
            ));
        }
        pos += 1;
    }
    let ghost target = b::views(xs@).take(pos as int) + seq![x@] + b::views(xs@).skip(pos as int);
    proof {
        assert(ck::insert_bytes(x@, b::views(xs@)) == target);
    }
    let mut out = Vec::new();
    let mut i = 0;
    while i < pos
        invariant
            i <= pos <= xs.len(),
            b::views(out@) == b::views(xs@).take(i as int),
        decreases pos - i,
    {
        out.push(slice_to_vec(&xs[i]));
        proof {
            assert(b::views(xs@).take(i as int + 1) =~= b::views(xs@).take(i as int).push(
                xs@[i as int]@,
            ));
        }
        i += 1;
    }
    out.push(slice_to_vec(x));
    proof {
        assert(b::views(xs@).subrange(pos as int, pos as int) =~= Seq::<u::Bytes>::empty());
    }
    while i < xs.len()
        invariant
            pos <= i <= xs.len(),
            b::views(out@) == b::views(xs@).take(pos as int) + seq![x@] + b::views(xs@).subrange(
                pos as int,
                i as int,
            ),
            ck::insert_bytes(x@, b::views(xs@)) == target,
        decreases xs.len() - i,
    {
        let ghost before = b::views(out@);
        out.push(slice_to_vec(&xs[i]));
        proof {
            assert(b::views(out@) =~= before.push(xs@[i as int]@));
            assert(b::views(xs@).subrange(pos as int, i as int + 1) =~= b::views(xs@).subrange(
                pos as int,
                i as int,
            ).push(xs@[i as int]@));
            assert(before.push(xs@[i as int]@) =~= b::views(xs@).take(pos as int) + seq![x@]
                + b::views(xs@).subrange(pos as int, i as int + 1));
        }
        i += 1;
    }
    proof {
        assert(b::views(xs@).subrange(pos as int, i as int) =~= b::views(xs@).skip(pos as int));
    }
    out
}

pub fn sorted(xs: &Vec<Vec<u8>>) -> (out: Vec<Vec<u8>>)
    ensures
        b::views(out@) == ck::sort_bytes(b::views(xs@)),
{
    let mut out = Vec::new();
    let mut i = xs.len();
    proof {
        assert(b::views(xs@).skip(i as int) =~= Seq::<u::Bytes>::empty());
    }
    while i > 0
        invariant
            i <= xs.len(),
            b::views(out@) == ck::sort_bytes(b::views(xs@).skip(i as int)),
        decreases i,
    {
        let next = insert(&xs[i - 1], &out);
        proof {
            assert(b::views(xs@).skip(i as int - 1).drop_first() =~= b::views(xs@).skip(i as int));
        }
        out = next;
        i -= 1;
    }
    proof {
        assert(b::views(xs@).skip(0) =~= b::views(xs@));
    }
    out
}

pub fn roster(g: &EGuideline, m: &model::Model) -> (out: EPage)
    requires
        model::bound(m, g@),
    ensures
        out@ == u::roster(g@),
{
    let raw_names = reviewers(&m.records);
    let unique = dedup(&raw_names);
    let names = sorted(&unique);
    let mut items = Vec::new();
    let mut i = 0;
    while i < names.len()
        invariant
            i <= names.len(),
            b::views(names@) == u::names(g@),
            h::pages(items@) == b::views(names@).take(i as int).map_values(
                |n: u::Bytes|
                    u::fixed("<option value=\""@) + u::attr(n) + u::fixed("\"></option>"@),
            ),
        decreases names.len() - i,
    {
        items.push(
            h::cat(
                h::cat(h::fixed("<option value=\""), h::attr(&names[i])),
                h::fixed("\"></option>"),
            ),
        );
        proof {
            assert(b::views(names@).take(i as int + 1).map_values(
                |n: u::Bytes|
                    u::fixed("<option value=\""@) + u::attr(n) + u::fixed("\"></option>"@),
            ) =~= b::views(names@).take(i as int).map_values(
                |n: u::Bytes|
                    u::fixed("<option value=\""@) + u::attr(n) + u::fixed("\"></option>"@),
            ).push(
                u::fixed("<option value=\""@) + u::attr(names@[i as int]@) + u::fixed(
                    "\"></option>"@,
                ),
            ));
        }
        i += 1;
    }
    proof {
        assert(b::views(names@).take(i as int) =~= b::views(names@));
    }
    h::cat(
        h::cat(h::fixed("<datalist id=\"reviewer-names\">"), h::flatten(&items)),
        h::fixed("</datalist>"),
    )
}

pub fn latest(rs: &Vec<ERecord>) -> (out: Vec<u8>)
    ensures
        out@ == u::latest_name(r::records(rs@), u::empty(), u::empty()),
{
    let mut date = Vec::new();
    let mut name = Vec::new();
    let mut i = 0;
    proof {
        assert(r::records(rs@).skip(0) =~= r::records(rs@));
    }
    while i < rs.len()
        invariant
            i <= rs.len(),
            u::latest_name(r::records(rs@).skip(i as int), date@, name@) == u::latest_name(
                r::records(rs@),
                u::empty(),
                u::empty(),
            ),
        decreases rs.len() - i,
    {
        let rec = &rs[i];
        if rec.reviewer.len() > 0 && !order::less(&rec.date, &date) {
            date = slice_to_vec(&rec.date);
            name = slice_to_vec(&rec.reviewer);
        }
        proof {
            assert(r::records(rs@).skip(i as int).drop_first() =~= r::records(rs@).skip(
                i as int + 1,
            ));
        }
        i += 1;
    }
    name
}

pub fn linebreak(c: char) -> (yes: bool)
    ensures
        yes == u::linebreak(c),
{
    c == '\n' || c == '\r' || c == '\u{b}' || c == '\u{c}' || c == '\u{1c}' || c == '\u{1d}' || c
        == '\u{1e}' || c == '\u{85}' || c == '\u{2028}' || c == '\u{2029}'
}

pub fn lines(s: &[u8]) -> (out: usize)
    ensures
        out as nat == u::splitline_count(u::chars(s@), false),
{
    let cs = text::chars(s);
    let mut i = 0;
    let mut count = 0;
    let mut pending = false;
    proof {
        assert(cs@.skip(0) =~= cs@);
    }
    while i < cs.len()
        invariant
            cs@ == u::chars(s@),
            i <= cs.len(),
            count <= i,
            pending ==> count < i,
            count as nat + u::splitline_count(cs@.skip(i as int), pending) == u::splitline_count(
                cs@,
                false,
            ),
        decreases cs.len() - i,
    {
        let n = if linebreak(cs[i]) {
            count += 1;
            pending = false;
            if cs.len() - i > 1 && cs[i] == '\r' && cs[i + 1] == '\n' {
                2
            } else {
                1
            }
        } else {
            pending = true;
            1
        };
        proof {
            assert(cs@.skip(i as int).skip(n as int) =~= cs@.skip(i as int + n as int));
        }
        i += n;
    }
    if pending {
        count + 1
    } else {
        count
    }
}

} // verus!
