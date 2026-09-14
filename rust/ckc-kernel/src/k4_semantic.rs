use crate::k4_bytes::{append, concat, copy, ends_with, split, starts_with};
use crate::k4_scalar::number;
use ckc_spec::check::*;
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub fn comment(l: &[u8]) -> (r: bool)
    ensures
        r == starts(l@, ckc_spec::v1text::ascii("%"@)),
{
    let r = starts_with(l, b"%");
    proof {
        reveal_byteslit(b"%");
        reveal_strlit("%");
        reveal(ckc_spec::v1text::ascii);
        assert(b"%"@ == ckc_spec::v1text::ascii("%"@));
        assert(r == starts(l@, ckc_spec::v1text::ascii("%"@)));
    }
    r
}

pub fn directive(l: &[u8]) -> (r: bool)
    ensures
        r == starts(l@, ckc_spec::v1text::ascii(":- "@)),
{
    let r = starts_with(l, b":- ");
    proof {
        reveal_byteslit(b":- ");
        reveal_strlit(":- ");
        reveal(ckc_spec::v1text::ascii);
        assert(b":- "@ == ckc_spec::v1text::ascii(":- "@));
        assert(r == starts(l@, ckc_spec::v1text::ascii(":- "@)));
    }
    r
}

pub fn record(l: &[u8]) -> (r: bool)
    ensures
        r == starts(l@, ckc_spec::v1text::ascii("guideline_document("@)),
{
    let r = starts_with(l, b"guideline_document(");
    proof {
        reveal_byteslit(b"guideline_document(");
        reveal_strlit("guideline_document(");
        reveal(ckc_spec::v1text::ascii);
        assert(b"guideline_document("@ == ckc_spec::v1text::ascii("guideline_document("@));
        assert(r == starts(l@, ckc_spec::v1text::ascii("guideline_document("@)));
    }
    r
}

pub fn dotted(l: &[u8]) -> (r: bool)
    ensures
        r == ends(l@, seq![0x2eu8]),
{
    let r = ends_with(l, b".");
    proof {
        reveal_byteslit(b".");
        assert(b"."@ == seq![0x2eu8]);
        assert(r == ends(l@, seq![0x2eu8]));
    }
    r
}

pub fn first_bad(lines: &Vec<Vec<u8>>) -> (r: Option<Vec<u8>>)
    ensures
        optional_bytes(r) == first_undotted(byte_rows(lines@)),
{
    let mut i = 0usize;
    proof {
        assert_seqs_equal!(byte_rows(lines@).skip(0) == byte_rows(lines@));
    }
    while i < lines.len()
        invariant
            i <= lines@.len(),
            first_undotted(byte_rows(lines@)) == first_undotted(byte_rows(lines@).skip(i as int)),
        decreases lines.len() - i,
    {
        let l = &lines[i];
        let good = comment(l) || directive(l) || dotted(l);
        proof {
            reveal_with_fuel(first_undotted, 2);
            assert_seqs_equal!(byte_rows(lines@).skip(i as int).drop_first() == byte_rows(lines@).skip(i as int + 1));
        }
        if !good {
            return Some(copy(l));
        }
        i += 1;
    }
    proof {
        reveal_with_fuel(first_undotted, 2);
    }
    None
}

pub fn records(lines: &Vec<Vec<u8>>) -> (n: usize)
    ensures
        n as nat == record_count(byte_rows(lines@)),
{
    let mut i = 0usize;
    let mut n = 0usize;
    let ghost pred = |l: Seq<u8>| starts(l, ckc_spec::v1text::ascii("guideline_document("@));
    while i < lines.len()
        invariant
            i <= lines@.len(),
            n <= i,
            n as nat == byte_rows(lines@).take(i as int).filter(pred).len(),
            forall|l: Seq<u8>| #[trigger]
                pred(l) == starts(l, ckc_spec::v1text::ascii("guideline_document("@)),
        decreases lines.len() - i,
    {
        let matched = record(&lines[i]);
        proof {
            assert(matched == pred(lines@[i as int]@));
            assert_seqs_equal!(byte_rows(lines@).take(i as int + 1) == byte_rows(lines@).take(i as int).push(lines@[i as int]@));
            byte_rows(lines@).take(i as int).lemma_filter_len_push(pred, lines@[i as int]@);
        }
        if matched {
            n += 1;
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(byte_rows(lines@).take(i as int) == byte_rows(lines@));
    }
    n
}

pub fn stream(lines: &Vec<Vec<u8>>) -> (r: Vec<u8>)
    ensures
        r@ == retained(byte_rows(lines@)),
{
    let mut r = Vec::new();
    let mut i = 0usize;
    proof {
        assert_seqs_equal!(byte_rows(lines@).skip(0) == byte_rows(lines@));
    }
    while i < lines.len()
        invariant
            i <= lines@.len(),
            retained(byte_rows(lines@)) == r@ + retained(byte_rows(lines@).skip(i as int)),
        decreases lines.len() - i,
    {
        let l = &lines[i];
        let omitted = record(l) || comment(l) || directive(l);
        proof {
            reveal_with_fuel(retained, 2);
            assert_seqs_equal!(byte_rows(lines@).skip(i as int).drop_first() == byte_rows(lines@).skip(i as int + 1));
        }
        if !omitted {
            append(&mut r, l);
            r.push(0x0a);
        }
        i += 1;
    }
    proof {
        reveal_with_fuel(retained, 2);
    }
    r
}

pub fn error(e: Vec<u8>) -> (r: Result<Vec<u8>, Vec<u8>>)
    ensures
        bytes_result(r) == Result::Err(e@),
{
    Err(e)
}

pub fn semantic(pl: &[u8], docid: &[u8]) -> (r: Result<Vec<u8>, Vec<u8>>)
    ensures
        bytes_result(r) == semantic_input(pl@, docid@),
{
    proof {
        reveal(ckc_spec::v1text::ascii);
        reveal_byteslit(b"compiled document lacks final newline: ");
        reveal_strlit("compiled document lacks final newline: ");
        assert(b"compiled document lacks final newline: "@ == ckc_spec::v1text::ascii(
            "compiled document lacks final newline: "@,
        ));
        reveal_byteslit(b"noncanonical document record in: ");
        reveal_strlit("noncanonical document record in: ");
        assert(b"noncanonical document record in: "@ == ckc_spec::v1text::ascii(
            "noncanonical document record in: "@,
        ));
        reveal_byteslit(b"noncanonical clause line in: ");
        reveal_strlit("noncanonical clause line in: ");
        assert(b"noncanonical clause line in: "@ == ckc_spec::v1text::ascii(
            "noncanonical clause line in: "@,
        ));
        reveal_byteslit(b"document record count ");
        reveal_strlit("document record count ");
        assert(b"document record count "@ == ckc_spec::v1text::ascii("document record count "@));
        reveal_byteslit(b" for: ");
        reveal_strlit(" for: ");
        assert(b" for: "@ == ckc_spec::v1text::ascii(" for: "@));
    }
    if pl.len() == 0 || pl[pl.len() - 1] != 0x0a {
        return error(concat(b"compiled document lacks final newline: ", docid));
    }
    let mut lines = split(pl, 0x0a);
    let ghost before = lines@;
    let last = lines.pop();
    proof {
        assert_seqs_equal!(byte_rows(lines@) == byte_rows(before).drop_last());
    }
    match first_bad(&lines) {
        Some(l) => {
            if record(&l) {
                return error(concat(b"noncanonical document record in: ", docid));
            } else {
                return error(concat(b"noncanonical clause line in: ", docid));
            }
        },
        None => {},
    }
    let count = records(&lines);
    if count != 1 {
        let mut e = copy(b"document record count ");
        append(&mut e, &number(count));
        append(&mut e, b" for: ");
        append(&mut e, docid);
        return error(e);
    }
    Ok(stream(&lines))
}

} // verus!
