//! Exact front-stage graph. Step 2 keeps explicit rows for differential grading.
use crate::domain::Counters;
use crate::exact::{Model, State};
use crate::exact_heads::Index;
use crate::features::Pattern;
use crate::graph::{Edge, Graph, body_signature};
use crate::source::Program;
use sb1_checker::program::Goal;
use sb1_checker::term::Id;
use std::collections::HashMap;

#[cfg(not(test))]
type MemoKey = Vec<usize>;
#[cfg(test)]
#[derive(Debug, Eq, PartialEq)]
struct MemoKey(Vec<usize>);
#[cfg(test)]
impl std::hash::Hash for MemoKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        if crate::faults::enabled(crate::faults::Kind::HashCollision) {
            crate::faults::take(crate::faults::Kind::HashCollision);
            std::hash::Hash::hash(&0usize, state);
        } else {
            std::hash::Hash::hash(&self.0, state);
        }
    }
}
fn memo_key(program: &Program, body: &[Goal]) -> MemoKey {
    let values = body_signature(&program.arena, body).0;
    #[cfg(test)]
    {
        if crate::faults::enabled(crate::faults::Kind::MemoAlias) {
            crate::faults::take(crate::faults::Kind::MemoAlias);
            MemoKey(Vec::new())
        } else {
            MemoKey(values)
        }
    }
    #[cfg(not(test))]
    {
        values
    }
}

struct Context<'a> {
    program: &'a Program,
    model: &'a Model,
    index: &'a Index,
    audit: bool,
    caller: usize,
}
impl Context<'_> {
    fn targets(
        &self,
        pattern: &Pattern,
        state: Option<&State>,
        goal: Id,
        naf: usize,
        counters: &mut Counters,
    ) -> Vec<usize> {
        let Some(state) = state else {
            return Vec::new();
        };
        self.index
            .candidates_in(
                self.program,
                self.model,
                goal,
                state,
                self.audit,
                Some(crate::exact_heads::QueryContext {
                    phase: "graph",
                    caller: self.caller,
                    pattern,
                    naf,
                }),
            )
            .into_iter()
            .filter(|to| {
                self.model
                    .forced(self.program, pattern, goal, *to, state, counters)
                    .is_some()
            })
            .collect()
    }
    fn body(&self, clause: usize, counters: &mut Counters) -> Vec<(bool, Vec<usize>)> {
        let (program, model) = (self.program, self.model);
        let c = &program.clauses[clause];
        let pattern = Pattern::body(&program.arena, &model.abs, &c.body, c.names.len(), None);
        let state = model.state(&program.arena, &pattern, counters);
        let mut rows = Vec::new();
        for (n, g) in c.body.iter().enumerate() {
            match g {
                Goal::Positive(t) => rows.push((
                    false,
                    self.targets(&pattern, state.as_ref(), *t, 0, counters),
                )),
                Goal::Not(inner) => {
                    let pattern = Pattern::body(
                        &program.arena,
                        &model.abs,
                        &c.body,
                        c.names.len(),
                        Some(inner),
                    );
                    let state = model.state(&program.arena, &pattern, counters);
                    for g in inner {
                        if let Goal::Positive(t) = g {
                            rows.push((
                                true,
                                self.targets(&pattern, state.as_ref(), *t, n + 1, counters),
                            ));
                        }
                    }
                }
                Goal::Builtin(_) => {}
            }
        }
        rows
    }
}

pub fn build(program: &Program, model: &mut Model, audit: bool) -> Graph {
    let index = Index::new(program, model, audit);
    let mut counters = Counters::default();
    let mut edges = vec![Vec::new(); program.clauses.len()];
    let mut cache = HashMap::new();
    let mut hits = 0;
    for (i, c) in program.clauses.iter().enumerate() {
        if program.patient(i) || c.body.is_empty() {
            continue;
        }
        let key = memo_key(program, &c.body);
        let rows = if let Some(rows) = cache.get(&key) {
            hits += 1;
            rows
        } else {
            cache.entry(key).or_insert_with(|| {
                Context {
                    program,
                    model,
                    index: &index,
                    audit,
                    caller: i,
                }
                .body(i, &mut counters)
            })
        };
        let mut row = rows
            .iter()
            .flat_map(|(negative, targets)| {
                targets.iter().map(|to| Edge {
                    to: *to,
                    negative: *negative,
                    growth: false,
                })
            })
            .collect::<Vec<_>>();
        row.sort_unstable();
        row.dedup();
        row.shrink_to_fit();
        edges[i] = row;
        model.maybe_compact(program);
    }
    let patterns = cache.len();
    drop(cache);
    model.compact(program);
    let mut graph = Graph::from_edges(edges, counters);
    graph.body_patterns = patterns;
    graph.body_cache_hits = hits;
    graph
}

pub fn build_compressed(
    program: &Program,
    model: &mut Model,
    audit: bool,
) -> crate::exact_scc::Compressed {
    build_selected(program, model, None, audit)
}

pub fn build_selected(
    program: &Program,
    model: &mut Model,
    selected: Option<&std::collections::BTreeSet<usize>>,
    audit: bool,
) -> crate::exact_scc::Compressed {
    let index = Index::selected(program, model, selected, audit);
    let mut counters = Counters::default();
    let mut builder = crate::exact_scc::Builder::new(program.clauses.len());
    let mut cache = HashMap::new();
    let mut hits = 0;
    for (i, c) in program.clauses.iter().enumerate() {
        if selected.is_some_and(|s| !s.contains(&i)) || program.patient(i) || c.body.is_empty() {
            continue;
        }
        let key = memo_key(program, &c.body);
        let row = if let Some(row) = cache.get(&key) {
            hits += 1;
            *row
        } else {
            *cache.entry(key).or_insert_with(|| {
                builder.calls(
                    Context {
                        program,
                        model,
                        index: &index,
                        audit,
                        caller: i,
                    }
                    .body(i, &mut counters),
                )
            })
        };
        builder.attach(i, row);
        model.maybe_compact(program);
    }
    let patterns = cache.len();
    drop(cache);
    drop(index);
    model.compact(program);
    let mut graph = builder.finish(counters);
    graph.body_patterns = patterns;
    graph.body_cache_hits = hits;
    graph
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shared_state_survivors_equal_native_graphs_on_plants() {
        for text in [
            include_str!("../plants/L1.pl.in"),
            include_str!("../plants/L2.pl.in"),
            include_str!("../plants/L3.pl.in"),
            include_str!("../plants/L4.pl.in"),
            include_str!("../plants/L5.pl.in"),
            include_str!("../plants/G1.pl.in"),
            include_str!("../plants/G2.pl.in"),
            include_str!("../plants/P3.pl.in"),
            include_str!("../plants/P4.pl.in"),
            include_str!("../plants/P5.pl.in"),
            include_str!("../plants/P6.pl.in"),
        ] {
            let mut p = Program::new();
            p.add("fixture".into(), text.into(), false).unwrap();
            let native = crate::features::Model::build(&p, None, true);
            let expected = Graph::build(&p, &native, None);
            let mut candidate = Model::build(&p, None, true, Some(native.abs.clone()), true);
            let actual = build(&p, &mut candidate, true);
            assert_eq!(actual.edges, expected.edges, "{text}");
            assert_eq!(actual.admitted(), expected.admitted());
        }
    }
}
