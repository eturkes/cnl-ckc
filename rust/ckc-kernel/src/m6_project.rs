#[cfg(verus_keep_ghost)]
use crate::k2_term::arena_ok;
use crate::k2_term::{ENode, ETermArena};
use crate::m6_drs::*;
use crate::m6_flat::Env;
use crate::m6_group::{error, fact_group};
use crate::m6_model::*;
use crate::m6_rules::{one_group, rule_groups};
use crate::m6_symbols::Sym;
use crate::m6_term::*;
use ckc_spec::emit as spec;
use ckc_spec::term::Term;
use ckc_spec::v1text::DocClause;
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub fn copy_clauses(arena: &ETermArena, cs: &Vec<Clause>) -> (out: Vec<Clause>)
    requires
        arena_ok(arena),
        clauses_valid(arena.nodes@, cs@),
    ensures
        clauses_valid(arena.nodes@, out@),
        clause_models(out@) == clause_models(cs@),
{
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < cs.len()
        invariant
            arena_ok(arena),
            clauses_valid(arena.nodes@, cs@),
            clauses_valid(arena.nodes@, out@),
            i <= cs.len(),
            out.len() == i,
            clause_models(out@) == clause_models(cs@).take(i as int),
        decreases cs.len() - i,
    {
        proof {
            assert(clause_valid(arena.nodes@, &cs@[i as int]));
        }
        let next = Clause { head: cs[i].head.cp(), body: copy_body(arena, &cs[i].body) };
        let ghost before = out@;
        out.push(next);
        proof {
            assert forall|j: int| 0 <= j < out.len() implies #[trigger] clause_valid(
                arena.nodes@,
                &out@[j],
            ) by {
                if j < before.len() {
                    assert(out@[j] == before[j]);
                }
            }
            assert_seqs_equal!(clause_models(out@) == clause_models(cs@).take(i as int + 1), j => {
                if j < before.len() { assert(out@[j] == before[j]); assert(clause_models(before)[j] == clause_models(cs@)[j]); }
                else { assert(j == i); assert(out@[j] == next); }
            });
        }
        i += 1;
    }
    out
}

pub fn concat_clauses(arena: &ETermArena, a: &Vec<Clause>, b: &Vec<Clause>) -> (out: Vec<Clause>)
    requires
        arena_ok(arena),
        clauses_valid(arena.nodes@, a@),
        clauses_valid(arena.nodes@, b@),
    ensures
        clauses_valid(arena.nodes@, out@),
        clause_models(out@) == clause_models(a@) + clause_models(b@),
{
    let mut out = copy_clauses(arena, a);
    let mut rest = copy_clauses(arena, b);
    let ghost first = out@;
    let ghost last = rest@;
    out.append(&mut rest);
    proof {
        assert_seqs_equal!(clause_models(out@) == clause_models(a@) + clause_models(b@));
        assert forall|i: int| 0 <= i < out.len() implies #[trigger] clause_valid(
            arena.nodes@,
            &out@[i],
        ) by {
            if i < first.len() {
                assert(out@[i] == first[i]);
            } else {
                assert(out@[i] == last[i - first.len()]);
            }
        }
    }
    out
}

pub proof fn group_clauses_unroll(gs: Seq<spec::Group>)
    requires
        gs.len() > 0,
    ensures
        spec::group_clauses(gs) == gs[0].clauses + spec::group_clauses(gs.drop_first()),
{
    let seqs = gs.map_values(|g: spec::Group| g.clauses);
    assert(seqs[0] == gs[0].clauses);
    assert_seqs_equal!(seqs.drop_first() == gs.drop_first().map_values(|g: spec::Group| g.clauses));
    reveal_with_fuel(Seq::flatten, 1);
    reveal(spec::group_clauses);
}

pub fn group_clauses(arena: &ETermArena, gs: &Vec<Group>) -> (out: Vec<Clause>)
    requires
        arena_ok(arena),
        groups_valid(arena.nodes@, gs@),
    ensures
        clauses_valid(arena.nodes@, out@),
        clause_models(out@) == spec::group_clauses(group_models(gs@)),
{
    let mut out = Vec::new();
    let mut i = 0usize;
    proof {
        assert(group_models(gs@).skip(0) == group_models(gs@));
        assert_seqs_equal!(clause_models(out@) == Seq::<DocClause>::empty());
    }
    while i < gs.len()
        invariant
            arena_ok(arena),
            groups_valid(arena.nodes@, gs@),
            clauses_valid(arena.nodes@, out@),
            i <= gs.len(),
            clause_models(out@) + spec::group_clauses(group_models(gs@).skip(i as int))
                == spec::group_clauses(group_models(gs@)),
        decreases gs.len() - i,
    {
        proof {
            assert(group_valid(arena.nodes@, &gs@[i as int]));
            assert(group_models(gs@).skip(i as int).drop_first() == group_models(gs@).skip(
                i as int + 1,
            ));
            group_clauses_unroll(group_models(gs@).skip(i as int));
        }
        out = concat_clauses(arena, &out, &gs[i].clauses);
        i += 1;
    }
    proof {
        reveal(spec::group_clauses);
        reveal(Seq::flatten);
    }
    out
}

pub fn fact_roots(roots: &Vec<ERoot>) -> (out: bool)
    ensures
        out == (forall|i: int|
            0 <= i < roots.len() ==> spec::is_fact_root(#[trigger] root_models(roots@)[i])),
{
    let mut i = 0usize;
    while i < roots.len()
        invariant
            i <= roots.len(),
            forall|j: int| 0 <= j < i ==> spec::is_fact_root(#[trigger] root_models(roots@)[j]),
        decreases roots.len() - i,
    {
        if let ERoot::Rule(_, _) = &roots[i] {
            proof {
                assert(!spec::is_fact_root(root_models(roots@)[i as int]));
            }
            return false;
        }
        i += 1;
    }
    true
}

pub open spec fn sentence_result(r: Result<(Vec<Group>, Vec<Binding>), T>) -> Result<
    (Seq<spec::Group>, Seq<(Term, Term)>),
    Term,
> {
    match r {
        Ok((gs, m)) => Ok((group_models(gs@), map_model(m@))),
        Err(e) => Err(e@),
    }
}

pub open spec fn sentence_result_valid(
    nodes: Seq<ENode>,
    r: &Result<(Vec<Group>, Vec<Binding>), T>,
) -> bool {
    match r {
        Ok((gs, m)) => groups_valid(nodes, gs@) && map_valid(nodes, m@),
        Err(e) => valid(nodes, e),
    }
}

pub fn sentence_groups(
    arena: &mut ETermArena,
    roots: &Vec<ERoot>,
    env: &Env,
    map: &Vec<Binding>,
) -> (out: Result<(Vec<Group>, Vec<Binding>), T>)
    requires
        arena_ok(old(arena)),
        roots_valid(old(arena).nodes@, roots@),
        map_valid(old(arena).nodes@, map@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        sentence_result_valid(final(arena).nodes@, &out),
        sentence_result(out) == spec::sentence_groups(
            root_models(roots@),
            env.s as nat,
            env.docid@,
            map_model(map@),
            env.base as nat,
        ),
{
    hide(spec::rule_groups);
    hide(spec::fact_group);
    let ghost start = arena.nodes@;
    if roots.len() == 1 {
        if let ERoot::Rule(a, c) = &roots[0] {
            proof {
                assert(root_valid(arena.nodes@, &roots@[0]));
            }
            return match rule_groups(arena, a, c, env, map) {
                Err(e) => Err(e),
                Ok(gs) => {
                    proof {
                        map_prefix(start, arena.nodes@, map@);
                    }
                    Ok((gs, copy_map(arena, map)))
                },
            };
        }
    }
    if !fact_roots(roots) {
        return Err(error(arena, &Sym::SentenceShape, Ghost(start)));
    }
    match fact_group(arena, roots, env, map) {
        Err(e) => Err(e),
        Ok((g, map)) => Ok((one_group(arena, g), map)),
    }
}

pub fn err_at(arena: &mut ETermArena, s: usize, why: &T, Ghost(start): Ghost<Seq<ENode>>) -> (out:
    T)
    requires
        arena_ok(old(arena)),
        valid(old(arena).nodes@, why),
        start.is_prefix_of(old(arena).nodes@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        start.is_prefix_of(final(arena).nodes@),
        valid(final(arena).nodes@, &out),
        out@ == spec::err_at(s as nat, why@),
{
    let ghost before = arena.nodes@;
    let sn = crate::m6_term::int(arena, s);
    let ghost middle = arena.nodes@;
    proof {
        prefix(before, middle, why);
        crate::k2_load::prefix_chain(start, before, middle);
    }
    let out = c2(arena, &Sym::Sentence, &sn, why);
    proof {
        crate::k2_load::prefix_chain(before, middle, arena.nodes@);
        crate::k2_load::prefix_chain(start, middle, arena.nodes@);
    }
    out
}

pub open spec fn project_result(r: Result<Vec<Projected>, T>) -> Result<
    Seq<spec::Projected>,
    Term,
> {
    match r {
        Ok(ps) => Ok(project_models(ps@)),
        Err(e) => Err(e@),
    }
}

pub open spec fn project_result_valid(nodes: Seq<ENode>, r: &Result<Vec<Projected>, T>) -> bool {
    match r {
        Ok(ps) => projections_valid(nodes, ps@),
        Err(e) => valid(nodes, e),
    }
}

pub open spec fn project_prefix(
    a: Seq<spec::Projected>,
    b: Result<Seq<spec::Projected>, Term>,
) -> Result<Seq<spec::Projected>, Term> {
    match b {
        Ok(b) => Ok(a + b),
        Err(e) => Err(e),
    }
}

pub proof fn project_prefix_assoc(
    a: Seq<spec::Projected>,
    b: Seq<spec::Projected>,
    r: Result<Seq<spec::Projected>, Term>,
)
    ensures
        project_prefix(a, project_prefix(b, r)) == project_prefix(a + b, r),
{
    match r {
        Ok(c) => {
            assert(a + (b + c) == (a + b) + c);
        },
        Err(_) => {},
    }
}

pub fn project_all(
    arena: &mut ETermArena,
    tagged: &Vec<ETag>,
    docid: &Vec<u8>,
    count: usize,
    base: usize,
) -> (out: Result<Vec<Projected>, T>)
    requires
        arena_ok(old(arena)),
        tagged_valid(old(arena).nodes@, tagged@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        project_result_valid(final(arena).nodes@, &out),
        out matches Ok(ps) ==> ps.len() == count,
        project_result(out) == spec::project_from(
            tag_models(tagged@),
            1,
            count as nat,
            docid@,
            Seq::empty(),
            base as nat,
        ),
{
    hide(spec::sentence_groups);
    hide(spec::group_clauses);
    let ghost start = arena.nodes@;
    let mut out = Vec::new();
    let mut map = Vec::new();
    let mut i = 0usize;
    proof {
        assert_seqs_equal!(project_models(out@) == Seq::<spec::Projected>::empty());
        assert_seqs_equal!(map_model(map@) == Seq::<(Term, Term)>::empty());
        match spec::project_from(
            tag_models(tagged@),
            1,
            count as nat,
            docid@,
            Seq::empty(),
            base as nat,
        ) {
            Ok(_) => {},
            Err(_) => {},
        }
    }
    while i < count
        invariant
            arena_ok(arena),
            start == old(arena).nodes@,
            start.is_prefix_of(arena.nodes@),
            tagged_valid(arena.nodes@, tagged@),
            projections_valid(arena.nodes@, out@),
            map_valid(arena.nodes@, map@),
            i <= count,
            out.len() == i,
            spec::project_from(
                tag_models(tagged@),
                1,
                count as nat,
                docid@,
                Seq::empty(),
                base as nat,
            ) == project_prefix(
                project_models(out@),
                spec::project_from(
                    tag_models(tagged@),
                    i as nat + 1,
                    count as nat,
                    docid@,
                    map_model(map@),
                    base as nat,
                ),
            ),
        decreases count - i,
    {
        let s = i + 1;
        let env = Env { docid: docid.clone(), s, base };
        let ghost before = arena.nodes@;
        let sn = crate::m6_term::int(arena, s);
        let ghost middle = arena.nodes@;
        proof {
            crate::k2_load::prefix_chain(start, before, middle);
            tags_prefix(before, middle, tagged@);
            projections_prefix(before, middle, out@);
            map_prefix(before, middle, map@);
            reveal_with_fuel(spec::project_from, 1);
        }
        let roots = of_sentence(arena, tagged, &sn);
        let result = sentence_groups(arena, &roots, &env, &map);
        proof {
            crate::k2_load::prefix_chain(start, middle, arena.nodes@);
            tags_prefix(middle, arena.nodes@, tagged@);
            projections_prefix(middle, arena.nodes@, out@);
        }
        let (groups, map2) = match result {
            Ok(pair) => pair,
            Err(e) => return Err(err_at(arena, s, &e, Ghost(start))),
        };
        let clauses = group_clauses(arena, &groups);
        if clauses.len() == 0 {
            let e = error(arena, &Sym::SentenceShape, Ghost(start));
            return Err(err_at(arena, s, &e, Ghost(start)));
        }
        proof {
            assert(spec::group_clauses(group_models(groups@)).len() > 0);
        }
        let p = Projected { s, groups };
        let ghost future = spec::project_from(
            tag_models(tagged@),
            i as nat + 2,
            count as nat,
            docid@,
            map_model(map2@),
            base as nat,
        );
        proof {
            match future {
                Ok(_) => {},
                Err(_) => {},
            }
            assert(spec::project_from(
                tag_models(tagged@),
                i as nat + 1,
                count as nat,
                docid@,
                map_model(map@),
                base as nat,
            ) == project_prefix(seq![p@], future));
            project_prefix_assoc(
                project_models(out@),
                seq![p@],
                spec::project_from(
                    tag_models(tagged@),
                    i as nat + 2,
                    count as nat,
                    docid@,
                    map_model(map2@),
                    base as nat,
                ),
            );
            assert(spec::project_from(
                tag_models(tagged@),
                1,
                count as nat,
                docid@,
                Seq::empty(),
                base as nat,
            ) == project_prefix(project_models(out@) + seq![p@], future));
        }
        let ghost previous = out@;
        out.push(p);
        map = map2;
        proof {
            assert(map_model(map@) == map_model(map2@));
            assert_seqs_equal!(project_models(out@) == project_models(previous).push(p@));
            assert_seqs_equal!(project_models(out@) == project_models(previous) + seq![p@]);
            assert(future == spec::project_from(
                tag_models(tagged@),
                i as nat + 2,
                count as nat,
                docid@,
                map_model(map@),
                base as nat,
            ));
            assert(spec::project_from(
                tag_models(tagged@),
                1,
                count as nat,
                docid@,
                Seq::empty(),
                base as nat,
            ) == project_prefix(project_models(out@), future));
            assert forall|j: int| 0 <= j < out.len() implies #[trigger] projected_valid(
                arena.nodes@,
                &out@[j],
            ) by {
                if j < previous.len() {
                    assert(out@[j] == previous[j]);
                }
            }
        }
        i += 1;
    }
    proof {
        reveal(spec::project_from);
    }
    Ok(out)
}

pub fn project(arena: &mut ETermArena, drs: &T, docid: &Vec<u8>, count: usize) -> (out: Result<
    Vec<Projected>,
    T,
>)
    requires
        arena_ok(old(arena)),
        valid(old(arena).nodes@, drs),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        project_result_valid(final(arena).nodes@, &out),
        out matches Ok(ps) ==> ps.len() == count,
        project_result(out) == spec::project(drs@, docid@, count as nat),
{
    hide(spec::project_from);
    hide(spec::collides);
    hide(spec::tags);
    let ghost start = arena.nodes@;
    let b = match box_parts(arena, drs) {
        Some(b) => b,
        None => return Err(error(arena, &Sym::InvalidDrsShape, Ghost(start))),
    };
    if collides(arena, drs) {
        return Err(error(arena, &Sym::ReservedNameCollision, Ghost(start)));
    }
    let cs = match items(arena, &b.conds) {
        Some(cs) => cs,
        None => return Err(error(arena, &Sym::InvalidDrsShape, Ghost(start))),
    };
    let tagged = match tags(arena, &cs) {
        Some(ts) => ts,
        None => return Err(error(arena, &Sym::RootCondition, Ghost(start))),
    };
    if !in_range(arena, &tagged, count) {
        return Err(error(arena, &Sym::ConditionOutsideSentenceRange, Ghost(start)));
    }
    let ghost middle = arena.nodes@;
    proof {
        prefix(start, middle, drs);
    }
    let base = nvars(arena, drs);
    let ghost before_project = arena.nodes@;
    proof {
        crate::k2_load::prefix_chain(start, middle, before_project);
        tags_prefix(start, before_project, tagged@);
    }
    let out = project_all(arena, &tagged, docid, count, base);
    proof {
        crate::k2_load::prefix_chain(start, before_project, arena.nodes@);
    }
    out
}

} // verus!
