use vstd::prelude::*;

verus! {
    proof fn test() {
        assert(2int + 3int == 5int) by (compute_only);
    }
}

fn main() {}
