use crate::{k5_bytes as b, k5_codes as c, k5_copy_match as m, k5_utf8 as utf};
use ckc_spec::ui as u;
use vstd::prelude::*;
use vstd::utf8::*;
verus! {

pub fn emoji(x: u32) -> (yes: bool)
    requires
        is_scalar(x),
    ensures
        yes == u::emoji(x as char),
{
    (126975 < x && x < 129792) || (9727 < x && x < 10176) || (11007 < x && x < 11264) || x == 65039
        || x == 8205
}

pub fn admitted(x: u32) -> (yes: bool)
    requires
        is_scalar(x),
    ensures
        yes == u::copy_admitted(x as char),
{
    x < 128 || x == 0x00a7 || x == 0x00b7 || x == 0x2014 || x == 0x2265
}

pub open spec fn selected_pred(em: bool) -> spec_fn(char) -> bool {
    if em {
        |ch: char| u::emoji(ch)
    } else {
        |ch: char| !u::copy_admitted(ch)
    }
}

pub fn first_char(s: &Vec<u32>, em: bool) -> (out: Option<u32>)
    requires
        c::scalar_list(s@),
    ensures
        match out {
            Some(x) => is_scalar(x) && u::first_copy_char(c::cvs(s@), selected_pred(em)) == Some(
                x as char,
            ),
            None => u::first_copy_char(c::cvs(s@), selected_pred(em)) is None,
        },
{
    let mut i = 0;
    proof {
        assert(c::cvs(s@).skip(0) =~= c::cvs(s@));
    }
    while i < s.len()
        invariant
            c::scalar_list(s@),
            i <= s.len(),
            u::first_copy_char(c::cvs(s@).skip(i as int), selected_pred(em)) == u::first_copy_char(
                c::cvs(s@),
                selected_pred(em),
            ),
        decreases s.len() - i,
    {
        let x = s[i];
        let bad = if em {
            emoji(x)
        } else {
            !admitted(x)
        };
        if bad {
            return Some(x);
        }
        proof {
            assert(c::cvs(s@).skip(i as int).drop_first() =~= c::cvs(s@).skip(i as int + 1));
        }
        i += 1;
    }
    None
}

pub fn hex_digit(n: u32) -> (out: u8)
    requires
        n < 16,
    ensures
        out == ckc_spec::v1text::uhex_digit(n as int),
{
    if n < 10 {
        48 + n as u8
    } else {
        55 + n as u8
    }
}

pub fn hex(n: u32) -> (out: Vec<u8>)
    ensures
        out@ == u::copy_hex(n as nat),
    decreases n,
{
    if n < 65536 {
        let mut out = Vec::new();
        out.push(hex_digit(n / 4096));
        out.push(hex_digit(n / 256 % 16));
        out.push(hex_digit(n / 16 % 16));
        out.push(hex_digit(n % 16));
        out
    } else {
        let mut out = hex(n / 16);
        out.push(hex_digit(n % 16));
        out
    }
}

pub fn css() -> (out: Vec<Vec<u8>>)
    ensures
        b::views(out@) == u::css_tokens(),
{
    let data = b::literal(
        "linear-gradient radial-gradient conic-gradient @keyframes animation transition backdrop-filter box-shadow",
    );
    b::split(&data, 32)
}

pub fn marketing() -> (out: Vec<Vec<u8>>)
    ensures
        b::views(out@) == u::marketing_tokens(),
{
    let data = b::literal(
        "simply seamless seamlessly powerful robust robustly leverage leverages leveraged leveraging effortless effortlessly intuitive streamline streamlined unlock empower empowering cutting-edge state-of-the-art world-class blazing stunning delightful revolutionize game-changing supercharge best-in-class next-generation",
    );
    b::split(&data, 32)
}

pub fn relative() -> (out: Vec<Vec<u8>>)
    ensures
        b::views(out@) == u::relative_tokens(),
{
    let mut out = Vec::new();
    out.push(b::literal("ago"));
    out.push(b::literal("just now"));
    out.push(b::literal("yesterday"));
    out.push(b::literal("tomorrow"));
    out.push(b::literal("recently"));
    out.push(b::literal("last week"));
    out.push(b::literal("last month"));
    out.push(b::literal("last year"));
    proof {
        assert(b::views(out@) =~= u::relative_tokens());
    }
    out
}

pub fn token_violation(s: &Vec<u32>) -> (out: Option<Vec<u8>>)
    requires
        c::scalar_list(s@),
    ensures
        m::ov(out) == u::copy_token_violation(c::cvs(s@)),
{
    let ps = css();
    if let Some(p) = m::first_css(s, &ps) {
        return Some(b::cat(b::literal("css: "), &p));
    }
    let ps = marketing();
    if let Some(p) = m::first_word(s, &ps) {
        return Some(b::cat(b::literal("marketing: "), &p));
    }
    let ps = relative();
    if let Some(p) = m::first_word(s, &ps) {
        return Some(b::cat(b::literal("relative-time: "), &p));
    }
    if let Some(p) = m::exclamation(s) {
        return Some(b::cat(b::literal("exclamatory: "), &p));
    }
    None
}

pub fn violation(bytes: &[u8]) -> (out: Option<Vec<u8>>)
    ensures
        m::ov(out) == u::copy_violation(bytes@),
{
    hide(u::copy_token_violation);
    hide(u::first_copy_char);
    hide(u::copy_hex);
    hide(u::chars);
    hide(valid_utf8);
    hide(decode_utf8);
    hide(encode_utf8);
    if !utf::valid(bytes) {
        return Some(b::literal("domain: invalid UTF-8"));
    }
    let s = c::decode(bytes);
    if let Some(x) = first_char(&s, true) {
        proof {
            assert((x as char) as u32 == x);
            assert((x as char) as nat == x as nat);
        }
        let suffix = hex(x);
        return Some(b::cat(b::literal("emoji: U+"), &suffix));
    }
    if let Some(x) = first_char(&s, false) {
        proof {
            assert((x as char) as u32 == x);
            assert((x as char) as nat == x as nat);
        }
        let suffix = hex(x);
        return Some(b::cat(b::literal("domain: U+"), &suffix));
    }
    token_violation(&s)
}

} // verus!
