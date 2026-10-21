use vstd::prelude::*;

verus! {
    spec fn g(x: int) -> int { x }
    spec fn c(a: int) -> int { choose|x: int| #[trigger] g(x) == a }

    proof fn test() ensures false {
        assert(c(5) == c(6)) by (compute_only);
        assert(g(5) == 5);
        assert(g(6) == 6);
    }
}

fn main() {}
