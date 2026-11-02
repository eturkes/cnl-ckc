use crate::number::{arithmetic, satisfies};
use crate::term::{Arena, parse_term};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

struct Scratch(PathBuf);
impl Drop for Scratch {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

#[test]
fn builtin_probes_match_the_committed_schema_relations() {
    let version = Command::new("swipl").arg("--version").output().unwrap();
    assert!(
        version.status.success() && String::from_utf8_lossy(&version.stdout).contains("10.0.2")
    );
    let directory = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target/schema-tests")
        .join(std::process::id().to_string());
    fs::create_dir_all(&directory).unwrap();
    let _scratch = Scratch(directory.clone());
    let mut arena = Arena::default();
    let mut cases = Vec::new();
    let mut expected = Vec::new();
    let cmps = [
        "eq", "exactly", "geq", "greater", "leq", "less", "about", "na",
    ];
    let numbers = [
        "-2",
        "0",
        "3",
        "rational(-1,2)",
        "rational(3,2)",
        "rational(2,2)",
        "a",
    ];
    for name in ["guideline_product", "guideline_quotient"] {
        for c1 in cmps {
            for c2 in cmps {
                for n1 in numbers {
                    for n2 in numbers {
                        let args = format!("{c1},{n1},{c2},{n2}");
                        let root = parse_term(&mut arena, &format!("x({args})")).unwrap();
                        let args_native = arena.call(root).unwrap().1.to_vec();
                        let result = arithmetic(&mut arena, name, &args_native)
                            .map(|(c, n)| format!("r({},{})", arena.render(c), arena.render(n)));
                        cases.push(format!("{name}({args},A,B),r(A,B)"));
                        expected.push(result.into_iter().collect::<Vec<_>>().join(","));
                    }
                }
            }
        }
    }
    for c1 in cmps {
        for c2 in cmps {
            for n1 in numbers {
                for n2 in numbers {
                    let args = format!("{c1},{n1},{c2},{n2}");
                    let root = parse_term(&mut arena, &format!("x({args})")).unwrap();
                    let answer = satisfies(&arena, arena.call(root).unwrap().1);
                    cases.push(format!("guideline_satisfies({args}),yes"));
                    expected.push(if answer { "yes" } else { "" }.into());
                }
            }
        }
    }
    let units = [
        "second", "minute", "hour", "day", "week", "month", "year", "unknown",
    ];
    for u in units {
        for v in units {
            for cmp in ["exactly", "geq", "greater", "leq", "less"] {
                let args = format!("exactly,1,{u},{cmp},1,{v}");
                let root = parse_term(&mut arena, &format!("x({args})")).unwrap();
                let answer = satisfies(&arena, arena.call(root).unwrap().1);
                cases.push(format!("guideline_satisfies({args}),yes"));
                expected.push(if answer { "yes" } else { "" }.into());
            }
        }
    }
    let huge = "12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890";
    for name in ["guideline_product", "guideline_quotient"] {
        let args = format!("geq,{huge},exactly,rational(3,20)");
        let root = parse_term(&mut arena, &format!("x({args})")).unwrap();
        let native = arena.call(root).unwrap().1.to_vec();
        let (c, n) = arithmetic(&mut arena, name, &native).unwrap();
        cases.push(format!("{name}({args},A,B),r(A,B)"));
        expected.push(format!("r({},{})", arena.render(c), arena.render(n)));
    }
    let mut script = String::from(
        "main :- current_prolog_flag(argv,[Schema]), load_files(Schema,[silent(true)]), forall(probe(I,G,R),(findall(R,call(G),Rs),sort(Rs,L),write_term(row(I,L),[quoted(true),ignore_ops(true),numbervars(true)]),write('.'),nl)),halt.\n",
    );
    for (i, case) in cases.iter().enumerate() {
        script.push_str(&format!("probe({i},{case}).\n"));
    }
    let script_path = directory.join("probes.prolog");
    fs::write(&script_path, script).unwrap();
    let result = Command::new("swipl")
        .args(["-q", "-f", "none", "-F", "none", "-s"])
        .arg(&script_path)
        .args(["-g", "main", "-t", "halt(9)", "--"])
        .arg(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../guidelines/schema.pl"))
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(
        result.stderr.is_empty(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let stdout = String::from_utf8(result.stdout).unwrap();
    let actual = stdout.lines().collect::<Vec<_>>();
    assert_eq!(actual.len(), cases.len());
    assert_eq!(actual.len(), 9730);
    for (i, (actual, expected)) in actual.iter().zip(&expected).enumerate() {
        assert_eq!(*actual, format!("row({i},[{expected}])."), "{}", cases[i]);
    }
}
