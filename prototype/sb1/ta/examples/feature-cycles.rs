// Clause-cycle replay + noun/shape probes; admission uses the origin-flow graph.
use sb1_ta::{
    features::{Counters, Model},
    graph::{Edge, Graph},
    source::Program,
};
use std::path::Path;
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let mut p = Program::load(Path::new(&args[1]), Path::new(&args[2]), false).unwrap();
    p.dc_shape().unwrap();
    let m = Model::build(&p, None, true);
    let mut edges = vec![Vec::new(); p.clauses.len()];
    for (doc, from, to, goal) in [
        ("cdc2022-opioid-s9-05.pl", 46, 61, 5),
        ("cdc2022-opioid-s9-05.pl", 61, 46, 2),
        ("cdc2022-opioid-s10-13.pl", 8, 23, 0),
        ("cdc2022-opioid-s10-13.pl", 23, 8, 9),
    ] {
        let find = |ordinal| {
            p.clauses
                .iter()
                .enumerate()
                .find(|(_, c)| {
                    c.ordinal == ordinal && p.documents[c.document - 1].path.ends_with(doc)
                })
                .unwrap()
                .0
        };
        let from = find(from);
        let to = find(to);
        let sb1_checker::program::Goal::Positive(_) = p.clauses[from].body[goal] else {
            panic!("goal")
        };
        edges[from].push(Edge {
            to,
            negative: false,
            growth: true,
        });
    }
    let graph = Graph::from_edges(edges, Counters::default());
    print!("{}", graph.render_witnesses(&p, &m));
    println!("origin\tpositive_noun\tpositive\tcycle_required_noun\thas_required_noun");
    for (doc, ordinal, positive, required) in [
        ("cdc2022-opioid-s9-05.pl", 46, "acute-pain", "pain"),
        ("cdc2022-opioid-s9-05.pl", 51, "subacute-pain", "pain"),
        ("cdc2022-opioid-s9-05.pl", 56, "chronic-pain", "pain"),
        ("cdc2022-opioid-s10-13.pl", 23, "palliative-care", "care"),
    ] {
        let i = p
            .clauses
            .iter()
            .enumerate()
            .find(|(_, c)| c.ordinal == ordinal && p.documents[c.document - 1].path.ends_with(doc))
            .unwrap()
            .0;
        let (_, a) = p.arena.call(p.clauses[i].head).unwrap();
        println!(
            "{}\t{positive}\t{}\t{required}\t{}",
            p.label(i),
            m.noun_at(&p.arena, a[1], positive),
            m.noun_at(&p.arena, a[1], required)
        );
    }
    let args = p
        .clauses
        .iter()
        .filter_map(|c| p.arena.named(c.head, "guideline_arg", 4))
        .collect::<Vec<_>>();
    let variable_events = args
        .iter()
        .filter(|a| matches!(p.arena.node(a[1]), sb1_checker::term::Node::Var(_)))
        .count();
    let variable_contexts = p
        .clauses
        .iter()
        .filter_map(|c| p.arena.call(c.head))
        .filter(|(n, a)| {
            matches!(
                p.arena.name(*n),
                "guideline_entity"
                    | "guideline_event"
                    | "guideline_arg"
                    | "guideline_pp"
                    | "guideline_cardinality"
                    | "guideline_operator"
            ) && matches!(p.arena.node(a[0]), sb1_checker::term::Node::Var(_))
        })
        .count();
    println!(
        "shape_control\tvalue\nclauses\t{}\nargument_heads\t{}\nvariable_event_argument_heads\t{variable_events}\nvariable_semantic_context_heads\t{variable_contexts}",
        p.clauses.len(),
        args.len()
    );
}
