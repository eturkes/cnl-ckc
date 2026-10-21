use vstd::prelude::*;

verus! {
    spec fn g(x: int) -> int { x }
    spec fn c(a: int) -> int { choose|x: int| #[trigger] g(x) == a }
    spec fn h(f: spec_fn(int) -> int, a: int) -> int { f(a) }
    spec fn forward(a: int, x: int) -> int { a }

    proof fn test(x: int, f: spec_fn(int) -> int)
        requires f(5) == 9,
    {
        assert(g(5) == 5);
        assert(g(x) == x);
        assert(c(5) == 5) by (compute);
        assert(c(x) == x) by (compute);
        assert(c(5) == c(5)) by (compute_only);
        assert(forward(c(x), 6) == c(x)) by (compute_only);
        assert(h(f, 5) == 9) by (compute);
        assert(h(f, 1int + 4) == f(5)) by (compute_only);
    }
}

fn main() {}
