//! Candidate-only reconstruction; the oracle keeps its independent solver/printer.
use crate::domain::Counters;
use crate::exact::Model;
use crate::exact_heads::Index;
use crate::exact_scc::Compressed;
use crate::features::Pattern;
use crate::source::Program;
use sb1_checker::program::Goal;

pub(crate) const HEADER: &str =
    "kind\tcomponent\tstep\tfrom\tsign\tgrowth\tto\tgoal\tmatched_features\n";

pub(crate) fn clauses(p: &Program, m: &Model, graph: &Compressed) -> Result<String, String> {
    let mut text = HEADER.to_owned();
    if graph.witnesses.is_empty() {
        return Ok(text);
    }
    let index = Index::new(p, m, false);
    for witness in &graph.witnesses {
        for (step, (from, edge)) in witness.cycle.iter().enumerate() {
            let c = &p.clauses[*from];
            let mut calls = Vec::new();
            for goal in &c.body {
                match goal {
                    Goal::Positive(t) => calls.push((*t, false, None)),
                    Goal::Not(inner) => {
                        for goal in inner {
                            if let Goal::Positive(t) = goal {
                                calls.push((*t, true, Some(inner.as_slice())));
                            }
                        }
                    }
                    Goal::Builtin(_) => {}
                }
            }
            let matched = calls
                .into_iter()
                .filter(|(_, sign, _)| *sign == edge.negative)
                .find_map(|(goal, _, inner)| {
                    let pattern = Pattern::body(&p.arena, &m.abs, &c.body, c.names.len(), inner);
                    let mut counters = Counters::default();
                    let base = m.state(&p.arena, &pattern, &mut counters)?;
                    if !index
                        .candidates(p, m, goal, &base, false)
                        .contains(&edge.to)
                    {
                        return None;
                    }
                    let state = m.forced(p, &pattern, goal, edge.to, &base, &mut counters)?;
                    let state = m.feature_witness(&p.arena, &pattern, state, &mut counters)?;
                    Some((
                        p.arena.render(goal),
                        m.describe_state(&p.arena, &pattern, &state)
                            .replace(['\n', '\t'], " | "),
                    ))
                })
                .ok_or_else(|| {
                    format!(
                        "witness: no call supports {} -> {}",
                        p.label(*from),
                        p.label(edge.to)
                    )
                })?;
            text.push_str(&format!(
                "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
                witness.kind.name(),
                witness.component,
                step + 1,
                p.label(*from),
                if edge.negative { "-" } else { "+" },
                edge.growth,
                p.label(edge.to),
                matched.0,
                matched.1
            ));
        }
    }
    Ok(text)
}
