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

} // verus!
