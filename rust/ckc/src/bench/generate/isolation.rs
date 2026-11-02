// Finite may-model: numbers collapse, identities keep their owner + kind,
// Skolem dependencies collapse. NAF + built-ins over-approximate success.
// A surviving foreign provenance is a refusal, never a union-oracle approval.
use super::text::{Clause, Document, Goal, Term};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Value {
    Atom(String),
    Number,
    Identity(String, String),
    Opaque(String),
    Any,
}
type Env = BTreeMap<String, Value>;
type Provenance = BTreeSet<usize>;
type Rows = BTreeMap<Env, Provenance>;
type Relation = BTreeMap<Vec<Value>, Provenance>;
type Model = BTreeMap<String, Relation>;
struct Abstraction {
    patient_docids: BTreeSet<String>,
}
impl Abstraction {
    fn value(&self, term: &Term) -> Value {
        match term {
            Term::Atom(_, a) => Value::Atom(a.clone()),
            Term::Number(_) => Value::Number,
            Term::Var(_) => Value::Any,
            Term::Compound(_, name, aa) if name == "$guideline_id" && aa.len() == 5 => {
                let doc = aa[1].atom().unwrap_or("");
                let owner = if self.patient_docids.contains(doc) {
                    format!("patient:{doc}")
                } else if doc.starts_with("sb") && doc.as_bytes().get(6) == Some(&b'-') {
                    doc[..6].to_owned()
                } else {
                    "base".into()
                };
                Value::Identity(owner, aa[0].atom().unwrap_or("").to_owned())
            }
            Term::Compound(_, name, _) if name == "rational" => Value::Number,
            Term::Compound(_, name, _) => Value::Opaque(name.clone()),
            Term::List(_, _) => Value::Opaque("[]".into()),
        }
    }
    fn instantiate(&self, t: &Term, env: &Env) -> Value {
        match t {
            Term::Var(v) => env.get(v).cloned().unwrap_or(Value::Any),
            _ => self.value(t),
        }
    }
    fn matches(&self, term: &Term, value: &Value, env: &mut Env) -> bool {
        if let Term::Var(v) = term {
            match env.get(v) {
                Some(old) if *old != Value::Any => value == &Value::Any || old == value,
                _ => {
                    env.insert(v.clone(), value.clone());
                    true
                }
            }
        } else {
            let expected = self.value(term);
            expected == *value || *value == Value::Any
        }
    }
}
fn builtin(name: &str) -> bool {
    matches!(
        name,
        "guideline_satisfies" | "guideline_product" | "guideline_quotient"
    )
}
fn join(term: &Term, rows: Rows, model: &Model, abs: &Abstraction) -> Rows {
    let Some((name, args)) = term.call() else {
        return Rows::new();
    };
    let mut out = Rows::new();
    if builtin(name) {
        for (mut env, p) in rows {
            for a in args {
                let mut vars = BTreeMap::new();
                a.variables(&mut vars);
                for v in vars.keys() {
                    env.entry(v.clone()).or_insert(Value::Any);
                }
            }
            out.entry(env).or_default().extend(p);
        }
        return out;
    }
    let Some(relation) = model.get(name) else {
        return out;
    };
    for (env, provenance) in rows {
        for (tuple, writers) in relation {
            if tuple.len() != args.len() {
                continue;
            }
            let mut next = env.clone();
            if args
                .iter()
                .zip(tuple)
                .all(|(a, v)| abs.matches(a, v, &mut next))
            {
                let p = out.entry(next).or_default();
                p.extend(&provenance);
                p.extend(writers);
            }
        }
    }
    out
}
fn positive_rows(clause: &Clause, model: &Model, abs: &Abstraction) -> Rows {
    let mut rows = Rows::from([(Env::new(), Provenance::new())]);
    for g in &clause.body {
        if let Goal::Positive(t) = g {
            rows = join(t, rows, model, abs);
            if rows.is_empty() {
                break;
            }
        }
    }
    rows
}
fn foreign(provenance: &Provenance, owner: usize) -> bool {
    provenance.iter().any(|p| *p != owner)
}
fn insert(
    model: &mut Model,
    head: &Term,
    env: &Env,
    owner: Option<usize>,
    abs: &Abstraction,
) -> Result<bool, String> {
    let (name, args) = head.call().ok_or("isolation non-predicate head")?;
    let tuple = args.iter().map(|t| abs.instantiate(t, env)).collect();
    let relation = model.entry(name.to_owned()).or_default();
    let new = !relation.contains_key(&tuple);
    let writers = relation.entry(tuple).or_default();
    Ok(owner.is_some_and(|o| writers.insert(o)) || new)
}
fn nested_body_variables(term: &Term) -> bool {
    let Some((_, args)) = term.call() else {
        return true;
    };
    args.iter().any(|a| {
        !matches!(a, Term::Var(_)) && {
            let mut vs = BTreeMap::new();
            a.variables(&mut vs);
            !vs.is_empty()
        }
    })
}
pub fn check(shards: &[(usize, Vec<Document>)], patient: &[Document]) -> Result<(), String> {
    let patient_docids = patient
        .iter()
        .flat_map(Document::clauses)
        .filter_map(|c| c.head.call())
        .filter(|(n, _)| *n == "guideline_document")
        .filter_map(|(_, a)| a.first().and_then(Term::atom))
        .map(str::to_owned)
        .collect();
    let abs = Abstraction { patient_docids };
    let mut model = Model::new();
    let mut rules = Vec::new();
    for document in patient {
        for c in document.clauses() {
            if !c.body.is_empty() {
                return Err("isolation patient projection contains a rule".into());
            }
            insert(&mut model, &c.head, &Env::new(), None, &abs)?;
        }
    }
    for (owner, documents) in shards {
        for d in documents {
            for c in d.clauses() {
                let (name, _) = c.head.call().ok_or("isolation head shape")?;
                if name == "guideline_schema_version" {
                    insert(&mut model, &c.head, &Env::new(), None, &abs)?;
                    continue;
                }
                for g in &c.body {
                    for t in match g {
                        Goal::Positive(t) => std::slice::from_ref(t),
                        Goal::Naf(ts) => ts,
                    } {
                        if nested_body_variables(t) {
                            return Err(
                                "isolation nested body variables require a stronger abstraction"
                                    .into(),
                            );
                        }
                    }
                }
                if c.body.is_empty() {
                    insert(&mut model, &c.head, &Env::new(), Some(*owner), &abs)?;
                } else {
                    rules.push((*owner, c));
                }
            }
        }
    }
    loop {
        let mut changed = false;
        for (owner, c) in &rules {
            let rows = positive_rows(c, &model, &abs);
            for (env, provenance) in &rows {
                if foreign(provenance, *owner) {
                    return Err(format!(
                        "isolation positive producer→consumer path into copy {owner}: {}",
                        c.render(true).trim()
                    ));
                }
                for g in &c.body {
                    if let Goal::Naf(terms) = g {
                        let mut negatives = Rows::from([(env.clone(), Provenance::new())]);
                        for t in terms {
                            negatives = join(t, negatives, &model, &abs);
                        }
                        if negatives.values().any(|p| foreign(p, *owner)) {
                            return Err(format!(
                                "isolation NAF producer→consumer path into copy {owner}: {}",
                                c.render(true).trim()
                            ));
                        }
                    }
                }
                changed |= insert(&mut model, &c.head, env, Some(*owner), &abs)?;
            }
        }
        if !changed {
            break;
        }
    }
    Ok(())
}
