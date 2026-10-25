use crate::answers::*;
use crate::engine::*;
use crate::replay::*;
use crate::term::*;
use crate::trace::*;
use crate::v1text::*;
use crate::vocabulary::*;
use vstd::prelude::*;

verus! {

// Trusted spec: emission certification (contract m6, rulings R40–R43). The
// fork `ace_to_pl.pl` compiles APE's resolved DRS into v1 documents and
// query projections; this module states the DRS→v1 relation it must have
// followed, so a committed artifact is certified against the DRS a thin
// upstream-APE driver derives at check time — never against the fork.
// Certified = (1) custody: the artifact names its source (docid, ACE +
// lexicon digests, one bundle per ACE line quoting that line); (2)
// projection: `project` over the DRS yields exactly the committed clauses,
// sentence by sentence; (3) payload: every clause group carries one ground
// obligation (the replay input K2 discharges). Acceptance only: a DRS the
// projection cannot express certifies nothing. APE (ACE→DRS) stays the
// trusted oracle; modality and cardinality are reified, never interpreted.
// --- the driver's dump: three canonical term lines ---
pub ghost struct Dump {
    pub sentences: Term,  // APE token lists, one per sentence
    pub drs: Term,  // the resolved DRS
    pub messages: Term,  // APE messages; certification requires none
}

pub open spec fn print_dump(d: Dump) -> Seq<u8> {
    term_line(Term::Comp(ascii("sentences"@), seq![d.sentences])) + term_line(
        Term::Comp(ascii("drs"@), seq![d.drs]),
    ) + term_line(Term::Comp(ascii("messages"@), seq![d.messages]))
}

// One numbervars pass over the whole dump: referents keep one identity
// across all three lines.
pub open spec fn wf_dump(d: Dump) -> bool {
    &&& wf_term(d.sentences) && wf_term(d.drs) && wf_term(d.messages)
    &&& no_dollar_var(d.sentences) && no_dollar_var(d.drs) && no_dollar_var(d.messages)
    &&& var_canonical(var_stream(d.sentences) + var_stream(d.drs) + var_stream(d.messages))
}

pub open spec fn dump_accepts(bytes: Seq<u8>) -> bool {
    exists|d: Dump| #[trigger] wf_dump(d) && print_dump(d) == bytes
}

pub open spec fn the_dump(bytes: Seq<u8>) -> Dump {
    choose|d: Dump| wf_dump(d) && print_dump(d) == bytes
}

// --- DRS shapes ---
pub open spec fn drs_name() -> Seq<u8> {
    ascii("drs"@)
}

pub open spec fn gid(role: Seq<char>, docid: Seq<u8>, s: nat, slot: Term, deps: Term) -> Term {
    Term::Comp(gid_name(), seq![atom(role), Term::Atom(docid), Term::Int(s as int), slot, deps])
}

pub open spec fn ref_slot(n: nat) -> Term {
    Term::Comp(ascii("ref"@), seq![Term::Int(n as int)])
}

pub open spec fn box_slot(b: nat) -> Term {
    Term::Comp(ascii("box"@), seq![Term::Int(b as int)])
}

pub open spec fn actual() -> Term {
    atom("actual"@)
}

pub open spec fn is_comp(t: Term, name: Seq<char>, arity: nat) -> bool {
    match t {
        Term::Comp(n, args) => n == ascii(name) && args.len() == arity,
        _ => false,
    }
}

// drs(Dom, Conds): Dom a proper list of referents; Conds a proper list.
pub open spec fn box_parts(t: Term) -> Option<(Seq<Term>, Term)> {
    if is_comp(t, "drs"@, 2) {
        match list_items(arg(t, 0)) {
            Option::Some(dom) => match list_items(arg(t, 1)) {
                Option::Some(_) => Option::Some((dom, arg(t, 1))),
                Option::None => Option::None,
            },
            Option::None => Option::None,
        }
    } else {
        Option::None
    }
}

// -(Inner, S/T): a condition anchored at sentence S, token T.
pub open spec fn anchor_s(t: Term) -> Option<int> {
    if is_comp(t, "-"@, 2) && is_comp(arg(t, 1), "/"@, 2) {
        match (arg(arg(t, 1), 0), arg(arg(t, 1), 1)) {
            (Term::Int(s), Term::Int(_)) => Option::Some(s),
            _ => Option::None,
        }
    } else {
        Option::None
    }
}

pub open spec fn is_modal(name: Seq<u8>) -> bool {
    name == ascii("should"@) || name == ascii("must"@) || name == ascii("can"@) || name == ascii(
        "may"@,
    )
}

pub open spec fn is_op(name: Seq<u8>) -> bool {
    is_modal(name) || name == ascii("-"@)
}

// Anchor carriers: boxes, wrappers, connectives, list spines (never leaf payloads).
pub open spec fn is_carrier(name: Seq<u8>, arity: nat) -> bool {
    ||| (name == drs_name() && arity == 2)
    ||| (name == ascii("question"@) && arity == 1)
    ||| (is_op(name) && arity == 1)
    ||| (name == ascii("~"@) && arity == 1)
    ||| (name == ascii("=>"@) && arity == 2)
    ||| (name == ascii("v"@) && arity == 2)
    ||| (name == cons_name() && arity == 2)
}

// Every anchor sentence reachable through carriers (the fork's sub_anchor).
pub open spec fn anchors(t: Term) -> Seq<int>
    decreases t, 0int,
{
    match anchor_s(t) {
        Option::Some(s) => seq![s] + anchors(arg(t, 0)),
        Option::None => match t {
            Term::Comp(name, args) => if is_carrier(name, args.len()) {
                anchors_all(args)
            } else {
                Seq::empty()
            },
            _ => Seq::empty(),
        },
    }
}

pub open spec fn anchors_all(ts: Seq<Term>) -> Seq<int>
    decreases ts, 1int,
{
    if ts.len() == 0 {
        Seq::empty()
    } else {
        anchors(ts[0]) + anchors_all(ts.drop_first())
    }
}

// Exactly one sentence id inside a subterm.
pub open spec fn inner_sentence(t: Term) -> Option<int> {
    let ss = anchors(t);
    if ss.len() > 0 && (forall|i: int| 0 <= i < ss.len() ==> #[trigger] ss[i] == ss[0]) {
        Option::Some(ss[0])
    } else {
        Option::None
    }
}

// Root exact group (contract root-exact D1). APE wraps an upper-bounded quantifier's
// scope in a list. A root list joins its sentence's facts when every upper-bounded
// object among its root-context leaves (anchored conditions reached through list
// spines alone; a box, NAF, rule or disjunction keeps its own law in the flatten)
// states `exactly N` with N >= 1, and one does: such a count implies that the counted
// things exist, while `at most`, `less than` and `exactly 0` allow zero.
pub open spec fn group_leaves(t: Term) -> Seq<Term>
    decreases t, 0int,
{
    match anchor_s(t) {
        Option::Some(_) => seq![arg(t, 0)],
        Option::None => match t {
            Term::Comp(name, args) => if name == cons_name() && args.len() == 2 {
                group_leaves_all(args)
            } else {
                Seq::empty()
            },
            _ => Seq::empty(),
        },
    }
}

pub open spec fn group_leaves_all(ts: Seq<Term>) -> Seq<Term>
    decreases ts, 1int,
{
    if ts.len() == 0 {
        Seq::empty()
    } else {
        group_leaves(ts[0]) + group_leaves_all(ts.drop_first())
    }
}

pub open spec fn upper_object(c: Term) -> bool {
    is_comp(c, "object"@, 6) && (arg(c, 4) == atom("exactly"@) || arg(c, 4) == atom("leq"@) || arg(
        c,
        4,
    ) == atom("less"@))
}

pub open spec fn exact_object(c: Term) -> bool {
    is_comp(c, "object"@, 6) && arg(c, 4) == atom("exactly"@) && match arg(c, 5) {
        Term::Int(n) => n >= 1,
        _ => false,
    }
}

pub open spec fn exact_group(t: Term) -> bool {
    let ls = group_leaves(t);
    &&& list_items(t) is Some
    &&& forall|i: int| 0 <= i < ls.len() && upper_object(#[trigger] ls[i]) ==> exact_object(ls[i])
    &&& exists|i: int| 0 <= i < ls.len() && exact_object(#[trigger] ls[i])
}

// Root condition classes (document mode): anchored fact, rule, operator box
// (an exact group rides the box form).
pub ghost enum Root {
    Anchored(Term),
    Rule(Term, Term),
    Boxed(Term),
}

pub open spec fn tag(c: Term) -> Option<(int, Root)> {
    match anchor_s(c) {
        Option::Some(s) => Option::Some((s, Root::Anchored(arg(c, 0)))),
        Option::None => match c {
            Term::Comp(name, args) => if name == ascii("=>"@) && args.len() == 2 {
                match inner_sentence(c) {
                    Option::Some(s) => Option::Some((s, Root::Rule(args[0], args[1]))),
                    Option::None => Option::None,
                }
            } else if is_op(name) && args.len() == 1 {
                match inner_sentence(c) {
                    Option::Some(s) => Option::Some((s, Root::Boxed(c))),
                    Option::None => Option::None,
                }
            } else if name == cons_name() && args.len() == 2 && exact_group(c) {
                match inner_sentence(c) {
                    Option::Some(s) => Option::Some((s, Root::Boxed(c))),
                    Option::None => Option::None,
                }
            } else {
                Option::None  // question, NAF and any other root shape reject

            },
            _ => Option::None,
        },
    }
}

pub open spec fn tags(cs: Seq<Term>) -> Option<Seq<(int, Root)>>
    decreases cs.len(),
{
    if cs.len() == 0 {
        Option::Some(Seq::empty())
    } else {
        match tag(cs[0]) {
            Option::None => Option::None,
            Option::Some(t) => match tags(cs.drop_first()) {
                Option::None => Option::None,
                Option::Some(rest) => Option::Some(seq![t] + rest),
            },
        }
    }
}

pub open spec fn of_sentence(tagged: Seq<(int, Root)>, s: int) -> Seq<Root> {
    tagged.filter(|p: (int, Root)| p.0 == s).map_values(|p: (int, Root)| p.1)
}

pub open spec fn in_range(tagged: Seq<(int, Root)>, count: nat) -> bool {
    forall|i: int| 0 <= i < tagged.len() ==> 1 <= (#[trigger] tagged[i]).0 <= count
}

// --- flattening: conditions → items (the fork's v1_flatten_items) ---
pub ghost enum Where {
    Root,
    Antecedent,
    Consequent,
}

pub ghost enum Encl {
    Top,
    Op,
    Naf,
}

// Anch(outer context, condition); Op(box ordinal, outer, inner context, operator);
// Naf(box domain, payload items).
pub ghost enum Item {
    Anch(Term, Term),
    Op(nat, Term, Term, Seq<u8>),
    Naf(Seq<Term>, Seq<Item>),
}

pub ghost struct Flat {
    pub items: Seq<Item>,
    pub n: nat,  // next box ordinal
}

pub open spec fn has_anch(items: Seq<Item>) -> bool {
    exists|i: int| 0 <= i < items.len() && (#[trigger] items[i]) is Anch
}

pub open spec fn op_context(
    w: Where,
    docid: Seq<u8>,
    s: nat,
    deps: Term,
    b: nat,
    base: nat,
) -> Term {
    match w {
        Where::Antecedent => Term::Var(base + b),  // an existential body variable
        _ => gid("context"@, docid, s, box_slot(b), deps),
    }
}

// One condition. `base` = a variable index above every DRS referent, so
// antecedent box contexts mint distinct fresh variables.
pub open spec fn flatten_cond(
    c: Term,
    w: Where,
    s: nat,
    docid: Seq<u8>,
    deps: Term,
    outer: Term,
    encl: Encl,
    n: nat,
    base: nat,
) -> Result<Flat, Term>
    decreases c, 1int,
{
    match anchor_s(c) {
        Option::Some(_) => Result::Ok(Flat { items: seq![Item::Anch(outer, arg(c, 0))], n }),
        Option::None => match c {
            Term::Nil => Result::Ok(Flat { items: Seq::empty(), n }),
            Term::Comp(name, args) => if name == cons_name() && args.len() == 2 {
                flatten_list(c, w, s, docid, deps, outer, encl, n, base)
            } else if name == ascii("=>"@) && args.len() == 2 {
                Result::Err(
                    match encl {
                        Encl::Op => atom("operator_scoped_rule"@),
                        _ => atom("condition_shape"@),
                    },
                )
            } else if name == ascii("v"@) && args.len() == 2 {
                Result::Err(atom("disjunction"@))
            } else if name == ascii("~"@) && args.len() == 1 {
                if w is Antecedent && encl is Top {
                    match box_parts(args[0]) {
                        Option::None => Result::Err(atom("invalid_drs_shape"@)),
                        Option::Some((ndom, nconds)) => match flatten_list(
                            nconds,
                            w,
                            s,
                            docid,
                            deps,
                            outer,
                            Encl::Naf,
                            n,
                            base,
                        ) {
                            Result::Err(e) => Result::Err(e),
                            Result::Ok(f) => if has_anch(f.items) {
                                Result::Ok(Flat { items: seq![Item::Naf(ndom, f.items)], n: f.n })
                            } else {
                                Result::Err(atom("naf_shape"@))
                            },
                        },
                    }
                } else {
                    Result::Err(atom("naf_placement"@))
                }
            } else if is_op(name) && args.len() == 1 {
                if encl is Naf {
                    Result::Err(atom("deferred_operator"@))
                } else {
                    match box_parts(args[0]) {
                        Option::None => Result::Err(atom("invalid_drs_shape"@)),
                        Option::Some((_, conds)) => {
                            let inner = op_context(w, docid, s, deps, n, base);
                            match flatten_list(
                                conds,
                                w,
                                s,
                                docid,
                                deps,
                                inner,
                                Encl::Op,
                                n + 1,
                                base,
                            ) {
                                Result::Err(e) => Result::Err(e),
                                Result::Ok(f) => if has_anch(f.items) {
                                    Result::Ok(
                                        Flat {
                                            items: seq![Item::Op(n, outer, inner, name)] + f.items,
                                            n: f.n,
                                        },
                                    )
                                } else {
                                    Result::Err(atom("condition_shape"@))
                                },
                            }
                        },
                    }
                }
            } else {
                Result::Err(atom("condition_shape"@))
            },
            _ => Result::Err(atom("condition_shape"@)),
        },
    }
}

// A condition list spine, left to right.
pub open spec fn flatten_list(
    l: Term,
    w: Where,
    s: nat,
    docid: Seq<u8>,
    deps: Term,
    outer: Term,
    encl: Encl,
    n: nat,
    base: nat,
) -> Result<Flat, Term>
    decreases l, 0int,
{
    match l {
        Term::Nil => Result::Ok(Flat { items: Seq::empty(), n }),
        Term::Comp(name, args) => if name == cons_name() && args.len() == 2 {
            match flatten_cond(args[0], w, s, docid, deps, outer, encl, n, base) {
                Result::Err(e) => Result::Err(e),
                Result::Ok(f1) => match flatten_list(
                    args[1],
                    w,
                    s,
                    docid,
                    deps,
                    outer,
                    encl,
                    f1.n,
                    base,
                ) {
                    Result::Err(e) => Result::Err(e),
                    Result::Ok(f2) => Result::Ok(Flat { items: f1.items + f2.items, n: f2.n }),
                },
            }
        } else {
            Result::Err(atom("invalid_drs_shape"@))
        },
        _ => Result::Err(atom("invalid_drs_shape"@)),
    }
}

// --- referent bookkeeping ---
// Referent slots of one condition: object → ref; predicate → event then
// participants; pp → event, object; property → ref.
pub open spec fn cond_refs(inner: Term) -> Seq<Term> {
    match inner {
        Term::Comp(name, args) => if name == ascii("object"@) && args.len() == 6 {
            seq![args[0]]
        } else if name == ascii("predicate"@) && args.len() >= 3 {
            seq![args[0]] + args.skip(2)
        } else if name == ascii("modifier_pp"@) && args.len() == 3 {
            seq![args[0], args[2]]
        } else if name == ascii("property"@) && args.len() == 3 {
            seq![args[0]]
        } else if (name == ascii("$guideline_bound"@) && args.len() == 8) || (name == ascii(
            "$guideline_measure"@,
        ) && args.len() == 4) || (name == ascii("$guideline_amount"@) && args.len() == 6) {
            seq![args[0]]
        } else {
            Seq::empty()
        },
        _ => Seq::empty(),
    }
}

pub open spec fn item_refs(it: Item) -> Seq<Term>
    decreases it, 0int,
{
    match it {
        Item::Anch(_, inner) => cond_refs(inner),
        Item::Op(_, _, _, _) => Seq::empty(),
        Item::Naf(_, payload) => ref_slots(payload),
    }
}

pub open spec fn ref_slots(items: Seq<Item>) -> Seq<Term>
    decreases items, 1int,
{
    if items.len() == 0 {
        Seq::empty()
    } else {
        item_refs(items[0]) + ref_slots(items.drop_first())
    }
}

// Distinct variables in first-occurrence order (non-variables skipped).
pub open spec fn first_vars(slots: Seq<Term>, acc: Seq<Term>) -> Seq<Term>
    decreases slots.len(),
{
    if slots.len() == 0 {
        acc
    } else if slots[0] is Var && !acc.contains(slots[0]) {
        first_vars(slots.drop_first(), acc.push(slots[0]))
    } else {
        first_vars(slots.drop_first(), acc)
    }
}

// 1-based position of a variable in an ordering.
pub open spec fn nth_var(ordered: Seq<Term>, v: Term) -> nat
    decreases ordered.len(),
{
    if ordered.len() == 0 || ordered[0] == v {
        1
    } else {
        1 + nth_var(ordered.drop_first(), v)
    }
}

pub open spec fn lookup(map: Seq<(Term, Term)>, v: Term) -> Option<Term>
    decreases map.len(),
{
    if map.len() == 0 {
        Option::None
    } else if map[0].0 == v {
        Option::Some(map[0].1)
    } else {
        lookup(map.drop_first(), v)
    }
}

// Root facts mint product identities for every unmapped referent, in order.
pub open spec fn mint(
    ordered: Seq<Term>,
    n: nat,
    s: nat,
    docid: Seq<u8>,
    map: Seq<(Term, Term)>,
) -> Seq<(Term, Term)>
    decreases ordered.len(),
{
    if ordered.len() == 0 {
        map
    } else {
        let map1 = if lookup(map, ordered[0]) is Some {
            map
        } else {
            map.push((ordered[0], gid("product"@, docid, s, ref_slot(n), Term::Nil)))
        };
        mint(ordered.drop_first(), n + 1, s, docid, map1)
    }
}

// --- condition expansion (the seven indicators) ---
pub open spec fn resolve(a: Term, map: Seq<(Term, Term)>, sko: Seq<(Term, Term)>) -> Result<
    Term,
    Term,
> {
    match a {
        Term::Var(_) => match lookup(map, a) {
            Option::Some(id) => Result::Ok(id),
            Option::None => match lookup(sko, a) {
                Option::Some(id) => Result::Ok(id),
                Option::None => Result::Ok(a),
            },
        },
        _ => Result::Err(atom("unresolved_argument"@)),
    }
}

// `about` (contract q13) = table-backed only: the annotate pass mints it in place
// of `eq`; APE's comparisons never include it.
pub open spec fn card_op_ok(op: Term) -> bool {
    ||| op == atom("eq"@)
    ||| op == atom("about"@)
    ||| op == atom("geq"@)
    ||| op == atom("greater"@)
    ||| op == atom("leq"@)
    ||| op == atom("less"@)
    ||| op == atom("exactly"@)
    ||| op == atom("na"@)
}

pub open spec fn lit(name: Seq<char>, args: Seq<Term>) -> Term {
    Term::Comp(ascii(name), args)
}

pub open spec fn participants(
    ctx: Term,
    e: Term,
    args: Seq<Term>,
    pos: nat,
    map: Seq<(Term, Term)>,
    sko: Seq<(Term, Term)>,
) -> Result<Seq<Term>, Term>
    decreases args.len(),
{
    if args.len() == 0 {
        Result::Ok(Seq::empty())
    } else {
        match resolve(args[0], map, sko) {
            Result::Err(e) => Result::Err(e),
            Result::Ok(r) => match participants(ctx, e, args.drop_first(), pos + 1, map, sko) {
                Result::Err(e) => Result::Err(e),
                Result::Ok(rest) => Result::Ok(
                    seq![lit("guideline_arg"@, seq![ctx, e, Term::Int(pos as int), r])] + rest,
                ),
            },
        }
    }
}

pub open spec fn condition(
    ctx: Term,
    inner: Term,
    map: Seq<(Term, Term)>,
    sko: Seq<(Term, Term)>,
) -> Result<Seq<Term>, Term> {
    match inner {
        Term::Comp(name, args) => if name == ascii("object"@) && args.len() == 6 {
            if !card_op_ok(args[4]) {
                Result::Err(atom("object_operator"@))
            } else {
                match resolve(args[0], map, sko) {
                    Result::Err(e) => Result::Err(e),
                    Result::Ok(r) => Result::Ok(
                        seq![
                            lit("guideline_entity"@, seq![ctx, r, args[1], args[2]]),
                            lit("guideline_cardinality"@, seq![ctx, r, args[3], args[4], args[5]]),
                        ],
                    ),
                }
            }
        } else if name == ascii("predicate"@) && args.len() >= 3 {
            if args.len() > 5 {
                Result::Err(atom("condition_shape"@))
            } else {
                match resolve(args[0], map, sko) {
                    Result::Err(e) => Result::Err(e),
                    Result::Ok(e) => match participants(ctx, e, args.skip(2), 1, map, sko) {
                        Result::Err(er) => Result::Err(er),
                        Result::Ok(ps) => Result::Ok(
                            seq![lit("guideline_event"@, seq![ctx, e, args[1]])] + ps,
                        ),
                    },
                }
            }
        } else if name == ascii("modifier_pp"@) && args.len() == 3 {
            match (resolve(args[0], map, sko), resolve(args[2], map, sko)) {
                (Result::Ok(e), Result::Ok(o)) => Result::Ok(
                    seq![lit("guideline_pp"@, seq![ctx, e, args[1], o])],
                ),
                _ => Result::Err(atom("unresolved_argument"@)),
            }
        } else if name == ascii("property"@) && args.len() == 3 {
            if args[2] != atom("pos"@) {
                Result::Err(atom("property_polarity"@))
            } else {
                match resolve(args[0], map, sko) {
                    Result::Err(e) => Result::Err(e),
                    Result::Ok(p) => Result::Ok(
                        seq![lit("guideline_property"@, seq![ctx, p, args[1], atom("pos"@)])],
                    ),
                }
            }
        } else if name == ascii("$guideline_interval"@) && args.len() == 5 {
            match (
                resolve(args[0], map, sko),
                resolve(args[2], map, sko),
                if args[4] is Var {
                    resolve(args[4], map, sko)
                } else {
                    Result::Ok(args[4])
                },
            ) {
                (Result::Ok(e), Result::Ok(q), Result::Ok(a)) => Result::Ok(
                    seq![lit("guideline_interval"@, seq![ctx, e, args[1], q, args[3], a])],
                ),
                _ => Result::Err(atom("unresolved_argument"@)),
            }
        } else if name == ascii("$guideline_recurrence"@) && args.len() == 3 {
            match (resolve(args[0], map, sko), resolve(args[1], map, sko)) {
                (Result::Ok(e), Result::Ok(q)) => Result::Ok(
                    seq![lit("guideline_recurrence"@, seq![ctx, e, q, args[2]])],
                ),
                _ => Result::Err(atom("unresolved_argument"@)),
            }
        } else if name == ascii("$guideline_frequency"@) && args.len() == 4 {
            match (
                resolve(args[0], map, sko),
                resolve(args[1], map, sko),
                resolve(args[2], map, sko),
            ) {
                (Result::Ok(e), Result::Ok(c), Result::Ok(w)) => Result::Ok(
                    seq![lit("guideline_frequency"@, seq![ctx, e, c, w, args[3]])],
                ),
                _ => Result::Err(atom("unresolved_argument"@)),
            }
        } else if name == ascii("$guideline_order"@) && args.len() == 3 {
            match (resolve(args[0], map, sko), resolve(args[2], map, sko)) {
                (Result::Ok(e), Result::Ok(a)) => Result::Ok(
                    seq![lit("guideline_order"@, seq![ctx, e, args[1], a])],
                ),
                _ => Result::Err(atom("unresolved_argument"@)),
            }
        } else if name == ascii("$guideline_range"@) && args.len() == 2 {
            match (resolve(args[0], map, sko), resolve(args[1], map, sko)) {
                (Result::Ok(l), Result::Ok(h)) => Result::Ok(
                    seq![lit("guideline_range"@, seq![ctx, l, h])],
                ),
                _ => Result::Err(atom("unresolved_argument"@)),
            }
        } else if name == ascii("$guideline_recurrence_window"@) && args.len() == 6 {
            match (
                resolve(args[0], map, sko),
                resolve(args[1], map, sko),
                resolve(args[3], map, sko),
                resolve(args[4], map, sko),
            ) {
                (Result::Ok(e), Result::Ok(q), Result::Ok(a), Result::Ok(l)) => Result::Ok(
                    seq![
                        lit(
                            "guideline_recurrence_window"@,
                            seq![ctx, e, q, args[2], a, l, args[5]],
                        ),
                    ],
                ),
                _ => Result::Err(atom("unresolved_argument"@)),
            }
        } else if name == ascii("$guideline_grade"@) && args.len() == 4 {
            match (resolve(args[0], map, sko), resolve(args[3], map, sko)) {
                (Result::Ok(r), Result::Ok(l)) => Result::Ok(
                    seq![lit("guideline_grade"@, seq![ctx, r, args[1], args[2], l])],
                ),
                _ => Result::Err(atom("unresolved_argument"@)),
            }
        } else if name == ascii("$guideline_preference"@) && args.len() == 3 {
            match (
                resolve(args[0], map, sko),
                resolve(args[1], map, sko),
                if args[2] is Var {
                    resolve(args[2], map, sko)
                } else {
                    Result::Ok(args[2])
                },
            ) {
                (Result::Ok(e), Result::Ok(p), Result::Ok(a)) => Result::Ok(
                    seq![lit("guideline_preference"@, seq![ctx, e, p, a])],
                ),
                _ => Result::Err(atom("unresolved_argument"@)),
            }
        } else if name == ascii("$guideline_qualifier"@) && args.len() == 3 {
            match resolve(args[0], map, sko) {
                Result::Err(er) => Result::Err(er),
                Result::Ok(e) => Result::Ok(
                    seq![lit("guideline_qualifier"@, seq![ctx, e, args[1], args[2]])],
                ),
            }
        } else if name == ascii("$guideline_unit"@) && args.len() == 2 {
            match resolve(args[0], map, sko) {
                Result::Err(er) => Result::Err(er),
                Result::Ok(q) => Result::Ok(seq![lit("guideline_unit"@, seq![ctx, q, args[1]])]),
            }
        } else if name == ascii("$guideline_bound"@) && args.len() == 8 {
            match resolve(args[0], map, sko) {
                Result::Err(er) => Result::Err(er),
                Result::Ok(r) => Result::Ok(
                    seq![
                        lit("guideline_entity"@, seq![ctx, r, args[1], args[2]]),
                        lit("guideline_cardinality"@, seq![ctx, r, args[3], args[4], args[5]]),
                        lit("guideline_satisfies"@, seq![args[4], args[5], args[6], args[7]]),
                    ],
                ),
            }
        } else if name == ascii("$guideline_measure"@) && args.len() == 4 {
            match resolve(args[0], map, sko) {
                Result::Err(er) => Result::Err(er),
                Result::Ok(r) => Result::Ok(
                    seq![lit("guideline_cardinality"@, seq![ctx, r, args[1], args[2], args[3]])],
                ),
            }
        } else if name == ascii("$guideline_scaled"@) && args.len() == 6 {
            Result::Ok(seq![lit("guideline_satisfies"@, args)])
        } else if name == ascii("$guideline_amount"@) && args.len() == 6 {
            // contract da2 D3: a formula quantity's count as a variable pair
            match resolve(args[0], map, sko) {
                Result::Err(er) => Result::Err(er),
                Result::Ok(r) => Result::Ok(
                    seq![
                        lit("guideline_entity"@, seq![ctx, r, args[1], args[2]]),
                        lit("guideline_cardinality"@, seq![ctx, r, args[3], args[4], args[5]]),
                    ],
                ),
            }
        } else if name == ascii("formula"@) && args.len() == 3 {
            // the formula pass takes every formula it admits out of the items first
            Result::Err(fshape("position"@))
        } else {
            Result::Err(atom("condition_shape"@))
        },
        _ => Result::Err(atom("condition_shape"@)),
    }
}

pub open spec fn pos_all(ts: Seq<Term>) -> Seq<BodyItem> {
    ts.map_values(|t: Term| BodyItem::Pos(t))
}

pub open spec fn all_pos(items: Seq<BodyItem>) -> bool {
    forall|i: int| 0 <= i < items.len() ==> (#[trigger] items[i]) is Pos
}

pub open spec fn pos_terms(items: Seq<BodyItem>) -> Seq<Term> {
    items.map_values(
        |it: BodyItem|
            match it {
                BodyItem::Pos(t) => t,
                BodyItem::Naf(_) => Term::Nil,
            },
    )
}

// Items → body items in order: operator edges, NAF boxes over positive
// payloads, expanded conditions.
pub open spec fn expand(items: Seq<Item>, map: Seq<(Term, Term)>, sko: Seq<(Term, Term)>) -> Result<
    Seq<BodyItem>,
    Term,
>
    decreases items, 1int,
{
    if items.len() == 0 {
        Result::Ok(Seq::empty())
    } else {
        match expand_item(items[0], map, sko) {
            Result::Err(e) => Result::Err(e),
            Result::Ok(head) => match expand(items.drop_first(), map, sko) {
                Result::Err(e) => Result::Err(e),
                Result::Ok(tail) => Result::Ok(head + tail),
            },
        }
    }
}

pub open spec fn expand_item(it: Item, map: Seq<(Term, Term)>, sko: Seq<(Term, Term)>) -> Result<
    Seq<BodyItem>,
    Term,
>
    decreases it, 0int,
{
    match it {
        Item::Op(_, outer, inner, op) => Result::Ok(
            seq![BodyItem::Pos(lit("guideline_operator"@, seq![outer, inner, Term::Atom(op)]))],
        ),
        Item::Naf(_, payload) => match expand(payload, map, sko) {
            Result::Err(e) => Result::Err(e),
            Result::Ok(goals) => if all_pos(goals) {
                Result::Ok(seq![BodyItem::Naf(pos_terms(goals))])
            } else {
                Result::Err(atom("naf_shape"@))
            },
        },
        Item::Anch(ctx, inner) => match condition(ctx, inner, map, sko) {
            Result::Err(e) => Result::Err(e),
            Result::Ok(ts) => Result::Ok(pos_all(ts)),
        },
    }
}

// --- safety (the fork's head/NAF variable laws) ---
pub open spec fn vars_of(t: Term) -> Set<nat> {
    var_stream(t).to_set()
}

pub open spec fn vars_all(ts: Seq<Term>) -> Set<nat> {
    var_stream_all(ts).to_set()
}

pub open spec fn positive_vars(body: Seq<BodyItem>) -> Set<nat> {
    var_stream_all(pos_terms(body.filter(|it: BodyItem| it is Pos))).to_set()
}

// Head variables are bound by positive body goals.
pub open spec fn head_safe(c: DocClause) -> bool {
    vars_of(c.head).subset_of(positive_vars(c.body))
}

pub open spec fn dom_vars(dom: Seq<Term>) -> Set<nat> {
    var_stream_all(dom).to_set()
}

pub open spec fn occ(x: nat, t: Term) -> nat {
    var_stream(t).filter(|k: nat| k == x).len()
}

pub open spec fn occ_all(x: nat, ts: Seq<Term>) -> nat {
    var_stream_all(ts).filter(|k: nat| k == x).len()
}

pub open spec fn clause_occ(x: nat, c: DocClause) -> nat {
    occ(x, c.head) + occ_all(x, pos_terms(c.body.filter(|it: BodyItem| it is Pos))) + naf_occ(
        x,
        c.body,
    )
}

pub open spec fn naf_occ(x: nat, body: Seq<BodyItem>) -> nat
    decreases body.len(),
{
    if body.len() == 0 {
        0
    } else {
        (match body[0] {
            BodyItem::Naf(gs) => occ_all(x, gs),
            BodyItem::Pos(_) => 0nat,
        }) + naf_occ(x, body.drop_first())
    }
}

// Inside a NAF goal every variable is box-local or bound by an earlier
// positive goal; a box-local variable never escapes its NAF goal.
pub open spec fn naf_safe(c: DocClause, doms: Seq<Seq<Term>>, i: nat, bound: Set<nat>) -> bool
    decreases c.body.len() - i,
{
    if i >= c.body.len() {
        true
    } else {
        match c.body[i as int] {
            BodyItem::Pos(t) => naf_safe(c, doms, i + 1, bound.union(vars_of(t))),
            BodyItem::Naf(gs) => {
                let local = dom_vars(doms[i as int]);
                vars_all(gs).subset_of(local.union(bound)) && (forall|x: nat|
                    local.contains(x) ==> #[trigger] clause_occ(x, c) == occ_all(x, gs))
                    && naf_safe(c, doms, i + 1, bound)
            },
        }
    }
}

// The NAF box domains, positionally parallel to the body items (Pos → empty).
pub open spec fn body_doms(items: Seq<Item>) -> Seq<Seq<Term>>
    decreases items,
{
    if items.len() == 0 {
        Seq::empty()
    } else {
        (match items[0] {
            Item::Naf(ndom, _) => seq![ndom],
            Item::Op(_, _, _, _) => seq![Seq::<Term>::empty()],
            Item::Anch(_, inner) => Seq::new(expanded_len(inner), |i: int| Seq::<Term>::empty()),
        }) + body_doms(items.drop_first())
    }
}

// Literals one condition expands to (object 2, predicate 1 + participants, others 1).
pub open spec fn expanded_len(inner: Term) -> nat {
    match inner {
        Term::Comp(name, args) => if name == ascii("object"@) && args.len() == 6 {
            2
        } else if name == ascii("$guideline_bound"@) && args.len() == 8 {
            3
        } else if name == ascii("$guideline_amount"@) && args.len() == 6 {
            2
        } else if name == ascii("predicate"@) && args.len() >= 3 {
            (args.len() - 1) as nat
        } else {
            1
        },
        _ => 1,
    }
}

// --- temporal annotations (contracts m7t D4/D5, q12, q13; c1 D6: with a table) ---
// One scope = one flat item list; a NAF payload is its own scope. A pattern's
// conditions share their context term (list nesting is irrelevant: an
// upper-bound determiner groups its conditions into a sub-list). The
// annotation = an extra condition right after its pp, reserved-named so only
// this pass mints one (`project` rejects any reserved name in the DRS).
pub open spec fn assoc(rows: Seq<(Seq<u8>, Seq<u8>)>, k: Seq<u8>) -> Option<Seq<u8>>
    decreases rows.len(),
{
    if rows.len() == 0 {
        Option::None
    } else if rows[0].0 == k {
        Option::Some(rows[0].1)
    } else {
        assoc(rows.drop_first(), k)
    }
}

// The first same-context `object(v, …)` condition.
pub open spec fn obj_of(items: Seq<Item>, ctx: Term, v: Term) -> Option<Term>
    decreases items.len(),
{
    if items.len() == 0 {
        Option::None
    } else {
        match items[0] {
            Item::Anch(c, inner) => if c == ctx && is_comp(inner, "object"@, 6) && arg(inner, 0)
                == v {
                Option::Some(inner)
            } else {
                obj_of(items.drop_first(), ctx, v)
            },
            _ => obj_of(items.drop_first(), ctx, v),
        }
    }
}

// The targets y of v's same-context `relation(v, of, y)` conditions, in order.
pub open spec fn of_links(items: Seq<Item>, ctx: Term, v: Term) -> Seq<Term>
    decreases items.len(),
{
    if items.len() == 0 {
        Seq::empty()
    } else {
        (match items[0] {
            Item::Anch(c, inner) => if c == ctx && is_comp(inner, "relation"@, 3) && arg(inner, 0)
                == v && arg(inner, 1) == atom("of"@) {
                seq![arg(inner, 2)]
            } else {
                Seq::empty()
            },
            _ => Seq::empty(),
        }) + of_links(items.drop_first(), ctx, v)
    }
}

// An object's noun lemma (empty = none: table lemmas are nonempty).
pub open spec fn noun_of(o: Term) -> Seq<u8> {
    match arg(o, 1) {
        Term::Atom(l) => l,
        _ => Seq::empty(),
    }
}

// v as a time quantity: (unit id, its object) when the noun is a unit lemma.
pub open spec fn qty(tab: Vocabulary, items: Seq<Item>, ctx: Term, v: Term) -> Option<
    (Seq<u8>, Term),
> {
    match obj_of(items, ctx, v) {
        Option::Some(o) => match assoc(tab.units, noun_of(o)) {
            Option::Some(u) => Option::Some((u, o)),
            Option::None => Option::None,
        },
        Option::None => Option::None,
    }
}

pub open spec fn is_frame(tab: Vocabulary, items: Seq<Item>, ctx: Term, v: Term) -> bool {
    match obj_of(items, ctx, v) {
        Option::Some(o) => frames(tab).contains(noun_of(o)),
        Option::None => false,
    }
}

pub open spec fn shape(why: Seq<char>) -> Term {
    Term::Comp(ascii("temporal_shape"@), seq![atom(why)])
}

// A quantity's bound: a count noun with a comparison and a count ≥ 1.
pub open spec fn bound_why(o: Term) -> Option<Term> {
    if !(arg(o, 2) == atom("countable"@) && arg(o, 3) == atom("na"@) && arg(o, 4) != atom("na"@)) {
        Option::Some(shape("no_bound"@))
    } else {
        match arg(o, 5) {
            Term::Int(n) => if n >= 1 {
                Option::None
            } else {
                Option::Some(shape("zero_bound"@))
            },
            _ => Option::Some(shape("no_bound"@)),
        }
    }
}

pub ghost enum Ann {
    Interval(Term, Seq<u8>, Term, Seq<u8>, Term),  // event, role, quantity, unit, anchor | none
    Recurrence(Term, Term, Term, Seq<u8>),  // event, frame, quantity, unit
    Window(Term, Term, Term, Seq<u8>, Term),  // event, frame, length, unit, anchor
    Frequency(Term, Term, Term, Seq<u8>),  // event, counted, period, unit
    Order(Term, Seq<u8>, Term),  // event, relation, anchor
    Range(Term, Term, Seq<u8>),  // q13: event, high end, unit
}

// --- referent classification over the document DRS (contract q12 D3) ---
// The annotation environment: the table + the declaration of every referent of
// the document (or question) DRS, in preorder. APE introduces each referent by
// one `object/6` condition, so variable identity finds its declaration in any
// box or sentence.
pub ghost struct Decl {
    pub var: nat,
    pub noun: Seq<u8>,
    pub bounded:
        bool,  // countable, unit na, a comparison, count ≥ 1 (bound_why None)
}

pub ghost struct Env {
    pub tab: Vocabulary,
    pub decls: Seq<Decl>,
}

pub open spec fn objects(t: Term) -> Seq<Term>
    decreases t, 0int,
{
    match t {
        Term::Comp(name, args) => if name == ascii("object"@) && args.len() == 6 {
            seq![t]
        } else {
            objects_all(args)
        },
        _ => Seq::empty(),
    }
}

pub open spec fn objects_all(ts: Seq<Term>) -> Seq<Term>
    decreases ts, 1int,
{
    if ts.len() == 0 {
        Seq::empty()
    } else {
        objects(ts[0]) + objects_all(ts.drop_first())
    }
}

// Object conditions with a variable referent, as declarations.
pub open spec fn decls_of(objs: Seq<Term>) -> Seq<Decl>
    decreases objs.len(),
{
    if objs.len() == 0 {
        Seq::empty()
    } else {
        (match arg(objs[0], 0) {
            Term::Var(k) => seq![
                Decl { var: k, noun: noun_of(objs[0]), bounded: bound_why(objs[0]) is None },
            ],
            _ => Seq::empty(),
        }) + decls_of(objs.drop_first())
    }
}

pub open spec fn env_of(tab: Option<Vocabulary>, drs: Term) -> Option<Env> {
    match tab {
        Option::Some(t) => Option::Some(Env { tab: t, decls: decls_of(objects(drs)) }),
        Option::None => Option::None,
    }
}

// The first declaration of variable k.
pub open spec fn decl(ds: Seq<Decl>, k: nat) -> Option<Decl>
    decreases ds.len(),
{
    if ds.len() == 0 {
        Option::None
    } else if ds[0].var == k {
        Option::Some(ds[0])
    } else {
        decl(ds.drop_first(), k)
    }
}

// A plain referent: a DRS variable whose noun is neither a unit lemma nor a
// frame noun.
pub open spec fn plain(env: Env, v: Term) -> bool {
    match v {
        Term::Var(k) => match decl(env.decls, k) {
            Option::Some(d) => d.noun.len() > 0 && assoc(env.tab.units, d.noun) is None && !frames(
                env.tab,
            ).contains(d.noun),
            Option::None => false,
        },
        _ => false,
    }
}

pub open spec fn frame_shaped(o: Term) -> bool {
    arg(o, 2) == atom("countable"@) && arg(o, 3) == atom("na"@) && arg(o, 4) == atom("eq"@) && arg(
        o,
        5,
    ) == Term::Int(1)
}

// A same-context `property(x, A, pos)` with A in `lemmas` (an approximation lemma, q13 D3;
// a preference adjective, s1 D4).
pub open spec fn pos_property(lemmas: Seq<Seq<u8>>, items: Seq<Item>, ctx: Term, x: Term) -> bool
    decreases items.len(),
{
    if items.len() == 0 {
        false
    } else {
        (match items[0] {
            Item::Anch(c, inner) => c == ctx && is_comp(inner, "property"@, 3) && arg(inner, 0) == x
                && arg(inner, 2) == atom("pos"@) && lemmas.contains(name_of(arg(inner, 1))),
            _ => false,
        }) || pos_property(lemmas, items.drop_first(), ctx, x)
    }
}

pub open spec fn approximate(tab: Vocabulary, items: Seq<Item>, ctx: Term, r: Term) -> bool {
    pos_property(tab.approximations, items, ctx, r)
}

// D3: a claimed quantity q (object o) marked approximate on itself or on its
// frame f (f = q without a frame) needs the comparison eq.
pub open spec fn approx_why(
    tab: Vocabulary,
    items: Seq<Item>,
    ctx: Term,
    q: Term,
    f: Term,
    o: Term,
) -> Option<Term> {
    if (approximate(tab, items, ctx, q) || approximate(tab, items, ctx, f)) && arg(o, 4) != atom(
        "eq"@,
    ) {
        Option::Some(shape("approximate_bound"@))
    } else {
        Option::None
    }
}

pub open spec fn name_of(t: Term) -> Seq<u8> {
    match t {
        Term::Atom(a) => a,
        _ => Seq::empty(),
    }
}

// E's first same-context `predicate/4|5` condition.
pub open spec fn pred_of(items: Seq<Item>, ctx: Term, e: Term) -> Option<Term>
    decreases items.len(),
{
    if items.len() == 0 {
        Option::None
    } else {
        match items[0] {
            Item::Anch(c, inner) => if c == ctx && (is_comp(inner, "predicate"@, 4) || is_comp(
                inner,
                "predicate"@,
                5,
            )) && arg(inner, 0) == e {
                Option::Some(inner)
            } else {
                pred_of(items.drop_first(), ctx, e)
            },
            _ => pred_of(items.drop_first(), ctx, e),
        }
    }
}

// D5: the counted item = E's second participant (the predicate's 4th
// argument), a plain referent with a bound.
pub open spec fn counted(env: Env, items: Seq<Item>, ctx: Term, e: Term) -> Option<Term> {
    match pred_of(items, ctx, e) {
        Option::Some(pr) => match arg(pr, 3) {
            Term::Var(k) => match decl(env.decls, k) {
                Option::Some(d) => if plain(env, arg(pr, 3)) && d.bounded {
                    Option::Some(arg(pr, 3))
                } else {
                    Option::None
                },
                Option::None => Option::None,
            },
            _ => Option::None,
        },
        Option::None => Option::None,
    }
}

// D6: the same-context window pps on event e among `rest` (scope = the whole scope).
pub open spec fn window_pps(
    tab: Vocabulary,
    scope: Seq<Item>,
    rest: Seq<Item>,
    ctx: Term,
    e: Term,
) -> nat
    decreases rest.len(),
{
    if rest.len() == 0 {
        0
    } else {
        (match rest[0] {
            Item::Anch(c, inner) => if c == ctx && is_comp(inner, "modifier_pp"@, 3) && arg(
                inner,
                0,
            ) == e && match obj_of(scope, ctx, arg(inner, 2)) {
                Option::Some(o) => tab.windows.contains((name_of(arg(inner, 1)), noun_of(o))),
                Option::None => false,
            } {
                1nat
            } else {
                0nat
            },
            _ => 0nat,
        }) + window_pps(tab, scope, rest.drop_first(), ctx, e)
    }
}

// D6: `modifier_pp(E, P, F)` with (P, noun F) a window pair: the only window pp on
// E, F frame-shaped, F of L (a time quantity), L of A (a plain referent).
pub open spec fn window_ann(env: Env, items: Seq<Item>, ctx: Term, pp: Term, o: Term) -> Result<
    Option<Ann>,
    Term,
> {
    let f = arg(pp, 2);
    let fl = of_links(items, ctx, f);
    if window_pps(env.tab, items, items, ctx, arg(pp, 0)) > 1 || !frame_shaped(o) || fl.len() != 1 {
        Result::Err(shape("window_shape"@))
    } else {
        match qty(env.tab, items, ctx, fl[0]) {
            Option::None => Result::Err(shape("window_shape"@)),
            Option::Some((unit, lo)) => if bound_why(lo) is Some {
                Result::Err(bound_why(lo).unwrap())
            } else if approx_why(env.tab, items, ctx, fl[0], f, lo) is Some {
                Result::Err(approx_why(env.tab, items, ctx, fl[0], f, lo).unwrap())
            } else {
                let al = of_links(items, ctx, fl[0]);
                if al.len() != 1 || !plain(env, al[0]) {
                    Result::Err(shape("window_shape"@))
                } else {
                    Result::Ok(Option::Some(Ann::Window(arg(pp, 0), f, fl[0], unit, al[0])))
                }
            },
        }
    }
}

// D5: `modifier_pp(E, P, W)` with P a frequency lemma and W a time quantity.
pub open spec fn frequency_ann(env: Env, items: Seq<Item>, ctx: Term, pp: Term) -> Result<
    Option<Ann>,
    Term,
> {
    let w = arg(pp, 2);
    match qty(env.tab, items, ctx, w) {
        Option::None => Result::Ok(Option::None),
        Option::Some((unit, wo)) => if bound_why(wo) is Some {
            Result::Err(bound_why(wo).unwrap())
        } else if arg(wo, 4) != atom("eq"@) || of_links(items, ctx, w).len() > 0 {
            Result::Err(shape("frequency_shape"@))
        } else {
            match counted(env, items, ctx, arg(pp, 0)) {
                Option::None => Result::Err(shape("frequency_shape"@)),
                Option::Some(c) => Result::Ok(Option::Some(Ann::Frequency(arg(pp, 0), c, w, unit))),
            }
        },
    }
}

// q13 D4: `modifier_pp(E, P, H)` with P a range lemma and H an exact, unanchored,
// unapproximated time quantity; its partner minimum pairs after the pass.
pub open spec fn range_ann(env: Env, items: Seq<Item>, ctx: Term, pp: Term) -> Result<
    Option<Ann>,
    Term,
> {
    let h = arg(pp, 2);
    match qty(env.tab, items, ctx, h) {
        Option::None => Result::Ok(Option::None),
        Option::Some((unit, ho)) => if bound_why(ho) is Some {
            Result::Err(bound_why(ho).unwrap())
        } else if arg(ho, 4) != atom("eq"@) || of_links(items, ctx, h).len() > 0 || approximate(
            env.tab,
            items,
            ctx,
            h,
        ) {
            Result::Err(shape("range_shape"@))
        } else {
            Result::Ok(Option::Some(Ann::Range(arg(pp, 0), h, unit)))
        },
    }
}

// The q12 patterns of a pp the m7t patterns leave plain: window, frequency, range,
// then order (q12 D4: a before|after relation over a plain referent).
pub open spec fn q12_ann(env: Env, items: Seq<Item>, ctx: Term, pp: Term) -> Result<
    Option<Ann>,
    Term,
> {
    let tab = env.tab;
    let p = name_of(arg(pp, 1));
    let x = arg(pp, 2);
    let role = assoc(tab.relations, p);
    match obj_of(items, ctx, x) {
        Option::Some(o) => if tab.windows.contains((p, noun_of(o))) {
            window_ann(env, items, ctx, pp, o)
        } else if tab.frequencies.contains(p) {
            frequency_ann(env, items, ctx, pp)
        } else if tab.ranges.contains(p) && qty(tab, items, ctx, x) is Some {
            range_ann(env, items, ctx, pp)
        } else if (role == Option::Some(ascii("before"@)) || role == Option::Some(ascii("after"@)))
            && plain(env, x) {
            Result::Ok(Option::Some(Ann::Order(arg(pp, 0), role.unwrap(), x)))
        } else {
            Result::Ok(Option::None)
        },
        Option::None => if (role == Option::Some(ascii("before"@)) || role == Option::Some(
            ascii("after"@),
        )) && plain(env, x) {
            Result::Ok(Option::Some(Ann::Order(arg(pp, 0), role.unwrap(), x)))
        } else {
            Result::Ok(Option::None)
        },
    }
}

// The annotation of one `modifier_pp(E, P, X)` in its scope; Ok(None) = a plain pp.
pub open spec fn pp_ann(env: Env, items: Seq<Item>, ctx: Term, pp: Term) -> Result<
    Option<Ann>,
    Term,
> {
    let tab = env.tab;
    let p = match arg(pp, 1) {
        Term::Atom(a) => a,
        _ => Seq::empty(),
    };
    let x = arg(pp, 2);
    let links = of_links(items, ctx, x);
    match (assoc(tab.relations, p), qty(tab, items, ctx, x)) {
        (Option::Some(role), Option::Some((unit, o))) => if bound_why(o) is Some {
            Result::Err(bound_why(o).unwrap())
        } else if approx_why(tab, items, ctx, x, x, o) is Some {
            Result::Err(approx_why(tab, items, ctx, x, x, o).unwrap())
        } else if links.len() > 1 {
            Result::Err(shape("anchor_count"@))
        } else if links.len() == 0 {
            Result::Ok(Option::Some(Ann::Interval(arg(pp, 0), role, x, unit, atom("none"@))))
        } else if role == ascii("duration"@) {
            Result::Err(shape("duration_anchor"@))
        } else if !(links[0] is Var) || qty(tab, items, ctx, links[0]) is Some || is_frame(
            tab,
            items,
            ctx,
            links[0],
        ) {
            Result::Err(shape("anchor_shape"@))
        } else {
            Result::Ok(Option::Some(Ann::Interval(arg(pp, 0), role, x, unit, links[0])))
        },
        _ => match obj_of(items, ctx, x) {
            Option::Some(f) => if !tab.spacings.contains((p, noun_of(f))) {
                q12_ann(env, items, ctx, pp)
            } else if !(arg(f, 2) == atom("countable"@) && arg(f, 3) == atom("na"@) && arg(f, 4)
                == atom("eq"@) && arg(f, 5) == Term::Int(1)) {
                Result::Err(shape("frame_shape"@))
            } else if links.len() != 1 {
                Result::Err(shape("anchor_count"@))
            } else {
                match qty(tab, items, ctx, links[0]) {
                    Option::None => Result::Err(shape("frame_shape"@)),
                    Option::Some((unit, o)) => if bound_why(o) is Some {
                        Result::Err(bound_why(o).unwrap())
                    } else if approx_why(tab, items, ctx, links[0], x, o) is Some {
                        Result::Err(approx_why(tab, items, ctx, links[0], x, o).unwrap())
                    } else if of_links(items, ctx, links[0]).len() > 0 {
                        Result::Err(shape("anchor_count"@))
                    } else {
                        Result::Ok(Option::Some(Ann::Recurrence(arg(pp, 0), x, links[0], unit)))
                    },
                }
            },
            Option::None => q12_ann(env, items, ctx, pp),
        },
    }
}

// The scope's annotations in item order (context, annotation), or the first violation.
pub open spec fn anns(env: Env, scope: Seq<Item>, items: Seq<Item>) -> Result<
    Seq<(Term, Ann)>,
    Term,
>
    decreases items.len(),
{
    if items.len() == 0 {
        Result::Ok(Seq::empty())
    } else {
        let head = match items[0] {
            Item::Anch(c, inner) => if is_comp(inner, "modifier_pp"@, 3) {
                match pp_ann(env, scope, c, inner) {
                    Result::Err(e) => Result::Err(e),
                    Result::Ok(Option::Some(a)) => Result::Ok(seq![(c, a)]),
                    Result::Ok(Option::None) => Result::Ok(Seq::empty()),
                }
            } else {
                Result::Ok(Seq::empty())
            },
            _ => Result::Ok(Seq::empty()),
        };
        match (head, anns(env, scope, items.drop_first())) {
            (Result::Err(e), _) => Result::Err(e),
            (_, Result::Err(e)) => Result::Err(e),
            (Result::Ok(h), Result::Ok(t)) => Result::Ok(h + t),
        }
    }
}

// Quantities and frames the annotations claim; a claim is exclusive.
pub open spec fn claims(ms: Seq<(Term, Ann)>) -> Seq<(Term, Term)> {
    ms.map_values(
        |m: (Term, Ann)|
            match m.1 {
                Ann::Interval(_, _, q, _, _) => seq![(m.0, q)],
                Ann::Recurrence(_, f, q, _) => seq![(m.0, f), (m.0, q)],
                Ann::Window(_, f, l, _, _) => seq![(m.0, f), (m.0, l)],
                Ann::Frequency(_, c, w, _) => seq![(m.0, c), (m.0, w)],
                Ann::Order(_, _, _) => Seq::empty(),
                Ann::Range(_, h, _) => seq![(m.0, h)],
            },
    ).flatten()
}

// The `of` links the annotations consume: an interval's anchor link, a frame's quantity link.
pub open spec fn consumed(ms: Seq<(Term, Ann)>) -> Seq<(Term, Term)> {
    ms.map_values(
        |m: (Term, Ann)|
            match m.1 {
                Ann::Interval(_, _, q, _, a) => if a is Var {
                    seq![(m.0, q)]
                } else {
                    Seq::empty()
                },
                Ann::Recurrence(_, f, _, _) => seq![(m.0, f)],
                Ann::Window(_, f, l, _, _) => seq![(m.0, f), (m.0, l)],
                _ => Seq::empty(),
            },
    ).flatten()
}

// The scope's window on event e in context c (D6), as (length, unit, anchor).
pub open spec fn window_of(ms: Seq<(Term, Ann)>, c: Term, e: Term) -> Option<(Term, Seq<u8>, Term)>
    decreases ms.len(),
{
    if ms.len() == 0 {
        Option::None
    } else {
        match ms[0].1 {
            Ann::Window(e2, _, l, unit, a) => if ms[0].0 == c && e2 == e {
                Option::Some((l, unit, a))
            } else {
                window_of(ms.drop_first(), c, e)
            },
            _ => window_of(ms.drop_first(), c, e),
        }
    }
}

// q13 D4: the minimums on event e in context c — interval quantities whose
// comparison is geq — as (quantity, unit), in item order.
pub open spec fn partners(scope: Seq<Item>, ms: Seq<(Term, Ann)>, c: Term, e: Term) -> Seq<
    (Term, Seq<u8>),
>
    decreases ms.len(),
{
    if ms.len() == 0 {
        Seq::empty()
    } else {
        (match ms[0].1 {
            Ann::Interval(e2, _, q, unit, _) => if ms[0].0 == c && e2 == e && match obj_of(
                scope,
                c,
                q,
            ) {
                Option::Some(o) => arg(o, 4) == atom("geq"@),
                Option::None => false,
            } {
                seq![(q, unit)]
            } else {
                Seq::empty()
            },
            _ => Seq::empty(),
        }) + partners(scope, ms.drop_first(), c, e)
    }
}

pub open spec fn count_of(scope: Seq<Item>, c: Term, q: Term) -> int {
    match obj_of(scope, c, q) {
        Option::Some(o) => match arg(o, 5) {
            Term::Int(n) => n,
            _ => 0,
        },
        Option::None => 0,
    }
}

// D4: each range's (context, low end) — its one partner minimum, of its unit and
// below its count — in order, or the first range that fails.
pub open spec fn range_lows(scope: Seq<Item>, ms: Seq<(Term, Ann)>, rs: Seq<(Term, Ann)>) -> Result<
    Seq<(Term, Term)>,
    Term,
>
    decreases rs.len(),
{
    if rs.len() == 0 {
        Result::Ok(Seq::empty())
    } else {
        let head = match rs[0].1 {
            Ann::Range(e, h, unit) => {
                let ps = partners(scope, ms, rs[0].0, e);
                if ps.len() != 1 || ps[0].1 != unit || count_of(scope, rs[0].0, ps[0].0)
                    >= count_of(scope, rs[0].0, h) {
                    Result::Err(shape("range_shape"@))
                } else {
                    Result::Ok(seq![(rs[0].0, ps[0].0)])
                }
            },
            _ => Result::Ok(Seq::empty()),
        };
        match (head, range_lows(scope, ms, rs.drop_first())) {
            (Result::Err(e), _) => Result::Err(e),
            (_, Result::Err(e)) => Result::Err(e),
            (Result::Ok(a), Result::Ok(b)) => Result::Ok(a + b),
        }
    }
}

// D3: claimed eq quantities an approximation marks, on the quantity or on its
// frame (a recurrence gap's spacing frame, a window length's window frame).
pub open spec fn eq_approx(tab: Vocabulary, scope: Seq<Item>, c: Term, q: Term, f: Term) -> bool {
    match obj_of(scope, c, q) {
        Option::Some(o) => arg(o, 4) == atom("eq"@) && (approximate(tab, scope, c, q)
            || approximate(tab, scope, c, f)),
        Option::None => false,
    }
}

pub open spec fn abouts(tab: Vocabulary, scope: Seq<Item>, ms: Seq<(Term, Ann)>) -> Seq<
    (Term, Term),
> {
    ms.map_values(
        |m: (Term, Ann)|
            match m.1 {
                Ann::Interval(_, _, q, _, _) => if eq_approx(tab, scope, m.0, q, q) {
                    seq![(m.0, q)]
                } else {
                    Seq::empty()
                },
                Ann::Recurrence(_, f, q, _) => if eq_approx(tab, scope, m.0, q, f) {
                    seq![(m.0, q)]
                } else {
                    Seq::empty()
                },
                Ann::Window(_, f, l, _, _) => if eq_approx(tab, scope, m.0, l, f) {
                    seq![(m.0, l)]
                } else {
                    Seq::empty()
                },
                Ann::Frequency(_, _, w, _) => if eq_approx(tab, scope, m.0, w, w) {
                    seq![(m.0, w)]
                } else {
                    Seq::empty()
                },
                _ => Seq::empty(),
            },
    ).flatten()
}

// The object condition of an approximate quantity, compared `about` in place of `eq`.
pub open spec fn about_object(o: Term) -> Term {
    Term::Comp(
        ascii("object"@),
        seq![arg(o, 0), arg(o, 1), arg(o, 2), arg(o, 3), atom("about"@), arg(o, 5)],
    )
}

// The reserved item an annotation adds after its pp; a recurrence under a
// window becomes a scoped recurrence; a window adds none of its own; a range
// names its partner minimum.
pub open spec fn ann_inner(scope: Seq<Item>, a: Ann, ms: Seq<(Term, Ann)>, c: Term) -> Option<
    Term,
> {
    match a {
        Ann::Interval(e, role, q, unit, anchor) => Option::Some(
            Term::Comp(
                ascii("$guideline_interval"@),
                seq![e, Term::Atom(role), q, Term::Atom(unit), anchor],
            ),
        ),
        Ann::Recurrence(e, _, q, unit) => match window_of(ms, c, e) {
            Option::None => Option::Some(
                Term::Comp(ascii("$guideline_recurrence"@), seq![e, q, Term::Atom(unit)]),
            ),
            Option::Some((l, lunit, anchor)) => Option::Some(
                Term::Comp(
                    ascii("$guideline_recurrence_window"@),
                    seq![e, q, Term::Atom(unit), anchor, l, Term::Atom(lunit)],
                ),
            ),
        },
        Ann::Window(_, _, _, _, _) => Option::None,
        Ann::Frequency(e, cnt, w, unit) => Option::Some(
            Term::Comp(ascii("$guideline_frequency"@), seq![e, cnt, w, Term::Atom(unit)]),
        ),
        Ann::Order(e, role, anchor) => Option::Some(
            Term::Comp(ascii("$guideline_order"@), seq![e, Term::Atom(role), anchor]),
        ),
        Ann::Range(e, h, _) => {
            let ps = partners(scope, ms, c, e);
            if ps.len() == 1 {
                Option::Some(Term::Comp(ascii("$guideline_range"@), seq![ps[0].0, h]))
            } else {
                Option::None
            }
        },
    }
}

pub open spec fn event_of(a: Ann) -> Term {
    match a {
        Ann::Interval(e, _, _, _, _) => e,
        Ann::Recurrence(e, _, _, _) => e,
        Ann::Window(e, _, _, _, _) => e,
        Ann::Frequency(e, _, _, _) => e,
        Ann::Order(e, _, _) => e,
        Ann::Range(e, _, _) => e,
    }
}

pub open spec fn recurrences_on(ms: Seq<(Term, Ann)>, c: Term, e: Term) -> nat
    decreases ms.len(),
{
    if ms.len() == 0 {
        0
    } else {
        (if ms[0].0 == c && event_of(ms[0].1) == e && ms[0].1 is Recurrence {
            1nat
        } else {
            0nat
        }) + recurrences_on(ms.drop_first(), c, e)
    }
}

// D6: each window's event carries ≥1 recurrence in the same context.
pub open spec fn windows_ok(ms: Seq<(Term, Ann)>, all: Seq<(Term, Ann)>) -> bool
    decreases ms.len(),
{
    ms.len() == 0 || ((match ms[0].1 {
        Ann::Window(e, _, _, _, _) => recurrences_on(all, ms[0].0, e) >= 1,
        _ => true,
    }) && windows_ok(ms.drop_first(), all))
}

pub open spec fn rebuild_item(
    env: Env,
    scope: Seq<Item>,
    ms: Seq<(Term, Ann)>,
    links: Seq<(Term, Term)>,
    approx: Seq<(Term, Term)>,
    it: Item,
) -> Result<Seq<Item>, Term>
    decreases it, 0int,
{
    match it {
        Item::Anch(c, inner) => if is_comp(inner, "modifier_pp"@, 3) {
            match pp_ann(env, scope, c, inner) {
                Result::Ok(Option::Some(a)) => match ann_inner(scope, a, ms, c) {
                    Option::Some(t) => Result::Ok(seq![it, Item::Anch(c, t)]),
                    Option::None => Result::Ok(seq![it]),
                },
                _ => Result::Ok(seq![it]),
            }
        } else if is_comp(inner, "relation"@, 3) && arg(inner, 1) == atom("of"@) && links.contains(
            (c, arg(inner, 0)),
        ) {
            Result::Ok(Seq::empty())
        } else if is_comp(inner, "object"@, 6) {
            let o = if approx.contains((c, arg(inner, 0))) {
                about_object(inner)
            } else {
                inner
            };
            // nt1 D2: a time quantity carries its unit id beside its cardinality
            match assoc(env.tab.units, noun_of(inner)) {
                Option::Some(u) => Result::Ok(
                    seq![
                        Item::Anch(c, o),
                        Item::Anch(
                            c,
                            Term::Comp(
                                ascii("$guideline_unit"@),
                                seq![arg(inner, 0), Term::Atom(u)],
                            ),
                        ),
                    ],
                ),
                Option::None => Result::Ok(seq![Item::Anch(c, o)]),
            }
        } else {
            Result::Ok(seq![it])
        },
        Item::Naf(dom, payload) => match annotate(env, payload) {
            Result::Err(e) => Result::Err(e),
            Result::Ok(p) => Result::Ok(seq![Item::Naf(dom, p)]),
        },
        Item::Op(_, _, _, _) => Result::Ok(seq![it]),
    }
}

pub open spec fn rebuild(
    env: Env,
    scope: Seq<Item>,
    ms: Seq<(Term, Ann)>,
    links: Seq<(Term, Term)>,
    approx: Seq<(Term, Term)>,
    items: Seq<Item>,
) -> Result<Seq<Item>, Term>
    decreases items, 1int,
{
    if items.len() == 0 {
        Result::Ok(Seq::empty())
    } else {
        match (
            rebuild_item(env, scope, ms, links, approx, items[0]),
            rebuild(env, scope, ms, links, approx, items.drop_first()),
        ) {
            (Result::Err(e), _) => Result::Err(e),
            (_, Result::Err(e)) => Result::Err(e),
            (Result::Ok(h), Result::Ok(t)) => Result::Ok(h + t),
        }
    }
}

// One scope annotated: ranges paired with one minimum each, exclusive claims,
// windows over recurrences, consumed `of` links dropped (any other `relation/3`
// stays and rejects in `condition`), approximate quantities compared `about`.
pub open spec fn annotate(env: Env, items: Seq<Item>) -> Result<Seq<Item>, Term>
    decreases items, 2int,
{
    match anns(env, items, items) {
        Result::Err(e) => Result::Err(e),
        Result::Ok(ms) => match range_lows(items, ms, ms) {
            Result::Err(e) => Result::Err(e),
            Result::Ok(lows) => if !lows.no_duplicates() {
                Result::Err(shape("range_shape"@))
            } else if !claims(ms).no_duplicates() {
                Result::Err(shape("shared_quantity"@))
            } else if !windows_ok(ms, ms) {
                Result::Err(shape("window_shape"@))
            } else {
                rebuild(env, items, ms, consumed(ms), abouts(env.tab, items, ms), items)
            },
        },
    }
}

// Without a table the items stay as flattened.
pub open spec fn annotated(tab: Option<Env>, items: Seq<Item>) -> Result<Seq<Item>, Term> {
    match tab {
        Option::None => Result::Ok(items),
        Option::Some(t) => annotate(t, items),
    }
}

pub open spec fn flatten_ann(
    tab: Option<Env>,
    l: Term,
    w: Where,
    s: nat,
    docid: Seq<u8>,
    deps: Term,
    outer: Term,
    encl: Encl,
    n: nat,
    base: nat,
) -> Result<Flat, Term> {
    match flatten_list(l, w, s, docid, deps, outer, encl, n, base) {
        Result::Err(e) => Result::Err(e),
        Result::Ok(f) => match annotated(tab, f.items) {
            Result::Err(e) => Result::Err(e),
            Result::Ok(items) => Result::Ok(Flat { items, n: f.n }),
        },
    }
}

// --- strength annotations (contracts s1 D3/D4, s2 D3: documents with a table) ---
// After a scope's temporal pass: a grade right after its root-fact copula; a
// preference right after its alternative pp or, without one, after its
// copula; a qualifier in place of its adverb, or right after its copula.
// Endpoints resolve document-wide (`plain`); questions never run it.
pub open spec fn s1_shape(kind: Seq<char>, why: Seq<char>) -> Term {
    Term::Comp(ascii(kind), seq![atom(why)])
}

pub open spec fn is_pred(t: Term) -> bool {
    is_comp(t, "predicate"@, 3) || is_comp(t, "predicate"@, 4) || is_comp(t, "predicate"@, 5)
}

// E's first same-context `predicate/3..5` condition.
pub open spec fn event_pred(items: Seq<Item>, ctx: Term, e: Term) -> Option<Term>
    decreases items.len(),
{
    if items.len() == 0 {
        Option::None
    } else {
        match items[0] {
            Item::Anch(c, inner) => if c == ctx && is_pred(inner) && arg(inner, 0) == e {
                Option::Some(inner)
            } else {
                event_pred(items.drop_first(), ctx, e)
            },
            _ => event_pred(items.drop_first(), ctx, e),
        }
    }
}

pub open spec fn preferred_prop(tab: Vocabulary, items: Seq<Item>, ctx: Term, p: Term) -> bool {
    pos_property(tab.preferences, items, ctx, p)
}

// The objects of E's same-context alternative pps, in item order.
pub open spec fn alternatives_of(tab: Vocabulary, items: Seq<Item>, ctx: Term, e: Term) -> Seq<Term>
    decreases items.len(),
{
    if items.len() == 0 {
        Seq::empty()
    } else {
        (match items[0] {
            Item::Anch(c, inner) => if c == ctx && is_comp(inner, "modifier_pp"@, 3) && arg(
                inner,
                0,
            ) == e && tab.alternatives.contains(name_of(arg(inner, 1))) {
                seq![arg(inner, 2)]
            } else {
                Seq::empty()
            },
            _ => Seq::empty(),
        }) + alternatives_of(tab, items.drop_first(), ctx, e)
    }
}

// D4: (preferred, alternative | none) of the event whose predicate is `pred`;
// none where no pattern applies. A preference copula `be(x, p)` (p a
// preference property) prefers x; any other event with one alternative pp
// prefers its second participant.
pub open spec fn preference_of(env: Env, items: Seq<Item>, ctx: Term, pred: Term) -> Result<
    Option<(Term, Term)>,
    Term,
> {
    let alts = alternatives_of(env.tab, items, ctx, arg(pred, 0));
    let copula = is_comp(pred, "predicate"@, 4) && arg(pred, 1) == atom("be"@) && preferred_prop(
        env.tab,
        items,
        ctx,
        arg(pred, 3),
    );
    if alts.len() > 1 {
        Result::Err(s1_shape("preference_shape"@, "alternative_count"@))
    } else if !copula && alts.len() == 0 {
        Result::Ok(Option::None)
    } else if !copula && is_comp(pred, "predicate"@, 3) {
        Result::Err(s1_shape("preference_shape"@, "no_preferred"@))
    } else {
        let p = if copula {
            arg(pred, 2)
        } else {
            arg(pred, 3)
        };
        let a = if alts.len() == 1 {
            alts[0]
        } else {
            atom("none"@)
        };
        if !plain(env, p) || (alts.len() == 1 && !plain(env, a)) {
            Result::Err(s1_shape("preference_shape"@, "endpoint"@))
        } else {
            Result::Ok(Option::Some((p, a)))
        }
    }
}

// D3: (dimension, value) of a root-fact copula `be(r, l)` in context actual
// whose complement's noun is a grade noun.
pub open spec fn grade_of(env: Env, ctx: Term, pred: Term) -> Option<(Seq<u8>, Seq<u8>)> {
    if ctx == actual() && is_comp(pred, "predicate"@, 4) && arg(pred, 1) == atom("be"@) && plain(
        env,
        arg(pred, 2),
    ) {
        match arg(pred, 3) {
            Term::Var(k) => match decl(env.decls, k) {
                Option::Some(d) => match assoc(env.tab.strengths, d.noun) {
                    Option::Some(v) => Option::Some((ascii("strength"@), v)),
                    Option::None => match assoc(env.tab.evidences, d.noun) {
                        Option::Some(v) => Option::Some((ascii("evidence"@), v)),
                        Option::None => Option::None,
                    },
                },
                Option::None => Option::None,
            },
            _ => Option::None,
        }
    } else {
        Option::None
    }
}

pub open spec fn preference_item(c: Term, e: Term, pa: (Term, Term)) -> Item {
    Item::Anch(c, Term::Comp(ascii("$guideline_preference"@), seq![e, pa.0, pa.1]))
}

pub open spec fn qualifier_item(c: Term, e: Term, k: Seq<u8>, word: Term) -> Item {
    Item::Anch(c, Term::Comp(ascii("$guideline_qualifier"@), seq![e, Term::Atom(k), word]))
}

// s2 D3.7: the classes whose adverb weakens what its clause asserts.
pub open spec fn attenuating(k: Seq<u8>) -> bool {
    ||| k == ascii("usuality"@)
    ||| k == ascii("ideality"@)
    ||| k == ascii("possibility"@)
    ||| k == ascii("applicability"@)
}

// s2 D3.7: c lies under a modal operator (should, must, can, may) on its path
// from `actual`; classical negations alone leave it asserted.
pub open spec fn edge_of(items: Seq<Item>, c: Term) -> Option<(Term, Seq<u8>)>
    decreases items.len(),
{
    if items.len() == 0 {
        Option::None
    } else {
        match items[0] {
            Item::Op(_, outer, inner, op) => if inner == c {
                Option::Some((outer, op))
            } else {
                edge_of(items.drop_first(), c)
            },
            _ => edge_of(items.drop_first(), c),
        }
    }
}

pub open spec fn under_modal(scope: Seq<Item>, c: Term, fuel: nat) -> bool
    decreases fuel,
{
    if fuel == 0 || c == actual() {
        false
    } else {
        match edge_of(scope, c) {
            Option::Some((outer, op)) => is_modal(op) || under_modal(
                scope,
                outer,
                (fuel - 1) as nat,
            ),
            Option::None => false,
        }
    }
}

// s2 D3.2: a copula complement that names no thing (no `object` declaration).
pub open spec fn property_only(env: Env, v: Term) -> bool {
    match v {
        Term::Var(k) => decl(env.decls, k) is None,
        _ => false,
    }
}

// s2 D3.2: one qualifier per same-context `property(p, A, pos)` of a copula
// `be(x, p)` whose A is a qualifier lemma, in item order.
pub open spec fn copula_qualifiers(
    tab: Vocabulary,
    items: Seq<Item>,
    ctx: Term,
    e: Term,
    p: Term,
) -> Seq<Item>
    decreases items.len(),
{
    if items.len() == 0 {
        Seq::empty()
    } else {
        (match items[0] {
            Item::Anch(c, inner) => if c == ctx && is_comp(inner, "property"@, 3) && arg(inner, 0)
                == p && arg(inner, 2) == atom("pos"@) {
                match assoc(tab.qualifiers, name_of(arg(inner, 1))) {
                    Option::Some(k) => seq![qualifier_item(ctx, e, k, arg(inner, 1))],
                    Option::None => Seq::empty(),
                }
            } else {
                Seq::empty()
            },
            _ => Seq::empty(),
        }) + copula_qualifiers(tab, items.drop_first(), ctx, e, p)
    }
}

// One item and the annotations that follow it (s2: an adverb item becomes its
// qualifier). A violation reports at its item. `w` = the scope's place: a grade
// comes from a root scope alone; a head outside the antecedent with no modal
// operator on its context path is asserted (s2 D3.7).
pub open spec fn strength_item(env: Env, w: Where, scope: Seq<Item>, it: Item) -> Result<
    Seq<Item>,
    Term,
>
    decreases it, 0int,
{
    match it {
        Item::Anch(c, inner) => if is_pred(inner) {
            let gs = match (if w is Root {
                grade_of(env, c, inner)
            } else {
                Option::None
            }) {
                Option::Some((dim, v)) => seq![
                    Item::Anch(
                        c,
                        Term::Comp(
                            ascii("$guideline_grade"@),
                            seq![arg(inner, 2), Term::Atom(dim), Term::Atom(v), arg(inner, 3)],
                        ),
                    ),
                ],
                Option::None => Seq::empty(),
            };
            let qs = if is_comp(inner, "predicate"@, 4) && arg(inner, 1) == atom("be"@)
                && property_only(env, arg(inner, 3)) {
                copula_qualifiers(env.tab, scope, c, arg(inner, 0), arg(inner, 3))
            } else {
                Seq::empty()
            };
            match preference_of(env, scope, c, inner) {
                Result::Err(e) => Result::Err(e),
                Result::Ok(Option::Some(pa)) => if pa.1 == atom("none"@) {
                    Result::Ok(seq![it] + gs + qs + seq![preference_item(c, arg(inner, 0), pa)])
                } else {
                    Result::Ok(seq![it] + gs + qs)
                },
                Result::Ok(Option::None) => Result::Ok(seq![it] + gs + qs),
            }
        } else if is_comp(inner, "modifier_pp"@, 3) && env.tab.alternatives.contains(
            name_of(arg(inner, 1)),
        ) {
            match event_pred(scope, c, arg(inner, 0)) {
                Option::Some(pred) => match preference_of(env, scope, c, pred) {
                    Result::Ok(Option::Some(pa)) => Result::Ok(
                        seq![it, preference_item(c, arg(inner, 0), pa)],
                    ),
                    _ => Result::Ok(seq![it]),
                },
                Option::None => Result::Ok(seq![it]),
            }
        } else if is_comp(inner, "modifier_adv"@, 3) {
            // An unmapped adverb stays; the expansion rejects it.
            match assoc(env.tab.qualifiers, name_of(arg(inner, 1))) {
                Option::None => Result::Ok(seq![it]),
                Option::Some(k) => if arg(inner, 2) != atom("pos"@) {
                    Result::Err(s1_shape("qualifier_shape"@, "degree"@))
                } else if attenuating(k) && !(w is Antecedent) && !under_modal(
                    scope,
                    c,
                    scope.len(),
                ) {
                    Result::Err(s1_shape("qualifier_shape"@, "asserted"@))
                } else {
                    Result::Ok(seq![qualifier_item(c, arg(inner, 0), k, arg(inner, 1))])
                },
            }
        } else {
            Result::Ok(seq![it])
        },
        Item::Naf(dom, payload) => match strength(env, Where::Antecedent, payload) {
            Result::Err(e) => Result::Err(e),
            Result::Ok(p) => Result::Ok(seq![Item::Naf(dom, p)]),
        },
        Item::Op(_, _, _, _) => Result::Ok(seq![it]),
    }
}

pub open spec fn strength_items(env: Env, w: Where, scope: Seq<Item>, items: Seq<Item>) -> Result<
    Seq<Item>,
    Term,
>
    decreases items, 1int,
{
    if items.len() == 0 {
        Result::Ok(Seq::empty())
    } else {
        match (
            strength_item(env, w, scope, items[0]),
            strength_items(env, w, scope, items.drop_first()),
        ) {
            (Result::Err(e), _) => Result::Err(e),
            (_, Result::Err(e)) => Result::Err(e),
            (Result::Ok(h), Result::Ok(t)) => Result::Ok(h + t),
        }
    }
}

pub open spec fn strength(env: Env, w: Where, items: Seq<Item>) -> Result<Seq<Item>, Term>
    decreases items, 2int,
{
    strength_items(env, w, items, items)
}

// A document scope: the temporal pass, then (with a table) the strength pass.
pub open spec fn flatten_doc(
    tab: Option<Env>,
    l: Term,
    w: Where,
    s: nat,
    docid: Seq<u8>,
    deps: Term,
    outer: Term,
    encl: Encl,
    n: nat,
    base: nat,
) -> Result<Flat, Term> {
    match flatten_ann(tab, l, w, s, docid, deps, outer, encl, n, base) {
        Result::Err(e) => Result::Err(e),
        Result::Ok(f) => match tab {
            Option::None => Result::Ok(f),
            Option::Some(env) => match strength(env, w, f.items) {
                Result::Err(e) => Result::Err(e),
                Result::Ok(items) => Result::Ok(Flat { items, n: f.n }),
            },
        },
    }
}

// --- comparison conditions (contract nt1 D3): rule bodies + question goals ---
// A body object whose comparison bounds its count compares the stated count of
// a matching fact with that bound through a comparison built-in. A time
// quantity (the referent D2 gave a unit item) leaves its entity literal out and
// compares across units: its unit and the unit slots of the annotations that
// name it become one variable. The j-th convertible object of the body (scope
// preorder, NAF payloads included) owns the fresh variables cb + 3j, +1, +2,
// above every DRS referent and box context of the sentence.
pub open spec fn convertible(c: Term) -> bool {
    c == atom("exactly"@) || c == atom("geq"@) || c == atom("greater"@) || c == atom("leq"@) || c
        == atom("less"@)
}

pub open spec fn is_object_cmp(inner: Term) -> bool {
    is_comp(inner, "object"@, 6) && convertible(arg(inner, 4))
}

pub open spec fn conv_item(it: Item) -> Seq<Term>
    decreases it, 0int,
{
    match it {
        Item::Anch(_, inner) => if is_object_cmp(inner) {
            seq![arg(inner, 0)]
        } else {
            Seq::empty()
        },
        Item::Naf(_, payload) => conv_refs(payload),
        Item::Op(_, _, _, _) => Seq::empty(),
    }
}

// Referents of the convertible objects, in preorder.
pub open spec fn conv_refs(items: Seq<Item>) -> Seq<Term>
    decreases items, 1int,
{
    if items.len() == 0 {
        Seq::empty()
    } else {
        conv_item(items[0]) + conv_refs(items.drop_first())
    }
}

pub open spec fn ref_pos(refs: Seq<Term>, r: Term) -> nat
    decreases refs.len(),
{
    if refs.len() == 0 || refs[0] == r {
        0
    } else {
        1 + ref_pos(refs.drop_first(), r)
    }
}

pub open spec fn cvar(cb: nat, refs: Seq<Term>, r: Term, i: nat) -> Term {
    Term::Var(cb + 3 * ref_pos(refs, r) + i)
}

pub open spec fn unit_of(scope: Seq<Item>, r: Term) -> Option<Term>
    decreases scope.len(),
{
    if scope.len() == 0 {
        Option::None
    } else {
        match scope[0] {
            Item::Anch(_, inner) => if is_comp(inner, "$guideline_unit"@, 2) && arg(inner, 0) == r {
                Option::Some(arg(inner, 1))
            } else {
                unit_of(scope.drop_first(), r)
            },
            _ => unit_of(scope.drop_first(), r),
        }
    }
}

// The convertible time quantities of one scope (no recursion into payloads).
pub open spec fn scope_times(scope: Seq<Item>, items: Seq<Item>) -> Seq<Term>
    decreases items.len(),
{
    if items.len() == 0 {
        Seq::empty()
    } else {
        (match items[0] {
            Item::Anch(_, inner) => if is_object_cmp(inner) && unit_of(
                scope,
                arg(inner, 0),
            ) is Some {
                seq![arg(inner, 0)]
            } else {
                Seq::empty()
            },
            _ => Seq::empty(),
        }) + scope_times(scope, items.drop_first())
    }
}

pub open spec fn time_item(it: Item) -> Seq<Term>
    decreases it, 0int,
{
    match it {
        Item::Naf(_, payload) => time_refs(payload),
        _ => Seq::empty(),
    }
}

pub open spec fn nested_times(items: Seq<Item>) -> Seq<Term>
    decreases items, 1int,
{
    if items.len() == 0 {
        Seq::empty()
    } else {
        time_item(items[0]) + nested_times(items.drop_first())
    }
}

pub open spec fn time_refs(items: Seq<Item>) -> Seq<Term>
    decreases items, 2int,
{
    scope_times(items, items) + nested_times(items)
}

// The first object of r in a scope.
pub open spec fn object_of(scope: Seq<Item>, r: Term) -> Option<Term>
    decreases scope.len(),
{
    if scope.len() == 0 {
        Option::None
    } else {
        match scope[0] {
            Item::Anch(_, inner) => if is_comp(inner, "object"@, 6) && arg(inner, 0) == r {
                Option::Some(inner)
            } else {
                object_of(scope.drop_first(), r)
            },
            _ => object_of(scope.drop_first(), r),
        }
    }
}

pub open spec fn comp(name: Seq<char>, args: Seq<Term>) -> Term {
    Term::Comp(ascii(name), args)
}

// An annotation naming a converted time quantity carries its unit variable.
pub open spec fn relink(times: Seq<Term>, refs: Seq<Term>, cb: nat, inner: Term) -> Term {
    if is_comp(inner, "$guideline_interval"@, 5) && times.contains(arg(inner, 2)) {
        comp(
            "$guideline_interval"@,
            seq![
                arg(inner, 0),
                arg(inner, 1),
                arg(inner, 2),
                cvar(cb, refs, arg(inner, 2), 2),
                arg(inner, 4),
            ],
        )
    } else if is_comp(inner, "$guideline_recurrence"@, 3) && times.contains(arg(inner, 1)) {
        comp(
            "$guideline_recurrence"@,
            seq![arg(inner, 0), arg(inner, 1), cvar(cb, refs, arg(inner, 1), 2)],
        )
    } else if is_comp(inner, "$guideline_recurrence_window"@, 6) {
        comp(
            "$guideline_recurrence_window"@,
            seq![
                arg(inner, 0),
                arg(inner, 1),
                if times.contains(arg(inner, 1)) {
                    cvar(cb, refs, arg(inner, 1), 2)
                } else {
                    arg(inner, 2)
                },
                arg(inner, 3),
                arg(inner, 4),
                if times.contains(arg(inner, 4)) {
                    cvar(cb, refs, arg(inner, 4), 2)
                } else {
                    arg(inner, 5)
                },
            ],
        )
    } else {
        inner
    }
}

// The fresh variables of a payload's conversions: local to that NAF goal.
pub open spec fn fresh_of(cb: nat, refs: Seq<Term>, rs: Seq<Term>) -> Seq<Term>
    decreases rs.len(),
{
    if rs.len() == 0 {
        Seq::empty()
    } else {
        seq![cvar(cb, refs, rs[0], 0), cvar(cb, refs, rs[0], 1), cvar(cb, refs, rs[0], 2)]
            + fresh_of(cb, refs, rs.drop_first())
    }
}

pub open spec fn compare_item(
    scope: Seq<Item>,
    times: Seq<Term>,
    refs: Seq<Term>,
    cb: nat,
    it: Item,
) -> Seq<Item>
    decreases it, 0int,
{
    match it {
        Item::Anch(c, inner) => if is_object_cmp(inner) {
            let r = arg(inner, 0);
            if times.contains(r) {
                seq![
                    Item::Anch(
                        c,
                        comp(
                            "$guideline_measure"@,
                            seq![r, arg(inner, 3), cvar(cb, refs, r, 0), cvar(cb, refs, r, 1)],
                        ),
                    ),
                ]
            } else {
                seq![
                    Item::Anch(
                        c,
                        comp(
                            "$guideline_bound"@,
                            seq![
                                r,
                                arg(inner, 1),
                                arg(inner, 2),
                                arg(inner, 3),
                                cvar(cb, refs, r, 0),
                                cvar(cb, refs, r, 1),
                                arg(inner, 4),
                                arg(inner, 5),
                            ],
                        ),
                    ),
                ]
            }
        } else if is_comp(inner, "$guideline_unit"@, 2) && times.contains(arg(inner, 0)) {
            let r = arg(inner, 0);
            match object_of(scope, r) {
                Option::Some(o) => seq![
                    Item::Anch(c, comp("$guideline_unit"@, seq![r, cvar(cb, refs, r, 2)])),
                    Item::Anch(
                        c,
                        comp(
                            "$guideline_scaled"@,
                            seq![
                                cvar(cb, refs, r, 0),
                                cvar(cb, refs, r, 1),
                                cvar(cb, refs, r, 2),
                                arg(o, 4),
                                arg(o, 5),
                                arg(inner, 1),
                            ],
                        ),
                    ),
                ],
                Option::None => seq![it],
            }
        } else {
            seq![Item::Anch(c, relink(times, refs, cb, inner))]
        },
        Item::Naf(dom, payload) => seq![
            Item::Naf(
                dom + fresh_of(cb, refs, conv_refs(payload)),
                compare_list(payload, times, refs, cb, payload),
            ),
        ],
        Item::Op(_, _, _, _) => seq![it],
    }
}

pub open spec fn compare_list(
    scope: Seq<Item>,
    times: Seq<Term>,
    refs: Seq<Term>,
    cb: nat,
    items: Seq<Item>,
) -> Seq<Item>
    decreases items, 1int,
{
    if items.len() == 0 {
        Seq::empty()
    } else {
        compare_item(scope, times, refs, cb, items[0]) + compare_list(
            scope,
            times,
            refs,
            cb,
            items.drop_first(),
        )
    }
}

pub open spec fn compared(items: Seq<Item>, cb: nat) -> Seq<Item> {
    compare_list(items, time_refs(items), conv_refs(items), cb, items)
}

pub open spec fn is_builtin_lit(t: Term) -> bool {
    match t {
        Term::Comp(name, args) => crate::compare::is_builtin(name, args.len()),
        _ => false,
    }
}

// --- dose arithmetic formulas (contract da2): rule consequents ---
// A formula `Y = E` at the top level of a rule's final consequent computes
// the count of its target Y from antecedent quantities and literals through
// the arithmetic built-ins (contract da1). The chain follows every antecedent
// goal; its result is the target's cardinality head. For formula quantities
// alone, a unit-noun amount stands for its count and `some` sets no bound
// (APE reads `some` + plural as at least 2). A literal's value comes from the
// numeral bytes of the ACE line: APE's floats lose the source spelling.
// The sentence's APE token list (index 0 = `^`; anchors index it) + its ACE line.
pub ghost struct Src {
    pub toks: Seq<Term>,
    pub line: Seq<u8>,
}

pub open spec fn fshape(why: Seq<char>) -> Term {
    Term::Comp(ascii("formula_shape"@), seq![atom(why)])
}

pub open spec fn is_formula(t: Term) -> bool {
    is_comp(t, "formula"@, 3)
}

// Anchored conditions reachable through carriers, each with its token index.
pub open spec fn anchored(t: Term) -> Seq<(Term, int)>
    decreases t, 0int,
{
    match anchor_s(t) {
        Option::Some(_) => match arg(arg(t, 1), 1) {
            Term::Int(k) => seq![(arg(t, 0), k)] + anchored(arg(t, 0)),
            _ => Seq::empty(),
        },
        Option::None => match t {
            Term::Comp(name, args) => if is_carrier(name, args.len()) {
                anchored_all(args)
            } else {
                Seq::empty()
            },
            _ => Seq::empty(),
        },
    }
}

pub open spec fn anchored_all(ts: Seq<Term>) -> Seq<(Term, int)>
    decreases ts, 1int,
{
    if ts.len() == 0 {
        Seq::empty()
    } else {
        anchored(ts[0]) + anchored_all(ts.drop_first())
    }
}

// The token index of r's object.
pub open spec fn object_token(an: Seq<(Term, int)>, r: Term) -> Option<int>
    decreases an.len(),
{
    if an.len() == 0 {
        Option::None
    } else if is_comp(an[0].0, "object"@, 6) && arg(an[0].0, 0) == r {
        Option::Some(an[0].1)
    } else {
        object_token(an.drop_first(), r)
    }
}

// The token index of the formula f (its `=`).
pub open spec fn formula_token(an: Seq<(Term, int)>, f: Term) -> Option<int>
    decreases an.len(),
{
    if an.len() == 0 {
        Option::None
    } else if an[0].0 == f {
        Option::Some(an[0].1)
    } else {
        formula_token(an.drop_first(), f)
    }
}

// The token before r's noun is `some` (no adjective between).
pub open spec fn some_before(src: Src, an: Seq<(Term, int)>, r: Term) -> bool {
    match object_token(an, r) {
        Option::Some(k) => 1 <= k <= src.toks.len() && src.toks[k - 1] == atom("some"@),
        Option::None => false,
    }
}

// A numeral token's text: an integer's spelling, sign included, or the
// driver's `'$decimal'(Text)` (the float as SWI prints it).
pub open spec fn numeral(t: Term) -> Option<Seq<u8>> {
    match t {
        Term::Int(n) => Option::Some(dec_bytes(n)),
        Term::Comp(name, args) => if name == ascii("$decimal"@) && args.len() == 1 {
            match args[0] {
                Term::Atom(a) => Option::Some(a),
                _ => Option::None,
            }
        } else {
            Option::None
        },
        _ => Option::None,
    }
}

pub open spec fn is_literal(t: Term) -> bool {
    is_comp(t, "int"@, 1) || is_comp(t, "real"@, 1)
}

// A byte that continues a word: an ASCII letter, a digit, `_` or `-`.
pub open spec fn wordy(b: u8) -> bool {
    is_alnum_b(b) || b == 0x2D
}

pub open spec fn digits_end(l: Seq<u8>, i: int) -> int
    decreases l.len() - i,
{
    if 0 <= i < l.len() && is_digit_b(l[i]) {
        digits_end(l, i + 1)
    } else {
        i
    }
}

// The end of the numeral run at i: digits, then `.` + digits when present.
pub open spec fn run_end(l: Seq<u8>, i: int) -> int {
    let j = digits_end(l, i);
    if 0 <= j && j + 1 < l.len() && l[j] == 0x2E && is_digit_b(l[j + 1]) {
        digits_end(l, j + 1)
    } else {
        j
    }
}

// A numeral lexeme starts at i: a digit run that no word byte or `.` precedes
// and no word byte follows (so `X1`, `covid-19` and `2-week` hold none).
pub open spec fn lexeme_at(l: Seq<u8>, i: int) -> bool {
    &&& 0 <= i < l.len() && is_digit_b(l[i])
    &&& i == 0 || !(wordy(l[i - 1]) || l[i - 1] == 0x2E)
    &&& run_end(l, i) == l.len() || !wordy(l[run_end(l, i)])
}

pub open spec fn lexemes_from(l: Seq<u8>, i: int) -> Seq<int>
    decreases l.len() - i,
{
    if i < 0 || i >= l.len() {
        Seq::empty()
    } else if lexeme_at(l, i) {
        seq![i] + lexemes_from(l, i + 1)
    } else {
        lexemes_from(l, i + 1)
    }
}

pub open spec fn lexeme_starts(l: Seq<u8>) -> Seq<int> {
    lexemes_from(l, 0)
}

pub open spec fn numeral_toks(toks: Seq<Term>) -> Seq<Term> {
    toks.filter(|t: Term| numeral(t) is Some)
}

// The numeral tokens spell the line's lexemes, one to one, in order.
pub open spec fn numerals_match(src: Src) -> bool {
    let ns = numeral_toks(src.toks);
    let ls = lexeme_starts(src.line);
    &&& ns.len() == ls.len()
    &&& forall|g: int|
        0 <= g < ns.len() ==> numeral(#[trigger] ns[g]) == Option::Some(
            src.line.subrange(ls[g], run_end(src.line, ls[g])),
        )
}

// A `-` or `+` token right before a numeral token (APE folds it into the value).
pub open spec fn signed(toks: Seq<Term>) -> bool {
    exists|i: int|
        0 <= i && i + 1 < toks.len() && ((#[trigger] toks[i]) == atom("-"@) || toks[i] == atom(
            "+"@,
        )) && numeral(toks[i + 1]) is Some
}

// The referent leaves of an expression, in token order.
pub open spec fn var_leaves(e: Term) -> Seq<Term>
    decreases e,
{
    match e {
        Term::Comp(name, args) => if name == ascii("expr"@) && args.len() == 3 {
            var_leaves(args[1]) + var_leaves(args[2])
        } else {
            Seq::empty()
        },
        Term::Var(_) => seq![e],
        _ => Seq::empty(),
    }
}

// The literal leaves of an expression, in token order.
pub open spec fn lit_leaves(e: Term) -> Seq<Term>
    decreases e,
{
    match e {
        Term::Comp(name, args) => if name == ascii("expr"@) && args.len() == 3 {
            lit_leaves(args[1]) + lit_leaves(args[2])
        } else if is_literal(e) {
            seq![e]
        } else {
            Seq::empty()
        },
        _ => Seq::empty(),
    }
}

// Every node is `*` or `/`; every leaf a referent or a literal.
pub open spec fn ops_ok(e: Term) -> bool
    decreases e,
{
    match e {
        Term::Comp(name, args) => if name == ascii("expr"@) && args.len() == 3 {
            (args[0] == atom("*"@) || args[0] == atom("/"@)) && ops_ok(args[1]) && ops_ok(args[2])
        } else {
            is_literal(e)
        },
        Term::Var(_) => true,
        _ => false,
    }
}

pub open spec fn pow10(n: nat) -> int
    decreases n,
{
    if n == 0 {
        1
    } else {
        10 * pow10((n - 1) as nat)
    }
}

// The exact value of the lexeme at i: `I` or `I.F` = (I·10^|F| + F) / 10^|F|.
pub open spec fn lexeme_value(l: Seq<u8>, i: int) -> Term {
    let j = digits_end(l, i);
    let k = run_end(l, i);
    let whole = crate::check::dec_of(l.subrange(i, j)) as int;
    if k == j {
        Term::Int(whole)
    } else {
        let f = l.subrange(j + 1, k);
        crate::arith::canon(whole * pow10(f.len()) + crate::check::dec_of(f) as int, pow10(f.len()))
    }
}

// A literal lexeme ends the line or stands before a space, `)`, `*`, `/` or
// the final `.` (APE reads `50%` as 50 and drops the `%`).
pub open spec fn literal_end_ok(l: Seq<u8>, k: int) -> bool {
    k == l.len() || (0 <= k < l.len() && (l[k] == 0x20 || l[k] == 0x29 || l[k] == 0x2A || l[k]
        == 0x2F || (l[k] == 0x2E && k + 1 == l.len())))
}

// The values of a formula's literal leaves, in token order: with the numeral
// tokens matched to the line's lexemes, the m-th leaf pairs with numeral g0 + m
// (g0 = the numeral tokens up to the formula's `=` token at kf), spells like
// it, and takes its lexeme's value (contract da2 D2).
pub open spec fn literal_values(src: Src, kf: int, lits: Seq<Term>) -> Option<Seq<Term>> {
    let ns = numeral_toks(src.toks);
    let ls = lexeme_starts(src.line);
    let g0 = numeral_toks(src.toks.take(kf + 1)).len() as int;
    if kf < 0 || kf >= src.toks.len() || !numerals_match(src) || signed(src.toks) || g0 + lits.len()
        > ns.len() {
        Option::None
    } else if forall|m: int|
        0 <= m < lits.len() ==> numeral(arg(#[trigger] lits[m], 0)) == numeral(ns[g0 + m])
            && literal_end_ok(src.line, run_end(src.line, ls[g0 + m])) {
        Option::Some(Seq::new(lits.len(), |m: int| lexeme_value(src.line, ls[g0 + m])))
    } else {
        Option::None
    }
}

// The top-level (context `actual`) object on y.
pub open spec fn top_object(items: Seq<Item>, y: Term) -> Option<Term>
    decreases items.len(),
{
    if items.len() == 0 {
        Option::None
    } else {
        match items[0] {
            Item::Anch(c, inner) => if c == actual() && is_comp(inner, "object"@, 6) && arg(
                inner,
                0,
            ) == y {
                Option::Some(inner)
            } else {
                top_object(items.drop_first(), y)
            },
            _ => top_object(items.drop_first(), y),
        }
    }
}

// The top-level antecedent object or nt1 bound item on r.
pub open spec fn operand_item(items: Seq<Item>, r: Term) -> Option<Term>
    decreases items.len(),
{
    if items.len() == 0 {
        Option::None
    } else {
        match items[0] {
            Item::Anch(c, inner) => if c == actual() && (is_comp(inner, "object"@, 6) || is_comp(
                inner,
                "$guideline_bound"@,
                8,
            )) && arg(inner, 0) == r {
                Option::Some(inner)
            } else {
                operand_item(items.drop_first(), r)
            },
            _ => operand_item(items.drop_first(), r),
        }
    }
}

// D1's operand law on a leaf referent: a top-level antecedent count
// (countable, Unit `na`, so no APE measurement) that is no time quantity (a
// plain-number one keeps its object beside `'$guideline_unit'`).
pub open spec fn operand_ok(aitems: Seq<Item>, r: Term) -> bool {
    match operand_item(aitems, r) {
        Option::Some(c) => r is Var && arg(c, 2) == atom("countable"@) && arg(c, 3) == atom("na"@)
            && unit_of(aitems, r) is None,
        Option::None => false,
    }
}

pub open spec fn operands_ok(aitems: Seq<Item>, rs: Seq<Term>) -> bool
    decreases rs.len(),
{
    rs.len() == 0 || (operand_ok(aitems, rs[0]) && operands_ok(aitems, rs.drop_first()))
}

// The (S, N) pair of an operand leaf (nt1's pair of a `some` or comparison
// item), else the reject.
pub open spec fn operand_pair(aitems: Seq<Item>, r: Term) -> Result<(Term, Term), Term> {
    match operand_item(aitems, r) {
        Option::Some(c) => if is_comp(c, "$guideline_bound"@, 8) {
            Result::Ok((arg(c, 4), arg(c, 5)))
        } else {
            Result::Err(fshape("operand_bound"@))
        },
        Option::None => Result::Err(fshape("operand"@)),
    }
}

pub open spec fn bound_pairs(aitems: Seq<Item>, rs: Seq<Term>) -> Result<
    Seq<(Term, (Term, Term))>,
    Term,
>
    decreases rs.len(),
{
    if rs.len() == 0 {
        Result::Ok(Seq::empty())
    } else {
        match (operand_pair(aitems, rs[0]), bound_pairs(aitems, rs.drop_first())) {
            (Result::Err(e), _) => Result::Err(e),
            (_, Result::Err(e)) => Result::Err(e),
            (Result::Ok(p), Result::Ok(ps)) => Result::Ok(seq![(rs[0], p)] + ps),
        }
    }
}

// Every leaf's operand law before any leaf's bound (contract da2 D5).
#[verifier::opaque]
pub open spec fn operand_pairs(aitems: Seq<Item>, rs: Seq<Term>) -> Result<
    Seq<(Term, (Term, Term))>,
    Term,
> {
    if operands_ok(aitems, rs) {
        bound_pairs(aitems, rs)
    } else {
        Result::Err(fshape("operand"@))
    }
}

pub open spec fn pair_of(ps: Seq<(Term, (Term, Term))>, r: Term) -> (Term, Term)
    decreases ps.len(),
{
    if ps.len() == 0 {
        (Term::Nil, Term::Nil)
    } else if ps[0].0 == r {
        ps[0].1
    } else {
        pair_of(ps.drop_first(), r)
    }
}

pub ghost struct Chain {
    pub goals: Seq<Term>,
    pub out: (Term, Term),
    pub v: nat,  // next fresh variable
    pub m: nat,  // literal leaves used
}

// The postorder built-in chain of E: an operand leaf = its pair, the m-th
// literal leaf = (exactly, vals[m]); node i owns the fresh variables v, v + 1.
pub open spec fn chain(
    e: Term,
    ps: Seq<(Term, (Term, Term))>,
    vals: Seq<Term>,
    v: nat,
    m: nat,
) -> Chain
    decreases e,
{
    match e {
        Term::Comp(name, args) => if name == ascii("expr"@) && args.len() == 3 {
            let l = chain(args[1], ps, vals, v, m);
            let r = chain(args[2], ps, vals, l.v, l.m);
            let op = if args[0] == atom("*"@) {
                ascii("guideline_product"@)
            } else {
                ascii("guideline_quotient"@)
            };
            let out = (Term::Var(r.v), Term::Var(r.v + 1));
            Chain {
                goals: l.goals + r.goals + seq![
                    Term::Comp(op, seq![l.out.0, l.out.1, r.out.0, r.out.1, out.0, out.1]),
                ],
                out,
                v: r.v + 2,
                m: r.m,
            }
        } else {
            Chain { goals: Seq::empty(), out: (atom("exactly"@), vals[m as int]), v, m: m + 1 }
        },
        _ => Chain { goals: Seq::empty(), out: pair_of(ps, e), v, m },
    }
}

// Witness values of the chain outputs, node by node (contract da2 D4); None
// when a node derives nothing on the witness.
pub open spec fn chain_pairs(goals: Seq<Term>, pairs: Seq<(Term, Term)>) -> Option<
    Seq<(Term, Term)>,
>
    decreases goals.len(),
{
    if goals.len() == 0 {
        Option::Some(pairs)
    } else {
        match crate::arith::arith_result(bind_all(goals[0], pairs)) {
            Option::Some((sv, nv)) => chain_pairs(
                goals.drop_first(),
                pairs.push((arg(goals[0], 4), sv)).push((arg(goals[0], 5), nv)),
            ),
            Option::None => Option::None,
        }
    }
}

// An operand's witness: an exact nonzero value its condition admits, where one
// exists (contract da2 D4).
pub open spec fn operand_witness(op: Term, count: Term) -> Term {
    match count {
        Term::Int(n) => Term::Int(
            if op == atom("geq"@) && n == 0 {
                1
            } else if op == atom("leq"@) && n == 0 {
                -1
            } else if op == atom("greater"@) {
                if n + 1 == 0 {
                    n + 2
                } else {
                    n + 1
                }
            } else if op == atom("less"@) {
                if n - 1 == 0 {
                    n - 2
                } else {
                    n - 1
                }
            } else {
                n
            },
        ),
        _ => count,
    }
}

// Formula leaves that are referents, across the formulas.
pub open spec fn formula_refs(fs: Seq<Term>) -> Seq<Term>
    decreases fs.len(),
{
    if fs.len() == 0 {
        Seq::empty()
    } else {
        var_leaves(arg(fs[0], 2)) + formula_refs(fs.drop_first())
    }
}

// The formulas whose target is y.
pub open spec fn targets(fs: Seq<Term>, y: Term) -> nat
    decreases fs.len(),
{
    if fs.len() == 0 {
        0
    } else {
        (if arg(fs[0], 0) == y {
            1nat
        } else {
            0nat
        }) + targets(fs.drop_first(), y)
    }
}

// A `some` operand loses its nt1 filter (extension 2); the target's object
// becomes its computed amount.
pub open spec fn free_operand(src: Src, an: Seq<(Term, int)>, ops: Seq<Term>, inner: Term) -> bool {
    &&& is_comp(inner, "$guideline_bound"@, 8)
    &&& ops.contains(arg(inner, 0))
    &&& arg(inner, 6) == atom("geq"@) && arg(inner, 7) == Term::Int(2)
    &&& some_before(src, an, arg(inner, 0))
}

pub open spec fn amount(inner: Term, sv: Term, nv: Term) -> Term {
    comp(
        "$guideline_amount"@,
        seq![arg(inner, 0), arg(inner, 1), arg(inner, 2), arg(inner, 3), sv, nv],
    )
}

pub open spec fn free_items(
    src: Src,
    an: Seq<(Term, int)>,
    ops: Seq<Term>,
    items: Seq<Item>,
) -> Seq<Item>
    decreases items.len(),
{
    if items.len() == 0 {
        Seq::empty()
    } else {
        seq![
            match items[0] {
                Item::Anch(c, inner) => if c == actual() && free_operand(src, an, ops, inner) {
                    Item::Anch(c, amount(inner, arg(inner, 4), arg(inner, 5)))
                } else {
                    items[0]
                },
                _ => items[0],
            },
        ] + free_items(src, an, ops, items.drop_first())
    }
}

// The target's object becomes its computed amount.
pub open spec fn rewrite_target(items: Seq<Item>, y: Term, sv: Term, nv: Term) -> Seq<Item>
    decreases items.len(),
{
    if items.len() == 0 {
        Seq::empty()
    } else {
        seq![
            match items[0] {
                Item::Anch(c, inner) => if c == actual() && is_comp(inner, "object"@, 6) && arg(
                    inner,
                    0,
                ) == y {
                    Item::Anch(c, amount(inner, sv, nv))
                } else {
                    items[0]
                },
                _ => items[0],
            },
        ] + rewrite_target(items.drop_first(), y, sv, nv)
    }
}

// A formula anywhere but the top level of the consequent (contract da2 D5).
pub open spec fn misplaced_item(it: Item, top_ok: bool) -> bool
    decreases it, 0int,
{
    match it {
        Item::Anch(c, inner) => is_formula(inner) && !(top_ok && c == actual()),
        Item::Naf(_, payload) => misplaced(payload, false),
        Item::Op(_, _, _, _) => false,
    }
}

pub open spec fn misplaced(items: Seq<Item>, top_ok: bool) -> bool
    decreases items, 1int,
{
    items.len() > 0 && (misplaced_item(items[0], top_ok) || misplaced(items.drop_first(), top_ok))
}

pub open spec fn top_formula(it: Item) -> bool {
    match it {
        Item::Anch(c, inner) => c == actual() && is_formula(inner),
        _ => false,
    }
}

// The consequent object that a formula's target names, as D1 requires it.
pub open spec fn target_ok(src: Src, an: Seq<(Term, int)>, citems: Seq<Item>, y: Term) -> bool {
    match top_object(citems, y) {
        Option::Some(o) => arg(o, 2) == atom("countable"@) && arg(o, 3) == atom("na"@) && arg(o, 4)
            == atom("geq"@) && arg(o, 5) == Term::Int(2) && some_before(src, an, y),
        Option::None => false,
    }
}

pub ghost struct FPass {
    pub aitems: Seq<Item>,
    pub citems: Seq<Item>,
    pub chain: Seq<Term>,
    pub pairs: Seq<(Term, Term)>,  // the chain outputs' witness values
}

// One formula after the earlier ones (contract da2 D1–D5): checks in order,
// then its chain, its target rewritten, its witness evaluated.
pub open spec fn formula_step(
    src: Src,
    an: Seq<(Term, int)>,
    fs: Seq<Term>,
    f: Term,
    aitems: Seq<Item>,
    acc: FPass,
    v: nat,
) -> Result<(FPass, nat), Term> {
    let y = arg(f, 0);
    let e = arg(f, 2);
    if arg(f, 1) != atom("="@) {
        Result::Err(fshape("relation"@))
    } else if !is_comp(e, "expr"@, 3) || !ops_ok(e) {
        Result::Err(fshape("operator"@))
    } else if !(y is Var) || formula_refs(fs).contains(y) || targets(fs, y) != 1 || !target_ok(
        src,
        an,
        acc.citems,
        y,
    ) {
        Result::Err(fshape("target"@))
    } else {
        match operand_pairs(aitems, var_leaves(e)) {
            Result::Err(r) => Result::Err(r),
            Result::Ok(ps) => match formula_token(an, f) {
                Option::None => Result::Err(fshape("literal"@)),
                Option::Some(kf) => match literal_values(src, kf, lit_leaves(e)) {
                    Option::None => Result::Err(fshape("literal"@)),
                    Option::Some(vals) => {
                        let ch = chain(e, ps, vals, v, 0);
                        match chain_pairs(ch.goals, acc.pairs) {
                            Option::None => Result::Err(fshape("witness"@)),
                            Option::Some(pairs) => Result::Ok(
                                (
                                    FPass {
                                        aitems: acc.aitems,
                                        citems: rewrite_target(acc.citems, y, ch.out.0, ch.out.1),
                                        chain: acc.chain + ch.goals,
                                        pairs,
                                    },
                                    ch.v,
                                ),
                            ),
                        }
                    },
                },
            },
        }
    }
}

pub open spec fn formula_steps(
    src: Src,
    an: Seq<(Term, int)>,
    fs: Seq<Term>,
    i: nat,
    aitems: Seq<Item>,
    acc: FPass,
    v: nat,
) -> Result<FPass, Term>
    decreases fs.len() - i,
{
    if i >= fs.len() {
        Result::Ok(acc)
    } else {
        match formula_step(src, an, fs, fs[i as int], aitems, acc, v) {
            Result::Err(e) => Result::Err(e),
            Result::Ok((acc2, v2)) => formula_steps(src, an, fs, i + 1, aitems, acc2, v2),
        }
    }
}

pub open spec fn top_formulas(items: Seq<Item>) -> Seq<Term>
    decreases items.len(),
{
    if items.len() == 0 {
        Seq::empty()
    } else {
        match items[0] {
            Item::Anch(c, inner) => if c == actual() && is_formula(inner) {
                seq![inner] + top_formulas(items.drop_first())
            } else {
                top_formulas(items.drop_first())
            },
            _ => top_formulas(items.drop_first()),
        }
    }
}

pub open spec fn drop_formulas(items: Seq<Item>) -> Seq<Item>
    decreases items.len(),
{
    if items.len() == 0 {
        Seq::empty()
    } else if top_formula(items[0]) {
        drop_formulas(items.drop_first())
    } else {
        seq![items[0]] + drop_formulas(items.drop_first())
    }
}

// The formula pass of one variant (contract da2 D3): `aitems` = the converted
// antecedent (nt1 first), `v` = the first variable past nt1's block. The
// consequent loses its formulas, a `some` operand its filter; the witness
// pairs cover the antecedent, then the chain outputs.
pub open spec fn formula_pass(
    src: Src,
    an: Seq<(Term, int)>,
    aitems: Seq<Item>,
    citems: Seq<Item>,
    ordered: Seq<Term>,
    map: Seq<(Term, Term)>,
    docid: Seq<u8>,
    s: nat,
    k: nat,
    v: nat,
) -> Result<FPass, Term> {
    let fs = top_formulas(citems);
    let free = free_items(src, an, formula_refs(fs), aitems);
    if misplaced(aitems, false) || misplaced(citems, true) {
        Result::Err(fshape("position"@))
    } else {
        formula_steps(
            src,
            an,
            fs,
            0,
            aitems,
            FPass {
                aitems: free,
                citems: drop_formulas(citems),
                chain: Seq::empty(),
                pairs: witness_pairs(
                    free,
                    ordered,
                    map,
                    docid,
                    s,
                    k,
                    formula_refs(fs),
                    Seq::empty(),
                ),
            },
            v,
        )
    }
}

// --- groups: fact clusters and rule variants ---
pub ghost struct Group {
    pub k: nat,  // variant ordinal (1 for facts)
    pub pairs: Seq<(Term, Term)>,  // witness bindings
    pub clauses: Seq<DocClause>,
}

pub open spec fn rule_clauses(heads: Seq<Term>, body: Seq<BodyItem>) -> Seq<DocClause> {
    heads.map_values(|h: Term| DocClause { head: h, body })
}

// --- binding-first body order (contract bf) ---
// A goal waits until the variables of some of its arguments are bound: a
// comparison its count inputs (/4: the comparison + count; /6: + the unit), an
// arithmetic built-in its two operand pairs, a semantic predicate other than
// the entity and event generators its referent (argument 2).
pub open spec fn waited(t: Term) -> Seq<Term> {
    match t {
        Term::Comp(name, args) => if crate::compare::is_compare(name, args.len()) {
            if args.len() == 4 {
                args.take(2)
            } else {
                args.take(3)
            }
        } else if crate::arith::is_arith(name, args.len()) {
            args.take(4)
        } else if is_semantic_pred(name, args.len()) && args.len() >= 2 && name != ascii(
            "guideline_entity"@,
        ) && name != ascii("guideline_event"@) {
            seq![args[1]]
        } else {
            Seq::empty()
        },
        _ => Seq::empty(),
    }
}

// A NAF item waits until its variables that positive goals share are bound.
pub open spec fn ready(it: BodyItem, bound: Set<nat>, shared: Set<nat>) -> bool {
    match it {
        BodyItem::Pos(t) => vars_all(waited(t)).subset_of(bound),
        BodyItem::Naf(gs) => vars_all(gs).intersect(shared).subset_of(bound),
    }
}

// Least position >= from whose item is ready.
pub open spec fn ready_from(
    items: Seq<BodyItem>,
    bound: Set<nat>,
    shared: Set<nat>,
    from: nat,
) -> Option<nat>
    decreases items.len() - from,
{
    if from >= items.len() {
        Option::None
    } else if ready(items[from as int], bound, shared) {
        Option::Some(from)
    } else {
        ready_from(items, bound, shared, from + 1)
    }
}

pub proof fn ready_from_bound(items: Seq<BodyItem>, bound: Set<nat>, shared: Set<nat>, from: nat)
    ensures
        ready_from(items, bound, shared, from) matches Option::Some(i) ==> from <= i < items.len(),
    decreases items.len() - from,
{
    if from < items.len() && !ready(items[from as int], bound, shared) {
        ready_from_bound(items, bound, shared, from + 1);
    }
}

// The leftmost ready item, else the leftmost item.
pub open spec fn pick(items: Seq<BodyItem>, bound: Set<nat>, shared: Set<nat>) -> nat {
    match ready_from(items, bound, shared, 0) {
        Option::Some(i) => i,
        Option::None => 0,
    }
}

pub open spec fn binds(it: BodyItem) -> Set<nat> {
    match it {
        BodyItem::Pos(t) => vars_of(t),
        BodyItem::Naf(_) => Set::empty(),
    }
}

pub open spec fn order_from(items: Seq<BodyItem>, bound: Set<nat>, shared: Set<nat>) -> Seq<
    BodyItem,
>
    decreases items.len(),
    via order_from_decreases
{
    if items.len() == 0 {
        Seq::empty()
    } else {
        let i = pick(items, bound, shared) as int;
        seq![items[i]] + order_from(items.remove(i), bound.union(binds(items[i])), shared)
    }
}

#[via_fn]
proof fn order_from_decreases(items: Seq<BodyItem>, bound: Set<nat>, shared: Set<nat>) {
    if items.len() > 0 {
        ready_from_bound(items, bound, shared, 0);
    }
}

// An open call binds no head variable, so the order starts from nothing bound.
pub open spec fn bind_first(body: Seq<BodyItem>) -> Seq<BodyItem> {
    order_from(body, Set::empty(), positive_vars(body))
}

pub open spec fn all_head_safe(cs: Seq<DocClause>) -> bool {
    forall|i: int| 0 <= i < cs.len() ==> head_safe(#[trigger] cs[i])
}

pub open spec fn all_naf_safe(cs: Seq<DocClause>, doms: Seq<Seq<Term>>) -> bool {
    forall|i: int| 0 <= i < cs.len() ==> naf_safe(#[trigger] cs[i], doms, 0, Set::empty())
}

pub open spec fn roots_conds(roots: Seq<Root>) -> Option<Term>
    decreases roots.len(),
{
    if roots.len() == 0 {
        Option::Some(Term::Nil)
    } else {
        let head = match roots[0] {
            Root::Anchored(inner) => Option::Some(
                Term::Comp(
                    ascii("-"@),
                    seq![inner, Term::Comp(ascii("/"@), seq![Term::Int(0), Term::Int(0)])],
                ),
            ),
            Root::Boxed(c) => Option::Some(c),
            Root::Rule(_, _) => Option::None,
        };
        match (head, roots_conds(roots.drop_first())) {
            (Option::Some(h), Option::Some(t)) => Option::Some(Term::Comp(cons_name(), seq![h, t])),
            _ => Option::None,
        }
    }
}

// A root fact cluster: flatten, enumerate referents, mint product identities
// (document map threads on), expand to ground fact heads.
pub open spec fn fact_group(
    roots: Seq<Root>,
    s: nat,
    docid: Seq<u8>,
    map: Seq<(Term, Term)>,
    base: nat,
    tab: Option<Env>,
) -> Result<(Group, Seq<(Term, Term)>), Term> {
    match roots_conds(roots) {
        Option::None => Result::Err(atom("sentence_shape"@)),
        Option::Some(conds) => match flatten_doc(
            tab,
            conds,
            Where::Root,
            s,
            docid,
            Term::Nil,
            actual(),
            Encl::Top,
            1,
            base,
        ) {
            Result::Err(e) => Result::Err(e),
            Result::Ok(f) => {
                let ordered = first_vars(ref_slots(f.items), Seq::empty());
                let map2 = mint(ordered, 1, s, docid, map);
                match expand(f.items, map2, Seq::empty()) {
                    Result::Err(e) => Result::Err(e),
                    Result::Ok(heads) => if !all_pos(heads) {
                        Result::Err(atom("sentence_shape"@))
                    } else {
                        let cs = pos_terms(heads).map_values(|h: Term| fact_clause(h));
                        if !all_head_safe(cs) {
                            Result::Err(atom("head_variable_not_bound_in_body"@))
                        } else {
                            Result::Ok((Group { k: 1, pairs: Seq::empty(), clauses: cs }, map2))
                        }
                    },
                }
            },
        },
    }
}

// Rules: consequent currying (a singleton bare implication under an empty
// intermediate domain folds its antecedent into the body, recursively).
pub ghost struct Seg {
    pub dom: Seq<Term>,
    pub conds: Term,
}

pub open spec fn curry(ante: Term, cons: Term, fuel: nat) -> Result<(Seq<Seg>, Term), Term>
    decreases fuel,
{
    match box_parts(ante) {
        Option::None => Result::Err(atom("invalid_drs_shape"@)),
        Option::Some((adom, aconds)) => {
            let seg = Seg { dom: adom, conds: aconds };
            if fuel > 0 && is_comp(cons, "drs"@, 2) && arg(cons, 0) == Term::Nil && is_comp(
                arg(cons, 1),
                "[|]"@,
                2,
            ) && arg(arg(cons, 1), 1) == Term::Nil && is_comp(arg(arg(cons, 1), 0), "=>"@, 2) {
                let single = arg(arg(cons, 1), 0);
                match curry(arg(single, 0), arg(single, 1), (fuel - 1) as nat) {
                    Result::Err(e) => Result::Err(e),
                    Result::Ok((segs, fc)) => Result::Ok((seq![seg] + segs, fc)),
                }
            } else {
                Result::Ok((seq![seg], cons))
            }
        },
    }
}

pub open spec fn seg_domain(segs: Seq<Seg>) -> Seq<Term>
    decreases segs.len(),
{
    if segs.len() == 0 {
        Seq::empty()
    } else {
        segs[0].dom + seg_domain(segs.drop_first())
    }
}

// Split a condition spine into shared conditions and top-level disjunctions.
pub open spec fn split_conds(l: Term) -> Option<(Seq<Term>, Seq<(Term, Term)>)>
    decreases l,
{
    match l {
        Term::Nil => Option::Some((Seq::empty(), Seq::empty())),
        Term::Comp(name, args) => if name == cons_name() && args.len() == 2 {
            match split_conds(args[1]) {
                Option::None => Option::None,
                Option::Some((sh, vs)) => if is_comp(args[0], "v"@, 2) {
                    Option::Some((sh, seq![(arg(args[0], 0), arg(args[0], 1))] + vs))
                } else {
                    Option::Some((seq![args[0]] + sh, vs))
                },
            }
        } else {
            Option::None
        },
        _ => Option::None,
    }
}

pub open spec fn split_scan(segs: Seq<Seg>) -> Option<(Seq<Term>, Seq<(Term, Term)>)>
    decreases segs.len(),
{
    if segs.len() == 0 {
        Option::Some((Seq::empty(), Seq::empty()))
    } else {
        match (split_conds(segs[0].conds), split_scan(segs.drop_first())) {
            (Option::Some((sh, vs)), Option::Some((sh2, vs2))) => Option::Some(
                (sh + sh2, vs + vs2),
            ),
            _ => Option::None,
        }
    }
}

pub open spec fn list_of(ts: Seq<Term>) -> Term {
    list_term(ts)
}

// Flatten a sequence of raw conditions as antecedent items, threading the box ordinal.
pub open spec fn flatten_seq(
    cs: Seq<Term>,
    s: nat,
    docid: Seq<u8>,
    n: nat,
    base: nat,
    tab: Option<Env>,
) -> Result<Flat, Term> {
    flatten_doc(
        tab,
        list_of(cs),
        Where::Antecedent,
        s,
        docid,
        Term::Nil,
        actual(),
        Encl::Top,
        n,
        base,
    )
}

pub open spec fn cons_locals(ordered: Seq<Term>, ante: Seq<Term>, map: Seq<(Term, Term)>) -> Seq<
    Term,
> {
    ordered.filter(|v: Term| !ante.contains(v) && lookup(map, v) is None)
}

pub open spec fn skolem(
    locals: Seq<Term>,
    ordered: Seq<Term>,
    deps: Term,
    s: nat,
    docid: Seq<u8>,
) -> Seq<(Term, Term)> {
    locals.map_values(|v: Term| (v, gid("product"@, docid, s, ref_slot(nth_var(ordered, v)), deps)))
}

// Witness bindings of one variant: anchored referents (not document-mapped)
// take ref(N) from the unsplit enumeration, operator contexts take box(B);
// NAF payloads contribute nothing.
pub open spec fn witness_pairs(
    items: Seq<Item>,
    ordered: Seq<Term>,
    map: Seq<(Term, Term)>,
    docid: Seq<u8>,
    s: nat,
    k: nat,
    ops: Seq<Term>,
    acc: Seq<(Term, Term)>,
) -> Seq<(Term, Term)>
    decreases items.len(),
{
    if items.len() == 0 {
        acc
    } else {
        let acc1 = match items[0] {
            Item::Op(b, _, inner, _) => if inner is Var && lookup(acc, inner) is None {
                acc.push((inner, gid("witness"@, docid, s, box_slot(b), variant_slot(k))))
            } else {
                acc
            },
            Item::Naf(_, _) => acc,
            Item::Anch(_, inner) => {
                let acc0 = witness_refs(cond_refs(inner), ordered, map, docid, s, k, acc);
                // nt1 D4: a converted condition's witness states its own bound;
                // da2 D4: a formula operand's, an exact nonzero value it admits
                if is_comp(inner, "$guideline_bound"@, 8) && ops.contains(arg(inner, 0)) {
                    acc0.push((arg(inner, 4), atom("exactly"@))).push(
                        (arg(inner, 5), operand_witness(arg(inner, 6), arg(inner, 7))),
                    )
                } else if is_comp(inner, "$guideline_bound"@, 8) {
                    acc0.push((arg(inner, 4), arg(inner, 6))).push((arg(inner, 5), arg(inner, 7)))
                } else if is_comp(inner, "$guideline_amount"@, 6) {
                    acc0.push((arg(inner, 4), atom("exactly"@))).push((arg(inner, 5), Term::Int(1)))
                } else if is_comp(inner, "$guideline_scaled"@, 6) {
                    acc0.push((arg(inner, 0), arg(inner, 3))).push(
                        (arg(inner, 1), arg(inner, 4)),
                    ).push((arg(inner, 2), arg(inner, 5)))
                } else {
                    acc0
                }
            },
        };
        witness_pairs(items.drop_first(), ordered, map, docid, s, k, ops, acc1)
    }
}

pub open spec fn variant_slot(k: nat) -> Term {
    Term::Comp(ascii("variant"@), seq![Term::Int(k as int)])
}

pub open spec fn witness_refs(
    refs: Seq<Term>,
    ordered: Seq<Term>,
    map: Seq<(Term, Term)>,
    docid: Seq<u8>,
    s: nat,
    k: nat,
    acc: Seq<(Term, Term)>,
) -> Seq<(Term, Term)>
    decreases refs.len(),
{
    if refs.len() == 0 {
        acc
    } else {
        let r = refs[0];
        let acc1 = if r is Var && lookup(map, r) is None && lookup(acc, r) is None {
            acc.push((r, gid("witness"@, docid, s, ref_slot(nth_var(ordered, r)), variant_slot(k))))
        } else {
            acc
        };
        witness_refs(refs.drop_first(), ordered, map, docid, s, k, acc1)
    }
}

// One Horn variant: body = antecedent items (comparison conditions converted,
// nt1 D3) + the formula chains (da2 D3), one clause per consequent head.
pub open spec fn variant(
    aitems0: Seq<Item>,
    citems0: Seq<Item>,
    ordered: Seq<Term>,
    map: Seq<(Term, Term)>,
    deps: Term,
    s: nat,
    docid: Seq<u8>,
    k: nat,
    cb: nat,
    src: Src,
    an: Seq<(Term, int)>,
) -> Result<Group, Term> {
    let ante_refs = first_vars(ref_slots(aitems0), Seq::empty());
    let sko = skolem(cons_locals(ordered, ante_refs, map), ordered, deps, s, docid);
    match formula_pass(
        src,
        an,
        compared(aitems0, cb),
        citems0,
        ordered,
        map,
        docid,
        s,
        k,
        cb + 3 * conv_refs(aitems0).len(),
    ) {
        Result::Err(e) => Result::Err(e),
        Result::Ok(fp) => match (expand(fp.aitems, map, sko), expand(fp.citems, map, sko)) {
            (Result::Ok(goals), Result::Ok(heads)) => if goals.len() == 0 {
                Result::Err(atom("rule_without_antecedent"@))
            } else if heads.len() == 0 || !all_pos(heads) {
                Result::Err(atom("rule_without_consequent"@))
            } else {
                let body = goals + pos_all(fp.chain);
                let cs = rule_clauses(pos_terms(heads), body);
                if !all_naf_safe(cs, body_doms(fp.aitems)) {
                    Result::Err(atom("naf_safety"@))
                } else if !all_head_safe(cs) {
                    Result::Err(atom("head_variable_not_bound_in_body"@))
                } else {
                    // The checks read the DRS order; the clauses run binding-first.
                    Result::Ok(
                        Group {
                            k,
                            pairs: fp.pairs,
                            clauses: rule_clauses(pos_terms(heads), bind_first(body)),
                        },
                    )
                }
            },
            (Result::Err(e), _) => Result::Err(e),
            (_, Result::Err(e)) => Result::Err(e),
        },
    }
}

pub open spec fn flatten_cons(
    cconds: Term,
    s: nat,
    docid: Seq<u8>,
    deps: Term,
    n: nat,
    base: nat,
    tab: Option<Env>,
) -> Result<Flat, Term> {
    flatten_doc(tab, cconds, Where::Consequent, s, docid, deps, actual(), Encl::Top, n, base)
}

// The rule law: curry → split scan → one variant, or two over a single
// top-level disjunction (shared → arm 1 → arm 2 → consequent = the one
// unsplit enumeration; variants differ by Deps alone).
pub open spec fn rule_groups(
    ante: Term,
    cons: Term,
    s: nat,
    docid: Seq<u8>,
    map: Seq<(Term, Term)>,
    base: nat,
    tab: Option<Env>,
    src: Src,
) -> Result<Seq<Group>, Term> {
    let an = anchored_all(seq![ante, cons]);
    match curry(ante, cons, 64) {
        Result::Err(e) => Result::Err(e),
        Result::Ok((segs, fcons)) => match box_parts(fcons) {
            Option::None => Result::Err(atom("invalid_drs_shape"@)),
            Option::Some((_, cconds)) => {
                let adom = seg_domain(segs);
                match split_scan(segs) {
                    Option::None => Result::Err(atom("invalid_drs_shape"@)),
                    Option::Some((shared, vs)) => if vs.len() == 0 {
                        match flatten_seq(shared, s, docid, 1, base, tab) {
                            Result::Err(e) => Result::Err(e),
                            Result::Ok(fa) => match flatten_cons(
                                cconds,
                                s,
                                docid,
                                list_of(adom),
                                fa.n,
                                base,
                                tab,
                            ) {
                                Result::Err(e) => Result::Err(e),
                                Result::Ok(fc) => {
                                    let ordered = first_vars(
                                        ref_slots(fa.items + fc.items),
                                        Seq::empty(),
                                    );
                                    match variant(
                                        fa.items,
                                        fc.items,
                                        ordered,
                                        map,
                                        list_of(adom),
                                        s,
                                        docid,
                                        1,
                                        base + fc.n,
                                        src,
                                        an,
                                    ) {
                                        Result::Err(e) => Result::Err(e),
                                        Result::Ok(g) => Result::Ok(seq![g]),
                                    }
                                },
                            },
                        }
                    } else if vs.len() == 1 {
                        match (box_parts(vs[0].0), box_parts(vs[0].1)) {
                            (
                                Option::Some((dom1, conds1)),
                                Option::Some((dom2, conds2)),
                            ) => if is_comp(vs[0].0, "v"@, 2) || is_comp(vs[0].1, "v"@, 2) {
                                Result::Err(atom("disjunctive_antecedent"@))
                            } else {
                                split_variants(
                                    shared,
                                    dom1,
                                    conds1,
                                    dom2,
                                    conds2,
                                    cconds,
                                    adom,
                                    s,
                                    docid,
                                    map,
                                    base,
                                    tab,
                                    src,
                                    an,
                                )
                            },
                            _ => Result::Err(atom("invalid_drs_shape"@)),
                        }
                    } else {
                        Result::Err(atom("disjunctive_antecedent"@))
                    },
                }
            },
        },
    }
}

pub open spec fn split_variants(
    shared: Seq<Term>,
    dom1: Seq<Term>,
    conds1: Term,
    dom2: Seq<Term>,
    conds2: Term,
    cconds: Term,
    adom: Seq<Term>,
    s: nat,
    docid: Seq<u8>,
    map: Seq<(Term, Term)>,
    base: nat,
    tab: Option<Env>,
    src: Src,
    an: Seq<(Term, int)>,
) -> Result<Seq<Group>, Term> {
    match flatten_seq(shared, s, docid, 1, base, tab) {
        Result::Err(e) => Result::Err(e),
        Result::Ok(fs) => match flatten_doc(
            tab,
            conds1,
            Where::Antecedent,
            s,
            docid,
            Term::Nil,
            actual(),
            Encl::Top,
            fs.n,
            base,
        ) {
            Result::Err(e) => Result::Err(e),
            Result::Ok(f1) => match flatten_doc(
                tab,
                conds2,
                Where::Antecedent,
                s,
                docid,
                Term::Nil,
                actual(),
                Encl::Top,
                f1.n,
                base,
            ) {
                Result::Err(e) => Result::Err(e),
                Result::Ok(f2) => {
                    let deps1 = list_of(adom + dom1);
                    let deps2 = list_of(adom + dom2);
                    match (
                        flatten_cons(cconds, s, docid, deps1, f2.n, base, tab),
                        flatten_cons(cconds, s, docid, deps2, f2.n, base, tab),
                    ) {
                        (Result::Ok(c1), Result::Ok(c2)) => {
                            let ordered = first_vars(
                                ref_slots(fs.items + f1.items + f2.items + c1.items),
                                Seq::empty(),
                            );
                            match (
                                variant(
                                    fs.items + f1.items,
                                    c1.items,
                                    ordered,
                                    map,
                                    deps1,
                                    s,
                                    docid,
                                    1,
                                    base + c1.n,
                                    src,
                                    an,
                                ),
                                variant(
                                    fs.items + f2.items,
                                    c2.items,
                                    ordered,
                                    map,
                                    deps2,
                                    s,
                                    docid,
                                    2,
                                    base + c1.n,
                                    src,
                                    an,
                                ),
                            ) {
                                (Result::Ok(g1), Result::Ok(g2)) => Result::Ok(seq![g1, g2]),
                                (Result::Err(e), _) => Result::Err(e),
                                (_, Result::Err(e)) => Result::Err(e),
                            }
                        },
                        (Result::Err(e), _) => Result::Err(e),
                        (_, Result::Err(e)) => Result::Err(e),
                    }
                },
            },
        },
    }
}

pub open spec fn is_fact_root(r: Root) -> bool {
    match r {
        Root::Rule(_, _) => false,
        _ => true,
    }
}

// A sentence: one rule, or a cluster of anchored/boxed root conditions.
pub open spec fn sentence_groups(
    roots: Seq<Root>,
    s: nat,
    docid: Seq<u8>,
    map: Seq<(Term, Term)>,
    base: nat,
    tab: Option<Env>,
    src: Src,
) -> Result<(Seq<Group>, Seq<(Term, Term)>), Term> {
    if roots.len() == 1 && roots[0] is Rule {
        match roots[0] {
            Root::Rule(a, c) => match rule_groups(a, c, s, docid, map, base, tab, src) {
                Result::Err(e) => Result::Err(e),
                Result::Ok(gs) => Result::Ok((gs, map)),
            },
            _ => Result::Err(atom("sentence_shape"@)),
        }
    } else if forall|i: int| 0 <= i < roots.len() ==> is_fact_root(#[trigger] roots[i]) {
        match fact_group(roots, s, docid, map, base, tab) {
            Result::Err(e) => Result::Err(e),
            Result::Ok((g, map2)) => Result::Ok((seq![g], map2)),
        }
    } else {
        Result::Err(atom("sentence_shape"@))
    }
}

pub open spec fn group_clauses(gs: Seq<Group>) -> Seq<DocClause> {
    gs.map_values(|g: Group| g.clauses).flatten()
}

// --- scope exclusions (contract cs1 D2–D5: documents with a scope + exclusion pair) ---
// An exclusion rule denies that the guideline's recommendations (the scope
// noun) apply (the exclusion verb) under its condition; it compiles to its own
// clauses + one ground record `guideline_exclusion(Key)`, and every
// should|must|may edge clause of the document ends with `\+ guideline_exclusion(Key)`.
pub open spec fn deontic(name: Seq<u8>) -> bool {
    name == ascii("should"@) || name == ascii("must"@) || name == ascii("may"@)
}

// A should|must|may box anywhere inside t.
pub open spec fn has_deontic(t: Term) -> bool
    decreases t, 0int,
{
    match t {
        Term::Comp(name, args) => (deontic(name) && args.len() == 1) || has_deontic_all(args),
        _ => false,
    }
}

pub open spec fn has_deontic_all(ts: Seq<Term>) -> bool
    decreases ts, 1int,
{
    ts.len() > 0 && (has_deontic(ts[0]) || has_deontic_all(ts.drop_first()))
}

// A classical negation box anywhere inside t that holds a should|must|may box.
pub open spec fn negated_deontic(t: Term) -> bool
    decreases t, 0int,
{
    match t {
        Term::Comp(name, args) => if name == ascii("-"@) && args.len() == 1 {
            has_deontic(args[0])
        } else {
            negated_deontic_all(args)
        },
        _ => false,
    }
}

pub open spec fn negated_deontic_all(ts: Seq<Term>) -> bool
    decreases ts, 1int,
{
    ts.len() > 0 && (negated_deontic(ts[0]) || negated_deontic_all(ts.drop_first()))
}

// The anchor-stripped leaves of a box's conditions, when every condition is an
// anchored leaf (None: a box, list or other condition stands in it).
pub open spec fn leaves_of(conds: Seq<Term>) -> Option<Seq<Term>>
    decreases conds.len(),
{
    if conds.len() == 0 {
        Option::Some(Seq::empty())
    } else {
        match anchor_s(conds[0]) {
            Option::None => Option::None,
            Option::Some(_) => match leaves_of(conds.drop_first()) {
                Option::None => Option::None,
                Option::Some(rest) => Option::Some(seq![arg(conds[0], 0)] + rest),
            },
        }
    }
}

// ls = the box's leaves through upper-bound groups (`group_leaves` of its
// condition list): `exactly|at most|less than N recommendations` sits in a
// group, and must reach `consequent`, never compile as plain data.
// Some leaf is an exclusion-verb `predicate/3..5` (a transitive one reaches
// D3, which then rejects it as `consequent`).
pub open spec fn names_verb(tab: Vocabulary, ls: Seq<Term>) -> bool {
    exists|i: int|
        0 <= i < ls.len() && is_pred(#[trigger] ls[i]) && arg(ls[i], 1) == Term::Atom(
            tab.exclusions[0],
        )
}

// The first such leaf.
pub open spec fn verb_leaf(tab: Vocabulary, ls: Seq<Term>) -> Term
    decreases ls.len(),
{
    if ls.len() == 0 {
        Term::Nil
    } else if is_pred(ls[0]) && arg(ls[0], 1) == Term::Atom(tab.exclusions[0]) {
        ls[0]
    } else {
        verb_leaf(tab, ls.drop_first())
    }
}

// The noun of referent x's first `object/6` leaf.
pub open spec fn noun_in(ls: Seq<Term>, x: Term) -> Option<Term>
    decreases ls.len(),
{
    if ls.len() == 0 {
        Option::None
    } else if is_comp(ls[0], "object"@, 6) && arg(ls[0], 0) == x {
        Option::Some(arg(ls[0], 1))
    } else {
        noun_in(ls.drop_first(), x)
    }
}

pub open spec fn pps_ok(ls: Seq<Term>, e: Term, dom: Seq<Term>) -> bool {
    forall|i: int|
        0 <= i < ls.len() ==> is_comp(#[trigger] ls[i], "modifier_pp"@, 3) && arg(ls[i], 0) == e
            && !dom.contains(arg(ls[i], 2))
}

// D3: the box's leaves = a plain indefinite scope-noun subject, the
// exclusion verb's `predicate/3` on it, then pps on that event whose objects
// lie outside the box.
pub open spec fn exclusion_box(tab: Vocabulary, dom: Seq<Term>, conds: Seq<Term>) -> bool {
    match leaves_of(conds) {
        Option::None => false,
        Option::Some(ls) => {
            &&& ls.len() >= 2
            &&& is_comp(ls[0], "object"@, 6)
            &&& arg(ls[0], 1) == Term::Atom(tab.scopes[0].0)
            &&& arg(ls[0], 2) == atom("countable"@)
            &&& arg(ls[0], 3) == atom("na"@)
            &&& arg(ls[0], 4) == atom("eq"@)
            &&& arg(ls[0], 5) == Term::Int(1)
            &&& is_comp(ls[1], "predicate"@, 3)
            &&& arg(ls[1], 1) == Term::Atom(tab.exclusions[0])
            &&& arg(ls[1], 2) == arg(ls[0], 0)
            &&& dom.contains(arg(ls[0], 0))
            &&& dom.contains(arg(ls[1], 0))
            &&& pps_ok(ls.skip(2), arg(ls[1], 0), dom)
        },
    }
}

// A top-level classical negation box of the conditions holding the exclusion verb.
pub open spec fn verb_box(tab: Vocabulary, cs: Seq<Term>) -> Option<Term>
    decreases cs.len(),
{
    if cs.len() == 0 {
        Option::None
    } else if is_comp(cs[0], "-"@, 1) && match box_parts(arg(cs[0], 0)) {
        Option::Some((_, l)) => list_items(l) is Some && names_verb(tab, group_leaves(l)),
        Option::None => false,
    } {
        Option::Some(arg(cs[0], 0))
    } else {
        verb_box(tab, cs.drop_first())
    }
}

pub ghost enum Scope {
    Plain,
    Exclusion,
    Reject(Seq<char>),
}

// A rule sentence's scope class (D4 order: deontic_condition, negated_deontic,
// subject, consequent).
pub open spec fn rule_scope(tab: Vocabulary, ante: Term, cons: Term) -> Scope {
    match curry(ante, cons, 64) {
        Result::Err(_) => Scope::Plain,
        Result::Ok((segs, fcons)) => if exists|i: int|
            0 <= i < segs.len() && has_deontic(#[trigger] segs[i].conds) {
            Scope::Reject("deontic_condition"@)
        } else if negated_deontic(fcons) {
            Scope::Reject("negated_deontic"@)
        } else {
            match box_parts(fcons) {
                Option::None => Scope::Plain,
                Option::Some((_, l)) => match list_items(l) {
                    Option::None => Scope::Plain,
                    Option::Some(cs) => match verb_box(tab, cs) {
                        Option::None => Scope::Plain,
                        Option::Some(b) => match box_parts(b) {
                            Option::None => Scope::Plain,
                            Option::Some((dom, bl)) => match list_items(bl) {
                                Option::None => Scope::Plain,
                                Option::Some(bcs) => {
                                    let ls = group_leaves(bl);
                                    let x = arg(verb_leaf(tab, ls), 2);
                                    if !dom.contains(x) || noun_in(ls, x) != Option::Some(
                                        Term::Atom(tab.scopes[0].0),
                                    ) {
                                        Scope::Reject("subject"@)
                                    } else if cs.len() == 1 && exclusion_box(tab, dom, bcs) {
                                        Scope::Exclusion
                                    } else {
                                        Scope::Reject("consequent"@)
                                    }
                                },
                            },
                        },
                    },
                },
            }
        },
    }
}

// A root sentence's scope class (D4 order: root_deontic, root).
pub open spec fn root_scope(tab: Vocabulary, roots: Seq<Root>) -> Scope {
    if exists|i: int|
        0 <= i < roots.len() && match #[trigger] roots[i] {
            Root::Anchored(c) => has_deontic(c),
            Root::Boxed(c) => has_deontic(c),
            Root::Rule(_, _) => false,
        } {
        Scope::Reject("root_deontic"@)
    } else if exists|i: int|
        0 <= i < roots.len() && match #[trigger] roots[i] {
            Root::Boxed(c) => verb_box(tab, seq![c]) is Some,
            _ => false,
        } {
        Scope::Reject("root"@)
    } else {
        Scope::Plain
    }
}

pub open spec fn exclusion_record(key: Seq<u8>) -> Term {
    lit("guideline_exclusion"@, seq![Term::Atom(key)])
}

pub open spec fn deontic_edge(h: Term) -> bool {
    is_comp(h, "guideline_operator"@, 3) && match arg(h, 2) {
        Term::Atom(op) => deontic(op),
        _ => false,
    }
}

// D5: a should|must|may edge clause ends with the guard.
pub open spec fn guard(c: DocClause, key: Seq<u8>) -> DocClause {
    if deontic_edge(c.head) {
        DocClause { head: c.head, body: c.body.push(BodyItem::Naf(seq![exclusion_record(key)])) }
    } else {
        c
    }
}

// D3 + D5 over one group: guards on its edges; an exclusion's record ends it.
pub open spec fn scoped_group(g: Group, key: Seq<u8>, excl: bool) -> Group {
    let cs = g.clauses.map_values(|c: DocClause| guard(c, key));
    Group {
        clauses: if excl && g.clauses.len() > 0 {
            cs.push(DocClause { head: exclusion_record(key), body: g.clauses[0].body })
        } else {
            cs
        },
        ..g
    }
}

// The scope pass over one translated sentence (after its existing translation
// + the duplicate-grade check, so every existing reject reports first).
pub open spec fn scope_pass(tab: Option<Env>, roots: Seq<Root>, gs: Seq<Group>) -> Result<
    Seq<Group>,
    Term,
> {
    match tab {
        Option::None => Result::Ok(gs),
        Option::Some(env) => if !crate::vocabulary::scoped(env.tab) {
            Result::Ok(gs)
        } else {
            let key = env.tab.scopes[0].1;
            let class = if roots.len() == 1 && roots[0] is Rule {
                match roots[0] {
                    Root::Rule(a, c) => rule_scope(env.tab, a, c),
                    _ => Scope::Plain,
                }
            } else {
                root_scope(env.tab, roots)
            };
            match class {
                Scope::Reject(why) => Result::Err(s1_shape("scope_shape"@, why)),
                Scope::Exclusion => Result::Ok(
                    gs.map_values(|g: Group| scoped_group(g, key, true)),
                ),
                Scope::Plain => Result::Ok(gs.map_values(|g: Group| scoped_group(g, key, false))),
            }
        },
    }
}

// --- superseded statements (contract sup1 D2–D5: a table row naming the document) ---
// A superseded document keeps its clauses (ex nunc: the statement held before its
// supersession took effect). Every group holding a should|must|may edge gains, on
// each clause, `guideline_evaluation_date(V), guideline_satisfies(exactly, V, less,
// Date)`: an earlier evaluation date the consumer states = a necessary condition.
// Every sentence bundle ends with `guideline_supersession(DocId, S, source(Citation,
// Url), Date)`, outside the groups (no obligation, no '$guideline_id' identity).
pub open spec fn superseding_of(rows: Seq<(Seq<u8>, Superseding)>, docid: Seq<u8>) -> Option<
    Superseding,
>
    decreases rows.len(),
{
    if rows.len() == 0 {
        Option::None
    } else if rows[0].0 == docid {
        Option::Some(rows[0].1)
    } else {
        superseding_of(rows.drop_first(), docid)
    }
}

pub open spec fn supersession_record(docid: Seq<u8>, s: nat, sup: Superseding) -> Term {
    lit(
        "guideline_supersession"@,
        seq![
            Term::Atom(docid),
            Term::Int(s as int),
            Term::Comp(ascii("source"@), seq![Term::Atom(sup.citation), Term::Atom(sup.url)]),
            Term::Int(sup.date as int),
        ],
    )
}

pub open spec fn date_goals(v: Term, date: nat) -> Seq<BodyItem> {
    seq![
        BodyItem::Pos(lit("guideline_evaluation_date"@, seq![v])),
        BodyItem::Pos(
            lit(
                "guideline_satisfies"@,
                seq![atom("exactly"@), v, atom("less"@), Term::Int(date as int)],
            ),
        ),
    ]
}

// The cs1 scope guard (authors cannot spell a reserved name, so no other body
// item has this shape).
pub open spec fn scope_guard_item(it: BodyItem) -> bool {
    match it {
        BodyItem::Naf(gs) => gs.len() == 1 && is_comp(gs[0], "guideline_exclusion"@, 1),
        _ => false,
    }
}

// D4: the date goals after every body goal; a trailing scope guard stays last.
pub open spec fn date_guarded(body: Seq<BodyItem>, goals: Seq<BodyItem>) -> Seq<BodyItem> {
    if body.len() > 0 && scope_guard_item(body.last()) {
        body.drop_last() + goals + seq![body.last()]
    } else {
        body + goals
    }
}

pub open spec fn item_nvars(it: BodyItem) -> nat {
    match it {
        BodyItem::Pos(t) => nvars(t),
        BodyItem::Naf(gs) => nvars_all(gs),
    }
}

pub open spec fn body_nvars(b: Seq<BodyItem>) -> nat
    decreases b.len(),
{
    if b.len() == 0 {
        0
    } else {
        max_nat(item_nvars(b[0]), body_nvars(b.drop_first()))
    }
}

// 1 + the largest variable index of the clauses: a variable no clause holds.
pub open spec fn clauses_nvars(cs: Seq<DocClause>) -> nat
    decreases cs.len(),
{
    if cs.len() == 0 {
        0
    } else {
        max_nat(max_nat(nvars(cs[0].head), body_nvars(cs[0].body)), clauses_nvars(cs.drop_first()))
    }
}

pub open spec fn has_deontic_edge(g: Group) -> bool {
    exists|i: int| 0 <= i < g.clauses.len() && deontic_edge(#[trigger] g.clauses[i].head)
}

// D4 over one group: the witness pairs gain V ↦ Date − 1 (a numeric replay
// witness below Date, not always a calendar date).
pub open spec fn date_group(g: Group, date: nat) -> Group {
    if !has_deontic_edge(g) {
        g
    } else {
        let v = Term::Var(clauses_nvars(g.clauses));
        Group {
            k: g.k,
            pairs: g.pairs.push((v, Term::Int(date as int - 1))),
            clauses: g.clauses.map_values(
                |c: DocClause|
                    DocClause { head: c.head, body: date_guarded(c.body, date_goals(v, date)) },
            ),
        }
    }
}

pub open spec fn root_deontic(roots: Seq<Root>) -> bool {
    exists|i: int|
        0 <= i < roots.len() && match #[trigger] roots[i] {
            Root::Anchored(c) => has_deontic(c),
            Root::Boxed(c) => has_deontic(c),
            Root::Rule(_, _) => false,
        }
}

// D2 + D4 + D5 over one translated sentence, after the scope pass: the guarded
// groups + the sentence's record.
pub open spec fn supersession_pass(
    tab: Option<Env>,
    docid: Seq<u8>,
    s: nat,
    roots: Seq<Root>,
    gs: Seq<Group>,
) -> Result<(Seq<Group>, Seq<DocClause>), Term> {
    match tab {
        Option::None => Result::Ok((gs, Seq::empty())),
        Option::Some(env) => match superseding_of(env.tab.supersessions, docid) {
            Option::None => Result::Ok((gs, Seq::empty())),
            Option::Some(sup) => if root_deontic(roots) {
                Result::Err(s1_shape("supersession_shape"@, "root_deontic"@))
            } else {
                Result::Ok(
                    (
                        gs.map_values(|g: Group| date_group(g, sup.date)),
                        seq![fact_clause(supersession_record(docid, s, sup))],
                    ),
                )
            },
        },
    }
}

// --- canonical variable numbering per emitted line ---
pub open spec fn renumber(t: Term, fs: Seq<nat>) -> Term
    decreases t,
{
    match t {
        Term::Var(k) => Term::Var(pos_of(fs, k)),
        Term::Comp(name, args) => Term::Comp(name, renumber_all(args, fs)),
        _ => t,
    }
}

pub open spec fn renumber_all(ts: Seq<Term>, fs: Seq<nat>) -> Seq<Term>
    decreases ts,
{
    if ts.len() == 0 {
        Seq::empty()
    } else {
        seq![renumber(ts[0], fs)] + renumber_all(ts.drop_first(), fs)
    }
}

pub open spec fn renumber_item(it: BodyItem, fs: Seq<nat>) -> BodyItem {
    match it {
        BodyItem::Pos(t) => BodyItem::Pos(renumber(t, fs)),
        BodyItem::Naf(gs) => BodyItem::Naf(renumber_all(gs, fs)),
    }
}

// One numbervars pass per clause line: head then body, firsts 0,1,2,..
pub open spec fn canon_clause(c: DocClause) -> DocClause {
    let fs = firsts(var_stream(c.head) + body_var_stream(c.body), Set::empty());
    DocClause {
        head: renumber(c.head, fs),
        body: c.body.map_values(|it: BodyItem| renumber_item(it, fs)),
    }
}

// --- the document projection ---
pub ghost struct Projected {
    pub s: nat,
    pub groups: Seq<Group>,
    pub records: Seq<
        DocClause,
    >,  // sup1 D2: after the groups' clauses, no obligation
}

pub open spec fn err_at(s: nat, why: Term) -> Term {
    Term::Comp(ascii("sentence"@), seq![Term::Int(s as int), why])
}

// s1 D3: the dimension of each grade literal among the clauses, in order.
pub open spec fn grade_dims(cs: Seq<DocClause>) -> Seq<Term> {
    cs.filter(|c: DocClause| is_comp(c.head, "guideline_grade"@, 5)).map_values(
        |c: DocClause| arg(c.head, 2),
    )
}

// `dims` = the grade dimensions of the earlier sentences: one grade per
// dimension per document.
pub open spec fn project_from(
    tagged: Seq<(int, Root)>,
    s: nat,
    count: nat,
    docid: Seq<u8>,
    map: Seq<(Term, Term)>,
    base: nat,
    tab: Option<Env>,
    dims: Seq<Term>,
    srcs: Seq<Src>,
) -> Result<Seq<Projected>, Term>
    decreases count + 1 - s,
{
    if s > count {
        Result::Ok(Seq::empty())
    } else {
        match sentence_groups(
            of_sentence(tagged, s as int),
            s,
            docid,
            map,
            base,
            tab,
            srcs[s - 1],
        ) {
            Result::Err(e) => Result::Err(err_at(s, e)),
            Result::Ok((gs0, map2)) => {
                let dims2 = dims + grade_dims(group_clauses(gs0));
                if group_clauses(gs0).len() == 0 {
                    Result::Err(err_at(s, atom("sentence_shape"@)))
                } else if !dims2.no_duplicates() {
                    Result::Err(err_at(s, s1_shape("grade_shape"@, "duplicate_dimension"@)))
                } else {
                    match scope_pass(tab, of_sentence(tagged, s as int), gs0) {
                        Result::Err(e) => Result::Err(err_at(s, e)),
                        Result::Ok(gs1) => match supersession_pass(
                            tab,
                            docid,
                            s,
                            of_sentence(tagged, s as int),
                            gs1,
                        ) {
                            Result::Err(e) => Result::Err(err_at(s, e)),
                            Result::Ok((gs, records)) => match project_from(
                                tagged,
                                s + 1,
                                count,
                                docid,
                                map2,
                                base,
                                tab,
                                dims2,
                                srcs,
                            ) {
                                Result::Err(e) => Result::Err(e),
                                Result::Ok(rest) => Result::Ok(
                                    seq![Projected { s, groups: gs, records }] + rest,
                                ),
                            },
                        },
                    }
                }
            },
        }
    }
}

pub open spec fn reserved_atom(a: Seq<u8>) -> bool {
    (a.len() >= 10 && a.take(10) == ascii("guideline_"@)) || (a.len() >= 11 && a.take(11) == ascii(
        "$guideline_"@,
    ))
}

pub open spec fn lemma_of(t: Term) -> Option<Term> {
    match t {
        Term::Comp(name, args) => if (name == ascii("object"@) && args.len() == 6) || (name
            == ascii("predicate"@) && args.len() >= 3) || (name == ascii("property"@) && args.len()
            == 3) || (name == ascii("modifier_pp"@) && args.len() == 3) || (name == ascii(
            "modifier_adv"@,
        ) && args.len() == 3) {
            Option::Some(args[1])
        } else {
            Option::None
        },
        _ => Option::None,
    }
}

// Reserved vocabulary anywhere in the DRS: a functor or lemma with the KB prefix.
pub open spec fn collides(t: Term) -> bool
    decreases t, 0int,
{
    match t {
        Term::Comp(name, args) => reserved_atom(name) || (match lemma_of(t) {
            Option::Some(Term::Atom(l)) => reserved_atom(l),
            _ => false,
        }) || collides_all(args),
        _ => false,
    }
}

pub open spec fn collides_all(ts: Seq<Term>) -> bool
    decreases ts, 1int,
{
    ts.len() > 0 && (collides(ts[0]) || collides_all(ts.drop_first()))
}

// project(drs, docid, count) = the sentence bundles' groups, or the first
// sentence-attributed reason the fork's law admits no projection; `srcs` = each
// sentence's tokens + ACE line (formula literals, da2 D2).
pub open spec fn project(
    drs: Term,
    docid: Seq<u8>,
    count: nat,
    tab: Option<Vocabulary>,
    srcs: Seq<Src>,
) -> Result<Seq<Projected>, Term> {
    match box_parts(drs) {
        Option::None => Result::Err(atom("invalid_drs_shape"@)),
        Option::Some((_, conds)) => if collides(drs) {
            Result::Err(atom("reserved_name_collision"@))
        } else {
            match list_items(conds) {
                Option::None => Result::Err(atom("invalid_drs_shape"@)),
                Option::Some(cs) => match tags(cs) {
                    Option::None => Result::Err(atom("root_condition"@)),
                    Option::Some(tagged) => if !in_range(tagged, count) {
                        Result::Err(atom("condition_outside_sentence_range"@))
                    } else {
                        project_from(
                            tagged,
                            1,
                            count,
                            docid,
                            Seq::empty(),
                            nvars(drs),
                            env_of(tab, drs),
                            Seq::empty(),
                            srcs,
                        )
                    },
                },
            }
        },
    }
}

// --- payload: one ground obligation per group ---
pub open spec fn bind_all(t: Term, pairs: Seq<(Term, Term)>) -> Term
    decreases pairs.len(),
{
    if pairs.len() == 0 {
        t
    } else {
        match pairs[0].0 {
            Term::Var(x) => bind_all(subst(t, x, pairs[0].1), pairs.drop_first()),
            _ => bind_all(t, pairs.drop_first()),
        }
    }
}

// Witness facts: the positive body goals other than the comparison built-ins,
// which hold under the witness without a fact (nt1 D4).
pub open spec fn positive_goals(body: Seq<BodyItem>) -> Seq<Term> {
    pos_terms(body.filter(|it: BodyItem| it is Pos)).filter(|t: Term| !is_builtin_lit(t))
}

pub open spec fn obligation(g: Group, docid: Seq<u8>, s: nat) -> Ob {
    let facts = if g.clauses.len() == 0 {
        Seq::<Term>::empty()
    } else {
        positive_goals(g.clauses[0].body).map_values(|t: Term| bind_all(t, g.pairs))
    };
    Ob {
        docid: Term::Atom(docid),
        s: Term::Int(s as int),
        k: Term::Int(g.k as int),
        facts,
        heads: g.clauses.map_values(|c: DocClause| bind_all(c.head, g.pairs)),
    }
}

pub open spec fn obligations(ps: Seq<Projected>, docid: Seq<u8>) -> Seq<Ob> {
    ps.map_values(
        |p: Projected| p.groups.map_values(|g: Group| obligation(g, docid, p.s)),
    ).flatten()
}

pub open spec fn first_nonground(obs: Seq<Ob>) -> Option<Ob>
    decreases obs.len(),
{
    if obs.len() == 0 {
        Option::None
    } else if !ground(ob_term(obs[0])) {
        Option::Some(obs[0])
    } else {
        first_nonground(obs.drop_first())
    }
}

// --- custody + the document relation ---
// The table a certification projects with: none, or the parsed raw table
// supplied with its digest (contract c1 D8). None = a custody or grammar failure.
pub open spec fn table_of(traw: Option<Seq<u8>>, tsha: Option<Seq<u8>>) -> Option<
    Option<Vocabulary>,
> {
    match (traw, tsha) {
        (Option::None, Option::None) => Option::Some(Option::None),
        (Option::Some(b), Option::Some(_)) => match parse_vocabulary(b) {
            Result::Ok(t) => Option::Some(Option::Some(t)),
            Result::Err(_) => Option::None,
        },
        _ => Option::None,
    }
}

pub open spec fn nonempty_lines(bytes: Seq<u8>) -> Seq<Seq<u8>> {
    lines_of(bytes).filter(|l: Seq<u8>| l.len() > 0)
}

pub open spec fn sentence_count(d: Dump) -> Option<nat> {
    match list_items(d.sentences) {
        Option::Some(ss) => Option::Some(ss.len()),
        Option::None => Option::None,
    }
}

pub open spec fn ulex_of(usha: Option<Seq<u8>>) -> Option<Seq<u8>> {
    usha
}

// Each sentence's APE tokens beside its ACE line.
pub open spec fn srcs_of(sentences: Term, lines: Seq<Seq<u8>>) -> Seq<Src> {
    let ss = match list_items(sentences) {
        Option::Some(ss) => ss,
        Option::None => Seq::empty(),
    };
    Seq::new(
        lines.len(),
        |i: int|
            Src {
                toks: if i < ss.len() {
                    match list_items(ss[i]) {
                        Option::Some(ts) => ts,
                        Option::None => Seq::empty(),
                    }
                } else {
                    Seq::empty()
                },
                line: lines[i],
            },
    )
}

// Bundle s of the committed document = the projected groups' clauses,
// canonically numbered, under the ACE line as its marker text.
pub open spec fn bundle_matches(b: Bundle, p: Projected, line: Seq<u8>) -> bool {
    &&& b.s == p.s
    &&& b.text == line
    &&& b.clauses == (group_clauses(p.groups) + p.records).map_values(
        |c: DocClause| canon_clause(c),
    )
}

pub open spec fn first_mismatch(
    bs: Seq<Bundle>,
    ps: Seq<Projected>,
    lines: Seq<Seq<u8>>,
    i: nat,
) -> Option<nat>
    decreases bs.len() - i,
{
    if i >= bs.len() {
        Option::None
    } else if !bundle_matches(bs[i as int], ps[i as int], lines[i as int]) {
        Option::Some(i + 1)
    } else {
        first_mismatch(bs, ps, lines, i + 1)
    }
}

// certify_doc: Ok(obligations) iff the committed document is the projection
// of the DRS the driver derived from its quoted source.
pub open spec fn certify_doc(
    ace: Seq<u8>,
    asha: Seq<u8>,
    usha: Option<Seq<u8>>,
    docid: Seq<u8>,
    dump: Seq<u8>,
    pl: Seq<u8>,
    traw: Option<Seq<u8>>,
    tsha: Option<Seq<u8>>,
) -> Result<Seq<Ob>, Term> {
    if !accepts(pl) {
        Result::Err(atom("noncanonical"@))
    } else {
        match the_v1(pl) {
            V1File::Doc(doc) => if !dump_accepts(dump) {
                Result::Err(atom("dump_noncanonical"@))
            } else {
                let d = the_dump(dump);
                let lines = nonempty_lines(ace);
                if d.messages != Term::Nil {
                    Result::Err(atom("ape_messages"@))
                } else if sentence_count(d) != Option::Some(lines.len()) {
                    Result::Err(atom("sentence_lines"@))
                } else if doc.docid != docid {
                    Result::Err(atom("docid"@))
                } else if doc.ace != asha {
                    Result::Err(atom("ace_sha256"@))
                } else if doc.ulex != ulex_of(usha) {
                    Result::Err(atom("ulex"@))
                } else if doc.vocabulary != tsha || table_of(traw, tsha) is None {
                    Result::Err(atom("vocabulary"@))
                } else if doc.bundles.len() != lines.len() {
                    Result::Err(atom("bundle_count"@))
                } else {
                    match project(
                        d.drs,
                        docid,
                        lines.len(),
                        table_of(traw, tsha).unwrap(),
                        srcs_of(d.sentences, lines),
                    ) {
                        Result::Err(e) => Result::Err(Term::Comp(ascii("unsupported"@), seq![e])),
                        Result::Ok(ps) => match first_mismatch(doc.bundles, ps, lines, 0) {
                            Option::Some(s) => Result::Err(
                                Term::Comp(ascii("clauses"@), seq![Term::Int(s as int)]),
                            ),
                            Option::None => {
                                let obs = obligations(ps, docid);
                                match first_nonground(obs) {
                                    Option::Some(o) => Result::Err(
                                        Term::Comp(
                                            ascii("nonground_obligation"@),
                                            seq![
                                                o.s,
                                                variant_slot(0),  /* placeholder */
                                            ],
                                        ),
                                    ),
                                    Option::None => Result::Ok(obs),
                                }
                            },
                        },
                    }
                }
            },
            _ => Result::Err(atom("record_shape"@)),
        }
    }
}

// --- query projection (question mode) ---
pub open spec fn strip_anchor(c: Term) -> Term {
    match anchor_s(c) {
        Option::Some(_) => arg(c, 0),
        Option::None => c,
    }
}

// Table-backed questions also carry `of` links (m7t D9); `annotate` consumes them or they reject.
pub open spec fn supported_leaf(name: Seq<u8>, arity: nat, table: bool) -> bool {
    ||| (table && name == ascii("relation"@) && arity == 3)
    ||| (name == ascii("object"@) && arity == 6)
    ||| (name == ascii("predicate"@) && 3 <= arity <= 5)
    ||| (name == ascii("modifier_pp"@) && arity == 3)
    ||| (name == ascii("property"@) && arity == 3)
}

pub open spec fn wh_tag_ok(t: Term) -> bool {
    t == atom("who"@) || t == atom("which"@) || t == atom("what"@)
}

// Pre-order blocker scan: conjunctive supported leaves and nested modal boxes only.
// A table also admits a nested condition list (an upper-bound determiner's group; c1 D6: table presence gates both, as before).
pub open spec fn scan_box(b: Term, table: bool) -> bool
    decreases b, 0int,
{
    match box_parts(b) {
        Option::None => false,
        Option::Some((_, conds)) => scan_conds(conds, table),
    }
}

pub open spec fn scan_conds(l: Term, table: bool) -> bool
    decreases l, 1int,
{
    match l {
        Term::Nil => true,
        Term::Comp(name, args) => name == cons_name() && args.len() == 2 && scan_leaf(
            strip_anchor(args[0]),
            table,
        ) && scan_conds(args[1], table),
        _ => false,
    }
}

pub open spec fn scan_leaf(leaf: Term, table: bool) -> bool
    decreases leaf, 2int,
{
    match leaf {
        Term::Comp(name, args) => if table && name == cons_name() && args.len() == 2 {
            scan_conds(leaf, table)
        } else if is_modal(name) && args.len() == 1 {
            scan_box(args[0], table)
        } else if name == ascii("query"@) && args.len() == 2 {
            wh_tag_ok(args[1])
        } else {
            supported_leaf(name, args.len(), table)
        },
        _ => false,
    }
}

pub ghost struct Marker {
    pub r: Term,
    pub tag: Term,
    pub sources: Seq<
        (Term, Term, Term),
    >,  // same-box object sources (ref, noun, class)
}

pub open spec fn box_sources(l: Term) -> Seq<(Term, Term, Term)>
    decreases l,
{
    match l {
        Term::Comp(name, args) => if name == cons_name() && args.len() == 2 {
            let inner = strip_anchor(args[0]);
            (if is_comp(inner, "object"@, 6) {
                seq![(arg(inner, 0), arg(inner, 1), arg(inner, 2))]
            } else {
                Seq::empty()
            }) + box_sources(args[1])
        } else {
            Seq::empty()
        },
        _ => Seq::empty(),
    }
}

pub open spec fn box_markers(b: Term) -> Seq<Marker>
    decreases b, 0int,
{
    match box_parts(b) {
        Option::None => Seq::empty(),
        Option::Some((_, conds)) => conds_markers(conds, box_sources(conds)),
    }
}

pub open spec fn conds_markers(l: Term, sources: Seq<(Term, Term, Term)>) -> Seq<Marker>
    decreases l, 1int,
{
    match l {
        Term::Comp(name, args) => if name == cons_name() && args.len() == 2 {
            let inner = strip_anchor(args[0]);
            (if is_comp(inner, "query"@, 2) {
                seq![Marker { r: arg(inner, 0), tag: arg(inner, 1), sources }]
            } else if (match inner {
                Term::Comp(n, a) => is_modal(n) && a.len() == 1,
                _ => false,
            }) {
                box_markers(arg(inner, 0))
            } else {
                Seq::empty()
            }) + conds_markers(args[1], sources)
        } else {
            Seq::empty()
        },
        _ => Seq::empty(),
    }
}

pub open spec fn sources_of(ss: Seq<(Term, Term, Term)>, r: Term) -> Seq<(Term, Term, Term)> {
    ss.filter(|s: (Term, Term, Term)| s.0 == r)
}

// Exactly one same-box source names noun(Noun, Class); a source-less who/what
// gives wh(Tag); a source-less which rejects.
pub open spec fn marker_desc(m: Marker) -> Option<Term> {
    let ss = sources_of(m.sources, m.r);
    if ss.len() == 1 {
        Option::Some(Term::Comp(ascii("noun"@), seq![ss[0].1, ss[0].2]))
    } else if ss.len() > 1 {
        Option::None
    } else if m.tag == atom("who"@) || m.tag == atom("what"@) {
        Option::Some(Term::Comp(ascii("wh"@), seq![m.tag]))
    } else {
        Option::None
    }
}

pub open spec fn answers_of(ms: Seq<Marker>, seen: Seq<Term>) -> Option<Seq<Term>>
    decreases ms.len(),
{
    if ms.len() == 0 {
        Option::Some(Seq::empty())
    } else if !(ms[0].r is Var) || seen.contains(ms[0].r) {
        Option::None
    } else {
        match marker_desc(ms[0]) {
            Option::None => Option::None,
            Option::Some(d) => match answers_of(ms.drop_first(), seen.push(ms[0].r)) {
                Option::None => Option::None,
                Option::Some(rest) => Option::Some(
                    seq![Term::Comp(ascii("answer"@), seq![ms[0].r, d])] + rest,
                ),
            },
        }
    }
}

// Marker removal keeps every other condition term intact.
pub open spec fn strip_box(b: Term) -> Term
    decreases b, 0int,
{
    match box_parts(b) {
        Option::None => b,
        Option::Some((dom, conds)) => Term::Comp(
            drs_name(),
            seq![list_of(dom), strip_conds(conds)],
        ),
    }
}

pub open spec fn strip_conds(l: Term) -> Term
    decreases l, 1int,
{
    match l {
        Term::Comp(name, args) => if name == cons_name() && args.len() == 2 {
            let inner = strip_anchor(args[0]);
            let rest = strip_conds(args[1]);
            if is_comp(inner, "query"@, 2) {
                rest
            } else {
                match inner {
                    Term::Comp(n, a) => if is_modal(n) && a.len() == 1 && is_comp(a[0], "drs"@, 2) {
                        Term::Comp(cons_name(), seq![Term::Comp(n, seq![strip_box(a[0])]), rest])
                    } else {
                        Term::Comp(cons_name(), seq![args[0], rest])
                    },
                    _ => Term::Comp(cons_name(), seq![args[0], rest]),
                }
            }
        } else {
            l
        },
        _ => l,
    }
}

// project_query(drs, qid) = (goal conjunction, answers list) of the one question.
pub open spec fn project_query(drs: Term, qid: Seq<u8>, tab: Option<Vocabulary>) -> Result<
    (Term, Term),
    Term,
> {
    match box_parts(drs) {
        Option::None => Result::Err(atom("invalid_drs_shape"@)),
        Option::Some((dom, conds)) => if collides(drs) {
            Result::Err(atom("reserved_name_collision"@))
        } else if !(dom.len() == 0 && is_comp(conds, "[|]"@, 2) && arg(conds, 1) == Term::Nil
            && is_comp(arg(conds, 0), "question"@, 1)) {
            Result::Err(atom("query_root"@))
        } else {
            let q = arg(arg(conds, 0), 0);
            if inner_sentence(q) != Option::Some(1int) {
                Result::Err(atom("mixed_or_missing_sentence_anchors"@))
            } else if !scan_box(q, tab is Some) {
                Result::Err(atom("query_unsupported"@))
            } else {
                match answers_of(box_markers(q), Seq::empty()) {
                    Option::None => Result::Err(atom("query_marker"@)),
                    Option::Some(answers) => match box_parts(strip_box(q)) {
                        Option::None => Result::Err(atom("invalid_drs_shape"@)),
                        Option::Some((_, clean)) => match flatten_ann(
                            env_of(tab, drs),
                            clean,
                            Where::Antecedent,
                            1,
                            qid,
                            Term::Nil,
                            actual(),
                            Encl::Top,
                            1,
                            nvars(drs),
                        ) {
                            Result::Err(e) => Result::Err(e),
                            Result::Ok(f) => match expand(
                                compared(f.items, nvars(drs) + f.n),
                                Seq::empty(),
                                Seq::empty(),
                            ) {
                                Result::Err(e) => Result::Err(e),
                                Result::Ok(goals) => if goals.len() == 0 || !all_pos(goals) {
                                    Result::Err(atom("query_unsupported"@))
                                } else {
                                    let conj = conj_term(pos_terms(goals));
                                    if !(forall|i: int|
                                        0 <= i < answers.len() ==> vars_of(conj).contains(
                                            var_index(arg(#[trigger] answers[i], 0)),
                                        )) {
                                        Result::Err(atom("query_marker_unbound"@))
                                    } else {
                                        Result::Ok((conj, list_of(answers)))
                                    }
                                },
                            },
                        },
                    },
                }
            }
        },
    }
}

pub open spec fn var_index(t: Term) -> nat {
    match t {
        Term::Var(k) => k,
        _ => 0,
    }
}

// The query line: the single ACE line minus a trailing CR.
pub open spec fn query_text(line: Seq<u8>) -> Seq<u8> {
    if line.len() > 0 && line.last() == 0x0D {
        line.drop_last()
    } else {
        line
    }
}

// One numbervars pass over goal then answers.
pub open spec fn canon_pair(goal: Term, answers: Term) -> (Term, Term) {
    let fs = firsts(var_stream(goal) + var_stream(answers), Set::empty());
    (renumber(goal, fs), renumber(answers, fs))
}

pub open spec fn certify_query(
    ace: Seq<u8>,
    asha: Seq<u8>,
    usha: Option<Seq<u8>>,
    qid: Seq<u8>,
    dump: Seq<u8>,
    pl: Seq<u8>,
    traw: Option<Seq<u8>>,
    tsha: Option<Seq<u8>>,
) -> Result<(), Term> {
    if !accepts(pl) {
        Result::Err(atom("noncanonical"@))
    } else {
        match the_v1(pl) {
            V1File::Query(q) => if !dump_accepts(dump) {
                Result::Err(atom("dump_noncanonical"@))
            } else {
                let d = the_dump(dump);
                let lines = nonempty_lines(ace);
                if d.messages != Term::Nil {
                    Result::Err(atom("ape_messages"@))
                } else if sentence_count(d) != Option::Some(1nat) || lines.len() != 1 {
                    Result::Err(atom("query_sentences"@))
                } else if q.qid != qid {
                    Result::Err(atom("qid"@))
                } else if q.ace != asha {
                    Result::Err(atom("ace_sha256"@))
                } else if q.ulex != ulex_of(usha) {
                    Result::Err(atom("ulex"@))
                } else if q.vocabulary != tsha || table_of(traw, tsha) is None {
                    Result::Err(atom("vocabulary"@))
                } else if q.qtext != query_text(lines[0]) {
                    Result::Err(atom("query_text"@))
                } else {
                    match project_query(d.drs, qid, table_of(traw, tsha).unwrap()) {
                        Result::Err(e) => Result::Err(Term::Comp(ascii("unsupported"@), seq![e])),
                        Result::Ok((goal, answers)) => if canon_pair(goal, answers) != (
                            q.goal,
                            q.answers,
                        ) {
                            Result::Err(atom("projection"@))
                        } else {
                            Result::Ok(())
                        },
                    }
                }
            },
            _ => Result::Err(atom("record_shape"@)),
        }
    }
}

// --- exec-facing mirror ---
pub open spec fn opt_view(u: Option<&Vec<u8>>) -> Option<Seq<u8>> {
    match u {
        Option::Some(v) => Option::Some(v@),
        Option::None => Option::None,
    }
}

// --- outputs (the `ckc certify` envelope) ---
pub open spec fn certify_reject(id: Seq<u8>, why: Term) -> Out {
    Out {
        rc: 1,
        out: Seq::empty(),
        err: ascii("ckc: certify: "@) + id + ascii(": "@) + term_line(why),
    }
}

pub open spec fn certify_doc_output(
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
        Result::Ok(obs) => ok(print_payload(obs)),  // stdout = the derived obligations
    }
}

pub open spec fn certify_query_output(
    ace: Seq<u8>,
    asha: Seq<u8>,
    usha: Option<Seq<u8>>,
    qid: Seq<u8>,
    dump: Seq<u8>,
    pl: Seq<u8>,
    traw: Option<Seq<u8>>,
    tsha: Option<Seq<u8>>,
) -> Out {
    match certify_query(ace, asha, usha, qid, dump, pl, traw, tsha) {
        Result::Err(why) => certify_reject(qid, why),
        Result::Ok(()) => ok(ascii("ckc: certify ok "@) + qid + seq![0x0Au8]),
    }
}

} // verus!
