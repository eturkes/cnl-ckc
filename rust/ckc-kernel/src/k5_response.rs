use crate::{k5_bytes as b, k5_frame as frame, k5_html as h, k5_render};
use ckc_spec::ui::{self as u, EPage, EPiece, EPostOutcome, EResponse};
use vstd::prelude::*;
use vstd::slice::slice_to_vec;
verus! {

pub fn error_page(status: u16, title: &[u8], body: EPage) -> (out: EResponse)
    ensures
        out@ == u::error_page(status as nat, title@, body@),
{
    let ghost bv = body@;
    let mut rows = Vec::new();
    rows.push(h::cat(h::cat(h::fixed("<h1>"), h::text(title)), h::fixed("</h1>")));
    rows.push(body);
    proof {
        assert(h::pages(rows@) =~= seq![
            u::fixed_bytes(u::h1_open()) + u::text(title@) + u::fixed_bytes(u::h1_close()),
            bv,
        ]);
    }
    let page = frame::frame(title, h::fixed("cnl-ckc reviewer"), h::lines(&rows));
    EResponse {
        status,
        body: k5_render::page(&page),
        allow: Vec::new(),
        location: Vec::new(),
        set_cookie: Vec::new(),
    }
}

pub fn annotated(copy: &[u8], detail: &[u8]) -> (out: EPage)
    ensures
        out@ == u::annotated_body(copy@, detail@),
{
    h::cat(
        h::cat(
            h::cat(h::cat(h::fixed("<p>"), h::text(copy)), h::fixed("</p>\n<!-- ")),
            h::one(EPiece::Comment(slice_to_vec(detail))),
        ),
        h::fixed(" -->"),
    )
}

pub fn refusal(status: u16, title: &[u8], detail: &[u8], copy: &[u8]) -> (out: EPostOutcome)
    ensures
        out@ == u::refusal(status as nat, title@, detail@, copy@),
{
    EPostOutcome::Refused(error_page(status, title, annotated(copy, detail)))
}

pub fn forbidden(detail: &[u8]) -> (out: EPostOutcome)
    ensures
        out@ == u::forbidden(detail@),
{
    refusal(
        403,
        &b::literal("Forbidden"),
        detail,
        &b::literal(
            "The request was refused. Open the document page again from this site and submit the decision again.",
        ),
    )
}

pub fn bad_form(detail: &[u8]) -> (out: EPostOutcome)
    ensures
        out@ == u::bad_form(detail@),
{
    refusal(
        400,
        &b::literal("Bad request"),
        detail,
        &b::literal(
            "The submitted form was not valid. Go back to the document page, reload it, and submit the decision again.",
        ),
    )
}

pub fn server_error(detail: &[u8]) -> (out: EPostOutcome)
    ensures
        out@ == u::server_error(detail@),
{
    refusal(
        500,
        &b::literal("Server error"),
        detail,
        &b::literal("The server could not complete the request. Reload the page and try again."),
    )
}

pub fn ledger_changed() -> (out: EPostOutcome)
    ensures
        out@ == u::ledger_changed(),
{
    refusal(
        409,
        &b::literal("Conflict"),
        &b::literal("ui: verdict: ledger changed"),
        &b::literal(
            "Another decision was recorded for this guideline before this one. The decision was not recorded. Open the document page again and check the current state.",
        ),
    )
}

pub fn method(shaped: bool) -> (out: EPostOutcome)
    ensures
        out@ == u::method_response(shaped),
{
    let body = if shaped {
        h::fixed("<p>Only GET and POST are supported on this page.</p>")
    } else {
        h::fixed("<p>Only GET is supported on this page.</p>")
    };
    let mut response = error_page(405, &b::literal("Method not allowed"), body);
    response.allow = if shaped {
        b::literal("GET, POST")
    } else {
        b::literal("GET")
    };
    EPostOutcome::Refused(response)
}

pub fn not_found() -> (out: EPostOutcome)
    ensures
        out@ == u::not_found(),
{
    EPostOutcome::Refused(
        error_page(
            404,
            &b::literal("Not found"),
            h::fixed("<p>The requested page does not exist.</p>"),
        ),
    )
}

pub fn cookie(port: u16, session: &[u8]) -> (out: Vec<u8>)
    ensures
        out@ == u::session_cookie(port as nat, session@),
{
    let name = b::cat(b::literal("ckc-session-"), &b::nat_bytes(port as u64));
    b::cat(b::cat(name, &b::literal("=")), session)
}

pub fn signin(port: u16, session: &[u8]) -> (out: EResponse)
    ensures
        out@ == u::signin_response(port as nat, session@),
{
    let mut response = error_page(
        303,
        &b::literal("cnl-ckc reviewer"),
        h::fixed("<p>Opening the review site.</p>"),
    );
    response.location = b::literal("/");
    response.set_cookie = b::cat(
        cookie(port, session),
        &b::literal("; HttpOnly; SameSite=Lax; Path=/"),
    );
    response
}

pub fn session_refusal() -> (out: EPostOutcome)
    ensures
        out@ == u::session_refusal(),
{
    refusal(
        403,
        &b::literal("Open the review site from ckc"),
        &b::literal("ui: request: no review session"),
        &b::literal(
            "This browser has not opened the review site yet. In the window where ckc ui serve runs, press Enter. The review site opens in this browser.",
        ),
    )
}

pub fn signin_page(port: u16, session: &[u8]) -> (out: EPage)
    ensures
        out@ == u::signin_page(port as nat, session@),
{
    hide(u::lit);
    let refresh = h::cat(
        h::cat(
            h::cat(
                h::cat(
                    h::cat(
                        h::fixed("<meta http-equiv=\"refresh\" content=\"0; url="),
                        h::fixed("http://127.0.0.1:"),
                    ),
                    h::number(port as usize),
                ),
                h::fixed("/session/"),
            ),
            h::attr(session),
        ),
        h::fixed("\">"),
    );
    let mut rows = Vec::new();
    rows.push(h::fixed("<!doctype html>"));
    rows.push(h::fixed("<html lang=\"en\">"));
    rows.push(h::fixed("<head>"));
    rows.push(h::fixed("<meta charset=\"utf-8\">"));
    rows.push(refresh);
    rows.push(
        h::cat(h::cat(h::fixed("<title>"), h::fixed("cnl-ckc reviewer")), h::fixed("</title>")),
    );
    rows.push(h::fixed("</head>"));
    rows.push(h::fixed("<body>"));
    rows.push(h::fixed("<p>Opening the review site.</p>"));
    rows.push(h::fixed("</body>"));
    rows.push(h::fixed("</html>"));
    proof {
        assert(h::pages(rows@) =~= seq![
            u::fixed_bytes(u::doctype_html()),
            u::fixed_bytes(u::html_open()),
            u::fixed_bytes(u::head_open()),
            u::fixed_bytes(u::meta_open()),
            u::fixed_bytes(u::meta_http_equiv_refresh_open()) + u::fixed_bytes(u::http_127_0_0_1())
                + u::number(port as nat) + u::fixed_bytes(u::session_path()) + u::attr(session@)
                + u::fixed_bytes(u::attr_end()),
            u::fixed_bytes(u::title_open()) + u::fixed_bytes(u::cnl_ckc_reviewer())
                + u::fixed_bytes(u::title_close()),
            u::fixed_bytes(u::head_close()),
            u::fixed_bytes(u::body_open()),
            u::fixed_bytes(u::p_open_opening_the_review_site_p_close()),
            u::fixed_bytes(u::body_close()),
            u::fixed_bytes(u::html_close()),
        ]);
    }
    h::cat(h::lines(&rows), h::fixed("\n"))
}

} // verus!
