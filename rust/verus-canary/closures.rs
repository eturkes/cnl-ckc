use vstd::prelude::*;

verus! {

    fn test(x: u64) {
        assert((|x:int| x + 1)(5) == 6) by (compute_only);
        let y = 5;
        // This cannot use compute_only, since it relies on
        // z3 filling in the value of y
        assert((|x:int| x + y)(5) == 10) by (compute);
        assert({
            let y:int = 10;
            (|x:int| x + y)(5) == 15
        }) by (compute_only);
        assert((|x:int,y:int| x + y)(40, 2) == 42) by (compute_only);
    }

    spec fn call_it(f: spec_fn(int) -> int, arg: int) -> int {
        let y: int = 100;
        f(arg)
    }

    proof fn scoping_test() {
        assert({
            let y: int = 10;
            call_it(|x: int| x + y, 3) == 13
        }) by (compute_only);
    }
}

fn main() {}
