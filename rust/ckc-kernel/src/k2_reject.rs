use crate::k2_manifest::udec_vec;
use crate::k2_term::{ETermArena, push_atom, push_comp, push_int, term_line};
#[cfg(verus_keep_ghost)]
use crate::k2_term::{arena_ok, child_terms, child_terms_match, root_ok};
use ckc_spec::replay::*;
use ckc_spec::term::Term;
use vstd::prelude::*;
use vstd::slice::slice_to_vec;

verus! {

struct ERootedArena {
    arena: ETermArena,
    root: usize,
}

pub(crate) fn empty_arena() -> (arena: ETermArena)
    ensures
        arena_ok(&arena),
{
    let arena = ETermArena { nodes: Vec::new() };
    proof {
        reveal(arena_ok);
    }
    arena
}

fn push_named_atom(arena: &mut ETermArena, bytes: &[u8], name: Ghost<Seq<u8>>) -> (root: usize)
    requires
        arena_ok(old(arena)),
        bytes@ == name@,
    ensures
        arena_ok(final(arena)),
        root == old(arena).nodes@.len(),
        final(arena).nodes@ == old(arena).nodes@.push(final(arena).nodes@[root as int]),
        final(arena)@[root as int] == Term::Atom(name@),
{
    let name_vec = slice_to_vec(bytes);
    push_atom(arena, name_vec)
}

fn push_named_comp(
    arena: &mut ETermArena,
    bytes: &[u8],
    name: Ghost<Seq<u8>>,
    child_roots: Vec<usize>,
    args: Ghost<Seq<Term>>,
) -> (root: usize)
    requires
        arena_ok(old(arena)),
        bytes@ == name@,
        child_roots@.len() > 0,
        forall|i: int|
            0 <= i < child_roots@.len() ==> #[trigger] child_roots@[i] < old(arena).nodes@.len(),
        child_terms(old(arena).nodes@, child_roots@) == args@,
    ensures
        arena_ok(final(arena)),
        root == old(arena).nodes@.len(),
        final(arena).nodes@ == old(arena).nodes@.push(final(arena).nodes@[root as int]),
        final(arena)@[root as int] == Term::Comp(name@, args@),
{
    let name_vec = slice_to_vec(bytes);
    push_comp(arena, name_vec, child_roots)
}

pub(crate) fn push_usize_int(arena: &mut ETermArena, n: usize) -> (root: usize)
    requires
        arena_ok(old(arena)),
    ensures
        arena_ok(final(arena)),
        root == old(arena).nodes@.len(),
        final(arena).nodes@ == old(arena).nodes@.push(final(arena).nodes@[root as int]),
        final(arena)@[root as int] == Term::Int(n as int),
{
    let spelling = udec_vec(n);
    let magnitude = udec_vec(n);
    proof {
        reveal(ckc_spec::v1text::dec_bytes);
    }
    push_int(arena, spelling, magnitude, false, Ghost(n as int))
}

fn one_child(arena: &ETermArena, child: usize, model: Ghost<Term>) -> (roots: Vec<usize>)
    requires
        root_ok(arena, child),
        arena@[child as int] == model@,
    ensures
        roots@ == seq![child],
        forall|i: int| 0 <= i < roots@.len() ==> #[trigger] roots@[i] < arena.nodes@.len(),
        child_terms(arena.nodes@, roots@) == seq![model@],
{
    let mut roots = Vec::new();
    roots.push(child);
    proof {
        reveal(root_ok);
        child_terms_match(arena.nodes@, roots@, seq![model@]);
    }
    roots
}

fn two_children(
    arena: &ETermArena,
    left: usize,
    right: usize,
    left_model: Ghost<Term>,
    right_model: Ghost<Term>,
) -> (roots: Vec<usize>)
    requires
        root_ok(arena, left),
        root_ok(arena, right),
        arena@[left as int] == left_model@,
        arena@[right as int] == right_model@,
    ensures
        roots@ == seq![left, right],
        forall|i: int| 0 <= i < roots@.len() ==> #[trigger] roots@[i] < arena.nodes@.len(),
        child_terms(arena.nodes@, roots@) == seq![left_model@, right_model@],
{
    let mut roots = Vec::new();
    roots.push(left);
    roots.push(right);
    proof {
        reveal(root_ok);
        child_terms_match(arena.nodes@, roots@, seq![left_model@, right_model@]);
    }
    roots
}

fn unreadable_arena() -> (out: ERootedArena)
    ensures
        root_ok(&out.arena, out.root),
        out.arena@[out.root as int] == Term::Comp(
            ckc_spec::v1text::ascii("ace_to_pl_error"@),
            seq![
                Term::Atom(ckc_spec::v1text::ascii("check_load"@)),
                Term::Atom(ckc_spec::v1text::ascii("unreadable"@)),
            ],
        ),
{
    let outer_bytes: &[u8] = b"ace_to_pl_error";
    let class_bytes: &[u8] = b"check_load";
    let detail_bytes: &[u8] = b"unreadable";
    proof {
        reveal_strlit("ace_to_pl_error");
        reveal_byteslit(b"ace_to_pl_error");
        reveal_strlit("check_load");
        reveal_byteslit(b"check_load");
        reveal_strlit("unreadable");
        reveal_byteslit(b"unreadable");
        reveal(ckc_spec::v1text::ascii);
    }
    let ghost outer_name = ckc_spec::v1text::ascii("ace_to_pl_error"@);
    let ghost class_name = ckc_spec::v1text::ascii("check_load"@);
    let ghost detail_name = ckc_spec::v1text::ascii("unreadable"@);
    proof {
        assert(outer_bytes@ == outer_name);
        assert(class_bytes@ == class_name);
        assert(detail_bytes@ == detail_name);
    }
    let ghost class_term = Term::Atom(class_name);
    let ghost detail_term = Term::Atom(detail_name);
    let ghost outer_args = seq![class_term, detail_term];
    let mut arena = empty_arena();
    let class_root = push_named_atom(&mut arena, class_bytes, Ghost(class_name));
    let detail_root = push_named_atom(&mut arena, detail_bytes, Ghost(detail_name));
    let children = two_children(
        &arena,
        class_root,
        detail_root,
        Ghost(class_term),
        Ghost(detail_term),
    );
    let root = push_named_comp(
        &mut arena,
        outer_bytes,
        Ghost(outer_name),
        children,
        Ghost(outer_args),
    );
    proof {
        reveal(root_ok);
    }
    ERootedArena { arena, root }
}

#[verifier::rlimit(5000)]
fn utf8_arena(off: usize) -> (out: ERootedArena)
    ensures
        root_ok(&out.arena, out.root),
        out.arena@[out.root as int] == Term::Comp(
            ckc_spec::v1text::ascii("ace_to_pl_error"@),
            seq![
                Term::Atom(ckc_spec::v1text::ascii("check_load"@)),
                Term::Comp(
                    ckc_spec::v1text::ascii("error"@),
                    seq![
                        Term::Comp(
                            ckc_spec::v1text::ascii("syntax_error"@),
                            seq![Term::Atom(ckc_spec::v1text::ascii("invalid_utf8"@))],
                        ),
                        ctx(
                            "read_utf8_input"@,
                            3,
                            Term::Comp(
                                ckc_spec::v1text::ascii("byte_offset"@),
                                seq![Term::Int(off as int)],
                            ),
                        ),
                    ],
                ),
            ],
        ),
{
    let outer_bytes: &[u8] = b"ace_to_pl_error";
    let class_bytes: &[u8] = b"check_load";
    let error_bytes: &[u8] = b"error";
    let syntax_bytes: &[u8] = b"syntax_error";
    let invalid_bytes: &[u8] = b"invalid_utf8";
    let context_bytes: &[u8] = b"context";
    let colon_bytes: &[u8] = b":";
    let ace_bytes: &[u8] = b"ace_to_pl";
    let slash_bytes: &[u8] = b"/";
    let offset_bytes: &[u8] = b"byte_offset";
    let reader_bytes: &[u8] = b"read_utf8_input";
    proof {
        reveal_strlit("ace_to_pl_error");
        reveal_byteslit(b"ace_to_pl_error");
        reveal_strlit("check_load");
        reveal_byteslit(b"check_load");
        reveal_strlit("error");
        reveal_byteslit(b"error");
        reveal_strlit("syntax_error");
        reveal_byteslit(b"syntax_error");
        reveal_strlit("invalid_utf8");
        reveal_byteslit(b"invalid_utf8");
        reveal_strlit("context");
        reveal_byteslit(b"context");
        reveal_strlit(":");
        reveal_byteslit(b":");
        reveal_strlit("ace_to_pl");
        reveal_byteslit(b"ace_to_pl");
        reveal_strlit("/");
        reveal_byteslit(b"/");
        reveal_strlit("byte_offset");
        reveal_byteslit(b"byte_offset");
        reveal_strlit("read_utf8_input");
        reveal_byteslit(b"read_utf8_input");
        reveal(ckc_spec::v1text::ascii);
    }
    let ghost outer_name = ckc_spec::v1text::ascii("ace_to_pl_error"@);
    let ghost class_name = ckc_spec::v1text::ascii("check_load"@);
    let ghost error_name = ckc_spec::v1text::ascii("error"@);
    let ghost syntax_name = ckc_spec::v1text::ascii("syntax_error"@);
    let ghost invalid_name = ckc_spec::v1text::ascii("invalid_utf8"@);
    let ghost context_name = ckc_spec::v1text::ascii("context"@);
    let ghost colon_name = ckc_spec::v1text::ascii(":"@);
    let ghost ace_name = ckc_spec::v1text::ascii("ace_to_pl"@);
    let ghost slash_name = ckc_spec::v1text::ascii("/"@);
    let ghost offset_name = ckc_spec::v1text::ascii("byte_offset"@);
    let ghost reader_name = ckc_spec::v1text::ascii("read_utf8_input"@);
    proof {
        assert(outer_bytes@ == outer_name);
        assert(class_bytes@ == class_name);
        assert(error_bytes@ == error_name);
        assert(syntax_bytes@ == syntax_name);
        assert(invalid_bytes@ == invalid_name);
        assert(context_bytes@ == context_name);
        assert(colon_bytes@ == colon_name);
        assert(ace_bytes@ == ace_name);
        assert(slash_bytes@ == slash_name);
        assert(offset_bytes@ == offset_name);
        assert(reader_bytes@ == reader_name);
    }
    let ghost invalid_term = Term::Atom(invalid_name);
    let ghost syntax_term = Term::Comp(syntax_name, seq![invalid_term]);
    let ghost reader_term = Term::Atom(reader_name);
    let ghost three_term = Term::Int(3);
    let ghost slash_term = Term::Comp(slash_name, seq![reader_term, three_term]);
    let ghost ace_term = Term::Atom(ace_name);
    let ghost colon_term = Term::Comp(colon_name, seq![ace_term, slash_term]);
    let ghost off_term = Term::Int(off as int);
    let ghost offset_term = Term::Comp(offset_name, seq![off_term]);
    let ghost context_term = Term::Comp(context_name, seq![colon_term, offset_term]);
    let ghost detail_term = Term::Comp(error_name, seq![syntax_term, context_term]);
    let ghost class_term = Term::Atom(class_name);
    let ghost syntax_args = seq![invalid_term];
    let ghost slash_args = seq![reader_term, three_term];
    let ghost colon_args = seq![ace_term, slash_term];
    let ghost offset_args = seq![off_term];
    let ghost context_args = seq![colon_term, offset_term];
    let ghost error_args = seq![syntax_term, context_term];
    let ghost outer_args = seq![class_term, detail_term];
    let mut arena = empty_arena();

    let invalid_root = push_named_atom(&mut arena, invalid_bytes, Ghost(invalid_name));
    let syntax_children = one_child(&arena, invalid_root, Ghost(invalid_term));
    let syntax_root = push_named_comp(
        &mut arena,
        syntax_bytes,
        Ghost(syntax_name),
        syntax_children,
        Ghost(syntax_args),
    );

    let reader_root = push_named_atom(&mut arena, reader_bytes, Ghost(reader_name));
    let three_root = push_usize_int(&mut arena, 3);
    let slash_children = two_children(
        &arena,
        reader_root,
        three_root,
        Ghost(reader_term),
        Ghost(three_term),
    );
    let slash_root = push_named_comp(
        &mut arena,
        slash_bytes,
        Ghost(slash_name),
        slash_children,
        Ghost(slash_args),
    );
    let ace_root = push_named_atom(&mut arena, ace_bytes, Ghost(ace_name));
    let colon_children = two_children(
        &arena,
        ace_root,
        slash_root,
        Ghost(ace_term),
        Ghost(slash_term),
    );
    let colon_root = push_named_comp(
        &mut arena,
        colon_bytes,
        Ghost(colon_name),
        colon_children,
        Ghost(colon_args),
    );

    let off_root = push_usize_int(&mut arena, off);
    let offset_children = one_child(&arena, off_root, Ghost(off_term));
    let offset_root = push_named_comp(
        &mut arena,
        offset_bytes,
        Ghost(offset_name),
        offset_children,
        Ghost(offset_args),
    );
    let context_children = two_children(
        &arena,
        colon_root,
        offset_root,
        Ghost(colon_term),
        Ghost(offset_term),
    );
    let context_root = push_named_comp(
        &mut arena,
        context_bytes,
        Ghost(context_name),
        context_children,
        Ghost(context_args),
    );

    let error_children = two_children(
        &arena,
        syntax_root,
        context_root,
        Ghost(syntax_term),
        Ghost(context_term),
    );
    let detail_root = push_named_comp(
        &mut arena,
        error_bytes,
        Ghost(error_name),
        error_children,
        Ghost(error_args),
    );
    let class_root = push_named_atom(&mut arena, class_bytes, Ghost(class_name));
    let outer_children = two_children(
        &arena,
        class_root,
        detail_root,
        Ghost(class_term),
        Ghost(detail_term),
    );
    let root = push_named_comp(
        &mut arena,
        outer_bytes,
        Ghost(outer_name),
        outer_children,
        Ghost(outer_args),
    );
    proof {
        reveal(ctx);
        reveal(root_ok);
    }
    ERootedArena { arena, root }
}

pub(crate) fn utf8_out(off: usize) -> (out: EOut)
    ensures
        out@ == utf8_reject(off as nat),
{
    let rooted = utf8_arena(off);
    let err = term_line(&rooted.arena, rooted.root);
    proof {
        reveal(utf8_reject);
        reveal(check_load);
        reveal(atom);
        reveal(reject);
        reveal(error_line);
    }
    EOut { rc: 2, out: Vec::new(), err }
}

#[verifier::rlimit(5000)]
fn manifest_arena(mpath: &[u8], detail_bytes: &[u8], bad_line: bool) -> (out: ERootedArena)
    ensures
        root_ok(&out.arena, out.root),
        out.arena@[out.root as int] == Term::Comp(
            ckc_spec::v1text::ascii("ace_to_pl_error"@),
            seq![
                Term::Atom(ckc_spec::v1text::ascii("check_load"@)),
                Term::Comp(
                    ckc_spec::v1text::ascii("error"@),
                    seq![
                        Term::Comp(
                            ckc_spec::v1text::ascii("aggregate_manifest"@),
                            seq![
                                if bad_line {
                                    Term::Comp(
                                        ckc_spec::v1text::ascii("line"@),
                                        seq![Term::Atom(detail_bytes@)],
                                    )
                                } else {
                                    Term::Atom(ckc_spec::v1text::ascii("missing_final_newline"@))
                                },
                            ],
                        ),
                        ctx("aggregate_read_manifest"@, 3, Term::Atom(mpath@)),
                    ],
                ),
            ],
        ),
{
    let outer_bytes: &[u8] = b"ace_to_pl_error";
    let class_bytes: &[u8] = b"check_load";
    let error_bytes: &[u8] = b"error";
    let aggregate_bytes: &[u8] = b"aggregate_manifest";
    let line_bytes: &[u8] = b"line";
    let missing_bytes: &[u8] = b"missing_final_newline";
    let context_bytes: &[u8] = b"context";
    let colon_bytes: &[u8] = b":";
    let ace_bytes: &[u8] = b"ace_to_pl";
    let slash_bytes: &[u8] = b"/";
    let reader_bytes: &[u8] = b"aggregate_read_manifest";
    proof {
        reveal_strlit("ace_to_pl_error");
        reveal_byteslit(b"ace_to_pl_error");
        reveal_strlit("check_load");
        reveal_byteslit(b"check_load");
        reveal_strlit("error");
        reveal_byteslit(b"error");
        reveal_strlit("aggregate_manifest");
        reveal_byteslit(b"aggregate_manifest");
        reveal_strlit("line");
        reveal_byteslit(b"line");
        reveal_strlit("missing_final_newline");
        reveal_byteslit(b"missing_final_newline");
        reveal_strlit("context");
        reveal_byteslit(b"context");
        reveal_strlit(":");
        reveal_byteslit(b":");
        reveal_strlit("ace_to_pl");
        reveal_byteslit(b"ace_to_pl");
        reveal_strlit("/");
        reveal_byteslit(b"/");
        reveal_strlit("aggregate_read_manifest");
        reveal_byteslit(b"aggregate_read_manifest");
        reveal(ckc_spec::v1text::ascii);
    }
    let ghost outer_name = ckc_spec::v1text::ascii("ace_to_pl_error"@);
    let ghost class_name = ckc_spec::v1text::ascii("check_load"@);
    let ghost error_name = ckc_spec::v1text::ascii("error"@);
    let ghost aggregate_name = ckc_spec::v1text::ascii("aggregate_manifest"@);
    let ghost line_name = ckc_spec::v1text::ascii("line"@);
    let ghost missing_name = ckc_spec::v1text::ascii("missing_final_newline"@);
    let ghost context_name = ckc_spec::v1text::ascii("context"@);
    let ghost colon_name = ckc_spec::v1text::ascii(":"@);
    let ghost ace_name = ckc_spec::v1text::ascii("ace_to_pl"@);
    let ghost slash_name = ckc_spec::v1text::ascii("/"@);
    let ghost reader_name = ckc_spec::v1text::ascii("aggregate_read_manifest"@);
    proof {
        assert(outer_bytes@ == outer_name);
        assert(class_bytes@ == class_name);
        assert(error_bytes@ == error_name);
        assert(aggregate_bytes@ == aggregate_name);
        assert(line_bytes@ == line_name);
        assert(missing_bytes@ == missing_name);
        assert(context_bytes@ == context_name);
        assert(colon_bytes@ == colon_name);
        assert(ace_bytes@ == ace_name);
        assert(slash_bytes@ == slash_name);
        assert(reader_bytes@ == reader_name);
    }
    let ghost detail_atom = Term::Atom(detail_bytes@);
    let ghost d = if bad_line {
        Term::Comp(line_name, seq![detail_atom])
    } else {
        Term::Atom(missing_name)
    };
    let ghost aggregate_term = Term::Comp(aggregate_name, seq![d]);
    let ghost reader_term = Term::Atom(reader_name);
    let ghost three_term = Term::Int(3);
    let ghost slash_term = Term::Comp(slash_name, seq![reader_term, three_term]);
    let ghost ace_term = Term::Atom(ace_name);
    let ghost colon_term = Term::Comp(colon_name, seq![ace_term, slash_term]);
    let ghost mpath_term = Term::Atom(mpath@);
    let ghost context_term = Term::Comp(context_name, seq![colon_term, mpath_term]);
    let ghost detail_term = Term::Comp(error_name, seq![aggregate_term, context_term]);
    let ghost class_term = Term::Atom(class_name);
    let ghost line_args = seq![detail_atom];
    let ghost aggregate_args = seq![d];
    let ghost slash_args = seq![reader_term, three_term];
    let ghost colon_args = seq![ace_term, slash_term];
    let ghost context_args = seq![colon_term, mpath_term];
    let ghost error_args = seq![aggregate_term, context_term];
    let ghost outer_args = seq![class_term, detail_term];
    let mut arena = empty_arena();

    let d_root = if bad_line {
        let detail_root = push_named_atom(&mut arena, detail_bytes, Ghost(detail_bytes@));
        let line_children = one_child(&arena, detail_root, Ghost(detail_atom));
        push_named_comp(&mut arena, line_bytes, Ghost(line_name), line_children, Ghost(line_args))
    } else {
        push_named_atom(&mut arena, missing_bytes, Ghost(missing_name))
    };
    let aggregate_children = one_child(&arena, d_root, Ghost(d));
    let aggregate_root = push_named_comp(
        &mut arena,
        aggregate_bytes,
        Ghost(aggregate_name),
        aggregate_children,
        Ghost(aggregate_args),
    );

    let reader_root = push_named_atom(&mut arena, reader_bytes, Ghost(reader_name));
    let three_root = push_usize_int(&mut arena, 3);
    let slash_children = two_children(
        &arena,
        reader_root,
        three_root,
        Ghost(reader_term),
        Ghost(three_term),
    );
    let slash_root = push_named_comp(
        &mut arena,
        slash_bytes,
        Ghost(slash_name),
        slash_children,
        Ghost(slash_args),
    );
    let ace_root = push_named_atom(&mut arena, ace_bytes, Ghost(ace_name));
    let colon_children = two_children(
        &arena,
        ace_root,
        slash_root,
        Ghost(ace_term),
        Ghost(slash_term),
    );
    let colon_root = push_named_comp(
        &mut arena,
        colon_bytes,
        Ghost(colon_name),
        colon_children,
        Ghost(colon_args),
    );
    let mpath_root = push_named_atom(&mut arena, mpath, Ghost(mpath@));
    let context_children = two_children(
        &arena,
        colon_root,
        mpath_root,
        Ghost(colon_term),
        Ghost(mpath_term),
    );
    let context_root = push_named_comp(
        &mut arena,
        context_bytes,
        Ghost(context_name),
        context_children,
        Ghost(context_args),
    );

    let error_children = two_children(
        &arena,
        aggregate_root,
        context_root,
        Ghost(aggregate_term),
        Ghost(context_term),
    );
    let detail_root = push_named_comp(
        &mut arena,
        error_bytes,
        Ghost(error_name),
        error_children,
        Ghost(error_args),
    );
    let class_root = push_named_atom(&mut arena, class_bytes, Ghost(class_name));
    let outer_children = two_children(
        &arena,
        class_root,
        detail_root,
        Ghost(class_term),
        Ghost(detail_term),
    );
    let root = push_named_comp(
        &mut arena,
        outer_bytes,
        Ghost(outer_name),
        outer_children,
        Ghost(outer_args),
    );
    proof {
        reveal(ctx);
        reveal(root_ok);
    }
    ERootedArena { arena, root }
}

pub(crate) fn manifest_error_out(mpath: &[u8], detail_bytes: &[u8], bad_line: bool) -> (out: EOut)
    ensures
        out@ == if bad_line {
            manifest_reject(
                mpath@,
                Term::Comp(ckc_spec::v1text::ascii("line"@), seq![Term::Atom(detail_bytes@)]),
            )
        } else {
            manifest_reject(mpath@, atom("missing_final_newline"@))
        },
{
    let rooted = manifest_arena(mpath, detail_bytes, bad_line);
    let err = term_line(&rooted.arena, rooted.root);
    proof {
        reveal(manifest_reject);
        reveal(check_load);
        reveal(atom);
        reveal(reject);
        reveal(error_line);
    }
    EOut { rc: 2, out: Vec::new(), err }
}

pub(crate) fn unreadable_out() -> (out: EOut)
    ensures
        out@ == check_load(atom("unreadable"@)),
{
    let rooted = unreadable_arena();
    let err = term_line(&rooted.arena, rooted.root);
    proof {
        reveal(check_load);
        reveal(atom);
        reveal(reject);
        reveal(error_line);
    }
    EOut { rc: 2, out: Vec::new(), err }
}

} // verus!
