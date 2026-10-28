use super::model::*;
use super::support::*;
use ckc_kernel::{ESrc, EV1Verdict, contract};

fn verdict(bytes: &[u8]) -> Option<usize> {
    match contract::v1_check(bytes) {
        EV1Verdict::Ok => None,
        EV1Verdict::Reject { at } => Some(at),
    }
}

fn check_file(s: &Scratch, checks: &mut Checks, label: &str, bytes: &[u8], offset: Option<usize>) {
    checks.require(
        verdict(bytes) == offset,
        format!(
            "{label} kernel: got {:?}, expected {offset:?}",
            verdict(bytes)
        ),
    );
    let file = s.file("input.pl", bytes);
    let got = native(&s.0, &["v1", "check", file.to_str().unwrap()]);
    let (rc, err) = if let Some(at) = offset {
        assert!(at <= bytes.len(), "oracle offset {label}");
        let prefix = &bytes[..at];
        let line = prefix.iter().filter(|&&b| b == b'\n').count() + 1;
        let col = at
            - prefix
                .iter()
                .rposition(|&b| b == b'\n')
                .map_or(0, |i| i + 1)
            + 1;
        (
            2,
            format!("ace_to_pl_error(check_load,noncanonical({line},{col})).\n"),
        )
    } else {
        (0, String::new())
    };
    checks.require(
        got.rc == rc && got.err == err.as_bytes() && got.out.is_empty(),
        format!(
            "{label} CLI expected rc={rc} stderr={err:?}; {}",
            got.description()
        ),
    );
}

fn trace_lines(
    doc: &[u8],
    query: &[u8],
    supplied_qsha: &str,
    answer_qsha: &str,
) -> Result<Vec<Vec<u8>>, ckc_kernel::EOut> {
    contract::v1_trace_lines(
        b"manifest.tsv",
        &ESrc::Bytes(b"doc.pl\tpayload.pl\n".to_vec()),
        &vec![ESrc::Bytes(doc.to_vec())],
        &vec![ESrc::Bytes(vec![])],
        &ESrc::Bytes(query.to_vec()),
        supplied_qsha.as_bytes(),
        &ESrc::Bytes(answers(answer_qsha, "no(finite_failure)").into_bytes()),
    )
}

fn check_lines(checks: &mut Checks, label: &str, raw: &[u8], clauses: &str) {
    let q = query("guideline_event(actual,sgw,missing)");
    let digest = sha(&q);
    match trace_lines(raw, q.as_bytes(), &digest, &digest) {
        Ok(lines) => {
            // replay::doc_db = the two document records, then its semantic clauses.
            let expected: Vec<Vec<u8>> = doc(clauses)
                .split_inclusive('\n')
                .filter(|l| l.starts_with("guideline_"))
                .map(|l| l.as_bytes().to_vec())
                .collect();
            checks.require(
                lines == expected,
                format!("{label} trace canonical lines differ from generated letter model"),
            );
        }
        Err(err) => checks.require(
            false,
            format!(
                "{label} trace-lines rc={} {}",
                err.rc,
                String::from_utf8_lossy(&err.err)
            ),
        ),
    }
}

fn swi_variants(s: &Scratch, pairs: &[(String, String, bool)]) {
    let mut source = String::from(
        "main :- forall(pair(I,A,B,W), (read_term_from_atom(A,X,[]), read_term_from_atom(B,Y,[]), (X =@= Y -> V=yes ; V=no), (V == W -> true ; throw(variant(I,V,W))))), halt.\n",
    );
    let mut count = 0;
    for (a, b, want) in pairs {
        assert_eq!(a.lines().count(), b.lines().count(), "clause-pair census");
        for (left, right) in a.lines().zip(b.lines()) {
            source.push_str(&format!(
                "pair({count},{},{},{}).\n",
                quoted(left),
                quoted(right),
                if *want { "yes" } else { "no" }
            ));
            count += 1;
        }
    }
    let driver = s.file("variants.pl", source);
    let got = swi(
        &s.0,
        &format!("consult({}),main.", quoted(driver.to_str().unwrap())),
    );
    got.require("SWI read_term variants");
    println!("SGW SWI variants: {count} clause pairs");
}

#[test]
fn p4a_literal_boundaries() {
    let cases: &[(&[u8], &[u8])] = &[
        (b"", b""),
        (b"_", b"A"),
        (b"_ _ _", b"A B C"),
        (b"A _ A _", b"A B A C"),
        (b"B B _", b"B B B"),
        (b"_ A A _", b"A \0 \0 B"),
        (b"_ B _ C", b"A B C \0"),
        (b"_a _X __ a_", b"_a _X __ a_"),
        (b"A1 A1 _", b"A1 A1 B"),
        (b"1A A_ _", b"1A A_ B"),
        (b"'_' _", b"'_' A"),
        (b"'_A A' _", b"'_A A' A"),
        (b"'a\\'_ A' _", b"'a\\'_ A' A"),
        (b"'a\\\\' _", b"'a\\\\' A"),
        (b"'_\n_ A", b"'_\nA \0"),
        (b"'\\\n_", b"'\\\nA"),
        (b"% _ A ' \\\n_", b"% _ A ' \\\nA"),
        (b" % _ A", b" % A \0"),
        (b"\r% _ A", b"\r% A \0"),
        (b"A\n_", b"A\nA"),
        (b"_\r_", b"A\rB"),
        (b"_\n_\n", b"A\nA\n"),
    ];
    let mut checks = Checks::default();
    for (i, &(raw, expected)) in cases.iter().enumerate() {
        assert_eq!(read(raw).text, expected, "independent reader literal {i}");
        checks.bytes(&contract::v1_read(raw), expected, &format!("literal {i}"));
    }
    for byte in 0..=255u8 {
        for (i, template) in [
            vec![b'_', byte, b'_'],
            vec![b'\'', byte, b'\'', b' ', b'_'],
            vec![b'%', byte, b'_', b'\n', b'_'],
            vec![b'\'', b'\\', byte, b'_', b'\n', b'_'],
        ]
        .into_iter()
        .enumerate()
        {
            checks.bytes(
                &contract::v1_read(&template),
                &read(&template).text,
                &format!("byte {byte} context {i}"),
            );
        }
    }
    checks.finish("P4a literal + all-byte contexts");
}

#[test]
fn p4a_generated_total_reading() {
    let tokens: &[&[u8]] = &[
        b"_",
        b"A",
        b"B",
        b"A1",
        b"Z",
        b"_a",
        b"_X",
        b"__",
        b"a_",
        b"1",
        b"'_'",
        b"'a\\'_A'",
        b"%_",
        b"\n",
        b"\r",
        b"\0",
    ];
    let mut checks = Checks::default();
    for (i, a) in tokens.iter().enumerate() {
        for (j, b) in tokens.iter().enumerate() {
            for (k, c) in tokens.iter().enumerate() {
                let input = [*a, b" ", *b, b",", *c].concat();
                let reference = read(&input);
                checks.bytes(
                    &contract::v1_read(&input),
                    &reference.text,
                    &format!("tokens {i}/{j}/{k}"),
                );
                assert_eq!(
                    read(&reference.text).text,
                    reference.text,
                    "reader idempotence"
                );
                assert_eq!(reference.raw_at.len(), reference.text.len() + 1);
                assert!(reference.raw_at.windows(2).all(|w| w[0] <= w[1]));
                assert_eq!(reference.raw_at.last(), Some(&input.len()));
            }
        }
    }
    let alphabet = b"_AABCZ19abc,()'\\\n%\r\0\x80\xff";
    let mut state = 0xc8e9_7b5a_921d_f053u64;
    for case in 0..2048 {
        let mut raw = Vec::new();
        for _ in 0..case % 257 {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            raw.push(alphabet[state as usize % alphabet.len()]);
        }
        checks.bytes(
            &contract::v1_read(&raw),
            &read(&raw).text,
            &format!("generated bytes {case}"),
        );
    }
    for n in [25, 26, 27, 51, 52, 259, 260, 261, 2599, 2600] {
        let raw = std::iter::repeat_n("_", n + 1)
            .collect::<Vec<_>>()
            .join(",");
        let expected = (0..=n).map(variable).collect::<Vec<_>>().join(",");
        checks.bytes(
            &contract::v1_read(raw.as_bytes()),
            expected.as_bytes(),
            &format!("numbering through {n}"),
        );
    }
    checks.finish("P4a generated reader differential");
}

#[test]
fn p4b_generated_documents_cli_models_and_swi() {
    let s = Scratch::new("documents");
    let mut checks = Checks::default();
    let mut variants = Vec::new();
    for n in [1, 2, 3, 24, 25, 26, 27, 28, 51, 52, 53, 259, 260, 261] {
        for seed in 0..3 {
            let clauses = generated_clauses(n, seed);
            let canonical = doc(&clauses);
            let raw = anonymize(canonical.as_bytes());
            let label = format!("vars={n} mask={seed}");
            assert_eq!(
                read(&raw).text,
                canonical.as_bytes(),
                "generator round trip {label}"
            );
            check_file(
                &s,
                &mut checks,
                &format!("{label} letter control"),
                canonical.as_bytes(),
                None,
            );
            check_file(&s, &mut checks, &label, &raw, None);
            check_lines(
                &mut checks,
                &format!("{label} letter control"),
                canonical.as_bytes(),
                &clauses,
            );
            check_lines(&mut checks, &label, &raw, &clauses);
            variants.push((
                String::from_utf8(anonymize(clauses.as_bytes())).unwrap(),
                clauses,
                true,
            ));
        }
    }
    for clauses in [
        "guideline_event(A,B,C).\n",
        "guideline_event('_',A,x).\n",
        "guideline_event('é_\\\'_',A,x).\n",
        "guideline_event(actual,A,x) :- \\+ (guideline_event(actual,B,y), guideline_event(actual,B,z)).\n",
        "guideline_event(actual,A,x).\nguideline_event(actual,A,y).\n",
    ] {
        let canonical = doc(clauses);
        let raw = anonymize(canonical.as_bytes());
        check_file(&s, &mut checks, clauses, &raw, None);
        check_lines(&mut checks, clauses, &raw, clauses);
        variants.push((
            String::from_utf8(anonymize(clauses.as_bytes())).unwrap(),
            clauses.to_owned(),
            true,
        ));
    }
    variants.push((
        "guideline_event(_,A,x).".into(),
        "guideline_event(A,A,x).".into(),
        false,
    ));
    variants.push((
        "guideline_event(_,_,x).".into(),
        "guideline_event(A,A,x).".into(),
        false,
    ));
    swi_variants(&s, &variants);
    checks.finish("P4b generated documents + canonical trace lines");
}

#[test]
fn p4b_rejection_and_literal_controls() {
    let s = Scratch::new("controls");
    let mut checks = Checks::default();
    for run in ["_a", "_X", "__"] {
        let text = doc(&format!("guideline_event(actual,{run},x).\n"));
        let at = text.rfind(run).unwrap();
        assert_eq!(read(text.as_bytes()).text, text.as_bytes());
        check_file(&s, &mut checks, run, text.as_bytes(), Some(at));
    }
    for value in ["a_", "'_'", "'é_A'", "'a\\\'_A'", "'a\\\\_A'"] {
        let text = doc(&format!("guideline_event(actual,{value},x).\n"));
        check_file(&s, &mut checks, value, text.as_bytes(), None);
    }
    for (raw_clause, token) in [
        ("guideline_event(_,A,x).\n", "A"),
        ("guideline_event(_,C,x).\n", "C"),
        ("guideline_event(A,_,B).\n", "B"),
    ] {
        let raw = doc(raw_clause);
        check_file(
            &s,
            &mut checks,
            raw_clause,
            raw.as_bytes(),
            Some(raw.rfind(token).unwrap()),
        );
    }
    for (kind, file, letter) in [
        (
            "answers",
            answers(&"a".repeat(64), "_"),
            answers(&"a".repeat(64), "A"),
        ),
        ("traces", traces("_"), traces("A")),
    ] {
        // D5 preserves the canonical parser's diagnostic, including its class fallback;
        // the non-ground result need not be the reported divergence. `_` → A keeps offsets.
        let reading = read(file.as_bytes());
        assert_eq!(reading.text, letter.as_bytes());
        assert!(reading.raw_at.iter().enumerate().all(|(i, &at)| i == at));
        let at = verdict(letter.as_bytes()).expect("non-ground letter result must reject");
        check_file(
            &s,
            &mut checks,
            &format!("{kind} letter control"),
            letter.as_bytes(),
            Some(at),
        );
        check_file(&s, &mut checks, kind, file.as_bytes(), Some(at));
        println!(
            "SGW P4b ground {kind}: letter offset={at}; raw offset={:?}",
            verdict(file.as_bytes())
        );
    }
    for file in [answers(&"a".repeat(64), "'_'"), traces("'_'")] {
        check_file(&s, &mut checks, "quoted ground data", file.as_bytes(), None);
    }
    checks.finish("P4b negative + quoted controls");
}

#[test]
fn p4b_raw_question_alias_and_custody() {
    let s = Scratch::new("question");
    let mut checks = Checks::default();
    let raw = query("guideline_event(actual,_,x)");
    let canonical = query("guideline_event(actual,A,x)");
    let doc = doc("guideline_event(actual,sgw,x).\n");
    let raw_digest = sha(&raw);
    let canonical_digest = sha(&canonical);
    assert_ne!(raw_digest, canonical_digest, "raw query custody control");
    assert_eq!(read(raw.as_bytes()).text, canonical.as_bytes());
    check_file(&s, &mut checks, "raw query alias", raw.as_bytes(), None);
    check_file(&s, &mut checks, "letter query", canonical.as_bytes(), None);
    let got = trace_lines(doc.as_bytes(), raw.as_bytes(), &raw_digest, &raw_digest);
    checks.require(
        got.is_ok(),
        "raw query + raw digest must pass trace custody",
    );
    let wrong = trace_lines(
        doc.as_bytes(),
        raw.as_bytes(),
        &raw_digest,
        &canonical_digest,
    );
    checks.require(matches!(wrong, Err(e) if e.rc == 2 && String::from_utf8_lossy(&e.err).contains("query_sha256_mismatch")), "letter digest on raw query must reject at query_sha256_mismatch");
    let rendered = native(
        &s.0,
        &[
            "v1",
            "render",
            s.file("raw-query.pl", &raw).to_str().unwrap(),
        ],
    );
    checks.require(
        rendered.clean() && rendered.out == raw.as_bytes(),
        "v1 render must preserve raw question alias bytes",
    );
    checks.finish("P4b query alias + raw custody");
}

#[test]
fn p7_offsets_after_expansion_and_collision() {
    let s = Scratch::new("offsets");
    let mut checks = Checks::default();
    check_file(
        &s,
        &mut checks,
        "earlier error beats late collision",
        b"!\nguideline_event(_,A,x).\n",
        Some(0),
    );
    for n in [1, 2, 26, 27, 28, 52, 261] {
        let values = std::iter::repeat_n("_", n).collect::<Vec<_>>().join(",");
        for prefix in ["actual", "'é'", "'漢字_\\\''"] {
            let text = doc(&format!("guideline_event({prefix},[{values}],x).\n"));
            let start = text.rfind("guideline_event(").unwrap();
            for (i, b) in text.as_bytes().iter().enumerate().skip(start) {
                if *b == b'_' && i > text.rfind('[').unwrap() {
                    let input = &text.as_bytes()[..i + 1];
                    check_file(
                        &s,
                        &mut checks,
                        &format!("EOF n={n} prefix={prefix} byte={i}"),
                        input,
                        Some(input.len()),
                    );
                }
            }
            let mut corrupted = text.into_bytes();
            let at = corrupted.len() - 2;
            corrupted[at] = b'!';
            check_file(
                &s,
                &mut checks,
                &format!("after expansions n={n} prefix={prefix}"),
                &corrupted,
                Some(at),
            );
        }
        let collision = variable(n - 1);
        let text = doc(&format!(
            "guideline_event(actual,[{values},{collision}],x).\n"
        ));
        let at = text.rfind(&collision).unwrap();
        check_file(
            &s,
            &mut checks,
            &format!("refused run {collision}"),
            text.as_bytes(),
            Some(at),
        );
    }
    let prefix = (0..26).map(variable).collect::<Vec<_>>().join(",");
    let raw = doc(&format!("guideline_event(actual,[{prefix},_,A1],x).\n"));
    check_file(
        &s,
        &mut checks,
        "refused multi-byte run begins at A",
        raw.as_bytes(),
        Some(raw.rfind("A1").unwrap()),
    );
    checks.finish("P7 reject offsets");
}
