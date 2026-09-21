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

// M5.2b K3: trace mode + trace-check (contract m5u2b). The shell hashes: it
// asks the kernel which lines (one canonical clause line per loaded clause,
// in `db_of` order), hashes each with the vendored sha2, and hands the
// lowercase-hex digests back; qsha/asha = hex sha256 of the raw query/answers
// bytes (R3). A mode's result = the exact triple of its spec fn.
pub fn v1_trace_lines(
    mpath: &[u8],
    m: &ckc_spec::replay::ESrc,
    pls: &Vec<ckc_spec::replay::ESrc>,
    pys: &Vec<ckc_spec::replay::ESrc>,
    query: &ckc_spec::replay::ESrc,
    qsha: &[u8],
    answers: &ckc_spec::replay::ESrc,
) -> (r: Result<Vec<Vec<u8>>, ckc_spec::replay::EOut>)
    requires
        ckc_spec::replay::cells_ok(m@, ckc_spec::replay::srcs(pls@), ckc_spec::replay::srcs(pys@)),
    ensures
        ckc_spec::trace::lines_view(r) == ckc_spec::trace::trace_lines(
            mpath@,
            m@,
            ckc_spec::replay::srcs(pls@),
            ckc_spec::replay::srcs(pys@),
            query@,
            qsha@,
            answers@,
        ),
{
    crate::k3_impl::v1_trace_lines_impl(mpath, m, pls, pys, query, qsha, answers)
}

pub fn v1_trace(
    mpath: &[u8],
    m: &ckc_spec::replay::ESrc,
    pls: &Vec<ckc_spec::replay::ESrc>,
    pys: &Vec<ckc_spec::replay::ESrc>,
    query: &ckc_spec::replay::ESrc,
    qsha: &[u8],
    answers: &ckc_spec::replay::ESrc,
    asha: &[u8],
    digests: &Vec<Vec<u8>>,
) -> (r: ckc_spec::replay::EOut)
    requires
        ckc_spec::replay::cells_ok(m@, ckc_spec::replay::srcs(pls@), ckc_spec::replay::srcs(pys@)),
    ensures
        r@ == ckc_spec::trace::trace_output(
            mpath@,
            m@,
            ckc_spec::replay::srcs(pls@),
            ckc_spec::replay::srcs(pys@),
            query@,
            qsha@,
            answers@,
            asha@,
            ckc_spec::trace::digests_view(digests@),
        ),
{
    crate::k3_impl::v1_trace_impl(mpath, m, pls, pys, query, qsha, answers, asha, digests)
}

pub fn v1_trace_check(
    mpath: &[u8],
    m: &ckc_spec::replay::ESrc,
    pls: &Vec<ckc_spec::replay::ESrc>,
    pys: &Vec<ckc_spec::replay::ESrc>,
    query: &ckc_spec::replay::ESrc,
    qsha: &[u8],
    answers: &ckc_spec::replay::ESrc,
    asha: &[u8],
    trace: &ckc_spec::replay::ESrc,
    digests: &Vec<Vec<u8>>,
) -> (r: ckc_spec::replay::EOut)
    requires
        ckc_spec::replay::cells_ok(m@, ckc_spec::replay::srcs(pls@), ckc_spec::replay::srcs(pys@)),
    ensures
        r@ == ckc_spec::trace::trace_check_output(
            mpath@,
            m@,
            ckc_spec::replay::srcs(pls@),
            ckc_spec::replay::srcs(pys@),
            query@,
            qsha@,
            answers@,
            asha@,
            trace@,
            ckc_spec::trace::digests_view(digests@),
        ),
{
    crate::k3_impl::v1_trace_check_impl(
        mpath,
        m,
        pls,
        pys,
        query,
        qsha,
        answers,
        asha,
        trace,
        digests,
    )
}

// K3 soundness (proved ⇒ derivable): every forest the trace derivation
// produces is a valid proof tree over the loaded program — each clause node
// resolves its goal through its clause, each naf leaf certifies a bounded
// finite failure of a generalization of the site goal.
pub proof fn k3_sound(db: Seq<ckc_spec::v1text::DocClause>, goal: ckc_spec::term::Term)
    requires
        ckc_spec::trace::bodies_wf(db),
        ckc_spec::answers::goal_walk(goal) is None,
        ckc_spec::trace::derived_forest(db, goal) is Some,
    ensures
        ckc_spec::trace::forest_valid(db, goal, ckc_spec::trace::derived_forest(db, goal).unwrap()),
{
    crate::k3_sound::k3_sound_proof(db, goal)
}

// M6 emission certification (contract m6, R40–R43). The shell stages APE,
// runs the trusted driver (twice, byte-equal), hashes the raw ACE/ulex
// bytes (R3) and hands everything to the kernel; the result = the exact
// triple of the spec fn. usha = None when the document declares ulex(none).
pub fn certify_doc(
    ace: &[u8],
    asha: &[u8],
    usha: Option<&Vec<u8>>,
    docid: &[u8],
    dump: &[u8],
    pl: &[u8],
) -> (r: ckc_spec::replay::EOut)
    ensures
        r@ == ckc_spec::emit::certify_doc_output(
            ace@,
            asha@,
            ckc_spec::emit::opt_view(usha),
            docid@,
            dump@,
            pl@,
        ),
{
    crate::m6_impl::certify_doc_impl(ace, asha, usha, docid, dump, pl)
}

pub fn certify_query(
    ace: &[u8],
    asha: &[u8],
    usha: Option<&Vec<u8>>,
    qid: &[u8],
    dump: &[u8],
    pl: &[u8],
) -> (r: ckc_spec::replay::EOut)
    ensures
        r@ == ckc_spec::emit::certify_query_output(
            ace@,
            asha@,
            ckc_spec::emit::opt_view(usha),
            qid@,
            dump@,
            pl@,
        ),
{
    crate::m6_impl::certify_query_impl(ace, asha, usha, qid, dump, pl)
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
    crate::k4_impl::check_coverage_impl(bytes, docids, files, root)
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

pub fn check_parse_manifest(src: &ckc_spec::replay::ESrc, path: &[u8]) -> (r: (
    Vec<ckc_spec::check::EBundle>,
    Option<Vec<u8>>,
))
    ensures
        (ckc_spec::check::bundles(r.0@), ckc_spec::check::optional_bytes(r.1))
            == ckc_spec::check::parse_manifest(src@, path@),
{
    crate::k4_impl::check_parse_manifest_impl(src, path)
}

pub fn check_ledger(src: &ckc_spec::replay::ESrc, known: &Vec<Vec<u8>>) -> (r: (
    Vec<ckc_spec::check::EDecision>,
    Option<ckc_spec::check::EVerdict>,
))
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

// M5.4 pipeline (contract m5u4 P1): the alignment resolver (occurrence-form
// input → committed align table, or the exact fail message) and the release
// manifest (meta block + sorted member/source/label rows) as pure functions of
// their spec; the shell reads files, hashes members and stages the bag.
pub fn align_resolve(input: &[char], src: &[char], ace: &[char]) -> (r: ckc_spec::align::EResolve)
    ensures
        r@ == ckc_spec::align::resolve_outcome(input@, src@, ace@),
{
    crate::release_impl::align_resolve_impl(input, src, ace)
}

pub fn release_manifest(
    head: &[u8],
    compiler: &[u8],
    lexicon: &[u8],
    staged: &Vec<ckc_spec::release::EMember>,
    profiles: &Vec<(Vec<u8>, Vec<u8>)>,
    urls: &Vec<(Vec<u8>, Vec<u8>)>,
    labels: &Vec<(Vec<u8>, Vec<u8>)>,
    tags: &Vec<ckc_spec::release::EMember>,
) -> (r: Vec<u8>)
    ensures
        r@ == ckc_spec::release::release_manifest(
            head@,
            compiler@,
            lexicon@,
            ckc_spec::release::members(staged@),
            ckc_spec::check::byte_pairs(profiles@),
            ckc_spec::check::byte_pairs(urls@),
            ckc_spec::check::byte_pairs(labels@),
            ckc_spec::release::members(tags@),
        ),
{
    crate::release_impl::release_manifest_impl(
        head,
        compiler,
        lexicon,
        staged,
        profiles,
        urls,
        labels,
        tags,
    )
}

// M5.6 dist (contract m5u6, shell tier over a pinned gzip dep): the two BagIt
// digest manifests are pure functions of the member list; tar/gzip bytes, member
// digests and publish semantics are shell code graded by the 62-case battery.
pub fn dist_digest_lines(ms: &Vec<ckc_spec::release::EMember>) -> (r: Vec<u8>)
    ensures
        r@ == ckc_spec::release::digest_lines(ckc_spec::release::members(ms@)),
{
    crate::release_impl::dist_digest_lines_impl(ms)
}

pub fn dist_tagmanifest_lines(tags: &Vec<ckc_spec::release::EMember>) -> (r: Vec<u8>)
    ensures
        r@ == ckc_spec::release::tagmanifest_lines(ckc_spec::release::members(tags@)),
{
    crate::release_impl::dist_tagmanifest_lines_impl(tags)
}

} // verus!
