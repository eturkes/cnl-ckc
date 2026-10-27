// Toolchain + bump recipe checks (.agent/archive/contracts/n14.md): each case runs one justfile
// recipe through the real `just` inside a private fixture root, with the network and the toolchain
// mocked. A failed version probe must fail its recipe, `amend-manifest` must refuse a staged
// manifest edit, and `bump-tool` must leave the lock untouched on a bad digest. Each probe and
// guard case also runs a planted pre-fix form of its recipe, which must slip through, so every run
// shows the check able to fire. CKC_RECIPES_JUSTFILE replaces the repo justfile (red replay).
use sha2::Digest;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}

fn justfile() -> String {
    let path = std::env::var_os("CKC_RECIPES_JUSTFILE")
        .map(PathBuf::from)
        .unwrap_or_else(|| root().join("justfile"));
    fs::read_to_string(path).unwrap()
}

// The test's own `just`; recipes see the fixture PATH alone.
fn just() -> PathBuf {
    let path = std::env::var_os("PATH").unwrap_or_default();
    std::env::split_paths(&path)
        .map(|dir| dir.join("just"))
        .find(|p| p.is_file())
        .expect("recipes: no `just` on PATH (run through `just test`)")
}

fn hex(bytes: &[u8]) -> String {
    sha2::Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

fn base64(bytes: &[u8]) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    for chunk in bytes.chunks(3) {
        let n = chunk
            .iter()
            .enumerate()
            .fold(0u32, |n, (i, &b)| n | (u32::from(b) << (16 - 8 * i)));
        for i in 0..4 {
            out.push(if i <= chunk.len() {
                ALPHABET[((n >> (18 - 6 * i)) & 63) as usize] as char
            } else {
                '='
            });
        }
    }
    out
}

fn put(path: &Path, bytes: impl AsRef<[u8]>) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, bytes).unwrap();
}

fn put_exec(path: &Path, script: &str) {
    put(path, script);
    fs::set_permissions(path, fs::Permissions::from_mode(0o755)).unwrap();
}

// A version probe prints `line`; a failing one then exits 23, so only its status tells.
fn probe(path: &Path, line: &str, fails: bool) {
    let exit = if fails { "exit 23\n" } else { "" };
    put_exec(
        path,
        &format!("#!/usr/bin/env bash\nprintf '%s\\n' '{line}'\n{exit}"),
    );
}

// Replaces the single occurrence of each anchor; any other count fails the plant.
fn plant(text: &str, edits: &[(&str, &str)]) -> String {
    let mut text = text.to_owned();
    for (old, new) in edits {
        assert_eq!(text.matches(old).count(), 1, "plant anchor {old:?}");
        text = text.replacen(old, new, 1);
    }
    text
}

fn show(out: &Output) -> String {
    format!(
        "rc {:?}\nstdout: {}\nstderr: {}",
        out.status.code(),
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    )
}

fn tar(dir: &Path, members: &[&str]) -> Vec<u8> {
    let out = dir.join("archive.tar.gz");
    let status = Command::new("tar")
        .arg("-czf")
        .arg(&out)
        .arg("-C")
        .arg(dir)
        .args(members)
        .status()
        .unwrap();
    assert!(status.success(), "tar {members:?}");
    let bytes = fs::read(&out).unwrap();
    fs::remove_file(out).unwrap();
    bytes
}

// Mock curl: the last bare argument names the URL; routes.tsv maps it to a data file.
const CURL: &str = r#"#!/usr/bin/env bash
out= url=
while [ "$#" -gt 0 ]; do
    case "$1" in
        -o) out=$2; shift 2 ;;
        -A | -H | --retry | --max-time | --proto) shift 2 ;;
        -*) shift ;;
        *) url=$1; shift ;;
    esac
done
src=$(awk -F'\t' -v u="$url" '$1 == u {print $2; exit}' "$MOCK_DATA/routes.tsv")
[ -n "$src" ] || { echo "mock curl: no route for $url" >&2; exit 22; }
if [ -n "$out" ]; then cp "$MOCK_DATA/$src" "$out"; else cat "$MOCK_DATA/$src"; fi
"#;

// Mock nested `just`: logs its arguments.
const JUST: &str = r#"#!/usr/bin/env bash
echo "$*" >> "$MOCK_DATA/just.log"
case "$1" in tools | _relock) exit 0 ;; *) exit 96 ;; esac
"#;

struct Scratch(PathBuf);
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn scratch(name: &str) -> Scratch {
    // One flat entry per test: removing it leaves nothing behind under rust/target/recipes/.
    let dir = root().join(format!("rust/target/recipes/{}-{name}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    Scratch(dir)
}

struct Fixture {
    root: PathBuf,
    toolchain: PathBuf,
    mock: PathBuf,
    data: PathBuf,
}

impl Fixture {
    fn new(scratch: &Scratch, case: &str, justfile: &str) -> Fixture {
        let root = scratch.0.join(case);
        let fx = Fixture {
            toolchain: root.join("toolchain"),
            mock: root.join("mock"),
            data: root.join("data"),
            root,
        };
        put(&fx.root.join("justfile"), justfile);
        put(&fx.data.join("routes.tsv"), "");
        fs::create_dir_all(fx.root.join("tmp")).unwrap();
        put_exec(&fx.mock.join("curl"), CURL);
        put_exec(&fx.mock.join("just"), JUST);
        fx
    }

    fn route(&self, url: &str, body: impl AsRef<[u8]>) {
        let table = self.data.join("routes.tsv");
        let mut routes = fs::read_to_string(&table).unwrap();
        let name = format!("r{}", routes.lines().count());
        put(&self.data.join(&name), body);
        routes.push_str(&format!("{url}\t{name}\n"));
        put(&table, routes);
    }

    fn command(&self, program: &Path) -> Command {
        let mut c = Command::new(program);
        c.current_dir(&self.root)
            .env_clear()
            .env("PATH", format!("{}:/usr/bin:/bin", self.mock.display()))
            .env("HOME", &self.root)
            .env("TMPDIR", self.root.join("tmp"))
            .env("CKC_TOOLCHAIN", &self.toolchain)
            .env("CKC_BIN", self.mock.join("ckc"))
            .env("MOCK_DATA", &self.data)
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("GIT_AUTHOR_NAME", "fixture")
            .env("GIT_AUTHOR_EMAIL", "fixture@localhost")
            .env("GIT_COMMITTER_NAME", "fixture")
            .env("GIT_COMMITTER_EMAIL", "fixture@localhost");
        c
    }

    fn run(&self, recipe: &str, args: &[&str]) -> Output {
        self.command(&just())
            .arg("--justfile")
            .arg(self.root.join("justfile"))
            .arg("--working-directory")
            .arg(&self.root)
            .arg(recipe)
            .args(args)
            .output()
            .unwrap()
    }

    fn git(&self, args: &[&str]) -> String {
        let out = self.command(Path::new("git")).args(args).output().unwrap();
        assert!(out.status.success(), "git {args:?}: {}", show(&out));
        String::from_utf8(out.stdout).unwrap()
    }

    fn calls(&self) -> String {
        fs::read_to_string(self.data.join("just.log")).unwrap_or_default()
    }

    fn read(&self, rel: &str) -> String {
        fs::read_to_string(self.root.join(rel)).unwrap()
    }
}

// The `<name>:` recipe of `text`: its header line through its last indented line.
fn recipe<'a>(text: &'a str, name: &str) -> &'a str {
    let start = text
        .find(&format!("\n{name}:"))
        .unwrap_or_else(|| panic!("no {name} recipe"))
        + 1;
    let body = &text[start..];
    let end = body
        .match_indices('\n')
        .map(|(i, _)| i + 1)
        .find(|&i| !body[i..].starts_with(' '))
        .unwrap_or(body.len());
    &body[..end]
}

// A `key=value` word of the swipl recipe (`v=`, `sha=`).
fn swipl_field(text: &str, key: &str) -> String {
    recipe(text, "swipl")
        .split([' ', ';', '\n'])
        .find_map(|w| w.strip_prefix(key))
        .unwrap_or_else(|| panic!("swipl recipe holds no {key}"))
        .to_owned()
}

#[test]
fn swipl_version_probe_fails_the_recipe() {
    let s = scratch("swipl");
    let text = justfile();
    let (version, pin) = (swipl_field(&text, "v="), swipl_field(&text, "sha="));
    let line = format!("SWI-Prolog version {version} for x86_64-linux");
    // The cached-prefix path: the pin matches, so the recipe skips the build.
    let case = |name: &str, justfile: &str, fails: bool| {
        let fx = Fixture::new(&s, name, justfile);
        let prefix = fx.toolchain.join(format!("swipl-{version}"));
        put(&prefix.join(".pin"), format!("{pin}\n"));
        probe(&prefix.join("bin/swipl"), &line, fails);
        put_exec(
            &fx.mock.join("uv"),
            "#!/usr/bin/env bash\necho /usr/bin/true\n",
        );
        fx.run("swipl", &[])
    };
    let out = case("control", &text, false);
    assert_eq!(out.status.code(), Some(0), "control: {}", show(&out));
    assert_eq!(
        String::from_utf8_lossy(&out.stdout),
        format!("swipl: ok {line}\n")
    );
    let out = case("probe-fails", &text, true);
    assert_ne!(
        out.status.code(),
        Some(0),
        "a failed swipl --version passed: {}",
        show(&out)
    );
    let planted = plant(
        &text,
        &[(
            "    ver=$(\"$T/bin/swipl\" --version)\n    echo \"swipl: ok $ver\"\n",
            "    echo \"swipl: ok $(\"$T/bin/swipl\" --version)\"\n",
        )],
    );
    let out = case("planted", &planted, true);
    assert_eq!(
        out.status.code(),
        Some(0),
        "the planted pre-fix swipl recipe no longer slips through: {}",
        show(&out)
    );
    // A uv that supplies no CPython 3.15 (the CI runner has no uv): swipl installs, python3 stays unlinked.
    let fx = Fixture::new(&s, "no-python", &text);
    let prefix = fx.toolchain.join(format!("swipl-{version}"));
    put(&prefix.join(".pin"), format!("{pin}\n"));
    probe(&prefix.join("bin/swipl"), &line, false);
    put_exec(&fx.mock.join("uv"), "#!/usr/bin/env bash\nexit 2\n");
    let out = fx.run("swipl", &[]);
    assert_eq!(out.status.code(), Some(0), "no-python: {}", show(&out));
    assert_eq!(
        String::from_utf8_lossy(&out.stdout),
        format!("swipl: ok {line}\n")
    );
    assert!(
        fs::symlink_metadata(fx.toolchain.join("bin/python3")).is_err(),
        "a python3 shim without uv's CPython"
    );
}

#[test]
fn bump_verus_version_probes_fail_the_recipe() {
    let s = scratch("bump-verus");
    let text = justfile();
    let tag = "release/0.2026.08.30.b432e82";
    let api = "https://api.github.com/repos/verus-lang/verus";
    let (channel, vstd, commit) = ("1.96.0", "0.0.0-2026-08-30-0058", "b".repeat(40));
    let tc = format!("{channel}-x86_64-unknown-linux-gnu");
    let content = |text: String| format!(r#"{{"content": "{}"}}"#, base64(text.as_bytes()));
    let case = |name: &str, justfile: &str, failing: &str| {
        let fx = Fixture::new(&s, name, justfile);
        fx.route(
            &format!("{api}/releases/tags/release%2F0.2026.08.30.b432e82"),
            format!(
                r#"{{"prerelease": false, "assets": [{{"name": "verus-0.2026.08.30.b432e82-x86-linux.zip", "digest": "sha256:{}"}}]}}"#,
                "a".repeat(64)
            ),
        );
        fx.route(
            &format!("{api}/git/ref/tags/{tag}"),
            format!(r#"{{"object": {{"type": "commit", "sha": "{commit}"}}}}"#),
        );
        fx.route(
            &format!("{api}/contents/rust-toolchain.toml?ref={commit}"),
            content(format!("[toolchain]\nchannel = \"{channel}\"\n")),
        );
        fx.route(
            &format!("{api}/contents/source/vstd/Cargo.toml?ref={commit}"),
            content(format!(
                "[package]\nname = \"vstd\"\nversion = \"{vstd}\"\n"
            )),
        );
        let rows = "verus_release verus_asset verus_asset_sha256 verus_asset_url rustc_toolchain z3_version";
        put(
            &fx.root.join("rust/verus.lock"),
            rows.split(' ')
                .map(|k| format!("{k}\told\n"))
                .collect::<String>(),
        );
        put(
            &fx.root.join("rust/kani.lock"),
            "bootstrap_rustc_toolchain\told\nbootstrap_rustc_version\told\n",
        );
        put(
            &fx.root.join("rust/rust-toolchain.toml"),
            "channel = \"old\"\n",
        );
        put(&fx.root.join("rust/Cargo.toml"), "vstd = \"=old\"\n");
        probe(
            &fx.toolchain.join("verus-x86-linux/z3"),
            "Z3 version 4.15.2 - 64 bit",
            failing == "z3",
        );
        probe(
            &fx.toolchain
                .join(format!("rustup/toolchains/{tc}/bin/rustc")),
            "rustc 1.96.0 (fixture 2026-08-30)",
            failing == "rustc",
        );
        let out = fx.run("bump-verus", &[tag]);
        (out, fx)
    };
    let (out, fx) = case("control", &text, "");
    assert_eq!(out.status.code(), Some(0), "control: {}", show(&out));
    assert!(fx.read("rust/verus.lock").contains("z3_version\t4.15.2\n"));
    assert!(
        fx.read("rust/kani.lock")
            .contains("bootstrap_rustc_version\trustc 1.96.0 (fixture 2026-08-30)\n")
    );
    assert_eq!(fx.calls(), format!("tools\n_relock vstd {vstd}\n"));
    let z3 = r#""{{ TOOLCHAIN }}/verus-x86-linux/z3" --version | awk '{print $3}'"#;
    let rustc = r#""$RUSTUP_HOME/toolchains/$tc/bin/rustc" --version"#;
    let plants = [
        (
            "z3",
            [
                (format!("    z3=$({z3})\n"), String::new()),
                (
                    "set_row rust/verus.lock z3_version \"$z3\"".to_owned(),
                    format!("set_row rust/verus.lock z3_version \"$({z3})\""),
                ),
            ],
        ),
        (
            "rustc",
            [
                (format!("    brv=$({rustc})\n"), String::new()),
                (
                    "set_row rust/kani.lock bootstrap_rustc_version \"$brv\"".to_owned(),
                    format!("set_row rust/kani.lock bootstrap_rustc_version \"$({rustc})\""),
                ),
            ],
        ),
    ];
    for (failing, edits) in &plants {
        let (out, fx) = case(&format!("{failing}-fails"), &text, failing);
        assert_ne!(
            out.status.code(),
            Some(0),
            "a failed {failing} --version passed: {}",
            show(&out)
        );
        assert!(!fx.calls().contains("_relock"), "{failing}: relocked");
        let edits: Vec<(&str, &str)> = edits
            .iter()
            .map(|(a, b)| (a.as_str(), b.as_str()))
            .collect();
        let (out, fx) = case(
            &format!("{failing}-planted"),
            &plant(&text, &edits),
            failing,
        );
        assert_eq!(
            out.status.code(),
            Some(0),
            "the planted pre-fix {failing} probe no longer slips through: {}",
            show(&out)
        );
        assert!(fx.calls().contains("_relock"));
    }
}

const KANI_LOCK: &str = "# fixture\nkani_version\told\n";

// One bump-kani run against a fixture release: its installer crate, bundle (CBMC + Kissat
// probes), nightly channel manifest and rustc.
fn kani_case(
    s: &Scratch,
    name: &str,
    justfile: &str,
    failing: &str,
    bad_digest: bool,
) -> (Output, Fixture) {
    let fx = Fixture::new(s, name, justfile);
    let (v, host, tc) = (
        "0.68.0",
        "x86_64-unknown-linux-gnu",
        "1.96.0-x86_64-unknown-linux-gnu",
    );
    let src = fx.root.join("src");
    put(
        &src.join(format!("kani-verifier-{v}/rust-toolchain.toml")),
        "[toolchain]\nchannel = \"nightly-2026-08-25\"\n",
    );
    put(
        &src.join(format!("kani-verifier-{v}/Cargo.lock")),
        format!(
            "version = 4\n\n[[package]]\nname = \"fixture-dep\"\nversion = \"1.0.0\"\nsource = \"registry+https://github.com/rust-lang/crates.io-index\"\nchecksum = \"{}\"\n",
            "c".repeat(64)
        ),
    );
    let installer = tar(&src, &[&format!("kani-verifier-{v}")]);
    probe(
        &src.join(format!("kani-{v}/bin/cbmc")),
        "6.11.0 (fixture)",
        failing == "cbmc",
    );
    probe(
        &src.join(format!("kani-{v}/bin/kissat")),
        "4.0.3",
        failing == "kissat",
    );
    let bundle = tar(
        &src,
        &[
            &format!("kani-{v}/bin/cbmc"),
            &format!("kani-{v}/bin/kissat"),
        ],
    );
    let mut manifest =
        String::from("[pkg.rustc]\nversion = \"1.96.0-nightly (fixture 2026-08-25)\"\n");
    for c in ["rustc", "rust-std", "cargo"] {
        manifest.push_str(&format!(
            "\n[pkg.{c}.target.{host}]\nxz_url = \"https://static.rust-lang.org/dist/2026-08-25/{c}-nightly-{host}.tar.xz\"\nxz_hash = \"{}\"\n",
            "d".repeat(64)
        ));
    }
    let api = "https://api.github.com/repos/model-checking/kani";
    let bundle_name = format!("kani-{v}-{host}.tar.gz");
    let bundle_url =
        format!("https://github.com/model-checking/kani/releases/download/kani-{v}/{bundle_name}");
    let manifest_url = "https://static.rust-lang.org/dist/2026-08-25/channel-rust-nightly.toml";
    let checksum = if bad_digest {
        "0".repeat(64)
    } else {
        hex(&installer)
    };
    fx.route(
        &format!("{api}/git/ref/tags/kani-{v}"),
        format!(
            r#"{{"object": {{"type": "commit", "sha": "{}"}}}}"#,
            "e".repeat(40)
        ),
    );
    fx.route(
        &format!("{api}/releases/tags/kani-{v}"),
        format!(
            r#"{{"assets": [{{"name": "{bundle_name}", "browser_download_url": "{bundle_url}", "digest": "sha256:{}"}}]}}"#,
            hex(&bundle)
        ),
    );
    fx.route(
        &format!("https://crates.io/api/v1/crates/kani-verifier/{v}"),
        format!(r#"{{"version": {{"checksum": "{checksum}"}}}}"#),
    );
    fx.route(
        &format!("https://static.crates.io/crates/kani-verifier/kani-verifier-{v}.crate"),
        &installer,
    );
    fx.route(
        &format!("{manifest_url}.sha256"),
        format!("{}  channel-rust-nightly.toml\n", hex(manifest.as_bytes())),
    );
    fx.route(manifest_url, &manifest);
    fx.route(&bundle_url, &bundle);
    put(
        &fx.root.join("rust/verus.lock"),
        format!("rustc_toolchain\t{tc}\n"),
    );
    put(&fx.root.join("rust/kani.lock"), KANI_LOCK);
    probe(
        &fx.toolchain
            .join(format!("rustup/toolchains/{tc}/bin/rustc")),
        "rustc 1.96.0 (fixture 2026-08-25)",
        failing == "rustc",
    );
    let out = fx.run("bump-kani", &[v]);
    (out, fx)
}

#[test]
fn bump_kani_version_probes_fail_the_recipe() {
    let s = scratch("bump-kani");
    let text = justfile();
    let (out, fx) = kani_case(&s, "control", &text, "", false);
    assert_eq!(out.status.code(), Some(0), "control: {}", show(&out));
    let lock = fx.read("rust/kani.lock");
    for row in [
        "# fixture\n".to_owned(),
        format!("kani_commit\t{}\n", "e".repeat(40)),
        "bootstrap_rustc_version\trustc 1.96.0 (fixture 2026-08-25)\n".to_owned(),
        "cbmc_version\t6.11.0\n".to_owned(),
        "kissat_version\t4.0.3\n".to_owned(),
        format!("vendor_crate\tfixture-dep\t1.0.0\t{}\n", "c".repeat(64)),
    ] {
        assert!(lock.contains(&row), "control lock lacks {row:?}:\n{lock}");
    }
    let (out, fx) = kani_case(&s, "bad-digest", &text, "", true);
    assert_ne!(out.status.code(), Some(0), "bad digest: {}", show(&out));
    assert_eq!(fx.read("rust/kani.lock"), KANI_LOCK);
    let cbmc = r#""$w/kani-$v/bin/cbmc" --version | awk '{print $1}'"#;
    let kissat = r#""$w/kani-$v/bin/kissat" --version"#;
    let rustc = r#""$RUSTUP_HOME/toolchains/$tc/bin/rustc" --version"#;
    let plants = [
        (
            "cbmc",
            vec![
                (format!("    cbmc=$({cbmc})\n"), String::new()),
                (
                    "for x in cbmc kissat rustc_v tc; do".to_owned(),
                    "for x in kissat rustc_v tc; do".to_owned(),
                ),
                (
                    "cbmc_version \"$cbmc\"".to_owned(),
                    format!("cbmc_version \"$({cbmc})\""),
                ),
            ],
        ),
        (
            "kissat",
            vec![
                (format!("    kissat=$({kissat})\n"), String::new()),
                (
                    "for x in cbmc kissat rustc_v tc; do".to_owned(),
                    "for x in cbmc rustc_v tc; do".to_owned(),
                ),
                (
                    "kissat_version \"$kissat\"".to_owned(),
                    format!("kissat_version \"$({kissat})\""),
                ),
            ],
        ),
        (
            "rustc",
            vec![
                (
                    format!("    brv=$({rustc}) || fail \"rustc $tc missing: run just tools\"\n"),
                    String::new(),
                ),
                (
                    "bootstrap_rustc_version \"$brv\" \\".to_owned(),
                    format!("bootstrap_rustc_version \"$({rustc})\" \\"),
                ),
            ],
        ),
    ];
    for (failing, edits) in &plants {
        let (out, fx) = kani_case(&s, &format!("{failing}-fails"), &text, failing, false);
        assert_ne!(
            out.status.code(),
            Some(0),
            "a failed {failing} --version passed: {}",
            show(&out)
        );
        assert_eq!(
            fx.read("rust/kani.lock"),
            KANI_LOCK,
            "{failing}: lock replaced"
        );
        let edits: Vec<(&str, &str)> = edits
            .iter()
            .map(|(a, b)| (a.as_str(), b.as_str()))
            .collect();
        let (out, fx) = kani_case(
            &s,
            &format!("{failing}-planted"),
            &plant(&text, &edits),
            failing,
            false,
        );
        assert_eq!(
            out.status.code(),
            Some(0),
            "the planted pre-fix {failing} probe no longer slips through: {}",
            show(&out)
        );
        assert_ne!(fx.read("rust/kani.lock"), KANI_LOCK);
    }
}

// A git fixture repo whose `ckc release-manifest` (CKC_BIN) digests NOTICE.
fn manifest_repo(s: &Scratch, name: &str, justfile: &str) -> Fixture {
    let fx = Fixture::new(s, name, justfile);
    put(&fx.root.join("NOTICE"), "notice\n");
    put(&fx.root.join("README.md"), "readme\n");
    put(
        &fx.root.join("release-manifest.tsv"),
        format!("{}  NOTICE\n", hex(b"notice\n")),
    );
    put(
        &fx.root.join(".gitignore"),
        "/toolchain/\n/mock/\n/data/\n/tmp/\n",
    );
    put_exec(
        &fx.mock.join("ckc"),
        "#!/usr/bin/env bash\n[ \"$1\" = release-manifest ] || exit 64\nsha256sum NOTICE > release-manifest.tsv\n",
    );
    fx.git(&["init", "-q"]);
    fx.git(&["add", "-A"]);
    fx.git(&["commit", "-q", "-m", "base"]);
    fx
}

// A staged manifest edit whose working file equals HEAD → (staged blob, HEAD).
fn stage_manifest_edit(fx: &Fixture) -> (String, String) {
    let path = fx.root.join("release-manifest.tsv");
    let pristine = fs::read(&path).unwrap();
    put(&path, [pristine.as_slice(), b"staged edit\n"].concat());
    fx.git(&["add", "--", "release-manifest.tsv"]);
    put(&path, &pristine);
    (
        fx.git(&["rev-parse", ":release-manifest.tsv"]),
        fx.git(&["rev-parse", "HEAD"]),
    )
}

#[test]
fn amend_manifest_refuses_a_staged_manifest_edit() {
    let s = scratch("amend-manifest");
    let text = justfile();

    let fx = manifest_repo(&s, "current", &text);
    let head = fx.git(&["rev-parse", "HEAD"]);
    let out = fx.run("amend-manifest", &[]);
    assert_eq!(out.status.code(), Some(0), "current: {}", show(&out));
    assert!(String::from_utf8_lossy(&out.stdout).contains("amend-manifest: current"));
    assert_eq!(fx.git(&["rev-parse", "HEAD"]), head);

    let fx = manifest_repo(&s, "stale", &text);
    let parent = fx.git(&["rev-parse", "HEAD"]);
    put(&fx.root.join("NOTICE"), "notice\nedited\n");
    fx.git(&["commit", "-q", "-am", "stale"]);
    let stale = fx.git(&["rev-parse", "HEAD"]);
    put(&fx.root.join("README.md"), "readme\nstaged\n");
    fx.git(&["add", "--", "README.md"]);
    let readme = fx.git(&["rev-parse", ":README.md"]);
    let out = fx.run("amend-manifest", &[]);
    assert_eq!(out.status.code(), Some(0), "stale: {}", show(&out));
    assert!(String::from_utf8_lossy(&out.stdout).contains("amend-manifest: amended"));
    assert_eq!(fx.git(&["rev-parse", "HEAD^"]), parent);
    assert_eq!(
        fx.git(&["diff", "--name-only", stale.trim(), "HEAD"]),
        "release-manifest.tsv\n"
    );
    assert_eq!(fx.git(&["rev-parse", ":README.md"]), readme);
    let out = fx.run("amend-manifest", &[]);
    assert!(String::from_utf8_lossy(&out.stdout).contains("amend-manifest: current"));

    let fx = manifest_repo(&s, "dirty", &text);
    let head = fx.git(&["rev-parse", "HEAD"]);
    let path = fx.root.join("release-manifest.tsv");
    put(
        &path,
        [fs::read(&path).unwrap(), b"dirty\n".to_vec()].concat(),
    );
    let dirty = fs::read(&path).unwrap();
    let out = fx.run("amend-manifest", &[]);
    assert_eq!(out.status.code(), Some(2), "dirty: {}", show(&out));
    assert_eq!(fs::read(&path).unwrap(), dirty);
    assert_eq!(fx.git(&["rev-parse", "HEAD"]), head);

    let fx = manifest_repo(&s, "staged-only", &text);
    let before = stage_manifest_edit(&fx);
    let out = fx.run("amend-manifest", &[]);
    assert_eq!(out.status.code(), Some(2), "staged-only: {}", show(&out));
    let after = (
        fx.git(&["rev-parse", ":release-manifest.tsv"]),
        fx.git(&["rev-parse", "HEAD"]),
    );
    assert_eq!(after, before, "staged-only: index or HEAD moved");

    let planted = plant(
        &text,
        &[(
            "{ git diff --quiet HEAD -- release-manifest.tsv && git diff --quiet --cached -- release-manifest.tsv; }",
            "git diff --quiet HEAD -- release-manifest.tsv",
        )],
    );
    let fx = manifest_repo(&s, "staged-only-planted", &planted);
    let before = stage_manifest_edit(&fx);
    let out = fx.run("amend-manifest", &[]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "the planted pre-fix guard no longer slips through: {}",
        show(&out)
    );
    let after = (
        fx.git(&["rev-parse", ":release-manifest.tsv"]),
        fx.git(&["rev-parse", "HEAD"]),
    );
    assert_ne!(after, before, "planted guard: nothing replaced");
}

const TOOLS_LOCK: &str = "tool\tversion\tasset\tsha256\turl
rustup-init\t1.29.1\trustup-init\t1111111111111111111111111111111111111111111111111111111111111111\thttps://static.rust-lang.org/rustup/archive/1.29.1/x86_64-unknown-linux-gnu/rustup-init
just\t1.58.0\tjust-1.58.0-x86_64-unknown-linux-musl.tar.gz\t3333333333333333333333333333333333333333333333333333333333333333\thttps://github.com/casey/just/releases/download/1.58.0/just-1.58.0-x86_64-unknown-linux-musl.tar.gz
zizmor\t1.30.1\tzizmor-x86_64-unknown-linux-gnu.tar.gz\t2222222222222222222222222222222222222222222222222222222222222222\thttps://github.com/zizmorcore/zizmor/releases/download/v1.30.1/zizmor-x86_64-unknown-linux-gnu.tar.gz
node\t24.22.0\tnode-v24.22.0-linux-x64.tar.xz\t4444444444444444444444444444444444444444444444444444444444444444\thttps://nodejs.org/dist/v24.22.0/node-v24.22.0-linux-x64.tar.xz
chrome-headless-shell\t155.0.8059.84\tchrome-headless-shell-linux64.zip\t5555555555555555555555555555555555555555555555555555555555555555\thttps://storage.googleapis.com/chrome-for-testing-public/155.0.8059.84/linux64/chrome-headless-shell-linux64.zip
";

// Where a bump control's digest comes from: the GitHub release API, rustup's `.sha256`,
// nodejs.org's SHASUMS256.txt, or the GCS object metadata of a Chrome for Testing asset (an md5
// that checks the download; the row pins the download's sha256).
enum Source {
    GitHub(&'static str),
    Rustup,
    Node,
    Gcs(&'static str),
}

// md5 of `chrome-headless-shell 156.0.7000.1 fixture asset\n`, base64 as GCS serves it.
const CFT_MD5: &str = "lwEUb9P188C1E6v8PPfR0g==";
const CFT_META: &str = "https://storage.googleapis.com/storage/v1/b/chrome-for-testing-public/o/156.0.7000.1%2Flinux64%2Fchrome-headless-shell-linux64.zip";

#[test]
fn bump_tool_writes_rows_from_release_data_and_refuses_bad_digests() {
    let s = scratch("bump-tool");
    let text = justfile();
    let fixture = |name: &str| {
        let fx = Fixture::new(&s, name, &text);
        put(&fx.root.join("rust/tools.lock"), TOOLS_LOCK);
        fx
    };
    // GitHub release JSON for one asset; `digest` = None drops the field.
    let release = |asset: &str, url: &str, digest: Option<&str>| {
        let digest = digest.map_or(String::new(), |d| format!(r#", "digest": "sha256:{d}""#));
        format!(r#"{{"assets": [{{"name": "{asset}", "browser_download_url": "{url}"{digest}}}]}}"#)
    };
    // Each control rewrites its row alone and installs through `just tools`.
    let controls = [
        (
            "zizmor",
            "1.31.0",
            Source::GitHub("https://api.github.com/repos/zizmorcore/zizmor/releases/tags/v1.31.0"),
            "zizmor-x86_64-unknown-linux-gnu.tar.gz",
            "https://github.com/zizmorcore/zizmor/releases/download/v1.31.0/zizmor-x86_64-unknown-linux-gnu.tar.gz",
        ),
        (
            "just",
            "1.59.0",
            Source::GitHub("https://api.github.com/repos/casey/just/releases/tags/1.59.0"),
            "just-1.59.0-x86_64-unknown-linux-musl.tar.gz",
            "https://github.com/casey/just/releases/download/1.59.0/just-1.59.0-x86_64-unknown-linux-musl.tar.gz",
        ),
        (
            "rustup-init",
            "1.29.2",
            Source::Rustup,
            "rustup-init",
            "https://static.rust-lang.org/rustup/archive/1.29.2/x86_64-unknown-linux-gnu/rustup-init",
        ),
        (
            "node",
            "24.23.0",
            Source::Node,
            "node-v24.23.0-linux-x64.tar.xz",
            "https://nodejs.org/dist/v24.23.0/node-v24.23.0-linux-x64.tar.xz",
        ),
        (
            "chrome-headless-shell",
            "156.0.7000.1",
            Source::Gcs(CFT_MD5),
            "chrome-headless-shell-linux64.zip",
            "https://storage.googleapis.com/chrome-for-testing-public/156.0.7000.1/linux64/chrome-headless-shell-linux64.zip",
        ),
    ];
    for (tool, version, source, asset, url) in controls {
        let fx = fixture(tool);
        let bytes = format!("{tool} {version} fixture asset\n");
        let digest = hex(bytes.as_bytes());
        match source {
            Source::GitHub(api) => fx.route(api, release(asset, url, Some(&digest))),
            Source::Rustup => fx.route(&format!("{url}.sha256"), format!("{digest} *./{asset}\n")),
            Source::Node => fx.route(
                "https://nodejs.org/dist/v24.23.0/SHASUMS256.txt",
                format!("{}  {asset}.sig\n{digest}  {asset}\n", "6".repeat(64)),
            ),
            Source::Gcs(md5) => fx.route(CFT_META, format!(r#"{{"md5Hash": "{md5}"}}"#)),
        }
        fx.route(url, &bytes);
        let out = fx.run("bump-tool", &[tool, version]);
        assert_eq!(out.status.code(), Some(0), "{tool}: {}", show(&out));
        let expected: String = TOOLS_LOCK
            .lines()
            .map(|row| {
                if row.split('\t').next() == Some(tool) {
                    format!("{tool}\t{version}\t{asset}\t{digest}\t{url}\n")
                } else {
                    format!("{row}\n")
                }
            })
            .collect();
        assert_eq!(fx.read("rust/tools.lock"), expected, "{tool} row");
        assert_eq!(fx.calls(), "tools\n", "{tool}: install");
        assert_eq!(
            fs::read(fx.toolchain.join("dl").join(asset)).unwrap(),
            bytes.as_bytes()
        );
    }
    // Digests that disagree with the download (GitHub, then GCS md5) and sources without one
    // (a GitHub release, a SHASUMS256.txt missing the asset): refused, untouched.
    let cft = "https://storage.googleapis.com/chrome-for-testing-public/156.0.7000.1/linux64/chrome-headless-shell-linux64.zip";
    for (name, md5) in [
        ("md5-mismatch", "euS8epvTd5yXpe+ELbSYlg=="),
        ("md5-missing", ""),
    ] {
        let fx = fixture(name);
        fx.route(CFT_META, format!(r#"{{"md5Hash": "{md5}"}}"#));
        fx.route(cft, "chrome-headless-shell 156.0.7000.1 fixture asset\n");
        let out = fx.run("bump-tool", &["chrome-headless-shell", "156.0.7000.1"]);
        assert_ne!(out.status.code(), Some(0), "{name}: {}", show(&out));
        assert_eq!(
            fx.read("rust/tools.lock"),
            TOOLS_LOCK,
            "{name}: lock changed"
        );
        assert_eq!(fx.calls(), "", "{name}: installed");
    }
    let fx = fixture("shasums-missing");
    let node = "https://nodejs.org/dist/v24.23.0/node-v24.23.0-linux-x64.tar.xz";
    fx.route(
        "https://nodejs.org/dist/v24.23.0/SHASUMS256.txt",
        format!("{}  node-v24.23.0-darwin-arm64.tar.gz\n", "6".repeat(64)),
    );
    fx.route(node, "node 24.23.0 fixture asset\n");
    let out = fx.run("bump-tool", &["node", "24.23.0"]);
    assert_ne!(
        out.status.code(),
        Some(0),
        "shasums-missing: {}",
        show(&out)
    );
    assert_eq!(
        fx.read("rust/tools.lock"),
        TOOLS_LOCK,
        "shasums-missing: lock changed"
    );
    assert_eq!(fx.calls(), "", "shasums-missing: installed");
    let api = "https://api.github.com/repos/zizmorcore/zizmor/releases/tags/v1.31.0";
    let asset = "zizmor-x86_64-unknown-linux-gnu.tar.gz";
    let url = "https://github.com/zizmorcore/zizmor/releases/download/v1.31.0/zizmor-x86_64-unknown-linux-gnu.tar.gz";
    let wrong = hex(b"another asset\n");
    for (name, digest) in [
        ("digest-mismatch", Some(wrong.as_str())),
        ("digest-missing", None),
    ] {
        let fx = fixture(name);
        fx.route(api, release(asset, url, digest));
        fx.route(url, "zizmor 1.31.0 fixture asset\n");
        let out = fx.run("bump-tool", &["zizmor", "1.31.0"]);
        assert_ne!(out.status.code(), Some(0), "{name}: {}", show(&out));
        assert_eq!(
            fx.read("rust/tools.lock"),
            TOOLS_LOCK,
            "{name}: lock changed"
        );
        assert_eq!(fx.calls(), "", "{name}: installed");
    }
}

// Mock cargo: logs its arguments; `metadata` + `update --dry-run` answer from fixture files per
// workspace (main | harness), `metadata-fails` makes metadata exit 101.
const CARGO: &str = r#"#!/usr/bin/env bash
echo "$*" >> "$MOCK_DATA/cargo.log"
ws=main
case "$*" in *ckc-kani-harness/Cargo.toml*) ws=harness ;; esac
case "$*" in
    *metadata*) [ ! -e "$MOCK_DATA/metadata-fails" ] || exit 101; cat "$MOCK_DATA/metadata-$ws.json" ;;
    *"update --dry-run"*) cat "$MOCK_DATA/update-$ws.txt" >&2 ;;
esac
"#;

const REGISTRY: &str = "registry+https://github.com/rust-lang/crates.io-index";

// The CI container digest of every `outdated` fixture.
const IMAGE: &str = "6666666666666666666666666666666666666666666666666666666666666666";

// Upstream SWI-Prolog for one `outdated` run: swi-prolog.org's latest stable + Docker Hub's
// digest of that version's tag (None = no such tag).
struct Swipl<'a> {
    latest: &'a str,
    digest: Option<&'a str>,
}

// One `outdated` fixture tree: member ckc pins sha2 `=0.11.0` (held by ckc alone), requires
// zlib-x `^1.0` (a newer major exists), and reaches syn through ext-macro; both CI container
// jobs pin the swipl recipe's version at IMAGE; node pins the Current line's 26.1.0.
fn outdated_fixture(
    s: &Scratch,
    name: &str,
    justfile: &str,
    metadata_fails: bool,
    swipl: Swipl,
) -> Fixture {
    let fx = Fixture::new(s, name, justfile);
    let image = format!(
        "      image: docker.io/library/swipl:{}@sha256:{IMAGE}\n",
        swipl_field(justfile, "v=")
    );
    put(&fx.root.join(".github/workflows/ci.yml"), image.repeat(2));
    fx.route(
        "https://www.swi-prolog.org/download/stable/src/swipl-latest.tar.gz",
        format!(
            "HTTP/2 303 \r\nlocation: /download/stable/src/swipl-{}.tar.gz\r\n\r\nHTTP/2 200 \r\n\r\n",
            swipl.latest
        ),
    );
    if let Some(digest) = swipl.digest {
        fx.route(
            &format!(
                "https://hub.docker.com/v2/repositories/library/swipl/tags/{}",
                swipl.latest
            ),
            format!(
                r#"{{"name": "{}", "digest": "sha256:{digest}"}}"#,
                swipl.latest
            ),
        );
    }
    put_exec(&fx.mock.join("cargo"), CARGO);
    put_exec(&fx.mock.join("sleep"), "#!/usr/bin/env bash\nexit 0\n");
    let rust = fx.root.join("rust");
    put(
        &rust.join("Cargo.toml"),
        "[workspace]\nmembers = [\"ckc\"]\n",
    );
    put(
        &rust.join("ckc-kani-harness/Cargo.toml"),
        "[package]\nname = \"ckc-kani-harness\"\n",
    );
    let package = |name: &str, version: &str, source: bool, deps: &[&str]| {
        let mut p = format!("\n[[package]]\nname = \"{name}\"\nversion = \"{version}\"\n");
        if source {
            p.push_str(&format!("source = \"{REGISTRY}\"\n"));
        }
        if !deps.is_empty() {
            p.push_str("dependencies = [\n");
            for d in deps {
                p.push_str(&format!(" \"{d}\",\n"));
            }
            p.push_str("]\n");
        }
        p
    };
    put(
        &rust.join("Cargo.lock"),
        [
            "version = 4\n".to_owned(),
            package("ckc", "0.1.0", false, &["ext-macro", "sha2", "zlib-x"]),
            package("ext-macro", "1.0.0", true, &["syn"]),
            package("sha2", "0.11.0", true, &[]),
            package("syn", "2.0.1", true, &[]),
            package("zlib-x", "1.0.0", true, &[]),
        ]
        .concat(),
    );
    put(
        &rust.join("ckc-kani-harness/Cargo.lock"),
        [
            "version = 4\n".to_owned(),
            package("ckc-kani-harness", "0.0.0", false, &[]),
        ]
        .concat(),
    );
    put(
        &rust.join("tools.lock"),
        format!(
            "tool\tversion\tasset\tsha256\turl\njust\t1.58.0\tjust.tar.gz\t{}\thttps://github.com/casey/just/releases/download/1.58.0/just.tar.gz
node\t26.1.0\tnode.tar.xz\t{}\thttps://nodejs.org/dist/v26.1.0/node-v26.1.0-linux-x64.tar.xz
chrome-headless-shell\t155.0.8059.84\tchs.zip\t{}\thttps://storage.googleapis.com/chrome-for-testing-public/155.0.8059.84/linux64/chrome-headless-shell-linux64.zip\n",
            "3".repeat(64),
            "4".repeat(64),
            "5".repeat(64)
        ),
    );
    put(
        &rust.join("verus.lock"),
        "verus_release\trelease/0.2026.10.04.426d8b0\nrustc_toolchain\t1.97.1-x86_64-unknown-linux-gnu\nz3_version\t4.16.0\n",
    );
    put(&rust.join("kani.lock"), "kani_version\t0.68.0\n");
    put(
        &fx.data.join("metadata-main.json"),
        r#"{"packages": [{"name": "ckc", "version": "0.1.0", "dependencies": [
            {"name": "ext-macro", "req": "^1.0"}, {"name": "sha2", "req": "=0.11.0"}, {"name": "zlib-x", "req": "^1.0"}]}]}"#,
    );
    put(
        &fx.data.join("metadata-harness.json"),
        r#"{"packages": [{"name": "ckc-kani-harness", "version": "0.0.0", "dependencies": []}]}"#,
    );
    let dry_run = "     Locking 0 packages to latest compatible versions\nwarning: aborting update due to dry run\n";
    put(&fx.data.join("update-main.txt"), dry_run);
    put(&fx.data.join("update-harness.txt"), dry_run);
    if metadata_fails {
        put(&fx.data.join("metadata-fails"), "");
    }
    let api = "https://api.github.com/repos";
    fx.route(
        &format!("{api}/verus-lang/verus/releases/latest"),
        r#"{"tag_name": "release/0.2026.10.04.426d8b0"}"#,
    );
    fx.route(
        &format!("{api}/model-checking/kani/releases/latest"),
        r#"{"tag_name": "kani-0.68.0"}"#,
    );
    fx.route(
        &format!("{api}/casey/just/releases/latest"),
        r#"{"tag_name": "1.58.0"}"#,
    );
    // Latest node = the highest version, the Current line included (user ruling), wherever
    // index.json lists it.
    fx.route(
        "https://nodejs.org/dist/index.json",
        r#"[{"version": "v24.22.0", "lts": "Krypton"}, {"version": "v26.1.0", "lts": false}, {"version": "v24.21.0", "lts": "Krypton"}]"#,
    );
    fx.route(
        "https://googlechromelabs.github.io/chrome-for-testing/last-known-good-versions.json",
        r#"{"channels": {"Stable": {"version": "155.0.8059.84"}, "Beta": {"version": "156.0.8100.2"}}}"#,
    );
    for (name, latest) in [
        ("ext-macro", "1.0.0"),
        ("sha2", "0.11.1"),
        ("syn", "3.0.0"),
        ("zlib-x", "2.0.0"),
    ] {
        fx.route(
            &format!("https://crates.io/api/v1/crates/{name}"),
            format!(r#"{{"crate": {{"max_stable_version": "{latest}"}}}}"#),
        );
    }
    fx
}

fn outdated_case(
    s: &Scratch,
    name: &str,
    justfile: &str,
    metadata_fails: bool,
    swipl: Swipl,
) -> Output {
    outdated_fixture(s, name, justfile, metadata_fails, swipl).run("outdated", &[])
}

#[test]
fn outdated_reports_a_crate_only_own_pins_hold_as_drift() {
    let s = scratch("outdated");
    let text = justfile();
    let v = swipl_field(&text, "v=");
    let current = || Swipl {
        latest: &v,
        digest: Some(IMAGE),
    };
    let head = format!(
        "verus\trelease/0.2026.10.04.426d8b0\trelease/0.2026.10.04.426d8b0\tok
rustc_toolchain\t1.97.1-x86_64-unknown-linux-gnu\t-\tfollows verus
z3_version\t4.16.0\t-\tfollows verus
kani\t0.68.0\t0.68.0\tok
swipl\t{v}\t{v}\tok
swipl-image\t{v}@sha256:{IMAGE}\t{v}@sha256:{IMAGE}\tok
tool:just\t1.58.0\t1.58.0\tok
tool:node\t26.1.0\t26.1.0\tok
tool:chrome-headless-shell\t155.0.8059.84\t155.0.8059.84\tok
crate:ext-macro\t1.0.0\t1.0.0\tok
"
    );
    let tail = "crate:syn\t2.0.1\t3.0.0\theld\tby ext-macro 1.0.0
crate:zlib-x\t1.0.0\t2.0.0\theld\tby ckc 0.1.0
";
    let out = outdated_case(&s, "own-pin", &text, false, current());
    assert_eq!(out.status.code(), Some(1), "own pin: {}", show(&out));
    assert_eq!(
        String::from_utf8_lossy(&out.stdout),
        format!(
            "{head}crate:sha2\t0.11.0\t0.11.1\tdrift\tpinned by ckc 0.1.0 (=0.11.0)\n{tail}outdated: 11 pins, 1 drift, 2 held, 0 errors\n"
        )
    );
    // An unreadable member requirement set is an error, never a silent `held`.
    let out = outdated_case(&s, "metadata-fails", &text, true, current());
    assert_eq!(out.status.code(), Some(2), "metadata fails: {}", show(&out));
    assert!(
        String::from_utf8_lossy(&out.stdout).contains("cargo-metadata:Cargo.toml\t-\t?\terror\n"),
        "metadata fails: {}",
        show(&out)
    );
    // The pre-rule recipe reports the own pin as held: rc 0, nothing for the weekly job.
    let planted = plant(
        &text,
        &[(
            "        elif [ -n \"$pin\" ]; then report \"crate:$name\" \"$version\" \"$latest\" drift \"pinned by $pin\"\n",
            "",
        )],
    );
    let out = outdated_case(&s, "own-pin-planted", &planted, false, current());
    assert_eq!(
        out.status.code(),
        Some(0),
        "the planted pre-rule recipe no longer slips through: {}",
        show(&out)
    );
    assert!(
        String::from_utf8_lossy(&out.stdout)
            .contains("crate:sha2\t0.11.0\t0.11.1\theld\tby ckc 0.1.0\n")
    );
}

// SWI-Prolog drift (user ruling): the swipl recipe's version vs swi-prolog.org's latest stable,
// the CI container's version@digest vs Docker Hub's digest of that version's tag. The fixture's
// sha2 own pin always drifts, so the summary line carries the SWI share of the counts.
#[test]
fn outdated_reports_swipl_source_and_image_drift() {
    let s = scratch("outdated-swipl");
    let text = justfile();
    let v = swipl_field(&text, "v=");
    let newer = "99.0.1";
    let other = "7".repeat(64);
    let lines = |out: &Output| -> Vec<String> {
        String::from_utf8_lossy(&out.stdout)
            .lines()
            .filter(|l| l.starts_with("swipl") || l.starts_with("outdated:"))
            .map(str::to_owned)
            .collect()
    };
    let out = outdated_case(
        &s,
        "newer",
        &text,
        false,
        Swipl {
            latest: newer,
            digest: Some(&other),
        },
    );
    assert_eq!(out.status.code(), Some(1), "newer: {}", show(&out));
    assert_eq!(
        lines(&out),
        [
            format!("swipl\t{v}\t{newer}\tdrift"),
            format!("swipl-image\t{v}@sha256:{IMAGE}\t{newer}@sha256:{other}\tdrift"),
            "outdated: 11 pins, 3 drift, 2 held, 0 errors".to_owned(),
        ]
    );
    // Docker Hub rebuilt the pinned version's tag: the image alone drifts.
    let out = outdated_case(
        &s,
        "rebuilt",
        &text,
        false,
        Swipl {
            latest: &v,
            digest: Some(&other),
        },
    );
    assert_eq!(
        lines(&out),
        [
            format!("swipl\t{v}\t{v}\tok"),
            format!("swipl-image\t{v}@sha256:{IMAGE}\t{v}@sha256:{other}\tdrift"),
            "outdated: 11 pins, 2 drift, 2 held, 0 errors".to_owned(),
        ]
    );
    // No Docker Hub tag for the latest release: an error, never ok.
    let out = outdated_case(
        &s,
        "no-tag",
        &text,
        false,
        Swipl {
            latest: newer,
            digest: None,
        },
    );
    assert_eq!(out.status.code(), Some(2), "no tag: {}", show(&out));
    assert_eq!(
        lines(&out),
        [
            format!("swipl\t{v}\t{newer}\tdrift"),
            format!("swipl-image\t{v}@sha256:{IMAGE}\t?\terror"),
            "outdated: 11 pins, 2 drift, 2 held, 1 errors".to_owned(),
        ]
    );
    // The pre-rule recipe reports no SWI row: a newer release passes unseen.
    let planted = plant(
        &text,
        &[
            ("    report swipl \"${sv:-?}\" \"${sv:+$sl}\"\n", ""),
            (
                "    report swipl-image \"${img:-?}\" \"${hub:+$sl@$hub}\"\n",
                "",
            ),
        ],
    );
    let out = outdated_case(
        &s,
        "planted",
        &planted,
        false,
        Swipl {
            latest: newer,
            digest: Some(&other),
        },
    );
    assert_eq!(
        lines(&out),
        ["outdated: 9 pins, 1 drift, 2 held, 0 errors"],
        "the planted pre-rule recipe no longer slips through: {}",
        show(&out)
    );
}

// Latest node = the highest released version, the Current line included (user ruling): a pin on
// the LTS line drifts once a newer Current release exists.
#[test]
fn outdated_reads_nodes_highest_release() {
    let s = scratch("outdated-node");
    let text = justfile();
    let v = swipl_field(&text, "v=");
    let node = |name: &str, justfile: &str| {
        let fx = outdated_fixture(
            &s,
            name,
            justfile,
            false,
            Swipl {
                latest: &v,
                digest: Some(IMAGE),
            },
        );
        let lock = fx.root.join("rust/tools.lock");
        put(
            &lock,
            fs::read_to_string(&lock)
                .unwrap()
                .replace("26.1.0", "24.22.0"),
        );
        let out = fx.run("outdated", &[]);
        String::from_utf8_lossy(&out.stdout)
            .lines()
            .find(|l| l.starts_with("tool:node\t"))
            .unwrap_or_else(|| panic!("{name}: no node row: {}", show(&out)))
            .to_owned()
    };
    assert_eq!(node("lts-pin", &text), "tool:node\t24.22.0\t26.1.0\tdrift");
    // The pre-rule recipe reads the newest LTS line: the newer Current release passes unseen.
    let planted = plant(
        &text,
        &[(
            r#"jq -r '[.[].version | ltrimstr("v") | split(".") | map(tonumber)] | max | select(. != null) | map(tostring) | join(".")')"#,
            r#"jq -r '[.[] | select(.lts != false)][0].version // empty' | sed 's/^v//')"#,
        )],
    );
    assert_eq!(
        node("lts-pin-planted", &planted),
        "tool:node\t24.22.0\t24.22.0\tok",
        "the planted pre-rule recipe no longer slips through"
    );
}

#[test]
fn bump_crate_moves_a_members_own_pin() {
    let s = scratch("bump-crate");
    let text = justfile();
    let ckc_toml = "[dependencies]\nflate2 = { version = \"=1.1.10\", default-features = false }\nsha2 = { version = \"=0.11.0\", default-features = false }\nsha2-extra = \"=0.11.0\"\n";
    let harness_toml = "[dependencies]\nsha2 = \"0.11\"\n";
    let workspace_toml = "[workspace]\nmembers = [\"ckc\"]\n\n[workspace.dependencies]\nvstd = \"=0.0.0-2026-08-30-0159\"\n";
    let case = |name: &str, justfile: &str| {
        let fx = Fixture::new(&s, name, justfile);
        put_exec(&fx.mock.join("cargo"), CARGO);
        put_exec(
            &fx.mock.join("ckc"),
            "#!/usr/bin/env bash\n[ \"$1 $2 $3\" = \"trust-audit --write rust\" ] || exit 64\n",
        );
        let rust = fx.root.join("rust");
        put(&rust.join("Cargo.toml"), workspace_toml);
        put(&rust.join("ckc/Cargo.toml"), ckc_toml);
        put(&rust.join("ckc-kani-harness/Cargo.toml"), harness_toml);
        put(
            &rust.join("Cargo.lock"),
            format!(
                "version = 4\n\n[[package]]\nname = \"sha2\"\nversion = \"0.11.0\"\nsource = \"{REGISTRY}\"\n"
            ),
        );
        put(&rust.join("ckc-kani-harness/Cargo.lock"), "version = 4\n");
        put(
            &rust.join("verus.lock"),
            "rustc_toolchain\t1.97.1-x86_64-unknown-linux-gnu\n",
        );
        put(
            &rust.join("trust/escape-allowlist.tsv"),
            "path\ttoken\tcount\tline\n",
        );
        let out = fx.run("bump-crate", &["sha2", "0.11.1"]);
        (out, fx)
    };
    let (out, fx) = case("own-pin", &text);
    assert_eq!(out.status.code(), Some(0), "bump-crate: {}", show(&out));
    assert_eq!(
        fx.read("rust/ckc/Cargo.toml"),
        ckc_toml.replace(
            "sha2 = { version = \"=0.11.0\"",
            "sha2 = { version = \"=0.11.1\""
        )
    );
    assert_eq!(fx.read("rust/Cargo.toml"), workspace_toml);
    assert_eq!(fx.read("rust/ckc-kani-harness/Cargo.toml"), harness_toml);
    let rust = fx.root.join("rust");
    let r = rust.display();
    assert_eq!(
        fs::read_to_string(fx.data.join("cargo.log")).unwrap(),
        format!(
            "update -q --manifest-path {r}/Cargo.toml -p sha2 --precise 0.11.1\nvendor -q --locked --manifest-path {r}/Cargo.toml --sync {r}/ckc-kani-harness/Cargo.toml {r}/vendor\n"
        )
    );
    // The pre-rule recipe leaves the pin, so cargo's --precise relock fails on a real tree.
    let planted = plant(
        &text,
        &[(
            "        sed -i -E 's/^({{ crate }} = (\\{ *version = )?\")=[^\"]*\"/\\1={{ version }}\"/' \"$toml\"\n",
            "",
        )],
    );
    let (out, fx) = case("own-pin-planted", &planted);
    assert_eq!(out.status.code(), Some(0), "planted: {}", show(&out));
    assert_eq!(
        fx.read("rust/ckc/Cargo.toml"),
        ckc_toml,
        "the planted pre-rule recipe no longer slips through"
    );
}

// Mock cargo for `test`: a run whose second binary dies by a signal, as cargo reports it: no
// panic, no `test result` line, the cause only in cargo's own error lines; exits 101.
const CARGO_TEST: &str = r#"#!/usr/bin/env bash
cat <<'LOG'
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.40s
     Running unittests src/lib.rs (target/debug/deps/ckc-0123456789abcdef)
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s

     Running tests/aaa_abort.rs (target/debug/deps/aaa_abort-33c9abcd291590e3)
error: test failed, to rerun pass `-p ckc --test aaa_abort`

Caused by:
  process didn't exit successfully: `/x/rust/target/debug/deps/aaa_abort-33c9abcd291590e3 --quiet` (signal: 6, SIGABRT: process abort signal)
LOG
exit 101
"#;

#[test]
fn test_recipe_names_a_binary_that_dies_without_a_result() {
    let s = scratch("test-recipe");
    let text = justfile();
    let case = |name: &str, justfile: &str| {
        let fx = Fixture::new(&s, name, justfile);
        put_exec(&fx.mock.join("cargo"), CARGO_TEST);
        fs::create_dir_all(fx.root.join("rust")).unwrap();
        let out = fx.run("test", &[]);
        (out, fx)
    };
    let (out, fx) = case("signal", &text);
    assert_eq!(out.status.code(), Some(101), "signal: {}", show(&out));
    let stdout = String::from_utf8_lossy(&out.stdout);
    for line in [
        "test: 1 binaries, 7 passed, 0 failed, 0 ignored\n",
        "test: no result line: aaa_abort\n",
        "error: test failed, to rerun pass `-p ckc --test aaa_abort`\n",
        "(signal: 6, SIGABRT: process abort signal)\n",
        "test: rc 101, log kept at rust/target/test.log\n",
    ] {
        assert!(stdout.contains(line), "missing {line:?}: {}", show(&out));
    }
    assert!(
        fx.read("rust/target/test.log").contains("SIGABRT"),
        "log kept"
    );
    // The pre-fix recipe (temp log deleted, panics + failure lists alone) prints no cause.
    let planted = plant(
        &text,
        &[
            (
                "        awk '$1 == \"start\" {if (n != \"\" && !d) print \"test: no result line: \" n; n = $2; d = 0; next} {d = 1}\n            END {if (n != \"\" && !d) print \"test: no result line: \" n}' \"$log.t\"\n",
                "",
            ),
            (
                "        grep -E \"^error|process didn't exit successfully\" \"$log\" | head -40\n",
                "",
            ),
            (
                "        echo \"test: rc $rc, log kept at rust/$log\"\n",
                "        rm -f \"$log\"\n",
            ),
        ],
    );
    let (out, fx) = case("signal-planted", &planted);
    assert_eq!(out.status.code(), Some(101), "planted: {}", show(&out));
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        !stdout.contains("SIGABRT") && !stdout.contains("aaa_abort"),
        "the planted pre-fix recipe no longer slips through: {}",
        show(&out)
    );
    assert!(!fx.root.join("rust/target/test.log").exists());
}

// Mock cargo with two failing binaries, honoring `--no-fail-fast` as cargo does: without it,
// the run stops after the first failing binary.
const CARGO_TWO_FAILURES: &str = r#"#!/usr/bin/env bash
nff=0
for a in "$@"; do [ "$a" = --no-fail-fast ] && nff=1; done
cat <<'LOG'
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.40s
     Running tests/aaa_fail.rs (target/debug/deps/aaa_fail-33c9abcd291590e3)
a1 --- FAILED
failures:
    a1

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

error: test failed, to rerun pass `-p ckc --test aaa_fail`
LOG
[ "$nff" = 1 ] || exit 101
cat <<'LOG'
     Running tests/bbb_fail.rs (target/debug/deps/bbb_fail-56fa083047ca9c05)
b1 --- FAILED
failures:
    b1

test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

error: test failed, to rerun pass `-p ckc --test bbb_fail`
     Running tests/ccc_ok.rs (target/debug/deps/ccc_ok-00112867f077590f)
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

error: 2 targets failed:
    `-p ckc --test aaa_fail`
    `-p ckc --test bbb_fail`
LOG
exit 101
"#;

#[test]
fn test_recipe_reports_every_failing_binary() {
    let s = scratch("test-every-binary");
    let text = justfile();
    let case = |name: &str, justfile: &str| {
        let fx = Fixture::new(&s, name, justfile);
        put_exec(&fx.mock.join("cargo"), CARGO_TWO_FAILURES);
        fs::create_dir_all(fx.root.join("rust")).unwrap();
        fx.run("test", &[])
    };
    let out = case("two-failures", &text);
    assert_eq!(out.status.code(), Some(101), "two failures: {}", show(&out));
    let stdout = String::from_utf8_lossy(&out.stdout);
    for line in [
        "test: 3 binaries, 4 passed, 3 failed, 0 ignored\n",
        "test: failed: aaa_fail (1 failed)\n",
        "test: failed: bbb_fail (2 failed)\n",
        "error: test failed, to rerun pass `-p ckc --test bbb_fail`\n",
    ] {
        assert!(stdout.contains(line), "missing {line:?}: {}", show(&out));
    }
    // The fail-fast recipe stops at the first failing binary: the second one goes unreported.
    let planted = plant(
        &text,
        &[(
            "--offline --no-fail-fast -- --quiet",
            "--offline -- --quiet",
        )],
    );
    let out = case("two-failures-planted", &planted);
    assert_eq!(out.status.code(), Some(101), "planted: {}", show(&out));
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("test: failed: aaa_fail (1 failed)\n") && !stdout.contains("bbb_fail"),
        "the planted fail-fast recipe no longer hides the second failure: {}",
        show(&out)
    );
}

// build-static inspects the static binary: a failed readelf, a grep that cannot read
// the binary, or dynamic markers ahead of a long readelf listing (an early `grep -q`
// exit = SIGPIPE under pipefail) must each fail the recipe, never print its green line.
#[test]
fn build_static_inspection_failures_fail_the_recipe() {
    let s = scratch("build-static");
    let text = justfile();
    let case = |name: &str, justfile: &str, readelf: &str, grep_fails: bool| {
        let fx = Fixture::new(&s, name, justfile);
        put_exec(&fx.mock.join("cargo"), "#!/usr/bin/env bash\nexit 0\n");
        put_exec(
            &fx.mock.join("rustc"),
            "#!/usr/bin/env bash\necho /fixture/sysroot\n",
        );
        put(
            &fx.root
                .join("rust/target/x86_64-unknown-linux-musl/release/ckc"),
            "fixture binary\n",
        );
        put_exec(&fx.mock.join("readelf"), readelf);
        if grep_fails {
            put_exec(
                &fx.mock.join("grep"),
                "#!/usr/bin/env bash\n[ \"$1\" != -qF ] || exit 2\nexec /usr/bin/grep \"$@\"\n",
            );
        }
        fx.run("build-static", &[])
    };
    let quiet = "#!/usr/bin/env bash\necho 'ELF Header:'\n";
    let fails = "#!/usr/bin/env bash\nexit 23\n";
    let long = "#!/usr/bin/env bash\ncase \"$1\" in -lW) echo '  INTERP' ;; -dW) echo ' (NEEDED)' ;; esac\nfor ((i = 0; i < 20000; i++)); do echo 'ELF filler line'; done\n";
    let green = |out: &Output| {
        out.status.code() == Some(0) && out.stdout.ends_with(b"gate: build-static ok\n")
    };
    let out = case("control", &text, quiet, false);
    assert!(green(&out), "control: {}", show(&out));
    for (name, readelf, grep_fails) in [
        ("readelf-fails", fails, false),
        ("grep-fails", quiet, true),
        ("dynamic-long", long, false),
    ] {
        let out = case(name, &text, readelf, grep_fails);
        assert!(
            out.status.code() != Some(0) && !out.stdout.ends_with(b"gate: build-static ok\n"),
            "{name} passed: {}",
            show(&out)
        );
    }
    let planted = plant(
        &text,
        &[(
            "    prog=$(readelf -lW \"$bin\"); dyn=$(readelf -dW \"$bin\")\n    if grep -q INTERP <<< \"$prog\" || grep -q NEEDED <<< \"$dyn\"; then\n",
            "    if readelf -lW \"$bin\" | grep -q INTERP || readelf -dW \"$bin\" | grep -q NEEDED; then\n",
        )],
    );
    let out = case("planted", &planted, fails, false);
    assert!(
        green(&out),
        "the planted pre-fix build-static no longer slips through: {}",
        show(&out)
    );
}

// rust/kani/bootstrap.sh over a cached install: a version probe that prints the pinned
// version and then exits 23 must fail the script, never stamp `kani: <v> ok`.
#[test]
fn kani_bootstrap_version_probes_fail_the_script() {
    let s = scratch("kani-bootstrap");
    let script = fs::read_to_string(root().join("rust/kani/bootstrap.sh")).unwrap();
    let base_lock = fs::read_to_string(root().join("rust/kani.lock")).unwrap();
    let data = b"fixture pinned bytes\n";
    let digest = hex(data);
    let lock: String = base_lock
        .lines()
        .map(|l| {
            let mut f: Vec<String> = l.split('\t').map(str::to_owned).collect();
            if f[0].ends_with("_sha256") {
                f[1] = digest.clone();
            }
            if f[0] == "vendor_crate" {
                f[3] = digest.clone();
            }
            f.join("\t") + "\n"
        })
        .collect();
    let value = |key: &str| -> String {
        lock.lines()
            .find_map(|l| l.strip_prefix(&format!("{key}\t")).map(str::to_owned))
            .unwrap()
    };
    let case = |name: &str, script: &str, failing: &str| {
        let fx = Fixture::new(&s, name, "");
        put_exec(&fx.root.join("rust/kani/bootstrap.sh"), script);
        put(&fx.root.join("rust/kani.lock"), &lock);
        put(
            &fx.root.join("rust/verus.lock"),
            fs::read(root().join("rust/verus.lock")).unwrap(),
        );
        let prefix = fx.toolchain.join("kani");
        for l in lock.lines() {
            let f: Vec<&str> = l.split('\t').collect();
            if f[0].ends_with("_asset") {
                put(&prefix.join("dl").join(f[1]), data);
            }
            if f[0] == "vendor_crate" {
                put(&prefix.join(format!("dl/{}-{}.crate", f[1], f[2])), data);
            }
        }
        put(
            &prefix.join("lock.sha256"),
            format!("{}\n", hex(lock.as_bytes())),
        );
        let tc = fx
            .toolchain
            .join("rustup/toolchains")
            .join(value("bootstrap_rustc_toolchain"))
            .join("bin");
        probe(
            &tc.join("rustc"),
            &value("bootstrap_rustc_version"),
            failing == "bootstrap-rustc",
        );
        put_exec(&tc.join("cargo"), "#!/usr/bin/env bash\nexit 97\n");
        probe(
            &prefix.join("toolchain/bin/rustc"),
            &value("rustc_version"),
            failing == "installed-rustc",
        );
        let id = format!("Kani Rust Verifier {}", value("kani_version"));
        put(
            &prefix.join("wrapper/.crates.toml"),
            format!(
                "\"kani-verifier {} (path+file:///fixture)\" = [\"cargo-kani\", \"kani\"]\n",
                value("kani_version")
            ),
        );
        probe(
            &prefix.join("wrapper/bin/cargo-kani"),
            &format!("{id} (cargo plugin)"),
            failing == "cargo-kani",
        );
        probe(
            &prefix.join("wrapper/bin/kani"),
            &format!("{id} (standalone)"),
            failing == "kani",
        );
        let installed = prefix.join(format!("kani-home/kani-{}", value("kani_version")));
        probe(
            &installed.join("bin/kani-driver"),
            &format!("{id} (standalone)"),
            failing == "kani-driver",
        );
        probe(
            &installed.join("bin/cbmc"),
            &value("cbmc_version"),
            failing == "cbmc",
        );
        probe(
            &installed.join("bin/kissat"),
            &value("kissat_version"),
            failing == "kissat",
        );
        std::os::unix::fs::symlink(prefix.join("toolchain"), installed.join("toolchain")).unwrap();
        fx.command(Path::new("/bin/bash"))
            .arg(fx.root.join("rust/kani/bootstrap.sh"))
            .env("RUSTUP_HOME", fx.toolchain.join("rustup"))
            .output()
            .unwrap()
    };
    let ok = format!("kani: {} ok\n", value("kani_version"));
    let out = case("control", &script, "");
    assert_eq!(out.status.code(), Some(0), "control: {}", show(&out));
    assert_eq!(String::from_utf8_lossy(&out.stdout), ok);
    for failing in [
        "bootstrap-rustc",
        "installed-rustc",
        "kani-driver",
        "cargo-kani",
        "kani",
        "cbmc",
        "kissat",
    ] {
        let out = case(failing, &script, failing);
        assert!(
            out.status.code() != Some(0) && !out.stdout.ends_with(ok.as_bytes()),
            "{failing}: a failed version probe passed: {}",
            show(&out)
        );
    }
    let planted = plant(
        &script,
        &[(
            "first_line_is \"$KANI_ID (cargo plugin)\" cargo-kani --version || fail 'final cargo-kani version mismatch'\n",
            "[[ \"$(cargo-kani --version 2>/dev/null || true)\" == \"$KANI_ID (cargo plugin)\" ]] || fail 'final cargo-kani version mismatch'\n",
        )],
    );
    let out = case("planted", &planted, "cargo-kani");
    assert_eq!(
        out.status.code(),
        Some(0),
        "the planted pre-fix bootstrap no longer slips through: {}",
        show(&out)
    );
}

// `just --list` is the gate's CLI help: every public recipe shows one standalone
// sentence (a `[doc("…")]`), never the last line of a longer comment.
fn help_fragments(justfile: &Path) -> Vec<String> {
    let out = Command::new(just())
        .arg("--justfile")
        .arg(justfile)
        .args(["--list", "--unsorted"])
        .output()
        .unwrap();
    assert!(out.status.success(), "just --list: {}", show(&out));
    String::from_utf8(out.stdout)
        .unwrap()
        .lines()
        .skip(1)
        .filter(|line| {
            let doc = line.split_once(" # ").map(|(_, d)| d).unwrap_or("");
            !(doc.starts_with(|c: char| c.is_ascii_uppercase()) && doc.ends_with('.'))
        })
        .map(str::to_owned)
        .collect()
}

#[test]
fn recipe_list_gives_every_recipe_a_sentence() {
    let text = justfile();
    let s = scratch("help");
    let put_justfile = |name: &str, body: &str| {
        let path = s.0.join(name).join("justfile");
        put(&path, body);
        path
    };
    let fragments = help_fragments(&put_justfile("control", &text));
    assert!(
        fragments.is_empty(),
        "recipes without a help sentence: {fragments:?}"
    );
    let planted = plant(
        &text,
        &[("[doc(\"Build the release ckc program.\")]\n", "")],
    );
    let fragments = help_fragments(&put_justfile("planted", &planted));
    assert!(
        fragments
            .iter()
            .any(|l| l.split_whitespace().next() == Some("build")),
        "a recipe without its [doc] slipped through: {fragments:?}"
    );
}

// The gate prints each step's wall-clock and the run's total (CLAUDE.md `Execution`):
// `_timed` over two steps (each a mocked nested `just`) prints both step lines + total.
fn timed_lines(s: &Scratch, case: &str, justfile: &str) -> Vec<String> {
    let fx = Fixture::new(s, case, justfile);
    put_exec(&fx.mock.join("just"), "#!/usr/bin/env bash\nexit 0\n");
    let out = fx.run("_timed", &["fmt-check clippy"]);
    assert_eq!(out.status.code(), Some(0), "{case}: {}", show(&out));
    String::from_utf8(out.stdout)
        .unwrap()
        .lines()
        .filter(|l| l.starts_with("time: "))
        .map(|l| {
            l.rsplit_once(' ')
                .unwrap()
                .0
                .rsplit_once(' ')
                .unwrap()
                .0
                .to_owned()
        })
        .collect()
}

#[test]
fn gate_steps_print_their_wall_clock() {
    let s = scratch("timed");
    let text = justfile();
    let want = ["time: fmt-check", "time: clippy", "time: total"];
    assert_eq!(timed_lines(&s, "control", &text), want);
    let planted = plant(
        &text,
        &[("    echo \"time: total $((SECONDS - start)) s\"\n", "")],
    );
    assert_ne!(
        timed_lines(&s, "planted", &planted),
        want,
        "a missing total line slipped through"
    );
    // A failing step still ends the run with its total, and the run keeps the step's rc.
    let fx = Fixture::new(&s, "failing", &text);
    put_exec(&fx.mock.join("just"), "#!/usr/bin/env bash\nexit 7\n");
    let out = fx.run("_timed", &["fmt-check clippy"]);
    let lines: Vec<String> = String::from_utf8(out.stdout.clone())
        .unwrap()
        .lines()
        .filter(|l| l.starts_with("time: "))
        .map(|l| {
            l.rsplit_once(' ')
                .unwrap()
                .0
                .rsplit_once(' ')
                .unwrap()
                .0
                .to_owned()
        })
        .collect();
    assert_ne!(out.status.code(), Some(0), "failing: {}", show(&out));
    assert_eq!(
        lines,
        ["time: fmt-check", "time: total"],
        "failing: {}",
        show(&out)
    );
}

// CI runs every chain `just gate` runs (CLAUDE.md `Execution` CI gate): each component
// of the gate recipe has a `just <recipe>` step in ci.yml (RUST_STEPS = `just rust`).
fn ci_gaps(justfile: &str, ci: &str) -> Vec<String> {
    let gate = justfile
        .lines()
        .find_map(|l| l.strip_prefix("gate: (_timed RUST_STEPS + \""))
        .and_then(|l| l.strip_suffix("\")"))
        .expect("gate recipe shape");
    std::iter::once("rust")
        .chain(gate.split_whitespace())
        .filter(|step| {
            !ci.lines().any(|l| {
                l.split_whitespace()
                    .collect::<Vec<_>>()
                    .windows(2)
                    .any(|w| w[0] == "just" && w[1] == *step)
            })
        })
        .map(str::to_owned)
        .collect()
}

#[test]
fn ci_runs_every_gate_chain() {
    let text = justfile();
    let ci = fs::read_to_string(root().join(".github/workflows/ci.yml")).unwrap();
    assert_eq!(ci_gaps(&text, &ci), Vec::<String>::new());
    let planted = ci.replace("just mobile", "just build");
    assert_eq!(
        ci_gaps(&text, &planted),
        ["mobile"],
        "a dropped CI chain slipped through"
    );
}

// Jobs of ci.yml that call a recipe before the shared bootstrap step installs the pinned
// `just` (.github/actions/bootstrap-just), or the whole workflow when that action stops
// verifying the tools.lock row: a runner image's own `just` would otherwise run unpinned.
fn unbootstrapped_jobs(ci: &str, action: &str) -> Vec<String> {
    if !(action.contains("$1==\"just\"{print $4}") && action.contains("sha256sum -c")) {
        return vec!["bootstrap action".to_owned()];
    }
    let mut out = Vec::new();
    let mut job: Option<(String, bool, bool)> = None; // (id, bootstrapped, reported)
    for line in ci.lines().skip_while(|l| *l != "jobs:").skip(1) {
        if let Some(id) = line.strip_prefix("  ").and_then(|l| l.strip_suffix(':'))
            && !id.starts_with([' ', '#'])
        {
            job = Some((id.to_owned(), false, false));
            continue;
        }
        let Some((id, booted, reported)) = job.as_mut() else {
            continue;
        };
        if line.trim() == "uses: $/.github/actions/bootstrap-just" {
            *booted = true;
        }
        let calls = line
            .split_whitespace()
            .collect::<Vec<_>>()
            .windows(2)
            .any(|w| w[0] == "just" && w[1].starts_with(|c: char| c.is_ascii_lowercase()));
        if calls && !*booted && !*reported {
            *reported = true;
            out.push(id.clone());
        }
    }
    out
}

#[test]
fn ci_jobs_bootstrap_the_pinned_just() {
    let ci = fs::read_to_string(root().join(".github/workflows/ci.yml")).unwrap();
    let action =
        fs::read_to_string(root().join(".github/actions/bootstrap-just/action.yml")).unwrap();
    assert_eq!(unbootstrapped_jobs(&ci, &action), Vec::<String>::new());
    let at = ci.find("  certify:\n").expect("certify job");
    let step = "        uses: $/.github/actions/bootstrap-just\n";
    let cut = at + ci[at..].find(step).expect("certify bootstrap step");
    let planted = format!("{}{}", &ci[..cut], &ci[cut + step.len()..]);
    assert_eq!(
        unbootstrapped_jobs(&planted, &action),
        ["certify"],
        "a job without the pinned just slipped through"
    );
    let unverified = action.replace("sha256sum -c", "true");
    assert_eq!(
        unbootstrapped_jobs(&ci, &unverified),
        ["bootstrap action"],
        "an unverified bootstrap slipped through"
    );
}

// Tools install inside the project (CLAUDE.md `Execution`: project-local): the toolchain
// default of the justfile + the Kani bootstrap sits under the checkout.
#[test]
fn toolchain_defaults_stay_project_local() {
    let local = |justfile: &str, bootstrap: &str| {
        justfile.contains("TOOLCHAIN := env(\"CKC_TOOLCHAIN\", ROOT / \".toolchain\")\n")
            && bootstrap.contains("BASE=${CKC_TOOLCHAIN:-$PROJECT/.toolchain}\n")
    };
    let text = justfile();
    let bootstrap = fs::read_to_string(root().join("rust/kani/bootstrap.sh")).unwrap();
    assert!(local(&text, &bootstrap));
    let planted = text.replacen("ROOT / \".toolchain\"", "env(\"HOME\") / \".ckc\"", 1);
    assert!(
        !local(&planted, &bootstrap),
        "a toolchain outside the project slipped through"
    );
}
