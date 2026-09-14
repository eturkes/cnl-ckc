#[cfg(verus_keep_ghost)]
use crate::k2_term::arena_ok;
use crate::k2_term::{ENode, ETermArena};
use crate::m6_bind::bind_all;
use crate::m6_flat::slot;
use crate::m6_model::*;
use crate::m6_safety::positive_goals;
#[cfg(verus_keep_ghost)]
use crate::m6_symbols::symbol;
use crate::m6_symbols::{Sym, symbol_bytes};
use crate::m6_term::*;
use ckc_spec::emit as spec;
use ckc_spec::replay::{self, EOut, Ob};
use ckc_spec::term::Term;
use ckc_spec::v1text::{self, DocClause};
use vstd::assert_seqs_equal;
use vstd::prelude::*;
use vstd::slice::slice_to_vec;

verus! {

pub struct CertOb {
    pub term: T,
    pub model: Ghost<Ob>,
}

impl View for CertOb {
    type V = Ob;

    open spec fn view(&self) -> Ob {
        self.model@
    }
}

pub open spec fn ob_valid(nodes: Seq<ENode>, o: &CertOb) -> bool {
    valid(nodes, &o.term) && o.term@ == replay::ob_term(o@)
}

pub open spec fn obs_valid(nodes: Seq<ENode>, os: Seq<CertOb>) -> bool {
    forall|i: int| 0 <= i < os.len() ==> #[trigger] ob_valid(nodes, &os[i])
}

pub open spec fn obs_models(os: Seq<CertOb>) -> Seq<Ob> {
    os.map_values(|o: CertOb| o@)
}

pub proof fn obs_prefix(before: Seq<ENode>, after: Seq<ENode>, os: Seq<CertOb>)
    requires
        before.is_prefix_of(after),
        obs_valid(before, os),
    ensures
        obs_valid(after, os),
{
    assert forall|i: int| 0 <= i < os.len() implies #[trigger] ob_valid(after, &os[i]) by {
        assert(ob_valid(before, &os[i]));
        prefix(before, after, &os[i].term);
    }
}

pub fn bind_terms(arena: &mut ETermArena, ts: &Vec<T>, pairs: &Vec<Binding>) -> (out: Vec<T>)
    requires
        arena_ok(old(arena)),
        valid_all(old(arena).nodes@, ts@),
        map_valid(old(arena).nodes@, pairs@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid_all(final(arena).nodes@, out@),
        models(out@) == models(ts@).map_values(|t: Term| spec::bind_all(t, map_model(pairs@))),
{
    let ghost start = arena.nodes@;
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < ts.len()
        invariant
            arena_ok(arena),
            start == old(arena).nodes@,
            start.is_prefix_of(arena.nodes@),
            valid_all(arena.nodes@, ts@),
            map_valid(arena.nodes@, pairs@),
            valid_all(arena.nodes@, out@),
            i <= ts.len(),
            out.len() == i,
            models(out@) == models(ts@).take(i as int).map_values(
                |t: Term| spec::bind_all(t, map_model(pairs@)),
            ),
        decreases ts.len() - i,
    {
        let ghost middle = arena.nodes@;
        let next = bind_all(arena, &ts[i], pairs);
        proof {
            crate::k2_load::prefix_chain(start, middle, arena.nodes@);
            prefix_all(middle, arena.nodes@, ts@);
            map_prefix(middle, arena.nodes@, pairs@);
            prefix_all(middle, arena.nodes@, out@);
            extend(arena.nodes@, out@, next);
        }
        let ghost before = out@;
        out.push(next);
        proof {
            assert_seqs_equal!(models(out@) == models(ts@).take(i as int + 1).map_values(|t: Term| spec::bind_all(t, map_model(pairs@))), j => {
            if j < before.len() { assert(out@[j] == before[j]); assert(models(before)[j] == spec::bind_all(models(ts@)[j], map_model(pairs@))); } else { assert(j == i); assert(out@[j] == next); }
        });
        }
        i += 1;
    }
    proof {
        assert(models(ts@).take(i as int) == models(ts@));
    }
    out
}

pub fn clause_heads(arena: &ETermArena, cs: &Vec<Clause>) -> (out: Vec<T>)
    requires
        arena_ok(arena),
        clauses_valid(arena.nodes@, cs@),
    ensures
        valid_all(arena.nodes@, out@),
        models(out@) == clause_models(cs@).map_values(|c: DocClause| c.head),
{
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < cs.len()
        invariant
            arena_ok(arena),
            clauses_valid(arena.nodes@, cs@),
            valid_all(arena.nodes@, out@),
            i <= cs.len(),
            out.len() == i,
            models(out@) == clause_models(cs@).take(i as int).map_values(|c: DocClause| c.head),
        decreases cs.len() - i,
    {
        proof {
            assert(clause_valid(arena.nodes@, &cs@[i as int]));
        }
        let next = cs[i].head.cp();
        proof {
            extend(arena.nodes@, out@, next);
        }
        out.push(next);
        proof {
            assert_seqs_equal!(models(out@) == clause_models(cs@).take(i as int + 1).map_values(|c: DocClause| c.head));
        }
        i += 1;
    }
    proof {
        assert(clause_models(cs@).take(i as int) == clause_models(cs@));
    }
    out
}

pub fn wrapped_list(arena: &mut ETermArena, name: &[u8], ts: &Vec<T>) -> (out: T)
    requires
        arena_ok(old(arena)),
        valid_all(old(arena).nodes@, ts@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid(final(arena).nodes@, &out),
        out@ == Term::Comp(name@, seq![ckc_spec::engine::list_term(models(ts@))]),
{
    let ghost start = arena.nodes@;
    let l = list(arena, ts);
    let ghost middle = arena.nodes@;
    let mut args = Vec::new();
    args.push(l);
    proof {
        assert_seqs_equal!(models(args@) == seq![l@]);
    }
    let out = comp(arena, name, &args);
    proof {
        crate::k2_load::prefix_chain(start, middle, arena.nodes@);
    }
    out
}

pub fn build_ob(
    arena: &mut ETermArena,
    docid: &Vec<u8>,
    s: usize,
    k: usize,
    facts: &Vec<T>,
    heads: &Vec<T>,
) -> (out: CertOb)
    requires
        arena_ok(old(arena)),
        valid_all(old(arena).nodes@, facts@),
        valid_all(old(arena).nodes@, heads@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        ob_valid(final(arena).nodes@, &out),
        out@ == (Ob {
            docid: Term::Atom(docid@),
            s: Term::Int(s as int),
            k: Term::Int(k as int),
            facts: models(facts@),
            heads: models(heads@),
        }),
{
    let ghost start = arena.nodes@;
    let ghost model = Ob {
        docid: Term::Atom(docid@),
        s: Term::Int(s as int),
        k: Term::Int(k as int),
        facts: models(facts@),
        heads: models(heads@),
    };
    let mut args = Vec::new();
    let d = atom(arena, docid);
    args.push(d);
    let ghost n1 = arena.nodes@;
    let sentence = crate::m6_term::int(arena, s);
    let ghost n2 = arena.nodes@;
    proof {
        crate::k2_load::prefix_chain(start, n1, n2);
        prefix_all(n1, n2, args@);
        extend(n2, args@, sentence);
    }
    args.push(sentence);
    let variant = slot(arena, &Sym::Variant, k);
    let ghost n3 = arena.nodes@;
    proof {
        crate::k2_load::prefix_chain(start, n2, n3);
        prefix_all(n2, n3, args@);
        extend(n3, args@, variant);
        prefix_all(start, n3, facts@);
    }
    args.push(variant);
    let witness_name: &[u8] = b"witness";
    let witness = wrapped_list(arena, witness_name, facts);
    let ghost n4 = arena.nodes@;
    proof {
        crate::k2_load::prefix_chain(start, n3, n4);
        prefix_all(n3, n4, args@);
        extend(n4, args@, witness);
        prefix_all(start, n4, heads@);
    }
    args.push(witness);
    let prove_name: &[u8] = b"prove";
    let prove = wrapped_list(arena, prove_name, heads);
    let ghost n5 = arena.nodes@;
    proof {
        crate::k2_load::prefix_chain(start, n4, n5);
        prefix_all(n4, n5, args@);
        extend(n5, args@, prove);
        reveal_byteslit(b"witness");
        reveal_strlit("witness");
        reveal_byteslit(b"prove");
        reveal_strlit("prove");
        reveal(v1text::ascii);
    }
    args.push(prove);
    proof {
        assert_seqs_equal!(witness_name@ == v1text::ascii("witness"@));
        assert_seqs_equal!(prove_name@ == v1text::ascii("prove"@));
        assert_seqs_equal!(models(args@) == seq![d@, sentence@, variant@, witness@, prove@]);
        reveal(replay::ob_term);
        assert_seqs_equal!(models(args@) == ckc_spec::engine::args_of(replay::ob_term(model)));
    }
    let term = c(arena, &Sym::DollarGuidelineProof, &args);
    proof {
        crate::k2_load::prefix_chain(start, n5, arena.nodes@);
    }
    CertOb { term, model: Ghost(model) }
}

pub fn obligation(arena: &mut ETermArena, g: &Group, docid: &Vec<u8>, s: usize) -> (out: CertOb)
    requires
        arena_ok(old(arena)),
        group_valid(old(arena).nodes@, g),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        ob_valid(final(arena).nodes@, &out),
        out@ == spec::obligation(g@, docid@, s as nat),
{
    hide(spec::bind_all);
    let ghost start = arena.nodes@;
    let unbound_facts = if g.clauses.len() == 0 {
        let empty = Vec::new();
        proof {
            assert_seqs_equal!(models(empty@) == Seq::<Term>::empty());
        }
        empty
    } else {
        proof {
            assert(clause_valid(arena.nodes@, &g.clauses@[0]));
        }
        positive_goals(arena, &g.clauses[0].body)
    };
    let facts = bind_terms(arena, &unbound_facts, &g.pairs);
    let ghost middle = arena.nodes@;
    proof {
        clauses_prefix(start, middle, g.clauses@);
        map_prefix(start, middle, g.pairs@);
    }
    let unbound_heads = clause_heads(arena, &g.clauses);
    let heads = bind_terms(arena, &unbound_heads, &g.pairs);
    let ghost before_build = arena.nodes@;
    proof {
        crate::k2_load::prefix_chain(start, middle, before_build);
        prefix_all(middle, before_build, facts@);
        assert_seqs_equal!(models(heads@) == g@.clauses.map_values(|c: DocClause| spec::bind_all(c.head, g@.pairs)));
        reveal(spec::obligation);
        assert_seqs_equal!(models(facts@) == spec::obligation(g@, docid@, s as nat).facts);
    }
    let out = build_ob(arena, docid, s, g.k, &facts, &heads);
    proof {
        crate::k2_load::prefix_chain(start, before_build, arena.nodes@);
    }
    out
}

pub fn group_obs(arena: &mut ETermArena, gs: &Vec<Group>, docid: &Vec<u8>, s: usize) -> (out: Vec<
    CertOb,
>)
    requires
        arena_ok(old(arena)),
        groups_valid(old(arena).nodes@, gs@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        obs_valid(final(arena).nodes@, out@),
        obs_models(out@) == group_models(gs@).map_values(
            |g: spec::Group| spec::obligation(g, docid@, s as nat),
        ),
{
    let ghost start = arena.nodes@;
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < gs.len()
        invariant
            arena_ok(arena),
            start == old(arena).nodes@,
            start.is_prefix_of(arena.nodes@),
            groups_valid(arena.nodes@, gs@),
            obs_valid(arena.nodes@, out@),
            i <= gs.len(),
            out.len() == i,
            obs_models(out@) == group_models(gs@).take(i as int).map_values(
                |g: spec::Group| spec::obligation(g, docid@, s as nat),
            ),
        decreases gs.len() - i,
    {
        proof {
            assert(group_valid(arena.nodes@, &gs@[i as int]));
        }
        let ghost middle = arena.nodes@;
        let next = obligation(arena, &gs[i], docid, s);
        proof {
            crate::k2_load::prefix_chain(start, middle, arena.nodes@);
            groups_prefix(middle, arena.nodes@, gs@);
            obs_prefix(middle, arena.nodes@, out@);
        }
        let ghost before = out@;
        out.push(next);
        proof {
            assert forall|j: int| 0 <= j < out.len() implies #[trigger] ob_valid(
                arena.nodes@,
                &out@[j],
            ) by {
                if j < before.len() {
                    assert(out@[j] == before[j]);
                }
            }
            assert_seqs_equal!(obs_models(out@) == group_models(gs@).take(i as int + 1).map_values(|g: spec::Group| spec::obligation(g, docid@, s as nat)), j => {
                if j < before.len() { assert(out@[j] == before[j]); assert(obs_models(before)[j] == spec::obligation(group_models(gs@)[j], docid@, s as nat)); } else { assert(j == i); assert(out@[j] == next); }
            });
        }
        i += 1;
    }
    proof {
        assert(group_models(gs@).take(i as int) == group_models(gs@));
    }
    out
}

pub proof fn obligations_unroll(ps: Seq<spec::Projected>, docid: Seq<u8>)
    requires
        ps.len() > 0,
    ensures
        spec::obligations(ps, docid) == ps[0].groups.map_values(
            |g: spec::Group| spec::obligation(g, docid, ps[0].s),
        ) + spec::obligations(ps.drop_first(), docid),
{
    let parts = ps.map_values(
        |p: spec::Projected| p.groups.map_values(|g: spec::Group| spec::obligation(g, docid, p.s)),
    );
    assert_seqs_equal!(parts.drop_first() == ps.drop_first().map_values(|p: spec::Projected| p.groups.map_values(|g: spec::Group| spec::obligation(g, docid, p.s))));
    reveal(spec::obligations);
    reveal_with_fuel(Seq::flatten, 1);
}

pub fn obligations(arena: &mut ETermArena, ps: &Vec<Projected>, docid: &Vec<u8>) -> (out: Vec<
    CertOb,
>)
    requires
        arena_ok(old(arena)),
        projections_valid(old(arena).nodes@, ps@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        obs_valid(final(arena).nodes@, out@),
        obs_models(out@) == spec::obligations(project_models(ps@), docid@),
{
    let ghost start = arena.nodes@;
    let mut out = Vec::new();
    let mut i = 0usize;
    proof {
        assert(project_models(ps@).skip(0) == project_models(ps@));
        assert_seqs_equal!(obs_models(out@) == Seq::<Ob>::empty());
    }
    while i < ps.len()
        invariant
            arena_ok(arena),
            start == old(arena).nodes@,
            start.is_prefix_of(arena.nodes@),
            projections_valid(arena.nodes@, ps@),
            obs_valid(arena.nodes@, out@),
            i <= ps.len(),
            obs_models(out@) + spec::obligations(project_models(ps@).skip(i as int), docid@)
                == spec::obligations(project_models(ps@), docid@),
        decreases ps.len() - i,
    {
        proof {
            assert(projected_valid(arena.nodes@, &ps@[i as int]));
            assert(project_models(ps@).skip(i as int).drop_first() == project_models(ps@).skip(
                i as int + 1,
            ));
            obligations_unroll(project_models(ps@).skip(i as int), docid@);
        }
        let ghost middle = arena.nodes@;
        let mut next = group_obs(arena, &ps[i].groups, docid, ps[i].s);
        proof {
            crate::k2_load::prefix_chain(start, middle, arena.nodes@);
            projections_prefix(middle, arena.nodes@, ps@);
            obs_prefix(middle, arena.nodes@, out@);
        }
        let ghost before = out@;
        let ghost added = next@;
        out.append(&mut next);
        proof {
            assert_seqs_equal!(obs_models(out@) == obs_models(before) + obs_models(added));
            assert forall|j: int| 0 <= j < out.len() implies #[trigger] ob_valid(
                arena.nodes@,
                &out@[j],
            ) by {
                if j < before.len() {
                    assert(out@[j] == before[j]);
                } else {
                    assert(out@[j] == added[j - before.len()]);
                }
            }
        }
        i += 1;
    }
    proof {
        reveal(spec::obligations);
        reveal(Seq::flatten);
    }
    out
}

pub open spec fn selected(os: Seq<CertOb>, i: Option<usize>) -> Option<Ob> {
    match i {
        Some(i) => Some(os[i as int]@),
        None => None,
    }
}

pub fn first_nonground(arena: &ETermArena, os: &Vec<CertOb>) -> (out: Option<usize>)
    requires
        arena_ok(arena),
        obs_valid(arena.nodes@, os@),
    ensures
        out matches Some(i) ==> i < os.len(),
        selected(os@, out) == spec::first_nonground(obs_models(os@)),
{
    let mut i = 0usize;
    proof {
        assert(obs_models(os@).skip(0) == obs_models(os@));
    }
    while i < os.len()
        invariant
            arena_ok(arena),
            obs_valid(arena.nodes@, os@),
            i <= os.len(),
            spec::first_nonground(obs_models(os@)) == spec::first_nonground(
                obs_models(os@).skip(i as int),
            ),
        decreases os.len() - i,
    {
        proof {
            assert(ob_valid(arena.nodes@, &os@[i as int]));
            assert(obs_models(os@).skip(i as int).drop_first() == obs_models(os@).skip(
                i as int + 1,
            ));
            reveal_with_fuel(spec::first_nonground, 1);
        }
        if !ground(arena, &os[i].term) {
            return Some(i);
        }
        i += 1;
    }
    proof {
        reveal(spec::first_nonground);
    }
    None
}

pub proof fn payload_unroll(os: Seq<Ob>)
    requires
        os.len() > 0,
    ensures
        replay::print_payload(os) == v1text::term_line(replay::ob_term(os[0]))
            + replay::print_payload(os.drop_first()),
{
    let parts = os.map_values(|o: Ob| v1text::term_line(replay::ob_term(o)));
    assert_seqs_equal!(parts.drop_first() == os.drop_first().map_values(|o: Ob| v1text::term_line(replay::ob_term(o))));
    reveal(replay::print_payload);
    reveal_with_fuel(Seq::flatten, 1);
}

pub fn print_payload(arena: &ETermArena, os: &Vec<CertOb>) -> (out: Vec<u8>)
    requires
        arena_ok(arena),
        obs_valid(arena.nodes@, os@),
    ensures
        out@ == replay::print_payload(obs_models(os@)),
{
    let mut out = Vec::new();
    let mut i = 0usize;
    proof {
        assert(obs_models(os@).skip(0) == obs_models(os@));
    }
    while i < os.len()
        invariant
            arena_ok(arena),
            obs_valid(arena.nodes@, os@),
            i <= os.len(),
            out@ + replay::print_payload(obs_models(os@).skip(i as int)) == replay::print_payload(
                obs_models(os@),
            ),
        decreases os.len() - i,
    {
        proof {
            assert(ob_valid(arena.nodes@, &os@[i as int]));
            assert(obs_models(os@).skip(i as int).drop_first() == obs_models(os@).skip(
                i as int + 1,
            ));
            payload_unroll(obs_models(os@).skip(i as int));
        }
        let mut line = crate::k2_term::term_line(arena, os[i].term.root);
        out.append(&mut line);
        i += 1;
    }
    proof {
        reveal(replay::print_payload);
        reveal(Seq::flatten);
    }
    out
}

pub fn reject(arena: &ETermArena, id: &[u8], why: &T) -> (out: EOut)
    requires
        arena_ok(arena),
        valid(arena.nodes@, why),
    ensures
        out@ == spec::certify_reject(id@, why@),
{
    let mut err = slice_to_vec(b"ckc: certify: ");
    let mut name = slice_to_vec(id);
    err.append(&mut name);
    let mut separator = slice_to_vec(b": ");
    err.append(&mut separator);
    let mut line = crate::k2_term::term_line(arena, why.root);
    err.append(&mut line);
    proof {
        reveal_byteslit(b"ckc: certify: ");
        reveal_strlit("ckc: certify: ");
        reveal_byteslit(b": ");
        reveal_strlit(": ");
        reveal(v1text::ascii);
        reveal(spec::certify_reject);
        assert(arena@[why.root as int] == why@);
        assert_seqs_equal!(err@ == v1text::ascii("ckc: certify: "@) + id@ + v1text::ascii(": "@) + v1text::term_line(why@));
    }
    EOut { rc: 1, out: Vec::new(), err }
}

pub fn reject_sym(arena: &mut ETermArena, id: &[u8], sym: &Sym) -> (out: EOut)
    requires
        arena_ok(old(arena)),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out@ == spec::certify_reject(id@, Term::Atom(symbol(sym))),
{
    let why = named(arena, sym);
    reject(arena, id, &why)
}

} // verus!
