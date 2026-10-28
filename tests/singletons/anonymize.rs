// Contract sgw D1 as a text-level Definition: the letter spelling of one canonical clause line →
// its emitted spelling. A variable run (an identifier run opening with A-Z, outside quoted
// atoms) that occurs once in the line becomes `_`; every other run stays, so the letters keep
// their numbers over all variables. Suites whose models print the letter form apply it to the
// expected side (as tests/bf/reorder.rs does for binding-first order).
pub fn anonymize(line: &str) -> String {
    let runs = variable_runs(line);
    let mut count = std::collections::BTreeMap::new();
    for (start, end) in &runs {
        *count.entry(&line[*start..*end]).or_insert(0usize) += 1;
    }
    let mut out = String::new();
    let mut at = 0;
    for (start, end) in &runs {
        if count[&line[*start..*end]] == 1 {
            out.push_str(&line[at..*start]);
            out.push('_');
            at = *end;
        }
    }
    out.push_str(&line[at..]);
    out
}

// Byte spans of the variable runs, in order.
fn variable_runs(line: &str) -> Vec<(usize, usize)> {
    let b = line.as_bytes();
    let ident = |c: u8| c.is_ascii_alphanumeric() || c == b'_';
    let (mut runs, mut i, mut quoted, mut escaped) = (Vec::new(), 0, false, false);
    while i < b.len() {
        let c = b[i];
        if quoted {
            if escaped {
                escaped = false;
            } else if c == b'\\' {
                escaped = true;
            } else if c == b'\'' {
                quoted = false;
            }
            i += 1;
        } else if c == b'\'' {
            quoted = true;
            i += 1;
        } else if ident(c) {
            let start = i;
            while i < b.len() && ident(b[i]) {
                i += 1;
            }
            if c.is_ascii_uppercase() {
                runs.push((start, i));
            }
        } else {
            i += 1;
        }
    }
    runs
}
