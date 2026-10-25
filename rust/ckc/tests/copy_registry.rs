// dead-copy D1/D5/P6: table shape, exact retired entries, named uses + census plants.
use std::collections::BTreeSet;
use std::fs;
use std::ops::Range;
use std::path::PathBuf;

const DEAD: [(&str, &str); 2] = [
    (
        "how_long_after_a_reference_point",
        "how long after a reference point the action occurs",
    ),
    (
        "how_long_before_a_reference_point",
        "how long before a reference point the action occurs",
    ),
];

struct Row<'a> {
    name: &'a str,
    literal: Option<String>,
}

struct Token<'a> {
    text: &'a str,
    span: Range<usize>,
    comment: bool,
}

fn source() -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../rust/ckc-spec/src/ui.rs");
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

fn quoted_end(source: &str, start: usize) -> Option<usize> {
    let bytes = source.as_bytes();
    let mut at = start + 1;
    while at < bytes.len() {
        match bytes[at] {
            b'"' => return Some(at + 1),
            b'\\' => at += 2,
            _ => at += 1,
        }
    }
    None
}

fn decode(literal: &str) -> Option<String> {
    let mut chars = literal
        .strip_prefix('"')?
        .strip_suffix('"')?
        .chars()
        .peekable();
    let mut out = String::new();
    while let Some(c) = chars.next() {
        if c != '\\' {
            if c == '"' {
                return None;
            }
            if c == '\r' {
                if chars.next()? != '\n' {
                    return None;
                }
                out.push('\n');
            } else {
                out.push(c);
            }
            continue;
        }
        out.push(match chars.next()? {
            'n' => '\n',
            'r' => '\r',
            't' => '\t',
            '0' => '\0',
            '\\' => '\\',
            '\'' => '\'',
            '"' => '"',
            'x' => {
                let n = 16 * chars.next()?.to_digit(16)? + chars.next()?.to_digit(16)?;
                if n > 0x7f {
                    return None;
                }
                char::from_u32(n)?
            }
            'u' => {
                if chars.next()? != '{' {
                    return None;
                }
                let mut n = 0;
                let mut digits = 0;
                loop {
                    match chars.next()? {
                        '}' if digits > 0 => break,
                        '_' if digits > 0 => (),
                        c if digits < 6 => {
                            n = 16 * n + c.to_digit(16)?;
                            digits += 1;
                        }
                        _ => return None,
                    }
                }
                char::from_u32(n)?
            }
            newline @ ('\n' | '\r') => {
                if newline == '\r' && chars.next()? != '\n' {
                    return None;
                }
                while chars
                    .peek()
                    .is_some_and(|c| matches!(*c, ' ' | '\t' | '\r' | '\n'))
                {
                    chars.next();
                }
                continue;
            }
            _ => return None,
        });
    }
    Some(out)
}

fn row(input: &str) -> Option<(Row<'_>, usize)> {
    let text = input.trim_start();
    let leading = input.len() - text.len();
    if leading == 0 {
        return None;
    }
    let size = text
        .bytes()
        .take_while(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || *b == b'_')
        .count();
    if size == 0 {
        return None;
    }
    let name = &text[..size];
    let mut end = size;
    let literal = if text[size..].starts_with(" = \"") {
        let start = size + 3;
        end = quoted_end(text, start)?;
        let literal = &text[start..end];
        let mut chars = literal.chars();
        while let Some(c) = chars.next() {
            if c == '\\' && chars.next()? == '\n' {
                return None;
            }
        }
        Some(decode(literal).expect("Rust copy-table string escape"))
    } else {
        None
    };
    if text.as_bytes().get(end) != Some(&b';')
        || !matches!(text.as_bytes().get(end + 1), None | Some(b'\n'))
    {
        return None;
    }
    Some((Row { name, literal }, leading + end + 1))
}

fn table(source: &str) -> (Vec<Row<'_>>, Range<usize>) {
    let marker = "copy_table! {";
    let start = source.find(marker).expect("copy_table! marker");
    let body = start + marker.len();
    let end = body + source[body..].find("\n}\n").expect("copy_table! end");
    let block = &source[body..end];
    let mut rows = Vec::new();
    let mut at = 0;
    while at < block.len() {
        if let Some((row, size)) = row(&block[at..]) {
            rows.push(row);
            at += size;
            if block.as_bytes().get(at) == Some(&b'\n') {
                at += 1;
            }
        } else {
            at += block[at..].find('\n').map_or(block.len() - at, |n| n + 1);
        }
    }
    (rows, start..end + 3)
}

fn raw_end(source: &str, start: usize) -> Option<usize> {
    let tail = &source[start..];
    let prefix = if tail.starts_with("br") || tail.starts_with("cr") {
        2
    } else if tail.starts_with('r') {
        1
    } else {
        return None;
    };
    let bytes = source.as_bytes();
    let mut at = start + prefix;
    while bytes.get(at) == Some(&b'#') {
        at += 1;
    }
    if bytes.get(at) != Some(&b'"') {
        return None;
    }
    let close = format!("\"{}", "#".repeat(at - start - prefix));
    let body = at + 1;
    Some(body + source[body..].find(&close).expect("raw string end") + close.len())
}

fn char_end(source: &str, start: usize) -> Option<usize> {
    let mut at = start + 1;
    if source.as_bytes().get(at) == Some(&b'\\') {
        at += 1;
        at += match source.as_bytes().get(at)? {
            b'x' => 3,
            b'u' => source[at..].find('}')? + 1,
            _ => 1,
        };
    } else {
        at += source[at..].chars().next()?.len_utf8();
    }
    (source.as_bytes().get(at) == Some(&b'\'')).then_some(at + 1)
}

fn tokens(source: &str) -> impl Iterator<Item = Token<'_>> {
    let bytes = source.as_bytes();
    let mut at = 0;
    std::iter::from_fn(move || {
        while at < bytes.len() {
            let start = at;
            if bytes[at].is_ascii_whitespace() {
                at += 1;
            } else if source[at..].starts_with("//") {
                at += source[at..].find('\n').unwrap_or(bytes.len() - at);
                return Some(Token {
                    text: &source[start..at],
                    span: start..at,
                    comment: true,
                });
            } else if source[at..].starts_with("/*") {
                at += 2;
                let mut depth = 1;
                while depth > 0 {
                    assert!(at < bytes.len(), "block comment end");
                    if bytes[at..].starts_with(b"/*") {
                        depth += 1;
                        at += 2;
                    } else if bytes[at..].starts_with(b"*/") {
                        depth -= 1;
                        at += 2;
                    } else {
                        at += 1;
                    }
                }
                return Some(Token {
                    text: &source[start..at],
                    span: start..at,
                    comment: true,
                });
            } else if let Some(end) = raw_end(source, at) {
                at = end;
            } else if matches!(bytes[at], b'b' | b'c') && bytes.get(at + 1) == Some(&b'"') {
                at = quoted_end(source, at + 1).expect("prefixed string end");
            } else if bytes[at] == b'"' {
                at = quoted_end(source, at).expect("string end");
            } else if bytes[at] == b'b'
                && bytes.get(at + 1) == Some(&b'\'')
                && let Some(end) = char_end(source, at + 1)
            {
                at = end;
            } else if bytes[at] == b'\''
                && let Some(end) = char_end(source, at)
            {
                at = end;
            } else {
                if source[at..].starts_with("r#") {
                    at += 2;
                }
                let name = at;
                if bytes[at].is_ascii_alphanumeric() || bytes[at] == b'_' || !bytes[at].is_ascii() {
                    while at < bytes.len()
                        && (bytes[at].is_ascii_alphanumeric()
                            || bytes[at] == b'_'
                            || !bytes[at].is_ascii())
                    {
                        at += 1;
                    }
                } else {
                    at += 1;
                }
                return Some(Token {
                    text: &source[name..at],
                    span: start..at,
                    comment: false,
                });
            }
        }
        None
    })
}

// A named use = a call `name(` outside the table: every row is a spec fn, so a parameter, a
// local or a field sharing a row's name never counts, and `fn name(` is its declaration.
fn uses(source: &str, table: Range<usize>) -> Vec<Token<'_>> {
    let code = tokens(source)
        .filter(|token| !token.comment)
        .collect::<Vec<_>>();
    let mut calls = Vec::new();
    for (i, token) in code.iter().enumerate() {
        let call = code.get(i + 1).is_some_and(|next| next.text == "(");
        let declaration = i > 0 && code[i - 1].text == "fn";
        if call && !declaration && !table.contains(&token.span.start) {
            calls.push(Token {
                text: token.text,
                span: token.span.clone(),
                comment: false,
            });
        }
    }
    calls
}

fn orphans(source: &str) -> BTreeSet<&str> {
    let (rows, span) = table(source);
    let mut orphaned = rows
        .into_iter()
        .map(|row| row.name)
        .collect::<BTreeSet<_>>();
    for token in uses(source, span) {
        orphaned.remove(token.text);
    }
    orphaned
}

fn retired(row: &Row<'_>) -> bool {
    DEAD.iter()
        .any(|(name, literal)| row.name == *name || row.literal.as_deref() == Some(literal))
}

#[test]
fn registry_shape() {
    let input = "copy_table! {\n    bare;\n    words = \"line one\nline two\";\n    // hidden;\n    Uppercase;\n    no_space=\"text\";\n    trailing; \n}\n";
    let (rows, _) = table(input);
    assert_eq!(
        rows.iter().map(|row| row.name).collect::<Vec<_>>(),
        ["bare", "words"]
    );
    assert_eq!(rows[0].literal, None);
    assert_eq!(rows[1].literal.as_deref(), Some("line one\nline two"));
    let source = source();
    let (rows, _) = table(&source);
    let names = rows.iter().map(|row| row.name).collect::<BTreeSet<_>>();
    assert_eq!(names.len(), rows.len(), "duplicate copy-table names");
    assert_eq!(rows.len(), 294, "copy_table! row count");
}

#[test]
fn retired_entries_absent() {
    for byte in 0..=127_u8 {
        assert_eq!(
            decode(&format!("\"\\x{byte:02x}\"")),
            Some(char::from(byte).to_string())
        );
    }
    for n in [0, 0x7f, 0x80, 0xd7ff, 0xe000, 0xffff, 0x10000, 0x10ffff] {
        assert_eq!(
            decode(&format!("\"\\u{{{n:x}_}}\"")),
            char::from_u32(n).map(|c| c.to_string())
        );
    }
    for (escape, expected) in [
        ('n', '\n'),
        ('r', '\r'),
        ('t', '\t'),
        ('0', '\0'),
        ('\'', '\''),
        ('"', '"'),
        ('\\', '\\'),
    ] {
        assert_eq!(
            decode(&format!("\"\\{escape}\"")),
            Some(expected.to_string())
        );
    }
    for invalid in [
        "x80",
        "x0",
        "u{}",
        "u{_1}",
        "u{1234567}",
        "u{d800}",
        "u{110000}",
        "q",
    ] {
        assert_eq!(
            decode(&format!("\"\\{invalid}\"")),
            None,
            "invalid escape: {invalid}"
        );
    }
    assert_eq!(
        decode("\"before\\\n \t after\""),
        Some("beforeafter".into())
    );
    assert_eq!(
        decode("\"before\\\r\n \t after\""),
        Some("beforeafter".into())
    );
    assert_eq!(decode("\"before\r\nafter\""), Some("before\nafter".into()));
    for (name, literal) in DEAD {
        assert!(!literal.is_empty(), "contract literal required: {name}");
        for escaped in [
            literal.to_owned(),
            literal.bytes().map(|b| format!("\\x{b:02x}")).collect(),
            literal
                .chars()
                .map(|c| format!("\\u{{{:x}}}", c as u32))
                .collect(),
        ] {
            let input = format!("copy_table! {{\n    renamed = \"{escaped}\";\n}}\n");
            let (rows, _) = table(&input);
            assert_eq!(rows.len(), 1);
            assert!(retired(&rows[0]), "decoded literal: {name}");
        }
        assert!(retired(&Row {
            name,
            literal: Some("changed words".into())
        }));
        assert!(
            !retired(&Row {
                name: &format!("{name}_more"),
                literal: Some(format!("{literal} or only that it occurs later")),
            }),
            "exact entries, not prefixes: {name}"
        );
    }
    let source = source();
    let (rows, _) = table(&source);
    let found = rows
        .iter()
        .filter(|row| retired(row))
        .map(|row| row.name)
        .collect::<Vec<_>>();
    assert!(found.is_empty(), "retired copy entries: {found:?}");
}

#[test]
fn registry_names_have_uses() {
    let source = source();
    let orphaned = orphans(&source);
    assert!(orphaned.is_empty(), "orphan copy names: {orphaned:?}");
}

#[test]
fn census_rejects_orphan_plants() {
    let source = source();
    let (rows, span) = table(&source);
    let commented = tokens(&source)
        .filter(|token| token.comment && !span.contains(&token.span.start))
        .flat_map(|token| token.text.split(|c: char| !c.is_alphanumeric() && c != '_'))
        .collect::<BTreeSet<_>>();
    let used = uses(&source, span);
    let baseline = orphans(&source);
    let live = rows
        .iter()
        .filter(|row| !baseline.contains(row.name))
        .collect::<Vec<_>>();
    assert!(!live.is_empty(), "live copy-table control");
    // Full-source plants: first/last, each 16th live row + every name in non-table comments.
    let sample = live
        .iter()
        .enumerate()
        .filter(|(i, row)| {
            *i == 0 || i + 1 == live.len() || (i + 1) % 16 == 0 || commented.contains(row.name)
        })
        .map(|(_, row)| *row)
        .collect::<Vec<_>>();
    eprintln!(
        "copy census plants: {}/{} live rows; {} comment-named",
        sample.len(),
        live.len(),
        live.iter()
            .filter(|row| commented.contains(row.name))
            .count()
    );
    for row in sample {
        let spans = used
            .iter()
            .filter(|token| token.text == row.name)
            .map(|token| token.span.clone())
            .collect::<Vec<_>>();
        assert!(!spans.is_empty(), "live use control: {}", row.name);
        let mut planted = source.clone();
        for span in spans.iter().rev() {
            planted.replace_range(span.clone(), " ");
        }
        let mut expected = baseline.clone();
        expected.insert(row.name);
        assert_eq!(orphans(&planted), expected, "removed uses: {}", row.name);
        planted.push_str(&format!("\n// {}\n", row.name));
        assert_eq!(
            orphans(&planted),
            expected,
            "comment-only orphan: {}",
            row.name
        );
    }
    let name = live[0].name;
    for mention in [
        format!("// {name}()"),
        format!("/* outer /* nested */ {name}() */"),
        format!("const WORDS: &str = \"escaped \\\" {name}()\";"),
        format!("const WORDS: &str = r##\"quoted \" {name}()\"##;"),
        format!("const WORDS: &[u8] = br#\"{name}()\"#;"),
        format!("const WORDS: &[u8] = b\"{name}()\";"),
        format!("const WORDS: &CStr = c\"{name}()\";"),
        format!("const WORDS: &CStr = cr#\"{name}()\"#;"),
        format!("fn /* between */ {name}() {{}}"),
        format!("fn r#{name}() {{}}"),
        format!("prefix_{name}(); {name}_suffix(); é{name}(); {name}\u{301}();"),
        format!("fn other({name}: ()) {{}}"),
        format!("fn other() {{ let {name} = (); }}"),
    ] {
        let input = format!("copy_table! {{\n    {name} = \"text\";\n}}\n{mention}\n");
        assert_eq!(
            orphans(&input),
            BTreeSet::from([name]),
            "non-use: {mention}"
        );
        for call in [format!("{name}();"), format!("r#{name}();")] {
            assert!(
                orphans(&format!("{input}\n{call}\n")).is_empty(),
                "use control: {mention}"
            );
        }
    }
}
