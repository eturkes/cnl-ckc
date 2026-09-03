use ckc_spec::term::Term;
#[cfg(verus_keep_ghost)]
use vstd::arithmetic::div_mod::{lemma_fundamental_div_mod, lemma_fundamental_div_mod_converse};
use vstd::prelude::*;
use vstd::{assert_seqs_equal, assert_sets_equal};

verus! {

pub enum ETermTop {
    Var,
    Int,
    Nil,
    Atom(Vec<u8>),
    Comp(Vec<u8>, usize),
}

pub struct EParsedTerm {
    pub term: Ghost<Term>,
    pub top: ETermTop,
    pub ground: bool,
    pub no_dollar: bool,
}

impl View for EParsedTerm {
    type V = Term;

    open spec fn view(&self) -> Term {
        self.term@
    }
}

pub open spec fn top_matches(top: &ETermTop, t: Term) -> bool {
    match (top, t) {
        (ETermTop::Var, Term::Var(_)) => true,
        (ETermTop::Int, Term::Int(_)) => true,
        (ETermTop::Nil, Term::Nil) => true,
        (ETermTop::Atom(name), Term::Atom(spec_name)) => name@ == spec_name,
        (ETermTop::Comp(name, arity), Term::Comp(spec_name, args)) => {
            name@ == spec_name && *arity == args.len()
        },
        _ => false,
    }
}

pub open spec fn parsed_term_ok(t: &EParsedTerm) -> bool {
    &&& ckc_spec::term::wf_term(t@)
    &&& t.ground == ckc_spec::term::ground(t@)
    &&& t.no_dollar == ckc_spec::term::no_dollar_var(t@)
    &&& top_matches(&t.top, t@)
}

pub fn nil_term() -> (out: EParsedTerm)
    ensures
        out@ == Term::Nil,
        parsed_term_ok(&out),
{
    EParsedTerm {
        term: Ghost(Term::Nil),
        top: ETermTop::Nil,
        ground: true,
        no_dollar: true,
    }
}

pub struct EParsedAtom {
    pub name: Vec<u8>,
    pub end: usize,
}

pub ghost struct GAtomExpected {
    pub name: Seq<u8>,
    pub end: usize,
}

pub open spec fn parsed_atom_ok(bytes: Seq<u8>, start: usize, a: &EParsedAtom) -> bool {
    &&& start < a.end <= bytes.len()
    &&& ckc_spec::v1text::atom_bytes(a.name@)
        == bytes.subrange(start as int, a.end as int)
}

proof fn subrange_push<A>(s: Seq<A>, start: int, end: int)
    requires 0 <= start <= end < s.len(),
    ensures
        s.subrange(start, end + 1) == s.subrange(start, end).push(s[end]),
{
    assert_seqs_equal!(s.subrange(start, end + 1) == s.subrange(start, end).push(s[end]));
}

fn is_lower_b(b: u8) -> (r: bool)
    ensures r == ckc_spec::v1text::is_lower_b(b),
{
    0x61 <= b && b <= 0x7a
}

fn is_digit_b(b: u8) -> (r: bool)
    ensures r == ckc_spec::v1text::is_digit_b(b),
{
    0x30 <= b && b <= 0x39
}

fn raise_at(at: &mut usize, boundary: usize, limit: usize)
    requires
        *old(at) <= limit,
        boundary <= limit,
    ensures
        *old(at) <= *final(at) <= limit,
        boundary <= *final(at),
{
    if *at < boundary {
        *at = boundary;
    }
}

fn match_slice_at(bytes: &[u8], start: usize, lit: &[u8], at: &mut usize) -> (r: bool)
    requires
        start <= bytes@.len(),
        *old(at) <= bytes@.len(),
    ensures
        r == {
            &&& start as int + lit@.len() <= bytes@.len()
            &&& bytes@.subrange(start as int, start as int + lit@.len()) == lit@
        },
        *old(at) <= *final(at) <= bytes@.len(),
        r ==> *final(at) == *old(at),
{
    let mut i = 0usize;
    while i < lit.len()
        invariant
            start <= bytes@.len(),
            i <= lit@.len(),
            i <= bytes@.len() - start,
            *old(at) <= *at <= bytes@.len(),
            *at == *old(at),
            forall|j: int| 0 <= j < i ==> bytes@[start as int + j] == lit@[j],
        decreases lit.len() - i,
    {
        if i == bytes.len() - start {
            raise_at(at, bytes.len(), bytes.len());
            return false;
        }
        let suffix_len = bytes.len() - start;
        let pos = bytes.len() - (suffix_len - i);
        proof {
            assert(pos as int == start as int + i as int);
        }
        if bytes[pos] != lit[i] {
            raise_at(at, pos, bytes.len());
            proof {
                if start as int + lit@.len() <= bytes@.len() {
                    assert(bytes@.subrange(
                        start as int,
                        start as int + lit@.len(),
                    )[i as int] == bytes@[start as int + i as int]);
                    assert(bytes@.subrange(
                        start as int,
                        start as int + lit@.len(),
                    ) != lit@);
                }
            }
            return false;
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(
            bytes@.subrange(start as int, start as int + lit@.len()) == lit@
        );
    }
    true
}

fn consume_literal(
    bytes: &[u8],
    start: usize,
    lit: &[u8],
    expected: Ghost<Seq<u8>>,
at: &mut usize,
) -> (r: Option<usize>)
    requires
        start <= bytes@.len(),
        lit@ == expected@,
        *old(at) <= bytes@.len(),
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r matches Some(_) ==> *final(at) == *old(at),
        r matches Some(end) ==> {
            &&& start <= end <= bytes@.len()
            &&& end as int == start as int + expected@.len()
            &&& bytes@.subrange(start as int, end as int) == expected@
        },
        start as int + expected@.len() <= bytes@.len()
            && bytes@.subrange(
                start as int,
                start as int + expected@.len(),
            ) == expected@
            ==> r == Some((start as int + expected@.len()) as usize),
{
    if match_slice_at(bytes, start, lit, at) {
        let remaining = bytes.len() - start;
        let slack = remaining - lit.len();
        let end = bytes.len() - slack;
        Some(end)
    } else {
        None
    }
}

fn copy_range(bytes: &[u8], start: usize, end: usize) -> (out: Vec<u8>)
    requires start <= end <= bytes@.len(),
    ensures out@ == bytes@.subrange(start as int, end as int),
{
    let mut out = Vec::new();
    let mut i = start;
    while i < end
        invariant
            start <= i <= end <= bytes@.len(),
            out@ == bytes@.subrange(start as int, i as int),
        decreases end - i,
    {
        out.push(bytes[i]);
        proof {
            subrange_push(bytes@, start as int, i as int);
        }
        i += 1;
    }
    out
}

fn vec_equal(left: &Vec<u8>, right: &Vec<u8>) -> (r: bool)
    ensures r == (left@ == right@),
{
    if left.len() != right.len() {
        return false;
    }
    let mut i = 0usize;
    while i < left.len()
        invariant
            left@.len() == right@.len(),
            i <= left@.len(),
            forall|j: int| 0 <= j < i ==> left@[j] == right@[j],
        decreases left.len() - i,
    {
        if left[i] != right[i] {
            return false;
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(left@ == right@);
    }
    true
}

fn vec_slice_equal(left: &Vec<u8>, right: &[u8]) -> (r: bool)
    ensures r == (left@ == right@),
{
    if left.len() != right.len() {
        return false;
    }
    let mut i = 0usize;
    while i < left.len()
        invariant
            left@.len() == right@.len(),
            i <= left@.len(),
            forall|j: int| 0 <= j < i ==> left@[j] == right@[j],
        decreases left.len() - i,
    {
        if left[i] != right[i] {
            return false;
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(left@ == right@);
    }
    true
}

fn consume_byte(bytes: &[u8], start: usize, byte: u8, at: &mut usize) -> (r: Option<usize>)
    requires
        start <= bytes@.len(),
        *old(at) <= bytes@.len(),
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r matches Some(_) ==> *final(at) == *old(at),
        r matches Some(end) ==> {
            &&& start < end <= bytes@.len()
            &&& end == start + 1
            &&& bytes@.subrange(start as int, end as int) == seq![byte]
        },
        start < bytes@.len() && bytes@[start as int] == byte
            ==> r == Some((start as int + 1) as usize),
{
    if start == bytes.len() || bytes[start] != byte {
        raise_at(at, start, bytes.len());
        return None;
    }
    proof {
        assert_seqs_equal!(bytes@.subrange(start as int, start as int + 1)
            == seq![byte]);
    }
    Some(start + 1)
}

pub struct EByteCursor {
    pub pos: usize,
    pub prefix: Ghost<Seq<u8>>,
}

pub closed spec fn cursor_ok(bytes: Seq<u8>, cursor: &EByteCursor) -> bool {
    &&& cursor.pos <= bytes.len()
    &&& cursor.prefix@ == bytes.subrange(0, cursor.pos as int)
}

fn new_cursor(bytes: &[u8]) -> (cursor: EByteCursor)
    ensures
        cursor_ok(bytes@, &cursor),
        cursor.pos == 0,
        cursor.prefix@ == Seq::<u8>::empty(),
{
    proof {
        assert_seqs_equal!(bytes@.subrange(0, 0) == Seq::<u8>::empty());
        reveal(cursor_ok);
    }
    EByteCursor { pos: 0, prefix: Ghost(Seq::empty()) }
}

fn cursor_literal(
    bytes: &[u8],
    cursor: &mut EByteCursor,
    lit: &[u8],
    expected: Ghost<Seq<u8>>,
at: &mut usize,
) -> (r: bool)
    requires
        cursor_ok(bytes@, old(cursor)),
        lit@ == expected@,
        *old(at) <= bytes@.len(),
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r ==> *final(at) == *old(at),
        r ==> {
            &&& cursor_ok(bytes@, final(cursor))
            &&& final(cursor).pos == old(cursor).pos + expected@.len()
            &&& final(cursor).prefix@ == old(cursor).prefix@ + expected@
        },
        !r ==> {
            &&& final(cursor).pos == old(cursor).pos
            &&& final(cursor).prefix@ == old(cursor).prefix@
        },
        old(cursor).pos as int + expected@.len() <= bytes@.len()
            && bytes@.subrange(
                old(cursor).pos as int,
                old(cursor).pos as int + expected@.len(),
            ) == expected@
            ==> r,
{
    let old_pos = cursor.pos;
    let ghost old_prefix = cursor.prefix@;
    let end = match consume_literal(bytes, old_pos, lit, expected, at) {
        Some(end) => end,
        None => return false,
    };
    proof {
        reveal(cursor_ok);
        range_concat(bytes@, 0, old_pos as int, end as int);
        assert(bytes@.subrange(0, end as int) == old_prefix + expected@);
    }
    cursor.pos = end;
    cursor.prefix = Ghost(old_prefix + expected@);
    proof {
        reveal(cursor_ok);
    }
    true
}

fn cursor_byte(
    bytes: &[u8],
    cursor: &mut EByteCursor,
    byte: u8,
at: &mut usize,
) -> (r: bool)
    requires
        cursor_ok(bytes@, old(cursor)),
        *old(at) <= bytes@.len(),
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r ==> *final(at) == *old(at),
        r ==> {
            &&& cursor_ok(bytes@, final(cursor))
            &&& old(cursor).pos < bytes@.len()
            &&& bytes@[old(cursor).pos as int] == byte
            &&& final(cursor).pos == old(cursor).pos + 1
            &&& final(cursor).prefix@ == old(cursor).prefix@ + seq![byte]
        },
        !r ==> {
            &&& final(cursor).pos == old(cursor).pos
            &&& final(cursor).prefix@ == old(cursor).prefix@
        },
        old(cursor).pos < bytes@.len()
            && bytes@[old(cursor).pos as int] == byte
            ==> r,
{
    let old_pos = cursor.pos;
    let ghost old_prefix = cursor.prefix@;
    let end = match consume_byte(bytes, old_pos, byte, at) {
        Some(end) => end,
        None => return false,
    };
    proof {
        reveal(cursor_ok);
        assert(bytes@.subrange(old_pos as int, end as int)[0]
            == bytes@[old_pos as int]);
        assert(bytes@[old_pos as int] == byte);
        range_concat(bytes@, 0, old_pos as int, end as int);
        assert(bytes@.subrange(0, end as int) == old_prefix + seq![byte]);
    }
    cursor.pos = end;
    cursor.prefix = Ghost(old_prefix + seq![byte]);
    proof {
        reveal(cursor_ok);
    }
    true
}

fn cursor_atom(
    bytes: &[u8],
    cursor: &mut EByteCursor,
    expected: Ghost<Option<GAtomExpected>>,
at: &mut usize,
) -> (r: Option<EParsedAtom>)
    requires
        *old(at) <= bytes@.len(),
        cursor_ok(bytes@, old(cursor)),
        expected@ matches Some(e) ==> {
            &&& old(cursor).pos < e.end <= bytes@.len()
            &&& ckc_spec::v1text::atom_bytes(e.name)
                == bytes@.subrange(old(cursor).pos as int, e.end as int)
            &&& atom_boundary(bytes@, e.end as int)
        },
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r matches Some(atom) ==> {
            &&& cursor_ok(bytes@, final(cursor))
            &&& final(cursor).pos == atom.end
            &&& final(cursor).prefix@ == old(cursor).prefix@
                + ckc_spec::v1text::atom_bytes(atom.name@)
            &&& parsed_atom_ok(bytes@, old(cursor).pos, &atom)
        },
        expected@ matches Some(e) ==> r matches Some(atom)
            && atom.name@ == e.name && atom.end == e.end,
{
    let start = cursor.pos;
    let ghost old_prefix = cursor.prefix@;
    if start == bytes.len() {
        raise_at(at, start, bytes.len());
        proof {
            if let Some(e) = expected@ {
                assert(start < e.end <= bytes@.len());
                assert(false);
            }
        }
        return None;
    }
    let atom = match parse_atom(bytes, start, expected, at) {
        Some(atom) => atom,
        None => return None,
    };
    proof {
        reveal(parsed_atom_ok);
        reveal(cursor_ok);
        range_concat(bytes@, 0, start as int, atom.end as int);
        assert(bytes@.subrange(0, atom.end as int)
            == old_prefix + ckc_spec::v1text::atom_bytes(atom.name@));
    }
    cursor.pos = atom.end;
    cursor.prefix = Ghost(
        old_prefix + ckc_spec::v1text::atom_bytes(atom.name@),
    );
    proof {
        reveal(cursor_ok);
    }
    Some(atom)
}

fn cursor_term(
    bytes: &[u8],
    cursor: &mut EByteCursor,
    expected: Ghost<Option<GTermExpected>>,
at: &mut usize,
) -> (r: Option<ESpannedTerm>)
    requires
        *old(at) <= bytes@.len(),
        cursor_ok(bytes@, old(cursor)),
        expected@ matches Some(e) ==>
            term_at(bytes@, old(cursor).pos as int, e.end as int, e.term),
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r matches Some(term) ==> {
            &&& cursor_ok(bytes@, final(cursor))
            &&& final(cursor).pos == term.end
            &&& final(cursor).prefix@ == old(cursor).prefix@
                + ckc_spec::v1text::term_bytes(term@)
            &&& spanned_term_ok(bytes@, &term)
            &&& term.start == old(cursor).pos
        },
        expected@ matches Some(e) ==> r matches Some(term)
            && term@ == e.term && term.end == e.end,
{
    let start = cursor.pos;
    let ghost old_prefix = cursor.prefix@;
    if start == bytes.len() {
        raise_at(at, start, bytes.len());
        proof {
            if let Some(e) = expected@ {
                reveal(term_at);
                assert(start < e.end);
                assert(e.end <= bytes@.len());
                assert(false);
            }
        }
        return None;
    }
    let mut tracker = new_var_tracker();
    let ghost initial_stream = tracker.stream@;
    let ghost tracking_expected = match expected@ {
        Some(e) => e,
        None => GTermExpected { term: Term::Nil, end: start },
    };
    let term = match parse_term(
        bytes,
        start,
        expected,
        Ghost(tracking_expected),
        Ghost(initial_stream),
        false,
        &mut tracker,
    at,
    ) {
        Some(term) => term,
        None => return None,
    };
    proof {
        reveal(spanned_term_ok);
        reveal(cursor_ok);
        range_concat(bytes@, 0, start as int, term.end as int);
        assert(bytes@.subrange(0, term.end as int)
            == old_prefix + ckc_spec::v1text::term_bytes(term@));
    }
    cursor.pos = term.end;
    cursor.prefix = Ghost(
        old_prefix + ckc_spec::v1text::term_bytes(term@),
    );
    proof {
        reveal(cursor_ok);
    }
    Some(term)
}

fn cursor_name(
    bytes: &[u8],
    cursor: &mut EByteCursor,
    expected: Ghost<Option<GNameExpected>>,
at: &mut usize,
) -> (r: Option<ENameField>)
    requires
        *old(at) <= bytes@.len(),
        cursor_ok(bytes@, old(cursor)),
        expected@ matches Some(e) ==> {
            &&& old(cursor).pos < e.end < bytes@.len()
            &&& ckc_spec::v1text::name_ok(e.value)
            &&& e.value == bytes@.subrange(old(cursor).pos as int, e.end as int)
            &&& !(ckc_spec::v1text::is_lower_b(bytes@[e.end as int])
                || ckc_spec::v1text::is_digit_b(bytes@[e.end as int])
                || bytes@[e.end as int] == 0x2d)
        },
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r matches Some(field) ==> {
            &&& cursor_ok(bytes@, final(cursor))
            &&& final(cursor).pos == field.end
            &&& final(cursor).prefix@ == old(cursor).prefix@ + field.value@
            &&& old(cursor).pos < field.end <= bytes@.len()
            &&& field.value@
                == bytes@.subrange(old(cursor).pos as int, field.end as int)
            &&& ckc_spec::v1text::name_ok(field.value@)
        },
        expected@ matches Some(e) ==> r matches Some(field)
            && field.value@ == e.value && field.end == e.end,
{
    let start = cursor.pos;
    let ghost old_prefix = cursor.prefix@;
    if start == bytes.len() {
        raise_at(at, start, bytes.len());
        proof {
            if let Some(e) = expected@ {
                assert(start < e.end < bytes@.len());
                assert(false);
            }
        }
        return None;
    }
    let field = match parse_raw_name(bytes, start, expected, at) {
        Some(field) => field,
        None => return None,
    };
    proof {
        reveal(cursor_ok);
        range_concat(bytes@, 0, start as int, field.end as int);
        assert(bytes@.subrange(0, field.end as int)
            == old_prefix + field.value@);
    }
    cursor.pos = field.end;
    cursor.prefix = Ghost(old_prefix + field.value@);
    proof {
        reveal(cursor_ok);
    }
    Some(field)
}

fn is_name_b(b: u8) -> (r: bool)
    ensures r == {
        ckc_spec::v1text::is_lower_b(b)
            || ckc_spec::v1text::is_digit_b(b)
            || b == 0x2d
    },
{
    is_lower_b(b) || is_digit_b(b) || b == 0x2d
}

fn name_ok_exec(name: &Vec<u8>) -> (r: bool)
    ensures r == ckc_spec::v1text::name_ok(name@),
{
    if name.len() == 0 {
        proof {
            reveal(ckc_spec::v1text::name_ok);
        }
        return false;
    }
    if name[0] == 0x2d {
        proof {
            reveal(ckc_spec::v1text::name_ok);
        }
        return false;
    }
    let mut i = 0usize;
    while i < name.len()
        invariant
            0 < name@.len(),
            name@[0] != 0x2d,
            i <= name@.len(),
            forall|j: int| 0 <= j < i ==> {
                ckc_spec::v1text::is_lower_b(name@[j])
                    || ckc_spec::v1text::is_digit_b(name@[j])
                    || name@[j] == 0x2d
            },
        decreases name.len() - i,
    {
        if !is_name_b(name[i]) {
            proof {
                reveal(ckc_spec::v1text::name_ok);
                reveal(ckc_spec::v1text::all_in);
            }
            return false;
        }
        i += 1;
    }
    proof {
        reveal(ckc_spec::v1text::name_ok);
        reveal(ckc_spec::v1text::all_in);
    }
    true
}

fn is_hex_lower_b(b: u8) -> (r: bool)
    ensures r == ckc_spec::v1text::is_hex_lower_b(b),
{
    is_digit_b(b) || (0x61 <= b && b <= 0x66)
}

fn hex64_exec(hash: &Vec<u8>) -> (r: bool)
    ensures r == ckc_spec::v1text::hex64(hash@),
{
    if hash.len() != 64 {
        proof {
            reveal(ckc_spec::v1text::hex64);
        }
        return false;
    }
    let mut i = 0usize;
    while i < hash.len()
        invariant
            hash@.len() == 64,
            i <= hash@.len(),
            forall|j: int| 0 <= j < i ==>
                ckc_spec::v1text::is_hex_lower_b(hash@[j]),
        decreases hash.len() - i,
    {
        if !is_hex_lower_b(hash[i]) {
            proof {
                reveal(ckc_spec::v1text::hex64);
                reveal(ckc_spec::v1text::all_in);
            }
            return false;
        }
        i += 1;
    }
    proof {
        reveal(ckc_spec::v1text::hex64);
        reveal(ckc_spec::v1text::all_in);
    }
    true
}

pub ghost struct GNameExpected {
    pub value: Seq<u8>,
    pub end: usize,
}

pub struct ENameField {
    pub value: Vec<u8>,
    pub end: usize,
}

fn parse_raw_name(
    bytes: &[u8],
    start: usize,
    expected: Ghost<Option<GNameExpected>>,
at: &mut usize,
) -> (r: Option<ENameField>)
    requires
        *old(at) <= bytes@.len(),
        start < bytes@.len(),
        expected@ matches Some(e) ==> {
            &&& start < e.end < bytes@.len()
            &&& ckc_spec::v1text::name_ok(e.value)
            &&& e.value == bytes@.subrange(start as int, e.end as int)
            &&& !(ckc_spec::v1text::is_lower_b(bytes@[e.end as int])
                || ckc_spec::v1text::is_digit_b(bytes@[e.end as int])
                || bytes@[e.end as int] == 0x2d)
        },
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r matches Some(field) ==> {
            &&& start < field.end <= bytes@.len()
            &&& field.value@ == bytes@.subrange(start as int, field.end as int)
            &&& ckc_spec::v1text::name_ok(field.value@)
        },
        expected@ matches Some(e) ==> r matches Some(field)
            && field.value@ == e.value && field.end == e.end,
{
    let mut pos = start;
    while pos < bytes.len() && is_name_b(bytes[pos])
        invariant
            *old(at) <= *at <= bytes@.len(),
            start <= pos <= bytes@.len(),
            forall|i: int| start <= i < pos ==> {
                ckc_spec::v1text::is_lower_b(bytes@[i])
                    || ckc_spec::v1text::is_digit_b(bytes@[i])
                    || bytes@[i] == 0x2d
            },
            expected@ matches Some(e) ==> {
                &&& start < e.end < bytes@.len()
                &&& ckc_spec::v1text::name_ok(e.value)
                &&& e.value == bytes@.subrange(start as int, e.end as int)
                &&& !(ckc_spec::v1text::is_lower_b(bytes@[e.end as int])
                    || ckc_spec::v1text::is_digit_b(bytes@[e.end as int])
                    || bytes@[e.end as int] == 0x2d)
                &&& pos <= e.end
            },
        decreases bytes.len() - pos,
    {
        proof {
            if let Some(e) = expected@ {
                if pos == e.end {
                    assert(false);
                }
            }
        }
        pos += 1;
    }
    proof {
        if let Some(e) = expected@ {
            if pos < e.end {
                reveal(ckc_spec::v1text::name_ok);
                reveal(ckc_spec::v1text::all_in);
                assert(bytes@[pos as int]
                    == e.value[pos as int - start as int]);
                assert(false);
            }
            assert(pos == e.end);
        }
    }
    if pos == start {
        raise_at(at, start, bytes.len());
        proof {
            if let Some(e) = expected@ {
                assert(false);
            }
        }
        return None;
    }
    let value = copy_range(bytes, start, pos);
    if !name_ok_exec(&value) {
        raise_at(at, start, bytes.len());
        proof {
            if let Some(e) = expected@ {
                assert(value@ == e.value);
                assert(false);
            }
        }
        return None;
    }
    proof {
        if let Some(e) = expected@ {
            assert(value@ == e.value);
        }
    }
    Some(ENameField { value, end: pos })
}

fn is_alnum_b(b: u8) -> (r: bool)
    ensures r == ckc_spec::v1text::is_alnum_b(b),
{
    is_lower_b(b) || (0x41 <= b && b <= 0x5a) || is_digit_b(b) || b == 0x5f
}

fn is_graphic_b(b: u8) -> (r: bool)
    ensures r == ckc_spec::v1text::is_graphic_b(b),
{
    b == 0x23 || b == 0x24 || b == 0x26 || b == 0x2a || b == 0x2b || b == 0x2d
        || b == 0x2e || b == 0x2f || b == 0x3a || b == 0x3c || b == 0x3d
        || b == 0x3e || b == 0x3f || b == 0x40 || b == 0x5c || b == 0x5e
        || b == 0x7e
}

pub open spec fn atom_boundary(bytes: Seq<u8>, end: int) -> bool {
    term_boundary(bytes, end)
        || 0 < end < bytes.len() && bytes[end] == 0x28
}

proof fn atom_boundary_stops_alnum(bytes: Seq<u8>, end: int)
    requires
        atom_boundary(bytes, end),
        end < bytes.len(),
    ensures !ckc_spec::v1text::is_alnum_b(bytes[end]),
{
    reveal(atom_boundary);
    if term_boundary(bytes, end) {
        boundary_stops_alnum(bytes, end);
    } else {
        reveal(ckc_spec::v1text::is_alnum_b);
        reveal(ckc_spec::v1text::is_lower_b);
        reveal(ckc_spec::v1text::is_digit_b);
    }
}

proof fn atom_boundary_stops_graphic_scan(bytes: Seq<u8>, end: int)
    requires
        atom_boundary(bytes, end),
        end < bytes.len(),
    ensures
        !ckc_spec::v1text::is_graphic_b(bytes[end])
            || bytes[end] == 0x2e && end + 1 < bytes.len() && bytes[end + 1] == 0x0a,
{
    reveal(atom_boundary);
    if term_boundary(bytes, end) {
        boundary_stops_graphic_scan(bytes, end);
    } else {
        reveal(ckc_spec::v1text::is_graphic_b);
    }
}

fn parse_alpha_atom(
    bytes: &[u8],
    start: usize,
    expected: Ghost<Option<GAtomExpected>>,
at: &mut usize,
) -> (r: Option<EParsedAtom>)
    requires
        *old(at) <= bytes@.len(),
        start < bytes@.len(),
        expected@ matches Some(e) ==> {
            &&& start < e.end <= bytes@.len()
            &&& ckc_spec::v1text::alpha_bare(e.name)
            &&& ckc_spec::v1text::atom_bytes(e.name)
                == bytes@.subrange(start as int, e.end as int)
            &&& atom_boundary(bytes@, e.end as int)
        },
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r matches Some(a) ==> parsed_atom_ok(bytes@, start, &a),
        expected@ matches Some(e) ==> r matches Some(a)
            && a.name@ == e.name && a.end == e.end,
{
    proof {
        if let Some(e) = expected@ {
            reveal(ckc_spec::v1text::alpha_bare);
            reveal(ckc_spec::v1text::atom_bare);
            reveal(ckc_spec::v1text::atom_bytes);
            assert(e.name == bytes@.subrange(start as int, e.end as int));
            assert(e.name.len() > 0);
            assert(bytes@[start as int] == e.name[0]);
            assert(ckc_spec::v1text::is_lower_b(bytes@[start as int]));
        }
    }
    if !is_lower_b(bytes[start]) {
        raise_at(at, start, bytes.len());
        return None;
    }
    let mut name = Vec::new();
    let mut pos = start;
    while pos < bytes.len() && is_alnum_b(bytes[pos])
        invariant
            *old(at) <= *at <= bytes@.len(),
            start <= pos <= bytes@.len(),
            name@ == bytes@.subrange(start as int, pos as int),
            name@.len() == pos - start,
            forall|i: int| 0 <= i < name@.len()
                ==> ckc_spec::v1text::is_alnum_b(name@[i]),
            ckc_spec::v1text::is_lower_b(bytes@[start as int]),
            expected@ matches Some(e) ==> {
                &&& start < e.end <= bytes@.len()
                &&& pos <= e.end
                &&& ckc_spec::v1text::alpha_bare(e.name)
                &&& e.name == bytes@.subrange(start as int, e.end as int)
                &&& atom_boundary(bytes@, e.end as int)
                &&& name@ == e.name.subrange(0, (pos - start) as int)
            },
        decreases bytes.len() - pos,
    {
        proof {
            if let Some(e) = expected@ {
                if pos == e.end {
                    atom_boundary_stops_alnum(bytes@, e.end as int);
                    assert(false);
                }
                assert(pos < e.end);
            }
        }
        let ghost old_pos = pos;
        name.push(bytes[pos]);
        proof {
            subrange_push(bytes@, start as int, old_pos as int);
            if let Some(e) = expected@ {
                let k = (old_pos - start) as int;
                assert(0 <= k < e.name.len());
                assert(bytes@[old_pos as int] == e.name[k]);
                assert_seqs_equal!(e.name.subrange(0, k + 1)
                    == e.name.subrange(0, k).push(e.name[k]));
                assert_seqs_equal!(name@ == e.name.subrange(0, k + 1));
            }
        }
        pos += 1;
    }
    proof {
        if let Some(e) = expected@ {
            if pos < e.end {
                reveal(ckc_spec::v1text::alpha_bare);
                reveal(ckc_spec::v1text::all_in);
                assert(ckc_spec::v1text::is_alnum_b(e.name[(pos - start) as int]));
                assert(bytes@[pos as int] == e.name[(pos - start) as int]);
                assert(false);
            }
            assert(pos == e.end);
            assert_seqs_equal!(name@ == e.name);
        }
        reveal(ckc_spec::v1text::all_in);
        reveal(ckc_spec::v1text::alpha_bare);
        reveal(ckc_spec::v1text::atom_bare);
        reveal(ckc_spec::v1text::atom_bytes);
    }
    Some(EParsedAtom { name, end: pos })
}

#[verifier::rlimit(5000)]
#[verifier::spinoff_prover]
fn parse_graphic_atom(
    bytes: &[u8],
    start: usize,
    expected: Ghost<Option<GAtomExpected>>,
at: &mut usize,
) -> (r: Option<EParsedAtom>)
    requires
        *old(at) <= bytes@.len(),
        start < bytes@.len(),
        expected@ matches Some(e) ==> {
            &&& start < e.end <= bytes@.len()
            &&& ckc_spec::v1text::graphic_bare(e.name)
            &&& ckc_spec::v1text::atom_bytes(e.name)
                == bytes@.subrange(start as int, e.end as int)
            &&& atom_boundary(bytes@, e.end as int)
        },
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r matches Some(a) ==> parsed_atom_ok(bytes@, start, &a),
        expected@ matches Some(e) ==> r matches Some(a)
            && a.name@ == e.name && a.end == e.end,
{
    proof {
        if let Some(e) = expected@ {
            reveal(ckc_spec::v1text::graphic_bare);
            reveal(ckc_spec::v1text::atom_bare);
            reveal(ckc_spec::v1text::atom_bytes);
            assert(e.name == bytes@.subrange(start as int, e.end as int));
            assert(e.name.len() > 0);
            assert(bytes@[start as int] == e.name[0]);
            assert(ckc_spec::v1text::is_graphic_b(bytes@[start as int]));
            if bytes@[start as int] == 0x2e
                && start + 1 < bytes@.len() && bytes@[start as int + 1] == 0x0a
            {
                reveal(ckc_spec::v1text::all_in);
                if e.name.len() == 1 {
                    assert(e.name == seq![0x2eu8]);
                    assert(false);
                } else {
                    assert(start + 1 < e.end);
                    assert(e.name[1] == bytes@[start as int + 1]);
                    assert(ckc_spec::v1text::is_graphic_b(e.name[1]));
                    reveal(ckc_spec::v1text::is_graphic_b);
                    assert(false);
                }
            }
        }
    }
    if !is_graphic_b(bytes[start])
        || (bytes[start] == 0x2e && start + 1 < bytes.len() && bytes[start + 1] == 0x0a)
    {
        if bytes[start] == 0x2e {
            raise_at(at, start + 1, bytes.len());
        } else {
            raise_at(at, start, bytes.len());
        }
        return None;
    }
    let mut name = Vec::new();
    let mut pos = start;
    while pos < bytes.len() && is_graphic_b(bytes[pos])
        && !(bytes[pos] == 0x2e && pos + 1 < bytes.len() && bytes[pos + 1] == 0x0a)
        invariant
            *old(at) <= *at <= bytes@.len(),
            start <= pos <= bytes@.len(),
            name@ == bytes@.subrange(start as int, pos as int),
            name@.len() == pos - start,
            forall|i: int| 0 <= i < name@.len()
                ==> ckc_spec::v1text::is_graphic_b(name@[i]),
            expected@ matches Some(e) ==> {
                &&& start < e.end <= bytes@.len()
                &&& pos <= e.end
                &&& ckc_spec::v1text::graphic_bare(e.name)
                &&& e.name == bytes@.subrange(start as int, e.end as int)
                &&& atom_boundary(bytes@, e.end as int)
                &&& name@ == e.name.subrange(0, (pos - start) as int)
            },
        decreases bytes.len() - pos,
    {
        proof {
            if let Some(e) = expected@ {
                if pos == e.end {
                    atom_boundary_stops_graphic_scan(bytes@, e.end as int);
                    assert(false);
                }
                assert(pos < e.end);
                if bytes@[pos as int] == 0x2e
                    && pos + 1 < bytes@.len() && bytes@[pos as int + 1] == 0x0a
                {
                    reveal(ckc_spec::v1text::graphic_bare);
                    reveal(ckc_spec::v1text::all_in);
                    if pos + 1 < e.end {
                        let k = (pos + 1 - start) as int;
                        assert(0 <= k < e.name.len());
                        assert(bytes@[pos as int + 1] == e.name[k]);
                        assert(ckc_spec::v1text::is_graphic_b(e.name[k]));
                        reveal(ckc_spec::v1text::is_graphic_b);
                        assert(false);
                    } else {
                        assert(pos + 1 == e.end);
                        reveal(atom_boundary);
                        reveal(term_boundary);
                        assert(false);
                    }
                }
            }
        }
        let ghost old_pos = pos;
        name.push(bytes[pos]);
        proof {
            subrange_push(bytes@, start as int, old_pos as int);
            if let Some(e) = expected@ {
                let k = (old_pos - start) as int;
                assert(0 <= k < e.name.len());
                assert(bytes@[old_pos as int] == e.name[k]);
                assert_seqs_equal!(e.name.subrange(0, k + 1)
                    == e.name.subrange(0, k).push(e.name[k]));
                assert_seqs_equal!(name@ == e.name.subrange(0, k + 1));
            }
        }
        pos += 1;
    }
    proof {
        if let Some(e) = expected@ {
            if pos < e.end {
                reveal(ckc_spec::v1text::graphic_bare);
                reveal(ckc_spec::v1text::all_in);
                assert(ckc_spec::v1text::is_graphic_b(e.name[(pos - start) as int]));
                assert(bytes@[pos as int] == e.name[(pos - start) as int]);
                if bytes@[pos as int] == 0x2e
                    && pos + 1 < bytes@.len() && bytes@[pos as int + 1] == 0x0a
                {
                    if pos + 1 < e.end {
                        let k = (pos + 1 - start) as int;
                        assert(0 <= k < e.name.len());
                        assert(bytes@[pos as int + 1] == e.name[k]);
                        assert(ckc_spec::v1text::is_graphic_b(e.name[k]));
                        reveal(ckc_spec::v1text::is_graphic_b);
                        assert(false);
                    } else {
                        assert(pos + 1 == e.end);
                        reveal(atom_boundary);
                        reveal(term_boundary);
                        assert(false);
                    }
                }
                assert(false);
            }
            assert(pos == e.end);
            assert_seqs_equal!(name@ == e.name);
            reveal(ckc_spec::v1text::graphic_bare);
            seq_eq_one(name@, 0x2e);
            assert(!(name@.len() == 1 && name@[0] == 0x2e));
            assert(!(name@.len() >= 2 && name@[0] == 0x2f && name@[1] == 0x2a));
        }
    }
    if name.len() == 1 && name[0] == 0x2e {
        raise_at(at, pos, bytes.len());
        return None;
    }
    if name.len() >= 2 && name[0] == 0x2f && name[1] == 0x2a {
        raise_at(at, start + 1, bytes.len());
        return None;
    }
    proof {
        reveal(ckc_spec::v1text::all_in);
        reveal(ckc_spec::v1text::graphic_bare);
        reveal(ckc_spec::v1text::atom_bare);
        reveal(ckc_spec::v1text::atom_bytes);
    }
    Some(EParsedAtom { name, end: pos })
}

fn parse_solo_atom(
    bytes: &[u8],
    start: usize,
    expected: Ghost<Option<GAtomExpected>>,
at: &mut usize,
) -> (r: Option<EParsedAtom>)
    requires
        *old(at) <= bytes@.len(),
        start < bytes@.len(),
        expected@ matches Some(e) ==> {
            &&& start < e.end <= bytes@.len()
            &&& ckc_spec::v1text::solo_bare(e.name)
            &&& ckc_spec::v1text::atom_bytes(e.name)
                == bytes@.subrange(start as int, e.end as int)
            &&& atom_boundary(bytes@, e.end as int)
        },
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r matches Some(a) ==> parsed_atom_ok(bytes@, start, &a),
        expected@ matches Some(e) ==> r matches Some(a)
            && a.name@ == e.name && a.end == e.end,
{
    proof {
        if let Some(e) = expected@ {
            reveal(ckc_spec::v1text::solo_bare);
            reveal(ckc_spec::v1text::atom_bare);
            reveal(ckc_spec::v1text::atom_bytes);
            assert(e.name == bytes@.subrange(start as int, e.end as int));
            seq_eq_one(e.name, 0x21);
            seq_eq_one(e.name, 0x3b);
            seq_eq_two(e.name, 0x7b, 0x7d);
        }
    }
    if bytes[start] == 0x21 || bytes[start] == 0x3b {
        let mut name = Vec::new();
        name.push(bytes[start]);
        proof {
            reveal(ckc_spec::v1text::solo_bare);
            reveal(ckc_spec::v1text::atom_bare);
            reveal(ckc_spec::v1text::atom_bytes);
            assert_seqs_equal!(name@ == bytes@.subrange(start as int, (start + 1) as int));
            if let Some(e) = expected@ {
                reveal(ckc_spec::v1text::solo_bare);
                seq_eq_one(e.name, 0x21);
                seq_eq_one(e.name, 0x3b);
                seq_eq_two(e.name, 0x7b, 0x7d);
                assert(e.name == name@);
                assert(e.end == start + 1);
            }
        }
        return Some(EParsedAtom { name, end: start + 1 });
    }
    if bytes[start] == 0x7b && start + 1 < bytes.len() && bytes[start + 1] == 0x7d {
        let mut name = Vec::new();
        name.push(0x7b);
        name.push(0x7d);
        proof {
            assert(name@ == seq![0x7bu8, 0x7du8]);
            assert_seqs_equal!(bytes@.subrange(start as int, (start + 2) as int)
                == seq![0x7bu8, 0x7du8]);
            reveal(ckc_spec::v1text::solo_bare);
            reveal(ckc_spec::v1text::atom_bare);
            reveal(ckc_spec::v1text::atom_bytes);
            if let Some(e) = expected@ {
                reveal(ckc_spec::v1text::solo_bare);
                seq_eq_one(e.name, 0x21);
                seq_eq_one(e.name, 0x3b);
                seq_eq_two(e.name, 0x7b, 0x7d);
                assert(e.name == name@);
                assert(e.end == start + 2);
            }
        }
        return Some(EParsedAtom { name, end: start + 2 });
    }
    proof {
        if let Some(e) = expected@ {
            reveal(ckc_spec::v1text::solo_bare);
            seq_eq_one(e.name, 0x21);
            seq_eq_one(e.name, 0x3b);
            seq_eq_two(e.name, 0x7b, 0x7d);
            assert(false);
        }
    }
    if bytes[start] == 0x7b {
        raise_at(at, start + 1, bytes.len());
    } else {
        raise_at(at, start, bytes.len());
    }
    None
}

proof fn esc_all_push(s: Seq<u8>, b: u8)
    ensures
        ckc_spec::v1text::esc_all(s.push(b))
            == ckc_spec::v1text::esc_all(s) + ckc_spec::v1text::esc_byte(b),
    decreases s.len(),
{
    if s.len() == 0 {
        assert(s.push(b)[0] == b);
        assert_seqs_equal!(s.push(b).drop_first() == Seq::<u8>::empty());
        reveal_with_fuel(ckc_spec::v1text::esc_all, 2);
    } else {
        esc_all_push(s.drop_first(), b);
        assert(s.push(b)[0] == s[0]);
        assert_seqs_equal!(s.push(b).drop_first() == s.drop_first().push(b));
        reveal_with_fuel(ckc_spec::v1text::esc_all, 2);
        assert_seqs_equal!(
            ckc_spec::v1text::esc_byte(s[0])
                + (ckc_spec::v1text::esc_all(s.drop_first()) + ckc_spec::v1text::esc_byte(b))
            == (ckc_spec::v1text::esc_byte(s[0]) + ckc_spec::v1text::esc_all(s.drop_first()))
                + ckc_spec::v1text::esc_byte(b)
        );
    }
}

fn all_alnum_b(s: &[u8]) -> (r: bool)
    ensures r == ckc_spec::v1text::all_in(s@, |b: u8| ckc_spec::v1text::is_alnum_b(b)),
{
    let mut i = 0usize;
    while i < s.len()
        invariant
            i <= s@.len(),
            forall|j: int| 0 <= j < i ==> ckc_spec::v1text::is_alnum_b(s@[j]),
        decreases s.len() - i,
    {
        if !is_alnum_b(s[i]) {
            return false;
        }
        i += 1;
    }
    proof { reveal(ckc_spec::v1text::all_in); }
    true
}

fn all_graphic_b(s: &[u8]) -> (r: bool)
    ensures r == ckc_spec::v1text::all_in(s@, |b: u8| ckc_spec::v1text::is_graphic_b(b)),
{
    let mut i = 0usize;
    while i < s.len()
        invariant
            i <= s@.len(),
            forall|j: int| 0 <= j < i ==> ckc_spec::v1text::is_graphic_b(s@[j]),
        decreases s.len() - i,
    {
        if !is_graphic_b(s[i]) {
            return false;
        }
        i += 1;
    }
    proof { reveal(ckc_spec::v1text::all_in); }
    true
}

proof fn seq_eq_one(s: Seq<u8>, x: u8)
    ensures s == seq![x] <==> s.len() == 1 && s[0] == x,
{
    if s.len() == 1 && s[0] == x {
        assert_seqs_equal!(s == seq![x]);
    }
}

proof fn seq_eq_two(s: Seq<u8>, x: u8, y: u8)
    ensures s == seq![x, y] <==> s.len() == 2 && s[0] == x && s[1] == y,
{
    if s.len() == 2 && s[0] == x && s[1] == y {
        assert_seqs_equal!(s == seq![x, y]);
    }
}

fn atom_bare_exec(name: &[u8]) -> (r: bool)
    ensures r == ckc_spec::v1text::atom_bare(name@),
{
    let alpha = name.len() > 0 && is_lower_b(name[0]) && all_alnum_b(name);
    let graphic = name.len() > 0 && all_graphic_b(name)
        && !(name.len() == 1 && name[0] == 0x2e)
        && !(name.len() >= 2 && name[0] == 0x2f && name[1] == 0x2a);
    let solo = (name.len() == 1 && (name[0] == 0x3b || name[0] == 0x21))
        || (name.len() == 2 && name[0] == 0x7b && name[1] == 0x7d);
    proof {
        seq_eq_one(name@, 0x2e);
        seq_eq_one(name@, 0x3b);
        seq_eq_one(name@, 0x21);
        seq_eq_two(name@, 0x7b, 0x7d);
        reveal(ckc_spec::v1text::alpha_bare);
        reveal(ckc_spec::v1text::graphic_bare);
        reveal(ckc_spec::v1text::solo_bare);
        reveal(ckc_spec::v1text::atom_bare);
        assert(alpha == ckc_spec::v1text::alpha_bare(name@));
        assert(graphic == ckc_spec::v1text::graphic_bare(name@));
        assert(solo == ckc_spec::v1text::solo_bare(name@));
    }
    alpha || graphic || solo
}

fn upper_hex_value(b: u8, expected: Ghost<Option<u8>>) -> (r: Option<u8>)
    requires expected@ matches Some(d) ==> d < 16
        && ckc_spec::v1text::uhex_digit(d as int) == b,
    ensures
        r matches Some(d) ==> d < 16 && ckc_spec::v1text::uhex_digit(d as int) == b,
        expected@ matches Some(d) ==> r == Some(d),
{
    let out = if 0x30 <= b && b <= 0x39 {
        Some(b - 0x30)
    } else if 0x41 <= b && b <= 0x46 {
        Some(b - 0x41 + 10)
    } else {
        None
    };
    proof { reveal(ckc_spec::v1text::uhex_digit); }
    out
}

fn named_escape(b: u8, expected: Ghost<Option<u8>>) -> (r: Option<u8>)
    requires expected@ matches Some(v) ==> named_escape_value(v)
        && named_escape_code(v) == b,
    ensures
        r matches Some(v) ==> ckc_spec::v1text::esc_byte(v) == seq![0x5cu8, b],
        expected@ matches Some(v) ==> r == Some(v),
{
    let out = if b == 0x5c { Some(0x5c) }
        else if b == 0x27 { Some(0x27) }
        else if b == 0x61 { Some(0x07) }
        else if b == 0x62 { Some(0x08) }
        else if b == 0x74 { Some(0x09) }
        else if b == 0x6e { Some(0x0a) }
        else if b == 0x76 { Some(0x0b) }
        else if b == 0x66 { Some(0x0c) }
        else if b == 0x72 { Some(0x0d) }
        else { None };
    proof {
        reveal_strlit("\\\\");
        reveal_strlit("\\'");
        reveal_strlit("\\a");
        reveal_strlit("\\b");
        reveal_strlit("\\t");
        reveal_strlit("\\n");
        reveal_strlit("\\v");
        reveal_strlit("\\f");
        reveal_strlit("\\r");
        reveal(ckc_spec::v1text::esc_byte);
        reveal(ckc_spec::v1text::ascii);
        reveal(named_escape_value);
        reveal(named_escape_code);
    }
    out
}

pub open spec fn named_escape_value(b: u8) -> bool {
    b == 0x5c || b == 0x27 || 0x07 <= b && b <= 0x0d
}

pub open spec fn unicode_escape_value(b: u8) -> bool {
    (b < 0x20 && !named_escape_value(b)) || b == 0x7f
}

pub open spec fn named_escape_code(b: u8) -> u8 {
    if b == 0x5c { 0x5c }
    else if b == 0x27 { 0x27 }
    else if b == 0x07 { 0x61 }
    else if b == 0x08 { 0x62 }
    else if b == 0x09 { 0x74 }
    else if b == 0x0a { 0x6e }
    else if b == 0x0b { 0x76 }
    else if b == 0x0c { 0x66 }
    else { 0x72 }
}

proof fn named_escape_code_injective(a: u8, b: u8)
    requires
        named_escape_value(a),
        named_escape_value(b),
        named_escape_code(a) == named_escape_code(b),
    ensures a == b,
{
    reveal(named_escape_value);
    reveal(named_escape_code);
}

proof fn uhex_digit_injective(a: int, b: int)
    requires
        0 <= a < 16,
        0 <= b < 16,
        ckc_spec::v1text::uhex_digit(a) == ckc_spec::v1text::uhex_digit(b),
    ensures a == b,
{
    reveal(ckc_spec::v1text::uhex_digit);
    reveal(ckc_spec::v1text::digit_byte);
}

proof fn uhex4_u8_injective(a: u8, b: u8)
    requires ckc_spec::v1text::uhex4(a as int) == ckc_spec::v1text::uhex4(b as int),
    ensures a == b,
{
    reveal(ckc_spec::v1text::uhex4);
    assert(0 <= a as int / 16 % 16 < 16);
    assert(0 <= b as int / 16 % 16 < 16);
    assert(0 <= a as int % 16 < 16);
    assert(0 <= b as int % 16 < 16);
    uhex_digit_injective(a as int / 16 % 16, b as int / 16 % 16);
    uhex_digit_injective(a as int % 16, b as int % 16);
    assert(a as int / 16 < 16);
    assert(b as int / 16 < 16);
    assert(a as int / 16 % 16 == a as int / 16);
    assert(b as int / 16 % 16 == b as int / 16);
    lemma_fundamental_div_mod_converse(
        a as int,
        16,
        a as int / 16,
        a as int % 16,
    );
    lemma_fundamental_div_mod_converse(
        b as int,
        16,
        b as int / 16,
        b as int % 16,
    );
}

proof fn esc_byte_shape(b: u8)
    ensures
        ckc_spec::v1text::esc_byte(b) == if named_escape_value(b) {
            seq![0x5cu8, named_escape_code(b)]
        } else if unicode_escape_value(b) {
            seq![0x5cu8, 0x75u8] + ckc_spec::v1text::uhex4(b as int)
        } else {
            seq![b]
        },
{
    reveal_strlit("\\\\");
    reveal_strlit("\\'");
    reveal_strlit("\\a");
    reveal_strlit("\\b");
    reveal_strlit("\\t");
    reveal_strlit("\\n");
    reveal_strlit("\\v");
    reveal_strlit("\\f");
    reveal_strlit("\\r");
    reveal_strlit("\\u");
    reveal(ckc_spec::v1text::ascii);
    reveal(ckc_spec::v1text::esc_byte);
    reveal(named_escape_value);
    reveal(unicode_escape_value);
    reveal(named_escape_code);
}

proof fn esc_byte_len(b: u8)
    ensures ckc_spec::v1text::esc_byte(b).len() == if named_escape_value(b) {
        2nat
    } else if unicode_escape_value(b) {
        6nat
    } else {
        1nat
    },
{
    esc_byte_shape(b);
    reveal(ckc_spec::v1text::uhex4);
}

proof fn named_escape_code_not_u(b: u8)
    requires named_escape_value(b),
    ensures named_escape_code(b) != 0x75,
{
    reveal(named_escape_value);
    reveal(named_escape_code);
}

proof fn unicode_escape_at(
    bytes: Seq<u8>,
    start: usize,
    end: usize,
    value: u8,
)
    requires
        start < end <= bytes.len(),
        unicode_escape_value(value),
        ckc_spec::v1text::esc_byte(value)
            == bytes.subrange(start as int, end as int),
    ensures
        end == start + 6,
        bytes[start as int] == 0x5c,
        bytes[start as int + 1] == 0x75,
        bytes[start as int + 2] == 0x30,
        bytes[start as int + 3] == 0x30,
        bytes[start as int + 4]
            == ckc_spec::v1text::uhex_digit((value / 16) as int),
        bytes[start as int + 5]
            == ckc_spec::v1text::uhex_digit((value % 16) as int),
{
    esc_byte_shape(value);
    esc_byte_len(value);
    reveal(ckc_spec::v1text::uhex4);
    reveal(ckc_spec::v1text::uhex_digit);
    reveal(ckc_spec::v1text::digit_byte);
    assert(bytes.subrange(start as int, end as int).len() == end - start);
    assert(end - start == 6);
    assert(value as int / 4096 % 16 == 0);
    assert(value as int / 256 % 16 == 0);
    assert(value as int / 16 % 16 == (value / 16) as int);
    assert(value as int % 16 == (value % 16) as int);
    assert(bytes.subrange(start as int, end as int)[0] == bytes[start as int]);
    assert(bytes.subrange(start as int, end as int)[1] == bytes[start as int + 1]);
    assert(bytes.subrange(start as int, end as int)[2] == bytes[start as int + 2]);
    assert(bytes.subrange(start as int, end as int)[3] == bytes[start as int + 3]);
    assert(bytes.subrange(start as int, end as int)[4] == bytes[start as int + 4]);
    assert(bytes.subrange(start as int, end as int)[5] == bytes[start as int + 5]);
}

proof fn esc_byte_first_not_quote(b: u8)
    ensures
        ckc_spec::v1text::esc_byte(b).len() > 0,
        ckc_spec::v1text::esc_byte(b)[0] != 0x27,
{
    esc_byte_shape(b);
    esc_byte_len(b);
    if !named_escape_value(b) && !unicode_escape_value(b) {
        reveal(named_escape_value);
    }
}

proof fn esc_byte_injective(a: u8, b: u8)
    requires ckc_spec::v1text::esc_byte(a) == ckc_spec::v1text::esc_byte(b),
    ensures a == b,
{
    esc_byte_shape(a);
    esc_byte_shape(b);
    esc_byte_len(a);
    esc_byte_len(b);
    if named_escape_value(a) {
        assert(ckc_spec::v1text::esc_byte(a).len() == 2);
        assert(ckc_spec::v1text::esc_byte(b).len() == 2);
        assert(named_escape_value(b));
        assert(named_escape_code(a) == named_escape_code(b));
        named_escape_code_injective(a, b);
    } else if unicode_escape_value(a) {
        assert(ckc_spec::v1text::esc_byte(a).len() == 6);
        assert(ckc_spec::v1text::esc_byte(b).len() == 6);
        assert(unicode_escape_value(b));
        let ua = ckc_spec::v1text::uhex4(a as int);
        let ub = ckc_spec::v1text::uhex4(b as int);
        assert(seq![0x5cu8, 0x75u8] + ua == seq![0x5cu8, 0x75u8] + ub);
        assert(ua.len() == 4 && ub.len() == 4) by {
            reveal(ckc_spec::v1text::uhex4);
        }
        assert forall|i: int| #![auto] 0 <= i < ua.len() ==> ua[i] == ub[i] by {
            if 0 <= i < ua.len() {
                assert((seq![0x5cu8, 0x75u8] + ua)[i + 2] == ua[i]);
                assert((seq![0x5cu8, 0x75u8] + ub)[i + 2] == ub[i]);
            }
        }
        assert_seqs_equal!(ua == ub);
        uhex4_u8_injective(a, b);
    } else {
        assert(ckc_spec::v1text::esc_byte(a).len() == 1);
        assert(ckc_spec::v1text::esc_byte(b).len() == 1);
        assert(!named_escape_value(b));
        assert(!unicode_escape_value(b));
        assert(seq![a] == seq![b]);
    }
}

proof fn escapes_at_same_start(
    bytes: Seq<u8>,
    start: usize,
    a: u8,
    a_end: usize,
    b: u8,
    b_end: usize,
)
    requires
        start < a_end <= bytes.len(),
        start < b_end <= bytes.len(),
        ckc_spec::v1text::esc_byte(a)
            == bytes.subrange(start as int, a_end as int),
        ckc_spec::v1text::esc_byte(b)
            == bytes.subrange(start as int, b_end as int),
    ensures
        a == b,
        a_end == b_end,
{
    esc_byte_shape(a);
    esc_byte_shape(b);
    esc_byte_len(a);
    esc_byte_len(b);
    assert(bytes.subrange(start as int, a_end as int).len() == a_end - start);
    assert(bytes.subrange(start as int, b_end as int).len() == b_end - start);
    assert(bytes[start as int] == ckc_spec::v1text::esc_byte(a)[0]);
    assert(bytes[start as int] == ckc_spec::v1text::esc_byte(b)[0]);
    if named_escape_value(a) {
        named_escape_code_not_u(a);
        assert(a_end >= start + 2);
        assert(bytes[start as int] == 0x5c);
        assert(bytes[start as int + 1] == named_escape_code(a));
        if named_escape_value(b) {
            assert(bytes[start as int + 1] == named_escape_code(b));
            named_escape_code_injective(a, b);
        } else if unicode_escape_value(b) {
            assert(bytes[start as int + 1] == 0x75);
            assert(false);
        } else {
            assert(bytes[start as int] == b);
            reveal(named_escape_value);
            assert(false);
        }
    } else if unicode_escape_value(a) {
        assert(a_end >= start + 2);
        assert(ckc_spec::v1text::esc_byte(a)[0] == 0x5c);
        assert(ckc_spec::v1text::esc_byte(a)[1] == 0x75);
        assert(bytes.subrange(start as int, a_end as int)[0] == bytes[start as int]);
        assert(bytes.subrange(start as int, a_end as int)[1] == bytes[start as int + 1]);
        assert(bytes[start as int] == 0x5c);
        assert(bytes[start as int + 1] == 0x75);
        if named_escape_value(b) {
            named_escape_code_not_u(b);
            assert(bytes[start as int + 1] == named_escape_code(b));
            assert(false);
        } else if unicode_escape_value(b) {
            assert(a_end - start == 6);
            assert(b_end - start == 6);
            assert(a_end == b_end);
            assert(ckc_spec::v1text::esc_byte(a) == ckc_spec::v1text::esc_byte(b));
            esc_byte_injective(a, b);
        } else {
            assert(bytes[start as int] == b);
            reveal(named_escape_value);
            assert(false);
        }
    } else {
        assert(bytes[start as int] == a);
        if named_escape_value(b) || unicode_escape_value(b) {
            assert(bytes[start as int] == 0x5c);
            reveal(named_escape_value);
            reveal(unicode_escape_value);
            assert(false);
        } else {
            assert(bytes[start as int] == b);
        }
    }
    assert(a == b);
    assert(ckc_spec::v1text::esc_byte(a).len()
        == ckc_spec::v1text::esc_byte(b).len());
    assert(a_end - start == b_end - start);
}

pub struct EParsedByte {
    pub value: u8,
    pub end: usize,
}

pub ghost struct GEscapeExpected {
    pub value: u8,
    pub end: usize,
}

pub open spec fn parsed_escape_ok(bytes: Seq<u8>, start: usize, e: &EParsedByte) -> bool {
    &&& start < e.end <= bytes.len()
    &&& ckc_spec::v1text::esc_byte(e.value)
        == bytes.subrange(start as int, e.end as int)
}

proof fn parsed_escape_matches_expected(
    bytes: Seq<u8>,
    start: usize,
    out: &EParsedByte,
    expected: GEscapeExpected,
)
    requires
        parsed_escape_ok(bytes, start, out),
        start < expected.end <= bytes.len(),
        ckc_spec::v1text::esc_byte(expected.value)
            == bytes.subrange(start as int, expected.end as int),
    ensures
        out.value == expected.value,
        out.end == expected.end,
{
    reveal(parsed_escape_ok);
    escapes_at_same_start(
        bytes,
        start,
        out.value,
        out.end,
        expected.value,
        expected.end,
    );
}

#[verifier::rlimit(300)]
fn parse_escaped_byte(
    bytes: &[u8],
    start: usize,
    expected: Ghost<Option<GEscapeExpected>>,
at: &mut usize,
) -> (r: Option<EParsedByte>)
    requires
        *old(at) <= bytes@.len(),
        start < bytes@.len(),
        expected@ matches Some(e) ==> {
            &&& start < e.end <= bytes@.len()
            &&& ckc_spec::v1text::esc_byte(e.value)
                == bytes@.subrange(start as int, e.end as int)
        },
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r matches Some(e) ==> parsed_escape_ok(bytes@, start, &e),
        expected@ matches Some(e) ==> r matches Some(out)
            && out.value == e.value && out.end == e.end,
{
    let b = bytes[start];
    let ghost expected_named = match expected@ {
        Some(e) => if named_escape_value(e.value) { Some(e.value) } else { None },
        None => None,
    };
    let ghost expected_unicode = match expected@ {
        Some(e) => if unicode_escape_value(e.value) { Some(e.value) } else { None },
        None => None,
    };
    proof {
        if let Some(e) = expected@ {
            esc_byte_shape(e.value);
            esc_byte_len(e.value);
            assert(bytes@.subrange(start as int, e.end as int).len() == e.end - start);
            assert(e.end - start == ckc_spec::v1text::esc_byte(e.value).len());
            assert(bytes@[start as int] == ckc_spec::v1text::esc_byte(e.value)[0]);
            if named_escape_value(e.value) {
                assert(b == 0x5c);
            } else if unicode_escape_value(e.value) {
                assert(b == 0x5c);
            } else {
                assert(b == e.value);
                reveal(named_escape_value);
                reveal(unicode_escape_value);
                assert(!(b < 0x20 || b == 0x27 || b == 0x7f));
            }
        }
    }
    if b != 0x5c {
        if b < 0x20 || b == 0x27 || b == 0x7f {
            raise_at(at, start, bytes.len());
            return None;
        }
        let out = EParsedByte { value: b, end: start + 1 };
        proof {
            reveal(ckc_spec::v1text::esc_byte);
            assert_seqs_equal!(bytes@.subrange(start as int, (start + 1) as int) == seq![b]);
            assert(parsed_escape_ok(bytes@, start, &out));
            if let Some(e) = expected@ {
                parsed_escape_matches_expected(bytes@, start, &out, e);
            }
        }
        return Some(out);
    }
    if bytes.len() - start < 2 {
        raise_at(at, bytes.len(), bytes.len());
        return None;
    }
    let code = bytes[start + 1];
    proof {
        if let Some(e) = expected@ {
            if named_escape_value(e.value) {
                assert(e.end - start == 2);
                assert(bytes@[start as int + 1] == named_escape_code(e.value));
                assert(code == named_escape_code(e.value));
            } else {
                assert(unicode_escape_value(e.value));
                unicode_escape_at(bytes@, start, e.end, e.value);
                assert(e.end - start == 6);
                assert(code == 0x75);
            }
        }
    }
    if let Some(value) = named_escape(code, Ghost(expected_named)) {
        let out = EParsedByte { value, end: start + 2 };
        proof {
            assert_seqs_equal!(bytes@.subrange(start as int, (start + 2) as int)
                == seq![0x5cu8, code]);
            assert(parsed_escape_ok(bytes@, start, &out));
            if let Some(e) = expected@ {
                parsed_escape_matches_expected(bytes@, start, &out, e);
            }
        }
        return Some(out);
    }
    proof {
        if let Some(e) = expected@ {
            assert(unicode_escape_value(e.value));
            unicode_escape_at(bytes@, start, e.end, e.value);
            assert(e.end == start + 6);
            assert(code == 0x75);
            assert(bytes@[start as int + 2] == 0x30);
            assert(bytes@[start as int + 3] == 0x30);
        }
    }
    if code != 0x75 {
        raise_at(at, start + 1, bytes.len());
        return None;
    }
    if bytes.len() - start < 3 {
        raise_at(at, bytes.len(), bytes.len());
        return None;
    }
    if bytes[start + 2] != 0x30 {
        raise_at(at, start + 2, bytes.len());
        return None;
    }
    if bytes.len() - start < 4 {
        raise_at(at, bytes.len(), bytes.len());
        return None;
    }
    if bytes[start + 3] != 0x30 {
        raise_at(at, start + 3, bytes.len());
        return None;
    }
    if bytes.len() - start < 6 {
        raise_at(at, bytes.len(), bytes.len());
        return None;
    }
    let ghost expected_d2 = match expected_unicode {
        Some(v) => Some(v / 16),
        None => None,
    };
    let ghost expected_d3 = match expected_unicode {
        Some(v) => Some(v % 16),
        None => None,
    };
    proof {
        if let Some(v) = expected_unicode {
            match expected@ {
                Some(e) => {
                    assert(e.value == v);
                    unicode_escape_at(bytes@, start, e.end, v);
                },
                None => assert(false),
            }
            assert(v / 16 < 16);
            assert(ckc_spec::v1text::uhex_digit((v / 16) as int)
                == bytes@[start as int + 4]);
            assert(v % 16 < 16);
            assert(ckc_spec::v1text::uhex_digit((v % 16) as int)
                == bytes@[start as int + 5]);
        }
    }
    let d2 = match upper_hex_value(bytes[start + 4], Ghost(expected_d2)) {
        Some(d) => d,
        None => {
            raise_at(at, start + 4, bytes.len());
            return None;
        },
    };
    if d2 != 0 && d2 != 1 && d2 != 7 {
        raise_at(at, start + 4, bytes.len());
        return None;
    }
    let d3 = match upper_hex_value(bytes[start + 5], Ghost(expected_d3)) {
        Some(d) => d,
        None => {
            raise_at(at, start + 5, bytes.len());
            return None;
        },
    };
    let value = d2 * 16 + d3;
    proof {
        if let Some(e) = expected@ {
            assert(expected_unicode == Some(e.value));
            assert(d2 == e.value / 16);
            assert(d3 == e.value % 16);
            lemma_fundamental_div_mod_converse(
                e.value as int,
                16,
                (e.value / 16) as int,
                (e.value % 16) as int,
            );
            assert(value == e.value);
            reveal(unicode_escape_value);
            reveal(named_escape_value);
            assert((value < 0x20 && !(0x07 <= value && value <= 0x0d))
                || value == 0x7f);
        }
    }
    if !((value < 0x20 && !(0x07 <= value && value <= 0x0d)) || value == 0x7f) {
        raise_at(at, start + 5, bytes.len());
        return None;
    }
    proof {
        reveal_strlit("\\u");
        reveal(ckc_spec::v1text::esc_byte);
        reveal(ckc_spec::v1text::ascii);
        reveal(ckc_spec::v1text::uhex4);
        reveal(ckc_spec::v1text::uhex_digit);
        assert(value < 256);
        assert(value as int / 4096 % 16 == 0);
        assert(value as int / 256 % 16 == 0);
        assert(value as int / 16 % 16 == d2 as int);
        assert(value as int % 16 == d3 as int);
        assert(ckc_spec::v1text::ascii("\\u"@) == seq![0x5cu8, 0x75u8]);
        assert(ckc_spec::v1text::uhex4(value as int)
            == seq![0x30u8, 0x30u8, bytes@[start as int + 4], bytes@[start as int + 5]]);
        assert(ckc_spec::v1text::esc_byte(value)
            == seq![0x5cu8, 0x75u8, 0x30u8, 0x30u8,
                bytes@[start as int + 4], bytes@[start as int + 5]]);
        assert_seqs_equal!(bytes@.subrange(start as int, (start + 6) as int)
            == seq![0x5cu8, 0x75u8, 0x30u8, 0x30u8, bytes@[start as int + 4], bytes@[start as int + 5]]);
    }
    let out = EParsedByte { value, end: start + 6 };
    proof {
        assert(parsed_escape_ok(bytes@, start, &out));
        if let Some(e) = expected@ {
            parsed_escape_matches_expected(bytes@, start, &out, e);
        }
    }
    Some(out)
}

proof fn suffix_after_prefix(
    bytes: Seq<u8>,
    start: int,
    end: int,
    prefix: Seq<u8>,
    suffix: Seq<u8>,
)
    requires
        0 <= start <= end <= bytes.len(),
        bytes.subrange(start, end) == prefix + suffix,
    ensures
        start + prefix.len() <= end,
        bytes.subrange(start + prefix.len(), end) == suffix,
{
    let mid = start + prefix.len();
    assert(bytes.subrange(start, end).len() == end - start);
    assert((prefix + suffix).len() == prefix.len() + suffix.len());
    assert(end - start == prefix.len() + suffix.len());
    assert(mid <= end);
    let out = bytes.subrange(mid, end);
    assert(out.len() == suffix.len());
    assert forall|i: int| #![auto] 0 <= i < suffix.len() ==> out[i] == suffix[i] by {
        if 0 <= i < suffix.len() {
            assert(out[i] == bytes[mid + i]);
            assert(bytes.subrange(start, end)[prefix.len() + i] == bytes[mid + i]);
            assert((prefix + suffix)[prefix.len() + i] == suffix[i]);
        }
    }
    assert_seqs_equal!(out == suffix);
}

proof fn prefix_before_suffix(
    bytes: Seq<u8>,
    start: int,
    end: int,
    prefix: Seq<u8>,
    suffix: Seq<u8>,
)
    requires
        0 <= start <= end <= bytes.len(),
        bytes.subrange(start, end) == prefix + suffix,
    ensures
        start + prefix.len() <= end,
        bytes.subrange(start, start + prefix.len()) == prefix,
{
    let out = bytes.subrange(start, start + prefix.len());
    assert(bytes.subrange(start, end).len() == end - start);
    assert((prefix + suffix).len() == prefix.len() + suffix.len());
    assert(end - start == prefix.len() + suffix.len());
    assert(start + prefix.len() <= end);
    assert(out.len() == prefix.len());
    assert forall|i: int| #![auto] 0 <= i < prefix.len() ==> out[i] == prefix[i] by {
        if 0 <= i < prefix.len() {
            assert(out[i] == bytes[start + i]);
            assert(bytes.subrange(start, end)[i] == bytes[start + i]);
            assert((prefix + suffix)[i] == prefix[i]);
        }
    }
    assert_seqs_equal!(out == prefix);
}

proof fn segment_of_three(
    bytes: Seq<u8>,
    start: int,
    end: int,
    before: Seq<u8>,
    part: Seq<u8>,
    after: Seq<u8>,
)
    requires
        0 <= start <= end <= bytes.len(),
        bytes.subrange(start, end) == before + part + after,
    ensures
        start + before.len() + part.len() <= end,
        bytes.subrange(
            start + before.len(),
            start + before.len() + part.len(),
        ) == part,
{
    assert_seqs_equal!(before + part + after == before + (part + after));
    suffix_after_prefix(bytes, start, end, before, part + after);
    prefix_before_suffix(
        bytes,
        start + before.len(),
        end,
        part,
        after,
    );
}

proof fn range_concat(
    bytes: Seq<u8>,
    start: int,
    mid: int,
    end: int,
)
    requires 0 <= start <= mid <= end <= bytes.len(),
    ensures bytes.subrange(start, end)
        == bytes.subrange(start, mid) + bytes.subrange(mid, end),
{
    assert_seqs_equal!(bytes.subrange(start, end)
        == bytes.subrange(start, mid) + bytes.subrange(mid, end));
}

proof fn range_between_bytes(
    bytes: Seq<u8>,
    start: int,
    end: int,
    left: u8,
    middle: Seq<u8>,
    right: u8,
)
    requires
        0 <= start < end <= bytes.len(),
        bytes.subrange(start, end) == seq![left] + middle + seq![right],
    ensures
        end == start + middle.len() + 2,
        bytes[start] == left,
        bytes[end - 1] == right,
        bytes.subrange(start + 1, end - 1) == middle,
{
    let span = bytes.subrange(start, end);
    let rendered = seq![left] + middle + seq![right];
    assert(span == rendered);
    assert(span.len() == end - start);
    assert(rendered.len() == middle.len() + 2);
    assert(span[0] == bytes[start]);
    assert(rendered[0] == left);
    assert(span[span.len() - 1] == bytes[end - 1]);
    assert(rendered[rendered.len() - 1] == right);
    let inner = bytes.subrange(start + 1, end - 1);
    assert(inner.len() == middle.len());
    assert forall|i: int| #![auto] 0 <= i < middle.len() ==> inner[i] == middle[i] by {
        if 0 <= i < middle.len() {
            assert(inner[i] == bytes[start + 1 + i]);
            assert(span[1 + i] == bytes[start + 1 + i]);
            assert(rendered[1 + i] == middle[i]);
        }
    }
    assert_seqs_equal!(inner == middle);
}

fn parse_quoted_atom(
    bytes: &[u8],
    start: usize,
    expected: Ghost<Option<GAtomExpected>>,
at: &mut usize,
) -> (r: Option<EParsedAtom>)
    requires
        *old(at) <= bytes@.len(),
        start < bytes@.len(),
        expected@ matches Some(e) ==> {
            &&& start < e.end <= bytes@.len()
            &&& !ckc_spec::v1text::atom_bare(e.name)
            &&& ckc_spec::v1text::atom_bytes(e.name)
                == bytes@.subrange(start as int, e.end as int)
            &&& atom_boundary(bytes@, e.end as int)
        },
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r matches Some(a) ==> parsed_atom_ok(bytes@, start, &a),
        expected@ matches Some(e) ==> r matches Some(a)
            && a.name@ == e.name && a.end == e.end,
{
    proof {
        if let Some(e) = expected@ {
            reveal(ckc_spec::v1text::atom_bytes);
            assert(bytes@.subrange(start as int, e.end as int)
                == seq![0x27u8] + ckc_spec::v1text::esc_all(e.name) + seq![0x27u8]);
            range_between_bytes(
                bytes@,
                start as int,
                e.end as int,
                0x27,
                ckc_spec::v1text::esc_all(e.name),
                0x27,
            );
            assert(e.end >= start + 2);
        }
    }
    if bytes[start] != 0x27 {
        raise_at(at, start, bytes.len());
        return None;
    }
    let mut name = Vec::new();
    let mut pos = start + 1;
    proof {
        if let Some(e) = expected@ {
            assert(name@ == Seq::<u8>::empty());
            assert_seqs_equal!(e.name.subrange(0, 0) == Seq::<u8>::empty());
            assert_seqs_equal!(e.name.subrange(0, e.name.len() as int) == e.name);
            assert(pos == start + 1);
        }
    }
    while pos < bytes.len() && bytes[pos] != 0x27
        invariant
            *old(at) <= *at <= bytes@.len(),
            start < pos <= bytes@.len(),
            ckc_spec::v1text::esc_all(name@)
                == bytes@.subrange(start as int + 1, pos as int),
            expected@ matches Some(e) ==> {
                &&& start < e.end <= bytes@.len()
                &&& bytes@[e.end as int - 1] == 0x27
                &&& name@.len() <= e.name.len()
                &&& name@ == e.name.subrange(0, name@.len() as int)
                &&& pos as int <= e.end as int - 1
                &&& bytes@.subrange(pos as int, e.end as int - 1)
                    == ckc_spec::v1text::esc_all(
                        e.name.subrange(name@.len() as int, e.name.len() as int),
                    )
            },
        decreases bytes.len() - pos,
    {
        proof {
            if let Some(e) = expected@ {
                let k = name@.len();
                if k == e.name.len() {
                    assert_seqs_equal!(e.name.subrange(k as int, e.name.len() as int)
                        == Seq::<u8>::empty());
                    reveal_with_fuel(ckc_spec::v1text::esc_all, 1);
                    assert(bytes@.subrange(pos as int, e.end as int - 1).len()
                        == e.end as int - 1 - pos as int);
                    assert(bytes@.subrange(pos as int, e.end as int - 1).len() == 0);
                    assert(pos as int == e.end as int - 1);
                    assert(pos == e.end - 1);
                    assert(bytes@[pos as int] == bytes@[e.end as int - 1]);
                    assert(bytes@[pos as int] == 0x27);
                    assert(false);
                }
                assert(k < e.name.len());
                let remaining = e.name.subrange(k as int, e.name.len() as int);
                let value = e.name[k as int];
                let rest = e.name.subrange(k as int + 1, e.name.len() as int);
                assert(remaining.len() > 0);
                assert(remaining[0] == value);
                assert_seqs_equal!(remaining.drop_first() == rest);
                reveal_with_fuel(ckc_spec::v1text::esc_all, 2);
                assert(ckc_spec::v1text::esc_all(remaining)
                    == ckc_spec::v1text::esc_byte(value)
                        + ckc_spec::v1text::esc_all(rest));
                esc_byte_first_not_quote(value);
                assert(0 <= pos as int <= e.end as int - 1 <= bytes@.len());
                prefix_before_suffix(
                    bytes@,
                    pos as int,
                    e.end as int - 1,
                    ckc_spec::v1text::esc_byte(value),
                    ckc_spec::v1text::esc_all(rest),
                );
                suffix_after_prefix(
                    bytes@,
                    pos as int,
                    e.end as int - 1,
                    ckc_spec::v1text::esc_byte(value),
                    ckc_spec::v1text::esc_all(rest),
                );
                assert(pos as int + ckc_spec::v1text::esc_byte(value).len()
                    <= e.end as int - 1);
            }
        }
        let ghost expected_escape = match expected@ {
            Some(e) => {
                let value = e.name[name@.len() as int];
                Some(GEscapeExpected {
                    value,
                    end: (pos as int + ckc_spec::v1text::esc_byte(value).len()) as usize,
                })
            },
            None => None,
        };
        proof {
            if let Some(e) = expected@ {
                let value = e.name[name@.len() as int];
                let end_int = pos as int + ckc_spec::v1text::esc_byte(value).len();
                assert(0 <= end_int <= e.end as int - 1 < bytes@.len());
                assert((end_int as usize) as int == end_int);
                assert(expected_escape matches Some(x)
                    && x.value == value && x.end as int == end_int);
                esc_byte_first_not_quote(value);
                assert(pos < end_int as usize <= bytes.len());
                assert(ckc_spec::v1text::esc_byte(value)
                    == bytes@.subrange(pos as int, end_int));
            }
        }
        let parsed = match parse_escaped_byte(bytes, pos, Ghost(expected_escape), at) {
            Some(e) => e,
            None => return None,
        };
        let ghost old_name = name@;
        let ghost old_pos = pos;
        name.push(parsed.value);
        pos = parsed.end;
        proof {
            esc_all_push(old_name, parsed.value);
            assert_seqs_equal!(bytes@.subrange(start as int + 1, pos as int)
                == bytes@.subrange(start as int + 1, old_pos as int)
                    + bytes@.subrange(old_pos as int, pos as int));
            if let Some(e) = expected@ {
                let k = old_name.len();
                let value = e.name[k as int];
                let rest = e.name.subrange(k as int + 1, e.name.len() as int);
                assert(expected_escape matches Some(x)
                    && x.value == value
                    && x.end as int
                        == old_pos as int + ckc_spec::v1text::esc_byte(value).len());
                assert(parsed.value == value);
                assert(pos as int
                    == old_pos as int + ckc_spec::v1text::esc_byte(value).len());
                assert_seqs_equal!(name@ == old_name.push(value));
                assert_seqs_equal!(e.name.subrange(0, k as int + 1)
                    == old_name.push(value));
                assert(0 <= old_pos as int <= e.end as int - 1 <= bytes@.len());
                suffix_after_prefix(
                    bytes@,
                    old_pos as int,
                    e.end as int - 1,
                    ckc_spec::v1text::esc_byte(value),
                    ckc_spec::v1text::esc_all(rest),
                );
            }
        }
    }
    proof {
        if let Some(e) = expected@ {
            assert(pos < bytes@.len());
            assert(bytes@[pos as int] == 0x27);
            if name@.len() < e.name.len() {
                let k = name@.len();
                let remaining = e.name.subrange(k as int, e.name.len() as int);
                let value = e.name[k as int];
                assert(remaining.len() > 0);
                assert(remaining[0] == value);
                reveal_with_fuel(ckc_spec::v1text::esc_all, 2);
                esc_byte_first_not_quote(value);
                assert(ckc_spec::v1text::esc_all(remaining).len() > 0);
                assert(pos < e.end - 1);
                assert(bytes@.subrange(pos as int, e.end as int - 1)[0]
                    == bytes@[pos as int]);
                assert(ckc_spec::v1text::esc_all(remaining)[0]
                    == ckc_spec::v1text::esc_byte(value)[0]);
                assert(false);
            }
            assert(name@.len() == e.name.len());
            assert_seqs_equal!(name@ == e.name);
            assert_seqs_equal!(e.name.subrange(name@.len() as int, e.name.len() as int)
                == Seq::<u8>::empty());
            reveal_with_fuel(ckc_spec::v1text::esc_all, 1);
            assert(bytes@.subrange(pos as int, e.end as int - 1).len() == 0);
            assert(pos == e.end - 1);
        }
    }
    if pos == bytes.len() {
        raise_at(at, pos, bytes.len());
        return None;
    }
    if atom_bare_exec(name.as_slice()) {
        raise_at(at, pos, bytes.len());
        return None;
    }
    let end = pos + 1;
    proof {
        reveal(ckc_spec::v1text::atom_bytes);
        assert_seqs_equal!(bytes@.subrange(start as int, end as int)
            == seq![0x27u8] + bytes@.subrange(start as int + 1, pos as int)
                + seq![0x27u8]);
        if let Some(e) = expected@ {
            assert(name@ == e.name);
            assert(end == e.end);
        }
    }
    Some(EParsedAtom { name, end })
}

pub open spec fn decimal_digit(b: u8) -> nat {
    (b as int - 0x30) as nat
}

pub open spec fn decimal_value(s: Seq<u8>) -> nat
    decreases s.len(),
{
    if s.len() == 0 {
        0
    } else {
        decimal_value(s.drop_last()) * 10 + decimal_digit(s.last())
    }
}

pub open spec fn canonical_decimal(s: Seq<u8>) -> bool {
    &&& s.len() > 0
    &&& ckc_spec::v1text::all_in(s, |b: u8| ckc_spec::v1text::is_digit_b(b))
    &&& (s.len() == 1 || s[0] != 0x30)
}

proof fn decimal_digit_bounds(b: u8)
    requires ckc_spec::v1text::is_digit_b(b),
    ensures
        decimal_digit(b) < 10,
        ckc_spec::v1text::digit_byte(decimal_digit(b) as int) == b,
{
    reveal(ckc_spec::v1text::is_digit_b);
    reveal(decimal_digit);
    reveal(ckc_spec::v1text::digit_byte);
}

proof fn decimal_all_drop_last(s: Seq<u8>)
    requires
        s.len() > 0,
        ckc_spec::v1text::all_in(s, |b: u8| ckc_spec::v1text::is_digit_b(b)),
    ensures
        ckc_spec::v1text::all_in(s.drop_last(), |b: u8| ckc_spec::v1text::is_digit_b(b)),
{
    reveal(ckc_spec::v1text::all_in);
    assert forall|i: int| #![auto] 0 <= i < s.drop_last().len()
        ==> ckc_spec::v1text::is_digit_b(s.drop_last()[i]) by {
        if 0 <= i < s.drop_last().len() {
            assert(i < s.len());
            assert(s.drop_last()[i] == s[i]);
        }
    }
}

proof fn canonical_decimal_drop_last(s: Seq<u8>)
    requires
        s.len() > 1,
        canonical_decimal(s),
    ensures canonical_decimal(s.drop_last()),
{
    decimal_all_drop_last(s);
    reveal(canonical_decimal);
    if s.drop_last().len() > 1 {
        assert(s.drop_last()[0] == s[0]);
    }
}

proof fn decimal_positive(s: Seq<u8>)
    requires
        canonical_decimal(s),
        s[0] != 0x30,
    ensures decimal_value(s) > 0,
    decreases s.len(),
{
    reveal_with_fuel(decimal_value, 2);
    reveal(canonical_decimal);
    decimal_digit_bounds(s.last());
    if s.len() == 1 {
        assert(s.last() == s[0]);
        assert(decimal_digit(s[0]) > 0);
    } else {
        canonical_decimal_drop_last(s);
        assert(s.drop_last()[0] == s[0]);
        decimal_positive(s.drop_last());
    }
}

proof fn canonical_decimal_roundtrip(s: Seq<u8>)
    requires canonical_decimal(s),
    ensures ckc_spec::v1text::udec_bytes(decimal_value(s)) == s,
    decreases s.len(),
{
    let p = s.drop_last();
    let d = decimal_digit(s.last());
    reveal(canonical_decimal);
    decimal_digit_bounds(s.last());
    reveal_with_fuel(decimal_value, 2);
    if s.len() == 1 {
        assert(p.len() == 0);
        assert(decimal_value(s) == d);
        assert(d < 10);
        reveal_with_fuel(ckc_spec::v1text::udec_bytes, 2);
        assert_seqs_equal!(s == seq![s.last()]);
    } else {
        canonical_decimal_drop_last(s);
        canonical_decimal_roundtrip(p);
        assert(s == p.push(s.last()));
        assert(p[0] == s[0]);
        decimal_positive(p);
        let q = decimal_value(p);
        let n = decimal_value(s);
        assert(n == q * 10 + d);
        lemma_fundamental_div_mod_converse(n as int, 10, q as int, d as int);
        assert(n / 10 == q);
        assert(n % 10 == d);
        assert(n >= 10);
        reveal_with_fuel(ckc_spec::v1text::udec_bytes, 2);
    }
}

proof fn udec_canonical(n: nat)
    ensures
        canonical_decimal(ckc_spec::v1text::udec_bytes(n)),
        n > 0 ==> ckc_spec::v1text::udec_bytes(n)[0] != 0x30,
    decreases n,
{
    reveal_with_fuel(ckc_spec::v1text::udec_bytes, 2);
    reveal(ckc_spec::v1text::digit_byte);
    reveal(ckc_spec::v1text::is_digit_b);
    reveal(ckc_spec::v1text::all_in);
    reveal(canonical_decimal);
    if n < 10 {
        assert(ckc_spec::v1text::udec_bytes(n).len() == 1);
        assert(ckc_spec::v1text::is_digit_b(
            ckc_spec::v1text::digit_byte(n as int),
        ));
        if n > 0 {
            assert(ckc_spec::v1text::digit_byte(n as int) != 0x30);
        }
    } else {
        let q = n / 10;
        let d = n % 10;
        assert(q > 0);
        assert(q < n);
        assert(d < 10);
        udec_canonical(q);
        let prefix = ckc_spec::v1text::udec_bytes(q);
        let digit = ckc_spec::v1text::digit_byte(d as int);
        assert(ckc_spec::v1text::is_digit_b(digit));
        assert forall|i: int| #![auto] 0 <= i < (prefix + seq![digit]).len()
            ==> ckc_spec::v1text::is_digit_b((prefix + seq![digit])[i]) by {
            if 0 <= i < (prefix + seq![digit]).len() {
                if i < prefix.len() {
                    assert((prefix + seq![digit])[i] == prefix[i]);
                } else {
                    assert(i == prefix.len());
                    assert((prefix + seq![digit])[i] == digit);
                }
            }
        }
        assert((prefix + seq![digit])[0] == prefix[0]);
        assert(prefix[0] != 0x30);
    }
}

pub open spec fn decimal_end(bytes: Seq<u8>, end: usize) -> bool {
    term_boundary(bytes, end as int)
        || end < bytes.len()
            && !ckc_spec::v1text::is_digit_b(bytes[end as int])
}

pub ghost struct GDecimalExpected {
    pub value: nat,
    pub end: usize,
}

pub struct EParsedDecimal {
    pub value: Ghost<nat>,
    pub end: usize,
}

pub open spec fn parsed_decimal_ok(bytes: Seq<u8>, start: usize, d: &EParsedDecimal) -> bool {
    &&& start < d.end <= bytes.len()
    &&& ckc_spec::v1text::udec_bytes(d.value@)
        == bytes.subrange(start as int, d.end as int)
}

fn parse_decimal(
    bytes: &[u8],
    start: usize,
    expected: Ghost<Option<GDecimalExpected>>,
at: &mut usize,
) -> (r: Option<EParsedDecimal>)
    requires
        *old(at) <= bytes@.len(),
        start < bytes@.len(),
        expected@ matches Some(e) ==> {
            &&& start < e.end <= bytes@.len()
            &&& ckc_spec::v1text::udec_bytes(e.value)
                == bytes@.subrange(start as int, e.end as int)
            &&& decimal_end(bytes@, e.end)
        },
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r matches Some(d) ==> parsed_decimal_ok(bytes@, start, &d),
        expected@ matches Some(e) ==> r matches Some(d)
            && d.value@ == e.value && d.end == e.end,
{
    proof {
        if let Some(e) = expected@ {
            udec_canonical(e.value);
            reveal(canonical_decimal);
            reveal(ckc_spec::v1text::all_in);
            assert(ckc_spec::v1text::udec_bytes(e.value).len() == e.end - start);
            assert(bytes@[start as int] == ckc_spec::v1text::udec_bytes(e.value)[0]);
            assert(ckc_spec::v1text::is_digit_b(bytes@[start as int]));
        }
    }
    if !is_digit_b(bytes[start]) {
        raise_at(at, start, bytes.len());
        return None;
    }
    let mut digits = Vec::new();
    let mut pos = start;
    while pos < bytes.len() && is_digit_b(bytes[pos])
        invariant
            *old(at) <= *at <= bytes@.len(),
            start <= pos <= bytes@.len(),
            digits@ == bytes@.subrange(start as int, pos as int),
            forall|i: int| 0 <= i < digits@.len()
                ==> ckc_spec::v1text::is_digit_b(digits@[i]),
            expected@ matches Some(e) ==> {
                &&& start < e.end <= bytes@.len()
                &&& ckc_spec::v1text::udec_bytes(e.value)
                    == bytes@.subrange(start as int, e.end as int)
                &&& canonical_decimal(ckc_spec::v1text::udec_bytes(e.value))
                &&& decimal_end(bytes@, e.end)
                &&& pos <= e.end
            },
        decreases bytes.len() - pos,
    {
        proof {
            if let Some(e) = expected@ {
                if pos == e.end {
                    if pos < bytes@.len() {
                        reveal(decimal_end);
                        if term_boundary(bytes@, e.end as int) {
                            boundary_stops_digit(bytes@, e.end as int);
                        }
                        assert(!ckc_spec::v1text::is_digit_b(bytes@[pos as int]));
                        assert(false);
                    }
                    assert(false);
                }
                assert(pos < e.end);
            }
        }
        let ghost old_pos = pos;
        digits.push(bytes[pos]);
        proof { subrange_push(bytes@, start as int, old_pos as int); }
        pos += 1;
    }
    proof {
        if let Some(e) = expected@ {
            if pos < e.end {
                let i = pos as int - start as int;
                assert(0 <= i < ckc_spec::v1text::udec_bytes(e.value).len());
                assert(bytes@.subrange(start as int, e.end as int)[i]
                    == bytes@[pos as int]);
                reveal(canonical_decimal);
                reveal(ckc_spec::v1text::all_in);
                assert(ckc_spec::v1text::is_digit_b(bytes@[pos as int]));
                assert(false);
            }
            assert(pos == e.end);
            assert_seqs_equal!(digits@ == ckc_spec::v1text::udec_bytes(e.value));
        }
    }
    if digits.len() > 1 && digits[0] == 0x30 {
        raise_at(at, start + 1, bytes.len());
        return None;
    }
    let ghost value = match expected@ {
        Some(e) => e.value,
        None => decimal_value(digits@),
    };
    proof {
        reveal(ckc_spec::v1text::all_in);
        reveal(canonical_decimal);
        match expected@ {
            Some(e) => {
                assert(value == e.value);
                assert(pos == e.end);
            },
            None => canonical_decimal_roundtrip(digits@),
        }
        reveal(parsed_decimal_ok);
    }
    Some(EParsedDecimal { value: Ghost(value), end: pos })
}

pub ghost struct GIntExpected {
    pub value: int,
    pub end: usize,
}

pub struct EParsedInt {
    pub value: Ghost<int>,
    pub end: usize,
}

pub open spec fn parsed_int_ok(bytes: Seq<u8>, start: usize, n: &EParsedInt) -> bool {
    &&& start < n.end <= bytes.len()
    &&& ckc_spec::v1text::dec_bytes(n.value@)
        == bytes.subrange(start as int, n.end as int)
}

fn parse_integer(
    bytes: &[u8],
    start: usize,
    expected: Ghost<Option<GIntExpected>>,
at: &mut usize,
) -> (r: Option<EParsedInt>)
    requires
        *old(at) <= bytes@.len(),
        start < bytes@.len(),
        expected@ matches Some(e) ==> {
            &&& start < e.end <= bytes@.len()
            &&& ckc_spec::v1text::dec_bytes(e.value)
                == bytes@.subrange(start as int, e.end as int)
            &&& term_boundary(bytes@, e.end as int)
        },
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r matches Some(n) ==> parsed_int_ok(bytes@, start, &n),
        expected@ matches Some(e) ==> r matches Some(n)
            && n.value@ == e.value && n.end == e.end,
{
    proof {
        if let Some(e) = expected@ {
            reveal(ckc_spec::v1text::dec_bytes);
            if e.value < 0 {
                let magnitude = (-e.value) as nat;
                udec_canonical(magnitude);
                assert(magnitude > 0);
                assert(bytes@.subrange(start as int, e.end as int)
                    == seq![0x2du8] + ckc_spec::v1text::udec_bytes(magnitude));
                prefix_before_suffix(
                    bytes@,
                    start as int,
                    e.end as int,
                    seq![0x2du8],
                    ckc_spec::v1text::udec_bytes(magnitude),
                );
                suffix_after_prefix(
                    bytes@,
                    start as int,
                    e.end as int,
                    seq![0x2du8],
                    ckc_spec::v1text::udec_bytes(magnitude),
                );
                assert(bytes@.subrange(start as int, start as int + 1)[0]
                    == bytes@[start as int]);
                assert(bytes@.subrange(start as int, start as int + 1)[0] == 0x2d);
                assert(bytes@[start as int] == 0x2d);
                assert(start + 1 < e.end);
            } else {
                udec_canonical(e.value as nat);
                reveal(canonical_decimal);
                reveal(ckc_spec::v1text::all_in);
                assert(bytes@[start as int]
                    == ckc_spec::v1text::udec_bytes(e.value as nat)[0]);
                assert(ckc_spec::v1text::is_digit_b(bytes@[start as int]));
                assert(bytes@[start as int] != 0x2d);
            }
        }
    }
    if bytes[start] == 0x2d {
        proof {
            if let Some(e) = expected@ {
                assert(e.value < 0);
            }
        }
        if bytes.len() - start < 2 {
            raise_at(at, bytes.len(), bytes.len());
            return None;
        }
        let ghost expected_decimal = match expected@ {
            Some(e) => Some(GDecimalExpected {
                value: (-e.value) as nat,
                end: e.end,
            }),
            None => None,
        };
        proof {
            if let Some(e) = expected@ {
                let magnitude = (-e.value) as nat;
                reveal(ckc_spec::v1text::dec_bytes);
                assert(expected_decimal matches Some(d)
                    && d.value == magnitude && d.end == e.end);
                assert(ckc_spec::v1text::udec_bytes(magnitude)
                    == bytes@.subrange(start as int + 1, e.end as int));
            }
        }
        let d = match parse_decimal(bytes, start + 1, Ghost(expected_decimal), at) {
            Some(d) => d,
            None => return None,
        };
        proof {
            if let Some(e) = expected@ {
                let magnitude = (-e.value) as nat;
                udec_canonical(magnitude);
                assert(magnitude > 0);
                assert(d.value@ == magnitude);
                assert(d.end == e.end);
                assert(bytes@[start as int + 1]
                    == ckc_spec::v1text::udec_bytes(magnitude)[0]);
                assert(bytes@[start as int + 1] != 0x30);
            }
        }
        if bytes[start + 1] == 0x30 {
            raise_at(at, start + 1, bytes.len());
            return None;
        }
        let ghost value = -(d.value@ as int);
        proof {
            assert(d.value@ > 0) by {
                let s = bytes@.subrange(start as int + 1, d.end as int);
                assert(ckc_spec::v1text::udec_bytes(d.value@) == s);
                assert(s[0] != 0x30);
                assert(d.value@ != 0) by {
                    reveal_with_fuel(ckc_spec::v1text::udec_bytes, 2);
                }
            }
            reveal(ckc_spec::v1text::dec_bytes);
            assert_seqs_equal!(bytes@.subrange(start as int, d.end as int)
                == seq![0x2du8] + bytes@.subrange(start as int + 1, d.end as int));
            if let Some(e) = expected@ {
                assert(value == e.value);
                assert(d.end == e.end);
            }
        }
        return Some(EParsedInt { value: Ghost(value), end: d.end });
    }
    proof {
        if let Some(e) = expected@ {
            assert(e.value >= 0);
        }
    }
    let ghost expected_decimal = match expected@ {
        Some(e) => Some(GDecimalExpected { value: e.value as nat, end: e.end }),
        None => None,
    };
    let d = match parse_decimal(bytes, start, Ghost(expected_decimal), at) {
        Some(d) => d,
        None => return None,
    };
    proof {
        reveal(ckc_spec::v1text::dec_bytes);
        if let Some(e) = expected@ {
            assert(d.value@ == e.value as nat);
            assert(d.value@ as int == e.value);
            assert(d.end == e.end);
        }
    }
    Some(EParsedInt { value: Ghost(d.value@ as int), end: d.end })
}

pub ghost struct GVarExpected {
    pub value: nat,
    pub end: usize,
}

pub struct EParsedVar {
    pub value: Ghost<nat>,
    pub end: usize,
}

pub open spec fn parsed_var_ok(bytes: Seq<u8>, start: usize, v: &EParsedVar) -> bool {
    &&& start < v.end <= bytes.len()
    &&& ckc_spec::v1text::var_bytes(v.value@)
        == bytes.subrange(start as int, v.end as int)
}

fn parse_variable(
    bytes: &[u8],
    start: usize,
    expected: Ghost<Option<GVarExpected>>,
at: &mut usize,
) -> (r: Option<EParsedVar>)
    requires
        *old(at) <= bytes@.len(),
        start < bytes@.len(),
        expected@ matches Some(e) ==> {
            &&& start < e.end <= bytes@.len()
            &&& ckc_spec::v1text::var_bytes(e.value)
                == bytes@.subrange(start as int, e.end as int)
            &&& term_boundary(bytes@, e.end as int)
        },
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r matches Some(v) ==> parsed_var_ok(bytes@, start, &v),
        expected@ matches Some(e) ==> r matches Some(v)
            && v.value@ == e.value && v.end == e.end,
{
    proof {
        if let Some(e) = expected@ {
            let index = e.value % 26;
            let q = e.value / 26;
            let letter = (0x41 + index) as u8;
            assert(index < 26);
            reveal(ckc_spec::v1text::var_bytes);
            if q == 0 {
                assert(bytes@.subrange(start as int, e.end as int) == seq![letter]);
                assert(e.end == start + 1);
                assert(bytes@.subrange(start as int, e.end as int)[0]
                    == bytes@[start as int]);
                assert(bytes@.subrange(start as int, e.end as int)[0] == letter);
                assert(bytes@[start as int] == letter);
                if e.end < bytes@.len() {
                    boundary_stops_digit(bytes@, e.end as int);
                }
            } else {
                udec_canonical(q);
                assert(bytes@.subrange(start as int, e.end as int)
                    == seq![letter] + ckc_spec::v1text::udec_bytes(q));
                prefix_before_suffix(
                    bytes@,
                    start as int,
                    e.end as int,
                    seq![letter],
                    ckc_spec::v1text::udec_bytes(q),
                );
                suffix_after_prefix(
                    bytes@,
                    start as int,
                    e.end as int,
                    seq![letter],
                    ckc_spec::v1text::udec_bytes(q),
                );
                assert(bytes@.subrange(start as int, start as int + 1)[0]
                    == bytes@[start as int]);
                assert(bytes@.subrange(start as int, start as int + 1)[0] == letter);
                assert(bytes@[start as int] == letter);
                assert(start + 1 < e.end);
                reveal(canonical_decimal);
                reveal(ckc_spec::v1text::all_in);
                assert(bytes@[start as int + 1]
                    == ckc_spec::v1text::udec_bytes(q)[0]);
                assert(ckc_spec::v1text::is_digit_b(bytes@[start as int + 1]));
            }
            assert(0x41 <= bytes@[start as int] <= 0x5a);
        }
    }
    let letter = bytes[start];
    if letter < 0x41 || letter > 0x5a {
        raise_at(at, start, bytes.len());
        return None;
    }
    let index = letter - 0x41;
    if start + 1 == bytes.len() || !is_digit_b(bytes[start + 1]) {
        proof {
            if let Some(e) = expected@ {
                let q = e.value / 26;
                if q > 0 {
                    reveal(ckc_spec::v1text::var_bytes);
                    udec_canonical(q);
                    reveal(canonical_decimal);
                    reveal(ckc_spec::v1text::all_in);
                    assert(start + 1 < e.end <= bytes@.len());
                    assert(bytes@[start as int + 1]
                        == ckc_spec::v1text::udec_bytes(q)[0]);
                    assert(ckc_spec::v1text::is_digit_b(bytes@[start as int + 1]));
                    assert(false);
                }
                assert(q == 0);
                reveal(ckc_spec::v1text::var_bytes);
                assert(e.end == start + 1);
            }
        }
        let ghost value = match expected@ {
            Some(e) => e.value,
            None => index as nat,
        };
        proof {
            match expected@ {
                Some(e) => {
                    assert(value == e.value);
                    assert(e.end == start + 1);
                },
                None => {
                    reveal(ckc_spec::v1text::var_bytes);
                    assert(index < 26);
                    assert(index as nat / 26 == 0);
                    assert(index as nat % 26 == index as nat);
                    assert_seqs_equal!(bytes@.subrange(start as int, (start + 1) as int)
                        == seq![letter]);
                },
            }
            reveal(parsed_var_ok);
        }
        return Some(EParsedVar { value: Ghost(value), end: start + 1 });
    }
    proof {
        if let Some(e) = expected@ {
            assert(e.value / 26 > 0);
        }
    }
    if bytes[start + 1] == 0x30 {
        raise_at(at, start + 1, bytes.len());
        return None;
    }
    let ghost expected_decimal = match expected@ {
        Some(e) => Some(GDecimalExpected { value: e.value / 26, end: e.end }),
        None => None,
    };
    proof {
        if let Some(e) = expected@ {
            let q = e.value / 26;
            reveal(ckc_spec::v1text::var_bytes);
            assert(ckc_spec::v1text::udec_bytes(q)
                == bytes@.subrange(start as int + 1, e.end as int));
        }
    }
    let d = match parse_decimal(bytes, start + 1, Ghost(expected_decimal), at) {
        Some(d) => d,
        None => return None,
    };
    proof {
        if let Some(e) = expected@ {
            let q = e.value / 26;
            udec_canonical(q);
            assert(q > 0);
            assert(d.value@ == q);
            assert(d.end == e.end);
            assert(bytes@[start as int + 1]
                == ckc_spec::v1text::udec_bytes(q)[0]);
            assert(bytes@[start as int + 1] != 0x30);
        }
    }
    let ghost q = d.value@;
    let ghost value = match expected@ {
        Some(e) => e.value,
        None => q * 26 + index as nat,
    };
    proof {
        match expected@ {
            Some(e) => {
                assert(value == e.value);
                assert(d.end == e.end);
                reveal(ckc_spec::v1text::var_bytes);
            },
            None => {
                assert(q > 0) by {
                    assert(ckc_spec::v1text::udec_bytes(q)[0] == bytes@[start as int + 1]);
                    if q == 0 {
                        reveal_with_fuel(ckc_spec::v1text::udec_bytes, 2);
                    }
                }
                lemma_fundamental_div_mod_converse(value as int, 26, q as int, index as int);
                assert(value / 26 == q);
                assert(value % 26 == index as nat);
                reveal(ckc_spec::v1text::var_bytes);
                assert_seqs_equal!(bytes@.subrange(start as int, d.end as int)
                    == seq![letter] + bytes@.subrange(start as int + 1, d.end as int));
            },
        }
        reveal(parsed_var_ok);
    }
    Some(EParsedVar { value: Ghost(value), end: d.end })
}

proof fn expected_atom_dispatch(
    bytes: Seq<u8>,
    start: usize,
    name: Seq<u8>,
    end: usize,
)
    requires
        start < end <= bytes.len(),
        ckc_spec::v1text::atom_bytes(name)
            == bytes.subrange(start as int, end as int),
    ensures
        ckc_spec::v1text::is_lower_b(bytes[start as int])
            ==> ckc_spec::v1text::alpha_bare(name),
        ckc_spec::v1text::is_graphic_b(bytes[start as int])
            ==> ckc_spec::v1text::graphic_bare(name),
        bytes[start as int] == 0x27 ==> !ckc_spec::v1text::atom_bare(name),
        !ckc_spec::v1text::is_lower_b(bytes[start as int])
            && !ckc_spec::v1text::is_graphic_b(bytes[start as int])
            && bytes[start as int] != 0x27
            ==> ckc_spec::v1text::solo_bare(name),
{
    reveal(ckc_spec::v1text::atom_bytes);
    if ckc_spec::v1text::atom_bare(name) {
        assert(name == bytes.subrange(start as int, end as int));
        assert(name.len() == end - start);
        assert(name.len() > 0);
        assert(name[0] == bytes.subrange(start as int, end as int)[0]);
        assert(bytes.subrange(start as int, end as int)[0] == bytes[start as int]);
        assert(name[0] == bytes[start as int]);
        reveal(ckc_spec::v1text::atom_bare);
        reveal(ckc_spec::v1text::alpha_bare);
        reveal(ckc_spec::v1text::graphic_bare);
        reveal(ckc_spec::v1text::solo_bare);
        reveal(ckc_spec::v1text::all_in);
        reveal(ckc_spec::v1text::is_lower_b);
        reveal(ckc_spec::v1text::is_graphic_b);
        seq_eq_one(name, 0x3b);
        seq_eq_one(name, 0x21);
        seq_eq_two(name, 0x7b, 0x7d);
    } else {
        assert(bytes.subrange(start as int, end as int)
            == seq![0x27u8] + ckc_spec::v1text::esc_all(name) + seq![0x27u8]);
        range_between_bytes(
            bytes,
            start as int,
            end as int,
            0x27,
            ckc_spec::v1text::esc_all(name),
            0x27,
        );
        reveal(ckc_spec::v1text::is_lower_b);
        reveal(ckc_spec::v1text::is_graphic_b);
    }
}

fn parse_atom(
    bytes: &[u8],
    start: usize,
    expected: Ghost<Option<GAtomExpected>>,
at: &mut usize,
) -> (r: Option<EParsedAtom>)
    requires
        *old(at) <= bytes@.len(),
        start < bytes@.len(),
        expected@ matches Some(e) ==> {
            &&& start < e.end <= bytes@.len()
            &&& ckc_spec::v1text::atom_bytes(e.name)
                == bytes@.subrange(start as int, e.end as int)
            &&& atom_boundary(bytes@, e.end as int)
        },
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r matches Some(a) ==> parsed_atom_ok(bytes@, start, &a),
        expected@ matches Some(e) ==> r matches Some(a)
            && a.name@ == e.name && a.end == e.end,
{
    proof {
        if let Some(e) = expected@ {
            expected_atom_dispatch(bytes@, start, e.name, e.end);
        }
    }
    if is_lower_b(bytes[start]) {
        parse_alpha_atom(bytes, start, expected, at)
    } else if is_graphic_b(bytes[start]) {
        parse_graphic_atom(bytes, start, expected, at)
    } else if bytes[start] == 0x27 {
        parse_quoted_atom(bytes, start, expected, at)
    } else {
        parse_solo_atom(bytes, start, expected, at)
    }
}

pub struct ESpannedTerm {
    pub parsed: EParsedTerm,
    pub start: usize,
    pub end: usize,
}

impl View for ESpannedTerm {
    type V = Term;

    open spec fn view(&self) -> Term {
        self.parsed@
    }
}

pub open spec fn spanned_term_ok(bytes: Seq<u8>, t: &ESpannedTerm) -> bool {
    &&& t.start < t.end <= bytes.len()
    &&& parsed_term_ok(&t.parsed)
    &&& ckc_spec::v1text::term_bytes(t@)
        == bytes.subrange(t.start as int, t.end as int)
}

fn span_atom(bytes: &[u8], start: usize, a: EParsedAtom) -> (out: ESpannedTerm)
    requires parsed_atom_ok(bytes@, start, &a),
    ensures
        spanned_term_ok(bytes@, &out),
        out.start == start,
        out@ == Term::Atom(a.name@),
{
    let end = a.end;
    let ghost term = Term::Atom(a.name@);
    proof {
        reveal(ckc_spec::term::wf_term);
        reveal(ckc_spec::term::ground);
        reveal(ckc_spec::term::no_dollar_var);
        reveal(ckc_spec::v1text::term_bytes);
        reveal(top_matches);
        reveal(parsed_term_ok);
        reveal(spanned_term_ok);
    }
    ESpannedTerm {
        parsed: EParsedTerm {
            term: Ghost(term),
            top: ETermTop::Atom(a.name),
            ground: true,
            no_dollar: true,
        },
        start,
        end,
    }
}

fn span_integer(bytes: &[u8], start: usize, n: EParsedInt) -> (out: ESpannedTerm)
    requires parsed_int_ok(bytes@, start, &n),
    ensures
        spanned_term_ok(bytes@, &out),
        out.start == start,
        out@ == Term::Int(n.value@),
{
    let end = n.end;
    let ghost term = Term::Int(n.value@);
    proof {
        reveal(ckc_spec::term::wf_term);
        reveal(ckc_spec::term::ground);
        reveal(ckc_spec::term::no_dollar_var);
        reveal(ckc_spec::v1text::term_bytes);
        reveal(top_matches);
        reveal(parsed_term_ok);
        reveal(spanned_term_ok);
    }
    ESpannedTerm {
        parsed: EParsedTerm {
            term: Ghost(term),
            top: ETermTop::Int,
            ground: true,
            no_dollar: true,
        },
        start,
        end,
    }
}

fn span_variable(bytes: &[u8], start: usize, v: EParsedVar) -> (out: ESpannedTerm)
    requires parsed_var_ok(bytes@, start, &v),
    ensures
        spanned_term_ok(bytes@, &out),
        out.start == start,
        out@ == Term::Var(v.value@),
{
    let end = v.end;
    let ghost term = Term::Var(v.value@);
    proof {
        reveal(ckc_spec::term::wf_term);
        reveal(ckc_spec::term::ground);
        reveal(ckc_spec::term::no_dollar_var);
        reveal(ckc_spec::v1text::term_bytes);
        reveal(top_matches);
        reveal(parsed_term_ok);
        reveal(spanned_term_ok);
    }
    ESpannedTerm {
        parsed: EParsedTerm {
            term: Ghost(term),
            top: ETermTop::Var,
            ground: false,
            no_dollar: true,
        },
        start,
        end,
    }
}

fn span_nil(bytes: &[u8], start: usize) -> (out: ESpannedTerm)
    requires
        start <= usize::MAX - 2,
        start as int + 1 < bytes@.len(),
        bytes@[start as int] == 0x5b,
        bytes@[start as int + 1] == 0x5d,
    ensures
        spanned_term_ok(bytes@, &out),
        out.start == start,
        out@ == Term::Nil,
{
    let parsed = nil_term();
    let end = start + 2;
    proof {
        reveal_strlit("[]");
        reveal(ckc_spec::v1text::ascii);
        assert(ckc_spec::v1text::ascii("[]"@) == seq![0x5bu8, 0x5du8]);
        reveal(ckc_spec::v1text::term_bytes);
        assert(ckc_spec::v1text::term_bytes(Term::Nil) == seq![0x5bu8, 0x5du8]);
        reveal(spanned_term_ok);
        assert_seqs_equal!(bytes@.subrange(start as int, end as int)
            == seq![0x5bu8, 0x5du8]);
    }
    ESpannedTerm { parsed, start, end }
}

pub open spec fn atomic_term(term: Term) -> bool {
    !matches!(term, Term::Comp(_, _))
}

pub ghost struct GAtomicExpected {
    pub term: Term,
    pub end: usize,
}

proof fn expected_atomic_shape(
    bytes: Seq<u8>,
    start: usize,
    end: usize,
    term: Term,
)
    requires
        term_at(bytes, start as int, end as int, term),
        atomic_term(term),
    ensures
        match term {
            Term::Var(_) => 0x41 <= bytes[start as int] <= 0x5a,
            Term::Int(_) => ckc_spec::v1text::is_digit_b(bytes[start as int])
                || bytes[start as int] == 0x2d
                    && start + 1 < bytes.len()
                    && ckc_spec::v1text::is_digit_b(bytes[start as int + 1]),
            Term::Nil => start + 1 < bytes.len()
                && bytes[start as int] == 0x5b
                && bytes[start as int + 1] == 0x5d,
            Term::Atom(_) => {
                &&& !(0x41 <= bytes[start as int] <= 0x5a)
                &&& !ckc_spec::v1text::is_digit_b(bytes[start as int])
                &&& bytes[start as int] != 0x5b
                &&& !(bytes[start as int] == 0x2d
                    && start + 1 < bytes.len()
                    && ckc_spec::v1text::is_digit_b(bytes[start as int + 1]))
            },
            Term::Comp(_, _) => false,
        },
{
    reveal(term_at);
    reveal(atomic_term);
    match term {
        Term::Var(k) => {
            reveal(ckc_spec::v1text::term_bytes);
            reveal(ckc_spec::v1text::var_bytes);
            let letter = (0x41 + k % 26) as u8;
            assert(k % 26 < 26);
            if k / 26 == 0 {
                assert(bytes.subrange(start as int, end as int) == seq![letter]);
                assert(bytes.subrange(start as int, end as int)[0] == bytes[start as int]);
                assert(bytes.subrange(start as int, end as int)[0] == letter);
            } else {
                prefix_before_suffix(
                    bytes,
                    start as int,
                    end as int,
                    seq![letter],
                    ckc_spec::v1text::udec_bytes(k / 26),
                );
                assert(bytes.subrange(start as int, start as int + 1)[0]
                    == bytes[start as int]);
                assert(bytes.subrange(start as int, start as int + 1)[0] == letter);
            }
            assert(0x41 <= letter <= 0x5a);
        },
        Term::Int(n) => {
            reveal(ckc_spec::v1text::term_bytes);
            reveal(ckc_spec::v1text::dec_bytes);
            if n < 0 {
                let magnitude = (-n) as nat;
                udec_canonical(magnitude);
                prefix_before_suffix(
                    bytes,
                    start as int,
                    end as int,
                    seq![0x2du8],
                    ckc_spec::v1text::udec_bytes(magnitude),
                );
                suffix_after_prefix(
                    bytes,
                    start as int,
                    end as int,
                    seq![0x2du8],
                    ckc_spec::v1text::udec_bytes(magnitude),
                );
                assert(start + 1 < end <= bytes.len());
                assert(bytes.subrange(start as int, start as int + 1)[0]
                    == bytes[start as int]);
                assert(bytes.subrange(start as int, start as int + 1)[0] == 0x2d);
                assert(bytes[start as int] == 0x2d);
                assert(bytes.subrange(start as int + 1, end as int)[0]
                    == bytes[start as int + 1]);
                assert(bytes.subrange(start as int + 1, end as int)[0]
                    == ckc_spec::v1text::udec_bytes(magnitude)[0]);
                reveal(canonical_decimal);
                reveal(ckc_spec::v1text::all_in);
                assert(ckc_spec::v1text::is_digit_b(bytes[start as int + 1]));
            } else {
                udec_canonical(n as nat);
                assert(bytes.subrange(start as int, end as int)
                    == ckc_spec::v1text::udec_bytes(n as nat));
                assert(bytes.subrange(start as int, end as int)[0]
                    == bytes[start as int]);
                reveal(canonical_decimal);
                reveal(ckc_spec::v1text::all_in);
                assert(ckc_spec::v1text::is_digit_b(bytes[start as int]));
            }
        },
        Term::Nil => {
            reveal(ckc_spec::v1text::term_bytes);
            reveal_strlit("[]");
            reveal(ckc_spec::v1text::ascii);
            assert(bytes.subrange(start as int, end as int) == seq![0x5bu8, 0x5du8]);
            assert(end == start + 2);
            assert(bytes.subrange(start as int, end as int)[0] == bytes[start as int]);
            assert(bytes.subrange(start as int, end as int)[1] == bytes[start as int + 1]);
        },
        Term::Atom(name) => {
            reveal(ckc_spec::v1text::term_bytes);
            expected_atom_dispatch(bytes, start, name, end);
            if 0x41 <= bytes[start as int] <= 0x5a {
                reveal(ckc_spec::v1text::is_lower_b);
                reveal(ckc_spec::v1text::is_graphic_b);
                reveal(ckc_spec::v1text::solo_bare);
                assert(false);
            }
            if ckc_spec::v1text::is_digit_b(bytes[start as int]) {
                reveal(ckc_spec::v1text::is_digit_b);
                reveal(ckc_spec::v1text::is_lower_b);
                reveal(ckc_spec::v1text::is_graphic_b);
                reveal(ckc_spec::v1text::solo_bare);
                assert(false);
            }
            if bytes[start as int] == 0x5b {
                reveal(ckc_spec::v1text::is_lower_b);
                reveal(ckc_spec::v1text::is_graphic_b);
                reveal(ckc_spec::v1text::solo_bare);
                assert(false);
            }
            if bytes[start as int] == 0x2d
                && start + 1 < bytes.len()
                && ckc_spec::v1text::is_digit_b(bytes[start as int + 1])
            {
                assert(ckc_spec::v1text::is_graphic_b(bytes[start as int])) by {
                    reveal(ckc_spec::v1text::is_graphic_b);
                }
                assert(ckc_spec::v1text::graphic_bare(name));
                reveal(ckc_spec::v1text::atom_bytes);
                reveal(ckc_spec::v1text::graphic_bare);
                reveal(ckc_spec::v1text::all_in);
                assert(name == bytes.subrange(start as int, end as int));
                if start + 1 < end {
                    assert(name[1] == bytes[start as int + 1]);
                    assert(ckc_spec::v1text::is_graphic_b(bytes[start as int + 1]));
                    reveal(ckc_spec::v1text::is_graphic_b);
                    reveal(ckc_spec::v1text::is_digit_b);
                    assert(false);
                } else {
                    assert(start + 1 == end);
                    boundary_stops_digit(bytes, end as int);
                    assert(false);
                }
            }
        },
        Term::Comp(_, _) => assert(false),
    }
}

fn parse_atomic(
    bytes: &[u8],
    start: usize,
    expected: Ghost<Option<GAtomicExpected>>,
at: &mut usize,
) -> (r: Option<ESpannedTerm>)
    requires
        *old(at) <= bytes@.len(),
        start < bytes@.len(),
        expected@ matches Some(e) ==> {
            &&& term_at(bytes@, start as int, e.end as int, e.term)
            &&& atomic_term(e.term)
        },
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r matches Some(t) ==> spanned_term_ok(bytes@, &t),
        r matches Some(t) ==> t.start == start,
        expected@ matches Some(e) ==> r matches Some(t)
            && t@ == e.term && t.end == e.end,
{
    proof {
        if let Some(e) = expected@ {
            expected_atomic_shape(bytes@, start, e.end, e.term);
        }
    }
    if bytes[start] == 0x5b && bytes.len() - start >= 2 && bytes[start + 1] == 0x5d {
        proof {
            if let Some(e) = expected@ {
                match e.term {
                    Term::Nil => {},
                    Term::Var(_) => assert(false),
                    Term::Int(_) => {
                        reveal(ckc_spec::v1text::is_digit_b);
                        assert(false);
                    },
                    Term::Atom(_) => assert(false),
                    Term::Comp(_, _) => assert(false),
                }
            }
        }
        let out = span_nil(bytes, start);
        proof {
            if let Some(e) = expected@ {
                assert(e.term == Term::Nil);
                reveal(term_at);
                assert(out@ == e.term);
                assert(out.end == e.end);
            }
        }
        return Some(out);
    }
    if 0x41 <= bytes[start] && bytes[start] <= 0x5a {
        proof {
            if let Some(e) = expected@ {
                match e.term {
                    Term::Var(_) => {},
                    Term::Int(_) => {
                        reveal(ckc_spec::v1text::is_digit_b);
                        assert(false);
                    },
                    Term::Nil => assert(false),
                    Term::Atom(_) => assert(false),
                    Term::Comp(_, _) => assert(false),
                }
            }
        }
        let ghost expected_var = match expected@ {
            Some(e) => match e.term {
                Term::Var(value) => Some(GVarExpected { value, end: e.end }),
                _ => None,
            },
            None => None,
        };
        return match parse_variable(bytes, start, Ghost(expected_var), at) {
            Some(v) => {
                let out = span_variable(bytes, start, v);
                proof {
                    if let Some(e) = expected@ {
                        match e.term {
                            Term::Var(value) => {
                                assert(v.value@ == value);
                                assert(v.end == e.end);
                                assert(out@ == e.term);
                                assert(out.end == e.end);
                            },
                            _ => assert(false),
                        }
                    }
                }
                Some(out)
            },
            None => None,
        };
    }
    if is_digit_b(bytes[start])
        || bytes[start] == 0x2d
            && start + 1 < bytes.len()
            && is_digit_b(bytes[start + 1])
    {
        proof {
            if let Some(e) = expected@ {
                match e.term {
                    Term::Int(_) => {},
                    Term::Var(_) => {
                        reveal(ckc_spec::v1text::is_digit_b);
                        assert(false);
                    },
                    Term::Nil => {
                        reveal(ckc_spec::v1text::is_digit_b);
                        assert(false);
                    },
                    Term::Atom(_) => assert(false),
                    Term::Comp(_, _) => assert(false),
                }
            }
        }
        let ghost expected_int = match expected@ {
            Some(e) => match e.term {
                Term::Int(value) => Some(GIntExpected { value, end: e.end }),
                _ => None,
            },
            None => None,
        };
        if let Some(n) = parse_integer(bytes, start, Ghost(expected_int), at) {
            let out = span_integer(bytes, start, n);
            proof {
                if let Some(e) = expected@ {
                    match e.term {
                        Term::Int(value) => {
                            assert(n.value@ == value);
                            assert(n.end == e.end);
                            assert(out@ == e.term);
                            assert(out.end == e.end);
                        },
                        _ => assert(false),
                    }
                }
            }
            return Some(out);
        }
    }
    proof {
        if let Some(e) = expected@ {
            match e.term {
                Term::Atom(_) => {},
                Term::Nil => assert(false),
                Term::Var(_) => assert(false),
                Term::Int(_) => assert(false),
                Term::Comp(_, _) => assert(false),
            }
        }
    }
    let ghost expected_atom = match expected@ {
        Some(e) => match e.term {
            Term::Atom(name) => Some(GAtomExpected { name, end: e.end }),
            _ => None,
        },
        None => None,
    };
    match parse_atom(bytes, start, Ghost(expected_atom), at) {
        Some(a) => {
            let out = span_atom(bytes, start, a);
            proof {
                if let Some(e) = expected@ {
                    match e.term {
                        Term::Atom(name) => {
                            assert(a.name@ == name);
                            assert(a.end == e.end);
                            assert(out@ == e.term);
                            assert(out.end == e.end);
                        },
                        _ => assert(false),
                    }
                }
            }
            Some(out)
        },
        None => None,
    }
}

pub open spec fn args_prefix(args: Seq<Term>) -> Seq<u8> {
    if args.len() == 0 {
        Seq::empty()
    } else {
        ckc_spec::v1text::args_bytes(args) + seq![0x2cu8]
    }
}

pub open spec fn list_term(elems: Seq<Term>, tail: Term) -> Term
    decreases elems.len(),
{
    if elems.len() == 0 {
        tail
    } else {
        Term::Comp(
            ckc_spec::v1text::cons_name(),
            seq![elems[0], list_term(elems.drop_first(), tail)],
        )
    }
}

proof fn wf_terms_push(ts: Seq<Term>, t: Term)
    requires
        ckc_spec::term::wf_terms(ts),
        ckc_spec::term::wf_term(t),
    ensures ckc_spec::term::wf_terms(ts.push(t)),
    decreases ts.len(),
{
    if ts.len() == 0 {
        reveal_with_fuel(ckc_spec::term::wf_terms, 2);
    } else {
        wf_terms_push(ts.drop_first(), t);
        assert_seqs_equal!(ts.push(t).drop_first() == ts.drop_first().push(t));
        reveal_with_fuel(ckc_spec::term::wf_terms, 2);
    }
}

proof fn ground_all_push(ts: Seq<Term>, t: Term)
    ensures
        ckc_spec::term::ground_all(ts.push(t))
            == (ckc_spec::term::ground_all(ts) && ckc_spec::term::ground(t)),
    decreases ts.len(),
{
    if ts.len() == 0 {
        reveal_with_fuel(ckc_spec::term::ground_all, 2);
    } else {
        ground_all_push(ts.drop_first(), t);
        assert_seqs_equal!(ts.push(t).drop_first() == ts.drop_first().push(t));
        reveal_with_fuel(ckc_spec::term::ground_all, 2);
    }
}

proof fn no_dollar_all_push(ts: Seq<Term>, t: Term)
    ensures
        ckc_spec::term::no_dollar_var_all(ts.push(t))
            == (ckc_spec::term::no_dollar_var_all(ts) && ckc_spec::term::no_dollar_var(t)),
    decreases ts.len(),
{
    if ts.len() == 0 {
        reveal_with_fuel(ckc_spec::term::no_dollar_var_all, 2);
    } else {
        no_dollar_all_push(ts.drop_first(), t);
        assert_seqs_equal!(ts.push(t).drop_first() == ts.drop_first().push(t));
        reveal_with_fuel(ckc_spec::term::no_dollar_var_all, 2);
    }
}

proof fn args_bytes_push(ts: Seq<Term>, t: Term)
    ensures
        ckc_spec::v1text::args_bytes(ts.push(t)) == if ts.len() == 0 {
            ckc_spec::v1text::term_bytes(t)
        } else {
            ckc_spec::v1text::args_bytes(ts) + seq![0x2cu8]
                + ckc_spec::v1text::term_bytes(t)
        },
    decreases ts.len(),
{
    if ts.len() == 0 {
        reveal_with_fuel(ckc_spec::v1text::args_bytes, 2);
    } else if ts.len() == 1 {
        assert_seqs_equal!(ts.push(t).drop_first() == seq![t]);
        reveal_with_fuel(ckc_spec::v1text::args_bytes, 3);
    } else {
        args_bytes_push(ts.drop_first(), t);
        assert_seqs_equal!(ts.push(t).drop_first() == ts.drop_first().push(t));
        reveal_with_fuel(ckc_spec::v1text::args_bytes, 2);
    }
}

proof fn wf_list_term(elems: Seq<Term>, tail: Term)
    requires
        ckc_spec::term::wf_terms(elems),
        ckc_spec::term::wf_term(tail),
    ensures ckc_spec::term::wf_term(list_term(elems, tail)),
    decreases elems.len(),
{
    if elems.len() == 0 {
        reveal_with_fuel(list_term, 2);
        assert(list_term(elems, tail) == tail);
    } else {
        reveal_with_fuel(ckc_spec::term::wf_terms, 2);
        assert(ckc_spec::term::wf_term(elems[0]));
        assert(ckc_spec::term::wf_terms(elems.drop_first()));
        wf_list_term(elems.drop_first(), tail);
        let rest = list_term(elems.drop_first(), tail);
        assert(ckc_spec::term::wf_term(rest));
        assert(ckc_spec::term::wf_terms(seq![elems[0], rest])) by {
            reveal_with_fuel(ckc_spec::term::wf_terms, 3);
        }
        reveal_with_fuel(list_term, 2);
        reveal_with_fuel(ckc_spec::term::wf_term, 2);
    }
}

proof fn ground_list_term(elems: Seq<Term>, tail: Term)
    ensures
        ckc_spec::term::ground(list_term(elems, tail))
            == (ckc_spec::term::ground_all(elems) && ckc_spec::term::ground(tail)),
    decreases elems.len(),
{
    if elems.len() == 0 {
        reveal_with_fuel(list_term, 2);
        reveal_with_fuel(ckc_spec::term::ground_all, 2);
        assert(list_term(elems, tail) == tail);
    } else {
        ground_list_term(elems.drop_first(), tail);
        let rest = list_term(elems.drop_first(), tail);
        assert(ckc_spec::term::ground(rest)
            == (ckc_spec::term::ground_all(elems.drop_first())
                && ckc_spec::term::ground(tail)));
        assert(ckc_spec::term::ground_all(elems)
            == (ckc_spec::term::ground(elems[0])
                && ckc_spec::term::ground_all(elems.drop_first()))) by {
            reveal_with_fuel(ckc_spec::term::ground_all, 2);
        }
        assert(ckc_spec::term::ground_all(seq![elems[0], rest])
            == (ckc_spec::term::ground(elems[0]) && ckc_spec::term::ground(rest))) by {
            reveal_with_fuel(ckc_spec::term::ground_all, 3);
        }
        reveal_with_fuel(list_term, 2);
        reveal_with_fuel(ckc_spec::term::ground, 2);
    }
}

proof fn no_dollar_list_term(elems: Seq<Term>, tail: Term)
    ensures
        ckc_spec::term::no_dollar_var(list_term(elems, tail))
            == (ckc_spec::term::no_dollar_var_all(elems)
                && ckc_spec::term::no_dollar_var(tail)),
    decreases elems.len(),
{
    if elems.len() == 0 {
        reveal_with_fuel(list_term, 2);
        reveal_with_fuel(ckc_spec::term::no_dollar_var_all, 2);
        assert(list_term(elems, tail) == tail);
    } else {
        no_dollar_list_term(elems.drop_first(), tail);
        let rest = list_term(elems.drop_first(), tail);
        assert(ckc_spec::term::no_dollar_var(rest)
            == (ckc_spec::term::no_dollar_var_all(elems.drop_first())
                && ckc_spec::term::no_dollar_var(tail)));
        assert(ckc_spec::term::no_dollar_var_all(elems)
            == (ckc_spec::term::no_dollar_var(elems[0])
                && ckc_spec::term::no_dollar_var_all(elems.drop_first()))) by {
            reveal_with_fuel(ckc_spec::term::no_dollar_var_all, 2);
        }
        assert(ckc_spec::term::no_dollar_var_all(seq![elems[0], rest])
            == (ckc_spec::term::no_dollar_var(elems[0])
                && ckc_spec::term::no_dollar_var(rest))) by {
            reveal_with_fuel(ckc_spec::term::no_dollar_var_all, 3);
        }
        reveal(ckc_spec::v1text::cons_name);
        reveal(ckc_spec::term::dollar_var_name);
        assert(ckc_spec::v1text::cons_name() != ckc_spec::term::dollar_var_name());
        reveal_with_fuel(list_term, 2);
        reveal_with_fuel(ckc_spec::term::no_dollar_var, 2);
    }
}

proof fn list_nil_tail_bytes(elems: Seq<Term>)
    ensures
        ckc_spec::v1text::tail_bytes(list_term(elems, Term::Nil)) == if elems.len() == 0 {
            Seq::empty()
        } else {
            seq![0x2cu8] + ckc_spec::v1text::args_bytes(elems)
        },
    decreases elems.len(),
{
    if elems.len() == 0 {
        reveal_with_fuel(list_term, 2);
        reveal_with_fuel(ckc_spec::v1text::tail_bytes, 2);
    } else {
        list_nil_tail_bytes(elems.drop_first());
        reveal(ckc_spec::v1text::cons_name);
        reveal_with_fuel(list_term, 2);
        reveal_with_fuel(ckc_spec::v1text::tail_bytes, 2);
        reveal_with_fuel(ckc_spec::v1text::args_bytes, 2);
        if elems.len() == 1 {
            assert(elems.drop_first().len() == 0);
        }
    }
}

proof fn list_nil_bytes(elems: Seq<Term>)
    requires elems.len() > 0,
    ensures
        ckc_spec::v1text::term_bytes(list_term(elems, Term::Nil))
            == seq![0x5bu8] + ckc_spec::v1text::args_bytes(elems) + seq![0x5du8],
{
    list_nil_tail_bytes(elems.drop_first());
    reveal(ckc_spec::v1text::cons_name);
    reveal_with_fuel(list_term, 2);
    reveal_with_fuel(ckc_spec::v1text::term_bytes, 2);
    reveal_with_fuel(ckc_spec::v1text::args_bytes, 2);
}

pub open spec fn plain_list_tail(t: Term) -> bool {
    match t {
        Term::Nil => false,
        Term::Comp(name, args) => !(name == ckc_spec::v1text::cons_name() && args.len() == 2),
        _ => true,
    }
}

pub open spec fn list_cons(t: Term) -> bool {
    match t {
        Term::Comp(name, args) => name == ckc_spec::v1text::cons_name() && args.len() == 2,
        _ => false,
    }
}

pub open spec fn list_elems(t: Term) -> Seq<Term>
    decreases t,
{
    match t {
        Term::Comp(name, args) if name == ckc_spec::v1text::cons_name() && args.len() == 2 => {
            seq![args[0]] + list_elems(args[1])
        },
        _ => Seq::empty(),
    }
}

pub open spec fn list_final_tail(t: Term) -> Term
    decreases t,
{
    match t {
        Term::Comp(name, args) if name == ckc_spec::v1text::cons_name() && args.len() == 2 => {
            list_final_tail(args[1])
        },
        _ => t,
    }
}

proof fn wf_terms_prepend(term: Term, terms: Seq<Term>)
    requires
        ckc_spec::term::wf_term(term),
        ckc_spec::term::wf_terms(terms),
    ensures ckc_spec::term::wf_terms(seq![term] + terms),
{
    assert((seq![term] + terms)[0] == term);
    assert_seqs_equal!((seq![term] + terms).drop_first() == terms);
    reveal_with_fuel(ckc_spec::term::wf_terms, 2);
}

proof fn list_decompose(term: Term)
    requires
        list_cons(term),
        ckc_spec::term::wf_term(term),
    ensures
        list_elems(term).len() > 0,
        list_term(list_elems(term), list_final_tail(term)) == term,
        ckc_spec::term::wf_terms(list_elems(term)),
        ckc_spec::term::wf_term(list_final_tail(term)),
        list_final_tail(term) == Term::Nil || plain_list_tail(list_final_tail(term)),
    decreases term,
{
    reveal(list_cons);
    match term {
        Term::Comp(name, args) => {
            assert(name == ckc_spec::v1text::cons_name());
            assert(args.len() == 2);
            reveal_with_fuel(ckc_spec::term::wf_term, 2);
            reveal_with_fuel(ckc_spec::term::wf_terms, 3);
            assert(ckc_spec::term::wf_term(args[0]));
            assert(ckc_spec::term::wf_term(args[1]));
            assert_seqs_equal!(args == seq![args[0], args[1]]);
            if list_cons(args[1]) {
                list_decompose(args[1]);
                wf_terms_prepend(args[0], list_elems(args[1]));
            } else {
                reveal(list_cons);
                reveal(plain_list_tail);
                match args[1] {
                    Term::Nil => {},
                    _ => assert(plain_list_tail(args[1])),
                }
                reveal_with_fuel(list_elems, 2);
                reveal_with_fuel(list_final_tail, 2);
                reveal_with_fuel(list_term, 2);
                reveal_with_fuel(ckc_spec::term::wf_terms, 2);
            }
            let elems = seq![args[0]] + list_elems(args[1]);
            let tail = list_final_tail(args[1]);
            assert(elems.len() > 0);
            assert(elems[0] == args[0]);
            assert_seqs_equal!(elems.drop_first() == list_elems(args[1]));
            assert(list_term(list_elems(args[1]), tail) == args[1]);
            reveal_with_fuel(list_elems, 2);
            reveal_with_fuel(list_final_tail, 2);
            reveal_with_fuel(list_term, 2);
            assert(list_term(elems, tail)
                == Term::Comp(
                    ckc_spec::v1text::cons_name(),
                    seq![args[0], list_term(list_elems(args[1]), tail)],
                ));
            assert(list_term(elems, tail)
                == Term::Comp(ckc_spec::v1text::cons_name(), seq![args[0], args[1]]));
        },
        _ => assert(false),
    }
}

proof fn list_plain_tail_bytes(elems: Seq<Term>, tail: Term)
    requires plain_list_tail(tail),
    ensures
        ckc_spec::v1text::tail_bytes(list_term(elems, tail)) == if elems.len() == 0 {
            seq![0x7cu8] + ckc_spec::v1text::term_bytes(tail)
        } else {
            seq![0x2cu8] + ckc_spec::v1text::args_bytes(elems) + seq![0x7cu8]
                + ckc_spec::v1text::term_bytes(tail)
        },
    decreases elems.len(),
{
    if elems.len() == 0 {
        reveal_with_fuel(list_term, 2);
        reveal(plain_list_tail);
        reveal_with_fuel(ckc_spec::v1text::tail_bytes, 2);
    } else {
        list_plain_tail_bytes(elems.drop_first(), tail);
        reveal(ckc_spec::v1text::cons_name);
        reveal_with_fuel(list_term, 2);
        reveal_with_fuel(ckc_spec::v1text::tail_bytes, 2);
        reveal_with_fuel(ckc_spec::v1text::args_bytes, 2);
        if elems.len() == 1 {
            assert(elems.drop_first().len() == 0);
        }
    }
}

proof fn list_plain_bytes(elems: Seq<Term>, tail: Term)
    requires
        elems.len() > 0,
        plain_list_tail(tail),
    ensures
        ckc_spec::v1text::term_bytes(list_term(elems, tail))
            == seq![0x5bu8] + ckc_spec::v1text::args_bytes(elems) + seq![0x7cu8]
                + ckc_spec::v1text::term_bytes(tail) + seq![0x5du8],
{
    list_plain_tail_bytes(elems.drop_first(), tail);
    reveal(ckc_spec::v1text::cons_name);
    reveal_with_fuel(list_term, 2);
    reveal_with_fuel(ckc_spec::v1text::term_bytes, 2);
    reveal_with_fuel(ckc_spec::v1text::args_bytes, 2);
}

pub enum ETermFrame {
    Comp {
        start: usize,
        child_start: usize,
        name: Vec<u8>,
        args: Ghost<Seq<Term>>,
        count: usize,
        ground: bool,
        no_dollar: bool,
    },
    List {
        start: usize,
        child_start: usize,
        elems: Ghost<Seq<Term>>,
        count: usize,
        ground: bool,
        no_dollar: bool,
        tail: bool,
    },
    Curly {
        start: usize,
        child_start: usize,
    },
}

pub open spec fn frame_start(f: &ETermFrame) -> usize {
    match f {
        ETermFrame::Comp { start, .. } => *start,
        ETermFrame::List { start, .. } => *start,
        ETermFrame::Curly { start, .. } => *start,
    }
}

pub open spec fn frame_child_start(f: &ETermFrame) -> usize {
    match f {
        ETermFrame::Comp { child_start, .. } => *child_start,
        ETermFrame::List { child_start, .. } => *child_start,
        ETermFrame::Curly { child_start, .. } => *child_start,
    }
}

fn frame_child_start_exec(f: &ETermFrame) -> (r: usize)
    ensures r == frame_child_start(f),
{
    match f {
        ETermFrame::Comp { child_start, .. } => *child_start,
        ETermFrame::List { child_start, .. } => *child_start,
        ETermFrame::Curly { child_start, .. } => *child_start,
    }
}

pub open spec fn frame_ok(bytes: Seq<u8>, f: &ETermFrame) -> bool {
    match f {
        ETermFrame::Comp {
            start, child_start, name, args, count, ground, no_dollar,
        } => {
            &&& *start < *child_start <= bytes.len()
            &&& *count == args@.len()
            &&& *count < *child_start - *start
            &&& ckc_spec::term::wf_terms(args@)
            &&& *ground == ckc_spec::term::ground_all(args@)
            &&& *no_dollar == ckc_spec::term::no_dollar_var_all(args@)
            &&& bytes.subrange(*start as int, *child_start as int)
                == ckc_spec::v1text::atom_bytes(name@) + seq![0x28u8]
                    + args_prefix(args@)
        },
        ETermFrame::List {
            start, child_start, elems, count, ground, no_dollar, tail,
        } => {
            &&& *start < *child_start <= bytes.len()
            &&& *count == elems@.len()
            &&& *count < *child_start - *start
            &&& ckc_spec::term::wf_terms(elems@)
            &&& *ground == ckc_spec::term::ground_all(elems@)
            &&& *no_dollar == ckc_spec::term::no_dollar_var_all(elems@)
            &&& (*tail ==> elems@.len() > 0)
            &&& bytes.subrange(*start as int, *child_start as int)
                == seq![0x5bu8] + if *tail {
                    ckc_spec::v1text::args_bytes(elems@) + seq![0x7cu8]
                } else {
                    args_prefix(elems@)
                }
        },
        ETermFrame::Curly { start, child_start } => {
            &&& *start < *child_start <= bytes.len()
            &&& *child_start == *start + 1
            &&& bytes[*start as int] == 0x7b
        },
    }
}

proof fn seq_eq_three(s: Seq<u8>, x: u8, y: u8, z: u8)
    ensures s == seq![x, y, z] <==> s.len() == 3 && s[0] == x && s[1] == y && s[2] == z,
{
    if s.len() == 3 && s[0] == x && s[1] == y && s[2] == z {
        assert_seqs_equal!(s == seq![x, y, z]);
    }
}

proof fn seq_eq_four(s: Seq<u8>, a: u8, b: u8, c: u8, d: u8)
    ensures s == seq![a, b, c, d]
        <==> s.len() == 4 && s[0] == a && s[1] == b && s[2] == c && s[3] == d,
{
    if s.len() == 4 && s[0] == a && s[1] == b && s[2] == c && s[3] == d {
        assert_seqs_equal!(s == seq![a, b, c, d]);
    }
}

fn is_dollar_name(name: &[u8]) -> (r: bool)
    ensures r == (name@ == ckc_spec::term::dollar_var_name()),
{
    let r = name.len() == 4 && name[0] == 0x24 && name[1] == 0x56
        && name[2] == 0x41 && name[3] == 0x52;
    proof {
        seq_eq_four(name@, 0x24, 0x56, 0x41, 0x52);
        reveal(ckc_spec::term::dollar_var_name);
    }
    r
}

fn is_cons_name(name: &[u8]) -> (r: bool)
    ensures r == (name@ == ckc_spec::v1text::cons_name()),
{
    let r = name.len() == 3 && name[0] == 0x5b && name[1] == 0x7c && name[2] == 0x5d;
    proof {
        seq_eq_three(name@, 0x5b, 0x7c, 0x5d);
        reveal(ckc_spec::v1text::cons_name);
    }
    r
}

fn is_curly_name(name: &[u8]) -> (r: bool)
    ensures r == (name@ == ckc_spec::v1text::curly_name()),
{
    let r = name.len() == 2 && name[0] == 0x7b && name[1] == 0x7d;
    proof {
        seq_eq_two(name@, 0x7b, 0x7d);
        reveal(ckc_spec::v1text::curly_name);
    }
    r
}

fn cons_name_vec() -> (out: Vec<u8>)
    ensures out@ == ckc_spec::v1text::cons_name(),
{
    let mut out = Vec::new();
    out.push(0x5b);
    out.push(0x7c);
    out.push(0x5d);
    proof { reveal(ckc_spec::v1text::cons_name); }
    out
}

fn curly_name_vec() -> (out: Vec<u8>)
    ensures out@ == ckc_spec::v1text::curly_name(),
{
    let mut out = Vec::new();
    out.push(0x7b);
    out.push(0x7d);
    proof { reveal(ckc_spec::v1text::curly_name); }
    out
}

fn open_comp_frame(bytes: &[u8], start: usize, atom: EParsedAtom) -> (out: ETermFrame)
    requires
        parsed_atom_ok(bytes@, start, &atom),
        atom.end < bytes@.len(),
        atom.end < usize::MAX,
        bytes@[atom.end as int] == 0x28,
    ensures
        frame_ok(bytes@, &out),
        frame_start(&out) == start,
        frame_child_start(&out) == atom.end + 1,
        match &out {
            ETermFrame::Comp {
                start: s,
                child_start,
                name,
                args,
                count,
                ground,
                no_dollar,
            } => {
                &&& *s == start
                &&& *child_start == atom.end + 1
                &&& name@ == atom.name@
                &&& args@ == Seq::<Term>::empty()
                &&& *count == 0
                &&& *ground
                &&& *no_dollar
            },
            _ => false,
        },
{
    let child_start = atom.end + 1;
    proof {
        reveal(args_prefix);
        reveal_with_fuel(ckc_spec::term::wf_terms, 2);
        reveal_with_fuel(ckc_spec::term::ground_all, 2);
        reveal_with_fuel(ckc_spec::term::no_dollar_var_all, 2);
        assert_seqs_equal!(bytes@.subrange(start as int, child_start as int)
            == bytes@.subrange(start as int, atom.end as int) + seq![0x28u8]);
        reveal(frame_ok);
        reveal(frame_start);
        reveal(frame_child_start);
    }
    ETermFrame::Comp {
        start,
        child_start,
        name: atom.name,
        args: Ghost(Seq::empty()),
        count: 0,
        ground: true,
        no_dollar: true,
    }
}

fn open_list_frame(bytes: &[u8], start: usize) -> (out: ETermFrame)
    requires
        start < bytes@.len(),
        start < usize::MAX,
        bytes@[start as int] == 0x5b,
    ensures
        frame_ok(bytes@, &out),
        frame_start(&out) == start,
        frame_child_start(&out) == start + 1,
        match &out {
            ETermFrame::List {
                start: s,
                child_start,
                elems,
                count,
                ground,
                no_dollar,
                tail,
            } => {
                &&& *s == start
                &&& *child_start == start + 1
                &&& elems@ == Seq::<Term>::empty()
                &&& *count == 0
                &&& *ground
                &&& *no_dollar
                &&& !*tail
            },
            _ => false,
        },
{
    let child_start = start + 1;
    proof {
        reveal(args_prefix);
        reveal_with_fuel(ckc_spec::term::wf_terms, 2);
        reveal_with_fuel(ckc_spec::term::ground_all, 2);
        reveal_with_fuel(ckc_spec::term::no_dollar_var_all, 2);
        assert_seqs_equal!(bytes@.subrange(start as int, child_start as int) == seq![0x5bu8]);
        reveal(frame_ok);
        reveal(frame_start);
        reveal(frame_child_start);
    }
    ETermFrame::List {
        start,
        child_start,
        elems: Ghost(Seq::empty()),
        count: 0,
        ground: true,
        no_dollar: true,
        tail: false,
    }
}

fn open_curly_frame(bytes: &[u8], start: usize) -> (out: ETermFrame)
    requires
        start < bytes@.len(),
        start < usize::MAX,
        bytes@[start as int] == 0x7b,
    ensures
        frame_ok(bytes@, &out),
        frame_start(&out) == start,
        frame_child_start(&out) == start + 1,
        match &out {
            ETermFrame::Curly { start: s, child_start } => {
                &&& *s == start
                &&& *child_start == start + 1
            },
            _ => false,
        },
{
    let child_start = start + 1;
    proof {
        reveal(frame_ok);
        reveal(frame_start);
        reveal(frame_child_start);
    }
    ETermFrame::Curly { start, child_start }
}

pub enum EFrameStep {
    Next(ETermFrame),
    Done(ESpannedTerm),
    Reject,
}

pub open spec fn frame_step_ok(
    bytes: Seq<u8>,
    start: usize,
    child_end: usize,
    step: &EFrameStep,
) -> bool {
    match step {
        EFrameStep::Next(next) => {
            &&& frame_ok(bytes, next)
            &&& frame_start(next) == start
            &&& frame_child_start(next) == child_end + 1
        },
        EFrameStep::Done(term) => {
            &&& spanned_term_ok(bytes, term)
            &&& term.start == start
            &&& term.end == child_end + 1
        },
        EFrameStep::Reject => true,
    }
}

pub open spec fn comp_frame_ok(
    bytes: Seq<u8>,
    start: usize,
    child_start: usize,
    name: Seq<u8>,
    args: Seq<Term>,
    count: usize,
    ground: bool,
    no_dollar: bool,
) -> bool {
    &&& start < child_start <= bytes.len()
    &&& count == args.len()
    &&& count < child_start - start
    &&& ckc_spec::term::wf_terms(args)
    &&& ground == ckc_spec::term::ground_all(args)
    &&& no_dollar == ckc_spec::term::no_dollar_var_all(args)
    &&& bytes.subrange(start as int, child_start as int)
        == ckc_spec::v1text::atom_bytes(name) + seq![0x28u8] + args_prefix(args)
}

fn feed_comp_frame(
    bytes: &[u8],
    start: usize,
    child_start: usize,
    name: Vec<u8>,
    args: Ghost<Seq<Term>>,
    count: usize,
    ground: bool,
    no_dollar: bool,
    child: ESpannedTerm,
    guide: Ghost<Option<GTermFrame>>,
) -> (out: EFrameStep)
    requires
        comp_frame_ok(bytes@, start, child_start, name@, args@, count, ground, no_dollar),
        spanned_term_ok(bytes@, &child),
        child.start == child_start,
        guide@ matches Some(g) ==> guided_step_pre(
            bytes@,
            &ETermFrame::Comp {
                start,
                child_start,
                name,
                args,
                count,
                ground,
                no_dollar,
            },
            &child,
            g,
        ),
    ensures
        frame_step_ok(bytes@, start, child.end, &out),
        guide@ matches Some(g) ==> guided_step_ok(
            bytes@,
            &ETermFrame::Comp {
                start,
                child_start,
                name,
                args,
                count,
                ground,
                no_dollar,
            },
            &child,
            g,
            &out,
        ),
{
    proof {
        if let Some(g) = guide@ {
            guided_delimiter_at(
                bytes@,
                &ETermFrame::Comp {
                    start,
                    child_start,
                    name,
                    args,
                    count,
                    ground,
                    no_dollar,
                },
                &child,
                g,
            );
            reveal(guided_step_pre);
            reveal(guide_frame_ok);
            reveal(guide_delimiter);
        }
    }
    if child.end == bytes.len() {
        return EFrameStep::Reject;
    }
    let delimiter = bytes[child.end];
    let ghost next_args = args@.push(child@);
    let next_count = count + 1;
    proof {
        assert(count < usize::MAX);
        wf_terms_push(args@, child@);
        ground_all_push(args@, child@);
        no_dollar_all_push(args@, child@);
        args_bytes_push(args@, child@);
    }
    if delimiter == 0x2c {
        let next_child_start = child.end + 1;
        let next = ETermFrame::Comp {
            start,
            child_start: next_child_start,
            name,
            args: Ghost(next_args),
            count: next_count,
            ground: ground && child.parsed.ground,
            no_dollar: no_dollar && child.parsed.no_dollar,
        };
        proof {
            assert_seqs_equal!(bytes@.subrange(start as int, next_child_start as int)
                == bytes@.subrange(start as int, child_start as int)
                    + bytes@.subrange(child_start as int, child.end as int)
                    + seq![0x2cu8]);
            reveal(args_prefix);
            reveal(comp_frame_ok);
            assert(next_count == next_args.len());
            assert(next_count < next_child_start - start);
            assert(ckc_spec::term::wf_terms(next_args));
            assert((ground && child.parsed.ground) == ckc_spec::term::ground_all(next_args));
            assert((no_dollar && child.parsed.no_dollar)
                == ckc_spec::term::no_dollar_var_all(next_args));
            assert(bytes@.subrange(start as int, next_child_start as int)
                == ckc_spec::v1text::atom_bytes(name@) + seq![0x28u8]
                    + args_prefix(next_args));
            reveal(frame_ok);
            reveal(frame_start);
            reveal(frame_child_start);
            reveal(frame_step_ok);
            assert(frame_ok(bytes@, &next));
            assert(frame_start(&next) == start);
            assert(frame_child_start(&next) == child.end + 1);
            if let Some(g) = guide@ {
                reveal(guided_step_ok);
                reveal(guided_step_pre);
                reveal(guide_frame_ok);
                reveal(guide_delimiter);
                match g {
                    GTermFrame::Comp {
                        name: gname,
                        built,
                        remaining,
                        end: guide_end,
                    } => {
                        assert(remaining.len() > 1);
                        assert(child@ == remaining[0]);
                        assert(args@ == built);
                        assert(name@ == gname);
                        assert(next_args == built.push(remaining[0]));
                        wf_terms_drop_first(remaining);
                        assert_seqs_equal!(built.push(remaining[0]) + remaining.drop_first()
                            == built + remaining);
                        reveal_with_fuel(ckc_spec::v1text::args_bytes, 2);
                        assert_seqs_equal!(bytes@.subrange(child_start as int, guide_end)
                            == ckc_spec::v1text::term_bytes(child@) + seq![0x2cu8]
                                + (ckc_spec::v1text::args_bytes(remaining.drop_first())
                                    + seq![0x29u8]));
                        suffix_after_term_delimiter(
                            bytes@,
                            child_start as int,
                            guide_end,
                            child@,
                            0x2c,
                            ckc_spec::v1text::args_bytes(remaining.drop_first())
                                + seq![0x29u8],
                        );
                        spanned_term_length(bytes@, &child);
                        assert(next_child_start as int == child_start as int
                            + ckc_spec::v1text::term_bytes(child@).len() + 1);
                        assert(bytes@.subrange(next_child_start as int, guide_end)
                            == ckc_spec::v1text::args_bytes(remaining.drop_first())
                                + seq![0x29u8]);
                        assert((next_child_start as int) < guide_end);
                        assert(guide_frame_ok(
                            bytes@,
                            &next,
                            GTermFrame::Comp {
                                name: gname,
                                built: built.push(remaining[0]),
                                remaining: remaining.drop_first(),
                                end: guide_end,
                            },
                        ));
                    },
                    _ => assert(false),
                }
            }
        }
        return EFrameStep::Next(next);
    }
    if delimiter != 0x29 {
        return EFrameStep::Reject;
    }
    let special = (next_count == 2 && is_cons_name(name.as_slice()))
        || (next_count == 1 && is_curly_name(name.as_slice()));
    if special {
        return EFrameStep::Reject;
    }
    let dollar = next_count == 1 && is_dollar_name(name.as_slice());
    let out_ground = ground && child.parsed.ground;
    let out_no_dollar = no_dollar && child.parsed.no_dollar && !dollar;
    let end = child.end + 1;
    let ghost term = Term::Comp(name@, next_args);
    proof {
        assert_seqs_equal!(bytes@.subrange(start as int, end as int)
            == bytes@.subrange(start as int, child_start as int)
                + bytes@.subrange(child_start as int, child.end as int)
                + seq![0x29u8]);
        reveal(args_prefix);
        reveal(comp_frame_ok);
        reveal_with_fuel(ckc_spec::term::wf_term, 2);
        reveal_with_fuel(ckc_spec::term::ground, 2);
        reveal_with_fuel(ckc_spec::term::no_dollar_var, 2);
        reveal_with_fuel(ckc_spec::v1text::term_bytes, 2);
        reveal(top_matches);
        reveal(parsed_term_ok);
        reveal(spanned_term_ok);
        assert(ckc_spec::term::wf_term(term));
        assert(out_ground == ckc_spec::term::ground(term));
        assert(out_no_dollar == ckc_spec::term::no_dollar_var(term));
        assert(ckc_spec::v1text::term_bytes(term)
            == bytes@.subrange(start as int, end as int));
        reveal(frame_step_ok);
    }
    let parsed = EParsedTerm {
        term: Ghost(term),
        top: ETermTop::Comp(name, next_count),
        ground: out_ground,
        no_dollar: out_no_dollar,
    };
    let done = ESpannedTerm { parsed, start, end };
    proof {
        assert(parsed_term_ok(&done.parsed));
        assert(ckc_spec::v1text::term_bytes(done@)
            == bytes@.subrange(done.start as int, done.end as int));
        assert(spanned_term_ok(bytes@, &done));
        assert(frame_step_ok(bytes@, start, child.end, &EFrameStep::Done(done)));
        if let Some(g) = guide@ {
            reveal(guided_step_ok);
            reveal(guided_step_pre);
            reveal(guide_frame_ok);
            reveal(guide_target);
            reveal(guide_end);
            reveal(guide_delimiter);
            match g {
                GTermFrame::Comp {
                    name: gname,
                    built,
                    remaining,
                    end: guide_end,
                } => {
                    assert(remaining.len() == 1);
                    assert(child@ == remaining[0]);
                    assert(args@ == built);
                    assert(name@ == gname);
                    assert_seqs_equal!(remaining == seq![remaining[0]]);
                    assert_seqs_equal!(built + remaining == built.push(remaining[0]));
                    assert(next_args == built.push(remaining[0]));
                    assert(term == guide_target(g));
                    assert(bytes@.subrange(child_start as int, guide_end).len()
                        == guide_end - child_start);
                    spanned_term_length(bytes@, &child);
                    assert(guide_end == child.end as int + 1);
                    assert(done@ == guide_target(g));
                    assert(done.end as int == guide_end);
                },
                _ => assert(false),
            }
        }
    }
    EFrameStep::Done(done)
}

pub open spec fn list_frame_ok(
    bytes: Seq<u8>,
    start: usize,
    child_start: usize,
    elems: Seq<Term>,
    count: usize,
    ground: bool,
    no_dollar: bool,
    tail: bool,
) -> bool {
    &&& start < child_start <= bytes.len()
    &&& count == elems.len()
    &&& count < child_start - start
    &&& ckc_spec::term::wf_terms(elems)
    &&& ground == ckc_spec::term::ground_all(elems)
    &&& no_dollar == ckc_spec::term::no_dollar_var_all(elems)
    &&& (tail ==> elems.len() > 0)
    &&& bytes.subrange(start as int, child_start as int)
        == seq![0x5bu8] + if tail {
            ckc_spec::v1text::args_bytes(elems) + seq![0x7cu8]
        } else {
            args_prefix(elems)
        }
}

fn is_plain_tail(term: &EParsedTerm) -> (r: bool)
    requires parsed_term_ok(term),
    ensures r == plain_list_tail(term@),
{
    let out = match &term.top {
        ETermTop::Nil => false,
        ETermTop::Comp(name, arity) => !(*arity == 2 && is_cons_name(name.as_slice())),
        _ => true,
    };
    proof {
        reveal(parsed_term_ok);
        reveal(top_matches);
        reveal(plain_list_tail);
    }
    out
}

fn finish_list_term(
    bytes: &[u8],
    start: usize,
    end: usize,
    elems: Ghost<Seq<Term>>,
    tail: Ghost<Term>,
    ground: bool,
    no_dollar: bool,
) -> (out: ESpannedTerm)
    requires
        start < end <= bytes@.len(),
        elems@.len() > 0,
        ckc_spec::term::wf_terms(elems@),
        ckc_spec::term::wf_term(tail@),
        ground == (ckc_spec::term::ground_all(elems@) && ckc_spec::term::ground(tail@)),
        no_dollar == (ckc_spec::term::no_dollar_var_all(elems@)
            && ckc_spec::term::no_dollar_var(tail@)),
        ckc_spec::v1text::term_bytes(list_term(elems@, tail@))
            == bytes@.subrange(start as int, end as int),
    ensures
        spanned_term_ok(bytes@, &out),
        out@ == list_term(elems@, tail@),
        out.start == start,
        out.end == end,
{
    let name = cons_name_vec();
    let ghost term = list_term(elems@, tail@);
    proof {
        wf_list_term(elems@, tail@);
        ground_list_term(elems@, tail@);
        no_dollar_list_term(elems@, tail@);
        reveal_with_fuel(list_term, 2);
        reveal(top_matches);
        reveal(parsed_term_ok);
        reveal(spanned_term_ok);
    }
    ESpannedTerm {
        parsed: EParsedTerm {
            term: Ghost(term),
            top: ETermTop::Comp(name, 2),
            ground,
            no_dollar,
        },
        start,
        end,
    }
}

#[verifier::rlimit(100)]
fn feed_list_frame(
    bytes: &[u8],
    start: usize,
    child_start: usize,
    elems: Ghost<Seq<Term>>,
    count: usize,
    ground: bool,
    no_dollar: bool,
    tail_mode: bool,
    child: ESpannedTerm,
    guide: Ghost<Option<GTermFrame>>,
) -> (out: EFrameStep)
    requires
        list_frame_ok(bytes@, start, child_start, elems@, count, ground, no_dollar, tail_mode),
        spanned_term_ok(bytes@, &child),
        child.start == child_start,
        guide@ matches Some(g) ==> guided_step_pre(
            bytes@,
            &ETermFrame::List {
                start,
                child_start,
                elems,
                count,
                ground,
                no_dollar,
                tail: tail_mode,
            },
            &child,
            g,
        ),
    ensures
        frame_step_ok(bytes@, start, child.end, &out),
        guide@ matches Some(g) ==> guided_step_ok(
            bytes@,
            &ETermFrame::List {
                start,
                child_start,
                elems,
                count,
                ground,
                no_dollar,
                tail: tail_mode,
            },
            &child,
            g,
            &out,
        ),
{
    proof {
        if let Some(g) = guide@ {
            guided_delimiter_at(
                bytes@,
                &ETermFrame::List {
                    start,
                    child_start,
                    elems,
                    count,
                    ground,
                    no_dollar,
                    tail: tail_mode,
                },
                &child,
                g,
            );
            reveal(guided_step_pre);
            reveal(guide_frame_ok);
            reveal(guide_delimiter);
        }
    }
    if child.end == bytes.len() {
        return EFrameStep::Reject;
    }
    let delimiter = bytes[child.end];
    if tail_mode {
        if delimiter != 0x5d || !is_plain_tail(&child.parsed) {
            return EFrameStep::Reject;
        }
        let end = child.end + 1;
        let out_ground = ground && child.parsed.ground;
        let out_no_dollar = no_dollar && child.parsed.no_dollar;
        proof {
            list_plain_bytes(elems@, child@);
            assert_seqs_equal!(bytes@.subrange(start as int, end as int)
                == bytes@.subrange(start as int, child_start as int)
                    + bytes@.subrange(child_start as int, child.end as int)
                    + seq![0x5du8]);
            reveal(list_frame_ok);
            reveal(spanned_term_ok);
            assert(bytes@.subrange(start as int, child_start as int)
                == seq![0x5bu8] + ckc_spec::v1text::args_bytes(elems@) + seq![0x7cu8]);
            assert(ckc_spec::v1text::term_bytes(child@)
                == bytes@.subrange(child_start as int, child.end as int));
            assert(ckc_spec::v1text::term_bytes(list_term(elems@, child@))
                == bytes@.subrange(start as int, end as int));
        }
        let done = finish_list_term(
            bytes,
            start,
            end,
            elems,
            Ghost(child@),
            out_ground,
            out_no_dollar,
        );
        proof {
            reveal(frame_step_ok);
            assert(spanned_term_ok(bytes@, &done));
            assert(done.start == start);
            assert(done.end == child.end + 1);
            assert(frame_step_ok(bytes@, start, child.end, &EFrameStep::Done(done)));
            if let Some(g) = guide@ {
                reveal(guided_step_ok);
                reveal(guided_step_pre);
                reveal(guide_frame_ok);
                reveal(guide_target);
                reveal(guide_end);
                match g {
                    GTermFrame::List {
                        built,
                        remaining,
                        tail,
                        tail_mode: guide_tail_mode,
                        end: guide_end,
                    } => {
                        assert(guide_tail_mode);
                        assert(remaining.len() == 0);
                        assert(elems@ == built);
                        assert(child@ == tail);
                        assert(done@ == list_term(built, tail));
                        spanned_term_length(bytes@, &child);
                        assert(bytes@.subrange(child_start as int, guide_end).len()
                            == guide_end - child_start);
                        assert(guide_end == child.end as int + 1);
                        assert(done@ == guide_target(g));
                        assert(done.end as int == guide_end);
                    },
                    _ => assert(false),
                }
            }
        }
        return EFrameStep::Done(done);
    }
    let ghost next_elems = elems@.push(child@);
    let next_count = count + 1;
    let next_ground = ground && child.parsed.ground;
    let next_no_dollar = no_dollar && child.parsed.no_dollar;
    proof {
        assert(count < usize::MAX);
        wf_terms_push(elems@, child@);
        ground_all_push(elems@, child@);
        no_dollar_all_push(elems@, child@);
        args_bytes_push(elems@, child@);
    }
    if delimiter == 0x2c || delimiter == 0x7c {
        let next_child_start = child.end + 1;
        let next_tail = delimiter == 0x7c;
        let next = ETermFrame::List {
            start,
            child_start: next_child_start,
            elems: Ghost(next_elems),
            count: next_count,
            ground: next_ground,
            no_dollar: next_no_dollar,
            tail: next_tail,
        };
        proof {
            assert_seqs_equal!(bytes@.subrange(start as int, next_child_start as int)
                == bytes@.subrange(start as int, child_start as int)
                    + bytes@.subrange(child_start as int, child.end as int)
                    + seq![delimiter]);
            reveal(args_prefix);
            reveal(list_frame_ok);
            reveal(spanned_term_ok);
            assert(next_count == next_elems.len());
            assert(next_count < next_child_start - start);
            assert(ckc_spec::term::wf_terms(next_elems));
            assert(next_ground == ckc_spec::term::ground_all(next_elems));
            assert(next_no_dollar == ckc_spec::term::no_dollar_var_all(next_elems));
            assert(bytes@.subrange(start as int, next_child_start as int)
                == seq![0x5bu8] + if next_tail {
                    ckc_spec::v1text::args_bytes(next_elems) + seq![0x7cu8]
                } else {
                    args_prefix(next_elems)
                });
            assert(list_frame_ok(
                bytes@,
                start,
                next_child_start,
                next_elems,
                next_count,
                next_ground,
                next_no_dollar,
                next_tail,
            ));
            reveal(frame_ok);
            reveal(frame_start);
            reveal(frame_child_start);
            reveal(frame_step_ok);
            assert(frame_ok(bytes@, &next));
            if let Some(g) = guide@ {
                reveal(guided_step_ok);
                reveal(guided_step_pre);
                reveal(guide_frame_ok);
                reveal(guide_delimiter);
                match g {
                    GTermFrame::List {
                        built,
                        remaining,
                        tail,
                        tail_mode: guide_tail_mode,
                        end: guide_end,
                    } => {
                        assert(!guide_tail_mode);
                        assert(child@ == remaining[0]);
                        assert(elems@ == built);
                        assert(next_elems == built.push(remaining[0]));
                        assert_seqs_equal!(built.push(remaining[0]) + remaining.drop_first()
                            == built + remaining);
                        if next_tail {
                            assert(remaining.len() == 1);
                            assert(tail != Term::Nil);
                            assert_seqs_equal!(remaining == seq![remaining[0]]);
                            reveal_with_fuel(ckc_spec::v1text::args_bytes, 2);
                            assert_seqs_equal!(bytes@.subrange(child_start as int, guide_end)
                                == ckc_spec::v1text::term_bytes(child@) + seq![0x7cu8]
                                    + (ckc_spec::v1text::term_bytes(tail) + seq![0x5du8]));
                            suffix_after_term_delimiter(
                                bytes@,
                                child_start as int,
                                guide_end,
                                child@,
                                0x7c,
                                ckc_spec::v1text::term_bytes(tail) + seq![0x5du8],
                            );
                            spanned_term_length(bytes@, &child);
                            assert(next_child_start as int == child_start as int
                                + ckc_spec::v1text::term_bytes(child@).len() + 1);
                            assert(bytes@.subrange(next_child_start as int, guide_end)
                                == ckc_spec::v1text::term_bytes(tail) + seq![0x5du8]);
                            assert((next_child_start as int) < guide_end);
                            assert(guide_frame_ok(
                                bytes@,
                                &next,
                                GTermFrame::List {
                                    built: built.push(remaining[0]),
                                    remaining: Seq::empty(),
                                    tail,
                                    tail_mode: true,
                                    end: guide_end,
                                },
                            ));
                        } else {
                            assert(remaining.len() > 1);
                            wf_terms_drop_first(remaining);
                            let rest = ckc_spec::v1text::args_bytes(remaining.drop_first())
                                + if tail == Term::Nil {
                                    seq![0x5du8]
                                } else {
                                    seq![0x7cu8] + ckc_spec::v1text::term_bytes(tail)
                                        + seq![0x5du8]
                                };
                            reveal_with_fuel(ckc_spec::v1text::args_bytes, 2);
                            assert_seqs_equal!(bytes@.subrange(child_start as int, guide_end)
                                == ckc_spec::v1text::term_bytes(child@) + seq![0x2cu8]
                                    + rest);
                            suffix_after_term_delimiter(
                                bytes@,
                                child_start as int,
                                guide_end,
                                child@,
                                0x2c,
                                rest,
                            );
                            spanned_term_length(bytes@, &child);
                            assert(next_child_start as int == child_start as int
                                + ckc_spec::v1text::term_bytes(child@).len() + 1);
                            assert(bytes@.subrange(next_child_start as int, guide_end) == rest);
                            assert((next_child_start as int) < guide_end);
                            assert(guide_frame_ok(
                                bytes@,
                                &next,
                                GTermFrame::List {
                                    built: built.push(remaining[0]),
                                    remaining: remaining.drop_first(),
                                    tail,
                                    tail_mode: false,
                                    end: guide_end,
                                },
                            ));
                        }
                    },
                    _ => assert(false),
                }
            }
        }
        return EFrameStep::Next(next);
    }
    if delimiter != 0x5d {
        return EFrameStep::Reject;
    }
    let end = child.end + 1;
    proof {
        list_nil_bytes(next_elems);
        assert_seqs_equal!(bytes@.subrange(start as int, end as int)
            == bytes@.subrange(start as int, child_start as int)
                + bytes@.subrange(child_start as int, child.end as int)
                + seq![0x5du8]);
        reveal(args_prefix);
        reveal(list_frame_ok);
        reveal_with_fuel(ckc_spec::term::ground, 2);
        reveal_with_fuel(ckc_spec::term::no_dollar_var, 2);
        reveal(spanned_term_ok);
        assert(bytes@.subrange(start as int, child_start as int)
            == seq![0x5bu8] + args_prefix(elems@));
        assert(ckc_spec::v1text::term_bytes(child@)
            == bytes@.subrange(child_start as int, child.end as int));
        assert(ckc_spec::v1text::term_bytes(list_term(next_elems, Term::Nil))
            == bytes@.subrange(start as int, end as int));
        assert(next_ground
            == (ckc_spec::term::ground_all(next_elems) && ckc_spec::term::ground(Term::Nil)));
        assert(next_no_dollar
            == (ckc_spec::term::no_dollar_var_all(next_elems)
                && ckc_spec::term::no_dollar_var(Term::Nil)));
    }
    let done = finish_list_term(
        bytes,
        start,
        end,
        Ghost(next_elems),
        Ghost(Term::Nil),
        next_ground,
        next_no_dollar,
    );
    proof {
        reveal(frame_step_ok);
        assert(spanned_term_ok(bytes@, &done));
        assert(done.start == start);
        assert(done.end == child.end + 1);
        assert(frame_step_ok(bytes@, start, child.end, &EFrameStep::Done(done)));
        if let Some(g) = guide@ {
            reveal(guided_step_ok);
            reveal(guided_step_pre);
            reveal(guide_frame_ok);
            reveal(guide_target);
            reveal(guide_end);
            match g {
                GTermFrame::List {
                    built,
                    remaining,
                    tail,
                    tail_mode: guide_tail_mode,
                    end: guide_end,
                } => {
                    assert(!guide_tail_mode);
                    assert(remaining.len() == 1);
                    assert(tail == Term::Nil);
                    assert(elems@ == built);
                    assert(child@ == remaining[0]);
                    assert_seqs_equal!(remaining == seq![remaining[0]]);
                    assert_seqs_equal!(built + remaining == built.push(remaining[0]));
                    assert(next_elems == built.push(remaining[0]));
                    assert(done@ == list_term(built + remaining, tail));
                    spanned_term_length(bytes@, &child);
                    assert(bytes@.subrange(child_start as int, guide_end).len()
                        == guide_end - child_start);
                    assert(guide_end == child.end as int + 1);
                    assert(done@ == guide_target(g));
                    assert(done.end as int == guide_end);
                },
                _ => assert(false),
            }
        }
    }
    EFrameStep::Done(done)
}

pub open spec fn curly_frame_ok(
    bytes: Seq<u8>,
    start: usize,
    child_start: usize,
) -> bool {
    &&& start < child_start <= bytes.len()
    &&& child_start == start + 1
    &&& bytes[start as int] == 0x7b
}

fn feed_curly_frame(
    bytes: &[u8],
    start: usize,
    child_start: usize,
    child: ESpannedTerm,
    guide: Ghost<Option<GTermFrame>>,
) -> (out: EFrameStep)
    requires
        curly_frame_ok(bytes@, start, child_start),
        spanned_term_ok(bytes@, &child),
        child.start == child_start,
        guide@ matches Some(g) ==> guided_step_pre(
            bytes@,
            &ETermFrame::Curly { start, child_start },
            &child,
            g,
        ),
    ensures
        frame_step_ok(bytes@, start, child.end, &out),
        guide@ matches Some(g) ==> guided_step_ok(
            bytes@,
            &ETermFrame::Curly { start, child_start },
            &child,
            g,
            &out,
        ),
{
    proof {
        if let Some(g) = guide@ {
            guided_delimiter_at(
                bytes@,
                &ETermFrame::Curly { start, child_start },
                &child,
                g,
            );
            reveal(guided_step_pre);
            reveal(guide_frame_ok);
            reveal(guide_delimiter);
        }
    }
    if child.end == bytes.len() || bytes[child.end] != 0x7d {
        return EFrameStep::Reject;
    }
    let end = child.end + 1;
    let name = curly_name_vec();
    let ghost args = seq![child@];
    let ghost term = Term::Comp(name@, args);
    proof {
        reveal(curly_frame_ok);
        reveal(spanned_term_ok);
        assert_seqs_equal!(bytes@.subrange(start as int, end as int)
            == seq![0x7bu8] + bytes@.subrange(child_start as int, child.end as int)
                + seq![0x7du8]);
        reveal(ckc_spec::v1text::curly_name);
        reveal(ckc_spec::term::dollar_var_name);
        assert(ckc_spec::v1text::curly_name() != ckc_spec::term::dollar_var_name());
        reveal_with_fuel(ckc_spec::term::wf_term, 2);
        reveal_with_fuel(ckc_spec::term::wf_terms, 2);
        reveal_with_fuel(ckc_spec::term::ground, 2);
        reveal_with_fuel(ckc_spec::term::ground_all, 2);
        reveal_with_fuel(ckc_spec::term::no_dollar_var, 2);
        reveal_with_fuel(ckc_spec::term::no_dollar_var_all, 2);
        reveal_with_fuel(ckc_spec::v1text::term_bytes, 2);
        reveal(top_matches);
        reveal(parsed_term_ok);
        reveal(frame_step_ok);
    }
    let parsed = EParsedTerm {
        term: Ghost(term),
        top: ETermTop::Comp(name, 1),
        ground: child.parsed.ground,
        no_dollar: child.parsed.no_dollar,
    };
    let done = ESpannedTerm { parsed, start, end };
    proof {
        reveal(spanned_term_ok);
        assert(spanned_term_ok(bytes@, &done));
        assert(frame_step_ok(bytes@, start, child.end, &EFrameStep::Done(done)));
    }
    EFrameStep::Done(done)
}

fn feed_frame(
    bytes: &[u8],
    frame: ETermFrame,
    child: ESpannedTerm,
    guide: Ghost<Option<GTermFrame>>,
) -> (out: EFrameStep)
    requires
        frame_ok(bytes@, &frame),
        spanned_term_ok(bytes@, &child),
        child.start == frame_child_start(&frame),
        guide@ matches Some(g) ==> guided_step_pre(bytes@, &frame, &child, g),
    ensures
        frame_step_ok(bytes@, frame_start(&frame), child.end, &out),
        guide@ matches Some(g) ==> guided_step_ok(bytes@, &frame, &child, g, &out),
{
    match frame {
        ETermFrame::Comp {
            start, child_start, name, args, count, ground, no_dollar,
        } => {
            proof {
                reveal(frame_ok);
                reveal(comp_frame_ok);
                reveal(frame_start);
                reveal(frame_child_start);
            }
            feed_comp_frame(
                bytes,
                start,
                child_start,
                name,
                args,
                count,
                ground,
                no_dollar,
                child,
                guide,
            )
        },
        ETermFrame::List {
            start, child_start, elems, count, ground, no_dollar, tail,
        } => {
            proof {
                reveal(frame_ok);
                reveal(list_frame_ok);
                reveal(frame_start);
                reveal(frame_child_start);
            }
            feed_list_frame(
                bytes,
                start,
                child_start,
                elems,
                count,
                ground,
                no_dollar,
                tail,
                child,
                guide,
            )
        },
        ETermFrame::Curly { start, child_start } => {
            proof {
                reveal(frame_ok);
                reveal(curly_frame_ok);
                reveal(frame_start);
                reveal(frame_child_start);
            }
            feed_curly_frame(bytes, start, child_start, child, guide)
        },
    }
}

pub open spec fn term_boundary(bytes: Seq<u8>, end: int) -> bool {
    &&& 0 < end <= bytes.len()
    &&& end == bytes.len() || {
        let b = bytes[end];
        ||| b == 0x2c
        ||| b == 0x29
        ||| b == 0x5d
        ||| b == 0x7c
        ||| b == 0x7d
        ||| b == 0x20
        ||| b == 0x2e && end + 1 < bytes.len() && bytes[end + 1] == 0x0a
    }
}

proof fn boundary_stops_alnum(bytes: Seq<u8>, end: int)
    requires
        term_boundary(bytes, end),
        end < bytes.len(),
    ensures !ckc_spec::v1text::is_alnum_b(bytes[end]),
{
    reveal(term_boundary);
    reveal(ckc_spec::v1text::is_alnum_b);
    reveal(ckc_spec::v1text::is_lower_b);
    reveal(ckc_spec::v1text::is_digit_b);
}

proof fn boundary_stops_digit(bytes: Seq<u8>, end: int)
    requires
        term_boundary(bytes, end),
        end < bytes.len(),
    ensures !ckc_spec::v1text::is_digit_b(bytes[end]),
{
    reveal(term_boundary);
    reveal(ckc_spec::v1text::is_digit_b);
}

proof fn boundary_stops_graphic_scan(bytes: Seq<u8>, end: int)
    requires
        term_boundary(bytes, end),
        end < bytes.len(),
    ensures
        !ckc_spec::v1text::is_graphic_b(bytes[end])
            || bytes[end] == 0x2e && end + 1 < bytes.len() && bytes[end + 1] == 0x0a,
{
    reveal(term_boundary);
    reveal(ckc_spec::v1text::is_graphic_b);
}

pub open spec fn term_at(
    bytes: Seq<u8>,
    start: int,
    end: int,
    term: Term,
) -> bool {
    &&& 0 <= start < end <= bytes.len()
    &&& ckc_spec::term::wf_term(term)
    &&& end == start + ckc_spec::v1text::term_bytes(term).len()
    &&& bytes.subrange(start, end) == ckc_spec::v1text::term_bytes(term)
    &&& term_boundary(bytes, end)
}

pub ghost struct GTermExpected {
    pub term: Term,
    pub end: usize,
}

pub ghost enum GTermFrame {
    Comp {
        name: Seq<u8>,
        built: Seq<Term>,
        remaining: Seq<Term>,
        end: int,
    },
    List {
        built: Seq<Term>,
        remaining: Seq<Term>,
        tail: Term,
        tail_mode: bool,
        end: int,
    },
    Curly {
        child: Term,
        end: int,
    },
}

pub open spec fn guide_target(g: GTermFrame) -> Term {
    match g {
        GTermFrame::Comp { name, built, remaining, .. } => {
            Term::Comp(name, built + remaining)
        },
        GTermFrame::List { built, remaining, tail, .. } => {
            list_term(built + remaining, tail)
        },
        GTermFrame::Curly { child, .. } => {
            Term::Comp(ckc_spec::v1text::curly_name(), seq![child])
        },
    }
}

pub open spec fn guide_end(g: GTermFrame) -> int {
    match g {
        GTermFrame::Comp { end, .. } => end,
        GTermFrame::List { end, .. } => end,
        GTermFrame::Curly { end, .. } => end,
    }
}

pub open spec fn guide_next(g: GTermFrame) -> Term
    recommends match g {
        GTermFrame::Comp { remaining, .. } => remaining.len() > 0,
        GTermFrame::List { remaining, tail, tail_mode, .. } => {
            if tail_mode { true } else { remaining.len() > 0 }
        },
        GTermFrame::Curly { .. } => true,
    },
{
    match g {
        GTermFrame::Comp { remaining, .. } => remaining[0],
        GTermFrame::List { remaining, tail, tail_mode, .. } => {
            if tail_mode { tail } else { remaining[0] }
        },
        GTermFrame::Curly { child, .. } => child,
    }
}

pub open spec fn guide_next_end(frame: &ETermFrame, g: GTermFrame) -> int {
    frame_child_start(frame) as int + ckc_spec::v1text::term_bytes(guide_next(g)).len()
}

pub open spec fn guide_frame_ok(
    bytes: Seq<u8>,
    frame: &ETermFrame,
    guide: GTermFrame,
) -> bool {
    &&& frame_ok(bytes, frame)
    &&& match (frame, guide) {
        (
            ETermFrame::Comp { start, child_start, name, args, count, .. },
            GTermFrame::Comp {
                name: gname,
                built,
                remaining,
                end,
            },
        ) => {
            let target = Term::Comp(gname, built + remaining);
            &&& name@ == gname
            &&& args@ == built
            &&& *count == built.len()
            &&& remaining.len() > 0
            &&& ckc_spec::term::wf_terms(built + remaining)
            &&& ckc_spec::term::wf_terms(remaining)
            &&& !(gname == ckc_spec::v1text::cons_name()
                && (built + remaining).len() == 2)
            &&& !(gname == ckc_spec::v1text::curly_name()
                && (built + remaining).len() == 1)
            &&& (*child_start as int) < end
            &&& term_at(bytes, *start as int, end, target)
            &&& bytes.subrange(*child_start as int, end)
                == ckc_spec::v1text::args_bytes(remaining) + seq![0x29u8]
        },
        (
            ETermFrame::List {
                start,
                child_start,
                elems,
                count,
                tail: frame_tail,
                ..
            },
            GTermFrame::List {
                built,
                remaining,
                tail,
                tail_mode,
                end,
            },
        ) => {
            let target = list_term(built + remaining, tail);
            &&& elems@ == built
            &&& *count == built.len()
            &&& *frame_tail == tail_mode
            &&& ckc_spec::term::wf_terms(built + remaining)
            &&& ckc_spec::term::wf_terms(remaining)
            &&& ckc_spec::term::wf_term(tail)
            &&& (tail == Term::Nil || plain_list_tail(tail))
            &&& (*child_start as int) < end
            &&& if tail_mode {
                &&& remaining.len() == 0
                &&& built.len() > 0
                &&& plain_list_tail(tail)
                &&& term_at(bytes, *start as int, end, target)
                &&& bytes.subrange(*child_start as int, end)
                    == ckc_spec::v1text::term_bytes(tail) + seq![0x5du8]
            } else {
                &&& remaining.len() > 0
                &&& term_at(bytes, *start as int, end, target)
                &&& bytes.subrange(*child_start as int, end)
                    == ckc_spec::v1text::args_bytes(remaining)
                        + if tail == Term::Nil {
                            seq![0x5du8]
                        } else {
                            seq![0x7cu8] + ckc_spec::v1text::term_bytes(tail)
                                + seq![0x5du8]
                        }
            }
        },
        (
            ETermFrame::Curly { start, child_start },
            GTermFrame::Curly { child, end },
        ) => {
            let target = Term::Comp(ckc_spec::v1text::curly_name(), seq![child]);
            &&& ckc_spec::term::wf_term(child)
            &&& (*child_start as int) < end
            &&& term_at(bytes, *start as int, end, target)
            &&& bytes.subrange(*child_start as int, end)
                == ckc_spec::v1text::term_bytes(child) + seq![0x7du8]
        },
        _ => false,
    }
}

pub open spec fn guide_stack_ok(
    bytes: Seq<u8>,
    frames: Seq<ETermFrame>,
    guides: Seq<GTermFrame>,
    root: Term,
    root_end: int,
) -> bool {
    &&& frames.len() == guides.len()
    &&& forall|i: int| #![auto] 0 <= i < frames.len()
        ==> guide_frame_ok(bytes, &frames[i], guides[i])
    &&& (guides.len() > 0 ==> guide_target(guides[0]) == root
        && guide_end(guides[0]) == root_end)
    &&& forall|i: int| #![auto] 0 <= i < guides.len() - 1
        ==> guide_next(guides[i]) == guide_target(guides[i + 1])
            && guide_next_end(&frames[i], guides[i]) == guide_end(guides[i + 1])
}

pub open spec fn guided_focus(guides: Seq<GTermFrame>, root: Term) -> Term {
    if guides.len() == 0 { root } else { guide_next(guides.last()) }
}

pub open spec fn guided_focus_end(
    frames: Seq<ETermFrame>,
    guides: Seq<GTermFrame>,
    root_end: int,
) -> int {
    if guides.len() == 0 {
        root_end
    } else {
        guide_next_end(&frames.last(), guides.last())
    }
}

pub open spec fn guided_state_ok(
    bytes: Seq<u8>,
    start: usize,
    pos: usize,
    frames: Seq<ETermFrame>,
    current: &Option<ESpannedTerm>,
    guides: Seq<GTermFrame>,
    root: Term,
    root_end: int,
) -> bool {
    &&& guide_stack_ok(bytes, frames, guides, root, root_end)
    &&& match current {
        Option::None => term_at(
            bytes,
            pos as int,
            guided_focus_end(frames, guides, root_end),
            guided_focus(guides, root),
        ),
        Option::Some(term) => {
            &&& term@ == guided_focus(guides, root)
            &&& term.end as int == guided_focus_end(frames, guides, root_end)
        },
    }
}

pub open spec fn simple_term_delimiter(b: u8) -> bool {
    b == 0x2c || b == 0x29 || b == 0x5d || b == 0x7c || b == 0x7d
}

proof fn udec_bytes_nonempty(n: nat)
    ensures ckc_spec::v1text::udec_bytes(n).len() > 0,
    decreases n,
{
    reveal_with_fuel(ckc_spec::v1text::udec_bytes, 2);
    if n >= 10 {
        assert(n / 10 < n);
        udec_bytes_nonempty(n / 10);
    }
}

proof fn term_bytes_nonempty(term: Term)
    ensures ckc_spec::v1text::term_bytes(term).len() > 0,
    decreases term,
{
    match term {
        Term::Var(k) => {
            reveal(ckc_spec::v1text::var_bytes);
            if k / 26 > 0 {
                udec_bytes_nonempty(k / 26);
            }
        },
        Term::Int(n) => {
            reveal(ckc_spec::v1text::dec_bytes);
            udec_bytes_nonempty(if n < 0 { (-n) as nat } else { n as nat });
        },
        Term::Nil => {
            reveal_strlit("[]");
            reveal(ckc_spec::v1text::ascii);
        },
        Term::Atom(name) => {
            reveal(ckc_spec::v1text::atom_bytes);
            if ckc_spec::v1text::atom_bare(name) {
                reveal(ckc_spec::v1text::atom_bare);
                reveal(ckc_spec::v1text::alpha_bare);
                reveal(ckc_spec::v1text::graphic_bare);
                reveal(ckc_spec::v1text::solo_bare);
            }
        },
        Term::Comp(name, args) => {
            reveal_with_fuel(ckc_spec::v1text::term_bytes, 2);
        },
    }
    reveal_with_fuel(ckc_spec::v1text::term_bytes, 2);
}

proof fn args_bytes_nonempty(terms: Seq<Term>)
    requires terms.len() > 0,
    ensures ckc_spec::v1text::args_bytes(terms).len() > 0,
{
    term_bytes_nonempty(terms[0]);
    reveal_with_fuel(ckc_spec::v1text::args_bytes, 2);
}

proof fn atom_bytes_first_safe(name: Seq<u8>)
    ensures
        ckc_spec::v1text::atom_bytes(name).len() > 0,
        ckc_spec::v1text::atom_bytes(name)[0] != 0x5b,
        ckc_spec::v1text::atom_bytes(name)[0] != 0x5d,
        ckc_spec::v1text::atom_bytes(name)[0] != 0x7d,
        !(0x41 <= ckc_spec::v1text::atom_bytes(name)[0] <= 0x5a),
        !ckc_spec::v1text::is_digit_b(ckc_spec::v1text::atom_bytes(name)[0]),
        ckc_spec::v1text::atom_bytes(name)[0] == 0x2d
            && ckc_spec::v1text::atom_bytes(name).len() > 1
            ==> !ckc_spec::v1text::is_digit_b(ckc_spec::v1text::atom_bytes(name)[1]),
        ckc_spec::v1text::atom_bytes(name)[0] == 0x7b ==> {
            &&& ckc_spec::v1text::atom_bytes(name).len() >= 2
            &&& ckc_spec::v1text::atom_bytes(name)[1] == 0x7d
        },
{
    reveal(ckc_spec::v1text::atom_bytes);
    if ckc_spec::v1text::atom_bare(name) {
        reveal(ckc_spec::v1text::atom_bare);
        reveal(ckc_spec::v1text::alpha_bare);
        reveal(ckc_spec::v1text::graphic_bare);
        reveal(ckc_spec::v1text::solo_bare);
        reveal(ckc_spec::v1text::all_in);
        reveal(ckc_spec::v1text::is_lower_b);
        reveal(ckc_spec::v1text::is_graphic_b);
        seq_eq_one(name, 0x3b);
        seq_eq_one(name, 0x21);
        seq_eq_two(name, 0x7b, 0x7d);
    }
}

proof fn term_bytes_first_safe(term: Term)
    ensures
        ckc_spec::v1text::term_bytes(term).len() > 0,
        ckc_spec::v1text::term_bytes(term)[0] != 0x5d,
        ckc_spec::v1text::term_bytes(term)[0] != 0x7d,
{
    term_bytes_nonempty(term);
    match term {
        Term::Var(k) => {
            let letter = (0x41 + k % 26) as u8;
            assert(k % 26 < 26);
            assert(0x41 <= letter <= 0x5a);
            reveal(ckc_spec::v1text::term_bytes);
            reveal(ckc_spec::v1text::var_bytes);
            if k / 26 == 0 {
                assert(ckc_spec::v1text::term_bytes(Term::Var(k)) == seq![letter]);
                assert(ckc_spec::v1text::term_bytes(Term::Var(k))[0] == letter);
            } else {
                assert(ckc_spec::v1text::term_bytes(Term::Var(k))[0] == letter);
            }
            assert(letter != 0x5d);
            assert(letter != 0x7d);
            assert(ckc_spec::v1text::term_bytes(term)[0] == letter);
            assert(ckc_spec::v1text::term_bytes(term)[0] != 0x5d);
            assert(ckc_spec::v1text::term_bytes(term)[0] != 0x7d);
            return;
        },
        Term::Int(n) => {
            reveal(ckc_spec::v1text::term_bytes);
            reveal(ckc_spec::v1text::dec_bytes);
            if n < 0 {
                assert(ckc_spec::v1text::term_bytes(term)[0] == 0x2d);
            } else {
                udec_canonical(n as nat);
                reveal(canonical_decimal);
                reveal(ckc_spec::v1text::all_in);
                assert(ckc_spec::v1text::is_digit_b(
                    ckc_spec::v1text::term_bytes(term)[0],
                ));
                reveal(ckc_spec::v1text::is_digit_b);
            }
            assert(ckc_spec::v1text::term_bytes(term)[0] != 0x5d);
            assert(ckc_spec::v1text::term_bytes(term)[0] != 0x7d);
            return;
        },
        Term::Nil => {
            reveal(ckc_spec::v1text::term_bytes);
            reveal_strlit("[]");
            reveal(ckc_spec::v1text::ascii);
            return;
        },
        Term::Atom(name) => {
            atom_bytes_first_safe(name);
            reveal(ckc_spec::v1text::term_bytes);
            return;
        },
        Term::Comp(name, args) => {
            reveal_with_fuel(ckc_spec::v1text::term_bytes, 2);
            if name == ckc_spec::v1text::cons_name() && args.len() == 2 {
            } else if name == ckc_spec::v1text::curly_name() && args.len() == 1 {
            } else {
                atom_bytes_first_safe(name);
            }
            return;
        },
    }
}

proof fn wf_terms_drop_first(terms: Seq<Term>)
    requires
        terms.len() > 0,
        ckc_spec::term::wf_terms(terms),
    ensures ckc_spec::term::wf_terms(terms.drop_first()),
{
    reveal_with_fuel(ckc_spec::term::wf_terms, 2);
}

proof fn wf_terms_first(terms: Seq<Term>)
    requires
        terms.len() > 0,
        ckc_spec::term::wf_terms(terms),
    ensures ckc_spec::term::wf_term(terms[0]),
{
    reveal_with_fuel(ckc_spec::term::wf_terms, 2);
}

proof fn term_before_simple_delimiter(
    bytes: Seq<u8>,
    start: int,
    end: int,
    term: Term,
    delimiter: u8,
    rest: Seq<u8>,
)
    requires
        0 <= start < end <= bytes.len(),
        ckc_spec::term::wf_term(term),
        simple_term_delimiter(delimiter),
        bytes.subrange(start, end)
            == ckc_spec::v1text::term_bytes(term) + seq![delimiter] + rest,
    ensures term_at(
        bytes,
        start,
        start + ckc_spec::v1text::term_bytes(term).len(),
        term,
    ),
{
    let term_bytes = ckc_spec::v1text::term_bytes(term);
    let next = start + term_bytes.len();
    term_bytes_nonempty(term);
    assert(bytes.subrange(start, end).len() == end - start);
    assert((term_bytes + seq![delimiter] + rest).len()
        == term_bytes.len() + 1 + rest.len());
    assert(end - start == term_bytes.len() + 1 + rest.len());
    assert(next < end);
    assert(bytes.subrange(start, end)[term_bytes.len() as int] == delimiter);
    assert(bytes.subrange(start, end)[next - start] == bytes[next]);
    assert(bytes[next] == delimiter);
    assert_seqs_equal!(bytes.subrange(start, next) == term_bytes);
    reveal(simple_term_delimiter);
    reveal(term_boundary);
    reveal(term_at);
}

proof fn byte_after_term(
    bytes: Seq<u8>,
    start: int,
    end: int,
    term: Term,
    delimiter: u8,
    rest: Seq<u8>,
)
    requires
        0 <= start < end <= bytes.len(),
        bytes.subrange(start, end)
            == ckc_spec::v1text::term_bytes(term) + seq![delimiter] + rest,
    ensures
        start + ckc_spec::v1text::term_bytes(term).len() < end,
        bytes[start + ckc_spec::v1text::term_bytes(term).len()] == delimiter,
{
    let term_bytes = ckc_spec::v1text::term_bytes(term);
    let next = start + term_bytes.len();
    assert(bytes.subrange(start, end).len() == end - start);
    assert((term_bytes + seq![delimiter] + rest).len()
        == term_bytes.len() + 1 + rest.len());
    assert(end - start == term_bytes.len() + 1 + rest.len());
    assert(next < end);
    assert(bytes.subrange(start, end)[term_bytes.len() as int] == delimiter);
    assert(bytes.subrange(start, end)[next - start] == bytes[next]);
}

proof fn suffix_after_term_delimiter(
    bytes: Seq<u8>,
    start: int,
    end: int,
    term: Term,
    delimiter: u8,
    rest: Seq<u8>,
)
    requires
        0 <= start < end <= bytes.len(),
        bytes.subrange(start, end)
            == ckc_spec::v1text::term_bytes(term) + seq![delimiter] + rest,
    ensures
        start + ckc_spec::v1text::term_bytes(term).len() + 1 <= end,
        bytes.subrange(
            start + ckc_spec::v1text::term_bytes(term).len() + 1,
            end,
        ) == rest,
{
    let term_bytes = ckc_spec::v1text::term_bytes(term);
    let mid = start + term_bytes.len() + 1;
    assert(bytes.subrange(start, end).len() == end - start);
    assert((term_bytes + seq![delimiter] + rest).len()
        == term_bytes.len() + 1 + rest.len());
    assert(end - start == term_bytes.len() + 1 + rest.len());
    assert(mid <= end);
    let suffix = bytes.subrange(mid, end);
    assert(suffix.len() == rest.len());
    assert forall|i: int| #![auto] 0 <= i < rest.len() ==> suffix[i] == rest[i] by {
        if 0 <= i < rest.len() {
            assert(suffix[i] == bytes[mid + i]);
            assert(bytes.subrange(start, end)[term_bytes.len() + 1 + i]
                == bytes[mid + i]);
            assert((term_bytes + seq![delimiter] + rest)[term_bytes.len() + 1 + i]
                == rest[i]);
        }
    }
    assert_seqs_equal!(suffix == rest);
}

proof fn guide_next_at(bytes: Seq<u8>, frame: &ETermFrame, guide: GTermFrame)
    requires guide_frame_ok(bytes, frame, guide),
    ensures term_at(
        bytes,
        frame_child_start(frame) as int,
        guide_next_end(frame, guide),
        guide_next(guide),
    ),
{
    reveal(guide_frame_ok);
    reveal(guide_next);
    reveal(guide_next_end);
    match (frame, guide) {
        (
            ETermFrame::Comp { child_start, .. },
            GTermFrame::Comp { remaining, end, .. },
        ) => {
            wf_terms_first(remaining);
            reveal_with_fuel(ckc_spec::v1text::args_bytes, 2);
            if remaining.len() == 1 {
                term_before_simple_delimiter(
                    bytes,
                    *child_start as int,
                    end,
                    remaining[0],
                    0x29,
                    Seq::empty(),
                );
            } else {
                assert_seqs_equal!(bytes.subrange(*child_start as int, end)
                    == ckc_spec::v1text::term_bytes(remaining[0]) + seq![0x2cu8]
                        + (ckc_spec::v1text::args_bytes(remaining.drop_first())
                            + seq![0x29u8]));
                term_before_simple_delimiter(
                    bytes,
                    *child_start as int,
                    end,
                    remaining[0],
                    0x2c,
                    ckc_spec::v1text::args_bytes(remaining.drop_first()) + seq![0x29u8],
                );
            }
        },
        (
            ETermFrame::List { child_start, .. },
            GTermFrame::List { remaining, tail, tail_mode, end, .. },
        ) => {
            if tail_mode {
                term_before_simple_delimiter(
                    bytes,
                    *child_start as int,
                    end,
                    tail,
                    0x5d,
                    Seq::empty(),
                );
            } else {
                wf_terms_first(remaining);
                reveal_with_fuel(ckc_spec::v1text::args_bytes, 2);
                if remaining.len() > 1 {
                    let rest = ckc_spec::v1text::args_bytes(remaining.drop_first())
                        + if tail == Term::Nil {
                            seq![0x5du8]
                        } else {
                            seq![0x7cu8] + ckc_spec::v1text::term_bytes(tail)
                                + seq![0x5du8]
                        };
                    assert_seqs_equal!(bytes.subrange(*child_start as int, end)
                        == ckc_spec::v1text::term_bytes(remaining[0]) + seq![0x2cu8]
                            + rest);
                    term_before_simple_delimiter(
                        bytes,
                        *child_start as int,
                        end,
                        remaining[0],
                        0x2c,
                        rest,
                    );
                } else if tail == Term::Nil {
                    term_before_simple_delimiter(
                        bytes,
                        *child_start as int,
                        end,
                        remaining[0],
                        0x5d,
                        Seq::empty(),
                    );
                } else {
                    assert_seqs_equal!(bytes.subrange(*child_start as int, end)
                        == ckc_spec::v1text::term_bytes(remaining[0]) + seq![0x7cu8]
                            + (ckc_spec::v1text::term_bytes(tail) + seq![0x5du8]));
                    term_before_simple_delimiter(
                        bytes,
                        *child_start as int,
                        end,
                        remaining[0],
                        0x7c,
                        ckc_spec::v1text::term_bytes(tail) + seq![0x5du8],
                    );
                }
            }
        },
        (
            ETermFrame::Curly { child_start, .. },
            GTermFrame::Curly { child, end },
        ) => {
            term_before_simple_delimiter(
                bytes,
                *child_start as int,
                end,
                child,
                0x7d,
                Seq::empty(),
            );
        },
        _ => assert(false),
    }
}

proof fn spanned_term_length(bytes: Seq<u8>, term: &ESpannedTerm)
    requires spanned_term_ok(bytes, term),
    ensures term.end as int
        == term.start as int + ckc_spec::v1text::term_bytes(term@).len(),
{
    reveal(spanned_term_ok);
    assert(bytes.subrange(term.start as int, term.end as int).len()
        == term.end - term.start);
}

pub open spec fn guide_delimiter(guide: GTermFrame) -> u8 {
    match guide {
        GTermFrame::Comp { remaining, .. } => {
            if remaining.len() > 1 { 0x2c } else { 0x29 }
        },
        GTermFrame::List { remaining, tail, tail_mode, .. } => {
            if tail_mode || remaining.len() == 1 && tail == Term::Nil {
                0x5d
            } else if remaining.len() > 1 {
                0x2c
            } else {
                0x7c
            }
        },
        GTermFrame::Curly { .. } => 0x7d,
    }
}

#[verifier::rlimit(30)]
proof fn guided_delimiter_at(
    bytes: Seq<u8>,
    frame: &ETermFrame,
    child: &ESpannedTerm,
    guide: GTermFrame,
)
    requires
        spanned_term_ok(bytes, child),
        guided_step_pre(bytes, frame, child, guide),
    ensures
        child.end < bytes.len(),
        bytes[child.end as int] == guide_delimiter(guide),
{
    reveal(guided_step_pre);
    reveal(guide_frame_ok);
    reveal(guide_next);
    reveal(guide_next_end);
    reveal(guide_delimiter);
    spanned_term_length(bytes, child);
    match (frame, guide) {
        (
            ETermFrame::Comp { child_start, .. },
            GTermFrame::Comp { remaining, end, .. },
        ) => {
            reveal_with_fuel(ckc_spec::v1text::args_bytes, 2);
            if remaining.len() == 1 {
                assert_seqs_equal!(remaining == seq![remaining[0]]);
                byte_after_term(
                    bytes,
                    *child_start as int,
                    end,
                    child@,
                    0x29,
                    Seq::empty(),
                );
            } else {
                assert_seqs_equal!(bytes.subrange(*child_start as int, end)
                    == ckc_spec::v1text::term_bytes(child@) + seq![0x2cu8]
                        + (ckc_spec::v1text::args_bytes(remaining.drop_first())
                            + seq![0x29u8]));
                byte_after_term(
                    bytes,
                    *child_start as int,
                    end,
                    child@,
                    0x2c,
                    ckc_spec::v1text::args_bytes(remaining.drop_first()) + seq![0x29u8],
                );
            }
        },
        (
            ETermFrame::List { child_start, .. },
            GTermFrame::List { remaining, tail, tail_mode, end, .. },
        ) => {
            if tail_mode {
                byte_after_term(
                    bytes,
                    *child_start as int,
                    end,
                    child@,
                    0x5d,
                    Seq::empty(),
                );
            } else {
                reveal_with_fuel(ckc_spec::v1text::args_bytes, 2);
                if remaining.len() > 1 {
                    let rest = ckc_spec::v1text::args_bytes(remaining.drop_first())
                        + if tail == Term::Nil {
                            seq![0x5du8]
                        } else {
                            seq![0x7cu8] + ckc_spec::v1text::term_bytes(tail)
                                + seq![0x5du8]
                        };
                    assert_seqs_equal!(bytes.subrange(*child_start as int, end)
                        == ckc_spec::v1text::term_bytes(child@) + seq![0x2cu8] + rest);
                    byte_after_term(
                        bytes,
                        *child_start as int,
                        end,
                        child@,
                        0x2c,
                        rest,
                    );
                } else if tail == Term::Nil {
                    byte_after_term(
                        bytes,
                        *child_start as int,
                        end,
                        child@,
                        0x5d,
                        Seq::empty(),
                    );
                } else {
                    assert_seqs_equal!(bytes.subrange(*child_start as int, end)
                        == ckc_spec::v1text::term_bytes(child@) + seq![0x7cu8]
                            + (ckc_spec::v1text::term_bytes(tail) + seq![0x5du8]));
                    byte_after_term(
                        bytes,
                        *child_start as int,
                        end,
                        child@,
                        0x7c,
                        ckc_spec::v1text::term_bytes(tail) + seq![0x5du8],
                    );
                }
            }
        },
        (
            ETermFrame::Curly { child_start, .. },
            GTermFrame::Curly { child: expected, end },
        ) => {
            byte_after_term(
                bytes,
                *child_start as int,
                end,
                expected,
                0x7d,
                Seq::empty(),
            );
        },
        _ => assert(false),
    }
    assert(child.end as int
        == frame_child_start(frame) as int + ckc_spec::v1text::term_bytes(child@).len());
}

pub open spec fn guided_step_pre(
    bytes: Seq<u8>,
    frame: &ETermFrame,
    child: &ESpannedTerm,
    guide: GTermFrame,
) -> bool {
    &&& guide_frame_ok(bytes, frame, guide)
    &&& child@ == guide_next(guide)
    &&& child.start == frame_child_start(frame)
    &&& child.end as int == guide_next_end(frame, guide)
}

pub open spec fn guided_step_ok(
    bytes: Seq<u8>,
    frame: &ETermFrame,
    child: &ESpannedTerm,
    guide: GTermFrame,
    out: &EFrameStep,
) -> bool {
    &&& guided_step_pre(bytes, frame, child, guide)
    &&& match guide {
        GTermFrame::Comp { name, built, remaining, end } => {
            if remaining.len() > 1 {
                match out {
                    EFrameStep::Next(next) => guide_frame_ok(
                        bytes,
                        next,
                        GTermFrame::Comp {
                            name,
                            built: built.push(remaining[0]),
                            remaining: remaining.drop_first(),
                            end,
                        },
                    ),
                    _ => false,
                }
            } else {
                match out {
                    EFrameStep::Done(done) => {
                        &&& done@ == guide_target(guide)
                        &&& done.start == frame_start(frame)
                        &&& done.end as int == guide_end(guide)
                    },
                    _ => false,
                }
            }
        },
        GTermFrame::List { built, remaining, tail, tail_mode, end } => {
            if tail_mode || remaining.len() == 1 && tail == Term::Nil {
                match out {
                    EFrameStep::Done(done) => {
                        &&& done@ == guide_target(guide)
                        &&& done.start == frame_start(frame)
                        &&& done.end as int == guide_end(guide)
                    },
                    _ => false,
                }
            } else if remaining.len() > 1 {
                match out {
                    EFrameStep::Next(next) => guide_frame_ok(
                        bytes,
                        next,
                        GTermFrame::List {
                            built: built.push(remaining[0]),
                            remaining: remaining.drop_first(),
                            tail,
                            tail_mode: false,
                            end,
                        },
                    ),
                    _ => false,
                }
            } else {
                match out {
                    EFrameStep::Next(next) => guide_frame_ok(
                        bytes,
                        next,
                        GTermFrame::List {
                            built: built.push(remaining[0]),
                            remaining: Seq::empty(),
                            tail,
                            tail_mode: true,
                            end,
                        },
                    ),
                    _ => false,
                }
            }
        },
        GTermFrame::Curly { .. } => match out {
            EFrameStep::Done(done) => {
                &&& done@ == guide_target(guide)
                &&& done.start == frame_start(frame)
                &&& done.end as int == guide_end(guide)
            },
            _ => false,
        },
    }
}

pub open spec fn guide_has_next(guide: GTermFrame) -> bool {
    match guide {
        GTermFrame::Comp { remaining, .. } => remaining.len() > 1,
        GTermFrame::List { remaining, tail, tail_mode, .. } => {
            !tail_mode && !(remaining.len() == 1 && tail == Term::Nil)
        },
        GTermFrame::Curly { .. } => false,
    }
}

pub open spec fn advance_guide(guide: GTermFrame) -> GTermFrame {
    match guide {
        GTermFrame::Comp { name, built, remaining, end } => {
            if remaining.len() > 0 {
                GTermFrame::Comp {
                    name,
                    built: built.push(remaining[0]),
                    remaining: remaining.drop_first(),
                    end,
                }
            } else {
                guide
            }
        },
        GTermFrame::List { built, remaining, tail, tail_mode, end } => {
            if !tail_mode && remaining.len() > 0 {
                if remaining.len() > 1 {
                    GTermFrame::List {
                        built: built.push(remaining[0]),
                        remaining: remaining.drop_first(),
                        tail,
                        tail_mode: false,
                        end,
                    }
                } else {
                    GTermFrame::List {
                        built: built.push(remaining[0]),
                        remaining: Seq::empty(),
                        tail,
                        tail_mode: true,
                        end,
                    }
                }
            } else {
                guide
            }
        },
        GTermFrame::Curly { .. } => guide,
    }
}

proof fn guided_next_facts(
    bytes: Seq<u8>,
    frame: &ETermFrame,
    child: &ESpannedTerm,
    guide: GTermFrame,
    next: &ETermFrame,
)
    requires guided_step_ok(
        bytes,
        frame,
        child,
        guide,
        &EFrameStep::Next(*next),
    ),
    ensures
        guide_has_next(guide),
        guide_frame_ok(bytes, next, advance_guide(guide)),
        guide_target(advance_guide(guide)) == guide_target(guide),
        guide_end(advance_guide(guide)) == guide_end(guide),
{
    reveal(guided_step_ok);
    reveal(guide_has_next);
    reveal(advance_guide);
    reveal(guide_target);
    reveal(guide_end);
    match guide {
        GTermFrame::Comp { built, remaining, .. } => {
            assert(remaining.len() > 1);
            assert_seqs_equal!(remaining
                == seq![remaining[0]] + remaining.drop_first());
            assert_seqs_equal!(built.push(remaining[0])
                == built + seq![remaining[0]]);
            assert_seqs_equal!(built.push(remaining[0]) + remaining.drop_first()
                == built + remaining);
        },
        GTermFrame::List { built, remaining, tail, tail_mode, .. } => {
            assert(!tail_mode);
            assert(remaining.len() > 0);
            assert(!(remaining.len() == 1 && tail == Term::Nil));
            assert_seqs_equal!(remaining
                == seq![remaining[0]] + remaining.drop_first());
            assert_seqs_equal!(built.push(remaining[0])
                == built + seq![remaining[0]]);
            if remaining.len() > 1 {
                assert_seqs_equal!(built.push(remaining[0]) + remaining.drop_first()
                    == built + remaining);
            } else {
                assert(remaining.len() == 1);
                assert_seqs_equal!(remaining == seq![remaining[0]]);
                assert_seqs_equal!(built.push(remaining[0]) == built + remaining);
            }
        },
        GTermFrame::Curly { .. } => assert(false),
    }
}

pub open spec fn parse_state_ok(
    bytes: Seq<u8>,
    start: usize,
    pos: usize,
    frames: Seq<ETermFrame>,
    current: &Option<ESpannedTerm>,
) -> bool {
    &&& start <= pos <= bytes.len()
    &&& forall|i: int| #![auto] 0 <= i < frames.len() ==> frame_ok(bytes, &frames[i])
    &&& (frames.len() > 0 ==> frame_start(&frames[0]) == start)
    &&& forall|i: int| #![auto] 0 <= i < frames.len() - 1
        ==> frame_child_start(&frames[i]) == frame_start(&frames[i + 1])
    &&& match current {
        Option::None => if frames.len() == 0 {
            pos == start
        } else {
            pos == frame_child_start(&frames.last())
        },
        Option::Some(term) => {
            &&& spanned_term_ok(bytes, term)
            &&& term.end == pos
            &&& if frames.len() == 0 {
                term.start == start
            } else {
                term.start == frame_child_start(&frames.last())
            }
        },
    }
}

proof fn parse_state_initial(bytes: Seq<u8>, start: usize)
    requires start <= bytes.len(),
    ensures parse_state_ok(bytes, start, start, Seq::empty(), &Option::None),
{
    reveal(parse_state_ok);
}

proof fn parse_state_push(
    bytes: Seq<u8>,
    start: usize,
    pos: usize,
    frames: Seq<ETermFrame>,
    frame: ETermFrame,
)
    requires
        parse_state_ok(bytes, start, pos, frames, &Option::None),
        frame_ok(bytes, &frame),
        frame_start(&frame) == pos,
    ensures
        parse_state_ok(
            bytes,
            start,
            frame_child_start(&frame),
            frames.push(frame),
            &Option::None,
        ),
{
    reveal(parse_state_ok);
    assert forall|i: int| #![auto] 0 <= i < frames.push(frame).len()
        ==> frame_ok(bytes, &frames.push(frame)[i]) by {
        if 0 <= i < frames.push(frame).len() {
            if i < frames.len() {
                assert(frames.push(frame)[i] == frames[i]);
            } else {
                assert(i == frames.len());
                assert(frames.push(frame)[i] == frame);
            }
        }
    }
    assert forall|i: int| #![auto] 0 <= i < frames.push(frame).len() - 1
        ==> frame_child_start(&frames.push(frame)[i])
            == frame_start(&frames.push(frame)[i + 1]) by {
        if 0 <= i < frames.push(frame).len() - 1 {
            if i + 1 < frames.len() {
                assert(frames.push(frame)[i] == frames[i]);
                assert(frames.push(frame)[i + 1] == frames[i + 1]);
            } else {
                assert(frames.len() > 0);
                assert(i == frames.len() - 1);
                assert(frames.push(frame)[i] == frames.last());
                assert(frames.push(frame)[i + 1] == frame);
            }
        }
    }
}

proof fn parse_state_current(
    bytes: Seq<u8>,
    start: usize,
    pos: usize,
    frames: Seq<ETermFrame>,
    term: ESpannedTerm,
)
    requires
        parse_state_ok(bytes, start, pos, frames, &Option::None),
        spanned_term_ok(bytes, &term),
        term.start == pos,
    ensures
        parse_state_ok(bytes, start, term.end, frames, &Option::Some(term)),
{
    reveal(parse_state_ok);
}

proof fn parse_state_replace_last(
    bytes: Seq<u8>,
    start: usize,
    pos: usize,
    frames: Seq<ETermFrame>,
    child: ESpannedTerm,
    next: ETermFrame,
)
    requires
        frames.len() > 0,
        parse_state_ok(bytes, start, pos, frames, &Option::Some(child)),
        frame_ok(bytes, &next),
        frame_start(&next) == frame_start(&frames.last()),
        frame_child_start(&next) == child.end + 1,
    ensures
        parse_state_ok(
            bytes,
            start,
            frame_child_start(&next),
            frames.drop_last().push(next),
            &Option::None,
        ),
{
    let out = frames.drop_last().push(next);
    reveal(parse_state_ok);
    assert(frames == frames.drop_last().push(frames.last()));
    assert forall|i: int| #![auto] 0 <= i < out.len() ==> frame_ok(bytes, &out[i]) by {
        if 0 <= i < out.len() {
            if i < frames.len() - 1 {
                assert(out[i] == frames[i]);
            } else {
                assert(i == frames.len() - 1);
                assert(out[i] == next);
            }
        }
    }
    assert forall|i: int| #![auto] 0 <= i < out.len() - 1
        ==> frame_child_start(&out[i]) == frame_start(&out[i + 1]) by {
        if 0 <= i < out.len() - 1 {
            if i + 1 < frames.len() - 1 {
                assert(out[i] == frames[i]);
                assert(out[i + 1] == frames[i + 1]);
            } else {
                assert(frames.len() >= 2);
                assert(i == frames.len() - 2);
                assert(out[i] == frames[i]);
                assert(out[i + 1] == next);
                assert(frames[i + 1] == frames.last());
            }
        }
    }
}

proof fn parse_state_close_last(
    bytes: Seq<u8>,
    start: usize,
    pos: usize,
    frames: Seq<ETermFrame>,
    child: ESpannedTerm,
    done: ESpannedTerm,
)
    requires
        frames.len() > 0,
        parse_state_ok(bytes, start, pos, frames, &Option::Some(child)),
        spanned_term_ok(bytes, &done),
        done.start == frame_start(&frames.last()),
        done.end == child.end + 1,
    ensures
        parse_state_ok(
            bytes,
            start,
            done.end,
            frames.drop_last(),
            &Option::Some(done),
        ),
{
    let out = frames.drop_last();
    reveal(parse_state_ok);
    assert(frames == out.push(frames.last()));
    assert forall|i: int| #![auto] 0 <= i < out.len() ==> frame_ok(bytes, &out[i]) by {
        if 0 <= i < out.len() {
            assert(out[i] == frames[i]);
        }
    }
    assert forall|i: int| #![auto] 0 <= i < out.len() - 1
        ==> frame_child_start(&out[i]) == frame_start(&out[i + 1]) by {
        if 0 <= i < out.len() - 1 {
            assert(out[i] == frames[i]);
            assert(out[i + 1] == frames[i + 1]);
        }
    }
    if out.len() > 0 {
        assert(out.last() == frames[frames.len() - 2]);
        assert(frame_child_start(&out.last()) == frame_start(&frames.last()));
    }
}

proof fn guide_stack_push(
    bytes: Seq<u8>,
    frames: Seq<ETermFrame>,
    guides: Seq<GTermFrame>,
    frame: ETermFrame,
    guide: GTermFrame,
    root: Term,
    root_end: int,
)
    requires
        guide_stack_ok(bytes, frames, guides, root, root_end),
        guide_frame_ok(bytes, &frame, guide),
        guide_target(guide) == guided_focus(guides, root),
        guide_end(guide) == guided_focus_end(frames, guides, root_end),
    ensures guide_stack_ok(
        bytes,
        frames.push(frame),
        guides.push(guide),
        root,
        root_end,
    ),
{
    reveal(guide_stack_ok);
    reveal(guided_focus);
    reveal(guided_focus_end);
    assert forall|i: int| #![auto] 0 <= i < frames.push(frame).len()
        ==> guide_frame_ok(bytes, &frames.push(frame)[i], guides.push(guide)[i]) by {
        if 0 <= i < frames.push(frame).len() {
            if i < frames.len() {
                assert(frames.push(frame)[i] == frames[i]);
                assert(guides.push(guide)[i] == guides[i]);
            } else {
                assert(i == frames.len());
                assert(frames.push(frame)[i] == frame);
                assert(guides.push(guide)[i] == guide);
            }
        }
    }
    assert forall|i: int| #![auto] 0 <= i < guides.push(guide).len() - 1
        ==> guide_next(guides.push(guide)[i])
                == guide_target(guides.push(guide)[i + 1])
            && guide_next_end(&frames.push(frame)[i], guides.push(guide)[i])
                == guide_end(guides.push(guide)[i + 1]) by {
        if 0 <= i < guides.push(guide).len() - 1 {
            if i + 1 < guides.len() {
                assert(guides.push(guide)[i] == guides[i]);
                assert(guides.push(guide)[i + 1] == guides[i + 1]);
                assert(frames.push(frame)[i] == frames[i]);
            } else {
                assert(guides.len() > 0);
                assert(i == guides.len() - 1);
                assert(guides.push(guide)[i] == guides.last());
                assert(guides.push(guide)[i + 1] == guide);
                assert(frames.push(frame)[i] == frames.last());
            }
        }
    }
}

proof fn guide_stack_replace_last(
    bytes: Seq<u8>,
    frames: Seq<ETermFrame>,
    guides: Seq<GTermFrame>,
    next: ETermFrame,
    advanced: GTermFrame,
    root: Term,
    root_end: int,
)
    requires
        frames.len() > 0,
        guide_stack_ok(bytes, frames, guides, root, root_end),
        guide_frame_ok(bytes, &next, advanced),
        guide_target(advanced) == guide_target(guides.last()),
        guide_end(advanced) == guide_end(guides.last()),
    ensures guide_stack_ok(
        bytes,
        frames.drop_last().push(next),
        guides.drop_last().push(advanced),
        root,
        root_end,
    ),
{
    let out_frames = frames.drop_last().push(next);
    let out_guides = guides.drop_last().push(advanced);
    reveal(guide_stack_ok);
    assert(frames.len() == guides.len());
    assert(guides.len() > 0);
    assert forall|i: int| #![auto] 0 <= i < out_frames.len()
        ==> guide_frame_ok(bytes, &out_frames[i], out_guides[i]) by {
        if 0 <= i < out_frames.len() {
            if i < frames.len() - 1 {
                assert(out_frames[i] == frames[i]);
                assert(out_guides[i] == guides[i]);
            } else {
                assert(i == frames.len() - 1);
                assert(out_frames[i] == next);
                assert(out_guides[i] == advanced);
            }
        }
    }
    if guides.len() == 1 {
        assert(out_guides[0] == advanced);
        assert(guides[0] == guides.last());
    } else {
        assert(out_guides[0] == guides[0]);
    }
    assert forall|i: int| #![auto] 0 <= i < out_guides.len() - 1
        ==> guide_next(out_guides[i]) == guide_target(out_guides[i + 1])
            && guide_next_end(&out_frames[i], out_guides[i])
                == guide_end(out_guides[i + 1]) by {
        if 0 <= i < out_guides.len() - 1 {
            if i + 1 < guides.len() - 1 {
                assert(out_guides[i] == guides[i]);
                assert(out_guides[i + 1] == guides[i + 1]);
                assert(out_frames[i] == frames[i]);
            } else {
                assert(guides.len() >= 2);
                assert(i == guides.len() - 2);
                assert(out_guides[i] == guides[i]);
                assert(out_guides[i + 1] == advanced);
                assert(out_frames[i] == frames[i]);
                assert(guides[i + 1] == guides.last());
            }
        }
    }
}

proof fn guide_stack_drop_last(
    bytes: Seq<u8>,
    frames: Seq<ETermFrame>,
    guides: Seq<GTermFrame>,
    root: Term,
    root_end: int,
)
    requires
        frames.len() > 0,
        guide_stack_ok(bytes, frames, guides, root, root_end),
    ensures guide_stack_ok(
        bytes,
        frames.drop_last(),
        guides.drop_last(),
        root,
        root_end,
    ),
{
    let out_frames = frames.drop_last();
    let out_guides = guides.drop_last();
    reveal(guide_stack_ok);
    assert forall|i: int| #![auto] 0 <= i < out_frames.len()
        ==> guide_frame_ok(bytes, &out_frames[i], out_guides[i]) by {
        if 0 <= i < out_frames.len() {
            assert(out_frames[i] == frames[i]);
            assert(out_guides[i] == guides[i]);
        }
    }
    if out_guides.len() > 0 {
        assert(out_guides[0] == guides[0]);
    }
    assert forall|i: int| #![auto] 0 <= i < out_guides.len() - 1
        ==> guide_next(out_guides[i]) == guide_target(out_guides[i + 1])
            && guide_next_end(&out_frames[i], out_guides[i])
                == guide_end(out_guides[i + 1]) by {
        if 0 <= i < out_guides.len() - 1 {
            assert(out_guides[i] == guides[i]);
            assert(out_guides[i + 1] == guides[i + 1]);
            assert(out_frames[i] == frames[i]);
        }
    }
}

proof fn guided_state_initial(
    bytes: Seq<u8>,
    start: usize,
    root: Term,
    root_end: int,
)
    requires term_at(bytes, start as int, root_end, root),
    ensures guided_state_ok(
        bytes,
        start,
        start,
        Seq::empty(),
        &Option::None,
        Seq::empty(),
        root,
        root_end,
    ),
{
    reveal(guided_state_ok);
    reveal(guide_stack_ok);
    reveal(guided_focus);
    reveal(guided_focus_end);
}

proof fn guided_state_push(
    bytes: Seq<u8>,
    start: usize,
    pos: usize,
    frames: Seq<ETermFrame>,
    guides: Seq<GTermFrame>,
    frame: ETermFrame,
    guide: GTermFrame,
    root: Term,
    root_end: int,
)
    requires
        guided_state_ok(
            bytes, start, pos, frames, &Option::None, guides, root, root_end,
        ),
        guide_frame_ok(bytes, &frame, guide),
        guide_target(guide) == guided_focus(guides, root),
        guide_end(guide) == guided_focus_end(frames, guides, root_end),
    ensures guided_state_ok(
        bytes,
        start,
        frame_child_start(&frame),
        frames.push(frame),
        &Option::None,
        guides.push(guide),
        root,
        root_end,
    ),
{
    reveal(guided_state_ok);
    guide_stack_push(bytes, frames, guides, frame, guide, root, root_end);
    guide_next_at(bytes, &frame, guide);
    reveal(guided_focus);
    reveal(guided_focus_end);
}

proof fn guided_state_current(
    bytes: Seq<u8>,
    start: usize,
    pos: usize,
    frames: Seq<ETermFrame>,
    guides: Seq<GTermFrame>,
    term: ESpannedTerm,
    root: Term,
    root_end: int,
)
    requires
        guided_state_ok(
            bytes, start, pos, frames, &Option::None, guides, root, root_end,
        ),
        term@ == guided_focus(guides, root),
        term.end as int == guided_focus_end(frames, guides, root_end),
    ensures guided_state_ok(
        bytes,
        start,
        term.end,
        frames,
        &Option::Some(term),
        guides,
        root,
        root_end,
    ),
{
    reveal(guided_state_ok);
}

proof fn guided_state_next(
    bytes: Seq<u8>,
    start: usize,
    frames: Seq<ETermFrame>,
    guides: Seq<GTermFrame>,
    next: ETermFrame,
    advanced: GTermFrame,
    root: Term,
    root_end: int,
)
    requires
        frames.len() > 0,
        guide_stack_ok(bytes, frames, guides, root, root_end),
        guide_frame_ok(bytes, &next, advanced),
        guide_target(advanced) == guide_target(guides.last()),
        guide_end(advanced) == guide_end(guides.last()),
    ensures guided_state_ok(
        bytes,
        start,
        frame_child_start(&next),
        frames.drop_last().push(next),
        &Option::None,
        guides.drop_last().push(advanced),
        root,
        root_end,
    ),
{
    guide_stack_replace_last(
        bytes, frames, guides, next, advanced, root, root_end,
    );
    guide_next_at(bytes, &next, advanced);
    reveal(guided_state_ok);
    reveal(guided_focus);
    reveal(guided_focus_end);
}

proof fn guided_state_done(
    bytes: Seq<u8>,
    start: usize,
    frames: Seq<ETermFrame>,
    guides: Seq<GTermFrame>,
    done: ESpannedTerm,
    root: Term,
    root_end: int,
)
    requires
        frames.len() > 0,
        guide_stack_ok(bytes, frames, guides, root, root_end),
        done@ == guide_target(guides.last()),
        done.end as int == guide_end(guides.last()),
    ensures guided_state_ok(
        bytes,
        start,
        done.end,
        frames.drop_last(),
        &Option::Some(done),
        guides.drop_last(),
        root,
        root_end,
    ),
{
    guide_stack_drop_last(bytes, frames, guides, root, root_end);
    reveal(guide_stack_ok);
    reveal(guided_state_ok);
    reveal(guided_focus);
    reveal(guided_focus_end);
    if guides.len() == 1 {
        assert(guides[0] == guides.last());
    } else {
        assert(guides.len() >= 2);
        let parent = guides[guides.len() - 2];
        assert(guides.drop_last().last() == parent);
        assert(frames.drop_last().last() == frames[frames.len() - 2]);
        assert(guide_next(parent) == guide_target(guides.last()));
        assert(guide_next_end(&frames[frames.len() - 2], parent)
            == guide_end(guides.last()));
    }
}

pub open spec fn curly_comp(term: Term) -> bool {
    match term {
        Term::Comp(name, args) => {
            name == ckc_spec::v1text::curly_name() && args.len() == 1
        },
        _ => false,
    }
}

pub open spec fn list_open_at(bytes: Seq<u8>, start: usize) -> bool {
    start + 1 < bytes.len()
        && bytes[start as int] == 0x5b
        && bytes[start as int + 1] != 0x5d
}

pub open spec fn curly_open_at(bytes: Seq<u8>, start: usize) -> bool {
    start + 1 < bytes.len()
        && bytes[start as int] == 0x7b
        && bytes[start as int + 1] != 0x7d
}

proof fn expected_term_openers(
    bytes: Seq<u8>,
    start: usize,
    end: usize,
    term: Term,
)
    requires term_at(bytes, start as int, end as int, term),
    ensures
        list_open_at(bytes, start) == list_cons(term),
        curly_open_at(bytes, start) == curly_comp(term),
{
    reveal(term_at);
    reveal(list_open_at);
    reveal(curly_open_at);
    reveal(list_cons);
    reveal(curly_comp);
    match term {
        Term::Var(_) | Term::Int(_) | Term::Nil => {
            expected_atomic_shape(bytes, start, end, term);
            reveal(atomic_term);
            reveal(ckc_spec::v1text::is_digit_b);
        },
        Term::Atom(name) => {
            reveal(ckc_spec::v1text::term_bytes);
            atom_bytes_first_safe(name);
            assert(bytes.subrange(start as int, end as int)
                == ckc_spec::v1text::atom_bytes(name));
            assert(bytes.subrange(start as int, end as int)[0] == bytes[start as int]);
            assert(bytes[start as int] != 0x5b);
            if bytes[start as int] == 0x7b {
                assert(ckc_spec::v1text::atom_bytes(name).len() >= 2);
                assert(start + 1 < end <= bytes.len());
                assert(bytes.subrange(start as int, end as int)[1]
                    == bytes[start as int + 1]);
                assert(bytes[start as int + 1] == 0x7d);
            }
        },
        Term::Comp(name, args) => {
            reveal_with_fuel(ckc_spec::v1text::term_bytes, 2);
            if name == ckc_spec::v1text::cons_name() && args.len() == 2 {
                let head = ckc_spec::v1text::term_bytes(args[0]);
                term_bytes_first_safe(args[0]);
                assert(bytes.subrange(start as int, end as int)
                    == seq![0x5bu8] + head
                        + ckc_spec::v1text::tail_bytes(args[1]) + seq![0x5du8]);
                assert(end >= start + 2);
                assert(bytes.subrange(start as int, end as int)[0]
                    == bytes[start as int]);
                assert(bytes[start as int] == 0x5b);
                assert(bytes.subrange(start as int, end as int)[1]
                    == bytes[start as int + 1]);
                assert(bytes.subrange(start as int, end as int)[1] == head[0]);
                assert(bytes[start as int + 1] != 0x5d);
            } else if name == ckc_spec::v1text::curly_name() && args.len() == 1 {
                let child = ckc_spec::v1text::term_bytes(args[0]);
                term_bytes_first_safe(args[0]);
                assert(bytes.subrange(start as int, end as int)
                    == seq![0x7bu8] + child + seq![0x7du8]);
                assert(end >= start + 2);
                assert(bytes.subrange(start as int, end as int)[0]
                    == bytes[start as int]);
                assert(bytes[start as int] == 0x7b);
                assert(bytes.subrange(start as int, end as int)[1]
                    == bytes[start as int + 1]);
                assert(bytes.subrange(start as int, end as int)[1] == child[0]);
                assert(bytes[start as int + 1] != 0x7d);
            } else {
                let atom = ckc_spec::v1text::atom_bytes(name);
                atom_bytes_first_safe(name);
                assert(bytes.subrange(start as int, end as int)
                    == atom + seq![0x28u8]
                        + ckc_spec::v1text::args_bytes(args) + seq![0x29u8]);
                assert(bytes.subrange(start as int, end as int)[0]
                    == bytes[start as int]);
                assert(bytes[start as int] == atom[0]);
                assert(bytes[start as int] != 0x5b);
                if bytes[start as int] == 0x7b {
                    assert(atom.len() >= 2);
                    assert(start + 1 < end <= bytes.len());
                    assert(bytes.subrange(start as int, end as int)[1]
                        == bytes[start as int + 1]);
                    assert(bytes.subrange(start as int, end as int)[1] == atom[1]);
                    assert(bytes[start as int + 1] == 0x7d);
                }
            }
        },
    }
}

pub open spec fn scalar_nonatom_at(bytes: Seq<u8>, start: usize) -> bool {
    ||| 0x41 <= bytes[start as int] <= 0x5a
    ||| ckc_spec::v1text::is_digit_b(bytes[start as int])
    ||| bytes[start as int] == 0x2d
        && start + 1 < bytes.len()
        && ckc_spec::v1text::is_digit_b(bytes[start as int + 1])
    ||| bytes[start as int] == 0x5b
        && start + 1 < bytes.len()
        && bytes[start as int + 1] == 0x5d
}

pub open spec fn scalar_nonatom_term(term: Term) -> bool {
    matches!(term, Term::Var(_) | Term::Int(_) | Term::Nil)
}

proof fn expected_term_scalar_class(
    bytes: Seq<u8>,
    start: usize,
    end: usize,
    term: Term,
)
    requires term_at(bytes, start as int, end as int, term),
    ensures scalar_nonatom_at(bytes, start) == scalar_nonatom_term(term),
{
    reveal(term_at);
    reveal(scalar_nonatom_at);
    reveal(scalar_nonatom_term);
    match term {
        Term::Var(_) | Term::Int(_) | Term::Nil | Term::Atom(_) => {
            expected_atomic_shape(bytes, start, end, term);
            reveal(atomic_term);
        },
        Term::Comp(name, args) => {
            reveal_with_fuel(ckc_spec::v1text::term_bytes, 2);
            if name == ckc_spec::v1text::cons_name() && args.len() == 2 {
                let head = ckc_spec::v1text::term_bytes(args[0]);
                term_bytes_first_safe(args[0]);
                assert(bytes.subrange(start as int, end as int)
                    == seq![0x5bu8] + head
                        + ckc_spec::v1text::tail_bytes(args[1]) + seq![0x5du8]);
                assert(start + 1 < end <= bytes.len());
                assert(bytes.subrange(start as int, end as int)[0]
                    == bytes[start as int]);
                assert(bytes.subrange(start as int, end as int)[0] == 0x5b);
                assert(bytes.subrange(start as int, end as int)[1]
                    == bytes[start as int + 1]);
                assert(bytes.subrange(start as int, end as int)[1] == head[0]);
                assert(bytes[start as int] == 0x5b);
                assert(bytes[start as int + 1] == head[0]);
                assert(bytes[start as int + 1] != 0x5d);
            } else if name == ckc_spec::v1text::curly_name() && args.len() == 1 {
                assert(bytes.subrange(start as int, end as int)[0]
                    == bytes[start as int]);
                assert(bytes.subrange(start as int, end as int)[0] == 0x7b);
                assert(bytes[start as int] == 0x7b);
                reveal(ckc_spec::v1text::is_digit_b);
            } else {
                let atom = ckc_spec::v1text::atom_bytes(name);
                atom_bytes_first_safe(name);
                let rest = seq![0x28u8]
                    + ckc_spec::v1text::args_bytes(args) + seq![0x29u8];
                assert(bytes.subrange(start as int, end as int) == atom + rest);
                prefix_before_suffix(bytes, start as int, end as int, atom, rest);
                suffix_after_prefix(bytes, start as int, end as int, atom, rest);
                assert(bytes.subrange(start as int, start as int + atom.len())[0]
                    == bytes[start as int]);
                assert(bytes.subrange(start as int, start as int + atom.len())[0]
                    == atom[0]);
                assert(bytes[start as int] == atom[0]);
                if bytes[start as int] == 0x2d && start + 1 < bytes.len() {
                    if atom.len() > 1 {
                        assert(bytes.subrange(start as int, start as int + atom.len())[1]
                            == bytes[start as int + 1]);
                        assert(bytes.subrange(start as int, start as int + atom.len())[1]
                            == atom[1]);
                        assert(bytes[start as int + 1] == atom[1]);
                    } else {
                        assert(bytes.subrange(start as int + atom.len(), end as int)[0]
                            == bytes[start as int + 1]);
                        assert(bytes.subrange(start as int + atom.len(), end as int)[0]
                            == 0x28);
                        assert(bytes[start as int + 1] == 0x28);
                        reveal(ckc_spec::v1text::is_digit_b);
                    }
                }
            }
        },
    }
}

proof fn ordinary_comp_functor_ready(
    bytes: Seq<u8>,
    start: usize,
    end: usize,
    name: Seq<u8>,
    args: Seq<Term>,
)
    requires
        term_at(bytes, start as int, end as int, Term::Comp(name, args)),
        args.len() > 0,
        !(name == ckc_spec::v1text::cons_name() && args.len() == 2),
        !(name == ckc_spec::v1text::curly_name() && args.len() == 1),
    ensures
        start < start as int + ckc_spec::v1text::atom_bytes(name).len() < end,
        ckc_spec::v1text::atom_bytes(name) == bytes.subrange(
            start as int,
            start as int + ckc_spec::v1text::atom_bytes(name).len(),
        ),
        bytes[start as int + ckc_spec::v1text::atom_bytes(name).len()] == 0x28,
        atom_boundary(
            bytes,
            start as int + ckc_spec::v1text::atom_bytes(name).len(),
        ),
{
    reveal(term_at);
    reveal_with_fuel(ckc_spec::v1text::term_bytes, 2);
    atom_bytes_first_safe(name);
    args_bytes_nonempty(args);
    let atom = ckc_spec::v1text::atom_bytes(name);
    let rest = seq![0x28u8] + ckc_spec::v1text::args_bytes(args) + seq![0x29u8];
    assert(bytes.subrange(start as int, end as int) == atom + rest);
    prefix_before_suffix(bytes, start as int, end as int, atom, rest);
    suffix_after_prefix(bytes, start as int, end as int, atom, rest);
    assert(bytes.subrange(start as int + atom.len(), end as int)[0]
        == bytes[start as int + atom.len()]);
    assert(bytes.subrange(start as int + atom.len(), end as int)[0] == 0x28);
    reveal(atom_boundary);
}

proof fn make_comp_guide(
    bytes: Seq<u8>,
    start: usize,
    end: usize,
    name: Seq<u8>,
    args: Seq<Term>,
    atom: &EParsedAtom,
    frame: &ETermFrame,
) -> (guide: GTermFrame)
    requires
        term_at(bytes, start as int, end as int, Term::Comp(name, args)),
        args.len() > 0,
        !(name == ckc_spec::v1text::cons_name() && args.len() == 2),
        !(name == ckc_spec::v1text::curly_name() && args.len() == 1),
        atom.name@ == name,
        atom.end as int == start as int + ckc_spec::v1text::atom_bytes(name).len(),
        match frame {
            ETermFrame::Comp {
                start: s,
                child_start,
                name: frame_name,
                args: built,
                count,
                ground,
                no_dollar,
            } => {
                &&& *s == start
                &&& *child_start == atom.end + 1
                &&& frame_name@ == atom.name@
                &&& built@ == Seq::<Term>::empty()
                &&& *count == 0
                &&& *ground
                &&& *no_dollar
            },
            _ => false,
        },
        frame_ok(bytes, frame),
    ensures
        guide_frame_ok(bytes, frame, guide),
        guide_target(guide) == Term::Comp(name, args),
        guide_end(guide) == end as int,
{
    let guide = GTermFrame::Comp {
        name,
        built: Seq::empty(),
        remaining: args,
        end: end as int,
    };
    reveal(term_at);
    reveal_with_fuel(ckc_spec::v1text::term_bytes, 2);
    let atom_bytes = ckc_spec::v1text::atom_bytes(name);
    let suffix = ckc_spec::v1text::args_bytes(args) + seq![0x29u8];
    assert(bytes.subrange(start as int, end as int)
        == (atom_bytes + seq![0x28u8]) + suffix);
    suffix_after_prefix(
        bytes,
        start as int,
        end as int,
        atom_bytes + seq![0x28u8],
        suffix,
    );
    args_bytes_nonempty(args);
    reveal(guide_frame_ok);
    reveal(guide_target);
    reveal(guide_end);
    match frame {
        ETermFrame::Comp { .. } => {},
        _ => assert(false),
    }
    guide
}

proof fn make_list_guide(
    bytes: Seq<u8>,
    start: usize,
    end: usize,
    term: Term,
    frame: &ETermFrame,
) -> (guide: GTermFrame)
    requires
        term_at(bytes, start as int, end as int, term),
        list_cons(term),
        match frame {
            ETermFrame::List {
                start: s,
                child_start,
                elems,
                count,
                ground,
                no_dollar,
                tail,
            } => {
                &&& *s == start
                &&& *child_start == start + 1
                &&& elems@ == Seq::<Term>::empty()
                &&& *count == 0
                &&& *ground
                &&& *no_dollar
                &&& !*tail
            },
            _ => false,
        },
        frame_ok(bytes, frame),
    ensures
        guide_frame_ok(bytes, frame, guide),
        guide_target(guide) == term,
        guide_end(guide) == end as int,
{
    reveal(term_at);
    list_decompose(term);
    let elems = list_elems(term);
    let tail = list_final_tail(term);
    let guide = GTermFrame::List {
        built: Seq::empty(),
        remaining: elems,
        tail,
        tail_mode: false,
        end: end as int,
    };
    if tail == Term::Nil {
        list_nil_bytes(elems);
        assert(bytes.subrange(start as int, end as int)
            == seq![0x5bu8] + ckc_spec::v1text::args_bytes(elems) + seq![0x5du8]);
        assert_seqs_equal!(bytes.subrange(start as int, end as int)
            == seq![0x5bu8]
                + (ckc_spec::v1text::args_bytes(elems) + seq![0x5du8]));
        suffix_after_prefix(
            bytes,
            start as int,
            end as int,
            seq![0x5bu8],
            ckc_spec::v1text::args_bytes(elems) + seq![0x5du8],
        );
    } else {
        assert(plain_list_tail(tail));
        list_plain_bytes(elems, tail);
        let suffix = ckc_spec::v1text::args_bytes(elems)
            + seq![0x7cu8] + ckc_spec::v1text::term_bytes(tail) + seq![0x5du8];
        assert(bytes.subrange(start as int, end as int) == seq![0x5bu8] + suffix);
        suffix_after_prefix(bytes, start as int, end as int, seq![0x5bu8], suffix);
        assert_seqs_equal!(bytes.subrange(start as int + 1, end as int)
            == ckc_spec::v1text::args_bytes(elems)
                + (seq![0x7cu8] + ckc_spec::v1text::term_bytes(tail) + seq![0x5du8]));
    }
    args_bytes_nonempty(elems);
    reveal(guide_frame_ok);
    reveal(guide_target);
    reveal(guide_end);
    match frame {
        ETermFrame::List { .. } => {},
        _ => assert(false),
    }
    guide
}

proof fn make_curly_guide(
    bytes: Seq<u8>,
    start: usize,
    end: usize,
    child: Term,
    frame: &ETermFrame,
) -> (guide: GTermFrame)
    requires
        term_at(
            bytes,
            start as int,
            end as int,
            Term::Comp(ckc_spec::v1text::curly_name(), seq![child]),
        ),
        match frame {
            ETermFrame::Curly { start: s, child_start } => {
                &&& *s == start
                &&& *child_start == start + 1
            },
            _ => false,
        },
        frame_ok(bytes, frame),
    ensures
        guide_frame_ok(bytes, frame, guide),
        guide_target(guide)
            == Term::Comp(ckc_spec::v1text::curly_name(), seq![child]),
        guide_end(guide) == end as int,
{
    let guide = GTermFrame::Curly { child, end: end as int };
    reveal(term_at);
    reveal_with_fuel(ckc_spec::term::wf_term, 2);
    reveal_with_fuel(ckc_spec::term::wf_terms, 2);
    reveal_with_fuel(ckc_spec::v1text::term_bytes, 2);
    assert(bytes.subrange(start as int, end as int)
        == seq![0x7bu8] + ckc_spec::v1text::term_bytes(child) + seq![0x7du8]);
    assert_seqs_equal!(bytes.subrange(start as int, end as int)
        == seq![0x7bu8]
            + (ckc_spec::v1text::term_bytes(child) + seq![0x7du8]));
    suffix_after_prefix(
        bytes,
        start as int,
        end as int,
        seq![0x7bu8],
        ckc_spec::v1text::term_bytes(child) + seq![0x7du8],
    );
    term_bytes_nonempty(child);
    reveal(guide_frame_ok);
    reveal(guide_target);
    reveal(guide_end);
    match frame {
        ETermFrame::Curly { .. } => {},
        _ => assert(false),
    }
    guide
}

#[verifier::rlimit(5000)]
pub fn parse_term(
    bytes: &[u8],
    start: usize,
    expected: Ghost<Option<GTermExpected>>,
    tracking_expected: Ghost<GTermExpected>,
    initial_stream: Ghost<Seq<nat>>,
    track_vars: bool,
    tracker: &mut EVarTracker,
at: &mut usize,
) -> (r: Option<ESpannedTerm>)
    requires
        *old(at) <= bytes@.len(),
        start < bytes@.len(),
        expected@ matches Some(e) ==>
            term_at(bytes@, start as int, e.end as int, e.term),
        track_vars ==> expected@ == Some(tracking_expected@),
        initial_stream@ == old(tracker).stream@,
        old(tracker).valid ==>
            tracker_state_ok(old(tracker).next, old(tracker).stream@),
        tracker_complete(old(tracker).valid, old(tracker).stream@),
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r matches Some(t) ==> spanned_term_ok(bytes@, &t),
        r matches Some(t) ==> t.start == start,
        expected@ matches Some(e) ==> r matches Some(t)
            && t@ == e.term && t.end == e.end,
        final(tracker).valid ==>
            tracker_state_ok(final(tracker).next, final(tracker).stream@),
        tracker_complete(final(tracker).valid, final(tracker).stream@),
        track_vars ==> match r {
            Some(t) => {
                &&& final(tracker).stream@
                    == initial_stream@ + ckc_spec::term::var_stream(t@)
                &&& initial_stream@.len() <= start
                    ==> final(tracker).stream@.len() <= t.end
            },
            None => true,
        },
{
    let ghost entry_stream = initial_stream@;
    let ghost root = match expected@ {
        Some(e) => e.term,
        None => Term::Nil,
    };
    let ghost root_end = match expected@ {
        Some(e) => e.end as int,
        None => start as int,
    };
    let ghost mut guides: Seq<GTermFrame> = Seq::empty();
    let mut frames: Vec<ETermFrame> = Vec::new();
    let mut current: Option<ESpannedTerm> = None;
    let mut pos = start;
    proof {
        parse_state_initial(bytes@, start);
        tracked_state_initial(entry_stream);
        reveal(current_term);
        if let Some(e) = expected@ {
            assert(root == e.term);
            assert(root_end == e.end as int);
            guided_state_initial(bytes@, start, root, root_end);
        }
    }
    while pos <= bytes.len()
        invariant
            *old(at) <= *at <= bytes@.len(),
            parse_state_ok(bytes@, start, pos, frames@, &current),
            expected@ matches Some(e) ==> {
                &&& root == e.term
                &&& root_end == e.end as int
                &&& guided_state_ok(
                    bytes@,
                    start,
                    pos,
                    frames@,
                    &current,
                    guides,
                    root,
                    root_end,
                )
            },
            entry_stream == initial_stream@,
            tracker.valid ==> tracker_state_ok(tracker.next, tracker.stream@),
            tracker_complete(tracker.valid, tracker.stream@),
            track_vars ==> tracker.stream@.len()
                <= entry_stream.len() + (pos - start),
            track_vars ==> expected@ == Some(tracking_expected@),
            track_vars ==> tracked_parse_state(
                entry_stream,
                tracker.stream@,
                guides,
                current_term(&current),
            ),
        decreases bytes.len() - pos,
    {
        if current.is_some() {
            if frames.len() == 0 {
                let out = current.unwrap();
                proof {
                    reveal(parse_state_ok);
                    assert(spanned_term_ok(bytes@, &out));
                    assert(out.start == start);
                    if let Some(e) = expected@ {
                        reveal(guided_state_ok);
                        reveal(guided_focus);
                        reveal(guided_focus_end);
                        reveal(guide_stack_ok);
                        assert(guides.len() == 0);
                        assert(out@ == root);
                        assert(out.end as int == root_end);
                        assert(out@ == e.term);
                        assert(out.end == e.end);
                    }
                    if track_vars {
                        assert(expected@ == Some(tracking_expected@));
                        reveal(guided_state_ok);
                        reveal(guide_stack_ok);
                        assert(guides.len() == 0);
                        reveal(current_term);
                        assert(current_term(&current) == Some(out@));
                        tracked_state_root(entry_stream, tracker.stream@, out@);
                        assert(pos == out.end);
                        if initial_stream@.len() <= start {
                            assert(tracker.stream@.len()
                                <= entry_stream.len() + (out.end - start));
                            assert(tracker.stream@.len() <= out.end);
                        }
                    }
                }
                return Some(out);
            }
            let ghost old_frames = frames@;
            let ghost old_guides = guides;
            let child = current.unwrap();
            proof {
                if track_vars {
                    reveal(current_term);
                    assert(current_term(&current) == Some(child@));
                    assert(tracked_parse_state(
                        entry_stream,
                        tracker.stream@,
                        old_guides,
                        Some(child@),
                    ));
                }
            }
            current = None;
            let frame = frames.pop().unwrap();
            proof {
                reveal(parse_state_ok);
                assert(frame == old_frames.last());
                assert(frame_ok(bytes@, &frame));
                assert(child.start == frame_child_start(&frame));
                if let Some(_) = expected@ {
                    reveal(guided_state_ok);
                    reveal(guide_stack_ok);
                    assert(old_guides.len() == old_frames.len());
                    assert(old_guides.len() > 0);
                }
            }
            let ghost active_guide = match expected@ {
                Some(_) => Some(old_guides.last()),
                None => None,
            };
            proof {
                if let Some(_) = expected@ {
                    let guide = old_guides.last();
                    assert(active_guide == Some(guide));
                    reveal(guided_state_ok);
                    reveal(guided_focus);
                    reveal(guided_focus_end);
                    reveal(guided_step_pre);
                    assert(child@ == guide_next(guide));
                    assert(child.end as int == guide_next_end(&frame, guide));
                    assert(guide_frame_ok(bytes@, &frame, guide));
                }
            }
            let step = feed_frame(bytes, frame, child, Ghost(active_guide));
            match step {
                EFrameStep::Next(next) => {
                    let next_pos = frame_child_start_exec(&next);
                    proof {
                        reveal(frame_step_ok);
                        parse_state_replace_last(
                            bytes@,
                            start,
                            pos,
                            old_frames,
                            child,
                            next,
                        );
                        if let Some(_) = expected@ {
                            let guide = old_guides.last();
                            assert(guided_step_ok(
                                bytes@,
                                &frame,
                                &child,
                                guide,
                                &EFrameStep::Next(next),
                            ));
                            guided_next_facts(bytes@, &frame, &child, guide, &next);
                            let advanced = advance_guide(guide);
                            let new_guides = old_guides.drop_last().push(advanced);
                            if track_vars {
                                assert(child@ == guide_next(guide));
                                tracked_next_transition(
                                    bytes@,
                                    &frame,
                                    old_guides,
                                    guide,
                                    child@,
                                );
                                tracked_state_next(
                                    entry_stream,
                                    tracker.stream@,
                                    old_guides,
                                    child@,
                                    new_guides,
                                );
                            }
                            reveal(guided_state_ok);
                            guided_state_next(
                                bytes@,
                                start,
                                old_frames,
                                old_guides,
                                next,
                                advanced,
                                root,
                                root_end,
                            );
                            guides = new_guides;
                        } else {
                            guides = old_guides;
                        }
                    }
                    frames.push(next);
                    proof {
                        if track_vars {
                            reveal(current_term);
                            assert(current_term(&current) == None);
                            assert(tracked_parse_state(
                                entry_stream,
                                tracker.stream@,
                                guides,
                                None,
                            ));
                        }
                    }
                    pos = next_pos;
                },
                EFrameStep::Done(done) => {
                    let next_pos = done.end;
                    let dollar_unary = match &done.parsed.top {
                        ETermTop::Comp(name, arity) => {
                            is_dollar_name(name.as_slice()) && *arity == 1
                        },
                        _ => false,
                    };
                    if dollar_unary {
                        proof {
                            reveal(frame_step_ok);
                            reveal(spanned_term_ok);
                            assert(done.end > 0);
                        }
                        raise_at(at, done.end - 1, bytes.len());
                    }
                    proof {
                        reveal(frame_step_ok);
                        parse_state_close_last(
                            bytes@,
                            start,
                            pos,
                            old_frames,
                            child,
                            done,
                        );
                        if let Some(_) = expected@ {
                            let guide = old_guides.last();
                            assert(guided_step_ok(
                                bytes@,
                                &frame,
                                &child,
                                guide,
                                &EFrameStep::Done(done),
                            ));
                            reveal(guided_step_ok);
                            assert(done@ == guide_target(guide));
                            assert(done.end as int == guide_end(guide));
                            if track_vars {
                                let new_guides = old_guides.drop_last();
                                assert(child@ == guide_next(guide));
                                reveal(guide_has_next);
                                assert(!guide_has_next(guide));
                                tracked_done_transition(
                                    bytes@,
                                    &frame,
                                    old_guides,
                                    guide,
                                    child@,
                                    done@,
                                );
                                tracked_state_done(
                                    entry_stream,
                                    tracker.stream@,
                                    old_guides,
                                    child@,
                                    new_guides,
                                    done@,
                                );
                            }
                            reveal(guided_state_ok);
                            guided_state_done(
                                bytes@,
                                start,
                                old_frames,
                                old_guides,
                                done,
                                root,
                                root_end,
                            );
                            guides = old_guides.drop_last();
                        } else {
                            guides = old_guides;
                        }
                    }
                    current = Some(done);
                    proof {
                        if track_vars {
                            reveal(current_term);
                            assert(current_term(&current) == Some(done@));
                            assert(tracked_parse_state(
                                entry_stream,
                                tracker.stream@,
                                guides,
                                Some(done@),
                            ));
                        }
                    }
                    pos = next_pos;
                },
                EFrameStep::Reject => {
                    raise_at(at, pos, bytes.len());
                    proof {
                        if let Some(_) = expected@ {
                            let guide = old_guides.last();
                            assert(guided_step_ok(
                                bytes@,
                                &frame,
                                &child,
                                guide,
                                &EFrameStep::Reject,
                            ));
                            reveal(guided_step_ok);
                            assert(false);
                        }
                    }
                    return None;
                },
            }
        } else {
            let ghost focus = guided_focus(guides, root);
            let ghost focus_end = guided_focus_end(frames@, guides, root_end);
            proof {
                if let Some(_) = expected@ {
                    reveal(guided_state_ok);
                    assert(term_at(bytes@, pos as int, focus_end, focus));
                    expected_term_openers(bytes@, pos, focus_end as usize, focus);
                    expected_term_scalar_class(bytes@, pos, focus_end as usize, focus);
                }
            }
            if pos == bytes.len() {
                raise_at(at, pos, bytes.len());
                proof {
                    if let Some(_) = expected@ {
                        reveal(term_at);
                        assert(false);
                    }
                }
                return None;
            }
            if bytes[pos] == 0x5b
                && bytes.len() - pos >= 2
                && bytes[pos + 1] != 0x5d
            {
                let frame = open_list_frame(bytes, pos);
                let next_pos = frame_child_start_exec(&frame);
                proof {
                    parse_state_push(bytes@, start, pos, frames@, frame);
                    if let Some(_) = expected@ {
                        reveal(list_open_at);
                        assert(list_cons(focus));
                        let guide = make_list_guide(
                            bytes@,
                            pos,
                            focus_end as usize,
                            focus,
                            &frame,
                        );
                        let new_guides = guides.push(guide);
                        if track_vars {
                            reveal(current_term);
                            assert(current_term(&current) == None);
                            assert(tracked_parse_state(
                                entry_stream,
                                tracker.stream@,
                                guides,
                                None,
                            ));
                            reveal(guide_var_prefix);
                            assert(guide_var_prefix(guide) == Seq::<nat>::empty());
                            tracked_state_push(
                                entry_stream,
                                tracker.stream@,
                                guides,
                                guide,
                            );
                        }
                        guided_state_push(
                            bytes@,
                            start,
                            pos,
                            frames@,
                            guides,
                            frame,
                            guide,
                            root,
                            root_end,
                        );
                        guides = new_guides;
                    }
                }
                frames.push(frame);
                proof {
                    if track_vars {
                        reveal(current_term);
                        assert(current_term(&current) == None);
                        assert(tracked_parse_state(
                            entry_stream,
                            tracker.stream@,
                            guides,
                            None,
                        ));
                    }
                }
                pos = next_pos;
            } else if bytes[pos] == 0x7b
                && bytes.len() - pos >= 2
                && bytes[pos + 1] != 0x7d
            {
                let frame = open_curly_frame(bytes, pos);
                let next_pos = frame_child_start_exec(&frame);
                proof {
                    parse_state_push(bytes@, start, pos, frames@, frame);
                    if let Some(_) = expected@ {
                        reveal(curly_open_at);
                        assert(curly_comp(focus));
                        reveal(curly_comp);
                        match focus {
                            Term::Comp(name, args) => {
                                assert(name == ckc_spec::v1text::curly_name());
                                assert(args.len() == 1);
                                assert_seqs_equal!(args == seq![args[0]]);
                                assert(focus == Term::Comp(
                                    ckc_spec::v1text::curly_name(),
                                    seq![args[0]],
                                ));
                                let guide = make_curly_guide(
                                    bytes@,
                                    pos,
                                    focus_end as usize,
                                    args[0],
                                    &frame,
                                );
                                let new_guides = guides.push(guide);
                                if track_vars {
                                    reveal(current_term);
                                    assert(current_term(&current) == None);
                                    assert(tracked_parse_state(
                                        entry_stream,
                                        tracker.stream@,
                                        guides,
                                        None,
                                    ));
                                    reveal(guide_var_prefix);
                                    assert(guide_var_prefix(guide)
                                        == Seq::<nat>::empty());
                                    tracked_state_push(
                                        entry_stream,
                                        tracker.stream@,
                                        guides,
                                        guide,
                                    );
                                }
                                guided_state_push(
                                    bytes@,
                                    start,
                                    pos,
                                    frames@,
                                    guides,
                                    frame,
                                    guide,
                                    root,
                                    root_end,
                                );
                                guides = new_guides;
                            },
                            _ => assert(false),
                        }
                    }
                }
                frames.push(frame);
                proof {
                    if track_vars {
                        reveal(current_term);
                        assert(current_term(&current) == None);
                        assert(tracked_parse_state(
                            entry_stream,
                            tracker.stream@,
                            guides,
                            None,
                        ));
                    }
                }
                pos = next_pos;
            } else if (0x41 <= bytes[pos] && bytes[pos] <= 0x5a)
                || is_digit_b(bytes[pos])
                || bytes[pos] == 0x2d
                    && pos + 1 < bytes.len()
                    && is_digit_b(bytes[pos + 1])
                || bytes[pos] == 0x5b
                    && pos + 1 < bytes.len()
                    && bytes[pos + 1] == 0x5d
            {
                let ghost expected_atomic = match expected@ {
                    Some(_) => Some(GAtomicExpected {
                        term: focus,
                        end: focus_end as usize,
                    }),
                    None => None,
                };
                proof {
                    if let Some(_) = expected@ {
                        reveal(scalar_nonatom_at);
                        reveal(scalar_nonatom_term);
                        assert(scalar_nonatom_term(focus));
                        reveal(atomic_term);
                    }
                }
                let term = match parse_atomic(bytes, pos, Ghost(expected_atomic), at) {
                    Some(t) => t,
                    None => return None,
                };
                if track_vars {
                    let ghost atomic_initial = tracker.stream@;
                    proof {
                        reveal(current_term);
                        assert(current_term(&current) == None);
                        assert(tracked_parse_state(
                            entry_stream,
                            atomic_initial,
                            guides,
                            None,
                        ));
                        assert(expected@ == Some(tracking_expected@));
                        assert(expected_atomic == Some(GAtomicExpected {
                            term: focus,
                            end: focus_end as usize,
                        }));
                        assert(spanned_term_ok(bytes@, &term));
                        assert(term.start == pos);
                        assert(term@ == focus);
                        assert(term.end as int == focus_end);
                        assert(term_at(bytes@, pos as int, term.end as int, term@));
                        reveal(atomic_term);
                        assert(atomic_term(term@));
                    }
                    record_atomic_term(
                        bytes,
                        pos,
                        &term,
                        Ghost(atomic_initial),
                        tracker,
                    at,
                    );
                    proof {
                        tracked_state_atomic(
                            entry_stream,
                            atomic_initial,
                            tracker.stream@,
                            guides,
                            term@,
                        );
                        atomic_var_stream_len(term@, pos, term.end);
                        assert(tracker.stream@.len()
                            == atomic_initial.len()
                                + ckc_spec::term::var_stream(term@).len());
                        assert(tracker.stream@.len()
                            <= entry_stream.len() + (term.end - start));
                    }
                }
                let next_pos = term.end;
                proof {
                    parse_state_current(bytes@, start, pos, frames@, term);
                    if let Some(_) = expected@ {
                        assert(term@ == focus);
                        assert(term.end as int == focus_end);
                        guided_state_current(
                            bytes@,
                            start,
                            pos,
                            frames@,
                            guides,
                            term,
                            root,
                            root_end,
                        );
                    }
                }
                current = Some(term);
                proof {
                    if track_vars {
                        reveal(current_term);
                        assert(current_term(&current) == Some(term@));
                        assert(tracked_parse_state(
                            entry_stream,
                            tracker.stream@,
                            guides,
                            Some(term@),
                        ));
                    }
                }
                pos = next_pos;
            } else {
                proof {
                    if let Some(_) = expected@ {
                        reveal(scalar_nonatom_at);
                        reveal(scalar_nonatom_term);
                        assert(!scalar_nonatom_term(focus));
                        reveal(list_open_at);
                        reveal(curly_open_at);
                        assert(!list_cons(focus));
                        assert(!curly_comp(focus));
                    }
                }
                let ghost expected_atom = match expected@ {
                    Some(_) => match focus {
                        Term::Atom(name) => Some(GAtomExpected {
                            name,
                            end: focus_end as usize,
                        }),
                        Term::Comp(name, args) => Some(GAtomExpected {
                            name,
                            end: (pos as int
                                + ckc_spec::v1text::atom_bytes(name).len()) as usize,
                        }),
                        _ => None,
                    },
                    None => None,
                };
                proof {
                    if let Some(_) = expected@ {
                        match focus {
                            Term::Atom(_) => {
                                reveal(term_at);
                                reveal_with_fuel(ckc_spec::v1text::term_bytes, 2);
                                reveal(atom_boundary);
                            },
                            Term::Comp(name, args) => {
                                reveal_with_fuel(ckc_spec::term::wf_term, 2);
                                assert(args.len() > 0);
                                assert(!(name == ckc_spec::v1text::cons_name()
                                    && args.len() == 2));
                                assert(!(name == ckc_spec::v1text::curly_name()
                                    && args.len() == 1));
                                ordinary_comp_functor_ready(
                                    bytes@,
                                    pos,
                                    focus_end as usize,
                                    name,
                                    args,
                                );
                            },
                            _ => assert(false),
                        }
                    }
                }
                if bytes[pos] == 0x5b && bytes.len() - pos < 2 {
                    raise_at(at, bytes.len(), bytes.len());
                }
                let atom = match parse_atom(bytes, pos, Ghost(expected_atom), at) {
                    Some(a) => a,
                    None => return None,
                };
                if atom.end < bytes.len() && bytes[atom.end] == 0x28 {
                    proof {
                        if let Some(_) = expected@ {
                            match focus {
                                Term::Comp(_, _) => {},
                                Term::Atom(_) => {
                                    assert(atom.end as int == focus_end);
                                    reveal(term_at);
                                    reveal(term_boundary);
                                    assert(false);
                                },
                                _ => assert(false),
                            }
                        }
                    }
                    let frame = open_comp_frame(bytes, pos, atom);
                    let next_pos = frame_child_start_exec(&frame);
                    proof {
                        parse_state_push(bytes@, start, pos, frames@, frame);
                        if let Some(_) = expected@ {
                            match focus {
                                Term::Comp(name, args) => {
                                    let guide = make_comp_guide(
                                        bytes@,
                                        pos,
                                        focus_end as usize,
                                        name,
                                        args,
                                        &atom,
                                        &frame,
                                    );
                                    let new_guides = guides.push(guide);
                                    if track_vars {
                                        reveal(current_term);
                                        assert(current_term(&current) == None);
                                        assert(tracked_parse_state(
                                            entry_stream,
                                            tracker.stream@,
                                            guides,
                                            None,
                                        ));
                                        reveal(guide_var_prefix);
                                        assert(guide_var_prefix(guide)
                                            == Seq::<nat>::empty());
                                        tracked_state_push(
                                            entry_stream,
                                            tracker.stream@,
                                            guides,
                                            guide,
                                        );
                                    }
                                    guided_state_push(
                                        bytes@,
                                        start,
                                        pos,
                                        frames@,
                                        guides,
                                        frame,
                                        guide,
                                        root,
                                        root_end,
                                    );
                                    guides = new_guides;
                                },
                                _ => assert(false),
                            }
                        }
                    }
                    frames.push(frame);
                    proof {
                        if track_vars {
                            reveal(current_term);
                            assert(current_term(&current) == None);
                            assert(tracked_parse_state(
                                entry_stream,
                                tracker.stream@,
                                guides,
                                None,
                            ));
                        }
                    }
                    pos = next_pos;
                } else {
                    proof {
                        if let Some(_) = expected@ {
                            match focus {
                                Term::Atom(_) => {},
                                Term::Comp(name, args) => {
                                    ordinary_comp_functor_ready(
                                        bytes@,
                                        pos,
                                        focus_end as usize,
                                        name,
                                        args,
                                    );
                                    assert(atom.end as int
                                        == pos as int
                                            + ckc_spec::v1text::atom_bytes(name).len());
                                    assert(atom.end < bytes.len());
                                    assert(bytes@[atom.end as int] == 0x28);
                                    assert(false);
                                },
                                _ => assert(false),
                            }
                        }
                    }
                    let term = span_atom(bytes, pos, atom);
                    let next_pos = term.end;
                    proof {
                        parse_state_current(bytes@, start, pos, frames@, term);
                        if track_vars {
                            reveal(current_term);
                            assert(current_term(&current) == None);
                            assert(tracked_parse_state(
                                entry_stream,
                                tracker.stream@,
                                guides,
                                None,
                            ));
                            reveal(ckc_spec::term::var_stream);
                            assert_seqs_equal!(
                                tracker.stream@
                                    + ckc_spec::term::var_stream(term@)
                                == tracker.stream@
                            );
                            tracked_state_atomic(
                                entry_stream,
                                tracker.stream@,
                                tracker.stream@,
                                guides,
                                term@,
                            );
                        }
                        if let Some(_) = expected@ {
                            assert(term@ == focus);
                            assert(term.end as int == focus_end);
                            guided_state_current(
                                bytes@,
                                start,
                                pos,
                                frames@,
                                guides,
                                term,
                                root,
                                root_end,
                            );
                        }
                    }
                    current = Some(term);
                    proof {
                        if track_vars {
                            reveal(current_term);
                            assert(current_term(&current) == Some(term@));
                            assert(tracked_parse_state(
                                entry_stream,
                                tracker.stream@,
                                guides,
                                Some(term@),
                            ));
                        }
                    }
                    pos = next_pos;
                }
            }
        }
    }
    None
}

proof fn udec_decimal_value(n: nat)
    ensures decimal_value(ckc_spec::v1text::udec_bytes(n)) == n,
    decreases n,
{
    reveal_with_fuel(ckc_spec::v1text::udec_bytes, 2);
    reveal_with_fuel(decimal_value, 2);
    reveal(decimal_digit);
    reveal(ckc_spec::v1text::digit_byte);
    if n < 10 {
        assert(ckc_spec::v1text::udec_bytes(n)
            == seq![ckc_spec::v1text::digit_byte(n as int)]);
    } else {
        let q = n / 10;
        let d = n % 10;
        assert(q < n);
        udec_decimal_value(q);
        let prefix = ckc_spec::v1text::udec_bytes(q);
        let digit = ckc_spec::v1text::digit_byte(d as int);
        assert((prefix + seq![digit]).last() == digit);
        assert_seqs_equal!((prefix + seq![digit]).drop_last() == prefix);
        lemma_fundamental_div_mod(n as int, 10);
        assert(n == 10 * q + d);
    }
}

proof fn udec_bytes_injective(a: nat, b: nat)
    requires ckc_spec::v1text::udec_bytes(a) == ckc_spec::v1text::udec_bytes(b),
    ensures a == b,
{
    udec_decimal_value(a);
    udec_decimal_value(b);
}

proof fn var_bytes_injective(a: nat, b: nat)
    requires ckc_spec::v1text::var_bytes(a) == ckc_spec::v1text::var_bytes(b),
    ensures a == b,
{
    let qa = a / 26;
    let qb = b / 26;
    let ra = a % 26;
    let rb = b % 26;
    let la = (0x41 + ra) as u8;
    let lb = (0x41 + rb) as u8;
    assert(ra < 26);
    assert(rb < 26);
    reveal(ckc_spec::v1text::var_bytes);
    assert(ckc_spec::v1text::var_bytes(a)[0] == la);
    assert(ckc_spec::v1text::var_bytes(b)[0] == lb);
    assert(la == lb);
    assert(ra == rb);
    if qa == 0 {
        if qb > 0 {
            udec_bytes_nonempty(qb);
            assert(ckc_spec::v1text::var_bytes(a).len() == 1);
            assert(ckc_spec::v1text::var_bytes(b).len()
                == 1 + ckc_spec::v1text::udec_bytes(qb).len());
            assert(false);
        }
    } else {
        if qb == 0 {
            udec_bytes_nonempty(qa);
            assert(ckc_spec::v1text::var_bytes(a).len()
                == 1 + ckc_spec::v1text::udec_bytes(qa).len());
            assert(ckc_spec::v1text::var_bytes(b).len() == 1);
            assert(false);
        }
        assert(qa > 0 && qb > 0);
        let va = ckc_spec::v1text::var_bytes(a);
        let vb = ckc_spec::v1text::var_bytes(b);
        assert(va == seq![la] + ckc_spec::v1text::udec_bytes(qa));
        assert(vb == seq![lb] + ckc_spec::v1text::udec_bytes(qb));
        assert_seqs_equal!(va.subrange(0, va.len() as int) == va);
        assert_seqs_equal!(vb.subrange(0, vb.len() as int) == vb);
        assert(va.subrange(0, va.len() as int)
            == seq![la] + ckc_spec::v1text::udec_bytes(qa));
        assert(vb.subrange(0, vb.len() as int)
            == seq![lb] + ckc_spec::v1text::udec_bytes(qb));
        suffix_after_prefix(
            va,
            0,
            va.len() as int,
            seq![la],
            ckc_spec::v1text::udec_bytes(qa),
        );
        suffix_after_prefix(
            vb,
            0,
            vb.len() as int,
            seq![lb],
            ckc_spec::v1text::udec_bytes(qb),
        );
        assert(va == vb);
        assert(va.len() == vb.len());
        assert(va.subrange(1, va.len() as int)
            == vb.subrange(1, vb.len() as int));
        assert_seqs_equal!(ckc_spec::v1text::udec_bytes(qa)
            == ckc_spec::v1text::udec_bytes(qb));
        udec_bytes_injective(qa, qb);
    }
    assert(qa == qb);
    lemma_fundamental_div_mod(a as int, 26);
    lemma_fundamental_div_mod(b as int, 26);
    assert(a == 26 * qa + ra);
    assert(b == 26 * qb + rb);
}

fn match_udec_usize(
    bytes: &[u8],
    start: usize,
    end: usize,
    n: usize,
) -> (r: bool)
    requires start <= end <= bytes@.len(),
    ensures r == (ckc_spec::v1text::udec_bytes(n as nat)
        == bytes@.subrange(start as int, end as int)),
    decreases n,
{
    if n < 10 {
        let digit = 0x30u8 + n as u8;
        if end - start != 1 {
            proof {
                reveal_with_fuel(ckc_spec::v1text::udec_bytes, 2);
                reveal(ckc_spec::v1text::digit_byte);
                assert(ckc_spec::v1text::udec_bytes(n as nat).len() == 1);
                assert(bytes@.subrange(start as int, end as int).len() == end - start);
            }
            return false;
        }
        let same = bytes[start] == digit;
        proof {
            reveal_with_fuel(ckc_spec::v1text::udec_bytes, 2);
            reveal(ckc_spec::v1text::digit_byte);
            assert(ckc_spec::v1text::udec_bytes(n as nat) == seq![digit]);
            assert_seqs_equal!(bytes@.subrange(start as int, end as int)
                == seq![bytes@[start as int]]);
            if same {
                assert(bytes@[start as int] == digit);
                assert(ckc_spec::v1text::udec_bytes(n as nat)
                    == bytes@.subrange(start as int, end as int));
            } else {
                assert(bytes@[start as int] != digit);
                if ckc_spec::v1text::udec_bytes(n as nat)
                    == bytes@.subrange(start as int, end as int)
                {
                    assert(ckc_spec::v1text::udec_bytes(n as nat)[0] == digit);
                    assert(bytes@.subrange(start as int, end as int)[0]
                        == bytes@[start as int]);
                    assert(false);
                }
            }
        }
        return same;
    }
    if end == start {
        proof { udec_bytes_nonempty(n as nat); }
        return false;
    }
    let q = n / 10;
    let d = n % 10;
    let digit = 0x30u8 + d as u8;
    if bytes[end - 1] != digit {
        proof {
            reveal_with_fuel(ckc_spec::v1text::udec_bytes, 2);
            reveal(ckc_spec::v1text::digit_byte);
            assert(ckc_spec::v1text::udec_bytes(n as nat).last() == digit);
            assert(bytes@.subrange(start as int, end as int).last()
                == bytes@[end as int - 1]);
        }
        return false;
    }
    let prefix_ok = match_udec_usize(bytes, start, end - 1, q);
    proof {
        assert(q < n);
        reveal_with_fuel(ckc_spec::v1text::udec_bytes, 2);
        reveal(ckc_spec::v1text::digit_byte);
        subrange_push(bytes@, start as int, end as int - 1);
        assert(bytes@.subrange(start as int, end as int)
            == bytes@.subrange(start as int, end as int - 1).push(digit));
        assert(ckc_spec::v1text::udec_bytes(n as nat)
            == ckc_spec::v1text::udec_bytes(q as nat).push(digit));
        if prefix_ok {
            assert(ckc_spec::v1text::udec_bytes(q as nat)
                == bytes@.subrange(start as int, end as int - 1));
        } else if ckc_spec::v1text::udec_bytes(n as nat)
            == bytes@.subrange(start as int, end as int)
        {
            assert_seqs_equal!(ckc_spec::v1text::udec_bytes(q as nat)
                == bytes@.subrange(start as int, end as int - 1));
            assert(false);
        }
    }
    prefix_ok
}

fn match_var_index(
    bytes: &[u8],
    start: usize,
    end: usize,
    index: usize,
) -> (r: bool)
    requires start < end <= bytes@.len(),
    ensures r == (ckc_spec::v1text::var_bytes(index as nat)
        == bytes@.subrange(start as int, end as int)),
{
    let rem = index % 26;
    let q = index / 26;
    let letter = 0x41u8 + rem as u8;
    proof {
        assert(rem < 26);
        reveal(ckc_spec::v1text::var_bytes);
        assert(ckc_spec::v1text::var_bytes(index as nat)[0] == letter);
        assert(bytes@.subrange(start as int, end as int)[0] == bytes@[start as int]);
    }
    if bytes[start] != letter {
        return false;
    }
    if q == 0 {
        let same = end == start + 1;
        proof {
            reveal(ckc_spec::v1text::var_bytes);
            if same {
                assert_seqs_equal!(bytes@.subrange(start as int, end as int)
                    == seq![letter]);
            } else {
                assert(ckc_spec::v1text::var_bytes(index as nat).len() == 1);
                assert(bytes@.subrange(start as int, end as int).len() == end - start);
            }
        }
        return same;
    }
    if end == start + 1 {
        proof {
            reveal(ckc_spec::v1text::var_bytes);
            udec_bytes_nonempty(q as nat);
            assert(ckc_spec::v1text::var_bytes(index as nat).len()
                == 1 + ckc_spec::v1text::udec_bytes(q as nat).len());
            assert(ckc_spec::v1text::var_bytes(index as nat).len() > 1);
            assert(bytes@.subrange(start as int, end as int).len() == 1);
        }
        return false;
    }
    let suffix_ok = match_udec_usize(bytes, start + 1, end, q);
    proof {
        reveal(ckc_spec::v1text::var_bytes);
        assert(ckc_spec::v1text::var_bytes(index as nat)
            == seq![letter] + ckc_spec::v1text::udec_bytes(q as nat));
        assert_seqs_equal!(bytes@.subrange(start as int, end as int)
            == seq![letter] + bytes@.subrange(start as int + 1, end as int));
        if suffix_ok {
            assert(ckc_spec::v1text::udec_bytes(q as nat)
                == bytes@.subrange(start as int + 1, end as int));
            assert(ckc_spec::v1text::var_bytes(index as nat)
                == bytes@.subrange(start as int, end as int));
        } else if ckc_spec::v1text::var_bytes(index as nat)
            == bytes@.subrange(start as int, end as int)
        {
            assert(bytes@.subrange(start as int, end as int)
                == seq![letter] + ckc_spec::v1text::udec_bytes(q as nat));
            suffix_after_prefix(
                bytes@,
                start as int,
                end as int,
                seq![letter],
                ckc_spec::v1text::udec_bytes(q as nat),
            );
            assert(ckc_spec::v1text::udec_bytes(q as nat)
                == bytes@.subrange(start as int + 1, end as int));
            assert(false);
        }
    }
    suffix_ok
}

pub open spec fn seen_after(values: Seq<nat>, seen: Set<nat>) -> Set<nat>
    decreases values.len(),
{
    if values.len() == 0 {
        seen
    } else {
        seen_after(values.drop_first(), seen.insert(values[0]))
    }
}

proof fn seen_after_push(values: Seq<nat>, seen: Set<nat>, value: nat)
    ensures seen_after(values.push(value), seen)
        == seen_after(values, seen).insert(value),
    decreases values.len(),
{
    if values.len() == 0 {
        assert_seqs_equal!(values.push(value) == seq![value]);
        reveal_with_fuel(seen_after, 2);
    } else {
        let next_seen = seen.insert(values[0]);
        seen_after_push(values.drop_first(), next_seen, value);
        assert_seqs_equal!(values.push(value).drop_first()
            == values.drop_first().push(value));
        reveal_with_fuel(seen_after, 2);
    }
}

proof fn firsts_push(values: Seq<nat>, seen: Set<nat>, value: nat)
    ensures ckc_spec::term::firsts(values.push(value), seen) ==
        if seen_after(values, seen).contains(value) {
            ckc_spec::term::firsts(values, seen)
        } else {
            ckc_spec::term::firsts(values, seen).push(value)
        },
    decreases values.len(),
{
    if values.len() == 0 {
        assert_seqs_equal!(values.push(value) == seq![value]);
        reveal_with_fuel(seen_after, 2);
        reveal_with_fuel(ckc_spec::term::firsts, 2);
    } else {
        let head = values[0];
        let rest = values.drop_first();
        let next_seen = seen.insert(head);
        firsts_push(rest, next_seen, value);
        assert_seqs_equal!(values.push(value).drop_first() == rest.push(value));
        reveal_with_fuel(seen_after, 2);
        reveal_with_fuel(ckc_spec::term::firsts, 2);
        if seen.contains(head) {
            assert_sets_equal!(next_seen == seen);
        } else if seen_after(values, seen).contains(value) {
            assert(seen_after(rest, next_seen).contains(value));
        } else {
            assert(!seen_after(rest, next_seen).contains(value));
            assert_seqs_equal!(
                (seq![head] + ckc_spec::term::firsts(rest, next_seen)).push(value)
                == seq![head]
                    + ckc_spec::term::firsts(rest, next_seen).push(value)
            );
        }
    }
}

pub open spec fn nat_prefix(n: nat) -> Seq<nat> {
    Seq::new(n, |i: int| i as nat)
}

proof fn nat_prefix_push(n: nat)
    ensures nat_prefix(n + 1) == nat_prefix(n).push(n),
{
    reveal(nat_prefix);
    assert_seqs_equal!(nat_prefix(n + 1) == nat_prefix(n).push(n));
}

pub open spec fn canonical_seen(next: nat) -> Set<nat>
    decreases next,
{
    if next == 0 {
        Set::empty()
    } else {
        canonical_seen((next - 1) as nat).insert((next - 1) as nat)
    }
}

proof fn canonical_seen_contains(next: nat, value: nat)
    ensures canonical_seen(next).contains(value) == (value < next),
    decreases next,
{
    reveal_with_fuel(canonical_seen, 2);
    if next > 0 {
        canonical_seen_contains((next - 1) as nat, value);
    }
}

proof fn canonical_seen_insert(next: nat)
    ensures canonical_seen(next).insert(next) == canonical_seen(next + 1),
{
    reveal_with_fuel(canonical_seen, 2);
}

pub open spec fn tracker_state_ok(next: usize, stream: Seq<nat>) -> bool {
    &&& ckc_spec::term::firsts(stream, Set::empty()) == nat_prefix(next as nat)
    &&& seen_after(stream, Set::empty()) == canonical_seen(next as nat)
}

pub struct EVarTracker {
    pub next: usize,
    pub stream: Ghost<Seq<nat>>,
    pub valid: bool,
}

fn new_var_tracker() -> (tracker: EVarTracker)
    ensures tracker.next == 0,
        tracker.stream@ == Seq::<nat>::empty(),
        tracker.valid,
        tracker_state_ok(tracker.next, tracker.stream@),
        tracker_complete(tracker.valid, tracker.stream@),
{
    proof {
        reveal(tracker_state_ok);
        reveal_with_fuel(ckc_spec::term::firsts, 2);
        reveal_with_fuel(seen_after, 2);
        reveal(nat_prefix);
        reveal_with_fuel(canonical_seen, 2);
        assert(ckc_spec::term::firsts(Seq::<nat>::empty(), Set::empty())
            == Seq::<nat>::empty());
        assert(seen_after(Seq::<nat>::empty(), Set::empty()) == Set::empty());
        assert(nat_prefix(0) == Seq::<nat>::empty());
        assert(canonical_seen(0) == Set::empty());
        assert(tracker_state_ok(0, Seq::<nat>::empty()));
    }
    EVarTracker { next: 0, stream: Ghost(Seq::empty()), valid: true }
}

proof fn tracker_append_existing(next: usize, stream: Seq<nat>, value: nat)
    requires
        tracker_state_ok(next, stream),
        value < next,
    ensures tracker_state_ok(next, stream.push(value)),
{
    reveal(tracker_state_ok);
    canonical_seen_contains(next as nat, value);
    assert(canonical_seen(next as nat).contains(value));
    assert(seen_after(stream, Set::empty()).contains(value));
    firsts_push(stream, Set::empty(), value);
    seen_after_push(stream, Set::empty(), value);
    assert_sets_equal!(canonical_seen(next as nat).insert(value)
        == canonical_seen(next as nat));
}

proof fn tracker_append_new(next: usize, stream: Seq<nat>)
    requires
        tracker_state_ok(next, stream),
        next < usize::MAX,
    ensures tracker_state_ok((next as int + 1) as usize, stream.push(next as nat)),
{
    reveal(tracker_state_ok);
    canonical_seen_contains(next as nat, next as nat);
    assert(!canonical_seen(next as nat).contains(next as nat));
    assert(!seen_after(stream, Set::empty()).contains(next as nat));
    firsts_push(stream, Set::empty(), next as nat);
    seen_after_push(stream, Set::empty(), next as nat);
    nat_prefix_push(next as nat);
    canonical_seen_insert(next as nat);
}


fn raise_variable_reject(
    bytes: &[u8],
    start: usize,
    end: usize,
    next: usize,
    at: &mut usize,
)
    requires
        start < end <= bytes@.len(),
        *old(at) <= bytes@.len(),
    ensures *old(at) <= *final(at) <= bytes@.len(),
{
    let letter = bytes[start];
    if letter < 0x41 || letter > 0x5a {
        raise_at(at, start, bytes.len());
        return;
    }
    let rem = (letter - 0x41) as usize;
    if rem > next {
        raise_at(at, start, bytes.len());
        return;
    }
    if end == start + 1 {
        raise_at(at, end, bytes.len());
        return;
    }
    let max_q = (next - rem) / 26;
    let mut q = 0usize;
    let mut pos = start + 1;
    while pos < end
        invariant
            start < pos <= end <= bytes@.len(),
            q <= max_q,
            *old(at) <= *at <= bytes@.len(),
        decreases end - pos,
    {
        if !is_digit_b(bytes[pos]) {
            raise_at(at, pos, bytes.len());
            return;
        }
        let d = (bytes[pos] - 0x30) as usize;
        if pos == start + 1 && d == 0 {
            raise_at(at, pos, bytes.len());
            return;
        }
        if d > max_q || q > (max_q - d) / 10 {
            raise_at(at, pos, bytes.len());
            return;
        }
        q = q * 10 + d;
        pos += 1;
    }
    raise_at(at, end, bytes.len());
}

fn observe_variable(
    bytes: &[u8],
    start: usize,
    end: usize,
    value: Ghost<nat>,
    tracker: &mut EVarTracker,
at: &mut usize,
) -> (r: bool)
    requires
        *old(at) <= bytes@.len(),
        start < end <= bytes@.len(),
        ckc_spec::v1text::var_bytes(value@)
            == bytes@.subrange(start as int, end as int),
        tracker_state_ok(old(tracker).next, old(tracker).stream@),
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r == (value@ < old(tracker).next
            || value@ == old(tracker).next && old(tracker).next < usize::MAX),
        r ==> {
            &&& tracker_state_ok(final(tracker).next, final(tracker).stream@)
            &&& final(tracker).stream@ == old(tracker).stream@.push(value@)
            &&& final(tracker).next == if value@ < old(tracker).next {
                old(tracker).next
            } else {
                (old(tracker).next as int + 1) as usize
            }
        },
        !r ==> final(tracker).next == old(tracker).next
            && final(tracker).stream@ == old(tracker).stream@,
        final(tracker).valid == old(tracker).valid,
{
    let ghost old_stream = tracker.stream@;
    let old_next = tracker.next;
    let mut i = 0usize;
    while i < old_next
        invariant
            *old(at) <= *at <= bytes@.len(),
            start < end <= bytes@.len(),
            ckc_spec::v1text::var_bytes(value@)
                == bytes@.subrange(start as int, end as int),
            i <= old_next,
            old_next == old(tracker).next,
            old_stream == old(tracker).stream@,
            tracker.next == old_next,
            tracker.stream@ == old_stream,
            tracker.valid == old(tracker).valid,
            tracker_state_ok(old_next, old_stream),
            i as nat <= value@,
        decreases old_next - i,
    {
        if match_var_index(bytes, start, end, i) {
            proof {
                assert(ckc_spec::v1text::var_bytes(i as nat)
                    == bytes@.subrange(start as int, end as int));
                var_bytes_injective(value@, i as nat);
                assert(value@ == i as nat);
                tracker_append_existing(old_next, old_stream, value@);
            }
            tracker.stream = Ghost(old_stream.push(value@));
            proof {
                assert(old_next == old(tracker).next);
                assert(old_stream == old(tracker).stream@);
                assert(value@ < old_next);
                assert(tracker.next == old(tracker).next);
                assert(tracker.stream@ == old(tracker).stream@.push(value@));
                assert(tracker_state_ok(tracker.next, tracker.stream@));
            }
            return true;
        }
        proof {
            if value@ == i as nat {
                assert(ckc_spec::v1text::var_bytes(i as nat)
                    == bytes@.subrange(start as int, end as int));
                assert(false);
            }
            assert(value@ != i as nat);
            assert((i as nat) < value@);
        }
        i += 1;
    }
    proof {
        if value@ < old_next {
            assert(value@ < i);
            assert(false);
        }
    }
    if old_next == usize::MAX {
        raise_variable_reject(bytes, start, end, old_next, at);
        return false;
    }
    if match_var_index(bytes, start, end, old_next) {
        proof {
            assert(ckc_spec::v1text::var_bytes(old_next as nat)
                == bytes@.subrange(start as int, end as int));
            var_bytes_injective(value@, old_next as nat);
            assert(value@ == old_next as nat);
            tracker_append_new(old_next, old_stream);
        }
        tracker.next += 1;
        tracker.stream = Ghost(old_stream.push(value@));
        return true;
    }
    proof {
        if value@ == old_next as nat {
            assert(ckc_spec::v1text::var_bytes(old_next as nat)
                == bytes@.subrange(start as int, end as int));
            assert(false);
        }
    }
    raise_variable_reject(bytes, start, end, old_next, at);
    false
}

proof fn var_stream_all_concat(left: Seq<Term>, right: Seq<Term>)
    ensures ckc_spec::term::var_stream_all(left + right)
        == ckc_spec::term::var_stream_all(left)
            + ckc_spec::term::var_stream_all(right),
    decreases left.len(),
{
    if left.len() == 0 {
        reveal_with_fuel(ckc_spec::term::var_stream_all, 2);
    } else {
        var_stream_all_concat(left.drop_first(), right);
        assert_seqs_equal!((left + right).drop_first()
            == left.drop_first() + right);
        reveal_with_fuel(ckc_spec::term::var_stream_all, 2);
        assert_seqs_equal!(
            ckc_spec::term::var_stream(left[0])
                + (ckc_spec::term::var_stream_all(left.drop_first())
                    + ckc_spec::term::var_stream_all(right))
            == (ckc_spec::term::var_stream(left[0])
                    + ckc_spec::term::var_stream_all(left.drop_first()))
                + ckc_spec::term::var_stream_all(right)
        );
    }
}

proof fn var_stream_all_push(terms: Seq<Term>, term: Term)
    ensures ckc_spec::term::var_stream_all(terms.push(term))
        == ckc_spec::term::var_stream_all(terms)
            + ckc_spec::term::var_stream(term),
{
    assert_seqs_equal!(terms.push(term) == terms + seq![term]);
    var_stream_all_concat(terms, seq![term]);
    reveal_with_fuel(ckc_spec::term::var_stream_all, 2);
}

proof fn var_stream_list_term(elems: Seq<Term>, tail: Term)
    ensures ckc_spec::term::var_stream(list_term(elems, tail))
        == ckc_spec::term::var_stream_all(elems)
            + ckc_spec::term::var_stream(tail),
    decreases elems.len(),
{
    if elems.len() == 0 {
        reveal_with_fuel(list_term, 2);
        reveal_with_fuel(ckc_spec::term::var_stream_all, 2);
    } else {
        var_stream_list_term(elems.drop_first(), tail);
        assert(list_term(elems, tail) == Term::Comp(
            ckc_spec::v1text::cons_name(),
            seq![elems[0], list_term(elems.drop_first(), tail)],
        )) by {
            reveal(list_term);
        }
        assert(ckc_spec::term::var_stream(list_term(elems, tail))
            == ckc_spec::term::var_stream_all(
                seq![elems[0], list_term(elems.drop_first(), tail)])) by {
            reveal(ckc_spec::term::var_stream);
        }
        assert(ckc_spec::term::var_stream_all(
                seq![elems[0], list_term(elems.drop_first(), tail)])
            == ckc_spec::term::var_stream(elems[0])
                + ckc_spec::term::var_stream(list_term(elems.drop_first(), tail))) by {
            reveal_with_fuel(ckc_spec::term::var_stream_all, 3);
        }
        reveal_with_fuel(ckc_spec::term::var_stream_all, 2);
        assert_seqs_equal!(
            ckc_spec::term::var_stream(elems[0])
                + (ckc_spec::term::var_stream_all(elems.drop_first())
                    + ckc_spec::term::var_stream(tail))
            == (ckc_spec::term::var_stream(elems[0])
                    + ckc_spec::term::var_stream_all(elems.drop_first()))
                + ckc_spec::term::var_stream(tail)
        );
    }
}

pub open spec fn current_term(current: &Option<ESpannedTerm>) -> Option<Term> {
    match current {
        Some(term) => Some(term@),
        None => None,
    }
}

pub open spec fn option_var_stream(current: Option<Term>) -> Seq<nat> {
    match current {
        Some(term) => ckc_spec::term::var_stream(term),
        None => Seq::empty(),
    }
}

pub closed spec fn tracked_parse_state(
    entry: Seq<nat>,
    stream: Seq<nat>,
    guides: Seq<GTermFrame>,
    current: Option<Term>,
) -> bool {
    stream == entry + stack_var_prefix(guides) + option_var_stream(current)
}

proof fn tracked_state_initial(entry: Seq<nat>)
    ensures tracked_parse_state(entry, entry, Seq::empty(), None),
{
    reveal(tracked_parse_state);
    reveal(stack_var_prefix);
    reveal(option_var_stream);
}

proof fn tracked_state_next(
    entry: Seq<nat>,
    stream: Seq<nat>,
    guides: Seq<GTermFrame>,
    child: Term,
    new_guides: Seq<GTermFrame>,
)
    requires
        tracked_parse_state(entry, stream, guides, Some(child)),
        stack_var_prefix(guides) + ckc_spec::term::var_stream(child)
            == stack_var_prefix(new_guides),
    ensures tracked_parse_state(entry, stream, new_guides, None),
{
    reveal(tracked_parse_state);
    reveal(option_var_stream);
    assert_seqs_equal!(
        entry + (stack_var_prefix(guides) + ckc_spec::term::var_stream(child))
        == (entry + stack_var_prefix(guides))
            + ckc_spec::term::var_stream(child)
    );
}

proof fn tracked_state_done(
    entry: Seq<nat>,
    stream: Seq<nat>,
    guides: Seq<GTermFrame>,
    child: Term,
    new_guides: Seq<GTermFrame>,
    done: Term,
)
    requires
        tracked_parse_state(entry, stream, guides, Some(child)),
        stack_var_prefix(guides) + ckc_spec::term::var_stream(child)
            == stack_var_prefix(new_guides) + ckc_spec::term::var_stream(done),
    ensures tracked_parse_state(entry, stream, new_guides, Some(done)),
{
    reveal(tracked_parse_state);
    reveal(option_var_stream);
    assert_seqs_equal!(
        entry + (stack_var_prefix(guides) + ckc_spec::term::var_stream(child))
        == (entry + stack_var_prefix(guides))
            + ckc_spec::term::var_stream(child)
    );
    assert_seqs_equal!(
        (entry + stack_var_prefix(new_guides))
            + ckc_spec::term::var_stream(done)
        == entry
            + (stack_var_prefix(new_guides) + ckc_spec::term::var_stream(done))
    );
}

proof fn tracked_state_push(
    entry: Seq<nat>,
    stream: Seq<nat>,
    guides: Seq<GTermFrame>,
    guide: GTermFrame,
)
    requires
        tracked_parse_state(entry, stream, guides, None),
        guide_var_prefix(guide) == Seq::<nat>::empty(),
    ensures tracked_parse_state(entry, stream, guides.push(guide), None),
{
    stack_var_prefix_push_empty(guides, guide);
    reveal(tracked_parse_state);
    reveal(option_var_stream);
}

proof fn tracked_state_atomic(
    entry: Seq<nat>,
    before: Seq<nat>,
    after: Seq<nat>,
    guides: Seq<GTermFrame>,
    term: Term,
)
    requires
        tracked_parse_state(entry, before, guides, None),
        after == before + ckc_spec::term::var_stream(term),
    ensures tracked_parse_state(entry, after, guides, Some(term)),
{
    reveal(tracked_parse_state);
    reveal(option_var_stream);
}

proof fn tracked_state_root(
    entry: Seq<nat>,
    stream: Seq<nat>,
    term: Term,
)
    requires tracked_parse_state(entry, stream, Seq::empty(), Some(term)),
    ensures stream == entry + ckc_spec::term::var_stream(term),
{
    reveal(tracked_parse_state);
    reveal(stack_var_prefix);
    reveal(option_var_stream);
}

pub open spec fn guide_var_prefix(guide: GTermFrame) -> Seq<nat> {
    match guide {
        GTermFrame::Comp { built, .. } => ckc_spec::term::var_stream_all(built),
        GTermFrame::List { built, .. } => ckc_spec::term::var_stream_all(built),
        GTermFrame::Curly { .. } => Seq::empty(),
    }
}

pub open spec fn stack_var_prefix(guides: Seq<GTermFrame>) -> Seq<nat>
    decreases guides.len(),
{
    if guides.len() == 0 {
        Seq::empty()
    } else {
        stack_var_prefix(guides.drop_last()) + guide_var_prefix(guides.last())
    }
}

proof fn stack_var_prefix_push(guides: Seq<GTermFrame>, guide: GTermFrame)
    ensures stack_var_prefix(guides.push(guide))
        == stack_var_prefix(guides) + guide_var_prefix(guide),
{
    assert_seqs_equal!(guides.push(guide).drop_last() == guides);
    assert(guides.push(guide).last() == guide);
    reveal_with_fuel(stack_var_prefix, 2);
}

proof fn stack_var_prefix_push_empty(
    guides: Seq<GTermFrame>,
    guide: GTermFrame,
)
    requires guide_var_prefix(guide) == Seq::<nat>::empty(),
    ensures stack_var_prefix(guides.push(guide)) == stack_var_prefix(guides),
{
    stack_var_prefix_push(guides, guide);
}

proof fn stack_var_prefix_last(guides: Seq<GTermFrame>)
    requires guides.len() > 0,
    ensures stack_var_prefix(guides)
        == stack_var_prefix(guides.drop_last()) + guide_var_prefix(guides.last()),
{
    reveal(stack_var_prefix);
}

proof fn stack_var_prefix_advance(
    guides: Seq<GTermFrame>,
    advanced: GTermFrame,
    child: Term,
)
    requires
        guides.len() > 0,
        guide_var_prefix(advanced)
            == guide_var_prefix(guides.last())
                + ckc_spec::term::var_stream(child),
    ensures stack_var_prefix(guides) + ckc_spec::term::var_stream(child)
        == stack_var_prefix(guides.drop_last().push(advanced)),
{
    stack_var_prefix_last(guides);
    stack_var_prefix_push(guides.drop_last(), advanced);
    assert_seqs_equal!(
        (stack_var_prefix(guides.drop_last()) + guide_var_prefix(guides.last()))
            + ckc_spec::term::var_stream(child)
        == stack_var_prefix(guides.drop_last())
            + (guide_var_prefix(guides.last())
                + ckc_spec::term::var_stream(child))
    );
}

proof fn stack_var_prefix_done(
    guides: Seq<GTermFrame>,
    child: Term,
    done: Term,
)
    requires
        guides.len() > 0,
        guide_var_prefix(guides.last()) + ckc_spec::term::var_stream(child)
            == ckc_spec::term::var_stream(done),
    ensures stack_var_prefix(guides) + ckc_spec::term::var_stream(child)
        == stack_var_prefix(guides.drop_last())
            + ckc_spec::term::var_stream(done),
{
    stack_var_prefix_last(guides);
    assert_seqs_equal!(
        (stack_var_prefix(guides.drop_last()) + guide_var_prefix(guides.last()))
            + ckc_spec::term::var_stream(child)
        == stack_var_prefix(guides.drop_last())
            + (guide_var_prefix(guides.last())
                + ckc_spec::term::var_stream(child))
    );
}

proof fn guide_advance_vars(
    bytes: Seq<u8>,
    frame: &ETermFrame,
    guide: GTermFrame,
)
    requires
        guide_frame_ok(bytes, frame, guide),
        guide_has_next(guide),
    ensures guide_var_prefix(advance_guide(guide))
        == guide_var_prefix(guide) + ckc_spec::term::var_stream(guide_next(guide)),
{
    reveal(guide_frame_ok);
    reveal(guide_has_next);
    reveal(advance_guide);
    reveal(guide_var_prefix);
    reveal(guide_next);
    match guide {
        GTermFrame::Comp { built, remaining, .. } => {
            assert(remaining.len() > 1);
            var_stream_all_push(built, remaining[0]);
        },
        GTermFrame::List { built, remaining, tail, tail_mode, .. } => {
            assert(!tail_mode);
            assert(remaining.len() > 0);
            var_stream_all_push(built, remaining[0]);
        },
        GTermFrame::Curly { .. } => assert(false),
    }
}

proof fn comp_guide_done_vars(
    name: Seq<u8>,
    built: Seq<Term>,
    remaining: Seq<Term>,
    end: int,
)
    requires remaining.len() == 1,
    ensures {
        let guide = GTermFrame::Comp { name, built, remaining, end };
        guide_var_prefix(guide) + ckc_spec::term::var_stream(guide_next(guide))
            == ckc_spec::term::var_stream(guide_target(guide))
    },
{
    let guide = GTermFrame::Comp { name, built, remaining, end };
    assert_seqs_equal!(remaining == seq![remaining[0]]);
    var_stream_all_concat(built, remaining);
    assert(ckc_spec::term::var_stream_all(remaining)
        == ckc_spec::term::var_stream(remaining[0])) by {
        reveal_with_fuel(ckc_spec::term::var_stream_all, 2);
    }
    reveal(guide_var_prefix);
    reveal(guide_next);
    reveal(guide_target);
    reveal(ckc_spec::term::var_stream);
}

proof fn list_tail_guide_done_vars(
    built: Seq<Term>,
    tail: Term,
    end: int,
)
    ensures {
        let guide = GTermFrame::List {
            built,
            remaining: Seq::empty(),
            tail,
            tail_mode: true,
            end,
        };
        guide_var_prefix(guide) + ckc_spec::term::var_stream(guide_next(guide))
            == ckc_spec::term::var_stream(guide_target(guide))
    },
{
    let remaining = Seq::<Term>::empty();
    var_stream_list_term(built, tail);
    assert_seqs_equal!(built + remaining == built);
    reveal(guide_var_prefix);
    reveal(guide_next);
    reveal(guide_target);
}

proof fn list_elem_guide_done_vars(
    built: Seq<Term>,
    remaining: Seq<Term>,
    end: int,
)
    requires remaining.len() == 1,
    ensures {
        let guide = GTermFrame::List {
            built,
            remaining,
            tail: Term::Nil,
            tail_mode: false,
            end,
        };
        guide_var_prefix(guide) + ckc_spec::term::var_stream(guide_next(guide))
            == ckc_spec::term::var_stream(guide_target(guide))
    },
{
    assert_seqs_equal!(remaining == seq![remaining[0]]);
    assert_seqs_equal!(built + remaining == built.push(remaining[0]));
    var_stream_list_term(built + remaining, Term::Nil);
    var_stream_all_push(built, remaining[0]);
    assert(ckc_spec::term::var_stream(Term::Nil) == Seq::<nat>::empty()) by {
        reveal(ckc_spec::term::var_stream);
    }
    reveal(guide_var_prefix);
    reveal(guide_next);
    reveal(guide_target);
}

proof fn curly_guide_done_vars(child: Term, end: int)
    ensures {
        let guide = GTermFrame::Curly { child, end };
        guide_var_prefix(guide) + ckc_spec::term::var_stream(guide_next(guide))
            == ckc_spec::term::var_stream(guide_target(guide))
    },
{
    reveal(guide_var_prefix);
    reveal(guide_next);
    reveal(guide_target);
    reveal(ckc_spec::v1text::curly_name);
    reveal_with_fuel(ckc_spec::term::var_stream, 2);
    reveal_with_fuel(ckc_spec::term::var_stream_all, 3);
}

proof fn tracked_next_transition(
    bytes: Seq<u8>,
    frame: &ETermFrame,
    guides: Seq<GTermFrame>,
    guide: GTermFrame,
    child: Term,
)
    requires
        guides.len() > 0,
        guides.last() == guide,
        guide_frame_ok(bytes, frame, guide),
        guide_has_next(guide),
        child == guide_next(guide),
    ensures stack_var_prefix(guides) + ckc_spec::term::var_stream(child)
        == stack_var_prefix(guides.drop_last().push(advance_guide(guide))),
{
    guide_advance_vars(bytes, frame, guide);
    assert(guide_var_prefix(advance_guide(guide))
        == guide_var_prefix(guides.last())
            + ckc_spec::term::var_stream(child));
    stack_var_prefix_advance(guides, advance_guide(guide), child);
}

proof fn tracked_done_transition(
    bytes: Seq<u8>,
    frame: &ETermFrame,
    guides: Seq<GTermFrame>,
    guide: GTermFrame,
    child: Term,
    done: Term,
)
    requires
        guides.len() > 0,
        guides.last() == guide,
        guide_frame_ok(bytes, frame, guide),
        !guide_has_next(guide),
        child == guide_next(guide),
        done == guide_target(guide),
    ensures stack_var_prefix(guides) + ckc_spec::term::var_stream(child)
        == stack_var_prefix(guides.drop_last())
            + ckc_spec::term::var_stream(done),
{
    reveal(guide_frame_ok);
    reveal(guide_has_next);
    match guide {
        GTermFrame::Comp { name, built, remaining, end } => {
            assert(remaining.len() == 1);
            let rebuilt = GTermFrame::Comp { name, built, remaining, end };
            assert(guide == rebuilt);
            comp_guide_done_vars(name, built, remaining, end);
            assert(guide_var_prefix(guide)
                    + ckc_spec::term::var_stream(child)
                == ckc_spec::term::var_stream(done));
            stack_var_prefix_done(guides, child, done);
        },
        GTermFrame::List { built, remaining, tail, tail_mode, end } => {
            if tail_mode {
                assert(remaining.len() == 0);
                assert_seqs_equal!(remaining == Seq::<Term>::empty());
                let rebuilt = GTermFrame::List {
                    built,
                    remaining: Seq::empty(),
                    tail,
                    tail_mode: true,
                    end,
                };
                assert(guide == rebuilt);
                list_tail_guide_done_vars(built, tail, end);
            } else {
                assert(remaining.len() == 1);
                assert(tail == Term::Nil);
                let rebuilt = GTermFrame::List {
                    built,
                    remaining,
                    tail: Term::Nil,
                    tail_mode: false,
                    end,
                };
                assert(guide == rebuilt);
                list_elem_guide_done_vars(built, remaining, end);
            }
            assert(guide_var_prefix(guide)
                    + ckc_spec::term::var_stream(child)
                == ckc_spec::term::var_stream(done));
            stack_var_prefix_done(guides, child, done);
        },
        GTermFrame::Curly { child: target, end } => {
            let rebuilt = GTermFrame::Curly { child: target, end };
            assert(guide == rebuilt);
            curly_guide_done_vars(target, end);
            assert(guide_var_prefix(guide)
                    + ckc_spec::term::var_stream(child)
                == ckc_spec::term::var_stream(done));
            stack_var_prefix_done(guides, child, done);
        },
    }
}

pub open spec fn tracker_complete(valid: bool, stream: Seq<nat>) -> bool {
    valid
        || !ckc_spec::term::var_canonical(stream)
        || stream.len() > usize::MAX as nat
}

proof fn firsts_len_le(
    values: Seq<nat>,
    seen: Set<nat>,
)
    ensures ckc_spec::term::firsts(values, seen).len() <= values.len(),
    decreases values.len(),
{
    if values.len() == 0 {
        reveal_with_fuel(ckc_spec::term::firsts, 2);
    } else {
        let rest = values.drop_first();
        firsts_len_le(rest, seen.insert(values[0]));
        reveal_with_fuel(ckc_spec::term::firsts, 2);
        assert(values.len() == rest.len() + 1);
        if seen.contains(values[0]) {
            assert_sets_equal!(seen.insert(values[0]) == seen);
            assert(ckc_spec::term::firsts(values, seen)
                == ckc_spec::term::firsts(rest, seen));
            firsts_len_le(rest, seen);
        } else {
            assert(ckc_spec::term::firsts(values, seen).len()
                == 1 + ckc_spec::term::firsts(
                    rest,
                    seen.insert(values[0]),
                ).len());
        }
    }
}

proof fn nat_prefix_push_tail(n: nat, value: nat)
    requires nat_prefix(n).push(value) == nat_prefix(n + 1),
    ensures value == n,
{
    nat_prefix_push(n);
    assert(nat_prefix(n).push(value).last() == value);
    assert(nat_prefix(n).push(n).last() == n);
}

#[verifier::rlimit(100)]
proof fn noncanonical_push(stream: Seq<nat>, value: nat)
    requires !ckc_spec::term::var_canonical(stream),
    ensures !ckc_spec::term::var_canonical(stream.push(value)),
{
    let old_firsts = ckc_spec::term::firsts(stream, Set::empty());
    let new_firsts = ckc_spec::term::firsts(
        stream.push(value),
        Set::empty(),
    );
    firsts_push(stream, Set::empty(), value);
    reveal(ckc_spec::term::var_canonical);
    reveal(nat_prefix);
    if seen_after(stream, Set::empty()).contains(value) {
        assert(new_firsts == old_firsts);
    } else {
        assert(new_firsts == old_firsts.push(value));
        if new_firsts
            == Seq::new(new_firsts.len(), |i: int| i as nat)
        {
            assert(new_firsts.len() == old_firsts.len() + 1);
            assert(new_firsts == nat_prefix(old_firsts.len() + 1));
            nat_prefix_push(old_firsts.len());
            assert(old_firsts.push(value)
                == nat_prefix(old_firsts.len()).push(old_firsts.len()));
            assert(old_firsts.push(value).drop_last() == old_firsts);
            assert(nat_prefix(old_firsts.len()).push(old_firsts.len()).drop_last()
                == nat_prefix(old_firsts.len()));
            assert(old_firsts == nat_prefix(old_firsts.len()));
            assert(false);
        }
    }
}

#[verifier::rlimit(200)]
proof fn rejected_push_complete(
    next: usize,
    stream: Seq<nat>,
    value: nat,
)
    requires
        tracker_state_ok(next, stream),
        !(value < next
            || value == next && next < usize::MAX),
    ensures
        !ckc_spec::term::var_canonical(stream.push(value))
            || stream.push(value).len() > usize::MAX as nat,
{
    let old_firsts = ckc_spec::term::firsts(stream, Set::empty());
    let new_firsts = ckc_spec::term::firsts(
        stream.push(value),
        Set::empty(),
    );
    reveal(tracker_state_ok);
    canonical_seen_contains(next as nat, value);
    firsts_push(stream, Set::empty(), value);
    if seen_after(stream, Set::empty()).contains(value) {
        assert(value < next);
        assert(false);
    }
    assert(new_firsts == old_firsts.push(value));
    if value == next as nat {
        assert(next == usize::MAX);
        firsts_len_le(stream, Set::empty());
        reveal(nat_prefix);
        assert(old_firsts.len() == next as nat);
        assert(stream.len() >= next as nat);
        assert(stream.push(value).len() > usize::MAX as nat);
    } else if ckc_spec::term::var_canonical(stream.push(value)) {
        reveal(ckc_spec::term::var_canonical);
        reveal(nat_prefix);
        assert(new_firsts.len() == next as nat + 1);
        assert(new_firsts == nat_prefix(next as nat + 1));
        nat_prefix_push_tail(next as nat, value);
        assert(false);
    }
}

proof fn tracker_complete_push_invalid(stream: Seq<nat>, value: nat)
    requires tracker_complete(false, stream),
    ensures tracker_complete(false, stream.push(value)),
{
    reveal(tracker_complete);
    if !ckc_spec::term::var_canonical(stream) {
        noncanonical_push(stream, value);
    } else {
        assert(stream.len() > usize::MAX as nat);
    }
}

proof fn tracker_state_canonical(next: usize, stream: Seq<nat>)
    requires tracker_state_ok(next, stream),
    ensures ckc_spec::term::var_canonical(stream),
{
    reveal(tracker_state_ok);
    reveal(ckc_spec::term::var_canonical);
    reveal(nat_prefix);
}

proof fn atomic_var_stream_len(
    term: Term,
    start: usize,
    end: usize,
)
    requires
        atomic_term(term),
        start < end,
    ensures ckc_spec::term::var_stream(term).len() <= end - start,
{
    match term {
        Term::Var(_) => {
            reveal(ckc_spec::term::var_stream);
        },
        Term::Int(_) | Term::Nil | Term::Atom(_) => {
            reveal(ckc_spec::term::var_stream);
        },
        Term::Comp(_, _) => {
            reveal(atomic_term);
            assert(false);
        },
    }
}

#[verifier::rlimit(500)]
fn record_variable(
    bytes: &[u8],
    start: usize,
    end: usize,
    value: Ghost<nat>,
    tracker: &mut EVarTracker,
at: &mut usize,
) -> (r: bool)
    requires
        *old(at) <= bytes@.len(),
        start < end <= bytes@.len(),
        ckc_spec::v1text::var_bytes(value@)
            == bytes@.subrange(start as int, end as int),
        old(tracker).valid ==>
            tracker_state_ok(old(tracker).next, old(tracker).stream@),
        tracker_complete(old(tracker).valid, old(tracker).stream@),
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        final(tracker).stream@ == old(tracker).stream@.push(value@),
        final(tracker).valid ==>
            tracker_state_ok(final(tracker).next, final(tracker).stream@),
        tracker_complete(final(tracker).valid, final(tracker).stream@),
        r == final(tracker).valid,
{
    let ghost old_stream = tracker.stream@;
    if tracker.valid {
        let old_next = tracker.next;
        let accepted = observe_variable(bytes, start, end, value, tracker, at);
        if accepted {
            proof {
                assert(tracker.valid);
                assert(tracker.stream@ == old_stream.push(value@));
                assert(tracker_state_ok(tracker.next, tracker.stream@));
                reveal(tracker_complete);
            }
            return true;
        }
        proof {
            assert(tracker.next == old_next);
            assert(tracker.stream@ == old_stream);
            rejected_push_complete(old_next, old_stream, value@);
        }
        tracker.stream = Ghost(old_stream.push(value@));
        tracker.valid = false;
        proof {
            reveal(tracker_complete);
        }
        return false;
    }
    proof {
        tracker_complete_push_invalid(old_stream, value@);
    }
    tracker.stream = Ghost(old_stream.push(value@));
    proof {
        reveal(tracker_complete);
    }
    false
}

pub open spec fn answers_flat(
    a: ckc_spec::v1text::AnswersFile,
) -> Seq<u8> {
    ckc_spec::v1text::ascii("% "@)
        + a.qid
        + ckc_spec::v1text::ascii(
            " answered against the loaded composition by ace_to_pl answer mode; do not edit.\n"@,
        )
        + ckc_spec::v1text::atom_bytes(
            ckc_spec::v1text::ascii("$guideline_answers"@),
        )
        + seq![0x28u8]
        + ckc_spec::v1text::atom_bytes(ckc_spec::v1text::ascii("v1"@))
        + seq![0x2cu8]
        + ckc_spec::v1text::atom_bytes(a.qid)
        + seq![0x2cu8]
        + ckc_spec::v1text::atom_bytes(
            ckc_spec::v1text::ascii("query_sha256"@),
        )
        + seq![0x28u8]
        + ckc_spec::v1text::atom_bytes(a.qsha)
        + seq![0x29u8, 0x2cu8]
        + ckc_spec::v1text::atom_bytes(ckc_spec::v1text::ascii("result"@))
        + seq![0x28u8]
        + ckc_spec::v1text::term_bytes(a.result)
        + seq![0x29u8, 0x29u8]
        + ckc_spec::v1text::ascii(".\n"@)
}

proof fn atom_term_bytes(name: Seq<u8>)
    ensures
        ckc_spec::v1text::term_bytes(Term::Atom(name))
            == ckc_spec::v1text::atom_bytes(name),
{
    reveal_with_fuel(ckc_spec::v1text::term_bytes, 1);
}

proof fn regular_comp_bytes(name: Seq<u8>, args: Seq<Term>)
    requires
        !(name == ckc_spec::v1text::cons_name() && args.len() == 2),
        !(name == ckc_spec::v1text::curly_name() && args.len() == 1),
    ensures
        ckc_spec::v1text::term_bytes(Term::Comp(name, args))
            == ckc_spec::v1text::atom_bytes(name)
                + seq![0x28u8]
                + ckc_spec::v1text::args_bytes(args)
                + seq![0x29u8],
{
    reveal_with_fuel(ckc_spec::v1text::term_bytes, 1);
}

proof fn args_one_bytes(a: Term)
    ensures
        ckc_spec::v1text::args_bytes(seq![a])
            == ckc_spec::v1text::term_bytes(a),
{
    reveal_with_fuel(ckc_spec::v1text::args_bytes, 2);
}

proof fn args_four_bytes(a: Term, b: Term, c: Term, d: Term)
    ensures
        ckc_spec::v1text::args_bytes(seq![a, b, c, d])
            == ckc_spec::v1text::term_bytes(a)
                + seq![0x2cu8]
                + ckc_spec::v1text::term_bytes(b)
                + seq![0x2cu8]
                + ckc_spec::v1text::term_bytes(c)
                + seq![0x2cu8]
                + ckc_spec::v1text::term_bytes(d),
{
    reveal_with_fuel(ckc_spec::v1text::args_bytes, 5);
}

pub open spec fn answers_parts(
    a: ckc_spec::v1text::AnswersFile,
) -> Seq<Seq<u8>> {
    seq![
        ckc_spec::v1text::ascii("% "@),
        a.qid,
        ckc_spec::v1text::ascii(
            " answered against the loaded composition by ace_to_pl answer mode; do not edit.\n"@,
        ),
        ckc_spec::v1text::atom_bytes(
            ckc_spec::v1text::ascii("$guideline_answers"@),
        ),
        seq![0x28u8],
        ckc_spec::v1text::atom_bytes(ckc_spec::v1text::ascii("v1"@)),
        seq![0x2cu8],
        ckc_spec::v1text::atom_bytes(a.qid),
        seq![0x2cu8],
        ckc_spec::v1text::atom_bytes(
            ckc_spec::v1text::ascii("query_sha256"@),
        ),
        seq![0x28u8],
        ckc_spec::v1text::atom_bytes(a.qsha),
        seq![0x29u8],
        seq![0x2cu8],
        ckc_spec::v1text::atom_bytes(ckc_spec::v1text::ascii("result"@)),
        seq![0x28u8],
        ckc_spec::v1text::term_bytes(a.result),
        seq![0x29u8],
        seq![0x29u8],
        ckc_spec::v1text::ascii(".\n"@),
    ]
}

#[verifier::rlimit(5000)]
#[verifier::spinoff_prover]
proof fn answers_parts_flat(a: ckc_spec::v1text::AnswersFile)
    ensures answers_parts(a).flatten() == answers_flat(a),
{
    reveal(answers_parts);
    reveal(answers_flat);
    reveal_with_fuel(Seq::<_>::flatten, 22);
}

proof fn flatten_take_step<A>(parts: Seq<Seq<A>>, i: int)
    requires 0 <= i < parts.len(),
    ensures
        parts.take(i + 1).flatten()
            == parts.take(i).flatten() + parts[i],
{
    parts.lemma_take_succ_push(i);
    parts.take(i).lemma_flatten_push(parts[i]);
}

proof fn flattened_part(bytes: Seq<u8>, parts: Seq<Seq<u8>>, i: int)
    requires
        bytes == parts.flatten(),
        0 <= i < parts.len(),
    ensures
        parts.take(i).flatten().len() + parts[i].len() <= bytes.len(),
        bytes.subrange(
            parts.take(i).flatten().len() as int,
            (parts.take(i).flatten().len() + parts[i].len()) as int,
        ) == parts[i],
{
    let before_parts = parts.take(i);
    let through_parts = parts.take(i + 1);
    let after_parts = parts.skip(i + 1);
    let before = before_parts.flatten();
    let part = parts[i];
    let after = after_parts.flatten();
    flatten_take_step(parts, i);
    assert_seqs_equal!(parts == through_parts + after_parts);
    vstd::seq_lib::lemma_flatten_concat(through_parts, after_parts);
    assert(bytes == before + part + after);
    assert_seqs_equal!(bytes.subrange(0, bytes.len() as int) == bytes);
    segment_of_three(
        bytes,
        0,
        bytes.len() as int,
        before,
        part,
        after,
    );
}

proof fn cursor_part_ready(
    bytes: Seq<u8>,
    cursor: &EByteCursor,
    parts: Seq<Seq<u8>>,
    i: int,
)
    requires
        cursor_ok(bytes, cursor),
        cursor.prefix@ == parts.take(i).flatten(),
        bytes == parts.flatten(),
        0 <= i < parts.len(),
    ensures
        cursor.pos as int == parts.take(i).flatten().len(),
        cursor.pos as int + parts[i].len() <= bytes.len(),
        bytes.subrange(
            cursor.pos as int,
            cursor.pos as int + parts[i].len(),
        ) == parts[i],
{
    reveal(cursor_ok);
    assert(cursor.prefix@.len() == cursor.pos);
    flattened_part(bytes, parts, i);
}

proof fn part_prefix_advanced(
    parts: Seq<Seq<u8>>,
    i: int,
    before: Seq<u8>,
    after: Seq<u8>,
)
    requires
        0 <= i < parts.len(),
        before == parts.take(i).flatten(),
        after == before + parts[i],
    ensures after == parts.take(i + 1).flatten(),
{
    flatten_take_step(parts, i);
}

proof fn atom_part_expected(
    bytes: Seq<u8>,
    cursor: &EByteCursor,
    parts: Seq<Seq<u8>>,
    i: int,
    name: Seq<u8>,
    next: u8,
    bound: usize,
) -> (g: GAtomExpected)
    requires
        bytes.len() == bound,
        cursor_ok(bytes, cursor),
        cursor.prefix@ == parts.take(i).flatten(),
        bytes == parts.flatten(),
        0 <= i,
        i + 1 < parts.len(),
        parts[i] == ckc_spec::v1text::atom_bytes(name),
        parts[i + 1].len() > 0,
        parts[i + 1][0] == next,
        next == 0x28 || next == 0x2c || next == 0x29,
    ensures
        cursor.pos < g.end <= bytes.len(),
        ckc_spec::v1text::atom_bytes(g.name)
            == bytes.subrange(cursor.pos as int, g.end as int),
        atom_boundary(bytes, g.end as int),
        g.name == name,
{
    cursor_part_ready(bytes, cursor, parts, i);
    atom_bytes_first_safe(name);
    flatten_take_step(parts, i);
    flattened_part(bytes, parts, i + 1);
    let end = cursor.pos as int + parts[i].len();
    assert(parts.take(i + 1).flatten().len() == end);
    assert(bytes[end] == parts[i + 1][0]) by {
        assert(bytes.subrange(
            end,
            end + parts[i + 1].len(),
        )[0] == bytes[end]);
    }
    reveal(atom_boundary);
    reveal(term_boundary);
    assert(0 <= end <= bound as int);
    assert(bound <= usize::MAX);
    assert(end <= usize::MAX);
    GAtomExpected { name, end: end as usize }
}

proof fn name_part_expected(
    bytes: Seq<u8>,
    cursor: &EByteCursor,
    parts: Seq<Seq<u8>>,
    i: int,
    name: Seq<u8>,
    next: u8,
    bound: usize,
) -> (g: GNameExpected)
    requires
        bytes.len() == bound,
        cursor_ok(bytes, cursor),
        cursor.prefix@ == parts.take(i).flatten(),
        bytes == parts.flatten(),
        0 <= i,
        i + 1 < parts.len(),
        parts[i] == name,
        ckc_spec::v1text::name_ok(name),
        parts[i + 1].len() > 0,
        parts[i + 1][0] == next,
        !(ckc_spec::v1text::is_lower_b(next)
            || ckc_spec::v1text::is_digit_b(next)
            || next == 0x2d),
    ensures
        cursor.pos < g.end < bytes.len(),
        ckc_spec::v1text::name_ok(g.value),
        g.value == bytes.subrange(cursor.pos as int, g.end as int),
        !(ckc_spec::v1text::is_lower_b(bytes[g.end as int])
            || ckc_spec::v1text::is_digit_b(bytes[g.end as int])
            || bytes[g.end as int] == 0x2d),
        g.value == name,
{
    cursor_part_ready(bytes, cursor, parts, i);
    flatten_take_step(parts, i);
    flattened_part(bytes, parts, i + 1);
    let end = cursor.pos as int + parts[i].len();
    assert(parts.take(i + 1).flatten().len() == end);
    assert(bytes[end] == next) by {
        assert(bytes.subrange(
            end,
            end + parts[i + 1].len(),
        )[0] == bytes[end]);
    }
    reveal(ckc_spec::v1text::is_lower_b);
    reveal(ckc_spec::v1text::is_digit_b);
    assert(0 <= end <= bound as int);
    assert(bound <= usize::MAX);
    assert(end <= usize::MAX);
    GNameExpected { value: name, end: end as usize }
}

proof fn term_part_expected(
    bytes: Seq<u8>,
    cursor: &EByteCursor,
    parts: Seq<Seq<u8>>,
    i: int,
    term: Term,
    next: u8,
    bound: usize,
) -> (g: GTermExpected)
    requires
        bytes.len() == bound,
        cursor_ok(bytes, cursor),
        cursor.prefix@ == parts.take(i).flatten(),
        bytes == parts.flatten(),
        0 <= i,
        i + 1 < parts.len(),
        parts[i] == ckc_spec::v1text::term_bytes(term),
        ckc_spec::term::wf_term(term),
        parts[i + 1].len() > 0,
        parts[i + 1][0] == next,
        next == 0x2c || next == 0x29 || next == 0x5d
            || next == 0x7c || next == 0x7d || next == 0x20
            || next == 0x2e && parts[i + 1].len() > 1
                && parts[i + 1][1] == 0x0a,
    ensures term_at(bytes, cursor.pos as int, g.end as int, g.term),
        g.term == term,
{
    cursor_part_ready(bytes, cursor, parts, i);
    term_bytes_nonempty(term);
    flatten_take_step(parts, i);
    flattened_part(bytes, parts, i + 1);
    let end = cursor.pos as int + parts[i].len();
    assert(parts.take(i + 1).flatten().len() == end);
    assert(bytes[end] == next) by {
        assert(bytes.subrange(
            end,
            end + parts[i + 1].len(),
        )[0] == bytes[end]);
    }
    if next == 0x2e {
        assert(end + 1 < bytes.len());
        assert(bytes[end + 1] == 0x0a) by {
            assert(bytes.subrange(
                end,
                end + parts[i + 1].len(),
            )[1] == bytes[end + 1]);
        }
    }
    reveal(term_at);
    reveal(term_boundary);
    assert(0 <= end <= bound as int);
    assert(bound <= usize::MAX);
    assert(end <= usize::MAX);
    GTermExpected { term, end: end as usize }
}

pub closed spec fn parts_progress(
    bytes: Seq<u8>,
    parts: Seq<Seq<u8>>,
    i: int,
    pos: usize,
    prefix: Seq<u8>,
) -> bool {
    &&& bytes == parts.flatten()
    &&& 0 <= i <= parts.len()
    &&& pos <= bytes.len()
    &&& prefix == bytes.subrange(0, pos as int)
    &&& prefix == parts.take(i).flatten()
}

proof fn parts_part_ready(
    bytes: Seq<u8>,
    parts: Seq<Seq<u8>>,
    i: int,
    cursor: &EByteCursor,
)
    requires
        parts_progress(bytes, parts, i, cursor.pos, cursor.prefix@),
        0 <= i < parts.len(),
    ensures
        cursor_ok(bytes, cursor),
        cursor.pos as int + parts[i].len() <= bytes.len(),
        bytes.subrange(
            cursor.pos as int,
            cursor.pos as int + parts[i].len(),
        ) == parts[i],
{
    reveal(parts_progress);
    reveal(cursor_ok);
    assert(cursor_ok(bytes, cursor));
    cursor_part_ready(bytes, cursor, parts, i);
}

proof fn advance_parts_progress(
    bytes: Seq<u8>,
    parts: Seq<Seq<u8>>,
    i: int,
    before_pos: usize,
    before_prefix: Seq<u8>,
    after: &EByteCursor,
)
    requires
        parts_progress(bytes, parts, i, before_pos, before_prefix),
        0 <= i < parts.len(),
        cursor_ok(bytes, after),
        after.prefix@ == before_prefix + parts[i],
    ensures
        parts_progress(bytes, parts, i + 1, after.pos, after.prefix@),
{
    reveal(parts_progress);
    reveal(cursor_ok);
    flatten_take_step(parts, i);
}

pub ghost struct GPartsGuide {
    pub parts: Seq<Seq<u8>>,
    pub index: int,
}

pub struct EGuidedCursor {
    pub cursor: EByteCursor,
    pub guide: Ghost<Option<GPartsGuide>>,
}

pub closed spec fn guided_cursor_ok(
    bytes: Seq<u8>,
    guided: &EGuidedCursor,
) -> bool {
    &&& cursor_ok(bytes, &guided.cursor)
    &&& guided.guide@ matches Some(g) ==>
        parts_progress(
            bytes,
            g.parts,
            g.index,
            guided.cursor.pos,
            guided.cursor.prefix@,
        )
}

fn new_guided_cursor(
    bytes: &[u8],
    parts: Ghost<Option<Seq<Seq<u8>>>>,
) -> (guided: EGuidedCursor)
    requires parts@ matches Some(ps) ==> bytes@ == ps.flatten(),
    ensures
        guided_cursor_ok(bytes@, &guided),
        guided.cursor.pos == 0,
        guided.cursor.prefix@ == Seq::<u8>::empty(),
        parts@ matches Some(ps) ==> guided.guide@ matches Some(g)
            && g.parts == ps && g.index == 0,
        parts@ is None ==> guided.guide@ is None,
{
    let cursor = new_cursor(bytes);
    let ghost guide = match parts@ {
        Some(ps) => Some(GPartsGuide { parts: ps, index: 0 }),
        None => None,
    };
    proof {
        reveal(guided_cursor_ok);
        reveal(parts_progress);
        reveal(cursor_ok);
        reveal_with_fuel(Seq::<_>::flatten, 1);
    }
    EGuidedCursor { cursor, guide: Ghost(guide) }
}

fn guided_literal(
    bytes: &[u8],
    guided: &mut EGuidedCursor,
    lit: &[u8],
    chunk: Ghost<Seq<u8>>,
at: &mut usize,
) -> (r: bool)
    requires
        *old(at) <= bytes@.len(),
        guided_cursor_ok(bytes@, old(guided)),
        lit@ == chunk@,
        old(guided).guide@ matches Some(g) ==> {
            &&& 0 <= g.index < g.parts.len()
            &&& g.parts[g.index] == chunk@
        },
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r ==> guided_cursor_ok(bytes@, final(guided)),
        r ==> {
            &&& final(guided).cursor.pos
                == old(guided).cursor.pos + chunk@.len()
            &&& final(guided).cursor.prefix@
                == old(guided).cursor.prefix@ + chunk@
        },
        old(guided).guide@ matches Some(g) ==> {
            &&& r
            &&& final(guided).guide@ matches Some(next)
                && next.parts == g.parts && next.index == g.index + 1
        },
        old(guided).guide@ is None ==> final(guided).guide@ is None,
{
    let old_pos = guided.cursor.pos;
    let ghost old_prefix = guided.cursor.prefix@;
    let ghost old_guide = guided.guide@;
    proof {
        reveal(guided_cursor_ok);
        if let Some(g) = old_guide {
            parts_part_ready(bytes@, g.parts, g.index, &guided.cursor);
        }
    }
    let accepted = cursor_literal(
        bytes,
        &mut guided.cursor,
        lit,
        chunk,
    at,
    );
    if !accepted {
        proof {
            if let Some(g) = old_guide {
                parts_part_ready(bytes@, g.parts, g.index, &guided.cursor);
                assert(false);
            }
        }
        return false;
    }
    let ghost next_guide = match old_guide {
        Some(g) => Some(GPartsGuide { parts: g.parts, index: g.index + 1 }),
        None => None,
    };
    proof {
        if let Some(g) = old_guide {
            advance_parts_progress(
                bytes@,
                g.parts,
                g.index,
                old_pos,
                old_prefix,
                &guided.cursor,
            );
        }
    }
    guided.guide = Ghost(next_guide);
    proof {
        reveal(guided_cursor_ok);
    }
    true
}

fn guided_byte(
    bytes: &[u8],
    guided: &mut EGuidedCursor,
    byte: u8,
at: &mut usize,
) -> (r: bool)
    requires
        *old(at) <= bytes@.len(),
        guided_cursor_ok(bytes@, old(guided)),
        old(guided).guide@ matches Some(g) ==> {
            &&& 0 <= g.index < g.parts.len()
            &&& g.parts[g.index] == seq![byte]
        },
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r ==> guided_cursor_ok(bytes@, final(guided)),
        r ==> {
            &&& old(guided).cursor.pos < bytes@.len()
            &&& bytes@[old(guided).cursor.pos as int] == byte
            &&& final(guided).cursor.pos == old(guided).cursor.pos + 1
            &&& final(guided).cursor.prefix@
                == old(guided).cursor.prefix@ + seq![byte]
        },
        old(guided).guide@ matches Some(g) ==> {
            &&& r
            &&& final(guided).guide@ matches Some(next)
                && next.parts == g.parts && next.index == g.index + 1
        },
        old(guided).guide@ is None ==> final(guided).guide@ is None,
{
    let old_pos = guided.cursor.pos;
    let ghost old_prefix = guided.cursor.prefix@;
    let ghost old_guide = guided.guide@;
    proof {
        reveal(guided_cursor_ok);
        if let Some(g) = old_guide {
            parts_part_ready(bytes@, g.parts, g.index, &guided.cursor);
            assert(bytes@.subrange(old_pos as int, old_pos as int + 1)[0]
                == bytes@[old_pos as int]);
            assert(bytes@[old_pos as int] == byte);
        }
    }
    let accepted = cursor_byte(bytes, &mut guided.cursor, byte, at);
    if !accepted {
        proof {
            if old_guide is Some {
                assert(false);
            }
        }
        return false;
    }
    let ghost next_guide = match old_guide {
        Some(g) => Some(GPartsGuide { parts: g.parts, index: g.index + 1 }),
        None => None,
    };
    proof {
        if let Some(g) = old_guide {
            advance_parts_progress(
                bytes@,
                g.parts,
                g.index,
                old_pos,
                old_prefix,
                &guided.cursor,
            );
        }
    }
    guided.guide = Ghost(next_guide);
    proof {
        reveal(guided_cursor_ok);
    }
    true
}

fn guided_atom(
    bytes: &[u8],
    guided: &mut EGuidedCursor,
    expected: Ghost<Option<(Seq<u8>, u8)>>,
at: &mut usize,
) -> (r: Option<EParsedAtom>)
    requires
        *old(at) <= bytes@.len(),
        guided_cursor_ok(bytes@, old(guided)),
        old(guided).guide@ matches Some(g) ==> expected@ matches Some(e)
            && {
                &&& 0 <= g.index
                &&& g.index + 1 < g.parts.len()
                &&& g.parts[g.index]
                    == ckc_spec::v1text::atom_bytes(e.0)
                &&& g.parts[g.index + 1].len() > 0
                &&& g.parts[g.index + 1][0] == e.1
                &&& (e.1 == 0x28 || e.1 == 0x2c || e.1 == 0x29)
            },
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r matches Some(atom) ==> {
            &&& guided_cursor_ok(bytes@, final(guided))
            &&& parsed_atom_ok(bytes@, old(guided).cursor.pos, &atom)
            &&& final(guided).cursor.pos == atom.end
            &&& final(guided).cursor.prefix@
                == old(guided).cursor.prefix@
                    + ckc_spec::v1text::atom_bytes(atom.name@)
        },
        old(guided).guide@ matches Some(g) ==> {
            &&& r matches Some(atom)
            && expected@ matches Some(e) && atom.name@ == e.0
            &&& final(guided).guide@ matches Some(next)
                && next.parts == g.parts && next.index == g.index + 1
        },
        old(guided).guide@ is None ==> final(guided).guide@ is None,
{
    let old_pos = guided.cursor.pos;
    let ghost old_prefix = guided.cursor.prefix@;
    let ghost old_guide = guided.guide@;
    proof {
        reveal(guided_cursor_ok);
        reveal(parts_progress);
        reveal(cursor_ok);
        assert(cursor_ok(bytes@, &guided.cursor));
    }
    let ghost atom_expected = match (old_guide, expected@) {
        (Some(g), Some(e)) => {
            Some(atom_part_expected(
                bytes@,
                &guided.cursor,
                g.parts,
                g.index,
                e.0,
                e.1,
                bytes.len(),
            ))
        },
        _ => None,
    };
    proof {
        reveal(guided_cursor_ok);
    }
    let atom = match cursor_atom(
        bytes,
        &mut guided.cursor,
        Ghost(atom_expected),
    at,
    ) {
        Some(atom) => atom,
        None => {
            proof {
                if old_guide is Some {
                    assert(false);
                }
            }
            return None;
        },
    };
    let ghost next_guide = match old_guide {
        Some(g) => Some(GPartsGuide { parts: g.parts, index: g.index + 1 }),
        None => None,
    };
    proof {
        if let Some(g) = old_guide {
            advance_parts_progress(
                bytes@,
                g.parts,
                g.index,
                old_pos,
                old_prefix,
                &guided.cursor,
            );
        }
    }
    guided.guide = Ghost(next_guide);
    proof {
        reveal(guided_cursor_ok);
    }
    Some(atom)
}

fn guided_name(
    bytes: &[u8],
    guided: &mut EGuidedCursor,
    next: Ghost<u8>,
    expected: Ghost<Option<Seq<u8>>>,
at: &mut usize,
) -> (r: Option<ENameField>)
    requires
        *old(at) <= bytes@.len(),
        guided_cursor_ok(bytes@, old(guided)),
        old(guided).guide@ matches Some(g) ==> expected@ matches Some(name)
            && {
                &&& 0 <= g.index
                &&& g.index + 1 < g.parts.len()
                &&& g.parts[g.index] == name
                &&& ckc_spec::v1text::name_ok(name)
                &&& g.parts[g.index + 1].len() > 0
                &&& g.parts[g.index + 1][0] == next@
                &&& !(ckc_spec::v1text::is_lower_b(next@)
                    || ckc_spec::v1text::is_digit_b(next@)
                    || next@ == 0x2d)
            },
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r matches Some(field) ==> {
            &&& guided_cursor_ok(bytes@, final(guided))
            &&& ckc_spec::v1text::name_ok(field.value@)
            &&& final(guided).cursor.prefix@
                == old(guided).cursor.prefix@ + field.value@
        },
        old(guided).guide@ matches Some(g) ==> {
            &&& r matches Some(field)
            && expected@ matches Some(name) && field.value@ == name
            &&& final(guided).guide@ matches Some(next)
                && next.parts == g.parts && next.index == g.index + 1
        },
        old(guided).guide@ is None ==> final(guided).guide@ is None,
{
    let old_pos = guided.cursor.pos;
    let ghost old_prefix = guided.cursor.prefix@;
    let ghost old_guide = guided.guide@;
    proof {
        reveal(guided_cursor_ok);
        reveal(parts_progress);
        reveal(cursor_ok);
        assert(cursor_ok(bytes@, &guided.cursor));
    }
    let ghost name_expected = match (old_guide, expected@) {
        (Some(g), Some(name)) => {
            Some(name_part_expected(
                bytes@,
                &guided.cursor,
                g.parts,
                g.index,
                name,
                next@,
                bytes.len(),
            ))
        },
        _ => None,
    };
    proof {
        reveal(guided_cursor_ok);
    }
    let field = match cursor_name(
        bytes,
        &mut guided.cursor,
        Ghost(name_expected),
    at,
    ) {
        Some(field) => field,
        None => {
            proof {
                if old_guide is Some {
                    assert(false);
                }
            }
            return None;
        },
    };
    let ghost next_guide = match old_guide {
        Some(g) => Some(GPartsGuide { parts: g.parts, index: g.index + 1 }),
        None => None,
    };
    proof {
        if let Some(g) = old_guide {
            advance_parts_progress(
                bytes@,
                g.parts,
                g.index,
                old_pos,
                old_prefix,
                &guided.cursor,
            );
        }
    }
    guided.guide = Ghost(next_guide);
    proof {
        reveal(guided_cursor_ok);
    }
    Some(field)
}

fn guided_term(
    bytes: &[u8],
    guided: &mut EGuidedCursor,
    expected: Ghost<Option<(Term, u8)>>,
at: &mut usize,
) -> (r: Option<ESpannedTerm>)
    requires
        *old(at) <= bytes@.len(),
        guided_cursor_ok(bytes@, old(guided)),
        old(guided).guide@ matches Some(g) ==> expected@ matches Some(e)
            && {
                &&& 0 <= g.index
                &&& g.index + 1 < g.parts.len()
                &&& g.parts[g.index]
                    == ckc_spec::v1text::term_bytes(e.0)
                &&& ckc_spec::term::wf_term(e.0)
                &&& g.parts[g.index + 1].len() > 0
                &&& g.parts[g.index + 1][0] == e.1
                &&& (e.1 == 0x2c || e.1 == 0x29 || e.1 == 0x5d
                    || e.1 == 0x7c || e.1 == 0x7d || e.1 == 0x20
                    || e.1 == 0x2e && g.parts[g.index + 1].len() > 1
                        && g.parts[g.index + 1][1] == 0x0a)
            },
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r matches Some(term) ==> {
            &&& guided_cursor_ok(bytes@, final(guided))
            &&& spanned_term_ok(bytes@, &term)
            &&& term.start == old(guided).cursor.pos
            &&& final(guided).cursor.pos == term.end
            &&& final(guided).cursor.prefix@
                == old(guided).cursor.prefix@
                    + ckc_spec::v1text::term_bytes(term@)
        },
        old(guided).guide@ matches Some(g) ==> {
            &&& r matches Some(term)
            && expected@ matches Some(e) && term@ == e.0
            &&& final(guided).guide@ matches Some(next)
                && next.parts == g.parts && next.index == g.index + 1
        },
        old(guided).guide@ is None ==> final(guided).guide@ is None,
{
    let old_pos = guided.cursor.pos;
    let ghost old_prefix = guided.cursor.prefix@;
    let ghost old_guide = guided.guide@;
    proof {
        reveal(guided_cursor_ok);
        reveal(parts_progress);
        reveal(cursor_ok);
        assert(cursor_ok(bytes@, &guided.cursor));
    }
    let ghost term_expected = match (old_guide, expected@) {
        (Some(g), Some(e)) => {
            Some(term_part_expected(
                bytes@,
                &guided.cursor,
                g.parts,
                g.index,
                e.0,
                e.1,
                bytes.len(),
            ))
        },
        _ => None,
    };
    proof {
        reveal(guided_cursor_ok);
    }
    let term = match cursor_term(
        bytes,
        &mut guided.cursor,
        Ghost(term_expected),
    at,
    ) {
        Some(term) => term,
        None => {
            proof {
                if old_guide is Some {
                    assert(false);
                }
            }
            return None;
        },
    };
    let ghost next_guide = match old_guide {
        Some(g) => Some(GPartsGuide { parts: g.parts, index: g.index + 1 }),
        None => None,
    };
    proof {
        if let Some(g) = old_guide {
            advance_parts_progress(
                bytes@,
                g.parts,
                g.index,
                old_pos,
                old_prefix,
                &guided.cursor,
            );
        }
    }
    guided.guide = Ghost(next_guide);
    proof {
        reveal(guided_cursor_ok);
    }
    Some(term)
}

#[verifier::rlimit(100)]
proof fn answers_flat_is_print(a: ckc_spec::v1text::AnswersFile)
    ensures answers_flat(a) == ckc_spec::v1text::print_answers(a),
{
    let v1_name = ckc_spec::v1text::ascii("v1"@);
    let query_name = ckc_spec::v1text::ascii("query_sha256"@);
    let result_name = ckc_spec::v1text::ascii("result"@);
    let wrapper_name = ckc_spec::v1text::ascii("$guideline_answers"@);
    let v1 = Term::Atom(v1_name);
    let qid = Term::Atom(a.qid);
    let qsha = Term::Atom(a.qsha);
    let query = Term::Comp(query_name, seq![qsha]);
    let result = Term::Comp(result_name, seq![a.result]);
    reveal_strlit("query_sha256");
    reveal_strlit("result");
    reveal(ckc_spec::v1text::ascii);
    reveal(ckc_spec::v1text::curly_name);
    assert(query_name != ckc_spec::v1text::curly_name());
    assert(result_name != ckc_spec::v1text::curly_name());
    atom_term_bytes(v1_name);
    atom_term_bytes(a.qid);
    atom_term_bytes(a.qsha);
    args_one_bytes(qsha);
    regular_comp_bytes(query_name, seq![qsha]);
    args_one_bytes(a.result);
    regular_comp_bytes(result_name, seq![a.result]);
    args_four_bytes(v1, qid, query, result);
    regular_comp_bytes(wrapper_name, seq![v1, qid, query, result]);
    reveal(answers_flat);
    reveal(ckc_spec::v1text::print_answers);
    reveal(ckc_spec::v1text::answers_line1);
    reveal(ckc_spec::v1text::term_line);
    reveal(ckc_spec::v1text::answers_record_term);
}

pub struct EParsedV1 {
    pub file: Ghost<ckc_spec::v1text::V1File>,
}

impl View for EParsedV1 {
    type V = ckc_spec::v1text::V1File;

    open spec fn view(&self) -> ckc_spec::v1text::V1File {
        self.file@
    }
}

pub open spec fn parsed_v1_ok(bytes: Seq<u8>, parsed: &EParsedV1) -> bool {
    ckc_spec::v1text::wf_v1(parsed@)
        && ckc_spec::v1text::print_v1(parsed@) == bytes
}

pub closed spec fn answers_progress(
    bytes: Seq<u8>,
    a: ckc_spec::v1text::AnswersFile,
    i: int,
    pos: usize,
    prefix: Seq<u8>,
) -> bool {
    let parts = answers_parts(a);
    &&& ckc_spec::v1text::wf_answers(a)
    &&& bytes == parts.flatten()
    &&& 0 <= i <= parts.len()
    &&& pos <= bytes.len()
    &&& prefix == bytes.subrange(0, pos as int)
    &&& prefix == parts.take(i).flatten()
}

proof fn answers_part_ready(
    bytes: Seq<u8>,
    a: ckc_spec::v1text::AnswersFile,
    i: int,
    cursor: &EByteCursor,
)
    requires
        answers_progress(bytes, a, i, cursor.pos, cursor.prefix@),
        0 <= i < answers_parts(a).len(),
    ensures
        cursor.pos as int + answers_parts(a)[i].len() <= bytes.len(),
        bytes.subrange(
            cursor.pos as int,
            cursor.pos as int + answers_parts(a)[i].len(),
        ) == answers_parts(a)[i],
{
    reveal(answers_progress);
    reveal(cursor_ok);
    assert(cursor_ok(bytes, cursor));
    cursor_part_ready(bytes, cursor, answers_parts(a), i);
}

proof fn answers_byte_ready(
    bytes: Seq<u8>,
    a: ckc_spec::v1text::AnswersFile,
    i: int,
    cursor: &EByteCursor,
    byte: u8,
)
    requires
        answers_progress(bytes, a, i, cursor.pos, cursor.prefix@),
        0 <= i < answers_parts(a).len(),
        answers_parts(a)[i] == seq![byte],
    ensures
        cursor.pos < bytes.len(),
        bytes[cursor.pos as int] == byte,
{
    answers_part_ready(bytes, a, i, cursor);
    assert(bytes.subrange(
        cursor.pos as int,
        cursor.pos as int + 1,
    )[0] == bytes[cursor.pos as int]);
}

proof fn advance_answers_progress(
    bytes: Seq<u8>,
    a: ckc_spec::v1text::AnswersFile,
    i: int,
    before_pos: usize,
    before_prefix: Seq<u8>,
    after: &EByteCursor,
)
    requires
        answers_progress(bytes, a, i, before_pos, before_prefix),
        0 <= i < answers_parts(a).len(),
        cursor_ok(bytes, after),
        after.prefix@ == before_prefix + answers_parts(a)[i],
    ensures
        answers_progress(bytes, a, i + 1, after.pos, after.prefix@),
{
    reveal(answers_progress);
    reveal(cursor_ok);
    flatten_take_step(answers_parts(a), i);
}

proof fn answers_atom_expected(
    bytes: Seq<u8>,
    a: ckc_spec::v1text::AnswersFile,
    i: int,
    cursor: &EByteCursor,
    name: Seq<u8>,
    next: u8,
    bound: usize,
) -> (g: GAtomExpected)
    requires
        answers_progress(bytes, a, i, cursor.pos, cursor.prefix@),
        0 <= i,
        i + 1 < answers_parts(a).len(),
        answers_parts(a)[i] == ckc_spec::v1text::atom_bytes(name),
        answers_parts(a)[i + 1].len() > 0,
        answers_parts(a)[i + 1][0] == next,
        next == 0x28 || next == 0x2c || next == 0x29,
        bytes.len() == bound,
    ensures
        cursor.pos < g.end <= bytes.len(),
        ckc_spec::v1text::atom_bytes(g.name)
            == bytes.subrange(cursor.pos as int, g.end as int),
        atom_boundary(bytes, g.end as int),
        g.name == name,
{
    reveal(answers_progress);
    reveal(cursor_ok);
    assert(cursor_ok(bytes, cursor));
    atom_part_expected(
        bytes,
        cursor,
        answers_parts(a),
        i,
        name,
        next,
        bound,
    )
}

proof fn answers_name_expected(
    bytes: Seq<u8>,
    a: ckc_spec::v1text::AnswersFile,
    cursor: &EByteCursor,
    bound: usize,
) -> (g: GNameExpected)
    requires
        answers_progress(bytes, a, 1, cursor.pos, cursor.prefix@),
        answers_parts(a)[2].len() > 0,
        answers_parts(a)[2][0] == 0x20,
        bytes.len() == bound,
    ensures
        cursor.pos < g.end < bytes.len(),
        ckc_spec::v1text::name_ok(g.value),
        g.value == bytes.subrange(cursor.pos as int, g.end as int),
        !(ckc_spec::v1text::is_lower_b(bytes[g.end as int])
            || ckc_spec::v1text::is_digit_b(bytes[g.end as int])
            || bytes[g.end as int] == 0x2d),
        g.value == a.qid,
{
    reveal(answers_progress);
    reveal(cursor_ok);
    assert(cursor_ok(bytes, cursor));
    reveal(answers_parts);
    name_part_expected(
        bytes,
        cursor,
        answers_parts(a),
        1,
        a.qid,
        0x20u8,
        bound,
    )
}

proof fn answers_term_expected(
    bytes: Seq<u8>,
    a: ckc_spec::v1text::AnswersFile,
    cursor: &EByteCursor,
    bound: usize,
) -> (g: GTermExpected)
    requires
        answers_progress(bytes, a, 16, cursor.pos, cursor.prefix@),
        answers_parts(a)[17].len() > 0,
        answers_parts(a)[17][0] == 0x29,
        bytes.len() == bound,
    ensures term_at(bytes, cursor.pos as int, g.end as int, g.term),
        g.term == a.result,
{
    reveal(answers_progress);
    reveal(cursor_ok);
    reveal(ckc_spec::v1text::wf_answers);
    assert(cursor_ok(bytes, cursor));
    reveal(answers_parts);
    term_part_expected(
        bytes,
        cursor,
        answers_parts(a),
        16,
        a.result,
        0x29,
        bound,
    )
}

#[verifier::rlimit(2000)]
#[verifier::spinoff_prover]
pub fn parse_answers(
    bytes: &[u8],
    expected: Ghost<Option<ckc_spec::v1text::AnswersFile>>,
at: &mut usize,
) -> (r: Option<EParsedV1>)
    requires
        *old(at) <= bytes@.len(),
        expected@ matches Some(a) ==>
            ckc_spec::v1text::wf_answers(a)
                && ckc_spec::v1text::print_answers(a) == bytes@,
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r matches Some(parsed) ==> parsed_v1_ok(bytes@, &parsed),
        expected@ matches Some(a) ==> r matches Some(parsed)
            && parsed@ == ckc_spec::v1text::V1File::Answers(a),
{
    let mut cursor = new_cursor(bytes);
    proof {
        if let Some(a) = expected@ {
            assert(answers_progress(
                bytes@,
                a,
                0,
                cursor.pos,
                cursor.prefix@,
            )) by {
                answers_flat_is_print(a);
                answers_parts_flat(a);
                reveal(answers_progress);
                reveal_with_fuel(Seq::<_>::flatten, 1);
                assert(bytes@ == answers_parts(a).flatten());
            }
        }
    }

    let percent_space: &[u8] = b"% ";
    proof {
        reveal_strlit("% ");
        reveal_byteslit(b"% ");
        reveal(ckc_spec::v1text::ascii);
        assert(percent_space@ == ckc_spec::v1text::ascii("% "@));
    }
    let before_pos_0 = cursor.pos;
    let ghost before_0 = cursor.prefix@;
    proof {
        if let Some(a) = expected@ {
            let parts = answers_parts(a);
            answers_part_ready(bytes@, a, 0, &cursor);
            assert(parts[0] == ckc_spec::v1text::ascii("% "@));
        }
    }
    if !cursor_literal(
        bytes,
        &mut cursor,
        percent_space,
        Ghost(ckc_spec::v1text::ascii("% "@)),
    at,
    ) {
        return None;
    }
    proof {
        if let Some(a) = expected@ {
            advance_answers_progress(
                bytes@,
                a,
                0,
                before_pos_0,
                before_0,
                &cursor,
            );
        }
    }

    let before_pos_1 = cursor.pos;
    let ghost before_1 = cursor.prefix@;
    proof {
        if let Some(a) = expected@ {
            reveal_strlit(
                " answered against the loaded composition by ace_to_pl answer mode; do not edit.\n",
            );
            reveal(ckc_spec::v1text::ascii);
            assert(answers_parts(a)[2].len() > 0);
            assert(answers_parts(a)[2][0] == 0x20);
        }
    }
    let ghost line_qid_expected = match expected@ {
        Some(a) => Some(answers_name_expected(
            bytes@,
            a,
            &cursor,
            bytes.len(),
        )),
        None => None,
    };
    let line_qid = match cursor_name(
        bytes,
        &mut cursor,
        Ghost(line_qid_expected),
    at,
    ) {
        Some(field) => field,
        None => return None,
    };
    proof {
        if let Some(a) = expected@ {
            advance_answers_progress(
                bytes@,
                a,
                1,
                before_pos_1,
                before_1,
                &cursor,
            );
        }
    }

    let line_suffix: &[u8] = b" answered against the loaded composition by ace_to_pl answer mode; do not edit.\n";
    proof {
        reveal_strlit(
            " answered against the loaded composition by ace_to_pl answer mode; do not edit.\n",
        );
        reveal_byteslit(
            b" answered against the loaded composition by ace_to_pl answer mode; do not edit.\n",
        );
        reveal(ckc_spec::v1text::ascii);
        assert(line_suffix@ == ckc_spec::v1text::ascii(
            " answered against the loaded composition by ace_to_pl answer mode; do not edit.\n"@,
        ));
    }
    let before_pos_2 = cursor.pos;
    let ghost before_2 = cursor.prefix@;
    proof {
        if let Some(a) = expected@ {
            let parts = answers_parts(a);
            answers_part_ready(bytes@, a, 2, &cursor);
            assert(parts[2] == ckc_spec::v1text::ascii(
                " answered against the loaded composition by ace_to_pl answer mode; do not edit.\n"@,
            ));
        }
    }
    if !cursor_literal(
        bytes,
        &mut cursor,
        line_suffix,
        Ghost(ckc_spec::v1text::ascii(
            " answered against the loaded composition by ace_to_pl answer mode; do not edit.\n"@,
        )),
    at,
    ) {
        return None;
    }
    proof {
        if let Some(a) = expected@ {
            advance_answers_progress(
                bytes@,
                a,
                2,
                before_pos_2,
                before_2,
                &cursor,
            );
        }
    }

    let before_pos_3 = cursor.pos;
    let ghost before_3 = cursor.prefix@;
    let ghost wrapper_expected = match expected@ {
        Some(a) => Some(answers_atom_expected(
            bytes@,
            a,
            3,
            &cursor,
            ckc_spec::v1text::ascii("$guideline_answers"@),
            0x28,
            bytes.len(),
        )),
        None => None,
    };
    let wrapper = match cursor_atom(
        bytes,
        &mut cursor,
        Ghost(wrapper_expected),
    at,
    ) {
        Some(atom) => atom,
        None => return None,
    };
    proof {
        if let Some(a) = expected@ {
            advance_answers_progress(
                bytes@,
                a,
                3,
                before_pos_3,
                before_3,
                &cursor,
            );
        }
    }
    let wrapper_name: &[u8] = b"$guideline_answers";
    proof {
        reveal_strlit("$guideline_answers");
        reveal_byteslit(b"$guideline_answers");
        reveal(ckc_spec::v1text::ascii);
        assert(wrapper_name@
            == ckc_spec::v1text::ascii("$guideline_answers"@));
    }
    if !vec_slice_equal(&wrapper.name, wrapper_name) {
        return None;
    }
    let before_pos_4 = cursor.pos;
    let ghost before_4 = cursor.prefix@;
    proof {
        if let Some(a) = expected@ {
            let parts = answers_parts(a);
            assert(parts[4] == seq![0x28u8]);
            answers_byte_ready(bytes@, a, 4, &cursor, 0x28);
        }
    }
    if !cursor_byte(bytes, &mut cursor, 0x28, at) {
        return None;
    }
    proof {
        if let Some(a) = expected@ {
            advance_answers_progress(
                bytes@,
                a,
                4,
                before_pos_4,
                before_4,
                &cursor,
            );
        }
    }

    let before_pos_5 = cursor.pos;
    let ghost before_5 = cursor.prefix@;
    let ghost version_expected = match expected@ {
        Some(a) => Some(answers_atom_expected(
            bytes@,
            a,
            5,
            &cursor,
            ckc_spec::v1text::ascii("v1"@),
            0x2c,
            bytes.len(),
        )),
        None => None,
    };
    let version = match cursor_atom(
        bytes,
        &mut cursor,
        Ghost(version_expected),
    at,
    ) {
        Some(atom) => atom,
        None => return None,
    };
    proof {
        if let Some(a) = expected@ {
            advance_answers_progress(
                bytes@,
                a,
                5,
                before_pos_5,
                before_5,
                &cursor,
            );
        }
    }
    let version_name: &[u8] = b"v1";
    proof {
        reveal_strlit("v1");
        reveal_byteslit(b"v1");
        reveal(ckc_spec::v1text::ascii);
        assert(version_name@ == ckc_spec::v1text::ascii("v1"@));
    }
    if !vec_slice_equal(&version.name, version_name) {
        return None;
    }
    let before_pos_6 = cursor.pos;
    let ghost before_6 = cursor.prefix@;
    proof {
        if let Some(a) = expected@ {
            let parts = answers_parts(a);
            assert(parts[6] == seq![0x2cu8]);
            answers_byte_ready(bytes@, a, 6, &cursor, 0x2c);
        }
    }
    if !cursor_byte(bytes, &mut cursor, 0x2c, at) {
        return None;
    }
    proof {
        if let Some(a) = expected@ {
            advance_answers_progress(
                bytes@,
                a,
                6,
                before_pos_6,
                before_6,
                &cursor,
            );
        }
    }

    let before_pos_7 = cursor.pos;
    let ghost before_7 = cursor.prefix@;
    let ghost record_qid_expected = match expected@ {
        Some(a) => Some(answers_atom_expected(
            bytes@,
            a,
            7,
            &cursor,
            a.qid,
            0x2c,
            bytes.len(),
        )),
        None => None,
    };
    let record_qid = match cursor_atom(
        bytes,
        &mut cursor,
        Ghost(record_qid_expected),
    at,
    ) {
        Some(atom) => atom,
        None => return None,
    };
    proof {
        if let Some(a) = expected@ {
            advance_answers_progress(
                bytes@,
                a,
                7,
                before_pos_7,
                before_7,
                &cursor,
            );
        }
    }
    if !vec_equal(&line_qid.value, &record_qid.name) {
        return None;
    }
    let before_pos_8 = cursor.pos;
    let ghost before_8 = cursor.prefix@;
    proof {
        if let Some(a) = expected@ {
            let parts = answers_parts(a);
            assert(parts[8] == seq![0x2cu8]);
            answers_byte_ready(bytes@, a, 8, &cursor, 0x2c);
        }
    }
    if !cursor_byte(bytes, &mut cursor, 0x2c, at) {
        return None;
    }
    proof {
        if let Some(a) = expected@ {
            advance_answers_progress(
                bytes@,
                a,
                8,
                before_pos_8,
                before_8,
                &cursor,
            );
        }
    }

    let before_pos_9 = cursor.pos;
    let ghost before_9 = cursor.prefix@;
    let ghost query_wrapper_expected = match expected@ {
        Some(a) => Some(answers_atom_expected(
            bytes@,
            a,
            9,
            &cursor,
            ckc_spec::v1text::ascii("query_sha256"@),
            0x28,
            bytes.len(),
        )),
        None => None,
    };
    let query_wrapper = match cursor_atom(
        bytes,
        &mut cursor,
        Ghost(query_wrapper_expected),
    at,
    ) {
        Some(atom) => atom,
        None => return None,
    };
    proof {
        if let Some(a) = expected@ {
            advance_answers_progress(
                bytes@,
                a,
                9,
                before_pos_9,
                before_9,
                &cursor,
            );
        }
    }
    let query_wrapper_name: &[u8] = b"query_sha256";
    proof {
        reveal_strlit("query_sha256");
        reveal_byteslit(b"query_sha256");
        reveal(ckc_spec::v1text::ascii);
        assert(query_wrapper_name@
            == ckc_spec::v1text::ascii("query_sha256"@));
    }
    if !vec_slice_equal(&query_wrapper.name, query_wrapper_name) {
        return None;
    }
    let before_pos_10 = cursor.pos;
    let ghost before_10 = cursor.prefix@;
    proof {
        if let Some(a) = expected@ {
            let parts = answers_parts(a);
            assert(parts[10] == seq![0x28u8]);
            answers_byte_ready(bytes@, a, 10, &cursor, 0x28);
        }
    }
    if !cursor_byte(bytes, &mut cursor, 0x28, at) {
        return None;
    }
    proof {
        if let Some(a) = expected@ {
            advance_answers_progress(
                bytes@,
                a,
                10,
                before_pos_10,
                before_10,
                &cursor,
            );
        }
    }

    let before_pos_11 = cursor.pos;
    let ghost before_11 = cursor.prefix@;
    let ghost qsha_expected = match expected@ {
        Some(a) => Some(answers_atom_expected(
            bytes@,
            a,
            11,
            &cursor,
            a.qsha,
            0x29,
            bytes.len(),
        )),
        None => None,
    };
    let qsha = match cursor_atom(
        bytes,
        &mut cursor,
        Ghost(qsha_expected),
    at,
    ) {
        Some(atom) => atom,
        None => return None,
    };
    proof {
        if let Some(a) = expected@ {
            advance_answers_progress(
                bytes@,
                a,
                11,
                before_pos_11,
                before_11,
                &cursor,
            );
        }
    }
    proof {
        if let Some(a) = expected@ {
            reveal(answers_progress);
            reveal(ckc_spec::v1text::wf_answers);
            assert(qsha.name@ == a.qsha);
            assert(ckc_spec::v1text::hex64(qsha.name@));
        }
    }
    if !hex64_exec(&qsha.name) {
        return None;
    }

    let before_pos_12 = cursor.pos;
    let ghost before_12 = cursor.prefix@;
    proof {
        if let Some(a) = expected@ {
            let parts = answers_parts(a);
            assert(parts[12] == seq![0x29u8]);
            answers_byte_ready(bytes@, a, 12, &cursor, 0x29);
        }
    }
    if !cursor_byte(bytes, &mut cursor, 0x29, at) {
        return None;
    }
    proof {
        if let Some(a) = expected@ {
            advance_answers_progress(
                bytes@,
                a,
                12,
                before_pos_12,
                before_12,
                &cursor,
            );
        }
    }

    let before_pos_13 = cursor.pos;
    let ghost before_13 = cursor.prefix@;
    proof {
        if let Some(a) = expected@ {
            let parts = answers_parts(a);
            assert(parts[13] == seq![0x2cu8]);
            answers_byte_ready(bytes@, a, 13, &cursor, 0x2c);
        }
    }
    if !cursor_byte(bytes, &mut cursor, 0x2c, at) {
        return None;
    }
    proof {
        if let Some(a) = expected@ {
            advance_answers_progress(
                bytes@,
                a,
                13,
                before_pos_13,
                before_13,
                &cursor,
            );
        }
    }

    let before_pos_14 = cursor.pos;
    let ghost before_14 = cursor.prefix@;
    let ghost result_wrapper_expected = match expected@ {
        Some(a) => Some(answers_atom_expected(
            bytes@,
            a,
            14,
            &cursor,
            ckc_spec::v1text::ascii("result"@),
            0x28,
            bytes.len(),
        )),
        None => None,
    };
    let result_wrapper = match cursor_atom(
        bytes,
        &mut cursor,
        Ghost(result_wrapper_expected),
    at,
    ) {
        Some(atom) => atom,
        None => return None,
    };
    proof {
        if let Some(a) = expected@ {
            advance_answers_progress(
                bytes@,
                a,
                14,
                before_pos_14,
                before_14,
                &cursor,
            );
        }
    }
    let result_wrapper_name: &[u8] = b"result";
    proof {
        reveal_strlit("result");
        reveal_byteslit(b"result");
        reveal(ckc_spec::v1text::ascii);
        assert(result_wrapper_name@
            == ckc_spec::v1text::ascii("result"@));
    }
    if !vec_slice_equal(&result_wrapper.name, result_wrapper_name) {
        return None;
    }
    let before_pos_15 = cursor.pos;
    let ghost before_15 = cursor.prefix@;
    proof {
        if let Some(a) = expected@ {
            let parts = answers_parts(a);
            assert(parts[15] == seq![0x28u8]);
            answers_byte_ready(bytes@, a, 15, &cursor, 0x28);
        }
    }
    if !cursor_byte(bytes, &mut cursor, 0x28, at) {
        return None;
    }
    proof {
        if let Some(a) = expected@ {
            advance_answers_progress(
                bytes@,
                a,
                15,
                before_pos_15,
                before_15,
                &cursor,
            );
        }
    }

    let before_pos_16 = cursor.pos;
    let ghost before_16 = cursor.prefix@;
    let ghost result_expected = match expected@ {
        Some(a) => Some(answers_term_expected(
            bytes@,
            a,
            &cursor,
            bytes.len(),
        )),
        None => None,
    };
    let result = match cursor_term(
        bytes,
        &mut cursor,
        Ghost(result_expected),
    at,
    ) {
        Some(term) => term,
        None => return None,
    };
    proof {
        if let Some(a) = expected@ {
            advance_answers_progress(
                bytes@,
                a,
                16,
                before_pos_16,
                before_16,
                &cursor,
            );
        }
    }
    proof {
        if let Some(a) = expected@ {
            reveal(answers_progress);
            reveal(ckc_spec::v1text::wf_answers);
            reveal(spanned_term_ok);
            reveal(parsed_term_ok);
            assert(result@ == a.result);
            assert(result.parsed.ground);
            assert(result.parsed.no_dollar);
        }
    }
    if !result.parsed.ground || !result.parsed.no_dollar {
        return None;
    }

    let before_pos_17 = cursor.pos;
    let ghost before_17 = cursor.prefix@;
    proof {
        if let Some(a) = expected@ {
            let parts = answers_parts(a);
            assert(parts[17] == seq![0x29u8]);
            answers_byte_ready(bytes@, a, 17, &cursor, 0x29);
        }
    }
    if !cursor_byte(bytes, &mut cursor, 0x29, at) {
        return None;
    }
    proof {
        if let Some(a) = expected@ {
            advance_answers_progress(
                bytes@,
                a,
                17,
                before_pos_17,
                before_17,
                &cursor,
            );
        }
    }

    let before_pos_18 = cursor.pos;
    let ghost before_18 = cursor.prefix@;
    proof {
        if let Some(a) = expected@ {
            let parts = answers_parts(a);
            assert(parts[18] == seq![0x29u8]);
            answers_byte_ready(bytes@, a, 18, &cursor, 0x29);
        }
    }
    if !cursor_byte(bytes, &mut cursor, 0x29, at) {
        return None;
    }
    proof {
        if let Some(a) = expected@ {
            advance_answers_progress(
                bytes@,
                a,
                18,
                before_pos_18,
                before_18,
                &cursor,
            );
        }
    }

    let line_end: &[u8] = b".\n";
    proof {
        reveal_strlit(".\n");
        reveal_byteslit(b".\n");
        reveal(ckc_spec::v1text::ascii);
        assert(line_end@ == ckc_spec::v1text::ascii(".\n"@));
    }
    let before_pos_19 = cursor.pos;
    let ghost before_19 = cursor.prefix@;
    proof {
        if let Some(a) = expected@ {
            let parts = answers_parts(a);
            answers_part_ready(bytes@, a, 19, &cursor);
            assert(parts[19] == ckc_spec::v1text::ascii(".\n"@));
        }
    }
    if !cursor_literal(
        bytes,
        &mut cursor,
        line_end,
        Ghost(ckc_spec::v1text::ascii(".\n"@)),
    at,
    ) {
        return None;
    }
    proof {
        if let Some(a) = expected@ {
            advance_answers_progress(
                bytes@,
                a,
                19,
                before_pos_19,
                before_19,
                &cursor,
            );
            answers_parts(a).lemma_take_len();
        }
    }
    proof {
        if let Some(a) = expected@ {
            reveal(answers_progress);
            answers_parts(a).lemma_take_len();
            assert(cursor.prefix@ == bytes@);
            assert(cursor.prefix@.len() == cursor.pos);
            assert(cursor.pos == bytes.len());
        }
    }
    if cursor.pos != bytes.len() {
        return None;
    }

    let ghost model = ckc_spec::v1text::AnswersFile {
        qid: line_qid.value@,
        qsha: qsha.name@,
        result: result@,
    };
    proof {
        reveal(spanned_term_ok);
        reveal(parsed_term_ok);
        assert(cursor_ok(bytes@, &cursor));
        reveal(cursor_ok);
        assert(cursor.pos == bytes@.len());
        assert(cursor.prefix@ == bytes@.subrange(0, bytes@.len() as int));
        assert_seqs_equal!(bytes@.subrange(0, bytes@.len() as int) == bytes@);
        assert(cursor.prefix@ == bytes@);
        reveal(answers_flat);
        answers_flat_is_print(model);
        assert_seqs_equal!(cursor.prefix@ == answers_flat(model));
        if let Some(a) = expected@ {
            assert(line_qid.value@ == a.qid);
            assert(qsha.name@ == a.qsha);
            assert(result@ == a.result);
            assert(model == a);
        }
        reveal(ckc_spec::v1text::wf_answers);
        reveal(ckc_spec::v1text::wf_v1);
        reveal(ckc_spec::v1text::print_v1);
        reveal(parsed_v1_ok);
    }
    Some(EParsedV1 {
        file: Ghost(ckc_spec::v1text::V1File::Answers(model)),
    })
}

#[verifier::rlimit(200)]
proof fn args_five_bytes(
    a: Term,
    b: Term,
    c: Term,
    d: Term,
    e: Term,
)
    ensures
        ckc_spec::v1text::args_bytes(seq![a, b, c, d, e])
            == ckc_spec::v1text::term_bytes(a)
                + seq![0x2cu8]
                + ckc_spec::v1text::term_bytes(b)
                + seq![0x2cu8]
                + ckc_spec::v1text::term_bytes(c)
                + seq![0x2cu8]
                + ckc_spec::v1text::term_bytes(d)
                + seq![0x2cu8]
                + ckc_spec::v1text::term_bytes(e),
{
    reveal_with_fuel(ckc_spec::v1text::args_bytes, 6);
}

pub open spec fn traces_flat(
    t: ckc_spec::v1text::TracesFile,
) -> Seq<u8> {
    traces_line_stage(t.qid)
        + traces_qsha_stage(t.qid, t.qsha)
        + traces_asha_stage(t.asha)
        + traces_result_stage(t.result)
}

pub open spec fn traces_parts(
    t: ckc_spec::v1text::TracesFile,
) -> Seq<Seq<u8>> {
    seq![
        ckc_spec::v1text::ascii("% "@),
        t.qid,
        ckc_spec::v1text::ascii(
            " traced against the loaded composition by ace_to_pl trace mode; do not edit.\n"@,
        ),
        ckc_spec::v1text::atom_bytes(
            ckc_spec::v1text::ascii("$guideline_traces"@),
        ),
        seq![0x28u8],
        ckc_spec::v1text::atom_bytes(ckc_spec::v1text::ascii("v1"@)),
        seq![0x2cu8],
        ckc_spec::v1text::atom_bytes(t.qid),
        seq![0x2cu8],
        ckc_spec::v1text::atom_bytes(
            ckc_spec::v1text::ascii("query_sha256"@),
        ),
        seq![0x28u8],
        ckc_spec::v1text::atom_bytes(t.qsha),
        seq![0x29u8],
        seq![0x2cu8],
        ckc_spec::v1text::atom_bytes(
            ckc_spec::v1text::ascii("answers_sha256"@),
        ),
        seq![0x28u8],
        ckc_spec::v1text::atom_bytes(t.asha),
        seq![0x29u8],
        seq![0x2cu8],
        ckc_spec::v1text::atom_bytes(ckc_spec::v1text::ascii("result"@)),
        seq![0x28u8],
        ckc_spec::v1text::term_bytes(t.result),
        seq![0x29u8],
        seq![0x29u8],
        ckc_spec::v1text::ascii(".\n"@),
    ]
}

#[verifier::rlimit(100)]
proof fn traces_parts_flat(t: ckc_spec::v1text::TracesFile)
    ensures traces_parts(t).flatten() == traces_flat(t),
{
    let line = seq![
        ckc_spec::v1text::ascii("% "@),
        t.qid,
        ckc_spec::v1text::ascii(
            " traced against the loaded composition by ace_to_pl trace mode; do not edit.\n"@,
        ),
    ];
    let qsha = seq![
        ckc_spec::v1text::atom_bytes(
            ckc_spec::v1text::ascii("$guideline_traces"@),
        ),
        seq![0x28u8],
        ckc_spec::v1text::atom_bytes(ckc_spec::v1text::ascii("v1"@)),
        seq![0x2cu8],
        ckc_spec::v1text::atom_bytes(t.qid),
        seq![0x2cu8],
        ckc_spec::v1text::atom_bytes(
            ckc_spec::v1text::ascii("query_sha256"@),
        ),
        seq![0x28u8],
        ckc_spec::v1text::atom_bytes(t.qsha),
        seq![0x29u8],
        seq![0x2cu8],
    ];
    let asha = seq![
        ckc_spec::v1text::atom_bytes(
            ckc_spec::v1text::ascii("answers_sha256"@),
        ),
        seq![0x28u8],
        ckc_spec::v1text::atom_bytes(t.asha),
        seq![0x29u8],
        seq![0x2cu8],
    ];
    let result = seq![
        ckc_spec::v1text::atom_bytes(ckc_spec::v1text::ascii("result"@)),
        seq![0x28u8],
        ckc_spec::v1text::term_bytes(t.result),
        seq![0x29u8],
        seq![0x29u8],
        ckc_spec::v1text::ascii(".\n"@),
    ];
    assert(traces_parts(t) == (line + qsha) + (asha + result)) by {
        reveal(traces_parts);
    }
    assert(line.flatten() == traces_line_stage(t.qid)) by {
        reveal(traces_line_stage);
        reveal_with_fuel(Seq::<_>::flatten, 5);
    }
    assert(qsha.flatten() == traces_qsha_stage(t.qid, t.qsha)) by {
        reveal(traces_qsha_stage);
        reveal_with_fuel(Seq::<_>::flatten, 13);
    }
    assert(asha.flatten() == traces_asha_stage(t.asha)) by {
        reveal(traces_asha_stage);
        reveal_with_fuel(Seq::<_>::flatten, 7);
    }
    assert(result.flatten() == traces_result_stage(t.result)) by {
        reveal(traces_result_stage);
        reveal_with_fuel(Seq::<_>::flatten, 8);
    }
    vstd::seq_lib::lemma_flatten_concat(line, qsha);
    vstd::seq_lib::lemma_flatten_concat(asha, result);
    vstd::seq_lib::lemma_flatten_concat(line + qsha, asha + result);
    traces_stages_flat(t);
}

#[verifier::rlimit(300)]
proof fn traces_flat_is_print(t: ckc_spec::v1text::TracesFile)
    ensures traces_flat(t) == ckc_spec::v1text::print_traces(t),
{
    let v1_name = ckc_spec::v1text::ascii("v1"@);
    let query_name = ckc_spec::v1text::ascii("query_sha256"@);
    let answers_name = ckc_spec::v1text::ascii("answers_sha256"@);
    let result_name = ckc_spec::v1text::ascii("result"@);
    let wrapper_name = ckc_spec::v1text::ascii("$guideline_traces"@);
    let v1 = Term::Atom(v1_name);
    let qid = Term::Atom(t.qid);
    let qsha = Term::Atom(t.qsha);
    let asha = Term::Atom(t.asha);
    let query = Term::Comp(query_name, seq![qsha]);
    let answers = Term::Comp(answers_name, seq![asha]);
    let result = Term::Comp(result_name, seq![t.result]);
    reveal_strlit("query_sha256");
    reveal_strlit("answers_sha256");
    reveal_strlit("result");
    reveal(ckc_spec::v1text::ascii);
    reveal(ckc_spec::v1text::curly_name);
    assert(query_name != ckc_spec::v1text::curly_name());
    assert(answers_name != ckc_spec::v1text::curly_name());
    assert(result_name != ckc_spec::v1text::curly_name());
    atom_term_bytes(v1_name);
    atom_term_bytes(t.qid);
    atom_term_bytes(t.qsha);
    atom_term_bytes(t.asha);
    args_one_bytes(qsha);
    regular_comp_bytes(query_name, seq![qsha]);
    args_one_bytes(asha);
    regular_comp_bytes(answers_name, seq![asha]);
    args_one_bytes(t.result);
    regular_comp_bytes(result_name, seq![t.result]);
    args_five_bytes(v1, qid, query, answers, result);
    regular_comp_bytes(
        wrapper_name,
        seq![v1, qid, query, answers, result],
    );
    reveal(traces_flat);
    reveal(traces_line_stage);
    reveal(traces_qsha_stage);
    reveal(traces_asha_stage);
    reveal(traces_result_stage);
    reveal(ckc_spec::v1text::print_traces);
    reveal(ckc_spec::v1text::traces_line1);
    reveal(ckc_spec::v1text::term_line);
    reveal(ckc_spec::v1text::traces_record_term);
}

pub open spec fn traces_line_stage(qid: Seq<u8>) -> Seq<u8> {
    ckc_spec::v1text::ascii("% "@)
        + qid
        + ckc_spec::v1text::ascii(
            " traced against the loaded composition by ace_to_pl trace mode; do not edit.\n"@,
        )
}

pub open spec fn traces_qsha_stage(
    qid: Seq<u8>,
    qsha: Seq<u8>,
) -> Seq<u8> {
    ckc_spec::v1text::atom_bytes(
        ckc_spec::v1text::ascii("$guideline_traces"@),
    )
        + seq![0x28u8]
        + ckc_spec::v1text::atom_bytes(ckc_spec::v1text::ascii("v1"@))
        + seq![0x2cu8]
        + ckc_spec::v1text::atom_bytes(qid)
        + seq![0x2cu8]
        + ckc_spec::v1text::atom_bytes(
            ckc_spec::v1text::ascii("query_sha256"@),
        )
        + seq![0x28u8]
        + ckc_spec::v1text::atom_bytes(qsha)
        + seq![0x29u8, 0x2cu8]
}

pub open spec fn traces_asha_stage(asha: Seq<u8>) -> Seq<u8> {
    ckc_spec::v1text::atom_bytes(
        ckc_spec::v1text::ascii("answers_sha256"@),
    )
        + seq![0x28u8]
        + ckc_spec::v1text::atom_bytes(asha)
        + seq![0x29u8, 0x2cu8]
}

pub open spec fn traces_result_stage(result: Term) -> Seq<u8> {
    ckc_spec::v1text::atom_bytes(ckc_spec::v1text::ascii("result"@))
        + seq![0x28u8]
        + ckc_spec::v1text::term_bytes(result)
        + seq![0x29u8, 0x29u8]
        + ckc_spec::v1text::ascii(".\n"@)
}

proof fn traces_stages_flat(t: ckc_spec::v1text::TracesFile)
    ensures
        traces_flat(t)
            == traces_line_stage(t.qid)
                + traces_qsha_stage(t.qid, t.qsha)
                + traces_asha_stage(t.asha)
                + traces_result_stage(t.result),
{
    reveal(traces_flat);
}

#[verifier::rlimit(200)]
fn parse_traces_line(
    bytes: &[u8],
    guided: &mut EGuidedCursor,
    expected: Ghost<Option<ckc_spec::v1text::TracesFile>>,
at: &mut usize,
) -> (r: Option<ENameField>)
    requires
        *old(at) <= bytes@.len(),
        guided_cursor_ok(bytes@, old(guided)),
        expected@ matches Some(t) ==> old(guided).guide@ matches Some(g)
            && g.parts == traces_parts(t) && g.index == 0
            && ckc_spec::v1text::wf_traces(t),
        expected@ is None ==> old(guided).guide@ is None,
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r matches Some(qid) ==> {
            &&& guided_cursor_ok(bytes@, final(guided))
            &&& ckc_spec::v1text::name_ok(qid.value@)
            &&& final(guided).cursor.prefix@
                == old(guided).cursor.prefix@ + traces_line_stage(qid.value@)
        },
        expected@ matches Some(t) ==> {
            &&& r matches Some(qid)
            &&& qid.value@ == t.qid
            &&& final(guided).guide@ matches Some(g)
                && g.parts == traces_parts(t) && g.index == 3
        },
        expected@ is None && r is Some ==> final(guided).guide@ is None,
{
    let ghost old_prefix = guided.cursor.prefix@;
    proof {
        if let Some(t) = expected@ {
            reveal(traces_parts);
            reveal(ckc_spec::v1text::wf_traces);
            reveal_strlit(
                " traced against the loaded composition by ace_to_pl trace mode; do not edit.\n",
            );
            reveal(ckc_spec::v1text::ascii);
        }
    }

    let percent_space: &[u8] = b"% ";
    proof {
        reveal_strlit("% ");
        reveal_byteslit(b"% ");
        reveal(ckc_spec::v1text::ascii);
        assert(percent_space@ == ckc_spec::v1text::ascii("% "@));
    }
    if !guided_literal(
        bytes,
        guided,
        percent_space,
        Ghost(ckc_spec::v1text::ascii("% "@)),
    at,
    ) {
        return None;
    }

    let ghost qid_expected = match expected@ {
        Some(t) => Some(t.qid),
        None => None,
    };
    let qid = match guided_name(bytes, guided, Ghost(0x20u8), Ghost(qid_expected), at) {
        Some(field) => field,
        None => return None,
    };

    let line_suffix: &[u8] = b" traced against the loaded composition by ace_to_pl trace mode; do not edit.\n";
    proof {
        reveal_strlit(
            " traced against the loaded composition by ace_to_pl trace mode; do not edit.\n",
        );
        reveal_byteslit(
            b" traced against the loaded composition by ace_to_pl trace mode; do not edit.\n",
        );
        reveal(ckc_spec::v1text::ascii);
        assert(line_suffix@ == ckc_spec::v1text::ascii(
            " traced against the loaded composition by ace_to_pl trace mode; do not edit.\n"@,
        ));
    }
    if !guided_literal(
        bytes,
        guided,
        line_suffix,
        Ghost(ckc_spec::v1text::ascii(
            " traced against the loaded composition by ace_to_pl trace mode; do not edit.\n"@,
        )),
    at,
    ) {
        return None;
    }
    proof {
        reveal(traces_line_stage);
        assert_seqs_equal!(
            guided.cursor.prefix@
                == old_prefix + traces_line_stage(qid.value@)
        );
    }
    Some(qid)
}

#[verifier::rlimit(300)]
fn parse_traces_qsha(
    bytes: &[u8],
    guided: &mut EGuidedCursor,
    line_qid: &ENameField,
    expected: Ghost<Option<ckc_spec::v1text::TracesFile>>,
at: &mut usize,
) -> (r: Option<EParsedAtom>)
    requires
        *old(at) <= bytes@.len(),
        guided_cursor_ok(bytes@, old(guided)),
        ckc_spec::v1text::name_ok(line_qid.value@),
        expected@ matches Some(t) ==> {
            &&& line_qid.value@ == t.qid
            &&& old(guided).guide@ matches Some(g)
                && g.parts == traces_parts(t) && g.index == 3
            &&& ckc_spec::v1text::wf_traces(t)
        },
        expected@ is None ==> old(guided).guide@ is None,
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r matches Some(qsha) ==> {
            &&& guided_cursor_ok(bytes@, final(guided))
            &&& ckc_spec::v1text::hex64(qsha.name@)
            &&& final(guided).cursor.prefix@
                == old(guided).cursor.prefix@
                    + traces_qsha_stage(line_qid.value@, qsha.name@)
        },
        expected@ matches Some(t) ==> {
            &&& r matches Some(qsha)
            &&& qsha.name@ == t.qsha
            &&& final(guided).guide@ matches Some(g)
                && g.parts == traces_parts(t) && g.index == 14
        },
        expected@ is None && r is Some ==> final(guided).guide@ is None,
{
    let ghost old_prefix = guided.cursor.prefix@;
    proof {
        if let Some(t) = expected@ {
            reveal(traces_parts);
            reveal(ckc_spec::v1text::wf_traces);
        }
    }

    let ghost wrapper_expected = match expected@ {
        Some(_) => Some((
            ckc_spec::v1text::ascii("$guideline_traces"@),
            0x28u8,
        )),
        None => None,
    };
    let wrapper = match guided_atom(bytes, guided, Ghost(wrapper_expected), at) {
        Some(atom) => atom,
        None => return None,
    };
    let wrapper_name: &[u8] = b"$guideline_traces";
    proof {
        reveal_strlit("$guideline_traces");
        reveal_byteslit(b"$guideline_traces");
        reveal(ckc_spec::v1text::ascii);
        assert(wrapper_name@
            == ckc_spec::v1text::ascii("$guideline_traces"@));
    }
    if !vec_slice_equal(&wrapper.name, wrapper_name) {
        return None;
    }
    if !guided_byte(bytes, guided, 0x28, at) {
        return None;
    }

    let ghost version_expected = match expected@ {
        Some(_) => Some((ckc_spec::v1text::ascii("v1"@), 0x2cu8)),
        None => None,
    };
    let version = match guided_atom(bytes, guided, Ghost(version_expected), at) {
        Some(atom) => atom,
        None => return None,
    };
    let version_name: &[u8] = b"v1";
    proof {
        reveal_strlit("v1");
        reveal_byteslit(b"v1");
        reveal(ckc_spec::v1text::ascii);
        assert(version_name@ == ckc_spec::v1text::ascii("v1"@));
    }
    if !vec_slice_equal(&version.name, version_name) {
        return None;
    }
    if !guided_byte(bytes, guided, 0x2c, at) {
        return None;
    }

    let ghost record_qid_expected = match expected@ {
        Some(t) => Some((t.qid, 0x2cu8)),
        None => None,
    };
    let record_qid = match guided_atom(
        bytes,
        guided,
        Ghost(record_qid_expected),
    at,
    ) {
        Some(atom) => atom,
        None => return None,
    };
    if !vec_equal(&line_qid.value, &record_qid.name) {
        return None;
    }
    if !guided_byte(bytes, guided, 0x2c, at) {
        return None;
    }

    let ghost query_wrapper_expected = match expected@ {
        Some(_) => Some((
            ckc_spec::v1text::ascii("query_sha256"@),
            0x28u8,
        )),
        None => None,
    };
    let query_wrapper = match guided_atom(
        bytes,
        guided,
        Ghost(query_wrapper_expected),
    at,
    ) {
        Some(atom) => atom,
        None => return None,
    };
    let query_wrapper_name: &[u8] = b"query_sha256";
    proof {
        reveal_strlit("query_sha256");
        reveal_byteslit(b"query_sha256");
        reveal(ckc_spec::v1text::ascii);
        assert(query_wrapper_name@
            == ckc_spec::v1text::ascii("query_sha256"@));
    }
    if !vec_slice_equal(&query_wrapper.name, query_wrapper_name) {
        return None;
    }
    if !guided_byte(bytes, guided, 0x28, at) {
        return None;
    }

    let ghost qsha_expected = match expected@ {
        Some(t) => Some((t.qsha, 0x29u8)),
        None => None,
    };
    let qsha = match guided_atom(bytes, guided, Ghost(qsha_expected), at) {
        Some(atom) => atom,
        None => return None,
    };
    proof {
        if let Some(t) = expected@ {
            assert(qsha.name@ == t.qsha);
            assert(ckc_spec::v1text::hex64(qsha.name@));
        }
    }
    if !hex64_exec(&qsha.name) {
        return None;
    }
    if !guided_byte(bytes, guided, 0x29, at) {
        return None;
    }
    if !guided_byte(bytes, guided, 0x2c, at) {
        return None;
    }

    proof {
        reveal(traces_qsha_stage);
        assert_seqs_equal!(
            guided.cursor.prefix@
                == old_prefix
                    + traces_qsha_stage(line_qid.value@, qsha.name@)
        );
    }
    Some(qsha)
}

#[verifier::rlimit(200)]
fn parse_traces_asha(
    bytes: &[u8],
    guided: &mut EGuidedCursor,
    expected: Ghost<Option<ckc_spec::v1text::TracesFile>>,
at: &mut usize,
) -> (r: Option<EParsedAtom>)
    requires
        *old(at) <= bytes@.len(),
        guided_cursor_ok(bytes@, old(guided)),
        expected@ matches Some(t) ==> old(guided).guide@ matches Some(g)
            && g.parts == traces_parts(t) && g.index == 14
            && ckc_spec::v1text::wf_traces(t),
        expected@ is None ==> old(guided).guide@ is None,
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r matches Some(asha) ==> {
            &&& guided_cursor_ok(bytes@, final(guided))
            &&& ckc_spec::v1text::hex64(asha.name@)
            &&& final(guided).cursor.prefix@
                == old(guided).cursor.prefix@ + traces_asha_stage(asha.name@)
        },
        expected@ matches Some(t) ==> {
            &&& r matches Some(asha)
            &&& asha.name@ == t.asha
            &&& final(guided).guide@ matches Some(g)
                && g.parts == traces_parts(t) && g.index == 19
        },
        expected@ is None && r is Some ==> final(guided).guide@ is None,
{
    let ghost old_prefix = guided.cursor.prefix@;
    proof {
        if let Some(t) = expected@ {
            reveal(traces_parts);
            reveal(ckc_spec::v1text::wf_traces);
        }
    }

    let ghost wrapper_expected = match expected@ {
        Some(_) => Some((
            ckc_spec::v1text::ascii("answers_sha256"@),
            0x28u8,
        )),
        None => None,
    };
    let wrapper = match guided_atom(bytes, guided, Ghost(wrapper_expected), at) {
        Some(atom) => atom,
        None => return None,
    };
    let wrapper_name: &[u8] = b"answers_sha256";
    proof {
        reveal_strlit("answers_sha256");
        reveal_byteslit(b"answers_sha256");
        reveal(ckc_spec::v1text::ascii);
        assert(wrapper_name@
            == ckc_spec::v1text::ascii("answers_sha256"@));
    }
    if !vec_slice_equal(&wrapper.name, wrapper_name) {
        return None;
    }
    if !guided_byte(bytes, guided, 0x28, at) {
        return None;
    }

    let ghost asha_expected = match expected@ {
        Some(t) => Some((t.asha, 0x29u8)),
        None => None,
    };
    let asha = match guided_atom(bytes, guided, Ghost(asha_expected), at) {
        Some(atom) => atom,
        None => return None,
    };
    proof {
        if let Some(t) = expected@ {
            assert(asha.name@ == t.asha);
            assert(ckc_spec::v1text::hex64(asha.name@));
        }
    }
    if !hex64_exec(&asha.name) {
        return None;
    }
    if !guided_byte(bytes, guided, 0x29, at) {
        return None;
    }
    if !guided_byte(bytes, guided, 0x2c, at) {
        return None;
    }

    proof {
        reveal(traces_asha_stage);
        assert_seqs_equal!(
            guided.cursor.prefix@
                == old_prefix + traces_asha_stage(asha.name@)
        );
    }
    Some(asha)
}

#[verifier::rlimit(200)]
fn parse_traces_result(
    bytes: &[u8],
    guided: &mut EGuidedCursor,
    expected: Ghost<Option<ckc_spec::v1text::TracesFile>>,
at: &mut usize,
) -> (r: Option<ESpannedTerm>)
    requires
        *old(at) <= bytes@.len(),
        guided_cursor_ok(bytes@, old(guided)),
        expected@ matches Some(t) ==> old(guided).guide@ matches Some(g)
            && g.parts == traces_parts(t) && g.index == 19
            && ckc_spec::v1text::wf_traces(t),
        expected@ is None ==> old(guided).guide@ is None,
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r matches Some(result) ==> {
            &&& guided_cursor_ok(bytes@, final(guided))
            &&& spanned_term_ok(bytes@, &result)
            &&& result.parsed.ground
            &&& final(guided).cursor.prefix@
                == old(guided).cursor.prefix@ + traces_result_stage(result@)
        },
        expected@ matches Some(t) ==> {
            &&& r matches Some(result)
            &&& result@ == t.result
            &&& final(guided).guide@ matches Some(g)
                && g.parts == traces_parts(t) && g.index == 25
        },
        expected@ is None && r is Some ==> final(guided).guide@ is None,
{
    let ghost old_prefix = guided.cursor.prefix@;
    proof {
        if let Some(t) = expected@ {
            reveal(traces_parts);
            reveal(ckc_spec::v1text::wf_traces);
        }
    }

    let ghost wrapper_expected = match expected@ {
        Some(_) => Some((ckc_spec::v1text::ascii("result"@), 0x28u8)),
        None => None,
    };
    let wrapper = match guided_atom(bytes, guided, Ghost(wrapper_expected), at) {
        Some(atom) => atom,
        None => return None,
    };
    let wrapper_name: &[u8] = b"result";
    proof {
        reveal_strlit("result");
        reveal_byteslit(b"result");
        reveal(ckc_spec::v1text::ascii);
        assert(wrapper_name@
            == ckc_spec::v1text::ascii("result"@));
    }
    if !vec_slice_equal(&wrapper.name, wrapper_name) {
        return None;
    }
    if !guided_byte(bytes, guided, 0x28, at) {
        return None;
    }

    let ghost result_expected = match expected@ {
        Some(t) => Some((t.result, 0x29u8)),
        None => None,
    };
    let result = match guided_term(bytes, guided, Ghost(result_expected), at) {
        Some(term) => term,
        None => return None,
    };
    proof {
        if let Some(t) = expected@ {
            reveal(spanned_term_ok);
            reveal(parsed_term_ok);
            assert(result@ == t.result);
            assert(result.parsed.ground);
        }
    }
    if !result.parsed.ground {
        return None;
    }
    if !guided_byte(bytes, guided, 0x29, at) {
        return None;
    }
    if !guided_byte(bytes, guided, 0x29, at) {
        return None;
    }

    let line_end: &[u8] = b".\n";
    proof {
        reveal_strlit(".\n");
        reveal_byteslit(b".\n");
        reveal(ckc_spec::v1text::ascii);
        assert(line_end@ == ckc_spec::v1text::ascii(".\n"@));
    }
    if !guided_literal(
        bytes,
        guided,
        line_end,
        Ghost(ckc_spec::v1text::ascii(".\n"@)),
    at,
    ) {
        return None;
    }

    proof {
        reveal(traces_result_stage);
        assert_seqs_equal!(
            guided.cursor.prefix@
                == old_prefix + traces_result_stage(result@)
        );
    }
    Some(result)
}

#[verifier::rlimit(300)]
pub fn parse_traces(
    bytes: &[u8],
    expected: Ghost<Option<ckc_spec::v1text::TracesFile>>,
at: &mut usize,
) -> (r: Option<EParsedV1>)
    requires
        *old(at) <= bytes@.len(),
        expected@ matches Some(t) ==>
            ckc_spec::v1text::wf_traces(t)
                && ckc_spec::v1text::print_traces(t) == bytes@,
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r matches Some(parsed) ==> parsed_v1_ok(bytes@, &parsed),
        expected@ matches Some(t) ==> r matches Some(parsed)
            && parsed@ == ckc_spec::v1text::V1File::Traces(t),
{
    let ghost expected_parts = match expected@ {
        Some(t) => Some(traces_parts(t)),
        None => None,
    };
    proof {
        if let Some(t) = expected@ {
            traces_flat_is_print(t);
            traces_parts_flat(t);
            assert(bytes@ == traces_parts(t).flatten());
        }
    }
    let mut guided = new_guided_cursor(bytes, Ghost(expected_parts));

    let line_qid = match parse_traces_line(bytes, &mut guided, expected, at) {
        Some(qid) => qid,
        None => return None,
    };
    let qsha = match parse_traces_qsha(
        bytes,
        &mut guided,
        &line_qid,
        expected,
    at,
    ) {
        Some(hash) => hash,
        None => return None,
    };
    let asha = match parse_traces_asha(bytes, &mut guided, expected, at) {
        Some(hash) => hash,
        None => return None,
    };
    let result = match parse_traces_result(bytes, &mut guided, expected, at) {
        Some(term) => term,
        None => return None,
    };

    proof {
        if let Some(t) = expected@ {
            reveal(guided_cursor_ok);
            reveal(parts_progress);
            traces_parts(t).lemma_take_len();
            assert(guided.cursor.prefix@ == bytes@);
            assert(guided.cursor.prefix@.len() == guided.cursor.pos);
            assert(guided.cursor.pos == bytes.len());
        }
    }
    if guided.cursor.pos != bytes.len() {
        return None;
    }

    let ghost model = ckc_spec::v1text::TracesFile {
        qid: line_qid.value@,
        qsha: qsha.name@,
        asha: asha.name@,
        result: result@,
    };
    proof {
        assert(guided_cursor_ok(bytes@, &guided));
        reveal(guided_cursor_ok);
        assert(cursor_ok(bytes@, &guided.cursor));
        reveal(cursor_ok);
        assert(guided.cursor.pos == bytes@.len());
        assert(guided.cursor.prefix@
            == bytes@.subrange(0, bytes@.len() as int));
        assert_seqs_equal!(
            bytes@.subrange(0, bytes@.len() as int) == bytes@
        );
        assert(guided.cursor.prefix@ == bytes@);
        traces_stages_flat(model);
        assert_seqs_equal!(guided.cursor.prefix@ == traces_flat(model));
        traces_flat_is_print(model);
        if let Some(t) = expected@ {
            assert(line_qid.value@ == t.qid);
            assert(qsha.name@ == t.qsha);
            assert(asha.name@ == t.asha);
            assert(result@ == t.result);
            assert(model == t);
        }
        reveal(spanned_term_ok);
        reveal(parsed_term_ok);
        reveal(ckc_spec::v1text::wf_traces);
        reveal(ckc_spec::v1text::wf_v1);
        reveal(ckc_spec::v1text::print_v1);
        reveal(parsed_v1_ok);
    }
    Some(EParsedV1 {
        file: Ghost(ckc_spec::v1text::V1File::Traces(model)),
    })
}

pub ghost struct GTextExpected {
    pub value: Seq<u8>,
    pub end: usize,
}

pub struct ETextField {
    pub value: Vec<u8>,
    pub end: usize,
}

fn parse_raw_text(
    bytes: &[u8],
    start: usize,
    expected: Ghost<Option<GTextExpected>>,
at: &mut usize,
) -> (r: Option<ETextField>)
    requires
        *old(at) <= bytes@.len(),
        start < bytes@.len(),
        expected@ matches Some(e) ==> {
            &&& start < e.end < bytes@.len()
            &&& ckc_spec::v1text::text_ok(e.value)
            &&& e.value == bytes@.subrange(start as int, e.end as int)
            &&& bytes@[e.end as int] == 0x0a
        },
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r matches Some(field) ==> {
            &&& start < field.end < bytes@.len()
            &&& field.value@
                == bytes@.subrange(start as int, field.end as int)
            &&& ckc_spec::v1text::text_ok(field.value@)
            &&& bytes@[field.end as int] == 0x0a
        },
        expected@ matches Some(e) ==> r matches Some(field)
            && field.value@ == e.value && field.end == e.end,
{
    let mut pos = start;
    while pos < bytes.len() && bytes[pos] != 0x0a
        invariant
            *old(at) <= *at <= bytes@.len(),
            start <= pos <= bytes@.len(),
            forall|i: int| start <= i < pos ==> bytes@[i] != 0x0a,
            expected@ matches Some(e) ==> {
                &&& start < e.end < bytes@.len()
                &&& ckc_spec::v1text::text_ok(e.value)
                &&& e.value
                    == bytes@.subrange(start as int, e.end as int)
                &&& bytes@[e.end as int] == 0x0a
                &&& pos <= e.end
            },
        decreases bytes.len() - pos,
    {
        proof {
            if let Some(e) = expected@ {
                if pos == e.end {
                    assert(bytes@[pos as int] == 0x0a);
                    assert(false);
                }
            }
        }
        pos += 1;
    }
    proof {
        if let Some(e) = expected@ {
            if pos < e.end {
                reveal(ckc_spec::v1text::text_ok);
                reveal(ckc_spec::v1text::all_in);
                assert(bytes@[pos as int]
                    == e.value[pos as int - start as int]);
                assert(e.value[pos as int - start as int] != 0x0a);
                assert(false);
            }
            assert(pos == e.end);
        }
    }
    if pos == start || pos == bytes.len() {
        if pos == bytes.len() {
            raise_at(at, pos, bytes.len());
        } else {
            raise_at(at, start, bytes.len());
        }
        proof {
            if expected@ is Some {
                assert(false);
            }
        }
        return None;
    }
    let value = copy_range(bytes, start, pos);
    proof {
        reveal(ckc_spec::v1text::text_ok);
        reveal(ckc_spec::v1text::all_in);
        assert(value@.len() > 0);
        assert forall|i: int| 0 <= i < value@.len()
            implies value@[i] != 0x0a by {
            assert(value@[i] == bytes@[start as int + i]);
        }
    }
    Some(ETextField { value, end: pos })
}

fn cursor_text(
    bytes: &[u8],
    cursor: &mut EByteCursor,
    expected: Ghost<Option<GTextExpected>>,
at: &mut usize,
) -> (r: Option<ETextField>)
    requires
        *old(at) <= bytes@.len(),
        cursor_ok(bytes@, old(cursor)),
        expected@ matches Some(e) ==> {
            &&& old(cursor).pos < e.end < bytes@.len()
            &&& ckc_spec::v1text::text_ok(e.value)
            &&& e.value == bytes@.subrange(
                old(cursor).pos as int,
                e.end as int,
            )
            &&& bytes@[e.end as int] == 0x0a
        },
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r matches Some(field) ==> {
            &&& cursor_ok(bytes@, final(cursor))
            &&& final(cursor).pos == field.end
            &&& final(cursor).prefix@
                == old(cursor).prefix@ + field.value@
            &&& old(cursor).pos < field.end < bytes@.len()
            &&& field.value@ == bytes@.subrange(
                old(cursor).pos as int,
                field.end as int,
            )
            &&& ckc_spec::v1text::text_ok(field.value@)
            &&& bytes@[field.end as int] == 0x0a
        },
        expected@ matches Some(e) ==> r matches Some(field)
            && field.value@ == e.value && field.end == e.end,
{
    let start = cursor.pos;
    let ghost old_prefix = cursor.prefix@;
    if start == bytes.len() {
        raise_at(at, start, bytes.len());
        proof {
            if expected@ is Some {
                assert(false);
            }
        }
        return None;
    }
    let field = match parse_raw_text(bytes, start, expected, at) {
        Some(field) => field,
        None => return None,
    };
    proof {
        reveal(cursor_ok);
        range_concat(bytes@, 0, start as int, field.end as int);
        assert(bytes@.subrange(0, field.end as int)
            == old_prefix + field.value@);
    }
    cursor.pos = field.end;
    cursor.prefix = Ghost(old_prefix + field.value@);
    proof {
        reveal(cursor_ok);
    }
    Some(field)
}

proof fn text_part_expected(
    bytes: Seq<u8>,
    cursor: &EByteCursor,
    parts: Seq<Seq<u8>>,
    i: int,
    text: Seq<u8>,
    bound: usize,
) -> (g: GTextExpected)
    requires
        bytes.len() == bound,
        cursor_ok(bytes, cursor),
        cursor.prefix@ == parts.take(i).flatten(),
        bytes == parts.flatten(),
        0 <= i,
        i + 1 < parts.len(),
        parts[i] == text,
        ckc_spec::v1text::text_ok(text),
        parts[i + 1].len() > 0,
        parts[i + 1][0] == 0x0a,
    ensures
        cursor.pos < g.end < bytes.len(),
        ckc_spec::v1text::text_ok(g.value),
        g.value == bytes.subrange(cursor.pos as int, g.end as int),
        bytes[g.end as int] == 0x0a,
        g.value == text,
{
    cursor_part_ready(bytes, cursor, parts, i);
    flatten_take_step(parts, i);
    flattened_part(bytes, parts, i + 1);
    let end = cursor.pos as int + parts[i].len();
    assert(parts.take(i + 1).flatten().len() == end);
    assert(bytes[end] == 0x0a) by {
        assert(bytes.subrange(
            end,
            end + parts[i + 1].len(),
        )[0] == bytes[end]);
    }
    reveal(ckc_spec::v1text::text_ok);
    assert(parts[i].len() > 0);
    assert(cursor.pos < end);
    assert(end < bytes.len());
    GTextExpected { value: text, end: end as usize }
}

fn guided_text(
    bytes: &[u8],
    guided: &mut EGuidedCursor,
    expected: Ghost<Option<Seq<u8>>>,
at: &mut usize,
) -> (r: Option<ETextField>)
    requires
        *old(at) <= bytes@.len(),
        guided_cursor_ok(bytes@, old(guided)),
        old(guided).guide@ matches Some(g) ==> expected@ matches Some(text)
            && {
                &&& 0 <= g.index
                &&& g.index + 1 < g.parts.len()
                &&& g.parts[g.index] == text
                &&& ckc_spec::v1text::text_ok(text)
                &&& g.parts[g.index + 1].len() > 0
                &&& g.parts[g.index + 1][0] == 0x0a
            },
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r matches Some(field) ==> {
            &&& guided_cursor_ok(bytes@, final(guided))
            &&& ckc_spec::v1text::text_ok(field.value@)
            &&& final(guided).cursor.prefix@
                == old(guided).cursor.prefix@ + field.value@
        },
        old(guided).guide@ matches Some(g) ==> {
            &&& r matches Some(field)
            && expected@ matches Some(text) && field.value@ == text
            &&& final(guided).guide@ matches Some(next)
                && next.parts == g.parts && next.index == g.index + 1
        },
        old(guided).guide@ is None ==> final(guided).guide@ is None,
{
    let old_pos = guided.cursor.pos;
    let ghost old_prefix = guided.cursor.prefix@;
    let ghost old_guide = guided.guide@;
    proof {
        reveal(guided_cursor_ok);
        reveal(parts_progress);
        reveal(cursor_ok);
        assert(cursor_ok(bytes@, &guided.cursor));
    }
    let ghost text_expected = match (old_guide, expected@) {
        (Some(g), Some(text)) => {
            Some(text_part_expected(
                bytes@,
                &guided.cursor,
                g.parts,
                g.index,
                text,
                bytes.len(),
            ))
        },
        _ => None,
    };
    proof {
        reveal(guided_cursor_ok);
    }
    let field = match cursor_text(
        bytes,
        &mut guided.cursor,
        Ghost(text_expected),
    at,
    ) {
        Some(field) => field,
        None => {
            proof {
                if old_guide is Some {
                    assert(false);
                }
            }
            return None;
        },
    };
    let ghost next_guide = match old_guide {
        Some(g) => Some(GPartsGuide {
            parts: g.parts,
            index: g.index + 1,
        }),
        None => None,
    };
    proof {
        if let Some(g) = old_guide {
            advance_parts_progress(
                bytes@,
                g.parts,
                g.index,
                old_pos,
                old_prefix,
                &guided.cursor,
            );
        }
    }
    guided.guide = Ghost(next_guide);
    proof {
        reveal(guided_cursor_ok);
    }
    Some(field)
}

pub open spec fn query_line_stage(qid: Seq<u8>) -> Seq<u8> {
    ckc_spec::v1text::ascii("% "@)
        + qid
        + ckc_spec::v1text::ascii(
            " compiled from ACE question by ace_to_pl question mode; do not edit.\n"@,
        )
}

pub open spec fn query_record_head_stage(qid: Seq<u8>) -> Seq<u8> {
    ckc_spec::v1text::atom_bytes(
        ckc_spec::v1text::ascii("$guideline_query"@),
    )
        + seq![0x28u8]
        + ckc_spec::v1text::atom_bytes(ckc_spec::v1text::ascii("v1"@))
        + seq![0x2cu8]
        + ckc_spec::v1text::atom_bytes(qid)
        + seq![0x2cu8]
}

pub open spec fn query_record_ace_stage(ace: Seq<u8>) -> Seq<u8> {
    ckc_spec::v1text::atom_bytes(
        ckc_spec::v1text::ascii("ace_sha256"@),
    )
        + seq![0x28u8]
        + ckc_spec::v1text::atom_bytes(ace)
        + seq![0x29u8, 0x2cu8]
}

pub open spec fn query_record_ulex_open_stage() -> Seq<u8> {
    ckc_spec::v1text::atom_bytes(ckc_spec::v1text::ascii("ulex"@))
        + seq![0x28u8]
}

pub open spec fn query_record_prefix_stage(
    qid: Seq<u8>,
    ace: Seq<u8>,
) -> Seq<u8> {
    query_record_head_stage(qid)
        + query_record_ace_stage(ace)
        + query_record_ulex_open_stage()
}

pub open spec fn query_ulex_stage(ulex: Option<Seq<u8>>) -> Seq<u8> {
    match ulex {
        None => ckc_spec::v1text::atom_bytes(
            ckc_spec::v1text::ascii("none"@),
        ),
        Some(hash) => ckc_spec::v1text::atom_bytes(
            ckc_spec::v1text::ascii("sha256"@),
        ) + seq![0x28u8]
            + ckc_spec::v1text::atom_bytes(hash)
            + seq![0x29u8],
    }
}

pub open spec fn query_record_suffix_stage() -> Seq<u8> {
    seq![0x29u8, 0x29u8] + ckc_spec::v1text::ascii(".\n"@)
}

pub open spec fn record_ulex_parts(
    ulex: Option<Seq<u8>>,
) -> Seq<Seq<u8>> {
    match ulex {
        None => seq![
            ckc_spec::v1text::atom_bytes(ckc_spec::v1text::ascii("none"@)),
            seq![0x29u8],
            seq![0x29u8],
            ckc_spec::v1text::ascii(".\n"@),
        ],
        Some(hash) => seq![
            ckc_spec::v1text::atom_bytes(
                ckc_spec::v1text::ascii("sha256"@),
            ),
            seq![0x28u8],
            ckc_spec::v1text::atom_bytes(hash),
            seq![0x29u8],
            seq![0x29u8],
            seq![0x29u8],
            ckc_spec::v1text::ascii(".\n"@),
        ],
    }
}

proof fn record_ulex_parts_flat(ulex: Option<Seq<u8>>)
    ensures
        record_ulex_parts(ulex).flatten()
            == query_ulex_stage(ulex) + query_record_suffix_stage(),
{
    reveal(record_ulex_parts);
    reveal(query_ulex_stage);
    reveal(query_record_suffix_stage);
    match ulex {
        None => reveal_with_fuel(Seq::<_>::flatten, 6),
        Some(_) => reveal_with_fuel(Seq::<_>::flatten, 9),
    }
}

pub open spec fn query_record_stage(
    qid: Seq<u8>,
    ace: Seq<u8>,
    ulex: Option<Seq<u8>>,
) -> Seq<u8> {
    query_record_prefix_stage(qid, ace)
        + query_ulex_stage(ulex)
        + query_record_suffix_stage()
}

pub open spec fn query_text_stage(text: Seq<u8>) -> Seq<u8> {
    ckc_spec::v1text::ascii("% Q1: "@) + text + seq![0x0au8]
}

pub open spec fn query_projection_prefix_stage() -> Seq<u8> {
    ckc_spec::v1text::atom_bytes(
        ckc_spec::v1text::ascii("$guideline_query_projection"@),
    )
        + seq![0x28u8]
        + ckc_spec::v1text::atom_bytes(ckc_spec::v1text::ascii("goal"@))
        + seq![0x28u8]
}

pub open spec fn query_projection_middle_stage() -> Seq<u8> {
    seq![0x29u8, 0x2cu8]
        + ckc_spec::v1text::atom_bytes(ckc_spec::v1text::ascii("answers"@))
        + seq![0x28u8]
}

pub open spec fn query_projection_suffix_stage() -> Seq<u8> {
    seq![0x29u8, 0x29u8] + ckc_spec::v1text::ascii(".\n"@)
}

pub open spec fn query_projection_stage(
    goal: Term,
    answers: Term,
) -> Seq<u8> {
    query_projection_prefix_stage()
        + ckc_spec::v1text::term_bytes(goal)
        + query_projection_middle_stage()
        + ckc_spec::v1text::term_bytes(answers)
        + query_projection_suffix_stage()
}

pub open spec fn query_flat(q: ckc_spec::v1text::QueryFile) -> Seq<u8> {
    query_line_stage(q.qid)
        + query_record_stage(q.qid, q.ace, q.ulex)
        + query_text_stage(q.qtext)
        + query_projection_stage(q.goal, q.answers)
}

pub open spec fn query_line_parts(qid: Seq<u8>) -> Seq<Seq<u8>> {
    seq![
        ckc_spec::v1text::ascii("% "@),
        qid,
        ckc_spec::v1text::ascii(
            " compiled from ACE question by ace_to_pl question mode; do not edit.\n"@,
        ),
    ]
}

pub open spec fn query_record_parts(
    qid: Seq<u8>,
    ace: Seq<u8>,
    ulex: Option<Seq<u8>>,
) -> Seq<Seq<u8>> {
    match ulex {
        None => seq![
            ckc_spec::v1text::atom_bytes(
                ckc_spec::v1text::ascii("$guideline_query"@),
            ),
            seq![0x28u8],
            ckc_spec::v1text::atom_bytes(ckc_spec::v1text::ascii("v1"@)),
            seq![0x2cu8],
            ckc_spec::v1text::atom_bytes(qid),
            seq![0x2cu8],
            ckc_spec::v1text::atom_bytes(
                ckc_spec::v1text::ascii("ace_sha256"@),
            ),
            seq![0x28u8],
            ckc_spec::v1text::atom_bytes(ace),
            seq![0x29u8],
            seq![0x2cu8],
            ckc_spec::v1text::atom_bytes(
                ckc_spec::v1text::ascii("ulex"@),
            ),
            seq![0x28u8],
            ckc_spec::v1text::atom_bytes(ckc_spec::v1text::ascii("none"@)),
            seq![0x29u8],
            seq![0x29u8],
            ckc_spec::v1text::ascii(".\n"@),
        ],
        Some(hash) => seq![
            ckc_spec::v1text::atom_bytes(
                ckc_spec::v1text::ascii("$guideline_query"@),
            ),
            seq![0x28u8],
            ckc_spec::v1text::atom_bytes(ckc_spec::v1text::ascii("v1"@)),
            seq![0x2cu8],
            ckc_spec::v1text::atom_bytes(qid),
            seq![0x2cu8],
            ckc_spec::v1text::atom_bytes(
                ckc_spec::v1text::ascii("ace_sha256"@),
            ),
            seq![0x28u8],
            ckc_spec::v1text::atom_bytes(ace),
            seq![0x29u8],
            seq![0x2cu8],
            ckc_spec::v1text::atom_bytes(
                ckc_spec::v1text::ascii("ulex"@),
            ),
            seq![0x28u8],
            ckc_spec::v1text::atom_bytes(
                ckc_spec::v1text::ascii("sha256"@),
            ),
            seq![0x28u8],
            ckc_spec::v1text::atom_bytes(hash),
            seq![0x29u8],
            seq![0x29u8],
            seq![0x29u8],
            ckc_spec::v1text::ascii(".\n"@),
        ],
    }
}

pub open spec fn query_text_parts(text: Seq<u8>) -> Seq<Seq<u8>> {
    seq![
        ckc_spec::v1text::ascii("% Q1: "@),
        text,
        seq![0x0au8],
    ]
}

pub open spec fn query_projection_parts(
    goal: Term,
    answers: Term,
) -> Seq<Seq<u8>> {
    seq![
        ckc_spec::v1text::atom_bytes(
            ckc_spec::v1text::ascii("$guideline_query_projection"@),
        ),
        seq![0x28u8],
        ckc_spec::v1text::atom_bytes(ckc_spec::v1text::ascii("goal"@)),
        seq![0x28u8],
        ckc_spec::v1text::term_bytes(goal),
        seq![0x29u8],
        seq![0x2cu8],
        ckc_spec::v1text::atom_bytes(
            ckc_spec::v1text::ascii("answers"@),
        ),
        seq![0x28u8],
        ckc_spec::v1text::term_bytes(answers),
        seq![0x29u8],
        seq![0x29u8],
        ckc_spec::v1text::ascii(".\n"@),
    ]
}

pub open spec fn query_parts(
    q: ckc_spec::v1text::QueryFile,
) -> Seq<Seq<u8>> {
    query_line_parts(q.qid)
        + query_record_parts(q.qid, q.ace, q.ulex)
        + query_text_parts(q.qtext)
        + query_projection_parts(q.goal, q.answers)
}

pub open spec fn query_record_end(ulex: Option<Seq<u8>>) -> int {
    match ulex {
        None => 20,
        Some(_) => 23,
    }
}

pub open spec fn query_text_end(ulex: Option<Seq<u8>>) -> int {
    query_record_end(ulex) + 3
}

pub open spec fn query_parts_end(ulex: Option<Seq<u8>>) -> int {
    query_text_end(ulex) + 13
}

proof fn query_line_parts_flat(qid: Seq<u8>)
    ensures query_line_parts(qid).flatten() == query_line_stage(qid),
{
    reveal(query_line_parts);
    reveal(query_line_stage);
    reveal_with_fuel(Seq::<_>::flatten, 5);
}

#[verifier::rlimit(500)]
proof fn query_record_parts_flat(
    qid: Seq<u8>,
    ace: Seq<u8>,
    ulex: Option<Seq<u8>>,
)
    ensures query_record_parts(qid, ace, ulex).flatten()
        == query_record_stage(qid, ace, ulex),
{
    let head = seq![
        ckc_spec::v1text::atom_bytes(
            ckc_spec::v1text::ascii("$guideline_query"@),
        ),
        seq![0x28u8],
        ckc_spec::v1text::atom_bytes(ckc_spec::v1text::ascii("v1"@)),
        seq![0x2cu8],
        ckc_spec::v1text::atom_bytes(qid),
        seq![0x2cu8],
    ];
    let ace_parts = seq![
        ckc_spec::v1text::atom_bytes(
            ckc_spec::v1text::ascii("ace_sha256"@),
        ),
        seq![0x28u8],
        ckc_spec::v1text::atom_bytes(ace),
        seq![0x29u8],
        seq![0x2cu8],
    ];
    let ulex_open = seq![
        ckc_spec::v1text::atom_bytes(ckc_spec::v1text::ascii("ulex"@)),
        seq![0x28u8],
    ];
    let ulex_parts = match ulex {
        None => seq![ckc_spec::v1text::atom_bytes(
            ckc_spec::v1text::ascii("none"@),
        )],
        Some(hash) => seq![
            ckc_spec::v1text::atom_bytes(
                ckc_spec::v1text::ascii("sha256"@),
            ),
            seq![0x28u8],
            ckc_spec::v1text::atom_bytes(hash),
            seq![0x29u8],
        ],
    };
    let suffix = seq![
        seq![0x29u8],
        seq![0x29u8],
        ckc_spec::v1text::ascii(".\n"@),
    ];
    assert(query_record_parts(qid, ace, ulex)
        == ((head + ace_parts) + (ulex_open + ulex_parts)) + suffix) by {
        reveal(query_record_parts);
    }
    assert(head.flatten() == query_record_head_stage(qid)) by {
        reveal(query_record_head_stage);
        reveal_with_fuel(Seq::<_>::flatten, 8);
    }
    assert(ace_parts.flatten() == query_record_ace_stage(ace)) by {
        reveal(query_record_ace_stage);
        reveal_with_fuel(Seq::<_>::flatten, 7);
    }
    assert(ulex_open.flatten() == query_record_ulex_open_stage()) by {
        reveal(query_record_ulex_open_stage);
        reveal_with_fuel(Seq::<_>::flatten, 4);
    }
    assert(ulex_parts.flatten() == query_ulex_stage(ulex)) by {
        reveal(query_ulex_stage);
        match ulex {
            None => reveal_with_fuel(Seq::<_>::flatten, 3),
            Some(_) => reveal_with_fuel(Seq::<_>::flatten, 6),
        }
    }
    assert(suffix.flatten() == query_record_suffix_stage()) by {
        reveal(query_record_suffix_stage);
        reveal_with_fuel(Seq::<_>::flatten, 5);
    }
    vstd::seq_lib::lemma_flatten_concat(head, ace_parts);
    vstd::seq_lib::lemma_flatten_concat(ulex_open, ulex_parts);
    vstd::seq_lib::lemma_flatten_concat(
        head + ace_parts,
        ulex_open + ulex_parts,
    );
    vstd::seq_lib::lemma_flatten_concat(
        (head + ace_parts) + (ulex_open + ulex_parts),
        suffix,
    );
    reveal(query_record_prefix_stage);
    reveal(query_record_stage);
}

proof fn query_text_parts_flat(text: Seq<u8>)
    ensures query_text_parts(text).flatten() == query_text_stage(text),
{
    reveal(query_text_parts);
    reveal(query_text_stage);
    reveal_with_fuel(Seq::<_>::flatten, 5);
}

proof fn query_projection_parts_flat(goal: Term, answers: Term)
    ensures query_projection_parts(goal, answers).flatten()
        == query_projection_stage(goal, answers),
{
    let prefix = seq![
        ckc_spec::v1text::atom_bytes(
            ckc_spec::v1text::ascii("$guideline_query_projection"@),
        ),
        seq![0x28u8],
        ckc_spec::v1text::atom_bytes(ckc_spec::v1text::ascii("goal"@)),
        seq![0x28u8],
    ];
    let goal_part = seq![ckc_spec::v1text::term_bytes(goal)];
    let middle = seq![
        seq![0x29u8],
        seq![0x2cu8],
        ckc_spec::v1text::atom_bytes(
            ckc_spec::v1text::ascii("answers"@),
        ),
        seq![0x28u8],
    ];
    let answers_part = seq![ckc_spec::v1text::term_bytes(answers)];
    let suffix = seq![
        seq![0x29u8],
        seq![0x29u8],
        ckc_spec::v1text::ascii(".\n"@),
    ];
    assert(query_projection_parts(goal, answers)
        == ((prefix + goal_part) + (middle + answers_part)) + suffix) by {
        reveal(query_projection_parts);
    }
    assert(prefix.flatten() == query_projection_prefix_stage()) by {
        reveal(query_projection_prefix_stage);
        reveal_with_fuel(Seq::<_>::flatten, 6);
    }
    assert(goal_part.flatten() == ckc_spec::v1text::term_bytes(goal)) by {
        reveal_with_fuel(Seq::<_>::flatten, 3);
    }
    assert(middle.flatten() == query_projection_middle_stage()) by {
        reveal(query_projection_middle_stage);
        reveal_with_fuel(Seq::<_>::flatten, 6);
    }
    assert(answers_part.flatten()
        == ckc_spec::v1text::term_bytes(answers)) by {
        reveal_with_fuel(Seq::<_>::flatten, 3);
    }
    assert(suffix.flatten() == query_projection_suffix_stage()) by {
        reveal(query_projection_suffix_stage);
        reveal_with_fuel(Seq::<_>::flatten, 5);
    }
    vstd::seq_lib::lemma_flatten_concat(prefix, goal_part);
    vstd::seq_lib::lemma_flatten_concat(middle, answers_part);
    vstd::seq_lib::lemma_flatten_concat(
        prefix + goal_part,
        middle + answers_part,
    );
    vstd::seq_lib::lemma_flatten_concat(
        (prefix + goal_part) + (middle + answers_part),
        suffix,
    );
    reveal(query_projection_stage);
}

proof fn query_parts_flat(q: ckc_spec::v1text::QueryFile)
    ensures query_parts(q).flatten() == query_flat(q),
{
    let line = query_line_parts(q.qid);
    let record = query_record_parts(q.qid, q.ace, q.ulex);
    let text = query_text_parts(q.qtext);
    let projection = query_projection_parts(q.goal, q.answers);
    assert(query_parts(q) == (line + record) + (text + projection)) by {
        reveal(query_parts);
    }
    query_line_parts_flat(q.qid);
    query_record_parts_flat(q.qid, q.ace, q.ulex);
    query_text_parts_flat(q.qtext);
    query_projection_parts_flat(q.goal, q.answers);
    vstd::seq_lib::lemma_flatten_concat(line, record);
    vstd::seq_lib::lemma_flatten_concat(text, projection);
    vstd::seq_lib::lemma_flatten_concat(line + record, text + projection);
    reveal(query_flat);
}

proof fn args_two_bytes(a: Term, b: Term)
    ensures
        ckc_spec::v1text::args_bytes(seq![a, b])
            == ckc_spec::v1text::term_bytes(a)
                + seq![0x2cu8]
                + ckc_spec::v1text::term_bytes(b),
{
    reveal_with_fuel(ckc_spec::v1text::args_bytes, 3);
}

#[verifier::rlimit(5000)]
#[verifier::spinoff_prover]
proof fn query_flat_is_print(q: ckc_spec::v1text::QueryFile)
    ensures query_flat(q) == ckc_spec::v1text::print_query(q),
{
    let v1_name = ckc_spec::v1text::ascii("v1"@);
    let ace_name = ckc_spec::v1text::ascii("ace_sha256"@);
    let ulex_name = ckc_spec::v1text::ascii("ulex"@);
    let sha_name = ckc_spec::v1text::ascii("sha256"@);
    let goal_name = ckc_spec::v1text::ascii("goal"@);
    let answers_name = ckc_spec::v1text::ascii("answers"@);
    let query_name = ckc_spec::v1text::ascii("$guideline_query"@);
    let projection_name = ckc_spec::v1text::ascii(
        "$guideline_query_projection"@,
    );
    let v1 = Term::Atom(v1_name);
    let qid = Term::Atom(q.qid);
    let ace_hash = Term::Atom(q.ace);
    let ace = Term::Comp(ace_name, seq![ace_hash]);
    let ulex_value = ckc_spec::v1text::ulex_term(q.ulex);
    let ulex = Term::Comp(ulex_name, seq![ulex_value]);
    let goal = Term::Comp(goal_name, seq![q.goal]);
    let answers = Term::Comp(answers_name, seq![q.answers]);
    reveal_strlit("v1");
    reveal_strlit("ace_sha256");
    reveal_strlit("ulex");
    reveal_strlit("sha256");
    reveal_strlit("goal");
    reveal_strlit("answers");
    reveal_strlit("$guideline_query");
    reveal_strlit("$guideline_query_projection");
    reveal(ckc_spec::v1text::ascii);
    reveal(ckc_spec::v1text::curly_name);
    assert(ace_name != ckc_spec::v1text::curly_name());
    assert(ulex_name != ckc_spec::v1text::curly_name());
    assert(sha_name != ckc_spec::v1text::curly_name());
    assert(goal_name != ckc_spec::v1text::curly_name());
    assert(answers_name != ckc_spec::v1text::curly_name());
    assert(query_name != ckc_spec::v1text::curly_name());
    assert(projection_name != ckc_spec::v1text::curly_name());
    atom_term_bytes(v1_name);
    atom_term_bytes(q.qid);
    atom_term_bytes(q.ace);
    args_one_bytes(ace_hash);
    regular_comp_bytes(ace_name, seq![ace_hash]);
    match q.ulex {
        None => {
            let none_name = ckc_spec::v1text::ascii("none"@);
            reveal_strlit("none");
            reveal(ckc_spec::v1text::ascii);
            atom_term_bytes(none_name);
        },
        Some(hash) => {
            let hash_term = Term::Atom(hash);
            atom_term_bytes(hash);
            args_one_bytes(hash_term);
            regular_comp_bytes(sha_name, seq![hash_term]);
        },
    }
    args_one_bytes(ulex_value);
    regular_comp_bytes(ulex_name, seq![ulex_value]);
    args_four_bytes(v1, qid, ace, ulex);
    regular_comp_bytes(query_name, seq![v1, qid, ace, ulex]);
    args_one_bytes(q.goal);
    regular_comp_bytes(goal_name, seq![q.goal]);
    args_one_bytes(q.answers);
    regular_comp_bytes(answers_name, seq![q.answers]);
    args_two_bytes(goal, answers);
    regular_comp_bytes(projection_name, seq![goal, answers]);
    reveal(query_flat);
    reveal(query_line_stage);
    reveal(query_record_stage);
    reveal(query_record_prefix_stage);
    reveal(query_record_head_stage);
    reveal(query_record_ace_stage);
    reveal(query_record_ulex_open_stage);
    reveal(query_ulex_stage);
    reveal(query_record_suffix_stage);
    reveal(query_text_stage);
    reveal(query_projection_stage);
    reveal(query_projection_prefix_stage);
    reveal(query_projection_middle_stage);
    reveal(query_projection_suffix_stage);
    reveal(ckc_spec::v1text::print_query);
    reveal(ckc_spec::v1text::query_line1);
    reveal(ckc_spec::v1text::term_line);
    reveal(ckc_spec::v1text::query_record_term);
    reveal(ckc_spec::v1text::projection_term);
    reveal(ckc_spec::v1text::ulex_term);
}

pub struct EUlexField {
    pub value: Ghost<Option<Seq<u8>>>,
}

#[verifier::rlimit(200)]
fn parse_query_line(
    bytes: &[u8],
    guided: &mut EGuidedCursor,
    expected: Ghost<Option<ckc_spec::v1text::QueryFile>>,
at: &mut usize,
) -> (r: Option<ENameField>)
    requires
        *old(at) <= bytes@.len(),
        guided_cursor_ok(bytes@, old(guided)),
        expected@ matches Some(q) ==> old(guided).guide@ matches Some(g)
            && g.parts == query_parts(q) && g.index == 0
            && ckc_spec::v1text::wf_query(q),
        expected@ is None ==> old(guided).guide@ is None,
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r matches Some(qid) ==> {
            &&& guided_cursor_ok(bytes@, final(guided))
            &&& ckc_spec::v1text::name_ok(qid.value@)
            &&& final(guided).cursor.prefix@
                == old(guided).cursor.prefix@ + query_line_stage(qid.value@)
        },
        expected@ matches Some(q) ==> {
            &&& r matches Some(qid)
            &&& qid.value@ == q.qid
            &&& final(guided).guide@ matches Some(g)
                && g.parts == query_parts(q) && g.index == 3
        },
        expected@ is None && r is Some ==> final(guided).guide@ is None,
{
    let ghost old_prefix = guided.cursor.prefix@;
    proof {
        if let Some(q) = expected@ {
            reveal(query_parts);
            reveal(query_line_parts);
            reveal(ckc_spec::v1text::wf_query);
            reveal_strlit(
                " compiled from ACE question by ace_to_pl question mode; do not edit.\n",
            );
            reveal(ckc_spec::v1text::ascii);
        }
    }

    let percent_space: &[u8] = b"% ";
    proof {
        reveal_strlit("% ");
        reveal_byteslit(b"% ");
        reveal(ckc_spec::v1text::ascii);
        assert(percent_space@ == ckc_spec::v1text::ascii("% "@));
    }
    if !guided_literal(
        bytes,
        guided,
        percent_space,
        Ghost(ckc_spec::v1text::ascii("% "@)),
    at,
    ) {
        return None;
    }

    let ghost qid_expected = match expected@ {
        Some(q) => Some(q.qid),
        None => None,
    };
    let qid = match guided_name(bytes, guided, Ghost(0x20u8), Ghost(qid_expected), at) {
        Some(field) => field,
        None => return None,
    };

    let line_suffix: &[u8] = b" compiled from ACE question by ace_to_pl question mode; do not edit.\n";
    proof {
        reveal_strlit(
            " compiled from ACE question by ace_to_pl question mode; do not edit.\n",
        );
        reveal_byteslit(
            b" compiled from ACE question by ace_to_pl question mode; do not edit.\n",
        );
        reveal(ckc_spec::v1text::ascii);
        assert(line_suffix@ == ckc_spec::v1text::ascii(
            " compiled from ACE question by ace_to_pl question mode; do not edit.\n"@,
        ));
    }
    if !guided_literal(
        bytes,
        guided,
        line_suffix,
        Ghost(ckc_spec::v1text::ascii(
            " compiled from ACE question by ace_to_pl question mode; do not edit.\n"@,
        )),
    at,
    ) {
        return None;
    }
    proof {
        reveal(query_line_stage);
        assert_seqs_equal!(
            guided.cursor.prefix@ == old_prefix + query_line_stage(qid.value@)
        );
    }
    Some(qid)
}

#[verifier::rlimit(500)]
fn parse_query_record_prefix(
    bytes: &[u8],
    guided: &mut EGuidedCursor,
    line_qid: &ENameField,
    expected: Ghost<Option<ckc_spec::v1text::QueryFile>>,
at: &mut usize,
) -> (r: Option<EParsedAtom>)
    requires
        *old(at) <= bytes@.len(),
        guided_cursor_ok(bytes@, old(guided)),
        ckc_spec::v1text::name_ok(line_qid.value@),
        expected@ matches Some(q) ==> {
            &&& line_qid.value@ == q.qid
            &&& old(guided).guide@ matches Some(g)
                && g.parts == query_parts(q) && g.index == 3
            &&& ckc_spec::v1text::wf_query(q)
        },
        expected@ is None ==> old(guided).guide@ is None,
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r matches Some(ace) ==> {
            &&& guided_cursor_ok(bytes@, final(guided))
            &&& ckc_spec::v1text::hex64(ace.name@)
            &&& final(guided).cursor.prefix@
                == old(guided).cursor.prefix@
                    + query_record_prefix_stage(line_qid.value@, ace.name@)
        },
        expected@ matches Some(q) ==> {
            &&& r matches Some(ace)
            &&& ace.name@ == q.ace
            &&& final(guided).guide@ matches Some(g)
                && g.parts == query_parts(q) && g.index == 16
        },
        expected@ is None && r is Some ==> final(guided).guide@ is None,
{
    let ghost old_prefix = guided.cursor.prefix@;
    proof {
        if let Some(q) = expected@ {
            reveal(query_parts);
            reveal(query_line_parts);
            reveal(query_record_parts);
            reveal(ckc_spec::v1text::wf_query);
            match q.ulex {
                None => {},
                Some(_) => {},
            }
        }
    }

    let ghost wrapper_expected = match expected@ {
        Some(_) => Some((
            ckc_spec::v1text::ascii("$guideline_query"@),
            0x28u8,
        )),
        None => None,
    };
    let wrapper = match guided_atom(bytes, guided, Ghost(wrapper_expected), at) {
        Some(atom) => atom,
        None => return None,
    };
    let wrapper_name: &[u8] = b"$guideline_query";
    proof {
        reveal_strlit("$guideline_query");
        reveal_byteslit(b"$guideline_query");
        reveal(ckc_spec::v1text::ascii);
        assert(wrapper_name@
            == ckc_spec::v1text::ascii("$guideline_query"@));
    }
    if !vec_slice_equal(&wrapper.name, wrapper_name) {
        return None;
    }
    if !guided_byte(bytes, guided, 0x28, at) {
        return None;
    }

    let ghost version_expected = match expected@ {
        Some(_) => Some((ckc_spec::v1text::ascii("v1"@), 0x2cu8)),
        None => None,
    };
    let version = match guided_atom(bytes, guided, Ghost(version_expected), at) {
        Some(atom) => atom,
        None => return None,
    };
    let version_name: &[u8] = b"v1";
    proof {
        reveal_strlit("v1");
        reveal_byteslit(b"v1");
        reveal(ckc_spec::v1text::ascii);
        assert(version_name@ == ckc_spec::v1text::ascii("v1"@));
    }
    if !vec_slice_equal(&version.name, version_name) {
        return None;
    }
    if !guided_byte(bytes, guided, 0x2c, at) {
        return None;
    }

    let ghost record_qid_expected = match expected@ {
        Some(q) => Some((q.qid, 0x2cu8)),
        None => None,
    };
    let record_qid = match guided_atom(
        bytes,
        guided,
        Ghost(record_qid_expected),
    at,
    ) {
        Some(atom) => atom,
        None => return None,
    };
    if !vec_equal(&line_qid.value, &record_qid.name) {
        return None;
    }
    if !guided_byte(bytes, guided, 0x2c, at) {
        return None;
    }

    let ghost ace_wrapper_expected = match expected@ {
        Some(_) => Some((
            ckc_spec::v1text::ascii("ace_sha256"@),
            0x28u8,
        )),
        None => None,
    };
    let ace_wrapper = match guided_atom(
        bytes,
        guided,
        Ghost(ace_wrapper_expected),
    at,
    ) {
        Some(atom) => atom,
        None => return None,
    };
    let ace_wrapper_name: &[u8] = b"ace_sha256";
    proof {
        reveal_strlit("ace_sha256");
        reveal_byteslit(b"ace_sha256");
        reveal(ckc_spec::v1text::ascii);
        assert(ace_wrapper_name@
            == ckc_spec::v1text::ascii("ace_sha256"@));
    }
    if !vec_slice_equal(&ace_wrapper.name, ace_wrapper_name) {
        return None;
    }
    if !guided_byte(bytes, guided, 0x28, at) {
        return None;
    }

    let ghost ace_expected = match expected@ {
        Some(q) => Some((q.ace, 0x29u8)),
        None => None,
    };
    let ace = match guided_atom(bytes, guided, Ghost(ace_expected), at) {
        Some(atom) => atom,
        None => return None,
    };
    proof {
        if let Some(q) = expected@ {
            assert(ace.name@ == q.ace);
            assert(ckc_spec::v1text::hex64(ace.name@));
        }
    }
    if !hex64_exec(&ace.name) {
        return None;
    }
    if !guided_byte(bytes, guided, 0x29, at) {
        return None;
    }
    if !guided_byte(bytes, guided, 0x2c, at) {
        return None;
    }

    let ghost ulex_wrapper_expected = match expected@ {
        Some(_) => Some((ckc_spec::v1text::ascii("ulex"@), 0x28u8)),
        None => None,
    };
    let ulex_wrapper = match guided_atom(
        bytes,
        guided,
        Ghost(ulex_wrapper_expected),
    at,
    ) {
        Some(atom) => atom,
        None => return None,
    };
    let ulex_wrapper_name: &[u8] = b"ulex";
    proof {
        reveal_strlit("ulex");
        reveal_byteslit(b"ulex");
        reveal(ckc_spec::v1text::ascii);
        assert(ulex_wrapper_name@
            == ckc_spec::v1text::ascii("ulex"@));
    }
    if !vec_slice_equal(&ulex_wrapper.name, ulex_wrapper_name) {
        return None;
    }
    if !guided_byte(bytes, guided, 0x28, at) {
        return None;
    }

    proof {
        reveal(query_record_prefix_stage);
        reveal(query_record_head_stage);
        reveal(query_record_ace_stage);
        reveal(query_record_ulex_open_stage);
        assert_seqs_equal!(
            guided.cursor.prefix@
                == old_prefix
                    + query_record_prefix_stage(line_qid.value@, ace.name@)
        );
    }
    Some(ace)
}

#[verifier::rlimit(2000)]
#[verifier::spinoff_prover]
fn parse_record_ulex(
    bytes: &[u8],
    guided: &mut EGuidedCursor,
    expected: Ghost<Option<(Option<Seq<u8>>, Seq<Seq<u8>>)>>,
at: &mut usize,
) -> (r: Option<EUlexField>)
    requires
        *old(at) <= bytes@.len(),
        guided_cursor_ok(bytes@, old(guided)),
        expected@ matches Some(e) ==> {
            &&& old(guided).guide@ matches Some(g)
                && guide_rest(g) == record_ulex_parts(e.0) + e.1
            &&& ckc_spec::v1text::ulex_ok(e.0)
        },
        expected@ is None ==> old(guided).guide@ is None,
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r matches Some(ulex) ==> {
            &&& guided_cursor_ok(bytes@, final(guided))
            &&& ckc_spec::v1text::ulex_ok(ulex.value@)
            &&& final(guided).cursor.prefix@
                == old(guided).cursor.prefix@
                    + query_ulex_stage(ulex.value@)
                    + query_record_suffix_stage()
        },
        expected@ matches Some(e) ==> {
            &&& r matches Some(ulex) && ulex.value@ == e.0
            &&& old(guided).guide@ matches Some(before)
            &&& final(guided).guide@ matches Some(after)
                && after.parts == before.parts
                && after.index
                    == before.index + record_ulex_parts(e.0).len()
                && guide_rest(after) == e.1
        },
        expected@ is None ==> final(guided).guide@ is None,
{
    let ghost entry_prefix = guided.cursor.prefix@;
    let ghost old_guide = guided.guide@;
    proof {
        if let Some(e) = expected@ {
            reveal(record_ulex_parts);
            reveal(ckc_spec::v1text::ulex_ok);
            match e.0 {
                None => {},
                Some(_) => {},
            }
        }
    }

    let ghost tag_expected = match expected@ {
        Some(e) => match e.0 {
            None => Some((
                ckc_spec::v1text::ascii("none"@),
                seq![
                    seq![0x29u8],
                    seq![0x29u8],
                    ckc_spec::v1text::ascii(".\n"@),
                ] + e.1,
            )),
            Some(hash) => Some((
                ckc_spec::v1text::ascii("sha256"@),
                seq![
                    seq![0x28u8],
                    ckc_spec::v1text::atom_bytes(hash),
                    seq![0x29u8],
                    seq![0x29u8],
                    seq![0x29u8],
                    ckc_spec::v1text::ascii(".\n"@),
                ] + e.1,
            )),
        },
        None => None,
    };
    let ghost tag_next = match expected@ {
        Some(e) => if e.0 is None { 0x29u8 } else { 0x28u8 },
        None => 0x29u8,
    };
    proof {
        if let Some(e) = expected@ {
            match e.0 {
                None => {
                    assert(tag_expected.unwrap().0
                        == ckc_spec::v1text::ascii("none"@));
                    assert(tag_next == 0x29);
                },
                Some(_) => {
                    assert(tag_expected.unwrap().0
                        == ckc_spec::v1text::ascii("sha256"@));
                    assert(tag_next == 0x28);
                },
            }
            assert_seqs_equal!(guide_rest(guided.guide@.unwrap())
                == seq![ckc_spec::v1text::atom_bytes(
                    tag_expected.unwrap().0,
                )] + tag_expected.unwrap().1);
            assert(tag_expected.unwrap().1.len() > 0);
            assert(tag_expected.unwrap().1[0].len() > 0);
            assert(tag_expected.unwrap().1[0][0] == tag_next);
        }
    }
    let tag = match parts_guided_atom(
        bytes,
        guided,
        Ghost(tag_next),
        Ghost(tag_expected),
    at,
    ) {
        Some(atom) => atom,
        None => return None,
    };
    let none_name: &[u8] = b"none";
    let sha_name: &[u8] = b"sha256";
    proof {
        reveal_strlit("none");
        reveal_byteslit(b"none");
        reveal_strlit("sha256");
        reveal_byteslit(b"sha256");
        reveal(ckc_spec::v1text::ascii);
        assert(none_name@ == ckc_spec::v1text::ascii("none"@));
        assert(sha_name@ == ckc_spec::v1text::ascii("sha256"@));
    }

    if vec_slice_equal(&tag.name, none_name) {
        proof {
            if let Some(e) = expected@ {
                match e.0 {
                    None => {},
                    Some(_) => assert(false),
                }
                assert(guide_rest(guided.guide@.unwrap())
                    == seq![
                        seq![0x29u8],
                        seq![0x29u8],
                        ckc_spec::v1text::ascii(".\n"@),
                    ] + e.1);
            }
        }
        let close: &[u8] = b")";
        let ghost close_chunk = seq![0x29u8];
        proof {
            reveal_byteslit(b")");
            assert(close@ == close_chunk);
        }
        let ghost after_first_close = match expected@ {
            Some(e) => Some(
                seq![seq![0x29u8], ckc_spec::v1text::ascii(".\n"@)] + e.1,
            ),
            None => None,
        };
        proof {
            if expected@ is Some {
                assert_seqs_equal!(guide_rest(guided.guide@.unwrap())
                    == seq![close_chunk] + after_first_close.unwrap());
            }
        }
        if !doc_guided_literal(
            bytes,
            guided,
            close,
            Ghost(close_chunk),
            Ghost(after_first_close),
        at,
        ) {
            return None;
        }
        let ghost after_second_close = match expected@ {
            Some(e) => Some(seq![ckc_spec::v1text::ascii(".\n"@)] + e.1),
            None => None,
        };
        proof {
            if expected@ is Some {
                assert_seqs_equal!(guide_rest(guided.guide@.unwrap())
                    == seq![close_chunk] + after_second_close.unwrap());
            }
        }
        if !doc_guided_literal(
            bytes,
            guided,
            close,
            Ghost(close_chunk),
            Ghost(after_second_close),
        at,
        ) {
            return None;
        }
        let line_end: &[u8] = b".\n";
        proof {
            reveal_strlit(".\n");
            reveal_byteslit(b".\n");
            reveal(ckc_spec::v1text::ascii);
            assert(line_end@ == ckc_spec::v1text::ascii(".\n"@));
        }
        let ghost after_line = match expected@ {
            Some(e) => Some(e.1),
            None => None,
        };
        proof {
            if expected@ is Some {
                assert_seqs_equal!(guide_rest(guided.guide@.unwrap())
                    == seq![ckc_spec::v1text::ascii(".\n"@)]
                        + after_line.unwrap());
            }
        }
        if !doc_guided_literal(
            bytes,
            guided,
            line_end,
            Ghost(ckc_spec::v1text::ascii(".\n"@)),
            Ghost(after_line),
        at,
        ) {
            return None;
        }
        proof {
            if let Some(e) = expected@ {
                assert(e.0 is None);
                reveal(record_ulex_parts);
                assert(record_ulex_parts(e.0).len() == 4);
            }
            record_ulex_parts_flat(None);
            assert_seqs_equal!(guided.cursor.prefix@
                == entry_prefix
                    + query_ulex_stage(None)
                    + query_record_suffix_stage());
            reveal(ckc_spec::v1text::ulex_ok);
        }
        return Some(EUlexField { value: Ghost(None) });
    }

    if !vec_slice_equal(&tag.name, sha_name) {
        proof {
            if expected@ is Some {
                assert(false);
            }
        }
        return None;
    }
    proof {
        if let Some(e) = expected@ {
            match e.0 {
                Some(_) => {},
                None => assert(false),
            }
            assert(guide_rest(guided.guide@.unwrap())
                == seq![
                    seq![0x28u8],
                    ckc_spec::v1text::atom_bytes(e.0.unwrap()),
                    seq![0x29u8],
                    seq![0x29u8],
                    seq![0x29u8],
                    ckc_spec::v1text::ascii(".\n"@),
                ] + e.1);
        }
    }
    let open: &[u8] = b"(";
    let ghost open_chunk = seq![0x28u8];
    proof {
        reveal_byteslit(b"(");
        assert(open@ == open_chunk);
    }
    let ghost after_open = match expected@ {
        Some(e) => Some(
            seq![
                ckc_spec::v1text::atom_bytes(e.0.unwrap()),
                seq![0x29u8],
                seq![0x29u8],
                seq![0x29u8],
                ckc_spec::v1text::ascii(".\n"@),
            ] + e.1,
        ),
        None => None,
    };
    proof {
        if expected@ is Some {
            assert_seqs_equal!(guide_rest(guided.guide@.unwrap())
                == seq![open_chunk] + after_open.unwrap());
        }
    }
    if !doc_guided_literal(
        bytes,
        guided,
        open,
        Ghost(open_chunk),
        Ghost(after_open),
    at,
    ) {
        return None;
    }
    let ghost hash_expected = match expected@ {
        Some(e) => Some((
            e.0.unwrap(),
            seq![
                seq![0x29u8],
                seq![0x29u8],
                seq![0x29u8],
                ckc_spec::v1text::ascii(".\n"@),
            ] + e.1,
        )),
        None => None,
    };
    proof {
        if expected@ is Some {
            assert_seqs_equal!(guide_rest(guided.guide@.unwrap())
                == seq![ckc_spec::v1text::atom_bytes(
                    hash_expected.unwrap().0,
                )] + hash_expected.unwrap().1);
            assert(hash_expected.unwrap().1.len() > 0);
            assert(hash_expected.unwrap().1[0].len() > 0);
            assert(hash_expected.unwrap().1[0][0] == 0x29);
        }
    }
    let hash = match parts_guided_atom(
        bytes,
        guided,
        Ghost(0x29u8),
        Ghost(hash_expected),
    at,
    ) {
        Some(atom) => atom,
        None => return None,
    };
    proof {
        if let Some(e) = expected@ {
            assert(hash.name@ == e.0.unwrap());
            assert(ckc_spec::v1text::hex64(hash.name@));
        }
    }
    if !hex64_exec(&hash.name) {
        return None;
    }
    let close: &[u8] = b")";
    let ghost close_chunk = seq![0x29u8];
    proof {
        reveal_byteslit(b")");
        assert(close@ == close_chunk);
    }
    let ghost after_hash_close = match expected@ {
        Some(e) => Some(
            seq![
                seq![0x29u8],
                seq![0x29u8],
                ckc_spec::v1text::ascii(".\n"@),
            ] + e.1,
        ),
        None => None,
    };
    proof {
        if expected@ is Some {
            assert_seqs_equal!(guide_rest(guided.guide@.unwrap())
                == seq![close_chunk] + after_hash_close.unwrap());
        }
    }
    if !doc_guided_literal(
        bytes,
        guided,
        close,
        Ghost(close_chunk),
        Ghost(after_hash_close),
    at,
    ) {
        return None;
    }
    let ghost after_ulex_close = match expected@ {
        Some(e) => Some(
            seq![seq![0x29u8], ckc_spec::v1text::ascii(".\n"@)] + e.1,
        ),
        None => None,
    };
    proof {
        if expected@ is Some {
            assert_seqs_equal!(guide_rest(guided.guide@.unwrap())
                == seq![close_chunk] + after_ulex_close.unwrap());
        }
    }
    if !doc_guided_literal(
        bytes,
        guided,
        close,
        Ghost(close_chunk),
        Ghost(after_ulex_close),
    at,
    ) {
        return None;
    }
    let ghost after_record_close = match expected@ {
        Some(e) => Some(seq![ckc_spec::v1text::ascii(".\n"@)] + e.1),
        None => None,
    };
    proof {
        if expected@ is Some {
            assert_seqs_equal!(guide_rest(guided.guide@.unwrap())
                == seq![close_chunk] + after_record_close.unwrap());
        }
    }
    if !doc_guided_literal(
        bytes,
        guided,
        close,
        Ghost(close_chunk),
        Ghost(after_record_close),
    at,
    ) {
        return None;
    }
    let line_end: &[u8] = b".\n";
    proof {
        reveal_strlit(".\n");
        reveal_byteslit(b".\n");
        reveal(ckc_spec::v1text::ascii);
        assert(line_end@ == ckc_spec::v1text::ascii(".\n"@));
    }
    let ghost after_line = match expected@ {
        Some(e) => Some(e.1),
        None => None,
    };
    proof {
        if expected@ is Some {
            assert_seqs_equal!(guide_rest(guided.guide@.unwrap())
                == seq![ckc_spec::v1text::ascii(".\n"@)]
                    + after_line.unwrap());
        }
    }
    if !doc_guided_literal(
        bytes,
        guided,
        line_end,
        Ghost(ckc_spec::v1text::ascii(".\n"@)),
        Ghost(after_line),
    at,
    ) {
        return None;
    }
    proof {
        if let Some(e) = expected@ {
            assert(e.0 == Some(hash.name@));
            reveal(record_ulex_parts);
            assert(record_ulex_parts(e.0).len() == 7);
        }
        record_ulex_parts_flat(Some(hash.name@));
        assert_seqs_equal!(guided.cursor.prefix@
            == entry_prefix
                + query_ulex_stage(Some(hash.name@))
                + query_record_suffix_stage());
        reveal(ckc_spec::v1text::ulex_ok);
    }
    Some(EUlexField { value: Ghost(Some(hash.name@)) })
}

#[verifier::rlimit(500)]
fn parse_query_ulex(
    bytes: &[u8],
    guided: &mut EGuidedCursor,
    expected: Ghost<Option<ckc_spec::v1text::QueryFile>>,
at: &mut usize,
) -> (r: Option<EUlexField>)
    requires
        *old(at) <= bytes@.len(),
        guided_cursor_ok(bytes@, old(guided)),
        expected@ matches Some(q) ==> old(guided).guide@ matches Some(g)
            && g.parts == query_parts(q) && g.index == 16
            && ckc_spec::v1text::wf_query(q),
        expected@ is None ==> old(guided).guide@ is None,
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r matches Some(ulex) ==> {
            &&& guided_cursor_ok(bytes@, final(guided))
            &&& ckc_spec::v1text::ulex_ok(ulex.value@)
            &&& final(guided).cursor.prefix@
                == old(guided).cursor.prefix@
                    + query_ulex_stage(ulex.value@)
                    + query_record_suffix_stage()
        },
        expected@ matches Some(q) ==> {
            &&& r matches Some(ulex)
            &&& ulex.value@ == q.ulex
            &&& final(guided).guide@ matches Some(g)
                && g.parts == query_parts(q)
                && g.index == query_record_end(q.ulex)
        },
        expected@ is None ==> final(guided).guide@ is None,
{
    let ghost tail = match expected@ {
        Some(q) => query_text_parts(q.qtext)
            + query_projection_parts(q.goal, q.answers),
        None => Seq::<Seq<u8>>::empty(),
    };
    let ghost generic_expected = match expected@ {
        Some(q) => Some((q.ulex, tail)),
        None => None,
    };
    proof {
        if let Some(q) = expected@ {
            reveal(ckc_spec::v1text::wf_query);
            reveal(query_parts);
            reveal(query_line_parts);
            reveal(query_record_parts);
            reveal(record_ulex_parts);
            match q.ulex {
                None => {},
                Some(_) => {},
            }
            reveal(guide_rest);
            assert(guide_rest(guided.guide@.unwrap())
                == record_ulex_parts(q.ulex) + tail);
        }
    }
    let ulex = match parse_record_ulex(
        bytes,
        guided,
        Ghost(generic_expected),
    at,
    ) {
        Some(ulex) => ulex,
        None => return None,
    };
    proof {
        if let Some(q) = expected@ {
            reveal(record_ulex_parts);
            reveal(query_record_end);
            match q.ulex {
                None => {},
                Some(_) => {},
            }
        }
    }
    Some(ulex)
}

#[verifier::rlimit(300)]
fn parse_query_text(
    bytes: &[u8],
    guided: &mut EGuidedCursor,
    expected: Ghost<Option<ckc_spec::v1text::QueryFile>>,
at: &mut usize,
) -> (r: Option<ETextField>)
    requires
        *old(at) <= bytes@.len(),
        guided_cursor_ok(bytes@, old(guided)),
        expected@ matches Some(q) ==> old(guided).guide@ matches Some(g)
            && g.parts == query_parts(q)
            && g.index == query_record_end(q.ulex)
            && ckc_spec::v1text::wf_query(q),
        expected@ is None ==> old(guided).guide@ is None,
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r matches Some(text) ==> {
            &&& guided_cursor_ok(bytes@, final(guided))
            &&& ckc_spec::v1text::text_ok(text.value@)
            &&& final(guided).cursor.prefix@
                == old(guided).cursor.prefix@ + query_text_stage(text.value@)
        },
        expected@ matches Some(q) ==> {
            &&& r matches Some(text)
            &&& text.value@ == q.qtext
            &&& final(guided).guide@ matches Some(g)
                && g.parts == query_parts(q)
                && g.index == query_text_end(q.ulex)
        },
        expected@ is None && r is Some ==> final(guided).guide@ is None,
{
    let ghost old_prefix = guided.cursor.prefix@;
    proof {
        if let Some(q) = expected@ {
            reveal(query_parts);
            reveal(query_line_parts);
            reveal(query_record_parts);
            reveal(query_text_parts);
            reveal(query_record_end);
            reveal(query_text_end);
            reveal(ckc_spec::v1text::wf_query);
            match q.ulex {
                None => {},
                Some(_) => {},
            }
        }
    }

    let marker: &[u8] = b"% Q1: ";
    proof {
        reveal_strlit("% Q1: ");
        reveal_byteslit(b"% Q1: ");
        reveal(ckc_spec::v1text::ascii);
        assert(marker@ == ckc_spec::v1text::ascii("% Q1: "@));
    }
    if !guided_literal(
        bytes,
        guided,
        marker,
        Ghost(ckc_spec::v1text::ascii("% Q1: "@)),
    at,
    ) {
        return None;
    }

    let ghost text_expected = match expected@ {
        Some(q) => Some(q.qtext),
        None => None,
    };
    let text = match guided_text(bytes, guided, Ghost(text_expected), at) {
        Some(field) => field,
        None => return None,
    };
    if !guided_byte(bytes, guided, 0x0a, at) {
        return None;
    }
    proof {
        reveal(query_text_stage);
        assert_seqs_equal!(
            guided.cursor.prefix@
                == old_prefix + query_text_stage(text.value@)
        );
    }
    Some(text)
}

proof fn spanned_term_at_close(bytes: Seq<u8>, term: &ESpannedTerm)
    requires
        spanned_term_ok(bytes, term),
        term.end < bytes.len(),
        bytes[term.end as int] == 0x29,
    ensures
        term_at(bytes, term.start as int, term.end as int, term@),
{
    reveal(spanned_term_ok);
    reveal(parsed_term_ok);
    reveal(term_at);
    reveal(term_boundary);
}

fn track_parsed_term(
    bytes: &[u8],
    term: &ESpannedTerm,
    tracker: &mut EVarTracker,
at: &mut usize,
)
    requires
        *old(at) <= bytes@.len(),
        spanned_term_ok(bytes@, term),
        term_at(
            bytes@,
            term.start as int,
            term.end as int,
            term@,
        ),
        old(tracker).valid ==>
            tracker_state_ok(old(tracker).next, old(tracker).stream@),
        tracker_complete(old(tracker).valid, old(tracker).stream@),
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        final(tracker).stream@
            == old(tracker).stream@ + ckc_spec::term::var_stream(term@),
        final(tracker).valid ==>
            tracker_state_ok(final(tracker).next, final(tracker).stream@),
        tracker_complete(final(tracker).valid, final(tracker).stream@),
        old(tracker).stream@.len() <= term.start ==>
            final(tracker).stream@.len() <= term.end,
{
    let ghost initial = tracker.stream@;
    let ghost guide = GTermExpected { term: term@, end: term.end };
    let replay = parse_term(
        bytes,
        term.start,
        Ghost(Some(guide)),
        Ghost(guide),
        Ghost(initial),
        true,
        tracker,
    at,
    );
    proof {
        assert(replay matches Some(parsed)
            && parsed@ == term@ && parsed.end == term.end);
    }
}

pub struct EQueryProjection {
    pub goal: ESpannedTerm,
    pub answers: ESpannedTerm,
}

#[verifier::rlimit(2000)]
#[verifier::spinoff_prover]
fn parse_query_projection(
    bytes: &[u8],
    guided: &mut EGuidedCursor,
    expected: Ghost<Option<ckc_spec::v1text::QueryFile>>,
at: &mut usize,
) -> (r: Option<EQueryProjection>)
    requires
        *old(at) <= bytes@.len(),
        guided_cursor_ok(bytes@, old(guided)),
        expected@ matches Some(q) ==> old(guided).guide@ matches Some(g)
            && g.parts == query_parts(q)
            && g.index == query_text_end(q.ulex)
            && ckc_spec::v1text::wf_query(q),
        expected@ is None ==> old(guided).guide@ is None,
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r matches Some(projection) ==> {
            &&& guided_cursor_ok(bytes@, final(guided))
            &&& spanned_term_ok(bytes@, &projection.goal)
            &&& spanned_term_ok(bytes@, &projection.answers)
            &&& projection.goal.parsed.no_dollar
            &&& projection.answers.parsed.no_dollar
            &&& ckc_spec::term::var_canonical(
                ckc_spec::term::var_stream(projection.goal@)
                    + ckc_spec::term::var_stream(projection.answers@),
            )
            &&& final(guided).cursor.prefix@
                == old(guided).cursor.prefix@
                    + query_projection_stage(
                        projection.goal@,
                        projection.answers@,
                    )
        },
        expected@ matches Some(q) ==> {
            &&& r matches Some(projection)
            &&& projection.goal@ == q.goal
            &&& projection.answers@ == q.answers
            &&& final(guided).guide@ matches Some(g)
                && g.parts == query_parts(q)
                && g.index == query_parts_end(q.ulex)
        },
        expected@ is None && r is Some ==> final(guided).guide@ is None,
{
    let ghost old_prefix = guided.cursor.prefix@;
    proof {
        if let Some(q) = expected@ {
            reveal(query_parts);
            reveal(query_line_parts);
            reveal(query_record_parts);
            reveal(query_text_parts);
            reveal(query_projection_parts);
            reveal(query_record_end);
            reveal(query_text_end);
            reveal(query_parts_end);
            reveal(ckc_spec::v1text::wf_query);
            match q.ulex {
                None => {},
                Some(_) => {},
            }
        }
    }

    let ghost wrapper_expected = match expected@ {
        Some(_) => Some((
            ckc_spec::v1text::ascii("$guideline_query_projection"@),
            0x28u8,
        )),
        None => None,
    };
    let wrapper = match guided_atom(bytes, guided, Ghost(wrapper_expected), at) {
        Some(atom) => atom,
        None => return None,
    };
    let wrapper_name: &[u8] = b"$guideline_query_projection";
    proof {
        reveal_strlit("$guideline_query_projection");
        reveal_byteslit(b"$guideline_query_projection");
        reveal(ckc_spec::v1text::ascii);
        assert(wrapper_name@
            == ckc_spec::v1text::ascii("$guideline_query_projection"@));
    }
    if !vec_slice_equal(&wrapper.name, wrapper_name) {
        return None;
    }
    if !guided_byte(bytes, guided, 0x28, at) {
        return None;
    }

    let ghost goal_wrapper_expected = match expected@ {
        Some(_) => Some((ckc_spec::v1text::ascii("goal"@), 0x28u8)),
        None => None,
    };
    let goal_wrapper = match guided_atom(
        bytes,
        guided,
        Ghost(goal_wrapper_expected),
    at,
    ) {
        Some(atom) => atom,
        None => return None,
    };
    let goal_wrapper_name: &[u8] = b"goal";
    proof {
        reveal_strlit("goal");
        reveal_byteslit(b"goal");
        reveal(ckc_spec::v1text::ascii);
        assert(goal_wrapper_name@ == ckc_spec::v1text::ascii("goal"@));
    }
    if !vec_slice_equal(&goal_wrapper.name, goal_wrapper_name) {
        return None;
    }
    if !guided_byte(bytes, guided, 0x28, at) {
        return None;
    }

    let ghost goal_expected = match expected@ {
        Some(q) => Some((q.goal, 0x29u8)),
        None => None,
    };
    let goal = match guided_term(bytes, guided, Ghost(goal_expected), at) {
        Some(term) => term,
        None => return None,
    };
    proof {
        if let Some(q) = expected@ {
            reveal(spanned_term_ok);
            reveal(parsed_term_ok);
            assert(goal@ == q.goal);
            assert(goal.parsed.no_dollar);
        }
    }
    if !goal.parsed.no_dollar {
        return None;
    }
    let goal_end = guided.cursor.pos;
    proof {
        assert(goal_end == goal.end);
    }
    if !guided_byte(bytes, guided, 0x29, at) {
        return None;
    }
    proof {
        assert(goal_end == goal.end);
        spanned_term_at_close(bytes@, &goal);
    }
    let mut tracker = new_var_tracker();
    track_parsed_term(bytes, &goal, &mut tracker, at);
    proof {
        assert(tracker.stream@ == ckc_spec::term::var_stream(goal@));
        assert(tracker.stream@.len() <= goal.end);
    }
    if !guided_byte(bytes, guided, 0x2c, at) {
        return None;
    }

    let ghost answers_wrapper_expected = match expected@ {
        Some(_) => Some((ckc_spec::v1text::ascii("answers"@), 0x28u8)),
        None => None,
    };
    let answers_wrapper = match guided_atom(
        bytes,
        guided,
        Ghost(answers_wrapper_expected),
    at,
    ) {
        Some(atom) => atom,
        None => return None,
    };
    let answers_wrapper_name: &[u8] = b"answers";
    proof {
        reveal_strlit("answers");
        reveal_byteslit(b"answers");
        reveal(ckc_spec::v1text::ascii);
        assert(answers_wrapper_name@
            == ckc_spec::v1text::ascii("answers"@));
    }
    if !vec_slice_equal(&answers_wrapper.name, answers_wrapper_name) {
        return None;
    }
    if !guided_byte(bytes, guided, 0x28, at) {
        return None;
    }
    proof {
        reveal(parsed_atom_ok);
        assert(goal.end <= guided.cursor.pos);
    }
    let answers_start = guided.cursor.pos;

    let ghost answers_expected = match expected@ {
        Some(q) => Some((q.answers, 0x29u8)),
        None => None,
    };
    let answers = match guided_term(bytes, guided, Ghost(answers_expected), at) {
        Some(term) => term,
        None => return None,
    };
    proof {
        if let Some(q) = expected@ {
            reveal(spanned_term_ok);
            reveal(parsed_term_ok);
            assert(answers@ == q.answers);
            assert(answers.parsed.no_dollar);
        }
        assert(answers.start == answers_start);
        assert(goal.end <= answers.start);
        assert(tracker.stream@.len() <= answers.start);
    }
    if !answers.parsed.no_dollar {
        return None;
    }
    let answers_end = guided.cursor.pos;
    proof {
        assert(answers_end == answers.end);
    }
    if !guided_byte(bytes, guided, 0x29, at) {
        return None;
    }
    proof {
        assert(answers_end == answers.end);
        spanned_term_at_close(bytes@, &answers);
    }
    track_parsed_term(bytes, &answers, &mut tracker, at);
    proof {
        assert(tracker.stream@
            == ckc_spec::term::var_stream(goal@)
                + ckc_spec::term::var_stream(answers@));
        assert(tracker.stream@.len() <= answers.end);
        assert(answers.end <= bytes.len());
        assert(tracker.stream@.len() <= usize::MAX as nat);
        if let Some(q) = expected@ {
            assert(goal@ == q.goal);
            assert(answers@ == q.answers);
            assert(ckc_spec::term::var_canonical(tracker.stream@));
            reveal(tracker_complete);
            assert(tracker.valid);
        }
    }
    if !tracker.valid {
        return None;
    }
    proof {
        assert(tracker_state_ok(tracker.next, tracker.stream@));
        tracker_state_canonical(tracker.next, tracker.stream@);
    }

    if !guided_byte(bytes, guided, 0x29, at) {
        return None;
    }
    let line_end: &[u8] = b".\n";
    proof {
        reveal_strlit(".\n");
        reveal_byteslit(b".\n");
        reveal(ckc_spec::v1text::ascii);
        assert(line_end@ == ckc_spec::v1text::ascii(".\n"@));
    }
    if !guided_literal(
        bytes,
        guided,
        line_end,
        Ghost(ckc_spec::v1text::ascii(".\n"@)),
    at,
    ) {
        return None;
    }

    proof {
        reveal(query_projection_stage);
        reveal(query_projection_prefix_stage);
        reveal(query_projection_middle_stage);
        reveal(query_projection_suffix_stage);
        assert_seqs_equal!(
            guided.cursor.prefix@
                == old_prefix + query_projection_stage(goal@, answers@)
        );
    }
    Some(EQueryProjection { goal, answers })
}

#[verifier::rlimit(2000)]
#[verifier::spinoff_prover]
pub fn parse_query(
    bytes: &[u8],
    expected: Ghost<Option<ckc_spec::v1text::QueryFile>>,
at: &mut usize,
) -> (r: Option<EParsedV1>)
    requires
        *old(at) <= bytes@.len(),
        expected@ matches Some(q) ==>
            ckc_spec::v1text::wf_query(q)
                && ckc_spec::v1text::print_query(q) == bytes@,
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r matches Some(parsed) ==> parsed_v1_ok(bytes@, &parsed),
        expected@ matches Some(q) ==> r matches Some(parsed)
            && parsed@ == ckc_spec::v1text::V1File::Query(q),
{
    let ghost expected_parts = match expected@ {
        Some(q) => Some(query_parts(q)),
        None => None,
    };
    proof {
        if let Some(q) = expected@ {
            query_flat_is_print(q);
            query_parts_flat(q);
            assert(bytes@ == query_parts(q).flatten());
        }
    }
    let mut guided = new_guided_cursor(bytes, Ghost(expected_parts));

    let line_qid = match parse_query_line(bytes, &mut guided, expected, at) {
        Some(qid) => qid,
        None => return None,
    };
    let ace = match parse_query_record_prefix(
        bytes,
        &mut guided,
        &line_qid,
        expected,
    at,
    ) {
        Some(hash) => hash,
        None => return None,
    };
    let ulex = match parse_query_ulex(bytes, &mut guided, expected, at) {
        Some(field) => field,
        None => return None,
    };
    let text = match parse_query_text(bytes, &mut guided, expected, at) {
        Some(field) => field,
        None => return None,
    };
    let projection = match parse_query_projection(
        bytes,
        &mut guided,
        expected,
    at,
    ) {
        Some(projection) => projection,
        None => return None,
    };

    proof {
        if let Some(q) = expected@ {
            reveal(guided_cursor_ok);
            reveal(parts_progress);
            reveal(query_parts_end);
            reveal(query_text_end);
            reveal(query_record_end);
            reveal(query_parts);
            reveal(query_line_parts);
            reveal(query_record_parts);
            reveal(query_text_parts);
            reveal(query_projection_parts);
            match q.ulex {
                None => {},
                Some(_) => {},
            }
            query_parts(q).lemma_take_len();
            assert(query_parts_end(q.ulex) == query_parts(q).len());
            assert(guided.cursor.prefix@ == bytes@);
            assert(guided.cursor.prefix@.len() == guided.cursor.pos);
            assert(guided.cursor.pos == bytes.len());
        }
    }
    if guided.cursor.pos != bytes.len() {
        return None;
    }

    let ghost model = ckc_spec::v1text::QueryFile {
        qid: line_qid.value@,
        ace: ace.name@,
        ulex: ulex.value@,
        qtext: text.value@,
        goal: projection.goal@,
        answers: projection.answers@,
    };
    proof {
        assert(guided_cursor_ok(bytes@, &guided));
        reveal(guided_cursor_ok);
        assert(cursor_ok(bytes@, &guided.cursor));
        reveal(cursor_ok);
        assert(guided.cursor.pos == bytes@.len());
        assert(guided.cursor.prefix@
            == bytes@.subrange(0, bytes@.len() as int));
        assert_seqs_equal!(
            bytes@.subrange(0, bytes@.len() as int) == bytes@
        );
        assert(guided.cursor.prefix@ == bytes@);
        reveal(query_flat);
        reveal(query_record_stage);
        assert_seqs_equal!(guided.cursor.prefix@ == query_flat(model));
        query_flat_is_print(model);
        if let Some(q) = expected@ {
            assert(line_qid.value@ == q.qid);
            assert(ace.name@ == q.ace);
            assert(ulex.value@ == q.ulex);
            assert(text.value@ == q.qtext);
            assert(projection.goal@ == q.goal);
            assert(projection.answers@ == q.answers);
            assert(model == q);
        }
        reveal(spanned_term_ok);
        reveal(parsed_term_ok);
        reveal(ckc_spec::v1text::wf_query);
        reveal(ckc_spec::v1text::wf_v1);
        reveal(ckc_spec::v1text::print_v1);
        reveal(parsed_v1_ok);
    }
    Some(EParsedV1 {
        file: Ghost(ckc_spec::v1text::V1File::Query(model)),
    })
}

#[verifier::rlimit(500)]
fn record_atomic_term(
    bytes: &[u8],
    start: usize,
    term: &ESpannedTerm,
    initial_stream: Ghost<Seq<nat>>,
    tracker: &mut EVarTracker,
at: &mut usize,
)
    requires
        *old(at) <= bytes@.len(),
        term.start == start,
        spanned_term_ok(bytes@, term),
        term_at(bytes@, start as int, term.end as int, term@),
        atomic_term(term@),
        initial_stream@ == old(tracker).stream@,
        old(tracker).valid ==>
            tracker_state_ok(old(tracker).next, old(tracker).stream@),
        tracker_complete(old(tracker).valid, old(tracker).stream@),
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        final(tracker).stream@
            == initial_stream@ + ckc_spec::term::var_stream(term@),
        final(tracker).valid ==>
            tracker_state_ok(final(tracker).next, final(tracker).stream@),
        tracker_complete(final(tracker).valid, final(tracker).stream@),
{
    proof {
        expected_atomic_shape(bytes@, start, term.end, term@);
    }
    if 0x41 <= bytes[start] && bytes[start] <= 0x5a {
        let ghost value = match term@ {
            Term::Var(k) => k,
            _ => 0,
        };
        proof {
            match term@ {
                Term::Var(k) => assert(value == k),
                Term::Int(_) => {
                    reveal(ckc_spec::v1text::is_digit_b);
                    assert(false);
                },
                Term::Nil => assert(false),
                Term::Atom(_) => assert(false),
                Term::Comp(_, _) => {
                    reveal(atomic_term);
                    assert(false);
                },
            }
            reveal(term_at);
            reveal_with_fuel(ckc_spec::v1text::term_bytes, 2);
            assert(ckc_spec::v1text::var_bytes(value)
                == bytes@.subrange(start as int, term.end as int));
        }
        let _valid = record_variable(
            bytes,
            start,
            term.end,
            Ghost(value),
            tracker,
        at,
        );
        proof {
            reveal(ckc_spec::term::var_stream);
            assert_seqs_equal!(
                initial_stream@.push(value) == initial_stream@ + seq![value]
            );
        }
    } else {
        proof {
            match term@ {
                Term::Var(_) => assert(false),
                Term::Int(_) | Term::Nil | Term::Atom(_) => {
                    reveal(ckc_spec::term::var_stream);
                },
                Term::Comp(_, _) => {
                    reveal(atomic_term);
                    assert(false);
                },
            }
            assert_seqs_equal!(
                initial_stream@ + Seq::<nat>::empty() == initial_stream@
            );
        }
    }
}

pub open spec fn doc_line_stage(docid: Seq<u8>) -> Seq<u8> {
    ckc_spec::v1text::doc_line1(docid)
}

pub open spec fn doc_record_head_stage(docid: Seq<u8>) -> Seq<u8> {
    ckc_spec::v1text::atom_bytes(
        ckc_spec::v1text::ascii("guideline_document"@),
    )
        + seq![0x28u8]
        + ckc_spec::v1text::atom_bytes(docid)
        + seq![0x2cu8]
}

pub open spec fn doc_record_suffix_stage() -> Seq<u8> {
    seq![0x29u8, 0x29u8] + ckc_spec::v1text::ascii(".\n"@)
}

pub open spec fn doc_record_stage(
    docid: Seq<u8>,
    ace: Seq<u8>,
    ulex: Option<Seq<u8>>,
) -> Seq<u8> {
    doc_record_head_stage(docid)
        + query_record_ace_stage(ace)
        + query_record_ulex_open_stage()
        + query_ulex_stage(ulex)
        + doc_record_suffix_stage()
}

pub open spec fn doc_prefix_stage(
    d: ckc_spec::v1text::DocFile,
) -> Seq<u8> {
    doc_line_stage(d.docid)
        + ckc_spec::v1text::decls_from(0)
        + ckc_spec::v1text::term_line(
            ckc_spec::v1text::schema_version_term(),
        )
        + doc_record_stage(d.docid, d.ace, d.ulex)
}

pub open spec fn doc_flat(d: ckc_spec::v1text::DocFile) -> Seq<u8> {
    doc_prefix_stage(d) + ckc_spec::v1text::bundles_bytes(d.bundles)
}

pub open spec fn doc_line_parts(docid: Seq<u8>) -> Seq<Seq<u8>> {
    seq![
        ckc_spec::v1text::ascii("% "@),
        docid,
        ckc_spec::v1text::ascii(
            ".pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.\n"@,
        ),
    ]
}

pub open spec fn doc_decl_parts() -> Seq<Seq<u8>> {
    seq![ckc_spec::v1text::decls_from(0)]
}

pub open spec fn doc_record_parts(
    docid: Seq<u8>,
    ace: Seq<u8>,
    ulex: Option<Seq<u8>>,
) -> Seq<Seq<u8>> {
    match ulex {
        None => seq![
            ckc_spec::v1text::atom_bytes(
                ckc_spec::v1text::ascii("guideline_document"@),
            ),
            seq![0x28u8],
            ckc_spec::v1text::atom_bytes(docid),
            seq![0x2cu8],
            ckc_spec::v1text::atom_bytes(
                ckc_spec::v1text::ascii("ace_sha256"@),
            ),
            seq![0x28u8],
            ckc_spec::v1text::atom_bytes(ace),
            seq![0x29u8],
            seq![0x2cu8],
            ckc_spec::v1text::atom_bytes(
                ckc_spec::v1text::ascii("ulex"@),
            ),
            seq![0x28u8],
            ckc_spec::v1text::atom_bytes(ckc_spec::v1text::ascii("none"@)),
            seq![0x29u8],
            seq![0x29u8],
            ckc_spec::v1text::ascii(".\n"@),
        ],
        Some(hash) => seq![
            ckc_spec::v1text::atom_bytes(
                ckc_spec::v1text::ascii("guideline_document"@),
            ),
            seq![0x28u8],
            ckc_spec::v1text::atom_bytes(docid),
            seq![0x2cu8],
            ckc_spec::v1text::atom_bytes(
                ckc_spec::v1text::ascii("ace_sha256"@),
            ),
            seq![0x28u8],
            ckc_spec::v1text::atom_bytes(ace),
            seq![0x29u8],
            seq![0x2cu8],
            ckc_spec::v1text::atom_bytes(
                ckc_spec::v1text::ascii("ulex"@),
            ),
            seq![0x28u8],
            ckc_spec::v1text::atom_bytes(
                ckc_spec::v1text::ascii("sha256"@),
            ),
            seq![0x28u8],
            ckc_spec::v1text::atom_bytes(hash),
            seq![0x29u8],
            seq![0x29u8],
            seq![0x29u8],
            ckc_spec::v1text::ascii(".\n"@),
        ],
    }
}

pub open spec fn doc_lit_parts(gs: Seq<Term>) -> Seq<Seq<u8>>
    decreases gs,
{
    if gs.len() == 0 {
        Seq::empty()
    } else if gs.len() == 1 {
        seq![ckc_spec::v1text::term_bytes(gs[0])]
    } else {
        seq![
            ckc_spec::v1text::term_bytes(gs[0]),
            ckc_spec::v1text::ascii(", "@),
        ] + doc_lit_parts(gs.drop_first())
    }
}

pub open spec fn doc_body_item_parts(
    it: ckc_spec::v1text::BodyItem,
) -> Seq<Seq<u8>> {
    match it {
        ckc_spec::v1text::BodyItem::Pos(l) => {
            seq![ckc_spec::v1text::term_bytes(l)]
        },
        ckc_spec::v1text::BodyItem::Naf(gs) => if gs.len() == 1 {
            seq![
                ckc_spec::v1text::ascii("\\+ "@),
                ckc_spec::v1text::term_bytes(gs[0]),
            ]
        } else {
            seq![
                ckc_spec::v1text::ascii("\\+ "@),
                seq![0x28u8],
            ] + doc_lit_parts(gs)
                + seq![seq![0x29u8]]
        },
    }
}

pub open spec fn doc_body_parts(
    items: Seq<ckc_spec::v1text::BodyItem>,
) -> Seq<Seq<u8>>
    decreases items,
{
    if items.len() == 0 {
        Seq::empty()
    } else if items.len() == 1 {
        doc_body_item_parts(items[0])
    } else {
        doc_body_item_parts(items[0])
            + seq![ckc_spec::v1text::ascii(", "@)]
            + doc_body_parts(items.drop_first())
    }
}

pub open spec fn doc_clause_parts(
    c: ckc_spec::v1text::DocClause,
) -> Seq<Seq<u8>> {
    if c.body.len() == 0 {
        seq![
            ckc_spec::v1text::term_bytes(c.head),
            ckc_spec::v1text::ascii(".\n"@),
        ]
    } else {
        seq![
            ckc_spec::v1text::term_bytes(c.head),
            ckc_spec::v1text::ascii(" :- "@),
        ] + doc_body_parts(c.body)
            + seq![ckc_spec::v1text::ascii(".\n"@)]
    }
}

pub open spec fn doc_clauses_parts(
    cs: Seq<ckc_spec::v1text::DocClause>,
) -> Seq<Seq<u8>>
    decreases cs,
{
    if cs.len() == 0 {
        Seq::empty()
    } else {
        doc_clause_parts(cs[0]) + doc_clauses_parts(cs.drop_first())
    }
}

pub open spec fn doc_marker_parts(
    b: ckc_spec::v1text::Bundle,
) -> Seq<Seq<u8>> {
    seq![
        ckc_spec::v1text::ascii("% S"@),
        ckc_spec::v1text::udec_bytes(b.s),
        ckc_spec::v1text::ascii(": "@),
        b.text,
        seq![0x0au8],
    ]
}

pub open spec fn doc_bundle_parts(
    b: ckc_spec::v1text::Bundle,
) -> Seq<Seq<u8>> {
    doc_marker_parts(b) + doc_clauses_parts(b.clauses)
}

pub open spec fn doc_bundles_parts(
    bs: Seq<ckc_spec::v1text::Bundle>,
) -> Seq<Seq<u8>>
    decreases bs,
{
    if bs.len() == 0 {
        Seq::empty()
    } else {
        doc_bundle_parts(bs[0]) + doc_bundles_parts(bs.drop_first())
    }
}

pub open spec fn doc_prefix_parts(
    d: ckc_spec::v1text::DocFile,
) -> Seq<Seq<u8>> {
    doc_line_parts(d.docid)
        + doc_decl_parts()
        + seq![ckc_spec::v1text::term_line(
            ckc_spec::v1text::schema_version_term(),
        )]
        + doc_record_parts(d.docid, d.ace, d.ulex)
}

pub open spec fn doc_parts(
    d: ckc_spec::v1text::DocFile,
) -> Seq<Seq<u8>> {
    doc_prefix_parts(d) + doc_bundles_parts(d.bundles)
}

proof fn doc_line_parts_flat(docid: Seq<u8>)
    ensures doc_line_parts(docid).flatten() == doc_line_stage(docid),
{
    reveal(doc_line_parts);
    reveal(doc_line_stage);
    reveal(ckc_spec::v1text::doc_line1);
    reveal_with_fuel(Seq::<_>::flatten, 5);
}

proof fn doc_decl_parts_flat()
    ensures doc_decl_parts().flatten() == ckc_spec::v1text::decls_from(0),
{
    reveal(doc_decl_parts);
    reveal_with_fuel(Seq::<_>::flatten, 11);
    reveal_with_fuel(ckc_spec::v1text::decls_from, 11);
}

#[verifier::rlimit(500)]
proof fn doc_record_parts_flat(
    docid: Seq<u8>,
    ace: Seq<u8>,
    ulex: Option<Seq<u8>>,
)
    ensures doc_record_parts(docid, ace, ulex).flatten()
        == doc_record_stage(docid, ace, ulex),
{
    let head = seq![
        ckc_spec::v1text::atom_bytes(
            ckc_spec::v1text::ascii("guideline_document"@),
        ),
        seq![0x28u8],
        ckc_spec::v1text::atom_bytes(docid),
        seq![0x2cu8],
    ];
    let ace_parts = seq![
        ckc_spec::v1text::atom_bytes(
            ckc_spec::v1text::ascii("ace_sha256"@),
        ),
        seq![0x28u8],
        ckc_spec::v1text::atom_bytes(ace),
        seq![0x29u8],
        seq![0x2cu8],
    ];
    let ulex_open = seq![
        ckc_spec::v1text::atom_bytes(ckc_spec::v1text::ascii("ulex"@)),
        seq![0x28u8],
    ];
    let ulex_parts = match ulex {
        None => seq![ckc_spec::v1text::atom_bytes(
            ckc_spec::v1text::ascii("none"@),
        )],
        Some(hash) => seq![
            ckc_spec::v1text::atom_bytes(
                ckc_spec::v1text::ascii("sha256"@),
            ),
            seq![0x28u8],
            ckc_spec::v1text::atom_bytes(hash),
            seq![0x29u8],
        ],
    };
    let suffix = seq![
        seq![0x29u8],
        seq![0x29u8],
        ckc_spec::v1text::ascii(".\n"@),
    ];
    assert(doc_record_parts(docid, ace, ulex)
        == ((head + ace_parts) + (ulex_open + ulex_parts)) + suffix) by {
        reveal(doc_record_parts);
    }
    assert(head.flatten() == doc_record_head_stage(docid)) by {
        reveal(doc_record_head_stage);
        reveal_with_fuel(Seq::<_>::flatten, 6);
    }
    assert(ace_parts.flatten() == query_record_ace_stage(ace)) by {
        reveal(query_record_ace_stage);
        reveal_with_fuel(Seq::<_>::flatten, 7);
    }
    assert(ulex_open.flatten() == query_record_ulex_open_stage()) by {
        reveal(query_record_ulex_open_stage);
        reveal_with_fuel(Seq::<_>::flatten, 4);
    }
    assert(ulex_parts.flatten() == query_ulex_stage(ulex)) by {
        reveal(query_ulex_stage);
        match ulex {
            None => reveal_with_fuel(Seq::<_>::flatten, 3),
            Some(_) => reveal_with_fuel(Seq::<_>::flatten, 6),
        }
    }
    assert(suffix.flatten() == doc_record_suffix_stage()) by {
        reveal(doc_record_suffix_stage);
        reveal_with_fuel(Seq::<_>::flatten, 5);
    }
    vstd::seq_lib::lemma_flatten_concat(head, ace_parts);
    vstd::seq_lib::lemma_flatten_concat(ulex_open, ulex_parts);
    vstd::seq_lib::lemma_flatten_concat(
        head + ace_parts,
        ulex_open + ulex_parts,
    );
    vstd::seq_lib::lemma_flatten_concat(
        (head + ace_parts) + (ulex_open + ulex_parts),
        suffix,
    );
    reveal(doc_record_stage);
}

proof fn doc_lit_parts_flat(gs: Seq<Term>)
    ensures doc_lit_parts(gs).flatten()
        == ckc_spec::v1text::lit_list_bytes(gs),
    decreases gs,
{
    reveal(doc_lit_parts);
    reveal(ckc_spec::v1text::lit_list_bytes);
    if gs.len() == 0 {
        reveal_with_fuel(Seq::<_>::flatten, 1);
    } else if gs.len() == 1 {
        reveal_with_fuel(Seq::<_>::flatten, 3);
    } else {
        doc_lit_parts_flat(gs.drop_first());
        let first = seq![
            ckc_spec::v1text::term_bytes(gs[0]),
            ckc_spec::v1text::ascii(", "@),
        ];
        vstd::seq_lib::lemma_flatten_concat(
            first,
            doc_lit_parts(gs.drop_first()),
        );
        reveal_with_fuel(Seq::<_>::flatten, 4);
    }
}

proof fn lit_list_bytes_push(gs: Seq<Term>, g: Term)
    requires gs.len() > 0,
    ensures
        ckc_spec::v1text::lit_list_bytes(gs.push(g))
            == ckc_spec::v1text::lit_list_bytes(gs)
                + ckc_spec::v1text::ascii(", "@)
                + ckc_spec::v1text::term_bytes(g),
    decreases gs.len(),
{
    if gs.len() == 1 {
        assert_seqs_equal!(gs.push(g).drop_first() == seq![g]);
        reveal_with_fuel(ckc_spec::v1text::lit_list_bytes, 3);
    } else {
        assert(gs.drop_first().len() > 0);
        lit_list_bytes_push(gs.drop_first(), g);
        assert_seqs_equal!(gs.push(g).drop_first() == gs.drop_first().push(g));
        reveal_with_fuel(ckc_spec::v1text::lit_list_bytes, 2);
    }
}

proof fn doc_body_item_parts_flat(it: ckc_spec::v1text::BodyItem)
    ensures doc_body_item_parts(it).flatten()
        == ckc_spec::v1text::body_item_bytes(it),
{
    reveal(doc_body_item_parts);
    reveal(ckc_spec::v1text::body_item_bytes);
    match it {
        ckc_spec::v1text::BodyItem::Pos(_) => {
            reveal_with_fuel(Seq::<_>::flatten, 3);
        },
        ckc_spec::v1text::BodyItem::Naf(gs) => if gs.len() == 1 {
            reveal_with_fuel(Seq::<_>::flatten, 4);
        } else {
            doc_lit_parts_flat(gs);
            let prefix = seq![
                ckc_spec::v1text::ascii("\\+ "@),
                seq![0x28u8],
            ];
            let suffix = seq![seq![0x29u8]];
            assert(prefix.flatten()
                == ckc_spec::v1text::ascii("\\+ ("@)) by {
                reveal_strlit("\\+ ");
                reveal_strlit("\\+ (");
                reveal(ckc_spec::v1text::ascii);
                reveal_with_fuel(Seq::<_>::flatten, 4);
            }
            vstd::seq_lib::lemma_flatten_concat(prefix, doc_lit_parts(gs));
            vstd::seq_lib::lemma_flatten_concat(
                prefix + doc_lit_parts(gs),
                suffix,
            );
            reveal_with_fuel(Seq::<_>::flatten, 3);
        },
    }
}

proof fn doc_body_parts_flat(items: Seq<ckc_spec::v1text::BodyItem>)
    ensures doc_body_parts(items).flatten()
        == ckc_spec::v1text::body_bytes(items),
    decreases items,
{
    reveal(doc_body_parts);
    reveal(ckc_spec::v1text::body_bytes);
    if items.len() == 0 {
        reveal_with_fuel(Seq::<_>::flatten, 1);
    } else if items.len() == 1 {
        doc_body_item_parts_flat(items[0]);
    } else {
        doc_body_item_parts_flat(items[0]);
        doc_body_parts_flat(items.drop_first());
        let comma = seq![ckc_spec::v1text::ascii(", "@)];
        vstd::seq_lib::lemma_flatten_concat(
            doc_body_item_parts(items[0]),
            comma,
        );
        vstd::seq_lib::lemma_flatten_concat(
            doc_body_item_parts(items[0]) + comma,
            doc_body_parts(items.drop_first()),
        );
        reveal_with_fuel(Seq::<_>::flatten, 3);
    }
}

proof fn doc_clause_parts_flat(c: ckc_spec::v1text::DocClause)
    ensures doc_clause_parts(c).flatten()
        == ckc_spec::v1text::clause_line(c),
{
    reveal(doc_clause_parts);
    reveal(ckc_spec::v1text::clause_line);
    reveal(ckc_spec::v1text::term_line);
    if c.body.len() == 0 {
        reveal_with_fuel(Seq::<_>::flatten, 4);
    } else {
        doc_body_parts_flat(c.body);
        let prefix = seq![
            ckc_spec::v1text::term_bytes(c.head),
            ckc_spec::v1text::ascii(" :- "@),
        ];
        let suffix = seq![ckc_spec::v1text::ascii(".\n"@)];
        vstd::seq_lib::lemma_flatten_concat(prefix, doc_body_parts(c.body));
        vstd::seq_lib::lemma_flatten_concat(
            prefix + doc_body_parts(c.body),
            suffix,
        );
        reveal_with_fuel(Seq::<_>::flatten, 4);
    }
}

proof fn doc_clauses_parts_flat(cs: Seq<ckc_spec::v1text::DocClause>)
    ensures doc_clauses_parts(cs).flatten()
        == ckc_spec::v1text::clauses_bytes(cs),
    decreases cs,
{
    reveal(doc_clauses_parts);
    reveal(ckc_spec::v1text::clauses_bytes);
    if cs.len() == 0 {
        reveal_with_fuel(Seq::<_>::flatten, 1);
    } else {
        doc_clause_parts_flat(cs[0]);
        doc_clauses_parts_flat(cs.drop_first());
        vstd::seq_lib::lemma_flatten_concat(
            doc_clause_parts(cs[0]),
            doc_clauses_parts(cs.drop_first()),
        );
    }
}

proof fn doc_marker_parts_flat(b: ckc_spec::v1text::Bundle)
    ensures doc_marker_parts(b).flatten()
        == ckc_spec::v1text::marker_line(b.s, b.text),
{
    reveal(doc_marker_parts);
    reveal(ckc_spec::v1text::marker_line);
    reveal_with_fuel(Seq::<_>::flatten, 7);
}

proof fn doc_bundle_parts_flat(b: ckc_spec::v1text::Bundle)
    ensures doc_bundle_parts(b).flatten()
        == ckc_spec::v1text::marker_line(b.s, b.text)
            + ckc_spec::v1text::clauses_bytes(b.clauses),
{
    doc_marker_parts_flat(b);
    doc_clauses_parts_flat(b.clauses);
    vstd::seq_lib::lemma_flatten_concat(
        doc_marker_parts(b),
        doc_clauses_parts(b.clauses),
    );
    reveal(doc_bundle_parts);
}

proof fn doc_bundles_parts_flat(bs: Seq<ckc_spec::v1text::Bundle>)
    ensures doc_bundles_parts(bs).flatten()
        == ckc_spec::v1text::bundles_bytes(bs),
    decreases bs,
{
    reveal(doc_bundles_parts);
    reveal(ckc_spec::v1text::bundles_bytes);
    if bs.len() == 0 {
        reveal_with_fuel(Seq::<_>::flatten, 1);
    } else {
        doc_bundle_parts_flat(bs[0]);
        doc_bundles_parts_flat(bs.drop_first());
        vstd::seq_lib::lemma_flatten_concat(
            doc_bundle_parts(bs[0]),
            doc_bundles_parts(bs.drop_first()),
        );
    }
}

proof fn args_three_bytes(a: Term, b: Term, c: Term)
    ensures
        ckc_spec::v1text::args_bytes(seq![a, b, c])
            == ckc_spec::v1text::term_bytes(a)
                + seq![0x2cu8]
                + ckc_spec::v1text::term_bytes(b)
                + seq![0x2cu8]
                + ckc_spec::v1text::term_bytes(c),
{
    reveal_with_fuel(ckc_spec::v1text::args_bytes, 4);
}

#[verifier::rlimit(5000)]
#[verifier::spinoff_prover]
proof fn doc_record_stage_is_print(d: ckc_spec::v1text::DocFile)
    ensures doc_record_stage(d.docid, d.ace, d.ulex)
        == ckc_spec::v1text::term_line(
            ckc_spec::v1text::doc_record_term(d),
        ),
{
    let doc_name = ckc_spec::v1text::ascii("guideline_document"@);
    let ace_name = ckc_spec::v1text::ascii("ace_sha256"@);
    let ulex_name = ckc_spec::v1text::ascii("ulex"@);
    let sha_name = ckc_spec::v1text::ascii("sha256"@);
    let docid = Term::Atom(d.docid);
    let ace_hash = Term::Atom(d.ace);
    let ace = Term::Comp(ace_name, seq![ace_hash]);
    let ulex_value = ckc_spec::v1text::ulex_term(d.ulex);
    let ulex = Term::Comp(ulex_name, seq![ulex_value]);
    reveal_strlit("guideline_document");
    reveal_strlit("ace_sha256");
    reveal_strlit("ulex");
    reveal_strlit("sha256");
    reveal(ckc_spec::v1text::ascii);
    reveal(ckc_spec::v1text::curly_name);
    assert(ace_name != ckc_spec::v1text::curly_name());
    assert(ulex_name != ckc_spec::v1text::curly_name());
    assert(sha_name != ckc_spec::v1text::curly_name());
    atom_term_bytes(d.docid);
    atom_term_bytes(d.ace);
    args_one_bytes(ace_hash);
    regular_comp_bytes(ace_name, seq![ace_hash]);
    match d.ulex {
        None => {
            let none_name = ckc_spec::v1text::ascii("none"@);
            reveal_strlit("none");
            reveal(ckc_spec::v1text::ascii);
            atom_term_bytes(none_name);
        },
        Some(hash) => {
            let hash_term = Term::Atom(hash);
            atom_term_bytes(hash);
            args_one_bytes(hash_term);
            regular_comp_bytes(sha_name, seq![hash_term]);
        },
    }
    args_one_bytes(ulex_value);
    regular_comp_bytes(ulex_name, seq![ulex_value]);
    args_three_bytes(docid, ace, ulex);
    regular_comp_bytes(doc_name, seq![docid, ace, ulex]);
    reveal(doc_record_stage);
    reveal(doc_record_head_stage);
    reveal(query_record_ace_stage);
    reveal(query_record_ulex_open_stage);
    reveal(query_ulex_stage);
    reveal(doc_record_suffix_stage);
    reveal(ckc_spec::v1text::term_line);
    reveal(ckc_spec::v1text::doc_record_term);
    reveal(ckc_spec::v1text::ulex_term);
}

proof fn doc_prefix_parts_flat(d: ckc_spec::v1text::DocFile)
    ensures doc_prefix_parts(d).flatten() == doc_prefix_stage(d),
{
    let line = doc_line_parts(d.docid);
    let decls = doc_decl_parts();
    let schema = seq![ckc_spec::v1text::term_line(
        ckc_spec::v1text::schema_version_term(),
    )];
    let record = doc_record_parts(d.docid, d.ace, d.ulex);
    assert(doc_prefix_parts(d) == ((line + decls) + schema) + record) by {
        reveal(doc_prefix_parts);
    }
    doc_line_parts_flat(d.docid);
    doc_decl_parts_flat();
    doc_record_parts_flat(d.docid, d.ace, d.ulex);
    assert(schema.flatten() == ckc_spec::v1text::term_line(
        ckc_spec::v1text::schema_version_term(),
    )) by {
        reveal_with_fuel(Seq::<_>::flatten, 3);
    }
    vstd::seq_lib::lemma_flatten_concat(line, decls);
    vstd::seq_lib::lemma_flatten_concat(line + decls, schema);
    vstd::seq_lib::lemma_flatten_concat((line + decls) + schema, record);
    reveal(doc_prefix_stage);
}

proof fn doc_parts_flat(d: ckc_spec::v1text::DocFile)
    ensures doc_parts(d).flatten() == doc_flat(d),
{
    doc_prefix_parts_flat(d);
    doc_bundles_parts_flat(d.bundles);
    vstd::seq_lib::lemma_flatten_concat(
        doc_prefix_parts(d),
        doc_bundles_parts(d.bundles),
    );
    reveal(doc_parts);
    reveal(doc_flat);
}

#[verifier::rlimit(5000)]
#[verifier::spinoff_prover]
proof fn doc_flat_is_print(d: ckc_spec::v1text::DocFile)
    ensures doc_flat(d) == ckc_spec::v1text::print_doc(d),
{
    doc_record_stage_is_print(d);
    reveal(doc_flat);
    reveal(doc_prefix_stage);
    reveal(doc_line_stage);
    reveal(ckc_spec::v1text::print_doc);
}

proof fn schema_version_name_bare()
    ensures
        ckc_spec::v1text::atom_bytes(
            ckc_spec::v1text::ascii("guideline_schema_version"@),
        ) == ckc_spec::v1text::ascii("guideline_schema_version"@),
{
    let name = ckc_spec::v1text::ascii("guideline_schema_version"@);
    reveal_strlit("guideline_schema_version");
    reveal(ckc_spec::v1text::ascii);
    assert_seqs_equal!(name == seq![
        0x67u8, 0x75u8, 0x69u8, 0x64u8, 0x65u8, 0x6cu8,
        0x69u8, 0x6eu8, 0x65u8, 0x5fu8, 0x73u8, 0x63u8,
        0x68u8, 0x65u8, 0x6du8, 0x61u8, 0x5fu8, 0x76u8,
        0x65u8, 0x72u8, 0x73u8, 0x69u8, 0x6fu8, 0x6eu8,
    ]);
    assert(ckc_spec::v1text::all_in(
        name,
        |b: u8| ckc_spec::v1text::is_alnum_b(b),
    )) by {
        reveal(ckc_spec::v1text::all_in);
        assert forall|i: int| 0 <= i < name.len()
            implies ckc_spec::v1text::is_alnum_b(name[i]) by {
            assert(i == 0 || i == 1 || i == 2 || i == 3 || i == 4
                || i == 5 || i == 6 || i == 7 || i == 8 || i == 9
                || i == 10 || i == 11 || i == 12 || i == 13
                || i == 14 || i == 15 || i == 16 || i == 17
                || i == 18 || i == 19 || i == 20 || i == 21
                || i == 22 || i == 23);
            reveal(ckc_spec::v1text::is_alnum_b);
            reveal(ckc_spec::v1text::is_lower_b);
            reveal(ckc_spec::v1text::is_digit_b);
        }
    }
    reveal(ckc_spec::v1text::alpha_bare);
    reveal(ckc_spec::v1text::atom_bare);
    reveal(ckc_spec::v1text::atom_bytes);
    reveal(ckc_spec::v1text::is_lower_b);
}

#[verifier::rlimit(500)]
fn parse_doc_line(
    bytes: &[u8],
    guided: &mut EGuidedCursor,
    expected: Ghost<Option<ckc_spec::v1text::DocFile>>,
at: &mut usize,
) -> (r: Option<ENameField>)
    requires
        *old(at) <= bytes@.len(),
        guided_cursor_ok(bytes@, old(guided)),
        expected@ matches Some(d) ==> old(guided).guide@ matches Some(g)
            && g.parts == doc_parts(d) && g.index == 0
            && ckc_spec::v1text::wf_doc(d),
        expected@ is None ==> old(guided).guide@ is None,
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r matches Some(docid) ==> {
            &&& guided_cursor_ok(bytes@, final(guided))
            &&& ckc_spec::v1text::name_ok(docid.value@)
            &&& final(guided).cursor.prefix@
                == old(guided).cursor.prefix@ + doc_line_stage(docid.value@)
        },
        expected@ matches Some(d) ==> {
            &&& r matches Some(docid) && docid.value@ == d.docid
            &&& final(guided).guide@ matches Some(g)
                && g.parts == doc_parts(d) && g.index == 3
        },
        expected@ is None ==> final(guided).guide@ is None,
{
    let ghost old_prefix = guided.cursor.prefix@;
    proof {
        if let Some(d) = expected@ {
            reveal(doc_parts);
            reveal(doc_prefix_parts);
            reveal(doc_line_parts);
            reveal(ckc_spec::v1text::wf_doc);
            reveal_strlit(
                ".pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.\n",
            );
            reveal(ckc_spec::v1text::ascii);
        }
    }
    let percent_space: &[u8] = b"% ";
    proof {
        reveal_strlit("% ");
        reveal_byteslit(b"% ");
        reveal(ckc_spec::v1text::ascii);
        assert(percent_space@ == ckc_spec::v1text::ascii("% "@));
    }
    if !guided_literal(
        bytes,
        guided,
        percent_space,
        Ghost(ckc_spec::v1text::ascii("% "@)),
    at,
    ) {
        return None;
    }
    let ghost docid_expected = match expected@ {
        Some(d) => Some(d.docid),
        None => None,
    };
    let docid = match guided_name(
        bytes,
        guided,
        Ghost(0x2eu8),
        Ghost(docid_expected),
    at,
    ) {
        Some(field) => field,
        None => return None,
    };
    let suffix: &[u8] = b".pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.\n";
    proof {
        reveal_strlit(
            ".pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.\n",
        );
        reveal_byteslit(
            b".pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.\n",
        );
        reveal(ckc_spec::v1text::ascii);
        assert(suffix@ == ckc_spec::v1text::ascii(
            ".pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.\n"@,
        ));
    }
    if !guided_literal(
        bytes,
        guided,
        suffix,
        Ghost(ckc_spec::v1text::ascii(
            ".pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.\n"@,
        )),
    at,
    ) {
        return None;
    }
    proof {
        reveal(doc_line_stage);
        reveal(ckc_spec::v1text::doc_line1);
        assert_seqs_equal!(guided.cursor.prefix@
            == old_prefix + doc_line_stage(docid.value@));
    }
    Some(docid)
}

#[verifier::rlimit(5000)]
#[verifier::spinoff_prover]
fn parse_doc_declarations(
    bytes: &[u8],
    guided: &mut EGuidedCursor,
    expected: Ghost<Option<ckc_spec::v1text::DocFile>>,
at: &mut usize,
) -> (r: bool)
    requires
        *old(at) <= bytes@.len(),
        guided_cursor_ok(bytes@, old(guided)),
        expected@ matches Some(d) ==> old(guided).guide@ matches Some(g)
            && g.parts == doc_parts(d) && g.index == 3
            && ckc_spec::v1text::wf_doc(d),
        expected@ is None ==> old(guided).guide@ is None,
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r ==> {
            &&& guided_cursor_ok(bytes@, final(guided))
            &&& final(guided).cursor.prefix@
                == old(guided).cursor.prefix@
                    + ckc_spec::v1text::decls_from(0)
                    + ckc_spec::v1text::term_line(
                        ckc_spec::v1text::schema_version_term(),
                    )
        },
        expected@ matches Some(d) ==> {
            &&& r
            &&& final(guided).guide@ matches Some(g)
                && g.parts == doc_parts(d) && g.index == 5
        },
        expected@ is None ==> final(guided).guide@ is None,
{
    let ghost old_prefix = guided.cursor.prefix@;
    let declarations: &[u8] = b":- multifile(guideline_schema_version/1).\n:- discontiguous(guideline_schema_version/1).\n:- multifile(guideline_document/3).\n:- discontiguous(guideline_document/3).\n:- multifile(guideline_entity/4).\n:- discontiguous(guideline_entity/4).\n:- multifile(guideline_cardinality/5).\n:- discontiguous(guideline_cardinality/5).\n:- multifile(guideline_event/3).\n:- discontiguous(guideline_event/3).\n:- multifile(guideline_arg/4).\n:- discontiguous(guideline_arg/4).\n:- multifile(guideline_pp/4).\n:- discontiguous(guideline_pp/4).\n:- multifile(guideline_property/4).\n:- discontiguous(guideline_property/4).\n:- multifile(guideline_operator/3).\n:- discontiguous(guideline_operator/3).\n";
    proof {
        reveal_byteslit(b":- multifile(guideline_schema_version/1).\n:- discontiguous(guideline_schema_version/1).\n:- multifile(guideline_document/3).\n:- discontiguous(guideline_document/3).\n:- multifile(guideline_entity/4).\n:- discontiguous(guideline_entity/4).\n:- multifile(guideline_cardinality/5).\n:- discontiguous(guideline_cardinality/5).\n:- multifile(guideline_event/3).\n:- discontiguous(guideline_event/3).\n:- multifile(guideline_arg/4).\n:- discontiguous(guideline_arg/4).\n:- multifile(guideline_pp/4).\n:- discontiguous(guideline_pp/4).\n:- multifile(guideline_property/4).\n:- discontiguous(guideline_property/4).\n:- multifile(guideline_operator/3).\n:- discontiguous(guideline_operator/3).\n");
        reveal_strlit(":- multifile(");
        reveal_strlit(").\n");
        reveal_strlit(":- discontiguous(");
        reveal_strlit("guideline_schema_version");
        reveal_strlit("guideline_document");
        reveal_strlit("guideline_entity");
        reveal_strlit("guideline_cardinality");
        reveal_strlit("guideline_event");
        reveal_strlit("guideline_arg");
        reveal_strlit("guideline_pp");
        reveal_strlit("guideline_property");
        reveal_strlit("guideline_operator");
        reveal(ckc_spec::v1text::ascii);
        reveal(ckc_spec::v1text::indicator);
        reveal(ckc_spec::v1text::decl_pair);
        reveal_with_fuel(ckc_spec::v1text::decls_from, 11);
        reveal(ckc_spec::v1text::digit_byte);
        reveal_with_fuel(ckc_spec::v1text::udec_bytes, 2);
        assert(declarations@ == ckc_spec::v1text::decls_from(0));
        if let Some(d) = expected@ {
            reveal(doc_parts);
            reveal(doc_prefix_parts);
            reveal(doc_line_parts);
            reveal(doc_decl_parts);
            reveal(ckc_spec::v1text::wf_doc);
        }
    }
    if !guided_literal(
        bytes,
        guided,
        declarations,
        Ghost(ckc_spec::v1text::decls_from(0)),
    at,
    ) {
        return false;
    }

    let schema: &[u8] = b"guideline_schema_version(1).\n";
    proof {
        let name = ckc_spec::v1text::ascii("guideline_schema_version"@);
        let one = Term::Int(1);
        reveal_byteslit(b"guideline_schema_version(1).\n");
        reveal_strlit("guideline_schema_version");
        reveal_strlit("guideline_schema_version(1).\n");
        reveal_strlit(".\n");
        reveal(ckc_spec::v1text::ascii);
        reveal(ckc_spec::v1text::curly_name);
        assert(name != ckc_spec::v1text::curly_name());
        schema_version_name_bare();
        args_one_bytes(one);
        regular_comp_bytes(name, seq![one]);
        reveal(ckc_spec::v1text::schema_version_term);
        reveal(ckc_spec::v1text::term_line);
        reveal_with_fuel(ckc_spec::v1text::term_bytes, 1);
        reveal(ckc_spec::v1text::dec_bytes);
        reveal_with_fuel(ckc_spec::v1text::udec_bytes, 2);
        reveal(ckc_spec::v1text::digit_byte);
        assert(schema@ == ckc_spec::v1text::term_line(
            ckc_spec::v1text::schema_version_term(),
        ));
    }
    if !guided_literal(
        bytes,
        guided,
        schema,
        Ghost(ckc_spec::v1text::term_line(
            ckc_spec::v1text::schema_version_term(),
        )),
    at,
    ) {
        return false;
    }
    proof {
        assert_seqs_equal!(guided.cursor.prefix@
            == old_prefix
                + ckc_spec::v1text::decls_from(0)
                + ckc_spec::v1text::term_line(
                    ckc_spec::v1text::schema_version_term(),
                ));
    }
    true
}

#[verifier::rlimit(1000)]
#[verifier::spinoff_prover]
fn parse_doc_record_prefix(
    bytes: &[u8],
    guided: &mut EGuidedCursor,
    line_docid: &ENameField,
    expected: Ghost<Option<ckc_spec::v1text::DocFile>>,
at: &mut usize,
) -> (r: Option<EParsedAtom>)
    requires
        *old(at) <= bytes@.len(),
        guided_cursor_ok(bytes@, old(guided)),
        ckc_spec::v1text::name_ok(line_docid.value@),
        expected@ matches Some(d) ==> {
            &&& line_docid.value@ == d.docid
            &&& old(guided).guide@ matches Some(g)
                && g.parts == doc_parts(d) && g.index == 5
            &&& ckc_spec::v1text::wf_doc(d)
        },
        expected@ is None ==> old(guided).guide@ is None,
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r matches Some(ace) ==> {
            &&& guided_cursor_ok(bytes@, final(guided))
            &&& ckc_spec::v1text::hex64(ace.name@)
            &&& final(guided).cursor.prefix@
                == old(guided).cursor.prefix@
                    + doc_record_head_stage(line_docid.value@)
                    + query_record_ace_stage(ace.name@)
                    + query_record_ulex_open_stage()
        },
        expected@ matches Some(d) ==> {
            &&& r matches Some(ace) && ace.name@ == d.ace
            &&& final(guided).guide@ matches Some(g)
                && g.parts == doc_parts(d) && g.index == 16
        },
        expected@ is None ==> final(guided).guide@ is None,
{
    let ghost old_prefix = guided.cursor.prefix@;
    proof {
        if let Some(d) = expected@ {
            reveal(doc_parts);
            reveal(doc_prefix_parts);
            reveal(doc_line_parts);
            reveal(doc_decl_parts);
            reveal(doc_record_parts);
            reveal(ckc_spec::v1text::wf_doc);
            match d.ulex {
                None => {},
                Some(_) => {},
            }
        }
    }
    let ghost wrapper_expected = match expected@ {
        Some(_) => Some((
            ckc_spec::v1text::ascii("guideline_document"@),
            0x28u8,
        )),
        None => None,
    };
    let wrapper = match guided_atom(bytes, guided, Ghost(wrapper_expected), at) {
        Some(atom) => atom,
        None => return None,
    };
    let wrapper_name: &[u8] = b"guideline_document";
    proof {
        reveal_strlit("guideline_document");
        reveal_byteslit(b"guideline_document");
        reveal(ckc_spec::v1text::ascii);
        assert(wrapper_name@
            == ckc_spec::v1text::ascii("guideline_document"@));
    }
    if !vec_slice_equal(&wrapper.name, wrapper_name) {
        return None;
    }
    if !guided_byte(bytes, guided, 0x28, at) {
        return None;
    }

    let ghost record_docid_expected = match expected@ {
        Some(d) => Some((d.docid, 0x2cu8)),
        None => None,
    };
    let record_docid = match guided_atom(
        bytes,
        guided,
        Ghost(record_docid_expected),
    at,
    ) {
        Some(atom) => atom,
        None => return None,
    };
    if !vec_equal(&line_docid.value, &record_docid.name) {
        return None;
    }
    if !guided_byte(bytes, guided, 0x2c, at) {
        return None;
    }

    let ghost ace_wrapper_expected = match expected@ {
        Some(_) => Some((
            ckc_spec::v1text::ascii("ace_sha256"@),
            0x28u8,
        )),
        None => None,
    };
    let ace_wrapper = match guided_atom(
        bytes,
        guided,
        Ghost(ace_wrapper_expected),
    at,
    ) {
        Some(atom) => atom,
        None => return None,
    };
    let ace_wrapper_name: &[u8] = b"ace_sha256";
    proof {
        reveal_strlit("ace_sha256");
        reveal_byteslit(b"ace_sha256");
        reveal(ckc_spec::v1text::ascii);
        assert(ace_wrapper_name@
            == ckc_spec::v1text::ascii("ace_sha256"@));
    }
    if !vec_slice_equal(&ace_wrapper.name, ace_wrapper_name) {
        return None;
    }
    if !guided_byte(bytes, guided, 0x28, at) {
        return None;
    }

    let ghost ace_expected = match expected@ {
        Some(d) => Some((d.ace, 0x29u8)),
        None => None,
    };
    let ace = match guided_atom(bytes, guided, Ghost(ace_expected), at) {
        Some(atom) => atom,
        None => return None,
    };
    proof {
        if let Some(d) = expected@ {
            assert(ace.name@ == d.ace);
            assert(ckc_spec::v1text::hex64(ace.name@));
        }
    }
    if !hex64_exec(&ace.name) {
        return None;
    }
    if !guided_byte(bytes, guided, 0x29, at) {
        return None;
    }
    if !guided_byte(bytes, guided, 0x2c, at) {
        return None;
    }

    let ghost ulex_wrapper_expected = match expected@ {
        Some(_) => Some((ckc_spec::v1text::ascii("ulex"@), 0x28u8)),
        None => None,
    };
    let ulex_wrapper = match guided_atom(
        bytes,
        guided,
        Ghost(ulex_wrapper_expected),
    at,
    ) {
        Some(atom) => atom,
        None => return None,
    };
    let ulex_wrapper_name: &[u8] = b"ulex";
    proof {
        reveal_strlit("ulex");
        reveal_byteslit(b"ulex");
        reveal(ckc_spec::v1text::ascii);
        assert(ulex_wrapper_name@ == ckc_spec::v1text::ascii("ulex"@));
    }
    if !vec_slice_equal(&ulex_wrapper.name, ulex_wrapper_name) {
        return None;
    }
    if !guided_byte(bytes, guided, 0x28, at) {
        return None;
    }
    proof {
        reveal(doc_record_head_stage);
        reveal(query_record_ace_stage);
        reveal(query_record_ulex_open_stage);
        assert_seqs_equal!(guided.cursor.prefix@
            == old_prefix
                + doc_record_head_stage(line_docid.value@)
                + query_record_ace_stage(ace.name@)
                + query_record_ulex_open_stage());
    }
    Some(ace)
}

#[verifier::rlimit(1000)]
#[verifier::spinoff_prover]
fn parse_doc_ulex(
    bytes: &[u8],
    guided: &mut EGuidedCursor,
    expected: Ghost<Option<ckc_spec::v1text::DocFile>>,
at: &mut usize,
) -> (r: Option<EUlexField>)
    requires
        *old(at) <= bytes@.len(),
        guided_cursor_ok(bytes@, old(guided)),
        expected@ matches Some(d) ==> old(guided).guide@ matches Some(g)
            && g.parts == doc_parts(d) && g.index == 16
            && ckc_spec::v1text::wf_doc(d),
        expected@ is None ==> old(guided).guide@ is None,
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r matches Some(ulex) ==> {
            &&& guided_cursor_ok(bytes@, final(guided))
            &&& ckc_spec::v1text::ulex_ok(ulex.value@)
            &&& final(guided).cursor.prefix@
                == old(guided).cursor.prefix@
                    + query_ulex_stage(ulex.value@)
                    + doc_record_suffix_stage()
        },
        expected@ matches Some(d) ==> {
            &&& r matches Some(ulex) && ulex.value@ == d.ulex
            &&& final(guided).guide@ matches Some(g)
                && g.parts == doc_parts(d)
                && g.index == query_record_end(d.ulex)
                && guide_rest(g) == doc_bundles_parts(d.bundles)
        },
        expected@ is None ==> final(guided).guide@ is None,
{
    let ghost tail = match expected@ {
        Some(d) => doc_bundles_parts(d.bundles),
        None => Seq::<Seq<u8>>::empty(),
    };
    let ghost generic_expected = match expected@ {
        Some(d) => Some((d.ulex, tail)),
        None => None,
    };
    proof {
        if let Some(d) = expected@ {
            reveal(ckc_spec::v1text::wf_doc);
            reveal(doc_parts);
            reveal(doc_prefix_parts);
            reveal(doc_line_parts);
            reveal(doc_decl_parts);
            reveal(doc_record_parts);
            reveal(record_ulex_parts);
            match d.ulex {
                None => {},
                Some(_) => {},
            }
            reveal(guide_rest);
            assert(guide_rest(guided.guide@.unwrap())
                == record_ulex_parts(d.ulex) + tail);
        }
    }
    let ulex = match parse_record_ulex(
        bytes,
        guided,
        Ghost(generic_expected),
    at,
    ) {
        Some(ulex) => ulex,
        None => return None,
    };
    proof {
        reveal(doc_record_suffix_stage);
        reveal(query_record_suffix_stage);
        if let Some(d) = expected@ {
            reveal(record_ulex_parts);
            reveal(query_record_end);
            match d.ulex {
                None => {},
                Some(_) => {},
            }
        }
    }
    Some(ulex)
}

pub open spec fn guide_rest(g: GPartsGuide) -> Seq<Seq<u8>> {
    g.parts.subrange(g.index, g.parts.len() as int)
}

proof fn guide_rest_advance(before: GPartsGuide, after: GPartsGuide)
    requires
        0 <= before.index < before.parts.len(),
        after.parts == before.parts,
        after.index == before.index + 1,
    ensures guide_rest(after) == guide_rest(before).drop_first(),
{
    reveal(guide_rest);
    assert_seqs_equal!(
        before.parts.subrange(before.index + 1, before.parts.len() as int)
            == before.parts.subrange(before.index, before.parts.len() as int).drop_first()
    );
}

proof fn guide_rest_head(
    g: GPartsGuide,
    part: Seq<u8>,
    tail: Seq<Seq<u8>>,
)
    requires
        0 <= g.index <= g.parts.len(),
        guide_rest(g) == seq![part] + tail,
    ensures
        g.index < g.parts.len(),
        g.parts[g.index] == part,
{
    reveal(guide_rest);
    assert(g.parts.subrange(g.index, g.parts.len() as int).len()
        == g.parts.len() as int - g.index);
    assert(guide_rest(g).len() == 1 + tail.len());
    assert(g.index < g.parts.len());
    assert(g.parts.subrange(g.index, g.parts.len() as int)[0]
        == g.parts[g.index]);
    assert((seq![part] + tail)[0] == part);
}

proof fn guide_rest_two(
    g: GPartsGuide,
    first: Seq<u8>,
    rest: Seq<Seq<u8>>,
)
    requires
        0 <= g.index <= g.parts.len(),
        guide_rest(g) == seq![first] + rest,
        rest.len() > 0,
    ensures
        g.index + 1 < g.parts.len(),
        g.parts[g.index] == first,
        g.parts[g.index + 1] == rest[0],
{
    reveal(guide_rest);
    assert(g.parts.subrange(g.index, g.parts.len() as int).len()
        == g.parts.len() as int - g.index);
    assert(guide_rest(g).len() == 1 + rest.len());
    assert(guide_rest(g).len() >= 2);
    assert(g.index + 1 < g.parts.len());
    assert(g.parts.subrange(g.index, g.parts.len() as int)[0]
        == g.parts[g.index]);
    assert(g.parts.subrange(g.index, g.parts.len() as int)[1]
        == g.parts[g.index + 1]);
    assert((seq![first] + rest)[0] == first);
    assert((seq![first] + rest)[1] == rest[0]);
}

proof fn guide_front_bytes(
    bytes: Seq<u8>,
    guided: &EGuidedCursor,
    front: Seq<u8>,
    tail: Seq<Seq<u8>>,
)
    requires
        guided_cursor_ok(bytes, guided),
        guided.guide@ matches Some(g)
            && guide_rest(g) == seq![front] + tail,
    ensures
        guided.cursor.pos as int + front.len() <= bytes.len(),
        bytes.subrange(
            guided.cursor.pos as int,
            guided.cursor.pos as int + front.len(),
        ) == front,
{
    reveal(guided_cursor_ok);
    reveal(parts_progress);
    if let Some(g) = guided.guide@ {
        guide_rest_head(g, front, tail);
        parts_part_ready(bytes, g.parts, g.index, &guided.cursor);
    }
}

fn doc_guided_literal(
    bytes: &[u8],
    guided: &mut EGuidedCursor,
    lit: &[u8],
    chunk: Ghost<Seq<u8>>,
    expected_rest: Ghost<Option<Seq<Seq<u8>>>>,
at: &mut usize,
) -> (r: bool)
    requires
        *old(at) <= bytes@.len(),
        guided_cursor_ok(bytes@, old(guided)),
        lit@ == chunk@,
        expected_rest@ matches Some(rest) ==> old(guided).guide@ matches Some(g)
            && guide_rest(g) == seq![chunk@] + rest,
        expected_rest@ is None ==> old(guided).guide@ is None,
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r ==> guided_cursor_ok(bytes@, final(guided)),
        r ==> final(guided).cursor.pos
            == old(guided).cursor.pos + chunk@.len(),
        r ==> final(guided).cursor.prefix@
            == old(guided).cursor.prefix@ + chunk@,
        expected_rest@ matches Some(rest) ==> {
            &&& r
            &&& old(guided).guide@ matches Some(before)
            &&& final(guided).guide@ matches Some(after)
                && after.parts == before.parts
                && after.index == before.index + 1
                && guide_rest(after) == rest
        },
        expected_rest@ is None ==> final(guided).guide@ is None,
{
    let ghost old_guide = guided.guide@;
    proof {
        if let (Some(rest), Some(g)) = (expected_rest@, old_guide) {
            reveal(guided_cursor_ok);
            reveal(parts_progress);
            guide_rest_head(g, chunk@, rest);
        }
    }
    let accepted = guided_literal(bytes, guided, lit, chunk, at);
    if !accepted {
        return false;
    }
    proof {
        if let (Some(rest), Some(before), Some(after)) =
            (expected_rest@, old_guide, guided.guide@)
        {
            guide_rest_advance(before, after);
            assert(guide_rest(before).drop_first() == rest) by {
                assert(guide_rest(before) == seq![chunk@] + rest);
            }
        }
    }
    true
}

fn parts_guided_atom(
    bytes: &[u8],
    guided: &mut EGuidedCursor,
    next: Ghost<u8>,
    expected: Ghost<Option<(Seq<u8>, Seq<Seq<u8>>)>>,
at: &mut usize,
) -> (r: Option<EParsedAtom>)
    requires
        *old(at) <= bytes@.len(),
        guided_cursor_ok(bytes@, old(guided)),
        expected@ matches Some(e) ==> {
            &&& old(guided).guide@ matches Some(g)
                && guide_rest(g)
                    == seq![ckc_spec::v1text::atom_bytes(e.0)] + e.1
            &&& e.1.len() > 0
            &&& e.1[0].len() > 0
            &&& e.1[0][0] == next@
            &&& next@ == 0x28 || next@ == 0x2c || next@ == 0x29
        },
        expected@ is None ==> old(guided).guide@ is None,
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r matches Some(atom) ==> {
            &&& guided_cursor_ok(bytes@, final(guided))
            &&& parsed_atom_ok(bytes@, old(guided).cursor.pos, &atom)
            &&& final(guided).cursor.prefix@
                == old(guided).cursor.prefix@
                    + ckc_spec::v1text::atom_bytes(atom.name@)
        },
        expected@ matches Some(e) ==> {
            &&& r matches Some(atom) && atom.name@ == e.0
            &&& old(guided).guide@ matches Some(before)
            &&& final(guided).guide@ matches Some(after)
                && after.parts == before.parts
                && after.index == before.index + 1
                && guide_rest(after) == e.1
        },
        expected@ is None ==> final(guided).guide@ is None,
{
    let ghost old_guide = guided.guide@;
    proof {
        if let (Some(e), Some(g)) = (expected@, old_guide) {
            guide_rest_two(
                g,
                ckc_spec::v1text::atom_bytes(e.0),
                e.1,
            );
        }
    }
    let ghost atom_expected = match expected@ {
        Some(e) => Some((e.0, next@)),
        None => None,
    };
    let atom = match guided_atom(bytes, guided, Ghost(atom_expected), at) {
        Some(atom) => atom,
        None => return None,
    };
    proof {
        if let (Some(e), Some(before), Some(after)) =
            (expected@, old_guide, guided.guide@)
        {
            guide_rest_advance(before, after);
            assert(guide_rest(before).drop_first() == e.1) by {
                assert(guide_rest(before)
                    == seq![ckc_spec::v1text::atom_bytes(e.0)] + e.1);
            }
        }
    }
    Some(atom)
}

fn doc_guided_term(
    bytes: &[u8],
    guided: &mut EGuidedCursor,
    next: Ghost<u8>,
    expected: Ghost<Option<(Term, Seq<Seq<u8>>)>>,
at: &mut usize,
) -> (r: Option<ESpannedTerm>)
    requires
        *old(at) <= bytes@.len(),
        guided_cursor_ok(bytes@, old(guided)),
        expected@ matches Some(e) ==> old(guided).guide@ matches Some(g)
            && guide_rest(g)
                == seq![ckc_spec::v1text::term_bytes(e.0)] + e.1
            && ckc_spec::term::wf_term(e.0)
            && e.1.len() > 0
            && e.1[0].len() > 0
            && e.1[0][0] == next@
            && (next@ == 0x2c || next@ == 0x29 || next@ == 0x5d
                || next@ == 0x7c || next@ == 0x7d || next@ == 0x20
                || next@ == 0x2e && e.1[0].len() > 1
                    && e.1[0][1] == 0x0a),
        expected@ is None ==> old(guided).guide@ is None,
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r matches Some(term) ==> {
            &&& guided_cursor_ok(bytes@, final(guided))
            &&& spanned_term_ok(bytes@, &term)
            &&& term.start == old(guided).cursor.pos
            &&& final(guided).cursor.pos == term.end
            &&& final(guided).cursor.prefix@
                == old(guided).cursor.prefix@
                    + ckc_spec::v1text::term_bytes(term@)
        },
        expected@ matches Some(e) ==> {
            &&& r matches Some(term) && term@ == e.0
            &&& final(guided).guide@ matches Some(g)
                && guide_rest(g) == e.1
            &&& final(guided).cursor.pos < bytes@.len()
            &&& bytes@[final(guided).cursor.pos as int] == next@
            &&& next@ == 0x2e ==> {
                &&& final(guided).cursor.pos + 1 < bytes@.len()
                &&& bytes@[final(guided).cursor.pos as int + 1] == 0x0a
            }
        },
        expected@ is None ==> final(guided).guide@ is None,
{
    let ghost old_guide = guided.guide@;
    proof {
        if let (Some(e), Some(g)) = (expected@, old_guide) {
            reveal(guided_cursor_ok);
            reveal(parts_progress);
            guide_rest_two(
                g,
                ckc_spec::v1text::term_bytes(e.0),
                e.1,
            );
        }
    }
    let ghost term_expected = match expected@ {
        Some(e) => Some((e.0, next@)),
        None => None,
    };
    let term = match guided_term(bytes, guided, Ghost(term_expected), at) {
        Some(term) => term,
        None => return None,
    };
    proof {
        if let (Some(e), Some(before), Some(after)) =
            (expected@, old_guide, guided.guide@)
        {
            guide_rest_advance(before, after);
            assert(guide_rest(before).drop_first() == e.1) by {
                assert(guide_rest(before)
                    == seq![ckc_spec::v1text::term_bytes(e.0)] + e.1);
            }
            assert_seqs_equal!(e.1 == seq![e.1[0]] + e.1.drop_first());
            guide_front_bytes(bytes@, &guided, e.1[0], e.1.drop_first());
            assert(bytes@.subrange(
                guided.cursor.pos as int,
                guided.cursor.pos as int + e.1[0].len(),
            )[0] == bytes@[guided.cursor.pos as int]);
            assert(bytes@[guided.cursor.pos as int] == next@);
            if next@ == 0x2e {
                assert(bytes@.subrange(
                    guided.cursor.pos as int,
                    guided.cursor.pos as int + e.1[0].len(),
                )[1] == bytes@[guided.cursor.pos as int + 1]);
                assert(bytes@[guided.cursor.pos as int + 1] == 0x0a);
            }
        }
    }
    Some(term)
}

fn semantic_pred_exec(name: &Vec<u8>, arity: usize) -> (r: bool)
    ensures r == ckc_spec::v1text::is_semantic_pred(name@, arity as nat),
{
    let entity_name: &[u8] = b"guideline_entity";
    let cardinality_name: &[u8] = b"guideline_cardinality";
    let event_name: &[u8] = b"guideline_event";
    let arg_name: &[u8] = b"guideline_arg";
    let pp_name: &[u8] = b"guideline_pp";
    let property_name: &[u8] = b"guideline_property";
    let operator_name: &[u8] = b"guideline_operator";
    proof {
        reveal_strlit("guideline_entity");
        reveal_strlit("guideline_cardinality");
        reveal_strlit("guideline_event");
        reveal_strlit("guideline_arg");
        reveal_strlit("guideline_pp");
        reveal_strlit("guideline_property");
        reveal_strlit("guideline_operator");
        reveal_byteslit(b"guideline_entity");
        reveal_byteslit(b"guideline_cardinality");
        reveal_byteslit(b"guideline_event");
        reveal_byteslit(b"guideline_arg");
        reveal_byteslit(b"guideline_pp");
        reveal_byteslit(b"guideline_property");
        reveal_byteslit(b"guideline_operator");
        reveal(ckc_spec::v1text::ascii);
        assert(entity_name@
            == ckc_spec::v1text::ascii("guideline_entity"@));
        assert(cardinality_name@
            == ckc_spec::v1text::ascii("guideline_cardinality"@));
        assert(event_name@
            == ckc_spec::v1text::ascii("guideline_event"@));
        assert(arg_name@ == ckc_spec::v1text::ascii("guideline_arg"@));
        assert(pp_name@ == ckc_spec::v1text::ascii("guideline_pp"@));
        assert(property_name@
            == ckc_spec::v1text::ascii("guideline_property"@));
        assert(operator_name@
            == ckc_spec::v1text::ascii("guideline_operator"@));
    }
    let entity = vec_slice_equal(name, entity_name);
    let cardinality = vec_slice_equal(name, cardinality_name);
    let event = vec_slice_equal(name, event_name);
    let arg = vec_slice_equal(name, arg_name);
    let pp = vec_slice_equal(name, pp_name);
    let property = vec_slice_equal(name, property_name);
    let operator = vec_slice_equal(name, operator_name);
    let accepted = entity && arity == 4
        || cardinality && arity == 5
        || event && arity == 3
        || arg && arity == 4
        || pp && arity == 4
        || property && arity == 4
        || operator && arity == 3;
    proof {
        reveal(ckc_spec::v1text::is_semantic_pred);
        reveal(ckc_spec::v1text::indicator);
        if accepted {
            if entity && arity == 4 {
                assert(exists|i: int| 2 <= i < 9
                    && ckc_spec::v1text::indicator(i)
                        == (name@, arity as nat)) by {
                    assert(ckc_spec::v1text::indicator(2)
                        == (name@, arity as nat));
                }
            } else if cardinality && arity == 5 {
                assert(exists|i: int| 2 <= i < 9
                    && ckc_spec::v1text::indicator(i)
                        == (name@, arity as nat)) by {
                    assert(ckc_spec::v1text::indicator(3)
                        == (name@, arity as nat));
                }
            } else if event && arity == 3 {
                assert(exists|i: int| 2 <= i < 9
                    && ckc_spec::v1text::indicator(i)
                        == (name@, arity as nat)) by {
                    assert(ckc_spec::v1text::indicator(4)
                        == (name@, arity as nat));
                }
            } else if arg && arity == 4 {
                assert(exists|i: int| 2 <= i < 9
                    && ckc_spec::v1text::indicator(i)
                        == (name@, arity as nat)) by {
                    assert(ckc_spec::v1text::indicator(5)
                        == (name@, arity as nat));
                }
            } else if pp && arity == 4 {
                assert(exists|i: int| 2 <= i < 9
                    && ckc_spec::v1text::indicator(i)
                        == (name@, arity as nat)) by {
                    assert(ckc_spec::v1text::indicator(6)
                        == (name@, arity as nat));
                }
            } else if property && arity == 4 {
                assert(exists|i: int| 2 <= i < 9
                    && ckc_spec::v1text::indicator(i)
                        == (name@, arity as nat)) by {
                    assert(ckc_spec::v1text::indicator(7)
                        == (name@, arity as nat));
                }
            } else {
                assert(operator && arity == 3);
                assert(exists|i: int| 2 <= i < 9
                    && ckc_spec::v1text::indicator(i)
                        == (name@, arity as nat)) by {
                    assert(ckc_spec::v1text::indicator(8)
                        == (name@, arity as nat));
                }
            }
        } else {
            assert forall|i: int| 2 <= i < 9 implies
                ckc_spec::v1text::indicator(i)
                    != (name@, arity as nat) by {
                if 2 <= i < 9 {
                    if i == 2 {
                        assert(!entity || arity != 4);
                    } else if i == 3 {
                        assert(!cardinality || arity != 5);
                    } else if i == 4 {
                        assert(!event || arity != 3);
                    } else if i == 5 {
                        assert(!arg || arity != 4);
                    } else if i == 6 {
                        assert(!pp || arity != 4);
                    } else if i == 7 {
                        assert(!property || arity != 4);
                    } else {
                        assert(i == 8);
                        assert(!operator || arity != 3);
                    }
                }
            }
        }
    }
    accepted
}

fn semantic_literal_exec(term: &EParsedTerm) -> (r: bool)
    requires parsed_term_ok(term),
    ensures r == ckc_spec::v1text::wf_literal(term@),
{
    let semantic = match &term.top {
        ETermTop::Comp(name, arity) => semantic_pred_exec(name, *arity),
        _ => false,
    };
    let accepted = semantic && term.no_dollar;
    proof {
        reveal(parsed_term_ok);
        reveal(top_matches);
        reveal(ckc_spec::v1text::wf_literal);
        reveal(ckc_spec::term::wf_term);
        reveal(ckc_spec::term::no_dollar_var);
        match term@ {
            Term::Comp(_, _) => {},
            _ => {},
        }
    }
    accepted
}

proof fn spanned_term_at_doc_delimiter(
    bytes: Seq<u8>,
    term: &ESpannedTerm,
)
    requires
        spanned_term_ok(bytes, term),
        term.end < bytes.len(),
        bytes[term.end as int] == 0x2c
            || bytes[term.end as int] == 0x29
            || bytes[term.end as int] == 0x20
            || bytes[term.end as int] == 0x2e
                && term.end + 1 < bytes.len()
                && bytes[term.end as int + 1] == 0x0a,
    ensures term_at(bytes, term.start as int, term.end as int, term@),
{
    reveal(spanned_term_ok);
    reveal(parsed_term_ok);
    reveal(term_at);
    reveal(term_boundary);
}

fn parse_doc_literal(
    bytes: &[u8],
    guided: &mut EGuidedCursor,
    next: Ghost<u8>,
    expected: Ghost<Option<(Term, Seq<Seq<u8>>)>>,
    tracker: &mut EVarTracker,
at: &mut usize,
) -> (r: Option<ESpannedTerm>)
    requires
        *old(at) <= bytes@.len(),
        guided_cursor_ok(bytes@, old(guided)),
        expected@ matches Some(e) ==> old(guided).guide@ matches Some(g)
            && guide_rest(g)
                == seq![ckc_spec::v1text::term_bytes(e.0)] + e.1
            && ckc_spec::v1text::wf_literal(e.0)
            && e.1.len() > 0
            && e.1[0].len() > 0
            && e.1[0][0] == next@
            && (next@ == 0x2c || next@ == 0x29 || next@ == 0x20
                || next@ == 0x2e && e.1[0].len() > 1
                    && e.1[0][1] == 0x0a),
        expected@ is None ==> old(guided).guide@ is None,
        old(tracker).valid ==>
            tracker_state_ok(old(tracker).next, old(tracker).stream@),
        tracker_complete(old(tracker).valid, old(tracker).stream@),
        old(tracker).stream@.len() <= old(guided).cursor.pos,
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r matches Some(term) ==> {
            &&& guided_cursor_ok(bytes@, final(guided))
            &&& spanned_term_ok(bytes@, &term)
            &&& ckc_spec::v1text::wf_literal(term@)
            &&& final(tracker).stream@
                == old(tracker).stream@ + ckc_spec::term::var_stream(term@)
            &&& final(tracker).valid ==>
                tracker_state_ok(final(tracker).next, final(tracker).stream@)
            &&& tracker_complete(final(tracker).valid, final(tracker).stream@)
            &&& final(tracker).stream@.len() <= term.end
            &&& term.start == old(guided).cursor.pos
            &&& final(guided).cursor.pos == term.end
            &&& final(guided).cursor.prefix@
                == old(guided).cursor.prefix@
                    + ckc_spec::v1text::term_bytes(term@)
            &&& final(guided).cursor.pos < bytes@.len()
            &&& old(guided).cursor.pos < final(guided).cursor.pos
        },
        expected@ matches Some(e) ==> {
            &&& r matches Some(term) && term@ == e.0
            &&& final(guided).guide@ matches Some(g)
                && guide_rest(g) == e.1
            &&& final(guided).cursor.pos < bytes@.len()
            &&& bytes@[final(guided).cursor.pos as int] == next@
            &&& next@ == 0x2e ==> {
                &&& final(guided).cursor.pos + 1 < bytes@.len()
                &&& bytes@[final(guided).cursor.pos as int + 1] == 0x0a
            }
        },
        expected@ is None ==> final(guided).guide@ is None,
{
    let term = match doc_guided_term(bytes, guided, next, expected, at) {
        Some(term) => term,
        None => return None,
    };
    if term.end >= bytes.len() {
        raise_at(at, term.end, bytes.len());
        proof {
            if expected@ is Some {
                assert(false);
            }
        }
        return None;
    }
    if bytes[term.end] == 0x2e {
        if term.end + 1 >= bytes.len() {
            raise_at(at, bytes.len(), bytes.len());
            proof {
                if let Some(e) = expected@ {
                    assert(e.1[0][0] == next@);
                    assert(next@ == 0x2e);
                    assert(false);
                }
            }
            return None;
        }
        if bytes[term.end + 1] != 0x0a {
            raise_at(at, term.end + 1, bytes.len());
            proof {
                if let Some(e) = expected@ {
                    assert(e.1[0][0] == next@);
                    assert(next@ == 0x2e);
                    assert(false);
                }
            }
            return None;
        }
    } else if bytes[term.end] != 0x2c
        && bytes[term.end] != 0x29
        && bytes[term.end] != 0x20
    {
        raise_at(at, term.end, bytes.len());
        proof {
            if expected@ is Some {
                assert(false);
            }
        }
        return None;
    }
    if !semantic_literal_exec(&term.parsed) {
        if term.parsed.no_dollar {
            raise_at(at, term.start, bytes.len());
        }
        proof {
            if let Some(e) = expected@ {
                assert(term@ == e.0);
                assert(false);
            }
        }
        return None;
    }
    proof {
        spanned_term_at_doc_delimiter(bytes@, &term);
    }
    track_parsed_term(bytes, &term, tracker, at);
    Some(term)
}

proof fn semantic_pred_name_first(name: Seq<u8>, arity: nat)
    requires ckc_spec::v1text::is_semantic_pred(name, arity),
    ensures name.len() > 0, name[0] == 0x67,
{
    reveal(ckc_spec::v1text::is_semantic_pred);
    let i = choose|i: int| 2 <= i < 9
        && ckc_spec::v1text::indicator(i) == (name, arity);
    reveal(ckc_spec::v1text::indicator);
    if i == 2 {
        reveal_strlit("guideline_entity");
        reveal(ckc_spec::v1text::ascii);
    } else if i == 3 {
        reveal_strlit("guideline_cardinality");
        reveal(ckc_spec::v1text::ascii);
    } else if i == 4 {
        reveal_strlit("guideline_event");
        reveal(ckc_spec::v1text::ascii);
    } else if i == 5 {
        reveal_strlit("guideline_arg");
        reveal(ckc_spec::v1text::ascii);
    } else if i == 6 {
        reveal_strlit("guideline_pp");
        reveal(ckc_spec::v1text::ascii);
    } else if i == 7 {
        reveal_strlit("guideline_property");
        reveal(ckc_spec::v1text::ascii);
    } else {
        assert(i == 8);
        reveal_strlit("guideline_operator");
        reveal(ckc_spec::v1text::ascii);
    }
}

proof fn semantic_pred_atom_prefix_safe(name: Seq<u8>, arity: nat)
    requires ckc_spec::v1text::is_semantic_pred(name, arity),
    ensures
        ckc_spec::v1text::atom_bytes(name).len() > 0,
        ckc_spec::v1text::atom_bytes(name)[0] != 0x25,
        ckc_spec::v1text::atom_bytes(name)[0] != 0x5c,
        ckc_spec::v1text::atom_bytes(name)[0] != 0x28,
        ckc_spec::v1text::atom_bytes(name)[0] != 0x29,
        ckc_spec::v1text::atom_bytes(name)[0] != 0x2c,
{
    semantic_pred_name_first(name, arity);
    reveal(ckc_spec::v1text::atom_bytes);
}

proof fn wf_literal_term_prefix_safe(term: Term)
    requires ckc_spec::v1text::wf_literal(term),
    ensures
        ckc_spec::v1text::term_bytes(term).len() > 0,
        ckc_spec::v1text::term_bytes(term)[0] != 0x25,
        ckc_spec::v1text::term_bytes(term)[0] != 0x5c,
        ckc_spec::v1text::term_bytes(term)[0] != 0x28,
        ckc_spec::v1text::term_bytes(term)[0] != 0x29,
        ckc_spec::v1text::term_bytes(term)[0] != 0x2c,
{
    reveal(ckc_spec::v1text::wf_literal);
    match term {
        Term::Comp(name, args) => {
            semantic_pred_atom_prefix_safe(name, args.len());
            reveal_with_fuel(ckc_spec::v1text::term_bytes, 1);
        },
        _ => assert(false),
    }
}

pub struct EDocBodyItem {
    pub item: Ghost<ckc_spec::v1text::BodyItem>,
}

impl View for EDocBodyItem {
    type V = ckc_spec::v1text::BodyItem;

    open spec fn view(&self) -> ckc_spec::v1text::BodyItem {
        self.item@
    }
}

#[verifier::rlimit(5000)]
proof fn guide_second_bytes(
    bytes: Seq<u8>,
    guided: &EGuidedCursor,
    first: Seq<u8>,
    second: Seq<u8>,
    tail: Seq<Seq<u8>>,
)
    requires
        guided_cursor_ok(bytes, guided),
        guided.guide@ matches Some(g)
            && guide_rest(g) == seq![first, second] + tail,
    ensures
        guided.cursor.pos as int + first.len() + second.len() <= bytes.len(),
        bytes.subrange(
            guided.cursor.pos as int + first.len(),
            guided.cursor.pos as int + first.len() + second.len(),
        ) == second,
{
    reveal(guided_cursor_ok);
    reveal(parts_progress);
    if let Some(g) = guided.guide@ {
        assert(guide_rest(g) == seq![first] + (seq![second] + tail));
        guide_rest_two(g, first, seq![second] + tail);
        flatten_take_step(g.parts, g.index);
        flattened_part(bytes, g.parts, g.index + 1);
        assert(g.parts.take(g.index).flatten().len()
            == guided.cursor.pos);
        assert(g.parts.take(g.index + 1).flatten().len()
            == guided.cursor.pos + first.len());
    }
}

#[verifier::rlimit(2000)]
#[verifier::spinoff_prover]
fn parse_doc_pos_item(
    bytes: &[u8],
    guided: &mut EGuidedCursor,
    next: Ghost<u8>,
    expected: Ghost<Option<(Term, Seq<Seq<u8>>)>>,
    tracker: &mut EVarTracker,
at: &mut usize,
) -> (r: Option<EDocBodyItem>)
    requires
        *old(at) <= bytes@.len(),
        guided_cursor_ok(bytes@, old(guided)),
        expected@ matches Some(e) ==> {
            &&& old(guided).guide@ matches Some(g)
                && guide_rest(g)
                    == seq![ckc_spec::v1text::term_bytes(e.0)] + e.1
            &&& ckc_spec::v1text::wf_literal(e.0)
            &&& e.1.len() > 0
            &&& e.1[0].len() > 0
            &&& e.1[0][0] == next@
            &&& next@ == 0x2c || next@ == 0x2e
            &&& next@ == 0x2e ==> {
                &&& e.1[0].len() > 1
                &&& e.1[0][1] == 0x0a
            }
        },
        expected@ is None ==> old(guided).guide@ is None,
        old(tracker).valid ==>
            tracker_state_ok(old(tracker).next, old(tracker).stream@),
        tracker_complete(old(tracker).valid, old(tracker).stream@),
        old(tracker).stream@.len() <= old(guided).cursor.pos,
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r matches Some(item) ==> {
            &&& guided_cursor_ok(bytes@, final(guided))
            &&& ckc_spec::v1text::wf_body_item(item@)
            &&& final(tracker).stream@
                == old(tracker).stream@
                    + ckc_spec::v1text::item_var_stream(item@)
            &&& final(tracker).valid ==>
                tracker_state_ok(final(tracker).next, final(tracker).stream@)
            &&& tracker_complete(final(tracker).valid, final(tracker).stream@)
            &&& final(tracker).stream@.len() <= final(guided).cursor.pos
            &&& old(guided).cursor.pos < final(guided).cursor.pos
            &&& final(guided).cursor.prefix@
                == old(guided).cursor.prefix@
                    + doc_body_item_parts(item@).flatten()
        },
        expected@ matches Some(e) ==> {
            &&& r matches Some(item)
                && item@ == ckc_spec::v1text::BodyItem::Pos(e.0)
            &&& final(guided).guide@ matches Some(g)
                && guide_rest(g) == e.1
        },
        expected@ is None ==> final(guided).guide@ is None,
{
    let ghost entry_stream = tracker.stream@;
    let term = match parse_doc_literal(bytes, guided, next, expected, tracker, at) {
        Some(term) => term,
        None => return None,
    };
    let ghost item = ckc_spec::v1text::BodyItem::Pos(term@);
    proof {
        reveal(ckc_spec::v1text::wf_body_item);
        reveal(ckc_spec::v1text::item_var_stream);
        reveal(doc_body_item_parts);
        reveal_with_fuel(Seq::<_>::flatten, 3);
        assert(entry_stream == old(tracker).stream@);
    }
    Some(EDocBodyItem { item: Ghost(item) })
}

#[verifier::rlimit(2000)]
#[verifier::spinoff_prover]
fn parse_doc_naf_singleton(
    bytes: &[u8],
    guided: &mut EGuidedCursor,
    next: Ghost<u8>,
    expected: Ghost<Option<(Term, Seq<Seq<u8>>)>>,
    tracker: &mut EVarTracker,
at: &mut usize,
) -> (r: Option<EDocBodyItem>)
    requires
        *old(at) <= bytes@.len(),
        guided_cursor_ok(bytes@, old(guided)),
        expected@ matches Some(e) ==> {
            &&& old(guided).guide@ matches Some(g)
                && guide_rest(g)
                    == seq![
                        ckc_spec::v1text::ascii("\\+ "@),
                        ckc_spec::v1text::term_bytes(e.0),
                    ] + e.1
            &&& ckc_spec::v1text::wf_literal(e.0)
            &&& e.1.len() > 0
            &&& e.1[0].len() > 0
            &&& e.1[0][0] == next@
            &&& next@ == 0x2c || next@ == 0x2e
            &&& next@ == 0x2e ==> {
                &&& e.1[0].len() > 1
                &&& e.1[0][1] == 0x0a
            }
        },
        expected@ is None ==> old(guided).guide@ is None,
        old(tracker).valid ==>
            tracker_state_ok(old(tracker).next, old(tracker).stream@),
        tracker_complete(old(tracker).valid, old(tracker).stream@),
        old(tracker).stream@.len() <= old(guided).cursor.pos,
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r matches Some(item) ==> {
            &&& guided_cursor_ok(bytes@, final(guided))
            &&& ckc_spec::v1text::wf_body_item(item@)
            &&& final(tracker).stream@
                == old(tracker).stream@
                    + ckc_spec::v1text::item_var_stream(item@)
            &&& final(tracker).valid ==>
                tracker_state_ok(final(tracker).next, final(tracker).stream@)
            &&& tracker_complete(final(tracker).valid, final(tracker).stream@)
            &&& final(tracker).stream@.len() <= final(guided).cursor.pos
            &&& old(guided).cursor.pos < final(guided).cursor.pos
            &&& final(guided).cursor.prefix@
                == old(guided).cursor.prefix@
                    + doc_body_item_parts(item@).flatten()
        },
        expected@ matches Some(e) ==> {
            &&& r matches Some(item)
                && item@
                    == ckc_spec::v1text::BodyItem::Naf(seq![e.0])
            &&& final(guided).guide@ matches Some(g)
                && guide_rest(g) == e.1
        },
        expected@ is None ==> final(guided).guide@ is None,
{
    let ghost entry_stream = tracker.stream@;
    let ghost entry_prefix = guided.cursor.prefix@;
    let prefix: &[u8] = b"\\+ ";
    proof {
        reveal_strlit("\\+ ");
        reveal_byteslit(b"\\+ ");
        reveal(ckc_spec::v1text::ascii);
        assert(prefix@ == ckc_spec::v1text::ascii("\\+ "@));
        if let Some(e) = expected@ {
            assert(guide_rest(guided.guide@.unwrap())
                == seq![ckc_spec::v1text::ascii("\\+ "@)]
                    + (seq![ckc_spec::v1text::term_bytes(e.0)] + e.1));
        }
    }
    let ghost after_prefix = match expected@ {
        Some(e) => Some(seq![ckc_spec::v1text::term_bytes(e.0)] + e.1),
        None => None,
    };
    if !doc_guided_literal(
        bytes,
        guided,
        prefix,
        Ghost(ckc_spec::v1text::ascii("\\+ "@)),
        Ghost(after_prefix),
    at,
    ) {
        return None;
    }
    let term = match parse_doc_literal(bytes, guided, next, expected, tracker, at) {
        Some(term) => term,
        None => return None,
    };
    let ghost item = ckc_spec::v1text::BodyItem::Naf(seq![term@]);
    proof {
        reveal(ckc_spec::v1text::wf_body_item);
        reveal(ckc_spec::v1text::item_var_stream);
        reveal_with_fuel(ckc_spec::term::var_stream_all, 2);
        assert(seq![term@].drop_first() == Seq::<Term>::empty());
        assert(ckc_spec::term::var_stream_all(seq![term@])
            == ckc_spec::term::var_stream(term@));
        assert forall|i: int| 0 <= i < 1
            implies ckc_spec::v1text::wf_literal(seq![term@][i]) by {
            assert(i == 0);
        }
        assert(entry_stream == old(tracker).stream@);
        reveal(doc_body_item_parts);
        reveal_with_fuel(Seq::<_>::flatten, 4);
        assert_seqs_equal!(
            (entry_prefix + ckc_spec::v1text::ascii("\\+ "@))
                + ckc_spec::v1text::term_bytes(term@)
            == entry_prefix
                + (ckc_spec::v1text::ascii("\\+ "@)
                    + ckc_spec::v1text::term_bytes(term@))
        );
        if let Some(e) = expected@ {
            assert(term@ == e.0);
        }
    }
    Some(EDocBodyItem { item: Ghost(item) })
}

#[verifier::rlimit(5000)]
#[verifier::spinoff_prover]
fn parse_doc_naf_conjunction(
    bytes: &[u8],
    guided: &mut EGuidedCursor,
    expected: Ghost<Option<(Seq<Term>, Seq<Seq<u8>>)>>,
    tracker: &mut EVarTracker,
at: &mut usize,
) -> (r: Option<EDocBodyItem>)
    requires
        *old(at) <= bytes@.len(),
        guided_cursor_ok(bytes@, old(guided)),
        expected@ matches Some(e) ==> {
            &&& old(guided).guide@ matches Some(g)
                && guide_rest(g)
                    == doc_body_item_parts(
                        ckc_spec::v1text::BodyItem::Naf(e.0),
                    ) + e.1
            &&& e.0.len() >= 2
            &&& forall|i: int| 0 <= i < e.0.len()
                ==> ckc_spec::v1text::wf_literal(e.0[i])
        },
        expected@ is None ==> old(guided).guide@ is None,
        old(tracker).valid ==>
            tracker_state_ok(old(tracker).next, old(tracker).stream@),
        tracker_complete(old(tracker).valid, old(tracker).stream@),
        old(tracker).stream@.len() <= old(guided).cursor.pos,
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r matches Some(item) ==> {
            &&& guided_cursor_ok(bytes@, final(guided))
            &&& ckc_spec::v1text::wf_body_item(item@)
            &&& final(tracker).stream@
                == old(tracker).stream@
                    + ckc_spec::v1text::item_var_stream(item@)
            &&& final(tracker).valid ==>
                tracker_state_ok(final(tracker).next, final(tracker).stream@)
            &&& tracker_complete(final(tracker).valid, final(tracker).stream@)
            &&& final(tracker).stream@.len() <= final(guided).cursor.pos
            &&& old(guided).cursor.pos < final(guided).cursor.pos
            &&& final(guided).cursor.prefix@
                == old(guided).cursor.prefix@
                    + doc_body_item_parts(item@).flatten()
        },
        expected@ matches Some(e) ==> {
            &&& r matches Some(item)
                && item@ == ckc_spec::v1text::BodyItem::Naf(e.0)
            &&& final(guided).guide@ matches Some(g)
                && guide_rest(g) == e.1
        },
        expected@ is None ==> final(guided).guide@ is None,
{
    let ghost entry_stream = tracker.stream@;
    let ghost entry_prefix = guided.cursor.prefix@;
    let entry_pos = guided.cursor.pos;
    let prefix: &[u8] = b"\\+ ";
    proof {
        reveal_strlit("\\+ ");
        reveal_byteslit(b"\\+ ");
        reveal(ckc_spec::v1text::ascii);
        assert(prefix@ == ckc_spec::v1text::ascii("\\+ "@));
        if let Some(e) = expected@ {
            reveal(doc_body_item_parts);
            assert(guide_rest(guided.guide@.unwrap())
                == seq![ckc_spec::v1text::ascii("\\+ "@)]
                    + (seq![seq![0x28u8]] + doc_lit_parts(e.0)
                        + seq![seq![0x29u8]] + e.1));
        }
    }
    let ghost after_prefix = match expected@ {
        Some(e) => Some(
            seq![seq![0x28u8]] + doc_lit_parts(e.0)
                + seq![seq![0x29u8]] + e.1,
        ),
        None => None,
    };
    if !doc_guided_literal(
        bytes,
        guided,
        prefix,
        Ghost(ckc_spec::v1text::ascii("\\+ "@)),
        Ghost(after_prefix),
    at,
    ) {
        return None;
    }
    proof {
        reveal_strlit("\\+ ");
        reveal(ckc_spec::v1text::ascii);
        assert(ckc_spec::v1text::ascii("\\+ "@).len() == 3);
        assert(guided.cursor.pos == entry_pos + 3);
        assert(entry_pos < guided.cursor.pos);
    }
    let open: &[u8] = b"(";
    let ghost open_chunk = seq![0x28u8];
    proof {
        reveal_byteslit(b"(");
        assert(open@ == open_chunk);
        if let Some(e) = expected@ {
            assert(guide_rest(guided.guide@.unwrap())
                == seq![open_chunk]
                    + (doc_lit_parts(e.0) + seq![seq![0x29u8]] + e.1));
        }
    }
    let ghost after_open = match expected@ {
        Some(e) => Some(doc_lit_parts(e.0) + seq![seq![0x29u8]] + e.1),
        None => None,
    };
    if !doc_guided_literal(
        bytes,
        guided,
        open,
        Ghost(open_chunk),
        Ghost(after_open),
    at,
    ) {
        return None;
    }

    let ghost mut lits: Seq<Term> = Seq::empty();
    let mut lit_count = 0usize;
    let mut more = true;
    proof {
        reveal_strlit("\\+ (");
        reveal(ckc_spec::v1text::ascii);
        reveal_with_fuel(ckc_spec::v1text::lit_list_bytes, 2);
        assert_seqs_equal!(
            ckc_spec::v1text::ascii("\\+ "@) + seq![0x28u8]
                == ckc_spec::v1text::ascii("\\+ ("@)
        );
        assert_seqs_equal!(
            (entry_prefix + ckc_spec::v1text::ascii("\\+ "@))
                + seq![0x28u8]
            == entry_prefix + ckc_spec::v1text::ascii("\\+ ("@)
        );
        reveal_with_fuel(ckc_spec::term::var_stream_all, 2);
        assert(ckc_spec::term::var_stream_all(lits) == Seq::<nat>::empty());
        assert_seqs_equal!(entry_stream + Seq::<nat>::empty() == entry_stream);
        assert(tracker.stream@
            == entry_stream + ckc_spec::term::var_stream_all(lits));
        assert forall|i: int| 0 <= i < lits.len()
            implies ckc_spec::v1text::wf_literal(lits[i]) by {
            assert(false);
        }
        if let Some(e) = expected@ {
            assert_seqs_equal!(e.0.take(0) == Seq::<Term>::empty());
            assert_seqs_equal!(e.0.skip(0) == e.0);
            assert(lits == e.0.take(lits.len() as int));
            assert(guide_rest(guided.guide@.unwrap())
                == doc_lit_parts(e.0.skip(lits.len() as int))
                    + seq![seq![0x29u8]] + e.1);
        }
    }
    while more
        invariant
            *old(at) <= *at <= bytes@.len(),
            guided_cursor_ok(bytes@, &guided),
            entry_pos < guided.cursor.pos,
            lit_count == lits.len(),
            lits.len() <= guided.cursor.pos,
            guided.cursor.prefix@ == if lits.len() == 0 {
                entry_prefix + ckc_spec::v1text::ascii("\\+ ("@)
            } else if more {
                entry_prefix + ckc_spec::v1text::ascii("\\+ ("@)
                    + ckc_spec::v1text::lit_list_bytes(lits)
                    + ckc_spec::v1text::ascii(", "@)
            } else {
                entry_prefix + ckc_spec::v1text::ascii("\\+ ("@)
                    + ckc_spec::v1text::lit_list_bytes(lits)
            },
            tracker.valid ==> tracker_state_ok(tracker.next, tracker.stream@),
            tracker_complete(tracker.valid, tracker.stream@),
            tracker.stream@ == entry_stream
                + ckc_spec::term::var_stream_all(lits),
            tracker.stream@.len() <= guided.cursor.pos,
            forall|i: int| 0 <= i < lits.len()
                ==> ckc_spec::v1text::wf_literal(lits[i]),
            !more ==> lits.len() >= 1,
            expected@ matches Some(e) ==> {
                &&& e.0.len() >= 2
                &&& forall|i: int| 0 <= i < e.0.len()
                    ==> ckc_spec::v1text::wf_literal(e.0[i])
                &&& lits == e.0.take(lits.len() as int)
                &&& lits.len() <= e.0.len()
                &&& more ==> lits.len() < e.0.len()
                &&& !more ==> lits.len() == e.0.len()
                &&& guided.guide@ matches Some(g)
                    && guide_rest(g) == if more {
                        doc_lit_parts(e.0.skip(lits.len() as int))
                            + seq![seq![0x29u8]] + e.1
                    } else {
                        seq![seq![0x29u8]] + e.1
                    }
            },
            expected@ is None ==> guided.guide@ is None,
        decreases if more {
            bytes.len() - guided.cursor.pos + 1
        } else {
            0
        },
    {
        let ghost term_expected = match expected@ {
            Some(e) => {
                let remaining = e.0.skip(lits.len() as int);
                let close_tail = seq![seq![0x29u8]] + e.1;
                let after = if remaining.len() == 1 {
                    close_tail
                } else {
                    seq![ckc_spec::v1text::ascii(", "@)]
                        + doc_lit_parts(remaining.drop_first()) + close_tail
                };
                Some((remaining[0], after))
            },
            None => None,
        };
        let ghost term_next = match expected@ {
            Some(e) => if e.0.len() == lits.len() + 1 {
                0x29u8
            } else {
                0x2cu8
            },
            None => 0x2cu8,
        };
        proof {
            if let Some(e) = expected@ {
                let remaining = e.0.skip(lits.len() as int);
                assert(0 <= lits.len() < e.0.len());
                assert(remaining.len() == e.0.len() - lits.len());
                assert(remaining.len() > 0);
                assert(ckc_spec::v1text::wf_literal(
                    e.0[lits.len() as int],
                ));
                assert(remaining[0] == e.0[lits.len() as int]);
                reveal(doc_lit_parts);
                assert(guide_rest(guided.guide@.unwrap())
                    == seq![ckc_spec::v1text::term_bytes(remaining[0])]
                        + term_expected.unwrap().1);
                if remaining.len() == 1 {
                    assert(e.0.len() == lits.len() + 1);
                    assert(term_next == 0x29);
                    assert(term_expected.unwrap().1[0] == seq![0x29u8]);
                    assert(term_expected.unwrap().1.len() > 0);
                    assert(term_expected.unwrap().1[0].len() > 0);
                    assert(term_expected.unwrap().1[0][0] == term_next);
                } else {
                    assert(remaining.len() > 1);
                    assert(e.0.len() != lits.len() + 1);
                    assert(term_next == 0x2c);
                    reveal_strlit(", ");
                    reveal(ckc_spec::v1text::ascii);
                    assert(ckc_spec::v1text::ascii(", "@).len() > 0);
                    assert(ckc_spec::v1text::ascii(", "@)[0] == 0x2c);
                    assert(term_expected.unwrap().1[0]
                        == ckc_spec::v1text::ascii(", "@));
                    assert(term_expected.unwrap().1.len() > 0);
                    assert(term_expected.unwrap().1[0].len() > 0);
                    assert(term_expected.unwrap().1[0][0] == term_next);
                }
            }
        }
        let ghost before_stream = tracker.stream@;
        let ghost old_lits = lits;
        let term = match parse_doc_literal(
            bytes,
            guided,
            Ghost(term_next),
            Ghost(term_expected),
            tracker,
        at,
        ) {
            Some(term) => term,
            None => return None,
        };
        proof {
            assert(lit_count < usize::MAX);
        }
        lit_count += 1;
        proof {
            assert(before_stream
                == entry_stream + ckc_spec::term::var_stream_all(old_lits));
            assert(tracker.stream@
                == before_stream + ckc_spec::term::var_stream(term@));
            var_stream_all_push(old_lits, term@);
            lits = old_lits.push(term@);
            assert(lits.len() == old_lits.len() + 1);
            assert(lits.len() >= 1);
            if old_lits.len() == 0 {
                assert_seqs_equal!(lits == seq![term@]);
                reveal_with_fuel(ckc_spec::v1text::lit_list_bytes, 2);
                assert_seqs_equal!(
                    (entry_prefix + ckc_spec::v1text::ascii("\\+ ("@))
                        + ckc_spec::v1text::term_bytes(term@)
                    == entry_prefix + ckc_spec::v1text::ascii("\\+ ("@)
                        + ckc_spec::v1text::lit_list_bytes(lits)
                );
            } else {
                lit_list_bytes_push(old_lits, term@);
                assert_seqs_equal!(
                    ((entry_prefix + ckc_spec::v1text::ascii("\\+ ("@)
                        + ckc_spec::v1text::lit_list_bytes(old_lits))
                        + ckc_spec::v1text::ascii(", "@))
                        + ckc_spec::v1text::term_bytes(term@)
                    == entry_prefix + ckc_spec::v1text::ascii("\\+ ("@)
                        + ckc_spec::v1text::lit_list_bytes(lits)
                );
            }
            assert(guided.cursor.prefix@
                == entry_prefix + ckc_spec::v1text::ascii("\\+ ("@)
                    + ckc_spec::v1text::lit_list_bytes(lits));
            assert_seqs_equal!(
                (entry_stream + ckc_spec::term::var_stream_all(old_lits))
                    + ckc_spec::term::var_stream(term@)
                == entry_stream
                    + (ckc_spec::term::var_stream_all(old_lits)
                        + ckc_spec::term::var_stream(term@))
            );
            assert(tracker.stream@
                == entry_stream + ckc_spec::term::var_stream_all(lits));
            assert forall|i: int| 0 <= i < lits.len()
                implies ckc_spec::v1text::wf_literal(lits[i]) by {
                if i < old_lits.len() {
                    assert(lits[i] == old_lits[i]);
                } else {
                    assert(i == old_lits.len());
                    assert(lits[i] == term@);
                }
            }
            if let Some(e) = expected@ {
                assert(term@ == e.0[old_lits.len() as int]);
                assert_seqs_equal!(
                    e.0.skip(old_lits.len() as int).drop_first()
                        == e.0.skip(lits.len() as int)
                );
                assert_seqs_equal!(
                    e.0.take(old_lits.len() as int).push(term@)
                        == e.0.take(lits.len() as int)
                );
                assert(lits == e.0.take(lits.len() as int));
                if lits.len() < e.0.len() {
                    assert(e.0.len() != old_lits.len() + 1);
                    assert(e.0.skip(old_lits.len() as int).len() != 1);
                    assert(term_expected.unwrap().1
                        == seq![ckc_spec::v1text::ascii(", "@)]
                            + doc_lit_parts(e.0.skip(lits.len() as int))
                            + seq![seq![0x29u8]] + e.1);
                } else {
                    assert(e.0.len() == old_lits.len() + 1);
                    assert(e.0.skip(old_lits.len() as int).len() == 1);
                    assert(term_expected.unwrap().1
                        == seq![seq![0x29u8]] + e.1);
                }
            }
        }
        if bytes[guided.cursor.pos] == 0x2c {
            proof {
                if let Some(e) = expected@ {
                    assert(term_next == 0x2c);
                    assert(lits.len() < e.0.len());
                    reveal(doc_lit_parts);
                    assert(guide_rest(guided.guide@.unwrap())
                        == seq![ckc_spec::v1text::ascii(", "@)]
                            + (doc_lit_parts(e.0.skip(lits.len() as int))
                                + seq![seq![0x29u8]] + e.1));
                }
            }
            let comma: &[u8] = b", ";
            proof {
                reveal_strlit(", ");
                reveal_byteslit(b", ");
                reveal(ckc_spec::v1text::ascii);
                assert(comma@ == ckc_spec::v1text::ascii(", "@));
            }
            let ghost after_comma = match expected@ {
                Some(e) => Some(
                    doc_lit_parts(e.0.skip(lits.len() as int))
                        + seq![seq![0x29u8]] + e.1,
                ),
                None => None,
            };
            if !doc_guided_literal(
                bytes,
                guided,
                comma,
                Ghost(ckc_spec::v1text::ascii(", "@)),
                Ghost(after_comma),
            at,
            ) {
                return None;
            }
            proof {
                assert_seqs_equal!(
                    (entry_prefix + ckc_spec::v1text::ascii("\\+ ("@)
                        + ckc_spec::v1text::lit_list_bytes(lits))
                        + ckc_spec::v1text::ascii(", "@)
                    == entry_prefix + ckc_spec::v1text::ascii("\\+ ("@)
                        + ckc_spec::v1text::lit_list_bytes(lits)
                        + ckc_spec::v1text::ascii(", "@)
                );
            }
        } else if bytes[guided.cursor.pos] == 0x29 {
            proof {
                if let Some(e) = expected@ {
                    assert(term_next == 0x29);
                    assert(lits.len() == e.0.len());
                    assert(lits == e.0);
                }
            }
            more = false;
        } else {
            proof {
                if expected@ is Some {
                    assert(false);
                }
            }
            return None;
        }
    }

    if lit_count < 2 {
        proof {
            if let Some(e) = expected@ {
                assert(e.0.len() >= 2);
                assert(lits == e.0);
                assert(lit_count == lits.len());
                assert(false);
            }
        }
        return None;
    }

    let close: &[u8] = b")";
    let ghost close_chunk = seq![0x29u8];
    proof {
        reveal_byteslit(b")");
        assert(close@ == close_chunk);
    }
    let ghost after_close = match expected@ {
        Some(e) => Some(e.1),
        None => None,
    };
    if !doc_guided_literal(
        bytes,
        guided,
        close,
        Ghost(close_chunk),
        Ghost(after_close),
    at,
    ) {
        return None;
    }
    let ghost item = ckc_spec::v1text::BodyItem::Naf(lits);
    proof {
        reveal(ckc_spec::v1text::wf_body_item);
        reveal(ckc_spec::v1text::item_var_stream);
        assert(lits.len() >= 1);
        assert(ckc_spec::v1text::wf_body_item(item));
        assert(entry_stream == old(tracker).stream@);
        assert(tracker.stream@
            == entry_stream + ckc_spec::term::var_stream_all(lits));
        assert(tracker.stream@
            == old(tracker).stream@
                + ckc_spec::v1text::item_var_stream(item));
        assert(guided_cursor_ok(bytes@, &guided));
        assert(tracker.valid ==>
            tracker_state_ok(tracker.next, tracker.stream@));
        assert(tracker_complete(tracker.valid, tracker.stream@));
        assert(tracker.stream@.len() <= guided.cursor.pos);
        assert(entry_pos == old(guided).cursor.pos);
        assert(entry_pos < guided.cursor.pos);
        assert(lits.len() >= 2);
        doc_body_item_parts_flat(item);
        reveal(ckc_spec::v1text::body_item_bytes);
        assert_seqs_equal!(
            (entry_prefix + ckc_spec::v1text::ascii("\\+ ("@)
                + ckc_spec::v1text::lit_list_bytes(lits)) + seq![0x29u8]
            == entry_prefix + doc_body_item_parts(item).flatten()
        );
        if let Some(e) = expected@ {
            assert(lits == e.0);
        }
    }
    Some(EDocBodyItem { item: Ghost(item) })
}

#[verifier::rlimit(2000)]
#[verifier::spinoff_prover]
fn parse_doc_body_item(
    bytes: &[u8],
    guided: &mut EGuidedCursor,
    next: Ghost<u8>,
    expected: Ghost<Option<(ckc_spec::v1text::BodyItem, Seq<Seq<u8>>)>>,
    tracker: &mut EVarTracker,
at: &mut usize,
) -> (r: Option<EDocBodyItem>)
    requires
        *old(at) <= bytes@.len(),
        guided_cursor_ok(bytes@, old(guided)),
        expected@ matches Some(e) ==> {
            &&& old(guided).guide@ matches Some(g)
                && guide_rest(g) == doc_body_item_parts(e.0) + e.1
            &&& ckc_spec::v1text::wf_body_item(e.0)
            &&& e.1.len() > 0
            &&& e.1[0].len() > 0
            &&& e.1[0][0] == next@
            &&& next@ == 0x2c || next@ == 0x2e
            &&& next@ == 0x2e ==> {
                &&& e.1[0].len() > 1
                &&& e.1[0][1] == 0x0a
            }
        },
        expected@ is None ==> old(guided).guide@ is None,
        old(tracker).valid ==>
            tracker_state_ok(old(tracker).next, old(tracker).stream@),
        tracker_complete(old(tracker).valid, old(tracker).stream@),
        old(tracker).stream@.len() <= old(guided).cursor.pos,
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r matches Some(item) ==> {
            &&& guided_cursor_ok(bytes@, final(guided))
            &&& ckc_spec::v1text::wf_body_item(item@)
            &&& final(tracker).stream@
                == old(tracker).stream@
                    + ckc_spec::v1text::item_var_stream(item@)
            &&& final(tracker).valid ==>
                tracker_state_ok(final(tracker).next, final(tracker).stream@)
            &&& tracker_complete(final(tracker).valid, final(tracker).stream@)
            &&& final(tracker).stream@.len() <= final(guided).cursor.pos
            &&& old(guided).cursor.pos < final(guided).cursor.pos
            &&& final(guided).cursor.prefix@
                == old(guided).cursor.prefix@
                    + doc_body_item_parts(item@).flatten()
        },
        expected@ matches Some(e) ==> {
            &&& r matches Some(item) && item@ == e.0
            &&& final(guided).guide@ matches Some(g)
                && guide_rest(g) == e.1
        },
        expected@ is None ==> final(guided).guide@ is None,
{
    proof {
        if let Some(e) = expected@ {
            reveal(ckc_spec::v1text::wf_body_item);
            reveal(doc_body_item_parts);
            match e.0 {
                ckc_spec::v1text::BodyItem::Pos(lit) => {
                    wf_literal_term_prefix_safe(lit);
                    assert(guide_rest(guided.guide@.unwrap())
                        == seq![ckc_spec::v1text::term_bytes(lit)] + e.1);
                    guide_front_bytes(
                        bytes@,
                        &guided,
                        ckc_spec::v1text::term_bytes(lit),
                        e.1,
                    );
                    assert(bytes@[guided.cursor.pos as int]
                        == ckc_spec::v1text::term_bytes(lit)[0]);
                },
                ckc_spec::v1text::BodyItem::Naf(gs) => {
                    assert(gs.len() >= 1);
                    let after = if gs.len() == 1 {
                        seq![ckc_spec::v1text::term_bytes(gs[0])] + e.1
                    } else {
                        seq![seq![0x28u8]] + doc_lit_parts(gs)
                            + seq![seq![0x29u8]] + e.1
                    };
                    assert(guide_rest(guided.guide@.unwrap())
                        == seq![ckc_spec::v1text::ascii("\\+ "@)] + after);
                    guide_front_bytes(
                        bytes@,
                        &guided,
                        ckc_spec::v1text::ascii("\\+ "@),
                        after,
                    );
                    reveal_strlit("\\+ ");
                    reveal(ckc_spec::v1text::ascii);
                    assert(ckc_spec::v1text::ascii("\\+ "@).len() == 3);
                    assert(ckc_spec::v1text::ascii("\\+ "@)[0] == 0x5c);
                    assert(bytes@[guided.cursor.pos as int] == 0x5c);
                    if gs.len() == 1 {
                        wf_literal_term_prefix_safe(gs[0]);
                        assert(guide_rest(guided.guide@.unwrap())
                            == seq![
                                ckc_spec::v1text::ascii("\\+ "@),
                                ckc_spec::v1text::term_bytes(gs[0]),
                            ] + e.1);
                        guide_second_bytes(
                            bytes@,
                            &guided,
                            ckc_spec::v1text::ascii("\\+ "@),
                            ckc_spec::v1text::term_bytes(gs[0]),
                            e.1,
                        );
                        assert(guided.cursor.pos as int + 4 <= bytes@.len());
                        assert(bytes@[guided.cursor.pos as int + 3]
                            == ckc_spec::v1text::term_bytes(gs[0])[0]);
                    } else {
                        let after_open = doc_lit_parts(gs)
                            + seq![seq![0x29u8]] + e.1;
                        assert(guide_rest(guided.guide@.unwrap())
                            == seq![
                                ckc_spec::v1text::ascii("\\+ "@),
                                seq![0x28u8],
                            ] + after_open);
                        guide_second_bytes(
                            bytes@,
                            &guided,
                            ckc_spec::v1text::ascii("\\+ "@),
                            seq![0x28u8],
                            after_open,
                        );
                        assert(guided.cursor.pos as int + 4 <= bytes@.len());
                        assert(bytes@.subrange(
                            guided.cursor.pos as int + 3,
                            guided.cursor.pos as int + 4,
                        ) == seq![0x28u8]);
                        assert(bytes@[guided.cursor.pos as int + 3]
                            == seq![0x28u8][0]);
                        assert(bytes@[guided.cursor.pos as int + 3] == 0x28);
                    }
                },
            }
        }
    }
    if guided.cursor.pos >= bytes.len() {
        proof {
            if expected@ is Some {
                assert(false);
            }
        }
        return None;
    }
    if bytes[guided.cursor.pos] != 0x5c {
        proof {
            if let Some(e) = expected@ {
                match e.0 {
                    ckc_spec::v1text::BodyItem::Pos(_) => {},
                    ckc_spec::v1text::BodyItem::Naf(_) => assert(false),
                }
            }
        }
        let ghost pos_expected = match expected@ {
            Some((ckc_spec::v1text::BodyItem::Pos(lit), tail)) => {
                Some((lit, tail))
            },
            _ => None,
        };
        return parse_doc_pos_item(bytes, guided, next, Ghost(pos_expected), tracker, at);
    }
    proof {
        if let Some(e) = expected@ {
            match e.0 {
                ckc_spec::v1text::BodyItem::Pos(lit) => {
                    wf_literal_term_prefix_safe(lit);
                    assert(bytes@[guided.cursor.pos as int]
                        == ckc_spec::v1text::term_bytes(lit)[0]);
                    assert(false);
                },
                ckc_spec::v1text::BodyItem::Naf(_) => {},
            }
        }
    }
    if bytes.len() - guided.cursor.pos <= 3 {
        proof {
            if expected@ is Some {
                assert(false);
            }
        }
        return None;
    }
    proof {
        assert(guided.cursor.pos + 3 < bytes.len());
        if let Some((ckc_spec::v1text::BodyItem::Naf(gs), tail)) = expected@ {
            if gs.len() == 1 {
                wf_literal_term_prefix_safe(gs[0]);
                assert(guide_rest(guided.guide@.unwrap())
                    == seq![
                        ckc_spec::v1text::ascii("\\+ "@),
                        ckc_spec::v1text::term_bytes(gs[0]),
                    ] + tail);
                guide_second_bytes(
                    bytes@,
                    &guided,
                    ckc_spec::v1text::ascii("\\+ "@),
                    ckc_spec::v1text::term_bytes(gs[0]),
                    tail,
                );
                assert(bytes@[guided.cursor.pos as int + 3]
                    == ckc_spec::v1text::term_bytes(gs[0])[0]);
            } else {
                let after_open = doc_lit_parts(gs) + seq![seq![0x29u8]] + tail;
                assert(guide_rest(guided.guide@.unwrap())
                    == seq![ckc_spec::v1text::ascii("\\+ "@), seq![0x28u8]]
                        + after_open);
                guide_second_bytes(
                    bytes@,
                    &guided,
                    ckc_spec::v1text::ascii("\\+ "@),
                    seq![0x28u8],
                    after_open,
                );
                assert(bytes@[guided.cursor.pos as int + 3] == 0x28);
            }
        }
    }
    if bytes[guided.cursor.pos + 3] == 0x28 {
        proof {
            if let Some((ckc_spec::v1text::BodyItem::Naf(gs), _)) = expected@ {
                if gs.len() == 1 {
                    wf_literal_term_prefix_safe(gs[0]);
                    assert(false);
                }
                assert(gs.len() >= 2);
            }
        }
        let ghost conj_expected = match expected@ {
            Some((ckc_spec::v1text::BodyItem::Naf(gs), tail)) => {
                Some((gs, tail))
            },
            _ => None,
        };
        let result = parse_doc_naf_conjunction(
            bytes,
            guided,
            Ghost(conj_expected),
            tracker,
        at,
        );
        proof {
            if let Some((ckc_spec::v1text::BodyItem::Naf(gs), tail)) = expected@ {
                assert(conj_expected == Some((gs, tail)));
                assert(result matches Some(item)
                    && item@ == ckc_spec::v1text::BodyItem::Naf(gs));
                assert(guided.guide@ matches Some(g)
                    && guide_rest(g) == tail);
            }
        }
        result
    } else {
        proof {
            if let Some((ckc_spec::v1text::BodyItem::Naf(gs), _)) = expected@ {
                if gs.len() != 1 {
                    assert(false);
                }
                assert(gs.len() == 1);
            }
        }
        let ghost singleton_expected = match expected@ {
            Some((ckc_spec::v1text::BodyItem::Naf(gs), tail)) => {
                Some((gs[0], tail))
            },
            _ => None,
        };
        let result = parse_doc_naf_singleton(
            bytes,
            guided,
            next,
            Ghost(singleton_expected),
            tracker,
        at,
        );
        proof {
            if let Some((ckc_spec::v1text::BodyItem::Naf(gs), tail)) = expected@ {
                assert(gs.len() == 1);
                assert(singleton_expected == Some((gs[0], tail)));
                assert(result matches Some(item)
                    && item@
                        == ckc_spec::v1text::BodyItem::Naf(seq![gs[0]]));
                assert_seqs_equal!(gs == seq![gs[0]]);
                assert(guided.guide@ matches Some(g)
                    && guide_rest(g) == tail);
            }
        }
        result
    }
}


proof fn body_var_stream_concat(
    left: Seq<ckc_spec::v1text::BodyItem>,
    right: Seq<ckc_spec::v1text::BodyItem>,
)
    ensures
        ckc_spec::v1text::body_var_stream(left + right)
            == ckc_spec::v1text::body_var_stream(left)
                + ckc_spec::v1text::body_var_stream(right),
    decreases left.len(),
{
    if left.len() == 0 {
        reveal_with_fuel(ckc_spec::v1text::body_var_stream, 2);
    } else {
        body_var_stream_concat(left.drop_first(), right);
        assert_seqs_equal!((left + right).drop_first()
            == left.drop_first() + right);
        reveal_with_fuel(ckc_spec::v1text::body_var_stream, 2);
        assert_seqs_equal!(
            ckc_spec::v1text::item_var_stream(left[0])
                + (ckc_spec::v1text::body_var_stream(left.drop_first())
                    + ckc_spec::v1text::body_var_stream(right))
            == (ckc_spec::v1text::item_var_stream(left[0])
                    + ckc_spec::v1text::body_var_stream(left.drop_first()))
                + ckc_spec::v1text::body_var_stream(right)
        );
    }
}

proof fn body_var_stream_push(
    items: Seq<ckc_spec::v1text::BodyItem>,
    item: ckc_spec::v1text::BodyItem,
)
    ensures
        ckc_spec::v1text::body_var_stream(items.push(item))
            == ckc_spec::v1text::body_var_stream(items)
                + ckc_spec::v1text::item_var_stream(item),
{
    assert_seqs_equal!(items.push(item) == items + seq![item]);
    body_var_stream_concat(items, seq![item]);
    reveal_with_fuel(ckc_spec::v1text::body_var_stream, 2);
}

proof fn body_bytes_push(
    items: Seq<ckc_spec::v1text::BodyItem>,
    item: ckc_spec::v1text::BodyItem,
)
    requires items.len() > 0,
    ensures
        ckc_spec::v1text::body_bytes(items.push(item))
            == ckc_spec::v1text::body_bytes(items)
                + ckc_spec::v1text::ascii(", "@)
                + ckc_spec::v1text::body_item_bytes(item),
    decreases items.len(),
{
    if items.len() == 1 {
        assert_seqs_equal!(items.push(item).drop_first() == seq![item]);
        reveal_with_fuel(ckc_spec::v1text::body_bytes, 3);
    } else {
        assert(items.drop_first().len() > 0);
        body_bytes_push(items.drop_first(), item);
        assert_seqs_equal!(items.push(item).drop_first()
            == items.drop_first().push(item));
        reveal_with_fuel(ckc_spec::v1text::body_bytes, 2);
    }
}

proof fn clauses_bytes_push(
    clauses: Seq<ckc_spec::v1text::DocClause>,
    clause: ckc_spec::v1text::DocClause,
)
    ensures
        ckc_spec::v1text::clauses_bytes(clauses.push(clause))
            == ckc_spec::v1text::clauses_bytes(clauses)
                + ckc_spec::v1text::clause_line(clause),
    decreases clauses.len(),
{
    if clauses.len() == 0 {
        assert_seqs_equal!(clauses.push(clause) == seq![clause]);
        reveal_with_fuel(ckc_spec::v1text::clauses_bytes, 3);
    } else {
        clauses_bytes_push(clauses.drop_first(), clause);
        assert_seqs_equal!(clauses.push(clause).drop_first()
            == clauses.drop_first().push(clause));
        reveal_with_fuel(ckc_spec::v1text::clauses_bytes, 2);
    }
}

pub struct EDocClause {
    pub clause: Ghost<ckc_spec::v1text::DocClause>,
}

impl View for EDocClause {
    type V = ckc_spec::v1text::DocClause;

    open spec fn view(&self) -> ckc_spec::v1text::DocClause {
        self.clause@
    }
}

#[verifier::rlimit(5000)]
#[verifier::spinoff_prover]
fn parse_doc_clause(
    bytes: &[u8],
    guided: &mut EGuidedCursor,
    expected: Ghost<Option<(ckc_spec::v1text::DocClause, Seq<Seq<u8>>)>>,
at: &mut usize,
) -> (r: Option<EDocClause>)
    requires
        *old(at) <= bytes@.len(),
        guided_cursor_ok(bytes@, old(guided)),
        expected@ matches Some(e) ==> {
            &&& old(guided).guide@ matches Some(g)
                && guide_rest(g) == doc_clause_parts(e.0) + e.1
            &&& ckc_spec::v1text::wf_clause(e.0)
        },
        expected@ is None ==> old(guided).guide@ is None,
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r matches Some(clause) ==> {
            &&& guided_cursor_ok(bytes@, final(guided))
            &&& ckc_spec::v1text::wf_clause(clause@)
            &&& old(guided).cursor.pos < final(guided).cursor.pos
            &&& final(guided).cursor.prefix@
                == old(guided).cursor.prefix@
                    + doc_clause_parts(clause@).flatten()
        },
        expected@ matches Some(e) ==> {
            &&& r matches Some(clause) && clause@ == e.0
            &&& final(guided).guide@ matches Some(g)
                && guide_rest(g) == e.1
        },
        expected@ is None ==> final(guided).guide@ is None,
{
    let entry_pos = guided.cursor.pos;
    let ghost entry_prefix = guided.cursor.prefix@;
    let ghost head_expected = match expected@ {
        Some(e) => {
            let tail = if e.0.body.len() == 0 {
                seq![ckc_spec::v1text::ascii(".\n"@)] + e.1
            } else {
                seq![ckc_spec::v1text::ascii(" :- "@)]
                    + doc_body_parts(e.0.body)
                    + seq![ckc_spec::v1text::ascii(".\n"@)] + e.1
            };
            Some((e.0.head, tail))
        },
        None => None,
    };
    let ghost head_next = match expected@ {
        Some(e) => if e.0.body.len() == 0 { 0x2eu8 } else { 0x20u8 },
        None => 0x20u8,
    };
    proof {
        if let Some(e) = expected@ {
            reveal(ckc_spec::v1text::wf_clause);
            reveal(doc_clause_parts);
            assert(ckc_spec::v1text::wf_literal(e.0.head));
            assert(guide_rest(guided.guide@.unwrap())
                == seq![ckc_spec::v1text::term_bytes(e.0.head)]
                    + head_expected.unwrap().1);
            if e.0.body.len() == 0 {
                reveal_strlit(".\n");
                reveal(ckc_spec::v1text::ascii);
                assert(head_expected.unwrap().1.len() > 0);
                assert(head_expected.unwrap().1[0].len() > 1);
                assert(head_expected.unwrap().1[0][0] == 0x2e);
                assert(head_expected.unwrap().1[0][1] == 0x0a);
                assert(head_next == 0x2e);
            } else {
                reveal_strlit(" :- ");
                reveal(ckc_spec::v1text::ascii);
                assert(head_expected.unwrap().1.len() > 0);
                assert(head_expected.unwrap().1[0].len() > 0);
                assert(head_expected.unwrap().1[0][0] == 0x20);
                assert(head_next == 0x20);
            }
        }
    }
    let mut tracker = new_var_tracker();
    let head = match parse_doc_literal(
        bytes,
        guided,
        Ghost(head_next),
        Ghost(head_expected),
        &mut tracker,
    at,
    ) {
        Some(head) => head,
        None => return None,
    };
    proof {
        assert(tracker.stream@
            == ckc_spec::term::var_stream(head@)) by {
            assert_seqs_equal!(Seq::<nat>::empty()
                + ckc_spec::term::var_stream(head@)
                == ckc_spec::term::var_stream(head@));
        }
    }

    if bytes[guided.cursor.pos] == 0x2e {
        proof {
            if let Some(e) = expected@ {
                assert(head_next == 0x2e);
                assert(e.0.body.len() == 0);
                assert(head@ == e.0.head);
                assert(guide_rest(guided.guide@.unwrap())
                    == seq![ckc_spec::v1text::ascii(".\n"@)] + e.1);
                assert(ckc_spec::term::var_canonical(tracker.stream@));
                reveal(tracker_complete);
                assert(tracker.stream@.len() <= guided.cursor.pos);
                assert(tracker.valid);
            }
        }
        if !tracker.valid {
            return None;
        }
        proof {
            tracker_state_canonical(tracker.next, tracker.stream@);
        }
        let line_end: &[u8] = b".\n";
        proof {
            reveal_strlit(".\n");
            reveal_byteslit(b".\n");
            reveal(ckc_spec::v1text::ascii);
            assert(line_end@ == ckc_spec::v1text::ascii(".\n"@));
        }
        let ghost after_line = match expected@ {
            Some(e) => Some(e.1),
            None => None,
        };
        if !doc_guided_literal(
            bytes,
            guided,
            line_end,
            Ghost(ckc_spec::v1text::ascii(".\n"@)),
            Ghost(after_line),
        at,
        ) {
            return None;
        }
        let ghost model = ckc_spec::v1text::DocClause {
            head: head@,
            body: Seq::empty(),
        };
        proof {
            reveal(ckc_spec::v1text::wf_clause);
            reveal_with_fuel(ckc_spec::v1text::body_var_stream, 2);
            assert forall|i: int| 0 <= i < model.body.len()
                implies ckc_spec::v1text::wf_body_item(model.body[i]) by {
                assert(false);
            }
            assert(ckc_spec::term::var_stream(model.head)
                + ckc_spec::v1text::body_var_stream(model.body)
                == tracker.stream@) by {
                assert_seqs_equal!(ckc_spec::term::var_stream(model.head)
                    + Seq::<nat>::empty()
                    == ckc_spec::term::var_stream(model.head));
            }
            assert(ckc_spec::v1text::wf_clause(model));
            assert(entry_pos == old(guided).cursor.pos);
            assert(entry_pos < guided.cursor.pos);
            doc_clause_parts_flat(model);
            assert_seqs_equal!(guided.cursor.prefix@
                == entry_prefix + doc_clause_parts(model).flatten());
            if let Some(e) = expected@ {
                assert(head@ == e.0.head);
                assert(e.0.body.len() == 0);
                assert_seqs_equal!(e.0.body
                    == Seq::<ckc_spec::v1text::BodyItem>::empty());
                assert(model.head == e.0.head);
                assert(model.body == e.0.body);
                assert(model == e.0);
            }
        }
        return Some(EDocClause { clause: Ghost(model) });
    }
    proof {
        if let Some(e) = expected@ {
            assert(head_next == 0x20);
            assert(e.0.body.len() > 0);
            assert(bytes@[guided.cursor.pos as int] == 0x20);
        }
    }
    if bytes[guided.cursor.pos] != 0x20 {
        return None;
    }
    let rule_open: &[u8] = b" :- ";
    proof {
        reveal_strlit(" :- ");
        reveal_byteslit(b" :- ");
        reveal(ckc_spec::v1text::ascii);
        assert(rule_open@ == ckc_spec::v1text::ascii(" :- "@));
        if let Some(e) = expected@ {
            assert(guide_rest(guided.guide@.unwrap())
                == seq![ckc_spec::v1text::ascii(" :- "@)]
                    + (doc_body_parts(e.0.body)
                        + seq![ckc_spec::v1text::ascii(".\n"@)] + e.1));
        }
    }
    let ghost after_open = match expected@ {
        Some(e) => Some(
            doc_body_parts(e.0.body)
                + seq![ckc_spec::v1text::ascii(".\n"@)] + e.1,
        ),
        None => None,
    };
    if !doc_guided_literal(
        bytes,
        guided,
        rule_open,
        Ghost(ckc_spec::v1text::ascii(" :- "@)),
        Ghost(after_open),
    at,
    ) {
        return None;
    }

    let ghost mut items: Seq<ckc_spec::v1text::BodyItem> = Seq::empty();
    let mut more = true;
    proof {
        reveal_with_fuel(ckc_spec::v1text::body_var_stream, 2);
        assert(ckc_spec::v1text::body_var_stream(items)
            == Seq::<nat>::empty());
        assert_seqs_equal!(ckc_spec::term::var_stream(head@)
            + Seq::<nat>::empty()
            == ckc_spec::term::var_stream(head@));
        assert(tracker.stream@
            == ckc_spec::term::var_stream(head@)
                + ckc_spec::v1text::body_var_stream(items));
        assert forall|i: int| 0 <= i < items.len()
            implies ckc_spec::v1text::wf_body_item(items[i]) by {
            assert(false);
        }
        assert_seqs_equal!(guided.cursor.prefix@
            == entry_prefix
                + ckc_spec::v1text::term_bytes(head@)
                + ckc_spec::v1text::ascii(" :- "@)
                + ckc_spec::v1text::body_bytes(items));
        if let Some(e) = expected@ {
            assert_seqs_equal!(e.0.body.take(0)
                == Seq::<ckc_spec::v1text::BodyItem>::empty());
            assert_seqs_equal!(e.0.body.skip(0) == e.0.body);
            assert(items == e.0.body.take(items.len() as int));
            assert(guide_rest(guided.guide@.unwrap())
                == doc_body_parts(e.0.body.skip(items.len() as int))
                    + seq![ckc_spec::v1text::ascii(".\n"@)] + e.1);
        }
    }
    while more
        invariant
            *old(at) <= *at <= bytes@.len(),
            guided_cursor_ok(bytes@, &guided),
            entry_pos < guided.cursor.pos,
            ckc_spec::v1text::wf_literal(head@),
            tracker.valid ==>
                tracker_state_ok(tracker.next, tracker.stream@),
            tracker_complete(tracker.valid, tracker.stream@),
            tracker.stream@
                == ckc_spec::term::var_stream(head@)
                    + ckc_spec::v1text::body_var_stream(items),
            tracker.stream@.len() <= guided.cursor.pos,
            guided.cursor.prefix@
                == entry_prefix
                    + ckc_spec::v1text::term_bytes(head@)
                    + ckc_spec::v1text::ascii(" :- "@)
                    + ckc_spec::v1text::body_bytes(items)
                    + if more && items.len() > 0 {
                        ckc_spec::v1text::ascii(", "@)
                    } else {
                        Seq::<u8>::empty()
                    },
            forall|i: int| 0 <= i < items.len()
                ==> ckc_spec::v1text::wf_body_item(items[i]),
            !more ==> items.len() >= 1,
            expected@ matches Some(e) ==> {
                &&& e.0.body.len() > 0
                &&& ckc_spec::v1text::wf_clause(e.0)
                &&& items == e.0.body.take(items.len() as int)
                &&& items.len() <= e.0.body.len()
                &&& more ==> items.len() < e.0.body.len()
                &&& !more ==> items.len() == e.0.body.len()
                &&& guided.guide@ matches Some(g)
                    && guide_rest(g) == if more {
                        doc_body_parts(e.0.body.skip(items.len() as int))
                            + seq![ckc_spec::v1text::ascii(".\n"@)] + e.1
                    } else {
                        seq![ckc_spec::v1text::ascii(".\n"@)] + e.1
                    }
            },
            expected@ is None ==> guided.guide@ is None,
        decreases if more {
            bytes.len() - guided.cursor.pos + 1
        } else {
            0
        },
    {
        let ghost item_expected = match expected@ {
            Some(e) => {
                let remaining = e.0.body.skip(items.len() as int);
                let line_tail = seq![ckc_spec::v1text::ascii(".\n"@)] + e.1;
                let after = if remaining.len() == 1 {
                    line_tail
                } else {
                    seq![ckc_spec::v1text::ascii(", "@)]
                        + doc_body_parts(remaining.drop_first()) + line_tail
                };
                Some((remaining[0], after))
            },
            None => None,
        };
        let ghost item_next = match expected@ {
            Some(e) => if e.0.body.len() == items.len() + 1 {
                0x2eu8
            } else {
                0x2cu8
            },
            None => 0x2cu8,
        };
        proof {
            if let Some(e) = expected@ {
                reveal(ckc_spec::v1text::wf_clause);
                let remaining = e.0.body.skip(items.len() as int);
                assert(0 <= items.len() < e.0.body.len());
                assert(remaining.len() == e.0.body.len() - items.len());
                assert(remaining.len() > 0);
                assert(ckc_spec::v1text::wf_body_item(
                    e.0.body[items.len() as int],
                ));
                assert(remaining[0] == e.0.body[items.len() as int]);
                reveal(doc_body_parts);
                assert(guide_rest(guided.guide@.unwrap())
                    == doc_body_item_parts(remaining[0])
                        + item_expected.unwrap().1);
                if remaining.len() == 1 {
                    reveal_strlit(".\n");
                    reveal(ckc_spec::v1text::ascii);
                    assert(e.0.body.len() == items.len() + 1);
                    assert(item_next == 0x2e);
                    assert(item_expected.unwrap().1.len() > 0);
                    assert(item_expected.unwrap().1[0].len() > 1);
                    assert(item_expected.unwrap().1[0][0] == item_next);
                    assert(item_expected.unwrap().1[0][1] == 0x0a);
                } else {
                    reveal_strlit(", ");
                    reveal(ckc_spec::v1text::ascii);
                    assert(remaining.len() > 1);
                    assert(e.0.body.len() != items.len() + 1);
                    assert(item_next == 0x2c);
                    assert(item_expected.unwrap().1.len() > 0);
                    assert(item_expected.unwrap().1[0].len() > 0);
                    assert(item_expected.unwrap().1[0][0] == item_next);
                }
            }
        }
        let ghost before_stream = tracker.stream@;
        let ghost before_item_prefix = guided.cursor.prefix@;
        let ghost old_items = items;
        let item = match parse_doc_body_item(
            bytes,
            guided,
            Ghost(item_next),
            Ghost(item_expected),
            &mut tracker,
        at,
        ) {
            Some(item) => item,
            None => return None,
        };
        proof {
            if expected@ is Some {
                let after = item_expected.unwrap().1;
                assert(guided.guide@ matches Some(g)
                    && guide_rest(g) == after);
                assert(after.len() > 0);
                assert(after[0].len() > 0);
                assert(after[0][0] == item_next);
                assert_seqs_equal!(after == seq![after[0]] + after.drop_first());
                guide_front_bytes(
                    bytes@,
                    &guided,
                    after[0],
                    after.drop_first(),
                );
                assert(guided.cursor.pos < bytes.len());
                assert(bytes@[guided.cursor.pos as int] == after[0][0]);
                assert(bytes@[guided.cursor.pos as int] == item_next);
            }
        }
        if guided.cursor.pos >= bytes.len() {
            proof {
                if expected@ is Some {
                    assert(false);
                }
            }
            return None;
        }
        proof {
            assert(before_stream
                == ckc_spec::term::var_stream(head@)
                    + ckc_spec::v1text::body_var_stream(old_items));
            assert(tracker.stream@
                == before_stream
                    + ckc_spec::v1text::item_var_stream(item@));
            body_var_stream_push(old_items, item@);
            items = old_items.push(item@);
            assert(items.len() == old_items.len() + 1);
            assert(items.len() >= 1);
            assert_seqs_equal!(
                (ckc_spec::term::var_stream(head@)
                    + ckc_spec::v1text::body_var_stream(old_items))
                    + ckc_spec::v1text::item_var_stream(item@)
                == ckc_spec::term::var_stream(head@)
                    + (ckc_spec::v1text::body_var_stream(old_items)
                        + ckc_spec::v1text::item_var_stream(item@))
            );
            assert(tracker.stream@
                == ckc_spec::term::var_stream(head@)
                    + ckc_spec::v1text::body_var_stream(items));
            doc_body_item_parts_flat(item@);
            if old_items.len() == 0 {
                assert_seqs_equal!(old_items
                    == Seq::<ckc_spec::v1text::BodyItem>::empty());
                assert_seqs_equal!(items == seq![item@]);
                reveal_with_fuel(ckc_spec::v1text::body_bytes, 3);
            } else {
                body_bytes_push(old_items, item@);
            }
            assert_seqs_equal!(guided.cursor.prefix@
                == entry_prefix
                    + ckc_spec::v1text::term_bytes(head@)
                    + ckc_spec::v1text::ascii(" :- "@)
                    + ckc_spec::v1text::body_bytes(items));
            assert forall|i: int| 0 <= i < items.len()
                implies ckc_spec::v1text::wf_body_item(items[i]) by {
                if i < old_items.len() {
                    assert(items[i] == old_items[i]);
                } else {
                    assert(i == old_items.len());
                    assert(items[i] == item@);
                }
            }
            if let Some(e) = expected@ {
                assert(item@ == e.0.body[old_items.len() as int]);
                assert_seqs_equal!(
                    e.0.body.skip(old_items.len() as int).drop_first()
                        == e.0.body.skip(items.len() as int)
                );
                assert_seqs_equal!(
                    e.0.body.take(old_items.len() as int).push(item@)
                        == e.0.body.take(items.len() as int)
                );
                assert(items == e.0.body.take(items.len() as int));
                if items.len() < e.0.body.len() {
                    assert(e.0.body.len() != old_items.len() + 1);
                    assert(item_expected.unwrap().1
                        == seq![ckc_spec::v1text::ascii(", "@)]
                            + doc_body_parts(e.0.body.skip(items.len() as int))
                            + seq![ckc_spec::v1text::ascii(".\n"@)] + e.1);
                } else {
                    assert(e.0.body.len() == old_items.len() + 1);
                    assert(item_expected.unwrap().1
                        == seq![ckc_spec::v1text::ascii(".\n"@)] + e.1);
                }
            }
        }
        if bytes[guided.cursor.pos] == 0x2c {
            proof {
                if let Some(e) = expected@ {
                    assert(item_next == 0x2c);
                    assert(items.len() < e.0.body.len());
                    assert(guide_rest(guided.guide@.unwrap())
                        == seq![ckc_spec::v1text::ascii(", "@)]
                            + (doc_body_parts(e.0.body.skip(items.len() as int))
                                + seq![ckc_spec::v1text::ascii(".\n"@)] + e.1));
                }
            }
            let comma: &[u8] = b", ";
            proof {
                reveal_strlit(", ");
                reveal_byteslit(b", ");
                reveal(ckc_spec::v1text::ascii);
                assert(comma@ == ckc_spec::v1text::ascii(", "@));
            }
            let ghost after_comma = match expected@ {
                Some(e) => Some(
                    doc_body_parts(e.0.body.skip(items.len() as int))
                        + seq![ckc_spec::v1text::ascii(".\n"@)] + e.1,
                ),
                None => None,
            };
            if !doc_guided_literal(
                bytes,
                guided,
                comma,
                Ghost(ckc_spec::v1text::ascii(", "@)),
                Ghost(after_comma),
            at,
            ) {
                return None;
            }
        } else if bytes[guided.cursor.pos] == 0x2e {
            proof {
                if let Some(e) = expected@ {
                    assert(item_next == 0x2e);
                    assert(items.len() == e.0.body.len());
                    assert(items == e.0.body);
                }
            }
            more = false;
        } else {
            proof {
                if expected@ is Some {
                    assert(false);
                }
            }
            return None;
        }
    }

    proof {
        if let Some(e) = expected@ {
            assert(items == e.0.body);
            assert(guide_rest(guided.guide@.unwrap())
                == seq![ckc_spec::v1text::ascii(".\n"@)] + e.1);
            assert(ckc_spec::term::var_canonical(tracker.stream@));
            reveal(tracker_complete);
            assert(tracker.stream@.len() <= guided.cursor.pos);
            assert(tracker.valid);
        }
    }
    if !tracker.valid {
        return None;
    }
    proof {
        tracker_state_canonical(tracker.next, tracker.stream@);
    }
    let line_end: &[u8] = b".\n";
    proof {
        reveal_strlit(".\n");
        reveal_byteslit(b".\n");
        reveal(ckc_spec::v1text::ascii);
        assert(line_end@ == ckc_spec::v1text::ascii(".\n"@));
    }
    let ghost after_line = match expected@ {
        Some(e) => Some(e.1),
        None => None,
    };
    if !doc_guided_literal(
        bytes,
        guided,
        line_end,
        Ghost(ckc_spec::v1text::ascii(".\n"@)),
        Ghost(after_line),
    at,
    ) {
        return None;
    }
    let ghost model = ckc_spec::v1text::DocClause {
        head: head@,
        body: items,
    };
    proof {
        reveal(ckc_spec::v1text::wf_clause);
        assert forall|i: int| 0 <= i < model.body.len()
            implies ckc_spec::v1text::wf_body_item(model.body[i]) by {
            assert(ckc_spec::v1text::wf_body_item(items[i]));
        }
        assert(ckc_spec::term::var_stream(model.head)
            + ckc_spec::v1text::body_var_stream(model.body)
            == tracker.stream@);
        assert(ckc_spec::v1text::wf_clause(model));
        assert(entry_pos == old(guided).cursor.pos);
        assert(entry_pos < guided.cursor.pos);
        doc_clause_parts_flat(model);
        assert_seqs_equal!(guided.cursor.prefix@
            == entry_prefix + doc_clause_parts(model).flatten());
        if let Some(e) = expected@ {
            assert(model == e.0);
        }
    }
    Some(EDocClause { clause: Ghost(model) })
}

pub struct EDocBundle {
    pub bundle: Ghost<ckc_spec::v1text::Bundle>,
    pub ordinal: Vec<u8>,
}

impl View for EDocBundle {
    type V = ckc_spec::v1text::Bundle;

    open spec fn view(&self) -> ckc_spec::v1text::Bundle {
        self.bundle@
    }
}

#[verifier::rlimit(5000)]
#[verifier::spinoff_prover]
fn parse_doc_bundle(
    bytes: &[u8],
    guided: &mut EGuidedCursor,
    expected: Ghost<Option<(ckc_spec::v1text::Bundle, Seq<Seq<u8>>)>>,
at: &mut usize,
) -> (r: Option<EDocBundle>)
    requires
        *old(at) <= bytes@.len(),
        guided_cursor_ok(bytes@, old(guided)),
        expected@ matches Some(e) ==> {
            &&& old(guided).guide@ matches Some(g)
                && guide_rest(g) == doc_bundle_parts(e.0) + e.1
            &&& ckc_spec::v1text::wf_bundle(e.0)
            &&& (e.1.len() == 0
                || e.1[0] == ckc_spec::v1text::ascii("% S"@))
        },
        expected@ is None ==> old(guided).guide@ is None,
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r matches Some(bundle) ==> {
            &&& guided_cursor_ok(bytes@, final(guided))
            &&& ckc_spec::v1text::wf_bundle(bundle@)
            &&& canonical_decimal(bundle.ordinal@)
            &&& bundle.ordinal@
                == ckc_spec::v1text::udec_bytes(bundle@.s)
            &&& old(guided).cursor.pos < final(guided).cursor.pos
            &&& final(guided).cursor.prefix@
                == old(guided).cursor.prefix@
                    + doc_bundle_parts(bundle@).flatten()
            &&& final(guided).cursor.pos == bytes@.len()
                || final(guided).cursor.pos < bytes@.len()
                    && bytes@[final(guided).cursor.pos as int] == 0x25
        },
        expected@ matches Some(e) ==> {
            &&& r matches Some(bundle) && bundle@ == e.0
            &&& final(guided).guide@ matches Some(g)
                && guide_rest(g) == e.1
        },
        expected@ is None ==> final(guided).guide@ is None,
{
    let entry_pos = guided.cursor.pos;
    let ghost entry_prefix = guided.cursor.prefix@;
    let marker: &[u8] = b"% S";
    proof {
        reveal_strlit("% S");
        reveal_byteslit(b"% S");
        reveal(ckc_spec::v1text::ascii);
        assert(marker@ == ckc_spec::v1text::ascii("% S"@));
        if let Some(e) = expected@ {
            reveal(ckc_spec::v1text::wf_bundle);
            reveal(doc_bundle_parts);
            reveal(doc_marker_parts);
            assert(guide_rest(guided.guide@.unwrap())
                == seq![ckc_spec::v1text::ascii("% S"@)]
                    + (seq![
                        ckc_spec::v1text::udec_bytes(e.0.s),
                        ckc_spec::v1text::ascii(": "@),
                        e.0.text,
                        seq![0x0au8],
                    ] + doc_clauses_parts(e.0.clauses) + e.1));
        }
    }
    let ghost after_marker = match expected@ {
        Some(e) => Some(
            seq![
                ckc_spec::v1text::udec_bytes(e.0.s),
                ckc_spec::v1text::ascii(": "@),
                e.0.text,
                seq![0x0au8],
            ] + doc_clauses_parts(e.0.clauses) + e.1,
        ),
        None => None,
    };
    if !doc_guided_literal(
        bytes,
        guided,
        marker,
        Ghost(ckc_spec::v1text::ascii("% S"@)),
        Ghost(after_marker),
    at,
    ) {
        return None;
    }
    proof {
        if let Some(e) = expected@ {
            let digits = ckc_spec::v1text::udec_bytes(e.0.s);
            let rest = seq![
                ckc_spec::v1text::ascii(": "@),
                e.0.text,
                seq![0x0au8],
            ] + doc_clauses_parts(e.0.clauses) + e.1;
            assert(guide_rest(guided.guide@.unwrap())
                == seq![digits] + rest);
            guide_front_bytes(bytes@, &guided, digits, rest);
            udec_bytes_nonempty(e.0.s);
            assert(guided.cursor.pos < bytes.len());
        }
    }

    if guided.cursor.pos >= bytes.len() {
        proof {
            if expected@ is Some {
                assert(false);
            }
        }
        return None;
    }
    let ordinal_start = guided.cursor.pos;
    let ghost decimal_expected = match expected@ {
        Some(e) => Some(GDecimalExpected {
            value: e.0.s,
            end: (ordinal_start as int
                + ckc_spec::v1text::udec_bytes(e.0.s).len()) as usize,
        }),
        None => None,
    };
    proof {
        if let Some(e) = expected@ {
            let digits = ckc_spec::v1text::udec_bytes(e.0.s);
            let rest = seq![
                ckc_spec::v1text::ascii(": "@),
                e.0.text,
                seq![0x0au8],
            ] + doc_clauses_parts(e.0.clauses) + e.1;
            assert(guide_rest(guided.guide@.unwrap())
                == seq![digits] + rest);
            guide_front_bytes(bytes@, &guided, digits, rest);
            udec_bytes_nonempty(e.0.s);
            assert(ordinal_start as int + digits.len() <= bytes@.len());
            assert(ordinal_start as int + digits.len() <= usize::MAX);
            assert(decimal_expected.unwrap().end as int
                == ordinal_start as int + digits.len());
            assert(bytes@.subrange(
                ordinal_start as int,
                decimal_expected.unwrap().end as int,
            ) == digits);
            assert(guide_rest(guided.guide@.unwrap())
                == seq![digits, ckc_spec::v1text::ascii(": "@)]
                    + (seq![e.0.text, seq![0x0au8]]
                        + doc_clauses_parts(e.0.clauses) + e.1));
            guide_second_bytes(
                bytes@,
                &guided,
                digits,
                ckc_spec::v1text::ascii(": "@),
                seq![e.0.text, seq![0x0au8]]
                    + doc_clauses_parts(e.0.clauses) + e.1,
            );
            reveal_strlit(": ");
            reveal(ckc_spec::v1text::ascii);
            assert(bytes@.subrange(
                decimal_expected.unwrap().end as int,
                decimal_expected.unwrap().end as int + 2,
            ) == ckc_spec::v1text::ascii(": "@));
            assert(bytes@[decimal_expected.unwrap().end as int]
                == ckc_spec::v1text::ascii(": "@)[0]);
            assert(bytes@[decimal_expected.unwrap().end as int] == 0x3a);
            reveal(ckc_spec::v1text::is_digit_b);
            reveal(decimal_end);
            assert(decimal_end(bytes@, decimal_expected.unwrap().end));
        }
    }
    let decimal = match parse_decimal(
        bytes,
        ordinal_start,
        Ghost(decimal_expected),
    at,
    ) {
        Some(decimal) => decimal,
        None => return None,
    };
    let ordinal = copy_range(bytes, ordinal_start, decimal.end);
    proof {
        reveal(parsed_decimal_ok);
        assert(ordinal@
            == ckc_spec::v1text::udec_bytes(decimal.value@));
        if let Some(e) = expected@ {
            assert(decimal.value@ == e.0.s);
            assert(ordinal@ == ckc_spec::v1text::udec_bytes(e.0.s));
            assert(guide_rest(guided.guide@.unwrap())
                == seq![ordinal@]
                    + (seq![
                        ckc_spec::v1text::ascii(": "@),
                        e.0.text,
                        seq![0x0au8],
                    ] + doc_clauses_parts(e.0.clauses) + e.1));
        }
    }
    let ghost after_ordinal = match expected@ {
        Some(e) => Some(
            seq![
                ckc_spec::v1text::ascii(": "@),
                e.0.text,
                seq![0x0au8],
            ] + doc_clauses_parts(e.0.clauses) + e.1,
        ),
        None => None,
    };
    if !doc_guided_literal(
        bytes,
        guided,
        &ordinal,
        Ghost(ordinal@),
        Ghost(after_ordinal),
    at,
    ) {
        return None;
    }
    if ordinal.len() == 1 && ordinal[0] == 0x30 {
        proof {
            if let Some(e) = expected@ {
                assert(e.0.s >= 1);
                reveal_with_fuel(ckc_spec::v1text::udec_bytes, 2);
                assert(false);
            }
        }
        return None;
    }
    proof {
        udec_canonical(decimal.value@);
        assert(canonical_decimal(ordinal@));
        if decimal.value@ == 0 {
            reveal_with_fuel(ckc_spec::v1text::udec_bytes, 2);
            assert(ordinal@ == seq![0x30u8]);
            assert(ordinal.len() == 1 && ordinal[0] == 0x30);
            assert(false);
        }
        assert(decimal.value@ >= 1);
    }

    let colon: &[u8] = b": ";
    proof {
        reveal_strlit(": ");
        reveal_byteslit(b": ");
        reveal(ckc_spec::v1text::ascii);
        assert(colon@ == ckc_spec::v1text::ascii(": "@));
        if let Some(e) = expected@ {
            assert(guide_rest(guided.guide@.unwrap())
                == seq![ckc_spec::v1text::ascii(": "@)]
                    + (seq![e.0.text, seq![0x0au8]]
                        + doc_clauses_parts(e.0.clauses) + e.1));
        }
    }
    let ghost after_colon = match expected@ {
        Some(e) => Some(
            seq![e.0.text, seq![0x0au8]]
                + doc_clauses_parts(e.0.clauses) + e.1,
        ),
        None => None,
    };
    if !doc_guided_literal(
        bytes,
        guided,
        colon,
        Ghost(ckc_spec::v1text::ascii(": "@)),
        Ghost(after_colon),
    at,
    ) {
        return None;
    }
    proof {
        if let Some(e) = expected@ {
            let rest = seq![seq![0x0au8]]
                + doc_clauses_parts(e.0.clauses) + e.1;
            assert(guide_rest(guided.guide@.unwrap())
                == seq![e.0.text] + rest);
            guide_front_bytes(bytes@, &guided, e.0.text, rest);
            reveal(ckc_spec::v1text::wf_bundle);
            assert(e.0.text.len() > 0);
            assert(guided.cursor.pos < bytes.len());
        }
    }

    if guided.cursor.pos >= bytes.len() {
        proof {
            if expected@ is Some {
                assert(false);
            }
        }
        return None;
    }
    let text_start = guided.cursor.pos;
    let ghost text_expected = match expected@ {
        Some(e) => Some(GTextExpected {
            value: e.0.text,
            end: (text_start as int + e.0.text.len()) as usize,
        }),
        None => None,
    };
    proof {
        if let Some(e) = expected@ {
            reveal(ckc_spec::v1text::wf_bundle);
            let rest = seq![seq![0x0au8]]
                + doc_clauses_parts(e.0.clauses) + e.1;
            assert(guide_rest(guided.guide@.unwrap())
                == seq![e.0.text] + rest);
            guide_front_bytes(bytes@, &guided, e.0.text, rest);
            assert(e.0.text.len() > 0);
            assert(text_start as int + e.0.text.len() <= bytes@.len());
            assert(text_start as int + e.0.text.len() <= usize::MAX);
            assert(text_expected.unwrap().end as int
                == text_start as int + e.0.text.len());
            assert(bytes@.subrange(
                text_start as int,
                text_expected.unwrap().end as int,
            ) == e.0.text);
            assert(guide_rest(guided.guide@.unwrap())
                == seq![e.0.text, seq![0x0au8]]
                    + (doc_clauses_parts(e.0.clauses) + e.1));
            guide_second_bytes(
                bytes@,
                &guided,
                e.0.text,
                seq![0x0au8],
                doc_clauses_parts(e.0.clauses) + e.1,
            );
            assert(text_start as int + e.0.text.len() + 1
                <= bytes@.len());
            assert(text_expected.unwrap().end < bytes.len());
            assert(bytes@.subrange(
                text_expected.unwrap().end as int,
                text_expected.unwrap().end as int + 1,
            ) == seq![0x0au8]);
            assert(bytes@[text_expected.unwrap().end as int]
                == seq![0x0au8][0]);
            assert(bytes@[text_expected.unwrap().end as int] == 0x0a);
        }
    }
    let text = match parse_raw_text(bytes, text_start, Ghost(text_expected), at) {
        Some(text) => text,
        None => return None,
    };
    proof {
        if let Some(e) = expected@ {
            assert(text.value@ == e.0.text);
            assert(guide_rest(guided.guide@.unwrap())
                == seq![text.value@]
                    + (seq![seq![0x0au8]]
                        + doc_clauses_parts(e.0.clauses) + e.1));
        }
    }
    let ghost after_text = match expected@ {
        Some(e) => Some(
            seq![seq![0x0au8]] + doc_clauses_parts(e.0.clauses) + e.1,
        ),
        None => None,
    };
    if !doc_guided_literal(
        bytes,
        guided,
        &text.value,
        Ghost(text.value@),
        Ghost(after_text),
    at,
    ) {
        return None;
    }
    let newline: &[u8] = b"\n";
    let ghost newline_chunk = seq![0x0au8];
    proof {
        reveal_byteslit(b"\n");
        assert(newline@ == newline_chunk);
        if let Some(e) = expected@ {
            assert(guide_rest(guided.guide@.unwrap())
                == seq![newline_chunk]
                    + (doc_clauses_parts(e.0.clauses) + e.1));
        }
    }
    let ghost after_newline = match expected@ {
        Some(e) => Some(doc_clauses_parts(e.0.clauses) + e.1),
        None => None,
    };
    if !doc_guided_literal(
        bytes,
        guided,
        newline,
        Ghost(newline_chunk),
        Ghost(after_newline),
    at,
    ) {
        return None;
    }

    let ghost mut clauses: Seq<ckc_spec::v1text::DocClause> = Seq::empty();
    let mut more = true;
    proof {
        reveal(ckc_spec::v1text::marker_line);
        reveal_with_fuel(ckc_spec::v1text::clauses_bytes, 2);
        assert_seqs_equal!(guided.cursor.prefix@
            == entry_prefix
                + ckc_spec::v1text::marker_line(
                    decimal.value@,
                    text.value@,
                )
                + ckc_spec::v1text::clauses_bytes(clauses));
        assert forall|i: int| 0 <= i < clauses.len()
            implies ckc_spec::v1text::wf_clause(clauses[i]) by {
            assert(false);
        }
        if let Some(e) = expected@ {
            reveal(ckc_spec::v1text::wf_bundle);
            assert(e.0.clauses.len() >= 1);
            assert_seqs_equal!(e.0.clauses.take(0)
                == Seq::<ckc_spec::v1text::DocClause>::empty());
            assert_seqs_equal!(e.0.clauses.skip(0) == e.0.clauses);
            assert(clauses == e.0.clauses.take(clauses.len() as int));
            assert(guide_rest(guided.guide@.unwrap())
                == doc_clauses_parts(e.0.clauses.skip(clauses.len() as int))
                    + e.1);
        }
    }
    while more
        invariant
            *old(at) <= *at <= bytes@.len(),
            guided_cursor_ok(bytes@, &guided),
            entry_pos < guided.cursor.pos,
            canonical_decimal(ordinal@),
            ordinal@ == ckc_spec::v1text::udec_bytes(decimal.value@),
            decimal.value@ >= 1,
            ckc_spec::v1text::text_ok(text.value@),
            guided.cursor.prefix@
                == entry_prefix
                    + ckc_spec::v1text::marker_line(
                        decimal.value@,
                        text.value@,
                    )
                    + ckc_spec::v1text::clauses_bytes(clauses),
            forall|i: int| 0 <= i < clauses.len()
                ==> ckc_spec::v1text::wf_clause(clauses[i]),
            !more ==> clauses.len() >= 1,
            expected@ matches Some(e) ==> {
                &&& ckc_spec::v1text::wf_bundle(e.0)
                &&& (e.1.len() == 0
                    || e.1[0] == ckc_spec::v1text::ascii("% S"@))
                &&& clauses == e.0.clauses.take(clauses.len() as int)
                &&& clauses.len() <= e.0.clauses.len()
                &&& more ==> clauses.len() < e.0.clauses.len()
                &&& !more ==> clauses.len() == e.0.clauses.len()
                &&& guided.guide@ matches Some(g)
                    && guide_rest(g) == if more {
                        doc_clauses_parts(
                            e.0.clauses.skip(clauses.len() as int),
                        ) + e.1
                    } else {
                        e.1
                    }
            },
            expected@ is None ==> guided.guide@ is None,
            !more ==> guided.cursor.pos == bytes@.len()
                || guided.cursor.pos < bytes@.len()
                    && bytes@[guided.cursor.pos as int] == 0x25,
        decreases if more {
            bytes.len() - guided.cursor.pos + 1
        } else {
            0
        },
    {
        let ghost clause_expected = match expected@ {
            Some(e) => {
                let remaining = e.0.clauses.skip(clauses.len() as int);
                Some((
                    remaining[0],
                    doc_clauses_parts(remaining.drop_first()) + e.1,
                ))
            },
            None => None,
        };
        proof {
            if let Some(e) = expected@ {
                reveal(ckc_spec::v1text::wf_bundle);
                let remaining = e.0.clauses.skip(clauses.len() as int);
                assert(0 <= clauses.len() < e.0.clauses.len());
                assert(remaining.len() > 0);
                assert(remaining[0]
                    == e.0.clauses[clauses.len() as int]);
                assert(ckc_spec::v1text::wf_clause(remaining[0]));
                reveal(doc_clauses_parts);
                assert(guide_rest(guided.guide@.unwrap())
                    == doc_clause_parts(remaining[0])
                        + clause_expected.unwrap().1);
            }
        }
        let ghost old_clauses = clauses;
        let ghost before_clause_prefix = guided.cursor.prefix@;
        let clause = match parse_doc_clause(
            bytes,
            guided,
            Ghost(clause_expected),
        at,
        ) {
            Some(clause) => clause,
            None => return None,
        };
        proof {
            clauses = old_clauses.push(clause@);
            assert(clauses.len() == old_clauses.len() + 1);
            assert(clauses.len() >= 1);
            doc_clause_parts_flat(clause@);
            clauses_bytes_push(old_clauses, clause@);
            assert_seqs_equal!(guided.cursor.prefix@
                == entry_prefix
                    + ckc_spec::v1text::marker_line(
                        decimal.value@,
                        text.value@,
                    )
                    + ckc_spec::v1text::clauses_bytes(clauses));
            assert forall|i: int| 0 <= i < clauses.len()
                implies ckc_spec::v1text::wf_clause(clauses[i]) by {
                if i < old_clauses.len() {
                    assert(clauses[i] == old_clauses[i]);
                } else {
                    assert(i == old_clauses.len());
                    assert(clauses[i] == clause@);
                }
            }
            if let Some(e) = expected@ {
                assert(clause@ == e.0.clauses[old_clauses.len() as int]);
                assert_seqs_equal!(
                    e.0.clauses.take(old_clauses.len() as int).push(clause@)
                        == e.0.clauses.take(clauses.len() as int)
                );
                assert(clauses
                    == e.0.clauses.take(clauses.len() as int));
                assert_seqs_equal!(
                    e.0.clauses.skip(old_clauses.len() as int).drop_first()
                        == e.0.clauses.skip(clauses.len() as int)
                );
                assert(guide_rest(guided.guide@.unwrap())
                    == doc_clauses_parts(
                        e.0.clauses.skip(clauses.len() as int),
                    ) + e.1);
            }
        }
        if guided.cursor.pos == bytes.len() {
            proof {
                if let Some(e) = expected@ {
                    if clauses.len() < e.0.clauses.len() {
                        let remaining = e.0.clauses.skip(clauses.len() as int);
                        let next = remaining[0];
                        reveal(doc_clauses_parts);
                        wf_literal_term_prefix_safe(next.head);
                        assert(guide_rest(guided.guide@.unwrap())
                            == doc_clause_parts(next)
                                + doc_clauses_parts(remaining.drop_first()) + e.1);
                        assert_seqs_equal!(guide_rest(guided.guide@.unwrap())
                            == seq![ckc_spec::v1text::term_bytes(next.head)]
                                + (doc_clause_parts(next).drop_first()
                                    + doc_clauses_parts(remaining.drop_first()) + e.1));
                        guide_front_bytes(
                            bytes@,
                            &guided,
                            ckc_spec::v1text::term_bytes(next.head),
                            doc_clause_parts(next).drop_first()
                                + doc_clauses_parts(remaining.drop_first()) + e.1,
                        );
                        assert(guided.cursor.pos < bytes.len());
                        assert(false);
                    }
                    assert(clauses.len() == e.0.clauses.len());
                    assert(clauses == e.0.clauses);
                    assert(guide_rest(guided.guide@.unwrap()) == e.1);
                    if e.1.len() > 0 {
                        assert(e.1[0] == ckc_spec::v1text::ascii("% S"@));
                        assert_seqs_equal!(e.1
                            == seq![e.1[0]] + e.1.drop_first());
                        guide_front_bytes(
                            bytes@,
                            &guided,
                            e.1[0],
                            e.1.drop_first(),
                        );
                        reveal_strlit("% S");
                        reveal(ckc_spec::v1text::ascii);
                        assert(e.1[0].len() > 0);
                        assert(guided.cursor.pos < bytes.len());
                        assert(false);
                    }
                    assert(e.1.len() == 0);
                }
            }
            more = false;
        } else if bytes[guided.cursor.pos] == 0x25 {
            proof {
                if let Some(e) = expected@ {
                    if clauses.len() < e.0.clauses.len() {
                        let remaining = e.0.clauses.skip(clauses.len() as int);
                        let next = remaining[0];
                        reveal(doc_clauses_parts);
                        wf_literal_term_prefix_safe(next.head);
                        assert(guide_rest(guided.guide@.unwrap())
                            == doc_clause_parts(next)
                                + doc_clauses_parts(remaining.drop_first()) + e.1);
                        assert(doc_clause_parts(next).len() > 0);
                        assert(doc_clause_parts(next)[0]
                            == ckc_spec::v1text::term_bytes(next.head));
                        assert_seqs_equal!(guide_rest(guided.guide@.unwrap())
                            == seq![ckc_spec::v1text::term_bytes(next.head)]
                                + (doc_clause_parts(next).drop_first()
                                    + doc_clauses_parts(remaining.drop_first()) + e.1));
                        guide_front_bytes(
                            bytes@,
                            &guided,
                            ckc_spec::v1text::term_bytes(next.head),
                            doc_clause_parts(next).drop_first()
                                + doc_clauses_parts(remaining.drop_first()) + e.1,
                        );
                        assert(bytes@[guided.cursor.pos as int]
                            == ckc_spec::v1text::term_bytes(next.head)[0]);
                        assert(false);
                    }
                    assert(clauses.len() == e.0.clauses.len());
                    assert(clauses == e.0.clauses);
                }
            }
            more = false;
        } else {
            proof {
                if let Some(e) = expected@ {
                    if clauses.len() == e.0.clauses.len() {
                        assert(guide_rest(guided.guide@.unwrap()) == e.1);
                        if e.1.len() == 0 {
                            let g = guided.guide@.unwrap();
                            assert(guide_rest(g) == Seq::<Seq<u8>>::empty());
                            reveal(guide_rest);
                            assert(g.parts.subrange(
                                g.index,
                                g.parts.len() as int,
                            ).len() == g.parts.len() - g.index);
                            assert(g.index == g.parts.len());
                            reveal(guided_cursor_ok);
                            reveal(parts_progress);
                            reveal(cursor_ok);
                            g.parts.lemma_take_len();
                            assert(guided.cursor.prefix@ == g.parts.flatten());
                            assert(guided.cursor.prefix@ == bytes@);
                            assert(guided.cursor.pos == bytes.len());
                            assert(false);
                        }
                        assert(e.1[0] == ckc_spec::v1text::ascii("% S"@));
                        assert_seqs_equal!(e.1 == seq![e.1[0]] + e.1.drop_first());
                        guide_front_bytes(
                            bytes@,
                            &guided,
                            e.1[0],
                            e.1.drop_first(),
                        );
                        reveal_strlit("% S");
                        reveal(ckc_spec::v1text::ascii);
                        assert(e.1[0].len() > 0);
                        assert(bytes@.subrange(
                            guided.cursor.pos as int,
                            guided.cursor.pos as int + e.1[0].len(),
                        ) == e.1[0]);
                        assert(bytes@[guided.cursor.pos as int] == e.1[0][0]);
                        assert(e.1[0][0] == 0x25);
                        assert(bytes@[guided.cursor.pos as int] == 0x25);
                        assert(false);
                    }
                    assert(clauses.len() < e.0.clauses.len());
                }
            }
        }
    }

    let ghost model = ckc_spec::v1text::Bundle {
        s: decimal.value@,
        text: text.value@,
        clauses,
    };
    proof {
        reveal(ckc_spec::v1text::wf_bundle);
        assert(model.s >= 1);
        assert(model.clauses.len() >= 1);
        assert forall|i: int| 0 <= i < model.clauses.len()
            implies ckc_spec::v1text::wf_clause(model.clauses[i]) by {
            assert(ckc_spec::v1text::wf_clause(clauses[i]));
        }
        assert(ckc_spec::v1text::wf_bundle(model));
        assert(entry_pos == old(guided).cursor.pos);
        assert(entry_pos < guided.cursor.pos);
        doc_bundle_parts_flat(model);
        assert_seqs_equal!(guided.cursor.prefix@
            == entry_prefix + doc_bundle_parts(model).flatten());
        if let Some(e) = expected@ {
            assert(decimal.value@ == e.0.s);
            assert(text.value@ == e.0.text);
            assert(clauses == e.0.clauses);
            assert(model == e.0);
        }
    }
    Some(EDocBundle {
        bundle: Ghost(model),
        ordinal,
    })
}

pub open spec fn pow10(n: nat) -> nat
    decreases n,
{
    if n == 0 { 1 } else { 10 * pow10((n - 1) as nat) }
}

proof fn pow10_positive(n: nat)
    ensures pow10(n) >= 1,
    decreases n,
{
    reveal_with_fuel(pow10, 2);
    if n > 0 {
        pow10_positive((n - 1) as nat);
    }
}

proof fn pow10_monotonic(a: nat, b: nat)
    requires a <= b,
    ensures pow10(a) <= pow10(b),
    decreases b - a,
{
    if a < b {
        pow10_monotonic(a, (b - 1) as nat);
        pow10_positive((b - 1) as nat);
        reveal_with_fuel(pow10, 2);
        assert(pow10((b - 1) as nat) <= 10 * pow10((b - 1) as nat)) by (nonlinear_arith);
    }
}

proof fn decimal_all_drop_first(s: Seq<u8>)
    requires
        s.len() > 0,
        ckc_spec::v1text::all_in(
            s,
            |b: u8| ckc_spec::v1text::is_digit_b(b),
        ),
    ensures
        ckc_spec::v1text::all_in(
            s.drop_first(),
            |b: u8| ckc_spec::v1text::is_digit_b(b),
        ),
{
    reveal(ckc_spec::v1text::all_in);
    assert forall|i: int| 0 <= i < s.drop_first().len()
        implies ckc_spec::v1text::is_digit_b(s.drop_first()[i]) by {
        assert(s.drop_first()[i] == s[i + 1]);
    }
}

proof fn decimal_value_bound(s: Seq<u8>)
    requires
        ckc_spec::v1text::all_in(
            s,
            |b: u8| ckc_spec::v1text::is_digit_b(b),
        ),
    ensures decimal_value(s) < pow10(s.len()),
    decreases s.len(),
{
    if s.len() == 0 {
        reveal_with_fuel(decimal_value, 2);
        reveal_with_fuel(pow10, 2);
    } else {
        decimal_all_drop_last(s);
        decimal_value_bound(s.drop_last());
        decimal_digit_bounds(s.last());
        reveal_with_fuel(decimal_value, 2);
        reveal_with_fuel(pow10, 2);
        let v = decimal_value(s.drop_last());
        let d = decimal_digit(s.last());
        let p = pow10((s.len() - 1) as nat);
        assert(s.drop_last().len() == s.len() - 1);
        assert(v < p);
        assert(d < 10);
        assert(decimal_value(s) == v * 10 + d);
        assert(pow10(s.len()) == 10 * p);
        assert(v * 10 + d < 10 * p);
    }
}

proof fn decimal_value_prepend(first: u8, rest: Seq<u8>)
    requires
        ckc_spec::v1text::is_digit_b(first),
        ckc_spec::v1text::all_in(
            rest,
            |b: u8| ckc_spec::v1text::is_digit_b(b),
        ),
    ensures
        decimal_value(seq![first] + rest)
            == decimal_digit(first) * pow10(rest.len())
                + decimal_value(rest),
    decreases rest.len(),
{
    decimal_digit_bounds(first);
    if rest.len() == 0 {
        reveal_with_fuel(decimal_value, 2);
        reveal_with_fuel(pow10, 2);
        assert_seqs_equal!((seq![first] + rest).drop_last() == Seq::<u8>::empty());
        assert((seq![first] + rest).last() == first);
        assert(decimal_value(Seq::<u8>::empty()) == 0);
        assert(decimal_value(seq![first] + rest) == decimal_digit(first));
        assert(pow10(rest.len()) == 1);
        assert(decimal_value(rest) == 0);
        assert(decimal_digit(first) * pow10(rest.len()) == decimal_digit(first))
            by (nonlinear_arith)
            requires pow10(rest.len()) == 1;
    } else {
        decimal_all_drop_last(rest);
        decimal_value_prepend(first, rest.drop_last());
        decimal_digit_bounds(rest.last());
        let whole = seq![first] + rest;
        let shorter = seq![first] + rest.drop_last();
        assert_seqs_equal!(whole.drop_last() == shorter);
        assert(whole.len() == rest.len() + 1);
        assert(whole.last() == rest.last());
        reveal_with_fuel(decimal_value, 2);
        reveal_with_fuel(pow10, 2);
        let q = pow10((rest.len() - 1) as nat);
        let df = decimal_digit(first);
        let w = decimal_value(rest.drop_last());
        let dl = decimal_digit(rest.last());
        assert(rest.drop_last().len() == rest.len() - 1);
        assert(decimal_value(shorter) == df * q + w);
        assert(decimal_value(whole) == decimal_value(shorter) * 10 + dl);
        assert(pow10(rest.len()) == 10 * q);
        assert(decimal_value(rest) == w * 10 + dl);
        assert((df * q + w) * 10 + dl == df * (10 * q) + (w * 10 + dl))
            by (nonlinear_arith);
        assert(decimal_value(whole) == df * pow10(rest.len()) + decimal_value(rest))
            by (nonlinear_arith)
            requires
                decimal_value(whole) == (df * q + w) * 10 + dl,
                pow10(rest.len()) == 10 * q,
                decimal_value(rest) == w * 10 + dl;
    }
}

proof fn decimal_nonzero_leading_min(s: Seq<u8>)
    requires
        s.len() > 0,
        ckc_spec::v1text::all_in(
            s,
            |b: u8| ckc_spec::v1text::is_digit_b(b),
        ),
        s[0] != 0x30,
    ensures pow10((s.len() - 1) as nat) <= decimal_value(s),
{
    let rest = s.drop_first();
    decimal_all_drop_first(s);
    reveal(ckc_spec::v1text::all_in);
    assert(ckc_spec::v1text::is_digit_b(s[0]));
    decimal_digit_bounds(s[0]);
    decimal_value_prepend(s[0], rest);
    assert_seqs_equal!(s == seq![s[0]] + rest);
    reveal(decimal_digit);
    reveal(ckc_spec::v1text::is_digit_b);
    let d = decimal_digit(s[0]);
    let p = pow10(rest.len());
    assert(d >= 1);
    assert(rest.len() == s.len() - 1);
    pow10_positive(rest.len());
    assert(p <= d * p) by (nonlinear_arith)
        requires d >= 1, p >= 0;
    assert(decimal_value(s) == d * p + decimal_value(rest));
}

proof fn decimal_shorter_less(a: Seq<u8>, b: Seq<u8>)
    requires
        canonical_decimal(a),
        canonical_decimal(b),
        a.len() < b.len(),
    ensures decimal_value(a) < decimal_value(b),
{
    reveal(canonical_decimal);
    decimal_value_bound(a);
    assert(b.len() > 1);
    assert(b[0] != 0x30);
    decimal_nonzero_leading_min(b);
    assert(a.len() <= b.len() - 1);
    pow10_monotonic(a.len(), (b.len() - 1) as nat);
}

pub open spec fn decimal_lex_lt(a: Seq<u8>, b: Seq<u8>) -> bool
    decreases a.len(),
{
    if a.len() == 0 || b.len() == 0 {
        false
    } else if decimal_digit(a[0]) < decimal_digit(b[0]) {
        true
    } else if decimal_digit(a[0]) > decimal_digit(b[0]) {
        false
    } else {
        decimal_lex_lt(a.drop_first(), b.drop_first())
    }
}

proof fn decimal_lex_value(a: Seq<u8>, b: Seq<u8>)
    requires
        a.len() == b.len(),
        ckc_spec::v1text::all_in(
            a,
            |x: u8| ckc_spec::v1text::is_digit_b(x),
        ),
        ckc_spec::v1text::all_in(
            b,
            |x: u8| ckc_spec::v1text::is_digit_b(x),
        ),
    ensures
        decimal_lex_lt(a, b) <==>
            decimal_value(a) < decimal_value(b),
    decreases a.len(),
{
    reveal_with_fuel(decimal_lex_lt, 2);
    if a.len() == 0 {
        reveal_with_fuel(decimal_value, 2);
    } else {
        let ar = a.drop_first();
        let br = b.drop_first();
        decimal_all_drop_first(a);
        decimal_all_drop_first(b);
        decimal_lex_value(ar, br);
        reveal(ckc_spec::v1text::all_in);
        assert(ckc_spec::v1text::is_digit_b(a[0]));
        assert(ckc_spec::v1text::is_digit_b(b[0]));
        decimal_value_prepend(a[0], ar);
        decimal_value_prepend(b[0], br);
        assert_seqs_equal!(a == seq![a[0]] + ar);
        assert_seqs_equal!(b == seq![b[0]] + br);
        decimal_value_bound(ar);
        decimal_value_bound(br);
        decimal_digit_bounds(a[0]);
        decimal_digit_bounds(b[0]);
        assert(ar.len() == br.len());
        let p = pow10(ar.len());
        pow10_positive(ar.len());
        let da = decimal_digit(a[0]);
        let db = decimal_digit(b[0]);
        let va = decimal_value(ar);
        let vb = decimal_value(br);
        assert(decimal_value(a) == da * p + va);
        assert(decimal_value(b) == db * p + vb);
        assert(va < p);
        assert(vb < p);
        if da < db {
            assert(da * p + va < db * p + vb) by (nonlinear_arith)
                requires da + 1 <= db, va < p, vb >= 0, p >= 0;
            assert(decimal_lex_lt(a, b));
        } else if da > db {
            assert(db * p + vb < da * p + va) by (nonlinear_arith)
                requires db + 1 <= da, vb < p, va >= 0, p >= 0;
            assert(!decimal_lex_lt(a, b));
        } else {
            assert(da == db);
            assert(decimal_lex_lt(a, b) == decimal_lex_lt(ar, br));
        }
    }
}

proof fn decimal_lex_difference(a: Seq<u8>, b: Seq<u8>, i: nat)
    requires
        a.len() == b.len(),
        ckc_spec::v1text::all_in(
            a,
            |x: u8| ckc_spec::v1text::is_digit_b(x),
        ),
        ckc_spec::v1text::all_in(
            b,
            |x: u8| ckc_spec::v1text::is_digit_b(x),
        ),
        i < a.len(),
        forall|j: int| 0 <= j < i ==> a[j] == b[j],
        a[i as int] != b[i as int],
    ensures
        decimal_lex_lt(a, b) ==
            (decimal_digit(a[i as int]) < decimal_digit(b[i as int])),
    decreases i,
{
    reveal_with_fuel(decimal_lex_lt, 2);
    reveal(ckc_spec::v1text::all_in);
    reveal(decimal_digit);
    reveal(ckc_spec::v1text::is_digit_b);
    if i == 0 {
        assert(ckc_spec::v1text::is_digit_b(a[0]));
        assert(ckc_spec::v1text::is_digit_b(b[0]));
        assert(decimal_digit(a[0]) != decimal_digit(b[0]));
    } else {
        assert(a[0] == b[0]);
        assert(decimal_digit(a[0]) == decimal_digit(b[0]));
        decimal_all_drop_first(a);
        decimal_all_drop_first(b);
        assert(a.drop_first().len() == b.drop_first().len());
        assert(i - 1 < a.drop_first().len());
        assert forall|j: int| 0 <= j < i - 1
            implies a.drop_first()[j] == b.drop_first()[j] by {
            assert(a.drop_first()[j] == a[j + 1]);
            assert(b.drop_first()[j] == b[j + 1]);
        }
        assert(a.drop_first()[(i - 1) as int] == a[i as int]);
        assert(b.drop_first()[(i - 1) as int] == b[i as int]);
        decimal_lex_difference(a.drop_first(), b.drop_first(), (i - 1) as nat);
        assert(decimal_lex_lt(a, b)
            == decimal_lex_lt(a.drop_first(), b.drop_first()));
    }
}

proof fn decimal_lex_irreflexive(a: Seq<u8>)
    ensures !decimal_lex_lt(a, a),
    decreases a.len(),
{
    reveal(decimal_lex_lt);
    if a.len() > 0 {
        decimal_lex_irreflexive(a.drop_first());
    }
}

fn decimal_bytes_less(a: &Vec<u8>, b: &Vec<u8>) -> (r: bool)
    requires
        canonical_decimal(a@),
        canonical_decimal(b@),
    ensures r == (decimal_value(a@) < decimal_value(b@)),
{
    if a.len() < b.len() {
        proof { decimal_shorter_less(a@, b@); }
        return true;
    }
    if a.len() > b.len() {
        proof {
            decimal_shorter_less(b@, a@);
            assert(!(decimal_value(a@) < decimal_value(b@)));
        }
        return false;
    }
    let mut i = 0usize;
    while i < a.len() && a[i] == b[i]
        invariant
            a@.len() == b@.len(),
            i <= a@.len(),
            forall|j: int| 0 <= j < i ==> a@[j] == b@[j],
        decreases a.len() - i,
    {
        i += 1;
    }
    proof {
        reveal(canonical_decimal);
        decimal_lex_value(a@, b@);
    }
    if i == a.len() {
        proof {
            assert_seqs_equal!(a@ == b@);
            decimal_lex_irreflexive(a@);
        }
        false
    } else {
        proof {
            reveal(canonical_decimal);
            reveal(ckc_spec::v1text::all_in);
            assert(a@[i as int] != b@[i as int]);
            decimal_lex_difference(a@, b@, i as nat);
            reveal(decimal_digit);
            reveal(ckc_spec::v1text::is_digit_b);
            assert(ckc_spec::v1text::is_digit_b(a@[i as int]));
            assert(ckc_spec::v1text::is_digit_b(b@[i as int]));
            assert((a@[i as int] < b@[i as int])
                == (decimal_digit(a@[i as int])
                    < decimal_digit(b@[i as int])));
        }
        a[i] < b[i]
    }
}

proof fn bundles_bytes_push(
    bundles: Seq<ckc_spec::v1text::Bundle>,
    bundle: ckc_spec::v1text::Bundle,
)
    ensures
        ckc_spec::v1text::bundles_bytes(bundles.push(bundle))
            == ckc_spec::v1text::bundles_bytes(bundles)
                + ckc_spec::v1text::marker_line(bundle.s, bundle.text)
                + ckc_spec::v1text::clauses_bytes(bundle.clauses),
    decreases bundles.len(),
{
    if bundles.len() == 0 {
        assert_seqs_equal!(bundles.push(bundle) == seq![bundle]);
        reveal_with_fuel(ckc_spec::v1text::bundles_bytes, 3);
    } else {
        bundles_bytes_push(bundles.drop_first(), bundle);
        assert_seqs_equal!(bundles.push(bundle).drop_first()
            == bundles.drop_first().push(bundle));
        reveal_with_fuel(ckc_spec::v1text::bundles_bytes, 2);
        assert_seqs_equal!(
            (ckc_spec::v1text::marker_line(bundles[0].s, bundles[0].text)
                + ckc_spec::v1text::clauses_bytes(bundles[0].clauses))
                + (ckc_spec::v1text::bundles_bytes(bundles.drop_first())
                    + ckc_spec::v1text::marker_line(bundle.s, bundle.text)
                    + ckc_spec::v1text::clauses_bytes(bundle.clauses))
            == ((ckc_spec::v1text::marker_line(
                    bundles[0].s,
                    bundles[0].text,
                ) + ckc_spec::v1text::clauses_bytes(bundles[0].clauses))
                    + ckc_spec::v1text::bundles_bytes(bundles.drop_first()))
                + ckc_spec::v1text::marker_line(bundle.s, bundle.text)
                + ckc_spec::v1text::clauses_bytes(bundle.clauses)
        );
    }
}

proof fn bundles_push_wf(
    bundles: Seq<ckc_spec::v1text::Bundle>,
    bundle: ckc_spec::v1text::Bundle,
)
    requires
        forall|i: int| 0 <= i < bundles.len()
            ==> #[trigger] ckc_spec::v1text::wf_bundle(bundles[i]),
        ckc_spec::v1text::wf_bundle(bundle),
    ensures
        forall|i: int| 0 <= i < bundles.push(bundle).len()
            ==> #[trigger] ckc_spec::v1text::wf_bundle(
                bundles.push(bundle)[i],
            ),
{
    assert forall|i: int| 0 <= i < bundles.push(bundle).len()
        implies ckc_spec::v1text::wf_bundle(bundles.push(bundle)[i]) by {
        if i < bundles.len() {
            assert(bundles.push(bundle)[i] == bundles[i]);
        } else {
            assert(i == bundles.len());
            assert(bundles.push(bundle)[i] == bundle);
        }
    }
}

proof fn bundles_push_ordered(
    bundles: Seq<ckc_spec::v1text::Bundle>,
    bundle: ckc_spec::v1text::Bundle,
)
    requires
        forall|i: int| 0 <= i < bundles.len() - 1
            ==> #[trigger] bundles[i].s < bundles[i + 1].s,
        bundles.len() > 0 ==> bundles.last().s < bundle.s,
    ensures
        forall|i: int| 0 <= i < bundles.push(bundle).len() - 1
            ==> #[trigger] bundles.push(bundle)[i].s
                < bundles.push(bundle)[i + 1].s,
{
    assert forall|i: int| 0 <= i < bundles.push(bundle).len() - 1
        implies #[trigger] bundles.push(bundle)[i].s
            < bundles.push(bundle)[i + 1].s by {
        if i < bundles.len() - 1 {
            assert(bundles.push(bundle)[i] == bundles[i]);
            assert(bundles.push(bundle)[i + 1] == bundles[i + 1]);
        } else {
            assert(i == bundles.len() - 1);
            assert(bundles.len() > 0);
            assert(bundles.push(bundle)[i] == bundles.last());
            assert(bundles.push(bundle)[i + 1] == bundle);
        }
    }
}

proof fn wf_doc_bundle_at(d: ckc_spec::v1text::DocFile, i: int)
    requires
        ckc_spec::v1text::wf_doc(d),
        0 <= i < d.bundles.len(),
    ensures
        ckc_spec::v1text::wf_bundle(d.bundles[i]),
{
    reveal(ckc_spec::v1text::wf_doc);
}

proof fn wf_doc_ordered_at(d: ckc_spec::v1text::DocFile, i: int)
    requires
        ckc_spec::v1text::wf_doc(d),
        0 <= i < d.bundles.len() - 1,
    ensures
        d.bundles[i].s < d.bundles[i + 1].s,
{
    reveal(ckc_spec::v1text::wf_doc);
}

proof fn wf_doc_nonempty(d: ckc_spec::v1text::DocFile)
    requires ckc_spec::v1text::wf_doc(d),
    ensures d.bundles.len() >= 1,
{
    reveal(ckc_spec::v1text::wf_doc);
}

proof fn wf_doc_intro(d: ckc_spec::v1text::DocFile)
    requires
        ckc_spec::v1text::name_ok(d.docid),
        ckc_spec::v1text::hex64(d.ace),
        ckc_spec::v1text::ulex_ok(d.ulex),
        d.bundles.len() >= 1,
        forall|i: int| 0 <= i < d.bundles.len()
            ==> #[trigger] ckc_spec::v1text::wf_bundle(d.bundles[i]),
        forall|i: int| 0 <= i < d.bundles.len() - 1
            ==> #[trigger] d.bundles[i].s < d.bundles[i + 1].s,
    ensures ckc_spec::v1text::wf_doc(d),
{
    reveal(ckc_spec::v1text::wf_doc);
}

#[verifier::rlimit(100)]
#[verifier::spinoff_prover]
pub fn parse_doc(
    bytes: &[u8],
    expected: Ghost<Option<ckc_spec::v1text::DocFile>>,
at: &mut usize,
) -> (r: Option<EParsedV1>)
    requires
        *old(at) <= bytes@.len(),
        expected@ matches Some(d) ==>
            ckc_spec::v1text::wf_doc(d)
                && ckc_spec::v1text::print_doc(d) == bytes@,
    ensures
        *old(at) <= *final(at) <= bytes@.len(),
        r matches Some(parsed) ==> parsed_v1_ok(bytes@, &parsed),
        expected@ matches Some(d) ==> r matches Some(parsed)
            && parsed@ == ckc_spec::v1text::V1File::Doc(d),
{
    hide(ckc_spec::v1text::wf_doc);
    let ghost expected_parts = match expected@ {
        Some(d) => Some(doc_parts(d)),
        None => None,
    };
    proof {
        if let Some(d) = expected@ {
            doc_flat_is_print(d);
            doc_parts_flat(d);
            assert(bytes@ == doc_parts(d).flatten());
        }
    }
    let mut guided = new_guided_cursor(bytes, Ghost(expected_parts));
    let docid = match parse_doc_line(bytes, &mut guided, expected, at) {
        Some(docid) => docid,
        None => return None,
    };
    if !parse_doc_declarations(bytes, &mut guided, expected, at) {
        return None;
    }
    let ace = match parse_doc_record_prefix(
        bytes,
        &mut guided,
        &docid,
        expected,
    at,
    ) {
        Some(ace) => ace,
        None => return None,
    };
    let ulex = match parse_doc_ulex(bytes, &mut guided, expected, at) {
        Some(ulex) => ulex,
        None => return None,
    };

    let ghost base = ckc_spec::v1text::DocFile {
        docid: docid.value@,
        ace: ace.name@,
        ulex: ulex.value@,
        bundles: Seq::empty(),
    };
    let ghost mut bundles: Seq<ckc_spec::v1text::Bundle> = Seq::empty();
    let mut bundle_count = 0usize;
    let mut previous_ordinal = Vec::<u8>::new();
    proof {
        reveal(doc_prefix_stage);
        reveal(doc_record_stage);
        reveal(doc_record_suffix_stage);
        reveal_with_fuel(ckc_spec::v1text::bundles_bytes, 2);
        assert_seqs_equal!(guided.cursor.prefix@
            == doc_prefix_stage(base)
                + ckc_spec::v1text::bundles_bytes(bundles));
        assert forall|i: int| 0 <= i < bundles.len()
            implies ckc_spec::v1text::wf_bundle(bundles[i]) by {
            assert(false);
        }
        assert forall|i: int| 0 <= i < bundles.len() - 1
            implies #[trigger] bundles[i].s < bundles[i + 1].s by {
            assert(false);
        }
        if let Some(d) = expected@ {
            assert_seqs_equal!(d.bundles.take(0)
                == Seq::<ckc_spec::v1text::Bundle>::empty());
            assert_seqs_equal!(d.bundles.skip(0) == d.bundles);
            assert(bundles == d.bundles.take(bundles.len() as int));
        }
    }
    let bundle_at_floor = *at;
    while guided.cursor.pos < bytes.len()
        invariant
            *old(at) <= bundle_at_floor <= bytes@.len(),
            guided_cursor_ok(bytes@, &guided),
            ckc_spec::v1text::name_ok(docid.value@),
            ckc_spec::v1text::hex64(ace.name@),
            ckc_spec::v1text::ulex_ok(ulex.value@),
            bundle_count == bundles.len(),
            bundle_count <= guided.cursor.pos,
            guided.cursor.prefix@
                == doc_prefix_stage(base)
                    + ckc_spec::v1text::bundles_bytes(bundles),
            forall|i: int| 0 <= i < bundles.len()
                ==> ckc_spec::v1text::wf_bundle(bundles[i]),
            forall|i: int| 0 <= i < bundles.len() - 1
                ==> #[trigger] bundles[i].s < bundles[i + 1].s,
            bundle_count == 0 ==> previous_ordinal@ == Seq::<u8>::empty(),
            bundle_count > 0 ==> {
                &&& canonical_decimal(previous_ordinal@)
                &&& previous_ordinal@
                    == ckc_spec::v1text::udec_bytes(bundles.last().s)
            },
            expected@ matches Some(d) ==> {
                &&& ckc_spec::v1text::wf_doc(d)
                &&& base.docid == d.docid
                &&& base.ace == d.ace
                &&& base.ulex == d.ulex
                &&& bundles == d.bundles.take(bundles.len() as int)
                &&& bundles.len() <= d.bundles.len()
                &&& guided.guide@ matches Some(g)
                    && guide_rest(g)
                        == doc_bundles_parts(
                            d.bundles.skip(bundles.len() as int),
                        )
            },
            expected@ is None ==> guided.guide@ is None,
        decreases bytes.len() - guided.cursor.pos,
    {
        let ghost bundle_expected = match expected@ {
            Some(d) => {
                let remaining = d.bundles.skip(bundles.len() as int);
                Some((
                    remaining[0],
                    doc_bundles_parts(remaining.drop_first()),
                ))
            },
            None => None,
        };
        proof {
            if let Some(d) = expected@ {
                let remaining = d.bundles.skip(bundles.len() as int);
                if bundles.len() == d.bundles.len() {
                    assert(remaining.len() == 0);
                    reveal(doc_bundles_parts);
                    assert(guide_rest(guided.guide@.unwrap())
                        == Seq::<Seq<u8>>::empty());
                    let g = guided.guide@.unwrap();
                    reveal(guide_rest);
                    assert(g.parts.subrange(g.index, g.parts.len() as int).len()
                        == g.parts.len() - g.index);
                    assert(g.index == g.parts.len());
                    reveal(guided_cursor_ok);
                    reveal(parts_progress);
                    reveal(cursor_ok);
                    g.parts.lemma_take_len();
                    assert(guided.cursor.prefix@ == g.parts.flatten());
                    assert(guided.cursor.prefix@ == bytes@);
                    assert(guided.cursor.pos == bytes.len());
                    assert(false);
                }
                assert(bundles.len() < d.bundles.len());
                assert(remaining.len() > 0);
                assert(remaining[0] == d.bundles[bundles.len() as int]);
                wf_doc_bundle_at(d, bundles.len() as int);
                assert(ckc_spec::v1text::wf_bundle(remaining[0]));
                reveal(doc_bundles_parts);
                assert(guide_rest(guided.guide@.unwrap())
                    == doc_bundle_parts(remaining[0])
                        + doc_bundles_parts(remaining.drop_first()));
                if remaining.drop_first().len() > 0 {
                    reveal(doc_bundles_parts);
                    reveal(doc_bundle_parts);
                    reveal(doc_marker_parts);
                    assert(doc_bundles_parts(remaining.drop_first())[0]
                        == ckc_spec::v1text::ascii("% S"@));
                }
            }
        }
        let ghost old_bundles = bundles;
        let old_count = bundle_count;
        let mut bundle_at = bundle_at_floor;
        let bundle = match parse_doc_bundle(
            bytes,
            &mut guided,
            Ghost(bundle_expected),
            &mut bundle_at,
        ) {
            Some(bundle) => bundle,
            None => {
                *at = bundle_at;
                return None;
            },
        };
        let ghost bundle_model = bundle@;
        let ordered = if old_count == 0 {
            true
        } else {
            decimal_bytes_less(&previous_ordinal, &bundle.ordinal)
        };
        proof {
            udec_decimal_value(bundle_model.s);
            assert(decimal_value(bundle.ordinal@) == bundle_model.s);
            if old_count > 0 {
                udec_decimal_value(old_bundles.last().s);
                assert(decimal_value(previous_ordinal@)
                    == old_bundles.last().s);
                assert(ordered
                    == (old_bundles.last().s < bundle_model.s));
                if let Some(d) = expected@ {
                    assert(old_bundles.len() > 0);
                    assert(old_bundles
                        == d.bundles.take(old_bundles.len() as int));
                    assert(old_bundles.last()
                        == d.bundles[(old_bundles.len() - 1) as int]);
                    assert(bundle_model
                        == d.bundles[old_bundles.len() as int]);
                    wf_doc_ordered_at(d, (old_bundles.len() - 1) as int);
                    assert(old_bundles.last().s < bundle_model.s);
                    assert(ordered);
                }
            }
        }
        if !ordered {
            *at = bundle_at;
            return None;
        }
        proof {
            bundles_push_wf(old_bundles, bundle_model);
            assert(old_bundles.len() > 0
                ==> old_bundles.last().s < bundle_model.s);
            bundles_push_ordered(old_bundles, bundle_model);
            bundles = old_bundles.push(bundle_model);
            assert(bundles.len() == old_bundles.len() + 1);
            doc_bundle_parts_flat(bundle_model);
            bundles_bytes_push(old_bundles, bundle_model);
            assert_seqs_equal!(guided.cursor.prefix@
                == doc_prefix_stage(base)
                    + ckc_spec::v1text::bundles_bytes(bundles));
            if let Some(d) = expected@ {
                assert(bundle_model
                    == d.bundles[old_bundles.len() as int]);
                assert_seqs_equal!(
                    d.bundles.take(old_bundles.len() as int).push(bundle_model)
                        == d.bundles.take(bundles.len() as int)
                );
                assert(bundles
                    == d.bundles.take(bundles.len() as int));
                assert_seqs_equal!(
                    d.bundles.skip(old_bundles.len() as int).drop_first()
                        == d.bundles.skip(bundles.len() as int)
                );
                assert(guide_rest(guided.guide@.unwrap())
                    == doc_bundles_parts(
                        d.bundles.skip(bundles.len() as int),
                    ));
            }
        }
        previous_ordinal = bundle.ordinal;
        bundle_count += 1;
        proof {
            assert(bundle_count == bundles.len());
            assert(bundle_count <= guided.cursor.pos);
            assert(previous_ordinal@
                == ckc_spec::v1text::udec_bytes(bundles.last().s));
            assert(canonical_decimal(previous_ordinal@));
        }
    }
    *at = bundle_at_floor;

    proof {
        assert(guided.cursor.pos == bytes.len());
        if let Some(d) = expected@ {
            if bundles.len() < d.bundles.len() {
                let remaining = d.bundles.skip(bundles.len() as int);
                assert(remaining.len() > 0);
                reveal(doc_bundles_parts);
                reveal(doc_bundle_parts);
                reveal(doc_marker_parts);
                let marker = ckc_spec::v1text::ascii("% S"@);
                assert(guide_rest(guided.guide@.unwrap())
                    == seq![marker]
                        + (doc_marker_parts(remaining[0]).drop_first()
                            + doc_clauses_parts(remaining[0].clauses)
                            + doc_bundles_parts(remaining.drop_first())));
                guide_front_bytes(
                    bytes@,
                    &guided,
                    marker,
                    doc_marker_parts(remaining[0]).drop_first()
                        + doc_clauses_parts(remaining[0].clauses)
                        + doc_bundles_parts(remaining.drop_first()),
                );
                reveal_strlit("% S");
                reveal(ckc_spec::v1text::ascii);
                assert(marker.len() > 0);
                assert(guided.cursor.pos < bytes.len());
                assert(false);
            }
            assert(bundles.len() == d.bundles.len());
            assert(bundles == d.bundles);
        }
    }
    if bundle_count == 0 {
        proof {
            if let Some(d) = expected@ {
                wf_doc_nonempty(d);
                assert(d.bundles.len() >= 1);
                assert(bundles == d.bundles);
                assert(false);
            }
        }
        return None;
    }

    let ghost model = ckc_spec::v1text::DocFile {
        docid: docid.value@,
        ace: ace.name@,
        ulex: ulex.value@,
        bundles,
    };
    proof {
        assert(guided_cursor_ok(bytes@, &guided));
        reveal(guided_cursor_ok);
        assert(cursor_ok(bytes@, &guided.cursor));
        reveal(cursor_ok);
        assert(guided.cursor.pos == bytes@.len());
        assert(guided.cursor.prefix@
            == bytes@.subrange(0, bytes@.len() as int));
        assert_seqs_equal!(
            bytes@.subrange(0, bytes@.len() as int) == bytes@
        );
        assert(guided.cursor.prefix@ == bytes@);
        assert(base.docid == model.docid);
        assert(base.ace == model.ace);
        assert(base.ulex == model.ulex);
        reveal(doc_flat);
        assert(doc_prefix_stage(base) == doc_prefix_stage(model));
        assert(guided.cursor.prefix@ == doc_flat(model));
        doc_flat_is_print(model);
        assert(model.bundles.len() >= 1);
        wf_doc_intro(model);
        assert(ckc_spec::v1text::wf_doc(model));
        if let Some(d) = expected@ {
            assert(docid.value@ == d.docid);
            assert(ace.name@ == d.ace);
            assert(ulex.value@ == d.ulex);
            assert(bundles == d.bundles);
            assert(model == d);
        }
        reveal(ckc_spec::v1text::wf_v1);
        reveal(ckc_spec::v1text::print_v1);
        reveal(parsed_v1_ok);
    }
    Some(EParsedV1 {
        file: Ghost(ckc_spec::v1text::V1File::Doc(model)),
    })
}

} // verus!
