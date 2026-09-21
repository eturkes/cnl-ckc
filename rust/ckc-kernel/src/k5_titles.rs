use crate::{
    k4_numbers as numbers, k4_search as search, k4_words as words, k5_bytes as b, k5_payload as p,
};
use ckc_spec::check as ck;
use ckc_spec::check::{ECoverageRow, EStatus};
use ckc_spec::ui::{self as u, EGuideline};
use vstd::prelude::*;
use vstd::slice::slice_to_vec;
verus! {

pub fn digits(s: &[u8]) -> (yes: bool)
    ensures
        yes == u::digits(s@),
{
    if s.len() == 0 {
        return false;
    }
    let mut i = 0;
    while i < s.len()
        invariant
            s.len() > 0,
            i <= s.len(),
            forall|j: int| 0 <= j < i ==> ckc_spec::v1text::is_digit_b(#[trigger] s@[j]),
        decreases s.len() - i,
    {
        if !crate::k4_scalar::digit(s[i]) {
            return false;
        }
        i += 1;
    }
    true
}

pub fn first_title(xs: &Vec<Vec<u8>>, fallback: &[u8]) -> (out: Vec<u8>)
    ensures
        out@ == u::first_title(b::views(xs@), fallback@),
{
    let prefix = b::literal("# ");
    let mut i = 0;
    proof {
        reveal_strlit("# ");
        vstd::utf8::is_ascii_chars_encode_utf8("# "@);
        assert(prefix.len() == 2);
        assert(b::views(xs@).skip(0) =~= b::views(xs@));
    }
    while i < xs.len()
        invariant
            i <= xs.len(),
            prefix@ == u::lit("# "@),
            prefix.len() == 2,
            u::first_title(b::views(xs@).skip(i as int), fallback@) == u::first_title(
                b::views(xs@),
                fallback@,
            ),
        decreases xs.len() - i,
    {
        let line = &xs[i];
        if b::starts(line, &prefix) {
            let trimmed = words::trim(&line[2..line.len()]);
            if trimmed.len() > 0 {
                return trimmed;
            }
        }
        proof {
            assert(b::views(xs@).skip(i as int).drop_first() =~= b::views(xs@).skip(i as int + 1));
        }
        i += 1;
    }
    slice_to_vec(fallback)
}

pub fn title(g: &EGuideline) -> (out: Vec<u8>)
    ensures
        out@ == u::title(g@),
{
    match &g.readme {
        Some(data) => {
            let lines = b::split(data, 10);
            first_title(&lines, &g.gid)
        },
        None => slice_to_vec(&g.gid),
    }
}

pub fn sections(s: &[u8]) -> (out: Vec<Vec<u8>>)
    ensures
        b::views(out@) == ck::split_on(s@, 62).map_values(|x: u::Bytes| ck::strip_ws(x)).filter(
            |x: u::Bytes| x.len() > 0,
        ),
{
    let xs = b::split(s, 62);
    let mut out = Vec::new();
    let mut i = 0;
    while i < xs.len()
        invariant
            i <= xs.len(),
            b::views(xs@) == ck::split_on(s@, 62),
            b::views(out@) == b::views(xs@).take(i as int).map_values(
                |x: u::Bytes| ck::strip_ws(x),
            ).filter(|x: u::Bytes| x.len() > 0),
        decreases xs.len() - i,
    {
        let t = words::trim(&xs[i]);
        let ghost tv = t@;
        if t.len() > 0 {
            let ghost prior = b::views(out@);
            out.push(t);
            proof {
                assert(b::views(out@) =~= prior.push(tv));
            }
        }
        proof {
            let before = b::views(xs@).take(i as int).map_values(|x: u::Bytes| ck::strip_ws(x));
            assert(b::views(xs@).take(i as int + 1).map_values(|x: u::Bytes| ck::strip_ws(x))
                =~= before.push(tv));
            before.lemma_filter_push(tv, |x: u::Bytes| x.len() > 0);
        }
        i += 1;
    }
    proof {
        assert(b::views(xs@).take(i as int) =~= b::views(xs@));
    }
    out
}

pub fn human_section(s: &[u8]) -> (out: Vec<u8>)
    ensures
        out@ == u::human_section(s@),
{
    let ss = sections(s);
    if ss.len() == 0 {
        return Vec::new();
    }
    let hs = b::split(&ss[0], 32);
    let first = b::at(&hs, 0);
    let n = b::at(&hs, 1);
    let special = hs.len() == 2 && digits(&n);
    let mut parts = Vec::new();
    if special && b::equal(&first, &b::literal("Rec")) {
        parts.push(b::cat(b::literal("Recommendation "), &n));
    } else if special && b::equal(&first, &b::literal("BOX")) && ss.len() > 1 {
    } else {
        parts.push(slice_to_vec(&ss[0]));
    }
    let ghost head = b::views(parts@);
    proof {
        assert(head =~= if special && first@ == u::lit("Rec"@) {
            seq![u::lit("Recommendation "@) + n@]
        } else if special && first@ == u::lit("BOX"@) && ss.len() > 1 {
            Seq::<u::Bytes>::empty()
        } else {
            seq![ss@[0]@]
        });
        assert(u::human_section(s@) == u::join(head + b::views(ss@).drop_first(), u::lit(" · "@)));
    }
    let mut i = 1;
    proof {
        assert(b::views(ss@).subrange(1, 1) =~= Seq::<u::Bytes>::empty());
    }
    while i < ss.len()
        invariant
            1 <= i <= ss.len(),
            u::human_section(s@) == u::join(head + b::views(ss@).drop_first(), u::lit(" · "@)),
            b::views(parts@) == head + b::views(ss@).subrange(1, i as int),
        decreases ss.len() - i,
    {
        let ghost before = b::views(parts@);
        parts.push(slice_to_vec(&ss[i]));
        proof {
            assert(b::views(parts@) =~= before.push(ss@[i as int]@));
            assert(b::views(ss@).subrange(1, i as int + 1) =~= b::views(ss@).subrange(
                1,
                i as int,
            ).push(ss@[i as int]@));
            assert(before.push(ss@[i as int]@) =~= head + b::views(ss@).subrange(1, i as int + 1));
        }
        i += 1;
    }
    proof {
        assert(b::views(ss@).subrange(1, i as int) =~= b::views(ss@).drop_first());
    }
    b::join(&parts, &b::literal(" · "))
}

pub fn section_count(g: &EGuideline, section: &[u8]) -> (n: usize)
    ensures
        n as nat == g@.documents.filter(
            |d: u::Document| u::coverage_field(g@, d.bundle.docid, 3) == section@,
        ).len(),
{
    let mut n = 0;
    let mut i = 0;
    while i < g.documents.len()
        invariant
            i <= g.documents.len(),
            n <= i,
            n as nat == g@.documents.take(i as int).filter(
                |d: u::Document| u::coverage_field(g@, d.bundle.docid, 3) == section@,
            ).len(),
        decreases g.documents.len() - i,
    {
        let d = &g.documents[i];
        let s = p::coverage_field(g, &d.bundle.docid, 3);
        if b::equal(&s, section) {
            n += 1;
        }
        proof {
            let before = g@.documents.take(i as int);
            assert(g@.documents.take(i as int + 1) =~= before.push(d@));
            before.lemma_filter_push(
                d@,
                |d: u::Document| u::coverage_field(g@, d.bundle.docid, 3) == section@,
            );
        }
        i += 1;
    }
    proof {
        assert(g@.documents.take(i as int) =~= g@.documents);
    }
    n
}

pub fn document_title(g: &EGuideline, id: &[u8]) -> (out: Vec<u8>)
    ensures
        out@ == u::document_title(g@, id@),
{
    let section = p::coverage_field(g, id, 3);
    let base = human_section(&section);
    if base.len() == 0 {
        return slice_to_vec(id);
    }
    if section_count(g, &section) <= 1 {
        return base;
    }
    let region = p::coverage_field(g, id, 0);
    let raw_page = p::coverage_field(g, id, 2);
    let page = p::unprefix(&words::trim(&raw_page), &b::literal("p"));
    let segs = b::split(&region, 45);
    let last = if segs.len() > 0 {
        b::at(&segs, segs.len() - 1)
    } else {
        Vec::new()
    };
    if digits(&page) && digits(&last) {
        let passage = numbers::normalized(&last);
        let mut out = base;
        b::append(&mut out, &b::literal(", page "));
        b::append(&mut out, &page);
        b::append(&mut out, &b::literal(", passage "));
        b::append(&mut out, &passage);
        out
    } else {
        b::cat(b::cat(b::cat(base, &b::literal(" (")), &region), &b::literal(")"))
    }
}

pub fn region_status(r: &ECoverageRow) -> (out: Vec<u8>)
    ensures
        out@ == u::region_status(r@),
{
    match &r.status {
        EStatus::Restates(_) => {
            let s = p::field(r, 4);
            let inner = p::unsuffix(&p::unprefix(&s, &b::literal("restates(")), &b::literal(")"));
            b::cat(b::literal("Restates "), &inner)
        },
        EStatus::Uncovered => {
            let s = p::field(r, 4);
            let inner = p::unsuffix(&p::unprefix(&s, &b::literal("uncovered(")), &b::literal(")"));
            let n = search::sub(&inner, &b::literal(": "), 0);
            let tail = if inner.len() - n >= 2 {
                slice_to_vec(&inner[n + 2..inner.len()])
            } else {
                Vec::new()
            };
            b::cat(b::literal("Not covered — "), &tail)
        },
        EStatus::Pending => b::literal("Pending"),
        _ => Vec::new(),
    }
}

} // verus!
