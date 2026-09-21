use crate::{k5_sound_corpus as corpus, k5_sound_model as m};
use ckc_spec::{check as ck, ui as u};
use vstd::prelude::*;
verus! {

broadcast use {vstd::seq::group_seq_axioms, vstd::seq_lib::group_seq_properties};

pub proof fn file_index(xs: Seq<u::Bytes>, s: u::Bytes)
    ensures
        0 <= ck::index_of(xs, s) <= xs.len(),
    decreases xs.len(),
{
    if xs.len() > 0 && xs[0] != s {
        file_index(xs.drop_first(), s);
    }
}

pub proof fn row_index(xs: Seq<ck::Row>, s: u::Bytes)
    ensures
        0 <= ck::row_index(xs, s) <= xs.len(),
    decreases xs.len(),
{
    if xs.len() > 0 && xs[0].id != s {
        row_index(xs.drop_first(), s);
    }
}

pub proof fn payload_in(ps: Seq<(u::Bytes, Seq<u::Bytes>)>, id: u::Bytes)
    ensures
        match ck::payload_in(ps, id) {
            Some(s) => ps.map_values(|p: (u::Bytes, Seq<u::Bytes>)| p.1).flatten().contains(s),
            None => true,
        },
    decreases ps.len(),
{
    if ps.len() > 0 {
        let bags = ps.map_values(|p: (u::Bytes, Seq<u::Bytes>)| p.1);
        if ps[0].0 == id {
            if ps[0].1.len() > 0 {
                let s = ps[0].1[0];
                assert(ps[0].1.contains(s));
                corpus::flat(bags, 0, s);
            }
        } else {
            payload_in(ps.drop_first(), id);
            let tail = ps.drop_first().map_values(|p: (u::Bytes, Seq<u::Bytes>)| p.1);
            assert(bags.drop_first() =~= tail);
            if let Some(s) = ck::payload_in(ps.drop_first(), id) {
                assert(bags.flatten() == bags[0] + tail.flatten());
            }
        }
    }
}

pub proof fn evidence(g: u::Guideline, i: int, s: u::Bytes, inputs: Seq<u::Bytes>)
    requires
        corpus::sources(g, inputs),
        0 <= i < g.coverage.evidence.len(),
        g.coverage.evidence[i].ordinal.contains(s) || g.coverage.evidence[i].payloads.map_values(
            |p: (u::Bytes, Seq<u::Bytes>)| p.1,
        ).flatten().contains(s),
    ensures
        inputs.contains(s),
{
    let bags = g.coverage.evidence.map_values(
        |e: ck::Evidence|
            e.ordinal + e.payloads.map_values(|p: (u::Bytes, Seq<u::Bytes>)| p.1).flatten(),
    );
    assert(bags[i].contains(s));
    corpus::flat(bags, i, s);
    assert(corpus::inputs(g).contains(s));
}

pub proof fn payload(g: u::Guideline, id: u::Bytes, inputs: Seq<u::Bytes>)
    requires
        corpus::sources(g, inputs),
    ensures
        m::backed(u::payload(g, id), inputs),
{
    hide(ck::ace_row);
    hide(ck::rows_in);
    hide(ck::payload_in);
    if let Some(r) = ck::ace_row(g.coverage, id) {
        file_index(g.coverage.files, r.file);
        let i = ck::file_index(g.coverage, r.file);
        if i < g.coverage.evidence.len() {
            let ev = g.coverage.evidence[i];
            if ev.locators.len() > 0 {
                payload_in(ev.payloads, r.id);
                if let Some(s) = ck::payload_in(ev.payloads, r.id) {
                    evidence(g, i, s, inputs);
                    m::own(s, inputs);
                }
            } else {
                let rows = ck::rows_in(g.coverage.rows, r.file);
                row_index(rows, r.id);
                let j = ck::row_index(rows, r.id);
                if j < ev.ordinal.len() {
                    let s = ev.ordinal[j];
                    assert(ev.ordinal.contains(s));
                    evidence(g, i, s, inputs);
                    m::own(s, inputs);
                }
            }
        }
    }
}

} // verus!
