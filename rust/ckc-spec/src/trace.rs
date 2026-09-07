use crate::answers::*;
use crate::engine::*;
use crate::replay::*;
use crate::term::*;
use crate::v1text::*;
use vstd::prelude::*;

verus! {

// Trusted spec: trace mode + trace-check (contract m5u2b, K3). Trace mode =
// the legacy `ace_to_pl.pl trace_mode` law: one directed first proof per
// positive claim of a committed answers artifact under the R8 trace bounds,
// materialized into clause nodes naming the resolving clause by sentence
// identity + the sha256 of its committed line. Trace-check = the
// committed-state law (REFERENCE § Proof traces): a committed trace is
// accepted iff it equals this derivation, every node's digest joins exactly
// one committed clause line of the named sentence, and every claim is proved.
// Digests arrive from the shell (R3): `digests[i]` = lowercase-hex sha256 of
// `clause_line(db[i])`, the bytes `trace_lines` hands it; the kernel never hashes.
pub open spec fn trace_run_inf() -> nat {
    1000000
}

pub open spec fn digest_at(digests: Seq<Seq<u8>>, m: nat) -> Seq<u8> {
    if m < digests.len() {
        digests[m as int]
    } else {
        Seq::empty()
    }
}

// Sentence coordinates of the loaded program, parallel to `db_of`: records
// sit at ordinal 0, every bundle clause at its bundle's ordinal.
pub ghost struct Coord {
    pub docid: Seq<u8>,
    pub s: nat,
}

pub open spec fn bundle_coords(docid: Seq<u8>, b: Bundle) -> Seq<Coord> {
    Seq::new(b.clauses.len(), |i: int| Coord { docid, s: b.s })
}

pub open spec fn doc_coords(d: DocFile) -> Seq<Coord> {
    seq![Coord { docid: d.docid, s: 0 }, Coord { docid: d.docid, s: 0 }] + d.bundles.map_values(
        |b: Bundle| bundle_coords(d.docid, b),
    ).flatten()
}

pub open spec fn coords_of(docs: Seq<DocFile>) -> Seq<Coord> {
    docs.map_values(|d: DocFile| doc_coords(d)).flatten()
}

// --- answers custody (legacy trace_read_answers order over the canonical class) ---
pub open spec fn answers_reject(why: Term) -> Out {
    check_load(Term::Comp(ascii("answers_file"@), seq![why]))
}

pub open spec fn yesno_shape(r: Term) -> bool {
    r == atom("yes"@) || r == Term::Comp(ascii("no"@), seq![atom("finite_failure"@)]) || r
        == limit()
}

pub open spec fn is_solutions(r: Term) -> bool {
    match r {
        Term::Comp(name, args) => name == ascii("solutions"@) && args.len() == 1,
        _ => false,
    }
}

pub open spec fn sol_reject(why: Seq<char>, i: nat) -> Term {
    Term::Comp(ascii(why), seq![Term::Int(i as int)])
}

// Row i (1-based): sol/1 wrapper → proper-list values → arity of the manifest.
pub open spec fn sols_check(sols: Seq<Term>, i: nat, arity: nat) -> Option<Term>
    decreases sols.len(),
{
    if sols.len() == 0 {
        Option::None
    } else {
        match sols[0] {
            Term::Comp(name, args) => if name == ascii("sol"@) && args.len() == 1 {
                match list_items(args[0]) {
                    Option::None => Option::Some(sol_reject("solution_values"@, i)),
                    Option::Some(vs) => if vs.len() != arity {
                        Option::Some(
                            Term::Comp(
                                ascii("solution_arity"@),
                                seq![
                                    Term::Int(i as int),
                                    Term::Int(arity as int),
                                    Term::Int(vs.len() as int),
                                ],
                            ),
                        )
                    } else {
                        sols_check(sols.drop_first(), i + 1, arity)
                    },
                }
            } else {
                Option::Some(sol_reject("solution_shape"@, i))
            },
            _ => Option::Some(sol_reject("solution_shape"@, i)),
        }
    }
}

// Closed result algebra, then fit against the query mode (wh = nonempty
// answers manifest), then the solution rows.
pub open spec fn result_check(r: Term, arity: nat) -> Option<Term> {
    if !(yesno_shape(r) || is_solutions(r)) {
        Option::Some(atom("result_shape"@))
    } else if arity == 0 {
        if yesno_shape(r) {
            Option::None
        } else {
            Option::Some(atom("result_mode_mismatch"@))
        }
    } else if !(is_solutions(r) || r == limit()) {
        Option::Some(atom("result_mode_mismatch"@))
    } else if is_solutions(r) {
        match list_items(arg(r, 0)) {
            Option::None => Option::Some(atom("solutions_list"@)),
            Option::Some(sols) => sols_check(sols, 1, arity),
        }
    } else {
        Option::None
    }
}

pub open spec fn answers_custody(answers: Src, qsha: Seq<u8>, q: QueryFile, arity: nat) -> Result<
    AnswersFile,
    Out,
> {
    match answers {
        Src::Missing => Result::Err(check_load(atom("unreadable"@))),
        Src::Bad(off) => Result::Err(utf8_reject(off)),
        Src::Bytes(b) => if !accepts(b) {
            Result::Err(answers_reject(atom("noncanonical"@)))
        } else {
            match the_v1(b) {
                V1File::Answers(a) => if a.qid != q.qid {
                    Result::Err(answers_reject(atom("qid_mismatch"@)))
                } else if a.qsha != qsha {
                    Result::Err(answers_reject(atom("query_sha256_mismatch"@)))
                } else {
                    match result_check(a.result, arity) {
                        Option::Some(w) => Result::Err(answers_reject(w)),
                        Option::None => Result::Ok(a),
                    }
                },
                V1File::Traces(_) => Result::Err(answers_reject(atom("record_shape"@))),
                f => Result::Err(
                    answers_reject(
                        Term::Comp(ascii("term_count"@), seq![Term::Int(term_count(f) as int)]),
                    ),
                ),
            }
        },
    }
}

// --- the trace machine: the engine's search with proof recording, a fresh
// site-local depth for every negation, and the legacy NAF site protocol ---
pub ghost enum TGoal {
    Lit(Term, nat, Seq<nat>),  // goal, remaining depth, proof-forest position
    NafCut(nat),
}

pub ghost enum TEv {
    Clause(nat),  // the resolving clause = db index
    Naf(Term),  // the negation succeeded: its inner goal as it stood at the site
}

pub ghost enum TAlt {
    Cl { stack: Seq<TGoal>, fresh: nat, ci: nat, log: Seq<(Seq<nat>, TEv)> },
    Naf {
        stack: Seq<TGoal>,
        fresh: nat,
        log: Seq<(Seq<nat>, TEv)>,
        path: Seq<nat>,
        inner: Term,
        pruned: bool,  // the row's prune flag at site entry
    },
}

pub ghost struct TCfg {
    pub stack: Seq<TGoal>,
    pub alts: Seq<TAlt>,
    pub fresh: nat,
    pub ci: nat,
    pub log: Seq<(Seq<nat>, TEv)>,
    pub pruned: bool,
}

pub ghost enum TStep {
    Next(TCfg),
    Sol(Seq<(Seq<nat>, TEv)>),
    Done(bool),  // complete = exhausted without a prune
    Limit,
}

pub ghost enum TUni {
    Ok(Seq<TGoal>),
    Fail,
    Out,
}

pub open spec fn tgoal_term(g: TGoal) -> Term {
    match g {
        TGoal::Lit(t, _, _) => t,
        TGoal::NafCut(_) => Term::Nil,
    }
}

pub open spec fn tgoal_with(g: TGoal, t: Term) -> TGoal {
    match g {
        TGoal::Lit(_, d, p) => TGoal::Lit(t, d, p),
        TGoal::NafCut(l) => TGoal::NafCut(l),
    }
}

// Inside a negation's inner search: nothing is recorded (legacy discards
// the inner skeleton) and a depth cut binds the row.
pub open spec fn in_naf(stack: Seq<TGoal>) -> bool {
    exists|i: int| 0 <= i < stack.len() && #[trigger] stack[i] is NafCut
}

pub open spec fn tbody_goals(items: Seq<BodyItem>, off: nat, d: nat, path: Seq<nat>) -> Seq<TGoal> {
    Seq::new(items.len(), |i: int| TGoal::Lit(item_term(items[i], off), d, path.push(i as nat)))
}

// Unification through the engine's law: the stack's terms ride as the
// template, so every binding lands in them.
pub open spec fn tunify(pairs: Seq<(Term, Term)>, stack: Seq<TGoal>) -> TUni {
    match unify(
        UState { pairs, stack: Seq::empty(), sol: stack.map_values(|g: TGoal| tgoal_term(g)) },
    ) {
        UOut::Ok(_, terms) => TUni::Ok(
            Seq::new(stack.len(), |i: int| tgoal_with(stack[i], terms[i])),
        ),
        UOut::Fail => TUni::Fail,
        UOut::Out => TUni::Out,
    }
}

// Backtrack: resume the newest choicepoint, or end the search. A negation
// frame resumes when its inner search failed: finitely = a sound leaf (the
// row's prune flag returns to its site-entry value); after a depth cut
// inside the site = the row's limit.
pub open spec fn tfail(c: TCfg) -> TStep {
    if c.alts.len() == 0 {
        TStep::Done(!c.pruned)
    } else {
        let rest = c.alts.drop_last();
        match c.alts.last() {
            TAlt::Cl { stack, fresh, ci, log } => TStep::Next(
                TCfg { stack, fresh, ci, log, alts: rest, pruned: c.pruned },
            ),
            TAlt::Naf { stack, fresh, log, path, inner, pruned } => if c.pruned {
                TStep::Limit
            } else {
                TStep::Next(
                    TCfg {
                        stack,
                        fresh,
                        ci: 0,
                        log: if in_naf(stack) {
                            log
                        } else {
                            log.push((path, TEv::Naf(inner)))
                        },
                        alts: rest,
                        pruned,
                    },
                )
            },
        }
    }
}

pub open spec fn tcall(
    db: Seq<DocClause>,
    c: TCfg,
    name: Seq<u8>,
    args: Seq<Term>,
    d: nat,
    rest: Seq<TGoal>,
    ci: nat,
    path: Seq<nat>,
) -> TStep
    decreases db.len() - ci,
{
    match next_match(db, name, args.len(), ci) {
        Option::None => tfail(c),
        Option::Some(m) => {
            let cl = db[m as int];
            proof {
                next_match_bound(db, name, args.len(), ci);
            }
            let stack2 = tbody_goals(cl.body, c.fresh, (d - 1) as nat, path) + rest;
            match tunify(zip(args, args_of(shift(cl.head, c.fresh))), stack2) {
                TUni::Ok(stack) => TStep::Next(
                    TCfg {
                        alts: c.alts.push(
                            TAlt::Cl { stack: c.stack, fresh: c.fresh, ci: m + 1, log: c.log },
                        ),
                        stack,
                        fresh: c.fresh + clause_nvars(cl),
                        ci: 0,
                        log: if in_naf(c.stack) {
                            c.log
                        } else {
                            c.log.push((path, TEv::Clause(m)))
                        },
                        pruned: c.pruned,
                    },
                ),
                TUni::Fail => tcall(db, c, name, args, d, rest, m + 1, path),
                TUni::Out => TStep::Limit,
            }
        },
    }
}

pub open spec fn tstep(db: Seq<DocClause>, c: TCfg) -> TStep {
    if c.stack.len() == 0 {
        TStep::Sol(c.log)
    } else {
        let rest = c.stack.drop_first();
        match c.stack[0] {
            // the negation's inner proof succeeded: the site fails; prunes inside the site are absorbed
            TGoal::NafCut(lvl) => match c.alts[lvl as int] {
                TAlt::Naf { pruned, .. } => tfail(
                    TCfg { alts: c.alts.take(lvl as int), pruned, ..c },
                ),
                _ => TStep::Limit,  // unreachable: the level holds the site's own frame
            },
            TGoal::Lit(g, d, path) => if d == 0 {
                tfail(TCfg { pruned: true, ..c })
            } else {
                match g {
                    Term::Comp(name, args) => if name == comma_name() && args.len() == 2 {
                        TStep::Next(
                            TCfg {
                                stack: seq![
                                    TGoal::Lit(args[0], d, path),
                                    TGoal::Lit(args[1], d, path),
                                ] + rest,
                                ..c
                            },
                        )
                    } else if name == naf_name() && args.len() == 1 {
                        TStep::Next(
                            TCfg {
                                alts: c.alts.push(
                                    TAlt::Naf {
                                        stack: rest,
                                        fresh: c.fresh,
                                        log: c.log,
                                        path,
                                        inner: args[0],
                                        pruned: c.pruned,
                                    },
                                ),
                                stack: seq![
                                    TGoal::Lit(args[0], trace_depth(), path),
                                    TGoal::NafCut(c.alts.len()),
                                ] + rest,
                                pruned: false,
                                ..c
                            },
                        )
                    } else {
                        tcall(db, c, name, args, d, rest, c.ci, path)
                    },
                    // variable/atomic goals: grammar-unrepresentable after custody (R14)
                    _ => tfail(c),
                }
            },
        }
    }
}

pub ghost enum TOut {
    Proved(Seq<(Seq<nat>, TEv)>),
    Failed(bool),
    Limit,
}

// The outcome and the fuel left; every transition charges one (R15).
pub open spec fn trun(db: Seq<DocClause>, c: TCfg, fuel: nat) -> (TOut, nat)
    decreases fuel,
{
    if fuel == 0 {
        (TOut::Limit, 0)
    } else {
        match tstep(db, c) {
            TStep::Next(c2) => trun(db, c2, (fuel - 1) as nat),
            TStep::Sol(log) => (TOut::Proved(log), (fuel - 1) as nat),
            TStep::Done(complete) => (TOut::Failed(complete), (fuel - 1) as nat),
            TStep::Limit => (TOut::Limit, (fuel - 1) as nat),
        }
    }
}

// --- the proof forest ---
pub ghost enum PNode {
    Clause(nat, Seq<PNode>),
    Naf(Term),
}

pub open spec fn ev_at(log: Seq<(Seq<nat>, TEv)>, path: Seq<nat>) -> Option<TEv>
    decreases log.len(),
{
    if log.len() == 0 {
        Option::None
    } else if log[0].0 == path {
        Option::Some(log[0].1)
    } else {
        ev_at(log.drop_first(), path)
    }
}

// A clause event at `path` owns one child position per body item. Every
// node of a derivation has one event per ancestor and itself, so a path
// longer than the log names no node; the guards are formal residue.
pub open spec fn build(db: Seq<DocClause>, log: Seq<(Seq<nat>, TEv)>, path: Seq<nat>) -> PNode
    decreases log.len() + 1 - path.len(), 0int,
{
    if path.len() > log.len() {
        PNode::Naf(Term::Nil)
    } else {
        match ev_at(log, path) {
            Option::Some(TEv::Clause(m)) => PNode::Clause(
                m,
                build_all(db, log, path, 0, db[m as int].body.len()),
            ),
            Option::Some(TEv::Naf(t)) => PNode::Naf(t),
            Option::None => PNode::Naf(Term::Nil),
        }
    }
}

pub open spec fn build_all(
    db: Seq<DocClause>,
    log: Seq<(Seq<nat>, TEv)>,
    path: Seq<nat>,
    i: nat,
    n: nat,
) -> Seq<PNode>
    decreases log.len() - path.len(), n - i,
{
    if i >= n || path.len() > log.len() {
        Seq::empty()
    } else {
        seq![build(db, log, path.push(i))] + build_all(db, log, path, i + 1, n)
    }
}

// --- materialization: sentence identity, digest, frozen payloads ---
pub open spec fn id_role(r: Term) -> bool {
    r == atom("context"@) || r == atom("product"@) || r == atom("witness"@)
}

pub open spec fn pos_int(t: Term) -> bool {
    match t {
        Term::Int(s) => s > 0,
        _ => false,
    }
}

// '$guideline_id'(Role, D, S, _, _) subterms with a schema role, an atom D and a positive S.
pub open spec fn id_pairs(t: Term) -> Seq<Term>
    decreases t,
{
    match t {
        Term::Comp(name, args) => (if name == gid_name() && args.len() == 5 && id_role(args[0])
            && args[1] is Atom && pos_int(args[2]) {
            seq![pair(args[1], args[2])]
        } else {
            Seq::empty()
        }) + id_pairs_all(args),
        _ => Seq::empty(),
    }
}

pub open spec fn id_pairs_all(ts: Seq<Term>) -> Seq<Term>
    decreases ts,
{
    if ts.len() == 0 {
        Seq::empty()
    } else {
        id_pairs(ts[0]) + id_pairs_all(ts.drop_first())
    }
}

pub open spec fn item_ids(it: BodyItem) -> Seq<Term> {
    match it {
        BodyItem::Pos(l) => id_pairs(l),
        BodyItem::Naf(gs) => id_pairs_all(gs),
    }
}

// Exactly one distinct pair names the clause; anything else rejects fail-closed.
pub open spec fn identity(c: DocClause) -> Result<Term, Out> {
    let ids = sort_unique(
        id_pairs(c.head) + c.body.map_values(|it: BodyItem| item_ids(it)).flatten(),
    );
    if ids.len() == 1 {
        Result::Ok(Term::Comp(ascii("sentence"@), args_of(ids[0])))
    } else if ids.len() == 0 {
        Result::Err(proof_fail(Term::Comp(ascii("clause_identity"@), seq![atom("none"@)])))
    } else {
        Result::Err(
            proof_fail(
                Term::Comp(
                    ascii("clause_identity"@),
                    seq![Term::Comp(ascii("multiple"@), seq![Term::Int(ids.len() as int)])],
                ),
            ),
        )
    }
}

// Payload variables print as '$VAR'(N): one numbervars pass over the whole
// artifact, so each frozen payload continues the run's counter.
pub open spec fn pos_of(fs: Seq<nat>, k: nat) -> nat
    decreases fs.len(),
{
    if fs.len() == 0 || fs[0] == k {
        0
    } else {
        1 + pos_of(fs.drop_first(), k)
    }
}

pub open spec fn dollar_var(n: nat) -> Term {
    Term::Comp(dollar_var_name(), seq![Term::Int(n as int)])
}

pub open spec fn number_with(t: Term, fs: Seq<nat>, base: nat) -> Term
    decreases t,
{
    match t {
        Term::Var(k) => dollar_var(base + pos_of(fs, k)),
        Term::Comp(name, args) => Term::Comp(name, number_all(args, fs, base)),
        _ => t,
    }
}

pub open spec fn number_all(ts: Seq<Term>, fs: Seq<nat>, base: nat) -> Seq<Term>
    decreases ts,
{
    if ts.len() == 0 {
        Seq::empty()
    } else {
        seq![number_with(ts[0], fs, base)] + number_all(ts.drop_first(), fs, base)
    }
}

pub open spec fn number(t: Term, base: nat) -> (Term, nat) {
    let fs = firsts(var_stream(t), Set::empty());
    (number_with(t, fs, base), base + fs.len())
}

pub ghost enum Mat {
    Ok(Seq<Term>, nat, nat),  // node terms, next numbering base, clause-node count
    Err(Out),
}

// Node, then its children, then its siblings (the legacy materialization order).
pub open spec fn mat(db: Seq<DocClause>, digests: Seq<Seq<u8>>, node: PNode, base: nat) -> Mat
    decreases node, 0int,
{
    match node {
        PNode::Naf(t) => {
            let n = number(t, base);
            Mat::Ok(seq![Term::Comp(ascii("naf"@), seq![n.0])], n.1, 0)
        },
        PNode::Clause(m, kids) => match identity(db[m as int]) {
            Result::Err(o) => Mat::Err(o),
            Result::Ok(sentence) => match mat_all(db, digests, kids, base) {
                Mat::Err(o) => Mat::Err(o),
                Mat::Ok(children, b, n) => Mat::Ok(
                    seq![
                        Term::Comp(
                            ascii("clause"@),
                            seq![
                                sentence,
                                Term::Comp(
                                    ascii("clause_sha256"@),
                                    seq![Term::Atom(digest_at(digests, m))],
                                ),
                                list_term(children),
                            ],
                        ),
                    ],
                    b,
                    n + 1,
                ),
            },
        },
    }
}

pub open spec fn mat_all(
    db: Seq<DocClause>,
    digests: Seq<Seq<u8>>,
    kids: Seq<PNode>,
    base: nat,
) -> Mat
    decreases kids, 1int,
{
    if kids.len() == 0 {
        Mat::Ok(Seq::empty(), base, 0)
    } else {
        match mat(db, digests, kids[0], base) {
            Mat::Err(o) => Mat::Err(o),
            Mat::Ok(first, b1, n1) => match mat_all(db, digests, kids.drop_first(), b1) {
                Mat::Err(o) => Mat::Err(o),
                Mat::Ok(rest, b2, n2) => Mat::Ok(first + rest, b2, n1 + n2),
            },
        }
    }
}

// --- proof-tree validity: the relation the soundness theorem (contract
// k3_sound) binds every derived forest to ---
// A substitution = bindings applied in order.
pub open spec fn apply(t: Term, s: Seq<(nat, Term)>) -> Term
    decreases s.len(),
{
    if s.len() == 0 {
        t
    } else {
        apply(subst(t, s[0].0, s[0].1), s.drop_first())
    }
}

pub open spec fn body_terms(items: Seq<BodyItem>, off: nat) -> Seq<Term> {
    items.map_values(|it: BodyItem| item_term(it, off))
}

// A frozen negation payload certifies bounded finite failure under the trace bounds.
pub open spec fn naf_fails(db: Seq<DocClause>, t: Term) -> bool {
    trun(db, roots_cfg(seq![t]), trace_inf()).0 == TOut::Failed(true)
}

// A clause node resolves its goal: some renaming + substitution equates the
// clause head with the goal and instantiates the body into the children's
// goals. A naf leaf's payload generalizes the site goal and fails finitely.
pub open spec fn node_valid(db: Seq<DocClause>, g: Term, node: PNode) -> bool
    decreases node, 0int,
{
    match node {
        PNode::Naf(t) => match g {
            Term::Comp(name, args) => name == naf_name() && args.len() == 1 && (exists|
                s: Seq<(nat, Term)>,
            | #[trigger]
                apply(t, s) == args[0]) && naf_fails(db, t),
            _ => false,
        },
        PNode::Clause(m, kids) => m < db.len() && exists|k: nat, s: Seq<(nat, Term)>| #[trigger]
            resolves(db, g, m, k, s, kids),
    }
}

// One resolution step: goal and renamed head coincide under `s`; the renamed
// body under `s` = the children's goals.
pub open spec fn resolves(
    db: Seq<DocClause>,
    g: Term,
    m: nat,
    k: nat,
    s: Seq<(nat, Term)>,
    kids: Seq<PNode>,
) -> bool
    decreases kids, 2int,
{
    m < db.len() && apply(g, s) == apply(shift(db[m as int].head, k), s) && kids_valid(
        db,
        body_terms(db[m as int].body, k).map_values(|b: Term| apply(b, s)),
        kids,
    )
}

pub open spec fn kids_valid(db: Seq<DocClause>, gs: Seq<Term>, kids: Seq<PNode>) -> bool
    decreases kids, 1int,
{
    kids.len() == gs.len() && (kids.len() == 0 || (node_valid(db, gs[0], kids[0]) && kids_valid(
        db,
        gs.drop_first(),
        kids.drop_first(),
    )))
}

pub open spec fn forest_valid(db: Seq<DocClause>, goal: Term, forest: Seq<PNode>) -> bool {
    kids_valid(db, conj_leaves(goal), forest)
}

// --- rows ---
pub open spec fn conj_leaves(t: Term) -> Seq<Term>
    decreases t,
{
    match t {
        Term::Comp(name, args) => if name == comma_name() && args.len() == 2 {
            conj_leaves(args[0]) + conj_leaves(args[1])
        } else {
            seq![t]
        },
        _ => seq![t],
    }
}

pub open spec fn roots_cfg(goals: Seq<Term>) -> TCfg {
    TCfg {
        stack: Seq::new(goals.len(), |i: int| TGoal::Lit(goals[i], trace_depth(), seq![i as nat])),
        alts: Seq::empty(),
        fresh: nvars_all(goals),
        ci: 0,
        log: Seq::empty(),
        pruned: false,
    }
}

pub open spec fn unproved(why: Seq<char>) -> Term {
    Term::Comp(ascii("unproved"@), seq![atom(why)])
}

pub ghost enum RowOut {
    Row(Term, nat, nat),  // proof payload, charge against the run, next numbering base
    Err(Out),
}

// The row's proof forest: one root per top-level goal, from the search log.
pub open spec fn forest_of(db: Seq<DocClause>, goals: Seq<Term>, log: Seq<(Seq<nat>, TEv)>) -> Seq<
    PNode,
> {
    Seq::new(goals.len(), |i: int| build(db, log, seq![i as nat]))
}

pub open spec fn derived_forest(db: Seq<DocClause>, goal: Term) -> Option<Seq<PNode>> {
    match trun(db, roots_cfg(conj_leaves(goal)), trace_inf()).0 {
        TOut::Proved(log) => Option::Some(forest_of(db, conj_leaves(goal), log)),
        _ => Option::None,
    }
}

// One row: the search runs under the row bounds; a proof materializes into
// nodes, charged to the run (one per clause node) but never to the row.
pub open spec fn prove_row(
    db: Seq<DocClause>,
    digests: Seq<Seq<u8>>,
    goal: Term,
    base: nat,
) -> RowOut {
    let goals = conj_leaves(goal);
    let r = trun(db, roots_cfg(goals), trace_inf());
    let consumed = (trace_inf() - r.1) as nat;
    match r.0 {
        TOut::Failed(true) => RowOut::Row(unproved("finite_failure"@), consumed, base),
        TOut::Failed(false) => RowOut::Row(unproved("limit"@), consumed, base),
        TOut::Limit => RowOut::Row(unproved("limit"@), consumed, base),
        TOut::Proved(log) => match mat_all(db, digests, forest_of(db, goals, log), base) {
            Mat::Err(o) => RowOut::Err(o),
            Mat::Ok(nodes, b, n) => RowOut::Row(
                Term::Comp(ascii("proved"@), seq![list_term(nodes)]),
                consumed + n,
                b,
            ),
        },
    }
}

// Every row proves a fresh copy of the goal with its answer variables bound
// to the committed values (no cross-row leakage).
pub open spec fn bind(goal: Term, vars: Seq<Term>, values: Seq<Term>, i: nat) -> Term
    decreases vars.len() - i,
{
    if i >= vars.len() || i >= values.len() {
        goal
    } else {
        match vars[i as int] {
            Term::Var(x) => bind(subst(goal, x, values[i as int]), vars, values, i + 1),
            _ => bind(goal, vars, values, i + 1),
        }
    }
}

pub ghost enum Rows {
    Ok(Seq<Term>),
    Trip,  // the whole-run bound
    Err(Out),
}

pub open spec fn trace_rows(
    db: Seq<DocClause>,
    digests: Seq<Seq<u8>>,
    goal: Term,
    vars: Seq<Term>,
    sols: Seq<Term>,
    i: nat,
    left: nat,
    base: nat,
    acc: Seq<Term>,
) -> Rows
    decreases sols.len() - i,
{
    if i >= sols.len() {
        Rows::Ok(acc)
    } else {
        let values = arg(sols[i as int], 0);
        match list_items(values) {
            Option::None => Rows::Ok(acc),  // unreachable: custody checked every row
            Option::Some(vs) => match prove_row(db, digests, bind(goal, vars, vs, 0), base) {
                RowOut::Err(o) => Rows::Err(o),
                RowOut::Row(p, consumed, b) => if consumed > left {
                    Rows::Trip
                } else {
                    trace_rows(
                        db,
                        digests,
                        goal,
                        vars,
                        sols,
                        i + 1,
                        (left - consumed) as nat,
                        b,
                        acc.push(Term::Comp(ascii("sol"@), seq![values, p])),
                    )
                },
            },
        }
    }
}

// Mirror law: yes gains its proof; each sol(Values) row gains one; no and
// indeterminate mirror verbatim; a whole-run trip replaces the mirror.
pub open spec fn trace_result(
    db: Seq<DocClause>,
    digests: Seq<Seq<u8>>,
    q: QueryFile,
    arows: Seq<Term>,
    a: AnswersFile,
) -> Result<Term, Out> {
    if arows.len() == 0 {
        if a.result == atom("yes"@) {
            match prove_row(db, digests, q.goal, 0) {
                RowOut::Err(o) => Result::Err(o),
                RowOut::Row(p, consumed, _) => Result::Ok(
                    if consumed > trace_run_inf() {
                        limit()
                    } else {
                        Term::Comp(ascii("yes"@), seq![p])
                    },
                ),
            }
        } else {
            Result::Ok(a.result)
        }
    } else if is_solutions(a.result) {
        match list_items(arg(a.result, 0)) {
            Option::None => Result::Ok(a.result),  // unreachable after custody
            Option::Some(sols) => match trace_rows(
                db,
                digests,
                q.goal,
                arows.map_values(|r: Term| arg(r, 0)),
                sols,
                0,
                trace_run_inf(),
                0,
                Seq::empty(),
            ) {
                Rows::Err(o) => Result::Err(o),
                Rows::Trip => Result::Ok(limit()),
                Rows::Ok(rows) => Result::Ok(
                    Term::Comp(ascii("solutions"@), seq![list_term(rows)]),
                ),
            },
        }
    } else {
        Result::Ok(a.result)
    }
}

// --- pipeline: manifest → query custody → answers custody → composition ---
pub ghost struct Front {
    pub q: QueryFile,
    pub arows: Seq<Term>,
    pub a: AnswersFile,
    pub docs: Seq<DocFile>,
}

pub open spec fn front(
    mpath: Seq<u8>,
    manifest: Src,
    pls: Seq<Src>,
    pys: Seq<Src>,
    query: Src,
    qsha: Seq<u8>,
    answers: Src,
) -> Result<Front, Out> {
    match manifest_stage(mpath, manifest, pls, pys) {
        Result::Err(o) => Result::Err(o),
        Result::Ok(rows) => match custody(query) {
            Result::Err(o) => Result::Err(o),
            Result::Ok((q, arows)) => match answers_custody(answers, qsha, q, arows.len()) {
                Result::Err(o) => Result::Err(o),
                Result::Ok(a) => match composition(rows, pls) {
                    Result::Err(o) => Result::Err(o),
                    Result::Ok(docs) => Result::Ok(Front { q, arows, a, docs }),
                },
            },
        },
    }
}

// Lines to hash, in the pipeline's own error order: one canonical line per
// loaded clause, parallel to `db_of`; the shell hashes each (R3) and hands the
// digests back.
pub open spec fn trace_lines(
    mpath: Seq<u8>,
    manifest: Src,
    pls: Seq<Src>,
    pys: Seq<Src>,
    query: Src,
    qsha: Seq<u8>,
    answers: Src,
) -> Result<Seq<Seq<u8>>, Out> {
    match front(mpath, manifest, pls, pys, query, qsha, answers) {
        Result::Err(o) => Result::Err(o),
        Result::Ok(f) => Result::Ok(db_of(f.docs).map_values(|c: DocClause| clause_line(c))),
    }
}

// asha = lowercase hex sha256 of the raw answers bytes, computed by the shell (digest law).
pub open spec fn trace_output(
    mpath: Seq<u8>,
    manifest: Src,
    pls: Seq<Src>,
    pys: Seq<Src>,
    query: Src,
    qsha: Seq<u8>,
    answers: Src,
    asha: Seq<u8>,
    digests: Seq<Seq<u8>>,
) -> Out {
    match front(mpath, manifest, pls, pys, query, qsha, answers) {
        Result::Err(o) => o,
        Result::Ok(f) => match trace_result(db_of(f.docs), digests, f.q, f.arows, f.a) {
            Result::Err(o) => o,
            Result::Ok(result) => ok(print_traces(TracesFile { qid: f.q.qid, qsha, asha, result })),
        },
    }
}

// --- trace-check: the committed-state relation ---
pub open spec fn trace_reject(why: Term) -> Out {
    check_load(Term::Comp(ascii("trace_file"@), seq![why]))
}

pub open spec fn tc_fail(why: Term) -> Out {
    proof_fail(Term::Comp(ascii("trace_check"@), seq![why]))
}

pub open spec fn trace_custody(trace: Src, q: QueryFile, qsha: Seq<u8>, asha: Seq<u8>) -> Result<
    TracesFile,
    Out,
> {
    match trace {
        Src::Missing => Result::Err(check_load(atom("unreadable"@))),
        Src::Bad(off) => Result::Err(utf8_reject(off)),
        Src::Bytes(b) => if !accepts(b) {
            Result::Err(trace_reject(atom("noncanonical"@)))
        } else {
            match the_v1(b) {
                V1File::Traces(t) => if t.qid != q.qid {
                    Result::Err(trace_reject(atom("qid_mismatch"@)))
                } else if t.qsha != qsha {
                    Result::Err(trace_reject(atom("query_sha256_mismatch"@)))
                } else if t.asha != asha {
                    Result::Err(trace_reject(atom("answers_sha256_mismatch"@)))
                } else {
                    Result::Ok(t)
                },
                _ => Result::Err(trace_reject(atom("record_shape"@))),
            }
        },
    }
}

// Join law: a node's (D, S, Hex) resolves to exactly one committed clause
// line of sentence S in document D. Node grammar = the legacy walker's:
// docid under the name law, 1 <= S < 10^9, lowercase hex64.
pub open spec fn coord_of(t: Term) -> Option<Coord> {
    match t {
        Term::Comp(name, args) => if name == ascii("sentence"@) && args.len() == 2 {
            match (args[0], args[1]) {
                (Term::Atom(d), Term::Int(s)) => if name_ok(d) && 1 <= s && s < 1000000000 {
                    Option::Some(Coord { docid: d, s: s as nat })
                } else {
                    Option::None
                },
                _ => Option::None,
            }
        } else {
            Option::None
        },
        _ => Option::None,
    }
}

pub open spec fn hex_of(t: Term) -> Option<Seq<u8>> {
    match t {
        Term::Comp(name, args) => if name == ascii("clause_sha256"@) && args.len() == 1 {
            match args[0] {
                Term::Atom(h) => if hex64(h) {
                    Option::Some(h)
                } else {
                    Option::None
                },
                _ => Option::None,
            }
        } else {
            Option::None
        },
        _ => Option::None,
    }
}

pub open spec fn join_count(
    coords: Seq<Coord>,
    digests: Seq<Seq<u8>>,
    c: Coord,
    h: Seq<u8>,
    i: nat,
) -> nat
    decreases coords.len() - i,
{
    if i >= coords.len() {
        0
    } else {
        (if coords[i as int] == c && digest_at(digests, i) == h {
            1nat
        } else {
            0nat
        }) + join_count(coords, digests, c, h, i + 1)
    }
}

pub ghost enum Join {
    Ok(nat),  // clause nodes counted
    Bad(Term),
}

pub open spec fn is_clause_node(node: Term) -> bool {
    match node {
        Term::Comp(name, args) => name == ascii("clause"@) && args.len() == 3,
        _ => false,
    }
}

pub open spec fn node_join(coords: Seq<Coord>, digests: Seq<Seq<u8>>, node: Term) -> Join
    decreases node, 0int,
{
    match node {
        Term::Comp(name, args) => if name == ascii("clause"@) && args.len() == 3 {
            match (coord_of(args[0]), hex_of(args[1])) {
                (Option::Some(c), Option::Some(h)) => {
                    let n = join_count(coords, digests, c, h, 0);
                    if n != 1 {
                        Join::Bad(Term::Comp(ascii("join"@), seq![args[0], Term::Int(n as int)]))
                    } else {
                        match list_join(coords, digests, args[2]) {
                            Join::Ok(k) => Join::Ok(k + 1),
                            bad => bad,
                        }
                    }
                },
                _ => Join::Bad(atom("node_shape"@)),
            }
        } else if name == ascii("naf"@) && args.len() == 1 {
            Join::Ok(0)
        } else {
            Join::Bad(atom("node_shape"@))
        },
        _ => Join::Bad(atom("node_shape"@)),
    }
}

// Children = a proper list of nodes.
pub open spec fn list_join(coords: Seq<Coord>, digests: Seq<Seq<u8>>, t: Term) -> Join
    decreases t, 1int,
{
    match t {
        Term::Nil => Join::Ok(0),
        Term::Comp(name, args) => if name == cons_name() && args.len() == 2 {
            match node_join(coords, digests, args[0]) {
                Join::Ok(a) => match list_join(coords, digests, args[1]) {
                    Join::Ok(b) => Join::Ok(a + b),
                    bad => bad,
                },
                bad => bad,
            }
        } else {
            Join::Bad(atom("node_shape"@))
        },
        _ => Join::Bad(atom("node_shape"@)),
    }
}

// Roots are clause nodes; naf leaves live only among a clause node's children.
pub open spec fn roots_join(coords: Seq<Coord>, digests: Seq<Seq<u8>>, nodes: Seq<Term>) -> Join
    decreases nodes.len(),
{
    if nodes.len() == 0 {
        Join::Ok(0)
    } else if !is_clause_node(nodes[0]) {
        Join::Bad(atom("node_shape"@))
    } else {
        match node_join(coords, digests, nodes[0]) {
            Join::Ok(a) => match roots_join(coords, digests, nodes.drop_first()) {
                Join::Ok(b) => Join::Ok(a + b),
                bad => bad,
            },
            bad => bad,
        }
    }
}

// proved(Nodes) with at least one node.
pub open spec fn proved_nodes(p: Term) -> Option<Seq<Term>> {
    match p {
        Term::Comp(name, args) => if name == ascii("proved"@) && args.len() == 1 {
            match list_items(args[0]) {
                Option::Some(nodes) => if nodes.len() >= 1 {
                    Option::Some(nodes)
                } else {
                    Option::None
                },
                Option::None => Option::None,
            }
        } else {
            Option::None
        },
        _ => Option::None,
    }
}

// Demonstration law over the rows: every sol(Values, P) carries a proof; joins tallied.
pub open spec fn rows_join(coords: Seq<Coord>, digests: Seq<Seq<u8>>, rows: Seq<Term>) -> Join
    decreases rows.len(),
{
    if rows.len() == 0 {
        Join::Ok(0)
    } else {
        match proved_nodes(arg(rows[0], 1)) {
            Option::None => Join::Bad(atom("non_demo"@)),
            Option::Some(nodes) => match roots_join(coords, digests, nodes) {
                Join::Ok(a) => match rows_join(coords, digests, rows.drop_first()) {
                    Join::Ok(b) => Join::Ok(a + b),
                    bad => bad,
                },
                bad => bad,
            },
        }
    }
}

// A committed result demonstrates its claim: yes(proved(..)) or a nonempty
// solutions list whose every row is proved; anything else is non-demo.
pub open spec fn result_join(coords: Seq<Coord>, digests: Seq<Seq<u8>>, result: Term) -> Join {
    match result {
        Term::Comp(name, args) => if name == ascii("yes"@) && args.len() == 1 {
            match proved_nodes(args[0]) {
                Option::Some(nodes) => roots_join(coords, digests, nodes),
                Option::None => Join::Bad(atom("non_demo"@)),
            }
        } else if name == ascii("solutions"@) && args.len() == 1 {
            match list_items(args[0]) {
                Option::Some(rows) => if rows.len() == 0 {
                    Join::Bad(atom("non_demo"@))
                } else {
                    rows_join(coords, digests, rows)
                },
                Option::None => Join::Bad(atom("non_demo"@)),
            }
        } else {
            Join::Bad(atom("non_demo"@))
        },
        _ => Join::Bad(atom("non_demo"@)),
    }
}

pub open spec fn tc_meter(qid: Seq<u8>, nodes: nat) -> Seq<u8> {
    ascii("ckc: trace-check ok "@) + qid + ascii(" nodes="@) + udec_bytes(nodes) + seq![0x0Au8]
}

// Accept iff the committed trace equals the derivation from the committed
// composition, query and answers (custody incl.), every node joins exactly
// one committed clause line, and the result demonstrates its claim.
pub open spec fn trace_check_output(
    mpath: Seq<u8>,
    manifest: Src,
    pls: Seq<Src>,
    pys: Seq<Src>,
    query: Src,
    qsha: Seq<u8>,
    answers: Src,
    asha: Seq<u8>,
    trace: Src,
    digests: Seq<Seq<u8>>,
) -> Out {
    match front(mpath, manifest, pls, pys, query, qsha, answers) {
        Result::Err(o) => o,
        Result::Ok(f) => match trace_custody(trace, f.q, qsha, asha) {
            Result::Err(o) => o,
            Result::Ok(t) => match trace_result(db_of(f.docs), digests, f.q, f.arows, f.a) {
                Result::Err(o) => o,
                Result::Ok(derived) => if derived != t.result {
                    tc_fail(atom("stale"@))
                } else {
                    match result_join(coords_of(f.docs), digests, t.result) {
                        Join::Bad(why) => tc_fail(why),
                        Join::Ok(n) => ok(tc_meter(f.q.qid, n)),
                    }
                },
            },
        },
    }
}

// --- exec-facing mirror (shell/kernel protocol) ---
pub open spec fn lines_view(r: Result<Vec<Vec<u8>>, EOut>) -> Result<Seq<Seq<u8>>, Out> {
    match r {
        Result::Ok(lines) => Result::Ok(lines@.map_values(|l: Vec<u8>| l@)),
        Result::Err(o) => Result::Err(o@),
    }
}

pub open spec fn digests_view(v: Seq<Vec<u8>>) -> Seq<Seq<u8>> {
    v.map_values(|d: Vec<u8>| d@)
}

} // verus!
