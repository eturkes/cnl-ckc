use crate::{k5_sound_literals as l, k5_sound_model as m, k5_sound_source as b};
use ckc_spec::{check as ck, ui as u};
use vstd::prelude::*;
verus! {

broadcast use {vstd::seq::group_seq_axioms, vstd::seq_lib::group_seq_properties};

pub proof fn tally_parts(t: (nat, nat, nat), earlier: bool, inputs: Seq<u::Bytes>)
    ensures
        forall|i: int|
            0 <= i < u::tally_parts(t, earlier).len() ==> u::copy_derived(
                #[trigger] u::tally_parts(t, earlier)[i],
                inputs,
                u::copy_registry(),
            ),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(ck::nat_bytes);
    l::l058(inputs);
    l::l059(inputs);
    l::l060(inputs);
    b::decimal(t.0, inputs);
    b::decimal(t.1, inputs);
    b::decimal(t.2, inputs);
    b::cat(ck::nat_bytes(t.0), u::lit(" approved"@), inputs);
    b::cat(ck::nat_bytes(t.1), u::lit(" rejected"@), inputs);
    b::cat(ck::nat_bytes(t.2), u::lit(" earlier"@), inputs);
}

pub proof fn tally_cell(t: (nat, nat, nat), inputs: Seq<u::Bytes>)
    ensures
        u::copy_derived(u::tally_cell(t), inputs, u::copy_registry()),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::tally_parts);
    tally_parts(t, true, inputs);
    let p = u::tally_parts(t, true);
    l::l061(inputs);
    l::l062(inputs);
    if p.len() > 0 {
        b::join(p, u::lit(", "@), inputs);
    }
}

pub proof fn tally_text(t: (nat, nat, nat), inputs: Seq<u::Bytes>)
    ensures
        u::copy_derived(u::tally_text(t), inputs, u::copy_registry()),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::tally_parts);
    hide(ck::nat_bytes);
    tally_parts(t, false, inputs);
    let p = u::tally_parts(t, false);
    l::l063(inputs);
    l::l064(inputs);
    l::l065(inputs);
    l::l066(inputs);
    l::l067(inputs);
    l::l068(inputs);
    let main = if p.len() > 0 {
        u::lit("Decisions on this version: "@) + u::join(p, u::lit(" and "@)) + u::lit("."@)
    } else if t.2 > 0 {
        u::lit("No decision is recorded on this version."@)
    } else {
        u::lit("No decision is recorded."@)
    };
    if p.len() > 0 {
        b::join(p, u::lit(" and "@), inputs);
        b::cat(u::lit("Decisions on this version: "@), u::join(p, u::lit(" and "@)), inputs);
        b::cat(
            u::lit("Decisions on this version: "@) + u::join(p, u::lit(" and "@)),
            u::lit("."@),
            inputs,
        );
    }
    let tail = if t.2 > 0 {
        u::lit(" Decisions on earlier versions: "@) + ck::nat_bytes(t.2) + u::lit("."@)
    } else {
        u::empty()
    };
    if t.2 > 0 {
        b::decimal(t.2, inputs);
        b::cat(u::lit(" Decisions on earlier versions: "@), ck::nat_bytes(t.2), inputs);
        b::cat(
            u::lit(" Decisions on earlier versions: "@) + ck::nat_bytes(t.2),
            u::lit("."@),
            inputs,
        );
    } else {
        m::copy(u::empty(), inputs);
    }
    b::cat(main, tail, inputs);
}

pub proof fn review(g: u::Guideline, inputs: Seq<u::Bytes>)
    ensures
        u::copy_derived(u::review_summary(g), inputs, u::copy_registry()),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::decisions);
    hide(ck::reviewed);
    hide(ck::nat_bytes);
    let n = u::decisions(g).len();
    b::decimal(g.documents.len(), inputs);
    l::l069(inputs);
    l::l070(inputs);
    l::l071(inputs);
    l::l072(inputs);
    l::l073(inputs);
    l::l074(inputs);
    if n == 0 {
        b::cat(
            u::lit("No decisions are recorded for the "@),
            ck::nat_bytes(g.documents.len()),
            inputs,
        );
        b::cat(
            u::lit("No decisions are recorded for the "@) + ck::nat_bytes(g.documents.len()),
            u::lit(" documents in this guideline."@),
            inputs,
        );
    } else {
        b::decimal(n, inputs);
        b::decimal(ck::reviewed(u::decisions(g)).len(), inputs);
        let a = u::lit("Reviewers recorded "@) + ck::nat_bytes(n);
        b::cat(u::lit("Reviewers recorded "@), ck::nat_bytes(n), inputs);
        b::cat(a, u::lit(" decisions on "@), inputs);
        let a = a + u::lit(" decisions on "@);
        b::cat(a, ck::nat_bytes(ck::reviewed(u::decisions(g)).len()), inputs);
        let a = a + ck::nat_bytes(ck::reviewed(u::decisions(g)).len());
        b::cat(a, u::lit(" of "@), inputs);
        let a = a + u::lit(" of "@);
        b::cat(a, ck::nat_bytes(g.documents.len()), inputs);
        let a = a + ck::nat_bytes(g.documents.len());
        b::cat(a, u::lit(" documents."@), inputs);
    }
}

pub proof fn region(r: ck::Row, inputs: Seq<u::Bytes>)
    requires
        m::backed(r.line, inputs),
    ensures
        u::copy_derived(u::region_status(r), inputs, u::copy_registry()),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::field);
    hide(ck::first_sub);
    m::field(r, 4, inputs);
    match r.status {
        ck::Status::Restates(_) => {
            let field = u::field(r, 4);
            m::unprefix(field, u::lit("restates("@), inputs);
            let inner = u::unprefix(field, u::lit("restates("@));
            m::unsuffix(inner, u::lit(")"@), inputs);
            let suffix = u::unsuffix(inner, u::lit(")"@));
            m::copy(suffix, inputs);
            l::l087(inputs);
            b::cat(u::lit("Restates "@), suffix, inputs);
        },
        ck::Status::Uncovered => {
            let field = u::field(r, 4);
            m::unprefix(field, u::lit("uncovered("@), inputs);
            let raw = u::unprefix(field, u::lit("uncovered("@));
            m::unsuffix(raw, u::lit(")"@), inputs);
            let inner = u::unsuffix(raw, u::lit(")"@));
            let i = ck::first_sub(inner, u::lit(": "@), 0);
            let tail = if i + 2 <= inner.len() {
                inner.skip(i as int + 2)
            } else {
                u::empty()
            };
            if i + 2 <= inner.len() {
                m::sub(inner, inputs, i as int + 2, inner.len() as int);
                assert(tail =~= inner.subrange(i as int + 2, inner.len() as int));
            }
            m::copy(tail, inputs);
            l::l091(inputs);
            b::cat(u::lit("Not covered — "@), tail, inputs);
        },
        ck::Status::Pending => l::l092(inputs),
        _ => m::copy(u::empty(), inputs),
    }
}

} // verus!
