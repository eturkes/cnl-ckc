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
