//! Canonical objects from checked data; no census matching/transfer routines.
use crate::core::analysis::{Checked, Diagnostics};
use crate::core::growth::TypeNode;
use crate::core::{self, Csr, May, Program};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

type Result<T> = std::result::Result<T, String>;
fn stream(out: &Path, name: &str) -> Result<BufWriter<File>> {
    File::create(out.join(name))
        .map(BufWriter::new)
        .map_err(|e| e.to_string())
}
fn save(out: &Path, name: &str, text: &str) -> Result<()> {
    fs::write(out.join(name), text).map_err(|e| e.to_string())
}
fn write_rows(out: &Path, name: &str, mut rows: Vec<String>) -> Result<()> {
    rows.sort();
    if rows.windows(2).any(|w| w[0] == w[1]) {
        return Err(format!("duplicate canonical {name} row"));
    }
    let mut file = stream(out, name)?;
    for row in rows {
        writeln!(file, "{row}").map_err(|e| e.to_string())?;
    }
    file.flush().map_err(|e| e.to_string())
}
fn features(p: &Program, m: &May, out: &Path) -> Result<()> {
    let mut dictionary = BTreeMap::<String, u32>::new();
    let name = |v| m.abs.describe(&p.arena, v);
    let mut unary = Vec::new();
    for (f, set) in m.unary_rows() {
        dictionary.insert(name(f.selector), f.selector);
        let values = m.sets.values(set).ok_or("nonfinite raw unary")?;
        for v in values.iter() {
            dictionary.insert(name(*v), *v);
        }
        unary.push((f, values));
    }
    let mut links = BTreeMap::<(u32, u32, u32, u32), Option<BTreeSet<u32>>>::new();
    for r in &m.rectangles {
        let targets = m.sets.values(r.target);
        for o in m.sets.values(r.owner).ok_or("nonfinite owner axis")?.iter() {
            for s in m
                .sets
                .values(r.source)
                .ok_or("nonfinite source axis")?
                .iter()
            {
                let old = links
                    .entry((*o, *s, r.predicate, r.selector))
                    .or_insert_with(|| Some(BTreeSet::new()));
                match (&mut *old, &targets) {
                    (Some(old), Some(new)) => old.extend(new.iter().copied()),
                    (_, None) => *old = None,
                    _ => {}
                }
            }
        }
    }
    for ((o, s, _, label), target) in &links {
        for v in [*o, *s, *label] {
            dictionary.insert(name(v), v);
        }
        if let Some(values) = target {
            for v in values {
                dictionary.insert(name(*v), *v);
            }
        } else {
            dictionary.insert(name(core::Abs::ANY), core::Abs::ANY);
        }
    }
    let ids = dictionary
        .values()
        .enumerate()
        .map(|(i, v)| (*v, i))
        .collect::<BTreeMap<_, _>>();
    let mut file = stream(out, "front-values.tsv")?;
    for (i, (name, _)) in dictionary.iter().enumerate() {
        writeln!(file, "{i}\t{name}").map_err(|e| e.to_string())?;
    }
    file.flush().map_err(|e| e.to_string())?;
    let mut rows = Vec::new();
    for (f, values) in unary {
        let kind = ["noun", "lemma", "operator", "property", "payload", "tag"][f.kind as usize];
        let predicate = if f.kind == 4 {
            p.arena.name(f.predicate as usize)
        } else {
            ""
        };
        let mut values = values.iter().map(|v| ids[v]).collect::<Vec<_>>();
        values.sort();
        rows.push(format!(
            "{kind}\t{predicate}\t{}\t{}",
            ids[&f.selector],
            values
                .iter()
                .map(usize::to_string)
                .collect::<Vec<_>>()
                .join(",")
        ));
    }
    write_rows(out, "front-unary.tsv", rows)?;
    let mut rows = Vec::new();
    for ((o, s, pred, label), target) in links {
        let target = if let Some(values) = target {
            let mut values = values.iter().map(|v| ids[v]).collect::<Vec<_>>();
            values.sort();
            values
                .iter()
                .map(usize::to_string)
                .collect::<Vec<_>>()
                .join(",")
        } else {
            "all".into()
        };
        rows.push((
            ids[&o],
            ids[&s],
            p.arena.name(pred as usize).to_owned(),
            ids[&label],
            target,
        ));
    }
    rows.sort();
    let mut file = stream(out, "front-links.tsv")?;
    for (o, s, pred, label, target) in rows {
        writeln!(file, "{o}\t{s}\t{pred}\t{label}\t{target}").map_err(|e| e.to_string())?;
    }
    file.flush().map_err(|e| e.to_string())
}
fn clauses(p: &Program, g: &Csr, out: &Path) -> Result<Vec<usize>> {
    let mut nodes = (0..p.clauses.len()).collect::<Vec<_>>();
    nodes.sort_by_key(|i| {
        (
            &p.documents[p.clauses[*i].document - 1],
            p.clauses[*i].ordinal,
        )
    });
    let mut ids = vec![0; nodes.len()];
    for (i, n) in nodes.iter().enumerate() {
        ids[*n] = i;
    }
    let partition = g.partition();
    let mut components = BTreeMap::<usize, Vec<usize>>::new();
    let mut inventory = stream(out, "front-nodes.tsv")?;
    let mut edges = stream(out, "front-edges.tsv")?;
    for (i, original) in nodes.iter().enumerate() {
        let c = &p.clauses[*original];
        writeln!(
            inventory,
            "{i}\t{}\t{}\t{}",
            p.documents[c.document - 1],
            c.ordinal,
            p.patient[*original]
        )
        .map_err(|e| e.to_string())?;
        components.entry(partition[*original]).or_default().push(i);
        let mut row = g
            .row(*original)
            .iter()
            .map(|e| (ids[(e >> 1) as usize], e & 1 != 0))
            .collect::<Vec<_>>();
        row.sort();
        for (to, negative) in row {
            writeln!(edges, "{i}\t{to}\t{negative}\tfalse").map_err(|e| e.to_string())?;
        }
    }
    inventory
        .flush()
        .and_then(|_| edges.flush())
        .map_err(|e| e.to_string())?;
    let mut components = components.into_values().collect::<Vec<_>>();
    components.sort();
    let mut file = stream(out, "front-scc.tsv")?;
    for c in components {
        writeln!(
            file,
            "{}",
            c.iter().map(usize::to_string).collect::<Vec<_>>().join(",")
        )
        .map_err(|e| e.to_string())?;
    }
    file.flush().map_err(|e| e.to_string())?;
    Ok(ids)
}
fn flow(
    out: &Path,
    name: &str,
    g: &Csr,
    labels: &[String],
    keep: impl Fn(usize) -> bool,
) -> Result<usize> {
    write_rows(
        out,
        &format!("flow-{name}-nodes.tsv"),
        labels.iter().skip(1).cloned().collect(),
    )?;
    let mut rows = Vec::new();
    let mut scalar = Vec::new();
    for from in 1..g.nodes() {
        for e in g.row(from) {
            let to = (e >> 1) as usize;
            if to == 0 {
                scalar.push(format!("{}\tscalar", labels[from]));
            } else if keep(from) {
                rows.push(format!("{}\t{}", labels[from], labels[to]));
            }
        }
    }
    let count = rows.len();
    write_rows(out, &format!("flow-{name}-edges.tsv"), rows)?;
    // Additional scalar-leaf checks stay visible but are outside the census's
    // origin/output-node universe. They can never hide a non-scalar edge.
    scalar.sort();
    scalar.dedup();
    save(
        out,
        &format!("flow-{name}-scalar.tsv"),
        &scalar.iter().map(|s| s.clone() + "\n").collect::<String>(),
    )?;
    Ok(count)
}
pub fn dump(c: &Checked, d: &Diagnostics, out: &Path, graph_only: bool) -> Result<()> {
    fs::create_dir_all(out).map_err(|e| e.to_string())?;
    if !graph_only {
        features(&c.program, &c.model, out)?;
    }
    clauses(&c.program, &c.graph, out)?;
    let labels = (0..d.flow.types.nodes.len())
        .map(|i| d.flow.types.label(&c.program, &c.model, i))
        .collect::<Vec<_>>();
    flow(out, "base", &d.base_graph, &labels, |_| true)?;
    let mut outcome =
        "mode\tadmitted\tfull_refinement\ttypes\tvariants\trefined_templates\tprojection_checks\n"
            .to_owned();
    if let Some((r, g)) = &d.refined {
        let base_components = d.base_graph.partition();
        let mut cyclic = BTreeSet::new();
        for from in 0..d.base_graph.nodes() {
            for e in d.base_graph.row(from) {
                if base_components[from] == base_components[(e >> 1) as usize] {
                    cyclic.insert(base_components[from]);
                }
            }
        }
        let labels = (0..r.flow.types.nodes.len())
            .map(|i| {
                r.registry
                    .node_label(&c.program, &c.model, &r.flow.types, i)
                    .map_err(|e| format!("{e:?}"))
            })
            .collect::<Result<Vec<_>>>()?;
        let keep = |from| {
            if r.registry.full_refinement {
                return true;
            }
            let base = match r.flow.types.nodes[from] {
                TypeNode::Scalar => 0,
                TypeNode::Identity(o) => d
                    .flow
                    .types
                    .identity(r.registry.keys[r.registry.origin_keys[o]].origin)
                    as usize,
                TypeNode::Arithmetic { clause, goal } => d
                    .flow
                    .types
                    .arithmetic(r.registry.variants[clause].original.unwrap(), goal)
                    .unwrap() as usize,
            };
            cyclic.contains(&base_components[base])
        };
        let projected = flow(out, "final", g, &labels, keep)?;
        let full = flow(out, "final-full", g, &labels, |_| true)?;
        save(
            out,
            "flow-complement.tsv",
            &format!(
                "projected\t{projected}\nfull\t{full}\ncomplement\t{}\n",
                full - projected
            ),
        )?;
        outcome.push_str(&format!(
            "k1\ttrue\t{}\t{}\t{}\t{}\t{projected}\n",
            r.registry.full_refinement,
            r.registry.keys.len(),
            r.registry
                .variants
                .iter()
                .filter(|v| v.original.is_some())
                .count(),
            r.registry.precise.iter().filter(|v| **v).count()
        ));
    } else {
        flow(out, "final", &d.base_graph, &labels, |_| true)?;
        let full = flow(out, "final-full", &d.base_graph, &labels, |_| true)?;
        save(
            out,
            "flow-complement.tsv",
            &format!("projected\t{full}\nfull\t{full}\ncomplement\t0\n"),
        )?;
        outcome.push_str("base\ttrue\tfalse\t0\t0\t0\t0\n");
    }
    save(out, "flow-outcome.tsv", &outcome)
}
pub fn reference_slice(c: &Checked, out: &Path, roots: &[usize]) -> Result<Vec<usize>> {
    let mut nodes = (0..c.program.clauses.len()).collect::<Vec<_>>();
    nodes.sort_by_key(|i| {
        (
            &c.program.documents[c.program.clauses[*i].document - 1],
            c.program.clauses[*i].ordinal,
        )
    });
    let mut ids = vec![0; nodes.len()];
    for (i, n) in nodes.iter().enumerate() {
        ids[*n] = i;
    }
    let mut adj = vec![Vec::new(); nodes.len()];
    for row in
        BufReader::new(File::open(out.join("front-edges.tsv")).map_err(|e| e.to_string())?).lines()
    {
        let row = row.map_err(|e| e.to_string())?;
        let fields = row.split('\t').collect::<Vec<_>>();
        let from = fields[0].parse::<usize>().map_err(|e| e.to_string())?;
        let to = fields[1].parse::<usize>().map_err(|e| e.to_string())?;
        adj[from].push(to);
    }
    let mut chosen = BTreeSet::new();
    let mut queue = VecDeque::new();
    for root in roots {
        queue.push_back(ids[*root]);
    }
    while let Some(node) = queue.pop_front() {
        if chosen.insert(node) {
            queue.extend(&adj[node]);
        }
    }
    let documents = chosen
        .iter()
        .map(|i| c.program.clauses[nodes[*i]].document)
        .collect::<BTreeSet<_>>();
    for (i, clause) in c.program.clauses.iter().enumerate() {
        if documents.contains(&clause.document) && c.program.metadata(i) {
            chosen.insert(ids[i]);
        }
    }
    let mut selected = chosen
        .into_iter()
        .map(|i| nodes[i])
        .filter(|i| !c.program.patient[*i])
        .collect::<Vec<_>>();
    selected.sort();
    Ok(selected)
}
