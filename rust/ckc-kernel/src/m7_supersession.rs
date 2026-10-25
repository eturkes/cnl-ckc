use crate::k4_bytes::{append, copy, eq, split, starts_with};
use ckc_spec::check::*;
#[cfg(verus_keep_ghost)]
use ckc_spec::v1text::{all_in, ascii, is_digit_b, udec_bytes};
use ckc_spec::vocabulary::*;
use vstd::assert_seqs_equal;
use vstd::prelude::*;
use vstd::slice::slice_subrange;

verus! {

pub struct ESuperseding {
    pub date: usize,
    pub url: Vec<u8>,
    pub citation: Vec<u8>,
}

impl View for ESuperseding {
    type V = Superseding;

    open spec fn view(&self) -> Self::V {
        Superseding { date: self.date as nat, url: self.url@, citation: self.citation@ }
    }
}

pub open spec fn sup_rows(v: Seq<(Vec<u8>, ESuperseding)>) -> Seq<(Seq<u8>, Superseding)> {
    v.map_values(|p: (Vec<u8>, ESuperseding)| (p.0@, p.1@))
}

pub open spec fn sup_opt(v: Option<ESuperseding>) -> Option<Superseding> {
    match v {
        Some(s) => Some(s@),
        None => None,
    }
}

pub fn clone_sup(s: &ESuperseding) -> (out: ESuperseding)
    ensures
        out@ == s@,
{
    ESuperseding { date: s.date, url: copy(&s.url), citation: copy(&s.citation) }
}

pub fn clone_rows(v: &Vec<(Vec<u8>, ESuperseding)>) -> (out: Vec<(Vec<u8>, ESuperseding)>)
    ensures
        sup_rows(out@) == sup_rows(v@),
{
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < v.len()
        invariant
            i <= v.len(),
            out.len() == i,
            sup_rows(out@) == sup_rows(v@).take(i as int),
        decreases v.len() - i,
    {
        let next = (copy(&v[i].0), clone_sup(&v[i].1));
        let ghost before = out@;
        out.push(next);
        proof {
            assert_seqs_equal!(sup_rows(out@) == sup_rows(v@).take(i + 1), j => {
                if j < i { assert(out@[j] == before[j]); }
            });
        }
        i += 1;
    }
    out
}

pub fn key_in(v: &Vec<(Vec<u8>, ESuperseding)>, key: &[u8]) -> (out: bool)
    ensures
        out == sup_keys(sup_rows(v@)).contains(key@),
{
    let mut i = 0usize;
    while i < v.len()
        invariant
            i <= v.len(),
            forall|j: int| 0 <= j < i ==> #[trigger] v@[j].0@ != key@,
        decreases v.len() - i,
    {
        if eq(&v[i].0, key) {
            proof {
                assert(sup_keys(sup_rows(v@))[i as int] == key@);
            }
            return true;
        }
        i += 1;
    }
    proof {
        if sup_keys(sup_rows(v@)).contains(key@) {
            let j = choose|j: int|
                0 <= j < sup_keys(sup_rows(v@)).len() && sup_keys(sup_rows(v@))[j] == key@;
            assert(v@[j].0@ != key@);
        }
    }
    false
}

pub proof fn literals()
    ensures
        b"supersession"@ == ascii("supersession"@),
        b"superseded"@ == ascii("superseded"@),
        b"supersession"@ != b"superseded"@,
        b"supersession document"@ == ascii("supersession document"@),
        b"supersession value"@ == ascii("supersession value"@),
        b"duplicate supersession"@ == ascii("duplicate supersession"@),
        b"duplicate superseded"@ == ascii("duplicate superseded"@),
        b"supersession document at row "@ == ascii("supersession document at row "@),
        b"superseded region at row "@ == ascii("superseded region at row "@),
        b"ckc: supersession ok "@ == ascii("ckc: supersession ok "@),
        b" documents "@ == ascii(" documents "@),
        b" passages\n"@ == ascii(" passages\n"@),
        b": "@ == ascii(": "@),
        b" "@ == ascii(" "@),
{
    reveal(ascii);
    reveal_byteslit(b"supersession");
    reveal_strlit("supersession");
    assert(b"supersession"@ =~= ascii("supersession"@));
    reveal_byteslit(b"superseded");
    reveal_strlit("superseded");
    assert(b"superseded"@ =~= ascii("superseded"@));
    reveal_byteslit(b"supersession document");
    reveal_strlit("supersession document");
    assert(b"supersession document"@ =~= ascii("supersession document"@));
    reveal_byteslit(b"supersession value");
    reveal_strlit("supersession value");
    assert(b"supersession value"@ =~= ascii("supersession value"@));
    reveal_byteslit(b"duplicate supersession");
    reveal_strlit("duplicate supersession");
    assert(b"duplicate supersession"@ =~= ascii("duplicate supersession"@));
    reveal_byteslit(b"duplicate superseded");
    reveal_strlit("duplicate superseded");
    assert(b"duplicate superseded"@ =~= ascii("duplicate superseded"@));
    reveal_byteslit(b"supersession document at row ");
    reveal_strlit("supersession document at row ");
    assert(b"supersession document at row "@ =~= ascii("supersession document at row "@));
    reveal_byteslit(b"superseded region at row ");
    reveal_strlit("superseded region at row ");
    assert(b"superseded region at row "@ =~= ascii("superseded region at row "@));
    reveal_byteslit(b"ckc: supersession ok ");
    reveal_strlit("ckc: supersession ok ");
    assert(b"ckc: supersession ok "@ =~= ascii("ckc: supersession ok "@));
    reveal_byteslit(b" documents ");
    reveal_strlit(" documents ");
    assert(b" documents "@ =~= ascii(" documents "@));
    reveal_byteslit(b" passages\n");
    reveal_strlit(" passages\n");
    assert(b" passages\n"@ =~= ascii(" passages\n"@));
    reveal_byteslit(b": ");
    reveal_strlit(": ");
    assert(b": "@ =~= ascii(": "@));
    reveal_byteslit(b" ");
    reveal_strlit(" ");
    assert(b" "@ =~= ascii(" "@));
}

fn digits(s: &[u8]) -> (out: bool)
    ensures
        out == all_in(s@, |b: u8| is_digit_b(b)),
{
    let mut i = 0usize;
    while i < s.len()
        invariant
            i <= s.len(),
            forall|j: int| 0 <= j < i ==> is_digit_b(#[trigger] s@[j]),
        decreases s.len() - i,
    {
        if s[i] < 0x30 || s[i] > 0x39 {
            return false;
        }
        i += 1;
    }
    true
}

pub fn digits_value_exec(s: &[u8]) -> (out: usize)
    requires
        s.len() <= 8,
        all_in(s@, |b: u8| is_digit_b(b)),
    ensures
        out as nat == digits_value(s@),
        out <= 99999999,
{
    let mut out = 0usize;
    let mut i = 0usize;
    proof {
        assert(s@.take(0) == Seq::<u8>::empty());
    }
    while i < s.len()
        invariant
            i <= s.len() <= 8,
            all_in(s@, |b: u8| is_digit_b(b)),
            out as nat == digits_value(s@.take(i as int)),
            i == 0 ==> out == 0,
            i == 1 ==> out <= 9,
            i == 2 ==> out <= 99,
            i == 3 ==> out <= 999,
            i == 4 ==> out <= 9999,
            i == 5 ==> out <= 99999,
            i == 6 ==> out <= 999999,
            i == 7 ==> out <= 9999999,
            i == 8 ==> out <= 99999999,
        decreases s.len() - i,
    {
        let d = s[i] - 0x30;
        proof {
            assert(s@.take(i + 1).drop_last() == s@.take(i as int));
            assert(s@.take(i + 1).last() == s@[i as int]);
            reveal_with_fuel(digits_value, 1);
        }
        out = out * 10 + d as usize;
        i += 1;
    }
    proof {
        assert(s@.take(s@.len() as int) =~= s@);
    }
    out
}

fn month_length_exec(y: usize, m: usize) -> (out: usize)
    ensures
        out as nat == month_length(y as nat, m as nat),
{
    if m == 2 {
        if y.is_multiple_of(4) && !y.is_multiple_of(100) || y.is_multiple_of(400) {
            29
        } else {
            28
        }
    } else if m == 4 || m == 6 || m == 9 || m == 11 {
        30
    } else {
        31
    }
}

pub fn date_ok_exec(d: &[u8]) -> (out: bool)
    ensures
        out == ckc_spec::vocabulary::date_ok(d@),
{
    if d.len() != 8 || !digits(d) {
        return false;
    }
    let ys = slice_subrange(d, 0, 4);
    let ms = slice_subrange(d, 4, 6);
    let ds = slice_subrange(d, 6, 8);
    proof {
        assert(all_in(ys@, |b: u8| is_digit_b(b)));
        assert(all_in(ms@, |b: u8| is_digit_b(b)));
        assert(all_in(ds@, |b: u8| is_digit_b(b)));
    }
    let y = digits_value_exec(ys);
    let m = digits_value_exec(ms);
    let day = digits_value_exec(ds);
    1 <= m && m <= 12 && 1 <= day && day <= month_length_exec(y, m)
}

pub fn url_ok_exec(u: &[u8]) -> (out: bool)
    ensures
        out == url_ok(u@),
{
    proof {
        reveal_byteslit(b"https://");
        reveal_strlit("https://");
        reveal(ascii);
        assert(b"https://"@ =~= ascii("https://"@));
    }
    if u.len() <= 8 || !starts_with(u, b"https://") {
        return false;
    }
    let mut i = 0usize;
    while i < u.len()
        invariant
            i <= u.len(),
            forall|j: int| 0 <= j < i ==> 0x21 <= #[trigger] u@[j] <= 0x7e,
        decreases u.len() - i,
    {
        if u[i] < 0x21 || u[i] > 0x7e {
            return false;
        }
        i += 1;
    }
    true
}

pub fn citation_ok_exec(c: &[u8]) -> (out: bool)
    ensures
        out == citation_ok(c@),
{
    if c.len() == 0 || c[0] == 0x20 || c[c.len() - 1] == 0x20 {
        return false;
    }
    let mut i = 0usize;
    while i < c.len()
        invariant
            i <= c.len(),
            forall|j: int| 0 <= j < i ==> #[trigger] c@[j] >= 0x20 && c@[j] != 0x7f,
        decreases c.len() - i,
    {
        if c[i] < 0x20 || c[i] == 0x7f {
            return false;
        }
        i += 1;
    }
    crate::k5_utf8::valid(c)
}

pub fn superseding_exec(v: &[u8]) -> (out: Option<ESuperseding>)
    ensures
        sup_opt(out) == superseding(v@),
{
    if v.len() < 9 || v[8] != 0x20 {
        return None;
    }
    let rest = slice_subrange(v, 9, v.len());
    let n = crate::k2_manifest::first_byte_exec(rest, 0x20, 0);
    if n >= rest.len() {
        return None;
    }
    let date = slice_subrange(v, 0, 8);
    let url = slice_subrange(rest, 0, n);
    let citation = slice_subrange(rest, n + 1, rest.len());
    if !date_ok_exec(date) || !url_ok_exec(url) || !citation_ok_exec(citation) {
        return None;
    }
    Some(ESuperseding { date: digits_value_exec(date), url: copy(url), citation: copy(citation) })
}

fn coverage_has(c: &ECoverage, key: &[u8], document: bool) -> (out: bool)
    ensures
        out == (if document {
            exists|i: int|
                0 <= i < c@.rows.len() && #[trigger] c@.rows[i].status == Status::Ace(key@)
        } else {
            exists|i: int|
                0 <= i < c@.rows.len() && #[trigger] c@.rows[i].id == key@ && !(
                c@.rows[i].status is Ace)
        }),
{
    if document {
        let found = crate::k4_rows::claimed(&c.rows, key);
        proof {
            assert(crate::k4_payload::rows(c.rows@) =~= c@.rows);
            if found {
                let j = choose|j: int|
                    0 <= j < c@.rows.len() && ace_docid(c@.rows[j]) == Some(key@);
                assert(c@.rows[j].status == Status::Ace(key@));
            } else if exists|j: int|
                0 <= j < c@.rows.len() && #[trigger] c@.rows[j].status == Status::Ace(key@) {
                let j = choose|j: int|
                    0 <= j < c@.rows.len() && c@.rows[j].status == Status::Ace(key@);
                assert(ace_docid(c@.rows[j]) == Some(key@));
            }
        }
        return found;
    }
    let rs = &c.rows;
    let mut i = 0usize;
    while i < rs.len()
        invariant
            !document,
            rs == &c.rows,
            i <= rs.len(),
            forall|j: int| 0 <= j < i ==> (#[trigger] rs@[j]).id@ != key@ || rs@[j].status@ is Ace,
        decreases rs.len() - i,
    {
        if eq(&rs[i].id, key) && !matches!(&rs[i].status, EStatus::Ace(_)) {
            proof {
                assert(c@.rows[i as int] == rs@[i as int]@);
                assert(c@.rows[i as int].id == key@ && !(c@.rows[i as int].status is Ace));
            }
            return true;
        }
        i += 1;
    }
    proof {
        assert forall|j: int| 0 <= j < c@.rows.len() implies #[trigger] c@.rows[j].id != key@
            || c@.rows[j].status is Ace by {
            assert(c@.rows[j] == rs@[j]@);
        }
    }
    false
}

fn sup_violation_exec(lines: &Vec<Vec<u8>>, n: usize, c: &ECoverage) -> (out: Option<Vec<u8>>)
    requires
        n < lines.len(),
    ensures
        (match out {
            Some(why) => Some(why@),
            None => None,
        }) == sup_violation(byte_rows(lines@).take(n as int), c@.rows, 1),
{
    hide(sup_violation);
    let ghost rows = byte_rows(lines@).take(n as int);
    let mut i = 0usize;
    proof {
        assert(rows.skip(0) == rows);
    }
    while i < n
        invariant
            i <= n < lines.len(),
            rows == byte_rows(lines@).take(n as int),
            sup_violation(rows, c@.rows, 1) == sup_violation(
                rows.skip(i as int),
                c@.rows,
                (i + 1) as nat,
            ),
        decreases n - i,
    {
        let fields = split(&lines[i], 0x09);
        proof {
            literals();
            assert(byte_rows(fields@).len() == fields.len());
            assert(rows.skip(i as int)[0] == lines@[i as int]@);
            assert(rows.skip(i as int).drop_first() == rows.skip(i + 1));
            reveal_with_fuel(sup_violation, 1);
        }
        if fields.len() == 3 {
            proof {
                assert(fields@[0]@ == split_on(lines@[i as int]@, 0x09)[0]);
                assert(fields@[1]@ == split_on(lines@[i as int]@, 0x09)[1]);
            }
            let doc = eq(&fields[0], b"supersession");
            let region = eq(&fields[0], b"superseded");
            if (doc || region) && !coverage_has(c, &fields[1], doc) {
                let mut why = copy(
                    if doc {
                        b"supersession document at row "
                    } else {
                        b"superseded region at row "
                    },
                );
                append(&mut why, &crate::k2_manifest::udec_vec(i + 1));
                return Some(why);
            }
        }
        i += 1;
    }
    proof {
        reveal_with_fuel(sup_violation, 1);
    }
    None
}

pub fn check(path: &[u8], bytes: &[u8], c: &ECoverage) -> (out: EVerdict)
    ensures
        out@ == supersession_check(path@, bytes@, c@.rows),
{
    proof {
        literals();
    }
    let (tab, _) = match crate::m7_temporal::parse_vocabulary_exec(bytes) {
        Err(_) => return EVerdict::Ok(Vec::new()),
        Ok(t) => t,
    };
    if crate::m7_temporal::guideline_of_exec(path).is_none() || tab.supersessions.len() == 0
        && tab.superseded.len() == 0 {
        return EVerdict::Ok(Vec::new());
    }
    let h: &[u8] = b"# format: kind\tlemma\tvalue\n";
    proof {
        reveal_byteslit(b"# format: kind\tlemma\tvalue\n");
        reveal_strlit("# format: kind\tlemma\tvalue\n");
        reveal(ascii);
        assert(h@ =~= vocabulary_header());
    }
    let body = slice_subrange(bytes, h.len(), bytes.len());
    let lines = split(body, 0x0a);
    let n = lines.len() - 1;
    proof {
        assert(byte_rows(lines@).take(n as int) == body_lines(body@));
    }
    if let Some(why) = sup_violation_exec(&lines, n, c) {
        let mut detail = copy(path);
        append(&mut detail, b": ");
        append(&mut detail, &why);
        return EVerdict::Fail(copy(b"supersession"), detail);
    }
    let mut meter = copy(b"ckc: supersession ok ");
    append(&mut meter, path);
    append(&mut meter, b" ");
    append(&mut meter, &crate::k2_manifest::udec_vec(tab.supersessions.len()));
    append(&mut meter, b" documents ");
    append(&mut meter, &crate::k2_manifest::udec_vec(tab.superseded.len()));
    append(&mut meter, b" passages\n");
    EVerdict::Ok(meter)
}

} // verus!
