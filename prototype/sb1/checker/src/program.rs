use crate::number::Rational;
use crate::term::{Arena, Id, Node, Parser};
use std::collections::BTreeSet;

pub const PREDICATES: [(&str, usize); 22] = [
    ("guideline_schema_version", 1),
    ("guideline_document", 4),
    ("guideline_entity", 4),
    ("guideline_cardinality", 5),
    ("guideline_event", 3),
    ("guideline_arg", 4),
    ("guideline_pp", 4),
    ("guideline_property", 4),
    ("guideline_operator", 3),
    ("guideline_interval", 6),
    ("guideline_recurrence", 4),
    ("guideline_frequency", 5),
    ("guideline_order", 4),
    ("guideline_recurrence_window", 7),
    ("guideline_range", 3),
    ("guideline_grade", 5),
    ("guideline_preference", 4),
    ("guideline_qualifier", 4),
    ("guideline_unit", 3),
    ("guideline_exclusion", 1),
    ("guideline_supersession", 4),
    ("guideline_evaluation_date", 1),
];

#[derive(Clone, Debug)]
pub enum Goal {
    Positive(Id),
    Builtin(Id),
    Not(Vec<Goal>),
}

impl Goal {
    pub fn vars(&self, arena: &Arena) -> BTreeSet<usize> {
        let mut out = BTreeSet::new();
        match self {
            Self::Positive(term) | Self::Builtin(term) => arena.vars(*term, &mut out),
            Self::Not(goals) => {
                for g in goals {
                    out.extend(g.vars(arena));
                }
            }
        }
        out
    }
}

pub struct Clause {
    pub document: usize,
    pub ordinal: usize,
    pub head: Id,
    pub body: Vec<Goal>,
    pub names: Vec<String>,
    pub nonlocals: Vec<usize>,
}

pub fn builtin(arena: &Arena, term: Id) -> bool {
    arena.call(term).is_some_and(|(name, args)| {
        matches!(
            (arena.name(name), args.len()),
            ("guideline_satisfies", 4 | 6) | ("guideline_product" | "guideline_quotient", 6)
        )
    })
}

pub fn goals_of(arena: &Arena, term: Id) -> Result<Vec<Goal>, String> {
    if let Some(args) = arena.named(term, ",", 2) {
        let mut out = goals_of(arena, args[0])?;
        out.extend(goals_of(arena, args[1])?);
        Ok(out)
    } else if let Some(args) = arena.named(term, "\\+", 1) {
        Ok(vec![Goal::Not(goals_of(arena, args[0])?)])
    } else if arena.call(term).is_some() {
        Ok(vec![if builtin(arena, term) {
            Goal::Builtin(term)
        } else {
            Goal::Positive(term)
        }])
    } else {
        Err("goal is not a predicate call".into())
    }
}

fn goal(parser: &mut Parser<'_>, arena: &mut Arena) -> Result<Vec<Goal>, String> {
    if parser.eat("\\+") {
        let goals = if parser.eat("(") {
            let out = body(parser, arena)?;
            parser.require(")")?;
            out
        } else {
            goal(parser, arena)?
        };
        Ok(vec![Goal::Not(goals)])
    } else if parser.eat("(") {
        let out = body(parser, arena)?;
        parser.require(")")?;
        Ok(out)
    } else {
        let term = parser.term(arena)?;
        goals_of(arena, term)
    }
}

fn body(parser: &mut Parser<'_>, arena: &mut Arena) -> Result<Vec<Goal>, String> {
    let mut out = goal(parser, arena)?;
    while parser.eat(",") {
        out.extend(goal(parser, arena)?);
    }
    Ok(out)
}

pub fn parse_clauses(
    arena: &mut Arena,
    text: &str,
    document: usize,
) -> Result<Vec<Clause>, String> {
    let mut clauses = Vec::new();
    for (line_no, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('%') || line.starts_with(":-") {
            continue;
        }
        let mut p = Parser::new(line);
        let head = p
            .term(arena)
            .map_err(|e| format!("line {} {e}", line_no + 1))?;
        let body = if p.eat(":-") {
            body(&mut p, arena)?
        } else {
            Vec::new()
        };
        p.require(".")?;
        if !p.done() {
            return Err(p.error("trailing clause bytes"));
        }
        let mut nonlocals = BTreeSet::new();
        arena.vars(head, &mut nonlocals);
        let sets = body.iter().map(|g| g.vars(arena)).collect::<Vec<_>>();
        for (i, g) in body.iter().enumerate() {
            match g {
                Goal::Not(_) => {
                    let mut outside = BTreeSet::new();
                    arena.vars(head, &mut outside);
                    for (j, vs) in sets.iter().enumerate() {
                        if i != j {
                            outside.extend(vs);
                        }
                    }
                    nonlocals.extend(sets[i].intersection(&outside));
                }
                _ => nonlocals.extend(&sets[i]),
            }
        }
        clauses.push(Clause {
            document,
            ordinal: clauses.len() + 1,
            head,
            body,
            names: p.vars,
            nonlocals: nonlocals.into_iter().collect(),
        });
    }
    Ok(clauses)
}

fn source_term(arena: &Arena, id: Id, fact: bool) -> bool {
    match arena.node(id) {
        Node::Atom(_) | Node::Integer(_) => true,
        Node::Var(_) => !fact,
        Node::Compound(name, args) if arena.name(*name) == "rational" && args.len() == 2 => {
            Rational::from_term(arena, id).is_some()
        }
        Node::Compound(name, args) if arena.name(*name) == "$guideline_id" && args.len() == 5 => {
            arena.atom_name(args[0]).is_some()
                && arena.atom_name(args[1]).is_some()
                && matches!(arena.node(args[2]), Node::Integer(_))
                && slot(arena, args[3])
                && arena.list(args[4]).is_some_and(|deps| {
                    if fact {
                        deps.is_empty()
                    } else {
                        deps.iter().all(|d| matches!(arena.node(*d), Node::Var(_)))
                    }
                })
        }
        _ => false,
    }
}

fn slot(arena: &Arena, id: Id) -> bool {
    arena.call(id).is_some_and(|(name, args)| {
        matches!(arena.name(name), "ref" | "box")
            && args.len() == 1
            && matches!(arena.node(args[0]), Node::Integer(_))
    })
}

pub fn ground_term(arena: &Arena, id: Id) -> bool {
    match arena.node(id) {
        Node::Atom(_) | Node::Integer(_) => true,
        Node::Compound(name, args) if arena.name(*name) == "rational" && args.len() == 2 => {
            Rational::from_term(arena, id).is_some()
        }
        Node::Compound(name, args) if arena.name(*name) == "$guideline_id" && args.len() == 5 => {
            arena.atom_name(args[0]).is_some()
                && arena.atom_name(args[1]).is_some()
                && matches!(arena.node(args[2]), Node::Integer(_))
                && slot(arena, args[3])
                && arena
                    .list(args[4])
                    .is_some_and(|deps| deps.into_iter().all(|d| ground_term(arena, d)))
        }
        _ => false,
    }
}

fn metadata(arena: &Arena, term: Id) -> bool {
    fn atom_wrapper(arena: &Arena, id: Id, name: &str) -> bool {
        arena
            .named(id, name, 1)
            .is_some_and(|args| arena.atom_name(args[0]).is_some())
    }
    if let Some(args) = arena.named(term, "guideline_document", 4) {
        arena.atom_name(args[0]).is_some()
            && atom_wrapper(arena, args[1], "ace_sha256")
            && [(args[2], "ulex"), (args[3], "vocabulary")]
                .into_iter()
                .all(|(id, name)| {
                    arena.named(id, name, 1).is_some_and(|a| {
                        arena.atom_name(a[0]) == Some("none") || atom_wrapper(arena, a[0], "sha256")
                    })
                })
    } else if let Some(args) = arena.named(term, "guideline_supersession", 4) {
        arena.atom_name(args[0]).is_some()
            && matches!(arena.node(args[1]), Node::Integer(_))
            && arena
                .named(args[2], "source", 2)
                .is_some_and(|a| a.iter().all(|id| arena.atom_name(*id).is_some()))
            && matches!(arena.node(args[3]), Node::Integer(_))
    } else {
        false
    }
}

pub fn model_fact(arena: &Arena, term: Id) -> bool {
    let Some((name, args)) = arena.call(term) else {
        return false;
    };
    let name = arena.name(name);
    if !PREDICATES.contains(&(name, args.len())) {
        return false;
    }
    if matches!(name, "guideline_document" | "guideline_supersession") {
        metadata(arena, term)
    } else {
        args.iter().all(|id| ground_term(arena, *id))
    }
}

fn literal_shape(arena: &Arena, term: Id, is_fact: bool, is_builtin: bool) -> bool {
    let Some((name, args)) = arena.call(term) else {
        return false;
    };
    if is_builtin {
        return builtin(arena, term) && args.iter().all(|id| source_term(arena, *id, false));
    }
    let name = arena.name(name);
    if !PREDICATES.contains(&(name, args.len())) {
        return false;
    }
    if matches!(name, "guideline_document" | "guideline_supersession") {
        is_fact && metadata(arena, term)
    } else {
        args.iter().all(|id| source_term(arena, *id, is_fact))
    }
}

fn bound(arena: &Arena, term: Id, bound: &BTreeSet<usize>) -> bool {
    let mut vars = BTreeSet::new();
    arena.vars(term, &mut vars);
    vars.is_subset(bound)
}

fn safe_goals(
    arena: &Arena,
    goals: &[Goal],
    bound_vars: &mut BTreeSet<usize>,
    nested: bool,
) -> Result<(), String> {
    for g in goals {
        match g {
            Goal::Positive(term) => {
                if !literal_shape(arena, *term, false, false) {
                    return Err("predicate/term shape".into());
                }
                bound_vars.extend(g.vars(arena));
            }
            Goal::Builtin(term) => {
                if !literal_shape(arena, *term, false, true) {
                    return Err("built-in shape".into());
                }
                let (name, args) = arena.call(*term).unwrap();
                let inputs = if arena.name(name) == "guideline_satisfies" {
                    args.len()
                } else {
                    4
                };
                if !args[..inputs]
                    .iter()
                    .all(|id| bound(arena, *id, bound_vars))
                {
                    return Err("unsafe built-in input".into());
                }
                for arg in &args[inputs..] {
                    arena.vars(*arg, bound_vars);
                }
            }
            Goal::Not(inner) => {
                if nested || inner.is_empty() {
                    return Err("negation shape".into());
                }
                let mut inner_bound = bound_vars.clone();
                safe_goals(arena, inner, &mut inner_bound, true)?;
            }
        }
    }
    Ok(())
}

pub fn admit_query(arena: &Arena, goals: &[Goal], projection: &[Id]) -> Result<(), String> {
    let mut bound_vars = BTreeSet::new();
    for (i, g) in goals.iter().enumerate() {
        if let Goal::Not(inner) = g {
            let mut outside = BTreeSet::new();
            for &term in projection {
                arena.vars(term, &mut outside);
            }
            for (j, other) in goals.iter().enumerate() {
                if i != j {
                    outside.extend(other.vars(arena));
                }
            }
            if !g
                .vars(arena)
                .intersection(&outside)
                .all(|v| bound_vars.contains(v))
            {
                return Err("unsafe query negation".into());
            }
            safe_goals(arena, inner, &mut bound_vars.clone(), true)?;
        } else {
            safe_goals(arena, std::slice::from_ref(g), &mut bound_vars, false)?;
        }
    }
    if !projection
        .iter()
        .all(|term| bound(arena, *term, &bound_vars))
    {
        return Err("unsafe query projection".into());
    }
    Ok(())
}

pub fn admit(arena: &Arena, clauses: &[Clause]) -> Result<(), String> {
    for clause in clauses {
        let result: Result<(), String> = (|| {
            if !literal_shape(arena, clause.head, clause.body.is_empty(), false) {
                return Err("head shape".into());
            }
            let mut bound_vars = BTreeSet::new();
            for (i, g) in clause.body.iter().enumerate() {
                if let Goal::Not(inner) = g {
                    let mut outside = BTreeSet::new();
                    arena.vars(clause.head, &mut outside);
                    for (j, other) in clause.body.iter().enumerate() {
                        if i != j {
                            outside.extend(other.vars(arena));
                        }
                    }
                    if !g
                        .vars(arena)
                        .intersection(&outside)
                        .all(|v| bound_vars.contains(v))
                    {
                        return Err("unsafe negation outer variable".into());
                    }
                    let mut inner_bound = bound_vars.clone();
                    safe_goals(arena, inner, &mut inner_bound, true)?;
                } else {
                    safe_goals(arena, std::slice::from_ref(g), &mut bound_vars, false)?;
                }
            }
            if !bound(arena, clause.head, &bound_vars) {
                return Err("unsafe head variable".into());
            }
            Ok(())
        })();
        if let Err(e) = result {
            return Err(format!("{}:{} {e}", clause.document, clause.ordinal));
        }
    }
    Ok(())
}
