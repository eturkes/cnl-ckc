pub mod domain;
pub mod features;
pub mod graph;
pub mod growth;
pub mod k1;
pub mod memory;
pub use features as may;
pub mod source;

use graph::{Graph, Question};
use may::{Counters, Model};
use source::Program;
use std::collections::BTreeSet;

pub struct Analysis {
    pub model: Model,
    pub graph: Graph,
    pub growth: growth::Growth,
    pub refined: Option<k1::Refined>,
}
impl Analysis {
    pub fn load(program: &Program) -> Result<Self, String> {
        program.validate_patient_boundary()?;
        let model = Model::build(program, None, true);
        let graph = Graph::build(program, &model, None);
        let growth = growth::Growth::build(program, &model, None);
        Self::from_parts(program, model, graph, growth)
    }
    pub fn from_parts(
        program: &Program,
        model: Model,
        graph: Graph,
        growth: growth::Growth,
    ) -> Result<Self, String> {
        let refined = if graph.admitted() && !growth.admitted() {
            Some(k1::Refined::build(program, &model, &growth)?)
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
                .map_or_else(|| self.growth.admitted(), k1::Refined::admitted)
    }
    pub fn witnesses(&self, program: &Program) -> String {
        let mut text = self.graph.render_witnesses(program, &self.model);
        let growth = self.refined.as_ref().map_or_else(
            || self.growth.render_witnesses(program, &self.model),
            |r| r.render_witnesses(program, &self.model),
        );
        text.push_str(growth.split_once('\n').map_or("", |(_, rows)| rows));
        text
    }
    pub fn slice(&self, program: &Program, query: &Question) -> Slice {
        let mut query_counters = Counters::default();
        let roots = query.roots(program, &self.model, &mut query_counters);
        let mut candidate = self.graph.reachable(&roots);
        // The patient is a separate per-call input. Its own facts supply features;
        // none of the load-time all-feature placeholders enter this model.
        for (i, _) in program.clauses.iter().enumerate() {
            if program.patient(i) {
                candidate.insert(i);
            }
        }
        let coarse_clauses = candidate.len();
        let model = Model::build_mode(
            program,
            Some(&candidate),
            false,
            Some(self.model.abs.clone()),
            self.model.mode,
        );
        let graph = Graph::build(program, &model, Some(&candidate));
        let growth = growth::Growth::build(program, &model, Some(&candidate));
        let roots = query.roots(program, &model, &mut query_counters);
        let mut clauses = graph.reachable(&roots);
        for (i, _) in program.clauses.iter().enumerate() {
            if program.patient(i) {
                clauses.insert(i);
            }
        }
        let documents = clauses
            .iter()
            .map(|i| program.clauses[*i].document)
            .collect::<BTreeSet<_>>();
        for (i, c) in program.clauses.iter().enumerate() {
            if documents.contains(&c.document) && program.metadata(i) {
                clauses.insert(i);
            }
        }
        Slice {
            clauses,
            model,
            graph,
            growth,
            query_counters,
            coarse_clauses,
            inherited_growth: self.admitted(),
        }
    }
}
pub struct Slice {
    pub clauses: BTreeSet<usize>,
    pub model: Model,
    pub graph: Graph,
    pub growth: growth::Growth,
    pub query_counters: Counters,
    pub coarse_clauses: usize,
    pub inherited_growth: bool,
}

impl Slice {
    pub fn admitted(&self) -> bool {
        self.graph.admitted() && (self.inherited_growth || self.growth.admitted())
    }
    pub fn witnesses(&self, program: &Program) -> String {
        let mut text = self.graph.render_witnesses(program, &self.model);
        // Removing rules and replacing patient-top by facts cannot create growth.
        if !self.inherited_growth {
            text.push_str(
                self.growth
                    .render_witnesses(program, &self.model)
                    .split_once('\n')
                    .map_or("", |(_, rows)| rows),
            );
        }
        text
    }
}

#[cfg(test)]
mod tests;
