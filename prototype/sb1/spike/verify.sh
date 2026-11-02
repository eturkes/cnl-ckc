#!/usr/bin/env bash
set -euo pipefail
HERE=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
ROOT=$(cd -- "$HERE/../../.." && pwd)
T=${CKC_TOOLCHAIN:-$ROOT/.toolchain}
export RUSTUP_HOME="$T/rustup" CARGO_HOME="$T/cargo"
export PATH="$T/verus-x86-linux:$T/cargo/bin:$T/bin:$PATH"
VERUS="$T/verus-x86-linux/verus"
expected=$(awk -F '\t' '$1 == "verus_release" {sub("release/", "", $2); print $2}' "$ROOT/rust/verus.lock")
[[ "$expected" =~ ^0\.[0-9]{4}\.[0-9]{2}\.[0-9]{2}\.[a-f0-9]+$ ]] || { printf 'spike: invalid verifier pin\n' >&2; exit 2; }
version=$("$VERUS" --version)
actual=$(printf '%s\n' "$version" | awk '/^  Version: / {print $2}')
[[ "$actual" == "$expected" ]] || { printf 'spike: verifier pin mismatch\n' >&2; exit 2; }
OUT=${SPIKE_OUT:-$HERE/target}
mkdir -p -- "$OUT"
rc=0
"$VERUS" --crate-name sb1_spike --crate-type lib "$HERE/lib.rs" \
    --no-cheating --rlimit 10 --num-threads 1 --time --output-json --triggers-mode silent \
    > "$OUT/verify.json" 2> "$OUT/verify.err" || rc=$?
if [[ $rc != 0 ]]; then
    cat -- "$OUT/verify.err" >&2
    printf 'spike: failed rc=%s\n' "$rc" >&2
    exit "$rc"
fi
if command grep -E '^warning' "$OUT/verify.err"; then
    printf 'spike: verifier warnings\n' >&2
    exit 1
fi
jq -e '.["verification-results"] | .success and .["is-verifying-entire-crate"] and .errors == 0' "$OUT/verify.json" > /dev/null
jq -r '.["times-ms"].smt["smt-run-module-times"][] |
    "spike: module=\(.module) smt_ms=\(.time) rlimit=\(.rlimit)"' "$OUT/verify.json"
jq -r '"spike: verified=\(.["verification-results"].verified) errors=0 total_ms=\(.["times-ms"].total)"' "$OUT/verify.json"
