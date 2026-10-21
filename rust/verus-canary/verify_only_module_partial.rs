// canary-args: --verify-only-module good
use vstd::prelude::*;

verus! {
    mod good {
        pub proof fn ok() {}
    }

    mod bad {
        pub proof fn contradiction()
            ensures
                false,
        {
        }
    }
}

fn main() {}
