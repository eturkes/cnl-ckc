#[cfg(verus_keep_ghost)]
use crate::k4_bytes::byte_rows_push;
use crate::k4_bytes::{copy, first, range, split, starts_with};
use crate::k4_words::{normal, trim};
use ckc_spec::check::*;
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub open spec fn entry_view(r: Option<(Vec<u8>, Vec<u8>, Vec<u8>)>) -> Option<
    (Seq<u8>, Seq<u8>, Seq<u8>),
> {
    match r {
        Some((k, s, l)) => Some((k@, s@, l@)),
        None => None,
    }
}

pub fn entry(line: &[u8]) -> (r: Option<(Vec<u8>, Vec<u8>, Vec<u8>)>)
    ensures
        entry_view(r) == entry_of(line@),
{
    let p = first(line, 0x28, 0);
    if p >= line.len() {
        return None;
    }
    let kind = range(line, 0, p);
    let rest = range(line, p + 1, line.len());
    let quoted = split(&rest, 0x27);
    if quoted.len() > 3 {
        Some((kind, copy(&quoted[1]), copy(&quoted[3])))
    } else {
        let bare = split(&rest, 0x2c);
        if bare.len() < 2 {
            None
        } else {
            Some((kind, copy(&bare[0]), copy(&bare[1])))
        }
    }
}

pub fn surface(line: &[u8]) -> (r: Vec<u8>)
    ensures
        r@ == surface_of(line@),
{
    match entry(line) {
        Some((_, s, _)) => s,
        None => Vec::new(),
    }
}

pub fn punct(b: u8) -> (r: bool)
    ensures
        r == (b == 0x2e || b == 0x2c || b == 0x3b || b == 0x3a || b == 0x3f || b == 0x21 || b
            == 0x22 || b == 0x28 || b == 0x29),
{
    b == 0x2e || b == 0x2c || b == 0x3b || b == 0x3a || b == 0x3f || b == 0x21 || b == 0x22 || b
        == 0x28 || b == 0x29
}

pub fn lead_punct(s: &[u8]) -> (i: usize)
    ensures
        i <= s@.len(),
        i as nat == lead_while(
            s@,
            0,
            |b: u8|
                b == 0x2e || b == 0x2c || b == 0x3b || b == 0x3a || b == 0x3f || b == 0x21 || b
                    == 0x22 || b == 0x28 || b == 0x29,
        ),
{
    let mut i = 0usize;
    while i < s.len()
        invariant
            i <= s@.len(),
            lead_while(
                s@,
                0,
                |b: u8|
                    b == 0x2e || b == 0x2c || b == 0x3b || b == 0x3a || b == 0x3f || b == 0x21 || b
                        == 0x22 || b == 0x28 || b == 0x29,
            ) == lead_while(
                s@,
                i as nat,
                |b: u8|
                    b == 0x2e || b == 0x2c || b == 0x3b || b == 0x3a || b == 0x3f || b == 0x21 || b
                        == 0x22 || b == 0x28 || b == 0x29,
            ),
        decreases s.len() - i,
    {
        let hit = punct(s[i]);
        if !hit {
            proof {
                reveal_with_fuel(lead_while, 2);
                assert(lead_while(
                    s@,
                    i as nat,
                    |b: u8|
                        b == 0x2e || b == 0x2c || b == 0x3b || b == 0x3a || b == 0x3f || b == 0x21
                            || b == 0x22 || b == 0x28 || b == 0x29,
                ) == i);
            }
            return i;
        }
        proof {
            reveal_with_fuel(lead_while, 2);
        }
        i += 1;
    }
    proof {
        reveal_with_fuel(lead_while, 2);
    }
    i
}

pub fn trail_punct(s: &[u8]) -> (n: usize)
    ensures
        n <= s@.len(),
        n as nat == trail_while(
            s@,
            s@.len(),
            |b: u8|
                b == 0x2e || b == 0x2c || b == 0x3b || b == 0x3a || b == 0x3f || b == 0x21 || b
                    == 0x22 || b == 0x28 || b == 0x29,
        ),
{
    let mut i = s.len();
    let mut n = 0usize;
    while i > 0
        invariant
            i <= s@.len(),
            n + i == s@.len(),
            trail_while(
                s@,
                s@.len(),
                |b: u8|
                    b == 0x2e || b == 0x2c || b == 0x3b || b == 0x3a || b == 0x3f || b == 0x21 || b
                        == 0x22 || b == 0x28 || b == 0x29,
            ) == n as nat + trail_while(
                s@,
                i as nat,
                |b: u8|
                    b == 0x2e || b == 0x2c || b == 0x3b || b == 0x3a || b == 0x3f || b == 0x21 || b
                        == 0x22 || b == 0x28 || b == 0x29,
            ),
        decreases i,
    {
        let hit = punct(s[i - 1]);
        if !hit {
            proof {
                reveal_with_fuel(trail_while, 2);
                assert(trail_while(
                    s@,
                    i as nat,
                    |b: u8|
                        b == 0x2e || b == 0x2c || b == 0x3b || b == 0x3a || b == 0x3f || b == 0x21
                            || b == 0x22 || b == 0x28 || b == 0x29,
                ) == 0);
            }
            return n;
        }
        proof {
            reveal_with_fuel(trail_while, 2);
        }
        n += 1;
        i -= 1;
    }
    proof {
        reveal_with_fuel(trail_while, 2);
    }
    n
}

pub fn token(s: &[u8]) -> (r: Vec<u8>)
    ensures
        r@ == strip_tok(s@),
{
    let l = lead_punct(s);
    let u = range(s, l, s.len());
    let n = trail_punct(&u);
    range(&u, 0, u.len() - n)
}

pub fn clex_skip(s: &[u8]) -> (r: bool)
    ensures
        r == (s@.len() == 0 || starts(s@, ckc_spec::v1text::ascii("%"@)) || starts(
            s@,
            ckc_spec::v1text::ascii(":-"@),
        )),
{
    let r = s.len() == 0 || starts_with(s, b"%") || starts_with(s, b":-");
    proof {
        reveal_byteslit(b"%");
        reveal_strlit("%");
        reveal_byteslit(b":-");
        reveal_strlit(":-");
        reveal(ckc_spec::v1text::ascii);
        assert(b"%"@ == ckc_spec::v1text::ascii("%"@));
        assert(b":-"@ == ckc_spec::v1text::ascii(":-"@));
        assert(r == (s@.len() == 0 || starts(s@, ckc_spec::v1text::ascii("%"@)) || starts(
            s@,
            ckc_spec::v1text::ascii(":-"@),
        )));
    }
    r
}

pub proof fn filter_same<A>(s: Seq<A>, p: spec_fn(A) -> bool, q: spec_fn(A) -> bool)
    requires
        forall|x: A| #[trigger] p(x) == q(x),
    ensures
        s.filter(p) == s.filter(q),
    decreases s.len(),
{
    if s.len() > 0 {
        filter_same(s.drop_last(), p, q);
        s.drop_last().lemma_filter_push(s.last(), p);
        s.drop_last().lemma_filter_push(s.last(), q);
        assert_seqs_equal!(s.drop_last().push(s.last()) == s);
    }
}

pub fn lines(s: &[u8], clex: bool) -> (out: Vec<Vec<u8>>)
    ensures
        byte_rows(out@) == if clex {
            clex_lines(s@)
        } else {
            ulex_lines(s@)
        },
{
    let input = split(s, 0x0a);
    let mut out = Vec::new();
    let mut i = 0usize;
    let ghost f = |l: Seq<u8>| strip_ws(l);
    let ghost p = |l: Seq<u8>|
        l.len() > 0 && (!clex || (!starts(l, ckc_spec::v1text::ascii("%"@)) && !starts(
            l,
            ckc_spec::v1text::ascii(":-"@),
        )));
    while i < input.len()
        invariant
            i <= input@.len(),
            byte_rows(out@) == byte_rows(input@).take(i as int).map_values(f).filter(p),
            forall|l: Seq<u8>| #[trigger] f(l) == strip_ws(l),
            forall|l: Seq<u8>| #[trigger]
                p(l) == (l.len() > 0 && (!clex || (!starts(l, ckc_spec::v1text::ascii("%"@))
                    && !starts(l, ckc_spec::v1text::ascii(":-"@))))),
        decreases input.len() - i,
    {
        let l = trim(&input[i]);
        let keep = if clex {
            !clex_skip(&l)
        } else {
            l.len() > 0
        };
        proof {
            assert(l@ == f(input@[i as int]@));
            assert(keep == p(l@));
            byte_rows(input@).lemma_map_take_succ(f, i as int);
            byte_rows(input@).take(i as int).map_values(f).lemma_filter_push(l@, p);
        }
        if keep {
            proof {
                byte_rows_push(out@, l);
            }
            out.push(l);
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(byte_rows(input@).take(i as int) == byte_rows(input@));
        let mapped = byte_rows(input@).map_values(f);
        assert_seqs_equal!(mapped == split_on(s@, 0x0a).map_values(|l: Seq<u8>| strip_ws(l)));
        if clex {
            let q = |l: Seq<u8>|
                l.len() > 0 && !starts(l, ckc_spec::v1text::ascii("%"@)) && !starts(
                    l,
                    ckc_spec::v1text::ascii(":-"@),
                );
            assert forall|l: Seq<u8>| #[trigger] p(l) == q(l) by {};
            filter_same(mapped, p, q);
        } else {
            let q = |l: Seq<u8>| l.len() > 0;
            assert forall|l: Seq<u8>| #[trigger] p(l) == q(l) by {};
            filter_same(mapped, p, q);
        }
    }
    out
}

pub fn normals(lines: &Vec<Vec<u8>>) -> (out: Vec<Vec<u8>>)
    ensures
        out@.len() == lines@.len(),
        byte_rows(out@) == byte_rows(lines@).map_values(|l: Seq<u8>| normalized(l)),
{
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < lines.len()
        invariant
            i <= lines@.len(),
            out@.len() == i,
            byte_rows(out@) == byte_rows(lines@).take(i as int).map_values(
                |l: Seq<u8>| normalized(l),
            ),
        decreases lines.len() - i,
    {
        let n = normal(&lines[i]);
        proof {
            byte_rows_push(out@, n);
            byte_rows(lines@).lemma_map_take_succ(|l: Seq<u8>| normalized(l), i as int);
        }
        out.push(n);
        i += 1;
    }
    proof {
        assert_seqs_equal!(byte_rows(lines@).take(i as int) == byte_rows(lines@));
    }
    out
}

} // verus!
