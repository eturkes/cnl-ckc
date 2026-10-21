use vstd::prelude::*;

verus! {
    uninterp spec fn p(x: int, y: int) -> bool;
    spec fn id(x: int) -> int { x }

    proof fn test() {
        assert((forall|x: int| #[trigger] p(id(x), id(x))
                && (exists|x: int| #[trigger] p(id(x), id(x))))
            == (forall|y: int| #[trigger] p(y, y)
                && (exists|z: int| #[trigger] p(z, z)))) by (compute_only);
        assert((forall|x: int| #[trigger] p(x, x)
                && (exists|y: int| #[trigger] p(x, y)))
            == (forall|y: int| #[trigger] p(y, y)
                && (exists|x: int| #[trigger] p(y, x)))) by (compute_only);
    }
}

fn main() {}
