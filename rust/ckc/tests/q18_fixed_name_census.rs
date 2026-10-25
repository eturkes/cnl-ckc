// S2 D1/P1 + retained S1/C1/Q18 law: canonical candidates + byte LCP oracle.
// Contract-derived seed bytes cover all 16 predicates and optional-digest states.
// CKC_V1_TEST_BIN selects a prebuilt executable for red replay.
use std::collections::BTreeSet;
use std::fs;
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::process::Command;

const PREDICATES: [(&str, usize); 16] = [
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
];

fn sha(bytes: &[u8]) -> String {
    use sha2::Digest;
    sha2::Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

fn atom(raw: &str) -> String {
    if raw.as_bytes()[0].is_ascii_lowercase()
        && raw.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
    {
        raw.into()
    } else {
        format!("'{raw}'")
    }
}

fn digest(bytes: Option<&[u8]>) -> String {
    bytes.map_or_else(|| "none".into(), |b| format!("sha256({})", atom(&sha(b))))
}

// No production printer or old seed bytes enter this model.
fn contract_seeds() -> Vec<(String, Vec<u8>)> {
    let declarations: String = [("guideline_schema_version", 1), ("guideline_document", 4)]
        .into_iter()
        .chain(PREDICATES)
        // nt1 D2 + cs1 D2 + sup1 D7 add declarations; the migrated seed payloads retain their census.
        .chain([
            ("guideline_unit", 3),
            ("guideline_exclusion", 1),
            ("guideline_supersession", 4),
            ("guideline_evaluation_date", 1),
        ])
        .map(|(name, arity)| {
            format!(":- multifile({name}/{arity}).\n:- discontiguous({name}/{arity}).\n")
        })
        .collect();
    let literals: Vec<_> = PREDICATES
        .iter()
        .map(|(name, arity)| format!("{name}({})", vec!["x"; *arity].join(",")))
        .collect();
    let clauses: String = literals.iter().map(|lit| format!("{lit}.\n")).collect();
    let body = literals.join(", ");
    let naf = literals
        .iter()
        .map(|lit| format!("\\+ {lit}"))
        .collect::<Vec<_>>()
        .join(", ");
    let bundle = format!(
        "% S1: A structural clause.\n{clauses}guideline_entity(control,control,control,control) :- {body}, {naf}, \\+ ({body}).\n"
    );
    let ace_doc = sha(b"A structural clause.\n");
    let ace_query = sha(b"Which person exists?\n");
    let mut seeds = Vec::new();
    for ulex in [false, true] {
        for vocabulary in [false, true] {
            let label = |present| if present { "sha256" } else { "none" };
            let suffix = format!("ulex-{}-vocabulary-{}", label(ulex), label(vocabulary));
            let u = digest(ulex.then_some(b"prep(for,for).\n"));
            let v =
                digest(vocabulary.then_some(b"# format: kind\tlemma\tvalue\nunit\thour\thour\n"));
            let docid = if ulex { "doc-id" } else { "doc" };
            let qid = if ulex { "query-id" } else { "query" };
            let document = format!(
                "% {docid}.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.\n{declarations}guideline_schema_version(1).\nguideline_document({},ace_sha256({}),ulex({u}),vocabulary({v})).\n{bundle}",
                atom(docid),
                atom(&ace_doc)
            );
            let query = format!(
                "% {qid} compiled from ACE question by ace_to_pl question mode; do not edit.\n'$guideline_query'(v1,{},ace_sha256({}),ulex({u}),vocabulary({v})).\n% Q1: Which person exists?\n'$guideline_query_projection'(goal(guideline_entity(x,A,x,x)),answers([answer(A,noun(person,countable))])).\n",
                atom(qid),
                atom(&ace_query)
            );
            seeds.push((format!("document-{suffix}"), document.into_bytes()));
            seeds.push((format!("query-{suffix}"), query.into_bytes()));
        }
    }
    let query = &seeds
        .iter()
        .find(|(name, _)| name == "query-ulex-sha256-vocabulary-sha256")
        .unwrap()
        .1;
    let qsha = atom(&sha(query));
    let answers = format!(
        "% query-id answered against the loaded composition by ckc queries; do not edit.\n'$guideline_answers'(v1,'query-id',query_sha256({qsha}),result(solutions([sol([x])]))).\n"
    );
    let traces = format!(
        "% query-id traced against the loaded composition by ckc queries; do not edit.\n'$guideline_traces'(v1,'query-id',query_sha256({qsha}),answers_sha256({}),result(solutions([sol([x],proved([clause(sentence('doc-id',1),clause_sha256({}),[])]))]))).\n",
        atom(&sha(answers.as_bytes())),
        atom(&sha(format!("{}.\n", literals[0]).as_bytes()))
    );
    seeds.push(("answers".into(), answers.into_bytes()));
    seeds.push(("traces".into(), traces.into_bytes()));
    seeds
}

fn seed_files(root: &Path) -> Result<Vec<(String, Vec<u8>)>, &'static str> {
    let seeds = contract_seeds();
    assert_eq!(seeds.len(), 10, "C1 seed census");
    let dir = root.join("tests/v1/q18-k1-001");
    let files: BTreeSet<_> = fs::read_dir(&dir)
        .unwrap()
        .map(|e| e.unwrap().file_name().into_string().unwrap())
        .filter(|name| name.ends_with(".pl.in") && name != "input.pl.in")
        .collect();
    let expected: BTreeSet<_> = seeds
        .iter()
        .map(|(name, _)| format!("{name}.pl.in"))
        .collect();
    if files != expected {
        return Err("seed inventory");
    }
    for (name, bytes) in &seeds {
        if fs::read(dir.join(format!("{name}.pl.in"))).unwrap() != *bytes {
            return Err("contract seed bytes");
        }
    }
    Ok(seeds)
}

#[test]
fn q18_contract_seed_bytes() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap();
    let seeds = seed_files(&root).unwrap();
    let scratch = Scratch(root.join(format!("rust/target/q18-seed-law/{}", std::process::id())));
    let dir = scratch.0.join("tests/v1/q18-k1-001");
    fs::create_dir_all(&dir).unwrap();
    for (name, bytes) in &seeds {
        fs::write(dir.join(format!("{name}.pl.in")), bytes).unwrap();
    }
    assert!(seed_files(&scratch.0).is_ok());
    let (name, bytes) = &seeds[0];
    let path = dir.join(format!("{name}.pl.in"));
    let mut changed = bytes.clone();
    changed.push(b' ');
    fs::write(&path, changed).unwrap();
    assert_eq!(seed_files(&scratch.0).unwrap_err(), "contract seed bytes");
    fs::remove_file(&path).unwrap();
    assert_eq!(seed_files(&scratch.0).unwrap_err(), "seed inventory");
    fs::write(&path, bytes).unwrap();
    let extra = dir.join("stray.pl.in");
    fs::write(&extra, bytes).unwrap();
    assert_eq!(seed_files(&scratch.0).unwrap_err(), "seed inventory");
    fs::remove_file(extra).unwrap();
    assert!(seed_files(&scratch.0).is_ok());
    eprintln!("Q18 seed law: 10 byte-equal seeds; changed/omitted/unlisted plants rejected");
}

struct Scratch(PathBuf);
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

struct Term {
    name: Range<usize>,
    span: Range<usize>,
    args: Vec<Term>,
}

// Only locate spans in independently accepted seeds; this is not an input validator.
fn term(bytes: &[u8], at: &mut usize) -> Term {
    let start = *at;
    if bytes[*at] == b'\'' {
        *at += 1;
        while bytes[*at] != b'\'' {
            *at += if bytes[*at] == b'\\' { 2 } else { 1 };
        }
        *at += 1;
    } else if bytes[*at] != b'[' {
        while *at < bytes.len() && !b"(),[].| ".contains(&bytes[*at]) {
            *at += 1;
        }
        assert!(*at > start, "seed term at {start}");
    }
    let name = start..*at;
    let mut args = Vec::new();
    if matches!(bytes.get(*at), Some(b'(' | b'[')) {
        let close = if bytes[*at] == b'(' { b')' } else { b']' };
        *at += 1;
        while bytes[*at] != close {
            args.push(term(bytes, at));
            if matches!(bytes[*at], b',' | b'|') {
                *at += 1;
            } else {
                assert_eq!(bytes[*at], close);
            }
        }
        *at += 1;
    }
    Term {
        name,
        span: start..*at,
        args,
    }
}

struct Slot {
    kind: usize,
    context: &'static str,
    name: Range<usize>,
    candidates: Vec<Vec<u8>>,
}

#[derive(Default)]
struct Domain {
    slots: Vec<Slot>,
    free: Vec<Range<usize>>,
    digests: Vec<Range<usize>>,
    ids: Vec<Range<usize>>,
    literal_controls: Vec<usize>,
    versions: Vec<Range<usize>>,
}

impl Domain {
    fn fixed(&mut self, bytes: &[u8], node: &Term, kind: usize, context: &'static str) {
        self.slots.push(Slot {
            kind,
            context,
            name: node.name.clone(),
            candidates: vec![bytes[node.name.start..=node.name.end].to_vec()],
        });
    }

    fn digest(&mut self, bytes: &[u8], node: &Term, kind: usize, context: &'static str) {
        self.fixed(bytes, node, kind, context);
        assert_eq!(node.args.len(), 1);
        self.digests.push(node.args[0].span.clone());
    }

    fn optional_digest(
        &mut self,
        bytes: &[u8],
        node: &Term,
        kind: usize,
        contexts: (&'static str, &'static str),
    ) {
        self.fixed(bytes, node, kind, contexts.0);
        let value = &node.args[0];
        self.slots.push(Slot {
            kind,
            context: contexts.1,
            name: value.name.clone(),
            candidates: vec![b"none)".to_vec(), b"sha256(".to_vec()],
        });
        if !value.args.is_empty() {
            self.digests.push(value.args[0].span.clone());
        }
    }

    fn backref(&mut self, bytes: &[u8], node: &Term, context: &'static str, document: bool) {
        let raw = &bytes[header_name(bytes, document)];
        let bare = raw[0].is_ascii_lowercase()
            && raw.iter().all(|b| b.is_ascii_alphanumeric() || *b == b'_');
        let spelling = if bare {
            raw.to_vec()
        } else {
            [b"'", raw, b"'"].concat()
        };
        assert_eq!(bytes[node.name.clone()], spelling);
        self.slots.push(Slot {
            kind: 7,
            context,
            name: node.name.clone(),
            candidates: vec![[spelling.as_slice(), b","].concat()],
        });
        self.ids.push(node.span.clone());
    }

    fn literal(&mut self, node: &Term, predicates: &[(&str, usize)], context: &'static str) {
        let mut candidates: Vec<Vec<u8>> = predicates
            .iter()
            .map(|(name, _)| format!("{name}(").into_bytes())
            .collect();
        match context {
            "head" => candidates.push(b"%".to_vec()),
            "body" => candidates.push(b"\\+".to_vec()),
            _ => {}
        }
        self.slots.push(Slot {
            kind: 6,
            context,
            name: node.name.clone(),
            candidates,
        });
        self.free
            .extend(node.args.iter().map(|arg| arg.span.clone()));
    }

    fn body(
        &mut self,
        bytes: &[u8],
        at: &mut usize,
        predicates: &[(&str, usize)],
        context: &'static str,
    ) {
        loop {
            if bytes[*at..].starts_with(b"\\+ ") {
                *at += 3;
                if bytes[*at] == b'(' {
                    *at += 1;
                    self.body(bytes, at, predicates, "grouped-naf");
                    assert_eq!(bytes[*at], b')');
                    *at += 1;
                } else {
                    self.literal(&term(bytes, at), predicates, "naf");
                }
            } else {
                self.literal(&term(bytes, at), predicates, context);
            }
            if !bytes[*at..].starts_with(b", ") {
                break;
            }
            *at += 2;
        }
    }
}

fn domain(bytes: &[u8], seed: &str) -> Domain {
    let mut domain = Domain::default();
    let predicates = &PREDICATES;
    let mut offset = 0;
    for line in bytes.split_inclusive(|&b| b == b'\n') {
        if offset == 0 || line.starts_with(b":-") || line.starts_with(b"guideline_schema_version(")
        {
            domain.literal_controls.push(offset + line.len() - 2);
        }
        if !line.starts_with(b"%") && !line.starts_with(b":-") {
            let mut at = offset;
            let record = term(bytes, &mut at);
            let name = &bytes[record.name.clone()];
            match name {
                b"'$guideline_answers'" | b"'$guideline_traces'" => {
                    let kind = if seed == "answers" { 1 } else { 2 };
                    domain.fixed(bytes, &record, 0, "wrapper");
                    domain.versions.push(record.args[0].name.clone());
                    domain.backref(
                        bytes,
                        &record.args[1],
                        if kind == 1 {
                            "answers-qid"
                        } else {
                            "traces-qid"
                        },
                        false,
                    );
                    domain.digest(bytes, &record.args[2], kind, "query_sha256");
                    if kind == 2 {
                        domain.digest(bytes, &record.args[3], kind, "answers_sha256");
                    }
                    let result = record.args.last().unwrap();
                    domain.fixed(bytes, result, kind, "result");
                    domain.free.push(result.args[0].span.clone());
                }
                b"'$guideline_query'" => {
                    domain.fixed(bytes, &record, 0, "wrapper");
                    domain.versions.push(record.args[0].name.clone());
                    domain.backref(bytes, &record.args[1], "query-qid", false);
                    domain.digest(bytes, &record.args[2], 3, "ace_sha256");
                    domain.optional_digest(bytes, &record.args[3], 3, ("ulex", "ulex-value"));
                    domain.optional_digest(
                        bytes,
                        &record.args[4],
                        3,
                        ("vocabulary", "vocabulary-value"),
                    );
                }
                b"'$guideline_query_projection'" => {
                    domain.fixed(bytes, &record, 0, "projection");
                    for node in &record.args {
                        domain.fixed(bytes, node, 4, "projection-field");
                        domain.free.push(node.args[0].span.clone());
                    }
                }
                b"guideline_document" => {
                    domain.fixed(bytes, &record, 5, "document");
                    domain.backref(bytes, &record.args[0], "document-docid", true);
                    domain.digest(bytes, &record.args[1], 5, "ace_sha256");
                    domain.optional_digest(bytes, &record.args[2], 5, ("ulex", "ulex-value"));
                    domain.optional_digest(
                        bytes,
                        &record.args[3],
                        5,
                        ("vocabulary", "vocabulary-value"),
                    );
                }
                b"guideline_schema_version" => {}
                _ => {
                    assert!(seed.starts_with("document-"));
                    domain.literal(&record, predicates, "head");
                    if bytes[at..].starts_with(b" :- ") {
                        at += 4;
                        domain.body(bytes, &mut at, predicates, "body");
                    }
                    assert_eq!(bytes[at], b'.');
                }
            }
        }
        offset += line.len();
    }
    domain
}

fn header_name(bytes: &[u8], document: bool) -> Range<usize> {
    assert!(bytes.starts_with(b"% "));
    let end = 2 + bytes[2..].iter().position(|&b| b == b' ').unwrap();
    if document {
        assert_eq!(&bytes[end - 3..end], b".pl");
    }
    2..end - if document { 3 } else { 0 }
}

fn replace(bytes: &[u8], span: Range<usize>, replacement: &[u8]) -> Vec<u8> {
    [&bytes[..span.start], replacement, &bytes[span.end..]].concat()
}

fn oracle(bytes: &[u8], slot: &Slot) -> usize {
    let suffix = &bytes[slot.name.start..];
    assert!(
        slot.candidates
            .iter()
            .all(|candidate| !suffix.starts_with(candidate))
    );
    slot.name.start
        + slot
            .candidates
            .iter()
            .map(|candidate| {
                suffix
                    .iter()
                    .zip(candidate)
                    .take_while(|(a, b)| a == b)
                    .count()
            })
            .max()
            .unwrap()
}

fn rejection(bytes: &[u8], offset: usize) -> Vec<u8> {
    let prefix = &bytes[..offset];
    let line = prefix.iter().filter(|&&b| b == b'\n').count() + 1;
    let column = prefix
        .iter()
        .rposition(|&b| b == b'\n')
        .map_or(offset + 1, |i| offset - i);
    format!("ace_to_pl_error(check_load,noncanonical({line},{column})).\n").into_bytes()
}

struct Runner {
    program: PathBuf,
    input: PathBuf,
    checked: usize,
    failures: Vec<String>,
}

impl Runner {
    fn check(&mut self, label: &str, bytes: &[u8], offset: Option<usize>) {
        self.checked += 1;
        fs::write(&self.input, bytes).unwrap();
        let output = Command::new(&self.program)
            .args(["v1", "check"])
            .arg(&self.input)
            .output()
            .unwrap();
        let expected = offset.map_or_else(Vec::new, |at| rejection(bytes, at));
        let rc = if offset.is_some() { 2 } else { 0 };
        if output.status.code() != Some(rc)
            || !output.stdout.is_empty()
            || output.stderr != expected
        {
            self.failures.push(format!(
                "{label}: expected rc {rc} {:?}, got rc {:?}, stdout {:?}, stderr {:?}",
                String::from_utf8_lossy(&expected),
                output.status.code(),
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr),
            ));
        }
    }

    fn mutation(&mut self, label: &str, bytes: &[u8], slot: &Slot, replacement: &[u8]) {
        let mutated = replace(bytes, slot.name.clone(), replacement);
        let expected = oracle(&mutated, slot);
        self.check(label, &mutated, Some(expected));
    }
}

#[test]
fn q18_fixed_name_census() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap();
    let scratch = Scratch(root.join(format!(
        "rust/target/q18-fixed-name-census/{}",
        std::process::id()
    )));
    fs::create_dir_all(&scratch.0).unwrap();
    let mut runner = Runner {
        program: std::env::var_os("CKC_V1_TEST_BIN")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(env!("CARGO_BIN_EXE_ckc"))),
        input: scratch.0.join("input.pl"),
        checked: 0,
        failures: Vec::new(),
    };
    let mut slots = [0usize; 8];
    let mut mandatory = [0usize; 8];
    let mut properties = BTreeSet::new();
    let mut predicate_coverage = BTreeSet::new();
    let mut controls = [0usize; 6];
    let mut property_checks = [0usize; 8];
    let seeds = seed_files(&root).unwrap();
    for (seed, bytes) in &seeds {
        runner.check(&format!("{seed}/canonical"), bytes, None);
        controls[0] += 1;
        let domain = domain(bytes, seed);
        for span in &domain.free {
            let mutated = replace(bytes, span.clone(), b"free_fixed_name");
            runner.check(&format!("{seed}/free@{}", span.start), &mutated, None);
            controls[1] += 1;
        }
        for span in &domain.digests {
            let mutated = replace(bytes, span.clone(), &[b'f'; 64]);
            runner.check(&format!("{seed}/digest@{}", span.start), &mutated, None);
            controls[2] += 1;
        }
        for span in &domain.ids {
            let mutated = replace(bytes, span.clone(), b"free");
            let coupled = replace(
                &mutated,
                header_name(bytes, seed.starts_with("document-")),
                b"free",
            );
            runner.check(&format!("{seed}/coupled-id@{}", span.start), &coupled, None);
            controls[3] += 1;
        }
        for &at in &domain.literal_controls {
            let mut mutated = bytes.clone();
            mutated[at] = b'z';
            runner.check(&format!("{seed}/literal@{at}"), &mutated, Some(at));
            controls[4] += 1;
        }
        for span in &domain.versions {
            let mut mutated = bytes.clone();
            mutated[span.end - 1] = b'z';
            runner.check(
                &format!("{seed}/version-last"),
                &mutated,
                Some(span.end - 1),
            );
            let quoted = [b"'", &bytes[span.clone()], b"'"].concat();
            runner.check(
                &format!("{seed}/version-quote"),
                &replace(bytes, span.clone(), &quoted),
                Some(span.start),
            );
            controls[5] += 2;
        }
        for slot in domain.slots {
            slots[slot.kind] += 1;
            let name = &bytes[slot.name.clone()];
            let label = format!(
                "{seed}/S{}/{}/{}@{}",
                slot.kind + 1,
                slot.context,
                String::from_utf8_lossy(name),
                slot.name.start
            );
            let quoted = name.starts_with(b"'");
            let mut typo = name.to_vec();
            let last = name.len() - if quoted { 2 } else { 1 };
            typo[last] = b'z';
            runner.mutation(&format!("{label}/last"), bytes, &slot, &typo);
            mandatory[slot.kind] += 1;
            if !quoted {
                runner.mutation(
                    &format!("{label}/quote"),
                    bytes,
                    &slot,
                    &[b"'", name, b"'"].concat(),
                );
                mandatory[slot.kind] += 1;
            }
            if slot.kind == 6 {
                predicate_coverage.insert((seed.clone(), slot.context, name.to_vec()));
            }
            if properties.insert((slot.kind, slot.context, name.to_vec())) {
                let before = runner.checked;
                // Every lexical prefix length: a different byte, an ended atom, and an extension.
                let first = usize::from(quoted);
                let end = name.len() - usize::from(quoted);
                for cut in first..end {
                    let mut altered = name.to_vec();
                    altered[cut] = b'z';
                    runner.mutation(&format!("{label}/byte-{cut}"), bytes, &slot, &altered);
                    let shorter = if quoted {
                        [&name[..cut], b"'"].concat()
                    } else if cut == 0 {
                        b"''".to_vec()
                    } else {
                        name[..cut].to_vec()
                    };
                    // A prefix that is another candidate is an arity question, outside D1.
                    if !slot
                        .candidates
                        .iter()
                        .any(|candidate| candidate[..candidate.len() - 1] == shorter)
                    {
                        runner.mutation(&format!("{label}/prefix-{cut}"), bytes, &slot, &shorter);
                    }
                }
                let extended = [&name[..end], b"z", &name[end..]].concat();
                runner.mutation(&format!("{label}/extended"), bytes, &slot, &extended);
                property_checks[slot.kind] += runner.checked - before;
            }
        }
    }
    for (seed, _) in seeds
        .iter()
        .filter(|(name, _)| name.starts_with("document-"))
    {
        for context in ["head", "body", "naf", "grouped-naf"] {
            for (name, _) in &PREDICATES {
                assert!(
                    predicate_coverage.contains(&(seed.clone(), context, name.as_bytes().to_vec())),
                    "{seed}/{context}/{name}"
                );
            }
        }
    }
    assert_eq!(
        predicate_coverage.len(),
        256,
        "per-document predicate/context census"
    );
    assert_eq!(slots, [10, 2, 3, 20, 8, 24, 260, 10], "S1–S8 slot census");
    assert_eq!(
        mandatory,
        [10, 4, 6, 40, 16, 48, 520, 14],
        "mutation census"
    );
    assert_eq!(properties.len(), 96, "prefix-property domains");
    assert_eq!(
        property_checks,
        [160, 38, 67, 95, 24, 132, 2308, 82],
        "prefix-property checks"
    );
    assert_eq!(
        controls,
        [10, 1130, 19, 10, 190, 12],
        "canonical/free/digest/id/literal/version controls"
    );
    assert_eq!(runner.checked, 4935, "complete check census");
    eprintln!(
        "Q18 census: seeds={} slots={slots:?} mandatory={mandatory:?} property-domains={} property-checks={property_checks:?} controls={controls:?} checked={} deviations={}",
        seeds.len(),
        properties.len(),
        runner.checked,
        runner.failures.len()
    );
    assert!(
        runner.failures.is_empty(),
        "Q18 fixed-name law: {} of {} differ:\n{}",
        runner.failures.len(),
        runner.checked,
        runner.failures.join("\n")
    );
}
