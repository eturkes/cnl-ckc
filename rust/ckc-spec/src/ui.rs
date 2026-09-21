// M5.5 K5 proposal. tools/ui.py: esc_text/esc_attr 35–38; title helpers 297–384;
// build_doc_states/build_guideline_model 568–952; hl_*/page_html/build_*_page 1083–1766;
// parse_form_fields/handle_verdict_post/respond 2256–2654; goal.py copy 3893–3951.
// Claims: corpus-bound displayed fields; exact deterministic page bytes;
// escaped dynamic slots; registry-bound fixed copy; ordered first refusal;
// seven-column insertion preserving every prior decision row.
// Shell: committed git intake + live ledger overlay, UTF-8/file intake,
// HTTP request parsing, sockets/headers, clock/token, hash computations,
// CSP digest, temporary files/fsync/lock/CAS/rename, asset/output writes,
// double-render probe + process exit codes. The script bytes belong to render.
// R76: copy domain = ASCII + U+00A7 section sign, U+00B7 middle dot,
// U+2014 em dash, U+2265 greater-than-or-equal. Each added scalar is non-word;
// ASCII folding/word boundaries equal Python regex behavior on this domain.
// Emoji gets its legacy violation first; other non-ASCII gets a domain violation.
// Controlled registry + copy fixtures contain no non-ASCII letters. Registry
// non-ASCII scalars = U+00B7/U+2014 only; no other emitted-copy scalars occur.
// Ten ASCII-only fixtures: copy-clean, copy-css-animation, copy-css-backdrop,
// copy-css-boxshadow, copy-css-gradient, copy-css-keyframes, copy-css-transition,
// copy-exclamatory, copy-marketing, copy-relative-time. Two non-ASCII fixtures:
// copy-lookalike adds U+00A7/U+00B7/U+2265 punctuation; copy-emoji tests U+2705.
// Invented-acronym judgment stays an unmechanized design rule.
use crate::align;
use crate::check::{self, Bundle, Coverage, Decision, ECoverage, Row, Status};
use crate::engine::*;
use crate::replay::Src;
use crate::v1text::{self, *};
use vstd::prelude::*;
use vstd::utf8::*;

verus! {

pub type Bytes = Seq<u8>;

pub open spec fn lit(s: Seq<char>) -> Bytes {
    encode_utf8(s)
}

pub open spec fn chars(s: Bytes) -> Seq<char> {
    if valid_utf8(s) {
        decode_utf8(s)
    } else {
        Seq::empty()
    }
}

pub open spec fn empty() -> Bytes {
    Seq::empty()
}

pub open spec fn at(xs: Seq<Bytes>, i: int) -> Bytes {
    if 0 <= i < xs.len() {
        xs[i]
    } else {
        empty()
    }
}

pub open spec fn join(xs: Seq<Bytes>, sep: Bytes) -> Bytes
    decreases xs.len(),
{
    if xs.len() == 0 {
        empty()
    } else if xs.len() == 1 {
        xs[0]
    } else {
        xs[0] + sep + join(xs.drop_first(), sep)
    }
}

pub open spec fn unprefix(s: Bytes, p: Bytes) -> Bytes {
    if check::starts(s, p) {
        s.skip(p.len() as int)
    } else {
        s
    }
}

pub open spec fn unsuffix(s: Bytes, p: Bytes) -> Bytes {
    if check::ends(s, p) {
        s.take(s.len() - p.len())
    } else {
        s
    }
}

pub open spec fn lower(s: Bytes) -> Bytes {
    s.map_values(
        |b: u8|
            if 65 <= b <= 90 {
                (b - 65 + 97) as u8
            } else {
                b
            },
    )
}

pub open spec fn digits(s: Bytes) -> bool {
    s.len() > 0 && v1text::all_in(s, |b: u8| is_digit_b(b))
}

// html.escape: quote=False in text; quote=True in attributes (ui.py:35–38).
pub open spec fn escape(s: Bytes, attr: bool) -> Bytes
    decreases s.len(),
{
    if s.len() == 0 {
        empty()
    } else {
        let c = s[0];
        (if c == 38 {
            lit("&amp;"@)
        } else if c == 60 {
            lit("&lt;"@)
        } else if c == 62 {
            lit("&gt;"@)
        } else if attr && c == 34 {
            lit("&quot;"@)
        } else if attr && c == 39 {
            lit("&#x27;"@)
        } else {
            seq![c]
        }) + escape(s.drop_first(), attr)
    }
}

pub open spec fn escape_text(s: Bytes) -> Bytes {
    escape(s, false)
}

pub open spec fn escape_attr(s: Bytes) -> Bytes {
    escape(s, true)
}

pub open spec fn escaped(s: Bytes, attr: bool) -> bool
    decreases s.len(),
{
    if s.len() == 0 {
        true
    } else if s[0] == 38 {
        let n: int = if check::starts(s, lit("&amp;"@)) {
            5
        } else if check::starts(s, lit("&lt;"@)) || check::starts(s, lit("&gt;"@)) {
            4
        } else if check::starts(s, lit("&quot;"@)) || check::starts(s, lit("&#x27;"@)) {
            6
        } else {
            0
        };
        n > 0 && n <= s.len() && escaped(s.skip(n), attr)
    } else {
        s[0] != 60 && s[0] != 62 && (!attr || (s[0] != 34 && s[0] != 39)) && escaped(
            s.drop_first(),
            attr,
        )
    }
}

pub open spec fn url_seg(s: Bytes) -> Bytes
    decreases s.len(),
{
    if s.len() == 0 {
        empty()
    } else {
        let b = s[0];
        let x = b as int;
        (if is_alnum_b(b) || b == 45 || b == 46 || b == 126 {
            seq![b]
        } else {
            seq![
                37u8,
                (if x / 16 < 10 {
                    48 + x / 16
                } else {
                    65 + x / 16 - 10
                }) as u8,
                (if x % 16 < 10 {
                    48 + x % 16
                } else {
                    65 + x % 16 - 10
                }) as u8,
            ]
        }) + url_seg(s.drop_first())
    }
}

// A page is an audited sequence of fixed chrome and typed dynamic slots.
// Fixed fragments can include visible copy; dynamic slots always escape.
pub ghost enum Piece {
    Fixed(Bytes),
    Text(Bytes),
    Attr(Bytes),
    Decimal(nat),
    Comment(Bytes),
}

pub type Html = Seq<Piece>;

pub open spec fn fixed(s: Seq<char>) -> Html {
    seq![Piece::Fixed(lit(s))]
}

pub open spec fn fixed_bytes(s: Bytes) -> Html {
    seq![Piece::Fixed(s)]
}

pub open spec fn text(s: Bytes) -> Html {
    seq![Piece::Text(s)]
}

pub open spec fn attr(s: Bytes) -> Html {
    seq![Piece::Attr(s)]
}

pub open spec fn number(n: nat) -> Html {
    seq![Piece::Decimal(n)]
}

pub open spec fn emit_piece(p: Piece) -> Bytes {
    match p {
        Piece::Fixed(s) => s,
        Piece::Text(s) => escape_text(s),
        Piece::Attr(s) => escape_attr(s),
        Piece::Decimal(n) => check::nat_bytes(n),
        Piece::Comment(s) => comment_safe(s),
    }
}

pub open spec fn render_page(p: Html) -> Bytes {
    p.map_values(|x: Piece| emit_piece(x)).flatten()
}

pub open spec fn hjoin(xs: Seq<Html>, sep: Html) -> Html
    decreases xs.len(),
{
    if xs.len() == 0 {
        Seq::empty()
    } else if xs.len() == 1 {
        xs[0]
    } else {
        xs[0] + sep + hjoin(xs.drop_first(), sep)
    }
}

pub open spec fn lines(xs: Seq<Html>) -> Html {
    hjoin(xs, fixed("\n"@))
}

pub open spec fn cell(x: Html) -> Html {
    fixed("<td>"@) + x + fixed("</td>"@)
}

pub open spec fn row(xs: Seq<Html>) -> Html {
    fixed("<tr>"@) + xs.flatten() + fixed("</tr>"@)
}

pub open spec fn link(href: Bytes, label: Html) -> Html {
    fixed("<a href=\""@) + attr(href) + fixed("\">"@) + label + fixed("</a>"@)
}

// These are byte-bearing inputs, not independently supplied counts/classes.
// K4 owns coverage selection, Bundle digests, ledger grammar/classification;
// alignment uses the existing character-offset model. Prolog stays exact raw
// text: renderer acceptance does not strengthen K1 (fixture Prolog can differ).
pub ghost struct Document {
    pub bundle: Bundle,
    pub ace: Bytes,
    pub pl: Bytes,
    pub alignment: Option<Bytes>,
}

pub ghost struct Guideline {
    pub gid: Bytes,
    pub readme: Option<Bytes>,
    pub coverage: Coverage,
    pub documents: Seq<Document>,
    pub ledger: Src,
    pub ledger_digest: Bytes,
    pub source_names: Seq<Bytes>,
}

pub ghost struct Corpus {
    pub guidelines: Seq<Guideline>,
    pub token: Bytes,
}

pub ghost struct Record {
    pub decision: Decision,
    pub reviewer: Bytes,
    pub comment: Bytes,
}

pub open spec fn docids(g: Guideline) -> Seq<Bytes> {
    g.documents.map_values(|d: Document| d.bundle.docid)
}

pub open spec fn bundles(g: Guideline) -> Seq<Bundle> {
    g.documents.map_values(|d: Document| d.bundle)
}

pub open spec fn decisions(g: Guideline) -> Seq<Decision> {
    check::ledger(g.ledger, docids(g)).0
}

pub open spec fn ledger_data(s: Src) -> Bytes {
    match s {
        Src::Bytes(b) => b,
        _ => empty(),
    }
}

pub open spec fn raw_rows(s: Bytes) -> Seq<Bytes> {
    check::split_on(s, 10).filter(|r: Bytes| r.len() > 0 && r[0] != 35)
}

pub open spec fn records(g: Guideline) -> Seq<Record> {
    let ds = decisions(g);
    let rows = raw_rows(ledger_data(g.ledger));
    Seq::new(
        ds.len(),
        |i: int|
            {
                let f = check::tab_fields(at(rows, i));
                Record { decision: ds[i], reviewer: at(f, 4), comment: at(f, 6) }
            },
    )
}

pub open spec fn history(g: Guideline, id: Bytes) -> Seq<Record> {
    records(g).filter(|r: Record| r.decision.docid == id)
}

pub open spec fn state(g: Guideline, id: Bytes) -> int {
    let ds = decisions(g);
    let bs = bundles(g);
    if check::cur(ds, bs, id, true) {
        if check::cur(ds, bs, id, false) {
            2
        } else {
            0
        }
    } else if check::cur(ds, bs, id, false) {
        1
    } else if check::reviewed(ds).contains(id) {
        3
    } else {
        4
    }
}

pub open spec fn state_name(k: int) -> Bytes {
    if k == 0 {
        lit("approved"@)
    } else if k == 1 {
        lit("rejected"@)
    } else if k == 2 {
        lit("contested"@)
    } else if k == 3 {
        lit("stale"@)
    } else {
        lit("unreviewed"@)
    }
}

pub open spec fn state_label(k: int) -> Bytes {
    if k == 0 {
        lit("Approved"@)
    } else if k == 1 {
        lit("Rejected"@)
    } else if k == 2 {
        lit("Contested"@)
    } else if k == 3 {
        lit("Outdated"@)
    } else {
        lit("Unreviewed"@)
    }
}

pub open spec fn chip(k: int) -> Html {
    fixed("<span class=\"chip chip-"@) + attr(state_name(k)) + fixed("\">"@) + text(state_label(k))
        + fixed("</span>"@)
}

pub open spec fn field(r: Row, n: int) -> Bytes {
    at(check::tab_fields(unsuffix(r.line, lit("\n"@))), n)
}

pub open spec fn coverage_field(g: Guideline, id: Bytes, n: int) -> Bytes {
    match check::ace_row(g.coverage, id) {
        Option::Some(r) => field(r, n),
        _ => empty(),
    }
}

pub open spec fn payload(g: Guideline, id: Bytes) -> Bytes {
    match check::payload(g.coverage, id) {
        Option::Some(x) => x,
        _ => empty(),
    }
}

pub open spec fn first_title(ls: Seq<Bytes>, fallback: Bytes) -> Bytes
    decreases ls.len(),
{
    if ls.len() == 0 {
        fallback
    } else if check::starts(ls[0], lit("# "@)) && check::strip_ws(ls[0].skip(2)).len() > 0 {
        check::strip_ws(ls[0].skip(2))
    } else {
        first_title(ls.drop_first(), fallback)
    }
}

pub open spec fn title(g: Guideline) -> Bytes {
    match g.readme {
        Option::Some(b) => first_title(check::split_on(b, 10), g.gid),
        _ => g.gid,
    }
}

pub open spec fn human_section(b: Bytes) -> Bytes {
    let ss = check::split_on(b, 62).map_values(|s: Bytes| check::strip_ws(s)).filter(
        |s: Bytes| s.len() > 0,
    );
    if ss.len() == 0 {
        empty()
    } else {
        let h = check::split_on(ss[0], 32);
        let special = h.len() == 2 && digits(at(h, 1));
        let head = if special && at(h, 0) == lit("Rec"@) {
            seq![lit("Recommendation "@) + at(h, 1)]
        } else if special && at(h, 0) == lit("BOX"@) && ss.len() > 1 {
            Seq::empty()
        } else {
            seq![ss[0]]
        };
        join(head + ss.drop_first(), lit(" · "@))
    }
}

pub open spec fn document_title(g: Guideline, id: Bytes) -> Bytes {
    let section = coverage_field(g, id, 3);
    let base = human_section(section);
    let region = coverage_field(g, id, 0);
    let shared = g.documents.filter(
        |d: Document| coverage_field(g, d.bundle.docid, 3) == section,
    ).len() > 1;
    let page = unprefix(check::strip_ws(coverage_field(g, id, 2)), lit("p"@));
    let segs = check::split_on(region, 45);
    let last = at(segs, segs.len() - 1);
    if base.len() == 0 {
        id
    } else if !shared {
        base
    } else if digits(page) && digits(last) {
        base + lit(", page "@) + page + lit(", passage "@) + check::nat_bytes(check::dec_of(last))
    } else {
        base + lit(" ("@) + region + lit(")"@)
    }
}

pub open spec fn human_date(d: Bytes) -> Bytes {
    let p = check::split_on(unsuffix(d, lit("Z"@)), 84);
    if check::ends(d, lit("Z"@)) && p.len() == 2 {
        p[0] + lit(" "@) + p[1] + lit(" UTC"@)
    } else {
        d
    }
}

// Fixed rendered bytes: tools/ui.py build_css 1149–1257.
pub open spec fn css_text() -> Bytes {
    lit(
        r##"body { margin: 0 auto; max-width: 72rem; padding: 0 1.5rem 4rem; font-family: system-ui, sans-serif; line-height: 1.55; color: #111827; background: #ffffff; }
a { color: #1d4ed8; }
a:focus-visible, summary:focus-visible { outline: 3px solid #1d4ed8; outline-offset: 2px; }
.skip { position: absolute; left: -999px; top: 0; padding: 0.5rem 1rem; background: #ffffff; color: #1d4ed8; }
.skip:focus { left: 0; z-index: 1; }
nav.crumbs { padding: 1rem 0; border-bottom: 1px solid #e5e7eb; }
h1 { font-size: 1.5rem; }
h2 { font-size: 1.25rem; }
h3 { font-size: 1.05rem; }
h1 a.source { font-size: 1rem; font-weight: 400; margin-left: 0.5rem; }
table { border-collapse: collapse; width: 100%; margin: 1rem 0; }
th, td { text-align: left; padding: 0.4rem 0.6rem; border-bottom: 1px solid #e5e7eb; vertical-align: top; }
th { border-bottom: 2px solid #111827; }
table.compact { width: auto; }
table.compact th, table.compact td { padding-right: 2rem; }
table.records { table-layout: fixed; }
table.records th { box-sizing: border-box; }
table.records th:nth-child(1) { width: 12%; }
table.records th:nth-child(2) { width: 18%; }
table.records th:nth-child(3) { width: 22%; }
table.records th:nth-child(4) { width: 10%; }
.chip { display: inline-block; padding: 0.1rem 0.6rem; border-radius: 999px; font-size: 0.85rem; font-weight: 600; }
.chip-approved { color: #14532d; background: #dcfce7; }
.chip-rejected { color: #7f1d1d; background: #fee2e2; }
.chip-contested { color: #4c1d95; background: #ede9fe; }
.chip-stale { color: #78350f; background: #fef3c7; }
.chip-unreviewed { color: #1f2937; background: #e5e7eb; }
pre { padding: 0.75rem 1rem; border: 1px solid #e5e7eb; white-space: pre-wrap; overflow-x: auto; }
pre, code { font-family: ui-monospace, Menlo, Consolas, monospace; font-size: 0.95rem; }
pre.prose { font-family: Georgia, serif; font-size: 1.05rem; overflow-wrap: anywhere; }
dt { font-weight: 600; margin-top: 0.6rem; }
dd { margin-left: 0; }
summary { cursor: pointer; }
section { margin: 1.5rem 0; }
nav.docnav { padding: 1rem 0; border-top: 1px solid #e5e7eb; }
footer.scope { margin-top: 2rem; padding: 1rem 0; border-top: 1px solid #e5e7eb; font-size: 0.9rem; }
form label { display: block; margin-top: 1rem; font-weight: 600; }
fieldset { border: 0; margin: 1rem 0 0; padding: 0; max-width: 28rem; }
legend { font-weight: 600; padding: 0; }
fieldset label { margin-top: 0.5rem; font-weight: 400; }
input[type="text"], textarea { display: block; box-sizing: border-box; width: 100%; max-width: 28rem; margin-top: 0.3rem; padding: 0.45rem 0.6rem; border: 1px solid #e5e7eb; font-family: inherit; font-size: 1rem; color: #111827; background: #ffffff; }
textarea { min-height: 6rem; }
input[type="radio"], input[type="checkbox"] { accent-color: #111827; }
input[type="text"]:focus-visible, input[type="radio"]:focus-visible, input[type="checkbox"]:focus-visible, textarea:focus-visible, button:focus-visible { outline: 3px solid #1d4ed8; outline-offset: 2px; }
button { margin-top: 1.25rem; padding: 0.5rem 1.2rem; border: 1px solid #111827; font-family: inherit; font-size: 1rem; font-weight: 600; color: #ffffff; background: #111827; }
mark { background: #dbeafe; color: inherit; text-decoration: underline dotted #4b5563; text-underline-offset: 0.15em; }
.kw { color: #4b5563; }
.hl-note { color: #4b5563; font-size: 0.9rem; }
mark.t1, mark.t13, mark.t25, mark.t37 { background: #fef9c3; }
mark.t2, mark.t14, mark.t26, mark.t38 { background: #f3e8ff; }
mark.t3, mark.t15, mark.t27, mark.t39 { background: #ffedd5; }
mark.t4, mark.t16, mark.t28, mark.t40 { background: #ccfbf1; }
mark.t5, mark.t17, mark.t29, mark.t41 { background: #ffe4e6; }
mark.t6, mark.t18, mark.t30, mark.t42 { background: #dcfce7; }
mark.t7, mark.t19, mark.t31, mark.t43 { background: #fae8ff; }
mark.t8, mark.t20, mark.t32, mark.t44 { background: #cffafe; }
mark.t9, mark.t21, mark.t33, mark.t45 { background: #ecfccb; }
mark.t10, mark.t22, mark.t34, mark.t46 { background: #e0e7ff; }
mark.t11, mark.t23, mark.t35, mark.t47 { background: #e7e5e4; }
mark.t0, mark.t12, mark.t24, mark.t36 { text-decoration-color: #2563eb; }
mark.t1, mark.t13, mark.t25, mark.t37 { text-decoration-color: #a16207; }
mark.t2, mark.t14, mark.t26, mark.t38 { text-decoration-color: #7c3aed; }
mark.t3, mark.t15, mark.t27, mark.t39 { text-decoration-color: #c2410c; }
mark.t4, mark.t16, mark.t28, mark.t40 { text-decoration-color: #0f766e; }
mark.t5, mark.t17, mark.t29, mark.t41 { text-decoration-color: #be123c; }
mark.t6, mark.t18, mark.t30, mark.t42 { text-decoration-color: #15803d; }
mark.t7, mark.t19, mark.t31, mark.t43 { text-decoration-color: #a21caf; }
mark.t8, mark.t20, mark.t32, mark.t44 { text-decoration-color: #0e7490; }
mark.t9, mark.t21, mark.t33, mark.t45 { text-decoration-color: #4d7c0f; }
mark.t10, mark.t22, mark.t34, mark.t46 { text-decoration-color: #4f46e5; }
mark.t11, mark.t23, mark.t35, mark.t47 { text-decoration-color: #57534e; }
main:has(mark.t0:hover) mark.t0 { background: #bfdbfe; text-decoration-style: solid; }
main:has(mark.t1:hover) mark.t1 { background: #fef08a; text-decoration-style: solid; }
main:has(mark.t2:hover) mark.t2 { background: #e9d5ff; text-decoration-style: solid; }
main:has(mark.t3:hover) mark.t3 { background: #fed7aa; text-decoration-style: solid; }
main:has(mark.t4:hover) mark.t4 { background: #99f6e4; text-decoration-style: solid; }
main:has(mark.t5:hover) mark.t5 { background: #fecdd3; text-decoration-style: solid; }
main:has(mark.t6:hover) mark.t6 { background: #bbf7d0; text-decoration-style: solid; }
main:has(mark.t7:hover) mark.t7 { background: #f5d0fe; text-decoration-style: solid; }
main:has(mark.t8:hover) mark.t8 { background: #a5f3fc; text-decoration-style: solid; }
main:has(mark.t9:hover) mark.t9 { background: #d9f99d; text-decoration-style: solid; }
main:has(mark.t10:hover) mark.t10 { background: #c7d2fe; text-decoration-style: solid; }
main:has(mark.t11:hover) mark.t11 { background: #d6d3d1; text-decoration-style: solid; }
main:has(mark.t12:hover) mark.t12 { background: #bfdbfe; text-decoration-style: solid; }
main:has(mark.t13:hover) mark.t13 { background: #fef08a; text-decoration-style: solid; }
main:has(mark.t14:hover) mark.t14 { background: #e9d5ff; text-decoration-style: solid; }
main:has(mark.t15:hover) mark.t15 { background: #fed7aa; text-decoration-style: solid; }
main:has(mark.t16:hover) mark.t16 { background: #99f6e4; text-decoration-style: solid; }
main:has(mark.t17:hover) mark.t17 { background: #fecdd3; text-decoration-style: solid; }
main:has(mark.t18:hover) mark.t18 { background: #bbf7d0; text-decoration-style: solid; }
main:has(mark.t19:hover) mark.t19 { background: #f5d0fe; text-decoration-style: solid; }
main:has(mark.t20:hover) mark.t20 { background: #a5f3fc; text-decoration-style: solid; }
main:has(mark.t21:hover) mark.t21 { background: #d9f99d; text-decoration-style: solid; }
main:has(mark.t22:hover) mark.t22 { background: #c7d2fe; text-decoration-style: solid; }
main:has(mark.t23:hover) mark.t23 { background: #d6d3d1; text-decoration-style: solid; }
main:has(mark.t24:hover) mark.t24 { background: #bfdbfe; text-decoration-style: solid; }
main:has(mark.t25:hover) mark.t25 { background: #fef08a; text-decoration-style: solid; }
main:has(mark.t26:hover) mark.t26 { background: #e9d5ff; text-decoration-style: solid; }
main:has(mark.t27:hover) mark.t27 { background: #fed7aa; text-decoration-style: solid; }
main:has(mark.t28:hover) mark.t28 { background: #99f6e4; text-decoration-style: solid; }
main:has(mark.t29:hover) mark.t29 { background: #fecdd3; text-decoration-style: solid; }
main:has(mark.t30:hover) mark.t30 { background: #bbf7d0; text-decoration-style: solid; }
main:has(mark.t31:hover) mark.t31 { background: #f5d0fe; text-decoration-style: solid; }
main:has(mark.t32:hover) mark.t32 { background: #a5f3fc; text-decoration-style: solid; }
main:has(mark.t33:hover) mark.t33 { background: #d9f99d; text-decoration-style: solid; }
main:has(mark.t34:hover) mark.t34 { background: #c7d2fe; text-decoration-style: solid; }
main:has(mark.t35:hover) mark.t35 { background: #d6d3d1; text-decoration-style: solid; }
main:has(mark.t36:hover) mark.t36 { background: #bfdbfe; text-decoration-style: solid; }
main:has(mark.t37:hover) mark.t37 { background: #fef08a; text-decoration-style: solid; }
main:has(mark.t38:hover) mark.t38 { background: #e9d5ff; text-decoration-style: solid; }
main:has(mark.t39:hover) mark.t39 { background: #fed7aa; text-decoration-style: solid; }
main:has(mark.t40:hover) mark.t40 { background: #99f6e4; text-decoration-style: solid; }
main:has(mark.t41:hover) mark.t41 { background: #fecdd3; text-decoration-style: solid; }
main:has(mark.t42:hover) mark.t42 { background: #bbf7d0; text-decoration-style: solid; }
main:has(mark.t43:hover) mark.t43 { background: #f5d0fe; text-decoration-style: solid; }
main:has(mark.t44:hover) mark.t44 { background: #a5f3fc; text-decoration-style: solid; }
main:has(mark.t45:hover) mark.t45 { background: #d9f99d; text-decoration-style: solid; }
main:has(mark.t46:hover) mark.t46 { background: #c7d2fe; text-decoration-style: solid; }
main:has(mark.t47:hover) mark.t47 { background: #d6d3d1; text-decoration-style: solid; }
.hl-note label { margin-right: 0.75rem; }
main.hl-click pre.prose { color: #9ca3af; }
main.hl-click pre.prose .kw { color: #9ca3af; }
main.hl-click pre.prose mark:not(.hl-pick) { background: none; text-decoration-color: #9ca3af; }
main.hl-click pre.prose mark.hl-pick { color: #111827; }
body:has(input.hl-toggle:not(:checked)) pre.prose mark { background: none; text-decoration: none; color: inherit; }
body:has(input.hl-toggle:not(:checked)) pre.prose .kw { color: inherit; }
@media print {
body { max-width: none; padding: 0; }
nav.crumbs, nav.docnav, .skip, form, .verdict-entry, .hl-note { display: none; }
mark, mark[class] { background: none; text-decoration-color: #4b5563; }
main.hl-click pre.prose, main.hl-click pre.prose mark.hl-pick { color: inherit; }
main.hl-click pre.prose .kw { color: #4b5563; }
main.hl-click pre.prose mark:not(.hl-pick) { text-decoration-color: #4b5563; }
details::details-content { content-visibility: visible; }
pre { border: none; padding: 0; white-space: pre-wrap; overflow-x: visible; }
a { color: inherit; text-decoration: none; }
.chip { border: 1px solid #111827; background: none; color: inherit; }
}"##@,
    )
}

// Fixed rendered bytes: tools/ui.py build_hl_script 1261–1312.
pub open spec fn script_html() -> Bytes {
    lit(
        r##"<script>
(function () {
"use strict";
var picked = "";
function groupOf(node) {
if (node === null) { return ""; }
var m = node.closest("mark");
if (m === null) { return ""; }
var name = m.classList.item(0);
if (name === null) { return ""; }
return name;
}
function clearPick() {
if (picked === "") { return; }
document.getElementById("main").classList.remove("hl-click");
var marks = document.querySelectorAll("mark.hl-pick");
var i = 0;
while (i < marks.length) { marks[i].classList.remove("hl-pick"); i += 1; }
picked = "";
}
function toggleBox() { return document.querySelector("input.hl-toggle"); }
document.addEventListener("click", function (ev) {
var name = groupOf(ev.target);
if (name === "") { return; }
var box = toggleBox();
if (box === null) { return; }
if (box.checked === false) { return; }
if (picked === name) { clearPick(); return; }
clearPick();
var marks = document.querySelectorAll("mark." + name);
var i = 0;
while (i < marks.length) { marks[i].classList.add("hl-pick"); i += 1; }
document.getElementById("main").classList.add("hl-click");
picked = name;
});
document.addEventListener("mouseout", function (ev) {
if (picked === "") { return; }
if (groupOf(ev.target) !== picked) { return; }
if (groupOf(ev.relatedTarget) === picked) { return; }
clearPick();
});
document.addEventListener("change", function (ev) {
var box = toggleBox();
if (box === null) { return; }
if (ev.target === box) { if (box.checked === false) { clearPick(); } }
});
})();
</script>"##@,
    )
}

// --- Pure render law: ui.py:1083–1766. ---
pub open spec fn frame(title: Bytes, crumbs: Html, body: Html) -> Html {
    lines(
        seq![
            fixed("<!doctype html>"@),
            fixed("<html lang=\"en\">"@),
            fixed("<head>"@),
            fixed("<meta charset=\"utf-8\">"@),
            fixed("<title>"@) + text(title) + fixed(" — cnl-ckc reviewer</title>"@),
            fixed("<style>"@),
            fixed_bytes(css_text()),
            fixed("</style>"@),
            fixed("</head>"@),
            fixed("<body>"@),
            fixed("<a class=\"skip\" href=\"#main\">Skip to content</a>"@),
            fixed("<nav class=\"crumbs\">"@) + crumbs + fixed("</nav>"@),
            fixed("<main id=\"main\">"@),
            body,
            fixed("</main>"@),
            fixed(
                "<footer class=\"scope\"><p>This page reports what the loaded guideline documents state. It does not give clinical advice.</p></footer>"@,
            ),
            fixed("</body>"@),
            fixed("</html>"@),
        ],
    ) + fixed("\n"@)
}

pub open spec fn current(g: Guideline, r: Record) -> bool {
    exists|i: int|
        0 <= i < g.documents.len() && (#[trigger] g.documents[i]).bundle.docid == r.decision.docid
            && g.documents[i].bundle.review == r.decision.digest
}

pub open spec fn tally(g: Guideline, id: Bytes) -> (nat, nat, nat) {
    let hs = history(g, id);
    (
        hs.filter(|r: Record| current(g, r) && r.decision.approved).len(),
        hs.filter(|r: Record| current(g, r) && !r.decision.approved).len(),
        hs.filter(|r: Record| !current(g, r)).len(),
    )
}

pub open spec fn tally_parts(t: (nat, nat, nat), earlier: bool) -> Seq<Bytes> {
    (if t.0 > 0 {
        seq![check::nat_bytes(t.0) + lit(" approved"@)]
    } else {
        Seq::empty()
    }) + (if t.1 > 0 {
        seq![check::nat_bytes(t.1) + lit(" rejected"@)]
    } else {
        Seq::empty()
    }) + (if earlier && t.2 > 0 {
        seq![check::nat_bytes(t.2) + lit(" earlier"@)]
    } else {
        Seq::empty()
    })
}

pub open spec fn tally_cell(t: (nat, nat, nat)) -> Bytes {
    let p = tally_parts(t, true);
    if p.len() == 0 {
        lit("None"@)
    } else {
        join(p, lit(", "@))
    }
}

pub open spec fn tally_text(t: (nat, nat, nat)) -> Bytes {
    let p = tally_parts(t, false);
    (if p.len() > 0 {
        lit("Decisions on this version: "@) + join(p, lit(" and "@)) + lit("."@)
    } else if t.2 > 0 {
        lit("No decision is recorded on this version."@)
    } else {
        lit("No decision is recorded."@)
    }) + (if t.2 > 0 {
        lit(" Decisions on earlier versions: "@) + check::nat_bytes(t.2) + lit("."@)
    } else {
        empty()
    })
}

pub open spec fn review_summary(g: Guideline) -> Bytes {
    let n = decisions(g).len();
    if n == 0 {
        lit("No decisions are recorded for the "@) + check::nat_bytes(g.documents.len()) + lit(
            " documents in this guideline."@,
        )
    } else {
        lit("Reviewers recorded "@) + check::nat_bytes(n) + lit(" decisions on "@)
            + check::nat_bytes(check::reviewed(decisions(g)).len()) + lit(" of "@)
            + check::nat_bytes(g.documents.len()) + lit(" documents."@)
    }
}

pub open spec fn class_counts(g: Guideline) -> Seq<nat> {
    let ds = decisions(g);
    let bs = bundles(g);
    seq![
        check::class_count(ds, bs, 0),
        check::class_count(ds, bs, 1),
        check::class_count(ds, bs, 2),
        check::class_count(ds, bs, 3),
        check::unreviewed(ds, bs),
    ]
}

pub open spec fn index_html(c: Corpus) -> Html {
    let rows = c.guidelines.map_values(
        |g: Guideline|
            row(
                seq![
                    cell(link(lit("g/"@) + url_seg(g.gid) + lit("/index.html"@), text(title(g)))),
                    cell(number(g.documents.len())),
                    cell(number(g.coverage.rows.len())),
                ] + class_counts(g).map_values(|n: nat| cell(number(n))),
            ),
    );
    frame(
        lit("Guidelines"@),
        fixed("cnl-ckc reviewer"@),
        lines(
            seq![
                fixed("<h1>Guidelines</h1>"@),
                fixed("<section>"@),
                fixed("<table>"@),
                fixed(
                    "<thead><tr><th>Guideline</th><th>Documents</th><th>Passages</th><th>Approved</th><th>Rejected</th><th>Contested</th><th>Outdated</th><th>Unreviewed</th></tr></thead>"@,
                ),
                fixed("<tbody>"@) + lines(rows) + fixed("</tbody>"@),
                fixed("</table>"@),
                fixed("</section>"@),
            ],
        ),
    )
}

pub open spec fn render_index(c: Corpus) -> Bytes {
    render_page(index_html(c))
}

pub open spec fn region_status(r: Row) -> Bytes {
    match r.status {
        Status::Restates(_) => lit("Restates "@) + unsuffix(
            unprefix(field(r, 4), lit("restates("@)),
            lit(")"@),
        ),
        Status::Uncovered => {
            let inner = unsuffix(unprefix(field(r, 4), lit("uncovered("@)), lit(")"@));
            let i = check::first_sub(inner, lit(": "@), 0);
            lit("Not covered — "@) + (if i + 2 <= inner.len() {
                inner.skip(i as int + 2)
            } else {
                empty()
            })
        },
        Status::Pending => lit("Pending"@),
        _ => empty(),
    }
}

pub open spec fn guideline_html(g: Guideline) -> Html {
    let labels = seq![
        lit("Passages"@),
        lit("With ACE"@),
        lit("Pending"@),
        lit("Approved"@),
        lit("Rejected"@),
        lit("Contested"@),
        lit("Outdated"@),
        lit("Unreviewed"@),
    ];
    let counts = seq![
        g.coverage.rows.len(),
        check::count_status(g.coverage.rows, 1),
        check::count_status(g.coverage.rows, 0),
    ] + class_counts(g);
    let status_rows = Seq::new(
        labels.len(),
        |i: int|
            row(
                seq![
                    cell(text(labels[i])),
                    cell(
                        number(
                            if i < counts.len() {
                                counts[i]
                            } else {
                                0
                            },
                        ),
                    ),
                ],
            ),
    );
    let doc_rows = g.documents.map_values(
        |d: Document|
            {
                let id = d.bundle.docid;
                row(
                    seq![
                        cell(
                            link(
                                lit("doc/"@) + url_seg(id) + lit(".html"@),
                                text(document_title(g, id)),
                            ),
                        ),
                        cell(chip(state(g, id))),
                        cell(text(tally_cell(tally(g, id)))),
                        cell(text(coverage_field(g, id, 0))),
                    ],
                )
            },
    );
    let others = g.coverage.rows.filter(|r: Row| !matches!(r.status,Status::Ace(_))).map_values(
        |r: Row| row(seq![cell(text(r.id)), cell(text(region_status(r))), cell(text(field(r, 3)))]),
    );
    let body = lines(
        seq![
            fixed("<h1>"@) + text(title(g)) + fixed("</h1>"@),
            fixed("<p>"@) + text(review_summary(g)) + fixed(
                " <a href=\"records.html\">All decision records</a></p>"@,
            ),
            fixed("<section>"@),
            fixed("<h2>Status</h2>"@),
            fixed("<table class=\"compact\">"@),
            fixed("<thead><tr><th>Status</th><th>Count</th></tr></thead>"@),
            fixed("<tbody>"@) + lines(status_rows) + fixed("</tbody>"@),
            fixed("</table>"@),
            fixed("</section>"@),
            fixed("<section>"@),
            fixed("<h2>Documents</h2>"@),
            fixed("<table class=\"compact\">"@),
            fixed(
                "<thead><tr><th>Document</th><th>Status</th><th>Decisions</th><th>Passage</th></tr></thead>"@,
            ),
            fixed("<tbody>"@) + lines(doc_rows) + fixed("</tbody>"@),
            fixed("</table>"@),
            fixed("</section>"@),
            fixed("<section>"@),
            fixed("<h2>Passages without ACE</h2>"@),
            fixed("<table class=\"compact\">"@),
            fixed("<thead><tr><th>Passage</th><th>Status</th><th>Section</th></tr></thead>"@),
            fixed("<tbody>"@) + lines(others) + fixed("</tbody>"@),
            fixed("</table>"@),
            fixed("</section>"@),
        ],
    );
    frame(title(g), fixed("<a href=\"../../index.html\">guidelines</a> / "@) + text(title(g)), body)
}

pub open spec fn render_guideline(g: Guideline) -> Bytes {
    render_page(guideline_html(g))
}

pub open spec fn version_link(r: Record, version: Bytes) -> Html {
    if r.decision.commit.len() > 0 {
        link(lit("https://github.com/eturkes/cnl-ckc/commit/"@) + r.decision.commit, text(version))
    } else {
        text(version)
    }
}

pub open spec fn record_row(g: Guideline, r: Record) -> Html {
    row(
        seq![
            cell(
                text(
                    state_label(
                        if r.decision.approved {
                            0
                        } else {
                            1
                        },
                    ),
                ),
            ),
            cell(text(r.reviewer)),
            cell(text(human_date(r.decision.date))),
            cell(
                version_link(
                    r,
                    if current(g, r) {
                        lit("Current"@)
                    } else {
                        lit("Earlier"@)
                    },
                ),
            ),
            cell(
                text(
                    if r.comment.len() == 0 {
                        lit("Not given"@)
                    } else {
                        r.comment
                    },
                ),
            ),
        ],
    )
}

pub open spec fn record_section(g: Guideline, id: Bytes) -> Seq<Html> {
    let hs = history(g, id);
    if hs.len() == 0 {
        Seq::empty()
    } else {
        let rows = Seq::new(hs.len(), |i: int| record_row(g, hs[hs.len() - i - 1]));
        seq![
            fixed("<section id=\""@) + attr(id) + fixed("\">"@),
            fixed("<h2>"@) + link(
                lit("doc/"@) + url_seg(id) + lit(".html"@),
                text(document_title(g, id)),
            ) + fixed("</h2>"@),
            fixed("<table class=\"records\">"@),
            fixed(
                "<thead><tr><th>Decision</th><th>Reviewer</th><th>Date</th><th>Version</th><th>Comment</th></tr></thead>"@,
            ),
            fixed("<tbody>"@) + lines(rows) + fixed("</tbody>"@),
            fixed("</table>"@),
            fixed("</section>"@),
        ]
    }
}

pub open spec fn records_html(g: Guideline) -> Html {
    let sections = g.documents.map_values(
        |d: Document| record_section(g, d.bundle.docid),
    ).flatten();
    let summary = review_summary(g) + (if decisions(g).len() > 0 {
        lit(" The newest decision for each document is first."@)
    } else {
        empty()
    });
    let notes = if sections.len() == 0 {
        seq![fixed("<p>Open a document and record a decision to start this list.</p>"@)]
    } else {
        seq![fixed("<p>Each reviewer name is recorded as entered and is not verified.</p>"@)] + (
        if records(g).filter(|r: Record| r.decision.commit.len() > 0).len() > 0 {
            seq![
                fixed(
                    "<p>Each version links to the stored version of the text that the reviewer read.</p>"@,
                ),
            ]
        } else {
            Seq::empty()
        })
    };
    frame(
        lit("Decision records"@),
        fixed("<a href=\"../../index.html\">guidelines</a> / <a href=\"index.html\">"@) + text(
            title(g),
        ) + fixed("</a> / records"@),
        lines(
            seq![
                fixed("<h1>Decision records</h1>"@),
                fixed("<p>"@) + text(summary) + fixed("</p>"@),
            ] + sections + notes + seq![
                fixed("<nav class=\"docnav\"><a href=\"index.html\">Guideline index</a></nav>"@),
            ],
        ),
    )
}

pub open spec fn render_records(g: Guideline) -> Bytes {
    render_page(records_html(g))
}

// The keyword regexp is ASCII; Unicode case folding never applies to its gaps.
pub open spec fn token_byte(b: u8) -> bool {
    is_alnum_b(b) && b != 95
}

pub open spec fn token_end(s: Bytes, i: nat) -> nat
    decreases s.len() - i,
{
    if i < s.len() && token_byte(s[i as int]) {
        token_end(s, i + 1)
    } else if i + 1 < s.len() && s[i as int] == 45 && token_byte(s[i as int + 1]) {
        token_end(s, i + 2)
    } else {
        i
    }
}

pub open spec fn keyword_html(s: Bytes) -> Html
    decreases s.len(),
{
    if s.len() == 0 {
        Seq::empty()
    } else if token_byte(s[0]) {
        let end = token_end(s, 1);
        let e = if 1 <= end <= s.len() {
            end
        } else {
            1
        };
        let word = s.take(e as int);
        (if stop_words().contains(lower(word)) {
            fixed("<span class=\"kw\">"@) + text(word) + fixed("</span>"@)
        } else {
            text(word)
        }) + keyword_html(s.skip(e as int))
    } else {
        text(seq![s[0]]) + keyword_html(s.drop_first())
    }
}

pub open spec fn slice_text(s: Bytes, start: int, end: int) -> Bytes {
    let cs = chars(s);
    if 0 <= start <= end <= cs.len() {
        encode_utf8(cs.subrange(start, end))
    } else {
        empty()
    }
}

pub open spec fn marked_html(
    s: Bytes,
    spans: Seq<align::OutSpan>,
    keywords: bool,
    cursor: int,
) -> Html
    decreases spans.len(),
{
    if spans.len() == 0 {
        let t = slice_text(s, cursor, chars(s).len() as int);
        if keywords {
            keyword_html(t)
        } else {
            text(t)
        }
    } else {
        let p = spans[0];
        let gap = slice_text(s, cursor, p.start);
        let part = slice_text(s, p.start, p.end);
        (if keywords {
            keyword_html(gap)
        } else {
            text(gap)
        }) + (if p.index < 48 {
            fixed("<mark class=\"t"@) + attr(v1text::dec_bytes(p.index)) + fixed("\">"@)
        } else {
            fixed("<mark>"@)
        }) + text(part) + fixed("</mark>"@) + marked_html(s, spans.drop_first(), keywords, p.end)
    }
}

pub open spec fn alignment(g: Guideline, d: Document) -> Option<align::AlignModel> {
    match d.alignment {
        Option::None => Option::None,
        Option::Some(b) => match align::align_outcome(
            chars(b),
            chars(payload(g, d.bundle.docid)),
            chars(d.ace),
        ) {
            align::AlignOutcome::Ok(m) => Option::Some(m),
            _ => Option::None,
        },
    }
}

pub open spec fn aligned_text(g: Guideline, d: Document, ace: bool) -> Html {
    let s = if ace {
        d.ace
    } else {
        payload(g, d.bundle.docid)
    };
    match alignment(g, d) {
        Option::Some(m) => marked_html(
            s,
            if ace {
                m.ace
            } else {
                m.src
            },
            ace,
            0,
        ),
        Option::None => if ace {
            keyword_html(s)
        } else {
            text(s)
        },
    }
}

pub open spec fn names(g: Guideline) -> Seq<Bytes> {
    check::sort_bytes(
        check::dedup_bytes(
            records(g).map_values(|r: Record| r.reviewer).filter(|n: Bytes| n.len() > 0),
        ),
    )
}

pub open spec fn latest_name(rs: Seq<Record>, date: Bytes, name: Bytes) -> Bytes
    decreases rs.len(),
{
    if rs.len() == 0 {
        name
    } else {
        let r = rs[0];
        if r.reviewer.len() > 0 && !bytes_lt(r.decision.date, date) {
            latest_name(rs.drop_first(), r.decision.date, r.reviewer)
        } else {
            latest_name(rs.drop_first(), date, name)
        }
    }
}

pub open spec fn roster(g: Guideline) -> Html {
    fixed("<datalist id=\"reviewer-names\">"@) + names(g).map_values(
        |n: Bytes| fixed("<option value=\""@) + attr(n) + fixed("\"></option>"@),
    ).flatten() + fixed("</datalist>"@)
}

pub open spec fn linebreak(c: char) -> bool {
    c == '\n' || c == '\r' || c == '\u{b}' || c == '\u{c}' || c == '\u{1c}' || c == '\u{1d}' || c
        == '\u{1e}' || c == '\u{85}' || c == '\u{2028}' || c == '\u{2029}'
}

pub open spec fn splitline_count(cs: Seq<char>, pending: bool) -> nat
    decreases cs.len(),
{
    if cs.len() == 0 {
        if pending {
            1
        } else {
            0
        }
    } else if linebreak(cs[0]) {
        1 + splitline_count(
            if cs.len() > 1 && cs[0] == '\r' && cs[1] == '\n' {
                cs.skip(2)
            } else {
                cs.drop_first()
            },
            false,
        )
    } else {
        splitline_count(cs.drop_first(), true)
    }
}

pub open spec fn document_html(
    g: Guideline,
    d: Document,
    prev: Bytes,
    next: Bytes,
    token: Bytes,
) -> Html {
    let id = d.bundle.docid;
    let k = state(g, id);
    let region = coverage_field(g, id, 0);
    let pdfs = g.source_names.filter(|n: Bytes| check::ends(n, lit(".pdf"@)));
    let heading = text(title(g)) + (if pdfs.len() == 1 {
        fixed(" <a class=\"source\" href=\"../source/"@) + attr(url_seg(pdfs[0])) + fixed(
            "\">PDF</a>"@,
        )
    } else {
        Seq::empty()
    });
    let source = unprefix(coverage_field(g, id, 1), lit("source/"@));
    let prov = (if region.len() > 0 {
        seq![text(region)]
    } else {
        Seq::empty()
    }) + (if g.source_names.contains(source) {
        seq![link(lit("../source/"@) + url_seg(source), fixed("Source text"@))]
    } else {
        Seq::empty()
    });
    let records_href = lit("../records.html"@) + (if history(g, id).len() > 0 {
        lit("#"@) + url_seg(id)
    } else {
        empty()
    });
    let shown = match alignment(g, d) {
        Option::Some(m) => m.count > 0,
        _ => false,
    };
    let nav = (if prev.len() > 0 {
        seq![link(url_seg(prev) + lit(".html"@), fixed("Previous document"@))]
    } else {
        Seq::empty()
    }) + seq![fixed("<a href=\"../index.html\">Guideline index</a>"@)] + (if next.len() > 0 {
        seq![link(url_seg(next) + lit(".html"@), fixed("Next document"@))]
    } else {
        Seq::empty()
    });
    let body = lines(
        seq![
            fixed("<h1>"@) + heading + fixed("</h1>"@),
            fixed("<h2>"@) + text(document_title(g, id)) + fixed(" "@) + chip(k) + fixed("</h2>"@),
        ] + (if prov.len() > 0 {
            seq![fixed("<p>"@) + hjoin(prov, fixed(" · "@)) + fixed("</p>"@)]
        } else {
            Seq::empty()
        }) + seq![
            fixed("<p>"@) + text(tally_text(tally(g, id))) + fixed(" "@) + link(
                records_href,
                fixed("All decision records"@),
            ) + fixed("</p>"@),
        ] + (if k == 3 {
            seq![
                fixed("<section class=\"stale\">"@),
                fixed(
                    "<p>The document or its source changed after the last decision. No recorded decision applies to the version shown here.</p>"@,
                ),
                fixed("</section>"@),
            ]
        } else {
            Seq::empty()
        }) + (if shown {
            seq![
                fixed(
                    "<p class=\"hl-note\"><label><input type=\"checkbox\" class=\"hl-toggle\" checked> Highlighting</label> Try hovering and clicking on highlighted terms for different levels of emphasis.</p>"@,
                ),
                fixed_bytes(script_html()),
            ]
        } else {
            Seq::empty()
        }) + seq![
            fixed("<section>"@),
            fixed("<h3>Original passage</h3>"@),
            fixed("<pre class=\"prose\">"@) + aligned_text(g, d, false) + fixed("</pre>"@),
            fixed("</section>"@),
            fixed("<section>"@),
            fixed("<h3>Attempto Controlled English (ACE)</h3>"@),
            fixed("<pre class=\"prose\">"@) + aligned_text(g, d, true) + fixed("</pre>"@),
            fixed("</section>"@),
            fixed("<section class=\"verdict-entry\">"@),
            fixed("<h3>Record a decision</h3>"@),
            fixed(
                "<p>Does the ACE representation appropriately reflect the original passage?</p>"@,
            ),
            fixed("<form method=\"post\">"@),
            fixed("<fieldset>"@),
            fixed("<legend>Decision</legend>"@),
            fixed(
                "<label><input type=\"radio\" name=\"verdict\" value=\"approved\" required> Approved</label>"@,
            ),
            fixed(
                "<label><input type=\"radio\" name=\"verdict\" value=\"rejected\" required> Rejected</label>"@,
            ),
            fixed("</fieldset>"@),
            fixed("<label for=\"reviewer\">Reviewer name</label>"@),
            fixed(
                "<input type=\"text\" id=\"reviewer\" name=\"reviewer\" list=\"reviewer-names\" value=\""@,
            ) + attr(latest_name(records(g), empty(), empty())) + fixed("\" required>"@),
            roster(g),
            fixed("<label for=\"comment\">Comment (optional)</label>"@),
            fixed("<textarea id=\"comment\" name=\"comment\"></textarea>"@),
            fixed("<input type=\"hidden\" name=\"review_sha256\" value=\""@) + attr(d.bundle.review)
                + fixed("\">"@),
            fixed("<input type=\"hidden\" name=\"ledger_sha256\" value=\""@) + attr(g.ledger_digest)
                + fixed("\">"@),
            fixed("<input type=\"hidden\" name=\"csrf\" value=\""@) + attr(token) + fixed("\">"@),
            fixed("<button>Record decision</button>"@),
            fixed("</form>"@),
            fixed("</section>"@),
            fixed("<section>"@),
            fixed("<details>"@),
            fixed("<summary>Compiled Prolog ("@) + number(splitline_count(chars(d.pl), false))
                + fixed(" lines)</summary>"@),
            fixed("<pre>"@) + text(d.pl) + fixed("</pre>"@),
            fixed("</details>"@),
            fixed("</section>"@),
            fixed("<nav class=\"docnav\">"@) + hjoin(nav, fixed(" · "@)) + fixed("</nav>"@),
        ],
    );
    frame(
        document_title(g, id),
        fixed("<a href=\"../../../index.html\">guidelines</a> / <a href=\"../index.html\">"@)
            + text(title(g)) + fixed("</a> / "@) + text(region),
        body,
    )
}

pub open spec fn render_document(
    g: Guideline,
    d: Document,
    prev: Bytes,
    next: Bytes,
    token: Bytes,
) -> Bytes {
    render_page(document_html(g, d, prev, next, token))
}

pub open spec fn stop_words() -> Seq<Bytes> {
    check::split_on(
        lit(
            "a an the every each no all some any this that these those such is are was were be been being has have had does do did should must may can cannot might will would shall could if then and or nor but not it its itself they them their he she who whom whose which what where when there something somebody someone everything everybody everyone nothing nobody of for with without during to at in on by from as against about after before through under over above below into onto per within between among around near than least most more less fewer greater"@,
        ),
        32,
    )
}

// --- POST: ui.py:2256–2457 + 2562–2654. Header/body IO is already parsed. ---
pub ghost struct Request {
    pub method: Bytes,
    pub path: Bytes,
    pub host: Bytes,
    pub origin: Option<Bytes>,
    pub content_type: Bytes,
    pub body: Option<Bytes>,
}

pub ghost struct Fields {
    pub verdict: Bytes,
    pub reviewer: Bytes,
    pub comment: Bytes,
    pub review: Bytes,
    pub ledger: Bytes,
    pub csrf: Bytes,
}

pub ghost struct PostDocument {
    pub docid: Bytes,
    pub render_error: Option<Bytes>,
    pub commit: Bytes,
}

pub ghost struct PostGuideline {
    pub gid: Bytes,
    pub documents: Seq<PostDocument>,
    pub fresh: Result<Seq<Bundle>, Bytes>,
    pub ledger: Src,
    pub ledger_digest: Bytes,
}

pub ghost struct PostState {
    pub port: nat,
    pub token: Bytes,
    pub models: Result<Seq<PostGuideline>, Bytes>,
    pub now: Bytes,
}

pub ghost struct Response {
    pub status: nat,
    pub body: Bytes,
    pub allow: Bytes,
    pub location: Bytes,
}

// Prepared is not a successful write. Shell must rehash under its lock and
// compare expected_ledger, then atomically install exactly candidate bytes.
pub ghost enum PostOutcome {
    Read,
    Refused(Response),
    Prepared { candidate: Bytes, expected_ledger: Bytes, response: Response },
}

pub open spec fn comment_safe(d: Bytes) -> Bytes {
    chars(d).map_values(
        |c: char|
            {
                let n = c as int;
                if (48 <= n <= 57) || (65 <= n <= 90) || (97 <= n <= 122) || n == 32 || n == 58 || n
                    == 95 || n == 46 {
                    n as u8
                } else {
                    95u8
                }
            },
    )
}

pub open spec fn error_page(status: nat, title: Bytes, body: Html) -> Response {
    Response {
        status,
        body: render_page(
            frame(
                title,
                fixed("cnl-ckc reviewer"@),
                lines(seq![fixed("<h1>"@) + text(title) + fixed("</h1>"@), body]),
            ),
        ),
        allow: empty(),
        location: empty(),
    }
}

pub open spec fn annotated_body(copy: Bytes, detail: Bytes) -> Html {
    fixed("<p>"@) + text(copy) + fixed("</p>\n<!-- "@) + seq![Piece::Comment(detail)] + fixed(
        " -->"@,
    )
}

pub open spec fn refusal(status: nat, title: Bytes, detail: Bytes, copy: Bytes) -> PostOutcome {
    PostOutcome::Refused(error_page(status, title, annotated_body(copy, detail)))
}

pub open spec fn refused_copy() -> Bytes {
    lit(
        "The request was refused. Open the document page again from this site and submit the decision again."@,
    )
}

pub open spec fn invalid_form_copy() -> Bytes {
    lit(
        "The submitted form was not valid. Go back to the document page, reload it, and submit the decision again."@,
    )
}

pub open spec fn forbidden(detail: Bytes) -> PostOutcome {
    refusal(403, lit("Forbidden"@), detail, refused_copy())
}

pub open spec fn bad_form(detail: Bytes) -> PostOutcome {
    refusal(400, lit("Bad request"@), detail, invalid_form_copy())
}

pub open spec fn server_error(detail: Bytes) -> PostOutcome {
    refusal(
        500,
        lit("Server error"@),
        detail,
        lit("The server could not complete the request. Reload the page and try again."@),
    )
}

pub open spec fn ledger_changed() -> PostOutcome {
    refusal(
        409,
        lit("Conflict"@),
        lit("ui: verdict: ledger changed"@),
        lit(
            "Another decision was recorded for this guideline before this one. The decision was not recorded. Open the document page again and check the current state."@,
        ),
    )
}

pub open spec fn doc_route(path: Bytes) -> Option<(Bytes, Bytes)> {
    if !check::starts(path, lit("/g/"@)) {
        Option::None
    } else {
        let xs = check::split_on(path.skip(3), 47);
        if xs.len() == 3 && xs[0].len() > 0 && xs[1] == lit("doc"@) && check::ends(
            xs[2],
            lit(".html"@),
        ) && xs[2].len() > 5 {
            Option::Some((xs[0], xs[2].take(xs[2].len() - 5)))
        } else {
            Option::None
        }
    }
}

pub open spec fn method_response(shaped: bool) -> PostOutcome {
    let r = error_page(
        405,
        lit("Method not allowed"@),
        if shaped {
            fixed("<p>Only GET and POST are supported on this page.</p>"@)
        } else {
            fixed("<p>Only GET is supported on this page.</p>"@)
        },
    );
    PostOutcome::Refused(
        Response {
            allow: if shaped {
                lit("GET, POST"@)
            } else {
                lit("GET"@)
            },
            ..r
        },
    )
}

pub open spec fn not_found() -> PostOutcome {
    PostOutcome::Refused(
        error_page(404, lit("Not found"@), fixed("<p>The requested page does not exist.</p>"@)),
    )
}

pub open spec fn hex_value(b: u8) -> int {
    if 48 <= b <= 57 {
        b as int - 48
    } else if 65 <= b <= 70 {
        b as int - 65 + 10
    } else if 97 <= b <= 102 {
        b as int - 97 + 10
    } else {
        -1
    }
}

pub open spec fn form_unquote(b: Bytes) -> Bytes
    decreases b.len(),
{
    if b.len() == 0 {
        empty()
    } else if b[0] == 43 {
        seq![32u8] + form_unquote(b.drop_first())
    } else if b.len() >= 3 && b[0] == 37 && hex_value(b[1]) >= 0 && hex_value(b[2]) >= 0 {
        seq![(16 * hex_value(b[1]) + hex_value(b[2])) as u8] + form_unquote(b.skip(3))
    } else {
        seq![b[0]] + form_unquote(b.drop_first())
    }
}

pub open spec fn form_pairs(xs: Seq<Bytes>) -> Result<Seq<(Bytes, Bytes)>, Bytes>
    decreases xs.len(),
{
    if xs.len() == 0 {
        Result::Ok(Seq::empty())
    } else {
        let raw = xs[0];
        let n = check::first_sub(raw, lit("="@), 0);
        if n >= raw.len() {
            Result::Err(lit("ui: verdict: body not parseable"@))
        } else {
            let k = form_unquote(raw.take(n as int));
            let v = form_unquote(raw.skip(n as int + 1));
            if !valid_utf8(k) || !valid_utf8(v) {
                Result::Err(lit("ui: verdict: body not parseable"@))
            } else {
                match form_pairs(xs.drop_first()) {
                    Result::Err(e) => Result::Err(e),
                    Result::Ok(ps) => Result::Ok(seq![(k, v)] + ps),
                }
            }
        }
    }
}

pub open spec fn field_names() -> Seq<Bytes> {
    seq![
        lit("verdict"@),
        lit("reviewer"@),
        lit("comment"@),
        lit("review_sha256"@),
        lit("ledger_sha256"@),
        lit("csrf"@),
    ]
}

pub open spec fn field_values(ps: Seq<(Bytes, Bytes)>, key: Bytes) -> Seq<Bytes> {
    ps.filter(|p: (Bytes, Bytes)| p.0 == key).map_values(|p: (Bytes, Bytes)| p.1)
}

pub open spec fn parse_fields(ps: Seq<(Bytes, Bytes)>) -> Result<Fields, Bytes> {
    let ns = field_names();
    let missing = ns.filter(|n: Bytes| field_values(ps, n).len() == 0);
    let duplicate = ns.filter(|n: Bytes| field_values(ps, n).len() > 1);
    let unknown = ps.filter(|p: (Bytes, Bytes)| !ns.contains(p.0));
    if missing.len() > 0 {
        Result::Err(lit("ui: verdict: missing field "@) + missing[0])
    } else if duplicate.len() > 0 {
        Result::Err(lit("ui: verdict: duplicate field "@) + duplicate[0])
    } else if unknown.len() > 0 {
        Result::Err(lit("ui: verdict: unknown field "@) + unknown[0].0)
    } else {
        let f = Fields {
            verdict: at(field_values(ps, lit("verdict"@)), 0),
            reviewer: at(field_values(ps, lit("reviewer"@)), 0),
            comment: at(field_values(ps, lit("comment"@)), 0),
            review: at(field_values(ps, lit("review_sha256"@)), 0),
            ledger: at(field_values(ps, lit("ledger_sha256"@)), 0),
            csrf: at(field_values(ps, lit("csrf"@)), 0),
        };
        if f.verdict != lit("approved"@) && f.verdict != lit("rejected"@) {
            Result::Err(lit("ui: verdict: invalid verdict"@))
        } else if f.reviewer.len() == 0 || !check::text_clean(f.reviewer) {
            Result::Err(lit("ui: verdict: invalid reviewer"@))
        } else if !check::text_clean(f.comment) {
            Result::Err(lit("ui: verdict: invalid comment"@))
        } else if !hex64(f.review) {
            Result::Err(lit("ui: verdict: invalid review_sha256"@))
        } else if f.ledger != lit("absent"@) && !hex64(f.ledger) {
            Result::Err(lit("ui: verdict: invalid ledger_sha256"@))
        } else {
            Result::Ok(f)
        }
    }
}

pub open spec fn parse_form(body: Bytes) -> Result<Fields, Bytes> {
    if !valid_utf8(body) {
        Result::Err(lit("ui: verdict: body not decodable"@))
    } else {
        let xs = if body.len() == 0 {
            Seq::empty()
        } else {
            check::split_on(body, 38)
        };
        if xs.len() > 32 {
            Result::Err(lit("ui: verdict: body not parseable"@))
        } else {
            match form_pairs(xs) {
                Result::Err(e) => Result::Err(e),
                Result::Ok(ps) => parse_fields(ps),
            }
        }
    }
}

pub open spec fn record_line(r: Record) -> Bytes {
    let d = r.decision;
    join(
        seq![
            d.docid,
            d.digest,
            d.commit,
            if d.approved {
                lit("approved"@)
            } else {
                lit("rejected"@)
            },
            r.reviewer,
            d.date,
            r.comment,
        ],
        lit("\t"@),
    )
}

pub open spec fn insert_position(rows: Seq<Bytes>, key: Bytes, i: nat, last: nat) -> nat
    decreases rows.len() - i,
{
    if i >= rows.len() {
        last
    } else {
        let f = check::tab_fields(rows[i as int]);
        let oldkey = at(f, 0) + lit("\t"@) + at(f, 5);
        insert_position(
            rows,
            key,
            i + 1,
            if !bytes_lt(key, oldkey) {
                i + 1
            } else {
                last
            },
        )
    }
}

pub open spec fn ledger_candidate(old: Bytes, decision: Record) -> Bytes {
    let rows = raw_rows(old);
    let pos = insert_position(
        rows,
        decision.decision.docid + lit("\t"@) + decision.decision.date,
        0,
        0,
    );
    let i = if pos <= rows.len() {
        pos as int
    } else {
        rows.len() as int
    };
    check::ledger_header() + join(
        rows.take(i) + seq![record_line(decision)] + rows.skip(i),
        lit("\n"@),
    ) + lit("\n"@)
}

pub open spec fn last_review(bs: Seq<Bundle>, id: Bytes, acc: Bytes) -> Bytes
    decreases bs.len(),
{
    if bs.len() == 0 {
        acc
    } else {
        last_review(
            bs.drop_first(),
            id,
            if bs[0].docid == id {
                bs[0].review
            } else {
                acc
            },
        )
    }
}

pub open spec fn find_guideline(gs: Seq<PostGuideline>, gid: Bytes) -> Option<PostGuideline>
    decreases gs.len(),
{
    if gs.len() == 0 {
        Option::None
    } else if gs[0].gid == gid {
        Option::Some(gs[0])
    } else {
        find_guideline(gs.drop_first(), gid)
    }
}

pub open spec fn find_document(ds: Seq<PostDocument>, id: Bytes) -> Option<PostDocument>
    decreases ds.len(),
{
    if ds.len() == 0 {
        Option::None
    } else if ds[0].docid == id {
        Option::Some(ds[0])
    } else {
        find_document(ds.drop_first(), id)
    }
}

pub open spec fn prepare_candidate(
    g: PostGuideline,
    d: PostDocument,
    f: Fields,
    now: Bytes,
) -> PostOutcome {
    let fresh = match g.fresh {
        Result::Ok(bs) => last_review(bs, d.docid, empty()),
        Result::Err(_) => empty(),
    };
    match g.fresh {
        Result::Err(e) => server_error(lit("ui: verdict: manifest derivation failed: "@) + e),
        Result::Ok(_) => if fresh.len() == 0 {
            server_error(lit("ui: verdict: manifest derivation failed: docid row missing"@))
        } else if f.review != fresh {
            refusal(
                409,
                lit("Conflict"@),
                lit("ui: verdict: subject changed"@),
                lit(
                    "The document or its source changed after this page was loaded. The decision was not recorded. Open the document page again and check the current version."@,
                ),
            )
        } else if f.ledger != g.ledger_digest {
            ledger_changed()
        } else {
            let r = Record {
                decision: Decision {
                    docid: d.docid,
                    digest: f.review,
                    commit: d.commit,
                    approved: f.verdict == lit("approved"@),
                    date: now,
                },
                reviewer: f.reviewer,
                comment: f.comment,
            };
            let candidate = ledger_candidate(ledger_data(g.ledger), r);
            let checked = check::ledger(
                Src::Bytes(candidate),
                g.documents.map_values(|x: PostDocument| x.docid),
            );
            match checked.1 {
                Option::Some(v) => server_error(
                    lit("ui: adjudication ledger invalid: "@) + check::strip_ws(check::render(v).1),
                ),
                Option::None => {
                    let response = error_page(
                        303,
                        lit("Decision recorded"@),
                        fixed("<p>The decision was recorded.</p>"@),
                    );
                    PostOutcome::Prepared {
                        candidate,
                        expected_ledger: g.ledger_digest,
                        response: Response {
                            location: lit("/g/"@) + url_seg(g.gid) + lit("/doc/"@) + url_seg(
                                d.docid,
                            ) + lit(".html"@),
                            ..response
                        },
                    }
                },
            }
        },
    }
}

pub open spec fn handle_post(
    req: Request,
    s: PostState,
    g: PostGuideline,
    d: PostDocument,
) -> PostOutcome {
    let expected = lit("http://127.0.0.1:"@) + check::nat_bytes(s.port);
    if req.origin.is_some() && req.origin != Option::Some(expected) {
        forbidden(lit("ui: verdict: origin not allowed"@))
    } else if req.content_type != lit("application/x-www-form-urlencoded"@) {
        bad_form(lit("ui: verdict: unsupported content type"@))
    } else {
        match req.body {
            Option::None => bad_form(lit("ui: verdict: missing body"@)),
            Option::Some(b) => match parse_form(b) {
                Result::Err(e) => bad_form(e),
                Result::Ok(f) => if s.token.len() == 0 || f.csrf != s.token {
                    forbidden(lit("ui: verdict: invalid csrf token"@))
                } else {
                    match d.render_error {
                        Option::Some(e) => server_error(e),
                        Option::None => prepare_candidate(g, d, f, s.now),
                    }
                },
            },
        }
    }
}

pub open spec fn post_outcome(req: Request, s: PostState) -> PostOutcome {
    let route = doc_route(req.path);
    if req.host != lit("127.0.0.1:"@) + check::nat_bytes(s.port) {
        forbidden(lit("ui: request: host not allowed"@))
    } else if req.method != lit("GET"@) && (req.method != lit("POST"@) || route.is_none()) {
        method_response(route.is_some())
    } else {
        match s.models {
            Result::Err(e) => server_error(e),
            Result::Ok(gs) => if req.method == lit("GET"@) {
                PostOutcome::Read
            } else {
                match route {
                    Option::None => not_found(),
                    Option::Some((gid, id)) => match find_guideline(gs, gid) {
                        Option::None => not_found(),
                        Option::Some(g) => match find_document(g.documents, id) {
                            Option::None => not_found(),
                            Option::Some(d) => handle_post(req, s, g, d),
                        },
                    },
                }
            },
        }
    }
}

// --- Executable mirrors: kernel entry points bind through View. ---
pub enum EPiece {
    Fixed(Vec<u8>),
    Text(Vec<u8>),
    Attr(Vec<u8>),
    Decimal(u64),
    Comment(Vec<u8>),
}

impl View for EPiece {
    type V = Piece;

    open spec fn view(&self) -> Piece {
        match self {
            EPiece::Fixed(b) => Piece::Fixed(b@),
            EPiece::Text(b) => Piece::Text(b@),
            EPiece::Attr(b) => Piece::Attr(b@),
            EPiece::Decimal(n) => Piece::Decimal(*n as nat),
            EPiece::Comment(b) => Piece::Comment(b@),
        }
    }
}

pub struct EPage {
    pub parts: Vec<EPiece>,
}

impl View for EPage {
    type V = Html;

    open spec fn view(&self) -> Html {
        self.parts@.map_values(|p: EPiece| p@)
    }
}

pub struct ERequest {
    pub method: Vec<u8>,
    pub path: Vec<u8>,
    pub host: Vec<u8>,
    pub origin: Option<Vec<u8>>,
    pub content_type: Vec<u8>,
    pub body: Option<Vec<u8>>,
}

impl View for ERequest {
    type V = Request;

    open spec fn view(&self) -> Request {
        Request {
            method: self.method@,
            path: self.path@,
            host: self.host@,
            origin: match self.origin {
                Some(b) => Some(b@),
                None => None,
            },
            content_type: self.content_type@,
            body: match self.body {
                Some(b) => Some(b@),
                None => None,
            },
        }
    }
}

pub struct ERecord {
    pub docid: Vec<u8>,
    pub digest: Vec<u8>,
    pub commit: Vec<u8>,
    pub approved: bool,
    pub reviewer: Vec<u8>,
    pub date: Vec<u8>,
    pub comment: Vec<u8>,
}

impl View for ERecord {
    type V = Record;

    open spec fn view(&self) -> Record {
        Record {
            decision: Decision {
                docid: self.docid@,
                digest: self.digest@,
                commit: self.commit@,
                approved: self.approved,
                date: self.date@,
            },
            reviewer: self.reviewer@,
            comment: self.comment@,
        }
    }
}

pub struct EPostDocument {
    pub docid: Vec<u8>,
    pub render_error: Option<Vec<u8>>,
    pub commit: Vec<u8>,
}

impl View for EPostDocument {
    type V = PostDocument;

    open spec fn view(&self) -> PostDocument {
        PostDocument {
            docid: self.docid@,
            render_error: match self.render_error {
                Some(b) => Some(b@),
                None => None,
            },
            commit: self.commit@,
        }
    }
}

pub struct EPostGuideline {
    pub gid: Vec<u8>,
    pub documents: Vec<EPostDocument>,
    pub fresh: Result<Vec<check::EBundle>, Vec<u8>>,
    pub ledger: crate::replay::ESrc,
    pub ledger_digest: Vec<u8>,
}

impl View for EPostGuideline {
    type V = PostGuideline;

    open spec fn view(&self) -> PostGuideline {
        PostGuideline {
            gid: self.gid@,
            documents: self.documents@.map_values(|d: EPostDocument| d@),
            fresh: match self.fresh {
                Ok(bs) => Ok(bs@.map_values(|b: check::EBundle| b@)),
                Err(e) => Err(e@),
            },
            ledger: self.ledger@,
            ledger_digest: self.ledger_digest@,
        }
    }
}

pub struct EPostState {
    pub port: u16,
    pub token: Vec<u8>,
    pub models: Result<Vec<EPostGuideline>, Vec<u8>>,
    pub now: Vec<u8>,
}

impl View for EPostState {
    type V = PostState;

    open spec fn view(&self) -> PostState {
        PostState {
            port: self.port as nat,
            token: self.token@,
            models: match self.models {
                Ok(gs) => Ok(gs@.map_values(|g: EPostGuideline| g@)),
                Err(e) => Err(e@),
            },
            now: self.now@,
        }
    }
}

pub struct EResponse {
    pub status: u16,
    pub body: Vec<u8>,
    pub allow: Vec<u8>,
    pub location: Vec<u8>,
}

impl View for EResponse {
    type V = Response;

    open spec fn view(&self) -> Response {
        Response {
            status: self.status as nat,
            body: self.body@,
            allow: self.allow@,
            location: self.location@,
        }
    }
}

pub enum EPostOutcome {
    Read,
    Refused(EResponse),
    Prepared { candidate: Vec<u8>, expected_ledger: Vec<u8>, response: EResponse },
}

impl View for EPostOutcome {
    type V = PostOutcome;

    open spec fn view(&self) -> PostOutcome {
        match self {
            EPostOutcome::Read => PostOutcome::Read,
            EPostOutcome::Refused(r) => PostOutcome::Refused(r@),
            EPostOutcome::Prepared {
                candidate,
                expected_ledger,
                response,
            } => PostOutcome::Prepared {
                candidate: candidate@,
                expected_ledger: expected_ledger@,
                response: response@,
            },
        }
    }
}

// --- Fidelity predicates over typed slots, whose serialization is render_page.
// Provenance is byte provenance, not NL-to-ACE fidelity. Exact renderer equality
// owns transformations; this predicate additionally limits each visible slot to
// copied corpus spans, declared copy, and canonical decimal sequences.
pub open spec fn corpus_bytes(c: Corpus) -> Seq<Bytes> {
    c.guidelines.map_values(
        |g: Guideline|
            seq![g.gid, ledger_data(g.ledger)] + (match g.readme {
                Some(b) => seq![b],
                None => Seq::empty(),
            }) + g.coverage.rows.map_values(|r: Row| seq![r.id, r.line]).flatten()
                + g.coverage.evidence.map_values(
                |e: check::Evidence|
                    e.ordinal + e.payloads.map_values(|p: (Bytes, Seq<Bytes>)| p.1).flatten(),
            ).flatten() + g.documents.map_values(
                |d: Document| seq![d.bundle.docid, d.ace, d.pl],
            ).flatten(),
    ).flatten()
}

pub open spec fn copied_span(s: Bytes, inputs: Seq<Bytes>) -> bool {
    exists|i: int|
        0 <= i < inputs.len() && check::first_sub(#[trigger] inputs[i], s, 0) + s.len()
            <= inputs[i].len()
}

pub open spec fn copy_derived(s: Bytes, inputs: Seq<Bytes>, registry: Seq<Bytes>) -> bool
    decreases s.len(),
{
    s.len() == 0 || exists|k: int|
        0 < k <= s.len() && (copied_span(#[trigger] s.take(k), inputs) || registry.contains(
            s.take(k),
        ) || (digits(s.take(k)) && (k == 1 || s[0] != 48))) && copy_derived(
            s.skip(k),
            inputs,
            registry,
        )
}

pub open spec fn visible_bytes_from(page: Html, corpus: Corpus, registry: Seq<Bytes>) -> bool {
    forall|i: int|
        0 <= i < page.len() ==> match #[trigger] page[i] {
            Piece::Fixed(b) => registry.contains(b),
            Piece::Text(b) => copy_derived(b, corpus_bytes(corpus), registry),
            Piece::Decimal(_) => true,
            Piece::Attr(_) => true,
            Piece::Comment(_) => true,
        }
}

pub open spec fn well_escaped(page: Html) -> bool {
    escaped_slots(page, 0)
}

// --- Declared static copy/chrome: each literal that reaches a page. ---
pub open spec fn copy_registry() -> Seq<Bytes> {
    seq![
        lit("&amp;"@),
        lit("&lt;"@),
        lit("&gt;"@),
        lit("&quot;"@),
        lit("&#x27;"@),
        lit("\n"@),
        lit("<td>"@),
        lit("</td>"@),
        lit("<tr>"@),
        lit("</tr>"@),
        lit("<a href=\""@),
        lit("\">"@),
        lit("</a>"@),
        lit("approved"@),
        lit("rejected"@),
        lit("contested"@),
        lit("stale"@),
        lit("unreviewed"@),
        lit("Approved"@),
        lit("Rejected"@),
        lit("Contested"@),
        lit("Outdated"@),
        lit("Unreviewed"@),
        lit("<span class=\"chip chip-"@),
        lit("</span>"@),
        lit("# "@),
        lit("Rec"@),
        lit("Recommendation "@),
        lit("BOX"@),
        lit(" · "@),
        lit("p"@),
        lit(", page "@),
        lit(", passage "@),
        lit(" ("@),
        lit(")"@),
        lit("Z"@),
        lit(" "@),
        lit(" UTC"@),
        css_text(),
        script_html(),
        lit("<!doctype html>"@),
        lit("<html lang=\"en\">"@),
        lit("<head>"@),
        lit("<meta charset=\"utf-8\">"@),
        lit("<title>"@),
        lit(" — cnl-ckc reviewer</title>"@),
        lit("<style>"@),
        lit("</style>"@),
        lit("</head>"@),
        lit("<body>"@),
        lit("<a class=\"skip\" href=\"#main\">Skip to content</a>"@),
        lit("<nav class=\"crumbs\">"@),
        lit("</nav>"@),
        lit("<main id=\"main\">"@),
        lit("</main>"@),
        lit(
            "<footer class=\"scope\"><p>This page reports what the loaded guideline documents state. It does not give clinical advice.</p></footer>"@,
        ),
        lit("</body>"@),
        lit("</html>"@),
        lit(" approved"@),
        lit(" rejected"@),
        lit(" earlier"@),
        lit("None"@),
        lit(", "@),
        lit("Decisions on this version: "@),
        lit(" and "@),
        lit("."@),
        lit("No decision is recorded on this version."@),
        lit("No decision is recorded."@),
        lit(" Decisions on earlier versions: "@),
        lit("No decisions are recorded for the "@),
        lit(" documents in this guideline."@),
        lit("Reviewers recorded "@),
        lit(" decisions on "@),
        lit(" of "@),
        lit(" documents."@),
        lit("g/"@),
        lit("/index.html"@),
        lit("Guidelines"@),
        lit("cnl-ckc reviewer"@),
        lit("<h1>Guidelines</h1>"@),
        lit("<section>"@),
        lit("<table>"@),
        lit(
            "<thead><tr><th>Guideline</th><th>Documents</th><th>Passages</th><th>Approved</th><th>Rejected</th><th>Contested</th><th>Outdated</th><th>Unreviewed</th></tr></thead>"@,
        ),
        lit("<tbody>"@),
        lit("</tbody>"@),
        lit("</table>"@),
        lit("</section>"@),
        lit("Restates "@),
        lit("restates("@),
        lit("uncovered("@),
        lit(": "@),
        lit("Not covered — "@),
        lit("Pending"@),
        lit("Passages"@),
        lit("With ACE"@),
        lit("doc/"@),
        lit(".html"@),
        lit("<h1>"@),
        lit("</h1>"@),
        lit("<p>"@),
        lit(" <a href=\"records.html\">All decision records</a></p>"@),
        lit("<h2>Status</h2>"@),
        lit("<table class=\"compact\">"@),
        lit("<thead><tr><th>Status</th><th>Count</th></tr></thead>"@),
        lit("<h2>Documents</h2>"@),
        lit(
            "<thead><tr><th>Document</th><th>Status</th><th>Decisions</th><th>Passage</th></tr></thead>"@,
        ),
        lit("<h2>Passages without ACE</h2>"@),
        lit("<thead><tr><th>Passage</th><th>Status</th><th>Section</th></tr></thead>"@),
        lit("<a href=\"../../index.html\">guidelines</a> / "@),
        lit("https://github.com/eturkes/cnl-ckc/commit/"@),
        lit("Current"@),
        lit("Earlier"@),
        lit("Not given"@),
        lit("<section id=\""@),
        lit("<h2>"@),
        lit("</h2>"@),
        lit("<table class=\"records\">"@),
        lit(
            "<thead><tr><th>Decision</th><th>Reviewer</th><th>Date</th><th>Version</th><th>Comment</th></tr></thead>"@,
        ),
        lit(" The newest decision for each document is first."@),
        lit("<p>Open a document and record a decision to start this list.</p>"@),
        lit("<p>Each reviewer name is recorded as entered and is not verified.</p>"@),
        lit("<p>Each version links to the stored version of the text that the reviewer read.</p>"@),
        lit("Decision records"@),
        lit("<a href=\"../../index.html\">guidelines</a> / <a href=\"index.html\">"@),
        lit("</a> / records"@),
        lit("<h1>Decision records</h1>"@),
        lit("</p>"@),
        lit("<nav class=\"docnav\"><a href=\"index.html\">Guideline index</a></nav>"@),
        lit("<span class=\"kw\">"@),
        lit("<mark class=\"t"@),
        lit("<mark>"@),
        lit("</mark>"@),
        lit("<datalist id=\"reviewer-names\">"@),
        lit("<option value=\""@),
        lit("\"></option>"@),
        lit("</datalist>"@),
        lit(".pdf"@),
        lit(" <a class=\"source\" href=\"../source/"@),
        lit("\">PDF</a>"@),
        lit("source/"@),
        lit("../source/"@),
        lit("Source text"@),
        lit("../records.html"@),
        lit("#"@),
        lit("Previous document"@),
        lit("<a href=\"../index.html\">Guideline index</a>"@),
        lit("Next document"@),
        lit("All decision records"@),
        lit("<section class=\"stale\">"@),
        lit(
            "<p>The document or its source changed after the last decision. No recorded decision applies to the version shown here.</p>"@,
        ),
        lit(
            "<p class=\"hl-note\"><label><input type=\"checkbox\" class=\"hl-toggle\" checked> Highlighting</label> Try hovering and clicking on highlighted terms for different levels of emphasis.</p>"@,
        ),
        lit("<h3>Original passage</h3>"@),
        lit("<pre class=\"prose\">"@),
        lit("</pre>"@),
        lit("<h3>Attempto Controlled English (ACE)</h3>"@),
        lit("<section class=\"verdict-entry\">"@),
        lit("<h3>Record a decision</h3>"@),
        lit("<p>Does the ACE representation appropriately reflect the original passage?</p>"@),
        lit("<form method=\"post\">"@),
        lit("<fieldset>"@),
        lit("<legend>Decision</legend>"@),
        lit(
            "<label><input type=\"radio\" name=\"verdict\" value=\"approved\" required> Approved</label>"@,
        ),
        lit(
            "<label><input type=\"radio\" name=\"verdict\" value=\"rejected\" required> Rejected</label>"@,
        ),
        lit("</fieldset>"@),
        lit("<label for=\"reviewer\">Reviewer name</label>"@),
        lit(
            "<input type=\"text\" id=\"reviewer\" name=\"reviewer\" list=\"reviewer-names\" value=\""@,
        ),
        lit("\" required>"@),
        lit("<label for=\"comment\">Comment (optional)</label>"@),
        lit("<textarea id=\"comment\" name=\"comment\"></textarea>"@),
        lit("<input type=\"hidden\" name=\"review_sha256\" value=\""@),
        lit("<input type=\"hidden\" name=\"ledger_sha256\" value=\""@),
        lit("<input type=\"hidden\" name=\"csrf\" value=\""@),
        lit("<button>Record decision</button>"@),
        lit("</form>"@),
        lit("<details>"@),
        lit("<summary>Compiled Prolog ("@),
        lit(" lines)</summary>"@),
        lit("<pre>"@),
        lit("</details>"@),
        lit("<nav class=\"docnav\">"@),
        lit("<a href=\"../../../index.html\">guidelines</a> / <a href=\"../index.html\">"@),
        lit("</a> / "@),
        lit(
            "a an the every each no all some any this that these those such is are was were be been being has have had does do did should must may can cannot might will would shall could if then and or nor but not it its itself they them their he she who whom whose which what where when there something somebody someone everything everybody everyone nothing nobody of for with without during to at in on by from as against about after before through under over above below into onto per within between among around near than least most more less fewer greater"@,
        ),
        lit("</p>\n<!-- "@),
        lit(" -->"@),
        lit(
            "The request was refused. Open the document page again from this site and submit the decision again."@,
        ),
        lit(
            "The submitted form was not valid. Go back to the document page, reload it, and submit the decision again."@,
        ),
        lit("Forbidden"@),
        lit("Bad request"@),
        lit("Server error"@),
        lit("The server could not complete the request. Reload the page and try again."@),
        lit("Conflict"@),
        lit("ui: verdict: ledger changed"@),
        lit(
            "Another decision was recorded for this guideline before this one. The decision was not recorded. Open the document page again and check the current state."@,
        ),
        lit("/g/"@),
        lit("doc"@),
        lit("Method not allowed"@),
        lit("<p>Only GET and POST are supported on this page.</p>"@),
        lit("<p>Only GET is supported on this page.</p>"@),
        lit("GET, POST"@),
        lit("GET"@),
        lit("Not found"@),
        lit("<p>The requested page does not exist.</p>"@),
        lit("="@),
        lit("ui: verdict: body not parseable"@),
        lit("verdict"@),
        lit("reviewer"@),
        lit("comment"@),
        lit("review_sha256"@),
        lit("ledger_sha256"@),
        lit("csrf"@),
        lit("ui: verdict: missing field "@),
        lit("ui: verdict: duplicate field "@),
        lit("ui: verdict: unknown field "@),
        lit("ui: verdict: invalid verdict"@),
        lit("ui: verdict: invalid reviewer"@),
        lit("ui: verdict: invalid comment"@),
        lit("ui: verdict: invalid review_sha256"@),
        lit("absent"@),
        lit("ui: verdict: invalid ledger_sha256"@),
        lit("ui: verdict: body not decodable"@),
        lit("\t"@),
        lit("ui: verdict: manifest derivation failed: "@),
        lit("ui: verdict: manifest derivation failed: docid row missing"@),
        lit("ui: verdict: subject changed"@),
        lit(
            "The document or its source changed after this page was loaded. The decision was not recorded. Open the document page again and check the current version."@,
        ),
        lit("ui: adjudication ledger invalid: "@),
        lit("Decision recorded"@),
        lit("<p>The decision was recorded.</p>"@),
        lit("/doc/"@),
        lit("http://127.0.0.1:"@),
        lit("ui: verdict: origin not allowed"@),
        lit("application/x-www-form-urlencoded"@),
        lit("ui: verdict: unsupported content type"@),
        lit("ui: verdict: missing body"@),
        lit("ui: verdict: invalid csrf token"@),
        lit("127.0.0.1:"@),
        lit("ui: request: host not allowed"@),
        lit("POST"@),
    ]
}

// --- Copy gate: tools/goal.py:3893–3934; enumerated copy domain (R76).
pub open spec fn ascii_word(c: char) -> bool {
    ('A' <= c <= 'Z') || ('a' <= c <= 'z') || ('0' <= c <= '9') || c == '_'
}

// Python IGNORECASE and lower() coincide for the admitted copy alphabet.
pub open spec fn copy_fold(c: char) -> char {
    if 'A' <= c <= 'Z' {
        (c as u32 + 32) as char
    } else {
        c
    }
}

pub open spec fn copy_match(s: Seq<char>, p: Seq<char>, i: int, words: bool) -> bool {
    0 <= i && i + p.len() <= s.len() && (forall|j: int|
        0 <= j < p.len() ==> copy_fold(#[trigger] s[i + j]) == p[j]) && (!words || ((i == 0
        || !ascii_word(s[i - 1])) && (i + p.len() == s.len() || !ascii_word(
        s[i + p.len() as int],
    ))))
}

pub open spec fn copy_contains(s: Seq<char>, patterns: Seq<Bytes>, words: bool) -> bool {
    exists|i: int, j: int|
        0 <= i < s.len() && 0 <= j < patterns.len() && #[trigger] copy_match(
            s,
            chars(patterns[j]),
            i,
            words,
        )
}

// Emoji retains the legacy first-pass diagnostic, before domain admission.
pub open spec fn emoji(c: char) -> bool {
    let n = c as int;
    126975 < n < 129792 || 9727 < n < 10176 || 11007 < n < 11264 || n == 65039 || n == 8205
}

pub open spec fn css_tokens() -> Seq<Bytes> {
    check::split_on(
        lit(
            "linear-gradient radial-gradient conic-gradient @keyframes animation transition backdrop-filter box-shadow"@,
        ),
        32,
    )
}

pub open spec fn marketing_tokens() -> Seq<Bytes> {
    check::split_on(
        lit(
            "simply seamless seamlessly powerful robust robustly leverage leverages leveraged leveraging effortless effortlessly intuitive streamline streamlined unlock empower empowering cutting-edge state-of-the-art world-class blazing stunning delightful revolutionize game-changing supercharge best-in-class next-generation"@,
        ),
        32,
    )
}

pub open spec fn relative_tokens() -> Seq<Bytes> {
    seq![
        lit("ago"@),
        lit("just now"@),
        lit("yesterday"@),
        lit("tomorrow"@),
        lit("recently"@),
        lit("last week"@),
        lit("last month"@),
        lit("last year"@),
    ]
}

// Format the scalar exactly as Python format(code_point, "04X").
pub open spec fn copy_hex(n: nat) -> Bytes
    decreases n,
{
    if n < 65536 {
        v1text::uhex4(n as int)
    } else {
        copy_hex(n / 16) + seq![v1text::uhex_digit((n % 16) as int)]
    }
}

pub open spec fn copy_admitted(c: char) -> bool {
    let n = c as int;
    n < 128 || n == 0x00A7 || n == 0x00B7 || n == 0x2014 || n == 0x2265
}

pub open spec fn first_copy_char(s: Seq<char>, p: spec_fn(char) -> bool) -> Option<char>
    decreases s.len(),
{
    if s.len() == 0 {
        None
    } else if p(s[0]) {
        Some(s[0])
    } else {
        first_copy_char(s.drop_first(), p)
    }
}

// CSS uses list order, not source position: animation wins over an earlier transition.
pub open spec fn first_css(s: Seq<char>, patterns: Seq<Bytes>) -> Option<Bytes>
    decreases patterns.len(),
{
    if patterns.len() == 0 {
        None
    } else if copy_contains(s, seq![patterns[0]], false) {
        Some(patterns[0])
    } else {
        first_css(s, patterns.drop_first())
    }
}

pub open spec fn word_pattern_at(s: Seq<char>, patterns: Seq<Bytes>, i: nat) -> Option<nat>
    decreases patterns.len(),
{
    if patterns.len() == 0 {
        None
    } else if copy_match(s, chars(patterns[0]), i as int, true) {
        Some(chars(patterns[0]).len())
    } else {
        word_pattern_at(s, patterns.drop_first(), i)
    }
}

// re.search: leftmost source position; alternatives retain registry order.
pub open spec fn first_copy_word(s: Seq<char>, patterns: Seq<Bytes>, i: nat) -> Option<Bytes>
    decreases s.len() - i,
{
    if i >= s.len() {
        None
    } else {
        match word_pattern_at(s, patterns, i) {
            Some(n) => if i + n <= s.len() {
                Some(encode_utf8(s.subrange(i as int, (i + n) as int)))
            } else {
                None
            },
            None => first_copy_word(s, patterns, i + 1),
        }
    }
}

pub open spec fn first_exclamation(s: Seq<char>, i: nat) -> Option<Bytes>
    decreases s.len() - i,
{
    if i + 1 >= s.len() {
        None
    } else if ascii_word(s[i as int]) && s[i as int] != '_' && s[i as int + 1] == '!' {
        Some(encode_utf8(s.subrange(i as int, i as int + 2)))
    } else {
        first_exclamation(s, i + 1)
    }
}

pub open spec fn copy_token_violation(s: Seq<char>) -> Option<Bytes> {
    match first_css(s, css_tokens()) {
        Some(p) => Some(lit("css: "@) + p),
        None => match first_copy_word(s, marketing_tokens(), 0) {
            Some(p) => Some(lit("marketing: "@) + p),
            None => match first_copy_word(s, relative_tokens(), 0) {
                Some(p) => Some(lit("relative-time: "@) + p),
                None => match first_exclamation(s, 0) {
                    Some(p) => Some(lit("exclamatory: "@) + p),
                    None => None,
                },
            },
        },
    }
}

// Decode -> legacy emoji pass -> repertoire -> CSS -> marketing -> relative -> !.
pub open spec fn copy_violation(b: Bytes) -> Option<Bytes> {
    if !valid_utf8(b) {
        Some(lit("domain: invalid UTF-8"@))
    } else {
        let s = chars(b);
        match first_copy_char(s, |c: char| emoji(c)) {
            Some(c) => Some(lit("emoji: U+"@) + copy_hex(c as nat)),
            None => match first_copy_char(s, |c: char| !copy_admitted(c)) {
                Some(c) => Some(lit("domain: U+"@) + copy_hex(c as nat)),
                None => copy_token_violation(s),
            },
        }
    }
}

pub open spec fn copy_literal_ok(b: Bytes) -> bool {
    copy_violation(b).is_none()
}

pub open spec fn copy_ok(registry: Seq<Bytes>) -> bool {
    forall|i: int| 0 <= i < registry.len() ==> copy_literal_ok(#[trigger] registry[i])
}

// Slot types must agree with their actual template context: 0 text, 1 tag,
// 2 double-quoted attribute, 3 single-quoted attribute, 4 comment, 5 style,
// 6 script. The two fixed raw-text elements carry no dynamic slots.
pub open spec fn fixed_context(s: Bytes, context: int) -> int
    decreases s.len(),
{
    if s.len() == 0 {
        context
    } else {
        let (n, next): (int, int) = if context == 0 && check::starts(s, ascii("<style>"@)) {
            (7, 5)
        } else if context == 0 && check::starts(s, ascii("<script>"@)) {
            (8, 6)
        } else if context == 0 && check::starts(s, ascii("<!--"@)) {
            (4, 4)
        } else if context == 4 && check::starts(s, ascii("-->"@)) {
            (3, 0)
        } else if context == 5 && check::starts(s, ascii("</style>"@)) {
            (8, 0)
        } else if context == 6 && check::starts(s, ascii("</script>"@)) {
            (9, 0)
        } else {
            (
                1,
                if context == 0 && s[0] == 60 {
                    1
                } else if context == 1 && s[0] == 34 {
                    2
                } else if context == 1 && s[0] == 39 {
                    3
                } else if context == 1 && s[0] == 62 {
                    0
                } else if (context == 2 && s[0] == 34) || (context == 3 && s[0] == 39) {
                    1
                } else {
                    context
                },
            )
        };
        if 0 < n <= s.len() {
            fixed_context(s.skip(n), next)
        } else {
            -1
        }
    }
}

pub open spec fn escaped_slots(page: Html, context: int) -> bool
    decreases page.len(),
{
    if page.len() == 0 {
        context == 0
    } else {
        match page[0] {
            Piece::Fixed(b) => copy_registry().contains(b) && escaped_slots(
                page.drop_first(),
                fixed_context(b, context),
            ),
            Piece::Text(b) => context == 0 && escaped(escape_text(b), false) && escaped_slots(
                page.drop_first(),
                context,
            ),
            Piece::Attr(b) => (context == 2 || context == 3) && escaped(escape_attr(b), true)
                && escaped_slots(page.drop_first(), context),
            Piece::Decimal(n) => (context == 0 || context == 2 || context == 3) && digits(
                check::nat_bytes(n),
            ) && escaped_slots(page.drop_first(), context),
            Piece::Comment(b) => context == 4 && v1text::all_in(
                comment_safe(b),
                |x: u8| is_alnum_b(x) || x == 32 || x == 58 || x == 46,
            ) && escaped_slots(page.drop_first(), context),
        }
    }
}

// Runtime page inputs are check.rs's K4 exec mirrors (R92), not parallel laws.
pub struct EDocument {
    pub bundle: check::EBundle,
    pub ace: Vec<u8>,
    pub pl: Vec<u8>,
    pub alignment: Option<Vec<u8>>,
}

impl View for EDocument {
    type V = Document;

    open spec fn view(&self) -> Document {
        Document {
            bundle: self.bundle@,
            ace: self.ace@,
            pl: self.pl@,
            alignment: match self.alignment {
                Some(x) => Some(x@),
                None => None,
            },
        }
    }
}

pub struct EGuideline {
    pub gid: Vec<u8>,
    pub readme: Option<Vec<u8>>,
    pub coverage: ECoverage,
    pub documents: Vec<EDocument>,
    pub ledger: crate::replay::ESrc,
    pub ledger_digest: Vec<u8>,
    pub source_names: Vec<Vec<u8>>,
}

impl View for EGuideline {
    type V = Guideline;

    open spec fn view(&self) -> Guideline {
        Guideline {
            gid: self.gid@,
            readme: match self.readme {
                Some(x) => Some(x@),
                None => None,
            },
            coverage: self.coverage@,
            documents: self.documents@.map_values(|d: EDocument| d@),
            ledger: self.ledger@,
            ledger_digest: self.ledger_digest@,
            source_names: self.source_names@.map_values(|x: Vec<u8>| x@),
        }
    }
}

pub struct ECorpus {
    pub guidelines: Vec<EGuideline>,
    pub token: Vec<u8>,
}

impl View for ECorpus {
    type V = Corpus;

    open spec fn view(&self) -> Corpus {
        Corpus { guidelines: self.guidelines@.map_values(|g: EGuideline| g@), token: self.token@ }
    }
}

} // verus!
