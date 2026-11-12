use crate::k2_term::{ENodeKind, ETermArena};
use ckc_spec::term::Term;
use vstd::prelude::*;

verus! {

// Contract mc D2 G1: the compiler's term shapes, decided over arena terms.
pub fn name_is(name: &Vec<u8>, lit: &[u8], Ghost(s): Ghost<Seq<char>>) -> (out: bool)
    requires
        lit@ == ckc_spec::v1text::ascii(s),
    ensures
        out == (name@ == ckc_spec::v1text::ascii(s)),
{
    crate::k3_front::bytes_equal(name, lit)
}

fn is_int(arena: &ETermArena, root: usize) -> (out: bool)
    requires
        crate::k2_term::root_ok(arena, root),
    ensures
        out == (arena@[root as int] is Int),
{
    proof {
        assert(crate::k2_term::node_ok(arena.nodes@, root as int));
    }
    matches!(arena.nodes[root].kind, ENodeKind::Int { .. })
}

fn is_atom(arena: &ETermArena, root: usize) -> (out: bool)
    requires
        crate::k2_term::root_ok(arena, root),
    ensures
        out == (arena@[root as int] is Atom),
{
    proof {
        assert(crate::k2_term::node_ok(arena.nodes@, root as int));
    }
    matches!(arena.nodes[root].kind, ENodeKind::Atom { .. })
}

fn is_var(arena: &ETermArena, root: usize) -> (out: bool)
    requires
        crate::k2_term::root_ok(arena, root),
    ensures
        out == (arena@[root as int] is Var),
{
    proof {
        assert(crate::k2_term::node_ok(arena.nodes@, root as int));
    }
    matches!(arena.nodes[root].kind, ENodeKind::Var { .. })
}

// `ref(N)` | `box(N)`, N an integer.
fn slot_ok_exec(arena: &ETermArena, root: usize) -> (out: bool)
    requires
        crate::k2_term::root_ok(arena, root),
    ensures
        out == ckc_spec::model::slot_ok(arena@[root as int]),
{
    proof {
        assert(crate::k2_term::node_ok(arena.nodes@, root as int));
    }
    match &arena.nodes[root].kind {
        ENodeKind::Comp { name, child_roots, .. } => {
            let r: &[u8] = b"ref";
            let b: &[u8] = b"box";
            proof {
                reveal_byteslit(b"ref");
                reveal_strlit("ref");
                reveal_byteslit(b"box");
                reveal_strlit("box");
                reveal(ckc_spec::v1text::ascii);
                assert(r@ == ckc_spec::v1text::ascii("ref"@));
                assert(b@ == ckc_spec::v1text::ascii("box"@));
            }
            (name_is(name, r, Ghost("ref"@)) || name_is(name, b, Ghost("box"@)))
                && child_roots.len() == 1 && is_int(arena, child_roots[0])
        },
        _ => false,
    }
}

// A list of variables, [] included: the cons spine followed from the root.
spec fn var_spine(t: Term) -> bool
    decreases t,
{
    match t {
        Term::Nil => true,
        Term::Comp(f, a) => f == ckc_spec::v1text::cons_name() && a.len() == 2 && a[0] is Var
            && var_spine(a[1]),
        _ => false,
    }
}

proof fn var_spine_is_var_list(t: Term)
    ensures
        var_spine(t) == ckc_spec::model::var_list(t),
    decreases t,
{
    if let Term::Comp(f, a) = t {
        if f == ckc_spec::v1text::cons_name() && a.len() == 2 {
            var_spine_is_var_list(a[1]);
            match ckc_spec::answers::list_items(a[1]) {
                Option::Some(rest) => {
                    let xs = seq![a[0]] + rest;
                    assert(xs[0] == a[0]);
                    assert forall|k: int| 0 <= k < rest.len() implies xs[k + 1] == rest[k] by {}
                    if ckc_spec::model::var_list(t) {
                        assert(xs[0] is Var);
                        assert forall|k: int| 0 <= k < rest.len() implies (
                        #[trigger] rest[k]) is Var by {
                            assert(xs[k + 1] is Var);
                        }
                    }
                    if a[0] is Var && ckc_spec::model::var_list(a[1]) {
                        assert forall|k: int| 0 <= k < xs.len() implies (
                        #[trigger] xs[k]) is Var by {
                            if k > 0 {
                                assert(xs[k] == rest[k - 1]);
                            }
                        }
                    }
                },
                Option::None => {},
            }
        }
    }
}

fn var_list_exec(arena: &ETermArena, root: usize) -> (out: bool)
    requires
        crate::k2_term::root_ok(arena, root),
    ensures
        out == ckc_spec::model::var_list(arena@[root as int]),
    decreases root,
{
    proof {
        assert(crate::k2_term::node_ok(arena.nodes@, root as int));
        var_spine_is_var_list(arena@[root as int]);
    }
    match &arena.nodes[root].kind {
        ENodeKind::Nil => true,
        ENodeKind::Comp { name, child_roots, .. } => {
            let cons: &[u8] = b"[|]";
            proof {
                reveal_byteslit(b"[|]");
                reveal(ckc_spec::v1text::cons_name);
                assert(cons@ == ckc_spec::v1text::cons_name());
            }
            if child_roots.len() == 2 && crate::k3_front::bytes_equal(name, cons) && is_var(
                arena,
                child_roots[0],
            ) {
                let tail = child_roots[1];
                proof {
                    var_spine_is_var_list(arena@[tail as int]);
                }
                var_list_exec(arena, tail)
            } else {
                false
            }
        },
        _ => false,
    }
}

// `'$guideline_id'(Kind, Doc, S, Slot, Deps)`.
fn id_ok_exec(arena: &ETermArena, root: usize) -> (out: bool)
    requires
        crate::k2_term::root_ok(arena, root),
    ensures
        out == match arena@[root as int] {
            Term::Comp(f, a) => f == ckc_spec::model::id_name() && a.len() == 5 && a[0] is Atom
                && a[1] is Atom && a[2] is Int && ckc_spec::model::slot_ok(a[3])
                && ckc_spec::model::var_list(a[4]),
            _ => false,
        },
{
    proof {
        assert(crate::k2_term::node_ok(arena.nodes@, root as int));
    }
    match &arena.nodes[root].kind {
        ENodeKind::Comp { name, child_roots, .. } => {
            let id: &[u8] = b"$guideline_id";
            proof {
                reveal_byteslit(b"$guideline_id");
                reveal_strlit("$guideline_id");
                reveal(ckc_spec::v1text::ascii);
                assert(id@ == ckc_spec::v1text::ascii("$guideline_id"@));
            }
            name_is(name, id, Ghost("$guideline_id"@)) && child_roots.len() == 5 && is_atom(
                arena,
                child_roots[0],
            ) && is_atom(arena, child_roots[1]) && is_int(arena, child_roots[2]) && slot_ok_exec(
                arena,
                child_roots[3],
            ) && var_list_exec(arena, child_roots[4])
        },
        _ => false,
    }
}

pub fn g1_term_exec(arena: &ETermArena, root: usize) -> (out: bool)
    requires
        crate::k2_term::root_ok(arena, root),
    ensures
        out == ckc_spec::model::g1_term(arena@[root as int]),
{
    proof {
        assert(crate::k2_term::node_ok(arena.nodes@, root as int));
    }
    match &arena.nodes[root].kind {
        ENodeKind::Var { .. } => true,
        ENodeKind::Atom { .. } => true,
        ENodeKind::Int { .. } => true,
        ENodeKind::Nil => false,
        ENodeKind::Comp { .. } => crate::arith_impl::decode_number(arena, root).is_some()
            || id_ok_exec(arena, root),
    }
}

// The single argument of `name(_)`, as a root.
fn unary_exec(arena: &ETermArena, root: usize, lit: &[u8], Ghost(s): Ghost<Seq<char>>) -> (out:
    Option<usize>)
    requires
        crate::k2_term::root_ok(arena, root),
        lit@ == ckc_spec::v1text::ascii(s),
    ensures
        match ckc_spec::model::unary(arena@[root as int], s) {
            Option::Some(x) => out matches Some(r) && r < arena.nodes@.len() && arena@[r as int]
                == x,
            Option::None => out is None,
        },
{
    proof {
        assert(crate::k2_term::node_ok(arena.nodes@, root as int));
    }
    match &arena.nodes[root].kind {
        ENodeKind::Comp { name, child_roots, .. } => {
            if child_roots.len() == 1 && name_is(name, lit, Ghost(s)) {
                Some(child_roots[0])
            } else {
                None
            }
        },
        _ => None,
    }
}

// `none` | `sha256(H)`, H an atom.
fn digest_opt_exec(arena: &ETermArena, root: usize) -> (out: bool)
    requires
        crate::k2_term::root_ok(arena, root),
    ensures
        out == ckc_spec::model::digest_opt(arena@[root as int]),
{
    let none: &[u8] = b"none";
    let sha: &[u8] = b"sha256";
    proof {
        reveal_byteslit(b"none");
        reveal_strlit("none");
        reveal_byteslit(b"sha256");
        reveal_strlit("sha256");
        reveal(ckc_spec::v1text::ascii);
        assert(none@ == ckc_spec::v1text::ascii("none"@));
        assert(sha@ == ckc_spec::v1text::ascii("sha256"@));
        assert(crate::k2_term::node_ok(arena.nodes@, root as int));
    }
    if crate::k3_front::is_atom(arena, root, none) {
        return true;
    }
    match unary_exec(arena, root, sha, Ghost("sha256"@)) {
        Some(h) => is_atom(arena, h),
        None => false,
    }
}

pub fn indicator_named(name: &Vec<u8>, i: usize) -> (out: bool)
    requires
        i < 22,
    ensures
        out == (name@ == ckc_spec::v1text::indicator(i as int).0),
{
    let (candidate, _) = crate::k2_recursion::indicator_exec(i);
    crate::k2_engine::vec_equal(&candidate, name)
}

fn metadata_pred_exec(arena: &ETermArena, root: usize) -> (out: bool)
    requires
        crate::k2_term::root_ok(arena, root),
    ensures
        out == ckc_spec::model::metadata_pred(arena@[root as int]),
{
    proof {
        assert(crate::k2_term::node_ok(arena.nodes@, root as int));
    }
    match &arena.nodes[root].kind {
        ENodeKind::Comp { name, child_roots, .. } => child_roots.len() == 4 && (indicator_named(
            name,
            1,
        ) || indicator_named(name, 20)),
        _ => false,
    }
}

fn metadata_ok_exec(arena: &ETermArena, root: usize) -> (out: bool)
    requires
        crate::k2_term::root_ok(arena, root),
    ensures
        out == ckc_spec::model::metadata_ok(arena@[root as int]),
{
    proof {
        assert(crate::k2_term::node_ok(arena.nodes@, root as int));
    }
    match &arena.nodes[root].kind {
        ENodeKind::Comp { name, child_roots, .. } => {
            if indicator_named(name, 1) && child_roots.len() == 4 {
                let ace: &[u8] = b"ace_sha256";
                let ulex: &[u8] = b"ulex";
                let voc: &[u8] = b"vocabulary";
                proof {
                    reveal_byteslit(b"ace_sha256");
                    reveal_strlit("ace_sha256");
                    reveal_byteslit(b"ulex");
                    reveal_strlit("ulex");
                    reveal_byteslit(b"vocabulary");
                    reveal_strlit("vocabulary");
                    reveal(ckc_spec::v1text::ascii);
                    assert(ace@ == ckc_spec::v1text::ascii("ace_sha256"@));
                    assert(ulex@ == ckc_spec::v1text::ascii("ulex"@));
                    assert(voc@ == ckc_spec::v1text::ascii("vocabulary"@));
                }
                is_atom(arena, child_roots[0]) && match unary_exec(
                    arena,
                    child_roots[1],
                    ace,
                    Ghost("ace_sha256"@),
                ) {
                    Some(h) => is_atom(arena, h),
                    None => false,
                } && match unary_exec(arena, child_roots[2], ulex, Ghost("ulex"@)) {
                    Some(u) => digest_opt_exec(arena, u),
                    None => false,
                } && match unary_exec(arena, child_roots[3], voc, Ghost("vocabulary"@)) {
                    Some(v) => digest_opt_exec(arena, v),
                    None => false,
                }
            } else if indicator_named(name, 20) && child_roots.len() == 4 {
                let src: &[u8] = b"source";
                proof {
                    reveal_byteslit(b"source");
                    reveal_strlit("source");
                    reveal(ckc_spec::v1text::ascii);
                    assert(src@ == ckc_spec::v1text::ascii("source"@));
                }
                is_atom(arena, child_roots[0]) && is_int(arena, child_roots[1]) && {
                    let s = child_roots[2];
                    proof {
                        assert(crate::k2_term::node_ok(arena.nodes@, s as int));
                    }
                    match &arena.nodes[s].kind {
                        ENodeKind::Comp { name: g, child_roots: b, .. } => name_is(
                            g,
                            src,
                            Ghost("source"@),
                        ) && b.len() == 2 && is_atom(arena, b[0]) && is_atom(arena, b[1]),
                        _ => false,
                    }
                } && is_int(arena, child_roots[3])
            } else {
                false
            }
        },
        _ => false,
    }
}

pub fn schema_literal_exec(arena: &ETermArena, root: usize) -> (out: bool)
    requires
        crate::k2_term::root_ok(arena, root),
    ensures
        out == ckc_spec::model::schema_literal(arena@[root as int]),
{
    proof {
        assert(crate::k2_term::node_ok(arena.nodes@, root as int));
        reveal(ckc_spec::v1text::indicator_count);
    }
    match &arena.nodes[root].kind {
        ENodeKind::Comp { name, child_roots, .. } => {
            let arity = child_roots.len();
            let ghost args = crate::k2_term::child_terms(arena.nodes@, child_roots@);
            proof {
                assert(arena@[root as int] == Term::Comp(name@, args));
            }
            let mut i = 0usize;
            while i < 22
                invariant
                    crate::k2_term::root_ok(arena, root),
                    arena@[root as int] == Term::Comp(name@, args),
                    args.len() == arity,
                    0 <= i <= 22,
                    ckc_spec::v1text::indicator_count() == 22,
                    forall|j: int|
                        0 <= j < i ==> #[trigger] ckc_spec::v1text::indicator(j) != (
                            name@,
                            arity as nat,
                        ),
                decreases 22 - i,
            {
                let (candidate, count) = crate::k2_recursion::indicator_exec(i);
                if count == arity && crate::k2_engine::vec_equal(&candidate, name) {
                    proof {
                        assert(ckc_spec::v1text::indicator(i as int) == (name@, arity as nat));
                    }
                    return true;
                }
                i += 1;
            }
            false
        },
        _ => false,
    }
}

pub fn g1_literal_exec(arena: &ETermArena, root: usize) -> (out: bool)
    requires
        crate::k2_term::root_ok(arena, root),
    ensures
        out == ckc_spec::model::g1_literal(arena@[root as int]),
{
    if metadata_pred_exec(arena, root) {
        return metadata_ok_exec(arena, root);
    }
    proof {
        assert(crate::k2_term::node_ok(arena.nodes@, root as int));
    }
    match &arena.nodes[root].kind {
        ENodeKind::Comp { name, child_roots, .. } => {
            let ghost args = crate::k2_term::child_terms(arena.nodes@, child_roots@);
            proof {
                assert(arena@[root as int] == Term::Comp(name@, args));
            }
            let mut k = 0usize;
            while k < child_roots.len()
                invariant
                    crate::k2_term::root_ok(arena, root),
                    arena@[root as int] == Term::Comp(name@, args),
                    !ckc_spec::model::metadata_pred(arena@[root as int]),
                    crate::k2_term::child_roots_valid(arena.nodes@, child_roots@),
                    args == crate::k2_term::child_terms(arena.nodes@, child_roots@),
                    k <= child_roots@.len(),
                    forall|q: int| 0 <= q < k ==> ckc_spec::model::g1_term(#[trigger] args[q]),
                decreases child_roots.len() - k,
            {
                proof {
                    assert(args[k as int] == arena@[child_roots@[k as int] as int]);
                }
                if !g1_term_exec(arena, child_roots[k]) {
                    proof {
                        assert(!ckc_spec::model::g1_term(args[k as int]));
                    }
                    return false;
                }
                k += 1;
            }
            true
        },
        _ => false,
    }
}

} // verus!
