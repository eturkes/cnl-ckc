use crate::answers::*;
use crate::engine::*;
use crate::replay::*;
use crate::term::*;
use crate::v1text::*;
use vstd::prelude::*;

verus! {

// Trusted spec: the model checker (contract mc, staged core) over decoded values: a
// composition's clause sequence, a set M of ground literals and a certificate with one claim per
// fact of M. Normative text = bench/sb/semantics.md § Syntax, § Meaning, § Answers. Valuations
// and negation witnesses range over every ground term (a ghost reading); the kernel decides each
// predicate by joins over M, finite under left-to-right safety (D3). The identity's byte framing
// + the certificate envelope = the D11 amendment. vstd's `Set` holds finite sets alone ⇒
// interpretations, fixpoints, answer sets + selections = `ISet`; variable + subterm sets = `Set`.
// --- variables ---
pub open spec fn vars(t: Term) -> Set<nat> {
    var_stream(t).to_set()
}

pub open spec fn vars_all(ts: Seq<Term>) -> Set<nat> {
    var_stream_all(ts).to_set()
}

pub open spec fn item_vars(it: BodyItem) -> Set<nat> {
    match it {
        BodyItem::Pos(l) => vars(l),
        BodyItem::Naf(gs) => vars_all(gs),
    }
}

// x occurs in c outside body item i.
pub open spec fn outside(c: DocClause, i: int, x: nat) -> bool {
    vars(c.head).contains(x) || exists|j: int|
        0 <= j < c.body.len() && j != i && #[trigger] item_vars(c.body[j]).contains(x)
}

// A variable occurring inside one negation alone is local to it; every other one is non-local.
pub open spec fn nonlocal(c: DocClause, x: nat) -> bool {
    vars(c.head).contains(x) || exists|j: int|
        0 <= j < c.body.len() && #[trigger] item_vars(c.body[j]).contains(x) && (c.body[j] is Pos
            || outside(c, j, x))
}

// --- valuations + ground instances ---
pub open spec fn ground_wf(t: Term) -> bool {
    ground(t) && wf_term(t)
}

// θ = one ground term per non-local variable of c, nothing else.
pub open spec fn valuation(c: DocClause, th: Map<nat, Term>) -> bool {
    &&& forall|x: nat|
        #![trigger th.contains_key(x)]
        #![trigger nonlocal(c, x)]
        th.contains_key(x) <==> nonlocal(c, x)
    &&& forall|x: nat| #[trigger] th.contains_key(x) ==> ground_wf(th[x])
}

pub open spec fn apply(t: Term, th: Map<nat, Term>) -> Term
    decreases t,
{
    match t {
        Term::Var(k) => if th.contains_key(k) {
            th[k]
        } else {
            t
        },
        Term::Comp(f, args) => Term::Comp(f, apply_all(args, th)),
        _ => t,
    }
}

pub open spec fn apply_all(ts: Seq<Term>, th: Map<nat, Term>) -> Seq<Term>
    decreases ts,
{
    if ts.len() == 0 {
        Seq::empty()
    } else {
        seq![apply(ts[0], th)] + apply_all(ts.drop_first(), th)
    }
}

// --- literals under an interpretation ---
pub open spec fn builtin(l: Term) -> bool {
    match l {
        Term::Comp(f, args) => crate::compare::is_builtin(f, args.len()),
        _ => false,
    }
}

// A ground literal holds in i: a built-in = its schema relation (REFERENCE § Comparison
// conditions + § Arithmetic built-ins), a schema literal = membership.
pub open spec fn holds(i: ISet<Term>, l: Term) -> bool {
    if builtin(l) {
        crate::compare::holds(l)
    } else {
        i.contains(l)
    }
}

// One witness for a whole conjunction: ground, well-formed, every goal holding in m.
pub open spec fn conj_holds(m: ISet<Term>, w: Seq<Term>) -> bool {
    &&& ground_all(w)
    &&& wf_terms(w)
    &&& forall|k: int| 0 <= k < w.len() ==> #[trigger] holds(m, w[k])
}

// `\+ (G1, …, Gm)` under θ: no σ over the negation's locals makes the conjunction hold in m
// (one correlated witness for the whole conjunction).
pub open spec fn naf_holds(m: ISet<Term>, gs: Seq<Term>, th: Map<nat, Term>) -> bool {
    !exists|s: Map<nat, Term>| #[trigger] conj_holds(m, apply_all(apply_all(gs, th), s))
}

pub open spec fn item_holds(
    m: ISet<Term>,
    i: ISet<Term>,
    it: BodyItem,
    th: Map<nat, Term>,
) -> bool {
    match it {
        BodyItem::Pos(l) => holds(i, apply(l, th)),
        BodyItem::Naf(gs) => naf_holds(m, gs, th),
    }
}

// The body of c under θ: schema literals in i, built-ins hold, negations hold under m.
pub open spec fn body_holds(
    m: ISet<Term>,
    i: ISet<Term>,
    c: DocClause,
    th: Map<nat, Term>,
) -> bool {
    forall|j: int| 0 <= j < c.body.len() ==> #[trigger] item_holds(m, i, c.body[j], th)
}

// --- program + selection (contract mc D1) ---
// Clause ids = positions in `clauses`: the composition's original clause sequence (knowledge
// documents in load order, then the patient suffix). `active` = the ids in play: every id for
// the whole program; a selection keeps every clause, id, document + the root identity.
pub ghost struct Program {
    pub clauses: Seq<DocClause>,
    pub active: ISet<nat>,
    pub doc: Seq<nat>,  // clause id ↦ document index
    pub docids: Seq<Seq<u8>>,  // document index ↦ docid
    pub patient: nat,  // documents from `patient` on = the patient suffix
    pub identity: Seq<
        u8,
    >,  // abstract composition identity (bytes = the D11 amendment's D4)
}

pub open spec fn select(p: Program, s: ISet<nat>) -> Program {
    Program {
        clauses: p.clauses,
        active: p.active.intersect(s),
        doc: p.doc,
        docids: p.docids,
        patient: p.patient,
        identity: p.identity,
    }
}

pub open spec fn instance_of(p: Program, ci: int, th: Map<nat, Term>) -> bool {
    &&& 0 <= ci < p.clauses.len()
    &&& p.active.contains(ci as nat)
    &&& valuation(p.clauses[ci], th)
}

// --- G1 program terms (contract mc D2) ---
pub open spec fn id_name() -> Seq<u8> {
    ascii("$guideline_id"@)
}

pub open spec fn slot_ok(s: Term) -> bool {
    match s {
        Term::Comp(f, a) => (f == ascii("ref"@) || f == ascii("box"@)) && a.len() == 1
            && a[0] is Int,
        _ => false,
    }
}

// A list of variables, [] included.
pub open spec fn var_list(t: Term) -> bool {
    match list_items(t) {
        Option::Some(xs) => forall|k: int| 0 <= k < xs.len() ==> (#[trigger] xs[k]) is Var,
        Option::None => false,
    }
}

// Atoms, integers, canonical rationals, source identities `'$guideline_id'(Kind, Doc, S, Slot,
// Deps)` (Deps a list of variables: [] in a fact, whose variables safety rules out) + variables.
pub open spec fn g1_term(t: Term) -> bool {
    match t {
        Term::Var(_) => true,
        Term::Atom(_) => true,
        Term::Int(_) => true,
        Term::Nil => false,
        Term::Comp(f, a) => crate::arith::number(t) is Some || (f == id_name() && a.len() == 5
            && a[0] is Atom && a[1] is Atom && a[2] is Int && slot_ok(a[3]) && var_list(a[4])),
    }
}

pub open spec fn unary(t: Term, name: Seq<char>) -> Option<Term> {
    match t {
        Term::Comp(f, a) => if f == ascii(name) && a.len() == 1 {
            Option::Some(a[0])
        } else {
            Option::None
        },
        _ => Option::None,
    }
}

pub open spec fn digest_opt(t: Term) -> bool {
    t == Term::Atom(ascii("none"@)) || (unary(t, "sha256"@) matches Option::Some(h) && h is Atom)
}

// Metadata records carry their compounds at these positions alone (§ Syntax):
// guideline_document(DocId, ace_sha256(H), ulex(none | sha256(U)), vocabulary(none | sha256(T)))
// + guideline_supersession(DocId, S, source(Citation, Url), Date).
pub open spec fn metadata_ok(l: Term) -> bool {
    match l {
        Term::Comp(f, a) => if f == indicator(1).0 && a.len() == 4 {
            &&& a[0] is Atom
            &&& unary(a[1], "ace_sha256"@) matches Option::Some(h) && h is Atom
            &&& unary(a[2], "ulex"@) matches Option::Some(u) && digest_opt(u)
            &&& unary(a[3], "vocabulary"@) matches Option::Some(v) && digest_opt(v)
        } else if f == indicator(20).0 && a.len() == 4 {
            &&& a[0] is Atom
            &&& a[1] is Int
            &&& a[2] matches Term::Comp(g, b) && g == ascii("source"@) && b.len() == 2
                && b[0] is Atom && b[1] is Atom
            &&& a[3] is Int
        } else {
            false
        },
        _ => false,
    }
}

pub open spec fn metadata_pred(l: Term) -> bool {
    match l {
        Term::Comp(f, a) => a.len() == 4 && (f == indicator(1).0 || f == indicator(20).0),
        _ => false,
    }
}

// A literal of one of the 22 schema indicators.
pub open spec fn schema_literal(l: Term) -> bool {
    match l {
        Term::Comp(f, a) => exists|i: int|
            0 <= i < indicator_count() && #[trigger] indicator(i) == (f, a.len()),
        _ => false,
    }
}

pub open spec fn g1_literal(l: Term) -> bool {
    if metadata_pred(l) {
        metadata_ok(l)
    } else {
        match l {
            Term::Comp(_, a) => forall|k: int| 0 <= k < a.len() ==> g1_term(#[trigger] a[k]),
            _ => false,
        }
    }
}

pub open spec fn g1_item(it: BodyItem) -> bool {
    match it {
        BodyItem::Pos(l) => g1_literal(l),
        BodyItem::Naf(gs) => forall|k: int| 0 <= k < gs.len() ==> g1_literal(#[trigger] gs[k]),
    }
}

pub open spec fn g1_clause(c: DocClause) -> bool {
    &&& schema_literal(c.head)
    &&& g1_literal(c.head)
    &&& forall|j: int| 0 <= j < c.body.len() ==> #[trigger] g1_item(c.body[j])
}

// --- left-to-right safety (contract mc D3) ---
// Built-in argument modes: a comparison reads every argument; an arithmetic built-in reads its
// first four and writes its last two (REFERENCE § Arithmetic built-ins).
pub open spec fn builtin_inputs(l: Term) -> Seq<Term> {
    match l {
        Term::Comp(f, args) => if crate::arith::is_arith(f, args.len()) {
            args.take(4)
        } else {
            args
        },
        _ => Seq::empty(),
    }
}

pub open spec fn builtin_outputs(l: Term) -> Seq<Term> {
    match l {
        Term::Comp(f, args) => if crate::arith::is_arith(f, args.len()) {
            args.skip(4)
        } else {
            Seq::empty()
        },
        _ => Seq::empty(),
    }
}

// The variables a positive literal binds: a schema literal's, a built-in's outputs.
pub open spec fn binds(l: Term) -> Set<nat> {
    if builtin(l) {
        vars_all(builtin_outputs(l))
    } else {
        vars(l)
    }
}

// Safe after the bound set b: every built-in input bound.
pub open spec fn lit_safe(l: Term, b: Set<nat>) -> bool {
    !builtin(l) || vars_all(builtin_inputs(l)).subset_of(b)
}

// A conjunction, left to right from b.
pub open spec fn conj_safe(gs: Seq<Term>, b: Set<nat>) -> bool
    decreases gs.len(),
{
    gs.len() == 0 || (lit_safe(gs[0], b) && conj_safe(gs.drop_first(), b.union(binds(gs[0]))))
}

// Body items j.. of c, left to right from b: a negation shares only variables bound before it,
// and its conjunction continues from that bound set; it binds nothing for later items.
pub open spec fn items_safe(c: DocClause, j: nat, b: Set<nat>) -> bool
    decreases c.body.len() - j,
{
    if j >= c.body.len() {
        true
    } else {
        match c.body[j as int] {
            BodyItem::Pos(l) => lit_safe(l, b) && items_safe(c, j + 1, b.union(binds(l))),
            BodyItem::Naf(gs) => (forall|x: nat| #[trigger]
                vars_all(gs).contains(x) && outside(c, j as int, x) ==> b.contains(x)) && conj_safe(
                gs,
                b,
            ) && items_safe(c, j + 1, b),
        }
    }
}

pub open spec fn item_binds(it: BodyItem) -> Set<nat> {
    match it {
        BodyItem::Pos(l) => binds(l),
        BodyItem::Naf(_) => Set::empty(),
    }
}

// Every variable some item binds, in any position.
pub open spec fn binders(items: Seq<BodyItem>) -> Set<nat>
    decreases items.len(),
{
    if items.len() == 0 {
        Set::empty()
    } else {
        item_binds(items[0]).union(binders(items.drop_first()))
    }
}

// The body is left-to-right safe and binds every head variable.
pub open spec fn safe_clause(c: DocClause) -> bool {
    items_safe(c, 0, Set::empty()) && vars(c.head).subset_of(binders(c.body))
}

// --- the patient boundary (contract mc D2 G3) ---
// Every subterm of t, t included.
pub open spec fn subterms(t: Term) -> Set<Term>
    decreases t,
{
    match t {
        Term::Comp(_, args) => set![t].union(subterms_all(args)),
        _ => set![t],
    }
}

pub open spec fn subterms_all(ts: Seq<Term>) -> Set<Term>
    decreases ts,
{
    if ts.len() == 0 {
        Set::empty()
    } else {
        subterms(ts[0]).union(subterms_all(ts.drop_first()))
    }
}

// The namespace a subterm names: an identity's Doc, a document record's docid.
pub open spec fn namespace_of(s: Term) -> Option<Term> {
    match s {
        Term::Comp(f, a) => if f == id_name() && a.len() == 5 {
            Option::Some(a[1])
        } else if f == indicator(1).0 && a.len() == 4 {
            Option::Some(a[0])
        } else {
            Option::None
        },
        _ => Option::None,
    }
}

pub open spec fn namespaces(t: Term) -> ISet<Term> {
    ISet::new(
        |n: Term|
            exists|s: Term| #[trigger]
                subterms(t).contains(s) && namespace_of(s) == Option::Some(n),
    )
}

pub open spec fn item_terms(it: BodyItem) -> Seq<Term> {
    match it {
        BodyItem::Pos(l) => seq![l],
        BodyItem::Naf(gs) => gs,
    }
}

pub open spec fn knowledge(p: Program, ci: int) -> bool {
    0 <= ci < p.clauses.len() && p.doc[ci] < p.patient
}

// KBNamespaces(P): every namespace a knowledge clause mentions (head, goals, negations).
pub open spec fn kb_namespaces(p: Program) -> ISet<Term> {
    ISet::new(
        |n: Term|
            exists|ci: int| #[trigger]
                knowledge(p, ci) && (namespaces(p.clauses[ci].head).contains(n) || exists|
                    j: int,
                    k: int,
                |
                    0 <= j < p.clauses[ci].body.len() && 0 <= k < item_terms(
                        p.clauses[ci].body[j],
                    ).len() && #[trigger] namespaces(item_terms(p.clauses[ci].body[j])[k]).contains(
                        n,
                    )),
    )
}

// G3: a patient clause = a ground fact none of whose subterms is a knowledge-base namespace.
pub open spec fn patient_ok(p: Program, ci: int) -> bool {
    &&& p.clauses[ci].body.len() == 0
    &&& ground(p.clauses[ci].head)
    &&& forall|s: Term| #[trigger]
        subterms(p.clauses[ci].head).contains(s) ==> !kb_namespaces(p).contains(s)
}

// --- admission (contract mc D3, MC half; TA adds the typed analysis) ---
pub open spec fn admitted_clause(p: Program, ci: int) -> bool {
    &&& g1_clause(p.clauses[ci])
    &&& safe_clause(p.clauses[ci])
    &&& p.doc[ci] >= p.patient ==> patient_ok(p, ci)
}

pub open spec fn admitted(p: Program) -> bool {
    &&& p.doc.len() == p.clauses.len()
    &&& forall|ci: int| 0 <= ci < p.clauses.len() ==> #[trigger] admitted_clause(p, ci)
}

// --- meaning (contract mc D6; § Meaning) ---
// T_M(I) = the heads of the active ground instances whose body holds (schema literals in I).
pub open spec fn immediate(p: Program, m: ISet<Term>, i: ISet<Term>) -> ISet<Term> {
    ISet::new(
        |h: Term|
            exists|ci: int, th: Map<nat, Term>| #[trigger]
                instance_of(p, ci, th) && body_holds(m, i, p.clauses[ci], th) && h == apply(
                    p.clauses[ci].head,
                    th,
                ),
    )
}

pub open spec fn closed(p: Program, m: ISet<Term>, x: ISet<Term>) -> bool {
    immediate(p, m, x).subset_of(x)
}

// lfp(T_M) = the intersection of every closed set (the reduct's least model).
pub open spec fn lfp(p: Program, m: ISet<Term>) -> ISet<Term> {
    ISet::new(|a: Term| forall|x: ISet<Term>| closed(p, m, x) ==> #[trigger] x.contains(a))
}

pub open spec fn stable(p: Program, m: ISet<Term>) -> bool {
    m == lfp(p, m)
}

// Well-founded model (alternating fixpoint): lower = lfp(Γ∘Γ) with Γ(X) = lfp(T_X), upper =
// Γ(lower); two-valued iff lower = upper. No rank, certificate or proposed model enters it.
pub open spec fn lower(p: Program) -> ISet<Term> {
    ISet::new(
        |a: Term| forall|x: ISet<Term>| lfp(p, lfp(p, x)).subset_of(x) ==> #[trigger] x.contains(a),
    )
}

pub open spec fn upper(p: Program) -> ISet<Term> {
    lfp(p, lower(p))
}

// H1 (contract mc D1; TA's conclusion, a hypothesis of MC's theorems): a two-valued well-founded
// model. MC proves every stable model of such a program = lower(p).
pub open spec fn two_valued(p: Program) -> bool {
    lower(p) == upper(p)
}

// --- certificate (decoded; § Answers) ---
pub ghost struct Claim {
    pub fact: Term,
    pub clause: nat,  // original clause id
    pub valuation: Map<nat, Term>,
    pub level: nat,
}

pub ghost struct Certificate {
    pub identity: Seq<u8>,
    pub claims: Seq<Claim>,  // a sequence: malformed coverage stays representable
}

// Exact coverage: every fact of m claimed once, every claimed fact in m.
pub open spec fn covers(m: ISet<Term>, c: Certificate) -> bool {
    &&& forall|k: int| 0 <= k < c.claims.len() ==> m.contains((#[trigger] c.claims[k]).fact)
    &&& forall|f: Term| #[trigger]
        m.contains(f) ==> exists|k: int|
            0 <= k < c.claims.len() && (#[trigger] c.claims[k]).fact == f
    &&& forall|k: int, l: int|
        0 <= k < l < c.claims.len() ==> (#[trigger] c.claims[k]).fact != (
        #[trigger] c.claims[l]).fact
}

// f has a claim in c at a level below `level`.
pub open spec fn claimed_below(c: Certificate, f: Term, level: nat) -> bool {
    exists|q: int|
        0 <= q < c.claims.len() && (#[trigger] c.claims[q]).fact == f && c.claims[q].level < level
}

// Body item j of claim k's clause: a positive schema literal of its instance has a claim
// strictly below claim k's level.
pub open spec fn supported(p: Program, c: Certificate, k: int, j: int) -> bool {
    let cl = c.claims[k];
    match p.clauses[cl.clause as int].body[j] {
        BodyItem::Pos(l) => builtin(l) || claimed_below(c, apply(l, cl.valuation), cl.level),
        BodyItem::Naf(_) => true,
    }
}

// Claim k: an active ground instance of its clause with head = its fact, the body holding in m,
// every positive schema literal claimed at a lower level; a fact clause at level 0, a rule
// instance at level ≥ 1 (levels are never required minimal).
pub open spec fn claim_valid(p: Program, m: ISet<Term>, c: Certificate, k: int) -> bool {
    let cl = c.claims[k];
    &&& instance_of(p, cl.clause as int, cl.valuation)
    &&& apply(p.clauses[cl.clause as int].head, cl.valuation) == cl.fact
    &&& body_holds(m, m, p.clauses[cl.clause as int], cl.valuation)
    &&& if p.clauses[cl.clause as int].body.len() == 0 {
        cl.level == 0
    } else {
        cl.level >= 1
    }
    &&& forall|j: int|
        0 <= j < p.clauses[cl.clause as int].body.len() ==> #[trigger] supported(p, c, k, j)
}

pub open spec fn accepted(p: Program, m: ISet<Term>, c: Certificate) -> bool {
    &&& c.identity == p.identity
    &&& covers(m, c)
    &&& forall|k: int| 0 <= k < c.claims.len() ==> #[trigger] claim_valid(p, m, c, k)
    &&& closed(p, m, m)
}

// --- questions (contract mc D3 + D7 MC-T3) ---
pub ghost struct Question {
    pub goals: Seq<Term>,  // custody admits schema literals + built-ins alone, no negation
    pub projection: Seq<Term>,  // each answer row's variable, row order
}

// The ','/2 tree of a goal, flattened left to right.
pub open spec fn conj_goals(g: Term) -> Seq<Term>
    decreases g,
{
    match g {
        Term::Comp(f, a) => if f == comma_name() && a.len() == 2 {
            conj_goals(a[0]) + conj_goals(a[1])
        } else {
            seq![g]
        },
        _ => seq![g],
    }
}

pub open spec fn question_of(q: QueryFile, rows: Seq<Term>) -> Question {
    Question { goals: conj_goals(q.goal), projection: rows.map_values(|r: Term| arg(r, 0)) }
}

// The question as the clause `sol([Row…]) :- Goals`: its valuations = the question's.
pub open spec fn question_clause(q: Question) -> DocClause {
    DocClause { head: sol_term(q.projection), body: q.goals.map_values(|g: Term| BodyItem::Pos(g)) }
}

// Question admission: the goals left-to-right safe; every answer-row variable bound by a schema
// literal or a built-in output, independently of goal order.
pub open spec fn safe_question(q: Question) -> bool {
    &&& conj_safe(q.goals, Set::empty())
    &&& vars_all(q.projection).subset_of(binders(question_clause(q).body))
}

// The answers over m: the `sol` terms of every valuation whose goals hold in m.
pub open spec fn answer_terms(m: ISet<Term>, q: Question) -> ISet<Term> {
    ISet::new(
        |t: Term|
            exists|th: Map<nat, Term>| #[trigger]
                valuation(question_clause(q), th) && body_holds(m, m, question_clause(q), th) && t
                    == apply(question_clause(q).head, th),
    )
}

// Strictly increasing in the standard order, exactly the answer set.
pub open spec fn canonical_answers(r: Seq<Term>, m: ISet<Term>, q: Question) -> bool {
    &&& forall|i: int, j: int| 0 <= i < j < r.len() ==> term_lt(#[trigger] r[i], #[trigger] r[j])
    &&& r.to_iset() == answer_terms(m, q)
}

// MC-T3: the existing result vocabulary (`answers::answer_result`) over the model: yes |
// no(finite_failure) without answer rows, else solutions(L), L the canonical answer list.
pub open spec fn result_ok(m: ISet<Term>, q: Question, result: Term) -> bool {
    if q.projection.len() == 0 {
        result == if exists|t: Term| #[trigger] answer_terms(m, q).contains(t) {
            atom("yes"@)
        } else {
            Term::Comp(ascii("no"@), seq![atom("finite_failure"@)])
        }
    } else {
        exists|r: Seq<Term>| #[trigger]
            canonical_answers(r, m, q) && result == Term::Comp(
                ascii("solutions"@),
                seq![list_term(r)],
            )
    }
}

// --- decoding: the core's test + bench surface (contract mc D1) ---
pub open spec fn decode_doc(bytes: Seq<u8>) -> Option<DocFile> {
    if accepts(bytes) {
        match the_v1(bytes) {
            V1File::Doc(d) => Option::Some(d),
            _ => Option::None,
        }
    } else {
        Option::None
    }
}

// A ground term's canonical v1 text, as the verified reader reads it.
pub open spec fn term_decodes(text: Seq<u8>, t: Term) -> bool {
    &&& wf_term(t)
    &&& ground(t)
    &&& no_dollar_var(t)
    &&& term_bytes(t) == text
}

pub open spec fn docs_decode(docs: Seq<Seq<u8>>, ds: Seq<DocFile>) -> bool {
    &&& docs.len() == ds.len()
    &&& forall|i: int| 0 <= i < docs.len() ==> decode_doc(#[trigger] docs[i]) == Option::Some(ds[i])
}

pub open spec fn terms_decode(texts: Seq<Seq<u8>>, ts: Seq<Term>) -> bool {
    &&& texts.len() == ts.len()
    &&& forall|i: int| 0 <= i < texts.len() ==> term_decodes(#[trigger] texts[i], ts[i])
}

// Clause id ↦ document index, aligned with `db_of`: per document, its index once per clause.
pub open spec fn doc_index(ds: Seq<DocFile>) -> Seq<nat>
    decreases ds.len(),
{
    if ds.len() == 0 {
        Seq::empty()
    } else {
        doc_index(ds.drop_last()) + Seq::new(
            doc_db(ds.last()).len(),
            |_k: int| (ds.len() - 1) as nat,
        )
    }
}

// The program of documents ds (load order; per document its two records, then its clauses:
// `replay::db_of`) + the facts supplied directly (one more document, docid empty). The patient
// suffix = the documents from `patient` on + the direct facts.
pub open spec fn program_of(
    ds: Seq<DocFile>,
    facts: Seq<Term>,
    patient: nat,
    identity: Seq<u8>,
) -> Program {
    let clauses = db_of(ds) + facts.map_values(|t: Term| fact_clause(t));
    Program {
        clauses,
        active: ISet::new(|ci: nat| ci < clauses.len()),
        doc: doc_index(ds) + Seq::new(facts.len(), |_k: int| ds.len()),
        docids: ds.map_values(|d: DocFile| d.docid).push(Seq::empty()),
        patient: if patient <= ds.len() {
            patient
        } else {
            ds.len()
        },
        identity,
    }
}

pub ghost struct ClaimText {
    pub fact: Seq<u8>,
    pub clause: nat,
    pub valuation: Seq<(nat, Seq<u8>)>,  // (variable index, ground term text)
    pub level: nat,
}

pub ghost struct CertText {
    pub identity: Seq<u8>,
    pub claims: Seq<ClaimText>,
}

pub open spec fn valuation_decodes(v: Seq<(nat, Seq<u8>)>, th: Map<nat, Term>) -> bool {
    &&& forall|a: int, b: int| 0 <= a < b < v.len() ==> (#[trigger] v[a]).0 != (#[trigger] v[b]).0
    &&& th.dom() == v.map_values(|b: (nat, Seq<u8>)| b.0).to_set()
    &&& forall|a: int| 0 <= a < v.len() ==> term_decodes((#[trigger] v[a]).1, th[v[a].0])
}

pub open spec fn claim_decodes(t: ClaimText, c: Claim) -> bool {
    &&& term_decodes(t.fact, c.fact)
    &&& c.clause == t.clause
    &&& c.level == t.level
    &&& valuation_decodes(t.valuation, c.valuation)
}

pub open spec fn cert_decodes(t: CertText, c: Certificate) -> bool {
    &&& c.identity == t.identity
    &&& c.claims.len() == t.claims.len()
    &&& forall|k: int|
        0 <= k < t.claims.len() ==> claim_decodes(#[trigger] t.claims[k], c.claims[k])
}

pub open spec fn under(p: Program, slice: Option<ISet<nat>>) -> Program {
    match slice {
        Option::Some(s) => select(p, s),
        Option::None => p,
    }
}

// MC-T2 (core half) at the test + bench surface: a verified check = the whole composition
// admitted + the spec's acceptance over the decoded values (on the slice when one is named).
pub open spec fn check_verified(
    docs: Seq<Seq<u8>>,
    facts: Seq<Seq<u8>>,
    patient: nat,
    identity: Seq<u8>,
    slice: Option<ISet<nat>>,
    model: Seq<Seq<u8>>,
    cert: CertText,
) -> bool {
    exists|ds: Seq<DocFile>, fs: Seq<Term>, ms: Seq<Term>, c: Certificate|
        #![trigger docs_decode(docs, ds), terms_decode(facts, fs), terms_decode(model, ms), cert_decodes(cert, c)]
        {
            &&& docs_decode(docs, ds)
            &&& terms_decode(facts, fs)
            &&& terms_decode(model, ms)
            &&& cert_decodes(cert, c)
            &&& admitted(program_of(ds, fs, patient, identity))
            &&& accepted(under(program_of(ds, fs, patient, identity), slice), ms.to_iset(), c)
        }
}

// MC-T3 at the test + bench surface: a verified answer = a custody-accepted question admitted
// by D3, a verified check over the slice, and the result's canonical bytes.
pub open spec fn answer_verified(
    docs: Seq<Seq<u8>>,
    facts: Seq<Seq<u8>>,
    patient: nat,
    identity: Seq<u8>,
    slice: ISet<nat>,
    model: Seq<Seq<u8>>,
    cert: CertText,
    query: Src,
    result: Seq<u8>,
) -> bool {
    match custody(query) {
        Result::Ok((qf, rows)) => {
            &&& safe_question(question_of(qf, rows))
            &&& exists|ds: Seq<DocFile>, fs: Seq<Term>, ms: Seq<Term>, c: Certificate|
                #![trigger docs_decode(docs, ds), terms_decode(facts, fs), terms_decode(model, ms), cert_decodes(cert, c)]
                {
                    &&& docs_decode(docs, ds)
                    &&& terms_decode(facts, fs)
                    &&& terms_decode(model, ms)
                    &&& cert_decodes(cert, c)
                    &&& admitted(program_of(ds, fs, patient, identity))
                    &&& accepted(
                        select(program_of(ds, fs, patient, identity), slice),
                        ms.to_iset(),
                        c,
                    )
                    &&& exists|t: Term| #[trigger]
                        result_ok(ms.to_iset(), question_of(qf, rows), t) && term_bytes(t) == result
                }
        },
        Result::Err(_) => false,
    }
}

// --- exec-facing mirrors (shell/kernel protocol) ---
pub struct EMcBinding {
    pub var: usize,
    pub term: Vec<u8>,
}

pub struct EMcClaim {
    pub fact: Vec<u8>,
    pub clause: usize,
    pub valuation: Vec<EMcBinding>,
    pub level: u64,
}

impl View for EMcClaim {
    type V = ClaimText;

    open spec fn view(&self) -> ClaimText {
        ClaimText {
            fact: self.fact@,
            clause: self.clause as nat,
            valuation: self.valuation@.map_values(|b: EMcBinding| (b.var as nat, b.term@)),
            level: self.level as nat,
        }
    }
}

pub struct EMcCertificate {
    pub identity: Vec<u8>,
    pub claims: Vec<EMcClaim>,
}

impl View for EMcCertificate {
    type V = CertText;

    open spec fn view(&self) -> CertText {
        CertText { identity: self.identity@, claims: self.claims@.map_values(|c: EMcClaim| c@) }
    }
}

// Contract mc D9: stages in precedence order.
pub enum EMcStage {
    Admission,
    Question,
    Identity,
    Coverage,
    Claim,
    Closure,
}

// The failing item a refusal names: a clause (document index, ordinal in its document: the
// direct facts = document `docs.len()`); a goal position | an answer-row ordinal (1-based, as
// custody numbers rows); the composition; an unclaimed fact of M (canonical text); a claim
// position (0-based); an active clause + a derivable ground head outside M.
pub enum EMcItem {
    Clause { doc: usize, ordinal: usize },
    Goal(usize),
    Row(usize),
    Composition,
    Fact(Vec<u8>),
    ClaimAt(usize),
    Derivable { clause: usize, head: Vec<u8> },
}

// Term + number size: the kernel's own bounds; memory + the deadline: its callers'.
pub enum EMcReason {
    TermSize,
    NumberSize,
    Memory,
    Deadline,
}

pub enum EMcOutcome {
    Verified,
    VerifiedResult(Vec<u8>),
    Refused(EMcStage, EMcItem),
    Indeterminate(EMcReason),
}

// Inputs outside the core's decoded domain: the question's custody reject (its own Out), then
// the first document, direct fact, model line or claim that does not decode.
pub enum EMcInput {
    Query(EOut),
    Document(usize),
    Fact(usize),
    ModelLine(usize),
    Claim(usize),
}

pub open spec fn bytes_seq(v: Seq<Vec<u8>>) -> Seq<Seq<u8>> {
    v.map_values(|b: Vec<u8>| b@)
}

pub open spec fn slice_set(s: Seq<usize>) -> ISet<nat> {
    s.map_values(|x: usize| x as nat).to_iset()
}

pub open spec fn slice_view(s: Option<Vec<usize>>) -> Option<ISet<nat>> {
    match s {
        Option::Some(v) => Option::Some(slice_set(v@)),
        Option::None => Option::None,
    }
}

} // verus!
