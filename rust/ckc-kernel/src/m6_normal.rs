use crate::k2_term::ETermArena;
#[cfg(verus_keep_ghost)]
use crate::k2_term::arena_ok;
use crate::m6_model::*;
use crate::m6_term::*;
use crate::m6_vars::*;
use ckc_spec::emit as spec;
use ckc_spec::term::Term;
use ckc_spec::v1text::{self, BodyItem};
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub fn body_stream(arena: &ETermArena, bs: &Vec<Body>) -> (out: Vec<usize>)
    requires
        arena_ok(arena),
        bodies_valid(arena.nodes@, bs@),
    ensures
        nums(out@) == v1text::body_var_stream(body_models(bs@)),
{
    let mut out = Vec::new();
    let mut i = 0usize;
    proof {
        assert(body_models(bs@).skip(0) == body_models(bs@));
        assert_seqs_equal!(nums(out@) == Seq::<nat>::empty());
    }
    while i < bs.len()
        invariant
            arena_ok(arena),
            bodies_valid(arena.nodes@, bs@),
            i <= bs.len(),
            nums(out@) + v1text::body_var_stream(body_models(bs@).skip(i as int))
                == v1text::body_var_stream(body_models(bs@)),
        decreases bs.len() - i,
    {
        proof {
            assert(body_valid(arena.nodes@, &bs@[i as int]));
            assert(body_models(bs@).skip(i as int).drop_first() == body_models(bs@).skip(
                i as int + 1,
            ));
            reveal_with_fuel(v1text::body_var_stream, 1);
        }
        let next = match &bs[i] {
            Body::Pos(t) => stream(arena, t),
            Body::Naf(ts) => stream_all(arena, ts),
        };
        out = concat_nums(&out, &next);
        i += 1;
    }
    proof {
        reveal(v1text::body_var_stream);
    }
    out
}

pub fn renumber(arena: &mut ETermArena, t: &T, fs: &Vec<usize>) -> (out: T)
    requires
        arena_ok(old(arena)),
        valid(old(arena).nodes@, t),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid(final(arena).nodes@, &out),
        out@ == spec::renumber(t@, nums(fs@)),
    decreases crate::k2_engine::term_size(t@), 0int,
{
    let ghost start = arena.nodes@;
    proof {
        reveal_with_fuel(spec::renumber, 1);
    }
    if is_var(arena, t) {
        let key = var_index(arena, t);
        let p = position(fs, key);
        return var(arena, p);
    }
    match parts(arena, t) {
        None => t.cp(),
        Some((name, args)) => {
            let original = from_root(arena, t.root);
            proof {
                assert(original@ == t@);
                assert(t@ == Term::Comp(name@, models(args@)));
                reveal(crate::k2_engine::term_size);
            }
            let renamed = renumber_all(arena, &args, fs);
            let ghost middle = arena.nodes@;
            let out = comp(arena, &name, &renamed);
            proof {
                crate::k2_load::prefix_chain(start, middle, arena.nodes@);
            }
            out
        },
    }
}

pub fn renumber_all(arena: &mut ETermArena, ts: &Vec<T>, fs: &Vec<usize>) -> (out: Vec<T>)
    requires
        arena_ok(old(arena)),
        valid_all(old(arena).nodes@, ts@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid_all(final(arena).nodes@, out@),
        out.len() == ts.len(),
        models(out@) == spec::renumber_all(models(ts@), nums(fs@)),
    decreases crate::k2_engine::terms_size(models(ts@)), 1int,
{
    let ghost start = arena.nodes@;
    let mut out = Vec::new();
    let mut i = 0usize;
    proof {
        assert(models(ts@).skip(0) == models(ts@));
        assert_seqs_equal!(models(out@) == Seq::<Term>::empty());
    }
    while i < ts.len()
        invariant
            arena_ok(arena),
            start == old(arena).nodes@,
            start.is_prefix_of(arena.nodes@),
            valid_all(arena.nodes@, ts@),
            valid_all(arena.nodes@, out@),
            i <= ts.len(),
            out.len() == i,
            models(out@) + spec::renumber_all(models(ts@).skip(i as int), nums(fs@))
                == spec::renumber_all(models(ts@), nums(fs@)),
        decreases ts.len() - i,
    {
        proof {
            crate::m6_drs::size_in(models(ts@), i as int);
            assert(models(ts@).skip(i as int).drop_first() == models(ts@).skip(i as int + 1));
            reveal_with_fuel(spec::renumber_all, 1);
        }
        let ghost middle = arena.nodes@;
        let next = renumber(arena, &ts[i], fs);
        proof {
            crate::k2_load::prefix_chain(start, middle, arena.nodes@);
            prefix_all(middle, arena.nodes@, ts@);
            prefix_all(middle, arena.nodes@, out@);
            extend(arena.nodes@, out@, next);
        }
        out.push(next);
        i += 1;
    }
    proof {
        reveal(spec::renumber_all);
    }
    out
}

pub fn renumber_body(arena: &mut ETermArena, b: &Body, fs: &Vec<usize>) -> (out: Body)
    requires
        arena_ok(old(arena)),
        body_valid(old(arena).nodes@, b),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        body_valid(final(arena).nodes@, &out),
        out@ == spec::renumber_item(b@, nums(fs@)),
{
    match b {
        Body::Pos(t) => Body::Pos(renumber(arena, t, fs)),
        Body::Naf(ts) => Body::Naf(renumber_all(arena, ts, fs)),
    }
}

pub fn renumber_bodies(arena: &mut ETermArena, bs: &Vec<Body>, fs: &Vec<usize>) -> (out: Vec<Body>)
    requires
        arena_ok(old(arena)),
        bodies_valid(old(arena).nodes@, bs@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        bodies_valid(final(arena).nodes@, out@),
        body_models(out@) == body_models(bs@).map_values(
            |b: BodyItem| spec::renumber_item(b, nums(fs@)),
        ),
{
    let ghost start = arena.nodes@;
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < bs.len()
        invariant
            arena_ok(arena),
            start == old(arena).nodes@,
            start.is_prefix_of(arena.nodes@),
            bodies_valid(arena.nodes@, bs@),
            bodies_valid(arena.nodes@, out@),
            i <= bs.len(),
            out.len() == i,
            body_models(out@) == body_models(bs@).take(i as int).map_values(
                |b: BodyItem| spec::renumber_item(b, nums(fs@)),
            ),
        decreases bs.len() - i,
    {
        proof {
            assert(body_valid(arena.nodes@, &bs@[i as int]));
        }
        let ghost middle = arena.nodes@;
        let next = renumber_body(arena, &bs[i], fs);
        proof {
            crate::k2_load::prefix_chain(start, middle, arena.nodes@);
            bodies_prefix(middle, arena.nodes@, bs@);
            bodies_prefix(middle, arena.nodes@, out@);
        }
        let ghost before = out@;
        out.push(next);
        proof {
            assert forall|j: int| 0 <= j < out.len() implies #[trigger] body_valid(
                arena.nodes@,
                &out@[j],
            ) by {
                if j < before.len() {
                    assert(out@[j] == before[j]);
                }
            }
            assert_seqs_equal!(body_models(out@) == body_models(bs@).take(i as int + 1).map_values(|b: BodyItem| spec::renumber_item(b, nums(fs@))), j => {
                if j < before.len() { assert(out@[j] == before[j]); assert(body_models(before)[j] == spec::renumber_item(body_models(bs@)[j], nums(fs@))); }
                else { assert(j == i); assert(out@[j] == next); }
            });
        }
        i += 1;
    }
    proof {
        assert(body_models(bs@).take(i as int) == body_models(bs@));
    }
    out
}

pub fn canon_clause(arena: &mut ETermArena, c: &Clause) -> (out: Clause)
    requires
        arena_ok(old(arena)),
        clause_valid(old(arena).nodes@, c),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        clause_valid(final(arena).nodes@, &out),
        out@ == spec::canon_clause(c@),
{
    let ghost start = arena.nodes@;
    let head_stream = stream(arena, &c.head);
    let body_stream = body_stream(arena, &c.body);
    let all = concat_nums(&head_stream, &body_stream);
    let fs = firsts(&all);
    let head = renumber(arena, &c.head, &fs);
    let ghost middle = arena.nodes@;
    proof {
        bodies_prefix(start, middle, c.body@);
    }
    let body = renumber_bodies(arena, &c.body, &fs);
    proof {
        crate::k2_load::prefix_chain(start, middle, arena.nodes@);
        prefix(middle, arena.nodes@, &head);
        assert_seqs_equal!(body_models(body@) == spec::canon_clause(c@).body);
    }
    Clause { head, body }
}

pub fn canon_pair(arena: &mut ETermArena, goal: &T, answers: &T) -> (out: (T, T))
    requires
        arena_ok(old(arena)),
        valid(old(arena).nodes@, goal),
        valid(old(arena).nodes@, answers),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid(final(arena).nodes@, &out.0),
        valid(final(arena).nodes@, &out.1),
        (out.0@, out.1@) == spec::canon_pair(goal@, answers@),
{
    let ghost start = arena.nodes@;
    let gs = stream(arena, goal);
    let as_ = stream(arena, answers);
    let all = concat_nums(&gs, &as_);
    let fs = firsts(&all);
    let g = renumber(arena, goal, &fs);
    let ghost middle = arena.nodes@;
    proof {
        prefix(start, middle, answers);
    }
    let a = renumber(arena, answers, &fs);
    proof {
        crate::k2_load::prefix_chain(start, middle, arena.nodes@);
        prefix(middle, arena.nodes@, &g);
    }
    (g, a)
}

} // verus!
