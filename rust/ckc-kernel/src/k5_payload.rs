use crate::k5_bytes as b;
use ckc_spec::check as ck;
use ckc_spec::ui::{self as u, ECoverageRow, EEvidence, EGuideline, EStatus};
use vstd::prelude::*;
use vstd::slice::slice_to_vec;
verus! {

pub open spec fn rows(xs: Seq<ECoverageRow>) -> Seq<ck::Row> {
    xs.map_values(|x: ECoverageRow| x@)
}

pub open spec fn refs(xs: Seq<&ECoverageRow>) -> Seq<ck::Row> {
    xs.map_values(|x: &ECoverageRow| x@)
}

pub fn unprefix(s: &[u8], p: &[u8]) -> (out: Vec<u8>)
    ensures
        out@ == u::unprefix(s@, p@),
{
    if b::starts(s, p) {
        slice_to_vec(&s[p.len()..s.len()])
    } else {
        slice_to_vec(s)
    }
}

pub fn unsuffix(s: &[u8], p: &[u8]) -> (out: Vec<u8>)
    ensures
        out@ == u::unsuffix(s@, p@),
{
    if b::ends(s, p) {
        slice_to_vec(&s[0..s.len() - p.len()])
    } else {
        slice_to_vec(s)
    }
}

pub fn field(r: &ECoverageRow, n: usize) -> (out: Vec<u8>)
    ensures
        out@ == u::field(r@, n as int),
{
    let newline = b::literal("\n");
    let line = unsuffix(&r.line, &newline);
    let fields = b::split(&line, 9);
    b::at(&fields, n)
}

pub fn ace_row<'a>(g: &'a EGuideline, id: &[u8]) -> (out: Option<&'a ECoverageRow>)
    ensures
        match out {
            Some(r) => ck::ace_row(g.coverage@, id@) == Some(r@),
            None => ck::ace_row(g.coverage@, id@) is None,
        },
{
    let mut i = 0;
    proof {
        assert(rows(g.coverage.rows@).skip(0) =~= rows(g.coverage.rows@));
    }
    while i < g.coverage.rows.len()
        invariant
            i <= g.coverage.rows.len(),
            ck::first_ace_row(rows(g.coverage.rows@).skip(i as int), id@) == ck::ace_row(
                g.coverage@,
                id@,
            ),
        decreases g.coverage.rows.len() - i,
    {
        let r = &g.coverage.rows[i];
        match &r.status {
            EStatus::Ace(d) => {
                if b::equal(d, id) {
                    return Some(r);
                }
            },
            _ => {},
        }
        proof {
            assert(rows(g.coverage.rows@).skip(i as int).drop_first() =~= rows(
                g.coverage.rows@,
            ).skip(i as int + 1));
        }
        i += 1;
    }
    None
}

pub fn coverage_field(g: &EGuideline, id: &[u8], n: usize) -> (out: Vec<u8>)
    ensures
        out@ == u::coverage_field(g@, id@, n as int),
{
    match ace_row(g, id) {
        Some(r) => field(r, n),
        None => Vec::new(),
    }
}

pub fn index(xs: &Vec<Vec<u8>>, key: &[u8]) -> (out: usize)
    ensures
        out <= xs.len(),
        out as int == ck::index_of(b::views(xs@), key@),
{
    let mut i = 0;
    proof {
        assert(b::views(xs@).skip(0) =~= b::views(xs@));
    }
    while i < xs.len()
        invariant
            i <= xs.len(),
            i as int + ck::index_of(b::views(xs@).skip(i as int), key@) == ck::index_of(
                b::views(xs@),
                key@,
            ),
        decreases xs.len() - i,
    {
        if b::equal(&xs[i], key) {
            return i;
        }
        proof {
            assert(b::views(xs@).skip(i as int).drop_first() =~= b::views(xs@).skip(i as int + 1));
        }
        i += 1;
    }
    i
}

pub fn cited<'a>(rs: &'a Vec<ECoverageRow>, file: &[u8]) -> (out: Vec<&'a ECoverageRow>)
    ensures
        refs(out@) == ck::rows_in(rows(rs@), file@),
{
    let mut out = Vec::new();
    let mut i = 0;
    while i < rs.len()
        invariant
            i <= rs.len(),
            refs(out@) == ck::rows_in(rows(rs@).take(i as int), file@),
        decreases rs.len() - i,
    {
        let r = &rs[i];
        if b::equal(&r.file, file) {
            out.push(r);
        }
        proof {
            assert(rows(rs@).take(i as int + 1) =~= rows(rs@).take(i as int).push(r@));
            rows(rs@).take(i as int).lemma_filter_push(r@, |r: ck::Row| r.file == file@);
        }
        i += 1;
    }
    proof {
        assert(rows(rs@).take(i as int) =~= rows(rs@));
    }
    out
}

pub fn row_index(rs: &Vec<&ECoverageRow>, id: &[u8]) -> (out: usize)
    ensures
        out <= rs.len(),
        out as int == ck::row_index(refs(rs@), id@),
{
    let mut i = 0;
    proof {
        assert(refs(rs@).skip(0) =~= refs(rs@));
    }
    while i < rs.len()
        invariant
            i <= rs.len(),
            i as int + ck::row_index(refs(rs@).skip(i as int), id@) == ck::row_index(
                refs(rs@),
                id@,
            ),
        decreases rs.len() - i,
    {
        if b::equal(&rs[i].id, id) {
            return i;
        }
        proof {
            assert(refs(rs@).skip(i as int).drop_first() =~= refs(rs@).skip(i as int + 1));
        }
        i += 1;
    }
    i
}

pub fn payload_of(e: &EEvidence, id: &[u8]) -> (out: Option<Vec<u8>>)
    ensures
        match out {
            Some(x) => ck::payload_of(e@, id@) == Some(x@),
            None => ck::payload_of(e@, id@) is None,
        },
{
    let mut i = 0;
    proof {
        assert(e@.payloads.skip(0) =~= e@.payloads);
    }
    while i < e.payloads.len()
        invariant
            i <= e.payloads.len(),
            ck::payload_in(e@.payloads.skip(i as int), id@) == ck::payload_of(e@, id@),
        decreases e.payloads.len() - i,
    {
        let p = &e.payloads[i];
        if b::equal(&p.0, id) {
            return if p.1.len() > 0 {
                Some(slice_to_vec(&p.1[0]))
            } else {
                None
            };
        }
        proof {
            assert(e@.payloads.skip(i as int).drop_first() =~= e@.payloads.skip(i as int + 1));
        }
        i += 1;
    }
    None
}

pub fn payload(g: &EGuideline, id: &[u8]) -> (out: Vec<u8>)
    ensures
        out@ == u::payload(g@, id@),
{
    let r = match ace_row(g, id) {
        Some(r) => r,
        None => return Vec::new(),
    };
    let at = index(&g.coverage.files, &r.file);
    if at >= g.coverage.evidence.len() {
        return Vec::new();
    }
    let e = &g.coverage.evidence[at];
    if e.locators.len() > 0 {
        match payload_of(e, &r.id) {
            Some(p) => p,
            None => Vec::new(),
        }
    } else {
        let rs = cited(&g.coverage.rows, &r.file);
        let n = row_index(&rs, &r.id);
        if n < e.ordinal.len() {
            slice_to_vec(&e.ordinal[n])
        } else {
            Vec::new()
        }
    }
}

} // verus!
