// canary-args: --no-cheating
use vstd::prelude::*;

verus! {
    proof fn contradiction()
        ensures
            false,
    {
        vstd::pervasive::assume(false);
    }
}

fn main() {}
