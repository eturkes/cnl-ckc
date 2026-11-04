// Contract sb D2 MX + F4 one-factor contrasts (scientist-3 round 4 A4–A6, A9): each
// changes one factor of its base instance; every other slot, namespace + load position stays.
use super::assembly::{Layout, Template};
use super::text::{Document, Line, Term};
use super::{Options, relative};

pub const NAMES: [&str; 5] = [
    "mx-bridge-b4",
    "mx-bridge-b1",
    "mx-key",
    "mx-shard",
    "f4-l2",
];
// MX shard slots: 2 VA-opioids (b1 + b2 readers), 3 VA-SUD (b4 reader), 6 T2DM hub.
const KEY_PATIENT: &str = "sb-p-mx";
const KEY_SENTENCE: &str = "17";

fn family(name: &str) -> &'static str {
    if name.starts_with("f4-") { "F4" } else { "MX" }
}
pub fn validate(o: &Options) -> Result<(), String> {
    let Some(name) = o.contrast.as_deref() else {
        return Ok(());
    };
    if !NAMES.contains(&name) {
        return Err(format!("unknown contrast {name}"));
    }
    if o.family != family(name) {
        return Err(format!(
            "contrast {name} belongs to family {}",
            family(name)
        ));
    }
    if family(name) == "MX" && o.scale < 6 {
        return Err("MX contrasts need the full six-source topology (G >= 6)".into());
    }
    Ok(())
}
fn drop_attached(
    o: &Options,
    layout: &mut Layout,
    slot: usize,
    fixture: &str,
) -> Result<String, String> {
    let path = super::assembly::fixture(o, fixture)?;
    let shard = layout
        .shards
        .iter_mut()
        .find(|s| s.index == slot)
        .ok_or("contrast slot absent")?;
    let before = shard.attached.len();
    shard.attached.retain(|p| *p != path);
    if shard.attached.len() + 1 != before {
        return Err(format!(
            "contrast fixture {fixture} is not attached at slot {slot}"
        ));
    }
    Ok(format!(
        "removed attached document {} from shard {slot:04}",
        relative(&o.root, &path)?
    ))
}
// Layout half: the bridge + chain contrasts drop one attached reader group; the shard
// contrast turns the T2DM hub slot into F1a-kind padding in place.
pub fn apply_layout(o: &Options, layout: &mut Layout) -> Result<Vec<(String, String)>, String> {
    let Some(name) = o.contrast.as_deref() else {
        return Ok(Vec::new());
    };
    let descriptor = match name {
        "mx-bridge-b4" => drop_attached(o, layout, 3, "f3-b4-reader")?,
        "mx-bridge-b1" => drop_attached(o, layout, 2, "f3-b1-reader")?,
        "f4-l2" => drop_attached(o, layout, 1, "f4-l2")?,
        "mx-shard" => {
            let shard = layout
                .shards
                .iter_mut()
                .find(|s| s.index == 6)
                .ok_or("contrast slot absent")?;
            if shard.template != Template::Shared
                || shard.source != "T2DM"
                || !shard.attached.is_empty()
            {
                return Err("contrast slot 6 is not the T2DM shared hub".into());
            }
            shard.template = Template::Padding;
            shard.source = "F1a-padding-0006".into();
            layout.current_k = layout
                .shards
                .iter()
                .filter(|s| s.template != Template::Padding)
                .count();
            "shard 0006 T2DM shared hub replaced in place by F1a-kind padding copy F1a-padding-0006"
                .into()
        }
        "mx-key" => format!(
            "removed {KEY_PATIENT} sentence {KEY_SENTENCE} (the oxycodone opioid-pain-medication prescription record) from the patient + every intake projection"
        ),
        _ => unreachable!(),
    };
    Ok(vec![
        ("contrast".into(), name.into()),
        ("contrast_descriptor".into(), descriptor),
    ])
}
fn mentions_sentence(t: &Term, doc: &str, sentence: &str) -> bool {
    match t {
        Term::Compound(_, name, args) => {
            (name == "$guideline_id"
                && args.get(1).and_then(Term::atom) == Some(doc)
                && matches!(args.get(2), Some(Term::Number(n)) if n == sentence))
                || args.iter().any(|a| mentions_sentence(a, doc, sentence))
        }
        Term::List(args, tail) => {
            args.iter().any(|a| mentions_sentence(a, doc, sentence))
                || tail
                    .as_ref()
                    .is_some_and(|t| mentions_sentence(t, doc, sentence))
        }
        _ => false,
    }
}
// Patient half (mx-key): every fact naming a sentence-17 identity + that sentence's
// comment leave; surviving identities keep their sentence numbers.
pub fn apply_patient(o: &Options, patient: Document) -> Result<Document, String> {
    if o.contrast.as_deref() != Some("mx-key") {
        return Ok(patient);
    }
    if super::document_id(&patient)? != KEY_PATIENT {
        return Err(format!("contrast mx-key needs patient {KEY_PATIENT}"));
    }
    let comment = format!("% S{KEY_SENTENCE}: ");
    let before = patient.lines.len();
    let mut dropped = 0;
    let lines: Vec<Line> = patient
        .lines
        .into_iter()
        .filter(|line| match line {
            Line::Text(s) => !s.starts_with(&comment),
            Line::Clause(c) => {
                let hit = c
                    .terms()
                    .any(|t| mentions_sentence(t, KEY_PATIENT, KEY_SENTENCE));
                dropped += usize::from(hit);
                !hit
            }
        })
        .collect();
    if dropped == 0 || before - lines.len() != dropped + 1 {
        return Err(format!(
            "contrast mx-key: sentence {KEY_SENTENCE} must hold one comment + at least one fact"
        ));
    }
    let out = Document {
        lines,
        singleton: patient.singleton,
    };
    out.validate()?;
    Ok(out)
}
