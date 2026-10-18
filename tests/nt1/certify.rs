use super::common::*;

const CONDITIONS: [&str; 4] = [
    "cert-non-time",
    "cert-time",
    "cert-query",
    "cert-time-query",
];
#[derive(Clone)]
struct Sample {
    id: &'static str,
    table: Option<String>,
    heads: Vec<String>,
    body: Vec<String>,
}
fn entity(r: &str, noun: &str) -> String {
    format!("guideline_entity(actual,{r},{noun},countable)")
}
fn card(r: &str, comparison: &str, count: &str) -> String {
    format!("guideline_cardinality(actual,{r},na,{comparison},{count})")
}
fn event(e: &str, verb: &str, a: &str, b: Option<&str>) -> Vec<String> {
    let mut out = vec![
        format!("guideline_event(actual,{e},{verb})"),
        format!("guideline_arg(actual,{e},1,{a})"),
    ];
    if let Some(b) = b {
        out.push(format!("guideline_arg(actual,{e},2,{b})"));
    }
    out
}
fn substituted(text: &str, replacement: impl Fn(char) -> String) -> String {
    let (mut quoted, mut escaped) = (false, false);
    let mut out = String::new();
    for ch in text.chars() {
        if escaped {
            out.push(ch);
            escaped = false;
        } else if quoted && ch == '\\' {
            out.push(ch);
            escaped = true;
        } else if ch == '\'' {
            quoted = !quoted;
            out.push(ch);
        } else if !quoted && ch.is_ascii_uppercase() {
            out.push_str(&replacement(ch));
        } else {
            out.push(ch);
        }
    }
    out
}
fn numeric_annotation(s: &str) -> bool {
    s.starts_with("guideline_satisfies(") || s.starts_with("guideline_unit(")
}
impl Sample {
    fn new(id: &'static str) -> Self {
        let mut sample = Self {
            id,
            table: Some(fixture("vocabulary.tsv.in")),
            heads: Vec::new(),
            body: Vec::new(),
        };
        if id == "cert-time-root" {
            let p = gid("product", id, 1, "[]");
            let q = gid("product", id, 2, "[]");
            let e = gid("product", id, 3, "[]");
            sample.heads = vec![
                entity(&p, "patient"),
                card(&p, "eq", "1"),
                entity(&q, "day"),
                card(&q, "eq", "3"),
                format!("guideline_unit(actual,{q},day)"),
            ];
            sample.heads.extend(event(&e, "take", &p, Some(&q)));
            return sample;
        }
        let time = sample.time();
        let question = sample.question();
        let (s, n, u, e) = match (question, time) {
            (false, false) => ("D", "E", "", "C"),
            (false, true) => ("D", "E", "F", "C"),
            (true, false) => ("C", "D", "", "E"),
            (true, true) => ("C", "D", "E", "F"),
        };
        sample.body = vec![entity("A", "patient"), card("A", "eq", "1")];
        if !time {
            sample.body.push(entity("B", "tablet"));
        }
        sample.body.push(card("B", s, n));
        if time {
            sample.body.push(format!("guideline_unit(actual,B,{u})"));
            sample
                .body
                .push(format!("guideline_satisfies({s},{n},{u},greater,3,day)"));
        } else {
            sample
                .body
                .push(format!("guideline_satisfies({s},{n},geq,50)"));
        }
        sample.body.extend(event(e, "take", "A", Some("B")));
        if !question {
            sample.heads = event(&gid("product", id, 4, "[A,B,C]"), "wait", "A", None);
        }
        sample
    }
    fn time(&self) -> bool {
        self.id.contains("time") && self.id != "cert-non-time"
    }
    fn question(&self) -> bool {
        self.id.ends_with("query")
    }
    fn pl(&self) -> String {
        let ace = fixture(&format!("{}.ace.in", self.id));
        if self.question() {
            query_text(
                self.id,
                &ace,
                self.table.as_deref(),
                &conjunction(&self.body),
                "[]",
            )
        } else {
            let clauses: Vec<_> = self
                .heads
                .iter()
                .map(|head| {
                    if self.body.is_empty() {
                        head.clone()
                    } else {
                        format!("{head} :- {}", self.body.join(", "))
                    }
                })
                .collect();
            document_text(self.id, &ace, self.table.as_deref(), &clauses)
        }
    }
    fn clean_output(&self) -> String {
        if self.question() {
            return format!("ckc: certify ok {}\n", self.id);
        }
        if self.body.is_empty() {
            return proof(self.id, &[], &self.heads);
        }
        let substitute = |text: &str| {
            substituted(text, |v| match v {
                'A'..='C' => gid(
                    "witness",
                    self.id,
                    v as usize - 'A' as usize + 1,
                    "variant(1)",
                ),
                'D' => {
                    if self.time() {
                        "greater".into()
                    } else {
                        "geq".into()
                    }
                }
                'E' => {
                    if self.time() {
                        "3".into()
                    } else {
                        "50".into()
                    }
                }
                'F' => "day".into(),
                _ => panic!("modeled witness variable {v}"),
            })
        };
        let witness = self
            .body
            .iter()
            .filter(|g| !g.starts_with("guideline_satisfies("))
            .map(|g| substitute(g))
            .collect::<Vec<_>>();
        let heads = self.heads.iter().map(|h| substitute(h)).collect::<Vec<_>>();
        proof(self.id, &witness, &heads)
    }
    fn run(&self, c: &mut Cases, label: &str, why: Option<&str>) {
        c.write("ace.txt", &fixture(&format!("{}.ace.in", self.id)));
        c.write("dump.txt", &fixture(&format!("{}.dump.in", self.id)));
        c.write("cert.pl", &self.pl());
        let mut args = vec![
            "certify-one",
            if self.question() { "query" } else { "doc" },
            self.id,
            "ace.txt",
            "-",
            "dump.txt",
            "cert.pl",
        ];
        if let Some(table) = &self.table {
            c.write("vocabulary.tsv", table);
            args.push("vocabulary.tsv");
        }
        let want = why.map_or_else(
            || Out::ok(self.clean_output()),
            |why| Out::err(1, format!("ckc: certify: {}: {why}.\n", self.id)),
        );
        c.expect(label, &args, want);
    }
    fn why(&self) -> &'static str {
        if self.question() {
            "projection"
        } else {
            "clauses(1)"
        }
    }
    fn primitive_body(&self) -> Vec<&str> {
        self.body
            .iter()
            .filter(|s| !numeric_annotation(s))
            .map(String::as_str)
            .collect()
    }
    fn builtin(&self) -> usize {
        self.body
            .iter()
            .position(|s| s.starts_with("guideline_satisfies("))
            .unwrap()
    }
    fn reject(&self, clean: &Self, c: &mut Cases, label: &str, primitives: bool) {
        assert_ne!(self.pl(), clean.pl(), "{label}: mutant changes bytes");
        if primitives {
            assert_eq!(
                self.primitive_body(),
                clean.primitive_body(),
                "{label}: primitives unchanged"
            );
        }
        c.check(&format!("{label}/canonical"), &self.pl());
        self.run(c, label, Some(self.why()));
    }
}
#[test]
fn p5_clean_condition_question_and_root_unit_certification() {
    let mut c = Cases::new("P5-controls");
    for id in CONDITIONS.into_iter().chain(["cert-time-root"]) {
        let mut s = Sample::new(id);
        c.check(id, &s.pl());
        s.run(&mut c, id, None);
        if matches!(id, "cert-non-time" | "cert-query") {
            s.table = None;
            s.run(&mut c, &format!("{id}/non-time-without-table"), None);
        }
    }
    c.finish();
}
#[test]
fn p5_bound_and_variable_link_mutants_reject() {
    let mut c = Cases::new("P5-bound-links");
    for id in CONDITIONS {
        let clean = Sample::new(id);
        clean.run(&mut c, &format!("{id}/control"), None);
        let at = clean.builtin();
        let args: Vec<_> = clean.body[at]
            .strip_prefix("guideline_satisfies(")
            .unwrap()
            .strip_suffix(')')
            .unwrap()
            .split(',')
            .map(str::to_owned)
            .collect();
        let bound = if clean.time() { 3 } else { 2 };
        let mut edits = vec![
            ("stated-count-link-swapped", 0, args[1].clone()),
            ("count-stated-link-swapped", 1, args[0].clone()),
            ("bound-comparison", bound, "less".into()),
            ("bound-eq-is-not-a-conversion", bound, "eq".into()),
            ("bound-value", bound + 1, "51".into()),
        ];
        if clean.time() {
            edits.extend([
                ("source-unit-link", 2, "A".into()),
                ("bound-unit", 5, "week".into()),
                ("out-of-domain-unit", 5, "fortnight".into()),
            ]);
        }
        for (label, slot, value) in edits {
            let mut changed = clean.clone();
            let mut args = args.clone();
            args[slot] = value;
            changed.body[at] = format!("guideline_satisfies({})", args.join(","));
            changed.reject(&clean, &mut c, &format!("{id}/{label}"), true);
        }
        let mut changed = clean.clone();
        let mut swapped = args.clone();
        swapped.swap(0, 1);
        changed.body[at] = format!("guideline_satisfies({})", swapped.join(","));
        changed.reject(&clean, &mut c, &format!("{id}/both-links-swapped"), true);
    }
    c.finish();
}
#[test]
fn p5_dropped_added_or_misplaced_builtin_rejects() {
    let mut c = Cases::new("P5-builtin-structure");
    for id in CONDITIONS {
        let clean = Sample::new(id);
        clean.run(&mut c, &format!("{id}/control"), None);
        for action in ["drop", "duplicate", "add", "misplace"] {
            let mut s = clean.clone();
            let at = s.builtin();
            match action {
                "drop" => {
                    s.body.remove(at);
                }
                "duplicate" => {
                    s.body.insert(at, s.body[at].clone());
                }
                "add" => {
                    s.body.push("guideline_satisfies(exactly,60,geq,50)".into());
                }
                "misplace" => {
                    let goal = s.body.remove(at);
                    s.body.push(goal);
                }
                _ => unreachable!(),
            }
            s.reject(&clean, &mut c, &format!("{id}/{action}"), true);
        }
    }
    c.finish();
}
#[test]
fn p5_unit_and_entity_projection_mutants_reject() {
    let mut c = Cases::new("P5-unit-entity");
    for id in CONDITIONS {
        let clean = Sample::new(id);
        clean.run(&mut c, &format!("{id}/control"), None);
        if clean.time() {
            let at = clean
                .body
                .iter()
                .position(|g| g.starts_with("guideline_unit("))
                .unwrap();
            for action in ["drop", "duplicate", "context", "referent", "unit-link"] {
                let mut s = clean.clone();
                match action {
                    "drop" => {
                        s.body.remove(at);
                    }
                    "duplicate" => {
                        s.body.insert(at, s.body[at].clone());
                    }
                    "context" => {
                        s.body[at] = s.body[at].replace("(actual,", "(other,");
                    }
                    "referent" => {
                        s.body[at] = s.body[at].replace(",B,", ",A,");
                    }
                    "unit-link" => {
                        s.body[at] = "guideline_unit(actual,B,A)".into();
                    }
                    _ => unreachable!(),
                }
                s.reject(&clean, &mut c, &format!("{id}/unit-{action}"), true);
            }
            let mut s = clean.clone();
            let at = s
                .body
                .iter()
                .position(|g| g.starts_with("guideline_cardinality(actual,B,"))
                .unwrap();
            s.body.insert(at, entity("B", "day"));
            assert_eq!(
                s.body
                    .iter()
                    .filter(|s| **s != entity("B", "day"))
                    .cloned()
                    .collect::<Vec<_>>(),
                clean.body
            );
            s.reject(
                &clean,
                &mut c,
                &format!("{id}/converted-time-keeps-entity"),
                false,
            );
        } else {
            let mut s = clean.clone();
            s.body.push("guideline_unit(actual,B,day)".into());
            s.reject(
                &clean,
                &mut c,
                &format!("{id}/unit-added-to-non-time"),
                true,
            );
            let mut s = clean.clone();
            let at = s
                .body
                .iter()
                .position(|g| *g == entity("B", "tablet"))
                .unwrap();
            s.body.remove(at);
            let mut restored = s.body.clone();
            restored.insert(at, entity("B", "tablet"));
            assert_eq!(restored, clean.body);
            s.reject(
                &clean,
                &mut c,
                &format!("{id}/non-time-loses-entity"),
                false,
            );
        }
    }
    let clean = Sample::new("cert-time-root");
    let at = clean
        .heads
        .iter()
        .position(|s| s.starts_with("guideline_unit("))
        .unwrap();
    clean.run(&mut c, "root/control", None);
    for action in ["drop", "duplicate", "unit", "misplace"] {
        let mut s = clean.clone();
        match action {
            "drop" => {
                s.heads.remove(at);
            }
            "duplicate" => {
                s.heads.insert(at, s.heads[at].clone());
            }
            "unit" => {
                s.heads[at] = s.heads[at].replace(",day)", ",week)");
            }
            "misplace" => {
                let g = s.heads.remove(at);
                s.heads.push(g);
            }
            _ => unreachable!(),
        }
        assert_eq!(
            s.heads
                .iter()
                .filter(|s| !numeric_annotation(s))
                .collect::<Vec<_>>(),
            clean
                .heads
                .iter()
                .filter(|s| !numeric_annotation(s))
                .collect::<Vec<_>>()
        );
        s.reject(&clean, &mut c, &format!("root/{action}"), true);
    }
    c.finish();
}
#[test]
fn p5_builtin_head_rejects_before_projection() {
    let mut c = Cases::new("P5-builtin-head");
    for id in ["cert-non-time", "cert-time", "cert-time-root"] {
        let mut s = Sample::new(id);
        s.run(&mut c, &format!("{id}/control"), None);
        s.heads
            .push("guideline_satisfies(exactly,60,geq,50)".into());
        // D7's head grammar precedes D8's projection; MAIN's explicit ruling.
        s.run(&mut c, &format!("{id}/head"), Some("noncanonical"));
    }
    c.finish();
}
