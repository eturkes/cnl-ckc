use crate::k2_manifest::udec_vec;
use crate::k2_reject::{empty_arena, push_usize_int};
use crate::k2_term::{push_atom, push_comp, term_line, ETermArena};
#[cfg(verus_keep_ghost)]
use crate::k2_term::{arena_ok, arena_prefix_stable, child_terms, child_terms_match, root_ok};
use ckc_spec::replay::*;
use ckc_spec::term::Term;
use vstd::prelude::*;
use vstd::slice::slice_to_vec;

verus! {

pub fn atom_root(arena: &mut ETermArena, bytes: &[u8]) -> (root: usize)
    requires arena_ok(old(arena)),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        root_ok(final(arena), root),
        final(arena)@[root as int] == Term::Atom(bytes@),
{
    let root = push_atom(arena, slice_to_vec(bytes));
    root
}

pub fn int_root(arena: &mut ETermArena, n: usize) -> (root: usize)
    requires arena_ok(old(arena)),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        root_ok(final(arena), root),
        final(arena)@[root as int] == Term::Int(n as int),
{
    push_usize_int(arena, n)
}

pub fn comp_root(arena: &mut ETermArena, name: &[u8], roots: Vec<usize>) -> (root: usize)
    requires
        arena_ok(old(arena)),
        roots@.len() > 0,
        forall|i: int| 0 <= i < roots@.len() ==> #[trigger] roots@[i] < old(arena).nodes@.len(),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        root_ok(final(arena), root),
        final(arena)@[root as int] == Term::Comp(name@, child_terms(old(arena).nodes@, roots@)),
{
    push_comp(arena, slice_to_vec(name), roots)
}

pub fn comp1(arena: &mut ETermArena, name: &[u8], a: usize) -> (root: usize)
    requires root_ok(old(arena), a),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        root_ok(final(arena), root),
        final(arena)@[root as int] == Term::Comp(name@, seq![old(arena)@[a as int]]),
{
    let mut roots = Vec::new();
    roots.push(a);
    proof { child_terms_match(arena.nodes@, roots@, seq![arena@[a as int]]); }
    comp_root(arena, name, roots)
}

pub fn comp2(arena: &mut ETermArena, name: &[u8], a: usize, b: usize) -> (root: usize)
    requires root_ok(old(arena), a), root_ok(old(arena), b),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        root_ok(final(arena), root),
        final(arena)@[root as int] == Term::Comp(name@, seq![old(arena)@[a as int], old(arena)@[b as int]]),
{
    let mut roots = Vec::new();
    roots.push(a);
    roots.push(b);
    proof { child_terms_match(arena.nodes@, roots@, seq![arena@[a as int], arena@[b as int]]); }
    comp_root(arena, name, roots)
}

pub fn comp3(arena: &mut ETermArena, name: &[u8], a: usize, b: usize, c: usize) -> (root: usize)
    requires root_ok(old(arena), a), root_ok(old(arena), b), root_ok(old(arena), c),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        root_ok(final(arena), root),
        final(arena)@[root as int] == Term::Comp(name@,
            seq![old(arena)@[a as int], old(arena)@[b as int], old(arena)@[c as int]]),
{
    let mut roots = Vec::new();
    roots.push(a);
    roots.push(b);
    roots.push(c);
    proof { child_terms_match(arena.nodes@, roots@,
        seq![arena@[a as int], arena@[b as int], arena@[c as int]]); }
    comp_root(arena, name, roots)
}

pub fn error_out(arena: &mut ETermArena, detail: usize, proof_failure: bool) -> (out: EOut)
    requires root_ok(old(arena), detail),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out@ == if proof_failure { proof_fail(old(arena)@[detail as int]) }
            else { check_load(old(arena)@[detail as int]) },
{
    let ghost before = arena.nodes@;
    let class_bytes: &[u8] = if proof_failure { b"proof" } else { b"check_load" };
    let class = atom_root(arena, class_bytes);
    proof { arena_prefix_stable(before, arena); }
    let outer_bytes: &[u8] = b"ace_to_pl_error";
    let root = comp2(arena, outer_bytes, class, detail);
    let err = term_line(arena, root);
    proof {
        reveal_byteslit(b"proof");
        reveal_strlit("proof");
        reveal_byteslit(b"check_load");
        reveal_strlit("check_load");
        reveal_byteslit(b"ace_to_pl_error");
        reveal_strlit("ace_to_pl_error");
        reveal(ckc_spec::v1text::ascii);
        assert(class_bytes@ == ckc_spec::v1text::ascii(if proof_failure { "proof"@ } else { "check_load"@ }));
        assert(outer_bytes@ == ckc_spec::v1text::ascii("ace_to_pl_error"@));
        assert(arena@[root as int] == Term::Comp(ckc_spec::v1text::ascii("ace_to_pl_error"@),
            seq![atom(if proof_failure { "proof"@ } else { "check_load"@ }), before[detail as int].term@]));
    }
    EOut { rc: if proof_failure { 1 } else { 2 }, out: Vec::new(), err }
}

pub open spec fn option_out_view(out: Option<EOut>) -> Option<Out> {
    match out { Some(out) => Some(out@), None => None }
}

pub fn counts_error(name: &[u8], expected: usize, actual: usize) -> (out: EOut)
    ensures out@ == proof_fail(Term::Comp(name@, seq![Term::Int(expected as int), Term::Int(actual as int)])),
{
    let mut arena = empty_arena();
    let expected_root = int_root(&mut arena, expected);
    let actual_root = int_root(&mut arena, actual);
    let detail = comp2(&mut arena, name, expected_root, actual_root);
    error_out(&mut arena, detail, true)
}

pub fn named_atom_error(name: &[u8], value: &[u8], proof_failure: bool) -> (out: EOut)
    ensures out@ == if proof_failure {
        proof_fail(Term::Comp(name@, seq![Term::Atom(value@)]))
    } else { check_load(Term::Comp(name@, seq![Term::Atom(value@)])) },
{
    let mut arena = empty_arena();
    let value_root = atom_root(&mut arena, value);
    let detail = comp1(&mut arena, name, value_root);
    error_out(&mut arena, detail, proof_failure)
}

pub fn manifest_unreadable_out(mpath: &[u8], path: &[u8]) -> (out: EOut)
    ensures out@ == unreadable(mpath@, path@),
{
    let unreadable_bytes: &[u8] = b"unreadable";
    let manifest_bytes: &[u8] = b"aggregate_manifest";
    let module_bytes: &[u8] = b"ace_to_pl";
    let pred_bytes: &[u8] = b"aggregate_read_manifest";
    let slash_bytes: &[u8] = b"/";
    let colon_bytes: &[u8] = b":";
    let context_bytes: &[u8] = b"context";
    let error_bytes: &[u8] = b"error";
    let mut arena = empty_arena();
    let path_root = atom_root(&mut arena, path);
    let detail = comp1(&mut arena, unreadable_bytes, path_root);
    let manifest = comp1(&mut arena, manifest_bytes, detail);
    let module = atom_root(&mut arena, module_bytes);
    let pred = atom_root(&mut arena, pred_bytes);
    let arity = int_root(&mut arena, 3);
    let pi = comp2(&mut arena, slash_bytes, pred, arity);
    let goal = comp2(&mut arena, colon_bytes, module, pi);
    let arg = atom_root(&mut arena, mpath);
    let context = comp2(&mut arena, context_bytes, goal, arg);
    let error = comp2(&mut arena, error_bytes, manifest, context);
    proof {
        reveal_byteslit(b"unreadable");
        reveal_strlit("unreadable");
        reveal_byteslit(b"aggregate_manifest");
        reveal_strlit("aggregate_manifest");
        reveal_byteslit(b"ace_to_pl");
        reveal_strlit("ace_to_pl");
        reveal_byteslit(b"aggregate_read_manifest");
        reveal_strlit("aggregate_read_manifest");
        reveal_byteslit(b"/");
        reveal_strlit("/");
        reveal_byteslit(b":");
        reveal_strlit(":");
        reveal_byteslit(b"context");
        reveal_strlit("context");
        reveal_byteslit(b"error");
        reveal_strlit("error");
        reveal(ckc_spec::v1text::ascii);
        assert(unreadable_bytes@ == ckc_spec::v1text::ascii("unreadable"@));
        assert(manifest_bytes@ == ckc_spec::v1text::ascii("aggregate_manifest"@));
        assert(module_bytes@ == ckc_spec::v1text::ascii("ace_to_pl"@));
        assert(pred_bytes@ == ckc_spec::v1text::ascii("aggregate_read_manifest"@));
        assert(slash_bytes@ == ckc_spec::v1text::ascii("/"@));
        assert(colon_bytes@ == ckc_spec::v1text::ascii(":"@));
        assert(context_bytes@ == ckc_spec::v1text::ascii("context"@));
        assert(error_bytes@ == ckc_spec::v1text::ascii("error"@));
    }
    error_out(&mut arena, error, false)
}

pub fn meter_out(rows: usize, count: usize, recursion: bool) -> (out: EOut)
    ensures out@ == if recursion { ok(rec_meter(rows as nat, count as nat)) }
        else { ok(agg_meter(rows as nat, count as nat)) },
{
    let prefix: &[u8] = if recursion { b"ace_to_pl recursion ok " } else { b"ace_to_pl aggregate ok " };
    let suffix: &[u8] = if recursion { b" rule clauses\n" } else { b" obligations\n" };
    let mut out = slice_to_vec(prefix);
    let mut row_bytes = udec_vec(rows);
    out.append(&mut row_bytes);
    let mut documents = slice_to_vec(b" documents ");
    out.append(&mut documents);
    let mut count_bytes = udec_vec(count);
    out.append(&mut count_bytes);
    let mut suffix_bytes = slice_to_vec(suffix);
    out.append(&mut suffix_bytes);
    proof {
        reveal_byteslit(b"ace_to_pl recursion ok ");
        reveal_strlit("ace_to_pl recursion ok ");
        reveal_byteslit(b"ace_to_pl aggregate ok ");
        reveal_strlit("ace_to_pl aggregate ok ");
        reveal_byteslit(b" documents ");
        reveal_strlit(" documents ");
        reveal_byteslit(b" rule clauses\n");
        reveal_strlit(" rule clauses\n");
        reveal_byteslit(b" obligations\n");
        reveal_strlit(" obligations\n");
        reveal(ckc_spec::v1text::ascii);
        assert(prefix@ == ckc_spec::v1text::ascii(if recursion { "ace_to_pl recursion ok "@ } else { "ace_to_pl aggregate ok "@ }));
        assert(suffix@ == ckc_spec::v1text::ascii(if recursion { " rule clauses\n"@ } else { " obligations\n"@ }));
        assert(out@ == prefix@ + ckc_spec::v1text::udec_bytes(rows as nat)
            + ckc_spec::v1text::ascii(" documents "@) + ckc_spec::v1text::udec_bytes(count as nat) + suffix@);
    }
    EOut { rc: 0, out, err: Vec::new() }
}

} // verus!
