use crate::k4_bytes::{append, concat, copy, eq, less, split, starts_with};
#[cfg(verus_keep_ghost)]
use ckc_spec::check::byte_pairs;
use ckc_spec::release::*;
#[cfg(verus_keep_ghost)]
use ckc_spec::v1text::ascii;
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub fn lookup_exec(v: &Vec<(Vec<u8>, Vec<u8>)>, key: &[u8]) -> (r: Vec<u8>)
    ensures
        r@ == lookup(byte_pairs(v@), key@),
{
    let mut i = 0usize;
    proof {
        assert_seqs_equal!(byte_pairs(v@).skip(0) == byte_pairs(v@));
    }
    while i < v.len()
        invariant
            i <= v.len(),
            lookup(byte_pairs(v@), key@) == lookup(byte_pairs(v@).skip(i as int), key@),
        decreases v.len() - i,
    {
        proof {
            reveal_with_fuel(lookup, 2);
        }
        if eq(&v[i].0, key) {
            return copy(&v[i].1);
        }
        proof {
            assert_seqs_equal!(byte_pairs(v@).skip(i as int).drop_first() == byte_pairs(v@).skip(i as int + 1));
        }
        i += 1;
    }
    Vec::new()
}

pub fn gid(path: &[u8]) -> (r: Vec<u8>)
    ensures
        r@ == gid_of(path@),
{
    let segs = split(path, b'/');
    if segs.len() >= 2 {
        copy(&segs[1])
    } else {
        Vec::new()
    }
}

pub fn source(path: &[u8]) -> (r: bool)
    ensures
        r == is_source(path@),
{
    let g = gid(path);
    let mut prefix = copy(b"guidelines/");
    append(&mut prefix, &g);
    append(&mut prefix, b"/source/");
    proof {
        reveal_byteslit(b"guidelines/");
        reveal_strlit("guidelines/");
        reveal_byteslit(b"/source/");
        reveal_strlit("/source/");
        reveal(ascii);
    }
    proof {
        assert(prefix@ == ascii("guidelines/"@) + gid_of(path@) + ascii("/source/"@));
    }
    starts_with(path, &prefix)
}

pub fn clone_member(m: &EMember) -> (r: EMember)
    ensures
        r@ == m@,
{
    EMember { path: copy(&m.path), sha: copy(&m.sha), size: m.size }
}

pub fn data(m: &EMember) -> (r: EMember)
    ensures
        r@ == data_member(m@),
{
    let path = concat(b"data/", &m.path);
    proof {
        reveal_byteslit(b"data/");
        reveal_strlit("data/");
        reveal(ascii);
    }
    proof {
        assert(path@ == ascii("data/"@) + m@.path);
    }
    EMember { path, sha: copy(&m.sha), size: m.size }
}

pub open spec fn source_views(v: Seq<(EMember, Vec<u8>)>) -> Seq<(Member, Seq<u8>)> {
    v.map_values(|x: (EMember, Vec<u8>)| (x.0@, x.1@))
}

pub fn partition(
    staged: &Vec<EMember>,
    profiles: &Vec<(Vec<u8>, Vec<u8>)>,
    urls: &Vec<(Vec<u8>, Vec<u8>)>,
) -> (r: (Vec<EMember>, Vec<(EMember, Vec<u8>)>))
    ensures
        members(r.0@) == payload(members(staged@), byte_pairs(profiles@)),
        source_views(r.1@) == sources(members(staged@), byte_pairs(profiles@), byte_pairs(urls@)),
{
    let mut pay = Vec::new();
    let mut src = Vec::new();
    let ghost ps = byte_pairs(profiles@);
    let ghost us = byte_pairs(urls@);
    let ghost keep = |m: Member|
        lookup(ps, gid_of(m.path)) != ascii("restricted"@) && !(lookup(ps, gid_of(m.path)) == ascii(
            "reconstructable"@,
        ) && is_source(m.path));
    let ghost rebuild = |m: Member|
        lookup(ps, gid_of(m.path)) == ascii("reconstructable"@) && is_source(m.path);
    let mut i = 0usize;
    while i < staged.len()
        invariant
            i <= staged.len(),
            ps == byte_pairs(profiles@),
            us == byte_pairs(urls@),
            keep == (|m: Member|
                lookup(ps, gid_of(m.path)) != ascii("restricted"@) && !(lookup(ps, gid_of(m.path))
                    == ascii("reconstructable"@) && is_source(m.path))),
            rebuild == (|m: Member|
                lookup(ps, gid_of(m.path)) == ascii("reconstructable"@) && is_source(m.path)),
            members(pay@) == payload(members(staged@).take(i as int), ps),
            source_views(src@) == sources(members(staged@).take(i as int), ps, us),
        decreases staged.len() - i,
    {
        let m = &staged[i];
        let g = gid(&m.path);
        let profile = lookup_exec(profiles, &g);
        let restricted = eq(&profile, b"restricted");
        let reconstructable = eq(&profile, b"reconstructable");
        let is_src = source(&m.path);
        proof {
            reveal_byteslit(b"restricted");
            reveal_strlit("restricted");
            reveal_byteslit(b"reconstructable");
            reveal_strlit("reconstructable");
            reveal(ascii);
            assert_seqs_equal!(b"restricted"@ == ascii("restricted"@));
            assert_seqs_equal!(b"reconstructable"@ == ascii("reconstructable"@));
            assert(keep(m@) == (!restricted && !(reconstructable && is_src)));
            assert(rebuild(m@) == (reconstructable && is_src));
            let s = members(staged@).take(i as int);
            assert_seqs_equal!(members(staged@).take(i as int + 1) == s.push(m@));
            s.lemma_filter_push(m@, keep);
            s.lemma_filter_push(m@, rebuild);
            assert_seqs_equal!(s.filter(keep).push(m@).map_values(|x: Member| data_member(x)) == s.filter(keep).map_values(|x: Member| data_member(x)).push(data_member(m@)));
            assert_seqs_equal!(s.filter(rebuild).push(m@).map_values(|x: Member| (data_member(x), lookup(us, gid_of(x.path)))) == s.filter(rebuild).map_values(|x: Member| (data_member(x), lookup(us, gid_of(x.path)))).push((data_member(m@), lookup(us, gid_of(m@.path)))));
        }
        if !restricted && !(reconstructable && is_src) {
            let x = data(m);
            let ghost before = pay@;
            pay.push(x);
            proof {
                assert_seqs_equal!(members(pay@) == members(before).push(x@));
            }
        }
        if reconstructable && is_src {
            let x = data(m);
            let u = lookup_exec(urls, &g);
            let ghost before = src@;
            let ghost uv = u@;
            src.push((x, u));
            proof {
                assert_seqs_equal!(source_views(src@) == source_views(before).push((x@, uv)));
            }
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(members(staged@).take(i as int) == members(staged@));
    }
    (pay, src)
}

proof fn insert_at(x: Member, s: Seq<Member>, i: nat)
    requires
        i <= s.len(),
        forall|j: int| 0 <= j < i ==> !ckc_spec::engine::bytes_lt(x.path, s[j].path),
        i == s.len() || ckc_spec::engine::bytes_lt(x.path, s[i as int].path),
    ensures
        insert_member(x, s) == s.take(i as int) + seq![x] + s.skip(i as int),
    decreases i,
{
    reveal_with_fuel(insert_member, 2);
    if i > 0 {
        insert_at(x, s.drop_first(), (i - 1) as nat);
        assert_seqs_equal!(s.take(i as int) == seq![s[0]] + s.drop_first().take(i as int - 1));
        assert_seqs_equal!(s.skip(i as int) == s.drop_first().skip(i as int - 1));
    }
}

pub fn insert(x: EMember, mut out: Vec<EMember>) -> (r: Vec<EMember>)
    ensures
        members(r@) == insert_member(x@, members(out@)),
{
    let mut i = 0usize;
    while i < out.len() && !less(&x.path, &out[i].path)
        invariant
            i <= out.len(),
            forall|j: int| 0 <= j < i ==> !ckc_spec::engine::bytes_lt(x@.path, out@[j]@.path),
        decreases out.len() - i,
    {
        i += 1;
    }
    let ghost before = out@;
    out.insert(i, x);
    proof {
        insert_at(x@, members(before), i as nat);
        assert_seqs_equal!(members(out@) == members(before).take(i as int) + seq![x@] + members(before).skip(i as int));
    }
    out
}

pub fn sorted(ms: &Vec<EMember>) -> (r: Vec<EMember>)
    ensures
        members(r@) == sort_members(members(ms@)),
{
    let mut r = Vec::new();
    let mut i = ms.len();
    while i > 0
        invariant
            i <= ms.len(),
            members(r@) == sort_members(members(ms@).skip(i as int)),
        decreases i,
    {
        let x = clone_member(&ms[i - 1]);
        r = insert(x, r);
        proof {
            assert_seqs_equal!(members(ms@).skip(i as int - 1).drop_first() == members(ms@).skip(i as int));
            reveal_with_fuel(sort_members, 2);
        }
        i -= 1;
    }
    proof {
        assert_seqs_equal!(members(ms@).skip(0) == members(ms@));
    }
    r
}

fn decimal(n: u64) -> (r: Vec<u8>)
    ensures
        r@ == ckc_spec::check::nat_bytes(n as nat),
    decreases n,
{
    let mut r = if n < 10 {
        Vec::new()
    } else {
        decimal(n / 10)
    };
    r.push(b'0' + (n % 10) as u8);
    proof {
        reveal_with_fuel(ckc_spec::v1text::udec_bytes, 2);
        reveal(ckc_spec::v1text::digit_byte);
    }
    r
}

fn member_row(m: &EMember) -> (r: Vec<u8>)
    ensures
        r@ == ascii("member\t"@) + m@.path + tab() + m@.sha + tab() + ckc_spec::check::nat_bytes(
            m@.size,
        ) + lf(),
{
    let mut r = copy(b"member\t");
    append(&mut r, &m.path);
    r.push(b'\t');
    append(&mut r, &m.sha);
    r.push(b'\t');
    let n = decimal(m.size);
    append(&mut r, &n);
    r.push(b'\n');
    proof {
        reveal_byteslit(b"member\t");
        reveal_strlit("member\t");
        reveal(ascii);
    }
    r
}

pub fn rows(ms: &Vec<EMember>) -> (r: Vec<u8>)
    ensures
        r@ == member_rows(members(ms@)),
{
    let mut r = Vec::new();
    let mut i = 0usize;
    let ghost f = |m: Member|
        ascii("member\t"@) + m.path + tab() + m.sha + tab() + ckc_spec::check::nat_bytes(m.size)
            + lf();
    while i < ms.len()
        invariant
            i <= ms.len(),
            r@ == members(ms@).take(i as int).map_values(f).flatten(),
            f == (|m: Member|
                ascii("member\t"@) + m.path + tab() + m.sha + tab() + ckc_spec::check::nat_bytes(
                    m.size,
                ) + lf()),
        decreases ms.len() - i,
    {
        let next = member_row(&ms[i]);
        append(&mut r, &next);
        proof {
            members(ms@).lemma_map_take_succ(f, i as int);
            members(ms@).take(i as int).map_values(f).lemma_flatten_push(f(ms@[i as int]@));
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(members(ms@).take(i as int) == members(ms@));
    }
    r
}

pub fn source_text(ss: &Vec<(EMember, Vec<u8>)>) -> (r: Vec<u8>)
    ensures
        r@ == source_rows(source_views(ss@)),
{
    let mut r = Vec::new();
    let mut i = 0usize;
    let ghost f = |s: (Member, Seq<u8>)|
        ascii("source\t"@) + s.0.path + tab() + s.0.sha + tab() + ckc_spec::check::nat_bytes(
            s.0.size,
        ) + tab() + s.1 + lf();
    proof {
        reveal_byteslit(b"source\t");
        reveal_strlit("source\t");
        reveal(ascii);
    }
    while i < ss.len()
        invariant
            i <= ss.len(),
            r@ == source_views(ss@).take(i as int).map_values(f).flatten(),
            f == (|s: (Member, Seq<u8>)|
                ascii("source\t"@) + s.0.path + tab() + s.0.sha + tab()
                    + ckc_spec::check::nat_bytes(s.0.size) + tab() + s.1 + lf()),
        decreases ss.len() - i,
    {
        let x = &ss[i];
        let mut row = copy(b"source\t");
        append(&mut row, &x.0.path);
        row.push(b'\t');
        append(&mut row, &x.0.sha);
        row.push(b'\t');
        let n = decimal(x.0.size);
        append(&mut row, &n);
        row.push(b'\t');
        append(&mut row, &x.1);
        row.push(b'\n');
        proof {
            reveal_byteslit(b"source\t");
            reveal_strlit("source\t");
            reveal(ascii);
            assert_seqs_equal!(b"source\t"@ == ascii("source\t"@));
            assert(row@ == f((x.0@, x.1@)));
        }
        append(&mut r, &row);
        proof {
            source_views(ss@).lemma_map_take_succ(f, i as int);
            source_views(ss@).take(i as int).map_values(f).lemma_flatten_push(f((x.0@, x.1@)));
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(source_views(ss@).take(i as int) == source_views(ss@));
    }
    r
}

pub fn label_text(ls: &Vec<(Vec<u8>, Vec<u8>)>) -> (r: Vec<u8>)
    ensures
        r@ == label_rows(byte_pairs(ls@)),
{
    let mut r = Vec::new();
    let mut i = 0usize;
    let ghost f = |l: (Seq<u8>, Seq<u8>)| ascii("label\t"@) + l.0 + tab() + l.1 + lf();
    proof {
        reveal_byteslit(b"label\t");
        reveal_strlit("label\t");
        reveal(ascii);
    }
    while i < ls.len()
        invariant
            i <= ls.len(),
            r@ == byte_pairs(ls@).take(i as int).map_values(f).flatten(),
            f == (|l: (Seq<u8>, Seq<u8>)| ascii("label\t"@) + l.0 + tab() + l.1 + lf()),
        decreases ls.len() - i,
    {
        let x = &ls[i];
        let mut row = copy(b"label\t");
        append(&mut row, &x.0);
        row.push(b'\t');
        append(&mut row, &x.1);
        row.push(b'\n');
        proof {
            reveal_byteslit(b"label\t");
            reveal_strlit("label\t");
            reveal(ascii);
            assert_seqs_equal!(b"label\t"@ == ascii("label\t"@));
            assert(row@ == f((x.0@, x.1@)));
        }
        append(&mut r, &row);
        proof {
            byte_pairs(ls@).lemma_map_take_succ(f, i as int);
            byte_pairs(ls@).take(i as int).map_values(f).lemma_flatten_push(f((x.0@, x.1@)));
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(byte_pairs(ls@).take(i as int) == byte_pairs(ls@));
    }
    r
}

// m7t D8: one `meta schema` row per schema version among the staged guidelines.
pub fn has_table_exec(staged: &Vec<EMember>, gid: &[u8]) -> (r: bool)
    ensures
        r == has_table(members(staged@), gid@),
{
    let mut want = copy(b"guidelines/");
    append(&mut want, gid);
    append(&mut want, b"/temporal.tsv");
    proof {
        reveal_byteslit(b"guidelines/");
        reveal_strlit("guidelines/");
        reveal_byteslit(b"/temporal.tsv");
        reveal_strlit("/temporal.tsv");
        reveal(ascii);
        assert(b"guidelines/"@ =~= ascii("guidelines/"@));
        assert(b"/temporal.tsv"@ =~= ascii("/temporal.tsv"@));
    }
    let mut i = 0usize;
    while i < staged.len()
        invariant
            i <= staged@.len(),
            want@ == ascii("guidelines/"@) + gid@ + ascii("/temporal.tsv"@),
            forall|j: int| 0 <= j < i ==> #[trigger] members(staged@)[j].path != want@,
        decreases staged@.len() - i,
    {
        if eq(&staged[i].path, &want) {
            proof {
                assert(members(staged@)[i as int].path == want@);
            }
            return true;
        }
        i += 1;
    }
    false
}

pub fn schema_rows_exec(staged: &Vec<EMember>, profiles: &Vec<(Vec<u8>, Vec<u8>)>) -> (r: Vec<u8>)
    ensures
        r@ == schema_rows(members(staged@), byte_pairs(profiles@)),
{
    let ghost ms = members(staged@);
    let ghost ps = byte_pairs(profiles@);
    let mut table = false;
    let mut plain = false;
    let mut i = 0usize;
    while i < profiles.len()
        invariant
            i <= profiles@.len(),
            ms == members(staged@),
            ps == byte_pairs(profiles@),
            table == exists|j: int| 0 <= j < i && has_table(ms, #[trigger] ps[j].0),
            plain == exists|j: int| 0 <= j < i && !has_table(ms, #[trigger] ps[j].0),
        decreases profiles@.len() - i,
    {
        let h = has_table_exec(staged, &profiles[i].0);
        proof {
            assert(ps[i as int].0 == profiles@[i as int].0@);
        }
        if h {
            table = true;
        } else {
            plain = true;
        }
        proof {
            if table {
                if !h {
                    let j = choose|j: int| 0 <= j < i && has_table(ms, #[trigger] ps[j].0);
                    assert(0 <= j < i + 1 && has_table(ms, ps[j].0));
                }
            }
            if plain {
                if h {
                    let j = choose|j: int| 0 <= j < i && !has_table(ms, #[trigger] ps[j].0);
                    assert(0 <= j < i + 1 && !has_table(ms, ps[j].0));
                }
            }
            assert forall|j: int|
                0 <= j < i + 1 && has_table(ms, #[trigger] ps[j].0) implies table by {
                if j == i {
                    assert(h);
                }
            }
            assert forall|j: int|
                0 <= j < i + 1 && !has_table(ms, #[trigger] ps[j].0) implies plain by {
                if j == i {
                    assert(!h);
                }
            }
        }
        i += 1;
    }
    proof {
        assert(table == some_table(ms, ps));
        assert(plain == some_plain(ms, ps));
        reveal_byteslit(b"meta\tschema\tv1\n");
        reveal_strlit("meta\tschema\tv1\n");
        reveal_byteslit(b"meta\tschema\tv2\n");
        reveal_strlit("meta\tschema\tv2\n");
        reveal(ascii);
        assert(b"meta\tschema\tv1\n"@ =~= ascii("meta\tschema\tv1\n"@));
        assert(b"meta\tschema\tv2\n"@ =~= ascii("meta\tschema\tv2\n"@));
    }
    let mut r = Vec::new();
    if !table || plain {
        append(&mut r, b"meta\tschema\tv1\n");
    }
    if table {
        append(&mut r, b"meta\tschema\tv2\n");
    }
    proof {
        assert(r@ =~= schema_rows(ms, ps));
    }
    r
}

pub fn meta(schema: &[u8], compiler: &[u8], lexicon: &[u8]) -> (r: Vec<u8>)
    ensures
        r@ == meta_block(schema@, compiler@, lexicon@),
{
    let mut r = copy(schema);
    append(&mut r, b"meta\tcompiler\t");
    append(&mut r, compiler);
    append(&mut r, b"\nmeta\tbase-lexicon\t");
    append(&mut r, lexicon);
    append(
        &mut r,
        b"\nmeta\tswipl\t9.2.9\nmeta\tverify\tsha256sum -c manifest-sha256.txt tagmanifest-sha256.txt\nmeta\treplay\tcompile: ckc compile <guideline-id>\nmeta\treplay\tcheck: ckc check\nmeta\treplay\tload: swipl -q -s data/guidelines/<guideline-id>/pl/<docid>.pl\nmeta\tgenerated\trelease-manifest.tsv\nmeta\tgenerated\tmanifest-sha256.txt\nmeta\tgenerated\ttagmanifest-sha256.txt\n",
    );
    proof {
        reveal_byteslit(b"meta\tcompiler\t");
        reveal_strlit("meta\tcompiler\t");
        reveal_byteslit(b"\nmeta\tbase-lexicon\t");
        reveal_strlit("\nmeta\tbase-lexicon\t");
        reveal_byteslit(
            b"\nmeta\tswipl\t9.2.9\nmeta\tverify\tsha256sum -c manifest-sha256.txt tagmanifest-sha256.txt\nmeta\treplay\tcompile: ckc compile <guideline-id>\nmeta\treplay\tcheck: ckc check\nmeta\treplay\tload: swipl -q -s data/guidelines/<guideline-id>/pl/<docid>.pl\nmeta\tgenerated\trelease-manifest.tsv\nmeta\tgenerated\tmanifest-sha256.txt\nmeta\tgenerated\ttagmanifest-sha256.txt\n",
        );
        reveal_strlit(
            "\nmeta\tswipl\t9.2.9\nmeta\tverify\tsha256sum -c manifest-sha256.txt tagmanifest-sha256.txt\nmeta\treplay\tcompile: ckc compile <guideline-id>\nmeta\treplay\tcheck: ckc check\nmeta\treplay\tload: swipl -q -s data/guidelines/<guideline-id>/pl/<docid>.pl\nmeta\tgenerated\trelease-manifest.tsv\nmeta\tgenerated\tmanifest-sha256.txt\nmeta\tgenerated\ttagmanifest-sha256.txt\n",
        );
        reveal(ascii);
    }
    r
}

} // verus!
