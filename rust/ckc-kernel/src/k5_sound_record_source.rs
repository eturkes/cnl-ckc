use crate::{
    k5_sound_corpus as corpus, k5_sound_literals as l, k5_sound_model as m, k5_sound_source as b,
};
use ckc_spec::replay::Src;
use ckc_spec::{check as ck, ui as u};
use vstd::prelude::*;
verus! {

broadcast use {vstd::seq::group_seq_axioms, vstd::seq_lib::group_seq_properties};

pub open spec fn record(r: u::Record, inputs: Seq<u::Bytes>) -> bool {
    m::backed(r.reviewer, inputs) && m::backed(r.comment, inputs) && m::backed(
        r.decision.date,
        inputs,
    )
}

pub proof fn parse_date(
    line: u::Bytes,
    n: nat,
    known: Seq<u::Bytes>,
    prev: Option<(u::Bytes, u::Bytes)>,
    inputs: Seq<u::Bytes>,
)
    requires
        m::backed(line, inputs),
    ensures
        match ck::parse_decision(line, n, known, prev) {
            Ok(d) => m::backed(d.date, inputs),
            Err(_) => true,
        },
{
    hide(ck::docid_ok);
    hide(ck::date_ok);
    hide(ck::text_clean);
    hide(ckc_spec::v1text::hex64);
    hide(ck::hex40);
    hide(ckc_spec::engine::bytes_lt);
    m::split(line, inputs, 9);
    m::at(ck::tab_fields(line), 5, inputs);
}

pub proof fn parse_date_at(
    lines: Seq<u::Bytes>,
    i: nat,
    known: Seq<u::Bytes>,
    prev: Option<(u::Bytes, u::Bytes)>,
    acc: Seq<ck::Decision>,
    inputs: Seq<u::Bytes>,
    j: int,
)
    requires
        forall|k: int| 0 <= k < lines.len() ==> m::backed(#[trigger] lines[k], inputs),
        forall|k: int| 0 <= k < acc.len() ==> m::backed((#[trigger] acc[k]).date, inputs),
        0 <= j < ck::parse_decisions(lines, i, known, prev, acc).0.len(),
    ensures
        m::backed(ck::parse_decisions(lines, i, known, prev, acc).0[j].date, inputs),
    decreases lines.len() - i,
{
    hide(ck::parse_decision);
    reveal(ck::parse_decisions);
    if i < lines.len() {
        parse_date(lines[i as int], i + 2, known, prev, inputs);
        if let Ok(d) = ck::parse_decision(lines[i as int], i + 2, known, prev) {
            assert(ck::parse_decisions(lines, i, known, prev, acc).0 == ck::parse_decisions(
                lines,
                i + 1,
                known,
                Some((d.docid, d.date)),
                acc.push(d),
            ).0);
            parse_date_at(lines, i + 1, known, Some((d.docid, d.date)), acc.push(d), inputs, j);
        } else {
            assert(ck::parse_decisions(lines, i, known, prev, acc).0 == acc);
        }
    } else {
        assert(ck::parse_decisions(lines, i, known, prev, acc).0 == acc);
    }
}

pub proof fn parse_dates(
    lines: Seq<u::Bytes>,
    i: nat,
    known: Seq<u::Bytes>,
    prev: Option<(u::Bytes, u::Bytes)>,
    acc: Seq<ck::Decision>,
    inputs: Seq<u::Bytes>,
)
    requires
        forall|j: int| 0 <= j < lines.len() ==> m::backed(#[trigger] lines[j], inputs),
        forall|j: int| 0 <= j < acc.len() ==> m::backed((#[trigger] acc[j]).date, inputs),
    ensures
        forall|j: int|
            0 <= j < ck::parse_decisions(lines, i, known, prev, acc).0.len() ==> m::backed(
                (#[trigger] ck::parse_decisions(lines, i, known, prev, acc).0[j]).date,
                inputs,
            ),
{
    hide(ck::parse_decisions);
    assert forall|j: int|
        0 <= j < ck::parse_decisions(lines, i, known, prev, acc).0.len() implies m::backed(
        (#[trigger] ck::parse_decisions(lines, i, known, prev, acc).0[j]).date,
        inputs,
    ) by {
        parse_date_at(lines, i, known, prev, acc, inputs, j);
    }
}

pub proof fn dates(g: u::Guideline, inputs: Seq<u::Bytes>)
    requires
        corpus::sources(g, inputs),
    ensures
        forall|i: int|
            0 <= i < u::decisions(g).len() ==> m::backed(
                (#[trigger] u::decisions(g)[i]).date,
                inputs,
            ),
{
    hide(ck::ledger_header);
    hide(ck::has_byte);
    hide(ck::parse_decisions);
    hide(u::docids);
    corpus::basics(g, inputs);
    if let Src::Bytes(bytes) = g.ledger {
        m::own(bytes, inputs);
        if !ck::has_byte(bytes, 13) && bytes.len() > 0 && bytes.last() == 10 && ck::starts(
            bytes,
            ck::ledger_header(),
        ) {
            let n = ck::ledger_header().len();
            m::sub(bytes, inputs, n as int, bytes.len() as int);
            assert(bytes.skip(n as int) =~= bytes.subrange(n as int, bytes.len() as int));
            let data = bytes.skip(n as int);
            m::split(data, inputs, 10);
            let lines = ck::body_lines(data);
            parse_dates(lines, 0, u::docids(g), None, Seq::empty(), inputs);
        }
    }
}

pub proof fn records(g: u::Guideline, inputs: Seq<u::Bytes>)
    requires
        corpus::sources(g, inputs),
    ensures
        forall|i: int| 0 <= i < u::records(g).len() ==> record(#[trigger] u::records(g)[i], inputs),
{
    hide(u::decisions);
    dates(g, inputs);
    corpus::basics(g, inputs);
    m::own(u::ledger_data(g.ledger), inputs);
    m::split(u::ledger_data(g.ledger), inputs, 10);
    let ls = ck::split_on(u::ledger_data(g.ledger), 10);
    m::filter(ls, |r: u::Bytes| r.len() > 0 && r[0] != 35, inputs);
    let rows = u::raw_rows(u::ledger_data(g.ledger));
    assert forall|i: int| 0 <= i < u::records(g).len() implies record(
        #[trigger] u::records(g)[i],
        inputs,
    ) by {
        m::at(rows, i, inputs);
        m::split(u::at(rows, i), inputs, 9);
        let fields = ck::tab_fields(u::at(rows, i));
        m::at(fields, 4, inputs);
        m::at(fields, 6, inputs);
    }
}

pub proof fn history(g: u::Guideline, id: u::Bytes, inputs: Seq<u::Bytes>)
    requires
        corpus::sources(g, inputs),
    ensures
        forall|i: int|
            0 <= i < u::history(g, id).len() ==> record(#[trigger] u::history(g, id)[i], inputs),
{
    hide(u::records);
    records(g, inputs);
    let hs = u::history(g, id);
    assert forall|i: int| 0 <= i < hs.len() implies record(#[trigger] hs[i], inputs) by {
        assert(hs.contains(hs[i]));
        u::records(g).lemma_filter_contains_rev(|r: u::Record| r.decision.docid == id, hs[i]);
        let j = choose|j: int| 0 <= j < u::records(g).len() && u::records(g)[j] == hs[i];
    }
}

pub proof fn human_date(date: u::Bytes, inputs: Seq<u::Bytes>)
    requires
        m::backed(date, inputs),
    ensures
        u::copy_derived(u::human_date(date), inputs, u::copy_registry()),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    m::unsuffix(date, u::lit("Z"@), inputs);
    m::split(u::unsuffix(date, u::lit("Z"@)), inputs, 84);
    let p = ck::split_on(u::unsuffix(date, u::lit("Z"@)), 84);
    if ck::ends(date, u::lit("Z"@)) && p.len() == 2 {
        m::copy(p[0], inputs);
        m::copy(p[1], inputs);
        l::l036(inputs);
        l::l037(inputs);
        b::cat(p[0], u::lit(" "@), inputs);
        b::cat(p[0] + u::lit(" "@), p[1], inputs);
        b::cat(p[0] + u::lit(" "@) + p[1], u::lit(" UTC"@), inputs);
    } else {
        m::copy(date, inputs);
    }
}

} // verus!
