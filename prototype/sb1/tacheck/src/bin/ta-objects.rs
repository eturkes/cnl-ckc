#![forbid(unsafe_code)]
use sb1_tacheck::{core, diagnostics, producer};
use std::path::PathBuf;

fn run() -> Result<(), (i32, String)> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.len() < 3 {
        return Err((
            2,
            "usage: ta-objects <root> <composition> <out> [--graph-only] [--query <file>]..."
                .into(),
        ));
    }
    let root = PathBuf::from(&args[0]);
    let composition = PathBuf::from(&args[1]);
    let out = PathBuf::from(&args[2]);
    let mut graph_only = false;
    let mut queries = Vec::new();
    let mut at = 3;
    while at < args.len() {
        match args[at].as_str() {
            "--graph-only" => {
                graph_only = true;
                at += 1;
            }
            "--query" => {
                queries.push(PathBuf::from(
                    args.get(at + 1).ok_or((2, "--query value".into()))?,
                ));
                at += 2;
            }
            _ => return Err((2, format!("argument {}", args[at]))),
        }
    }
    std::fs::create_dir_all(&out).map_err(|e| (2, e.to_string()))?;
    let mut p = sb1_ta::source::Program::load(&root, &composition, true).map_err(|e| (2, e))?;
    p.dc_shape().map_err(|e| (2, e))?;
    let mut a = sb1_ta::hints::analyze(&p).map_err(|e| (3, e))?;
    if !graph_only && let Some(r) = a.refined.as_mut() {
        let g = r.hint_growth();
        assert_eq!(
            g.nodes, r.growth.nodes,
            "hint diagnostic type registry order"
        );
        let labels = r.canonical_nodes(&p, &a.model);
        let mut rows = Vec::new();
        for from in 0..g.nodes.len() {
            for edge in g.graph.row(from) {
                rows.push(format!("{}\t{}\n", labels[from], labels[edge.to]));
            }
        }
        rows.sort();
        std::fs::write(out.join("hint-final-full-edges.tsv"), rows.concat())
            .map_err(|e| (2, e.to_string()))?;
    }
    let h = sb1_ta::hints::export(&p, a).map_err(|e| (3, e))?;
    let p = producer::program(p);
    let classify = |e: core::Failure| {
        let rc = if matches!(e, core::Failure::Indeterminate(_)) {
            3
        } else {
            1
        };
        (rc, format!("{e:?}"))
    };
    let cert = producer::decode(&p, h).map_err(classify)?;
    let (mut c, d) = core::analysis::check_diagnostic(p, cert, &mut core::Budget::default())
        .map_err(classify)?;
    diagnostics::dump(&c, &d, &out, graph_only).map_err(|e| (2, e))?;
    let mut rows = String::from("query\troots\tselected\treference_equal\n");
    for query in queries {
        let bytes = std::fs::read(&query).map_err(|e| (2, e.to_string()))?;
        let (selected, roots) = c
            .select_diagnostic(&bytes, &mut core::Budget::default())
            .map_err(classify)?;
        let reference = diagnostics::reference_slice(&c, &out, &roots).map_err(|e| (2, e))?;
        if selected != reference {
            return Err((
                1,
                format!(
                    "reference reachability differs {} actual={selected:?} expected={reference:?}",
                    query.display()
                ),
            ));
        }
        let name = query
            .file_name()
            .ok_or((2, "query name".into()))?
            .to_string_lossy()
            .replace(['.', '-'], "_");
        let mut chosen = selected
            .iter()
            .map(|i| {
                let clause = &c.program.clauses[*i];
                format!(
                    "{}\t{}\tfalse\n",
                    c.program.documents[clause.document - 1],
                    clause.ordinal
                )
            })
            .collect::<Vec<_>>();
        chosen.extend(
            c.program
                .clauses
                .iter()
                .enumerate()
                .filter(|(i, _)| c.program.patient[*i])
                .map(|(_, clause)| {
                    format!(
                        "{}\t{}\ttrue\n",
                        c.program.documents[clause.document - 1],
                        clause.ordinal
                    )
                }),
        );
        chosen.sort();
        std::fs::write(out.join(format!("slice-{name}.tsv")), chosen.concat())
            .map_err(|e| (2, e.to_string()))?;
        rows.push_str(&format!(
            "{}\t{}\t{}\ttrue\n",
            query.display(),
            roots.len(),
            selected.len()
        ));
    }
    std::fs::write(out.join("generic-slices.tsv"), rows).map_err(|e| (2, e.to_string()))?;
    println!(
        "admitted\tclauses={}\tedges={}\tdiagnostic",
        c.graph.nodes(),
        c.graph.targets.len()
    );
    Ok(())
}
fn main() {
    if let Err((rc, text)) = run() {
        eprintln!("{text}");
        std::process::exit(rc);
    }
}
