use vstd::prelude::*;

verus! {
    #[verifier::exec]
    proof fn helper()
        no_unwind
    {
    }

    fn main()
        no_unwind
    {
        helper();
    }
}
