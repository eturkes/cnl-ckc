use vstd::prelude::*;

verus! {

// TRUSTED binding surface (spec-manifest-pinned): the kernel's public API
// with ensures binding every exec result to ckc-spec. Bodies delegate to
// uninspected impl modules; `cargo verus verify` enforces the binding.
pub fn align_check(align: &[char], src: &[char], ace: &[char]) -> (r: ckc_spec::align::ECheck)
    ensures
        r@ == ckc_spec::align::align_outcome(align@, src@, ace@),
{
    crate::align_impl::align_check_impl(align, src, ace)
}

// M5.2 K1: v1 clause-file acceptance (contract m5u2 R9/R10). The Reject
// offset = deterministic diagnostic (first divergence), pinned by fixtures,
// outside the theorem beyond its bound.
pub fn v1_check(bytes: &[u8]) -> (r: ckc_spec::v1text::EV1Verdict)
    ensures
        r.is_accept() <==> ckc_spec::v1text::accepts(bytes@),
        r matches ckc_spec::v1text::EV1Verdict::Reject { at } ==> at <= bytes@.len(),
{
    crate::v1_impl::v1_check_impl(bytes)
}

// M5.2 K2: composition modes (contract m5u2 R24). Every file the shell read
// arrives as an `ESrc`; a mode's result = the exact (rc, stdout, stderr)
// triple of its spec fn over those sources.
pub fn v1_manifest(mpath: &[u8], m: &ckc_spec::replay::ESrc) -> (r: Result<
    Vec<ckc_spec::replay::ERow>,
    ckc_spec::replay::EOut,
>)
    ensures
        ckc_spec::replay::rows_view(r) == ckc_spec::replay::manifest_rows(mpath@, m@),
{
    crate::k2_impl::v1_manifest_impl(mpath, m)
}

pub fn v1_aggregate_check(
    mpath: &[u8],
    m: &ckc_spec::replay::ESrc,
    pls: &Vec<ckc_spec::replay::ESrc>,
    pys: &Vec<ckc_spec::replay::ESrc>,
) -> (r: ckc_spec::replay::EOut)
    requires
        ckc_spec::replay::cells_ok(m@, ckc_spec::replay::srcs(pls@), ckc_spec::replay::srcs(pys@)),
    ensures
        r@ == ckc_spec::replay::agg_output(
            mpath@,
            m@,
            ckc_spec::replay::srcs(pls@),
            ckc_spec::replay::srcs(pys@),
        ),
{
    crate::k2_impl::v1_aggregate_check_impl(mpath, m, pls, pys)
}

pub fn v1_recursion_check(
    mpath: &[u8],
    m: &ckc_spec::replay::ESrc,
    pls: &Vec<ckc_spec::replay::ESrc>,
    pys: &Vec<ckc_spec::replay::ESrc>,
) -> (r: ckc_spec::replay::EOut)
    requires
        ckc_spec::replay::cells_ok(m@, ckc_spec::replay::srcs(pls@), ckc_spec::replay::srcs(pys@)),
    ensures
        r@ == ckc_spec::replay::recursion_output(
            mpath@,
            m@,
            ckc_spec::replay::srcs(pls@),
            ckc_spec::replay::srcs(pys@),
        ),
{
    crate::k2_impl::v1_recursion_check_impl(mpath, m, pls, pys)
}

// qsha = lowercase hex sha256 of the raw query bytes, computed by the shell (R3).
pub fn v1_answer(
    mpath: &[u8],
    m: &ckc_spec::replay::ESrc,
    pls: &Vec<ckc_spec::replay::ESrc>,
    pys: &Vec<ckc_spec::replay::ESrc>,
    query: &ckc_spec::replay::ESrc,
    qsha: &[u8],
) -> (r: ckc_spec::replay::EOut)
    requires
        ckc_spec::replay::cells_ok(m@, ckc_spec::replay::srcs(pls@), ckc_spec::replay::srcs(pys@)),
    ensures
        r@ == ckc_spec::answers::answer_output(
            mpath@,
            m@,
            ckc_spec::replay::srcs(pls@),
            ckc_spec::replay::srcs(pys@),
            query@,
            qsha@,
        ),
{
    crate::k2_impl::v1_answer_impl(mpath, m, pls, pys, query, qsha)
}

// M5.5 K5: reviewer interface (contract m5u5 P1, R84). Pages = the exact bytes
// of the spec's render fns; every page passes the escaping automaton
// (`well_escaped`) and the fidelity predicate (`visible_bytes_from`: text slots
// = copied corpus spans | registry literals | canonical decimals); POST = the
// ordered first-refusal law; the ledger candidate = byte-exact row insertion;
// the copy registry passes the copy law. Pure spec fns ⇒ determinism.
pub fn ui_render_page(p: &ckc_spec::ui::EPage) -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::ui::render_page(p@),
{
    crate::k5_impl::ui_render_page_impl(p)
}

pub fn ui_render_index(c: &ckc_spec::ui::ECorpus) -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::ui::render_index(c@),
{
    crate::k5_impl::ui_render_index_impl(c)
}

pub fn ui_render_guideline(g: &ckc_spec::ui::EGuideline) -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::ui::render_guideline(g@),
{
    crate::k5_impl::ui_render_guideline_impl(g)
}

pub fn ui_render_records(g: &ckc_spec::ui::EGuideline) -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::ui::render_records(g@),
{
    crate::k5_impl::ui_render_records_impl(g)
}

pub fn ui_render_document(
    g: &ckc_spec::ui::EGuideline,
    d: &ckc_spec::ui::EDocument,
    prev: &[u8],
    next: &[u8],
    token: &[u8],
) -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::ui::render_document(g@, d@, prev@, next@, token@),
{
    crate::k5_impl::ui_render_document_impl(g, d, prev, next, token)
}

pub fn ui_post_outcome(req: &ckc_spec::ui::ERequest, s: &ckc_spec::ui::EPostState) -> (o:
    ckc_spec::ui::EPostOutcome)
    ensures
        o@ == ckc_spec::ui::post_outcome(req@, s@),
{
    crate::k5_impl::ui_post_outcome_impl(req, s)
}

pub fn ui_ledger_candidate(old: &[u8], r: &ckc_spec::ui::ERecord) -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::ui::ledger_candidate(old@, r@),
{
    crate::k5_impl::ui_ledger_candidate_impl(old, r)
}

pub fn ui_copy_violation(b: &[u8]) -> (v: Option<Vec<u8>>)
    ensures
        match v {
            Option::Some(x) => ckc_spec::ui::copy_violation(b@) == Option::Some(x@),
            Option::None => ckc_spec::ui::copy_violation(b@) is None,
        },
{
    crate::k5_impl::ui_copy_violation_impl(b)
}

// A response body is a rendered page that passes both page theorems; the
// refusal/confirmation pages draw no corpus text, so the empty corpus suffices.
pub open spec fn response_sound(r: ckc_spec::ui::Response) -> bool {
    exists|h: ckc_spec::ui::Html| #[trigger]
        ckc_spec::ui::render_page(h) == r.body && ckc_spec::ui::well_escaped(h)
            && ckc_spec::ui::visible_bytes_from(
            h,
            ckc_spec::ui::Corpus { guidelines: Seq::empty(), token: Seq::empty() },
            ckc_spec::ui::copy_registry(),
        )
}

pub proof fn ui_copy_ok()
    ensures
        ckc_spec::ui::copy_ok(ckc_spec::ui::copy_registry()),
{
    crate::k5_sound::copy_ok_proof()
}

pub proof fn ui_index_sound(c: ckc_spec::ui::Corpus)
    ensures
        ckc_spec::ui::well_escaped(ckc_spec::ui::index_html(c)),
        ckc_spec::ui::visible_bytes_from(
            ckc_spec::ui::index_html(c),
            c,
            ckc_spec::ui::copy_registry(),
        ),
{
    crate::k5_sound::index_sound_proof(c)
}

pub proof fn ui_guideline_sound(c: ckc_spec::ui::Corpus, i: int)
    requires
        0 <= i < c.guidelines.len(),
    ensures
        ckc_spec::ui::well_escaped(ckc_spec::ui::guideline_html(c.guidelines[i])),
        ckc_spec::ui::visible_bytes_from(
            ckc_spec::ui::guideline_html(c.guidelines[i]),
            c,
            ckc_spec::ui::copy_registry(),
        ),
{
    crate::k5_sound::guideline_sound_proof(c, i)
}

pub proof fn ui_records_sound(c: ckc_spec::ui::Corpus, i: int)
    requires
        0 <= i < c.guidelines.len(),
    ensures
        ckc_spec::ui::well_escaped(ckc_spec::ui::records_html(c.guidelines[i])),
        ckc_spec::ui::visible_bytes_from(
            ckc_spec::ui::records_html(c.guidelines[i]),
            c,
            ckc_spec::ui::copy_registry(),
        ),
{
    crate::k5_sound::records_sound_proof(c, i)
}

pub proof fn ui_document_sound(
    c: ckc_spec::ui::Corpus,
    i: int,
    j: int,
    prev: ckc_spec::ui::Bytes,
    next: ckc_spec::ui::Bytes,
    token: ckc_spec::ui::Bytes,
)
    requires
        0 <= i < c.guidelines.len(),
        0 <= j < c.guidelines[i].documents.len(),
    ensures
        ckc_spec::ui::well_escaped(
            ckc_spec::ui::document_html(
                c.guidelines[i],
                c.guidelines[i].documents[j],
                prev,
                next,
                token,
            ),
        ),
        ckc_spec::ui::visible_bytes_from(
            ckc_spec::ui::document_html(
                c.guidelines[i],
                c.guidelines[i].documents[j],
                prev,
                next,
                token,
            ),
            c,
            ckc_spec::ui::copy_registry(),
        ),
{
    crate::k5_sound::document_sound_proof(c, i, j, prev, next, token)
}

pub proof fn ui_post_sound(req: ckc_spec::ui::Request, s: ckc_spec::ui::PostState)
    ensures
        match ckc_spec::ui::post_outcome(req, s) {
            ckc_spec::ui::PostOutcome::Read => true,
            ckc_spec::ui::PostOutcome::Refused(r) => response_sound(r),
            ckc_spec::ui::PostOutcome::Prepared { response, .. } => response_sound(response),
        },
{
    crate::k5_sound::post_sound_proof(req, s)
}

} // verus!
