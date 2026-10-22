use crate::k2_manifest::udec_vec;
#[cfg(verus_keep_ghost)]
use ckc_spec::v1text::{digit_byte, udec_bytes};
#[cfg(verus_keep_ghost)]
use vstd::arithmetic::div_mod::{
    lemma_div_non_zero, lemma_div_nonincreasing, lemma_fundamental_div_mod,
    lemma_fundamental_div_mod_converse,
};
#[cfg(verus_keep_ghost)]
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub(crate) proof fn decimal_shape(n: nat)
    ensures
        udec_bytes(n).len() > 0,
        (udec_bytes(n).len() == 1) == (n < 10),
        n < 10 ==> udec_bytes(n) == seq![digit_byte(n as int)],
        n >= 10 ==> udec_bytes(n).drop_last() == udec_bytes(n / 10),
        n >= 10 ==> udec_bytes(n).last() == digit_byte((n % 10) as int),
    decreases n,
{
    reveal_with_fuel(udec_bytes, 2);
    if n >= 10 {
        decimal_shape(n / 10);
        assert_seqs_equal!((udec_bytes(n / 10) + seq![digit_byte((n % 10) as int)]).drop_last() == udec_bytes(n / 10));
    }
}

proof fn multiply_step(n: nat, factor: nat, carry: nat)
    requires
        n >= 10,
        factor > 0,
        carry < factor,
    ensures
        udec_bytes(n * factor + carry) == udec_bytes(
            (n / 10) * factor + ((n % 10) * factor + carry) / 10,
        ) + seq![digit_byte((((n % 10) * factor + carry) % 10) as int)],
{
    let q = n / 10;
    let d = n % 10;
    let t = d * factor + carry;
    let next = q * factor + t / 10;
    lemma_fundamental_div_mod(n as int, 10);
    lemma_fundamental_div_mod(t as int, 10);
    assert(next > 0) by (nonlinear_arith)
        requires
            q > 0,
            factor > 0,
            t / 10 >= 0,
            next == q * factor + t / 10,
    ;
    assert(n * factor + carry == 10 * next + t % 10) by (nonlinear_arith)
        requires
            n == 10 * q + d,
            t == d * factor + carry,
            t == 10 * (t / 10) + t % 10,
            next == q * factor + t / 10,
    ;
    lemma_fundamental_div_mod_converse(
        (n * factor + carry) as int,
        10,
        next as int,
        (t % 10) as int,
    );
    reveal_with_fuel(udec_bytes, 1);
}

fn multiply_digit(digit: usize, factor: usize, carry: usize) -> (r: usize)
    requires
        digit < 10,
        1 <= factor <= 604800,
        carry < factor,
    ensures
        r == digit * factor + carry,
        r < 10 * factor,
{
    proof {
        assert(0 <= digit * factor + carry < 6048000) by (nonlinear_arith)
            requires
                0 <= digit < 10,
                1 <= factor <= 604800,
                0 <= carry < factor,
        ;
        assert(digit * factor + carry < 10 * factor) by (nonlinear_arith)
            requires
                0 <= digit < 10,
                factor > 0,
                0 <= carry < factor,
        ;
    }
    digit * factor + carry
}

// Carry stays below factor; only one decimal digit enters machine arithmetic.
pub fn multiply(magnitude: &Vec<u8>, factor: usize, n: Ghost<nat>) -> (out: Vec<u8>)
    requires
        magnitude@ == udec_bytes(n@),
        1 <= factor <= 604800,
    ensures
        out@ == udec_bytes(n@ * factor as nat),
{
    proof {
        decimal_shape(n@);
    }
    let mut i = magnitude.len();
    let mut carry = 0usize;
    let mut digits: Vec<u8> = Vec::new();
    let ghost mut q = n@;
    proof {
        assert_seqs_equal!(magnitude@.take(i as int) == magnitude@);
        assert_seqs_equal!(digits@.reverse() == Seq::<u8>::empty());
    }
    while i > 1
        invariant
            1 <= i <= magnitude@.len(),
            1 <= factor <= 604800,
            carry < factor,
            magnitude@.take(i as int) == udec_bytes(q),
            udec_bytes(n@ * factor as nat) == udec_bytes(q * factor as nat + carry as nat)
                + digits@.reverse(),
        decreases i,
    {
        proof {
            decimal_shape(q);
            assert(q >= 10);
            assert(magnitude@[i as int - 1] == udec_bytes(q).last());
            reveal(digit_byte);
        }
        let d = (magnitude[i - 1] - 0x30) as usize;
        let t = multiply_digit(d, factor, carry);
        let next = t / 10;
        let digit = 0x30u8 + (t % 10) as u8;
        let ghost prior = digits@;
        digits.push(digit);
        proof {
            multiply_step(q, factor as nat, carry as nat);
            assert_seqs_equal!(digits@.reverse() == seq![digit] + prior.reverse());
            assert_seqs_equal!(magnitude@.take(i as int - 1) == magnitude@.take(i as int).drop_last());
            q = q / 10;
        }
        carry = next;
        i -= 1;
    }
    proof {
        decimal_shape(q);
        assert(q < 10);
        assert(magnitude@[0] == digit_byte(q as int));
        reveal(digit_byte);
    }
    let leading = multiply_digit((magnitude[0] - 0x30) as usize, factor, carry);
    let mut out = udec_vec(leading);
    while digits.len() > 0
        invariant
            out@ + digits@.reverse() == udec_bytes(n@ * factor as nat),
        decreases digits.len(),
    {
        let ghost before = digits@;
        let digit = digits.pop().unwrap();
        proof {
            assert_seqs_equal!(before.reverse() == seq![digit] + digits@.reverse());
        }
        out.push(digit);
    }
    out
}

} // verus!
