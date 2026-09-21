#[cfg(verus_keep_ghost)]
use crate::k4_bytes::byte_rows_push;
use crate::k4_bytes::range;
use ckc_spec::check::*;
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub fn width(s: &[u8], i: usize) -> (n: usize)
    requires
        i <= s@.len(),
    ensures
        n as nat == ws_len(s@, i as nat),
        n <= s@.len() - i,
{
    if i == s.len() {
        return 0;
    }
    let a = s[i];
    let rem = s.len() - i;
    if (a >= 0x09 && a <= 0x0d) || (a >= 0x1c && a <= 0x20) {
        1
    } else if rem >= 2 && a == 0xc2 && (s[i + 1] == 0x85 || s[i + 1] == 0xa0) {
        2
    } else if rem >= 3 && a == 0xe1 && s[i + 1] == 0x9a && s[i + 2] == 0x80 {
        3
    } else if rem >= 3 && a == 0xe2 && s[i + 1] == 0x80 && ((s[i + 2] >= 0x80 && s[i + 2] <= 0x8a)
        || s[i + 2] == 0xa8 || s[i + 2] == 0xa9 || s[i + 2] == 0xaf) {
        3
    } else if rem >= 3 && a == 0xe2 && s[i + 1] == 0x81 && s[i + 2] == 0x9f {
        3
    } else if rem >= 3 && a == 0xe3 && s[i + 1] == 0x80 && s[i + 2] == 0x80 {
        3
    } else {
        0
    }
}

pub fn first_nonspace(s: &[u8]) -> (i: usize)
    ensures
        i as nat == lead_ws(s@, 0),
        i <= s@.len(),
{
    let mut i = 0usize;
    while i < s.len()
        invariant
            i <= s@.len(),
            lead_ws(s@, 0) == lead_ws(s@, i as nat),
        decreases s.len() - i,
    {
        let n = width(s, i);
        proof {
            reveal_with_fuel(lead_ws, 2);
        }
        if n == 0 {
            return i;
        }
        i += n;
    }
    proof {
        reveal_with_fuel(lead_ws, 2);
    }
    i
}

pub fn end_nonspace(s: &[u8]) -> (end: usize)
    ensures
        end as nat == last_end(s@, 0, 0),
        end <= s@.len(),
{
    let mut i = 0usize;
    let mut end = 0usize;
    while i < s.len()
        invariant
            end <= i <= s@.len(),
            last_end(s@, 0, 0) == last_end(s@, i as nat, end as nat),
        decreases s.len() - i,
    {
        let n = width(s, i);
        proof {
            reveal_with_fuel(last_end, 2);
        }
        if n > 0 {
            i += n;
        } else {
            i += 1;
            end = i;
        }
    }
    proof {
        reveal_with_fuel(last_end, 2);
    }
    end
}

pub fn trim(s: &[u8]) -> (r: Vec<u8>)
    ensures
        r@ == strip_ws(s@),
{
    let start = first_nonspace(s);
    let end = end_nonspace(s);
    if start < end {
        range(s, start, end)
    } else {
        Vec::new()
    }
}

pub fn tokens(s: &[u8]) -> (out: Vec<Vec<u8>>)
    ensures
        byte_rows(out@) == ws_split(s@),
{
    let mut i = 0usize;
    let mut cur = Vec::new();
    let mut out = Vec::new();
    proof {
        assert(byte_rows(out@) == Seq::<Seq<u8>>::empty());
    }
    while i < s.len()
        invariant
            i <= s@.len(),
            ws_split(s@) == tokens_acc(s@, i as nat, cur@, byte_rows(out@)),
        decreases s.len() - i,
    {
        let n = width(s, i);
        proof {
            reveal_with_fuel(tokens_acc, 2);
        }
        if n > 0 {
            if cur.len() > 0 {
                proof {
                    byte_rows_push(out@, cur);
                }
                out.push(cur);
            }
            cur = Vec::new();
            i += n;
        } else {
            cur.push(s[i]);
            i += 1;
        }
    }
    proof {
        reveal_with_fuel(tokens_acc, 2);
    }
    if cur.len() > 0 {
        proof {
            byte_rows_push(out@, cur);
        }
        out.push(cur);
    }
    out
}

pub fn remove_separators(s: &[u8]) -> (r: Vec<u8>)
    ensures
        r@ == s@.filter(|b: u8| b != 0x27 && b != 0x20 && b != 0x09),
{
    let mut i = 0usize;
    let mut r = Vec::new();
    let ghost pred = |b: u8| b != 0x27 && b != 0x20 && b != 0x09;
    while i < s.len()
        invariant
            i <= s@.len(),
            r@ == s@.take(i as int).filter(pred),
            forall|b: u8| #[trigger] pred(b) == (b != 0x27 && b != 0x20 && b != 0x09),
        decreases s.len() - i,
    {
        let b = s[i];
        proof {
            assert_seqs_equal!(s@.take(i as int + 1) == s@.take(i as int).push(b));
            s@.take(i as int).lemma_filter_push(b, pred);
        }
        if b != 0x27 && b != 0x20 && b != 0x09 {
            r.push(b);
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(s@.take(i as int) == s@);
    }
    r
}

pub fn normal(s: &[u8]) -> (r: Vec<u8>)
    ensures
        r@ == normalized(s@),
{
    let mut t = trim(s);
    if t.len() > 0 && t[t.len() - 1] == 0x2e {
        let last = t.pop();
    }
    remove_separators(&t)
}

} // verus!
