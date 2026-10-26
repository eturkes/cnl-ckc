use vstd::prelude::*;

#[allow(unused_macros)]
macro_rules! panic {
    ($($t:tt)*) => {
        ()
    };
}

verus! {

pub struct S {
    pub a: u64,
    pub b: u64,
}

global layout S is size == 4, align == 1;

fn size() -> (r: usize)
    ensures
        r == 4,
{
    core::mem::size_of::<S>()
}

fn main() {
}

} // verus!
