use ckc_spec::replay::*;
use ckc_spec::term::Term;
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

proof fn subrange_push<A>(s: Seq<A>, start: int, end: int)
    requires
        0 <= start <= end < s.len(),
    ensures
        s.subrange(start, end + 1) == s.subrange(start, end).push(s[end]),
{
    assert_seqs_equal!(s.subrange(start, end + 1) == s.subrange(start, end).push(s[end]));
}

fn copy_range(bytes: &[u8], start: usize, end: usize) -> (out: Vec<u8>)
    requires
        start <= end <= bytes@.len(),
    ensures
        out@ == bytes@.subrange(start as int, end as int),
{
    let mut out = Vec::new();
    let mut i = start;
    while i < end
        invariant
            start <= i <= end <= bytes@.len(),
            out@ == bytes@.subrange(start as int, i as int),
        decreases end - i,
    {
        out.push(bytes[i]);
        proof {
            subrange_push(bytes@, start as int, i as int);
        }
        i += 1;
    }
    out
}

pub(crate) fn udec_vec(n: usize) -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::udec_bytes(n as nat),
    decreases n,
{
    if n < 10 {
        let mut out = Vec::new();
        out.push(0x30 + n as u8);
        proof {
            reveal_with_fuel(ckc_spec::v1text::udec_bytes, 2);
            reveal(ckc_spec::v1text::digit_byte);
        }
        out
    } else {
        let q = n / 10;
        let mut out = udec_vec(q);
        out.push(0x30 + (n % 10) as u8);
        proof {
            assert(q < n);
            reveal_with_fuel(ckc_spec::v1text::udec_bytes, 2);
            reveal(ckc_spec::v1text::digit_byte);
            assert(n as nat / 10 == q as nat);
            assert(n as nat % 10 == (n % 10) as nat);
        }
        out
    }
}

proof fn first_byte_exact(bytes: Seq<u8>, b: u8, start: nat, end: nat)
    requires
        start <= end <= bytes.len(),
        forall|i: int| start <= i < end ==> bytes[i] != b,
        end == bytes.len() || bytes[end as int] == b,
    ensures
        first_byte(bytes, b, start) == end,
    decreases end - start,
{
    reveal_with_fuel(first_byte, 2);
    if start < end {
        assert(bytes[start as int] != b);
        first_byte_exact(bytes, b, start + 1, end);
    }
}

fn first_byte_exec(bytes: &[u8], b: u8, start: usize) -> (end: usize)
    requires
        start <= bytes@.len(),
    ensures
        start <= end <= bytes@.len(),
        end as nat == first_byte(bytes@, b, start as nat),
        forall|i: int| start <= i < end ==> bytes@[i] != b,
        end < bytes@.len() ==> bytes@[end as int] == b,
{
    let mut end = start;
    while end < bytes.len()
        invariant
            start <= end <= bytes@.len(),
            forall|i: int| start <= i < end ==> bytes@[i] != b,
        decreases bytes.len() - end,
    {
        if bytes[end] == b {
            proof {
                first_byte_exact(bytes@, b, start as nat, end as nat);
            }
            return end;
        }
        end += 1;
    }
    proof {
        first_byte_exact(bytes@, b, start as nat, end as nat);
    }
    end
}

pub open spec fn option_row_view(r: Option<ERow>) -> Option<MRow> {
    match r {
        Option::Some(e) => Option::Some(e@),
        Option::None => Option::None,
    }
}

fn entry_exec(line: &[u8]) -> (out: Option<ERow>)
    ensures
        option_row_view(out) == entry(line@),
{
    let tab = first_byte_exec(line, 0x09, 0);
    if tab == 0 || tab >= line.len() {
        proof {
            reveal(entry);
        }
        return None;
    }
    if tab + 1 >= line.len() {
        proof {
            reveal(entry);
        }
        return None;
    }
    let second = first_byte_exec(line, 0x09, tab + 1);
    if second < line.len() {
        proof {
            reveal(entry);
        }
        return None;
    }
    let pl = copy_range(line, 0, tab);
    let payload = copy_range(line, tab + 1, line.len());
    let row = ERow { pl, payload };
    proof {
        reveal(entry);
        reveal(option_row_view);
    }
    Some(row)
}

pub open spec fn row_views(rows: Seq<ERow>) -> Seq<MRow> {
    rows.map_values(|r: ERow| r@)
}

proof fn row_views_push(rows: Seq<ERow>, row: ERow)
    ensures
        row_views(rows.push(row)) == row_views(rows).push(row@),
{
    reveal(row_views);
    assert_seqs_equal!(rows.map_values(|r: ERow| r@).push(row@)
        == rows.push(row).map_values(|r: ERow| r@));
}

pub open spec fn prefix_out(prefix: Seq<MRow>, tail: MOut) -> MOut {
    match tail {
        MOut::Rows(rows) => MOut::Rows(prefix + rows),
        MOut::MissingLf => MOut::MissingLf,
        MOut::BadLine(line) => MOut::BadLine(line),
    }
}

proof fn prefix_assoc(a: Seq<MRow>, b: Seq<MRow>, tail: MOut)
    ensures
        prefix_out(a, prefix_out(b, tail)) == prefix_out(a + b, tail),
{
    reveal(prefix_out);
    match tail {
        MOut::Rows(rows) => {
            assert_seqs_equal!(a + (b + rows) == (a + b) + rows);
        },
        _ => {},
    }
}

proof fn rows_cons(line: Seq<u8>, rest: Seq<Seq<u8>>)
    ensures
        rows_of(seq![line] + rest) == match entry(line) {
            Option::None => MOut::BadLine(line),
            Option::Some(row) => prefix_out(seq![row], rows_of(rest)),
        },
{
    let lines = seq![line] + rest;
    assert(lines.len() > 0);
    assert(lines[0] == line);
    assert_seqs_equal!(lines.drop_first() == rest);
    reveal_with_fuel(rows_of, 2);
    reveal(prefix_out);
    match entry(line) {
        Option::None => {},
        Option::Some(row) => match rows_of(rest) {
            MOut::Rows(rows) => {
                assert_seqs_equal!(seq![row] + rows == seq![row] + rows);
            },
            _ => {},
        },
    }
}

proof fn suffix_line(bytes: Seq<u8>, start: nat, end: nat)
    requires
        start <= end < bytes.len(),
        bytes[end as int] == 0x0a,
        forall|i: int| start <= i < end ==> bytes[i] != 0x0a,
    ensures
        lines_of(bytes.skip(start as int)) == seq![bytes.subrange(start as int, end as int)]
            + lines_of(bytes.skip((end + 1) as int)),
{
    let suffix = bytes.skip(start as int);
    let n: nat = (end - start) as nat;
    assert forall|i: int| 0 <= i < n implies suffix[i] != 0x0a by {
        if 0 <= i < n {
            assert(suffix[i] == bytes[start as int + i]);
        }
    }
    assert(suffix[n as int] == bytes[end as int]);
    first_byte_exact(suffix, 0x0a, 0, n);
    assert_seqs_equal!(suffix.take(n as int) == bytes.subrange(start as int, end as int));
    assert_seqs_equal!(suffix.skip(n as int + 1) == bytes.skip((end + 1) as int));
    reveal_with_fuel(lines_of, 2);
}

pub enum EManifestBody {
    Rows(Vec<ERow>),
    BadLine(Vec<u8>),
}

pub open spec fn body_view(body: &EManifestBody) -> MOut {
    match body {
        EManifestBody::Rows(rows) => MOut::Rows(row_views(rows@)),
        EManifestBody::BadLine(line) => MOut::BadLine(line@),
    }
}

#[verifier::rlimit(40)]
fn parse_rows_exec(bytes: &[u8]) -> (out: EManifestBody)
    requires
        bytes@.len() > 0,
        bytes@[bytes@.len() - 1] == 0x0a,
    ensures
        body_view(&out) == rows_of(lines_of(bytes@)),
{
    let mut rows: Vec<ERow> = Vec::new();
    let mut start = 0usize;
    proof {
        reveal(row_views);
        reveal(prefix_out);
        assert_seqs_equal!(bytes@.skip(0) == bytes@);
    }
    while start < bytes.len()
        invariant
            start <= bytes@.len(),
            bytes@.len() > 0,
            bytes@[bytes@.len() - 1] == 0x0a,
            rows_of(lines_of(bytes@)) == prefix_out(
                row_views(rows@),
                rows_of(lines_of(bytes@.skip(start as int))),
            ),
        decreases bytes.len() - start,
    {
        let end = first_byte_exec(bytes, 0x0a, start);
        if end == bytes.len() {
            proof {
                let last: int = bytes@.len() as int - 1;
                assert(start < bytes@.len());
                assert(start as int <= last < end as int);
                assert(bytes@[last] == 0x0a);
                assert(bytes@[last] != 0x0a);
                assert(false);
            }
            return EManifestBody::BadLine(Vec::new());
        }
        let line = copy_range(bytes, start, end);
        let next = end + 1;
        proof {
            suffix_line(bytes@, start as nat, end as nat);
            rows_cons(line@, lines_of(bytes@.skip(next as int)));
        }
        match entry_exec(line.as_slice()) {
            None => {
                proof {
                    assert(entry(line@) is None);
                    assert(rows_of(lines_of(bytes@.skip(start as int))) == MOut::BadLine(line@));
                    reveal(prefix_out);
                    reveal(body_view);
                }
                return EManifestBody::BadLine(line);
            },
            Some(row) => {
                let ghost old_rows = rows@;
                let ghost row_view = row@;
                proof {
                    assert(entry(line@) == Option::Some(row_view));
                    assert(rows_of(lines_of(bytes@.skip(start as int))) == prefix_out(
                        seq![row_view],
                        rows_of(lines_of(bytes@.skip(next as int))),
                    ));
                    prefix_assoc(
                        row_views(old_rows),
                        seq![row_view],
                        rows_of(lines_of(bytes@.skip(next as int))),
                    );
                    row_views_push(old_rows, row);
                }
                rows.push(row);
                proof {
                    assert_seqs_equal!(row_views(old_rows).push(row_view)
                        == row_views(old_rows) + seq![row_view]);
                }
                start = next;
            },
        }
    }
    proof {
        assert(start == bytes@.len());
        assert_seqs_equal!(bytes@.skip(start as int) == Seq::<u8>::empty());
        reveal_with_fuel(lines_of, 2);
        reveal_with_fuel(rows_of, 2);
        reveal(prefix_out);
        reveal(body_view);
    }
    EManifestBody::Rows(rows)
}

fn utf8_out(off: usize) -> (out: EOut)
    ensures
        out@ == utf8_reject(off as nat),
{
    crate::k2_reject::utf8_out(off)
}

fn manifest_error_out(mpath: &[u8], detail_bytes: &[u8], bad_line: bool) -> (out: EOut)
    ensures
        out@ == if bad_line {
            manifest_reject(
                mpath@,
                Term::Comp(ckc_spec::v1text::ascii("line"@), seq![Term::Atom(detail_bytes@)]),
            )
        } else {
            manifest_reject(mpath@, atom("missing_final_newline"@))
        },
{
    crate::k2_reject::manifest_error_out(mpath, detail_bytes, bad_line)
}

fn unreadable_out() -> (out: EOut)
    ensures
        out@ == check_load(atom("unreadable"@)),
{
    crate::k2_reject::unreadable_out()
}

pub fn v1_manifest_impl(mpath: &[u8], m: &ESrc) -> (r: Result<Vec<ERow>, EOut>)
    ensures
        rows_view(r) == manifest_rows(mpath@, m@),
{
    match m {
        ESrc::Missing => {
            let out = unreadable_out();
            proof {
                reveal(rows_view);
                reveal(manifest_rows);
            }
            Result::Err(out)
        },
        ESrc::Bad(off) => {
            let out = utf8_out(*off);
            proof {
                reveal(rows_view);
                reveal(manifest_rows);
            }
            Result::Err(out)
        },
        ESrc::Bytes(bytes) => {
            if bytes.len() == 0 {
                let empty: Vec<ERow> = Vec::new();
                proof {
                    assert_seqs_equal!(bytes@ == Seq::<u8>::empty());
                    assert(m@ == Src::Bytes(bytes@));
                    reveal(rows_view);
                    reveal(manifest_rows);
                    reveal(parse_manifest);
                    assert_seqs_equal!(empty@.map_values(|e: ERow| e@) == Seq::<MRow>::empty());
                }
                return Result::Ok(empty);
            }
            if bytes[bytes.len() - 1] != 0x0a {
                let out = manifest_error_out(mpath, bytes.as_slice(), false);
                proof {
                    reveal(rows_view);
                    reveal(manifest_rows);
                    reveal(parse_manifest);
                }
                return Result::Err(out);
            }
            let body = parse_rows_exec(bytes.as_slice());
            match body {
                EManifestBody::Rows(rows) => {
                    proof {
                        reveal(body_view);
                        reveal(rows_view);
                        reveal(manifest_rows);
                        reveal(parse_manifest);
                    }
                    Result::Ok(rows)
                },
                EManifestBody::BadLine(line) => {
                    let out = manifest_error_out(mpath, line.as_slice(), true);
                    proof {
                        reveal(body_view);
                        reveal(rows_view);
                        reveal(manifest_rows);
                        reveal(parse_manifest);
                    }
                    Result::Err(out)
                },
            }
        },
    }
}

} // verus!
