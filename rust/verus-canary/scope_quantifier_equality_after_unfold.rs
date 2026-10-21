use vstd::prelude::*;

verus! {
    uninterp spec fn p(x: int) -> bool;
    spec fn id(x: int) -> int { x }

    proof fn test() {
        assert((forall|x: int| #[trigger] p(id(x)))
            == (forall|x: int| #[trigger] p(x))) by (compute_only);
    }
}

fn main() {}
