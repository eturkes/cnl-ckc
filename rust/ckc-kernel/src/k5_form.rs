use crate::{k5_bytes as b, k5_highlight as bytes, k5_utf8 as utf};
use ckc_spec::ui as u;
use vstd::prelude::*;
use vstd::slice::slice_to_vec;
verus! {

pub struct Fields {
    pub verdict: Vec<u8>,
    pub reviewer: Vec<u8>,
    pub comment: Vec<u8>,
    pub review: Vec<u8>,
    pub ledger: Vec<u8>,
    pub csrf: Vec<u8>,
}

impl View for Fields {
    type V = u::Fields;

    open spec fn view(&self) -> u::Fields {
        u::Fields {
            verdict: self.verdict@,
            reviewer: self.reviewer@,
            comment: self.comment@,
            review: self.review@,
            ledger: self.ledger@,
            csrf: self.csrf@,
        }
    }
}

pub open spec fn pairs(ps: Seq<(Vec<u8>, Vec<u8>)>) -> Seq<(u::Bytes, u::Bytes)> {
    ps.map_values(|p: (Vec<u8>, Vec<u8>)| (p.0@, p.1@))
}

pub open spec fn pair_result(r: Result<Vec<(Vec<u8>, Vec<u8>)>, Vec<u8>>) -> Result<
    Seq<(u::Bytes, u::Bytes)>,
    u::Bytes,
> {
    match r {
        Ok(ps) => Ok(pairs(ps@)),
        Err(e) => Err(e@),
    }
}

pub open spec fn field_result(r: Result<Fields, Vec<u8>>) -> Result<u::Fields, u::Bytes> {
    match r {
        Ok(f) => Ok(f@),
        Err(e) => Err(e@),
    }
}

pub fn hex_value(x: u8) -> (n: i16)
    ensures
        -1 <= n <= 15,
        n as int == u::hex_value(x),
{
    if 48 <= x && x <= 57 {
        x as i16 - 48
    } else if 65 <= x && x <= 70 {
        x as i16 - 65 + 10
    } else if 97 <= x && x <= 102 {
        x as i16 - 97 + 10
    } else {
        -1
    }
}

pub fn unquote(s: &[u8]) -> (out: Vec<u8>)
    ensures
        out@ == u::form_unquote(s@),
{
    let mut out = Vec::new();
    let mut i = 0;
    proof {
        assert(s@.skip(0) =~= s@);
    }
    while i < s.len()
        invariant
            i <= s.len(),
            out@ + u::form_unquote(s@.skip(i as int)) == u::form_unquote(s@),
        decreases s.len() - i,
    {
        let n = if s[i] == 43 {
            out.push(32);
            1
        } else if s.len() - i >= 3 && s[i] == 37 && hex_value(s[i + 1]) >= 0 && hex_value(s[i + 2])
            >= 0 {
            let a = hex_value(s[i + 1]);
            let z = hex_value(s[i + 2]);
            out.push((16 * a + z) as u8);
            3
        } else {
            out.push(s[i]);
            1
        };
        proof {
            assert(s@.skip(i as int).skip(n as int) =~= s@.skip(i as int + n as int));
        }
        i += n;
    }
    out
}

pub proof fn first_single(s: Seq<u8>, x: u8, i: nat)
    requires
        i <= s.len(),
    ensures
        ckc_spec::check::first_sub(s, seq![x], i) == ckc_spec::replay::first_byte(s, x, i),
    decreases s.len() - i,
{
    reveal_with_fuel(ckc_spec::check::first_sub, 2);
    reveal_with_fuel(ckc_spec::replay::first_byte, 2);
    let p = seq![x];
    assert(p.len() == 1);
    if i == s.len() {
        assert(ckc_spec::check::first_sub(s, p, i) == s.len());
        assert(ckc_spec::replay::first_byte(s, x, i) == s.len());
    } else if s[i as int] == x {
        assert(s.subrange(i as int, i as int + 1) =~= p);
        assert(ckc_spec::check::first_sub(s, p, i) == i);
        assert(ckc_spec::replay::first_byte(s, x, i) == i);
    } else {
        assert(s.subrange(i as int, i as int + 1)[0] == s[i as int]);
        assert(s.subrange(i as int, i as int + 1) != p);
        first_single(s, x, i + 1);
        assert(ckc_spec::check::first_sub(s, p, i) == ckc_spec::check::first_sub(s, p, i + 1));
        assert(ckc_spec::replay::first_byte(s, x, i) == ckc_spec::replay::first_byte(s, x, i + 1));
    }
}

pub fn form_pairs(xs: &Vec<Vec<u8>>) -> (out: Result<Vec<(Vec<u8>, Vec<u8>)>, Vec<u8>>)
    ensures
        pair_result(out) == u::form_pairs(b::views(xs@)),
{
    let mut ps = Vec::new();
    let mut i = 0;
    proof {
        assert(b::views(xs@).skip(0) =~= b::views(xs@));
        reveal_strlit("=");
        assert(vstd::utf8::is_ascii_chars("="@));
        vstd::utf8::is_ascii_chars_encode_utf8("="@);
        assert(u::lit("="@) =~= seq![61u8]);
    }
    while i < xs.len()
        invariant
            i <= xs.len(),
            u::lit("="@) == seq![61u8],
            u::form_pairs(b::views(xs@)) == match u::form_pairs(b::views(xs@).skip(i as int)) {
                Ok(t) => Ok(pairs(ps@) + t),
                Err(e) => Err(e),
            },
        decreases xs.len() - i,
    {
        let source = &xs[i];
        let n = b::first_byte(source, 61, 0);
        proof {
            first_single(source@, 61, 0);
        }
        if n == source.len() {
            return Err(b::literal("ui: verdict: body not parseable"));
        }
        proof {
            assert(source@.subrange(0, n as int) =~= source@.take(n as int));
            assert(source@.subrange(n as int + 1, source@.len() as int) =~= source@.skip(
                n as int + 1,
            ));
        }
        let key = unquote(&source[0..n]);
        let value = unquote(&source[n + 1..source.len()]);
        if !utf::valid(&key) || !utf::valid(&value) {
            return Err(b::literal("ui: verdict: body not parseable"));
        }
        let ghost before = pairs(ps@);
        let ghost pair = (key@, value@);
        ps.push((key, value));
        proof {
            assert(pairs(ps@) =~= before.push(pair));
            let rest = b::views(xs@).skip(i as int).drop_first();
            assert(rest =~= b::views(xs@).skip(i as int + 1));
            match u::form_pairs(rest) {
                Ok(t) => {
                    assert(pairs(ps@) + t =~= before + (seq![pair] + t));
                },
                Err(_) => {},
            }
            assert(u::form_pairs(b::views(xs@).skip(i as int)) == match u::form_pairs(rest) {
                Ok(t) => Ok(seq![pair] + t),
                Err(e) => Err(e),
            });
        }
        i += 1;
    }
    Ok(ps)
}

pub fn names() -> (out: Vec<Vec<u8>>)
    ensures
        b::views(out@) == u::field_names(),
{
    let mut out = Vec::new();
    out.push(b::literal("verdict"));
    out.push(b::literal("reviewer"));
    out.push(b::literal("comment"));
    out.push(b::literal("review_sha256"));
    out.push(b::literal("ledger_sha256"));
    out.push(b::literal("csrf"));
    proof {
        assert(b::views(out@) =~= u::field_names());
    }
    out
}

pub fn values(ps: &Vec<(Vec<u8>, Vec<u8>)>, key: &[u8]) -> (out: Vec<Vec<u8>>)
    ensures
        b::views(out@) == u::field_values(pairs(ps@), key@),
{
    let mut out = Vec::new();
    let mut i = 0;
    while i < ps.len()
        invariant
            i <= ps.len(),
            b::views(out@) == u::field_values(pairs(ps@).take(i as int), key@),
        decreases ps.len() - i,
    {
        let p = &ps[i];
        if b::equal(&p.0, key) {
            let ghost prior = b::views(out@);
            out.push(slice_to_vec(&p.1));
            proof {
                assert(b::views(out@) =~= prior.push(p.1@));
            }
        }
        proof {
            let before = pairs(ps@).take(i as int);
            assert(pairs(ps@).take(i as int + 1) =~= before.push((p.0@, p.1@)));
            before.lemma_filter_push((p.0@, p.1@), |p: (u::Bytes, u::Bytes)| p.0 == key@);
            assert(u::field_values(before.push((p.0@, p.1@)), key@) =~= if p.0@ == key@ {
                u::field_values(before, key@).push(p.1@)
            } else {
                u::field_values(before, key@)
            });
        }
        i += 1;
    }
    proof {
        assert(pairs(ps@).take(i as int) =~= pairs(ps@));
    }
    out
}

pub open spec fn name_pred(ps: Seq<(u::Bytes, u::Bytes)>, missing: bool) -> spec_fn(
    u::Bytes,
) -> bool {
    if missing {
        |n: u::Bytes| u::field_values(ps, n).len() == 0
    } else {
        |n: u::Bytes| u::field_values(ps, n).len() > 1
    }
}

pub fn selected(ps: &Vec<(Vec<u8>, Vec<u8>)>, ns: &Vec<Vec<u8>>, missing: bool) -> (out: Vec<
    Vec<u8>,
>)
    ensures
        b::views(out@) == b::views(ns@).filter(name_pred(pairs(ps@), missing)),
{
    hide(u::field_values);
    let mut out = Vec::new();
    let mut i = 0;
    while i < ns.len()
        invariant
            i <= ns.len(),
            b::views(out@) == b::views(ns@).take(i as int).filter(name_pred(pairs(ps@), missing)),
        decreases ns.len() - i,
    {
        let n = &ns[i];
        let vs = values(ps, n);
        if (missing && vs.len() == 0) || (!missing && vs.len() > 1) {
            out.push(slice_to_vec(n));
        }
        proof {
            let before = b::views(ns@).take(i as int);
            assert(b::views(ns@).take(i as int + 1) =~= before.push(n@));
            before.lemma_filter_push(n@, name_pred(pairs(ps@), missing));
        }
        i += 1;
    }
    proof {
        assert(b::views(ns@).take(i as int) =~= b::views(ns@));
    }
    out
}

pub fn unknown(ps: &Vec<(Vec<u8>, Vec<u8>)>, ns: &Vec<Vec<u8>>) -> (out: Vec<Vec<u8>>)
    ensures
        b::views(out@) == pairs(ps@).filter(
            |p: (u::Bytes, u::Bytes)| !b::views(ns@).contains(p.0),
        ).map_values(|p: (u::Bytes, u::Bytes)| p.0),
{
    let mut out = Vec::new();
    let mut i = 0;
    while i < ps.len()
        invariant
            i <= ps.len(),
            b::views(out@) == pairs(ps@).take(i as int).filter(
                |p: (u::Bytes, u::Bytes)| !b::views(ns@).contains(p.0),
            ).map_values(|p: (u::Bytes, u::Bytes)| p.0),
        decreases ps.len() - i,
    {
        let p = &ps[i];
        if !bytes::contains(ns, &p.0) {
            let ghost prior = b::views(out@);
            out.push(slice_to_vec(&p.0));
            proof {
                assert(b::views(out@) =~= prior.push(p.0@));
            }
        }
        proof {
            let before = pairs(ps@).take(i as int);
            assert(pairs(ps@).take(i as int + 1) =~= before.push((p.0@, p.1@)));
            before.lemma_filter_push(
                (p.0@, p.1@),
                |p: (u::Bytes, u::Bytes)| !b::views(ns@).contains(p.0),
            );
            let selected = before.filter(|q: (u::Bytes, u::Bytes)| !b::views(ns@).contains(q.0));
            assert(selected.push((p.0@, p.1@)).map_values(|q: (u::Bytes, u::Bytes)| q.0)
                =~= selected.map_values(|q: (u::Bytes, u::Bytes)| q.0).push(p.0@));
        }
        i += 1;
    }
    proof {
        assert(pairs(ps@).take(i as int) =~= pairs(ps@));
    }
    out
}

pub fn clean(s: &[u8]) -> (yes: bool)
    ensures
        yes == ckc_spec::check::text_clean(s@),
{
    let mut i = 0;
    while i < s.len()
        invariant
            i <= s.len(),
            forall|j: int| 0 <= j < i ==> #[trigger] s@[j] >= 32 && s@[j] != 127,
        decreases s.len() - i,
    {
        if s[i] < 32 || s[i] == 127 {
            return false;
        }
        i += 1;
    }
    true
}

pub fn hex64(s: &[u8]) -> (yes: bool)
    ensures
        yes == ckc_spec::v1text::hex64(s@),
{
    if s.len() != 64 {
        return false;
    }
    let mut i = 0;
    while i < s.len()
        invariant
            s.len() == 64,
            i <= s.len(),
            forall|j: int| 0 <= j < i ==> ckc_spec::v1text::is_hex_lower_b(#[trigger] s@[j]),
        decreases s.len() - i,
    {
        let x = s[i];
        if !((48 <= x && x <= 57) || (97 <= x && x <= 102)) {
            return false;
        }
        i += 1;
    }
    true
}

pub fn value(ps: &Vec<(Vec<u8>, Vec<u8>)>, key: &str) -> (out: Vec<u8>)
    ensures
        out@ == u::at(u::field_values(pairs(ps@), u::lit(key@)), 0),
{
    let key = b::literal(key);
    let vs = values(ps, &key);
    b::at(&vs, 0)
}

pub fn parse_fields(ps: &Vec<(Vec<u8>, Vec<u8>)>) -> (out: Result<Fields, Vec<u8>>)
    ensures
        field_result(out) == u::parse_fields(pairs(ps@)),
{
    hide(u::field_values);
    let ns = names();
    let missing = selected(ps, &ns, true);
    let duplicate = selected(ps, &ns, false);
    let unknown = unknown(ps, &ns);
    let ghost extras = pairs(ps@).filter(|p: (u::Bytes, u::Bytes)| !u::field_names().contains(p.0));
    proof {
        assert((|p: (u::Bytes, u::Bytes)| !b::views(ns@).contains(p.0)) =~= (|
            p: (u::Bytes, u::Bytes),
        |
            !u::field_names().contains(p.0)));
        assert(b::views(unknown@) == extras.map_values(|p: (u::Bytes, u::Bytes)| p.0));
        assert(unknown.len() == extras.len());
    }
    if missing.len() > 0 {
        return Err(b::cat(b::literal("ui: verdict: missing field "), &missing[0]));
    }
    if duplicate.len() > 0 {
        return Err(b::cat(b::literal("ui: verdict: duplicate field "), &duplicate[0]));
    }
    if unknown.len() > 0 {
        proof {
            assert(b::views(unknown@)[0] == unknown@[0]@);
            assert(extras.map_values(|p: (u::Bytes, u::Bytes)| p.0)[0] == extras[0].0);
            assert(unknown@[0]@ == extras[0].0);
        }
        return Err(b::cat(b::literal("ui: verdict: unknown field "), &unknown[0]));
    }
    let f = Fields {
        verdict: value(ps, "verdict"),
        reviewer: value(ps, "reviewer"),
        comment: value(ps, "comment"),
        review: value(ps, "review_sha256"),
        ledger: value(ps, "ledger_sha256"),
        csrf: value(ps, "csrf"),
    };
    if !b::equal(&f.verdict, &b::literal("approved")) && !b::equal(
        &f.verdict,
        &b::literal("rejected"),
    ) {
        return Err(b::literal("ui: verdict: invalid verdict"));
    }
    if f.reviewer.len() == 0 || !clean(&f.reviewer) {
        return Err(b::literal("ui: verdict: invalid reviewer"));
    }
    if !clean(&f.comment) {
        return Err(b::literal("ui: verdict: invalid comment"));
    }
    if !hex64(&f.review) {
        return Err(b::literal("ui: verdict: invalid review_sha256"));
    }
    if !b::equal(&f.ledger, &b::literal("absent")) && !hex64(&f.ledger) {
        return Err(b::literal("ui: verdict: invalid ledger_sha256"));
    }
    Ok(f)
}

pub fn parse(body: &[u8]) -> (out: Result<Fields, Vec<u8>>)
    ensures
        field_result(out) == u::parse_form(body@),
{
    hide(u::form_pairs);
    hide(u::parse_fields);
    if !utf::valid(body) {
        return Err(b::literal("ui: verdict: body not decodable"));
    }
    let xs = if body.len() == 0 {
        Vec::new()
    } else {
        b::split(body, 38)
    };
    proof {
        assert(b::views(xs@) =~= if body@.len() == 0 {
            Seq::empty()
        } else {
            ckc_spec::check::split_on(body@, 38)
        });
    }
    if xs.len() > 32 {
        return Err(b::literal("ui: verdict: body not parseable"));
    }
    match form_pairs(&xs) {
        Err(e) => Err(e),
        Ok(ps) => parse_fields(&ps),
    }
}

} // verus!
