use crate::{
    k5_bytes as b, k5_frame as frame, k5_html as h, k5_model as model, k5_records as r,
    k5_titles as titles, k5_url as url,
};
use ckc_spec::ui::{self as u, EGuideline, EPage, ERecord};
use vstd::prelude::*;
verus! {

pub fn section(g: &EGuideline, m: &model::Model, id: &[u8]) -> (out: Vec<EPage>)
    requires
        model::bound(m, g@),
    ensures
        h::pages(out@) == u::record_section(g@, id@),
{
    hide(u::document_title);
    hide(u::record_row);
    hide(u::lit);
    let hs = r::history(&m.records, id);
    if hs.len() == 0 {
        return Vec::new();
    }
    let mut rows = Vec::new();
    let mut i = 0;
    while i < hs.len()
        invariant
            i <= hs.len(),
            r::refs(hs@) == u::history(g@, id@),
            h::pages(rows@) == Seq::new(
                i as nat,
                |j: int| u::record_row(g@, u::history(g@, id@)[hs.len() as int - j - 1]),
            ),
        decreases hs.len() - i,
    {
        let rec = hs[hs.len() - i - 1];
        let ghost before = h::pages(rows@);
        rows.push(r::record_row(g, rec));
        proof {
            assert(h::pages(rows@) =~= before.push(u::record_row(g@, rec@)));
            assert(Seq::new(
                i as nat + 1,
                |j: int| u::record_row(g@, u::history(g@, id@)[hs.len() as int - j - 1]),
            ) =~= before.push(u::record_row(g@, rec@)));
        }
        i += 1;
    }
    let href = b::cat(b::cat(b::literal("doc/"), &url::segment(id)), &b::literal(".html"));
    let mut out = Vec::new();
    out.push(h::cat(h::cat(h::fixed("<section id=\""), h::attr(id)), h::fixed("\">")));
    out.push(
        h::cat(
            h::cat(h::fixed("<h2>"), h::link(&href, h::text(&titles::document_title(g, id)))),
            h::fixed("</h2>"),
        ),
    );
    out.push(h::fixed("<table class=\"records\">"));
    out.push(
        h::fixed(
            "<thead><tr><th>Decision</th><th>Reviewer</th><th>Date</th><th>Version</th><th>Comment</th></tr></thead>",
        ),
    );
    out.push(h::cat(h::cat(h::fixed("<tbody>"), h::lines(&rows)), h::fixed("</tbody>")));
    out.push(h::fixed("</table>"));
    out.push(h::fixed("</section>"));
    proof {
        assert(h::pages(out@) =~= seq![
            u::fixed("<section id=\""@) + u::attr(id@) + u::fixed("\">"@),
            u::fixed("<h2>"@) + u::link(
                u::lit("doc/"@) + u::url_seg(id@) + u::lit(".html"@),
                u::text(u::document_title(g@, id@)),
            ) + u::fixed("</h2>"@),
            u::fixed("<table class=\"records\">"@),
            u::fixed(
                "<thead><tr><th>Decision</th><th>Reviewer</th><th>Date</th><th>Version</th><th>Comment</th></tr></thead>"@,
            ),
            u::fixed("<tbody>"@) + u::lines(h::pages(rows@)) + u::fixed("</tbody>"@),
            u::fixed("</table>"@),
            u::fixed("</section>"@),
        ]);
    }
    out
}

pub fn commit_count(rs: &Vec<ERecord>) -> (n: usize)
    ensures
        n as nat == r::records(rs@).filter(|r: u::Record| r.decision.commit.len() > 0).len(),
{
    let mut n = 0;
    let mut i = 0;
    while i < rs.len()
        invariant
            i <= rs.len(),
            n <= i,
            n as nat == r::records(rs@).take(i as int).filter(
                |r: u::Record| r.decision.commit.len() > 0,
            ).len(),
        decreases rs.len() - i,
    {
        let rec = &rs[i];
        if rec.commit.len() > 0 {
            n += 1;
        }
        proof {
            let before = r::records(rs@).take(i as int);
            assert(r::records(rs@).take(i as int + 1) =~= before.push(rec@));
            before.lemma_filter_push(rec@, |r: u::Record| r.decision.commit.len() > 0);
        }
        i += 1;
    }
    proof {
        assert(r::records(rs@).take(i as int) =~= r::records(rs@));
    }
    n
}

pub fn page(g: &EGuideline) -> (out: EPage)
    ensures
        out@ == u::records_html(g@),
{
    hide(u::lit);
    hide(u::title);
    hide(u::record_section);
    hide(u::review_summary);
    let m = model::prepare(g);
    let mut sections = Vec::new();
    let mut i = 0;
    while i < g.documents.len()
        invariant
            model::bound(&m, g@),
            i <= g.documents.len(),
            h::pages(sections@) == g@.documents.take(i as int).map_values(
                |d: u::Document| u::record_section(g@, d.bundle.docid),
            ).flatten(),
        decreases g.documents.len() - i,
    {
        let id = &g.documents[i].bundle.docid;
        let mut part = section(g, &m, id);
        let ghost before = h::pages(sections@);
        let ghost pv = h::pages(part@);
        sections.append(&mut part);
        proof {
            assert(h::pages(sections@) =~= before + pv);
            let previous = g@.documents.take(i as int).map_values(
                |d: u::Document| u::record_section(g@, d.bundle.docid),
            );
            assert(g@.documents.take(i as int + 1).map_values(
                |d: u::Document| u::record_section(g@, d.bundle.docid),
            ) =~= previous.push(u::record_section(g@, id@)));
            previous.lemma_flatten_push(u::record_section(g@, id@));
        }
        i += 1;
    }
    proof {
        assert(g@.documents.take(i as int) =~= g@.documents);
    }
    let empty = sections.len() == 0;
    let ghost sv = h::pages(sections@);
    let mut summary = model::summary(g, &m);
    if m.decisions.len() > 0 {
        b::append(&mut summary, &b::literal(" The newest decision for each document is first."));
    }
    let mut notes = Vec::new();
    if empty {
        notes.push(h::fixed("<p>Open a document and record a decision to start this list.</p>"));
    } else {
        notes.push(
            h::fixed("<p>Each reviewer name is recorded as entered and is not verified.</p>"),
        );
        if commit_count(&m.records) > 0 {
            notes.push(
                h::fixed(
                    "<p>Each version links to the stored version of the text that the reviewer read.</p>",
                ),
            );
        }
    }
    let ghost nv = h::pages(notes@);
    proof {
        assert(sv == g@.documents.map_values(
            |d: u::Document| u::record_section(g@, d.bundle.docid),
        ).flatten());
        assert(summary@ == u::review_summary(g@) + (if u::decisions(g@).len() > 0 {
            u::lit(" The newest decision for each document is first."@)
        } else {
            u::empty()
        }));
        assert(nv =~= if sv.len() == 0 {
            seq![u::fixed("<p>Open a document and record a decision to start this list.</p>"@)]
        } else {
            seq![u::fixed("<p>Each reviewer name is recorded as entered and is not verified.</p>"@)]
                + (if u::records(g@).filter(|r: u::Record| r.decision.commit.len() > 0).len() > 0 {
                seq![
                    u::fixed(
                        "<p>Each version links to the stored version of the text that the reviewer read.</p>"@,
                    ),
                ]
            } else {
                Seq::empty()
            })
        });
    }
    let mut body = Vec::new();
    body.push(h::fixed("<h1>Decision records</h1>"));
    body.push(h::cat(h::cat(h::fixed("<p>"), h::text(&summary)), h::fixed("</p>")));
    body.append(&mut sections);
    body.append(&mut notes);
    body.push(h::fixed("<nav class=\"docnav\"><a href=\"index.html\">Guideline index</a></nav>"));
    proof {
        assert(h::pages(body@) =~= seq![
            u::fixed("<h1>Decision records</h1>"@),
            u::fixed("<p>"@) + u::text(summary@) + u::fixed("</p>"@),
        ] + sv + nv + seq![
            u::fixed("<nav class=\"docnav\"><a href=\"index.html\">Guideline index</a></nav>"@),
        ]);
    }
    let crumbs = h::cat(
        h::cat(
            h::fixed("<a href=\"../../index.html\">guidelines</a> / <a href=\"index.html\">"),
            h::text(&titles::title(g)),
        ),
        h::fixed("</a> / records"),
    );
    frame::frame(&b::literal("Decision records"), crumbs, h::lines(&body))
}

} // verus!
