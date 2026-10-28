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

[doc("List the recipes.")]
default:
    @just --list --unsorted

RUST_STEPS := "fmt-check clippy verify build trust test deny secrets workflows shellcheck eslint"

# Full gate: rust chain, phone layout, corpus chain, emission certification.
gate: (_timed RUST_STEPS + " mobile check certify")

# Rust chain: format, lint, verify, build, trust-audit, test, dependency, secret, workflow, shell + JavaScript scans.
rust: (_timed RUST_STEPS)

# CI gate (.claude/rules/ops.md): the full gate as the ci.yml fan-out on HEAD, pushed to ci/<unit>.
# Green deletes the branch; red keeps it (fix, amend, rerun). A dirty tree is refused: CI grades HEAD alone.
[doc("Run the full gate in CI on HEAD (branch ci/<unit>); green deletes the branch.")]
ci unit:
    #!/usr/bin/env bash
    set -euo pipefail
    cd "{{ ROOT }}"
    unit="{{ unit }}"
    [[ "$unit" =~ ^[A-Za-z0-9._-]+$ ]] || { echo "ci: unit must match [A-Za-z0-9._-]+" >&2; exit 2; }
    status=$(git status --porcelain)
    [ -z "$status" ] || { echo "ci: working tree differs from HEAD (commit first)" >&2; exit 2; }
    b="ci/{{ unit }}"; sha=$(git rev-parse HEAD); tree=$(git rev-parse "HEAD^{tree}")
    git push -q -f origin "HEAD:$b"
    id=""
    for _ in $(seq 60); do
        id=$(gh run list --branch "$b" --commit "$sha" --workflow ci.yml --event push --json databaseId -q '.[0].databaseId')
        [ -n "$id" ] && break; sleep 5
    done
    [ -n "$id" ] || { echo "ci: no run for $sha on $b after 5 min" >&2; exit 2; }
    echo "ci: run $id, commit $sha, tree $tree"
    rc=0; gh run watch "$id" --exit-status -i 30 > /dev/null 2>&1 || rc=$?
    # `gh run watch` can return while GitHub still records a finished job's end as 0001-01-01
    # (gh's jq refuses `fromdate` on it): the jobs are re-read until each one holds its end, ≤ 5 min.
    for i in $(seq 31); do
        pending=$(gh run view "$id" --json jobs -q '[.jobs[] | select(.conclusion != "skipped"
            and .conclusion != "cancelled") | select(.completedAt | startswith("0001-"))] | length')
        [ "$pending" = 0 ] && break
        [ "$i" -lt 31 ] || { echo "ci: run $id: $pending jobs without an end time after 5 min" >&2; exit 2; }
        sleep 10
    done
    gh run view "$id" --json jobs -q '.jobs[] | select(.conclusion != "skipped") | "time: \(.name) \(.conclusion) \(
        if [.startedAt, .completedAt] | any(startswith("0001-")) then "-"
        else (.completedAt | fromdate) - (.startedAt | fromdate) end) s"'
    if [ "$rc" -ne 0 ]; then
        gh run view "$id" --log-failed | tail -80
        echo "ci: run $id red, branch $b kept" >&2; exit 1
    fi
    git push -q origin --delete "$b"
    echo "gate: ci ok"

# Every gate run blocks the critical path, so each step's wall-clock follows its own output, a failed
# step's included. Steps run in list order without their dependencies (trust's build precedes it).
_timed steps:
    #!/usr/bin/env bash
    set -uo pipefail
    start=$SECONDS
    steps=({{ steps }})
    for s in "${steps[@]}"; do
        t=$SECONDS
        just --justfile "{{ justfile() }}" --no-deps "$s"; rc=$?
        echo "time: $s $((SECONDS - t)) s"
        [ "$rc" -eq 0 ] || break
    done
    echo "time: total $((SECONDS - start)) s"
    exit "$rc"

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
    while IFS=$'\t' read -r tool _ asset sha url; do
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
        rm -rf "${T:?}/$tool"; mkdir -p "$T/$tool"
        case "$asset" in
            *.tar.gz) tar -xzf "$DL/$asset" -C "$T/$tool" ;;
            *.tar.xz) tar -xJf "$DL/$asset" -C "$T/$tool" ;;
            *.zip) unzip -q "$DL/$asset" -d "$T/$tool" ;;
            *) cp "$DL/$asset" "$T/$tool/$tool"; chmod +x "$T/$tool/$tool" ;;
        esac
        bin=$(find "$T/$tool" -type f -name "$tool" -perm -u+x | head -1)
        ln -sfn "$bin" "$T/bin/$tool"
        printf '%s\n' "$sha" > "$T/$tool/.pin"
    done < "{{ ROOT }}/rust/tools.lock"
    # ESLint = tools/eslint/package-lock.json (an integrity hash per package) through the pinned node's npm.
    lockv=$(sha256sum < "{{ ROOT }}/tools/eslint/package-lock.json" | cut -d' ' -f1)
    if [ ! -x "$T/eslint/node_modules/eslint/bin/eslint.js" ] || ! pinned "$T/eslint" "$lockv"; then
        rm -rf "${T:?}/eslint"; mkdir -p "$T/eslint"
        cp "{{ ROOT }}/tools/eslint/package.json" "{{ ROOT }}/tools/eslint/package-lock.json" "$T/eslint/"
        npm=$(find "$T/node" -path '*lib/node_modules/npm/bin/npm-cli.js' | head -n 1)
        (cd "$T/eslint" && "$T/bin/node" "$npm" ci --ignore-scripts --no-audit --no-fund > /dev/null)
        printf '%s\n' "$lockv" > "$T/eslint/.pin"
    fi
    tc=$(lock rustc_toolchain)
    # rustup installs from a local mirror of digest-checked files, never from the network: the channel
    # manifest (verus.lock sha256) pins each component's hash, which rustup checks; the rust_component rows
    # name the files this install needs. Pin = those rows.
    rpin=$(awk -F'\t' '$1 == "rust_manifest_sha256" || $1 == "rust_component"' "{{ ROOT }}/rust/verus.lock" | sha256sum | cut -d' ' -f1)
    if [ ! -x "$RUSTUP_HOME/toolchains/$tc/bin/rustc" ] || ! pinned "$T/rust-toolchain" "$rpin"; then
        man=$(lock rust_manifest_url); m=${man##*/}
        fetch "$m" "$man" "$(lock rust_manifest_sha256)"
        mirror=$(mktemp -d); trap 'rm -rf "$mirror"' EXIT
        mkdir -p "$mirror/dist"; ln -s "$DL/$m" "$mirror/dist/$m"
        printf '%s  %s\n' "$(lock rust_manifest_sha256)" "$m" > "$mirror/dist/$m.sha256"
        while IFS=$'\t' read -r _ url sha; do
            fetch "${url##*/}" "$url" "$sha"
            p=${url#https://static.rust-lang.org/}; mkdir -p "$mirror/${p%/*}"; ln -s "$DL/${url##*/}" "$mirror/$p"
        done < <(awk -F'\t' '$1 == "rust_component"' "{{ ROOT }}/rust/verus.lock")
        # musl target = `just build-static` (the CI container jobs' binary).
        RUSTUP_DIST_SERVER="file://$mirror" "$T/cargo/bin/rustup" -q toolchain install "$tc" --profile minimal \
            --component clippy --component rustfmt --target x86_64-unknown-linux-musl
        mkdir -p "$T/rust-toolchain"; printf '%s\n' "$rpin" > "$T/rust-toolchain/.pin"
    fi
    if [ ! -x "$T/verus-x86-linux/verus" ] || ! pinned "$T/verus-x86-linux" "$(lock verus_asset_sha256)"; then
        fetch "$(lock verus_asset)" "$(lock verus_asset_url)" "$(lock verus_asset_sha256)"
        rm -rf "$T/verus-x86-linux"
        unzip -q -o "$DL/$(lock verus_asset)" -d "$T"
        printf '%s\n' "$(lock verus_asset_sha256)" > "$T/verus-x86-linux/.pin"
    fi
    echo "tools: ok rustc=$tc verus=$(lock verus_release) $(awk -F'\t' 'NR>1{printf "%s=%s ", $1, $2}' "{{ ROOT }}/rust/tools.lock")"

# The swipl shim shadows any system copy; CI's corpus jobs run the digest-pinned swipl:10.0.2 container,
# its rust job this recipe. Shims: swipl → this prefix, python3 → uv cpython 3.15 where uv supplies it
# (scratch + archived migration scripts only); .pin = tarball sha256. 10.0 names the xpce package set GUI.
# SWI-Prolog 10.0.2: digest-verified source build into the toolchain dir (idempotent).
swipl:
    #!/usr/bin/env bash
    set -euo pipefail
    T="{{ TOOLCHAIN }}"; v=10.0.2; sha=e42cc098f7b8a6051c4f79a99b55162d467098aba60f69649bdc7583f0734b57
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
            -DSWIPL_PACKAGES_JAVA=OFF -DSWIPL_PACKAGES_GUI=OFF -DSWIPL_PACKAGES_ODBC=OFF
        step build.log ninja -C "$b/build"
        rm -rf "$P"; step install.log ninja -C "$b/build" install
        printf '%s\n' "$sha" > "$P/.pin"
    fi
    ln -sfn "../swipl-$v/bin/swipl" "$T/bin/swipl"
    if py=$(uv python find 3.15 2>/dev/null); then ln -sfn "$py" "$T/bin/python3"; fi
    # Probes run as assignments: set -e ignores a failed command substitution inside an argument.
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
    # Own pins follow upstream (user ruling): a member's `=` requirement on the crate moves with it.
    for toml in "$R/Cargo.toml" "$R"/*/Cargo.toml; do
        grep -qE '^{{ crate }} = (\{ *version = )?"=' "$toml" || continue
        sed -i -E 's/^({{ crate }} = (\{ *version = )?")=[^"]*"/\1={{ version }}"/' "$toml"
    done
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
# commit's rust-toolchain.toml channel + source/vstd version, that channel's manifest sha256 (its published
# .sha256, checked against the download) + each installed component's xz_url + xz_hash, Z3 = the bundled z3's
# version; rust-toolchain.toml, the vstd pin + kani.lock bootstrap rows follow; `just tools` installs the
# release; the vstd family relocks.
# Verus bump: repin to the stable release <tag> (e.g. release/0.2026.10.04.426d8b0); then `just rust`.
bump-verus tag:
    #!/usr/bin/env bash
    set -euo pipefail
    cd "{{ ROOT }}"
    fail() { echo "bump-verus: $*" >&2; exit 1; }
    tag="{{ tag }}"; enc=${tag//\//%2F}; api=https://api.github.com/repos/verus-lang/verus
    [[ "$tag" =~ ^release/0\.[0-9]{4}\.[0-9]{2}\.[0-9]{2}\.[0-9a-f]{7}$ ]] || fail "not a stable release tag: $tag"
    tok="${GITHUB_TOKEN:-${GH_TOKEN:-}}"
    get() { curl -sSfL --retry 3 --max-time 60 --proto '=https' -A 'cnl-ckc-bump (https://github.com/eturkes/cnl-ckc)' \
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
    # Upstream strings reach sed + the lock rows: each must match its strict shape before any write.
    [[ "$asset" =~ ^verus-[0-9a-z.]+-x86-linux\.zip$ ]] || fail "unexpected asset name: $asset"
    [[ "$sha" =~ ^[0-9a-f]{64}$ ]] || fail "unexpected asset digest: $sha"
    [[ "$commit" =~ ^[0-9a-f]{40}$ ]] || fail "unexpected tag commit: $commit"
    [[ "$channel" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]] || fail "unexpected rust-toolchain channel: $channel"
    [[ "$vstd" =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[0-9A-Za-z]+)*$ ]] || fail "unexpected vstd version: $vstd"
    tc="$channel-x86_64-unknown-linux-gnu"
    # The channel manifest pins each component's hash (rustup checks every file against it): its sha256 +
    # the xz_url + xz_hash of each component `just tools` installs = verus.lock rows.
    man_url="https://static.rust-lang.org/dist/channel-rust-$channel.toml"
    man_sha=$(get "$man_url.sha256" | cut -d' ' -f1)
    [[ "$man_sha" =~ ^[0-9a-f]{64}$ ]] || fail "unexpected manifest digest: $man_sha"
    w=$(mktemp -d); trap 'rm -rf "$w"' EXIT
    get -o "$w/manifest.toml" "$man_url"
    [ "$(sha256sum < "$w/manifest.toml" | cut -d' ' -f1)" = "$man_sha" ] || fail "manifest digest mismatch: $man_url"
    g=${tc#*-}; rows=""
    for c in "rustc $g" "rust-std $g" "rust-std x86_64-unknown-linux-musl" "cargo $g" "clippy-preview $g" "rustfmt-preview $g"; do
        row=$(awk -v s="[pkg.${c% *}.target.${c#* }]" '$0 == s {f = 1; next} /^\[/ {f = 0}
            f && /^xz_url = / {u = $3} f && /^xz_hash = / {h = $3} END {gsub(/"/, "", u); gsub(/"/, "", h); print u "\t" h}' "$w/manifest.toml")
        [[ "$row" =~ ^https://static\.rust-lang\.org/dist/[0-9]{4}-[0-9]{2}-[0-9]{2}/[A-Za-z0-9._-]+\.tar\.xz$'\t'[0-9a-f]{64}$ ]] \
            || fail "unexpected manifest entry for $c: $row"
        rows+="rust_component"$'\t'"$row"$'\n'
    done
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
    set_row rust/verus.lock rust_manifest_url "$man_url"
    set_row rust/verus.lock rust_manifest_sha256 "$man_sha"
    R="$rows" awk '$1 == "rust_component" {next} {print} $1 == "rust_manifest_sha256" {printf "%s", ENVIRON["R"]}' \
        rust/verus.lock > rust/verus.lock.new; mv rust/verus.lock.new rust/verus.lock
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
# version + its published .sha256; node → the version swapped + nodejs.org's SHASUMS256.txt;
# chrome-headless-shell (Chrome for Testing publishes no sha256) → the version swapped, the GCS
# object md5 checks the download, whose sha256 becomes the row's. The asset downloads into the
# toolchain's dl/ (the cache `just tools` reuses) and must match before the row changes.
# Tool bump: rewrite the rust/tools.lock row of <tool> for <version> (e.g. zizmor 1.31.0); then `just rust`.
bump-tool tool version:
    #!/usr/bin/env bash
    set -euo pipefail
    cd "{{ ROOT }}"
    fail() { echo "bump-tool: $*" >&2; exit 1; }
    L=rust/tools.lock t="{{ tool }}" v="{{ version }}" DL="{{ TOOLCHAIN }}/dl" sha='' md5=''
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
        https://nodejs.org/dist/*)
            url=${url//"$old"/$v} asset=${asset//"$old"/$v}
            sha=$(get "${url%/*}/SHASUMS256.txt" | awk -v a="$asset" '$2 == a {print $1}') ;;
        https://storage.googleapis.com/chrome-for-testing-public/*)
            url=${url//"$old"/$v} obj=${url#https://storage.googleapis.com/chrome-for-testing-public/}
            md5=$(get "https://storage.googleapis.com/storage/v1/b/chrome-for-testing-public/o/${obj//\//%2F}" \
                | jq -r '.md5Hash // empty' | base64 -d | od -An -tx1 | tr -d ' \n') ;;
        *) fail "no release source for $url" ;;
    esac
    if ! { [[ -n "$url" ]] && { [[ "$sha" =~ ^[0-9a-f]{64}$ ]] || [[ "$md5" =~ ^[0-9a-f]{32}$ ]]; }; }; then
        fail "no $asset with a digest for $t $v"
    fi
    mkdir -p "$DL"; w=$(mktemp -d); trap 'rm -rf "$w"' EXIT
    get -o "$w/$asset" "$url"
    got=$(sha256sum < "$w/$asset" | cut -d' ' -f1)
    if [ -n "$md5" ]; then
        [ "$(md5sum < "$w/$asset" | cut -d' ' -f1)" = "$md5" ] || fail "digest mismatch: $asset"
        sha=$got
    fi
    [ "$got" = "$sha" ] || fail "digest mismatch: $asset"
    mv "$w/$asset" "$DL/$asset"
    T="$t" V="$v" A="$asset" S="$sha" U="$url" awk -F'\t' -v OFS='\t' \
        '$1 == ENVIRON["T"] {$2 = ENVIRON["V"]; $3 = ENVIRON["A"]; $4 = ENVIRON["S"]; $5 = ENVIRON["U"]} 1' "$L" > "$w/tools.lock"
    mv "$w/tools.lock" "$L"
    just tools > /dev/null
    echo "bump-tool: $t $old → $v"

# Reformat the workspace (verusfmt inside verus!, rustfmt outside).
fmt:
    cd "{{ ROOT }}/rust" && find ckc-spec/src ckc-kernel/src ckc/src ckc/tests -name '*.rs' -print0 | sort -z | xargs -0 verusfmt --edition 2024

[doc("Check the workspace format (verusfmt inside verus!, rustfmt outside).")]
fmt-check:
    cd "{{ ROOT }}/rust" && find ckc-spec/src ckc-kernel/src ckc/src ckc/tests -name '*.rs' -print0 | sort -z | xargs -0 verusfmt --edition 2024 --check
    @echo "gate: fmt ok"

[doc("Lint every workspace target; a warning fails the step.")]
clippy:
    cd "{{ ROOT }}/rust" && cargo clippy --workspace --locked --offline --all-targets -q -- -D warnings
    @echo "gate: clippy ok"

# Verus gate: verifier canary, warm vstd plain, then re-verify every first-party crate under --no-cheating.
verify:
    #!/usr/bin/env bash
    set -euo pipefail
    cd "{{ ROOT }}/rust"
    # The pinned verifier must reject every known false proof first.
    bash "{{ ROOT }}/rust/verus-canary/run.sh" "{{ TOOLCHAIN }}/verus-x86-linux/verus"
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

[doc("Build the release ckc program.")]
build:
    cd "{{ ROOT }}/rust" && cargo build --release --locked --offline -q
    @echo "gate: build ok"

# The swipl:10.0.2 image (Debian glibc 2.36) cannot load the runner's glibc-2.43 build.
# Static musl ckc = the binary of the CI container jobs + the reviewer program of a release.
# Panic locations name the vendored crates by absolute path; remapping the checkout path keeps
# it out of the binary, and the pinned toolchain's rust-lld links it (the host `cc` adds its
# own build-id + layout), so every checkout of a commit builds the same bytes on any host.
[doc("Build the static ckc program for the CI containers and the release.")]
build-static:
    #!/usr/bin/env bash
    set -euo pipefail
    cd "{{ ROOT }}/rust"
    sysroot=$(rustc --print sysroot)
    CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER="$sysroot/lib/rustlib/x86_64-unknown-linux-gnu/bin/rust-lld" \
    CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_RUSTFLAGS="--remap-path-prefix={{ ROOT }}=cnl-ckc -C linker-flavor=ld.lld" \
        cargo build --release --locked --offline -q -p ckc --target x86_64-unknown-linux-musl
    bin=target/x86_64-unknown-linux-musl/release/ckc
    # Each inspection is a checked assignment: inside an `if`, a failed readelf (or a
    # grep -q that quits early, SIGPIPE under pipefail) reads as "no match" = green.
    prog=$(readelf -lW "$bin"); dyn=$(readelf -dW "$bin")
    if grep -q INTERP <<< "$prog" || grep -q NEEDED <<< "$dyn"; then
        echo "build-static: $bin is dynamically linked" >&2; exit 1
    fi
    rc=0; grep -qF "{{ ROOT }}" "$bin" || rc=$?
    case $rc in
        0) echo "build-static: $bin holds the checkout path {{ ROOT }}" >&2; exit 1 ;;
        1) ;;
        *) echo "build-static: cannot search $bin for the checkout path (grep rc $rc)" >&2; exit 1 ;;
    esac
    echo "gate: build-static ok"

# Reviewer program of a release (docs/REFERENCE.md § Releasing): the static ckc as
# <dest>/ckc-x86_64-linux + its sha256sum sidecar; prints the sidecar line.
[doc("Write the release reviewer program and its sha256sum sidecar to <dest>.")]
release-ckc dest="dist": build-static
    #!/usr/bin/env bash
    set -euo pipefail
    cd "{{ ROOT }}"
    mkdir -p "{{ dest }}"
    install -m 0755 rust/target/x86_64-unknown-linux-musl/release/ckc "{{ dest }}/ckc-x86_64-linux"
    cd "{{ dest }}" && sha256sum ckc-x86_64-linux | tee ckc-x86_64-linux.sha256

[doc("Run the trust audit: escape tokens, dependency set, trusted-surface pins.")]
trust: build
    cd "{{ ROOT }}/rust" && ./target/release/ckc trust-audit

# Summary lines only: totals + the 5 slowest test binaries; a failing run also prints each failed
# binary, each binary that started without a result line, its panics + failure lists and cargo's
# own error lines: a binary killed by a signal or a harness error prints no panic and no `test
# result` line, so cargo's lines hold its cause. Every binary runs (`--no-fail-fast`), so one run
# reports every failing binary. Every run leaves its log in rust/target/test.log.
[doc("Run every test binary and report each failing one.")]
test:
    #!/usr/bin/env bash
    set -uo pipefail
    cd "{{ ROOT }}/rust" || exit 2
    mkdir -p target; log=target/test.log
    # cargo's status lines stay on, uncolored: each `Running` | `Doc-tests` line names the binary
    # whose `test result` line follows; libtest stays terse.
    CARGO_TERM_COLOR=never cargo test --workspace --locked --offline --no-fail-fast -- --quiet > "$log" 2>&1; rc=$?
    awk '/^ +Running /{n=$NF; gsub(/[()]/, "", n); sub(/.*\//, "", n); sub(/-[0-9a-f]+$/, "", n); print "start", n}
        /^ +Doc-tests /{n="doc:" $2; print "start", n}
        /^test result/{t=$NF; sub(/s$/, "", t); printf "%s %s %s %s %s\n", t, n, $4, $6, $8}' "$log" > "$log.t"
    grep -v '^start ' "$log.t" | awk '{b++; p+=$3; f+=$4; i+=$5} END {printf "test: %d binaries, %d passed, %d failed, %d ignored\n", b, p, f, i}'
    grep -v '^start ' "$log.t" | sort -rn | head -5 | awk '{printf "test: slowest %s s %s (%d tests)\n", $1, $2, $3 + $4 + $5}'
    if [ "$rc" -ne 0 ]; then
        awk '$1 != "start" && $4 > 0 {print "test: failed: " $2 " (" $4 " failed)"}' "$log.t"
        awk '$1 == "start" {if (n != "" && !d) print "test: no result line: " n; n = $2; d = 0; next} {d = 1}
            END {if (n != "" && !d) print "test: no result line: " n}' "$log.t"
        grep -E -B2 -A12 'panicked|^failures:' "$log" | head -200
        grep -E "^error|process didn't exit successfully" "$log" | head -40
        echo "test: rc $rc, log kept at rust/$log"
    fi
    rm -f "$log.t"
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

# Workflow scan: zizmor's auditor persona over .github/ (offline audits; online audits too when GH_TOKEN is set).
workflows:
    cd "{{ ROOT }}" && if [ -n "${GH_TOKEN:-}" ]; then zizmor --no-progress --color never --persona auditor .github; else zizmor --no-progress --color never --persona auditor --offline .github; fi
    @echo "gate: workflows ok"

# Shell scan: ShellCheck over every recipe body (tools/shell-recipes.awk renders `just --dump`; an
# interpolation renders as one word) + the repository's shell scripts; its own red input runs every
# time: an unquoted expansion must be flagged.
[doc("Check the recipe bodies and the shell scripts with ShellCheck.")]
shellcheck:
    #!/usr/bin/env bash
    set -euo pipefail
    cd "{{ ROOT }}"
    w=$(mktemp -d); trap 'rm -rf "$w"' EXIT
    mkdir "$w/recipes"
    just --justfile "{{ justfile() }}" --dump | awk -v DIR="$w/recipes" -f tools/shell-recipes.awk
    shellcheck "$w"/recipes/*.sh rust/kani/bootstrap.sh rust/verus-canary/run.sh .agent/spec-lint
    # shellcheck disable=SC2016 # the red input's `$x` stays literal on purpose
    printf '#!/usr/bin/env bash\nx="a b"\nrm -- $x\n' > "$w/red.sh"
    if shellcheck "$w/red.sh" > /dev/null; then echo "shellcheck: the unquoted red input passed" >&2; exit 2; fi
    echo "gate: shellcheck ok"

# JavaScript scan: ESLint (dynamic-code + DOM-sink rules, tools/eslint/eslint.config.mjs) over the
# review page's script (rust/ckc-spec/src/ui.rs script_html) + the browser probes, inside a scratch
# dir beside the pinned node_modules; its own red input runs every time: an innerHTML sink.
[doc("Check the review page script and the browser probes with ESLint.")]
eslint:
    #!/usr/bin/env bash
    set -euo pipefail
    cd "{{ ROOT }}"
    w=$(mktemp -d); trap 'rm -rf "$w"' EXIT
    ln -s "{{ TOOLCHAIN }}/eslint/node_modules" "$w/node_modules"
    cp tools/eslint/eslint.config.mjs tools/*.mjs "$w/"
    awk '/^pub open spec fn script_html/ {f = 1} f && /^<\/script>"##@/ {exit} f && p {print} f && /r##"<script>$/ {p = 1}' \
        rust/ckc-spec/src/ui.rs > "$w/page.js"
    [ -s "$w/page.js" ] || { echo "eslint: no page script in rust/ckc-spec/src/ui.rs" >&2; exit 2; }
    lint() { (cd "$w" && "{{ TOOLCHAIN }}/bin/node" node_modules/eslint/bin/eslint.js --max-warnings 0 "$@"); }
    lint page.js ./*.mjs
    printf 'document.body.innerHTML = location.hash;\n' > "$w/red.js"
    if lint red.js > /dev/null 2>&1; then echo "eslint: the innerHTML red input passed" >&2; exit 2; fi
    echo "gate: eslint ok"

# Image scan (weekly, network): Trivy over the CI container image digest; a vulnerability with a
# fixed version at HIGH or CRITICAL fails it unless .trivyignore.yaml lists it (a reason + an
# expiry 30 days out; Trivy drops an expired entry, so the scan fails again). Red inputs first:
# alpine 3.9.6 by digest (known CVEs); the ignore list with every entry expired, which must
# report each listed CVE (an entry whose CVE no longer reaches the image = stale).
[doc("Scan the pinned CI container image with Trivy (network).")]
image-scan:
    #!/usr/bin/env bash
    set -euo pipefail
    cd "{{ ROOT }}"
    img=$(grep -oE 'docker\.io/library/swipl:[0-9.]+@sha256:[0-9a-f]{64}' .github/workflows/ci.yml | sort -u)
    [ "$(printf '%s\n' "$img" | wc -l)" = 1 ] || { echo "image-scan: expected one container image in ci.yml" >&2; exit 2; }
    w=$(mktemp -d); trap 'rm -rf "$w"' EXIT
    scan() { trivy image --quiet --cache-dir "$w" --image-src remote --scanners vuln --severity HIGH,CRITICAL --ignore-unfixed --exit-code 1 "$@"; }
    red=docker.io/library/alpine@sha256:65b3a80ebe7471beecbc090c5b2cdd0aafeaefa0715f8f12e40dc918a3a70e32
    if scan --format json --output "$w/red.json" "$red" 2> /dev/null || ! grep -q '"VulnerabilityID"' "$w/red.json"; then
        echo "image-scan: the known-vulnerable red input passed" >&2; exit 2
    fi
    ids=$(awk '$1 == "-" && $2 == "id:" {print $3}' .trivyignore.yaml)
    if [ -n "$ids" ]; then
        sed 's/^\( *expired_at:\).*/\1 2000-01-01/' .trivyignore.yaml > "$w/expired.yaml"
        if scan --ignorefile "$w/expired.yaml" --format json --output "$w/expired.json" "$img" 2> /dev/null; then
            echo "image-scan: the expired ignore list passed" >&2; exit 2
        fi
        for id in $ids; do
            grep -q "\"VulnerabilityID\": \"$id\"" "$w/expired.json" || { echo "image-scan: $id: its expired entry reports nothing (stale entry)" >&2; exit 2; }
        done
    fi
    scan --ignorefile .trivyignore.yaml "$img"
    echo "gate: image-scan ok"

# Kani installer advisories (weekly, network): cargo-deny over the Cargo.lock of the kani-verifier
# crate rust/kani.lock pins (outside the workspace's `just deny`). Red input first: a lock holding
# smallvec 0.6.13 (RUSTSEC-2021-0003).
[doc("Check the Kani installer crate's locked dependencies against RustSec (network).")]
kani-advisories:
    #!/usr/bin/env bash
    set -euo pipefail
    cd "{{ ROOT }}"
    row() { awk -F'\t' -v k="$1" '$1 == k {print $2}' rust/kani.lock; }
    # Outside rust/ no rust-toolchain.toml applies, and `just tools` installs rustup without a default.
    RUSTUP_TOOLCHAIN=$(awk -F'\t' '$1 == "rustc_toolchain" {print $2}' rust/verus.lock); export RUSTUP_TOOLCHAIN
    w=$(mktemp -d); trap 'rm -rf "$w"' EXIT
    curl -sSfL --retry 3 --proto '=https' -o "$w/installer.crate" "$(row installer_url)"
    printf '%s  %s\n' "$(row installer_sha256)" "$w/installer.crate" | sha256sum -c --quiet
    tar -xzf "$w/installer.crate" -C "$w"
    src=$(find "$w" -maxdepth 1 -type d -name 'kani-verifier-*')
    printf '%s  %s\n' "$(row installer_cargo_lock_sha256)" "$src/Cargo.lock" | sha256sum -c --quiet
    deny() { cargo-deny --manifest-path "$1/Cargo.toml" check advisories; }
    mkdir -p "$w/red/src"; : > "$w/red/src/lib.rs"
    printf '[package]\nname = "red"\nversion = "0.1.0"\nedition = "2021"\n\n[dependencies]\nsmallvec = "=0.6.13"\n' > "$w/red/Cargo.toml"
    (cd "$w/red" && cargo generate-lockfile -q)
    if deny "$w/red" > "$w/red.log" 2>&1 || ! grep -q RUSTSEC-2021-0003 "$w/red.log"; then
        echo "kani-advisories: the vulnerable red input passed" >&2; exit 2
    fi
    deny "$src"
    echo "gate: kani-advisories ok"

# Prints `pin current latest status [detail]`; rc 1 = drift (bump procedure: .claude/rules/rust.md),
# rc 2 = a failed or empty upstream answer or a failed dry run; CKC_OUTDATED_ROOT = tree holding rust/ (locks + manifests).
# held = a crate with a newer upstream version that cargo's resolver cannot select under the
# locked graph's requirements; `by` names its direct dependents in the locks.
# drift `pinned by` = a crate that only members' own `=` requirements hold (own pins follow upstream).
# Pin drift: every lock pin vs upstream latest (crates.io, GitHub releases, rustup).
outdated:
    #!/usr/bin/env bash
    set -uo pipefail
    O="{{ env('CKC_OUTDATED_ROOT', ROOT) }}"; R="$O/rust"
    pins=0 drift=0 held=0 errors=0
    for f in rust/Cargo.toml rust/Cargo.lock rust/ckc-kani-harness/Cargo.toml rust/ckc-kani-harness/Cargo.lock \
        rust/tools.lock rust/verus.lock rust/kani.lock justfile .github/workflows/ci.yml; do
        [ -s "$O/$f" ] || { echo "outdated: missing $O/$f" >&2; exit 2; }
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
    # SWI-Prolog (user ruling: drift tracked) = the swipl recipe's version vs swi-prolog.org's latest
    # stable (the 303 that `swipl-latest.tar.gz` answers), and the CI container's `version@digest` (one
    # value across its jobs) vs Docker Hub's digest of that latest version's tag.
    sv=$(grep -oE 'v=[0-9.]+; sha=[0-9a-f]{64}' "$O/justfile" | head -n 1 | sed 's/^v=//; s/;.*//')
    sl=$(get -I https://www.swi-prolog.org/download/stable/src/swipl-latest.tar.gz | tr -d '\r' \
        | sed -n 's|^[Ll]ocation: .*/swipl-\([0-9][0-9.]*\)\.tar\.gz$|\1|p' | tail -n 1) || sl=
    report swipl "${sv:-?}" "${sv:+$sl}"
    img=$(grep -oE 'docker\.io/library/swipl:[0-9.]+@sha256:[0-9a-f]{64}' "$O/.github/workflows/ci.yml" \
        | sed 's|^docker\.io/library/swipl:||' | sort -u | paste -sd' ' -)
    hub=
    if [ -n "$sl" ] && [[ "$img" =~ ^[0-9.]+@sha256:[0-9a-f]{64}$ ]]; then
        hub=$(get "https://hub.docker.com/v2/repositories/library/swipl/tags/$sl" | jq -r '.digest // empty') || hub=
    fi
    report swipl-image "${img:-?}" "${hub:+$sl@$hub}"
    while IFS=$'\t' read -r tool version _ _ url; do
        [ "$tool" = tool ] && continue
        case "$url" in
            https://github.com/*) latest=$(latest_tag "$(cut -d/ -f4-5 <<< "$url")" | sed 's/^v//') ;;
            https://static.rust-lang.org/rustup/*) latest=$(get https://static.rust-lang.org/rustup/release-stable.toml | sed -n "s/^version = '\(.*\)'$/\1/p") || latest= ;;
            https://nodejs.org/dist/*) latest=$(get https://nodejs.org/dist/index.json | jq -r '[.[].version | ltrimstr("v") | split(".") | map(tonumber)] | max | select(. != null) | map(tostring) | join(".")') || latest= ;;
            https://storage.googleapis.com/chrome-for-testing-public/*) latest=$(get https://googlechromelabs.github.io/chrome-for-testing/last-known-good-versions.json | jq -r '.channels.Stable.version // empty') || latest= ;;
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
            errors=$((errors + 1)); printf 'cargo-update:%s\t-\t?\terror\n' "$m"
            # shellcheck disable=SC2001 # a per-line prefix has no parameter-expansion form
            sed 's/^/  /' <<< "$out" >&2
        fi
    done
    # Requirements the workspace members declare (`member version crate req`, tab-separated, from
    # cargo metadata without resolution): own `=` pins follow upstream (user ruling), so a crate
    # held only by members pinning it with `=` reports drift.
    reqs=
    for m in Cargo.toml ckc-kani-harness/Cargo.toml; do
        if out=$(cd / && CARGO_HOME="$tmp/cargo" RUSTUP_TOOLCHAIN="$(lockval verus.lock rustc_toolchain)" \
            cargo metadata --format-version 1 --no-deps --offline --manifest-path "$R/$m" 2> "$tmp/err") &&
            rows=$(jq -r '.packages[] | .name as $p | .version as $v | .dependencies[] | [$p, $v, .name, .req] | @tsv' <<< "$out"); then
            reqs+=$rows$'\n'
        else
            errors=$((errors + 1)); printf 'cargo-metadata:%s\t-\t?\terror\n' "$m"; sed 's/^/  /' "$tmp/err" >&2
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
        # Own pin = every direct dependent is a member whose requirement on the crate starts `=`.
        pin=$(awk -F'\t' -v n="$name" -v v="$version" '
            FILENAME == ARGV[1] { if ($4 ~ /^=/) req[$1 " " $2 " " $3] = $4; next }
            { split($0, e, " ") }
            e[1] == n && e[2] == v { d++; r = req[e[3] " " e[4] " " n]; if (r != "") { k++; s = s (s != "" ? ", " : "") e[3] " " e[4] " (" r ")" } }
            END { if (d && k == d) print s }' <(printf '%s' "$reqs") <(printf '%s\n' "$edges"))
        if [ -n "$fix" ]; then report "crate:$name" "$version" "$latest" drift "$fix"
        elif [ -n "$pin" ]; then report "crate:$name" "$version" "$latest" drift "pinned by $pin"
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

# Verifier canary: the Verus build rejects every known false proof + verifies every control
# (default = the pinned one, which `just verify` runs first).
[doc("Check that a Verus build rejects every known false proof (default: the pinned build).")]
canary verus=(TOOLCHAIN / "verus-x86-linux/verus"):
    bash "{{ ROOT }}/rust/verus-canary/run.sh" "{{ verus }}"

# Secondary bounded gate (Kani 0.68.0 over the kernel's public exec surface;
# harnesses stay outside the Verus workspace). Bootstrap = rust/kani.lock.
[doc("Run the Kani harnesses over the kernel's public exec surface.")]
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
[doc("Replay the late ckc check mutants (one full check each).")]
late-mutants:
    #!/usr/bin/env bash
    set -uo pipefail
    cd "{{ ROOT }}" || exit 2
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
# files it imports or calls (CLAUDE.md `Engineering`). In place inside a disposable worktree
# of HEAD: copy mode would copy rust/ alone, while suites read repo fixtures outside it. Target
# words reach every cargo phase (the build phase otherwise compiles every test target);
# --no-config keeps the default operators; cargo metadata ignores --cargo-arg, so CARGO_NET_OFFLINE
# keeps it offline. `verus!` bodies yield no mutants (cargo-mutants skips macro calls). Positional
# arguments keep each file glob one argument: `{{ files }}` would re-split it at its spaces.
# Mutation run: <target> = cargo target selection [`-- <test-binary args>`], <files> rust/-relative;
# results → rust/target/mutants/<HEAD>/mutants.out/; rc = cargo-mutants' (2 = missed mutants).
[doc("Run cargo-mutants over <files> with the <target> tests.")]
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

# Phone layout (n22): every `ckc ui render` page of HEAD + of the superseded-statement fixture
# tree tests/sup4-ui/mobile (contract sup4 D7) loads at 390 px in the pinned headless browser
# (tools/ui-mobile-probe.mjs under the pinned node). The recipe's own red inputs run every time:
# the render with its viewport meta stripped must fail on every page, with its narrow-screen
# table rule stripped on at least one. CKC_BIN names a prebuilt binary.
[doc("Check the phone layout and the reading words of the rendered pages.")]
mobile:
    #!/usr/bin/env bash
    set -euo pipefail
    cd "{{ ROOT }}"
    bin="${CKC_BIN:-}"
    if [ -z "$bin" ]; then just build >/dev/null; bin="{{ ROOT }}/rust/target/release/ckc"; fi
    w=$(mktemp -d); trap 'rm -rf "$w"' EXIT
    "{{ TOOLCHAIN }}/bin/node" tools/cdp-pipe-test.mjs
    probe() { "{{ TOOLCHAIN }}/bin/node" tools/ui-mobile-probe.mjs "{{ TOOLCHAIN }}/bin/chrome-headless-shell" "$1"; }
    "$bin" ui render "$w/pages" >/dev/null
    probe "$w/pages"
    tree() { # copy + materialize a fixture tree (`.pl.in` → `.pl`, fresh review manifest)
        cp -a "$1" "$w/$2"
        while IFS= read -r -d '' input; do mv "$input" "${input%.in}"; done < <(find "$w/$2" -name '*.pl.in' -print0)
        "$bin" derive-review-manifest "$w/$2/guidelines/alpha" > "$w/$2/guidelines/alpha/audit/review-manifest.tsv"
    }
    tree tests/sup4-ui/mobile/tree superseded-tree
    "$bin" ui render "$w/superseded-pages" "$w/superseded-tree" >/dev/null
    probe "$w/superseded-pages"
    n=$(find "$w/pages" -name '*.html' | wc -l)
    red() { # name pages anchor min-bad: the anchor sits on every page; stripped, ≥ min-bad pages fail (rc 1)
        local m; m=$(find "$2" -name '*.html' | wc -l)
        cp -a "$2" "$w/$1"
        [ "$(grep -rlF -- "$3" "$w/$1" | wc -l)" = "$m" ] || { echo "mobile: $1: anchor missing from a page" >&2; exit 2; }
        find "$w/$1" -name '*.html' -exec sed -i "s|$3||" {} +
        ! grep -rqF -- "$3" "$w/$1" || { echo "mobile: $1: anchor left on a page" >&2; exit 2; }
        rc=0; out=$(probe "$w/$1") || rc=$?
        bad=$(sed -n 's/^ui-mobile: [0-9]* pages, \([0-9]*\) bad.*/\1/p' <<< "$out")
        if ! { [ "$rc" = 1 ] && [ "${bad:-0}" -ge "$4" ]; }; then
            printf '%s\n' "$out" >&2; echo "mobile: $1: rc $rc, $bad bad pages (≥ $4 expected)" >&2; exit 1
        fi
        echo "mobile: red input $1: $bad of $m pages bad"
    }
    red no-viewport-meta "$w/pages" '<meta name="viewport" content="width=device-width, initial-scale=1">' "$n"
    red no-narrow-table-rule "$w/pages" '@media screen and (max-width: 40rem) { table { display: block; overflow-x: auto; } }' 1
    # The superseded tree's z page cites a 120-character run without a space (DQ10).
    red no-superseded-wrap "$w/superseded-pages" 'section.superseded p { overflow-wrap: anywhere; }' 1
    tree tests/ur-ui/browser/tree reading-tree
    "$bin" ui render "$w/reading-pages" "$w/reading-tree" >/dev/null
    reading() { "{{ TOOLCHAIN }}/bin/node" tools/ui-reading-probe.mjs "{{ TOOLCHAIN }}/bin/chrome-headless-shell" "$1"; }
    reading "$w/reading-pages"
    "{{ TOOLCHAIN }}/bin/node" tools/ui-reading-probe.mjs --strip-handler "$w/reading-pages" "$w/no-reading-handler"
    rc=0; out=$(reading "$w/no-reading-handler") || rc=$?
    bad=$(sed -n 's/^ui-reading: [0-9]* transitions, \([0-9]*\) failures$/\1/p' <<< "$out")
    if ! { [ "$rc" = 1 ] && [ "${bad:-0}" -gt 0 ] && grep -qF 'visible reading' <<< "$out"; }; then
        printf '%s\n' "$out" >&2
        echo "mobile: no-reading-handler: rc $rc, $bad failed transitions (rc 1 + reading failures expected)" >&2; exit 1
    fi
    echo "mobile: red input no-reading-handler: $bad failed transitions"
    echo "gate: mobile ok"

# Corpus chain: `.agent/spec-lint` (spec.md shape, the template's byte copy),
# `ckc check` (every custody, coverage, lexicon, fixture and release-manifest law),
# `ckc ui check`, then a fresh compile + queries per guideline must leave
# guidelines/ byte-identical. CKC_BIN names a prebuilt binary.
[doc("Run the corpus chain: spec shape, ckc check, ui check, fresh compile + queries.")]
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
[doc("Certify every committed document and question against the emission spec.")]
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
