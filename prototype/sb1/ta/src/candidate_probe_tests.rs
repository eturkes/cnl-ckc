use super::*;
use crate::exact_sets::{ALL, EMPTY};
use crate::features::Pattern;
use sb1_checker::term::Node;

#[test]
fn repeat_keys_preserve_symbols_owner_roles_aliases_and_naf_context() {
    let mut p = Program::new();
    p.add(
        "fixture".into(),
        "guideline_arg(actual,p,1,q).\n".into(),
        false,
    )
    .unwrap();
    let a = p.arena.atom("actual");
    let b = p.arena.atom("other");
    let x = p.arena.put(Node::Var(0));
    let y = p.arena.put(Node::Var(1));
    let one = p.arena.put(Node::Integer("1".into()));
    let two = p.arena.put(Node::Integer("2".into()));
    let g = p.arena.compound("guideline_arg", vec![a, x, one, y]);
    let owner = p.arena.compound("guideline_arg", vec![b, x, one, y]);
    let role = p.arena.compound("guideline_arg", vec![a, x, two, y]);
    let constant = p.arena.compound("guideline_arg", vec![a, b, one, y]);
    let symbol = p.arena.compound("guideline_pp", vec![a, x, one, y]);
    let m = Model::build(&p, None, true, None, true);
    let pat = Pattern::new(&p.arena, &m.abs, vec![g], 2);
    let changed = Pattern::new(&p.arena, &m.abs, vec![g, symbol], 2);
    let state = State::new(2);
    let context = QueryContext {
        phase: "graph",
        caller: 0,
        pattern: &pat,
        naf: 0,
    };
    let original = Trace::key(&p, &m, g, &state, &context);
    assert_eq!(original, Trace::key(&p, &m, g, &state, &context));
    for goal in [owner, role, constant, symbol] {
        assert_ne!(original.0, Trace::key(&p, &m, goal, &state, &context).0);
    }
    let mut alias = state.clone();
    alias.parent[1] = 0;
    assert_ne!(original.0, Trace::key(&p, &m, g, &alias, &context).0);
    let negative = QueryContext { naf: 1, ..context };
    assert_ne!(original.0, Trace::key(&p, &m, g, &state, &negative).0);
    let different_body = QueryContext {
        phase: "graph",
        caller: 0,
        pattern: &changed,
        naf: 0,
    };
    let other = Trace::key(&p, &m, g, &state, &different_body);
    assert_eq!(
        original.0, other.0,
        "equal index arguments do not imply equal survivor context"
    );
    assert_ne!(original.1, other.1);
    m.stats.borrow_mut().collections += 1;
    assert_ne!(
        original.0,
        Trace::key(&p, &m, g, &state, &context).0,
        "set-id reuse after collection must miss"
    );
    let mut first = HashMap::new();
    assert_eq!(Trace::hit(&mut first, original.0.clone(), Some(0)).1, "new");
    assert_eq!(
        Trace::hit(&mut first, original.0.clone(), Some(0)).1,
        "same"
    );
    assert_eq!(
        Trace::hit(&mut first, original.0.clone(), Some(1)).1,
        "cross"
    );
    let mut other_index = HashMap::new();
    assert_eq!(
        Trace::hit(&mut other_index, original.0, Some(1)).1,
        "new",
        "a changed producer-head dataset owns a separate cache"
    );
}

#[test]
fn composite_replay_is_native_intersection_on_any_numeric_patient_and_alias_domains() {
    let mut p = Program::new();
    p.add("fixture".into(), concat!(
        "guideline_arg(actual,p,1,q).\n",
        "guideline_arg(actual,7,2,p).\n",
        "guideline_arg(other,rational(1,2),1,q).\n",
        "guideline_arg(actual,A,1,B) :- guideline_entity(actual,A,person,countable), guideline_entity(actual,B,thing,countable).\n"
    ).into(), false).unwrap();
    let vars = (0..4)
        .map(|v| p.arena.put(Node::Var(v)))
        .collect::<Vec<_>>();
    let goal = p.arena.compound("guideline_arg", vars);
    let m = Model::build(&p, None, true, None, true);
    let index = crate::exact_heads::Index::new(&p, &m, false);
    let predicate = p.arena.call(goal).unwrap().0;
    let heads = &index.predicates[&predicate];
    let atom = p
        .arena
        .nodes
        .iter()
        .enumerate()
        .find_map(|(i, _)| (p.arena.atom_name(i) == Some("p")).then_some(i))
        .unwrap();
    let integer = p
        .arena
        .nodes
        .iter()
        .enumerate()
        .find_map(|(i, _)| matches!(p.arena.node(i), Node::Integer(s) if s=="1").then_some(i))
        .unwrap();
    let options = [
        ALL,
        EMPTY,
        m.sets.raw_one(Value::Any),
        m.sets.one(Value::Patient),
        m.sets.one(Value::Number),
        m.sets.one(Value::Exact(atom)),
        m.sets.one(Value::Exact(integer)),
        m.sets.intern(vec![Value::Patient, Value::Any]),
    ];
    let args = (0..4).map(Expr::Var).collect::<Vec<_>>();
    let mut rng = 0x72b3_d6a1u64;
    for _ in 0..512 {
        let mut state = State::new(4);
        for v in 0..4 {
            rng ^= rng << 13;
            rng ^= rng >> 7;
            rng ^= rng << 17;
            state.domains[v] = options[rng as usize % options.len()];
            state.parent[v] = rng as usize % (v + 1);
        }
        let native = heads.native(&args, &state, &m.sets);
        for plan in plans(heads, &args, &state, &m) {
            for at in 0..4 {
                let composite = Composite::build(&p, &m, heads, plan.at, at);
                let arg = Argument::new(state.domain(args[at], &m.sets), &m.sets);
                let (rows, _) = composite.matching(&plan, &arg, &p);
                let actual = merged(&rows);
                let selected = merged(&plan.buckets.iter().map(|(_, r)| *r).collect::<Vec<_>>());
                let expected = selected
                    .into_iter()
                    .filter(|i| {
                        let (_, head) = p.arena.call(p.clauses[*i].head).unwrap();
                        let value = match p.arena.node(head[at]) {
                            Node::Var(_) => Value::Any,
                            _ => m.abs.value(head[at]),
                        };
                        arg.accepts(value, &p.arena)
                    })
                    .collect::<Vec<_>>();
                assert_eq!(actual, expected);
                if Some(plan.at)
                    == plans(heads, &args, &state, &m)
                        .iter()
                        .min_by_key(|p| p.size)
                        .map(|p| p.at)
                {
                    assert!(actual.iter().all(|i| native.binary_search(i).is_ok()));
                }
            }
        }
    }
}
