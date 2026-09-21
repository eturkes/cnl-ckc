use ckc_spec::{check as ck, ui as u, v1text as v};
use vstd::prelude::*;
verus! {

pub open spec fn step(s: u::Bytes, c: int) -> (int, int) {
    if c == 0 && ck::starts(s, seq![60u8, 115, 116, 121, 108, 101, 62]) {
        (7, 5)
    } else if c == 0 && ck::starts(s, seq![60u8, 115, 99, 114, 105, 112, 116, 62]) {
        (8, 6)
    } else if c == 0 && ck::starts(s, seq![60u8, 33, 45, 45]) {
        (4, 4)
    } else if c == 4 && ck::starts(s, seq![45u8, 45, 62]) {
        (3, 0)
    } else if c == 5 && ck::starts(s, seq![60u8, 47, 115, 116, 121, 108, 101, 62]) {
        (8, 0)
    } else if c == 6 && ck::starts(s, seq![60u8, 47, 115, 99, 114, 105, 112, 116, 62]) {
        (9, 0)
    } else {
        (
            1,
            if c == 0 && s[0] == 60 {
                1
            } else if c == 1 && s[0] == 34 {
                2
            } else if c == 1 && s[0] == 39 {
                3
            } else if c == 1 && s[0] == 62 {
                0
            } else if (c == 2 && s[0] == 34) || (c == 3 && s[0] == 39) {
                1
            } else {
                c
            },
        )
    }
}

pub open spec fn scan(s: u::Bytes, c: int) -> int
    decreases s.len(),
{
    if s.len() == 0 {
        c
    } else {
        let (n, next) = step(s, c);
        if 0 < n <= s.len() {
            scan(s.skip(n), next)
        } else {
            -1
        }
    }
}

pub proof fn prefixes()
    ensures
        v::ascii("<style>"@) == seq![60u8, 115, 116, 121, 108, 101, 62],
        v::ascii("<script>"@) == seq![60u8, 115, 99, 114, 105, 112, 116, 62],
        v::ascii("<!--"@) == seq![60u8, 33, 45, 45],
        v::ascii("-->"@) == seq![45u8, 45, 62],
        v::ascii("</style>"@) == seq![60u8, 47, 115, 116, 121, 108, 101, 62],
        v::ascii("</script>"@) == seq![60u8, 47, 115, 99, 114, 105, 112, 116, 62],
{
    reveal_strlit("<style>");
    reveal_strlit("<script>");
    reveal_strlit("<!--");
    reveal_strlit("-->");
    reveal_strlit("</style>");
    reveal_strlit("</script>");
    assert(v::ascii("<style>"@) =~= seq![60u8, 115, 116, 121, 108, 101, 62]);
    assert(v::ascii("<script>"@) =~= seq![60u8, 115, 99, 114, 105, 112, 116, 62]);
    assert(v::ascii("<!--"@) =~= seq![60u8, 33, 45, 45]);
    assert(v::ascii("-->"@) =~= seq![45u8, 45, 62]);
    assert(v::ascii("</style>"@) =~= seq![60u8, 47, 115, 116, 121, 108, 101, 62]);
    assert(v::ascii("</script>"@) =~= seq![60u8, 47, 115, 99, 114, 105, 112, 116, 62]);
}

pub proof fn exact(s: u::Bytes, c: int)
    ensures
        scan(s, c) == u::fixed_context(s, c),
    decreases s.len(),
{
    prefixes();
    if s.len() > 0 {
        let (n, next) = step(s, c);
        if 0 < n <= s.len() {
            exact(s.skip(n), next);
        }
    }
}

pub open spec fn absent(s: u::Bytes, b: u8, a: nat, z: nat) -> bool
    decreases z - a,
{
    if a >= z {
        true
    } else if a + 1 == z {
        s[a as int] != b
    } else {
        let m = (a + z) / 2;
        absent(s, b, a, m) && absent(s, b, m, z)
    }
}

pub proof fn absent_at(s: u::Bytes, b: u8, a: nat, z: nat, i: int)
    requires
        absent(s, b, a, z),
        a <= i < z,
    ensures
        s[i] != b,
    decreases z - a,
{
    if a + 1 < z {
        let m = (a + z) / 2;
        if i < m {
            absent_at(s, b, a, m, i);
        } else {
            absent_at(s, b, m, z, i);
        }
    }
}

pub proof fn absent_all(s: u::Bytes, b: u8)
    requires
        absent(s, b, 0, s.len()),
    ensures
        forall|i: int| 0 <= i < s.len() ==> #[trigger] s[i] != b,
{
    assert forall|i: int| 0 <= i < s.len() implies #[trigger] s[i] != b by {
        absent_at(s, b, 0, s.len(), i);
    }
}

pub proof fn style(s: u::Bytes)
    requires
        forall|i: int| 0 <= i < s.len() ==> #[trigger] s[i] != 60,
    ensures
        scan(s, 5) == 5,
    decreases s.len(),
{
    if s.len() > 0 {
        style(s.drop_first());
        assert(s.skip(1) =~= s.drop_first());
    }
}

pub proof fn script_middle(s: u::Bytes)
    requires
        forall|i: int| 0 <= i < s.len() ==> #[trigger] s[i] != 47,
    ensures
        scan(s, 6) == 6,
    decreases s.len(),
{
    if s.len() > 0 {
        script_middle(s.drop_first());
        assert(s.skip(1) =~= s.drop_first());
    }
}

pub proof fn prefix_line(a: u::Bytes, b: u::Bytes, p: u::Bytes)
    requires
        a.len() > 0,
        a.last() == 10,
        forall|i: int| 0 <= i < p.len() ==> #[trigger] p[i] != 10,
    ensures
        ck::starts(a + b, p) == ck::starts(a, p),
{
    if p.len() <= a.len() {
        assert((a + b).take(p.len() as int) =~= a.take(p.len() as int));
    } else if ck::starts(a + b, p) {
        assert(p[a.len() - 1] == 10);
    }
}

pub proof fn step_line(a: u::Bytes, b: u::Bytes, c: int)
    requires
        a.len() > 0,
        a.last() == 10,
    ensures
        step(a + b, c) == step(a, c),
{
    prefix_line(a, b, seq![60u8, 115, 116, 121, 108, 101, 62]);
    prefix_line(a, b, seq![60u8, 115, 99, 114, 105, 112, 116, 62]);
    prefix_line(a, b, seq![60u8, 33, 45, 45]);
    prefix_line(a, b, seq![45u8, 45, 62]);
    prefix_line(a, b, seq![60u8, 47, 115, 116, 121, 108, 101, 62]);
    prefix_line(a, b, seq![60u8, 47, 115, 99, 114, 105, 112, 116, 62]);
}

pub proof fn step_bounds(s: u::Bytes, c: int)
    requires
        s.len() > 0,
    ensures
        0 < step(s, c).0 <= s.len(),
{
}

pub proof fn add_line(a: u::Bytes, b: u::Bytes, c: int)
    requires
        a.len() > 0,
        a.last() == 10,
    ensures
        scan(a + b, c) == scan(b, scan(a, c)),
    decreases a.len(),
{
    hide(step);
    reveal_with_fuel(scan, 2);
    step_line(a, b, c);
    step_bounds(a, c);
    let (n, next) = step(a, c);
    assert((a + b).skip(n) =~= a.skip(n) + b);
    if n < a.len() {
        add_line(a.skip(n), b, next);
    } else {
        assert(a.skip(n) + b =~= b);
    }
}

} // verus!
