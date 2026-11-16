use std::fs;
use std::path::Path;

fn check(source: &str) -> Result<(), &'static str> {
    if source.contains("sb1_ta") {
        return Err("census dependency");
    }
    if source
        .split(|c: char| !c.is_ascii_alphanumeric() && c != '_')
        .any(|token| token == "dyn")
    {
        return Err("trait object");
    }
    for (_, suffix) in source
        .match_indices("sb1_checker::")
        .map(|(i, _)| (i, &source[i + "sb1_checker::".len()..]))
    {
        let module = suffix
            .split(|c: char| !c.is_ascii_alphanumeric() && c != '_')
            .next()
            .unwrap();
        if !["program", "term", "query"].contains(&module) {
            return Err("non-IR shared dependency");
        }
    }
    for needle in ["HashMap<", "HashMap::<", "BTreeMap<", "BTreeMap::<"] {
        for (at, _) in source.match_indices(needle) {
            let suffix = &source[at + needle.len()..];
            let key = suffix.split(',').next().unwrap().trim();
            if !["u32", "u64", "usize"].contains(&key) {
                return Err("nonprimitive explicit map key");
            }
        }
    }
    Ok(())
}
#[test]
fn checking_core_keeps_the_declared_primitive_ir_boundary() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/core");
    let mut checked = 0;
    for entry in fs::read_dir(directory).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            check(&fs::read_to_string(&path).unwrap())
                .unwrap_or_else(|e| panic!("{}: {e}", path.display()));
            checked += 1;
        }
    }
    assert!(checked >= 8);
    let control = "use sb1_checker::program::Clause; let a: HashMap<u32, Vec<usize>>;";
    assert_eq!(check(control), Ok(()));
    for plant in [
        "use sb1_ta::features::Model;",
        "use sb1_checker::engine::Model;",
        "let x: HashMap<(u32,u32), bool>;",
        "let x: &dyn Fn();",
    ] {
        assert!(check(plant).is_err(), "{plant}");
    }
}
