use vstd::prelude::*;

verus! {
    spec fn h<const N: i8>() -> i8 { N }
    spec fn wide<const N: i128>() -> i128 { N }
    spec fn pointer_sized<const N: isize>() -> isize { N }

    proof fn test() {
        assert(h::<{-1}>() == -1) by (compute_only);
        assert(h::<{i8::MIN}>() == -128) by (compute_only);
        assert(wide::<{i128::MIN}>() == i128::MIN) by (compute_only);
        assert(wide::<{i128::MAX}>() == i128::MAX) by (compute_only);
        assert(pointer_sized::<{-1}>() == -1) by (compute_only);
    }
}

fn main() {}
