# Gate driver. `just gate` = rust chain + legacy chain; CI runs the same
# recipes. Every tool is sha256-pinned (rust/verus.lock, rust/tools.lock)
# and installed under .toolchain/ by `just tools`; CKC_TOOLCHAIN relocates it.

set shell := ["bash", "-euo", "pipefail", "-c"]

ROOT := justfile_directory()
TOOLCHAIN := env("CKC_TOOLCHAIN", ROOT / ".toolchain")
export RUSTUP_HOME := TOOLCHAIN / "rustup"
export CARGO_HOME := TOOLCHAIN / "cargo"
# .toolchain/bin first: the swipl + python3 shims must shadow system copies.
export PATH := (TOOLCHAIN / "bin") + ":" + (TOOLCHAIN / "verus-x86-linux") + ":" + (TOOLCHAIN / "cargo/bin") + ":" + env("PATH")

default:
    @just --list --unsorted

# Full gate: rust chain, then the legacy chain (authoritative until M5.7).
gate: rust legacy

# Rust chain: format, lint, verify, build, trust-audit, test, dependency audit, secret scan.
rust: fmt-check clippy verify build trust test deny secrets

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
    while IFS=$'\t' read -r tool version asset sha url; do
        [ "$tool" = tool ] && continue
        if [ "$tool" = rustup-init ]; then
            [ -x "$T/cargo/bin/rustup" ] && continue
            fetch "$asset" "$url" "$sha"; chmod +x "$DL/$asset"
            "$DL/$asset" -y -q --no-modify-path --default-toolchain none --profile minimal >/dev/null
            continue
        fi
        [ -x "$T/bin/$tool" ] && continue
        fetch "$asset" "$url" "$sha"
        rm -rf "$T/$tool"; mkdir -p "$T/$tool"
        case "$asset" in
            *.tar.gz) tar -xzf "$DL/$asset" -C "$T/$tool" ;;
            *.tar.xz) tar -xJf "$DL/$asset" -C "$T/$tool" ;;
            *) cp "$DL/$asset" "$T/$tool/$tool"; chmod +x "$T/$tool/$tool" ;;
        esac
        bin=$(find "$T/$tool" -type f -name "$tool" -perm -u+x | head -1)
        ln -sfn "$bin" "$T/bin/$tool"
    done < "{{ ROOT }}/rust/tools.lock"
    tc=$(lock rustc_toolchain)
    "$T/cargo/bin/rustup" -q toolchain install "$tc" --profile minimal --component clippy --component rustfmt
    if [ ! -x "$T/verus-x86-linux/verus" ]; then
        fetch "$(lock verus_asset)" "$(lock verus_asset_url)" "$(lock verus_asset_sha256)"
        unzip -q -o "$DL/$(lock verus_asset)" -d "$T"
    fi
    echo "tools: ok rustc=$tc verus=$(lock verus_release) $(awk -F'\t' 'NR>1{printf "%s=%s ", $1, $2}' "{{ ROOT }}/rust/tools.lock")"

# Reformat the workspace (verusfmt inside verus!, rustfmt outside).
fmt:
    cd "{{ ROOT }}/rust" && verusfmt --edition 2024 ckc-spec/src/*.rs ckc-kernel/src/*.rs ckc/src/*.rs ckc/tests/*.rs

fmt-check:
    cd "{{ ROOT }}/rust" && verusfmt --edition 2024 --check ckc-spec/src/*.rs ckc-kernel/src/*.rs ckc/src/*.rs ckc/tests/*.rs
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

# Legacy chain (E-- → Python identity, corpus check, fresh compile byte-stable).
legacy:
    #!/usr/bin/env bash
    set -euo pipefail
    cd "{{ ROOT }}"
    export PYTHONDONTWRITEBYTECODE=1
    PYTHONPATH=vendor/e--/src python3 -P -m e_minus_minus.strict tools/regen.emm | cmp - tools/regen.py
    python3 -P tools/regen.py --check
    python3 -P tools/goal.py check
    for g in guidelines/*/; do python3 -P tools/goal.py compile "$(basename "$g")"; done
    git diff --quiet -- guidelines/
    echo "gate: legacy ok"
