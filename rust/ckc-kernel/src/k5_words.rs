// K5 render of the q14 time-word section (`u::words_section`): the guideline's
// temporal.tsv rows in plain language, placed after the Status section.
use crate::k5_html as h;
use crate::k5_timing::{is, pl_word_exec, unit_exec};
use crate::m7_temporal::parse_temporal_exec;
#[cfg(verus_keep_ghost)]
use ckc_spec::temporal::{Temporal, parse_temporal};
use ckc_spec::ui::{self as u, EGuideline, EPage};
use vstd::prelude::*;
verus! {

pub open spec fn unit_row(raw: u::Bytes, p: (u::Bytes, u::Bytes)) -> u::Html {
    u::word_row(
        u::pl_word(raw, p.0),
        u::fixed_bytes(u::unit_of_time()) + match u::unit_html(p.1, true) {
            Option::Some(w) => w,
            Option::None => u::fixed_bytes(u::not_stated()),
        },
    )
}

pub open spec fn relation_row(raw: u::Bytes, v: nat, p: (u::Bytes, u::Bytes)) -> u::Html {
    u::word_row(u::pl_word(raw, p.0), u::role_html_v(p.1, v))
}

pub open spec fn spacing_row(raw: u::Bytes, p: (u::Bytes, u::Bytes)) -> u::Html {
    u::word_row(
        u::framed_word(raw, p.0, p.1),
        u::fixed_bytes(u::how_far_apart_repeats_of_the_action_lie()),
    )
}

pub open spec fn window_row(raw: u::Bytes, p: (u::Bytes, u::Bytes)) -> u::Html {
    u::word_row(
        u::framed_word(raw, p.0, p.1),
        u::fixed_bytes(u::the_time_window_in_which_the_spacing()),
    )
}

pub open spec fn frequency_row(raw: u::Bytes, f: u::Bytes) -> u::Html {
    u::word_row(u::pl_word(raw, f), u::fixed_bytes(u::how_many_of_an_item_the_action()))
}

pub open spec fn approximation_row(raw: u::Bytes, a: u::Bytes) -> u::Html {
    u::word_row(u::pl_word(raw, a), u::fixed_bytes(u::marks_a_time_limit_as_approximate()))
}

pub open spec fn range_row(raw: u::Bytes, r: u::Bytes) -> u::Html {
    u::word_row(u::pl_word(raw, r), u::fixed_bytes(u::joins_the_upper_end_of_a_minimum()))
}

// Lemma row kinds: 0 = frequency, 1 = approximation, 2 = range.
pub open spec fn lemma_row(raw: u::Bytes, kind: u8, f: u::Bytes) -> u::Html {
    if kind == 0 {
        frequency_row(raw, f)
    } else if kind == 1 {
        approximation_row(raw, f)
    } else {
        range_row(raw, f)
    }
}

pub open spec fn rows_of(raw: u::Bytes, t: Temporal) -> Seq<u::Html> {
    t.units.map_values(|p: (u::Bytes, u::Bytes)| unit_row(raw, p)) + t.relations.map_values(
        |p: (u::Bytes, u::Bytes)| relation_row(raw, t.version, p),
    ) + t.spacings.map_values(|p: (u::Bytes, u::Bytes)| spacing_row(raw, p)) + t.windows.map_values(
        |p: (u::Bytes, u::Bytes)| window_row(raw, p),
    ) + t.frequencies.map_values(|f: u::Bytes| frequency_row(raw, f)) + t.approximations.map_values(
        |a: u::Bytes| approximation_row(raw, a),
    ) + t.ranges.map_values(|r: u::Bytes| range_row(raw, r))
}

pub proof fn rows_unfold(t: Option<u::Bytes>)
    ensures
        u::word_rows(t) == match t {
            Option::Some(raw) => match parse_temporal(raw) {
                Result::Ok(m) => rows_of(raw, m),
                Result::Err(_) => Seq::<u::Html>::empty(),
            },
            Option::None => Seq::<u::Html>::empty(),
        },
{
    hide(parse_temporal);
    hide(u::pl_word);
    hide(u::unit_html);
    hide(u::role_html_v);
    hide(u::framed_word);
    hide(u::word_row);
    reveal(u::word_rows);
    if let Option::Some(raw) = t {
        if let Result::Ok(m) = parse_temporal(raw) {
            assert(m.units.map_values(
                |p: (u::Bytes, u::Bytes)|
                    u::word_row(
                        u::pl_word(raw, p.0),
                        u::fixed_bytes(u::unit_of_time()) + match u::unit_html(p.1, true) {
                            Option::Some(w) => w,
                            Option::None => u::fixed_bytes(u::not_stated()),
                        },
                    ),
            ) =~= m.units.map_values(|p: (u::Bytes, u::Bytes)| unit_row(raw, p)));
            assert(m.relations.map_values(
                |r: (u::Bytes, u::Bytes)|
                    u::word_row(u::pl_word(raw, r.0), u::role_html_v(r.1, m.version)),
            ) =~= m.relations.map_values(
                |p: (u::Bytes, u::Bytes)| relation_row(raw, m.version, p),
            ));
            assert(m.spacings.map_values(
                |s: (u::Bytes, u::Bytes)|
                    u::word_row(
                        u::framed_word(raw, s.0, s.1),
                        u::fixed_bytes(u::how_far_apart_repeats_of_the_action_lie()),
                    ),
            ) =~= m.spacings.map_values(|p: (u::Bytes, u::Bytes)| spacing_row(raw, p)));
            assert(m.windows.map_values(
                |w: (u::Bytes, u::Bytes)|
                    u::word_row(
                        u::framed_word(raw, w.0, w.1),
                        u::fixed_bytes(u::the_time_window_in_which_the_spacing()),
                    ),
            ) =~= m.windows.map_values(|p: (u::Bytes, u::Bytes)| window_row(raw, p)));
            assert(m.frequencies.map_values(
                |f: u::Bytes|
                    u::word_row(
                        u::pl_word(raw, f),
                        u::fixed_bytes(u::how_many_of_an_item_the_action()),
                    ),
            ) =~= m.frequencies.map_values(|f: u::Bytes| frequency_row(raw, f)));
            assert(m.approximations.map_values(
                |a: u::Bytes|
                    u::word_row(
                        u::pl_word(raw, a),
                        u::fixed_bytes(u::marks_a_time_limit_as_approximate()),
                    ),
            ) =~= m.approximations.map_values(|a: u::Bytes| approximation_row(raw, a)));
            assert(m.ranges.map_values(
                |r: u::Bytes|
                    u::word_row(
                        u::pl_word(raw, r),
                        u::fixed_bytes(u::joins_the_upper_end_of_a_minimum()),
                    ),
            ) =~= m.ranges.map_values(|r: u::Bytes| range_row(raw, r)));
        }
    }
}

pub fn role_exec(r: &[u8]) -> (out: EPage)
    ensures
        out@ == u::role_html(r@),
{
    if is(r, "duration") {
        h::fixed("how long the action lasts")
    } else if is(r, "within") {
        h::fixed("how far the action lies from a reference point, before or after it")
    } else if is(r, "after") {
        h::fixed("how long after a reference point the action occurs")
    } else if is(r, "before") {
        h::fixed("how long before a reference point the action occurs")
    } else {
        h::fixed("not stated")
    }
}

pub fn role_v_exec(r: &[u8], v: u8) -> (out: EPage)
    ensures
        out@ == u::role_html_v(r@, v as nat),
{
    if v == 3 && is(r, "after") {
        h::fixed(
            "how long after a reference point the action occurs, or only that it occurs after it",
        )
    } else if v == 3 && is(r, "before") {
        h::fixed(
            "how long before a reference point the action occurs, or only that it occurs before it",
        )
    } else {
        role_exec(r)
    }
}

fn row2(a: EPage, b: EPage) -> (out: EPage)
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

fn framed_exec(raw: &[u8], p: &[u8], f: &[u8]) -> (out: EPage)
    ensures
        out@ == u::framed_word(raw@, p@, f@),
{
    h::cat(
        h::cat(
            h::cat(h::cat(pl_word_exec(raw, p), h::fixed(" (")), h::fixed("with ")),
            pl_word_exec(raw, f),
        ),
        h::fixed(")"),
    )
}

// Row kinds: 0 = unit, 1 = relation, 2 = spacing, 3 = window.
pub open spec fn kind_row(raw: u::Bytes, v: nat, kind: u8, p: (u::Bytes, u::Bytes)) -> u::Html {
    if kind == 0 {
        unit_row(raw, p)
    } else if kind == 1 {
        relation_row(raw, v, p)
    } else if kind == 2 {
        spacing_row(raw, p)
    } else {
        window_row(raw, p)
    }
}

fn kind_row_exec(raw: &[u8], v: u8, kind: u8, p: &(Vec<u8>, Vec<u8>)) -> (out: EPage)
    ensures
        out@ == kind_row(raw@, v as nat, kind, (p.0@, p.1@)),
{
    if kind == 0 {
        let w = match unit_exec(&p.1, true) {
            Some(w) => w,
            None => h::fixed("not stated"),
        };
        row2(pl_word_exec(raw, &p.0), h::cat(h::fixed("unit of time:"), w))
    } else if kind == 1 {
        row2(pl_word_exec(raw, &p.0), role_v_exec(&p.1, v))
    } else if kind == 2 {
        row2(framed_exec(raw, &p.0, &p.1), h::fixed("how far apart repeats of the action lie"))
    } else {
        row2(
            framed_exec(raw, &p.0, &p.1),
            h::fixed("the time window in which the spacing of repeats holds"),
        )
    }
}

fn kind_rows(raw: &[u8], v: u8, ps: &Vec<(Vec<u8>, Vec<u8>)>, kind: u8) -> (out: Vec<EPage>)
    ensures
        h::pages(out@) == crate::m7_temporal::pairs_view(ps@).map_values(
            |p: (u::Bytes, u::Bytes)| kind_row(raw@, v as nat, kind, p),
        ),
{
    let ghost f = |p: (u::Bytes, u::Bytes)| kind_row(raw@, v as nat, kind, p);
    let ghost pv = crate::m7_temporal::pairs_view(ps@);
    let mut out: Vec<EPage> = Vec::new();
    let mut i = 0usize;
    proof {
        assert(h::pages(out@) =~= pv.take(0).map_values(f));
    }
    while i < ps.len()
        invariant
            i <= ps@.len(),
            f == (|p: (u::Bytes, u::Bytes)| kind_row(raw@, v as nat, kind, p)),
            pv == crate::m7_temporal::pairs_view(ps@),
            pv.len() == ps@.len(),
            h::pages(out@) == pv.take(i as int).map_values(f),
        decreases ps@.len() - i,
    {
        let r = kind_row_exec(raw, v, kind, &ps[i]);
        let ghost before = out@;
        out.push(r);
        proof {
            assert(pv[i as int] == (ps@[i as int].0@, ps@[i as int].1@));
            assert(r@ == f(pv[i as int]));
            assert(pv.take(i + 1).map_values(f) =~= pv.take(i as int).map_values(f).push(
                f(pv[i as int]),
            ));
            assert(h::pages(out@) =~= h::pages(before).push(r@));
        }
        i += 1;
    }
    proof {
        assert(pv.take(ps@.len() as int) =~= pv);
    }
    out
}

fn lemma_row_exec(raw: &[u8], kind: u8, f: &[u8]) -> (out: EPage)
    ensures
        out@ == lemma_row(raw@, kind, f@),
{
    if kind == 0 {
        row2(
            pl_word_exec(raw, f),
            h::fixed("how many of an item the action involves in each period"),
        )
    } else if kind == 1 {
        row2(pl_word_exec(raw, f), h::fixed("marks a time limit as approximate"))
    } else {
        row2(
            pl_word_exec(raw, f),
            h::fixed("joins the upper end of a minimum that is stated as a range"),
        )
    }
}

fn lemma_rows(raw: &[u8], fs: &Vec<Vec<u8>>, kind: u8) -> (out: Vec<EPage>)
    ensures
        h::pages(out@) == crate::m7_temporal::lemmas_view(fs@).map_values(
            |f: u::Bytes| lemma_row(raw@, kind, f),
        ),
{
    let ghost g = |f: u::Bytes| lemma_row(raw@, kind, f);
    let ghost lv = crate::m7_temporal::lemmas_view(fs@);
    let mut out: Vec<EPage> = Vec::new();
    let mut i = 0usize;
    proof {
        assert(h::pages(out@) =~= lv.take(0).map_values(g));
    }
    while i < fs.len()
        invariant
            i <= fs@.len(),
            g == (|f: u::Bytes| lemma_row(raw@, kind, f)),
            lv == crate::m7_temporal::lemmas_view(fs@),
            lv.len() == fs@.len(),
            h::pages(out@) == lv.take(i as int).map_values(g),
        decreases fs@.len() - i,
    {
        let r = lemma_row_exec(raw, kind, &fs[i]);
        let ghost before = out@;
        out.push(r);
        proof {
            assert(lv[i as int] == fs@[i as int]@);
            assert(r@ == g(lv[i as int]));
            assert(lv.take(i + 1).map_values(g) =~= lv.take(i as int).map_values(g).push(
                g(lv[i as int]),
            ));
            assert(h::pages(out@) =~= h::pages(before).push(r@));
        }
        i += 1;
    }
    proof {
        assert(lv.take(fs@.len() as int) =~= lv);
    }
    out
}

pub fn word_rows_exec(g: &EGuideline) -> (out: Vec<EPage>)
    ensures
        h::pages(out@) == u::word_rows(g@.temporal),
{
    proof {
        rows_unfold(g@.temporal);
    }
    match &g.temporal {
        None => {
            let out: Vec<EPage> = Vec::new();
            proof {
                assert(h::pages(out@) =~= Seq::<u::Html>::empty());
            }
            out
        },
        Some(raw) => match parse_temporal_exec(raw) {
            Err(_) => {
                let out: Vec<EPage> = Vec::new();
                proof {
                    assert(h::pages(out@) =~= Seq::<u::Html>::empty());
                }
                out
            },
            Ok((t, _)) => {
                let v = t.version;
                let mut out = kind_rows(raw, v, &t.units, 0);
                let mut rel = kind_rows(raw, v, &t.relations, 1);
                let mut spa = kind_rows(raw, v, &t.spacings, 2);
                let mut win = kind_rows(raw, v, &t.windows, 3);
                let mut fre = lemma_rows(raw, &t.frequencies, 0);
                let mut apx = lemma_rows(raw, &t.approximations, 1);
                let mut rng = lemma_rows(raw, &t.ranges, 2);
                let ghost a = h::pages(out@);
                let ghost b = h::pages(rel@);
                let ghost c = h::pages(spa@);
                let ghost d = h::pages(win@);
                let ghost e = h::pages(fre@);
                let ghost x = h::pages(apx@);
                let ghost y = h::pages(rng@);
                out.append(&mut rel);
                out.append(&mut spa);
                out.append(&mut win);
                out.append(&mut fre);
                out.append(&mut apx);
                out.append(&mut rng);
                proof {
                    let m = t@;
                    assert(h::pages(out@) =~= a + b + c + d + e + x + y);
                    assert(a =~= m.units.map_values(|p: (u::Bytes, u::Bytes)| unit_row(raw@, p)));
                    assert(b =~= m.relations.map_values(
                        |p: (u::Bytes, u::Bytes)| relation_row(raw@, m.version, p),
                    ));
                    assert(c =~= m.spacings.map_values(
                        |p: (u::Bytes, u::Bytes)| spacing_row(raw@, p),
                    ));
                    assert(d =~= m.windows.map_values(
                        |p: (u::Bytes, u::Bytes)| window_row(raw@, p),
                    ));
                    assert(e =~= m.frequencies.map_values(|f: u::Bytes| frequency_row(raw@, f)));
                    assert(x =~= m.approximations.map_values(
                        |f: u::Bytes| approximation_row(raw@, f),
                    ));
                    assert(y =~= m.ranges.map_values(|f: u::Bytes| range_row(raw@, f)));
                    assert(h::pages(out@) =~= rows_of(raw@, m));
                }
                out
            },
        },
    }
}

pub fn words_section_exec(g: &EGuideline) -> (out: Vec<EPage>)
    ensures
        h::pages(out@) == u::words_section(g@.temporal),
{
    hide(u::lit);
    let rows = word_rows_exec(g);
    if rows.len() == 0 {
        let out: Vec<EPage> = Vec::new();
        proof {
            assert(h::pages(out@) =~= u::words_section(g@.temporal));
        }
        return out;
    }
    let mut out: Vec<EPage> = Vec::new();
    out.push(h::fixed("<section>"));
    out.push(h::fixed("<h2>Time words</h2>"));
    out.push(
        h::fixed(
            "<p>The compiler reads a time limit from the ACE text only through the words in this table. The table applies to every document in this guideline. Each time limit that the compiler read appears on its document page under Timing as compiled.</p>",
        ),
    );
    out.push(h::fixed("<table class=\"compact\">"));
    out.push(h::fixed("<thead><tr><th>Word</th><th>Read as</th></tr></thead>"));
    out.push(h::cat(h::cat(h::fixed("<tbody>"), h::lines(&rows)), h::fixed("</tbody>")));
    out.push(h::fixed("</table>"));
    out.push(h::fixed("</section>"));
    proof {
        assert(h::pages(out@) =~= u::words_section(g@.temporal));
    }
    out
}

} // verus!
