#[cfg(not(target_pointer_width = "64"))]
compile_error!("This canary requires a 64-bit verifier host.");

use vstd::prelude::*;

verus! {
    spec fn h<const N: isize>(n: nat) -> isize
        decreases n,
    {
        if n == 0 { N } else { h::<N>((n - 1) as nat) }
    }

    proof fn test() requires arch_word_bits() == 32 ensures false {
        let x = h::<{ isize::MIN }>(0);
        assert(x >= -2147483648);
        assert(h::<{ isize::MIN }>(0) == -9223372036854775808int) by (compute_only);
        assert(false);
    }
}

fn main() {}
