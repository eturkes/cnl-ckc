#[cfg(verus_keep_ghost)]
use crate::k4_bytes::byte_rows_push;
use crate::k4_bytes::{append, concat, copy, has, range, split};
use crate::k4_numbers::{equals_count, normalized as decimal_bytes};
#[cfg(verus_keep_ghost)]
use crate::k4_payload::pays as pay_views;
use crate::k4_scalar::{digit, number};
use crate::k4_search::{census, sub};
use ckc_spec::check::*;
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub type Walk = (Vec<Vec<u8>>, Vec<(Vec<u8>, Vec<Vec<u8>>)>, Vec<Vec<u8>>);

pub open spec fn walk_view(w: Walk) -> (Seq<Seq<u8>>, Seq<(Seq<u8>, Seq<Seq<u8>>)>, Seq<Seq<u8>>) {
    (byte_rows(w.0@), pay_views(w.1@), byte_rows(w.2@))
}

pub open spec fn evidence_result(r: Result<EEvidence, Vec<u8>>) -> Result<Evidence, Seq<u8>> {
    match r {
        Ok(e) => Ok(e@),
        Err(e) => Err(e@),
    }
}

pub proof fn pays_push(v: Seq<(Vec<u8>, Vec<Vec<u8>>)>, id: Vec<u8>, lines: Vec<Vec<u8>>)
    ensures
        pay_views(v.push((id, lines))) == pay_views(v).push((id@, byte_rows(lines@))),
{
    assert_seqs_equal!(pay_views(v.push((id, lines))) == pay_views(v).push((id@, byte_rows(lines@))));
}

pub fn locator(line: &[u8]) -> (r: Option<Vec<u8>>)
    ensures
        optional_bytes(r) == locator_id(line@),
{
    proof {
        reveal_byteslit(b" | ");
        reveal_strlit(" | ");
        reveal(ckc_spec::v1text::ascii);
        assert(b" | "@ == ckc_spec::v1text::ascii(" | "@));
    }
    if line.len() < 2 || line[0] != 0x5b || line[line.len() - 1] != 0x5d {
        return None;
    }
    if sub(line, b" | ", 0) >= line.len() {
        return None;
    }
    let body = range(line, 1, line.len());
    let n = sub(&body, b" | ", 0);
    let id = range(&body, 0, n);
    let spaced = has(&id, 0x20);
    if id.len() > 0 && !spaced {
        Some(id)
    } else {
        None
    }
}

pub fn all_digits(s: &[u8]) -> (r: bool)
    ensures
        r == ckc_spec::v1text::all_in(s@, |b: u8| ckc_spec::v1text::is_digit_b(b)),
{
    let mut i = 0usize;
    while i < s.len()
        invariant
            i <= s@.len(),
            forall|j: int| 0 <= j < i ==> ckc_spec::v1text::is_digit_b(s@[j]),
        decreases s.len() - i,
    {
        if !digit(s[i]) {
            return false;
        }
        i += 1;
    }
    true
}

pub fn label(line: &[u8]) -> (r: Vec<u8>)
    ensures
        r@ == strip_label(line@),
{
    proof {
        reveal_byteslit(b". ");
        reveal_strlit(". ");
        reveal(ckc_spec::v1text::ascii);
        assert(b". "@ == ckc_spec::v1text::ascii(". "@));
    }
    let k = sub(line, b". ", 0);
    if k < line.len() && k > 0 {
        let prefix = range(line, 0, k);
        if all_digits(&prefix) {
            return range(line, k + 2, line.len());
        }
    }
    copy(line)
}

pub fn walk_lines(lines: &Vec<Vec<u8>>) -> (r: Walk)
    ensures
        walk_view(r) == walk(
            byte_rows(lines@),
            0,
            None,
            Seq::empty(),
            false,
            Seq::empty(),
            Seq::empty(),
            Seq::empty(),
        ),
{
    let mut cur: Option<Vec<u8>> = None;
    let mut content = Vec::new();
    let mut locs = Vec::new();
    let mut pays = Vec::new();
    let mut ord = Vec::new();
    let mut blank = false;
    let mut i = 0usize;
    proof {
        assert(byte_rows(content@) == Seq::<Seq<u8>>::empty());
        assert(byte_rows(locs@) == Seq::<Seq<u8>>::empty());
        assert(pay_views(pays@) == Seq::<(Seq<u8>, Seq<Seq<u8>>)>::empty());
        assert(byte_rows(ord@) == Seq::<Seq<u8>>::empty());
    }
    while i < lines.len()
        invariant
            i <= lines@.len(),
            walk(
                byte_rows(lines@),
                0,
                None,
                Seq::empty(),
                false,
                Seq::empty(),
                Seq::empty(),
                Seq::empty(),
            ) == walk(
                byte_rows(lines@),
                i as nat,
                optional_bytes(cur),
                byte_rows(content@),
                blank,
                byte_rows(locs@),
                pay_views(pays@),
                byte_rows(ord@),
            ),
        decreases lines.len() - i,
    {
        let ghost before_step = walk(
            byte_rows(lines@),
            i as nat,
            optional_bytes(cur),
            byte_rows(content@),
            blank,
            byte_rows(locs@),
            pay_views(pays@),
            byte_rows(ord@),
        );
        let ghost old_cur = optional_bytes(cur);
        let ghost old_content = byte_rows(content@);
        let ghost old_locs = byte_rows(locs@);
        let ghost old_pays = pay_views(pays@);
        let ghost old_ord = byte_rows(ord@);
        let ghost old_blank = blank;
        let line = &lines[i];
        let loc = locator(line);
        proof {
            reveal_with_fuel(walk, 2);
        }
        match loc {
            Some(id) => {
                let ghost id_view = id@;
                match cur {
                    Some(c) => {
                        proof {
                            pays_push(pays@, c, content);
                        }
                        pays.push((c, content));
                    },
                    None => {},
                }
                let id_copy = copy(&id);
                proof {
                    byte_rows_push(locs@, id_copy);
                }
                locs.push(id_copy);
                cur = Some(id);
                content = Vec::new();
                proof {
                    assert(optional_bytes(cur) == Some(id_view));
                    assert(byte_rows(content@) == Seq::<Seq<u8>>::empty());
                    assert(byte_rows(locs@) == old_locs.push(id_view));
                    assert(pay_views(pays@) == match old_cur {
                        Some(c) => old_pays.push((c, old_content)),
                        None => old_pays,
                    });
                    assert(byte_rows(ord@) == old_ord && blank == old_blank);
                    reveal_with_fuel(walk, 2);
                    assert(before_step == walk(
                        byte_rows(lines@),
                        i as nat + 1,
                        optional_bytes(cur),
                        byte_rows(content@),
                        blank,
                        byte_rows(locs@),
                        pay_views(pays@),
                        byte_rows(ord@),
                    ));
                }
            },
            None => {
                if line.len() == 0 {
                    blank = true;
                } else {
                    if cur.is_some() {
                        let l = copy(line);
                        proof {
                            byte_rows_push(content@, l);
                        }
                        content.push(l);
                    }
                    if blank {
                        let l = copy(line);
                        proof {
                            byte_rows_push(ord@, l);
                        }
                        ord.push(l);
                    }
                }
                proof {
                    assert(optional_bytes(cur) == old_cur);
                    assert(byte_rows(locs@) == old_locs && pay_views(pays@) == old_pays);
                    assert(blank == (old_blank || line@.len() == 0));
                    assert(byte_rows(content@) == if old_cur is Some && line@.len() > 0 {
                        old_content.push(line@)
                    } else {
                        old_content
                    });
                    assert(byte_rows(ord@) == if old_blank && line@.len() > 0 {
                        old_ord.push(line@)
                    } else {
                        old_ord
                    });
                    reveal_with_fuel(walk, 2);
                    assert(before_step == walk(
                        byte_rows(lines@),
                        i as nat + 1,
                        optional_bytes(cur),
                        byte_rows(content@),
                        blank,
                        byte_rows(locs@),
                        pay_views(pays@),
                        byte_rows(ord@),
                    ));
                }
            },
        }
        proof {
            reveal_with_fuel(walk, 2);
            assert(before_step == walk(
                byte_rows(lines@),
                i as nat + 1,
                optional_bytes(cur),
                byte_rows(content@),
                blank,
                byte_rows(locs@),
                pay_views(pays@),
                byte_rows(ord@),
            ));
        }
        i += 1;
    }
    proof {
        reveal_with_fuel(walk, 2);
    }
    match cur {
        Some(c) => {
            proof {
                pays_push(pays@, c, content);
            }
            pays.push((c, content));
        },
        None => {},
    }
    (locs, pays, ord)
}

pub fn first_bad(pays: &Vec<(Vec<u8>, Vec<Vec<u8>>)>) -> (i: usize)
    ensures
        i <= pays@.len(),
        i < pays@.len() ==> first_multi(pay_views(pays@), 0) == Some(
            (pays@[i as int].0@, pays@[i as int].1@.len()),
        ),
        i == pays@.len() ==> first_multi(pay_views(pays@), 0) is None,
{
    let mut i = 0usize;
    while i < pays.len()
        invariant
            i <= pays@.len(),
            first_multi(pay_views(pays@), 0) == first_multi(pay_views(pays@), i as nat),
        decreases pays.len() - i,
    {
        proof {
            reveal_with_fuel(first_multi, 2);
        }
        if pays[i].1.len() != 1 {
            return i;
        }
        i += 1;
    }
    proof {
        reveal_with_fuel(first_multi, 2);
    }
    i
}

pub fn labels(lines: &Vec<Vec<u8>>) -> (out: Vec<Vec<u8>>)
    ensures
        byte_rows(out@) == byte_rows(lines@).map_values(|l: Seq<u8>| strip_label(l)),
{
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < lines.len()
        invariant
            i <= lines@.len(),
            byte_rows(out@) == byte_rows(lines@).take(i as int).map_values(
                |l: Seq<u8>| strip_label(l),
            ),
        decreases lines.len() - i,
    {
        let l = label(&lines[i]);
        proof {
            byte_rows_push(out@, l);
            byte_rows(lines@).lemma_map_take_succ(|l: Seq<u8>| strip_label(l), i as int);
        }
        out.push(l);
        i += 1;
    }
    proof {
        assert_seqs_equal!(byte_rows(lines@).take(i as int) == byte_rows(lines@));
    }
    out
}

pub fn parse(text: &[u8], path: &[u8]) -> (r: Result<EEvidence, Vec<u8>>)
    ensures
        evidence_result(r) == evidence_of(text@, path@),
        r matches Ok(e) ==> e.census@.len() > 0 && ckc_spec::v1text::all_in(
            e.census@,
            |b: u8| ckc_spec::v1text::is_digit_b(b),
        ),
{
    proof {
        reveal(ckc_spec::v1text::ascii);
        reveal_byteslit(b"evidence lacks region-authority census: ");
        reveal_strlit("evidence lacks region-authority census: ");
        assert(b"evidence lacks region-authority census: "@ == ckc_spec::v1text::ascii(
            "evidence lacks region-authority census: "@,
        ));
        reveal_byteslit(b"evidence region ");
        reveal_strlit("evidence region ");
        assert(b"evidence region "@ == ckc_spec::v1text::ascii("evidence region "@));
        reveal_byteslit(b" carries ");
        reveal_strlit(" carries ");
        assert(b" carries "@ == ckc_spec::v1text::ascii(" carries "@));
        reveal_byteslit(b" content lines in: ");
        reveal_strlit(" content lines in: ");
        assert(b" content lines in: "@ == ckc_spec::v1text::ascii(" content lines in: "@));
        reveal_byteslit(b"payload lines ");
        reveal_strlit("payload lines ");
        assert(b"payload lines "@ == ckc_spec::v1text::ascii("payload lines "@));
        reveal_byteslit(b" differ from census ");
        reveal_strlit(" differ from census ");
        assert(b" differ from census "@ == ckc_spec::v1text::ascii(" differ from census "@));
        reveal_byteslit(b" for: ");
        reveal_strlit(" for: ");
        assert(b" for: "@ == ckc_spec::v1text::ascii(" for: "@));
    }
    let digits = match census(text) {
        Some(n) => n,
        None => return Err(concat(b"evidence lacks region-authority census: ", path)),
    };
    let lines = split(text, 0x0a);
    let (locs, pays, ord) = walk_lines(&lines);
    let bad = first_bad(&pays);
    if bad < pays.len() {
        let mut e = concat(b"evidence region ", &pays[bad].0);
        append(&mut e, b" carries ");
        append(&mut e, &number(pays[bad].1.len()));
        append(&mut e, b" content lines in: ");
        append(&mut e, path);
        return Err(e);
    }
    if locs.len() == 0 && !equals_count(&digits, ord.len()) {
        let mut e = concat(b"payload lines ", &number(ord.len()));
        append(&mut e, b" differ from census ");
        append(&mut e, &decimal_bytes(&digits));
        append(&mut e, b" for: ");
        append(&mut e, path);
        return Err(e);
    }
    let ordinal = labels(&ord);
    Ok(EEvidence { census: digits, locators: locs, payloads: pays, ordinal })
}

} // verus!
