//! Untrusted solver + transport adapter; no matching/checking predicates live here.
use crate::core::{self, May, Program, RawValue, Result};
use sb1_ta::domain::Value;
use sb1_ta::features::Feature;
use sb1_ta::hints::MayHint;

fn value(v: Value) -> RawValue {
    match v {
        Value::Any => RawValue { kind: 0, at: 0 },
        Value::Number => RawValue { kind: 1, at: 0 },
        Value::Patient => RawValue { kind: 2, at: 0 },
        Value::Exact(t) => RawValue { kind: 3, at: t },
        Value::Origin(o) => RawValue { kind: 4, at: o },
    }
}
pub fn decode_model(p: &Program, hint: MayHint) -> Result<May> {
    let mut model = May::begin(&p.arena, hint.generic, &hint.origins)?;
    // Convert one root vector at a time; its census-side Rc drops immediately.
    for set in hint.sets {
        match set {
            Some(values) => model.add_set(Some(values.iter().copied().map(value)))?,
            None => model.add_set(None::<std::iter::Empty<RawValue>>)?,
        }
    }
    for (feature, set) in hint.unary {
        let (kind, predicate, v) = match feature {
            Feature::Noun(v) => (0, 0, v),
            Feature::Lemma(v) => (1, 0, v),
            Feature::Operator(v) => (2, 0, v),
            Feature::Property(v) => (3, 0, v),
            Feature::Payload(p, v) => (4, p as u32, v),
            Feature::Tag(v) => (5, 0, v),
        };
        model.add_unary(kind, predicate, value(v), set)?;
    }
    for (label, owner, source, target) in hint.rectangles {
        model.add_rect(label.0 as u32, value(label.1), owner, source, target)?;
    }
    model.finish(
        &p.arena,
        hint.predicates.into_iter().map(|p| p as u32).collect(),
    )?;
    Ok(model)
}
pub use core::growth::Evidence as FlowEvidence;
pub fn decode_flow(p: &Program, m: &May, hint: sb1_ta::hints::FlowHint) -> Result<FlowEvidence> {
    use core::growth::{Source, Sources, Types};
    use sb1_ta::growth::Origin;
    let types = Types::new(p, m)?;
    let mut seen = vec![false; types.nodes.len()];
    seen[0] = true;
    let mut mapping = Vec::new();
    for node in hint.nodes {
        let ty = match node {
            Origin::Skolem(o) => {
                let v = m
                    .input_origins
                    .get(o)
                    .copied()
                    .ok_or_else(|| core::bad("source-node origin index"))?;
                types.identity(
                    m.abs
                        .origin(v)
                        .ok_or_else(|| core::bad("source-node nonorigin"))?,
                )
            }
            Origin::Arithmetic { clause, goal } => types
                .arithmetic(clause, goal)
                .ok_or_else(|| core::bad("source-node arithmetic index"))?,
        };
        if seen[ty as usize] {
            return Err(core::bad("duplicate source type node"));
        }
        seen[ty as usize] = true;
        mapping.push(ty);
    }
    if seen.iter().any(|v| !v) {
        return Err(core::bad("missing source type node"));
    }
    let rows = hint
        .sources
        .into_iter()
        .map(|s| {
            let choices = s
                .sources
                .iter()
                .map(|i| {
                    mapping
                        .get(*i)
                        .copied()
                        .ok_or_else(|| core::bad("source-hint type index"))
                })
                .collect::<Result<Vec<_>>>()?;
            Ok(Source {
                clause: s.clause,
                variable: s.variable,
                types: choices,
                binder: s.binder,
            })
        })
        .collect::<Result<Vec<_>>>()?;
    let sources = Sources::new(p, &types, rows)?;
    let ranks = hint
        .ranks
        .map(|values| {
            if values.len() != mapping.len() {
                return Err(core::bad("growth-rank length"));
            }
            let mut ranks = vec![0; types.nodes.len()];
            for (ty, r) in mapping.into_iter().zip(values) {
                ranks[ty as usize] = r;
            }
            Ok(ranks)
        })
        .transpose()?;
    Ok(FlowEvidence {
        types,
        sources,
        ranks,
    })
}
pub fn decode(p: &Program, hint: sb1_ta::hints::Hints) -> Result<core::analysis::Certificate> {
    use core::analysis::{Certificate, Refined};
    use core::refinement::{Key, Registry, Tag, Variant};
    let model = decode_model(p, hint.model)?;
    let flow = decode_flow(p, &model, hint.flow)?;
    let refined = hint
        .refined
        .map(|r| {
            let q = program(r.program);
            let qm = decode_model(&q, r.model)?;
            let qflow = decode_flow(&q, &qm, r.flow)?;
            let origin = |old: usize| -> Result<usize> {
                model
                    .input_origins
                    .get(old)
                    .and_then(|v| model.abs.origin(*v))
                    .ok_or_else(|| core::bad("k1 source-origin map"))
            };
            let tag = |t: sb1_ta::exact_k1::Tag| -> Result<Tag> {
                Ok(match t {
                    sb1_ta::exact_k1::Tag::Base => Tag::Scalar,
                    sb1_ta::exact_k1::Tag::Skolem(o) => Tag::Origin(origin(o)?),
                    sb1_ta::exact_k1::Tag::Arithmetic(c, g) => Tag::Arithmetic(c, g),
                })
            };
            let keys = r
                .registry
                .into_iter()
                .map(|k| {
                    Ok(Key {
                        origin: origin(k.template)?,
                        precise: k.precise,
                        deps: k.deps.into_iter().map(&tag).collect::<Result<Vec<_>>>()?,
                    })
                })
                .collect::<Result<Vec<_>>>()?;
            let variants = r
                .variants
                .into_iter()
                .map(|(original, tags)| {
                    Ok(Variant {
                        original: (original != usize::MAX).then_some(original),
                        tags: tags
                            .into_iter()
                            .map(|(v, t)| Ok((v, tag(t)?)))
                            .collect::<Result<Vec<_>>>()?,
                    })
                })
                .collect::<Result<Vec<_>>>()?;
            let registry = Registry::new(p, &model, &q, &qm, keys, variants, r.full_refinement)?;
            Ok(Refined {
                program: q,
                model: qm,
                flow: qflow,
                registry,
            })
        })
        .transpose()?;
    Ok(Certificate {
        version: hint.version,
        model,
        clause_ranks: hint.clause_ranks,
        flow,
        refined,
    })
}
pub fn program(p: sb1_ta::source::Program) -> Program {
    let patient = (0..p.clauses.len()).map(|i| p.patient(i)).collect();
    let documents = p
        .documents
        .into_iter()
        .map(|d| d.path.display().to_string())
        .collect();
    Program {
        arena: p.arena,
        clauses: p.clauses,
        patient,
        documents,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Budget, Failure};
    fn source(text: &str) -> sb1_ta::source::Program {
        let mut p = sb1_ta::source::Program::new();
        p.add("fixture".into(), text.into(), false).unwrap();
        p
    }
    #[test]
    fn complete_checker_admits_the_forwarding_k1_control() {
        let p = source(
            "guideline_entity(actual,seed_value,s,countable).\nguideline_entity(actual,'$guideline_id'(product,d,1,ref(1),[X]),z,countable) :- guideline_entity(actual,X,s,countable).\nguideline_entity(actual,'$guideline_id'(product,d,2,ref(1),[X]),w,countable) :- guideline_entity(actual,X,z2,countable).\nguideline_entity(actual,other_value,q,countable).\nguideline_entity(actual,'$guideline_id'(product,d,2,ref(1),[X]),w2,countable) :- guideline_entity(actual,X,q,countable).\nguideline_entity(actual,'$guideline_id'(product,d,1,ref(1),[X]),z,countable) :- guideline_entity(actual,X,w2,countable).\nguideline_entity(actual,X,z2,countable) :- guideline_entity(actual,X,z,countable).\n",
        );
        let a = sb1_ta::hints::analyze(&p).unwrap();
        assert!(a.admitted());
        assert!(a.refined.is_some());
        let h = sb1_ta::hints::export(&p, a).unwrap();
        let p = program(p);
        let cert = decode(&p, h).unwrap();
        let checked = core::analysis::check(p, cert, &mut Budget::default()).unwrap();
        assert_eq!(checked.graph.nodes(), 7);
    }
    #[test]
    fn independent_seeds_transfer_and_signed_edges_match_named_chain() {
        let p = source(
            "guideline_entity(actual,'$guideline_id'(product,d,1,ref(1),[]),seed,countable).\nguideline_entity(actual,X,next,countable) :- guideline_entity(actual,X,seed,countable).\nguideline_exclusion(d) :- guideline_entity(actual,X,next,countable), \\+ guideline_entity(actual,X,absent,countable).\n",
        );
        let h = sb1_ta::hints::export(&p, sb1_ta::hints::analyze(&p).unwrap()).unwrap();
        let p = program(p);
        let m = decode_model(&p, h.model).unwrap();
        let mut budget = Budget::default();
        core::check_envelope(&p, &m, &mut budget).unwrap();
        let g = core::signed_graph(&p, &m, h.clause_ranks.as_deref(), &mut budget).unwrap();
        assert_eq!(g.row(0), &[]);
        assert_eq!(g.row(1), &[0]);
        assert_eq!(g.row(2), &[2]);
    }
    #[test]
    fn independent_closed_sources_and_every_constructor_input_descend() {
        let p = source(
            "guideline_entity(actual,'$guideline_id'(product,d,1,ref(1),[]),seed,countable).\nguideline_entity(actual,X,next,countable) :- guideline_entity(actual,X,seed,countable).\nguideline_entity(actual,'$guideline_id'(product,d,2,ref(1),[X]),out,countable) :- guideline_entity(actual,X,next,countable).\n",
        );
        let h = sb1_ta::hints::export(&p, sb1_ta::hints::analyze(&p).unwrap()).unwrap();
        let p = program(p);
        let m = decode_model(&p, h.model).unwrap();
        let mut f = decode_flow(&p, &m, h.flow).unwrap();
        let mut budget = Budget::default();
        core::check_envelope(&p, &m, &mut budget).unwrap();
        f.sources
            .complete_scalars(&p, &m, &f.types, &mut budget)
            .unwrap();
        f.sources.check(&p, &m, &f.types, &mut budget).unwrap();
        let g = core::growth::graph(
            &p,
            &m,
            &f.types,
            &f.sources,
            f.ranks.as_deref(),
            &mut budget,
        )
        .unwrap();
        assert!(g.targets.iter().any(|edge| edge >> 1 != 0));
        let row = f.sources.rows.iter_mut().find(|r| r.clause == 1).unwrap();
        row.types.retain(|t| *t == 0);
        assert!(matches!(
            f.sources.check(&p, &m, &f.types, &mut budget),
            Err(Failure::Certificate(_))
        ));
    }
    #[test]
    fn missing_seed_refuses_and_search_exhaustion_is_indeterminate() {
        let p = source(
            "guideline_entity(actual,'$guideline_id'(product,d,1,ref(1),[]),seed,countable).\nguideline_entity(actual,X,next,countable) :- guideline_entity(actual,X,seed,countable).\n",
        );
        let mut h = sb1_ta::hints::export(&p, sb1_ta::hints::analyze(&p).unwrap()).unwrap();
        h.model
            .unary
            .retain(|(f, _)| !matches!(f, Feature::Noun(_)));
        let p = program(p);
        let m = decode_model(&p, h.model).unwrap();
        assert!(matches!(
            core::check_envelope(&p, &m, &mut Budget::default()),
            Err(Failure::Certificate(_))
        ));
        let p = source(
            "guideline_entity(actual,a,seed,countable).\nguideline_entity(actual,X,next,countable) :- guideline_entity(actual,X,seed,countable).\n",
        );
        let h = sb1_ta::hints::export(&p, sb1_ta::hints::analyze(&p).unwrap()).unwrap();
        let p = program(p);
        let m = decode_model(&p, h.model).unwrap();
        assert!(matches!(
            core::check_envelope(&p, &m, &mut Budget { used: 0, limit: 0 }),
            Err(Failure::Indeterminate(_))
        ));
    }
}
