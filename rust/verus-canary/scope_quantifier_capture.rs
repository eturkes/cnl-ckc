use vstd::prelude::*;

verus! {
    spec fn g(x: int) -> int { x }
    spec fn f(y: int) -> bool { exists|x: int| #[trigger] g(x) == x && x != y }

    proof fn test() {
        assert(forall|x: int| !#[trigger] f(x)) by (compute); // FAILS
    }
}

fn main() {}
