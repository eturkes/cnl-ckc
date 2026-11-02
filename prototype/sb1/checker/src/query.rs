use crate::digest::sha256;
use crate::engine::{Evaluation, Model, q_all};
use crate::program::{Goal, goals_of};
use crate::term::{Arena, Id, Parser};
use crate::{Counters, Failure};

pub enum Query {
    None,
    All,
    Projected {
        qid: Id,
        digest: String,
        goals: Vec<Goal>,
        projection: Vec<Id>,
        vars: usize,
    },
}

impl Query {
    pub fn parse(arena: &mut Arena, bytes: Option<&[u8]>) -> Result<Self, Failure> {
        let Some(bytes) = bytes else {
            return Ok(Self::None);
        };
        if bytes == b"sb1-q-all 0\n" {
            return Ok(Self::All);
        }
        if !crate::canonical(bytes) {
            return Err(Failure::parse("noncanonical query"));
        }
        let read = ckc_kernel::contract::v1_read(bytes);
        let text =
            std::str::from_utf8(&read).map_err(|e| Failure::parse(format!("query UTF-8 {e}")))?;
        let lines = text
            .lines()
            .filter(|l| !l.is_empty() && !l.starts_with('%'))
            .collect::<Vec<_>>();
        if lines.len() != 2 {
            return Err(Failure::parse("query term count"));
        }
        let mut parser = Parser::new(lines[0]);
        let header = parser.term(arena).map_err(Failure::parse)?;
        parser.finish().map_err(Failure::parse)?;
        let Some(args) = arena.named(header, "$guideline_query", 5) else {
            return Err(Failure::parse("query record"));
        };
        if arena.atom_name(args[0]) != Some("v2") || arena.atom_name(args[1]).is_none() {
            return Err(Failure::parse("query identity"));
        }
        let qid = args[1];
        let mut parser = Parser::new(lines[1]);
        let projection = parser.term(arena).map_err(Failure::parse)?;
        parser.finish().map_err(Failure::parse)?;
        let Some(args) = arena.named(projection, "$guideline_query_projection", 2) else {
            return Err(Failure::parse("query projection"));
        };
        let Some(goal) = arena.named(args[0], "goal", 1) else {
            return Err(Failure::parse("query goal"));
        };
        let goals = goals_of(arena, goal[0]).map_err(Failure::parse)?;
        let Some(answers) = arena.named(args[1], "answers", 1) else {
            return Err(Failure::parse("query answers"));
        };
        let Some(answers) = arena.list(answers[0]) else {
            return Err(Failure::parse("query answers list"));
        };
        let projection = answers
            .into_iter()
            .map(|a| arena.named(a, "answer", 2).map(|a| a[0]))
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| Failure::parse("query answer shape"))?;
        crate::program::admit_query(arena, &goals, &projection).map_err(Failure::parse)?;
        Ok(Self::Projected {
            qid,
            digest: sha256(bytes),
            goals,
            projection,
            vars: parser.vars.len(),
        })
    }

    pub fn goals(&self) -> &[Goal] {
        match self {
            Self::Projected { goals, .. } => goals,
            _ => &[],
        }
    }

    pub fn answers(
        &self,
        arena: &mut Arena,
        model: &Model,
        counters: &mut Counters,
    ) -> Result<String, Failure> {
        match self {
            Self::None => Ok(format!("accepted {} facts\n", model.facts.len())),
            Self::All => Ok(q_all(arena, model, counters)),
            Self::Projected {
                qid,
                digest,
                goals,
                projection,
                vars,
            } => {
                let mut rows = Vec::new();
                let mut evaluation = Evaluation {
                    arena,
                    model,
                    counters,
                };
                evaluation.walk(goals, 0, &vec![None; *vars], &mut |arena, env, _| {
                    let Some(values) = projection
                        .iter()
                        .map(|a| arena.instantiate(*a, env))
                        .collect::<Option<Vec<_>>>()
                    else {
                        return Err(Failure::parse("query projects an unbound variable"));
                    };
                    rows.push(values);
                    Ok(projection.is_empty())
                })?;
                let result = if projection.is_empty() {
                    arena.atom(if rows.is_empty() { "no" } else { "yes" })
                } else {
                    rows.sort_by(|a, b| {
                        counters.runtime("comparisons", 1);
                        a.iter()
                            .zip(b)
                            .map(|(a, b)| arena.compare(*a, *b))
                            .find(|o| *o != std::cmp::Ordering::Equal)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    });
                    rows.dedup();
                    counters.logical("answer_rows", rows.len() as u64);
                    let mut solutions = Vec::with_capacity(rows.len());
                    for row in rows {
                        let row = arena.make_list(&row);
                        solutions.push(arena.compound("sol", vec![row]));
                    }
                    let list = arena.make_list(&solutions);
                    arena.compound("solutions", vec![list])
                };
                let result = arena.compound("result", vec![result]);
                let hash = arena.atom(digest);
                let hash = arena.compound("query_sha256", vec![hash]);
                let v1 = arena.atom("v1");
                let record = arena.compound("$guideline_answers", vec![v1, *qid, hash, result]);
                let name = arena.atom_name(*qid).expect("query id");
                Ok(format!(
                    "% {name} answered against the loaded composition by ckc queries; do not edit.\n{}.\n",
                    arena.render(record)
                ))
            }
        }
    }
}
