use ckc_spec::ui as u;
use vstd::prelude::*;
#[cfg(verus_keep_ghost)]
macro_rules! ns { ($($x:expr),* $(,)?) => { seq![$($x),*] }; }
pub mod batch_000 {
    use crate::{
        k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
        k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
    };
    use ckc_spec::ui as u;
    use vstd::prelude::*;
    use vstd::utf8::*;
    verus! {

pub open spec fn data_000() -> Seq<nat> {
    ns![38u32,97,109,112,59].map_values(|x: u32| x as nat)
}

pub proof fn literal_000()
    ensures
        u::copy_literal_ok(u::copy_registry()[0]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l000(Seq::empty());
    p::all();
    reveal_strlit("&amp;");
    assert(n::codes("&amp;"@) =~= data_000());
    assert(n::clean(data_000(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("&amp;"@);
}

pub open spec fn data_001() -> Seq<nat> {
    ns![38u32,108,116,59].map_values(|x: u32| x as nat)
}

pub proof fn literal_001()
    ensures
        u::copy_literal_ok(u::copy_registry()[1]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l001(Seq::empty());
    p::all();
    reveal_strlit("&lt;");
    assert(n::codes("&lt;"@) =~= data_001());
    assert(n::clean(data_001(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("&lt;"@);
}

pub open spec fn data_002() -> Seq<nat> {
    ns![38u32,103,116,59].map_values(|x: u32| x as nat)
}

pub proof fn literal_002()
    ensures
        u::copy_literal_ok(u::copy_registry()[2]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l002(Seq::empty());
    p::all();
    reveal_strlit("&gt;");
    assert(n::codes("&gt;"@) =~= data_002());
    assert(n::clean(data_002(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("&gt;"@);
}

pub open spec fn data_003() -> Seq<nat> {
    ns![38u32,113,117,111,116,59].map_values(|x: u32| x as nat)
}

pub proof fn literal_003()
    ensures
        u::copy_literal_ok(u::copy_registry()[3]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l003(Seq::empty());
    p::all();
    reveal_strlit("&quot;");
    assert(n::codes("&quot;"@) =~= data_003());
    assert(n::clean(data_003(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("&quot;"@);
}

pub open spec fn data_004() -> Seq<nat> {
    ns![38u32,35,120,50,55,59].map_values(|x: u32| x as nat)
}

pub proof fn literal_004()
    ensures
        u::copy_literal_ok(u::copy_registry()[4]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l004(Seq::empty());
    p::all();
    reveal_strlit("&#x27;");
    assert(n::codes("&#x27;"@) =~= data_004());
    assert(n::clean(data_004(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("&#x27;"@);
}

pub open spec fn data_005() -> Seq<nat> {
    ns![10u32,].map_values(|x: u32| x as nat)
}

pub proof fn literal_005()
    ensures
        u::copy_literal_ok(u::copy_registry()[5]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l005(Seq::empty());
    p::all();
    reveal_strlit("\n");
    assert(n::codes("\n"@) =~= data_005());
    assert(n::clean(data_005(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("\n"@);
}

pub open spec fn data_006() -> Seq<nat> {
    ns![60u32,116,100,62].map_values(|x: u32| x as nat)
}

pub proof fn literal_006()
    ensures
        u::copy_literal_ok(u::copy_registry()[6]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l006(Seq::empty());
    p::all();
    reveal_strlit("<td>");
    assert(n::codes("<td>"@) =~= data_006());
    assert(n::clean(data_006(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<td>"@);
}

pub open spec fn data_007() -> Seq<nat> {
    ns![60u32,47,116,100,62].map_values(|x: u32| x as nat)
}

pub proof fn literal_007()
    ensures
        u::copy_literal_ok(u::copy_registry()[7]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l007(Seq::empty());
    p::all();
    reveal_strlit("</td>");
    assert(n::codes("</td>"@) =~= data_007());
    assert(n::clean(data_007(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("</td>"@);
}

pub open spec fn data_008() -> Seq<nat> {
    ns![60u32,116,114,62].map_values(|x: u32| x as nat)
}

pub proof fn literal_008()
    ensures
        u::copy_literal_ok(u::copy_registry()[8]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l008(Seq::empty());
    p::all();
    reveal_strlit("<tr>");
    assert(n::codes("<tr>"@) =~= data_008());
    assert(n::clean(data_008(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<tr>"@);
}

pub open spec fn data_009() -> Seq<nat> {
    ns![60u32,47,116,114,62].map_values(|x: u32| x as nat)
}

pub proof fn literal_009()
    ensures
        u::copy_literal_ok(u::copy_registry()[9]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l009(Seq::empty());
    p::all();
    reveal_strlit("</tr>");
    assert(n::codes("</tr>"@) =~= data_009());
    assert(n::clean(data_009(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("</tr>"@);
}

pub open spec fn data_010() -> Seq<nat> {
    ns![60u32,97,32,104,114,101,102,61,34].map_values(|x: u32| x as nat)
}

pub proof fn literal_010()
    ensures
        u::copy_literal_ok(u::copy_registry()[10]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l010(Seq::empty());
    p::all();
    reveal_strlit("<a href=\"");
    assert(n::codes("<a href=\""@) =~= data_010());
    assert(n::clean(data_010(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<a href=\""@);
}

pub open spec fn data_011() -> Seq<nat> {
    ns![34u32,62].map_values(|x: u32| x as nat)
}

pub proof fn literal_011()
    ensures
        u::copy_literal_ok(u::copy_registry()[11]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l011(Seq::empty());
    p::all();
    reveal_strlit("\">");
    assert(n::codes("\">"@) =~= data_011());
    assert(n::clean(data_011(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("\">"@);
}

pub open spec fn data_012() -> Seq<nat> {
    ns![60u32,47,97,62].map_values(|x: u32| x as nat)
}

pub proof fn literal_012()
    ensures
        u::copy_literal_ok(u::copy_registry()[12]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l012(Seq::empty());
    p::all();
    reveal_strlit("</a>");
    assert(n::codes("</a>"@) =~= data_012());
    assert(n::clean(data_012(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("</a>"@);
}

pub open spec fn data_013() -> Seq<nat> {
    ns![97u32,112,112,114,111,118,101,100].map_values(|x: u32| x as nat)
}

pub proof fn literal_013()
    ensures
        u::copy_literal_ok(u::copy_registry()[13]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l013(Seq::empty());
    p::all();
    reveal_strlit("approved");
    assert(n::codes("approved"@) =~= data_013());
    assert(n::clean(data_013(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("approved"@);
}

pub open spec fn data_014() -> Seq<nat> {
    ns![114u32,101,106,101,99,116,101,100].map_values(|x: u32| x as nat)
}

pub proof fn literal_014()
    ensures
        u::copy_literal_ok(u::copy_registry()[14]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l014(Seq::empty());
    p::all();
    reveal_strlit("rejected");
    assert(n::codes("rejected"@) =~= data_014());
    assert(n::clean(data_014(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("rejected"@);
}

pub open spec fn data_015() -> Seq<nat> {
    ns![99u32,111,110,116,101,115,116,101,100].map_values(|x: u32| x as nat)
}

pub proof fn literal_015()
    ensures
        u::copy_literal_ok(u::copy_registry()[15]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l015(Seq::empty());
    p::all();
    reveal_strlit("contested");
    assert(n::codes("contested"@) =~= data_015());
    assert(n::clean(data_015(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("contested"@);
}

} // verus!
}
pub mod batch_016 {
    use crate::{
        k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
        k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
    };
    use ckc_spec::ui as u;
    use vstd::prelude::*;
    use vstd::utf8::*;
    verus! {

pub open spec fn data_016() -> Seq<nat> {
    ns![115u32,116,97,108,101].map_values(|x: u32| x as nat)
}

pub proof fn literal_016()
    ensures
        u::copy_literal_ok(u::copy_registry()[16]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l016(Seq::empty());
    p::all();
    reveal_strlit("stale");
    assert(n::codes("stale"@) =~= data_016());
    assert(n::clean(data_016(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("stale"@);
}

pub open spec fn data_017() -> Seq<nat> {
    ns![117u32,110,114,101,118,105,101,119,101,100].map_values(|x: u32| x as nat)
}

pub proof fn literal_017()
    ensures
        u::copy_literal_ok(u::copy_registry()[17]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l017(Seq::empty());
    p::all();
    reveal_strlit("unreviewed");
    assert(n::codes("unreviewed"@) =~= data_017());
    assert(n::clean(data_017(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("unreviewed"@);
}

pub open spec fn data_018() -> Seq<nat> {
    ns![65u32,112,112,114,111,118,101,100].map_values(|x: u32| x as nat)
}

pub proof fn literal_018()
    ensures
        u::copy_literal_ok(u::copy_registry()[18]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l018(Seq::empty());
    p::all();
    reveal_strlit("Approved");
    assert(n::codes("Approved"@) =~= data_018());
    assert(n::clean(data_018(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("Approved"@);
}

pub open spec fn data_019() -> Seq<nat> {
    ns![82u32,101,106,101,99,116,101,100].map_values(|x: u32| x as nat)
}

pub proof fn literal_019()
    ensures
        u::copy_literal_ok(u::copy_registry()[19]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l019(Seq::empty());
    p::all();
    reveal_strlit("Rejected");
    assert(n::codes("Rejected"@) =~= data_019());
    assert(n::clean(data_019(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("Rejected"@);
}

pub open spec fn data_020() -> Seq<nat> {
    ns![67u32,111,110,116,101,115,116,101,100].map_values(|x: u32| x as nat)
}

pub proof fn literal_020()
    ensures
        u::copy_literal_ok(u::copy_registry()[20]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l020(Seq::empty());
    p::all();
    reveal_strlit("Contested");
    assert(n::codes("Contested"@) =~= data_020());
    assert(n::clean(data_020(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("Contested"@);
}

pub open spec fn data_021() -> Seq<nat> {
    ns![79u32,117,116,100,97,116,101,100].map_values(|x: u32| x as nat)
}

pub proof fn literal_021()
    ensures
        u::copy_literal_ok(u::copy_registry()[21]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l021(Seq::empty());
    p::all();
    reveal_strlit("Outdated");
    assert(n::codes("Outdated"@) =~= data_021());
    assert(n::clean(data_021(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("Outdated"@);
}

pub open spec fn data_022() -> Seq<nat> {
    ns![85u32,110,114,101,118,105,101,119,101,100].map_values(|x: u32| x as nat)
}

pub proof fn literal_022()
    ensures
        u::copy_literal_ok(u::copy_registry()[22]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l022(Seq::empty());
    p::all();
    reveal_strlit("Unreviewed");
    assert(n::codes("Unreviewed"@) =~= data_022());
    assert(n::clean(data_022(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("Unreviewed"@);
}

pub open spec fn data_023() -> Seq<nat> {
    ns![60u32,115,112,97,110,32,99,108,97,115,115,61,34,99,104,105,112,32,99,104,105,112,45].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_023()
    ensures
        u::copy_literal_ok(u::copy_registry()[23]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l023(Seq::empty());
    p::all();
    reveal_strlit("<span class=\"chip chip-");
    assert(n::codes("<span class=\"chip chip-"@) =~= data_023());
    assert(n::clean(data_023(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<span class=\"chip chip-"@);
}

pub open spec fn data_024() -> Seq<nat> {
    ns![60u32,47,115,112,97,110,62].map_values(|x: u32| x as nat)
}

pub proof fn literal_024()
    ensures
        u::copy_literal_ok(u::copy_registry()[24]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l024(Seq::empty());
    p::all();
    reveal_strlit("</span>");
    assert(n::codes("</span>"@) =~= data_024());
    assert(n::clean(data_024(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("</span>"@);
}

pub open spec fn data_025() -> Seq<nat> {
    ns![35u32,32].map_values(|x: u32| x as nat)
}

pub proof fn literal_025()
    ensures
        u::copy_literal_ok(u::copy_registry()[25]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l025(Seq::empty());
    p::all();
    reveal_strlit("# ");
    assert(n::codes("# "@) =~= data_025());
    assert(n::clean(data_025(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("# "@);
}

pub open spec fn data_026() -> Seq<nat> {
    ns![82u32,101,99].map_values(|x: u32| x as nat)
}

pub proof fn literal_026()
    ensures
        u::copy_literal_ok(u::copy_registry()[26]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l026(Seq::empty());
    p::all();
    reveal_strlit("Rec");
    assert(n::codes("Rec"@) =~= data_026());
    assert(n::clean(data_026(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("Rec"@);
}

pub open spec fn data_027() -> Seq<nat> {
    ns![82u32,101,99,111,109,109,101,110,100,97,116,105,111,110,32].map_values(|x: u32| x as nat)
}

pub proof fn literal_027()
    ensures
        u::copy_literal_ok(u::copy_registry()[27]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l027(Seq::empty());
    p::all();
    reveal_strlit("Recommendation ");
    assert(n::codes("Recommendation "@) =~= data_027());
    assert(n::clean(data_027(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("Recommendation "@);
}

pub open spec fn data_028() -> Seq<nat> {
    ns![66u32,79,88].map_values(|x: u32| x as nat)
}

pub proof fn literal_028()
    ensures
        u::copy_literal_ok(u::copy_registry()[28]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l028(Seq::empty());
    p::all();
    reveal_strlit("BOX");
    assert(n::codes("BOX"@) =~= data_028());
    assert(n::clean(data_028(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("BOX"@);
}

pub open spec fn data_029() -> Seq<nat> {
    ns![32u32,183,32].map_values(|x: u32| x as nat)
}

pub proof fn literal_029()
    ensures
        u::copy_literal_ok(u::copy_registry()[29]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l029(Seq::empty());
    p::all();
    reveal_strlit(" · ");
    assert(n::codes(" · "@) =~= data_029());
    assert(n::clean(data_029(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge(" · "@);
}

pub open spec fn data_030() -> Seq<nat> {
    ns![112u32,].map_values(|x: u32| x as nat)
}

pub proof fn literal_030()
    ensures
        u::copy_literal_ok(u::copy_registry()[30]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l030(Seq::empty());
    p::all();
    reveal_strlit("p");
    assert(n::codes("p"@) =~= data_030());
    assert(n::clean(data_030(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("p"@);
}

pub open spec fn data_031() -> Seq<nat> {
    ns![44u32,32,112,97,103,101,32].map_values(|x: u32| x as nat)
}

pub proof fn literal_031()
    ensures
        u::copy_literal_ok(u::copy_registry()[31]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l031(Seq::empty());
    p::all();
    reveal_strlit(", page ");
    assert(n::codes(", page "@) =~= data_031());
    assert(n::clean(data_031(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge(", page "@);
}

} // verus!
}
pub mod batch_032 {
    use crate::{
        k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
        k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
    };
    use ckc_spec::ui as u;
    use vstd::prelude::*;
    use vstd::utf8::*;
    verus! {

pub open spec fn data_032() -> Seq<nat> {
    ns![44u32,32,112,97,115,115,97,103,101,32].map_values(|x: u32| x as nat)
}

pub proof fn literal_032()
    ensures
        u::copy_literal_ok(u::copy_registry()[32]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l032(Seq::empty());
    p::all();
    reveal_strlit(", passage ");
    assert(n::codes(", passage "@) =~= data_032());
    assert(n::clean(data_032(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge(", passage "@);
}

pub open spec fn data_033() -> Seq<nat> {
    ns![32u32,40].map_values(|x: u32| x as nat)
}

pub proof fn literal_033()
    ensures
        u::copy_literal_ok(u::copy_registry()[33]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l033(Seq::empty());
    p::all();
    reveal_strlit(" (");
    assert(n::codes(" ("@) =~= data_033());
    assert(n::clean(data_033(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge(" ("@);
}

pub open spec fn data_034() -> Seq<nat> {
    ns![41u32,].map_values(|x: u32| x as nat)
}

pub proof fn literal_034()
    ensures
        u::copy_literal_ok(u::copy_registry()[34]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l034(Seq::empty());
    p::all();
    reveal_strlit(")");
    assert(n::codes(")"@) =~= data_034());
    assert(n::clean(data_034(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge(")"@);
}

pub open spec fn data_035() -> Seq<nat> {
    ns![90u32,].map_values(|x: u32| x as nat)
}

pub proof fn literal_035()
    ensures
        u::copy_literal_ok(u::copy_registry()[35]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l035(Seq::empty());
    p::all();
    reveal_strlit("Z");
    assert(n::codes("Z"@) =~= data_035());
    assert(n::clean(data_035(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("Z"@);
}

pub open spec fn data_036() -> Seq<nat> {
    ns![32u32,].map_values(|x: u32| x as nat)
}

pub proof fn literal_036()
    ensures
        u::copy_literal_ok(u::copy_registry()[36]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l036(Seq::empty());
    p::all();
    reveal_strlit(" ");
    assert(n::codes(" "@) =~= data_036());
    assert(n::clean(data_036(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge(" "@);
}

pub open spec fn data_037() -> Seq<nat> {
    ns![32u32,85,84,67].map_values(|x: u32| x as nat)
}

pub proof fn literal_037()
    ensures
        u::copy_literal_ok(u::copy_registry()[37]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l037(Seq::empty());
    p::all();
    reveal_strlit(" UTC");
    assert(n::codes(" UTC"@) =~= data_037());
    assert(n::clean(data_037(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge(" UTC"@);
}

pub open spec fn data_040() -> Seq<nat> {
    ns![60u32,33,100,111,99,116,121,112,101,32,104,116,109,108,62].map_values(|x: u32| x as nat)
}

pub proof fn literal_040()
    ensures
        u::copy_literal_ok(u::copy_registry()[40]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l040(Seq::empty());
    p::all();
    reveal_strlit("<!doctype html>");
    assert(n::codes("<!doctype html>"@) =~= data_040());
    assert(n::clean(data_040(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<!doctype html>"@);
}

pub open spec fn data_041() -> Seq<nat> {
    ns![60u32,104,116,109,108,32,108,97,110,103,61,34,101,110,34,62].map_values(|x: u32| x as nat)
}

pub proof fn literal_041()
    ensures
        u::copy_literal_ok(u::copy_registry()[41]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l041(Seq::empty());
    p::all();
    reveal_strlit("<html lang=\"en\">");
    assert(n::codes("<html lang=\"en\">"@) =~= data_041());
    assert(n::clean(data_041(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<html lang=\"en\">"@);
}

pub open spec fn data_042() -> Seq<nat> {
    ns![60u32,104,101,97,100,62].map_values(|x: u32| x as nat)
}

pub proof fn literal_042()
    ensures
        u::copy_literal_ok(u::copy_registry()[42]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l042(Seq::empty());
    p::all();
    reveal_strlit("<head>");
    assert(n::codes("<head>"@) =~= data_042());
    assert(n::clean(data_042(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<head>"@);
}

pub open spec fn data_043() -> Seq<nat> {
    ns![60u32,109,101,116,97,32,99,104,97,114,115,101,116,61,34,117,116,102,45,56,34,62].map_values(
        |x: u32| x as nat,
    )
}

pub proof fn literal_043()
    ensures
        u::copy_literal_ok(u::copy_registry()[43]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l043(Seq::empty());
    p::all();
    reveal_strlit("<meta charset=\"utf-8\">");
    assert(n::codes("<meta charset=\"utf-8\">"@) =~= data_043());
    assert(n::clean(data_043(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<meta charset=\"utf-8\">"@);
}

pub open spec fn data_044() -> Seq<nat> {
    ns![60u32,116,105,116,108,101,62].map_values(|x: u32| x as nat)
}

pub proof fn literal_044()
    ensures
        u::copy_literal_ok(u::copy_registry()[44]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l044(Seq::empty());
    p::all();
    reveal_strlit("<title>");
    assert(n::codes("<title>"@) =~= data_044());
    assert(n::clean(data_044(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<title>"@);
}

pub open spec fn data_045() -> Seq<nat> {
    ns![32u32,8212,32,99,110,108,45,99,107,99,32,114,101,118,105,101,119,101,114,60,47,116,105,116,108,101,62].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_045()
    ensures
        u::copy_literal_ok(u::copy_registry()[45]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l045(Seq::empty());
    p::all();
    reveal_strlit(" — cnl-ckc reviewer</title>");
    assert(n::codes(" — cnl-ckc reviewer</title>"@) =~= data_045());
    assert(n::clean(data_045(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge(" — cnl-ckc reviewer</title>"@);
}

pub open spec fn data_046() -> Seq<nat> {
    ns![60u32,115,116,121,108,101,62].map_values(|x: u32| x as nat)
}

pub proof fn literal_046()
    ensures
        u::copy_literal_ok(u::copy_registry()[46]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l046(Seq::empty());
    p::all();
    reveal_strlit("<style>");
    assert(n::codes("<style>"@) =~= data_046());
    assert(n::clean(data_046(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<style>"@);
}

pub open spec fn data_047() -> Seq<nat> {
    ns![60u32,47,115,116,121,108,101,62].map_values(|x: u32| x as nat)
}

pub proof fn literal_047()
    ensures
        u::copy_literal_ok(u::copy_registry()[47]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l047(Seq::empty());
    p::all();
    reveal_strlit("</style>");
    assert(n::codes("</style>"@) =~= data_047());
    assert(n::clean(data_047(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("</style>"@);
}

} // verus!
}
pub mod batch_048 {
    use crate::{
        k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
        k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
    };
    use ckc_spec::ui as u;
    use vstd::prelude::*;
    use vstd::utf8::*;
    verus! {

pub open spec fn data_048() -> Seq<nat> {
    ns![60u32,47,104,101,97,100,62].map_values(|x: u32| x as nat)
}

pub proof fn literal_048()
    ensures
        u::copy_literal_ok(u::copy_registry()[48]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l048(Seq::empty());
    p::all();
    reveal_strlit("</head>");
    assert(n::codes("</head>"@) =~= data_048());
    assert(n::clean(data_048(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("</head>"@);
}

pub open spec fn data_049() -> Seq<nat> {
    ns![60u32,98,111,100,121,62].map_values(|x: u32| x as nat)
}

pub proof fn literal_049()
    ensures
        u::copy_literal_ok(u::copy_registry()[49]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l049(Seq::empty());
    p::all();
    reveal_strlit("<body>");
    assert(n::codes("<body>"@) =~= data_049());
    assert(n::clean(data_049(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<body>"@);
}

pub open spec fn data_050() -> Seq<nat> {
    ns![60u32,97,32,99,108,97,115,115,61,34,115,107,105,112,34,32,104,114,101,102,61,34,35,109,97,105,110,34,62,83,107,105,112,32,116,111,32,99,111,110,116,101,110,116,60,47,97,62].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_050()
    ensures
        u::copy_literal_ok(u::copy_registry()[50]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l050(Seq::empty());
    p::all();
    reveal_strlit("<a class=\"skip\" href=\"#main\">Skip to content</a>");
    assert(n::codes("<a class=\"skip\" href=\"#main\">Skip to content</a>"@) =~= data_050());
    assert(n::clean(data_050(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<a class=\"skip\" href=\"#main\">Skip to content</a>"@);
}

pub open spec fn data_051() -> Seq<nat> {
    ns![60u32,110,97,118,32,99,108,97,115,115,61,34,99,114,117,109,98,115,34,62].map_values(
        |x: u32| x as nat,
    )
}

pub proof fn literal_051()
    ensures
        u::copy_literal_ok(u::copy_registry()[51]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l051(Seq::empty());
    p::all();
    reveal_strlit("<nav class=\"crumbs\">");
    assert(n::codes("<nav class=\"crumbs\">"@) =~= data_051());
    assert(n::clean(data_051(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<nav class=\"crumbs\">"@);
}

pub open spec fn data_052() -> Seq<nat> {
    ns![60u32,47,110,97,118,62].map_values(|x: u32| x as nat)
}

pub proof fn literal_052()
    ensures
        u::copy_literal_ok(u::copy_registry()[52]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l052(Seq::empty());
    p::all();
    reveal_strlit("</nav>");
    assert(n::codes("</nav>"@) =~= data_052());
    assert(n::clean(data_052(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("</nav>"@);
}

pub open spec fn data_053() -> Seq<nat> {
    ns![60u32,109,97,105,110,32,105,100,61,34,109,97,105,110,34,62].map_values(|x: u32| x as nat)
}

pub proof fn literal_053()
    ensures
        u::copy_literal_ok(u::copy_registry()[53]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l053(Seq::empty());
    p::all();
    reveal_strlit("<main id=\"main\">");
    assert(n::codes("<main id=\"main\">"@) =~= data_053());
    assert(n::clean(data_053(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<main id=\"main\">"@);
}

pub open spec fn data_054() -> Seq<nat> {
    ns![60u32,47,109,97,105,110,62].map_values(|x: u32| x as nat)
}

pub proof fn literal_054()
    ensures
        u::copy_literal_ok(u::copy_registry()[54]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l054(Seq::empty());
    p::all();
    reveal_strlit("</main>");
    assert(n::codes("</main>"@) =~= data_054());
    assert(n::clean(data_054(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("</main>"@);
}

pub open spec fn data_055() -> Seq<nat> {
    ns![60u32,102,111,111,116,101,114,32,99,108,97,115,115,61,34,115,99,111,112,101,34,62,60,112,62,84,104,105,115,32,112,97,103,101,32,114,101,112,111,114,116,115,32,119,104,97,116,32,116,104,101,32,108,111,97,100,101,100,32,103,117,105,100,101,108,105,110,101,32,100,111,99,117,109,101,110,116,115,32,115,116,97,116,101,46,32,73,116,32,100,111,101,115,32,110,111,116,32,103,105,118,101,32,99,108,105,110,105,99,97,108,32,97,100,118,105,99,101,46,60,47,112,62,60,47,102,111,111,116,101,114,62].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_055()
    ensures
        u::copy_literal_ok(u::copy_registry()[55]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l055(Seq::empty());
    p::all();
    reveal_strlit(
        "<footer class=\"scope\"><p>This page reports what the loaded guideline documents state. It does not give clinical advice.</p></footer>",
    );
    assert(n::codes(
        "<footer class=\"scope\"><p>This page reports what the loaded guideline documents state. It does not give clinical advice.</p></footer>"@,
    ) =~= data_055());
    assert(n::clean(data_055(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge(
        "<footer class=\"scope\"><p>This page reports what the loaded guideline documents state. It does not give clinical advice.</p></footer>"@,
    );
}

pub open spec fn data_056() -> Seq<nat> {
    ns![60u32,47,98,111,100,121,62].map_values(|x: u32| x as nat)
}

pub proof fn literal_056()
    ensures
        u::copy_literal_ok(u::copy_registry()[56]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l056(Seq::empty());
    p::all();
    reveal_strlit("</body>");
    assert(n::codes("</body>"@) =~= data_056());
    assert(n::clean(data_056(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("</body>"@);
}

pub open spec fn data_057() -> Seq<nat> {
    ns![60u32,47,104,116,109,108,62].map_values(|x: u32| x as nat)
}

pub proof fn literal_057()
    ensures
        u::copy_literal_ok(u::copy_registry()[57]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l057(Seq::empty());
    p::all();
    reveal_strlit("</html>");
    assert(n::codes("</html>"@) =~= data_057());
    assert(n::clean(data_057(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("</html>"@);
}

pub open spec fn data_058() -> Seq<nat> {
    ns![32u32,97,112,112,114,111,118,101,100].map_values(|x: u32| x as nat)
}

pub proof fn literal_058()
    ensures
        u::copy_literal_ok(u::copy_registry()[58]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l058(Seq::empty());
    p::all();
    reveal_strlit(" approved");
    assert(n::codes(" approved"@) =~= data_058());
    assert(n::clean(data_058(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge(" approved"@);
}

pub open spec fn data_059() -> Seq<nat> {
    ns![32u32,114,101,106,101,99,116,101,100].map_values(|x: u32| x as nat)
}

pub proof fn literal_059()
    ensures
        u::copy_literal_ok(u::copy_registry()[59]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l059(Seq::empty());
    p::all();
    reveal_strlit(" rejected");
    assert(n::codes(" rejected"@) =~= data_059());
    assert(n::clean(data_059(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge(" rejected"@);
}

pub open spec fn data_060() -> Seq<nat> {
    ns![32u32,101,97,114,108,105,101,114].map_values(|x: u32| x as nat)
}

pub proof fn literal_060()
    ensures
        u::copy_literal_ok(u::copy_registry()[60]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l060(Seq::empty());
    p::all();
    reveal_strlit(" earlier");
    assert(n::codes(" earlier"@) =~= data_060());
    assert(n::clean(data_060(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge(" earlier"@);
}

pub open spec fn data_061() -> Seq<nat> {
    ns![78u32,111,110,101].map_values(|x: u32| x as nat)
}

pub proof fn literal_061()
    ensures
        u::copy_literal_ok(u::copy_registry()[61]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l061(Seq::empty());
    p::all();
    reveal_strlit("None");
    assert(n::codes("None"@) =~= data_061());
    assert(n::clean(data_061(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("None"@);
}

pub open spec fn data_062() -> Seq<nat> {
    ns![44u32,32].map_values(|x: u32| x as nat)
}

pub proof fn literal_062()
    ensures
        u::copy_literal_ok(u::copy_registry()[62]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l062(Seq::empty());
    p::all();
    reveal_strlit(", ");
    assert(n::codes(", "@) =~= data_062());
    assert(n::clean(data_062(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge(", "@);
}

pub open spec fn data_063() -> Seq<nat> {
    ns![68u32,101,99,105,115,105,111,110,115,32,111,110,32,116,104,105,115,32,118,101,114,115,105,111,110,58,32].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_063()
    ensures
        u::copy_literal_ok(u::copy_registry()[63]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l063(Seq::empty());
    p::all();
    reveal_strlit("Decisions on this version: ");
    assert(n::codes("Decisions on this version: "@) =~= data_063());
    assert(n::clean(data_063(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("Decisions on this version: "@);
}

} // verus!
}
pub mod batch_064 {
    use crate::{
        k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
        k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
    };
    use ckc_spec::ui as u;
    use vstd::prelude::*;
    use vstd::utf8::*;
    verus! {

pub open spec fn data_064() -> Seq<nat> {
    ns![32u32,97,110,100,32].map_values(|x: u32| x as nat)
}

pub proof fn literal_064()
    ensures
        u::copy_literal_ok(u::copy_registry()[64]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l064(Seq::empty());
    p::all();
    reveal_strlit(" and ");
    assert(n::codes(" and "@) =~= data_064());
    assert(n::clean(data_064(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge(" and "@);
}

pub open spec fn data_065() -> Seq<nat> {
    ns![46u32,].map_values(|x: u32| x as nat)
}

pub proof fn literal_065()
    ensures
        u::copy_literal_ok(u::copy_registry()[65]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l065(Seq::empty());
    p::all();
    reveal_strlit(".");
    assert(n::codes("."@) =~= data_065());
    assert(n::clean(data_065(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("."@);
}

pub open spec fn data_066() -> Seq<nat> {
    ns![78u32,111,32,100,101,99,105,115,105,111,110,32,105,115,32,114,101,99,111,114,100,101,100,32,111,110,32,116,104,105,115,32,118,101,114,115,105,111,110,46].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_066()
    ensures
        u::copy_literal_ok(u::copy_registry()[66]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l066(Seq::empty());
    p::all();
    reveal_strlit("No decision is recorded on this version.");
    assert(n::codes("No decision is recorded on this version."@) =~= data_066());
    assert(n::clean(data_066(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("No decision is recorded on this version."@);
}

pub open spec fn data_067() -> Seq<nat> {
    ns![78u32,111,32,100,101,99,105,115,105,111,110,32,105,115,32,114,101,99,111,114,100,101,100,46].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_067()
    ensures
        u::copy_literal_ok(u::copy_registry()[67]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l067(Seq::empty());
    p::all();
    reveal_strlit("No decision is recorded.");
    assert(n::codes("No decision is recorded."@) =~= data_067());
    assert(n::clean(data_067(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("No decision is recorded."@);
}

pub open spec fn data_068() -> Seq<nat> {
    ns![32u32,68,101,99,105,115,105,111,110,115,32,111,110,32,101,97,114,108,105,101,114,32,118,101,114,115,105,111,110,115,58,32].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_068()
    ensures
        u::copy_literal_ok(u::copy_registry()[68]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l068(Seq::empty());
    p::all();
    reveal_strlit(" Decisions on earlier versions: ");
    assert(n::codes(" Decisions on earlier versions: "@) =~= data_068());
    assert(n::clean(data_068(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge(" Decisions on earlier versions: "@);
}

pub open spec fn data_069() -> Seq<nat> {
    ns![78u32,111,32,100,101,99,105,115,105,111,110,115,32,97,114,101,32,114,101,99,111,114,100,101,100,32,102,111,114,32,116,104,101,32].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_069()
    ensures
        u::copy_literal_ok(u::copy_registry()[69]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l069(Seq::empty());
    p::all();
    reveal_strlit("No decisions are recorded for the ");
    assert(n::codes("No decisions are recorded for the "@) =~= data_069());
    assert(n::clean(data_069(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("No decisions are recorded for the "@);
}

pub open spec fn data_070() -> Seq<nat> {
    ns![32u32,100,111,99,117,109,101,110,116,115,32,105,110,32,116,104,105,115,32,103,117,105,100,101,108,105,110,101,46].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_070()
    ensures
        u::copy_literal_ok(u::copy_registry()[70]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l070(Seq::empty());
    p::all();
    reveal_strlit(" documents in this guideline.");
    assert(n::codes(" documents in this guideline."@) =~= data_070());
    assert(n::clean(data_070(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge(" documents in this guideline."@);
}

pub open spec fn data_071() -> Seq<nat> {
    ns![82u32,101,118,105,101,119,101,114,115,32,114,101,99,111,114,100,101,100,32].map_values(
        |x: u32| x as nat,
    )
}

pub proof fn literal_071()
    ensures
        u::copy_literal_ok(u::copy_registry()[71]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l071(Seq::empty());
    p::all();
    reveal_strlit("Reviewers recorded ");
    assert(n::codes("Reviewers recorded "@) =~= data_071());
    assert(n::clean(data_071(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("Reviewers recorded "@);
}

pub open spec fn data_072() -> Seq<nat> {
    ns![32u32,100,101,99,105,115,105,111,110,115,32,111,110,32].map_values(|x: u32| x as nat)
}

pub proof fn literal_072()
    ensures
        u::copy_literal_ok(u::copy_registry()[72]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l072(Seq::empty());
    p::all();
    reveal_strlit(" decisions on ");
    assert(n::codes(" decisions on "@) =~= data_072());
    assert(n::clean(data_072(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge(" decisions on "@);
}

pub open spec fn data_073() -> Seq<nat> {
    ns![32u32,111,102,32].map_values(|x: u32| x as nat)
}

pub proof fn literal_073()
    ensures
        u::copy_literal_ok(u::copy_registry()[73]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l073(Seq::empty());
    p::all();
    reveal_strlit(" of ");
    assert(n::codes(" of "@) =~= data_073());
    assert(n::clean(data_073(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge(" of "@);
}

pub open spec fn data_074() -> Seq<nat> {
    ns![32u32,100,111,99,117,109,101,110,116,115,46].map_values(|x: u32| x as nat)
}

pub proof fn literal_074()
    ensures
        u::copy_literal_ok(u::copy_registry()[74]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l074(Seq::empty());
    p::all();
    reveal_strlit(" documents.");
    assert(n::codes(" documents."@) =~= data_074());
    assert(n::clean(data_074(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge(" documents."@);
}

pub open spec fn data_075() -> Seq<nat> {
    ns![103u32,47].map_values(|x: u32| x as nat)
}

pub proof fn literal_075()
    ensures
        u::copy_literal_ok(u::copy_registry()[75]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l075(Seq::empty());
    p::all();
    reveal_strlit("g/");
    assert(n::codes("g/"@) =~= data_075());
    assert(n::clean(data_075(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("g/"@);
}

pub open spec fn data_076() -> Seq<nat> {
    ns![47u32,105,110,100,101,120,46,104,116,109,108].map_values(|x: u32| x as nat)
}

pub proof fn literal_076()
    ensures
        u::copy_literal_ok(u::copy_registry()[76]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l076(Seq::empty());
    p::all();
    reveal_strlit("/index.html");
    assert(n::codes("/index.html"@) =~= data_076());
    assert(n::clean(data_076(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("/index.html"@);
}

pub open spec fn data_077() -> Seq<nat> {
    ns![71u32,117,105,100,101,108,105,110,101,115].map_values(|x: u32| x as nat)
}

pub proof fn literal_077()
    ensures
        u::copy_literal_ok(u::copy_registry()[77]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l077(Seq::empty());
    p::all();
    reveal_strlit("Guidelines");
    assert(n::codes("Guidelines"@) =~= data_077());
    assert(n::clean(data_077(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("Guidelines"@);
}

pub open spec fn data_078() -> Seq<nat> {
    ns![99u32,110,108,45,99,107,99,32,114,101,118,105,101,119,101,114].map_values(|x: u32| x as nat)
}

pub proof fn literal_078()
    ensures
        u::copy_literal_ok(u::copy_registry()[78]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l078(Seq::empty());
    p::all();
    reveal_strlit("cnl-ckc reviewer");
    assert(n::codes("cnl-ckc reviewer"@) =~= data_078());
    assert(n::clean(data_078(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("cnl-ckc reviewer"@);
}

pub open spec fn data_079() -> Seq<nat> {
    ns![60u32,104,49,62,71,117,105,100,101,108,105,110,101,115,60,47,104,49,62].map_values(
        |x: u32| x as nat,
    )
}

pub proof fn literal_079()
    ensures
        u::copy_literal_ok(u::copy_registry()[79]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l079(Seq::empty());
    p::all();
    reveal_strlit("<h1>Guidelines</h1>");
    assert(n::codes("<h1>Guidelines</h1>"@) =~= data_079());
    assert(n::clean(data_079(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<h1>Guidelines</h1>"@);
}

} // verus!
}
pub mod batch_080 {
    use crate::{
        k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
        k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
    };
    use ckc_spec::ui as u;
    use vstd::prelude::*;
    use vstd::utf8::*;
    verus! {

pub open spec fn data_080() -> Seq<nat> {
    ns![60u32,115,101,99,116,105,111,110,62].map_values(|x: u32| x as nat)
}

pub proof fn literal_080()
    ensures
        u::copy_literal_ok(u::copy_registry()[80]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l080(Seq::empty());
    p::all();
    reveal_strlit("<section>");
    assert(n::codes("<section>"@) =~= data_080());
    assert(n::clean(data_080(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<section>"@);
}

pub open spec fn data_081() -> Seq<nat> {
    ns![60u32,116,97,98,108,101,62].map_values(|x: u32| x as nat)
}

pub proof fn literal_081()
    ensures
        u::copy_literal_ok(u::copy_registry()[81]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l081(Seq::empty());
    p::all();
    reveal_strlit("<table>");
    assert(n::codes("<table>"@) =~= data_081());
    assert(n::clean(data_081(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<table>"@);
}

pub open spec fn data_082() -> Seq<nat> {
    ns![60u32,116,104,101,97,100,62,60,116,114,62,60,116,104,62,71,117,105,100,101,108,105,110,101,60,47,116,104,62,60,116,104,62,68,111,99,117,109,101,110,116,115,60,47,116,104,62,60,116,104,62,80,97,115,115,97,103,101,115,60,47,116,104,62,60,116,104,62,65,112,112,114,111,118,101,100,60,47,116,104,62,60,116,104,62,82,101,106,101,99,116,101,100,60,47,116,104,62,60,116,104,62,67,111,110,116,101,115,116,101,100,60,47,116,104,62,60,116,104,62,79,117,116,100,97,116,101,100,60,47,116,104,62,60,116,104,62,85,110,114,101,118,105,101,119,101,100,60,47,116,104,62,60,47,116,114,62,60,47,116,104,101,97,100,62].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_082()
    ensures
        u::copy_literal_ok(u::copy_registry()[82]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l082(Seq::empty());
    p::all();
    reveal_strlit(
        "<thead><tr><th>Guideline</th><th>Documents</th><th>Passages</th><th>Approved</th><th>Rejected</th><th>Contested</th><th>Outdated</th><th>Unreviewed</th></tr></thead>",
    );
    assert(n::codes(
        "<thead><tr><th>Guideline</th><th>Documents</th><th>Passages</th><th>Approved</th><th>Rejected</th><th>Contested</th><th>Outdated</th><th>Unreviewed</th></tr></thead>"@,
    ) =~= data_082());
    assert(n::clean(data_082(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge(
        "<thead><tr><th>Guideline</th><th>Documents</th><th>Passages</th><th>Approved</th><th>Rejected</th><th>Contested</th><th>Outdated</th><th>Unreviewed</th></tr></thead>"@,
    );
}

pub open spec fn data_083() -> Seq<nat> {
    ns![60u32,116,98,111,100,121,62].map_values(|x: u32| x as nat)
}

pub proof fn literal_083()
    ensures
        u::copy_literal_ok(u::copy_registry()[83]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l083(Seq::empty());
    p::all();
    reveal_strlit("<tbody>");
    assert(n::codes("<tbody>"@) =~= data_083());
    assert(n::clean(data_083(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<tbody>"@);
}

pub open spec fn data_084() -> Seq<nat> {
    ns![60u32,47,116,98,111,100,121,62].map_values(|x: u32| x as nat)
}

pub proof fn literal_084()
    ensures
        u::copy_literal_ok(u::copy_registry()[84]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l084(Seq::empty());
    p::all();
    reveal_strlit("</tbody>");
    assert(n::codes("</tbody>"@) =~= data_084());
    assert(n::clean(data_084(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("</tbody>"@);
}

pub open spec fn data_085() -> Seq<nat> {
    ns![60u32,47,116,97,98,108,101,62].map_values(|x: u32| x as nat)
}

pub proof fn literal_085()
    ensures
        u::copy_literal_ok(u::copy_registry()[85]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l085(Seq::empty());
    p::all();
    reveal_strlit("</table>");
    assert(n::codes("</table>"@) =~= data_085());
    assert(n::clean(data_085(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("</table>"@);
}

pub open spec fn data_086() -> Seq<nat> {
    ns![60u32,47,115,101,99,116,105,111,110,62].map_values(|x: u32| x as nat)
}

pub proof fn literal_086()
    ensures
        u::copy_literal_ok(u::copy_registry()[86]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l086(Seq::empty());
    p::all();
    reveal_strlit("</section>");
    assert(n::codes("</section>"@) =~= data_086());
    assert(n::clean(data_086(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("</section>"@);
}

pub open spec fn data_087() -> Seq<nat> {
    ns![82u32,101,115,116,97,116,101,115,32].map_values(|x: u32| x as nat)
}

pub proof fn literal_087()
    ensures
        u::copy_literal_ok(u::copy_registry()[87]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l087(Seq::empty());
    p::all();
    reveal_strlit("Restates ");
    assert(n::codes("Restates "@) =~= data_087());
    assert(n::clean(data_087(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("Restates "@);
}

pub open spec fn data_088() -> Seq<nat> {
    ns![114u32,101,115,116,97,116,101,115,40].map_values(|x: u32| x as nat)
}

pub proof fn literal_088()
    ensures
        u::copy_literal_ok(u::copy_registry()[88]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l088(Seq::empty());
    p::all();
    reveal_strlit("restates(");
    assert(n::codes("restates("@) =~= data_088());
    assert(n::clean(data_088(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("restates("@);
}

pub open spec fn data_089() -> Seq<nat> {
    ns![117u32,110,99,111,118,101,114,101,100,40].map_values(|x: u32| x as nat)
}

pub proof fn literal_089()
    ensures
        u::copy_literal_ok(u::copy_registry()[89]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l089(Seq::empty());
    p::all();
    reveal_strlit("uncovered(");
    assert(n::codes("uncovered("@) =~= data_089());
    assert(n::clean(data_089(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("uncovered("@);
}

pub open spec fn data_090() -> Seq<nat> {
    ns![58u32,32].map_values(|x: u32| x as nat)
}

pub proof fn literal_090()
    ensures
        u::copy_literal_ok(u::copy_registry()[90]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l090(Seq::empty());
    p::all();
    reveal_strlit(": ");
    assert(n::codes(": "@) =~= data_090());
    assert(n::clean(data_090(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge(": "@);
}

pub open spec fn data_091() -> Seq<nat> {
    ns![78u32,111,116,32,99,111,118,101,114,101,100,32,8212,32].map_values(|x: u32| x as nat)
}

pub proof fn literal_091()
    ensures
        u::copy_literal_ok(u::copy_registry()[91]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l091(Seq::empty());
    p::all();
    reveal_strlit("Not covered — ");
    assert(n::codes("Not covered — "@) =~= data_091());
    assert(n::clean(data_091(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("Not covered — "@);
}

pub open spec fn data_092() -> Seq<nat> {
    ns![80u32,101,110,100,105,110,103].map_values(|x: u32| x as nat)
}

pub proof fn literal_092()
    ensures
        u::copy_literal_ok(u::copy_registry()[92]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l092(Seq::empty());
    p::all();
    reveal_strlit("Pending");
    assert(n::codes("Pending"@) =~= data_092());
    assert(n::clean(data_092(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("Pending"@);
}

pub open spec fn data_093() -> Seq<nat> {
    ns![80u32,97,115,115,97,103,101,115].map_values(|x: u32| x as nat)
}

pub proof fn literal_093()
    ensures
        u::copy_literal_ok(u::copy_registry()[93]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l093(Seq::empty());
    p::all();
    reveal_strlit("Passages");
    assert(n::codes("Passages"@) =~= data_093());
    assert(n::clean(data_093(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("Passages"@);
}

pub open spec fn data_094() -> Seq<nat> {
    ns![87u32,105,116,104,32,65,67,69].map_values(|x: u32| x as nat)
}

pub proof fn literal_094()
    ensures
        u::copy_literal_ok(u::copy_registry()[94]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l094(Seq::empty());
    p::all();
    reveal_strlit("With ACE");
    assert(n::codes("With ACE"@) =~= data_094());
    assert(n::clean(data_094(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("With ACE"@);
}

pub open spec fn data_095() -> Seq<nat> {
    ns![100u32,111,99,47].map_values(|x: u32| x as nat)
}

pub proof fn literal_095()
    ensures
        u::copy_literal_ok(u::copy_registry()[95]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l095(Seq::empty());
    p::all();
    reveal_strlit("doc/");
    assert(n::codes("doc/"@) =~= data_095());
    assert(n::clean(data_095(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("doc/"@);
}

} // verus!
}
pub mod batch_096 {
    use crate::{
        k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
        k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
    };
    use ckc_spec::ui as u;
    use vstd::prelude::*;
    use vstd::utf8::*;
    verus! {

pub open spec fn data_096() -> Seq<nat> {
    ns![46u32,104,116,109,108].map_values(|x: u32| x as nat)
}

pub proof fn literal_096()
    ensures
        u::copy_literal_ok(u::copy_registry()[96]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l096(Seq::empty());
    p::all();
    reveal_strlit(".html");
    assert(n::codes(".html"@) =~= data_096());
    assert(n::clean(data_096(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge(".html"@);
}

pub open spec fn data_097() -> Seq<nat> {
    ns![60u32,104,49,62].map_values(|x: u32| x as nat)
}

pub proof fn literal_097()
    ensures
        u::copy_literal_ok(u::copy_registry()[97]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l097(Seq::empty());
    p::all();
    reveal_strlit("<h1>");
    assert(n::codes("<h1>"@) =~= data_097());
    assert(n::clean(data_097(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<h1>"@);
}

pub open spec fn data_098() -> Seq<nat> {
    ns![60u32,47,104,49,62].map_values(|x: u32| x as nat)
}

pub proof fn literal_098()
    ensures
        u::copy_literal_ok(u::copy_registry()[98]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l098(Seq::empty());
    p::all();
    reveal_strlit("</h1>");
    assert(n::codes("</h1>"@) =~= data_098());
    assert(n::clean(data_098(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("</h1>"@);
}

pub open spec fn data_099() -> Seq<nat> {
    ns![60u32,112,62].map_values(|x: u32| x as nat)
}

pub proof fn literal_099()
    ensures
        u::copy_literal_ok(u::copy_registry()[99]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l099(Seq::empty());
    p::all();
    reveal_strlit("<p>");
    assert(n::codes("<p>"@) =~= data_099());
    assert(n::clean(data_099(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<p>"@);
}

pub open spec fn data_100() -> Seq<nat> {
    ns![32u32,60,97,32,104,114,101,102,61,34,114,101,99,111,114,100,115,46,104,116,109,108,34,62,65,108,108,32,100,101,99,105,115,105,111,110,32,114,101,99,111,114,100,115,60,47,97,62,60,47,112,62].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_100()
    ensures
        u::copy_literal_ok(u::copy_registry()[100]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l100(Seq::empty());
    p::all();
    reveal_strlit(" <a href=\"records.html\">All decision records</a></p>");
    assert(n::codes(" <a href=\"records.html\">All decision records</a></p>"@) =~= data_100());
    assert(n::clean(data_100(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge(" <a href=\"records.html\">All decision records</a></p>"@);
}

pub open spec fn data_101() -> Seq<nat> {
    ns![60u32,104,50,62,83,116,97,116,117,115,60,47,104,50,62].map_values(|x: u32| x as nat)
}

pub proof fn literal_101()
    ensures
        u::copy_literal_ok(u::copy_registry()[101]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l101(Seq::empty());
    p::all();
    reveal_strlit("<h2>Status</h2>");
    assert(n::codes("<h2>Status</h2>"@) =~= data_101());
    assert(n::clean(data_101(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<h2>Status</h2>"@);
}

pub open spec fn data_102() -> Seq<nat> {
    ns![60u32,116,97,98,108,101,32,99,108,97,115,115,61,34,99,111,109,112,97,99,116,34,62].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_102()
    ensures
        u::copy_literal_ok(u::copy_registry()[102]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l102(Seq::empty());
    p::all();
    reveal_strlit("<table class=\"compact\">");
    assert(n::codes("<table class=\"compact\">"@) =~= data_102());
    assert(n::clean(data_102(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<table class=\"compact\">"@);
}

pub open spec fn data_103() -> Seq<nat> {
    ns![60u32,116,104,101,97,100,62,60,116,114,62,60,116,104,62,83,116,97,116,117,115,60,47,116,104,62,60,116,104,62,67,111,117,110,116,60,47,116,104,62,60,47,116,114,62,60,47,116,104,101,97,100,62].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_103()
    ensures
        u::copy_literal_ok(u::copy_registry()[103]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l103(Seq::empty());
    p::all();
    reveal_strlit("<thead><tr><th>Status</th><th>Count</th></tr></thead>");
    assert(n::codes("<thead><tr><th>Status</th><th>Count</th></tr></thead>"@) =~= data_103());
    assert(n::clean(data_103(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<thead><tr><th>Status</th><th>Count</th></tr></thead>"@);
}

pub open spec fn data_104() -> Seq<nat> {
    ns![60u32,104,50,62,68,111,99,117,109,101,110,116,115,60,47,104,50,62].map_values(
        |x: u32| x as nat,
    )
}

pub proof fn literal_104()
    ensures
        u::copy_literal_ok(u::copy_registry()[104]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l104(Seq::empty());
    p::all();
    reveal_strlit("<h2>Documents</h2>");
    assert(n::codes("<h2>Documents</h2>"@) =~= data_104());
    assert(n::clean(data_104(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<h2>Documents</h2>"@);
}

pub open spec fn data_105() -> Seq<nat> {
    ns![60u32,116,104,101,97,100,62,60,116,114,62,60,116,104,62,68,111,99,117,109,101,110,116,60,47,116,104,62,60,116,104,62,83,116,97,116,117,115,60,47,116,104,62,60,116,104,62,68,101,99,105,115,105,111,110,115,60,47,116,104,62,60,116,104,62,80,97,115,115,97,103,101,60,47,116,104,62,60,47,116,114,62,60,47,116,104,101,97,100,62].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_105()
    ensures
        u::copy_literal_ok(u::copy_registry()[105]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l105(Seq::empty());
    p::all();
    reveal_strlit(
        "<thead><tr><th>Document</th><th>Status</th><th>Decisions</th><th>Passage</th></tr></thead>",
    );
    assert(n::codes(
        "<thead><tr><th>Document</th><th>Status</th><th>Decisions</th><th>Passage</th></tr></thead>"@,
    ) =~= data_105());
    assert(n::clean(data_105(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge(
        "<thead><tr><th>Document</th><th>Status</th><th>Decisions</th><th>Passage</th></tr></thead>"@,
    );
}

pub open spec fn data_106() -> Seq<nat> {
    ns![60u32,104,50,62,80,97,115,115,97,103,101,115,32,119,105,116,104,111,117,116,32,65,67,69,60,47,104,50,62].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_106()
    ensures
        u::copy_literal_ok(u::copy_registry()[106]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l106(Seq::empty());
    p::all();
    reveal_strlit("<h2>Passages without ACE</h2>");
    assert(n::codes("<h2>Passages without ACE</h2>"@) =~= data_106());
    assert(n::clean(data_106(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<h2>Passages without ACE</h2>"@);
}

pub open spec fn data_107() -> Seq<nat> {
    ns![60u32,116,104,101,97,100,62,60,116,114,62,60,116,104,62,80,97,115,115,97,103,101,60,47,116,104,62,60,116,104,62,83,116,97,116,117,115,60,47,116,104,62,60,116,104,62,83,101,99,116,105,111,110,60,47,116,104,62,60,47,116,114,62,60,47,116,104,101,97,100,62].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_107()
    ensures
        u::copy_literal_ok(u::copy_registry()[107]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l107(Seq::empty());
    p::all();
    reveal_strlit("<thead><tr><th>Passage</th><th>Status</th><th>Section</th></tr></thead>");
    assert(n::codes("<thead><tr><th>Passage</th><th>Status</th><th>Section</th></tr></thead>"@)
        =~= data_107());
    assert(n::clean(data_107(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<thead><tr><th>Passage</th><th>Status</th><th>Section</th></tr></thead>"@);
}

pub open spec fn data_108() -> Seq<nat> {
    ns![60u32,97,32,104,114,101,102,61,34,46,46,47,46,46,47,105,110,100,101,120,46,104,116,109,108,34,62,103,117,105,100,101,108,105,110,101,115,60,47,97,62,32,47,32].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_108()
    ensures
        u::copy_literal_ok(u::copy_registry()[108]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l108(Seq::empty());
    p::all();
    reveal_strlit("<a href=\"../../index.html\">guidelines</a> / ");
    assert(n::codes("<a href=\"../../index.html\">guidelines</a> / "@) =~= data_108());
    assert(n::clean(data_108(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<a href=\"../../index.html\">guidelines</a> / "@);
}

pub open spec fn data_109() -> Seq<nat> {
    ns![104u32,116,116,112,115,58,47,47,103,105,116,104,117,98,46,99,111,109,47,101,116,117,114,107,101,115,47,99,110,108,45,99,107,99,47,99,111,109,109,105,116,47].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_109()
    ensures
        u::copy_literal_ok(u::copy_registry()[109]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l109(Seq::empty());
    p::all();
    reveal_strlit("https://github.com/eturkes/cnl-ckc/commit/");
    assert(n::codes("https://github.com/eturkes/cnl-ckc/commit/"@) =~= data_109());
    assert(n::clean(data_109(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("https://github.com/eturkes/cnl-ckc/commit/"@);
}

pub open spec fn data_110() -> Seq<nat> {
    ns![67u32,117,114,114,101,110,116].map_values(|x: u32| x as nat)
}

pub proof fn literal_110()
    ensures
        u::copy_literal_ok(u::copy_registry()[110]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l110(Seq::empty());
    p::all();
    reveal_strlit("Current");
    assert(n::codes("Current"@) =~= data_110());
    assert(n::clean(data_110(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("Current"@);
}

pub open spec fn data_111() -> Seq<nat> {
    ns![69u32,97,114,108,105,101,114].map_values(|x: u32| x as nat)
}

pub proof fn literal_111()
    ensures
        u::copy_literal_ok(u::copy_registry()[111]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l111(Seq::empty());
    p::all();
    reveal_strlit("Earlier");
    assert(n::codes("Earlier"@) =~= data_111());
    assert(n::clean(data_111(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("Earlier"@);
}

} // verus!
}
pub mod batch_112 {
    use crate::{
        k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
        k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
    };
    use ckc_spec::ui as u;
    use vstd::prelude::*;
    use vstd::utf8::*;
    verus! {

pub open spec fn data_112() -> Seq<nat> {
    ns![78u32,111,116,32,103,105,118,101,110].map_values(|x: u32| x as nat)
}

pub proof fn literal_112()
    ensures
        u::copy_literal_ok(u::copy_registry()[112]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l112(Seq::empty());
    p::all();
    reveal_strlit("Not given");
    assert(n::codes("Not given"@) =~= data_112());
    assert(n::clean(data_112(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("Not given"@);
}

pub open spec fn data_113() -> Seq<nat> {
    ns![60u32,115,101,99,116,105,111,110,32,105,100,61,34].map_values(|x: u32| x as nat)
}

pub proof fn literal_113()
    ensures
        u::copy_literal_ok(u::copy_registry()[113]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l113(Seq::empty());
    p::all();
    reveal_strlit("<section id=\"");
    assert(n::codes("<section id=\""@) =~= data_113());
    assert(n::clean(data_113(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<section id=\""@);
}

pub open spec fn data_114() -> Seq<nat> {
    ns![60u32,104,50,62].map_values(|x: u32| x as nat)
}

pub proof fn literal_114()
    ensures
        u::copy_literal_ok(u::copy_registry()[114]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l114(Seq::empty());
    p::all();
    reveal_strlit("<h2>");
    assert(n::codes("<h2>"@) =~= data_114());
    assert(n::clean(data_114(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<h2>"@);
}

pub open spec fn data_115() -> Seq<nat> {
    ns![60u32,47,104,50,62].map_values(|x: u32| x as nat)
}

pub proof fn literal_115()
    ensures
        u::copy_literal_ok(u::copy_registry()[115]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l115(Seq::empty());
    p::all();
    reveal_strlit("</h2>");
    assert(n::codes("</h2>"@) =~= data_115());
    assert(n::clean(data_115(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("</h2>"@);
}

pub open spec fn data_116() -> Seq<nat> {
    ns![60u32,116,97,98,108,101,32,99,108,97,115,115,61,34,114,101,99,111,114,100,115,34,62].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_116()
    ensures
        u::copy_literal_ok(u::copy_registry()[116]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l116(Seq::empty());
    p::all();
    reveal_strlit("<table class=\"records\">");
    assert(n::codes("<table class=\"records\">"@) =~= data_116());
    assert(n::clean(data_116(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<table class=\"records\">"@);
}

pub open spec fn data_117() -> Seq<nat> {
    ns![60u32,116,104,101,97,100,62,60,116,114,62,60,116,104,62,68,101,99,105,115,105,111,110,60,47,116,104,62,60,116,104,62,82,101,118,105,101,119,101,114,60,47,116,104,62,60,116,104,62,68,97,116,101,60,47,116,104,62,60,116,104,62,86,101,114,115,105,111,110,60,47,116,104,62,60,116,104,62,67,111,109,109,101,110,116,60,47,116,104,62,60,47,116,114,62,60,47,116,104,101,97,100,62].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_117()
    ensures
        u::copy_literal_ok(u::copy_registry()[117]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l117(Seq::empty());
    p::all();
    reveal_strlit(
        "<thead><tr><th>Decision</th><th>Reviewer</th><th>Date</th><th>Version</th><th>Comment</th></tr></thead>",
    );
    assert(n::codes(
        "<thead><tr><th>Decision</th><th>Reviewer</th><th>Date</th><th>Version</th><th>Comment</th></tr></thead>"@,
    ) =~= data_117());
    assert(n::clean(data_117(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge(
        "<thead><tr><th>Decision</th><th>Reviewer</th><th>Date</th><th>Version</th><th>Comment</th></tr></thead>"@,
    );
}

pub open spec fn data_118() -> Seq<nat> {
    ns![32u32,84,104,101,32,110,101,119,101,115,116,32,100,101,99,105,115,105,111,110,32,102,111,114,32,101,97,99,104,32,100,111,99,117,109,101,110,116,32,105,115,32,102,105,114,115,116,46].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_118()
    ensures
        u::copy_literal_ok(u::copy_registry()[118]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l118(Seq::empty());
    p::all();
    reveal_strlit(" The newest decision for each document is first.");
    assert(n::codes(" The newest decision for each document is first."@) =~= data_118());
    assert(n::clean(data_118(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge(" The newest decision for each document is first."@);
}

pub open spec fn data_119() -> Seq<nat> {
    ns![60u32,112,62,79,112,101,110,32,97,32,100,111,99,117,109,101,110,116,32,97,110,100,32,114,101,99,111,114,100,32,97,32,100,101,99,105,115,105,111,110,32,116,111,32,115,116,97,114,116,32,116,104,105,115,32,108,105,115,116,46,60,47,112,62].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_119()
    ensures
        u::copy_literal_ok(u::copy_registry()[119]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l119(Seq::empty());
    p::all();
    reveal_strlit("<p>Open a document and record a decision to start this list.</p>");
    assert(n::codes("<p>Open a document and record a decision to start this list.</p>"@)
        =~= data_119());
    assert(n::clean(data_119(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<p>Open a document and record a decision to start this list.</p>"@);
}

pub open spec fn data_120() -> Seq<nat> {
    ns![60u32,112,62,69,97,99,104,32,114,101,118,105,101,119,101,114,32,110,97,109,101,32,105,115,32,114,101,99,111,114,100,101,100,32,97,115,32,101,110,116,101,114,101,100,32,97,110,100,32,105,115,32,110,111,116,32,118,101,114,105,102,105,101,100,46,60,47,112,62].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_120()
    ensures
        u::copy_literal_ok(u::copy_registry()[120]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l120(Seq::empty());
    p::all();
    reveal_strlit("<p>Each reviewer name is recorded as entered and is not verified.</p>");
    assert(n::codes("<p>Each reviewer name is recorded as entered and is not verified.</p>"@)
        =~= data_120());
    assert(n::clean(data_120(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<p>Each reviewer name is recorded as entered and is not verified.</p>"@);
}

pub open spec fn data_121() -> Seq<nat> {
    ns![60u32,112,62,69,97,99,104,32,118,101,114,115,105,111,110,32,108,105,110,107,115,32,116,111,32,116,104,101,32,115,116,111,114,101,100,32,118,101,114,115,105,111,110,32,111,102,32,116,104,101,32,116,101,120,116,32,116,104,97,116,32,116,104,101,32,114,101,118,105,101,119,101,114,32,114,101,97,100,46,60,47,112,62].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_121()
    ensures
        u::copy_literal_ok(u::copy_registry()[121]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l121(Seq::empty());
    p::all();
    reveal_strlit(
        "<p>Each version links to the stored version of the text that the reviewer read.</p>",
    );
    assert(n::codes(
        "<p>Each version links to the stored version of the text that the reviewer read.</p>"@,
    ) =~= data_121());
    assert(n::clean(data_121(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge(
        "<p>Each version links to the stored version of the text that the reviewer read.</p>"@,
    );
}

pub open spec fn data_122() -> Seq<nat> {
    ns![68u32,101,99,105,115,105,111,110,32,114,101,99,111,114,100,115].map_values(
        |x: u32| x as nat,
    )
}

pub proof fn literal_122()
    ensures
        u::copy_literal_ok(u::copy_registry()[122]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l122(Seq::empty());
    p::all();
    reveal_strlit("Decision records");
    assert(n::codes("Decision records"@) =~= data_122());
    assert(n::clean(data_122(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("Decision records"@);
}

pub open spec fn data_123() -> Seq<nat> {
    ns![60u32,97,32,104,114,101,102,61,34,46,46,47,46,46,47,105,110,100,101,120,46,104,116,109,108,34,62,103,117,105,100,101,108,105,110,101,115,60,47,97,62,32,47,32,60,97,32,104,114,101,102,61,34,105,110,100,101,120,46,104,116,109,108,34,62].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_123()
    ensures
        u::copy_literal_ok(u::copy_registry()[123]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l123(Seq::empty());
    p::all();
    reveal_strlit("<a href=\"../../index.html\">guidelines</a> / <a href=\"index.html\">");
    assert(n::codes("<a href=\"../../index.html\">guidelines</a> / <a href=\"index.html\">"@)
        =~= data_123());
    assert(n::clean(data_123(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<a href=\"../../index.html\">guidelines</a> / <a href=\"index.html\">"@);
}

pub open spec fn data_124() -> Seq<nat> {
    ns![60u32,47,97,62,32,47,32,114,101,99,111,114,100,115].map_values(|x: u32| x as nat)
}

pub proof fn literal_124()
    ensures
        u::copy_literal_ok(u::copy_registry()[124]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l124(Seq::empty());
    p::all();
    reveal_strlit("</a> / records");
    assert(n::codes("</a> / records"@) =~= data_124());
    assert(n::clean(data_124(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("</a> / records"@);
}

pub open spec fn data_125() -> Seq<nat> {
    ns![60u32,104,49,62,68,101,99,105,115,105,111,110,32,114,101,99,111,114,100,115,60,47,104,49,62].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_125()
    ensures
        u::copy_literal_ok(u::copy_registry()[125]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l125(Seq::empty());
    p::all();
    reveal_strlit("<h1>Decision records</h1>");
    assert(n::codes("<h1>Decision records</h1>"@) =~= data_125());
    assert(n::clean(data_125(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<h1>Decision records</h1>"@);
}

pub open spec fn data_126() -> Seq<nat> {
    ns![60u32,47,112,62].map_values(|x: u32| x as nat)
}

pub proof fn literal_126()
    ensures
        u::copy_literal_ok(u::copy_registry()[126]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l126(Seq::empty());
    p::all();
    reveal_strlit("</p>");
    assert(n::codes("</p>"@) =~= data_126());
    assert(n::clean(data_126(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("</p>"@);
}

pub open spec fn data_127() -> Seq<nat> {
    ns![60u32,110,97,118,32,99,108,97,115,115,61,34,100,111,99,110,97,118,34,62,60,97,32,104,114,101,102,61,34,105,110,100,101,120,46,104,116,109,108,34,62,71,117,105,100,101,108,105,110,101,32,105,110,100,101,120,60,47,97,62,60,47,110,97,118,62].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_127()
    ensures
        u::copy_literal_ok(u::copy_registry()[127]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l127(Seq::empty());
    p::all();
    reveal_strlit("<nav class=\"docnav\"><a href=\"index.html\">Guideline index</a></nav>");
    assert(n::codes("<nav class=\"docnav\"><a href=\"index.html\">Guideline index</a></nav>"@)
        =~= data_127());
    assert(n::clean(data_127(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<nav class=\"docnav\"><a href=\"index.html\">Guideline index</a></nav>"@);
}

} // verus!
}
pub mod batch_128 {
    use crate::{
        k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
        k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
    };
    use ckc_spec::ui as u;
    use vstd::prelude::*;
    use vstd::utf8::*;
    verus! {

pub open spec fn data_128() -> Seq<nat> {
    ns![60u32,115,112,97,110,32,99,108,97,115,115,61,34,107,119,34,62].map_values(|x: u32| x as nat)
}

pub proof fn literal_128()
    ensures
        u::copy_literal_ok(u::copy_registry()[128]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l128(Seq::empty());
    p::all();
    reveal_strlit("<span class=\"kw\">");
    assert(n::codes("<span class=\"kw\">"@) =~= data_128());
    assert(n::clean(data_128(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<span class=\"kw\">"@);
}

pub open spec fn data_129() -> Seq<nat> {
    ns![60u32,109,97,114,107,32,99,108,97,115,115,61,34,116].map_values(|x: u32| x as nat)
}

pub proof fn literal_129()
    ensures
        u::copy_literal_ok(u::copy_registry()[129]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l129(Seq::empty());
    p::all();
    reveal_strlit("<mark class=\"t");
    assert(n::codes("<mark class=\"t"@) =~= data_129());
    assert(n::clean(data_129(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<mark class=\"t"@);
}

pub open spec fn data_130() -> Seq<nat> {
    ns![60u32,109,97,114,107,62].map_values(|x: u32| x as nat)
}

pub proof fn literal_130()
    ensures
        u::copy_literal_ok(u::copy_registry()[130]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l130(Seq::empty());
    p::all();
    reveal_strlit("<mark>");
    assert(n::codes("<mark>"@) =~= data_130());
    assert(n::clean(data_130(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<mark>"@);
}

pub open spec fn data_131() -> Seq<nat> {
    ns![60u32,47,109,97,114,107,62].map_values(|x: u32| x as nat)
}

pub proof fn literal_131()
    ensures
        u::copy_literal_ok(u::copy_registry()[131]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l131(Seq::empty());
    p::all();
    reveal_strlit("</mark>");
    assert(n::codes("</mark>"@) =~= data_131());
    assert(n::clean(data_131(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("</mark>"@);
}

pub open spec fn data_132() -> Seq<nat> {
    ns![60u32,100,97,116,97,108,105,115,116,32,105,100,61,34,114,101,118,105,101,119,101,114,45,110,97,109,101,115,34,62].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_132()
    ensures
        u::copy_literal_ok(u::copy_registry()[132]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l132(Seq::empty());
    p::all();
    reveal_strlit("<datalist id=\"reviewer-names\">");
    assert(n::codes("<datalist id=\"reviewer-names\">"@) =~= data_132());
    assert(n::clean(data_132(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<datalist id=\"reviewer-names\">"@);
}

pub open spec fn data_133() -> Seq<nat> {
    ns![60u32,111,112,116,105,111,110,32,118,97,108,117,101,61,34].map_values(|x: u32| x as nat)
}

pub proof fn literal_133()
    ensures
        u::copy_literal_ok(u::copy_registry()[133]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l133(Seq::empty());
    p::all();
    reveal_strlit("<option value=\"");
    assert(n::codes("<option value=\""@) =~= data_133());
    assert(n::clean(data_133(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<option value=\""@);
}

pub open spec fn data_134() -> Seq<nat> {
    ns![34u32,62,60,47,111,112,116,105,111,110,62].map_values(|x: u32| x as nat)
}

pub proof fn literal_134()
    ensures
        u::copy_literal_ok(u::copy_registry()[134]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l134(Seq::empty());
    p::all();
    reveal_strlit("\"></option>");
    assert(n::codes("\"></option>"@) =~= data_134());
    assert(n::clean(data_134(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("\"></option>"@);
}

pub open spec fn data_135() -> Seq<nat> {
    ns![60u32,47,100,97,116,97,108,105,115,116,62].map_values(|x: u32| x as nat)
}

pub proof fn literal_135()
    ensures
        u::copy_literal_ok(u::copy_registry()[135]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l135(Seq::empty());
    p::all();
    reveal_strlit("</datalist>");
    assert(n::codes("</datalist>"@) =~= data_135());
    assert(n::clean(data_135(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("</datalist>"@);
}

pub open spec fn data_136() -> Seq<nat> {
    ns![46u32,112,100,102].map_values(|x: u32| x as nat)
}

pub proof fn literal_136()
    ensures
        u::copy_literal_ok(u::copy_registry()[136]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l136(Seq::empty());
    p::all();
    reveal_strlit(".pdf");
    assert(n::codes(".pdf"@) =~= data_136());
    assert(n::clean(data_136(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge(".pdf"@);
}

pub open spec fn data_137() -> Seq<nat> {
    ns![32u32,60,97,32,99,108,97,115,115,61,34,115,111,117,114,99,101,34,32,104,114,101,102,61,34,46,46,47,115,111,117,114,99,101,47].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_137()
    ensures
        u::copy_literal_ok(u::copy_registry()[137]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l137(Seq::empty());
    p::all();
    reveal_strlit(" <a class=\"source\" href=\"../source/");
    assert(n::codes(" <a class=\"source\" href=\"../source/"@) =~= data_137());
    assert(n::clean(data_137(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge(" <a class=\"source\" href=\"../source/"@);
}

pub open spec fn data_138() -> Seq<nat> {
    ns![34u32,62,80,68,70,60,47,97,62].map_values(|x: u32| x as nat)
}

pub proof fn literal_138()
    ensures
        u::copy_literal_ok(u::copy_registry()[138]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l138(Seq::empty());
    p::all();
    reveal_strlit("\">PDF</a>");
    assert(n::codes("\">PDF</a>"@) =~= data_138());
    assert(n::clean(data_138(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("\">PDF</a>"@);
}

pub open spec fn data_139() -> Seq<nat> {
    ns![115u32,111,117,114,99,101,47].map_values(|x: u32| x as nat)
}

pub proof fn literal_139()
    ensures
        u::copy_literal_ok(u::copy_registry()[139]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l139(Seq::empty());
    p::all();
    reveal_strlit("source/");
    assert(n::codes("source/"@) =~= data_139());
    assert(n::clean(data_139(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("source/"@);
}

pub open spec fn data_140() -> Seq<nat> {
    ns![46u32,46,47,115,111,117,114,99,101,47].map_values(|x: u32| x as nat)
}

pub proof fn literal_140()
    ensures
        u::copy_literal_ok(u::copy_registry()[140]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l140(Seq::empty());
    p::all();
    reveal_strlit("../source/");
    assert(n::codes("../source/"@) =~= data_140());
    assert(n::clean(data_140(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("../source/"@);
}

pub open spec fn data_141() -> Seq<nat> {
    ns![83u32,111,117,114,99,101,32,116,101,120,116].map_values(|x: u32| x as nat)
}

pub proof fn literal_141()
    ensures
        u::copy_literal_ok(u::copy_registry()[141]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l141(Seq::empty());
    p::all();
    reveal_strlit("Source text");
    assert(n::codes("Source text"@) =~= data_141());
    assert(n::clean(data_141(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("Source text"@);
}

pub open spec fn data_142() -> Seq<nat> {
    ns![46u32,46,47,114,101,99,111,114,100,115,46,104,116,109,108].map_values(|x: u32| x as nat)
}

pub proof fn literal_142()
    ensures
        u::copy_literal_ok(u::copy_registry()[142]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l142(Seq::empty());
    p::all();
    reveal_strlit("../records.html");
    assert(n::codes("../records.html"@) =~= data_142());
    assert(n::clean(data_142(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("../records.html"@);
}

pub open spec fn data_143() -> Seq<nat> {
    ns![35u32,].map_values(|x: u32| x as nat)
}

pub proof fn literal_143()
    ensures
        u::copy_literal_ok(u::copy_registry()[143]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l143(Seq::empty());
    p::all();
    reveal_strlit("#");
    assert(n::codes("#"@) =~= data_143());
    assert(n::clean(data_143(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("#"@);
}

} // verus!
}
pub mod batch_144 {
    use crate::{
        k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
        k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
    };
    use ckc_spec::ui as u;
    use vstd::prelude::*;
    use vstd::utf8::*;
    verus! {

pub open spec fn data_144() -> Seq<nat> {
    ns![80u32,114,101,118,105,111,117,115,32,100,111,99,117,109,101,110,116].map_values(
        |x: u32| x as nat,
    )
}

pub proof fn literal_144()
    ensures
        u::copy_literal_ok(u::copy_registry()[144]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l144(Seq::empty());
    p::all();
    reveal_strlit("Previous document");
    assert(n::codes("Previous document"@) =~= data_144());
    assert(n::clean(data_144(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("Previous document"@);
}

pub open spec fn data_145() -> Seq<nat> {
    ns![60u32,97,32,104,114,101,102,61,34,46,46,47,105,110,100,101,120,46,104,116,109,108,34,62,71,117,105,100,101,108,105,110,101,32,105,110,100,101,120,60,47,97,62].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_145()
    ensures
        u::copy_literal_ok(u::copy_registry()[145]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l145(Seq::empty());
    p::all();
    reveal_strlit("<a href=\"../index.html\">Guideline index</a>");
    assert(n::codes("<a href=\"../index.html\">Guideline index</a>"@) =~= data_145());
    assert(n::clean(data_145(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<a href=\"../index.html\">Guideline index</a>"@);
}

pub open spec fn data_146() -> Seq<nat> {
    ns![78u32,101,120,116,32,100,111,99,117,109,101,110,116].map_values(|x: u32| x as nat)
}

pub proof fn literal_146()
    ensures
        u::copy_literal_ok(u::copy_registry()[146]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l146(Seq::empty());
    p::all();
    reveal_strlit("Next document");
    assert(n::codes("Next document"@) =~= data_146());
    assert(n::clean(data_146(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("Next document"@);
}

pub open spec fn data_147() -> Seq<nat> {
    ns![65u32,108,108,32,100,101,99,105,115,105,111,110,32,114,101,99,111,114,100,115].map_values(
        |x: u32| x as nat,
    )
}

pub proof fn literal_147()
    ensures
        u::copy_literal_ok(u::copy_registry()[147]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l147(Seq::empty());
    p::all();
    reveal_strlit("All decision records");
    assert(n::codes("All decision records"@) =~= data_147());
    assert(n::clean(data_147(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("All decision records"@);
}

pub open spec fn data_148() -> Seq<nat> {
    ns![60u32,115,101,99,116,105,111,110,32,99,108,97,115,115,61,34,115,116,97,108,101,34,62].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_148()
    ensures
        u::copy_literal_ok(u::copy_registry()[148]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l148(Seq::empty());
    p::all();
    reveal_strlit("<section class=\"stale\">");
    assert(n::codes("<section class=\"stale\">"@) =~= data_148());
    assert(n::clean(data_148(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<section class=\"stale\">"@);
}

pub open spec fn data_149() -> Seq<nat> {
    ns![60u32,112,62,84,104,101,32,100,111,99,117,109,101,110,116,32,111,114,32,105,116,115,32,115,111,117,114,99,101,32,99,104,97,110,103,101,100,32,97,102,116,101,114,32,116,104,101,32,108,97,115,116,32,100,101,99,105,115,105,111,110,46,32,78,111,32,114,101,99,111,114,100,101,100,32,100,101,99,105,115,105,111,110,32,97,112,112,108,105,101,115,32,116,111,32,116,104,101,32,118,101,114,115,105,111,110,32,115,104,111,119,110,32,104,101,114,101,46,60,47,112,62].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_149()
    ensures
        u::copy_literal_ok(u::copy_registry()[149]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l149(Seq::empty());
    p::all();
    reveal_strlit(
        "<p>The document or its source changed after the last decision. No recorded decision applies to the version shown here.</p>",
    );
    assert(n::codes(
        "<p>The document or its source changed after the last decision. No recorded decision applies to the version shown here.</p>"@,
    ) =~= data_149());
    assert(n::clean(data_149(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge(
        "<p>The document or its source changed after the last decision. No recorded decision applies to the version shown here.</p>"@,
    );
}

pub open spec fn data_150() -> Seq<nat> {
    ns![60u32,112,32,99,108,97,115,115,61,34,104,108,45,110,111,116,101,34,62,60,108,97,98,101,108,62,60,105,110,112,117,116,32,116,121,112,101,61,34,99,104,101,99,107,98,111,120,34,32,99,108,97,115,115,61,34,104,108,45,116,111,103,103,108,101,34,32,99,104,101,99,107,101,100,62,32,72,105,103,104,108,105,103,104,116,105,110,103,60,47,108,97,98,101,108,62,32,84,114,121,32,104,111,118,101,114,105,110,103,32,97,110,100,32,99,108,105,99,107,105,110,103,32,111,110,32,104,105,103,104,108,105,103,104,116,101,100,32,116,101,114,109,115,32,102,111,114,32,100,105,102,102,101,114,101,110,116,32,108,101,118,101,108,115,32,111,102,32,101,109,112,104,97,115,105,115,46,60,47,112,62].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_150()
    ensures
        u::copy_literal_ok(u::copy_registry()[150]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l150(Seq::empty());
    p::all();
    reveal_strlit(
        "<p class=\"hl-note\"><label><input type=\"checkbox\" class=\"hl-toggle\" checked> Highlighting</label> Try hovering and clicking on highlighted terms for different levels of emphasis.</p>",
    );
    assert(n::codes(
        "<p class=\"hl-note\"><label><input type=\"checkbox\" class=\"hl-toggle\" checked> Highlighting</label> Try hovering and clicking on highlighted terms for different levels of emphasis.</p>"@,
    ) =~= data_150());
    assert(n::clean(data_150(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge(
        "<p class=\"hl-note\"><label><input type=\"checkbox\" class=\"hl-toggle\" checked> Highlighting</label> Try hovering and clicking on highlighted terms for different levels of emphasis.</p>"@,
    );
}

pub open spec fn data_151() -> Seq<nat> {
    ns![60u32,104,51,62,79,114,105,103,105,110,97,108,32,112,97,115,115,97,103,101,60,47,104,51,62].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_151()
    ensures
        u::copy_literal_ok(u::copy_registry()[151]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l151(Seq::empty());
    p::all();
    reveal_strlit("<h3>Original passage</h3>");
    assert(n::codes("<h3>Original passage</h3>"@) =~= data_151());
    assert(n::clean(data_151(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<h3>Original passage</h3>"@);
}

pub open spec fn data_152() -> Seq<nat> {
    ns![60u32,112,114,101,32,99,108,97,115,115,61,34,112,114,111,115,101,34,62].map_values(
        |x: u32| x as nat,
    )
}

pub proof fn literal_152()
    ensures
        u::copy_literal_ok(u::copy_registry()[152]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l152(Seq::empty());
    p::all();
    reveal_strlit("<pre class=\"prose\">");
    assert(n::codes("<pre class=\"prose\">"@) =~= data_152());
    assert(n::clean(data_152(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<pre class=\"prose\">"@);
}

pub open spec fn data_153() -> Seq<nat> {
    ns![60u32,47,112,114,101,62].map_values(|x: u32| x as nat)
}

pub proof fn literal_153()
    ensures
        u::copy_literal_ok(u::copy_registry()[153]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l153(Seq::empty());
    p::all();
    reveal_strlit("</pre>");
    assert(n::codes("</pre>"@) =~= data_153());
    assert(n::clean(data_153(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("</pre>"@);
}

pub open spec fn data_154() -> Seq<nat> {
    ns![60u32,104,51,62,65,116,116,101,109,112,116,111,32,67,111,110,116,114,111,108,108,101,100,32,69,110,103,108,105,115,104,32,40,65,67,69,41,60,47,104,51,62].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_154()
    ensures
        u::copy_literal_ok(u::copy_registry()[154]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l154(Seq::empty());
    p::all();
    reveal_strlit("<h3>Attempto Controlled English (ACE)</h3>");
    assert(n::codes("<h3>Attempto Controlled English (ACE)</h3>"@) =~= data_154());
    assert(n::clean(data_154(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<h3>Attempto Controlled English (ACE)</h3>"@);
}

pub open spec fn data_155() -> Seq<nat> {
    ns![60u32,115,101,99,116,105,111,110,32,99,108,97,115,115,61,34,118,101,114,100,105,99,116,45,101,110,116,114,121,34,62].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_155()
    ensures
        u::copy_literal_ok(u::copy_registry()[155]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l155(Seq::empty());
    p::all();
    reveal_strlit("<section class=\"verdict-entry\">");
    assert(n::codes("<section class=\"verdict-entry\">"@) =~= data_155());
    assert(n::clean(data_155(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<section class=\"verdict-entry\">"@);
}

pub open spec fn data_156() -> Seq<nat> {
    ns![60u32,104,51,62,82,101,99,111,114,100,32,97,32,100,101,99,105,115,105,111,110,60,47,104,51,62].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_156()
    ensures
        u::copy_literal_ok(u::copy_registry()[156]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l156(Seq::empty());
    p::all();
    reveal_strlit("<h3>Record a decision</h3>");
    assert(n::codes("<h3>Record a decision</h3>"@) =~= data_156());
    assert(n::clean(data_156(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<h3>Record a decision</h3>"@);
}

pub open spec fn data_157() -> Seq<nat> {
    ns![60u32,112,62,68,111,101,115,32,116,104,101,32,65,67,69,32,114,101,112,114,101,115,101,110,116,97,116,105,111,110,32,97,112,112,114,111,112,114,105,97,116,101,108,121,32,114,101,102,108,101,99,116,32,116,104,101,32,111,114,105,103,105,110,97,108,32,112,97,115,115,97,103,101,63,60,47,112,62].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_157()
    ensures
        u::copy_literal_ok(u::copy_registry()[157]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l157(Seq::empty());
    p::all();
    reveal_strlit("<p>Does the ACE representation appropriately reflect the original passage?</p>");
    assert(n::codes(
        "<p>Does the ACE representation appropriately reflect the original passage?</p>"@,
    ) =~= data_157());
    assert(n::clean(data_157(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge(
        "<p>Does the ACE representation appropriately reflect the original passage?</p>"@,
    );
}

pub open spec fn data_158() -> Seq<nat> {
    ns![60u32,102,111,114,109,32,109,101,116,104,111,100,61,34,112,111,115,116,34,62].map_values(
        |x: u32| x as nat,
    )
}

pub proof fn literal_158()
    ensures
        u::copy_literal_ok(u::copy_registry()[158]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l158(Seq::empty());
    p::all();
    reveal_strlit("<form method=\"post\">");
    assert(n::codes("<form method=\"post\">"@) =~= data_158());
    assert(n::clean(data_158(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<form method=\"post\">"@);
}

pub open spec fn data_159() -> Seq<nat> {
    ns![60u32,102,105,101,108,100,115,101,116,62].map_values(|x: u32| x as nat)
}

pub proof fn literal_159()
    ensures
        u::copy_literal_ok(u::copy_registry()[159]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l159(Seq::empty());
    p::all();
    reveal_strlit("<fieldset>");
    assert(n::codes("<fieldset>"@) =~= data_159());
    assert(n::clean(data_159(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<fieldset>"@);
}

} // verus!
}
pub mod batch_160 {
    use crate::{
        k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
        k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
    };
    use ckc_spec::ui as u;
    use vstd::prelude::*;
    use vstd::utf8::*;
    verus! {

pub open spec fn data_160() -> Seq<nat> {
    ns![60u32,108,101,103,101,110,100,62,68,101,99,105,115,105,111,110,60,47,108,101,103,101,110,100,62].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_160()
    ensures
        u::copy_literal_ok(u::copy_registry()[160]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l160(Seq::empty());
    p::all();
    reveal_strlit("<legend>Decision</legend>");
    assert(n::codes("<legend>Decision</legend>"@) =~= data_160());
    assert(n::clean(data_160(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<legend>Decision</legend>"@);
}

pub open spec fn data_161() -> Seq<nat> {
    ns![60u32,108,97,98,101,108,62,60,105,110,112,117,116,32,116,121,112,101,61,34,114,97,100,105,111,34,32,110,97,109,101,61,34,118,101,114,100,105,99,116,34,32,118,97,108,117,101,61,34,97,112,112,114,111,118,101,100,34,32,114,101,113,117,105,114,101,100,62,32,65,112,112,114,111,118,101,100,60,47,108,97,98,101,108,62].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_161()
    ensures
        u::copy_literal_ok(u::copy_registry()[161]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l161(Seq::empty());
    p::all();
    reveal_strlit(
        "<label><input type=\"radio\" name=\"verdict\" value=\"approved\" required> Approved</label>",
    );
    assert(n::codes(
        "<label><input type=\"radio\" name=\"verdict\" value=\"approved\" required> Approved</label>"@,
    ) =~= data_161());
    assert(n::clean(data_161(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge(
        "<label><input type=\"radio\" name=\"verdict\" value=\"approved\" required> Approved</label>"@,
    );
}

pub open spec fn data_162() -> Seq<nat> {
    ns![60u32,108,97,98,101,108,62,60,105,110,112,117,116,32,116,121,112,101,61,34,114,97,100,105,111,34,32,110,97,109,101,61,34,118,101,114,100,105,99,116,34,32,118,97,108,117,101,61,34,114,101,106,101,99,116,101,100,34,32,114,101,113,117,105,114,101,100,62,32,82,101,106,101,99,116,101,100,60,47,108,97,98,101,108,62].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_162()
    ensures
        u::copy_literal_ok(u::copy_registry()[162]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l162(Seq::empty());
    p::all();
    reveal_strlit(
        "<label><input type=\"radio\" name=\"verdict\" value=\"rejected\" required> Rejected</label>",
    );
    assert(n::codes(
        "<label><input type=\"radio\" name=\"verdict\" value=\"rejected\" required> Rejected</label>"@,
    ) =~= data_162());
    assert(n::clean(data_162(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge(
        "<label><input type=\"radio\" name=\"verdict\" value=\"rejected\" required> Rejected</label>"@,
    );
}

pub open spec fn data_163() -> Seq<nat> {
    ns![60u32,47,102,105,101,108,100,115,101,116,62].map_values(|x: u32| x as nat)
}

pub proof fn literal_163()
    ensures
        u::copy_literal_ok(u::copy_registry()[163]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l163(Seq::empty());
    p::all();
    reveal_strlit("</fieldset>");
    assert(n::codes("</fieldset>"@) =~= data_163());
    assert(n::clean(data_163(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("</fieldset>"@);
}

pub open spec fn data_164() -> Seq<nat> {
    ns![60u32,108,97,98,101,108,32,102,111,114,61,34,114,101,118,105,101,119,101,114,34,62,82,101,118,105,101,119,101,114,32,110,97,109,101,60,47,108,97,98,101,108,62].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_164()
    ensures
        u::copy_literal_ok(u::copy_registry()[164]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l164(Seq::empty());
    p::all();
    reveal_strlit("<label for=\"reviewer\">Reviewer name</label>");
    assert(n::codes("<label for=\"reviewer\">Reviewer name</label>"@) =~= data_164());
    assert(n::clean(data_164(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<label for=\"reviewer\">Reviewer name</label>"@);
}

pub open spec fn data_165() -> Seq<nat> {
    ns![60u32,105,110,112,117,116,32,116,121,112,101,61,34,116,101,120,116,34,32,105,100,61,34,114,101,118,105,101,119,101,114,34,32,110,97,109,101,61,34,114,101,118,105,101,119,101,114,34,32,108,105,115,116,61,34,114,101,118,105,101,119,101,114,45,110,97,109,101,115,34,32,118,97,108,117,101,61,34].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_165()
    ensures
        u::copy_literal_ok(u::copy_registry()[165]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l165(Seq::empty());
    p::all();
    reveal_strlit(
        "<input type=\"text\" id=\"reviewer\" name=\"reviewer\" list=\"reviewer-names\" value=\"",
    );
    assert(n::codes(
        "<input type=\"text\" id=\"reviewer\" name=\"reviewer\" list=\"reviewer-names\" value=\""@,
    ) =~= data_165());
    assert(n::clean(data_165(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge(
        "<input type=\"text\" id=\"reviewer\" name=\"reviewer\" list=\"reviewer-names\" value=\""@,
    );
}

pub open spec fn data_166() -> Seq<nat> {
    ns![34u32,32,114,101,113,117,105,114,101,100,62].map_values(|x: u32| x as nat)
}

pub proof fn literal_166()
    ensures
        u::copy_literal_ok(u::copy_registry()[166]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l166(Seq::empty());
    p::all();
    reveal_strlit("\" required>");
    assert(n::codes("\" required>"@) =~= data_166());
    assert(n::clean(data_166(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("\" required>"@);
}

pub open spec fn data_167() -> Seq<nat> {
    ns![60u32,108,97,98,101,108,32,102,111,114,61,34,99,111,109,109,101,110,116,34,62,67,111,109,109,101,110,116,32,40,111,112,116,105,111,110,97,108,41,60,47,108,97,98,101,108,62].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_167()
    ensures
        u::copy_literal_ok(u::copy_registry()[167]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l167(Seq::empty());
    p::all();
    reveal_strlit("<label for=\"comment\">Comment (optional)</label>");
    assert(n::codes("<label for=\"comment\">Comment (optional)</label>"@) =~= data_167());
    assert(n::clean(data_167(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<label for=\"comment\">Comment (optional)</label>"@);
}

pub open spec fn data_168() -> Seq<nat> {
    ns![60u32,116,101,120,116,97,114,101,97,32,105,100,61,34,99,111,109,109,101,110,116,34,32,110,97,109,101,61,34,99,111,109,109,101,110,116,34,62,60,47,116,101,120,116,97,114,101,97,62].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_168()
    ensures
        u::copy_literal_ok(u::copy_registry()[168]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l168(Seq::empty());
    p::all();
    reveal_strlit("<textarea id=\"comment\" name=\"comment\"></textarea>");
    assert(n::codes("<textarea id=\"comment\" name=\"comment\"></textarea>"@) =~= data_168());
    assert(n::clean(data_168(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<textarea id=\"comment\" name=\"comment\"></textarea>"@);
}

pub open spec fn data_169() -> Seq<nat> {
    ns![60u32,105,110,112,117,116,32,116,121,112,101,61,34,104,105,100,100,101,110,34,32,110,97,109,101,61,34,114,101,118,105,101,119,95,115,104,97,50,53,54,34,32,118,97,108,117,101,61,34].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_169()
    ensures
        u::copy_literal_ok(u::copy_registry()[169]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l169(Seq::empty());
    p::all();
    reveal_strlit("<input type=\"hidden\" name=\"review_sha256\" value=\"");
    assert(n::codes("<input type=\"hidden\" name=\"review_sha256\" value=\""@) =~= data_169());
    assert(n::clean(data_169(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<input type=\"hidden\" name=\"review_sha256\" value=\""@);
}

pub open spec fn data_170() -> Seq<nat> {
    ns![60u32,105,110,112,117,116,32,116,121,112,101,61,34,104,105,100,100,101,110,34,32,110,97,109,101,61,34,108,101,100,103,101,114,95,115,104,97,50,53,54,34,32,118,97,108,117,101,61,34].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_170()
    ensures
        u::copy_literal_ok(u::copy_registry()[170]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l170(Seq::empty());
    p::all();
    reveal_strlit("<input type=\"hidden\" name=\"ledger_sha256\" value=\"");
    assert(n::codes("<input type=\"hidden\" name=\"ledger_sha256\" value=\""@) =~= data_170());
    assert(n::clean(data_170(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<input type=\"hidden\" name=\"ledger_sha256\" value=\""@);
}

pub open spec fn data_171() -> Seq<nat> {
    ns![60u32,105,110,112,117,116,32,116,121,112,101,61,34,104,105,100,100,101,110,34,32,110,97,109,101,61,34,99,115,114,102,34,32,118,97,108,117,101,61,34].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_171()
    ensures
        u::copy_literal_ok(u::copy_registry()[171]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l171(Seq::empty());
    p::all();
    reveal_strlit("<input type=\"hidden\" name=\"csrf\" value=\"");
    assert(n::codes("<input type=\"hidden\" name=\"csrf\" value=\""@) =~= data_171());
    assert(n::clean(data_171(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<input type=\"hidden\" name=\"csrf\" value=\""@);
}

pub open spec fn data_172() -> Seq<nat> {
    ns![60u32,98,117,116,116,111,110,62,82,101,99,111,114,100,32,100,101,99,105,115,105,111,110,60,47,98,117,116,116,111,110,62].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_172()
    ensures
        u::copy_literal_ok(u::copy_registry()[172]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l172(Seq::empty());
    p::all();
    reveal_strlit("<button>Record decision</button>");
    assert(n::codes("<button>Record decision</button>"@) =~= data_172());
    assert(n::clean(data_172(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<button>Record decision</button>"@);
}

pub open spec fn data_173() -> Seq<nat> {
    ns![60u32,47,102,111,114,109,62].map_values(|x: u32| x as nat)
}

pub proof fn literal_173()
    ensures
        u::copy_literal_ok(u::copy_registry()[173]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l173(Seq::empty());
    p::all();
    reveal_strlit("</form>");
    assert(n::codes("</form>"@) =~= data_173());
    assert(n::clean(data_173(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("</form>"@);
}

pub open spec fn data_174() -> Seq<nat> {
    ns![60u32,100,101,116,97,105,108,115,62].map_values(|x: u32| x as nat)
}

pub proof fn literal_174()
    ensures
        u::copy_literal_ok(u::copy_registry()[174]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l174(Seq::empty());
    p::all();
    reveal_strlit("<details>");
    assert(n::codes("<details>"@) =~= data_174());
    assert(n::clean(data_174(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<details>"@);
}

pub open spec fn data_175() -> Seq<nat> {
    ns![60u32,115,117,109,109,97,114,121,62,67,111,109,112,105,108,101,100,32,80,114,111,108,111,103,32,40].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_175()
    ensures
        u::copy_literal_ok(u::copy_registry()[175]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l175(Seq::empty());
    p::all();
    reveal_strlit("<summary>Compiled Prolog (");
    assert(n::codes("<summary>Compiled Prolog ("@) =~= data_175());
    assert(n::clean(data_175(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<summary>Compiled Prolog ("@);
}

} // verus!
}
pub mod batch_176 {
    use crate::{
        k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
        k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
    };
    use ckc_spec::ui as u;
    use vstd::prelude::*;
    use vstd::utf8::*;
    verus! {

pub open spec fn data_176() -> Seq<nat> {
    ns![32u32,108,105,110,101,115,41,60,47,115,117,109,109,97,114,121,62].map_values(
        |x: u32| x as nat,
    )
}

pub proof fn literal_176()
    ensures
        u::copy_literal_ok(u::copy_registry()[176]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l176(Seq::empty());
    p::all();
    reveal_strlit(" lines)</summary>");
    assert(n::codes(" lines)</summary>"@) =~= data_176());
    assert(n::clean(data_176(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge(" lines)</summary>"@);
}

pub open spec fn data_177() -> Seq<nat> {
    ns![60u32,112,114,101,62].map_values(|x: u32| x as nat)
}

pub proof fn literal_177()
    ensures
        u::copy_literal_ok(u::copy_registry()[177]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l177(Seq::empty());
    p::all();
    reveal_strlit("<pre>");
    assert(n::codes("<pre>"@) =~= data_177());
    assert(n::clean(data_177(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<pre>"@);
}

pub open spec fn data_178() -> Seq<nat> {
    ns![60u32,47,100,101,116,97,105,108,115,62].map_values(|x: u32| x as nat)
}

pub proof fn literal_178()
    ensures
        u::copy_literal_ok(u::copy_registry()[178]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l178(Seq::empty());
    p::all();
    reveal_strlit("</details>");
    assert(n::codes("</details>"@) =~= data_178());
    assert(n::clean(data_178(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("</details>"@);
}

pub open spec fn data_179() -> Seq<nat> {
    ns![60u32,110,97,118,32,99,108,97,115,115,61,34,100,111,99,110,97,118,34,62].map_values(
        |x: u32| x as nat,
    )
}

pub proof fn literal_179()
    ensures
        u::copy_literal_ok(u::copy_registry()[179]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l179(Seq::empty());
    p::all();
    reveal_strlit("<nav class=\"docnav\">");
    assert(n::codes("<nav class=\"docnav\">"@) =~= data_179());
    assert(n::clean(data_179(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<nav class=\"docnav\">"@);
}

pub open spec fn data_180() -> Seq<nat> {
    ns![60u32,97,32,104,114,101,102,61,34,46,46,47,46,46,47,46,46,47,105,110,100,101,120,46,104,116,109,108,34,62,103,117,105,100,101,108,105,110,101,115,60,47,97,62,32,47,32,60,97,32,104,114,101,102,61,34,46,46,47,105,110,100,101,120,46,104,116,109,108,34,62].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_180()
    ensures
        u::copy_literal_ok(u::copy_registry()[180]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l180(Seq::empty());
    p::all();
    reveal_strlit("<a href=\"../../../index.html\">guidelines</a> / <a href=\"../index.html\">");
    assert(n::codes("<a href=\"../../../index.html\">guidelines</a> / <a href=\"../index.html\">"@)
        =~= data_180());
    assert(n::clean(data_180(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<a href=\"../../../index.html\">guidelines</a> / <a href=\"../index.html\">"@);
}

pub open spec fn data_181() -> Seq<nat> {
    ns![60u32,47,97,62,32,47,32].map_values(|x: u32| x as nat)
}

pub proof fn literal_181()
    ensures
        u::copy_literal_ok(u::copy_registry()[181]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l181(Seq::empty());
    p::all();
    reveal_strlit("</a> / ");
    assert(n::codes("</a> / "@) =~= data_181());
    assert(n::clean(data_181(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("</a> / "@);
}

pub open spec fn data_182() -> Seq<nat> {
    ns![97u32,32,97,110,32,116,104,101,32,101,118,101,114,121,32,101,97,99,104,32,110,111,32,97,108,108,32,115,111,109,101,32,97,110,121,32,116,104,105,115,32,116,104,97,116,32,116,104,101,115,101,32,116,104,111,115,101,32,115,117,99,104,32,105,115,32,97,114,101,32,119,97,115,32,119,101,114,101,32,98,101,32,98,101,101,110,32,98,101,105,110,103,32,104,97,115,32,104,97,118,101,32,104,97,100,32,100,111,101,115,32,100,111,32,100,105,100,32,115,104,111,117,108,100,32,109,117,115,116,32,109,97,121,32,99,97,110,32,99,97,110,110,111,116,32,109,105,103,104,116,32,119,105,108,108,32,119,111,117,108,100,32,115,104,97,108,108,32,99,111,117,108,100,32,105,102,32,116,104,101,110,32,97,110,100,32,111,114,32,110,111,114,32,98,117,116,32,110,111,116,32,105,116,32,105,116,115,32,105,116,115,101,108,102,32,116,104,101,121,32,116,104,101,109,32,116,104,101,105,114,32,104,101,32,115,104,101,32,119,104,111,32,119,104,111,109,32,119,104,111,115,101,32,119,104,105,99,104,32,119,104,97,116,32,119,104,101,114,101,32,119,104,101,110,32,116,104,101,114,101,32,115,111,109,101,116,104,105,110,103,32,115,111,109,101,98,111,100,121,32,115,111,109,101,111,110,101,32,101,118,101,114,121,116,104,105,110,103,32,101,118,101,114,121,98,111,100,121,32,101,118,101,114,121,111,110,101,32,110,111,116,104,105,110,103,32,110,111,98,111,100,121,32,111,102,32,102,111,114,32,119,105,116,104,32,119,105,116,104,111,117,116,32,100,117,114,105,110,103,32,116,111,32,97,116,32,105,110,32,111,110,32,98,121,32,102,114,111,109,32,97,115,32,97,103,97,105,110,115,116,32,97,98,111,117,116,32,97,102,116,101,114,32,98,101,102,111,114,101,32,116,104,114,111,117,103,104,32,117,110,100,101,114,32,111,118,101,114,32,97,98,111,118,101,32,98,101,108,111,119,32,105,110,116,111,32,111,110,116,111,32,112,101,114,32,119,105,116,104,105,110,32,98,101,116,119,101,101,110,32,97,109,111,110,103,32,97,114,111,117,110,100,32,110,101,97,114,32,116,104,97,110,32,108,101,97,115,116,32,109,111,115,116,32,109,111,114,101,32,108,101,115,115,32,102,101,119,101,114,32,103,114,101,97,116,101,114].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_182()
    ensures
        u::copy_literal_ok(u::copy_registry()[182]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l182(Seq::empty());
    p::all();
    reveal_strlit(
        "a an the every each no all some any this that these those such is are was were be been being has have had does do did should must may can cannot might will would shall could if then and or nor but not it its itself they them their he she who whom whose which what where when there something somebody someone everything everybody everyone nothing nobody of for with without during to at in on by from as against about after before through under over above below into onto per within between among around near than least most more less fewer greater",
    );
    assert(n::codes(
        "a an the every each no all some any this that these those such is are was were be been being has have had does do did should must may can cannot might will would shall could if then and or nor but not it its itself they them their he she who whom whose which what where when there something somebody someone everything everybody everyone nothing nobody of for with without during to at in on by from as against about after before through under over above below into onto per within between among around near than least most more less fewer greater"@,
    ) =~= data_182());
    assert(n::clean(data_182(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge(
        "a an the every each no all some any this that these those such is are was were be been being has have had does do did should must may can cannot might will would shall could if then and or nor but not it its itself they them their he she who whom whose which what where when there something somebody someone everything everybody everyone nothing nobody of for with without during to at in on by from as against about after before through under over above below into onto per within between among around near than least most more less fewer greater"@,
    );
}

pub open spec fn data_183() -> Seq<nat> {
    ns![60u32,47,112,62,10,60,33,45,45,32].map_values(|x: u32| x as nat)
}

pub proof fn literal_183()
    ensures
        u::copy_literal_ok(u::copy_registry()[183]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l183(Seq::empty());
    p::all();
    reveal_strlit("</p>\n<!-- ");
    assert(n::codes("</p>\n<!-- "@) =~= data_183());
    assert(n::clean(data_183(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("</p>\n<!-- "@);
}

pub open spec fn data_184() -> Seq<nat> {
    ns![32u32,45,45,62].map_values(|x: u32| x as nat)
}

pub proof fn literal_184()
    ensures
        u::copy_literal_ok(u::copy_registry()[184]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l184(Seq::empty());
    p::all();
    reveal_strlit(" -->");
    assert(n::codes(" -->"@) =~= data_184());
    assert(n::clean(data_184(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge(" -->"@);
}

pub open spec fn data_185() -> Seq<nat> {
    ns![84u32,104,101,32,114,101,113,117,101,115,116,32,119,97,115,32,114,101,102,117,115,101,100,46,32,79,112,101,110,32,116,104,101,32,100,111,99,117,109,101,110,116,32,112,97,103,101,32,97,103,97,105,110,32,102,114,111,109,32,116,104,105,115,32,115,105,116,101,32,97,110,100,32,115,117,98,109,105,116,32,116,104,101,32,100,101,99,105,115,105,111,110,32,97,103,97,105,110,46].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_185()
    ensures
        u::copy_literal_ok(u::copy_registry()[185]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l185(Seq::empty());
    p::all();
    reveal_strlit(
        "The request was refused. Open the document page again from this site and submit the decision again.",
    );
    assert(n::codes(
        "The request was refused. Open the document page again from this site and submit the decision again."@,
    ) =~= data_185());
    assert(n::clean(data_185(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge(
        "The request was refused. Open the document page again from this site and submit the decision again."@,
    );
}

pub open spec fn data_186() -> Seq<nat> {
    ns![84u32,104,101,32,115,117,98,109,105,116,116,101,100,32,102,111,114,109,32,119,97,115,32,110,111,116,32,118,97,108,105,100,46,32,71,111,32,98,97,99,107,32,116,111,32,116,104,101,32,100,111,99,117,109,101,110,116,32,112,97,103,101,44,32,114,101,108,111,97,100,32,105,116,44,32,97,110,100,32,115,117,98,109,105,116,32,116,104,101,32,100,101,99,105,115,105,111,110,32,97,103,97,105,110,46].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_186()
    ensures
        u::copy_literal_ok(u::copy_registry()[186]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l186(Seq::empty());
    p::all();
    reveal_strlit(
        "The submitted form was not valid. Go back to the document page, reload it, and submit the decision again.",
    );
    assert(n::codes(
        "The submitted form was not valid. Go back to the document page, reload it, and submit the decision again."@,
    ) =~= data_186());
    assert(n::clean(data_186(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge(
        "The submitted form was not valid. Go back to the document page, reload it, and submit the decision again."@,
    );
}

pub open spec fn data_187() -> Seq<nat> {
    ns![70u32,111,114,98,105,100,100,101,110].map_values(|x: u32| x as nat)
}

pub proof fn literal_187()
    ensures
        u::copy_literal_ok(u::copy_registry()[187]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l187(Seq::empty());
    p::all();
    reveal_strlit("Forbidden");
    assert(n::codes("Forbidden"@) =~= data_187());
    assert(n::clean(data_187(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("Forbidden"@);
}

pub open spec fn data_188() -> Seq<nat> {
    ns![66u32,97,100,32,114,101,113,117,101,115,116].map_values(|x: u32| x as nat)
}

pub proof fn literal_188()
    ensures
        u::copy_literal_ok(u::copy_registry()[188]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l188(Seq::empty());
    p::all();
    reveal_strlit("Bad request");
    assert(n::codes("Bad request"@) =~= data_188());
    assert(n::clean(data_188(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("Bad request"@);
}

pub open spec fn data_189() -> Seq<nat> {
    ns![83u32,101,114,118,101,114,32,101,114,114,111,114].map_values(|x: u32| x as nat)
}

pub proof fn literal_189()
    ensures
        u::copy_literal_ok(u::copy_registry()[189]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l189(Seq::empty());
    p::all();
    reveal_strlit("Server error");
    assert(n::codes("Server error"@) =~= data_189());
    assert(n::clean(data_189(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("Server error"@);
}

pub open spec fn data_190() -> Seq<nat> {
    ns![84u32,104,101,32,115,101,114,118,101,114,32,99,111,117,108,100,32,110,111,116,32,99,111,109,112,108,101,116,101,32,116,104,101,32,114,101,113,117,101,115,116,46,32,82,101,108,111,97,100,32,116,104,101,32,112,97,103,101,32,97,110,100,32,116,114,121,32,97,103,97,105,110,46].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_190()
    ensures
        u::copy_literal_ok(u::copy_registry()[190]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l190(Seq::empty());
    p::all();
    reveal_strlit("The server could not complete the request. Reload the page and try again.");
    assert(n::codes("The server could not complete the request. Reload the page and try again."@)
        =~= data_190());
    assert(n::clean(data_190(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("The server could not complete the request. Reload the page and try again."@);
}

pub open spec fn data_191() -> Seq<nat> {
    ns![67u32,111,110,102,108,105,99,116].map_values(|x: u32| x as nat)
}

pub proof fn literal_191()
    ensures
        u::copy_literal_ok(u::copy_registry()[191]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l191(Seq::empty());
    p::all();
    reveal_strlit("Conflict");
    assert(n::codes("Conflict"@) =~= data_191());
    assert(n::clean(data_191(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("Conflict"@);
}

} // verus!
}
pub mod batch_192 {
    use crate::{
        k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
        k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
    };
    use ckc_spec::ui as u;
    use vstd::prelude::*;
    use vstd::utf8::*;
    verus! {

pub open spec fn data_192() -> Seq<nat> {
    ns![117u32,105,58,32,118,101,114,100,105,99,116,58,32,108,101,100,103,101,114,32,99,104,97,110,103,101,100].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_192()
    ensures
        u::copy_literal_ok(u::copy_registry()[192]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l192(Seq::empty());
    p::all();
    reveal_strlit("ui: verdict: ledger changed");
    assert(n::codes("ui: verdict: ledger changed"@) =~= data_192());
    assert(n::clean(data_192(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("ui: verdict: ledger changed"@);
}

pub open spec fn data_193() -> Seq<nat> {
    ns![65u32,110,111,116,104,101,114,32,100,101,99,105,115,105,111,110,32,119,97,115,32,114,101,99,111,114,100,101,100,32,102,111,114,32,116,104,105,115,32,103,117,105,100,101,108,105,110,101,32,98,101,102,111,114,101,32,116,104,105,115,32,111,110,101,46,32,84,104,101,32,100,101,99,105,115,105,111,110,32,119,97,115,32,110,111,116,32,114,101,99,111,114,100,101,100,46,32,79,112,101,110,32,116,104,101,32,100,111,99,117,109,101,110,116,32,112,97,103,101,32,97,103,97,105,110,32,97,110,100,32,99,104,101,99,107,32,116,104,101,32,99,117,114,114,101,110,116,32,115,116,97,116,101,46].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_193()
    ensures
        u::copy_literal_ok(u::copy_registry()[193]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l193(Seq::empty());
    p::all();
    reveal_strlit(
        "Another decision was recorded for this guideline before this one. The decision was not recorded. Open the document page again and check the current state.",
    );
    assert(n::codes(
        "Another decision was recorded for this guideline before this one. The decision was not recorded. Open the document page again and check the current state."@,
    ) =~= data_193());
    assert(n::clean(data_193(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge(
        "Another decision was recorded for this guideline before this one. The decision was not recorded. Open the document page again and check the current state."@,
    );
}

pub open spec fn data_194() -> Seq<nat> {
    ns![47u32,103,47].map_values(|x: u32| x as nat)
}

pub proof fn literal_194()
    ensures
        u::copy_literal_ok(u::copy_registry()[194]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l194(Seq::empty());
    p::all();
    reveal_strlit("/g/");
    assert(n::codes("/g/"@) =~= data_194());
    assert(n::clean(data_194(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("/g/"@);
}

pub open spec fn data_195() -> Seq<nat> {
    ns![100u32,111,99].map_values(|x: u32| x as nat)
}

pub proof fn literal_195()
    ensures
        u::copy_literal_ok(u::copy_registry()[195]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l195(Seq::empty());
    p::all();
    reveal_strlit("doc");
    assert(n::codes("doc"@) =~= data_195());
    assert(n::clean(data_195(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("doc"@);
}

pub open spec fn data_196() -> Seq<nat> {
    ns![77u32,101,116,104,111,100,32,110,111,116,32,97,108,108,111,119,101,100].map_values(
        |x: u32| x as nat,
    )
}

pub proof fn literal_196()
    ensures
        u::copy_literal_ok(u::copy_registry()[196]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l196(Seq::empty());
    p::all();
    reveal_strlit("Method not allowed");
    assert(n::codes("Method not allowed"@) =~= data_196());
    assert(n::clean(data_196(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("Method not allowed"@);
}

pub open spec fn data_197() -> Seq<nat> {
    ns![60u32,112,62,79,110,108,121,32,71,69,84,32,97,110,100,32,80,79,83,84,32,97,114,101,32,115,117,112,112,111,114,116,101,100,32,111,110,32,116,104,105,115,32,112,97,103,101,46,60,47,112,62].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_197()
    ensures
        u::copy_literal_ok(u::copy_registry()[197]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l197(Seq::empty());
    p::all();
    reveal_strlit("<p>Only GET and POST are supported on this page.</p>");
    assert(n::codes("<p>Only GET and POST are supported on this page.</p>"@) =~= data_197());
    assert(n::clean(data_197(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<p>Only GET and POST are supported on this page.</p>"@);
}

pub open spec fn data_198() -> Seq<nat> {
    ns![60u32,112,62,79,110,108,121,32,71,69,84,32,105,115,32,115,117,112,112,111,114,116,101,100,32,111,110,32,116,104,105,115,32,112,97,103,101,46,60,47,112,62].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_198()
    ensures
        u::copy_literal_ok(u::copy_registry()[198]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l198(Seq::empty());
    p::all();
    reveal_strlit("<p>Only GET is supported on this page.</p>");
    assert(n::codes("<p>Only GET is supported on this page.</p>"@) =~= data_198());
    assert(n::clean(data_198(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<p>Only GET is supported on this page.</p>"@);
}

pub open spec fn data_199() -> Seq<nat> {
    ns![71u32,69,84,44,32,80,79,83,84].map_values(|x: u32| x as nat)
}

pub proof fn literal_199()
    ensures
        u::copy_literal_ok(u::copy_registry()[199]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l199(Seq::empty());
    p::all();
    reveal_strlit("GET, POST");
    assert(n::codes("GET, POST"@) =~= data_199());
    assert(n::clean(data_199(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("GET, POST"@);
}

pub open spec fn data_200() -> Seq<nat> {
    ns![71u32,69,84].map_values(|x: u32| x as nat)
}

pub proof fn literal_200()
    ensures
        u::copy_literal_ok(u::copy_registry()[200]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l200(Seq::empty());
    p::all();
    reveal_strlit("GET");
    assert(n::codes("GET"@) =~= data_200());
    assert(n::clean(data_200(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("GET"@);
}

pub open spec fn data_201() -> Seq<nat> {
    ns![78u32,111,116,32,102,111,117,110,100].map_values(|x: u32| x as nat)
}

pub proof fn literal_201()
    ensures
        u::copy_literal_ok(u::copy_registry()[201]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l201(Seq::empty());
    p::all();
    reveal_strlit("Not found");
    assert(n::codes("Not found"@) =~= data_201());
    assert(n::clean(data_201(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("Not found"@);
}

pub open spec fn data_202() -> Seq<nat> {
    ns![60u32,112,62,84,104,101,32,114,101,113,117,101,115,116,101,100,32,112,97,103,101,32,100,111,101,115,32,110,111,116,32,101,120,105,115,116,46,60,47,112,62].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_202()
    ensures
        u::copy_literal_ok(u::copy_registry()[202]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l202(Seq::empty());
    p::all();
    reveal_strlit("<p>The requested page does not exist.</p>");
    assert(n::codes("<p>The requested page does not exist.</p>"@) =~= data_202());
    assert(n::clean(data_202(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<p>The requested page does not exist.</p>"@);
}

pub open spec fn data_203() -> Seq<nat> {
    ns![61u32,].map_values(|x: u32| x as nat)
}

pub proof fn literal_203()
    ensures
        u::copy_literal_ok(u::copy_registry()[203]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l203(Seq::empty());
    p::all();
    reveal_strlit("=");
    assert(n::codes("="@) =~= data_203());
    assert(n::clean(data_203(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("="@);
}

pub open spec fn data_204() -> Seq<nat> {
    ns![117u32,105,58,32,118,101,114,100,105,99,116,58,32,98,111,100,121,32,110,111,116,32,112,97,114,115,101,97,98,108,101].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_204()
    ensures
        u::copy_literal_ok(u::copy_registry()[204]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l204(Seq::empty());
    p::all();
    reveal_strlit("ui: verdict: body not parseable");
    assert(n::codes("ui: verdict: body not parseable"@) =~= data_204());
    assert(n::clean(data_204(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("ui: verdict: body not parseable"@);
}

pub open spec fn data_205() -> Seq<nat> {
    ns![118u32,101,114,100,105,99,116].map_values(|x: u32| x as nat)
}

pub proof fn literal_205()
    ensures
        u::copy_literal_ok(u::copy_registry()[205]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l205(Seq::empty());
    p::all();
    reveal_strlit("verdict");
    assert(n::codes("verdict"@) =~= data_205());
    assert(n::clean(data_205(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("verdict"@);
}

pub open spec fn data_206() -> Seq<nat> {
    ns![114u32,101,118,105,101,119,101,114].map_values(|x: u32| x as nat)
}

pub proof fn literal_206()
    ensures
        u::copy_literal_ok(u::copy_registry()[206]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l206(Seq::empty());
    p::all();
    reveal_strlit("reviewer");
    assert(n::codes("reviewer"@) =~= data_206());
    assert(n::clean(data_206(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("reviewer"@);
}

pub open spec fn data_207() -> Seq<nat> {
    ns![99u32,111,109,109,101,110,116].map_values(|x: u32| x as nat)
}

pub proof fn literal_207()
    ensures
        u::copy_literal_ok(u::copy_registry()[207]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l207(Seq::empty());
    p::all();
    reveal_strlit("comment");
    assert(n::codes("comment"@) =~= data_207());
    assert(n::clean(data_207(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("comment"@);
}

} // verus!
}
pub mod batch_208 {
    use crate::{
        k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
        k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
    };
    use ckc_spec::ui as u;
    use vstd::prelude::*;
    use vstd::utf8::*;
    verus! {

pub open spec fn data_208() -> Seq<nat> {
    ns![114u32,101,118,105,101,119,95,115,104,97,50,53,54].map_values(|x: u32| x as nat)
}

pub proof fn literal_208()
    ensures
        u::copy_literal_ok(u::copy_registry()[208]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l208(Seq::empty());
    p::all();
    reveal_strlit("review_sha256");
    assert(n::codes("review_sha256"@) =~= data_208());
    assert(n::clean(data_208(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("review_sha256"@);
}

pub open spec fn data_209() -> Seq<nat> {
    ns![108u32,101,100,103,101,114,95,115,104,97,50,53,54].map_values(|x: u32| x as nat)
}

pub proof fn literal_209()
    ensures
        u::copy_literal_ok(u::copy_registry()[209]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l209(Seq::empty());
    p::all();
    reveal_strlit("ledger_sha256");
    assert(n::codes("ledger_sha256"@) =~= data_209());
    assert(n::clean(data_209(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("ledger_sha256"@);
}

pub open spec fn data_210() -> Seq<nat> {
    ns![99u32,115,114,102].map_values(|x: u32| x as nat)
}

pub proof fn literal_210()
    ensures
        u::copy_literal_ok(u::copy_registry()[210]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l210(Seq::empty());
    p::all();
    reveal_strlit("csrf");
    assert(n::codes("csrf"@) =~= data_210());
    assert(n::clean(data_210(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("csrf"@);
}

pub open spec fn data_211() -> Seq<nat> {
    ns![117u32,105,58,32,118,101,114,100,105,99,116,58,32,109,105,115,115,105,110,103,32,102,105,101,108,100,32].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_211()
    ensures
        u::copy_literal_ok(u::copy_registry()[211]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l211(Seq::empty());
    p::all();
    reveal_strlit("ui: verdict: missing field ");
    assert(n::codes("ui: verdict: missing field "@) =~= data_211());
    assert(n::clean(data_211(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("ui: verdict: missing field "@);
}

pub open spec fn data_212() -> Seq<nat> {
    ns![117u32,105,58,32,118,101,114,100,105,99,116,58,32,100,117,112,108,105,99,97,116,101,32,102,105,101,108,100,32].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_212()
    ensures
        u::copy_literal_ok(u::copy_registry()[212]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l212(Seq::empty());
    p::all();
    reveal_strlit("ui: verdict: duplicate field ");
    assert(n::codes("ui: verdict: duplicate field "@) =~= data_212());
    assert(n::clean(data_212(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("ui: verdict: duplicate field "@);
}

pub open spec fn data_213() -> Seq<nat> {
    ns![117u32,105,58,32,118,101,114,100,105,99,116,58,32,117,110,107,110,111,119,110,32,102,105,101,108,100,32].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_213()
    ensures
        u::copy_literal_ok(u::copy_registry()[213]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l213(Seq::empty());
    p::all();
    reveal_strlit("ui: verdict: unknown field ");
    assert(n::codes("ui: verdict: unknown field "@) =~= data_213());
    assert(n::clean(data_213(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("ui: verdict: unknown field "@);
}

pub open spec fn data_214() -> Seq<nat> {
    ns![117u32,105,58,32,118,101,114,100,105,99,116,58,32,105,110,118,97,108,105,100,32,118,101,114,100,105,99,116].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_214()
    ensures
        u::copy_literal_ok(u::copy_registry()[214]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l214(Seq::empty());
    p::all();
    reveal_strlit("ui: verdict: invalid verdict");
    assert(n::codes("ui: verdict: invalid verdict"@) =~= data_214());
    assert(n::clean(data_214(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("ui: verdict: invalid verdict"@);
}

pub open spec fn data_215() -> Seq<nat> {
    ns![117u32,105,58,32,118,101,114,100,105,99,116,58,32,105,110,118,97,108,105,100,32,114,101,118,105,101,119,101,114].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_215()
    ensures
        u::copy_literal_ok(u::copy_registry()[215]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l215(Seq::empty());
    p::all();
    reveal_strlit("ui: verdict: invalid reviewer");
    assert(n::codes("ui: verdict: invalid reviewer"@) =~= data_215());
    assert(n::clean(data_215(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("ui: verdict: invalid reviewer"@);
}

pub open spec fn data_216() -> Seq<nat> {
    ns![117u32,105,58,32,118,101,114,100,105,99,116,58,32,105,110,118,97,108,105,100,32,99,111,109,109,101,110,116].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_216()
    ensures
        u::copy_literal_ok(u::copy_registry()[216]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l216(Seq::empty());
    p::all();
    reveal_strlit("ui: verdict: invalid comment");
    assert(n::codes("ui: verdict: invalid comment"@) =~= data_216());
    assert(n::clean(data_216(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("ui: verdict: invalid comment"@);
}

pub open spec fn data_217() -> Seq<nat> {
    ns![117u32,105,58,32,118,101,114,100,105,99,116,58,32,105,110,118,97,108,105,100,32,114,101,118,105,101,119,95,115,104,97,50,53,54].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_217()
    ensures
        u::copy_literal_ok(u::copy_registry()[217]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l217(Seq::empty());
    p::all();
    reveal_strlit("ui: verdict: invalid review_sha256");
    assert(n::codes("ui: verdict: invalid review_sha256"@) =~= data_217());
    assert(n::clean(data_217(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("ui: verdict: invalid review_sha256"@);
}

pub open spec fn data_218() -> Seq<nat> {
    ns![97u32,98,115,101,110,116].map_values(|x: u32| x as nat)
}

pub proof fn literal_218()
    ensures
        u::copy_literal_ok(u::copy_registry()[218]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l218(Seq::empty());
    p::all();
    reveal_strlit("absent");
    assert(n::codes("absent"@) =~= data_218());
    assert(n::clean(data_218(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("absent"@);
}

pub open spec fn data_219() -> Seq<nat> {
    ns![117u32,105,58,32,118,101,114,100,105,99,116,58,32,105,110,118,97,108,105,100,32,108,101,100,103,101,114,95,115,104,97,50,53,54].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_219()
    ensures
        u::copy_literal_ok(u::copy_registry()[219]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l219(Seq::empty());
    p::all();
    reveal_strlit("ui: verdict: invalid ledger_sha256");
    assert(n::codes("ui: verdict: invalid ledger_sha256"@) =~= data_219());
    assert(n::clean(data_219(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("ui: verdict: invalid ledger_sha256"@);
}

pub open spec fn data_220() -> Seq<nat> {
    ns![117u32,105,58,32,118,101,114,100,105,99,116,58,32,98,111,100,121,32,110,111,116,32,100,101,99,111,100,97,98,108,101].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_220()
    ensures
        u::copy_literal_ok(u::copy_registry()[220]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l220(Seq::empty());
    p::all();
    reveal_strlit("ui: verdict: body not decodable");
    assert(n::codes("ui: verdict: body not decodable"@) =~= data_220());
    assert(n::clean(data_220(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("ui: verdict: body not decodable"@);
}

pub open spec fn data_221() -> Seq<nat> {
    ns![9u32,].map_values(|x: u32| x as nat)
}

pub proof fn literal_221()
    ensures
        u::copy_literal_ok(u::copy_registry()[221]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l221(Seq::empty());
    p::all();
    reveal_strlit("\t");
    assert(n::codes("\t"@) =~= data_221());
    assert(n::clean(data_221(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("\t"@);
}

pub open spec fn data_222() -> Seq<nat> {
    ns![117u32,105,58,32,118,101,114,100,105,99,116,58,32,109,97,110,105,102,101,115,116,32,100,101,114,105,118,97,116,105,111,110,32,102,97,105,108,101,100,58,32].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_222()
    ensures
        u::copy_literal_ok(u::copy_registry()[222]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l222(Seq::empty());
    p::all();
    reveal_strlit("ui: verdict: manifest derivation failed: ");
    assert(n::codes("ui: verdict: manifest derivation failed: "@) =~= data_222());
    assert(n::clean(data_222(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("ui: verdict: manifest derivation failed: "@);
}

pub open spec fn data_223() -> Seq<nat> {
    ns![117u32,105,58,32,118,101,114,100,105,99,116,58,32,109,97,110,105,102,101,115,116,32,100,101,114,105,118,97,116,105,111,110,32,102,97,105,108,101,100,58,32,100,111,99,105,100,32,114,111,119,32,109,105,115,115,105,110,103].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_223()
    ensures
        u::copy_literal_ok(u::copy_registry()[223]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l223(Seq::empty());
    p::all();
    reveal_strlit("ui: verdict: manifest derivation failed: docid row missing");
    assert(n::codes("ui: verdict: manifest derivation failed: docid row missing"@) =~= data_223());
    assert(n::clean(data_223(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("ui: verdict: manifest derivation failed: docid row missing"@);
}

} // verus!
}
pub mod batch_224 {
    use crate::{
        k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
        k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
    };
    use ckc_spec::ui as u;
    use vstd::prelude::*;
    use vstd::utf8::*;
    verus! {

pub open spec fn data_224() -> Seq<nat> {
    ns![117u32,105,58,32,118,101,114,100,105,99,116,58,32,115,117,98,106,101,99,116,32,99,104,97,110,103,101,100].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_224()
    ensures
        u::copy_literal_ok(u::copy_registry()[224]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l224(Seq::empty());
    p::all();
    reveal_strlit("ui: verdict: subject changed");
    assert(n::codes("ui: verdict: subject changed"@) =~= data_224());
    assert(n::clean(data_224(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("ui: verdict: subject changed"@);
}

pub open spec fn data_225() -> Seq<nat> {
    ns![84u32,104,101,32,100,111,99,117,109,101,110,116,32,111,114,32,105,116,115,32,115,111,117,114,99,101,32,99,104,97,110,103,101,100,32,97,102,116,101,114,32,116,104,105,115,32,112,97,103,101,32,119,97,115,32,108,111,97,100,101,100,46,32,84,104,101,32,100,101,99,105,115,105,111,110,32,119,97,115,32,110,111,116,32,114,101,99,111,114,100,101,100,46,32,79,112,101,110,32,116,104,101,32,100,111,99,117,109,101,110,116,32,112,97,103,101,32,97,103,97,105,110,32,97,110,100,32,99,104,101,99,107,32,116,104,101,32,99,117,114,114,101,110,116,32,118,101,114,115,105,111,110,46].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_225()
    ensures
        u::copy_literal_ok(u::copy_registry()[225]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l225(Seq::empty());
    p::all();
    reveal_strlit(
        "The document or its source changed after this page was loaded. The decision was not recorded. Open the document page again and check the current version.",
    );
    assert(n::codes(
        "The document or its source changed after this page was loaded. The decision was not recorded. Open the document page again and check the current version."@,
    ) =~= data_225());
    assert(n::clean(data_225(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge(
        "The document or its source changed after this page was loaded. The decision was not recorded. Open the document page again and check the current version."@,
    );
}

pub open spec fn data_226() -> Seq<nat> {
    ns![117u32,105,58,32,97,100,106,117,100,105,99,97,116,105,111,110,32,108,101,100,103,101,114,32,105,110,118,97,108,105,100,58,32].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_226()
    ensures
        u::copy_literal_ok(u::copy_registry()[226]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l226(Seq::empty());
    p::all();
    reveal_strlit("ui: adjudication ledger invalid: ");
    assert(n::codes("ui: adjudication ledger invalid: "@) =~= data_226());
    assert(n::clean(data_226(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("ui: adjudication ledger invalid: "@);
}

pub open spec fn data_227() -> Seq<nat> {
    ns![68u32,101,99,105,115,105,111,110,32,114,101,99,111,114,100,101,100].map_values(
        |x: u32| x as nat,
    )
}

pub proof fn literal_227()
    ensures
        u::copy_literal_ok(u::copy_registry()[227]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l227(Seq::empty());
    p::all();
    reveal_strlit("Decision recorded");
    assert(n::codes("Decision recorded"@) =~= data_227());
    assert(n::clean(data_227(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("Decision recorded"@);
}

pub open spec fn data_228() -> Seq<nat> {
    ns![60u32,112,62,84,104,101,32,100,101,99,105,115,105,111,110,32,119,97,115,32,114,101,99,111,114,100,101,100,46,60,47,112,62].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_228()
    ensures
        u::copy_literal_ok(u::copy_registry()[228]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l228(Seq::empty());
    p::all();
    reveal_strlit("<p>The decision was recorded.</p>");
    assert(n::codes("<p>The decision was recorded.</p>"@) =~= data_228());
    assert(n::clean(data_228(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("<p>The decision was recorded.</p>"@);
}

pub open spec fn data_229() -> Seq<nat> {
    ns![47u32,100,111,99,47].map_values(|x: u32| x as nat)
}

pub proof fn literal_229()
    ensures
        u::copy_literal_ok(u::copy_registry()[229]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l229(Seq::empty());
    p::all();
    reveal_strlit("/doc/");
    assert(n::codes("/doc/"@) =~= data_229());
    assert(n::clean(data_229(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("/doc/"@);
}

pub open spec fn data_230() -> Seq<nat> {
    ns![104u32,116,116,112,58,47,47,49,50,55,46,48,46,48,46,49,58].map_values(|x: u32| x as nat)
}

pub proof fn literal_230()
    ensures
        u::copy_literal_ok(u::copy_registry()[230]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l230(Seq::empty());
    p::all();
    reveal_strlit("http://127.0.0.1:");
    assert(n::codes("http://127.0.0.1:"@) =~= data_230());
    assert(n::clean(data_230(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("http://127.0.0.1:"@);
}

pub open spec fn data_231() -> Seq<nat> {
    ns![117u32,105,58,32,118,101,114,100,105,99,116,58,32,111,114,105,103,105,110,32,110,111,116,32,97,108,108,111,119,101,100].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_231()
    ensures
        u::copy_literal_ok(u::copy_registry()[231]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l231(Seq::empty());
    p::all();
    reveal_strlit("ui: verdict: origin not allowed");
    assert(n::codes("ui: verdict: origin not allowed"@) =~= data_231());
    assert(n::clean(data_231(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("ui: verdict: origin not allowed"@);
}

pub open spec fn data_232() -> Seq<nat> {
    ns![97u32,112,112,108,105,99,97,116,105,111,110,47,120,45,119,119,119,45,102,111,114,109,45,117,114,108,101,110,99,111,100,101,100].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_232()
    ensures
        u::copy_literal_ok(u::copy_registry()[232]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l232(Seq::empty());
    p::all();
    reveal_strlit("application/x-www-form-urlencoded");
    assert(n::codes("application/x-www-form-urlencoded"@) =~= data_232());
    assert(n::clean(data_232(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("application/x-www-form-urlencoded"@);
}

pub open spec fn data_233() -> Seq<nat> {
    ns![117u32,105,58,32,118,101,114,100,105,99,116,58,32,117,110,115,117,112,112,111,114,116,101,100,32,99,111,110,116,101,110,116,32,116,121,112,101].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_233()
    ensures
        u::copy_literal_ok(u::copy_registry()[233]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l233(Seq::empty());
    p::all();
    reveal_strlit("ui: verdict: unsupported content type");
    assert(n::codes("ui: verdict: unsupported content type"@) =~= data_233());
    assert(n::clean(data_233(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("ui: verdict: unsupported content type"@);
}

pub open spec fn data_234() -> Seq<nat> {
    ns![117u32,105,58,32,118,101,114,100,105,99,116,58,32,109,105,115,115,105,110,103,32,98,111,100,121].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_234()
    ensures
        u::copy_literal_ok(u::copy_registry()[234]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l234(Seq::empty());
    p::all();
    reveal_strlit("ui: verdict: missing body");
    assert(n::codes("ui: verdict: missing body"@) =~= data_234());
    assert(n::clean(data_234(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("ui: verdict: missing body"@);
}

pub open spec fn data_235() -> Seq<nat> {
    ns![117u32,105,58,32,118,101,114,100,105,99,116,58,32,105,110,118,97,108,105,100,32,99,115,114,102,32,116,111,107,101,110].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_235()
    ensures
        u::copy_literal_ok(u::copy_registry()[235]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l235(Seq::empty());
    p::all();
    reveal_strlit("ui: verdict: invalid csrf token");
    assert(n::codes("ui: verdict: invalid csrf token"@) =~= data_235());
    assert(n::clean(data_235(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("ui: verdict: invalid csrf token"@);
}

pub open spec fn data_236() -> Seq<nat> {
    ns![49u32,50,55,46,48,46,48,46,49,58].map_values(|x: u32| x as nat)
}

pub proof fn literal_236()
    ensures
        u::copy_literal_ok(u::copy_registry()[236]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l236(Seq::empty());
    p::all();
    reveal_strlit("127.0.0.1:");
    assert(n::codes("127.0.0.1:"@) =~= data_236());
    assert(n::clean(data_236(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("127.0.0.1:"@);
}

pub open spec fn data_237() -> Seq<nat> {
    ns![117u32,105,58,32,114,101,113,117,101,115,116,58,32,104,111,115,116,32,110,111,116,32,97,108,108,111,119,101,100].map_values(
    |x: u32| x as nat)
}

pub proof fn literal_237()
    ensures
        u::copy_literal_ok(u::copy_registry()[237]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l237(Seq::empty());
    p::all();
    reveal_strlit("ui: request: host not allowed");
    assert(n::codes("ui: request: host not allowed"@) =~= data_237());
    assert(n::clean(data_237(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("ui: request: host not allowed"@);
}

pub open spec fn data_238() -> Seq<nat> {
    ns![80u32,79,83,84].map_values(|x: u32| x as nat)
}

pub proof fn literal_238()
    ensures
        u::copy_literal_ok(u::copy_registry()[238]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    l::l238(Seq::empty());
    p::all();
    reveal_strlit("POST");
    assert(n::codes("POST"@) =~= data_238());
    assert(n::clean(data_238(), p::css_codes(), p::marketing_codes(), p::relative_codes()))
        by (compute_only);
    n::clean_bridge("POST"@);
}

} // verus!
}
pub mod css {
    use crate::{
        k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
        k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
    };
    use ckc_spec::ui as u;
    use vstd::prelude::*;
    use vstd::utf8::*;
    pub mod window_0 {
        #[cfg(verus_keep_ghost)]
        use super::{data_038, shape_038};
        use crate::{
            k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
            k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
        };
        use ckc_spec::ui as u;
        use vstd::prelude::*;
        use vstd::utf8::*;
        verus! {

pub closed spec fn bytes_038_0() -> u::Bytes {
    ns![98u8,111,100,121,32,123,32,109,97,114,103,105,110,58,32,48,32,97,117,116,111,59,32,109,97,120,45,119,105,100,116,104,58,32,55,50,114,101,109,59,32,112,97,100,100,105,110,103,58,32,48,32,49,46,53,114,101,109,32,52,114,101,109,59,32,102,111,110,116,45,102,97,109,105,108,121,58,32,115,121,115,116,101,109,45,117,105,44,32,115,97,110,115,45,115,101,114,105,102,59,32,108,105,110,101,45,104,101,105,103,104,116,58,32,49,46,53,53,59,32,99,111,108,111,114,58,32,35,49,49,49,56,50,55,59,32,98,97,99,107,103,114,111,117,110,100,58,32,35,102,102,102,102,102,102,59,32,125,10,97,32,123,32,99,111,108,111,114,58,32,35,49,100,52,101,100,56,59,32,125,10,97,58,102,111,99,117,115,45,118,105,115,105,98,108,101,44,32,115,117,109,109,97,114,121,58,102,111,99,117,115,45,118,105,115,105,98,108,101,32,123,32,111,117,116,108,105,110,101,58,32,51,112,120,32,115,111,108,105,100,32,35,49,100,52,101,100,56,59,32,111,117,116,108,105,110,101,45,111,102,102,115,101,116,58,32,50,112,120,59,32,125,10,46,115,107,105,112,32,123,32,112,111,115,105,116,105,111]
}

pub closed spec fn window_038_0() -> Seq<nat> {
    scan::byte_codes(bytes_038_0())
}

pub proof fn bind_038_0()
    ensures
        data_038().len() == 10326,
        data_038().subrange(0, 288) == window_038_0(),
        window_038_0().len() == 288,
{
    hide(data_038);
    hide(ch::css_text_bytes);
    shape_038();
    assert(ch::css_text_bytes().subrange(0, 288) == bytes_038_0()) by (compute_only);
    assert(bytes_038_0().len() == 288) by (compute_only);
    scan::byte_codes_subrange(ch::css_text_bytes(), 0, 288);
    assert(data_038() == scan::byte_codes(ch::css_text_bytes())) by {
        reveal(data_038);
    };
    assert(window_038_0() == scan::byte_codes(bytes_038_0())) by {
        reveal(window_038_0);
    };
}

pub proof fn check_038_0()
    ensures
        scan::clean_range(
            data_038(),
            p::css_codes(),
            p::marketing_codes(),
            p::relative_codes(),
            0,
            256,
        ),
{
    hide(data_038);
    hide(scan::clean_range);
    hide(p::css_codes);
    hide(p::marketing_codes);
    hide(p::relative_codes);
    bind_038_0();
    p::bounds();
    assert(scan::clean_range(
        window_038_0(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        0,
        256,
    )) by (compute_only);
    scan::clean_window(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        0,
        288,
        0,
        256,
    );
}

} // verus!
    }
    pub mod window_1 {
        #[cfg(verus_keep_ghost)]
        use super::{data_038, shape_038};
        use crate::{
            k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
            k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
        };
        use ckc_spec::ui as u;
        use vstd::prelude::*;
        use vstd::utf8::*;
        verus! {

pub closed spec fn bytes_038_1() -> u::Bytes {
    ns![110u8,101,45,111,102,102,115,101,116,58,32,50,112,120,59,32,125,10,46,115,107,105,112,32,123,32,112,111,115,105,116,105,111,110,58,32,97,98,115,111,108,117,116,101,59,32,108,101,102,116,58,32,45,57,57,57,112,120,59,32,116,111,112,58,32,48,59,32,112,97,100,100,105,110,103,58,32,48,46,53,114,101,109,32,49,114,101,109,59,32,98,97,99,107,103,114,111,117,110,100,58,32,35,102,102,102,102,102,102,59,32,99,111,108,111,114,58,32,35,49,100,52,101,100,56,59,32,125,10,46,115,107,105,112,58,102,111,99,117,115,32,123,32,108,101,102,116,58,32,48,59,32,122,45,105,110,100,101,120,58,32,49,59,32,125,10,110,97,118,46,99,114,117,109,98,115,32,123,32,112,97,100,100,105,110,103,58,32,49,114,101,109,32,48,59,32,98,111,114,100,101,114,45,98,111,116,116,111,109,58,32,49,112,120,32,115,111,108,105,100,32,35,101,53,101,55,101,98,59,32,125,10,104,49,32,123,32,102,111,110,116,45,115,105,122,101,58,32,49,46,53,114,101,109,59,32,125,10,104,50,32,123,32,102,111,110,116,45,115,105,122,101,58,32,49,46,50,53,114,101,109,59,32,125,10,104,51,32,123]
}

pub closed spec fn window_038_1() -> Seq<nat> {
    scan::byte_codes(bytes_038_1())
}

pub proof fn bind_038_1()
    ensures
        data_038().len() == 10326,
        data_038().subrange(255, 544) == window_038_1(),
        window_038_1().len() == 289,
{
    hide(data_038);
    hide(ch::css_text_bytes);
    shape_038();
    assert(ch::css_text_bytes().subrange(255, 544) == bytes_038_1()) by (compute_only);
    assert(bytes_038_1().len() == 289) by (compute_only);
    scan::byte_codes_subrange(ch::css_text_bytes(), 255, 544);
    assert(data_038() == scan::byte_codes(ch::css_text_bytes())) by {
        reveal(data_038);
    };
    assert(window_038_1() == scan::byte_codes(bytes_038_1())) by {
        reveal(window_038_1);
    };
}

pub proof fn check_038_1()
    ensures
        scan::clean_range(
            data_038(),
            p::css_codes(),
            p::marketing_codes(),
            p::relative_codes(),
            256,
            512,
        ),
{
    hide(data_038);
    hide(scan::clean_range);
    hide(p::css_codes);
    hide(p::marketing_codes);
    hide(p::relative_codes);
    bind_038_1();
    p::bounds();
    assert(scan::clean_range(
        window_038_1(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        1,
        257,
    )) by (compute_only);
    scan::clean_window(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        255,
        544,
        256,
        512,
    );
}

} // verus!
    }
    pub mod window_2 {
        #[cfg(verus_keep_ghost)]
        use super::{data_038, shape_038};
        use crate::{
            k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
            k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
        };
        use ckc_spec::ui as u;
        use vstd::prelude::*;
        use vstd::utf8::*;
        verus! {

pub closed spec fn bytes_038_2() -> u::Bytes {
    ns![125u8,10,104,50,32,123,32,102,111,110,116,45,115,105,122,101,58,32,49,46,50,53,114,101,109,59,32,125,10,104,51,32,123,32,102,111,110,116,45,115,105,122,101,58,32,49,46,48,53,114,101,109,59,32,125,10,104,49,32,97,46,115,111,117,114,99,101,32,123,32,102,111,110,116,45,115,105,122,101,58,32,49,114,101,109,59,32,102,111,110,116,45,119,101,105,103,104,116,58,32,52,48,48,59,32,109,97,114,103,105,110,45,108,101,102,116,58,32,48,46,53,114,101,109,59,32,125,10,116,97,98,108,101,32,123,32,98,111,114,100,101,114,45,99,111,108,108,97,112,115,101,58,32,99,111,108,108,97,112,115,101,59,32,119,105,100,116,104,58,32,49,48,48,37,59,32,109,97,114,103,105,110,58,32,49,114,101,109,32,48,59,32,125,10,116,104,44,32,116,100,32,123,32,116,101,120,116,45,97,108,105,103,110,58,32,108,101,102,116,59,32,112,97,100,100,105,110,103,58,32,48,46,52,114,101,109,32,48,46,54,114,101,109,59,32,98,111,114,100,101,114,45,98,111,116,116,111,109,58,32,49,112,120,32,115,111,108,105,100,32,35,101,53,101,55,101,98,59,32,118,101,114,116,105,99,97,108,45,97]
}

pub closed spec fn window_038_2() -> Seq<nat> {
    scan::byte_codes(bytes_038_2())
}

pub proof fn bind_038_2()
    ensures
        data_038().len() == 10326,
        data_038().subrange(511, 800) == window_038_2(),
        window_038_2().len() == 289,
{
    hide(data_038);
    hide(ch::css_text_bytes);
    shape_038();
    assert(ch::css_text_bytes().subrange(511, 800) == bytes_038_2()) by (compute_only);
    assert(bytes_038_2().len() == 289) by (compute_only);
    scan::byte_codes_subrange(ch::css_text_bytes(), 511, 800);
    assert(data_038() == scan::byte_codes(ch::css_text_bytes())) by {
        reveal(data_038);
    };
    assert(window_038_2() == scan::byte_codes(bytes_038_2())) by {
        reveal(window_038_2);
    };
}

pub proof fn check_038_2()
    ensures
        scan::clean_range(
            data_038(),
            p::css_codes(),
            p::marketing_codes(),
            p::relative_codes(),
            512,
            768,
        ),
{
    hide(data_038);
    hide(scan::clean_range);
    hide(p::css_codes);
    hide(p::marketing_codes);
    hide(p::relative_codes);
    bind_038_2();
    p::bounds();
    assert(scan::clean_range(
        window_038_2(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        1,
        257,
    )) by (compute_only);
    scan::clean_window(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        511,
        800,
        512,
        768,
    );
}

} // verus!
    }
    pub mod window_3 {
        #[cfg(verus_keep_ghost)]
        use super::{data_038, shape_038};
        use crate::{
            k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
            k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
        };
        use ckc_spec::ui as u;
        use vstd::prelude::*;
        use vstd::utf8::*;
        verus! {

pub closed spec fn bytes_038_3() -> u::Bytes {
    ns![111u8,109,58,32,49,112,120,32,115,111,108,105,100,32,35,101,53,101,55,101,98,59,32,118,101,114,116,105,99,97,108,45,97,108,105,103,110,58,32,116,111,112,59,32,125,10,116,104,32,123,32,98,111,114,100,101,114,45,98,111,116,116,111,109,58,32,50,112,120,32,115,111,108,105,100,32,35,49,49,49,56,50,55,59,32,125,10,116,97,98,108,101,46,99,111,109,112,97,99,116,32,123,32,119,105,100,116,104,58,32,97,117,116,111,59,32,125,10,116,97,98,108,101,46,99,111,109,112,97,99,116,32,116,104,44,32,116,97,98,108,101,46,99,111,109,112,97,99,116,32,116,100,32,123,32,112,97,100,100,105,110,103,45,114,105,103,104,116,58,32,50,114,101,109,59,32,125,10,116,97,98,108,101,46,114,101,99,111,114,100,115,32,123,32,116,97,98,108,101,45,108,97,121,111,117,116,58,32,102,105,120,101,100,59,32,125,10,116,97,98,108,101,46,114,101,99,111,114,100,115,32,116,104,32,123,32,98,111,120,45,115,105,122,105,110,103,58,32,98,111,114,100,101,114,45,98,111,120,59,32,125,10,116,97,98,108,101,46,114,101,99,111,114,100,115,32,116,104,58,110,116,104,45,99,104,105,108,100,40]
}

pub closed spec fn window_038_3() -> Seq<nat> {
    scan::byte_codes(bytes_038_3())
}

pub proof fn bind_038_3()
    ensures
        data_038().len() == 10326,
        data_038().subrange(767, 1056) == window_038_3(),
        window_038_3().len() == 289,
{
    hide(data_038);
    hide(ch::css_text_bytes);
    shape_038();
    assert(ch::css_text_bytes().subrange(767, 1056) == bytes_038_3()) by (compute_only);
    assert(bytes_038_3().len() == 289) by (compute_only);
    scan::byte_codes_subrange(ch::css_text_bytes(), 767, 1056);
    assert(data_038() == scan::byte_codes(ch::css_text_bytes())) by {
        reveal(data_038);
    };
    assert(window_038_3() == scan::byte_codes(bytes_038_3())) by {
        reveal(window_038_3);
    };
}

pub proof fn check_038_3()
    ensures
        scan::clean_range(
            data_038(),
            p::css_codes(),
            p::marketing_codes(),
            p::relative_codes(),
            768,
            1024,
        ),
{
    hide(data_038);
    hide(scan::clean_range);
    hide(p::css_codes);
    hide(p::marketing_codes);
    hide(p::relative_codes);
    bind_038_3();
    p::bounds();
    assert(scan::clean_range(
        window_038_3(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        1,
        257,
    )) by (compute_only);
    scan::clean_window(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        767,
        1056,
        768,
        1024,
    );
}

} // verus!
    }
    pub mod window_4 {
        #[cfg(verus_keep_ghost)]
        use super::{data_038, shape_038};
        use crate::{
            k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
            k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
        };
        use ckc_spec::ui as u;
        use vstd::prelude::*;
        use vstd::utf8::*;
        verus! {

pub closed spec fn bytes_038_4() -> u::Bytes {
    ns![111u8,120,59,32,125,10,116,97,98,108,101,46,114,101,99,111,114,100,115,32,116,104,58,110,116,104,45,99,104,105,108,100,40,49,41,32,123,32,119,105,100,116,104,58,32,49,50,37,59,32,125,10,116,97,98,108,101,46,114,101,99,111,114,100,115,32,116,104,58,110,116,104,45,99,104,105,108,100,40,50,41,32,123,32,119,105,100,116,104,58,32,49,56,37,59,32,125,10,116,97,98,108,101,46,114,101,99,111,114,100,115,32,116,104,58,110,116,104,45,99,104,105,108,100,40,51,41,32,123,32,119,105,100,116,104,58,32,50,50,37,59,32,125,10,116,97,98,108,101,46,114,101,99,111,114,100,115,32,116,104,58,110,116,104,45,99,104,105,108,100,40,52,41,32,123,32,119,105,100,116,104,58,32,49,48,37,59,32,125,10,46,99,104,105,112,32,123,32,100,105,115,112,108,97,121,58,32,105,110,108,105,110,101,45,98,108,111,99,107,59,32,112,97,100,100,105,110,103,58,32,48,46,49,114,101,109,32,48,46,54,114,101,109,59,32,98,111,114,100,101,114,45,114,97,100,105,117,115,58,32,57,57,57,112,120,59,32,102,111,110,116,45,115,105,122,101,58,32,48,46,56,53,114,101,109,59,32,102,111]
}

pub closed spec fn window_038_4() -> Seq<nat> {
    scan::byte_codes(bytes_038_4())
}

pub proof fn bind_038_4()
    ensures
        data_038().len() == 10326,
        data_038().subrange(1023, 1312) == window_038_4(),
        window_038_4().len() == 289,
{
    hide(data_038);
    hide(ch::css_text_bytes);
    shape_038();
    assert(ch::css_text_bytes().subrange(1023, 1312) == bytes_038_4()) by (compute_only);
    assert(bytes_038_4().len() == 289) by (compute_only);
    scan::byte_codes_subrange(ch::css_text_bytes(), 1023, 1312);
    assert(data_038() == scan::byte_codes(ch::css_text_bytes())) by {
        reveal(data_038);
    };
    assert(window_038_4() == scan::byte_codes(bytes_038_4())) by {
        reveal(window_038_4);
    };
}

pub proof fn check_038_4()
    ensures
        scan::clean_range(
            data_038(),
            p::css_codes(),
            p::marketing_codes(),
            p::relative_codes(),
            1024,
            1280,
        ),
{
    hide(data_038);
    hide(scan::clean_range);
    hide(p::css_codes);
    hide(p::marketing_codes);
    hide(p::relative_codes);
    bind_038_4();
    p::bounds();
    assert(scan::clean_range(
        window_038_4(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        1,
        257,
    )) by (compute_only);
    scan::clean_window(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        1023,
        1312,
        1024,
        1280,
    );
}

} // verus!
    }
    pub mod window_5 {
        #[cfg(verus_keep_ghost)]
        use super::{data_038, shape_038};
        use crate::{
            k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
            k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
        };
        use ckc_spec::ui as u;
        use vstd::prelude::*;
        use vstd::utf8::*;
        verus! {

pub closed spec fn bytes_038_5() -> u::Bytes {
    ns![117u8,115,58,32,57,57,57,112,120,59,32,102,111,110,116,45,115,105,122,101,58,32,48,46,56,53,114,101,109,59,32,102,111,110,116,45,119,101,105,103,104,116,58,32,54,48,48,59,32,125,10,46,99,104,105,112,45,97,112,112,114,111,118,101,100,32,123,32,99,111,108,111,114,58,32,35,49,52,53,51,50,100,59,32,98,97,99,107,103,114,111,117,110,100,58,32,35,100,99,102,99,101,55,59,32,125,10,46,99,104,105,112,45,114,101,106,101,99,116,101,100,32,123,32,99,111,108,111,114,58,32,35,55,102,49,100,49,100,59,32,98,97,99,107,103,114,111,117,110,100,58,32,35,102,101,101,50,101,50,59,32,125,10,46,99,104,105,112,45,99,111,110,116,101,115,116,101,100,32,123,32,99,111,108,111,114,58,32,35,52,99,49,100,57,53,59,32,98,97,99,107,103,114,111,117,110,100,58,32,35,101,100,101,57,102,101,59,32,125,10,46,99,104,105,112,45,115,116,97,108,101,32,123,32,99,111,108,111,114,58,32,35,55,56,51,53,48,102,59,32,98,97,99,107,103,114,111,117,110,100,58,32,35,102,101,102,51,99,55,59,32,125,10,46,99,104,105,112,45,117,110,114,101,118,105,101,119,101,100]
}

pub closed spec fn window_038_5() -> Seq<nat> {
    scan::byte_codes(bytes_038_5())
}

pub proof fn bind_038_5()
    ensures
        data_038().len() == 10326,
        data_038().subrange(1279, 1568) == window_038_5(),
        window_038_5().len() == 289,
{
    hide(data_038);
    hide(ch::css_text_bytes);
    shape_038();
    assert(ch::css_text_bytes().subrange(1279, 1568) == bytes_038_5()) by (compute_only);
    assert(bytes_038_5().len() == 289) by (compute_only);
    scan::byte_codes_subrange(ch::css_text_bytes(), 1279, 1568);
    assert(data_038() == scan::byte_codes(ch::css_text_bytes())) by {
        reveal(data_038);
    };
    assert(window_038_5() == scan::byte_codes(bytes_038_5())) by {
        reveal(window_038_5);
    };
}

pub proof fn check_038_5()
    ensures
        scan::clean_range(
            data_038(),
            p::css_codes(),
            p::marketing_codes(),
            p::relative_codes(),
            1280,
            1536,
        ),
{
    hide(data_038);
    hide(scan::clean_range);
    hide(p::css_codes);
    hide(p::marketing_codes);
    hide(p::relative_codes);
    bind_038_5();
    p::bounds();
    assert(scan::clean_range(
        window_038_5(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        1,
        257,
    )) by (compute_only);
    scan::clean_window(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        1279,
        1568,
        1280,
        1536,
    );
}

} // verus!
    }
    pub mod window_6 {
        #[cfg(verus_keep_ghost)]
        use super::{data_038, shape_038};
        use crate::{
            k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
            k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
        };
        use ckc_spec::ui as u;
        use vstd::prelude::*;
        use vstd::utf8::*;
        verus! {

pub closed spec fn bytes_038_6() -> u::Bytes {
    ns![111u8,117,110,100,58,32,35,102,101,102,51,99,55,59,32,125,10,46,99,104,105,112,45,117,110,114,101,118,105,101,119,101,100,32,123,32,99,111,108,111,114,58,32,35,49,102,50,57,51,55,59,32,98,97,99,107,103,114,111,117,110,100,58,32,35,101,53,101,55,101,98,59,32,125,10,112,114,101,32,123,32,112,97,100,100,105,110,103,58,32,48,46,55,53,114,101,109,32,49,114,101,109,59,32,98,111,114,100,101,114,58,32,49,112,120,32,115,111,108,105,100,32,35,101,53,101,55,101,98,59,32,119,104,105,116,101,45,115,112,97,99,101,58,32,112,114,101,45,119,114,97,112,59,32,111,118,101,114,102,108,111,119,45,120,58,32,97,117,116,111,59,32,125,10,112,114,101,44,32,99,111,100,101,32,123,32,102,111,110,116,45,102,97,109,105,108,121,58,32,117,105,45,109,111,110,111,115,112,97,99,101,44,32,77,101,110,108,111,44,32,67,111,110,115,111,108,97,115,44,32,109,111,110,111,115,112,97,99,101,59,32,102,111,110,116,45,115,105,122,101,58,32,48,46,57,53,114,101,109,59,32,125,10,112,114,101,46,112,114,111,115,101,32,123,32,102,111,110,116,45,102,97,109,105,108,121,58,32,71]
}

pub closed spec fn window_038_6() -> Seq<nat> {
    scan::byte_codes(bytes_038_6())
}

pub proof fn bind_038_6()
    ensures
        data_038().len() == 10326,
        data_038().subrange(1535, 1824) == window_038_6(),
        window_038_6().len() == 289,
{
    hide(data_038);
    hide(ch::css_text_bytes);
    shape_038();
    assert(ch::css_text_bytes().subrange(1535, 1824) == bytes_038_6()) by (compute_only);
    assert(bytes_038_6().len() == 289) by (compute_only);
    scan::byte_codes_subrange(ch::css_text_bytes(), 1535, 1824);
    assert(data_038() == scan::byte_codes(ch::css_text_bytes())) by {
        reveal(data_038);
    };
    assert(window_038_6() == scan::byte_codes(bytes_038_6())) by {
        reveal(window_038_6);
    };
}

pub proof fn check_038_6()
    ensures
        scan::clean_range(
            data_038(),
            p::css_codes(),
            p::marketing_codes(),
            p::relative_codes(),
            1536,
            1792,
        ),
{
    hide(data_038);
    hide(scan::clean_range);
    hide(p::css_codes);
    hide(p::marketing_codes);
    hide(p::relative_codes);
    bind_038_6();
    p::bounds();
    assert(scan::clean_range(
        window_038_6(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        1,
        257,
    )) by (compute_only);
    scan::clean_window(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        1535,
        1824,
        1536,
        1792,
    );
}

} // verus!
    }
    pub mod window_7 {
        #[cfg(verus_keep_ghost)]
        use super::{data_038, shape_038};
        use crate::{
            k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
            k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
        };
        use ckc_spec::ui as u;
        use vstd::prelude::*;
        use vstd::utf8::*;
        verus! {

pub closed spec fn bytes_038_7() -> u::Bytes {
    ns![114u8,101,109,59,32,125,10,112,114,101,46,112,114,111,115,101,32,123,32,102,111,110,116,45,102,97,109,105,108,121,58,32,71,101,111,114,103,105,97,44,32,115,101,114,105,102,59,32,102,111,110,116,45,115,105,122,101,58,32,49,46,48,53,114,101,109,59,32,111,118,101,114,102,108,111,119,45,119,114,97,112,58,32,97,110,121,119,104,101,114,101,59,32,125,10,100,116,32,123,32,102,111,110,116,45,119,101,105,103,104,116,58,32,54,48,48,59,32,109,97,114,103,105,110,45,116,111,112,58,32,48,46,54,114,101,109,59,32,125,10,100,100,32,123,32,109,97,114,103,105,110,45,108,101,102,116,58,32,48,59,32,125,10,115,117,109,109,97,114,121,32,123,32,99,117,114,115,111,114,58,32,112,111,105,110,116,101,114,59,32,125,10,115,101,99,116,105,111,110,32,123,32,109,97,114,103,105,110,58,32,49,46,53,114,101,109,32,48,59,32,125,10,110,97,118,46,100,111,99,110,97,118,32,123,32,112,97,100,100,105,110,103,58,32,49,114,101,109,32,48,59,32,98,111,114,100,101,114,45,116,111,112,58,32,49,112,120,32,115,111,108,105,100,32,35,101,53,101,55,101,98,59,32,125,10,102,111,111,116]
}

pub closed spec fn window_038_7() -> Seq<nat> {
    scan::byte_codes(bytes_038_7())
}

pub proof fn bind_038_7()
    ensures
        data_038().len() == 10326,
        data_038().subrange(1791, 2080) == window_038_7(),
        window_038_7().len() == 289,
{
    hide(data_038);
    hide(ch::css_text_bytes);
    shape_038();
    assert(ch::css_text_bytes().subrange(1791, 2080) == bytes_038_7()) by (compute_only);
    assert(bytes_038_7().len() == 289) by (compute_only);
    scan::byte_codes_subrange(ch::css_text_bytes(), 1791, 2080);
    assert(data_038() == scan::byte_codes(ch::css_text_bytes())) by {
        reveal(data_038);
    };
    assert(window_038_7() == scan::byte_codes(bytes_038_7())) by {
        reveal(window_038_7);
    };
}

pub proof fn check_038_7()
    ensures
        scan::clean_range(
            data_038(),
            p::css_codes(),
            p::marketing_codes(),
            p::relative_codes(),
            1792,
            2048,
        ),
{
    hide(data_038);
    hide(scan::clean_range);
    hide(p::css_codes);
    hide(p::marketing_codes);
    hide(p::relative_codes);
    bind_038_7();
    p::bounds();
    assert(scan::clean_range(
        window_038_7(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        1,
        257,
    )) by (compute_only);
    scan::clean_window(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        1791,
        2080,
        1792,
        2048,
    );
}

} // verus!
    }
    pub mod window_8 {
        #[cfg(verus_keep_ghost)]
        use super::{data_038, shape_038};
        use crate::{
            k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
            k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
        };
        use ckc_spec::ui as u;
        use vstd::prelude::*;
        use vstd::utf8::*;
        verus! {

pub closed spec fn bytes_038_8() -> u::Bytes {
    ns![101u8,114,45,116,111,112,58,32,49,112,120,32,115,111,108,105,100,32,35,101,53,101,55,101,98,59,32,125,10,102,111,111,116,101,114,46,115,99,111,112,101,32,123,32,109,97,114,103,105,110,45,116,111,112,58,32,50,114,101,109,59,32,112,97,100,100,105,110,103,58,32,49,114,101,109,32,48,59,32,98,111,114,100,101,114,45,116,111,112,58,32,49,112,120,32,115,111,108,105,100,32,35,101,53,101,55,101,98,59,32,102,111,110,116,45,115,105,122,101,58,32,48,46,57,114,101,109,59,32,125,10,102,111,114,109,32,108,97,98,101,108,32,123,32,100,105,115,112,108,97,121,58,32,98,108,111,99,107,59,32,109,97,114,103,105,110,45,116,111,112,58,32,49,114,101,109,59,32,102,111,110,116,45,119,101,105,103,104,116,58,32,54,48,48,59,32,125,10,102,105,101,108,100,115,101,116,32,123,32,98,111,114,100,101,114,58,32,48,59,32,109,97,114,103,105,110,58,32,49,114,101,109,32,48,32,48,59,32,112,97,100,100,105,110,103,58,32,48,59,32,109,97,120,45,119,105,100,116,104,58,32,50,56,114,101,109,59,32,125,10,108,101,103,101,110,100,32,123,32,102,111,110,116,45,119,101,105,103,104]
}

pub closed spec fn window_038_8() -> Seq<nat> {
    scan::byte_codes(bytes_038_8())
}

pub proof fn bind_038_8()
    ensures
        data_038().len() == 10326,
        data_038().subrange(2047, 2336) == window_038_8(),
        window_038_8().len() == 289,
{
    hide(data_038);
    hide(ch::css_text_bytes);
    shape_038();
    assert(ch::css_text_bytes().subrange(2047, 2336) == bytes_038_8()) by (compute_only);
    assert(bytes_038_8().len() == 289) by (compute_only);
    scan::byte_codes_subrange(ch::css_text_bytes(), 2047, 2336);
    assert(data_038() == scan::byte_codes(ch::css_text_bytes())) by {
        reveal(data_038);
    };
    assert(window_038_8() == scan::byte_codes(bytes_038_8())) by {
        reveal(window_038_8);
    };
}

pub proof fn check_038_8()
    ensures
        scan::clean_range(
            data_038(),
            p::css_codes(),
            p::marketing_codes(),
            p::relative_codes(),
            2048,
            2304,
        ),
{
    hide(data_038);
    hide(scan::clean_range);
    hide(p::css_codes);
    hide(p::marketing_codes);
    hide(p::relative_codes);
    bind_038_8();
    p::bounds();
    assert(scan::clean_range(
        window_038_8(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        1,
        257,
    )) by (compute_only);
    scan::clean_window(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        2047,
        2336,
        2048,
        2304,
    );
}

} // verus!
    }
    pub mod window_9 {
        #[cfg(verus_keep_ghost)]
        use super::{data_038, shape_038};
        use crate::{
            k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
            k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
        };
        use ckc_spec::ui as u;
        use vstd::prelude::*;
        use vstd::utf8::*;
        verus! {

pub closed spec fn bytes_038_9() -> u::Bytes {
    ns![100u8,116,104,58,32,50,56,114,101,109,59,32,125,10,108,101,103,101,110,100,32,123,32,102,111,110,116,45,119,101,105,103,104,116,58,32,54,48,48,59,32,112,97,100,100,105,110,103,58,32,48,59,32,125,10,102,105,101,108,100,115,101,116,32,108,97,98,101,108,32,123,32,109,97,114,103,105,110,45,116,111,112,58,32,48,46,53,114,101,109,59,32,102,111,110,116,45,119,101,105,103,104,116,58,32,52,48,48,59,32,125,10,105,110,112,117,116,91,116,121,112,101,61,34,116,101,120,116,34,93,44,32,116,101,120,116,97,114,101,97,32,123,32,100,105,115,112,108,97,121,58,32,98,108,111,99,107,59,32,98,111,120,45,115,105,122,105,110,103,58,32,98,111,114,100,101,114,45,98,111,120,59,32,119,105,100,116,104,58,32,49,48,48,37,59,32,109,97,120,45,119,105,100,116,104,58,32,50,56,114,101,109,59,32,109,97,114,103,105,110,45,116,111,112,58,32,48,46,51,114,101,109,59,32,112,97,100,100,105,110,103,58,32,48,46,52,53,114,101,109,32,48,46,54,114,101,109,59,32,98,111,114,100,101,114,58,32,49,112,120,32,115,111,108,105,100,32,35,101,53,101,55,101,98,59,32,102,111,110]
}

pub closed spec fn window_038_9() -> Seq<nat> {
    scan::byte_codes(bytes_038_9())
}

pub proof fn bind_038_9()
    ensures
        data_038().len() == 10326,
        data_038().subrange(2303, 2592) == window_038_9(),
        window_038_9().len() == 289,
{
    hide(data_038);
    hide(ch::css_text_bytes);
    shape_038();
    assert(ch::css_text_bytes().subrange(2303, 2592) == bytes_038_9()) by (compute_only);
    assert(bytes_038_9().len() == 289) by (compute_only);
    scan::byte_codes_subrange(ch::css_text_bytes(), 2303, 2592);
    assert(data_038() == scan::byte_codes(ch::css_text_bytes())) by {
        reveal(data_038);
    };
    assert(window_038_9() == scan::byte_codes(bytes_038_9())) by {
        reveal(window_038_9);
    };
}

pub proof fn check_038_9()
    ensures
        scan::clean_range(
            data_038(),
            p::css_codes(),
            p::marketing_codes(),
            p::relative_codes(),
            2304,
            2560,
        ),
{
    hide(data_038);
    hide(scan::clean_range);
    hide(p::css_codes);
    hide(p::marketing_codes);
    hide(p::relative_codes);
    bind_038_9();
    p::bounds();
    assert(scan::clean_range(
        window_038_9(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        1,
        257,
    )) by (compute_only);
    scan::clean_window(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        2303,
        2592,
        2304,
        2560,
    );
}

} // verus!
    }
    pub mod window_10 {
        #[cfg(verus_keep_ghost)]
        use super::{data_038, shape_038};
        use crate::{
            k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
            k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
        };
        use ckc_spec::ui as u;
        use vstd::prelude::*;
        use vstd::utf8::*;
        verus! {

pub closed spec fn bytes_038_10() -> u::Bytes {
    ns![109u8,59,32,98,111,114,100,101,114,58,32,49,112,120,32,115,111,108,105,100,32,35,101,53,101,55,101,98,59,32,102,111,110,116,45,102,97,109,105,108,121,58,32,105,110,104,101,114,105,116,59,32,102,111,110,116,45,115,105,122,101,58,32,49,114,101,109,59,32,99,111,108,111,114,58,32,35,49,49,49,56,50,55,59,32,98,97,99,107,103,114,111,117,110,100,58,32,35,102,102,102,102,102,102,59,32,125,10,116,101,120,116,97,114,101,97,32,123,32,109,105,110,45,104,101,105,103,104,116,58,32,54,114,101,109,59,32,125,10,105,110,112,117,116,91,116,121,112,101,61,34,114,97,100,105,111,34,93,44,32,105,110,112,117,116,91,116,121,112,101,61,34,99,104,101,99,107,98,111,120,34,93,32,123,32,97,99,99,101,110,116,45,99,111,108,111,114,58,32,35,49,49,49,56,50,55,59,32,125,10,105,110,112,117,116,91,116,121,112,101,61,34,116,101,120,116,34,93,58,102,111,99,117,115,45,118,105,115,105,98,108,101,44,32,105,110,112,117,116,91,116,121,112,101,61,34,114,97,100,105,111,34,93,58,102,111,99,117,115,45,118,105,115,105,98,108,101,44,32,105,110,112,117,116,91,116,121,112,101]
}

pub closed spec fn window_038_10() -> Seq<nat> {
    scan::byte_codes(bytes_038_10())
}

pub proof fn bind_038_10()
    ensures
        data_038().len() == 10326,
        data_038().subrange(2559, 2848) == window_038_10(),
        window_038_10().len() == 289,
{
    hide(data_038);
    hide(ch::css_text_bytes);
    shape_038();
    assert(ch::css_text_bytes().subrange(2559, 2848) == bytes_038_10()) by (compute_only);
    assert(bytes_038_10().len() == 289) by (compute_only);
    scan::byte_codes_subrange(ch::css_text_bytes(), 2559, 2848);
    assert(data_038() == scan::byte_codes(ch::css_text_bytes())) by {
        reveal(data_038);
    };
    assert(window_038_10() == scan::byte_codes(bytes_038_10())) by {
        reveal(window_038_10);
    };
}

pub proof fn check_038_10()
    ensures
        scan::clean_range(
            data_038(),
            p::css_codes(),
            p::marketing_codes(),
            p::relative_codes(),
            2560,
            2816,
        ),
{
    hide(data_038);
    hide(scan::clean_range);
    hide(p::css_codes);
    hide(p::marketing_codes);
    hide(p::relative_codes);
    bind_038_10();
    p::bounds();
    assert(scan::clean_range(
        window_038_10(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        1,
        257,
    )) by (compute_only);
    scan::clean_window(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        2559,
        2848,
        2560,
        2816,
    );
}

} // verus!
    }
    pub mod window_11 {
        #[cfg(verus_keep_ghost)]
        use super::{data_038, shape_038};
        use crate::{
            k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
            k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
        };
        use ckc_spec::ui as u;
        use vstd::prelude::*;
        use vstd::utf8::*;
        verus! {

pub closed spec fn bytes_038_11() -> u::Bytes {
    ns![114u8,97,100,105,111,34,93,58,102,111,99,117,115,45,118,105,115,105,98,108,101,44,32,105,110,112,117,116,91,116,121,112,101,61,34,99,104,101,99,107,98,111,120,34,93,58,102,111,99,117,115,45,118,105,115,105,98,108,101,44,32,116,101,120,116,97,114,101,97,58,102,111,99,117,115,45,118,105,115,105,98,108,101,44,32,98,117,116,116,111,110,58,102,111,99,117,115,45,118,105,115,105,98,108,101,32,123,32,111,117,116,108,105,110,101,58,32,51,112,120,32,115,111,108,105,100,32,35,49,100,52,101,100,56,59,32,111,117,116,108,105,110,101,45,111,102,102,115,101,116,58,32,50,112,120,59,32,125,10,98,117,116,116,111,110,32,123,32,109,97,114,103,105,110,45,116,111,112,58,32,49,46,50,53,114,101,109,59,32,112,97,100,100,105,110,103,58,32,48,46,53,114,101,109,32,49,46,50,114,101,109,59,32,98,111,114,100,101,114,58,32,49,112,120,32,115,111,108,105,100,32,35,49,49,49,56,50,55,59,32,102,111,110,116,45,102,97,109,105,108,121,58,32,105,110,104,101,114,105,116,59,32,102,111,110,116,45,115,105,122,101,58,32,49,114,101,109,59,32,102,111,110,116,45,119,101,105,103,104]
}

pub closed spec fn window_038_11() -> Seq<nat> {
    scan::byte_codes(bytes_038_11())
}

pub proof fn bind_038_11()
    ensures
        data_038().len() == 10326,
        data_038().subrange(2815, 3104) == window_038_11(),
        window_038_11().len() == 289,
{
    hide(data_038);
    hide(ch::css_text_bytes);
    shape_038();
    assert(ch::css_text_bytes().subrange(2815, 3104) == bytes_038_11()) by (compute_only);
    assert(bytes_038_11().len() == 289) by (compute_only);
    scan::byte_codes_subrange(ch::css_text_bytes(), 2815, 3104);
    assert(data_038() == scan::byte_codes(ch::css_text_bytes())) by {
        reveal(data_038);
    };
    assert(window_038_11() == scan::byte_codes(bytes_038_11())) by {
        reveal(window_038_11);
    };
}

pub proof fn check_038_11()
    ensures
        scan::clean_range(
            data_038(),
            p::css_codes(),
            p::marketing_codes(),
            p::relative_codes(),
            2816,
            3072,
        ),
{
    hide(data_038);
    hide(scan::clean_range);
    hide(p::css_codes);
    hide(p::marketing_codes);
    hide(p::relative_codes);
    bind_038_11();
    p::bounds();
    assert(scan::clean_range(
        window_038_11(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        1,
        257,
    )) by (compute_only);
    scan::clean_window(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        2815,
        3104,
        2816,
        3072,
    );
}

} // verus!
    }
    pub mod window_12 {
        #[cfg(verus_keep_ghost)]
        use super::{data_038, shape_038};
        use crate::{
            k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
            k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
        };
        use ckc_spec::ui as u;
        use vstd::prelude::*;
        use vstd::utf8::*;
        verus! {

pub closed spec fn bytes_038_12() -> u::Bytes {
    ns![101u8,114,105,116,59,32,102,111,110,116,45,115,105,122,101,58,32,49,114,101,109,59,32,102,111,110,116,45,119,101,105,103,104,116,58,32,54,48,48,59,32,99,111,108,111,114,58,32,35,102,102,102,102,102,102,59,32,98,97,99,107,103,114,111,117,110,100,58,32,35,49,49,49,56,50,55,59,32,125,10,109,97,114,107,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,100,98,101,97,102,101,59,32,99,111,108,111,114,58,32,105,110,104,101,114,105,116,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,58,32,117,110,100,101,114,108,105,110,101,32,100,111,116,116,101,100,32,35,52,98,53,53,54,51,59,32,116,101,120,116,45,117,110,100,101,114,108,105,110,101,45,111,102,102,115,101,116,58,32,48,46,49,53,101,109,59,32,125,10,46,107,119,32,123,32,99,111,108,111,114,58,32,35,52,98,53,53,54,51,59,32,125,10,46,104,108,45,110,111,116,101,32,123,32,99,111,108,111,114,58,32,35,52,98,53,53,54,51,59,32,102,111,110,116,45,115,105,122,101,58,32,48,46,57,114,101,109,59,32,125,10,109,97,114,107,46,116,49,44,32,109,97,114,107,46,116,49,51]
}

pub closed spec fn window_038_12() -> Seq<nat> {
    scan::byte_codes(bytes_038_12())
}

pub proof fn bind_038_12()
    ensures
        data_038().len() == 10326,
        data_038().subrange(3071, 3360) == window_038_12(),
        window_038_12().len() == 289,
{
    hide(data_038);
    hide(ch::css_text_bytes);
    shape_038();
    assert(ch::css_text_bytes().subrange(3071, 3360) == bytes_038_12()) by (compute_only);
    assert(bytes_038_12().len() == 289) by (compute_only);
    scan::byte_codes_subrange(ch::css_text_bytes(), 3071, 3360);
    assert(data_038() == scan::byte_codes(ch::css_text_bytes())) by {
        reveal(data_038);
    };
    assert(window_038_12() == scan::byte_codes(bytes_038_12())) by {
        reveal(window_038_12);
    };
}

pub proof fn check_038_12()
    ensures
        scan::clean_range(
            data_038(),
            p::css_codes(),
            p::marketing_codes(),
            p::relative_codes(),
            3072,
            3328,
        ),
{
    hide(data_038);
    hide(scan::clean_range);
    hide(p::css_codes);
    hide(p::marketing_codes);
    hide(p::relative_codes);
    bind_038_12();
    p::bounds();
    assert(scan::clean_range(
        window_038_12(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        1,
        257,
    )) by (compute_only);
    scan::clean_window(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        3071,
        3360,
        3072,
        3328,
    );
}

} // verus!
    }
    pub mod window_13 {
        #[cfg(verus_keep_ghost)]
        use super::{data_038, shape_038};
        use crate::{
            k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
            k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
        };
        use ckc_spec::ui as u;
        use vstd::prelude::*;
        use vstd::utf8::*;
        verus! {

pub closed spec fn bytes_038_13() -> u::Bytes {
    ns![115u8,105,122,101,58,32,48,46,57,114,101,109,59,32,125,10,109,97,114,107,46,116,49,44,32,109,97,114,107,46,116,49,51,44,32,109,97,114,107,46,116,50,53,44,32,109,97,114,107,46,116,51,55,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,102,101,102,57,99,51,59,32,125,10,109,97,114,107,46,116,50,44,32,109,97,114,107,46,116,49,52,44,32,109,97,114,107,46,116,50,54,44,32,109,97,114,107,46,116,51,56,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,102,51,101,56,102,102,59,32,125,10,109,97,114,107,46,116,51,44,32,109,97,114,107,46,116,49,53,44,32,109,97,114,107,46,116,50,55,44,32,109,97,114,107,46,116,51,57,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,102,102,101,100,100,53,59,32,125,10,109,97,114,107,46,116,52,44,32,109,97,114,107,46,116,49,54,44,32,109,97,114,107,46,116,50,56,44,32,109,97,114,107,46,116,52,48,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,99,99,102,98,102,49,59,32,125,10,109,97,114,107,46,116,53,44,32,109,97,114,107,46,116,49,55,44,32,109,97]
}

pub closed spec fn window_038_13() -> Seq<nat> {
    scan::byte_codes(bytes_038_13())
}

pub proof fn bind_038_13()
    ensures
        data_038().len() == 10326,
        data_038().subrange(3327, 3616) == window_038_13(),
        window_038_13().len() == 289,
{
    hide(data_038);
    hide(ch::css_text_bytes);
    shape_038();
    assert(ch::css_text_bytes().subrange(3327, 3616) == bytes_038_13()) by (compute_only);
    assert(bytes_038_13().len() == 289) by (compute_only);
    scan::byte_codes_subrange(ch::css_text_bytes(), 3327, 3616);
    assert(data_038() == scan::byte_codes(ch::css_text_bytes())) by {
        reveal(data_038);
    };
    assert(window_038_13() == scan::byte_codes(bytes_038_13())) by {
        reveal(window_038_13);
    };
}

pub proof fn check_038_13()
    ensures
        scan::clean_range(
            data_038(),
            p::css_codes(),
            p::marketing_codes(),
            p::relative_codes(),
            3328,
            3584,
        ),
{
    hide(data_038);
    hide(scan::clean_range);
    hide(p::css_codes);
    hide(p::marketing_codes);
    hide(p::relative_codes);
    bind_038_13();
    p::bounds();
    assert(scan::clean_range(
        window_038_13(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        1,
        257,
    )) by (compute_only);
    scan::clean_window(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        3327,
        3616,
        3328,
        3584,
    );
}

} // verus!
    }
    pub mod window_14 {
        #[cfg(verus_keep_ghost)]
        use super::{data_038, shape_038};
        use crate::{
            k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
            k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
        };
        use ckc_spec::ui as u;
        use vstd::prelude::*;
        use vstd::utf8::*;
        verus! {

pub closed spec fn bytes_038_14() -> u::Bytes {
    ns![32u8,35,99,99,102,98,102,49,59,32,125,10,109,97,114,107,46,116,53,44,32,109,97,114,107,46,116,49,55,44,32,109,97,114,107,46,116,50,57,44,32,109,97,114,107,46,116,52,49,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,102,102,101,52,101,54,59,32,125,10,109,97,114,107,46,116,54,44,32,109,97,114,107,46,116,49,56,44,32,109,97,114,107,46,116,51,48,44,32,109,97,114,107,46,116,52,50,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,100,99,102,99,101,55,59,32,125,10,109,97,114,107,46,116,55,44,32,109,97,114,107,46,116,49,57,44,32,109,97,114,107,46,116,51,49,44,32,109,97,114,107,46,116,52,51,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,102,97,101,56,102,102,59,32,125,10,109,97,114,107,46,116,56,44,32,109,97,114,107,46,116,50,48,44,32,109,97,114,107,46,116,51,50,44,32,109,97,114,107,46,116,52,52,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,99,102,102,97,102,101,59,32,125,10,109,97,114,107,46,116,57,44,32,109,97,114,107,46,116,50,49,44,32,109,97,114,107,46,116]
}

pub closed spec fn window_038_14() -> Seq<nat> {
    scan::byte_codes(bytes_038_14())
}

pub proof fn bind_038_14()
    ensures
        data_038().len() == 10326,
        data_038().subrange(3583, 3872) == window_038_14(),
        window_038_14().len() == 289,
{
    hide(data_038);
    hide(ch::css_text_bytes);
    shape_038();
    assert(ch::css_text_bytes().subrange(3583, 3872) == bytes_038_14()) by (compute_only);
    assert(bytes_038_14().len() == 289) by (compute_only);
    scan::byte_codes_subrange(ch::css_text_bytes(), 3583, 3872);
    assert(data_038() == scan::byte_codes(ch::css_text_bytes())) by {
        reveal(data_038);
    };
    assert(window_038_14() == scan::byte_codes(bytes_038_14())) by {
        reveal(window_038_14);
    };
}

pub proof fn check_038_14()
    ensures
        scan::clean_range(
            data_038(),
            p::css_codes(),
            p::marketing_codes(),
            p::relative_codes(),
            3584,
            3840,
        ),
{
    hide(data_038);
    hide(scan::clean_range);
    hide(p::css_codes);
    hide(p::marketing_codes);
    hide(p::relative_codes);
    bind_038_14();
    p::bounds();
    assert(scan::clean_range(
        window_038_14(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        1,
        257,
    )) by (compute_only);
    scan::clean_window(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        3583,
        3872,
        3584,
        3840,
    );
}

} // verus!
    }
    pub mod window_15 {
        #[cfg(verus_keep_ghost)]
        use super::{data_038, shape_038};
        use crate::{
            k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
            k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
        };
        use ckc_spec::ui as u;
        use vstd::prelude::*;
        use vstd::utf8::*;
        verus! {

pub closed spec fn bytes_038_15() -> u::Bytes {
    ns![102u8,97,102,101,59,32,125,10,109,97,114,107,46,116,57,44,32,109,97,114,107,46,116,50,49,44,32,109,97,114,107,46,116,51,51,44,32,109,97,114,107,46,116,52,53,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,101,99,102,99,99,98,59,32,125,10,109,97,114,107,46,116,49,48,44,32,109,97,114,107,46,116,50,50,44,32,109,97,114,107,46,116,51,52,44,32,109,97,114,107,46,116,52,54,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,101,48,101,55,102,102,59,32,125,10,109,97,114,107,46,116,49,49,44,32,109,97,114,107,46,116,50,51,44,32,109,97,114,107,46,116,51,53,44,32,109,97,114,107,46,116,52,55,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,101,55,101,53,101,52,59,32,125,10,109,97,114,107,46,116,48,44,32,109,97,114,107,46,116,49,50,44,32,109,97,114,107,46,116,50,52,44,32,109,97,114,107,46,116,51,54,32,123,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,99,111,108,111,114,58,32,35,50,53,54,51,101,98,59,32,125,10,109,97,114,107,46,116,49,44,32,109,97,114,107,46,116,49]
}

pub closed spec fn window_038_15() -> Seq<nat> {
    scan::byte_codes(bytes_038_15())
}

pub proof fn bind_038_15()
    ensures
        data_038().len() == 10326,
        data_038().subrange(3839, 4128) == window_038_15(),
        window_038_15().len() == 289,
{
    hide(data_038);
    hide(ch::css_text_bytes);
    shape_038();
    assert(ch::css_text_bytes().subrange(3839, 4128) == bytes_038_15()) by (compute_only);
    assert(bytes_038_15().len() == 289) by (compute_only);
    scan::byte_codes_subrange(ch::css_text_bytes(), 3839, 4128);
    assert(data_038() == scan::byte_codes(ch::css_text_bytes())) by {
        reveal(data_038);
    };
    assert(window_038_15() == scan::byte_codes(bytes_038_15())) by {
        reveal(window_038_15);
    };
}

pub proof fn check_038_15()
    ensures
        scan::clean_range(
            data_038(),
            p::css_codes(),
            p::marketing_codes(),
            p::relative_codes(),
            3840,
            4096,
        ),
{
    hide(data_038);
    hide(scan::clean_range);
    hide(p::css_codes);
    hide(p::marketing_codes);
    hide(p::relative_codes);
    bind_038_15();
    p::bounds();
    assert(scan::clean_range(
        window_038_15(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        1,
        257,
    )) by (compute_only);
    scan::clean_window(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        3839,
        4128,
        3840,
        4096,
    );
}

} // verus!
    }
    pub mod window_16 {
        #[cfg(verus_keep_ghost)]
        use super::{data_038, shape_038};
        use crate::{
            k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
            k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
        };
        use ckc_spec::ui as u;
        use vstd::prelude::*;
        use vstd::utf8::*;
        verus! {

pub closed spec fn bytes_038_16() -> u::Bytes {
    ns![111u8,108,111,114,58,32,35,50,53,54,51,101,98,59,32,125,10,109,97,114,107,46,116,49,44,32,109,97,114,107,46,116,49,51,44,32,109,97,114,107,46,116,50,53,44,32,109,97,114,107,46,116,51,55,32,123,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,99,111,108,111,114,58,32,35,97,49,54,50,48,55,59,32,125,10,109,97,114,107,46,116,50,44,32,109,97,114,107,46,116,49,52,44,32,109,97,114,107,46,116,50,54,44,32,109,97,114,107,46,116,51,56,32,123,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,99,111,108,111,114,58,32,35,55,99,51,97,101,100,59,32,125,10,109,97,114,107,46,116,51,44,32,109,97,114,107,46,116,49,53,44,32,109,97,114,107,46,116,50,55,44,32,109,97,114,107,46,116,51,57,32,123,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,99,111,108,111,114,58,32,35,99,50,52,49,48,99,59,32,125,10,109,97,114,107,46,116,52,44,32,109,97,114,107,46,116,49,54,44,32,109,97,114,107,46,116,50,56,44,32,109,97,114,107,46,116,52,48,32,123,32,116,101,120,116,45,100,101,99,111,114]
}

pub closed spec fn window_038_16() -> Seq<nat> {
    scan::byte_codes(bytes_038_16())
}

pub proof fn bind_038_16()
    ensures
        data_038().len() == 10326,
        data_038().subrange(4095, 4384) == window_038_16(),
        window_038_16().len() == 289,
{
    hide(data_038);
    hide(ch::css_text_bytes);
    shape_038();
    assert(ch::css_text_bytes().subrange(4095, 4384) == bytes_038_16()) by (compute_only);
    assert(bytes_038_16().len() == 289) by (compute_only);
    scan::byte_codes_subrange(ch::css_text_bytes(), 4095, 4384);
    assert(data_038() == scan::byte_codes(ch::css_text_bytes())) by {
        reveal(data_038);
    };
    assert(window_038_16() == scan::byte_codes(bytes_038_16())) by {
        reveal(window_038_16);
    };
}

pub proof fn check_038_16()
    ensures
        scan::clean_range(
            data_038(),
            p::css_codes(),
            p::marketing_codes(),
            p::relative_codes(),
            4096,
            4352,
        ),
{
    hide(data_038);
    hide(scan::clean_range);
    hide(p::css_codes);
    hide(p::marketing_codes);
    hide(p::relative_codes);
    bind_038_16();
    p::bounds();
    assert(scan::clean_range(
        window_038_16(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        1,
        257,
    )) by (compute_only);
    scan::clean_window(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        4095,
        4384,
        4096,
        4352,
    );
}

} // verus!
    }
    pub mod window_17 {
        #[cfg(verus_keep_ghost)]
        use super::{data_038, shape_038};
        use crate::{
            k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
            k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
        };
        use ckc_spec::ui as u;
        use vstd::prelude::*;
        use vstd::utf8::*;
        verus! {

pub closed spec fn bytes_038_17() -> u::Bytes {
    ns![44u8,32,109,97,114,107,46,116,50,56,44,32,109,97,114,107,46,116,52,48,32,123,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,99,111,108,111,114,58,32,35,48,102,55,54,54,101,59,32,125,10,109,97,114,107,46,116,53,44,32,109,97,114,107,46,116,49,55,44,32,109,97,114,107,46,116,50,57,44,32,109,97,114,107,46,116,52,49,32,123,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,99,111,108,111,114,58,32,35,98,101,49,50,51,99,59,32,125,10,109,97,114,107,46,116,54,44,32,109,97,114,107,46,116,49,56,44,32,109,97,114,107,46,116,51,48,44,32,109,97,114,107,46,116,52,50,32,123,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,99,111,108,111,114,58,32,35,49,53,56,48,51,100,59,32,125,10,109,97,114,107,46,116,55,44,32,109,97,114,107,46,116,49,57,44,32,109,97,114,107,46,116,51,49,44,32,109,97,114,107,46,116,52,51,32,123,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,99,111,108,111,114,58,32,35,97,50,49,99,97,102,59,32,125,10,109,97,114,107,46,116,56,44,32,109]
}

pub closed spec fn window_038_17() -> Seq<nat> {
    scan::byte_codes(bytes_038_17())
}

pub proof fn bind_038_17()
    ensures
        data_038().len() == 10326,
        data_038().subrange(4351, 4640) == window_038_17(),
        window_038_17().len() == 289,
{
    hide(data_038);
    hide(ch::css_text_bytes);
    shape_038();
    assert(ch::css_text_bytes().subrange(4351, 4640) == bytes_038_17()) by (compute_only);
    assert(bytes_038_17().len() == 289) by (compute_only);
    scan::byte_codes_subrange(ch::css_text_bytes(), 4351, 4640);
    assert(data_038() == scan::byte_codes(ch::css_text_bytes())) by {
        reveal(data_038);
    };
    assert(window_038_17() == scan::byte_codes(bytes_038_17())) by {
        reveal(window_038_17);
    };
}

pub proof fn check_038_17()
    ensures
        scan::clean_range(
            data_038(),
            p::css_codes(),
            p::marketing_codes(),
            p::relative_codes(),
            4352,
            4608,
        ),
{
    hide(data_038);
    hide(scan::clean_range);
    hide(p::css_codes);
    hide(p::marketing_codes);
    hide(p::relative_codes);
    bind_038_17();
    p::bounds();
    assert(scan::clean_range(
        window_038_17(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        1,
        257,
    )) by (compute_only);
    scan::clean_window(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        4351,
        4640,
        4352,
        4608,
    );
}

} // verus!
    }
    pub mod window_18 {
        #[cfg(verus_keep_ghost)]
        use super::{data_038, shape_038};
        use crate::{
            k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
            k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
        };
        use ckc_spec::ui as u;
        use vstd::prelude::*;
        use vstd::utf8::*;
        verus! {

pub closed spec fn bytes_038_18() -> u::Bytes {
    ns![116u8,105,111,110,45,99,111,108,111,114,58,32,35,97,50,49,99,97,102,59,32,125,10,109,97,114,107,46,116,56,44,32,109,97,114,107,46,116,50,48,44,32,109,97,114,107,46,116,51,50,44,32,109,97,114,107,46,116,52,52,32,123,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,99,111,108,111,114,58,32,35,48,101,55,52,57,48,59,32,125,10,109,97,114,107,46,116,57,44,32,109,97,114,107,46,116,50,49,44,32,109,97,114,107,46,116,51,51,44,32,109,97,114,107,46,116,52,53,32,123,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,99,111,108,111,114,58,32,35,52,100,55,99,48,102,59,32,125,10,109,97,114,107,46,116,49,48,44,32,109,97,114,107,46,116,50,50,44,32,109,97,114,107,46,116,51,52,44,32,109,97,114,107,46,116,52,54,32,123,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,99,111,108,111,114,58,32,35,52,102,52,54,101,53,59,32,125,10,109,97,114,107,46,116,49,49,44,32,109,97,114,107,46,116,50,51,44,32,109,97,114,107,46,116,51,53,44,32,109,97,114,107,46,116,52,55,32,123,32,116,101]
}

pub closed spec fn window_038_18() -> Seq<nat> {
    scan::byte_codes(bytes_038_18())
}

pub proof fn bind_038_18()
    ensures
        data_038().len() == 10326,
        data_038().subrange(4607, 4896) == window_038_18(),
        window_038_18().len() == 289,
{
    hide(data_038);
    hide(ch::css_text_bytes);
    shape_038();
    assert(ch::css_text_bytes().subrange(4607, 4896) == bytes_038_18()) by (compute_only);
    assert(bytes_038_18().len() == 289) by (compute_only);
    scan::byte_codes_subrange(ch::css_text_bytes(), 4607, 4896);
    assert(data_038() == scan::byte_codes(ch::css_text_bytes())) by {
        reveal(data_038);
    };
    assert(window_038_18() == scan::byte_codes(bytes_038_18())) by {
        reveal(window_038_18);
    };
}

pub proof fn check_038_18()
    ensures
        scan::clean_range(
            data_038(),
            p::css_codes(),
            p::marketing_codes(),
            p::relative_codes(),
            4608,
            4864,
        ),
{
    hide(data_038);
    hide(scan::clean_range);
    hide(p::css_codes);
    hide(p::marketing_codes);
    hide(p::relative_codes);
    bind_038_18();
    p::bounds();
    assert(scan::clean_range(
        window_038_18(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        1,
        257,
    )) by (compute_only);
    scan::clean_window(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        4607,
        4896,
        4608,
        4864,
    );
}

} // verus!
    }
    pub mod window_19 {
        #[cfg(verus_keep_ghost)]
        use super::{data_038, shape_038};
        use crate::{
            k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
            k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
        };
        use ckc_spec::ui as u;
        use vstd::prelude::*;
        use vstd::utf8::*;
        verus! {

pub closed spec fn bytes_038_19() -> u::Bytes {
    ns![109u8,97,114,107,46,116,50,51,44,32,109,97,114,107,46,116,51,53,44,32,109,97,114,107,46,116,52,55,32,123,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,99,111,108,111,114,58,32,35,53,55,53,51,52,101,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97,114,107,46,116,48,58,104,111,118,101,114,41,32,109,97,114,107,46,116,48,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,98,102,100,98,102,101,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97,114,107,46,116,49,58,104,111,118,101,114,41,32,109,97,114,107,46,116,49,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,102,101,102,48,56,97,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97,114,107,46,116,50,58,104,111,118,101,114,41,32,109,97,114,107,46,116,50,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,101,57,100]
}

pub closed spec fn window_038_19() -> Seq<nat> {
    scan::byte_codes(bytes_038_19())
}

pub proof fn bind_038_19()
    ensures
        data_038().len() == 10326,
        data_038().subrange(4863, 5152) == window_038_19(),
        window_038_19().len() == 289,
{
    hide(data_038);
    hide(ch::css_text_bytes);
    shape_038();
    assert(ch::css_text_bytes().subrange(4863, 5152) == bytes_038_19()) by (compute_only);
    assert(bytes_038_19().len() == 289) by (compute_only);
    scan::byte_codes_subrange(ch::css_text_bytes(), 4863, 5152);
    assert(data_038() == scan::byte_codes(ch::css_text_bytes())) by {
        reveal(data_038);
    };
    assert(window_038_19() == scan::byte_codes(bytes_038_19())) by {
        reveal(window_038_19);
    };
}

pub proof fn check_038_19()
    ensures
        scan::clean_range(
            data_038(),
            p::css_codes(),
            p::marketing_codes(),
            p::relative_codes(),
            4864,
            5120,
        ),
{
    hide(data_038);
    hide(scan::clean_range);
    hide(p::css_codes);
    hide(p::marketing_codes);
    hide(p::relative_codes);
    bind_038_19();
    p::bounds();
    assert(scan::clean_range(
        window_038_19(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        1,
        257,
    )) by (compute_only);
    scan::clean_window(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        4863,
        5152,
        4864,
        5120,
    );
}

} // verus!
    }
    pub mod window_20 {
        #[cfg(verus_keep_ghost)]
        use super::{data_038, shape_038};
        use crate::{
            k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
            k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
        };
        use ckc_spec::ui as u;
        use vstd::prelude::*;
        use vstd::utf8::*;
        verus! {

pub closed spec fn bytes_038_20() -> u::Bytes {
    ns![104u8,111,118,101,114,41,32,109,97,114,107,46,116,50,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,101,57,100,53,102,102,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97,114,107,46,116,51,58,104,111,118,101,114,41,32,109,97,114,107,46,116,51,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,102,101,100,55,97,97,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97,114,107,46,116,52,58,104,111,118,101,114,41,32,109,97,114,107,46,116,52,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,57,57,102,54,101,52,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97,114,107,46,116,53,58,104,111,118,101,114,41,32,109,97,114,107,46,116,53,32,123,32,98,97,99,107,103,114,111,117,110,100,58]
}

pub closed spec fn window_038_20() -> Seq<nat> {
    scan::byte_codes(bytes_038_20())
}

pub proof fn bind_038_20()
    ensures
        data_038().len() == 10326,
        data_038().subrange(5119, 5408) == window_038_20(),
        window_038_20().len() == 289,
{
    hide(data_038);
    hide(ch::css_text_bytes);
    shape_038();
    assert(ch::css_text_bytes().subrange(5119, 5408) == bytes_038_20()) by (compute_only);
    assert(bytes_038_20().len() == 289) by (compute_only);
    scan::byte_codes_subrange(ch::css_text_bytes(), 5119, 5408);
    assert(data_038() == scan::byte_codes(ch::css_text_bytes())) by {
        reveal(data_038);
    };
    assert(window_038_20() == scan::byte_codes(bytes_038_20())) by {
        reveal(window_038_20);
    };
}

pub proof fn check_038_20()
    ensures
        scan::clean_range(
            data_038(),
            p::css_codes(),
            p::marketing_codes(),
            p::relative_codes(),
            5120,
            5376,
        ),
{
    hide(data_038);
    hide(scan::clean_range);
    hide(p::css_codes);
    hide(p::marketing_codes);
    hide(p::relative_codes);
    bind_038_20();
    p::bounds();
    assert(scan::clean_range(
        window_038_20(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        1,
        257,
    )) by (compute_only);
    scan::clean_window(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        5119,
        5408,
        5120,
        5376,
    );
}

} // verus!
    }
    pub mod window_21 {
        #[cfg(verus_keep_ghost)]
        use super::{data_038, shape_038};
        use crate::{
            k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
            k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
        };
        use ckc_spec::ui as u;
        use vstd::prelude::*;
        use vstd::utf8::*;
        verus! {

pub closed spec fn bytes_038_21() -> u::Bytes {
    ns![107u8,46,116,53,58,104,111,118,101,114,41,32,109,97,114,107,46,116,53,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,102,101,99,100,100,51,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97,114,107,46,116,54,58,104,111,118,101,114,41,32,109,97,114,107,46,116,54,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,98,98,102,55,100,48,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97,114,107,46,116,55,58,104,111,118,101,114,41,32,109,97,114,107,46,116,55,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,102,53,100,48,102,101,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97,114,107,46,116,56,58,104,111,118,101,114,41,32,109,97,114,107,46,116,56,32,123,32,98,97,99,107,103,114]
}

pub closed spec fn window_038_21() -> Seq<nat> {
    scan::byte_codes(bytes_038_21())
}

pub proof fn bind_038_21()
    ensures
        data_038().len() == 10326,
        data_038().subrange(5375, 5664) == window_038_21(),
        window_038_21().len() == 289,
{
    hide(data_038);
    hide(ch::css_text_bytes);
    shape_038();
    assert(ch::css_text_bytes().subrange(5375, 5664) == bytes_038_21()) by (compute_only);
    assert(bytes_038_21().len() == 289) by (compute_only);
    scan::byte_codes_subrange(ch::css_text_bytes(), 5375, 5664);
    assert(data_038() == scan::byte_codes(ch::css_text_bytes())) by {
        reveal(data_038);
    };
    assert(window_038_21() == scan::byte_codes(bytes_038_21())) by {
        reveal(window_038_21);
    };
}

pub proof fn check_038_21()
    ensures
        scan::clean_range(
            data_038(),
            p::css_codes(),
            p::marketing_codes(),
            p::relative_codes(),
            5376,
            5632,
        ),
{
    hide(data_038);
    hide(scan::clean_range);
    hide(p::css_codes);
    hide(p::marketing_codes);
    hide(p::relative_codes);
    bind_038_21();
    p::bounds();
    assert(scan::clean_range(
        window_038_21(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        1,
        257,
    )) by (compute_only);
    scan::clean_window(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        5375,
        5664,
        5376,
        5632,
    );
}

} // verus!
    }
    pub mod window_22 {
        #[cfg(verus_keep_ghost)]
        use super::{data_038, shape_038};
        use crate::{
            k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
            k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
        };
        use ckc_spec::ui as u;
        use vstd::prelude::*;
        use vstd::utf8::*;
        verus! {

pub closed spec fn bytes_038_22() -> u::Bytes {
    ns![115u8,40,109,97,114,107,46,116,56,58,104,111,118,101,114,41,32,109,97,114,107,46,116,56,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,97,53,102,51,102,99,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97,114,107,46,116,57,58,104,111,118,101,114,41,32,109,97,114,107,46,116,57,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,100,57,102,57,57,100,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97,114,107,46,116,49,48,58,104,111,118,101,114,41,32,109,97,114,107,46,116,49,48,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,99,55,100,50,102,101,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97,114,107,46,116,49,49,58,104,111,118,101,114,41,32,109,97,114,107,46,116,49,49]
}

pub closed spec fn window_038_22() -> Seq<nat> {
    scan::byte_codes(bytes_038_22())
}

pub proof fn bind_038_22()
    ensures
        data_038().len() == 10326,
        data_038().subrange(5631, 5920) == window_038_22(),
        window_038_22().len() == 289,
{
    hide(data_038);
    hide(ch::css_text_bytes);
    shape_038();
    assert(ch::css_text_bytes().subrange(5631, 5920) == bytes_038_22()) by (compute_only);
    assert(bytes_038_22().len() == 289) by (compute_only);
    scan::byte_codes_subrange(ch::css_text_bytes(), 5631, 5920);
    assert(data_038() == scan::byte_codes(ch::css_text_bytes())) by {
        reveal(data_038);
    };
    assert(window_038_22() == scan::byte_codes(bytes_038_22())) by {
        reveal(window_038_22);
    };
}

pub proof fn check_038_22()
    ensures
        scan::clean_range(
            data_038(),
            p::css_codes(),
            p::marketing_codes(),
            p::relative_codes(),
            5632,
            5888,
        ),
{
    hide(data_038);
    hide(scan::clean_range);
    hide(p::css_codes);
    hide(p::marketing_codes);
    hide(p::relative_codes);
    bind_038_22();
    p::bounds();
    assert(scan::clean_range(
        window_038_22(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        1,
        257,
    )) by (compute_only);
    scan::clean_window(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        5631,
        5920,
        5632,
        5888,
    );
}

} // verus!
    }
    pub mod window_23 {
        #[cfg(verus_keep_ghost)]
        use super::{data_038, shape_038};
        use crate::{
            k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
            k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
        };
        use ckc_spec::ui as u;
        use vstd::prelude::*;
        use vstd::utf8::*;
        verus! {

pub closed spec fn bytes_038_23() -> u::Bytes {
    ns![109u8,97,105,110,58,104,97,115,40,109,97,114,107,46,116,49,49,58,104,111,118,101,114,41,32,109,97,114,107,46,116,49,49,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,100,54,100,51,100,49,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97,114,107,46,116,49,50,58,104,111,118,101,114,41,32,109,97,114,107,46,116,49,50,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,98,102,100,98,102,101,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97,114,107,46,116,49,51,58,104,111,118,101,114,41,32,109,97,114,107,46,116,49,51,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,102,101,102,48,56,97,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97,114,107,46,116,49,52,58,104,111,118,101]
}

pub closed spec fn window_038_23() -> Seq<nat> {
    scan::byte_codes(bytes_038_23())
}

pub proof fn bind_038_23()
    ensures
        data_038().len() == 10326,
        data_038().subrange(5887, 6176) == window_038_23(),
        window_038_23().len() == 289,
{
    hide(data_038);
    hide(ch::css_text_bytes);
    shape_038();
    assert(ch::css_text_bytes().subrange(5887, 6176) == bytes_038_23()) by (compute_only);
    assert(bytes_038_23().len() == 289) by (compute_only);
    scan::byte_codes_subrange(ch::css_text_bytes(), 5887, 6176);
    assert(data_038() == scan::byte_codes(ch::css_text_bytes())) by {
        reveal(data_038);
    };
    assert(window_038_23() == scan::byte_codes(bytes_038_23())) by {
        reveal(window_038_23);
    };
}

pub proof fn check_038_23()
    ensures
        scan::clean_range(
            data_038(),
            p::css_codes(),
            p::marketing_codes(),
            p::relative_codes(),
            5888,
            6144,
        ),
{
    hide(data_038);
    hide(scan::clean_range);
    hide(p::css_codes);
    hide(p::marketing_codes);
    hide(p::relative_codes);
    bind_038_23();
    p::bounds();
    assert(scan::clean_range(
        window_038_23(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        1,
        257,
    )) by (compute_only);
    scan::clean_window(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        5887,
        6176,
        5888,
        6144,
    );
}

} // verus!
    }
    pub mod window_24 {
        #[cfg(verus_keep_ghost)]
        use super::{data_038, shape_038};
        use crate::{
            k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
            k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
        };
        use ckc_spec::ui as u;
        use vstd::prelude::*;
        use vstd::utf8::*;
        verus! {

pub closed spec fn bytes_038_24() -> u::Bytes {
    ns![58u8,32,115,111,108,105,100,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97,114,107,46,116,49,52,58,104,111,118,101,114,41,32,109,97,114,107,46,116,49,52,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,101,57,100,53,102,102,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97,114,107,46,116,49,53,58,104,111,118,101,114,41,32,109,97,114,107,46,116,49,53,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,102,101,100,55,97,97,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97,114,107,46,116,49,54,58,104,111,118,101,114,41,32,109,97,114,107,46,116,49,54,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,57,57,102,54,101,52,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97]
}

pub closed spec fn window_038_24() -> Seq<nat> {
    scan::byte_codes(bytes_038_24())
}

pub proof fn bind_038_24()
    ensures
        data_038().len() == 10326,
        data_038().subrange(6143, 6432) == window_038_24(),
        window_038_24().len() == 289,
{
    hide(data_038);
    hide(ch::css_text_bytes);
    shape_038();
    assert(ch::css_text_bytes().subrange(6143, 6432) == bytes_038_24()) by (compute_only);
    assert(bytes_038_24().len() == 289) by (compute_only);
    scan::byte_codes_subrange(ch::css_text_bytes(), 6143, 6432);
    assert(data_038() == scan::byte_codes(ch::css_text_bytes())) by {
        reveal(data_038);
    };
    assert(window_038_24() == scan::byte_codes(bytes_038_24())) by {
        reveal(window_038_24);
    };
}

pub proof fn check_038_24()
    ensures
        scan::clean_range(
            data_038(),
            p::css_codes(),
            p::marketing_codes(),
            p::relative_codes(),
            6144,
            6400,
        ),
{
    hide(data_038);
    hide(scan::clean_range);
    hide(p::css_codes);
    hide(p::marketing_codes);
    hide(p::relative_codes);
    bind_038_24();
    p::bounds();
    assert(scan::clean_range(
        window_038_24(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        1,
        257,
    )) by (compute_only);
    scan::clean_window(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        6143,
        6432,
        6144,
        6400,
    );
}

} // verus!
    }
    pub mod window_25 {
        #[cfg(verus_keep_ghost)]
        use super::{data_038, shape_038};
        use crate::{
            k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
            k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
        };
        use ckc_spec::ui as u;
        use vstd::prelude::*;
        use vstd::utf8::*;
        verus! {

pub closed spec fn bytes_038_25() -> u::Bytes {
    ns![97u8,116,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97,114,107,46,116,49,55,58,104,111,118,101,114,41,32,109,97,114,107,46,116,49,55,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,102,101,99,100,100,51,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97,114,107,46,116,49,56,58,104,111,118,101,114,41,32,109,97,114,107,46,116,49,56,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,98,98,102,55,100,48,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97,114,107,46,116,49,57,58,104,111,118,101,114,41,32,109,97,114,107,46,116,49,57,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,102,53,100,48,102,101,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10]
}

pub closed spec fn window_038_25() -> Seq<nat> {
    scan::byte_codes(bytes_038_25())
}

pub proof fn bind_038_25()
    ensures
        data_038().len() == 10326,
        data_038().subrange(6399, 6688) == window_038_25(),
        window_038_25().len() == 289,
{
    hide(data_038);
    hide(ch::css_text_bytes);
    shape_038();
    assert(ch::css_text_bytes().subrange(6399, 6688) == bytes_038_25()) by (compute_only);
    assert(bytes_038_25().len() == 289) by (compute_only);
    scan::byte_codes_subrange(ch::css_text_bytes(), 6399, 6688);
    assert(data_038() == scan::byte_codes(ch::css_text_bytes())) by {
        reveal(data_038);
    };
    assert(window_038_25() == scan::byte_codes(bytes_038_25())) by {
        reveal(window_038_25);
    };
}

pub proof fn check_038_25()
    ensures
        scan::clean_range(
            data_038(),
            p::css_codes(),
            p::marketing_codes(),
            p::relative_codes(),
            6400,
            6656,
        ),
{
    hide(data_038);
    hide(scan::clean_range);
    hide(p::css_codes);
    hide(p::marketing_codes);
    hide(p::relative_codes);
    bind_038_25();
    p::bounds();
    assert(scan::clean_range(
        window_038_25(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        1,
        257,
    )) by (compute_only);
    scan::clean_window(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        6399,
        6688,
        6400,
        6656,
    );
}

} // verus!
    }
    pub mod window_26 {
        #[cfg(verus_keep_ghost)]
        use super::{data_038, shape_038};
        use crate::{
            k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
            k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
        };
        use ckc_spec::ui as u;
        use vstd::prelude::*;
        use vstd::utf8::*;
        verus! {

pub closed spec fn bytes_038_26() -> u::Bytes {
    ns![32u8,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97,114,107,46,116,50,48,58,104,111,118,101,114,41,32,109,97,114,107,46,116,50,48,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,97,53,102,51,102,99,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97,114,107,46,116,50,49,58,104,111,118,101,114,41,32,109,97,114,107,46,116,50,49,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,100,57,102,57,57,100,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97,114,107,46,116,50,50,58,104,111,118,101,114,41,32,109,97,114,107,46,116,50,50,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,99,55,100,50,102,101,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101]
}

pub closed spec fn window_038_26() -> Seq<nat> {
    scan::byte_codes(bytes_038_26())
}

pub proof fn bind_038_26()
    ensures
        data_038().len() == 10326,
        data_038().subrange(6655, 6944) == window_038_26(),
        window_038_26().len() == 289,
{
    hide(data_038);
    hide(ch::css_text_bytes);
    shape_038();
    assert(ch::css_text_bytes().subrange(6655, 6944) == bytes_038_26()) by (compute_only);
    assert(bytes_038_26().len() == 289) by (compute_only);
    scan::byte_codes_subrange(ch::css_text_bytes(), 6655, 6944);
    assert(data_038() == scan::byte_codes(ch::css_text_bytes())) by {
        reveal(data_038);
    };
    assert(window_038_26() == scan::byte_codes(bytes_038_26())) by {
        reveal(window_038_26);
    };
}

pub proof fn check_038_26()
    ensures
        scan::clean_range(
            data_038(),
            p::css_codes(),
            p::marketing_codes(),
            p::relative_codes(),
            6656,
            6912,
        ),
{
    hide(data_038);
    hide(scan::clean_range);
    hide(p::css_codes);
    hide(p::marketing_codes);
    hide(p::relative_codes);
    bind_038_26();
    p::bounds();
    assert(scan::clean_range(
        window_038_26(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        1,
        257,
    )) by (compute_only);
    scan::clean_window(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        6655,
        6944,
        6656,
        6912,
    );
}

} // verus!
    }
    pub mod window_27 {
        #[cfg(verus_keep_ghost)]
        use super::{data_038, shape_038};
        use crate::{
            k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
            k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
        };
        use ckc_spec::ui as u;
        use vstd::prelude::*;
        use vstd::utf8::*;
        verus! {

pub closed spec fn bytes_038_27() -> u::Bytes {
    ns![100u8,58,32,35,99,55,100,50,102,101,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97,114,107,46,116,50,51,58,104,111,118,101,114,41,32,109,97,114,107,46,116,50,51,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,100,54,100,51,100,49,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97,114,107,46,116,50,52,58,104,111,118,101,114,41,32,109,97,114,107,46,116,50,52,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,98,102,100,98,102,101,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97,114,107,46,116,50,53,58,104,111,118,101,114,41,32,109,97,114,107,46,116,50,53,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,102,101,102,48,56,97,59,32,116,101,120,116,45,100,101,99,111,114]
}

pub closed spec fn window_038_27() -> Seq<nat> {
    scan::byte_codes(bytes_038_27())
}

pub proof fn bind_038_27()
    ensures
        data_038().len() == 10326,
        data_038().subrange(6911, 7200) == window_038_27(),
        window_038_27().len() == 289,
{
    hide(data_038);
    hide(ch::css_text_bytes);
    shape_038();
    assert(ch::css_text_bytes().subrange(6911, 7200) == bytes_038_27()) by (compute_only);
    assert(bytes_038_27().len() == 289) by (compute_only);
    scan::byte_codes_subrange(ch::css_text_bytes(), 6911, 7200);
    assert(data_038() == scan::byte_codes(ch::css_text_bytes())) by {
        reveal(data_038);
    };
    assert(window_038_27() == scan::byte_codes(bytes_038_27())) by {
        reveal(window_038_27);
    };
}

pub proof fn check_038_27()
    ensures
        scan::clean_range(
            data_038(),
            p::css_codes(),
            p::marketing_codes(),
            p::relative_codes(),
            6912,
            7168,
        ),
{
    hide(data_038);
    hide(scan::clean_range);
    hide(p::css_codes);
    hide(p::marketing_codes);
    hide(p::relative_codes);
    bind_038_27();
    p::bounds();
    assert(scan::clean_range(
        window_038_27(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        1,
        257,
    )) by (compute_only);
    scan::clean_window(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        6911,
        7200,
        6912,
        7168,
    );
}

} // verus!
    }
    pub mod window_28 {
        #[cfg(verus_keep_ghost)]
        use super::{data_038, shape_038};
        use crate::{
            k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
            k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
        };
        use ckc_spec::ui as u;
        use vstd::prelude::*;
        use vstd::utf8::*;
        verus! {

pub closed spec fn bytes_038_28() -> u::Bytes {
    ns![123u8,32,98,97,99,107,103,114,111,117,110,100,58,32,35,102,101,102,48,56,97,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97,114,107,46,116,50,54,58,104,111,118,101,114,41,32,109,97,114,107,46,116,50,54,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,101,57,100,53,102,102,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97,114,107,46,116,50,55,58,104,111,118,101,114,41,32,109,97,114,107,46,116,50,55,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,102,101,100,55,97,97,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97,114,107,46,116,50,56,58,104,111,118,101,114,41,32,109,97,114,107,46,116,50,56,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,57,57,102,54,101,52,59]
}

pub closed spec fn window_038_28() -> Seq<nat> {
    scan::byte_codes(bytes_038_28())
}

pub proof fn bind_038_28()
    ensures
        data_038().len() == 10326,
        data_038().subrange(7167, 7456) == window_038_28(),
        window_038_28().len() == 289,
{
    hide(data_038);
    hide(ch::css_text_bytes);
    shape_038();
    assert(ch::css_text_bytes().subrange(7167, 7456) == bytes_038_28()) by (compute_only);
    assert(bytes_038_28().len() == 289) by (compute_only);
    scan::byte_codes_subrange(ch::css_text_bytes(), 7167, 7456);
    assert(data_038() == scan::byte_codes(ch::css_text_bytes())) by {
        reveal(data_038);
    };
    assert(window_038_28() == scan::byte_codes(bytes_038_28())) by {
        reveal(window_038_28);
    };
}

pub proof fn check_038_28()
    ensures
        scan::clean_range(
            data_038(),
            p::css_codes(),
            p::marketing_codes(),
            p::relative_codes(),
            7168,
            7424,
        ),
{
    hide(data_038);
    hide(scan::clean_range);
    hide(p::css_codes);
    hide(p::marketing_codes);
    hide(p::relative_codes);
    bind_038_28();
    p::bounds();
    assert(scan::clean_range(
        window_038_28(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        1,
        257,
    )) by (compute_only);
    scan::clean_window(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        7167,
        7456,
        7168,
        7424,
    );
}

} // verus!
    }
    pub mod window_29 {
        #[cfg(verus_keep_ghost)]
        use super::{data_038, shape_038};
        use crate::{
            k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
            k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
        };
        use ckc_spec::ui as u;
        use vstd::prelude::*;
        use vstd::utf8::*;
        verus! {

pub closed spec fn bytes_038_29() -> u::Bytes {
    ns![41u8,32,109,97,114,107,46,116,50,56,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,57,57,102,54,101,52,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97,114,107,46,116,50,57,58,104,111,118,101,114,41,32,109,97,114,107,46,116,50,57,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,102,101,99,100,100,51,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97,114,107,46,116,51,48,58,104,111,118,101,114,41,32,109,97,114,107,46,116,51,48,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,98,98,102,55,100,48,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97,114,107,46,116,51,49,58,104,111,118,101,114,41,32,109,97,114,107,46,116,51,49,32,123,32,98,97,99,107,103,114,111,117,110]
}

pub closed spec fn window_038_29() -> Seq<nat> {
    scan::byte_codes(bytes_038_29())
}

pub proof fn bind_038_29()
    ensures
        data_038().len() == 10326,
        data_038().subrange(7423, 7712) == window_038_29(),
        window_038_29().len() == 289,
{
    hide(data_038);
    hide(ch::css_text_bytes);
    shape_038();
    assert(ch::css_text_bytes().subrange(7423, 7712) == bytes_038_29()) by (compute_only);
    assert(bytes_038_29().len() == 289) by (compute_only);
    scan::byte_codes_subrange(ch::css_text_bytes(), 7423, 7712);
    assert(data_038() == scan::byte_codes(ch::css_text_bytes())) by {
        reveal(data_038);
    };
    assert(window_038_29() == scan::byte_codes(bytes_038_29())) by {
        reveal(window_038_29);
    };
}

pub proof fn check_038_29()
    ensures
        scan::clean_range(
            data_038(),
            p::css_codes(),
            p::marketing_codes(),
            p::relative_codes(),
            7424,
            7680,
        ),
{
    hide(data_038);
    hide(scan::clean_range);
    hide(p::css_codes);
    hide(p::marketing_codes);
    hide(p::relative_codes);
    bind_038_29();
    p::bounds();
    assert(scan::clean_range(
        window_038_29(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        1,
        257,
    )) by (compute_only);
    scan::clean_window(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        7423,
        7712,
        7424,
        7680,
    );
}

} // verus!
    }
    pub mod window_30 {
        #[cfg(verus_keep_ghost)]
        use super::{data_038, shape_038};
        use crate::{
            k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
            k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
        };
        use ckc_spec::ui as u;
        use vstd::prelude::*;
        use vstd::utf8::*;
        verus! {

pub closed spec fn bytes_038_30() -> u::Bytes {
    ns![107u8,46,116,51,49,58,104,111,118,101,114,41,32,109,97,114,107,46,116,51,49,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,102,53,100,48,102,101,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97,114,107,46,116,51,50,58,104,111,118,101,114,41,32,109,97,114,107,46,116,51,50,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,97,53,102,51,102,99,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97,114,107,46,116,51,51,58,104,111,118,101,114,41,32,109,97,114,107,46,116,51,51,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,100,57,102,57,57,100,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97,114,107,46,116,51,52,58,104,111,118,101,114,41,32,109,97,114,107,46,116,51,52,32]
}

pub closed spec fn window_038_30() -> Seq<nat> {
    scan::byte_codes(bytes_038_30())
}

pub proof fn bind_038_30()
    ensures
        data_038().len() == 10326,
        data_038().subrange(7679, 7968) == window_038_30(),
        window_038_30().len() == 289,
{
    hide(data_038);
    hide(ch::css_text_bytes);
    shape_038();
    assert(ch::css_text_bytes().subrange(7679, 7968) == bytes_038_30()) by (compute_only);
    assert(bytes_038_30().len() == 289) by (compute_only);
    scan::byte_codes_subrange(ch::css_text_bytes(), 7679, 7968);
    assert(data_038() == scan::byte_codes(ch::css_text_bytes())) by {
        reveal(data_038);
    };
    assert(window_038_30() == scan::byte_codes(bytes_038_30())) by {
        reveal(window_038_30);
    };
}

pub proof fn check_038_30()
    ensures
        scan::clean_range(
            data_038(),
            p::css_codes(),
            p::marketing_codes(),
            p::relative_codes(),
            7680,
            7936,
        ),
{
    hide(data_038);
    hide(scan::clean_range);
    hide(p::css_codes);
    hide(p::marketing_codes);
    hide(p::relative_codes);
    bind_038_30();
    p::bounds();
    assert(scan::clean_range(
        window_038_30(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        1,
        257,
    )) by (compute_only);
    scan::clean_window(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        7679,
        7968,
        7680,
        7936,
    );
}

} // verus!
    }
    pub mod window_31 {
        #[cfg(verus_keep_ghost)]
        use super::{data_038, shape_038};
        use crate::{
            k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
            k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
        };
        use ckc_spec::ui as u;
        use vstd::prelude::*;
        use vstd::utf8::*;
        verus! {

pub closed spec fn bytes_038_31() -> u::Bytes {
    ns![97u8,105,110,58,104,97,115,40,109,97,114,107,46,116,51,52,58,104,111,118,101,114,41,32,109,97,114,107,46,116,51,52,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,99,55,100,50,102,101,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97,114,107,46,116,51,53,58,104,111,118,101,114,41,32,109,97,114,107,46,116,51,53,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,100,54,100,51,100,49,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97,114,107,46,116,51,54,58,104,111,118,101,114,41,32,109,97,114,107,46,116,51,54,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,98,102,100,98,102,101,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97,114,107,46,116,51,55,58,104,111,118,101,114]
}

pub closed spec fn window_038_31() -> Seq<nat> {
    scan::byte_codes(bytes_038_31())
}

pub proof fn bind_038_31()
    ensures
        data_038().len() == 10326,
        data_038().subrange(7935, 8224) == window_038_31(),
        window_038_31().len() == 289,
{
    hide(data_038);
    hide(ch::css_text_bytes);
    shape_038();
    assert(ch::css_text_bytes().subrange(7935, 8224) == bytes_038_31()) by (compute_only);
    assert(bytes_038_31().len() == 289) by (compute_only);
    scan::byte_codes_subrange(ch::css_text_bytes(), 7935, 8224);
    assert(data_038() == scan::byte_codes(ch::css_text_bytes())) by {
        reveal(data_038);
    };
    assert(window_038_31() == scan::byte_codes(bytes_038_31())) by {
        reveal(window_038_31);
    };
}

pub proof fn check_038_31()
    ensures
        scan::clean_range(
            data_038(),
            p::css_codes(),
            p::marketing_codes(),
            p::relative_codes(),
            7936,
            8192,
        ),
{
    hide(data_038);
    hide(scan::clean_range);
    hide(p::css_codes);
    hide(p::marketing_codes);
    hide(p::relative_codes);
    bind_038_31();
    p::bounds();
    assert(scan::clean_range(
        window_038_31(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        1,
        257,
    )) by (compute_only);
    scan::clean_window(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        7935,
        8224,
        7936,
        8192,
    );
}

} // verus!
    }
    pub mod window_32 {
        #[cfg(verus_keep_ghost)]
        use super::{data_038, shape_038};
        use crate::{
            k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
            k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
        };
        use ckc_spec::ui as u;
        use vstd::prelude::*;
        use vstd::utf8::*;
        verus! {

pub closed spec fn bytes_038_32() -> u::Bytes {
    ns![32u8,115,111,108,105,100,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97,114,107,46,116,51,55,58,104,111,118,101,114,41,32,109,97,114,107,46,116,51,55,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,102,101,102,48,56,97,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97,114,107,46,116,51,56,58,104,111,118,101,114,41,32,109,97,114,107,46,116,51,56,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,101,57,100,53,102,102,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97,114,107,46,116,51,57,58,104,111,118,101,114,41,32,109,97,114,107,46,116,51,57,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,102,101,100,55,97,97,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97,114]
}

pub closed spec fn window_038_32() -> Seq<nat> {
    scan::byte_codes(bytes_038_32())
}

pub proof fn bind_038_32()
    ensures
        data_038().len() == 10326,
        data_038().subrange(8191, 8480) == window_038_32(),
        window_038_32().len() == 289,
{
    hide(data_038);
    hide(ch::css_text_bytes);
    shape_038();
    assert(ch::css_text_bytes().subrange(8191, 8480) == bytes_038_32()) by (compute_only);
    assert(bytes_038_32().len() == 289) by (compute_only);
    scan::byte_codes_subrange(ch::css_text_bytes(), 8191, 8480);
    assert(data_038() == scan::byte_codes(ch::css_text_bytes())) by {
        reveal(data_038);
    };
    assert(window_038_32() == scan::byte_codes(bytes_038_32())) by {
        reveal(window_038_32);
    };
}

pub proof fn check_038_32()
    ensures
        scan::clean_range(
            data_038(),
            p::css_codes(),
            p::marketing_codes(),
            p::relative_codes(),
            8192,
            8448,
        ),
{
    hide(data_038);
    hide(scan::clean_range);
    hide(p::css_codes);
    hide(p::marketing_codes);
    hide(p::relative_codes);
    bind_038_32();
    p::bounds();
    assert(scan::clean_range(
        window_038_32(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        1,
        257,
    )) by (compute_only);
    scan::clean_window(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        8191,
        8480,
        8192,
        8448,
    );
}

} // verus!
    }
    pub mod window_33 {
        #[cfg(verus_keep_ghost)]
        use super::{data_038, shape_038};
        use crate::{
            k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
            k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
        };
        use ckc_spec::ui as u;
        use vstd::prelude::*;
        use vstd::utf8::*;
        verus! {

pub closed spec fn bytes_038_33() -> u::Bytes {
    ns![116u8,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97,114,107,46,116,52,48,58,104,111,118,101,114,41,32,109,97,114,107,46,116,52,48,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,57,57,102,54,101,52,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97,114,107,46,116,52,49,58,104,111,118,101,114,41,32,109,97,114,107,46,116,52,49,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,102,101,99,100,100,51,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97,114,107,46,116,52,50,58,104,111,118,101,114,41,32,109,97,114,107,46,116,52,50,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,98,98,102,55,100,48,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10,109]
}

pub closed spec fn window_038_33() -> Seq<nat> {
    scan::byte_codes(bytes_038_33())
}

pub proof fn bind_038_33()
    ensures
        data_038().len() == 10326,
        data_038().subrange(8447, 8736) == window_038_33(),
        window_038_33().len() == 289,
{
    hide(data_038);
    hide(ch::css_text_bytes);
    shape_038();
    assert(ch::css_text_bytes().subrange(8447, 8736) == bytes_038_33()) by (compute_only);
    assert(bytes_038_33().len() == 289) by (compute_only);
    scan::byte_codes_subrange(ch::css_text_bytes(), 8447, 8736);
    assert(data_038() == scan::byte_codes(ch::css_text_bytes())) by {
        reveal(data_038);
    };
    assert(window_038_33() == scan::byte_codes(bytes_038_33())) by {
        reveal(window_038_33);
    };
}

pub proof fn check_038_33()
    ensures
        scan::clean_range(
            data_038(),
            p::css_codes(),
            p::marketing_codes(),
            p::relative_codes(),
            8448,
            8704,
        ),
{
    hide(data_038);
    hide(scan::clean_range);
    hide(p::css_codes);
    hide(p::marketing_codes);
    hide(p::relative_codes);
    bind_038_33();
    p::bounds();
    assert(scan::clean_range(
        window_038_33(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        1,
        257,
    )) by (compute_only);
    scan::clean_window(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        8447,
        8736,
        8448,
        8704,
    );
}

} // verus!
    }
    pub mod window_34 {
        #[cfg(verus_keep_ghost)]
        use super::{data_038, shape_038};
        use crate::{
            k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
            k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
        };
        use ckc_spec::ui as u;
        use vstd::prelude::*;
        use vstd::utf8::*;
        verus! {

pub closed spec fn bytes_038_34() -> u::Bytes {
    ns![116u8,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97,114,107,46,116,52,51,58,104,111,118,101,114,41,32,109,97,114,107,46,116,52,51,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,102,53,100,48,102,101,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97,114,107,46,116,52,52,58,104,111,118,101,114,41,32,109,97,114,107,46,116,52,52,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,97,53,102,51,102,99,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97,114,107,46,116,52,53,58,104,111,118,101,114,41,32,109,97,114,107,46,116,52,53,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,100,57,102,57,57,100,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101,58]
}

pub closed spec fn window_038_34() -> Seq<nat> {
    scan::byte_codes(bytes_038_34())
}

pub proof fn bind_038_34()
    ensures
        data_038().len() == 10326,
        data_038().subrange(8703, 8992) == window_038_34(),
        window_038_34().len() == 289,
{
    hide(data_038);
    hide(ch::css_text_bytes);
    shape_038();
    assert(ch::css_text_bytes().subrange(8703, 8992) == bytes_038_34()) by (compute_only);
    assert(bytes_038_34().len() == 289) by (compute_only);
    scan::byte_codes_subrange(ch::css_text_bytes(), 8703, 8992);
    assert(data_038() == scan::byte_codes(ch::css_text_bytes())) by {
        reveal(data_038);
    };
    assert(window_038_34() == scan::byte_codes(bytes_038_34())) by {
        reveal(window_038_34);
    };
}

pub proof fn check_038_34()
    ensures
        scan::clean_range(
            data_038(),
            p::css_codes(),
            p::marketing_codes(),
            p::relative_codes(),
            8704,
            8960,
        ),
{
    hide(data_038);
    hide(scan::clean_range);
    hide(p::css_codes);
    hide(p::marketing_codes);
    hide(p::relative_codes);
    bind_038_34();
    p::bounds();
    assert(scan::clean_range(
        window_038_34(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        1,
        257,
    )) by (compute_only);
    scan::clean_window(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        8703,
        8992,
        8704,
        8960,
    );
}

} // verus!
    }
    pub mod window_35 {
        #[cfg(verus_keep_ghost)]
        use super::{data_038, shape_038};
        use crate::{
            k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
            k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
        };
        use ckc_spec::ui as u;
        use vstd::prelude::*;
        use vstd::utf8::*;
        verus! {

pub closed spec fn bytes_038_35() -> u::Bytes {
    ns![58u8,32,35,100,57,102,57,57,100,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97,114,107,46,116,52,54,58,104,111,118,101,114,41,32,109,97,114,107,46,116,52,54,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,99,55,100,50,102,101,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10,109,97,105,110,58,104,97,115,40,109,97,114,107,46,116,52,55,58,104,111,118,101,114,41,32,109,97,114,107,46,116,52,55,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,35,100,54,100,51,100,49,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,115,116,121,108,101,58,32,115,111,108,105,100,59,32,125,10,46,104,108,45,110,111,116,101,32,108,97,98,101,108,32,123,32,109,97,114,103,105,110,45,114,105,103,104,116,58,32,48,46,55,53,114,101,109,59,32,125,10,109,97,105,110,46,104,108,45,99,108,105,99,107,32,112,114,101,46,112,114,111,115,101,32,123,32]
}

pub closed spec fn window_038_35() -> Seq<nat> {
    scan::byte_codes(bytes_038_35())
}

pub proof fn bind_038_35()
    ensures
        data_038().len() == 10326,
        data_038().subrange(8959, 9248) == window_038_35(),
        window_038_35().len() == 289,
{
    hide(data_038);
    hide(ch::css_text_bytes);
    shape_038();
    assert(ch::css_text_bytes().subrange(8959, 9248) == bytes_038_35()) by (compute_only);
    assert(bytes_038_35().len() == 289) by (compute_only);
    scan::byte_codes_subrange(ch::css_text_bytes(), 8959, 9248);
    assert(data_038() == scan::byte_codes(ch::css_text_bytes())) by {
        reveal(data_038);
    };
    assert(window_038_35() == scan::byte_codes(bytes_038_35())) by {
        reveal(window_038_35);
    };
}

pub proof fn check_038_35()
    ensures
        scan::clean_range(
            data_038(),
            p::css_codes(),
            p::marketing_codes(),
            p::relative_codes(),
            8960,
            9216,
        ),
{
    hide(data_038);
    hide(scan::clean_range);
    hide(p::css_codes);
    hide(p::marketing_codes);
    hide(p::relative_codes);
    bind_038_35();
    p::bounds();
    assert(scan::clean_range(
        window_038_35(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        1,
        257,
    )) by (compute_only);
    scan::clean_window(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        8959,
        9248,
        8960,
        9216,
    );
}

} // verus!
    }
    pub mod window_36 {
        #[cfg(verus_keep_ghost)]
        use super::{data_038, shape_038};
        use crate::{
            k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
            k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
        };
        use ckc_spec::ui as u;
        use vstd::prelude::*;
        use vstd::utf8::*;
        verus! {

pub closed spec fn bytes_038_36() -> u::Bytes {
    ns![114u8,101,109,59,32,125,10,109,97,105,110,46,104,108,45,99,108,105,99,107,32,112,114,101,46,112,114,111,115,101,32,123,32,99,111,108,111,114,58,32,35,57,99,97,51,97,102,59,32,125,10,109,97,105,110,46,104,108,45,99,108,105,99,107,32,112,114,101,46,112,114,111,115,101,32,46,107,119,32,123,32,99,111,108,111,114,58,32,35,57,99,97,51,97,102,59,32,125,10,109,97,105,110,46,104,108,45,99,108,105,99,107,32,112,114,101,46,112,114,111,115,101,32,109,97,114,107,58,110,111,116,40,46,104,108,45,112,105,99,107,41,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,110,111,110,101,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,99,111,108,111,114,58,32,35,57,99,97,51,97,102,59,32,125,10,109,97,105,110,46,104,108,45,99,108,105,99,107,32,112,114,101,46,112,114,111,115,101,32,109,97,114,107,46,104,108,45,112,105,99,107,32,123,32,99,111,108,111,114,58,32,35,49,49,49,56,50,55,59,32,125,10,98,111,100,121,58,104,97,115,40,105,110,112,117,116,46,104,108,45,116,111,103,103,108,101,58,110,111,116,40,58,99,104,101,99,107,101]
}

pub closed spec fn window_038_36() -> Seq<nat> {
    scan::byte_codes(bytes_038_36())
}

pub proof fn bind_038_36()
    ensures
        data_038().len() == 10326,
        data_038().subrange(9215, 9504) == window_038_36(),
        window_038_36().len() == 289,
{
    hide(data_038);
    hide(ch::css_text_bytes);
    shape_038();
    assert(ch::css_text_bytes().subrange(9215, 9504) == bytes_038_36()) by (compute_only);
    assert(bytes_038_36().len() == 289) by (compute_only);
    scan::byte_codes_subrange(ch::css_text_bytes(), 9215, 9504);
    assert(data_038() == scan::byte_codes(ch::css_text_bytes())) by {
        reveal(data_038);
    };
    assert(window_038_36() == scan::byte_codes(bytes_038_36())) by {
        reveal(window_038_36);
    };
}

pub proof fn check_038_36()
    ensures
        scan::clean_range(
            data_038(),
            p::css_codes(),
            p::marketing_codes(),
            p::relative_codes(),
            9216,
            9472,
        ),
{
    hide(data_038);
    hide(scan::clean_range);
    hide(p::css_codes);
    hide(p::marketing_codes);
    hide(p::relative_codes);
    bind_038_36();
    p::bounds();
    assert(scan::clean_range(
        window_038_36(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        1,
        257,
    )) by (compute_only);
    scan::clean_window(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        9215,
        9504,
        9216,
        9472,
    );
}

} // verus!
    }
    pub mod window_37 {
        #[cfg(verus_keep_ghost)]
        use super::{data_038, shape_038};
        use crate::{
            k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
            k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
        };
        use ckc_spec::ui as u;
        use vstd::prelude::*;
        use vstd::utf8::*;
        verus! {

pub closed spec fn bytes_038_37() -> u::Bytes {
    ns![121u8,58,104,97,115,40,105,110,112,117,116,46,104,108,45,116,111,103,103,108,101,58,110,111,116,40,58,99,104,101,99,107,101,100,41,41,32,112,114,101,46,112,114,111,115,101,32,109,97,114,107,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,110,111,110,101,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,58,32,110,111,110,101,59,32,99,111,108,111,114,58,32,105,110,104,101,114,105,116,59,32,125,10,98,111,100,121,58,104,97,115,40,105,110,112,117,116,46,104,108,45,116,111,103,103,108,101,58,110,111,116,40,58,99,104,101,99,107,101,100,41,41,32,112,114,101,46,112,114,111,115,101,32,46,107,119,32,123,32,99,111,108,111,114,58,32,105,110,104,101,114,105,116,59,32,125,10,64,109,101,100,105,97,32,112,114,105,110,116,32,123,10,98,111,100,121,32,123,32,109,97,120,45,119,105,100,116,104,58,32,110,111,110,101,59,32,112,97,100,100,105,110,103,58,32,48,59,32,125,10,110,97,118,46,99,114,117,109,98,115,44,32,110,97,118,46,100,111,99,110,97,118,44,32,46,115,107,105,112,44,32,102,111,114,109,44,32,46,118,101,114,100,105,99,116,45,101,110,116]
}

pub closed spec fn window_038_37() -> Seq<nat> {
    scan::byte_codes(bytes_038_37())
}

pub proof fn bind_038_37()
    ensures
        data_038().len() == 10326,
        data_038().subrange(9471, 9760) == window_038_37(),
        window_038_37().len() == 289,
{
    hide(data_038);
    hide(ch::css_text_bytes);
    shape_038();
    assert(ch::css_text_bytes().subrange(9471, 9760) == bytes_038_37()) by (compute_only);
    assert(bytes_038_37().len() == 289) by (compute_only);
    scan::byte_codes_subrange(ch::css_text_bytes(), 9471, 9760);
    assert(data_038() == scan::byte_codes(ch::css_text_bytes())) by {
        reveal(data_038);
    };
    assert(window_038_37() == scan::byte_codes(bytes_038_37())) by {
        reveal(window_038_37);
    };
}

pub proof fn check_038_37()
    ensures
        scan::clean_range(
            data_038(),
            p::css_codes(),
            p::marketing_codes(),
            p::relative_codes(),
            9472,
            9728,
        ),
{
    hide(data_038);
    hide(scan::clean_range);
    hide(p::css_codes);
    hide(p::marketing_codes);
    hide(p::relative_codes);
    bind_038_37();
    p::bounds();
    assert(scan::clean_range(
        window_038_37(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        1,
        257,
    )) by (compute_only);
    scan::clean_window(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        9471,
        9760,
        9472,
        9728,
    );
}

} // verus!
    }
    pub mod window_38 {
        #[cfg(verus_keep_ghost)]
        use super::{data_038, shape_038};
        use crate::{
            k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
            k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
        };
        use ckc_spec::ui as u;
        use vstd::prelude::*;
        use vstd::utf8::*;
        verus! {

pub closed spec fn bytes_038_38() -> u::Bytes {
    ns![100u8,111,99,110,97,118,44,32,46,115,107,105,112,44,32,102,111,114,109,44,32,46,118,101,114,100,105,99,116,45,101,110,116,114,121,44,32,46,104,108,45,110,111,116,101,32,123,32,100,105,115,112,108,97,121,58,32,110,111,110,101,59,32,125,10,109,97,114,107,44,32,109,97,114,107,91,99,108,97,115,115,93,32,123,32,98,97,99,107,103,114,111,117,110,100,58,32,110,111,110,101,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,99,111,108,111,114,58,32,35,52,98,53,53,54,51,59,32,125,10,109,97,105,110,46,104,108,45,99,108,105,99,107,32,112,114,101,46,112,114,111,115,101,44,32,109,97,105,110,46,104,108,45,99,108,105,99,107,32,112,114,101,46,112,114,111,115,101,32,109,97,114,107,46,104,108,45,112,105,99,107,32,123,32,99,111,108,111,114,58,32,105,110,104,101,114,105,116,59,32,125,10,109,97,105,110,46,104,108,45,99,108,105,99,107,32,112,114,101,46,112,114,111,115,101,32,46,107,119,32,123,32,99,111,108,111,114,58,32,35,52,98,53,53,54,51,59,32,125,10,109,97,105,110,46,104,108,45,99,108,105,99,107,32,112,114,101,46,112,114,111,115]
}

pub closed spec fn window_038_38() -> Seq<nat> {
    scan::byte_codes(bytes_038_38())
}

pub proof fn bind_038_38()
    ensures
        data_038().len() == 10326,
        data_038().subrange(9727, 10016) == window_038_38(),
        window_038_38().len() == 289,
{
    hide(data_038);
    hide(ch::css_text_bytes);
    shape_038();
    assert(ch::css_text_bytes().subrange(9727, 10016) == bytes_038_38()) by (compute_only);
    assert(bytes_038_38().len() == 289) by (compute_only);
    scan::byte_codes_subrange(ch::css_text_bytes(), 9727, 10016);
    assert(data_038() == scan::byte_codes(ch::css_text_bytes())) by {
        reveal(data_038);
    };
    assert(window_038_38() == scan::byte_codes(bytes_038_38())) by {
        reveal(window_038_38);
    };
}

pub proof fn check_038_38()
    ensures
        scan::clean_range(
            data_038(),
            p::css_codes(),
            p::marketing_codes(),
            p::relative_codes(),
            9728,
            9984,
        ),
{
    hide(data_038);
    hide(scan::clean_range);
    hide(p::css_codes);
    hide(p::marketing_codes);
    hide(p::relative_codes);
    bind_038_38();
    p::bounds();
    assert(scan::clean_range(
        window_038_38(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        1,
        257,
    )) by (compute_only);
    scan::clean_window(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        9727,
        10016,
        9728,
        9984,
    );
}

} // verus!
    }
    pub mod window_39 {
        #[cfg(verus_keep_ghost)]
        use super::{data_038, shape_038};
        use crate::{
            k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
            k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
        };
        use ckc_spec::ui as u;
        use vstd::prelude::*;
        use vstd::utf8::*;
        verus! {

pub closed spec fn bytes_038_39() -> u::Bytes {
    ns![35u8,52,98,53,53,54,51,59,32,125,10,109,97,105,110,46,104,108,45,99,108,105,99,107,32,112,114,101,46,112,114,111,115,101,32,109,97,114,107,58,110,111,116,40,46,104,108,45,112,105,99,107,41,32,123,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,45,99,111,108,111,114,58,32,35,52,98,53,53,54,51,59,32,125,10,100,101,116,97,105,108,115,58,58,100,101,116,97,105,108,115,45,99,111,110,116,101,110,116,32,123,32,99,111,110,116,101,110,116,45,118,105,115,105,98,105,108,105,116,121,58,32,118,105,115,105,98,108,101,59,32,125,10,112,114,101,32,123,32,98,111,114,100,101,114,58,32,110,111,110,101,59,32,112,97,100,100,105,110,103,58,32,48,59,32,119,104,105,116,101,45,115,112,97,99,101,58,32,112,114,101,45,119,114,97,112,59,32,111,118,101,114,102,108,111,119,45,120,58,32,118,105,115,105,98,108,101,59,32,125,10,97,32,123,32,99,111,108,111,114,58,32,105,110,104,101,114,105,116,59,32,116,101,120,116,45,100,101,99,111,114,97,116,105,111,110,58,32,110,111,110,101,59,32,125,10,46,99,104,105,112,32,123,32,98,111,114,100,101,114,58,32,49,112]
}

pub closed spec fn window_038_39() -> Seq<nat> {
    scan::byte_codes(bytes_038_39())
}

pub proof fn bind_038_39()
    ensures
        data_038().len() == 10326,
        data_038().subrange(9983, 10272) == window_038_39(),
        window_038_39().len() == 289,
{
    hide(data_038);
    hide(ch::css_text_bytes);
    shape_038();
    assert(ch::css_text_bytes().subrange(9983, 10272) == bytes_038_39()) by (compute_only);
    assert(bytes_038_39().len() == 289) by (compute_only);
    scan::byte_codes_subrange(ch::css_text_bytes(), 9983, 10272);
    assert(data_038() == scan::byte_codes(ch::css_text_bytes())) by {
        reveal(data_038);
    };
    assert(window_038_39() == scan::byte_codes(bytes_038_39())) by {
        reveal(window_038_39);
    };
}

pub proof fn check_038_39()
    ensures
        scan::clean_range(
            data_038(),
            p::css_codes(),
            p::marketing_codes(),
            p::relative_codes(),
            9984,
            10240,
        ),
{
    hide(data_038);
    hide(scan::clean_range);
    hide(p::css_codes);
    hide(p::marketing_codes);
    hide(p::relative_codes);
    bind_038_39();
    p::bounds();
    assert(scan::clean_range(
        window_038_39(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        1,
        257,
    )) by (compute_only);
    scan::clean_window(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        9983,
        10272,
        9984,
        10240,
    );
}

} // verus!
    }
    pub mod window_40 {
        #[cfg(verus_keep_ghost)]
        use super::{data_038, shape_038};
        use crate::{
            k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
            k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
        };
        use ckc_spec::ui as u;
        use vstd::prelude::*;
        use vstd::utf8::*;
        verus! {

pub closed spec fn bytes_038_40() -> u::Bytes {
    ns![97u8,116,105,111,110,58,32,110,111,110,101,59,32,125,10,46,99,104,105,112,32,123,32,98,111,114,100,101,114,58,32,49,112,120,32,115,111,108,105,100,32,35,49,49,49,56,50,55,59,32,98,97,99,107,103,114,111,117,110,100,58,32,110,111,110,101,59,32,99,111,108,111,114,58,32,105,110,104,101,114,105,116,59,32,125,10,125]
}

pub closed spec fn window_038_40() -> Seq<nat> {
    scan::byte_codes(bytes_038_40())
}

pub proof fn bind_038_40()
    ensures
        data_038().len() == 10326,
        data_038().subrange(10239, 10326) == window_038_40(),
        window_038_40().len() == 87,
{
    hide(data_038);
    hide(ch::css_text_bytes);
    shape_038();
    assert(ch::css_text_bytes().subrange(10239, 10326) == bytes_038_40()) by (compute_only);
    assert(bytes_038_40().len() == 87) by (compute_only);
    scan::byte_codes_subrange(ch::css_text_bytes(), 10239, 10326);
    assert(data_038() == scan::byte_codes(ch::css_text_bytes())) by {
        reveal(data_038);
    };
    assert(window_038_40() == scan::byte_codes(bytes_038_40())) by {
        reveal(window_038_40);
    };
}

pub proof fn check_038_40()
    ensures
        scan::clean_range(
            data_038(),
            p::css_codes(),
            p::marketing_codes(),
            p::relative_codes(),
            10240,
            10326,
        ),
{
    hide(data_038);
    hide(scan::clean_range);
    hide(p::css_codes);
    hide(p::marketing_codes);
    hide(p::relative_codes);
    bind_038_40();
    p::bounds();
    assert(scan::clean_range(
        window_038_40(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        1,
        87,
    )) by (compute_only);
    scan::clean_window(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        10239,
        10326,
        10240,
        10326,
    );
}

} // verus!
    }
    verus! {

pub open spec fn data_038() -> Seq<nat> {
    scan::byte_codes(ch::css_text_bytes())
}

pub proof fn shape_038()
    ensures
        data_038().len() == 10326,
        ch::css_text_bytes().len() == 10326,
{
    hide(ch::css_text_bytes);
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
}

pub proof fn clean_038()
    ensures
        n::clean(data_038(), p::css_codes(), p::marketing_codes(), p::relative_codes()),
{
    hide(data_038);
    hide(scan::clean_range);
    hide(n::clean);
    shape_038();
    scan::clean_empty(data_038(), p::css_codes(), p::marketing_codes(), p::relative_codes(), 0);
    window_0::check_038_0();
    scan::clean_join(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        0,
        0,
        256,
    );
    window_1::check_038_1();
    scan::clean_join(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        0,
        256,
        512,
    );
    window_2::check_038_2();
    scan::clean_join(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        0,
        512,
        768,
    );
    window_3::check_038_3();
    scan::clean_join(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        0,
        768,
        1024,
    );
    window_4::check_038_4();
    scan::clean_join(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        0,
        1024,
        1280,
    );
    window_5::check_038_5();
    scan::clean_join(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        0,
        1280,
        1536,
    );
    window_6::check_038_6();
    scan::clean_join(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        0,
        1536,
        1792,
    );
    window_7::check_038_7();
    scan::clean_join(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        0,
        1792,
        2048,
    );
    window_8::check_038_8();
    scan::clean_join(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        0,
        2048,
        2304,
    );
    window_9::check_038_9();
    scan::clean_join(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        0,
        2304,
        2560,
    );
    window_10::check_038_10();
    scan::clean_join(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        0,
        2560,
        2816,
    );
    window_11::check_038_11();
    scan::clean_join(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        0,
        2816,
        3072,
    );
    window_12::check_038_12();
    scan::clean_join(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        0,
        3072,
        3328,
    );
    window_13::check_038_13();
    scan::clean_join(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        0,
        3328,
        3584,
    );
    window_14::check_038_14();
    scan::clean_join(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        0,
        3584,
        3840,
    );
    window_15::check_038_15();
    scan::clean_join(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        0,
        3840,
        4096,
    );
    window_16::check_038_16();
    scan::clean_join(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        0,
        4096,
        4352,
    );
    window_17::check_038_17();
    scan::clean_join(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        0,
        4352,
        4608,
    );
    window_18::check_038_18();
    scan::clean_join(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        0,
        4608,
        4864,
    );
    window_19::check_038_19();
    scan::clean_join(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        0,
        4864,
        5120,
    );
    window_20::check_038_20();
    scan::clean_join(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        0,
        5120,
        5376,
    );
    window_21::check_038_21();
    scan::clean_join(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        0,
        5376,
        5632,
    );
    window_22::check_038_22();
    scan::clean_join(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        0,
        5632,
        5888,
    );
    window_23::check_038_23();
    scan::clean_join(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        0,
        5888,
        6144,
    );
    window_24::check_038_24();
    scan::clean_join(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        0,
        6144,
        6400,
    );
    window_25::check_038_25();
    scan::clean_join(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        0,
        6400,
        6656,
    );
    window_26::check_038_26();
    scan::clean_join(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        0,
        6656,
        6912,
    );
    window_27::check_038_27();
    scan::clean_join(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        0,
        6912,
        7168,
    );
    window_28::check_038_28();
    scan::clean_join(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        0,
        7168,
        7424,
    );
    window_29::check_038_29();
    scan::clean_join(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        0,
        7424,
        7680,
    );
    window_30::check_038_30();
    scan::clean_join(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        0,
        7680,
        7936,
    );
    window_31::check_038_31();
    scan::clean_join(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        0,
        7936,
        8192,
    );
    window_32::check_038_32();
    scan::clean_join(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        0,
        8192,
        8448,
    );
    window_33::check_038_33();
    scan::clean_join(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        0,
        8448,
        8704,
    );
    window_34::check_038_34();
    scan::clean_join(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        0,
        8704,
        8960,
    );
    window_35::check_038_35();
    scan::clean_join(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        0,
        8960,
        9216,
    );
    window_36::check_038_36();
    scan::clean_join(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        0,
        9216,
        9472,
    );
    window_37::check_038_37();
    scan::clean_join(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        0,
        9472,
        9728,
    );
    window_38::check_038_38();
    scan::clean_join(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        0,
        9728,
        9984,
    );
    window_39::check_038_39();
    scan::clean_join(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        0,
        9984,
        10240,
    );
    window_40::check_038_40();
    scan::clean_join(
        data_038(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        0,
        10240,
        10326,
    );
    scan::clean_full(data_038(), p::css_codes(), p::marketing_codes(), p::relative_codes());
}

pub proof fn literal_038()
    ensures
        u::copy_literal_ok(u::copy_registry()[38]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    hide(u::css_text);
    hide(ch::css_text_bytes);
    hide(enc::css_text_chars);
    l::l038(Seq::empty());
    p::all();
    enc::css_text_shape();
    enc::css_text_encoding();
    assert forall|j: int| 0 <= j < enc::css_text_chars().len() implies '\x00'
        <= #[trigger] enc::css_text_chars()[j] <= '\x7f' by {
        enc::css_text_range_0_162(j);
    }
    scan::ascii_input(enc::css_text_chars());
    assert(n::codes(enc::css_text_chars()) == data_038());
    clean_038();
    n::clean_bridge(enc::css_text_chars());
}

} // verus!
}
pub mod script {
    use crate::{
        k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
        k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
    };
    use ckc_spec::ui as u;
    use vstd::prelude::*;
    use vstd::utf8::*;
    pub mod window_0 {
        #[cfg(verus_keep_ghost)]
        use super::{data_039, shape_039};
        use crate::{
            k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
            k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
        };
        use ckc_spec::ui as u;
        use vstd::prelude::*;
        use vstd::utf8::*;
        verus! {

pub closed spec fn bytes_039_0() -> u::Bytes {
    ns![60u8,115,99,114,105,112,116,62,10,40,102,117,110,99,116,105,111,110,32,40,41,32,123,10,34,117,115,101,32,115,116,114,105,99,116,34,59,10,118,97,114,32,112,105,99,107,101,100,32,61,32,34,34,59,10,102,117,110,99,116,105,111,110,32,103,114,111,117,112,79,102,40,110,111,100,101,41,32,123,10,105,102,32,40,110,111,100,101,32,61,61,61,32,110,117,108,108,41,32,123,32,114,101,116,117,114,110,32,34,34,59,32,125,10,118,97,114,32,109,32,61,32,110,111,100,101,46,99,108,111,115,101,115,116,40,34,109,97,114,107,34,41,59,10,105,102,32,40,109,32,61,61,61,32,110,117,108,108,41,32,123,32,114,101,116,117,114,110,32,34,34,59,32,125,10,118,97,114,32,110,97,109,101,32,61,32,109,46,99,108,97,115,115,76,105,115,116,46,105,116,101,109,40,48,41,59,10,105,102,32,40,110,97,109,101,32,61,61,61,32,110,117,108,108,41,32,123,32,114,101,116,117,114,110,32,34,34,59,32,125,10,114,101,116,117,114,110,32,110,97,109,101,59,10,125,10,102,117,110,99,116,105,111,110,32,99,108,101,97,114,80,105,99,107,40,41,32,123,10,105,102,32,40,112,105,99,107,101]
}

pub closed spec fn window_039_0() -> Seq<nat> {
    scan::byte_codes(bytes_039_0())
}

pub proof fn bind_039_0()
    ensures
        data_039().len() == 1479,
        data_039().subrange(0, 288) == window_039_0(),
        window_039_0().len() == 288,
{
    hide(data_039);
    hide(ch::script_html_bytes);
    shape_039();
    assert(ch::script_html_bytes().subrange(0, 288) == bytes_039_0()) by (compute_only);
    assert(bytes_039_0().len() == 288) by (compute_only);
    scan::byte_codes_subrange(ch::script_html_bytes(), 0, 288);
    assert(data_039() == scan::byte_codes(ch::script_html_bytes())) by {
        reveal(data_039);
    };
    assert(window_039_0() == scan::byte_codes(bytes_039_0())) by {
        reveal(window_039_0);
    };
}

pub proof fn check_039_0()
    ensures
        scan::clean_range(
            data_039(),
            p::css_codes(),
            p::marketing_codes(),
            p::relative_codes(),
            0,
            256,
        ),
{
    hide(data_039);
    hide(scan::clean_range);
    hide(p::css_codes);
    hide(p::marketing_codes);
    hide(p::relative_codes);
    bind_039_0();
    p::bounds();
    assert(scan::clean_range(
        window_039_0(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        0,
        256,
    )) by (compute_only);
    scan::clean_window(
        data_039(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        0,
        288,
        0,
        256,
    );
}

} // verus!
    }
    pub mod window_1 {
        #[cfg(verus_keep_ghost)]
        use super::{data_039, shape_039};
        use crate::{
            k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
            k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
        };
        use ckc_spec::ui as u;
        use vstd::prelude::*;
        use vstd::utf8::*;
        verus! {

pub closed spec fn bytes_039_1() -> u::Bytes {
    ns![10u8,102,117,110,99,116,105,111,110,32,99,108,101,97,114,80,105,99,107,40,41,32,123,10,105,102,32,40,112,105,99,107,101,100,32,61,61,61,32,34,34,41,32,123,32,114,101,116,117,114,110,59,32,125,10,100,111,99,117,109,101,110,116,46,103,101,116,69,108,101,109,101,110,116,66,121,73,100,40,34,109,97,105,110,34,41,46,99,108,97,115,115,76,105,115,116,46,114,101,109,111,118,101,40,34,104,108,45,99,108,105,99,107,34,41,59,10,118,97,114,32,109,97,114,107,115,32,61,32,100,111,99,117,109,101,110,116,46,113,117,101,114,121,83,101,108,101,99,116,111,114,65,108,108,40,34,109,97,114,107,46,104,108,45,112,105,99,107,34,41,59,10,118,97,114,32,105,32,61,32,48,59,10,119,104,105,108,101,32,40,105,32,60,32,109,97,114,107,115,46,108,101,110,103,116,104,41,32,123,32,109,97,114,107,115,91,105,93,46,99,108,97,115,115,76,105,115,116,46,114,101,109,111,118,101,40,34,104,108,45,112,105,99,107,34,41,59,32,105,32,43,61,32,49,59,32,125,10,112,105,99,107,101,100,32,61,32,34,34,59,10,125,10,102,117,110,99,116,105,111,110,32,116,111,103,103,108,101,66]
}

pub closed spec fn window_039_1() -> Seq<nat> {
    scan::byte_codes(bytes_039_1())
}

pub proof fn bind_039_1()
    ensures
        data_039().len() == 1479,
        data_039().subrange(255, 544) == window_039_1(),
        window_039_1().len() == 289,
{
    hide(data_039);
    hide(ch::script_html_bytes);
    shape_039();
    assert(ch::script_html_bytes().subrange(255, 544) == bytes_039_1()) by (compute_only);
    assert(bytes_039_1().len() == 289) by (compute_only);
    scan::byte_codes_subrange(ch::script_html_bytes(), 255, 544);
    assert(data_039() == scan::byte_codes(ch::script_html_bytes())) by {
        reveal(data_039);
    };
    assert(window_039_1() == scan::byte_codes(bytes_039_1())) by {
        reveal(window_039_1);
    };
}

pub proof fn check_039_1()
    ensures
        scan::clean_range(
            data_039(),
            p::css_codes(),
            p::marketing_codes(),
            p::relative_codes(),
            256,
            512,
        ),
{
    hide(data_039);
    hide(scan::clean_range);
    hide(p::css_codes);
    hide(p::marketing_codes);
    hide(p::relative_codes);
    bind_039_1();
    p::bounds();
    assert(scan::clean_range(
        window_039_1(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        1,
        257,
    )) by (compute_only);
    scan::clean_window(
        data_039(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        255,
        544,
        256,
        512,
    );
}

} // verus!
    }
    pub mod window_2 {
        #[cfg(verus_keep_ghost)]
        use super::{data_039, shape_039};
        use crate::{
            k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
            k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
        };
        use ckc_spec::ui as u;
        use vstd::prelude::*;
        use vstd::utf8::*;
        verus! {

pub closed spec fn bytes_039_2() -> u::Bytes {
    ns![125u8,10,112,105,99,107,101,100,32,61,32,34,34,59,10,125,10,102,117,110,99,116,105,111,110,32,116,111,103,103,108,101,66,111,120,40,41,32,123,32,114,101,116,117,114,110,32,100,111,99,117,109,101,110,116,46,113,117,101,114,121,83,101,108,101,99,116,111,114,40,34,105,110,112,117,116,46,104,108,45,116,111,103,103,108,101,34,41,59,32,125,10,100,111,99,117,109,101,110,116,46,97,100,100,69,118,101,110,116,76,105,115,116,101,110,101,114,40,34,99,108,105,99,107,34,44,32,102,117,110,99,116,105,111,110,32,40,101,118,41,32,123,10,118,97,114,32,110,97,109,101,32,61,32,103,114,111,117,112,79,102,40,101,118,46,116,97,114,103,101,116,41,59,10,105,102,32,40,110,97,109,101,32,61,61,61,32,34,34,41,32,123,32,114,101,116,117,114,110,59,32,125,10,118,97,114,32,98,111,120,32,61,32,116,111,103,103,108,101,66,111,120,40,41,59,10,105,102,32,40,98,111,120,32,61,61,61,32,110,117,108,108,41,32,123,32,114,101,116,117,114,110,59,32,125,10,105,102,32,40,98,111,120,46,99,104,101,99,107,101,100,32,61,61,61,32,102,97,108,115,101,41,32,123,32,114,101,116,117]
}

pub closed spec fn window_039_2() -> Seq<nat> {
    scan::byte_codes(bytes_039_2())
}

pub proof fn bind_039_2()
    ensures
        data_039().len() == 1479,
        data_039().subrange(511, 800) == window_039_2(),
        window_039_2().len() == 289,
{
    hide(data_039);
    hide(ch::script_html_bytes);
    shape_039();
    assert(ch::script_html_bytes().subrange(511, 800) == bytes_039_2()) by (compute_only);
    assert(bytes_039_2().len() == 289) by (compute_only);
    scan::byte_codes_subrange(ch::script_html_bytes(), 511, 800);
    assert(data_039() == scan::byte_codes(ch::script_html_bytes())) by {
        reveal(data_039);
    };
    assert(window_039_2() == scan::byte_codes(bytes_039_2())) by {
        reveal(window_039_2);
    };
}

pub proof fn check_039_2()
    ensures
        scan::clean_range(
            data_039(),
            p::css_codes(),
            p::marketing_codes(),
            p::relative_codes(),
            512,
            768,
        ),
{
    hide(data_039);
    hide(scan::clean_range);
    hide(p::css_codes);
    hide(p::marketing_codes);
    hide(p::relative_codes);
    bind_039_2();
    p::bounds();
    assert(scan::clean_range(
        window_039_2(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        1,
        257,
    )) by (compute_only);
    scan::clean_window(
        data_039(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        511,
        800,
        512,
        768,
    );
}

} // verus!
    }
    pub mod window_3 {
        #[cfg(verus_keep_ghost)]
        use super::{data_039, shape_039};
        use crate::{
            k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
            k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
        };
        use ckc_spec::ui as u;
        use vstd::prelude::*;
        use vstd::utf8::*;
        verus! {

pub closed spec fn bytes_039_3() -> u::Bytes {
    ns![105u8,102,32,40,98,111,120,46,99,104,101,99,107,101,100,32,61,61,61,32,102,97,108,115,101,41,32,123,32,114,101,116,117,114,110,59,32,125,10,105,102,32,40,112,105,99,107,101,100,32,61,61,61,32,110,97,109,101,41,32,123,32,99,108,101,97,114,80,105,99,107,40,41,59,32,114,101,116,117,114,110,59,32,125,10,99,108,101,97,114,80,105,99,107,40,41,59,10,118,97,114,32,109,97,114,107,115,32,61,32,100,111,99,117,109,101,110,116,46,113,117,101,114,121,83,101,108,101,99,116,111,114,65,108,108,40,34,109,97,114,107,46,34,32,43,32,110,97,109,101,41,59,10,118,97,114,32,105,32,61,32,48,59,10,119,104,105,108,101,32,40,105,32,60,32,109,97,114,107,115,46,108,101,110,103,116,104,41,32,123,32,109,97,114,107,115,91,105,93,46,99,108,97,115,115,76,105,115,116,46,97,100,100,40,34,104,108,45,112,105,99,107,34,41,59,32,105,32,43,61,32,49,59,32,125,10,100,111,99,117,109,101,110,116,46,103,101,116,69,108,101,109,101,110,116,66,121,73,100,40,34,109,97,105,110,34,41,46,99,108,97,115,115,76,105,115,116,46,97,100,100,40,34,104,108,45,99,108,105]
}

pub closed spec fn window_039_3() -> Seq<nat> {
    scan::byte_codes(bytes_039_3())
}

pub proof fn bind_039_3()
    ensures
        data_039().len() == 1479,
        data_039().subrange(767, 1056) == window_039_3(),
        window_039_3().len() == 289,
{
    hide(data_039);
    hide(ch::script_html_bytes);
    shape_039();
    assert(ch::script_html_bytes().subrange(767, 1056) == bytes_039_3()) by (compute_only);
    assert(bytes_039_3().len() == 289) by (compute_only);
    scan::byte_codes_subrange(ch::script_html_bytes(), 767, 1056);
    assert(data_039() == scan::byte_codes(ch::script_html_bytes())) by {
        reveal(data_039);
    };
    assert(window_039_3() == scan::byte_codes(bytes_039_3())) by {
        reveal(window_039_3);
    };
}

pub proof fn check_039_3()
    ensures
        scan::clean_range(
            data_039(),
            p::css_codes(),
            p::marketing_codes(),
            p::relative_codes(),
            768,
            1024,
        ),
{
    hide(data_039);
    hide(scan::clean_range);
    hide(p::css_codes);
    hide(p::marketing_codes);
    hide(p::relative_codes);
    bind_039_3();
    p::bounds();
    assert(scan::clean_range(
        window_039_3(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        1,
        257,
    )) by (compute_only);
    scan::clean_window(
        data_039(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        767,
        1056,
        768,
        1024,
    );
}

} // verus!
    }
    pub mod window_4 {
        #[cfg(verus_keep_ghost)]
        use super::{data_039, shape_039};
        use crate::{
            k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
            k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
        };
        use ckc_spec::ui as u;
        use vstd::prelude::*;
        use vstd::utf8::*;
        verus! {

pub closed spec fn bytes_039_4() -> u::Bytes {
    ns![121u8,73,100,40,34,109,97,105,110,34,41,46,99,108,97,115,115,76,105,115,116,46,97,100,100,40,34,104,108,45,99,108,105,99,107,34,41,59,10,112,105,99,107,101,100,32,61,32,110,97,109,101,59,10,125,41,59,10,100,111,99,117,109,101,110,116,46,97,100,100,69,118,101,110,116,76,105,115,116,101,110,101,114,40,34,109,111,117,115,101,111,117,116,34,44,32,102,117,110,99,116,105,111,110,32,40,101,118,41,32,123,10,105,102,32,40,112,105,99,107,101,100,32,61,61,61,32,34,34,41,32,123,32,114,101,116,117,114,110,59,32,125,10,105,102,32,40,103,114,111,117,112,79,102,40,101,118,46,116,97,114,103,101,116,41,32,33,61,61,32,112,105,99,107,101,100,41,32,123,32,114,101,116,117,114,110,59,32,125,10,105,102,32,40,103,114,111,117,112,79,102,40,101,118,46,114,101,108,97,116,101,100,84,97,114,103,101,116,41,32,61,61,61,32,112,105,99,107,101,100,41,32,123,32,114,101,116,117,114,110,59,32,125,10,99,108,101,97,114,80,105,99,107,40,41,59,10,125,41,59,10,100,111,99,117,109,101,110,116,46,97,100,100,69,118,101,110,116,76,105,115,116,101,110,101,114,40,34,99]
}

pub closed spec fn window_039_4() -> Seq<nat> {
    scan::byte_codes(bytes_039_4())
}

pub proof fn bind_039_4()
    ensures
        data_039().len() == 1479,
        data_039().subrange(1023, 1312) == window_039_4(),
        window_039_4().len() == 289,
{
    hide(data_039);
    hide(ch::script_html_bytes);
    shape_039();
    assert(ch::script_html_bytes().subrange(1023, 1312) == bytes_039_4()) by (compute_only);
    assert(bytes_039_4().len() == 289) by (compute_only);
    scan::byte_codes_subrange(ch::script_html_bytes(), 1023, 1312);
    assert(data_039() == scan::byte_codes(ch::script_html_bytes())) by {
        reveal(data_039);
    };
    assert(window_039_4() == scan::byte_codes(bytes_039_4())) by {
        reveal(window_039_4);
    };
}

pub proof fn check_039_4()
    ensures
        scan::clean_range(
            data_039(),
            p::css_codes(),
            p::marketing_codes(),
            p::relative_codes(),
            1024,
            1280,
        ),
{
    hide(data_039);
    hide(scan::clean_range);
    hide(p::css_codes);
    hide(p::marketing_codes);
    hide(p::relative_codes);
    bind_039_4();
    p::bounds();
    assert(scan::clean_range(
        window_039_4(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        1,
        257,
    )) by (compute_only);
    scan::clean_window(
        data_039(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        1023,
        1312,
        1024,
        1280,
    );
}

} // verus!
    }
    pub mod window_5 {
        #[cfg(verus_keep_ghost)]
        use super::{data_039, shape_039};
        use crate::{
            k5_sound_chrome as ch, k5_sound_codes as n, k5_sound_encoding as enc,
            k5_sound_literals as l, k5_sound_patterns as p, k5_sound_scan as scan,
        };
        use ckc_spec::ui as u;
        use vstd::prelude::*;
        use vstd::utf8::*;
        verus! {

pub closed spec fn bytes_039_5() -> u::Bytes {
    ns![10u8,125,41,59,10,100,111,99,117,109,101,110,116,46,97,100,100,69,118,101,110,116,76,105,115,116,101,110,101,114,40,34,99,104,97,110,103,101,34,44,32,102,117,110,99,116,105,111,110,32,40,101,118,41,32,123,10,118,97,114,32,98,111,120,32,61,32,116,111,103,103,108,101,66,111,120,40,41,59,10,105,102,32,40,98,111,120,32,61,61,61,32,110,117,108,108,41,32,123,32,114,101,116,117,114,110,59,32,125,10,105,102,32,40,101,118,46,116,97,114,103,101,116,32,61,61,61,32,98,111,120,41,32,123,32,105,102,32,40,98,111,120,46,99,104,101,99,107,101,100,32,61,61,61,32,102,97,108,115,101,41,32,123,32,99,108,101,97,114,80,105,99,107,40,41,59,32,125,32,125,10,125,41,59,10,125,41,40,41,59,10,60,47,115,99,114,105,112,116,62]
}

pub closed spec fn window_039_5() -> Seq<nat> {
    scan::byte_codes(bytes_039_5())
}

pub proof fn bind_039_5()
    ensures
        data_039().len() == 1479,
        data_039().subrange(1279, 1479) == window_039_5(),
        window_039_5().len() == 200,
{
    hide(data_039);
    hide(ch::script_html_bytes);
    shape_039();
    assert(ch::script_html_bytes().subrange(1279, 1479) == bytes_039_5()) by (compute_only);
    assert(bytes_039_5().len() == 200) by (compute_only);
    scan::byte_codes_subrange(ch::script_html_bytes(), 1279, 1479);
    assert(data_039() == scan::byte_codes(ch::script_html_bytes())) by {
        reveal(data_039);
    };
    assert(window_039_5() == scan::byte_codes(bytes_039_5())) by {
        reveal(window_039_5);
    };
}

pub proof fn check_039_5()
    ensures
        scan::clean_range(
            data_039(),
            p::css_codes(),
            p::marketing_codes(),
            p::relative_codes(),
            1280,
            1479,
        ),
{
    hide(data_039);
    hide(scan::clean_range);
    hide(p::css_codes);
    hide(p::marketing_codes);
    hide(p::relative_codes);
    bind_039_5();
    p::bounds();
    assert(scan::clean_range(
        window_039_5(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        1,
        200,
    )) by (compute_only);
    scan::clean_window(
        data_039(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        1279,
        1479,
        1280,
        1479,
    );
}

} // verus!
    }
    verus! {

pub open spec fn data_039() -> Seq<nat> {
    scan::byte_codes(ch::script_html_bytes())
}

pub proof fn shape_039()
    ensures
        data_039().len() == 1479,
        ch::script_html_bytes().len() == 1479,
{
    hide(ch::script_html_bytes);
    assert(ch::script_html_bytes().len() == 1479) by (compute_only);
}

pub proof fn clean_039()
    ensures
        n::clean(data_039(), p::css_codes(), p::marketing_codes(), p::relative_codes()),
{
    hide(data_039);
    hide(scan::clean_range);
    hide(n::clean);
    shape_039();
    scan::clean_empty(data_039(), p::css_codes(), p::marketing_codes(), p::relative_codes(), 0);
    window_0::check_039_0();
    scan::clean_join(
        data_039(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        0,
        0,
        256,
    );
    window_1::check_039_1();
    scan::clean_join(
        data_039(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        0,
        256,
        512,
    );
    window_2::check_039_2();
    scan::clean_join(
        data_039(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        0,
        512,
        768,
    );
    window_3::check_039_3();
    scan::clean_join(
        data_039(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        0,
        768,
        1024,
    );
    window_4::check_039_4();
    scan::clean_join(
        data_039(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        0,
        1024,
        1280,
    );
    window_5::check_039_5();
    scan::clean_join(
        data_039(),
        p::css_codes(),
        p::marketing_codes(),
        p::relative_codes(),
        0,
        1280,
        1479,
    );
    scan::clean_full(data_039(), p::css_codes(), p::marketing_codes(), p::relative_codes());
}

pub proof fn literal_039()
    ensures
        u::copy_literal_ok(u::copy_registry()[39]),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    hide(u::copy_literal_ok);
    hide(n::clean);
    hide(u::script_html);
    hide(ch::script_html_bytes);
    hide(enc::script_html_chars);
    l::l039(Seq::empty());
    p::all();
    enc::script_html_shape();
    enc::script_html_encoding();
    assert forall|j: int| 0 <= j < enc::script_html_chars().len() implies '\x00'
        <= #[trigger] enc::script_html_chars()[j] <= '\x7f' by {
        enc::script_html_range_0_24(j);
    }
    scan::ascii_input(enc::script_html_chars());
    assert(n::codes(enc::script_html_chars()) == data_039());
    clean_039();
    n::clean_bridge(enc::script_html_chars());
}

} // verus!
}
verus! {

pub proof fn block_000(i: int)
    requires
        0 <= i < 16,
    ensures
        u::copy_literal_ok(u::copy_registry()[i]),
{
    hide(u::copy_registry);
    hide(u::copy_literal_ok);
    if i == 0 {
        batch_000::literal_000();
    } else if i == 1 {
        batch_000::literal_001();
    } else if i == 2 {
        batch_000::literal_002();
    } else if i == 3 {
        batch_000::literal_003();
    } else if i == 4 {
        batch_000::literal_004();
    } else if i == 5 {
        batch_000::literal_005();
    } else if i == 6 {
        batch_000::literal_006();
    } else if i == 7 {
        batch_000::literal_007();
    } else if i == 8 {
        batch_000::literal_008();
    } else if i == 9 {
        batch_000::literal_009();
    } else if i == 10 {
        batch_000::literal_010();
    } else if i == 11 {
        batch_000::literal_011();
    } else if i == 12 {
        batch_000::literal_012();
    } else if i == 13 {
        batch_000::literal_013();
    } else if i == 14 {
        batch_000::literal_014();
    } else {
        batch_000::literal_015();
    }
}

pub proof fn block_016(i: int)
    requires
        16 <= i < 32,
    ensures
        u::copy_literal_ok(u::copy_registry()[i]),
{
    hide(u::copy_registry);
    hide(u::copy_literal_ok);
    if i == 16 {
        batch_016::literal_016();
    } else if i == 17 {
        batch_016::literal_017();
    } else if i == 18 {
        batch_016::literal_018();
    } else if i == 19 {
        batch_016::literal_019();
    } else if i == 20 {
        batch_016::literal_020();
    } else if i == 21 {
        batch_016::literal_021();
    } else if i == 22 {
        batch_016::literal_022();
    } else if i == 23 {
        batch_016::literal_023();
    } else if i == 24 {
        batch_016::literal_024();
    } else if i == 25 {
        batch_016::literal_025();
    } else if i == 26 {
        batch_016::literal_026();
    } else if i == 27 {
        batch_016::literal_027();
    } else if i == 28 {
        batch_016::literal_028();
    } else if i == 29 {
        batch_016::literal_029();
    } else if i == 30 {
        batch_016::literal_030();
    } else {
        batch_016::literal_031();
    }
}

pub proof fn block_032(i: int)
    requires
        32 <= i < 48,
    ensures
        u::copy_literal_ok(u::copy_registry()[i]),
{
    hide(u::copy_registry);
    hide(u::copy_literal_ok);
    if i == 32 {
        batch_032::literal_032();
    } else if i == 33 {
        batch_032::literal_033();
    } else if i == 34 {
        batch_032::literal_034();
    } else if i == 35 {
        batch_032::literal_035();
    } else if i == 36 {
        batch_032::literal_036();
    } else if i == 37 {
        batch_032::literal_037();
    } else if i == 38 {
        css::literal_038();
    } else if i == 39 {
        script::literal_039();
    } else if i == 40 {
        batch_032::literal_040();
    } else if i == 41 {
        batch_032::literal_041();
    } else if i == 42 {
        batch_032::literal_042();
    } else if i == 43 {
        batch_032::literal_043();
    } else if i == 44 {
        batch_032::literal_044();
    } else if i == 45 {
        batch_032::literal_045();
    } else if i == 46 {
        batch_032::literal_046();
    } else {
        batch_032::literal_047();
    }
}

pub proof fn block_048(i: int)
    requires
        48 <= i < 64,
    ensures
        u::copy_literal_ok(u::copy_registry()[i]),
{
    hide(u::copy_registry);
    hide(u::copy_literal_ok);
    if i == 48 {
        batch_048::literal_048();
    } else if i == 49 {
        batch_048::literal_049();
    } else if i == 50 {
        batch_048::literal_050();
    } else if i == 51 {
        batch_048::literal_051();
    } else if i == 52 {
        batch_048::literal_052();
    } else if i == 53 {
        batch_048::literal_053();
    } else if i == 54 {
        batch_048::literal_054();
    } else if i == 55 {
        batch_048::literal_055();
    } else if i == 56 {
        batch_048::literal_056();
    } else if i == 57 {
        batch_048::literal_057();
    } else if i == 58 {
        batch_048::literal_058();
    } else if i == 59 {
        batch_048::literal_059();
    } else if i == 60 {
        batch_048::literal_060();
    } else if i == 61 {
        batch_048::literal_061();
    } else if i == 62 {
        batch_048::literal_062();
    } else {
        batch_048::literal_063();
    }
}

pub proof fn block_064(i: int)
    requires
        64 <= i < 80,
    ensures
        u::copy_literal_ok(u::copy_registry()[i]),
{
    hide(u::copy_registry);
    hide(u::copy_literal_ok);
    if i == 64 {
        batch_064::literal_064();
    } else if i == 65 {
        batch_064::literal_065();
    } else if i == 66 {
        batch_064::literal_066();
    } else if i == 67 {
        batch_064::literal_067();
    } else if i == 68 {
        batch_064::literal_068();
    } else if i == 69 {
        batch_064::literal_069();
    } else if i == 70 {
        batch_064::literal_070();
    } else if i == 71 {
        batch_064::literal_071();
    } else if i == 72 {
        batch_064::literal_072();
    } else if i == 73 {
        batch_064::literal_073();
    } else if i == 74 {
        batch_064::literal_074();
    } else if i == 75 {
        batch_064::literal_075();
    } else if i == 76 {
        batch_064::literal_076();
    } else if i == 77 {
        batch_064::literal_077();
    } else if i == 78 {
        batch_064::literal_078();
    } else {
        batch_064::literal_079();
    }
}

pub proof fn block_080(i: int)
    requires
        80 <= i < 96,
    ensures
        u::copy_literal_ok(u::copy_registry()[i]),
{
    hide(u::copy_registry);
    hide(u::copy_literal_ok);
    if i == 80 {
        batch_080::literal_080();
    } else if i == 81 {
        batch_080::literal_081();
    } else if i == 82 {
        batch_080::literal_082();
    } else if i == 83 {
        batch_080::literal_083();
    } else if i == 84 {
        batch_080::literal_084();
    } else if i == 85 {
        batch_080::literal_085();
    } else if i == 86 {
        batch_080::literal_086();
    } else if i == 87 {
        batch_080::literal_087();
    } else if i == 88 {
        batch_080::literal_088();
    } else if i == 89 {
        batch_080::literal_089();
    } else if i == 90 {
        batch_080::literal_090();
    } else if i == 91 {
        batch_080::literal_091();
    } else if i == 92 {
        batch_080::literal_092();
    } else if i == 93 {
        batch_080::literal_093();
    } else if i == 94 {
        batch_080::literal_094();
    } else {
        batch_080::literal_095();
    }
}

pub proof fn block_096(i: int)
    requires
        96 <= i < 112,
    ensures
        u::copy_literal_ok(u::copy_registry()[i]),
{
    hide(u::copy_registry);
    hide(u::copy_literal_ok);
    if i == 96 {
        batch_096::literal_096();
    } else if i == 97 {
        batch_096::literal_097();
    } else if i == 98 {
        batch_096::literal_098();
    } else if i == 99 {
        batch_096::literal_099();
    } else if i == 100 {
        batch_096::literal_100();
    } else if i == 101 {
        batch_096::literal_101();
    } else if i == 102 {
        batch_096::literal_102();
    } else if i == 103 {
        batch_096::literal_103();
    } else if i == 104 {
        batch_096::literal_104();
    } else if i == 105 {
        batch_096::literal_105();
    } else if i == 106 {
        batch_096::literal_106();
    } else if i == 107 {
        batch_096::literal_107();
    } else if i == 108 {
        batch_096::literal_108();
    } else if i == 109 {
        batch_096::literal_109();
    } else if i == 110 {
        batch_096::literal_110();
    } else {
        batch_096::literal_111();
    }
}

pub proof fn block_112(i: int)
    requires
        112 <= i < 128,
    ensures
        u::copy_literal_ok(u::copy_registry()[i]),
{
    hide(u::copy_registry);
    hide(u::copy_literal_ok);
    if i == 112 {
        batch_112::literal_112();
    } else if i == 113 {
        batch_112::literal_113();
    } else if i == 114 {
        batch_112::literal_114();
    } else if i == 115 {
        batch_112::literal_115();
    } else if i == 116 {
        batch_112::literal_116();
    } else if i == 117 {
        batch_112::literal_117();
    } else if i == 118 {
        batch_112::literal_118();
    } else if i == 119 {
        batch_112::literal_119();
    } else if i == 120 {
        batch_112::literal_120();
    } else if i == 121 {
        batch_112::literal_121();
    } else if i == 122 {
        batch_112::literal_122();
    } else if i == 123 {
        batch_112::literal_123();
    } else if i == 124 {
        batch_112::literal_124();
    } else if i == 125 {
        batch_112::literal_125();
    } else if i == 126 {
        batch_112::literal_126();
    } else {
        batch_112::literal_127();
    }
}

pub proof fn block_128(i: int)
    requires
        128 <= i < 144,
    ensures
        u::copy_literal_ok(u::copy_registry()[i]),
{
    hide(u::copy_registry);
    hide(u::copy_literal_ok);
    if i == 128 {
        batch_128::literal_128();
    } else if i == 129 {
        batch_128::literal_129();
    } else if i == 130 {
        batch_128::literal_130();
    } else if i == 131 {
        batch_128::literal_131();
    } else if i == 132 {
        batch_128::literal_132();
    } else if i == 133 {
        batch_128::literal_133();
    } else if i == 134 {
        batch_128::literal_134();
    } else if i == 135 {
        batch_128::literal_135();
    } else if i == 136 {
        batch_128::literal_136();
    } else if i == 137 {
        batch_128::literal_137();
    } else if i == 138 {
        batch_128::literal_138();
    } else if i == 139 {
        batch_128::literal_139();
    } else if i == 140 {
        batch_128::literal_140();
    } else if i == 141 {
        batch_128::literal_141();
    } else if i == 142 {
        batch_128::literal_142();
    } else {
        batch_128::literal_143();
    }
}

pub proof fn block_144(i: int)
    requires
        144 <= i < 160,
    ensures
        u::copy_literal_ok(u::copy_registry()[i]),
{
    hide(u::copy_registry);
    hide(u::copy_literal_ok);
    if i == 144 {
        batch_144::literal_144();
    } else if i == 145 {
        batch_144::literal_145();
    } else if i == 146 {
        batch_144::literal_146();
    } else if i == 147 {
        batch_144::literal_147();
    } else if i == 148 {
        batch_144::literal_148();
    } else if i == 149 {
        batch_144::literal_149();
    } else if i == 150 {
        batch_144::literal_150();
    } else if i == 151 {
        batch_144::literal_151();
    } else if i == 152 {
        batch_144::literal_152();
    } else if i == 153 {
        batch_144::literal_153();
    } else if i == 154 {
        batch_144::literal_154();
    } else if i == 155 {
        batch_144::literal_155();
    } else if i == 156 {
        batch_144::literal_156();
    } else if i == 157 {
        batch_144::literal_157();
    } else if i == 158 {
        batch_144::literal_158();
    } else {
        batch_144::literal_159();
    }
}

pub proof fn block_160(i: int)
    requires
        160 <= i < 176,
    ensures
        u::copy_literal_ok(u::copy_registry()[i]),
{
    hide(u::copy_registry);
    hide(u::copy_literal_ok);
    if i == 160 {
        batch_160::literal_160();
    } else if i == 161 {
        batch_160::literal_161();
    } else if i == 162 {
        batch_160::literal_162();
    } else if i == 163 {
        batch_160::literal_163();
    } else if i == 164 {
        batch_160::literal_164();
    } else if i == 165 {
        batch_160::literal_165();
    } else if i == 166 {
        batch_160::literal_166();
    } else if i == 167 {
        batch_160::literal_167();
    } else if i == 168 {
        batch_160::literal_168();
    } else if i == 169 {
        batch_160::literal_169();
    } else if i == 170 {
        batch_160::literal_170();
    } else if i == 171 {
        batch_160::literal_171();
    } else if i == 172 {
        batch_160::literal_172();
    } else if i == 173 {
        batch_160::literal_173();
    } else if i == 174 {
        batch_160::literal_174();
    } else {
        batch_160::literal_175();
    }
}

pub proof fn block_176(i: int)
    requires
        176 <= i < 192,
    ensures
        u::copy_literal_ok(u::copy_registry()[i]),
{
    hide(u::copy_registry);
    hide(u::copy_literal_ok);
    if i == 176 {
        batch_176::literal_176();
    } else if i == 177 {
        batch_176::literal_177();
    } else if i == 178 {
        batch_176::literal_178();
    } else if i == 179 {
        batch_176::literal_179();
    } else if i == 180 {
        batch_176::literal_180();
    } else if i == 181 {
        batch_176::literal_181();
    } else if i == 182 {
        batch_176::literal_182();
    } else if i == 183 {
        batch_176::literal_183();
    } else if i == 184 {
        batch_176::literal_184();
    } else if i == 185 {
        batch_176::literal_185();
    } else if i == 186 {
        batch_176::literal_186();
    } else if i == 187 {
        batch_176::literal_187();
    } else if i == 188 {
        batch_176::literal_188();
    } else if i == 189 {
        batch_176::literal_189();
    } else if i == 190 {
        batch_176::literal_190();
    } else {
        batch_176::literal_191();
    }
}

pub proof fn block_192(i: int)
    requires
        192 <= i < 208,
    ensures
        u::copy_literal_ok(u::copy_registry()[i]),
{
    hide(u::copy_registry);
    hide(u::copy_literal_ok);
    if i == 192 {
        batch_192::literal_192();
    } else if i == 193 {
        batch_192::literal_193();
    } else if i == 194 {
        batch_192::literal_194();
    } else if i == 195 {
        batch_192::literal_195();
    } else if i == 196 {
        batch_192::literal_196();
    } else if i == 197 {
        batch_192::literal_197();
    } else if i == 198 {
        batch_192::literal_198();
    } else if i == 199 {
        batch_192::literal_199();
    } else if i == 200 {
        batch_192::literal_200();
    } else if i == 201 {
        batch_192::literal_201();
    } else if i == 202 {
        batch_192::literal_202();
    } else if i == 203 {
        batch_192::literal_203();
    } else if i == 204 {
        batch_192::literal_204();
    } else if i == 205 {
        batch_192::literal_205();
    } else if i == 206 {
        batch_192::literal_206();
    } else {
        batch_192::literal_207();
    }
}

pub proof fn block_208(i: int)
    requires
        208 <= i < 224,
    ensures
        u::copy_literal_ok(u::copy_registry()[i]),
{
    hide(u::copy_registry);
    hide(u::copy_literal_ok);
    if i == 208 {
        batch_208::literal_208();
    } else if i == 209 {
        batch_208::literal_209();
    } else if i == 210 {
        batch_208::literal_210();
    } else if i == 211 {
        batch_208::literal_211();
    } else if i == 212 {
        batch_208::literal_212();
    } else if i == 213 {
        batch_208::literal_213();
    } else if i == 214 {
        batch_208::literal_214();
    } else if i == 215 {
        batch_208::literal_215();
    } else if i == 216 {
        batch_208::literal_216();
    } else if i == 217 {
        batch_208::literal_217();
    } else if i == 218 {
        batch_208::literal_218();
    } else if i == 219 {
        batch_208::literal_219();
    } else if i == 220 {
        batch_208::literal_220();
    } else if i == 221 {
        batch_208::literal_221();
    } else if i == 222 {
        batch_208::literal_222();
    } else {
        batch_208::literal_223();
    }
}

pub proof fn block_224(i: int)
    requires
        224 <= i < 239,
    ensures
        u::copy_literal_ok(u::copy_registry()[i]),
{
    hide(u::copy_registry);
    hide(u::copy_literal_ok);
    if i == 224 {
        batch_224::literal_224();
    } else if i == 225 {
        batch_224::literal_225();
    } else if i == 226 {
        batch_224::literal_226();
    } else if i == 227 {
        batch_224::literal_227();
    } else if i == 228 {
        batch_224::literal_228();
    } else if i == 229 {
        batch_224::literal_229();
    } else if i == 230 {
        batch_224::literal_230();
    } else if i == 231 {
        batch_224::literal_231();
    } else if i == 232 {
        batch_224::literal_232();
    } else if i == 233 {
        batch_224::literal_233();
    } else if i == 234 {
        batch_224::literal_234();
    } else if i == 235 {
        batch_224::literal_235();
    } else if i == 236 {
        batch_224::literal_236();
    } else if i == 237 {
        batch_224::literal_237();
    } else {
        batch_224::literal_238();
    }
}

pub proof fn literal(i: int)
    requires
        0 <= i < 239,
    ensures
        u::copy_literal_ok(u::copy_registry()[i]),
{
    hide(u::copy_registry);
    hide(u::copy_literal_ok);
    if i < 16 {
        block_000(i);
    } else if i < 32 {
        block_016(i);
    } else if i < 48 {
        block_032(i);
    } else if i < 64 {
        block_048(i);
    } else if i < 80 {
        block_064(i);
    } else if i < 96 {
        block_080(i);
    } else if i < 112 {
        block_096(i);
    } else if i < 128 {
        block_112(i);
    } else if i < 144 {
        block_128(i);
    } else if i < 160 {
        block_144(i);
    } else if i < 176 {
        block_160(i);
    } else if i < 192 {
        block_176(i);
    } else if i < 208 {
        block_192(i);
    } else if i < 224 {
        block_208(i);
    } else {
        block_224(i);
    }
}

pub proof fn registry_size()
    ensures
        u::copy_registry().len() == 239,
{
    hide(u::lit);
    hide(u::css_text);
    hide(u::script_html);
}

pub proof fn copy_ok()
    ensures
        u::copy_ok(u::copy_registry()),
{
    hide(u::copy_registry);
    hide(u::copy_literal_ok);
    registry_size();
    assert forall|i: int| 0 <= i < u::copy_registry().len() implies u::copy_literal_ok(
        #[trigger] u::copy_registry()[i],
    ) by {
        literal(i);
    }
}

} // verus!
