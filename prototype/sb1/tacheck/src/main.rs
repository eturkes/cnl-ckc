#![forbid(unsafe_code)]
use sb1_tacheck::{core, producer};
use std::path::PathBuf;

fn run() -> Result<(), (i32, String)> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let (root, composition, already_shaped) = match args.as_slice() {
        [root, composition] => (root, composition, false),
        [root, composition, flag] if flag == "--already-shaped" => (root, composition, true),
        _ => {
            return Err((
                2,
                "usage: sb1-tacheck <root> <composition> [--already-shaped]".into(),
            ));
        }
    };
    let mut p =
        sb1_ta::source::Program::load(&PathBuf::from(root), &PathBuf::from(composition), true)
            .map_err(|e| (2, e))?;
    if !already_shaped {
        p.dc_shape().map_err(|e| (2, e))?;
    }
    let a = sb1_ta::hints::analyze(&p).map_err(|e| (3, format!("indeterminate solver {e}")))?;
    let hint = sb1_ta::hints::export(&p, a)
        .map_err(|e| (3, format!("indeterminate hint producer {e}")))?;
    let labels = p
        .clauses
        .iter()
        .map(|c| (c.document, c.line + 1, c.ordinal))
        .collect::<Vec<_>>();
    let documents = p
        .documents
        .iter()
        .map(|d| d.path.display().to_string())
        .collect::<Vec<_>>();
    let label = |clause: usize| {
        let (doc, line, ordinal) = labels[clause];
        format!("{}:{line}:{ordinal}", documents[doc - 1])
    };
    let p = producer::program(p);
    let certificate = producer::decode(&p, hint).map_err(|e| match e {
        core::Failure::Indeterminate(detail) => (3, format!("indeterminate {detail}")),
        other => (1, format!("refused (certificate) {other:?}")),
    })?;
    match core::analysis::check(p, certificate, &mut core::Budget::default()) {
        Ok(checked) => {
            println!(
                "admitted\tclauses={}\tedges={}",
                checked.graph.nodes(),
                checked.graph.targets.len()
            );
            Ok(())
        }
        Err(core::Failure::Admission(detail)) => Err((1, format!("refused (admission) {detail}"))),
        Err(core::Failure::Certificate(detail)) => {
            Err((1, format!("refused (certificate) {detail}")))
        }
        Err(core::Failure::Indeterminate(detail)) => Err((3, format!("indeterminate {detail}"))),
        Err(core::Failure::Cycle {
            kind,
            edges,
            sites,
            labels,
        }) => {
            let mut text = format!("refused ({kind} cycle)\n");
            if kind == "negative" {
                for (from, to, negative) in edges {
                    text.push_str(&format!(
                        "{}\t{}\t{}\n",
                        label(from),
                        label(to),
                        if negative { "-" } else { "+" }
                    ));
                }
            } else {
                if edges.len() != sites.len() || edges.len() != labels.len() {
                    return Err((1, "refused (certificate) incomplete growth witness".into()));
                }
                for ((from, to), (clause, variable)) in labels.into_iter().zip(sites) {
                    text.push_str(&format!(
                        "{from}\t{to}\t{}\t{}\n",
                        label(clause),
                        sb1_checker::term::variable_name(variable)
                    ));
                }
            }
            Err((1, text.trim_end().into()))
        }
    }
}
fn main() {
    if let Err((rc, message)) = run() {
        eprintln!("{message}");
        std::process::exit(rc);
    }
}
