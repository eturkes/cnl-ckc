#[cfg(verus_keep_ghost)]
use crate::k4_bytes::byte_rows_push;
use crate::k4_bytes::{append, contains, copy, eq};
use crate::k4_scalar::number;
use ckc_spec::check::*;
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub proof fn reviewed_push(ds: Seq<EDecision>, d: EDecision)
    ensures
        reviewed(decisions(ds.push(d))) == if reviewed(decisions(ds)).contains(d.docid@) {
            reviewed(decisions(ds))
        } else {
            reviewed(decisions(ds)).push(d.docid@)
        },
{
    let ids = decisions(ds).map_values(|x: Decision| x.docid);
    assert_seqs_equal!(decisions(ds.push(d)).map_values(|x: Decision| x.docid) == ids.push(d.docid@));
    assert_seqs_equal!(ids.push(d.docid@).drop_last() == ids);
    reveal_with_fuel(Seq::fold_left, 2);
}

pub fn reviewed_list(ds: &Vec<EDecision>) -> (out: Vec<Vec<u8>>)
    ensures
        byte_rows(out@) == reviewed(decisions(ds@)),
        out@.len() <= ds@.len(),
{
    let mut out = Vec::new();
    let mut i = 0usize;
    proof {
        assert_seqs_equal!(ds@.take(0) == Seq::<EDecision>::empty());
    }
    while i < ds.len()
        invariant
            i <= ds@.len(),
            out@.len() <= i,
            byte_rows(out@) == reviewed(decisions(ds@.take(i as int))),
        decreases ds.len() - i,
    {
        let seen = contains(&out, &ds[i].docid);
        proof {
            reviewed_push(ds@.take(i as int), ds@[i as int]);
            assert_seqs_equal!(ds@.take(i as int + 1) == ds@.take(i as int).push(ds@[i as int]));
        }
        if !seen {
            let name = copy(&ds[i].docid);
            proof {
                byte_rows_push(out@, name);
            }
            out.push(name);
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(ds@.take(i as int) == ds@);
    }
    out
}

pub open spec fn current_pair(x: Decision, b: Bundle, d: Seq<u8>, a: bool) -> bool {
    x.docid == d && b.docid == d && x.digest == b.review && x.approved == a
}

pub fn current(ds: &Vec<EDecision>, bs: &Vec<EBundle>, d: &[u8], approved: bool) -> (r: bool)
    ensures
        r == cur(decisions(ds@), bundles(bs@), d@, approved),
{
    let mut i = 0usize;
    while i < ds.len()
        invariant
            i <= ds@.len(),
            forall|p: int, q: int|
                0 <= p < i && 0 <= q < bs@.len() ==> !current_pair(ds@[p]@, bs@[q]@, d@, approved),
        decreases ds.len() - i,
    {
        let decision = &ds[i];
        if eq(&decision.docid, d) && decision.approved == approved {
            let mut j = 0usize;
            while j < bs.len()
                invariant
                    i < ds@.len(),
                    j <= bs@.len(),
                    decision@ == ds@[i as int]@,
                    decision.docid@ == d@,
                    decision.approved == approved,
                    forall|p: int, q: int|
                        0 <= p < i && 0 <= q < bs@.len() ==> !current_pair(
                            ds@[p]@,
                            bs@[q]@,
                            d@,
                            approved,
                        ),
                    forall|q: int|
                        0 <= q < j ==> !current_pair(ds@[i as int]@, bs@[q]@, d@, approved),
                decreases bs.len() - j,
            {
                let bundle = &bs[j];
                if eq(&bundle.docid, d) && eq(&bundle.review, &decision.digest) {
                    proof {
                        assert(current_pair(ds@[i as int]@, bs@[j as int]@, d@, approved));
                        assert(decisions(ds@)[i as int] == ds@[i as int]@);
                        assert(bundles(bs@)[j as int] == bs@[j as int]@);
                        reveal(cur);
                    }
                    return true;
                }
                j += 1;
            }
        }
        i += 1;
    }
    proof {
        assert forall|p: int, q: int|
            0 <= p < decisions(ds@).len() && 0 <= q < bundles(bs@).len() implies !current_pair(
            decisions(ds@)[p],
            bundles(bs@)[q],
            d@,
            approved,
        ) by {
            assert(decisions(ds@)[p] == ds@[p]@);
            assert(bundles(bs@)[q] == bs@[q]@);
            assert(!current_pair(ds@[p]@, bs@[q]@, d@, approved));
        }
        assert forall|p: int, q: int|
            0 <= p < decisions(ds@).len() && 0 <= q < bundles(bs@).len() implies !(decisions(
            ds@,
        )[p].docid == d@ && bundles(bs@)[q].docid == d@ && decisions(ds@)[p].digest == bundles(
            bs@,
        )[q].review && decisions(ds@)[p].approved == approved) by {
            assert(!current_pair(decisions(ds@)[p], bundles(bs@)[q], d@, approved));
            reveal(current_pair);
        }
        reveal(cur);
        assert(!cur(decisions(ds@), bundles(bs@), d@, approved));
    }
    false
}

pub fn count(ds: &Vec<EDecision>, bs: &Vec<EBundle>, k: i64) -> (n: usize)
    ensures
        n as nat == class_count(decisions(ds@), bundles(bs@), k as int),
{
    let names = reviewed_list(ds);
    let ghost pred = |d: Seq<u8>|
        {
            let a = cur(decisions(ds@), bundles(bs@), d, true);
            let r = cur(decisions(ds@), bundles(bs@), d, false);
            if k == 0 {
                a && !r
            } else if k == 1 {
                !a && r
            } else if k == 2 {
                a && r
            } else {
                !a && !r
            }
        };
    let mut i = 0usize;
    let mut n = 0usize;
    while i < names.len()
        invariant
            i <= names@.len(),
            n <= i,
            byte_rows(names@) == reviewed(decisions(ds@)),
            n as nat == byte_rows(names@).take(i as int).filter(pred).len(),
            forall|d: Seq<u8>| #[trigger]
                pred(d) == {
                    let a = cur(decisions(ds@), bundles(bs@), d, true);
                    let r = cur(decisions(ds@), bundles(bs@), d, false);
                    if k == 0 {
                        a && !r
                    } else if k == 1 {
                        !a && r
                    } else if k == 2 {
                        a && r
                    } else {
                        !a && !r
                    }
                },
        decreases names.len() - i,
    {
        let a = current(ds, bs, &names[i], true);
        let r = current(ds, bs, &names[i], false);
        let matched = if k == 0 {
            a && !r
        } else if k == 1 {
            !a && r
        } else if k == 2 {
            a && r
        } else {
            !a && !r
        };
        proof {
            assert(a == cur(decisions(ds@), bundles(bs@), names@[i as int]@, true));
            assert(r == cur(decisions(ds@), bundles(bs@), names@[i as int]@, false));
            assert(matched == pred(names@[i as int]@));
            assert_seqs_equal!(byte_rows(names@).take(i as int + 1) == byte_rows(names@).take(i as int).push(names@[i as int]@));
            byte_rows(names@).take(i as int).lemma_filter_len_push(pred, names@[i as int]@);
        }
        if matched {
            n += 1;
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(byte_rows(names@).take(i as int) == byte_rows(names@));
        reveal(class_count);
    }
    n
}

pub fn meter(gid: &[u8], ds: &Vec<EDecision>, bs: &Vec<EBundle>) -> (r: Vec<u8>)
    ensures
        r@ == adjudication_meter(gid@, decisions(ds@), bundles(bs@)),
{
    let approved = number(count(ds, bs, 0));
    let rejected = number(count(ds, bs, 1));
    let contested = number(count(ds, bs, 2));
    let stale = number(count(ds, bs, 3));
    let names = reviewed_list(ds);
    let unreviewed = number(
        if bs.len() >= names.len() {
            bs.len() - names.len()
        } else {
            0
        },
    );
    let total = number(ds.len());
    let mut r = copy(b"goal: adjudication ");
    append(&mut r, gid);
    append(&mut r, b" approved=");
    append(&mut r, &approved);
    append(&mut r, b" rejected=");
    append(&mut r, &rejected);
    append(&mut r, b" contested=");
    append(&mut r, &contested);
    append(&mut r, b" stale=");
    append(&mut r, &stale);
    append(&mut r, b" unreviewed=");
    append(&mut r, &unreviewed);
    append(&mut r, b" decisions=");
    append(&mut r, &total);
    r.push(0x0a);
    proof {
        reveal_byteslit(b"goal: adjudication ");
        reveal_strlit("goal: adjudication ");
        reveal_byteslit(b" approved=");
        reveal_strlit(" approved=");
        reveal_byteslit(b" rejected=");
        reveal_strlit(" rejected=");
        reveal_byteslit(b" contested=");
        reveal_strlit(" contested=");
        reveal_byteslit(b" stale=");
        reveal_strlit(" stale=");
        reveal_byteslit(b" unreviewed=");
        reveal_strlit(" unreviewed=");
        reveal_byteslit(b" decisions=");
        reveal_strlit(" decisions=");
        reveal(ckc_spec::v1text::ascii);
        reveal(adjudication_meter);
    }
    r
}

} // verus!
