use crate::k5_bytes as b;
#[cfg(verus_keep_ghost)]
use ckc_spec::engine::bytes_lt;
use ckc_spec::ui::{self as u, ERecord};
use vstd::prelude::*;
use vstd::slice::slice_to_vec;
verus! {

pub fn less(a: &[u8], b: &[u8]) -> (yes: bool)
    ensures
        yes == bytes_lt(a@, b@),
{
    let mut i = 0;
    proof {
        assert(a@.skip(0) =~= a@);
        assert(b@.skip(0) =~= b@);
    }
    while i < a.len() && i < b.len()
        invariant
            i <= a.len(),
            i <= b.len(),
            bytes_lt(a@, b@) == bytes_lt(a@.skip(i as int), b@.skip(i as int)),
        decreases a.len() - i,
    {
        if a[i] != b[i] {
            return a[i] < b[i];
        }
        proof {
            assert(a@.skip(i as int).drop_first() =~= a@.skip(i as int + 1));
            assert(b@.skip(i as int).drop_first() =~= b@.skip(i as int + 1));
        }
        i += 1;
    }
    i < b.len()
}

pub fn record_line(r: &ERecord) -> (out: Vec<u8>)
    ensures
        out@ == u::record_line(r@),
{
    let mut xs = Vec::new();
    xs.push(slice_to_vec(&r.docid));
    xs.push(slice_to_vec(&r.digest));
    xs.push(slice_to_vec(&r.commit));
    xs.push(
        if r.approved {
            b::literal("approved")
        } else {
            b::literal("rejected")
        },
    );
    xs.push(slice_to_vec(&r.reviewer));
    xs.push(slice_to_vec(&r.date));
    xs.push(slice_to_vec(&r.comment));
    let sep = b::literal("\t");
    proof {
        assert(b::views(xs@) =~= seq![
            r.docid@,
            r.digest@,
            r.commit@,
            if r.approved {
                u::lit("approved"@)
            } else {
                u::lit("rejected"@)
            },
            r.reviewer@,
            r.date@,
            r.comment@,
        ]);
    }
    b::join(&xs, &sep)
}

pub fn position(rows: &Vec<Vec<u8>>, key: &[u8]) -> (pos: usize)
    ensures
        pos <= rows.len(),
        pos == u::insert_position(b::views(rows@), key@, 0, 0),
{
    let mut i = 0;
    let mut last = 0;
    while i < rows.len()
        invariant
            i <= rows.len(),
            last <= i,
            u::insert_position(b::views(rows@), key@, i as nat, last as nat) == u::insert_position(
                b::views(rows@),
                key@,
                0,
                0,
            ),
        decreases rows.len() - i,
    {
        let f = b::split(&rows[i], 9);
        let mut oldkey = b::at(&f, 0);
        let tab = b::literal("\t");
        b::append(&mut oldkey, &tab);
        let date = b::at(&f, 5);
        b::append(&mut oldkey, &date);
        if !less(key, &oldkey) {
            last = i + 1;
        }
        i += 1;
    }
    last
}

pub fn candidate(old: &[u8], r: &ERecord) -> (out: Vec<u8>)
    ensures
        out@ == u::ledger_candidate(old@, r@),
{
    let rows = b::raw_rows(old);
    let mut key = slice_to_vec(&r.docid);
    let tab = b::literal("\t");
    b::append(&mut key, &tab);
    b::append(&mut key, &r.date);
    let pos = position(&rows, &key);
    let record = record_line(r);
    let mut lines = Vec::new();
    let mut i = 0;
    while i < pos
        invariant
            i <= pos <= rows.len(),
            b::views(lines@) == b::views(rows@).take(i as int),
        decreases pos - i,
    {
        lines.push(slice_to_vec(&rows[i]));
        proof {
            assert(b::views(rows@).take(i as int + 1) =~= b::views(rows@).take(i as int).push(
                rows@[i as int]@,
            ));
        }
        i += 1;
    }
    lines.push(record);
    proof {
        assert(b::views(rows@).subrange(pos as int, pos as int) =~= Seq::<Seq<u8>>::empty());
    }
    while i < rows.len()
        invariant
            pos <= i <= rows.len(),
            b::views(lines@) == b::views(rows@).take(pos as int) + seq![u::record_line(r@)]
                + b::views(rows@).subrange(pos as int, i as int),
        decreases rows.len() - i,
    {
        let ghost before = b::views(lines@);
        lines.push(slice_to_vec(&rows[i]));
        proof {
            assert(b::views(lines@) =~= before.push(rows@[i as int]@));
            assert(before.push(rows@[i as int]@) =~= b::views(rows@).take(pos as int) + seq![
                u::record_line(r@),
            ] + b::views(rows@).subrange(pos as int, i as int).push(rows@[i as int]@));
            assert(b::views(rows@).subrange(pos as int, i as int + 1) =~= b::views(rows@).subrange(
                pos as int,
                i as int,
            ).push(rows@[i as int]@));
        }
        i += 1;
    }
    proof {
        assert(b::views(rows@).subrange(pos as int, i as int) =~= b::views(rows@).skip(pos as int));
    }
    let header =
        b"# format: docid<TAB>review_sha256<TAB>ace_commit<TAB>verdict<TAB>reviewer<TAB>date<TAB>comment\n";
    proof {
        reveal_byteslit(
            b"# format: docid<TAB>review_sha256<TAB>ace_commit<TAB>verdict<TAB>reviewer<TAB>date<TAB>comment\n",
        );
        reveal_strlit(
            "# format: docid<TAB>review_sha256<TAB>ace_commit<TAB>verdict<TAB>reviewer<TAB>date<TAB>comment\n",
        );
        assert(header@ =~= ckc_spec::check::ledger_header());
    }
    let mut out = slice_to_vec(header);
    let newline = b::literal("\n");
    let data = b::join(&lines, &newline);
    b::append(&mut out, &data);
    b::append(&mut out, &newline);
    out
}

} // verus!
