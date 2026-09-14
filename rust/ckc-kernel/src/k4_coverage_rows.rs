#[cfg(verus_keep_ghost)]
use crate::k4_bytes::byte_rows_push;
use crate::k4_bytes::{append, concat, contains, copy, eq};
#[cfg(verus_keep_ghost)]
use crate::k4_payload::rows as row_views;
use crate::k4_rows::{claimed, named};
use crate::k4_scalar::number;
use ckc_spec::check::*;
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub fn lookup(rs: &Vec<ECoverageRow>, id: &[u8]) -> (i: usize)
    ensures
        i <= rs@.len(),
        i < rs@.len() ==> row_by_id(row_views(rs@), id@) == Some(rs@[i as int]@),
        i == rs@.len() ==> row_by_id(row_views(rs@), id@) is None,
{
    let mut i = 0usize;
    proof {
        assert_seqs_equal!(row_views(rs@).skip(0) == row_views(rs@));
    }
    while i < rs.len()
        invariant
            i <= rs@.len(),
            row_by_id(row_views(rs@), id@) == row_by_id(row_views(rs@).skip(i as int), id@),
        decreases rs.len() - i,
    {
        let hit = eq(&rs[i].id, id);
        proof {
            reveal_with_fuel(row_by_id, 2);
            assert_seqs_equal!(row_views(rs@).skip(i as int).drop_first() == row_views(rs@).skip(i as int + 1));
        }
        if hit {
            return i;
        }
        i += 1;
    }
    proof {
        reveal_with_fuel(row_by_id, 2);
    }
    i
}

pub fn restates(rs: &Vec<ECoverageRow>) -> (r: Option<Vec<u8>>)
    ensures
        optional_bytes(r) == restates_check(row_views(rs@), 0),
{
    let mut i = 0usize;
    while i < rs.len()
        invariant
            i <= rs@.len(),
            restates_check(row_views(rs@), 0) == restates_check(row_views(rs@), i as nat),
        decreases rs.len() - i,
    {
        proof {
            reveal(ckc_spec::v1text::ascii);
            reveal_byteslit(b"restates itself: ");
            reveal_strlit("restates itself: ");
            assert(b"restates itself: "@ == ckc_spec::v1text::ascii("restates itself: "@));
            reveal_byteslit(b"restates unknown region for ");
            reveal_strlit("restates unknown region for ");
            assert(b"restates unknown region for "@ == ckc_spec::v1text::ascii(
                "restates unknown region for "@,
            ));
            reveal_byteslit(b"restates a restatement for ");
            reveal_strlit("restates a restatement for ");
            assert(b"restates a restatement for "@ == ckc_spec::v1text::ascii(
                "restates a restatement for "@,
            ));
            reveal_byteslit(b": ");
            reveal_strlit(": ");
            assert(b": "@ == ckc_spec::v1text::ascii(": "@));
            reveal_with_fuel(restates_check, 2);
        }
        let row = &rs[i];
        match &row.status {
            EStatus::Restates(t) => {
                if eq(t, &row.id) {
                    return Some(concat(b"restates itself: ", &row.id));
                }
                let target = lookup(rs, t);
                if target == rs.len() {
                    return Some(named(b"restates unknown region for ", &row.id, b": ", t));
                }
                let restatement = match &rs[target].status {
                    EStatus::Restates(_) => true,
                    _ => false,
                };
                if restatement {
                    return Some(named(b"restates a restatement for ", &row.id, b": ", t));
                }
            },
            _ => {},
        }
        i += 1;
    }
    proof {
        reveal_with_fuel(restates_check, 2);
    }
    None
}

pub fn unclaimed(rs: &Vec<ECoverageRow>, docids: &Vec<Vec<u8>>) -> (r: Option<Vec<u8>>)
    ensures
        optional_bytes(r) == first_unclaimed(row_views(rs@), byte_rows(docids@), 0),
{
    let mut i = 0usize;
    while i < docids.len()
        invariant
            i <= docids@.len(),
            first_unclaimed(row_views(rs@), byte_rows(docids@), 0) == first_unclaimed(
                row_views(rs@),
                byte_rows(docids@),
                i as nat,
            ),
        decreases docids.len() - i,
    {
        let known = claimed(rs, &docids[i]);
        proof {
            reveal_with_fuel(first_unclaimed, 2);
        }
        if !known {
            return Some(copy(&docids[i]));
        }
        i += 1;
    }
    proof {
        reveal_with_fuel(first_unclaimed, 2);
    }
    None
}

pub fn files(rs: &Vec<ECoverageRow>) -> (out: Vec<Vec<u8>>)
    ensures
        byte_rows(out@) == files_of(row_views(rs@), 0, Seq::empty()),
{
    let mut out = Vec::new();
    let mut i = 0usize;
    proof {
        assert(byte_rows(out@) == Seq::<Seq<u8>>::empty());
    }
    while i < rs.len()
        invariant
            i <= rs@.len(),
            files_of(row_views(rs@), 0, Seq::empty()) == files_of(
                row_views(rs@),
                i as nat,
                byte_rows(out@),
            ),
        decreases rs.len() - i,
    {
        let seen = contains(&out, &rs[i].file);
        proof {
            reveal_with_fuel(files_of, 2);
        }
        if !seen {
            let f = copy(&rs[i].file);
            proof {
                byte_rows_push(out@, f);
            }
            out.push(f);
        }
        i += 1;
    }
    proof {
        reveal_with_fuel(files_of, 2);
    }
    out
}

pub fn count(rs: &Vec<ECoverageRow>, k: i64) -> (n: usize)
    ensures
        n as nat == count_status(row_views(rs@), k as int),
{
    let ghost pred = |r: Row|
        match r.status {
            Status::Pending => k == 0,
            Status::Ace(_) => k == 1,
            Status::Restates(_) => k == 2,
            Status::Uncovered => k == 3,
        };
    let mut i = 0usize;
    let mut n = 0usize;
    while i < rs.len()
        invariant
            i <= rs@.len(),
            n <= i,
            n as nat == row_views(rs@).take(i as int).filter(pred).len(),
            forall|r: Row| #[trigger]
                pred(r) == match r.status {
                    Status::Pending => k == 0,
                    Status::Ace(_) => k == 1,
                    Status::Restates(_) => k == 2,
                    Status::Uncovered => k == 3,
                },
        decreases rs.len() - i,
    {
        let hit = match &rs[i].status {
            EStatus::Pending => k == 0,
            EStatus::Ace(_) => k == 1,
            EStatus::Restates(_) => k == 2,
            EStatus::Uncovered => k == 3,
        };
        proof {
            assert(hit == pred(rs@[i as int]@));
            assert_seqs_equal!(row_views(rs@).take(i as int + 1) == row_views(rs@).take(i as int).push(rs@[i as int]@));
            row_views(rs@).take(i as int).lemma_filter_len_push(pred, rs@[i as int]@);
        }
        if hit {
            n += 1;
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(row_views(rs@).take(i as int) == row_views(rs@));
    }
    n
}

pub fn meter(gid: &[u8], c: &ECoverage) -> (r: Vec<u8>)
    ensures
        r@ == coverage_meter(gid@, c@.rows),
{
    let pending = number(count(&c.rows, 0));
    let ace = number(count(&c.rows, 1));
    let restates = number(count(&c.rows, 2));
    let uncovered = number(count(&c.rows, 3));
    let total = number(c.rows.len());
    let mut r = concat(b"goal: coverage ok ", gid);
    r.push(0x20);
    append(&mut r, &total);
    append(&mut r, b" regions; ace=");
    append(&mut r, &ace);
    append(&mut r, b" restates=");
    append(&mut r, &restates);
    append(&mut r, b" uncovered=");
    append(&mut r, &uncovered);
    append(&mut r, b" pending=");
    append(&mut r, &pending);
    r.push(0x0a);
    proof {
        reveal_byteslit(b"goal: coverage ok ");
        reveal_strlit("goal: coverage ok ");
        reveal_byteslit(b" regions; ace=");
        reveal_strlit(" regions; ace=");
        reveal_byteslit(b" restates=");
        reveal_strlit(" restates=");
        reveal_byteslit(b" uncovered=");
        reveal_strlit(" uncovered=");
        reveal_byteslit(b" pending=");
        reveal_strlit(" pending=");
        reveal(ckc_spec::v1text::ascii);
        assert(r@ == coverage_meter(gid@, c@.rows));
    }
    r
}

} // verus!
