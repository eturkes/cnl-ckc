use crate::*;

verus! {

pub ghost enum Value {
    Scalar(int),
    Identity(int, Seq<Value>),
}

// A constructor symbol encodes (Kind, Doc, Sentence, Slot); arguments are its
// Deps variables. The same finite-image proof permits deterministic arithmetic
// outputs via eval, without assuming a finite numeric range.
pub struct Operation {
    pub symbol: int,
    pub output: int,
    pub inputs: Seq<int>,
}

pub struct Signature {
    pub base: Set<Value>,
    pub operations: Seq<Operation>,
    pub flow: rank::Graph,
}

pub open spec fn indices(n: nat) -> Set<int> {
    Set::range(0, n as int)
}

pub open spec fn vector_ok<A>(s: Seq<A>, values: Set<A>, n: nat) -> bool {
    s.len() == n && forall|j: int| 0 <= j < s.len() ==> values.contains(s[j])
}

pub open spec fn vectors<A>(values: Set<A>, n: nat) -> Set<Seq<A>>
    decreases n,
{
    if n == 0 {
        Set::empty().insert(Seq::empty())
    } else {
        values.map(
            |last: A| vectors(values, (n - 1) as nat).map(|prefix: Seq<A>| prefix.push(last)),
        ).flatten()
    }
}

pub proof fn vector_membership<A>(values: Set<A>, n: nat, s: Seq<A>)
    ensures
        vectors(values, n).contains(s) == vector_ok(s, values, n),
    decreases n,
{
    broadcast use {Set::lemma_map_contains, Set::lemma_flatten_contains};

    if n == 0 {
        if s.len() == 0 {
            assert(s =~= Seq::<A>::empty());
        }
    } else {
        let smaller = vectors(values, (n - 1) as nat);
        let family = values.map(|last: A| smaller.map(|prefix: Seq<A>| prefix.push(last)));
        if vector_ok(s, values, n) {
            let prefix = s.drop_last();
            let last = s[s.len() - 1];
            vector_membership(values, (n - 1) as nat, prefix);
            assert(prefix.push(last) =~= s);
            let image = smaller.map(|prefix: Seq<A>| prefix.push(last));
            assert(image.contains(s));
            assert(family.contains(image));
        }
        if vectors(values, n).contains(s) {
            let image = choose|image: Set<Seq<A>>| family.contains(image) && image.contains(s);
            let last = choose|last: A|
                values.contains(last) && image == smaller.map(|prefix: Seq<A>| prefix.push(last));
            let prefix = choose|prefix: Seq<A>| smaller.contains(prefix) && s == prefix.push(last);
            vector_membership(values, (n - 1) as nat, prefix);
            assert(vector_ok(s, values, n));
        }
    }
}

pub open spec fn outputs(
    values: Set<Value>,
    operations: Seq<Operation>,
    eval: spec_fn(int, Seq<Value>) -> Value,
) -> Set<Value> {
    indices(operations.len()).map(
        |i: int| vectors(values, operations[i].inputs.len()).map(|args: Seq<Value>| eval(i, args)),
    ).flatten()
}

pub open spec fn universe(
    signature: Signature,
    eval: spec_fn(int, Seq<Value>) -> Value,
    depth: nat,
) -> Set<Value>
    decreases depth,
{
    if depth == 0 {
        signature.base
    } else {
        let previous = universe(signature, eval, (depth - 1) as nat);
        previous.union(outputs(previous, signature.operations, eval))
    }
}

pub proof fn universe_monotone(
    signature: Signature,
    eval: spec_fn(int, Seq<Value>) -> Value,
    a: nat,
    b: nat,
)
    requires
        a <= b,
    ensures
        universe(signature, eval, a).subset_of(universe(signature, eval, b)),
    decreases b,
{
    if a < b {
        universe_monotone(signature, eval, a, (b - 1) as nat);
    }
}

pub open spec fn signature_wf(s: Signature) -> bool {
    &&& rank::graph_wf(s.flow)
    &&& rank::no_negative_cycle(s.flow)
    &&& forall|i: int|
        #![trigger s.operations[i]]
        0 <= i < s.operations.len() ==> {
            let op = s.operations[i];
            &&& s.flow.nodes.contains(op.output)
            &&& forall|j: int|
                0 <= j < op.inputs.len() ==> s.flow.negative.contains((op.output, op.inputs[j]))
        }
}

pub open spec fn type_values(s: Signature, eval: spec_fn(int, Seq<Value>) -> Value, ty: int) -> Set<
    Value,
> {
    universe(s, eval, rank::rank(s.flow, ty))
}

pub proof fn output_member(
    values: Set<Value>,
    operations: Seq<Operation>,
    eval: spec_fn(int, Seq<Value>) -> Value,
    i: int,
    args: Seq<Value>,
)
    requires
        0 <= i < operations.len(),
        vector_ok(args, values, operations[i].inputs.len()),
    ensures
        outputs(values, operations, eval).contains(eval(i, args)),
{
    broadcast use {Set::lemma_map_contains, Set::lemma_flatten_contains};

    vector_membership(values, operations[i].inputs.len(), args);
    let image = vectors(values, operations[i].inputs.len()).map(|a: Seq<Value>| eval(i, a));
    assert(image.contains(eval(i, args)));
    assert(indices(operations.len()).contains(i));
    assert(indices(operations.len()).map(
        |id: int| vectors(values, operations[id].inputs.len()).map(|a: Seq<Value>| eval(id, a)),
    ).contains(image));
}

// (d), substitution closure: every operation's input-to-output flow is a
// strict-growth edge; absence of a growth cycle supplies a finite rank. Each
// input may already be an arbitrarily nested identity in its type's universe.
// The result stays in the output type's program-derived universe.
pub proof fn substitution_closed(
    s: Signature,
    eval: spec_fn(int, Seq<Value>) -> Value,
    i: int,
    args: Seq<Value>,
)
    requires
        signature_wf(s),
        0 <= i < s.operations.len(),
        args.len() == s.operations[i].inputs.len(),
        forall|j: int|
            0 <= j < args.len() ==> type_values(s, eval, s.operations[i].inputs[j]).contains(
                args[j],
            ),
    ensures
        type_values(s, eval, s.operations[i].output).contains(eval(i, args)),
{
    rank::signed_graph_rank(s.flow);
    let op = s.operations[i];
    let h = rank::rank(s.flow, op.output);
    assert(rank::reach(s.flow, op.output).contains(op.output));
    assert(h > 0);
    let previous = universe(s, eval, (h - 1) as nat);
    assert forall|j: int| 0 <= j < args.len() implies previous.contains(args[j]) by {
        assert(rank::rank(s.flow, op.inputs[j]) < h);
        universe_monotone(s, eval, rank::rank(s.flow, op.inputs[j]), (h - 1) as nat);
    }
    output_member(previous, s.operations, eval, i, args);
}

pub proof fn copy_closed(
    s: Signature,
    eval: spec_fn(int, Seq<Value>) -> Value,
    output: int,
    input: int,
)
    requires
        signature_wf(s),
        s.flow.positive.contains((output, input)),
    ensures
        type_values(s, eval, input).subset_of(type_values(s, eval, output)),
{
    rank::signed_graph_rank(s.flow);
    universe_monotone(s, eval, rank::rank(s.flow, input), rank::rank(s.flow, output));
}

pub ghost enum Template {
    Constant(Value),
    Variable(nat),
    Apply(int, Seq<nat>),
}

pub struct Clause {
    pub variables: Seq<int>,
    pub terms: Seq<Template>,
}

pub open spec fn template_wf(s: Signature, c: Clause, t: Template) -> bool {
    match t {
        Template::Constant(v) => s.base.contains(v),
        Template::Variable(v) => v < c.variables.len(),
        Template::Apply(i, deps) => 0 <= i < s.operations.len() && deps.len()
            == s.operations[i].inputs.len() && forall|j: int|
            0 <= j < deps.len() ==> deps[j] < c.variables.len() && c.variables[deps[j] as int]
                == s.operations[i].inputs[j],
    }
}

pub open spec fn clause_wf(s: Signature, c: Clause) -> bool {
    &&& forall|j: int| 0 <= j < c.variables.len() ==> s.flow.nodes.contains(c.variables[j])
    &&& forall|j: int| 0 <= j < c.terms.len() ==> template_wf(s, c, c.terms[j])
}

pub open spec fn valuation_ok(
    s: Signature,
    c: Clause,
    eval: spec_fn(int, Seq<Value>) -> Value,
    valuation: Seq<Value>,
) -> bool {
    valuation.len() == c.variables.len() && forall|j: int|
        0 <= j < valuation.len() ==> type_values(s, eval, c.variables[j]).contains(valuation[j])
}

pub open spec fn substitute(
    t: Template,
    valuation: Seq<Value>,
    eval: spec_fn(int, Seq<Value>) -> Value,
) -> Value {
    match t {
        Template::Constant(v) => v,
        Template::Variable(v) => valuation[v as int],
        Template::Apply(i, deps) => eval(i, deps.map_values(|v: nat| valuation[v as int])),
    }
}

pub open spec fn instantiate(
    c: Clause,
    valuation: Seq<Value>,
    eval: spec_fn(int, Seq<Value>) -> Value,
) -> Seq<Value> {
    c.terms.map_values(|t: Template| substitute(t, valuation, eval))
}

pub open spec fn global_values(s: Signature, eval: spec_fn(int, Seq<Value>) -> Value) -> Set<
    Value,
> {
    universe(s, eval, s.flow.nodes.len())
}

pub proof fn type_bounded(s: Signature, eval: spec_fn(int, Seq<Value>) -> Value, ty: int)
    ensures
        type_values(s, eval, ty).subset_of(global_values(s, eval)),
{
    vstd::set_lib::lemma_len_subset(rank::reach(s.flow, ty), s.flow.nodes);
    universe_monotone(s, eval, rank::rank(s.flow, ty), s.flow.nodes.len());
}

pub proof fn term_closed(
    s: Signature,
    c: Clause,
    eval: spec_fn(int, Seq<Value>) -> Value,
    valuation: Seq<Value>,
    t: Template,
)
    requires
        signature_wf(s),
        clause_wf(s, c),
        template_wf(s, c, t),
        valuation_ok(s, c, eval, valuation),
    ensures
        global_values(s, eval).contains(substitute(t, valuation, eval)),
{
    match t {
        Template::Constant(v) => {
            universe_monotone(s, eval, 0, s.flow.nodes.len());
        },
        Template::Variable(v) => {
            type_bounded(s, eval, c.variables[v as int]);
        },
        Template::Apply(i, deps) => {
            let args = deps.map_values(|v: nat| valuation[v as int]);
            assert forall|j: int| 0 <= j < args.len() implies type_values(
                s,
                eval,
                s.operations[i].inputs[j],
            ).contains(args[j]) by {
                assert(args[j] == valuation[deps[j] as int]);
            }
            substitution_closed(s, eval, i, args);
            type_bounded(s, eval, s.operations[i].output);
        },
    }
}

pub open spec fn ground_instances(
    s: Signature,
    clauses: Seq<Clause>,
    eval: spec_fn(int, Seq<Value>) -> Value,
) -> Set<(int, Seq<Value>)> {
    indices(clauses.len()).map(
        |i: int|
            vectors(global_values(s, eval), clauses[i].variables.len()).filter(
                |v: Seq<Value>| valuation_ok(s, clauses[i], eval, v),
            ).map(|v: Seq<Value>| (i, instantiate(clauses[i], v, eval))),
    ).flatten()
}

pub open spec fn all_legal_instances(
    s: Signature,
    clauses: Seq<Clause>,
    eval: spec_fn(int, Seq<Value>) -> Value,
) -> ISet<(int, Seq<Value>)> {
    ISet::new(
        |item: (int, Seq<Value>)|
            exists|v: Seq<Value>|
                0 <= item.0 < clauses.len() && valuation_ok(s, clauses[item.0], eval, v) && item.1
                    == instantiate(clauses[item.0], v, eval),
    )
}

// (d), finiteness + completeness: the entire semantic comprehension of ground
// clause instances is equal to a constructed finite set. The construction uses
// only the finite program's base values, operation templates + flow graph; it
// never takes a proposed model or a producer-supplied universe as input.
pub proof fn finite_grounding(
    s: Signature,
    clauses: Seq<Clause>,
    eval: spec_fn(int, Seq<Value>) -> Value,
)
    requires
        signature_wf(s),
        forall|i: int| 0 <= i < clauses.len() ==> clause_wf(s, clauses[i]),
    ensures
        all_legal_instances(s, clauses, eval).finite(),
        all_legal_instances(s, clauses, eval) == ground_instances(s, clauses, eval).to_iset(),
        forall|i: int, v: Seq<Value>, j: int|
            0 <= i < clauses.len() && valuation_ok(s, clauses[i], eval, v) && 0 <= j
                < clauses[i].terms.len() ==> global_values(s, eval).contains(
                instantiate(clauses[i], v, eval)[j],
            ),
{
    broadcast use {Set::lemma_map_contains, Set::lemma_flatten_contains};

    let u = global_values(s, eval);
    assert forall|item: (int, Seq<Value>)|
        all_legal_instances(s, clauses, eval).contains(item) implies ground_instances(
        s,
        clauses,
        eval,
    ).contains(item) by {
        let i = item.0;
        let v = choose|v: Seq<Value>|
            0 <= i < clauses.len() && valuation_ok(s, clauses[i], eval, v) && item.1 == instantiate(
                clauses[i],
                v,
                eval,
            );
        assert forall|j: int| 0 <= j < v.len() implies u.contains(v[j]) by {
            type_bounded(s, eval, clauses[i].variables[j]);
        }
        vector_membership(u, clauses[i].variables.len(), v);
        let vals = vectors(u, clauses[i].variables.len()).filter(
            |v: Seq<Value>| valuation_ok(s, clauses[i], eval, v),
        );
        let image = vals.map(|v: Seq<Value>| (i, instantiate(clauses[i], v, eval)));
        assert(vals.contains(v));
        assert(item == (i, instantiate(clauses[i], v, eval)));
        assert(image.contains(item));
        assert(indices(clauses.len()).contains(i));
        assert(indices(clauses.len()).map(
            |id: int|
                vectors(u, clauses[id].variables.len()).filter(
                    |v: Seq<Value>| valuation_ok(s, clauses[id], eval, v),
                ).map(|v: Seq<Value>| (id, instantiate(clauses[id], v, eval))),
        ).contains(image));
    }
    assert forall|item: (int, Seq<Value>)|
        ground_instances(s, clauses, eval).contains(item) implies all_legal_instances(
        s,
        clauses,
        eval,
    ).contains(item) by {
        let family = indices(clauses.len()).map(
            |i: int|
                vectors(u, clauses[i].variables.len()).filter(
                    |v: Seq<Value>| valuation_ok(s, clauses[i], eval, v),
                ).map(|v: Seq<Value>| (i, instantiate(clauses[i], v, eval))),
        );
        let image = choose|image: Set<(int, Seq<Value>)>|
            family.contains(image) && image.contains(item);
        let i = choose|i: int|
            indices(clauses.len()).contains(i) && image == vectors(
                u,
                clauses[i].variables.len(),
            ).filter(|v: Seq<Value>| valuation_ok(s, clauses[i], eval, v)).map(
                |v: Seq<Value>| (i, instantiate(clauses[i], v, eval)),
            );
        let v = choose|v: Seq<Value>|
            vectors(u, clauses[i].variables.len()).filter(
                |v: Seq<Value>| valuation_ok(s, clauses[i], eval, v),
            ).contains(v) && item == (i, instantiate(clauses[i], v, eval));
        assert(valuation_ok(s, clauses[item.0], eval, v));
    }
    assert(all_legal_instances(s, clauses, eval) =~= ground_instances(s, clauses, eval).to_iset());
    assert forall|i: int, v: Seq<Value>, j: int|
        0 <= i < clauses.len() && valuation_ok(s, clauses[i], eval, v) && 0 <= j
            < clauses[i].terms.len() implies u.contains(instantiate(clauses[i], v, eval)[j]) by {
        term_closed(s, clauses[i], eval, v, clauses[i].terms[j]);
    }
}

pub open spec fn identity_eval(operations: Seq<Operation>, i: int, args: Seq<Value>) -> Value {
    Value::Identity(operations[i].symbol, args)
}

// Nested '$guideline_id' case: Deps is exactly a variable list, with repetition
// + ordering preserved. Its substituted values need not be scalars.
pub proof fn nested_identity_substitution(
    s: Signature,
    c: Clause,
    valuation: Seq<Value>,
    i: int,
    deps: Seq<nat>,
)
    requires
        signature_wf(s),
        clause_wf(s, c),
        template_wf(s, c, Template::Apply(i, deps)),
        valuation_ok(
            s,
            c,
            |id: int, args: Seq<Value>| identity_eval(s.operations, id, args),
            valuation,
        ),
    ensures
        global_values(
            s,
            |id: int, args: Seq<Value>| identity_eval(s.operations, id, args),
        ).contains(
            Value::Identity(s.operations[i].symbol, deps.map_values(|v: nat| valuation[v as int])),
        ),
{
    let eval = |id: int, args: Seq<Value>| identity_eval(s.operations, id, args);
    term_closed(s, c, eval, valuation, Template::Apply(i, deps));
}

} // verus!
