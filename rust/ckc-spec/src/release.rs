use crate::check::*;
use crate::engine::*;
use crate::v1text::*;
use vstd::prelude::*;

verus! {

// Trusted spec: release-manifest derivation (contract m5u4 P5; legacy
// tools/dist.py derive_release, byte-exact). Kernel = the TSV the consumer's
// `sha256sum -c` anchors to: member partition by rights profile, row order,
// meta block. Shell = git reads (HEAD, archive, show), path grammar +
// regular-file checks, rights/label validators, README-dist/NOTICE prose,
// sha256 (R3) — it hands digests, sizes, profiles, urls and labels in.
pub ghost struct Member {
    pub path: Seq<u8>,
    pub sha: Seq<u8>,
    pub size: nat,
}

// First match; absent = empty (legacy `dict.get(k, "")`).
pub open spec fn lookup(v: Seq<(Seq<u8>, Seq<u8>)>, key: Seq<u8>) -> Seq<u8>
    decreases v.len(),
{
    if v.len() == 0 {
        Seq::empty()
    } else if v[0].0 == key {
        v[0].1
    } else {
        lookup(v.drop_first(), key)
    }
}

// `guidelines/<gid>/…` → gid (the second `/`-separated segment).
pub open spec fn gid_of(path: Seq<u8>) -> Seq<u8> {
    let segs = split_on(path, 0x2F);
    if segs.len() >= 2 {
        segs[1]
    } else {
        Seq::empty()
    }
}

pub open spec fn is_source(path: Seq<u8>) -> bool {
    starts(path, ascii("guidelines/"@) + gid_of(path) + ascii("/source/"@))
}

pub open spec fn data_member(m: Member) -> Member {
    Member { path: ascii("data/"@) + m.path, sha: m.sha, size: m.size }
}

// Shipped payload: every staged member of a non-restricted guideline except
// the source files of a reconstructable one.
pub open spec fn payload(staged: Seq<Member>, profiles: Seq<(Seq<u8>, Seq<u8>)>) -> Seq<Member> {
    staged.filter(
        |m: Member|
            lookup(profiles, gid_of(m.path)) != ascii("restricted"@) && !(lookup(
                profiles,
                gid_of(m.path),
            ) == ascii("reconstructable"@) && is_source(m.path)),
    ).map_values(|m: Member| data_member(m))
}

// Reconstructable source files: named with digest, size and fetch url.
pub open spec fn sources(
    staged: Seq<Member>,
    profiles: Seq<(Seq<u8>, Seq<u8>)>,
    urls: Seq<(Seq<u8>, Seq<u8>)>,
) -> Seq<(Member, Seq<u8>)> {
    staged.filter(
        |m: Member|
            lookup(profiles, gid_of(m.path)) == ascii("reconstructable"@) && is_source(m.path),
    ).map_values(|m: Member| (data_member(m), lookup(urls, gid_of(m.path))))
}

pub open spec fn insert_member(x: Member, s: Seq<Member>) -> Seq<Member>
    decreases s.len(),
{
    if s.len() == 0 {
        seq![x]
    } else if bytes_lt(x.path, s[0].path) {
        seq![x] + s
    } else {
        seq![s[0]] + insert_member(x, s.drop_first())
    }
}

pub open spec fn sort_members(s: Seq<Member>) -> Seq<Member>
    decreases s.len(),
{
    if s.len() == 0 {
        s
    } else {
        insert_member(s[0], sort_members(s.drop_first()))
    }
}

pub open spec fn lf() -> Seq<u8> {
    seq![0x0Au8]
}

pub open spec fn tab() -> Seq<u8> {
    seq![0x09u8]
}

pub open spec fn meta_block(head: Seq<u8>, compiler: Seq<u8>, lexicon: Seq<u8>) -> Seq<u8> {
    ascii("meta\tschema\tv1\nmeta\thead\t"@) + head + ascii("\nmeta\tcompiler\t"@) + compiler
        + ascii("\nmeta\tbase-lexicon\t"@) + lexicon + ascii(
        "\nmeta\tpython\t3.11\nmeta\tswipl\t9.2.9\nmeta\tverify\tsha256sum -c manifest-sha256.txt tagmanifest-sha256.txt\nmeta\treplay\tcompile: python3 -P tools/goal.py compile <guideline-id>\nmeta\treplay\tcheck: python3 -P tools/goal.py check\nmeta\treplay\tload: swipl -q -s data/guidelines/<guideline-id>/pl/<docid>.pl\nmeta\tgenerated\trelease-manifest.tsv\nmeta\tgenerated\tmanifest-sha256.txt\nmeta\tgenerated\ttagmanifest-sha256.txt\n"@,
    )
}

pub open spec fn member_rows(ms: Seq<Member>) -> Seq<u8> {
    ms.map_values(
        |m: Member| ascii("member\t"@) + m.path + tab() + m.sha + tab() + nat_bytes(m.size) + lf(),
    ).flatten()
}

pub open spec fn source_rows(ss: Seq<(Member, Seq<u8>)>) -> Seq<u8> {
    ss.map_values(
        |s: (Member, Seq<u8>)|
            ascii("source\t"@) + s.0.path + tab() + s.0.sha + tab() + nat_bytes(s.0.size) + tab()
                + s.1 + lf(),
    ).flatten()
}

pub open spec fn label_rows(ls: Seq<(Seq<u8>, Seq<u8>)>) -> Seq<u8> {
    ls.map_values(|l: (Seq<u8>, Seq<u8>)| ascii("label\t"@) + l.0 + tab() + l.1 + lf()).flatten()
}

// `staged` = the corpus members in path order; `labels` = (docid, class) in
// docid order; `tags` = bagit.txt / README-dist.md / NOTICE with their digests.
pub open spec fn release_manifest(
    head: Seq<u8>,
    compiler: Seq<u8>,
    lexicon: Seq<u8>,
    staged: Seq<Member>,
    profiles: Seq<(Seq<u8>, Seq<u8>)>,
    urls: Seq<(Seq<u8>, Seq<u8>)>,
    labels: Seq<(Seq<u8>, Seq<u8>)>,
    tags: Seq<Member>,
) -> Seq<u8> {
    meta_block(head, compiler, lexicon) + member_rows(
        sort_members(payload(staged, profiles) + tags),
    ) + source_rows(sources(staged, profiles, urls)) + label_rows(labels)
}

// --- exec mirror ---
pub struct EMember {
    pub path: Vec<u8>,
    pub sha: Vec<u8>,
    pub size: u64,
}

impl View for EMember {
    type V = Member;

    open spec fn view(&self) -> Member {
        Member { path: self.path@, sha: self.sha@, size: self.size as nat }
    }
}

pub open spec fn members(v: Seq<EMember>) -> Seq<Member> {
    v.map_values(|m: EMember| m@)
}

} // verus!
