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

pub open spec fn relation_row(raw: u::Bytes, p: (u::Bytes, u::Bytes)) -> u::Html {
    u::word_row(u::pl_word(raw, p.0), u::role_html(p.1))
}

pub open spec fn spacing_row(raw: u::Bytes, p: (u::Bytes, u::Bytes)) -> u::Html {
    u::word_row(
        u::pl_word(raw, p.0) + u::fixed_bytes(u::paren_open_sep()) + u::fixed_bytes(u::with_sp())
            + u::pl_word(raw, p.1) + u::fixed_bytes(u::paren_close()),
        u::fixed_bytes(u::how_far_apart_repeats_of_the_action_lie()),
    )
}

pub open spec fn rows_of(raw: u::Bytes, t: Temporal) -> Seq<u::Html> {
    t.units.map_values(|p: (u::Bytes, u::Bytes)| unit_row(raw, p)) + t.relations.map_values(
        |p: (u::Bytes, u::Bytes)| relation_row(raw, p),
    ) + t.spacings.map_values(|p: (u::Bytes, u::Bytes)| spacing_row(raw, p))
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
                |p: (u::Bytes, u::Bytes)| u::word_row(u::pl_word(raw, p.0), u::role_html(p.1)),
            ) =~= m.relations.map_values(|p: (u::Bytes, u::Bytes)| relation_row(raw, p)));
            assert(m.spacings.map_values(
                |p: (u::Bytes, u::Bytes)|
                    u::word_row(
                        u::pl_word(raw, p.0) + u::fixed_bytes(u::paren_open_sep()) + u::fixed_bytes(
                            u::with_sp(),
                        ) + u::pl_word(raw, p.1) + u::fixed_bytes(u::paren_close()),
                        u::fixed_bytes(u::how_far_apart_repeats_of_the_action_lie()),
                    ),
            ) =~= m.spacings.map_values(|p: (u::Bytes, u::Bytes)| spacing_row(raw, p)));
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

fn unit_row_exec(raw: &[u8], p: &(Vec<u8>, Vec<u8>)) -> (out: EPage)
    ensures
        out@ == unit_row(raw@, (p.0@, p.1@)),
{
    let w = match unit_exec(&p.1, true) {
        Some(w) => w,
        None => h::fixed("not stated"),
    };
    row2(pl_word_exec(raw, &p.0), h::cat(h::fixed("unit of time:"), w))
}

fn relation_row_exec(raw: &[u8], p: &(Vec<u8>, Vec<u8>)) -> (out: EPage)
    ensures
        out@ == relation_row(raw@, (p.0@, p.1@)),
{
    row2(pl_word_exec(raw, &p.0), role_exec(&p.1))
}

fn spacing_row_exec(raw: &[u8], p: &(Vec<u8>, Vec<u8>)) -> (out: EPage)
    ensures
        out@ == spacing_row(raw@, (p.0@, p.1@)),
{
    let a = h::cat(
        h::cat(
            h::cat(h::cat(pl_word_exec(raw, &p.0), h::fixed(" (")), h::fixed("with ")),
            pl_word_exec(raw, &p.1),
        ),
        h::fixed(")"),
    );
    row2(a, h::fixed("how far apart repeats of the action lie"))
}

// Row kinds: 0 = unit, 1 = relation, 2 = spacing.
pub open spec fn kind_row(raw: u::Bytes, kind: u8, p: (u::Bytes, u::Bytes)) -> u::Html {
    if kind == 0 {
        unit_row(raw, p)
    } else if kind == 1 {
        relation_row(raw, p)
    } else {
        spacing_row(raw, p)
    }
}

fn kind_rows(raw: &[u8], ps: &Vec<(Vec<u8>, Vec<u8>)>, kind: u8) -> (out: Vec<EPage>)
    ensures
        h::pages(out@) == crate::m7_temporal::pairs_view(ps@).map_values(
            |p: (u::Bytes, u::Bytes)| kind_row(raw@, kind, p),
        ),
{
    let ghost f = |p: (u::Bytes, u::Bytes)| kind_row(raw@, kind, p);
    let ghost pv = crate::m7_temporal::pairs_view(ps@);
    let mut out: Vec<EPage> = Vec::new();
    let mut i = 0usize;
    proof {
        assert(h::pages(out@) =~= pv.take(0).map_values(f));
    }
    while i < ps.len()
        invariant
            i <= ps@.len(),
            f == (|p: (u::Bytes, u::Bytes)| kind_row(raw@, kind, p)),
            pv == crate::m7_temporal::pairs_view(ps@),
            pv.len() == ps@.len(),
            h::pages(out@) == pv.take(i as int).map_values(f),
        decreases ps@.len() - i,
    {
        let r = if kind == 0 {
            unit_row_exec(raw, &ps[i])
        } else if kind == 1 {
            relation_row_exec(raw, &ps[i])
        } else {
            spacing_row_exec(raw, &ps[i])
        };
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
                let mut out = kind_rows(raw, &t.units, 0);
                let mut rel = kind_rows(raw, &t.relations, 1);
                let mut spa = kind_rows(raw, &t.spacings, 2);
                let ghost a = h::pages(out@);
                let ghost b = h::pages(rel@);
                let ghost c = h::pages(spa@);
                out.append(&mut rel);
                out.append(&mut spa);
                proof {
                    let m = t@;
                    assert(h::pages(out@) =~= a + b + c);
                    assert(a =~= m.units.map_values(|p: (u::Bytes, u::Bytes)| unit_row(raw@, p)));
                    assert(b =~= m.relations.map_values(
                        |p: (u::Bytes, u::Bytes)| relation_row(raw@, p),
                    ));
                    assert(c =~= m.spacings.map_values(
                        |p: (u::Bytes, u::Bytes)| spacing_row(raw@, p),
                    ));
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
