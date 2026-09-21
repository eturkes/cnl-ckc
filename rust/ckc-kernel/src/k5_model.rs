use crate::{
    k4_classify as classify, k4_ledger, k5_bytes as b, k5_highlight as bytes, k5_records as r,
};
use ckc_spec::check::{self as ck, EBundle, EDecision};
use ckc_spec::replay::ESrc;
use ckc_spec::ui::{self as u, EGuideline, ERecord};
use vstd::prelude::*;
use vstd::slice::slice_to_vec;
verus! {

pub struct Model {
    pub decisions: Vec<EDecision>,
    pub records: Vec<ERecord>,
    pub bundles: Vec<EBundle>,
}

pub open spec fn bound(m: &Model, g: u::Guideline) -> bool {
    ck::decisions(m.decisions@) == u::decisions(g) && r::records(m.records@) == u::records(g)
        && ck::bundles(m.bundles@) == u::bundles(g)
}

pub fn ledger_data(s: &ESrc) -> (out: Vec<u8>)
    ensures
        out@ == u::ledger_data(s@),
{
    match s {
        ESrc::Bytes(b) => slice_to_vec(b),
        _ => Vec::new(),
    }
}

pub fn docids(g: &EGuideline) -> (out: Vec<Vec<u8>>)
    ensures
        b::views(out@) == u::docids(g@),
{
    let mut out = Vec::new();
    let mut i = 0;
    while i < g.documents.len()
        invariant
            i <= g.documents.len(),
            b::views(out@) == u::docids(g@).take(i as int),
        decreases g.documents.len() - i,
    {
        out.push(slice_to_vec(&g.documents[i].bundle.docid));
        proof {
            assert(u::docids(g@).take(i as int + 1) =~= u::docids(g@).take(i as int).push(
                g@.documents[i as int].bundle.docid,
            ));
        }
        i += 1;
    }
    proof {
        assert(u::docids(g@).take(i as int) =~= u::docids(g@));
    }
    out
}

pub fn bundles(g: &EGuideline) -> (out: Vec<EBundle>)
    ensures
        ck::bundles(out@) == u::bundles(g@),
{
    let mut out = Vec::new();
    let mut i = 0;
    while i < g.documents.len()
        invariant
            i <= g.documents.len(),
            ck::bundles(out@) == u::bundles(g@).take(i as int),
        decreases g.documents.len() - i,
    {
        let d = &g.documents[i].bundle;
        out.push(
            EBundle {
                docid: slice_to_vec(&d.docid),
                ace: slice_to_vec(&d.ace),
                cov: slice_to_vec(&d.cov),
                pay: slice_to_vec(&d.pay),
                cl: slice_to_vec(&d.cl),
                review: slice_to_vec(&d.review),
            },
        );
        proof {
            assert(u::bundles(g@).take(i as int + 1) =~= u::bundles(g@).take(i as int).push(d@));
        }
        i += 1;
    }
    proof {
        assert(u::bundles(g@).take(i as int) =~= u::bundles(g@));
    }
    out
}

pub fn records(g: &EGuideline, ds: &Vec<EDecision>) -> (out: Vec<ERecord>)
    requires
        ck::decisions(ds@) == u::decisions(g@),
    ensures
        r::records(out@) == u::records(g@),
{
    hide(u::decisions);
    let data = ledger_data(&g.ledger);
    let rows = b::raw_rows(&data);
    let mut out = Vec::new();
    let mut i = 0;
    while i < ds.len()
        invariant
            i <= ds.len(),
            ck::decisions(ds@) == u::decisions(g@),
            b::views(rows@) == u::raw_rows(u::ledger_data(g@.ledger)),
            r::records(out@) == u::records(g@).take(i as int),
        decreases ds.len() - i,
    {
        let row = b::at(&rows, i);
        let fields = b::split(&row, 9);
        let d = &ds[i];
        let rec = ERecord {
            docid: slice_to_vec(&d.docid),
            digest: slice_to_vec(&d.digest),
            commit: slice_to_vec(&d.commit),
            approved: d.approved,
            reviewer: b::at(&fields, 4),
            date: slice_to_vec(&d.date),
            comment: b::at(&fields, 6),
        };
        let ghost rv = rec@;
        let ghost before = r::records(out@);
        out.push(rec);
        proof {
            assert(rv == u::records(g@)[i as int]);
            assert(r::records(out@) =~= before.push(rv));
            assert(u::records(g@).take(i as int + 1) =~= u::records(g@).take(i as int).push(rv));
        }
        i += 1;
    }
    proof {
        assert(u::records(g@).take(i as int) =~= u::records(g@));
    }
    out
}

pub fn prepare(g: &EGuideline) -> (out: Model)
    ensures
        bound(&out, g@),
{
    let known = docids(g);
    let (ds, _) = k4_ledger::validate(&g.ledger, &known);
    let rs = records(g, &ds);
    let bs = bundles(g);
    Model { decisions: ds, records: rs, bundles: bs }
}

pub fn state(g: &EGuideline, m: &Model, id: &[u8]) -> (out: u8)
    requires
        bound(m, g@),
    ensures
        out as int == u::state(g@, id@),
{
    let a = classify::current(&m.decisions, &m.bundles, id, true);
    let no = classify::current(&m.decisions, &m.bundles, id, false);
    if a {
        if no {
            2
        } else {
            0
        }
    } else if no {
        1
    } else {
        let reviewed = classify::reviewed_list(&m.decisions);
        if bytes::contains(&reviewed, id) {
            3
        } else {
            4
        }
    }
}

pub fn counts(g: &EGuideline, m: &Model) -> (out: Vec<usize>)
    requires
        bound(m, g@),
    ensures
        out@.map_values(|n: usize| n as nat) == u::class_counts(g@),
{
    let mut out = Vec::new();
    out.push(classify::count(&m.decisions, &m.bundles, 0));
    out.push(classify::count(&m.decisions, &m.bundles, 1));
    out.push(classify::count(&m.decisions, &m.bundles, 2));
    out.push(classify::count(&m.decisions, &m.bundles, 3));
    let seen = classify::reviewed_list(&m.decisions);
    out.push(
        if seen.len() <= m.bundles.len() {
            m.bundles.len() - seen.len()
        } else {
            0
        },
    );
    proof {
        assert(out@.map_values(|n: usize| n as nat) =~= u::class_counts(g@));
    }
    out
}

pub fn summary(g: &EGuideline, m: &Model) -> (out: Vec<u8>)
    requires
        bound(m, g@),
    ensures
        out@ == u::review_summary(g@),
{
    let n = m.decisions.len();
    if n == 0 {
        b::cat(
            b::cat(
                b::literal("No decisions are recorded for the "),
                &b::nat_bytes(g.documents.len() as u64),
            ),
            &b::literal(" documents in this guideline."),
        )
    } else {
        let seen = classify::reviewed_list(&m.decisions);
        let mut out = b::literal("Reviewers recorded ");
        b::append(&mut out, &b::nat_bytes(n as u64));
        b::append(&mut out, &b::literal(" decisions on "));
        b::append(&mut out, &b::nat_bytes(seen.len() as u64));
        b::append(&mut out, &b::literal(" of "));
        b::append(&mut out, &b::nat_bytes(g.documents.len() as u64));
        b::append(&mut out, &b::literal(" documents."));
        out
    }
}

} // verus!
