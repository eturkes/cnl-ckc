use crate::k2_term::ETermArena;
#[cfg(verus_keep_ghost)]
use crate::k2_term::arena_ok;
use crate::m6_term::*;
use ckc_spec::term::{self, Term};
use vstd::prelude::*;
use vstd::{assert_seqs_equal, assert_sets_equal};

verus! {

pub open spec fn nums(xs: Seq<usize>) -> Seq<nat> {
    xs.map_values(|x: usize| x as nat)
}

pub fn concat_nums(a: &Vec<usize>, b: &Vec<usize>) -> (out: Vec<usize>)
    ensures
        out@ == a@ + b@,
        nums(out@) == nums(a@) + nums(b@),
{
    let mut out = a.clone();
    let mut rest = b.clone();
    out.append(&mut rest);
    proof {
        assert_seqs_equal!(nums(out@) == nums(a@) + nums(b@));
    }
    out
}

pub proof fn streams_concat(a: Seq<Term>, b: Seq<Term>)
    ensures
        term::var_stream_all(a + b) == term::var_stream_all(a) + term::var_stream_all(b),
    decreases a.len(),
{
    if a.len() > 0 {
        assert((a + b).drop_first() == a.drop_first() + b);
        streams_concat(a.drop_first(), b);
    }
    reveal_with_fuel(term::var_stream_all, 2);
}

pub fn stream(arena: &ETermArena, t: &T) -> (out: Vec<usize>)
    requires
        arena_ok(arena),
        valid(arena.nodes@, t),
    ensures
        nums(out@) == term::var_stream(t@),
{
    let mut todo = Vec::new();
    todo.push(t.cp());
    let mut out = Vec::new();
    proof {
        reveal_with_fuel(term::var_stream_all, 2);
        assert_seqs_equal!(nums(out@) == Seq::<nat>::empty());
    }
    while todo.len() > 0
        invariant
            arena_ok(arena),
            valid(arena.nodes@, t),
            valid_all(arena.nodes@, todo@),
            nums(out@) + term::var_stream_all(models(todo@)) == term::var_stream(t@),
        decreases crate::k2_engine::terms_size(models(todo@)),
    {
        let ghost before = todo@;
        let current = todo.remove(0);
        proof {
            assert(current == before[0]);
            assert(models(before)[0] == current@);
            assert(models(before).drop_first() == models(todo@));
            reveal_with_fuel(term::var_stream_all, 1);
            reveal_with_fuel(term::var_stream, 1);
            reveal_with_fuel(crate::k2_engine::terms_size, 1);
            assert(term::var_stream_all(models(before)) == term::var_stream(current@)
                + term::var_stream_all(models(todo@)));
            assert(crate::k2_engine::terms_size(models(before)) == crate::k2_engine::term_size(
                current@,
            ) + crate::k2_engine::terms_size(models(todo@)));
        }
        if is_var(arena, &current) {
            let key = var_index(arena, &current);
            let ghost previous = out@;
            out.push(key);
            proof {
                assert_seqs_equal!(nums(out@) == nums(previous).push(key as nat));
                reveal(crate::k2_engine::term_size);
            }
        } else if let Some((name, children)) = parts(arena, &current) {
            proof {
                assert(current@ == Term::Comp(name@, models(children@)));
                streams_concat(models(children@), models(todo@));
                crate::m6_drs::sizes_concat(models(children@), models(todo@));
                reveal(crate::k2_engine::term_size);
            }
            todo = concat(&children, &todo);
        } else {
            proof {
                reveal(crate::k2_engine::term_size);
            }
        }
    }
    proof {
        reveal(term::var_stream_all);
    }
    out
}

pub fn stream_all(arena: &ETermArena, ts: &Vec<T>) -> (out: Vec<usize>)
    requires
        arena_ok(arena),
        valid_all(arena.nodes@, ts@),
    ensures
        nums(out@) == term::var_stream_all(models(ts@)),
{
    let mut out = Vec::new();
    let mut i = 0usize;
    proof {
        assert(models(ts@).skip(0) == models(ts@));
        assert_seqs_equal!(nums(out@) == Seq::<nat>::empty());
    }
    while i < ts.len()
        invariant
            arena_ok(arena),
            valid_all(arena.nodes@, ts@),
            i <= ts.len(),
            nums(out@) + term::var_stream_all(models(ts@).skip(i as int)) == term::var_stream_all(
                models(ts@),
            ),
        decreases ts.len() - i,
    {
        proof {
            assert(models(ts@).skip(i as int).drop_first() == models(ts@).skip(i as int + 1));
            reveal_with_fuel(term::var_stream_all, 1);
        }
        let next = stream(arena, &ts[i]);
        out = concat_nums(&out, &next);
        i += 1;
    }
    proof {
        reveal(term::var_stream_all);
    }
    out
}

pub fn member(xs: &Vec<usize>, x: usize) -> (out: bool)
    ensures
        out == nums(xs@).contains(x as nat),
{
    let mut i = 0usize;
    while i < xs.len()
        invariant
            i <= xs.len(),
            forall|j: int| 0 <= j < i ==> #[trigger] nums(xs@)[j] != x as nat,
        decreases xs.len() - i,
    {
        if xs[i] == x {
            proof {
                assert(nums(xs@)[i as int] == x as nat);
            }
            return true;
        }
        i += 1;
    }
    false
}

pub fn subset(a: &Vec<usize>, b: &Vec<usize>) -> (out: bool)
    ensures
        out == nums(a@).to_set().subset_of(nums(b@).to_set()),
{
    let mut i = 0usize;
    while i < a.len()
        invariant
            i <= a.len(),
            forall|j: int| 0 <= j < i ==> nums(b@).to_set().contains(#[trigger] nums(a@)[j]),
        decreases a.len() - i,
    {
        if !member(b, a[i]) {
            proof {
                assert(nums(a@).to_set().contains(nums(a@)[i as int]));
            }
            return false;
        }
        i += 1;
    }
    proof {
        assert forall|x: nat| nums(a@).to_set().contains(x) implies nums(b@).to_set().contains(
            x,
        ) by {
            let j = choose|j: int| 0 <= j < nums(a@).len() && nums(a@)[j] == x;
            assert(nums(b@).to_set().contains(nums(a@)[j]));
        }
    }
    true
}

pub fn firsts(xs: &Vec<usize>) -> (out: Vec<usize>)
    ensures
        nums(out@) == term::firsts(nums(xs@), Set::empty()),
{
    let mut out = Vec::new();
    let mut i = 0usize;
    proof {
        assert(nums(xs@).skip(0) == nums(xs@));
        assert_seqs_equal!(nums(out@) == Seq::<nat>::empty());
        nums(out@).to_set_ensures();
        assert_sets_equal!(nums(out@).to_set() == Set::<nat>::empty());
    }
    while i < xs.len()
        invariant
            i <= xs.len(),
            nums(out@) + term::firsts(nums(xs@).skip(i as int), nums(out@).to_set())
                == term::firsts(nums(xs@), Set::empty()),
        decreases xs.len() - i,
    {
        proof {
            assert(nums(xs@).skip(i as int).drop_first() == nums(xs@).skip(i as int + 1));
            reveal_with_fuel(term::firsts, 1);
        }
        if !member(&out, xs[i]) {
            let ghost before = out@;
            out.push(xs[i]);
            proof {
                assert_seqs_equal!(nums(out@) == nums(before).push(xs@[i as int] as nat));
                nums(before).lemma_push_to_set_commute(xs@[i as int] as nat);
                assert_sets_equal!(nums(out@).to_set() == nums(before).to_set().insert(xs@[i as int] as nat));
            }
        }
        i += 1;
    }
    proof {
        reveal(term::firsts);
    }
    out
}

pub fn position(fs: &Vec<usize>, x: usize) -> (out: usize)
    ensures
        out as nat == ckc_spec::trace::pos_of(nums(fs@), x as nat),
        out <= fs.len(),
{
    let mut i = 0usize;
    proof {
        assert(nums(fs@).skip(0) == nums(fs@));
    }
    while i < fs.len()
        invariant
            i <= fs.len(),
            ckc_spec::trace::pos_of(nums(fs@), x as nat) == i as nat + ckc_spec::trace::pos_of(
                nums(fs@).skip(i as int),
                x as nat,
            ),
        decreases fs.len() - i,
    {
        proof {
            assert(nums(fs@).skip(i as int).drop_first() == nums(fs@).skip(i as int + 1));
            reveal_with_fuel(ckc_spec::trace::pos_of, 1);
        }
        if fs[i] == x {
            return i;
        }
        i += 1;
    }
    proof {
        reveal(ckc_spec::trace::pos_of);
    }
    i
}

pub open spec fn occurrences(xs: Seq<nat>, x: nat) -> nat {
    xs.filter(|k: nat| k == x).len()
}

pub proof fn occurrence_step(xs: Seq<nat>, k: nat, x: nat)
    ensures
        occurrences(xs.push(k), x) == occurrences(xs, x) + if k == x {
            1nat
        } else {
            0nat
        },
{
    reveal(Seq::filter);
    reveal(occurrences);
    assert(xs.push(k).drop_last() == xs);
    assert(xs.push(k).last() == k);
}

pub fn count(xs: &Vec<usize>, x: usize) -> (out: usize)
    ensures
        out as nat == occurrences(nums(xs@), x as nat),
        out <= xs.len(),
{
    let mut out = 0usize;
    let mut i = 0usize;
    proof {
        reveal(Seq::filter);
        reveal(occurrences);
    }
    while i < xs.len()
        invariant
            i <= xs.len(),
            out <= i,
            out as nat == occurrences(nums(xs@).take(i as int), x as nat),
        decreases xs.len() - i,
    {
        proof {
            assert(nums(xs@).take(i as int + 1) == nums(xs@).take(i as int).push(
                xs@[i as int] as nat,
            ));
            occurrence_step(nums(xs@).take(i as int), xs@[i as int] as nat, x as nat);
        }
        if xs[i] == x {
            out += 1;
        }
        i += 1;
    }
    proof {
        assert(nums(xs@).take(i as int) == nums(xs@));
    }
    out
}

} // verus!
