#!/usr/bin/env bash
# Pinned downloads only; installer build + Kani setup use local inputs.
# --env emits Bash exports on stdout; diagnostics and success go to stderr.
set -euo pipefail

fail() { printf 'bootstrap-kani: %s\n' "$*" >&2; exit 1; }
mode=${1:-}
[[ $# -le 1 && ( -z "$mode" || "$mode" == --env ) ]] || fail 'usage: bootstrap.sh [--env]'
PROJECT=$(CDPATH='' cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." && pwd -P)
LOCK="$PROJECT/rust/kani.lock"
BASE=${CKC_TOOLCHAIN:-$PROJECT/.toolchain}
mkdir -p "$BASE/kani"
PREFIX=$(CDPATH='' cd -- "$BASE/kani" && pwd -P)
ASSETS="$PREFIX/dl"; STATE="$PREFIX/state"; LOGS="$PREFIX/logs"
mkdir -p "$ASSETS" "$STATE" "$LOGS"

required=(
    kani_version kani_tag kani_commit host
    bootstrap_rustc_toolchain bootstrap_rustc_version
    installer_asset installer_url installer_sha256 installer_cargo_lock_sha256
    bundle_asset bundle_url bundle_sha256 rust_channel rustc_version
    rust_manifest_asset rust_manifest_url rust_manifest_sha256
    rustc_component_asset rustc_component_url rustc_component_sha256
    rust_std_component_asset rust_std_component_url rust_std_component_sha256
    cargo_component_asset cargo_component_url cargo_component_sha256
    cbmc_version kissat_version
)
declare -A allowed=() value=() vendor_seen=()
vendor_names=(); vendor_versions=(); vendor_hashes=()
for key in "${required[@]}"; do allowed["$key"]=1; done
[[ -f "$LOCK" ]] || fail "missing lock: $LOCK"
line=0
while IFS= read -r entry || [[ -n "$entry" ]]; do
    ((line += 1))
    [[ -z "$entry" || "$entry" == \#* ]] && continue
    [[ "$entry" != $'\t'* && "$entry" != *$'\t' && "$entry" != *$'\t\t'* ]] \
        || fail "empty field at lock line $line"
    IFS=$'\t' read -r -a fields <<< "$entry"
    key=${fields[0]}
    if [[ "$key" == vendor_crate ]]; then
        [[ ${#fields[@]} -eq 4 ]] || fail "vendor_crate line $line needs four fields"
        name=${fields[1]}; version=${fields[2]}; hash=${fields[3]}
        [[ "$name" =~ ^[A-Za-z0-9_-]+$ && "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+([+-][A-Za-z0-9.-]+)?$ ]] \
            || fail "invalid crate identity at lock line $line"
        [[ "$hash" =~ ^[0-9a-f]{64}$ ]] || fail "invalid crate SHA-256 at lock line $line"
        [[ -z "${vendor_seen[$name-$version]+x}" ]] || fail "duplicate crate: $name-$version"
        vendor_seen["$name-$version"]=1
        vendor_names+=("$name"); vendor_versions+=("$version"); vendor_hashes+=("$hash")
    else
        [[ ${#fields[@]} -eq 2 ]] || fail "lock line $line needs two fields"
        [[ -n "${allowed[$key]+x}" ]] || fail "unknown lock key: $key"
        [[ -z "${value[$key]+x}" ]] || fail "duplicate lock key: $key"
        value["$key"]=${fields[1]}
    fi
done < "$LOCK"
for key in "${required[@]}"; do
    [[ -n "${value[$key]+x}" ]] || fail "missing lock key: $key"
    case "$key" in
        *_sha256) [[ "${value[$key]}" =~ ^[0-9a-f]{64}$ ]] || fail "invalid SHA-256: $key" ;;
        *_url) [[ "${value[$key]}" == https://* && ! "${value[$key]}" =~ [[:space:]] ]] || fail "non-HTTPS URL: $key" ;;
        *_asset) [[ "${value[$key]}" =~ ^[A-Za-z0-9][A-Za-z0-9._+-]+$ ]] || fail "invalid asset basename: $key" ;;
    esac
done
[[ ${#vendor_names[@]} -gt 0 ]] || fail 'missing vendor_crate rows'
[[ "${value[host]}" == x86_64-unknown-linux-gnu && "$(uname -m)" == x86_64 && "$(uname -s)" == Linux ]] \
    || fail 'supported host: x86_64-unknown-linux-gnu'

mapfile -t rustc_tools < <(awk -F'\t' '$1=="rustc_toolchain" {print $2}' "$PROJECT/rust/verus.lock")
[[ ${#rustc_tools[@]} -eq 1 && "${rustc_tools[0]}" == "${value[bootstrap_rustc_toolchain]}" ]] \
    || fail 'bootstrap rustc pin differs from verus.lock'
# Preserve the read-only source when --env replaces RUSTUP_HOME with Kani's isolated home.
export CKC_KANI_BOOTSTRAP_RUSTUP_HOME=${CKC_KANI_BOOTSTRAP_RUSTUP_HOME:-${RUSTUP_HOME:-$BASE/rustup}}
BOOTSTRAP_TOOLCHAIN="$CKC_KANI_BOOTSTRAP_RUSTUP_HOME/toolchains/${rustc_tools[0]}"
BOOTSTRAP_TOOLCHAIN=$(readlink -f -- "$BOOTSTRAP_TOOLCHAIN") || fail 'bootstrap toolchain missing; run just tools'
BOOTSTRAP_CARGO="$BOOTSTRAP_TOOLCHAIN/bin/cargo"
BOOTSTRAP_RUSTC="$BOOTSTRAP_TOOLCHAIN/bin/rustc"
[[ -x "$BOOTSTRAP_CARGO" && -x "$BOOTSTRAP_RUSTC" ]] || fail 'bootstrap toolchain missing; run just tools'
[[ "$("$BOOTSTRAP_RUSTC" --version)" == "${value[bootstrap_rustc_version]}" ]] || fail 'bootstrap rustc version mismatch'

verify() {
    local actual
    actual=$(sha256sum -- "$1"); actual=${actual%% *}
    [[ "$actual" == "$2" ]] || fail "digest mismatch: $1"
}
download_tmp=
trap 'if [[ -n "$download_tmp" ]]; then rm -f -- "$download_tmp"; fi' EXIT
fetch() {
    local asset=$1 url=$2 hash=$3
    if [[ ! -f "$ASSETS/$asset" ]]; then
        download_tmp="$ASSETS/.$asset.$$"
        printf 'kani: fetch %s\n' "$asset" >&2
        curl -sSL --fail --retry 3 --proto '=https' --proto-redir '=https' -o "$download_tmp" "$url"
        verify "$download_tmp" "$hash"
        mv -- "$download_tmp" "$ASSETS/$asset"
        download_tmp=
    fi
    # Cached inputs are rechecked on every run, including unchanged installs.
    verify "$ASSETS/$asset" "$hash"
}
for kind in installer bundle rust_manifest rustc_component rust_std_component cargo_component; do
    fetch "${value[${kind}_asset]}" "${value[${kind}_url]}" "${value[${kind}_sha256]}"
done
for i in "${!vendor_names[@]}"; do
    name=${vendor_names[$i]}; version=${vendor_versions[$i]}
    fetch "$name-$version.crate" "https://static.crates.io/crates/$name/$name-$version.crate" "${vendor_hashes[$i]}"
done
lock_hash=$(sha256sum -- "$LOCK"); lock_hash=${lock_hash%% *}
installed_hash=
[[ ! -f "$PREFIX/lock.sha256" ]] || installed_hash=$(< "$PREFIX/lock.sha256")

TOOLCHAIN="$PREFIX/toolchain"
if [[ "$installed_hash" != "$lock_hash" || ! -x "$TOOLCHAIN/bin/rustc" ]] \
    || [[ "$("$TOOLCHAIN/bin/rustc" --version 2>/dev/null || true)" != "${value[rustc_version]}" ]]; then
    extract="$STATE/toolchain-extract"; candidate="$STATE/toolchain-candidate"
    rm -rf -- "$extract" "$candidate"
    mkdir -p "$extract" "$candidate"
    : > "$LOGS/toolchain-install.log"
    for component in rustc rust_std cargo; do
        case "$component" in
            rustc) asset=${value[rustc_component_asset]}; root="rustc-nightly-${value[host]}" ;;
            rust_std) asset=${value[rust_std_component_asset]}; root="rust-std-nightly-${value[host]}" ;;
            cargo) asset=${value[cargo_component_asset]}; root="cargo-nightly-${value[host]}" ;;
        esac
        tar -xJf "$ASSETS/$asset" -C "$extract"
        sh "$extract/$root/install.sh" --prefix="$candidate" --disable-ldconfig >> "$LOGS/toolchain-install.log" 2>&1
    done
    [[ "$("$candidate/bin/rustc" --version)" == "${value[rustc_version]}" ]] || fail 'reconstructed rustc version mismatch'
    rm -rf -- "$TOOLCHAIN"
    mv -- "$candidate" "$TOOLCHAIN"
    rm -rf -- "$extract"
fi

WRAPPER="$PREFIX/wrapper"
if [[ "$installed_hash" != "$lock_hash" || ! -x "$WRAPPER/bin/cargo-kani" ]] \
    || [[ "$("$WRAPPER/bin/cargo-kani" --version 2>/dev/null || true)" != "cargo-kani ${value[kani_version]}" ]]; then
    build="$STATE/wrapper-build"; cargo_home="$STATE/build-cargo-home"; target="$STATE/wrapper-target"
    rm -rf -- "$build" "$cargo_home" "$target" "$WRAPPER"
    mkdir -p "$build/src" "$build/vendor" "$cargo_home" "$target"
    tar -xzf "$ASSETS/${value[installer_asset]}" -C "$build/src"
    source_dir="$build/src/kani-verifier-${value[kani_version]}"
    verify "$source_dir/Cargo.lock" "${value[installer_cargo_lock_sha256]}"
    for i in "${!vendor_names[@]}"; do
        crate="${vendor_names[$i]}-${vendor_versions[$i]}"
        tar -xzf "$ASSETS/$crate.crate" -C "$build/vendor"
        printf '{"files":{},"package":"%s"}\n' "${vendor_hashes[$i]}" > "$build/vendor/$crate/.cargo-checksum.json"
    done
    printf '%s\n' '[source.crates-io]' 'replace-with = "vendored-sources"' \
        '[source.vendored-sources]' "directory = \"$build/vendor\"" \
        '[net]' 'offline = true' > "$cargo_home/config.toml"
    CARGO_HOME="$cargo_home" CARGO_TARGET_DIR="$target" CARGO_NET_OFFLINE=true \
        RUSTC="$BOOTSTRAP_RUSTC" RUSTDOC="$BOOTSTRAP_TOOLCHAIN/bin/rustdoc" \
        "$BOOTSTRAP_CARGO" install --path "$source_dir" --locked --offline \
        --root "$WRAPPER" > "$LOGS/wrapper-install.log" 2>&1
    rm -rf -- "$build" "$cargo_home" "$target"
fi
[[ "$("$WRAPPER/bin/cargo-kani" --version)" == "cargo-kani ${value[kani_version]}" ]] || fail 'cargo-kani version mismatch'

export CARGO_HOME="$STATE/runtime-cargo-home" RUSTUP_HOME="$STATE/runtime-rustup-home"
export KANI_HOME="$PREFIX/kani-home" CARGO_NET_OFFLINE=true
export PATH="$WRAPPER/bin:$TOOLCHAIN/bin:$PATH"
mkdir -p "$CARGO_HOME" "$RUSTUP_HOME"
install="$KANI_HOME/kani-${value[kani_version]}"
if [[ "$installed_hash" != "$lock_hash" || ! -x "$install/bin/kani-driver" || ! -L "$install/toolchain" ]] \
    || [[ "$(readlink -f -- "$install/toolchain" 2>/dev/null || true)" != "$TOOLCHAIN" ]] \
    || [[ "$("$install/bin/kani-driver" --version 2>/dev/null || true)" != "kani ${value[kani_version]}" ]]; then
    rm -rf -- "$install"
    cargo-kani setup --use-local-bundle "$ASSETS/${value[bundle_asset]}" \
        --use-local-toolchain "$TOOLCHAIN" > "$LOGS/setup.log" 2>&1
fi
[[ "$(cargo-kani --version)" == "cargo-kani ${value[kani_version]}" ]] || fail 'final cargo-kani version mismatch'
[[ "$(kani --version)" == "kani ${value[kani_version]}" ]] || fail 'final kani version mismatch'
[[ "$("$install/bin/cbmc" --version)" == "${value[cbmc_version]}"* ]] || fail 'CBMC version mismatch'
[[ "$("$install/bin/kissat" --version)" == "${value[kissat_version]}" ]] || fail 'Kissat version mismatch'
printf '%s\n' "$lock_hash" > "$PREFIX/lock.sha256"
if [[ "$mode" == --env ]]; then
    for key in CKC_KANI_BOOTSTRAP_RUSTUP_HOME CARGO_HOME RUSTUP_HOME KANI_HOME CARGO_NET_OFFLINE PATH; do
        printf 'export %s=%q\n' "$key" "${!key}"
    done
    printf 'kani: %s ok\n' "${value[kani_version]}" >&2
else
    printf 'kani: %s ok\n' "${value[kani_version]}"
fi
