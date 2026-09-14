// K3 public bindings: custody, directed proofs, materialization, and committed joins.
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
    let mut arena = crate::k2_reject::empty_arena();
    let front = match crate::k3_front::front_exec(&mut arena, mpath, m, pls, pys, query, qsha, answers) {
        Err(o) => return o,
        Ok(f) => f,
    };
    let result = match crate::k3_rows::result_exec(&mut arena, &front.loaded.db, digests, &front.query, &front.answer) {
        Err(o) => return o,
        Ok(root) => root,
    };
    crate::k3_print::traces_exec(&mut arena, front.query.qid.as_slice(), qsha, asha, result)
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
    hide(ckc_spec::trace::front);
    hide(ckc_spec::trace::trace_custody);
    hide(ckc_spec::trace::trace_result);
    hide(ckc_spec::trace::result_join);
    let mut arena = crate::k2_reject::empty_arena();
    let front = match crate::k3_front::front_exec(&mut arena, mpath, m, pls, pys, query, qsha, answers) {
        Err(o) => return o,
        Ok(f) => f,
    };
    let ghost before_trace = arena.nodes@;
    let committed = match crate::k3_front::trace_custody_exec(
        &mut arena, trace, qsha, asha, &front.query.qid, Ghost(front.query.file@),
    ) {
        Err(o) => return o,
        Ok(t) => t,
    };
    proof { crate::k3_front::front_prefix(before_trace, arena.nodes@, &front); }
    let ghost before_derive = arena.nodes@;
    let derived = match crate::k3_rows::result_exec(&mut arena, &front.loaded.db, digests, &front.query, &front.answer) {
        Err(o) => return o,
        Ok(root) => root,
    };
    proof {
        crate::k3_front::front_prefix(before_derive, arena.nodes@, &front);
        assert(crate::k3_front::trace_ok(arena.nodes@, &committed));
    }
    if !crate::k2_walk::ground_root(&arena, derived)
        || !crate::k2_sort::term_equal(&arena, derived, committed.result) {
        let name: &[u8] = b"trace_check";
        let stale: &[u8] = b"stale";
        proof {
            reveal_byteslit(b"trace_check"); reveal_strlit("trace_check");
            reveal_byteslit(b"stale"); reveal_strlit("stale"); reveal(ckc_spec::v1text::ascii);
            assert(name@ == ckc_spec::v1text::ascii("trace_check"@));
            assert(stale@ == ckc_spec::v1text::ascii("stale"@));
        }
        return crate::k2_output::named_atom_error(name, stale, true);
    }
    match crate::k3_join::result_join_exec(&arena, &front.loaded.coords, digests, committed.result) {
        crate::k3_join::EJoin::Bad(why) => crate::k3_join::tc_fail_exec(&mut arena, why),
        crate::k3_join::EJoin::Ok(nodes) => crate::k3_join::tc_meter_exec(front.query.qid.as_slice(), nodes.len()),
    }
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
