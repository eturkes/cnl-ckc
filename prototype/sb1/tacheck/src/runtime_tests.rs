use crate::runtime::B1Provider;
use sb1_checker::orchestrator::{ClauseId, ClauseSource, SliceProvider};
use std::fs;
use std::path::{Path, PathBuf};

struct Case(PathBuf);
impl Case {
    fn new(name: &str, text: &str) -> Self {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(3)
            .unwrap();
        let path = root.join(format!(
            "rust/target/bench/builder-20-runtime-{}-{name}",
            std::process::id()
        ));
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::create_dir(&path).unwrap();
        fs::write(path.join("kb.pl.in"), text).unwrap();
        fs::write(
            path.join("composition.tsv"),
            "# patient_documents\t1\nkb.pl.in\nmissing-patient.pl.in\n",
        )
        .unwrap();
        Self(path)
    }
    fn sources(&self) -> Vec<ClauseSource> {
        let p = sb1_ta::source::Program::load_knowledge(&self.0, &self.0.join("composition.tsv"))
            .unwrap();
        p.clauses
            .iter()
            .enumerate()
            .map(|(i, c)| ClauseSource {
                id: ClauseId {
                    document: c.document,
                    ordinal: c.ordinal,
                },
                text: p.render_clause(i).trim_end_matches('.').to_owned(),
            })
            .collect()
    }
    fn provider(&self, all: bool) -> B1Provider {
        B1Provider::new(
            self.0.clone(),
            self.0.join("composition.tsv"),
            self.0.join("arm"),
            all,
        )
        .unwrap()
    }
}
impl Drop for Case {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            fs::remove_dir_all(&self.0).unwrap();
        }
    }
}

#[test]
fn adapter_checks_original_binding_before_solver_and_excludes_patient_io() {
    let case = Case::new("binding", "guideline_entity(actual,a,seed,countable).\n");
    let mut sources = case.sources();
    sources[0].text = "guideline_entity(actual,b,seed,countable)".into();
    let mut provider = case.provider(false);
    let failure = provider.prepare(&sources).unwrap_err();
    assert_eq!(failure.rc, 1);
    assert!(failure.message.starts_with("refused (certificate)"));
    let phases = fs::read_to_string(case.0.join("arm/ta-phases.tsv")).unwrap();
    assert!(phases.contains("ta-shape-binding\t"));
    assert!(!phases.contains("ta-solver\t"));
}

#[test]
fn adapter_prepares_once_rechecks_patient_and_uses_original_ids() {
    let case = Case::new(
        "calls",
        "guideline_entity(actual,X,out,countable) :- guideline_entity(actual,X,seed,countable).\n",
    );
    let mut provider = case.provider(false);
    assert_eq!(provider.select(&[], b"sb1-q-all 0\n").unwrap_err().rc, 3);
    provider.prepare(&case.sources()).unwrap();
    let info = fs::read_to_string(case.0.join("arm/ta-info.tsv")).unwrap();
    assert!(
        info.contains("hint_version\t1\n")
            && info.contains("proposed_refinement\tbase\n")
            && info.contains("checked_admission\tadmitted\n")
    );
    let patient = [ClauseSource {
        id: ClauseId {
            document: 2,
            ordinal: 1,
        },
        text: "guideline_entity(actual,patient,seed,countable)".into(),
    }];
    assert_eq!(
        provider.select(&patient, b"sb1-q-all 0\n").unwrap(),
        vec![ClauseId {
            document: 1,
            ordinal: 1
        }]
    );
    assert_eq!(provider.prepare(&case.sources()).unwrap_err().rc, 1);
    let rule = [ClauseSource { id:ClauseId { document:2,ordinal:1 }, text:"guideline_entity(actual,patient,seed,countable) :- guideline_entity(actual,patient,out,countable)".into() }];
    let error = provider.select(&rule, b"sb1-q-all 0\n").unwrap_err();
    assert_eq!(error.rc, 1);
    assert!(error.message.starts_with("refused (admission)"));
}
