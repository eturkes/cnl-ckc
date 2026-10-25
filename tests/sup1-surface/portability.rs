use super::base::{self, quote};
use super::model::{self, DATE, atom, edge};
use super::support::*;
use super::surface::results;
use std::path::{Path, PathBuf};

#[derive(Clone)]
struct World {
    name: &'static str,
    patient: bool,
    excluded: bool,
    dates: &'static [&'static str],
}
fn earlier(text: &str) -> bool {
    let ratio = if let Ok(integer) = text.parse::<i128>() {
        Some((integer, 1))
    } else if let Some(pair) = text
        .strip_prefix("rational(")
        .and_then(|s| s.strip_suffix(')'))
    {
        pair.split_once(',')
            .and_then(|(n, d)| Some((n.parse::<i128>().ok()?, d.parse::<i128>().ok()?)))
    } else {
        None
    };
    ratio.is_some_and(|(n, d)| d > 0 && n < i128::from(DATE) * d)
}
fn worlds() -> Vec<World> {
    vec![
        World {
            name: "knowledge-only",
            patient: false,
            excluded: false,
            dates: &[],
        },
        World {
            name: "absent-date",
            patient: true,
            excluded: false,
            dates: &[],
        },
        World {
            name: "before",
            patient: true,
            excluded: false,
            dates: &["20221228"],
        },
        World {
            name: "on",
            patient: true,
            excluded: false,
            dates: &["20221229"],
        },
        World {
            name: "after",
            patient: true,
            excluded: false,
            dates: &["20221230"],
        },
        World {
            name: "two-before-after",
            patient: true,
            excluded: false,
            dates: &["20221228", "20221230"],
        },
        World {
            name: "two-after-before",
            patient: true,
            excluded: false,
            dates: &["20221230", "20221228"],
        },
        World {
            name: "two-no-earlier",
            patient: true,
            excluded: false,
            dates: &["20221229", "20221230"],
        },
        World {
            name: "condition-unmet",
            patient: false,
            excluded: false,
            dates: &["20221228"],
        },
        World {
            name: "scope-active",
            patient: true,
            excluded: true,
            dates: &["20221228"],
        },
        World {
            name: "scope-no-date",
            patient: true,
            excluded: true,
            dates: &[],
        },
        World {
            name: "compound-date",
            patient: true,
            excluded: false,
            dates: &["date(2022,12,28)"],
        },
        World {
            name: "rational-date",
            patient: true,
            excluded: false,
            dates: &["rational(40442457,2)"],
        },
        World {
            name: "malformed-integer",
            patient: true,
            excluded: false,
            dates: &["20220230"],
        },
        World {
            name: "zero",
            patient: true,
            excluded: false,
            dates: &["0"],
        },
        World {
            name: "negative",
            patient: true,
            excluded: false,
            dates: &["-1"],
        },
        World {
            name: "atom",
            patient: true,
            excluded: false,
            dates: &["earlier"],
        },
    ]
}
fn store(cwd: &Path, id: &str, doc: &str, docs: &mut Vec<PathBuf>, manifest: &mut String) {
    let pl = cwd.join(format!("{id}.pl"));
    let payload = cwd.join(format!("{id}.proof"));
    write(&pl, doc);
    write(&payload, "");
    manifest.push_str(&format!("{}\t{}\n", pl.display(), payload.display()));
    docs.push(pl);
}
fn swi(cwd: &Path, docs: &[PathBuf], goals: &[(String, String, bool)]) -> Run {
    let driver = cwd.join("driver.pl");
    let files = std::iter::once(root().join("guidelines/schema.pl"))
        .chain(docs.iter().cloned())
        .map(|p| quote(p.to_str().unwrap()))
        .collect::<Vec<_>>()
        .join(",");
    let calls = goals
        .iter()
        .map(|(label, goal, _)| format!("verdict({}, {})", atom(label), quote(goal)))
        .collect::<Vec<_>>()
        .join(",\n");
    write(
        &driver,
        format!(
            "verdict(Label, Text) :- read_term_from_atom(Text, Goal, []), (once(Goal) -> Answer=yes ; Answer=no), format('~w=~w~n', [Label, Answer]).\nprobe_main :- consult([{files}]), {calls}, halt.\n"
        ),
    );
    command(
        Path::new("swipl"),
        cwd,
        &[
            "-q",
            "-f",
            "none",
            "-F",
            "none",
            "-s",
            driver.to_str().unwrap(),
            "-g",
            "probe_main",
            "-t",
            "halt(9)",
        ],
        b"",
    )
}
#[test]
fn p6_swi_native_portability_all_contract_fact_sets() {
    let scratch = Scratch::new("portability");
    let mut checks = Checks::default();
    let policy_ids = [
        "sup-should",
        "sup-must",
        "sup-may",
        "sup-under-can",
        "sup-under-negative",
        "sup-mixed",
        "sup-can-only",
        "sup-asserted",
        "sup-scoped",
    ];
    for world in worlds() {
        let cwd = scratch.0.join(world.name);
        std::fs::create_dir_all(&cwd).unwrap();
        let (mut docs, mut manifest) = (Vec::new(), String::new());
        // P2 establishes equality with the emitted product; an independent product here
        // also exercises native admission when the seed fork refuses the new table.
        for id in policy_ids {
            let p = &results()[id].projection;
            store(&cwd, id, &p.document(true, true), &mut docs, &mut manifest);
        }
        let independent = results()["sup-may"]
            .projection
            .document(false, true)
            .replace("sup-may", "sup-independent");
        store(&cwd, "independent", &independent, &mut docs, &mut manifest);
        let transitive = model::structural("dependent", &["guideline_property(actual,A,depends,B) :- guideline_event('$guideline_id'(context,'sup-should',1,box(1),A),B,wait)".into()]);
        store(&cwd, "dependent", &transitive, &mut docs, &mut manifest);
        let mut facts: Vec<String> = world
            .dates
            .iter()
            .map(|d| format!("guideline_evaluation_date({d})"))
            .collect();
        if world.patient {
            facts.extend(
                [
                    "guideline_entity(actual,p,patient,countable)",
                    "guideline_cardinality(actual,p,na,eq,1)",
                    "guideline_event(actual,e,sleep)",
                    "guideline_arg(actual,e,1,p)",
                ]
                .map(str::to_owned),
            );
        }
        if world.excluded {
            facts.push(format!("guideline_exclusion({})", atom(base::KEY)));
        }
        if !facts.is_empty() {
            store(
                &cwd,
                "consumer",
                &model::structural("consumer", &facts),
                &mut docs,
                &mut manifest,
            );
        }
        write(cwd.join("manifest.tsv"), manifest);
        let enabled = world.patient && world.dates.iter().any(|d| earlier(d));
        let goals = vec![
            ("should".into(), edge("sup-should", "should", 1, None), enabled),
            ("must".into(), edge("sup-must", "must", 1, None), enabled),
            ("may".into(), edge("sup-may", "may", 1, None), enabled),
            ("nested-should".into(), edge("sup-under-can", "should", 2, Some(1)), enabled),
            ("nested-can-also-guarded".into(), edge("sup-under-can", "can", 1, None), enabled),
            ("negative-should".into(), edge("sup-under-negative", "should", 2, Some(1)), enabled),
            ("whole-payload".into(), "guideline_event('$guideline_id'(context,'sup-should',1,box(1),D),E,wait)".into(), enabled),
            ("mixed-asserted".into(), "guideline_event(actual,'$guideline_id'(product,'sup-mixed',1,ref(3),D),work)".into(), enabled),
            ("can-only".into(), edge("sup-can-only", "can", 1, None), world.patient),
            ("scoped".into(), edge("sup-scoped", "should", 1, None), enabled && !world.excluded),
            ("independent".into(), edge("sup-independent", "may", 1, None), world.patient),
            ("transitive".into(), "guideline_property(actual,D,depends,E)".into(), enabled),
            ("record".into(), format!("guideline_supersession('sup-should',1,S,{DATE})"), true),
            ("caller-continuation".into(), "','(guideline_operator(actual,C,should),','(guideline_event(C,E,wait),guideline_arg(C,E,1,p)))".into(), enabled),
        ];
        let expected: String = goals
            .iter()
            .map(|(label, _, yes)| format!("{label}={}\n", if *yes { "yes" } else { "no" }))
            .collect();
        let got = swi(&cwd, &docs, &goals);
        checks.output(
            &got,
            &Run::ok(expected),
            &format!("{}/SWI consult, empty stderr, edge oracle", world.name),
        );
        for (label, goal, yes) in goals {
            let id = format!("{}-{label}", world.name);
            let query = model::query(&id, &goal);
            write(cwd.join("query.pl"), &query);
            let got = native(&cwd, &["v1", "answer", "manifest.tsv", "query.pl"]);
            checks.output(
                &got,
                &Run::ok(model::answer(&id, &query, yes)),
                &format!("{}/native=edge oracle", id),
            );
        }
        let scan = native(&cwd, &["v1", "recursion-check", "manifest.tsv"]);
        checks.require(
            scan.success() && scan.stdout.starts_with("ace_to_pl recursion ok "),
            format!(
                "{} finite composition recursion scan: {}",
                world.name,
                scan.describe()
            ),
        );
    }
    checks.finish("P6 17 finite worlds × 14 goals; native = SWI = direct date/patient oracle");
}
