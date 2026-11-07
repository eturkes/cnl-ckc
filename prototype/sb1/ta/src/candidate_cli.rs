use super::{phase, rss, write};
use sb1_ta::complete::Analysis;
use sb1_ta::exact::Model;
use sb1_ta::exact_growth::Growth;
use sb1_ta::graph::Question;
use sb1_ta::source::Program;
use std::path::{Path, PathBuf};
use std::time::Instant;

pub struct Options {
    pub dump_front: bool,
    pub dump_graph: bool,
    pub dump_analysis: bool,
    pub emit_edges: bool,
    pub skip_audit: bool,
    pub header_audit: bool,
    pub emit: Option<PathBuf>,
    pub slice_dir: Option<PathBuf>,
}
fn stats(model: &Model, out: &Path, name: &str) -> Result<(), String> {
    let s = model.stats.borrow();
    let mut metrics = format!(
        "key\tvalue\nevaluations\t{}\nskipped\t{}\naudited\t{}\nlink_groups\t{}\nlink_rectangles\t{}\nheader_candidates\t{}\nheader_survivors\t{}\nheader_audits\t{}\nbody_solves\t{}\nstate_cells_copied\t{}\nheader_point_checks\t{}\ncollections\t{}\nallocation_charge\t{}\n",
        s.evaluations,
        s.skipped,
        s.audited,
        s.link_groups,
        s.link_rectangles,
        s.header_candidates,
        s.header_survivors,
        s.header_audits,
        s.body_solves,
        s.state_cells_copied,
        s.header_point_checks,
        s.collections,
        model.sets.allocation_charge()
    );
    metrics.push_str(&format!("header_requests\t{}\n", s.header_requests));
    for (key, value) in model.sets.metrics().into_iter().chain(model.retention()) {
        metrics.push_str(&format!("{key}\t{value}\n"));
    }
    write(&out.join(format!("{name}-stats.tsv")), &metrics)
}
fn flow_stats(g: &Growth, out: &Path, name: &str) -> Result<(), String> {
    let mut text = format!(
        "key\tvalue\nevaluations\t{}\nskipped\t{}\naudited\t{}\nforwarded_memberships\t{}\nallowed_checks\t{}\nsource_patterns\t{}\nsource_cache_hits\t{}\n",
        g.stats.evaluations,
        g.stats.skipped,
        g.stats.audited,
        g.stats.forwarded_memberships,
        g.stats.allowed_checks,
        g.source_patterns,
        g.source_cache_hits
    );
    for (key, value) in g.graph.metrics() {
        text.push_str(&format!("{key}\t{value}\n"));
    }
    write(&out.join(format!("{name}-stats.tsv")), &text)
}
pub fn run(
    program: &Program,
    out: &Path,
    phases: &mut String,
    questions: &[(PathBuf, Question)],
    options: Options,
) -> Result<i32, String> {
    let start = Instant::now();
    let mut model = Model::build(program, None, true, None, options.skip_audit);
    let may = start.elapsed().as_secs_f64();
    phase(out, phases, "may_fixpoint", may)?;
    if options.dump_front {
        sb1_ta::front::features(program, &model.abs, model.dump(), out)?;
    }
    let start = Instant::now();
    let graph = sb1_ta::exact_graph::build_compressed(program, &mut model, options.header_audit);
    let graph_seconds = start.elapsed().as_secs_f64();
    phase(out, phases, "graph_scc", graph_seconds)?;
    if options.dump_front || options.dump_graph {
        sb1_ta::front::graph_rows(program, |i| graph.row(i), &graph.components, out)?;
    }
    let start = Instant::now();
    let growth = Growth::build(
        program,
        &mut model,
        options.header_audit,
        options.skip_audit,
    );
    let growth_seconds = start.elapsed().as_secs_f64();
    phase(out, phases, "origin_growth", growth_seconds)?;
    let start = Instant::now();
    let a = Analysis::from_parts(
        program,
        model,
        graph,
        growth,
        options.header_audit,
        options.skip_audit,
    )?;
    let k1_seconds = start.elapsed().as_secs_f64();
    phase(out, phases, "k1_growth", k1_seconds)?;
    stats(&a.model, out, "base")?;
    flow_stats(&a.growth, out, "base-flow")?;
    let active = if let Some(r) = &a.refined {
        stats(&r.model, out, "k1")?;
        flow_stats(&r.growth, out, "k1-flow")?;
        &r.growth
    } else {
        &a.growth
    };
    let wall = may + graph_seconds + growth_seconds + k1_seconds;
    let verdict = if a.admitted() { "admitted" } else { "refused" };
    write(
        &out.join("analysis.tsv"),
        &format!(
            "phase\tverdict\tclauses\twall_seconds\tpeak_rss_bytes\tedges\tscc_count\tgrowth_nodes\tgrowth_edges\tgrowth_scc_count\tprecision_fallbacks\tgrowth_mode\tbase_growth_cycles\tk1_types\tk1_variants\trefined_templates\tprojection_checks\tfull_refinement\tgrowth_cycles\nload-analysis\t{verdict}\t{}\t{wall:.9}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
            program.clauses.len(),
            rss(),
            a.graph.edge_count(),
            a.graph.sizes.len(),
            active.nodes.len(),
            active.graph.edge_count(),
            active.graph.sizes.len(),
            a.growth.precision_fallbacks,
            if a.refined.is_some() { "k1" } else { "base" },
            a.growth.graph.witnesses.len(),
            a.refined.as_ref().map_or(0, |r| r.keys.len()),
            a.refined.as_ref().map_or(0, |r| r.generated_variants),
            a.refined.as_ref().map_or(0, |r| r.refined_templates),
            a.refined.as_ref().map_or(0, |r| r.projection_checks),
            a.refined.as_ref().is_some_and(|r| r.full_refinement),
            active.graph.witnesses.len()
        ),
    )?;
    if options.dump_analysis {
        a.dump(program, out)?;
    }
    write(&out.join("witnesses.tsv"), &a.witnesses(program)?)?;
    if options.emit_edges {
        if let Some(refined) = &a.refined {
            let file =
                std::fs::File::create(out.join("growth-edges.tsv")).map_err(|e| e.to_string())?;
            let mut stream = std::io::BufWriter::new(file);
            refined
                .write_edges(program, &a.model, &mut stream)
                .map_err(|e| e.to_string())?;
            std::io::Write::flush(&mut stream).map_err(|e| e.to_string())?;
        } else {
            write(
                &out.join("growth-edges.tsv"),
                &a.growth.edge_table(program, &a.model),
            )?;
        }
    }
    println!(
        "load\t{verdict}\tclauses={}\tedges={}",
        program.clauses.len(),
        a.graph.edge_count()
    );
    if !a.admitted() {
        return Ok(1);
    }
    if let Some(emit) = options.emit {
        println!("composition\t{}", program.emit(&emit, None)?.display());
    }
    let mut slices = String::from(
        "query\tverdict\tcoarse_clauses\tslice_clauses\twall_seconds\tpeak_rss_bytes\n",
    );
    for (path, q) in questions {
        let start = Instant::now();
        let slice = a.slice(program, q, options.header_audit, options.skip_audit);
        let name = path
            .file_name()
            .ok_or("query name")?
            .to_string_lossy()
            .replace(['.', '-'], "_");
        if options.dump_analysis {
            sb1_ta::complete::slice_dump(program, &slice.clauses, out, &name)?;
        }
        let verdict = if slice.admitted {
            "admitted"
        } else {
            "refused"
        };
        let row = format!(
            "{}\t{verdict}\t{}\t{}\t{:.9}\t{}\n",
            path.display(),
            slice.coarse_clauses,
            slice.clauses.len(),
            start.elapsed().as_secs_f64(),
            rss()
        );
        print!("slice\t{row}");
        slices.push_str(&row);
        let ids = slice
            .clauses
            .iter()
            .filter(|i| !program.patient(**i))
            .map(|i| {
                let c = &program.clauses[*i];
                format!("{}:{}\n", c.document, c.ordinal)
            })
            .collect::<String>();
        write(&out.join(format!("{name}-ids.txt")), &ids)?;
        if !slice.admitted {
            write(&out.join("slices.tsv"), &slices)?;
            return Ok(1);
        }
        if let Some(dir) = &options.slice_dir {
            println!(
                "slice-composition\t{}",
                program
                    .emit(&dir.join(name), Some(&slice.clauses))?
                    .display()
            );
        }
    }
    write(&out.join("slices.tsv"), &slices)?;
    Ok(0)
}
