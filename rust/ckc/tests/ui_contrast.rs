// Contrast law (`CLAUDE.md` UI quality floor, `.claude/rules/clinician-design.md`): the census
// classes every color of the rendered stylesheet — text ≥ 4.5:1 on the background it sits on
// (WCAG 2 AA), a border, focus ring, accent or underline cue ≥ 3:1, a separator in DECORATIVE
// exempt — and fails on a color it cannot class, so a new rule must take a class. Each run grades
// its plants first. CKC_UI_TEST_BIN = candidate binary.
use std::fs;
use std::path::PathBuf;
use std::process::Command;

const PAGE: &str = "#ffffff";
// Text that can sit on a mark tint: body text + the function-word color (`.kw`, also inside a
// picked mark).
const MARK_TEXT: [&str; 2] = ["#111827", "#4b5563"];
const DECORATIVE: [(&str, &str); 5] = [
    ("nav.crumbs", "#e5e7eb"),
    ("th, td", "#e5e7eb"),
    ("pre", "#e5e7eb"),
    ("nav.docnav", "#e5e7eb"),
    ("footer.scope", "#e5e7eb"),
];

type Rule = (String, Vec<(String, String)>);

fn stylesheet() -> String {
    let program = std::env::var_os("CKC_UI_TEST_BIN")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_BIN_EXE_ckc")));
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let out = std::env::temp_dir().join(format!("ckc-ui-contrast-{}", std::process::id()));
    let _ = fs::remove_dir_all(&out);
    let run = Command::new(program)
        .args(["ui", "render"])
        .arg(&out)
        .arg(root.join("tests/ui/green/empty-guidelines/tree"))
        .output()
        .unwrap();
    assert!(run.status.success(), "render: {run:?}");
    let page = fs::read_to_string(out.join("index.html")).unwrap();
    fs::remove_dir_all(&out).unwrap();
    let (_, tail) = page.split_once("<style>").expect("stylesheet");
    tail.split_once("</style>")
        .expect("stylesheet end")
        .0
        .to_owned()
}

// Rule blocks in order; an `@media` block contributes its inner rules.
fn rules(css: &str) -> Vec<Rule> {
    let mut out = Vec::new();
    let mut rest = css;
    while let Some(open) = rest.find('{') {
        let head = rest[..open].trim();
        if head.starts_with("@media") {
            let mut depth = 0;
            let close = rest[open..]
                .char_indices()
                .find_map(|(i, c)| {
                    depth += match c {
                        '{' => 1,
                        '}' => -1,
                        _ => 0,
                    };
                    (depth == 0).then_some(open + i)
                })
                .expect("@media close");
            out.extend(rules(&rest[open + 1..close]));
            rest = &rest[close + 1..];
        } else {
            let close = open + rest[open..].find('}').expect("rule close");
            let body = rest[open + 1..close]
                .split(';')
                .filter_map(|d| d.split_once(':'));
            let decls = body.map(|(p, v)| (p.trim().to_owned(), v.trim().to_owned()));
            out.push((head.to_owned(), decls.collect()));
            rest = &rest[close + 1..];
        }
    }
    assert!(rest.trim().is_empty(), "stylesheet tail {rest:?}");
    out
}

// Every `#` of the stylesheet opens a six-digit lowercase color (ui.md palette law).
fn colors(value: &str) -> Vec<&str> {
    value
        .match_indices('#')
        .map(|(i, _)| {
            let hex = value.get(i..i + 7).unwrap_or_default();
            assert!(
                hex.len() == 7
                    && hex[1..]
                        .bytes()
                        .all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f')),
                "unparsed color in {value:?}"
            );
            hex
        })
        .collect()
}

fn luminance(hex: &str) -> f64 {
    let channel = |i: usize| {
        let v = f64::from(u8::from_str_radix(&hex[i..i + 2], 16).unwrap()) / 255.0;
        if v <= 0.04045 {
            v / 12.92
        } else {
            ((v + 0.055) / 1.055).powf(2.4)
        }
    };
    0.2126 * channel(1) + 0.7152 * channel(3) + 0.0722 * channel(5)
}

fn ratio(a: &str, b: &str) -> f64 {
    let (x, y) = (luminance(a), luminance(b));
    (x.max(y) + 0.05) / (x.min(y) + 0.05)
}

// (failures, classed color count)
fn census(css: &str) -> (Vec<String>, usize) {
    let rules = rules(css);
    let tints: Vec<&str> = rules
        .iter()
        .filter(|(selector, _)| selector.contains("mark"))
        .flat_map(|(_, decls)| decls.iter())
        .filter(|(prop, _)| prop.starts_with("background"))
        .flat_map(|(_, value)| colors(value))
        .collect();
    let mut bad = Vec::new();
    let mut classed = 0;
    for (selector, decls) in &rules {
        let mark = selector.contains("mark");
        let own = |p: &str| decls.iter().find(|(prop, _)| prop == p).map(|(_, v)| v);
        let background = own("background").and_then(|v| colors(v).first().copied());
        for (prop, value) in decls {
            for color in colors(value) {
                classed += 1;
                let (min, grounds): (f64, Vec<&str>) = match prop.as_str() {
                    "color" if mark => (4.5, tints.iter().copied().chain([PAGE]).collect()),
                    "color" => (4.5, vec![background.unwrap_or(PAGE)]),
                    "background" if mark => {
                        for text in MARK_TEXT {
                            if ratio(text, color) < 4.5 {
                                bad.push(format!(
                                    "{text} text on {selector} {{ {prop}: {color} }}"
                                ));
                            }
                        }
                        continue;
                    }
                    "background" if own("color").is_some() => continue,
                    p if p.starts_with("text-decoration") => {
                        (3.0, tints.iter().copied().chain([PAGE]).collect())
                    }
                    p if p.starts_with("border")
                        || p.starts_with("outline")
                        || p == "accent-color" =>
                    {
                        if DECORATIVE.contains(&(selector.as_str(), color)) {
                            continue;
                        }
                        (3.0, vec![PAGE])
                    }
                    _ => {
                        bad.push(format!("unclassed {selector} {{ {prop}: {color} }}"));
                        continue;
                    }
                };
                for ground in grounds {
                    let r = ratio(color, ground);
                    if r < min {
                        bad.push(format!(
                            "{selector} {{ {prop}: {color} }} on {ground}: {r:.3}:1 < {min}:1"
                        ));
                    }
                }
            }
        }
    }
    (bad, classed)
}

#[test]
fn every_stylesheet_color_is_classed_and_meets_its_contrast() {
    assert!(
        (ratio("#000000", "#ffffff") - 21.0).abs() < 1e-9,
        "WCAG ratio positive control"
    );
    let css = stylesheet();
    let (bad, classed) = census(&css);
    assert!(bad.is_empty(), "contrast failures:\n{}", bad.join("\n"));
    assert!(
        classed > 100,
        "census read the stylesheet: {classed} colors"
    );
    // Plants: the pre-REV-K dimming + field-border colors (screen rules; the print block keeps
    // its own `.kw` rule), and a color no class covers.
    let (screen, print) = css.split_once("@media print {").expect("print block");
    let mut old = screen.to_owned();
    for (rule, pre) in [
        ("main.hl-click pre.prose { color: ", "#9ca3af"),
        ("main.hl-click pre.prose .kw { color: ", "#9ca3af"),
        (
            "main.hl-click pre.prose mark:not(.hl-pick) { background: none; text-decoration-color: ",
            "#9ca3af",
        ),
        ("padding: 0.45rem 0.6rem; border: 1px solid ", "#e5e7eb"),
    ] {
        let site = format!("{rule}#4b5563");
        assert_eq!(old.matches(&site).count(), 1, "plant anchor {site}");
        old = old.replacen(&site, &format!("{rule}{pre}"), 1);
    }
    let (planted, _) = census(&format!("{old}@media print {{{print}"));
    for want in [
        "main.hl-click pre.prose { color: #9ca3af } on #ffffff: 2.539:1 < 4.5:1",
        "main.hl-click pre.prose .kw { color: #9ca3af } on #ffffff: 2.539:1 < 4.5:1",
        "main.hl-click pre.prose mark:not(.hl-pick) { text-decoration-color: #9ca3af } on #ffffff: 2.539:1 < 3:1",
        "textarea { border: #e5e7eb } on #ffffff: 1.238:1 < 3:1",
    ] {
        assert!(
            planted.iter().any(|f| f.ends_with(want)),
            "plant not caught: {want}\n{planted:#?}"
        );
    }
    let (unclassed, _) = census(&format!("{css}p.reading {{ caret-color: #111827; }}"));
    assert_eq!(unclassed, ["unclassed p.reading { caret-color: #111827 }"]);
    eprintln!("ui contrast: {classed} colors classed, 0 failures; plants caught");
}
