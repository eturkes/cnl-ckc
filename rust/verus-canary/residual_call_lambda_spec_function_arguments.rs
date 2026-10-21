use vstd::prelude::*;

verus! {
    spec fn h(f: spec_fn(int) -> int, a: int) -> int { f(a) }

    proof fn lemma(f: spec_fn(int) -> int) ensures h(f, 5) == h(f, 6) {
        assert(h(f, 5) == h(f, 6)) by (compute_only);
    }

    proof fn test() ensures false {
        lemma(|x: int| x);
    }
}

fn main() {}
