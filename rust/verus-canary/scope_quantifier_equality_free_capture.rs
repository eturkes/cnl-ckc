use vstd::prelude::*;

verus! {
    uninterp spec fn p(x: int, y: int) -> bool;
    spec fn id(x: int) -> int { x }

    proof fn test(x: int) {
        assert((forall|y: int| #[trigger] p(id(y), x))
            == (forall|x: int| #[trigger] p(x, x))) by (compute_only);
    }
}

fn main() {}
