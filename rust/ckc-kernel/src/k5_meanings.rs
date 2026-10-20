// Contract ur: the Word meanings section + the reading words of a document page.
use crate::m7_temporal::{EVocabulary, parse_vocabulary_exec};
#[cfg(verus_keep_ghost)]
use crate::m7_temporal::{lemmas_view, pairs_view};
use crate::{
    k4_bytes as order, k5_bytes as b, k5_codes as c, k5_document_data as data, k5_highlight as hl,
    k5_html as h,
};
#[cfg(verus_keep_ghost)]
use ckc_spec::align::OutSpan;
use ckc_spec::align::{EModel, ESpan};
use ckc_spec::ui::{self as u, EDocument, EGuideline, EPage};
use ckc_spec::vocabulary as voc;
use vstd::prelude::*;
use vstd::slice::slice_to_vec;
verus! {

// A reading word: characters [start, end) of the ACE text + its table lemma.
pub struct EReading {
    pub start: u64,
    pub end: u64,
    pub lemma: Vec<u8>,
}

impl View for EReading {
    type V = u::Reading;

    open spec fn view(&self) -> u::Reading {
        u::Reading { start: self.start as int, end: self.end as int, lemma: self.lemma@ }
    }
}

pub open spec fn readings(xs: Seq<EReading>) -> Seq<u::Reading> {
    xs.map_values(|x: EReading| x@)
}

pub fn is(a: &[u8], s: &str) -> (out: bool)
    ensures
        out == (a@ == u::lit(s@)),
{
    let b = crate::k5_bytes::literal(s);
    crate::k4_bytes::eq(a, &b)
}

// u::pl_word: the word as visible text when its bytes occur in the table.
pub fn pl_word_exec(pl: &[u8], w: &[u8]) -> (out: EPage)
    ensures
        out@ == u::pl_word(pl@, w@),
{
    let nf: &str = "not stated";
    if w.len() == 0 || w.len() > pl.len() {
        proof {
            if w@.len() > 0 {
                assert(ckc_spec::check::first_sub(pl@, w@, 0) == pl@.len());
            }
        }
        return h::fixed(nf);
    }
    let last = pl.len() - w.len();
    let mut i = 0usize;
    while i <= last
        invariant
            0 < w@.len() <= pl@.len(),
            last == pl@.len() - w@.len(),
            i <= last + 1,
            ckc_spec::check::first_sub(pl@, w@, 0) == ckc_spec::check::first_sub(pl@, w@, i as nat),
        decreases last + 1 - i,
    {
        proof {
            assert(i + w@.len() <= pl@.len());
        }
        let end = pl.len() - (last - i);
        let sub = vstd::slice::slice_subrange(pl, i, end);
        if crate::k4_bytes::eq(sub, w) {
            proof {
                assert(pl@.subrange(i as int, i + w@.len()) =~= sub@);
            }
            return h::text(w);
        }
        proof {
            assert(pl@.subrange(i as int, i + w@.len()) =~= sub@);
        }
        i += 1;
    }
    h::fixed(nf)
}

pub fn kind_exec(kind: &[u8]) -> (out: EPage)
    ensures
        out@ == u::kind_html(kind@),
{
    if is(kind, "usuality") {
        h::fixed("How often")
    } else if is(kind, "ideality") {
        h::fixed("Best case")
    } else if is(kind, "focus") {
        h::fixed("Emphasis")
    } else if is(kind, "manner") {
        h::fixed("Manner")
    } else if is(kind, "force") {
        h::fixed("Strength of wording")
    } else if is(kind, "possibility") {
        h::fixed("Possibility")
    } else if is(kind, "applicability") {
        h::fixed("Condition of use")
    } else {
        h::fixed("not stated")
    }
}

pub fn row2(a: EPage, b: EPage) -> (out: EPage)
    ensures
        out@ == u::word_row(a@, b@),
{
    let ghost av = a@;
    let ghost bv = b@;
    let mut cells: Vec<EPage> = Vec::new();
    cells.push(h::cell(a));
    cells.push(h::cell(b));
    proof {
        assert(h::pages(cells@) =~= seq![u::cell(av), u::cell(bv)]);
    }
    h::row(&cells)
}

pub fn no_rows() -> (out: Vec<EPage>)
    ensures
        h::pages(out@) == Seq::<u::Html>::empty(),
{
    let out: Vec<EPage> = Vec::new();
    proof {
        assert(h::pages(out@) =~= Seq::<u::Html>::empty());
    }
    out
}

// --- D3: readings of the table rows.
pub open spec fn rows_view(xs: Seq<(Vec<u8>, EPage)>) -> Seq<(u::Bytes, u::Html)> {
    xs.map_values(|x: (Vec<u8>, EPage)| (x.0@, x.1@))
}

pub fn reading_of_exec(kind: u8, v: &[u8]) -> (out: EPage)
    ensures
        out@ == u::reading_of(kind as int, v@),
{
    if kind == 0 && is(v, "strong") {
        h::fixed("a strong recommendation")
    } else if kind == 0 && is(v, "conditional") {
        h::fixed("a conditional recommendation")
    } else if kind == 1 && is(v, "high") {
        h::fixed("high certainty of the evidence")
    } else if kind == 1 && is(v, "moderate") {
        h::fixed("moderate certainty of the evidence")
    } else if kind == 1 && is(v, "low") {
        h::fixed("low certainty of the evidence")
    } else if kind == 1 && is(v, "very_low") {
        h::fixed("very low certainty of the evidence")
    } else if kind == 2 {
        h::fixed("the preferred option")
    } else if kind == 3 {
        h::fixed("the alternative to the preferred option")
    } else if kind == 4 {
        h::cat(h::fixed("a qualifier: "), kind_exec(v))
    } else if kind == 5 {
        h::fixed("every recommendation of this guideline")
    } else if kind == 6 {
        h::fixed(
            "the verb of a scope exclusion: when its condition holds, no recommendation of this guideline applies to the patient",
        )
    } else {
        h::fixed("not stated")
    }
}

// Rows of one kind: (lemma, reading of the row's value | of no value).
fn push_rows(
    out: &mut Vec<(Vec<u8>, EPage)>,
    rows: &Vec<(Vec<u8>, Vec<u8>)>,
    kind: u8,
    valued: bool,
)
    ensures
        rows_view(final(out)@) == rows_view(old(out)@) + pairs_view(rows@).map_values(
            |r: (u::Bytes, u::Bytes)|
                (
                    r.0,
                    u::reading_of(
                        kind as int,
                        if valued {
                            r.1
                        } else {
                            u::empty()
                        },
                    ),
                ),
        ),
{
    let ghost start = rows_view(out@);
    let ghost f = |r: (u::Bytes, u::Bytes)|
        (
            r.0,
            u::reading_of(
                kind as int,
                if valued {
                    r.1
                } else {
                    u::empty()
                },
            ),
        );
    let ghost pv = pairs_view(rows@);
    let none: Vec<u8> = Vec::new();
    let mut i = 0usize;
    while i < rows.len()
        invariant
            i <= rows.len(),
            none@ == u::empty(),
            pv == pairs_view(rows@),
            f == (|r: (u::Bytes, u::Bytes)|
                (
                    r.0,
                    u::reading_of(
                        kind as int,
                        if valued {
                            r.1
                        } else {
                            u::empty()
                        },
                    ),
                )),
            rows_view(out@) == start + pv.take(i as int).map_values(f),
        decreases rows.len() - i,
    {
        let page = if valued {
            reading_of_exec(kind, &rows[i].1)
        } else {
            reading_of_exec(kind, &none)
        };
        let ghost before = rows_view(out@);
        out.push((slice_to_vec(&rows[i].0), page));
        proof {
            assert(pv[i as int] == (rows@[i as int].0@, rows@[i as int].1@));
            assert(pv.take(i + 1).map_values(f) =~= pv.take(i as int).map_values(f).push(
                f(pv[i as int]),
            ));
            assert(rows_view(out@) =~= before.push(f(pv[i as int])));
        }
        i += 1;
    }
    proof {
        assert(pv.take(i as int) =~= pv);
    }
}

fn push_lemmas(out: &mut Vec<(Vec<u8>, EPage)>, lemmas: &Vec<Vec<u8>>, kind: u8)
    ensures
        rows_view(final(out)@) == rows_view(old(out)@) + lemmas_view(lemmas@).map_values(
            |a: u::Bytes| (a, u::reading_of(kind as int, u::empty())),
        ),
{
    let ghost start = rows_view(out@);
    let ghost f = |a: u::Bytes| (a, u::reading_of(kind as int, u::empty()));
    let ghost lv = lemmas_view(lemmas@);
    let none: Vec<u8> = Vec::new();
    let mut i = 0usize;
    while i < lemmas.len()
        invariant
            i <= lemmas.len(),
            none@ == u::empty(),
            lv == lemmas_view(lemmas@),
            f == (|a: u::Bytes| (a, u::reading_of(kind as int, u::empty()))),
            rows_view(out@) == start + lv.take(i as int).map_values(f),
        decreases lemmas.len() - i,
    {
        let page = reading_of_exec(kind, &none);
        let ghost before = rows_view(out@);
        out.push((slice_to_vec(&lemmas[i]), page));
        proof {
            assert(lv.take(i + 1).map_values(f) =~= lv.take(i as int).map_values(f).push(
                f(lv[i as int]),
            ));
            assert(rows_view(out@) =~= before.push(f(lv[i as int])));
        }
        i += 1;
    }
    proof {
        assert(lv.take(i as int) =~= lv);
    }
}

pub fn reading_rows_exec(t: &EVocabulary) -> (out: Vec<(Vec<u8>, EPage)>)
    ensures
        rows_view(out@) == u::reading_rows(t@),
{
    let mut out: Vec<(Vec<u8>, EPage)> = Vec::new();
    push_rows(&mut out, &t.strengths, 0, true);
    push_rows(&mut out, &t.evidences, 1, true);
    push_lemmas(&mut out, &t.preferences, 2);
    push_lemmas(&mut out, &t.alternatives, 3);
    push_rows(&mut out, &t.qualifiers, 4, true);
    push_rows(&mut out, &t.scopes, 5, false);
    push_lemmas(&mut out, &t.exclusions, 6);
    proof {
        assert(rows_view(out@) =~= u::reading_rows(t@));
    }
    out
}

pub fn first_reading_exec(rows: &Vec<(Vec<u8>, EPage)>, l: &[u8]) -> (out: EPage)
    ensures
        out@ == u::first_reading(rows_view(rows@), l@),
{
    let ghost rv = rows_view(rows@);
    let mut i = 0usize;
    proof {
        assert(rv.skip(0) =~= rv);
    }
    while i < rows.len()
        invariant
            i <= rows.len(),
            rv == rows_view(rows@),
            u::first_reading(rv.skip(i as int), l@) == u::first_reading(rv, l@),
        decreases rows.len() - i,
    {
        proof {
            assert(rv.skip(i as int)[0] == (rows@[i as int].0@, rows@[i as int].1@));
            assert(rv.skip(i as int).drop_first() =~= rv.skip(i + 1));
        }
        if b::equal(&rows[i].0, l) {
            return h::copy(&rows[i].1);
        }
        i += 1;
    }
    proof {
        assert(rv.skip(i as int) =~= Seq::<(u::Bytes, u::Html)>::empty());
    }
    h::fixed("not stated")
}

// --- D4: Word meanings.
fn push_keys(out: &mut Vec<Vec<u8>>, rows: &Vec<(Vec<u8>, Vec<u8>)>)
    ensures
        b::views(final(out)@) == b::views(old(out)@) + voc::keys(pairs_view(rows@)),
{
    let ghost start = b::views(out@);
    let mut i = 0usize;
    while i < rows.len()
        invariant
            i <= rows.len(),
            b::views(out@) == start + voc::keys(pairs_view(rows@).take(i as int)),
        decreases rows.len() - i,
    {
        let ghost before = b::views(out@);
        out.push(slice_to_vec(&rows[i].0));
        proof {
            assert(pairs_view(rows@).take(i + 1) =~= pairs_view(rows@).take(i as int).push(
                (rows@[i as int].0@, rows@[i as int].1@),
            ));
            assert(voc::keys(pairs_view(rows@).take(i + 1)) =~= voc::keys(
                pairs_view(rows@).take(i as int),
            ).push(rows@[i as int].0@));
            assert(b::views(out@) =~= before.push(rows@[i as int].0@));
        }
        i += 1;
    }
    proof {
        assert(pairs_view(rows@).take(i as int) =~= pairs_view(rows@));
    }
}

fn push_all(out: &mut Vec<Vec<u8>>, lemmas: &Vec<Vec<u8>>)
    ensures
        b::views(final(out)@) == b::views(old(out)@) + lemmas_view(lemmas@),
{
    let ghost start = b::views(out@);
    let mut i = 0usize;
    while i < lemmas.len()
        invariant
            i <= lemmas.len(),
            b::views(out@) == start + lemmas_view(lemmas@).take(i as int),
        decreases lemmas.len() - i,
    {
        let ghost before = b::views(out@);
        out.push(slice_to_vec(&lemmas[i]));
        proof {
            assert(lemmas_view(lemmas@).take(i + 1) =~= lemmas_view(lemmas@).take(i as int).push(
                lemmas@[i as int]@,
            ));
            assert(b::views(out@) =~= before.push(lemmas@[i as int]@));
        }
        i += 1;
    }
    proof {
        assert(lemmas_view(lemmas@).take(i as int) =~= lemmas_view(lemmas@));
    }
}

pub fn plain_lemmas_exec(t: &EVocabulary) -> (out: Vec<Vec<u8>>)
    ensures
        b::views(out@) == u::plain_lemmas(t@),
{
    let mut out: Vec<Vec<u8>> = Vec::new();
    proof {
        assert(b::views(out@) =~= Seq::<u::Bytes>::empty());
    }
    push_keys(&mut out, &t.strengths);
    push_keys(&mut out, &t.evidences);
    push_all(&mut out, &t.preferences);
    push_all(&mut out, &t.alternatives);
    push_keys(&mut out, &t.qualifiers);
    proof {
        assert(b::views(out@) =~= u::plain_lemmas(t@));
    }
    out
}

pub fn verb_forms_exec(v: &[u8]) -> (out: Vec<Vec<u8>>)
    ensures
        b::views(out@) == u::verb_forms(v@),
{
    let mut out: Vec<Vec<u8>> = Vec::new();
    out.push(slice_to_vec(v));
    out.push(b::cat(slice_to_vec(v), &b::literal("s")));
    out.push(b::cat(slice_to_vec(v), &b::literal("es")));
    if v.len() > 0 && v[v.len() - 1] == 121 {
        let stem = &v[0..v.len() - 1];
        proof {
            assert(stem@ =~= v@.drop_last());
        }
        out.push(b::cat(slice_to_vec(stem), &b::literal("ies")));
    }
    proof {
        assert(b::views(out@) =~= u::verb_forms(v@));
    }
    out
}

pub fn scope_shown_exec(t: &EVocabulary, plain: &Vec<Vec<u8>>) -> (out: bool)
    requires
        b::views(plain@) == u::plain_lemmas(t@),
    ensures
        out == u::scope_shown(t@),
{
    if !(t.scopes.len() > 0 && t.exclusions.len() > 0) {
        return false;
    }
    proof {
        assert(t@.scopes[0] == (t.scopes@[0].0@, t.scopes@[0].1@));
        assert(t@.exclusions[0] == t.exclusions@[0]@);
    }
    if hl::contains(plain, &t.scopes[0].0) {
        return false;
    }
    let forms = verb_forms_exec(&t.exclusions[0]);
    let ghost fv = b::views(forms@);
    let ghost pred = |f: u::Bytes| u::plain_lemmas(t@).contains(f);
    let mut n = 0usize;
    let mut i = 0usize;
    while i < forms.len()
        invariant
            i <= forms.len(),
            n <= i,
            fv == b::views(forms@),
            b::views(plain@) == u::plain_lemmas(t@),
            pred == (|f: u::Bytes| u::plain_lemmas(t@).contains(f)),
            n == fv.take(i as int).filter(pred).len(),
        decreases forms.len() - i,
    {
        let hit = hl::contains(plain, &forms[i]);
        proof {
            assert(fv.take(i + 1) =~= fv.take(i as int).push(fv[i as int]));
            fv.take(i as int).lemma_filter_push(fv[i as int], pred);
        }
        if hit {
            n += 1;
        }
        i += 1;
    }
    proof {
        assert(fv.take(i as int) =~= fv);
    }
    n == 0
}

pub fn meanings_section_exec(g: &EGuideline) -> (out: Vec<EPage>)
    ensures
        h::pages(out@) == u::meanings_section(g@.vocabulary),
{
    hide(u::lit);
    hide(u::reading_rows);
    hide(u::scope_shown);
    hide(u::pl_word);
    let raw = match &g.vocabulary {
        None => return no_rows(),
        Some(r) => r,
    };
    let t = match parse_vocabulary_exec(raw) {
        Err(_) => return no_rows(),
        Ok((t, _)) => t,
    };
    let rows = reading_rows_exec(&t);
    if rows.len() == 0 {
        proof {
            assert(u::reading_rows(t@).len() == 0);
        }
        return no_rows();
    }
    let plain = plain_lemmas_exec(&t);
    let shown = scope_shown_exec(&t, &plain);
    let mut table: Vec<EPage> = Vec::new();
    let ghost rv = rows_view(rows@);
    let ghost f = |r: (u::Bytes, u::Html)| u::word_row(u::pl_word(raw@, r.0), r.1);
    let mut i = 0usize;
    while i < rows.len()
        invariant
            i <= rows.len(),
            rv == rows_view(rows@),
            f == (|r: (u::Bytes, u::Html)| u::word_row(u::pl_word(raw@, r.0), r.1)),
            h::pages(table@) == rv.take(i as int).map_values(f),
        decreases rows.len() - i,
    {
        let r = row2(pl_word_exec(raw, &rows[i].0), h::copy(&rows[i].1));
        let ghost before = h::pages(table@);
        table.push(r);
        proof {
            assert(rv[i as int] == (rows@[i as int].0@, rows@[i as int].1@));
            assert(rv.take(i + 1).map_values(f) =~= rv.take(i as int).map_values(f).push(
                f(rv[i as int]),
            ));
            assert(h::pages(table@) =~= before.push(f(rv[i as int])));
        }
        i += 1;
    }
    proof {
        assert(rv.take(i as int) =~= rv);
    }
    let mut out: Vec<EPage> = Vec::new();
    out.push(h::fixed("<section>"));
    out.push(h::fixed("<h2>Word meanings</h2>"));
    out.push(
        h::fixed(
            "<p>The compiler reads each word in this table as shown, in every document of this guideline. In the ACE text of a document page, select a word with a solid underline to see its reading.</p>",
        ),
    );
    if shown {
        proof {
            reveal(u::scope_shown);
            assert(t@.scopes[0] == (t.scopes@[0].0@, t.scopes@[0].1@));
            assert(t@.exclusions[0] == t.exclusions@[0]@);
        }
        out.push(
            h::cat(
                h::cat(
                    h::cat(
                        h::cat(h::fixed("<p>The words "), pl_word_exec(raw, &t.scopes[0].0)),
                        h::fixed(" and "),
                    ),
                    pl_word_exec(raw, &t.exclusions[0]),
                ),
                h::fixed(
                    " have a solid underline only in the sentences that the compiler read as scope exclusions.</p>",
                ),
            ),
        );
    }
    out.push(h::fixed("<table class=\"compact\">"));
    out.push(h::fixed("<thead><tr><th>Word</th><th>Read as</th></tr></thead>"));
    out.push(h::cat(h::cat(h::fixed("<tbody>"), h::lines(&table)), h::fixed("</tbody>")));
    out.push(h::fixed("</table>"));
    out.push(h::fixed("</section>"));
    proof {
        assert(h::pages(out@) =~= u::meanings_section(g@.vocabulary));
    }
    out
}

// --- D5: reading words.
pub open spec fn opt_bytes(o: Option<Vec<u8>>) -> Option<u::Bytes> {
    match o {
        Some(x) => Some(x@),
        None => None,
    }
}

pub fn word_lemma_exec(
    t: &EVocabulary,
    plain: &Vec<Vec<u8>>,
    forms: &Vec<Vec<u8>>,
    w: &[u8],
    exclusion: bool,
) -> (out: Option<Vec<u8>>)
    requires
        b::views(plain@) == u::plain_lemmas(t@),
        voc::scoped(t@) ==> b::views(forms@) == u::verb_forms(t@.exclusions[0]),
    ensures
        opt_bytes(out) == u::word_lemma(t@, w@, exclusion),
{
    if hl::contains(plain, w) {
        return Some(slice_to_vec(w));
    }
    if exclusion && t.scopes.len() > 0 && t.exclusions.len() > 0 {
        proof {
            assert(t@.scopes[0] == (t.scopes@[0].0@, t.scopes@[0].1@));
            assert(t@.exclusions[0] == t.exclusions@[0]@);
        }
        if b::equal(w, &t.scopes[0].0) {
            return Some(slice_to_vec(w));
        }
        if hl::contains(forms, w) {
            return Some(slice_to_vec(&t.exclusions[0]));
        }
    }
    None
}

pub fn token_char_exec(c: char) -> (out: bool)
    ensures
        out == u::token_char(c),
{
    ('0' <= c && c <= '9') || ('A' <= c && c <= 'Z') || ('a' <= c && c <= 'z')
}

pub fn char_token_end_exec(cs: &Vec<char>, start: usize) -> (end: usize)
    requires
        start <= cs.len(),
    ensures
        start <= end <= cs.len(),
        end as int == u::char_token_end(cs@, start as int),
{
    let mut i = start;
    while i < cs.len()
        invariant
            start <= i <= cs.len(),
            u::char_token_end(cs@, i as int) == u::char_token_end(cs@, start as int),
        decreases cs.len() - i,
    {
        if token_char_exec(cs[i]) {
            i += 1;
        } else if cs.len() - i > 1 && cs[i] == '-' && token_char_exec(cs[i + 1]) {
            i += 2;
        } else {
            return i;
        }
    }
    i
}

pub fn contains_u64(xs: &Vec<u64>, x: u64) -> (yes: bool)
    ensures
        yes == xs@.map_values(|n: u64| n as nat).contains(x as nat),
{
    let ghost v = xs@.map_values(|n: u64| n as nat);
    let mut i = 0usize;
    while i < xs.len()
        invariant
            i <= xs.len(),
            v == xs@.map_values(|n: u64| n as nat),
            forall|j: int| 0 <= j < i ==> #[trigger] v[j] != x as nat,
        decreases xs.len() - i,
    {
        if xs[i] == x {
            proof {
                assert(v[i as int] == x as nat);
            }
            return true;
        }
        i += 1;
    }
    false
}

pub fn reading_scan_exec(
    cs: &Vec<char>,
    codes: &Vec<u32>,
    t: &EVocabulary,
    plain: &Vec<Vec<u8>>,
    forms: &Vec<Vec<u8>>,
    ex: &Vec<u64>,
) -> (out: Vec<EReading>)
    requires
        c::scalar_list(codes@),
        c::cvs(codes@) == cs@,
        b::views(plain@) == u::plain_lemmas(t@),
        voc::scoped(t@) ==> b::views(forms@) == u::verb_forms(t@.exclusions[0]),
    ensures
        readings(out@) == u::reading_scan(cs@, 0, 0, true, t@, ex@.map_values(|n: u64| n as nat)),
{
    let ghost exv = ex@.map_values(|n: u64| n as nat);
    let mut out: Vec<EReading> = Vec::new();
    let mut i = 0usize;
    let mut line = 0u64;
    let mut fresh = true;
    while i < cs.len()
        invariant
            i <= cs.len(),
            line <= i,
            c::scalar_list(codes@),
            c::cvs(codes@) == cs@,
            codes@.len() == cs@.len(),
            b::views(plain@) == u::plain_lemmas(t@),
            voc::scoped(t@) ==> b::views(forms@) == u::verb_forms(t@.exclusions[0]),
            exv == ex@.map_values(|n: u64| n as nat),
            readings(out@) + u::reading_scan(cs@, i as int, line as nat, fresh, t@, exv)
                == u::reading_scan(cs@, 0, 0, true, t@, exv),
        decreases cs.len() - i,
    {
        let ghost before = readings(out@);
        if cs[i] == '\n' {
            fresh = true;
            i += 1;
        } else {
            let n = if fresh {
                line + 1
            } else {
                line
            };
            if token_char_exec(cs[i]) {
                let e = char_token_end_exec(cs, i + 1);
                let word = hl::slice_codes(codes, i as u64, e as u64);
                let inside = contains_u64(ex, n);
                let lemma = word_lemma_exec(t, plain, forms, &word, inside);
                proof {
                    assert(word@ == vstd::utf8::encode_utf8(cs@.subrange(i as int, e as int)));
                }
                match lemma {
                    Some(l) => {
                        out.push(EReading { start: i as u64, end: e as u64, lemma: l });
                        proof {
                            assert(readings(out@) =~= before.push(out@.last()@));
                        }
                    },
                    None => {},
                }
                i = e;
            } else {
                i += 1;
            }
            line = n;
            fresh = false;
        }
    }
    out
}

pub fn reading_words_exec(g: &EGuideline, d: &EDocument) -> (out: Vec<EReading>)
    ensures
        readings(out@) == u::reading_words(g@, d@),
{
    reveal(u::reading_words);
    let raw = match &g.vocabulary {
        None => {
            let out: Vec<EReading> = Vec::new();
            proof {
                assert(readings(out@) =~= Seq::<u::Reading>::empty());
            }
            return out;
        },
        Some(r) => r,
    };
    let t = match parse_vocabulary_exec(raw) {
        Err(_) => {
            let out: Vec<EReading> = Vec::new();
            proof {
                assert(readings(out@) =~= Seq::<u::Reading>::empty());
            }
            return out;
        },
        Ok((t, _)) => t,
    };
    let scoped = t.scopes.len() > 0 && t.exclusions.len() > 0;
    let ex: Vec<u64> = if scoped {
        proof {
            assert(t@.scopes[0] == (t.scopes@[0].0@, t.scopes@[0].1@));
        }
        crate::k5_exclusion::exclusion_lines_exec(d, &t.scopes[0].1)
    } else {
        let ex: Vec<u64> = Vec::new();
        proof {
            assert(ex@.map_values(|n: u64| n as nat) =~= Seq::<nat>::empty());
        }
        ex
    };
    let plain = plain_lemmas_exec(&t);
    let forms = if scoped {
        proof {
            assert(t@.exclusions[0] == t.exclusions@[0]@);
        }
        verb_forms_exec(&t.exclusions[0])
    } else {
        Vec::new()
    };
    let cs = crate::k5_text::chars(&d.ace);
    let codes = c::decode(&d.ace);
    reading_scan_exec(&cs, &codes, &t, &plain, &forms, &ex)
}

// --- D5 rendering: pieces split at the reading words.
pub fn clip_exec(spans: &Vec<ESpan>, a: u64, b: u64) -> (out: Vec<ESpan>)
    ensures
        hl::spans(out@) == u::clip(hl::spans(spans@), a as int, b as int),
{
    let ghost sv = hl::spans(spans@);
    let ghost keep = |p: OutSpan|
        (if p.start < a as int {
            a as int
        } else {
            p.start
        }) < (if p.end < b as int {
            p.end
        } else {
            b as int
        });
    let ghost shift = |p: OutSpan|
        OutSpan {
            start: (if p.start < a as int {
                a as int
            } else {
                p.start
            }) - a as int,
            end: (if p.end < b as int {
                p.end
            } else {
                b as int
            }) - a as int,
            index: p.index,
        };
    let mut out: Vec<ESpan> = Vec::new();
    let mut i = 0usize;
    while i < spans.len()
        invariant
            i <= spans.len(),
            sv == hl::spans(spans@),
            keep == (|p: OutSpan|
                (if p.start < a as int {
                    a as int
                } else {
                    p.start
                }) < (if p.end < b as int {
                    p.end
                } else {
                    b as int
                })),
            shift == (|p: OutSpan|
                OutSpan {
                    start: (if p.start < a as int {
                        a as int
                    } else {
                        p.start
                    }) - a as int,
                    end: (if p.end < b as int {
                        p.end
                    } else {
                        b as int
                    }) - a as int,
                    index: p.index,
                }),
            hl::spans(out@) == sv.take(i as int).filter(keep).map_values(shift),
            forall|j: int| 0 <= j < out@.len() ==> #[trigger] out@[j].end <= b,
        decreases spans.len() - i,
    {
        let p = &spans[i];
        let lo = if p.start < a {
            a
        } else {
            p.start
        };
        let hi = if p.end < b {
            p.end
        } else {
            b
        };
        let ghost before = hl::spans(out@);
        proof {
            assert(sv.take(i + 1) =~= sv.take(i as int).push(sv[i as int]));
            sv.take(i as int).lemma_filter_push(sv[i as int], keep);
        }
        if lo < hi {
            out.push(ESpan { start: lo - a, end: hi - a, index: p.index });
            proof {
                assert(hl::spans(out@) =~= before.push(shift(sv[i as int])));
                assert(sv.take(i as int).filter(keep).push(sv[i as int]).map_values(shift)
                    =~= sv.take(i as int).filter(keep).map_values(shift).push(shift(sv[i as int])));
            }
        }
        i += 1;
    }
    proof {
        assert(sv.take(i as int) =~= sv);
    }
    out
}

pub fn piece_exec(s: &[u8], codes: &Vec<u32>, spans: &Vec<ESpan>, a: u64, b: u64) -> (out: EPage)
    requires
        c::scalar_list(codes@),
        c::cvs(codes@) == u::chars(s@),
    ensures
        out@ == u::piece_html(s@, hl::spans(spans@), a as int, b as int),
{
    hide(u::marked_html);
    let sub = hl::slice_codes(codes, a, b);
    let clipped = clip_exec(spans, a, b);
    proof {
        assert(sub@ == u::slice_text(s@, a as int, b as int));
    }
    hl::marked(&sub, &clipped, true)
}

pub fn words_html_exec(s: &[u8], spans: &Vec<ESpan>, words: &Vec<EReading>) -> (out: EPage)
    ensures
        out@ == u::words_html(s@, hl::spans(spans@), readings(words@), 0),
{
    hide(u::piece_html);
    hide(u::lit);
    let codes = c::decode(s);
    let ghost sv = hl::spans(spans@);
    let ghost ws = readings(words@);
    let mut out = h::empty();
    let mut cursor = 0u64;
    let mut i = 0usize;
    proof {
        assert(ws.skip(0) =~= ws);
    }
    while i < words.len()
        invariant
            i <= words.len(),
            c::scalar_list(codes@),
            c::cvs(codes@) == u::chars(s@),
            sv == hl::spans(spans@),
            ws == readings(words@),
            out@ + u::words_html(s@, sv, ws.skip(i as int), cursor as int) == u::words_html(
                s@,
                sv,
                ws,
                0,
            ),
        decreases words.len() - i,
    {
        let w = &words[i];
        let ghost rest = ws.skip(i as int);
        proof {
            assert(rest[0] == w@);
            assert(rest.drop_first() =~= ws.skip(i + 1));
        }
        h::append(&mut out, piece_exec(s, &codes, spans, cursor, w.start));
        h::append(
            &mut out,
            h::cat(
                h::cat(
                    h::fixed(
                        "<button type=\"button\" class=\"rw\" aria-pressed=\"false\" data-w=\"",
                    ),
                    h::attr(&w.lemma),
                ),
                h::fixed("\">"),
            ),
        );
        h::append(&mut out, piece_exec(s, &codes, spans, w.start, w.end));
        h::append(&mut out, h::fixed("</button>"));
        cursor = w.end;
        i += 1;
    }
    proof {
        assert(ws.skip(i as int) =~= Seq::<u::Reading>::empty());
        assert(codes@.len() == u::chars(s@).len());
    }
    h::append(&mut out, piece_exec(s, &codes, spans, cursor, codes.len() as u64));
    out
}

pub fn ace_exec(
    g: &EGuideline,
    d: &EDocument,
    source: &[u8],
    alignment: &Option<EModel>,
    words: &Vec<EReading>,
) -> (out: EPage)
    requires
        source@ == u::payload(g@, d@.bundle.docid),
        data::alignment_view(*alignment) == u::alignment(g@, d@),
        readings(words@) == u::reading_words(g@, d@),
    ensures
        out@ == u::ace_html(g@, d@),
{
    hide(u::aligned_text);
    hide(u::words_html);
    if words.len() == 0 {
        return data::aligned(g, d, source, alignment, true);
    }
    match alignment {
        Some(m) => words_html_exec(&d.ace, &m.ace, words),
        None => {
            let none: Vec<ESpan> = Vec::new();
            proof {
                assert(hl::spans(none@) =~= Seq::<OutSpan>::empty());
            }
            words_html_exec(&d.ace, &none, words)
        },
    }
}

// --- D6: the reserved line.
pub fn reading_span_exec(
    raw: &[u8],
    rows: &Vec<(Vec<u8>, EPage)>,
    l: &[u8],
    Ghost(t): Ghost<voc::Vocabulary>,
) -> (out: EPage)
    requires
        rows_view(rows@) == u::reading_rows(t),
    ensures
        out@ == u::reading_span(raw@, t, l@),
{
    hide(u::lit);
    hide(u::pl_word);
    hide(u::first_reading);
    hide(u::reading_rows);
    let mut out = h::cat(
        h::cat(h::fixed("<span data-w=\""), h::attr(l)),
        h::fixed("\" hidden>In this guideline, the compiler reads "),
    );
    h::append(&mut out, pl_word_exec(raw, l));
    h::append(&mut out, h::fixed(" as "));
    h::append(&mut out, first_reading_exec(rows, l));
    h::append(&mut out, h::fixed("."));
    h::append(&mut out, h::fixed("</span>"));
    out
}

pub fn reading_line_exec(g: &EGuideline, d: &EDocument, words: &Vec<EReading>) -> (out: Vec<EPage>)
    requires
        readings(words@) == u::reading_words(g@, d@),
    ensures
        h::pages(out@) == u::reading_line(g@, d@),
{
    hide(u::lit);
    hide(u::reading_span);
    hide(u::reading_rows);
    if words.len() == 0 {
        return no_rows();
    }
    proof {
        reveal(u::reading_words);
    }
    let raw = match &g.vocabulary {
        None => return no_rows(),
        Some(r) => r,
    };
    let t = match parse_vocabulary_exec(raw) {
        Err(_) => return no_rows(),
        Ok((t, _)) => t,
    };
    let rows = reading_rows_exec(&t);
    let mut lemmas: Vec<Vec<u8>> = Vec::new();
    let mut i = 0usize;
    while i < words.len()
        invariant
            i <= words.len(),
            b::views(lemmas@) == readings(words@).take(i as int).map_values(
                |w: u::Reading| w.lemma,
            ),
        decreases words.len() - i,
    {
        let ghost before = b::views(lemmas@);
        lemmas.push(slice_to_vec(&words[i].lemma));
        proof {
            assert(readings(words@).take(i + 1) =~= readings(words@).take(i as int).push(
                words@[i as int]@,
            ));
            assert(b::views(lemmas@) =~= before.push(words@[i as int].lemma@));
        }
        i += 1;
    }
    proof {
        assert(readings(words@).take(i as int) =~= readings(words@));
    }
    let uniq = data::dedup(&lemmas);
    let ghost uv = b::views(uniq@);
    let ghost f = |l: u::Bytes| u::reading_span(raw@, t@, l);
    let mut spans: Vec<EPage> = Vec::new();
    let mut k = 0usize;
    while k < uniq.len()
        invariant
            k <= uniq.len(),
            uv == b::views(uniq@),
            rows_view(rows@) == u::reading_rows(t@),
            f == (|l: u::Bytes| u::reading_span(raw@, t@, l)),
            h::pages(spans@) == uv.take(k as int).map_values(f),
        decreases uniq.len() - k,
    {
        let span = reading_span_exec(raw, &rows, &uniq[k], Ghost(t@));
        let ghost before = h::pages(spans@);
        spans.push(span);
        proof {
            assert(uv.take(k + 1).map_values(f) =~= uv.take(k as int).map_values(f).push(
                f(uv[k as int]),
            ));
            assert(h::pages(spans@) =~= before.push(f(uv[k as int])));
        }
        k += 1;
    }
    proof {
        assert(uv.take(k as int) =~= uv);
    }
    let line = h::cat(
        h::cat(
            h::fixed(
                "<p class=\"reading\" aria-live=\"polite\"><span data-w=\"\">Select a word with a solid underline to see how the compiler reads it.</span>",
            ),
            h::flatten(&spans),
        ),
        h::fixed("</p>"),
    );
    let mut out: Vec<EPage> = Vec::new();
    out.push(line);
    proof {
        assert(h::pages(out@) =~= u::reading_line(g@, d@));
    }
    out
}

} // verus!
