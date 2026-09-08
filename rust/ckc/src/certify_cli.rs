// M6 shell: immutable input snapshots, upstream APE staging, duplicate DRS
// runs, source hashes, verified certification. Derived payloads stay in memory.
use ckc_kernel::EOut;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};

struct Stage(PathBuf);

impl Drop for Stage {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn copy_tree(from: &Path, to: &Path) -> io::Result<()> {
    fs::create_dir(to)?;
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let target = to.join(entry.file_name());
        if entry.path().is_dir() {
            copy_tree(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), target)?;
        }
    }
    Ok(())
}

fn read_text(path: &Path, role: &str) -> Result<Vec<u8>, String> {
    let bytes = fs::read(path).map_err(|_| format!("unreadable({role})."))?;
    std::str::from_utf8(&bytes).map_err(|_| format!("invalid_utf8({role})."))?;
    Ok(bytes)
}

fn optional_ulex(path: &Path) -> Result<Option<Vec<u8>>, String> {
    match fs::read(path) {
        Ok(bytes) => {
            std::str::from_utf8(&bytes).map_err(|_| "invalid_utf8(ulex).".to_owned())?;
            Ok(Some(bytes))
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(_) => Err("unreadable(ulex).".to_owned()),
    }
}

impl Stage {
    fn new(root: &Path, ulex: Option<&[u8]>) -> Result<Self, String> {
        let mut index = 0u64;
        let stage = loop {
            let path =
                std::env::temp_dir().join(format!("ckc-certify-{}-{index}", std::process::id()));
            match fs::create_dir(&path) {
                Ok(()) => break Self(path),
                Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
                    index = index.checked_add(1).ok_or("stage_unavailable.")?;
                }
                Err(_) => return Err("stage_unavailable.".to_owned()),
            }
        };
        let ape = stage.0.join("ape-stage");
        copy_tree(&root.join("vendor/ape"), &ape).map_err(|_| "stage_copy_failed.")?;
        fs::copy(
            root.join("vendor/clex/clex_lexicon.pl"),
            ape.join("prolog/lexicon/clex_lexicon.pl"),
        )
        .map_err(|_| "stage_clex_failed.")?;
        if let Some(bytes) = ulex {
            fs::write(stage.0.join("lexicon.ulex"), bytes).map_err(|_| "stage_ulex_failed.")?;
        }
        let parser = ape.join("prolog/parser");
        let parser_text = parser
            .to_str()
            .ok_or("stage_path_utf8.")?
            .replace('\\', "\\\\")
            .replace('\'', "\\'");
        let goal = format!("working_directory(_, '{parser_text}'), [fit_to_plp], halt.");
        let output = Command::new("swipl")
            .args(["-O", "-f", "none", "-F", "none", "-g", &goal, "-t", "halt"])
            .output()
            .map_err(|_| "swipl_unavailable.")?;
        if !output.status.success() || !parser.join("grammar.plp").is_file() {
            return Err("stage_grammar_failed.".to_owned());
        }
        Ok(stage)
    }

    fn dump(&self, root: &Path, ace: &[u8], with_ulex: bool) -> Result<Vec<u8>, String> {
        let mut command = Command::new("swipl");
        command
            .args(["-q", "-f", "none", "-F", "none", "-s"])
            .arg(root.join("rust/ckc/prolog/drs_dump.pl"))
            .args(["-g", "main", "-t", "halt(9)", "--"])
            .arg(self.0.join("ape-stage"));
        if with_ulex {
            command.arg(self.0.join("lexicon.ulex"));
        }
        let mut child = command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|_| "swipl_unavailable.")?;
        let written = child
            .stdin
            .take()
            .ok_or_else(|| io::Error::other("driver stdin"))
            .and_then(|mut stdin| stdin.write_all(ace));
        if written.is_err() {
            let _ = child.kill();
            let _ = child.wait();
            return Err("driver_input_failed.".to_owned());
        }
        let output = child
            .wait_with_output()
            .map_err(|_| "driver_wait_failed.")?;
        if !output.status.success() {
            return Err(format!(
                "driver_failed({}).",
                output.status.code().unwrap_or(-1)
            ));
        }
        std::str::from_utf8(&output.stdout).map_err(|_| "invalid_utf8(dump).")?;
        Ok(output.stdout)
    }
}

fn certify(
    query: bool,
    id: &str,
    ace: &[u8],
    usha: Option<&Vec<u8>>,
    dump: &[u8],
    pl: &[u8],
) -> EOut {
    let asha = crate::trust::sha256_hex(ace);
    if query {
        ckc_kernel::contract::certify_query(ace, asha.as_bytes(), usha, id.as_bytes(), dump, pl)
    } else {
        ckc_kernel::contract::certify_doc(ace, asha.as_bytes(), usha, id.as_bytes(), dump, pl)
    }
}

fn reject(id: &str, why: &str) -> ExitCode {
    eprintln!("ckc: certify: {id}: {why}");
    ExitCode::from(1)
}

fn emit(output: EOut) -> ExitCode {
    if std::io::stdout().write_all(&output.out).is_err()
        || std::io::stderr().write_all(&output.err).is_err()
    {
        return ExitCode::from(1);
    }
    ExitCode::from(output.rc)
}

fn name_ok(id: &str) -> bool {
    !id.is_empty()
        && id
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}

pub fn run_one(args: &[String]) -> ExitCode {
    let query = args[0] == "query";
    let id = &args[1];
    if (!query && args[0] != "doc") || !name_ok(id) {
        return reject(id, "arguments.");
    }
    let inputs = (|| {
        let ace = read_text(Path::new(&args[2]), "ace")?;
        let ulex = if args[3] == "-" {
            None
        } else {
            Some(read_text(Path::new(&args[3]), "ulex")?)
        };
        let dump = read_text(Path::new(&args[4]), "dump")?;
        let pl = read_text(Path::new(&args[5]), "pl")?;
        Ok::<_, String>((ace, ulex, dump, pl))
    })();
    match inputs {
        Err(why) => reject(id, &why),
        Ok((ace, ulex, dump, pl)) => {
            let usha = ulex
                .as_ref()
                .map(|u| crate::trust::sha256_hex(u).into_bytes());
            emit(certify(query, id, &ace, usha.as_ref(), &dump, &pl))
        }
    }
}

fn ace_paths(dir: &Path, optional: bool) -> Result<Vec<PathBuf>, String> {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(e) if optional && e.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(_) => return Err("unreadable(ace_directory).".to_owned()),
    };
    let mut paths = Vec::new();
    for entry in entries {
        let path = entry.map_err(|_| "unreadable(ace_directory).")?.path();
        if path.extension().is_some_and(|e| e == "ace") {
            paths.push(path);
        }
    }
    paths.sort();
    Ok(paths)
}

pub fn run(id: &str) -> ExitCode {
    if !name_ok(id) {
        return reject(id, "guideline_id.");
    }
    let root = match std::env::current_dir() {
        Ok(root) => root,
        Err(_) => return reject(id, "repository_root."),
    };
    let guideline = root.join("guidelines").join(id);
    let inputs = (|| {
        let docs = ace_paths(&guideline.join("ace"), false)?;
        let queries = ace_paths(&guideline.join("queries"), true)?;
        let ulex = optional_ulex(&guideline.join("lexicon.ulex"))?;
        let stage = Stage::new(&root, ulex.as_deref())?;
        Ok::<_, String>((docs, queries, ulex, stage))
    })();
    let (docs, queries, ulex, stage) = match inputs {
        Ok(inputs) => inputs,
        Err(why) => return reject(id, &why),
    };
    let usha = ulex
        .as_ref()
        .map(|u| crate::trust::sha256_hex(u).into_bytes());
    let mut payloads = Vec::new();
    for (query, paths) in [(false, &docs), (true, &queries)] {
        for path in paths {
            let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
                return reject(id, "artifact_name.");
            };
            if !name_ok(stem) {
                return reject(id, "artifact_name.");
            }
            let pl_dir = if query { "queries/pl" } else { "pl" };
            let result = (|| {
                let ace = read_text(path, "ace")?;
                let pl = read_text(&guideline.join(pl_dir).join(format!("{stem}.pl")), "pl")?;
                let first = stage.dump(&root, &ace, ulex.is_some())?;
                let second = stage.dump(&root, &ace, ulex.is_some())?;
                if first != second {
                    return Err("dump_nondeterministic.".to_owned());
                }
                Ok(certify(query, stem, &ace, usha.as_ref(), &first, &pl))
            })();
            match result {
                Err(why) => return reject(stem, &why),
                Ok(output) if output.rc != 0 => return emit(output),
                Ok(output) if !query => payloads.push(output.out),
                Ok(_) => {}
            }
        }
    }
    println!(
        "ckc: certify ok {id} {} documents {} queries",
        docs.len(),
        queries.len()
    );
    ExitCode::SUCCESS
}
