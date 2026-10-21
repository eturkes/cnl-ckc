use vstd::prelude::*;

verus! {
    proof fn test(c: bool) {
        assert((if c { 1int } else { 2int }) == (if c { 1int } else { 2int })) by(compute_only);
        assert((if c { 1int } else { 2int }) != (if c { 3int } else { 4int })) by(compute_only);
    }
}

fn main() {}
