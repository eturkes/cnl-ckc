use crate::{k5_bytes as b, k5_codes as c, k5_html as h};
use ckc_spec::align::ESpan;
use ckc_spec::ui::{self as u, EPage};
use vstd::prelude::*;
use vstd::slice::slice_to_vec;
verus! {

pub fn token_byte(x: u8) -> (yes: bool)
    ensures
        yes == u::token_byte(x),
{
    (48 <= x && x <= 57) || (65 <= x && x <= 90) || (97 <= x && x <= 122)
}

pub fn token_end(s: &[u8], start: usize) -> (end: usize)
    requires
        start <= s.len(),
    ensures
        start <= end <= s.len(),
        end == u::token_end(s@, start as nat),
{
    let mut i = start;
    while i < s.len()
        invariant
            start <= i <= s.len(),
            u::token_end(s@, i as nat) == u::token_end(s@, start as nat),
        decreases s.len() - i,
    {
        if token_byte(s[i]) {
            i += 1;
        } else if s.len() - i > 1 && s[i] == 45 && token_byte(s[i + 1]) {
            i += 2;
        } else {
            return i;
        }
    }
    i
}

pub fn lower(s: &[u8]) -> (out: Vec<u8>)
    ensures
        out@ == u::lower(s@),
{
    let mut out = Vec::new();
    let mut i = 0;
    while i < s.len()
        invariant
            i <= s.len(),
            out@ == u::lower(s@.take(i as int)),
        decreases s.len() - i,
    {
        let x = s[i];
        out.push(
            if 65 <= x && x <= 90 {
                x - 65 + 97
            } else {
                x
            },
        );
        proof {
            assert(u::lower(s@.take(i as int + 1)) =~= u::lower(s@.take(i as int)).push(
                if 65 <= x <= 90 {
                    (x - 65 + 97) as u8
                } else {
                    x
                },
            ));
        }
        i += 1;
    }
    proof {
        assert(s@.take(i as int) =~= s@);
    }
    out
}

pub fn contains(xs: &Vec<Vec<u8>>, x: &[u8]) -> (yes: bool)
    ensures
        yes == b::views(xs@).contains(x@),
{
    let mut i = 0;
    while i < xs.len()
        invariant
            i <= xs.len(),
            forall|j: int| 0 <= j < i ==> #[trigger] b::views(xs@)[j] != x@,
        decreases xs.len() - i,
    {
        if b::equal(&xs[i], x) {
            proof {
                assert(b::views(xs@)[i as int] == x@);
            }
            return true;
        }
        i += 1;
    }
    false
}

pub fn stop_words() -> (out: Vec<Vec<u8>>)
    ensures
        b::views(out@) == u::stop_words(),
{
    let data = b::literal(
        "a an the every each no all some any this that these those such is are was were be been being has have had does do did should must may can cannot might will would shall could if then and or nor but not it its itself they them their he she who whom whose which what where when there something somebody someone everything everybody everyone nothing nobody of for with without during to at in on by from as against about after before through under over above below into onto per within between among around near than least most more less fewer greater",
    );
    b::split(&data, 32)
}

pub fn keyword(s: &[u8]) -> (out: EPage)
    ensures
        out@ == u::keyword_html(s@),
{
    let stops = stop_words();
    let mut out = h::empty();
    let mut i = 0;
    proof {
        assert(s@.skip(0) =~= s@);
    }
    while i < s.len()
        invariant
            i <= s.len(),
            b::views(stops@) == u::stop_words(),
            out@ + u::keyword_html(s@.skip(i as int)) == u::keyword_html(s@),
        decreases s.len() - i,
    {
        let tail = &s[i..s.len()];
        let ghost before = out@;
        let n = if token_byte(tail[0]) {
            let n = token_end(tail, 1);
            let word = &tail[0..n];
            proof {
                assert(word@ =~= tail@.take(n as int));
            }
            let low = lower(word);
            let piece = if contains(&stops, &low) {
                h::cat(h::cat(h::fixed("<span class=\"kw\">"), h::text(word)), h::fixed("</span>"))
            } else {
                h::text(word)
            };
            h::append(&mut out, piece);
            n
        } else {
            let byte = &tail[0..1];
            proof {
                assert(byte@ =~= seq![tail@[0]]);
            }
            h::append(&mut out, h::text(byte));
            1
        };
        proof {
            assert(out@ + u::keyword_html(tail@.skip(n as int)) == before + u::keyword_html(tail@));
            assert(tail@.skip(n as int) =~= s@.skip(i as int + n as int));
            assert(tail@ =~= s@.skip(i as int));
        }
        i += n;
    }
    out
}

pub fn slice_codes(cs: &Vec<u32>, start: u64, end: u64) -> (out: Vec<u8>)
    requires
        c::scalar_list(cs@),
    ensures
        out@ == if 0 <= start <= end <= c::cvs(cs@).len() {
            vstd::utf8::encode_utf8(c::cvs(cs@).subrange(start as int, end as int))
        } else {
            Seq::empty()
        },
{
    if start <= end && end <= cs.len() as u64 {
        let part = &cs[start as usize..end as usize];
        proof {
            assert(c::scalar_list(part@));
            assert(c::cvs(part@) =~= c::cvs(cs@).subrange(start as int, end as int));
        }
        c::encode(part)
    } else {
        Vec::new()
    }
}

pub open spec fn spans(xs: Seq<ESpan>) -> Seq<ckc_spec::align::OutSpan> {
    xs.map_values(|x: ESpan| x@)
}

pub fn marked(s: &[u8], ps: &Vec<ESpan>, keywords: bool) -> (out: EPage)
    ensures
        out@ == u::marked_html(s@, spans(ps@), keywords, 0),
{
    hide(u::keyword_html);
    hide(u::chars);
    hide(u::lit);
    hide(vstd::utf8::encode_utf8);
    let cs = c::decode(s);
    let mut out = h::empty();
    let mut i = 0;
    let mut cursor = 0u64;
    proof {
        assert(spans(ps@).skip(0) =~= spans(ps@));
    }
    while i < ps.len()
        invariant
            i <= ps.len(),
            c::scalar_list(cs@),
            c::cvs(cs@) == u::chars(s@),
            out@ + u::marked_html(s@, spans(ps@).skip(i as int), keywords, cursor as int)
                == u::marked_html(s@, spans(ps@), keywords, 0),
        decreases ps.len() - i,
    {
        let p = &ps[i];
        let gap = slice_codes(&cs, cursor, p.start);
        let part = slice_codes(&cs, p.start, p.end);
        let gap_page = if keywords {
            keyword(&gap)
        } else {
            h::text(&gap)
        };
        let begin = if p.index < 48 {
            let n = b::nat_bytes(p.index);
            h::cat(h::cat(h::fixed("<mark class=\"t"), h::attr(&n)), h::fixed("\">"))
        } else {
            h::fixed("<mark>")
        };
        let piece = h::cat(h::cat(h::cat(gap_page, begin), h::text(&part)), h::fixed("</mark>"));
        h::append(&mut out, piece);
        proof {
            assert(spans(ps@).skip(i as int).drop_first() =~= spans(ps@).skip(i as int + 1));
        }
        cursor = p.end;
        i += 1;
    }
    let rest = slice_codes(&cs, cursor, cs.len() as u64);
    h::append(
        &mut out,
        if keywords {
            keyword(&rest)
        } else {
            h::text(&rest)
        },
    );
    out
}

} // verus!
