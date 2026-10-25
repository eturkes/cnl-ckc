// SUP1 over the independent CS1 projection. No compiler output enters this oracle.
use super::base::{self, Goal, Product, Term, a, c, parse, quote, split, v};
use super::binding_first;
use super::support::sha;
use std::collections::BTreeMap;

pub const DATE: u32 = 20221229;
pub const URL: &str = "https://authority.example/act%20one";
pub const CITATION: &str = "Public Law, § 12 — replacement";
pub const NEW: [(&str, usize); 2] = [
    ("guideline_supersession", 4),
    ("guideline_evaluation_date", 1),
];
pub fn atom(s: &str) -> String {
    if s.as_bytes().first().is_some_and(u8::is_ascii_lowercase)
        && s.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
    {
        s.into()
    } else {
        format!("'{}'", s.replace('\\', "\\\\").replace('\'', "\\'"))
    }
}
pub fn declarations() -> String {
    base::declarations()
}
pub fn without_new_block(text: &str) -> String {
    text.split_inclusive('\n')
        .filter(|l| {
            !NEW.iter()
                .any(|(n, a)| l.starts_with(":-") && l.contains(&format!("({n}/{a})")))
        })
        .collect()
}
#[derive(Clone, Debug)]
pub struct Scene {
    pub id: String,
    pub family: &'static str,
    pub ace: String,
    pub scope: bool,
    pub exclusion: bool,
    pub date: u32,
    pub url: String,
    pub citation: String,
}
impl Scene {
    pub fn new(id: &str, family: &'static str, ace: &str) -> Self {
        Self {
            id: format!("sup-{id}"),
            family,
            ace: format!("{}\n", ace.trim_end()),
            scope: false,
            exclusion: false,
            date: DATE,
            url: URL.into(),
            citation: CITATION.into(),
        }
    }
    pub fn table(&self, matched: bool) -> String {
        let mut out = base::table(self.scope.then_some(base::KEY));
        if matched {
            out.push_str(&format!(
                "supersession\t{}\t{:08} {} {}\n",
                self.id, self.date, self.url, self.citation
            ));
        }
        out
    }
}
pub fn scenes() -> Vec<Scene> {
    let mut out = vec![
        Scene::new(
            "should",
            "a",
            "If a patient sleeps then the patient should wait.",
        ),
        Scene::new(
            "must",
            "b",
            "If a patient sleeps then the patient must wait.",
        ),
        Scene::new("may", "b", "If a patient sleeps then the patient may wait."),
        Scene::new(
            "split",
            "c",
            "If a man sleeps or a woman waits then a patient should work.",
        ),
        Scene::new(
            "under-can",
            "d",
            "If a patient sleeps then it is possible that the patient should wait.",
        ),
        Scene::new(
            "under-negative",
            "d",
            "If a patient sleeps then it is false that the patient should wait.",
        ),
        Scene::new(
            "mixed",
            "e",
            "If a patient sleeps then the patient works and the patient should wait.",
        ),
        Scene::new(
            "can-only",
            "f",
            "If a patient sleeps then the patient can wait.",
        ),
        Scene::new(
            "asserted",
            "f",
            "If a patient sleeps then the patient waits.",
        ),
        Scene::new(
            "scoped",
            "g",
            "If a patient sleeps then the patient should wait.",
        ),
        Scene::new(
            "exclusion",
            "h",
            "If a patient sleeps then it is false that a recommendation applies to the patient.",
        ),
        Scene::new("fact", "i", "A man sleeps."),
        Scene::new("root-can", "i", "It is possible that a patient sleeps."),
        Scene::new(
            "bundles",
            "bundles",
            "A man sleeps.\nIf a patient sleeps then the patient should wait.\nIf a woman waits then the woman can work.",
        ),
        Scene::new(
            "naf",
            "fresh",
            "If a patient sleeps and the patient does not provably work then the patient should wait.",
        ),
        Scene::new(
            "curried",
            "fresh",
            "If a patient sleeps then if the patient works then the patient should wait.",
        ),
    ];
    for item in &mut out {
        item.scope = matches!(item.family, "g" | "h");
        item.exclusion = item.family == "h";
    }
    for (date, citation, url) in [
        (101, "2022", "https://a"),
        (229, "Year zero's act \\ annex", "https://a/\\'"),
        (20230101, "§ January 1", "https://a?q=x&v=%20"),
        (99991231, "Last year — Ω", "https://a/#last"),
    ] {
        let mut item = Scene::new(
            &format!("boundary-{date}"),
            "boundary",
            "If a patient sleeps then the patient should wait.",
        );
        item.date = date;
        item.citation = citation.into();
        item.url = url.into();
        out.push(item);
    }
    out
}
#[derive(Clone)]
pub struct Projection {
    pub scene: Scene,
    pub sentences: Vec<Product>,
}
fn deontic(g: &base::Group) -> bool {
    g.heads.iter().any(|h| {
        h.name() == "guideline_operator" && ["should", "must", "may"].contains(&h.args()[2].name())
    })
}
fn sentence_term(t: &Term, sentence: usize) -> Term {
    match t {
        Term::C(n, xs) => {
            let mut xs: Vec<_> = xs.iter().map(|x| sentence_term(x, sentence)).collect();
            if n == "$guideline_id" {
                xs[2] = a(&sentence.to_string());
            }
            c(n, xs)
        }
        Term::L(xs) => Term::L(xs.iter().map(|x| sentence_term(x, sentence)).collect()),
        _ => t.clone(),
    }
}
fn body(text: &str, sentence: usize) -> Vec<Goal> {
    split(text)
        .into_iter()
        .map(|part| {
            if let Some(inner) = part.strip_prefix("\\+ ") {
                let xs = if inner.starts_with('(') {
                    split(&inner[1..inner.len() - 1])
                } else {
                    vec![inner]
                };
                Goal::Naf(
                    xs.into_iter()
                        .map(|t| sentence_term(&parse(t), sentence))
                        .collect(),
                )
            } else {
                Goal::Pos(sentence_term(&parse(part), sentence))
            }
        })
        .collect()
}
fn clause(line: &str, sentence: usize, date: Option<u32>) -> String {
    let text = line.strip_suffix('.').unwrap();
    let (head, mut goals) = match text.split_once(" :- ") {
        Some((h, b)) => (sentence_term(&parse(h), sentence), body(b, sentence)),
        None => (sentence_term(&parse(text), sentence), vec![]),
    };
    if let Some(date) = date {
        assert!(!goals.is_empty(), "root deontic rejected before projection");
        let at = goals.len()
            - usize::from(
                matches!(goals.last(), Some(Goal::Naf(xs)) if xs.len() == 1 && xs[0].name() == "guideline_exclusion"),
            );
        goals.splice(
            at..at,
            [
                Goal::Pos(c("guideline_evaluation_date", vec![v("SUP1Date")])),
                Goal::Pos(c(
                    "guideline_satisfies",
                    vec![a("exactly"), v("SUP1Date"), a("less"), a(&date.to_string())],
                )),
            ],
        );
    }
    let mut vars = BTreeMap::new();
    let mut out = head.text(&mut vars);
    if !goals.is_empty() {
        out.push_str(" :- ");
        out.push_str(
            &goals
                .iter()
                .map(|g| g.text(&mut vars))
                .collect::<Vec<_>>()
                .join(", "),
        );
    }
    format!("{out}.\n")
}
impl Projection {
    pub fn document(&self, matched: bool, with_table: bool) -> String {
        let scene = &self.scene;
        let table = with_table.then(|| scene.table(matched));
        let tab = table
            .as_ref()
            .map_or_else(|| "none".into(), |t| format!("sha256({})", atom(&sha(t))));
        let mut out = format!(
            "% {}.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.\n{}guideline_schema_version(1).\nguideline_document({},ace_sha256({}),ulex(sha256({})),vocabulary({tab})).\n",
            scene.id,
            declarations(),
            atom(&scene.id),
            atom(&sha(&scene.ace)),
            atom(&sha(base::ULEX))
        );
        for (i, p) in self.sentences.iter().enumerate() {
            let sentence = i + 1;
            out.push_str(&format!("% S{sentence}: {}\n", p.ace.trim_end()));
            for g in &p.groups {
                let one = Product {
                    groups: vec![g.clone()],
                    ..p.clone()
                };
                let text = binding_first::document(&one.clauses(
                    (with_table && scene.scope).then_some(base::KEY),
                    scene.exclusion && with_table,
                ));
                for line in text.lines() {
                    out.push_str(&clause(
                        line,
                        sentence,
                        (matched && deontic(g)).then_some(scene.date),
                    ));
                }
            }
            if matched {
                out.push_str(&format!(
                    "guideline_supersession({},{sentence},source({},{}),{}).\n",
                    atom(&scene.id),
                    atom(&scene.citation),
                    atom(&scene.url),
                    scene.date
                ));
            }
        }
        out
    }
    pub fn proof(&self, matched: bool, with_table: bool) -> String {
        let mut out = String::new();
        for (i, p) in self.sentences.iter().enumerate() {
            for g in &p.groups {
                let one = Product {
                    groups: vec![g.clone()],
                    ..p.clone()
                };
                let text = binding_first::document(&one.proof(
                    (with_table && self.scene.scope).then_some(base::KEY),
                    self.scene.exclusion && with_table,
                ));
                let mut t = sentence_term(&parse(text.trim_end().trim_end_matches('.')), i + 1);
                let Term::C(_, ref mut fields) = t else {
                    panic!("payload")
                };
                fields[1] = a(&(i + 1).to_string());
                if matched && deontic(g) {
                    let Term::C(_, ref mut witness) = fields[3] else {
                        panic!("witness")
                    };
                    let Term::L(ref mut facts) = witness[0] else {
                        panic!("witness facts")
                    };
                    facts.push(c(
                        "guideline_evaluation_date",
                        vec![a(&(self.scene.date - 1).to_string())],
                    ));
                }
                out.push_str(&format!("{}.\n", t.render()));
            }
        }
        out
    }
    pub fn baseline_definition(&self) -> String {
        assert_eq!(self.sentences.len(), 1);
        binding_first::document(
            &self.sentences[0]
                .document(self.scene.scope.then_some(base::KEY), self.scene.exclusion),
        )
    }
}
pub fn query(id: &str, goal: &str) -> String {
    format!(
        "% {id} compiled from ACE question by ace_to_pl question mode; do not edit.\n'$guideline_query'(v1,{},ace_sha256({}),ulex(none),vocabulary(none)).\n% Q1: Is there a result?\n'$guideline_query_projection'(goal({}),answers([])).\n",
        atom(id),
        atom(&sha("Is there a result?\n")),
        parse(goal).render()
    )
}
pub fn answer(id: &str, query: &str, yes: bool) -> String {
    format!(
        "% {id} answered against the loaded composition by ckc queries; do not edit.\n'$guideline_answers'(v1,{},query_sha256({}),result({})).\n",
        atom(id),
        atom(&sha(query)),
        if yes { "yes" } else { "no(finite_failure)" }
    )
}
pub fn structural(id: &str, clauses: &[String]) -> String {
    format!(
        "% {id}.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.\n{}guideline_schema_version(1).\nguideline_document({},ace_sha256({}),ulex(none),vocabulary(none)).\n% S1: A composition fixture.\n{}",
        declarations(),
        atom(id),
        atom(&sha("A composition fixture.\n")),
        clauses
            .iter()
            .map(|l| format!("{l}.\n"))
            .collect::<String>()
    )
}
pub fn edge(doc: &str, op: &str, inner: usize, outer: Option<usize>) -> String {
    let ctx = |n| format!("'$guideline_id'(context,{},1,box({n}),D)", quote(doc));
    format!(
        "guideline_operator({},{},{op})",
        outer.map_or_else(|| "actual".into(), ctx),
        ctx(inner)
    )
}
