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
        ckc_spec::trace::derived_forest(db, goal) is Some,
    ensures
        ckc_spec::trace::forest_valid(db, goal, ckc_spec::trace::derived_forest(db, goal).unwrap()),
{
    crate::k3_impl::k3_sound_impl(db, goal)
}

} // verus!
