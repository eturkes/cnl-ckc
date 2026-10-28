use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

#[cfg(verus_keep_ghost)]
use ckc_spec::v1text::{ReadMode, is_alnum_b, is_upper_b, nul_bytes, read_from, read_v1, run_end};

// The reading law (contract sgw; spec `v1text::read_v1`): the read bytes, and for each read
// byte the raw offset it comes from (every byte of a `_` replacement names that `_`).
pub struct ERead {
    pub text: Vec<u8>,
    pub raw: Vec<usize>,
}

pub open spec fn read_ok(bytes: Seq<u8>, r: &ERead) -> bool {
    &&& r.text@ == read_v1(bytes)
    &&& r.raw@.len() == r.text@.len()
    &&& forall|k: int| 0 <= k < r.raw@.len() ==> r.raw@[k] < bytes.len()
}

pub enum EMode {
    LineStart,
    Comment,
    Code,
    Quoted,
    Escaped,
}

pub open spec fn mode_view(m: EMode) -> ReadMode {
    match m {
        EMode::LineStart => ReadMode::LineStart,
        EMode::Comment => ReadMode::Comment,
        EMode::Code => ReadMode::Code,
        EMode::Quoted => ReadMode::Quoted,
        EMode::Escaped => ReadMode::Escaped,
    }
}

pub open spec fn views(list: Seq<Vec<u8>>) -> Seq<Seq<u8>> {
    list.map_values(|v: Vec<u8>| v@)
}

fn is_alnum(b: u8) -> (r: bool)
    ensures
        r == is_alnum_b(b),
{
    (0x61 <= b && b <= 0x7A) || (0x41 <= b && b <= 0x5A) || (0x30 <= b && b <= 0x39) || b == 0x5F
}

fn contains(list: &Vec<Vec<u8>>, x: &Vec<u8>) -> (r: bool)
    ensures
        r == views(list@).contains(x@),
{
    let mut k = 0usize;
    while k < list.len()
        invariant
            0 <= k <= list@.len(),
            forall|m: int| 0 <= m < k ==> list@[m]@ != x@,
        decreases list@.len() - k,
    {
        if crate::k2_engine::vec_equal(&list[k], x) {
            proof {
                assert(views(list@)[k as int] == x@);
            }
            return true;
        }
        k += 1;
    }
    proof {
        assert forall|m: int| 0 <= m < views(list@).len() implies views(list@)[m] != x@ by {
            assert(views(list@)[m] == list@[m]@);
        }
    }
    false
}

// The end of the maximal identifier run that starts at `i` (spec `run_end`).
fn run_end_exec(bytes: &[u8], i: usize) -> (j: usize)
    requires
        i < bytes@.len(),
        is_alnum_b(bytes@[i as int]),
    ensures
        j as nat == run_end(bytes@, i as nat),
        i < j <= bytes@.len(),
{
    let mut j = i + 1;
    proof {
        assert(run_end(bytes@, i as nat) == run_end(bytes@, (i + 1) as nat));
    }
    while j < bytes.len() && is_alnum(bytes[j])
        invariant
            i < j <= bytes@.len(),
            run_end(bytes@, i as nat) == run_end(bytes@, j as nat),
        decreases bytes@.len() - j,
    {
        proof {
            assert(run_end(bytes@, j as nat) == run_end(bytes@, (j + 1) as nat));
        }
        j += 1;
    }
    j
}

fn copy_range(bytes: &[u8], start: usize, end: usize) -> (out: Vec<u8>)
    requires
        start <= end <= bytes@.len(),
    ensures
        out@ == bytes@.subrange(start as int, end as int),
{
    let mut out = Vec::new();
    let mut k = start;
    while k < end
        invariant
            start <= k <= end <= bytes@.len(),
            out@ == bytes@.subrange(start as int, k as int),
        decreases end - k,
    {
        out.push(bytes[k]);
        proof {
            assert_seqs_equal!(bytes@.subrange(start as int, k + 1) == bytes@.subrange(start as int, k as int).push(bytes@[k as int]));
        }
        k += 1;
    }
    out
}

// Appends `piece` to the read text, every byte naming raw offset `at`.
fn put_all(text: &mut Vec<u8>, raw: &mut Vec<usize>, piece: &Vec<u8>, at: usize)
    requires
        old(raw)@.len() == old(text)@.len(),
    ensures
        final(text)@ == old(text)@ + piece@,
        final(raw)@.len() == final(text)@.len(),
        forall|k: int| 0 <= k < old(raw)@.len() ==> final(raw)@[k] == old(raw)@[k],
        forall|k: int| old(raw)@.len() <= k < final(raw)@.len() ==> final(raw)@[k] == at,
{
    let mut k = 0usize;
    while k < piece.len()
        invariant
            0 <= k <= piece@.len(),
            text@ == old(text)@ + piece@.subrange(0, k as int),
            raw@.len() == text@.len(),
            old(raw)@.len() <= raw@.len(),
            forall|m: int| 0 <= m < old(raw)@.len() ==> raw@[m] == old(raw)@[m],
            forall|m: int| old(raw)@.len() <= m < raw@.len() ==> raw@[m] == at,
        decreases piece@.len() - k,
    {
        let ghost prev = raw@;
        text.push(piece[k]);
        raw.push(at);
        proof {
            assert forall|m: int| 0 <= m < old(raw)@.len() implies raw@[m] == old(raw)@[m] by {
                assert(raw@[m] == prev[m]);
            }
            assert forall|m: int| old(raw)@.len() <= m < raw@.len() implies raw@[m] == at by {
                if m < prev.len() {
                    assert(raw@[m] == prev[m]);
                }
            }
            assert_seqs_equal!(old(text)@ + piece@.subrange(0, k + 1) == old(text)@ + piece@.subrange(0, k as int) + seq![piece@[k as int]]);
        }
        k += 1;
    }
    proof {
        assert_seqs_equal!(piece@.subrange(0, piece@.len() as int) == piece@);
    }
}

// Appends the run bytes[start..end] (or as many NUL bytes), each naming its own raw offset.
fn put_run(
    text: &mut Vec<u8>,
    raw: &mut Vec<usize>,
    bytes: &[u8],
    start: usize,
    end: usize,
    nul: bool,
)
    requires
        start <= end <= bytes@.len(),
        old(raw)@.len() == old(text)@.len(),
    ensures
        final(text)@ == old(text)@ + if nul {
            nul_bytes((end - start) as nat)
        } else {
            bytes@.subrange(start as int, end as int)
        },
        final(raw)@.len() == final(text)@.len(),
        forall|k: int| 0 <= k < old(raw)@.len() ==> final(raw)@[k] == old(raw)@[k],
        forall|k: int| old(raw)@.len() <= k < final(raw)@.len() ==> final(raw)@[k] < end,
{
    let ghost piece = if nul {
        nul_bytes((end - start) as nat)
    } else {
        bytes@.subrange(start as int, end as int)
    };
    let mut k = start;
    while k < end
        invariant
            start <= k <= end <= bytes@.len(),
            piece.len() == end - start,
            piece == if nul {
                nul_bytes((end - start) as nat)
            } else {
                bytes@.subrange(start as int, end as int)
            },
            text@ == old(text)@ + piece.subrange(0, k - start),
            raw@.len() == text@.len(),
            old(raw)@.len() <= raw@.len(),
            forall|m: int| 0 <= m < old(raw)@.len() ==> raw@[m] == old(raw)@[m],
            forall|m: int| old(raw)@.len() <= m < raw@.len() ==> raw@[m] < end,
        decreases end - k,
    {
        let b = if nul {
            0u8
        } else {
            bytes[k]
        };
        proof {
            assert(piece[k - start] == b);
        }
        let ghost prev = raw@;
        text.push(b);
        raw.push(k);
        proof {
            assert forall|m: int| 0 <= m < old(raw)@.len() implies raw@[m] == old(raw)@[m] by {
                assert(raw@[m] == prev[m]);
            }
            assert forall|m: int| old(raw)@.len() <= m < raw@.len() implies raw@[m] < end by {
                if m < prev.len() {
                    assert(raw@[m] == prev[m]);
                }
            }
            assert_seqs_equal!(old(text)@ + piece.subrange(0, k + 1 - start) == old(text)@ + piece.subrange(0, k - start) + seq![b]);
        }
        k += 1;
    }
    proof {
        assert_seqs_equal!(piece.subrange(0, end - start) == piece);
    }
}

pub fn read_bytes(bytes: &[u8]) -> (r: ERead)
    ensures
        read_ok(bytes@, &r),
{
    let mut text: Vec<u8> = Vec::new();
    let mut raw: Vec<usize> = Vec::new();
    let mut seen: Vec<Vec<u8>> = Vec::new();
    let mut given: Vec<Vec<u8>> = Vec::new();
    let mut mode = EMode::LineStart;
    let mut i = 0usize;
    proof {
        assert_seqs_equal!(views(seen@) == Seq::<Seq<u8>>::empty());
        assert_seqs_equal!(views(given@) == Seq::<Seq<u8>>::empty());
        assert_seqs_equal!(text@ + read_v1(bytes@) == read_v1(bytes@));
    }
    while i < bytes.len()
        invariant
            i <= bytes@.len(),
            read_v1(bytes@) == text@ + read_from(
                bytes@,
                i as nat,
                mode_view(mode),
                views(seen@),
                views(given@),
            ),
            raw@.len() == text@.len(),
            forall|k: int| 0 <= k < raw@.len() ==> raw@[k] < bytes@.len(),
            seen@.len() + given@.len() <= i,
        decreases bytes@.len() - i,
    {
        let ghost s0 = views(seen@);
        let ghost g0 = views(given@);
        let ghost m0 = mode_view(mode);
        let ghost t0 = text@;
        let b = bytes[i];
        let comment = match mode {
            EMode::Comment => true,
            EMode::LineStart => b == 0x25,
            _ => false,
        };
        let inside = match mode {
            EMode::Quoted => true,
            EMode::Escaped => true,
            _ => false,
        };
        if b == 0x0A {
            text.push(b);
            raw.push(i);
            seen = Vec::new();
            given = Vec::new();
            mode = EMode::LineStart;
            proof {
                assert_seqs_equal!(views(seen@) == Seq::<Seq<u8>>::empty());
                assert_seqs_equal!(views(given@) == Seq::<Seq<u8>>::empty());
                let rest = read_from(
                    bytes@,
                    (i + 1) as nat,
                    ReadMode::LineStart,
                    Seq::empty(),
                    Seq::empty(),
                );
                assert(read_from(bytes@, i as nat, m0, s0, g0) == seq![b] + rest);
                assert_seqs_equal!(t0 + (seq![b] + rest) == text@ + rest);
            }
            i += 1;
        } else if comment || inside || b == 0x27 || !is_alnum(b) {
            let next = match mode {
                EMode::Escaped => EMode::Quoted,
                EMode::Quoted => if b == 0x5C {
                    EMode::Escaped
                } else if b == 0x27 {
                    EMode::Code
                } else {
                    EMode::Quoted
                },
                _ => if comment {
                    EMode::Comment
                } else if b == 0x27 {
                    EMode::Quoted
                } else {
                    EMode::Code
                },
            };
            text.push(b);
            raw.push(i);
            mode = next;
            proof {
                let rest = read_from(bytes@, (i + 1) as nat, mode_view(next), s0, g0);
                assert(read_from(bytes@, i as nat, m0, s0, g0) == seq![b] + rest);
                assert_seqs_equal!(t0 + (seq![b] + rest) == text@ + rest);
            }
            i += 1;
        } else {
            let j = run_end_exec(bytes, i);
            let run = copy_range(bytes, i, j);
            let upper = 0x41 <= b && b <= 0x5A;
            proof {
                assert(run@.len() == j - i);
                assert(run@[0] == b);
            }
            if run.len() == 1 && run[0] == 0x5F {
                let spelling = crate::k2_engine::var_spelling(seen.len() + given.len());
                let ghost sp = spelling@;
                put_all(&mut text, &mut raw, &spelling, i);
                given.push(spelling);
                proof {
                    assert_seqs_equal!(run@ == seq![0x5Fu8]);
                    assert(sp == ckc_spec::v1text::var_bytes(s0.len() + g0.len()));
                    let rest = read_from(bytes@, j as nat, ReadMode::Code, s0, g0.push(sp));
                    assert(read_from(bytes@, i as nat, m0, s0, g0) == sp + rest);
                    assert_seqs_equal!(views(given@) == g0.push(sp));
                    assert_seqs_equal!(t0 + (sp + rest) == text@ + rest);
                }
            } else {
                proof {
                    if run@ == seq![0x5Fu8] {
                        assert(run@.len() == 1 && run@[0] == 0x5F);
                    }
                    assert(run@ != seq![0x5Fu8]);
                }
                if upper && contains(&given, &run) {
                    put_run(&mut text, &mut raw, bytes, i, j, true);
                    proof {
                        let rest = read_from(bytes@, j as nat, ReadMode::Code, s0, g0);
                        assert(read_from(bytes@, i as nat, m0, s0, g0) == nul_bytes(run@.len())
                            + rest);
                        assert_seqs_equal!(t0 + (nul_bytes(run@.len()) + rest) == text@ + rest);
                    }
                } else if upper && !contains(&seen, &run) {
                    put_run(&mut text, &mut raw, bytes, i, j, false);
                    let ghost rv = run@;
                    seen.push(run);
                    proof {
                        let rest = read_from(bytes@, j as nat, ReadMode::Code, s0.push(rv), g0);
                        assert(read_from(bytes@, i as nat, m0, s0, g0) == rv + rest);
                        assert_seqs_equal!(views(seen@) == s0.push(rv));
                        assert_seqs_equal!(t0 + (rv + rest) == text@ + rest);
                    }
                } else {
                    put_run(&mut text, &mut raw, bytes, i, j, false);
                    proof {
                        let rest = read_from(bytes@, j as nat, ReadMode::Code, s0, g0);
                        assert(read_from(bytes@, i as nat, m0, s0, g0) == run@ + rest);
                        assert_seqs_equal!(t0 + (run@ + rest) == text@ + rest);
                    }
                }
            }
            mode = EMode::Code;
            i = j;
        }
    }
    proof {
        assert(read_from(bytes@, i as nat, mode_view(mode), views(seen@), views(given@)) == Seq::<
            u8,
        >::empty());
        assert_seqs_equal!(text@ + Seq::<u8>::empty() == text@);
    }
    ERead { text, raw }
}

// The raw offset of a read offset (R9 diagnostics, contract sgw D5); read EOF = raw EOF.
pub fn raw_offset(r: &ERead, raw_len: usize, at: usize) -> (out: usize)
    requires
        forall|k: int| 0 <= k < r.raw@.len() ==> r.raw@[k] < raw_len,
    ensures
        out <= raw_len,
{
    if at < r.raw.len() {
        r.raw[at]
    } else {
        raw_len
    }
}

} // verus!
