# Gate driver. `just gate` = rust chain + corpus chain + emission certification;
# CI runs the same recipes. Every tool is sha256-pinned (rust/verus.lock,
# rust/tools.lock) and installed under .toolchain/ by `just tools`; CKC_TOOLCHAIN
# relocates it.

set shell := ["bash", "-euo", "pipefail", "-c"]

ROOT := justfile_directory()
TOOLCHAIN := env("CKC_TOOLCHAIN", ROOT / ".toolchain")
export RUSTUP_HOME := TOOLCHAIN / "rustup"
export CARGO_HOME := TOOLCHAIN / "cargo"
# .toolchain/bin first: the swipl shim must shadow a system copy.
export PATH := (TOOLCHAIN / "bin") + ":" + (TOOLCHAIN / "verus-x86-linux") + ":" + (TOOLCHAIN / "cargo/bin") + ":" + env("PATH")

default:
    @just --list --unsorted

# Full gate: rust chain, corpus chain, emission certification.
gate: rust check certify

# Rust chain: format, lint, verify, build, trust-audit, test, dependency audit, secret scan, workflow scan.
rust: fmt-check clippy verify build trust test deny secrets workflows

# Install every pinned tool into the toolchain dir (idempotent, digest-verified).
tools:
    #!/usr/bin/env bash
    set -euo pipefail
    T="{{ TOOLCHAIN }}"; DL="$T/dl"; mkdir -p "$DL" "$T/bin"
    lock() { awk -F'\t' -v k="$1" '$1==k{print $2}' "{{ ROOT }}/rust/verus.lock"; }
    fetch() { # asset url sha256 → $DL/asset
        if [ ! -f "$DL/$1" ] || ! printf '%s  %s\n' "$3" "$DL/$1" | sha256sum -c --quiet >/dev/null 2>&1; then
            curl -sSL --retry 3 -o "$DL/$1" "$2"
        fi
        printf '%s  %s\n' "$3" "$DL/$1" | sha256sum -c --quiet
    }
    # $T/<tool>/.pin = the lock row's sha256 at install; any other value reinstalls.
    pinned() { [ "$(cat "$1/.pin" 2>/dev/null)" = "$2" ]; }
    while IFS=$'\t' read -r tool version asset sha url; do
        [ "$tool" = tool ] && continue
        if [ "$tool" = rustup-init ]; then
            [ -x "$T/cargo/bin/rustup" ] && pinned "$T/$tool" "$sha" && continue
            fetch "$asset" "$url" "$sha"; chmod +x "$DL/$asset"
            "$DL/$asset" -y -q --no-modify-path --default-toolchain none --profile minimal >/dev/null
            mkdir -p "$T/$tool"; printf '%s\n' "$sha" > "$T/$tool/.pin"
            continue
        fi
        [ -x "$T/bin/$tool" ] && pinned "$T/$tool" "$sha" && continue
        fetch "$asset" "$url" "$sha"
        rm -rf "$T/$tool"; mkdir -p "$T/$tool"
        case "$asset" in
            *.tar.gz) tar -xzf "$DL/$asset" -C "$T/$tool" ;;
            *.tar.xz) tar -xJf "$DL/$asset" -C "$T/$tool" ;;
            *) cp "$DL/$asset" "$T/$tool/$tool"; chmod +x "$T/$tool/$tool" ;;
        esac
        bin=$(find "$T/$tool" -type f -name "$tool" -perm -u+x | head -1)
        ln -sfn "$bin" "$T/bin/$tool"
        printf '%s\n' "$sha" > "$T/$tool/.pin"
    done < "{{ ROOT }}/rust/tools.lock"
    tc=$(lock rustc_toolchain)
    # musl target = `just build-static` (the CI container jobs' binary).
    "$T/cargo/bin/rustup" -q toolchain install "$tc" --profile minimal --component clippy --component rustfmt \
        --target x86_64-unknown-linux-musl
    if [ ! -x "$T/verus-x86-linux/verus" ] || ! pinned "$T/verus-x86-linux" "$(lock verus_asset_sha256)"; then
        fetch "$(lock verus_asset)" "$(lock verus_asset_url)" "$(lock verus_asset_sha256)"
        rm -rf "$T/verus-x86-linux"
        unzip -q -o "$DL/$(lock verus_asset)" -d "$T"
        printf '%s\n' "$(lock verus_asset_sha256)" > "$T/verus-x86-linux/.pin"
    fi
    echo "tools: ok rustc=$tc verus=$(lock verus_release) $(awk -F'\t' 'NR>1{printf "%s=%s ", $1, $2}' "{{ ROOT }}/rust/tools.lock")"

# Reformat the workspace (verusfmt inside verus!, rustfmt outside).
fmt:
    cd "{{ ROOT }}/rust" && verusfmt --edition 2024 $(find ckc-spec/src ckc-kernel/src ckc/src ckc/tests -name '*.rs' | sort)

fmt-check:
    cd "{{ ROOT }}/rust" && verusfmt --edition 2024 --check $(find ckc-spec/src ckc-kernel/src ckc/src ckc/tests -name '*.rs' | sort)
    @echo "gate: fmt ok"

clippy:
    cd "{{ ROOT }}/rust" && cargo clippy --workspace --locked --offline --all-targets -q -- -D warnings
    @echo "gate: clippy ok"

# Verus gate: warm vstd plain, then re-verify every first-party crate under --no-cheating.
verify:
    #!/usr/bin/env bash
    set -euo pipefail
    cd "{{ ROOT }}/rust"
    # vstd axiomatizes std and cannot pass --no-cheating, so its cache warms
    # plain through ckc-spec; cargo then skips unchanged crates whatever the
    # verifier flag, so the first-party crates are cleaned to force the check.
    cargo verus verify -p ckc-spec --locked --offline >/dev/null
    cargo clean -p ckc-spec -p ckc-kernel --locked --offline
    cargo verus verify --workspace --locked --offline -- --no-cheating
    echo "gate: verify ok"

build:
    cd "{{ ROOT }}/rust" && cargo build --release --locked --offline -q
    @echo "gate: build ok"

# The swipl:9.2.9 image (Debian glibc 2.36) cannot load the runner's glibc-2.39 build.
# Static musl ckc = the binary of the CI container jobs.
build-static:
    #!/usr/bin/env bash
    set -euo pipefail
    cd "{{ ROOT }}/rust"
    cargo build --release --locked --offline -q -p ckc --target x86_64-unknown-linux-musl
    bin=target/x86_64-unknown-linux-musl/release/ckc
    if readelf -lW "$bin" | grep -q INTERP || readelf -dW "$bin" | grep -q NEEDED; then
        echo "build-static: $bin is dynamically linked" >&2; exit 1
    fi
    echo "gate: build-static ok"

trust: build
    cd "{{ ROOT }}/rust" && ./target/release/ckc trust-audit

test:
    cd "{{ ROOT }}/rust" && cargo test --workspace --locked --offline -q 2>&1 | grep -E '^test result' | sort | uniq -c
    @echo "gate: test ok"

# Dependency audit: RustSec advisories, license allowlist, bans, sources (rust/deny.toml).
deny:
    cd "{{ ROOT }}/rust" && cargo-deny -L warn check
    @echo "gate: deny ok"

# Secret scan: full history, then unstaged and staged changes (.gitleaks.toml).
secrets:
    cd "{{ ROOT }}" && gitleaks git --no-banner --redact -c .gitleaks.toml . >/dev/null
    cd "{{ ROOT }}" && gitleaks git --no-banner --redact -c .gitleaks.toml --pre-commit . >/dev/null
    cd "{{ ROOT }}" && gitleaks git --no-banner --redact -c .gitleaks.toml --staged . >/dev/null
    @echo "gate: secrets ok"

# Workflow scan: zizmor over .github/ (offline audits; online audits too when GH_TOKEN is set).
workflows:
    cd "{{ ROOT }}" && zizmor --no-progress --color never $([ -n "${GH_TOKEN:-}" ] || echo --offline) .github
    @echo "gate: workflows ok"

# Prints `pin current latest status`; rc 1 = drift (bump procedure: .claude/rules/rust.md),
# rc 2 = a failed or empty upstream answer; CKC_OUTDATED_ROOT = tree holding the locks.
# Pin drift: every lock pin vs upstream latest (crates.io, GitHub releases, rustup).
outdated:
    #!/usr/bin/env bash
    set -uo pipefail
    R="{{ env('CKC_OUTDATED_ROOT', ROOT) }}/rust"
    pins=0 drift=0 errors=0
    for f in Cargo.lock ckc-kani-harness/Cargo.lock tools.lock verus.lock kani.lock; do
        [ -s "$R/$f" ] || { echo "outdated: missing $R/$f" >&2; exit 2; }
    done
    tok="${GITHUB_TOKEN:-${GH_TOKEN:-}}"
    get() { curl -sSfL --retry 3 --max-time 60 -A 'cnl-ckc-outdated (https://github.com/eturkes/cnl-ckc)' "$@"; }
    latest_tag() { # owner/repo → tag of the latest GitHub release
        if [ -n "$tok" ]; then get -H "Authorization: Bearer $tok" "https://api.github.com/repos/$1/releases/latest"
        else get "https://api.github.com/repos/$1/releases/latest"; fi | jq -r '.tag_name // empty'
    }
    report() { # pin current latest
        pins=$((pins + 1))
        if [ -z "$3" ]; then errors=$((errors + 1)); printf '%s\t%s\t?\terror\n' "$1" "$2"
        elif [ "$2" = "$3" ]; then printf '%s\t%s\t%s\tok\n' "$1" "$2" "$3"
        else drift=$((drift + 1)); printf '%s\t%s\t%s\tdrift\n' "$1" "$2" "$3"; fi
    }
    lockval() { awk -F'\t' -v k="$2" '$1==k{print $2}' "$R/$1"; }
    # Verifier family = one unit keyed on the Verus release: its rustc, Z3 and
    # vstd/verus_* crates move with it, never alone.
    report verus "$(lockval verus.lock verus_release)" "$(latest_tag verus-lang/verus)"
    for k in rustc_toolchain z3_version; do printf '%s\t%s\t-\tfollows verus\n' "$k" "$(lockval verus.lock "$k")"; done
    report kani "$(lockval kani.lock kani_version)" "$(latest_tag model-checking/kani | sed 's/^kani-//')"
    while IFS=$'\t' read -r tool version asset sha url; do
        [ "$tool" = tool ] && continue
        case "$url" in
            https://github.com/*) latest=$(latest_tag "$(cut -d/ -f4-5 <<< "$url")" | sed 's/^v//') ;;
            https://static.rust-lang.org/rustup/*) latest=$(get https://static.rust-lang.org/rustup/release-stable.toml | sed -n "s/^version = '\(.*\)'$/\1/p") ;;
            *) latest= ;;
        esac
        report "tool:$tool" "$version" "$latest"
    done < "$R/tools.lock"
    while read -r name version; do
        case "$name" in
            vstd|verus_*) printf 'crate:%s\t%s\t-\tfollows verus\n' "$name" "$version"; continue ;;
        esac
        report "crate:$name" "$version" "$(get "https://crates.io/api/v1/crates/$name" | jq -r '.crate.max_stable_version // empty')"
        sleep 1 # crates.io crawler policy: at most one request per second
    done < <(awk -F' = ' '$1=="name"{n=$2} $1=="version"{v=$2} $1=="source" && $2 ~ /crates\.io-index/{print n, v}' \
        "$R/Cargo.lock" "$R/ckc-kani-harness/Cargo.lock" | tr -d '"' | sort -u)
    echo "outdated: $pins pins, $drift drift, $errors errors"
    [ "$errors" -eq 0 ] || exit 2
    [ "$drift" -eq 0 ] || exit 1

# Secondary bounded gate (Kani 0.67.0 over the kernel's public exec surface;
# harnesses stay outside the Verus workspace). Bootstrap = rust/kani.lock.
kani:
    #!/usr/bin/env bash
    set -euo pipefail
    cd "{{ ROOT }}/rust/ckc-kani-harness"
    cargo fmt --check
    cargo clippy --locked --offline --all-targets -q --target-dir "{{ ROOT }}/rust/target/kani-lint" -- -D warnings
    # Cargo reads .cargo/config.toml from cwd, not from --manifest-path; a failed
    # bootstrap must not hide behind `eval "$(...)"`.
    kani_env=$(CKC_TOOLCHAIN="{{ TOOLCHAIN }}" bash "{{ ROOT }}/rust/kani/bootstrap.sh" --env)
    eval "$kani_env"
    cargo metadata --locked --offline --format-version 1 --no-deps >/dev/null
    for harness in align_two_byte_domain reader_v1_check_small reader_v1_check_prefix_of_committed; do
        cargo-kani --manifest-path "{{ ROOT }}/rust/ckc-kani-harness/Cargo.toml" \
            --harness "harness::$harness" --exact --output-format terse \
            --target-dir "{{ ROOT }}/rust/target/kani"
    done
    echo "gate: kani ok"

# Corpus chain: `ckc check` (every custody, coverage, lexicon, fixture and
# release-manifest law), `ckc ui check`, then a fresh compile + queries per
# guideline must leave guidelines/ byte-identical. CKC_BIN names a prebuilt binary.
check:
    #!/usr/bin/env bash
    set -euo pipefail
    cd "{{ ROOT }}"
    bin="${CKC_BIN:-}"
    if [ -z "$bin" ]; then just build >/dev/null; bin="{{ ROOT }}/rust/target/release/ckc"; fi
    "$bin" check
    "$bin" ui check
    for g in guidelines/*/; do
        "$bin" compile "$(basename "$g")"
        "$bin" queries "$(basename "$g")"
    done
    git diff --quiet -- guidelines/
    echo "gate: check ok"

# Emission certification (M6, R83/R86): every committed document + query certifies
# against rust/ckc-spec/src/emit.rs through the upstream APE parser, then the
# tests/certify battery replays. CKC_BIN names a prebuilt binary (CI artifact).
certify:
    #!/usr/bin/env bash
    set -euo pipefail
    cd "{{ ROOT }}"
    bin="${CKC_BIN:-}"
    if [ -z "$bin" ]; then just build >/dev/null; bin="{{ ROOT }}/rust/target/release/ckc"; fi
    for g in guidelines/*/; do "$bin" certify "$(basename "$g")"; done
    "$bin" certify --cases tests/certify/cases.tsv
    echo "gate: certify ok"
