use crate::{
    k5_bytes as b, k5_frame as frame, k5_html as h, k5_model as model, k5_payload as payload,
    k5_records as records, k5_titles as titles, k5_url as url,
};
use ckc_spec::check as ck;
use ckc_spec::check::{ECoverageRow, EStatus};
use ckc_spec::ui::{self as u, EDocument, EGuideline, EPage};
use vstd::prelude::*;
verus! {

pub open spec fn labels_spec() -> Seq<u::Bytes> {
    seq![
        u::lit("Passages"@),
        u::lit("With ACE"@),
        u::lit("Pending"@),
        u::lit("Approved"@),
        u::lit("Rejected"@),
        u::lit("Contested"@),
        u::lit("Outdated"@),
        u::lit("Unreviewed"@),
    ]
}

pub fn labels() -> (out: Vec<Vec<u8>>)
    ensures
        b::views(out@) == labels_spec(),
{
    let mut out = Vec::new();
    out.push(b::literal("Passages"));
    out.push(b::literal("With ACE"));
    out.push(b::literal("Pending"));
    out.push(b::literal("Approved"));
    out.push(b::literal("Rejected"));
    out.push(b::literal("Contested"));
    out.push(b::literal("Outdated"));
    out.push(b::literal("Unreviewed"));
    proof {
        assert(b::views(out@) =~= labels_spec());
    }
    out
}

pub open spec fn status_pred(r: ck::Row, k: u8) -> bool {
    match r.status {
        ck::Status::Pending => k == 0,
        ck::Status::Ace(_) => k == 1,
        ck::Status::Restates(_) => k == 2,
        ck::Status::Uncovered => k == 3,
    }
}

pub fn coverage_count(g: &EGuideline, k: u8) -> (n: usize)
    ensures
        n as nat == ck::count_status(g@.coverage.rows, k as int),
{
    let mut i = 0;
    let mut n = 0;
    while i < g.coverage.rows.len()
        invariant
            i <= g.coverage.rows.len(),
            n <= i,
            n as nat == g@.coverage.rows.take(i as int).filter(
                |r: ck::Row| status_pred(r, k),
            ).len(),
        decreases g.coverage.rows.len() - i,
    {
        let r = &g.coverage.rows[i];
        let matches = match &r.status {
            EStatus::Pending => k == 0,
            EStatus::Ace(_) => k == 1,
            EStatus::Restates(_) => k == 2,
            EStatus::Uncovered => k == 3,
        };
        if matches {
            n += 1;
        }
        proof {
            let before = g@.coverage.rows.take(i as int);
            assert(g@.coverage.rows.take(i as int + 1) =~= before.push(r@));
            before.lemma_filter_push(r@, |r: ck::Row| status_pred(r, k));
        }
        i += 1;
    }
    proof {
        assert(g@.coverage.rows.take(i as int) =~= g@.coverage.rows);
        assert((|r: ck::Row| status_pred(r, k)) =~= (|r: ck::Row|
            match r.status {
                ck::Status::Pending => k as int == 0,
                ck::Status::Ace(_) => k as int == 1,
                ck::Status::Restates(_) => k as int == 2,
                ck::Status::Uncovered => k as int == 3,
            }));
    }
    n
}

pub open spec fn counts_spec(g: u::Guideline) -> Seq<nat> {
    seq![
        g.coverage.rows.len(),
        ck::count_status(g.coverage.rows, 1),
        ck::count_status(g.coverage.rows, 0),
    ] + u::class_counts(g)
}

pub open spec fn status_row_spec(g: u::Guideline, i: int) -> u::Html {
    u::row(
        seq![
            u::cell(u::text(labels_spec()[i])),
            u::cell(
                u::number(
                    if i < counts_spec(g).len() {
                        counts_spec(g)[i]
                    } else {
                        0
                    },
                ),
            ),
        ],
    )
}

pub fn status_rows(g: &EGuideline, m: &model::Model) -> (out: Vec<EPage>)
    requires
        model::bound(m, g@),
    ensures
        h::pages(out@) == Seq::new(labels_spec().len(), |i: int| status_row_spec(g@, i)),
{
    let labels = labels();
    let mut counts = Vec::new();
    counts.push(g.coverage.rows.len());
    counts.push(coverage_count(g, 1));
    counts.push(coverage_count(g, 0));
    let mut more = model::counts(g, m);
    counts.append(&mut more);
    proof {
        assert(counts@.map_values(|n: usize| n as nat) =~= counts_spec(g@));
    }
    let mut out = Vec::new();
    let mut i = 0;
    while i < labels.len()
        invariant
            i <= labels.len(),
            b::views(labels@) == labels_spec(),
            counts@.map_values(|n: usize| n as nat) == counts_spec(g@),
            h::pages(out@) == Seq::new(i as nat, |j: int| status_row_spec(g@, j)),
        decreases labels.len() - i,
    {
        let mut cells = Vec::new();
        cells.push(h::cell(h::text(&labels[i])));
        cells.push(
            h::cell(
                h::number(
                    if i < counts.len() {
                        counts[i]
                    } else {
                        0
                    },
                ),
            ),
        );
        proof {
            assert(h::pages(cells@) =~= seq![
                u::cell(u::text(labels_spec()[i as int])),
                u::cell(
                    u::number(
                        if i < counts_spec(g@).len() {
                            counts_spec(g@)[i as int]
                        } else {
                            0
                        },
                    ),
                ),
            ]);
        }
        let ghost before = h::pages(out@);
        out.push(h::row(&cells));
        proof {
            assert(h::pages(out@) =~= before.push(status_row_spec(g@, i as int)));
            assert(Seq::new(i as nat + 1, |j: int| status_row_spec(g@, j)) =~= before.push(
                status_row_spec(g@, i as int),
            ));
        }
        i += 1;
    }
    out
}

pub open spec fn doc_row_spec(g: u::Guideline, d: u::Document) -> u::Html {
    let id = d.bundle.docid;
    u::row(
        seq![
            u::cell(
                u::link(
                    u::lit("doc/"@) + u::url_seg(id) + u::lit(".html"@),
                    u::text(u::document_title(g, id)),
                ),
            ),
            u::cell(u::chip(u::state(g, id))),
            u::cell(u::text(u::tally_cell(u::tally(g, id)))),
            u::cell(u::text(u::coverage_field(g, id, 0))),
        ],
    )
}

pub fn doc_row(g: &EGuideline, m: &model::Model, d: &EDocument) -> (out: EPage)
    requires
        model::bound(m, g@),
    ensures
        out@ == doc_row_spec(g@, d@),
{
    hide(u::lit);
    hide(u::document_title);
    hide(u::state);
    hide(u::tally);
    hide(u::tally_cell);
    hide(u::coverage_field);
    let id = &d.bundle.docid;
    let href = b::cat(b::cat(b::literal("doc/"), &url::segment(id)), &b::literal(".html"));
    let mut cells = Vec::new();
    cells.push(h::cell(h::link(&href, h::text(&titles::document_title(g, id)))));
    cells.push(h::cell(frame::chip(model::state(g, m, id))));
    cells.push(h::cell(h::text(&records::tally_cell(records::tally(g, &m.records, id)))));
    cells.push(h::cell(h::text(&payload::coverage_field(g, id, 0))));
    proof {
        assert(h::pages(cells@) =~= seq![
            u::cell(
                u::link(
                    u::lit("doc/"@) + u::url_seg(id@) + u::lit(".html"@),
                    u::text(u::document_title(g@, id@)),
                ),
            ),
            u::cell(u::chip(u::state(g@, id@))),
            u::cell(u::text(u::tally_cell(u::tally(g@, id@)))),
            u::cell(u::text(u::coverage_field(g@, id@, 0))),
        ]);
    }
    h::row(&cells)
}

pub fn doc_rows(g: &EGuideline, m: &model::Model) -> (out: Vec<EPage>)
    requires
        model::bound(m, g@),
    ensures
        h::pages(out@) == g@.documents.map_values(|d: u::Document| doc_row_spec(g@, d)),
{
    let mut out = Vec::new();
    let mut i = 0;
    while i < g.documents.len()
        invariant
            model::bound(m, g@),
            i <= g.documents.len(),
            h::pages(out@) == g@.documents.take(i as int).map_values(
                |d: u::Document| doc_row_spec(g@, d),
            ),
        decreases g.documents.len() - i,
    {
        out.push(doc_row(g, m, &g.documents[i]));
        proof {
            assert(g@.documents.take(i as int + 1).map_values(|d: u::Document| doc_row_spec(g@, d))
                =~= g@.documents.take(i as int).map_values(
                |d: u::Document| doc_row_spec(g@, d),
            ).push(doc_row_spec(g@, g@.documents[i as int])));
        }
        i += 1;
    }
    proof {
        assert(g@.documents.take(i as int) =~= g@.documents);
    }
    out
}

pub open spec fn other_row_spec(r: ck::Row) -> u::Html {
    u::row(
        seq![
            u::cell(u::text(r.id)),
            u::cell(u::text(u::region_status(r))),
            u::cell(u::text(u::field(r, 3))),
        ],
    )
}

pub fn other_row(r: &ECoverageRow) -> (out: EPage)
    ensures
        out@ == other_row_spec(r@),
{
    hide(u::field);
    hide(u::region_status);
    let mut cells = Vec::new();
    cells.push(h::cell(h::text(&r.id)));
    cells.push(h::cell(h::text(&titles::region_status(r))));
    cells.push(h::cell(h::text(&payload::field(r, 3))));
    proof {
        assert(h::pages(cells@) =~= seq![
            u::cell(u::text(r@.id)),
            u::cell(u::text(u::region_status(r@))),
            u::cell(u::text(u::field(r@, 3))),
        ]);
    }
    h::row(&cells)
}

pub fn others(g: &EGuideline) -> (out: Vec<EPage>)
    ensures
        h::pages(out@) == g@.coverage.rows.filter(
            |r: ck::Row| !matches!(r.status,ck::Status::Ace(_)),
        ).map_values(|r: ck::Row| other_row_spec(r)),
{
    let mut out = Vec::new();
    let mut i = 0;
    while i < g.coverage.rows.len()
        invariant
            i <= g.coverage.rows.len(),
            h::pages(out@) == g@.coverage.rows.take(i as int).filter(
                |r: ck::Row| !matches!(r.status,ck::Status::Ace(_)),
            ).map_values(|r: ck::Row| other_row_spec(r)),
        decreases g.coverage.rows.len() - i,
    {
        let r = &g.coverage.rows[i];
        let show = match &r.status {
            EStatus::Ace(_) => false,
            _ => true,
        };
        if show {
            let ghost prior = h::pages(out@);
            out.push(other_row(r));
            proof {
                assert(h::pages(out@) =~= prior.push(other_row_spec(r@)));
            }
        }
        proof {
            let before = g@.coverage.rows.take(i as int);
            assert(g@.coverage.rows.take(i as int + 1) =~= before.push(r@));
            before.lemma_filter_push(r@, |r: ck::Row| !matches!(r.status,ck::Status::Ace(_)));
            let selected = before.filter(|r: ck::Row| !matches!(r.status,ck::Status::Ace(_)));
            assert(selected.push(r@).map_values(|r: ck::Row| other_row_spec(r))
                =~= selected.map_values(|r: ck::Row| other_row_spec(r)).push(other_row_spec(r@)));
        }
        i += 1;
    }
    proof {
        assert(g@.coverage.rows.take(i as int) =~= g@.coverage.rows);
    }
    out
}

pub fn page(g: &EGuideline) -> (out: EPage)
    ensures
        out@ == u::guideline_html(g@),
{
    hide(u::lit);
    hide(u::title);
    hide(u::review_summary);
    hide(u::document_title);
    hide(u::region_status);
    hide(u::state);
    hide(u::tally);
    hide(u::class_counts);
    let m = model::prepare(g);
    let title = titles::title(g);
    let stats = status_rows(g, &m);
    let docs = doc_rows(g, &m);
    let other = others(g);
    proof {
        let labels = labels_spec();
        let counts = counts_spec(g@);
        assert(h::pages(stats@) =~= Seq::new(
            labels.len(),
            |i: int|
                u::row(
                    seq![
                        u::cell(u::text(labels[i])),
                        u::cell(
                            u::number(
                                if i < counts.len() {
                                    counts[i]
                                } else {
                                    0
                                },
                            ),
                        ),
                    ],
                ),
        ));
        assert(h::pages(docs@) =~= g@.documents.map_values(
            |d: u::Document|
                {
                    let id = d.bundle.docid;
                    u::row(
                        seq![
                            u::cell(
                                u::link(
                                    u::lit("doc/"@) + u::url_seg(id) + u::lit(".html"@),
                                    u::text(u::document_title(g@, id)),
                                ),
                            ),
                            u::cell(u::chip(u::state(g@, id))),
                            u::cell(u::text(u::tally_cell(u::tally(g@, id)))),
                            u::cell(u::text(u::coverage_field(g@, id, 0))),
                        ],
                    )
                },
        ));
        assert(h::pages(other@) =~= g@.coverage.rows.filter(
            |r: ck::Row| !matches!(r.status,ck::Status::Ace(_)),
        ).map_values(
            |r: ck::Row|
                u::row(
                    seq![
                        u::cell(u::text(r.id)),
                        u::cell(u::text(u::region_status(r))),
                        u::cell(u::text(u::field(r, 3))),
                    ],
                ),
        ));
    }
    let mut body = Vec::new();
    body.push(h::cat(h::cat(h::fixed("<h1>"), h::text(&title)), h::fixed("</h1>")));
    body.push(
        h::cat(
            h::cat(h::fixed("<p>"), h::text(&model::summary(g, &m))),
            h::fixed(" <a href=\"records.html\">All decision records</a></p>"),
        ),
    );
    body.push(h::fixed("<section>"));
    body.push(h::fixed("<h2>Status</h2>"));
    body.push(h::fixed("<table class=\"compact\">"));
    body.push(h::fixed("<thead><tr><th>Status</th><th>Count</th></tr></thead>"));
    body.push(h::cat(h::cat(h::fixed("<tbody>"), h::lines(&stats)), h::fixed("</tbody>")));
    body.push(h::fixed("</table>"));
    body.push(h::fixed("</section>"));
    body.push(h::fixed("<section>"));
    body.push(h::fixed("<h2>Documents</h2>"));
    body.push(h::fixed("<table class=\"compact\">"));
    body.push(
        h::fixed(
            "<thead><tr><th>Document</th><th>Status</th><th>Decisions</th><th>Passage</th></tr></thead>",
        ),
    );
    body.push(h::cat(h::cat(h::fixed("<tbody>"), h::lines(&docs)), h::fixed("</tbody>")));
    body.push(h::fixed("</table>"));
    body.push(h::fixed("</section>"));
    body.push(h::fixed("<section>"));
    body.push(h::fixed("<h2>Passages without ACE</h2>"));
    body.push(h::fixed("<table class=\"compact\">"));
    body.push(h::fixed("<thead><tr><th>Passage</th><th>Status</th><th>Section</th></tr></thead>"));
    body.push(h::cat(h::cat(h::fixed("<tbody>"), h::lines(&other)), h::fixed("</tbody>")));
    body.push(h::fixed("</table>"));
    body.push(h::fixed("</section>"));
    proof {
        assert(h::pages(body@) =~= seq![
            u::fixed("<h1>"@) + u::text(title@) + u::fixed("</h1>"@),
            u::fixed("<p>"@) + u::text(u::review_summary(g@)) + u::fixed(
                " <a href=\"records.html\">All decision records</a></p>"@,
            ),
            u::fixed("<section>"@),
            u::fixed("<h2>Status</h2>"@),
            u::fixed("<table class=\"compact\">"@),
            u::fixed("<thead><tr><th>Status</th><th>Count</th></tr></thead>"@),
            u::fixed("<tbody>"@) + u::lines(h::pages(stats@)) + u::fixed("</tbody>"@),
            u::fixed("</table>"@),
            u::fixed("</section>"@),
            u::fixed("<section>"@),
            u::fixed("<h2>Documents</h2>"@),
            u::fixed("<table class=\"compact\">"@),
            u::fixed(
                "<thead><tr><th>Document</th><th>Status</th><th>Decisions</th><th>Passage</th></tr></thead>"@,
            ),
            u::fixed("<tbody>"@) + u::lines(h::pages(docs@)) + u::fixed("</tbody>"@),
            u::fixed("</table>"@),
            u::fixed("</section>"@),
            u::fixed("<section>"@),
            u::fixed("<h2>Passages without ACE</h2>"@),
            u::fixed("<table class=\"compact\">"@),
            u::fixed("<thead><tr><th>Passage</th><th>Status</th><th>Section</th></tr></thead>"@),
            u::fixed("<tbody>"@) + u::lines(h::pages(other@)) + u::fixed("</tbody>"@),
            u::fixed("</table>"@),
            u::fixed("</section>"@),
        ]);
    }
    frame::frame(
        &title,
        h::cat(h::fixed("<a href=\"../../index.html\">guidelines</a> / "), h::text(&title)),
        h::lines(&body),
    )
}

} // verus!
