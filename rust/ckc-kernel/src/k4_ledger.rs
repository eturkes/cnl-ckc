use crate::k4_bytes::{append, concat, contains, copy, eq, has, less, range, split, starts_with};
use crate::k4_scalar::{clean, date, docid, hex, n_plus, number};
use ckc_spec::check::*;
use ckc_spec::replay::ESrc;
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub open spec fn previous(p: Option<(Vec<u8>, Vec<u8>)>) -> Option<(Seq<u8>, Seq<u8>)> {
    match p {
        Some((a, b)) => Some((a@, b@)),
        None => None,
    }
}

pub open spec fn decision_result(r: Result<EDecision, Vec<u8>>) -> Result<Decision, Seq<u8>> {
    match r {
        Ok(d) => Ok(d@),
        Err(e) => Err(e@),
    }
}

pub open spec fn decision_list_result(r: Result<Vec<EDecision>, Vec<u8>>) -> Result<
    Seq<Decision>,
    Seq<u8>,
> {
    match r {
        Ok(d) => Ok(decisions(d@)),
        Err(e) => Err(e@),
    }
}

pub fn detail(index: usize, what: &[u8]) -> (r: Vec<u8>)
    ensures
        r@ == ckc_spec::v1text::ascii("ledger row "@) + nat_bytes(index as nat + 2) + seq![0x20u8]
            + what@,
{
    let mut r = copy(b"ledger row ");
    append(&mut r, &n_plus(index, 2));
    r.push(0x20);
    append(&mut r, what);
    proof {
        reveal_byteslit(b"ledger row ");
        reveal_strlit("ledger row ");
        reveal(ckc_spec::v1text::ascii);
    }
    r
}

pub fn row_error(e: Vec<u8>) -> (r: Result<EDecision, Vec<u8>>)
    ensures
        decision_result(r) == Result::Err(e@),
{
    let ghost text = e@;
    let r: Result<EDecision, Vec<u8>> = Err(e);
    proof {
        assert(decision_result(r) == Result::Err(text));
    }
    r
}

pub fn decision(
    line: &[u8],
    index: usize,
    known: &Vec<Vec<u8>>,
    prev: &Option<(Vec<u8>, Vec<u8>)>,
) -> (r: Result<EDecision, Vec<u8>>)
    ensures
        decision_result(r) == parse_decision(
            line@,
            index as nat + 2,
            byte_rows(known@),
            previous(*prev),
        ),
{
    proof {
        reveal(ckc_spec::v1text::ascii);
        reveal(parse_decision);
        reveal(decision_result);
        reveal(row_detail);
        reveal(previous);
        reveal_byteslit(b"ledger header");
        reveal_strlit("ledger header");
        assert(b"ledger header"@ == ckc_spec::v1text::ascii("ledger header"@));
        reveal_byteslit(b"field-count ");
        reveal_strlit("field-count ");
        assert(b"field-count "@ == ckc_spec::v1text::ascii("field-count "@));
        reveal_byteslit(b"docid-grammar");
        reveal_strlit("docid-grammar");
        assert(b"docid-grammar"@ == ckc_spec::v1text::ascii("docid-grammar"@));
        reveal_byteslit(b"unknown-docid ");
        reveal_strlit("unknown-docid ");
        assert(b"unknown-docid "@ == ckc_spec::v1text::ascii("unknown-docid "@));
        reveal_byteslit(b"sort-order ");
        reveal_strlit("sort-order ");
        assert(b"sort-order "@ == ckc_spec::v1text::ascii("sort-order "@));
        reveal_byteslit(b" after ");
        reveal_strlit(" after ");
        assert(b" after "@ == ckc_spec::v1text::ascii(" after "@));
        reveal_byteslit(b"hex");
        reveal_strlit("hex");
        assert(b"hex"@ == ckc_spec::v1text::ascii("hex"@));
        reveal_byteslit(b"ace-commit");
        reveal_strlit("ace-commit");
        assert(b"ace-commit"@ == ckc_spec::v1text::ascii("ace-commit"@));
        reveal_byteslit(b"verdict");
        reveal_strlit("verdict");
        assert(b"verdict"@ == ckc_spec::v1text::ascii("verdict"@));
        reveal_byteslit(b"reviewer");
        reveal_strlit("reviewer");
        assert(b"reviewer"@ == ckc_spec::v1text::ascii("reviewer"@));
        reveal_byteslit(b"date");
        reveal_strlit("date");
        assert(b"date"@ == ckc_spec::v1text::ascii("date"@));
        reveal_byteslit(b"comment");
        reveal_strlit("comment");
        assert(b"comment"@ == ckc_spec::v1text::ascii("comment"@));
        reveal_byteslit(b"approved");
        reveal_strlit("approved");
        assert(b"approved"@ == ckc_spec::v1text::ascii("approved"@));
        reveal_byteslit(b"rejected");
        reveal_strlit("rejected");
        assert(b"rejected"@ == ckc_spec::v1text::ascii("rejected"@));
        reveal_byteslit(b"\t");
        assert(b"\t"@ == seq![0x09u8]);
        reveal(ckc_spec::v1text::ascii);
    }
    if line.len() > 0 && line[0] == 0x23 {
        let e = copy(b"ledger header");
        proof {
            assert(e@ == ckc_spec::v1text::ascii("ledger header"@));
            assert(parse_decision(line@, index as nat + 2, byte_rows(known@), previous(*prev))
                == Result::Err(e@));
        }
        return row_error(e);
    }
    let fields = split(line, 0x09);
    if fields.len() != 7 {
        let mut e = detail(index, b"field-count ");
        append(&mut e, &number(fields.len()));
        proof {
            assert(tab_fields(line@) == byte_rows(fields@));
            assert(e@ == row_detail(index as nat + 2, "field-count "@) + nat_bytes(fields@.len()));
            assert(parse_decision(line@, index as nat + 2, byte_rows(known@), previous(*prev))
                == Result::Err(e@));
        }
        return row_error(e);
    }
    proof {
        assert(tab_fields(line@) == byte_rows(fields@));
    }
    let d = &fields[0];
    let hash = &fields[1];
    let commit = &fields[2];
    let verdict = &fields[3];
    let reviewer = &fields[4];
    let stamp = &fields[5];
    let comment = &fields[6];
    if !docid(d) {
        return row_error(detail(index, b"docid-grammar"));
    }
    if !contains(known, d) {
        let mut e = detail(index, b"unknown-docid ");
        append(&mut e, d);
        return row_error(e);
    }
    match prev {
        Some((pd, pt)) => {
            proof {
                assert(previous(*prev) == Some((pd@, pt@)));
            }
            let mut key = concat(d, b"\t");
            append(&mut key, stamp);
            let mut pkey = concat(pd, b"\t");
            append(&mut pkey, pt);
            if less(&key, &pkey) {
                let mut e = detail(index, b"sort-order ");
                append(&mut e, d);
                e.push(0x20);
                append(&mut e, stamp);
                append(&mut e, b" after ");
                append(&mut e, pd);
                e.push(0x20);
                append(&mut e, pt);
                proof {
                    assert(e@ == row_detail(index as nat + 2, "sort-order "@) + d@ + seq![0x20u8]
                        + stamp@ + ckc_spec::v1text::ascii(" after "@) + pd@ + seq![0x20u8] + pt@);
                    assert(parse_decision(
                        line@,
                        index as nat + 2,
                        byte_rows(known@),
                        previous(*prev),
                    ) == Result::Err(e@));
                }
                return row_error(e);
            }
        },
        None => {},
    }
    proof {
        assert(!(match previous(*prev) {
            Some((pd, pt)) => ckc_spec::engine::bytes_lt(
                d@ + seq![0x09u8] + stamp@,
                pd + seq![0x09u8] + pt,
            ),
            None => false,
        }));
    }
    if !hex(hash, 64) {
        return row_error(detail(index, b"hex"));
    }
    if commit.len() != 0 && !hex(commit, 40) {
        return row_error(detail(index, b"ace-commit"));
    }
    let approved = eq(verdict, b"approved");
    if !approved && !eq(verdict, b"rejected") {
        return row_error(detail(index, b"verdict"));
    }
    if reviewer.len() == 0 || !clean(reviewer) {
        return row_error(detail(index, b"reviewer"));
    }
    if !date(stamp) {
        return row_error(detail(index, b"date"));
    }
    if !clean(comment) {
        return row_error(detail(index, b"comment"));
    }
    Ok(
        EDecision {
            docid: copy(d),
            digest: copy(hash),
            commit: copy(commit),
            approved,
            date: copy(stamp),
        },
    )
}

pub proof fn decisions_push(ds: Seq<EDecision>, d: EDecision)
    ensures
        decisions(ds.push(d)) == decisions(ds).push(d@),
{
    assert_seqs_equal!(decisions(ds.push(d)) == decisions(ds).push(d@));
}

pub fn parse(lines: &Vec<Vec<u8>>, known: &Vec<Vec<u8>>) -> (r: Result<Vec<EDecision>, Vec<u8>>)
    ensures
        decision_list_result(r) == parse_decisions(
            byte_rows(lines@),
            0,
            byte_rows(known@),
            None,
            Seq::empty(),
        ),
{
    let mut out = Vec::new();
    let mut prev = None;
    let mut i = 0usize;
    proof {
        assert(decisions(out@) == Seq::<Decision>::empty());
    }
    while i < lines.len()
        invariant
            i <= lines@.len(),
            parse_decisions(byte_rows(lines@), 0, byte_rows(known@), None, Seq::empty())
                == parse_decisions(
                byte_rows(lines@),
                i as nat,
                byte_rows(known@),
                previous(prev),
                decisions(out@),
            ),
        decreases lines.len() - i,
    {
        let row = decision(&lines[i], i, known, &prev);
        proof {
            reveal_with_fuel(parse_decisions, 2);
        }
        match row {
            Err(e) => return Err(e),
            Ok(d) => {
                let next_prev = Some((copy(&d.docid), copy(&d.date)));
                proof {
                    decisions_push(out@, d);
                }
                out.push(d);
                prev = next_prev;
            },
        }
        i += 1;
    }
    proof {
        reveal_with_fuel(parse_decisions, 2);
    }
    Ok(out)
}

pub fn header() -> (r: Vec<u8>)
    ensures
        r@ == ledger_header(),
{
    proof {
        reveal_byteslit(
            b"# format: docid<TAB>review_sha256<TAB>ace_commit<TAB>verdict<TAB>reviewer<TAB>date<TAB>comment\n",
        );
        reveal_strlit(
            "# format: docid<TAB>review_sha256<TAB>ace_commit<TAB>verdict<TAB>reviewer<TAB>date<TAB>comment\n",
        );
        reveal(ckc_spec::v1text::ascii);
    }
    copy(
        b"# format: docid<TAB>review_sha256<TAB>ace_commit<TAB>verdict<TAB>reviewer<TAB>date<TAB>comment\n",
    )
}

pub fn failure(detail: Vec<u8>) -> (r: EVerdict)
    ensures
        r@ == fail("adjudication"@, detail@),
{
    proof {
        reveal(fail);
        reveal_byteslit(b"adjudication");
        reveal_strlit("adjudication");
        reveal(ckc_spec::v1text::ascii);
    }
    let category = copy(b"adjudication");
    let ghost detail_view = detail@;
    proof {
        assert(category@ == ckc_spec::v1text::ascii("adjudication"@));
    }
    let r = EVerdict::Fail(category, detail);
    proof {
        assert(r@ == Verdict::Fail(ckc_spec::v1text::ascii("adjudication"@), detail_view));
        assert(fail("adjudication"@, detail_view) == Verdict::Fail(
            ckc_spec::v1text::ascii("adjudication"@),
            detail_view,
        ));
    }
    r
}

pub fn ledger_error(e: EVerdict) -> (r: Result<Vec<EDecision>, EVerdict>)
    ensures
        ledger_result(r) == Result::Err(e@),
{
    let ghost verdict = e@;
    let r: Result<Vec<EDecision>, EVerdict> = Err(e);
    proof {
        assert(ledger_result(r) == Result::Err(verdict));
    }
    r
}

pub fn validate(src: &ESrc, known: &Vec<Vec<u8>>) -> (r: Result<Vec<EDecision>, EVerdict>)
    ensures
        ledger_result(r) == ledger(src@, byte_rows(known@)),
{
    proof {
        reveal(ckc_spec::v1text::ascii);
        reveal(ledger);
        reveal(ledger_result);
        reveal(fail);
        reveal_byteslit(b"ledger encoding");
        reveal_strlit("ledger encoding");
        assert(b"ledger encoding"@ == ckc_spec::v1text::ascii("ledger encoding"@));
        reveal_byteslit(b"ledger carriage-return");
        reveal_strlit("ledger carriage-return");
        assert(b"ledger carriage-return"@ == ckc_spec::v1text::ascii("ledger carriage-return"@));
        reveal_byteslit(b"ledger final-newline");
        reveal_strlit("ledger final-newline");
        assert(b"ledger final-newline"@ == ckc_spec::v1text::ascii("ledger final-newline"@));
        reveal_byteslit(b"ledger header");
        reveal_strlit("ledger header");
        assert(b"ledger header"@ == ckc_spec::v1text::ascii("ledger header"@));
        reveal(ckc_spec::v1text::ascii);
    }
    match src {
        ESrc::Missing => {
            let out: Vec<EDecision> = Vec::new();
            proof {
                assert(decisions(out@) == Seq::<Decision>::empty());
                assert(src@ == ckc_spec::replay::Src::Missing);
            }
            Ok(out)
        },
        ESrc::Bad(_) => ledger_error(failure(copy(b"ledger encoding"))),
        ESrc::Bytes(b) => {
            proof {
                assert(src@ == ckc_spec::replay::Src::Bytes(b@));
            }
            if has(b, 0x0d) {
                let e = failure(copy(b"ledger carriage-return"));
                proof {
                    assert(e@ == fail(
                        "adjudication"@,
                        ckc_spec::v1text::ascii("ledger carriage-return"@),
                    ));
                    assert(ledger(src@, byte_rows(known@)) == Result::Err(e@));
                }
                return ledger_error(e);
            }
            if b.len() == 0 || b[b.len() - 1] != 0x0a {
                let e = failure(copy(b"ledger final-newline"));
                proof {
                    assert(e@ == fail(
                        "adjudication"@,
                        ckc_spec::v1text::ascii("ledger final-newline"@),
                    ));
                    assert(ledger(src@, byte_rows(known@)) == Result::Err(e@));
                }
                return ledger_error(e);
            }
            let h = header();
            if !starts_with(b, &h) {
                return ledger_error(failure(copy(b"ledger header")));
            }
            let body = range(b, h.len(), b.len());
            let mut lines = split(&body, 0x0a);
            let ghost before = lines@;
            let last = lines.pop();
            proof {
                assert_seqs_equal!(byte_rows(lines@) == byte_rows(before).drop_last());
                assert_seqs_equal!(body@ == b@.skip(ledger_header().len() as int));
                assert(byte_rows(lines@) == body_lines(b@.skip(ledger_header().len() as int)));
            }
            let parsed = parse(&lines, known);
            proof {
                assert(decision_list_result(parsed) == parse_decisions(
                    body_lines(b@.skip(ledger_header().len() as int)),
                    0,
                    byte_rows(known@),
                    None,
                    Seq::empty(),
                ));
                reveal(ledger);
            }
            match parsed {
                Ok(ds) => Ok(ds),
                Err(e) => ledger_error(failure(e)),
            }
        },
    }
}

} // verus!
