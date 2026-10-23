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
    refusal(409, b::c(191), u::ui_verdict_ledger_changed(), b::c(193));
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

pub proof fn prepare(
    g: u::PostGuideline,
    d: u::PostDocument,
    f: u::Fields,
    snapshot: u::Bytes,
    now: u::Bytes,
)
    ensures
        good(u::prepare_candidate(g, d, f, snapshot, now)),
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
        Err(e) => server_error(u::ui_verdict_manifest_derivation_failed() + e),
        Ok(_) => if fresh.len() == 0 {
            server_error(u::ui_verdict_manifest_derivation_failed_docid());
        } else if f.review != fresh {
            l::l191(Seq::empty());
            l::l225(Seq::empty());
            refusal(409, b::c(191), u::ui_verdict_subject_changed(), b::c(225));
        } else if f.ledger != g.ledger_digest {
            ledger_changed();
        } else if !((snapshot.len() == 0 && f.commit.len() == 0) || d.at_commit == Some(
            (f.commit, f.review),
        )) {
            l::l191(Seq::empty());
            l::l225(Seq::empty());
            refusal(409, b::c(191), u::ui_verdict_commit_does_not_hold(), b::c(225));
        } else {
            let r = u::Record {
                decision: ck::Decision {
                    docid: d.docid,
                    digest: f.review,
                    commit: f.commit,
                    approved: f.verdict == u::approved_2(),
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
                    u::ui_adjudication_ledger_invalid() + ck::strip_ws(ck::render(v).1),
                ),
                None => {
                    l::l227(Seq::empty());
                    l::f228(Seq::empty());
                    error(303, b::c(227), b::f(228));
                    if let u::PostOutcome::Prepared { response, .. } = u::prepare_candidate(
                        g,
                        d,
                        f,
                        snapshot,
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
    let expected = u::http_127_0_0_1() + ck::nat_bytes(s.port);
    if req.origin.is_some() && req.origin != Some(expected) {
        forbidden(u::ui_verdict_origin_not_allowed());
    } else if req.content_type != u::application_x_www_form_urlencoded() {
        bad_form(u::ui_verdict_unsupported_content_type());
    } else {
        match req.body {
            None => bad_form(u::ui_verdict_missing_body()),
            Some(bytes) => match u::parse_form(bytes) {
                Err(e) => bad_form(e),
                Ok(f) => if s.token.len() == 0 || f.csrf != s.token {
                    forbidden(u::ui_verdict_invalid_csrf_token());
                } else {
                    match d.render_error {
                        Some(e) => server_error(e),
                        None => prepare(g, d, f, s.commit, s.now),
                    }
                },
            },
        }
    }
}

pub proof fn signin(port: nat, session: u::Bytes)
    ensures
        good(u::PostOutcome::Refused(u::signin_response(port, session))),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l::l078(Seq::empty());
    l::f284(Seq::empty());
    error(303, b::c(78), b::f(284));
    same_body(u::error_page(303, b::c(78), b::f(284)), u::signin_response(port, session));
}

pub proof fn session_refusal()
    ensures
        good(u::session_refusal()),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l::l281(Seq::empty());
    l::l282(Seq::empty());
    refusal(403, b::c(281), u::ui_request_no_review_session(), b::c(282));
}

pub proof fn gate(req: u::Request, port: nat, session: u::Bytes)
    ensures
        match u::gate(req, port, session) {
            Some(o) => good(o),
            None => true,
        },
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::doc_route);
    hide(ck::nat_bytes);
    hide(u::forbidden);
    hide(u::method_response);
    hide(u::signin_response);
    hide(u::session_refusal);
    hide(u::has_cookie);
    let route = u::doc_route(req.path);
    if req.host != u::copy_127_0_0_1() + ck::nat_bytes(port) {
        forbidden(u::ui_request_host_not_allowed());
    } else if session.len() > 0 && req.method == u::get_cap() && req.path == u::session_path()
        + session {
        signin(port, session);
    } else if req.method != u::get_cap() && (req.method != u::post_cap() || route.is_none()) {
        method(route.is_some());
    } else if session.len() > 0 && !u::has_cookie(req.cookie, u::session_cookie(port, session)) {
        session_refusal();
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
    hide(u::server_error);
    hide(u::not_found);
    hide(u::gate);
    gate(req, s.port, s.session);
    let route = u::doc_route(req.path);
    if u::gate(req, s.port, s.session) is None {
        match s.models {
            Err(e) => server_error(e),
            Ok(gs) => if req.method == u::get_cap() {
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

// The redirect page `ckc ui serve` writes: registry pieces, the port as a
// decimal + the session as an attribute slot inside the refresh target.
pub proof fn signin_page(port: nat, session: u::Bytes)
    ensures
        u::well_escaped(u::signin_page(port, session)),
        u::visible_bytes_from(u::signin_page(port, session), empty_corpus(), u::copy_registry()),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::lines);
    let inputs = Seq::empty();
    l::f040(inputs);
    l::f041(inputs);
    l::f042(inputs);
    l::f043(inputs);
    l::f285(inputs);
    l::f230(inputs);
    h::number(port, inputs, 2);
    l::f277(inputs);
    h::attr(session, inputs, 2);
    l::f011(inputs);
    h::add(b::f(285), b::f(230), inputs, 0, 2, 2);
    h::add(b::f(285) + b::f(230), u::number(port), inputs, 0, 2, 2);
    h::add(b::f(285) + b::f(230) + u::number(port), b::f(277), inputs, 0, 2, 2);
    h::add(b::f(285) + b::f(230) + u::number(port) + b::f(277), u::attr(session), inputs, 0, 2, 2);
    h::add(
        b::f(285) + b::f(230) + u::number(port) + b::f(277) + u::attr(session),
        b::f(11),
        inputs,
        0,
        2,
        0,
    );
    let refresh = b::f(285) + b::f(230) + u::number(port) + b::f(277) + u::attr(session) + b::f(11);
    l::f044(inputs);
    l::f078(inputs);
    l::f045(inputs);
    b::add(b::f(44), b::f(78), inputs);
    b::add(b::f(44) + b::f(78), b::f(45), inputs);
    let title = b::f(44) + b::f(78) + b::f(45);
    l::f048(inputs);
    l::f049(inputs);
    l::f284(inputs);
    l::f056(inputs);
    l::f057(inputs);
    let parts = seq![
        b::f(40),
        b::f(41),
        b::f(42),
        b::f(43),
        refresh,
        title,
        b::f(48),
        b::f(49),
        b::f(284),
        b::f(56),
        b::f(57),
    ];
    assert(parts =~= seq![
        u::fixed_bytes(u::doctype_html()),
        u::fixed_bytes(u::html_open()),
        u::fixed_bytes(u::head_open()),
        u::fixed_bytes(u::meta_open()),
        u::fixed_bytes(u::meta_http_equiv_refresh_open()) + u::fixed_bytes(u::http_127_0_0_1())
            + u::number(port) + u::fixed_bytes(u::session_path()) + u::attr(session)
            + u::fixed_bytes(u::attr_end()),
        u::fixed_bytes(u::title_open()) + u::fixed_bytes(u::cnl_ckc_reviewer()) + u::fixed_bytes(
            u::title_close(),
        ),
        u::fixed_bytes(u::head_close()),
        u::fixed_bytes(u::body_open()),
        u::fixed_bytes(u::p_open_opening_the_review_site_p_close()),
        u::fixed_bytes(u::body_close()),
        u::fixed_bytes(u::html_close()),
    ]);
    b::lines(parts, inputs);
    l::f005(inputs, 0);
    b::add(u::lines(parts), b::f(5), inputs);
    assert(u::corpus_bytes(empty_corpus()) =~= Seq::<u::Bytes>::empty());
    h::page(u::signin_page(port, session), empty_corpus());
}

} // verus!
