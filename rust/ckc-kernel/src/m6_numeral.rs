use crate::arith_impl::{ERat, canon_push, decode_number, push_value};
use crate::arith_mag::{is_zero, mag_add, mag_sub};
use crate::k2_term::*;
use crate::m6_symbols::Sym;
use crate::m6_term::*;
#[cfg(verus_keep_ghost)]
use ckc_spec::arith as a;
use ckc_spec::emit as spec;
use ckc_spec::term::Term;
#[cfg(verus_keep_ghost)]
use ckc_spec::v1text::{dec_bytes, is_digit_b, udec_bytes};
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

// The literal law (contract da2 D2) over a sentence's tokens + ACE line and the
// operand witness (D4): the exec mirror of `emit.rs` § dose arithmetic formulas.
pub struct ESrc {
    pub toks: Vec<T>,
    pub line: Vec<u8>,
}

impl View for ESrc {
    type V = spec::Src;

    open spec fn view(&self) -> spec::Src {
        spec::Src { toks: models(self.toks@), line: self.line@ }
    }
}

pub open spec fn src_valid(nodes: Seq<ENode>, s: &ESrc) -> bool {
    valid_all(nodes, s.toks@)
}

pub open spec fn src_models(v: Seq<ESrc>) -> Seq<spec::Src> {
    v.map_values(|x: ESrc| x@)
}

pub open spec fn srcs_valid(nodes: Seq<ENode>, v: Seq<ESrc>) -> bool {
    forall|i: int| 0 <= i < v.len() ==> src_valid(nodes, #[trigger] &v[i])
}

pub proof fn srcs_prefix(before: Seq<ENode>, after: Seq<ENode>, v: Seq<ESrc>)
    requires
        before.is_prefix_of(after),
        srcs_valid(before, v),
    ensures
        srcs_valid(after, v),
{
    assert forall|i: int| 0 <= i < v.len() implies src_valid(after, #[trigger] &v[i]) by {
        prefix_all(before, after, v[i].toks@);
    }
}

pub open spec fn opt_seq(o: Option<Vec<u8>>) -> Option<Seq<u8>> {
    match o {
        Some(v) => Some(v@),
        None => None,
    }
}

fn atom_text(arena: &ETermArena, t: &T) -> (out: Option<Vec<u8>>)
    requires
        arena_ok(arena),
        valid(arena.nodes@, t),
    ensures
        opt_seq(out) == (match t@ {
            Term::Atom(x) => Option::Some(x),
            _ => Option::None,
        }),
{
    proof {
        assert(node_ok(arena.nodes@, t.root as int));
        reveal(node_ok);
    }
    match &arena.nodes[t.root].kind {
        ENodeKind::Atom { name } => Some(name.clone()),
        _ => None,
    }
}

pub fn numeral(arena: &ETermArena, t: &T) -> (out: Option<Vec<u8>>)
    requires
        arena_ok(arena),
        valid(arena.nodes@, t),
    ensures
        opt_seq(out) == spec::numeral(t@),
{
    proof {
        assert(node_ok(arena.nodes@, t.root as int));
        reveal(node_ok);
    }
    match &arena.nodes[t.root].kind {
        ENodeKind::Int { spelling, .. } => Some(spelling.clone()),
        ENodeKind::Comp { .. } => {
            if is_comp(arena, t, &Sym::DollarDecimal, 1) {
                let xs = args(arena, t);
                proof {
                    assert(valid(arena.nodes@, &xs@[0]));
                }
                atom_text(arena, &xs[0])
            } else {
                None
            }
        },
        _ => None,
    }
}

fn digit(b: u8) -> (r: bool)
    ensures
        r == is_digit_b(b),
{
    0x30 <= b && b <= 0x39
}

fn wordy(b: u8) -> (r: bool)
    ensures
        r == spec::wordy(b),
{
    (0x61 <= b && b <= 0x7A) || (0x41 <= b && b <= 0x5A) || digit(b) || b == 0x5F || b == 0x2D
}

fn digits_end(l: &Vec<u8>, i: usize) -> (out: usize)
    requires
        i <= l.len(),
    ensures
        out as int == spec::digits_end(l@, i as int),
        i <= out <= l.len(),
{
    let mut j = i;
    while j < l.len() && digit(l[j])
        invariant
            i <= j <= l.len(),
            spec::digits_end(l@, i as int) == spec::digits_end(l@, j as int),
        decreases l.len() - j,
    {
        j += 1;
    }
    j
}

pub fn run_end(l: &Vec<u8>, i: usize) -> (out: usize)
    requires
        i <= l.len(),
    ensures
        out as int == spec::run_end(l@, i as int),
        i <= out <= l.len(),
{
    let j = digits_end(l, i);
    if j < l.len() && l.len() - j >= 2 && l[j] == 0x2E && digit(l[j + 1]) {
        digits_end(l, j + 1)
    } else {
        j
    }
}

fn lexeme_at(l: &Vec<u8>, i: usize) -> (r: bool)
    requires
        i < l.len(),
    ensures
        r == spec::lexeme_at(l@, i as int),
{
    if !digit(l[i]) {
        return false;
    }
    if i > 0 && (wordy(l[i - 1]) || l[i - 1] == 0x2E) {
        return false;
    }
    let k = run_end(l, i);
    k == l.len() || !wordy(l[k])
}

pub open spec fn uints(v: Seq<usize>) -> Seq<int> {
    v.map_values(|i: usize| i as int)
}

pub fn lexeme_starts(l: &Vec<u8>) -> (out: Vec<usize>)
    ensures
        uints(out@) == spec::lexeme_starts(l@),
        forall|k: int|
            0 <= k < out.len() ==> #[trigger] out@[k] < l.len() && spec::lexeme_at(
                l@,
                out@[k] as int,
            ),
{
    let mut out: Vec<usize> = Vec::new();
    let mut i = 0usize;
    while i < l.len()
        invariant
            i <= l.len(),
            spec::lexeme_starts(l@) == uints(out@) + spec::lexemes_from(l@, i as int),
            forall|k: int|
                0 <= k < out.len() ==> #[trigger] out@[k] < l.len() && spec::lexeme_at(
                    l@,
                    out@[k] as int,
                ),
        decreases l.len() - i,
    {
        let ghost before = out@;
        if lexeme_at(l, i) {
            out.push(i);
            proof {
                assert(uints(out@) == uints(before) + seq![i as int]) by {
                    assert_seqs_equal!(uints(out@) == uints(before) + seq![i as int]);
                }
                assert(spec::lexemes_from(l@, i as int) == seq![i as int] + spec::lexemes_from(
                    l@,
                    i as int + 1,
                ));
                assert(uints(before) + spec::lexemes_from(l@, i as int) == uints(out@)
                    + spec::lexemes_from(l@, i as int + 1)) by {
                    assert_seqs_equal!(uints(before) + (seq![i as int] + spec::lexemes_from(l@, i as int + 1)) == (uints(before) + seq![i as int]) + spec::lexemes_from(l@, i as int + 1));
                }
            }
        }
        i += 1;
    }
    proof {
        assert(spec::lexemes_from(l@, i as int) == Seq::<int>::empty());
        assert_seqs_equal!(uints(out@) + Seq::<int>::empty() == uints(out@));
    }
    out
}

pub fn numeral_toks(arena: &ETermArena, toks: &Vec<T>) -> (out: Vec<T>)
    requires
        arena_ok(arena),
        valid_all(arena.nodes@, toks@),
    ensures
        valid_all(arena.nodes@, out@),
        models(out@) == spec::numeral_toks(models(toks@)),
{
    let mut out: Vec<T> = Vec::new();
    let mut i = 0usize;
    proof {
        reveal(Seq::filter);
    }
    while i < toks.len()
        invariant
            arena_ok(arena),
            valid_all(arena.nodes@, toks@),
            valid_all(arena.nodes@, out@),
            i <= toks.len(),
            models(out@) == models(toks@).take(i as int).filter(|t: Term| spec::numeral(t) is Some),
        decreases toks.len() - i,
    {
        proof {
            assert(models(toks@).take(i as int + 1) == models(toks@).take(i as int).push(
                models(toks@)[i as int],
            ));
            models(toks@).take(i as int).lemma_filter_push(
                models(toks@)[i as int],
                |t: Term| spec::numeral(t) is Some,
            );
        }
        if numeral(arena, &toks[i]).is_some() {
            let next = toks[i].cp();
            proof {
                extend(arena.nodes@, out@, next);
            }
            out.push(next);
        }
        i += 1;
    }
    proof {
        assert(models(toks@).take(i as int) == models(toks@));
    }
    out
}

fn slice_eq(l: &Vec<u8>, i: usize, k: usize, text: &Vec<u8>) -> (r: bool)
    requires
        i <= k <= l.len(),
    ensures
        r == (text@ == l@.subrange(i as int, k as int)),
{
    if text.len() != k - i {
        return false;
    }
    let mut j = 0usize;
    while j < text.len()
        invariant
            text.len() == k - i,
            i <= k <= l.len(),
            j <= text.len(),
            forall|m: int| 0 <= m < j ==> text@[m] == l@[i + m],
        decreases text.len() - j,
    {
        if text[j] != l[i + j] {
            return false;
        }
        j += 1;
    }
    proof {
        assert_seqs_equal!(text@ == l@.subrange(i as int, k as int));
    }
    true
}

// The numeral tokens spell the line's lexemes, one to one, in order.
pub fn numerals_match(arena: &ETermArena, src: &ESrc, ns: &Vec<T>, ls: &Vec<usize>) -> (r: bool)
    requires
        arena_ok(arena),
        src_valid(arena.nodes@, src),
        valid_all(arena.nodes@, ns@),
        models(ns@) == spec::numeral_toks(src@.toks),
        uints(ls@) == spec::lexeme_starts(src@.line),
        forall|k: int| 0 <= k < ls.len() ==> #[trigger] ls@[k] < src.line.len(),
    ensures
        r == spec::numerals_match(src@),
{
    if ns.len() != ls.len() {
        return false;
    }
    let mut g = 0usize;
    while g < ns.len()
        invariant
            arena_ok(arena),
            valid_all(arena.nodes@, ns@),
            ns.len() == ls.len(),
            models(ns@) == spec::numeral_toks(src@.toks),
            uints(ls@) == spec::lexeme_starts(src@.line),
            forall|k: int| 0 <= k < ls.len() ==> #[trigger] ls@[k] < src.line.len(),
            g <= ns.len(),
            forall|m: int|
                0 <= m < g ==> spec::numeral(#[trigger] models(ns@)[m]) == Option::Some(
                    src@.line.subrange(
                        spec::lexeme_starts(src@.line)[m],
                        spec::run_end(src@.line, spec::lexeme_starts(src@.line)[m]),
                    ),
                ),
        decreases ns.len() - g,
    {
        proof {
            assert(valid(arena.nodes@, &ns@[g as int]));
            assert(uints(ls@)[g as int] == ls@[g as int] as int);
        }
        let k = run_end(&src.line, ls[g]);
        let ok = match numeral(arena, &ns[g]) {
            Some(text) => slice_eq(&src.line, ls[g], k, &text),
            None => false,
        };
        if !ok {
            proof {
                assert(models(ns@)[g as int] == ns@[g as int]@);
            }
            return false;
        }
        g += 1;
    }
    true
}

// A `-` or `+` token right before a numeral token.
pub closed spec fn signed_at(toks: Seq<Term>, j: int) -> bool {
    (toks[j] == ckc_spec::replay::atom("-"@) || toks[j] == ckc_spec::replay::atom("+"@))
        && spec::numeral(toks[j + 1]) is Some
}

proof fn signed_bridge(toks: Seq<Term>)
    ensures
        spec::signed(toks) == exists|j: int|
            0 <= j && j + 1 < toks.len() && #[trigger] signed_at(toks, j),
{
    reveal(signed_at);
    if spec::signed(toks) {
        let j = choose|j: int|
            0 <= j && j + 1 < toks.len() && ((#[trigger] toks[j]) == ckc_spec::replay::atom("-"@)
                || toks[j] == ckc_spec::replay::atom("+"@)) && spec::numeral(toks[j + 1]) is Some;
        assert(signed_at(toks, j));
    }
    if exists|j: int| 0 <= j && j + 1 < toks.len() && #[trigger] signed_at(toks, j) {
        let j = choose|j: int| 0 <= j && j + 1 < toks.len() && #[trigger] signed_at(toks, j);
        assert(toks[j] == toks[j]);
    }
}

pub fn signed(arena: &ETermArena, toks: &Vec<T>) -> (r: bool)
    requires
        arena_ok(arena),
        valid_all(arena.nodes@, toks@),
    ensures
        r == spec::signed(models(toks@)),
{
    proof {
        signed_bridge(models(toks@));
    }
    let mut i = 0usize;
    while i < toks.len()
        invariant
            arena_ok(arena),
            valid_all(arena.nodes@, toks@),
            i <= toks.len(),
            forall|j: int|
                0 <= j < i && j + 1 < toks.len() ==> !#[trigger] signed_at(models(toks@), j),
        decreases toks.len() - i,
    {
        if i + 1 < toks.len() {
            proof {
                assert(valid(arena.nodes@, &toks@[i as int]));
                assert(valid(arena.nodes@, &toks@[i as int + 1]));
                reveal_strlit("-");
                reveal_strlit("+");
                reveal(signed_at);
                assert(models(toks@)[i as int] == toks@[i as int]@);
                assert(models(toks@)[i as int + 1] == toks@[i as int + 1]@);
            }
            if (is_atom(arena, &toks[i], &Sym::Minus) || is_atom(arena, &toks[i], &Sym::Plus))
                && numeral(arena, &toks[i + 1]).is_some() {
                proof {
                    assert(signed_at(models(toks@), i as int));
                }
                return true;
            }
            proof {
                assert(!signed_at(models(toks@), i as int));
            }
        }
        i += 1;
    }
    false
}

proof fn digits_upto(l: Seq<u8>, i: int, m: int)
    requires
        0 <= i <= m < spec::digits_end(l, i),
    ensures
        is_digit_b(l[m]),
    decreases m - i,
{
    if m > i {
        digits_upto(l, i + 1, m);
    }
}

proof fn digits_end_ge(l: Seq<u8>, i: int)
    requires
        0 <= i,
    ensures
        i <= spec::digits_end(l, i),
        i <= l.len() ==> spec::digits_end(l, i) <= l.len(),
    decreases l.len() - i,
{
    if 0 <= i < l.len() && is_digit_b(l[i]) {
        digits_end_ge(l, i + 1);
    }
}

fn digit_vec(l: &Vec<u8>, i: usize, j: usize) -> (out: Vec<u8>)
    requires
        i <= j <= l.len(),
    ensures
        out@ == l@.subrange(i as int, j as int),
{
    let mut out: Vec<u8> = Vec::new();
    let mut m = i;
    while m < j
        invariant
            i <= m <= j <= l.len(),
            out@ == l@.subrange(i as int, m as int),
        decreases j - m,
    {
        out.push(l[m]);
        proof {
            assert_seqs_equal!(out@ == l@.subrange(i as int, m as int + 1));
        }
        m += 1;
    }
    out
}

proof fn dec_concat(x: Seq<u8>, y: Seq<u8>)
    ensures
        ckc_spec::check::dec_of(x + y) == ckc_spec::check::dec_of(x) * crate::k2_term::pow10(
            y.len(),
        ) + ckc_spec::check::dec_of(y),
    decreases y.len(),
{
    reveal_with_fuel(ckc_spec::check::dec_of, 2);
    reveal_with_fuel(crate::k2_term::pow10, 2);
    if y.len() == 0 {
        assert(x + y =~= x);
        assert(ckc_spec::check::dec_of(y) == 0);
        assert(crate::k2_term::pow10(0) == 1);
    } else {
        assert((x + y).drop_last() == x + y.drop_last());
        assert((x + y).last() == y.last());
        dec_concat(x, y.drop_last());
        let a = ckc_spec::check::dec_of(x);
        let p = crate::k2_term::pow10((y.len() - 1) as nat);
        let b = ckc_spec::check::dec_of(y.drop_last());
        let d = (y.last() - 0x30) as nat;
        assert(ckc_spec::check::dec_of(x + y) == ckc_spec::check::dec_of((x + y).drop_last()) * 10
            + ((x + y).last() - 0x30) as nat);
        assert(ckc_spec::check::dec_of(y) == b * 10 + d);
        assert(crate::k2_term::pow10(y.len()) == 10 * p);
        assert(ckc_spec::check::dec_of(x + y) == (a * p + b) * 10 + d);
        assert((a * p + b) * 10 + d == a * (10 * p) + (b * 10 + d)) by (nonlinear_arith);
    }
}

proof fn p10_pos(k: nat)
    ensures
        crate::k2_term::pow10(k) >= 1,
    decreases k,
{
    if k > 0 {
        p10_pos((k - 1) as nat);
    }
}

proof fn pow10_same(n: nat)
    ensures
        spec::pow10(n) == crate::k2_term::pow10(n) as int,
    decreases n,
{
    if n > 0 {
        pow10_same((n - 1) as nat);
    }
}

fn power10(n: usize) -> (out: Vec<u8>)
    ensures
        out@ == udec_bytes(crate::k2_term::pow10(n as nat)),
{
    let mut raw: Vec<u8> = Vec::new();
    raw.push(0x31);
    let mut m = 0usize;
    proof {
        reveal_with_fuel(ckc_spec::check::dec_of, 2);
        assert(raw@.drop_last() =~= Seq::<u8>::empty());
    }
    while m < n
        invariant
            m <= n,
            raw@.len() == m + 1,
            ckc_spec::check::dec_of(raw@) == crate::k2_term::pow10(m as nat),
            ckc_spec::v1text::all_in(raw@, |b: u8| is_digit_b(b)),
        decreases n - m,
    {
        let ghost before = raw@;
        raw.push(0x30);
        proof {
            assert(raw@.drop_last() == before);
            reveal_with_fuel(ckc_spec::check::dec_of, 2);
            reveal_with_fuel(crate::k2_term::pow10, 2);
            reveal(ckc_spec::v1text::all_in);
        }
        m += 1;
    }
    crate::k4_numbers::normalized(raw.as_slice())
}

// The value of the lexeme at i (`I` or `I.F`) as its canonical number term.
pub fn lexeme_value(arena: &mut ETermArena, l: &Vec<u8>, i: usize) -> (out: T)
    requires
        arena_ok(old(arena)),
        i < l.len(),
        is_digit_b(l@[i as int]),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid(final(arena).nodes@, &out),
        out@ == spec::lexeme_value(l@, i as int),
{
    let j = digits_end(l, i);
    let k = run_end(l, i);
    proof {
        digits_end_ge(l@, i as int + 1);
        assert forall|m: int| i <= m < j implies is_digit_b(#[trigger] l@[m]) by {
            digits_upto(l@, i as int, m);
        }
    }
    let whole = digit_vec(l, i, j);
    proof {
        reveal(ckc_spec::v1text::all_in);
        assert(whole@.len() > 0);
    }
    if k == j {
        let ghost v = ckc_spec::check::dec_of(l@.subrange(i as int, j as int));
        let mag = crate::k4_numbers::normalized(whole.as_slice());
        let root = push_value(arena, false, mag, Ghost(v as int));
        from_root(arena, root)
    } else {
        proof {
            digits_end_ge(l@, j as int + 1);
            assert forall|m: int| j + 1 <= m < k implies is_digit_b(#[trigger] l@[m]) by {
                digits_upto(l@, j as int + 1, m);
            }
        }
        let frac = digit_vec(l, j + 1, k);
        let mut raw = whole.clone();
        let mut m = 0usize;
        while m < frac.len()
            invariant
                raw@ == whole@ + frac@.take(m as int),
                m <= frac.len(),
            decreases frac.len() - m,
        {
            raw.push(frac[m]);
            proof {
                assert_seqs_equal!(raw@ == whole@ + frac@.take(m as int + 1));
            }
            m += 1;
        }
        let ghost w = ckc_spec::check::dec_of(whole@);
        let ghost f = ckc_spec::check::dec_of(frac@);
        let ghost len = frac@.len();
        proof {
            assert(frac@.take(len as int) == frac@);
            dec_concat(whole@, frac@);
            pow10_same(len);
            reveal(ckc_spec::v1text::all_in);
        }
        let num = crate::k4_numbers::normalized(raw.as_slice());
        let den = power10(k - j - 1);
        let ghost pv = w * crate::k2_term::pow10(len) + f;
        proof {
            p10_pos(len);
        }
        let r = ERat {
            neg: false,
            num,
            den,
            n: Ghost(pv as int),
            d: Ghost(crate::k2_term::pow10(len)),
        };
        let root = canon_push(arena, &r);
        from_root(arena, root)
    }
}

// Whether n + delta = 0, for the integer r (delta = ±1).
fn shifted_zero(r: &ERat, up: bool) -> (z: bool)
    requires
        crate::arith_impl::rat_ok(r),
    ensures
        z == (r.n@ + (if up {
            1int
        } else {
            -1int
        }) == 0),
{
    let one = crate::arith_mag::is_one(&r.num, Ghost(a::abs(r.n@)));
    if up {
        r.neg && one
    } else {
        !r.neg && one
    }
}

// The Int term of n + shift (shift ∈ {−2, −1, 1, 2}) for the integer r.
fn int_shift(arena: &mut ETermArena, r: &ERat, shift: i8) -> (root: usize)
    requires
        arena_ok(old(arena)),
        crate::arith_impl::rat_ok(r),
        shift == 1 || shift == 2 || shift == -1 || shift == -2,
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        root < final(arena).nodes@.len(),
        final(arena)@[root as int] == Term::Int(r.n@ + shift as int),
{
    let up = shift > 0;
    let k: usize = if up {
        shift as usize
    } else {
        (-shift) as usize
    };
    let kv = crate::k2_manifest::udec_vec(k);
    let ghost n = r.n@;
    let ghost m = a::abs(n);
    let ghost v = n + shift as int;
    if up == !r.neg {
        let mag = mag_add(&r.num, Ghost(m), &kv, Ghost(k as nat));
        proof {
            assert(a::abs(v) == m + k);
        }
        push_value(arena, r.neg, mag, Ghost(v))
    } else {
        match crate::k2_term::int_order(
            &r.num,
            false,
            Ghost(m as int),
            &kv,
            false,
            Ghost(k as int),
        ) {
            crate::k2_term::EOrder::Less => {
                proof {
                    reveal(crate::k2_term::int_order_ok);
                    assert(a::abs(v) == k - m);
                }
                let mag = mag_sub(&kv, Ghost(k as nat), &r.num, Ghost(m));
                push_value(arena, !up, mag, Ghost(v))
            },
            _ => {
                proof {
                    reveal(crate::k2_term::int_order_ok);
                    assert(a::abs(v) == m - k);
                }
                let mag = mag_sub(&r.num, Ghost(m), &kv, Ghost(k as nat));
                let z = is_zero(&mag, Ghost((m - k) as nat));
                push_value(arena, up && !z, mag, Ghost(v))
            },
        }
    }
}

// A literal lexeme ends the line or stands before a space, `)`, `*`, `/` or
// the final `.`.
pub fn literal_end_ok(l: &Vec<u8>, k: usize) -> (r: bool)
    requires
        k <= l.len(),
    ensures
        r == spec::literal_end_ok(l@, k as int),
{
    k == l.len() || l[k] == 0x20 || l[k] == 0x29 || l[k] == 0x2A || l[k] == 0x2F || (l[k] == 0x2E
        && k + 1 == l.len())
}

fn take_vec(ts: &Vec<T>, n: usize) -> (out: Vec<T>)
    requires
        n <= ts.len(),
    ensures
        out@.len() == n,
        forall|x: int| 0 <= x < n ==> #[trigger] out@[x] == ts@[x],
        models(out@) == models(ts@).take(n as int),
{
    let mut out: Vec<T> = Vec::new();
    let mut i = 0usize;
    while i < n
        invariant
            i <= n <= ts.len(),
            out@.len() == i,
            forall|x: int| 0 <= x < i ==> #[trigger] out@[x] == ts@[x],
        decreases n - i,
    {
        out.push(ts[i].cp());
        i += 1;
    }
    proof {
        assert_seqs_equal!(models(out@) == models(ts@).take(n as int));
    }
    out
}

// The values of a formula's literal leaves (contract da2 D2), else None.
pub fn literal_values(arena: &mut ETermArena, src: &ESrc, kf: usize, lits: &Vec<T>) -> (out: Option<
    Vec<T>,
>)
    requires
        arena_ok(old(arena)),
        src_valid(old(arena).nodes@, src),
        valid_all(old(arena).nodes@, lits@),
        kf < src.toks.len(),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out matches Some(vs) ==> valid_all(final(arena).nodes@, vs@),
        option_models(out) == spec::literal_values(src@, kf as int, models(lits@)),
{
    let ghost start = arena.nodes@;
    let ns = numeral_toks(arena, &src.toks);
    let ls = lexeme_starts(&src.line);
    let k1 = kf + 1;
    let head = take_vec(&src.toks, k1);
    proof {
        assert(models(head@) == models(src.toks@).take(k1 as int));
        assert forall|x: int| 0 <= x < head.len() implies valid(
            arena.nodes@,
            #[trigger] &head@[x],
        ) by {
            assert(head@[x] == src.toks@[x]);
        }
    }
    let g0 = numeral_toks(arena, &head).len();
    if !numerals_match(arena, src, &ns, &ls) || signed(arena, &src.toks) || lits.len() > ns.len()
        || g0 > ns.len() - lits.len() {
        return None;
    }
    let mut m = 0usize;
    while m < lits.len()
        invariant
            arena_ok(arena),
            valid_all(arena.nodes@, lits@),
            valid_all(arena.nodes@, ns@),
            models(ns@) == spec::numeral_toks(src@.toks),
            uints(ls@) == spec::lexeme_starts(src@.line),
            ns.len() == ls.len(),
            forall|k: int|
                0 <= k < ls.len() ==> #[trigger] ls@[k] < src.line.len() && spec::lexeme_at(
                    src.line@,
                    ls@[k] as int,
                ),
            g0 + lits.len() <= ns.len(),
            g0 == spec::numeral_toks(src@.toks.take(kf as int + 1)).len(),
            m <= lits.len(),
            forall|x: int|
                0 <= x < m ==> spec::numeral(ckc_spec::replay::arg(#[trigger] models(lits@)[x], 0))
                    == spec::numeral(models(ns@)[g0 + x]) && spec::literal_end_ok(
                    src.line@,
                    spec::run_end(src.line@, spec::lexeme_starts(src.line@)[g0 + x]),
                ),
        decreases lits.len() - m,
    {
        proof {
            assert(valid(arena.nodes@, &lits@[m as int]));
            assert(valid(arena.nodes@, &ns@[g0 + m]));
            assert(uints(ls@)[g0 + m] == ls@[g0 + m] as int);
        }
        let xs = args(arena, &lits[m]);
        let leaf = if xs.len() > 0 {
            proof {
                assert(valid(arena.nodes@, &xs@[0]));
            }
            numeral(arena, &xs[0])
        } else {
            None
        };
        let tok = numeral(arena, &ns[g0 + m]);
        let same = match (&leaf, &tok) {
            (Some(x), Some(y)) => bytes_eq(x.as_slice(), y.as_slice()),
            (None, None) => true,
            _ => false,
        };
        let k = run_end(&src.line, ls[g0 + m]);
        if !same || !literal_end_ok(&src.line, k) {
            proof {
                assert(models(lits@)[m as int] == lits@[m as int]@);
                assert(models(ns@)[g0 + m] == ns@[g0 + m]@);
                reveal(ckc_spec::replay::arg);
            }
            return None;
        }
        proof {
            assert(models(lits@)[m as int] == lits@[m as int]@);
            assert(models(ns@)[g0 + m] == ns@[g0 + m]@);
            reveal(ckc_spec::replay::arg);
            if xs.len() > 0 {
                assert(models(xs@)[0] == xs@[0]@);
                assert(ckc_spec::engine::args_of(lits@[m as int]@).len() > 0);
                assert(lits@[m as int]@ is Comp);
                assert(ckc_spec::replay::arg(lits@[m as int]@, 0) == ckc_spec::engine::args_of(
                    lits@[m as int]@,
                )[0]);
                assert(xs@[0]@ == ckc_spec::replay::arg(lits@[m as int]@, 0));
            } else {
                assert(ckc_spec::replay::arg(lits@[m as int]@, 0) == Term::Nil);
            }
            assert(opt_seq(leaf) == opt_seq(tok));
            assert(spec::numeral(ckc_spec::replay::arg(models(lits@)[m as int], 0))
                == spec::numeral(models(ns@)[g0 + m]));
            assert(spec::lexeme_starts(src.line@)[g0 + m] == ls@[g0 + m] as int);
            assert(spec::literal_end_ok(
                src.line@,
                spec::run_end(src.line@, spec::lexeme_starts(src.line@)[g0 + m]),
            ));
        }
        m += 1;
    }
    let mut vals: Vec<T> = Vec::new();
    let mut m = 0usize;
    while m < lits.len()
        invariant
            arena_ok(arena),
            start.is_prefix_of(arena.nodes@),
            uints(ls@) == spec::lexeme_starts(src@.line),
            ns.len() == ls.len(),
            forall|k: int|
                0 <= k < ls.len() ==> #[trigger] ls@[k] < src.line.len() && spec::lexeme_at(
                    src.line@,
                    ls@[k] as int,
                ),
            g0 + lits.len() <= ns.len(),
            m <= lits.len(),
            valid_all(arena.nodes@, vals@),
            vals.len() == m,
            forall|x: int|
                0 <= x < m ==> (#[trigger] vals@[x])@ == spec::lexeme_value(
                    src.line@,
                    spec::lexeme_starts(src.line@)[g0 + x],
                ),
        decreases lits.len() - m,
    {
        proof {
            assert(uints(ls@)[g0 + m] == ls@[g0 + m] as int);
            assert(spec::lexeme_at(src.line@, ls@[g0 + m] as int));
        }
        let ghost before = arena.nodes@;
        let v = lexeme_value(arena, &src.line, ls[g0 + m]);
        proof {
            prefix_all(before, arena.nodes@, vals@);
            extend(arena.nodes@, vals@, v);
        }
        vals.push(v);
        m += 1;
    }
    proof {
        assert_seqs_equal!(models(vals@) == Seq::new(lits.len() as nat, |x: int| spec::lexeme_value(src@.line, spec::lexeme_starts(src@.line)[g0 + x])));
    }
    Some(vals)
}

// An operand's witness: an exact nonzero value its condition admits (D4).
// Contract da2 D4: an operand's witness is a value its condition admits,
// nonzero unless the condition is `exactly 0`.
pub proof fn operand_witness_admitted(op: Term, n: int)
    requires
        op == ckc_spec::replay::atom("exactly"@) || op == ckc_spec::replay::atom("geq"@) || op
            == ckc_spec::replay::atom("leq"@) || op == ckc_spec::replay::atom("greater"@) || op
            == ckc_spec::replay::atom("less"@),
    ensures
        spec::operand_witness(op, Term::Int(n)) matches Term::Int(w) && (op
            == ckc_spec::replay::atom("exactly"@) ==> w == n) && (op == ckc_spec::replay::atom(
            "geq"@,
        ) ==> w >= n) && (op == ckc_spec::replay::atom("leq"@) ==> w <= n) && (op
            == ckc_spec::replay::atom("greater"@) ==> w > n) && (op == ckc_spec::replay::atom(
            "less"@,
        ) ==> w < n) && (w == 0 ==> op == ckc_spec::replay::atom("exactly"@) && n == 0),
{
    reveal_strlit("exactly");
    reveal_strlit("geq");
    reveal_strlit("leq");
    reveal_strlit("greater");
    reveal_strlit("less");
    let e = ckc_spec::v1text::ascii("exactly"@);
    let g = ckc_spec::v1text::ascii("geq"@);
    let l = ckc_spec::v1text::ascii("leq"@);
    let gr = ckc_spec::v1text::ascii("greater"@);
    let ls = ckc_spec::v1text::ascii("less"@);
    assert(e[0] != gr[0] && g[0] != l[0]);
    assert(e.len() != g.len() && e.len() != l.len() && e.len() != ls.len());
    assert(g.len() != gr.len() && g.len() != ls.len() && l.len() != gr.len() && l.len()
        != ls.len());
    assert(gr.len() != ls.len());
}

pub fn operand_witness(arena: &mut ETermArena, op: &T, count: &T) -> (out: T)
    requires
        arena_ok(old(arena)),
        valid(old(arena).nodes@, op),
        valid(old(arena).nodes@, count),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid(final(arena).nodes@, &out),
        out@ == spec::operand_witness(op@, count@),
{
    if !is_int(arena, count) {
        return count.cp();
    }
    let r = match decode_number(arena, count.root) {
        Some(r) => r,
        None => return count.cp(),
    };
    proof {
        reveal(a::number);
        assert(r.d@ == 1 && count@ == Term::Int(r.n@));
    }
    let zero = is_zero(&r.num, Ghost(a::abs(r.n@)));
    let geq = is_atom(arena, op, &Sym::Geq);
    let leq = is_atom(arena, op, &Sym::Leq);
    let greater = is_atom(arena, op, &Sym::Greater);
    let less = is_atom(arena, op, &Sym::Less);
    proof {
        reveal_strlit("geq");
        reveal_strlit("leq");
        reveal_strlit("greater");
        reveal_strlit("less");
    }
    let shift: i8 = if geq && zero {
        1
    } else if leq && zero {
        -1
    } else if greater {
        if shifted_zero(&r, true) {
            2
        } else {
            1
        }
    } else if less {
        if shifted_zero(&r, false) {
            -2
        } else {
            -1
        }
    } else {
        0
    };
    if shift == 0 {
        return count.cp();
    }
    let root = int_shift(arena, &r, shift);
    from_root(arena, root)
}

} // verus!
