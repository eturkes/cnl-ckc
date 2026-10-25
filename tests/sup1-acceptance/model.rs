use super::support::*;

pub const DATE: i64 = 20221229;
pub const CITATION: &str = "Public Law 117-328";
pub const URL: &str = "https://example.org/law";
pub fn gid(kind: &str, id: &str, s: usize, slot: usize, deps: &str) -> String {
    format!(
        "'$guideline_id'({kind},{},{s},ref({slot}),{deps})",
        atom(id)
    )
}
pub fn context(id: &str, s: usize, slot: usize, deps: &str) -> String {
    format!(
        "'$guideline_id'(context,{},{s},box({slot}),{deps})",
        atom(id)
    )
}
pub fn entity(ctx: &str, r: &str, noun: &str) -> Vec<String> {
    vec![
        format!("guideline_entity({ctx},{r},{},countable)", atom(noun)),
        format!("guideline_cardinality({ctx},{r},na,eq,1)"),
    ]
}
pub fn event(ctx: &str, e: &str, verb: &str, subject: &str) -> Vec<String> {
    vec![
        format!("guideline_event({ctx},{e},{})", atom(verb)),
        format!("guideline_arg({ctx},{e},1,{subject})"),
    ]
}
pub fn date_goals(variable: &str, date: i64) -> [String; 2] {
    [
        format!("guideline_evaluation_date({variable})"),
        format!("guideline_satisfies(exactly,{variable},less,{date})"),
    ]
}
pub fn record(id: &str, s: usize, date: i64) -> String {
    format!(
        "guideline_supersession({},{s},source({},{}),{date})",
        atom(id),
        atom(CITATION),
        atom(URL)
    )
}
#[derive(Clone)]
pub struct Group {
    pub k: usize,
    pub body: Vec<String>,
    pub heads: Vec<String>,
}
impl Group {
    pub fn deontic(&self) -> bool {
        self.heads.iter().any(|h| {
            h.starts_with("guideline_operator(")
                && [",should)", ",must)", ",may)"]
                    .iter()
                    .any(|end| h.ends_with(end))
        })
    }
    pub fn clauses(&self, date: Option<i64>, scope: bool) -> Vec<String> {
        self.heads
            .iter()
            .map(|head| {
                let mut body = self.body.clone();
                if self.deontic()
                    && let Some(date) = date
                {
                    // Fresh relative to every model clause; canonical numbering is per line.
                    body.extend(date_goals("Z", date));
                }
                if scope
                    && head.starts_with("guideline_operator(")
                    && [",should)", ",must)", ",may)"]
                        .iter()
                        .any(|end| head.ends_with(end))
                {
                    body.push("\\+ guideline_exclusion(alpha)".into());
                }
                canonical(&if body.is_empty() {
                    head.clone()
                } else {
                    format!("{head} :- {}", body.join(", "))
                })
            })
            .collect()
    }
    pub fn payload(&self, id: &str, s: usize, date: Option<i64>) -> String {
        let bind = |text: &str| {
            substitute(text, |v| {
                gid(
                    "witness",
                    id,
                    s,
                    v as usize - 'A' as usize + 1,
                    &format!("variant({})", self.k),
                )
            })
        };
        let mut facts: Vec<_> = self
            .body
            .iter()
            .filter(|g| !g.starts_with("\\+ ") && !g.starts_with("guideline_satisfies("))
            .map(|g| bind(g))
            .collect();
        if self.deontic()
            && let Some(date) = date
        {
            facts.push(format!("guideline_evaluation_date({})", date - 1));
        }
        proof(
            id,
            s,
            self.k,
            &facts,
            &self.heads.iter().map(|h| bind(h)).collect::<Vec<_>>(),
        )
    }
}
#[derive(Clone)]
pub struct Scene {
    pub id: String,
    pub ace: String,
    pub groups: Vec<Vec<Group>>,
    pub date: Option<i64>,
    pub scope: bool,
}
impl Scene {
    pub fn table(&self) -> Option<String> {
        if self.date.is_none() && !self.scope {
            return None;
        }
        let mut table = "# format: kind\tlemma\tvalue\n".to_owned();
        if self.scope {
            table.push_str("scope\trecommendation\talpha\nexclusion\tapply\tscope\n");
        }
        if let Some(date) = self.date {
            table.push_str(&format!(
                "supersession\t{}\t{date:08} {URL} {CITATION}\n",
                self.id
            ));
        }
        Some(table)
    }
    pub fn bundles(&self) -> Vec<Vec<String>> {
        self.groups
            .iter()
            .enumerate()
            .map(|(i, groups)| {
                let mut clauses: Vec<_> = groups
                    .iter()
                    .flat_map(|g| g.clauses(self.date, self.scope))
                    .collect();
                if let Some(date) = self.date {
                    clauses.push(record(&self.id, i + 1, date));
                }
                clauses
            })
            .collect()
    }
    pub fn member(&self) -> Member {
        let bundles = self.bundles();
        Member {
            id: self.id.clone(),
            pl: document_text(&self.id, &self.ace, self.table().as_deref(), &bundles),
            payload: self
                .groups
                .iter()
                .enumerate()
                .flat_map(|(i, gs)| {
                    gs.iter()
                        .map(move |g| g.payload(&self.id, i + 1, self.date))
                })
                .collect(),
            clauses: bundles
                .into_iter()
                .enumerate()
                .flat_map(|(i, cs)| cs.into_iter().map(move |c| (i + 1, c)))
                .collect(),
        }
    }
}
fn group(body: Vec<String>, heads: Vec<String>) -> Group {
    Group { k: 1, body, heads }
}
pub fn recommendation(
    id: &str,
    ops: &[&str],
    wrapper: Option<&str>,
    date: Option<i64>,
    scope: bool,
) -> Scene {
    let mut heads = Vec::new();
    let mut source = Vec::new();
    let mut box_slot = 1;
    for (i, op) in ops.iter().enumerate() {
        let parent = if let Some(w) = wrapper {
            let parent = context(id, 1, box_slot, "[A]");
            heads.push(format!("guideline_operator(actual,{parent},{})", atom(w)));
            box_slot += 1;
            parent
        } else {
            "actual".into()
        };
        let ctx = context(id, 1, box_slot, "[A]");
        box_slot += 1;
        heads.push(format!("guideline_operator({parent},{ctx},{op})"));
        heads.extend(event(
            &ctx,
            &gid("product", id, 1, i + 2, "[A]"),
            "wait",
            "A",
        ));
        source.push(format!(
            "{}the patient {op} wait",
            match wrapper {
                Some("can") => "it is possible that ",
                Some("-") => "it is false that ",
                None => "",
                _ => panic!("wrapper"),
            }
        ));
    }
    Scene {
        id: id.into(),
        ace: format!("If there is a patient then {}.\n", source.join(" and ")),
        groups: vec![vec![group(entity("actual", "A", "patient"), heads)]],
        date,
        scope,
    }
}
pub fn split(id: &str) -> Scene {
    let mut s = recommendation(id, &["should"], None, Some(DATE), false);
    let template = s.groups[0][0].clone();
    s.groups[0] = [("B", "pain", "[A,B]"), ("C", "fever", "[A,C]")]
        .iter()
        .enumerate()
        .map(|(i, (r, noun, deps))| {
            let mut g = template.clone();
            g.k = i + 1;
            g.body.extend(entity("actual", r, noun));
            g.heads = g
                .heads
                .iter()
                .map(|h| h.replace("ref(2)", "ref(4)").replace("[A]", deps))
                .collect();
            g
        })
        .collect();
    s.ace = include_str!("split.ace.in").into();
    s
}
pub fn mixed(id: &str) -> Scene {
    let mut s = recommendation(id, &["should"], None, Some(DATE), false);
    let ctx = context(id, 1, 1, "[A]");
    let mut heads = event("actual", &gid("product", id, 1, 2, "[A]"), "wait", "A");
    heads.push(format!("guideline_operator(actual,{ctx},should)"));
    heads.extend(event(&ctx, &gid("product", id, 1, 3, "[A]"), "sleep", "A"));
    s.groups[0][0].heads = heads;
    s.ace = include_str!("mixed.ace.in").into();
    s
}
pub fn asserted(id: &str, date: Option<i64>) -> Scene {
    Scene {
        id: id.into(),
        ace: "If there is a patient then the patient waits.\n".into(),
        date,
        scope: false,
        groups: vec![vec![group(
            entity("actual", "A", "patient"),
            event("actual", &gid("product", id, 1, 2, "[A]"), "wait", "A"),
        )]],
    }
}
pub fn root_facts(id: &str, two: bool) -> Scene {
    let mut groups = Vec::new();
    let ace = if two {
        include_str!("two-sentences.ace.in")
    } else {
        include_str!("root-fact.ace.in")
    }
    .into();
    for (i, (noun, verb)) in [("cat", "wait"), ("dog", "sleep")]
        .iter()
        .take(if two { 2 } else { 1 })
        .enumerate()
    {
        let r = gid("product", id, i + 1, 1, "[]");
        let ev = gid("product", id, i + 1, 2, "[]");
        let mut heads = entity("actual", &r, noun);
        heads.extend(event("actual", &ev, verb, &r));
        groups.push(vec![group(Vec::new(), heads)]);
    }
    Scene {
        id: id.into(),
        ace,
        groups,
        date: Some(DATE),
        scope: false,
    }
}
pub fn exclusion_scene(id: &str) -> Scene {
    let ctx = context(id, 1, 1, "[A]");
    let r = gid("product", id, 1, 2, "[A]");
    let ev = gid("product", id, 1, 3, "[A]");
    let mut heads = vec![format!("guideline_operator(actual,{ctx},-)")];
    heads.extend(entity(&ctx, &r, "recommendation"));
    heads.extend(event(&ctx, &ev, "apply", &r));
    heads.push(format!("guideline_pp({ctx},{ev},to,A)"));
    heads.push("guideline_exclusion(alpha)".into());
    Scene {
        id: id.into(),
        ace: include_str!("excluded.ace.in").into(),
        date: Some(DATE),
        scope: true,
        groups: vec![vec![group(entity("actual", "A", "patient"), heads)]],
    }
}
pub fn goldens() -> Vec<Scene> {
    let mut year = recommendation("year-boundary", &["should"], None, Some(20230101), false);
    year.scope = true;
    vec![
        recommendation("plain", &["should"], None, Some(DATE), false),
        recommendation("must-may", &["must", "may"], None, Some(DATE), false),
        split("split"),
        recommendation("under-can", &["should"], Some("can"), Some(DATE), false),
        recommendation("under-minus", &["should"], Some("-"), Some(DATE), false),
        mixed("mixed"),
        recommendation("can-only", &["can"], None, Some(DATE), false),
        asserted("asserted", Some(DATE)),
        recommendation("scoped", &["should"], None, Some(DATE), true),
        exclusion_scene("excluded"),
        root_facts("root-fact", false),
        root_facts("two-sentences", true),
        year,
    ]
}
pub fn patient(present: bool) -> Member {
    Member::raw(
        "patient",
        &if present {
            entity("actual", "p", "patient")
        } else {
            Vec::new()
        },
    )
}
pub fn dates(values: &[String]) -> Member {
    Member::raw(
        "dates",
        &values
            .iter()
            .map(|d| format!("guideline_evaluation_date({d})"))
            .collect::<Vec<_>>(),
    )
}
pub fn blocked_scope(active: bool) -> Member {
    Member::raw(
        "scope",
        &if active {
            vec!["guideline_exclusion(alpha)".into()]
        } else {
            Vec::new()
        },
    )
}
