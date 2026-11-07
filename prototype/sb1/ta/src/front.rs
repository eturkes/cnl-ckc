//! Front-stage diagnostics, never a complete admission receipt.
use crate::domain::{Abstraction, Domain, Value};
use crate::features::{Feature, Label};
use crate::graph::{Edge, Graph};
use crate::source::Program;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

pub type Dump = (
    BTreeMap<Feature, BTreeSet<Value>>,
    BTreeMap<(Value, Value, Label), Domain>,
);
fn stream(path: &Path) -> Result<BufWriter<File>, String> {
    File::create(path)
        .map(BufWriter::new)
        .map_err(|e| e.to_string())
}
pub fn features(
    program: &Program,
    abs: &Abstraction,
    dump: Dump,
    out: &Path,
) -> Result<(), String> {
    fn values(d: &Domain) -> Vec<Value> {
        d.values()
            .map_or_else(|| vec![Value::Any], |s| s.iter().copied().collect())
    }
    let name = |v| {
        format!(
            "{}:{}",
            match v {
                Value::Any => "any",
                Value::Patient => "patient",
                Value::Number => "number",
                Value::Exact(_) => "exact",
                Value::Origin(_) => "origin",
            },
            abs.describe(&program.arena, v)
        )
    };
    let (unary, links) = dump;
    let mut dictionary = BTreeMap::<String, Value>::new();
    for (f, set) in &unary {
        dictionary.insert(name(f.selector()), f.selector());
        for value in set {
            dictionary.insert(name(*value), *value);
        }
    }
    for ((o, s, l), t) in &links {
        for value in [*o, *s, l.1].into_iter().chain(values(t)) {
            dictionary.insert(name(value), value);
        }
    }
    let ids = dictionary
        .values()
        .enumerate()
        .map(|(i, v)| (*v, i))
        .collect::<BTreeMap<_, _>>();
    let mut file = stream(&out.join("front-values.tsv"))?;
    for (i, (name, _)) in dictionary.iter().enumerate() {
        writeln!(file, "{i}\t{name}").map_err(|e| e.to_string())?;
    }
    file.flush().map_err(|e| e.to_string())?;
    let mut rows = Vec::new();
    for (feature, set) in unary {
        let (kind, predicate) = match feature {
            Feature::Noun(_) => ("noun", ""),
            Feature::Lemma(_) => ("lemma", ""),
            Feature::Operator(_) => ("operator", ""),
            Feature::Property(_) => ("property", ""),
            Feature::Payload(p, _) => ("payload", program.arena.name(p)),
            Feature::Tag(_) => ("tag", ""),
        };
        let mut targets = set.iter().map(|v| ids[v]).collect::<Vec<_>>();
        targets.sort_unstable();
        rows.push(format!(
            "{kind}\t{predicate}\t{}\t{}",
            ids[&feature.selector()],
            targets
                .iter()
                .map(usize::to_string)
                .collect::<Vec<_>>()
                .join(",")
        ));
    }
    rows.sort_unstable();
    let mut file = stream(&out.join("front-unary.tsv"))?;
    for row in rows {
        writeln!(file, "{row}").map_err(|e| e.to_string())?;
    }
    file.flush().map_err(|e| e.to_string())?;
    let mut rows = Vec::new();
    for ((owner, source, label), target) in links {
        let target = if target == Domain::All {
            "all".into()
        } else {
            let mut targets = values(&target).iter().map(|v| ids[v]).collect::<Vec<_>>();
            targets.sort_unstable();
            targets
                .iter()
                .map(usize::to_string)
                .collect::<Vec<_>>()
                .join(",")
        };
        rows.push((
            ids[&owner],
            ids[&source],
            program.arena.name(label.0).to_owned(),
            ids[&label.1],
            target,
        ));
    }
    rows.sort_unstable();
    let mut file = stream(&out.join("front-links.tsv"))?;
    for (owner, source, predicate, label, target) in rows {
        writeln!(file, "{owner}\t{source}\t{predicate}\t{label}\t{target}")
            .map_err(|e| e.to_string())?;
    }
    file.flush().map_err(|e| e.to_string())
}

pub fn graph(program: &Program, graph: &Graph, out: &Path) -> Result<(), String> {
    graph_rows(program, |i| graph.edges[i].clone(), &graph.components, out)
}
pub fn graph_rows(
    program: &Program,
    row: impl Fn(usize) -> Vec<Edge>,
    components: &[usize],
    out: &Path,
) -> Result<(), String> {
    // Original identities = source path + clause ordinal. Neither parse-local term
    // ids nor SCC discovery order enter these canonical objects.
    let mut nodes = (0..program.clauses.len()).collect::<Vec<_>>();
    nodes.sort_by_key(|i| {
        (
            program.documents[program.clauses[*i].document - 1]
                .path
                .clone(),
            program.clauses[*i].ordinal,
        )
    });
    let mut ids = vec![0; nodes.len()];
    for (canonical, original) in nodes.iter().enumerate() {
        ids[*original] = canonical;
    }
    let mut inventory = stream(&out.join("front-nodes.tsv"))?;
    let mut file = stream(&out.join("front-edges.tsv"))?;
    let mut groups = BTreeMap::<usize, Vec<usize>>::new();
    for (canonical, original) in nodes.iter().enumerate() {
        let clause = &program.clauses[*original];
        writeln!(
            inventory,
            "{canonical}\t{}\t{}\t{}",
            program.documents[clause.document - 1].path.display(),
            clause.ordinal,
            program.patient(*original)
        )
        .map_err(|e| e.to_string())?;
        groups
            .entry(components[*original])
            .or_default()
            .push(canonical);
        let mut edges = row(*original)
            .into_iter()
            .map(|e| Edge { to: ids[e.to], ..e })
            .collect::<Vec<_>>();
        edges.sort_unstable();
        for e in edges {
            writeln!(file, "{canonical}\t{}\t{}\t{}", e.to, e.negative, e.growth)
                .map_err(|e| e.to_string())?;
        }
    }
    inventory.flush().map_err(|e| e.to_string())?;
    file.flush().map_err(|e| e.to_string())?;
    let mut groups = groups.into_values().collect::<Vec<_>>();
    groups.sort_unstable();
    let mut file = stream(&out.join("front-scc.tsv"))?;
    for group in groups {
        writeln!(
            file,
            "{}",
            group
                .iter()
                .map(usize::to_string)
                .collect::<Vec<_>>()
                .join(",")
        )
        .map_err(|e| e.to_string())?;
    }
    file.flush().map_err(|e| e.to_string())
}
