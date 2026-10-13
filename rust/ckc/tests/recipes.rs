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
    let dir = root().join(format!("rust/target/recipes/{}/{name}", std::process::id()));
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

#[test]
fn swipl_version_probe_fails_the_recipe() {
    let s = scratch("swipl");
    let text = justfile();
    let field = |key: &str| {
        recipe(&text, "swipl")
            .split([' ', ';', '\n'])
            .find_map(|w| w.strip_prefix(key))
            .unwrap_or_else(|| panic!("swipl recipe holds no {key}"))
            .to_owned()
    };
    let (version, pin) = (field("v="), field("sha="));
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
";

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
            "https://api.github.com/repos/zizmorcore/zizmor/releases/tags/v1.31.0",
            "zizmor-x86_64-unknown-linux-gnu.tar.gz",
            "https://github.com/zizmorcore/zizmor/releases/download/v1.31.0/zizmor-x86_64-unknown-linux-gnu.tar.gz",
        ),
        (
            "just",
            "1.59.0",
            "https://api.github.com/repos/casey/just/releases/tags/1.59.0",
            "just-1.59.0-x86_64-unknown-linux-musl.tar.gz",
            "https://github.com/casey/just/releases/download/1.59.0/just-1.59.0-x86_64-unknown-linux-musl.tar.gz",
        ),
        (
            "rustup-init",
            "1.29.2",
            "",
            "rustup-init",
            "https://static.rust-lang.org/rustup/archive/1.29.2/x86_64-unknown-linux-gnu/rustup-init",
        ),
    ];
    for (tool, version, api, asset, url) in controls {
        let fx = fixture(tool);
        let bytes = format!("{tool} {version} fixture asset\n");
        let digest = hex(bytes.as_bytes());
        if api.is_empty() {
            fx.route(&format!("{url}.sha256"), format!("{digest} *./{asset}\n"));
        } else {
            fx.route(api, release(asset, url, Some(&digest)));
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
    // A digest that disagrees with the download, then a release without one: refused, untouched.
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
