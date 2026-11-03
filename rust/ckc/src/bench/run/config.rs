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
    pub query: String,
    pub blocks: u64,
    pub seed: u64,
    pub out: PathBuf,
    pub deadline: Duration,
    pub period: Duration,
    pub memory_bytes: u64,
    pub size_bytes: u64,
    pub identities: Vec<PathBuf>,
}
pub(super) const HELP: &str = "usage: ckc bench run <composition-dir> (--producer <cmd> --checker <cmd> | --orchestrator <cmd>) --query <q> --blocks <n> --out <dir> [--root <repo>] [--paired <composition-dir>] [--seed <n>] [--deadline-ms <n>] [--sample-ms <n>] [--memory-bytes <n>] [--size-bytes <n>] [--identity-file <file>]";

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
            query: String::new(),
            blocks: 1,
            seed: 0x5342_315f_5041_4952,
            out: PathBuf::new(),
            deadline: Duration::from_secs(300),
            period: Duration::from_millis(100),
            memory_bytes: 32_000_000_000,
            size_bytes: 8_000_000_000,
            identities: Vec::new(),
        };
        let mut seen = BTreeSet::new();
        for pair in args[1..].as_chunks::<2>().0 {
            let (option, value) = (pair[0].as_str(), pair[1].as_str());
            if option != "--identity-file" && !seen.insert(option) {
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
                "--query" => config.query = value.to_owned(),
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
                _ => return Err(format!("unknown option {option}; {HELP}")),
            }
        }
        let commands_valid = if let Some(command) = &config.orchestrator {
            !command.trim().is_empty() && config.producer.is_empty() && config.checker.is_empty()
        } else {
            !config.producer.trim().is_empty() && !config.checker.trim().is_empty()
        };
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
    pub fn query(&self, composition: &Path) -> Result<PathBuf> {
        let path = match self.query.as_str() {
            "q-all" if composition.join("queries/q-all.txt").is_file() => {
                composition.join("queries/q-all.txt")
            }
            "q-all" | "q-one" | "q-patient" => composition
                .join("queries")
                .join(format!("{}.pl", self.query)),
            _ => {
                let path = PathBuf::from(&self.query);
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

pub(super) struct Random(u64);
impl Random {
    pub fn new(seed: u64) -> Self {
        Self(seed)
    }
    pub fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut value = self.0;
        value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        value ^ (value >> 31)
    }
    pub fn below(&mut self, bound: usize) -> usize {
        let bound = bound as u64;
        let threshold = bound.wrapping_neg() % bound;
        loop {
            let value = self.next();
            if value >= threshold {
                return (value % bound) as usize;
            }
        }
    }
}
