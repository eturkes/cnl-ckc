use vstd::prelude::*;
use crate::term::*;
use crate::v1text::*;

verus! {

// Trusted spec: the bounded v1 engine (contract m5u2 R8/R13/R14/R15/R16).
// One deterministic machine = SWI's depth-first, leftmost, clause-order
// search with negation as failure, occurs-check unification (R13) and two
// bounds: a depth budget per goal (call_with_depth_limit) and one fuel of
// machine transitions (call_with_inference_limit; SWI's outer run limit
// counts the same inferences at 10x, so the per-call limit alone binds).
// Every transition costs one fuel; the exact fuel accounting is this
// machine's own law — committed queries sit far below every bound (R15).

// --- R8 constants ---

pub open spec fn replay_depth() -> nat { 4000 }
pub open spec fn replay_inf() -> nat { 1000000 }
pub open spec fn answer_depth() -> nat { 100 }
pub open spec fn answer_inf() -> nat { 100000 }
pub open spec fn trace_depth() -> nat { 1000 }
pub open spec fn trace_inf() -> nat { 100000 }

pub open spec fn comma_name() -> Seq<u8> { ascii(","@) }
pub open spec fn naf_name() -> Seq<u8> { ascii("\\+"@) }

// --- term helpers ---

pub open spec fn max_nat(a: nat, b: nat) -> nat { if a >= b { a } else { b } }

pub open spec fn args_of(t: Term) -> Seq<Term> {
    match t { Term::Comp(_, args) => args, _ => Seq::empty() }
}

// Rename apart: every variable index moves up by `off`.
pub open spec fn shift(t: Term, off: nat) -> Term
    decreases t,
{
    match t {
        Term::Var(k) => Term::Var(k + off),
        Term::Comp(name, args) => Term::Comp(name, shift_all(args, off)),
        _ => t,
    }
}

pub open spec fn shift_all(ts: Seq<Term>, off: nat) -> Seq<Term>
    decreases ts,
{
    if ts.len() == 0 { Seq::empty() } else { seq![shift(ts[0], off)] + shift_all(ts.drop_first(), off) }
}

// 1 + the largest variable index (0 when ground).
pub open spec fn nvars(t: Term) -> nat
    decreases t,
{
    match t {
        Term::Var(k) => k + 1,
        Term::Comp(_, args) => nvars_all(args),
        _ => 0,
    }
}

pub open spec fn nvars_all(ts: Seq<Term>) -> nat
    decreases ts,
{
    if ts.len() == 0 { 0 } else { max_nat(nvars(ts[0]), nvars_all(ts.drop_first())) }
}

pub open spec fn occurs(x: nat, t: Term) -> bool
    decreases t,
{
    match t {
        Term::Var(k) => k == x,
        Term::Comp(_, args) => occurs_all(x, args),
        _ => false,
    }
}

pub open spec fn occurs_all(x: nat, ts: Seq<Term>) -> bool
    decreases ts,
{
    ts.len() > 0 && (occurs(x, ts[0]) || occurs_all(x, ts.drop_first()))
}

pub open spec fn subst(t: Term, x: nat, v: Term) -> Term
    decreases t,
{
    match t {
        Term::Var(k) => if k == x { v } else { t },
        Term::Comp(name, args) => Term::Comp(name, subst_all(args, x, v)),
        _ => t,
    }
}

pub open spec fn subst_all(ts: Seq<Term>, x: nat, v: Term) -> Seq<Term>
    decreases ts,
{
    if ts.len() == 0 { Seq::empty() } else { seq![subst(ts[0], x, v)] + subst_all(ts.drop_first(), x, v) }
}

pub open spec fn zip(xs: Seq<Term>, ys: Seq<Term>) -> Seq<(Term, Term)> {
    Seq::new(xs.len(), |i: int| (xs[i], ys[i]))
}

// Right-nested ','/2 over a nonempty literal list (a NAF body item).
pub open spec fn conj_term(gs: Seq<Term>) -> Term
    decreases gs.len(),
{
    if gs.len() <= 1 { gs[0] } else { Term::Comp(comma_name(), seq![gs[0], conj_term(gs.drop_first())]) }
}

pub open spec fn list_term(ts: Seq<Term>) -> Term
    decreases ts.len(),
{
    if ts.len() == 0 { Term::Nil } else { Term::Comp(cons_name(), seq![ts[0], list_term(ts.drop_first())]) }
}

// --- the machine ---

pub ghost enum Goal {
    Lit(Term, nat),   // goal term + remaining depth
    NafCut(nat),      // inner NAF proof succeeded: discard alternatives above the level
}

pub ghost struct Alt {   // choicepoint = the state to resume, ci = next clause index to try
    pub stack: Seq<Goal>,
    pub sol: Seq<Term>,
    pub fresh: nat,
    pub ci: nat,
}

pub ghost struct Cfg {
    pub stack: Seq<Goal>,
    pub sol: Seq<Term>,                   // solution template (answer variables), bindings applied
    pub alts: Seq<Alt>,
    pub fresh: nat,                       // every live variable index is below fresh
    pub ci: nat,                          // clause index the front goal resumes from
    pub uni: Option<Seq<(Term, Term)>>,   // pending unification pairs (Some = unifying)
    pub pruned: bool,                     // some branch hit depth 0
    pub rows: Seq<Seq<Term>>,             // collected solutions
    pub collect: bool,                    // true = findall, false = stop at the first solution
}

pub ghost enum Step { Next(Cfg), Sol, Done(Cfg) }

pub ghost enum ROut {
    Sol,
    End { complete: bool, rows: Seq<Seq<Term>> },   // complete = exhausted without prune or fuel-out
}

pub open spec fn subst_goal(g: Goal, x: nat, v: Term) -> Goal {
    match g { Goal::Lit(t, d) => Goal::Lit(subst(t, x, v), d), Goal::NafCut(l) => Goal::NafCut(l) }
}

pub open spec fn item_term(it: BodyItem, off: nat) -> Term {
    match it {
        BodyItem::Pos(l) => shift(l, off),
        BodyItem::Naf(gs) => Term::Comp(naf_name(), seq![conj_term(shift_all(gs, off))]),
    }
}

pub open spec fn body_goals(items: Seq<BodyItem>, off: nat, d: nat) -> Seq<Goal> {
    items.map_values(|it: BodyItem| Goal::Lit(item_term(it, off), d))
}

pub open spec fn items_nvars(items: Seq<BodyItem>) -> nat
    decreases items,
{
    if items.len() == 0 { 0 } else {
        let n = match items[0] { BodyItem::Pos(l) => nvars(l), BodyItem::Naf(gs) => nvars_all(gs) };
        max_nat(n, items_nvars(items.drop_first()))
    }
}

pub open spec fn clause_nvars(c: DocClause) -> nat { max_nat(nvars(c.head), items_nvars(c.body)) }

pub open spec fn lit_fa(t: Term) -> Option<(Seq<u8>, nat)> {
    match t { Term::Comp(name, args) => Option::Some((name, args.len())), _ => Option::None }
}

// Least clause index >= from whose head has the goal's name/arity.
pub open spec fn next_match(db: Seq<DocClause>, name: Seq<u8>, arity: nat, from: nat) -> Option<nat>
    decreases db.len() - from,
{
    if from >= db.len() { Option::None }
    else if lit_fa(db[from as int].head) == Option::Some((name, arity)) { Option::Some(from) }
    else { next_match(db, name, arity, from + 1) }
}

// Backtrack: resume the newest choicepoint, or end the search.
pub open spec fn fail(c: Cfg) -> Step {
    if c.alts.len() == 0 { Step::Done(c) } else {
        let a = c.alts.last();
        Step::Next(Cfg { stack: a.stack, sol: a.sol, fresh: a.fresh, ci: a.ci, alts: c.alts.drop_last(), uni: Option::None, ..c })
    }
}

pub open spec fn continue_with(c: Cfg, pairs: Seq<(Term, Term)>) -> Step {
    Step::Next(Cfg { uni: Option::Some(pairs), ..c })
}

// Bind x := v everywhere live (pairs, stack, template); choicepoints keep
// their own snapshots. Occurs-check failure = unification failure (R13).
pub open spec fn bind(c: Cfg, pairs: Seq<(Term, Term)>, x: nat, v: Term) -> Step {
    if occurs(x, v) { fail(c) } else {
        Step::Next(Cfg {
            uni: Option::Some(pairs.map_values(|p: (Term, Term)| (subst(p.0, x, v), subst(p.1, x, v)))),
            stack: c.stack.map_values(|g: Goal| subst_goal(g, x, v)),
            sol: subst_all(c.sol, x, v),
            ..c
        })
    }
}

pub open spec fn unify_step(c: Cfg, pairs: Seq<(Term, Term)>) -> Step {
    if pairs.len() == 0 { Step::Next(Cfg { uni: Option::None, ci: 0, ..c }) } else {
        let a = pairs[0].0;
        let b = pairs[0].1;
        let rest = pairs.drop_first();
        match (a, b) {
            (Term::Var(x), _) => if a == b { continue_with(c, rest) } else { bind(c, rest, x, b) },
            (_, Term::Var(y)) => bind(c, rest, y, a),
            (Term::Comp(n, xs), Term::Comp(m, ys)) =>
                if n == m && xs.len() == ys.len() { continue_with(c, zip(xs, ys) + rest) } else { fail(c) },
            _ => if a == b { continue_with(c, rest) } else { fail(c) },
        }
    }
}

// Resolve the front goal against clause m: push the retry choicepoint,
// rename the clause apart at `fresh`, unify the arguments, then run the body.
pub open spec fn resolve(db: Seq<DocClause>, m: nat, c: Cfg, args: Seq<Term>, d: nat, rest: Seq<Goal>) -> Step {
    let cl = db[m as int];
    Step::Next(Cfg {
        alts: c.alts.push(Alt { stack: c.stack, sol: c.sol, fresh: c.fresh, ci: m + 1 }),
        stack: body_goals(cl.body, c.fresh, (d - 1) as nat) + rest,
        uni: Option::Some(zip(args, args_of(shift(cl.head, c.fresh)))),
        fresh: c.fresh + clause_nvars(cl),
        ..c
    })
}

pub open spec fn step(db: Seq<DocClause>, c: Cfg) -> Step {
    match c.uni {
        Option::Some(pairs) => unify_step(c, pairs),
        Option::None => if c.stack.len() == 0 {
            if c.collect { fail(Cfg { rows: c.rows.push(c.sol), ..c }) } else { Step::Sol }
        } else {
            let rest = c.stack.drop_first();
            match c.stack[0] {
                Goal::NafCut(lvl) => fail(Cfg { alts: c.alts.take(lvl as int), ..c }),
                Goal::Lit(g, d) => if d == 0 { fail(Cfg { pruned: true, ..c }) } else {
                    match g {
                        Term::Comp(name, args) =>
                            if name == comma_name() && args.len() == 2 {
                                Step::Next(Cfg { stack: seq![Goal::Lit(args[0], d), Goal::Lit(args[1], d)] + rest, ..c })
                            } else if name == naf_name() && args.len() == 1 {
                                let lvl = c.alts.len();
                                Step::Next(Cfg {
                                    alts: c.alts.push(Alt { stack: rest, sol: c.sol, fresh: c.fresh, ci: 0 }),
                                    stack: seq![Goal::Lit(args[0], (d - 1) as nat), Goal::NafCut(lvl)] + rest,
                                    ..c
                                })
                            } else {
                                match next_match(db, name, args.len(), c.ci) {
                                    Option::None => fail(c),
                                    Option::Some(m) => resolve(db, m, c, args, d, rest),
                                }
                            },
                        _ => fail(c),
                    }
                },
            }
        },
    }
}

pub open spec fn run(db: Seq<DocClause>, c: Cfg, fuel: nat) -> ROut
    decreases fuel,
{
    if fuel == 0 { ROut::End { complete: false, rows: c.rows } } else {
        match step(db, c) {
            Step::Next(c2) => run(db, c2, (fuel - 1) as nat),
            Step::Sol => ROut::Sol,
            Step::Done(c2) => ROut::End { complete: !c2.pruned, rows: c2.rows },
        }
    }
}

pub open spec fn solve(db: Seq<DocClause>, goal: Term, depth: nat, sol: Seq<Term>, collect: bool, fuel: nat) -> ROut {
    run(db, Cfg {
        stack: seq![Goal::Lit(goal, depth)],
        sol,
        alts: Seq::empty(),
        fresh: max_nat(nvars(goal), nvars_all(sol)),
        ci: 0,
        uni: Option::None,
        pruned: false,
        rows: Seq::empty(),
        collect,
    }, fuel)
}

// --- replay + recursion probes ---

pub open spec fn fact_clause(t: Term) -> DocClause { DocClause { head: t, body: Seq::empty() } }

// asserta order: the last witness fact ends up frontmost.
pub open spec fn witness_db(facts: Seq<Term>, db: Seq<DocClause>) -> Seq<DocClause> {
    facts.reverse().map_values(|f: Term| fact_clause(f)) + db
}

pub open spec fn head_proved(db: Seq<DocClause>, head: Term) -> bool {
    solve(db, head, replay_depth(), Seq::empty(), false, replay_inf()) is Sol
}

pub open spec fn heads_proved(db: Seq<DocClause>, heads: Seq<Term>) -> bool {
    forall|i: int| 0 <= i < heads.len() ==> head_proved(db, #[trigger] heads[i])
}

// Occurs-check unifiability of a goal with a renamed-apart head = the
// machine on a one-fact database reaching a solution under some fuel.
pub open spec fn unifiable_apart(goal: Term, head: Term) -> bool {
    exists|fuel: nat| solve(seq![fact_clause(head)], goal, 1, Seq::empty(), false, fuel) is Sol
}

// --- R16 standard order (comparator matrix pins) ---

pub open spec fn rank(t: Term) -> int {
    match t { Term::Var(_) => 0, Term::Int(_) => 1, Term::Nil => 2, Term::Atom(_) => 3, Term::Comp(_, _) => 4 }
}

pub open spec fn bytes_lt(a: Seq<u8>, b: Seq<u8>) -> bool
    decreases a.len(),
{
    if b.len() == 0 { false }
    else if a.len() == 0 { true }
    else if a[0] != b[0] { a[0] < b[0] }
    else { bytes_lt(a.drop_first(), b.drop_first()) }
}

pub open spec fn term_lt(a: Term, b: Term) -> bool
    decreases a,
{
    if rank(a) != rank(b) { rank(a) < rank(b) } else {
        match (a, b) {
            (Term::Var(x), Term::Var(y)) => x < y,
            (Term::Int(x), Term::Int(y)) => x < y,
            (Term::Atom(x), Term::Atom(y)) => bytes_lt(x, y),
            (Term::Comp(n, xs), Term::Comp(m, ys)) =>
                if xs.len() != ys.len() { xs.len() < ys.len() }
                else if n != m { bytes_lt(n, m) }
                else { args_lt(xs, ys) },
            _ => false,
        }
    }
}

pub open spec fn args_lt(xs: Seq<Term>, ys: Seq<Term>) -> bool
    decreases xs,
{
    if xs.len() == 0 { false }
    else if xs[0] != ys[0] { term_lt(xs[0], ys[0]) }
    else { args_lt(xs.drop_first(), ys.drop_first()) }
}

pub open spec fn insert_sorted(x: Term, s: Seq<Term>) -> Seq<Term>
    decreases s.len(),
{
    if s.len() > 0 && term_lt(s[0], x) { seq![s[0]] + insert_sorted(x, s.drop_first()) } else { seq![x] + s }
}

// msort/2: stable, duplicates kept.
pub open spec fn msort(s: Seq<Term>) -> Seq<Term>
    decreases s.len(),
{
    if s.len() == 0 { Seq::empty() } else { insert_sorted(s[0], msort(s.drop_first())) }
}

pub open spec fn dedup(s: Seq<Term>) -> Seq<Term>
    decreases s.len(),
{
    if s.len() < 2 { s } else if s[0] == s[1] { dedup(s.drop_first()) } else { seq![s[0]] + dedup(s.drop_first()) }
}

// sort/2: standard order, duplicates removed.
pub open spec fn sort_unique(s: Seq<Term>) -> Seq<Term> { dedup(msort(s)) }

} // verus!
