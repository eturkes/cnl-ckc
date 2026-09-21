use vstd::prelude::*;

verus! {

pub proof fn copy_ok_proof()
    ensures
        ckc_spec::ui::copy_ok(ckc_spec::ui::copy_registry()),
{
    assert(false);
}

pub proof fn index_sound_proof(c: ckc_spec::ui::Corpus)
    ensures
        ckc_spec::ui::well_escaped(ckc_spec::ui::index_html(c)),
        ckc_spec::ui::visible_bytes_from(
            ckc_spec::ui::index_html(c),
            c,
            ckc_spec::ui::copy_registry(),
        ),
{
    hide(ckc_spec::ui::copy_registry);
    hide(ckc_spec::ui::index_html);
    hide(ckc_spec::ui::corpus_bytes);
    crate::k5_sound_pages::index(c);
    crate::k5_sound_escape::page(ckc_spec::ui::index_html(c), c);
}

pub proof fn guideline_sound_proof(c: ckc_spec::ui::Corpus, i: int)
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
    hide(ckc_spec::ui::copy_registry);
    hide(ckc_spec::ui::guideline_html);
    hide(ckc_spec::ui::corpus_bytes);
    crate::k5_sound_corpus::guideline(c, i);
    crate::k5_sound_guideline::page(c.guidelines[i], ckc_spec::ui::corpus_bytes(c));
    crate::k5_sound_escape::page(ckc_spec::ui::guideline_html(c.guidelines[i]), c);
}

pub proof fn records_sound_proof(c: ckc_spec::ui::Corpus, i: int)
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
    assert(false);
}

pub proof fn document_sound_proof(
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
    assert(false);
}

pub proof fn post_sound_proof(req: ckc_spec::ui::Request, s: ckc_spec::ui::PostState)
    ensures
        match ckc_spec::ui::post_outcome(req, s) {
            ckc_spec::ui::PostOutcome::Read => true,
            ckc_spec::ui::PostOutcome::Refused(r) => crate::contract::response_sound(r),
            ckc_spec::ui::PostOutcome::Prepared { response, .. } => crate::contract::response_sound(
                response,
            ),
        },
{
    hide(ckc_spec::ui::copy_registry);
    hide(ckc_spec::ui::post_outcome);
    crate::k5_sound_post::post(req, s);
}

} // verus!
