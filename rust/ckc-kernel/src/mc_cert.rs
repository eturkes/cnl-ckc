use crate::k2_engine::{EBodyItem, EClause};
use crate::k2_term::{ENode, ETermArena};
use crate::mc_model::EMcModel;
use ckc_spec::model::{Certificate, Claim, EMcCertificate};
use ckc_spec::term::Term;
use ckc_spec::v1text::BodyItem;
use vstd::prelude::*;

verus! {

// Contract mc D6 + § Answers: the certificate, decoded to arena terms, and its claim checks.
pub struct EMcDClaim {
    pub fact: usize,
    pub clause: usize,
    pub vals: Vec<(usize, usize)>,
    pub level: u64,
}

pub open spec fn dclaim_ok(nodes: Seq<ENode>, d: EMcDClaim, cl: Claim) -> bool {
    &&& d.fact < nodes.len()
    &&& nodes[d.fact as int].term@ == cl.fact
    &&& ckc_spec::term::ground(cl.fact)
    &&& cl.clause == d.clause as nat
    &&& cl.level == d.level as nat
    &&& forall|a: int|
        0 <= a < d.vals@.len() ==> {
            &&& (#[trigger] d.vals@[a]).1 < nodes.len()
            &&& cl.valuation.contains_key(d.vals@[a].0 as nat)
            &&& cl.valuation[d.vals@[a].0 as nat] == nodes[d.vals@[a].1 as int].term@
        }
    &&& forall|x: nat| #[trigger]
        cl.valuation.contains_key(x) <==> exists|a: int|
            0 <= a < d.vals@.len() && (#[trigger] d.vals@[a]).0 as nat == x
    &&& forall|x: nat| #[trigger]
        cl.valuation.contains_key(x) ==> ckc_spec::term::ground(cl.valuation[x])
            && ckc_spec::term::wf_term(cl.valuation[x])
}

pub open spec fn dcert_ok(nodes: Seq<ENode>, ds: Seq<EMcDClaim>, c: Certificate) -> bool {
    &&& ds.len() == c.claims.len()
    &&& forall|k: int| 0 <= k < ds.len() ==> #[trigger] dclaim_ok(nodes, ds[k], c.claims[k])
}

pub proof fn dcert_prefix(before: Seq<ENode>, after: Seq<ENode>, ds: Seq<EMcDClaim>, c: Certificate)
    requires
        before.is_prefix_of(after),
        dcert_ok(before, ds, c),
    ensures
        dcert_ok(after, ds, c),
{
    assert forall|k: int| 0 <= k < ds.len() implies #[trigger] dclaim_ok(
        after,
        ds[k],
        c.claims[k],
    ) by {
        assert(dclaim_ok(before, ds[k], c.claims[k]));
        assert(after[ds[k].fact as int] == before[ds[k].fact as int]);
        assert forall|a: int| 0 <= a < ds[k].vals@.len() implies {
            &&& (#[trigger] ds[k].vals@[a]).1 < after.len()
            &&& c.claims[k].valuation.contains_key(ds[k].vals@[a].0 as nat)
            &&& c.claims[k].valuation[ds[k].vals@[a].0 as nat]
                == after[ds[k].vals@[a].1 as int].term@
        } by {
            assert(after[ds[k].vals@[a].1 as int] == before[ds[k].vals@[a].1 as int]);
        }
    }
}

// var is already bound in vals.
fn has_var(vals: &Vec<(usize, usize)>, var: usize) -> (out: bool)
    ensures
        out == exists|i: int| 0 <= i < vals@.len() && (#[trigger] vals@[i]).0 == var,
{
    let mut i = 0usize;
    while i < vals.len()
        invariant
            i <= vals@.len(),
            forall|i2: int| 0 <= i2 < i ==> (#[trigger] vals@[i2]).0 != var,
        decreases vals.len() - i,
    {
        if vals[i].0 == var {
            return true;
        }
        i += 1;
    }
    false
}

// Decodes every claim (Err(k) = the first claim that does not decode).
pub fn decode_cert(arena: &mut ETermArena, cert: &EMcCertificate) -> (out: Result<
    (Vec<EMcDClaim>, Ghost<Certificate>),
    usize,
>)
    requires
        crate::k2_term::arena_ok(old(arena)),
    ensures
        crate::k2_term::arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out matches Ok((ds, c)) ==> {
            &&& ckc_spec::model::cert_decodes(cert@, c@)
            &&& dcert_ok(final(arena).nodes@, ds@, c@)
        },
{
    let ghost origin = arena.nodes@;
    let ghost mut claims: Seq<Claim> = Seq::empty();
    let mut ds: Vec<EMcDClaim> = Vec::new();
    proof {
        assert(origin.is_prefix_of(origin)) by {
            assert(origin.take(origin.len() as int) =~= origin);
        }
    }
    let mut k = 0usize;
    while k < cert.claims.len()
        invariant
            crate::k2_term::arena_ok(arena),
            origin == old(arena).nodes@,
            origin.is_prefix_of(arena.nodes@),
            k <= cert.claims@.len(),
            claims.len() == k,
            dcert_ok(arena.nodes@, ds@, Certificate { identity: cert.identity@, claims }),
            forall|q: int|
                0 <= q < k ==> ckc_spec::model::claim_decodes(
                    #[trigger] cert@.claims[q],
                    claims[q],
                ),
        decreases cert.claims.len() - k,
    {
        let ct = &cert.claims[k];
        let ghost before = arena.nodes@;
        let fact = crate::mc_term::decode_term(arena, &ct.fact);
        let ghost after_fact = arena.nodes@;
        proof {
            crate::k2_load::prefix_chain(origin, before, after_fact);
        }
        let fact = match fact {
            Some(r) => r,
            None => return Err(k),
        };
        let mut vals: Vec<(usize, usize)> = Vec::new();
        let ghost mut th: Map<nat, Term> = Map::empty();
        let mut a = 0usize;
        while a < ct.valuation.len()
            invariant
                crate::k2_term::arena_ok(arena),
                origin == old(arena).nodes@,
                origin.is_prefix_of(arena.nodes@),
                after_fact.is_prefix_of(arena.nodes@),
                fact < after_fact.len(),
                a <= ct.valuation@.len(),
                vals@.len() == a,
                forall|i: int|
                    0 <= i < a ==> {
                        &&& (#[trigger] vals@[i]).0 == ct.valuation@[i].var
                        &&& vals@[i].1 < arena.nodes@.len()
                        &&& th.contains_key(vals@[i].0 as nat)
                        &&& th[vals@[i].0 as nat] == arena.nodes@[vals@[i].1 as int].term@
                        &&& ckc_spec::model::term_decodes(
                            ct.valuation@[i].term@,
                            th[vals@[i].0 as nat],
                        )
                    },
                forall|i: int, j: int|
                    0 <= i < j < a ==> (#[trigger] vals@[i]).0 != (#[trigger] vals@[j]).0,
                forall|x: nat| #[trigger]
                    th.contains_key(x) <==> exists|i: int|
                        0 <= i < a && (#[trigger] vals@[i]).0 as nat == x,
            decreases ct.valuation.len() - a,
        {
            let var = ct.valuation[a].var;
            if has_var(&vals, var) {
                return Err(k);
            }
            let ghost before_v = arena.nodes@;
            let ghost vals0 = vals@;
            let ghost th0 = th;
            let r = match crate::mc_term::decode_term(arena, &ct.valuation[a].term) {
                Some(r) => r,
                None => {
                    proof {
                        crate::k2_load::prefix_chain(origin, before_v, arena.nodes@);
                    }
                    return Err(k);
                },
            };
            proof {
                crate::k2_load::prefix_chain(origin, before_v, arena.nodes@);
                crate::k2_load::prefix_chain(after_fact, before_v, arena.nodes@);
                assert forall|i2: int| 0 <= i2 < a implies arena.nodes@[vals0[i2].1 as int]
                    == before_v[vals0[i2].1 as int] by {}
                th = th.insert(var as nat, arena@[r as int]);
            }
            vals.push((var, r));
            proof {
                assert(vals@[a as int] == (var, r));
                assert forall|i2: int| 0 <= i2 < a implies vals@[i2] == vals0[i2] by {}
                assert forall|i2: int| 0 <= i2 < a + 1 implies {
                    &&& (#[trigger] vals@[i2]).0 == ct.valuation@[i2].var
                    &&& vals@[i2].1 < arena.nodes@.len()
                    &&& th.contains_key(vals@[i2].0 as nat)
                    &&& th[vals@[i2].0 as nat] == arena.nodes@[vals@[i2].1 as int].term@
                    &&& ckc_spec::model::term_decodes(
                        ct.valuation@[i2].term@,
                        th[vals@[i2].0 as nat],
                    )
                } by {
                    if i2 < a {
                        assert(vals0[i2].0 != var);
                        assert(th0.contains_key(vals0[i2].0 as nat));
                    }
                }
                assert forall|i2: int, j2: int| 0 <= i2 < j2 < a + 1 implies (
                #[trigger] vals@[i2]).0 != (#[trigger] vals@[j2]).0 by {
                    if j2 == a {
                        assert(vals0[i2].0 != var);
                    }
                }
                assert forall|x: nat| #[trigger]
                    th.contains_key(x) <==> exists|i2: int|
                        0 <= i2 < a + 1 && (#[trigger] vals@[i2]).0 as nat == x by {
                    if x == var as nat {
                        assert(vals@[a as int].0 as nat == x);
                    } else {
                        if th0.contains_key(x) {
                            let i0 = choose|i0: int|
                                0 <= i0 < a && (#[trigger] vals0[i0]).0 as nat == x;
                            assert(vals@[i0] == vals0[i0]);
                        }
                        if exists|i2: int| 0 <= i2 < a + 1 && (#[trigger] vals@[i2]).0 as nat == x {
                            let i0 = choose|i0: int|
                                0 <= i0 < a + 1 && (#[trigger] vals@[i0]).0 as nat == x;
                            assert(i0 != a);
                            assert(vals0[i0].0 as nat == x);
                        }
                    }
                }
            }
            a += 1;
        }
        let ghost cl = Claim {
            fact: arena@[fact as int],
            clause: ct.clause as nat,
            valuation: th,
            level: ct.level as nat,
        };
        proof {
            assert(arena.nodes@[fact as int] == after_fact[fact as int]);
            assert(ckc_spec::model::term_decodes(ct.fact@, cl.fact));
            let v = cert@.claims[k as int].valuation;
            assert(cert@.claims[k as int] == cert.claims@[k as int]@);
            assert(v.len() == a);
            assert forall|i: int| 0 <= i < v.len() implies #[trigger] v[i] == (
                ct.valuation@[i].var as nat,
                ct.valuation@[i].term@,
            ) by {}
            assert forall|i: int, j: int| 0 <= i < j < v.len() implies (#[trigger] v[i]).0 != (
            #[trigger] v[j]).0 by {
                assert(vals@[i].0 != vals@[j].0);
            }
            v.map_values(|b: (nat, Seq<u8>)| b.0).to_set_ensures();
            assert forall|x: nat| #[trigger]
                th.dom().contains(x) == v.map_values(|b: (nat, Seq<u8>)| b.0).to_set().contains(
                    x,
                ) by {
                if th.contains_key(x) {
                    let i = choose|i: int| 0 <= i < a && (#[trigger] vals@[i]).0 as nat == x;
                    assert(v.map_values(|b: (nat, Seq<u8>)| b.0)[i] == x);
                }
                let vm = v.map_values(|b: (nat, Seq<u8>)| b.0);
                if vm.contains(x) {
                    let i = choose|i: int| 0 <= i < vm.len() && #[trigger] vm[i] == x;
                    assert(vals@[i].0 as nat == x);
                }
            }
            assert(th.dom() =~= v.map_values(|b: (nat, Seq<u8>)| b.0).to_set());
            assert forall|i: int| 0 <= i < v.len() implies ckc_spec::model::term_decodes(
                (#[trigger] v[i]).1,
                th[v[i].0],
            ) by {
                assert(vals@[i].0 == ct.valuation@[i].var);
            }
            assert(ckc_spec::model::claim_decodes(cert@.claims[k as int], cl));
            assert forall|x: nat| #[trigger] th.contains_key(x) implies ckc_spec::term::ground(
                th[x],
            ) && ckc_spec::term::wf_term(th[x]) by {
                let i = choose|i: int| 0 <= i < a && (#[trigger] vals@[i]).0 as nat == x;
            }
            crate::mc_cert::dcert_prefix(
                before,
                arena.nodes@,
                ds@,
                Certificate { identity: cert.identity@, claims },
            );
        }
        let d = EMcDClaim { fact, clause: ct.clause, vals, level: ct.level };
        proof {
            let c1 = Certificate { identity: cert.identity@, claims: claims.push(cl) };
            assert(dclaim_ok(arena.nodes@, d, cl));
            assert forall|q: int| 0 <= q < ds@.len() + 1 implies #[trigger] dclaim_ok(
                arena.nodes@,
                ds@.push(d)[q],
                c1.claims[q],
            ) by {
                if q < ds@.len() {
                    assert(dclaim_ok(arena.nodes@, ds@[q], claims[q]));
                }
            }
            assert forall|q: int| 0 <= q < k + 1 implies ckc_spec::model::claim_decodes(
                #[trigger] cert@.claims[q],
                claims.push(cl)[q],
            ) by {
                if q < k {
                    assert(claims.push(cl)[q] == claims[q]);
                }
            }
            claims = claims.push(cl);
        }
        ds.push(d);
        k += 1;
    }
    let ghost c = Certificate { identity: cert.identity@, claims };
    Ok((ds, Ghost(c)))
}

// nonlocal(c, x): x occurs in the head, or in a positive item, or in two different items.
pub fn nonlocal_exec(arena: &ETermArena, clause: &EClause, x: usize) -> (out: bool)
    requires
        crate::k2_term::arena_ok(arena),
        crate::k2_engine::clause_valid(arena.nodes@, clause),
    ensures
        out == ckc_spec::model::nonlocal(
            crate::k2_engine::clause_view(arena.nodes@, clause),
            x as nat,
        ),
{
    let ghost c = crate::k2_engine::clause_view(arena.nodes@, clause);
    proof {
        crate::mc_safe::vars_occurs(c.head, x as nat);
    }
    if crate::k2_engine::occurs_root(arena, x, clause.head) {
        return true;
    }
    let mut j = 0usize;
    while j < clause.body.len()
        invariant
            crate::k2_term::arena_ok(arena),
            crate::k2_engine::clause_valid(arena.nodes@, clause),
            c == crate::k2_engine::clause_view(arena.nodes@, clause),
            !ckc_spec::model::vars(c.head).contains(x as nat),
            j <= clause.body@.len(),
            forall|q: int|
                0 <= q < j ==> !(#[trigger] ckc_spec::model::item_vars(c.body[q]).contains(x as nat)
                    && (c.body[q] is Pos || ckc_spec::model::outside(c, q, x as nat))),
        decreases clause.body.len() - j,
    {
        proof {
            assert(crate::k2_engine::body_item_valid(arena.nodes@, &clause.body@[j as int]));
            assert(c.body[j as int] == crate::k2_engine::body_item_view(
                arena.nodes@,
                &clause.body@[j as int],
            ));
        }
        if crate::mc_safe::occ_item(arena, &clause.body[j], x) {
            let pos = match &clause.body[j] {
                EBodyItem::Pos { .. } => true,
                EBodyItem::Naf { .. } => false,
            };
            if pos || crate::mc_safe::outside_exec(arena, clause, j, x) {
                return true;
            }
        }
        j += 1;
    }
    false
}

// No extension of th satisfies the conjunction ⇒ the negation holds under th (th ground-valued).
pub proof fn naf_of_no_witness(m: ISet<Term>, gs: Seq<Term>, th: Map<nat, Term>)
    requires
        forall|k: nat| #[trigger] th.contains_key(k) ==> ckc_spec::term::ground(th[k]),
        !exists|t3: Map<nat, Term>|
            crate::mc_subst::extends(t3, th) && crate::mc_model::goals_sat(m, gs, 0, t3),
    ensures
        ckc_spec::model::naf_holds(m, gs, th),
{
    if !ckc_spec::model::naf_holds(m, gs, th) {
        let s = choose|s: Map<nat, Term>| #[trigger]
            ckc_spec::model::conj_holds(
                m,
                ckc_spec::model::apply_all(ckc_spec::model::apply_all(gs, th), s),
            );
        let t3 = s.union_prefer_right(th);
        crate::mc_subst::apply_all_index(gs, th);
        crate::mc_subst::apply_all_index(ckc_spec::model::apply_all(gs, th), s);
        crate::mc_subst::apply_all_index(gs, t3);
        assert forall|q: int| 0 <= q < gs.len() implies ckc_spec::model::apply_all(
            ckc_spec::model::apply_all(gs, th),
            s,
        )[q] == ckc_spec::model::apply_all(gs, t3)[q] by {
            crate::mc_model::apply_compose(gs[q], th, s);
        }
        assert(ckc_spec::model::apply_all(ckc_spec::model::apply_all(gs, th), s)
            =~= ckc_spec::model::apply_all(gs, t3));
        crate::mc_model::conj_is_goals(m, gs, t3);
        assert(crate::mc_subst::extends(t3, th));
    }
}

// Some claim names f at a level below `level`.
fn claimed_below_exec(
    arena: &ETermArena,
    ds: &Vec<EMcDClaim>,
    Ghost(c): Ghost<Certificate>,
    root: usize,
    level: u64,
) -> (out: bool)
    requires
        crate::k2_term::root_ok(arena, root),
        ckc_spec::term::ground(arena@[root as int]),
        dcert_ok(arena.nodes@, ds@, c),
    ensures
        out ==> ckc_spec::model::claimed_below(c, arena@[root as int], level as nat),
{
    let mut q = 0usize;
    while q < ds.len()
        invariant
            crate::k2_term::root_ok(arena, root),
            ckc_spec::term::ground(arena@[root as int]),
            dcert_ok(arena.nodes@, ds@, c),
            q <= ds@.len(),
        decreases ds.len() - q,
    {
        proof {
            assert(dclaim_ok(arena.nodes@, ds@[q as int], c.claims[q as int]));
        }
        if ds[q].level < level && crate::k2_sort::term_equal(arena, ds[q].fact, root) {
            proof {
                assert(c.claims[q as int].fact == arena@[root as int]);
            }
            return true;
        }
        q += 1;
    }
    false
}

// Claim k checked in p (its clause admitted): true ⇒ claim_valid.
pub fn claim_ok(
    arena: &mut ETermArena,
    prog: &crate::mc_load::EMcProgram,
    active: &Vec<bool>,
    m: &EMcModel,
    ds: &Vec<EMcDClaim>,
    Ghost(c): Ghost<Certificate>,
    Ghost(p): Ghost<ckc_spec::model::Program>,
    k: usize,
) -> (out: bool)
    requires
        crate::k2_term::arena_ok(old(arena)),
        crate::mc_load::mc_program_ok(old(arena).nodes@, prog),
        p.clauses == crate::k2_engine::db_view(old(arena).nodes@, prog.clauses@),
        active@.len() == prog.clauses@.len(),
        forall|ci: int|
            0 <= ci < active@.len() && #[trigger] active@[ci] ==> p.active.contains(ci as nat),
        forall|ci: int|
            0 <= ci < p.clauses.len() ==> ckc_spec::model::g1_clause(#[trigger] p.clauses[ci])
                && ckc_spec::model::safe_clause(p.clauses[ci]),
        crate::mc_model::model_ok(old(arena).nodes@, m),
        dcert_ok(old(arena).nodes@, ds@, c),
        k < ds@.len(),
    ensures
        crate::k2_term::arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out ==> ckc_spec::model::claim_valid(p, m.ms@.to_iset(), c, k as int),
{
    let ghost origin = arena.nodes@;
    let ghost mm = m.ms@.to_iset();
    proof {
        assert(origin.is_prefix_of(origin)) by {
            assert(origin.take(origin.len() as int) =~= origin);
        }
        assert(dclaim_ok(origin, ds@[k as int], c.claims[k as int]));
    }
    let ci = ds[k].clause;
    if ci >= prog.clauses.len() || !active[ci] {
        return false;
    }
    let clause = &prog.clauses[ci];
    let ghost cl = c.claims[k as int];
    let ghost th = cl.valuation;
    let ghost cc = p.clauses[ci as int];
    proof {
        assert(crate::k2_engine::clause_valid(origin, &prog.clauses@[ci as int]));
        assert(cc == crate::k2_engine::clause_view(origin, clause));
        assert(ckc_spec::model::g1_clause(cc) && ckc_spec::model::safe_clause(cc));
    }
    let n = match crate::mc_safe::clause_bound(arena, clause) {
        Some(n) => n,
        None => return false,
    };
    let mut sub: Vec<Option<usize>> = Vec::new();
    let mut x = 0usize;
    while x < n
        invariant
            x <= n,
            sub@.len() == x,
            forall|y: int| 0 <= y < x ==> sub@[y] is None,
        decreases n - x,
    {
        sub.push(None);
        x += 1;
    }
    let d = &ds[k];
    let mut a = 0usize;
    while a < d.vals.len()
        invariant
            crate::k2_term::arena_ok(arena),
            arena.nodes@ == origin,
            dclaim_ok(origin, *d, cl),
            th == cl.valuation,
            sub@.len() == n,
            a <= d.vals@.len(),
            forall|y: nat|
                y < n && (#[trigger] sub@[y as int]) is Some ==> (exists|i: int|
                    0 <= i < a && (#[trigger] d.vals@[i]).0 as nat == y) && sub@[y as int]->Some_0
                    < origin.len() && origin[sub@[y as int]->Some_0 as int].term@ == th[y],
            forall|i: int|
                0 <= i < a ==> (#[trigger] d.vals@[i]).0 < n && sub@[d.vals@[i].0 as int] is Some,
        decreases d.vals.len() - a,
    {
        let (var, r) = d.vals[a];
        if var >= n {
            return false;
        }
        let ghost sub0 = sub@;
        sub.set(var, Some(r));
        proof {
            assert(d.vals@[a as int] == (var, r));
            assert(th.contains_key(var as nat));
            assert(th[var as nat] == origin[r as int].term@);
            assert forall|y: nat| y < n && (#[trigger] sub@[y as int]) is Some implies (exists|
                i: int,
            |
                0 <= i < a + 1 && (#[trigger] d.vals@[i]).0 as nat == y) && sub@[y as int]->Some_0
                < origin.len() && origin[sub@[y as int]->Some_0 as int].term@ == th[y] by {
                if y == var as nat {
                    assert(d.vals@[a as int].0 as nat == y);
                } else {
                    assert(sub@[y as int] == sub0[y as int]);
                    let i0 = choose|i0: int| 0 <= i0 < a && (#[trigger] d.vals@[i0]).0 as nat == y;
                }
            }
            assert forall|i: int| 0 <= i < a + 1 implies (#[trigger] d.vals@[i]).0 < n
                && sub@[d.vals@[i].0 as int] is Some by {
                if i < a {
                    assert(sub0[d.vals@[i].0 as int] is Some);
                }
            }
        }
        a += 1;
    }
    proof {
        assert(crate::mc_subst::sub_ok(origin, sub@, th)) by {
            assert forall|y: nat| #[trigger]
                th.contains_key(y) <==> (y < sub@.len() && sub@[y as int] is Some) by {
                if th.contains_key(y) {
                    let i = choose|i: int|
                        0 <= i < d.vals@.len() && (#[trigger] d.vals@[i]).0 as nat == y;
                    assert(d.vals@[i].0 < n);
                }
            }
        }
    }
    // the valuation's domain = the clause's non-local variables
    let mut y = 0usize;
    while y < n
        invariant
            crate::k2_term::arena_ok(arena),
            arena.nodes@ == origin,
            crate::k2_engine::clause_valid(origin, clause),
            cc == crate::k2_engine::clause_view(origin, clause),
            crate::mc_subst::sub_ok(origin, sub@, th),
            sub@.len() == n,
            y <= n,
            forall|z: nat|
                z < y ==> (#[trigger] th.contains_key(z)) == ckc_spec::model::nonlocal(cc, z),
        decreases n - y,
    {
        if sub[y].is_some() != nonlocal_exec(arena, clause, y) {
            return false;
        }
        y += 1;
    }
    proof {
        assert forall|z: nat| #[trigger] th.contains_key(z) == ckc_spec::model::nonlocal(cc, z) by {
            if z >= n {
                if ckc_spec::model::nonlocal(cc, z) {
                    if !ckc_spec::model::vars(cc.head).contains(z) {
                        let j = choose|j: int|
                            0 <= j < cc.body.len() && #[trigger] ckc_spec::model::item_vars(
                                cc.body[j],
                            ).contains(z) && (cc.body[j] is Pos || ckc_spec::model::outside(
                                cc,
                                j,
                                z,
                            ));
                    }
                }
            }
        }
        assert(ckc_spec::model::valuation(cc, th));
    }
    // head
    let mut s2 = sub.clone();
    proof {
        assert forall|z: nat| #[trigger]
            ckc_spec::engine::occurs(z, origin[clause.head as int].term@) implies z < s2@.len() by {
            crate::mc_safe::vars_occurs(cc.head, z);
        }
    }
    let (ok, _th2) = crate::mc_subst::match_term(arena, clause.head, d.fact, &mut s2, Ghost(th));
    if !ok {
        return false;
    }
    proof {
        assert(crate::mc_subst::extends(th, th));
        assert forall|z: nat| #[trigger]
            ckc_spec::engine::occurs(z, cc.head) implies th.contains_key(z) by {
            crate::mc_safe::vars_occurs(cc.head, z);
        }
        assert(ckc_spec::model::apply(cc.head, th) == cl.fact) by {
            assert forall|z: nat| #[trigger] ckc_spec::engine::occurs(z, cc.head) implies (
            th.contains_key(z) == _th2@.contains_key(z) && (th.contains_key(z) ==> th[z]
                == _th2@[z])) by {
                crate::mc_safe::vars_occurs(cc.head, z);
            }
            crate::mc_subst::apply_agree(cc.head, th, _th2@);
        }
    }
    if clause.body.len() == 0 {
        if d.level != 0 {
            return false;
        }
    } else if d.level < 1 {
        return false;
    }
    let level = d.level;
    let mut j = 0usize;
    while j < clause.body.len()
        invariant
            crate::k2_term::arena_ok(arena),
            origin.is_prefix_of(arena.nodes@),
            crate::k2_engine::clause_valid(origin, clause),
            cc == crate::k2_engine::clause_view(origin, clause),
            crate::mc_subst::sub_ok(arena.nodes@, sub@, th),
            crate::mc_model::model_ok(arena.nodes@, m),
            dcert_ok(arena.nodes@, ds@, c),
            mm == m.ms@.to_iset(),
            ckc_spec::model::valuation(cc, th),
            ckc_spec::model::g1_clause(cc),
            ckc_spec::model::safe_clause(cc),
            crate::mc_safe::vars_below(cc, n as nat),
            sub@.len() == n,
            cl == c.claims[k as int],
            k < c.claims.len(),
            cl.clause == ci as nat,
            ci < prog.clauses@.len(),
            cc == p.clauses[ci as int],
            origin == old(arena).nodes@,
            th == cl.valuation,
            level as nat == cl.level,
            j <= clause.body@.len(),
            forall|q: int|
                0 <= q < j ==> #[trigger] ckc_spec::model::item_holds(mm, mm, cc.body[q], th),
            forall|q: int| 0 <= q < j ==> #[trigger] ckc_spec::model::supported(p, c, k as int, q),
        decreases clause.body.len() - j,
    {
        let ghost before = arena.nodes@;
        proof {
            assert(crate::k2_engine::body_item_valid(origin, &clause.body@[j as int]));
            assert(cc.body[j as int] == crate::k2_engine::body_item_view(
                origin,
                &clause.body@[j as int],
            ));
            assert(ckc_spec::model::g1_item(cc.body[j as int]));
            crate::mc_model::sub_vals_wf(arena, sub@, th);
        }
        match &clause.body[j] {
            EBodyItem::Pos { root } => {
                let root = *root;
                let ghost l = origin[root as int].term@;
                proof {
                    assert(before[root as int] == origin[root as int]);
                    assert(cc.body[j as int] == BodyItem::Pos(l));
                    assert(l is Comp);
                    assert forall|z: nat| #[trigger]
                        ckc_spec::engine::occurs(z, l) implies th.contains_key(z) by {
                        crate::mc_safe::vars_occurs(l, z);
                        assert(ckc_spec::model::item_vars(cc.body[j as int]).contains(z));
                    }
                }
                let (builtin, _kids, _k_in) = crate::mc_safe::literal_parts(arena, root);
                if builtin {
                    proof {
                        assert(ckc_spec::model::lit_safe(l, th.dom())) by {
                            assert forall|z: nat|
                                ckc_spec::model::vars_all(
                                    ckc_spec::model::builtin_inputs(l),
                                ).contains(z) implies th.dom().contains(z) by {
                                let args = l->Comp_1;
                                if ckc_spec::arith::is_arith(l->Comp_0, args.len()) {
                                    crate::mc_safe::vars_all_sub(args, 0, 4, z);
                                    assert(ckc_spec::model::builtin_inputs(l) =~= args.subrange(
                                        0,
                                        4,
                                    ));
                                } else {
                                    assert(ckc_spec::model::builtin_inputs(l) =~= args.subrange(
                                        0,
                                        args.len() as int,
                                    ));
                                    crate::mc_safe::vars_all_sub(args, 0, args.len() as int, z);
                                }
                                crate::mc_safe::vars_comp(l, z);
                                crate::mc_safe::vars_occurs(l, z);
                            }
                        }
                        assert forall|z: nat| #[trigger]
                            ckc_spec::engine::occurs(z, before[root as int].term@) implies z
                            < sub@.len() by {
                            crate::mc_safe::vars_occurs(l, z);
                        }
                    }
                    match crate::mc_join::builtin_step(arena, root, &sub, Ghost(th), Ghost(mm)) {
                        None => {
                            proof {
                                crate::k2_load::prefix_chain(origin, before, arena.nodes@);
                            }
                            return false;
                        },
                        Some((_s2, th2)) => {
                            proof {
                                assert forall|z: nat| #[trigger]
                                    ckc_spec::engine::occurs(z, l) implies (th2@.contains_key(z)
                                    == th.contains_key(z) && (th.contains_key(z) ==> th2@[z]
                                    == th[z])) by {}
                                crate::mc_subst::apply_agree(l, th2@, th);
                                assert(ckc_spec::model::item_holds(mm, mm, cc.body[j as int], th));
                                assert(ckc_spec::model::supported(p, c, k as int, j as int));
                            }
                        },
                    }
                } else {
                    let inst = crate::mc_subst::instantiate(arena, root, &sub, Ghost(th));
                    proof {
                        crate::mc_subst::apply_ground(l, th);
                        crate::mc_subst::builtin_apply_iff(l, th);
                        crate::mc_model::model_ok_prefix(before, arena.nodes@, m);
                        dcert_prefix(before, arena.nodes@, ds@, c);
                    }
                    if !crate::mc_model::in_model(arena, m, inst) {
                        proof {
                            crate::k2_load::prefix_chain(origin, before, arena.nodes@);
                        }
                        return false;
                    }
                    if !claimed_below_exec(arena, ds, Ghost(c), inst, level) {
                        proof {
                            crate::k2_load::prefix_chain(origin, before, arena.nodes@);
                        }
                        return false;
                    }
                    proof {
                        assert(ckc_spec::model::item_holds(mm, mm, cc.body[j as int], th));
                        assert(ckc_spec::model::supported(p, c, k as int, j as int));
                    }
                }
            },
            EBodyItem::Naf { roots } => {
                let ghost gs = crate::k2_engine::root_terms(origin, roots@);
                proof {
                    assert(cc.body[j as int] == BodyItem::Naf(gs));
                    assert(crate::k2_engine::root_terms(before, roots@) =~= gs) by {
                        assert forall|q: int| 0 <= q < roots@.len() implies before[roots@[q] as int]
                            == origin[roots@[q] as int] by {}
                    }
                    crate::mc_lemma::naf_safe_at(cc, j as int);
                    assert(gs.skip(0) =~= gs);
                    assert(crate::mc_lemma::binds_upto(cc, j as int).subset_of(th.dom())) by {
                        assert forall|z: nat|
                            crate::mc_lemma::binds_upto(cc, j as int).contains(
                                z,
                            ) implies th.dom().contains(z) by {
                            crate::mc_lemma::binds_upto_nonlocal(cc, j as int, z);
                        }
                    }
                    crate::mc_lemma::conj_safe_mono(
                        gs,
                        crate::mc_lemma::binds_upto(cc, j as int),
                        th.dom(),
                    );
                    assert forall|q: int, z: nat|
                        0 <= q < roots@.len() && #[trigger] ckc_spec::engine::occurs(
                            z,
                            gs[q],
                        ) implies z < sub@.len() by {
                        crate::mc_safe::vars_all_any(gs, z);
                        assert(ckc_spec::model::item_vars(cc.body[j as int]).contains(z));
                    }
                    assert forall|q: int| 0 <= q < roots@.len() implies (
                    #[trigger] gs[q]) is Comp by {
                        assert(ckc_spec::model::g1_literal(gs[q]));
                    }
                }
                let found = crate::mc_join::witness_from(arena, roots, 0, &sub, Ghost(th), m);
                proof {
                    crate::k2_load::prefix_chain(origin, before, arena.nodes@);
                }
                if found {
                    return false;
                }
                proof {
                    naf_of_no_witness(mm, gs, th);
                    assert(ckc_spec::model::item_holds(mm, mm, cc.body[j as int], th));
                    assert(ckc_spec::model::supported(p, c, k as int, j as int));
                }
            },
        }
        proof {
            crate::k2_load::prefix_chain(origin, before, arena.nodes@);
            crate::mc_subst::sub_ok_prefix(before, arena.nodes@, sub@, th);
            crate::mc_model::model_ok_prefix(before, arena.nodes@, m);
            dcert_prefix(before, arena.nodes@, ds@, c);
        }
        j += 1;
    }
    proof {
        assert(ckc_spec::model::body_holds(mm, mm, cc, th));
        assert(ckc_spec::model::instance_of(p, ci as int, th));
    }
    true
}

pub open spec fn claimed_fact(c: Certificate, f: Term) -> bool {
    exists|q: int| 0 <= q < c.claims.len() && (#[trigger] c.claims[q]).fact == f
}

// Exact coverage (contract mc § Answers): Ok ⇒ covers(M, c). Err = the refusal item: an
// unclaimed model fact (its index in M) | a claim outside M or repeating an earlier fact.
pub enum ECov {
    Unclaimed(usize),
    ClaimAt(usize),
}

pub fn coverage_check(
    arena: &ETermArena,
    m: &EMcModel,
    ds: &Vec<EMcDClaim>,
    Ghost(c): Ghost<Certificate>,
) -> (out: Result<(), ECov>)
    requires
        crate::k2_term::arena_ok(arena),
        crate::mc_model::model_ok(arena.nodes@, m),
        dcert_ok(arena.nodes@, ds@, c),
    ensures
        out is Ok ==> ckc_spec::model::covers(m.ms@.to_iset(), c),
        out matches Err(ECov::Unclaimed(i)) ==> i < m.facts@.len(),
{
    let ghost mm = m.ms@.to_iset();
    let mut k = 0usize;
    while k < ds.len()
        invariant
            crate::k2_term::arena_ok(arena),
            crate::mc_model::model_ok(arena.nodes@, m),
            dcert_ok(arena.nodes@, ds@, c),
            mm == m.ms@.to_iset(),
            k <= ds@.len(),
            forall|q: int| 0 <= q < k ==> mm.contains((#[trigger] c.claims[q]).fact),
            forall|q: int, l: int|
                0 <= q < l < k ==> (#[trigger] c.claims[q]).fact != (#[trigger] c.claims[l]).fact,
        decreases ds.len() - k,
    {
        proof {
            assert(dclaim_ok(arena.nodes@, ds@[k as int], c.claims[k as int]));
        }
        if !crate::mc_model::in_model(arena, m, ds[k].fact) {
            return Err(ECov::ClaimAt(k));
        }
        let mut q = 0usize;
        while q < k
            invariant
                crate::k2_term::arena_ok(arena),
                dcert_ok(arena.nodes@, ds@, c),
                k < ds@.len(),
                q <= k,
                forall|q2: int|
                    0 <= q2 < q ==> (#[trigger] c.claims[q2]).fact != c.claims[k as int].fact,
            decreases k - q,
        {
            proof {
                assert(dclaim_ok(arena.nodes@, ds@[q as int], c.claims[q as int]));
                assert(dclaim_ok(arena.nodes@, ds@[k as int], c.claims[k as int]));
            }
            if crate::k2_sort::term_equal(arena, ds[q].fact, ds[k].fact) {
                return Err(ECov::ClaimAt(k));
            }
            q += 1;
        }
        k += 1;
    }
    let mut i = 0usize;
    while i < m.facts.len()
        invariant
            crate::k2_term::arena_ok(arena),
            crate::mc_model::model_ok(arena.nodes@, m),
            dcert_ok(arena.nodes@, ds@, c),
            i <= m.facts@.len(),
            forall|i2: int| 0 <= i2 < i ==> #[trigger] claimed_fact(c, m.ms@[i2]),
        decreases m.facts.len() - i,
    {
        let mut q = 0usize;
        let mut hit = false;
        while q < ds.len() && !hit
            invariant
                crate::k2_term::arena_ok(arena),
                crate::mc_model::model_ok(arena.nodes@, m),
                dcert_ok(arena.nodes@, ds@, c),
                i < m.facts@.len(),
                q <= ds@.len(),
                hit ==> claimed_fact(c, m.ms@[i as int]),
            decreases ds.len() - q,
        {
            proof {
                assert(dclaim_ok(arena.nodes@, ds@[q as int], c.claims[q as int]));
            }
            if crate::k2_sort::term_equal(arena, ds[q].fact, m.facts[i]) {
                hit = true;
                proof {
                    assert(c.claims[q as int].fact == m.ms@[i as int]);
                    assert(claimed_fact(c, m.ms@[i as int]));
                }
            }
            q += 1;
        }
        if !hit {
            return Err(ECov::Unclaimed(i));
        }
        i += 1;
    }
    proof {
        assert forall|f: Term| #[trigger] mm.contains(f) implies exists|q: int|
            0 <= q < c.claims.len() && (#[trigger] c.claims[q]).fact == f by {
            crate::mc_model::iset_of_seq(m.ms@, f);
            let i2 = choose|i2: int| 0 <= i2 < m.ms@.len() && m.ms@[i2] == f;
            assert(claimed_fact(c, m.ms@[i2]));
        }
    }
    Ok(())
}

} // verus!
