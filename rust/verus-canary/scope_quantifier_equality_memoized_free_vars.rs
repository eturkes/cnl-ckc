use vstd::prelude::*;

verus! {
    uninterp spec fn p(x: int, y: int) -> bool;

    #[verifier::memoize]
    spec fn cached(b: bool) -> bool { b }

    proof fn test(y: int, z: int) {
        assert(cached(forall|x: int| #[trigger] p(x, y))
            == cached(forall|x: int| #[trigger] p(x, z))) by (compute_only);
    }
}

fn main() {}
