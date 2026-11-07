use crate::*;

verus! {

broadcast use vstd::iset::group_iset_lemmas;
// p(a), q(b) cannot jointly witness exists X. p(X) and q(X).

pub proof fn correlated_naf_control() {
    let r = Instance {
        head: 0,
        positive: Set::empty(),
        naf: seq![seq![set![1int, 2int], set![3int, 4int]]],
        builtins: true,
    };
    assert(r.naf[0][0] == set![1int, 2int]);
    assert(r.naf[0][1] == set![3int, 4int]);
    assert(r.naf[0][0].contains(2int));
    assert(r.naf[0][1].contains(3int));
    assert(!set![1int, 4int].contains(2int));
    assert(!set![1int, 4int].contains(3int));
    assert(!r.naf[0][0].subset_of(set![1int, 4int]));
    assert(!r.naf[0][1].subset_of(set![1int, 4int]));
    assert(retained(r, set![1int, 4int]));
    assert(r.naf[0][0].subset_of(set![1int, 2int]));
    assert(!retained(r, set![1int, 2int]));
    let empty_witness = Instance {
        head: 0,
        positive: Set::empty(),
        naf: seq![seq![Set::<Atom>::empty()]],
        builtins: true,
    };
    assert(empty_witness.naf[0][0].subset_of(Set::<Atom>::empty()));
    assert(!retained(empty_witness, Set::empty()));
    let no_witnesses = Instance {
        head: 0,
        positive: Set::empty(),
        naf: seq![Seq::<Set<Atom>>::empty()],
        builtins: true,
    };
    assert(retained(no_witnesses, Set::empty()));
}

pub proof fn support_and_closure_control() {
    let fact = Instance { head: 1, positive: Set::empty(), naf: Seq::empty(), builtins: true };
    let p = Program { atoms: set![1int, 2int], rules: seq![fact] };
    let m = set![1int];
    let level = |_: Atom| 0nat;
    assert(wf(p));
    assert(p.rules[0].head == 1);
    assert(retained(p.rules[0], m));
    assert(founded(p, m, level));
    assert(closed(p, m, m));
    support::support_closure_lfp(p, m, level);
    assert(stable(p, m));
    assert(closed(p, set![1int, 2int], set![1int, 2int]));
    assert(!founded(p, set![1int, 2int], level)) by {
        if founded(p, set![1int, 2int], level) {
            assert(set![1int, 2int].contains(2int));
            assert(exists|i: int| 0 <= i < p.rules.len() && p.rules[i].head == 2);
        }
    }
    assert(founded(p, Set::empty(), level));
    assert(!closed(p, Set::empty(), Set::empty()));
}

pub proof fn growth_loop_control() {
    let g = rank::Graph { nodes: set![0int], positive: Set::empty(), negative: set![(0int, 0int)] };
    assert(rank::graph_wf(g));
    assert(rank::reach(g, 0).contains(0));
    assert(g.negative.contains((0int, 0int)));
    assert(!rank::no_negative_cycle(g));
    let s = grounding::Signature {
        base: set![grounding::Value::Scalar(7)],
        operations: seq![grounding::Operation { symbol: 10, output: 0, inputs: seq![0int] }],
        flow: g,
    };
    assert(!grounding::signature_wf(s));
}

// A genuine two-constructor chain admits a nested ground identity. This gives
// a concrete inhabited instance of every growth/substitution hypothesis.
pub proof fn nested_identity_control() {
    let g = rank::Graph {
        nodes: set![0int, 1int, 2int],
        positive: Set::empty(),
        negative: set![(1int, 0int), (2int, 1int)],
    };
    assert(rank::graph_wf(g));
    assert(rank::forward_closed(g, set![0int]));
    assert(rank::forward_closed(g, set![0int, 1int]));
    assert(!rank::reach(g, 0).contains(1));
    assert(!rank::reach(g, 1).contains(2));
    assert(rank::no_negative_cycle(g));
    let s = grounding::Signature {
        base: set![grounding::Value::Scalar(7)],
        operations: seq![
            grounding::Operation { symbol: 10, output: 1, inputs: seq![0int] },
            grounding::Operation { symbol: 20, output: 2, inputs: seq![1int] },
        ],
        flow: g,
    };
    let eval = |id: int, args: Seq<grounding::Value>|
        grounding::identity_eval(s.operations, id, args);
    assert(grounding::signature_wf(s));
    grounding::universe_monotone(s, eval, 0, rank::rank(g, 0));
    assert(grounding::type_values(s, eval, 0).contains(grounding::Value::Scalar(7)));
    grounding::substitution_closed(s, eval, 0, seq![grounding::Value::Scalar(7)]);
    let inner = grounding::Value::Identity(10, seq![grounding::Value::Scalar(7)]);
    assert(grounding::type_values(s, eval, 1).contains(inner));
    let c = grounding::Clause {
        variables: seq![1int],
        terms: seq![grounding::Template::Apply(1, seq![0nat])],
    };
    let valuation = seq![inner];
    assert(grounding::clause_wf(s, c));
    assert(grounding::valuation_ok(s, c, eval, valuation));
    grounding::nested_identity_substitution(s, c, valuation, 1, seq![0nat]);
    let nested = grounding::Value::Identity(20, seq![inner]);
    assert(seq![0nat].map_values(|v: nat| valuation[v as int]) =~= seq![inner]);
    assert(grounding::substitute(grounding::Template::Apply(1, seq![0nat]), valuation, eval)
        == nested);
    assert(grounding::global_values(s, eval).contains(nested));
    grounding::finite_grounding(s, seq![c], eval);
    assert(grounding::instantiate(c, valuation, eval) =~= seq![nested]);
    assert(seq![c][0] == c);
    assert(grounding::valuation_ok(s, seq![c][0], eval, valuation));
    assert(seq![nested] == grounding::instantiate(seq![c][0], valuation, eval));
    let item = (0int, seq![nested]);
    assert(item.1 == grounding::instantiate(seq![c][item.0], valuation, eval));
    assert(exists|v: Seq<grounding::Value>|
        0 <= item.0 < seq![c].len() && grounding::valuation_ok(s, seq![c][item.0], eval, v)
            && item.1 == grounding::instantiate(seq![c][item.0], v, eval));
    assert(grounding::all_legal_instances(s, seq![c], eval).contains(item));
}

// Stratified NAF is total; an odd negative self-loop remains undefined in the
// independent AFP semantics, so totality is not built into the WFM definition.
pub proof fn well_founded_controls() {
    let fact = Instance { head: 1, positive: Set::empty(), naf: Seq::empty(), builtins: true };
    let negative = Instance {
        head: 0,
        positive: Set::empty(),
        naf: seq![seq![set![2int]]],
        builtins: true,
    };
    let p = Program { atoms: set![0int, 1int, 2int], rules: seq![fact, negative] };
    let m = set![0int, 1int];
    let r = |a: Atom|
        if a == 0 {
            1nat
        } else {
            0nat
        };
    let level = |_: Atom| 0nat;
    assert(wf(p));
    assert(rank::local_stratification(p, r));
    assert(retained(p.rules[0], m));
    assert(p.rules[1].naf[0][0] == set![2int]);
    assert(set![2int].contains(2int));
    assert(!m.contains(2int));
    assert(!set![2int].subset_of(m));
    assert(retained(p.rules[1], m));
    assert(founded(p, m, level));
    assert(closed(p, m, m));
    afp::certified_is_wfm(p, r, m, level);
    assert(afp::lower(p) == m);
    assert(afp::upper(p) == m);
    assert(afp::false_atoms(p) =~= set![2int]);

    let odd = Instance {
        head: 0,
        positive: Set::empty(),
        naf: seq![seq![set![0int]]],
        builtins: true,
    };
    let q = Program { atoms: set![0int], rules: seq![odd] };
    let empty = Set::<Atom>::empty();
    let singleton = set![0int];
    assert(wf(q));
    assert(q.rules[0].naf[0][0] == singleton);
    assert(retained(q.rules[0], empty));
    assert(q.rules[0].naf[0][0].subset_of(singleton));
    assert(!retained(q.rules[0], singleton));
    assert(closed(q, singleton, empty));
    support::least(q, singleton, empty);
    assert(lfp(q, singleton) =~= empty);
    support::lfp_fixed(q, empty);
    assert(lfp(q, empty) =~= singleton);
    assert(afp::double_gamma(q, empty) == empty);
    afp::lower_least(q, empty);
    assert(afp::lower(q) =~= empty);
    assert(afp::upper(q) == singleton);
    assert(afp::undefined_atoms(q) =~= singleton);
}

// Selected clauses can derive an extra head outside the all-producer atom
// slice. Both query shapes are preserved; global mask/model equality is false.
pub proof fn clause_selection_control() {
    // AFP contracts suffice; keep their second-order definitions opaque here.
    hide(afp::lower);
    hide(afp::upper);
    let seed = Instance { head: 1, positive: Set::empty(), naf: Seq::empty(), builtins: true };
    let derive = Instance { head: 0, positive: set![1int], naf: Seq::empty(), builtins: true };
    let extra = Instance { head: 2, positive: Set::empty(), naf: Seq::empty(), builtins: true };
    let p = Program {
        atoms: set![0int, 1int, 2int, 3int],
        rules: seq![seed, derive, extra, extra],
    };
    let origin = seq![0int, 1int, 1int, 2int];
    let chosen = set![0int, 1int];
    let g = rank::Graph {
        nodes: set![0int, 1int, 2int],
        positive: set![(1int, 0int)],
        negative: Set::empty(),
    };
    assert(wf(p));
    assert(rank::graph_wf(g));
    assert(rank::no_negative_cycle(g));
    assert(origins::clause_coverage(p, g, origin));
    assert(rank::forward_closed(g, chosen));
    let s = origins::atom_slice(p, origin, chosen);
    assert(s.contains(0int));
    assert(s.contains(1int));
    assert(s.contains(3int));
    assert(p.rules[3].head == 2);
    assert(!chosen.contains(origin[3]));
    assert(!s.contains(2int));
    let q = selection::select(p, origin, chosen);
    let empty = Set::<Atom>::empty();
    selection::clause_selection_reduct(p, g, origin, chosen, empty);
    selection::selection_exact(p, origin, chosen);
    assert(chosen.contains(origin[2]));
    assert(q.rules.contains(extra));
    let j = choose|j: int| 0 <= j < q.rules.len() && q.rules[j] == extra;
    support::lfp_fixed(q, empty);
    assert(retained(q.rules[j], empty));
    assert(q.rules[j].positive.subset_of(lfp(q, empty)));
    assert(immediate(q, empty, lfp(q, empty)).contains(2int));
    assert(lfp(q, empty).contains(2int));
    let masked = lfp(slice::mask(p, s), empty.intersect(s));
    assert(!masked.contains(2int));
    assert(lfp(q, empty) != masked);

    let operators = set![0int];
    let valuations = set![0int];
    let goals = |_: int|
        Instance {
            head: 99,
            positive: set![0int, 1int],
            naf: seq![seq![set![3int]]],
            builtins: true,
        };
    assert(operators.subset_of(s));
    assert(slice::reads(goals(0), s));
    selection::selected_reduct_queries(p, g, origin, chosen, empty, operators, valuations, goals);
    selection::selected_wfm_queries(p, g, origin, chosen, operators, valuations, goals);

    let m = set![0int, 1int, 2int];
    let level = |a: Atom|
        if a == 0 {
            1nat
        } else {
            0nat
        };
    assert(p.rules[0] == seed);
    assert(p.rules[1] == derive);
    assert(p.rules[2] == extra);
    assert(founded(p, m, level));
    assert(closed(p, m, m));
    support::support_closure_lfp(p, m, level);
    afp::covered_wfm(p, g, origin);
    assert(afp::lower(p) == m);
    assert(slice::enumerate(afp::lower(p), operators) =~= operators);
    assert(goals(0).naf[0][0].contains(3int));
    assert(!m.contains(3int));
    assert(!goals(0).naf[0][0].subset_of(m));
    assert(slice::goal_holds(goals(0), m));
    assert(slice::goal_answers(afp::lower(p), valuations, goals) =~= valuations);
}

} // verus!
verus! {

// TA-T0 pair (`a(beta)` has no producer): `x :- a(beta), \+ x.` + `a(alpha).`
// The rule can never fire, so the pruned program needs no edge and its source
// WFM is total; the unpruned premise is false (a negative self-loop) for it.
pub proof fn inactive_negative_control() {
    let alpha = Instance { head: 0, positive: Set::empty(), naf: Seq::empty(), builtins: true };
    let dead = Instance {
        head: 2,
        positive: set![1int],
        naf: seq![seq![set![2int]]],
        builtins: true,
    };
    let p = Program { atoms: set![0int, 1int, 2int], rules: seq![alpha, dead] };
    let l = set![0int];
    assert(wf(p));
    assert(set![1int].contains(1int));
    assert(!set![1int].subset_of(l));
    assert(envelope::envelope(p, l));
    let q = envelope::prune(p, l);
    assert(q.rules[0].naf =~= Seq::<Seq<Set<Atom>>>::empty());
    assert(q.rules[1].naf =~= Seq::<Seq<Set<Atom>>>::empty());
    assert(q.rules[1].positive == Set::<Atom>::empty());
    let g = rank::Graph { nodes: set![0int, 1int], positive: Set::empty(), negative: Set::empty() };
    let origin = seq![0int, 1int];
    assert(rank::graph_wf(g));
    assert(rank::no_negative_cycle(g));
    assert(origins::clause_coverage(q, g, origin));
    assert forall|h: rank::Graph, o: Seq<int>|
        rank::graph_wf(h) && origins::clause_coverage(p, h, o) implies !rank::no_negative_cycle(
        h,
    ) by {
        assert(p.rules[1].naf[0][0].contains(p.rules[1].head));
        assert(h.negative.contains((o[1], o[1])));
        assert(rank::reach(h, o[1]).contains(o[1]));
    }
    let m = set![0int];
    let level = |_: Atom| 0nat;
    assert(retained(p.rules[1], m)) by {
        assert(set![2int].contains(2int));
    }
    assert(founded(p, m, level));
    assert(closed(p, m, m));
    envelope::pruned_certified_is_wfm(p, l, g, origin, m, level);
    assert(afp::lower(p) == m);
    assert(afp::upper(p) == m);
}

// Its live twin adds `a(beta).`: every envelope holds `a(beta)` + `x`, every
// graph covering any pruned program has a negative cycle, and the source WFM
// leaves `x` undefined, so refusal is necessary, not an artifact.
pub proof fn active_negative_control() {
    hide(afp::lower);
    let alpha = Instance { head: 0, positive: Set::empty(), naf: Seq::empty(), builtins: true };
    let beta = Instance { head: 1, positive: Set::empty(), naf: Seq::empty(), builtins: true };
    let live = Instance {
        head: 2,
        positive: set![1int],
        naf: seq![seq![set![2int]]],
        builtins: true,
    };
    let p = Program { atoms: set![0int, 1int, 2int], rules: seq![alpha, live, beta] };
    assert(wf(p));
    assert forall|l: Set<Atom>, h: rank::Graph, o: Seq<int>|
        envelope::envelope(p, l) && rank::graph_wf(h) && origins::clause_coverage(
            envelope::prune(p, l),
            h,
            o,
        ) implies !rank::no_negative_cycle(h) by {
        assert(p.rules[2].positive.subset_of(l));
        assert(l.contains(1int));
        assert(p.rules[1].positive.subset_of(l));
        assert(l.contains(2int));
        let q = envelope::prune(p, l);
        envelope::kept_witness(p.rules[1], l, 0);
        assert(p.rules[1].naf[0][0].subset_of(l));
        assert(q.rules[1].naf[0].contains(set![2int]));
        let k = choose|k: int|
            0 <= k < q.rules[1].naf[0].len() && q.rules[1].naf[0][k] == set![2int];
        assert(q.rules[1].naf[0][k].contains(q.rules[1].head));
        assert(h.negative.contains((o[1], o[1])));
        assert(rank::reach(h, o[1]).contains(o[1]));
    }
    let all = set![0int, 1int, 2int];
    let facts = set![0int, 1int];
    assert(set![2int].contains(2int));
    assert(!retained(p.rules[1], all));
    assert(retained(p.rules[1], facts));
    assert(closed(p, all, facts));
    support::least(p, all, facts);
    support::lfp_closed(p, all);
    assert(lfp(p, all) =~= facts);
    support::lfp_closed(p, facts);
    assert(lfp(p, facts) =~= all);
    assert(afp::double_gamma(p, facts) == facts);
    afp::lower_least(p, facts);
    afp::lower_fixed(p);
    let lower = afp::lower(p);
    support::lfp_closed(p, lfp(p, lower));
    assert(facts.subset_of(lower));
    assert(lower =~= facts);
    assert(afp::upper(p) == all);
    assert(afp::undefined_atoms(p) =~= set![2int]);
}

} // verus!
verus! {

pub open spec fn base_origin(v: grounding::Value) -> int {
    match v {
        grounding::Value::Scalar(_) => 0,
        grounding::Value::Identity(sym, _) => sym,
    }
}

// k = 1 refinement: an identity's type = its symbol + its immediate Deps' base
// origins; a scalar's = the scalar class 0.
pub open spec fn k1_type(v: grounding::Value) -> (int, Seq<int>) {
    match v {
        grounding::Value::Scalar(_) => (0, Seq::empty()),
        grounding::Value::Identity(sym, args) => (
            sym,
            args.map_values(|a: grounding::Value| base_origin(a)),
        ),
    }
}

pub open spec fn k1_ty() -> spec_fn(grounding::Value) -> (int, Seq<int>) {
    |v: grounding::Value| k1_type(v)
}

pub open spec fn k1_ct() -> spec_fn(int, Seq<(int, Seq<int>)>) -> (int, Seq<int>) {
    |sym: int, ts: Seq<(int, Seq<int>)>| (sym, ts.map_values(|t: (int, Seq<int>)| t.0))
}

pub open spec fn base_ty() -> spec_fn(grounding::Value) -> int {
    |v: grounding::Value| base_origin(v)
}

pub open spec fn base_ct() -> spec_fn(int, Seq<int>) -> int {
    |sym: int, ts: Seq<int>| sym
}

pub open spec fn var_goal(pred: int) -> flow::Goal {
    flow::Goal { pred, args: seq![flow::Term::Variable(0)] }
}

pub open spec fn build_goal(pred: int, sym: int) -> flow::Goal {
    flow::Goal { pred, args: seq![flow::Term::Construct(sym, seq![0nat])] }
}

pub open spec fn scalar_goal(pred: int, n: int) -> flow::Goal {
    flow::Goal { pred, args: seq![flow::Term::Constant(grounding::Value::Scalar(n))] }
}

// Predicates s=1 z=2 z2=3 w=4 q=5 w2=6; origins O=10 P=20. Rule 6 forwards z
// into z2 unchanged. Base origins cycle (O <- P via rule 2, P <- O via rule 5),
// but rule 5 only ever reads P values built from scalars (rule 4), so k = 1
// types never cycle.
pub open spec fn k1_rules() -> Seq<flow::Rule> {
    seq![
        flow::Rule { head: scalar_goal(1, 7), body: Seq::empty() },
        flow::Rule { head: build_goal(2, 10), body: seq![var_goal(1)] },
        flow::Rule { head: build_goal(4, 20), body: seq![var_goal(3)] },
        flow::Rule { head: scalar_goal(5, 8), body: Seq::empty() },
        flow::Rule { head: build_goal(6, 20), body: seq![var_goal(5)] },
        flow::Rule { head: build_goal(2, 10), body: seq![var_goal(6)] },
        flow::Rule { head: var_goal(3), body: seq![var_goal(2)] },
    ]
}

pub open spec fn k1_hint() -> spec_fn(int, nat) -> Set<(int, Seq<int>)> {
    |i: int, x: nat|
        if i == 1 || i == 4 {
            set![(0int, Seq::<int>::empty())]
        } else if i == 2 || i == 6 {
            set![(10int, seq![0int]), (10int, seq![20int])]
        } else if i == 5 {
            set![(20int, seq![0int])]
        } else {
            Set::empty()
        }
}

pub open spec fn first_goal() -> spec_fn(int, nat) -> (int, int) {
    |i: int, x: nat| (0int, 0int)
}

pub open spec fn same_pred() -> spec_fn(int, int, int) -> bool {
    |i: int, j: int, q: int| k1_rules()[q].head.pred == k1_rules()[i].body[j].pred
}

pub open spec fn is_construct(t: flow::Term) -> bool {
    match t {
        flow::Term::Construct(_, _) => true,
        _ => false,
    }
}

pub open spec fn construct_deps(t: flow::Term) -> Seq<nat> {
    match t {
        flow::Term::Construct(_, deps) => deps,
        _ => Seq::empty(),
    }
}

pub open spec fn k1_rank(t: (int, Seq<int>)) -> nat {
    if t == (20int, seq![10int]) {
        3
    } else if t == (10int, seq![20int]) {
        2
    } else if t == (10int, seq![0int]) || t == (20int, seq![0int]) {
        1
    } else {
        0
    }
}

pub proof fn k1_shape(i: int, x: nat)
    requires
        0 <= i < 7,
    ensures
        k1_rules()[i].head.args.len() == 1,
        k1_rules()[i].body.len() == (if i == 0 || i == 3 {
            0int
        } else {
            1int
        }),
        i != 0 && i != 3 ==> k1_rules()[i].body[0] == var_goal(
            if i == 1 {
                1int
            } else if i == 2 {
                3int
            } else if i == 4 {
                5int
            } else if i == 5 {
                6int
            } else {
                2int
            },
        ),
        flow::head_var(k1_rules()[i], x) ==> i != 0 && i != 3 && x == 0,
        i != 0 && i != 3 ==> flow::mentions(k1_rules()[i].head.args[0], 0),
{
    let r = k1_rules()[i];
    if flow::head_var(r, x) {
        let k = choose|k: int|
            0 <= k < r.head.args.len() && #[trigger] flow::mentions(r.head.args[k], x);
        if i == 1 || i == 2 || i == 4 || i == 5 {
            assert(seq![0nat].contains(x));
            let n = choose|n: int| 0 <= n < seq![0nat].len() && seq![0nat][n] == x;
        }
    }
    if i != 0 && i != 3 && i != 6 {
        assert(seq![0nat][0] == 0);
    }
}

// The base-origin abstraction must refuse: every closed hint forces the
// growth edges P -> O and O -> P, so no rank descends them.
pub proof fn k1_base_refuses(types: spec_fn(int, nat) -> Set<int>, g: spec_fn(int) -> nat)
    requires
        flow::closed_hint(k1_rules(), types, first_goal(), same_pred(), base_ty(), base_ct()),
        forall|u: int, v: int|
            flow::growth_edge(k1_rules(), types, base_ct(), u, v) ==> #[trigger] g(v)
                < #[trigger] g(u),
    ensures
        false,
{
    let rules = k1_rules();
    let may = same_pred();
    let binder = first_goal();
    k1_shape(1, 0);
    k1_shape(2, 0);
    k1_shape(4, 0);
    k1_shape(5, 0);
    k1_shape(6, 0);
    assert(flow::head_var(rules[1], 0));
    assert(may(1, binder(1, 0).0, 0));
    assert(flow::term_has(types, base_ty(), base_ct(), 0, rules[0].head.args[binder(1, 0).1], 0));
    assert(types(1, 0).contains(0));
    assert(flow::head_var(rules[6], 0));
    assert(may(6, binder(6, 0).0, 1));
    let one = seq![0int];
    let dep = seq![0nat];
    assert(one[0] == 0 && dep[0] == 0);
    assert(forall|d: int| 0 <= d < dep.len() ==> types(1, dep[d]).contains(one[d]));
    assert(rules[1].head.args[0] == flow::Term::Construct(10, dep));
    assert(rules[4].head.args[0] == flow::Term::Construct(20, dep));
    assert(base_ct()(10, one) == 10);
    flow::construct_has(types, base_ty(), base_ct(), 1, 10, dep, one);
    assert(rules[1].head.args[binder(6, 0).1] == flow::Term::Construct(10, dep));
    assert(flow::term_has(types, base_ty(), base_ct(), 1, rules[1].head.args[binder(6, 0).1], 10));
    assert(types(6, 0).contains(10));
    assert(flow::head_var(rules[2], 0));
    assert(may(2, binder(2, 0).0, 6));
    assert(flow::term_has(types, base_ty(), base_ct(), 6, rules[6].head.args[binder(2, 0).1], 10));
    assert(types(2, 0).contains(10));
    let down = seq![10int];
    assert(rules[2].head.args[0] == flow::Term::Construct(20, seq![0nat]));
    assert(flow::growth_edge(rules, types, base_ct(), 20, down[0]));
    assert(flow::head_var(rules[4], 0));
    assert(may(4, binder(4, 0).0, 3));
    assert(flow::term_has(types, base_ty(), base_ct(), 3, rules[3].head.args[binder(4, 0).1], 0));
    assert(types(4, 0).contains(0));
    assert(forall|d: int| 0 <= d < dep.len() ==> types(4, dep[d]).contains(one[d]));
    assert(flow::head_var(rules[5], 0));
    assert(may(5, binder(5, 0).0, 4));
    assert(base_ct()(20, one) == 20);
    flow::construct_has(types, base_ty(), base_ct(), 4, 20, dep, one);
    assert(rules[4].head.args[binder(5, 0).1] == flow::Term::Construct(20, dep));
    assert(flow::term_has(types, base_ty(), base_ct(), 4, rules[4].head.args[binder(5, 0).1], 20));
    assert(types(5, 0).contains(20));
    let up = seq![20int];
    assert(rules[5].head.args[0] == flow::Term::Construct(10, seq![0nat]));
    assert(flow::growth_edge(rules, types, base_ct(), 10, up[0]));
    assert(g(10) < g(20));
    assert(g(20) < g(10));
}

// Every premise of growth_covered holds for the k = 1 hint, its refined growth
// edges descend a rank, and so every concrete Deps dependency of every
// derivable instance descends it (the finite-grounding premise at k = 1).
pub proof fn k1_forwarding_control() {
    let rules = k1_rules();
    let types = k1_hint();
    let binder = first_goal();
    let may = same_pred();
    let ty = k1_ty();
    let ct = k1_ct();
    assert forall|sym: int, args: Seq<grounding::Value>| #[trigger]
        ty(grounding::Value::Identity(sym, args)) == ct(sym, args.map_values(ty)) by {
        assert(args.map_values(|a: grounding::Value| base_origin(a)) =~= args.map_values(
            ty,
        ).map_values(|t: (int, Seq<int>)| t.0));
    }
    assert(flow::compositional(ty, ct));
    assert forall|i: int, x: nat|
        0 <= i < rules.len() && #[trigger] flow::head_var(rules[i], x) implies {
        let j = binder(i, x).0;
        let k = binder(i, x).1;
        &&& 0 <= j < rules[i].body.len()
        &&& 0 <= k < rules[i].body[j].args.len()
        &&& rules[i].body[j].args[k] == flow::Term::Variable(x)
    } by {
        k1_shape(i, x);
    }
    assert(flow::bound(rules, binder));
    assert(flow::may_sound(rules, may));
    assert forall|i: int, x: nat, q: int, c: (int, Seq<int>)|
        #![trigger may(i, binder(i, x).0, q), flow::term_has(types, ty, ct, q, rules[q].head.args[binder(i, x).1], c)]
        0 <= i < rules.len() && flow::head_var(rules[i], x) && 0 <= q < rules.len() && may(
            i,
            binder(i, x).0,
            q,
        ) && binder(i, x).1 < rules[q].head.args.len() && flow::term_has(
            types,
            ty,
            ct,
            q,
            rules[q].head.args[binder(i, x).1],
            c,
        ) implies types(i, x).contains(c) by {
        k1_shape(i, x);
        k1_shape(q, x);
        if q == 1 || q == 2 || q == 4 || q == 5 {
            let sym = if q == 1 || q == 5 {
                10int
            } else {
                20int
            };
            assert(rules[q].head.args[0] == flow::Term::Construct(sym, seq![0nat]));
            let ts = choose|ts: Seq<(int, Seq<int>)>| #[trigger]
                ct(sym, ts) == c && ts.len() == seq![0nat].len() && forall|d: int|
                    0 <= d < seq![0nat].len() ==> types(q, seq![0nat][d]).contains(ts[d]);
            assert(types(q, 0).contains(ts[0]));
            assert(ts.map_values(|t: (int, Seq<int>)| t.0) =~= seq![ts[0].0]);
        }
    }
    assert(flow::closed_hint(rules, types, binder, may, ty, ct));
    assert forall|u: (int, Seq<int>), v: (int, Seq<int>)|
        flow::growth_edge(rules, types, ct, u, v) implies #[trigger] k1_rank(v)
        < #[trigger] k1_rank(u) by {
        let (i, k, ts, d) = choose|i: int, k: int, ts: Seq<(int, Seq<int>)>, d: int|
            #![trigger rules[i].head.args[k], ts[d]]
            0 <= i < rules.len() && 0 <= k < rules[i].head.args.len()
                && match rules[i].head.args[k] {
                flow::Term::Construct(sym, deps) => {
                    &&& ts.len() == deps.len()
                    &&& forall|e: int| 0 <= e < deps.len() ==> types(i, deps[e]).contains(ts[e])
                    &&& u == ct(sym, ts)
                    &&& 0 <= d < deps.len()
                    &&& v == ts[d]
                },
                _ => false,
            };
        k1_shape(i, 0);
        assert(types(i, 0).contains(ts[0]));
        assert(ts.map_values(|t: (int, Seq<int>)| t.0) =~= seq![ts[0].0]);
        assert(seq![10int][0] != seq![0int][0]);
        assert(seq![20int][0] != seq![0int][0]);
        assert(seq![10int][0] != seq![20int][0]);
        assert(seq![0int].len() != Seq::<int>::empty().len());
    }
    assert forall|i: int, val: Seq<grounding::Value>, depth: nat, k: int, e: int|
        #![trigger flow::body_derived(rules, i, val, depth), construct_deps(rules[i].head.args[k])[e]]
        0 <= i < rules.len() && flow::body_derived(rules, i, val, depth) && 0 <= k
            < rules[i].head.args.len() && is_construct(rules[i].head.args[k]) && 0 <= e
            < construct_deps(rules[i].head.args[k]).len() implies k1_rank(
        ty(val[construct_deps(rules[i].head.args[k])[e] as int]),
    ) < k1_rank(ty(flow::value(rules[i].head.args[k], val))) by {
        let t = rules[i].head.args[k];
        let (sym, deps) = match t {
            flow::Term::Construct(sym, deps) => (sym, deps),
            _ => (0, Seq::empty()),
        };
        assert(t == flow::Term::Construct(sym, deps));
        flow::growth_covered(rules, types, binder, may, ty, ct, i, val, depth, k, sym, deps, e);
    }
}

} // verus!
