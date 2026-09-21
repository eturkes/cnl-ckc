use crate::k4_bytes::{copy, eq};
use ckc_spec::check::*;
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub open spec fn rows(v: Seq<ECoverageRow>) -> Seq<Row> {
    v.map_values(|r: ECoverageRow| r@)
}

pub open spec fn pays(v: Seq<(Vec<u8>, Vec<Vec<u8>>)>) -> Seq<(Seq<u8>, Seq<Seq<u8>>)> {
    v.map_values(|p: (Vec<u8>, Vec<Vec<u8>>)| (p.0@, byte_rows(p.1@)))
}

pub fn ace_index(rs: &Vec<ECoverageRow>, d: &[u8]) -> (i: usize)
    ensures
        i <= rs@.len(),
        i < rs@.len() ==> first_ace_row(rows(rs@), d@) == Some(rs@[i as int]@),
        i == rs@.len() ==> first_ace_row(rows(rs@), d@) is None,
{
    let mut i = 0usize;
    proof {
        assert_seqs_equal!(rows(rs@).skip(0) == rows(rs@));
    }
    while i < rs.len()
        invariant
            i <= rs@.len(),
            first_ace_row(rows(rs@), d@) == first_ace_row(rows(rs@).skip(i as int), d@),
        decreases rs.len() - i,
    {
        let r = &rs[i];
        let matched = match &r.status {
            EStatus::Ace(id) => eq(id, d),
            _ => false,
        };
        if matched {
            proof {
                assert(ace_docid(r@) == Some(d@));
                reveal_with_fuel(first_ace_row, 2);
            }
            return i;
        }
        proof {
            assert(ace_docid(r@) != Some(d@));
            reveal_with_fuel(first_ace_row, 2);
            assert_seqs_equal!(rows(rs@).skip(i as int).drop_first() == rows(rs@).skip(i as int + 1));
        }
        i += 1;
    }
    proof {
        reveal_with_fuel(first_ace_row, 2);
    }
    i
}

pub fn file_index_exec(files: &Vec<Vec<u8>>, f: &[u8]) -> (i: usize)
    ensures
        i <= files@.len(),
        i as int == index_of(byte_rows(files@), f@),
{
    let mut i = 0usize;
    proof {
        assert_seqs_equal!(byte_rows(files@).skip(0) == byte_rows(files@));
    }
    while i < files.len()
        invariant
            i <= files@.len(),
            index_of(byte_rows(files@), f@) == i as int + index_of(
                byte_rows(files@).skip(i as int),
                f@,
            ),
        decreases files.len() - i,
    {
        let matches = eq(&files[i], f);
        proof {
            reveal_with_fuel(index_of, 2);
            assert_seqs_equal!(byte_rows(files@).skip(i as int).drop_first() == byte_rows(files@).skip(i as int + 1));
        }
        if matches {
            return i;
        }
        i += 1;
    }
    proof {
        reveal_with_fuel(index_of, 2);
    }
    i
}

pub fn ordinal_index(rs: &Vec<ECoverageRow>, f: &[u8], id: &[u8]) -> (n: usize)
    ensures
        n <= rs@.len(),
        n as int == row_index(rows_in(rows(rs@), f@), id@),
{
    let mut i = 0usize;
    let mut n = 0usize;
    proof {
        assert_seqs_equal!(rows(rs@).skip(0) == rows(rs@));
    }
    while i < rs.len()
        invariant
            i <= rs@.len(),
            n <= i,
            row_index(rows_in(rows(rs@), f@), id@) == n as int + row_index(
                rows_in(rows(rs@).skip(i as int), f@),
                id@,
            ),
        decreases rs.len() - i,
    {
        let r = &rs[i];
        let cited = eq(&r.file, f);
        proof {
            let tail = rows(rs@).skip(i as int + 1);
            assert_seqs_equal!(rows(rs@).skip(i as int) == seq![r@] + tail);
            tail.lemma_filter_prepend(r@, |r: Row| r.file == f@);
            assert(rows_in(rows(rs@).skip(i as int), f@) == if cited {
                seq![r@] + rows_in(rows(rs@).skip(i as int + 1), f@)
            } else {
                rows_in(rows(rs@).skip(i as int + 1), f@)
            });
        }
        if cited {
            let found = eq(&r.id, id);
            proof {
                let current = rows_in(rows(rs@).skip(i as int), f@);
                let next = rows_in(rows(rs@).skip(i as int + 1), f@);
                assert(current.len() > 0);
                assert(current[0] == r@);
                assert_seqs_equal!(current.drop_first() == next);
                assert(found == (current[0].id == id@));
                reveal_with_fuel(row_index, 2);
                assert(row_index(rows_in(rows(rs@).skip(i as int), f@), id@) == if found {
                    0int
                } else {
                    1 + row_index(rows_in(rows(rs@).skip(i as int + 1), f@), id@)
                });
            }
            if found {
                return n;
            }
            n += 1;
        }
        i += 1;
    }
    proof {
        assert(rows_in(rows(rs@).skip(i as int), f@).len() == 0);
        reveal_with_fuel(row_index, 2);
    }
    n
}

pub fn locator(ev: &EEvidence, id: &[u8]) -> (r: Option<Vec<u8>>)
    ensures
        optional_bytes(r) == payload_of(ev@, id@),
{
    let mut i = 0usize;
    proof {
        assert_seqs_equal!(pays(ev.payloads@).skip(0) == pays(ev.payloads@));
    }
    while i < ev.payloads.len()
        invariant
            i <= ev.payloads@.len(),
            payload_in(pays(ev.payloads@), id@) == payload_in(
                pays(ev.payloads@).skip(i as int),
                id@,
            ),
        decreases ev.payloads.len() - i,
    {
        let p = &ev.payloads[i];
        if eq(&p.0, id) {
            proof {
                reveal_with_fuel(payload_in, 2);
            }
            if p.1.len() == 0 {
                return None;
            }
            return Some(copy(&p.1[0]));
        }
        proof {
            reveal_with_fuel(payload_in, 2);
            assert_seqs_equal!(pays(ev.payloads@).skip(i as int).drop_first() == pays(ev.payloads@).skip(i as int + 1));
        }
        i += 1;
    }
    proof {
        reveal_with_fuel(payload_in, 2);
    }
    None
}

pub fn select(c: &ECoverage, d: &[u8]) -> (r: (Option<Vec<u8>>, Option<Vec<u8>>))
    ensures
        optional_bytes(r.0) == (match ace_row(c@, d@) {
            Some(row) => Some(row.line),
            None => None,
        }),
        optional_bytes(r.1) == payload(c@, d@),
{
    let i = ace_index(&c.rows, d);
    if i == c.rows.len() {
        return (None, None);
    }
    let r = &c.rows[i];
    let row_line = copy(&r.line);
    let f = file_index_exec(&c.files, &r.file);
    if f >= c.evidence.len() {
        return (Some(row_line), None);
    }
    let ev = &c.evidence[f];
    let payload = if ev.locators.len() > 0 {
        locator(ev, &r.id)
    } else {
        let k = ordinal_index(&c.rows, &r.file, &r.id);
        if k < ev.ordinal.len() {
            Some(copy(&ev.ordinal[k]))
        } else {
            None
        }
    };
    (Some(row_line), payload)
}

} // verus!
