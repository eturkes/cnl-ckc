use crate::{k5_bytes as b, k5_chrome as chrome, k5_html as h};
use ckc_spec::ui::{self as u, EPage};
use vstd::prelude::*;
verus! {

pub fn frame(title: &[u8], crumbs: EPage, body: EPage) -> (out: EPage)
    ensures
        out@ == u::frame(title@, crumbs@, body@),
{
    hide(u::lit);
    hide(u::css_text);
    let ghost cv = crumbs@;
    let ghost bv = body@;
    let mut rows = Vec::new();
    rows.push(h::fixed("<!doctype html>"));
    rows.push(h::fixed("<html lang=\"en\">"));
    rows.push(h::fixed("<head>"));
    rows.push(h::fixed("<meta charset=\"utf-8\">"));
    rows.push(
        h::cat(
            h::cat(h::fixed("<title>"), h::text(title)),
            h::fixed(" — cnl-ckc reviewer</title>"),
        ),
    );
    rows.push(h::fixed("<style>"));
    rows.push(h::fixed_bytes(&chrome::css_text()));
    rows.push(h::fixed("</style>"));
    rows.push(h::fixed("</head>"));
    rows.push(h::fixed("<body>"));
    rows.push(h::fixed("<a class=\"skip\" href=\"#main\">Skip to content</a>"));
    rows.push(h::cat(h::cat(h::fixed("<nav class=\"crumbs\">"), crumbs), h::fixed("</nav>")));
    rows.push(h::fixed("<main id=\"main\">"));
    rows.push(body);
    rows.push(h::fixed("</main>"));
    rows.push(
        h::fixed(
            "<footer class=\"scope\"><p>This page reports what the loaded guideline documents state. It does not give clinical advice.</p></footer>",
        ),
    );
    rows.push(h::fixed("</body>"));
    rows.push(h::fixed("</html>"));
    proof {
        assert(h::pages(rows@) =~= seq![
            u::fixed_bytes(u::doctype_html()),
            u::fixed_bytes(u::html_open()),
            u::fixed_bytes(u::head_open()),
            u::fixed_bytes(u::meta_open()),
            u::fixed_bytes(u::title_open()) + u::text(title@) + u::fixed_bytes(
                u::sp_cnl_ckc_reviewer_title_close(),
            ),
            u::fixed_bytes(u::style_open()),
            u::fixed_bytes(u::css_text()),
            u::fixed_bytes(u::style_close()),
            u::fixed_bytes(u::head_close()),
            u::fixed_bytes(u::body_open()),
            u::fixed_bytes(u::a_open_skip_to_content_a_close()),
            u::fixed_bytes(u::nav_open()) + cv + u::fixed_bytes(u::nav_close()),
            u::fixed_bytes(u::main_open()),
            bv,
            u::fixed_bytes(u::main_close()),
            u::fixed_bytes(u::footer_open_p_open_this_page_reports_what()),
            u::fixed_bytes(u::body_close()),
            u::fixed_bytes(u::html_close()),
        ]);
    }
    h::cat(h::lines(&rows), h::fixed("\n"))
}

pub fn state_name(k: u8) -> (out: Vec<u8>)
    ensures
        out@ == u::state_name(k as int),
{
    if k == 0 {
        b::literal("approved")
    } else if k == 1 {
        b::literal("rejected")
    } else if k == 2 {
        b::literal("contested")
    } else if k == 3 {
        b::literal("stale")
    } else {
        b::literal("unreviewed")
    }
}

pub fn state_label(k: u8) -> (out: Vec<u8>)
    ensures
        out@ == u::state_label(k as int),
{
    if k == 0 {
        b::literal("Approved")
    } else if k == 1 {
        b::literal("Rejected")
    } else if k == 2 {
        b::literal("Contested")
    } else if k == 3 {
        b::literal("Outdated")
    } else {
        b::literal("Unreviewed")
    }
}

pub fn chip(k: u8) -> (out: EPage)
    ensures
        out@ == u::chip(k as int),
{
    let name = state_name(k);
    let label = state_label(k);
    h::cat(
        h::cat(
            h::cat(h::cat(h::fixed("<span class=\"chip chip-"), h::attr(&name)), h::fixed("\">")),
            h::text(&label),
        ),
        h::fixed("</span>"),
    )
}

} // verus!
