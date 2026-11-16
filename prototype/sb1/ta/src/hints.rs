//! Untrusted B1 proposals. The checking crate decodes these into its own ids.
use crate::complete::Analysis;
use crate::domain::{Domain, Value};
use crate::exact::Model;
use crate::exact_growth::{Growth, Origin};
use crate::exact_k1::{Key, Tag};
use crate::exact_scc::Compressed;
use crate::features::{Feature, Label};
use crate::source::Program;
use sb1_checker::program::Goal;
use sb1_checker::term::{Id, Node, Symbol};
use std::collections::{BTreeMap, VecDeque};
use std::io::Write;
use std::rc::Rc;

pub const VERSION: u32 = 1;

pub struct MayHint {
    pub generic: bool,
    pub origins: Vec<[Id; 4]>,
    // None = All; a finite Any member remains distinct from All.
    pub sets: Vec<Option<Rc<[Value]>>>,
    pub unary: Vec<(Feature, usize)>,
    pub rectangles: Vec<(Label, usize, usize, usize)>,
    pub predicates: Vec<Symbol>,
}
impl MayHint {
    pub fn dump(&self) -> crate::front::Dump {
        let unary = self
            .unary
            .iter()
            .map(|(f, set)| {
                (
                    *f,
                    self.sets[*set]
                        .as_ref()
                        .expect("finite unary")
                        .iter()
                        .copied()
                        .collect(),
                )
            })
            .collect();
        let mut links = BTreeMap::<_, Domain>::new();
        for (label, owner, source, target) in &self.rectangles {
            let target = self.sets[*target]
                .as_ref()
                .map_or(Domain::All, |v| Domain::Set(v.iter().copied().collect()));
            for o in self.sets[*owner].as_ref().expect("finite owner").iter() {
                for s in self.sets[*source].as_ref().expect("finite source").iter() {
                    links
                        .entry((*o, *s, *label))
                        .or_insert_with(Domain::empty)
                        .union(&target);
                }
            }
        }
        (unary, links)
    }
    pub fn payload_bytes(&self) -> usize {
        use std::mem::size_of;
        self.sets
            .iter()
            .flatten()
            .map(|s| s.len() * size_of::<Value>())
            .sum::<usize>()
            + self.sets.capacity() * size_of::<Option<Rc<[Value]>>>()
            + self.unary.capacity() * size_of::<(Feature, usize)>()
            + self.rectangles.capacity() * size_of::<(Label, usize, usize, usize)>()
            + self.origins.capacity() * size_of::<[Id; 4]>()
            + self.predicates.capacity() * size_of::<Symbol>()
    }
}

#[derive(Clone, Debug)]
pub struct SourceHint {
    pub clause: usize,
    pub variable: usize,
    pub sources: Rc<[usize]>,
    // (body goal, argument). None names a non-flat/arithmetic-only binder.
    pub binder: Option<(usize, usize)>,
}
pub struct FlowHint {
    pub nodes: Vec<Origin>,
    pub sources: Vec<SourceHint>,
    pub ranks: Option<Vec<u64>>,
    pub precision_fallbacks: usize,
}
pub struct RefinedHint {
    pub program: Program,
    pub model: MayHint,
    pub flow: FlowHint,
    pub registry: Vec<Key>,
    pub variants: Vec<(usize, Vec<(usize, Tag)>)>,
    pub full_refinement: bool,
}
pub struct Hints {
    pub version: u32,
    pub model: MayHint,
    pub clause_ranks: Option<Vec<u64>>,
    pub flow: FlowHint,
    pub refined: Option<RefinedHint>,
}

// Solver-side rank proposal: condensation DAG, weak ordinary edges, strict
// negative/growing edges. No SCC implementation crosses into the checker.
pub fn ranks(graph: &Compressed, nodes: usize) -> Option<Vec<u64>> {
    if !graph.admitted() {
        return None;
    }
    let n = graph.sizes.len();
    let mut incoming = vec![Vec::<(usize, u64)>::new(); n];
    let mut pending = vec![0_usize; n];
    for from in 0..nodes {
        let a = graph.components[from];
        for edge in graph.row(from) {
            let b = graph.components[edge.to];
            if a != b {
                incoming[b].push((a, u64::from(edge.negative || edge.growth)));
                pending[a] += 1;
            } else if edge.negative || edge.growth {
                return None;
            }
        }
    }
    let mut rank = vec![1_u64; n];
    let mut queue = pending
        .iter()
        .enumerate()
        .filter_map(|(i, n)| (*n == 0).then_some(i))
        .collect::<VecDeque<_>>();
    let mut visited = 0;
    while let Some(to) = queue.pop_front() {
        visited += 1;
        for (from, strict) in &incoming[to] {
            rank[*from] = rank[*from].max(rank[to].checked_add(*strict)?);
            pending[*from] -= 1;
            if pending[*from] == 0 {
                queue.push_back(*from);
            }
        }
    }
    (visited == n).then(|| graph.components.iter().map(|c| rank[*c]).collect())
}
fn flow(p: &Program, growth: Growth) -> FlowHint {
    let ranks = ranks(&growth.graph, growth.nodes.len());
    let mut sources = growth.var_sources.into_iter().map(|((clause, variable), sources)| {
        let binder = p.clauses[clause].body.iter().enumerate().find_map(|(j, goal)| {
            if let Goal::Positive(t) = goal {
                p.arena.call(*t).and_then(|(_, args)| {
                    args.iter().position(|arg| matches!(p.arena.node(*arg), Node::Var(v) if *v == variable)).map(|k| (j, k))
                })
            } else {
                None
            }
        });
        SourceHint { clause, variable, sources, binder }
    }).collect::<Vec<_>>();
    sources.sort_unstable_by_key(|s| (s.clause, s.variable));
    FlowHint {
        nodes: growth.nodes,
        sources,
        ranks,
        precision_fallbacks: growth.precision_fallbacks,
    }
}

pub fn analyze(p: &Program) -> Result<Analysis, String> {
    p.validate_patient_boundary()?;
    let mut model = Model::build(p, None, true, None, false);
    let graph = crate::exact_graph::build_compressed(p, &mut model, false);
    let growth = Growth::build(p, &mut model, false, false);
    Analysis::from_parts(p, model, graph, growth, false, false)
}

pub fn export(p: &Program, mut a: Analysis) -> Result<Hints, String> {
    let clause_ranks = ranks(&a.graph, p.clauses.len());
    let growth = Growth::hint_sources(p, &mut a.model);
    let model = a.model.hint(p);
    let base_flow = flow(p, growth);
    let refined = a.refined.take().map(|mut r| {
        let growth = r.hint_growth();
        let model = r.model.hint(&r.program);
        let flow = flow(&r.program, growth);
        let variants = r.hint_variants();
        RefinedHint {
            program: r.program,
            model,
            flow,
            registry: r.keys,
            variants,
            full_refinement: r.full_refinement,
        }
    });
    // `a` drops the solver graphs, caches, tables + indices here. MayHint keeps
    // only its root immutable vectors; the checker converts then releases them.
    Ok(Hints {
        version: VERSION,
        model,
        clause_ranks,
        flow: base_flow,
        refined,
    })
}

impl Hints {
    pub fn write_canonical(&self, p: &Program, out: &mut impl Write) -> std::io::Result<()> {
        fn may(
            p: &Program,
            hint: &MayHint,
            name: &str,
            out: &mut impl Write,
        ) -> std::io::Result<()> {
            let describe = |v| match v {
                Value::Any => "any".into(),
                Value::Number => "number".into(),
                Value::Patient => "patient".into(),
                Value::Exact(t) => format!("exact({})", p.arena.render(t)),
                Value::Origin(o) => {
                    let k = hint.origins[o];
                    format!(
                        "origin({},{},{},{})",
                        p.arena.render(k[0]),
                        p.arena.render(k[1]),
                        p.arena.render(k[2]),
                        p.arena.render(k[3])
                    )
                }
            };
            let domain = |set: usize| {
                hint.sets[set].as_ref().map_or("all".into(), |v| {
                    let mut names = v.iter().map(|v| describe(*v)).collect::<Vec<_>>();
                    names.sort_unstable();
                    names.join("|")
                })
            };
            let mut rows = Vec::new();
            for o in 0..hint.origins.len() {
                rows.push(format!(
                    "{name}\torigin-id\t{o}\t{}",
                    describe(Value::Origin(o))
                ));
            }
            for (feature, set) in &hint.unary {
                let f = match feature {
                    Feature::Noun(v) => format!("noun({})", describe(*v)),
                    Feature::Lemma(v) => format!("lemma({})", describe(*v)),
                    Feature::Operator(v) => format!("operator({})", describe(*v)),
                    Feature::Property(v) => format!("property({})", describe(*v)),
                    Feature::Payload(pred, v) => {
                        format!("payload({},{})", p.arena.name(*pred), describe(*v))
                    }
                    Feature::Tag(v) => format!("tag({})", describe(*v)),
                };
                rows.push(format!("{name}\tunary\t{f}\t{}", domain(*set)));
            }
            for (label, owner, source, target) in &hint.rectangles {
                rows.push(format!(
                    "{name}\trect\t{}\t{}\t{}\t{}\t{}",
                    p.arena.name(label.0),
                    describe(label.1),
                    domain(*owner),
                    domain(*source),
                    domain(*target)
                ));
            }
            rows.sort_unstable();
            for row in rows {
                writeln!(out, "{row}")?;
            }
            Ok(())
        }
        fn flow(
            p: &Program,
            hint: &FlowHint,
            name: &str,
            out: &mut impl Write,
        ) -> std::io::Result<()> {
            let label = |node: usize| match hint.nodes[node] {
                Origin::Skolem(o) => format!("origin#{o}"),
                Origin::Arithmetic { clause, goal } => {
                    format!("arithmetic({},{})", p.label(clause), goal)
                }
            };
            for source in &hint.sources {
                let mut values = source.sources.iter().map(|v| label(*v)).collect::<Vec<_>>();
                values.sort_unstable();
                writeln!(
                    out,
                    "{name}\tsource\t{}\t{}\t{:?}\t{}",
                    p.label(source.clause),
                    source.variable,
                    source.binder,
                    values.join("|")
                )?;
            }
            if let Some(rank) = &hint.ranks {
                for (i, r) in rank.iter().enumerate() {
                    writeln!(out, "{name}\trank\t{}\t{r}", label(i))?;
                }
            }
            Ok(())
        }
        writeln!(out, "sb1-ta-hint\t{}", self.version)?;
        may(p, &self.model, "base", out)?;
        if let Some(rank) = &self.clause_ranks {
            for (i, r) in rank.iter().enumerate() {
                writeln!(out, "clause-rank\t{}\t{r}", p.label(i))?;
            }
        }
        flow(p, &self.flow, "base", out)?;
        if let Some(r) = &self.refined {
            may(&r.program, &r.model, "k1", out)?;
            flow(&r.program, &r.flow, "k1", out)?;
            for (i, key) in r.registry.iter().enumerate() {
                writeln!(out, "registry\t{i}\t{key:?}")?;
            }
            for (i, (original, tags)) in r.variants.iter().enumerate() {
                writeln!(out, "variant\t{i}\t{original}\t{tags:?}")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn snapshot_keeps_all_unary_and_rectangle_rows() {
        let mut p = Program::new();
        p.add("fixture".into(), "guideline_entity(actual,'$guideline_id'(product,d,1,ref(1),[]),seed,countable).\nguideline_entity(actual,X,next,countable) :- guideline_entity(actual,X,seed,countable).\nguideline_arg(actual,'$guideline_id'(product,d,1,ref(1),[]),1,X) :- guideline_entity(actual,X,next,countable).\n".into(), false).unwrap();
        let a = analyze(&p).unwrap();
        let expected = a.model.dump();
        let hint = export(&p, a).unwrap();
        assert_eq!(hint.version, VERSION);
        assert_eq!(hint.model.dump(), expected);
        assert!(hint.clause_ranks.as_ref().unwrap().iter().all(|r| *r >= 1));
        assert!(
            hint.flow
                .sources
                .iter()
                .any(|s| s.clause == 1 && s.variable == 0 && s.binder == Some((0, 1)))
        );
        let mut first = Vec::new();
        hint.write_canonical(&p, &mut first).unwrap();
        let hint = export(&p, analyze(&p).unwrap()).unwrap();
        let mut second = Vec::new();
        hint.write_canonical(&p, &mut second).unwrap();
        assert_eq!(first, second);
    }
    #[test]
    fn signed_rank_proposal_uses_condensation_not_positive_cycle_length() {
        use crate::graph::Edge;
        let e = |to, negative| Edge {
            to,
            negative,
            growth: false,
        };
        let graph = Compressed::from_edges(
            vec![vec![e(1, false)], vec![e(0, false), e(2, true)], vec![]],
            Default::default(),
        );
        assert_eq!(ranks(&graph, 3), Some(vec![2, 2, 1]));
        let graph = Compressed::from_edges(vec![vec![e(0, true)]], Default::default());
        assert_eq!(ranks(&graph, 1), None);
    }
}
