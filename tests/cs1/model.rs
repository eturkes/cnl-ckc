use super::common::*;

#[derive(Clone)]
pub struct Group {
    pub k: usize,
    pub body: Vec<String>,
    pub heads: Vec<String>,
    pub record: bool,
    pub bindings: Vec<(char, String)>,
}
impl Group {
    fn bind(&self, id: &str, text: &str) -> String {
        substitute(text, |v| {
            self.bindings
                .iter()
                .find(|(name, _)| *name == v)
                .map_or_else(
                    || {
                        gid(
                            "witness",
                            id,
                            v as usize - 'A' as usize + 1,
                            &format!("variant({})", self.k),
                        )
                    },
                    |(_, value)| value.clone(),
                )
        })
    }
    pub fn heads(&self, key: Option<&str>) -> Vec<String> {
        let mut heads = self.heads.clone();
        if self.record {
            heads.push(exclusion(key.unwrap()));
        }
        heads
    }
    pub fn clauses(&self, key: Option<&str>) -> Vec<String> {
        self.heads(key)
            .iter()
            .map(|head| {
                let mut body = self.body.clone();
                if let Some(key) = key
                    && head.starts_with("guideline_operator(")
                    && [",should)", ",must)", ",may)"]
                        .iter()
                        .any(|suffix| head.ends_with(suffix))
                {
                    body.push(format!("\\+ {}", exclusion(key)));
                }
                // The compiler runs the body binding-first (contract bf).
                let ordered = crate::reorder::document(&format!(
                    "{}.",
                    canonical(&format!("{head} :- {}", body.join(", ")))
                ));
                ordered.strip_suffix('.').unwrap().to_owned()
            })
            .collect()
    }
    pub fn payload(&self, id: &str, key: Option<&str>) -> String {
        let facts: Vec<_> = self
            .body
            .iter()
            .filter(|g| !g.starts_with("\\+ ") && !g.starts_with("guideline_satisfies("))
            .map(|g| self.bind(id, g))
            .collect();
        let heads: Vec<_> = self.heads(key).iter().map(|h| self.bind(id, h)).collect();
        crate::reorder::document(&proof(id, self.k, &facts, &heads))
    }
}
#[derive(Clone)]
pub struct Scene {
    pub id: String,
    pub ace: String,
    pub drs: String,
    pub key: Option<String>,
    pub groups: Vec<Group>,
}
impl Scene {
    pub fn table(&self) -> Option<String> {
        self.key
            .as_ref()
            .map(|key| fixture("vocabulary.tsv.in").replace("\talpha\n", &format!("\t{key}\n")))
    }
    pub fn member(&self) -> Member {
        Member {
            id: self.id.clone(),
            clauses: self
                .groups
                .iter()
                .flat_map(|g| g.clauses(self.key.as_deref()))
                .collect(),
            payload: self
                .groups
                .iter()
                .map(|g| g.payload(&self.id, self.key.as_deref()))
                .collect(),
        }
    }
    pub fn pl(&self) -> String {
        document_text(
            &self.id,
            &self.ace,
            self.table().as_deref(),
            &self.member().clauses,
        )
    }
    pub fn dump(&self) -> String {
        let text = self.ace.trim_end().strip_suffix('.').unwrap();
        let tokens = text
            .split_whitespace()
            .map(atom)
            .collect::<Vec<_>>()
            .join(",");
        format!(
            "sentences([[^,{tokens},'.']]).\ndrs({}).\nmessages([]).\n",
            canonical(&self.drs)
        )
    }
}
fn anchor(condition: &str, at: usize) -> String {
    format!("-({condition},/(1,{at}))")
}
fn object(r: &str, noun: &str, at: usize) -> String {
    anchor(&format!("object({r},{},countable,na,eq,1)", atom(noun)), at)
}
fn group(body: Vec<String>, heads: Vec<String>, record: bool) -> Group {
    Group {
        k: 1,
        body,
        heads,
        record,
        bindings: Vec::new(),
    }
}
pub fn recommendation(id: &str, key: Option<&str>, ops: &[&str], nested: bool) -> Scene {
    let mut heads = Vec::new();
    let mut conditions = Vec::new();
    let mut source = Vec::new();
    let mut box_no = 1;
    if ops.is_empty() {
        heads.extend(event(
            "actual",
            &gid("product", id, 2, "[A]"),
            "wait",
            "A",
            None,
        ));
        conditions.push(anchor("predicate(B,wait,A)", 12));
        source.push("the patient waits".to_owned());
    }
    for (i, op) in ops.iter().enumerate() {
        let variable = char::from(b'B' + i as u8);
        let parent = if nested {
            let parent = context(id, box_no, "[A]");
            heads.push(format!("guideline_operator(actual,{parent},can)"));
            box_no += 1;
            parent
        } else {
            "actual".into()
        };
        let ctx = context(id, box_no, "[A]");
        box_no += 1;
        heads.push(format!("guideline_operator({parent},{ctx},{op})"));
        heads.extend(event(
            &ctx,
            &gid("product", id, i + 2, "[A]"),
            "wait",
            "A",
            None,
        ));
        let modal = format!(
            "{op}(drs([{variable}],[{}]))",
            anchor(&format!("predicate({variable},wait,A)"), 12 + i * 5)
        );
        conditions.push(if nested {
            format!("can(drs([],[{modal}]))")
        } else {
            modal
        });
        source.push(if nested {
            format!("it is possible that the patient {op} wait")
        } else {
            format!("the patient {op} wait")
        });
    }
    let dom = if ops.is_empty() { "[B]" } else { "[]" };
    Scene {
        id: id.into(),
        ace: format!("If there is a patient then {}.\n", source.join(" and ")),
        drs: format!(
            "drs([],[=>(drs([A],[{}]),drs({dom},[{}]))])",
            object("A", "patient", 6),
            conditions.join(",")
        ),
        key: key.map(str::to_owned),
        groups: vec![group(entity("actual", "A", "patient"), heads, false)],
    }
}

pub fn split_recommendation(id: &str, key: &str, op: &str, nested: bool) -> Scene {
    let mut scene = recommendation(id, Some(key), &[op], nested);
    let template = scene.groups[0].clone();
    scene.groups = [("B", "pain", "[A,B]"), ("C", "fever", "[A,C]")]
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
    let modal = format!("{op}(drs([D],[{}]))", anchor("predicate(D,wait,A)", 23));
    let modal = if nested {
        format!("can(drs([],[{modal}]))")
    } else {
        modal
    };
    scene.drs = format!(
        "drs([],[=>(drs([A],[{},v(drs([B],[{}]),drs([C],[{}]))]),drs([],[{modal}]))])",
        object("A", "patient", 6),
        object("B", "pain", 11),
        object("C", "fever", 16)
    );
    scene.ace = format!(
        "If there is a patient and there is a pain or there is a fever then {}the patient {op} wait.\n",
        if nested { "it is possible that " } else { "" }
    );
    scene
}

#[derive(Clone, Copy, Debug)]
pub enum ExclusionForm {
    Bare,
    OnePp,
    TwoPp,
    Curried,
    Split,
    Exception,
    Comparison,
    TimePp,
}
pub fn exclusion_scene(id: &str, key: &str, form: ExclusionForm) -> Scene {
    let patient = object("A", "patient", 3);
    let mut body = entity("actual", "A", "patient");
    let (ace, antecedent, deps, subject_slot, event_slot, pps) = match form {
        ExclusionForm::Bare => (
            "If there is a patient then it is false that a recommendation applies.\n".into(),
            format!("drs([A],[{patient}])"), "[A]", 2, 3, vec![],
        ),
        ExclusionForm::OnePp => (
            "If there is a patient then it is false that a recommendation applies to the patient.\n".into(),
            format!("drs([A],[{patient}])"), "[A]", 2, 3, vec![("to", "A")],
        ),
        ExclusionForm::TwoPp | ExclusionForm::Curried => {
            body.extend(entity("actual", "B", "clinician"));
            let clinician = object("B", "clinician", 8);
            let (ace, ante) = if matches!(form, ExclusionForm::Curried) {
                ("If there is a patient then if there is a clinician then it is false that a recommendation applies to the patient for the clinician.\n", format!("drs([A],[{patient}])"))
            } else {
                ("If there is a patient and there is a clinician then it is false that a recommendation applies to the patient for the clinician.\n", format!("drs([A,B],[{patient},{clinician}])"))
            };
            (ace.into(), ante, "[A,B]", 3, 4, vec![("to", "A"), ("for", "B")])
        }
        ExclusionForm::Split => (
            "If there is a patient and there is a pain or there is a fever then it is false that a recommendation applies to the patient.\n".into(),
            format!("drs([A],[{patient},v(drs([B],[{}]),drs([C],[{}]))])", object("B", "pain", 10), object("C", "fever", 15)),
            "[A,B]", 4, 5, vec![("to", "A")],
        ),
        ExclusionForm::Exception => {
            body.extend(entity("actual", "B", "pain"));
            body.extend(event("actual", "C", "have", "A", Some("B")));
            let mut exception = entity("actual", "D", "facility-discharge");
            exception.extend(event("actual", "E", "have", "A", Some("D")));
            body.push(format!("\\+ ({})", exception.join(", ")));
            (
                "If a patient has a pain and the patient does not provably have a facility-discharge then it is false that a recommendation applies to the patient.\n".into(),
                format!("drs([A,B,C],[{patient},{},{},~(drs([D,E],[{},{}]))])", object("B", "pain", 6), anchor("predicate(C,have,A,B)", 4), object("D", "facility-discharge", 16), anchor("predicate(E,have,A,D)", 14)),
                "[A,B,C]", 6, 7, vec![("to", "A")],
            )
        }
        ExclusionForm::Comparison => {
            body.extend([
                "guideline_cardinality(actual,B,na,X,Y)".into(),
                "guideline_unit(actual,B,Z)".into(),
                "guideline_satisfies(X,Y,Z,less,18,year)".into(),
            ]);
            body.extend(event("actual", "C", "include", "A", Some("B")));
            (
                "If a patient includes less than 18 years then it is false that a recommendation applies to the patient.\n".into(),
                format!("drs([A,B,C],[{patient},[{},{}]])", anchor("object(B,year,countable,na,less,18)", 8), anchor("predicate(C,include,A,B)", 4)),
                "[A,B,C]", 4, 5, vec![("to", "A")],
            )
        }
        ExclusionForm::TimePp => {
            body.extend(entity("actual", "B", "surgery"));
            body.extend(event("actual", "C", "have", "A", Some("B")));
            (
                "If a patient has a surgery then it is false that a recommendation applies before the surgery.\n".into(),
                format!("drs([A,B,C],[{patient},{},{}])", object("B", "surgery", 6), anchor("predicate(C,have,A,B)", 4)),
                "[A,B,C]", 4, 5, vec![("before", "B")],
            )
        }
    };
    let subject = char::from(b'A' + subject_slot as u8 - 1);
    let ev = char::from(b'A' + event_slot as u8 - 1);
    let mut conds = vec![
        object(&subject.to_string(), "recommendation", 20),
        anchor(&format!("predicate({ev},apply,{subject})"), 21),
    ];
    for (prep, obj) in &pps {
        conds.push(anchor(&format!("modifier_pp({ev},{prep},{obj})"), 22));
    }
    let cons = format!("drs([],[-(drs([{subject},{ev}],[{}]))])", conds.join(","));
    let cons = if matches!(form, ExclusionForm::Curried) {
        format!(
            "drs([],[=>(drs([B],[{}]),{cons})])",
            object("B", "clinician", 11)
        )
    } else {
        cons
    };
    let heads_for = |deps: &str| {
        let ctx = context(id, 1, deps);
        let r = gid("product", id, subject_slot, deps);
        let e = gid("product", id, event_slot, deps);
        let mut heads = vec![format!("guideline_operator(actual,{ctx},-)")];
        heads.extend(entity(&ctx, &r, "recommendation"));
        heads.extend(event(&ctx, &e, "apply", &r, None));
        for (prep, obj) in &pps {
            heads.push(format!("guideline_pp({ctx},{e},{prep},{obj})"));
            if matches!(form, ExclusionForm::TimePp) {
                heads.push(format!("guideline_order({ctx},{e},before,{obj})"));
            }
        }
        heads
    };
    let groups = if matches!(form, ExclusionForm::Split) {
        [("B", "pain", "[A,B]"), ("C", "fever", "[A,C]")]
            .iter()
            .enumerate()
            .map(|(i, (r, noun, deps))| {
                let mut branch = body.clone();
                branch.extend(entity("actual", r, noun));
                let mut g = group(branch, heads_for(deps), true);
                g.k = i + 1;
                g
            })
            .collect()
    } else {
        let mut g = group(body, heads_for(deps), true);
        if matches!(form, ExclusionForm::Comparison) {
            g.bindings = vec![
                ('X', "less".into()),
                ('Y', "18".into()),
                ('Z', "year".into()),
            ];
        }
        vec![g]
    };
    Scene {
        id: id.into(),
        ace,
        drs: format!("drs([],[=>({antecedent},{cons})])"),
        key: Some(key.into()),
        groups,
    }
}

// P4 compositions vary the condition noun; certification keeps its source scene.
pub fn patient_exclusion(id: &str, key: &str, noun: &str) -> Member {
    let mut scene = exclusion_scene(id, key, ExclusionForm::Exception);
    for body in &mut scene.groups[0].body {
        *body = body.replace(",pain,", &format!(",{},", atom(noun)));
    }
    scene.member()
}
pub fn patient(id: &str, pain: bool, fever: bool, discharge: bool) -> Member {
    let p = gid("product", id, 1, "[]");
    let mut facts = entity("actual", &p, "patient");
    for (i, (present, noun)) in [
        (pain, "pain"),
        (fever, "fever"),
        (discharge, "facility-discharge"),
    ]
    .iter()
    .enumerate()
    {
        if *present {
            let q = gid("product", id, i * 2 + 2, "[]");
            let e = gid("product", id, i * 2 + 3, "[]");
            facts.extend(entity("actual", &q, noun));
            facts.extend(event("actual", &e, "have", &p, Some(&q)));
        }
    }
    Member::facts(id, facts)
}
