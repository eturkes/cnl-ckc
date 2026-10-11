// Q18 D1/P2: fixed-name mutations; oracle = local canonical candidates + byte LCP.
// Seeds cover every schema predicate in heads, bodies and both NAF forms.
// CKC_V1_TEST_BIN selects a prebuilt executable for red replay.
use std::collections::BTreeSet;
use std::fs;
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::process::Command;

const PREDICATES: [&str; 13] = [
    "guideline_entity",
    "guideline_cardinality",
    "guideline_event",
    "guideline_arg",
    "guideline_pp",
    "guideline_property",
    "guideline_operator",
    "guideline_interval",
    "guideline_recurrence",
    "guideline_frequency",
    "guideline_order",
    "guideline_recurrence_window",
    "guideline_range",
];
const SEEDS: [&str; 14] = [
    "answers",
    "traces",
    "query-v1-none",
    "query-v1-sha256",
    "query-v2-none",
    "query-v2-sha256",
    "query-v3-none",
    "query-v3-sha256",
    "document-v1-none",
    "document-v1-sha256",
    "document-v2-none",
    "document-v2-sha256",
    "document-v3-none",
    "document-v3-sha256",
];

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

    fn ulex(&mut self, bytes: &[u8], node: &Term, kind: usize) {
        self.fixed(bytes, node, kind, "ulex");
        let value = &node.args[0];
        self.slots.push(Slot {
            kind,
            context: "ulex-value",
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

    fn temporal(&mut self, bytes: &[u8], node: &Term, kind: usize) {
        self.fixed(bytes, node, kind, "temporal");
        self.digest(bytes, &node.args[0], kind, "temporal-sha256");
    }

    fn literal(&mut self, node: &Term, predicates: &[&str], context: &'static str) {
        let mut candidates: Vec<Vec<u8>> = predicates
            .iter()
            .map(|name| format!("{name}(").into_bytes())
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

    fn body(&mut self, bytes: &[u8], at: &mut usize, predicates: &[&str], context: &'static str) {
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
    let version = if seed.contains("v3") {
        3
    } else if seed.contains("v2") {
        2
    } else {
        1
    };
    let predicates = &PREDICATES[..[0, 7, 9, 13][version]];
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
                    domain.ulex(bytes, &record.args[3], 3);
                    if version > 1 {
                        domain.temporal(bytes, &record.args[4], 3);
                    }
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
                    domain.ulex(bytes, &record.args[2], 5);
                    if version > 1 {
                        domain.temporal(bytes, &record.args[3], 5);
                    }
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
    let mut controls = 0;
    for seed in SEEDS {
        let bytes = fs::read(root.join(format!("tests/v1/q18-k1-001/{seed}.pl.in"))).unwrap();
        runner.check(&format!("{seed}/canonical"), &bytes, None);
        controls += 1;
        let domain = domain(&bytes, seed);
        let version = seed.split('-').nth(1).unwrap_or("v1");
        for span in &domain.free {
            let mutated = replace(&bytes, span.clone(), b"free_fixed_name");
            runner.check(&format!("{seed}/free@{}", span.start), &mutated, None);
            controls += 1;
        }
        for span in &domain.digests {
            let mutated = replace(&bytes, span.clone(), &[b'f'; 64]);
            runner.check(&format!("{seed}/digest@{}", span.start), &mutated, None);
            controls += 1;
        }
        for span in &domain.ids {
            let mutated = replace(&bytes, span.clone(), b"free");
            let coupled = replace(
                &mutated,
                header_name(&bytes, seed.starts_with("document-")),
                b"free",
            );
            runner.check(&format!("{seed}/coupled-id@{}", span.start), &coupled, None);
            controls += 1;
        }
        for &at in &domain.literal_controls {
            let mut mutated = bytes.clone();
            mutated[at] = b'z';
            runner.check(&format!("{seed}/literal@{at}"), &mutated, Some(at));
            controls += 1;
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
                &replace(&bytes, span.clone(), &quoted),
                Some(span.start),
            );
            controls += 2;
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
            runner.mutation(&format!("{label}/last"), &bytes, &slot, &typo);
            mandatory[slot.kind] += 1;
            if !quoted {
                runner.mutation(
                    &format!("{label}/quote"),
                    &bytes,
                    &slot,
                    &[b"'", name, b"'"].concat(),
                );
                mandatory[slot.kind] += 1;
            }
            if slot.kind == 6 {
                predicate_coverage.insert((version.to_owned(), slot.context, name.to_vec()));
            }
            if properties.insert((version.to_owned(), slot.kind, slot.context, name.to_vec())) {
                // Every lexical prefix length: a different byte, an ended atom, and an extension.
                let first = usize::from(quoted);
                let end = name.len() - usize::from(quoted);
                for cut in first..end {
                    let mut altered = name.to_vec();
                    altered[cut] = b'z';
                    runner.mutation(&format!("{label}/byte-{cut}"), &bytes, &slot, &altered);
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
                        runner.mutation(&format!("{label}/prefix-{cut}"), &bytes, &slot, &shorter);
                    }
                }
                let extended = [&name[..end], b"z", &name[end..]].concat();
                runner.mutation(&format!("{label}/extended"), &bytes, &slot, &extended);
            }
        }
    }
    for (version, count) in [("v1", 7), ("v2", 9), ("v3", 13)] {
        for context in ["head", "body", "naf", "grouped-naf"] {
            for name in &PREDICATES[..count] {
                assert!(predicate_coverage.contains(&(
                    version.to_owned(),
                    context,
                    name.as_bytes().to_vec()
                )));
            }
        }
    }
    assert_eq!(slots, [14, 2, 3, 26, 12, 32, 238, 14], "S1–S8 slot census");
    assert_eq!(
        mandatory,
        [14, 4, 6, 52, 24, 64, 476, 24],
        "mutation census"
    );
    assert_eq!(properties.len(), 178, "prefix-property domains");
    assert_eq!(
        controls, 1225,
        "canonical, free-slot and fixed-byte controls"
    );
    eprintln!(
        "Q18 census: seeds={} slots={slots:?} mandatory={mandatory:?} property-domains={} controls={controls} checked={} deviations={}",
        SEEDS.len(),
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
