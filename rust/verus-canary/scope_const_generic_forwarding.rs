use vstd::prelude::*;

verus! {
    spec fn h<const N: u64, const M: u64>() -> int { M as int }
    spec fn forward<const N: u64, const M: u64>() -> int { h::<N, M>() }

    proof fn test<const N: u64>() {
        assert(forward::<5, N>() == N as int) by (compute_only);
    }
}

fn main() {}
