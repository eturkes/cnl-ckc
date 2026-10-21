use vstd::prelude::*;

verus! {
    proof fn test() ensures false {
        let c = true;
        assert((if c { 1int } else { 2int }) != (if c { 1int } else { 3int })) by(compute_only);
    }
}

fn main() {}
