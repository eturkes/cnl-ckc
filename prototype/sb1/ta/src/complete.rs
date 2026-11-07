//! Complete exact candidate + canonical diagnostics; no production soundness claim.
use crate::domain::Counters;
use crate::exact::{Model, State};
use crate::exact_growth::Growth;
use crate::exact_heads::Index;
use crate::exact_k1::Refined;
use crate::exact_scc::Compressed;
use crate::exact_sets::EMPTY;
use crate::features::{Expr, Pattern};
use crate::graph::Question;
use crate::source::Program;
use sb1_checker::program::Goal;
use sb1_checker::term::Id;
use std::collections::BTreeSet;
use std::path::Path;

pub struct Analysis {
    pub model: Model,
    pub graph: Compressed,
    pub growth: Growth,
    pub refined: Option<Refined>,
}
impl Analysis {
    pub fn from_parts(
        p: &Program,
        mut model: Model,
        graph: Compressed,
        growth: Growth,
        audit: bool,
        skip_audit: bool,
    ) -> Result<Self, String> {
        let refined = if graph.admitted() && !growth.admitted() {
            Some(Refined::build(p, &mut model, &growth, audit, skip_audit)?)
        } else {
            None
        };
        Ok(Self {
            model,
            graph,
            growth,
            refined,
        })
    }
    pub fn admitted(&self) -> bool {
        self.graph.admitted()
            && self
                .refined
                .as_ref()
                .map_or_else(|| self.growth.admitted(), Refined::admitted)
    }
    pub fn slice(&self, p: &Program, q: &Question, audit: bool, skip_audit: bool) -> Slice {
        let mut counters = Counters::default();
        let index = Index::new(p, &self.model, audit);
        let root = roots(q, p, &self.model, &index, audit, &mut counters);
        let mut coarse = self.graph.reachable(&root);
        coarse.extend((0..p.clauses.len()).filter(|i| p.patient(*i)));
        let mut model = Model::build(
            p,
            Some(&coarse),
            false,
            Some(self.model.abs.clone()),
            skip_audit,
        );
        let graph = crate::exact_graph::build_selected(p, &mut model, Some(&coarse), audit);
        let index = Index::selected(p, &model, Some(&coarse), audit);
        let root = roots(q, p, &model, &index, audit, &mut counters);
        let mut clauses = graph.reachable(&root);
        clauses.extend((0..p.clauses.len()).filter(|i| p.patient(*i)));
        let docs = clauses
            .iter()
            .map(|i| p.clauses[*i].document)
            .collect::<BTreeSet<_>>();
        clauses.extend(
            (0..p.clauses.len())
                .filter(|i| docs.contains(&p.clauses[*i].document) && p.metadata(*i)),
        );
        Slice {
            clauses,
            coarse_clauses: coarse.len(),
            admitted: self.admitted() && graph.admitted(),
        }
    }
    pub fn witnesses(&self, p: &Program) -> Result<String, String> {
        let mut text = crate::witness::clauses(p, &self.model, &self.graph)?;
        let growth = self.refined.as_ref().map_or_else(
            || self.growth.render_witnesses(p, &self.model),
            |r| r.render_witnesses(p, &self.model),
        );
        text.push_str(growth.split_once('\n').map_or("", |(_, rows)| rows));
        Ok(text)
    }
    pub fn dump(&self, p: &Program, out: &Path) -> Result<(), String> {
        let nodes = (0..self.growth.nodes.len())
            .map(|n| self.growth.label(p, &self.model, n))
            .collect();
        let mut edges = Vec::new();
        for from in 0..self.growth.nodes.len() {
            for edge in self.growth.graph.row(from) {
                edges.push((
                    self.growth.label(p, &self.model, from),
                    self.growth.label(p, &self.model, edge.to),
                ));
            }
        }
        flow(out, "base", nodes, edges)?;
        let outcome = if let Some(r) = &self.refined {
            flow(
                out,
                "final",
                r.canonical_nodes(p, &self.model),
                r.canonical_edges(p, &self.model),
            )?;
            format!(
                "k1\t{}\t{}\t{}\t{}\t{}\t{}\n",
                r.admitted(),
                r.full_refinement,
                r.keys.len(),
                r.generated_variants,
                r.refined_templates,
                r.projection_checks
            )
        } else {
            let nodes = (0..self.growth.nodes.len())
                .map(|n| self.growth.label(p, &self.model, n))
                .collect();
            let edges = (0..self.growth.nodes.len())
                .flat_map(|from| {
                    self.growth.graph.row(from).into_iter().map(move |edge| {
                        (
                            self.growth.label(p, &self.model, from),
                            self.growth.label(p, &self.model, edge.to),
                        )
                    })
                })
                .collect();
            flow(out, "final", nodes, edges)?;
            format!("base\t{}\tfalse\t0\t0\t0\t0\n", self.growth.admitted())
        };
        std::fs::write(out.join("flow-outcome.tsv"), outcome_header() + &outcome)
            .map_err(|e| e.to_string())
    }
}
pub struct Slice {
    pub clauses: BTreeSet<usize>,
    pub coarse_clauses: usize,
    pub admitted: bool,
}
struct RootContext<'a> {
    p: &'a Program,
    m: &'a Model,
    index: &'a Index,
    audit: bool,
}
impl RootContext<'_> {
    fn targets(
        &self,
        pattern: &Pattern,
        state: Option<&State>,
        goal: Id,
        counters: &mut Counters,
        mut accept: impl FnMut(usize, State),
    ) {
        let Some(state) = state else {
            return;
        };
        for to in self
            .index
            .candidates(self.p, self.m, goal, state, self.audit)
        {
            if let Some(found) = self.m.forced(self.p, pattern, goal, to, state, counters) {
                accept(to, found);
            }
        }
    }
}
pub fn roots(
    q: &Question,
    p: &Program,
    m: &Model,
    index: &Index,
    audit: bool,
    counters: &mut Counters,
) -> BTreeSet<usize> {
    let arena = &p.arena;
    let context = RootContext { p, m, index, audit };
    let mut roots = BTreeSet::new();
    match q {
        Question::Goals { goals, vars } => {
            let positive = Pattern::body(arena, &m.abs, goals, *vars, None);
            for goal in goals {
                match goal {
                    Goal::Positive(t) => {
                        let state = m.state(arena, &positive, counters);
                        context.targets(&positive, state.as_ref(), *t, counters, |to, _| {
                            roots.insert(to);
                        });
                    }
                    Goal::Not(inner) => {
                        let pattern = Pattern::body(arena, &m.abs, goals, *vars, Some(inner));
                        for goal in inner {
                            if let Goal::Positive(t) = goal {
                                let state = m.state(arena, &pattern, counters);
                                context.targets(&pattern, state.as_ref(), *t, counters, |to, _| {
                                    roots.insert(to);
                                });
                            }
                        }
                    }
                    Goal::Builtin(_) => {}
                }
            }
        }
        Question::All { operators, payload } => {
            let mut contexts = EMPTY;
            for op in operators {
                let pattern = Pattern::new(arena, &m.abs, vec![*op], 8);
                let state = m.state(arena, &pattern, counters);
                context.targets(&pattern, state.as_ref(), *op, counters, |to, state| {
                    roots.insert(to);
                    contexts = m.sets.union(contexts, state.domain(Expr::Var(0), &m.sets));
                });
            }
            for goal in payload {
                let pattern = Pattern::new(arena, &m.abs, vec![*goal], 8);
                let state = m.bound_state(arena, &pattern, Expr::Var(0), contexts, counters);
                context.targets(&pattern, state.as_ref(), *goal, counters, |to, _| {
                    roots.insert(to);
                });
            }
        }
    }
    roots
}
fn flow(
    out: &Path,
    name: &str,
    mut nodes: Vec<String>,
    mut edges: Vec<(String, String)>,
) -> Result<(), String> {
    nodes.sort_unstable();
    // Semantic labels are injective; preserving duplicates catches a bad identity map.
    assert!(
        !nodes.windows(2).any(|w| w[0] == w[1]),
        "duplicate {name} semantic node"
    );
    edges.sort_unstable();
    assert!(
        !edges.windows(2).any(|w| w[0] == w[1]),
        "duplicate {name} semantic edge"
    );
    std::fs::write(
        out.join(format!("flow-{name}-nodes.tsv")),
        nodes.into_iter().map(|s| s + "\n").collect::<String>(),
    )
    .map_err(|e| e.to_string())?;
    std::fs::write(
        out.join(format!("flow-{name}-edges.tsv")),
        edges
            .into_iter()
            .map(|(a, b)| format!("{a}\t{b}\n"))
            .collect::<String>(),
    )
    .map_err(|e| e.to_string())
}
fn outcome_header() -> String {
    "mode\tadmitted\tfull_refinement\ttypes\tvariants\trefined_templates\tprojection_checks\n"
        .into()
}
pub fn oracle_dump(p: &Program, a: &crate::Analysis, out: &Path) -> Result<(), String> {
    let nodes = (0..a.growth.nodes.len())
        .map(|n| a.growth.label(p, &a.model, n))
        .collect();
    let edges = a
        .growth
        .graph
        .edges
        .iter()
        .enumerate()
        .flat_map(|(from, row)| {
            row.iter().map(move |edge| {
                (
                    a.growth.label(p, &a.model, from),
                    a.growth.label(p, &a.model, edge.to),
                )
            })
        })
        .collect();
    flow(out, "base", nodes, edges)?;
    let outcome = if let Some(r) = &a.refined {
        flow(
            out,
            "final",
            r.canonical_nodes(p, &a.model),
            r.canonical_edges(p, &a.model),
        )?;
        format!(
            "k1\t{}\t{}\t{}\t{}\t{}\t{}\n",
            r.admitted(),
            r.full_refinement,
            r.keys.len(),
            r.generated_variants,
            r.refined_templates,
            r.projection_checks
        )
    } else {
        let nodes = (0..a.growth.nodes.len())
            .map(|n| a.growth.label(p, &a.model, n))
            .collect();
        let edges = a
            .growth
            .graph
            .edges
            .iter()
            .enumerate()
            .flat_map(|(from, row)| {
                row.iter().map(move |edge| {
                    (
                        a.growth.label(p, &a.model, from),
                        a.growth.label(p, &a.model, edge.to),
                    )
                })
            })
            .collect();
        flow(out, "final", nodes, edges)?;
        format!("base\t{}\tfalse\t0\t0\t0\t0\n", a.growth.admitted())
    };
    std::fs::write(out.join("flow-outcome.tsv"), outcome_header() + &outcome)
        .map_err(|e| e.to_string())
}
pub fn slice_dump(
    p: &Program,
    clauses: &BTreeSet<usize>,
    out: &Path,
    name: &str,
) -> Result<(), String> {
    let mut rows = clauses
        .iter()
        .map(|i| {
            let c = &p.clauses[*i];
            format!(
                "{}\t{}\t{}\n",
                p.documents[c.document - 1].path.display(),
                c.ordinal,
                p.patient(*i)
            )
        })
        .collect::<Vec<_>>();
    rows.sort_unstable();
    std::fs::write(out.join(format!("slice-{name}.tsv")), rows.concat()).map_err(|e| e.to_string())
}

#[cfg(test)]
#[path = "complete_tests.rs"]
mod tests;
