use crate::{
    k5_bytes as b, k5_highlight as hl, k5_html as h, k5_model as model, k5_payload as payload,
    k5_titles as titles, k5_url as url, m7_temporal as vocabulary,
};
use ckc_spec::{
    check as ck,
    ui::{self as u, EGuideline, EPage},
};
use vstd::prelude::*;
use vstd::slice::slice_to_vec;
verus! {

pub struct ESuperseded {
    pub document: bool,
    pub key: Vec<u8>,
    pub date: Vec<u8>,
    pub url: Vec<u8>,
    pub citation: Vec<u8>,
}

impl View for ESuperseded {
    type V = u::Superseded;

    open spec fn view(&self) -> u::Superseded {
        u::Superseded {
            document: self.document,
            key: self.key@,
            date: self.date@,
            url: self.url@,
            citation: self.citation@,
        }
    }
}

pub open spec fn rows_view(rs: Seq<ESuperseded>) -> Seq<u::Superseded> {
    rs.map_values(|r: ESuperseded| r@)
}

pub open spec fn opt_view(r: Option<ESuperseded>) -> Option<u::Superseded> {
    match r {
        Some(r) => Some(r@),
        None => None,
    }
}

pub fn row(line: &[u8]) -> (out: Option<ESuperseded>)
    ensures
        opt_view(out) == u::superseded_row(line@),
        out matches Some(r) ==> r.date.len() == 8,
{
    let f = b::split(line, 9);
    if f.len() != 3 {
        return None;
    }
    let document = b::equal(&f[0], &b::literal("supersession"));
    if !document && !b::equal(&f[0], &b::literal("superseded")) {
        return None;
    }
    if f[2].len() < 9 {
        return None;
    }
    let value = &f[2];
    let rest = &value[9..value.len()];
    let n = b::first_byte(rest, 32, 0);
    let citation = if n < rest.len() {
        slice_to_vec(&rest[n + 1..rest.len()])
    } else {
        Vec::new()
    };
    Some(
        ESuperseded {
            document,
            key: slice_to_vec(&f[1]),
            date: slice_to_vec(&value[0..8]),
            url: slice_to_vec(&rest[0..n]),
            citation,
        },
    )
}

pub fn scan(lines: &Vec<Vec<u8>>, n: usize) -> (out: Vec<ESuperseded>)
    requires
        n <= lines.len(),
    ensures
        rows_view(out@) == u::superseded_scan(b::views(lines@).take(n as int)),
        forall|j: int| 0 <= j < out.len() ==> (#[trigger] out@[j]).date.len() == 8,
{
    hide(u::superseded_row);
    let ghost ls = b::views(lines@).take(n as int);
    let mut out: Vec<ESuperseded> = Vec::new();
    let mut i = 0usize;
    proof {
        assert(ls.skip(0) =~= ls);
        assert(rows_view(out@) =~= Seq::<u::Superseded>::empty());
    }
    while i < n
        invariant
            i <= n <= lines.len(),
            ls == b::views(lines@).take(n as int),
            rows_view(out@) + u::superseded_scan(ls.skip(i as int)) == u::superseded_scan(ls),
            forall|j: int| 0 <= j < out.len() ==> (#[trigger] out@[j]).date.len() == 8,
        decreases n - i,
    {
        let ghost before = rows_view(out@);
        let r = row(&lines[i]);
        proof {
            assert(ls.skip(i as int).drop_first() =~= ls.skip(i as int + 1));
        }
        match r {
            Some(r) => {
                let ghost rv = r@;
                out.push(r);
                proof {
                    assert(rows_view(out@) =~= before.push(rv));
                }
            },
            None => {},
        }
        i += 1;
    }
    out
}

pub proof fn header_bound(raw: u::Bytes)
    requires
        ckc_spec::vocabulary::parse_vocabulary(raw).is_ok(),
    ensures
        ckc_spec::vocabulary::vocabulary_header().len() <= raw.len(),
{
}

pub fn rows(g: &EGuideline) -> (out: Vec<ESuperseded>)
    ensures
        rows_view(out@) == u::superseded_rows(g@.vocabulary),
        forall|j: int| 0 <= j < out.len() ==> (#[trigger] out@[j]).date.len() == 8,
{
    hide(ckc_spec::vocabulary::parse_vocabulary);
    hide(u::superseded_scan);
    let raw = match &g.vocabulary {
        Some(raw) => raw,
        None => return Vec::new(),
    };
    match vocabulary::parse_vocabulary_exec(raw) {
        Err(_) => Vec::new(),
        Ok(_) => {
            let header = b::literal("# format: kind\tlemma\tvalue\n");
            proof {
                reveal_strlit("# format: kind\tlemma\tvalue\n");
                vstd::utf8::is_ascii_chars_encode_utf8("# format: kind\tlemma\tvalue\n"@);
                assert(header@ =~= ckc_spec::vocabulary::vocabulary_header());
                header_bound(raw@);
            }
            let body = &raw [header.len()..raw.len()];
            let lines = b::split(body, 10);
            proof {
                reveal_with_fuel(ck::split_on, 1);
                assert(lines.len() > 0);
                assert(b::views(lines@).take(lines.len() - 1) =~= ck::body_lines(body@));
            }
            scan(&lines, lines.len() - 1)
        },
    }
}

pub fn iso_date(date: &[u8]) -> (out: EPage)
    requires
        date.len() >= 8,
    ensures
        out@ == u::iso_date(date@),
{
    h::cat(
        h::cat(
            h::cat(h::cat(h::text(&date[0..4]), h::fixed("-")), h::text(&date[4..6])),
            h::fixed("-"),
        ),
        h::text(&date[6..8]),
    )
}

pub fn source_link(r: &ESuperseded) -> (out: EPage)
    ensures
        out@ == u::source_link(r@),
{
    h::link(&r.url, h::text(&r.citation))
}

pub fn first(rs: &Vec<ESuperseded>, id: &[u8]) -> (out: Option<usize>)
    ensures
        match out {
            Some(i) => i < rs.len() && rows_view(rs@).filter(
                |r: u::Superseded| r.document && r.key == id@,
            ).len() > 0 && rs@[i as int]@ == rows_view(rs@).filter(
                |r: u::Superseded| r.document && r.key == id@,
            )[0],
            None => rows_view(rs@).filter(|r: u::Superseded| r.document && r.key == id@).len() == 0,
        },
{
    let ghost full = rows_view(rs@);
    let ghost pred = |r: u::Superseded| r.document && r.key == id@;
    let mut i = 0usize;
    proof {
        assert(full.skip(0) =~= full);
    }
    while i < rs.len()
        invariant
            i <= rs.len(),
            full == rows_view(rs@),
            pred == (|r: u::Superseded| r.document && r.key == id@),
            full.filter(pred) == full.skip(i as int).filter(pred),
        decreases rs.len() - i,
    {
        proof {
            let tail = full.skip(i as int + 1);
            assert(full.skip(i as int) =~= seq![full[i as int]] + tail);
            tail.lemma_filter_prepend(full[i as int], pred);
        }
        if rs[i].document && b::equal(&rs[i].key, id) {
            return Some(i);
        }
        i += 1;
    }
    None
}

pub fn notice(g: &EGuideline, id: &[u8]) -> (out: Vec<EPage>)
    ensures
        h::pages(out@) == u::superseded_notice(g@, id@),
{
    hide(u::superseded_rows);
    hide(u::iso_date);
    hide(u::source_link);
    hide(u::lit);
    let rs = rows(g);
    let i = match first(&rs, id) {
        Some(i) => i,
        None => return Vec::new(),
    };
    let r = &rs[i];
    let mut out = Vec::new();
    out.push(h::fixed("<section class=\"superseded\">"));
    out.push(
        h::cat(
            h::cat(
                h::cat(h::cat(h::fixed("<p>On "), iso_date(&r.date)), h::fixed(", ")),
                source_link(r),
            ),
            h::fixed(
                " superseded the guideline statements that this page represents. This page shows the original passage and its ACE representation for review.</p>",
            ),
        ),
    );
    out.push(h::fixed("</section>"));
    proof {
        assert(h::pages(out@) =~= u::superseded_notice(g@, id@));
    }
    out
}

pub fn cells(g: &EGuideline, r: &ESuperseded, ids: &Vec<Vec<u8>>) -> (out: EPage)
    requires
        r.date.len() >= 8,
        b::views(ids@) == u::docids(g@),
    ensures
        out@ == u::superseded_cells(g@, r@),
{
    hide(u::coverage_field);
    hide(u::document_title);
    hide(u::iso_date);
    hide(u::source_link);
    hide(u::lit);
    let passage = if r.document {
        payload::coverage_field(g, &r.key, 0)
    } else {
        slice_to_vec(&r.key)
    };
    let document = if r.document && hl::contains(ids, &r.key) {
        let href = b::cat(b::cat(b::literal("doc/"), &url::segment(&r.key)), &b::literal(".html"));
        h::link(&href, h::text(&titles::document_title(g, &r.key)))
    } else {
        h::empty()
    };
    let mut cells = Vec::new();
    cells.push(h::cell(h::text(&passage)));
    cells.push(h::cell(document));
    cells.push(h::cell(source_link(r)));
    cells.push(h::cell(iso_date(&r.date)));
    proof {
        let p = if r.document {
            u::coverage_field(g@, r@.key, 0)
        } else {
            r@.key
        };
        let d = if r.document && u::docids(g@).contains(r@.key) {
            u::link(
                u::document_dir() + u::url_seg(r@.key) + u::html_ext(),
                u::text(u::document_title(g@, r@.key)),
            )
        } else {
            Seq::empty()
        };
        assert(h::pages(cells@) =~= seq![
            u::cell(u::text(p)),
            u::cell(d),
            u::cell(u::source_link(r@)),
            u::cell(u::iso_date(r@.date)),
        ]);
    }
    h::row(&cells)
}

pub fn section(g: &EGuideline) -> (out: Vec<EPage>)
    ensures
        h::pages(out@) == u::superseded_section(g@),
{
    hide(u::superseded_rows);
    hide(u::superseded_cells);
    hide(u::lit);
    let rs = rows(g);
    if rs.len() == 0 {
        return Vec::new();
    }
    let ids = model::docids(g);
    let ghost rv = rows_view(rs@);
    let ghost f = |r: u::Superseded| u::superseded_cells(g@, r);
    let mut table: Vec<EPage> = Vec::new();
    let mut i = 0usize;
    while i < rs.len()
        invariant
            i <= rs.len(),
            rv == rows_view(rs@),
            f == (|r: u::Superseded| u::superseded_cells(g@, r)),
            b::views(ids@) == u::docids(g@),
            forall|j: int| 0 <= j < rs.len() ==> (#[trigger] rs@[j]).date.len() == 8,
            h::pages(table@) == rv.take(i as int).map_values(f),
        decreases rs.len() - i,
    {
        let ghost before = h::pages(table@);
        table.push(cells(g, &rs[i], &ids));
        proof {
            assert(h::pages(table@) =~= before.push(f(rv[i as int])));
            assert(rv.take(i as int + 1).map_values(f) =~= before.push(f(rv[i as int])));
        }
        i += 1;
    }
    proof {
        assert(rv.take(i as int) =~= rv);
    }
    let mut out = Vec::new();
    out.push(h::fixed("<section>"));
    out.push(h::fixed("<h2>Superseded passages</h2>"));
    out.push(
        h::fixed(
            "<p>Later law, regulation or guidance superseded all or part of each passage in this table.</p>",
        ),
    );
    out.push(h::fixed("<table class=\"compact\">"));
    out.push(
        h::fixed(
            "<thead><tr><th>Passage</th><th>Document</th><th>Superseded by</th><th>Superseded on</th></tr></thead>",
        ),
    );
    out.push(h::cat(h::cat(h::fixed("<tbody>"), h::lines(&table)), h::fixed("</tbody>")));
    out.push(h::fixed("</table>"));
    out.push(h::fixed("</section>"));
    proof {
        assert(h::pages(out@) =~= u::superseded_section(g@));
    }
    out
}

} // verus!
