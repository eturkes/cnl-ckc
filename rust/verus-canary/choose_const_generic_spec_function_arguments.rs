use vstd::prelude::*;

verus! {
    spec fn g(x: int) -> int { x }
    spec fn c<const N: u64, const M: u64>(a: int) -> int {
        choose|x: int| #[trigger] g(x) == a + M as int
    }

    proof fn test<const N: u64>(a: int) {
        assert(c::<5, N>(a) == c::<N, N>(a)) by (compute);
    }
}

fn main() {}
