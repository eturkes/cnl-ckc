use super::growth::{self, Evidence};
use super::matching::{self, Expr, Heads, forced, solve, state};
use super::namespaces::Namespaces;
use super::refinement::Registry;
use super::{Budget, Csr, EMPTY, May, Pattern, Program, Result, SetId, State, bad};
use sb1_checker::program::Goal;
use sb1_checker::query::Query;

pub struct Refined {
    pub program: Program,
    pub model: May,
    pub flow: Evidence,
    pub registry: Registry,
}
pub struct Certificate {
    pub version: u32,
    pub model: May,
    pub clause_ranks: Option<Vec<u64>>,
    pub flow: Evidence,
    pub refined: Option<Refined>,
}
pub struct Checked {
    pub program: Program,
    pub model: May,
    pub graph: Csr,
    heads: Heads,
    namespaces: Namespaces,
}

pub fn check(p: Program, cert: Certificate, budget: &mut Budget) -> Result<Checked> {
    check_with_observer(p, cert, budget, |_| Ok(()))
}
pub struct Diagnostics {
    pub flow: Evidence,
    pub base_graph: Csr,
    pub refined: Option<(Refined, Csr)>,
}
pub fn check_diagnostic(
    p: Program,
    cert: Certificate,
    budget: &mut Budget,
) -> Result<(Checked, Diagnostics)> {
    check_inner(p, cert, budget, |_| Ok(()), true)
        .map(|(checked, details)| (checked, details.expect("diagnostic capture")))
}
pub fn check_with_observer(
    p: Program,
    cert: Certificate,
    budget: &mut Budget,
    observe: impl FnMut(&'static str) -> Result<()>,
) -> Result<Checked> {
    check_inner(p, cert, budget, observe, false).map(|(checked, _)| checked)
}
fn check_inner(
    p: Program,
    mut cert: Certificate,
    budget: &mut Budget,
    mut observe: impl FnMut(&'static str) -> Result<()>,
    capture: bool,
) -> Result<(Checked, Option<Diagnostics>)> {
    observe("ta-source-admission")?;
    if cert.version != 1 || !cert.model.generic {
        return Err(bad("hint version/patient-top mode"));
    }
    if p.patient.len() != p.clauses.len()
        || p.clauses
            .iter()
            .any(|c| c.document == 0 || c.document > p.documents.len())
    {
        return Err(bad("source clause inventory"));
    }
    sb1_checker::program::admit(&p.arena, &p.clauses).map_err(super::Failure::Admission)?;
    let namespaces = Namespaces::build(&p, &cert.model.abs);
    namespaces
        .check_initial(&p)
        .map_err(super::Failure::Admission)?;
    observe("ta-feature-seeds-closure")?;
    matching::check_envelope(&p, &cert.model, budget)?;
    observe("ta-signed-edges-ranks-csr")?;
    let graph = super::signed_graph(&p, &cert.model, cert.clause_ranks.as_deref(), budget)?;
    observe("ta-source-hint-closure")?;
    cert.flow
        .sources
        .complete_scalars(&p, &cert.model, &cert.flow.types, budget)?;
    cert.flow
        .sources
        .check(&p, &cert.model, &cert.flow.types, budget)?;
    let mut base_graph = None;
    let mut refined_graph = None;
    if let Some(mut refined) = cert.refined {
        if capture {
            base_graph = Some(growth::enumerate(
                &p,
                &cert.model,
                &cert.flow.types,
                &cert.flow.sources,
                budget,
            )?);
        }
        if !refined.model.generic
            || refined.program.patient.len() != refined.program.clauses.len()
            || refined.program.patient.iter().any(|p| *p)
            || refined
                .program
                .clauses
                .iter()
                .any(|c| c.document == 0 || c.document > refined.program.documents.len())
        {
            return Err(bad("refined program patient-top inventory"));
        }
        observe("ta-k1-registry-variants")?;
        refined
            .registry
            .check_variants(&p, &cert.model, &refined.program, &refined.model)?;
        refined.registry.check_complete(
            &p,
            &cert.model,
            &cert.flow.types,
            &cert.flow.sources,
            budget,
        )?;
        observe("ta-k1-feature-closure")?;
        matching::check_envelope(&refined.program, &refined.model, budget)?;
        observe("ta-k1-source-hint-closure")?;
        refined.flow.sources.complete_scalars(
            &refined.program,
            &refined.model,
            &refined.flow.types,
            budget,
        )?;
        refined.flow.sources.check(
            &refined.program,
            &refined.model,
            &refined.flow.types,
            budget,
        )?;
        observe("ta-mixed-growth-ranks")?;
        refined.registry.check_compositional(
            &refined.program,
            &refined.model,
            &refined.flow.types,
            &refined.flow.sources,
        )?;
        let mixed_graph = growth::graph(
            &refined.program,
            &refined.model,
            &refined.flow.types,
            &refined.flow.sources,
            refined.flow.ranks.as_deref(),
            budget,
        )
        .map_err(|failure| match failure {
            super::Failure::Cycle {
                kind, edges, sites, ..
            } => {
                let remap = (|| {
                    let sites = sites
                        .into_iter()
                        .map(|(clause, variable)| {
                            refined.registry.variants[clause]
                                .original
                                .map(|c| (c, variable))
                                .ok_or_else(|| bad("growth witness names a registry declaration"))
                        })
                        .collect::<Result<Vec<_>>>()?;
                    let labels = edges
                        .iter()
                        .map(|(from, to, _)| {
                            Ok((
                                refined.registry.node_label(
                                    &p,
                                    &cert.model,
                                    &refined.flow.types,
                                    *from,
                                )?,
                                refined.registry.node_label(
                                    &p,
                                    &cert.model,
                                    &refined.flow.types,
                                    *to,
                                )?,
                            ))
                        })
                        .collect::<Result<Vec<_>>>()?;
                    Ok((sites, labels))
                })();
                match remap {
                    Ok((sites, labels)) => super::Failure::Cycle {
                        kind,
                        edges,
                        sites,
                        labels,
                    },
                    Err(failure) => failure,
                }
            }
            other => other,
        })?;
        if capture {
            refined_graph = Some((refined, mixed_graph));
        }
    } else {
        observe("ta-base-growth-ranks")?;
        let base = growth::graph(
            &p,
            &cert.model,
            &cert.flow.types,
            &cert.flow.sources,
            cert.flow.ranks.as_deref(),
            budget,
        )?;
        if capture {
            base_graph = Some(base);
        }
    }
    observe("ta-retained-head-index")?;
    cert.model.reset_scratch();
    let heads = Heads::build(&p, &cert.model)?;
    observe("ta-core-complete")?;
    let details = if capture {
        Some(Diagnostics {
            flow: cert.flow,
            base_graph: base_graph.expect("captured growth graph"),
            refined: refined_graph,
        })
    } else {
        None
    };
    Ok((
        Checked {
            program: p,
            model: cert.model,
            graph,
            heads,
            namespaces,
        },
        details,
    ))
}
impl Checked {
    pub fn check_patient(
        &self,
        patient: &[super::extension::Original<'_>],
    ) -> std::result::Result<(), String> {
        self.namespaces.check(&self.program, patient)
    }
    fn query_top_term(
        p: &Program,
        m: &mut May,
        ns: &Namespaces,
        term: usize,
        changed: &mut Vec<(usize, u32)>,
    ) {
        if p.arena.named(term, "$guideline_id", 5).is_some() {
            let value = m.abs.terms[term];
            if let Some(origin) = m.abs.origin(value)
                && !ns.origins.get(origin).copied().unwrap_or(false)
            {
                changed.push((term, value));
                m.abs.terms[term] = super::Abs::PATIENT;
            }
        }
        if let Some((_, args)) = p.arena.call(term) {
            for term in args {
                Self::query_top_term(p, m, ns, *term, changed);
            }
        }
    }
    fn query_top_goals(
        p: &Program,
        m: &mut May,
        ns: &Namespaces,
        goals: &[Goal],
        changed: &mut Vec<(usize, u32)>,
    ) {
        for goal in goals {
            match goal {
                Goal::Positive(t) | Goal::Builtin(t) => Self::query_top_term(p, m, ns, *t, changed),
                Goal::Not(inner) => Self::query_top_goals(p, m, ns, inner, changed),
            }
        }
    }
    fn add_roots(
        &self,
        pattern: &Pattern,
        state: &State,
        goal: usize,
        roots: &mut Vec<usize>,
        contexts: &mut SetId,
        budget: &mut Budget,
    ) -> Result<()> {
        for to in self
            .heads
            .candidates(&self.program, &self.model, goal, state)
        {
            if let Some(found) =
                forced(&self.program, &self.model, pattern, goal, to, state, budget)?
            {
                #[cfg(test)]
                if crate::faults::on("query-root-omitted") {
                    continue;
                }
                roots.push(to);
                if let Some(a) = self.program.arena.named(goal, "guideline_operator", 3) {
                    let context = Pattern::expr(&self.program.arena, &self.model.abs, a[1]);
                    *contexts = self
                        .model
                        .sets
                        .union(*contexts, found.domain(context, &self.model));
                }
            }
        }
        Ok(())
    }
    pub fn select(&mut self, query: &[u8], budget: &mut Budget) -> Result<Vec<usize>> {
        self.select_inner(query, budget, None)
    }
    pub fn select_diagnostic(
        &mut self,
        query: &[u8],
        budget: &mut Budget,
    ) -> Result<(Vec<usize>, Vec<usize>)> {
        let mut roots = Vec::new();
        self.select_inner(query, budget, Some(&mut roots))
            .map(|selected| (selected, roots))
    }
    fn select_inner(
        &mut self,
        query: &[u8],
        budget: &mut Budget,
        mut captured_roots: Option<&mut Vec<usize>>,
    ) -> Result<Vec<usize>> {
        let checkpoint = self.program.arena.checkpoint();
        let abstraction = self.model.abs.checkpoint();
        let mut changed = Vec::new();
        let result = (|| {
            let question = Query::parse(&mut self.program.arena, Some(query))
                .map_err(|e| super::Failure::Admission(e.message))?;
            self.model.abs.extend(&self.program.arena)?;
            if matches!(question, Query::All | Query::None) {
                let selected = (0..self.program.clauses.len())
                    .filter(|i| !self.program.patient[*i])
                    .collect::<Vec<_>>();
                if let Some(roots) = &mut captured_roots {
                    roots.extend_from_slice(&selected);
                }
                return Ok(selected);
            }
            let Query::Projected { goals, vars, .. } = question else {
                unreachable!()
            };
            Self::query_top_goals(
                &self.program,
                &mut self.model,
                &self.namespaces,
                &goals,
                &mut changed,
            );
            let pattern = Pattern::body(&self.program.arena, &self.model.abs, &goals, vars, None);
            let base = state(&self.program.arena, &self.model, &pattern, budget)?;
            let mut roots = Vec::new();
            let mut contexts = EMPTY;
            for goal in &goals {
                match goal {
                    Goal::Positive(t) => {
                        if let Some(s) = &base {
                            self.add_roots(&pattern, s, *t, &mut roots, &mut contexts, budget)?;
                        }
                    }
                    Goal::Not(inner) => {
                        #[cfg(test)]
                        if crate::faults::on("query-naf-omitted") {
                            continue;
                        }
                        let pattern = Pattern::body(
                            &self.program.arena,
                            &self.model.abs,
                            &goals,
                            vars,
                            Some(inner),
                        );
                        if let Some(s) = state(&self.program.arena, &self.model, &pattern, budget)?
                        {
                            for goal in inner {
                                if let Goal::Positive(t) = goal {
                                    self.add_roots(
                                        &pattern,
                                        &s,
                                        *t,
                                        &mut roots,
                                        &mut contexts,
                                        budget,
                                    )?;
                                }
                            }
                        }
                    }
                    Goal::Builtin(_) => {}
                }
            }
            let include_payload = true;
            #[cfg(test)]
            let include_payload = include_payload && !crate::faults::on("query-payload-omitted");
            if contexts != EMPTY && include_payload {
                let mut payload = Vec::new();
                for (name, arity) in sb1_checker::program::PREDICATES {
                    if matches!(
                        name,
                        "guideline_schema_version"
                            | "guideline_document"
                            | "guideline_supersession"
                            | "guideline_exclusion"
                            | "guideline_evaluation_date"
                    ) {
                        continue;
                    }
                    let args = (0..arity)
                        .map(|v| self.program.arena.put(sb1_checker::term::Node::Var(v)))
                        .collect();
                    let goal = self.program.arena.compound(name, args);
                    payload.push((goal, arity));
                }
                self.model.abs.extend(&self.program.arena)?;
                for (goal, vars) in payload {
                    let pattern =
                        Pattern::new(&self.program.arena, &self.model.abs, vec![goal], vars);
                    let mut s = State::new(vars);
                    if s.constrain(Expr::Var(0), contexts, &self.model)
                        && let Some(s) =
                            solve(&self.program.arena, &self.model, &pattern, s, budget)?
                    {
                        let mut ignored = EMPTY;
                        self.add_roots(&pattern, &s, goal, &mut roots, &mut ignored, budget)?;
                    }
                }
            }
            roots.sort_unstable();
            roots.dedup();
            if let Some(captured) = &mut captured_roots {
                captured.extend_from_slice(&roots);
            }
            let mut marked = self.graph.reach(&roots)?;
            let mut documents = vec![false; self.program.documents.len()];
            for (i, selected) in marked.iter().enumerate() {
                if *selected {
                    documents[self.program.clauses[i].document - 1] = true;
                }
            }
            for (i, c) in self.program.clauses.iter().enumerate() {
                if documents[c.document - 1] && self.program.metadata(i) {
                    marked[i] = true;
                }
            }
            Ok(marked
                .iter()
                .enumerate()
                .filter_map(|(i, selected)| (*selected && !self.program.patient[i]).then_some(i))
                .collect())
        })();
        for (term, value) in changed {
            self.model.abs.terms[term] = value;
        }
        self.model.reset_scratch();
        // Query interning is process-local scratch. Abs appended values are
        // rolled back with the arena before another query reuses those ids.
        self.model.abs.rewind(abstraction);
        self.program.arena.rewind(checkpoint);
        result
    }
}
