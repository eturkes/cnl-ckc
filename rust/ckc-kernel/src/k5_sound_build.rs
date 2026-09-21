use crate::{k5_sound_chrome as chrome, k5_sound_escape as h, k5_sound_literals as l};
use ckc_spec::ui as u;
use vstd::prelude::*;
verus! {

broadcast use {vstd::seq::group_seq_axioms, vstd::seq_lib::group_seq_properties};

pub open spec fn c(i: int) -> u::Bytes {
    u::copy_registry()[i]
}

pub open spec fn f(i: int) -> u::Html {
    u::fixed_bytes(c(i))
}

pub proof fn add(a: u::Html, b: u::Html, inputs: Seq<u::Bytes>)
    requires
        h::sound(a, inputs),
        h::sound(b, inputs),
    ensures
        h::sound(a + b, inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    h::add(a, b, inputs, 0, 0, 0);
}

pub proof fn flatten(xs: Seq<u::Html>, inputs: Seq<u::Bytes>)
    requires
        forall|i: int| 0 <= i < xs.len() ==> h::sound(#[trigger] xs[i], inputs),
    ensures
        h::sound(xs.flatten(), inputs),
    decreases xs.len(),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    if xs.len() == 0 {
        h::empty(inputs, 0);
    } else {
        flatten(xs.drop_first(), inputs);
        add(xs[0], xs.drop_first().flatten(), inputs);
    }
}

pub proof fn hjoin(xs: Seq<u::Html>, sep: u::Html, inputs: Seq<u::Bytes>)
    requires
        h::sound(sep, inputs),
        forall|i: int| 0 <= i < xs.len() ==> h::sound(#[trigger] xs[i], inputs),
    ensures
        h::sound(u::hjoin(xs, sep), inputs),
    decreases xs.len(),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    if xs.len() == 0 {
        h::empty(inputs, 0);
    } else if xs.len() > 1 {
        hjoin(xs.drop_first(), sep, inputs);
        add(xs[0], sep, inputs);
        add(xs[0] + sep, u::hjoin(xs.drop_first(), sep), inputs);
    }
}

pub proof fn lines(xs: Seq<u::Html>, inputs: Seq<u::Bytes>)
    requires
        forall|i: int| 0 <= i < xs.len() ==> h::sound(#[trigger] xs[i], inputs),
    ensures
        h::sound(u::lines(xs), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l::f005(inputs, 0);
    hjoin(xs, u::fixed("\n"@), inputs);
}

pub proof fn flow(xs: Seq<u::Html>, cs: Seq<int>, inputs: Seq<u::Bytes>)
    requires
        cs.len() == xs.len() + 1,
        forall|i: int|
            0 <= i < xs.len() ==> h::fragment(#[trigger] xs[i], inputs, cs[i], cs[i + 1]),
    ensures
        h::fragment(u::lines(xs), inputs, cs[0], cs.last()),
    decreases xs.len(),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    if xs.len() == 0 {
        h::empty(inputs, cs[0]);
    } else if xs.len() > 1 {
        flow(xs.drop_first(), cs.drop_first(), inputs);
        l::f005(inputs, cs[1]);
        h::add(xs[0], u::fixed("\n"@), inputs, cs[0], cs[1], cs[1]);
        h::add(xs[0] + u::fixed("\n"@), u::lines(xs.drop_first()), inputs, cs[0], cs[1], cs.last());
    }
}

pub proof fn cell(x: u::Html, inputs: Seq<u::Bytes>)
    requires
        h::sound(x, inputs),
    ensures
        h::sound(u::cell(x), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l::f006(inputs);
    l::f007(inputs);
    add(f(6), x, inputs);
    add(f(6) + x, f(7), inputs);
}

pub proof fn row(xs: Seq<u::Html>, inputs: Seq<u::Bytes>)
    requires
        forall|i: int| 0 <= i < xs.len() ==> h::sound(#[trigger] xs[i], inputs),
    ensures
        h::sound(u::row(xs), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l::f008(inputs);
    l::f009(inputs);
    flatten(xs, inputs);
    add(f(8), xs.flatten(), inputs);
    add(f(8) + xs.flatten(), f(9), inputs);
}

pub proof fn link(href: u::Bytes, label: u::Html, inputs: Seq<u::Bytes>)
    requires
        h::sound(label, inputs),
    ensures
        h::sound(u::link(href, label), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l::f010(inputs);
    l::f011(inputs);
    l::f012(inputs);
    h::attr(href, inputs, 2);
    h::add(f(10), u::attr(href), inputs, 0, 2, 2);
    h::add(f(10) + u::attr(href), f(11), inputs, 0, 2, 0);
    add(f(10) + u::attr(href) + f(11), label, inputs);
    add(f(10) + u::attr(href) + f(11) + label, f(12), inputs);
}

pub proof fn state_label(k: int, inputs: Seq<u::Bytes>)
    ensures
        u::copy_derived(u::state_label(k), inputs, u::copy_registry()),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    if k == 0 {
        l::l018(inputs);
    } else if k == 1 {
        l::l019(inputs);
    } else if k == 2 {
        l::l020(inputs);
    } else if k == 3 {
        l::l021(inputs);
    } else {
        l::l022(inputs);
    }
}

pub proof fn chip(k: int, inputs: Seq<u::Bytes>)
    ensures
        h::sound(u::chip(k), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    state_label(k, inputs);
    h::text(u::state_label(k), inputs);
    l::f023(inputs);
    h::attr(u::state_name(k), inputs, 2);
    l::f011(inputs);
    l::f024(inputs);
    h::add(f(23), u::attr(u::state_name(k)), inputs, 0, 2, 2);
    h::add(f(23) + u::attr(u::state_name(k)), f(11), inputs, 0, 2, 0);
    add(f(23) + u::attr(u::state_name(k)) + f(11), u::text(u::state_label(k)), inputs);
    add(f(23) + u::attr(u::state_name(k)) + f(11) + u::text(u::state_label(k)), f(24), inputs);
}

pub proof fn frame(title: u::Bytes, crumbs: u::Html, body: u::Html, inputs: Seq<u::Bytes>)
    requires
        u::copy_derived(title, inputs, u::copy_registry()),
        h::sound(crumbs, inputs),
        h::sound(body, inputs),
    ensures
        h::sound(u::frame(title, crumbs, body), inputs),
{
    hide(u::copy_registry);
    hide(u::copy_derived);
    l::f040(inputs);
    l::f041(inputs);
    l::f042(inputs);
    l::f043(inputs);
    l::f044(inputs);
    l::f045(inputs);
    l::f046(inputs);
    l::f047(inputs);
    l::f048(inputs);
    l::f049(inputs);
    l::f050(inputs);
    l::f051(inputs);
    l::f052(inputs);
    l::f053(inputs);
    l::f054(inputs);
    l::f055(inputs);
    l::f056(inputs);
    l::f057(inputs);
    h::text(title, inputs);
    chrome::css_text(inputs);
    add(f(44), u::text(title), inputs);
    add(f(44) + u::text(title), f(45), inputs);
    add(f(51), crumbs, inputs);
    add(f(51) + crumbs, f(52), inputs);
    let xs = seq![
        f(40),
        f(41),
        f(42),
        f(43),
        f(44) + u::text(title) + f(45),
        f(46),
        u::fixed_bytes(u::css_text()),
        f(47),
        f(48),
        f(49),
        f(50),
        f(51) + crumbs + f(52),
        f(53),
        body,
        f(54),
        f(55),
        f(56),
        f(57),
    ];
    let cs = seq![0int, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    flow(xs, cs, inputs);
    l::f005(inputs, 0);
    add(u::lines(xs), u::fixed("\n"@), inputs);
    assert(u::frame(title, crumbs, body) =~= u::lines(xs) + u::fixed("\n"@));
}

} // verus!
