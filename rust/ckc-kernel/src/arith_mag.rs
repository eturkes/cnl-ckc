#[cfg(verus_keep_ghost)]
use crate::compare_decimal::decimal_shape;
use crate::k2_manifest::udec_vec;
use crate::k2_term::{EOrder, int_order};
#[cfg(verus_keep_ghost)]
use crate::k2_term::{int_order_ok, pow10};
#[cfg(verus_keep_ghost)]
use crate::k4_numbers::decimal_print_inverse;
use crate::k4_numbers::normalized;
#[cfg(verus_keep_ghost)]
use ckc_spec::check::{dec_of, nat_bytes};
#[cfg(verus_keep_ghost)]
use ckc_spec::v1text::{all_in, digit_byte, is_digit_b, udec_bytes};
#[cfg(verus_keep_ghost)]
use vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse;
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

// Arbitrary-precision magnitudes (contract da1 D11): canonical decimal digit
// vectors (`udec_bytes`), as the term arena stores integer magnitudes. Each
// operation computes a digit string whose value is the result (leading zeros
// allowed) and `normalized` makes it canonical.
pub open spec fn digits(s: Seq<u8>) -> bool {
    all_in(s, |b: u8| is_digit_b(b))
}

proof fn p10_pos(k: nat)
    ensures
        pow10(k) >= 1,
    decreases k,
{
    reveal_with_fuel(pow10, 2);
    if k > 0 {
        p10_pos((k - 1) as nat);
    }
}

proof fn udec_digits(n: nat)
    ensures
        digits(udec_bytes(n)),
        udec_bytes(n).len() > 0,
        dec_of(udec_bytes(n)) == n,
{
    decimal_print_inverse(n);
}

// A more significant digit in front.
proof fn dec_prepend(d: u8, s: Seq<u8>)
    requires
        is_digit_b(d),
        digits(s),
    ensures
        dec_of(seq![d] + s) == (d - 0x30) as nat * pow10(s.len()) + dec_of(s),
    decreases s.len(),
{
    reveal_with_fuel(dec_of, 2);
    reveal_with_fuel(pow10, 2);
    if s.len() == 0 {
        assert(seq![d] + s =~= seq![d]);
        assert(seq![d].drop_last() =~= Seq::<u8>::empty());
    } else {
        let p = s.drop_last();
        assert(digits(p));
        dec_prepend(d, p);
        assert((seq![d] + s).drop_last() =~= seq![d] + p);
        assert((seq![d] + s).last() == s.last());
        let dv = (d - 0x30) as nat;
        let l = (s.last() - 0x30) as nat;
        let pp = pow10(p.len());
        assert(pow10(s.len()) == 10 * pp);
        assert((dv * pp + dec_of(p)) * 10 + l == dv * (10 * pp) + (dec_of(p) * 10 + l))
            by (nonlinear_arith);
    }
}

proof fn dec_bound(s: Seq<u8>)
    requires
        digits(s),
    ensures
        dec_of(s) < pow10(s.len()),
    decreases s.len(),
{
    reveal_with_fuel(dec_of, 2);
    reveal_with_fuel(pow10, 2);
    if s.len() > 0 {
        let p = s.drop_last();
        assert(digits(p));
        dec_bound(p);
        let l = (s.last() - 0x30) as nat;
        assert(l < 10);
        assert(dec_of(p) * 10 + l < 10 * pow10(p.len())) by (nonlinear_arith)
            requires
                dec_of(p) < pow10(p.len()),
                l < 10,
        ;
    }
}

proof fn dec_push(s: Seq<u8>, d: u8)
    ensures
        dec_of(s.push(d)) == dec_of(s) * 10 + (d - 0x30) as nat,
{
    reveal_with_fuel(dec_of, 2);
    assert(s.push(d).drop_last() =~= s);
}

proof fn udec_times10(n: nat)
    requires
        n > 0,
    ensures
        udec_bytes(10 * n) == udec_bytes(n).push(0x30),
{
    decimal_shape(10 * n);
    assert((10 * n) / 10 == n);
    assert((10 * n) % 10 == 0);
    reveal(digit_byte);
    assert_seqs_equal!(udec_bytes(10 * n) == udec_bytes(n).push(0x30));
}

proof fn udec_append(n: nat, d: nat)
    requires
        n > 0,
        d < 10,
    ensures
        udec_bytes(10 * n + d) == udec_bytes(n).push(digit_byte(d as int)),
{
    decimal_shape(10 * n + d);
    assert((10 * n + d) / 10 == n);
    assert((10 * n + d) % 10 == d);
    assert_seqs_equal!(udec_bytes(10 * n + d) == udec_bytes(n).push(digit_byte(d as int)));
}

proof fn udec_zero(n: nat)
    ensures
        (udec_bytes(n) == seq![0x30u8]) == (n == 0),
{
    decimal_shape(n);
    reveal(digit_byte);
    if n >= 10 {
        decimal_shape(n / 10);
        assert(udec_bytes(n).len() >= 2);
        assert(seq![0x30u8].len() == 1);
    } else {
        assert(udec_bytes(n) =~= seq![digit_byte(n as int)]);
        if udec_bytes(n) == seq![0x30u8] {
            assert(udec_bytes(n)[0] == 0x30u8);
        }
    }
}

pub fn is_zero(x: &Vec<u8>, a: Ghost<nat>) -> (r: bool)
    requires
        x@ == udec_bytes(a@),
    ensures
        r == (a@ == 0),
{
    proof {
        udec_zero(a@);
    }
    x.len() == 1 && x[0] == 0x30
}

proof fn udec_one(n: nat)
    ensures
        (udec_bytes(n) == seq![0x31u8]) == (n == 1),
{
    decimal_shape(n);
    reveal(digit_byte);
    if n >= 10 {
        decimal_shape(n / 10);
        assert(udec_bytes(n).len() >= 2);
        assert(seq![0x31u8].len() == 1);
    } else {
        assert(udec_bytes(n) =~= seq![digit_byte(n as int)]);
        if udec_bytes(n) == seq![0x31u8] {
            assert(udec_bytes(n)[0] == 0x31u8);
        }
    }
}

pub fn is_one(x: &Vec<u8>, a: Ghost<nat>) -> (r: bool)
    requires
        x@ == udec_bytes(a@),
    ensures
        r == (a@ == 1),
{
    proof {
        udec_one(a@);
    }
    x.len() == 1 && x[0] == 0x31
}

pub fn zero() -> (r: Vec<u8>)
    ensures
        r@ == udec_bytes(0),
{
    udec_vec(0)
}

pub fn one() -> (r: Vec<u8>)
    ensures
        r@ == udec_bytes(1),
{
    udec_vec(1)
}

// `lead` (if any) followed by `low` read back to front.
fn assemble(lead: Option<u8>, low: Vec<u8>) -> (out: Vec<u8>)
    ensures
        out@ == match lead {
            Option::Some(d) => seq![d] + low@.reverse(),
            Option::None => low@.reverse(),
        },
{
    let mut out: Vec<u8> = Vec::new();
    match lead {
        Option::Some(d) => out.push(d),
        Option::None => {},
    }
    let mut rest = low;
    let ghost start = out@;
    proof {
        assert_seqs_equal!(start + rest@.reverse() == match lead {
            Option::Some(d) => seq![d] + low@.reverse(),
            Option::None => low@.reverse(),
        });
    }
    while rest.len() > 0
        invariant
            out@ + rest@.reverse() == match lead {
                Option::Some(d) => seq![d] + low@.reverse(),
                Option::None => low@.reverse(),
            },
        decreases rest.len(),
    {
        let ghost before = rest@;
        let d = rest.pop().unwrap();
        proof {
            assert_seqs_equal!(before.reverse() == seq![d] + rest@.reverse());
            assert_seqs_equal!(out@.push(d) + rest@.reverse() == out@ + before.reverse());
        }
        out.push(d);
    }
    proof {
        assert_seqs_equal!(out@ + rest@.reverse() == out@);
    }
    out
}

proof fn reverse_digits(s: Seq<u8>)
    requires
        digits(s),
    ensures
        digits(s.reverse()),
        s.reverse().len() == s.len(),
{
    assert forall|i: int| 0 <= i < s.reverse().len() implies is_digit_b(
        #[trigger] s.reverse()[i],
    ) by {
        assert(s.reverse()[i] == s[s.len() - 1 - i]);
    }
}

// One digit step of a prefix walk: the canonical prefix value q (≥ 1) drops
// its last digit.
proof fn take_step(x: Seq<u8>, i: int, q: nat)
    requires
        0 < i <= x.len(),
        x.take(i) == udec_bytes(q),
    ensures
        x[i - 1] == digit_byte((q % 10) as int),
        q >= 10 ==> i > 1 && x.take(i - 1) == udec_bytes(q / 10),
        q < 10 ==> i == 1 && q / 10 == 0,
        is_digit_b(x[i - 1]),
        (x[i - 1] - 0x30) as nat == q % 10,
{
    decimal_shape(q);
    reveal(digit_byte);
    assert(x[i - 1] == x.take(i)[i - 1]);
    if q >= 10 {
        decimal_shape(q / 10);
        assert_seqs_equal!(x.take(i - 1) == x.take(i).drop_last());
    } else {
        assert(udec_bytes(q).len() == 1);
    }
}

pub fn mag_add(x: &Vec<u8>, a: Ghost<nat>, y: &Vec<u8>, b: Ghost<nat>) -> (out: Vec<u8>)
    requires
        x@ == udec_bytes(a@),
        y@ == udec_bytes(b@),
    ensures
        out@ == udec_bytes(a@ + b@),
{
    proof {
        udec_digits(a@);
        udec_digits(b@);
        assert_seqs_equal!(x@.take(x@.len() as int) == x@);
        assert_seqs_equal!(y@.take(y@.len() as int) == y@);
    }
    let mut i = x.len();
    let mut j = y.len();
    let mut carry: u8 = 0;
    let mut low: Vec<u8> = Vec::new();
    let ghost mut qa = a@;
    let ghost mut qb = b@;
    proof {
        reveal_with_fuel(pow10, 1);
        reveal_with_fuel(dec_of, 1);
        assert(low@.reverse() =~= Seq::<u8>::empty());
        assert(dec_of(low@.reverse()) == 0);
        assert(pow10(low@.len()) == 1);
    }
    while i > 0 || j > 0
        invariant
            i <= x@.len(),
            j <= y@.len(),
            i > 0 ==> x@.take(i as int) == udec_bytes(qa),
            i == 0 ==> qa == 0,
            j > 0 ==> y@.take(j as int) == udec_bytes(qb),
            j == 0 ==> qb == 0,
            carry <= 1,
            digits(low@),
            x@.len() > 0,
            low@.len() + i >= x@.len(),
            a@ + b@ == (qa + qb + carry as nat) * pow10(low@.len()) + dec_of(low@.reverse()),
        decreases i + j,
    {
        let ghost (qa0, qb0, low0, c0) = (qa, qb, low@, carry as nat);
        let mut dx: u8 = 0;
        if i > 0 {
            proof {
                take_step(x@, i as int, qa);
            }
            dx = x[i - 1] - 0x30;
            i -= 1;
            proof {
                qa = qa / 10;
            }
        }
        let mut dy: u8 = 0;
        if j > 0 {
            proof {
                take_step(y@, j as int, qb);
            }
            dy = y[j - 1] - 0x30;
            j -= 1;
            proof {
                qb = qb / 10;
            }
        }
        let t = dx + dy + carry;
        let d = 0x30 + t % 10;
        carry = t / 10;
        low.push(d);
        proof {
            assert(qa0 == 10 * qa + dx as nat);
            assert(qb0 == 10 * qb + dy as nat);
            let k = low0.len();
            let pk = pow10(k);
            assert(low@.reverse() =~= seq![d] + low0.reverse());
            reverse_digits(low0);
            dec_prepend(d, low0.reverse());
            reveal_with_fuel(pow10, 2);
            assert(pow10(low@.len()) == 10 * pk);
            assert((qa0 + qb0 + c0) * pk == (qa + qb + carry as nat) * (10 * pk) + (t % 10) as nat
                * pk) by (nonlinear_arith)
                requires
                    qa0 + qb0 + c0 == 10 * (qa + qb) + t as nat,
                    t as nat == 10 * carry as nat + (t % 10) as nat,
            ;
            assert(digits(low@)) by {
                assert forall|m: int| 0 <= m < low@.len() implies is_digit_b(
                    #[trigger] low@[m],
                ) by {
                    if m < low0.len() {
                        assert(low@[m] == low0[m]);
                    }
                }
            }
        }
    }
    proof {
        assert(low@.len() > 0);
        reverse_digits(low@);
        assert(qa == 0 && qb == 0);
    }
    let lead = if carry == 1 {
        Option::Some(0x31u8)
    } else {
        Option::None
    };
    let ghost lowv = low@;
    let raw = assemble(lead, low);
    proof {
        assert(lowv.reverse().len() == lowv.len());
        if carry == 1 {
            dec_prepend(0x31, lowv.reverse());
            assert(dec_of(raw@) == a@ + b@);
            assert(digits(raw@)) by {
                assert forall|m: int| 0 <= m < raw@.len() implies is_digit_b(
                    #[trigger] raw@[m],
                ) by {
                    if m > 0 {
                        assert(raw@[m] == lowv.reverse()[m - 1]);
                    }
                }
            }
        } else {
            assert(carry == 0);
            assert(raw@ == lowv.reverse());
            assert((qa + qb + carry as nat) * pow10(lowv.len()) == 0) by (nonlinear_arith)
                requires
                    qa + qb + carry as nat == 0,
            ;
            assert(dec_of(raw@) == a@ + b@);
        }
    }
    normalized(raw.as_slice())
}

// a − b for a ≥ b.
pub fn mag_sub(x: &Vec<u8>, a: Ghost<nat>, y: &Vec<u8>, b: Ghost<nat>) -> (out: Vec<u8>)
    requires
        x@ == udec_bytes(a@),
        y@ == udec_bytes(b@),
        a@ >= b@,
    ensures
        out@ == udec_bytes((a@ - b@) as nat),
{
    proof {
        udec_digits(a@);
        udec_digits(b@);
        assert_seqs_equal!(x@.take(x@.len() as int) == x@);
        assert_seqs_equal!(y@.take(y@.len() as int) == y@);
    }
    let mut i = x.len();
    let mut j = y.len();
    let mut borrow: u8 = 0;
    let mut low: Vec<u8> = Vec::new();
    let ghost mut qa = a@;
    let ghost mut qb = b@;
    proof {
        reveal_with_fuel(pow10, 1);
        reveal_with_fuel(dec_of, 1);
        assert(low@.reverse() =~= Seq::<u8>::empty());
        assert(dec_of(low@.reverse()) == 0);
        assert(pow10(low@.len()) == 1);
    }
    while i > 0 || j > 0
        invariant
            i <= x@.len(),
            j <= y@.len(),
            i > 0 ==> x@.take(i as int) == udec_bytes(qa),
            i == 0 ==> qa == 0,
            j > 0 ==> y@.take(j as int) == udec_bytes(qb),
            j == 0 ==> qb == 0,
            borrow <= 1,
            digits(low@),
            x@.len() > 0,
            low@.len() + i >= x@.len(),
            a@ - b@ == (qa - qb - borrow as int) * pow10(low@.len()) + dec_of(low@.reverse()),
        decreases i + j,
    {
        let ghost (qa0, qb0, low0, w0) = (qa, qb, low@, borrow as int);
        let mut dx: u8 = 0;
        if i > 0 {
            proof {
                take_step(x@, i as int, qa);
            }
            dx = x[i - 1] - 0x30;
            i -= 1;
            proof {
                qa = qa / 10;
            }
        }
        let mut dy: u8 = 0;
        if j > 0 {
            proof {
                take_step(y@, j as int, qb);
            }
            dy = y[j - 1] - 0x30;
            j -= 1;
            proof {
                qb = qb / 10;
            }
        }
        let d: u8;
        if dx >= dy + borrow {
            d = 0x30 + (dx - dy - borrow);
            borrow = 0;
        } else {
            d = 0x30 + (dx + 10 - dy - borrow);
            borrow = 1;
        }
        low.push(d);
        proof {
            assert(qa0 == 10 * qa + dx as nat);
            assert(qb0 == 10 * qb + dy as nat);
            let k = low0.len();
            let pk = pow10(k);
            assert(low@.reverse() =~= seq![d] + low0.reverse());
            reverse_digits(low0);
            dec_prepend(d, low0.reverse());
            reveal_with_fuel(pow10, 2);
            assert(pow10(low@.len()) == 10 * pk);
            let dv = (d - 0x30) as int;
            assert((qa0 - qb0 - w0) == 10 * (qa - qb - borrow as int) + dv);
            assert((qa0 - qb0 - w0) * pk == (qa - qb - borrow as int) * (10 * pk) + dv * pk)
                by (nonlinear_arith)
                requires
                    (qa0 - qb0 - w0) == 10 * (qa - qb - borrow as int) + dv,
            ;
            assert(digits(low@)) by {
                assert forall|m: int| 0 <= m < low@.len() implies is_digit_b(
                    #[trigger] low@[m],
                ) by {
                    if m < low0.len() {
                        assert(low@[m] == low0[m]);
                    }
                }
            }
        }
    }
    proof {
        reverse_digits(low@);
        dec_bound(low@.reverse());
        p10_pos(low@.len());
        // a − b = dec(low) − borrow·10^k ≥ 0 with dec(low) < 10^k ⇒ no borrow left
        if borrow == 1 {
            assert(a@ - b@ == dec_of(low@.reverse()) - pow10(low@.len()));
            assert(false);
        }
        assert(low@.len() > 0);
    }
    let ghost lowv = low@;
    let raw = assemble(Option::None, low);
    proof {
        assert(qa == 0 && qb == 0 && borrow == 0);
        assert(raw@ == lowv.reverse());
        assert((qa - qb - borrow as int) * pow10(lowv.len()) == 0) by (nonlinear_arith)
            requires
                qa - qb - borrow as int == 0,
        ;
        assert(dec_of(raw@) == a@ - b@);
    }
    normalized(raw.as_slice())
}

fn times10(x: &Vec<u8>, a: Ghost<nat>) -> (out: Vec<u8>)
    requires
        x@ == udec_bytes(a@),
    ensures
        out@ == udec_bytes(10 * a@),
{
    if is_zero(x, a) {
        zero()
    } else {
        let mut out = x.clone();
        out.push(0x30);
        proof {
            udec_times10(a@);
        }
        out
    }
}

pub fn mag_mul(x: &Vec<u8>, a: Ghost<nat>, y: &Vec<u8>, b: Ghost<nat>) -> (out: Vec<u8>)
    requires
        x@ == udec_bytes(a@),
        y@ == udec_bytes(b@),
    ensures
        out@ == udec_bytes(a@ * b@),
{
    proof {
        udec_digits(b@);
    }
    let mut acc = zero();
    let ghost mut av: nat = 0;
    let mut j = 0usize;
    proof {
        reveal_with_fuel(dec_of, 1);
        assert(y@.take(0) =~= Seq::<u8>::empty());
        assert(dec_of(y@.take(0)) == 0);
        assert(av == a@ * dec_of(y@.take(0)));
    }
    while j < y.len()
        invariant
            j <= y@.len(),
            y@ == udec_bytes(b@),
            x@ == udec_bytes(a@),
            digits(y@),
            acc@ == udec_bytes(av),
            av == a@ * dec_of(y@.take(j as int)),
        decreases y.len() - j,
    {
        let d = y[j] - 0x30;
        let shifted = times10(&acc, Ghost(av));
        let part = if d == 0 {
            zero()
        } else {
            crate::compare_decimal::multiply(x, d as usize, a)
        };
        proof {
            assert(d as nat * a@ == a@ * d as nat) by (nonlinear_arith);
        }
        let next = mag_add(&shifted, Ghost(10 * av), &part, Ghost(a@ * d as nat));
        proof {
            assert(y@.take(j as int + 1) =~= y@.take(j as int).push(y@[j as int]));
            dec_push(y@.take(j as int), y@[j as int]);
            assert(10 * av + a@ * d as nat == a@ * (dec_of(y@.take(j as int)) * 10 + d as nat))
                by (nonlinear_arith)
                requires
                    av == a@ * dec_of(y@.take(j as int)),
            ;
            av = 10 * av + a@ * d as nat;
        }
        acc = next;
        j += 1;
    }
    proof {
        assert(y@.take(y@.len() as int) =~= y@);
    }
    acc
}

fn append_digit(r: &Vec<u8>, rv: Ghost<nat>, d: u8) -> (out: Vec<u8>)
    requires
        r@ == udec_bytes(rv@),
        d < 10,
    ensures
        out@ == udec_bytes(10 * rv@ + d as nat),
{
    if is_zero(r, rv) {
        udec_vec(d as usize)
    } else {
        let mut out = r.clone();
        out.push(0x30 + d);
        proof {
            udec_append(rv@, d as nat);
            reveal(digit_byte);
        }
        out
    }
}

// (a / b, a % b) for b > 0, by long division.
pub fn mag_divmod(x: &Vec<u8>, a: Ghost<nat>, y: &Vec<u8>, b: Ghost<nat>) -> (out: (
    Vec<u8>,
    Vec<u8>,
))
    requires
        x@ == udec_bytes(a@),
        y@ == udec_bytes(b@),
        b@ > 0,
    ensures
        out.0@ == udec_bytes(a@ / b@),
        out.1@ == udec_bytes(a@ % b@),
{
    proof {
        udec_digits(a@);
    }
    let mut r = zero();
    let ghost mut rv: nat = 0;
    let mut qd: Vec<u8> = Vec::new();
    let mut j = 0usize;
    proof {
        reveal_with_fuel(dec_of, 1);
        assert(x@.take(0) =~= Seq::<u8>::empty());
        assert(dec_of(x@.take(0)) == 0);
        assert(dec_of(qd@) == 0);
    }
    while j < x.len()
        invariant
            j <= x@.len(),
            x@ == udec_bytes(a@),
            y@ == udec_bytes(b@),
            digits(x@),
            b@ > 0,
            r@ == udec_bytes(rv),
            rv < b@,
            digits(qd@),
            qd@.len() == j,
            x@.len() > 0,
            dec_of(x@.take(j as int)) == dec_of(qd@) * b@ + rv,
        decreases x.len() - j,
    {
        let d = x[j] - 0x30;
        let mut cur = append_digit(&r, Ghost(rv), d);
        let ghost mut cv: nat = 10 * rv + d as nat;
        let ghost start = cv;
        let mut k: u8 = 0;
        proof {
            assert(start < 10 * b@) by (nonlinear_arith)
                requires
                    rv < b@,
                    d < 10,
                    start == 10 * rv + d as nat,
            ;
        }
        loop
            invariant
                cur@ == udec_bytes(cv),
                y@ == udec_bytes(b@),
                b@ > 0,
                k <= 9,
                start < 10 * b@,
                start == cv + k as nat * b@,
            ensures
                cur@ == udec_bytes(cv),
                k <= 9,
                cv < b@,
                start == cv + k as nat * b@,
            decreases cv,
        {
            let order = int_order(&cur, false, Ghost(cv as int), y, false, Ghost(b@ as int));
            match order {
                EOrder::Less => {
                    proof {
                        reveal(int_order_ok);
                    }
                    break;
                },
                _ => {
                    proof {
                        reveal(int_order_ok);
                        assert(k < 9) by (nonlinear_arith)
                            requires
                                start == cv + k as nat * b@,
                                cv >= b@,
                                start < 10 * b@,
                        ;
                    }
                    cur = mag_sub(&cur, Ghost(cv), y, b);
                    let ghost k0 = k as nat;
                    proof {
                        cv = (cv - b@) as nat;
                    }
                    k += 1;
                    proof {
                        assert(k as nat * b@ == k0 * b@ + b@) by (nonlinear_arith)
                            requires
                                k as nat == k0 + 1,
                        ;
                    }
                },
            }
        }
        let ghost q0 = qd@;
        qd.push(0x30 + k);
        proof {
            assert(cv < b@);
            assert(x@.take(j as int + 1) =~= x@.take(j as int).push(x@[j as int]));
            dec_push(x@.take(j as int), x@[j as int]);
            dec_push(q0, (0x30 + k) as u8);
            assert(10 * (dec_of(q0) * b@ + rv) + d as nat == (dec_of(q0) * 10 + k as nat) * b@ + cv)
                by (nonlinear_arith)
                requires
                    start == 10 * rv + d as nat,
                    start == cv + k as nat * b@,
            ;
            assert(digits(qd@)) by {
                assert forall|m: int| 0 <= m < qd@.len() implies is_digit_b(#[trigger] qd@[m]) by {
                    if m < q0.len() {
                        assert(qd@[m] == q0[m]);
                    }
                }
            }
            rv = cv;
        }
        r = cur;
        j += 1;
    }
    proof {
        assert(x@.take(x@.len() as int) =~= x@);
        lemma_fundamental_div_mod_converse(a@ as int, b@ as int, dec_of(qd@) as int, rv as int);
        assert(qd@.len() > 0);
    }
    let q = normalized(qd.as_slice());
    (q, r)
}

pub fn mag_gcd(x: &Vec<u8>, a: Ghost<nat>, y: &Vec<u8>, b: Ghost<nat>) -> (out: Vec<u8>)
    requires
        x@ == udec_bytes(a@),
        y@ == udec_bytes(b@),
    ensures
        out@ == udec_bytes(ckc_spec::arith::gcd(a@, b@)),
{
    let mut u = x.clone();
    let mut v = y.clone();
    let ghost mut uv = a@;
    let ghost mut vv = b@;
    while !is_zero(&v, Ghost(vv))
        invariant
            u@ == udec_bytes(uv),
            v@ == udec_bytes(vv),
            ckc_spec::arith::gcd(uv, vv) == ckc_spec::arith::gcd(a@, b@),
        decreases vv,
    {
        let (_, rem) = mag_divmod(&u, Ghost(uv), &v, Ghost(vv));
        proof {
            reveal_with_fuel(ckc_spec::arith::gcd, 2);
            assert(ckc_spec::arith::gcd(uv, vv) == ckc_spec::arith::gcd(vv, uv % vv));
            let w = uv % vv;
            uv = vv;
            vv = w;
        }
        u = v;
        v = rem;
    }
    proof {
        reveal_with_fuel(ckc_spec::arith::gcd, 2);
    }
    u
}

} // verus!
