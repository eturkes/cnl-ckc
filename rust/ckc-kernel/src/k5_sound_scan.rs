use crate::{k5_sound_codes as n, k5_sound_source as source};
use ckc_spec::{check as ck, ui as u};
use vstd::prelude::*;
use vstd::utf8::*;
verus! {

broadcast use {vstd::seq::group_seq_axioms, vstd::seq_lib::group_seq_properties};

pub open spec fn byte_codes(b: u::Bytes) -> Seq<nat> {
    b.map_values(|x: u8| x as nat)
}

pub open spec fn pattern_codes(ps: Seq<u::Bytes>) -> Seq<Seq<nat>> {
    ps.map_values(|p: u::Bytes| byte_codes(p))
}

pub proof fn ascii_decode(bytes: u::Bytes)
    requires
        forall|i: int| 0 <= i < bytes.len() ==> #[trigger] bytes[i] < 128,
    ensures
        valid_utf8(bytes),
        n::codes(u::chars(bytes)) == byte_codes(bytes),
{
    let cs = bytes.map_values(|b: u8| b as char);
    assert forall|i: int| 0 <= i < cs.len() implies '\x00' <= #[trigger] cs[i] <= '\x7f' by {
        assert(bytes[i] < 128);
    }
    is_ascii_chars_encode_utf8(cs);
    assert(encode_utf8(cs) =~= bytes);
    encode_utf8_valid_utf8(cs);
    encode_utf8_decode_utf8(cs);
    assert(n::codes(u::chars(bytes)) =~= byte_codes(bytes));
}

pub proof fn ascii_patterns(ps: Seq<u::Bytes>)
    requires
        forall|i: int, j: int|
            0 <= i < ps.len() && 0 <= j < ps[i].len() ==> #[trigger] ps[i][j] < 128,
    ensures
        n::patterns(ps) == pattern_codes(ps),
{
    assert forall|i: int| 0 <= i < ps.len() implies n::codes(u::chars(#[trigger] ps[i]))
        == byte_codes(ps[i]) by {
        ascii_decode(ps[i]);
    }
    assert(n::patterns(ps) =~= pattern_codes(ps));
}

pub proof fn split_ascii(bytes: u::Bytes, delim: u8)
    requires
        forall|i: int| 0 <= i < bytes.len() ==> #[trigger] bytes[i] < 128,
    ensures
        forall|i: int, j: int|
            0 <= i < ck::split_on(bytes, delim).len() && 0 <= j < ck::split_on(
                bytes,
                delim,
            )[i].len() ==> #[trigger] ck::split_on(bytes, delim)[i][j] < 128,
{
    source::split(bytes, delim);
    assert forall|i: int, j: int|
        0 <= i < ck::split_on(bytes, delim).len() && 0 <= j < ck::split_on(
            bytes,
            delim,
        )[i].len() implies #[trigger] ck::split_on(bytes, delim)[i][j] < 128 by {
        let part = ck::split_on(bytes, delim)[i];
        let a = choose|a: int| #[trigger] source::span_at(part, bytes, a);
        assert(part[j] == bytes[a + j]);
    }
}

pub proof fn ascii_input(cs: Seq<char>)
    requires
        is_ascii_chars(cs),
    ensures
        n::codes(cs) == byte_codes(u::lit(cs)),
{
    is_ascii_chars_encode_utf8(cs);
    assert(n::codes(cs) =~= byte_codes(u::lit(cs)));
}

pub proof fn scan_empty(s: Seq<nat>, p: Seq<nat>, w: bool, a: nat)
    ensures
        n::scan(s, p, w, a, a),
{
}

pub proof fn scan_complete(s: Seq<nat>, p: Seq<nat>, w: bool, a: nat, z: nat)
    requires
        forall|k: int| a <= k < z ==> !#[trigger] n::matches(s, p, k, w),
    ensures
        n::scan(s, p, w, a, z),
    decreases z - a,
{
    hide(n::matches);
    if a + 1 < z {
        let m = (a + z) / 2;
        scan_complete(s, p, w, a, m);
        scan_complete(s, p, w, m, z);
    }
}

pub proof fn scan_join(s: Seq<nat>, p: Seq<nat>, w: bool, a: nat, m: nat, z: nat)
    requires
        a <= m <= z,
        n::scan(s, p, w, a, m),
        n::scan(s, p, w, m, z),
    ensures
        n::scan(s, p, w, a, z),
{
    hide(n::scan);
    hide(n::matches);
    assert forall|k: int| a <= k < z implies !#[trigger] n::matches(s, p, k, w) by {
        if k < m {
            n::scan_at(s, p, w, a, m, k);
        } else {
            n::scan_at(s, p, w, m, z, k);
        }
    }
    scan_complete(s, p, w, a, z);
}

pub proof fn same_window(s: Seq<nat>, p: Seq<nat>, left: int, right: int, k: int, j: nat)
    requires
        0 <= left <= k,
        k + p.len() <= right <= s.len(),
        j <= p.len(),
    ensures
        n::same(s, p, k, j) == n::same(s.subrange(left, right), p, k - left, j),
    decreases p.len() - j,
{
    if j < p.len() {
        assert(s[k + j as int] == s.subrange(left, right)[k - left + j as int]);
        same_window(s, p, left, right, k, j + 1);
    }
}

pub proof fn match_window(
    s: Seq<nat>,
    p: Seq<nat>,
    w: bool,
    left: nat,
    right: nat,
    a: nat,
    z: nat,
    k: int,
)
    requires
        left <= a <= k < z <= s.len(),
        left == 0 || left < a,
        z <= right <= s.len(),
        right == s.len() || z + p.len() <= right,
    ensures
        n::matches(s, p, k, w) == n::matches(
            s.subrange(left as int, right as int),
            p,
            k - left as int,
            w,
        ),
{
    hide(n::same);
    if k + p.len() <= s.len() {
        same_window(s, p, left as int, right as int, k, 0);
        let win = s.subrange(left as int, right as int);
        assert((k == 0) == (k - left as int == 0));
        if k > 0 {
            assert(win[k - left as int - 1] == s[k - 1]);
        }
        assert((k + p.len() == s.len()) == (k - left as int + p.len() == win.len()));
        if k + p.len() < s.len() {
            assert(win[k - left as int + p.len() as int] == s[k + p.len() as int]);
        }
    }
}

pub proof fn scan_window(s: Seq<nat>, p: Seq<nat>, w: bool, left: nat, right: nat, a: nat, z: nat)
    requires
        left <= a <= z <= s.len(),
        left == 0 || left < a,
        z <= right <= s.len(),
        right == s.len() || z + p.len() <= right,
        n::scan(s.subrange(left as int, right as int), p, w, (a - left) as nat, (z - left) as nat),
    ensures
        n::scan(s, p, w, a, z),
{
    hide(n::matches);
    hide(n::scan);
    let win = s.subrange(left as int, right as int);
    assert forall|k: int| a <= k < z implies !#[trigger] n::matches(s, p, k, w) by {
        n::scan_at(win, p, w, (a - left) as nat, (z - left) as nat, k - left as int);
        match_window(s, p, w, left, right, a, z, k);
    }
    scan_complete(s, p, w, a, z);
}

pub proof fn byte_codes_subrange(bytes: u::Bytes, left: int, right: int)
    requires
        0 <= left <= right <= bytes.len(),
    ensures
        byte_codes(bytes).subrange(left, right) == byte_codes(bytes.subrange(left, right)),
{
    assert(byte_codes(bytes).subrange(left, right) =~= byte_codes(bytes.subrange(left, right)));
}

pub open spec fn ranges(s: Seq<nat>, ps: Seq<Seq<nat>>, w: bool, a: nat, z: nat) -> bool
    decreases ps.len(),
{
    ps.len() == 0 || (n::scan(s, ps[0], w, a, z) && ranges(s, ps.drop_first(), w, a, z))
}

pub open spec fn clean_range(
    s: Seq<nat>,
    css: Seq<Seq<nat>>,
    market: Seq<Seq<nat>>,
    relative: Seq<Seq<nat>>,
    a: nat,
    z: nat,
) -> bool {
    n::chars_clean(s, a, z) && n::calm(s, a, z) && ranges(s, css, false, a, z) && ranges(
        s,
        market,
        true,
        a,
        z,
    ) && ranges(s, relative, true, a, z)
}

pub proof fn ranges_at(s: Seq<nat>, ps: Seq<Seq<nat>>, w: bool, a: nat, z: nat, j: int)
    requires
        ranges(s, ps, w, a, z),
        0 <= j < ps.len(),
    ensures
        n::scan(s, ps[j], w, a, z),
    decreases ps.len(),
{
    hide(n::scan);
    if j > 0 {
        ranges_at(s, ps.drop_first(), w, a, z, j - 1);
    }
}

pub proof fn ranges_make(s: Seq<nat>, ps: Seq<Seq<nat>>, w: bool, a: nat, z: nat)
    requires
        forall|j: int| 0 <= j < ps.len() ==> n::scan(s, #[trigger] ps[j], w, a, z),
    ensures
        ranges(s, ps, w, a, z),
    decreases ps.len(),
{
    hide(n::scan);
    if ps.len() > 0 {
        ranges_make(s, ps.drop_first(), w, a, z);
    }
}

pub proof fn chars_make(s: Seq<nat>, a: nat, z: nat)
    requires
        forall|k: int| a <= k < z ==> n::admitted(#[trigger] s[k]) && !n::emoji(s[k]),
    ensures
        n::chars_clean(s, a, z),
    decreases z - a,
{
    if a + 1 < z {
        let m = (a + z) / 2;
        chars_make(s, a, m);
        chars_make(s, m, z);
    }
}

pub proof fn calm_make(s: Seq<nat>, a: nat, z: nat)
    requires
        forall|k: int|
            a <= k < z ==> k + 1 >= s.len() || !n::word(#[trigger] s[k]) || s[k] == 95 || s[k + 1]
                != 33,
    ensures
        n::calm(s, a, z),
    decreases z - a,
{
    if a + 1 < z {
        let m = (a + z) / 2;
        calm_make(s, a, m);
        calm_make(s, m, z);
    }
}

pub proof fn range_window(
    s: Seq<nat>,
    ps: Seq<Seq<nat>>,
    w: bool,
    left: nat,
    right: nat,
    a: nat,
    z: nat,
)
    requires
        left <= a <= z <= s.len(),
        left == 0 || left < a,
        z <= right <= s.len(),
        forall|j: int|
            0 <= j < ps.len() ==> right == s.len() || z + (#[trigger] ps[j]).len() <= right,
        ranges(s.subrange(left as int, right as int), ps, w, (a - left) as nat, (z - left) as nat),
    ensures
        ranges(s, ps, w, a, z),
{
    hide(n::scan);
    hide(ranges);
    assert forall|j: int| 0 <= j < ps.len() implies n::scan(s, #[trigger] ps[j], w, a, z) by {
        ranges_at(
            s.subrange(left as int, right as int),
            ps,
            w,
            (a - left) as nat,
            (z - left) as nat,
            j,
        );
        scan_window(s, ps[j], w, left, right, a, z);
    }
    ranges_make(s, ps, w, a, z);
}

pub proof fn clean_window(
    s: Seq<nat>,
    css: Seq<Seq<nat>>,
    market: Seq<Seq<nat>>,
    relative: Seq<Seq<nat>>,
    left: nat,
    right: nat,
    a: nat,
    z: nat,
)
    requires
        left <= a <= z <= s.len(),
        left == 0 || left < a,
        z <= right <= s.len(),
        right == s.len() || z < right,
        forall|j: int|
            0 <= j < css.len() ==> right == s.len() || z + (#[trigger] css[j]).len() <= right,
        forall|j: int|
            0 <= j < market.len() ==> right == s.len() || z + (#[trigger] market[j]).len() <= right,
        forall|j: int|
            0 <= j < relative.len() ==> right == s.len() || z + (#[trigger] relative[j]).len()
                <= right,
        clean_range(
            s.subrange(left as int, right as int),
            css,
            market,
            relative,
            (a - left) as nat,
            (z - left) as nat,
        ),
    ensures
        clean_range(s, css, market, relative, a, z),
{
    hide(n::chars_clean);
    hide(n::calm);
    hide(ranges);
    let win = s.subrange(left as int, right as int);
    assert forall|k: int| a <= k < z implies n::admitted(#[trigger] s[k]) && !n::emoji(s[k]) by {
        n::chars_at(win, (a - left) as nat, (z - left) as nat, k - left as int);
        assert(s[k] == win[k - left as int]);
    }
    chars_make(s, a, z);
    assert forall|k: int| a <= k < z implies k + 1 >= s.len() || !n::word(#[trigger] s[k]) || s[k]
        == 95 || s[k + 1] != 33 by {
        n::calm_at(win, (a - left) as nat, (z - left) as nat, k - left as int);
        assert(s[k] == win[k - left as int]);
        if k + 1 < s.len() {
            assert(k - left as int + 1 < win.len());
            assert(s[k + 1] == win[k - left as int + 1]);
        }
    }
    calm_make(s, a, z);
    range_window(s, css, false, left, right, a, z);
    range_window(s, market, true, left, right, a, z);
    range_window(s, relative, true, left, right, a, z);
}

pub proof fn ranges_join(s: Seq<nat>, ps: Seq<Seq<nat>>, w: bool, a: nat, m: nat, z: nat)
    requires
        a <= m <= z,
        ranges(s, ps, w, a, m),
        ranges(s, ps, w, m, z),
    ensures
        ranges(s, ps, w, a, z),
    decreases ps.len(),
{
    hide(n::scan);
    if ps.len() > 0 {
        scan_join(s, ps[0], w, a, m, z);
        ranges_join(s, ps.drop_first(), w, a, m, z);
    }
}

pub proof fn clean_join(
    s: Seq<nat>,
    css: Seq<Seq<nat>>,
    market: Seq<Seq<nat>>,
    relative: Seq<Seq<nat>>,
    a: nat,
    m: nat,
    z: nat,
)
    requires
        a <= m <= z,
        clean_range(s, css, market, relative, a, m),
        clean_range(s, css, market, relative, m, z),
    ensures
        clean_range(s, css, market, relative, a, z),
{
    hide(n::chars_clean);
    hide(n::calm);
    hide(ranges);
    assert forall|k: int| a <= k < z implies n::admitted(#[trigger] s[k]) && !n::emoji(s[k]) by {
        if k < m {
            n::chars_at(s, a, m, k);
        } else {
            n::chars_at(s, m, z, k);
        }
    }
    chars_make(s, a, z);
    assert forall|k: int| a <= k < z implies k + 1 >= s.len() || !n::word(#[trigger] s[k]) || s[k]
        == 95 || s[k + 1] != 33 by {
        if k < m {
            n::calm_at(s, a, m, k);
        } else {
            n::calm_at(s, m, z, k);
        }
    }
    calm_make(s, a, z);
    ranges_join(s, css, false, a, m, z);
    ranges_join(s, market, true, a, m, z);
    ranges_join(s, relative, true, a, m, z);
}

pub proof fn ranges_empty(s: Seq<nat>, ps: Seq<Seq<nat>>, w: bool, a: nat)
    ensures
        ranges(s, ps, w, a, a),
    decreases ps.len(),
{
    if ps.len() > 0 {
        scan_empty(s, ps[0], w, a);
        ranges_empty(s, ps.drop_first(), w, a);
    }
}

pub proof fn clean_empty(
    s: Seq<nat>,
    css: Seq<Seq<nat>>,
    market: Seq<Seq<nat>>,
    relative: Seq<Seq<nat>>,
    a: nat,
)
    ensures
        clean_range(s, css, market, relative, a, a),
{
    ranges_empty(s, css, false, a);
    ranges_empty(s, market, true, a);
    ranges_empty(s, relative, true, a);
}

pub proof fn ranges_full(s: Seq<nat>, ps: Seq<Seq<nat>>, w: bool)
    ensures
        ranges(s, ps, w, 0, s.len()) == n::no_patterns(s, ps, w),
    decreases ps.len(),
{
    hide(n::scan);
    if ps.len() > 0 {
        ranges_full(s, ps.drop_first(), w);
    }
}

pub proof fn clean_full(
    s: Seq<nat>,
    css: Seq<Seq<nat>>,
    market: Seq<Seq<nat>>,
    relative: Seq<Seq<nat>>,
)
    requires
        clean_range(s, css, market, relative, 0, s.len()),
    ensures
        n::clean(s, css, market, relative),
{
    hide(n::chars_clean);
    hide(n::calm);
    ranges_full(s, css, false);
    ranges_full(s, market, true);
    ranges_full(s, relative, true);
}

} // verus!
