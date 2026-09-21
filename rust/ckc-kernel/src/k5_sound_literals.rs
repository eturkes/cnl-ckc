use crate::{k5_sound_context as cx, k5_sound_escape as h, k5_sound_source as b};
use ckc_spec::ui as u;
use vstd::prelude::*;
use vstd::utf8::*;
verus! {

pub proof fn l000(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("&amp;"@)),
        u::copy_derived(u::lit("&amp;"@), inputs, u::copy_registry()),
        u::copy_registry()[0] == u::lit("&amp;"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[0] == u::lit("&amp;"@) && 0 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("&amp;"@), inputs);
}

pub proof fn l001(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("&lt;"@)),
        u::copy_derived(u::lit("&lt;"@), inputs, u::copy_registry()),
        u::copy_registry()[1] == u::lit("&lt;"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[1] == u::lit("&lt;"@) && 1 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("&lt;"@), inputs);
}

pub proof fn l002(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("&gt;"@)),
        u::copy_derived(u::lit("&gt;"@), inputs, u::copy_registry()),
        u::copy_registry()[2] == u::lit("&gt;"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[2] == u::lit("&gt;"@) && 2 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("&gt;"@), inputs);
}

pub proof fn l003(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("&quot;"@)),
        u::copy_derived(u::lit("&quot;"@), inputs, u::copy_registry()),
        u::copy_registry()[3] == u::lit("&quot;"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[3] == u::lit("&quot;"@) && 3 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("&quot;"@), inputs);
}

pub proof fn l004(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("&#x27;"@)),
        u::copy_derived(u::lit("&#x27;"@), inputs, u::copy_registry()),
        u::copy_registry()[4] == u::lit("&#x27;"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[4] == u::lit("&#x27;"@) && 4 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("&#x27;"@), inputs);
}

pub proof fn l005(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("\n"@)),
        u::copy_derived(u::lit("\n"@), inputs, u::copy_registry()),
        u::copy_registry()[5] == u::lit("\n"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[5] == u::lit("\n"@) && 5 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("\n"@), inputs);
}

pub proof fn f005(inputs: Seq<u::Bytes>, ctx: int)
    ensures
        h::fragment(u::fixed("\n"@), inputs, ctx, ctx),
        u::copy_registry()[5] == u::lit("\n"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l005(inputs);
    reveal_strlit("\n");
    is_ascii_chars_encode_utf8("\n"@);
    reveal_with_fuel(cx::scan, 2);
    assert(u::lit("\n"@) =~= seq![10u8]);
    assert(cx::scan(seq![10u8], ctx) == ctx);
    cx::exact(u::lit("\n"@), ctx);
    h::fixed(u::lit("\n"@), inputs, ctx, ctx);
}

pub proof fn l006(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<td>"@)),
        u::copy_derived(u::lit("<td>"@), inputs, u::copy_registry()),
        u::copy_registry()[6] == u::lit("<td>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[6] == u::lit("<td>"@) && 6 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<td>"@), inputs);
}

pub proof fn f006(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<td>"@), inputs, 0, 0),
        u::copy_registry()[6] == u::lit("<td>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l006(inputs);
    reveal_strlit("<td>");
    is_ascii_chars_encode_utf8("<td>"@);
    assert(u::lit("<td>"@) =~= seq![60u8, 116, 100, 62]);
    assert(cx::scan(seq![60u8, 116, 100, 62], 0) == 0) by (compute_only);
    cx::exact(u::lit("<td>"@), 0);
    h::fixed(u::lit("<td>"@), inputs, 0, 0);
}

pub proof fn l007(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("</td>"@)),
        u::copy_derived(u::lit("</td>"@), inputs, u::copy_registry()),
        u::copy_registry()[7] == u::lit("</td>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[7] == u::lit("</td>"@) && 7 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("</td>"@), inputs);
}

pub proof fn f007(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("</td>"@), inputs, 0, 0),
        u::copy_registry()[7] == u::lit("</td>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l007(inputs);
    reveal_strlit("</td>");
    is_ascii_chars_encode_utf8("</td>"@);
    assert(u::lit("</td>"@) =~= seq![60u8, 47, 116, 100, 62]);
    assert(cx::scan(seq![60u8, 47, 116, 100, 62], 0) == 0) by (compute_only);
    cx::exact(u::lit("</td>"@), 0);
    h::fixed(u::lit("</td>"@), inputs, 0, 0);
}

pub proof fn l008(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<tr>"@)),
        u::copy_derived(u::lit("<tr>"@), inputs, u::copy_registry()),
        u::copy_registry()[8] == u::lit("<tr>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[8] == u::lit("<tr>"@) && 8 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<tr>"@), inputs);
}

pub proof fn f008(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<tr>"@), inputs, 0, 0),
        u::copy_registry()[8] == u::lit("<tr>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l008(inputs);
    reveal_strlit("<tr>");
    is_ascii_chars_encode_utf8("<tr>"@);
    assert(u::lit("<tr>"@) =~= seq![60u8, 116, 114, 62]);
    assert(cx::scan(seq![60u8, 116, 114, 62], 0) == 0) by (compute_only);
    cx::exact(u::lit("<tr>"@), 0);
    h::fixed(u::lit("<tr>"@), inputs, 0, 0);
}

pub proof fn l009(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("</tr>"@)),
        u::copy_derived(u::lit("</tr>"@), inputs, u::copy_registry()),
        u::copy_registry()[9] == u::lit("</tr>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[9] == u::lit("</tr>"@) && 9 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("</tr>"@), inputs);
}

pub proof fn f009(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("</tr>"@), inputs, 0, 0),
        u::copy_registry()[9] == u::lit("</tr>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l009(inputs);
    reveal_strlit("</tr>");
    is_ascii_chars_encode_utf8("</tr>"@);
    assert(u::lit("</tr>"@) =~= seq![60u8, 47, 116, 114, 62]);
    assert(cx::scan(seq![60u8, 47, 116, 114, 62], 0) == 0) by (compute_only);
    cx::exact(u::lit("</tr>"@), 0);
    h::fixed(u::lit("</tr>"@), inputs, 0, 0);
}

pub proof fn l010(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<a href=\""@)),
        u::copy_derived(u::lit("<a href=\""@), inputs, u::copy_registry()),
        u::copy_registry()[10] == u::lit("<a href=\""@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[10] == u::lit("<a href=\""@) && 10 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<a href=\""@), inputs);
}

pub proof fn f010(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<a href=\""@), inputs, 0, 2),
        u::copy_registry()[10] == u::lit("<a href=\""@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l010(inputs);
    reveal_strlit("<a href=\"");
    is_ascii_chars_encode_utf8("<a href=\""@);
    assert(u::lit("<a href=\""@) =~= seq![60u8, 97, 32, 104, 114, 101, 102, 61, 34]);
    assert(cx::scan(seq![60u8, 97, 32, 104, 114, 101, 102, 61, 34], 0) == 2) by (compute_only);
    cx::exact(u::lit("<a href=\""@), 0);
    h::fixed(u::lit("<a href=\""@), inputs, 0, 2);
}

pub proof fn l011(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("\">"@)),
        u::copy_derived(u::lit("\">"@), inputs, u::copy_registry()),
        u::copy_registry()[11] == u::lit("\">"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[11] == u::lit("\">"@) && 11 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("\">"@), inputs);
}

pub proof fn f011(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("\">"@), inputs, 2, 0),
        u::copy_registry()[11] == u::lit("\">"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l011(inputs);
    reveal_strlit("\">");
    is_ascii_chars_encode_utf8("\">"@);
    assert(u::lit("\">"@) =~= seq![34u8, 62]);
    assert(cx::scan(seq![34u8, 62], 2) == 0) by (compute_only);
    cx::exact(u::lit("\">"@), 2);
    h::fixed(u::lit("\">"@), inputs, 2, 0);
}

pub proof fn l012(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("</a>"@)),
        u::copy_derived(u::lit("</a>"@), inputs, u::copy_registry()),
        u::copy_registry()[12] == u::lit("</a>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[12] == u::lit("</a>"@) && 12 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("</a>"@), inputs);
}

pub proof fn f012(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("</a>"@), inputs, 0, 0),
        u::copy_registry()[12] == u::lit("</a>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l012(inputs);
    reveal_strlit("</a>");
    is_ascii_chars_encode_utf8("</a>"@);
    assert(u::lit("</a>"@) =~= seq![60u8, 47, 97, 62]);
    assert(cx::scan(seq![60u8, 47, 97, 62], 0) == 0) by (compute_only);
    cx::exact(u::lit("</a>"@), 0);
    h::fixed(u::lit("</a>"@), inputs, 0, 0);
}

pub proof fn l013(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("approved"@)),
        u::copy_derived(u::lit("approved"@), inputs, u::copy_registry()),
        u::copy_registry()[13] == u::lit("approved"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[13] == u::lit("approved"@) && 13 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("approved"@), inputs);
}

pub proof fn l014(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("rejected"@)),
        u::copy_derived(u::lit("rejected"@), inputs, u::copy_registry()),
        u::copy_registry()[14] == u::lit("rejected"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[14] == u::lit("rejected"@) && 14 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("rejected"@), inputs);
}

pub proof fn l015(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("contested"@)),
        u::copy_derived(u::lit("contested"@), inputs, u::copy_registry()),
        u::copy_registry()[15] == u::lit("contested"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[15] == u::lit("contested"@) && 15 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("contested"@), inputs);
}

pub proof fn l016(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("stale"@)),
        u::copy_derived(u::lit("stale"@), inputs, u::copy_registry()),
        u::copy_registry()[16] == u::lit("stale"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[16] == u::lit("stale"@) && 16 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("stale"@), inputs);
}

pub proof fn l017(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("unreviewed"@)),
        u::copy_derived(u::lit("unreviewed"@), inputs, u::copy_registry()),
        u::copy_registry()[17] == u::lit("unreviewed"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[17] == u::lit("unreviewed"@) && 17 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("unreviewed"@), inputs);
}

pub proof fn l018(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("Approved"@)),
        u::copy_derived(u::lit("Approved"@), inputs, u::copy_registry()),
        u::copy_registry()[18] == u::lit("Approved"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[18] == u::lit("Approved"@) && 18 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("Approved"@), inputs);
}

pub proof fn l019(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("Rejected"@)),
        u::copy_derived(u::lit("Rejected"@), inputs, u::copy_registry()),
        u::copy_registry()[19] == u::lit("Rejected"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[19] == u::lit("Rejected"@) && 19 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("Rejected"@), inputs);
}

pub proof fn l020(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("Contested"@)),
        u::copy_derived(u::lit("Contested"@), inputs, u::copy_registry()),
        u::copy_registry()[20] == u::lit("Contested"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[20] == u::lit("Contested"@) && 20 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("Contested"@), inputs);
}

pub proof fn l021(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("Outdated"@)),
        u::copy_derived(u::lit("Outdated"@), inputs, u::copy_registry()),
        u::copy_registry()[21] == u::lit("Outdated"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[21] == u::lit("Outdated"@) && 21 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("Outdated"@), inputs);
}

pub proof fn l022(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("Unreviewed"@)),
        u::copy_derived(u::lit("Unreviewed"@), inputs, u::copy_registry()),
        u::copy_registry()[22] == u::lit("Unreviewed"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[22] == u::lit("Unreviewed"@) && 22 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("Unreviewed"@), inputs);
}

pub proof fn l023(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<span class=\"chip chip-"@)),
        u::copy_derived(u::lit("<span class=\"chip chip-"@), inputs, u::copy_registry()),
        u::copy_registry()[23] == u::lit("<span class=\"chip chip-"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[23] == u::lit("<span class=\"chip chip-"@) && 23
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<span class=\"chip chip-"@), inputs);
}

pub proof fn f023(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<span class=\"chip chip-"@), inputs, 0, 2),
        u::copy_registry()[23] == u::lit("<span class=\"chip chip-"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l023(inputs);
    reveal_strlit("<span class=\"chip chip-");
    is_ascii_chars_encode_utf8("<span class=\"chip chip-"@);
    assert(u::lit("<span class=\"chip chip-"@) =~= seq![
        60u8,
        115,
        112,
        97,
        110,
        32,
        99,
        108,
        97,
        115,
        115,
        61,
        34,
        99,
        104,
        105,
        112,
        32,
        99,
        104,
        105,
        112,
        45,
    ]);
    assert(cx::scan(
        seq![
            60u8,
            115,
            112,
            97,
            110,
            32,
            99,
            108,
            97,
            115,
            115,
            61,
            34,
            99,
            104,
            105,
            112,
            32,
            99,
            104,
            105,
            112,
            45,
        ],
        0,
    ) == 2) by (compute_only);
    cx::exact(u::lit("<span class=\"chip chip-"@), 0);
    h::fixed(u::lit("<span class=\"chip chip-"@), inputs, 0, 2);
}

pub proof fn l024(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("</span>"@)),
        u::copy_derived(u::lit("</span>"@), inputs, u::copy_registry()),
        u::copy_registry()[24] == u::lit("</span>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[24] == u::lit("</span>"@) && 24 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("</span>"@), inputs);
}

pub proof fn f024(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("</span>"@), inputs, 0, 0),
        u::copy_registry()[24] == u::lit("</span>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l024(inputs);
    reveal_strlit("</span>");
    is_ascii_chars_encode_utf8("</span>"@);
    assert(u::lit("</span>"@) =~= seq![60u8, 47, 115, 112, 97, 110, 62]);
    assert(cx::scan(seq![60u8, 47, 115, 112, 97, 110, 62], 0) == 0) by (compute_only);
    cx::exact(u::lit("</span>"@), 0);
    h::fixed(u::lit("</span>"@), inputs, 0, 0);
}

pub proof fn l025(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("# "@)),
        u::copy_derived(u::lit("# "@), inputs, u::copy_registry()),
        u::copy_registry()[25] == u::lit("# "@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[25] == u::lit("# "@) && 25 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("# "@), inputs);
}

pub proof fn l026(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("Rec"@)),
        u::copy_derived(u::lit("Rec"@), inputs, u::copy_registry()),
        u::copy_registry()[26] == u::lit("Rec"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[26] == u::lit("Rec"@) && 26 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("Rec"@), inputs);
}

pub proof fn l027(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("Recommendation "@)),
        u::copy_derived(u::lit("Recommendation "@), inputs, u::copy_registry()),
        u::copy_registry()[27] == u::lit("Recommendation "@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[27] == u::lit("Recommendation "@) && 27 < u::copy_registry().len())
        by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("Recommendation "@), inputs);
}

pub proof fn l028(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("BOX"@)),
        u::copy_derived(u::lit("BOX"@), inputs, u::copy_registry()),
        u::copy_registry()[28] == u::lit("BOX"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[28] == u::lit("BOX"@) && 28 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("BOX"@), inputs);
}

pub proof fn l029(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit(" · "@)),
        u::copy_derived(u::lit(" · "@), inputs, u::copy_registry()),
        u::copy_registry()[29] == u::lit(" · "@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[29] == u::lit(" · "@) && 29 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit(" · "@), inputs);
}

pub proof fn f029(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed(" · "@), inputs, 0, 0),
        u::copy_registry()[29] == u::lit(" · "@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l029(inputs);
    reveal_strlit(" · ");
    let cs = " · "@;
    assert(cs =~= cs.take(1) + seq![cs[1]] + cs.skip(2));
    is_ascii_chars_encode_utf8(cs.take(1));
    is_ascii_chars_encode_utf8(cs.skip(2));
    encode_utf8_concat(cs.take(1), seq![cs[1]]);
    encode_utf8_concat(cs.take(1) + seq![cs[1]], cs.skip(2));
    reveal_with_fuel(encode_utf8, 2);
    assert(encode_scalar(183u32) == seq![194u8, 183]) by (compute_only);
    assert(cs[1] as u32 == 183);
    assert(u::lit(" · "@) =~= seq![32u8, 194, 183, 32]);
    assert(cx::scan(seq![32u8, 194, 183, 32], 0) == 0) by (compute_only);
    cx::exact(u::lit(" · "@), 0);
    h::fixed(u::lit(" · "@), inputs, 0, 0);
}

pub proof fn l030(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("p"@)),
        u::copy_derived(u::lit("p"@), inputs, u::copy_registry()),
        u::copy_registry()[30] == u::lit("p"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[30] == u::lit("p"@) && 30 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("p"@), inputs);
}

pub proof fn l031(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit(", page "@)),
        u::copy_derived(u::lit(", page "@), inputs, u::copy_registry()),
        u::copy_registry()[31] == u::lit(", page "@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[31] == u::lit(", page "@) && 31 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit(", page "@), inputs);
}

pub proof fn l032(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit(", passage "@)),
        u::copy_derived(u::lit(", passage "@), inputs, u::copy_registry()),
        u::copy_registry()[32] == u::lit(", passage "@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[32] == u::lit(", passage "@) && 32 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit(", passage "@), inputs);
}

pub proof fn l033(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit(" ("@)),
        u::copy_derived(u::lit(" ("@), inputs, u::copy_registry()),
        u::copy_registry()[33] == u::lit(" ("@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[33] == u::lit(" ("@) && 33 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit(" ("@), inputs);
}

pub proof fn l034(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit(")"@)),
        u::copy_derived(u::lit(")"@), inputs, u::copy_registry()),
        u::copy_registry()[34] == u::lit(")"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[34] == u::lit(")"@) && 34 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit(")"@), inputs);
}

pub proof fn l035(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("Z"@)),
        u::copy_derived(u::lit("Z"@), inputs, u::copy_registry()),
        u::copy_registry()[35] == u::lit("Z"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[35] == u::lit("Z"@) && 35 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("Z"@), inputs);
}

pub proof fn l036(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit(" "@)),
        u::copy_derived(u::lit(" "@), inputs, u::copy_registry()),
        u::copy_registry()[36] == u::lit(" "@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[36] == u::lit(" "@) && 36 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit(" "@), inputs);
}

pub proof fn f036(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed(" "@), inputs, 0, 0),
        u::copy_registry()[36] == u::lit(" "@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l036(inputs);
    reveal_strlit(" ");
    is_ascii_chars_encode_utf8(" "@);
    assert(u::lit(" "@) =~= seq![32u8]);
    assert(cx::scan(seq![32u8], 0) == 0) by (compute_only);
    cx::exact(u::lit(" "@), 0);
    h::fixed(u::lit(" "@), inputs, 0, 0);
}

pub proof fn l037(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit(" UTC"@)),
        u::copy_derived(u::lit(" UTC"@), inputs, u::copy_registry()),
        u::copy_registry()[37] == u::lit(" UTC"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[37] == u::lit(" UTC"@) && 37 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit(" UTC"@), inputs);
}

pub proof fn l038(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::css_text()),
        u::copy_derived(u::css_text(), inputs, u::copy_registry()),
        u::copy_registry()[38] == u::css_text(),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[38] == u::css_text() && 38 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::css_text(), inputs);
}

pub proof fn l039(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::script_html()),
        u::copy_derived(u::script_html(), inputs, u::copy_registry()),
        u::copy_registry()[39] == u::script_html(),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[39] == u::script_html() && 39 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::script_html(), inputs);
}

pub proof fn l040(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<!doctype html>"@)),
        u::copy_derived(u::lit("<!doctype html>"@), inputs, u::copy_registry()),
        u::copy_registry()[40] == u::lit("<!doctype html>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[40] == u::lit("<!doctype html>"@) && 40 < u::copy_registry().len())
        by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<!doctype html>"@), inputs);
}

pub proof fn f040(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<!doctype html>"@), inputs, 0, 0),
        u::copy_registry()[40] == u::lit("<!doctype html>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l040(inputs);
    reveal_strlit("<!doctype html>");
    is_ascii_chars_encode_utf8("<!doctype html>"@);
    assert(u::lit("<!doctype html>"@) =~= seq![
        60u8,
        33,
        100,
        111,
        99,
        116,
        121,
        112,
        101,
        32,
        104,
        116,
        109,
        108,
        62,
    ]);
    assert(cx::scan(seq![60u8, 33, 100, 111, 99, 116, 121, 112, 101, 32, 104, 116, 109, 108, 62], 0)
        == 0) by (compute_only);
    cx::exact(u::lit("<!doctype html>"@), 0);
    h::fixed(u::lit("<!doctype html>"@), inputs, 0, 0);
}

pub proof fn l041(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<html lang=\"en\">"@)),
        u::copy_derived(u::lit("<html lang=\"en\">"@), inputs, u::copy_registry()),
        u::copy_registry()[41] == u::lit("<html lang=\"en\">"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[41] == u::lit("<html lang=\"en\">"@) && 41 < u::copy_registry().len())
        by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<html lang=\"en\">"@), inputs);
}

pub proof fn f041(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<html lang=\"en\">"@), inputs, 0, 0),
        u::copy_registry()[41] == u::lit("<html lang=\"en\">"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l041(inputs);
    reveal_strlit("<html lang=\"en\">");
    is_ascii_chars_encode_utf8("<html lang=\"en\">"@);
    assert(u::lit("<html lang=\"en\">"@) =~= seq![
        60u8,
        104,
        116,
        109,
        108,
        32,
        108,
        97,
        110,
        103,
        61,
        34,
        101,
        110,
        34,
        62,
    ]);
    assert(cx::scan(
        seq![60u8, 104, 116, 109, 108, 32, 108, 97, 110, 103, 61, 34, 101, 110, 34, 62],
        0,
    ) == 0) by (compute_only);
    cx::exact(u::lit("<html lang=\"en\">"@), 0);
    h::fixed(u::lit("<html lang=\"en\">"@), inputs, 0, 0);
}

pub proof fn l042(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<head>"@)),
        u::copy_derived(u::lit("<head>"@), inputs, u::copy_registry()),
        u::copy_registry()[42] == u::lit("<head>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[42] == u::lit("<head>"@) && 42 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<head>"@), inputs);
}

pub proof fn f042(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<head>"@), inputs, 0, 0),
        u::copy_registry()[42] == u::lit("<head>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l042(inputs);
    reveal_strlit("<head>");
    is_ascii_chars_encode_utf8("<head>"@);
    assert(u::lit("<head>"@) =~= seq![60u8, 104, 101, 97, 100, 62]);
    assert(cx::scan(seq![60u8, 104, 101, 97, 100, 62], 0) == 0) by (compute_only);
    cx::exact(u::lit("<head>"@), 0);
    h::fixed(u::lit("<head>"@), inputs, 0, 0);
}

pub proof fn l043(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<meta charset=\"utf-8\">"@)),
        u::copy_derived(u::lit("<meta charset=\"utf-8\">"@), inputs, u::copy_registry()),
        u::copy_registry()[43] == u::lit("<meta charset=\"utf-8\">"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[43] == u::lit("<meta charset=\"utf-8\">"@) && 43
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<meta charset=\"utf-8\">"@), inputs);
}

pub proof fn f043(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<meta charset=\"utf-8\">"@), inputs, 0, 0),
        u::copy_registry()[43] == u::lit("<meta charset=\"utf-8\">"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l043(inputs);
    reveal_strlit("<meta charset=\"utf-8\">");
    is_ascii_chars_encode_utf8("<meta charset=\"utf-8\">"@);
    assert(u::lit("<meta charset=\"utf-8\">"@) =~= seq![
        60u8,
        109,
        101,
        116,
        97,
        32,
        99,
        104,
        97,
        114,
        115,
        101,
        116,
        61,
        34,
        117,
        116,
        102,
        45,
        56,
        34,
        62,
    ]);
    assert(cx::scan(
        seq![
            60u8,
            109,
            101,
            116,
            97,
            32,
            99,
            104,
            97,
            114,
            115,
            101,
            116,
            61,
            34,
            117,
            116,
            102,
            45,
            56,
            34,
            62,
        ],
        0,
    ) == 0) by (compute_only);
    cx::exact(u::lit("<meta charset=\"utf-8\">"@), 0);
    h::fixed(u::lit("<meta charset=\"utf-8\">"@), inputs, 0, 0);
}

pub proof fn l044(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<title>"@)),
        u::copy_derived(u::lit("<title>"@), inputs, u::copy_registry()),
        u::copy_registry()[44] == u::lit("<title>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[44] == u::lit("<title>"@) && 44 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<title>"@), inputs);
}

pub proof fn f044(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<title>"@), inputs, 0, 0),
        u::copy_registry()[44] == u::lit("<title>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l044(inputs);
    reveal_strlit("<title>");
    is_ascii_chars_encode_utf8("<title>"@);
    assert(u::lit("<title>"@) =~= seq![60u8, 116, 105, 116, 108, 101, 62]);
    assert(cx::scan(seq![60u8, 116, 105, 116, 108, 101, 62], 0) == 0) by (compute_only);
    cx::exact(u::lit("<title>"@), 0);
    h::fixed(u::lit("<title>"@), inputs, 0, 0);
}

pub proof fn l045(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit(" — cnl-ckc reviewer</title>"@)),
        u::copy_derived(u::lit(" — cnl-ckc reviewer</title>"@), inputs, u::copy_registry()),
        u::copy_registry()[45] == u::lit(" — cnl-ckc reviewer</title>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[45] == u::lit(" — cnl-ckc reviewer</title>"@) && 45
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit(" — cnl-ckc reviewer</title>"@), inputs);
}

pub proof fn f045(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed(" — cnl-ckc reviewer</title>"@), inputs, 0, 0),
        u::copy_registry()[45] == u::lit(" — cnl-ckc reviewer</title>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l045(inputs);
    reveal_strlit(" — cnl-ckc reviewer</title>");
    let cs = " — cnl-ckc reviewer</title>"@;
    assert(cs =~= cs.take(1) + seq![cs[1]] + cs.skip(2));
    is_ascii_chars_encode_utf8(cs.take(1));
    is_ascii_chars_encode_utf8(cs.skip(2));
    encode_utf8_concat(cs.take(1), seq![cs[1]]);
    encode_utf8_concat(cs.take(1) + seq![cs[1]], cs.skip(2));
    reveal_with_fuel(encode_utf8, 2);
    assert(encode_scalar(8212u32) == seq![226u8, 128, 148]) by (compute_only);
    assert(cs[1] as u32 == 8212);
    assert(u::lit(" — cnl-ckc reviewer</title>"@) =~= seq![
        32u8,
        226,
        128,
        148,
        32,
        99,
        110,
        108,
        45,
        99,
        107,
        99,
        32,
        114,
        101,
        118,
        105,
        101,
        119,
        101,
        114,
        60,
        47,
        116,
        105,
        116,
        108,
        101,
        62,
    ]);
    assert(cx::scan(
        seq![
            32u8,
            226,
            128,
            148,
            32,
            99,
            110,
            108,
            45,
            99,
            107,
            99,
            32,
            114,
            101,
            118,
            105,
            101,
            119,
            101,
            114,
            60,
            47,
            116,
            105,
            116,
            108,
            101,
            62,
        ],
        0,
    ) == 0) by (compute_only);
    cx::exact(u::lit(" — cnl-ckc reviewer</title>"@), 0);
    h::fixed(u::lit(" — cnl-ckc reviewer</title>"@), inputs, 0, 0);
}

pub proof fn l046(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<style>"@)),
        u::copy_derived(u::lit("<style>"@), inputs, u::copy_registry()),
        u::copy_registry()[46] == u::lit("<style>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[46] == u::lit("<style>"@) && 46 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<style>"@), inputs);
}

pub proof fn f046(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<style>"@), inputs, 0, 5),
        u::copy_registry()[46] == u::lit("<style>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l046(inputs);
    reveal_strlit("<style>");
    is_ascii_chars_encode_utf8("<style>"@);
    assert(u::lit("<style>"@) =~= seq![60u8, 115, 116, 121, 108, 101, 62]);
    assert(cx::scan(seq![60u8, 115, 116, 121, 108, 101, 62], 0) == 5) by (compute_only);
    cx::exact(u::lit("<style>"@), 0);
    h::fixed(u::lit("<style>"@), inputs, 0, 5);
}

pub proof fn l047(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("</style>"@)),
        u::copy_derived(u::lit("</style>"@), inputs, u::copy_registry()),
        u::copy_registry()[47] == u::lit("</style>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[47] == u::lit("</style>"@) && 47 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("</style>"@), inputs);
}

pub proof fn f047(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("</style>"@), inputs, 5, 0),
        u::copy_registry()[47] == u::lit("</style>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l047(inputs);
    reveal_strlit("</style>");
    is_ascii_chars_encode_utf8("</style>"@);
    assert(u::lit("</style>"@) =~= seq![60u8, 47, 115, 116, 121, 108, 101, 62]);
    assert(cx::scan(seq![60u8, 47, 115, 116, 121, 108, 101, 62], 5) == 0) by (compute_only);
    cx::exact(u::lit("</style>"@), 5);
    h::fixed(u::lit("</style>"@), inputs, 5, 0);
}

pub proof fn l048(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("</head>"@)),
        u::copy_derived(u::lit("</head>"@), inputs, u::copy_registry()),
        u::copy_registry()[48] == u::lit("</head>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[48] == u::lit("</head>"@) && 48 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("</head>"@), inputs);
}

pub proof fn f048(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("</head>"@), inputs, 0, 0),
        u::copy_registry()[48] == u::lit("</head>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l048(inputs);
    reveal_strlit("</head>");
    is_ascii_chars_encode_utf8("</head>"@);
    assert(u::lit("</head>"@) =~= seq![60u8, 47, 104, 101, 97, 100, 62]);
    assert(cx::scan(seq![60u8, 47, 104, 101, 97, 100, 62], 0) == 0) by (compute_only);
    cx::exact(u::lit("</head>"@), 0);
    h::fixed(u::lit("</head>"@), inputs, 0, 0);
}

pub proof fn l049(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<body>"@)),
        u::copy_derived(u::lit("<body>"@), inputs, u::copy_registry()),
        u::copy_registry()[49] == u::lit("<body>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[49] == u::lit("<body>"@) && 49 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<body>"@), inputs);
}

pub proof fn f049(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<body>"@), inputs, 0, 0),
        u::copy_registry()[49] == u::lit("<body>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l049(inputs);
    reveal_strlit("<body>");
    is_ascii_chars_encode_utf8("<body>"@);
    assert(u::lit("<body>"@) =~= seq![60u8, 98, 111, 100, 121, 62]);
    assert(cx::scan(seq![60u8, 98, 111, 100, 121, 62], 0) == 0) by (compute_only);
    cx::exact(u::lit("<body>"@), 0);
    h::fixed(u::lit("<body>"@), inputs, 0, 0);
}

pub proof fn l050(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(
            u::lit("<a class=\"skip\" href=\"#main\">Skip to content</a>"@),
        ),
        u::copy_derived(
            u::lit("<a class=\"skip\" href=\"#main\">Skip to content</a>"@),
            inputs,
            u::copy_registry(),
        ),
        u::copy_registry()[50] == u::lit("<a class=\"skip\" href=\"#main\">Skip to content</a>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[50] == u::lit("<a class=\"skip\" href=\"#main\">Skip to content</a>"@)
        && 50 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<a class=\"skip\" href=\"#main\">Skip to content</a>"@), inputs);
}

pub proof fn f050(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(
            u::fixed("<a class=\"skip\" href=\"#main\">Skip to content</a>"@),
            inputs,
            0,
            0,
        ),
        u::copy_registry()[50] == u::lit("<a class=\"skip\" href=\"#main\">Skip to content</a>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l050(inputs);
    reveal_strlit("<a class=\"skip\" href=\"#main\">Skip to content</a>");
    is_ascii_chars_encode_utf8("<a class=\"skip\" href=\"#main\">Skip to content</a>"@);
    assert(u::lit("<a class=\"skip\" href=\"#main\">Skip to content</a>"@) =~= seq![
        60u8,
        97,
        32,
        99,
        108,
        97,
        115,
        115,
        61,
        34,
        115,
        107,
        105,
        112,
        34,
        32,
        104,
        114,
        101,
        102,
        61,
        34,
        35,
        109,
        97,
        105,
        110,
        34,
        62,
        83,
        107,
        105,
        112,
        32,
        116,
        111,
        32,
        99,
        111,
        110,
        116,
        101,
        110,
        116,
        60,
        47,
        97,
        62,
    ]);
    assert(cx::scan(
        seq![
            60u8,
            97,
            32,
            99,
            108,
            97,
            115,
            115,
            61,
            34,
            115,
            107,
            105,
            112,
            34,
            32,
            104,
            114,
            101,
            102,
            61,
            34,
            35,
            109,
            97,
            105,
            110,
            34,
            62,
            83,
            107,
            105,
            112,
            32,
            116,
            111,
            32,
            99,
            111,
            110,
            116,
            101,
            110,
            116,
            60,
            47,
            97,
            62,
        ],
        0,
    ) == 0) by (compute_only);
    cx::exact(u::lit("<a class=\"skip\" href=\"#main\">Skip to content</a>"@), 0);
    h::fixed(u::lit("<a class=\"skip\" href=\"#main\">Skip to content</a>"@), inputs, 0, 0);
}

pub proof fn l051(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<nav class=\"crumbs\">"@)),
        u::copy_derived(u::lit("<nav class=\"crumbs\">"@), inputs, u::copy_registry()),
        u::copy_registry()[51] == u::lit("<nav class=\"crumbs\">"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[51] == u::lit("<nav class=\"crumbs\">"@) && 51
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<nav class=\"crumbs\">"@), inputs);
}

pub proof fn f051(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<nav class=\"crumbs\">"@), inputs, 0, 0),
        u::copy_registry()[51] == u::lit("<nav class=\"crumbs\">"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l051(inputs);
    reveal_strlit("<nav class=\"crumbs\">");
    is_ascii_chars_encode_utf8("<nav class=\"crumbs\">"@);
    assert(u::lit("<nav class=\"crumbs\">"@) =~= seq![
        60u8,
        110,
        97,
        118,
        32,
        99,
        108,
        97,
        115,
        115,
        61,
        34,
        99,
        114,
        117,
        109,
        98,
        115,
        34,
        62,
    ]);
    assert(cx::scan(
        seq![
            60u8,
            110,
            97,
            118,
            32,
            99,
            108,
            97,
            115,
            115,
            61,
            34,
            99,
            114,
            117,
            109,
            98,
            115,
            34,
            62,
        ],
        0,
    ) == 0) by (compute_only);
    cx::exact(u::lit("<nav class=\"crumbs\">"@), 0);
    h::fixed(u::lit("<nav class=\"crumbs\">"@), inputs, 0, 0);
}

pub proof fn l052(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("</nav>"@)),
        u::copy_derived(u::lit("</nav>"@), inputs, u::copy_registry()),
        u::copy_registry()[52] == u::lit("</nav>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[52] == u::lit("</nav>"@) && 52 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("</nav>"@), inputs);
}

pub proof fn f052(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("</nav>"@), inputs, 0, 0),
        u::copy_registry()[52] == u::lit("</nav>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l052(inputs);
    reveal_strlit("</nav>");
    is_ascii_chars_encode_utf8("</nav>"@);
    assert(u::lit("</nav>"@) =~= seq![60u8, 47, 110, 97, 118, 62]);
    assert(cx::scan(seq![60u8, 47, 110, 97, 118, 62], 0) == 0) by (compute_only);
    cx::exact(u::lit("</nav>"@), 0);
    h::fixed(u::lit("</nav>"@), inputs, 0, 0);
}

pub proof fn l053(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<main id=\"main\">"@)),
        u::copy_derived(u::lit("<main id=\"main\">"@), inputs, u::copy_registry()),
        u::copy_registry()[53] == u::lit("<main id=\"main\">"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[53] == u::lit("<main id=\"main\">"@) && 53 < u::copy_registry().len())
        by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<main id=\"main\">"@), inputs);
}

pub proof fn f053(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<main id=\"main\">"@), inputs, 0, 0),
        u::copy_registry()[53] == u::lit("<main id=\"main\">"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l053(inputs);
    reveal_strlit("<main id=\"main\">");
    is_ascii_chars_encode_utf8("<main id=\"main\">"@);
    assert(u::lit("<main id=\"main\">"@) =~= seq![
        60u8,
        109,
        97,
        105,
        110,
        32,
        105,
        100,
        61,
        34,
        109,
        97,
        105,
        110,
        34,
        62,
    ]);
    assert(cx::scan(
        seq![60u8, 109, 97, 105, 110, 32, 105, 100, 61, 34, 109, 97, 105, 110, 34, 62],
        0,
    ) == 0) by (compute_only);
    cx::exact(u::lit("<main id=\"main\">"@), 0);
    h::fixed(u::lit("<main id=\"main\">"@), inputs, 0, 0);
}

pub proof fn l054(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("</main>"@)),
        u::copy_derived(u::lit("</main>"@), inputs, u::copy_registry()),
        u::copy_registry()[54] == u::lit("</main>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[54] == u::lit("</main>"@) && 54 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("</main>"@), inputs);
}

pub proof fn f054(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("</main>"@), inputs, 0, 0),
        u::copy_registry()[54] == u::lit("</main>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l054(inputs);
    reveal_strlit("</main>");
    is_ascii_chars_encode_utf8("</main>"@);
    assert(u::lit("</main>"@) =~= seq![60u8, 47, 109, 97, 105, 110, 62]);
    assert(cx::scan(seq![60u8, 47, 109, 97, 105, 110, 62], 0) == 0) by (compute_only);
    cx::exact(u::lit("</main>"@), 0);
    h::fixed(u::lit("</main>"@), inputs, 0, 0);
}

pub proof fn l055(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(
            u::lit(
                "<footer class=\"scope\"><p>This page reports what the loaded guideline documents state. It does not give clinical advice.</p></footer>"@,
            ),
        ),
        u::copy_derived(
            u::lit(
                "<footer class=\"scope\"><p>This page reports what the loaded guideline documents state. It does not give clinical advice.</p></footer>"@,
            ),
            inputs,
            u::copy_registry(),
        ),
        u::copy_registry()[55] == u::lit(
            "<footer class=\"scope\"><p>This page reports what the loaded guideline documents state. It does not give clinical advice.</p></footer>"@,
        ),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[55] == u::lit(
        "<footer class=\"scope\"><p>This page reports what the loaded guideline documents state. It does not give clinical advice.</p></footer>"@,
    ) && 55 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(
        u::lit(
            "<footer class=\"scope\"><p>This page reports what the loaded guideline documents state. It does not give clinical advice.</p></footer>"@,
        ),
        inputs,
    );
}

pub proof fn f055(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(
            u::fixed(
                "<footer class=\"scope\"><p>This page reports what the loaded guideline documents state. It does not give clinical advice.</p></footer>"@,
            ),
            inputs,
            0,
            0,
        ),
        u::copy_registry()[55] == u::lit(
            "<footer class=\"scope\"><p>This page reports what the loaded guideline documents state. It does not give clinical advice.</p></footer>"@,
        ),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l055(inputs);
    reveal_strlit(
        "<footer class=\"scope\"><p>This page reports what the loaded guideline documents state. It does not give clinical advice.</p></footer>",
    );
    is_ascii_chars_encode_utf8(
        "<footer class=\"scope\"><p>This page reports what the loaded guideline documents state. It does not give clinical advice.</p></footer>"@,
    );
    assert(u::lit(
        "<footer class=\"scope\"><p>This page reports what the loaded guideline documents state. It does not give clinical advice.</p></footer>"@,
    ) =~= seq![
        60u8,
        102,
        111,
        111,
        116,
        101,
        114,
        32,
        99,
        108,
        97,
        115,
        115,
        61,
        34,
        115,
        99,
        111,
        112,
        101,
        34,
        62,
        60,
        112,
        62,
        84,
        104,
        105,
        115,
        32,
        112,
        97,
        103,
        101,
        32,
        114,
        101,
        112,
        111,
        114,
        116,
        115,
        32,
        119,
        104,
        97,
        116,
        32,
        116,
        104,
        101,
        32,
        108,
        111,
        97,
        100,
        101,
        100,
        32,
        103,
        117,
        105,
        100,
        101,
        108,
        105,
        110,
        101,
        32,
        100,
        111,
        99,
        117,
        109,
        101,
        110,
        116,
        115,
        32,
        115,
        116,
        97,
        116,
        101,
        46,
        32,
        73,
        116,
        32,
        100,
        111,
        101,
        115,
        32,
        110,
        111,
        116,
        32,
        103,
        105,
        118,
        101,
        32,
        99,
        108,
        105,
        110,
        105,
        99,
        97,
        108,
        32,
        97,
        100,
        118,
        105,
        99,
        101,
        46,
        60,
        47,
        112,
        62,
        60,
        47,
        102,
        111,
        111,
        116,
        101,
        114,
        62,
    ]);
    assert(cx::scan(
        seq![
            60u8,
            102,
            111,
            111,
            116,
            101,
            114,
            32,
            99,
            108,
            97,
            115,
            115,
            61,
            34,
            115,
            99,
            111,
            112,
            101,
            34,
            62,
            60,
            112,
            62,
            84,
            104,
            105,
            115,
            32,
            112,
            97,
            103,
            101,
            32,
            114,
            101,
            112,
            111,
            114,
            116,
            115,
            32,
            119,
            104,
            97,
            116,
            32,
            116,
            104,
            101,
            32,
            108,
            111,
            97,
            100,
            101,
            100,
            32,
            103,
            117,
            105,
            100,
            101,
            108,
            105,
            110,
            101,
            32,
            100,
            111,
            99,
            117,
            109,
            101,
            110,
            116,
            115,
            32,
            115,
            116,
            97,
            116,
            101,
            46,
            32,
            73,
            116,
            32,
            100,
            111,
            101,
            115,
            32,
            110,
            111,
            116,
            32,
            103,
            105,
            118,
            101,
            32,
            99,
            108,
            105,
            110,
            105,
            99,
            97,
            108,
            32,
            97,
            100,
            118,
            105,
            99,
            101,
            46,
            60,
            47,
            112,
            62,
            60,
            47,
            102,
            111,
            111,
            116,
            101,
            114,
            62,
        ],
        0,
    ) == 0) by (compute_only);
    cx::exact(
        u::lit(
            "<footer class=\"scope\"><p>This page reports what the loaded guideline documents state. It does not give clinical advice.</p></footer>"@,
        ),
        0,
    );
    h::fixed(
        u::lit(
            "<footer class=\"scope\"><p>This page reports what the loaded guideline documents state. It does not give clinical advice.</p></footer>"@,
        ),
        inputs,
        0,
        0,
    );
}

pub proof fn l056(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("</body>"@)),
        u::copy_derived(u::lit("</body>"@), inputs, u::copy_registry()),
        u::copy_registry()[56] == u::lit("</body>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[56] == u::lit("</body>"@) && 56 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("</body>"@), inputs);
}

pub proof fn f056(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("</body>"@), inputs, 0, 0),
        u::copy_registry()[56] == u::lit("</body>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l056(inputs);
    reveal_strlit("</body>");
    is_ascii_chars_encode_utf8("</body>"@);
    assert(u::lit("</body>"@) =~= seq![60u8, 47, 98, 111, 100, 121, 62]);
    assert(cx::scan(seq![60u8, 47, 98, 111, 100, 121, 62], 0) == 0) by (compute_only);
    cx::exact(u::lit("</body>"@), 0);
    h::fixed(u::lit("</body>"@), inputs, 0, 0);
}

pub proof fn l057(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("</html>"@)),
        u::copy_derived(u::lit("</html>"@), inputs, u::copy_registry()),
        u::copy_registry()[57] == u::lit("</html>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[57] == u::lit("</html>"@) && 57 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("</html>"@), inputs);
}

pub proof fn f057(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("</html>"@), inputs, 0, 0),
        u::copy_registry()[57] == u::lit("</html>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l057(inputs);
    reveal_strlit("</html>");
    is_ascii_chars_encode_utf8("</html>"@);
    assert(u::lit("</html>"@) =~= seq![60u8, 47, 104, 116, 109, 108, 62]);
    assert(cx::scan(seq![60u8, 47, 104, 116, 109, 108, 62], 0) == 0) by (compute_only);
    cx::exact(u::lit("</html>"@), 0);
    h::fixed(u::lit("</html>"@), inputs, 0, 0);
}

pub proof fn l058(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit(" approved"@)),
        u::copy_derived(u::lit(" approved"@), inputs, u::copy_registry()),
        u::copy_registry()[58] == u::lit(" approved"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[58] == u::lit(" approved"@) && 58 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit(" approved"@), inputs);
}

pub proof fn l059(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit(" rejected"@)),
        u::copy_derived(u::lit(" rejected"@), inputs, u::copy_registry()),
        u::copy_registry()[59] == u::lit(" rejected"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[59] == u::lit(" rejected"@) && 59 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit(" rejected"@), inputs);
}

pub proof fn l060(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit(" earlier"@)),
        u::copy_derived(u::lit(" earlier"@), inputs, u::copy_registry()),
        u::copy_registry()[60] == u::lit(" earlier"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[60] == u::lit(" earlier"@) && 60 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit(" earlier"@), inputs);
}

pub proof fn l061(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("None"@)),
        u::copy_derived(u::lit("None"@), inputs, u::copy_registry()),
        u::copy_registry()[61] == u::lit("None"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[61] == u::lit("None"@) && 61 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("None"@), inputs);
}

pub proof fn l062(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit(", "@)),
        u::copy_derived(u::lit(", "@), inputs, u::copy_registry()),
        u::copy_registry()[62] == u::lit(", "@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[62] == u::lit(", "@) && 62 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit(", "@), inputs);
}

pub proof fn l063(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("Decisions on this version: "@)),
        u::copy_derived(u::lit("Decisions on this version: "@), inputs, u::copy_registry()),
        u::copy_registry()[63] == u::lit("Decisions on this version: "@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[63] == u::lit("Decisions on this version: "@) && 63
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("Decisions on this version: "@), inputs);
}

pub proof fn l064(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit(" and "@)),
        u::copy_derived(u::lit(" and "@), inputs, u::copy_registry()),
        u::copy_registry()[64] == u::lit(" and "@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[64] == u::lit(" and "@) && 64 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit(" and "@), inputs);
}

pub proof fn l065(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("."@)),
        u::copy_derived(u::lit("."@), inputs, u::copy_registry()),
        u::copy_registry()[65] == u::lit("."@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[65] == u::lit("."@) && 65 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("."@), inputs);
}

pub proof fn l066(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("No decision is recorded on this version."@)),
        u::copy_derived(
            u::lit("No decision is recorded on this version."@),
            inputs,
            u::copy_registry(),
        ),
        u::copy_registry()[66] == u::lit("No decision is recorded on this version."@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[66] == u::lit("No decision is recorded on this version."@) && 66
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("No decision is recorded on this version."@), inputs);
}

pub proof fn l067(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("No decision is recorded."@)),
        u::copy_derived(u::lit("No decision is recorded."@), inputs, u::copy_registry()),
        u::copy_registry()[67] == u::lit("No decision is recorded."@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[67] == u::lit("No decision is recorded."@) && 67
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("No decision is recorded."@), inputs);
}

pub proof fn l068(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit(" Decisions on earlier versions: "@)),
        u::copy_derived(u::lit(" Decisions on earlier versions: "@), inputs, u::copy_registry()),
        u::copy_registry()[68] == u::lit(" Decisions on earlier versions: "@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[68] == u::lit(" Decisions on earlier versions: "@) && 68
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit(" Decisions on earlier versions: "@), inputs);
}

pub proof fn l069(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("No decisions are recorded for the "@)),
        u::copy_derived(u::lit("No decisions are recorded for the "@), inputs, u::copy_registry()),
        u::copy_registry()[69] == u::lit("No decisions are recorded for the "@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[69] == u::lit("No decisions are recorded for the "@) && 69
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("No decisions are recorded for the "@), inputs);
}

pub proof fn l070(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit(" documents in this guideline."@)),
        u::copy_derived(u::lit(" documents in this guideline."@), inputs, u::copy_registry()),
        u::copy_registry()[70] == u::lit(" documents in this guideline."@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[70] == u::lit(" documents in this guideline."@) && 70
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit(" documents in this guideline."@), inputs);
}

pub proof fn l071(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("Reviewers recorded "@)),
        u::copy_derived(u::lit("Reviewers recorded "@), inputs, u::copy_registry()),
        u::copy_registry()[71] == u::lit("Reviewers recorded "@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[71] == u::lit("Reviewers recorded "@) && 71
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("Reviewers recorded "@), inputs);
}

pub proof fn l072(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit(" decisions on "@)),
        u::copy_derived(u::lit(" decisions on "@), inputs, u::copy_registry()),
        u::copy_registry()[72] == u::lit(" decisions on "@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[72] == u::lit(" decisions on "@) && 72 < u::copy_registry().len())
        by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit(" decisions on "@), inputs);
}

pub proof fn l073(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit(" of "@)),
        u::copy_derived(u::lit(" of "@), inputs, u::copy_registry()),
        u::copy_registry()[73] == u::lit(" of "@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[73] == u::lit(" of "@) && 73 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit(" of "@), inputs);
}

pub proof fn l074(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit(" documents."@)),
        u::copy_derived(u::lit(" documents."@), inputs, u::copy_registry()),
        u::copy_registry()[74] == u::lit(" documents."@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[74] == u::lit(" documents."@) && 74 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit(" documents."@), inputs);
}

pub proof fn l075(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("g/"@)),
        u::copy_derived(u::lit("g/"@), inputs, u::copy_registry()),
        u::copy_registry()[75] == u::lit("g/"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[75] == u::lit("g/"@) && 75 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("g/"@), inputs);
}

pub proof fn l076(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("/index.html"@)),
        u::copy_derived(u::lit("/index.html"@), inputs, u::copy_registry()),
        u::copy_registry()[76] == u::lit("/index.html"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[76] == u::lit("/index.html"@) && 76 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("/index.html"@), inputs);
}

pub proof fn l077(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("Guidelines"@)),
        u::copy_derived(u::lit("Guidelines"@), inputs, u::copy_registry()),
        u::copy_registry()[77] == u::lit("Guidelines"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[77] == u::lit("Guidelines"@) && 77 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("Guidelines"@), inputs);
}

pub proof fn l078(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("cnl-ckc reviewer"@)),
        u::copy_derived(u::lit("cnl-ckc reviewer"@), inputs, u::copy_registry()),
        u::copy_registry()[78] == u::lit("cnl-ckc reviewer"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[78] == u::lit("cnl-ckc reviewer"@) && 78 < u::copy_registry().len())
        by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("cnl-ckc reviewer"@), inputs);
}

pub proof fn f078(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("cnl-ckc reviewer"@), inputs, 0, 0),
        u::copy_registry()[78] == u::lit("cnl-ckc reviewer"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l078(inputs);
    reveal_strlit("cnl-ckc reviewer");
    is_ascii_chars_encode_utf8("cnl-ckc reviewer"@);
    assert(u::lit("cnl-ckc reviewer"@) =~= seq![
        99u8,
        110,
        108,
        45,
        99,
        107,
        99,
        32,
        114,
        101,
        118,
        105,
        101,
        119,
        101,
        114,
    ]);
    assert(cx::scan(
        seq![99u8, 110, 108, 45, 99, 107, 99, 32, 114, 101, 118, 105, 101, 119, 101, 114],
        0,
    ) == 0) by (compute_only);
    cx::exact(u::lit("cnl-ckc reviewer"@), 0);
    h::fixed(u::lit("cnl-ckc reviewer"@), inputs, 0, 0);
}

pub proof fn l079(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<h1>Guidelines</h1>"@)),
        u::copy_derived(u::lit("<h1>Guidelines</h1>"@), inputs, u::copy_registry()),
        u::copy_registry()[79] == u::lit("<h1>Guidelines</h1>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[79] == u::lit("<h1>Guidelines</h1>"@) && 79
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<h1>Guidelines</h1>"@), inputs);
}

pub proof fn f079(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<h1>Guidelines</h1>"@), inputs, 0, 0),
        u::copy_registry()[79] == u::lit("<h1>Guidelines</h1>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l079(inputs);
    reveal_strlit("<h1>Guidelines</h1>");
    is_ascii_chars_encode_utf8("<h1>Guidelines</h1>"@);
    assert(u::lit("<h1>Guidelines</h1>"@) =~= seq![
        60u8,
        104,
        49,
        62,
        71,
        117,
        105,
        100,
        101,
        108,
        105,
        110,
        101,
        115,
        60,
        47,
        104,
        49,
        62,
    ]);
    assert(cx::scan(
        seq![
            60u8,
            104,
            49,
            62,
            71,
            117,
            105,
            100,
            101,
            108,
            105,
            110,
            101,
            115,
            60,
            47,
            104,
            49,
            62,
        ],
        0,
    ) == 0) by (compute_only);
    cx::exact(u::lit("<h1>Guidelines</h1>"@), 0);
    h::fixed(u::lit("<h1>Guidelines</h1>"@), inputs, 0, 0);
}

pub proof fn l080(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<section>"@)),
        u::copy_derived(u::lit("<section>"@), inputs, u::copy_registry()),
        u::copy_registry()[80] == u::lit("<section>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[80] == u::lit("<section>"@) && 80 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<section>"@), inputs);
}

pub proof fn f080(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<section>"@), inputs, 0, 0),
        u::copy_registry()[80] == u::lit("<section>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l080(inputs);
    reveal_strlit("<section>");
    is_ascii_chars_encode_utf8("<section>"@);
    assert(u::lit("<section>"@) =~= seq![60u8, 115, 101, 99, 116, 105, 111, 110, 62]);
    assert(cx::scan(seq![60u8, 115, 101, 99, 116, 105, 111, 110, 62], 0) == 0) by (compute_only);
    cx::exact(u::lit("<section>"@), 0);
    h::fixed(u::lit("<section>"@), inputs, 0, 0);
}

pub proof fn l081(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<table>"@)),
        u::copy_derived(u::lit("<table>"@), inputs, u::copy_registry()),
        u::copy_registry()[81] == u::lit("<table>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[81] == u::lit("<table>"@) && 81 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<table>"@), inputs);
}

pub proof fn f081(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<table>"@), inputs, 0, 0),
        u::copy_registry()[81] == u::lit("<table>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l081(inputs);
    reveal_strlit("<table>");
    is_ascii_chars_encode_utf8("<table>"@);
    assert(u::lit("<table>"@) =~= seq![60u8, 116, 97, 98, 108, 101, 62]);
    assert(cx::scan(seq![60u8, 116, 97, 98, 108, 101, 62], 0) == 0) by (compute_only);
    cx::exact(u::lit("<table>"@), 0);
    h::fixed(u::lit("<table>"@), inputs, 0, 0);
}

pub proof fn l082(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(
            u::lit(
                "<thead><tr><th>Guideline</th><th>Documents</th><th>Passages</th><th>Approved</th><th>Rejected</th><th>Contested</th><th>Outdated</th><th>Unreviewed</th></tr></thead>"@,
            ),
        ),
        u::copy_derived(
            u::lit(
                "<thead><tr><th>Guideline</th><th>Documents</th><th>Passages</th><th>Approved</th><th>Rejected</th><th>Contested</th><th>Outdated</th><th>Unreviewed</th></tr></thead>"@,
            ),
            inputs,
            u::copy_registry(),
        ),
        u::copy_registry()[82] == u::lit(
            "<thead><tr><th>Guideline</th><th>Documents</th><th>Passages</th><th>Approved</th><th>Rejected</th><th>Contested</th><th>Outdated</th><th>Unreviewed</th></tr></thead>"@,
        ),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[82] == u::lit(
        "<thead><tr><th>Guideline</th><th>Documents</th><th>Passages</th><th>Approved</th><th>Rejected</th><th>Contested</th><th>Outdated</th><th>Unreviewed</th></tr></thead>"@,
    ) && 82 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(
        u::lit(
            "<thead><tr><th>Guideline</th><th>Documents</th><th>Passages</th><th>Approved</th><th>Rejected</th><th>Contested</th><th>Outdated</th><th>Unreviewed</th></tr></thead>"@,
        ),
        inputs,
    );
}

pub proof fn f082(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(
            u::fixed(
                "<thead><tr><th>Guideline</th><th>Documents</th><th>Passages</th><th>Approved</th><th>Rejected</th><th>Contested</th><th>Outdated</th><th>Unreviewed</th></tr></thead>"@,
            ),
            inputs,
            0,
            0,
        ),
        u::copy_registry()[82] == u::lit(
            "<thead><tr><th>Guideline</th><th>Documents</th><th>Passages</th><th>Approved</th><th>Rejected</th><th>Contested</th><th>Outdated</th><th>Unreviewed</th></tr></thead>"@,
        ),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l082(inputs);
    reveal_strlit(
        "<thead><tr><th>Guideline</th><th>Documents</th><th>Passages</th><th>Approved</th><th>Rejected</th><th>Contested</th><th>Outdated</th><th>Unreviewed</th></tr></thead>",
    );
    is_ascii_chars_encode_utf8(
        "<thead><tr><th>Guideline</th><th>Documents</th><th>Passages</th><th>Approved</th><th>Rejected</th><th>Contested</th><th>Outdated</th><th>Unreviewed</th></tr></thead>"@,
    );
    assert(u::lit(
        "<thead><tr><th>Guideline</th><th>Documents</th><th>Passages</th><th>Approved</th><th>Rejected</th><th>Contested</th><th>Outdated</th><th>Unreviewed</th></tr></thead>"@,
    ) =~= seq![
        60u8,
        116,
        104,
        101,
        97,
        100,
        62,
        60,
        116,
        114,
        62,
        60,
        116,
        104,
        62,
        71,
        117,
        105,
        100,
        101,
        108,
        105,
        110,
        101,
        60,
        47,
        116,
        104,
        62,
        60,
        116,
        104,
        62,
        68,
        111,
        99,
        117,
        109,
        101,
        110,
        116,
        115,
        60,
        47,
        116,
        104,
        62,
        60,
        116,
        104,
        62,
        80,
        97,
        115,
        115,
        97,
        103,
        101,
        115,
        60,
        47,
        116,
        104,
        62,
        60,
        116,
        104,
        62,
        65,
        112,
        112,
        114,
        111,
        118,
        101,
        100,
        60,
        47,
        116,
        104,
        62,
        60,
        116,
        104,
        62,
        82,
        101,
        106,
        101,
        99,
        116,
        101,
        100,
        60,
        47,
        116,
        104,
        62,
        60,
        116,
        104,
        62,
        67,
        111,
        110,
        116,
        101,
        115,
        116,
        101,
        100,
        60,
        47,
        116,
        104,
        62,
        60,
        116,
        104,
        62,
        79,
        117,
        116,
        100,
        97,
        116,
        101,
        100,
        60,
        47,
        116,
        104,
        62,
        60,
        116,
        104,
        62,
        85,
        110,
        114,
        101,
        118,
        105,
        101,
        119,
        101,
        100,
        60,
        47,
        116,
        104,
        62,
        60,
        47,
        116,
        114,
        62,
        60,
        47,
        116,
        104,
        101,
        97,
        100,
        62,
    ]);
    assert(cx::scan(
        seq![
            60u8,
            116,
            104,
            101,
            97,
            100,
            62,
            60,
            116,
            114,
            62,
            60,
            116,
            104,
            62,
            71,
            117,
            105,
            100,
            101,
            108,
            105,
            110,
            101,
            60,
            47,
            116,
            104,
            62,
            60,
            116,
            104,
            62,
            68,
            111,
            99,
            117,
            109,
            101,
            110,
            116,
            115,
            60,
            47,
            116,
            104,
            62,
            60,
            116,
            104,
            62,
            80,
            97,
            115,
            115,
            97,
            103,
            101,
            115,
            60,
            47,
            116,
            104,
            62,
            60,
            116,
            104,
            62,
            65,
            112,
            112,
            114,
            111,
            118,
            101,
            100,
            60,
            47,
            116,
            104,
            62,
            60,
            116,
            104,
            62,
            82,
            101,
            106,
            101,
            99,
            116,
            101,
            100,
            60,
            47,
            116,
            104,
            62,
            60,
            116,
            104,
            62,
            67,
            111,
            110,
            116,
            101,
            115,
            116,
            101,
            100,
            60,
            47,
            116,
            104,
            62,
            60,
            116,
            104,
            62,
            79,
            117,
            116,
            100,
            97,
            116,
            101,
            100,
            60,
            47,
            116,
            104,
            62,
            60,
            116,
            104,
            62,
            85,
            110,
            114,
            101,
            118,
            105,
            101,
            119,
            101,
            100,
            60,
            47,
            116,
            104,
            62,
            60,
            47,
            116,
            114,
            62,
            60,
            47,
            116,
            104,
            101,
            97,
            100,
            62,
        ],
        0,
    ) == 0) by (compute_only);
    cx::exact(
        u::lit(
            "<thead><tr><th>Guideline</th><th>Documents</th><th>Passages</th><th>Approved</th><th>Rejected</th><th>Contested</th><th>Outdated</th><th>Unreviewed</th></tr></thead>"@,
        ),
        0,
    );
    h::fixed(
        u::lit(
            "<thead><tr><th>Guideline</th><th>Documents</th><th>Passages</th><th>Approved</th><th>Rejected</th><th>Contested</th><th>Outdated</th><th>Unreviewed</th></tr></thead>"@,
        ),
        inputs,
        0,
        0,
    );
}

pub proof fn l083(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<tbody>"@)),
        u::copy_derived(u::lit("<tbody>"@), inputs, u::copy_registry()),
        u::copy_registry()[83] == u::lit("<tbody>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[83] == u::lit("<tbody>"@) && 83 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<tbody>"@), inputs);
}

pub proof fn f083(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<tbody>"@), inputs, 0, 0),
        u::copy_registry()[83] == u::lit("<tbody>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l083(inputs);
    reveal_strlit("<tbody>");
    is_ascii_chars_encode_utf8("<tbody>"@);
    assert(u::lit("<tbody>"@) =~= seq![60u8, 116, 98, 111, 100, 121, 62]);
    assert(cx::scan(seq![60u8, 116, 98, 111, 100, 121, 62], 0) == 0) by (compute_only);
    cx::exact(u::lit("<tbody>"@), 0);
    h::fixed(u::lit("<tbody>"@), inputs, 0, 0);
}

pub proof fn l084(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("</tbody>"@)),
        u::copy_derived(u::lit("</tbody>"@), inputs, u::copy_registry()),
        u::copy_registry()[84] == u::lit("</tbody>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[84] == u::lit("</tbody>"@) && 84 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("</tbody>"@), inputs);
}

pub proof fn f084(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("</tbody>"@), inputs, 0, 0),
        u::copy_registry()[84] == u::lit("</tbody>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l084(inputs);
    reveal_strlit("</tbody>");
    is_ascii_chars_encode_utf8("</tbody>"@);
    assert(u::lit("</tbody>"@) =~= seq![60u8, 47, 116, 98, 111, 100, 121, 62]);
    assert(cx::scan(seq![60u8, 47, 116, 98, 111, 100, 121, 62], 0) == 0) by (compute_only);
    cx::exact(u::lit("</tbody>"@), 0);
    h::fixed(u::lit("</tbody>"@), inputs, 0, 0);
}

pub proof fn l085(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("</table>"@)),
        u::copy_derived(u::lit("</table>"@), inputs, u::copy_registry()),
        u::copy_registry()[85] == u::lit("</table>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[85] == u::lit("</table>"@) && 85 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("</table>"@), inputs);
}

pub proof fn f085(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("</table>"@), inputs, 0, 0),
        u::copy_registry()[85] == u::lit("</table>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l085(inputs);
    reveal_strlit("</table>");
    is_ascii_chars_encode_utf8("</table>"@);
    assert(u::lit("</table>"@) =~= seq![60u8, 47, 116, 97, 98, 108, 101, 62]);
    assert(cx::scan(seq![60u8, 47, 116, 97, 98, 108, 101, 62], 0) == 0) by (compute_only);
    cx::exact(u::lit("</table>"@), 0);
    h::fixed(u::lit("</table>"@), inputs, 0, 0);
}

pub proof fn l086(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("</section>"@)),
        u::copy_derived(u::lit("</section>"@), inputs, u::copy_registry()),
        u::copy_registry()[86] == u::lit("</section>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[86] == u::lit("</section>"@) && 86 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("</section>"@), inputs);
}

pub proof fn f086(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("</section>"@), inputs, 0, 0),
        u::copy_registry()[86] == u::lit("</section>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l086(inputs);
    reveal_strlit("</section>");
    is_ascii_chars_encode_utf8("</section>"@);
    assert(u::lit("</section>"@) =~= seq![60u8, 47, 115, 101, 99, 116, 105, 111, 110, 62]);
    assert(cx::scan(seq![60u8, 47, 115, 101, 99, 116, 105, 111, 110, 62], 0) == 0)
        by (compute_only);
    cx::exact(u::lit("</section>"@), 0);
    h::fixed(u::lit("</section>"@), inputs, 0, 0);
}

pub proof fn l087(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("Restates "@)),
        u::copy_derived(u::lit("Restates "@), inputs, u::copy_registry()),
        u::copy_registry()[87] == u::lit("Restates "@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[87] == u::lit("Restates "@) && 87 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("Restates "@), inputs);
}

pub proof fn l088(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("restates("@)),
        u::copy_derived(u::lit("restates("@), inputs, u::copy_registry()),
        u::copy_registry()[88] == u::lit("restates("@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[88] == u::lit("restates("@) && 88 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("restates("@), inputs);
}

pub proof fn l089(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("uncovered("@)),
        u::copy_derived(u::lit("uncovered("@), inputs, u::copy_registry()),
        u::copy_registry()[89] == u::lit("uncovered("@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[89] == u::lit("uncovered("@) && 89 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("uncovered("@), inputs);
}

pub proof fn l090(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit(": "@)),
        u::copy_derived(u::lit(": "@), inputs, u::copy_registry()),
        u::copy_registry()[90] == u::lit(": "@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[90] == u::lit(": "@) && 90 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit(": "@), inputs);
}

pub proof fn l091(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("Not covered — "@)),
        u::copy_derived(u::lit("Not covered — "@), inputs, u::copy_registry()),
        u::copy_registry()[91] == u::lit("Not covered — "@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[91] == u::lit("Not covered — "@) && 91 < u::copy_registry().len())
        by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("Not covered — "@), inputs);
}

pub proof fn l092(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("Pending"@)),
        u::copy_derived(u::lit("Pending"@), inputs, u::copy_registry()),
        u::copy_registry()[92] == u::lit("Pending"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[92] == u::lit("Pending"@) && 92 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("Pending"@), inputs);
}

pub proof fn l093(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("Passages"@)),
        u::copy_derived(u::lit("Passages"@), inputs, u::copy_registry()),
        u::copy_registry()[93] == u::lit("Passages"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[93] == u::lit("Passages"@) && 93 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("Passages"@), inputs);
}

pub proof fn l094(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("With ACE"@)),
        u::copy_derived(u::lit("With ACE"@), inputs, u::copy_registry()),
        u::copy_registry()[94] == u::lit("With ACE"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[94] == u::lit("With ACE"@) && 94 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("With ACE"@), inputs);
}

pub proof fn l095(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("doc/"@)),
        u::copy_derived(u::lit("doc/"@), inputs, u::copy_registry()),
        u::copy_registry()[95] == u::lit("doc/"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[95] == u::lit("doc/"@) && 95 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("doc/"@), inputs);
}

pub proof fn l096(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit(".html"@)),
        u::copy_derived(u::lit(".html"@), inputs, u::copy_registry()),
        u::copy_registry()[96] == u::lit(".html"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[96] == u::lit(".html"@) && 96 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit(".html"@), inputs);
}

pub proof fn l097(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<h1>"@)),
        u::copy_derived(u::lit("<h1>"@), inputs, u::copy_registry()),
        u::copy_registry()[97] == u::lit("<h1>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[97] == u::lit("<h1>"@) && 97 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<h1>"@), inputs);
}

pub proof fn f097(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<h1>"@), inputs, 0, 0),
        u::copy_registry()[97] == u::lit("<h1>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l097(inputs);
    reveal_strlit("<h1>");
    is_ascii_chars_encode_utf8("<h1>"@);
    assert(u::lit("<h1>"@) =~= seq![60u8, 104, 49, 62]);
    assert(cx::scan(seq![60u8, 104, 49, 62], 0) == 0) by (compute_only);
    cx::exact(u::lit("<h1>"@), 0);
    h::fixed(u::lit("<h1>"@), inputs, 0, 0);
}

pub proof fn l098(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("</h1>"@)),
        u::copy_derived(u::lit("</h1>"@), inputs, u::copy_registry()),
        u::copy_registry()[98] == u::lit("</h1>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[98] == u::lit("</h1>"@) && 98 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("</h1>"@), inputs);
}

pub proof fn f098(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("</h1>"@), inputs, 0, 0),
        u::copy_registry()[98] == u::lit("</h1>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l098(inputs);
    reveal_strlit("</h1>");
    is_ascii_chars_encode_utf8("</h1>"@);
    assert(u::lit("</h1>"@) =~= seq![60u8, 47, 104, 49, 62]);
    assert(cx::scan(seq![60u8, 47, 104, 49, 62], 0) == 0) by (compute_only);
    cx::exact(u::lit("</h1>"@), 0);
    h::fixed(u::lit("</h1>"@), inputs, 0, 0);
}

pub proof fn l099(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<p>"@)),
        u::copy_derived(u::lit("<p>"@), inputs, u::copy_registry()),
        u::copy_registry()[99] == u::lit("<p>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[99] == u::lit("<p>"@) && 99 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<p>"@), inputs);
}

pub proof fn f099(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<p>"@), inputs, 0, 0),
        u::copy_registry()[99] == u::lit("<p>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l099(inputs);
    reveal_strlit("<p>");
    is_ascii_chars_encode_utf8("<p>"@);
    assert(u::lit("<p>"@) =~= seq![60u8, 112, 62]);
    assert(cx::scan(seq![60u8, 112, 62], 0) == 0) by (compute_only);
    cx::exact(u::lit("<p>"@), 0);
    h::fixed(u::lit("<p>"@), inputs, 0, 0);
}

pub proof fn l100(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(
            u::lit(" <a href=\"records.html\">All decision records</a></p>"@),
        ),
        u::copy_derived(
            u::lit(" <a href=\"records.html\">All decision records</a></p>"@),
            inputs,
            u::copy_registry(),
        ),
        u::copy_registry()[100] == u::lit(
            " <a href=\"records.html\">All decision records</a></p>"@,
        ),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[100] == u::lit(
        " <a href=\"records.html\">All decision records</a></p>"@,
    ) && 100 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit(" <a href=\"records.html\">All decision records</a></p>"@), inputs);
}

pub proof fn f100(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(
            u::fixed(" <a href=\"records.html\">All decision records</a></p>"@),
            inputs,
            0,
            0,
        ),
        u::copy_registry()[100] == u::lit(
            " <a href=\"records.html\">All decision records</a></p>"@,
        ),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l100(inputs);
    reveal_strlit(" <a href=\"records.html\">All decision records</a></p>");
    is_ascii_chars_encode_utf8(" <a href=\"records.html\">All decision records</a></p>"@);
    assert(u::lit(" <a href=\"records.html\">All decision records</a></p>"@) =~= seq![
        32u8,
        60,
        97,
        32,
        104,
        114,
        101,
        102,
        61,
        34,
        114,
        101,
        99,
        111,
        114,
        100,
        115,
        46,
        104,
        116,
        109,
        108,
        34,
        62,
        65,
        108,
        108,
        32,
        100,
        101,
        99,
        105,
        115,
        105,
        111,
        110,
        32,
        114,
        101,
        99,
        111,
        114,
        100,
        115,
        60,
        47,
        97,
        62,
        60,
        47,
        112,
        62,
    ]);
    assert(cx::scan(
        seq![
            32u8,
            60,
            97,
            32,
            104,
            114,
            101,
            102,
            61,
            34,
            114,
            101,
            99,
            111,
            114,
            100,
            115,
            46,
            104,
            116,
            109,
            108,
            34,
            62,
            65,
            108,
            108,
            32,
            100,
            101,
            99,
            105,
            115,
            105,
            111,
            110,
            32,
            114,
            101,
            99,
            111,
            114,
            100,
            115,
            60,
            47,
            97,
            62,
            60,
            47,
            112,
            62,
        ],
        0,
    ) == 0) by (compute_only);
    cx::exact(u::lit(" <a href=\"records.html\">All decision records</a></p>"@), 0);
    h::fixed(u::lit(" <a href=\"records.html\">All decision records</a></p>"@), inputs, 0, 0);
}

pub proof fn l101(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<h2>Status</h2>"@)),
        u::copy_derived(u::lit("<h2>Status</h2>"@), inputs, u::copy_registry()),
        u::copy_registry()[101] == u::lit("<h2>Status</h2>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[101] == u::lit("<h2>Status</h2>"@) && 101 < u::copy_registry().len())
        by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<h2>Status</h2>"@), inputs);
}

pub proof fn f101(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<h2>Status</h2>"@), inputs, 0, 0),
        u::copy_registry()[101] == u::lit("<h2>Status</h2>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l101(inputs);
    reveal_strlit("<h2>Status</h2>");
    is_ascii_chars_encode_utf8("<h2>Status</h2>"@);
    assert(u::lit("<h2>Status</h2>"@) =~= seq![
        60u8,
        104,
        50,
        62,
        83,
        116,
        97,
        116,
        117,
        115,
        60,
        47,
        104,
        50,
        62,
    ]);
    assert(cx::scan(seq![60u8, 104, 50, 62, 83, 116, 97, 116, 117, 115, 60, 47, 104, 50, 62], 0)
        == 0) by (compute_only);
    cx::exact(u::lit("<h2>Status</h2>"@), 0);
    h::fixed(u::lit("<h2>Status</h2>"@), inputs, 0, 0);
}

pub proof fn l102(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<table class=\"compact\">"@)),
        u::copy_derived(u::lit("<table class=\"compact\">"@), inputs, u::copy_registry()),
        u::copy_registry()[102] == u::lit("<table class=\"compact\">"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[102] == u::lit("<table class=\"compact\">"@) && 102
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<table class=\"compact\">"@), inputs);
}

pub proof fn f102(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<table class=\"compact\">"@), inputs, 0, 0),
        u::copy_registry()[102] == u::lit("<table class=\"compact\">"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l102(inputs);
    reveal_strlit("<table class=\"compact\">");
    is_ascii_chars_encode_utf8("<table class=\"compact\">"@);
    assert(u::lit("<table class=\"compact\">"@) =~= seq![
        60u8,
        116,
        97,
        98,
        108,
        101,
        32,
        99,
        108,
        97,
        115,
        115,
        61,
        34,
        99,
        111,
        109,
        112,
        97,
        99,
        116,
        34,
        62,
    ]);
    assert(cx::scan(
        seq![
            60u8,
            116,
            97,
            98,
            108,
            101,
            32,
            99,
            108,
            97,
            115,
            115,
            61,
            34,
            99,
            111,
            109,
            112,
            97,
            99,
            116,
            34,
            62,
        ],
        0,
    ) == 0) by (compute_only);
    cx::exact(u::lit("<table class=\"compact\">"@), 0);
    h::fixed(u::lit("<table class=\"compact\">"@), inputs, 0, 0);
}

pub proof fn l103(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(
            u::lit("<thead><tr><th>Status</th><th>Count</th></tr></thead>"@),
        ),
        u::copy_derived(
            u::lit("<thead><tr><th>Status</th><th>Count</th></tr></thead>"@),
            inputs,
            u::copy_registry(),
        ),
        u::copy_registry()[103] == u::lit("<thead><tr><th>Status</th><th>Count</th></tr></thead>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[103] == u::lit(
        "<thead><tr><th>Status</th><th>Count</th></tr></thead>"@,
    ) && 103 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<thead><tr><th>Status</th><th>Count</th></tr></thead>"@), inputs);
}

pub proof fn f103(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(
            u::fixed("<thead><tr><th>Status</th><th>Count</th></tr></thead>"@),
            inputs,
            0,
            0,
        ),
        u::copy_registry()[103] == u::lit("<thead><tr><th>Status</th><th>Count</th></tr></thead>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l103(inputs);
    reveal_strlit("<thead><tr><th>Status</th><th>Count</th></tr></thead>");
    is_ascii_chars_encode_utf8("<thead><tr><th>Status</th><th>Count</th></tr></thead>"@);
    assert(u::lit("<thead><tr><th>Status</th><th>Count</th></tr></thead>"@) =~= seq![
        60u8,
        116,
        104,
        101,
        97,
        100,
        62,
        60,
        116,
        114,
        62,
        60,
        116,
        104,
        62,
        83,
        116,
        97,
        116,
        117,
        115,
        60,
        47,
        116,
        104,
        62,
        60,
        116,
        104,
        62,
        67,
        111,
        117,
        110,
        116,
        60,
        47,
        116,
        104,
        62,
        60,
        47,
        116,
        114,
        62,
        60,
        47,
        116,
        104,
        101,
        97,
        100,
        62,
    ]);
    assert(cx::scan(
        seq![
            60u8,
            116,
            104,
            101,
            97,
            100,
            62,
            60,
            116,
            114,
            62,
            60,
            116,
            104,
            62,
            83,
            116,
            97,
            116,
            117,
            115,
            60,
            47,
            116,
            104,
            62,
            60,
            116,
            104,
            62,
            67,
            111,
            117,
            110,
            116,
            60,
            47,
            116,
            104,
            62,
            60,
            47,
            116,
            114,
            62,
            60,
            47,
            116,
            104,
            101,
            97,
            100,
            62,
        ],
        0,
    ) == 0) by (compute_only);
    cx::exact(u::lit("<thead><tr><th>Status</th><th>Count</th></tr></thead>"@), 0);
    h::fixed(u::lit("<thead><tr><th>Status</th><th>Count</th></tr></thead>"@), inputs, 0, 0);
}

pub proof fn l104(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<h2>Documents</h2>"@)),
        u::copy_derived(u::lit("<h2>Documents</h2>"@), inputs, u::copy_registry()),
        u::copy_registry()[104] == u::lit("<h2>Documents</h2>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[104] == u::lit("<h2>Documents</h2>"@) && 104
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<h2>Documents</h2>"@), inputs);
}

pub proof fn f104(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<h2>Documents</h2>"@), inputs, 0, 0),
        u::copy_registry()[104] == u::lit("<h2>Documents</h2>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l104(inputs);
    reveal_strlit("<h2>Documents</h2>");
    is_ascii_chars_encode_utf8("<h2>Documents</h2>"@);
    assert(u::lit("<h2>Documents</h2>"@) =~= seq![
        60u8,
        104,
        50,
        62,
        68,
        111,
        99,
        117,
        109,
        101,
        110,
        116,
        115,
        60,
        47,
        104,
        50,
        62,
    ]);
    assert(cx::scan(
        seq![60u8, 104, 50, 62, 68, 111, 99, 117, 109, 101, 110, 116, 115, 60, 47, 104, 50, 62],
        0,
    ) == 0) by (compute_only);
    cx::exact(u::lit("<h2>Documents</h2>"@), 0);
    h::fixed(u::lit("<h2>Documents</h2>"@), inputs, 0, 0);
}

pub proof fn l105(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(
            u::lit(
                "<thead><tr><th>Document</th><th>Status</th><th>Decisions</th><th>Passage</th></tr></thead>"@,
            ),
        ),
        u::copy_derived(
            u::lit(
                "<thead><tr><th>Document</th><th>Status</th><th>Decisions</th><th>Passage</th></tr></thead>"@,
            ),
            inputs,
            u::copy_registry(),
        ),
        u::copy_registry()[105] == u::lit(
            "<thead><tr><th>Document</th><th>Status</th><th>Decisions</th><th>Passage</th></tr></thead>"@,
        ),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[105] == u::lit(
        "<thead><tr><th>Document</th><th>Status</th><th>Decisions</th><th>Passage</th></tr></thead>"@,
    ) && 105 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(
        u::lit(
            "<thead><tr><th>Document</th><th>Status</th><th>Decisions</th><th>Passage</th></tr></thead>"@,
        ),
        inputs,
    );
}

pub proof fn f105(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(
            u::fixed(
                "<thead><tr><th>Document</th><th>Status</th><th>Decisions</th><th>Passage</th></tr></thead>"@,
            ),
            inputs,
            0,
            0,
        ),
        u::copy_registry()[105] == u::lit(
            "<thead><tr><th>Document</th><th>Status</th><th>Decisions</th><th>Passage</th></tr></thead>"@,
        ),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l105(inputs);
    reveal_strlit(
        "<thead><tr><th>Document</th><th>Status</th><th>Decisions</th><th>Passage</th></tr></thead>",
    );
    is_ascii_chars_encode_utf8(
        "<thead><tr><th>Document</th><th>Status</th><th>Decisions</th><th>Passage</th></tr></thead>"@,
    );
    assert(u::lit(
        "<thead><tr><th>Document</th><th>Status</th><th>Decisions</th><th>Passage</th></tr></thead>"@,
    ) =~= seq![
        60u8,
        116,
        104,
        101,
        97,
        100,
        62,
        60,
        116,
        114,
        62,
        60,
        116,
        104,
        62,
        68,
        111,
        99,
        117,
        109,
        101,
        110,
        116,
        60,
        47,
        116,
        104,
        62,
        60,
        116,
        104,
        62,
        83,
        116,
        97,
        116,
        117,
        115,
        60,
        47,
        116,
        104,
        62,
        60,
        116,
        104,
        62,
        68,
        101,
        99,
        105,
        115,
        105,
        111,
        110,
        115,
        60,
        47,
        116,
        104,
        62,
        60,
        116,
        104,
        62,
        80,
        97,
        115,
        115,
        97,
        103,
        101,
        60,
        47,
        116,
        104,
        62,
        60,
        47,
        116,
        114,
        62,
        60,
        47,
        116,
        104,
        101,
        97,
        100,
        62,
    ]);
    assert(cx::scan(
        seq![
            60u8,
            116,
            104,
            101,
            97,
            100,
            62,
            60,
            116,
            114,
            62,
            60,
            116,
            104,
            62,
            68,
            111,
            99,
            117,
            109,
            101,
            110,
            116,
            60,
            47,
            116,
            104,
            62,
            60,
            116,
            104,
            62,
            83,
            116,
            97,
            116,
            117,
            115,
            60,
            47,
            116,
            104,
            62,
            60,
            116,
            104,
            62,
            68,
            101,
            99,
            105,
            115,
            105,
            111,
            110,
            115,
            60,
            47,
            116,
            104,
            62,
            60,
            116,
            104,
            62,
            80,
            97,
            115,
            115,
            97,
            103,
            101,
            60,
            47,
            116,
            104,
            62,
            60,
            47,
            116,
            114,
            62,
            60,
            47,
            116,
            104,
            101,
            97,
            100,
            62,
        ],
        0,
    ) == 0) by (compute_only);
    cx::exact(
        u::lit(
            "<thead><tr><th>Document</th><th>Status</th><th>Decisions</th><th>Passage</th></tr></thead>"@,
        ),
        0,
    );
    h::fixed(
        u::lit(
            "<thead><tr><th>Document</th><th>Status</th><th>Decisions</th><th>Passage</th></tr></thead>"@,
        ),
        inputs,
        0,
        0,
    );
}

pub proof fn l106(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<h2>Passages without ACE</h2>"@)),
        u::copy_derived(u::lit("<h2>Passages without ACE</h2>"@), inputs, u::copy_registry()),
        u::copy_registry()[106] == u::lit("<h2>Passages without ACE</h2>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[106] == u::lit("<h2>Passages without ACE</h2>"@) && 106
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<h2>Passages without ACE</h2>"@), inputs);
}

pub proof fn f106(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<h2>Passages without ACE</h2>"@), inputs, 0, 0),
        u::copy_registry()[106] == u::lit("<h2>Passages without ACE</h2>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l106(inputs);
    reveal_strlit("<h2>Passages without ACE</h2>");
    is_ascii_chars_encode_utf8("<h2>Passages without ACE</h2>"@);
    assert(u::lit("<h2>Passages without ACE</h2>"@) =~= seq![
        60u8,
        104,
        50,
        62,
        80,
        97,
        115,
        115,
        97,
        103,
        101,
        115,
        32,
        119,
        105,
        116,
        104,
        111,
        117,
        116,
        32,
        65,
        67,
        69,
        60,
        47,
        104,
        50,
        62,
    ]);
    assert(cx::scan(
        seq![
            60u8,
            104,
            50,
            62,
            80,
            97,
            115,
            115,
            97,
            103,
            101,
            115,
            32,
            119,
            105,
            116,
            104,
            111,
            117,
            116,
            32,
            65,
            67,
            69,
            60,
            47,
            104,
            50,
            62,
        ],
        0,
    ) == 0) by (compute_only);
    cx::exact(u::lit("<h2>Passages without ACE</h2>"@), 0);
    h::fixed(u::lit("<h2>Passages without ACE</h2>"@), inputs, 0, 0);
}

pub proof fn l107(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(
            u::lit("<thead><tr><th>Passage</th><th>Status</th><th>Section</th></tr></thead>"@),
        ),
        u::copy_derived(
            u::lit("<thead><tr><th>Passage</th><th>Status</th><th>Section</th></tr></thead>"@),
            inputs,
            u::copy_registry(),
        ),
        u::copy_registry()[107] == u::lit(
            "<thead><tr><th>Passage</th><th>Status</th><th>Section</th></tr></thead>"@,
        ),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[107] == u::lit(
        "<thead><tr><th>Passage</th><th>Status</th><th>Section</th></tr></thead>"@,
    ) && 107 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(
        u::lit("<thead><tr><th>Passage</th><th>Status</th><th>Section</th></tr></thead>"@),
        inputs,
    );
}

pub proof fn f107(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(
            u::fixed("<thead><tr><th>Passage</th><th>Status</th><th>Section</th></tr></thead>"@),
            inputs,
            0,
            0,
        ),
        u::copy_registry()[107] == u::lit(
            "<thead><tr><th>Passage</th><th>Status</th><th>Section</th></tr></thead>"@,
        ),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l107(inputs);
    reveal_strlit("<thead><tr><th>Passage</th><th>Status</th><th>Section</th></tr></thead>");
    is_ascii_chars_encode_utf8(
        "<thead><tr><th>Passage</th><th>Status</th><th>Section</th></tr></thead>"@,
    );
    assert(u::lit("<thead><tr><th>Passage</th><th>Status</th><th>Section</th></tr></thead>"@)
        =~= seq![
        60u8,
        116,
        104,
        101,
        97,
        100,
        62,
        60,
        116,
        114,
        62,
        60,
        116,
        104,
        62,
        80,
        97,
        115,
        115,
        97,
        103,
        101,
        60,
        47,
        116,
        104,
        62,
        60,
        116,
        104,
        62,
        83,
        116,
        97,
        116,
        117,
        115,
        60,
        47,
        116,
        104,
        62,
        60,
        116,
        104,
        62,
        83,
        101,
        99,
        116,
        105,
        111,
        110,
        60,
        47,
        116,
        104,
        62,
        60,
        47,
        116,
        114,
        62,
        60,
        47,
        116,
        104,
        101,
        97,
        100,
        62,
    ]);
    assert(cx::scan(
        seq![
            60u8,
            116,
            104,
            101,
            97,
            100,
            62,
            60,
            116,
            114,
            62,
            60,
            116,
            104,
            62,
            80,
            97,
            115,
            115,
            97,
            103,
            101,
            60,
            47,
            116,
            104,
            62,
            60,
            116,
            104,
            62,
            83,
            116,
            97,
            116,
            117,
            115,
            60,
            47,
            116,
            104,
            62,
            60,
            116,
            104,
            62,
            83,
            101,
            99,
            116,
            105,
            111,
            110,
            60,
            47,
            116,
            104,
            62,
            60,
            47,
            116,
            114,
            62,
            60,
            47,
            116,
            104,
            101,
            97,
            100,
            62,
        ],
        0,
    ) == 0) by (compute_only);
    cx::exact(
        u::lit("<thead><tr><th>Passage</th><th>Status</th><th>Section</th></tr></thead>"@),
        0,
    );
    h::fixed(
        u::lit("<thead><tr><th>Passage</th><th>Status</th><th>Section</th></tr></thead>"@),
        inputs,
        0,
        0,
    );
}

pub proof fn l108(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<a href=\"../../index.html\">guidelines</a> / "@)),
        u::copy_derived(
            u::lit("<a href=\"../../index.html\">guidelines</a> / "@),
            inputs,
            u::copy_registry(),
        ),
        u::copy_registry()[108] == u::lit("<a href=\"../../index.html\">guidelines</a> / "@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[108] == u::lit("<a href=\"../../index.html\">guidelines</a> / "@)
        && 108 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<a href=\"../../index.html\">guidelines</a> / "@), inputs);
}

pub proof fn f108(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<a href=\"../../index.html\">guidelines</a> / "@), inputs, 0, 0),
        u::copy_registry()[108] == u::lit("<a href=\"../../index.html\">guidelines</a> / "@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l108(inputs);
    reveal_strlit("<a href=\"../../index.html\">guidelines</a> / ");
    is_ascii_chars_encode_utf8("<a href=\"../../index.html\">guidelines</a> / "@);
    assert(u::lit("<a href=\"../../index.html\">guidelines</a> / "@) =~= seq![
        60u8,
        97,
        32,
        104,
        114,
        101,
        102,
        61,
        34,
        46,
        46,
        47,
        46,
        46,
        47,
        105,
        110,
        100,
        101,
        120,
        46,
        104,
        116,
        109,
        108,
        34,
        62,
        103,
        117,
        105,
        100,
        101,
        108,
        105,
        110,
        101,
        115,
        60,
        47,
        97,
        62,
        32,
        47,
        32,
    ]);
    assert(cx::scan(
        seq![
            60u8,
            97,
            32,
            104,
            114,
            101,
            102,
            61,
            34,
            46,
            46,
            47,
            46,
            46,
            47,
            105,
            110,
            100,
            101,
            120,
            46,
            104,
            116,
            109,
            108,
            34,
            62,
            103,
            117,
            105,
            100,
            101,
            108,
            105,
            110,
            101,
            115,
            60,
            47,
            97,
            62,
            32,
            47,
            32,
        ],
        0,
    ) == 0) by (compute_only);
    cx::exact(u::lit("<a href=\"../../index.html\">guidelines</a> / "@), 0);
    h::fixed(u::lit("<a href=\"../../index.html\">guidelines</a> / "@), inputs, 0, 0);
}

pub proof fn l109(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("https://github.com/eturkes/cnl-ckc/commit/"@)),
        u::copy_derived(
            u::lit("https://github.com/eturkes/cnl-ckc/commit/"@),
            inputs,
            u::copy_registry(),
        ),
        u::copy_registry()[109] == u::lit("https://github.com/eturkes/cnl-ckc/commit/"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[109] == u::lit("https://github.com/eturkes/cnl-ckc/commit/"@) && 109
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("https://github.com/eturkes/cnl-ckc/commit/"@), inputs);
}

pub proof fn l110(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("Current"@)),
        u::copy_derived(u::lit("Current"@), inputs, u::copy_registry()),
        u::copy_registry()[110] == u::lit("Current"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[110] == u::lit("Current"@) && 110 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("Current"@), inputs);
}

pub proof fn l111(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("Earlier"@)),
        u::copy_derived(u::lit("Earlier"@), inputs, u::copy_registry()),
        u::copy_registry()[111] == u::lit("Earlier"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[111] == u::lit("Earlier"@) && 111 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("Earlier"@), inputs);
}

pub proof fn l112(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("Not given"@)),
        u::copy_derived(u::lit("Not given"@), inputs, u::copy_registry()),
        u::copy_registry()[112] == u::lit("Not given"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[112] == u::lit("Not given"@) && 112 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("Not given"@), inputs);
}

pub proof fn l113(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<section id=\""@)),
        u::copy_derived(u::lit("<section id=\""@), inputs, u::copy_registry()),
        u::copy_registry()[113] == u::lit("<section id=\""@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[113] == u::lit("<section id=\""@) && 113 < u::copy_registry().len())
        by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<section id=\""@), inputs);
}

pub proof fn f113(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<section id=\""@), inputs, 0, 2),
        u::copy_registry()[113] == u::lit("<section id=\""@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l113(inputs);
    reveal_strlit("<section id=\"");
    is_ascii_chars_encode_utf8("<section id=\""@);
    assert(u::lit("<section id=\""@) =~= seq![
        60u8,
        115,
        101,
        99,
        116,
        105,
        111,
        110,
        32,
        105,
        100,
        61,
        34,
    ]);
    assert(cx::scan(seq![60u8, 115, 101, 99, 116, 105, 111, 110, 32, 105, 100, 61, 34], 0) == 2)
        by (compute_only);
    cx::exact(u::lit("<section id=\""@), 0);
    h::fixed(u::lit("<section id=\""@), inputs, 0, 2);
}

pub proof fn l114(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<h2>"@)),
        u::copy_derived(u::lit("<h2>"@), inputs, u::copy_registry()),
        u::copy_registry()[114] == u::lit("<h2>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[114] == u::lit("<h2>"@) && 114 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<h2>"@), inputs);
}

pub proof fn f114(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<h2>"@), inputs, 0, 0),
        u::copy_registry()[114] == u::lit("<h2>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l114(inputs);
    reveal_strlit("<h2>");
    is_ascii_chars_encode_utf8("<h2>"@);
    assert(u::lit("<h2>"@) =~= seq![60u8, 104, 50, 62]);
    assert(cx::scan(seq![60u8, 104, 50, 62], 0) == 0) by (compute_only);
    cx::exact(u::lit("<h2>"@), 0);
    h::fixed(u::lit("<h2>"@), inputs, 0, 0);
}

pub proof fn l115(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("</h2>"@)),
        u::copy_derived(u::lit("</h2>"@), inputs, u::copy_registry()),
        u::copy_registry()[115] == u::lit("</h2>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[115] == u::lit("</h2>"@) && 115 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("</h2>"@), inputs);
}

pub proof fn f115(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("</h2>"@), inputs, 0, 0),
        u::copy_registry()[115] == u::lit("</h2>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l115(inputs);
    reveal_strlit("</h2>");
    is_ascii_chars_encode_utf8("</h2>"@);
    assert(u::lit("</h2>"@) =~= seq![60u8, 47, 104, 50, 62]);
    assert(cx::scan(seq![60u8, 47, 104, 50, 62], 0) == 0) by (compute_only);
    cx::exact(u::lit("</h2>"@), 0);
    h::fixed(u::lit("</h2>"@), inputs, 0, 0);
}

pub proof fn l116(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<table class=\"records\">"@)),
        u::copy_derived(u::lit("<table class=\"records\">"@), inputs, u::copy_registry()),
        u::copy_registry()[116] == u::lit("<table class=\"records\">"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[116] == u::lit("<table class=\"records\">"@) && 116
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<table class=\"records\">"@), inputs);
}

pub proof fn f116(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<table class=\"records\">"@), inputs, 0, 0),
        u::copy_registry()[116] == u::lit("<table class=\"records\">"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l116(inputs);
    reveal_strlit("<table class=\"records\">");
    is_ascii_chars_encode_utf8("<table class=\"records\">"@);
    assert(u::lit("<table class=\"records\">"@) =~= seq![
        60u8,
        116,
        97,
        98,
        108,
        101,
        32,
        99,
        108,
        97,
        115,
        115,
        61,
        34,
        114,
        101,
        99,
        111,
        114,
        100,
        115,
        34,
        62,
    ]);
    assert(cx::scan(
        seq![
            60u8,
            116,
            97,
            98,
            108,
            101,
            32,
            99,
            108,
            97,
            115,
            115,
            61,
            34,
            114,
            101,
            99,
            111,
            114,
            100,
            115,
            34,
            62,
        ],
        0,
    ) == 0) by (compute_only);
    cx::exact(u::lit("<table class=\"records\">"@), 0);
    h::fixed(u::lit("<table class=\"records\">"@), inputs, 0, 0);
}

pub proof fn l117(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(
            u::lit(
                "<thead><tr><th>Decision</th><th>Reviewer</th><th>Date</th><th>Version</th><th>Comment</th></tr></thead>"@,
            ),
        ),
        u::copy_derived(
            u::lit(
                "<thead><tr><th>Decision</th><th>Reviewer</th><th>Date</th><th>Version</th><th>Comment</th></tr></thead>"@,
            ),
            inputs,
            u::copy_registry(),
        ),
        u::copy_registry()[117] == u::lit(
            "<thead><tr><th>Decision</th><th>Reviewer</th><th>Date</th><th>Version</th><th>Comment</th></tr></thead>"@,
        ),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[117] == u::lit(
        "<thead><tr><th>Decision</th><th>Reviewer</th><th>Date</th><th>Version</th><th>Comment</th></tr></thead>"@,
    ) && 117 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(
        u::lit(
            "<thead><tr><th>Decision</th><th>Reviewer</th><th>Date</th><th>Version</th><th>Comment</th></tr></thead>"@,
        ),
        inputs,
    );
}

pub proof fn f117(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(
            u::fixed(
                "<thead><tr><th>Decision</th><th>Reviewer</th><th>Date</th><th>Version</th><th>Comment</th></tr></thead>"@,
            ),
            inputs,
            0,
            0,
        ),
        u::copy_registry()[117] == u::lit(
            "<thead><tr><th>Decision</th><th>Reviewer</th><th>Date</th><th>Version</th><th>Comment</th></tr></thead>"@,
        ),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l117(inputs);
    reveal_strlit(
        "<thead><tr><th>Decision</th><th>Reviewer</th><th>Date</th><th>Version</th><th>Comment</th></tr></thead>",
    );
    is_ascii_chars_encode_utf8(
        "<thead><tr><th>Decision</th><th>Reviewer</th><th>Date</th><th>Version</th><th>Comment</th></tr></thead>"@,
    );
    assert(u::lit(
        "<thead><tr><th>Decision</th><th>Reviewer</th><th>Date</th><th>Version</th><th>Comment</th></tr></thead>"@,
    ) =~= seq![
        60u8,
        116,
        104,
        101,
        97,
        100,
        62,
        60,
        116,
        114,
        62,
        60,
        116,
        104,
        62,
        68,
        101,
        99,
        105,
        115,
        105,
        111,
        110,
        60,
        47,
        116,
        104,
        62,
        60,
        116,
        104,
        62,
        82,
        101,
        118,
        105,
        101,
        119,
        101,
        114,
        60,
        47,
        116,
        104,
        62,
        60,
        116,
        104,
        62,
        68,
        97,
        116,
        101,
        60,
        47,
        116,
        104,
        62,
        60,
        116,
        104,
        62,
        86,
        101,
        114,
        115,
        105,
        111,
        110,
        60,
        47,
        116,
        104,
        62,
        60,
        116,
        104,
        62,
        67,
        111,
        109,
        109,
        101,
        110,
        116,
        60,
        47,
        116,
        104,
        62,
        60,
        47,
        116,
        114,
        62,
        60,
        47,
        116,
        104,
        101,
        97,
        100,
        62,
    ]);
    assert(cx::scan(
        seq![
            60u8,
            116,
            104,
            101,
            97,
            100,
            62,
            60,
            116,
            114,
            62,
            60,
            116,
            104,
            62,
            68,
            101,
            99,
            105,
            115,
            105,
            111,
            110,
            60,
            47,
            116,
            104,
            62,
            60,
            116,
            104,
            62,
            82,
            101,
            118,
            105,
            101,
            119,
            101,
            114,
            60,
            47,
            116,
            104,
            62,
            60,
            116,
            104,
            62,
            68,
            97,
            116,
            101,
            60,
            47,
            116,
            104,
            62,
            60,
            116,
            104,
            62,
            86,
            101,
            114,
            115,
            105,
            111,
            110,
            60,
            47,
            116,
            104,
            62,
            60,
            116,
            104,
            62,
            67,
            111,
            109,
            109,
            101,
            110,
            116,
            60,
            47,
            116,
            104,
            62,
            60,
            47,
            116,
            114,
            62,
            60,
            47,
            116,
            104,
            101,
            97,
            100,
            62,
        ],
        0,
    ) == 0) by (compute_only);
    cx::exact(
        u::lit(
            "<thead><tr><th>Decision</th><th>Reviewer</th><th>Date</th><th>Version</th><th>Comment</th></tr></thead>"@,
        ),
        0,
    );
    h::fixed(
        u::lit(
            "<thead><tr><th>Decision</th><th>Reviewer</th><th>Date</th><th>Version</th><th>Comment</th></tr></thead>"@,
        ),
        inputs,
        0,
        0,
    );
}

pub proof fn l118(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit(" The newest decision for each document is first."@)),
        u::copy_derived(
            u::lit(" The newest decision for each document is first."@),
            inputs,
            u::copy_registry(),
        ),
        u::copy_registry()[118] == u::lit(" The newest decision for each document is first."@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[118] == u::lit(" The newest decision for each document is first."@)
        && 118 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit(" The newest decision for each document is first."@), inputs);
}

pub proof fn l119(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(
            u::lit("<p>Open a document and record a decision to start this list.</p>"@),
        ),
        u::copy_derived(
            u::lit("<p>Open a document and record a decision to start this list.</p>"@),
            inputs,
            u::copy_registry(),
        ),
        u::copy_registry()[119] == u::lit(
            "<p>Open a document and record a decision to start this list.</p>"@,
        ),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[119] == u::lit(
        "<p>Open a document and record a decision to start this list.</p>"@,
    ) && 119 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<p>Open a document and record a decision to start this list.</p>"@), inputs);
}

pub proof fn f119(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(
            u::fixed("<p>Open a document and record a decision to start this list.</p>"@),
            inputs,
            0,
            0,
        ),
        u::copy_registry()[119] == u::lit(
            "<p>Open a document and record a decision to start this list.</p>"@,
        ),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l119(inputs);
    reveal_strlit("<p>Open a document and record a decision to start this list.</p>");
    is_ascii_chars_encode_utf8("<p>Open a document and record a decision to start this list.</p>"@);
    assert(u::lit("<p>Open a document and record a decision to start this list.</p>"@) =~= seq![
        60u8,
        112,
        62,
        79,
        112,
        101,
        110,
        32,
        97,
        32,
        100,
        111,
        99,
        117,
        109,
        101,
        110,
        116,
        32,
        97,
        110,
        100,
        32,
        114,
        101,
        99,
        111,
        114,
        100,
        32,
        97,
        32,
        100,
        101,
        99,
        105,
        115,
        105,
        111,
        110,
        32,
        116,
        111,
        32,
        115,
        116,
        97,
        114,
        116,
        32,
        116,
        104,
        105,
        115,
        32,
        108,
        105,
        115,
        116,
        46,
        60,
        47,
        112,
        62,
    ]);
    assert(cx::scan(
        seq![
            60u8,
            112,
            62,
            79,
            112,
            101,
            110,
            32,
            97,
            32,
            100,
            111,
            99,
            117,
            109,
            101,
            110,
            116,
            32,
            97,
            110,
            100,
            32,
            114,
            101,
            99,
            111,
            114,
            100,
            32,
            97,
            32,
            100,
            101,
            99,
            105,
            115,
            105,
            111,
            110,
            32,
            116,
            111,
            32,
            115,
            116,
            97,
            114,
            116,
            32,
            116,
            104,
            105,
            115,
            32,
            108,
            105,
            115,
            116,
            46,
            60,
            47,
            112,
            62,
        ],
        0,
    ) == 0) by (compute_only);
    cx::exact(u::lit("<p>Open a document and record a decision to start this list.</p>"@), 0);
    h::fixed(
        u::lit("<p>Open a document and record a decision to start this list.</p>"@),
        inputs,
        0,
        0,
    );
}

pub proof fn l120(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(
            u::lit("<p>Each reviewer name is recorded as entered and is not verified.</p>"@),
        ),
        u::copy_derived(
            u::lit("<p>Each reviewer name is recorded as entered and is not verified.</p>"@),
            inputs,
            u::copy_registry(),
        ),
        u::copy_registry()[120] == u::lit(
            "<p>Each reviewer name is recorded as entered and is not verified.</p>"@,
        ),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[120] == u::lit(
        "<p>Each reviewer name is recorded as entered and is not verified.</p>"@,
    ) && 120 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(
        u::lit("<p>Each reviewer name is recorded as entered and is not verified.</p>"@),
        inputs,
    );
}

pub proof fn f120(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(
            u::fixed("<p>Each reviewer name is recorded as entered and is not verified.</p>"@),
            inputs,
            0,
            0,
        ),
        u::copy_registry()[120] == u::lit(
            "<p>Each reviewer name is recorded as entered and is not verified.</p>"@,
        ),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l120(inputs);
    reveal_strlit("<p>Each reviewer name is recorded as entered and is not verified.</p>");
    is_ascii_chars_encode_utf8(
        "<p>Each reviewer name is recorded as entered and is not verified.</p>"@,
    );
    assert(u::lit("<p>Each reviewer name is recorded as entered and is not verified.</p>"@)
        =~= seq![
        60u8,
        112,
        62,
        69,
        97,
        99,
        104,
        32,
        114,
        101,
        118,
        105,
        101,
        119,
        101,
        114,
        32,
        110,
        97,
        109,
        101,
        32,
        105,
        115,
        32,
        114,
        101,
        99,
        111,
        114,
        100,
        101,
        100,
        32,
        97,
        115,
        32,
        101,
        110,
        116,
        101,
        114,
        101,
        100,
        32,
        97,
        110,
        100,
        32,
        105,
        115,
        32,
        110,
        111,
        116,
        32,
        118,
        101,
        114,
        105,
        102,
        105,
        101,
        100,
        46,
        60,
        47,
        112,
        62,
    ]);
    assert(cx::scan(
        seq![
            60u8,
            112,
            62,
            69,
            97,
            99,
            104,
            32,
            114,
            101,
            118,
            105,
            101,
            119,
            101,
            114,
            32,
            110,
            97,
            109,
            101,
            32,
            105,
            115,
            32,
            114,
            101,
            99,
            111,
            114,
            100,
            101,
            100,
            32,
            97,
            115,
            32,
            101,
            110,
            116,
            101,
            114,
            101,
            100,
            32,
            97,
            110,
            100,
            32,
            105,
            115,
            32,
            110,
            111,
            116,
            32,
            118,
            101,
            114,
            105,
            102,
            105,
            101,
            100,
            46,
            60,
            47,
            112,
            62,
        ],
        0,
    ) == 0) by (compute_only);
    cx::exact(u::lit("<p>Each reviewer name is recorded as entered and is not verified.</p>"@), 0);
    h::fixed(
        u::lit("<p>Each reviewer name is recorded as entered and is not verified.</p>"@),
        inputs,
        0,
        0,
    );
}

pub proof fn l121(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(
            u::lit(
                "<p>Each version links to the stored version of the text that the reviewer read.</p>"@,
            ),
        ),
        u::copy_derived(
            u::lit(
                "<p>Each version links to the stored version of the text that the reviewer read.</p>"@,
            ),
            inputs,
            u::copy_registry(),
        ),
        u::copy_registry()[121] == u::lit(
            "<p>Each version links to the stored version of the text that the reviewer read.</p>"@,
        ),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[121] == u::lit(
        "<p>Each version links to the stored version of the text that the reviewer read.</p>"@,
    ) && 121 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(
        u::lit(
            "<p>Each version links to the stored version of the text that the reviewer read.</p>"@,
        ),
        inputs,
    );
}

pub proof fn f121(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(
            u::fixed(
                "<p>Each version links to the stored version of the text that the reviewer read.</p>"@,
            ),
            inputs,
            0,
            0,
        ),
        u::copy_registry()[121] == u::lit(
            "<p>Each version links to the stored version of the text that the reviewer read.</p>"@,
        ),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l121(inputs);
    reveal_strlit(
        "<p>Each version links to the stored version of the text that the reviewer read.</p>",
    );
    is_ascii_chars_encode_utf8(
        "<p>Each version links to the stored version of the text that the reviewer read.</p>"@,
    );
    assert(u::lit(
        "<p>Each version links to the stored version of the text that the reviewer read.</p>"@,
    ) =~= seq![
        60u8,
        112,
        62,
        69,
        97,
        99,
        104,
        32,
        118,
        101,
        114,
        115,
        105,
        111,
        110,
        32,
        108,
        105,
        110,
        107,
        115,
        32,
        116,
        111,
        32,
        116,
        104,
        101,
        32,
        115,
        116,
        111,
        114,
        101,
        100,
        32,
        118,
        101,
        114,
        115,
        105,
        111,
        110,
        32,
        111,
        102,
        32,
        116,
        104,
        101,
        32,
        116,
        101,
        120,
        116,
        32,
        116,
        104,
        97,
        116,
        32,
        116,
        104,
        101,
        32,
        114,
        101,
        118,
        105,
        101,
        119,
        101,
        114,
        32,
        114,
        101,
        97,
        100,
        46,
        60,
        47,
        112,
        62,
    ]);
    assert(cx::scan(
        seq![
            60u8,
            112,
            62,
            69,
            97,
            99,
            104,
            32,
            118,
            101,
            114,
            115,
            105,
            111,
            110,
            32,
            108,
            105,
            110,
            107,
            115,
            32,
            116,
            111,
            32,
            116,
            104,
            101,
            32,
            115,
            116,
            111,
            114,
            101,
            100,
            32,
            118,
            101,
            114,
            115,
            105,
            111,
            110,
            32,
            111,
            102,
            32,
            116,
            104,
            101,
            32,
            116,
            101,
            120,
            116,
            32,
            116,
            104,
            97,
            116,
            32,
            116,
            104,
            101,
            32,
            114,
            101,
            118,
            105,
            101,
            119,
            101,
            114,
            32,
            114,
            101,
            97,
            100,
            46,
            60,
            47,
            112,
            62,
        ],
        0,
    ) == 0) by (compute_only);
    cx::exact(
        u::lit(
            "<p>Each version links to the stored version of the text that the reviewer read.</p>"@,
        ),
        0,
    );
    h::fixed(
        u::lit(
            "<p>Each version links to the stored version of the text that the reviewer read.</p>"@,
        ),
        inputs,
        0,
        0,
    );
}

pub proof fn l122(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("Decision records"@)),
        u::copy_derived(u::lit("Decision records"@), inputs, u::copy_registry()),
        u::copy_registry()[122] == u::lit("Decision records"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[122] == u::lit("Decision records"@) && 122 < u::copy_registry().len())
        by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("Decision records"@), inputs);
}

pub proof fn l123(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(
            u::lit("<a href=\"../../index.html\">guidelines</a> / <a href=\"index.html\">"@),
        ),
        u::copy_derived(
            u::lit("<a href=\"../../index.html\">guidelines</a> / <a href=\"index.html\">"@),
            inputs,
            u::copy_registry(),
        ),
        u::copy_registry()[123] == u::lit(
            "<a href=\"../../index.html\">guidelines</a> / <a href=\"index.html\">"@,
        ),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[123] == u::lit(
        "<a href=\"../../index.html\">guidelines</a> / <a href=\"index.html\">"@,
    ) && 123 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(
        u::lit("<a href=\"../../index.html\">guidelines</a> / <a href=\"index.html\">"@),
        inputs,
    );
}

pub proof fn f123(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(
            u::fixed("<a href=\"../../index.html\">guidelines</a> / <a href=\"index.html\">"@),
            inputs,
            0,
            0,
        ),
        u::copy_registry()[123] == u::lit(
            "<a href=\"../../index.html\">guidelines</a> / <a href=\"index.html\">"@,
        ),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l123(inputs);
    reveal_strlit("<a href=\"../../index.html\">guidelines</a> / <a href=\"index.html\">");
    is_ascii_chars_encode_utf8(
        "<a href=\"../../index.html\">guidelines</a> / <a href=\"index.html\">"@,
    );
    assert(u::lit("<a href=\"../../index.html\">guidelines</a> / <a href=\"index.html\">"@)
        =~= seq![
        60u8,
        97,
        32,
        104,
        114,
        101,
        102,
        61,
        34,
        46,
        46,
        47,
        46,
        46,
        47,
        105,
        110,
        100,
        101,
        120,
        46,
        104,
        116,
        109,
        108,
        34,
        62,
        103,
        117,
        105,
        100,
        101,
        108,
        105,
        110,
        101,
        115,
        60,
        47,
        97,
        62,
        32,
        47,
        32,
        60,
        97,
        32,
        104,
        114,
        101,
        102,
        61,
        34,
        105,
        110,
        100,
        101,
        120,
        46,
        104,
        116,
        109,
        108,
        34,
        62,
    ]);
    assert(cx::scan(
        seq![
            60u8,
            97,
            32,
            104,
            114,
            101,
            102,
            61,
            34,
            46,
            46,
            47,
            46,
            46,
            47,
            105,
            110,
            100,
            101,
            120,
            46,
            104,
            116,
            109,
            108,
            34,
            62,
            103,
            117,
            105,
            100,
            101,
            108,
            105,
            110,
            101,
            115,
            60,
            47,
            97,
            62,
            32,
            47,
            32,
            60,
            97,
            32,
            104,
            114,
            101,
            102,
            61,
            34,
            105,
            110,
            100,
            101,
            120,
            46,
            104,
            116,
            109,
            108,
            34,
            62,
        ],
        0,
    ) == 0) by (compute_only);
    cx::exact(u::lit("<a href=\"../../index.html\">guidelines</a> / <a href=\"index.html\">"@), 0);
    h::fixed(
        u::lit("<a href=\"../../index.html\">guidelines</a> / <a href=\"index.html\">"@),
        inputs,
        0,
        0,
    );
}

pub proof fn l124(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("</a> / records"@)),
        u::copy_derived(u::lit("</a> / records"@), inputs, u::copy_registry()),
        u::copy_registry()[124] == u::lit("</a> / records"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[124] == u::lit("</a> / records"@) && 124 < u::copy_registry().len())
        by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("</a> / records"@), inputs);
}

pub proof fn f124(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("</a> / records"@), inputs, 0, 0),
        u::copy_registry()[124] == u::lit("</a> / records"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l124(inputs);
    reveal_strlit("</a> / records");
    is_ascii_chars_encode_utf8("</a> / records"@);
    assert(u::lit("</a> / records"@) =~= seq![
        60u8,
        47,
        97,
        62,
        32,
        47,
        32,
        114,
        101,
        99,
        111,
        114,
        100,
        115,
    ]);
    assert(cx::scan(seq![60u8, 47, 97, 62, 32, 47, 32, 114, 101, 99, 111, 114, 100, 115], 0) == 0)
        by (compute_only);
    cx::exact(u::lit("</a> / records"@), 0);
    h::fixed(u::lit("</a> / records"@), inputs, 0, 0);
}

pub proof fn l125(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<h1>Decision records</h1>"@)),
        u::copy_derived(u::lit("<h1>Decision records</h1>"@), inputs, u::copy_registry()),
        u::copy_registry()[125] == u::lit("<h1>Decision records</h1>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[125] == u::lit("<h1>Decision records</h1>"@) && 125
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<h1>Decision records</h1>"@), inputs);
}

pub proof fn f125(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<h1>Decision records</h1>"@), inputs, 0, 0),
        u::copy_registry()[125] == u::lit("<h1>Decision records</h1>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l125(inputs);
    reveal_strlit("<h1>Decision records</h1>");
    is_ascii_chars_encode_utf8("<h1>Decision records</h1>"@);
    assert(u::lit("<h1>Decision records</h1>"@) =~= seq![
        60u8,
        104,
        49,
        62,
        68,
        101,
        99,
        105,
        115,
        105,
        111,
        110,
        32,
        114,
        101,
        99,
        111,
        114,
        100,
        115,
        60,
        47,
        104,
        49,
        62,
    ]);
    assert(cx::scan(
        seq![
            60u8,
            104,
            49,
            62,
            68,
            101,
            99,
            105,
            115,
            105,
            111,
            110,
            32,
            114,
            101,
            99,
            111,
            114,
            100,
            115,
            60,
            47,
            104,
            49,
            62,
        ],
        0,
    ) == 0) by (compute_only);
    cx::exact(u::lit("<h1>Decision records</h1>"@), 0);
    h::fixed(u::lit("<h1>Decision records</h1>"@), inputs, 0, 0);
}

pub proof fn l126(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("</p>"@)),
        u::copy_derived(u::lit("</p>"@), inputs, u::copy_registry()),
        u::copy_registry()[126] == u::lit("</p>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[126] == u::lit("</p>"@) && 126 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("</p>"@), inputs);
}

pub proof fn f126(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("</p>"@), inputs, 0, 0),
        u::copy_registry()[126] == u::lit("</p>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l126(inputs);
    reveal_strlit("</p>");
    is_ascii_chars_encode_utf8("</p>"@);
    assert(u::lit("</p>"@) =~= seq![60u8, 47, 112, 62]);
    assert(cx::scan(seq![60u8, 47, 112, 62], 0) == 0) by (compute_only);
    cx::exact(u::lit("</p>"@), 0);
    h::fixed(u::lit("</p>"@), inputs, 0, 0);
}

pub proof fn l127(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(
            u::lit("<nav class=\"docnav\"><a href=\"index.html\">Guideline index</a></nav>"@),
        ),
        u::copy_derived(
            u::lit("<nav class=\"docnav\"><a href=\"index.html\">Guideline index</a></nav>"@),
            inputs,
            u::copy_registry(),
        ),
        u::copy_registry()[127] == u::lit(
            "<nav class=\"docnav\"><a href=\"index.html\">Guideline index</a></nav>"@,
        ),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[127] == u::lit(
        "<nav class=\"docnav\"><a href=\"index.html\">Guideline index</a></nav>"@,
    ) && 127 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(
        u::lit("<nav class=\"docnav\"><a href=\"index.html\">Guideline index</a></nav>"@),
        inputs,
    );
}

pub proof fn f127(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(
            u::fixed("<nav class=\"docnav\"><a href=\"index.html\">Guideline index</a></nav>"@),
            inputs,
            0,
            0,
        ),
        u::copy_registry()[127] == u::lit(
            "<nav class=\"docnav\"><a href=\"index.html\">Guideline index</a></nav>"@,
        ),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l127(inputs);
    reveal_strlit("<nav class=\"docnav\"><a href=\"index.html\">Guideline index</a></nav>");
    is_ascii_chars_encode_utf8(
        "<nav class=\"docnav\"><a href=\"index.html\">Guideline index</a></nav>"@,
    );
    assert(u::lit("<nav class=\"docnav\"><a href=\"index.html\">Guideline index</a></nav>"@)
        =~= seq![
        60u8,
        110,
        97,
        118,
        32,
        99,
        108,
        97,
        115,
        115,
        61,
        34,
        100,
        111,
        99,
        110,
        97,
        118,
        34,
        62,
        60,
        97,
        32,
        104,
        114,
        101,
        102,
        61,
        34,
        105,
        110,
        100,
        101,
        120,
        46,
        104,
        116,
        109,
        108,
        34,
        62,
        71,
        117,
        105,
        100,
        101,
        108,
        105,
        110,
        101,
        32,
        105,
        110,
        100,
        101,
        120,
        60,
        47,
        97,
        62,
        60,
        47,
        110,
        97,
        118,
        62,
    ]);
    assert(cx::scan(
        seq![
            60u8,
            110,
            97,
            118,
            32,
            99,
            108,
            97,
            115,
            115,
            61,
            34,
            100,
            111,
            99,
            110,
            97,
            118,
            34,
            62,
            60,
            97,
            32,
            104,
            114,
            101,
            102,
            61,
            34,
            105,
            110,
            100,
            101,
            120,
            46,
            104,
            116,
            109,
            108,
            34,
            62,
            71,
            117,
            105,
            100,
            101,
            108,
            105,
            110,
            101,
            32,
            105,
            110,
            100,
            101,
            120,
            60,
            47,
            97,
            62,
            60,
            47,
            110,
            97,
            118,
            62,
        ],
        0,
    ) == 0) by (compute_only);
    cx::exact(u::lit("<nav class=\"docnav\"><a href=\"index.html\">Guideline index</a></nav>"@), 0);
    h::fixed(
        u::lit("<nav class=\"docnav\"><a href=\"index.html\">Guideline index</a></nav>"@),
        inputs,
        0,
        0,
    );
}

pub proof fn l128(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<span class=\"kw\">"@)),
        u::copy_derived(u::lit("<span class=\"kw\">"@), inputs, u::copy_registry()),
        u::copy_registry()[128] == u::lit("<span class=\"kw\">"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[128] == u::lit("<span class=\"kw\">"@) && 128
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<span class=\"kw\">"@), inputs);
}

pub proof fn f128(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<span class=\"kw\">"@), inputs, 0, 0),
        u::copy_registry()[128] == u::lit("<span class=\"kw\">"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l128(inputs);
    reveal_strlit("<span class=\"kw\">");
    is_ascii_chars_encode_utf8("<span class=\"kw\">"@);
    assert(u::lit("<span class=\"kw\">"@) =~= seq![
        60u8,
        115,
        112,
        97,
        110,
        32,
        99,
        108,
        97,
        115,
        115,
        61,
        34,
        107,
        119,
        34,
        62,
    ]);
    assert(cx::scan(
        seq![60u8, 115, 112, 97, 110, 32, 99, 108, 97, 115, 115, 61, 34, 107, 119, 34, 62],
        0,
    ) == 0) by (compute_only);
    cx::exact(u::lit("<span class=\"kw\">"@), 0);
    h::fixed(u::lit("<span class=\"kw\">"@), inputs, 0, 0);
}

pub proof fn l129(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<mark class=\"t"@)),
        u::copy_derived(u::lit("<mark class=\"t"@), inputs, u::copy_registry()),
        u::copy_registry()[129] == u::lit("<mark class=\"t"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[129] == u::lit("<mark class=\"t"@) && 129 < u::copy_registry().len())
        by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<mark class=\"t"@), inputs);
}

pub proof fn f129(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<mark class=\"t"@), inputs, 0, 2),
        u::copy_registry()[129] == u::lit("<mark class=\"t"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l129(inputs);
    reveal_strlit("<mark class=\"t");
    is_ascii_chars_encode_utf8("<mark class=\"t"@);
    assert(u::lit("<mark class=\"t"@) =~= seq![
        60u8,
        109,
        97,
        114,
        107,
        32,
        99,
        108,
        97,
        115,
        115,
        61,
        34,
        116,
    ]);
    assert(cx::scan(seq![60u8, 109, 97, 114, 107, 32, 99, 108, 97, 115, 115, 61, 34, 116], 0) == 2)
        by (compute_only);
    cx::exact(u::lit("<mark class=\"t"@), 0);
    h::fixed(u::lit("<mark class=\"t"@), inputs, 0, 2);
}

pub proof fn l130(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<mark>"@)),
        u::copy_derived(u::lit("<mark>"@), inputs, u::copy_registry()),
        u::copy_registry()[130] == u::lit("<mark>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[130] == u::lit("<mark>"@) && 130 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<mark>"@), inputs);
}

pub proof fn f130(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<mark>"@), inputs, 0, 0),
        u::copy_registry()[130] == u::lit("<mark>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l130(inputs);
    reveal_strlit("<mark>");
    is_ascii_chars_encode_utf8("<mark>"@);
    assert(u::lit("<mark>"@) =~= seq![60u8, 109, 97, 114, 107, 62]);
    assert(cx::scan(seq![60u8, 109, 97, 114, 107, 62], 0) == 0) by (compute_only);
    cx::exact(u::lit("<mark>"@), 0);
    h::fixed(u::lit("<mark>"@), inputs, 0, 0);
}

pub proof fn l131(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("</mark>"@)),
        u::copy_derived(u::lit("</mark>"@), inputs, u::copy_registry()),
        u::copy_registry()[131] == u::lit("</mark>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[131] == u::lit("</mark>"@) && 131 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("</mark>"@), inputs);
}

pub proof fn f131(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("</mark>"@), inputs, 0, 0),
        u::copy_registry()[131] == u::lit("</mark>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l131(inputs);
    reveal_strlit("</mark>");
    is_ascii_chars_encode_utf8("</mark>"@);
    assert(u::lit("</mark>"@) =~= seq![60u8, 47, 109, 97, 114, 107, 62]);
    assert(cx::scan(seq![60u8, 47, 109, 97, 114, 107, 62], 0) == 0) by (compute_only);
    cx::exact(u::lit("</mark>"@), 0);
    h::fixed(u::lit("</mark>"@), inputs, 0, 0);
}

pub proof fn l132(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<datalist id=\"reviewer-names\">"@)),
        u::copy_derived(u::lit("<datalist id=\"reviewer-names\">"@), inputs, u::copy_registry()),
        u::copy_registry()[132] == u::lit("<datalist id=\"reviewer-names\">"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[132] == u::lit("<datalist id=\"reviewer-names\">"@) && 132
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<datalist id=\"reviewer-names\">"@), inputs);
}

pub proof fn f132(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<datalist id=\"reviewer-names\">"@), inputs, 0, 0),
        u::copy_registry()[132] == u::lit("<datalist id=\"reviewer-names\">"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l132(inputs);
    reveal_strlit("<datalist id=\"reviewer-names\">");
    is_ascii_chars_encode_utf8("<datalist id=\"reviewer-names\">"@);
    assert(u::lit("<datalist id=\"reviewer-names\">"@) =~= seq![
        60u8,
        100,
        97,
        116,
        97,
        108,
        105,
        115,
        116,
        32,
        105,
        100,
        61,
        34,
        114,
        101,
        118,
        105,
        101,
        119,
        101,
        114,
        45,
        110,
        97,
        109,
        101,
        115,
        34,
        62,
    ]);
    assert(cx::scan(
        seq![
            60u8,
            100,
            97,
            116,
            97,
            108,
            105,
            115,
            116,
            32,
            105,
            100,
            61,
            34,
            114,
            101,
            118,
            105,
            101,
            119,
            101,
            114,
            45,
            110,
            97,
            109,
            101,
            115,
            34,
            62,
        ],
        0,
    ) == 0) by (compute_only);
    cx::exact(u::lit("<datalist id=\"reviewer-names\">"@), 0);
    h::fixed(u::lit("<datalist id=\"reviewer-names\">"@), inputs, 0, 0);
}

pub proof fn l133(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<option value=\""@)),
        u::copy_derived(u::lit("<option value=\""@), inputs, u::copy_registry()),
        u::copy_registry()[133] == u::lit("<option value=\""@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[133] == u::lit("<option value=\""@) && 133 < u::copy_registry().len())
        by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<option value=\""@), inputs);
}

pub proof fn f133(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<option value=\""@), inputs, 0, 2),
        u::copy_registry()[133] == u::lit("<option value=\""@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l133(inputs);
    reveal_strlit("<option value=\"");
    is_ascii_chars_encode_utf8("<option value=\""@);
    assert(u::lit("<option value=\""@) =~= seq![
        60u8,
        111,
        112,
        116,
        105,
        111,
        110,
        32,
        118,
        97,
        108,
        117,
        101,
        61,
        34,
    ]);
    assert(cx::scan(seq![60u8, 111, 112, 116, 105, 111, 110, 32, 118, 97, 108, 117, 101, 61, 34], 0)
        == 2) by (compute_only);
    cx::exact(u::lit("<option value=\""@), 0);
    h::fixed(u::lit("<option value=\""@), inputs, 0, 2);
}

pub proof fn l134(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("\"></option>"@)),
        u::copy_derived(u::lit("\"></option>"@), inputs, u::copy_registry()),
        u::copy_registry()[134] == u::lit("\"></option>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[134] == u::lit("\"></option>"@) && 134 < u::copy_registry().len())
        by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("\"></option>"@), inputs);
}

pub proof fn f134(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("\"></option>"@), inputs, 2, 0),
        u::copy_registry()[134] == u::lit("\"></option>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l134(inputs);
    reveal_strlit("\"></option>");
    is_ascii_chars_encode_utf8("\"></option>"@);
    assert(u::lit("\"></option>"@) =~= seq![34u8, 62, 60, 47, 111, 112, 116, 105, 111, 110, 62]);
    assert(cx::scan(seq![34u8, 62, 60, 47, 111, 112, 116, 105, 111, 110, 62], 2) == 0)
        by (compute_only);
    cx::exact(u::lit("\"></option>"@), 2);
    h::fixed(u::lit("\"></option>"@), inputs, 2, 0);
}

pub proof fn l135(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("</datalist>"@)),
        u::copy_derived(u::lit("</datalist>"@), inputs, u::copy_registry()),
        u::copy_registry()[135] == u::lit("</datalist>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[135] == u::lit("</datalist>"@) && 135 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("</datalist>"@), inputs);
}

pub proof fn f135(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("</datalist>"@), inputs, 0, 0),
        u::copy_registry()[135] == u::lit("</datalist>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l135(inputs);
    reveal_strlit("</datalist>");
    is_ascii_chars_encode_utf8("</datalist>"@);
    assert(u::lit("</datalist>"@) =~= seq![60u8, 47, 100, 97, 116, 97, 108, 105, 115, 116, 62]);
    assert(cx::scan(seq![60u8, 47, 100, 97, 116, 97, 108, 105, 115, 116, 62], 0) == 0)
        by (compute_only);
    cx::exact(u::lit("</datalist>"@), 0);
    h::fixed(u::lit("</datalist>"@), inputs, 0, 0);
}

pub proof fn l136(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit(".pdf"@)),
        u::copy_derived(u::lit(".pdf"@), inputs, u::copy_registry()),
        u::copy_registry()[136] == u::lit(".pdf"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[136] == u::lit(".pdf"@) && 136 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit(".pdf"@), inputs);
}

pub proof fn l137(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit(" <a class=\"source\" href=\"../source/"@)),
        u::copy_derived(
            u::lit(" <a class=\"source\" href=\"../source/"@),
            inputs,
            u::copy_registry(),
        ),
        u::copy_registry()[137] == u::lit(" <a class=\"source\" href=\"../source/"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[137] == u::lit(" <a class=\"source\" href=\"../source/"@) && 137
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit(" <a class=\"source\" href=\"../source/"@), inputs);
}

pub proof fn f137(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed(" <a class=\"source\" href=\"../source/"@), inputs, 0, 2),
        u::copy_registry()[137] == u::lit(" <a class=\"source\" href=\"../source/"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l137(inputs);
    reveal_strlit(" <a class=\"source\" href=\"../source/");
    is_ascii_chars_encode_utf8(" <a class=\"source\" href=\"../source/"@);
    assert(u::lit(" <a class=\"source\" href=\"../source/"@) =~= seq![
        32u8,
        60,
        97,
        32,
        99,
        108,
        97,
        115,
        115,
        61,
        34,
        115,
        111,
        117,
        114,
        99,
        101,
        34,
        32,
        104,
        114,
        101,
        102,
        61,
        34,
        46,
        46,
        47,
        115,
        111,
        117,
        114,
        99,
        101,
        47,
    ]);
    assert(cx::scan(
        seq![
            32u8,
            60,
            97,
            32,
            99,
            108,
            97,
            115,
            115,
            61,
            34,
            115,
            111,
            117,
            114,
            99,
            101,
            34,
            32,
            104,
            114,
            101,
            102,
            61,
            34,
            46,
            46,
            47,
            115,
            111,
            117,
            114,
            99,
            101,
            47,
        ],
        0,
    ) == 2) by (compute_only);
    cx::exact(u::lit(" <a class=\"source\" href=\"../source/"@), 0);
    h::fixed(u::lit(" <a class=\"source\" href=\"../source/"@), inputs, 0, 2);
}

pub proof fn l138(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("\">PDF</a>"@)),
        u::copy_derived(u::lit("\">PDF</a>"@), inputs, u::copy_registry()),
        u::copy_registry()[138] == u::lit("\">PDF</a>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[138] == u::lit("\">PDF</a>"@) && 138 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("\">PDF</a>"@), inputs);
}

pub proof fn f138(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("\">PDF</a>"@), inputs, 2, 0),
        u::copy_registry()[138] == u::lit("\">PDF</a>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l138(inputs);
    reveal_strlit("\">PDF</a>");
    is_ascii_chars_encode_utf8("\">PDF</a>"@);
    assert(u::lit("\">PDF</a>"@) =~= seq![34u8, 62, 80, 68, 70, 60, 47, 97, 62]);
    assert(cx::scan(seq![34u8, 62, 80, 68, 70, 60, 47, 97, 62], 2) == 0) by (compute_only);
    cx::exact(u::lit("\">PDF</a>"@), 2);
    h::fixed(u::lit("\">PDF</a>"@), inputs, 2, 0);
}

pub proof fn l139(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("source/"@)),
        u::copy_derived(u::lit("source/"@), inputs, u::copy_registry()),
        u::copy_registry()[139] == u::lit("source/"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[139] == u::lit("source/"@) && 139 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("source/"@), inputs);
}

pub proof fn l140(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("../source/"@)),
        u::copy_derived(u::lit("../source/"@), inputs, u::copy_registry()),
        u::copy_registry()[140] == u::lit("../source/"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[140] == u::lit("../source/"@) && 140 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("../source/"@), inputs);
}

pub proof fn l141(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("Source text"@)),
        u::copy_derived(u::lit("Source text"@), inputs, u::copy_registry()),
        u::copy_registry()[141] == u::lit("Source text"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[141] == u::lit("Source text"@) && 141 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("Source text"@), inputs);
}

pub proof fn f141(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("Source text"@), inputs, 0, 0),
        u::copy_registry()[141] == u::lit("Source text"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l141(inputs);
    reveal_strlit("Source text");
    is_ascii_chars_encode_utf8("Source text"@);
    assert(u::lit("Source text"@) =~= seq![83u8, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116]);
    assert(cx::scan(seq![83u8, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116], 0) == 0)
        by (compute_only);
    cx::exact(u::lit("Source text"@), 0);
    h::fixed(u::lit("Source text"@), inputs, 0, 0);
}

pub proof fn l142(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("../records.html"@)),
        u::copy_derived(u::lit("../records.html"@), inputs, u::copy_registry()),
        u::copy_registry()[142] == u::lit("../records.html"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[142] == u::lit("../records.html"@) && 142 < u::copy_registry().len())
        by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("../records.html"@), inputs);
}

pub proof fn l143(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("#"@)),
        u::copy_derived(u::lit("#"@), inputs, u::copy_registry()),
        u::copy_registry()[143] == u::lit("#"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[143] == u::lit("#"@) && 143 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("#"@), inputs);
}

pub proof fn l144(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("Previous document"@)),
        u::copy_derived(u::lit("Previous document"@), inputs, u::copy_registry()),
        u::copy_registry()[144] == u::lit("Previous document"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[144] == u::lit("Previous document"@) && 144
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("Previous document"@), inputs);
}

pub proof fn f144(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("Previous document"@), inputs, 0, 0),
        u::copy_registry()[144] == u::lit("Previous document"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l144(inputs);
    reveal_strlit("Previous document");
    is_ascii_chars_encode_utf8("Previous document"@);
    assert(u::lit("Previous document"@) =~= seq![
        80u8,
        114,
        101,
        118,
        105,
        111,
        117,
        115,
        32,
        100,
        111,
        99,
        117,
        109,
        101,
        110,
        116,
    ]);
    assert(cx::scan(
        seq![80u8, 114, 101, 118, 105, 111, 117, 115, 32, 100, 111, 99, 117, 109, 101, 110, 116],
        0,
    ) == 0) by (compute_only);
    cx::exact(u::lit("Previous document"@), 0);
    h::fixed(u::lit("Previous document"@), inputs, 0, 0);
}

pub proof fn l145(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<a href=\"../index.html\">Guideline index</a>"@)),
        u::copy_derived(
            u::lit("<a href=\"../index.html\">Guideline index</a>"@),
            inputs,
            u::copy_registry(),
        ),
        u::copy_registry()[145] == u::lit("<a href=\"../index.html\">Guideline index</a>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[145] == u::lit("<a href=\"../index.html\">Guideline index</a>"@)
        && 145 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<a href=\"../index.html\">Guideline index</a>"@), inputs);
}

pub proof fn f145(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<a href=\"../index.html\">Guideline index</a>"@), inputs, 0, 0),
        u::copy_registry()[145] == u::lit("<a href=\"../index.html\">Guideline index</a>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l145(inputs);
    reveal_strlit("<a href=\"../index.html\">Guideline index</a>");
    is_ascii_chars_encode_utf8("<a href=\"../index.html\">Guideline index</a>"@);
    assert(u::lit("<a href=\"../index.html\">Guideline index</a>"@) =~= seq![
        60u8,
        97,
        32,
        104,
        114,
        101,
        102,
        61,
        34,
        46,
        46,
        47,
        105,
        110,
        100,
        101,
        120,
        46,
        104,
        116,
        109,
        108,
        34,
        62,
        71,
        117,
        105,
        100,
        101,
        108,
        105,
        110,
        101,
        32,
        105,
        110,
        100,
        101,
        120,
        60,
        47,
        97,
        62,
    ]);
    assert(cx::scan(
        seq![
            60u8,
            97,
            32,
            104,
            114,
            101,
            102,
            61,
            34,
            46,
            46,
            47,
            105,
            110,
            100,
            101,
            120,
            46,
            104,
            116,
            109,
            108,
            34,
            62,
            71,
            117,
            105,
            100,
            101,
            108,
            105,
            110,
            101,
            32,
            105,
            110,
            100,
            101,
            120,
            60,
            47,
            97,
            62,
        ],
        0,
    ) == 0) by (compute_only);
    cx::exact(u::lit("<a href=\"../index.html\">Guideline index</a>"@), 0);
    h::fixed(u::lit("<a href=\"../index.html\">Guideline index</a>"@), inputs, 0, 0);
}

pub proof fn l146(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("Next document"@)),
        u::copy_derived(u::lit("Next document"@), inputs, u::copy_registry()),
        u::copy_registry()[146] == u::lit("Next document"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[146] == u::lit("Next document"@) && 146 < u::copy_registry().len())
        by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("Next document"@), inputs);
}

pub proof fn f146(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("Next document"@), inputs, 0, 0),
        u::copy_registry()[146] == u::lit("Next document"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l146(inputs);
    reveal_strlit("Next document");
    is_ascii_chars_encode_utf8("Next document"@);
    assert(u::lit("Next document"@) =~= seq![
        78u8,
        101,
        120,
        116,
        32,
        100,
        111,
        99,
        117,
        109,
        101,
        110,
        116,
    ]);
    assert(cx::scan(seq![78u8, 101, 120, 116, 32, 100, 111, 99, 117, 109, 101, 110, 116], 0) == 0)
        by (compute_only);
    cx::exact(u::lit("Next document"@), 0);
    h::fixed(u::lit("Next document"@), inputs, 0, 0);
}

pub proof fn l147(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("All decision records"@)),
        u::copy_derived(u::lit("All decision records"@), inputs, u::copy_registry()),
        u::copy_registry()[147] == u::lit("All decision records"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[147] == u::lit("All decision records"@) && 147
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("All decision records"@), inputs);
}

pub proof fn f147(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("All decision records"@), inputs, 0, 0),
        u::copy_registry()[147] == u::lit("All decision records"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l147(inputs);
    reveal_strlit("All decision records");
    is_ascii_chars_encode_utf8("All decision records"@);
    assert(u::lit("All decision records"@) =~= seq![
        65u8,
        108,
        108,
        32,
        100,
        101,
        99,
        105,
        115,
        105,
        111,
        110,
        32,
        114,
        101,
        99,
        111,
        114,
        100,
        115,
    ]);
    assert(cx::scan(
        seq![
            65u8,
            108,
            108,
            32,
            100,
            101,
            99,
            105,
            115,
            105,
            111,
            110,
            32,
            114,
            101,
            99,
            111,
            114,
            100,
            115,
        ],
        0,
    ) == 0) by (compute_only);
    cx::exact(u::lit("All decision records"@), 0);
    h::fixed(u::lit("All decision records"@), inputs, 0, 0);
}

pub proof fn l148(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<section class=\"stale\">"@)),
        u::copy_derived(u::lit("<section class=\"stale\">"@), inputs, u::copy_registry()),
        u::copy_registry()[148] == u::lit("<section class=\"stale\">"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[148] == u::lit("<section class=\"stale\">"@) && 148
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<section class=\"stale\">"@), inputs);
}

pub proof fn f148(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<section class=\"stale\">"@), inputs, 0, 0),
        u::copy_registry()[148] == u::lit("<section class=\"stale\">"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l148(inputs);
    reveal_strlit("<section class=\"stale\">");
    is_ascii_chars_encode_utf8("<section class=\"stale\">"@);
    assert(u::lit("<section class=\"stale\">"@) =~= seq![
        60u8,
        115,
        101,
        99,
        116,
        105,
        111,
        110,
        32,
        99,
        108,
        97,
        115,
        115,
        61,
        34,
        115,
        116,
        97,
        108,
        101,
        34,
        62,
    ]);
    assert(cx::scan(
        seq![
            60u8,
            115,
            101,
            99,
            116,
            105,
            111,
            110,
            32,
            99,
            108,
            97,
            115,
            115,
            61,
            34,
            115,
            116,
            97,
            108,
            101,
            34,
            62,
        ],
        0,
    ) == 0) by (compute_only);
    cx::exact(u::lit("<section class=\"stale\">"@), 0);
    h::fixed(u::lit("<section class=\"stale\">"@), inputs, 0, 0);
}

pub proof fn l149(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(
            u::lit(
                "<p>The document or its source changed after the last decision. No recorded decision applies to the version shown here.</p>"@,
            ),
        ),
        u::copy_derived(
            u::lit(
                "<p>The document or its source changed after the last decision. No recorded decision applies to the version shown here.</p>"@,
            ),
            inputs,
            u::copy_registry(),
        ),
        u::copy_registry()[149] == u::lit(
            "<p>The document or its source changed after the last decision. No recorded decision applies to the version shown here.</p>"@,
        ),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[149] == u::lit(
        "<p>The document or its source changed after the last decision. No recorded decision applies to the version shown here.</p>"@,
    ) && 149 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(
        u::lit(
            "<p>The document or its source changed after the last decision. No recorded decision applies to the version shown here.</p>"@,
        ),
        inputs,
    );
}

pub proof fn f149(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(
            u::fixed(
                "<p>The document or its source changed after the last decision. No recorded decision applies to the version shown here.</p>"@,
            ),
            inputs,
            0,
            0,
        ),
        u::copy_registry()[149] == u::lit(
            "<p>The document or its source changed after the last decision. No recorded decision applies to the version shown here.</p>"@,
        ),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l149(inputs);
    reveal_strlit(
        "<p>The document or its source changed after the last decision. No recorded decision applies to the version shown here.</p>",
    );
    is_ascii_chars_encode_utf8(
        "<p>The document or its source changed after the last decision. No recorded decision applies to the version shown here.</p>"@,
    );
    assert(u::lit(
        "<p>The document or its source changed after the last decision. No recorded decision applies to the version shown here.</p>"@,
    ) =~= seq![
        60u8,
        112,
        62,
        84,
        104,
        101,
        32,
        100,
        111,
        99,
        117,
        109,
        101,
        110,
        116,
        32,
        111,
        114,
        32,
        105,
        116,
        115,
        32,
        115,
        111,
        117,
        114,
        99,
        101,
        32,
        99,
        104,
        97,
        110,
        103,
        101,
        100,
        32,
        97,
        102,
        116,
        101,
        114,
        32,
        116,
        104,
        101,
        32,
        108,
        97,
        115,
        116,
        32,
        100,
        101,
        99,
        105,
        115,
        105,
        111,
        110,
        46,
        32,
        78,
        111,
        32,
        114,
        101,
        99,
        111,
        114,
        100,
        101,
        100,
        32,
        100,
        101,
        99,
        105,
        115,
        105,
        111,
        110,
        32,
        97,
        112,
        112,
        108,
        105,
        101,
        115,
        32,
        116,
        111,
        32,
        116,
        104,
        101,
        32,
        118,
        101,
        114,
        115,
        105,
        111,
        110,
        32,
        115,
        104,
        111,
        119,
        110,
        32,
        104,
        101,
        114,
        101,
        46,
        60,
        47,
        112,
        62,
    ]);
    assert(cx::scan(
        seq![
            60u8,
            112,
            62,
            84,
            104,
            101,
            32,
            100,
            111,
            99,
            117,
            109,
            101,
            110,
            116,
            32,
            111,
            114,
            32,
            105,
            116,
            115,
            32,
            115,
            111,
            117,
            114,
            99,
            101,
            32,
            99,
            104,
            97,
            110,
            103,
            101,
            100,
            32,
            97,
            102,
            116,
            101,
            114,
            32,
            116,
            104,
            101,
            32,
            108,
            97,
            115,
            116,
            32,
            100,
            101,
            99,
            105,
            115,
            105,
            111,
            110,
            46,
            32,
            78,
            111,
            32,
            114,
            101,
            99,
            111,
            114,
            100,
            101,
            100,
            32,
            100,
            101,
            99,
            105,
            115,
            105,
            111,
            110,
            32,
            97,
            112,
            112,
            108,
            105,
            101,
            115,
            32,
            116,
            111,
            32,
            116,
            104,
            101,
            32,
            118,
            101,
            114,
            115,
            105,
            111,
            110,
            32,
            115,
            104,
            111,
            119,
            110,
            32,
            104,
            101,
            114,
            101,
            46,
            60,
            47,
            112,
            62,
        ],
        0,
    ) == 0) by (compute_only);
    cx::exact(
        u::lit(
            "<p>The document or its source changed after the last decision. No recorded decision applies to the version shown here.</p>"@,
        ),
        0,
    );
    h::fixed(
        u::lit(
            "<p>The document or its source changed after the last decision. No recorded decision applies to the version shown here.</p>"@,
        ),
        inputs,
        0,
        0,
    );
}

pub proof fn l150(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(
            u::lit(
                "<p class=\"hl-note\"><label><input type=\"checkbox\" class=\"hl-toggle\" checked> Highlighting</label> Try hovering and clicking on highlighted terms for different levels of emphasis.</p>"@,
            ),
        ),
        u::copy_derived(
            u::lit(
                "<p class=\"hl-note\"><label><input type=\"checkbox\" class=\"hl-toggle\" checked> Highlighting</label> Try hovering and clicking on highlighted terms for different levels of emphasis.</p>"@,
            ),
            inputs,
            u::copy_registry(),
        ),
        u::copy_registry()[150] == u::lit(
            "<p class=\"hl-note\"><label><input type=\"checkbox\" class=\"hl-toggle\" checked> Highlighting</label> Try hovering and clicking on highlighted terms for different levels of emphasis.</p>"@,
        ),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[150] == u::lit(
        "<p class=\"hl-note\"><label><input type=\"checkbox\" class=\"hl-toggle\" checked> Highlighting</label> Try hovering and clicking on highlighted terms for different levels of emphasis.</p>"@,
    ) && 150 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(
        u::lit(
            "<p class=\"hl-note\"><label><input type=\"checkbox\" class=\"hl-toggle\" checked> Highlighting</label> Try hovering and clicking on highlighted terms for different levels of emphasis.</p>"@,
        ),
        inputs,
    );
}

pub proof fn f150(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(
            u::fixed(
                "<p class=\"hl-note\"><label><input type=\"checkbox\" class=\"hl-toggle\" checked> Highlighting</label> Try hovering and clicking on highlighted terms for different levels of emphasis.</p>"@,
            ),
            inputs,
            0,
            0,
        ),
        u::copy_registry()[150] == u::lit(
            "<p class=\"hl-note\"><label><input type=\"checkbox\" class=\"hl-toggle\" checked> Highlighting</label> Try hovering and clicking on highlighted terms for different levels of emphasis.</p>"@,
        ),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l150(inputs);
    reveal_strlit(
        "<p class=\"hl-note\"><label><input type=\"checkbox\" class=\"hl-toggle\" checked> Highlighting</label> Try hovering and clicking on highlighted terms for different levels of emphasis.</p>",
    );
    is_ascii_chars_encode_utf8(
        "<p class=\"hl-note\"><label><input type=\"checkbox\" class=\"hl-toggle\" checked> Highlighting</label> Try hovering and clicking on highlighted terms for different levels of emphasis.</p>"@,
    );
    assert(u::lit(
        "<p class=\"hl-note\"><label><input type=\"checkbox\" class=\"hl-toggle\" checked> Highlighting</label> Try hovering and clicking on highlighted terms for different levels of emphasis.</p>"@,
    ) =~= seq![
        60u8,
        112,
        32,
        99,
        108,
        97,
        115,
        115,
        61,
        34,
        104,
        108,
        45,
        110,
        111,
        116,
        101,
        34,
        62,
        60,
        108,
        97,
        98,
        101,
        108,
        62,
        60,
        105,
        110,
        112,
        117,
        116,
        32,
        116,
        121,
        112,
        101,
        61,
        34,
        99,
        104,
        101,
        99,
        107,
        98,
        111,
        120,
        34,
        32,
        99,
        108,
        97,
        115,
        115,
        61,
        34,
        104,
        108,
        45,
        116,
        111,
        103,
        103,
        108,
        101,
        34,
        32,
        99,
        104,
        101,
        99,
        107,
        101,
        100,
        62,
        32,
        72,
        105,
        103,
        104,
        108,
        105,
        103,
        104,
        116,
        105,
        110,
        103,
        60,
        47,
        108,
        97,
        98,
        101,
        108,
        62,
        32,
        84,
        114,
        121,
        32,
        104,
        111,
        118,
        101,
        114,
        105,
        110,
        103,
        32,
        97,
        110,
        100,
        32,
        99,
        108,
        105,
        99,
        107,
        105,
        110,
        103,
        32,
        111,
        110,
        32,
        104,
        105,
        103,
        104,
        108,
        105,
        103,
        104,
        116,
        101,
        100,
        32,
        116,
        101,
        114,
        109,
        115,
        32,
        102,
        111,
        114,
        32,
        100,
        105,
        102,
        102,
        101,
        114,
        101,
        110,
        116,
        32,
        108,
        101,
        118,
        101,
        108,
        115,
        32,
        111,
        102,
        32,
        101,
        109,
        112,
        104,
        97,
        115,
        105,
        115,
        46,
        60,
        47,
        112,
        62,
    ]);
    assert(cx::scan(
        seq![
            60u8,
            112,
            32,
            99,
            108,
            97,
            115,
            115,
            61,
            34,
            104,
            108,
            45,
            110,
            111,
            116,
            101,
            34,
            62,
            60,
            108,
            97,
            98,
            101,
            108,
            62,
            60,
            105,
            110,
            112,
            117,
            116,
            32,
            116,
            121,
            112,
            101,
            61,
            34,
            99,
            104,
            101,
            99,
            107,
            98,
            111,
            120,
            34,
            32,
            99,
            108,
            97,
            115,
            115,
            61,
            34,
            104,
            108,
            45,
            116,
            111,
            103,
            103,
            108,
            101,
            34,
            32,
            99,
            104,
            101,
            99,
            107,
            101,
            100,
            62,
            32,
            72,
            105,
            103,
            104,
            108,
            105,
            103,
            104,
            116,
            105,
            110,
            103,
            60,
            47,
            108,
            97,
            98,
            101,
            108,
            62,
            32,
            84,
            114,
            121,
            32,
            104,
            111,
            118,
            101,
            114,
            105,
            110,
            103,
            32,
            97,
            110,
            100,
            32,
            99,
            108,
            105,
            99,
            107,
            105,
            110,
            103,
            32,
            111,
            110,
            32,
            104,
            105,
            103,
            104,
            108,
            105,
            103,
            104,
            116,
            101,
            100,
            32,
            116,
            101,
            114,
            109,
            115,
            32,
            102,
            111,
            114,
            32,
            100,
            105,
            102,
            102,
            101,
            114,
            101,
            110,
            116,
            32,
            108,
            101,
            118,
            101,
            108,
            115,
            32,
            111,
            102,
            32,
            101,
            109,
            112,
            104,
            97,
            115,
            105,
            115,
            46,
            60,
            47,
            112,
            62,
        ],
        0,
    ) == 0) by (compute_only);
    cx::exact(
        u::lit(
            "<p class=\"hl-note\"><label><input type=\"checkbox\" class=\"hl-toggle\" checked> Highlighting</label> Try hovering and clicking on highlighted terms for different levels of emphasis.</p>"@,
        ),
        0,
    );
    h::fixed(
        u::lit(
            "<p class=\"hl-note\"><label><input type=\"checkbox\" class=\"hl-toggle\" checked> Highlighting</label> Try hovering and clicking on highlighted terms for different levels of emphasis.</p>"@,
        ),
        inputs,
        0,
        0,
    );
}

pub proof fn l151(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<h3>Original passage</h3>"@)),
        u::copy_derived(u::lit("<h3>Original passage</h3>"@), inputs, u::copy_registry()),
        u::copy_registry()[151] == u::lit("<h3>Original passage</h3>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[151] == u::lit("<h3>Original passage</h3>"@) && 151
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<h3>Original passage</h3>"@), inputs);
}

pub proof fn f151(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<h3>Original passage</h3>"@), inputs, 0, 0),
        u::copy_registry()[151] == u::lit("<h3>Original passage</h3>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l151(inputs);
    reveal_strlit("<h3>Original passage</h3>");
    is_ascii_chars_encode_utf8("<h3>Original passage</h3>"@);
    assert(u::lit("<h3>Original passage</h3>"@) =~= seq![
        60u8,
        104,
        51,
        62,
        79,
        114,
        105,
        103,
        105,
        110,
        97,
        108,
        32,
        112,
        97,
        115,
        115,
        97,
        103,
        101,
        60,
        47,
        104,
        51,
        62,
    ]);
    assert(cx::scan(
        seq![
            60u8,
            104,
            51,
            62,
            79,
            114,
            105,
            103,
            105,
            110,
            97,
            108,
            32,
            112,
            97,
            115,
            115,
            97,
            103,
            101,
            60,
            47,
            104,
            51,
            62,
        ],
        0,
    ) == 0) by (compute_only);
    cx::exact(u::lit("<h3>Original passage</h3>"@), 0);
    h::fixed(u::lit("<h3>Original passage</h3>"@), inputs, 0, 0);
}

pub proof fn l152(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<pre class=\"prose\">"@)),
        u::copy_derived(u::lit("<pre class=\"prose\">"@), inputs, u::copy_registry()),
        u::copy_registry()[152] == u::lit("<pre class=\"prose\">"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[152] == u::lit("<pre class=\"prose\">"@) && 152
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<pre class=\"prose\">"@), inputs);
}

pub proof fn f152(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<pre class=\"prose\">"@), inputs, 0, 0),
        u::copy_registry()[152] == u::lit("<pre class=\"prose\">"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l152(inputs);
    reveal_strlit("<pre class=\"prose\">");
    is_ascii_chars_encode_utf8("<pre class=\"prose\">"@);
    assert(u::lit("<pre class=\"prose\">"@) =~= seq![
        60u8,
        112,
        114,
        101,
        32,
        99,
        108,
        97,
        115,
        115,
        61,
        34,
        112,
        114,
        111,
        115,
        101,
        34,
        62,
    ]);
    assert(cx::scan(
        seq![
            60u8,
            112,
            114,
            101,
            32,
            99,
            108,
            97,
            115,
            115,
            61,
            34,
            112,
            114,
            111,
            115,
            101,
            34,
            62,
        ],
        0,
    ) == 0) by (compute_only);
    cx::exact(u::lit("<pre class=\"prose\">"@), 0);
    h::fixed(u::lit("<pre class=\"prose\">"@), inputs, 0, 0);
}

pub proof fn l153(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("</pre>"@)),
        u::copy_derived(u::lit("</pre>"@), inputs, u::copy_registry()),
        u::copy_registry()[153] == u::lit("</pre>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[153] == u::lit("</pre>"@) && 153 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("</pre>"@), inputs);
}

pub proof fn f153(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("</pre>"@), inputs, 0, 0),
        u::copy_registry()[153] == u::lit("</pre>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l153(inputs);
    reveal_strlit("</pre>");
    is_ascii_chars_encode_utf8("</pre>"@);
    assert(u::lit("</pre>"@) =~= seq![60u8, 47, 112, 114, 101, 62]);
    assert(cx::scan(seq![60u8, 47, 112, 114, 101, 62], 0) == 0) by (compute_only);
    cx::exact(u::lit("</pre>"@), 0);
    h::fixed(u::lit("</pre>"@), inputs, 0, 0);
}

pub proof fn l154(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<h3>Attempto Controlled English (ACE)</h3>"@)),
        u::copy_derived(
            u::lit("<h3>Attempto Controlled English (ACE)</h3>"@),
            inputs,
            u::copy_registry(),
        ),
        u::copy_registry()[154] == u::lit("<h3>Attempto Controlled English (ACE)</h3>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[154] == u::lit("<h3>Attempto Controlled English (ACE)</h3>"@) && 154
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<h3>Attempto Controlled English (ACE)</h3>"@), inputs);
}

pub proof fn f154(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<h3>Attempto Controlled English (ACE)</h3>"@), inputs, 0, 0),
        u::copy_registry()[154] == u::lit("<h3>Attempto Controlled English (ACE)</h3>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l154(inputs);
    reveal_strlit("<h3>Attempto Controlled English (ACE)</h3>");
    is_ascii_chars_encode_utf8("<h3>Attempto Controlled English (ACE)</h3>"@);
    assert(u::lit("<h3>Attempto Controlled English (ACE)</h3>"@) =~= seq![
        60u8,
        104,
        51,
        62,
        65,
        116,
        116,
        101,
        109,
        112,
        116,
        111,
        32,
        67,
        111,
        110,
        116,
        114,
        111,
        108,
        108,
        101,
        100,
        32,
        69,
        110,
        103,
        108,
        105,
        115,
        104,
        32,
        40,
        65,
        67,
        69,
        41,
        60,
        47,
        104,
        51,
        62,
    ]);
    assert(cx::scan(
        seq![
            60u8,
            104,
            51,
            62,
            65,
            116,
            116,
            101,
            109,
            112,
            116,
            111,
            32,
            67,
            111,
            110,
            116,
            114,
            111,
            108,
            108,
            101,
            100,
            32,
            69,
            110,
            103,
            108,
            105,
            115,
            104,
            32,
            40,
            65,
            67,
            69,
            41,
            60,
            47,
            104,
            51,
            62,
        ],
        0,
    ) == 0) by (compute_only);
    cx::exact(u::lit("<h3>Attempto Controlled English (ACE)</h3>"@), 0);
    h::fixed(u::lit("<h3>Attempto Controlled English (ACE)</h3>"@), inputs, 0, 0);
}

pub proof fn l155(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<section class=\"verdict-entry\">"@)),
        u::copy_derived(u::lit("<section class=\"verdict-entry\">"@), inputs, u::copy_registry()),
        u::copy_registry()[155] == u::lit("<section class=\"verdict-entry\">"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[155] == u::lit("<section class=\"verdict-entry\">"@) && 155
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<section class=\"verdict-entry\">"@), inputs);
}

pub proof fn f155(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<section class=\"verdict-entry\">"@), inputs, 0, 0),
        u::copy_registry()[155] == u::lit("<section class=\"verdict-entry\">"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l155(inputs);
    reveal_strlit("<section class=\"verdict-entry\">");
    is_ascii_chars_encode_utf8("<section class=\"verdict-entry\">"@);
    assert(u::lit("<section class=\"verdict-entry\">"@) =~= seq![
        60u8,
        115,
        101,
        99,
        116,
        105,
        111,
        110,
        32,
        99,
        108,
        97,
        115,
        115,
        61,
        34,
        118,
        101,
        114,
        100,
        105,
        99,
        116,
        45,
        101,
        110,
        116,
        114,
        121,
        34,
        62,
    ]);
    assert(cx::scan(
        seq![
            60u8,
            115,
            101,
            99,
            116,
            105,
            111,
            110,
            32,
            99,
            108,
            97,
            115,
            115,
            61,
            34,
            118,
            101,
            114,
            100,
            105,
            99,
            116,
            45,
            101,
            110,
            116,
            114,
            121,
            34,
            62,
        ],
        0,
    ) == 0) by (compute_only);
    cx::exact(u::lit("<section class=\"verdict-entry\">"@), 0);
    h::fixed(u::lit("<section class=\"verdict-entry\">"@), inputs, 0, 0);
}

pub proof fn l156(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<h3>Record a decision</h3>"@)),
        u::copy_derived(u::lit("<h3>Record a decision</h3>"@), inputs, u::copy_registry()),
        u::copy_registry()[156] == u::lit("<h3>Record a decision</h3>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[156] == u::lit("<h3>Record a decision</h3>"@) && 156
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<h3>Record a decision</h3>"@), inputs);
}

pub proof fn f156(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<h3>Record a decision</h3>"@), inputs, 0, 0),
        u::copy_registry()[156] == u::lit("<h3>Record a decision</h3>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l156(inputs);
    reveal_strlit("<h3>Record a decision</h3>");
    is_ascii_chars_encode_utf8("<h3>Record a decision</h3>"@);
    assert(u::lit("<h3>Record a decision</h3>"@) =~= seq![
        60u8,
        104,
        51,
        62,
        82,
        101,
        99,
        111,
        114,
        100,
        32,
        97,
        32,
        100,
        101,
        99,
        105,
        115,
        105,
        111,
        110,
        60,
        47,
        104,
        51,
        62,
    ]);
    assert(cx::scan(
        seq![
            60u8,
            104,
            51,
            62,
            82,
            101,
            99,
            111,
            114,
            100,
            32,
            97,
            32,
            100,
            101,
            99,
            105,
            115,
            105,
            111,
            110,
            60,
            47,
            104,
            51,
            62,
        ],
        0,
    ) == 0) by (compute_only);
    cx::exact(u::lit("<h3>Record a decision</h3>"@), 0);
    h::fixed(u::lit("<h3>Record a decision</h3>"@), inputs, 0, 0);
}

pub proof fn l157(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(
            u::lit(
                "<p>Does the ACE representation appropriately reflect the original passage?</p>"@,
            ),
        ),
        u::copy_derived(
            u::lit(
                "<p>Does the ACE representation appropriately reflect the original passage?</p>"@,
            ),
            inputs,
            u::copy_registry(),
        ),
        u::copy_registry()[157] == u::lit(
            "<p>Does the ACE representation appropriately reflect the original passage?</p>"@,
        ),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[157] == u::lit(
        "<p>Does the ACE representation appropriately reflect the original passage?</p>"@,
    ) && 157 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(
        u::lit("<p>Does the ACE representation appropriately reflect the original passage?</p>"@),
        inputs,
    );
}

pub proof fn f157(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(
            u::fixed(
                "<p>Does the ACE representation appropriately reflect the original passage?</p>"@,
            ),
            inputs,
            0,
            0,
        ),
        u::copy_registry()[157] == u::lit(
            "<p>Does the ACE representation appropriately reflect the original passage?</p>"@,
        ),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l157(inputs);
    reveal_strlit("<p>Does the ACE representation appropriately reflect the original passage?</p>");
    is_ascii_chars_encode_utf8(
        "<p>Does the ACE representation appropriately reflect the original passage?</p>"@,
    );
    assert(u::lit("<p>Does the ACE representation appropriately reflect the original passage?</p>"@)
        =~= seq![
        60u8,
        112,
        62,
        68,
        111,
        101,
        115,
        32,
        116,
        104,
        101,
        32,
        65,
        67,
        69,
        32,
        114,
        101,
        112,
        114,
        101,
        115,
        101,
        110,
        116,
        97,
        116,
        105,
        111,
        110,
        32,
        97,
        112,
        112,
        114,
        111,
        112,
        114,
        105,
        97,
        116,
        101,
        108,
        121,
        32,
        114,
        101,
        102,
        108,
        101,
        99,
        116,
        32,
        116,
        104,
        101,
        32,
        111,
        114,
        105,
        103,
        105,
        110,
        97,
        108,
        32,
        112,
        97,
        115,
        115,
        97,
        103,
        101,
        63,
        60,
        47,
        112,
        62,
    ]);
    assert(cx::scan(
        seq![
            60u8,
            112,
            62,
            68,
            111,
            101,
            115,
            32,
            116,
            104,
            101,
            32,
            65,
            67,
            69,
            32,
            114,
            101,
            112,
            114,
            101,
            115,
            101,
            110,
            116,
            97,
            116,
            105,
            111,
            110,
            32,
            97,
            112,
            112,
            114,
            111,
            112,
            114,
            105,
            97,
            116,
            101,
            108,
            121,
            32,
            114,
            101,
            102,
            108,
            101,
            99,
            116,
            32,
            116,
            104,
            101,
            32,
            111,
            114,
            105,
            103,
            105,
            110,
            97,
            108,
            32,
            112,
            97,
            115,
            115,
            97,
            103,
            101,
            63,
            60,
            47,
            112,
            62,
        ],
        0,
    ) == 0) by (compute_only);
    cx::exact(
        u::lit("<p>Does the ACE representation appropriately reflect the original passage?</p>"@),
        0,
    );
    h::fixed(
        u::lit("<p>Does the ACE representation appropriately reflect the original passage?</p>"@),
        inputs,
        0,
        0,
    );
}

pub proof fn l158(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<form method=\"post\">"@)),
        u::copy_derived(u::lit("<form method=\"post\">"@), inputs, u::copy_registry()),
        u::copy_registry()[158] == u::lit("<form method=\"post\">"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[158] == u::lit("<form method=\"post\">"@) && 158
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<form method=\"post\">"@), inputs);
}

pub proof fn f158(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<form method=\"post\">"@), inputs, 0, 0),
        u::copy_registry()[158] == u::lit("<form method=\"post\">"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l158(inputs);
    reveal_strlit("<form method=\"post\">");
    is_ascii_chars_encode_utf8("<form method=\"post\">"@);
    assert(u::lit("<form method=\"post\">"@) =~= seq![
        60u8,
        102,
        111,
        114,
        109,
        32,
        109,
        101,
        116,
        104,
        111,
        100,
        61,
        34,
        112,
        111,
        115,
        116,
        34,
        62,
    ]);
    assert(cx::scan(
        seq![
            60u8,
            102,
            111,
            114,
            109,
            32,
            109,
            101,
            116,
            104,
            111,
            100,
            61,
            34,
            112,
            111,
            115,
            116,
            34,
            62,
        ],
        0,
    ) == 0) by (compute_only);
    cx::exact(u::lit("<form method=\"post\">"@), 0);
    h::fixed(u::lit("<form method=\"post\">"@), inputs, 0, 0);
}

pub proof fn l159(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<fieldset>"@)),
        u::copy_derived(u::lit("<fieldset>"@), inputs, u::copy_registry()),
        u::copy_registry()[159] == u::lit("<fieldset>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[159] == u::lit("<fieldset>"@) && 159 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<fieldset>"@), inputs);
}

pub proof fn f159(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<fieldset>"@), inputs, 0, 0),
        u::copy_registry()[159] == u::lit("<fieldset>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l159(inputs);
    reveal_strlit("<fieldset>");
    is_ascii_chars_encode_utf8("<fieldset>"@);
    assert(u::lit("<fieldset>"@) =~= seq![60u8, 102, 105, 101, 108, 100, 115, 101, 116, 62]);
    assert(cx::scan(seq![60u8, 102, 105, 101, 108, 100, 115, 101, 116, 62], 0) == 0)
        by (compute_only);
    cx::exact(u::lit("<fieldset>"@), 0);
    h::fixed(u::lit("<fieldset>"@), inputs, 0, 0);
}

pub proof fn l160(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<legend>Decision</legend>"@)),
        u::copy_derived(u::lit("<legend>Decision</legend>"@), inputs, u::copy_registry()),
        u::copy_registry()[160] == u::lit("<legend>Decision</legend>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[160] == u::lit("<legend>Decision</legend>"@) && 160
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<legend>Decision</legend>"@), inputs);
}

pub proof fn f160(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<legend>Decision</legend>"@), inputs, 0, 0),
        u::copy_registry()[160] == u::lit("<legend>Decision</legend>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l160(inputs);
    reveal_strlit("<legend>Decision</legend>");
    is_ascii_chars_encode_utf8("<legend>Decision</legend>"@);
    assert(u::lit("<legend>Decision</legend>"@) =~= seq![
        60u8,
        108,
        101,
        103,
        101,
        110,
        100,
        62,
        68,
        101,
        99,
        105,
        115,
        105,
        111,
        110,
        60,
        47,
        108,
        101,
        103,
        101,
        110,
        100,
        62,
    ]);
    assert(cx::scan(
        seq![
            60u8,
            108,
            101,
            103,
            101,
            110,
            100,
            62,
            68,
            101,
            99,
            105,
            115,
            105,
            111,
            110,
            60,
            47,
            108,
            101,
            103,
            101,
            110,
            100,
            62,
        ],
        0,
    ) == 0) by (compute_only);
    cx::exact(u::lit("<legend>Decision</legend>"@), 0);
    h::fixed(u::lit("<legend>Decision</legend>"@), inputs, 0, 0);
}

pub proof fn l161(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(
            u::lit(
                "<label><input type=\"radio\" name=\"verdict\" value=\"approved\" required> Approved</label>"@,
            ),
        ),
        u::copy_derived(
            u::lit(
                "<label><input type=\"radio\" name=\"verdict\" value=\"approved\" required> Approved</label>"@,
            ),
            inputs,
            u::copy_registry(),
        ),
        u::copy_registry()[161] == u::lit(
            "<label><input type=\"radio\" name=\"verdict\" value=\"approved\" required> Approved</label>"@,
        ),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[161] == u::lit(
        "<label><input type=\"radio\" name=\"verdict\" value=\"approved\" required> Approved</label>"@,
    ) && 161 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(
        u::lit(
            "<label><input type=\"radio\" name=\"verdict\" value=\"approved\" required> Approved</label>"@,
        ),
        inputs,
    );
}

pub proof fn f161(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(
            u::fixed(
                "<label><input type=\"radio\" name=\"verdict\" value=\"approved\" required> Approved</label>"@,
            ),
            inputs,
            0,
            0,
        ),
        u::copy_registry()[161] == u::lit(
            "<label><input type=\"radio\" name=\"verdict\" value=\"approved\" required> Approved</label>"@,
        ),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l161(inputs);
    reveal_strlit(
        "<label><input type=\"radio\" name=\"verdict\" value=\"approved\" required> Approved</label>",
    );
    is_ascii_chars_encode_utf8(
        "<label><input type=\"radio\" name=\"verdict\" value=\"approved\" required> Approved</label>"@,
    );
    assert(u::lit(
        "<label><input type=\"radio\" name=\"verdict\" value=\"approved\" required> Approved</label>"@,
    ) =~= seq![
        60u8,
        108,
        97,
        98,
        101,
        108,
        62,
        60,
        105,
        110,
        112,
        117,
        116,
        32,
        116,
        121,
        112,
        101,
        61,
        34,
        114,
        97,
        100,
        105,
        111,
        34,
        32,
        110,
        97,
        109,
        101,
        61,
        34,
        118,
        101,
        114,
        100,
        105,
        99,
        116,
        34,
        32,
        118,
        97,
        108,
        117,
        101,
        61,
        34,
        97,
        112,
        112,
        114,
        111,
        118,
        101,
        100,
        34,
        32,
        114,
        101,
        113,
        117,
        105,
        114,
        101,
        100,
        62,
        32,
        65,
        112,
        112,
        114,
        111,
        118,
        101,
        100,
        60,
        47,
        108,
        97,
        98,
        101,
        108,
        62,
    ]);
    assert(cx::scan(
        seq![
            60u8,
            108,
            97,
            98,
            101,
            108,
            62,
            60,
            105,
            110,
            112,
            117,
            116,
            32,
            116,
            121,
            112,
            101,
            61,
            34,
            114,
            97,
            100,
            105,
            111,
            34,
            32,
            110,
            97,
            109,
            101,
            61,
            34,
            118,
            101,
            114,
            100,
            105,
            99,
            116,
            34,
            32,
            118,
            97,
            108,
            117,
            101,
            61,
            34,
            97,
            112,
            112,
            114,
            111,
            118,
            101,
            100,
            34,
            32,
            114,
            101,
            113,
            117,
            105,
            114,
            101,
            100,
            62,
            32,
            65,
            112,
            112,
            114,
            111,
            118,
            101,
            100,
            60,
            47,
            108,
            97,
            98,
            101,
            108,
            62,
        ],
        0,
    ) == 0) by (compute_only);
    cx::exact(
        u::lit(
            "<label><input type=\"radio\" name=\"verdict\" value=\"approved\" required> Approved</label>"@,
        ),
        0,
    );
    h::fixed(
        u::lit(
            "<label><input type=\"radio\" name=\"verdict\" value=\"approved\" required> Approved</label>"@,
        ),
        inputs,
        0,
        0,
    );
}

pub proof fn l162(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(
            u::lit(
                "<label><input type=\"radio\" name=\"verdict\" value=\"rejected\" required> Rejected</label>"@,
            ),
        ),
        u::copy_derived(
            u::lit(
                "<label><input type=\"radio\" name=\"verdict\" value=\"rejected\" required> Rejected</label>"@,
            ),
            inputs,
            u::copy_registry(),
        ),
        u::copy_registry()[162] == u::lit(
            "<label><input type=\"radio\" name=\"verdict\" value=\"rejected\" required> Rejected</label>"@,
        ),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[162] == u::lit(
        "<label><input type=\"radio\" name=\"verdict\" value=\"rejected\" required> Rejected</label>"@,
    ) && 162 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(
        u::lit(
            "<label><input type=\"radio\" name=\"verdict\" value=\"rejected\" required> Rejected</label>"@,
        ),
        inputs,
    );
}

pub proof fn f162(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(
            u::fixed(
                "<label><input type=\"radio\" name=\"verdict\" value=\"rejected\" required> Rejected</label>"@,
            ),
            inputs,
            0,
            0,
        ),
        u::copy_registry()[162] == u::lit(
            "<label><input type=\"radio\" name=\"verdict\" value=\"rejected\" required> Rejected</label>"@,
        ),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l162(inputs);
    reveal_strlit(
        "<label><input type=\"radio\" name=\"verdict\" value=\"rejected\" required> Rejected</label>",
    );
    is_ascii_chars_encode_utf8(
        "<label><input type=\"radio\" name=\"verdict\" value=\"rejected\" required> Rejected</label>"@,
    );
    assert(u::lit(
        "<label><input type=\"radio\" name=\"verdict\" value=\"rejected\" required> Rejected</label>"@,
    ) =~= seq![
        60u8,
        108,
        97,
        98,
        101,
        108,
        62,
        60,
        105,
        110,
        112,
        117,
        116,
        32,
        116,
        121,
        112,
        101,
        61,
        34,
        114,
        97,
        100,
        105,
        111,
        34,
        32,
        110,
        97,
        109,
        101,
        61,
        34,
        118,
        101,
        114,
        100,
        105,
        99,
        116,
        34,
        32,
        118,
        97,
        108,
        117,
        101,
        61,
        34,
        114,
        101,
        106,
        101,
        99,
        116,
        101,
        100,
        34,
        32,
        114,
        101,
        113,
        117,
        105,
        114,
        101,
        100,
        62,
        32,
        82,
        101,
        106,
        101,
        99,
        116,
        101,
        100,
        60,
        47,
        108,
        97,
        98,
        101,
        108,
        62,
    ]);
    assert(cx::scan(
        seq![
            60u8,
            108,
            97,
            98,
            101,
            108,
            62,
            60,
            105,
            110,
            112,
            117,
            116,
            32,
            116,
            121,
            112,
            101,
            61,
            34,
            114,
            97,
            100,
            105,
            111,
            34,
            32,
            110,
            97,
            109,
            101,
            61,
            34,
            118,
            101,
            114,
            100,
            105,
            99,
            116,
            34,
            32,
            118,
            97,
            108,
            117,
            101,
            61,
            34,
            114,
            101,
            106,
            101,
            99,
            116,
            101,
            100,
            34,
            32,
            114,
            101,
            113,
            117,
            105,
            114,
            101,
            100,
            62,
            32,
            82,
            101,
            106,
            101,
            99,
            116,
            101,
            100,
            60,
            47,
            108,
            97,
            98,
            101,
            108,
            62,
        ],
        0,
    ) == 0) by (compute_only);
    cx::exact(
        u::lit(
            "<label><input type=\"radio\" name=\"verdict\" value=\"rejected\" required> Rejected</label>"@,
        ),
        0,
    );
    h::fixed(
        u::lit(
            "<label><input type=\"radio\" name=\"verdict\" value=\"rejected\" required> Rejected</label>"@,
        ),
        inputs,
        0,
        0,
    );
}

pub proof fn l163(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("</fieldset>"@)),
        u::copy_derived(u::lit("</fieldset>"@), inputs, u::copy_registry()),
        u::copy_registry()[163] == u::lit("</fieldset>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[163] == u::lit("</fieldset>"@) && 163 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("</fieldset>"@), inputs);
}

pub proof fn f163(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("</fieldset>"@), inputs, 0, 0),
        u::copy_registry()[163] == u::lit("</fieldset>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l163(inputs);
    reveal_strlit("</fieldset>");
    is_ascii_chars_encode_utf8("</fieldset>"@);
    assert(u::lit("</fieldset>"@) =~= seq![60u8, 47, 102, 105, 101, 108, 100, 115, 101, 116, 62]);
    assert(cx::scan(seq![60u8, 47, 102, 105, 101, 108, 100, 115, 101, 116, 62], 0) == 0)
        by (compute_only);
    cx::exact(u::lit("</fieldset>"@), 0);
    h::fixed(u::lit("</fieldset>"@), inputs, 0, 0);
}

pub proof fn l164(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<label for=\"reviewer\">Reviewer name</label>"@)),
        u::copy_derived(
            u::lit("<label for=\"reviewer\">Reviewer name</label>"@),
            inputs,
            u::copy_registry(),
        ),
        u::copy_registry()[164] == u::lit("<label for=\"reviewer\">Reviewer name</label>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[164] == u::lit("<label for=\"reviewer\">Reviewer name</label>"@)
        && 164 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<label for=\"reviewer\">Reviewer name</label>"@), inputs);
}

pub proof fn f164(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<label for=\"reviewer\">Reviewer name</label>"@), inputs, 0, 0),
        u::copy_registry()[164] == u::lit("<label for=\"reviewer\">Reviewer name</label>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l164(inputs);
    reveal_strlit("<label for=\"reviewer\">Reviewer name</label>");
    is_ascii_chars_encode_utf8("<label for=\"reviewer\">Reviewer name</label>"@);
    assert(u::lit("<label for=\"reviewer\">Reviewer name</label>"@) =~= seq![
        60u8,
        108,
        97,
        98,
        101,
        108,
        32,
        102,
        111,
        114,
        61,
        34,
        114,
        101,
        118,
        105,
        101,
        119,
        101,
        114,
        34,
        62,
        82,
        101,
        118,
        105,
        101,
        119,
        101,
        114,
        32,
        110,
        97,
        109,
        101,
        60,
        47,
        108,
        97,
        98,
        101,
        108,
        62,
    ]);
    assert(cx::scan(
        seq![
            60u8,
            108,
            97,
            98,
            101,
            108,
            32,
            102,
            111,
            114,
            61,
            34,
            114,
            101,
            118,
            105,
            101,
            119,
            101,
            114,
            34,
            62,
            82,
            101,
            118,
            105,
            101,
            119,
            101,
            114,
            32,
            110,
            97,
            109,
            101,
            60,
            47,
            108,
            97,
            98,
            101,
            108,
            62,
        ],
        0,
    ) == 0) by (compute_only);
    cx::exact(u::lit("<label for=\"reviewer\">Reviewer name</label>"@), 0);
    h::fixed(u::lit("<label for=\"reviewer\">Reviewer name</label>"@), inputs, 0, 0);
}

pub proof fn l165(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(
            u::lit(
                "<input type=\"text\" id=\"reviewer\" name=\"reviewer\" list=\"reviewer-names\" value=\""@,
            ),
        ),
        u::copy_derived(
            u::lit(
                "<input type=\"text\" id=\"reviewer\" name=\"reviewer\" list=\"reviewer-names\" value=\""@,
            ),
            inputs,
            u::copy_registry(),
        ),
        u::copy_registry()[165] == u::lit(
            "<input type=\"text\" id=\"reviewer\" name=\"reviewer\" list=\"reviewer-names\" value=\""@,
        ),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[165] == u::lit(
        "<input type=\"text\" id=\"reviewer\" name=\"reviewer\" list=\"reviewer-names\" value=\""@,
    ) && 165 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(
        u::lit(
            "<input type=\"text\" id=\"reviewer\" name=\"reviewer\" list=\"reviewer-names\" value=\""@,
        ),
        inputs,
    );
}

pub proof fn f165(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(
            u::fixed(
                "<input type=\"text\" id=\"reviewer\" name=\"reviewer\" list=\"reviewer-names\" value=\""@,
            ),
            inputs,
            0,
            2,
        ),
        u::copy_registry()[165] == u::lit(
            "<input type=\"text\" id=\"reviewer\" name=\"reviewer\" list=\"reviewer-names\" value=\""@,
        ),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l165(inputs);
    reveal_strlit(
        "<input type=\"text\" id=\"reviewer\" name=\"reviewer\" list=\"reviewer-names\" value=\"",
    );
    is_ascii_chars_encode_utf8(
        "<input type=\"text\" id=\"reviewer\" name=\"reviewer\" list=\"reviewer-names\" value=\""@,
    );
    assert(u::lit(
        "<input type=\"text\" id=\"reviewer\" name=\"reviewer\" list=\"reviewer-names\" value=\""@,
    ) =~= seq![
        60u8,
        105,
        110,
        112,
        117,
        116,
        32,
        116,
        121,
        112,
        101,
        61,
        34,
        116,
        101,
        120,
        116,
        34,
        32,
        105,
        100,
        61,
        34,
        114,
        101,
        118,
        105,
        101,
        119,
        101,
        114,
        34,
        32,
        110,
        97,
        109,
        101,
        61,
        34,
        114,
        101,
        118,
        105,
        101,
        119,
        101,
        114,
        34,
        32,
        108,
        105,
        115,
        116,
        61,
        34,
        114,
        101,
        118,
        105,
        101,
        119,
        101,
        114,
        45,
        110,
        97,
        109,
        101,
        115,
        34,
        32,
        118,
        97,
        108,
        117,
        101,
        61,
        34,
    ]);
    assert(cx::scan(
        seq![
            60u8,
            105,
            110,
            112,
            117,
            116,
            32,
            116,
            121,
            112,
            101,
            61,
            34,
            116,
            101,
            120,
            116,
            34,
            32,
            105,
            100,
            61,
            34,
            114,
            101,
            118,
            105,
            101,
            119,
            101,
            114,
            34,
            32,
            110,
            97,
            109,
            101,
            61,
            34,
            114,
            101,
            118,
            105,
            101,
            119,
            101,
            114,
            34,
            32,
            108,
            105,
            115,
            116,
            61,
            34,
            114,
            101,
            118,
            105,
            101,
            119,
            101,
            114,
            45,
            110,
            97,
            109,
            101,
            115,
            34,
            32,
            118,
            97,
            108,
            117,
            101,
            61,
            34,
        ],
        0,
    ) == 2) by (compute_only);
    cx::exact(
        u::lit(
            "<input type=\"text\" id=\"reviewer\" name=\"reviewer\" list=\"reviewer-names\" value=\""@,
        ),
        0,
    );
    h::fixed(
        u::lit(
            "<input type=\"text\" id=\"reviewer\" name=\"reviewer\" list=\"reviewer-names\" value=\""@,
        ),
        inputs,
        0,
        2,
    );
}

pub proof fn l166(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("\" required>"@)),
        u::copy_derived(u::lit("\" required>"@), inputs, u::copy_registry()),
        u::copy_registry()[166] == u::lit("\" required>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[166] == u::lit("\" required>"@) && 166 < u::copy_registry().len())
        by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("\" required>"@), inputs);
}

pub proof fn f166(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("\" required>"@), inputs, 2, 0),
        u::copy_registry()[166] == u::lit("\" required>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l166(inputs);
    reveal_strlit("\" required>");
    is_ascii_chars_encode_utf8("\" required>"@);
    assert(u::lit("\" required>"@) =~= seq![34u8, 32, 114, 101, 113, 117, 105, 114, 101, 100, 62]);
    assert(cx::scan(seq![34u8, 32, 114, 101, 113, 117, 105, 114, 101, 100, 62], 2) == 0)
        by (compute_only);
    cx::exact(u::lit("\" required>"@), 2);
    h::fixed(u::lit("\" required>"@), inputs, 2, 0);
}

pub proof fn l167(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<label for=\"comment\">Comment (optional)</label>"@)),
        u::copy_derived(
            u::lit("<label for=\"comment\">Comment (optional)</label>"@),
            inputs,
            u::copy_registry(),
        ),
        u::copy_registry()[167] == u::lit("<label for=\"comment\">Comment (optional)</label>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[167] == u::lit("<label for=\"comment\">Comment (optional)</label>"@)
        && 167 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<label for=\"comment\">Comment (optional)</label>"@), inputs);
}

pub proof fn f167(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<label for=\"comment\">Comment (optional)</label>"@), inputs, 0, 0),
        u::copy_registry()[167] == u::lit("<label for=\"comment\">Comment (optional)</label>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l167(inputs);
    reveal_strlit("<label for=\"comment\">Comment (optional)</label>");
    is_ascii_chars_encode_utf8("<label for=\"comment\">Comment (optional)</label>"@);
    assert(u::lit("<label for=\"comment\">Comment (optional)</label>"@) =~= seq![
        60u8,
        108,
        97,
        98,
        101,
        108,
        32,
        102,
        111,
        114,
        61,
        34,
        99,
        111,
        109,
        109,
        101,
        110,
        116,
        34,
        62,
        67,
        111,
        109,
        109,
        101,
        110,
        116,
        32,
        40,
        111,
        112,
        116,
        105,
        111,
        110,
        97,
        108,
        41,
        60,
        47,
        108,
        97,
        98,
        101,
        108,
        62,
    ]);
    assert(cx::scan(
        seq![
            60u8,
            108,
            97,
            98,
            101,
            108,
            32,
            102,
            111,
            114,
            61,
            34,
            99,
            111,
            109,
            109,
            101,
            110,
            116,
            34,
            62,
            67,
            111,
            109,
            109,
            101,
            110,
            116,
            32,
            40,
            111,
            112,
            116,
            105,
            111,
            110,
            97,
            108,
            41,
            60,
            47,
            108,
            97,
            98,
            101,
            108,
            62,
        ],
        0,
    ) == 0) by (compute_only);
    cx::exact(u::lit("<label for=\"comment\">Comment (optional)</label>"@), 0);
    h::fixed(u::lit("<label for=\"comment\">Comment (optional)</label>"@), inputs, 0, 0);
}

pub proof fn l168(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(
            u::lit("<textarea id=\"comment\" name=\"comment\"></textarea>"@),
        ),
        u::copy_derived(
            u::lit("<textarea id=\"comment\" name=\"comment\"></textarea>"@),
            inputs,
            u::copy_registry(),
        ),
        u::copy_registry()[168] == u::lit("<textarea id=\"comment\" name=\"comment\"></textarea>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[168] == u::lit(
        "<textarea id=\"comment\" name=\"comment\"></textarea>"@,
    ) && 168 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<textarea id=\"comment\" name=\"comment\"></textarea>"@), inputs);
}

pub proof fn f168(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(
            u::fixed("<textarea id=\"comment\" name=\"comment\"></textarea>"@),
            inputs,
            0,
            0,
        ),
        u::copy_registry()[168] == u::lit("<textarea id=\"comment\" name=\"comment\"></textarea>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l168(inputs);
    reveal_strlit("<textarea id=\"comment\" name=\"comment\"></textarea>");
    is_ascii_chars_encode_utf8("<textarea id=\"comment\" name=\"comment\"></textarea>"@);
    assert(u::lit("<textarea id=\"comment\" name=\"comment\"></textarea>"@) =~= seq![
        60u8,
        116,
        101,
        120,
        116,
        97,
        114,
        101,
        97,
        32,
        105,
        100,
        61,
        34,
        99,
        111,
        109,
        109,
        101,
        110,
        116,
        34,
        32,
        110,
        97,
        109,
        101,
        61,
        34,
        99,
        111,
        109,
        109,
        101,
        110,
        116,
        34,
        62,
        60,
        47,
        116,
        101,
        120,
        116,
        97,
        114,
        101,
        97,
        62,
    ]);
    assert(cx::scan(
        seq![
            60u8,
            116,
            101,
            120,
            116,
            97,
            114,
            101,
            97,
            32,
            105,
            100,
            61,
            34,
            99,
            111,
            109,
            109,
            101,
            110,
            116,
            34,
            32,
            110,
            97,
            109,
            101,
            61,
            34,
            99,
            111,
            109,
            109,
            101,
            110,
            116,
            34,
            62,
            60,
            47,
            116,
            101,
            120,
            116,
            97,
            114,
            101,
            97,
            62,
        ],
        0,
    ) == 0) by (compute_only);
    cx::exact(u::lit("<textarea id=\"comment\" name=\"comment\"></textarea>"@), 0);
    h::fixed(u::lit("<textarea id=\"comment\" name=\"comment\"></textarea>"@), inputs, 0, 0);
}

pub proof fn l169(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(
            u::lit("<input type=\"hidden\" name=\"review_sha256\" value=\""@),
        ),
        u::copy_derived(
            u::lit("<input type=\"hidden\" name=\"review_sha256\" value=\""@),
            inputs,
            u::copy_registry(),
        ),
        u::copy_registry()[169] == u::lit(
            "<input type=\"hidden\" name=\"review_sha256\" value=\""@,
        ),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[169] == u::lit(
        "<input type=\"hidden\" name=\"review_sha256\" value=\""@,
    ) && 169 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<input type=\"hidden\" name=\"review_sha256\" value=\""@), inputs);
}

pub proof fn f169(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(
            u::fixed("<input type=\"hidden\" name=\"review_sha256\" value=\""@),
            inputs,
            0,
            2,
        ),
        u::copy_registry()[169] == u::lit(
            "<input type=\"hidden\" name=\"review_sha256\" value=\""@,
        ),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l169(inputs);
    reveal_strlit("<input type=\"hidden\" name=\"review_sha256\" value=\"");
    is_ascii_chars_encode_utf8("<input type=\"hidden\" name=\"review_sha256\" value=\""@);
    assert(u::lit("<input type=\"hidden\" name=\"review_sha256\" value=\""@) =~= seq![
        60u8,
        105,
        110,
        112,
        117,
        116,
        32,
        116,
        121,
        112,
        101,
        61,
        34,
        104,
        105,
        100,
        100,
        101,
        110,
        34,
        32,
        110,
        97,
        109,
        101,
        61,
        34,
        114,
        101,
        118,
        105,
        101,
        119,
        95,
        115,
        104,
        97,
        50,
        53,
        54,
        34,
        32,
        118,
        97,
        108,
        117,
        101,
        61,
        34,
    ]);
    assert(cx::scan(
        seq![
            60u8,
            105,
            110,
            112,
            117,
            116,
            32,
            116,
            121,
            112,
            101,
            61,
            34,
            104,
            105,
            100,
            100,
            101,
            110,
            34,
            32,
            110,
            97,
            109,
            101,
            61,
            34,
            114,
            101,
            118,
            105,
            101,
            119,
            95,
            115,
            104,
            97,
            50,
            53,
            54,
            34,
            32,
            118,
            97,
            108,
            117,
            101,
            61,
            34,
        ],
        0,
    ) == 2) by (compute_only);
    cx::exact(u::lit("<input type=\"hidden\" name=\"review_sha256\" value=\""@), 0);
    h::fixed(u::lit("<input type=\"hidden\" name=\"review_sha256\" value=\""@), inputs, 0, 2);
}

pub proof fn l170(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(
            u::lit("<input type=\"hidden\" name=\"ledger_sha256\" value=\""@),
        ),
        u::copy_derived(
            u::lit("<input type=\"hidden\" name=\"ledger_sha256\" value=\""@),
            inputs,
            u::copy_registry(),
        ),
        u::copy_registry()[170] == u::lit(
            "<input type=\"hidden\" name=\"ledger_sha256\" value=\""@,
        ),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[170] == u::lit(
        "<input type=\"hidden\" name=\"ledger_sha256\" value=\""@,
    ) && 170 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<input type=\"hidden\" name=\"ledger_sha256\" value=\""@), inputs);
}

pub proof fn f170(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(
            u::fixed("<input type=\"hidden\" name=\"ledger_sha256\" value=\""@),
            inputs,
            0,
            2,
        ),
        u::copy_registry()[170] == u::lit(
            "<input type=\"hidden\" name=\"ledger_sha256\" value=\""@,
        ),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l170(inputs);
    reveal_strlit("<input type=\"hidden\" name=\"ledger_sha256\" value=\"");
    is_ascii_chars_encode_utf8("<input type=\"hidden\" name=\"ledger_sha256\" value=\""@);
    assert(u::lit("<input type=\"hidden\" name=\"ledger_sha256\" value=\""@) =~= seq![
        60u8,
        105,
        110,
        112,
        117,
        116,
        32,
        116,
        121,
        112,
        101,
        61,
        34,
        104,
        105,
        100,
        100,
        101,
        110,
        34,
        32,
        110,
        97,
        109,
        101,
        61,
        34,
        108,
        101,
        100,
        103,
        101,
        114,
        95,
        115,
        104,
        97,
        50,
        53,
        54,
        34,
        32,
        118,
        97,
        108,
        117,
        101,
        61,
        34,
    ]);
    assert(cx::scan(
        seq![
            60u8,
            105,
            110,
            112,
            117,
            116,
            32,
            116,
            121,
            112,
            101,
            61,
            34,
            104,
            105,
            100,
            100,
            101,
            110,
            34,
            32,
            110,
            97,
            109,
            101,
            61,
            34,
            108,
            101,
            100,
            103,
            101,
            114,
            95,
            115,
            104,
            97,
            50,
            53,
            54,
            34,
            32,
            118,
            97,
            108,
            117,
            101,
            61,
            34,
        ],
        0,
    ) == 2) by (compute_only);
    cx::exact(u::lit("<input type=\"hidden\" name=\"ledger_sha256\" value=\""@), 0);
    h::fixed(u::lit("<input type=\"hidden\" name=\"ledger_sha256\" value=\""@), inputs, 0, 2);
}

pub proof fn l171(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<input type=\"hidden\" name=\"csrf\" value=\""@)),
        u::copy_derived(
            u::lit("<input type=\"hidden\" name=\"csrf\" value=\""@),
            inputs,
            u::copy_registry(),
        ),
        u::copy_registry()[171] == u::lit("<input type=\"hidden\" name=\"csrf\" value=\""@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[171] == u::lit("<input type=\"hidden\" name=\"csrf\" value=\""@)
        && 171 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<input type=\"hidden\" name=\"csrf\" value=\""@), inputs);
}

pub proof fn f171(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<input type=\"hidden\" name=\"csrf\" value=\""@), inputs, 0, 2),
        u::copy_registry()[171] == u::lit("<input type=\"hidden\" name=\"csrf\" value=\""@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l171(inputs);
    reveal_strlit("<input type=\"hidden\" name=\"csrf\" value=\"");
    is_ascii_chars_encode_utf8("<input type=\"hidden\" name=\"csrf\" value=\""@);
    assert(u::lit("<input type=\"hidden\" name=\"csrf\" value=\""@) =~= seq![
        60u8,
        105,
        110,
        112,
        117,
        116,
        32,
        116,
        121,
        112,
        101,
        61,
        34,
        104,
        105,
        100,
        100,
        101,
        110,
        34,
        32,
        110,
        97,
        109,
        101,
        61,
        34,
        99,
        115,
        114,
        102,
        34,
        32,
        118,
        97,
        108,
        117,
        101,
        61,
        34,
    ]);
    assert(cx::scan(
        seq![
            60u8,
            105,
            110,
            112,
            117,
            116,
            32,
            116,
            121,
            112,
            101,
            61,
            34,
            104,
            105,
            100,
            100,
            101,
            110,
            34,
            32,
            110,
            97,
            109,
            101,
            61,
            34,
            99,
            115,
            114,
            102,
            34,
            32,
            118,
            97,
            108,
            117,
            101,
            61,
            34,
        ],
        0,
    ) == 2) by (compute_only);
    cx::exact(u::lit("<input type=\"hidden\" name=\"csrf\" value=\""@), 0);
    h::fixed(u::lit("<input type=\"hidden\" name=\"csrf\" value=\""@), inputs, 0, 2);
}

pub proof fn l172(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<button>Record decision</button>"@)),
        u::copy_derived(u::lit("<button>Record decision</button>"@), inputs, u::copy_registry()),
        u::copy_registry()[172] == u::lit("<button>Record decision</button>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[172] == u::lit("<button>Record decision</button>"@) && 172
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<button>Record decision</button>"@), inputs);
}

pub proof fn f172(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<button>Record decision</button>"@), inputs, 0, 0),
        u::copy_registry()[172] == u::lit("<button>Record decision</button>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l172(inputs);
    reveal_strlit("<button>Record decision</button>");
    is_ascii_chars_encode_utf8("<button>Record decision</button>"@);
    assert(u::lit("<button>Record decision</button>"@) =~= seq![
        60u8,
        98,
        117,
        116,
        116,
        111,
        110,
        62,
        82,
        101,
        99,
        111,
        114,
        100,
        32,
        100,
        101,
        99,
        105,
        115,
        105,
        111,
        110,
        60,
        47,
        98,
        117,
        116,
        116,
        111,
        110,
        62,
    ]);
    assert(cx::scan(
        seq![
            60u8,
            98,
            117,
            116,
            116,
            111,
            110,
            62,
            82,
            101,
            99,
            111,
            114,
            100,
            32,
            100,
            101,
            99,
            105,
            115,
            105,
            111,
            110,
            60,
            47,
            98,
            117,
            116,
            116,
            111,
            110,
            62,
        ],
        0,
    ) == 0) by (compute_only);
    cx::exact(u::lit("<button>Record decision</button>"@), 0);
    h::fixed(u::lit("<button>Record decision</button>"@), inputs, 0, 0);
}

pub proof fn l173(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("</form>"@)),
        u::copy_derived(u::lit("</form>"@), inputs, u::copy_registry()),
        u::copy_registry()[173] == u::lit("</form>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[173] == u::lit("</form>"@) && 173 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("</form>"@), inputs);
}

pub proof fn f173(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("</form>"@), inputs, 0, 0),
        u::copy_registry()[173] == u::lit("</form>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l173(inputs);
    reveal_strlit("</form>");
    is_ascii_chars_encode_utf8("</form>"@);
    assert(u::lit("</form>"@) =~= seq![60u8, 47, 102, 111, 114, 109, 62]);
    assert(cx::scan(seq![60u8, 47, 102, 111, 114, 109, 62], 0) == 0) by (compute_only);
    cx::exact(u::lit("</form>"@), 0);
    h::fixed(u::lit("</form>"@), inputs, 0, 0);
}

pub proof fn l174(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<details>"@)),
        u::copy_derived(u::lit("<details>"@), inputs, u::copy_registry()),
        u::copy_registry()[174] == u::lit("<details>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[174] == u::lit("<details>"@) && 174 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<details>"@), inputs);
}

pub proof fn f174(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<details>"@), inputs, 0, 0),
        u::copy_registry()[174] == u::lit("<details>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l174(inputs);
    reveal_strlit("<details>");
    is_ascii_chars_encode_utf8("<details>"@);
    assert(u::lit("<details>"@) =~= seq![60u8, 100, 101, 116, 97, 105, 108, 115, 62]);
    assert(cx::scan(seq![60u8, 100, 101, 116, 97, 105, 108, 115, 62], 0) == 0) by (compute_only);
    cx::exact(u::lit("<details>"@), 0);
    h::fixed(u::lit("<details>"@), inputs, 0, 0);
}

pub proof fn l175(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<summary>Compiled Prolog ("@)),
        u::copy_derived(u::lit("<summary>Compiled Prolog ("@), inputs, u::copy_registry()),
        u::copy_registry()[175] == u::lit("<summary>Compiled Prolog ("@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[175] == u::lit("<summary>Compiled Prolog ("@) && 175
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<summary>Compiled Prolog ("@), inputs);
}

pub proof fn f175(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<summary>Compiled Prolog ("@), inputs, 0, 0),
        u::copy_registry()[175] == u::lit("<summary>Compiled Prolog ("@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l175(inputs);
    reveal_strlit("<summary>Compiled Prolog (");
    is_ascii_chars_encode_utf8("<summary>Compiled Prolog ("@);
    assert(u::lit("<summary>Compiled Prolog ("@) =~= seq![
        60u8,
        115,
        117,
        109,
        109,
        97,
        114,
        121,
        62,
        67,
        111,
        109,
        112,
        105,
        108,
        101,
        100,
        32,
        80,
        114,
        111,
        108,
        111,
        103,
        32,
        40,
    ]);
    assert(cx::scan(
        seq![
            60u8,
            115,
            117,
            109,
            109,
            97,
            114,
            121,
            62,
            67,
            111,
            109,
            112,
            105,
            108,
            101,
            100,
            32,
            80,
            114,
            111,
            108,
            111,
            103,
            32,
            40,
        ],
        0,
    ) == 0) by (compute_only);
    cx::exact(u::lit("<summary>Compiled Prolog ("@), 0);
    h::fixed(u::lit("<summary>Compiled Prolog ("@), inputs, 0, 0);
}

pub proof fn l176(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit(" lines)</summary>"@)),
        u::copy_derived(u::lit(" lines)</summary>"@), inputs, u::copy_registry()),
        u::copy_registry()[176] == u::lit(" lines)</summary>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[176] == u::lit(" lines)</summary>"@) && 176
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit(" lines)</summary>"@), inputs);
}

pub proof fn f176(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed(" lines)</summary>"@), inputs, 0, 0),
        u::copy_registry()[176] == u::lit(" lines)</summary>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l176(inputs);
    reveal_strlit(" lines)</summary>");
    is_ascii_chars_encode_utf8(" lines)</summary>"@);
    assert(u::lit(" lines)</summary>"@) =~= seq![
        32u8,
        108,
        105,
        110,
        101,
        115,
        41,
        60,
        47,
        115,
        117,
        109,
        109,
        97,
        114,
        121,
        62,
    ]);
    assert(cx::scan(
        seq![32u8, 108, 105, 110, 101, 115, 41, 60, 47, 115, 117, 109, 109, 97, 114, 121, 62],
        0,
    ) == 0) by (compute_only);
    cx::exact(u::lit(" lines)</summary>"@), 0);
    h::fixed(u::lit(" lines)</summary>"@), inputs, 0, 0);
}

pub proof fn l177(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<pre>"@)),
        u::copy_derived(u::lit("<pre>"@), inputs, u::copy_registry()),
        u::copy_registry()[177] == u::lit("<pre>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[177] == u::lit("<pre>"@) && 177 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<pre>"@), inputs);
}

pub proof fn f177(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<pre>"@), inputs, 0, 0),
        u::copy_registry()[177] == u::lit("<pre>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l177(inputs);
    reveal_strlit("<pre>");
    is_ascii_chars_encode_utf8("<pre>"@);
    assert(u::lit("<pre>"@) =~= seq![60u8, 112, 114, 101, 62]);
    assert(cx::scan(seq![60u8, 112, 114, 101, 62], 0) == 0) by (compute_only);
    cx::exact(u::lit("<pre>"@), 0);
    h::fixed(u::lit("<pre>"@), inputs, 0, 0);
}

pub proof fn l178(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("</details>"@)),
        u::copy_derived(u::lit("</details>"@), inputs, u::copy_registry()),
        u::copy_registry()[178] == u::lit("</details>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[178] == u::lit("</details>"@) && 178 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("</details>"@), inputs);
}

pub proof fn f178(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("</details>"@), inputs, 0, 0),
        u::copy_registry()[178] == u::lit("</details>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l178(inputs);
    reveal_strlit("</details>");
    is_ascii_chars_encode_utf8("</details>"@);
    assert(u::lit("</details>"@) =~= seq![60u8, 47, 100, 101, 116, 97, 105, 108, 115, 62]);
    assert(cx::scan(seq![60u8, 47, 100, 101, 116, 97, 105, 108, 115, 62], 0) == 0)
        by (compute_only);
    cx::exact(u::lit("</details>"@), 0);
    h::fixed(u::lit("</details>"@), inputs, 0, 0);
}

pub proof fn l179(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<nav class=\"docnav\">"@)),
        u::copy_derived(u::lit("<nav class=\"docnav\">"@), inputs, u::copy_registry()),
        u::copy_registry()[179] == u::lit("<nav class=\"docnav\">"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[179] == u::lit("<nav class=\"docnav\">"@) && 179
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<nav class=\"docnav\">"@), inputs);
}

pub proof fn f179(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<nav class=\"docnav\">"@), inputs, 0, 0),
        u::copy_registry()[179] == u::lit("<nav class=\"docnav\">"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l179(inputs);
    reveal_strlit("<nav class=\"docnav\">");
    is_ascii_chars_encode_utf8("<nav class=\"docnav\">"@);
    assert(u::lit("<nav class=\"docnav\">"@) =~= seq![
        60u8,
        110,
        97,
        118,
        32,
        99,
        108,
        97,
        115,
        115,
        61,
        34,
        100,
        111,
        99,
        110,
        97,
        118,
        34,
        62,
    ]);
    assert(cx::scan(
        seq![
            60u8,
            110,
            97,
            118,
            32,
            99,
            108,
            97,
            115,
            115,
            61,
            34,
            100,
            111,
            99,
            110,
            97,
            118,
            34,
            62,
        ],
        0,
    ) == 0) by (compute_only);
    cx::exact(u::lit("<nav class=\"docnav\">"@), 0);
    h::fixed(u::lit("<nav class=\"docnav\">"@), inputs, 0, 0);
}

pub proof fn l180(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(
            u::lit("<a href=\"../../../index.html\">guidelines</a> / <a href=\"../index.html\">"@),
        ),
        u::copy_derived(
            u::lit("<a href=\"../../../index.html\">guidelines</a> / <a href=\"../index.html\">"@),
            inputs,
            u::copy_registry(),
        ),
        u::copy_registry()[180] == u::lit(
            "<a href=\"../../../index.html\">guidelines</a> / <a href=\"../index.html\">"@,
        ),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[180] == u::lit(
        "<a href=\"../../../index.html\">guidelines</a> / <a href=\"../index.html\">"@,
    ) && 180 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(
        u::lit("<a href=\"../../../index.html\">guidelines</a> / <a href=\"../index.html\">"@),
        inputs,
    );
}

pub proof fn f180(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(
            u::fixed(
                "<a href=\"../../../index.html\">guidelines</a> / <a href=\"../index.html\">"@,
            ),
            inputs,
            0,
            0,
        ),
        u::copy_registry()[180] == u::lit(
            "<a href=\"../../../index.html\">guidelines</a> / <a href=\"../index.html\">"@,
        ),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l180(inputs);
    reveal_strlit("<a href=\"../../../index.html\">guidelines</a> / <a href=\"../index.html\">");
    is_ascii_chars_encode_utf8(
        "<a href=\"../../../index.html\">guidelines</a> / <a href=\"../index.html\">"@,
    );
    assert(u::lit("<a href=\"../../../index.html\">guidelines</a> / <a href=\"../index.html\">"@)
        =~= seq![
        60u8,
        97,
        32,
        104,
        114,
        101,
        102,
        61,
        34,
        46,
        46,
        47,
        46,
        46,
        47,
        46,
        46,
        47,
        105,
        110,
        100,
        101,
        120,
        46,
        104,
        116,
        109,
        108,
        34,
        62,
        103,
        117,
        105,
        100,
        101,
        108,
        105,
        110,
        101,
        115,
        60,
        47,
        97,
        62,
        32,
        47,
        32,
        60,
        97,
        32,
        104,
        114,
        101,
        102,
        61,
        34,
        46,
        46,
        47,
        105,
        110,
        100,
        101,
        120,
        46,
        104,
        116,
        109,
        108,
        34,
        62,
    ]);
    assert(cx::scan(
        seq![
            60u8,
            97,
            32,
            104,
            114,
            101,
            102,
            61,
            34,
            46,
            46,
            47,
            46,
            46,
            47,
            46,
            46,
            47,
            105,
            110,
            100,
            101,
            120,
            46,
            104,
            116,
            109,
            108,
            34,
            62,
            103,
            117,
            105,
            100,
            101,
            108,
            105,
            110,
            101,
            115,
            60,
            47,
            97,
            62,
            32,
            47,
            32,
            60,
            97,
            32,
            104,
            114,
            101,
            102,
            61,
            34,
            46,
            46,
            47,
            105,
            110,
            100,
            101,
            120,
            46,
            104,
            116,
            109,
            108,
            34,
            62,
        ],
        0,
    ) == 0) by (compute_only);
    cx::exact(
        u::lit("<a href=\"../../../index.html\">guidelines</a> / <a href=\"../index.html\">"@),
        0,
    );
    h::fixed(
        u::lit("<a href=\"../../../index.html\">guidelines</a> / <a href=\"../index.html\">"@),
        inputs,
        0,
        0,
    );
}

pub proof fn l181(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("</a> / "@)),
        u::copy_derived(u::lit("</a> / "@), inputs, u::copy_registry()),
        u::copy_registry()[181] == u::lit("</a> / "@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[181] == u::lit("</a> / "@) && 181 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("</a> / "@), inputs);
}

pub proof fn f181(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("</a> / "@), inputs, 0, 0),
        u::copy_registry()[181] == u::lit("</a> / "@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l181(inputs);
    reveal_strlit("</a> / ");
    is_ascii_chars_encode_utf8("</a> / "@);
    assert(u::lit("</a> / "@) =~= seq![60u8, 47, 97, 62, 32, 47, 32]);
    assert(cx::scan(seq![60u8, 47, 97, 62, 32, 47, 32], 0) == 0) by (compute_only);
    cx::exact(u::lit("</a> / "@), 0);
    h::fixed(u::lit("</a> / "@), inputs, 0, 0);
}

pub proof fn l182(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(
            u::lit(
                "a an the every each no all some any this that these those such is are was were be been being has have had does do did should must may can cannot might will would shall could if then and or nor but not it its itself they them their he she who whom whose which what where when there something somebody someone everything everybody everyone nothing nobody of for with without during to at in on by from as against about after before through under over above below into onto per within between among around near than least most more less fewer greater"@,
            ),
        ),
        u::copy_derived(
            u::lit(
                "a an the every each no all some any this that these those such is are was were be been being has have had does do did should must may can cannot might will would shall could if then and or nor but not it its itself they them their he she who whom whose which what where when there something somebody someone everything everybody everyone nothing nobody of for with without during to at in on by from as against about after before through under over above below into onto per within between among around near than least most more less fewer greater"@,
            ),
            inputs,
            u::copy_registry(),
        ),
        u::copy_registry()[182] == u::lit(
            "a an the every each no all some any this that these those such is are was were be been being has have had does do did should must may can cannot might will would shall could if then and or nor but not it its itself they them their he she who whom whose which what where when there something somebody someone everything everybody everyone nothing nobody of for with without during to at in on by from as against about after before through under over above below into onto per within between among around near than least most more less fewer greater"@,
        ),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[182] == u::lit(
        "a an the every each no all some any this that these those such is are was were be been being has have had does do did should must may can cannot might will would shall could if then and or nor but not it its itself they them their he she who whom whose which what where when there something somebody someone everything everybody everyone nothing nobody of for with without during to at in on by from as against about after before through under over above below into onto per within between among around near than least most more less fewer greater"@,
    ) && 182 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(
        u::lit(
            "a an the every each no all some any this that these those such is are was were be been being has have had does do did should must may can cannot might will would shall could if then and or nor but not it its itself they them their he she who whom whose which what where when there something somebody someone everything everybody everyone nothing nobody of for with without during to at in on by from as against about after before through under over above below into onto per within between among around near than least most more less fewer greater"@,
        ),
        inputs,
    );
}

pub proof fn l183(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("</p>\n<!-- "@)),
        u::copy_derived(u::lit("</p>\n<!-- "@), inputs, u::copy_registry()),
        u::copy_registry()[183] == u::lit("</p>\n<!-- "@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[183] == u::lit("</p>\n<!-- "@) && 183 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("</p>\n<!-- "@), inputs);
}

pub proof fn f183(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("</p>\n<!-- "@), inputs, 0, 4),
        u::copy_registry()[183] == u::lit("</p>\n<!-- "@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l183(inputs);
    reveal_strlit("</p>\n<!-- ");
    is_ascii_chars_encode_utf8("</p>\n<!-- "@);
    assert(u::lit("</p>\n<!-- "@) =~= seq![60u8, 47, 112, 62, 10, 60, 33, 45, 45, 32]);
    assert(cx::scan(seq![60u8, 47, 112, 62, 10, 60, 33, 45, 45, 32], 0) == 4) by (compute_only);
    cx::exact(u::lit("</p>\n<!-- "@), 0);
    h::fixed(u::lit("</p>\n<!-- "@), inputs, 0, 4);
}

pub proof fn l184(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit(" -->"@)),
        u::copy_derived(u::lit(" -->"@), inputs, u::copy_registry()),
        u::copy_registry()[184] == u::lit(" -->"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[184] == u::lit(" -->"@) && 184 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit(" -->"@), inputs);
}

pub proof fn f184(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed(" -->"@), inputs, 4, 0),
        u::copy_registry()[184] == u::lit(" -->"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l184(inputs);
    reveal_strlit(" -->");
    is_ascii_chars_encode_utf8(" -->"@);
    assert(u::lit(" -->"@) =~= seq![32u8, 45, 45, 62]);
    assert(cx::scan(seq![32u8, 45, 45, 62], 4) == 0) by (compute_only);
    cx::exact(u::lit(" -->"@), 4);
    h::fixed(u::lit(" -->"@), inputs, 4, 0);
}

pub proof fn l185(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(
            u::lit(
                "The request was refused. Open the document page again from this site and submit the decision again."@,
            ),
        ),
        u::copy_derived(
            u::lit(
                "The request was refused. Open the document page again from this site and submit the decision again."@,
            ),
            inputs,
            u::copy_registry(),
        ),
        u::copy_registry()[185] == u::lit(
            "The request was refused. Open the document page again from this site and submit the decision again."@,
        ),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[185] == u::lit(
        "The request was refused. Open the document page again from this site and submit the decision again."@,
    ) && 185 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(
        u::lit(
            "The request was refused. Open the document page again from this site and submit the decision again."@,
        ),
        inputs,
    );
}

pub proof fn l186(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(
            u::lit(
                "The submitted form was not valid. Go back to the document page, reload it, and submit the decision again."@,
            ),
        ),
        u::copy_derived(
            u::lit(
                "The submitted form was not valid. Go back to the document page, reload it, and submit the decision again."@,
            ),
            inputs,
            u::copy_registry(),
        ),
        u::copy_registry()[186] == u::lit(
            "The submitted form was not valid. Go back to the document page, reload it, and submit the decision again."@,
        ),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[186] == u::lit(
        "The submitted form was not valid. Go back to the document page, reload it, and submit the decision again."@,
    ) && 186 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(
        u::lit(
            "The submitted form was not valid. Go back to the document page, reload it, and submit the decision again."@,
        ),
        inputs,
    );
}

pub proof fn l187(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("Forbidden"@)),
        u::copy_derived(u::lit("Forbidden"@), inputs, u::copy_registry()),
        u::copy_registry()[187] == u::lit("Forbidden"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[187] == u::lit("Forbidden"@) && 187 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("Forbidden"@), inputs);
}

pub proof fn l188(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("Bad request"@)),
        u::copy_derived(u::lit("Bad request"@), inputs, u::copy_registry()),
        u::copy_registry()[188] == u::lit("Bad request"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[188] == u::lit("Bad request"@) && 188 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("Bad request"@), inputs);
}

pub proof fn l189(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("Server error"@)),
        u::copy_derived(u::lit("Server error"@), inputs, u::copy_registry()),
        u::copy_registry()[189] == u::lit("Server error"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[189] == u::lit("Server error"@) && 189 < u::copy_registry().len())
        by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("Server error"@), inputs);
}

pub proof fn l190(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(
            u::lit("The server could not complete the request. Reload the page and try again."@),
        ),
        u::copy_derived(
            u::lit("The server could not complete the request. Reload the page and try again."@),
            inputs,
            u::copy_registry(),
        ),
        u::copy_registry()[190] == u::lit(
            "The server could not complete the request. Reload the page and try again."@,
        ),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[190] == u::lit(
        "The server could not complete the request. Reload the page and try again."@,
    ) && 190 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(
        u::lit("The server could not complete the request. Reload the page and try again."@),
        inputs,
    );
}

pub proof fn l191(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("Conflict"@)),
        u::copy_derived(u::lit("Conflict"@), inputs, u::copy_registry()),
        u::copy_registry()[191] == u::lit("Conflict"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[191] == u::lit("Conflict"@) && 191 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("Conflict"@), inputs);
}

pub proof fn l192(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("ui: verdict: ledger changed"@)),
        u::copy_derived(u::lit("ui: verdict: ledger changed"@), inputs, u::copy_registry()),
        u::copy_registry()[192] == u::lit("ui: verdict: ledger changed"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[192] == u::lit("ui: verdict: ledger changed"@) && 192
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("ui: verdict: ledger changed"@), inputs);
}

pub proof fn l193(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(
            u::lit(
                "Another decision was recorded for this guideline before this one. The decision was not recorded. Open the document page again and check the current state."@,
            ),
        ),
        u::copy_derived(
            u::lit(
                "Another decision was recorded for this guideline before this one. The decision was not recorded. Open the document page again and check the current state."@,
            ),
            inputs,
            u::copy_registry(),
        ),
        u::copy_registry()[193] == u::lit(
            "Another decision was recorded for this guideline before this one. The decision was not recorded. Open the document page again and check the current state."@,
        ),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[193] == u::lit(
        "Another decision was recorded for this guideline before this one. The decision was not recorded. Open the document page again and check the current state."@,
    ) && 193 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(
        u::lit(
            "Another decision was recorded for this guideline before this one. The decision was not recorded. Open the document page again and check the current state."@,
        ),
        inputs,
    );
}

pub proof fn l194(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("/g/"@)),
        u::copy_derived(u::lit("/g/"@), inputs, u::copy_registry()),
        u::copy_registry()[194] == u::lit("/g/"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[194] == u::lit("/g/"@) && 194 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("/g/"@), inputs);
}

pub proof fn l195(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("doc"@)),
        u::copy_derived(u::lit("doc"@), inputs, u::copy_registry()),
        u::copy_registry()[195] == u::lit("doc"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[195] == u::lit("doc"@) && 195 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("doc"@), inputs);
}

pub proof fn l196(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("Method not allowed"@)),
        u::copy_derived(u::lit("Method not allowed"@), inputs, u::copy_registry()),
        u::copy_registry()[196] == u::lit("Method not allowed"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[196] == u::lit("Method not allowed"@) && 196
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("Method not allowed"@), inputs);
}

pub proof fn l197(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(
            u::lit("<p>Only GET and POST are supported on this page.</p>"@),
        ),
        u::copy_derived(
            u::lit("<p>Only GET and POST are supported on this page.</p>"@),
            inputs,
            u::copy_registry(),
        ),
        u::copy_registry()[197] == u::lit("<p>Only GET and POST are supported on this page.</p>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[197] == u::lit(
        "<p>Only GET and POST are supported on this page.</p>"@,
    ) && 197 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<p>Only GET and POST are supported on this page.</p>"@), inputs);
}

pub proof fn f197(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(
            u::fixed("<p>Only GET and POST are supported on this page.</p>"@),
            inputs,
            0,
            0,
        ),
        u::copy_registry()[197] == u::lit("<p>Only GET and POST are supported on this page.</p>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l197(inputs);
    reveal_strlit("<p>Only GET and POST are supported on this page.</p>");
    is_ascii_chars_encode_utf8("<p>Only GET and POST are supported on this page.</p>"@);
    assert(u::lit("<p>Only GET and POST are supported on this page.</p>"@) =~= seq![
        60u8,
        112,
        62,
        79,
        110,
        108,
        121,
        32,
        71,
        69,
        84,
        32,
        97,
        110,
        100,
        32,
        80,
        79,
        83,
        84,
        32,
        97,
        114,
        101,
        32,
        115,
        117,
        112,
        112,
        111,
        114,
        116,
        101,
        100,
        32,
        111,
        110,
        32,
        116,
        104,
        105,
        115,
        32,
        112,
        97,
        103,
        101,
        46,
        60,
        47,
        112,
        62,
    ]);
    assert(cx::scan(
        seq![
            60u8,
            112,
            62,
            79,
            110,
            108,
            121,
            32,
            71,
            69,
            84,
            32,
            97,
            110,
            100,
            32,
            80,
            79,
            83,
            84,
            32,
            97,
            114,
            101,
            32,
            115,
            117,
            112,
            112,
            111,
            114,
            116,
            101,
            100,
            32,
            111,
            110,
            32,
            116,
            104,
            105,
            115,
            32,
            112,
            97,
            103,
            101,
            46,
            60,
            47,
            112,
            62,
        ],
        0,
    ) == 0) by (compute_only);
    cx::exact(u::lit("<p>Only GET and POST are supported on this page.</p>"@), 0);
    h::fixed(u::lit("<p>Only GET and POST are supported on this page.</p>"@), inputs, 0, 0);
}

pub proof fn l198(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<p>Only GET is supported on this page.</p>"@)),
        u::copy_derived(
            u::lit("<p>Only GET is supported on this page.</p>"@),
            inputs,
            u::copy_registry(),
        ),
        u::copy_registry()[198] == u::lit("<p>Only GET is supported on this page.</p>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[198] == u::lit("<p>Only GET is supported on this page.</p>"@) && 198
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<p>Only GET is supported on this page.</p>"@), inputs);
}

pub proof fn f198(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<p>Only GET is supported on this page.</p>"@), inputs, 0, 0),
        u::copy_registry()[198] == u::lit("<p>Only GET is supported on this page.</p>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l198(inputs);
    reveal_strlit("<p>Only GET is supported on this page.</p>");
    is_ascii_chars_encode_utf8("<p>Only GET is supported on this page.</p>"@);
    assert(u::lit("<p>Only GET is supported on this page.</p>"@) =~= seq![
        60u8,
        112,
        62,
        79,
        110,
        108,
        121,
        32,
        71,
        69,
        84,
        32,
        105,
        115,
        32,
        115,
        117,
        112,
        112,
        111,
        114,
        116,
        101,
        100,
        32,
        111,
        110,
        32,
        116,
        104,
        105,
        115,
        32,
        112,
        97,
        103,
        101,
        46,
        60,
        47,
        112,
        62,
    ]);
    assert(cx::scan(
        seq![
            60u8,
            112,
            62,
            79,
            110,
            108,
            121,
            32,
            71,
            69,
            84,
            32,
            105,
            115,
            32,
            115,
            117,
            112,
            112,
            111,
            114,
            116,
            101,
            100,
            32,
            111,
            110,
            32,
            116,
            104,
            105,
            115,
            32,
            112,
            97,
            103,
            101,
            46,
            60,
            47,
            112,
            62,
        ],
        0,
    ) == 0) by (compute_only);
    cx::exact(u::lit("<p>Only GET is supported on this page.</p>"@), 0);
    h::fixed(u::lit("<p>Only GET is supported on this page.</p>"@), inputs, 0, 0);
}

pub proof fn l199(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("GET, POST"@)),
        u::copy_derived(u::lit("GET, POST"@), inputs, u::copy_registry()),
        u::copy_registry()[199] == u::lit("GET, POST"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[199] == u::lit("GET, POST"@) && 199 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("GET, POST"@), inputs);
}

pub proof fn l200(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("GET"@)),
        u::copy_derived(u::lit("GET"@), inputs, u::copy_registry()),
        u::copy_registry()[200] == u::lit("GET"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[200] == u::lit("GET"@) && 200 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("GET"@), inputs);
}

pub proof fn l201(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("Not found"@)),
        u::copy_derived(u::lit("Not found"@), inputs, u::copy_registry()),
        u::copy_registry()[201] == u::lit("Not found"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[201] == u::lit("Not found"@) && 201 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("Not found"@), inputs);
}

pub proof fn l202(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<p>The requested page does not exist.</p>"@)),
        u::copy_derived(
            u::lit("<p>The requested page does not exist.</p>"@),
            inputs,
            u::copy_registry(),
        ),
        u::copy_registry()[202] == u::lit("<p>The requested page does not exist.</p>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[202] == u::lit("<p>The requested page does not exist.</p>"@) && 202
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<p>The requested page does not exist.</p>"@), inputs);
}

pub proof fn f202(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<p>The requested page does not exist.</p>"@), inputs, 0, 0),
        u::copy_registry()[202] == u::lit("<p>The requested page does not exist.</p>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l202(inputs);
    reveal_strlit("<p>The requested page does not exist.</p>");
    is_ascii_chars_encode_utf8("<p>The requested page does not exist.</p>"@);
    assert(u::lit("<p>The requested page does not exist.</p>"@) =~= seq![
        60u8,
        112,
        62,
        84,
        104,
        101,
        32,
        114,
        101,
        113,
        117,
        101,
        115,
        116,
        101,
        100,
        32,
        112,
        97,
        103,
        101,
        32,
        100,
        111,
        101,
        115,
        32,
        110,
        111,
        116,
        32,
        101,
        120,
        105,
        115,
        116,
        46,
        60,
        47,
        112,
        62,
    ]);
    assert(cx::scan(
        seq![
            60u8,
            112,
            62,
            84,
            104,
            101,
            32,
            114,
            101,
            113,
            117,
            101,
            115,
            116,
            101,
            100,
            32,
            112,
            97,
            103,
            101,
            32,
            100,
            111,
            101,
            115,
            32,
            110,
            111,
            116,
            32,
            101,
            120,
            105,
            115,
            116,
            46,
            60,
            47,
            112,
            62,
        ],
        0,
    ) == 0) by (compute_only);
    cx::exact(u::lit("<p>The requested page does not exist.</p>"@), 0);
    h::fixed(u::lit("<p>The requested page does not exist.</p>"@), inputs, 0, 0);
}

pub proof fn l203(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("="@)),
        u::copy_derived(u::lit("="@), inputs, u::copy_registry()),
        u::copy_registry()[203] == u::lit("="@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[203] == u::lit("="@) && 203 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("="@), inputs);
}

pub proof fn l204(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("ui: verdict: body not parseable"@)),
        u::copy_derived(u::lit("ui: verdict: body not parseable"@), inputs, u::copy_registry()),
        u::copy_registry()[204] == u::lit("ui: verdict: body not parseable"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[204] == u::lit("ui: verdict: body not parseable"@) && 204
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("ui: verdict: body not parseable"@), inputs);
}

pub proof fn l205(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("verdict"@)),
        u::copy_derived(u::lit("verdict"@), inputs, u::copy_registry()),
        u::copy_registry()[205] == u::lit("verdict"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[205] == u::lit("verdict"@) && 205 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("verdict"@), inputs);
}

pub proof fn l206(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("reviewer"@)),
        u::copy_derived(u::lit("reviewer"@), inputs, u::copy_registry()),
        u::copy_registry()[206] == u::lit("reviewer"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[206] == u::lit("reviewer"@) && 206 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("reviewer"@), inputs);
}

pub proof fn l207(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("comment"@)),
        u::copy_derived(u::lit("comment"@), inputs, u::copy_registry()),
        u::copy_registry()[207] == u::lit("comment"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[207] == u::lit("comment"@) && 207 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("comment"@), inputs);
}

pub proof fn l208(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("review_sha256"@)),
        u::copy_derived(u::lit("review_sha256"@), inputs, u::copy_registry()),
        u::copy_registry()[208] == u::lit("review_sha256"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[208] == u::lit("review_sha256"@) && 208 < u::copy_registry().len())
        by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("review_sha256"@), inputs);
}

pub proof fn l209(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("ledger_sha256"@)),
        u::copy_derived(u::lit("ledger_sha256"@), inputs, u::copy_registry()),
        u::copy_registry()[209] == u::lit("ledger_sha256"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[209] == u::lit("ledger_sha256"@) && 209 < u::copy_registry().len())
        by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("ledger_sha256"@), inputs);
}

pub proof fn l210(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("csrf"@)),
        u::copy_derived(u::lit("csrf"@), inputs, u::copy_registry()),
        u::copy_registry()[210] == u::lit("csrf"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[210] == u::lit("csrf"@) && 210 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("csrf"@), inputs);
}

pub proof fn l211(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("ui: verdict: missing field "@)),
        u::copy_derived(u::lit("ui: verdict: missing field "@), inputs, u::copy_registry()),
        u::copy_registry()[211] == u::lit("ui: verdict: missing field "@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[211] == u::lit("ui: verdict: missing field "@) && 211
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("ui: verdict: missing field "@), inputs);
}

pub proof fn l212(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("ui: verdict: duplicate field "@)),
        u::copy_derived(u::lit("ui: verdict: duplicate field "@), inputs, u::copy_registry()),
        u::copy_registry()[212] == u::lit("ui: verdict: duplicate field "@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[212] == u::lit("ui: verdict: duplicate field "@) && 212
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("ui: verdict: duplicate field "@), inputs);
}

pub proof fn l213(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("ui: verdict: unknown field "@)),
        u::copy_derived(u::lit("ui: verdict: unknown field "@), inputs, u::copy_registry()),
        u::copy_registry()[213] == u::lit("ui: verdict: unknown field "@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[213] == u::lit("ui: verdict: unknown field "@) && 213
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("ui: verdict: unknown field "@), inputs);
}

pub proof fn l214(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("ui: verdict: invalid verdict"@)),
        u::copy_derived(u::lit("ui: verdict: invalid verdict"@), inputs, u::copy_registry()),
        u::copy_registry()[214] == u::lit("ui: verdict: invalid verdict"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[214] == u::lit("ui: verdict: invalid verdict"@) && 214
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("ui: verdict: invalid verdict"@), inputs);
}

pub proof fn l215(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("ui: verdict: invalid reviewer"@)),
        u::copy_derived(u::lit("ui: verdict: invalid reviewer"@), inputs, u::copy_registry()),
        u::copy_registry()[215] == u::lit("ui: verdict: invalid reviewer"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[215] == u::lit("ui: verdict: invalid reviewer"@) && 215
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("ui: verdict: invalid reviewer"@), inputs);
}

pub proof fn l216(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("ui: verdict: invalid comment"@)),
        u::copy_derived(u::lit("ui: verdict: invalid comment"@), inputs, u::copy_registry()),
        u::copy_registry()[216] == u::lit("ui: verdict: invalid comment"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[216] == u::lit("ui: verdict: invalid comment"@) && 216
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("ui: verdict: invalid comment"@), inputs);
}

pub proof fn l217(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("ui: verdict: invalid review_sha256"@)),
        u::copy_derived(u::lit("ui: verdict: invalid review_sha256"@), inputs, u::copy_registry()),
        u::copy_registry()[217] == u::lit("ui: verdict: invalid review_sha256"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[217] == u::lit("ui: verdict: invalid review_sha256"@) && 217
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("ui: verdict: invalid review_sha256"@), inputs);
}

pub proof fn l218(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("absent"@)),
        u::copy_derived(u::lit("absent"@), inputs, u::copy_registry()),
        u::copy_registry()[218] == u::lit("absent"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[218] == u::lit("absent"@) && 218 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("absent"@), inputs);
}

pub proof fn l219(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("ui: verdict: invalid ledger_sha256"@)),
        u::copy_derived(u::lit("ui: verdict: invalid ledger_sha256"@), inputs, u::copy_registry()),
        u::copy_registry()[219] == u::lit("ui: verdict: invalid ledger_sha256"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[219] == u::lit("ui: verdict: invalid ledger_sha256"@) && 219
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("ui: verdict: invalid ledger_sha256"@), inputs);
}

pub proof fn l220(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("ui: verdict: body not decodable"@)),
        u::copy_derived(u::lit("ui: verdict: body not decodable"@), inputs, u::copy_registry()),
        u::copy_registry()[220] == u::lit("ui: verdict: body not decodable"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[220] == u::lit("ui: verdict: body not decodable"@) && 220
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("ui: verdict: body not decodable"@), inputs);
}

pub proof fn l221(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("\t"@)),
        u::copy_derived(u::lit("\t"@), inputs, u::copy_registry()),
        u::copy_registry()[221] == u::lit("\t"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[221] == u::lit("\t"@) && 221 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("\t"@), inputs);
}

pub proof fn l222(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("ui: verdict: manifest derivation failed: "@)),
        u::copy_derived(
            u::lit("ui: verdict: manifest derivation failed: "@),
            inputs,
            u::copy_registry(),
        ),
        u::copy_registry()[222] == u::lit("ui: verdict: manifest derivation failed: "@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[222] == u::lit("ui: verdict: manifest derivation failed: "@) && 222
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("ui: verdict: manifest derivation failed: "@), inputs);
}

pub proof fn l223(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(
            u::lit("ui: verdict: manifest derivation failed: docid row missing"@),
        ),
        u::copy_derived(
            u::lit("ui: verdict: manifest derivation failed: docid row missing"@),
            inputs,
            u::copy_registry(),
        ),
        u::copy_registry()[223] == u::lit(
            "ui: verdict: manifest derivation failed: docid row missing"@,
        ),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[223] == u::lit(
        "ui: verdict: manifest derivation failed: docid row missing"@,
    ) && 223 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("ui: verdict: manifest derivation failed: docid row missing"@), inputs);
}

pub proof fn l224(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("ui: verdict: subject changed"@)),
        u::copy_derived(u::lit("ui: verdict: subject changed"@), inputs, u::copy_registry()),
        u::copy_registry()[224] == u::lit("ui: verdict: subject changed"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[224] == u::lit("ui: verdict: subject changed"@) && 224
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("ui: verdict: subject changed"@), inputs);
}

pub proof fn l225(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(
            u::lit(
                "The document or its source changed after this page was loaded. The decision was not recorded. Open the document page again and check the current version."@,
            ),
        ),
        u::copy_derived(
            u::lit(
                "The document or its source changed after this page was loaded. The decision was not recorded. Open the document page again and check the current version."@,
            ),
            inputs,
            u::copy_registry(),
        ),
        u::copy_registry()[225] == u::lit(
            "The document or its source changed after this page was loaded. The decision was not recorded. Open the document page again and check the current version."@,
        ),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[225] == u::lit(
        "The document or its source changed after this page was loaded. The decision was not recorded. Open the document page again and check the current version."@,
    ) && 225 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(
        u::lit(
            "The document or its source changed after this page was loaded. The decision was not recorded. Open the document page again and check the current version."@,
        ),
        inputs,
    );
}

pub proof fn l226(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("ui: adjudication ledger invalid: "@)),
        u::copy_derived(u::lit("ui: adjudication ledger invalid: "@), inputs, u::copy_registry()),
        u::copy_registry()[226] == u::lit("ui: adjudication ledger invalid: "@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[226] == u::lit("ui: adjudication ledger invalid: "@) && 226
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("ui: adjudication ledger invalid: "@), inputs);
}

pub proof fn l227(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("Decision recorded"@)),
        u::copy_derived(u::lit("Decision recorded"@), inputs, u::copy_registry()),
        u::copy_registry()[227] == u::lit("Decision recorded"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[227] == u::lit("Decision recorded"@) && 227
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("Decision recorded"@), inputs);
}

pub proof fn l228(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("<p>The decision was recorded.</p>"@)),
        u::copy_derived(u::lit("<p>The decision was recorded.</p>"@), inputs, u::copy_registry()),
        u::copy_registry()[228] == u::lit("<p>The decision was recorded.</p>"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[228] == u::lit("<p>The decision was recorded.</p>"@) && 228
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("<p>The decision was recorded.</p>"@), inputs);
}

pub proof fn f228(inputs: Seq<u::Bytes>)
    ensures
        h::fragment(u::fixed("<p>The decision was recorded.</p>"@), inputs, 0, 0),
        u::copy_registry()[228] == u::lit("<p>The decision was recorded.</p>"@),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l228(inputs);
    reveal_strlit("<p>The decision was recorded.</p>");
    is_ascii_chars_encode_utf8("<p>The decision was recorded.</p>"@);
    assert(u::lit("<p>The decision was recorded.</p>"@) =~= seq![
        60u8,
        112,
        62,
        84,
        104,
        101,
        32,
        100,
        101,
        99,
        105,
        115,
        105,
        111,
        110,
        32,
        119,
        97,
        115,
        32,
        114,
        101,
        99,
        111,
        114,
        100,
        101,
        100,
        46,
        60,
        47,
        112,
        62,
    ]);
    assert(cx::scan(
        seq![
            60u8,
            112,
            62,
            84,
            104,
            101,
            32,
            100,
            101,
            99,
            105,
            115,
            105,
            111,
            110,
            32,
            119,
            97,
            115,
            32,
            114,
            101,
            99,
            111,
            114,
            100,
            101,
            100,
            46,
            60,
            47,
            112,
            62,
        ],
        0,
    ) == 0) by (compute_only);
    cx::exact(u::lit("<p>The decision was recorded.</p>"@), 0);
    h::fixed(u::lit("<p>The decision was recorded.</p>"@), inputs, 0, 0);
}

pub proof fn l229(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("/doc/"@)),
        u::copy_derived(u::lit("/doc/"@), inputs, u::copy_registry()),
        u::copy_registry()[229] == u::lit("/doc/"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[229] == u::lit("/doc/"@) && 229 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("/doc/"@), inputs);
}

pub proof fn l230(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("http://127.0.0.1:"@)),
        u::copy_derived(u::lit("http://127.0.0.1:"@), inputs, u::copy_registry()),
        u::copy_registry()[230] == u::lit("http://127.0.0.1:"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[230] == u::lit("http://127.0.0.1:"@) && 230
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("http://127.0.0.1:"@), inputs);
}

pub proof fn l231(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("ui: verdict: origin not allowed"@)),
        u::copy_derived(u::lit("ui: verdict: origin not allowed"@), inputs, u::copy_registry()),
        u::copy_registry()[231] == u::lit("ui: verdict: origin not allowed"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[231] == u::lit("ui: verdict: origin not allowed"@) && 231
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("ui: verdict: origin not allowed"@), inputs);
}

pub proof fn l232(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("application/x-www-form-urlencoded"@)),
        u::copy_derived(u::lit("application/x-www-form-urlencoded"@), inputs, u::copy_registry()),
        u::copy_registry()[232] == u::lit("application/x-www-form-urlencoded"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[232] == u::lit("application/x-www-form-urlencoded"@) && 232
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("application/x-www-form-urlencoded"@), inputs);
}

pub proof fn l233(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("ui: verdict: unsupported content type"@)),
        u::copy_derived(
            u::lit("ui: verdict: unsupported content type"@),
            inputs,
            u::copy_registry(),
        ),
        u::copy_registry()[233] == u::lit("ui: verdict: unsupported content type"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[233] == u::lit("ui: verdict: unsupported content type"@) && 233
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("ui: verdict: unsupported content type"@), inputs);
}

pub proof fn l234(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("ui: verdict: missing body"@)),
        u::copy_derived(u::lit("ui: verdict: missing body"@), inputs, u::copy_registry()),
        u::copy_registry()[234] == u::lit("ui: verdict: missing body"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[234] == u::lit("ui: verdict: missing body"@) && 234
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("ui: verdict: missing body"@), inputs);
}

pub proof fn l235(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("ui: verdict: invalid csrf token"@)),
        u::copy_derived(u::lit("ui: verdict: invalid csrf token"@), inputs, u::copy_registry()),
        u::copy_registry()[235] == u::lit("ui: verdict: invalid csrf token"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[235] == u::lit("ui: verdict: invalid csrf token"@) && 235
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("ui: verdict: invalid csrf token"@), inputs);
}

pub proof fn l236(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("127.0.0.1:"@)),
        u::copy_derived(u::lit("127.0.0.1:"@), inputs, u::copy_registry()),
        u::copy_registry()[236] == u::lit("127.0.0.1:"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[236] == u::lit("127.0.0.1:"@) && 236 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("127.0.0.1:"@), inputs);
}

pub proof fn l237(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("ui: request: host not allowed"@)),
        u::copy_derived(u::lit("ui: request: host not allowed"@), inputs, u::copy_registry()),
        u::copy_registry()[237] == u::lit("ui: request: host not allowed"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[237] == u::lit("ui: request: host not allowed"@) && 237
        < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("ui: request: host not allowed"@), inputs);
}

pub proof fn l238(inputs: Seq<u::Bytes>)
    ensures
        u::copy_registry().contains(u::lit("POST"@)),
        u::copy_derived(u::lit("POST"@), inputs, u::copy_registry()),
        u::copy_registry()[238] == u::lit("POST"@),
{
    hide(u::copy_derived);
    hide(u::copy_registry);
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
    assert(u::copy_registry()[238] == u::lit("POST"@) && 238 < u::copy_registry().len()) by {
        reveal(u::copy_registry);
    };
    b::literal(u::lit("POST"@), inputs);
}

} // verus!
