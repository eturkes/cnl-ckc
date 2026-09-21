use vstd::prelude::*;

verus! {

// M5.5 K5 seed: red stubs, one verification error each (contract.rs binds them).
pub fn ui_render_page_impl(p: &ckc_spec::ui::EPage) -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::ui::render_page(p@),
{
    crate::k5_render::page(p)
}

pub fn ui_render_index_impl(c: &ckc_spec::ui::ECorpus) -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::ui::render_index(c@),
{
    assert(false);
    Vec::new()
}

pub fn ui_render_guideline_impl(g: &ckc_spec::ui::EGuideline) -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::ui::render_guideline(g@),
{
    assert(false);
    Vec::new()
}

pub fn ui_render_records_impl(g: &ckc_spec::ui::EGuideline) -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::ui::render_records(g@),
{
    assert(false);
    Vec::new()
}

pub fn ui_render_document_impl(
    g: &ckc_spec::ui::EGuideline,
    d: &ckc_spec::ui::EDocument,
    prev: &[u8],
    next: &[u8],
    token: &[u8],
) -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::ui::render_document(g@, d@, prev@, next@, token@),
{
    assert(false);
    Vec::new()
}

pub fn ui_post_outcome_impl(req: &ckc_spec::ui::ERequest, s: &ckc_spec::ui::EPostState) -> (o:
    ckc_spec::ui::EPostOutcome)
    ensures
        o@ == ckc_spec::ui::post_outcome(req@, s@),
{
    assert(false);
    ckc_spec::ui::EPostOutcome::Read
}

pub fn ui_ledger_candidate_impl(old: &[u8], r: &ckc_spec::ui::ERecord) -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::ui::ledger_candidate(old@, r@),
{
    crate::k5_ledger::candidate(old, r)
}

pub fn ui_copy_violation_impl(b: &[u8]) -> (v: Option<Vec<u8>>)
    ensures
        match v {
            Option::Some(x) => ckc_spec::ui::copy_violation(b@) == Option::Some(x@),
            Option::None => ckc_spec::ui::copy_violation(b@) is None,
        },
{
    crate::k5_copy::violation(b)
}

} // verus!
