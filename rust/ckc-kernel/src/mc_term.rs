use crate::k2_term::{ENodeKind, ETermArena};
use crate::v1_term_impl::GTermExpected;
use ckc_spec::term::Term;
use vstd::prelude::*;

verus! {

// Contract mc D1: a fact, model line, claimed fact or valuation value = one ground term's
// canonical v1 text. The v1 term parser is canonical by construction (`spanned_term_ok`: the
// span's bytes = `term_bytes` of the parse), so a parse spanning the whole text decodes it.
proof fn lemma_no_dollar_all(ts: Seq<Term>)
    ensures
        ckc_spec::term::no_dollar_var_all(ts) <==> forall|k: int|
            0 <= k < ts.len() ==> #[trigger] ckc_spec::term::no_dollar_var(ts[k]),
    decreases ts.len(),
{
    if ts.len() > 0 {
        let rest = ts.drop_first();
        lemma_no_dollar_all(rest);
        assert(ckc_spec::term::no_dollar_var_all(ts) == (ckc_spec::term::no_dollar_var(ts[0])
            && ckc_spec::term::no_dollar_var_all(rest)));
        if ckc_spec::term::no_dollar_var_all(ts) {
            assert forall|k: int|
                0 <= k < ts.len() implies #[trigger] ckc_spec::term::no_dollar_var(ts[k]) by {
                if k > 0 {
                    assert(rest[k - 1] == ts[k]);
                    assert(ckc_spec::term::no_dollar_var(rest[k - 1]));
                }
            }
        } else if ckc_spec::term::no_dollar_var(ts[0]) {
            let k = choose|k: int|
                0 <= k < rest.len() && !#[trigger] ckc_spec::term::no_dollar_var(rest[k]);
            assert(rest[k] == ts[k + 1]);
            assert(!ckc_spec::term::no_dollar_var(ts[k + 1]));
        }
    }
}

// No `'$VAR'/1` subterm.
pub fn no_dollar_root(arena: &ETermArena, root: usize) -> (out: bool)
    requires
        crate::k2_term::root_ok(arena, root),
    ensures
        out == ckc_spec::term::no_dollar_var(arena@[root as int]),
    decreases root,
{
    proof {
        assert(crate::k2_term::node_ok(arena.nodes@, root as int));
    }
    match &arena.nodes[root].kind {
        ENodeKind::Comp { name, child_roots, .. } => {
            let ghost args = crate::k2_term::child_terms(arena.nodes@, child_roots@);
            proof {
                assert(arena@[root as int] == Term::Comp(name@, args));
            }
            if child_roots.len() == 1 && name.len() == 4 && name[0] == 0x24 && name[1] == 0x56
                && name[2] == 0x41 && name[3] == 0x52 {
                proof {
                    assert(name@ =~= ckc_spec::term::dollar_var_name());
                }
                return false;
            }
            proof {
                if name@ == ckc_spec::term::dollar_var_name() && args.len() == 1 {
                    assert(name@.len() == 4);
                    assert(name@[0] == 0x24u8 && name@[1] == 0x56u8 && name@[2] == 0x41u8
                        && name@[3] == 0x52u8);
                }
            }
            let mut i = 0usize;
            while i < child_roots.len()
                invariant
                    crate::k2_term::root_ok(arena, root),
                    crate::k2_term::child_roots_before(child_roots@, root as int),
                    crate::k2_term::child_roots_valid(arena.nodes@, child_roots@),
                    args == crate::k2_term::child_terms(arena.nodes@, child_roots@),
                    arena@[root as int] == Term::Comp(name@, args),
                    !(name@ == ckc_spec::term::dollar_var_name() && args.len() == 1),
                    i <= child_roots@.len(),
                    forall|k: int| 0 <= k < i ==> #[trigger] ckc_spec::term::no_dollar_var(args[k]),
                decreases child_roots.len() - i,
            {
                let c = child_roots[i];
                proof {
                    assert(args[i as int] == arena@[c as int]);
                }
                if !no_dollar_root(arena, c) {
                    proof {
                        lemma_no_dollar_all(args);
                    }
                    return false;
                }
                i += 1;
            }
            proof {
                lemma_no_dollar_all(args);
            }
            true
        },
        _ => true,
    }
}

pub fn decode_term(arena: &mut ETermArena, text: &[u8]) -> (r: Option<usize>)
    requires
        crate::k2_term::arena_ok(old(arena)),
    ensures
        crate::k2_term::arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        r matches Some(root) ==> root < final(arena).nodes@.len() && ckc_spec::model::term_decodes(
            text@,
            final(arena)@[root as int],
        ),
{
    if text.len() == 0 {
        return None;
    }
    let mut tracker = crate::v1_term_impl::new_var_tracker();
    let mut at = 0usize;
    let parsed = crate::v1_term_impl::parse_term(
        text,
        0,
        arena,
        Ghost(None),
        Ghost(GTermExpected { term: Term::Nil, end: 0 }),
        Ghost(Seq::empty()),
        false,
        &mut tracker,
        &mut at,
    );
    let t = match parsed {
        Some(t) => t,
        None => return None,
    };
    if t.end != text.len() {
        return None;
    }
    if !crate::k2_walk::ground_root(arena, t.root) {
        return None;
    }
    if !no_dollar_root(arena, t.root) {
        return None;
    }
    proof {
        assert(crate::k2_term::node_ok(arena.nodes@, t.root as int));
        assert(text@.subrange(0, text@.len() as int) =~= text@);
    }
    Some(t.root)
}

} // verus!
