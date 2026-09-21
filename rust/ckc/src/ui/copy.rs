use super::common::*;
use std::fs;
use std::path::Path;

fn literal(s: &str) -> Option<String> {
    let (_, rhs) = s.split_once('=')?;
    let mut chars = rhs.trim().strip_prefix('"')?.chars();
    let mut out = String::new();
    while let Some(c) = chars.next() {
        if c == '"' {
            return chars.as_str().trim().is_empty().then_some(out);
        }
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next()? {
            '\\' => out.push('\u{5C}'),
            '"' => out.push('"'),
            'n' => out.push('\n'),
            't' => out.push('\t'),
            escape @ ('x' | 'u' | 'U') => {
                let len = match escape {
                    'x' => 2,
                    'u' => 4,
                    _ => 8,
                };
                let mut value = 0u32;
                for _ in 0..len {
                    value = value
                        .checked_mul(16)?
                        .checked_add(chars.next()?.to_digit(16)?)?;
                }
                out.push(char::from_u32(value)?);
            }
            _ => return None,
        }
    }
    None
}
pub(super) fn check(path: &Path) -> Result<u8> {
    let source =
        fs::read_to_string(path).map_err(|e| format!("ui: copy-check: {}: {e}", path.display()))?;
    let mut rejected = false;
    for (index, line) in source.lines().enumerate() {
        if line.trim().is_empty() || line.trim_start().starts_with('#') {
            continue;
        }
        let value = literal(line)
            .ok_or_else(|| format!("ui: copy-check: malformed literal at line {}", index + 1))?;
        if let Some(detail) = ckc_kernel::contract::ui_copy_violation(value.as_bytes()) {
            println!("{} {}", index + 1, text(&detail));
            rejected = true;
        }
    }
    Ok(u8::from(rejected))
}
