use crate::k2_output::{atom_root, comp_root, comp1, error_out};
use crate::k2_term::{ETermArena, push_atom, push_int};
#[cfg(verus_keep_ghost)]
use crate::k2_term::{arena_ok, root_ok};
use crate::k3_coords::ECoord;
#[cfg(verus_keep_ghost)]
use crate::k3_coords::{coord_ok, coords_ok, coords_view};
use ckc_spec::replay::EOut;
#[cfg(verus_keep_ghost)]
use ckc_spec::term::Term;
use ckc_spec::trace::*;
use vstd::prelude::*;

verus! {

// The clause's own `% S<n>:` block as the node's `sentence(DocId, S)`.
pub fn sentence_exec(arena: &mut ETermArena, coords: &Vec<ECoord>, m: usize) -> (out: Result<
    usize,
    EOut,
>)
    requires
        arena_ok(old(arena)),
        coords_ok(coords@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out matches Ok(root) ==> root_ok(final(arena), root),
        crate::k2_answers::result_view(final(arena).nodes@, out) == sentence_of(
            coords_view(coords@),
            m as nat,
        ),
{
    if m < coords.len() {
        let c = &coords[m];
        proof {
            assert(coord_ok(&coords@[m as int]));
        }
        let ghost before = arena.nodes@;
        let d = push_atom(arena, c.docid.clone());
        let s = push_int(
            arena,
            c.ordinal.clone(),
            c.ordinal.clone(),
            false,
            Ghost(c.model@.s as int),
        );
        let mut roots = Vec::new();
        roots.push(d);
        roots.push(s);
        let name: &[u8] = b"sentence";
        proof {
            reveal_byteslit(b"sentence");
            reveal_strlit("sentence");
            reveal(ckc_spec::v1text::ascii);
            assert(name@ == ckc_spec::v1text::ascii("sentence"@));
        }
        let ghost mid = arena.nodes@;
        let root = comp_root(arena, name, roots);
        proof {
            assert(crate::k2_term::child_terms(mid, roots@) =~= seq![
                Term::Atom(c.model@.docid),
                Term::Int(c.model@.s as int),
            ]);
        }
        return Ok(root);
    }
    let name: &[u8] = b"clause_identity";
    let none: &[u8] = b"none";
    proof {
        reveal_byteslit(b"clause_identity");
        reveal_strlit("clause_identity");
        reveal_byteslit(b"none");
        reveal_strlit("none");
        reveal(ckc_spec::v1text::ascii);
        assert(name@ == ckc_spec::v1text::ascii("clause_identity"@));
        assert(none@ == ckc_spec::v1text::ascii("none"@));
    }
    let detail = atom_root(arena, none);
    let why = comp1(arena, name, detail);
    Err(error_out(arena, why, true))
}

} // verus!
