use vstd::assert_seqs_equal;
use vstd::prelude::*;
verus! {

pub fn newline() -> (r: Vec<char>)
    ensures
        r@ == "input lacks final newline"@,
{
    let r = vstd::slice::slice_to_vec(
        &[
            'i',
            'n',
            'p',
            'u',
            't',
            ' ',
            'l',
            'a',
            'c',
            'k',
            's',
            ' ',
            'f',
            'i',
            'n',
            'a',
            'l',
            ' ',
            'n',
            'e',
            'w',
            'l',
            'i',
            'n',
            'e',
        ],
    );
    proof {
        reveal_strlit("input lacks final newline");
        assert_seqs_equal!(r@ == "input lacks final newline"@);
    }
    r
}

pub fn empty() -> (r: Vec<char>)
    ensures
        r@ == "empty input"@,
{
    let r = vstd::slice::slice_to_vec(&['e', 'm', 'p', 't', 'y', ' ', 'i', 'n', 'p', 'u', 't']);
    proof {
        reveal_strlit("empty input");
        assert_seqs_equal!(r@ == "empty input"@);
    }
    r
}

pub fn fields() -> (r: Vec<char>)
    ensures
        r@ == "expected group, side, occurrence, span"@,
{
    let r = vstd::slice::slice_to_vec(
        &[
            'e',
            'x',
            'p',
            'e',
            'c',
            't',
            'e',
            'd',
            ' ',
            'g',
            'r',
            'o',
            'u',
            'p',
            ',',
            ' ',
            's',
            'i',
            'd',
            'e',
            ',',
            ' ',
            'o',
            'c',
            'c',
            'u',
            'r',
            'r',
            'e',
            'n',
            'c',
            'e',
            ',',
            ' ',
            's',
            'p',
            'a',
            'n',
        ],
    );
    proof {
        reveal_strlit("expected group, side, occurrence, span");
        assert_seqs_equal!(r@ == "expected group, side, occurrence, span"@);
    }
    r
}

pub fn group() -> (r: Vec<char>)
    ensures
        r@ == "group must be a canonical decimal"@,
{
    let r = vstd::slice::slice_to_vec(
        &[
            'g',
            'r',
            'o',
            'u',
            'p',
            ' ',
            'm',
            'u',
            's',
            't',
            ' ',
            'b',
            'e',
            ' ',
            'a',
            ' ',
            'c',
            'a',
            'n',
            'o',
            'n',
            'i',
            'c',
            'a',
            'l',
            ' ',
            'd',
            'e',
            'c',
            'i',
            'm',
            'a',
            'l',
        ],
    );
    proof {
        reveal_strlit("group must be a canonical decimal");
        assert_seqs_equal!(r@ == "group must be a canonical decimal"@);
    }
    r
}

pub fn negative() -> (r: Vec<char>)
    ensures
        r@ == "group below 0"@,
{
    let r = vstd::slice::slice_to_vec(
        &['g', 'r', 'o', 'u', 'p', ' ', 'b', 'e', 'l', 'o', 'w', ' ', '0'],
    );
    proof {
        reveal_strlit("group below 0");
        assert_seqs_equal!(r@ == "group below 0"@);
    }
    r
}

pub fn occurrence() -> (r: Vec<char>)
    ensures
        r@ == "occurrence must be a canonical decimal"@,
{
    let r = vstd::slice::slice_to_vec(
        &[
            'o',
            'c',
            'c',
            'u',
            'r',
            'r',
            'e',
            'n',
            'c',
            'e',
            ' ',
            'm',
            'u',
            's',
            't',
            ' ',
            'b',
            'e',
            ' ',
            'a',
            ' ',
            'c',
            'a',
            'n',
            'o',
            'n',
            'i',
            'c',
            'a',
            'l',
            ' ',
            'd',
            'e',
            'c',
            'i',
            'm',
            'a',
            'l',
        ],
    );
    proof {
        reveal_strlit("occurrence must be a canonical decimal");
        assert_seqs_equal!(r@ == "occurrence must be a canonical decimal"@);
    }
    r
}

pub fn zero() -> (r: Vec<char>)
    ensures
        r@ == "occurrence below 1"@,
{
    let r = vstd::slice::slice_to_vec(
        &['o', 'c', 'c', 'u', 'r', 'r', 'e', 'n', 'c', 'e', ' ', 'b', 'e', 'l', 'o', 'w', ' ', '1'],
    );
    proof {
        reveal_strlit("occurrence below 1");
        assert_seqs_equal!(r@ == "occurrence below 1"@);
    }
    r
}

pub fn span() -> (r: Vec<char>)
    ensures
        r@ == "empty span"@,
{
    let r = vstd::slice::slice_to_vec(&['e', 'm', 'p', 't', 'y', ' ', 's', 'p', 'a', 'n']);
    proof {
        reveal_strlit("empty span");
        assert_seqs_equal!(r@ == "empty span"@);
    }
    r
}

pub fn side() -> (r: Vec<char>)
    ensures
        r@ == "side must be src or ace"@,
{
    let r = vstd::slice::slice_to_vec(
        &[
            's',
            'i',
            'd',
            'e',
            ' ',
            'm',
            'u',
            's',
            't',
            ' ',
            'b',
            'e',
            ' ',
            's',
            'r',
            'c',
            ' ',
            'o',
            'r',
            ' ',
            'a',
            'c',
            'e',
        ],
    );
    proof {
        reveal_strlit("side must be src or ace");
        assert_seqs_equal!(r@ == "side must be src or ace"@);
    }
    r
}

pub fn occ() -> (r: Vec<char>)
    ensures
        r@ == "occurrence "@,
{
    let r = vstd::slice::slice_to_vec(&['o', 'c', 'c', 'u', 'r', 'r', 'e', 'n', 'c', 'e', ' ']);
    proof {
        reveal_strlit("occurrence ");
        assert_seqs_equal!(r@ == "occurrence "@);
    }
    r
}

pub fn not_found() -> (r: Vec<char>)
    ensures
        r@ == " of span not found in "@,
{
    let r = vstd::slice::slice_to_vec(
        &[
            ' ',
            'o',
            'f',
            ' ',
            's',
            'p',
            'a',
            'n',
            ' ',
            'n',
            'o',
            't',
            ' ',
            'f',
            'o',
            'u',
            'n',
            'd',
            ' ',
            'i',
            'n',
            ' ',
        ],
    );
    proof {
        reveal_strlit(" of span not found in ");
        assert_seqs_equal!(r@ == " of span not found in "@);
    }
    r
}

pub fn groups() -> (r: Vec<char>)
    ensures
        r@ == "every group needs both a src span and an ace span"@,
{
    let r = vstd::slice::slice_to_vec(
        &[
            'e',
            'v',
            'e',
            'r',
            'y',
            ' ',
            'g',
            'r',
            'o',
            'u',
            'p',
            ' ',
            'n',
            'e',
            'e',
            'd',
            's',
            ' ',
            'b',
            'o',
            't',
            'h',
            ' ',
            'a',
            ' ',
            's',
            'r',
            'c',
            ' ',
            's',
            'p',
            'a',
            'n',
            ' ',
            'a',
            'n',
            'd',
            ' ',
            'a',
            'n',
            ' ',
            'a',
            'c',
            'e',
            ' ',
            's',
            'p',
            'a',
            'n',
        ],
    );
    proof {
        reveal_strlit("every group needs both a src span and an ace span");
        assert_seqs_equal!(r@ == "every group needs both a src span and an ace span"@);
    }
    r
}

pub fn overlap() -> (r: Vec<char>)
    ensures
        r@ == "overlapping "@,
{
    let r = vstd::slice::slice_to_vec(
        &['o', 'v', 'e', 'r', 'l', 'a', 'p', 'p', 'i', 'n', 'g', ' '],
    );
    proof {
        reveal_strlit("overlapping ");
        assert_seqs_equal!(r@ == "overlapping "@);
    }
    r
}

pub fn offset() -> (r: Vec<char>)
    ensures
        r@ == " spans at offset "@,
{
    let r = vstd::slice::slice_to_vec(
        &[' ', 's', 'p', 'a', 'n', 's', ' ', 'a', 't', ' ', 'o', 'f', 'f', 's', 'e', 't', ' '],
    );
    proof {
        reveal_strlit(" spans at offset ");
        assert_seqs_equal!(r@ == " spans at offset "@);
    }
    r
}

pub fn src() -> (r: Vec<char>)
    ensures
        r@ == "src"@,
{
    let r = vstd::slice::slice_to_vec(&['s', 'r', 'c']);
    proof {
        reveal_strlit("src");
        assert_seqs_equal!(r@ == "src"@);
    }
    r
}

pub fn ace() -> (r: Vec<char>)
    ensures
        r@ == "ace"@,
{
    let r = vstd::slice::slice_to_vec(&['a', 'c', 'e']);
    proof {
        reveal_strlit("ace");
        assert_seqs_equal!(r@ == "ace"@);
    }
    r
}

pub fn digit_zero() -> (r: Vec<char>)
    ensures
        r@ == "0"@,
{
    let mut r = Vec::new();
    r.push('0');
    proof {
        reveal_strlit("0");
        assert_seqs_equal!(r@ == "0"@);
    }
    r
}

} // verus!
