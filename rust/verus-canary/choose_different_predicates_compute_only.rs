use vstd::prelude::*;

verus! {
    struct U;

    uninterp spec fn p(x: U) -> bool;

    proof fn test() ensures false {
        // Different predicates can choose the same value; U has only one value.
        assert((choose|x: U| #![trigger p(x)] true) != (choose|x: U| #![trigger p(x)] false))
            by(compute_only);
    }
}

fn main() {}
