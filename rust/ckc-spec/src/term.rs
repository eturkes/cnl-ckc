use vstd::prelude::*;

verus! {

// Trusted spec: the v1 term model (contract m5u2 K1). Atom/functor names
// are UTF-8 bytes (validation = shell boundary). Floats are excluded from
// the grammar by ruling R21; rationals/strings/back-quotes never had a v1
// spelling. Nil is SWI's reserved '[]' constant, distinct from the atom
// named "[]" (probe: that atom prints quoted '[]'). Var(k) = numbervar
// index k, printed A..Z,A1..Z1,A2..; the legacy writers number every
// emitted line with numbervars(Copy,0,_), so a canonical line's distinct
// variables appear in first-occurrence order 0,1,2,.. (var_canonical).

pub ghost enum Term {
    Var(nat),
    Int(int),
    Nil,
    Atom(Seq<u8>),
    Comp(Seq<u8>, Seq<Term>),
}

// --- structural laws ---

// Compounds carry at least one argument (arity 0 is an atom).
pub open spec fn wf_term(t: Term) -> bool
    decreases t,
{
    match t {
        Term::Comp(_, args) => args.len() >= 1 && wf_terms(args),
        _ => true,
    }
}

pub open spec fn wf_terms(ts: Seq<Term>) -> bool
    decreases ts,
{
    if ts.len() == 0 {
        true
    } else {
        wf_term(ts[0]) && wf_terms(ts.drop_first())
    }
}

pub open spec fn ground(t: Term) -> bool
    decreases t,
{
    match t {
        Term::Var(_) => false,
        Term::Comp(_, args) => ground_all(args),
        _ => true,
    }
}

pub open spec fn ground_all(ts: Seq<Term>) -> bool
    decreases ts,
{
    if ts.len() == 0 {
        true
    } else {
        ground(ts[0]) && ground_all(ts.drop_first())
    }
}

// '$VAR'/1 compounds are banned wherever the legacy writer runs with
// numbervars(true) (doc/query/answers lines): the writer would print them
// as variable letters, which cannot round-trip. Trace lines print them
// structurally (their writer omits the numbervars option), so the traces
// class admits them.
pub open spec fn dollar_var_name() -> Seq<u8> {
    seq![0x24u8, 0x56u8, 0x41u8, 0x52u8]  // "$VAR"
}

pub open spec fn no_dollar_var(t: Term) -> bool
    decreases t,
{
    match t {
        Term::Comp(name, args) => !(name == dollar_var_name() && args.len() == 1)
            && no_dollar_var_all(args),
        _ => true,
    }
}

pub open spec fn no_dollar_var_all(ts: Seq<Term>) -> bool
    decreases ts,
{
    if ts.len() == 0 {
        true
    } else {
        no_dollar_var(ts[0]) && no_dollar_var_all(ts.drop_first())
    }
}

// --- numbervar canonicality ---

// Depth-first left-to-right stream of variable indices, duplicates kept —
// exactly the order numbervars/3 visits fresh variables.
pub open spec fn var_stream(t: Term) -> Seq<nat>
    decreases t,
{
    match t {
        Term::Var(k) => seq![k],
        Term::Comp(_, args) => var_stream_all(args),
        _ => Seq::empty(),
    }
}

pub open spec fn var_stream_all(ts: Seq<Term>) -> Seq<nat>
    decreases ts,
{
    if ts.len() == 0 {
        Seq::empty()
    } else {
        var_stream(ts[0]) + var_stream_all(ts.drop_first())
    }
}

pub open spec fn firsts(s: Seq<nat>, seen: Set<nat>) -> Seq<nat>
    decreases s.len(),
{
    if s.len() == 0 {
        Seq::empty()
    } else if seen.contains(s[0]) {
        firsts(s.drop_first(), seen)
    } else {
        seq![s[0]] + firsts(s.drop_first(), seen.insert(s[0]))
    }
}

// A line's variables are canonically numbered when the first occurrences
// read 0,1,2,.. in stream order.
pub open spec fn var_canonical(stream: Seq<nat>) -> bool {
    firsts(stream, Set::empty()) == Seq::new(
        firsts(stream, Set::empty()).len(),
        |i: int| i as nat,
    )
}

} // verus!
