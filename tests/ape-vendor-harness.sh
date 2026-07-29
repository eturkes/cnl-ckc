#!/usr/bin/env bash
set -euo pipefail

ROOT=$PWD
if ! [ -d vendor/ape ] || ! [ -f vendor/clex/clex_lexicon.pl ]; then
    printf 'FAIL repo-root: run from cnl-ckc repository root\n'
    printf 'SUMMARY: 0 passed, 1 failed\n'
    exit 1
fi

SWIPL=${SWIPL:-swipl}
SCRATCH="$ROOT/.scratch/ape-vendor-harness.$$"
TREE="$SCRATCH/tree"
CLEX="$ROOT/vendor/clex/clex_lexicon.pl"
PASS_COUNT=0
EXPECTED_PASS_COUNT=10

pass_case() {
    PASS_COUNT=$((PASS_COUNT + 1))
    printf 'PASS %s\n' "$1"
}

fail_case() {
    printf 'FAIL %s: %s\n' "$1" "$2"
    printf 'SUMMARY: %s passed, 1 failed\n' "$PASS_COUNT"
    exit 1
}

if swipl_version=$("$SWIPL" --version 2>&1); then
    case $swipl_version in
        *'SWI-Prolog version 9.2.9 '*)
            pass_case "swipl/version: $swipl_version"
            ;;
        *)
            fail_case "swipl/version" "expected SWI-Prolog version 9.2.9, got: $swipl_version"
            ;;
    esac
else
    fail_case "swipl/version" "could not run $SWIPL"
fi

vendor_status=''
if ! vendor_status=$(git status --porcelain --ignored -- vendor/); then
    fail_case "vendor/status" "git status failed"
fi
if [ -n "$vendor_status" ]; then
    fail_case "vendor/status" "git status --porcelain --ignored -- vendor/ was not empty"
fi
pass_case "vendor/status"

rm -rf "$SCRATCH"
mkdir -p "$TREE"
trap 'rm -rf "$SCRATCH"' EXIT

manifest_paths="$SCRATCH/manifest.paths"
tracked_paths="$SCRATCH/tracked.paths"
if ! cut -c67- "$ROOT/vendor/ape/MANIFEST.sha256" |
    sed 's#^\./##' |
    LC_ALL=C sort >"$manifest_paths"; then
    fail_case "vendor/manifest-paths" "could not normalize MANIFEST.sha256 paths"
fi
if ! git ls-files vendor/ape |
    sed -e 's#^vendor/ape/##' \
        -e '/^PROVENANCE$/d' \
        -e '/^PROVENANCE\.md$/d' \
        -e '/^MANIFEST\.sha256$/d' \
        -e '/^patches\//d' |
    LC_ALL=C sort >"$tracked_paths"; then
    fail_case "vendor/manifest-paths" "could not normalize tracked vendor/ape paths"
fi
if ! cmp -s "$manifest_paths" "$tracked_paths"; then
    fail_case "vendor/manifest-paths" "MANIFEST.sha256 paths differ from tracked vendor/ape files"
fi
pass_case "vendor/manifest-paths"

cp -a "$ROOT/vendor/ape/." "$TREE/"
pass_case "vendor/copy"

if ! make -C "$TREE" install "swipl=$SWIPL"; then
    fail_case "vendor/build" "make install failed"
fi
if ! [ -x "$TREE/ape.exe" ]; then
    fail_case "vendor/build" "ape.exe is missing or not executable"
fi
pass_case "vendor/build"

exe_stdout="$SCRATCH/ape-exe.stdout"
exe_stderr="$SCRATCH/ape-exe.stderr"
if ! (cd "$TREE" && ./ape.exe -text "John waits." -solo drs) >"$exe_stdout" 2>"$exe_stderr"; then
    fail_case "anchor/executable" "ape.exe exited nonzero"
fi
if ! [ -s "$exe_stdout" ]; then
    fail_case "anchor/executable" "stdout is empty"
fi
if grep -Fq 'drs([],[])' "$exe_stdout"; then
    fail_case "anchor/executable" "stdout contains empty DRS"
fi
pass_case "anchor/executable"

direct_stdout="$SCRATCH/direct-source.stdout"
direct_stderr="$SCRATCH/direct-source.stderr"
if (
    cd "$TREE"
    "$SWIPL" -q \
        -f none -F none \
        -s prolog/parser/ace_to_drs.pl \
        -g "(ace_to_drs:acetext_to_drs('John waits.',off,off,_S,_T,Drs,Msgs,_Time)->(Drs\\=drs([],[])->(Msgs==[]->halt(0);halt(8));halt(7));halt(6))" \
        -t 'halt(9)'
) >"$direct_stdout" 2>"$direct_stderr"; then
    direct_status=0
else
    direct_status=$?
fi
if [ "$direct_status" -ne 0 ]; then
    fail_case "anchor/direct-source" "swipl exited $direct_status"
fi
pass_case "anchor/direct-source"

clex_manifest_paths="$SCRATCH/clex-manifest.paths"
clex_tracked_paths="$SCRATCH/clex-tracked.paths"
if ! cut -c67- "$ROOT/vendor/clex/MANIFEST.sha256" |
    sed 's#^\./##' |
    LC_ALL=C sort >"$clex_manifest_paths"; then
    fail_case "clex/manifest-paths" "could not normalize MANIFEST.sha256 paths"
fi
if ! git ls-files vendor/clex |
    sed -e 's#^vendor/clex/##' \
        -e '/^PROVENANCE$/d' \
        -e '/^MANIFEST\.sha256$/d' |
    LC_ALL=C sort >"$clex_tracked_paths"; then
    fail_case "clex/manifest-paths" "could not normalize tracked vendor/clex paths"
fi
if ! cmp -s "$clex_manifest_paths" "$clex_tracked_paths"; then
    fail_case "clex/manifest-paths" \
        "MANIFEST.sha256 paths differ from tracked vendor/clex files"
fi
pass_case "clex/manifest-paths"

if ! (cd "$ROOT/vendor/clex" && sha256sum --quiet -c MANIFEST.sha256); then
    fail_case "clex/digest" "MANIFEST.sha256 verification failed"
fi
pass_case "clex/digest"
cp "$CLEX" "$TREE/tests/clex_lexicon.pl"

regression_stdout="$SCRATCH/test-ape.stdout"
regression_stderr="$SCRATCH/test-ape.stderr"
if (
    cd "$TREE/tests"
    "$SWIPL" -f test_ape.pl -F none -g main -t halt -q
) >"$regression_stdout" 2>"$regression_stderr"; then
    regression_status=0
else
    regression_status=$?
fi
if [ "$regression_status" -ne 0 ]; then
    fail_case "regression/ape" "swipl exited $regression_status"
fi

pass_count=$(grep -Fc '[----]' "$regression_stdout" || true)
zero_count=$(grep -Fc '[0000]' "$regression_stdout" || true)
fail_count=$(grep -Fc '[####]' "$regression_stdout" || true)
zero_to_fail_count=$(grep -Fc '[0->#]' "$regression_stdout" || true)
fail_to_zero_count=$(grep -Fc '[#->0]' "$regression_stdout" || true)
printf 'REGRESSION: [----]=%s [0000]=%s [####]=%s [0->#]=%s [#->0]=%s\n' \
    "$pass_count" "$zero_count" "$fail_count" "$zero_to_fail_count" "$fail_to_zero_count"

if [ "$pass_count" -ne 2813 ] || [ "$zero_count" -ne 920 ] || \
    [ "$fail_count" -ne 0 ] || [ "$zero_to_fail_count" -ne 0 ] || \
    [ "$fail_to_zero_count" -ne 0 ]; then
    fail_case "regression/counts" "unexpected result counts"
fi
pass_case "regression/counts"

if [ "$PASS_COUNT" -ne "$EXPECTED_PASS_COUNT" ]; then
    fail_case "harness/pass-count" \
        "expected $EXPECTED_PASS_COUNT, got $PASS_COUNT"
fi
printf 'SUMMARY: %s passed, 0 failed\n' "$PASS_COUNT"
