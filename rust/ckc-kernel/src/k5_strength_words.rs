// S1 D8: vocabulary grade + preference groups, preserving each kind's order.
use crate::k5_html as h;
use crate::k5_strength::grade_value_exec;
use crate::k5_timing::pl_word_exec;
use crate::k5_words::row2;
use crate::m7_temporal::parse_vocabulary_exec;
use ckc_spec::ui::{self as u, EGuideline, EPage};
#[cfg(verus_keep_ghost)]
use ckc_spec::vocabulary::{Vocabulary, parse_vocabulary};
use vstd::prelude::*;
verus! {

pub open spec fn grade_word(raw: u::Bytes, p: (u::Bytes, u::Bytes), strength: bool) -> u::Html {
    u::word_row(
        u::pl_word(raw, p.0),
        u::fixed_bytes(
            if strength {
                u::strength_of_the_recommendation_sep()
            } else {
                u::certainty_of_the_evidence_sep()
            },
        ) + u::grade_value_html(p.1),
    )
}

pub open spec fn preference_word(raw: u::Bytes, p: u::Bytes, preference: bool) -> u::Html {
    u::word_row(
        u::pl_word(raw, p),
        u::fixed_bytes(
            if preference {
                u::marks_the_preferred_option()
            } else {
                u::joins_the_option_that_the_preferred()
            },
        ),
    )
}

pub open spec fn rows_of(raw: u::Bytes, m: Vocabulary, grades: bool) -> Seq<u::Html> {
    if grades {
        m.strengths.map_values(|p: (u::Bytes, u::Bytes)| grade_word(raw, p, true))
            + m.evidences.map_values(|p: (u::Bytes, u::Bytes)| grade_word(raw, p, false))
    } else {
        m.preferences.map_values(|p: u::Bytes| preference_word(raw, p, true))
            + m.alternatives.map_values(|p: u::Bytes| preference_word(raw, p, false))
    }
}

pub proof fn rows_unfold(t: Option<u::Bytes>, grades: bool)
    ensures
        u::strength_word_rows(t, grades) == match t {
            Some(raw) => match parse_vocabulary(raw) {
                Ok(m) => rows_of(raw, m, grades),
                Err(_) => Seq::<u::Html>::empty(),
            },
            None => Seq::<u::Html>::empty(),
        },
{
    hide(parse_vocabulary);
    hide(u::pl_word);
    hide(u::grade_value_html);
    hide(u::word_row);
    reveal(u::strength_word_rows);
    if let Some(raw) = t {
        if let Ok(m) = parse_vocabulary(raw) {
            assert(m.strengths.map_values(|p: (u::Bytes, u::Bytes)| grade_word(raw, p, true))
                =~= m.strengths.map_values(
                |r: (u::Bytes, u::Bytes)|
                    u::word_row(
                        u::pl_word(raw, r.0),
                        u::fixed_bytes(u::strength_of_the_recommendation_sep())
                            + u::grade_value_html(r.1),
                    ),
            ));
            assert(m.evidences.map_values(|p: (u::Bytes, u::Bytes)| grade_word(raw, p, false))
                =~= m.evidences.map_values(
                |r: (u::Bytes, u::Bytes)|
                    u::word_row(
                        u::pl_word(raw, r.0),
                        u::fixed_bytes(u::certainty_of_the_evidence_sep()) + u::grade_value_html(
                            r.1,
                        ),
                    ),
            ));
            assert(m.preferences.map_values(|p: u::Bytes| preference_word(raw, p, true))
                =~= m.preferences.map_values(
                |a: u::Bytes|
                    u::word_row(
                        u::pl_word(raw, a),
                        u::fixed_bytes(u::marks_the_preferred_option()),
                    ),
            ));
            assert(m.alternatives.map_values(|p: u::Bytes| preference_word(raw, p, false))
                =~= m.alternatives.map_values(
                |a: u::Bytes|
                    u::word_row(
                        u::pl_word(raw, a),
                        u::fixed_bytes(u::joins_the_option_that_the_preferred()),
                    ),
            ));
        }
    }
}

fn grade_word_exec(raw: &[u8], p: &(Vec<u8>, Vec<u8>), strength: bool) -> (out: EPage)
    ensures
        out@ == grade_word(raw@, (p.0@, p.1@), strength),
{
    let label = h::fixed(
        if strength {
            "strength of the recommendation: "
        } else {
            "certainty of the evidence: "
        },
    );
    row2(pl_word_exec(raw, &p.0), h::cat(label, grade_value_exec(&p.1)))
}

fn grade_words(raw: &[u8], ps: &Vec<(Vec<u8>, Vec<u8>)>, strength: bool) -> (out: Vec<EPage>)
    ensures
        h::pages(out@) == crate::m7_temporal::pairs_view(ps@).map_values(
            |p: (u::Bytes, u::Bytes)| grade_word(raw@, p, strength),
        ),
{
    let ghost f = |p: (u::Bytes, u::Bytes)| grade_word(raw@, p, strength);
    let ghost pv = crate::m7_temporal::pairs_view(ps@);
    let mut out: Vec<EPage> = Vec::new();
    let mut i = 0usize;
    proof {
        assert(h::pages(out@) =~= pv.take(0).map_values(f));
    }
    while i < ps.len()
        invariant
            i <= ps@.len(),
            f == (|p: (u::Bytes, u::Bytes)| grade_word(raw@, p, strength)),
            pv == crate::m7_temporal::pairs_view(ps@),
            pv.len() == ps@.len(),
            h::pages(out@) == pv.take(i as int).map_values(f),
        decreases ps@.len() - i,
    {
        let r = grade_word_exec(raw, &ps[i], strength);
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

fn preference_word_exec(raw: &[u8], p: &[u8], preference: bool) -> (out: EPage)
    ensures
        out@ == preference_word(raw@, p@, preference),
{
    row2(
        pl_word_exec(raw, p),
        h::fixed(
            if preference {
                "marks the preferred option"
            } else {
                "joins the option that the preferred option replaces"
            },
        ),
    )
}

fn preference_words(raw: &[u8], ps: &Vec<Vec<u8>>, preference: bool) -> (out: Vec<EPage>)
    ensures
        h::pages(out@) == crate::m7_temporal::lemmas_view(ps@).map_values(
            |p: u::Bytes| preference_word(raw@, p, preference),
        ),
{
    let ghost f = |p: u::Bytes| preference_word(raw@, p, preference);
    let ghost pv = crate::m7_temporal::lemmas_view(ps@);
    let mut out: Vec<EPage> = Vec::new();
    let mut i = 0usize;
    proof {
        assert(h::pages(out@) =~= pv.take(0).map_values(f));
    }
    while i < ps.len()
        invariant
            i <= ps@.len(),
            f == (|p: u::Bytes| preference_word(raw@, p, preference)),
            pv == crate::m7_temporal::lemmas_view(ps@),
            pv.len() == ps@.len(),
            h::pages(out@) == pv.take(i as int).map_values(f),
        decreases ps@.len() - i,
    {
        let r = preference_word_exec(raw, &ps[i], preference);
        let ghost before = out@;
        out.push(r);
        proof {
            assert(pv[i as int] == ps@[i as int]@);
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

pub fn strength_word_rows_exec(g: &EGuideline, grades: bool) -> (out: Vec<EPage>)
    ensures
        h::pages(out@) == u::strength_word_rows(g@.vocabulary, grades),
{
    proof {
        rows_unfold(g@.vocabulary, grades);
    }
    let raw = match &g.vocabulary {
        None => return crate::k5_compiled::no_rows(),
        Some(r) => r,
    };
    let t = match parse_vocabulary_exec(raw) {
        Err(_) => return crate::k5_compiled::no_rows(),
        Ok((t, _)) => t,
    };
    let mut first = if grades {
        grade_words(raw, &t.strengths, true)
    } else {
        preference_words(raw, &t.preferences, true)
    };
    let mut second = if grades {
        grade_words(raw, &t.evidences, false)
    } else {
        preference_words(raw, &t.alternatives, false)
    };
    let ghost a = h::pages(first@);
    let ghost b = h::pages(second@);
    first.append(&mut second);
    proof {
        assert(h::pages(first@) =~= a + b);
        assert(h::pages(first@) =~= rows_of(raw@, t@, grades));
    }
    first
}

pub fn strength_words_section_exec(g: &EGuideline, grades: bool) -> (out: Vec<EPage>)
    ensures
        h::pages(out@) == u::strength_words_section(g@.vocabulary, grades),
{
    hide(u::lit);
    let rows = strength_word_rows_exec(g, grades);
    if rows.len() == 0 {
        return crate::k5_compiled::no_rows();
    }
    let mut out: Vec<EPage> = Vec::new();
    out.push(h::fixed("<section>"));
    out.push(
        h::fixed(
            if grades {
                "<h2>Grade words</h2>"
            } else {
                "<h2>Preference words</h2>"
            },
        ),
    );
    out.push(
        h::fixed(
            if grades {
                "<p>The compiler reads the grade of a recommendation only through the words in this table. The table applies to every document in this guideline. Each grade that the compiler read appears on its document page under Grade as compiled.</p>"
            } else {
                "<p>The compiler reads a preference only through the words in this table. The table applies to every document in this guideline. Each preference that the compiler read appears on its document page under Preferences as compiled.</p>"
            },
        ),
    );
    out.push(h::fixed("<table class=\"compact\">"));
    out.push(h::fixed("<thead><tr><th>Word</th><th>Read as</th></tr></thead>"));
    out.push(h::cat(h::cat(h::fixed("<tbody>"), h::lines(&rows)), h::fixed("</tbody>")));
    out.push(h::fixed("</table>"));
    out.push(h::fixed("</section>"));
    proof {
        assert(h::pages(out@) =~= u::strength_words_section(g@.vocabulary, grades));
    }
    out
}

} // verus!
