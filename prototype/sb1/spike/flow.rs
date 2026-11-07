use crate::grounding::Value;
use crate::*;

verus! {

// Positive (envelope) layer with argument structure: flat body goals, heads
// that may build a Skolem identity from Deps variables. No finite universe is
// assumed; derivation depth carries every induction.
pub ghost enum Term {
    Constant(Value),
    Variable(nat),
    Construct(int, Seq<nat>),
}

pub struct Goal {
    pub pred: int,
    pub args: Seq<Term>,
}

pub struct Rule {
    pub head: Goal,
    pub body: Seq<Goal>,
}

pub struct Fact {
    pub pred: int,
    pub args: Seq<Value>,
}

pub open spec fn value(t: Term, val: Seq<Value>) -> Value {
    match t {
        Term::Constant(v) => v,
        Term::Variable(x) => val[x as int],
        Term::Construct(sym, deps) => Value::Identity(sym, deps.map_values(|x: nat| val[x as int])),
    }
}

pub open spec fn ground(g: Goal, val: Seq<Value>) -> Fact {
    Fact { pred: g.pred, args: g.args.map_values(|t: Term| value(t, val)) }
}

pub open spec fn derived(rules: Seq<Rule>, a: Fact, depth: nat) -> bool
    decreases depth,
{
    if depth == 0 {
        false
    } else {
        exists|i: int, val: Seq<Value>|
            0 <= i < rules.len() && #[trigger] ground(rules[i].head, val) == a && forall|j: int|
                #![trigger rules[i].body[j]]
                0 <= j < rules[i].body.len() ==> derived(
                    rules,
                    ground(rules[i].body[j], val),
                    (depth - 1) as nat,
                )
    }
}

pub open spec fn body_derived(rules: Seq<Rule>, i: int, val: Seq<Value>, depth: nat) -> bool {
    forall|j: int|
        #![trigger rules[i].body[j]]
        0 <= j < rules[i].body.len() ==> derived(rules, ground(rules[i].body[j], val), depth)
}

pub open spec fn mentions(t: Term, x: nat) -> bool {
    match t {
        Term::Constant(_) => false,
        Term::Variable(y) => y == x,
        Term::Construct(_, deps) => deps.contains(x),
    }
}

pub open spec fn head_var(r: Rule, x: nat) -> bool {
    exists|k: int| 0 <= k < r.head.args.len() && #[trigger] mentions(r.head.args[k], x)
}

// An identity's type = a function of its symbol + its immediate arguments'
// types. Base origins, k = 1 refinement (the immediate Deps' base origins) and
// a per-symbol mix of both are instances.
pub open spec fn compositional<T>(
    ty: spec_fn(Value) -> T,
    ctor: spec_fn(int, Seq<T>) -> T,
) -> bool {
    forall|sym: int, args: Seq<Value>| #[trigger]
        ty(Value::Identity(sym, args)) == ctor(sym, args.map_values(ty))
}

// Untrusted source hint: per rule variable, its possible value types, plus one
// binding body occurrence; types of a head term follow from them.
pub open spec fn term_has<T>(
    types: spec_fn(int, nat) -> Set<T>,
    ty: spec_fn(Value) -> T,
    ctor: spec_fn(int, Seq<T>) -> T,
    i: int,
    t: Term,
    c: T,
) -> bool {
    match t {
        Term::Constant(v) => c == ty(v),
        Term::Variable(x) => types(i, x).contains(c),
        Term::Construct(sym, deps) => exists|ts: Seq<T>| #[trigger]
            ctor(sym, ts) == c && ts.len() == deps.len() && forall|d: int|
                0 <= d < deps.len() ==> types(i, deps[d]).contains(ts[d]),
    }
}

// Witness introduction inside the generic context (keeps the trigger's term
// shape for every instantiation of T).
pub proof fn construct_has<T>(
    types: spec_fn(int, nat) -> Set<T>,
    ty: spec_fn(Value) -> T,
    ctor: spec_fn(int, Seq<T>) -> T,
    i: int,
    sym: int,
    deps: Seq<nat>,
    ts: Seq<T>,
)
    requires
        ts.len() == deps.len(),
        forall|d: int| 0 <= d < deps.len() ==> types(i, deps[d]).contains(ts[d]),
    ensures
        term_has(types, ty, ctor, i, Term::Construct(sym, deps), ctor(sym, ts)),
{
    let t = Term::Construct(sym, deps);
    assert(t->Construct_0 == sym && t->Construct_1 == deps);
}

// Safety, checked per rule: every head variable has a binding body occurrence.
pub open spec fn bound(rules: Seq<Rule>, binder: spec_fn(int, nat) -> (int, int)) -> bool {
    forall|i: int, x: nat|
        0 <= i < rules.len() && #[trigger] head_var(rules[i], x) ==> {
            let j = binder(i, x).0;
            let k = binder(i, x).1;
            &&& 0 <= j < rules[i].body.len()
            &&& 0 <= k < rules[i].body[j].args.len()
            &&& rules[i].body[j].args[k] == Term::Variable(x)
        }
}

// The verified matcher's obligation: a derivable producer instance whose head
// equals a derivable consumer's body-goal instance is a may-edge.
pub open spec fn may_sound(rules: Seq<Rule>, may: spec_fn(int, int, int) -> bool) -> bool {
    forall|i: int, j: int, q: int, val: Seq<Value>, val2: Seq<Value>|
        0 <= i < rules.len() && 0 <= j < rules[i].body.len() && 0 <= q < rules.len() && (exists|
            d: nat,
        |
            body_derived(rules, i, val, d)) && (exists|d: nat| body_derived(rules, q, val2, d))
            && #[trigger] ground(rules[q].head, val2) == #[trigger] ground(rules[i].body[j], val)
            ==> may(i, j, q)
}

// The checker's closure test over every may-producer of each binder (a
// post-fixpoint check: leastness never needed).
pub open spec fn closed_hint<T>(
    rules: Seq<Rule>,
    types: spec_fn(int, nat) -> Set<T>,
    binder: spec_fn(int, nat) -> (int, int),
    may: spec_fn(int, int, int) -> bool,
    ty: spec_fn(Value) -> T,
    ctor: spec_fn(int, Seq<T>) -> T,
) -> bool {
    forall|i: int, x: nat, q: int, c: T|
        #![trigger may(i, binder(i, x).0, q), term_has(types, ty, ctor, q, rules[q].head.args[binder(i, x).1], c)]
        0 <= i < rules.len() && head_var(rules[i], x) && 0 <= q < rules.len() && may(
            i,
            binder(i, x).0,
            q,
        ) && binder(i, x).1 < rules[q].head.args.len() && term_has(
            types,
            ty,
            ctor,
            q,
            rules[q].head.args[binder(i, x).1],
            c,
        ) ==> types(i, x).contains(c)
}

// The refined growth edges the checker enumerates from the hint: constructor
// output type -> each Deps input type, over every hinted type tuple.
pub open spec fn growth_edge<T>(
    rules: Seq<Rule>,
    types: spec_fn(int, nat) -> Set<T>,
    ctor: spec_fn(int, Seq<T>) -> T,
    u: T,
    v: T,
) -> bool {
    exists|i: int, k: int, ts: Seq<T>, d: int|
        #![trigger rules[i].head.args[k], ts[d]]
        0 <= i < rules.len() && 0 <= k < rules[i].head.args.len() && match rules[i].head.args[k] {
            Term::Construct(sym, deps) => {
                &&& ts.len() == deps.len()
                &&& forall|e: int| 0 <= e < deps.len() ==> types(i, deps[e]).contains(ts[e])
                &&& u == ctor(sym, ts)
                &&& 0 <= d < deps.len()
                &&& v == ts[d]
            },
            _ => false,
        }
}

pub proof fn term_sound<T>(
    types: spec_fn(int, nat) -> Set<T>,
    ty: spec_fn(Value) -> T,
    ctor: spec_fn(int, Seq<T>) -> T,
    q: int,
    val: Seq<Value>,
    t: Term,
)
    requires
        compositional(ty, ctor),
        forall|x: nat| #[trigger] mentions(t, x) ==> types(q, x).contains(ty(val[x as int])),
    ensures
        term_has(types, ty, ctor, q, t, ty(value(t, val))),
{
    match t {
        Term::Constant(v) => {},
        Term::Variable(x) => {
            assert(mentions(t, x));
        },
        Term::Construct(sym, deps) => {
            let args = deps.map_values(|x: nat| val[x as int]);
            let ts = args.map_values(ty);
            assert(ty(Value::Identity(sym, args)) == ctor(sym, ts));
            assert forall|d: int| 0 <= d < deps.len() implies types(q, deps[d]).contains(ts[d]) by {
                assert(deps.contains(deps[d]));
                assert(mentions(t, deps[d]));
            }
        },
    }
}

// Forwarding soundness: in every derivable instance, each head variable's
// value has a hinted type. Induction on derivation depth through binders,
// copies (Variable heads) + constructors; no finite universe is used.
pub proof fn hint_sound<T>(
    rules: Seq<Rule>,
    types: spec_fn(int, nat) -> Set<T>,
    binder: spec_fn(int, nat) -> (int, int),
    may: spec_fn(int, int, int) -> bool,
    ty: spec_fn(Value) -> T,
    ctor: spec_fn(int, Seq<T>) -> T,
    depth: nat,
)
    requires
        compositional(ty, ctor),
        bound(rules, binder),
        may_sound(rules, may),
        closed_hint(rules, types, binder, may, ty, ctor),
    ensures
        forall|i: int, val: Seq<Value>, x: nat|
            0 <= i < rules.len() && body_derived(rules, i, val, depth) && head_var(rules[i], x)
                ==> #[trigger] types(i, x).contains(ty(val[x as int])),
    decreases depth,
{
    assert forall|i: int, val: Seq<Value>, x: nat|
        0 <= i < rules.len() && body_derived(rules, i, val, depth) && head_var(
            rules[i],
            x,
        ) implies #[trigger] types(i, x).contains(ty(val[x as int])) by {
        let j = binder(i, x).0;
        let k = binder(i, x).1;
        let a = ground(rules[i].body[j], val);
        assert(rules[i].body[j] == rules[i].body[j]);
        assert(derived(rules, a, depth));
        let (q, val2) = choose|q: int, val2: Seq<Value>|
            0 <= q < rules.len() && #[trigger] ground(rules[q].head, val2) == a && forall|jj: int|
                #![trigger rules[q].body[jj]]
                0 <= jj < rules[q].body.len() ==> derived(
                    rules,
                    ground(rules[q].body[jj], val2),
                    (depth - 1) as nat,
                );
        let previous = (depth - 1) as nat;
        assert(body_derived(rules, q, val2, previous));
        hint_sound(rules, types, binder, may, ty, ctor, previous);
        let t = rules[q].head.args[k];
        assert(a.args.len() == rules[i].body[j].args.len());
        assert(ground(rules[q].head, val2).args.len() == rules[q].head.args.len());
        assert forall|y: nat| #[trigger] mentions(t, y) implies types(q, y).contains(
            ty(val2[y as int]),
        ) by {
            assert(head_var(rules[q], y));
        }
        term_sound(types, ty, ctor, q, val2, t);
        assert(ground(rules[q].head, val2).args[k] == value(t, val2));
        assert(a.args[k] == value(rules[i].body[j].args[k], val));
        assert(value(t, val2) == val[x as int]);
        assert(may(i, j, q));
    }
}

// The k1/forwarding coverage lemma: every concrete growth dependency of a
// derivable instance (constructor output -> one Deps input) is a refined edge
// the checker enumerates from a closed hint.
pub proof fn growth_covered<T>(
    rules: Seq<Rule>,
    types: spec_fn(int, nat) -> Set<T>,
    binder: spec_fn(int, nat) -> (int, int),
    may: spec_fn(int, int, int) -> bool,
    ty: spec_fn(Value) -> T,
    ctor: spec_fn(int, Seq<T>) -> T,
    i: int,
    val: Seq<Value>,
    depth: nat,
    k: int,
    sym: int,
    deps: Seq<nat>,
    e: int,
)
    requires
        compositional(ty, ctor),
        bound(rules, binder),
        may_sound(rules, may),
        closed_hint(rules, types, binder, may, ty, ctor),
        0 <= i < rules.len(),
        body_derived(rules, i, val, depth),
        0 <= k < rules[i].head.args.len(),
        rules[i].head.args[k] == Term::Construct(sym, deps),
        0 <= e < deps.len(),
    ensures
        growth_edge(
            rules,
            types,
            ctor,
            ty(value(rules[i].head.args[k], val)),
            ty(val[deps[e] as int]),
        ),
{
    hint_sound(rules, types, binder, may, ty, ctor, depth);
    let args = deps.map_values(|x: nat| val[x as int]);
    let ts = args.map_values(ty);
    assert forall|d: int| 0 <= d < deps.len() implies types(i, deps[d]).contains(ts[d]) by {
        assert(deps.contains(deps[d]));
        assert(mentions(rules[i].head.args[k], deps[d]));
        assert(head_var(rules[i], deps[d]));
    }
    assert(ty(Value::Identity(sym, args)) == ctor(sym, ts));
    assert(ts[e] == ty(val[deps[e] as int]));
}

} // verus!
