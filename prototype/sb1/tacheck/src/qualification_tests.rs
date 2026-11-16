use crate::finite_oracle::{Model, Program, set, two_atom_rules};

#[test]
fn b3_reproduces_the_independent_1177_program_t0_table() {
    let domain = two_atom_rules();
    assert_eq!(domain.len(), 48);
    let mut programs = vec![vec![]];
    for r in &domain {
        programs.push(vec![r.clone()]);
    }
    for i in 0..domain.len() {
        for j in i + 1..domain.len() {
            programs.push(vec![domain[i].clone(), domain[j].clone()]);
        }
    }
    let references = [set(&[]), set(&[0]), set(&[1]), set(&[0, 1])];
    let mut outside = 0;
    let mut checked = 0;
    let mut table =
        String::from("program\trules\tenvelope\tlower\tupper\tstable\tgamma_references\n");
    for (id, rules) in programs.iter().enumerate() {
        let p = Program {
            atoms: 2,
            rules: rules.clone(),
        };
        let envelope = p.envelope();
        let q = p.prune(&envelope);
        let mut gamma = Vec::new();
        for reference in &references {
            checked += 1;
            let a = p.gamma(reference);
            let b = q.gamma(reference);
            assert!(
                a.is_subset(&envelope),
                "program {id} reference {reference:?}"
            );
            if reference.is_subset(&envelope) {
                assert_eq!(a, b, "program {id} reference {reference:?}");
            } else if a != b {
                outside += 1;
            }
            gamma.push(a);
        }
        let (lower, upper) = p.wfm();
        assert_eq!((lower.clone(), upper.clone()), q.wfm(), "program {id}");
        let stable = p.stable_models();
        assert_eq!(stable, q.stable_models(), "program {id}");
        assert!(stable.iter().all(|m| m.is_subset(&envelope)));
        table.push_str(&format!(
            "{id}\t{rules:?}\t{envelope:?}\t{lower:?}\t{upper:?}\t{stable:?}\t{gamma:?}\n"
        ));
    }
    assert_eq!(programs.len(), 1177);
    assert_eq!(checked, 4708);
    assert_eq!(outside, 330);
    if let Some(out) = std::env::var_os("SB1_TAS_B3_OUT") {
        let out = std::path::PathBuf::from(out);
        assert!(out.is_absolute());
        std::fs::write(out, table).unwrap();
    }
    println!(
        "B3 PASS programs=1177 references=4708 outside-L-counterexamples=330; independent agenda reduct + AFP + exhaustive stable sets"
    );
}

#[test]
fn b3_pins_the_correlated_naf_and_inactive_active_expectations() {
    use crate::finite_oracle::Rule;
    let r = |head, positive: &[usize], naf| Rule {
        head,
        positive: set(positive),
        naf,
        builtins: true,
    };
    let inactive = Program {
        atoms: 3,
        rules: vec![r(0, &[1], vec![vec![set(&[0])]]), r(2, &[], vec![])],
    };
    let active = Program {
        atoms: 3,
        rules: vec![r(0, &[1], vec![vec![set(&[0])]]), r(1, &[], vec![])],
    };
    assert_eq!(inactive.wfm(), (set(&[2]), set(&[2])));
    assert_eq!(active.wfm(), (set(&[1]), set(&[0, 1])));
    let correlated = Program {
        atoms: 3,
        rules: vec![r(0, &[], vec![vec![set(&[1, 2])]])],
    };
    assert_eq!(correlated.gamma(&set(&[1])), set(&[0]));
    assert_eq!(correlated.gamma(&set(&[1, 2])), Model::new());
    let separate = Program {
        atoms: 3,
        rules: vec![r(0, &[], vec![vec![set(&[1])], vec![set(&[2])]])],
    };
    assert_eq!(separate.gamma(&set(&[1])), Model::new());
    let all_reference_counterexample = Program {
        atoms: 2,
        rules: vec![r(0, &[], vec![vec![set(&[1])]])],
    };
    assert_eq!(all_reference_counterexample.envelope(), set(&[0]));
    assert_eq!(all_reference_counterexample.gamma(&set(&[1])), Model::new());
    assert_eq!(
        all_reference_counterexample
            .prune(&set(&[0]))
            .gamma(&set(&[1])),
        set(&[0])
    );
}
