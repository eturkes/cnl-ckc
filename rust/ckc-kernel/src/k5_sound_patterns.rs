use crate::{k5_sound_codes as n, k5_sound_scan as s};
use ckc_spec::{check as ck, ui as u};
use vstd::prelude::*;
use vstd::utf8::*;
#[cfg(verus_keep_ghost)]
macro_rules! bs { ($($x:expr),* $(,)?) => { seq![$($x),*] }; }
verus! {

pub open spec fn css_bytes() -> Seq<u::Bytes> {
    seq![
        bs![108u8,105,110,101,97,114,45,103,114,97,100,105,101,110,116],
        bs![114u8,97,100,105,97,108,45,103,114,97,100,105,101,110,116],
        bs![99u8,111,110,105,99,45,103,114,97,100,105,101,110,116],
        bs![64u8,107,101,121,102,114,97,109,101,115],
        bs![97u8,110,105,109,97,116,105,111,110],
        bs![116u8,114,97,110,115,105,116,105,111,110],
        bs![98u8,97,99,107,100,114,111,112,45,102,105,108,116,101,114],
        bs![98u8,111,120,45,115,104,97,100,111,119],
    ]
}

pub open spec fn css_codes() -> Seq<Seq<nat>> {
    s::pattern_codes(css_bytes())
}

pub open spec fn css_flat() -> u::Bytes {
    bs![108u8,105,110,101,97,114,45,103,114,97,100,105,101,110,116,32,114,97,100,105,97,108,45,103,114,97,100,105,101,110,116,32,99,111,110,105,99,45,103,114,97,100,105,101,110,116,32,64,107,101,121,102,114,97,109,101,115,32,97,110,105,109,97,116,105,111,110,32,116,114,97,110,115,105,116,105,111,110,32,98,97,99,107,100,114,111,112,45,102,105,108,116,101,114,32,98,111,120,45,115,104,97,100,111,119]
}

pub proof fn css()
    ensures
        n::patterns(u::css_tokens()) == css_codes(),
{
    hide(n::patterns);
    hide(s::pattern_codes);
    hide(ck::split_on);
    reveal_strlit(
        "linear-gradient radial-gradient conic-gradient @keyframes animation transition backdrop-filter box-shadow",
    );
    is_ascii_chars_encode_utf8(
        "linear-gradient radial-gradient conic-gradient @keyframes animation transition backdrop-filter box-shadow"@,
    );
    assert(u::lit(
        "linear-gradient radial-gradient conic-gradient @keyframes animation transition backdrop-filter box-shadow"@,
    ) =~= css_flat());
    assert(ck::split_on(css_flat(), 32) == css_bytes()) by (compute_only);
    assert(forall|i: int| 0 <= i < css_flat().len() ==> #[trigger] css_flat()[i] < 128);
    s::split_ascii(css_flat(), 32);
    s::ascii_patterns(css_bytes());
}

pub open spec fn marketing_bytes() -> Seq<u::Bytes> {
    seq![
        bs![115u8,105,109,112,108,121],
        bs![115u8,101,97,109,108,101,115,115],
        bs![115u8,101,97,109,108,101,115,115,108,121],
        bs![112u8,111,119,101,114,102,117,108],
        bs![114u8,111,98,117,115,116],
        bs![114u8,111,98,117,115,116,108,121],
        bs![108u8,101,118,101,114,97,103,101],
        bs![108u8,101,118,101,114,97,103,101,115],
        bs![108u8,101,118,101,114,97,103,101,100],
        bs![108u8,101,118,101,114,97,103,105,110,103],
        bs![101u8,102,102,111,114,116,108,101,115,115],
        bs![101u8,102,102,111,114,116,108,101,115,115,108,121],
        bs![105u8,110,116,117,105,116,105,118,101],
        bs![115u8,116,114,101,97,109,108,105,110,101],
        bs![115u8,116,114,101,97,109,108,105,110,101,100],
        bs![117u8,110,108,111,99,107],
        bs![101u8,109,112,111,119,101,114],
        bs![101u8,109,112,111,119,101,114,105,110,103],
        bs![99u8,117,116,116,105,110,103,45,101,100,103,101],
        bs![115u8,116,97,116,101,45,111,102,45,116,104,101,45,97,114,116],
        bs![119u8,111,114,108,100,45,99,108,97,115,115],
        bs![98u8,108,97,122,105,110,103],
        bs![115u8,116,117,110,110,105,110,103],
        bs![100u8,101,108,105,103,104,116,102,117,108],
        bs![114u8,101,118,111,108,117,116,105,111,110,105,122,101],
        bs![103u8,97,109,101,45,99,104,97,110,103,105,110,103],
        bs![115u8,117,112,101,114,99,104,97,114,103,101],
        bs![98u8,101,115,116,45,105,110,45,99,108,97,115,115],
        bs![110u8,101,120,116,45,103,101,110,101,114,97,116,105,111,110],
    ]
}

pub open spec fn marketing_codes() -> Seq<Seq<nat>> {
    s::pattern_codes(marketing_bytes())
}

pub open spec fn marketing_flat() -> u::Bytes {
    bs![115u8,105,109,112,108,121,32,115,101,97,109,108,101,115,115,32,115,101,97,109,108,101,115,115,108,121,32,112,111,119,101,114,102,117,108,32,114,111,98,117,115,116,32,114,111,98,117,115,116,108,121,32,108,101,118,101,114,97,103,101,32,108,101,118,101,114,97,103,101,115,32,108,101,118,101,114,97,103,101,100,32,108,101,118,101,114,97,103,105,110,103,32,101,102,102,111,114,116,108,101,115,115,32,101,102,102,111,114,116,108,101,115,115,108,121,32,105,110,116,117,105,116,105,118,101,32,115,116,114,101,97,109,108,105,110,101,32,115,116,114,101,97,109,108,105,110,101,100,32,117,110,108,111,99,107,32,101,109,112,111,119,101,114,32,101,109,112,111,119,101,114,105,110,103,32,99,117,116,116,105,110,103,45,101,100,103,101,32,115,116,97,116,101,45,111,102,45,116,104,101,45,97,114,116,32,119,111,114,108,100,45,99,108,97,115,115,32,98,108,97,122,105,110,103,32,115,116,117,110,110,105,110,103,32,100,101,108,105,103,104,116,102,117,108,32,114,101,118,111,108,117,116,105,111,110,105,122,101,32,103,97,109,101,45,99,104,97,110,103,105,110,103,32,115,117,112,101,114,99,104,97,114,103,101,32,98,101,115,116,45,105,110,45,99,108,97,115,115,32,110,101,120,116,45,103,101,110,101,114,97,116,105,111,110]
}

pub proof fn marketing()
    ensures
        n::patterns(u::marketing_tokens()) == marketing_codes(),
{
    hide(n::patterns);
    hide(s::pattern_codes);
    hide(ck::split_on);
    reveal_strlit(
        "simply seamless seamlessly powerful robust robustly leverage leverages leveraged leveraging effortless effortlessly intuitive streamline streamlined unlock empower empowering cutting-edge state-of-the-art world-class blazing stunning delightful revolutionize game-changing supercharge best-in-class next-generation",
    );
    is_ascii_chars_encode_utf8(
        "simply seamless seamlessly powerful robust robustly leverage leverages leveraged leveraging effortless effortlessly intuitive streamline streamlined unlock empower empowering cutting-edge state-of-the-art world-class blazing stunning delightful revolutionize game-changing supercharge best-in-class next-generation"@,
    );
    assert(u::lit(
        "simply seamless seamlessly powerful robust robustly leverage leverages leveraged leveraging effortless effortlessly intuitive streamline streamlined unlock empower empowering cutting-edge state-of-the-art world-class blazing stunning delightful revolutionize game-changing supercharge best-in-class next-generation"@,
    ) =~= marketing_flat());
    assert(ck::split_on(marketing_flat(), 32) == marketing_bytes()) by (compute_only);
    assert(forall|i: int| 0 <= i < marketing_flat().len() ==> #[trigger] marketing_flat()[i] < 128);
    s::split_ascii(marketing_flat(), 32);
    s::ascii_patterns(marketing_bytes());
}

pub open spec fn relative_bytes() -> Seq<u::Bytes> {
    seq![
        bs![97u8,103,111],
        bs![106u8,117,115,116,32,110,111,119],
        bs![121u8,101,115,116,101,114,100,97,121],
        bs![116u8,111,109,111,114,114,111,119],
        bs![114u8,101,99,101,110,116,108,121],
        bs![108u8,97,115,116,32,119,101,101,107],
        bs![108u8,97,115,116,32,109,111,110,116,104],
        bs![108u8,97,115,116,32,121,101,97,114],
    ]
}

pub open spec fn relative_codes() -> Seq<Seq<nat>> {
    s::pattern_codes(relative_bytes())
}

pub proof fn relative()
    ensures
        n::patterns(u::relative_tokens()) == relative_codes(),
{
    hide(n::patterns);
    hide(s::pattern_codes);
    hide(ck::split_on);
    reveal_strlit("ago");
    is_ascii_chars_encode_utf8("ago"@);
    assert(u::lit("ago"@) =~= bs![97u8,103,111]);
    reveal_strlit("just now");
    is_ascii_chars_encode_utf8("just now"@);
    assert(u::lit("just now"@) =~= bs![106u8,117,115,116,32,110,111,119]);
    reveal_strlit("yesterday");
    is_ascii_chars_encode_utf8("yesterday"@);
    assert(u::lit("yesterday"@) =~= bs![121u8,101,115,116,101,114,100,97,121]);
    reveal_strlit("tomorrow");
    is_ascii_chars_encode_utf8("tomorrow"@);
    assert(u::lit("tomorrow"@) =~= bs![116u8,111,109,111,114,114,111,119]);
    reveal_strlit("recently");
    is_ascii_chars_encode_utf8("recently"@);
    assert(u::lit("recently"@) =~= bs![114u8,101,99,101,110,116,108,121]);
    reveal_strlit("last week");
    is_ascii_chars_encode_utf8("last week"@);
    assert(u::lit("last week"@) =~= bs![108u8,97,115,116,32,119,101,101,107]);
    reveal_strlit("last month");
    is_ascii_chars_encode_utf8("last month"@);
    assert(u::lit("last month"@) =~= bs![108u8,97,115,116,32,109,111,110,116,104]);
    reveal_strlit("last year");
    is_ascii_chars_encode_utf8("last year"@);
    assert(u::lit("last year"@) =~= bs![108u8,97,115,116,32,121,101,97,114]);
    assert(u::relative_tokens() =~= relative_bytes());
    assert(forall|i: int, j: int|
        0 <= i < relative_bytes().len() && 0 <= j < relative_bytes()[i].len()
            ==> #[trigger] relative_bytes()[i][j] < 128);
    s::ascii_patterns(relative_bytes());
}

pub proof fn bounds()
    ensures
        forall|j: int| 0 <= j < css_codes().len() ==> (#[trigger] css_codes()[j]).len() <= 32,
        forall|j: int|
            0 <= j < marketing_codes().len() ==> (#[trigger] marketing_codes()[j]).len() <= 32,
        forall|j: int|
            0 <= j < relative_codes().len() ==> (#[trigger] relative_codes()[j]).len() <= 32,
{
}

pub proof fn all()
    ensures
        n::patterns(u::css_tokens()) == css_codes(),
        n::patterns(u::marketing_tokens()) == marketing_codes(),
        n::patterns(u::relative_tokens()) == relative_codes(),
{
    css();
    marketing();
    relative();
}

} // verus!
