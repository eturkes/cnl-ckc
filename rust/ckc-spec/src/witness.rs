use crate::arith::*;
use crate::emit::*;
use crate::engine::*;
use crate::replay::*;
use crate::term::*;
use crate::v1text::*;
use vstd::prelude::*;

verus! {

// Trusted spec: witness selection for formula rules (contract ws). A formula
// rule's proof obligation states its antecedent under one witness tuple: one
// exact count per formula operand. The default tuple (da2 D4) can break the
// rule's own negations, so the selection walks a finite set of exact
// candidates per operand, near the bounds that can test it, and takes the
// first tuple whose chain derives and whose obligation replays over the
// document's program. The clauses never change; only the witness pairs do.
// --- what a formula variant records (D4) ---
pub ghost struct Opnd {
    pub s: Term,  // the count's comparison variable (witness `exactly`)
    pub n: Term,  // the count's value variable = the tuple's coordinate
    pub cond: Option<
        (Term, Term),
    >,  // own condition (comparison, bound); None = `some`
}

pub ghost struct Sel {
    pub ops: Seq<Opnd>,  // the distinct operands, antecedent condition order
    pub chain: Seq<Term>,  // every formula's arithmetic chain, formula order
}

pub open spec fn exactly() -> Term {
    atom("exactly"@)
}

// The formula operands of a converted antecedent, once each, in condition
// order: a `$guideline_bound` item on a formula leaf (its own condition) or a
// `$guideline_amount` item (a `some` quantity: no bound; `emit::free_items`
// mints one for a formula leaf alone, so no leaf check is needed).
pub open spec fn operands(items: Seq<Item>, refs: Seq<Term>, seen: Seq<Term>) -> Seq<Opnd>
    decreases items.len(),
{
    if items.len() == 0 {
        Seq::empty()
    } else {
        let rest = items.drop_first();
        match items[0] {
            Item::Anch(_, inner) => if !seen.contains(arg(inner, 0)) && is_comp(
                inner,
                "$guideline_bound"@,
                8,
            ) && refs.contains(arg(inner, 0)) {
                seq![
                    Opnd {
                        s: arg(inner, 4),
                        n: arg(inner, 5),
                        cond: Option::Some((arg(inner, 6), arg(inner, 7))),
                    },
                ] + operands(rest, refs, seen.push(arg(inner, 0)))
            } else if !seen.contains(arg(inner, 0)) && is_comp(inner, "$guideline_amount"@, 6) {
                seq![Opnd { s: arg(inner, 4), n: arg(inner, 5), cond: Option::None }] + operands(
                    rest,
                    refs,
                    seen.push(arg(inner, 0)),
                )
            } else {
                operands(rest, refs, seen)
            },
            _ => operands(rest, refs, seen),
        }
    }
}

// --- D2: the default coordinate ---
// x + k for a number x and an integer k, exact.
pub open spec fn plus(x: Term, k: int) -> Term {
    match number(x) {
        Option::Some((p, q)) => canon(p + k * q, q),
        Option::None => x,
    }
}

// `some` → 1; `exactly N` → N; `geq N` → N, or 1 when N = 0; `leq N` → N, or
// −1 when N = 0; `greater N` → N + 1, or N + 2 when N + 1 = 0; `less N` →
// N − 1, or N − 2 when N − 1 = 0 (for an integer N = da2 D4's value).
pub open spec fn w0(cond: Option<(Term, Term)>) -> Term {
    match cond {
        Option::None => Term::Int(1),
        Option::Some((op, b)) => match number(b) {
            Option::None => b,
            Option::Some((p, q)) => if op == kw("geq"@) && p == 0 {
                Term::Int(1)
            } else if op == kw("leq"@) && p == 0 {
                Term::Int(-1)
            } else if op == kw("greater"@) {
                plus(
                    b,
                    if p + q == 0 {
                        2
                    } else {
                        1
                    },
                )
            } else if op == kw("less"@) {
                plus(
                    b,
                    if p == q {
                        -2
                    } else {
                        -1
                    },
                )
            } else {
                b
            },
        },
    }
}

// The operand's own condition admits the number v (`some` admits every
// number): the comparison the operand's clause runs on its witness count.
pub open spec fn admits(cond: Option<(Term, Term)>, v: Term) -> bool {
    number(v) is Some && match cond {
        Option::None => true,
        Option::Some((op, b)) => crate::compare::compare_holds(
            lit("guideline_satisfies"@, seq![exactly(), v, op, b]),
        ),
    }
}

// --- rational order ---
pub open spec fn lt(x: Term, y: Term) -> bool {
    match (number(x), number(y)) {
        (Option::Some((p1, q1)), Option::Some((p2, q2))) => p1 * q2 < p2 * q1,
        _ => false,
    }
}

// x into the ascending, duplicate-free numbers xs.
pub open spec fn insert(xs: Seq<Term>, x: Term) -> Seq<Term>
    decreases xs.len(),
{
    if xs.len() == 0 {
        seq![x]
    } else if xs[0] == x {
        xs
    } else if lt(x, xs[0]) {
        seq![x] + xs
    } else {
        seq![xs[0]] + insert(xs.drop_first(), x)
    }
}

// The distinct numbers of xs, ascending (number terms are canonical: equal
// values are equal terms).
pub open spec fn ascending(xs: Seq<Term>) -> Seq<Term>
    decreases xs.len(),
{
    if xs.len() == 0 {
        Seq::empty()
    } else {
        insert(ascending(xs.drop_last()), xs.last())
    }
}

// --- D3: boundaries + candidates ---
pub open spec fn counts(sel: Sel) -> Seq<Term> {
    sel.ops.map_values(|o: Opnd| o.n)
}

// The witness pairs with every operand count open.
pub open spec fn open_pairs(g: Group, sel: Sel) -> Seq<(Term, Term)> {
    g.pairs.filter(|p: (Term, Term)| !counts(sel).contains(p.0))
}

// The skeleton: the witness facts with every operand count open (a variable,
// the operand's slot), fixed before the walk.
pub open spec fn skeleton(g: Group, sel: Sel) -> Seq<Term> {
    if g.clauses.len() == 0 {
        Seq::empty()
    } else {
        positive_goals(g.clauses[0].body).map_values(|t: Term| bind_all(t, open_pairs(g, sel)))
    }
}

// The NAF items of a body, in order, outer variables bound by pairs.
pub open spec fn nafs_of(body: Seq<BodyItem>, pairs: Seq<(Term, Term)>) -> Seq<Seq<Term>>
    decreases body.len(),
{
    if body.len() == 0 {
        Seq::empty()
    } else {
        (match body[0] {
            BodyItem::Naf(gs) => seq![gs.map_values(|t: Term| bind_all(t, pairs))],
            BodyItem::Pos(_) => Seq::empty(),
        }) + nafs_of(body.drop_first(), pairs)
    }
}

// The variant's NAF items, outer variables bound as in the skeleton.
pub open spec fn naf_items(g: Group, sel: Sel) -> Seq<Seq<Term>> {
    if g.clauses.len() == 0 {
        Seq::empty()
    } else {
        nafs_of(g.clauses[0].body, open_pairs(g, sel))
    }
}

// Every outcome of matching the n goals gs, in order, each against one of the
// facts under one shared substitution (occurs check; bindings stay in their
// branch): the watched terms w with the branch's bindings applied.
pub open spec fn joins(n: nat, gs: Seq<Term>, facts: Seq<Term>, w: Seq<Term>) -> Seq<Seq<Term>>
    decreases n, facts.len() + 1,
{
    if n == 0 || gs.len() == 0 {
        seq![w]
    } else {
        joins_from(n, gs, facts, w, 0)
    }
}

pub open spec fn joins_from(n: nat, gs: Seq<Term>, facts: Seq<Term>, w: Seq<Term>, j: nat) -> Seq<
    Seq<Term>,
>
    decreases n, facts.len() - j,
{
    if n == 0 || gs.len() == 0 || j >= facts.len() {
        Seq::empty()
    } else {
        let k = gs.len() - 1;
        let here = match unify(
            UState {
                pairs: seq![(gs[0], facts[j as int])],
                stack: Seq::empty(),
                sol: w + gs.drop_first() + facts,
            },
        ) {
            UOut::Ok(_, s) => joins(
                (n - 1) as nat,
                s.subrange(w.len() as int, w.len() + k),
                s.skip(w.len() + k),
                s.take(w.len() as int),
            ),
            _ => Seq::empty(),
        };
        here + joins_from(n, gs, facts, w, j + 1)
    }
}

// The goals of gs other than the schema built-ins, in order.
pub open spec fn semantic_goals(gs: Seq<Term>) -> Seq<Term>
    decreases gs.len(),
{
    if gs.len() == 0 {
        Seq::empty()
    } else {
        (if is_builtin_lit(gs[0]) {
            Seq::empty()
        } else {
            seq![gs[0]]
        }) + semantic_goals(gs.drop_first())
    }
}

// The comparison c of the NAF item gs is relevant to a slot when some match
// of the item's semantic goals against the facts makes c's count input
// originate in that slot: both end as one unbound variable (an equal number
// from another fact is no origin).
pub open spec fn relevant(gs: Seq<Term>, facts: Seq<Term>, c: Term, slot: Term) -> bool {
    let sem = semantic_goals(gs);
    let js = joins(sem.len(), sem, facts, seq![arg(c, 1), slot]);
    exists|k: int|
        0 <= k < js.len() && (#[trigger] js[k]).len() == 2 && js[k][1] is Var && js[k][0]
            == js[k][1]
}

// The bounds of the comparisons among cs (goals of the NAF item gs) that are
// relevant to the slot.
pub open spec fn item_bounds(gs: Seq<Term>, cs: Seq<Term>, facts: Seq<Term>, slot: Term) -> Seq<
    Term,
>
    decreases cs.len(),
{
    if cs.len() == 0 {
        Seq::empty()
    } else {
        let c = cs[0];
        (if is_comp(c, "guideline_satisfies"@, 4) && number(arg(c, 3)) is Some && relevant(
            gs,
            facts,
            c,
            slot,
        ) {
            seq![arg(c, 3)]
        } else {
            Seq::empty()
        }) + item_bounds(gs, cs.drop_first(), facts, slot)
    }
}

pub open spec fn bounds_of(nafs: Seq<Seq<Term>>, facts: Seq<Term>, slot: Term) -> Seq<Term>
    decreases nafs.len(),
{
    if nafs.len() == 0 {
        Seq::empty()
    } else {
        item_bounds(nafs[0], nafs[0], facts, slot) + bounds_of(nafs.drop_first(), facts, slot)
    }
}

// The bounds of the NAF comparisons relevant to operand i.
pub open spec fn naf_bounds(g: Group, sel: Sel, i: int) -> Seq<Term> {
    bounds_of(naf_items(g, sel), skeleton(g, sel), sel.ops[i].n)
}

// B of operand i: its own bound, the relevant NAF bounds and 0, ascending.
pub open spec fn boundaries(g: Group, sel: Sel, i: int) -> Seq<Term> {
    let own = match sel.ops[i].cond {
        Option::Some((_, b)) => if number(b) is Some {
            seq![b]
        } else {
            Seq::empty()
        },
        Option::None => Seq::empty(),
    };
    ascending(own + naf_bounds(g, sel, i) + seq![Term::Int(0)])
}

pub open spec fn mid(x: Term, y: Term) -> Term {
    match (number(x), number(y)) {
        (Option::Some((p1, q1)), Option::Some((p2, q2))) => canon(p1 * q2 + p2 * q1, 2 * q1 * q2),
        _ => x,
    }
}

// The midpoint of each consecutive pair of the ascending bs.
pub open spec fn midpoints(bs: Seq<Term>) -> Seq<Term> {
    Seq::new(
        if bs.len() == 0 {
            0
        } else {
            (bs.len() - 1) as nat
        },
        |m: int| mid(bs[m], bs[m + 1]),
    )
}

// The candidates of operand i: its default first, then every number of B,
// b ± 1 and the consecutive midpoints that its own condition admits,
// ascending, duplicates dropped.
pub open spec fn candidates(g: Group, sel: Sel, i: int) -> Seq<Term> {
    let bs = boundaries(g, sel, i);
    let cond = sel.ops[i].cond;
    let d = w0(cond);
    seq![d] + ascending(
        (bs + bs.map_values(|b: Term| plus(b, -1)) + bs.map_values(|b: Term| plus(b, 1))
            + midpoints(bs)).filter(|x: Term| admits(cond, x) && x != d),
    )
}

// --- D4: the odometer ---
pub open spec fn cand_lists(g: Group, sel: Sel) -> Seq<Seq<Term>> {
    Seq::new(sel.ops.len(), |i: int| candidates(g, sel, i))
}

pub open spec fn tuple_count(cs: Seq<Seq<Term>>) -> nat
    decreases cs.len(),
{
    if cs.len() == 0 {
        1
    } else {
        tuple_count(cs.drop_last()) * cs.last().len()
    }
}

// The digits of tuple i, the last operand varying fastest.
pub open spec fn digits(cs: Seq<Seq<Term>>, i: nat) -> Seq<nat>
    decreases cs.len(),
{
    if cs.len() == 0 {
        Seq::empty()
    } else {
        digits(cs.drop_last(), i / cs.last().len()) + seq![i % cs.last().len()]
    }
}

pub open spec fn nth_tuple(cs: Seq<Seq<Term>>, i: nat) -> Seq<Term> {
    Seq::new(cs.len(), |k: int| cs[k][digits(cs, i)[k] as int])
}

// --- D5: the winner ---
// The operands' pairs under the tuple t: each count exact.
pub open spec fn op_pairs(sel: Sel, t: Seq<Term>) -> Seq<(Term, Term)> {
    Seq::new(
        sel.ops.len(),
        |i: int| seq![(sel.ops[i].s, exactly()), (sel.ops[i].n, t[i])],
    ).flatten()
}

// The variant's witness pairs under t: each operand count and each chain
// output re-valued, every other pair kept; None = a chain node derives
// nothing.
pub open spec fn tuple_pairs(g: Group, sel: Sel, t: Seq<Term>) -> Option<Seq<(Term, Term)>> {
    match chain_pairs(sel.chain, op_pairs(sel, t)) {
        Option::Some(vs) => Option::Some(
            g.pairs.map_values(
                |p: (Term, Term)|
                    match lookup(vs, p.0) {
                        Option::Some(v) => (p.0, v),
                        Option::None => p,
                    },
            ),
        ),
        Option::None => Option::None,
    }
}

// The replay of head h of the variant's obligation under t over the program
// p: the K2 relation of `ckc v1 aggregate-check` (the witness facts asserted
// in front of p, the replay bounds, a negation cut by a bound fails closed).
pub open spec fn replay_of(p: Seq<DocClause>, g: Group, sel: Sel, t: Seq<Term>, h: int) -> Option<
    ROut,
> {
    match tuple_pairs(g, sel, t) {
        Option::Some(pairs) => {
            let o = obligation(Group { pairs, ..g }, Seq::empty(), 0);
            Option::Some(
                solve(
                    witness_db(o.facts, p),
                    o.heads[h],
                    replay_depth(),
                    Seq::empty(),
                    false,
                    replay_inf(),
                ),
            )
        },
        Option::None => Option::None,
    }
}

// t wins: every chain node derives and every head of the obligation replays.
pub open spec fn win(p: Seq<DocClause>, g: Group, sel: Sel, t: Seq<Term>) -> bool {
    match tuple_pairs(g, sel, t) {
        Option::Some(pairs) => {
            let o = obligation(Group { pairs, ..g }, Seq::empty(), 0);
            heads_proved(witness_db(o.facts, p), o.heads)
        },
        Option::None => false,
    }
}

// The first winning tuple index from i on.
pub open spec fn first_win(p: Seq<DocClause>, g: Group, sel: Sel, i: nat) -> Option<nat>
    decreases tuple_count(cand_lists(g, sel)) - i,
{
    if i >= tuple_count(cand_lists(g, sel)) {
        Option::None
    } else if win(p, g, sel, nth_tuple(cand_lists(g, sel), i)) {
        Option::Some(i)
    } else {
        first_win(p, g, sel, i + 1)
    }
}

// --- D6: the selection ---
// A formula variant takes its first winner's pairs; without a winner it keeps
// its default pairs (the replay then refuses its obligation).
pub open spec fn select_group(p: Seq<DocClause>, g: Group) -> Group {
    match g.sel {
        Option::Some(sel) => match first_win(p, g, sel, 0) {
            Option::Some(i) => match tuple_pairs(g, sel, nth_tuple(cand_lists(g, sel), i)) {
                Option::Some(pairs) => Group { pairs, ..g },
                Option::None => g,
            },
            Option::None => g,
        },
        Option::None => g,
    }
}

// The selection over a document's projection; p = the document's program as
// the aggregate loads it (`replay::doc_db`: its two records, then every
// clause in file order; no body calls a record).
pub open spec fn select_doc(ps: Seq<Projected>, p: Seq<DocClause>) -> Seq<Projected> {
    ps.map_values(
        |pr: Projected|
            Projected { groups: pr.groups.map_values(|g: Group| select_group(p, g)), ..pr },
    )
}

// --- D7: the completeness fragment ---
// Inside it, tuples whose coordinates share their cells (per operand: the
// same side of, or equal to, every boundary) replay alike (contract.rs
// `ws_cell_invariance`), so a winner exists whenever some admitted rational
// tuple wins (`ws_complete`). The recognizer walks the replay of each head
// over the skeleton, its slots standing for the tuple's coordinates: every
// call resolves against facts alone, binds no slot, a slot meets a
// comparison only as its count input, against a number of the slot's B, and
// each arithmetic goal is its chain node, reading each operand's own slot.
pub open spec fn outputs(sel: Sel) -> Seq<Term> {
    sel.chain.map_values(|c: Term| seq![arg(c, 4), arg(c, 5)]).flatten()
}

// Head j under the witness pairs, operand counts and chain outputs open.
pub open spec fn open_head(g: Group, sel: Sel, j: int) -> Term {
    bind_all(
        g.clauses[j].head,
        g.pairs.filter(|p: (Term, Term)| !counts(sel).contains(p.0) && !outputs(sel).contains(p.0)),
    )
}

pub open spec fn unifiable(a: Term, b: Term) -> bool {
    unify(UState { pairs: seq![(a, b)], stack: Seq::empty(), sol: Seq::empty() }) is Ok
}

pub open spec fn ground_fact(c: DocClause) -> bool {
    c.body.len() == 0 && ground(c.head)
}

pub open spec fn mentions(t: Term, slot: Term) -> bool {
    match slot {
        Term::Var(x) => occurs(x, t),
        _ => false,
    }
}

// What the walk reads: the program, the facts a call may match (the skeleton
// + the program's ground facts), the slots, each slot's B, the chain, and an
// offset past every variable of the walk (rules are renamed above it).
pub ghost struct WalkEnv {
    pub p: Seq<DocClause>,
    pub facts: Seq<Term>,
    pub slots: Seq<Term>,
    pub bounds: Seq<Seq<Term>>,
    pub chain: Seq<Term>,
    pub off: nat,
}

pub open spec fn slot_free(e: WalkEnv, t: Term) -> bool {
    forall|i: int| 0 <= i < e.slots.len() ==> !mentions(t, #[trigger] e.slots[i])
}

// A built-in reads a slot only as a comparison's count input, against a
// number of that slot's B.
pub open spec fn test_ok(e: WalkEnv, c: Term) -> bool {
    ||| slot_free(e, c)
    ||| exists|i: int|
        0 <= i < e.slots.len() && is_comp(c, "guideline_satisfies"@, 4) && arg(c, 1)
            == #[trigger] e.slots[i] && slot_free(e, arg(c, 0)) && slot_free(e, arg(c, 2))
            && e.bounds[i].contains(arg(c, 3))
}

// Input pair q (comparator q, count q + 1) of a walked arithmetic goal g
// against its chain node c: an operand count = that operand's own slot under
// `exactly`; an earlier output = a variable pair; an exact literal = itself.
pub open spec fn input_ok(e: WalkEnv, g: Term, c: Term, q: int) -> bool {
    if e.slots.contains(arg(c, q + 1)) {
        arg(g, q + 1) == arg(c, q + 1) && arg(g, q) == exactly()
    } else if arg(c, q + 1) is Var {
        arg(g, q) is Var && arg(g, q + 1) is Var
    } else {
        arg(g, q) == arg(c, q) && arg(g, q + 1) == arg(c, q + 1)
    }
}

// The walk's arithmetic goal g = its chain node c: the same operation, both
// input pairs as `input_ok`, the output pair variables. A goal matching a
// ground fact in place of its operand's skeleton fact would feed the chain a
// constant, and the head's output would then decide the branch by equality.
pub open spec fn arith_ok(e: WalkEnv, g: Term, c: Term) -> bool {
    &&& match (g, c) {
        (Term::Comp(gn, ga), Term::Comp(cn, ca)) => gn == cn && ga.len() == ca.len(),
        _ => false,
    }
    &&& input_ok(e, g, c, 0)
    &&& input_ok(e, g, c, 2)
    &&& arg(g, 4) is Var && arg(g, 5) is Var
}

// The walk of one branch: the remaining goals, bindings applied, k = the next
// chain node (fuel n; out of fuel = outside the fragment). A negation walks
// its inner goals from the branch's bindings, then the branch goes on.
pub open spec fn walk_ok(e: WalkEnv, gs: Seq<Term>, n: nat, k: nat) -> bool
    decreases n, 0nat,
{
    if gs.len() == 0 {
        true
    } else if n == 0 {
        false
    } else {
        let g = gs[0];
        let rest = gs.drop_first();
        match g {
            Term::Comp(name, args) => if name == comma_name() && args.len() == 2 {
                walk_ok(e, seq![args[0], args[1]] + rest, (n - 1) as nat, k)
            } else if name == naf_name() && args.len() == 1 {
                walk_ok(e, seq![args[0]], (n - 1) as nat, k) && walk_ok(e, rest, (n - 1) as nat, k)
            } else if crate::arith::is_arith(name, args.len()) {
                k < e.chain.len() && arith_ok(e, g, e.chain[k as int]) && walk_ok(
                    e,
                    rest,
                    (n - 1) as nat,
                    k + 1,
                )
            } else if crate::compare::is_builtin(name, args.len()) {
                test_ok(e, g) && walk_ok(e, rest, (n - 1) as nat, k)
            } else if name == operator_name() {
                false
            } else {
                call_ok(e, g, rest, (n - 1) as nat, k)
            },
            _ => true,
        }
    }
}

// A call: no rule of the program (a clause other than a ground fact) can
// resolve it, and every fact it matches binds no slot and continues the walk.
pub open spec fn call_ok(e: WalkEnv, g: Term, rest: Seq<Term>, n: nat, k: nat) -> bool
    decreases n, e.facts.len() + 1,
{
    &&& forall|k: int|
        0 <= k < e.p.len() && !ground_fact(#[trigger] e.p[k]) ==> !unifiable(
            g,
            shift(e.p[k].head, e.off),
        )
    &&& facts_ok(e, g, rest, n, k, 0)
}

pub open spec fn facts_ok(e: WalkEnv, g: Term, rest: Seq<Term>, n: nat, k: nat, j: nat) -> bool
    decreases n, e.facts.len() - j,
{
    if j >= e.facts.len() {
        true
    } else {
        (match unify(
            UState {
                pairs: seq![(g, e.facts[j as int])],
                stack: Seq::empty(),
                sol: rest + e.slots,
            },
        ) {
            UOut::Ok(_, s) => s.skip(rest.len() as int) == e.slots && walk_ok(
                e,
                s.take(rest.len() as int),
                n,
                k,
            ),
            UOut::Fail => true,
            UOut::Out => false,
        }) && facts_ok(e, g, rest, n, k, j + 1)
    }
}

// Goal nodes of a body term: a conjunction or negation node + its parts.
pub open spec fn goal_nodes(t: Term) -> nat
    decreases t,
{
    match t {
        Term::Comp(name, args) => if name == comma_name() && args.len() == 2 {
            1 + goal_nodes(args[0]) + goal_nodes(args[1])
        } else if name == naf_name() && args.len() == 1 {
            1 + goal_nodes(args[0])
        } else {
            1
        },
        _ => 1,
    }
}

pub open spec fn nodes_all(ts: Seq<Term>) -> nat
    decreases ts.len(),
{
    if ts.len() == 0 {
        0
    } else {
        goal_nodes(ts[0]) + nodes_all(ts.drop_first())
    }
}

// K2 calls the head as a predicate: a comparison, an arithmetic goal, a
// conjunction or a negation would be decided at the head itself.
pub open spec fn head_call(h: Term) -> bool {
    primitive(h) && match h {
        Term::Comp(name, args) => !crate::compare::is_builtin(name, args.len()),
        _ => false,
    }
}

// Clause j: its open head is a predicate call that reaches exactly its own
// canonical clause c of p (no skeleton fact, no other clause) and holds no
// slot, and the walk of c's body from that unification (c renamed above the
// skeleton, as a call renames it) stays inside the fragment.
pub open spec fn clause_ok(p: Seq<DocClause>, g: Group, sel: Sel, j: int) -> bool {
    let c = canon_clause(g.clauses[j]);
    let h = open_head(g, sel, j);
    let sk = skeleton(g, sel);
    let off = max_nat(nvars_all(sk), nvars(h));
    let body = c.body.map_values(|it: BodyItem| item_term(it, off));
    let e = WalkEnv {
        p,
        facts: sk + p.filter(|d: DocClause| ground_fact(d)).map_values(|d: DocClause| d.head),
        slots: counts(sel),
        bounds: Seq::new(sel.ops.len(), |i: int| boundaries(g, sel, i)),
        chain: sel.chain,
        off: off + clause_nvars(c),
    };
    &&& p.contains(c)
    &&& head_call(h)
    &&& slot_free(e, h)
    &&& forall|k: int|
        0 <= k < p.len() && unifiable(h, shift((#[trigger] p[k]).head, off)) ==> p[k] == c
    &&& forall|k: int| 0 <= k < sk.len() ==> !unifiable(h, #[trigger] sk[k])
    &&& match unify(
        UState { pairs: seq![(h, shift(c.head, off))], stack: Seq::empty(), sol: body },
    ) {
        UOut::Ok(_, b) => walk_ok(e, b, 1 + nodes_all(body), 0),
        _ => false,
    }
}

// The slots are distinct variables, each held once by the skeleton, the
// skeleton's only variables.
pub open spec fn fixed_skeleton(g: Group, sel: Sel) -> bool {
    let sk = skeleton(g, sel);
    &&& counts(sel).no_duplicates()
    &&& forall|i: int|
        0 <= i < sel.ops.len() ==> match (#[trigger] sel.ops[i]).n {
            Term::Var(x) => occ_all(x, sk) == 1,
            _ => false,
        }
    &&& forall|x: nat| #[trigger] vars_all(sk).contains(x) ==> counts(sel).contains(Term::Var(x))
}

pub open spec fn is_arith_lit(t: Term) -> bool {
    match t {
        Term::Comp(name, args) => crate::arith::is_arith(name, args.len()),
        _ => false,
    }
}

pub open spec fn naf_goals(body: Seq<BodyItem>) -> Seq<Term> {
    body.map_values(
        |it: BodyItem|
            match it {
                BodyItem::Naf(gs) => gs,
                BodyItem::Pos(_) => Seq::empty(),
            },
    ).flatten()
}

// A clause body's arithmetic = the variant's chain, in order (binding-first
// keeps a postorder chain's order; where it would not, the variant only falls
// outside the fragment), outside every negation, and no other goal reads a
// chain output.
pub open spec fn chain_body(sel: Sel, body: Seq<BodyItem>) -> bool {
    let pos = pos_terms(body.filter(|it: BodyItem| it is Pos));
    let others = pos.filter(|t: Term| !is_arith_lit(t)) + naf_goals(body);
    &&& pos.filter(|t: Term| is_arith_lit(t)) == sel.chain
    &&& forall|k: int|
        0 <= k < naf_goals(body).len() ==> !is_arith_lit(#[trigger] naf_goals(body)[k])
    &&& forall|k: int, m: int|
        0 <= k < others.len() && 0 <= m < outputs(sel).len() ==> !mentions(
            #[trigger] others[k],
            #[trigger] outputs(sel)[m],
        )
}

// A compound literal, not a control term (`,` or `\+`) nor a variable a
// substitution could fill: what emission writes as a body item or a
// negation's goal.
pub open spec fn primitive(t: Term) -> bool {
    match t {
        Term::Comp(name, args) => !(name == comma_name() && args.len() == 2) && !(name == naf_name()
            && args.len() == 1),
        _ => false,
    }
}

pub open spec fn primitive_body(body: Seq<BodyItem>) -> bool {
    forall|k: int|
        0 <= k < body.len() ==> match #[trigger] body[k] {
            BodyItem::Pos(t) => primitive(t),
            BodyItem::Naf(gs) => gs.len() > 0 && forall|m: int|
                0 <= m < gs.len() ==> primitive(#[trigger] gs[m]),
        }
}

// The default pairs bind each operand's comparison to `exactly` and its count,
// as every tuple's facts do (the skeleton = those facts but for the slots).
pub open spec fn exact_comparators(g: Group, sel: Sel) -> bool {
    forall|i: int|
        0 <= i < sel.ops.len() ==> lookup(g.pairs, (#[trigger] sel.ops[i]).s) == Option::Some(
            exactly(),
        ) && lookup(g.pairs, sel.ops[i].n) is Some
}

pub open spec fn in_fragment(p: Seq<DocClause>, g: Group, sel: Sel) -> bool {
    &&& g.clauses.len() > 0
    &&& forall|k: int| 0 <= k < g.pairs.len() ==> ground((#[trigger] g.pairs[k]).1)
    &&& chain_shape(sel)
    &&& exact_comparators(g, sel)
    &&& fixed_skeleton(g, sel)
    &&& forall|j: int|
        0 <= j < g.clauses.len() ==> primitive_body((#[trigger] g.clauses[j]).body) && chain_body(
            sel,
            g.clauses[j].body,
        )
    &&& forall|j: int| 0 <= j < g.clauses.len() ==> #[trigger] clause_ok(p, g, sel, j)
}

// --- theorem vocabulary (contract.rs) ---
// Two numbers share their cell of the ascending bs: the same side of, or
// equal to, every boundary.
pub open spec fn same_cell(bs: Seq<Term>, x: Term, y: Term) -> bool {
    &&& number(x) is Some && number(y) is Some
    &&& forall|k: int|
        0 <= k < bs.len() ==> lt(x, #[trigger] bs[k]) == lt(y, bs[k]) && (x == bs[k]) == (y
            == bs[k])
}

pub open spec fn defaults(sel: Sel) -> Seq<Term> {
    sel.ops.map_values(|o: Opnd| w0(o.cond))
}

// A chain input pair: an operand's count, an exact number, or an earlier
// node's output.
pub open spec fn chain_input(sel: Sel, k: int, s: Term, n: Term) -> bool {
    ||| exists|i: int| 0 <= i < sel.ops.len() && s == (#[trigger] sel.ops[i]).s && n == sel.ops[i].n
    ||| s == exactly() && number(n) is Some
    ||| exists|m: int|
        0 <= m < k && s == arg(#[trigger] sel.chain[m], 4) && n == arg(sel.chain[m], 5)
}

// The chain as the formula pass builds it (da2 D3): products and quotients
// over chain inputs, the operand and output variables all distinct.
pub open spec fn chain_shape(sel: Sel) -> bool {
    &&& forall|k: int|
        0 <= k < sel.chain.len() ==> is_arith_lit(#[trigger] sel.chain[k]) && chain_input(
            sel,
            k,
            arg(sel.chain[k], 0),
            arg(sel.chain[k], 1),
        ) && chain_input(sel, k, arg(sel.chain[k], 2), arg(sel.chain[k], 3))
    &&& forall|i: int|
        0 <= i < sel.ops.len() ==> (#[trigger] sel.ops[i]).s is Var && sel.ops[i].n is Var
    &&& forall|m: int| 0 <= m < outputs(sel).len() ==> (#[trigger] outputs(sel)[m]) is Var
    &&& (sel.ops.map_values(|o: Opnd| o.s) + counts(sel) + outputs(sel)).no_duplicates()
}

// --- the walk report (contract ws P6) ---
// A formula variant's candidates per operand, the winner's digits (else
// `none`: the whole product walked) and whether the variant lies in the
// fragment.
pub open spec fn walk_term(p: Seq<DocClause>, g: Group, sel: Sel, s: nat) -> Term {
    let cs = cand_lists(g, sel);
    Term::Comp(
        ascii("$guideline_walk"@),
        seq![
            Term::Int(s as int),
            variant_slot(g.k),
            list_term(cs.map_values(|c: Seq<Term>| list_term(c))),
            match first_win(p, g, sel, 0) {
                Option::Some(i) => Term::Comp(
                    ascii("winner"@),
                    seq![list_term(digits(cs, i).map_values(|d: nat| Term::Int(d as int)))],
                ),
                Option::None => atom("none"@),
            },
            Term::Comp(
                ascii("fragment"@),
                seq![
                    atom(
                        if in_fragment(p, g, sel) {
                            "true"@
                        } else {
                            "false"@
                        },
                    ),
                ],
            ),
        ],
    )
}

pub open spec fn walks(ps: Seq<Projected>, p: Seq<DocClause>) -> Seq<Term> {
    ps.map_values(
        |pr: Projected|
            pr.groups.filter(|g: Group| g.sel is Some).map_values(
                |g: Group| walk_term(p, g, g.sel.unwrap(), pr.s),
            ),
    ).flatten()
}

// `ckc certify-one walks`: a certified document's walk reports, one line per
// formula variant in document order; a refusal = `certify-one doc`'s.
pub open spec fn certify_walks_output(
    ace: Seq<u8>,
    asha: Seq<u8>,
    usha: Option<Seq<u8>>,
    docid: Seq<u8>,
    dump: Seq<u8>,
    pl: Seq<u8>,
    traw: Option<Seq<u8>>,
    tsha: Option<Seq<u8>>,
) -> Out {
    match certify_doc(ace, asha, usha, docid, dump, pl, traw, tsha) {
        Result::Err(why) => certify_reject(docid, why),
        Result::Ok(_) => {
            let d = the_dump(dump);
            let lines = nonempty_lines(ace);
            match (
                the_v1(pl),
                project(
                    d.drs,
                    docid,
                    lines.len(),
                    table_of(traw, tsha).unwrap(),
                    srcs_of(d.sentences, lines),
                ),
            ) {
                (V1File::Doc(doc), Result::Ok(ps)) => ok(
                    walks(ps, doc_db(doc)).map_values(|t: Term| term_line(t)).flatten(),
                ),
                _ => certify_reject(docid, atom("walks"@)),
            }
        },
    }
}

} // verus!
