use crate::answers::*;
use crate::engine::*;
use crate::replay::*;
use crate::term::*;
use crate::trace::*;
use crate::v1text::*;
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

// Root condition classes (document mode): anchored fact, rule, operator box.
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

pub open spec fn card_op_ok(op: Term) -> bool {
    ||| op == atom("eq"@)
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
        } else if name == ascii("predicate"@) && args.len() >= 3 {
            (args.len() - 1) as nat
        } else {
            1
        },
        _ => 1,
    }
}

// --- groups: fact clusters and rule variants ---
pub ghost struct Group {
    pub k: nat,  // variant ordinal (1 for facts)
    pub pairs: Seq<(Term, Term)>,  // witness bindings
    pub clauses: Seq<DocClause>,
}

pub open spec fn fact_clause(h: Term) -> DocClause {
    DocClause { head: h, body: Seq::empty() }
}

pub open spec fn rule_clauses(heads: Seq<Term>, body: Seq<BodyItem>) -> Seq<DocClause> {
    heads.map_values(|h: Term| DocClause { head: h, body })
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
) -> Result<(Group, Seq<(Term, Term)>), Term> {
    match roots_conds(roots) {
        Option::None => Result::Err(atom("sentence_shape"@)),
        Option::Some(conds) => match flatten_list(
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
pub open spec fn flatten_seq(cs: Seq<Term>, s: nat, docid: Seq<u8>, n: nat, base: nat) -> Result<
    Flat,
    Term,
> {
    flatten_list(list_of(cs), Where::Antecedent, s, docid, Term::Nil, actual(), Encl::Top, n, base)
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
            Item::Anch(_, inner) => witness_refs(cond_refs(inner), ordered, map, docid, s, k, acc),
        };
        witness_pairs(items.drop_first(), ordered, map, docid, s, k, acc1)
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

// One Horn variant: body = antecedent items, one clause per consequent head.
pub open spec fn variant(
    aitems: Seq<Item>,
    citems: Seq<Item>,
    ordered: Seq<Term>,
    map: Seq<(Term, Term)>,
    deps: Term,
    s: nat,
    docid: Seq<u8>,
    k: nat,
) -> Result<Group, Term> {
    let ante_refs = first_vars(ref_slots(aitems), Seq::empty());
    let sko = skolem(cons_locals(ordered, ante_refs, map), ordered, deps, s, docid);
    match (expand(aitems, map, sko), expand(citems, map, sko)) {
        (Result::Ok(goals), Result::Ok(heads)) => if goals.len() == 0 {
            Result::Err(atom("rule_without_antecedent"@))
        } else if heads.len() == 0 || !all_pos(heads) {
            Result::Err(atom("rule_without_consequent"@))
        } else {
            let cs = rule_clauses(pos_terms(heads), goals);
            if !all_naf_safe(cs, body_doms(aitems)) {
                Result::Err(atom("naf_safety"@))
            } else if !all_head_safe(cs) {
                Result::Err(atom("head_variable_not_bound_in_body"@))
            } else {
                Result::Ok(
                    Group {
                        k,
                        pairs: witness_pairs(aitems, ordered, map, docid, s, k, Seq::empty()),
                        clauses: cs,
                    },
                )
            }
        },
        (Result::Err(e), _) => Result::Err(e),
        (_, Result::Err(e)) => Result::Err(e),
    }
}

pub open spec fn flatten_cons(
    cconds: Term,
    s: nat,
    docid: Seq<u8>,
    deps: Term,
    n: nat,
    base: nat,
) -> Result<Flat, Term> {
    flatten_list(cconds, Where::Consequent, s, docid, deps, actual(), Encl::Top, n, base)
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
) -> Result<Seq<Group>, Term> {
    match curry(ante, cons, 64) {
        Result::Err(e) => Result::Err(e),
        Result::Ok((segs, fcons)) => match box_parts(fcons) {
            Option::None => Result::Err(atom("invalid_drs_shape"@)),
            Option::Some((_, cconds)) => {
                let adom = seg_domain(segs);
                match split_scan(segs) {
                    Option::None => Result::Err(atom("invalid_drs_shape"@)),
                    Option::Some((shared, vs)) => if vs.len() == 0 {
                        match flatten_seq(shared, s, docid, 1, base) {
                            Result::Err(e) => Result::Err(e),
                            Result::Ok(fa) => match flatten_cons(
                                cconds,
                                s,
                                docid,
                                list_of(adom),
                                fa.n,
                                base,
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
) -> Result<Seq<Group>, Term> {
    match flatten_seq(shared, s, docid, 1, base) {
        Result::Err(e) => Result::Err(e),
        Result::Ok(fs) => match flatten_list(
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
            Result::Ok(f1) => match flatten_list(
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
                        flatten_cons(cconds, s, docid, deps1, f2.n, base),
                        flatten_cons(cconds, s, docid, deps2, f2.n, base),
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
) -> Result<(Seq<Group>, Seq<(Term, Term)>), Term> {
    if roots.len() == 1 && roots[0] is Rule {
        match roots[0] {
            Root::Rule(a, c) => match rule_groups(a, c, s, docid, map, base) {
                Result::Err(e) => Result::Err(e),
                Result::Ok(gs) => Result::Ok((gs, map)),
            },
            _ => Result::Err(atom("sentence_shape"@)),
        }
    } else if forall|i: int| 0 <= i < roots.len() ==> is_fact_root(#[trigger] roots[i]) {
        match fact_group(roots, s, docid, map, base) {
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
}

pub open spec fn err_at(s: nat, why: Term) -> Term {
    Term::Comp(ascii("sentence"@), seq![Term::Int(s as int), why])
}

pub open spec fn project_from(
    tagged: Seq<(int, Root)>,
    s: nat,
    count: nat,
    docid: Seq<u8>,
    map: Seq<(Term, Term)>,
    base: nat,
) -> Result<Seq<Projected>, Term>
    decreases count + 1 - s,
{
    if s > count {
        Result::Ok(Seq::empty())
    } else {
        match sentence_groups(of_sentence(tagged, s as int), s, docid, map, base) {
            Result::Err(e) => Result::Err(err_at(s, e)),
            Result::Ok((gs, map2)) => if group_clauses(gs).len() == 0 {
                Result::Err(err_at(s, atom("sentence_shape"@)))
            } else {
                match project_from(tagged, s + 1, count, docid, map2, base) {
                    Result::Err(e) => Result::Err(e),
                    Result::Ok(rest) => Result::Ok(seq![Projected { s, groups: gs }] + rest),
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
            == 3) || (name == ascii("modifier_pp"@) && args.len() == 3) {
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
// sentence-attributed reason the fork's law admits no projection.
pub open spec fn project(drs: Term, docid: Seq<u8>, count: nat) -> Result<Seq<Projected>, Term> {
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
                        project_from(tagged, 1, count, docid, Seq::empty(), nvars(drs))
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

pub open spec fn positive_goals(body: Seq<BodyItem>) -> Seq<Term> {
    pos_terms(body.filter(|it: BodyItem| it is Pos))
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

// Bundle s of the committed document = the projected groups' clauses,
// canonically numbered, under the ACE line as its marker text.
pub open spec fn bundle_matches(b: Bundle, p: Projected, line: Seq<u8>) -> bool {
    &&& b.s == p.s
    &&& b.text == line
    &&& b.clauses == group_clauses(p.groups).map_values(|c: DocClause| canon_clause(c))
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
                } else if doc.bundles.len() != lines.len() {
                    Result::Err(atom("bundle_count"@))
                } else {
                    match project(d.drs, docid, lines.len()) {
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

pub open spec fn supported_leaf(name: Seq<u8>, arity: nat) -> bool {
    ||| (name == ascii("object"@) && arity == 6)
    ||| (name == ascii("predicate"@) && 3 <= arity <= 5)
    ||| (name == ascii("modifier_pp"@) && arity == 3)
    ||| (name == ascii("property"@) && arity == 3)
}

pub open spec fn wh_tag_ok(t: Term) -> bool {
    t == atom("who"@) || t == atom("which"@) || t == atom("what"@)
}

// Pre-order blocker scan: conjunctive supported leaves and nested modal boxes only.
pub open spec fn scan_box(b: Term) -> bool
    decreases b, 0int,
{
    match box_parts(b) {
        Option::None => false,
        Option::Some((_, conds)) => scan_conds(conds),
    }
}

pub open spec fn scan_conds(l: Term) -> bool
    decreases l, 1int,
{
    match l {
        Term::Nil => true,
        Term::Comp(name, args) => name == cons_name() && args.len() == 2 && scan_leaf(
            strip_anchor(args[0]),
        ) && scan_conds(args[1]),
        _ => false,
    }
}

pub open spec fn scan_leaf(leaf: Term) -> bool
    decreases leaf, 2int,
{
    match leaf {
        Term::Comp(name, args) => if is_modal(name) && args.len() == 1 {
            scan_box(args[0])
        } else if name == ascii("query"@) && args.len() == 2 {
            wh_tag_ok(args[1])
        } else {
            supported_leaf(name, args.len())
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
pub open spec fn project_query(drs: Term, qid: Seq<u8>) -> Result<(Term, Term), Term> {
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
            } else if !scan_box(q) {
                Result::Err(atom("query_unsupported"@))
            } else {
                match answers_of(box_markers(q), Seq::empty()) {
                    Option::None => Result::Err(atom("query_marker"@)),
                    Option::Some(answers) => match box_parts(strip_box(q)) {
                        Option::None => Result::Err(atom("invalid_drs_shape"@)),
                        Option::Some((_, clean)) => match flatten_list(
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
                            Result::Ok(f) => match expand(f.items, Seq::empty(), Seq::empty()) {
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
                } else if q.qtext != query_text(lines[0]) {
                    Result::Err(atom("query_text"@))
                } else {
                    match project_query(d.drs, qid) {
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
) -> Out {
    match certify_doc(ace, asha, usha, docid, dump, pl) {
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
) -> Out {
    match certify_query(ace, asha, usha, qid, dump, pl) {
        Result::Err(why) => certify_reject(qid, why),
        Result::Ok(()) => ok(ascii("ckc: certify ok "@) + qid + seq![0x0Au8]),
    }
}

} // verus!
