use vstd::prelude::*;

verus! {

mod m {
    pub fn helper()
        no_unwind
    {
    }

}

use m::*;

#[cfg(any())]
proof fn helper() {
}

fn main()
    no_unwind
{
    helper();
}

} // verus!
