// DQ10 (deferred row `Superseded notice overflow`) as a byte map between page generations, for
// the oracles that compare a current page with frozen pre-DQ10 bytes
// (`.agent/archive/migrations/dq10-repin.py` applies the forward map to the committed goldens).
// Its one site: the stylesheet rule RULE right after ANCHOR, before `@media print {`. The map
// asserts the shape it reads, so the inverse also checks that the current page carries the rule.
const PRINT: &str = "@media print {";
const ANCHOR: &str = "p.provenance span + a { margin-left: 0.75rem; }\n";
const RULE: &str = "section.superseded p { overflow-wrap: anywhere; }\n";

/// `forward`: pre-DQ10 page bytes → DQ10 page bytes; else DQ10 page bytes → the bytes the
/// pre-DQ10 renderer printed. Bytes without a stylesheet pass unchanged.
pub fn map(page: &str, forward: bool) -> String {
    let Some((head, tail)) = page.split_once(PRINT) else {
        return page.to_owned();
    };
    let (old, new) = (ANCHOR.to_owned(), format!("{ANCHOR}{RULE}"));
    let (from, to) = if forward { (old, new) } else { (new, old) };
    assert_eq!(head.matches(&from).count(), 1, "DQ10 stylesheet site {from:?}");
    assert_eq!(
        head.matches(RULE).count(),
        usize::from(!forward),
        "DQ10 rule count"
    );
    format!("{}{PRINT}{tail}", head.replacen(&from, &to, 1))
}
