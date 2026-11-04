use sb1_ta::{features::Model, growth::Growth, k1::Refined, source::Program};
use std::path::Path;
use std::time::Instant;
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let mut p = Program::load(
        Path::new(&args[1]),
        Path::new(&args[2]),
        args.get(3).is_some_and(|s| s == "last"),
    )
    .unwrap();
    p.dc_shape().unwrap();
    let start = Instant::now();
    let m = Model::build(&p, None, true);
    let f = start.elapsed().as_secs_f64();
    let start = Instant::now();
    let base = Growth::build(&p, &m, None);
    let b = start.elapsed().as_secs_f64();
    let start = Instant::now();
    let refined = Refined::build(&p, &m, &base).unwrap();
    let k = start.elapsed().as_secs_f64();
    let status = std::fs::read_to_string("/proc/self/status").unwrap();
    let rss = status
        .lines()
        .find_map(|l| {
            l.strip_prefix("VmHWM:")
                .and_then(|l| l.split_whitespace().next())
                .and_then(|n| n.parse::<u64>().ok())
        })
        .unwrap()
        * 1024;
    println!(
        "metric\tvalue\nsource_clauses\t{}\nbase_growth_edges\t{}\nbase_growth_sccs\t{}\nk1_types\t{}\nrefined_templates\t{}\nprojection_checks\t{}\nfull_refinement\t{}\nk1_variants\t{}\nk1_edges\t{}\nk1_growth_sccs\t{}\nassignments\t{}\nrejected_assignments\t{}\nfeature_seconds\t{f:.9}\nbase_flow_seconds\t{b:.9}\nk1_seconds\t{k:.9}\npeak_rss_bytes\t{rss}",
        p.clauses.len(),
        base.graph.edge_count(),
        base.graph.witnesses.len(),
        refined.keys.len(),
        refined.refined_templates,
        refined.projection_checks,
        refined.full_refinement,
        refined.generated_variants,
        refined.growth.graph.edge_count(),
        refined.growth.graph.witnesses.len(),
        refined.candidate_assignments,
        refined.rejected_assignments
    );
    print!("{}", refined.render_witnesses(&p, &m));
    std::process::exit(if refined.admitted() { 0 } else { 1 });
}
