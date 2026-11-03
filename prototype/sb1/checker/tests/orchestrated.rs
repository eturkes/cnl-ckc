use sb1_checker::orchestrator::{
    ClauseId, ClauseSource, OrchestratedRequest, PreparedProgram, Provider, SliceProvider,
    orchestrate,
};
use sb1_checker::{Failure, Request, SourceCodec, check};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

const ADULT: &str = include_str!("../../../../tests/consumer/adult.pl.in");
const CLAUSES: &str = "guideline_property(actual,a,seed,true).\n\
guideline_property(actual,A,p,true) :- guideline_property(actual,A,seed,true).\n\
guideline_property(actual,A,good,true) :- guideline_property(actual,A,p,true), \\+ (guideline_property(actual,A,edge,B), guideline_property(actual,B,bad,true)).\n\
guideline_operator(actual,box,should) :- guideline_property(actual,a,good,true).\n\
guideline_property(actual,dormant,noise,true).\n";
fn document(id: &str, clauses: &str) -> String {
    let prefix = ADULT
        .split("% S1:")
        .next()
        .unwrap()
        .lines()
        .filter(|line| !line.starts_with('%'))
        .collect::<Vec<_>>()
        .join("\n")
        .replace(
            "guideline_document(adult,",
            &format!("guideline_document({id},"),
        );
    format!(
        "% {id}.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.\n{prefix}\n% S1: Prototype fixture.\n{clauses}"
    )
}
fn question() -> String {
    format!(
        "% q-one compiled from ACE question by ace_to_pl question mode; do not edit.\n'$guideline_query'(v2,'q-one',ace_sha256('{}'),ulex(none),vocabulary(none)).\n% Q1: Prototype fixture?\n'$guideline_query_projection'(goal(guideline_property(actual,A,good,true)),answers([answer(A,noun(patient,countable))])).\n",
        "0".repeat(64)
    )
}
struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("target/orchestrated")
            .join(format!(
                "{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
        fs::create_dir_all(path.join("guidelines")).unwrap();
        fs::write(
            path.join("guidelines/schema.pl"),
            include_bytes!("../../../../guidelines/schema.pl"),
        )
        .unwrap();
        fs::write(path.join("knowledge.pl"), document("knowledge", CLAUSES)).unwrap();
        fs::write(
            path.join("patient.pl"),
            document("patient", "guideline_property(actual,a,patient,true).\n"),
        )
        .unwrap();
        fs::write(path.join("composition.tsv"), "knowledge.pl\npatient.pl\n").unwrap();
        fs::write(path.join("query.pl"), question()).unwrap();
        Self(path)
    }
    fn request(&self, name: &str, provider: Provider) -> OrchestratedRequest {
        let output = self.0.join(name);
        fs::create_dir(&output).unwrap();
        OrchestratedRequest {
            root: self.0.clone(),
            composition: self.0.join("composition.tsv"),
            output,
            query: self.0.join("query.pl"),
            provider,
            producer: Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../producer.prolog")
                .canonicalize()
                .unwrap(),
            swipl: "swipl".into(),
        }
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

#[test]
fn all_reproduces_standalone_and_static_keeps_original_sparse_ids() {
    let f = Fixture::new();
    let baseline = f.0.join("baseline");
    let producer = Path::new(env!("CARGO_MANIFEST_DIR")).join("../producer.prolog");
    let out = Command::new("swipl")
        .args(["-q", "-f", "none", "-F", "none", "-s"])
        .arg(producer)
        .args(["-g", "sb1_main", "-t", "halt(9)", "--", "--root"])
        .arg(&f.0)
        .arg(f.0.join("composition.tsv"))
        .arg(&baseline)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(out.stdout.is_empty() && out.stderr.is_empty());
    let original = check(&Request {
        root: f.0.clone(),
        composition: f.0.join("composition.tsv"),
        model: baseline.join("model.txt"),
        certificate: baseline.join("certificate.txt"),
        query: Some(f.0.join("query.pl")),
    });
    assert_eq!(original.rc, 0, "{}", original.stderr);
    assert!(original.stdout.contains("sol([a])"));
    for (name, provider) in [("all", Provider::All), ("static", Provider::Static)] {
        let request = f.request(name, provider);
        let actual = orchestrate(&request, SourceCodec::Parsed).unwrap();
        assert_eq!(actual, original.stdout);
        let certificate = fs::read_to_string(request.output.join("certificate.txt")).unwrap();
        let baseline_certificate = fs::read_to_string(baseline.join("certificate.txt")).unwrap();
        assert_eq!(
            certificate.lines().nth(1),
            baseline_certificate.lines().nth(1),
            "incremental prepared identity = independent full-composition identity"
        );
        let phases = fs::read_to_string(request.output.join("orchestrator-phases.tsv")).unwrap();
        assert_eq!(phases.lines().count(), 14);
        assert_eq!(phases.lines().filter(|l| l.ends_with("\tload")).count(), 4);
        let counters = fs::read_to_string(request.output.join("checker-counters.tsv")).unwrap();
        assert!(counters.contains("logical\tdocuments\t2\n"));
        assert!(counters.contains("logical\tclauses\t10\n"));
        if name == "static" {
            assert_eq!(
                fs::read_to_string(request.output.join("selected-clauses.tsv")).unwrap(),
                "1:3\n1:4\n1:5\n2:1\n2:2\n2:3\n"
            );
            let certificate = fs::read_to_string(request.output.join("certificate.txt")).unwrap();
            assert!(certificate.lines().skip(2).all(|l| {
                ["1:3", "1:4", "1:5", "2:1", "2:2", "2:3"].contains(&l.split('\t').nth(1).unwrap())
            }));
        } else {
            assert_eq!(
                fs::read(request.output.join("model.txt")).unwrap(),
                fs::read(baseline.join("model.txt")).unwrap()
            );
        }
    }
    for (name, trace, file_sink) in [
        ("cli", false, false),
        ("cli-trace", true, false),
        ("closed-sink", true, true),
    ] {
        let request = f.request(name, Provider::Static);
        let mut command = Command::new(env!("CARGO_BIN_EXE_checker"));
        command
            .args(["orchestrate", "--provider", "static", "--producer"])
            .arg(&request.producer)
            .arg("--root")
            .arg(&request.root)
            .arg(&request.composition)
            .arg(&request.output)
            .arg(&request.query)
            .env_remove("SB1_STAGE_TRACE")
            .env_remove("CKC_BENCH_SINK");
        if file_sink {
            command.env("CKC_BENCH_SINK", "closed-file-v1");
        }
        if trace {
            command.env("SB1_STAGE_TRACE", "1");
        }
        let out = command.output().unwrap();
        assert_eq!(
            out.status.code(),
            Some(0),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        assert!(out.stderr.is_empty());
        if file_sink {
            assert!(out.stdout.is_empty());
            assert_eq!(
                fs::read(request.output.join("answers.txt")).unwrap(),
                original.stdout.as_bytes()
            );
            assert!(
                fs::read_to_string(request.output.join("sink-complete.tsv"))
                    .unwrap()
                    .starts_with("sb1-sink 1\nclosed_unix_ns\t")
            );
        } else {
            assert_eq!(out.stdout, original.stdout.as_bytes());
        }
        let path = request.output.join("orchestrator-trace.tsv");
        assert_eq!(path.exists(), trace);
        if trace {
            let text = fs::read_to_string(path).unwrap();
            let rows = text
                .lines()
                .skip(1)
                .map(|line| line.split('\t').collect::<Vec<_>>())
                .collect::<Vec<_>>();
            assert!(rows.iter().all(|row| row.len() == 4));
            assert_eq!(rows.first().unwrap()[0], "call_begin");
            assert_eq!(rows.last().unwrap()[0], "pre_process_exit");
            assert!(rows.windows(2).all(
                |pair| pair[0][1].parse::<u64>().unwrap() <= pair[1][1].parse::<u64>().unwrap()
            ));
            for event in [
                "identity_begin",
                "identity_end",
                "slice_select_end",
                "producer_exited",
                "model_certificate_decode_end",
                "check_end",
                "arena_rewound",
                "metrics_write_end",
                if file_sink {
                    "answer_sink_closed"
                } else {
                    "stdout_flushed"
                },
            ] {
                assert!(rows.iter().any(|row| row[0] == event), "missing {event}");
            }
        }
    }
}

#[test]
fn prepared_program_reuses_knowledge_and_reads_each_patient_afresh() {
    let f = Fixture::new();
    let mut prepared = PreparedProgram::prepare(
        &f.0,
        &f.0.join("composition.tsv"),
        SourceCodec::Parsed,
        None,
    )
    .unwrap();
    let first = prepared
        .call(&f.request("first", Provider::Static), None)
        .unwrap();
    assert!(first.contains("sol([a])"));
    fs::write(
        f.0.join("knowledge.pl"),
        "not the prepared knowledge base\n",
    )
    .unwrap();
    fs::write(
        f.0.join("patient.pl"),
        document(
            "patient",
            "guideline_property(actual,a,edge,b).\nguideline_property(actual,b,bad,true).\n",
        ),
    )
    .unwrap();
    let second = prepared
        .call(&f.request("second", Provider::Static), None)
        .unwrap();
    assert!(second.contains("solutions([])"), "{second}");
    assert_eq!(
        prepared
            .phases
            .iter()
            .filter(|p| p.name == "load_parse")
            .count(),
        1
    );
    assert_eq!(
        prepared
            .phases
            .iter()
            .filter(|p| p.name == "patient")
            .count(),
        2
    );
}

struct External {
    knowledge: Vec<ClauseId>,
    prepares: usize,
    selects: usize,
}
impl SliceProvider for External {
    fn prepare(&mut self, knowledge: &[ClauseSource]) -> Result<(), Failure> {
        self.knowledge = knowledge.iter().map(|s| s.id).collect();
        self.prepares += 1;
        Ok(())
    }
    fn select(&mut self, patient: &[ClauseSource], query: &[u8]) -> Result<Vec<ClauseId>, Failure> {
        assert_eq!(query, question().as_bytes());
        self.selects += 1;
        let mut ids = self.knowledge.clone();
        ids.extend(patient.iter().map(|s| s.id));
        Ok(ids)
    }
}
#[test]
fn external_provider_prepares_once_and_returns_only_original_ids() {
    let f = Fixture::new();
    let mut provider = External {
        knowledge: Vec::new(),
        prepares: 0,
        selects: 0,
    };
    let mut prepared = PreparedProgram::prepare(
        &f.0,
        &f.0.join("composition.tsv"),
        SourceCodec::Parsed,
        Some(&mut provider),
    )
    .unwrap();
    for name in ["first", "second"] {
        let actual = prepared
            .call(&f.request(name, Provider::Static), Some(&mut provider))
            .unwrap();
        assert!(actual.contains("sol([a])"));
    }
    assert_eq!((provider.prepares, provider.selects), (1, 2));
    assert!(
        prepared
            .counters
            .render()
            .contains("logical\ttyped_analysis_hook\t1\n")
    );
}

#[derive(Default)]
struct KnowledgeOnly {
    knowledge: Vec<ClauseId>,
    patient_documents: Vec<usize>,
}
impl SliceProvider for KnowledgeOnly {
    fn prepare(&mut self, knowledge: &[ClauseSource]) -> Result<(), Failure> {
        self.knowledge = knowledge.iter().map(|source| source.id).collect();
        Ok(())
    }
    fn select(&mut self, patient: &[ClauseSource], _: &[u8]) -> Result<Vec<ClauseId>, Failure> {
        self.patient_documents = patient.iter().map(|source| source.id.document).collect();
        self.patient_documents.sort_unstable();
        self.patient_documents.dedup();
        Ok(self.knowledge.clone())
    }
}

#[test]
fn declared_patient_suffix_is_fresh_and_absent_from_preparation() {
    let f = Fixture::new();
    fs::write(
        f.0.join("knowledge.pl"),
        document(
            "knowledge",
            "guideline_property(actual,A,good,true) :- guideline_property(actual,A,seed,true).\n",
        ),
    )
    .unwrap();
    fs::write(
        f.0.join("composition.tsv"),
        "knowledge.pl\nprojection.pl\npatient.pl\n",
    )
    .unwrap();
    fs::write(
        f.0.join("manifest.tsv"),
        "family\tfixture\npatient_documents\t2\n",
    )
    .unwrap();
    fs::remove_file(f.0.join("patient.pl")).unwrap();
    let mut provider = KnowledgeOnly::default();
    let mut prepared = PreparedProgram::prepare(
        &f.0,
        &f.0.join("composition.tsv"),
        SourceCodec::Parsed,
        Some(&mut provider),
    )
    .unwrap();
    assert!(provider.knowledge.iter().all(|id| id.document == 1));
    assert_eq!(provider.knowledge.len(), 3);
    for (name, value) in [("first", "a"), ("second", "b")] {
        fs::write(
            f.0.join("projection.pl"),
            document(
                "projection",
                &format!("guideline_property(actual,{value},seed,true).\n"),
            ),
        )
        .unwrap();
        fs::write(
            f.0.join("patient.pl"),
            document(
                "patient",
                "guideline_property(actual,patient,intake,true).\n",
            ),
        )
        .unwrap();
        let request = f.request(name, Provider::Static);
        let answer = prepared.call(&request, Some(&mut provider)).unwrap();
        assert!(answer.contains(&format!("sol([{value}])")), "{answer}");
        assert_eq!(provider.patient_documents, [2, 3]);
        let selected = fs::read_to_string(request.output.join("selected-clauses.tsv")).unwrap();
        for id in ["2:1", "2:2", "2:3", "3:1", "3:2", "3:3"] {
            assert!(
                selected.lines().any(|line| line == id),
                "patient fact {id} missing"
            );
        }
        let baseline = f.0.join(format!("baseline-{name}"));
        let out = Command::new("swipl")
            .args(["-q", "-f", "none", "-F", "none", "-s"])
            .arg(&request.producer)
            .args(["-g", "sb1_main", "-t", "halt(9)", "--", "--root"])
            .arg(&f.0)
            .arg(&request.composition)
            .arg(&baseline)
            .output()
            .unwrap();
        assert!(
            out.status.success() && out.stderr.is_empty(),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        let actual = fs::read_to_string(request.output.join("certificate.txt")).unwrap();
        let expected = fs::read_to_string(baseline.join("certificate.txt")).unwrap();
        assert_eq!(
            actual.lines().nth(1),
            expected.lines().nth(1),
            "identity frames every patient document in order"
        );
    }
}

#[test]
fn any_patient_suffix_document_with_a_rule_refuses_per_call() {
    let f = Fixture::new();
    fs::write(
        f.0.join("composition.tsv"),
        "knowledge.pl\nprojection.pl\npatient.pl\n",
    )
    .unwrap();
    fs::write(f.0.join("manifest.tsv"), "patient_documents\t2\n").unwrap();
    fs::write(f.0.join("projection.pl"), document("projection", CLAUSES)).unwrap();
    let mut prepared = PreparedProgram::prepare(
        &f.0,
        &f.0.join("composition.tsv"),
        SourceCodec::Parsed,
        None,
    )
    .unwrap();
    let error = prepared
        .call(&f.request("rule-in-projection", Provider::All), None)
        .unwrap_err();
    assert_eq!(error.rc, 1);
    assert!(
        error.message.contains("patient contains a rule"),
        "{}",
        error.message
    );
}

#[test]
fn invalid_or_duplicate_patient_counts_refuse_instead_of_loading_intakes() {
    let f = Fixture::new();
    for text in [
        "patient_documents\t0\n",
        "patient_documents\t-1\n",
        "patient_documents\tx\n",
        "patient_documents\t3\n",
        "patient_documents\t1\npatient_documents\t1\n",
    ] {
        fs::write(f.0.join("manifest.tsv"), text).unwrap();
        let error = PreparedProgram::prepare(
            &f.0,
            &f.0.join("composition.tsv"),
            SourceCodec::Parsed,
            None,
        )
        .err()
        .expect("bad count must refuse");
        assert_eq!(error.rc, 2, "{}", error.message);
    }
}

#[test]
fn patient_rules_and_unknown_or_duplicate_selection_ids_refuse() {
    let f = Fixture::new();
    let selection = f.0.join("ids.tsv");
    for (name, text) in [("unknown", "9:1\n"), ("duplicate", "1:3\n1:3\n")] {
        fs::write(&selection, text).unwrap();
        let error = orchestrate(
            &f.request(name, Provider::Ids(selection.clone())),
            SourceCodec::Parsed,
        )
        .unwrap_err();
        assert_eq!(error.rc, 2);
    }
    fs::write(f.0.join("patient.pl"), document("patient", CLAUSES)).unwrap();
    let error = orchestrate(
        &f.request("patient-rule", Provider::All),
        SourceCodec::Parsed,
    )
    .unwrap_err();
    assert_eq!(error.rc, 1);
    assert!(error.message.contains("patient contains a rule"));
}
