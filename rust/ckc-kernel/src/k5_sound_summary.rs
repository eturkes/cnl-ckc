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
    b::cat(ck::nat_bytes(t.0), u::sp_approved(), inputs);
    b::cat(ck::nat_bytes(t.1), u::sp_rejected(), inputs);
    b::cat(ck::nat_bytes(t.2), u::sp_earlier(), inputs);
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
        b::join(p, u::comma_sep(), inputs);
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
        u::decisions_on_this_version_cap() + u::join(p, u::sp_and_sp()) + u::period()
    } else if t.2 > 0 {
        u::no_decision_is_recorded_on_this_cap()
    } else {
        u::no_decision_is_recorded_cap()
    };
    if p.len() > 0 {
        b::join(p, u::sp_and_sp(), inputs);
        b::cat(u::decisions_on_this_version_cap(), u::join(p, u::sp_and_sp()), inputs);
        b::cat(
            u::decisions_on_this_version_cap() + u::join(p, u::sp_and_sp()),
            u::period(),
            inputs,
        );
    }
    let tail = if t.2 > 0 {
        u::sp_decisions_on_earlier_versions_cap() + ck::nat_bytes(t.2) + u::period()
    } else {
        u::empty()
    };
    if t.2 > 0 {
        b::decimal(t.2, inputs);
        b::cat(u::sp_decisions_on_earlier_versions_cap(), ck::nat_bytes(t.2), inputs);
        b::cat(u::sp_decisions_on_earlier_versions_cap() + ck::nat_bytes(t.2), u::period(), inputs);
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
            u::no_decisions_are_recorded_for_the_cap_sp(),
            ck::nat_bytes(g.documents.len()),
            inputs,
        );
        b::cat(
            u::no_decisions_are_recorded_for_the_cap_sp() + ck::nat_bytes(g.documents.len()),
            u::sp_documents_in_this_guideline(),
            inputs,
        );
    } else {
        b::decimal(n, inputs);
        b::decimal(ck::reviewed(u::decisions(g)).len(), inputs);
        let a = u::reviewers_recorded_cap_sp() + ck::nat_bytes(n);
        b::cat(u::reviewers_recorded_cap_sp(), ck::nat_bytes(n), inputs);
        b::cat(a, u::sp_decisions_on_sp(), inputs);
        let a = a + u::sp_decisions_on_sp();
        b::cat(a, ck::nat_bytes(ck::reviewed(u::decisions(g)).len()), inputs);
        let a = a + ck::nat_bytes(ck::reviewed(u::decisions(g)).len());
        b::cat(a, u::sp_of_sp(), inputs);
        let a = a + u::sp_of_sp();
        b::cat(a, ck::nat_bytes(g.documents.len()), inputs);
        let a = a + ck::nat_bytes(g.documents.len());
        b::cat(a, u::sp_documents(), inputs);
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
            m::unprefix(field, u::restates_2(), inputs);
            let inner = u::unprefix(field, u::restates_2());
            m::unsuffix(inner, u::paren_close(), inputs);
            let suffix = u::unsuffix(inner, u::paren_close());
            m::copy(suffix, inputs);
            l::l087(inputs);
            b::cat(u::restates_cap_sp(), suffix, inputs);
        },
        ck::Status::Uncovered(_) => {
            let field = u::field(r, 4);
            m::unprefix(field, u::uncovered_2(), inputs);
            let raw = u::unprefix(field, u::uncovered_2());
            m::unsuffix(raw, u::paren_close(), inputs);
            let inner = u::unsuffix(raw, u::paren_close());
            let i = ck::first_sub(inner, u::colon_sep(), 0);
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
            b::cat(u::not_covered_cap_sp(), tail, inputs);
        },
        ck::Status::Pending => l::l092(inputs),
        _ => m::copy(u::empty(), inputs),
    }
}

} // verus!
