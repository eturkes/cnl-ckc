use std::collections::BTreeSet;

pub const HINT: &str = "Select a word with a solid underline to see how the compiler reads it.";
pub const INTRO: &str = "The compiler reads each word in this table as shown, in every document of this guideline. In the ACE text of a document page, select a word with a solid underline to see its reading.";
pub const CSS: &str = "button.rw { margin: 0; padding: 0; border: 0; font: inherit; color: inherit; background: none; cursor: pointer; text-decoration: underline solid #1d4ed8 1px; text-underline-offset: 0.35em; }\nbutton.rw[aria-pressed=\"true\"] { text-decoration-thickness: 3px; }\np.reading { min-height: 1.55em; margin: 0.5rem 0 0; overflow-wrap: anywhere; }\np.reading span[data-w=\"\"] { color: #4b5563; }\n";
pub const PRINT_CSS: &str = "button.rw { text-decoration: none; }\np.reading { display: none; }\n";
pub const NARROW: &str =
    "@media screen and (max-width: 40rem) { table { display: block; overflow-x: auto; } }";

#[derive(Clone, Debug)]
pub struct Row {
    pub kind: String,
    pub lemma: String,
    pub value: String,
}
impl Row {
    pub fn new(kind: &str, lemma: &str, value: &str) -> Self {
        Self {
            kind: kind.into(),
            lemma: lemma.into(),
            value: value.into(),
        }
    }

    fn rank(&self) -> usize {
        [
            "strength",
            "evidence",
            "preference",
            "alternative",
            "qualifier",
            "scope",
            "exclusion",
        ]
        .iter()
        .position(|k| *k == self.kind)
        .unwrap_or(7)
    }

    pub fn reading(&self) -> String {
        match self.kind.as_str() {
            "strength" => format!("a {} recommendation", self.value),
            "evidence" => format!("{} certainty of the evidence", self.value.replace('_', " ")),
            "preference" => "the preferred option".into(),
            "alternative" => "the alternative to the preferred option".into(),
            "qualifier" => format!("a qualifier: {}", match self.value.as_str() {
                "usuality" => "How often", "ideality" => "Best case", "focus" => "Emphasis",
                "manner" => "Manner", "force" => "Strength of wording", "possibility" => "Possibility",
                "applicability" => "Condition of use", other => panic!("oracle qualifier {other}"),
            }),
            "scope" => "every recommendation of this guideline".into(),
            "exclusion" => "the verb of a scope exclusion: when its condition holds, no recommendation of this guideline applies to the patient".into(),
            other => panic!("oracle reading kind {other}"),
        }
    }
}

pub fn rows(table: &str) -> Vec<Row> {
    table
        .lines()
        .skip(1)
        .map(|s| {
            let f: Vec<_> = s.split('\t').collect();
            assert_eq!(f.len(), 3, "oracle table row");
            Row::new(f[0], f[1], f[2])
        })
        .collect()
}

pub fn table(rows: &[Row]) -> String {
    "# format: kind\tlemma\tvalue\n".to_owned()
        + &rows
            .iter()
            .map(|r| format!("{}\t{}\t{}\n", r.kind, r.lemma, r.value))
            .collect::<String>()
}

pub fn reading_rows(rows: &[Row]) -> Vec<&Row> {
    let mut result: Vec<_> = rows.iter().filter(|r| r.rank() < 7).collect();
    result.sort_by_key(|r| r.rank());
    result
}

pub fn text(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

pub fn attr(s: &str) -> String {
    text(s).replace('"', "&quot;").replace('\'', "&#39;")
}

pub fn forms(verb: &str) -> Vec<String> {
    let mut forms = vec![verb.into(), format!("{verb}s"), format!("{verb}es")];
    if let Some(stem) = verb.strip_suffix('y') {
        forms.push(format!("{stem}ies"));
    }
    forms
}

pub fn meanings(rows: &[Row]) -> String {
    let readings = reading_rows(rows);
    if readings.is_empty() {
        return String::new();
    }
    let mut out = format!("<section>\n<h2>Word meanings</h2>\n<p>{INTRO}</p>\n");
    let noun = rows.iter().find(|r| r.kind == "scope");
    let verb = rows.iter().find(|r| r.kind == "exclusion");
    if let (Some(noun), Some(verb)) = (noun, verb) {
        let mut collisions = forms(&verb.lemma);
        collisions.push(noun.lemma.clone());
        if !readings
            .iter()
            .any(|r| r.rank() < 5 && collisions.contains(&r.lemma))
        {
            out += &format!(
                "<p>The words {} and {} have a solid underline only in the sentences that the compiler read as scope exclusions.</p>\n",
                text(&noun.lemma),
                text(&verb.lemma)
            );
        }
    }
    // D4 joins the tbody as one block; only adjacent rows carry LF separators.
    out +=
        "<table class=\"compact\">\n<thead><tr><th>Word</th><th>Read as</th></tr></thead>\n<tbody>";
    out += &readings
        .iter()
        .map(|r| {
            format!(
                "<tr><td>{}</td><td>{}</td></tr>",
                text(&r.lemma),
                r.reading()
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    out += "</tbody>\n</table>\n</section>\n";
    out
}

#[derive(Clone, Debug)]
pub struct Mark {
    pub start: usize,
    pub end: usize,
    pub group: usize,
}
impl Mark {
    pub fn new(start: usize, end: usize, group: usize) -> Self {
        Self { start, end, group }
    }
}

pub fn tokens(chars: &[char]) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        if !chars[i].is_ascii_alphanumeric() {
            i += 1;
            continue;
        }
        let start = i;
        i += 1;
        while i < chars.len() {
            if chars[i].is_ascii_alphanumeric() {
                i += 1;
            } else if chars[i] == '-' && chars.get(i + 1).is_some_and(char::is_ascii_alphanumeric) {
                i += 2;
            } else {
                break;
            }
        }
        result.push((start, i));
    }
    result
}

// D5 preserves legacy function words, including words exposed by clipping:
// recommendation can leave a final `on`. Generated prose uses old golden words
// plus such boundaries, not an inferred complete ACE function-word vocabulary.
fn keyword(word: &str) -> bool {
    matches!(
        word.to_ascii_lowercase().as_str(),
        "a" | "an"
            | "the"
            | "every"
            | "if"
            | "then"
            | "and"
            | "is"
            | "it"
            | "that"
            | "does"
            | "not"
            | "should"
            | "at"
            | "least"
            | "less"
            | "more"
            | "most"
            | "than"
            | "of"
            | "on"
            | "for"
            | "to"
            | "before"
            | "after"
            | "during"
            | "within"
            | "per"
            | "about"
    )
}

fn plain(chars: &[char]) -> String {
    let mut out = String::new();
    let mut from = 0;
    for (a, b) in tokens(chars) {
        out += &text(&chars[from..a].iter().collect::<String>());
        let token: String = chars[a..b].iter().collect();
        if keyword(&token) {
            out += &format!("<span class=\"kw\">{}</span>", text(&token));
        } else {
            out += &text(&token);
        }
        from = b;
    }
    out + &text(&chars[from..].iter().collect::<String>())
}

fn piece(chars: &[char], marks: &[Mark], start: usize, end: usize) -> String {
    let mut out = String::new();
    let mut from = start;
    for mark in marks {
        let lo = mark.start.max(start);
        let hi = mark.end.min(end);
        if lo >= hi {
            continue;
        }
        assert!(lo >= from, "oracle nonoverlapping marks");
        out += &plain(&chars[from..lo]);
        out += &if mark.group < 48 {
            format!("<mark class=\"t{}\">", mark.group)
        } else {
            "<mark>".into()
        };
        out += &text(&chars[lo..hi].iter().collect::<String>());
        out += "</mark>";
        from = hi;
    }
    out + &plain(&chars[from..end])
}

pub fn line_ranges(ace: &str, selected: &[usize]) -> Vec<(usize, usize)> {
    let mut ordinal = 0;
    let mut offset = 0;
    let mut out = Vec::new();
    for line in ace.split('\n') {
        let length = line.chars().count();
        if !line.is_empty() {
            ordinal += 1;
            if selected.contains(&ordinal) {
                out.push((offset, offset + length));
            }
        }
        offset += length + 1;
    }
    out
}

#[derive(Debug)]
pub struct Expected {
    pub ace_html: String,
    pub reserved: String,
    pub lemmas: Vec<String>,
    pub buttons: Vec<(String, String)>,
}

pub fn render(ace: &str, marks: &[Mark], rows: &[Row], exclusions: &[usize]) -> Expected {
    let chars: Vec<_> = ace.chars().collect();
    let exclusions = line_ranges(ace, exclusions);
    let ordered = reading_rows(rows);
    let noun = rows.iter().find(|r| r.kind == "scope");
    let verb = rows.iter().find(|r| r.kind == "exclusion");
    let mut words = Vec::new();
    for (a, b) in tokens(&chars) {
        let word: String = chars[a..b].iter().collect();
        let global = ordered
            .iter()
            .find(|r| r.rank() < 5 && r.lemma == word)
            .copied();
        let in_scope = noun.is_some()
            && verb.is_some()
            && exclusions.iter().any(|&(lo, hi)| lo <= a && b <= hi);
        let found = global.or_else(|| {
            if !in_scope {
                return None;
            }
            noun.filter(|r| r.lemma == word)
                .or_else(|| verb.filter(|r| forms(&r.lemma).contains(&word)))
        });
        if let Some(row) = found {
            words.push((a, b, row.lemma.clone(), word));
        }
    }
    let mut out = String::new();
    let mut from = 0;
    let mut lemmas = Vec::new();
    let mut seen = BTreeSet::new();
    let mut buttons = Vec::new();
    for (a, b, lemma, word) in &words {
        out += &piece(&chars, marks, from, *a);
        out += &format!(
            "<button type=\"button\" class=\"rw\" aria-pressed=\"false\" data-w=\"{}\">",
            attr(lemma)
        );
        out += &piece(&chars, marks, *a, *b);
        out += "</button>";
        from = *b;
        if seen.insert(lemma.clone()) {
            lemmas.push(lemma.clone());
        }
        buttons.push((lemma.clone(), word.clone()));
    }
    out += &piece(&chars, marks, from, chars.len());
    let reserved = if words.is_empty() {
        String::new()
    } else {
        let mut line =
            format!("<p class=\"reading\" aria-live=\"polite\"><span data-w=\"\">{HINT}</span>");
        for lemma in &lemmas {
            let row = ordered
                .iter()
                .find(|r| r.lemma == *lemma)
                .expect("oracle lemma row");
            line += &format!(
                "<span data-w=\"{}\" hidden>In this guideline, the compiler reads {} as {}.</span>",
                attr(lemma),
                text(lemma),
                row.reading()
            );
        }
        line + "</p>\n"
    };
    Expected {
        ace_html: out,
        reserved,
        lemmas,
        buttons,
    }
}
