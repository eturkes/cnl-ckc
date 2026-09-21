use crate::{
    k5_bytes as b, k5_frame as frame, k5_html as h, k5_model as model, k5_titles as titles,
    k5_url as url,
};
use ckc_spec::ui::{self as u, ECorpus, EGuideline, EPage};
use vstd::prelude::*;
verus! {

pub open spec fn row_spec(g: u::Guideline) -> u::Html {
    u::row(
        seq![
            u::cell(
                u::link(
                    u::lit("g/"@) + u::url_seg(g.gid) + u::lit("/index.html"@),
                    u::text(u::title(g)),
                ),
            ),
            u::cell(u::number(g.documents.len())),
            u::cell(u::number(g.coverage.rows.len())),
        ] + u::class_counts(g).map_values(|n: nat| u::cell(u::number(n))),
    )
}

pub fn row(g: &EGuideline) -> (out: EPage)
    ensures
        out@ == row_spec(g@),
{
    hide(u::title);
    hide(u::class_counts);
    hide(u::lit);
    let m = model::prepare(g);
    let counts = model::counts(g, &m);
    let href = b::cat(b::cat(b::literal("g/"), &url::segment(&g.gid)), &b::literal("/index.html"));
    let mut cells = Vec::new();
    cells.push(h::cell(h::link(&href, h::text(&titles::title(g)))));
    cells.push(h::cell(h::number(g.documents.len())));
    cells.push(h::cell(h::number(g.coverage.rows.len())));
    let ghost head = h::pages(cells@);
    let mut i = 0;
    proof {
        assert(head =~= seq![
            u::cell(
                u::link(
                    u::lit("g/"@) + u::url_seg(g@.gid) + u::lit("/index.html"@),
                    u::text(u::title(g@)),
                ),
            ),
            u::cell(u::number(g@.documents.len())),
            u::cell(u::number(g@.coverage.rows.len())),
        ]);
    }
    while i < counts.len()
        invariant
            i <= counts.len(),
            counts@.map_values(|n: usize| n as nat) == u::class_counts(g@),
            h::pages(cells@) == head + u::class_counts(g@).take(i as int).map_values(
                |n: nat| u::cell(u::number(n)),
            ),
        decreases counts.len() - i,
    {
        let ghost before = h::pages(cells@);
        cells.push(h::cell(h::number(counts[i])));
        proof {
            assert(h::pages(cells@) =~= before.push(u::cell(u::number(counts@[i as int] as nat))));
            assert(u::class_counts(g@).take(i as int + 1).map_values(|n: nat| u::cell(u::number(n)))
                =~= u::class_counts(g@).take(i as int).map_values(
                |n: nat| u::cell(u::number(n)),
            ).push(u::cell(u::number(counts@[i as int] as nat))));
        }
        i += 1;
    }
    proof {
        assert(u::class_counts(g@).take(i as int) =~= u::class_counts(g@));
    }
    h::row(&cells)
}

pub fn page(c: &ECorpus) -> (out: EPage)
    ensures
        out@ == u::index_html(c@),
{
    hide(u::lit);
    hide(u::title);
    hide(u::class_counts);
    let mut rows = Vec::new();
    let mut i = 0;
    while i < c.guidelines.len()
        invariant
            i <= c.guidelines.len(),
            h::pages(rows@) == c@.guidelines.take(i as int).map_values(
                |g: u::Guideline| row_spec(g),
            ),
        decreases c.guidelines.len() - i,
    {
        rows.push(row(&c.guidelines[i]));
        proof {
            assert(c@.guidelines.take(i as int + 1).map_values(|g: u::Guideline| row_spec(g))
                =~= c@.guidelines.take(i as int).map_values(|g: u::Guideline| row_spec(g)).push(
                row_spec(c@.guidelines[i as int]),
            ));
        }
        i += 1;
    }
    proof {
        assert(c@.guidelines.take(i as int) =~= c@.guidelines);
    }
    let mut body = Vec::new();
    body.push(h::fixed("<h1>Guidelines</h1>"));
    body.push(h::fixed("<section>"));
    body.push(h::fixed("<table>"));
    body.push(
        h::fixed(
            "<thead><tr><th>Guideline</th><th>Documents</th><th>Passages</th><th>Approved</th><th>Rejected</th><th>Contested</th><th>Outdated</th><th>Unreviewed</th></tr></thead>",
        ),
    );
    body.push(h::cat(h::cat(h::fixed("<tbody>"), h::lines(&rows)), h::fixed("</tbody>")));
    body.push(h::fixed("</table>"));
    body.push(h::fixed("</section>"));
    proof {
        assert(h::pages(body@) =~= seq![
            u::fixed("<h1>Guidelines</h1>"@),
            u::fixed("<section>"@),
            u::fixed("<table>"@),
            u::fixed(
                "<thead><tr><th>Guideline</th><th>Documents</th><th>Passages</th><th>Approved</th><th>Rejected</th><th>Contested</th><th>Outdated</th><th>Unreviewed</th></tr></thead>"@,
            ),
            u::fixed("<tbody>"@) + u::lines(h::pages(rows@)) + u::fixed("</tbody>"@),
            u::fixed("</table>"@),
            u::fixed("</section>"@),
        ]);
        assert(c@.guidelines.map_values(|g: u::Guideline| row_spec(g)) =~= c@.guidelines.map_values(
            |g: u::Guideline|
                u::row(
                    seq![
                        u::cell(
                            u::link(
                                u::lit("g/"@) + u::url_seg(g.gid) + u::lit("/index.html"@),
                                u::text(u::title(g)),
                            ),
                        ),
                        u::cell(u::number(g.documents.len())),
                        u::cell(u::number(g.coverage.rows.len())),
                    ] + u::class_counts(g).map_values(|n: nat| u::cell(u::number(n))),
                ),
        ));
    }
    frame::frame(&b::literal("Guidelines"), h::fixed("cnl-ckc reviewer"), h::lines(&body))
}

} // verus!
