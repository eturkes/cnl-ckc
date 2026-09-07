// M5.2b K3 seed: red stubs behind the contract bindings (prod fills).
use ckc_spec::replay::{EOut, ESrc};
use vstd::prelude::*;

verus! {

pub fn v1_trace_lines_impl(
    mpath: &[u8],
    m: &ESrc,
    pls: &Vec<ESrc>,
    pys: &Vec<ESrc>,
    query: &ESrc,
    qsha: &[u8],
    answers: &ESrc,
) -> (r: Result<Vec<Vec<u8>>, EOut>)
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
    let mut arena = crate::k2_reject::empty_arena();
    let front = match crate::k3_front::front_exec(
        &mut arena,
        mpath,
        m,
        pls,
        pys,
        query,
        qsha,
        answers,
    ) {
        Err(o) => return Err(o),
        Ok(front) => front,
    };
    Ok(crate::k3_print::db_lines(&arena, &front.loaded.db))
}

pub fn v1_trace_impl(
    mpath: &[u8],
    m: &ESrc,
    pls: &Vec<ESrc>,
    pys: &Vec<ESrc>,
    query: &ESrc,
    qsha: &[u8],
    answers: &ESrc,
    asha: &[u8],
    digests: &Vec<Vec<u8>>,
) -> (r: EOut)
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
    assert(false);  // SEED: unimplemented
    EOut { rc: 2, out: Vec::new(), err: Vec::new() }
}

pub fn v1_trace_check_impl(
    mpath: &[u8],
    m: &ESrc,
    pls: &Vec<ESrc>,
    pys: &Vec<ESrc>,
    query: &ESrc,
    qsha: &[u8],
    answers: &ESrc,
    asha: &[u8],
    trace: &ESrc,
    digests: &Vec<Vec<u8>>,
) -> (r: EOut)
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
    assert(false);  // SEED: unimplemented
    EOut { rc: 2, out: Vec::new(), err: Vec::new() }
}

pub proof fn k3_sound_impl(db: Seq<ckc_spec::v1text::DocClause>, goal: ckc_spec::term::Term)
    requires
        ckc_spec::trace::bodies_wf(db),
        ckc_spec::trace::derived_forest(db, goal) is Some,
    ensures
        ckc_spec::trace::forest_valid(db, goal, ckc_spec::trace::derived_forest(db, goal).unwrap()),
{
    assert(false);  // SEED: unimplemented
}

} // verus!
