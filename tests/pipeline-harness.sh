#!/usr/bin/env bash
set -eu

ROOT=$PWD
if ! [ -d vendor/ape ] || ! [ -f tools/ace_front_end.py ]; then
    printf 'FAIL repo-root: run from cnl-ckc repository root\n'
    printf 'SUMMARY: 0 passed, 1 failed\n'
    exit 1
fi

SWIPL=${SWIPL:-swipl}
DOCS="$ROOT/tests/fixtures/pipeline/docs"
GOLDEN="$ROOT/tests/fixtures/pipeline/golden"
RED="$ROOT/tests/fixtures/pipeline/red"
SCRATCH="$ROOT/.scratch/pipeline-harness"
TREE="$SCRATCH/tree"
OUT1="$SCRATCH/out1"
OUT2="$SCRATCH/out2"
PASS_COUNT=0
RUN_STATUS=0

pass_case() {
    PASS_COUNT=$((PASS_COUNT + 1))
    printf 'PASS %s\n' "$1"
}

fail_case() {
    printf 'FAIL %s: %s\n' "$1" "$2"
    printf 'SUMMARY: %s passed, 1 failed\n' "$PASS_COUNT"
    exit 1
}

run_front_end() {
    local stdout_path stderr_path
    stdout_path=$1
    stderr_path=$2
    shift 2

    if SWIPL="$SWIPL" PYTHONDONTWRITEBYTECODE=1 python3 -P tools/ace_front_end.py "$@" \
        >"$stdout_path" 2>"$stderr_path"; then
        RUN_STATUS=0
    else
        RUN_STATUS=$?
    fi
}

write_file_set() {
    local directory output_path
    directory=$1
    output_path=$2
    find "$directory" -mindepth 1 -maxdepth 1 -printf '%f\n' | LC_ALL=C sort >"$output_path"
}

check_failure() {
    local label expected_status expected_text out_dir stdout_path stderr_path line_count
    label=$1
    expected_status=$2
    expected_text=$3
    out_dir=$4
    stdout_path=$5
    stderr_path=$6

    if [ "$RUN_STATUS" -ne "$expected_status" ]; then
        fail_case "$label/status" "expected $expected_status, got $RUN_STATUS"
    fi
    if [ -s "$stdout_path" ]; then
        fail_case "$label/stdout" "expected zero bytes"
    fi
    if ! command grep -Fq "$expected_text" "$stderr_path"; then
        fail_case "$label/stderr" "missing: $expected_text"
    fi
    line_count=$(command grep -c '^' "$stderr_path" || :)
    if [ "$line_count" -ne 1 ]; then
        fail_case "$label/stderr" "expected one line, got $line_count"
    fi
    if ! printf '%s\n' "$(<"$stderr_path")" | cmp - "$stderr_path"; then
        fail_case "$label/stderr" "expected exactly one LF-terminated line"
    fi
    if [ -e "$out_dir" ] || [ -L "$out_dir" ]; then
        fail_case "$label/zero-write" "output directory exists"
    fi
    pass_case "$label"
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

rm -rf "$SCRATCH"
mkdir -p "$TREE" "$SCRATCH/run1" "$SCRATCH/run2" "$SCRATCH/red"
trap 'rm -rf "$SCRATCH"' EXIT
cp -a "$ROOT/vendor/ape/." "$TREE/"
pass_case "vendor/copy"

if ! make -C "$TREE" plp "swipl=$SWIPL"; then
    fail_case "vendor/plp" "make plp failed"
fi
if ! [ -f "$TREE/prolog/parser/grammar.plp" ]; then
    fail_case "vendor/plp" "grammar.plp is missing"
fi
pass_case "vendor/plp"

run1_stdout="$SCRATCH/run1/stdout"
run1_stderr="$SCRATCH/run1/stderr"
run_front_end "$run1_stdout" "$run1_stderr" "$TREE" "$DOCS" "$OUT1"
if [ "$RUN_STATUS" -ne 0 ]; then
    fail_case "green/run1/status" "expected 0, got $RUN_STATUS"
fi
if [ -s "$run1_stderr" ]; then
    fail_case "green/run1/stderr" "expected zero bytes"
fi
pass_case "green/run1"

run1_expected="$SCRATCH/run1/expected-stdout"
printf '%s\n' \
    "ace-front-end: wrote $OUT1/anchor.drs.pl" \
    "ace-front-end: wrote $OUT1/manifest.pl" \
    "ace-front-end: wrote $OUT1/twosent.drs.pl" \
    "ace-front-end: wrote $OUT1/zorbomat.drs.pl" \
    "ace-front-end: ok 3 documents" \
    >"$run1_expected"
if ! cmp "$run1_stdout" "$run1_expected"; then
    fail_case "green/run1-stdout" "success output differs"
fi
pass_case "green/run1-stdout"

out1_files="$SCRATCH/run1/out-files"
golden_files="$SCRATCH/run1/golden-files"
write_file_set "$OUT1" "$out1_files"
write_file_set "$GOLDEN" "$golden_files"
if ! cmp "$out1_files" "$golden_files"; then
    fail_case "green/file-set" "output and golden file sets differ"
fi
pass_case "green/file-set"

for golden_path in "$GOLDEN"/*; do
    name=${golden_path##*/}
    if ! cmp "$OUT1/$name" "$golden_path"; then
        fail_case "green/goldens" "mismatch: $name"
    fi
done
pass_case "green/goldens"

if ! command grep -Fq '/(1,' "$GOLDEN/twosent.drs.pl"; then
    fail_case "green/twosent-ordinals" "missing sentence 1 annotation"
fi
if ! command grep -Fq '/(2,' "$GOLDEN/twosent.drs.pl"; then
    fail_case "green/twosent-ordinals" "missing sentence 2 annotation"
fi
pass_case "green/twosent-ordinals"

run2_stdout="$SCRATCH/run2/stdout"
run2_stderr="$SCRATCH/run2/stderr"
run_front_end "$run2_stdout" "$run2_stderr" "$TREE" "$DOCS" "$OUT2"
if [ "$RUN_STATUS" -ne 0 ]; then
    fail_case "determinism/run2/status" "expected 0, got $RUN_STATUS"
fi
if [ -s "$run2_stderr" ]; then
    fail_case "determinism/run2/stderr" "expected zero bytes"
fi
pass_case "determinism/run2"

out2_files="$SCRATCH/run2/out-files"
write_file_set "$OUT2" "$out2_files"
if ! cmp "$out1_files" "$out2_files"; then
    fail_case "determinism/file-set" "fresh run file sets differ"
fi
pass_case "determinism/file-set"

for first_path in "$OUT1"/*; do
    name=${first_path##*/}
    if ! cmp "$first_path" "$OUT2/$name"; then
        fail_case "determinism/bytes" "fresh runs differ: $name"
    fi
done
pass_case "determinism/bytes"

oov_out="$SCRATCH/red/oov-out"
oov_stdout="$SCRATCH/red/oov.stdout"
oov_stderr="$SCRATCH/red/oov.stderr"
run_front_end "$oov_stdout" "$oov_stderr" "$TREE" "$RED/oov" "$oov_out"
check_failure "red/oov" 1 'adapter_error(ape_messages,' "$oov_out" "$oov_stdout" "$oov_stderr"

badid_out="$SCRATCH/red/badid-out"
badid_stdout="$SCRATCH/red/badid.stdout"
badid_stderr="$SCRATCH/red/badid.stderr"
run_front_end "$badid_stdout" "$badid_stderr" "$TREE" "$RED/badid" "$badid_out"
check_failure "red/badid" 2 'ace-front-end: docid:' "$badid_out" "$badid_stdout" "$badid_stderr"

orphan_out="$SCRATCH/red/orphan-out"
orphan_stdout="$SCRATCH/red/orphan.stdout"
orphan_stderr="$SCRATCH/red/orphan.stderr"
run_front_end "$orphan_stdout" "$orphan_stderr" "$TREE" "$RED/orphan" "$orphan_out"
check_failure "red/orphan" 2 'ace-front-end: docs-dir:' "$orphan_out" "$orphan_stdout" "$orphan_stderr"

exists_out="$SCRATCH/red/exists-out"
exists_stdout="$SCRATCH/red/exists.stdout"
exists_stderr="$SCRATCH/red/exists.stderr"
mkdir "$exists_out"
run_front_end "$exists_stdout" "$exists_stderr" "$TREE" "$DOCS" "$exists_out"
if [ "$RUN_STATUS" -ne 2 ]; then
    fail_case "red/out-dir-exists/status" "expected 2, got $RUN_STATUS"
fi
if [ -s "$exists_stdout" ]; then
    fail_case "red/out-dir-exists/stdout" "expected zero bytes"
fi
if ! command grep -Fq 'ace-front-end: out-dir:' "$exists_stderr"; then
    fail_case "red/out-dir-exists/stderr" "missing out-dir error"
fi
exists_line_count=$(command grep -c '^' "$exists_stderr" || :)
if [ "$exists_line_count" -ne 1 ]; then
    fail_case "red/out-dir-exists/stderr" "expected one line, got $exists_line_count"
fi
if ! printf '%s\n' "$(<"$exists_stderr")" | cmp - "$exists_stderr"; then
    fail_case "red/out-dir-exists/stderr" "expected exactly one LF-terminated line"
fi
pass_case "red/out-dir-exists"

usage_out="$SCRATCH/red/usage-out"
usage_stdout="$SCRATCH/red/usage.stdout"
usage_stderr="$SCRATCH/red/usage.stderr"
run_front_end "$usage_stdout" "$usage_stderr" "$TREE" "$DOCS" "$usage_out" extra
if [ "$RUN_STATUS" -ne 2 ]; then
    fail_case "red/usage/status" "expected 2, got $RUN_STATUS"
fi
if [ -s "$usage_stdout" ]; then
    fail_case "red/usage/stdout" "expected zero bytes"
fi
if [ -e "$usage_out" ] || [ -L "$usage_out" ]; then
    fail_case "red/usage/zero-write" "output directory exists"
fi
pass_case "red/usage"

empty_swipl="$SCRATCH/red/empty-swipl"
printf '%s\n' '#!/usr/bin/env sh' 'exit 0' >"$empty_swipl"
chmod +x "$empty_swipl"
empty_adapter_out="$SCRATCH/red/empty-adapter-out"
empty_adapter_stdout="$SCRATCH/red/empty-adapter.stdout"
empty_adapter_stderr="$SCRATCH/red/empty-adapter.stderr"
real_swipl=$SWIPL
SWIPL=$empty_swipl
run_front_end "$empty_adapter_stdout" "$empty_adapter_stderr" "$TREE" "$DOCS" "$empty_adapter_out"
SWIPL=$real_swipl
check_failure "red/empty-adapter-stdout" 2 'ace-front-end: adapter-stdout:' "$empty_adapter_out" "$empty_adapter_stdout" "$empty_adapter_stderr"

missing_tree_out="$SCRATCH/red/missing-tree-out"
missing_tree_stdout="$SCRATCH/red/missing-tree.stdout"
missing_tree_stderr="$SCRATCH/red/missing-tree.stderr"
run_front_end "$missing_tree_stdout" "$missing_tree_stderr" "$SCRATCH/no-such-tree" "$DOCS" "$missing_tree_out"
check_failure "red/missing-ape-tree" 2 'adapter_error(ape_load,' "$missing_tree_out" "$missing_tree_stdout" "$missing_tree_stderr"

if ! vendor_status=$(git status --porcelain -- vendor/); then
    fail_case "vendor/clean" "git status failed"
fi
if [ -n "$vendor_status" ]; then
    printf '%s\n' "$vendor_status"
    fail_case "vendor/clean" "vendor tree changed"
fi
pass_case "vendor/clean"

printf 'SUMMARY: %s passed, 0 failed\n' "$PASS_COUNT"
