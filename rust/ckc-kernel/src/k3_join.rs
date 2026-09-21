#[cfg(verus_keep_ghost)]
use crate::k2_engine::{root_terms, roots_valid, term_size};
use crate::k2_term::{ENode, ENodeKind, ETermArena};
#[cfg(verus_keep_ghost)]
use crate::k2_term::{arena_ok, root_ok};
use crate::k3_coords::ECoord;
#[cfg(verus_keep_ghost)]
use crate::k3_coords::{coords_ok, coords_view};
use crate::k3_front::is_comp;
use ckc_spec::replay::EOut;
#[cfg(verus_keep_ghost)]
use ckc_spec::term::Term;
use ckc_spec::trace::*;
use ckc_spec::v1text::*;
use vstd::assert_seqs_equal;
use vstd::prelude::*;
use vstd::slice::slice_to_vec;

verus! {

pub enum EWhy {
    NodeShape,
    NonDemo,
    Join { sentence: usize, count: usize },
}

pub open spec fn why_valid(nodes: Seq<ENode>, why: &EWhy) -> bool {
    match why {
        EWhy::Join { sentence, .. } => *sentence < nodes.len(),
        _ => true,
    }
}

pub open spec fn why_view(nodes: Seq<ENode>, why: &EWhy) -> Term {
    match why {
        EWhy::NodeShape => ckc_spec::replay::atom("node_shape"@),
        EWhy::NonDemo => ckc_spec::replay::atom("non_demo"@),
        EWhy::Join { sentence, count } => Term::Comp(
            ascii("join"@),
            seq![nodes[*sentence as int].term@, Term::Int(*count as int)],
        ),
    }
}

pub enum EJoin {
    Ok(Vec<usize>),
    Bad(EWhy),
}

pub open spec fn join_view(nodes: Seq<ENode>, join: &EJoin) -> Join {
    match join {
        EJoin::Ok(counted) => Join::Ok(counted.len() as nat),
        EJoin::Bad(why) => Join::Bad(why_view(nodes, why)),
    }
}

pub open spec fn join_valid(nodes: Seq<ENode>, join: &EJoin) -> bool {
    match join {
        EJoin::Ok(_) => true,
        EJoin::Bad(why) => why_valid(nodes, why),
    }
}

pub open spec fn plus(a: Join, b: Join) -> Join {
    match a {
        Join::Bad(_) => a,
        Join::Ok(x) => match b {
            Join::Bad(_) => b,
            Join::Ok(y) => Join::Ok(x + y),
        },
    }
}

proof fn plus_assoc(a: Join, b: Join, c: Join)
    ensures
        plus(plus(a, b), c) == plus(a, plus(b, c)),
{
    match a {
        Join::Bad(_) => {},
        Join::Ok(_) => match b {
            Join::Bad(_) => {},
            Join::Ok(_) => match c {
                Join::Bad(_) => {},
                Join::Ok(_) => {},
            },
        },
    }
}

proof fn charge_node(n: nat, child: Join, rest: Join)
    ensures
        plus(Join::Ok(n), plus(plus(Join::Ok(1), child), rest)) == plus(
            Join::Ok(n + 1),
            plus(child, rest),
        ),
{
    match child {
        Join::Bad(_) => {},
        Join::Ok(_) => match rest {
            Join::Bad(_) => {},
            Join::Ok(_) => {},
        },
    }
}

proof fn size_three(name: Seq<u8>, a: Term, b: Term, c: Term)
    ensures
        term_size(Term::Comp(name, seq![a, b, c])) == 1 + term_size(a) + term_size(b) + term_size(
            c,
        ),
{
    assert_seqs_equal!(seq![a, b, c].drop_first() == seq![b, c]);
    assert_seqs_equal!(seq![b, c].drop_first() == seq![c]);
    assert_seqs_equal!(seq![c].drop_first() == Seq::empty());
    reveal_with_fuel(crate::k2_engine::term_size, 8);
    reveal_with_fuel(crate::k2_engine::terms_size, 8);
}

proof fn join_clause_law(
    cs: Seq<Coord>,
    ds: Seq<Seq<u8>>,
    sentence: Term,
    hex: Term,
    kids: Term,
    c: Coord,
    h: Seq<u8>,
)
    requires
        coord_of(sentence) == Some(c),
        hex_of(hex) == Some(h),
        join_count(cs, ds, c, h, 0) == 1,
    ensures
        node_join(cs, ds, Term::Comp(ascii("clause"@), seq![sentence, hex, kids])) == plus(
            Join::Ok(1),
            list_join(cs, ds, kids),
        ),
{
    reveal_with_fuel(node_join, 1);
    match list_join(cs, ds, kids) {
        Join::Bad(_) => {},
        Join::Ok(_) => {},
    }
}

proof fn size_two(name: Seq<u8>, a: Term, b: Term)
    ensures
        term_size(Term::Comp(name, seq![a, b])) == 1 + term_size(a) + term_size(b),
{
    assert_seqs_equal!(seq![a, b].drop_first() == seq![b]);
    assert_seqs_equal!(seq![b].drop_first() == Seq::empty());
    reveal_with_fuel(crate::k2_engine::term_size, 6);
    reveal_with_fuel(crate::k2_engine::terms_size, 6);
}

proof fn join_list_law(cs: Seq<Coord>, ds: Seq<Seq<u8>>, head: Term, tail: Term)
    ensures
        list_join(cs, ds, Term::Comp(cons_name(), seq![head, tail])) == plus(
            node_join(cs, ds, head),
            list_join(cs, ds, tail),
        ),
{
    reveal_with_fuel(list_join, 1);
    match node_join(cs, ds, head) {
        Join::Bad(_) => {},
        Join::Ok(_) => match list_join(cs, ds, tail) {
            Join::Bad(_) => {},
            Join::Ok(_) => {},
        },
    }
}

enum Task {
    Node(usize),
    List(usize),
}

spec fn task_valid(nodes: Seq<ENode>, task: &Task) -> bool {
    match task {
        Task::Node(root) | Task::List(root) => *root < nodes.len(),
    }
}

spec fn task_join(
    nodes: Seq<ENode>,
    coords: Seq<Coord>,
    digests: Seq<Seq<u8>>,
    task: &Task,
) -> Join {
    match task {
        Task::Node(root) => node_join(coords, digests, nodes[*root as int].term@),
        Task::List(root) => list_join(coords, digests, nodes[*root as int].term@),
    }
}

spec fn pending_join(
    nodes: Seq<ENode>,
    coords: Seq<Coord>,
    digests: Seq<Seq<u8>>,
    pending: Seq<Task>,
) -> Join
    decreases pending.len(),
{
    if pending.len() == 0 {
        Join::Ok(0)
    } else {
        plus(
            task_join(nodes, coords, digests, &pending.last()),
            pending_join(nodes, coords, digests, pending.drop_last()),
        )
    }
}

spec fn task_work(nodes: Seq<ENode>, task: &Task) -> nat {
    match task {
        Task::Node(root) => 2 * term_size(nodes[*root as int].term@),
        Task::List(root) => 2 * term_size(nodes[*root as int].term@) + 1,
    }
}

spec fn pending_work(nodes: Seq<ENode>, pending: Seq<Task>) -> nat
    decreases pending.len(),
{
    if pending.len() == 0 {
        0
    } else {
        task_work(nodes, &pending.last()) + pending_work(nodes, pending.drop_last())
    }
}

spec fn pending_valid(nodes: Seq<ENode>, pending: Seq<Task>) -> bool {
    forall|i: int| 0 <= i < pending.len() ==> #[trigger] task_valid(nodes, &pending[i])
}

proof fn pending_push(
    nodes: Seq<ENode>,
    coords: Seq<Coord>,
    digests: Seq<Seq<u8>>,
    pending: Seq<Task>,
    task: Task,
)
    requires
        pending_valid(nodes, pending),
        task_valid(nodes, &task),
    ensures
        pending_valid(nodes, pending.push(task)),
        pending_join(nodes, coords, digests, pending.push(task)) == plus(
            task_join(nodes, coords, digests, &task),
            pending_join(nodes, coords, digests, pending),
        ),
        pending_work(nodes, pending.push(task)) == task_work(nodes, &task) + pending_work(
            nodes,
            pending,
        ),
{
    assert_seqs_equal!(pending.push(task).drop_last() == pending);
    reveal_with_fuel(pending_join, 1);
    reveal_with_fuel(pending_work, 1);
    assert forall|i: int| 0 <= i < pending.push(task).len() implies #[trigger] task_valid(
        nodes,
        &pending.push(task)[i],
    ) by {
        if i < pending.len() {
            assert(pending.push(task)[i] == pending[i]);
        }
    }
}

pub fn node_join_exec(
    arena: &ETermArena,
    coords: &Vec<ECoord>,
    digests: &Vec<Vec<u8>>,
    root: usize,
) -> (out: EJoin)
    requires
        root_ok(arena, root),
        coords_ok(coords@),
    ensures
        join_valid(arena.nodes@, &out),
        join_view(arena.nodes@, &out) == node_join(
            coords_view(coords@),
            digests_view(digests@),
            arena@[root as int],
        ),
{
    hide(node_join);
    hide(list_join);
    hide(pending_join);
    hide(pending_work);
    let ghost cs = coords_view(coords@);
    let ghost ds = digests_view(digests@);
    let ghost model = arena@[root as int];
    let clause: &[u8] = b"clause";
    let naf: &[u8] = b"naf";
    let cons: &[u8] = b"[|]";
    proof {
        reveal_byteslit(b"clause");
        reveal_strlit("clause");
        reveal_byteslit(b"naf");
        reveal_strlit("naf");
        reveal_byteslit(b"[|]");
        reveal(ascii);
        assert(clause@ == ascii("clause"@));
        assert(naf@ == ascii("naf"@));
        assert(cons@ == cons_name());
    }
    let mut pending = Vec::new();
    let initial = Task::Node(root);
    proof {
        pending_push(arena.nodes@, cs, ds, pending@, initial);
        reveal_with_fuel(pending_join, 1);
    }
    pending.push(initial);
    // The tally retains one root per visited clause: its length is an executable natural.
    let mut counted: Vec<usize> = Vec::new();
    while pending.len() > 0
        invariant
            root_ok(arena, root),
            coords_ok(coords@),
            cs == coords_view(coords@),
            ds == digests_view(digests@),
            model == arena@[root as int],
            pending_valid(arena.nodes@, pending@),
            node_join(cs, ds, model) == plus(
                Join::Ok(counted.len() as nat),
                pending_join(arena.nodes@, cs, ds, pending@),
            ),
            clause@ == ascii("clause"@),
            naf@ == ascii("naf"@),
            cons@ == cons_name(),
        decreases pending_work(arena.nodes@, pending@),
    {
        let ghost before = pending@;
        let task = pending.pop().unwrap();
        proof {
            assert(task == before.last());
            assert_seqs_equal!(pending@ == before.drop_last());
            assert(task_valid(arena.nodes@, &task));
            assert(task_work(arena.nodes@, &task) > 0) by {
                reveal_with_fuel(crate::k2_engine::term_size, 1);
                match task {
                    Task::Node(_) => {},
                    Task::List(_) => {},
                }
            }
            reveal_with_fuel(pending_join, 1);
            reveal_with_fuel(pending_work, 1);
            plus_assoc(
                Join::Ok(counted.len() as nat),
                task_join(arena.nodes@, cs, ds, &task),
                pending_join(arena.nodes@, cs, ds, pending@),
            );
        }
        match task {
            Task::Node(n) => {
                proof {
                    reveal_with_fuel(node_join, 1);
                }
                if is_comp(arena, n, clause, 3) {
                    let args = crate::k2_engine::args_roots(arena, n);
                    proof {
                        assert(args.len() == 3);
                        assert(arena@[n as int] == Term::Comp(
                            clause@,
                            ckc_spec::engine::args_of(arena@[n as int]),
                        ));
                        assert_seqs_equal!(ckc_spec::engine::args_of(arena@[n as int]) == seq![arena@[args@[0] as int], arena@[args@[1] as int], arena@[args@[2] as int]]);
                    }
                    let c = match crate::k3_coords::coord_of_exec(arena, args[0]) {
                        Some(c) => c,
                        None => return EJoin::Bad(EWhy::NodeShape),
                    };
                    let h = match crate::k3_coords::hex_of_exec(arena, args[1]) {
                        Some(h) => h,
                        None => return EJoin::Bad(EWhy::NodeShape),
                    };
                    let count = crate::k3_coords::join_count_exec(coords, digests, &c, &h);
                    if count != 1 {
                        return EJoin::Bad(EWhy::Join { sentence: args[0], count });
                    }
                    let next = Task::List(args[2]);
                    proof {
                        pending_push(arena.nodes@, cs, ds, pending@, next);
                        reveal_with_fuel(crate::k2_engine::term_size, 1);
                        reveal_with_fuel(crate::k2_engine::terms_size, 4);
                        reveal(task_work);
                        size_three(
                            clause@,
                            arena@[args@[0] as int],
                            arena@[args@[1] as int],
                            arena@[args@[2] as int],
                        );
                        assert(term_size(arena@[n as int]) == 1 + term_size(arena@[args@[0] as int])
                            + term_size(arena@[args@[1] as int]) + term_size(
                            arena@[args@[2] as int],
                        ));
                        assert(task_work(arena.nodes@, &next) < task_work(arena.nodes@, &task));
                        let child = list_join(cs, ds, arena@[args@[2] as int]);
                        join_clause_law(
                            cs,
                            ds,
                            arena@[args@[0] as int],
                            arena@[args@[1] as int],
                            arena@[args@[2] as int],
                            c.model@,
                            h@,
                        );
                        assert(node_join(cs, ds, arena@[n as int]) == plus(Join::Ok(1), child)) by {
                            reveal_with_fuel(node_join, 1);
                            assert(coord_of(arena@[args@[0] as int]) == Some(c.model@));
                            assert(hex_of(arena@[args@[1] as int]) == Some(h@));
                            assert(join_count(cs, ds, c.model@, h@, 0) == 1);
                            match child {
                                Join::Bad(_) => {},
                                Join::Ok(_) => {},
                            }
                        }
                        charge_node(
                            counted.len() as nat,
                            child,
                            pending_join(arena.nodes@, cs, ds, pending@),
                        );
                    }
                    pending.push(next);
                    counted.push(n);
                    proof {
                        assert(node_join(cs, ds, model) == plus(
                            Join::Ok(counted.len() as nat),
                            pending_join(arena.nodes@, cs, ds, pending@),
                        ));
                    }
                } else if !is_comp(arena, n, naf, 1) {
                    return EJoin::Bad(EWhy::NodeShape);
                }
            },
            Task::List(n) => {
                proof {
                    reveal_with_fuel(list_join, 1);
                    assert(crate::k2_term::node_ok(arena.nodes@, n as int));
                }
                if matches!(&arena.nodes[n].kind, ENodeKind::Nil) {
                    continue;
                }
                if !is_comp(arena, n, cons, 2) {
                    return EJoin::Bad(EWhy::NodeShape);
                }
                let args = crate::k2_engine::args_roots(arena, n);
                proof {
                    assert(args.len() == 2);
                    assert(arena@[n as int] == Term::Comp(
                        cons@,
                        ckc_spec::engine::args_of(arena@[n as int]),
                    ));
                    assert_seqs_equal!(ckc_spec::engine::args_of(arena@[n as int]) == seq![arena@[args@[0] as int], arena@[args@[1] as int]]);
                }
                let tail = Task::List(args[1]);
                let head = Task::Node(args[0]);
                proof {
                    pending_push(arena.nodes@, cs, ds, pending@, tail);
                }
                pending.push(tail);
                proof {
                    pending_push(arena.nodes@, cs, ds, pending@, head);
                    plus_assoc(
                        task_join(arena.nodes@, cs, ds, &head),
                        task_join(arena.nodes@, cs, ds, &tail),
                        pending_join(arena.nodes@, cs, ds, before.drop_last()),
                    );
                    reveal_with_fuel(crate::k2_engine::term_size, 1);
                    reveal_with_fuel(crate::k2_engine::terms_size, 3);
                    size_two(cons@, arena@[args@[0] as int], arena@[args@[1] as int]);
                    join_list_law(cs, ds, arena@[args@[0] as int], arena@[args@[1] as int]);
                    assert(task_work(arena.nodes@, &head) + task_work(arena.nodes@, &tail)
                        < task_work(arena.nodes@, &task));
                }
                pending.push(head);
                proof {
                    assert(node_join(cs, ds, model) == plus(
                        Join::Ok(counted.len() as nat),
                        pending_join(arena.nodes@, cs, ds, pending@),
                    ));
                }
            },
        }
    }
    proof {
        reveal_with_fuel(pending_join, 1);
    }
    EJoin::Ok(counted)
}

pub fn proved_nodes_exec(arena: &ETermArena, root: usize) -> (out: Option<Vec<usize>>)
    requires
        root_ok(arena, root),
    ensures
        out matches Some(nodes) ==> roots_valid(arena.nodes@, nodes@),
        crate::k2_walk::list_view(arena.nodes@, out) == proved_nodes(arena@[root as int]),
{
    let name: &[u8] = b"proved";
    proof {
        reveal_byteslit(b"proved");
        reveal_strlit("proved");
        reveal(ascii);
        assert(name@ == ascii("proved"@));
    }
    if !is_comp(arena, root, name, 1) {
        return None;
    }
    let args = crate::k2_engine::args_roots(arena, root);
    match crate::k2_walk::list_items_exec(arena, args[0]) {
        Some(nodes) => if nodes.len() == 0 {
            None
        } else {
            Some(nodes)
        },
        None => None,
    }
}

pub fn roots_join_exec(
    arena: &ETermArena,
    coords: &Vec<ECoord>,
    digests: &Vec<Vec<u8>>,
    nodes: &Vec<usize>,
) -> (out: EJoin)
    requires
        arena_ok(arena),
        roots_valid(arena.nodes@, nodes@),
        coords_ok(coords@),
    ensures
        join_valid(arena.nodes@, &out),
        join_view(arena.nodes@, &out) == roots_join(
            coords_view(coords@),
            digests_view(digests@),
            root_terms(arena.nodes@, nodes@),
        ),
{
    hide(node_join);
    hide(roots_join);
    let ghost cs = coords_view(coords@);
    let ghost ds = digests_view(digests@);
    let ghost models = root_terms(arena.nodes@, nodes@);
    let clause: &[u8] = b"clause";
    proof {
        reveal_byteslit(b"clause");
        reveal_strlit("clause");
        reveal(ascii);
        assert(clause@ == ascii("clause"@));
        assert_seqs_equal!(models.skip(0) == models);
    }
    let mut counted: Vec<usize> = Vec::new();
    let mut i = 0usize;
    while i < nodes.len()
        invariant
            arena_ok(arena),
            roots_valid(arena.nodes@, nodes@),
            coords_ok(coords@),
            i <= nodes.len(),
            cs == coords_view(coords@),
            ds == digests_view(digests@),
            models == root_terms(arena.nodes@, nodes@),
            clause@ == ascii("clause"@),
            roots_join(cs, ds, models) == plus(
                Join::Ok(counted.len() as nat),
                roots_join(cs, ds, models.skip(i as int)),
            ),
        decreases nodes.len() - i,
    {
        proof {
            reveal_with_fuel(roots_join, 1);
            assert_seqs_equal!(models.skip(i as int).drop_first() == models.skip(i as int + 1));
        }
        if !is_comp(arena, nodes[i], clause, 3) {
            return EJoin::Bad(EWhy::NodeShape);
        }
        match node_join_exec(arena, coords, digests, nodes[i]) {
            EJoin::Bad(why) => return EJoin::Bad(why),
            EJoin::Ok(mut next) => {
                proof {
                    plus_assoc(
                        Join::Ok(counted.len() as nat),
                        Join::Ok(next.len() as nat),
                        roots_join(cs, ds, models.skip(i as int + 1)),
                    );
                }
                counted.append(&mut next);
            },
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(models.skip(i as int) == Seq::empty());
        reveal_with_fuel(roots_join, 1);
    }
    EJoin::Ok(counted)
}

pub fn rows_join_exec(
    arena: &ETermArena,
    coords: &Vec<ECoord>,
    digests: &Vec<Vec<u8>>,
    rows: &Vec<usize>,
) -> (out: EJoin)
    requires
        arena_ok(arena),
        roots_valid(arena.nodes@, rows@),
        coords_ok(coords@),
    ensures
        join_valid(arena.nodes@, &out),
        join_view(arena.nodes@, &out) == rows_join(
            coords_view(coords@),
            digests_view(digests@),
            root_terms(arena.nodes@, rows@),
        ),
{
    hide(roots_join);
    hide(rows_join);
    let ghost cs = coords_view(coords@);
    let ghost ds = digests_view(digests@);
    let ghost models = root_terms(arena.nodes@, rows@);
    proof {
        assert_seqs_equal!(models.skip(0) == models);
    }
    let mut counted: Vec<usize> = Vec::new();
    let mut i = 0usize;
    while i < rows.len()
        invariant
            arena_ok(arena),
            roots_valid(arena.nodes@, rows@),
            coords_ok(coords@),
            i <= rows.len(),
            cs == coords_view(coords@),
            ds == digests_view(digests@),
            models == root_terms(arena.nodes@, rows@),
            rows_join(cs, ds, models) == plus(
                Join::Ok(counted.len() as nat),
                rows_join(cs, ds, models.skip(i as int)),
            ),
        decreases rows.len() - i,
    {
        proof {
            reveal_with_fuel(rows_join, 1);
            assert_seqs_equal!(models.skip(i as int).drop_first() == models.skip(i as int + 1));
        }
        let args = crate::k2_engine::args_roots(arena, rows[i]);
        if args.len() <= 1 {
            return EJoin::Bad(EWhy::NonDemo);
        }
        let nodes = match proved_nodes_exec(arena, args[1]) {
            Some(nodes) => nodes,
            None => return EJoin::Bad(EWhy::NonDemo),
        };
        match roots_join_exec(arena, coords, digests, &nodes) {
            EJoin::Bad(why) => return EJoin::Bad(why),
            EJoin::Ok(mut next) => {
                proof {
                    plus_assoc(
                        Join::Ok(counted.len() as nat),
                        Join::Ok(next.len() as nat),
                        rows_join(cs, ds, models.skip(i as int + 1)),
                    );
                }
                counted.append(&mut next);
            },
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(models.skip(i as int) == Seq::empty());
        reveal_with_fuel(rows_join, 1);
    }
    EJoin::Ok(counted)
}

pub fn result_join_exec(
    arena: &ETermArena,
    coords: &Vec<ECoord>,
    digests: &Vec<Vec<u8>>,
    root: usize,
) -> (out: EJoin)
    requires
        root_ok(arena, root),
        coords_ok(coords@),
    ensures
        join_valid(arena.nodes@, &out),
        join_view(arena.nodes@, &out) == result_join(
            coords_view(coords@),
            digests_view(digests@),
            arena@[root as int],
        ),
{
    hide(roots_join);
    hide(rows_join);
    let yes: &[u8] = b"yes";
    let sols: &[u8] = b"solutions";
    proof {
        reveal_byteslit(b"yes");
        reveal_strlit("yes");
        reveal_byteslit(b"solutions");
        reveal_strlit("solutions");
        reveal(ascii);
        assert(yes@ == ascii("yes"@));
        assert(sols@ == ascii("solutions"@));
    }
    if is_comp(arena, root, yes, 1) {
        let args = crate::k2_engine::args_roots(arena, root);
        let nodes = match proved_nodes_exec(arena, args[0]) {
            Some(nodes) => nodes,
            None => return EJoin::Bad(EWhy::NonDemo),
        };
        roots_join_exec(arena, coords, digests, &nodes)
    } else if is_comp(arena, root, sols, 1) {
        let args = crate::k2_engine::args_roots(arena, root);
        let rows = match crate::k2_walk::list_items_exec(arena, args[0]) {
            Some(rows) => rows,
            None => return EJoin::Bad(EWhy::NonDemo),
        };
        if rows.len() == 0 {
            return EJoin::Bad(EWhy::NonDemo);
        }
        rows_join_exec(arena, coords, digests, &rows)
    } else {
        EJoin::Bad(EWhy::NonDemo)
    }
}

pub fn tc_fail_exec(arena: &mut ETermArena, why: EWhy) -> (out: EOut)
    requires
        arena_ok(old(arena)),
        why_valid(old(arena).nodes@, &why),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out@ == tc_fail(why_view(old(arena).nodes@, &why)),
{
    let ghost before = arena.nodes@;
    let root = match why {
        EWhy::NodeShape => {
            let name: &[u8] = b"node_shape";
            proof {
                reveal_byteslit(b"node_shape");
                reveal_strlit("node_shape");
                reveal(ascii);
                assert(name@ == ascii("node_shape"@));
            }
            crate::k2_output::atom_root(arena, name)
        },
        EWhy::NonDemo => {
            let name: &[u8] = b"non_demo";
            proof {
                reveal_byteslit(b"non_demo");
                reveal_strlit("non_demo");
                reveal(ascii);
                assert(name@ == ascii("non_demo"@));
            }
            crate::k2_output::atom_root(arena, name)
        },
        EWhy::Join { sentence, count } => {
            let name: &[u8] = b"join";
            proof {
                reveal_byteslit(b"join");
                reveal_strlit("join");
                reveal(ascii);
                assert(name@ == ascii("join"@));
            }
            let count = crate::k2_output::int_root(arena, count);
            crate::k2_output::comp2(arena, name, sentence, count)
        },
    };
    let name: &[u8] = b"trace_check";
    proof {
        reveal_byteslit(b"trace_check");
        reveal_strlit("trace_check");
        reveal(ascii);
        assert(name@ == ascii("trace_check"@));
    }
    let detail = crate::k2_output::comp1(arena, name, root);
    crate::k2_output::error_out(arena, detail, true)
}

pub fn tc_meter_exec(qid: &[u8], count: usize) -> (out: EOut)
    ensures
        out@ == ckc_spec::replay::ok(tc_meter(qid@, count as nat)),
{
    let prefix: &[u8] = b"ckc: trace-check ok ";
    let glue: &[u8] = b" nodes=";
    proof {
        reveal_byteslit(b"ckc: trace-check ok ");
        reveal_strlit("ckc: trace-check ok ");
        reveal_byteslit(b" nodes=");
        reveal_strlit(" nodes=");
        reveal(ascii);
        assert(prefix@ == ascii("ckc: trace-check ok "@));
        assert(glue@ == ascii(" nodes="@));
    }
    let mut out = slice_to_vec(prefix);
    let mut id = slice_to_vec(qid);
    out.append(&mut id);
    let mut middle = slice_to_vec(glue);
    out.append(&mut middle);
    let mut count_bytes = crate::k2_manifest::udec_vec(count);
    out.append(&mut count_bytes);
    out.push(0x0a);
    proof {
        assert_seqs_equal!(out@ == tc_meter(qid@, count as nat));
    }
    EOut { rc: 0, out, err: Vec::new() }
}

} // verus!
