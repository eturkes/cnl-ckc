use sb1_ta::Analysis;
use sb1_ta::graph::{Graph, Question};
use sb1_ta::growth::Growth;
use sb1_ta::k1::Refined;
use sb1_ta::may::{Mode, Model};
use sb1_ta::source::Program;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

fn write(path: &Path, text: &str) -> Result<(), String> {
    fs::write(path, text).map_err(|e| format!("{}: {e}", path.display()))
}
fn phase(out: &Path, phases: &mut String, name: &str, seconds: f64) -> Result<(), String> {
    phases.push_str(&format!("{name}\t{seconds:.9}\n"));
    write(&out.join("phases.tsv"), phases)
}
fn rss() -> String {
    fs::read_to_string("/proc/self/status")
        .ok()
        .and_then(|s| {
            s.lines().find_map(|l| {
                l.strip_prefix("VmHWM:")
                    .map(|v| v.split_whitespace().next().unwrap_or("0").to_owned())
            })
        })
        .and_then(|s| s.parse::<u64>().ok())
        .map_or_else(|| "unavailable".into(), |n| (n * 1024).to_string())
}
fn report(
    model: &Model,
    graph: &Graph,
    growth: &Growth,
    refined: Option<&Refined>,
    inherited_growth: bool,
    phase: &str,
    wall: f64,
) -> String {
    let clauses = graph.edges.len();
    let may = &model.counters;
    let edges = &graph.counters;
    let active = refined.map_or(growth, |r| &r.growth);
    let mut features = may.clone();
    let mut candidate_pairs =
        may.candidate_pairs + edges.candidate_pairs + growth.graph.counters.candidate_pairs;
    let mut join_rows = may.join_rows + edges.join_rows + growth.graph.counters.join_rows;
    if let Some(r) = refined {
        features.feature_insertions += r.model.counters.feature_insertions;
        features.writer_insertions += r.model.counters.writer_insertions;
        features.worklist_steps += r.model.counters.worklist_steps;
        candidate_pairs +=
            r.model.counters.candidate_pairs + r.growth.graph.counters.candidate_pairs;
        join_rows += r.model.counters.join_rows + r.growth.graph.counters.join_rows;
    }
    let growth_mode = if inherited_growth {
        "checked-superset"
    } else if refined.is_some() {
        "k1"
    } else {
        "base"
    };
    format!(
        "phase\tverdict\tclauses\tfeatures\tfeature_insertions\twriter_insertions\tcandidate_pairs\tedges\tscc_count\tscc_max\tworklist_steps\tjoin_rows\twall_seconds\tpeak_rss_bytes\tgrowth_nodes\tgrowth_edges\tgrowth_scc_count\tgrowth_scc_max\tprecision_fallbacks\tgrowth_mode\tbase_growth_cycles\tk1_types\tk1_variants\trefined_templates\tprojection_checks\tfull_refinement\tgrowth_cycles\n{phase}\t{}\t{clauses}\t{}\t{}\t{}\t{candidate_pairs}\t{}\t{}\t{}\t{}\t{join_rows}\t{wall:.9}\t{}\t{}\t{}\t{}\t{}\t{}\t{growth_mode}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
        if graph.admitted() && (inherited_growth || active.admitted()) {
            "admitted"
        } else {
            "refused"
        },
        model.fact_count(),
        features.feature_insertions,
        features.writer_insertions,
        graph.edge_count(),
        graph.sizes.len(),
        graph.sizes.iter().max().unwrap_or(&0),
        features.worklist_steps,
        rss(),
        active.nodes.len(),
        active.graph.edge_count(),
        active.graph.sizes.len(),
        active.graph.sizes.iter().max().unwrap_or(&0),
        growth.precision_fallbacks,
        growth.graph.witnesses.len(),
        refined.map_or(0, |r| r.keys.len()),
        refined.map_or(0, |r| r.generated_variants),
        refined.map_or(0, |r| r.refined_templates),
        refined.map_or(0, |r| r.projection_checks),
        refined.is_some_and(|r| r.full_refinement),
        if inherited_growth {
            0
        } else {
            active.graph.witnesses.len()
        },
    )
}
fn run() -> Result<i32, String> {
    let mut root = None;
    let mut composition = None;
    let mut out = None;
    let mut dc = false;
    let mut patient_last = true;
    let mut emit = None;
    let mut queries = Vec::new();
    let mut slice_dir = None;
    let mut emit_edges = false;
    let mut mode = Mode::Refined;
    let mut shape_only = false;
    let mut features_only = false;
    let mut compare_reference = false;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--root" => root = Some(PathBuf::from(args.next().ok_or("--root value")?)),
            "--composition" => {
                composition = Some(PathBuf::from(args.next().ok_or("--composition value")?))
            }
            "--out" => out = Some(PathBuf::from(args.next().ok_or("--out value")?)),
            "--dc" => dc = true,
            "--reference" => mode = Mode::Reference,
            "--shape-only" => shape_only = true,
            "--features-only" => features_only = true,
            "--compare-reference" => compare_reference = true,
            "--edges" => emit_edges = true,
            "--patient" => {
                patient_last = match args.next().as_deref() {
                    Some("none") => false,
                    Some("last") => true,
                    _ => return Err("--patient none|last".into()),
                }
            }
            "--emit" => emit = Some(PathBuf::from(args.next().ok_or("--emit value")?)),
            "--query" => queries.push(PathBuf::from(args.next().ok_or("--query value")?)),
            "--slice-dir" => {
                slice_dir = Some(PathBuf::from(args.next().ok_or("--slice-dir value")?))
            }
            _ => {
                return Err(format!(
                    "argument {arg}; usage: ta --root ROOT --composition FILE --out DIR [--dc] [--reference] [--shape-only --emit DIR | --features-only] [--compare-reference] [--edges] [--patient none|last] [--emit DIR] [--query FILE]... [--slice-dir DIR]"
                ));
            }
        }
    }
    if shape_only && (emit.is_none() || features_only || compare_reference || !queries.is_empty()) {
        return Err("--shape-only requires --emit and no analysis/query options".into());
    }
    if compare_reference && mode == Mode::Reference {
        return Err("--compare-reference requires refined mode".into());
    }
    let root = root.ok_or("--root required")?;
    let composition = composition.ok_or("--composition required")?;
    let out = out.ok_or("--out required")?;
    fs::create_dir_all(&out).map_err(|e| e.to_string())?;
    let start = Instant::now();
    let mut program = Program::load(&root, &composition, patient_last)?;
    let guard_goals = if dc { program.dc_shape()? } else { 0 };
    let mut questions = Vec::new();
    for q in queries {
        let bytes = fs::read(&q).map_err(|e| e.to_string())?;
        questions.push((q, Question::parse(&mut program.arena, &bytes)?));
    }
    let parse_seconds = start.elapsed().as_secs_f64();
    let mut phases = String::from("phase\tseconds\n");
    phase(&out, &mut phases, "parse_shape", parse_seconds)?;
    program.record_memory("parse_shape");
    write(
        &out.join("transform.tsv"),
        &format!(
            "key\tvalue\nclauses\t{}\ndocuments\t{}\nguard_goals_added\t{guard_goals}\nchanged_clauses\t{}\npatient_documents\t{}\n",
            program.clauses.len(),
            program.documents.len(),
            program.changed.len(),
            program.documents.iter().filter(|d| d.patient).count(),
        ),
    )?;
    if shape_only {
        println!("shape-only\tno-admission\tguard_goals_added={guard_goals}");
        println!(
            "composition\t{}",
            program.emit(&emit.unwrap(), None)?.display()
        );
        return Ok(0);
    }
    let start = Instant::now();
    let model = Model::build_mode(&program, None, true, None, mode);
    let may_seconds = start.elapsed().as_secs_f64();
    phase(&out, &mut phases, "may_fixpoint", may_seconds)?;
    model.record_memory("may_fixpoint");
    let (unary, links) = model.feature_counts();
    write(
        &out.join("features.tsv"),
        &format!(
            "mode\tunary\tlinks\tincidences\tfeature_insertions\twriter_insertions\tworklist_steps\tseconds\tpeak_rss_bytes\n{mode:?}\t{unary}\t{links}\t{}\t{}\t{}\t{}\t{may_seconds:.9}\t{}\n",
            model.fact_count(),
            model.counters.feature_insertions,
            model.counters.writer_insertions,
            model.counters.worklist_steps,
            rss(),
        ),
    )?;
    if compare_reference {
        let reference = Model::build_mode(
            &program,
            None,
            true,
            Some(model.abs.clone()),
            Mode::Reference,
        );
        model.refines_reference(&reference, &program.arena)?;
        write(
            &out.join("refinement.tsv"),
            "check\tresult\nunary-subset\tpass\nowner-forgotten-link-subset\tpass\n",
        )?;
        println!("features\trefined-subset-reference");
    }
    if features_only {
        println!(
            "features-only\tno-admission\tincidences={}",
            model.fact_count()
        );
        return Ok(0);
    }
    let start = Instant::now();
    let graph = Graph::build(&program, &model, None);
    let graph_seconds = start.elapsed().as_secs_f64();
    phase(&out, &mut phases, "graph_scc", graph_seconds)?;
    graph.record_memory("graph_scc");
    let start = Instant::now();
    let growth = Growth::build(&program, &model, None);
    let growth_seconds = start.elapsed().as_secs_f64();
    phase(&out, &mut phases, "origin_growth", growth_seconds)?;
    growth.record_memory("origin_growth");
    let start = Instant::now();
    let analysis = Analysis::from_parts(&program, model, graph, growth)?;
    let k1_seconds = start.elapsed().as_secs_f64();
    phase(&out, &mut phases, "k1_growth", k1_seconds)?;
    if let Some(r) = &analysis.refined {
        r.program.record_memory("k1-source");
        r.model.record_memory("k1-features");
        r.growth.record_memory("k1-growth");
    }
    write(
        &out.join("analysis.tsv"),
        &report(
            &analysis.model,
            &analysis.graph,
            &analysis.growth,
            analysis.refined.as_ref(),
            false,
            "load-analysis",
            may_seconds + graph_seconds + growth_seconds + k1_seconds,
        ),
    )?;
    write(&out.join("witnesses.tsv"), &analysis.witnesses(&program))?;
    if emit_edges {
        if let Some(refined) = &analysis.refined {
            let file = fs::File::create(out.join("growth-edges.tsv")).map_err(|e| e.to_string())?;
            let mut stream = std::io::BufWriter::new(file);
            refined
                .write_edges(&program, &analysis.model, &mut stream)
                .map_err(|e| e.to_string())?;
            std::io::Write::flush(&mut stream).map_err(|e| e.to_string())?;
        } else {
            write(
                &out.join("growth-edges.tsv"),
                &analysis.growth.edge_table(&program, &analysis.model),
            )?;
        }
    }
    println!(
        "load\t{}\tclauses={}\tfeatures={}\tedges={}\tguard_goals_added={guard_goals}",
        if analysis.admitted() {
            "admitted"
        } else {
            "refused"
        },
        program.clauses.len(),
        analysis.model.fact_count(),
        analysis.graph.edge_count()
    );
    if !analysis.admitted() {
        return Ok(1);
    }
    if let Some(emit) = emit {
        println!("composition\t{}", program.emit(&emit, None)?.display());
    }
    let mut slices = String::from(
        "query\tverdict\tcoarse_clauses\tslice_clauses\tslice_rules\tslice_documents\tslice_features\tcandidate_pairs\tworklist_steps\twall_seconds\tpeak_rss_bytes\n",
    );
    for (path, question) in questions {
        let start = Instant::now();
        let slice = analysis.slice(&program, &question);
        let seconds = start.elapsed().as_secs_f64();
        let name = path
            .file_name()
            .ok_or("query name")?
            .to_string_lossy()
            .replace(['.', '-'], "_");
        let documents = slice
            .clauses
            .iter()
            .map(|i| program.clauses[*i].document)
            .collect::<std::collections::BTreeSet<_>>();
        let rules = slice
            .clauses
            .iter()
            .filter(|i| !program.clauses[**i].body.is_empty())
            .count();
        let candidates = slice.model.counters.candidate_pairs
            + slice.graph.counters.candidate_pairs
            + slice.growth.graph.counters.candidate_pairs
            + slice.query_counters.candidate_pairs;
        let row = format!(
            "{}\t{}\t{}\t{}\t{rules}\t{}\t{}\t{candidates}\t{}\t{seconds:.9}\t{}\n",
            path.display(),
            if slice.admitted() {
                "admitted"
            } else {
                "refused"
            },
            slice.coarse_clauses,
            slice.clauses.len(),
            documents.len(),
            slice.model.fact_count(),
            slice.model.counters.worklist_steps,
            rss()
        );
        print!("slice\t{row}");
        slices.push_str(&row);
        write(
            &out.join(format!("{name}-analysis.tsv")),
            &report(
                &slice.model,
                &slice.graph,
                &slice.growth,
                None,
                slice.inherited_growth,
                "slice-analysis",
                seconds,
            ),
        )?;
        write(
            &out.join(format!("{name}-witnesses.tsv")),
            &slice.witnesses(&program),
        )?;
        let ids = slice
            .clauses
            .iter()
            .filter(|i| !program.patient(**i))
            .map(|i| {
                let clause = &program.clauses[*i];
                format!("{}:{}\n", clause.document, clause.ordinal)
            })
            .collect::<String>();
        write(&out.join(format!("{name}-ids.txt")), &ids)?;
        if !slice.admitted() {
            write(&out.join("slices.tsv"), &slices)?;
            return Ok(1);
        }
        if let Some(dir) = &slice_dir {
            println!(
                "slice-composition\t{}",
                program
                    .emit(&dir.join(&name), Some(&slice.clauses))?
                    .display()
            );
        }
    }
    write(&out.join("slices.tsv"), &slices)?;
    Ok(0)
}
fn main() {
    match run() {
        Ok(rc) => std::process::exit(rc),
        Err(e) => {
            eprintln!("ta: {e}");
            std::process::exit(2);
        }
    }
}
