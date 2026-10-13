# Gate driver. `just gate` = rust chain + corpus chain + emission certification;
# CI runs the same recipes. Every tool is sha256-pinned (rust/verus.lock,
# rust/tools.lock, the swipl recipe) and installed under .toolchain/ by
# `just tools` + `just swipl`; CKC_TOOLCHAIN relocates it.

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

# The system swipl is too new; CI runs the digest-pinned swipl:9.2.9 container. Shims: swipl → this
# prefix, python3 → uv cpython 3.13 (scratch + archived migration scripts only); .pin = tarball sha256.
# Dev-workstation SWI-Prolog 9.2.9: digest-verified source build into the toolchain dir (idempotent).
swipl:
    #!/usr/bin/env bash
    set -euo pipefail
    T="{{ TOOLCHAIN }}"; v=9.2.9; sha=53f428e2d9bbdf30e53b06c9c42def9a13ff82fc36a111d410fc8b0bc889ee2d
    P="$T/swipl-$v"; tgz="$T/dl/swipl-$v.tar.gz"; mkdir -p "$T/dl" "$T/bin"
    if [ ! -x "$P/bin/swipl" ] || [ "$(cat "$P/.pin" 2>/dev/null)" != "$sha" ]; then
        printf '%s  %s\n' "$sha" "$tgz" | sha256sum -c --quiet >/dev/null 2>&1 \
            || curl -sSL --fail --retry 3 -o "$tgz" "https://www.swi-prolog.org/download/stable/src/swipl-$v.tar.gz"
        printf '%s  %s\n' "$sha" "$tgz" | sha256sum -c --quiet
        b=$(mktemp -d "$T/swipl-build.XXXXXX"); trap 'rm -rf "$b"' EXIT
        tar -xzf "$tgz" -C "$b"
        step() { log=$1; shift; "$@" > "$b/$log" 2>&1 || { tail -n 30 "$b/$log" >&2; exit 1; }; }
        step cmake.log cmake -S "$b/swipl-$v" -B "$b/build" -G Ninja -DCMAKE_BUILD_TYPE=Release \
            -DCMAKE_INSTALL_PREFIX="$P" -DINSTALL_DOCUMENTATION=OFF \
            -DSWIPL_PACKAGES_JAVA=OFF -DSWIPL_PACKAGES_X=OFF -DSWIPL_PACKAGES_ODBC=OFF
        step build.log ninja -C "$b/build"
        rm -rf "$P"; step install.log ninja -C "$b/build" install
        printf '%s\n' "$sha" > "$P/.pin"
    fi
    # Probes run as assignments: set -e ignores a failed command substitution inside an argument.
    py=$(uv python find 3.13)
    ln -sfn "../swipl-$v/bin/swipl" "$T/bin/swipl"
    ln -sfn "$py" "$T/bin/python3"
    ver=$("$T/bin/swipl" --version)
    echo "swipl: ok $ver"

# Crate bump: <crate> at <version> in every Cargo.lock holding it + rust/vendor/ + trust re-pin; then `just rust`.
bump-crate crate version: (_relock crate version)

# Cargo runs from cwd `/` (as in `outdated`), so the vendored-source and offline configs stay out
# without edits, under the pinned rustc whose cargo produced rust/vendor/ (one tree for both
# workspaces). Escape rows come from member sources, which a relock leaves alone. CKC_BIN = prebuilt binary.
# Relock <crate> at <version>, revendor, re-pin trust (deps-allowlist + spec-manifest).
_relock crate version:
    #!/usr/bin/env bash
    set -euo pipefail
    R="{{ ROOT }}/rust"
    tc=$(awk -F'\t' '$1 == "rustc_toolchain" {print $2}' "$R/verus.lock")
    [ -n "$tc" ] || { echo "bump: no rustc_toolchain in rust/verus.lock" >&2; exit 2; }
    export RUSTUP_TOOLCHAIN=$tc
    n=0
    for ws in "$R" "$R/ckc-kani-harness"; do
        grep -qx 'name = "{{ crate }}"' "$ws/Cargo.lock" || continue
        (cd / && cargo update -q --manifest-path "$ws/Cargo.toml" -p "{{ crate }}" --precise "{{ version }}")
        n=$((n + 1))
    done
    [ "$n" -gt 0 ] || { echo "bump: {{ crate }} is in no Cargo.lock" >&2; exit 2; }
    (cd / && cargo vendor -q --locked --manifest-path "$R/Cargo.toml" --sync "$R/ckc-kani-harness/Cargo.toml" "$R/vendor" > /dev/null)
    escape=$(sha256sum < "$R/trust/escape-allowlist.tsv")
    cd "{{ ROOT }}"
    bin="${CKC_BIN:-}"
    if [ -z "$bin" ]; then just build > /dev/null; bin="$R/target/release/ckc"; fi
    "$bin" trust-audit --write rust
    [ "$(sha256sum < "$R/trust/escape-allowlist.tsv")" = "$escape" ] || { echo "bump: escape-allowlist changed" >&2; exit 1; }

# A pin = a stable `release/<version>` tag: the Monday cron rebuilds the rolling tag into one, and every
# push to verus main deletes the rolling assets. Rows: GitHub release asset name + `digest` + URL, the tag
# commit's rust-toolchain.toml channel + source/vstd version, Z3 = the bundled z3's version; rust-toolchain.toml,
# the vstd pin + kani.lock bootstrap rows follow; `just tools` installs the release; the vstd family relocks.
# Verus bump: repin to the stable release <tag> (e.g. release/0.2026.10.04.426d8b0); then `just rust`.
bump-verus tag:
    #!/usr/bin/env bash
    set -euo pipefail
    cd "{{ ROOT }}"
    fail() { echo "bump-verus: $*" >&2; exit 1; }
    tag="{{ tag }}"; enc=${tag//\//%2F}; api=https://api.github.com/repos/verus-lang/verus
    [[ "$tag" =~ ^release/0\.[0-9]{4}\.[0-9]{2}\.[0-9]{2}\.[0-9a-f]{7}$ ]] || fail "not a stable release tag: $tag"
    tok="${GITHUB_TOKEN:-${GH_TOKEN:-}}"
    get() { curl -sSfL --retry 3 --max-time 60 -A 'cnl-ckc-bump (https://github.com/eturkes/cnl-ckc)' \
        ${tok:+-H "Authorization: Bearer $tok"} "$@"; }
    rel=$(get "$api/releases/tags/$enc")
    [ "$(jq -r .prerelease <<< "$rel")" = false ] || fail "$tag is a prerelease"
    asset=$(jq -r '[.assets[] | select(.name | endswith("-x86-linux.zip"))][0].name // empty' <<< "$rel")
    sha=$(jq -r --arg a "$asset" '.assets[] | select(.name == $a) | .digest // empty' <<< "$rel" | sed -n 's/^sha256://p')
    ref=$(get "$api/git/ref/tags/$tag" | jq -r '.object.type + " " + .object.sha')
    case "$ref" in "tag "*) commit=$(get "$api/git/tags/${ref#tag }" | jq -r .object.sha) ;; *) commit=${ref#commit } ;; esac
    file() { get "$api/contents/$1?ref=$commit" | jq -r .content | base64 -d; }
    channel=$(file rust-toolchain.toml | sed -n 's/^channel = "\(.*\)"$/\1/p')
    vstd=$(file source/vstd/Cargo.toml | sed -n '0,/^version = /s/^version = "\(.*\)"$/\1/p')
    for x in asset sha commit channel vstd; do [ -n "${!x}" ] || fail "no $x for $tag"; done
    tc="$channel-x86_64-unknown-linux-gnu"
    set_row() { # lock key value: replace the value of an existing key row
        [ -n "$3" ] || fail "empty $2"
        grep -q "^$2"$'\t' "$1" || fail "$1 has no $2 row"
        K="$2" V="$3" awk -F'\t' -v OFS='\t' '$1 == ENVIRON["K"] {$2 = ENVIRON["V"]} 1' "$1" > "$1.new"; mv "$1.new" "$1"
    }
    set_row rust/verus.lock verus_release "$tag"
    set_row rust/verus.lock verus_asset "$asset"
    set_row rust/verus.lock verus_asset_sha256 "$sha"
    set_row rust/verus.lock verus_asset_url "https://github.com/verus-lang/verus/releases/download/$enc/$asset"
    set_row rust/verus.lock rustc_toolchain "$tc"
    sed -i "s/^channel = \".*\"$/channel = \"$channel\"/" rust/rust-toolchain.toml
    sed -i "s/^vstd = \"=.*\"$/vstd = \"=$vstd\"/" rust/Cargo.toml
    just tools > /dev/null
    # Probes run as assignments: set -e ignores a failed command substitution inside an argument.
    z3=$("{{ TOOLCHAIN }}/verus-x86-linux/z3" --version | awk '{print $3}')
    brv=$("$RUSTUP_HOME/toolchains/$tc/bin/rustc" --version)
    set_row rust/verus.lock z3_version "$z3"
    set_row rust/kani.lock bootstrap_rustc_toolchain "$tc"
    set_row rust/kani.lock bootstrap_rustc_version "$brv"
    just _relock vstd "$vstd"
    echo "bump-verus: $tag commit=${commit:0:10} rustc=$channel vstd=$vstd"

# kani.lock rows: tag commit (GitHub ref API), bundle = release asset `digest`, installer crate checksum
# (crates.io) + its Cargo.lock sha256 + one vendor_crate row per registry package (lock order), the
# nightly its rust-toolchain.toml names (channel manifest sha256 = the published .sha256; component
# xz_hash; rustc version), CBMC + Kissat = the bundle binaries' --version, bootstrap rows = the
# verus.lock rustc as installed. Downloads land in the toolchain's kani/dl, which bootstrap.sh reuses.
# Kani bump: rewrite rust/kani.lock for Kani <version> (e.g. 0.68.0); then `just kani` + the Q1 trigger rerun.
bump-kani version:
    #!/usr/bin/env bash
    set -euo pipefail
    fail() { echo "bump-kani: $*" >&2; exit 1; }
    v="{{ version }}"; tag="kani-$v"; L="{{ ROOT }}/rust/kani.lock"; DL="{{ TOOLCHAIN }}/kani/dl"; host=x86_64-unknown-linux-gnu
    mkdir -p "$DL"; w=$(mktemp -d); trap 'rm -rf "$w"' EXIT
    tok="${GITHUB_TOKEN:-${GH_TOKEN:-}}"; api=https://api.github.com/repos/model-checking/kani
    get() { curl -sSfL --retry 3 --max-time 600 --proto '=https' -A 'cnl-ckc-bump (https://github.com/eturkes/cnl-ckc)' "$@"; }
    hub() { get ${tok:+-H "Authorization: Bearer $tok"} "$api/$1"; }
    fetch() { # asset url sha256 → $DL/asset, digest-verified
        [ -f "$DL/$1" ] && [ "$(sha256sum < "$DL/$1" | cut -d' ' -f1)" = "$3" ] && return
        get -o "$w/$1" "$2"; [ "$(sha256sum < "$w/$1" | cut -d' ' -f1)" = "$3" ] || fail "digest mismatch: $1"
        mv "$w/$1" "$DL/$1"
    }
    ref=$(hub "git/ref/tags/$tag" | jq -r '.object.type + " " + .object.sha')
    case "$ref" in "tag "*) commit=$(hub "git/tags/${ref#tag }" | jq -r .object.sha) ;; *) commit=${ref#commit } ;; esac
    bundle="kani-$v-$host.tar.gz"; rel=$(hub "releases/tags/$tag")
    bundle_url=$(jq -r --arg a "$bundle" '.assets[] | select(.name == $a) | .browser_download_url' <<< "$rel")
    bundle_sha=$(jq -r --arg a "$bundle" '.assets[] | select(.name == $a) | .digest // empty' <<< "$rel" | sed -n 's/^sha256://p')
    inst="kani-verifier-$v.crate"; inst_url="https://static.crates.io/crates/kani-verifier/$inst"
    inst_sha=$(get "https://crates.io/api/v1/crates/kani-verifier/$v" | jq -r '.version.checksum // empty')
    for x in commit bundle_url bundle_sha inst_sha; do [ -n "${!x}" ] || fail "no $x for $tag"; done
    fetch "$inst" "$inst_url" "$inst_sha"
    tar -xzf "$DL/$inst" -C "$w"; src="$w/kani-verifier-$v"
    channel=$(sed -n 's/^channel = "\(.*\)"$/\1/p' "$src/rust-toolchain.toml")
    man_url="https://static.rust-lang.org/dist/${channel#nightly-}/channel-rust-nightly.toml"
    man_sha=$(get "$man_url.sha256" | cut -d' ' -f1)
    fetch channel-rust-nightly.toml "$man_url" "$man_sha"
    section() { # TOML table header → its `key = "value"` lines as key<TAB>value
        awk -v s="[$1]" '$0 == s {f = 1; next} /^\[/ {f = 0} f && / = "/ {k = $1; sub(/^[^"]*"/, ""); sub(/"$/, ""); print k "\t" $0}' \
            "$DL/channel-rust-nightly.toml"
    }
    fetch "$bundle" "$bundle_url" "$bundle_sha"
    tar -xzf "$DL/$bundle" -C "$w" "kani-$v/bin/cbmc" "kani-$v/bin/kissat"
    # Probes run as assignments: set -e ignores a failed command substitution inside an argument.
    cbmc=$("$w/kani-$v/bin/cbmc" --version | awk '{print $1}')
    kissat=$("$w/kani-$v/bin/kissat" --version)
    rustc_v=$(section pkg.rustc | awk -F'\t' '$1 == "version" {print $2}')
    lock_sha=$(sha256sum < "$src/Cargo.lock" | cut -d' ' -f1)
    tc=$(awk -F'\t' '$1 == "rustc_toolchain" {print $2}' "{{ ROOT }}/rust/verus.lock")
    brv=$("$RUSTUP_HOME/toolchains/$tc/bin/rustc" --version) || fail "rustc $tc missing: run just tools"
    for x in cbmc kissat rustc_v tc; do [ -n "${!x}" ] || fail "no $x for $tag"; done
    {
        sed -n '/^#/p' "$L"
        printf '%s\t%s\n' kani_version "$v" kani_tag "$tag" kani_commit "$commit" host "$host" \
            bootstrap_rustc_toolchain "$tc" bootstrap_rustc_version "$brv" \
            installer_asset "$inst" installer_url "$inst_url" installer_sha256 "$inst_sha" installer_cargo_lock_sha256 "$lock_sha" \
            bundle_asset "$bundle" bundle_url "$bundle_url" bundle_sha256 "$bundle_sha" \
            rust_channel "$channel" rustc_version "rustc $rustc_v" \
            rust_manifest_asset channel-rust-nightly.toml rust_manifest_url "$man_url" rust_manifest_sha256 "$man_sha"
        for c in rustc rust-std cargo; do
            u=$(section "pkg.$c.target.$host" | awk -F'\t' '$1 == "xz_url" {print $2}')
            h=$(section "pkg.$c.target.$host" | awk -F'\t' '$1 == "xz_hash" {print $2}')
            printf '%s\t%s\n' "${c//-/_}_component_asset" "${u##*/}" "${c//-/_}_component_url" "$u" "${c//-/_}_component_sha256" "$h"
        done
        printf '%s\t%s\n' cbmc_version "$cbmc" kissat_version "$kissat"
        awk '/^\[\[package\]\]/ {n = v = ""} /^name = / {n = $3} /^version = / {v = $3}
            /^checksum = / {s = $3; gsub(/"/, "", n); gsub(/"/, "", v); gsub(/"/, "", s); print "vendor_crate\t" n "\t" v "\t" s}' "$src/Cargo.lock"
    } > "$w/kani.lock"
    awk -F'\t' '!/^#/ && ($2 == "" || $NF == "")' "$w/kani.lock" | grep -q . && fail "empty field in generated lock"
    mv "$w/kani.lock" "$L"
    echo "bump-kani: $tag commit=${commit:0:10} $channel"

# tools.lock row from the release: GitHub → the row's tag + asset name with the version swapped,
# sha256 = the release asset `digest`, URL = its download URL; rustup-init → the archive URL for the
# version + its published .sha256. The asset downloads into the toolchain's dl/ (the cache `just tools`
# reuses) and must match before the row changes.
# Tool bump: rewrite the rust/tools.lock row of <tool> for <version> (e.g. zizmor 1.31.0); then `just rust`.
bump-tool tool version:
    #!/usr/bin/env bash
    set -euo pipefail
    cd "{{ ROOT }}"
    fail() { echo "bump-tool: $*" >&2; exit 1; }
    L=rust/tools.lock t="{{ tool }}" v="{{ version }}" DL="{{ TOOLCHAIN }}/dl"
    row=$(awk -F'\t' -v t="$t" 'NR > 1 && $1 == t' "$L")
    [ -n "$row" ] || fail "no $t row in $L"
    IFS=$'\t' read -r _ old asset _ url <<< "$row"
    tok="${GITHUB_TOKEN:-${GH_TOKEN:-}}"
    get() { curl -sSfL --retry 3 --max-time 300 --proto '=https' -A 'cnl-ckc-bump (https://github.com/eturkes/cnl-ckc)' "$@"; }
    case "$url" in
        https://github.com/*/releases/download/*)
            repo=$(cut -d/ -f4-5 <<< "$url") tag=$(cut -d/ -f8 <<< "$url")
            [[ "$tag" == *"$old" ]] || fail "tag $tag does not end in $old"
            tag=${tag%"$old"}$v asset=${asset//"$old"/$v}
            rel=$(get ${tok:+-H "Authorization: Bearer $tok"} "https://api.github.com/repos/$repo/releases/tags/$tag")
            sha=$(jq -r --arg a "$asset" '.assets[] | select(.name == $a) | .digest // empty' <<< "$rel" | sed -n 's/^sha256://p')
            url=$(jq -r --arg a "$asset" '.assets[] | select(.name == $a) | .browser_download_url // empty' <<< "$rel") ;;
        https://static.rust-lang.org/rustup/archive/*)
            url=${url/"/archive/$old/"/"/archive/$v/"}
            sha=$(get "$url.sha256" | cut -d' ' -f1) ;;
        *) fail "no release source for $url" ;;
    esac
    [[ "$sha" =~ ^[0-9a-f]{64}$ && -n "$url" ]] || fail "no $asset with a sha256 for $t $v"
    mkdir -p "$DL"; w=$(mktemp -d); trap 'rm -rf "$w"' EXIT
    get -o "$w/$asset" "$url"
    got=$(sha256sum < "$w/$asset" | cut -d' ' -f1)
    [ "$got" = "$sha" ] || fail "digest mismatch: $asset"
    mv "$w/$asset" "$DL/$asset"
    T="$t" V="$v" A="$asset" S="$sha" U="$url" awk -F'\t' -v OFS='\t' \
        '$1 == ENVIRON["T"] {$2 = ENVIRON["V"]; $3 = ENVIRON["A"]; $4 = ENVIRON["S"]; $5 = ENVIRON["U"]} 1' "$L" > "$w/tools.lock"
    mv "$w/tools.lock" "$L"
    just tools > /dev/null
    echo "bump-tool: $t $old → $v"

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
    # Verus-pass lints reach no other gate (clippy builds with ghost code
    # erased), so any first-party warning line fails here; color stays off so
    # each diagnostic line starts with its `warning` prefix.
    log="{{ ROOT }}/rust/target/verify.log"
    mkdir -p "${log%/*}"
    CARGO_TERM_COLOR=never cargo verus verify --workspace --locked --offline -- --no-cheating 2>&1 | tee "$log"
    n=$(grep -c '^warning' "$log") || [ $? -eq 1 ]
    if [ "$n" != 0 ]; then echo "verify: $n warning lines" >&2; exit 1; fi
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

# Summary lines only; a failing run also prints its panics + failure lists.
test:
    #!/usr/bin/env bash
    set -uo pipefail
    cd "{{ ROOT }}/rust"
    log=$(mktemp)
    cargo test --workspace --locked --offline -q > "$log" 2>&1; rc=$?
    grep -E '^test result' "$log" | sort | uniq -c
    if [ "$rc" -ne 0 ]; then grep -E -B2 -A12 'panicked|^failures:' "$log" | head -200; fi
    rm -f "$log"
    [ "$rc" -eq 0 ] && echo "gate: test ok"
    exit "$rc"

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

# Prints `pin current latest status [detail]`; rc 1 = drift (bump procedure: .claude/rules/rust.md),
# rc 2 = a failed or empty upstream answer or a failed dry run; CKC_OUTDATED_ROOT = tree holding rust/ (locks + manifests).
# held = a crate with a newer upstream version that cargo's resolver cannot select under the
# locked graph's requirements; `by` names its direct dependents in the locks.
# Pin drift: every lock pin vs upstream latest (crates.io, GitHub releases, rustup).
outdated:
    #!/usr/bin/env bash
    set -uo pipefail
    R="{{ env('CKC_OUTDATED_ROOT', ROOT) }}/rust"
    pins=0 drift=0 held=0 errors=0
    for f in Cargo.toml Cargo.lock ckc-kani-harness/Cargo.toml ckc-kani-harness/Cargo.lock tools.lock verus.lock kani.lock; do
        [ -s "$R/$f" ] || { echo "outdated: missing $R/$f" >&2; exit 2; }
    done
    tok="${GITHUB_TOKEN:-${GH_TOKEN:-}}"
    get() { curl -sSfL --retry 3 --max-time 60 -A 'cnl-ckc-outdated (https://github.com/eturkes/cnl-ckc)' "$@"; }
    latest_tag() { # owner/repo → tag of the latest GitHub release; nothing on a failed request
        local tag
        tag=$(if [ -n "$tok" ]; then get -H "Authorization: Bearer $tok" "https://api.github.com/repos/$1/releases/latest"
            else get "https://api.github.com/repos/$1/releases/latest"; fi | jq -r '.tag_name // empty') && printf '%s\n' "$tag"
    }
    tab=$'\t'
    report() { # pin current latest [status detail]
        pins=$((pins + 1))
        local latest=$3 status=${4:-drift} detail=${5:-}
        if [ -z "$latest" ]; then errors=$((errors + 1)) latest='?' status=error detail=
        elif [ "$2" = "$latest" ]; then status=ok detail=
        elif [ "$status" = held ]; then held=$((held + 1))
        else drift=$((drift + 1)); fi
        printf '%s\t%s\t%s\t%s%s\n' "$1" "$2" "$latest" "$status" "${detail:+$tab$detail}"
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
            https://static.rust-lang.org/rustup/*) latest=$(get https://static.rust-lang.org/rustup/release-stable.toml | sed -n "s/^version = '\(.*\)'$/\1/p") || latest= ;;
            *) latest= ;;
        esac
        report "tool:$tool" "$version" "$latest"
    done < "$R/tools.lock"
    # Fixable = what `cargo update` would move: a dry run per workspace from cwd `/` with an
    # empty CARGO_HOME, so neither the vendored-source config nor a user config applies.
    tmp=$(mktemp -d); trap 'rm -rf "$tmp"' EXIT
    fixable=
    for m in Cargo.toml ckc-kani-harness/Cargo.toml; do
        if out=$(cd / && CARGO_HOME="$tmp/cargo" CARGO_TERM_COLOR=never RUSTUP_TOOLCHAIN="$(lockval verus.lock rustc_toolchain)" \
            cargo update --dry-run --manifest-path "$R/$m" 2>&1); then
            # A row cargo would change: `name version to <v>` (Updating, Downgrading) or
            # `name version removed` (Removing = consolidated into another locked version).
            fixable+=$(sed -n -e 's/^ *\(Updating\|Downgrading\) \([^ ]*\) v\([^ ]*\) -> v\([^ ]*\).*/\2 \3 to \4/p' \
                -e 's/^ *Removing \([^ ]*\) v\([^ ]*\).*/\1 \2 removed/p' <<< "$out")$'\n'
        else
            errors=$((errors + 1)); printf 'cargo-update:%s\t-\t?\terror\n' "$m"; sed 's/^/  /' <<< "$out" >&2
        fi
    done
    # Direct dependents per locked crate: `name version dependent`; an unqualified entry
    # names the one version of that crate in its own lock (pass 1 per lock).
    edges=$(for lock in "$R/Cargo.lock" "$R/ckc-kani-harness/Cargo.lock"; do
        awk '
            /^\[\[package\]\]/ { n = ""; f = 0; next }
            /^name = / { n = $3; gsub(/"/, "", n); next }
            /^version = / { if (n != "") { v = $3; gsub(/"/, "", v); if (FNR == NR) only[n] = v }; next }
            FNR == NR { next }
            /^dependencies = \[/ { f = 1; next }
            f && /^\]/ { f = 0; next }
            f { d = $0; gsub(/^[ \t]*"|",?[ \t]*$/, "", d); split(d, a, " "); print a[1], (a[2] != "" ? a[2] : only[a[1]]), n " " v }
        ' "$lock" "$lock"
    done | sort -u)
    while read -r name version; do
        case "$name" in
            vstd|verus_*) printf 'crate:%s\t%s\t-\tfollows verus\n' "$name" "$version"; continue ;;
        esac
        latest=$(get "https://crates.io/api/v1/crates/$name" | jq -r '.crate.max_stable_version // empty') || latest=
        fix=$(awk -v n="$name" -v v="$version" '$1==n && $2==v {$1 = $2 = ""; sub(/^ +/, ""); print; exit}' <<< "$fixable")
        if [ -n "$fix" ]; then report "crate:$name" "$version" "$latest" drift "$fix"
        else
            by=$(awk -v n="$name" -v v="$version" '$1==n && $2==v {print $3 " " $4}' <<< "$edges" | paste -sd, - | sed 's/,/, /g')
            report "crate:$name" "$version" "$latest" held "by ${by:-the workspace}"
        fi
        sleep 1 # crates.io crawler policy: at most one request per second
    done < <(awk -F' = ' '$1=="name"{n=$2} $1=="version"{v=$2} $1=="source" && $2 ~ /crates\.io-index/{print n, v}' \
        "$R/Cargo.lock" "$R/ckc-kani-harness/Cargo.lock" | tr -d '"' | sort -u)
    echo "outdated: $pins pins, $drift drift, $held held, $errors errors"
    [ "$errors" -eq 0 ] || exit 2
    [ "$drift" -eq 0 ] || exit 1

# Secondary bounded gate (Kani 0.68.0 over the kernel's public exec surface;
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

# Late `ckc check` mutants (secondary gate; one full check each, so outside `just test`):
# sections after the SWI-Prolog stage. Each tests/check-mutants/late.tsv row mutates a
# fresh HEAD clone and must end on its pinned first violation (stderr when nonempty,
# else the last stdout line). CKC_BIN names a prebuilt binary.
late-mutants:
    #!/usr/bin/env bash
    set -uo pipefail
    cd "{{ ROOT }}"
    bin="${CKC_BIN:-}"
    if [ -z "$bin" ]; then just build >/dev/null; bin="{{ ROOT }}/rust/target/release/ckc"; fi
    bin=$(readlink -f "$bin")
    head=$(git rev-parse HEAD)
    table=tests/check-mutants/late.tsv
    [ "$(awk -F'\t' 'NR > 1 && NF == 2' "$table")" ] || { echo "late-mutants: no rows in $table" >&2; exit 2; }
    work=$(mktemp -d)
    trap 'rm -rf "$work"' EXIT
    fails=0 ran=0
    while IFS=$'\t' read -r name rc; do
        [ "$name" = name ] && continue
        tree="$work/$name"
        git clone -q --shared --no-checkout "{{ ROOT }}" "$tree"
        git -C "$tree" checkout -q --detach "$head"
        case "$name" in
            nonfinite-anchor) sed -i 's/result(bogus)/result(other)/' \
                "$tree/tests/queries/green/trace-direct-rejects/trace-reject/result-shape.answers" ;;
            queries-fixture-pin) printf '\n' >> "$tree/tests/queries/red/bad-qid/expect" ;;
            red-expect-altered) printf '\n' >> "$tree/tests/red/ape_messages--definite-no-antecedent.expect" ;;
            red-ace-compiles) sed -i 's/^The doctor waits\.$/A doctor waits./' \
                "$tree/tests/red/ape_messages--definite-no-antecedent.ace" ;;
            *) echo "late-mutants: unknown row $name" >&2; exit 2 ;;
        esac
        [ -z "$(git -C "$tree" status --porcelain)" ] && { echo "late-mutants: $name mutation missed" >&2; exit 2; }
        (cd "$tree" && "$bin" check > "$work/$name.out" 2> "$work/$name.err"); got=$?
        if [ -s "$work/$name.err" ]; then first=$(cat "$work/$name.err"); else first=$(tail -n 1 "$work/$name.out"); fi
        ran=$((ran + 1))
        if [ "$got" = "$rc" ] && [ "$first" = "$(cat "tests/check-mutants/late/$name.expect")" ]; then
            echo "late-mutants: $name ok"
        else
            echo "late-mutants: $name rc $got first violation: $first" >&2
            fails=$((fails + 1))
        fi
    done < "$table"
    [ "$ran" -eq "$(awk 'NR > 1' "$table" | wc -l)" ] || { echo "late-mutants: ran $ran rows" >&2; exit 2; }
    [ "$fails" -eq 0 ] || exit 1
    echo "gate: late-mutants ok"

# A test that pins already-passing behavior earns credit once its run kills the mutants of the
# files it imports or calls (global CLAUDE.md `Engineering`). In place inside a disposable worktree
# of HEAD: copy mode would copy rust/ alone, while suites read repo fixtures outside it. Target
# words reach every cargo phase (the build phase otherwise compiles every test target);
# --no-config keeps the default operators; cargo metadata ignores --cargo-arg, so CARGO_NET_OFFLINE
# keeps it offline. `verus!` bodies yield no mutants (cargo-mutants skips macro calls). Positional
# arguments keep each file glob one argument: `{{ files }}` would re-split it at its spaces.
# Mutation run: <target> = cargo target selection [`-- <test-binary args>`], <files> rust/-relative;
# results → rust/target/mutants/<HEAD>/mutants.out/; rc = cargo-mutants' (2 = missed mutants).
[positional-arguments]
mutants target +files:
    #!/usr/bin/env bash
    set -euo pipefail
    sel=$1 filter=; shift
    cd "{{ ROOT }}"
    out="{{ ROOT }}/rust/target/mutants/$(git rev-parse --short HEAD)"
    git worktree remove --force "$out/tree" 2>/dev/null || true
    rm -rf "$out"; git worktree prune
    git worktree add -q --detach "$out/tree" HEAD
    trap 'git -C "{{ ROOT }}" worktree remove --force "$out/tree"' EXIT
    set -f # target words split unexpanded
    if [[ "$sel" == *" -- "* ]]; then filter=${sel#* -- } sel=${sel%% -- *}; fi
    args=()
    for a in $sel; do args+=("--cargo-arg=$a"); done
    for f in "$@"; do args+=(--file "$f"); done
    cd "$out/tree/rust"
    CARGO_NET_OFFLINE=true cargo mutants --in-place --no-config --cargo-arg=--locked --cargo-arg=--offline \
        "${args[@]}" --output "$out" ${filter:+-- -- $filter}

# Corpus chain: `.agent/spec-lint` (spec.md shape, the template's byte copy),
# `ckc check` (every custody, coverage, lexicon, fixture and release-manifest law),
# `ckc ui check`, then a fresh compile + queries per guideline must leave
# guidelines/ byte-identical. CKC_BIN names a prebuilt binary.
check:
    #!/usr/bin/env bash
    set -euo pipefail
    cd "{{ ROOT }}"
    .agent/spec-lint
    echo "spec-lint: ok"
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

# `ckc release-manifest` digests HEAD's release inputs (guidelines/, the fork, clex, NOTICE, the
# REFERENCE schema section) without a commit id, so a commit touching them carries its own manifest.
# Release manifest: refresh it from HEAD, amend it into HEAD when it changed. CKC_BIN = prebuilt binary.
amend-manifest:
    #!/usr/bin/env bash
    set -euo pipefail
    cd "{{ ROOT }}"
    # `git diff HEAD` reads the working file alone, and the pathspec amend below would
    # replace a staged manifest edit: the working file and the index must both match HEAD.
    { git diff --quiet HEAD -- release-manifest.tsv && git diff --quiet --cached -- release-manifest.tsv; } \
        || { echo "amend-manifest: release-manifest.tsv differs from HEAD (working file or index); commit or restore it first" >&2; exit 2; }
    bin="${CKC_BIN:-}"
    if [ -z "$bin" ]; then just build >/dev/null; bin="{{ ROOT }}/rust/target/release/ckc"; fi
    "$bin" release-manifest
    if git diff --quiet -- release-manifest.tsv; then echo "amend-manifest: current"; exit 0; fi
    # The pathspec amends the manifest alone; other staged changes stay staged.
    git commit --amend --no-edit -q -- release-manifest.tsv
    echo "amend-manifest: amended $(git rev-parse --short HEAD)"
