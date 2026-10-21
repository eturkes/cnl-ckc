use vstd::prelude::*;

verus! {
    spec fn g(x: int) -> int { x }
    spec fn f(y: int) -> bool { exists|x: int| #[trigger] g(x) == x && x != y }

    proof fn test(x: int) {
        assert(g(x + 1) == x + 1);
        assert(f(x)) by (compute);
    }
}

fn main() {}
