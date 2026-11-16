//! Independent finite semantics. No solver/checker/feature/rank implementation.
pub mod ground;
use std::collections::{BTreeSet, VecDeque};

pub type Model = BTreeSet<usize>;
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Rule {
    pub head: usize,
    pub positive: Model,
    pub naf: Vec<Vec<Model>>,
    pub builtins: bool,
}
#[derive(Clone, Debug)]
pub struct Program {
    pub atoms: usize,
    pub rules: Vec<Rule>,
}
impl Program {
    pub fn gamma(&self, reference: &Model) -> Model {
        self.least(reference, false)
    }
    pub fn envelope(&self) -> Model {
        self.least(&Model::new(), true)
    }
    fn least(&self, reference: &Model, ignore_naf: bool) -> Model {
        // Each reduct rule fires after its last distinct positive premise.
        let mut waiting = vec![Vec::new(); self.atoms];
        let mut missing = vec![0; self.rules.len()];
        let mut agenda = VecDeque::new();
        for (i, r) in self.rules.iter().enumerate() {
            assert!(r.head < self.atoms && r.positive.iter().all(|a| *a < self.atoms));
            if !r.builtins
                || (!ignore_naf
                    && r.naf
                        .iter()
                        .any(|item| item.iter().any(|w| w.is_subset(reference))))
            {
                continue;
            }
            missing[i] = r.positive.len();
            if r.positive.is_empty() {
                agenda.push_back(r.head);
            }
            for atom in &r.positive {
                waiting[*atom].push(i);
            }
        }
        let mut out = Model::new();
        while let Some(atom) = agenda.pop_front() {
            if !out.insert(atom) {
                continue;
            }
            for rule in &waiting[atom] {
                missing[*rule] -= 1;
                if missing[*rule] == 0 {
                    agenda.push_back(self.rules[*rule].head);
                }
            }
        }
        out
    }
    pub fn prune(&self, envelope: &Model) -> Self {
        let rules = self
            .rules
            .iter()
            .filter(|r| r.builtins && r.positive.is_subset(envelope))
            .map(|r| Rule {
                head: r.head,
                positive: r.positive.clone(),
                builtins: true,
                naf: r
                    .naf
                    .iter()
                    .map(|item| {
                        item.iter()
                            .filter(|w| w.is_subset(envelope))
                            .cloned()
                            .collect()
                    })
                    .collect(),
            })
            .collect();
        Self {
            atoms: self.atoms,
            rules,
        }
    }
    pub fn wfm(&self) -> (Model, Model) {
        let mut lower = Model::new();
        for _ in 0..=self.atoms {
            let upper = self.gamma(&lower);
            let next = self.gamma(&upper);
            assert!(lower.is_subset(&next));
            if next == lower {
                return (lower, upper);
            }
            lower = next;
        }
        panic!("finite alternating fixpoint failed to converge")
    }
    pub fn stable_models(&self) -> Vec<Model> {
        assert!(self.atoms < 20, "explicit stable enumeration domain");
        (0..1_u64 << self.atoms)
            .filter_map(|bits| {
                let m = (0..self.atoms)
                    .filter(|i| bits & (1 << i) != 0)
                    .collect::<Model>();
                (self.gamma(&m) == m).then_some(m)
            })
            .collect()
    }
}
pub fn set(items: &[usize]) -> Model {
    items.iter().copied().collect()
}

pub fn two_atom_rules() -> Vec<Rule> {
    let forms = vec![
        vec![],
        vec![vec![set(&[0])]],
        vec![vec![set(&[1])]],
        vec![vec![set(&[0, 1])]],
        vec![vec![set(&[0]), set(&[1])]],
        vec![vec![set(&[0])], vec![set(&[1])]],
    ];
    let mut rules = Vec::new();
    for head in 0..2 {
        for positive in [set(&[]), set(&[0]), set(&[1]), set(&[0, 1])] {
            for naf in &forms {
                rules.push(Rule {
                    head,
                    positive: positive.clone(),
                    naf: naf.clone(),
                    builtins: true,
                });
            }
        }
    }
    rules
}
