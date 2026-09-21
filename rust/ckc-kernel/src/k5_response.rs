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
            u::fixed("<h1>"@) + u::text(title@) + u::fixed("</h1>"@),
            bv,
        ]);
    }
    let page = frame::frame(title, h::fixed("cnl-ckc reviewer"), h::lines(&rows));
    EResponse { status, body: k5_render::page(&page), allow: Vec::new(), location: Vec::new() }
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

} // verus!
