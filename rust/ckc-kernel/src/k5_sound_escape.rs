use ckc_spec::{check as ck, ui as u, v1text as v};
use vstd::prelude::*;
verus! {

broadcast use {vstd::seq::group_seq_axioms, vstd::seq_lib::group_seq_properties};

pub proof fn escaped(s: u::Bytes, attr: bool)
    ensures
        u::escaped(u::escape(s, attr), attr),
    decreases s.len(),
{
    hide(u::copy_registry);
    if s.len() > 0 {
        escaped(s.drop_first(), attr);
        let tail = u::escape(s.drop_first(), attr);
        let out = u::escape(s, attr);
        reveal_strlit("&amp;");
        vstd::utf8::is_ascii_chars_encode_utf8("&amp;"@);
        assert(u::lit("&amp;"@) =~= seq![38u8, 97u8, 109u8, 112u8, 59u8]);
        reveal_strlit("&lt;");
        vstd::utf8::is_ascii_chars_encode_utf8("&lt;"@);
        assert(u::lit("&lt;"@) =~= seq![38u8, 108u8, 116u8, 59u8]);
        reveal_strlit("&gt;");
        vstd::utf8::is_ascii_chars_encode_utf8("&gt;"@);
        assert(u::lit("&gt;"@) =~= seq![38u8, 103u8, 116u8, 59u8]);
        reveal_strlit("&quot;");
        vstd::utf8::is_ascii_chars_encode_utf8("&quot;"@);
        assert(u::lit("&quot;"@) =~= seq![38u8, 113u8, 117u8, 111u8, 116u8, 59u8]);
        reveal_strlit("&#x27;");
        vstd::utf8::is_ascii_chars_encode_utf8("&#x27;"@);
        assert(u::lit("&#x27;"@) =~= seq![38u8, 35u8, 120u8, 50u8, 55u8, 59u8]);
        if s[0] == 38 {
            assert(out.skip(5) =~= tail);
        } else if s[0] == 60 || s[0] == 62 {
            assert(out.skip(4) =~= tail);
        } else if attr && (s[0] == 34 || s[0] == 39) {
            assert(out.skip(6) =~= tail);
        } else {
            assert(out.drop_first() =~= tail);
        }
    }
}

pub proof fn decimal(n: nat)
    ensures
        u::digits(ck::nat_bytes(n)),
        n > 0 ==> ck::nat_bytes(n)[0] != 48,
    decreases n,
{
    hide(u::copy_registry);
    reveal(v::udec_bytes);
    if n >= 10 {
        decimal(n / 10);
        assert(n / 10 > 0);
        assert forall|i: int| 0 <= i < ck::nat_bytes(n).len() implies v::is_digit_b(
            #[trigger] ck::nat_bytes(n)[i],
        ) by {
            if i < ck::nat_bytes(n / 10).len() {
            }
        }
    }
}

pub proof fn comment(s: u::Bytes)
    ensures
        v::all_in(u::comment_safe(s), |x: u8| v::is_alnum_b(x) || x == 32 || x == 58 || x == 46),
{
    hide(u::copy_registry);
    assert forall|i: int| 0 <= i < u::comment_safe(s).len() implies ({
        let x = #[trigger] u::comment_safe(s)[i];
        v::is_alnum_b(x) || x == 32 || x == 58 || x == 46
    }) by {
        let c = u::chars(s)[i];
    }
}

pub open spec fn next(p: u::Piece, ctx: int) -> int {
    match p {
        u::Piece::Fixed(b) => u::fixed_context(b, ctx),
        _ => ctx,
    }
}

pub open spec fn piece_ok(p: u::Piece, ctx: int) -> bool {
    match p {
        u::Piece::Fixed(b) => u::copy_registry().contains(b),
        u::Piece::Text(b) => ctx == 0 && u::escaped(u::escape_text(b), false),
        u::Piece::Attr(b) => (ctx == 2 || ctx == 3) && u::escaped(u::escape_attr(b), true),
        u::Piece::Decimal(n) => (ctx == 0 || ctx == 2 || ctx == 3) && u::digits(ck::nat_bytes(n)),
        u::Piece::Comment(b) => ctx == 4 && v::all_in(
            u::comment_safe(b),
            |x: u8| v::is_alnum_b(x) || x == 32 || x == 58 || x == 46,
        ),
    }
}

pub closed spec fn between(h: u::Html, a: int, b: int) -> bool
    decreases h.len(),
{
    if h.len() == 0 {
        a == b
    } else {
        piece_ok(h[0], a) && between(h.drop_first(), next(h[0], a), b)
    }
}

pub proof fn between_slots(h: u::Html, a: int)
    requires
        between(h, a, 0),
    ensures
        u::escaped_slots(h, a),
    decreases h.len(),
{
    hide(u::copy_registry);
    reveal(between);
    if h.len() > 0 {
        between_slots(h.drop_first(), next(h[0], a));
    }
}

pub proof fn between_add(x: u::Html, y: u::Html, a: int, b: int, c: int)
    requires
        between(x, a, b),
        between(y, b, c),
    ensures
        between(x + y, a, c),
    decreases x.len(),
{
    hide(u::copy_registry);
    reveal(between);
    if x.len() > 0 {
        assert((x + y).drop_first() =~= x.drop_first() + y);
        between_add(x.drop_first(), y, next(x[0], a), b, c);
    } else {
        assert(x + y =~= y);
    }
}

pub open spec fn visible(h: u::Html, inputs: Seq<u::Bytes>) -> bool {
    forall|i: int|
        0 <= i < h.len() ==> match #[trigger] h[i] {
            u::Piece::Fixed(b) => u::copy_registry().contains(b),
            u::Piece::Text(b) => u::copy_derived(b, inputs, u::copy_registry()),
            _ => true,
        }
}

pub open spec fn fragment(h: u::Html, inputs: Seq<u::Bytes>, a: int, b: int) -> bool {
    between(h, a, b) && visible(h, inputs)
}

pub open spec fn sound(h: u::Html, inputs: Seq<u::Bytes>) -> bool {
    fragment(h, inputs, 0, 0)
}

pub proof fn empty(inputs: Seq<u::Bytes>, a: int)
    ensures
        fragment(Seq::empty(), inputs, a, a),
{
    hide(u::copy_registry);
    reveal(between);
}

pub proof fn add(x: u::Html, y: u::Html, inputs: Seq<u::Bytes>, a: int, b: int, c: int)
    requires
        fragment(x, inputs, a, b),
        fragment(y, inputs, b, c),
    ensures
        fragment(x + y, inputs, a, c),
{
    hide(u::copy_registry);
    between_add(x, y, a, b, c);
    assert forall|i: int| 0 <= i < (x + y).len() implies match #[trigger] (x + y)[i] {
        u::Piece::Fixed(b) => u::copy_registry().contains(b),
        u::Piece::Text(b) => u::copy_derived(b, inputs, u::copy_registry()),
        _ => true,
    } by {
        if i < x.len() {
            assert((x + y)[i] == x[i]);
        } else {
            assert((x + y)[i] == y[i - x.len()]);
        }
    }
}

pub proof fn fixed(b: u::Bytes, inputs: Seq<u::Bytes>, a: int, z: int)
    requires
        u::copy_registry().contains(b),
        u::fixed_context(b, a) == z,
    ensures
        fragment(u::fixed_bytes(b), inputs, a, z),
{
    hide(u::copy_registry);
    reveal_with_fuel(between, 2);
}

pub proof fn text(b: u::Bytes, inputs: Seq<u::Bytes>)
    requires
        u::copy_derived(b, inputs, u::copy_registry()),
    ensures
        sound(u::text(b), inputs),
{
    hide(u::copy_registry);
    escaped(b, false);
    reveal_with_fuel(between, 2);
}

pub proof fn attr(b: u::Bytes, inputs: Seq<u::Bytes>, a: int)
    requires
        a == 2 || a == 3,
    ensures
        fragment(u::attr(b), inputs, a, a),
{
    hide(u::copy_registry);
    escaped(b, true);
    reveal_with_fuel(between, 2);
}

pub proof fn number(n: nat, inputs: Seq<u::Bytes>, a: int)
    requires
        a == 0 || a == 2 || a == 3,
    ensures
        fragment(u::number(n), inputs, a, a),
{
    hide(u::copy_registry);
    decimal(n);
    reveal_with_fuel(between, 2);
}

pub proof fn comment_piece(b: u::Bytes, inputs: Seq<u::Bytes>)
    ensures
        fragment(seq![u::Piece::Comment(b)], inputs, 4, 4),
{
    hide(u::copy_registry);
    comment(b);
    reveal_with_fuel(between, 2);
}

pub proof fn page(h: u::Html, c: u::Corpus)
    requires
        sound(h, u::corpus_bytes(c)),
    ensures
        u::well_escaped(h),
        u::visible_bytes_from(h, c, u::copy_registry()),
{
    hide(u::copy_registry);
    between_slots(h, 0);
}

} // verus!
