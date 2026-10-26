// REV-K (contract rev-k P1/P2) as a byte map between page generations, for the oracles that
// compare a current page with frozen pre-REV-K bytes (`.agent/archive/migrations/rev-k-repin.py`
// applies the forward map to the committed goldens). Its sites: the stylesheet before
// `@media print {` + the count sentences, whose `<p>` / `<summary>` openings corpus text
// cannot spell (it renders escaped). Each map asserts the shape it reads, so the inverse also
// checks that the current page carries every REV-K form.
const PRINT: &str = "@media print {";
const NEW: &str = "#4b5563";
// (rule with `@` at its color, pre-REV-K color)
const RULES: [(&str, &str); 4] = [
    ("main.hl-click pre.prose { color: @; }\n", "#9ca3af"),
    ("main.hl-click pre.prose .kw { color: @; }\n", "#9ca3af"),
    (
        "main.hl-click pre.prose mark:not(.hl-pick) { background: none; text-decoration-color: @; }\n",
        "#9ca3af",
    ),
    (
        "input[type=\"text\"], textarea { display: block; box-sizing: border-box; width: 100%; max-width: 28rem; margin-top: 0.3rem; padding: 0.45rem 0.6rem; border: 1px solid @; font-family: inherit; font-size: 1rem; color: #111827; background: #ffffff; }\n",
        "#e5e7eb",
    ),
];
// (opening, [(singular, plural) after each number]); equal forms = no count noun.
type Slots = &'static [(&'static str, &'static str)];
const SENTENCES: [(&str, Slots); 3] = [
    (
        "<p>Reviewers recorded ",
        &[
            (" decision on ", " decisions on "),
            (" of ", " of "),
            (" document.", " documents."),
        ],
    ),
    (
        "<p>No decisions are recorded for the ",
        &[(
            " document in this guideline.",
            " documents in this guideline.",
        )],
    ),
    (
        "<summary>Compiled Prolog (",
        &[(" line)</summary>", " lines)</summary>")],
    ),
];

/// `forward`: pre-REV-K page bytes → REV-K page bytes; else REV-K page bytes → the bytes the
/// pre-REV-K renderer printed. Bytes without a stylesheet pass unchanged.
pub fn map(page: &str, forward: bool) -> String {
    let Some((head, tail)) = page.split_once(PRINT) else {
        return page.to_owned();
    };
    let mut head = head.to_owned();
    for (rule, old) in RULES {
        let (old, new) = (rule.replace('@', old), rule.replace('@', NEW));
        let (from, to) = if forward { (old, new) } else { (new, old) };
        assert_eq!(
            head.matches(&from).count(),
            1,
            "REV-K stylesheet site {from:?}"
        );
        head = head.replacen(&from, &to, 1);
    }
    let mut page = format!("{head}{PRINT}{tail}");
    for (opening, slots) in SENTENCES {
        let mut at = 0;
        while let Some(found) = page[at..].find(opening) {
            at += found + opening.len();
            for &(one, many) in slots {
                let digits = page[at..].bytes().take_while(u8::is_ascii_digit).count();
                assert!(digits > 0, "REV-K count after {opening:?}");
                let current = if &page[at..at + digits] == "1" {
                    one
                } else {
                    many
                };
                at += digits;
                let (from, to) = if forward {
                    (many, current)
                } else {
                    (current, many)
                };
                assert!(
                    page[at..].starts_with(from),
                    "REV-K count noun {from:?} after {opening:?}"
                );
                page.replace_range(at..at + from.len(), to);
                at += to.len();
            }
        }
    }
    page
}
