use crate::{
    k5_bytes as b, k5_chrome as chrome, k5_document_data as data, k5_frame as frame,
    k5_highlight as bytes, k5_html as h, k5_model as model, k5_payload as payload,
    k5_records as records, k5_titles as titles, k5_url as url,
};
use ckc_spec::check as ck;
use ckc_spec::ui::{self as u, EDocument, EGuideline, EPage};
use vstd::prelude::*;
use vstd::slice::slice_to_vec;
verus! {

pub fn pdfs(g: &EGuideline) -> (out: Vec<Vec<u8>>)
    ensures
        b::views(out@) == g@.source_names.filter(|n: u::Bytes| ck::ends(n, u::lit(".pdf"@))),
{
    let suffix = b::literal(".pdf");
    let mut out = Vec::new();
    let mut i = 0;
    while i < g.source_names.len()
        invariant
            i <= g.source_names.len(),
            suffix@ == u::lit(".pdf"@),
            b::views(out@) == g@.source_names.take(i as int).filter(
                |n: u::Bytes| ck::ends(n, u::lit(".pdf"@)),
            ),
        decreases g.source_names.len() - i,
    {
        let name = &g.source_names[i];
        if b::ends(name, &suffix) {
            let ghost before = b::views(out@);
            out.push(slice_to_vec(name));
            proof {
                assert(b::views(out@) =~= before.push(name@));
            }
        }
        proof {
            let before = g@.source_names.take(i as int);
            assert(g@.source_names.take(i as int + 1) =~= before.push(name@));
            before.lemma_filter_push(name@, |n: u::Bytes| ck::ends(n, u::lit(".pdf"@)));
        }
        i += 1;
    }
    proof {
        assert(g@.source_names.take(i as int) =~= g@.source_names);
    }
    out
}

pub open spec fn heading_spec(g: u::Guideline) -> u::Html {
    let pdfs = g.source_names.filter(|n: u::Bytes| ck::ends(n, u::lit(".pdf"@)));
    u::text(u::title(g)) + (if pdfs.len() == 1 {
        u::fixed(" <a class=\"source\" href=\"../source/"@) + u::attr(u::url_seg(pdfs[0]))
            + u::fixed("\">PDF</a>"@)
    } else {
        Seq::empty()
    })
}

pub fn heading(g: &EGuideline) -> (out: EPage)
    ensures
        out@ == heading_spec(g@),
{
    let ps = pdfs(g);
    let title = titles::title(g);
    let mut out = h::text(&title);
    if ps.len() == 1 {
        h::append(
            &mut out,
            h::cat(
                h::cat(
                    h::fixed(" <a class=\"source\" href=\"../source/"),
                    h::attr(&url::segment(&ps[0])),
                ),
                h::fixed("\">PDF</a>"),
            ),
        );
    }
    out
}

pub open spec fn provenance_spec(g: u::Guideline, id: u::Bytes) -> Seq<u::Html> {
    let region = u::coverage_field(g, id, 0);
    let source = u::unprefix(u::coverage_field(g, id, 1), u::lit("source/"@));
    (if region.len() > 0 {
        seq![u::text(region)]
    } else {
        Seq::empty()
    }) + (if g.source_names.contains(source) {
        seq![u::link(u::lit("../source/"@) + u::url_seg(source), u::fixed("Source text"@))]
    } else {
        Seq::empty()
    })
}

pub fn provenance(g: &EGuideline, id: &[u8]) -> (out: Vec<EPage>)
    ensures
        h::pages(out@) == provenance_spec(g@, id@),
{
    let region = payload::coverage_field(g, id, 0);
    let file = payload::coverage_field(g, id, 1);
    let source = payload::unprefix(&file, &b::literal("source/"));
    let mut out = Vec::new();
    if region.len() > 0 {
        out.push(h::text(&region));
    }
    if bytes::contains(&g.source_names, &source) {
        let href = b::cat(b::literal("../source/"), &url::segment(&source));
        out.push(h::link(&href, h::fixed("Source text")));
    }
    proof {
        assert(h::pages(out@) =~= provenance_spec(g@, id@));
    }
    out
}

pub open spec fn navigation_spec(prev: u::Bytes, next: u::Bytes) -> Seq<u::Html> {
    (if prev.len() > 0 {
        seq![u::link(u::url_seg(prev) + u::lit(".html"@), u::fixed("Previous document"@))]
    } else {
        Seq::empty()
    }) + seq![u::fixed("<a href=\"../index.html\">Guideline index</a>"@)] + (if next.len() > 0 {
        seq![u::link(u::url_seg(next) + u::lit(".html"@), u::fixed("Next document"@))]
    } else {
        Seq::empty()
    })
}

pub fn navigation(prev: &[u8], next: &[u8]) -> (out: Vec<EPage>)
    ensures
        h::pages(out@) == navigation_spec(prev@, next@),
{
    let mut out = Vec::new();
    if prev.len() > 0 {
        out.push(
            h::link(
                &b::cat(url::segment(prev), &b::literal(".html")),
                h::fixed("Previous document"),
            ),
        );
    }
    out.push(h::fixed("<a href=\"../index.html\">Guideline index</a>"));
    if next.len() > 0 {
        out.push(
            h::link(&b::cat(url::segment(next), &b::literal(".html")), h::fixed("Next document")),
        );
    }
    proof {
        assert(h::pages(out@) =~= navigation_spec(prev@, next@));
    }
    out
}

pub open spec fn form_spec(g: u::Guideline, d: u::Document, token: u::Bytes) -> Seq<u::Html> {
    seq![
        u::fixed("<section class=\"verdict-entry\">"@),
        u::fixed("<h3>Record a decision</h3>"@),
        u::fixed("<p>Does the ACE representation appropriately reflect the original passage?</p>"@),
        u::fixed("<form method=\"post\">"@),
        u::fixed("<fieldset>"@),
        u::fixed("<legend>Decision</legend>"@),
        u::fixed(
            "<label><input type=\"radio\" name=\"verdict\" value=\"approved\" required> Approved</label>"@,
        ),
        u::fixed(
            "<label><input type=\"radio\" name=\"verdict\" value=\"rejected\" required> Rejected</label>"@,
        ),
        u::fixed("</fieldset>"@),
        u::fixed("<label for=\"reviewer\">Reviewer name</label>"@),
        u::fixed(
            "<input type=\"text\" id=\"reviewer\" name=\"reviewer\" list=\"reviewer-names\" value=\""@,
        ) + u::attr(u::latest_name(u::records(g), u::empty(), u::empty())) + u::fixed(
            "\" required>"@,
        ),
        u::roster(g),
        u::fixed("<label for=\"comment\">Comment (optional)</label>"@),
        u::fixed("<textarea id=\"comment\" name=\"comment\"></textarea>"@),
        u::fixed("<input type=\"hidden\" name=\"review_sha256\" value=\""@) + u::attr(
            d.bundle.review,
        ) + u::fixed("\">"@),
        u::fixed("<input type=\"hidden\" name=\"ledger_sha256\" value=\""@) + u::attr(
            g.ledger_digest,
        ) + u::fixed("\">"@),
        u::fixed("<input type=\"hidden\" name=\"csrf\" value=\""@) + u::attr(token) + u::fixed(
            "\">"@,
        ),
        u::fixed("<button>Record decision</button>"@),
        u::fixed("</form>"@),
        u::fixed("</section>"@),
    ]
}

pub fn form(g: &EGuideline, d: &EDocument, m: &model::Model, token: &[u8]) -> (out: Vec<EPage>)
    requires
        model::bound(m, g@),
    ensures
        h::pages(out@) == form_spec(g@, d@, token@),
{
    hide(u::roster);
    hide(u::latest_name);
    hide(u::lit);
    let mut out = Vec::new();
    out.push(h::fixed("<section class=\"verdict-entry\">"));
    out.push(h::fixed("<h3>Record a decision</h3>"));
    out.push(
        h::fixed("<p>Does the ACE representation appropriately reflect the original passage?</p>"),
    );
    out.push(h::fixed("<form method=\"post\">"));
    out.push(h::fixed("<fieldset>"));
    out.push(h::fixed("<legend>Decision</legend>"));
    out.push(
        h::fixed(
            "<label><input type=\"radio\" name=\"verdict\" value=\"approved\" required> Approved</label>",
        ),
    );
    out.push(
        h::fixed(
            "<label><input type=\"radio\" name=\"verdict\" value=\"rejected\" required> Rejected</label>",
        ),
    );
    out.push(h::fixed("</fieldset>"));
    out.push(h::fixed("<label for=\"reviewer\">Reviewer name</label>"));
    out.push(
        h::cat(
            h::cat(
                h::fixed(
                    "<input type=\"text\" id=\"reviewer\" name=\"reviewer\" list=\"reviewer-names\" value=\"",
                ),
                h::attr(&data::latest(&m.records)),
            ),
            h::fixed("\" required>"),
        ),
    );
    out.push(data::roster(g, m));
    out.push(h::fixed("<label for=\"comment\">Comment (optional)</label>"));
    out.push(h::fixed("<textarea id=\"comment\" name=\"comment\"></textarea>"));
    out.push(
        h::cat(
            h::cat(
                h::fixed("<input type=\"hidden\" name=\"review_sha256\" value=\""),
                h::attr(&d.bundle.review),
            ),
            h::fixed("\">"),
        ),
    );
    out.push(
        h::cat(
            h::cat(
                h::fixed("<input type=\"hidden\" name=\"ledger_sha256\" value=\""),
                h::attr(&g.ledger_digest),
            ),
            h::fixed("\">"),
        ),
    );
    out.push(
        h::cat(
            h::cat(h::fixed("<input type=\"hidden\" name=\"csrf\" value=\""), h::attr(token)),
            h::fixed("\">"),
        ),
    );
    out.push(h::fixed("<button>Record decision</button>"));
    out.push(h::fixed("</form>"));
    out.push(h::fixed("</section>"));
    proof {
        assert(h::pages(out@) =~= form_spec(g@, d@, token@));
    }
    out
}

pub open spec fn body_spec(
    g: u::Guideline,
    d: u::Document,
    prev: u::Bytes,
    next: u::Bytes,
    token: u::Bytes,
) -> Seq<u::Html> {
    let id = d.bundle.docid;
    let k = u::state(g, id);
    let prov = provenance_spec(g, id);
    let nav = navigation_spec(prev, next);
    let records_href = u::lit("../records.html"@) + (if u::history(g, id).len() > 0 {
        u::lit("#"@) + u::url_seg(id)
    } else {
        u::empty()
    });
    let shown = match u::alignment(g, d) {
        Some(m) => m.count > 0,
        _ => false,
    };
    seq![
        u::fixed("<h1>"@) + heading_spec(g) + u::fixed("</h1>"@),
        u::fixed("<h2>"@) + u::text(u::document_title(g, id)) + u::fixed(" "@) + u::chip(k)
            + u::fixed("</h2>"@),
    ] + (if prov.len() > 0 {
        seq![u::fixed("<p>"@) + u::hjoin(prov, u::fixed(" · "@)) + u::fixed("</p>"@)]
    } else {
        Seq::empty()
    }) + seq![
        u::fixed("<p>"@) + u::text(u::tally_text(u::tally(g, id))) + u::fixed(" "@) + u::link(
            records_href,
            u::fixed("All decision records"@),
        ) + u::fixed("</p>"@),
    ] + (if k == 3 {
        seq![
            u::fixed("<section class=\"stale\">"@),
            u::fixed(
                "<p>The document or its source changed after the last decision. No recorded decision applies to the version shown here.</p>"@,
            ),
            u::fixed("</section>"@),
        ]
    } else {
        Seq::empty()
    }) + (if shown {
        seq![
            u::fixed(
                "<p class=\"hl-note\"><label><input type=\"checkbox\" class=\"hl-toggle\" checked> Highlighting</label> Try hovering and clicking on highlighted terms for different levels of emphasis.</p>"@,
            ),
            u::fixed_bytes(u::script_html()),
        ]
    } else {
        Seq::empty()
    }) + seq![
        u::fixed("<section>"@),
        u::fixed("<h3>Original passage</h3>"@),
        u::fixed("<pre class=\"prose\">"@) + u::aligned_text(g, d, false) + u::fixed("</pre>"@),
        u::fixed("</section>"@),
        u::fixed("<section>"@),
        u::fixed("<h3>Attempto Controlled English (ACE)</h3>"@),
        u::fixed("<pre class=\"prose\">"@) + u::aligned_text(g, d, true) + u::fixed("</pre>"@),
        u::fixed("</section>"@),
    ] + form_spec(g, d, token) + seq![
        u::fixed("<section>"@),
        u::fixed("<details>"@),
        u::fixed("<summary>Compiled Prolog ("@) + u::number(
            u::splitline_count(u::chars(d.pl), false),
        ) + u::fixed(" lines)</summary>"@),
        u::fixed("<pre>"@) + u::text(d.pl) + u::fixed("</pre>"@),
        u::fixed("</details>"@),
        u::fixed("</section>"@),
        u::fixed("<nav class=\"docnav\">"@) + u::hjoin(nav, u::fixed(" · "@)) + u::fixed("</nav>"@),
    ]
}

pub proof fn decomposition(
    g: u::Guideline,
    d: u::Document,
    prev: u::Bytes,
    next: u::Bytes,
    token: u::Bytes,
)
    ensures
        u::document_html(g, d, prev, next, token) == u::frame(
            u::document_title(g, d.bundle.docid),
            u::fixed("<a href=\"../../../index.html\">guidelines</a> / <a href=\"../index.html\">"@)
                + u::text(u::title(g)) + u::fixed("</a> / "@) + u::text(
                u::coverage_field(g, d.bundle.docid, 0),
            ),
            u::lines(body_spec(g, d, prev, next, token)),
        ),
{
    hide(u::frame);
    hide(u::lit);
    hide(u::title);
    hide(u::document_title);
    hide(u::alignment);
    hide(u::aligned_text);
    hide(u::tally);
    hide(u::tally_text);
    hide(u::state);
    hide(u::roster);
    hide(u::latest_name);
    hide(u::splitline_count);
    let id = d.bundle.docid;
    let k = u::state(g, id);
    let region = u::coverage_field(g, id, 0);
    let pdfs = g.source_names.filter(|n: u::Bytes| ck::ends(n, u::lit(".pdf"@)));
    let heading = u::text(u::title(g)) + (if pdfs.len() == 1 {
        u::fixed(" <a class=\"source\" href=\"../source/"@) + u::attr(u::url_seg(pdfs[0]))
            + u::fixed("\">PDF</a>"@)
    } else {
        Seq::empty()
    });
    let source = u::unprefix(u::coverage_field(g, id, 1), u::lit("source/"@));
    let prov = (if region.len() > 0 {
        seq![u::text(region)]
    } else {
        Seq::empty()
    }) + (if g.source_names.contains(source) {
        seq![u::link(u::lit("../source/"@) + u::url_seg(source), u::fixed("Source text"@))]
    } else {
        Seq::empty()
    });
    let records_href = u::lit("../records.html"@) + (if u::history(g, id).len() > 0 {
        u::lit("#"@) + u::url_seg(id)
    } else {
        u::empty()
    });
    let shown = match u::alignment(g, d) {
        Option::Some(m) => m.count > 0,
        _ => false,
    };
    let nav = (if prev.len() > 0 {
        seq![u::link(u::url_seg(prev) + u::lit(".html"@), u::fixed("Previous document"@))]
    } else {
        Seq::empty()
    }) + seq![u::fixed("<a href=\"../index.html\">Guideline index</a>"@)] + (if next.len() > 0 {
        seq![u::link(u::url_seg(next) + u::lit(".html"@), u::fixed("Next document"@))]
    } else {
        Seq::empty()
    });
    let original = seq![
        u::fixed("<h1>"@) + heading + u::fixed("</h1>"@),
        u::fixed("<h2>"@) + u::text(u::document_title(g, id)) + u::fixed(" "@) + u::chip(k)
            + u::fixed("</h2>"@),
    ] + (if prov.len() > 0 {
        seq![u::fixed("<p>"@) + u::hjoin(prov, u::fixed(" · "@)) + u::fixed("</p>"@)]
    } else {
        Seq::empty()
    }) + seq![
        u::fixed("<p>"@) + u::text(u::tally_text(u::tally(g, id))) + u::fixed(" "@) + u::link(
            records_href,
            u::fixed("All decision records"@),
        ) + u::fixed("</p>"@),
    ] + (if k == 3 {
        seq![
            u::fixed("<section class=\"stale\">"@),
            u::fixed(
                "<p>The document or its source changed after the last decision. No recorded decision applies to the version shown here.</p>"@,
            ),
            u::fixed("</section>"@),
        ]
    } else {
        Seq::empty()
    }) + (if shown {
        seq![
            u::fixed(
                "<p class=\"hl-note\"><label><input type=\"checkbox\" class=\"hl-toggle\" checked> Highlighting</label> Try hovering and clicking on highlighted terms for different levels of emphasis.</p>"@,
            ),
            u::fixed_bytes(u::script_html()),
        ]
    } else {
        Seq::empty()
    }) + seq![
        u::fixed("<section>"@),
        u::fixed("<h3>Original passage</h3>"@),
        u::fixed("<pre class=\"prose\">"@) + u::aligned_text(g, d, false) + u::fixed("</pre>"@),
        u::fixed("</section>"@),
        u::fixed("<section>"@),
        u::fixed("<h3>Attempto Controlled English (ACE)</h3>"@),
        u::fixed("<pre class=\"prose\">"@) + u::aligned_text(g, d, true) + u::fixed("</pre>"@),
        u::fixed("</section>"@),
        u::fixed("<section class=\"verdict-entry\">"@),
        u::fixed("<h3>Record a decision</h3>"@),
        u::fixed("<p>Does the ACE representation appropriately reflect the original passage?</p>"@),
        u::fixed("<form method=\"post\">"@),
        u::fixed("<fieldset>"@),
        u::fixed("<legend>Decision</legend>"@),
        u::fixed(
            "<label><input type=\"radio\" name=\"verdict\" value=\"approved\" required> Approved</label>"@,
        ),
        u::fixed(
            "<label><input type=\"radio\" name=\"verdict\" value=\"rejected\" required> Rejected</label>"@,
        ),
        u::fixed("</fieldset>"@),
        u::fixed("<label for=\"reviewer\">Reviewer name</label>"@),
        u::fixed(
            "<input type=\"text\" id=\"reviewer\" name=\"reviewer\" list=\"reviewer-names\" value=\""@,
        ) + u::attr(u::latest_name(u::records(g), u::empty(), u::empty())) + u::fixed(
            "\" required>"@,
        ),
        u::roster(g),
        u::fixed("<label for=\"comment\">Comment (optional)</label>"@),
        u::fixed("<textarea id=\"comment\" name=\"comment\"></textarea>"@),
        u::fixed("<input type=\"hidden\" name=\"review_sha256\" value=\""@) + u::attr(
            d.bundle.review,
        ) + u::fixed("\">"@),
        u::fixed("<input type=\"hidden\" name=\"ledger_sha256\" value=\""@) + u::attr(
            g.ledger_digest,
        ) + u::fixed("\">"@),
        u::fixed("<input type=\"hidden\" name=\"csrf\" value=\""@) + u::attr(token) + u::fixed(
            "\">"@,
        ),
        u::fixed("<button>Record decision</button>"@),
        u::fixed("</form>"@),
        u::fixed("</section>"@),
        u::fixed("<section>"@),
        u::fixed("<details>"@),
        u::fixed("<summary>Compiled Prolog ("@) + u::number(
            u::splitline_count(u::chars(d.pl), false),
        ) + u::fixed(" lines)</summary>"@),
        u::fixed("<pre>"@) + u::text(d.pl) + u::fixed("</pre>"@),
        u::fixed("</details>"@),
        u::fixed("</section>"@),
        u::fixed("<nav class=\"docnav\">"@) + u::hjoin(nav, u::fixed(" · "@)) + u::fixed("</nav>"@),
    ];
    assert(body_spec(g, d, prev, next, token) =~= original);
}

pub open spec fn leading_spec(g: u::Guideline, d: u::Document, shown: bool) -> Seq<u::Html> {
    let id = d.bundle.docid;
    let k = u::state(g, id);
    let prov = provenance_spec(g, id);
    let href = u::lit("../records.html"@) + (if u::history(g, id).len() > 0 {
        u::lit("#"@) + u::url_seg(id)
    } else {
        u::empty()
    });
    seq![
        u::fixed("<h1>"@) + heading_spec(g) + u::fixed("</h1>"@),
        u::fixed("<h2>"@) + u::text(u::document_title(g, id)) + u::fixed(" "@) + u::chip(k)
            + u::fixed("</h2>"@),
    ] + (if prov.len() > 0 {
        seq![u::fixed("<p>"@) + u::hjoin(prov, u::fixed(" · "@)) + u::fixed("</p>"@)]
    } else {
        Seq::empty()
    }) + seq![
        u::fixed("<p>"@) + u::text(u::tally_text(u::tally(g, id))) + u::fixed(" "@) + u::link(
            href,
            u::fixed("All decision records"@),
        ) + u::fixed("</p>"@),
    ] + (if k == 3 {
        seq![
            u::fixed("<section class=\"stale\">"@),
            u::fixed(
                "<p>The document or its source changed after the last decision. No recorded decision applies to the version shown here.</p>"@,
            ),
            u::fixed("</section>"@),
        ]
    } else {
        Seq::empty()
    }) + (if shown {
        seq![
            u::fixed(
                "<p class=\"hl-note\"><label><input type=\"checkbox\" class=\"hl-toggle\" checked> Highlighting</label> Try hovering and clicking on highlighted terms for different levels of emphasis.</p>"@,
            ),
            u::fixed_bytes(u::script_html()),
        ]
    } else {
        Seq::empty()
    })
}

pub open spec fn passages_spec(g: u::Guideline, d: u::Document) -> Seq<u::Html> {
    seq![
        u::fixed("<section>"@),
        u::fixed("<h3>Original passage</h3>"@),
        u::fixed("<pre class=\"prose\">"@) + u::aligned_text(g, d, false) + u::fixed("</pre>"@),
        u::fixed("</section>"@),
        u::fixed("<section>"@),
        u::fixed("<h3>Attempto Controlled English (ACE)</h3>"@),
        u::fixed("<pre class=\"prose\">"@) + u::aligned_text(g, d, true) + u::fixed("</pre>"@),
        u::fixed("</section>"@),
    ]
}

pub open spec fn compiled_spec(d: u::Document, prev: u::Bytes, next: u::Bytes) -> Seq<u::Html> {
    seq![
        u::fixed("<section>"@),
        u::fixed("<details>"@),
        u::fixed("<summary>Compiled Prolog ("@) + u::number(
            u::splitline_count(u::chars(d.pl), false),
        ) + u::fixed(" lines)</summary>"@),
        u::fixed("<pre>"@) + u::text(d.pl) + u::fixed("</pre>"@),
        u::fixed("</details>"@),
        u::fixed("</section>"@),
        u::fixed("<nav class=\"docnav\">"@) + u::hjoin(
            navigation_spec(prev, next),
            u::fixed(" · "@),
        ) + u::fixed("</nav>"@),
    ]
}

pub proof fn body_groups(
    g: u::Guideline,
    d: u::Document,
    prev: u::Bytes,
    next: u::Bytes,
    token: u::Bytes,
    shown: bool,
)
    requires
        shown == match u::alignment(g, d) {
            Some(m) => m.count > 0,
            _ => false,
        },
    ensures
        body_spec(g, d, prev, next, token) == leading_spec(g, d, shown) + passages_spec(g, d)
            + form_spec(g, d, token) + compiled_spec(d, prev, next),
{
    hide(u::lit);
    hide(u::title);
    hide(u::document_title);
    hide(u::alignment);
    hide(u::aligned_text);
    hide(u::tally);
    hide(u::tally_text);
    hide(u::state);
    hide(u::roster);
    hide(u::latest_name);
    hide(u::splitline_count);
    assert(body_spec(g, d, prev, next, token) =~= leading_spec(g, d, shown) + passages_spec(g, d)
        + form_spec(g, d, token) + compiled_spec(d, prev, next));
}

pub fn leading(g: &EGuideline, d: &EDocument, m: &model::Model, shown: bool) -> (out: Vec<EPage>)
    requires
        model::bound(m, g@),
    ensures
        h::pages(out@) == leading_spec(g@, d@, shown),
{
    hide(u::lit);
    hide(u::title);
    hide(u::document_title);
    hide(u::state);
    hide(u::tally);
    hide(u::tally_text);
    hide(u::records);
    hide(u::decisions);
    let id = &d.bundle.docid;
    let k = model::state(g, m, id);
    let prov = provenance(g, id);
    let title = titles::document_title(g, id);
    let mut records_href = b::literal("../records.html");
    let history = records::history(&m.records, id);
    proof {
        assert(records::refs(history@) == u::history(g@, id@));
    }
    if history.len() > 0 {
        b::append(&mut records_href, &b::literal("#"));
        b::append(&mut records_href, &url::segment(id));
    }
    proof {
        assert(records_href@ == u::lit("../records.html"@) + (if u::history(g@, id@).len() > 0 {
            u::lit("#"@) + u::url_seg(id@)
        } else {
            u::empty()
        }));
    }
    let mut body = Vec::new();
    body.push(h::cat(h::cat(h::fixed("<h1>"), heading(g)), h::fixed("</h1>")));
    body.push(
        h::cat(
            h::cat(
                h::cat(h::cat(h::fixed("<h2>"), h::text(&title)), h::fixed(" ")),
                frame::chip(k),
            ),
            h::fixed("</h2>"),
        ),
    );
    if prov.len() > 0 {
        body.push(
            h::cat(h::cat(h::fixed("<p>"), h::hjoin(&prov, &h::fixed(" · "))), h::fixed("</p>")),
        );
    }
    body.push(
        h::cat(
            h::cat(
                h::cat(
                    h::cat(
                        h::fixed("<p>"),
                        h::text(&records::tally_text(records::tally(g, &m.records, id))),
                    ),
                    h::fixed(" "),
                ),
                h::link(&records_href, h::fixed("All decision records")),
            ),
            h::fixed("</p>"),
        ),
    );
    if k == 3 {
        body.push(h::fixed("<section class=\"stale\">"));
        body.push(
            h::fixed(
                "<p>The document or its source changed after the last decision. No recorded decision applies to the version shown here.</p>",
            ),
        );
        body.push(h::fixed("</section>"));
    }
    if shown {
        body.push(
            h::fixed(
                "<p class=\"hl-note\"><label><input type=\"checkbox\" class=\"hl-toggle\" checked> Highlighting</label> Try hovering and clicking on highlighted terms for different levels of emphasis.</p>",
            ),
        );
        body.push(h::fixed_bytes(&chrome::script_html()));
    }
    proof {
        assert(h::pages(body@) =~= leading_spec(g@, d@, shown));
    }
    body
}

pub fn passages(
    g: &EGuideline,
    d: &EDocument,
    source: &[u8],
    alignment: &Option<ckc_spec::align::EModel>,
) -> (out: Vec<EPage>)
    requires
        source@ == u::payload(g@, d@.bundle.docid),
        data::alignment_view(*alignment) == u::alignment(g@, d@),
    ensures
        h::pages(out@) == passages_spec(g@, d@),
{
    hide(u::aligned_text);
    hide(u::lit);
    let mut body = Vec::new();
    body.push(h::fixed("<section>"));
    body.push(h::fixed("<h3>Original passage</h3>"));
    body.push(
        h::cat(
            h::cat(
                h::fixed("<pre class=\"prose\">"),
                data::aligned(g, d, source, alignment, false),
            ),
            h::fixed("</pre>"),
        ),
    );
    body.push(h::fixed("</section>"));
    body.push(h::fixed("<section>"));
    body.push(h::fixed("<h3>Attempto Controlled English (ACE)</h3>"));
    body.push(
        h::cat(
            h::cat(h::fixed("<pre class=\"prose\">"), data::aligned(g, d, source, alignment, true)),
            h::fixed("</pre>"),
        ),
    );
    body.push(h::fixed("</section>"));
    proof {
        assert(h::pages(body@) =~= passages_spec(g@, d@));
    }
    body
}

pub fn compiled(d: &EDocument, prev: &[u8], next: &[u8]) -> (out: Vec<EPage>)
    ensures
        h::pages(out@) == compiled_spec(d@, prev@, next@),
{
    hide(u::lit);
    hide(u::splitline_count);
    let nav = navigation(prev, next);
    let mut body = Vec::new();
    body.push(h::fixed("<section>"));
    body.push(h::fixed("<details>"));
    body.push(
        h::cat(
            h::cat(h::fixed("<summary>Compiled Prolog ("), h::number(data::lines(&d.pl))),
            h::fixed(" lines)</summary>"),
        ),
    );
    body.push(h::cat(h::cat(h::fixed("<pre>"), h::text(&d.pl)), h::fixed("</pre>")));
    body.push(h::fixed("</details>"));
    body.push(h::fixed("</section>"));
    body.push(
        h::cat(
            h::cat(h::fixed("<nav class=\"docnav\">"), h::hjoin(&nav, &h::fixed(" · "))),
            h::fixed("</nav>"),
        ),
    );
    proof {
        assert(h::pages(body@) =~= compiled_spec(d@, prev@, next@));
    }
    body
}

pub fn page(g: &EGuideline, d: &EDocument, prev: &[u8], next: &[u8], token: &[u8]) -> (out: EPage)
    ensures
        out@ == u::document_html(g@, d@, prev@, next@, token@),
{
    hide(u::frame);
    hide(u::lit);
    hide(u::document_html);
    hide(body_spec);
    hide(leading_spec);
    hide(passages_spec);
    hide(form_spec);
    hide(compiled_spec);
    let m = model::prepare(g);
    let source = payload::payload(g, &d.bundle.docid);
    let alignment = data::alignment(g, d, &source);
    let shown = match &alignment {
        Some(a) => a.count > 0,
        None => false,
    };
    let mut body = leading(g, d, &m, shown);
    let mut part = passages(g, d, &source, &alignment);
    let ghost first = h::pages(body@);
    let ghost second = h::pages(part@);
    body.append(&mut part);
    proof {
        assert(h::pages(body@) =~= first + second);
    }
    let mut form = form(g, d, &m, token);
    let ghost first = h::pages(body@);
    let ghost second = h::pages(form@);
    body.append(&mut form);
    proof {
        assert(h::pages(body@) =~= first + second);
    }
    let mut tail = compiled(d, prev, next);
    let ghost first = h::pages(body@);
    let ghost second = h::pages(tail@);
    body.append(&mut tail);
    proof {
        assert(h::pages(body@) =~= first + second);
        body_groups(g@, d@, prev@, next@, token@, shown);
        decomposition(g@, d@, prev@, next@, token@);
    }
    let title = titles::document_title(g, &d.bundle.docid);
    let region = payload::coverage_field(g, &d.bundle.docid, 0);
    let crumbs = h::cat(
        h::cat(
            h::cat(
                h::fixed(
                    "<a href=\"../../../index.html\">guidelines</a> / <a href=\"../index.html\">",
                ),
                h::text(&titles::title(g)),
            ),
            h::fixed("</a> / "),
        ),
        h::text(&region),
    );
    frame::frame(&title, crumbs, h::lines(&body))
}

} // verus!
