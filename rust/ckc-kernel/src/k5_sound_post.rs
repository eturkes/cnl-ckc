use crate::{k5_sound_build as b, k5_sound_escape as h, k5_sound_literals as l};
use ckc_spec::{check as ck, ui as u};
use vstd::prelude::*;
verus! {

pub open spec fn empty_corpus() -> u::Corpus {
    u::Corpus { guidelines: Seq::empty(), token: Seq::empty() }
}

pub open spec fn response_ok(r: u::Response) -> bool {
    exists|page: u::Html| #[trigger]
        u::render_page(page) == r.body && u::well_escaped(page) && u::visible_bytes_from(
            page,
            empty_corpus(),
            u::copy_registry(),
        )
}

pub open spec fn good(o: u::PostOutcome) -> bool {
    match o {
        u::PostOutcome::Read => true,
        u::PostOutcome::Refused(r) => response_ok(r),
        u::PostOutcome::Prepared { response, .. } => response_ok(response),
    }
}

pub proof fn witness(page: u::Html, r: u::Response)
    requires
        h::sound(page, Seq::empty()),
        u::render_page(page) == r.body,
    ensures
        response_ok(r),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    assert(u::corpus_bytes(empty_corpus()) =~= Seq::<u::Bytes>::empty());
    h::page(page, empty_corpus());
}

pub proof fn same_body(a: u::Response, b: u::Response)
    requires
        response_ok(a),
        a.body == b.body,
    ensures
        response_ok(b),
{
}

pub proof fn error(status: nat, title: u::Bytes, body: u::Html)
    requires
        u::copy_derived(title, Seq::empty(), u::copy_registry()),
        h::sound(body, Seq::empty()),
    ensures
        response_ok(u::error_page(status, title, body)),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::frame);
    hide(u::lines);
    let inputs = Seq::empty();
    l::f078(inputs);
    l::f097(inputs);
    l::f098(inputs);
    h::text(title, inputs);
    b::add(b::f(97), u::text(title), inputs);
    b::add(b::f(97) + u::text(title), b::f(98), inputs);
    let parts = seq![b::f(97) + u::text(title) + b::f(98), body];
    b::lines(parts, inputs);
    b::frame(title, b::f(78), u::lines(parts), inputs);
    let page = u::frame(title, b::f(78), u::lines(parts));
    witness(page, u::error_page(status, title, body));
}

pub proof fn annotated(copy: u::Bytes, detail: u::Bytes)
    requires
        u::copy_derived(copy, Seq::empty(), u::copy_registry()),
    ensures
        h::sound(u::annotated_body(copy, detail), Seq::empty()),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    let inputs = Seq::empty();
    l::f099(inputs);
    h::text(copy, inputs);
    l::f183(inputs);
    h::comment_piece(detail, inputs);
    l::f184(inputs);
    b::add(b::f(99), u::text(copy), inputs);
    h::add(b::f(99) + u::text(copy), b::f(183), inputs, 0, 0, 4);
    h::add(b::f(99) + u::text(copy) + b::f(183), seq![u::Piece::Comment(detail)], inputs, 0, 4, 4);
    h::add(
        b::f(99) + u::text(copy) + b::f(183) + seq![u::Piece::Comment(detail)],
        b::f(184),
        inputs,
        0,
        4,
        0,
    );
}

pub proof fn refusal(status: nat, title: u::Bytes, detail: u::Bytes, copy: u::Bytes)
    requires
        u::copy_derived(title, Seq::empty(), u::copy_registry()),
        u::copy_derived(copy, Seq::empty(), u::copy_registry()),
    ensures
        good(u::refusal(status, title, detail, copy)),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    annotated(copy, detail);
    error(status, title, u::annotated_body(copy, detail));
}

pub proof fn forbidden(detail: u::Bytes)
    ensures
        good(u::forbidden(detail)),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l::l187(Seq::empty());
    l::l185(Seq::empty());
    refusal(403, b::c(187), detail, u::refused_copy());
}

pub proof fn bad_form(detail: u::Bytes)
    ensures
        good(u::bad_form(detail)),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l::l188(Seq::empty());
    l::l186(Seq::empty());
    refusal(400, b::c(188), detail, u::invalid_form_copy());
}

pub proof fn server_error(detail: u::Bytes)
    ensures
        good(u::server_error(detail)),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l::l189(Seq::empty());
    l::l190(Seq::empty());
    refusal(500, b::c(189), detail, b::c(190));
}

pub proof fn ledger_changed()
    ensures
        good(u::ledger_changed()),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l::l191(Seq::empty());
    l::l193(Seq::empty());
    refusal(409, b::c(191), u::lit("ui: verdict: ledger changed"@), b::c(193));
}

pub proof fn method(shaped: bool)
    ensures
        good(u::method_response(shaped)),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l::l196(Seq::empty());
    let body = if shaped {
        b::f(197)
    } else {
        b::f(198)
    };
    if shaped {
        l::f197(Seq::empty());
    } else {
        l::f198(Seq::empty());
    }
    error(405, b::c(196), body);
    if let u::PostOutcome::Refused(r) = u::method_response(shaped) {
        same_body(u::error_page(405, b::c(196), body), r);
    }
}

pub proof fn not_found()
    ensures
        good(u::not_found()),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l::l201(Seq::empty());
    l::f202(Seq::empty());
    error(404, b::c(201), b::f(202));
}

pub proof fn prepare(g: u::PostGuideline, d: u::PostDocument, f: u::Fields, now: u::Bytes)
    ensures
        good(u::prepare_candidate(g, d, f, now)),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::ledger_candidate);
    hide(ck::ledger);
    hide(u::last_review);
    hide(u::ledger_data);
    hide(ck::render);
    hide(ck::strip_ws);
    hide(u::url_seg);
    let fresh = match g.fresh {
        Ok(bs) => u::last_review(bs, d.docid, u::empty()),
        Err(_) => u::empty(),
    };
    match g.fresh {
        Err(e) => server_error(u::lit("ui: verdict: manifest derivation failed: "@) + e),
        Ok(_) => if fresh.len() == 0 {
            server_error(u::lit("ui: verdict: manifest derivation failed: docid row missing"@));
        } else if f.review != fresh {
            l::l191(Seq::empty());
            l::l225(Seq::empty());
            refusal(409, b::c(191), u::lit("ui: verdict: subject changed"@), b::c(225));
        } else if f.ledger != g.ledger_digest {
            ledger_changed();
        } else {
            let r = u::Record {
                decision: ck::Decision {
                    docid: d.docid,
                    digest: f.review,
                    commit: d.commit,
                    approved: f.verdict == u::lit("approved"@),
                    date: now,
                },
                reviewer: f.reviewer,
                comment: f.comment,
            };
            let candidate = u::ledger_candidate(u::ledger_data(g.ledger), r);
            let checked = ck::ledger(
                ckc_spec::replay::Src::Bytes(candidate),
                g.documents.map_values(|x: u::PostDocument| x.docid),
            );
            match checked.1 {
                Some(v) => server_error(
                    u::lit("ui: adjudication ledger invalid: "@) + ck::strip_ws(ck::render(v).1),
                ),
                None => {
                    l::l227(Seq::empty());
                    l::f228(Seq::empty());
                    error(303, b::c(227), b::f(228));
                    if let u::PostOutcome::Prepared { response, .. } = u::prepare_candidate(
                        g,
                        d,
                        f,
                        now,
                    ) {
                        same_body(u::error_page(303, b::c(227), b::f(228)), response);
                    }
                },
            }
        },
    }
}

pub proof fn handle(req: u::Request, s: u::PostState, g: u::PostGuideline, d: u::PostDocument)
    ensures
        good(u::handle_post(req, s, g, d)),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::parse_form);
    hide(ck::nat_bytes);
    hide(u::prepare_candidate);
    hide(u::forbidden);
    hide(u::bad_form);
    hide(u::server_error);
    let expected = u::lit("http://127.0.0.1:"@) + ck::nat_bytes(s.port);
    if req.origin.is_some() && req.origin != Some(expected) {
        forbidden(u::lit("ui: verdict: origin not allowed"@));
    } else if req.content_type != u::lit("application/x-www-form-urlencoded"@) {
        bad_form(u::lit("ui: verdict: unsupported content type"@));
    } else {
        match req.body {
            None => bad_form(u::lit("ui: verdict: missing body"@)),
            Some(bytes) => match u::parse_form(bytes) {
                Err(e) => bad_form(e),
                Ok(f) => if s.token.len() == 0 || f.csrf != s.token {
                    forbidden(u::lit("ui: verdict: invalid csrf token"@));
                } else {
                    match d.render_error {
                        Some(e) => server_error(e),
                        None => prepare(g, d, f, s.now),
                    }
                },
            },
        }
    }
}

pub proof fn post(req: u::Request, s: u::PostState)
    ensures
        good(u::post_outcome(req, s)),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::doc_route);
    hide(u::find_guideline);
    hide(u::find_document);
    hide(ck::nat_bytes);
    hide(u::handle_post);
    hide(u::forbidden);
    hide(u::method_response);
    hide(u::server_error);
    hide(u::not_found);
    let route = u::doc_route(req.path);
    if req.host != u::lit("127.0.0.1:"@) + ck::nat_bytes(s.port) {
        forbidden(u::lit("ui: request: host not allowed"@));
    } else if req.method != u::lit("GET"@) && (req.method != u::lit("POST"@) || route.is_none()) {
        method(route.is_some());
    } else {
        match s.models {
            Err(e) => server_error(e),
            Ok(gs) => if req.method == u::lit("GET"@) {
            } else {
                match route {
                    None => not_found(),
                    Some((gid, id)) => match u::find_guideline(gs, gid) {
                        None => not_found(),
                        Some(g) => match u::find_document(g.documents, id) {
                            None => not_found(),
                            Some(d) => handle(req, s, g, d),
                        },
                    },
                }
            },
        }
    }
}

} // verus!
