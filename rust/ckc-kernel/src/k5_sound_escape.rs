use ckc_spec::ui as u;
use ckc_spec::{check as ck, v1text as v};
use vstd::prelude::*;
verus! {

pub proof fn ascii_literal(s: Seq<char>)
    requires
        vstd::utf8::is_ascii_chars(s),
    ensures
        u::lit(s) == v::ascii(s),
{
    vstd::utf8::is_ascii_chars_encode_utf8(s);
    assert(u::lit(s) =~= v::ascii(s));
}

pub proof fn entities()
    ensures
        u::lit("&amp;"@) == seq![38u8, 97u8, 109u8, 112u8, 59u8],
        u::lit("&lt;"@) == seq![38u8, 108u8, 116u8, 59u8],
        u::lit("&gt;"@) == seq![38u8, 103u8, 116u8, 59u8],
        u::lit("&quot;"@) == seq![38u8, 113u8, 117u8, 111u8, 116u8, 59u8],
        u::lit("&#x27;"@) == seq![38u8, 35u8, 120u8, 50u8, 55u8, 59u8],
{
    reveal_strlit("&amp;");
    reveal_strlit("&lt;");
    reveal_strlit("&gt;");
    reveal_strlit("&quot;");
    reveal_strlit("&#x27;");
    ascii_literal("&amp;"@);
    ascii_literal("&lt;"@);
    ascii_literal("&gt;"@);
    ascii_literal("&quot;"@);
    ascii_literal("&#x27;"@);
    assert(u::lit("&amp;"@) =~= seq![38u8, 97u8, 109u8, 112u8, 59u8]);
    assert(u::lit("&lt;"@) =~= seq![38u8, 108u8, 116u8, 59u8]);
    assert(u::lit("&gt;"@) =~= seq![38u8, 103u8, 116u8, 59u8]);
    assert(u::lit("&quot;"@) =~= seq![38u8, 113u8, 117u8, 111u8, 116u8, 59u8]);
    assert(u::lit("&#x27;"@) =~= seq![38u8, 35u8, 120u8, 50u8, 55u8, 59u8]);
}

pub proof fn not_starts(s: u::Bytes, p: u::Bytes, i: int)
    requires
        0 <= i < s.len(),
        i < p.len(),
        s[i] != p[i],
    ensures
        !ck::starts(s, p),
{
    if ck::starts(s, p) {
        assert(s.take(p.len() as int)[i] == s[i]);
    }
}

pub open spec fn entity(k: nat) -> u::Bytes {
    if k == 0 {
        u::lit("&amp;"@)
    } else if k == 1 {
        u::lit("&lt;"@)
    } else if k == 2 {
        u::lit("&gt;"@)
    } else if k == 3 {
        u::lit("&quot;"@)
    } else {
        u::lit("&#x27;"@)
    }
}

pub proof fn entity_escaped(k: nat, tail: u::Bytes, attr: bool)
    requires
        k < 5,
        u::escaped(tail, attr),
    ensures
        u::escaped(entity(k) + tail, attr),
{
    entities();
    let p = entity(k);
    let s = p + tail;
    assert(p.len() > 1);
    assert(s[0] == 38);
    assert(s.take(p.len() as int) =~= p);
    assert(s.skip(p.len() as int) =~= tail);
    if k > 0 {
        not_starts(s, u::lit("&amp;"@), 1);
    }
    if k >= 3 {
        not_starts(s, u::lit("&lt;"@), 1);
        not_starts(s, u::lit("&gt;"@), 1);
    }
}

pub proof fn escape_good(s: u::Bytes, attr: bool)
    ensures
        u::escaped(u::escape(s, attr), attr),
    decreases s.len(),
{
    if s.len() > 0 {
        escape_good(s.drop_first(), attr);
        let rest = u::escape(s.drop_first(), attr);
        let x = s[0];
        if x == 38 {
            entity_escaped(0, rest, attr);
        } else if x == 60 {
            entity_escaped(1, rest, attr);
        } else if x == 62 {
            entity_escaped(2, rest, attr);
        } else if attr && x == 34 {
            entity_escaped(3, rest, attr);
        } else if attr && x == 39 {
            entity_escaped(4, rest, attr);
        } else {
            assert((seq![x] + rest).drop_first() =~= rest);
        }
    }
}

pub proof fn comment_good(s: u::Bytes)
    ensures
        v::all_in(u::comment_safe(s), |x: u8| v::is_alnum_b(x) || x == 32 || x == 58 || x == 46),
{
    let out = u::comment_safe(s);
    let cs = u::chars(s);
    assert forall|i: int| 0 <= i < out.len() implies v::is_alnum_b(#[trigger] out[i]) || out[i]
        == 32 || out[i] == 58 || out[i] == 46 by {
        let n = cs[i] as int;
        if (48 <= n <= 57) || (65 <= n <= 90) || (97 <= n <= 122) || n == 32 || n == 58 || n == 95
            || n == 46 {
            assert(out[i] == n as u8);
        } else {
            assert(out[i] == 95);
        }
    }
}

pub proof fn digits_append(a: u::Bytes, b: u::Bytes)
    requires
        u::digits(a),
        u::digits(b),
    ensures
        u::digits(a + b),
{
    assert forall|i: int| 0 <= i < (a + b).len() implies v::is_digit_b(#[trigger] (a + b)[i]) by {
        if i < a.len() {
            assert((a + b)[i] == a[i]);
        } else {
            assert((a + b)[i] == b[i - a.len() as int]);
        }
    }
}

pub proof fn decimal_good(n: nat)
    ensures
        u::digits(ck::nat_bytes(n)),
        n > 0 ==> ck::nat_bytes(n)[0] != 48,
        n == 0 ==> ck::nat_bytes(n) == seq![48u8],
        ck::nat_bytes(n).len() == 1 || ck::nat_bytes(n)[0] != 48,
    decreases n,
{
    reveal_with_fuel(v::udec_bytes, 2);
    if n < 10 {
        assert(v::is_digit_b(v::digit_byte(n as int)));
    } else {
        decimal_good(n / 10);
        let prefix = ck::nat_bytes(n / 10);
        let last = seq![v::digit_byte((n % 10) as int)];
        assert(u::digits(last));
        digits_append(prefix, last);
        assert((prefix + last)[0] == prefix[0]);
    }
}

} // verus!
