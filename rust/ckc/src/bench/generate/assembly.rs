use super::{Options, relative, resolve_input};
use std::collections::BTreeSet;
use std::path::PathBuf;

pub const CLASSIFIERS: [&str; 4] = [
    "f3-b1-classifier",
    "f3-b2-classifier",
    "f3-b3-classifier",
    "f3-b4-classifier",
];
pub const READERS: [&str; 4] = [
    "f3-b1-reader",
    "f3-b2-reader",
    "f3-b3-reader",
    "f3-b4-reader",
];

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Template {
    Original,
    Independent,
    Shared,
    Padding,
}
impl Template {
    pub fn role(self) -> &'static str {
        match self {
            Self::Original => "active-source",
            Self::Independent | Self::Shared => "active-transform",
            Self::Padding => "padding",
        }
    }
    pub fn sample_key(self) -> &'static str {
        match self {
            Self::Original => "original",
            Self::Independent => "independent",
            Self::Shared => "shared",
            Self::Padding => "padding",
        }
    }
}
pub struct Shard {
    pub index: usize,
    pub template: Template,
    pub source: String,
    pub attached: Vec<PathBuf>,
}
pub struct Layout {
    pub shards: Vec<Shard>,
    pub declared_sources: Vec<String>,
    pub fixture_inputs: Vec<PathBuf>,
    pub current_k: usize,
    pub smoke_only: bool,
    pub grouped_active: bool,
}
pub fn shared_family(family: &str) -> bool {
    matches!(family, "F3" | "MX")
}
pub fn supported(family: &str) -> bool {
    matches!(
        family,
        "F1a" | "F1b" | "F2" | "F3" | "F4" | "F5a" | "F5b" | "F5c" | "MX"
    )
}
pub fn fixture(o: &Options, name: &str) -> Result<PathBuf, String> {
    resolve_input(
        &o.root,
        &relative(
            &o.root,
            &o.root
                .join("bench/sb/fixtures/bridges")
                .join(format!("{name}.pl.in")),
        )?,
    )
}
pub fn layout(o: &Options, declared_k: usize) -> Result<Layout, String> {
    let sources: Vec<String> = match o.family.as_str() {
        "F2" => (1..=declared_k).map(|i| format!("F2-G{i:02}")).collect(),
        "F3" | "MX" => ["CDC", "VA-opioids", "VA-SUD", "TIP63", "CKD", "T2DM"]
            .into_iter()
            .map(str::to_owned)
            .collect(),
        "F4" => ["VA-opioids", "ASAM", "CDC"]
            .into_iter()
            .map(str::to_owned)
            .collect(),
        _ => vec!["CDC".into()],
    };
    if sources.len() != declared_k {
        return Err("family declared source count differs from the shard map".into());
    }
    let mut inputs = BTreeSet::new();
    if shared_family(&o.family) {
        for name in CLASSIFIERS.into_iter().chain(READERS) {
            inputs.insert(fixture(o, name)?);
        }
    }
    let mut shards = Vec::new();
    for index in 1..=o.scale {
        let template = if o.family == "F2" && index <= declared_k {
            Template::Independent
        } else if shared_family(&o.family) && (2..=declared_k).contains(&index) {
            Template::Shared
        } else if index == 1 {
            Template::Original
        } else {
            Template::Padding
        };
        let source = match template {
            Template::Original => "CDC".into(),
            Template::Independent | Template::Shared => sources[index - 1].clone(),
            Template::Padding => format!(
                "{}-padding-{index:04}",
                if o.family == "F1b" { "F1b" } else { "F1a" }
            ),
        };
        let mut attached = Vec::new();
        if shared_family(&o.family) {
            match index {
                1 => {
                    for name in CLASSIFIERS {
                        attached.push(fixture(o, name)?);
                    }
                    attached.push(fixture(o, "f3-b3-reader")?);
                }
                2 => {
                    attached.push(fixture(o, "f3-b1-reader")?);
                    attached.push(fixture(o, "f3-b2-reader")?);
                }
                3 => attached.push(fixture(o, "f3-b4-reader")?),
                _ => (),
            }
        } else if o.family == "F4" && index == 1 {
            attached.push(fixture(o, "f4-l1")?);
            attached.push(fixture(o, "f4-l2")?);
        }
        inputs.extend(attached.iter().cloned());
        shards.push(Shard {
            index,
            template,
            source,
            attached,
        });
    }
    let active = shards
        .iter()
        .filter(|s| s.template != Template::Padding)
        .count();
    Ok(Layout {
        shards,
        declared_sources: sources,
        fixture_inputs: inputs.into_iter().collect(),
        // F4's three source fragments share the CDC capacity shard.
        current_k: if o.family == "F4" { declared_k } else { active },
        smoke_only: o.scale < declared_k,
        grouped_active: shared_family(&o.family) || o.family == "F4",
    })
}
