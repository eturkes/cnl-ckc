use vstd::prelude::*;

verus! {

spec const N: usize = 5;

fn f() -> (r: usize)
    ensures
        r == 8,
{
    let a = [0u8;core::mem::size_of_val(&N)];
    a.len()
}

fn main() {
}

} // verus!
