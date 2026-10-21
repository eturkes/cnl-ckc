use vstd::prelude::*;

verus! {
    spec fn f(x: int, y: int) -> bool { x == y }

    spec fn g(x: int) -> bool { f(3int, x) }

    spec fn sum(x: nat) -> nat
        decreases(x)
    {
        if x == 0 { 0 } else { 1 + sum((x - 1) as nat) }
    }

    uninterp spec fn f_no_body(x: nat) -> nat;

    uninterp spec fn g_no_body(x: nat) -> nat;

    fn test() {
        assert(sum(20) == 20) by (compute_only);
        assert(sum(10) == 10) by (compute_only);
        assert(sum(0) == 0) by (compute_only);
        assert({
            let x = 22;
            let z = g(x);
            !!!z    // Exercise an unusual recursive case inside the interpreter
        }) by (compute_only);
        assert(f_no_body(5) == f_no_body(5)) by (compute_only);
    }
}

fn main() {}
