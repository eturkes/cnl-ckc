#[cfg(verus_keep_ghost)]
use crate::k4_bytes::byte_rows_push;
use crate::k4_bytes::{append, copy, eq, less};
use crate::k4_lexparse::{lines, normals, surface, token};
use crate::k4_set::{self, ByteSet};
use crate::k4_words;
use ckc_spec::check::*;
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub fn set_from(rows: &Vec<Vec<u8>>) -> (set: ByteSet)
    ensures
        k4_set::valid(&set),
        set@ == byte_rows(rows@).to_set(),
{
    let mut set = k4_set::empty();
    let ghost mut n = 0nat;
    let mut i = 0usize;
    while i < rows.len()
        invariant
            i <= rows@.len(),
            n <= i,
            n as nat == set@.len(),
            k4_set::valid(&set),
            set@ == byte_rows(rows@).take(i as int).to_set(),
        decreases rows.len() - i,
    {
        let added = k4_set::insert(&mut set, &rows[i]);
        proof {
            assert_seqs_equal!(byte_rows(rows@).take(i as int + 1) == byte_rows(rows@).take(i as int).push(rows@[i as int]@));
            byte_rows(rows@).take(i as int).lemma_push_to_set_commute(rows@[i as int]@);
        }
        if added {
            proof {
                n = n + 1;
            }
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(byte_rows(rows@).take(i as int) == byte_rows(rows@));
    }
    set
}

pub proof fn dedup_push(xs: Seq<Seq<u8>>, x: Seq<u8>)
    ensures
        dedup_bytes(xs.push(x)) == if dedup_bytes(xs).contains(x) {
            dedup_bytes(xs)
        } else {
            dedup_bytes(xs).push(x)
        },
{
    assert_seqs_equal!(xs.push(x).drop_last() == xs);
    reveal_with_fuel(Seq::fold_left, 2);
}

pub fn unique(input: &Vec<Vec<u8>>) -> (out: Vec<Vec<u8>>)
    ensures
        byte_rows(out@) == dedup_bytes(byte_rows(input@)),
{
    let mut seen = k4_set::empty();
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < input.len()
        invariant
            i <= input@.len(),
            out@.len() <= i,
            k4_set::valid(&seen),
            seen@ == byte_rows(out@).to_set(),
            seen@.len() == out@.len(),
            byte_rows(out@) == dedup_bytes(byte_rows(input@).take(i as int)),
        decreases input.len() - i,
    {
        let ghost old_out = byte_rows(out@);
        let added = k4_set::insert(&mut seen, &input[i]);
        proof {
            dedup_push(byte_rows(input@).take(i as int), input@[i as int]@);
            assert_seqs_equal!(byte_rows(input@).take(i as int + 1) == byte_rows(input@).take(i as int).push(input@[i as int]@));
            assert(added == !old_out.contains(input@[i as int]@));
            old_out.lemma_push_to_set_commute(input@[i as int]@);
        }
        if added {
            let x = copy(&input[i]);
            proof {
                byte_rows_push(out@, x);
            }
            out.push(x);
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(byte_rows(input@).take(i as int) == byte_rows(input@));
    }
    out
}

pub proof fn insert_at(x: Seq<u8>, s: Seq<Seq<u8>>, i: nat)
    requires
        i <= s.len(),
        forall|j: int| 0 <= j < i ==> ckc_spec::engine::bytes_lt(s[j], x),
        i == s.len() || !ckc_spec::engine::bytes_lt(s[i as int], x),
    ensures
        insert_bytes(x, s) == s.take(i as int) + seq![x] + s.skip(i as int),
    decreases i,
{
    reveal_with_fuel(insert_bytes, 2);
    if i > 0 {
        insert_at(x, s.drop_first(), (i - 1) as nat);
        assert_seqs_equal!(s.take(i as int) == seq![s[0]] + s.drop_first().take(i as int - 1));
        assert_seqs_equal!(s.skip(i as int) == s.drop_first().skip(i as int - 1));
    }
}

pub fn insert_row(x: &[u8], rows: &Vec<Vec<u8>>) -> (out: Vec<Vec<u8>>)
    ensures
        byte_rows(out@) == insert_bytes(x@, byte_rows(rows@)),
{
    let mut i = 0usize;
    while i < rows.len() && less(&rows[i], x)
        invariant
            i <= rows@.len(),
            forall|j: int| 0 <= j < i ==> ckc_spec::engine::bytes_lt(rows@[j]@, x@),
        decreases rows.len() - i,
    {
        i += 1;
    }
    let mut out = k4_set::copy_rows(rows);
    let item = copy(x);
    let ghost before = out@;
    out.insert(i, item);
    proof {
        insert_at(x@, byte_rows(rows@), i as nat);
        assert_seqs_equal!(byte_rows(out@) == byte_rows(before).take(i as int) + seq![x@] + byte_rows(before).skip(i as int));
    }
    out
}

pub fn sorted(rows: &Vec<Vec<u8>>) -> (out: Vec<Vec<u8>>)
    ensures
        byte_rows(out@) == sort_bytes(byte_rows(rows@)),
{
    let mut out = Vec::new();
    let mut i = rows.len();
    while i > 0
        invariant
            i <= rows@.len(),
            byte_rows(out@) == sort_bytes(byte_rows(rows@).skip(i as int)),
        decreases i,
    {
        let next = insert_row(&rows[i - 1], &out);
        proof {
            assert_seqs_equal!(byte_rows(rows@).skip(i as int - 1).drop_first() == byte_rows(rows@).skip(i as int));
            reveal_with_fuel(sort_bytes, 2);
        }
        out = next;
        i -= 1;
    }
    proof {
        assert_seqs_equal!(byte_rows(rows@).skip(0) == byte_rows(rows@));
    }
    out
}

pub proof fn plus_push(s: Seq<Seq<u8>>, x: Seq<u8>)
    ensures
        join_plus(s.push(x)) == if s.len() == 0 {
            x
        } else {
            join_plus(s) + seq![0x2bu8] + x
        },
    decreases s.len(),
{
    reveal_with_fuel(join_plus, 2);
    if s.len() > 0 {
        assert_seqs_equal!(s.push(x).drop_first() == s.drop_first().push(x));
        if s.len() > 1 {
            plus_push(s.drop_first(), x);
        }
    }
}

pub fn plus(rows: &Vec<Vec<u8>>) -> (out: Vec<u8>)
    ensures
        out@ == join_plus(byte_rows(rows@)),
{
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < rows.len()
        invariant
            i <= rows@.len(),
            out@ == join_plus(byte_rows(rows@).take(i as int)),
        decreases rows.len() - i,
    {
        proof {
            plus_push(byte_rows(rows@).take(i as int), rows@[i as int]@);
        }
        if i > 0 {
            out.push(0x2b);
        }
        append(&mut out, &rows[i]);
        proof {
            assert_seqs_equal!(byte_rows(rows@).take(i as int + 1) == byte_rows(rows@).take(i as int).push(rows@[i as int]@));
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(byte_rows(rows@).take(i as int) == byte_rows(rows@));
    }
    out
}

pub fn key(clex: &[u8], wanted: &[u8]) -> (r: Vec<u8>)
    ensures
        r@ == clex_key(clex@, wanted@),
{
    let input = lines(clex, true);
    let norm_rows = normals(&input);
    let mut selected = Vec::new();
    let mut i = 0usize;
    let ghost p = |l: Seq<u8>| surface_of(l) == wanted@;
    while i < input.len()
        invariant
            i <= input@.len(),
            norm_rows@.len() == input@.len(),
            byte_rows(input@) == clex_lines(clex@),
            byte_rows(norm_rows@) == byte_rows(input@).map_values(|l: Seq<u8>| normalized(l)),
            byte_rows(selected@) == byte_rows(input@).take(i as int).filter(p).map_values(
                |l: Seq<u8>| normalized(l),
            ),
            forall|l: Seq<u8>| #[trigger] p(l) == (surface_of(l) == wanted@),
        decreases input.len() - i,
    {
        let s = surface(&input[i]);
        let hit = eq(&s, wanted);
        proof {
            assert(hit == p(input@[i as int]@));
            assert_seqs_equal!(byte_rows(input@).take(i as int + 1) == byte_rows(input@).take(i as int).push(input@[i as int]@));
            byte_rows(input@).take(i as int).lemma_filter_push(input@[i as int]@, p);
        }
        if hit {
            let n = copy(&norm_rows[i]);
            proof {
                assert(n@ == norm_rows@[i as int]@);
                assert(byte_rows(norm_rows@)[i as int] == norm_rows@[i as int]@);
                assert(byte_rows(norm_rows@)[i as int] == normalized(byte_rows(input@)[i as int]));
                assert(byte_rows(input@)[i as int] == input@[i as int]@);
                assert(n@ == normalized(input@[i as int]@));
                byte_rows(input@).take(i as int).filter(p).lemma_push_map_commute(
                    |l: Seq<u8>| normalized(l),
                    input@[i as int]@,
                );
                byte_rows_push(selected@, n);
            }
            selected.push(n);
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(byte_rows(input@).take(i as int) == byte_rows(input@));
    }
    let unique = unique(&selected);
    let sorted = sorted(&unique);
    plus(&sorted)
}

pub fn token_rows(ace: &Vec<Vec<u8>>) -> (out: Vec<Vec<u8>>)
    ensures
        byte_rows(out@) == byte_rows(ace@).map_values(
            |t: Seq<u8>| ws_split(t).map_values(|w: Seq<u8>| strip_tok(w)),
        ).flatten(),
{
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < ace.len()
        invariant
            i <= ace@.len(),
            byte_rows(out@) == byte_rows(ace@).take(i as int).map_values(
                |t: Seq<u8>| ws_split(t).map_values(|w: Seq<u8>| strip_tok(w)),
            ).flatten(),
        decreases ace.len() - i,
    {
        let words = k4_words::tokens(&ace[i]);
        let mut j = 0usize;
        let ghost before = byte_rows(out@);
        while j < words.len()
            invariant
                j <= words@.len(),
                byte_rows(words@) == ws_split(ace@[i as int]@),
                byte_rows(out@) == before + byte_rows(words@).take(j as int).map_values(
                    |w: Seq<u8>| strip_tok(w),
                ),
            decreases words.len() - j,
        {
            let t = token(&words[j]);
            proof {
                byte_rows_push(out@, t);
                byte_rows(words@).lemma_map_take_succ(|w: Seq<u8>| strip_tok(w), j as int);
            }
            out.push(t);
            j += 1;
        }
        proof {
            byte_rows(ace@).lemma_map_take_succ(
                |t: Seq<u8>| ws_split(t).map_values(|w: Seq<u8>| strip_tok(w)),
                i as int,
            );
            byte_rows(ace@).take(i as int).map_values(
                |t: Seq<u8>| ws_split(t).map_values(|w: Seq<u8>| strip_tok(w)),
            ).lemma_flatten_push(ws_split(ace@[i as int]@).map_values(|w: Seq<u8>| strip_tok(w)));
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(byte_rows(ace@).take(i as int) == byte_rows(ace@));
    }
    out
}

} // verus!
