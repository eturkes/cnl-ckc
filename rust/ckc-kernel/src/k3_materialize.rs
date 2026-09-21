use crate::k2_engine::EClause;
#[cfg(verus_keep_ghost)]
use crate::k2_engine::{clause_valid, db_valid, db_view, root_terms, roots_valid};
use crate::k2_term::{ENode, ETermArena};
#[cfg(verus_keep_ghost)]
use crate::k2_term::{arena_ok, root_ok};
use crate::k3_state::{EEntry, EEvent};
#[cfg(verus_keep_ghost)]
use crate::k3_state::{entry_valid, entry_view, event_view, log_valid, log_view, path_view};
use ckc_spec::replay::EOut;
#[cfg(verus_keep_ghost)]
use ckc_spec::term::Term;
use ckc_spec::trace::*;
use vstd::assert_seqs_equal;
use vstd::prelude::*;
use vstd::slice::slice_to_vec;

verus! {

pub enum EMat {
    Ok { roots: Vec<usize>, base: usize, nodes: usize },
    Err(EOut),
}

pub open spec fn view(nodes: Seq<ENode>, out: &EMat) -> Mat {
    match out {
        EMat::Ok { roots, base, nodes: count } => Mat::Ok(
            root_terms(nodes, roots@),
            *base as nat,
            *count as nat,
        ),
        EMat::Err(o) => Mat::Err(o@),
    }
}

pub open spec fn valid(nodes: Seq<ENode>, out: &EMat) -> bool {
    match out {
        EMat::Ok { roots, base, .. } => roots_valid(nodes, roots@) && *base <= nodes.len(),
        EMat::Err(_) => true,
    }
}

pub open spec fn allocation(out: &EMat, before: nat, after: nat) -> bool {
    match out {
        EMat::Ok { nodes, .. } => *nodes <= after - before,
        EMat::Err(_) => true,
    }
}

pub open spec fn prepend(acc: Seq<Term>, count: nat, tail: Mat) -> Mat {
    match tail {
        Mat::Ok(ts, b, n) => Mat::Ok(acc + ts, b, count + n),
        Mat::Err(o) => Mat::Err(o),
    }
}

proof fn mat_cons(
    db: Seq<ckc_spec::v1text::DocClause>,
    digests: Seq<Seq<u8>>,
    head: PNode,
    tail: Seq<PNode>,
    base: nat,
)
    ensures
        mat_all(db, digests, seq![head] + tail, base) == match mat(db, digests, head, base) {
            Mat::Err(o) => Mat::Err(o),
            Mat::Ok(first, next, n) => prepend(first, n, mat_all(db, digests, tail, next)),
        },
{
    assert((seq![head] + tail).len() > 0);
    assert((seq![head] + tail)[0] == head);
    assert_seqs_equal!((seq![head] + tail).drop_first() == tail);
    reveal(mat_all);
}

proof fn build_cons(
    db: Seq<ckc_spec::v1text::DocClause>,
    log: Seq<(Seq<nat>, TEv)>,
    path: Seq<nat>,
    i: nat,
    n: nat,
)
    requires
        i < n,
        path.len() <= log.len(),
    ensures
        build_all(db, log, path, i, n) == seq![build(db, log, path.push(i))] + build_all(
            db,
            log,
            path,
            i + 1,
            n,
        ),
{
    reveal(build_all);
}

proof fn prepend_assoc(a: Seq<Term>, n: nat, b: Seq<Term>, m: nat, tail: Mat)
    ensures
        prepend(a, n, prepend(b, m, tail)) == prepend(a + b, n + m, tail),
{
    if let Mat::Ok(ts, _, _) = tail {
        assert_seqs_equal!(a + (b + ts) == (a + b) + ts);
    }
}

fn path_equal(a: &Vec<usize>, b: &Vec<usize>) -> (out: bool)
    ensures
        out == (path_view(a@) == path_view(b@)),
{
    if a.len() != b.len() {
        proof {
            assert(path_view(a@).len() == a.len());
            assert(path_view(b@).len() == b.len());
        }
        return false;
    }
    let mut i = 0usize;
    while i < a.len()
        invariant
            a.len() == b.len(),
            i <= a.len(),
            forall|j: int| 0 <= j < i ==> a@[j] == b@[j],
        decreases a.len() - i,
    {
        if a[i] != b[i] {
            proof {
                assert(path_view(a@)[i as int] == a@[i as int] as nat);
                assert(path_view(b@)[i as int] == b@[i as int] as nat);
            }
            return false;
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(path_view(a@) == path_view(b@));
    }
    true
}

fn event_at(arena: &ETermArena, log: &Vec<EEntry>, path: &Vec<usize>) -> (out: Option<usize>)
    ensures
        match out {
            Some(i) => i < log.len() && ev_at(log_view(arena.nodes@, log@), path_view(path@))
                == Some(event_view(arena.nodes@, &log@[i as int].event)),
            None => ev_at(log_view(arena.nodes@, log@), path_view(path@)) is None,
        },
{
    let mut i = 0usize;
    proof {
        assert_seqs_equal!(log@.skip(0) == log@);
    }
    while i < log.len()
        invariant
            i <= log.len(),
            ev_at(log_view(arena.nodes@, log@), path_view(path@)) == ev_at(
                log_view(arena.nodes@, log@.skip(i as int)),
                path_view(path@),
            ),
        decreases log.len() - i,
    {
        let found = path_equal(&log[i].path, path);
        proof {
            assert(log_view(arena.nodes@, log@.skip(i as int))[0] == entry_view(
                arena.nodes@,
                &log@[i as int],
            ));
            assert_seqs_equal!(log_view(arena.nodes@, log@.skip(i as int)).drop_first() == log_view(arena.nodes@, log@.skip(i as int + 1)));
            reveal(ev_at);
        }
        if found {
            return Some(i);
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(log_view(arena.nodes@, log@.skip(i as int)) == Seq::empty());
        reveal(ev_at);
    }
    None
}

fn naf(mut arena: ETermArena, root: usize, base: usize) -> (out: (ETermArena, EMat))
    requires
        root_ok(&arena, root),
        base <= arena.nodes.len(),
    ensures
        arena_ok(&out.0),
        arena.nodes@.is_prefix_of(out.0.nodes@),
        valid(out.0.nodes@, &out.1),
        allocation(&out.1, arena.nodes.len() as nat, out.0.nodes.len() as nat),
        view(out.0.nodes@, &out.1) == Mat::Ok(
            seq![
                Term::Comp(
                    ckc_spec::v1text::ascii("naf"@),
                    seq![number(arena@[root as int], base as nat).0],
                ),
            ],
            number(arena@[root as int], base as nat).1,
            0,
        ),
{
    let ghost numbered_model = number(arena@[root as int], base as nat);
    let (numbered, next) = crate::k3_number::number_exec(&mut arena, root, base);
    let name: &[u8] = b"naf";
    proof {
        reveal_byteslit(b"naf");
        reveal_strlit("naf");
        reveal(ckc_spec::v1text::ascii);
        assert(name@ == ckc_spec::v1text::ascii("naf"@));
    }
    let root = crate::k2_output::comp1(&mut arena, name, numbered);
    let mut roots = Vec::new();
    roots.push(root);
    proof {
        assert(arena@[root as int] == Term::Comp(
            ckc_spec::v1text::ascii("naf"@),
            seq![numbered_model.0],
        ));
        assert_seqs_equal!(root_terms(arena.nodes@, roots@) == seq![arena@[root as int]]);
    }
    (arena, EMat::Ok { roots, base: next, nodes: 0 })
}

fn clause_node(arena: &mut ETermArena, sentence: usize, hex: &[u8], children: &Vec<usize>) -> (out:
    usize)
    requires
        root_ok(old(arena), sentence),
        roots_valid(old(arena).nodes@, children@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        old(arena).nodes.len() < final(arena).nodes.len(),
        root_ok(final(arena), out),
        final(arena)@[out as int] == Term::Comp(
            ckc_spec::v1text::ascii("clause"@),
            seq![
                old(arena)@[sentence as int],
                Term::Comp(ckc_spec::v1text::ascii("clause_sha256"@), seq![Term::Atom(hex@)]),
                ckc_spec::engine::list_term(root_terms(old(arena).nodes@, children@)),
            ],
        ),
{
    let ghost before = arena.nodes@;
    let children_root = crate::k2_walk::list_root(arena, children);
    let digest = crate::k2_output::atom_root(arena, hex);
    let hash_name: &[u8] = b"clause_sha256";
    let clause_name: &[u8] = b"clause";
    proof {
        reveal_byteslit(b"clause_sha256");
        reveal_strlit("clause_sha256");
        reveal_byteslit(b"clause");
        reveal_strlit("clause");
        reveal(ckc_spec::v1text::ascii);
        assert(hash_name@ == ckc_spec::v1text::ascii("clause_sha256"@));
        assert(clause_name@ == ckc_spec::v1text::ascii("clause"@));
    }
    let hash = crate::k2_output::comp1(arena, hash_name, digest);
    let mut fields = Vec::new();
    fields.push(sentence);
    fields.push(hash);
    fields.push(children_root);
    proof {
        crate::k2_term::arena_prefix_stable(before, arena);
        assert(roots_valid(arena.nodes@, fields@));
        crate::k2_term::child_terms_match(
            arena.nodes@,
            fields@,
            seq![
                before[sentence as int].term@,
                Term::Comp(ckc_spec::v1text::ascii("clause_sha256"@), seq![Term::Atom(hex@)]),
                ckc_spec::engine::list_term(root_terms(before, children@)),
            ],
        );
    }
    crate::k2_term::push_comp(arena, slice_to_vec(clause_name), fields)
}

fn node(
    input: ETermArena,
    db: &Vec<EClause>,
    digests: &Vec<Vec<u8>>,
    log: &Vec<EEntry>,
    path: &Vec<usize>,
    base: usize,
) -> (out: (ETermArena, EMat))
    requires
        arena_ok(&input),
        db_valid(input.nodes@, db@),
        log_valid(input.nodes@, db.len() as nat, log@),
        base <= input.nodes.len(),
    ensures
        arena_ok(&out.0),
        input.nodes@.is_prefix_of(out.0.nodes@),
        valid(out.0.nodes@, &out.1),
        allocation(&out.1, input.nodes.len() as nat, out.0.nodes.len() as nat),
        view(out.0.nodes@, &out.1) == mat(
            db_view(input.nodes@, db@),
            digests_view(digests@),
            build(db_view(input.nodes@, db@), log_view(input.nodes@, log@), path_view(path@)),
            base as nat,
        ),
    decreases log.len() + 1 - path.len(), 0int,
{
    let ghost origin = input.nodes@;
    let ghost program = db_view(origin, db@);
    let ghost events = log_view(origin, log@);
    let mut arena = input;
    if path.len() > log.len() {
        let nil = crate::k2_term::push_nil(&mut arena);
        proof {
            reveal(build);
            reveal(mat);
        }
        return naf(arena, nil, base);
    }
    let found = event_at(&arena, log, path);
    match found {
        None => {
            let nil = crate::k2_term::push_nil(&mut arena);
            proof {
                reveal(build);
                reveal(mat);
            }
            naf(arena, nil, base)
        },
        Some(i) => {
            proof {
                assert(entry_valid(origin, db.len() as nat, &log@[i as int]));
                reveal(build);
            }
            match &log[i].event {
                EEvent::Naf(root) => {
                    proof {
                        reveal(mat);
                    }
                    naf(arena, *root, base)
                },
                EEvent::Clause(m) => {
                    proof {
                        assert(clause_valid(arena.nodes@, &db@[*m as int]));
                        assert(build(program, events, path_view(path@)) == PNode::Clause(
                            *m as nat,
                            build_all(
                                program,
                                events,
                                path_view(path@),
                                0,
                                program[*m as int].body.len(),
                            ),
                        ));
                        reveal(mat);
                    }
                    let sentence = match crate::k3_identity::identity_exec(&mut arena, &db[*m]) {
                        Err(e) => return (arena, EMat::Err(e)),
                        Ok(s) => s,
                    };
                    let ghost before_children = arena.nodes@;
                    let ghost sentence_model = arena@[sentence as int];
                    proof {
                        crate::k2_engine::db_models_prefix(origin, arena.nodes@, db@);
                        crate::k3_state::log_prefix(origin, arena.nodes@, db.len() as nat, log@);
                    }
                    let (next_arena, result) = children(
                        arena,
                        db,
                        digests,
                        log,
                        path,
                        db[*m].body.len(),
                        base,
                    );
                    arena = next_arena;
                    match result {
                        EMat::Err(e) => (arena, EMat::Err(e)),
                        EMat::Ok { roots: kids, base: next, nodes: count } => {
                            proof {
                                assert(root_ok(&arena, sentence));
                            }
                            let empty = Vec::new();
                            let digest = if *m < digests.len() {
                                &digests[*m]
                            } else {
                                &empty
                            };
                            let ghost kid_models = root_terms(arena.nodes@, kids@);
                            proof {
                                assert(digest@ == digest_at(digests_view(digests@), *m as nat));
                            }
                            let parent = clause_node(
                                &mut arena,
                                sentence,
                                digest.as_slice(),
                                &kids,
                            );
                            proof {
                                assert(count < arena.nodes.len());
                            }
                            let mut roots = Vec::new();
                            roots.push(parent);
                            proof {
                                assert_seqs_equal!(root_terms(arena.nodes@, roots@) == seq![arena@[parent as int]]);
                                assert(arena@[parent as int] == Term::Comp(
                                    ckc_spec::v1text::ascii("clause"@),
                                    seq![
                                        sentence_model,
                                        Term::Comp(
                                            ckc_spec::v1text::ascii("clause_sha256"@),
                                            seq![
                                                Term::Atom(
                                                    digest_at(digests_view(digests@), *m as nat),
                                                ),
                                            ],
                                        ),
                                        ckc_spec::engine::list_term(kid_models),
                                    ],
                                ));
                            }
                            (arena, EMat::Ok { roots, base: next, nodes: count + 1 })
                        },
                    }
                },
            }
        },
    }
}

fn children(
    input: ETermArena,
    db: &Vec<EClause>,
    digests: &Vec<Vec<u8>>,
    log: &Vec<EEntry>,
    path: &Vec<usize>,
    count: usize,
    base: usize,
) -> (out: (ETermArena, EMat))
    requires
        arena_ok(&input),
        db_valid(input.nodes@, db@),
        log_valid(input.nodes@, db.len() as nat, log@),
        base <= input.nodes.len(),
        path.len() <= log.len(),
    ensures
        arena_ok(&out.0),
        input.nodes@.is_prefix_of(out.0.nodes@),
        valid(out.0.nodes@, &out.1),
        allocation(&out.1, input.nodes.len() as nat, out.0.nodes.len() as nat),
        view(out.0.nodes@, &out.1) == mat_all(
            db_view(input.nodes@, db@),
            digests_view(digests@),
            build_all(
                db_view(input.nodes@, db@),
                log_view(input.nodes@, log@),
                path_view(path@),
                0,
                count as nat,
            ),
            base as nat,
        ),
    decreases log.len() - path.len(), 1int,
{
    hide(build);
    hide(mat);
    let ghost origin = input.nodes@;
    let ghost program = db_view(origin, db@);
    let ghost events = log_view(origin, log@);
    let ghost target = mat_all(
        program,
        digests_view(digests@),
        build_all(program, events, path_view(path@), 0, count as nat),
        base as nat,
    );
    let mut arena = input;
    let mut roots = Vec::new();
    let mut numbered = base;
    let mut total = 0usize;
    let mut i = 0usize;
    while i < count
        invariant
            origin == input.nodes@,
            arena_ok(&arena),
            origin.is_prefix_of(arena.nodes@),
            db_valid(arena.nodes@, db@),
            db_view(arena.nodes@, db@) == program,
            program == db_view(origin, db@),
            log_valid(arena.nodes@, db.len() as nat, log@),
            log_view(arena.nodes@, log@) == events,
            events == log_view(origin, log@),
            path.len() <= log.len(),
            i <= count,
            numbered <= arena.nodes.len(),
            roots_valid(arena.nodes@, roots@),
            total <= arena.nodes.len() - origin.len(),
            target == mat_all(
                program,
                digests_view(digests@),
                build_all(program, events, path_view(path@), 0, count as nat),
                base as nat,
            ),
            target == prepend(
                root_terms(arena.nodes@, roots@),
                total as nat,
                mat_all(
                    program,
                    digests_view(digests@),
                    build_all(program, events, path_view(path@), i as nat, count as nat),
                    numbered as nat,
                ),
            ),
        decreases count - i,
    {
        let ghost before = arena.nodes@;
        let ghost acc = root_terms(before, roots@);
        let ghost old_total = total as nat;
        let ghost old_base = numbered as nat;
        let mut child_path = path.clone();
        child_path.push(i);
        proof {
            assert_seqs_equal!(path_view(child_path@) == path_view(path@).push(i as nat));
            build_cons(program, events, path_view(path@), i as nat, count as nat);
            mat_cons(
                program,
                digests_view(digests@),
                build(program, events, path_view(child_path@)),
                build_all(program, events, path_view(path@), i as nat + 1, count as nat),
                old_base,
            );
        }
        let (next_arena, child) = node(arena, db, digests, log, &child_path, numbered);
        arena = next_arena;
        proof {
            crate::k2_engine::db_models_prefix(before, arena.nodes@, db@);
            crate::k3_state::log_prefix(before, arena.nodes@, db.len() as nat, log@);
            crate::k2_engine::roots_models_prefix(before, arena.nodes@, roots@);
            assert(origin.is_prefix_of(arena.nodes@));
        }
        match child {
            EMat::Err(e) => {
                proof {
                    assert(target == Mat::Err(e@));
                }
                return (arena, EMat::Err(e));
            },
            EMat::Ok { roots: mut added, base: next, nodes } => {
                let ghost left = roots@;
                let ghost right = added@;
                proof {
                    assert(total as nat + nodes as nat <= arena.nodes.len() - origin.len());
                    prepend_assoc(
                        acc,
                        old_total,
                        root_terms(arena.nodes@, right),
                        nodes as nat,
                        mat_all(
                            program,
                            digests_view(digests@),
                            build_all(
                                program,
                                events,
                                path_view(path@),
                                i as nat + 1,
                                count as nat,
                            ),
                            next as nat,
                        ),
                    );
                }
                total += nodes;
                roots.append(&mut added);
                numbered = next;
                proof {
                    assert_seqs_equal!(root_terms(arena.nodes@, roots@) == root_terms(arena.nodes@, left) + root_terms(arena.nodes@, right));
                    assert(roots_valid(arena.nodes@, roots@));
                }
                i += 1;
            },
        }
    }
    proof {
        reveal(build_all);
        reveal(mat_all);
    }
    (arena, EMat::Ok { roots, base: numbered, nodes: total })
}

proof fn build_all_shape(
    db: Seq<ckc_spec::v1text::DocClause>,
    log: Seq<(Seq<nat>, TEv)>,
    path: Seq<nat>,
    i: nat,
    n: nat,
)
    requires
        i <= n,
        path.len() <= log.len(),
    ensures
        build_all(db, log, path, i, n) == Seq::new(
            (n - i) as nat,
            |j: int| build(db, log, path.push((i + j) as nat)),
        ),
    decreases n - i,
{
    if i < n {
        build_all_shape(db, log, path, i + 1, n);
        reveal(build_all);
        assert_seqs_equal!(build_all(db, log, path, i, n) == Seq::new((n - i) as nat, |j: int| build(db, log, path.push((i + j) as nat))));
    } else {
        reveal(build_all);
    }
}

pub fn forest(
    arena: &mut ETermArena,
    db: &Vec<EClause>,
    digests: &Vec<Vec<u8>>,
    goals: &Vec<usize>,
    log: &Vec<EEntry>,
    base: usize,
) -> (out: EMat)
    requires
        arena_ok(old(arena)),
        db_valid(old(arena).nodes@, db@),
        log_valid(old(arena).nodes@, db.len() as nat, log@),
        roots_valid(old(arena).nodes@, goals@),
        base <= old(arena).nodes.len(),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid(final(arena).nodes@, &out),
        allocation(&out, old(arena).nodes.len() as nat, final(arena).nodes.len() as nat),
        view(final(arena).nodes@, &out) == mat_all(
            db_view(old(arena).nodes@, db@),
            digests_view(digests@),
            forest_of(
                db_view(old(arena).nodes@, db@),
                root_terms(old(arena).nodes@, goals@),
                log_view(old(arena).nodes@, log@),
            ),
            base as nat,
        ),
{
    let path = Vec::new();
    proof {
        build_all_shape(
            db_view(arena.nodes@, db@),
            log_view(arena.nodes@, log@),
            path_view(path@),
            0,
            goals.len() as nat,
        );
        assert_seqs_equal!(build_all(db_view(arena.nodes@, db@), log_view(arena.nodes@, log@), path_view(path@), 0, goals.len() as nat)
            == forest_of(db_view(arena.nodes@, db@), root_terms(arena.nodes@, goals@), log_view(arena.nodes@, log@)), i => {
                assert_seqs_equal!(path_view(path@).push(i as nat) == seq![i as nat]);
            });
    }
    let mut owned = crate::k2_reject::empty_arena();
    core::mem::swap(arena, &mut owned);
    let (mut owned, result) = children(owned, db, digests, log, &path, goals.len(), base);
    core::mem::swap(arena, &mut owned);
    result
}

} // verus!
