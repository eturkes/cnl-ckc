// S2 D7: vocabulary qualifiers in file order, without empty sections.
use crate::k5_html as h;
use crate::k5_qualifier::kind_exec;
use crate::k5_timing::pl_word_exec;
use crate::k5_words::row2;
use crate::m7_temporal::parse_vocabulary_exec;
use ckc_spec::ui::{self as u, EGuideline, EPage};
#[cfg(verus_keep_ghost)]
use ckc_spec::vocabulary::parse_vocabulary;
use vstd::prelude::*;
verus! {

pub open spec fn word(raw: u::Bytes, p: (u::Bytes, u::Bytes)) -> u::Html {
    u::word_row(u::pl_word(raw, p.0), u::kind_html(p.1))
}

pub proof fn rows_unfold(table: Option<u::Bytes>)
    ensures
        u::qualifier_word_rows(table) == match table {
            Some(raw) => match parse_vocabulary(raw) {
                Ok(m) => m.qualifiers.map_values(|p: (u::Bytes, u::Bytes)| word(raw, p)),
                Err(_) => Seq::<u::Html>::empty(),
            },
            None => Seq::<u::Html>::empty(),
        },
{
    hide(parse_vocabulary);
    hide(u::pl_word);
    hide(u::kind_html);
    hide(u::word_row);
    reveal(u::qualifier_word_rows);
    if let Some(raw) = table {
        if let Ok(m) = parse_vocabulary(raw) {
            assert(m.qualifiers.map_values(|p: (u::Bytes, u::Bytes)| word(raw, p))
                =~= m.qualifiers.map_values(
                |p: (u::Bytes, u::Bytes)| u::word_row(u::pl_word(raw, p.0), u::kind_html(p.1)),
            ));
        }
    }
}

fn words(raw: &[u8], pairs: &Vec<(Vec<u8>, Vec<u8>)>) -> (out: Vec<EPage>)
    ensures
        h::pages(out@) == crate::m7_temporal::pairs_view(pairs@).map_values(
            |p: (u::Bytes, u::Bytes)| word(raw@, p),
        ),
{
    let ghost f = |p: (u::Bytes, u::Bytes)| word(raw@, p);
    let ghost pv = crate::m7_temporal::pairs_view(pairs@);
    let mut out: Vec<EPage> = Vec::new();
    let mut i = 0usize;
    proof {
        assert(h::pages(out@) =~= pv.take(0).map_values(f));
    }
    while i < pairs.len()
        invariant
            i <= pairs@.len(),
            f == (|p: (u::Bytes, u::Bytes)| word(raw@, p)),
            pv == crate::m7_temporal::pairs_view(pairs@),
            pv.len() == pairs@.len(),
            h::pages(out@) == pv.take(i as int).map_values(f),
        decreases pairs@.len() - i,
    {
        let r = row2(pl_word_exec(raw, &pairs[i].0), kind_exec(&pairs[i].1));
        let ghost before = out@;
        out.push(r);
        proof {
            assert(pv[i as int] == (pairs@[i as int].0@, pairs@[i as int].1@));
            assert(r@ == f(pv[i as int]));
            assert(pv.take(i + 1).map_values(f) =~= pv.take(i as int).map_values(f).push(
                f(pv[i as int]),
            ));
            assert(h::pages(out@) =~= h::pages(before).push(r@));
        }
        i += 1;
    }
    proof {
        assert(pv.take(pairs@.len() as int) =~= pv);
    }
    out
}

pub fn qualifier_word_rows_exec(g: &EGuideline) -> (out: Vec<EPage>)
    ensures
        h::pages(out@) == u::qualifier_word_rows(g@.vocabulary),
{
    proof {
        rows_unfold(g@.vocabulary);
    }
    let raw = match &g.vocabulary {
        None => return crate::k5_compiled::no_rows(),
        Some(r) => r,
    };
    let table = match parse_vocabulary_exec(raw) {
        Err(_) => return crate::k5_compiled::no_rows(),
        Ok((t, _)) => t,
    };
    words(raw, &table.qualifiers)
}

pub fn qualifier_words_section_exec(g: &EGuideline) -> (out: Vec<EPage>)
    ensures
        h::pages(out@) == u::qualifier_words_section(g@.vocabulary),
{
    hide(u::lit);
    let rows = qualifier_word_rows_exec(g);
    if rows.len() == 0 {
        return crate::k5_compiled::no_rows();
    }
    let mut out: Vec<EPage> = Vec::new();
    out.push(h::fixed("<section>"));
    out.push(h::fixed("<h2>Qualifier words</h2>"));
    out.push(
        h::fixed(
            "<p>The compiler reads a qualifier only through the words in this table. The table applies to every document in this guideline. Each qualifier that the compiler read appears on its document page under Qualifiers as compiled.</p>",
        ),
    );
    out.push(h::fixed("<table class=\"compact\">"));
    out.push(h::fixed("<thead><tr><th>Word</th><th>Read as</th></tr></thead>"));
    out.push(h::cat(h::cat(h::fixed("<tbody>"), h::lines(&rows)), h::fixed("</tbody>")));
    out.push(h::fixed("</table>"));
    out.push(h::fixed("</section>"));
    proof {
        assert(h::pages(out@) =~= u::qualifier_words_section(g@.vocabulary));
    }
    out
}

} // verus!
