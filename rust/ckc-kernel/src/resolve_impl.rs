use crate::{resolve_data as d, resolve_primitives as a, resolve_text as t};
use ckc_spec::align::*;
use vstd::assert_seqs_equal;
use vstd::prelude::*;
use vstd::slice::{slice_subrange, slice_to_vec};

verus! {

pub open spec fn walk_view(r: Result<(Vec<d::Span>, Vec<d::Span>), Vec<char>>) -> Result<
    (Seq<ResSpan>, Seq<ResSpan>),
    Seq<char>,
> {
    match r {
        Ok((s, a)) => Ok((d::spans(s@), d::spans(a@))),
        Err(e) => Err(e@),
    }
}

fn bad(prefix: Vec<char>, detail: Vec<char>) -> (r: Result<(Vec<d::Span>, Vec<d::Span>), Vec<char>>)
    ensures
        walk_view(r) == Err(prefix@ + detail@),
{
    Err(a::concat_chars(prefix, detail))
}

fn walk(rows: &Vec<Vec<char>>, src: &[char], ace: &[char]) -> (r: Result<
    (Vec<d::Span>, Vec<d::Span>),
    Vec<char>,
>)
    ensures
        walk_view(r) == resolve_rows(
            a::seqs_view(rows@),
            1,
            src@,
            ace@,
            Seq::empty(),
            Seq::empty(),
        ),
        r matches Ok((s, a)) ==> d::valid(s@) && d::valid(a@) && s.len() + a.len() == rows.len(),
{
    let mut ss = Vec::new();
    let mut aa = Vec::new();
    let mut i = 0usize;
    proof {
        a::seqs_view_len(rows@);
        assert_seqs_equal!(a::seqs_view(rows@).skip(0)==a::seqs_view(rows@));
        assert_seqs_equal!(d::spans(ss@) == Seq::<ResSpan>::empty());
        assert_seqs_equal!(d::spans(aa@) == Seq::<ResSpan>::empty());
    }
    while i < rows.len()
        invariant
            i <= rows.len(),
            a::seqs_view(rows@).len() == rows.len(),
            d::valid(ss@),
            d::valid(aa@),
            ss.len() + aa.len() == i,
            resolve_rows(a::seqs_view(rows@), 1, src@, ace@, Seq::empty(), Seq::empty())
                == resolve_rows(
                a::seqs_view(rows@).skip(i as int),
                i as int + 1,
                src@,
                ace@,
                d::spans(ss@),
                d::spans(aa@),
            ),
        decreases rows.len() - i,
    {
        proof {
            a::seqs_view_index(rows@, i as int);
            reveal_with_fuel(resolve_rows, 2);
        }
        let p = a::row_prefix(i + 1);
        let fs = a::split_at_seps(&rows[i], '\t');
        proof {
            a::seqs_view_len(fs@);
        }
        if fs.len() != 4 {
            return bad(p, t::fields());
        }
        proof {
            a::seqs_view_index(fs@, 0);
            a::seqs_view_index(fs@, 1);
            a::seqs_view_index(fs@, 2);
            a::seqs_view_index(fs@, 3);
        }
        if !d::canonical_int(&fs[0]) {
            return bad(p, t::group());
        }
        if fs[0][0] == '-' {
            proof {
                d::nonzero(fs@[0]@.drop_first());
                reveal(int_value);
            }
            return bad(p, t::negative());
        }
        proof {
            assert(is_canonical_decimal(fs@[0]@));
            a::dec_nonnegative(fs@[0]@);
        }
        if !d::canonical_int(&fs[2]) {
            return bad(p, t::occurrence());
        }
        if fs[2][0] == '-' {
            proof {
                d::nonzero(fs@[2]@.drop_first());
                reveal(int_value);
            }
            return bad(p, t::zero());
        }
        if a::seq_equal(&fs[2], &t::digit_zero()) {
            proof {
                reveal_strlit("0");
                reveal_with_fuel(dec_value, 2);
            }
            return bad(p, t::zero());
        }
        proof {
            assert(is_canonical_decimal(fs@[2]@));
            d::nonzero(fs@[2]@);
        }
        if fs[3].len() == 0 {
            return bad(p, t::span());
        }
        let source = a::seq_equal(&fs[1], &t::src());
        if !source && !a::seq_equal(&fs[1], &t::ace()) {
            return bad(p, t::side());
        }
        let text = if source {
            src
        } else {
            ace
        };
        let number = a::bounded_decimal(&fs[2], text.len());
        let k = match number {
            None => {
                proof {
                    d::too_many(text@, fs@[3]@, int_value(fs@[2]@), 0);
                }
                None
            },
            Some(n) => d::nth(text, &fs[3], n, 0),
        };
        let start = match k {
            None => {
                let e = a::concat_chars(t::occ(), slice_to_vec(fs[2].as_slice()));
                let e = a::concat_chars(e, t::not_found());
                let e = a::concat_chars(e, slice_to_vec(fs[1].as_slice()));
                proof {
                    assert(nth_start(text@, fs@[3]@, int_value(fs@[2]@), 0) == -1);
                    assert_seqs_equal!(p@ + e@ == row_prefix(i as int + 1) + "occurrence "@ + fs@[2]@ + " of span not found in "@ + fs@[1]@);
                    reveal_with_fuel(resolve_rows, 2);
                    assert(resolve_rows(
                        a::seqs_view(rows@).skip(i as int),
                        i as int + 1,
                        src@,
                        ace@,
                        d::spans(ss@),
                        d::spans(aa@),
                    ) == Err(p@ + e@));
                }
                return bad(p, e);
            },
            Some(k) => k,
        };
        let x = d::Span {
            start,
            end: start + fs[3].len(),
            group: slice_to_vec(fs[0].as_slice()),
            span: slice_to_vec(fs[3].as_slice()),
        };
        proof {
            assert_seqs_equal!(a::seqs_view(rows@).skip(i as int).drop_first()==a::seqs_view(rows@).skip(i as int+1));
        }
        if source {
            let ghost before = ss@;
            ss.push(x);
            proof {
                assert_seqs_equal!(d::spans(ss@)==d::spans(before).push(x@));
            }
        } else {
            let ghost before = aa@;
            aa.push(x);
            proof {
                assert_seqs_equal!(d::spans(aa@)==d::spans(before).push(x@));
            }
        }
        i += 1;
    }
    proof {
        reveal_with_fuel(resolve_rows, 2);
    }
    Ok((ss, aa))
}

proof fn split_bound(s: Seq<char>, sep: char)
    ensures
        1 <= split_at_seps(s, sep).len() <= s.len() + 1,
    decreases s.len(),
{
    reveal_with_fuel(split_at_seps, 2);
    if s.len() > 0 {
        split_bound(s.drop_first(), sep);
    }
}

pub fn resolve_impl(input: &[char], src: &[char], ace: &[char]) -> (r: EResolve)
    ensures
        r@ == resolve_outcome(input@, src@, ace@),
{
    if input.len() == 0 || input[input.len() - 1] != '\n' {
        return EResolve::Err(t::newline());
    }
    if input.len() == 1 {
        return EResolve::Err(t::empty());
    }
    let body = slice_subrange(input, 0, input.len() - 1);
    proof {
        assert_seqs_equal!(body@==input@.drop_last());
    }
    let rows = a::split_at_seps(body, '\n');
    proof {
        split_bound(body@, '\n');
        a::seqs_view_len(rows@);
    }
    let (ss, aa) = match walk(&rows, src, ace) {
        Err(e) => return EResolve::Err(e),
        Ok(x) => x,
    };
    proof {
        assert_seqs_equal!(d::groups(Seq::<Vec<char>>::empty()) == Seq::<int>::empty());
        reveal(resolve_outcome);
        reveal(resolve);
        assert(resolve_rows(
            split_at_seps(input@.drop_last(), '\n'),
            1,
            src@,
            ace@,
            Seq::empty(),
            Seq::empty(),
        ) == Ok((d::spans(ss@), d::spans(aa@))));
    }
    let sg = d::group_list_exec(&ss, 0, Vec::new());
    let ag = d::group_list_exec(&aa, 0, Vec::new());
    proof {
        assert_seqs_equal!(d::spans(ss@).skip(0)==d::spans(ss@));
        assert_seqs_equal!(d::spans(aa@).skip(0)==d::spans(aa@));
    }
    if !d::same(&sg, &ag) {
        return EResolve::Err(t::groups());
    }
    let sorted_a = d::sorted(&aa);
    proof {
        assert_seqs_equal!(d::spans(sorted_a@).skip(0)==d::spans(sorted_a@));
    }
    let out_a = match d::side(&t::ace(), &sorted_a, 0, 0) {
        Err(e) => return EResolve::Err(e),
        Ok(x) => x,
    };
    let sorted_s = d::sorted(&ss);
    proof {
        assert_seqs_equal!(d::spans(sorted_s@).skip(0)==d::spans(sorted_s@));
    }
    let out_s = match d::side(&t::src(), &sorted_s, 0, 0) {
        Err(e) => return EResolve::Err(e),
        Ok(x) => x,
    };
    proof {
        assert_seqs_equal!(d::spans(sorted_a@).skip(0)==d::spans(sorted_a@));
        assert_seqs_equal!(d::spans(sorted_s@).skip(0)==d::spans(sorted_s@));
    }
    let text = a::concat_chars(out_a, out_s);
    EResolve::Ok(EResolved { text, groups: ag.len() as u64, spans: (ss.len() + aa.len()) as u64 })
}

} // verus!
