use vstd::prelude::*;

verus! {

pub fn check_render_impl(v: &ckc_spec::check::EVerdict) -> (r: ckc_spec::check::ERendered)
    ensures
        r@ == ckc_spec::check::render(v@),
{
    crate::k4_render::verdict(v)
}

pub fn check_coverage_impl(
    bytes: &[u8],
    docids: &Vec<Vec<u8>>,
    files: &Vec<(Vec<u8>, ckc_spec::check::EFileSrc)>,
    root: &[u8],
) -> (r: Result<ckc_spec::check::ECoverage, ckc_spec::check::EVerdict>)
    ensures
        ckc_spec::check::coverage_result(r) == ckc_spec::check::coverage(
            bytes@,
            ckc_spec::check::byte_rows(docids@),
            |f: Seq<u8>|
                ckc_spec::check::source_lookup(ckc_spec::check::source_pairs(files@), f, 0),
            root@,
        ),
{
    assert(false);
    Err(ckc_spec::check::EVerdict::Ok(Vec::new()))
}

pub fn check_coverage_meter_impl(gid: &[u8], c: &ckc_spec::check::ECoverage) -> (r: Vec<u8>)
    ensures
        r@ == ckc_spec::check::coverage_meter(gid@, c@.rows),
{
    assert(false);
    Vec::new()
}

pub fn check_payload_impl(c: &ckc_spec::check::ECoverage, docid: &[u8]) -> (r: (
    Option<Vec<u8>>,
    Option<Vec<u8>>,
))
    ensures
        ckc_spec::check::optional_bytes(r.0) == (match ckc_spec::check::ace_row(c@, docid@) {
            Option::Some(row) => Option::Some(row.line),
            Option::None => Option::None,
        }),
        ckc_spec::check::optional_bytes(r.1) == ckc_spec::check::payload(c@, docid@),
{
    crate::k4_payload::select(c, docid)
}

pub fn check_semantic_input_impl(pl: &[u8], docid: &[u8]) -> (r: Result<Vec<u8>, Vec<u8>>)
    ensures
        ckc_spec::check::bytes_result(r) == ckc_spec::check::semantic_input(pl@, docid@),
{
    crate::k4_semantic::semantic(pl, docid)
}

pub fn check_bundle_block_impl(b: &ckc_spec::check::EBundle) -> (r: Vec<u8>)
    ensures
        r@ == ckc_spec::check::bundle_block(b@.docid, b@.ace, b@.cov, b@.pay, b@.cl),
{
    crate::k4_bundle::bundle(b)
}

pub fn check_print_manifest_impl(bs: &Vec<ckc_spec::check::EBundle>) -> (r: Vec<u8>)
    ensures
        r@ == ckc_spec::check::print_manifest(ckc_spec::check::bundles(bs@)),
{
    crate::k4_bundle::manifest(bs)
}

pub fn check_parse_manifest_impl(src: &ckc_spec::replay::ESrc, path: &[u8]) -> (r: (
    Vec<ckc_spec::check::EBundle>,
    Option<Vec<u8>>,
))
    ensures
        (ckc_spec::check::bundles(r.0@), ckc_spec::check::optional_bytes(r.1))
            == ckc_spec::check::parse_manifest(src@, path@),
{
    assert(false);
    (Vec::new(), None)
}

pub fn check_ledger_impl(src: &ckc_spec::replay::ESrc, known: &Vec<Vec<u8>>) -> (r: Result<
    Vec<ckc_spec::check::EDecision>,
    ckc_spec::check::EVerdict,
>)
    ensures
        ckc_spec::check::ledger_result(r) == ckc_spec::check::ledger(
            src@,
            ckc_spec::check::byte_rows(known@),
        ),
{
    crate::k4_ledger::validate(src, known)
}

pub fn check_class_count_impl(
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
    crate::k4_classify::count(ds, bs, k)
}

pub fn check_adjudication_meter_impl(
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
    crate::k4_classify::meter(gid, ds, bs)
}

pub fn check_lexicon_impl(
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
    assert(false);
    ckc_spec::check::EVerdict::Ok(Vec::new())
}

} // verus!
