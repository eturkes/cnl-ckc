use vstd::prelude::*;

verus! {
    spec fn call<const N: u64>(f: spec_fn() -> int) -> int { f() }

    proof fn test<const N: u64>() {
        assert(call::<5>(|| N as int) == N as int) by (compute_only);
    }
}

fn main() {}
