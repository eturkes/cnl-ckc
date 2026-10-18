// S2 D7: qualifier rows share the compiled-table traversal and operator path.
use crate::k2_term::ETermArena;
#[cfg(verus_keep_ghost)]
use crate::k2_term::arena_ok;
use crate::k5_html as h;
use crate::k5_strength as s;
use crate::k5_timing as t;
use crate::m6_term::T;
#[cfg(verus_keep_ghost)]
use crate::m6_term::{models, valid, valid_all};
use ckc_spec::ui::{self as u, EPage};
use ckc_spec::v1text as v;
use vstd::prelude::*;
verus! {

pub fn kind_exec(kind: &[u8]) -> (out: EPage)
    ensures
        out@ == u::kind_html(kind@),
{
    if t::is(kind, "usuality") {
        h::fixed("How often")
    } else if t::is(kind, "ideality") {
        h::fixed("Best case")
    } else if t::is(kind, "focus") {
        h::fixed("Emphasis")
    } else if t::is(kind, "manner") {
        h::fixed("Manner")
    } else if t::is(kind, "force") {
        h::fixed("Strength of wording")
    } else if t::is(kind, "possibility") {
        h::fixed("Possibility")
    } else if t::is(kind, "applicability") {
        h::fixed("Condition of use")
    } else {
        h::fixed("not stated")
    }
}

pub fn action_exec(
    arena: &ETermArena,
    pl: &[u8],
    join: &Vec<T>,
    Ghost(d): Ghost<v::DocFile>,
    Ghost(c): Ghost<v::DocClause>,
    event: &T,
) -> (out: EPage)
    requires
        arena_ok(arena),
        valid_all(arena.nodes@, join@),
        models(join@) == u::join_terms(d, c),
        valid(arena.nodes@, event),
    ensures
        out@ == u::action_html(pl@, d, c, event@),
{
    hide(u::verb_html);
    hide(u::pl_word);
    let verb = s::verb_exec(arena, pl, join, Ghost(d), Ghost(c), event);
    let object = match s::participant_exec(arena, join, event, 2) {
        None => return verb,
        Some(x) => x,
    };
    let name = crate::k5_bytes::literal("guideline_entity");
    let args = match t::first_with_exec(
        arena,
        join,
        &name,
        Ghost("guideline_entity"@),
        4,
        1,
        &object,
    ) {
        None => return match s::bound_unit_exec(arena, join, Ghost(d), Ghost(c), &object) {
            Some(w) => h::cat(verb, w),
            None => verb,
        },
        Some(a) => a,
    };
    proof {
        assert(models(args@)[2] == args@[2]@);
        assert(valid(arena.nodes@, &args@[2]));
    }
    let noun = t::pl_word_exec(pl, &s::atom_name_exec(arena, &args[2]));
    h::cat(h::cat(verb, h::fixed(" ")), noun)
}

pub fn row_exec(
    arena: &ETermArena,
    pl: &[u8],
    join: &Vec<T>,
    Ghost(d): Ghost<v::DocFile>,
    Ghost(c): Ghost<v::DocClause>,
    ordinal: &[u8],
    Ghost(sentence): Ghost<nat>,
    literal: &T,
    part: u8,
) -> (out: Option<EPage>)
    requires
        arena_ok(arena),
        valid_all(arena.nodes@, join@),
        models(join@) == u::join_terms(d, c),
        valid(arena.nodes@, literal),
        ordinal@ == v::udec_bytes(sentence),
        part <= 2,
    ensures
        t::opt_page(out) == u::qualifier_row(pl@, d, sentence, c, literal@, part as int),
{
    hide(u::stated_html);
    hide(u::action_html);
    hide(u::kind_html);
    hide(u::pl_word);
    hide(u::part_html);
    hide(u::row);
    hide(u::cell);
    hide(u::lit);
    let (name, args) = match crate::m6_term::parts(arena, literal) {
        None => return None,
        Some(x) => x,
    };
    if !t::is(&name, "guideline_qualifier") || args.len() != 4 {
        return None;
    }
    proof {
        assert forall|i: int| 0 <= i < args@.len() implies models(args@)[i] == args@[i]@ && valid(
            arena.nodes@,
            &args@[i],
        ) by {}
    }
    let a = h::text(ordinal);
    let b = t::part_exec(part);
    let stated = s::stated_exec(arena, join, Ghost(d), Ghost(c), &args[0]);
    let action = action_exec(arena, pl, join, Ghost(d), Ghost(c), &args[1]);
    let kind = kind_exec(&s::atom_name_exec(arena, &args[2]));
    let word = t::pl_word_exec(pl, &s::atom_name_exec(arena, &args[3]));
    let ghost av = a@;
    let ghost bv = b@;
    let ghost cv = stated@;
    let ghost dv = action@;
    let ghost ev = kind@;
    let ghost fv = word@;
    let mut cells: Vec<EPage> = Vec::new();
    cells.push(h::cell(a));
    cells.push(h::cell(b));
    cells.push(h::cell(stated));
    cells.push(h::cell(action));
    cells.push(h::cell(kind));
    cells.push(h::cell(word));
    proof {
        assert(h::pages(cells@) =~= seq![
            u::cell(av),
            u::cell(bv),
            u::cell(cv),
            u::cell(dv),
            u::cell(ev),
            u::cell(fv),
        ]);
    }
    Some(h::row(&cells))
}

pub fn qualifier_rows_exec(pl: &[u8]) -> (out: Vec<EPage>)
    ensures
        h::pages(out@) == u::qualifier_rows(pl@),
{
    crate::k5_compiled::rows_exec(pl, 3)
}

pub fn qualifier_section_exec(pl: &[u8]) -> (out: Vec<EPage>)
    ensures
        h::pages(out@) == u::qualifier_section(pl@),
{
    hide(u::lit);
    let rows = qualifier_rows_exec(pl);
    if rows.len() == 0 {
        return crate::k5_compiled::no_rows();
    }
    let mut out: Vec<EPage> = Vec::new();
    out.push(h::fixed("<section>"));
    out.push(h::fixed("<h3>Qualifiers as compiled</h3>"));
    out.push(
        h::fixed(
            "<p>Each row is one qualifier word that the compiler read from the ACE text. A qualifier describes the wording of its action or condition. A qualifier on a condition must also hold in the facts that meet the condition. An excluded condition tests the condition together with its qualifier.</p>",
        ),
    );
    out.push(h::fixed("<table>"));
    out.push(
        h::fixed(
            "<thead><tr><th>Sentence</th><th>Part</th><th>Stated as</th><th>Action</th><th>Kind</th><th>Word</th></tr></thead>",
        ),
    );
    out.push(h::cat(h::cat(h::fixed("<tbody>"), h::lines(&rows)), h::fixed("</tbody>")));
    out.push(h::fixed("</table>"));
    out.push(h::fixed("</section>"));
    proof {
        assert(h::pages(out@) =~= u::qualifier_section(pl@));
    }
    out
}

} // verus!
