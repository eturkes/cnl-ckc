use super::assembly::{Layout, Template};
use super::{Options, digest};

pub struct Variation {
    pub load_order: Vec<usize>,
    pub rows: Vec<(String, String)>,
}

fn permute(seed: u64, domain: &str, indices: &[usize]) -> Vec<usize> {
    let mut ranked: Vec<_> = indices
        .iter()
        .map(|&index| {
            (
                digest(format!("sb1-holdout-1\0{seed}\0{domain}\0{index}").as_bytes()),
                index,
            )
        })
        .collect();
    ranked.sort();
    ranked.into_iter().map(|(_, index)| index).collect()
}

pub fn apply(o: &Options, layout: &mut Layout) -> Variation {
    if o.seed == 0 {
        return Variation {
            load_order: layout.shards.iter().map(|s| s.index).collect(),
            rows: Vec::new(),
        };
    }
    let private: Vec<_> = layout
        .shards
        .iter()
        .filter(|s| s.template != Template::Original)
        .map(|s| s.index)
        .collect();
    let namespaces = permute(o.seed, "namespace", &private);
    for (slot, namespace) in layout
        .shards
        .iter_mut()
        .filter(|s| s.template != Template::Original)
        .zip(namespaces)
    {
        slot.namespace = namespace;
    }
    let padding: Vec<_> = layout
        .shards
        .iter()
        .filter(|s| s.template == Template::Padding)
        .map(|s| s.index)
        .collect();
    let padding = permute(o.seed, "padding", &padding);
    let mut load_order: Vec<_> = layout
        .shards
        .iter()
        .filter(|s| s.template != Template::Padding)
        .map(|s| s.index)
        .collect();
    load_order.extend(&padding);
    let mut rows = vec![
        ("variation_version".into(), "sb1-holdout-1".into()),
        ("variation_salt".into(), format!("s{:016x}", o.seed)),
        (
            "variation_padding_order".into(),
            if padding.is_empty() {
                "none".into()
            } else {
                padding
                    .iter()
                    .map(usize::to_string)
                    .collect::<Vec<_>>()
                    .join(",")
            },
        ),
    ];
    rows.extend(layout.shards.iter().map(|s| {
        (
            format!("variation_namespace:{:04}", s.index),
            s.namespace.to_string(),
        )
    }));
    Variation { load_order, rows }
}
