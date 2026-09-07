use ckc_spec::term::Term;
#[cfg(verus_keep_ghost)]
use vstd::arithmetic::div_mod::lemma_fundamental_div_mod;
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub enum ECompForm {
    Cons,
    Curly,
    Regular,
}

pub enum ENodeKind {
    Var { key: usize, spelling: Vec<u8>, value: Ghost<nat> },
    Int { spelling: Vec<u8>, magnitude: Vec<u8>, negative: bool, value: Ghost<int> },
    Nil,
    Atom { name: Vec<u8> },
    Comp { name: Vec<u8>, child_roots: Vec<usize>, form: ECompForm },
}

pub struct ENode {
    pub kind: ENodeKind,
    pub term: Ghost<Term>,
}

pub struct ETermArena {
    pub nodes: Vec<ENode>,
}

impl View for ETermArena {
    type V = Seq<Term>;

    open spec fn view(&self) -> Seq<Term> {
        self.nodes@.map_values(|n: ENode| n.term@)
    }
}

pub open spec fn child_terms(nodes: Seq<ENode>, roots: Seq<usize>) -> Seq<Term> {
    Seq::new(roots.len(), |i: int| nodes[roots[i] as int].term@)
}

pub open spec fn child_roots_before(roots: Seq<usize>, parent: int) -> bool {
    forall|j: int| 0 <= j < roots.len() ==> ((#[trigger] roots[j]) as int) < parent
}

pub open spec fn child_roots_valid(nodes: Seq<ENode>, roots: Seq<usize>) -> bool {
    forall|j: int| 0 <= j < roots.len() ==> #[trigger] roots[j] < nodes.len()
}

pub open spec fn comp_form_ok(form: &ECompForm, name: Seq<u8>, arity: nat) -> bool {
    match form {
        ECompForm::Cons => name == ckc_spec::v1text::cons_name() && arity == 2,
        ECompForm::Curly => name == ckc_spec::v1text::curly_name() && arity == 1,
        ECompForm::Regular => {
            !(name == ckc_spec::v1text::cons_name() && arity == 2) && !(name
                == ckc_spec::v1text::curly_name() && arity == 1)
        },
    }
}

pub open spec fn node_ok(nodes: Seq<ENode>, i: int) -> bool {
    &&& 0 <= i < nodes.len()
    &&& ckc_spec::term::wf_term(nodes[i].term@)
    &&& match (&nodes[i].kind, nodes[i].term@) {
        (ENodeKind::Var { key, spelling, value }, Term::Var(k)) => {
            &&& *key as nat == k
            &&& value@ == k
            &&& spelling@ == ckc_spec::v1text::var_bytes(k)
        },
        (ENodeKind::Int { spelling, magnitude, negative, value }, Term::Int(n)) => {
            &&& value@ == n
            &&& *negative == (n < 0)
            &&& spelling@ == ckc_spec::v1text::dec_bytes(n)
            &&& magnitude@ == ckc_spec::v1text::udec_bytes(
                if n < 0 {
                    (-n) as nat
                } else {
                    n as nat
                },
            )
        },
        (ENodeKind::Nil, Term::Nil) => true,
        (ENodeKind::Atom { name }, Term::Atom(spec_name)) => name@ == spec_name,
        (ENodeKind::Comp { name, child_roots, form }, Term::Comp(spec_name, args)) => {
            &&& name@ == spec_name
            &&& child_roots@.len() == args.len()
            &&& child_roots_before(child_roots@, i)
            &&& child_roots_valid(nodes, child_roots@)
            &&& child_terms(nodes, child_roots@) == args
            &&& comp_form_ok(form, spec_name, args.len())
        },
        _ => false,
    }
}

pub open spec fn arena_ok(arena: &ETermArena) -> bool {
    forall|i: int| 0 <= i < arena.nodes@.len() ==> node_ok(arena.nodes@, i)
}

pub open spec fn root_ok(arena: &ETermArena, root: usize) -> bool {
    arena_ok(arena) && root < arena.nodes@.len()
}

pub enum EPrintTask {
    Term { index: usize, model: Ghost<Term> },
    Args { parent: usize, next: usize, model: Ghost<Seq<Term>> },
    Tail { index: usize, model: Ghost<Term> },
    Byte(u8),
}

pub open spec fn task_ok(nodes: Seq<ENode>, task: &EPrintTask) -> bool {
    match task {
        EPrintTask::Term { index, model } | EPrintTask::Tail { index, model } => {
            &&& *index < nodes.len()
            &&& model@ == nodes[*index as int].term@
        },
        EPrintTask::Args { parent, next, model } => {
            &&& *parent < nodes.len()
            &&& match &nodes[*parent as int].kind {
                ENodeKind::Comp { child_roots, .. } => {
                    &&& *next < child_roots@.len()
                    &&& child_roots_valid(nodes, child_roots@)
                    &&& model@ == child_terms(nodes, child_roots@.skip(*next as int))
                },
                _ => false,
            }
        },
        EPrintTask::Byte(_) => true,
    }
}

pub open spec fn tasks_ok(nodes: Seq<ENode>, tasks: Seq<EPrintTask>) -> bool
    decreases tasks.len(),
{
    tasks.len() == 0 || (task_ok(nodes, &tasks.last()) && tasks_ok(nodes, tasks.drop_last()))
}

pub open spec fn task_bytes(task: &EPrintTask) -> Seq<u8> {
    match task {
        EPrintTask::Term { model, .. } => ckc_spec::v1text::term_bytes(model@),
        EPrintTask::Args { model, .. } => ckc_spec::v1text::args_bytes(model@),
        EPrintTask::Tail { model, .. } => ckc_spec::v1text::tail_bytes(model@),
        EPrintTask::Byte(b) => seq![*b],
    }
}

pub open spec fn tasks_bytes(tasks: Seq<EPrintTask>) -> Seq<u8>
    decreases tasks.len(),
{
    if tasks.len() == 0 {
        Seq::empty()
    } else {
        task_bytes(&tasks.last()) + tasks_bytes(tasks.drop_last())
    }
}

pub open spec fn term_work(t: Term) -> nat
    decreases t, 0int,
{
    match t {
        Term::Comp(name, args) => if name == ckc_spec::v1text::cons_name() && args.len() == 2 {
            2 + term_work(args[0]) + tail_work(args[1])
        } else if name == ckc_spec::v1text::curly_name() && args.len() == 1 {
            2 + term_work(args[0])
        } else {
            2 + args_work(args)
        },
        _ => 1,
    }
}

pub open spec fn args_work(args: Seq<Term>) -> nat
    decreases args, 0int,
{
    if args.len() == 0 {
        0
    } else if args.len() == 1 {
        1 + term_work(args[0])
    } else {
        2 + term_work(args[0]) + args_work(args.drop_first())
    }
}

pub open spec fn tail_work(t: Term) -> nat
    decreases t, 1int,
{
    match t {
        Term::Nil => 1,
        Term::Comp(name, args) => if name == ckc_spec::v1text::cons_name() && args.len() == 2 {
            2 + term_work(args[0]) + tail_work(args[1])
        } else {
            2 + term_work(t)
        },
        _ => 2 + term_work(t),
    }
}

pub open spec fn task_work(task: &EPrintTask) -> nat {
    match task {
        EPrintTask::Term { model, .. } => term_work(model@),
        EPrintTask::Args { model, .. } => args_work(model@),
        EPrintTask::Tail { model, .. } => tail_work(model@),
        EPrintTask::Byte(_) => 1,
    }
}

pub open spec fn tasks_work(tasks: Seq<EPrintTask>) -> nat
    decreases tasks.len(),
{
    if tasks.len() == 0 {
        0
    } else {
        task_work(&tasks.last()) + tasks_work(tasks.drop_last())
    }
}

proof fn child_terms_step(nodes: Seq<ENode>, roots: Seq<usize>)
    requires
        roots.len() > 0,
        roots[0] < nodes.len(),
    ensures
        child_terms(nodes, roots) == seq![nodes[roots[0] as int].term@] + child_terms(
            nodes,
            roots.drop_first(),
        ),
{
    reveal(child_terms);
    assert_seqs_equal!(
        child_terms(nodes, roots)
            == seq![nodes[roots[0] as int].term@] + child_terms(nodes, roots.drop_first())
    );
}

proof fn child_terms_skip_step(nodes: Seq<ENode>, roots: Seq<usize>, next: usize)
    requires
        next < roots.len(),
        roots[next as int] < nodes.len(),
    ensures
        child_terms(nodes, roots.skip(next as int)) == seq![nodes[roots[next as int] as int].term@]
            + child_terms(nodes, roots.skip(next as int + 1)),
{
    child_terms_step(nodes, roots.skip(next as int));
    assert(roots.skip(next as int)[0] == roots[next as int]);
    assert_seqs_equal!(roots.skip(next as int).drop_first()
        == roots.skip(next as int + 1));
}

proof fn tasks_ok_push(nodes: Seq<ENode>, tasks: Seq<EPrintTask>, task: EPrintTask)
    requires
        tasks_ok(nodes, tasks),
        task_ok(nodes, &task),
    ensures
        tasks_ok(nodes, tasks.push(task)),
{
    assert(tasks.push(task).last() == task);
    assert(tasks.push(task).drop_last() == tasks);
    reveal_with_fuel(tasks_ok, 2);
}

proof fn tasks_bytes_push(tasks: Seq<EPrintTask>, task: EPrintTask)
    ensures
        tasks_bytes(tasks.push(task)) == task_bytes(&task) + tasks_bytes(tasks),
{
    assert(tasks.push(task).last() == task);
    assert(tasks.push(task).drop_last() == tasks);
    reveal_with_fuel(tasks_bytes, 2);
}

proof fn tasks_work_push(tasks: Seq<EPrintTask>, task: EPrintTask)
    ensures
        tasks_work(tasks.push(task)) == task_work(&task) + tasks_work(tasks),
{
    assert(tasks.push(task).last() == task);
    assert(tasks.push(task).drop_last() == tasks);
    reveal_with_fuel(tasks_work, 2);
}

pub proof fn child_terms_match(nodes: Seq<ENode>, roots: Seq<usize>, terms: Seq<Term>)
    requires
        roots.len() == terms.len(),
        forall|i: int|
            0 <= i < roots.len() ==> {
                &&& roots[i] < nodes.len()
                &&& nodes[roots[i] as int].term@ == terms[i]
            },
    ensures
        child_terms(nodes, roots) == terms,
{
    reveal(child_terms);
    assert_seqs_equal!(child_terms(nodes, roots) == terms);
}

proof fn child_terms_prefix_stable(before: Seq<ENode>, after: Seq<ENode>, roots: Seq<usize>)
    requires
        before.is_prefix_of(after),
        forall|j: int| 0 <= j < roots.len() ==> #[trigger] roots[j] < before.len(),
    ensures
        child_terms(before, roots) == child_terms(after, roots),
{
    reveal(child_terms);
    assert_seqs_equal!(child_terms(before, roots) == child_terms(after, roots));
}

proof fn node_ok_prefix_stable(before: Seq<ENode>, after: Seq<ENode>, i: int)
    requires
        before.is_prefix_of(after),
        0 <= i < before.len(),
        node_ok(before, i),
    ensures
        node_ok(after, i),
{
    assert(before[i] == after[i]);
    reveal(node_ok);
    match &before[i].kind {
        ENodeKind::Comp { child_roots, .. } => {
            assert forall|j: int| 0 <= j < child_roots@.len() implies #[trigger] child_roots@[j]
                < before.len() by {
                assert((child_roots@[j] as int) < i);
            }
            child_terms_prefix_stable(before, after, child_roots@);
        },
        _ => {},
    }
}

proof fn arena_ok_push(nodes: Seq<ENode>, node: ENode)
    requires
        forall|i: int| 0 <= i < nodes.len() ==> node_ok(nodes, i),
        node_ok(nodes.push(node), nodes.len() as int),
    ensures
        forall|i: int| 0 <= i < nodes.push(node).len() ==> node_ok(nodes.push(node), i),
{
    assert(nodes.is_prefix_of(nodes.push(node)));
    assert forall|i: int| 0 <= i < nodes.push(node).len() implies node_ok(nodes.push(node), i) by {
        if i < nodes.len() {
            node_ok_prefix_stable(nodes, nodes.push(node), i);
        } else {
            assert(i == nodes.len());
        }
    }
}

proof fn wf_terms_from_all(terms: Seq<Term>)
    requires
        forall|i: int| 0 <= i < terms.len() ==> ckc_spec::term::wf_term(#[trigger] terms[i]),
    ensures
        ckc_spec::term::wf_terms(terms),
    decreases terms.len(),
{
    if terms.len() > 0 {
        assert forall|i: int| 0 <= i < terms.drop_first().len() implies ckc_spec::term::wf_term(
            #[trigger] terms.drop_first()[i],
        ) by {
            assert(terms.drop_first()[i] == terms[i + 1]);
        }
        wf_terms_from_all(terms.drop_first());
        reveal_with_fuel(ckc_spec::term::wf_terms, 2);
    } else {
        reveal(ckc_spec::term::wf_terms);
    }
}

proof fn child_terms_wf(nodes: Seq<ENode>, roots: Seq<usize>)
    requires
        forall|j: int|
            0 <= j < roots.len() ==> {
                &&& roots[j] < nodes.len()
                &&& node_ok(nodes, roots[j] as int)
            },
    ensures
        ckc_spec::term::wf_terms(child_terms(nodes, roots)),
{
    assert forall|i: int| 0 <= i < child_terms(nodes, roots).len() implies ckc_spec::term::wf_term(
        #[trigger] child_terms(nodes, roots)[i],
    ) by {
        reveal(child_terms);
        assert(node_ok(nodes, roots[i] as int));
        reveal(node_ok);
    }
    wf_terms_from_all(child_terms(nodes, roots));
}

pub proof fn arena_prefix_stable(before: Seq<ENode>, after: &ETermArena)
    requires
        before.is_prefix_of(after.nodes@),
    ensures
        forall|i: int| 0 <= i < before.len() ==> before[i].term@ == after@[i],
{
    assert forall|i: int| 0 <= i < before.len() implies before[i].term@ == after@[i] by {
        assert(before[i] == after.nodes@[i]);
    }
}

fn push_node(arena: &mut ETermArena, node: ENode) -> (root: usize)
    requires
        arena_ok(old(arena)),
        node_ok(old(arena).nodes@.push(node), old(arena).nodes@.len() as int),
    ensures
        arena_ok(final(arena)),
        root == old(arena).nodes@.len(),
        final(arena).nodes@ == old(arena).nodes@.push(node),
        final(arena)@[root as int] == node.term@,
{
    let root = arena.nodes.len();
    proof {
        reveal(arena_ok);
        arena_ok_push(arena.nodes@, node);
    }
    arena.nodes.push(node);
    root
}

pub fn push_var(arena: &mut ETermArena, key: usize, spelling: Vec<u8>) -> (root: usize)
    requires
        arena_ok(old(arena)),
        spelling@ == ckc_spec::v1text::var_bytes(key as nat),
    ensures
        arena_ok(final(arena)),
        root == old(arena).nodes@.len(),
        final(arena).nodes@ == old(arena).nodes@.push(final(arena).nodes@[root as int]),
        final(arena)@[root as int] == Term::Var(key as nat),
{
    let node = ENode {
        kind: ENodeKind::Var { key, spelling, value: Ghost(key as nat) },
        term: Ghost(Term::Var(key as nat)),
    };
    proof {
        reveal(node_ok);
        reveal(ckc_spec::term::wf_term);
    }
    push_node(arena, node)
}

pub fn push_int(
    arena: &mut ETermArena,
    spelling: Vec<u8>,
    magnitude: Vec<u8>,
    negative: bool,
    value: Ghost<int>,
) -> (root: usize)
    requires
        arena_ok(old(arena)),
        negative == (value@ < 0),
        spelling@ == ckc_spec::v1text::dec_bytes(value@),
        magnitude@ == ckc_spec::v1text::udec_bytes(
            if value@ < 0 {
                (-value@) as nat
            } else {
                value@ as nat
            },
        ),
    ensures
        arena_ok(final(arena)),
        root == old(arena).nodes@.len(),
        final(arena).nodes@ == old(arena).nodes@.push(final(arena).nodes@[root as int]),
        final(arena)@[root as int] == Term::Int(value@),
{
    let node = ENode {
        kind: ENodeKind::Int { spelling, magnitude, negative, value },
        term: Ghost(Term::Int(value@)),
    };
    proof {
        reveal(node_ok);
        reveal(ckc_spec::term::wf_term);
    }
    push_node(arena, node)
}

pub fn push_nil(arena: &mut ETermArena) -> (root: usize)
    requires
        arena_ok(old(arena)),
    ensures
        arena_ok(final(arena)),
        root == old(arena).nodes@.len(),
        final(arena).nodes@ == old(arena).nodes@.push(final(arena).nodes@[root as int]),
        final(arena)@[root as int] == Term::Nil,
{
    let node = ENode { kind: ENodeKind::Nil, term: Ghost(Term::Nil) };
    proof {
        reveal(node_ok);
        reveal(ckc_spec::term::wf_term);
    }
    push_node(arena, node)
}

pub fn push_atom(arena: &mut ETermArena, name: Vec<u8>) -> (root: usize)
    requires
        arena_ok(old(arena)),
    ensures
        arena_ok(final(arena)),
        root == old(arena).nodes@.len(),
        final(arena).nodes@ == old(arena).nodes@.push(final(arena).nodes@[root as int]),
        final(arena)@[root as int] == Term::Atom(name@),
{
    let node = ENode { kind: ENodeKind::Atom { name }, term: Ghost(Term::Atom(name@)) };
    proof {
        reveal(node_ok);
        reveal(ckc_spec::term::wf_term);
    }
    push_node(arena, node)
}

fn vec_slice_equal(left: &Vec<u8>, right: &[u8]) -> (equal: bool)
    ensures
        equal == (left@ == right@),
{
    if left.len() != right.len() {
        return false;
    }
    let mut i = 0usize;
    while i < left.len()
        invariant
            left@.len() == right@.len(),
            i <= left@.len(),
            forall|j: int| 0 <= j < i ==> left@[j] == right@[j],
        decreases left.len() - i,
    {
        if left[i] != right[i] {
            return false;
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(left@ == right@);
    }
    true
}

#[verifier::rlimit(5000)]
pub fn push_comp(arena: &mut ETermArena, name: Vec<u8>, child_roots: Vec<usize>) -> (root: usize)
    requires
        arena_ok(old(arena)),
        child_roots@.len() > 0,
        forall|j: int|
            0 <= j < child_roots@.len() ==> #[trigger] child_roots@[j] < old(arena).nodes@.len(),
    ensures
        arena_ok(final(arena)),
        root == old(arena).nodes@.len(),
        final(arena).nodes@ == old(arena).nodes@.push(final(arena).nodes@[root as int]),
        final(arena)@[root as int] == Term::Comp(
            name@,
            child_terms(old(arena).nodes@, child_roots@),
        ),
{
    let cons_bytes: &[u8] = b"[|]";
    let curly_bytes: &[u8] = b"{}";
    proof {
        reveal_byteslit(b"[|]");
        reveal_byteslit(b"{}");
        reveal(ckc_spec::v1text::cons_name);
        reveal(ckc_spec::v1text::curly_name);
    }
    let is_cons = vec_slice_equal(&name, cons_bytes);
    let is_curly = vec_slice_equal(&name, curly_bytes);
    let form = if is_cons && child_roots.len() == 2 {
        ECompForm::Cons
    } else if is_curly && child_roots.len() == 1 {
        ECompForm::Curly
    } else {
        ECompForm::Regular
    };
    let ghost old_nodes = arena.nodes@;
    let ghost roots = child_roots@;
    let ghost spec_name = name@;
    let ghost args = child_terms(old_nodes, roots);
    let ghost model = Term::Comp(spec_name, args);
    proof {
        assert(cons_bytes@ == ckc_spec::v1text::cons_name());
        assert(curly_bytes@ == ckc_spec::v1text::curly_name());
        assert(is_cons == (spec_name == ckc_spec::v1text::cons_name()));
        assert(is_curly == (spec_name == ckc_spec::v1text::curly_name()));
        reveal(comp_form_ok);
        assert(comp_form_ok(&form, spec_name, roots.len()));
    }
    let node = ENode { kind: ENodeKind::Comp { name, child_roots, form }, term: Ghost(model) };
    proof {
        reveal(arena_ok);
        assert forall|j: int| 0 <= j < roots.len() implies {
            &&& roots[j] < old_nodes.len()
            &&& node_ok(old_nodes, roots[j] as int)
        } by {
            assert(node_ok(old_nodes, roots[j] as int));
        }
        child_terms_wf(old_nodes, roots);
        assert(ckc_spec::term::wf_terms(args));
        reveal_with_fuel(ckc_spec::term::wf_term, 2);
        assert(ckc_spec::term::wf_term(model));
        let appended = old_nodes.push(node);
        assert(old_nodes.is_prefix_of(appended));
        child_terms_prefix_stable(old_nodes, appended, roots);
        assert(child_terms(appended, roots) == args);
        assert(child_roots_before(roots, old_nodes.len() as int)) by {
            reveal(child_roots_before);
            assert forall|j: int| 0 <= j < roots.len() implies ((roots[j] as int)
                < old_nodes.len() as int) by {}
        }
        assert(child_roots_valid(appended, roots)) by {
            reveal(child_roots_valid);
            assert forall|j: int| 0 <= j < roots.len() implies roots[j] < appended.len() by {}
        }
        assert(appended[old_nodes.len() as int] == node);
        reveal(node_ok);
        assert(node_ok(appended, old_nodes.len() as int));
    }
    push_node(arena, node)
}

fn append_bytes(out: &mut Vec<u8>, bytes: &[u8])
    ensures
        final(out)@ == old(out)@ + bytes@,
{
    let ghost base = out@;
    let mut i = 0usize;
    while i < bytes.len()
        invariant
            i <= bytes@.len(),
            out@ == base + bytes@.take(i as int),
        decreases bytes.len() - i,
    {
        out.push(bytes[i]);
        proof {
            assert_seqs_equal!(bytes@.take(i as int + 1)
                == bytes@.take(i as int).push(bytes@[i as int]));
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(bytes@.take(bytes@.len() as int) == bytes@);
    }
}

fn is_lower_b(b: u8) -> (r: bool)
    ensures
        r == ckc_spec::v1text::is_lower_b(b),
{
    0x61 <= b && b <= 0x7a
}

fn is_digit_b(b: u8) -> (r: bool)
    ensures
        r == ckc_spec::v1text::is_digit_b(b),
{
    0x30 <= b && b <= 0x39
}

fn is_alnum_b(b: u8) -> (r: bool)
    ensures
        r == ckc_spec::v1text::is_alnum_b(b),
{
    is_lower_b(b) || (0x41 <= b && b <= 0x5a) || is_digit_b(b) || b == 0x5f
}

fn is_graphic_b(b: u8) -> (r: bool)
    ensures
        r == ckc_spec::v1text::is_graphic_b(b),
{
    b == 0x23 || b == 0x24 || b == 0x26 || b == 0x2a || b == 0x2b || b == 0x2d || b == 0x2e || b
        == 0x2f || b == 0x3a || b == 0x3c || b == 0x3d || b == 0x3e || b == 0x3f || b == 0x40 || b
        == 0x5c || b == 0x5e || b == 0x7e
}

fn all_alnum_b(s: &[u8]) -> (r: bool)
    ensures
        r == ckc_spec::v1text::all_in(s@, |b: u8| ckc_spec::v1text::is_alnum_b(b)),
{
    let mut i = 0usize;
    while i < s.len()
        invariant
            i <= s@.len(),
            forall|j: int| 0 <= j < i ==> ckc_spec::v1text::is_alnum_b(s@[j]),
        decreases s.len() - i,
    {
        if !is_alnum_b(s[i]) {
            return false;
        }
        i += 1;
    }
    proof {
        reveal(ckc_spec::v1text::all_in);
    }
    true
}

fn all_graphic_b(s: &[u8]) -> (r: bool)
    ensures
        r == ckc_spec::v1text::all_in(s@, |b: u8| ckc_spec::v1text::is_graphic_b(b)),
{
    let mut i = 0usize;
    while i < s.len()
        invariant
            i <= s@.len(),
            forall|j: int| 0 <= j < i ==> ckc_spec::v1text::is_graphic_b(s@[j]),
        decreases s.len() - i,
    {
        if !is_graphic_b(s[i]) {
            return false;
        }
        i += 1;
    }
    proof {
        reveal(ckc_spec::v1text::all_in);
    }
    true
}

proof fn seq_eq_one(s: Seq<u8>, x: u8)
    ensures
        s == seq![x] <==> s.len() == 1 && s[0] == x,
{
    if s.len() == 1 && s[0] == x {
        assert_seqs_equal!(s == seq![x]);
    }
}

proof fn seq_eq_two(s: Seq<u8>, x: u8, y: u8)
    ensures
        s == seq![x, y] <==> s.len() == 2 && s[0] == x && s[1] == y,
{
    if s.len() == 2 && s[0] == x && s[1] == y {
        assert_seqs_equal!(s == seq![x, y]);
    }
}

fn atom_bare_exec(name: &[u8]) -> (r: bool)
    ensures
        r == ckc_spec::v1text::atom_bare(name@),
{
    let alpha = name.len() > 0 && is_lower_b(name[0]) && all_alnum_b(name);
    let graphic = name.len() > 0 && all_graphic_b(name) && !(name.len() == 1 && name[0] == 0x2e)
        && !(name.len() >= 2 && name[0] == 0x2f && name[1] == 0x2a);
    let solo = (name.len() == 1 && (name[0] == 0x3b || name[0] == 0x21)) || (name.len() == 2
        && name[0] == 0x7b && name[1] == 0x7d);
    proof {
        seq_eq_one(name@, 0x2e);
        seq_eq_one(name@, 0x3b);
        seq_eq_one(name@, 0x21);
        seq_eq_two(name@, 0x7b, 0x7d);
        reveal(ckc_spec::v1text::alpha_bare);
        reveal(ckc_spec::v1text::graphic_bare);
        reveal(ckc_spec::v1text::solo_bare);
        reveal(ckc_spec::v1text::atom_bare);
    }
    alpha || graphic || solo
}

fn uhex_digit_exec(d: u8) -> (r: u8)
    requires
        d < 16,
    ensures
        r == ckc_spec::v1text::uhex_digit(d as int),
{
    let r = if d < 10 {
        0x30 + d
    } else {
        0x41 + (d - 10)
    };
    proof {
        reveal(ckc_spec::v1text::uhex_digit);
        reveal(ckc_spec::v1text::digit_byte);
    }
    r
}

fn append_esc(out: &mut Vec<u8>, b: u8)
    ensures
        final(out)@ == old(out)@ + ckc_spec::v1text::esc_byte(b),
{
    if b == 0x5c || b == 0x27 || (0x07 <= b && b <= 0x0d) {
        let code = if b == 0x5c {
            0x5c
        } else if b == 0x27 {
            0x27
        } else if b == 0x07 {
            0x61
        } else if b == 0x08 {
            0x62
        } else if b == 0x09 {
            0x74
        } else if b == 0x0a {
            0x6e
        } else if b == 0x0b {
            0x76
        } else if b == 0x0c {
            0x66
        } else {
            0x72
        };
        out.push(0x5c);
        out.push(code);
        proof {
            reveal_strlit("\u{5C}\u{5C}");
            reveal_strlit("\\'");
            reveal_strlit("\\a");
            reveal_strlit("\\b");
            reveal_strlit("\\t");
            reveal_strlit("\\n");
            reveal_strlit("\\v");
            reveal_strlit("\\f");
            reveal_strlit("\\r");
            reveal(ckc_spec::v1text::ascii);
            reveal(ckc_spec::v1text::esc_byte);
        }
    } else if b < 0x20 || b == 0x7f {
        let hi = uhex_digit_exec(b / 16);
        let lo = uhex_digit_exec(b % 16);
        out.push(0x5c);
        out.push(0x75);
        out.push(0x30);
        out.push(0x30);
        out.push(hi);
        out.push(lo);
        proof {
            reveal_strlit("\\u");
            reveal(ckc_spec::v1text::ascii);
            reveal(ckc_spec::v1text::esc_byte);
            reveal(ckc_spec::v1text::uhex4);
            reveal(ckc_spec::v1text::uhex_digit);
            reveal(ckc_spec::v1text::digit_byte);
            assert(b as int / 4096 % 16 == 0);
            assert(b as int / 256 % 16 == 0);
            assert(b as int / 16 % 16 == (b / 16) as int);
            assert(b as int % 16 == (b % 16) as int);
        }
    } else {
        out.push(b);
        proof {
            reveal(ckc_spec::v1text::esc_byte);
        }
    }
}

fn append_atom(out: &mut Vec<u8>, name: &[u8])
    ensures
        final(out)@ == old(out)@ + ckc_spec::v1text::atom_bytes(name@),
{
    if atom_bare_exec(name) {
        append_bytes(out, name);
        proof {
            reveal(ckc_spec::v1text::atom_bytes);
        }
    } else {
        let ghost base = out@;
        out.push(0x27);
        proof {
            assert_seqs_equal!(name@.skip(0) == name@);
            assert(out@ == base + seq![0x27u8]);
        }
        let mut i = 0usize;
        while i < name.len()
            invariant
                i <= name@.len(),
                out@ + ckc_spec::v1text::esc_all(name@.skip(i as int)) == base + seq![0x27u8]
                    + ckc_spec::v1text::esc_all(name@),
            decreases name.len() - i,
        {
            append_esc(out, name[i]);
            proof {
                reveal_with_fuel(ckc_spec::v1text::esc_all, 2);
                assert(name@.skip(i as int)[0] == name@[i as int]);
                assert_seqs_equal!(name@.skip(i as int).drop_first()
                    == name@.skip(i as int + 1));
            }
            i += 1;
        }
        proof {
            reveal_with_fuel(ckc_spec::v1text::esc_all, 2);
            assert_seqs_equal!(name@.skip(name@.len() as int) == Seq::<u8>::empty());
        }
        out.push(0x27);
        proof {
            reveal(ckc_spec::v1text::atom_bytes);
        }
    }
}

#[verifier::rlimit(5000)]
fn process_task(
    arena: &ETermArena,
    task: EPrintTask,
    tasks: &mut Vec<EPrintTask>,
    out: &mut Vec<u8>,
)
    requires
        arena_ok(arena),
        task_ok(arena.nodes@, &task),
        tasks_ok(arena.nodes@, old(tasks)@),
    ensures
        tasks_ok(arena.nodes@, final(tasks)@),
        final(out)@ + tasks_bytes(final(tasks)@) == old(out)@ + task_bytes(&task) + tasks_bytes(
            old(tasks)@,
        ),
        tasks_work(final(tasks)@) < task_work(&task) + tasks_work(old(tasks)@),
{
    let ghost old_stack = tasks@;
    let ghost old_out = out@;
    let node_count = arena.nodes.len();
    proof {
        assert(arena.nodes@.len() == node_count as int);
        assert(arena.nodes@.len() <= usize::MAX as int);
    }
    match task {
        EPrintTask::Byte(b) => {
            out.push(b);
            proof {
                reveal(task_bytes);
                reveal(task_work);
            }
        },
        EPrintTask::Args { parent, next, model } => {
            proof {
                reveal(task_ok);
                assert(parent < arena.nodes@.len());
            }
            match &arena.nodes[parent].kind {
                ENodeKind::Comp { child_roots, .. } => {
                    proof {
                        reveal(child_roots_valid);
                        child_terms_skip_step(arena.nodes@, child_roots@, next);
                        assert(next < child_roots.len());
                        assert(model@.len() == child_roots@.len() - next);
                        assert(model@.len() > 0);
                    }
                    let first = EPrintTask::Term {
                        index: child_roots[next],
                        model: Ghost(model@[0]),
                    };
                    let last = child_roots.len() - 1;
                    if next == last {
                        proof {
                            assert(next + 1 == child_roots@.len());
                            assert(model@.len() == 1);
                            reveal_with_fuel(ckc_spec::v1text::args_bytes, 2);
                            reveal(args_work);
                            assert(task_ok(arena.nodes@, &first));
                            tasks_ok_push(arena.nodes@, tasks@, first);
                            tasks_bytes_push(tasks@, first);
                            tasks_work_push(tasks@, first);
                        }
                        tasks.push(first);
                    } else {
                        let rest = EPrintTask::Args {
                            parent,
                            next: next + 1,
                            model: Ghost(model@.drop_first()),
                        };
                        let comma = EPrintTask::Byte(0x2c);
                        proof {
                            assert(next < last);
                            assert(next + 1 < child_roots@.len());
                            assert(model@.len() > 1);
                            assert_seqs_equal!(model@.drop_first()
                                == child_terms(
                                    arena.nodes@,
                                    child_roots@.skip(next as int + 1),
                                ));
                            reveal_with_fuel(ckc_spec::v1text::args_bytes, 2);
                            reveal(args_work);
                            assert(task_ok(arena.nodes@, &rest));
                            assert(task_ok(arena.nodes@, &comma));
                            assert(task_ok(arena.nodes@, &first));
                            tasks_ok_push(arena.nodes@, tasks@, rest);
                            tasks_bytes_push(tasks@, rest);
                            tasks_work_push(tasks@, rest);
                        }
                        tasks.push(rest);
                        proof {
                            tasks_ok_push(arena.nodes@, old_stack.push(rest), comma);
                            tasks_bytes_push(old_stack.push(rest), comma);
                            tasks_work_push(old_stack.push(rest), comma);
                        }
                        tasks.push(comma);
                        proof {
                            tasks_ok_push(arena.nodes@, old_stack.push(rest).push(comma), first);
                            tasks_bytes_push(old_stack.push(rest).push(comma), first);
                            tasks_work_push(old_stack.push(rest).push(comma), first);
                        }
                        tasks.push(first);
                    }
                },
                _ => {
                    proof {
                        assert(false);
                    }
                },
            }
        },
        EPrintTask::Term { index, model } => {
            proof {
                reveal(task_ok);
                reveal(arena_ok);
                assert(node_ok(arena.nodes@, index as int));
                reveal(node_ok);
            }
            match &arena.nodes[index].kind {
                ENodeKind::Var { spelling, .. } => {
                    append_bytes(out, spelling.as_slice());
                    proof {
                        reveal_with_fuel(ckc_spec::v1text::term_bytes, 2);
                        reveal(task_bytes);
                        reveal(term_work);
                    }
                },
                ENodeKind::Int { spelling, .. } => {
                    append_bytes(out, spelling.as_slice());
                    proof {
                        reveal_with_fuel(ckc_spec::v1text::term_bytes, 2);
                        reveal(task_bytes);
                        reveal(term_work);
                    }
                },
                ENodeKind::Nil => {
                    out.push(0x5b);
                    out.push(0x5d);
                    proof {
                        reveal_strlit("[]");
                        reveal(ckc_spec::v1text::ascii);
                        reveal_with_fuel(ckc_spec::v1text::term_bytes, 2);
                        reveal(task_bytes);
                        reveal(term_work);
                    }
                },
                ENodeKind::Atom { name } => {
                    append_atom(out, name.as_slice());
                    proof {
                        reveal_with_fuel(ckc_spec::v1text::term_bytes, 2);
                        reveal(task_bytes);
                        reveal(term_work);
                    }
                },
                ENodeKind::Comp { name, child_roots, form } => {
                    let ghost args = match model@ {
                        Term::Comp(_, args) => args,
                        _ => Seq::empty(),
                    };
                    match form {
                        ECompForm::Cons => {
                            out.push(0x5b);
                            let close = EPrintTask::Byte(0x5d);
                            let tail = EPrintTask::Tail {
                                index: child_roots[1],
                                model: Ghost(args[1]),
                            };
                            let head = EPrintTask::Term {
                                index: child_roots[0],
                                model: Ghost(args[0]),
                            };
                            proof {
                                reveal_with_fuel(ckc_spec::v1text::term_bytes, 2);
                                reveal(task_bytes);
                                reveal(term_work);
                                assert(child_roots@.len() == 2);
                                assert(task_ok(arena.nodes@, &close));
                                assert(task_ok(arena.nodes@, &tail));
                                assert(task_ok(arena.nodes@, &head));
                                tasks_ok_push(arena.nodes@, tasks@, close);
                                tasks_bytes_push(tasks@, close);
                                tasks_work_push(tasks@, close);
                            }
                            tasks.push(close);
                            proof {
                                tasks_ok_push(arena.nodes@, old_stack.push(close), tail);
                                tasks_bytes_push(old_stack.push(close), tail);
                                tasks_work_push(old_stack.push(close), tail);
                            }
                            tasks.push(tail);
                            proof {
                                tasks_ok_push(arena.nodes@, old_stack.push(close).push(tail), head);
                                tasks_bytes_push(old_stack.push(close).push(tail), head);
                                tasks_work_push(old_stack.push(close).push(tail), head);
                            }
                            tasks.push(head);
                        },
                        ECompForm::Curly => {
                            out.push(0x7b);
                            let close = EPrintTask::Byte(0x7d);
                            let child = EPrintTask::Term {
                                index: child_roots[0],
                                model: Ghost(args[0]),
                            };
                            proof {
                                reveal_with_fuel(ckc_spec::v1text::term_bytes, 2);
                                reveal(task_bytes);
                                reveal(term_work);
                                assert(child_roots@.len() == 1);
                                assert(task_ok(arena.nodes@, &close));
                                assert(task_ok(arena.nodes@, &child));
                                tasks_ok_push(arena.nodes@, tasks@, close);
                                tasks_bytes_push(tasks@, close);
                                tasks_work_push(tasks@, close);
                            }
                            tasks.push(close);
                            proof {
                                tasks_ok_push(arena.nodes@, old_stack.push(close), child);
                                tasks_bytes_push(old_stack.push(close), child);
                                tasks_work_push(old_stack.push(close), child);
                            }
                            tasks.push(child);
                        },
                        ECompForm::Regular => {
                            append_atom(out, name.as_slice());
                            out.push(0x28);
                            let close = EPrintTask::Byte(0x29);
                            let children = EPrintTask::Args {
                                parent: index,
                                next: 0,
                                model: Ghost(args),
                            };
                            proof {
                                reveal_with_fuel(ckc_spec::v1text::term_bytes, 2);
                                reveal(task_bytes);
                                reveal(term_work);
                                assert(child_roots@.len() > 0);
                                assert_seqs_equal!(child_roots@.skip(0) == child_roots@);
                                assert(task_ok(arena.nodes@, &close));
                                assert(task_ok(arena.nodes@, &children));
                                tasks_ok_push(arena.nodes@, tasks@, close);
                                tasks_bytes_push(tasks@, close);
                                tasks_work_push(tasks@, close);
                            }
                            tasks.push(close);
                            proof {
                                tasks_ok_push(arena.nodes@, old_stack.push(close), children);
                                tasks_bytes_push(old_stack.push(close), children);
                                tasks_work_push(old_stack.push(close), children);
                            }
                            tasks.push(children);
                        },
                    }
                },
            }
        },
        EPrintTask::Tail { index, model } => {
            proof {
                reveal(task_ok);
                reveal(arena_ok);
                assert(node_ok(arena.nodes@, index as int));
                reveal(node_ok);
            }
            match &arena.nodes[index].kind {
                ENodeKind::Nil => {
                    proof {
                        reveal_with_fuel(ckc_spec::v1text::tail_bytes, 2);
                        reveal(task_bytes);
                        reveal(tail_work);
                    }
                },
                ENodeKind::Comp { child_roots, form: ECompForm::Cons, .. } => {
                    let ghost args = match model@ {
                        Term::Comp(_, args) => args,
                        _ => Seq::empty(),
                    };
                    let tail = EPrintTask::Tail { index: child_roots[1], model: Ghost(args[1]) };
                    let head = EPrintTask::Term { index: child_roots[0], model: Ghost(args[0]) };
                    let comma = EPrintTask::Byte(0x2c);
                    proof {
                        reveal_with_fuel(ckc_spec::v1text::tail_bytes, 2);
                        reveal(task_bytes);
                        reveal(tail_work);
                        assert(child_roots@.len() == 2);
                        assert(task_ok(arena.nodes@, &tail));
                        assert(task_ok(arena.nodes@, &head));
                        assert(task_ok(arena.nodes@, &comma));
                        tasks_ok_push(arena.nodes@, tasks@, tail);
                        tasks_bytes_push(tasks@, tail);
                        tasks_work_push(tasks@, tail);
                    }
                    tasks.push(tail);
                    proof {
                        tasks_ok_push(arena.nodes@, old_stack.push(tail), head);
                        tasks_bytes_push(old_stack.push(tail), head);
                        tasks_work_push(old_stack.push(tail), head);
                    }
                    tasks.push(head);
                    proof {
                        tasks_ok_push(arena.nodes@, old_stack.push(tail).push(head), comma);
                        tasks_bytes_push(old_stack.push(tail).push(head), comma);
                        tasks_work_push(old_stack.push(tail).push(head), comma);
                    }
                    tasks.push(comma);
                },
                _ => {
                    let term = EPrintTask::Term { index, model };
                    let pipe = EPrintTask::Byte(0x7c);
                    proof {
                        reveal_with_fuel(ckc_spec::v1text::tail_bytes, 2);
                        reveal(task_bytes);
                        reveal(tail_work);
                        assert(task_ok(arena.nodes@, &term));
                        assert(task_ok(arena.nodes@, &pipe));
                        tasks_ok_push(arena.nodes@, tasks@, term);
                        tasks_bytes_push(tasks@, term);
                        tasks_work_push(tasks@, term);
                    }
                    tasks.push(term);
                    proof {
                        tasks_ok_push(arena.nodes@, old_stack.push(term), pipe);
                        tasks_bytes_push(old_stack.push(term), pipe);
                        tasks_work_push(old_stack.push(term), pipe);
                    }
                    tasks.push(pipe);
                },
            }
        },
    }
    proof {
        assert(tasks@.len() >= old_stack.len());
        let _ = old_out;
    }
}

#[verifier::rlimit(5000)]
pub fn term_line(arena: &ETermArena, root: usize) -> (out: Vec<u8>)
    requires
        root_ok(arena, root),
    ensures
        out@ == ckc_spec::v1text::term_line(arena@[root as int]),
{
    let ghost model = arena@[root as int];
    let mut out = Vec::new();
    let mut tasks = Vec::new();
    let first = EPrintTask::Term { index: root, model: Ghost(model) };
    proof {
        reveal(root_ok);
        assert(task_ok(arena.nodes@, &first));
        tasks_ok_push(arena.nodes@, Seq::empty(), first);
        tasks_bytes_push(Seq::empty(), first);
        reveal(tasks_bytes);
        reveal(task_bytes);
    }
    tasks.push(first);
    while tasks.len() > 0
        invariant
            arena_ok(arena),
            tasks_ok(arena.nodes@, tasks@),
            out@ + tasks_bytes(tasks@) == ckc_spec::v1text::term_bytes(model),
        decreases tasks_work(tasks@),
    {
        let ghost before = tasks@;
        let task = tasks.pop().unwrap();
        proof {
            reveal_with_fuel(tasks_ok, 2);
            reveal_with_fuel(tasks_bytes, 2);
            reveal_with_fuel(tasks_work, 2);
            assert(task == before.last());
            assert(tasks@ == before.drop_last());
            assert(task_ok(arena.nodes@, &task));
        }
        process_task(arena, task, &mut tasks, &mut out);
    }
    proof {
        reveal(tasks_bytes);
        assert(out@ == ckc_spec::v1text::term_bytes(model));
    }
    out.push(0x2e);
    out.push(0x0a);
    proof {
        reveal_strlit(".\n");
        reveal(ckc_spec::v1text::ascii);
        reveal(ckc_spec::v1text::term_line);
    }
    out
}

pub open spec fn decimal_digit(b: u8) -> nat {
    (b as int - 0x30) as nat
}

pub open spec fn decimal_value(s: Seq<u8>) -> nat
    decreases s.len(),
{
    if s.len() == 0 {
        0
    } else {
        decimal_value(s.drop_last()) * 10 + decimal_digit(s.last())
    }
}

pub open spec fn canonical_decimal(s: Seq<u8>) -> bool {
    &&& s.len() > 0
    &&& ckc_spec::v1text::all_in(s, |b: u8| ckc_spec::v1text::is_digit_b(b))
    &&& (s.len() == 1 || s[0] != 0x30)
}

proof fn decimal_digit_bounds(b: u8)
    requires
        ckc_spec::v1text::is_digit_b(b),
    ensures
        decimal_digit(b) < 10,
        ckc_spec::v1text::digit_byte(decimal_digit(b) as int) == b,
{
    reveal(ckc_spec::v1text::is_digit_b);
    reveal(decimal_digit);
    reveal(ckc_spec::v1text::digit_byte);
}

proof fn decimal_all_drop_last(s: Seq<u8>)
    requires
        s.len() > 0,
        ckc_spec::v1text::all_in(s, |b: u8| ckc_spec::v1text::is_digit_b(b)),
    ensures
        ckc_spec::v1text::all_in(s.drop_last(), |b: u8| ckc_spec::v1text::is_digit_b(b)),
{
    reveal(ckc_spec::v1text::all_in);
    assert forall|i: int|
        #![auto]
        0 <= i < s.drop_last().len() ==> ckc_spec::v1text::is_digit_b(s.drop_last()[i]) by {
        if 0 <= i < s.drop_last().len() {
            assert(i < s.len());
            assert(s.drop_last()[i] == s[i]);
        }
    }
}

proof fn canonical_decimal_drop_last(s: Seq<u8>)
    requires
        s.len() > 1,
        canonical_decimal(s),
    ensures
        canonical_decimal(s.drop_last()),
{
    decimal_all_drop_last(s);
    reveal(canonical_decimal);
    if s.drop_last().len() > 1 {
        assert(s.drop_last()[0] == s[0]);
    }
}

proof fn decimal_positive(s: Seq<u8>)
    requires
        canonical_decimal(s),
        s[0] != 0x30,
    ensures
        decimal_value(s) > 0,
    decreases s.len(),
{
    reveal_with_fuel(decimal_value, 2);
    reveal(canonical_decimal);
    decimal_digit_bounds(s.last());
    if s.len() == 1 {
        assert(s.last() == s[0]);
        assert(decimal_digit(s[0]) > 0);
    } else {
        canonical_decimal_drop_last(s);
        assert(s.drop_last()[0] == s[0]);
        decimal_positive(s.drop_last());
    }
}

proof fn udec_canonical(n: nat)
    ensures
        canonical_decimal(ckc_spec::v1text::udec_bytes(n)),
        n > 0 ==> ckc_spec::v1text::udec_bytes(n)[0] != 0x30,
    decreases n,
{
    reveal_with_fuel(ckc_spec::v1text::udec_bytes, 2);
    reveal(ckc_spec::v1text::digit_byte);
    reveal(ckc_spec::v1text::is_digit_b);
    reveal(ckc_spec::v1text::all_in);
    reveal(canonical_decimal);
    if n < 10 {
        assert(ckc_spec::v1text::udec_bytes(n).len() == 1);
        assert(ckc_spec::v1text::is_digit_b(ckc_spec::v1text::digit_byte(n as int)));
        if n > 0 {
            assert(ckc_spec::v1text::digit_byte(n as int) != 0x30);
        }
    } else {
        let q = n / 10;
        let d = n % 10;
        assert(q > 0);
        assert(q < n);
        assert(d < 10);
        udec_canonical(q);
        let prefix = ckc_spec::v1text::udec_bytes(q);
        let digit = ckc_spec::v1text::digit_byte(d as int);
        assert(ckc_spec::v1text::is_digit_b(digit));
        assert forall|i: int|
            #![auto]
            0 <= i < (prefix + seq![digit]).len() ==> ckc_spec::v1text::is_digit_b(
                (prefix + seq![digit])[i],
            ) by {
            if 0 <= i < (prefix + seq![digit]).len() {
                if i < prefix.len() {
                    assert((prefix + seq![digit])[i] == prefix[i]);
                } else {
                    assert(i == prefix.len());
                    assert((prefix + seq![digit])[i] == digit);
                }
            }
        }
        assert((prefix + seq![digit])[0] == prefix[0]);
        assert(prefix[0] != 0x30);
    }
}

proof fn udec_decimal_value(n: nat)
    ensures
        decimal_value(ckc_spec::v1text::udec_bytes(n)) == n,
    decreases n,
{
    reveal_with_fuel(ckc_spec::v1text::udec_bytes, 2);
    reveal_with_fuel(decimal_value, 2);
    reveal(decimal_digit);
    reveal(ckc_spec::v1text::digit_byte);
    if n < 10 {
        assert(ckc_spec::v1text::udec_bytes(n) == seq![ckc_spec::v1text::digit_byte(n as int)]);
    } else {
        let q = n / 10;
        let d = n % 10;
        assert(q < n);
        udec_decimal_value(q);
        let prefix = ckc_spec::v1text::udec_bytes(q);
        let digit = ckc_spec::v1text::digit_byte(d as int);
        assert((prefix + seq![digit]).last() == digit);
        assert_seqs_equal!((prefix + seq![digit]).drop_last() == prefix);
        lemma_fundamental_div_mod(n as int, 10);
        assert(n == 10 * q + d);
    }
}

pub open spec fn pow10(n: nat) -> nat
    decreases n,
{
    if n == 0 {
        1
    } else {
        10 * pow10((n - 1) as nat)
    }
}

proof fn pow10_positive(n: nat)
    ensures
        pow10(n) >= 1,
    decreases n,
{
    reveal_with_fuel(pow10, 2);
    if n > 0 {
        pow10_positive((n - 1) as nat);
    }
}

proof fn pow10_monotonic(a: nat, b: nat)
    requires
        a <= b,
    ensures
        pow10(a) <= pow10(b),
    decreases b - a,
{
    if a < b {
        pow10_monotonic(a, (b - 1) as nat);
        pow10_positive((b - 1) as nat);
        reveal_with_fuel(pow10, 2);
        assert(pow10((b - 1) as nat) <= 10 * pow10((b - 1) as nat)) by (nonlinear_arith);
    }
}

proof fn decimal_all_drop_first(s: Seq<u8>)
    requires
        s.len() > 0,
        ckc_spec::v1text::all_in(s, |b: u8| ckc_spec::v1text::is_digit_b(b)),
    ensures
        ckc_spec::v1text::all_in(s.drop_first(), |b: u8| ckc_spec::v1text::is_digit_b(b)),
{
    reveal(ckc_spec::v1text::all_in);
    assert forall|i: int| 0 <= i < s.drop_first().len() implies ckc_spec::v1text::is_digit_b(
        s.drop_first()[i],
    ) by {
        assert(s.drop_first()[i] == s[i + 1]);
    }
}

proof fn decimal_value_bound(s: Seq<u8>)
    requires
        ckc_spec::v1text::all_in(s, |b: u8| ckc_spec::v1text::is_digit_b(b)),
    ensures
        decimal_value(s) < pow10(s.len()),
    decreases s.len(),
{
    if s.len() == 0 {
        reveal_with_fuel(decimal_value, 2);
        reveal_with_fuel(pow10, 2);
    } else {
        decimal_all_drop_last(s);
        decimal_value_bound(s.drop_last());
        decimal_digit_bounds(s.last());
        reveal_with_fuel(decimal_value, 2);
        reveal_with_fuel(pow10, 2);
        let v = decimal_value(s.drop_last());
        let d = decimal_digit(s.last());
        let p = pow10((s.len() - 1) as nat);
        assert(s.drop_last().len() == s.len() - 1);
        assert(v < p);
        assert(d < 10);
        assert(decimal_value(s) == v * 10 + d);
        assert(pow10(s.len()) == 10 * p);
        assert(v * 10 + d < 10 * p);
    }
}

proof fn decimal_value_prepend(first: u8, rest: Seq<u8>)
    requires
        ckc_spec::v1text::is_digit_b(first),
        ckc_spec::v1text::all_in(rest, |b: u8| ckc_spec::v1text::is_digit_b(b)),
    ensures
        decimal_value(seq![first] + rest) == decimal_digit(first) * pow10(rest.len())
            + decimal_value(rest),
    decreases rest.len(),
{
    decimal_digit_bounds(first);
    if rest.len() == 0 {
        reveal_with_fuel(decimal_value, 2);
        reveal_with_fuel(pow10, 2);
        assert_seqs_equal!((seq![first] + rest).drop_last() == Seq::<u8>::empty());
        assert((seq![first] + rest).last() == first);
        assert(decimal_value(Seq::<u8>::empty()) == 0);
        assert(decimal_value(seq![first] + rest) == decimal_digit(first));
        assert(pow10(rest.len()) == 1);
        assert(decimal_value(rest) == 0);
        assert(decimal_digit(first) * pow10(rest.len()) == decimal_digit(first))
            by (nonlinear_arith)
            requires
                pow10(rest.len()) == 1,
        ;
    } else {
        decimal_all_drop_last(rest);
        decimal_value_prepend(first, rest.drop_last());
        decimal_digit_bounds(rest.last());
        let whole = seq![first] + rest;
        let shorter = seq![first] + rest.drop_last();
        assert_seqs_equal!(whole.drop_last() == shorter);
        assert(whole.len() == rest.len() + 1);
        assert(whole.last() == rest.last());
        reveal_with_fuel(decimal_value, 2);
        reveal_with_fuel(pow10, 2);
        let q = pow10((rest.len() - 1) as nat);
        let df = decimal_digit(first);
        let w = decimal_value(rest.drop_last());
        let dl = decimal_digit(rest.last());
        assert(rest.drop_last().len() == rest.len() - 1);
        assert(decimal_value(shorter) == df * q + w);
        assert(decimal_value(whole) == decimal_value(shorter) * 10 + dl);
        assert(pow10(rest.len()) == 10 * q);
        assert(decimal_value(rest) == w * 10 + dl);
        assert((df * q + w) * 10 + dl == df * (10 * q) + (w * 10 + dl)) by (nonlinear_arith);
        assert(decimal_value(whole) == df * pow10(rest.len()) + decimal_value(rest))
            by (nonlinear_arith)
            requires
                decimal_value(whole) == (df * q + w) * 10 + dl,
                pow10(rest.len()) == 10 * q,
                decimal_value(rest) == w * 10 + dl,
        ;
    }
}

proof fn decimal_nonzero_leading_min(s: Seq<u8>)
    requires
        s.len() > 0,
        ckc_spec::v1text::all_in(s, |b: u8| ckc_spec::v1text::is_digit_b(b)),
        s[0] != 0x30,
    ensures
        pow10((s.len() - 1) as nat) <= decimal_value(s),
{
    let rest = s.drop_first();
    decimal_all_drop_first(s);
    reveal(ckc_spec::v1text::all_in);
    assert(ckc_spec::v1text::is_digit_b(s[0]));
    decimal_digit_bounds(s[0]);
    decimal_value_prepend(s[0], rest);
    assert_seqs_equal!(s == seq![s[0]] + rest);
    reveal(decimal_digit);
    reveal(ckc_spec::v1text::is_digit_b);
    let d = decimal_digit(s[0]);
    let p = pow10(rest.len());
    assert(d >= 1);
    assert(rest.len() == s.len() - 1);
    pow10_positive(rest.len());
    assert(p <= d * p) by (nonlinear_arith)
        requires
            d >= 1,
            p >= 0,
    ;
    assert(decimal_value(s) == d * p + decimal_value(rest));
}

proof fn decimal_shorter_less(a: Seq<u8>, b: Seq<u8>)
    requires
        canonical_decimal(a),
        canonical_decimal(b),
        a.len() < b.len(),
    ensures
        decimal_value(a) < decimal_value(b),
{
    reveal(canonical_decimal);
    decimal_value_bound(a);
    assert(b.len() > 1);
    assert(b[0] != 0x30);
    decimal_nonzero_leading_min(b);
    assert(a.len() <= b.len() - 1);
    pow10_monotonic(a.len(), (b.len() - 1) as nat);
}

pub open spec fn decimal_lex_lt(a: Seq<u8>, b: Seq<u8>) -> bool
    decreases a.len(),
{
    if a.len() == 0 || b.len() == 0 {
        false
    } else if decimal_digit(a[0]) < decimal_digit(b[0]) {
        true
    } else if decimal_digit(a[0]) > decimal_digit(b[0]) {
        false
    } else {
        decimal_lex_lt(a.drop_first(), b.drop_first())
    }
}

proof fn decimal_lex_value(a: Seq<u8>, b: Seq<u8>)
    requires
        a.len() == b.len(),
        ckc_spec::v1text::all_in(a, |x: u8| ckc_spec::v1text::is_digit_b(x)),
        ckc_spec::v1text::all_in(b, |x: u8| ckc_spec::v1text::is_digit_b(x)),
    ensures
        decimal_lex_lt(a, b) <==> decimal_value(a) < decimal_value(b),
    decreases a.len(),
{
    reveal_with_fuel(decimal_lex_lt, 2);
    if a.len() == 0 {
        reveal_with_fuel(decimal_value, 2);
    } else {
        let ar = a.drop_first();
        let br = b.drop_first();
        decimal_all_drop_first(a);
        decimal_all_drop_first(b);
        decimal_lex_value(ar, br);
        reveal(ckc_spec::v1text::all_in);
        assert(ckc_spec::v1text::is_digit_b(a[0]));
        assert(ckc_spec::v1text::is_digit_b(b[0]));
        decimal_value_prepend(a[0], ar);
        decimal_value_prepend(b[0], br);
        assert_seqs_equal!(a == seq![a[0]] + ar);
        assert_seqs_equal!(b == seq![b[0]] + br);
        decimal_value_bound(ar);
        decimal_value_bound(br);
        decimal_digit_bounds(a[0]);
        decimal_digit_bounds(b[0]);
        assert(ar.len() == br.len());
        let p = pow10(ar.len());
        pow10_positive(ar.len());
        let da = decimal_digit(a[0]);
        let db = decimal_digit(b[0]);
        let va = decimal_value(ar);
        let vb = decimal_value(br);
        assert(decimal_value(a) == da * p + va);
        assert(decimal_value(b) == db * p + vb);
        assert(va < p);
        assert(vb < p);
        if da < db {
            assert(da * p + va < db * p + vb) by (nonlinear_arith)
                requires
                    da + 1 <= db,
                    va < p,
                    vb >= 0,
                    p >= 0,
            ;
            assert(decimal_lex_lt(a, b));
        } else if da > db {
            assert(db * p + vb < da * p + va) by (nonlinear_arith)
                requires
                    db + 1 <= da,
                    vb < p,
                    va >= 0,
                    p >= 0,
            ;
            assert(!decimal_lex_lt(a, b));
        } else {
            assert(da == db);
            assert(decimal_lex_lt(a, b) == decimal_lex_lt(ar, br));
        }
    }
}

proof fn decimal_lex_difference(a: Seq<u8>, b: Seq<u8>, i: nat)
    requires
        a.len() == b.len(),
        ckc_spec::v1text::all_in(a, |x: u8| ckc_spec::v1text::is_digit_b(x)),
        ckc_spec::v1text::all_in(b, |x: u8| ckc_spec::v1text::is_digit_b(x)),
        i < a.len(),
        forall|j: int| 0 <= j < i ==> a[j] == b[j],
        a[i as int] != b[i as int],
    ensures
        decimal_lex_lt(a, b) == (decimal_digit(a[i as int]) < decimal_digit(b[i as int])),
    decreases i,
{
    reveal_with_fuel(decimal_lex_lt, 2);
    reveal(ckc_spec::v1text::all_in);
    reveal(decimal_digit);
    reveal(ckc_spec::v1text::is_digit_b);
    if i == 0 {
        assert(ckc_spec::v1text::is_digit_b(a[0]));
        assert(ckc_spec::v1text::is_digit_b(b[0]));
        assert(decimal_digit(a[0]) != decimal_digit(b[0]));
    } else {
        assert(a[0] == b[0]);
        assert(decimal_digit(a[0]) == decimal_digit(b[0]));
        decimal_all_drop_first(a);
        decimal_all_drop_first(b);
        assert(a.drop_first().len() == b.drop_first().len());
        assert(i - 1 < a.drop_first().len());
        assert forall|j: int| 0 <= j < i - 1 implies a.drop_first()[j] == b.drop_first()[j] by {
            assert(a.drop_first()[j] == a[j + 1]);
            assert(b.drop_first()[j] == b[j + 1]);
        }
        assert(a.drop_first()[(i - 1) as int] == a[i as int]);
        assert(b.drop_first()[(i - 1) as int] == b[i as int]);
        decimal_lex_difference(a.drop_first(), b.drop_first(), (i - 1) as nat);
        assert(decimal_lex_lt(a, b) == decimal_lex_lt(a.drop_first(), b.drop_first()));
    }
}

proof fn decimal_lex_irreflexive(a: Seq<u8>)
    ensures
        !decimal_lex_lt(a, a),
    decreases a.len(),
{
    reveal(decimal_lex_lt);
    if a.len() > 0 {
        decimal_lex_irreflexive(a.drop_first());
    }
}

fn decimal_bytes_less(a: &Vec<u8>, b: &Vec<u8>) -> (r: bool)
    requires
        canonical_decimal(a@),
        canonical_decimal(b@),
    ensures
        r == (decimal_value(a@) < decimal_value(b@)),
{
    if a.len() < b.len() {
        proof {
            decimal_shorter_less(a@, b@);
        }
        return true;
    }
    if a.len() > b.len() {
        proof {
            decimal_shorter_less(b@, a@);
            assert(!(decimal_value(a@) < decimal_value(b@)));
        }
        return false;
    }
    let mut i = 0usize;
    while i < a.len() && a[i] == b[i]
        invariant
            a@.len() == b@.len(),
            i <= a@.len(),
            forall|j: int| 0 <= j < i ==> a@[j] == b@[j],
        decreases a.len() - i,
    {
        i += 1;
    }
    proof {
        reveal(canonical_decimal);
        decimal_lex_value(a@, b@);
    }
    if i == a.len() {
        proof {
            assert_seqs_equal!(a@ == b@);
            decimal_lex_irreflexive(a@);
        }
        false
    } else {
        proof {
            reveal(canonical_decimal);
            reveal(ckc_spec::v1text::all_in);
            assert(a@[i as int] != b@[i as int]);
            decimal_lex_difference(a@, b@, i as nat);
            reveal(decimal_digit);
            reveal(ckc_spec::v1text::is_digit_b);
            assert(ckc_spec::v1text::is_digit_b(a@[i as int]));
            assert(ckc_spec::v1text::is_digit_b(b@[i as int]));
            assert((a@[i as int] < b@[i as int]) == (decimal_digit(a@[i as int]) < decimal_digit(
                b@[i as int],
            )));
        }
        a[i] < b[i]
    }
}

pub enum EOrder {
    Less,
    Equal,
    Greater,
}

pub open spec fn bytes_order_ok(order: &EOrder, a: Seq<u8>, b: Seq<u8>) -> bool {
    match order {
        EOrder::Less => ckc_spec::engine::bytes_lt(a, b),
        EOrder::Equal => a == b,
        EOrder::Greater => a != b && !ckc_spec::engine::bytes_lt(a, b),
    }
}

proof fn bytes_lt_skip_step(a: Seq<u8>, b: Seq<u8>, i: nat)
    requires
        i < a.len(),
        i < b.len(),
        a[i as int] == b[i as int],
    ensures
        ckc_spec::engine::bytes_lt(a.skip(i as int), b.skip(i as int))
            == ckc_spec::engine::bytes_lt(a.skip(i as int + 1), b.skip(i as int + 1)),
{
    reveal_with_fuel(ckc_spec::engine::bytes_lt, 2);
    assert(a.skip(i as int)[0] == a[i as int]);
    assert(b.skip(i as int)[0] == b[i as int]);
    assert_seqs_equal!(a.skip(i as int).drop_first() == a.skip(i as int + 1));
    assert_seqs_equal!(b.skip(i as int).drop_first() == b.skip(i as int + 1));
}

fn bytes_order(a: &Vec<u8>, b: &Vec<u8>) -> (order: EOrder)
    ensures
        bytes_order_ok(&order, a@, b@),
{
    let mut i = 0usize;
    proof {
        assert_seqs_equal!(a@.skip(0) == a@);
        assert_seqs_equal!(b@.skip(0) == b@);
        assert_seqs_equal!(a@.take(0) == Seq::<u8>::empty());
        assert_seqs_equal!(b@.take(0) == Seq::<u8>::empty());
    }
    while i < a.len() && i < b.len() && a[i] == b[i]
        invariant
            i <= a@.len(),
            i <= b@.len(),
            a@.take(i as int) == b@.take(i as int),
            ckc_spec::engine::bytes_lt(a@.skip(i as int), b@.skip(i as int))
                == ckc_spec::engine::bytes_lt(a@, b@),
        decreases a.len() - i,
    {
        proof {
            bytes_lt_skip_step(a@, b@, i as nat);
        }
        i += 1;
    }
    if i == a.len() {
        if i == b.len() {
            proof {
                assert_seqs_equal!(a@.take(i as int) == a@);
                assert_seqs_equal!(b@.take(i as int) == b@);
                assert_seqs_equal!(a@ == b@);
                reveal(bytes_order_ok);
            }
            EOrder::Equal
        } else {
            proof {
                reveal_with_fuel(ckc_spec::engine::bytes_lt, 2);
                assert(a@.skip(i as int).len() == 0);
                assert(b@.skip(i as int).len() > 0);
                reveal(bytes_order_ok);
            }
            EOrder::Less
        }
    } else if i == b.len() {
        proof {
            reveal_with_fuel(ckc_spec::engine::bytes_lt, 2);
            assert(a@.skip(i as int).len() > 0);
            assert(b@.skip(i as int).len() == 0);
            reveal(bytes_order_ok);
        }
        EOrder::Greater
    } else if a[i] < b[i] {
        proof {
            reveal_with_fuel(ckc_spec::engine::bytes_lt, 2);
            assert(a@.skip(i as int)[0] == a@[i as int]);
            assert(b@.skip(i as int)[0] == b@[i as int]);
            assert(a@ != b@);
            reveal(bytes_order_ok);
        }
        EOrder::Less
    } else {
        proof {
            assert(a@[i as int] != b@[i as int]);
            assert(a@[i as int] > b@[i as int]);
            reveal_with_fuel(ckc_spec::engine::bytes_lt, 2);
            assert(a@.skip(i as int)[0] == a@[i as int]);
            assert(b@.skip(i as int)[0] == b@[i as int]);
            assert(a@ != b@);
            reveal(bytes_order_ok);
        }
        EOrder::Greater
    }
}

pub open spec fn int_order_ok(order: &EOrder, a: int, b: int) -> bool {
    match order {
        EOrder::Less => a < b,
        EOrder::Equal => a == b,
        EOrder::Greater => a > b,
    }
}

fn magnitude_order(a: &Vec<u8>, b: &Vec<u8>) -> (order: EOrder)
    requires
        canonical_decimal(a@),
        canonical_decimal(b@),
    ensures
        int_order_ok(&order, decimal_value(a@) as int, decimal_value(b@) as int),
{
    if decimal_bytes_less(a, b) {
        proof {
            reveal(int_order_ok);
        }
        EOrder::Less
    } else if decimal_bytes_less(b, a) {
        proof {
            reveal(int_order_ok);
        }
        EOrder::Greater
    } else {
        proof {
            reveal(int_order_ok);
        }
        EOrder::Equal
    }
}

pub fn int_order(
    a_magnitude: &Vec<u8>,
    a_negative: bool,
    a: Ghost<int>,
    b_magnitude: &Vec<u8>,
    b_negative: bool,
    b: Ghost<int>,
) -> (order: EOrder)
    requires
        a_negative == (a@ < 0),
        b_negative == (b@ < 0),
        a_magnitude@ == ckc_spec::v1text::udec_bytes(
            if a@ < 0 {
                (-a@) as nat
            } else {
                a@ as nat
            },
        ),
        b_magnitude@ == ckc_spec::v1text::udec_bytes(
            if b@ < 0 {
                (-b@) as nat
            } else {
                b@ as nat
            },
        ),
    ensures
        int_order_ok(&order, a@, b@),
{
    let ghost av = if a@ < 0 {
        (-a@) as nat
    } else {
        a@ as nat
    };
    let ghost bv = if b@ < 0 {
        (-b@) as nat
    } else {
        b@ as nat
    };
    proof {
        udec_canonical(av);
        udec_canonical(bv);
        udec_decimal_value(av);
        udec_decimal_value(bv);
    }
    if a_negative && !b_negative {
        proof {
            reveal(int_order_ok);
        }
        EOrder::Less
    } else if !a_negative && b_negative {
        proof {
            reveal(int_order_ok);
        }
        EOrder::Greater
    } else if !a_negative {
        let order = magnitude_order(a_magnitude, b_magnitude);
        proof {
            reveal(int_order_ok);
            match order {
                EOrder::Less => {},
                EOrder::Equal => {},
                EOrder::Greater => {},
            }
        }
        order
    } else {
        let magnitude = magnitude_order(b_magnitude, a_magnitude);
        let order = match magnitude {
            EOrder::Less => EOrder::Less,
            EOrder::Equal => EOrder::Equal,
            EOrder::Greater => EOrder::Greater,
        };
        proof {
            reveal(int_order_ok);
            match magnitude {
                EOrder::Less => {},
                EOrder::Equal => {},
                EOrder::Greater => {},
            }
        }
        order
    }
}

proof fn bytes_lt_irreflexive(bytes: Seq<u8>)
    ensures
        !ckc_spec::engine::bytes_lt(bytes, bytes),
    decreases bytes.len(),
{
    reveal_with_fuel(ckc_spec::engine::bytes_lt, 2);
    if bytes.len() > 0 {
        bytes_lt_irreflexive(bytes.drop_first());
    }
}

pub proof fn term_lt_irreflexive(term: Term)
    ensures
        !ckc_spec::engine::term_lt(term, term),
    decreases term,
{
    reveal_with_fuel(ckc_spec::engine::term_lt, 2);
    reveal(ckc_spec::engine::rank);
    match term {
        Term::Atom(name) => bytes_lt_irreflexive(name),
        Term::Comp(_, args) => args_lt_irreflexive(args),
        _ => {},
    }
}

proof fn args_lt_irreflexive(args: Seq<Term>)
    ensures
        !ckc_spec::engine::args_lt(args, args),
    decreases args,
{
    reveal_with_fuel(ckc_spec::engine::args_lt, 2);
    if args.len() > 0 {
        term_lt_irreflexive(args[0]);
        args_lt_irreflexive(args.drop_first());
    }
}

pub enum ECmpTask {
    Terms { left: usize, right: usize, left_model: Ghost<Term>, right_model: Ghost<Term> },
    Args {
        left_parent: usize,
        right_parent: usize,
        next: usize,
        left_args: Ghost<Seq<Term>>,
        right_args: Ghost<Seq<Term>>,
    },
}

pub open spec fn cmp_task_ok(nodes: Seq<ENode>, task: &ECmpTask) -> bool {
    match task {
        ECmpTask::Terms { left, right, left_model, right_model } => {
            &&& *left < nodes.len()
            &&& *right < nodes.len()
            &&& left_model@ == nodes[*left as int].term@
            &&& right_model@ == nodes[*right as int].term@
            &&& ckc_spec::term::ground(left_model@)
            &&& ckc_spec::term::ground(right_model@)
        },
        ECmpTask::Args { left_parent, right_parent, next, left_args, right_args } => {
            &&& *left_parent < nodes.len()
            &&& *right_parent < nodes.len()
            &&& match (&nodes[*left_parent as int].kind, &nodes[*right_parent as int].kind) {
                (
                    ENodeKind::Comp { child_roots: left_roots, .. },
                    ENodeKind::Comp { child_roots: right_roots, .. },
                ) => {
                    &&& *next < left_roots@.len()
                    &&& left_roots@.len() == right_roots@.len()
                    &&& child_roots_valid(nodes, left_roots@)
                    &&& child_roots_valid(nodes, right_roots@)
                    &&& left_args@ == child_terms(nodes, left_roots@.skip(*next as int))
                    &&& right_args@ == child_terms(nodes, right_roots@.skip(*next as int))
                    &&& ckc_spec::term::ground_all(left_args@)
                    &&& ckc_spec::term::ground_all(right_args@)
                },
                _ => false,
            }
        },
    }
}

pub open spec fn cmp_tasks_ok(nodes: Seq<ENode>, tasks: Seq<ECmpTask>) -> bool
    decreases tasks.len(),
{
    tasks.len() == 0 || (cmp_task_ok(nodes, &tasks.last()) && cmp_tasks_ok(
        nodes,
        tasks.drop_last(),
    ))
}

pub open spec fn cmp_task_equal(task: &ECmpTask) -> bool {
    match task {
        ECmpTask::Terms { left_model, right_model, .. } => left_model@ == right_model@,
        ECmpTask::Args { left_args, right_args, .. } => left_args@ == right_args@,
    }
}

pub open spec fn cmp_task_less(task: &ECmpTask) -> bool {
    match task {
        ECmpTask::Terms { left_model, right_model, .. } => {
            ckc_spec::engine::term_lt(left_model@, right_model@)
        },
        ECmpTask::Args { left_args, right_args, .. } => {
            ckc_spec::engine::args_lt(left_args@, right_args@)
        },
    }
}

pub open spec fn cmp_pending(tasks: Seq<ECmpTask>) -> bool
    decreases tasks.len(),
{
    if tasks.len() == 0 {
        false
    } else if cmp_task_equal(&tasks.last()) {
        cmp_pending(tasks.drop_last())
    } else {
        cmp_task_less(&tasks.last())
    }
}

pub open spec fn pair_work(a: Term, b: Term) -> nat
    decreases a,
{
    match (a, b) {
        (Term::Comp(an, aa), Term::Comp(bn, ba)) => {
            if an == bn && aa.len() == ba.len() {
                1 + arg_pair_work(aa, ba)
            } else {
                1
            }
        },
        _ => 1,
    }
}

pub open spec fn arg_pair_work(a: Seq<Term>, b: Seq<Term>) -> nat
    decreases a,
{
    if a.len() == 0 || b.len() == 0 {
        0
    } else {
        1 + pair_work(a[0], b[0]) + arg_pair_work(a.drop_first(), b.drop_first())
    }
}

pub open spec fn cmp_task_work(task: &ECmpTask) -> nat {
    match task {
        ECmpTask::Terms { left_model, right_model, .. } => { pair_work(left_model@, right_model@) },
        ECmpTask::Args { left_args, right_args, .. } => { arg_pair_work(left_args@, right_args@) },
    }
}

pub open spec fn cmp_tasks_work(tasks: Seq<ECmpTask>) -> nat
    decreases tasks.len(),
{
    if tasks.len() == 0 {
        0
    } else {
        cmp_task_work(&tasks.last()) + cmp_tasks_work(tasks.drop_last())
    }
}

proof fn cmp_tasks_ok_push(nodes: Seq<ENode>, tasks: Seq<ECmpTask>, task: ECmpTask)
    requires
        cmp_tasks_ok(nodes, tasks),
        cmp_task_ok(nodes, &task),
    ensures
        cmp_tasks_ok(nodes, tasks.push(task)),
{
    assert(tasks.push(task).last() == task);
    assert(tasks.push(task).drop_last() == tasks);
    reveal_with_fuel(cmp_tasks_ok, 2);
}

proof fn cmp_pending_push(tasks: Seq<ECmpTask>, task: ECmpTask)
    ensures
        cmp_pending(tasks.push(task)) == if cmp_task_equal(&task) {
            cmp_pending(tasks)
        } else {
            cmp_task_less(&task)
        },
{
    assert(tasks.push(task).last() == task);
    assert(tasks.push(task).drop_last() == tasks);
    reveal_with_fuel(cmp_pending, 2);
}

proof fn cmp_tasks_work_push(tasks: Seq<ECmpTask>, task: ECmpTask)
    ensures
        cmp_tasks_work(tasks.push(task)) == cmp_task_work(&task) + cmp_tasks_work(tasks),
{
    assert(tasks.push(task).last() == task);
    assert(tasks.push(task).drop_last() == tasks);
    reveal_with_fuel(cmp_tasks_work, 2);
}

fn node_rank(arena: &ETermArena, index: usize) -> (rank: u8)
    requires
        arena_ok(arena),
        index < arena.nodes@.len(),
    ensures
        rank as int == ckc_spec::engine::rank(arena@[index as int]),
{
    proof {
        reveal(arena_ok);
        assert(node_ok(arena.nodes@, index as int));
        reveal(node_ok);
    }
    let rank = match &arena.nodes[index].kind {
        ENodeKind::Var { .. } => 0,
        ENodeKind::Int { .. } => 1,
        ENodeKind::Nil => 2,
        ENodeKind::Atom { .. } => 3,
        ENodeKind::Comp { .. } => 4,
    };
    proof {
        reveal(ckc_spec::engine::rank);
    }
    rank
}

#[verifier::rlimit(5000)]
fn term_lt_inner(
    arena: &ETermArena,
    left_root_index: usize,
    right_root_index: usize,
    left_expected: Ghost<Term>,
    right_expected: Ghost<Term>,
) -> (less: bool)
    requires
        root_ok(arena, left_root_index),
        root_ok(arena, right_root_index),
        left_expected@ == arena@[left_root_index as int],
        right_expected@ == arena@[right_root_index as int],
        ckc_spec::term::ground(left_expected@),
        ckc_spec::term::ground(right_expected@),
    ensures
        less == ckc_spec::engine::term_lt(left_expected@, right_expected@),
{
    let node_count = arena.nodes.len();
    proof {
        assert(arena.nodes@.len() == node_count as int);
        assert(arena.nodes@.len() <= usize::MAX as int);
    }
    let mut tasks = Vec::new();
    let first = ECmpTask::Terms {
        left: left_root_index,
        right: right_root_index,
        left_model: Ghost(left_expected@),
        right_model: Ghost(right_expected@),
    };
    proof {
        reveal(root_ok);
        assert(cmp_task_ok(arena.nodes@, &first));
        cmp_tasks_ok_push(arena.nodes@, Seq::empty(), first);
        cmp_pending_push(Seq::empty(), first);
        if left_expected@ == right_expected@ {
            term_lt_irreflexive(left_expected@);
        }
        reveal(cmp_pending);
    }
    tasks.push(first);
    while tasks.len() > 0
        invariant
            arena_ok(arena),
            arena.nodes@.len() == node_count as int,
            cmp_tasks_ok(arena.nodes@, tasks@),
            cmp_pending(tasks@) == ckc_spec::engine::term_lt(left_expected@, right_expected@),
        decreases cmp_tasks_work(tasks@),
    {
        let ghost before = tasks@;
        let task = tasks.pop().unwrap();
        let ghost current_equal = cmp_task_equal(&task);
        let ghost current_less = cmp_task_less(&task);
        proof {
            reveal_with_fuel(cmp_tasks_ok, 2);
            reveal_with_fuel(cmp_pending, 2);
            reveal_with_fuel(cmp_tasks_work, 2);
            assert(task == before.last());
            assert(tasks@ == before.drop_last());
            assert_seqs_equal!(before == tasks@.push(task));
            assert(cmp_task_ok(arena.nodes@, &task));
            cmp_pending_push(tasks@, task);
            cmp_tasks_work_push(tasks@, task);
            if !current_equal {
                assert(current_less == ckc_spec::engine::term_lt(left_expected@, right_expected@));
            }
        }
        match task {
            ECmpTask::Args { left_parent, right_parent, next, left_args, right_args } => {
                proof {
                    reveal(cmp_task_ok);
                }
                match (&arena.nodes[left_parent].kind, &arena.nodes[right_parent].kind) {
                    (
                        ENodeKind::Comp { child_roots: left_roots, .. },
                        ENodeKind::Comp { child_roots: right_roots, .. },
                    ) => {
                        proof {
                            reveal(child_roots_valid);
                            child_terms_skip_step(arena.nodes@, left_roots@, next);
                            child_terms_skip_step(arena.nodes@, right_roots@, next);
                            assert(next < left_roots.len());
                            reveal_with_fuel(ckc_spec::term::ground_all, 2);
                            assert(left_args@.len() == left_roots@.len() - next);
                            assert(right_args@.len() == right_roots@.len() - next);
                        }
                        let pair = ECmpTask::Terms {
                            left: left_roots[next],
                            right: right_roots[next],
                            left_model: Ghost(left_args@[0]),
                            right_model: Ghost(right_args@[0]),
                        };
                        let last = left_roots.len() - 1;
                        if next == last {
                            proof {
                                assert(next + 1 == left_roots@.len());
                                assert(next + 1 == right_roots@.len());
                                assert(left_args@.len() == 1);
                                assert(right_args@.len() == 1);
                                assert(cmp_task_ok(arena.nodes@, &pair));
                                cmp_tasks_ok_push(arena.nodes@, tasks@, pair);
                                cmp_pending_push(tasks@, pair);
                                cmp_tasks_work_push(tasks@, pair);
                                reveal_with_fuel(ckc_spec::engine::args_lt, 2);
                                reveal(arg_pair_work);
                            }
                            tasks.push(pair);
                        } else {
                            let rest = ECmpTask::Args {
                                left_parent,
                                right_parent,
                                next: next + 1,
                                left_args: Ghost(left_args@.drop_first()),
                                right_args: Ghost(right_args@.drop_first()),
                            };
                            proof {
                                assert(next < last);
                                assert(next + 1 < left_roots@.len());
                                assert(next + 1 < right_roots@.len());
                                assert_seqs_equal!(left_args@.drop_first()
                                    == child_terms(
                                        arena.nodes@,
                                        left_roots@.skip(next as int + 1),
                                    ));
                                assert_seqs_equal!(right_args@.drop_first()
                                    == child_terms(
                                        arena.nodes@,
                                        right_roots@.skip(next as int + 1),
                                    ));
                                assert(cmp_task_ok(arena.nodes@, &rest));
                                assert(cmp_task_ok(arena.nodes@, &pair));
                                cmp_tasks_ok_push(arena.nodes@, tasks@, rest);
                                cmp_pending_push(tasks@, rest);
                                cmp_tasks_work_push(tasks@, rest);
                            }
                            tasks.push(rest);
                            proof {
                                cmp_tasks_ok_push(
                                    arena.nodes@,
                                    before.drop_last().push(rest),
                                    pair,
                                );
                                cmp_pending_push(before.drop_last().push(rest), pair);
                                cmp_tasks_work_push(before.drop_last().push(rest), pair);
                                reveal_with_fuel(ckc_spec::engine::args_lt, 2);
                                reveal(arg_pair_work);
                            }
                            tasks.push(pair);
                        }
                    },
                    _ => {
                        proof {
                            assert(false);
                        }
                    },
                }
            },
            ECmpTask::Terms { left, right, left_model, right_model } => {
                proof {
                    reveal(cmp_task_ok);
                    reveal(cmp_task_equal);
                    reveal(cmp_task_less);
                    reveal(arena_ok);
                    assert(node_ok(arena.nodes@, left as int));
                    assert(node_ok(arena.nodes@, right as int));
                    assert(current_equal == (left_model@ == right_model@));
                    assert(current_less == ckc_spec::engine::term_lt(left_model@, right_model@));
                }
                let left_rank = node_rank(arena, left);
                let right_rank = node_rank(arena, right);
                if left_rank < right_rank {
                    proof {
                        assert(left_model@ != right_model@);
                        reveal(ckc_spec::engine::term_lt);
                        reveal(cmp_pending);
                        assert(current_less);
                        assert(ckc_spec::engine::term_lt(left_expected@, right_expected@));
                        assert(ckc_spec::engine::term_lt(left_expected@, right_expected@));
                    }
                    return true;
                }
                if left_rank > right_rank {
                    proof {
                        assert(left_model@ != right_model@);
                        reveal(ckc_spec::engine::term_lt);
                        reveal(cmp_pending);
                        assert(!current_less);
                        assert(!ckc_spec::engine::term_lt(left_expected@, right_expected@));
                        assert(!ckc_spec::engine::term_lt(left_expected@, right_expected@));
                    }
                    return false;
                }
                match (&arena.nodes[left].kind, &arena.nodes[right].kind) {
                    (ENodeKind::Var { .. }, _) | (_, ENodeKind::Var { .. }) => {
                        proof {
                            reveal(node_ok);
                            reveal(ckc_spec::term::ground);
                            assert(false);
                        }
                        return false;
                    },
                    (
                        ENodeKind::Int {
                            magnitude: left_magnitude,
                            negative: left_negative,
                            value: left_value,
                            ..
                        },
                        ENodeKind::Int {
                            magnitude: right_magnitude,
                            negative: right_negative,
                            value: right_value,
                            ..
                        },
                    ) => {
                        proof {
                            reveal(node_ok);
                        }
                        let order = int_order(
                            left_magnitude,
                            *left_negative,
                            *left_value,
                            right_magnitude,
                            *right_negative,
                            *right_value,
                        );
                        match order {
                            EOrder::Less => {
                                proof {
                                    reveal(int_order_ok);
                                    reveal(ckc_spec::engine::term_lt);
                                    reveal(cmp_pending);
                                    assert(left_model@ != right_model@);
                                    assert(current_less);
                                    assert(ckc_spec::engine::term_lt(
                                        left_expected@,
                                        right_expected@,
                                    ));
                                }
                                return true;
                            },
                            EOrder::Greater => {
                                proof {
                                    reveal(int_order_ok);
                                    reveal(ckc_spec::engine::term_lt);
                                    reveal(cmp_pending);
                                    assert(left_model@ != right_model@);
                                    assert(!current_less);
                                    assert(!ckc_spec::engine::term_lt(
                                        left_expected@,
                                        right_expected@,
                                    ));
                                }
                                return false;
                            },
                            EOrder::Equal => {
                                proof {
                                    reveal(int_order_ok);
                                    reveal(ckc_spec::engine::term_lt);
                                    reveal(cmp_pending);
                                    reveal(pair_work);
                                }
                            },
                        }
                    },
                    (ENodeKind::Nil, ENodeKind::Nil) => {
                        proof {
                            reveal(node_ok);
                            reveal(ckc_spec::engine::term_lt);
                            reveal(cmp_pending);
                            reveal(pair_work);
                        }
                    },
                    (ENodeKind::Atom { name: left_name }, ENodeKind::Atom { name: right_name }) => {
                        proof {
                            reveal(node_ok);
                        }
                        let order = bytes_order(left_name, right_name);
                        match order {
                            EOrder::Less => {
                                proof {
                                    reveal(bytes_order_ok);
                                    reveal(ckc_spec::engine::term_lt);
                                    reveal(cmp_pending);
                                    bytes_lt_irreflexive(left_name@);
                                    assert(left_name@ != right_name@);
                                    assert(left_model@ != right_model@);
                                    assert(current_less);
                                    assert(ckc_spec::engine::term_lt(
                                        left_expected@,
                                        right_expected@,
                                    ));
                                }
                                return true;
                            },
                            EOrder::Greater => {
                                proof {
                                    reveal(bytes_order_ok);
                                    reveal(ckc_spec::engine::term_lt);
                                    reveal(cmp_pending);
                                    assert(left_model@ != right_model@);
                                    assert(!current_less);
                                    assert(!ckc_spec::engine::term_lt(
                                        left_expected@,
                                        right_expected@,
                                    ));
                                }
                                return false;
                            },
                            EOrder::Equal => {
                                proof {
                                    reveal(bytes_order_ok);
                                    reveal(ckc_spec::engine::term_lt);
                                    reveal(cmp_pending);
                                    reveal(pair_work);
                                }
                            },
                        }
                    },
                    (
                        ENodeKind::Comp { name: left_name, child_roots: left_roots, .. },
                        ENodeKind::Comp { name: right_name, child_roots: right_roots, .. },
                    ) => {
                        proof {
                            reveal(node_ok);
                        }
                        if left_roots.len() < right_roots.len() {
                            proof {
                                reveal(ckc_spec::engine::term_lt);
                                reveal(cmp_pending);
                                assert(left_model@ != right_model@);
                                assert(current_less);
                                assert(ckc_spec::engine::term_lt(left_expected@, right_expected@));
                            }
                            return true;
                        }
                        if left_roots.len() > right_roots.len() {
                            proof {
                                reveal(ckc_spec::engine::term_lt);
                                reveal(cmp_pending);
                                assert(left_model@ != right_model@);
                                assert(!current_less);
                                assert(!ckc_spec::engine::term_lt(left_expected@, right_expected@));
                            }
                            return false;
                        }
                        let name_order = bytes_order(left_name, right_name);
                        match name_order {
                            EOrder::Less => {
                                proof {
                                    reveal(bytes_order_ok);
                                    reveal(ckc_spec::engine::term_lt);
                                    reveal(cmp_pending);
                                    bytes_lt_irreflexive(left_name@);
                                    assert(left_name@ != right_name@);
                                    assert(left_model@ != right_model@);
                                    assert(current_less);
                                    assert(ckc_spec::engine::term_lt(
                                        left_expected@,
                                        right_expected@,
                                    ));
                                }
                                return true;
                            },
                            EOrder::Greater => {
                                proof {
                                    reveal(bytes_order_ok);
                                    reveal(ckc_spec::engine::term_lt);
                                    reveal(cmp_pending);
                                    assert(left_model@ != right_model@);
                                    assert(!current_less);
                                    assert(!ckc_spec::engine::term_lt(
                                        left_expected@,
                                        right_expected@,
                                    ));
                                }
                                return false;
                            },
                            EOrder::Equal => {
                                let ghost left_args = match left_model@ {
                                    Term::Comp(_, args) => args,
                                    _ => Seq::empty(),
                                };
                                let ghost right_args = match right_model@ {
                                    Term::Comp(_, args) => args,
                                    _ => Seq::empty(),
                                };
                                let args = ECmpTask::Args {
                                    left_parent: left,
                                    right_parent: right,
                                    next: 0,
                                    left_args: Ghost(left_args),
                                    right_args: Ghost(right_args),
                                };
                                proof {
                                    reveal(bytes_order_ok);
                                    reveal(ckc_spec::engine::term_lt);
                                    reveal(cmp_pending);
                                    reveal(pair_work);
                                    reveal(ckc_spec::term::ground);
                                    assert(left_roots@.len() > 0);
                                    assert(right_roots@.len() > 0);
                                    assert_seqs_equal!(left_roots@.skip(0) == left_roots@);
                                    assert_seqs_equal!(right_roots@.skip(0) == right_roots@);
                                    assert(cmp_task_ok(arena.nodes@, &args));
                                    cmp_tasks_ok_push(arena.nodes@, tasks@, args);
                                    cmp_pending_push(tasks@, args);
                                    cmp_tasks_work_push(tasks@, args);
                                }
                                tasks.push(args);
                            },
                        }
                    },
                    _ => {
                        proof {
                            reveal(node_ok);
                            assert(false);
                        }
                        return false;
                    },
                }
            },
        }
    }
    proof {
        reveal(cmp_pending);
        assert(!ckc_spec::engine::term_lt(left_expected@, right_expected@));
    }
    false
}

pub fn term_lt(arena: &ETermArena, left: usize, right: usize) -> (less: bool)
    requires
        root_ok(arena, left),
        root_ok(arena, right),
        ckc_spec::term::ground(arena@[left as int]),
        ckc_spec::term::ground(arena@[right as int]),
    ensures
        less == ckc_spec::engine::term_lt(arena@[left as int], arena@[right as int]),
{
    term_lt_inner(arena, left, right, Ghost(arena@[left as int]), Ghost(arena@[right as int]))
}

} // verus!
