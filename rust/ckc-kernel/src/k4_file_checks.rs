#[cfg(verus_keep_ghost)]
use crate::k4_bytes::byte_rows_push;
use crate::k4_bytes::{append, concat, contains, copy, eq, split};
use crate::k4_evidence;
use crate::k4_numbers::{equals_count, normalized as decimal_bytes};
#[cfg(verus_keep_ghost)]
use crate::k4_payload::rows as row_views;
#[cfg(verus_keep_ghost)]
use crate::k4_rows::push_row;
use crate::k4_scalar::number;
use ckc_spec::check::*;
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub open spec fn sources(s: Seq<EFileSrc>) -> Seq<FileSrc> {
    s.map_values(|s: EFileSrc| s@)
}

pub open spec fn evidences(v: Seq<EEvidence>) -> Seq<Evidence> {
    v.map_values(|e: EEvidence| e@)
}

pub open spec fn result_view(v: Result<Vec<EEvidence>, Vec<u8>>) -> Result<Seq<Evidence>, Seq<u8>> {
    match v {
        Ok(es) => Ok(evidences(es@)),
        Err(e) => Err(e@),
    }
}

pub open spec fn prefix(acc: Seq<Evidence>, tail: Result<Seq<Evidence>, Seq<u8>>) -> Result<
    Seq<Evidence>,
    Seq<u8>,
> {
    match tail {
        Ok(es) => Ok(acc + es),
        Err(e) => Err(e),
    }
}

pub proof fn prefix_step(acc: Seq<Evidence>, e: Evidence, tail: Result<Seq<Evidence>, Seq<u8>>)
    ensures
        prefix(
            acc,
            match tail {
                Ok(es) => Ok(seq![e] + es),
                Err(x) => Err(x),
            },
        ) == prefix(acc.push(e), tail),
{
    match tail {
        Ok(es) => {
            assert_seqs_equal!(acc + (seq![e] + es) == acc.push(e) + es);
        },
        Err(_) => {},
    }
}

pub proof fn evidence_push(v: Seq<EEvidence>, e: EEvidence)
    ensures
        evidences(v.push(e)) == evidences(v).push(e@),
{
    assert_seqs_equal!(evidences(v.push(e)) == evidences(v).push(e@));
}

pub fn clone_source(s: &EFileSrc) -> (r: EFileSrc)
    ensures
        r@ == s@,
{
    match s {
        EFileSrc::Symlink => EFileSrc::Symlink,
        EFileSrc::Missing => EFileSrc::Missing,
        EFileSrc::Unreadable => EFileSrc::Unreadable,
        EFileSrc::Bad(i) => EFileSrc::Bad(*i),
        EFileSrc::Bytes(b) => EFileSrc::Bytes(copy(b)),
    }
}

pub fn source(table: &Vec<(Vec<u8>, EFileSrc)>, key: &[u8]) -> (r: EFileSrc)
    ensures
        r@ == source_lookup(source_pairs(table@), key@, 0),
{
    let mut i = 0usize;
    while i < table.len()
        invariant
            i <= table@.len(),
            source_lookup(source_pairs(table@), key@, 0) == source_lookup(
                source_pairs(table@),
                key@,
                i as nat,
            ),
        decreases table.len() - i,
    {
        let hit = eq(&table[i].0, key);
        proof {
            reveal_with_fuel(source_lookup, 2);
        }
        if hit {
            return clone_source(&table[i].1);
        }
        i += 1;
    }
    proof {
        reveal_with_fuel(source_lookup, 2);
    }
    EFileSrc::Missing
}

pub fn read_sources(files: &Vec<Vec<u8>>, table: &Vec<(Vec<u8>, EFileSrc)>) -> (out: Vec<EFileSrc>)
    ensures
        out@.len() == files@.len(),
        sources(out@) == byte_rows(files@).map_values(
            |f: Seq<u8>| source_lookup(source_pairs(table@), f, 0),
        ),
{
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < files.len()
        invariant
            i <= files@.len(),
            out@.len() == i,
            sources(out@) == byte_rows(files@).take(i as int).map_values(
                |f: Seq<u8>| source_lookup(source_pairs(table@), f, 0),
            ),
        decreases files.len() - i,
    {
        let s = source(table, &files[i]);
        proof {
            assert_seqs_equal!(sources(out@.push(s)) == sources(out@).push(s@));
            byte_rows(files@).lemma_map_take_succ(
                |f: Seq<u8>| source_lookup(source_pairs(table@), f, 0),
                i as int,
            );
        }
        out.push(s);
        i += 1;
    }
    proof {
        assert_seqs_equal!(byte_rows(files@).take(i as int) == byte_rows(files@));
    }
    out
}

pub proof fn slash_push(s: Seq<Seq<u8>>, x: Seq<u8>)
    ensures
        join_slash(s.push(x)) == if s.len() == 0 {
            x
        } else {
            join_slash(s) + seq![0x2fu8] + x
        },
    decreases s.len(),
{
    reveal_with_fuel(join_slash, 2);
    if s.len() > 0 {
        assert_seqs_equal!(s.push(x).drop_first() == s.drop_first().push(x));
        if s.len() > 1 {
            slash_push(s.drop_first(), x);
        }
    }
}

pub fn path(root: &[u8], file: &[u8]) -> (r: Vec<u8>)
    ensures
        r@ == full_path(root@, file@),
{
    let parts = split(file, 0x2f);
    let mut kept = Vec::new();
    let mut i = 0usize;
    let mut joined = Vec::new();
    let ghost pred = |s: Seq<u8>| s.len() > 0 && s != ckc_spec::v1text::ascii("."@);
    while i < parts.len()
        invariant
            i <= parts@.len(),
            byte_rows(kept@) == byte_rows(parts@).take(i as int).filter(pred),
            joined@ == join_slash(byte_rows(kept@)),
            forall|s: Seq<u8>| #[trigger]
                pred(s) == (s.len() > 0 && s != ckc_spec::v1text::ascii("."@)),
        decreases parts.len() - i,
    {
        let p = &parts[i];
        let keep = p.len() > 0 && !eq(p, b".");
        proof {
            reveal_byteslit(b".");
            reveal_strlit(".");
            reveal(ckc_spec::v1text::ascii);
            assert(b"."@ == ckc_spec::v1text::ascii("."@));
            assert(keep == pred(p@));
            assert_seqs_equal!(byte_rows(parts@).take(i as int + 1) == byte_rows(parts@).take(i as int).push(p@));
            byte_rows(parts@).take(i as int).lemma_filter_push(p@, pred);
        }
        if keep {
            let next = copy(p);
            proof {
                slash_push(byte_rows(kept@), p@);
                byte_rows_push(kept@, next);
            }
            if kept.len() > 0 {
                joined.push(0x2f);
            }
            append(&mut joined, p);
            kept.push(next);
        }
        i += 1;
    }
    let mut r = copy(root);
    r.push(0x2f);
    append(&mut r, &joined);
    proof {
        assert_seqs_equal!(byte_rows(parts@).take(i as int) == byte_rows(parts@));
    }
    r
}

pub fn clone_status(s: &EStatus) -> (r: EStatus)
    ensures
        r@ == s@,
{
    match s {
        EStatus::Pending => EStatus::Pending,
        EStatus::Uncovered => EStatus::Uncovered,
        EStatus::Ace(d) => EStatus::Ace(copy(d)),
        EStatus::Restates(d) => EStatus::Restates(copy(d)),
    }
}

pub fn clone_row(r: &ECoverageRow) -> (out: ECoverageRow)
    ensures
        out@ == r@,
{
    ECoverageRow {
        id: copy(&r.id),
        file: copy(&r.file),
        status: clone_status(&r.status),
        line: copy(&r.line),
    }
}

pub fn cited(rs: &Vec<ECoverageRow>, file: &[u8]) -> (out: Vec<ECoverageRow>)
    ensures
        row_views(out@) == rows_in(row_views(rs@), file@),
{
    let ghost pred = |r: Row| r.file == file@;
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < rs.len()
        invariant
            i <= rs@.len(),
            row_views(out@) == row_views(rs@).take(i as int).filter(pred),
            forall|r: Row| #[trigger] pred(r) == (r.file == file@),
        decreases rs.len() - i,
    {
        let keep = eq(&rs[i].file, file);
        proof {
            assert(keep == pred(rs@[i as int]@));
            assert_seqs_equal!(row_views(rs@).take(i as int + 1) == row_views(rs@).take(i as int).push(rs@[i as int]@));
            row_views(rs@).take(i as int).lemma_filter_push(rs@[i as int]@, pred);
        }
        if keep {
            let r = clone_row(&rs[i]);
            proof {
                push_row(out@, r);
            }
            out.push(r);
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(row_views(rs@).take(i as int) == row_views(rs@));
    }
    out
}

pub fn prefix_has(rows: &Vec<Vec<u8>>, end: usize, key: &[u8]) -> (r: bool)
    requires
        end <= rows@.len(),
    ensures
        r == byte_rows(rows@).take(end as int).contains(key@),
{
    let mut i = 0usize;
    while i < end
        invariant
            i <= end <= rows@.len(),
            forall|j: int| 0 <= j < i ==> rows@[j]@ != key@,
        decreases end - i,
    {
        if eq(&rows[i], key) {
            proof {
                assert(byte_rows(rows@).take(end as int)[i as int] == key@);
            }
            return true;
        }
        i += 1;
    }
    false
}

pub fn duplicate(locs: &Vec<Vec<u8>>) -> (r: Option<Vec<u8>>)
    ensures
        optional_bytes(r) == dup_locator(byte_rows(locs@), 0),
{
    let mut i = 0usize;
    while i < locs.len()
        invariant
            i <= locs@.len(),
            dup_locator(byte_rows(locs@), 0) == dup_locator(byte_rows(locs@), i as nat),
        decreases locs.len() - i,
    {
        let dup = prefix_has(locs, i, &locs[i]);
        proof {
            reveal_with_fuel(dup_locator, 2);
        }
        if dup {
            return Some(copy(&locs[i]));
        }
        i += 1;
    }
    proof {
        reveal_with_fuel(dup_locator, 2);
    }
    None
}

pub fn unanchored(claimed: &Vec<ECoverageRow>, locs: &Vec<Vec<u8>>) -> (r: Option<Vec<u8>>)
    ensures
        optional_bytes(r) == first_unanchored(row_views(claimed@), byte_rows(locs@), 0),
{
    let mut i = 0usize;
    while i < claimed.len()
        invariant
            i <= claimed@.len(),
            first_unanchored(row_views(claimed@), byte_rows(locs@), 0) == first_unanchored(
                row_views(claimed@),
                byte_rows(locs@),
                i as nat,
            ),
        decreases claimed.len() - i,
    {
        let present = contains(locs, &claimed[i].id);
        proof {
            reveal_with_fuel(first_unanchored, 2);
        }
        if !present {
            return Some(copy(&claimed[i].id));
        }
        i += 1;
    }
    proof {
        reveal_with_fuel(first_unanchored, 2);
    }
    None
}

pub fn check(
    rs: &Vec<ECoverageRow>,
    files: &Vec<Vec<u8>>,
    texts: &Vec<EFileSrc>,
    root: &[u8],
) -> (r: Result<Vec<EEvidence>, Vec<u8>>)
    requires
        texts@.len() == files@.len(),
    ensures
        result_view(r) == files_check(row_views(rs@), byte_rows(files@), sources(texts@), root@, 0),
{
    let mut out = Vec::new();
    let mut i = 0usize;
    proof {
        assert(evidences(out@) == Seq::<Evidence>::empty());
    }
    while i < files.len()
        invariant
            i <= files@.len(),
            texts@.len() == files@.len(),
            files_check(row_views(rs@), byte_rows(files@), sources(texts@), root@, 0) == prefix(
                evidences(out@),
                files_check(row_views(rs@), byte_rows(files@), sources(texts@), root@, i as nat),
            ),
        decreases files.len() - i,
    {
        let f = &files[i];
        let full = path(root, f);
        proof {
            reveal(ckc_spec::v1text::ascii);
            reveal_byteslit(b"is a symlink: ");
            reveal_strlit("is a symlink: ");
            assert(b"is a symlink: "@ == ckc_spec::v1text::ascii("is a symlink: "@));
            reveal_byteslit(b"missing: ");
            reveal_strlit("missing: ");
            assert(b"missing: "@ == ckc_spec::v1text::ascii("missing: "@));
            reveal_byteslit(b"unreadable ");
            reveal_strlit("unreadable ");
            assert(b"unreadable "@ == ckc_spec::v1text::ascii("unreadable "@));
            reveal_byteslit(b"invalid_utf8 ");
            reveal_strlit("invalid_utf8 ");
            assert(b"invalid_utf8 "@ == ckc_spec::v1text::ascii("invalid_utf8 "@));
            reveal_byteslit(b"rows ");
            reveal_strlit("rows ");
            assert(b"rows "@ == ckc_spec::v1text::ascii("rows "@));
            reveal_byteslit(b"locators ");
            reveal_strlit("locators ");
            assert(b"locators "@ == ckc_spec::v1text::ascii("locators "@));
            reveal_byteslit(b" differ from census ");
            reveal_strlit(" differ from census ");
            assert(b" differ from census "@ == ckc_spec::v1text::ascii(" differ from census "@));
            reveal_byteslit(b" for: ");
            reveal_strlit(" for: ");
            assert(b" for: "@ == ckc_spec::v1text::ascii(" for: "@));
            reveal_byteslit(b"duplicate evidence locator: ");
            reveal_strlit("duplicate evidence locator: ");
            assert(b"duplicate evidence locator: "@ == ckc_spec::v1text::ascii(
                "duplicate evidence locator: "@,
            ));
            reveal_byteslit(b"coverage row without evidence region: ");
            reveal_strlit("coverage row without evidence region: ");
            assert(b"coverage row without evidence region: "@ == ckc_spec::v1text::ascii(
                "coverage row without evidence region: "@,
            ));
            reveal_with_fuel(files_check, 2);
        }
        let text = match &texts[i] {
            EFileSrc::Symlink => return Err(concat(b"is a symlink: ", &full)),
            EFileSrc::Missing => return Err(concat(b"missing: ", &full)),
            EFileSrc::Unreadable => return Err(concat(b"unreadable ", &full)),
            EFileSrc::Bad(_) => return Err(concat(b"invalid_utf8 ", &full)),
            EFileSrc::Bytes(b) => b,
        };
        let ev = match k4_evidence::parse(text, &full) {
            Ok(e) => e,
            Err(e) => return Err(e),
        };
        let claimed = cited(rs, f);
        if !equals_count(&ev.census, claimed.len()) {
            let mut e = concat(b"rows ", &number(claimed.len()));
            append(&mut e, b" differ from census ");
            append(&mut e, &decimal_bytes(&ev.census));
            append(&mut e, b" for: ");
            append(&mut e, f);
            return Err(e);
        }
        if ev.locators.len() > 0 {
            if !equals_count(&ev.census, ev.locators.len()) {
                let mut e = concat(b"locators ", &number(ev.locators.len()));
                append(&mut e, b" differ from census ");
                append(&mut e, &decimal_bytes(&ev.census));
                append(&mut e, b" for: ");
                append(&mut e, f);
                return Err(e);
            }
            match duplicate(&ev.locators) {
                Some(id) => return Err(concat(b"duplicate evidence locator: ", &id)),
                None => {},
            }
            match unanchored(&claimed, &ev.locators) {
                Some(id) => return Err(concat(b"coverage row without evidence region: ", &id)),
                None => {},
            }
        }
        proof {
            prefix_step(
                evidences(out@),
                ev@,
                files_check(
                    row_views(rs@),
                    byte_rows(files@),
                    sources(texts@),
                    root@,
                    i as nat + 1,
                ),
            );
            evidence_push(out@, ev);
        }
        out.push(ev);
        i += 1;
    }
    proof {
        reveal_with_fuel(files_check, 2);
    }
    Ok(out)
}

} // verus!
