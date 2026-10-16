use ckc_spec::answers::*;
use ckc_spec::replay::*;
use vstd::prelude::*;

verus! {

pub fn v1_manifest_impl(mpath: &[u8], m: &ESrc) -> (r: Result<Vec<ERow>, EOut>)
    ensures
        rows_view(r) == manifest_rows(mpath@, m@),
{
    crate::k2_manifest::v1_manifest_impl(mpath, m)
}

pub fn v1_aggregate_check_impl(mpath: &[u8], m: &ESrc, pls: &Vec<ESrc>, pys: &Vec<ESrc>) -> (r:
    EOut)
    requires
        cells_ok(m@, srcs(pls@), srcs(pys@)),
    ensures
        r@ == agg_output(mpath@, m@, srcs(pls@), srcs(pys@)),
{
    hide(payload_stage);
    hide(key_check);
    hide(coverage_check);
    hide(first_unproved);
    let rows = match crate::k2_load::manifest_stage_exec(mpath, m, pls, pys) {
        Err(out) => return out,
        Ok(rows) => rows,
    };
    if rows.len() == 0 {
        return crate::k2_output::meter_out(0, 0, false);
    }
    let mut arena = crate::k2_reject::empty_arena();
    let loaded = match crate::k2_load::load_stage_exec(&mut arena, &rows, pls) {
        Err(out) => return out,
        Ok(loaded) => loaded,
    };
    if let Some(out) = crate::k2_load::assertions_exec(&arena, &rows, &loaded) {
        return out;
    }
    let ghost before_payload = arena.nodes@;
    let obs = match crate::k2_payload::payload_stage_exec(&mut arena, &rows, pys) {
        Err(out) => return out,
        Ok(obs) => obs,
    };
    proof {
        crate::k2_load::loaded_prefix(before_payload, arena.nodes@, &loaded);
    }
    let ghost before_key = arena.nodes@;
    let (failed, pairs) = crate::k2_aggregate::key_check_exec(&mut arena, &obs);
    if let Some(out) = failed {
        return out;
    }
    proof {
        crate::k2_load::loaded_prefix(before_key, arena.nodes@, &loaded);
        crate::k2_payload::obs_prefix(before_key, arena.nodes@, obs@);
    }
    let ghost before_coverage = arena.nodes@;
    let ghost models = crate::k2_payload::obs_view(before_coverage, obs@);
    if let Some(out) = crate::k2_aggregate::coverage_check_exec(
        &mut arena,
        &loaded.db,
        &pairs,
        Ghost(models),
    ) {
        return out;
    }
    proof {
        crate::k2_load::loaded_prefix(before_coverage, arena.nodes@, &loaded);
        crate::k2_payload::obs_prefix(before_coverage, arena.nodes@, obs@);
    }
    let ghost before_replay = arena.nodes@;
    if let Some(index) = crate::k2_aggregate::first_unproved_exec(&mut arena, &loaded.db, &obs) {
        proof {
            crate::k2_payload::obs_prefix(before_replay, arena.nodes@, obs@);
            assert(crate::k2_payload::ob_valid(arena.nodes@, &obs@[index as int]));
        }
        let name: &[u8] = b"obligation_failed";
        proof {
            reveal_byteslit(b"obligation_failed");
            reveal_strlit("obligation_failed");
            reveal(ckc_spec::v1text::ascii);
            assert(name@ == ckc_spec::v1text::ascii("obligation_failed"@));
        }
        return crate::k2_payload::obligation_error(&mut arena, &obs[index], name);
    }
    crate::k2_output::meter_out(rows.len(), obs.len(), false)
}

pub fn v1_recursion_check_impl(mpath: &[u8], m: &ESrc, pls: &Vec<ESrc>, pys: &Vec<ESrc>) -> (r:
    EOut)
    requires
        cells_ok(m@, srcs(pls@), srcs(pys@)),
    ensures
        r@ == recursion_output(mpath@, m@, srcs(pls@), srcs(pys@)),
{
    let rows = match crate::k2_load::manifest_stage_exec(mpath, m, pls, pys) {
        Err(out) => return out,
        Ok(rows) => rows,
    };
    if rows.len() == 0 {
        return crate::k2_output::meter_out(0, 0, true);
    }
    let mut arena = crate::k2_reject::empty_arena();
    let loaded = match crate::k2_load::load_stage_exec(&mut arena, &rows, pls) {
        Err(out) => return out,
        Ok(loaded) => loaded,
    };
    let indices = crate::k2_recursion::rule_census(&arena, &loaded.db);
    if let Some(index) = crate::k2_recursion::first_recursive(&mut arena, &loaded.db, &indices) {
        proof {
            assert(crate::k2_engine::clause_valid(arena.nodes@, &loaded.db@[index as int]));
        }
        if let Some(out) = crate::k2_recursion::left_recursive_error(
            &mut arena,
            &loaded.db[index],
        ) {
            return out;
        }
    }
    crate::k2_output::meter_out(rows.len(), indices.len(), true)
}

pub fn v1_answer_impl(
    mpath: &[u8],
    m: &ESrc,
    pls: &Vec<ESrc>,
    pys: &Vec<ESrc>,
    query: &ESrc,
    qsha: &[u8],
) -> (r: EOut)
    requires
        cells_ok(m@, srcs(pls@), srcs(pys@)),
    ensures
        r@ == answer_output(mpath@, m@, srcs(pls@), srcs(pys@), query@, qsha@),
{
    hide(custody);
    hide(composition);
    hide(answer_result);
    let rows = match crate::k2_load::manifest_stage_exec(mpath, m, pls, pys) {
        Err(out) => return out,
        Ok(rows) => rows,
    };
    let mut arena = crate::k2_reject::empty_arena();
    let query = match crate::k2_answers::custody_exec(&mut arena, query) {
        Err(out) => return out,
        Ok(query) => query,
    };
    let ghost before_composition = arena.nodes@;
    let loaded = match crate::k2_load::composition_exec(&mut arena, &rows, pls) {
        Err(out) => return out,
        Ok(loaded) => loaded,
    };
    proof {
        crate::k2_answers::query_prefix(before_composition, arena.nodes@, &query);
    }
    let result = match crate::k2_answers::answer_result_exec(&mut arena, &loaded.db, &query) {
        Err(out) => return out,
        Ok(root) => root,
    };
    crate::k2_answers::print_answers_exec(&mut arena, query.qid.as_slice(), qsha, result)
}

} // verus!
