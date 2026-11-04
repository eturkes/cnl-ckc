use sb1_ta::{features::Model, graph::Graph, source::Program};
use std::path::Path;
use std::time::Instant;
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let mut p = Program::load(
        Path::new(&args[1]),
        Path::new(&args[2]),
        args.get(3).is_some_and(|a| a == "last"),
    )
    .unwrap();
    p.dc_shape().unwrap();
    let start = Instant::now();
    let model = Model::build(&p, None, true);
    let features = start.elapsed().as_secs_f64();
    let start = Instant::now();
    let graph = Graph::build(&p, &model, None);
    let seconds = start.elapsed().as_secs_f64();
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
        "clauses\tfeatures\tpatterns\tcache_hits\tcandidates\tedges\tsccs\tnegative_sccs\tfeature_seconds\tgraph_seconds\tpeak_rss_bytes\n{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{features:.9}\t{seconds:.9}\t{rss}",
        p.clauses.len(),
        model.fact_count(),
        graph.body_patterns,
        graph.body_cache_hits,
        graph.counters.candidate_pairs,
        graph.edge_count(),
        graph.sizes.len(),
        graph.witnesses.len()
    );
}
