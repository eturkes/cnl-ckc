use super::Result;
use std::collections::BTreeSet;
use std::env;
use std::path::{Path, PathBuf};
use std::time::Duration;

pub(super) struct Config {
    pub root: PathBuf,
    pub target: PathBuf,
    pub paired: Option<PathBuf>,
    pub producer: String,
    pub checker: String,
    pub orchestrator: Option<String>,
    // Warm mode (`--serve`): one server per arm, every `--query` called once per block.
    pub serve: Option<String>,
    pub queries: Vec<String>,
    pub load_deadline: Duration,
    pub keep_products: bool,
    pub query: String,
    pub blocks: u64,
    pub seed: u64,
    pub out: PathBuf,
    pub deadline: Duration,
    pub period: Duration,
    pub memory_bytes: u64,
    pub size_bytes: u64,
    pub identities: Vec<PathBuf>,
    // Warm mode: block b's first call starts no earlier than every server's ready line +
    // (b − 1) × spacing (contract sb § Carried rulings, R pair offsets).
    pub spacing: Duration,
    // Sparse replay: the full plan's draws, then only these blocks (their run numbers kept).
    pub only: Option<BTreeSet<u64>>,
}
pub(super) const HELP: &str = "usage: ckc bench run <composition-dir> (--producer <cmd> --checker <cmd> | --orchestrator <cmd> | --serve <cmd> [--load-deadline-ms <n>] [--keep-products yes|no]) --query <q> (repeatable with --serve) --blocks <n> --out <dir> [--root <repo>] [--paired <composition-dir>] [--seed <n>] [--deadline-ms <n>] [--sample-ms <n>] [--memory-bytes <n>] [--size-bytes <n>] [--identity-file <file>] [--block-spacing-ms <n>] [--only-blocks <b>,…]";

fn natural(value: &str, option: &str) -> Result<u64> {
    value
        .parse::<u64>()
        .map_err(|_| format!("{option}: expected a natural number"))
}
fn positive(value: &str, option: &str) -> Result<u64> {
    let value = natural(value, option)?;
    if value == 0 {
        Err(format!("{option}: expected a positive number"))
    } else {
        Ok(value)
    }
}
fn directory(path: &Path) -> Result<PathBuf> {
    let path = path
        .canonicalize()
        .map_err(|e| format!("{}: {e}", path.display()))?;
    if !path.is_dir() {
        return Err(format!("{}: expected a directory", path.display()));
    }
    Ok(path)
}
impl Config {
    pub fn parse(args: &[String]) -> Result<Self> {
        let target = args
            .first()
            .filter(|s| !s.starts_with('-'))
            .ok_or_else(|| HELP.to_owned())?;
        if !(args.len() - 1).is_multiple_of(2) {
            return Err(HELP.to_owned());
        }
        let mut config = Self {
            root: env::current_dir().map_err(|e| e.to_string())?,
            target: target.into(),
            paired: None,
            producer: String::new(),
            checker: String::new(),
            orchestrator: None,
            serve: None,
            queries: Vec::new(),
            load_deadline: Duration::ZERO,
            keep_products: false,
            query: String::new(),
            blocks: 1,
            seed: 0x5342_315f_5041_4952,
            out: PathBuf::new(),
            deadline: Duration::from_secs(300),
            period: Duration::from_millis(100),
            memory_bytes: 32_000_000_000,
            size_bytes: 8_000_000_000,
            identities: Vec::new(),
            spacing: Duration::ZERO,
            only: None,
        };
        let mut seen = BTreeSet::new();
        for pair in args[1..].as_chunks::<2>().0 {
            let (option, value) = (pair[0].as_str(), pair[1].as_str());
            if !matches!(option, "--identity-file" | "--query") && !seen.insert(option) {
                return Err(format!("duplicate option {option}"));
            }
            if value.contains('\0') {
                return Err(format!("{option}: invalid value"));
            }
            match option {
                "--root" => config.root = value.into(),
                "--paired" => config.paired = Some(value.into()),
                "--producer" => config.producer = value.to_owned(),
                "--checker" => config.checker = value.to_owned(),
                "--orchestrator" => config.orchestrator = Some(value.to_owned()),
                "--query" => {
                    seen.insert(option);
                    config.queries.push(value.to_owned())
                }
                "--serve" => config.serve = Some(value.to_owned()),
                "--load-deadline-ms" => {
                    config.load_deadline = Duration::from_millis(positive(value, option)?)
                }
                "--keep-products" => {
                    config.keep_products = match value {
                        "yes" => true,
                        "no" => false,
                        _ => return Err(format!("{option}: expected yes|no")),
                    }
                }
                "--blocks" => config.blocks = positive(value, option)?,
                "--seed" => config.seed = natural(value, option)?,
                "--out" => config.out = value.into(),
                "--deadline-ms" => {
                    config.deadline = Duration::from_millis(positive(value, option)?)
                }
                "--sample-ms" => config.period = Duration::from_millis(positive(value, option)?),
                "--memory-bytes" => config.memory_bytes = positive(value, option)?,
                "--size-bytes" => config.size_bytes = positive(value, option)?,
                "--identity-file" => config.identities.push(value.into()),
                "--block-spacing-ms" => {
                    config.spacing = Duration::from_millis(natural(value, option)?)
                }
                "--only-blocks" => {
                    let mut blocks = BTreeSet::new();
                    for item in value.split(',') {
                        if !blocks.insert(positive(item, option)?) {
                            return Err(format!("{option}: block {item} listed twice"));
                        }
                    }
                    config.only = Some(blocks);
                }
                _ => return Err(format!("unknown option {option}; {HELP}")),
            }
        }
        let commands_valid = match (&config.orchestrator, &config.serve) {
            (Some(command), None) | (None, Some(command)) => {
                !command.trim().is_empty()
                    && config.producer.is_empty()
                    && config.checker.is_empty()
            }
            (None, None) => !config.producer.trim().is_empty() && !config.checker.trim().is_empty(),
            (Some(_), Some(_)) => false,
        };
        let warm = config.serve.is_some();
        if (!warm
            && [
                "--load-deadline-ms",
                "--keep-products",
                "--block-spacing-ms",
            ]
            .iter()
            .any(|o| seen.contains(o)))
            || config
                .only
                .as_ref()
                .is_some_and(|only| only.last().is_some_and(|&b| b > config.blocks))
            || (!warm && config.queries.len() != 1)
            || config.queries.is_empty()
            || config.queries.iter().any(String::is_empty)
        {
            return Err(HELP.to_owned());
        }
        config.query = config.queries[0].clone();
        if config.load_deadline.is_zero() {
            config.load_deadline = config.deadline;
        }
        if !commands_valid
            || config.query.is_empty()
            || config.out.as_os_str().is_empty()
            || !seen.contains("--blocks")
        {
            return Err(HELP.to_owned());
        }
        config.root = directory(&config.root)?;
        config.target = directory(&config.target)?;
        config.paired = config.paired.as_deref().map(directory).transpose()?;
        if let Some(parent) = config.out.parent() {
            let parent = if parent.as_os_str().is_empty() {
                Path::new(".")
            } else {
                parent
            };
            config.out = directory(parent)?.join(config.out.file_name().ok_or("invalid --out")?);
        }
        for path in &mut config.identities {
            *path = path
                .canonicalize()
                .map_err(|e| format!("{}: {e}", path.display()))?;
        }
        for path in [&config.root, &config.target, &config.out]
            .into_iter()
            .chain(config.paired.iter())
        {
            if path.to_string_lossy().contains(['\t', '\r', '\n']) {
                return Err("paths must contain no tabs or newlines".to_owned());
            }
        }
        Ok(config)
    }
    pub fn named_query(&self, name: &str, composition: &Path) -> Result<PathBuf> {
        let path = match name {
            "q-all" if composition.join("queries/q-all.txt").is_file() => {
                composition.join("queries/q-all.txt")
            }
            "q-all" | "q-one" | "q-patient" => {
                composition.join("queries").join(format!("{name}.pl"))
            }
            _ => {
                let path = PathBuf::from(name);
                if path.is_absolute() {
                    path
                } else {
                    self.root.join(path)
                }
            }
        };
        path.canonicalize()
            .map_err(|e| format!("{}: {e}", path.display()))
    }
}
