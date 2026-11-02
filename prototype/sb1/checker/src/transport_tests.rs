use crate::term::{Arena, parse_checked, parse_term};

fn compare(text: &str, period: bool) {
    let mut arena = Arena::default();
    let before = parse_term(&mut arena, text);
    let after = parse_checked(&mut arena, text, period);
    match (before, after) {
        (Ok(a), Ok((b, canonical))) => {
            assert_eq!(a, b, "{text:?}");
            let mut image = arena.render(a);
            if period {
                image.push('.');
            }
            assert_eq!(canonical, image == text, "{text:?} => {image:?}");
        }
        (Err(_), Err(_)) => {}
        (a, b) => panic!("parse domain changed: {text:?}: {a:?} / {b:?}"),
    }
}

#[test]
fn checked_tokens_equal_parse_render_on_explicit_spelling_domains_and_mutations() {
    let seeds = [
        "guideline_property(actual,a,p,true)",
        "guideline_entity(actual,'$guideline_id'(product,d,1,ref(2),['$guideline_id'(product,p,1,ref(1),[])]),'Ω',countable)",
        "[=(v1,'x\\n'),=(v2,rational(-1,2)),=(v3,'\\u0000')]",
        "[a,b|tail]",
        "{f(a)}",
        "-1000000000000000000000000000000000000000000000",
        "'\\u001F'",
        "'\\u007F'",
        "'a\\\'b\\\\c'",
        "'[]'",
        "[]",
        "{}",
        ";",
        "!",
        "A",
        "f(A,B,A)",
    ];
    for seed in seeds {
        for period in [false, true] {
            let source = format!("{seed}{}", if period { "." } else { "" });
            compare(&source, period);
            for at in 0..=source.len() {
                if !source.is_char_boundary(at) {
                    continue;
                }
                for insert in [
                    " ", "\t", "\r", ".", "'", "_", "[", "]", "|", ",", "0", "\\", "Ω",
                ] {
                    let mut text = source.clone();
                    text.insert_str(at, insert);
                    compare(&text, period);
                }
                if at < source.len() {
                    let mut text = source.clone();
                    text.remove(at);
                    compare(&text, period);
                }
            }
        }
    }
    for text in [
        "[a|[]]",
        "[a|[b]]",
        "'[|]'(a,[])",
        "'{}'(a)",
        "'a'",
        "'\\u0061'",
        "'\\u001f'",
        "_",
        "f(_,_)",
        "'x\ny'",
        "f(A1)",
    ] {
        compare(text, false);
    }
}
