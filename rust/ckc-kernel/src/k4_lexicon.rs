#[cfg(verus_keep_ghost)]
use crate::k4_bytes::byte_rows_push;
use crate::k4_bytes::{append, concat, copy, eq};
use crate::k4_lexdata::{key, set_from, token_rows};
use crate::k4_lexparse::{entry, lines, normals, surface};
use crate::k4_scalar::number;
use crate::k4_set::{self, ByteSet};
use crate::k4_words::normal;
use ckc_spec::check::*;
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub open spec fn verdict_option(v: Option<EVerdict>) -> Option<Verdict> {
    match v {
        Some(x) => Some(x@),
        None => None,
    }
}

pub fn failure(cat: &[u8], detail: Vec<u8>) -> (v: EVerdict)
    ensures
        v@ == Verdict::Fail(cat@, detail@),
{
    EVerdict::Fail(copy(cat), detail)
}

pub fn surface_names(clex: &[u8]) -> (out: Vec<Vec<u8>>)
    ensures
        byte_rows(out@) == clex_lines(clex@).map_values(|l: Seq<u8>| surface_of(l)),
{
    let ls = lines(clex, true);
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < ls.len()
        invariant
            i <= ls@.len(),
            byte_rows(ls@) == clex_lines(clex@),
            byte_rows(out@) == byte_rows(ls@).take(i as int).map_values(|l: Seq<u8>| surface_of(l)),
        decreases ls.len() - i,
    {
        let s = surface(&ls[i]);
        proof {
            byte_rows_push(out@, s);
            byte_rows(ls@).lemma_map_take_succ(|l: Seq<u8>| surface_of(l), i as int);
        }
        out.push(s);
        i += 1;
    }
    proof {
        assert_seqs_equal!(byte_rows(ls@).take(i as int) == byte_rows(ls@));
    }
    out
}

pub fn shared(clex: &[u8], names: &Vec<Vec<u8>>, wanted: &[u8]) -> (r: bool)
    requires
        byte_rows(names@) == clex_lines(clex@).map_values(|l: Seq<u8>| surface_of(l)),
    ensures
        r == has_surface(clex@, wanted@),
{
    proof {
        assert(byte_rows(names@).len() == names@.len());
        assert(clex_lines(clex@).map_values(|l: Seq<u8>| surface_of(l)).len() == clex_lines(
            clex@,
        ).len());
    }
    let mut i = 0usize;
    while i < names.len()
        invariant
            i <= names@.len(),
            names@.len() == clex_lines(clex@).len(),
            byte_rows(names@) == clex_lines(clex@).map_values(|l: Seq<u8>| surface_of(l)),
            forall|j: int| 0 <= j < i ==> names@[j]@ != wanted@,
        decreases names.len() - i,
    {
        if eq(&names[i], wanted) {
            proof {
                assert(byte_rows(names@)[i as int] == names@[i as int]@);
                assert(byte_rows(names@)[i as int] == surface_of(clex_lines(clex@)[i as int]));
                assert(surface_of(clex_lines(clex@)[i as int]) == wanted@);
            }
            return true;
        }
        i += 1;
    }
    proof {
        assert forall|j: int| 0 <= j < clex_lines(clex@).len() implies surface_of(
            clex_lines(clex@)[j],
        ) != wanted@ by {
            assert(byte_rows(names@)[j] == names@[j]@);
            assert(byte_rows(names@)[j] == surface_of(clex_lines(clex@)[j]));
            assert(names@[j]@ == surface_of(clex_lines(clex@)[j]));
        }
        reveal(has_surface);
    }
    false
}

pub fn rule(rs: &Vec<(Vec<u8>, Vec<u8>)>, norm: &[u8], clex_key: &[u8]) -> (r: bool)
    ensures
        r == byte_pairs(rs@).contains((norm@, clex_key@)),
{
    let mut i = 0usize;
    while i < rs.len()
        invariant
            i <= rs@.len(),
            forall|j: int| 0 <= j < i ==> rs@[j].0@ != norm@ || rs@[j].1@ != clex_key@,
        decreases rs.len() - i,
    {
        if eq(&rs[i].0, norm) && eq(&rs[i].1, clex_key) {
            proof {
                assert(byte_pairs(rs@)[i as int] == (norm@, clex_key@));
            }
            return true;
        }
        i += 1;
    }
    false
}

pub fn audit(
    ls: &Vec<Vec<u8>>,
    clex: &[u8],
    norms: &ByteSet,
    names: &Vec<Vec<u8>>,
    rs: &Vec<(Vec<u8>, Vec<u8>)>,
) -> (r: Option<EVerdict>)
    requires
        norms@ == clex_norms(clex@),
        byte_rows(names@) == clex_lines(clex@).map_values(|l: Seq<u8>| surface_of(l)),
    ensures
        verdict_option(r) == ulex_check(byte_rows(ls@), 0, clex@, byte_pairs(rs@), Seq::empty()),
{
    let mut seen = k4_set::empty();
    let mut seen_rows = Vec::new();
    let mut i = 0usize;
    proof {
        assert(byte_rows(seen_rows@) == Seq::<Seq<u8>>::empty());
    }
    while i < ls.len()
        invariant
            i <= ls@.len(),
            norms@ == clex_norms(clex@),
            byte_rows(names@) == clex_lines(clex@).map_values(|l: Seq<u8>| surface_of(l)),
            k4_set::valid(&seen),
            seen@ == byte_rows(seen_rows@).to_set(),
            seen@.len() == i,
            seen_rows@.len() == i,
            ulex_check(byte_rows(ls@), 0, clex@, byte_pairs(rs@), Seq::empty()) == ulex_check(
                byte_rows(ls@),
                i as nat,
                clex@,
                byte_pairs(rs@),
                byte_rows(seen_rows@),
            ),
        decreases ls.len() - i,
    {
        let l = &ls[i];
        let n = normal(l);
        proof {
            reveal(ckc_spec::v1text::ascii);
            reveal_byteslit(b"lexicon-redundant");
            reveal_strlit("lexicon-redundant");
            assert(b"lexicon-redundant"@ == ckc_spec::v1text::ascii("lexicon-redundant"@));
            reveal_byteslit(b"clex already provides: ");
            reveal_strlit("clex already provides: ");
            assert(b"clex already provides: "@ == ckc_spec::v1text::ascii(
                "clex already provides: "@,
            ));
            reveal_byteslit(b"lexicon-duplicate");
            reveal_strlit("lexicon-duplicate");
            assert(b"lexicon-duplicate"@ == ckc_spec::v1text::ascii("lexicon-duplicate"@));
            reveal_byteslit(b"entry repeated in lexicon: ");
            reveal_strlit("entry repeated in lexicon: ");
            assert(b"entry repeated in lexicon: "@ == ckc_spec::v1text::ascii(
                "entry repeated in lexicon: "@,
            ));
            reveal_byteslit(b"lexicon-entry");
            reveal_strlit("lexicon-entry");
            assert(b"lexicon-entry"@ == ckc_spec::v1text::ascii("lexicon-entry"@));
            reveal_byteslit(b"malformed entry: ");
            reveal_strlit("malformed entry: ");
            assert(b"malformed entry: "@ == ckc_spec::v1text::ascii("malformed entry: "@));
            reveal_byteslit(b"lexicon-shadow");
            reveal_strlit("lexicon-shadow");
            assert(b"lexicon-shadow"@ == ckc_spec::v1text::ascii("lexicon-shadow"@));
            reveal_byteslit(b"clex shares surface without ruling: ");
            reveal_strlit("clex shares surface without ruling: ");
            assert(b"clex shares surface without ruling: "@ == ckc_spec::v1text::ascii(
                "clex shares surface without ruling: "@,
            ));
            reveal_byteslit(b" vs ");
            reveal_strlit(" vs ");
            assert(b" vs "@ == ckc_spec::v1text::ascii(" vs "@));
            reveal_with_fuel(ulex_check, 2);
        }
        if k4_set::has(norms, &n) {
            return Some(failure(b"lexicon-redundant", concat(b"clex already provides: ", l)));
        }
        if k4_set::has(&seen, &n) {
            return Some(failure(b"lexicon-duplicate", concat(b"entry repeated in lexicon: ", l)));
        }
        let s = match entry(l) {
            Some((_, s, _)) => s,
            None => return Some(failure(b"lexicon-entry", concat(b"malformed entry: ", l))),
        };
        if shared(clex, names, &s) {
            let k = key(clex, &s);
            if !rule(rs, &n, &k) {
                let mut d = concat(b"clex shares surface without ruling: ", l);
                append(&mut d, b" vs ");
                append(&mut d, &k);
                return Some(failure(b"lexicon-shadow", d));
            }
        }
        let added = k4_set::insert(&mut seen, &n);
        proof {
            assert(added);
            byte_rows_push(seen_rows@, n);
            byte_rows(seen_rows@).lemma_push_to_set_commute(n@);
        }
        seen_rows.push(n);
        i += 1;
    }
    proof {
        reveal_with_fuel(ulex_check, 2);
    }
    None
}

pub fn used(clex: &[u8], ls: &Vec<Vec<u8>>, names: &Vec<Vec<u8>>, r: &(Vec<u8>, Vec<u8>)) -> (used:
    bool)
    requires
        byte_rows(names@) == clex_lines(clex@).map_values(|l: Seq<u8>| surface_of(l)),
    ensures
        used == ruling_used(clex@, byte_rows(ls@), (r.0@, r.1@)),
{
    let mut i = 0usize;
    while i < ls.len()
        invariant
            i <= ls@.len(),
            byte_rows(names@) == clex_lines(clex@).map_values(|l: Seq<u8>| surface_of(l)),
            forall|j: int|
                0 <= j < i ==> !(normalized(ls@[j]@) == r.0@ && has_surface(
                    clex@,
                    surface_of(ls@[j]@),
                ) && clex_key(clex@, surface_of(ls@[j]@)) == r.1@),
        decreases ls.len() - i,
    {
        let n = normal(&ls[i]);
        if eq(&n, &r.0) {
            let s = surface(&ls[i]);
            if shared(clex, names, &s) && eq(&key(clex, &s), &r.1) {
                proof {
                    assert(normalized(byte_rows(ls@)[i as int]) == r.0@);
                    assert(has_surface(clex@, surface_of(byte_rows(ls@)[i as int])));
                    assert(clex_key(clex@, surface_of(byte_rows(ls@)[i as int])) == r.1@);
                    assert(ruling_used(clex@, byte_rows(ls@), (r.0@, r.1@)));
                }
                return true;
            }
        }
        i += 1;
    }
    false
}

pub fn stale(
    clex: &[u8],
    ls: &Vec<Vec<u8>>,
    names: &Vec<Vec<u8>>,
    rs: &Vec<(Vec<u8>, Vec<u8>)>,
) -> (r: Option<Vec<u8>>)
    requires
        byte_rows(names@) == clex_lines(clex@).map_values(|l: Seq<u8>| surface_of(l)),
    ensures
        optional_bytes(r) == first_stale(clex@, byte_rows(ls@), byte_pairs(rs@), 0),
{
    let mut i = 0usize;
    while i < rs.len()
        invariant
            i <= rs@.len(),
            byte_rows(names@) == clex_lines(clex@).map_values(|l: Seq<u8>| surface_of(l)),
            first_stale(clex@, byte_rows(ls@), byte_pairs(rs@), 0) == first_stale(
                clex@,
                byte_rows(ls@),
                byte_pairs(rs@),
                i as nat,
            ),
        decreases rs.len() - i,
    {
        let live = used(clex, ls, names, &rs[i]);
        proof {
            reveal_with_fuel(first_stale, 2);
        }
        if !live {
            return Some(copy(&rs[i].0));
        }
        i += 1;
    }
    proof {
        reveal_with_fuel(first_stale, 2);
    }
    None
}

pub fn lemma_live(ls: &Vec<Vec<u8>>, toks: &ByteSet, wanted: &[u8]) -> (r: bool)
    ensures
        r == live(byte_rows(ls@), toks@, wanted@),
{
    let mut i = 0usize;
    while i < ls.len()
        invariant
            i <= ls@.len(),
            forall|j: int|
                0 <= j < i ==> !(match #[trigger] entry_of(ls@[j]@) {
                    Some((_, s, l)) => l == wanted@ && toks@.contains(s),
                    None => false,
                }),
        decreases ls.len() - i,
    {
        if let Some((_, s, l)) = entry(&ls[i]) {
            if eq(&l, wanted) && k4_set::has(toks, &s) {
                proof {
                    assert(match entry_of(byte_rows(ls@)[i as int]) {
                        Some((_, sf, lm)) => lm == wanted@ && toks@.contains(sf),
                        None => false,
                    });
                    assert(live(byte_rows(ls@), toks@, wanted@));
                }
                return true;
            }
        }
        i += 1;
    }
    false
}

pub fn dead(ls: &Vec<Vec<u8>>, toks: &ByteSet) -> (r: Option<Vec<u8>>)
    ensures
        optional_bytes(r) == first_dead(byte_rows(ls@), toks@, 0),
{
    let mut i = 0usize;
    while i < ls.len()
        invariant
            i <= ls@.len(),
            first_dead(byte_rows(ls@), toks@, 0) == first_dead(byte_rows(ls@), toks@, i as nat),
        decreases ls.len() - i,
    {
        proof {
            reveal_with_fuel(first_dead, 2);
        }
        if let Some((_, s, l)) = entry(&ls[i]) {
            if !lemma_live(ls, toks, &l) {
                return Some(s);
            }
        }
        i += 1;
    }
    proof {
        reveal_with_fuel(first_dead, 2);
    }
    None
}

pub fn ruled(
    clex: &[u8],
    ls: &Vec<Vec<u8>>,
    names: &Vec<Vec<u8>>,
    rs: &Vec<(Vec<u8>, Vec<u8>)>,
) -> (n: usize)
    requires
        byte_rows(names@) == clex_lines(clex@).map_values(|l: Seq<u8>| surface_of(l)),
    ensures
        n as nat == ruled_count(clex@, byte_rows(ls@), byte_pairs(rs@)),
{
    let ghost p = |l: Seq<u8>|
        has_surface(clex@, surface_of(l)) && byte_pairs(rs@).contains(
            (normalized(l), clex_key(clex@, surface_of(l))),
        );
    let mut i = 0usize;
    let mut n = 0usize;
    while i < ls.len()
        invariant
            i <= ls@.len(),
            n <= i,
            byte_rows(names@) == clex_lines(clex@).map_values(|l: Seq<u8>| surface_of(l)),
            n as nat == byte_rows(ls@).take(i as int).filter(p).len(),
            forall|l: Seq<u8>| #[trigger]
                p(l) == (has_surface(clex@, surface_of(l)) && byte_pairs(rs@).contains(
                    (normalized(l), clex_key(clex@, surface_of(l))),
                )),
        decreases ls.len() - i,
    {
        let s = surface(&ls[i]);
        let hit = shared(clex, names, &s) && rule(rs, &normal(&ls[i]), &key(clex, &s));
        proof {
            assert(hit == p(ls@[i as int]@));
            assert_seqs_equal!(byte_rows(ls@).take(i as int + 1) == byte_rows(ls@).take(i as int).push(ls@[i as int]@));
            byte_rows(ls@).take(i as int).lemma_filter_len_push(p, ls@[i as int]@);
        }
        if hit {
            n += 1;
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(byte_rows(ls@).take(i as int) == byte_rows(ls@));
    }
    n
}

pub fn meter(path: &[u8], entries: usize, clex_facts: usize, ruled: usize) -> (r: Vec<u8>)
    ensures
        r@ == lexicon_meter(path@, entries as nat, clex_facts as nat, ruled as nat),
{
    let mut r = concat(b"goal: lexicon ok ", path);
    r.push(0x20);
    append(&mut r, &number(entries));
    append(&mut r, b" entries ");
    append(&mut r, &number(clex_facts));
    append(&mut r, b" clex facts ");
    append(&mut r, &number(ruled));
    append(&mut r, b" ruled shadows\n");
    proof {
        reveal_byteslit(b"goal: lexicon ok ");
        reveal_strlit("goal: lexicon ok ");
        reveal_byteslit(b" entries ");
        reveal_strlit(" entries ");
        reveal_byteslit(b" clex facts ");
        reveal_strlit(" clex facts ");
        reveal_byteslit(b" ruled shadows\n");
        reveal_strlit(" ruled shadows");
        reveal(ckc_spec::v1text::ascii);
        assert(r@ == lexicon_meter(path@, entries as nat, clex_facts as nat, ruled as nat));
    }
    r
}

pub fn check(
    path: &[u8],
    ulex: &[u8],
    clex: &[u8],
    ace: &Vec<Vec<u8>>,
    rs: &Vec<(Vec<u8>, Vec<u8>)>,
) -> (r: EVerdict)
    ensures
        r@ == lexicon(path@, ulex@, clex@, byte_rows(ace@), byte_pairs(rs@)),
{
    let ls = lines(ulex, false);
    let cls = lines(clex, true);
    let cns = normals(&cls);
    let norms = set_from(&cns);
    let names = surface_names(clex);
    match audit(&ls, clex, &norms, &names, rs) {
        Some(v) => return v,
        None => {},
    }
    proof {
        reveal(ckc_spec::v1text::ascii);
        reveal_byteslit(b"lexicon-shadow");
        reveal_strlit("lexicon-shadow");
        assert(b"lexicon-shadow"@ == ckc_spec::v1text::ascii("lexicon-shadow"@));
        reveal_byteslit(b"stale ruling matches no live shadow: ");
        reveal_strlit("stale ruling matches no live shadow: ");
        assert(b"stale ruling matches no live shadow: "@ == ckc_spec::v1text::ascii(
            "stale ruling matches no live shadow: "@,
        ));
        reveal_byteslit(b"lexicon-dead-lexeme");
        reveal_strlit("lexicon-dead-lexeme");
        assert(b"lexicon-dead-lexeme"@ == ckc_spec::v1text::ascii("lexicon-dead-lexeme"@));
        reveal_byteslit(b"no ace document references: ");
        reveal_strlit("no ace document references: ");
        assert(b"no ace document references: "@ == ckc_spec::v1text::ascii(
            "no ace document references: "@,
        ));
    }
    match stale(clex, &ls, &names, rs) {
        Some(u) => return failure(
            b"lexicon-shadow",
            concat(b"stale ruling matches no live shadow: ", &u),
        ),
        None => {},
    }
    let tokens = token_rows(ace);
    let token_set = set_from(&tokens);
    match dead(&ls, &token_set) {
        Some(s) => return failure(
            b"lexicon-dead-lexeme",
            concat(b"no ace document references: ", &s),
        ),
        None => {},
    }
    EVerdict::Ok(meter(path, ls.len(), k4_set::len(&norms), ruled(clex, &ls, &names, rs)))
}

} // verus!
