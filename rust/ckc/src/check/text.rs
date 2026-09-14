// Python text law: Unicode case folding / decimal categories, not locale collation.
pub(super) fn space(c: char) -> bool {
    c.is_whitespace() || matches!(c, '\u{1c}'..='\u{1f}')
}
pub(super) fn strip(s: &str) -> &str {
    s.trim_matches(space)
}
pub(super) fn lines(s: &str) -> impl DoubleEndedIterator<Item = &str> {
    let mut result = Vec::new();
    let mut start = 0;
    let mut iter = s.char_indices().peekable();
    while let Some((i, c)) = iter.next() {
        if matches!(
            c,
            '\n' | '\r' | '\u{b}' | '\u{c}' | '\u{1c}'
                ..='\u{1e}' | '\u{85}' | '\u{2028}' | '\u{2029}'
        ) {
            result.push(&s[start..i]);
            start = i + c.len_utf8();
            if c == '\r' && iter.peek().is_some_and(|(_, c)| *c == '\n') {
                iter.next();
                start += 1;
            }
        }
    }
    if start < s.len() {
        result.push(&s[start..]);
    }
    result.into_iter()
}
