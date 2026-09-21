use crate::{k5_bytes as b, k5_frame as frame, k5_html as h, k5_payload as payload};
use ckc_spec::ui::{self as u, EGuideline, EPage, ERecord};
use vstd::prelude::*;
use vstd::slice::slice_to_vec;
verus! {

pub open spec fn records(xs: Seq<ERecord>) -> Seq<u::Record> {
    xs.map_values(|x: ERecord| x@)
}

pub open spec fn refs(xs: Seq<&ERecord>) -> Seq<u::Record> {
    xs.map_values(|x: &ERecord| x@)
}

pub fn current(g: &EGuideline, r: &ERecord) -> (yes: bool)
    ensures
        yes == u::current(g@, r@),
{
    let mut i = 0;
    while i < g.documents.len()
        invariant
            i <= g.documents.len(),
            forall|j: int|
                0 <= j < i ==> !((#[trigger] g@.documents[j]).bundle.docid == r.docid@
                    && g@.documents[j].bundle.review == r.digest@),
        decreases g.documents.len() - i,
    {
        let d = &g.documents[i];
        if b::equal(&d.bundle.docid, &r.docid) && b::equal(&d.bundle.review, &r.digest) {
            proof {
                assert(g@.documents[i as int].bundle.docid == r.docid@
                    && g@.documents[i as int].bundle.review == r.digest@);
            }
            return true;
        }
        i += 1;
    }
    false
}

pub fn history<'a>(rs: &'a Vec<ERecord>, id: &[u8]) -> (out: Vec<&'a ERecord>)
    ensures
        refs(out@) == records(rs@).filter(|r: u::Record| r.decision.docid == id@),
{
    let mut out = Vec::new();
    let mut i = 0;
    while i < rs.len()
        invariant
            i <= rs.len(),
            refs(out@) == records(rs@).take(i as int).filter(
                |r: u::Record| r.decision.docid == id@,
            ),
        decreases rs.len() - i,
    {
        let r = &rs[i];
        if b::equal(&r.docid, id) {
            out.push(r);
        }
        proof {
            assert(records(rs@).take(i as int + 1) =~= records(rs@).take(i as int).push(r@));
            records(rs@).take(i as int).lemma_filter_push(
                r@,
                |r: u::Record| r.decision.docid == id@,
            );
        }
        i += 1;
    }
    proof {
        assert(records(rs@).take(i as int) =~= records(rs@));
    }
    out
}

pub fn tally(g: &EGuideline, rs: &Vec<ERecord>, id: &[u8]) -> (out: (usize, usize, usize))
    requires
        records(rs@) == u::records(g@),
    ensures
        (out.0 as nat, out.1 as nat, out.2 as nat) == u::tally(g@, id@),
{
    let hs = history(rs, id);
    let mut a = 0;
    let mut r = 0;
    let mut old = 0;
    let mut i = 0;
    while i < hs.len()
        invariant
            refs(hs@) == u::history(g@, id@),
            i <= hs.len(),
            a <= i,
            r <= i,
            old <= i,
            a as nat == refs(hs@).take(i as int).filter(
                |x: u::Record| u::current(g@, x) && x.decision.approved,
            ).len(),
            r as nat == refs(hs@).take(i as int).filter(
                |x: u::Record| u::current(g@, x) && !x.decision.approved,
            ).len(),
            old as nat == refs(hs@).take(i as int).filter(|x: u::Record| !u::current(g@, x)).len(),
        decreases hs.len() - i,
    {
        let rec = hs[i];
        let is_current = current(g, rec);
        if is_current {
            if rec.approved {
                a += 1;
            } else {
                r += 1;
            }
        } else {
            old += 1;
        }
        proof {
            let before = refs(hs@).take(i as int);
            assert(refs(hs@).take(i as int + 1) =~= before.push(rec@));
            before.lemma_filter_push(rec@, |x: u::Record| u::current(g@, x) && x.decision.approved);
            before.lemma_filter_push(
                rec@,
                |x: u::Record| u::current(g@, x) && !x.decision.approved,
            );
            before.lemma_filter_push(rec@, |x: u::Record| !u::current(g@, x));
        }
        i += 1;
    }
    proof {
        assert(refs(hs@).take(i as int) =~= refs(hs@));
    }
    (a, r, old)
}

pub fn tally_parts(t: (usize, usize, usize), earlier: bool) -> (out: Vec<Vec<u8>>)
    ensures
        b::views(out@) == u::tally_parts((t.0 as nat, t.1 as nat, t.2 as nat), earlier),
{
    let mut out = Vec::new();
    if t.0 > 0 {
        out.push(b::cat(b::nat_bytes(t.0 as u64), &b::literal(" approved")));
    }
    if t.1 > 0 {
        out.push(b::cat(b::nat_bytes(t.1 as u64), &b::literal(" rejected")));
    }
    if earlier && t.2 > 0 {
        out.push(b::cat(b::nat_bytes(t.2 as u64), &b::literal(" earlier")));
    }
    proof {
        assert(b::views(out@) =~= u::tally_parts((t.0 as nat, t.1 as nat, t.2 as nat), earlier));
    }
    out
}

pub fn tally_cell(t: (usize, usize, usize)) -> (out: Vec<u8>)
    ensures
        out@ == u::tally_cell((t.0 as nat, t.1 as nat, t.2 as nat)),
{
    let ps = tally_parts(t, true);
    if ps.len() == 0 {
        b::literal("None")
    } else {
        b::join(&ps, &b::literal(", "))
    }
}

pub fn tally_text(t: (usize, usize, usize)) -> (out: Vec<u8>)
    ensures
        out@ == u::tally_text((t.0 as nat, t.1 as nat, t.2 as nat)),
{
    let ps = tally_parts(t, false);
    let mut out = if ps.len() > 0 {
        let joined = b::join(&ps, &b::literal(" and "));
        b::cat(b::cat(b::literal("Decisions on this version: "), &joined), &b::literal("."))
    } else if t.2 > 0 {
        b::literal("No decision is recorded on this version.")
    } else {
        b::literal("No decision is recorded.")
    };
    if t.2 > 0 {
        b::append(&mut out, &b::literal(" Decisions on earlier versions: "));
        b::append(&mut out, &b::nat_bytes(t.2 as u64));
        b::append(&mut out, &b::literal("."));
    }
    out
}

pub fn human_date(d: &[u8]) -> (out: Vec<u8>)
    ensures
        out@ == u::human_date(d@),
{
    let z = b::literal("Z");
    let stripped = payload::unsuffix(d, &z);
    let ps = b::split(&stripped, 84);
    if b::ends(d, &z) && ps.len() == 2 {
        b::cat(b::cat(b::cat(slice_to_vec(&ps[0]), &b::literal(" ")), &ps[1]), &b::literal(" UTC"))
    } else {
        slice_to_vec(d)
    }
}

pub fn version_link(r: &ERecord, version: &[u8]) -> (out: EPage)
    ensures
        out@ == u::version_link(r@, version@),
{
    if r.commit.len() > 0 {
        let href = b::cat(b::literal("https://github.com/eturkes/cnl-ckc/commit/"), &r.commit);
        h::link(&href, h::text(version))
    } else {
        h::text(version)
    }
}

pub fn record_row(g: &EGuideline, r: &ERecord) -> (out: EPage)
    ensures
        out@ == u::record_row(g@, r@),
{
    let mut cells = Vec::new();
    cells.push(
        h::cell(
            h::text(
                &frame::state_label(
                    if r.approved {
                        0
                    } else {
                        1
                    },
                ),
            ),
        ),
    );
    cells.push(h::cell(h::text(&r.reviewer)));
    cells.push(h::cell(h::text(&human_date(&r.date))));
    let version = if current(g, r) {
        b::literal("Current")
    } else {
        b::literal("Earlier")
    };
    cells.push(h::cell(version_link(r, &version)));
    cells.push(
        h::cell(
            h::text(
                &if r.comment.len() == 0 {
                    b::literal("Not given")
                } else {
                    slice_to_vec(&r.comment)
                },
            ),
        ),
    );
    proof {
        assert(h::pages(cells@) =~= seq![
            u::cell(
                u::text(
                    u::state_label(
                        if r.approved {
                            0
                        } else {
                            1
                        },
                    ),
                ),
            ),
            u::cell(u::text(r.reviewer@)),
            u::cell(u::text(u::human_date(r.date@))),
            u::cell(
                u::version_link(
                    r@,
                    if u::current(g@, r@) {
                        u::lit("Current"@)
                    } else {
                        u::lit("Earlier"@)
                    },
                ),
            ),
            u::cell(
                u::text(
                    if r.comment@.len() == 0 {
                        u::lit("Not given"@)
                    } else {
                        r.comment@
                    },
                ),
            ),
        ]);
    }
    h::row(&cells)
}

} // verus!
