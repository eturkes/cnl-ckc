// Contract sb D2 MX + F4 contrasts (scientist-3 round 4 G3/G4): each contrast model is
// diffed against its base point's exact recipe model; the declared delta must hold, the
// shard contrast's cross-guideline delta is the treatment, reported.
use super::*;
use std::collections::BTreeSet;

pub const NAMES: [(&str, &str); 4] = [
    ("MX", "mx-bridge-b4"),
    ("MX", "mx-key"),
    ("MX", "mx-shard"),
    ("F4", "f4-l2"),
];
const KEY_IDENTITY: &str = "'sb-p-mx',17,";
const L2_TARGET: &str =
    "guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s60-11',";

fn listing(c: &Config, p: &Point) -> Result<BTreeMap<String, String>, String> {
    let dir = relative(&c.root, &p.dir(&c.root))?;
    let mut out = BTreeMap::new();
    for line in string(&p.dir(&c.root).join("composition.tsv"))?.lines() {
        let suffix = line
            .strip_prefix(&dir)
            .and_then(|s| s.strip_prefix('/'))
            .ok_or("contrast composition lists a file outside its instance")?;
        out.insert(suffix.to_owned(), digest(&read(&c.root.join(line))?));
    }
    Ok(out)
}
fn facts(path: &Path) -> Result<BTreeSet<String>, String> {
    Ok(string(path)?.lines().map(str::to_owned).collect())
}
fn boxed(fact: &str) -> bool {
    fact.starts_with("guideline_operator(actual,")
        && [",should).", ",must).", ",may)."]
            .iter()
            .any(|o| fact.ends_with(o))
}
fn namespace(c: &Config, p: &Point, slot: usize) -> Result<String, String> {
    let m = manifest(&p.base().dir(&c.root).join("manifest.tsv"))?;
    let ns: usize = match m.get(&format!("variation_namespace:{slot:04}")) {
        Some(v) => v.parse().map_err(|_| "variation namespace")?,
        None => slot,
    };
    Ok(if p.seed == 0 {
        format!("sb{ns:04}")
    } else {
        format!("sb{ns:04}s{:016x}", p.seed)
    })
}
struct Inputs<'a> {
    name: &'a str,
    // Composition listings (path inside the instance → file sha256) + recipe models.
    want: BTreeMap<String, String>,
    got: BTreeMap<String, String>,
    before: BTreeSet<String>,
    after: BTreeSet<String>,
    // The shard contrast's slot-6 namespace prefix (seed-dependent).
    namespace: String,
}
struct Delta {
    rows: Vec<(&'static str, String)>,
    removed: BTreeSet<String>,
    added: BTreeSet<String>,
}
fn judge(i: &Inputs) -> Result<Delta, String> {
    let (want, got, before, after) = (&i.want, &i.got, &i.before, &i.after);
    let removed_files: Vec<_> = want
        .keys()
        .filter(|k| !got.contains_key(*k))
        .cloned()
        .collect();
    let added_files: Vec<_> = got
        .keys()
        .filter(|k| !want.contains_key(*k))
        .cloned()
        .collect();
    let changed_files: Vec<_> = want
        .iter()
        .filter(|(k, v)| got.get(*k).is_some_and(|g| g != *v))
        .map(|(k, _)| k.clone())
        .collect();
    let removed: BTreeSet<_> = before.difference(after).cloned().collect();
    let added: BTreeSet<_> = after.difference(before).cloned().collect();
    let removed_boxes = removed.iter().filter(|f| boxed(f)).count();
    let added_boxes = added.iter().filter(|f| boxed(f)).count();
    let files = || {
        format!("files removed {removed_files:?} added {added_files:?} changed {changed_files:?}")
    };
    let declared = match i.name {
        "mx-bridge-b4" | "f4-l2" => {
            let [file] = removed_files.as_slice() else {
                return Err(files());
            };
            if !added_files.is_empty() || !changed_files.is_empty() {
                return Err(files());
            }
            let docid = file
                .rsplit('/')
                .next()
                .and_then(|f| f.strip_suffix(".pl"))
                .ok_or("removed document name")?;
            let quoted = format!("'{docid}'");
            let mention: BTreeSet<_> = before
                .iter()
                .filter(|f| f.contains(&quoted))
                .cloned()
                .collect();
            if removed != mention || !added.is_empty() {
                return Err(format!(
                    "{} removed facts vs {} naming {docid}; {} added",
                    removed.len(),
                    mention.len(),
                    added.len()
                ));
            }
            if i.name == "f4-l2" {
                let target = |s: &BTreeSet<String>| {
                    s.iter()
                        .filter(|f| f.starts_with(L2_TARGET) && f.ends_with(",must)."))
                        .count()
                };
                if target(before) == 0 || target(after) != 0 {
                    return Err("the s60-11 MUST target is absent from the base or survives".into());
                }
            } else if removed_boxes == 0 {
                return Err("no should/must/may box removed".into());
            }
            format!("removed facts = every base fact naming {docid}; none added")
        }
        "mx-key" => {
            let projection = |f: &String| f == "patient.pl" || f.starts_with("intakes/");
            if !removed_files.is_empty()
                || !added_files.is_empty()
                || changed_files.is_empty()
                || !changed_files.iter().all(projection)
            {
                return Err(files());
            }
            let mention: BTreeSet<_> = before
                .iter()
                .filter(|f| f.contains(KEY_IDENTITY))
                .cloned()
                .collect();
            if removed != mention || !added.is_empty() || removed_boxes == 0 {
                return Err(format!(
                    "{} removed facts vs {} naming a sentence-17 identity; {} added; {removed_boxes} boxes removed",
                    removed.len(),
                    mention.len(),
                    added.len()
                ));
            }
            "removed facts = every base fact naming a sb-p-mx sentence-17 identity; none added"
                .into()
        }
        "mx-shard" => {
            // Same slot + namespace ⇒ the copy keeps its file names; the shared-hub
            // spelling becomes padding spelling and the hub's intake projection leaves.
            let copy = |f: &String| f.starts_with("copies/0006/");
            if changed_files.is_empty()
                || !changed_files.iter().all(copy)
                || !added_files.iter().all(copy)
                || removed_files != ["intakes/0006/sb-p-mx.pl"]
            {
                return Err(files());
            }
            let own = |s: &BTreeSet<String>| s.iter().filter(|f| f.contains(&i.namespace)).count();
            format!(
                "treatment reported: slot-6 namespace {}: {} removed + {} added facts; other: {} removed + {} added",
                i.namespace,
                own(&removed),
                own(&added),
                removed.len() - own(&removed),
                added.len() - own(&added)
            )
        }
        other => return Err(format!("unknown contrast {other}")),
    };
    let rows = vec![
        ("contrast", i.name.to_owned()),
        ("removed_files", removed_files.len().to_string()),
        ("added_files", added_files.len().to_string()),
        ("changed_files", changed_files.len().to_string()),
        ("removed_facts", removed.len().to_string()),
        ("added_facts", added.len().to_string()),
        ("removed_boxes", removed_boxes.to_string()),
        ("added_boxes", added_boxes.to_string()),
        ("declared", declared),
    ];
    Ok(Delta {
        rows,
        removed,
        added,
    })
}
fn delta(c: &Config, p: &Point) -> Result<Delta, String> {
    let name = p.contrast.ok_or("contrast point")?;
    let base = p.base();
    let model = |q: &Point| q.oracle(&c.root).join("union/model.out");
    let mut d = judge(&Inputs {
        name,
        want: listing(c, &base)?,
        got: listing(c, p)?,
        before: facts(&model(&base))?,
        after: facts(&model(p))?,
        namespace: namespace(c, p, 6)?,
    })
    .map_err(|why| format!("{}: contrast delta: {why}", p.id()))?;
    d.rows.insert(1, ("base", base.id()));
    Ok(d)
}
fn lines(set: &BTreeSet<String>) -> String {
    set.iter().map(|f| format!("{f}\n")).collect()
}
fn receipt(d: &Delta) -> String {
    d.rows.iter().map(|(k, v)| format!("{k}\t{v}\n")).collect()
}
pub(super) fn run(c: &Config, p: &Point) -> Result<(), String> {
    let d = delta(c, p)?;
    let out = p.oracle(&c.root).join("contrast");
    write(&out.join("removed.out"), lines(&d.removed))?;
    write(&out.join("added.out"), lines(&d.added))?;
    write(&out.join("receipt.tsv"), receipt(&d))?;
    check(c, p)
}
pub(super) fn check(c: &Config, p: &Point) -> Result<(), String> {
    let d = delta(c, p)?;
    let out = p.oracle(&c.root).join("contrast");
    if string(&out.join("receipt.tsv"))? != receipt(&d)
        || string(&out.join("removed.out"))? != lines(&d.removed)
        || string(&out.join("added.out"))? != lines(&d.added)
    {
        return Err(format!("{}: contrast receipt differs", p.id()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set(xs: &[&str]) -> BTreeSet<String> {
        xs.iter().map(|s| s.to_string()).collect()
    }
    fn files(xs: &[(&str, &str)]) -> BTreeMap<String, String> {
        xs.iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }
    const KEEP: &str = "guideline_entity(actual,'$guideline_id'(product,'sb-p-mx',2,ref(1),[]),patient,countable).";
    const READER_BOX: &str =
        "guideline_operator(actual,'$guideline_id'(context,'sb-f3-b4-reader',1,ref(3),[]),should).";
    const READER_DOC: &str = "guideline_document('sb-f3-b4-reader',x).";
    const S60_BOX: &str = "guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s60-11',2,ref(4),['$guideline_id'(entity,'sb-f4-l2',1,ref(1),[])]),must).";
    const L2_DOC: &str = "guideline_document('sb-f4-l2',x).";
    const KEY_FACT: &str =
        "guideline_event(actual,'$guideline_id'(product,'sb-p-mx',17,ref(2),[]),prescribe).";
    const KEY_BOX: &str = "guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec11',1,ref(3),['$guideline_id'(product,'sb-p-mx',17,ref(1),[])]),should).";

    fn inputs<'a>(
        name: &'a str,
        want: &[(&str, &str)],
        got: &[(&str, &str)],
        before: &[&str],
        after: &[&str],
    ) -> Inputs<'a> {
        Inputs {
            name,
            want: files(want),
            got: files(got),
            before: set(before),
            after: set(after),
            namespace: "sb0006".into(),
        }
    }
    fn bridge(before: &[&str], after: &[&str], got: &[(&str, &str)]) -> Result<Delta, String> {
        judge(&inputs(
            "mx-bridge-b4",
            &[
                ("copies/0001/a.pl", "1"),
                ("copies/0003/sb-f3-b4-reader.pl", "2"),
            ],
            got,
            before,
            after,
        ))
    }
    #[test]
    fn document_removal_delta_holds_and_each_plant_fires() {
        let full = [KEEP, READER_BOX, READER_DOC];
        let one = [("copies/0001/a.pl", "1")];
        let d = bridge(&full, &[KEEP], &one).unwrap();
        assert_eq!(d.removed.len(), 2);
        assert!(
            d.rows
                .iter()
                .any(|(k, v)| *k == "removed_boxes" && v == "1")
        );
        // Plants: a fact added, a reader fact surviving, a foreign fact removed, no box
        // removed, a changed or a second removed file.
        assert!(bridge(&full, &[KEEP, "guideline_x(new)."], &one).is_err());
        assert!(bridge(&full, &[KEEP, READER_DOC], &one).is_err());
        assert!(bridge(&full, &[], &one).is_err());
        assert!(bridge(&[KEEP, READER_DOC], &[KEEP], &one).is_err());
        assert!(bridge(&full, &[KEEP], &[("copies/0001/a.pl", "9")]).is_err());
        assert!(bridge(&full, &[KEEP], &[]).is_err());
    }
    #[test]
    fn chain_contrast_requires_the_s60_target_to_vanish() {
        let want = [
            ("copies/0001/cdc.pl", "1"),
            ("copies/0001/sb-f4-l2.pl", "2"),
        ];
        let got = [("copies/0001/cdc.pl", "1")];
        let l2 =
            |before: &[&str], after: &[&str]| judge(&inputs("f4-l2", &want, &got, before, after));
        assert!(l2(&[KEEP, S60_BOX, L2_DOC], &[KEEP]).is_ok());
        // Plants: the target absent from the base; a second s60-11 support surviving.
        assert!(l2(&[KEEP, L2_DOC], &[KEEP]).is_err());
        let other = "guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s60-11',2,ref(4),[x]),must).";
        assert!(l2(&[KEEP, S60_BOX, L2_DOC, other], &[KEEP, other]).is_err());
    }
    #[test]
    fn key_contrast_removes_exactly_the_sentence_17_closure() {
        let want = [
            ("patient.pl", "1"),
            ("intakes/0002/sb-p-mx.pl", "2"),
            ("copies/0001/a.pl", "3"),
        ];
        let got = [
            ("patient.pl", "7"),
            ("intakes/0002/sb-p-mx.pl", "8"),
            ("copies/0001/a.pl", "3"),
        ];
        let key = |got: &[(&str, &str)], before: &[&str], after: &[&str]| {
            judge(&inputs("mx-key", &want, got, before, after))
        };
        assert!(key(&got, &[KEEP, KEY_FACT, KEY_BOX], &[KEEP]).is_ok());
        // Plants: a sentence-17 fact surviving; no box removed; a knowledge file changed;
        // a fact added (a negation newly satisfied).
        assert!(key(&got, &[KEEP, KEY_FACT, KEY_BOX], &[KEEP, KEY_FACT]).is_err());
        assert!(key(&got, &[KEEP, KEY_FACT], &[KEEP]).is_err());
        let knowledge = [
            ("patient.pl", "7"),
            ("intakes/0002/sb-p-mx.pl", "8"),
            ("copies/0001/a.pl", "4"),
        ];
        assert!(key(&knowledge, &[KEEP, KEY_FACT, KEY_BOX], &[KEEP]).is_err());
        assert!(
            key(
                &got,
                &[KEEP, KEY_FACT, KEY_BOX],
                &[KEEP, "guideline_x(new)."]
            )
            .is_err()
        );
    }
    #[test]
    fn shard_contrast_reports_its_treatment_inside_slot_6_files() {
        let want = [
            ("copies/0006/sb0006-a.pl", "1"),
            ("intakes/0006/sb-p-mx.pl", "2"),
            ("patient.pl", "3"),
        ];
        let got = [("copies/0006/sb0006-a.pl", "9"), ("patient.pl", "3")];
        let shard = |got: &[(&str, &str)]| {
            judge(&inputs(
                "mx-shard",
                &want,
                got,
                &[KEEP, "f('sb0006__x')."],
                &[KEEP, "f('sb0006__y')."],
            ))
        };
        let d = shard(&got).unwrap();
        assert!(
            d.rows
                .iter()
                .any(|(_, v)| v.contains("sb0006: 1 removed + 1 added"))
        );
        // Plants: the patient changed; the intake kept.
        assert!(shard(&[("copies/0006/sb0006-a.pl", "9"), ("patient.pl", "4")]).is_err());
        assert!(
            shard(&[
                ("copies/0006/sb0006-a.pl", "9"),
                ("intakes/0006/sb-p-mx.pl", "2"),
                ("patient.pl", "3")
            ])
            .is_err()
        );
    }
}
