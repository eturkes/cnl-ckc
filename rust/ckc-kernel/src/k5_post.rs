use crate::{
    k4_ledger, k4_render, k4_words, k5_bytes as b, k5_form as form, k5_html as h, k5_ledger,
    k5_model, k5_response as response, k5_route as route, k5_url as url,
};
use ckc_spec::replay::ESrc;
use ckc_spec::ui::{
    self as u, EPostDocument, EPostGuideline, EPostOutcome, EPostState, ERecord, ERequest,
};
use vstd::prelude::*;
use vstd::slice::slice_to_vec;
verus! {

pub open spec fn guidelines(gs: Seq<EPostGuideline>) -> Seq<u::PostGuideline> {
    gs.map_values(|g: EPostGuideline| g@)
}

pub open spec fn documents(ds: Seq<EPostDocument>) -> Seq<u::PostDocument> {
    ds.map_values(|d: EPostDocument| d@)
}

pub fn find_guideline<'a>(gs: &'a Vec<EPostGuideline>, id: &[u8]) -> (out: Option<
    &'a EPostGuideline,
>)
    ensures
        match out {
            Some(g) => u::find_guideline(guidelines(gs@), id@) == Some(g@),
            None => u::find_guideline(guidelines(gs@), id@) is None,
        },
{
    let mut i = 0;
    proof {
        assert(guidelines(gs@).skip(0) =~= guidelines(gs@));
    }
    while i < gs.len()
        invariant
            i <= gs.len(),
            u::find_guideline(guidelines(gs@).skip(i as int), id@) == u::find_guideline(
                guidelines(gs@),
                id@,
            ),
        decreases gs.len() - i,
    {
        if b::equal(&gs[i].gid, id) {
            return Some(&gs[i]);
        }
        proof {
            assert(guidelines(gs@).skip(i as int).drop_first() =~= guidelines(gs@).skip(
                i as int + 1,
            ));
        }
        i += 1;
    }
    None
}

pub fn find_document<'a>(ds: &'a Vec<EPostDocument>, id: &[u8]) -> (out: Option<&'a EPostDocument>)
    ensures
        match out {
            Some(d) => u::find_document(documents(ds@), id@) == Some(d@),
            None => u::find_document(documents(ds@), id@) is None,
        },
{
    let mut i = 0;
    proof {
        assert(documents(ds@).skip(0) =~= documents(ds@));
    }
    while i < ds.len()
        invariant
            i <= ds.len(),
            u::find_document(documents(ds@).skip(i as int), id@) == u::find_document(
                documents(ds@),
                id@,
            ),
        decreases ds.len() - i,
    {
        if b::equal(&ds[i].docid, id) {
            return Some(&ds[i]);
        }
        proof {
            assert(documents(ds@).skip(i as int).drop_first() =~= documents(ds@).skip(
                i as int + 1,
            ));
        }
        i += 1;
    }
    None
}

pub fn known(g: &EPostGuideline) -> (out: Vec<Vec<u8>>)
    ensures
        b::views(out@) == g@.documents.map_values(|d: u::PostDocument| d.docid),
{
    let mut out = Vec::new();
    let mut i = 0;
    while i < g.documents.len()
        invariant
            i <= g.documents.len(),
            b::views(out@) == g@.documents.take(i as int).map_values(|d: u::PostDocument| d.docid),
        decreases g.documents.len() - i,
    {
        out.push(slice_to_vec(&g.documents[i].docid));
        proof {
            assert(g@.documents.take(i as int + 1).map_values(|d: u::PostDocument| d.docid)
                =~= g@.documents.take(i as int).map_values(|d: u::PostDocument| d.docid).push(
                g@.documents[i as int].docid,
            ));
        }
        i += 1;
    }
    proof {
        assert(g@.documents.take(i as int) =~= g@.documents);
    }
    out
}

pub fn fresh_review(g: &EPostGuideline, id: &[u8]) -> (out: Vec<u8>)
    ensures
        out@ == match g@.fresh {
            Ok(bs) => u::last_review(bs, id@, u::empty()),
            Err(_) => u::empty(),
        },
{
    match &g.fresh {
        Err(_) => Vec::new(),
        Ok(bs) => {
            let ghost selected = match g@.fresh {
                Ok(values) => values,
                Err(_) => Seq::empty(),
            };
            let mut out = Vec::new();
            let mut i = 0;
            proof {
                assert(selected.skip(0) =~= selected);
            }
            while i < bs.len()
                invariant
                    g@.fresh == Ok(selected),
                    i <= bs.len(),
                    selected.len() == bs.len(),
                    forall|j: int| 0 <= j < bs.len() ==> #[trigger] selected[j] == bs@[j]@,
                    u::last_review(selected.skip(i as int), id@, out@) == u::last_review(
                        selected,
                        id@,
                        u::empty(),
                    ),
                decreases bs.len() - i,
            {
                if b::equal(&bs[i].docid, id) {
                    out = slice_to_vec(&bs[i].review);
                }
                proof {
                    assert(selected.skip(i as int).drop_first() =~= selected.skip(i as int + 1));
                }
                i += 1;
            }
            out
        },
    }
}

pub fn candidate(g: &EPostGuideline, d: &EPostDocument, f: &form::Fields, now: &[u8]) -> (out:
    EPostOutcome)
    ensures
        out@ == u::prepare_candidate(g@, d@, f@, now@),
{
    hide(u::server_error);
    hide(u::refusal);
    hide(u::ledger_candidate);
    hide(ckc_spec::check::ledger);
    hide(u::lit);
    match &g.fresh {
        Err(e) => return response::server_error(
            &b::cat(b::literal("ui: verdict: manifest derivation failed: "), e),
        ),
        Ok(_) => {},
    }
    let fresh = fresh_review(g, &d.docid);
    if fresh.len() == 0 {
        return response::server_error(
            &b::literal("ui: verdict: manifest derivation failed: docid row missing"),
        );
    }
    if !b::equal(&f.review, &fresh) {
        return response::refusal(
            409,
            &b::literal("Conflict"),
            &b::literal("ui: verdict: subject changed"),
            &b::literal(
                "The document or its source changed after this page was loaded. The decision was not recorded. Open the document page again and check the current version.",
            ),
        );
    }
    if !b::equal(&f.ledger, &g.ledger_digest) {
        return response::ledger_changed();
    }
    let r = ERecord {
        docid: slice_to_vec(&d.docid),
        digest: slice_to_vec(&f.review),
        commit: slice_to_vec(&d.commit),
        approved: b::equal(&f.verdict, &b::literal("approved")),
        reviewer: slice_to_vec(&f.reviewer),
        date: slice_to_vec(now),
        comment: slice_to_vec(&f.comment),
    };
    let old = k5_model::ledger_data(&g.ledger);
    let candidate = k5_ledger::candidate(&old, &r);
    let source = ESrc::Bytes(slice_to_vec(&candidate));
    let ids = known(g);
    let (_, checked) = k4_ledger::validate(&source, &ids);
    match checked {
        Some(v) => {
            let rendered = k4_render::verdict(&v);
            response::server_error(
                &b::cat(
                    b::literal("ui: adjudication ledger invalid: "),
                    &k4_words::trim(&rendered.out),
                ),
            )
        },
        None => {
            let mut result = response::error_page(
                303,
                &b::literal("Decision recorded"),
                h::fixed("<p>The decision was recorded.</p>"),
            );
            let mut location = b::literal("/g/");
            b::append(&mut location, &url::segment(&g.gid));
            b::append(&mut location, &b::literal("/doc/"));
            b::append(&mut location, &url::segment(&d.docid));
            b::append(&mut location, &b::literal(".html"));
            result.location = location;
            EPostOutcome::Prepared {
                candidate,
                expected_ledger: slice_to_vec(&g.ledger_digest),
                response: result,
            }
        },
    }
}

pub fn handle(req: &ERequest, s: &EPostState, g: &EPostGuideline, d: &EPostDocument) -> (out:
    EPostOutcome)
    ensures
        out@ == u::handle_post(req@, s@, g@, d@),
{
    hide(u::forbidden);
    hide(u::bad_form);
    hide(u::server_error);
    hide(u::prepare_candidate);
    hide(u::parse_form);
    hide(u::lit);
    let expected = b::cat(b::literal("http://127.0.0.1:"), &b::nat_bytes(s.port as u64));
    match &req.origin {
        Some(origin) => {
            if !b::equal(origin, &expected) {
                return response::forbidden(&b::literal("ui: verdict: origin not allowed"));
            }
        },
        None => {},
    }
    if !b::equal(&req.content_type, &b::literal("application/x-www-form-urlencoded")) {
        return response::bad_form(&b::literal("ui: verdict: unsupported content type"));
    }
    let body = match &req.body {
        None => return response::bad_form(&b::literal("ui: verdict: missing body")),
        Some(b) => b,
    };
    let fields = match form::parse(body) {
        Err(e) => return response::bad_form(&e),
        Ok(f) => f,
    };
    if s.token.len() == 0 || !b::equal(&fields.csrf, &s.token) {
        return response::forbidden(&b::literal("ui: verdict: invalid csrf token"));
    }
    match &d.render_error {
        Some(e) => response::server_error(e),
        None => candidate(g, d, &fields, &s.now),
    }
}

pub fn outcome(req: &ERequest, s: &EPostState) -> (out: EPostOutcome)
    ensures
        out@ == u::post_outcome(req@, s@),
{
    hide(u::forbidden);
    hide(u::method_response);
    hide(u::server_error);
    hide(u::not_found);
    hide(u::handle_post);
    hide(u::lit);
    let route = route::document(&req.path);
    let host = b::cat(b::literal("127.0.0.1:"), &b::nat_bytes(s.port as u64));
    if !b::equal(&req.host, &host) {
        return response::forbidden(&b::literal("ui: request: host not allowed"));
    }
    let get = b::equal(&req.method, &b::literal("GET"));
    let post = b::equal(&req.method, &b::literal("POST"));
    if !get && (!post || route.is_none()) {
        return response::method(route.is_some());
    }
    let gs = match &s.models {
        Err(e) => return response::server_error(e),
        Ok(gs) => gs,
    };
    if get {
        return EPostOutcome::Read;
    }
    let (gid, id) = match route {
        None => return response::not_found(),
        Some(pair) => pair,
    };
    let g = match find_guideline(gs, &gid) {
        None => return response::not_found(),
        Some(g) => g,
    };
    let d = match find_document(&g.documents, &id) {
        None => return response::not_found(),
        Some(d) => d,
    };
    handle(req, s, g, d)
}

} // verus!
