use vstd::prelude::*;

verus! {
    proof fn test(f: spec_fn(int) -> int)
        requires f(5) == f(6),
        ensures false,
    {
        assert(f(5) != f(6)) by (compute_only);
    }
}

fn main() {}
