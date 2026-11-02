#![allow(dead_code)]
use vstd::prelude::*;

pub mod controls;
pub mod grounding;
pub mod origins;
pub mod rank;
pub mod slice;
pub mod stratified;
pub mod support;

verus! {

pub type Atom = int;

// One ground rule. Each NAF item lists all valuations of its correlated
// conjunction; a valuation is the set of atoms that jointly witness it.
pub struct Instance {
    pub head: Atom,
    pub positive: Set<Atom>,
    pub naf: Seq<Seq<Set<Atom>>>,
    pub builtins: bool,
}

pub struct Program {
    pub atoms: Set<Atom>,
    pub rules: Seq<Instance>,
}

pub open spec fn wf(p: Program) -> bool {
    forall|i: int|
        #![trigger p.rules[i]]
        0 <= i < p.rules.len() ==> {
            let r = p.rules[i];
            &&& p.atoms.contains(r.head)
            &&& r.positive.subset_of(p.atoms)
            &&& forall|j: int, k: int|
                0 <= j < r.naf.len() && 0 <= k < r.naf[j].len() ==> r.naf[j][k].subset_of(p.atoms)
        }
}

pub open spec fn retained(r: Instance, m: Set<Atom>) -> bool {
    r.builtins && forall|j: int, k: int|
        0 <= j < r.naf.len() && 0 <= k < r.naf[j].len() ==> !r.naf[j][k].subset_of(m)
}

pub open spec fn closed(p: Program, m: Set<Atom>, x: Set<Atom>) -> bool {
    forall|i: int|
        0 <= i < p.rules.len() && retained(p.rules[i], m) && p.rules[i].positive.subset_of(x)
            ==> x.contains(p.rules[i].head)
}

// The intersection of all models of the positive reduct. support::lfp_fixed
// proves that this is a fixed point of the immediate-consequence operator,
// not only a minimal pre-fixed point.
pub open spec fn lfp(p: Program, m: Set<Atom>) -> Set<Atom> {
    p.atoms.filter(|a: Atom| forall|x: Set<Atom>| closed(p, m, x) ==> x.contains(a))
}

pub open spec fn immediate(p: Program, m: Set<Atom>, x: Set<Atom>) -> Set<Atom> {
    p.atoms.filter(
        |a: Atom|
            exists|i: int|
                0 <= i < p.rules.len() && p.rules[i].head == a && retained(p.rules[i], m)
                    && p.rules[i].positive.subset_of(x),
    )
}

pub open spec fn stable(p: Program, m: Set<Atom>) -> bool {
    m == lfp(p, m)
}

pub open spec fn founded(p: Program, m: Set<Atom>, level: spec_fn(Atom) -> nat) -> bool {
    forall|a: Atom|
        m.contains(a) ==> exists|i: int|
            0 <= i < p.rules.len() && p.rules[i].head == a && retained(p.rules[i], m)
                && p.rules[i].positive.subset_of(m) && forall|b: Atom|
                p.rules[i].positive.contains(b) ==> level(b) < level(a)
}

} // verus!
