use vstd::prelude::*;

verus! {
    spec fn apply(f: spec_fn(int) -> int, y: int) -> int { f(0) }

    proof fn test(y: int) {
        assert(apply(|z: int| y, 5) == y) by (compute_only);
    }
}

fn main() {}
