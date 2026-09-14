use vstd::prelude::*;

verus! {

// TRUSTED binding surface (spec-manifest-pinned): the kernel's public API
// with ensures binding every exec result to ckc-spec. Bodies delegate to
// uninspected impl modules; `cargo verus verify` enforces the binding.
pub fn align_check(align: &[char], src: &[char], ace: &[char]) -> (r: ckc_spec::align::ECheck)
    ensures
        r@ == ckc_spec::align::align_outcome(align@, src@, ace@),
{
    crate::align_impl::align_check_impl(align, src, ace)
}

// M5.2 K1: v1 clause-file acceptance (contract m5u2 R9/R10). The Reject
// offset = deterministic diagnostic (first divergence), pinned by fixtures,
// outside the theorem beyond its bound.
pub fn v1_check(bytes: &[u8]) -> (r: ckc_spec::v1text::EV1Verdict)
    ensures
        r.is_accept() <==> ckc_spec::v1text::accepts(bytes@),
        r matches ckc_spec::v1text::EV1Verdict::Reject { at } ==> at <= bytes@.len(),
{
    crate::v1_impl::v1_check_impl(bytes)
}

// M5.2 K2: composition modes (contract m5u2 R24). Every file the shell read
// arrives as an `ESrc`; a mode's result = the exact (rc, stdout, stderr)
// triple of its spec fn over those sources.
pub fn v1_manifest(mpath: &[u8], m: &ckc_spec::replay::ESrc) -> (r: Result<
    Vec<ckc_spec::replay::ERow>,
    ckc_spec::replay::EOut,
>)
    ensures
        ckc_spec::replay::rows_view(r) == ckc_spec::replay::manifest_rows(mpath@, m@),
{
    crate::k2_impl::v1_manifest_impl(mpath, m)
}

pub fn v1_aggregate_check(
    mpath: &[u8],
    m: &ckc_spec::replay::ESrc,
    pls: &Vec<ckc_spec::replay::ESrc>,
    pys: &Vec<ckc_spec::replay::ESrc>,
) -> (r: ckc_spec::replay::EOut)
    requires
        ckc_spec::replay::cells_ok(m@, ckc_spec::replay::srcs(pls@), ckc_spec::replay::srcs(pys@)),
    ensures
        r@ == ckc_spec::replay::agg_output(
            mpath@,
            m@,
            ckc_spec::replay::srcs(pls@),
            ckc_spec::replay::srcs(pys@),
        ),
{
    crate::k2_impl::v1_aggregate_check_impl(mpath, m, pls, pys)
}

pub fn v1_recursion_check(
    mpath: &[u8],
    m: &ckc_spec::replay::ESrc,
    pls: &Vec<ckc_spec::replay::ESrc>,
    pys: &Vec<ckc_spec::replay::ESrc>,
) -> (r: ckc_spec::replay::EOut)
    requires
        ckc_spec::replay::cells_ok(m@, ckc_spec::replay::srcs(pls@), ckc_spec::replay::srcs(pys@)),
    ensures
        r@ == ckc_spec::replay::recursion_output(
            mpath@,
            m@,
            ckc_spec::replay::srcs(pls@),
            ckc_spec::replay::srcs(pys@),
        ),
{
    crate::k2_impl::v1_recursion_check_impl(mpath, m, pls, pys)
}

// qsha = lowercase hex sha256 of the raw query bytes, computed by the shell (R3).
pub fn v1_answer(
    mpath: &[u8],
    m: &ckc_spec::replay::ESrc,
    pls: &Vec<ckc_spec::replay::ESrc>,
    pys: &Vec<ckc_spec::replay::ESrc>,
    query: &ckc_spec::replay::ESrc,
    qsha: &[u8],
) -> (r: ckc_spec::replay::EOut)
    requires
        ckc_spec::replay::cells_ok(m@, ckc_spec::replay::srcs(pls@), ckc_spec::replay::srcs(pys@)),
    ensures
        r@ == ckc_spec::answers::answer_output(
            mpath@,
            m@,
            ckc_spec::replay::srcs(pls@),
            ckc_spec::replay::srcs(pys@),
            query@,
            qsha@,
        ),
{
    crate::k2_impl::v1_answer_impl(mpath, m, pls, pys, query, qsha)
}

// M5.3 K4: custody sections; shell owns source reads and digest computation.
pub fn check_render(v: &ckc_spec::check::EVerdict) -> (r: ckc_spec::check::ERendered)
    ensures
        r@ == ckc_spec::check::render(v@),
{
    crate::k4_impl::check_render_impl(v)
}

pub fn check_coverage(
    bytes: &[u8],
    docids: &Vec<Vec<u8>>,
    files: &Vec<(Vec<u8>, ckc_spec::replay::ESrc)>,
) -> (r: Result<ckc_spec::check::ECoverage, ckc_spec::check::EVerdict>)
    ensures
        ckc_spec::check::coverage_result(r) == ckc_spec::check::coverage(
            bytes@,
            ckc_spec::check::byte_rows(docids@),
            |f: Seq<u8>|
                ckc_spec::check::source_lookup(ckc_spec::check::source_pairs(files@), f, 0),
        ),
{
    crate::k4_impl::check_coverage_impl(bytes, docids, files)
}

pub fn check_coverage_meter(gid: &[u8], c: &ckc_spec::check::ECoverage) -> (r: Vec<u8>)
    ensures
        r@ == ckc_spec::check::coverage_meter(gid@, c@.rows),
{
    crate::k4_impl::check_coverage_meter_impl(gid, c)
}

pub fn check_payload(c: &ckc_spec::check::ECoverage, docid: &[u8]) -> (r: (
    Option<Vec<u8>>,
    Option<Vec<u8>>,
))
    requires
        exists|b: Seq<u8>, ds: Seq<Seq<u8>>, fs: spec_fn(Seq<u8>) -> ckc_spec::replay::Src|
            ckc_spec::check::coverage(b, ds, fs) == Result::Ok(c@),
    ensures
        ckc_spec::check::optional_bytes(r.0) == (match ckc_spec::check::ace_row(c@, docid@) {
            Option::Some(row) => Option::Some(row.line),
            Option::None => Option::None,
        }),
        ckc_spec::check::optional_bytes(r.1) == ckc_spec::check::payload(c@, docid@),
{
    crate::k4_impl::check_payload_impl(c, docid)
}

pub fn check_semantic_input(pl: &[u8], docid: &[u8]) -> (r: Result<Vec<u8>, Vec<u8>>)
    ensures
        ckc_spec::check::bytes_result(r) == ckc_spec::check::semantic_input(pl@, docid@),
{
    crate::k4_impl::check_semantic_input_impl(pl, docid)
}

pub fn check_bundle_block(b: &ckc_spec::check::EBundle) -> (r: Vec<u8>)
    ensures
        r@ == ckc_spec::check::bundle_block(b@.docid, b@.ace, b@.cov, b@.pay, b@.cl),
{
    crate::k4_impl::check_bundle_block_impl(b)
}

pub fn check_print_manifest(bs: &Vec<ckc_spec::check::EBundle>) -> (r: Vec<u8>)
    ensures
        r@ == ckc_spec::check::print_manifest(ckc_spec::check::bundles(bs@)),
{
    crate::k4_impl::check_print_manifest_impl(bs)
}

pub fn check_manifest_accepts(bytes: &[u8], hashes: &Vec<(Vec<u8>, Vec<u8>)>) -> (r: bool)
    ensures
        r == ckc_spec::check::manifest_accepts(
            bytes@,
            |b: Seq<u8>| ckc_spec::check::digest_lookup(ckc_spec::check::byte_pairs(hashes@), b, 0),
        ),
{
    crate::k4_impl::check_manifest_accepts_impl(bytes, hashes)
}

pub fn check_ledger(src: &ckc_spec::replay::ESrc, known: &Vec<Vec<u8>>) -> (r: Result<
    Vec<ckc_spec::check::EDecision>,
    ckc_spec::check::EVerdict,
>)
    ensures
        ckc_spec::check::ledger_result(r) == ckc_spec::check::ledger(
            src@,
            ckc_spec::check::byte_rows(known@),
        ),
{
    crate::k4_impl::check_ledger_impl(src, known)
}

pub fn check_class_count(
    ds: &Vec<ckc_spec::check::EDecision>,
    bs: &Vec<ckc_spec::check::EBundle>,
    k: i64,
) -> (r: usize)
    ensures
        r as nat == ckc_spec::check::class_count(
            ckc_spec::check::decisions(ds@),
            ckc_spec::check::bundles(bs@),
            k as int,
        ),
{
    crate::k4_impl::check_class_count_impl(ds, bs, k)
}

pub fn check_adjudication_meter(
    gid: &[u8],
    ds: &Vec<ckc_spec::check::EDecision>,
    bs: &Vec<ckc_spec::check::EBundle>,
) -> (r: Vec<u8>)
    ensures
        r@ == ckc_spec::check::adjudication_meter(
            gid@,
            ckc_spec::check::decisions(ds@),
            ckc_spec::check::bundles(bs@),
        ),
{
    crate::k4_impl::check_adjudication_meter_impl(gid, ds, bs)
}

pub fn check_lexicon(
    path: &[u8],
    ulex: &[u8],
    clex: &[u8],
    ace: &Vec<Vec<u8>>,
    rulings: &Vec<(Vec<u8>, Vec<u8>)>,
) -> (r: ckc_spec::check::EVerdict)
    ensures
        r@ == ckc_spec::check::lexicon(
            path@,
            ulex@,
            clex@,
            ckc_spec::check::byte_rows(ace@),
            ckc_spec::check::byte_pairs(rulings@),
        ),
{
    crate::k4_impl::check_lexicon_impl(path, ulex, clex, ace, rulings)
}

} // verus!
