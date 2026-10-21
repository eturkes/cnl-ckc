use vstd::prelude::*;

verus! {
    spec fn h<const N: u64, const M: u64>() -> int { M as int }

    proof fn test<const N: u64>() {
        assert(h::<5, N>() == 5) by (compute_only);
    }
}

fn main() {}
