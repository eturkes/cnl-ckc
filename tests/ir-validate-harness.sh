#!/usr/bin/env bash
set -eu

ROOT=$PWD
if ! [ -f src/prolog/ir_tool.pl ] || ! [ -d tests/fixtures/ir/green ] || ! [ -d tests/fixtures/ir/red ]; then
    printf 'FAIL repo-root: run from cnl-ckc repository root\n'
    printf 'SUMMARY: 0 passed, 1 failed\n'
    exit 1
fi

SWIPL=${SWIPL:-swipl}
GREEN="$ROOT/tests/fixtures/ir/green"
RED="$ROOT/tests/fixtures/ir/red"
SCRATCH="$ROOT/.scratch/ir-validate-harness"
PASS_COUNT=0
RUN_STATUS=0
EXPECTED_PASS_COUNT=53

pass_case() {
    PASS_COUNT=$((PASS_COUNT + 1))
    printf 'PASS %s\n' "$1"
}

fail_case() {
    printf 'FAIL %s: %s\n' "$1" "$2"
    printf 'SUMMARY: %s passed, 1 failed\n' "$PASS_COUNT"
    exit 1
}

run_tool() {
    local input stdout_path stderr_path
    input=$1
    stdout_path=$2
    stderr_path=$3
    shift 3

    if "$SWIPL" -q -f none -F none -s "$ROOT/src/prolog/ir_tool.pl" -g main -t 'halt(9)' -- "$@" \
        <"$input" >"$stdout_path" 2>"$stderr_path"; then
        RUN_STATUS=0
    else
        RUN_STATUS=$?
    fi
}

check_rejection() {
    local label expected_status expected_stage expected_class stdout_path stderr_path
    local line_count
    label=$1
    expected_status=$2
    expected_stage=$3
    expected_class=$4
    stdout_path=$5
    stderr_path=$6

    if [ "$RUN_STATUS" -ne "$expected_status" ]; then
        fail_case "$label/status" "expected $expected_status, got $RUN_STATUS"
    fi
    if [ -s "$stdout_path" ]; then
        fail_case "$label/stdout" "expected zero bytes"
    fi
    line_count=$(command grep -c '^' "$stderr_path" || :)
    if [ "$line_count" -ne 1 ]; then
        fail_case "$label/stderr" "expected one line, got $line_count"
    fi
    if ! printf '%s\n' "$(<"$stderr_path")" | cmp - "$stderr_path"; then
        fail_case "$label/stderr" "expected exactly one LF-terminated line"
    fi
    if ! command grep -Eq "^ir_tool_error\\(${expected_stage},${expected_class},.*\\)\\.$" "$stderr_path"; then
        fail_case "$label/class" "expected ir_tool_error($expected_stage,$expected_class,...)"
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

set -- "$GREEN"/*.pl
if [ "$#" -ne 3 ]; then
    fail_case "fixtures/count" "expected 3 green fixtures, got $#"
fi
set -- "$RED"/*.pl
if [ "$#" -ne 31 ]; then
    fail_case "fixtures/count" "expected 31 red fixtures, got $#"
fi
pass_case "fixtures/count"

rm -rf "$SCRATCH"
mkdir -p "$SCRATCH/green" "$SCRATCH/red" "$SCRATCH/usage" "$SCRATCH/determinism"
trap 'rm -rf "$SCRATCH"' EXIT

for input in "$GREEN"/*.pl; do
    name=${input##*/}
    stem=${name%.pl}
    stdout_path="$SCRATCH/green/$stem.stdout"
    stderr_path="$SCRATCH/green/$stem.stderr"
    run_tool "$input" "$stdout_path" "$stderr_path" validate
    if [ "$RUN_STATUS" -ne 0 ]; then
        fail_case "green/$stem/status" "expected 0, got $RUN_STATUS"
    fi
    if [ -s "$stdout_path" ]; then
        fail_case "green/$stem/stdout" "expected zero bytes"
    fi
    if [ -s "$stderr_path" ]; then
        fail_case "green/$stem/stderr" "expected zero bytes"
    fi
    pass_case "green/$stem"
done

run_committed_red() {
    local name expected_class stdout_path stderr_path
    name=$1
    expected_class=$2
    stdout_path="$SCRATCH/red/$name.stdout"
    stderr_path="$SCRATCH/red/$name.stderr"
    run_tool "$RED/$name.pl" "$stdout_path" "$stderr_path" validate
    check_rejection "red/$name" 1 validate "$expected_class" "$stdout_path" "$stderr_path"
}

run_committed_red envelope-wrong-version envelope
run_committed_red envelope-missing-document envelope
run_committed_red query-zero query_count
run_committed_red query-two query_count
run_committed_red section-interleave section_order
run_committed_red envelope-trailing-term envelope
run_committed_red shape-unknown-constructor shape
run_committed_red shape-bad-float shape
run_committed_red canonical-string-atomic canonical
run_committed_red shape-bad-string-codes shape
run_committed_red shape-native-variable shape
run_committed_red shape-empty-args shape
run_committed_red shape-empty-tokens shape
run_committed_red identity-kind-mismatch identity
run_committed_red identity-sentence-mismatch identity
run_committed_red identity-zero-ordinal identity
run_committed_red identity-negative-ordinal identity
run_committed_red identity-bad-docid identity
run_committed_red identity-bad-hex identity
run_committed_red ordering-out-of-order ordering
run_committed_red ordering-duplicate-id ordering
run_committed_red ordering-descending-tokens ordering
run_committed_red scope-var-in-fact scope
run_committed_red scope-var-in-query scope
run_committed_red scope-non-dense scope
run_committed_red scope-wrong-first-occurrence scope
run_committed_red safety-head-var safety
run_committed_red safety-empty-body safety
run_committed_red naf-literal naf
run_committed_red cycle-two-rule cycle
run_committed_red cycle-self-loop cycle

base="$GREEN/minimal.pl"
printf '\xff' >"$SCRATCH/red/utf8-lone-ff.pl"
printf '\xc0\xaf' >"$SCRATCH/red/utf8-overlong.pl"
printf '\xed\xa0\x80' >"$SCRATCH/red/utf8-surrogate.pl"
printf '\xf4\x90\x80\x80' >"$SCRATCH/red/utf8-out-of-range.pl"
printf '\xe2\x82' >"$SCRATCH/red/utf8-truncated.pl"
while IFS= read -r line; do
    printf '%s\r\n' "$line"
done <"$base" >"$SCRATCH/red/crlf.pl"
base_size=$(wc -c <"$base")
if ! dd if="$base" of="$SCRATCH/red/missing-final-lf.pl" bs=1 count=$((base_size - 1)) status=none; then
    fail_case "scratch/generate" "could not remove final LF"
fi
{
    printf '\xef\xbb\xbf'
    dd if="$base" status=none
} >"$SCRATCH/red/bom.pl"
: >"$SCRATCH/red/empty.pl"
printf '%s\n' 'cnl_ir_record(1).' 'document(.' >"$SCRATCH/red/syntax.pl"
if ! command sed '3s/,/, /' "$base" >"$SCRATCH/red/noncanonical-spacing.pl"; then
    fail_case "scratch/generate" "could not create spacing case"
fi
if ! command sed '3s/\[named(alice)\]/[named(alice)|[]]/' "$base" >"$SCRATCH/red/operator-notation.pl"; then
    fail_case "scratch/generate" "could not create operator case"
fi

run_scratch_red() {
    local name expected_class stdout_path stderr_path
    name=$1
    expected_class=$2
    stdout_path="$SCRATCH/red/$name.stdout"
    stderr_path="$SCRATCH/red/$name.stderr"
    run_tool "$SCRATCH/red/$name.pl" "$stdout_path" "$stderr_path" validate
    check_rejection "scratch/$name" 1 validate "$expected_class" "$stdout_path" "$stderr_path"
}

run_scratch_red utf8-lone-ff input_utf8
run_scratch_red utf8-overlong input_utf8
run_scratch_red utf8-surrogate input_utf8
run_scratch_red utf8-out-of-range input_utf8
run_scratch_red utf8-truncated input_utf8
run_scratch_red crlf canonical
run_scratch_red missing-final-lf canonical
run_scratch_red bom syntax
run_scratch_red empty envelope
run_scratch_red syntax syntax
run_scratch_red noncanonical-spacing canonical
run_scratch_red operator-notation canonical

usage_case() {
    local label stdout_path stderr_path
    label=$1
    shift
    stdout_path="$SCRATCH/usage/$label.stdout"
    stderr_path="$SCRATCH/usage/$label.stderr"
    run_tool "$base" "$stdout_path" "$stderr_path" "$@"
    check_rejection "usage/$label" 2 cli usage "$stdout_path" "$stderr_path"
}

usage_case no-args
usage_case unknown compile
usage_case extra validate extra

red_stdout1="$SCRATCH/determinism/red.run1.stdout"
red_stderr1="$SCRATCH/determinism/red.run1.stderr"
run_tool "$RED/naf-literal.pl" "$red_stdout1" "$red_stderr1" validate
red_status1=$RUN_STATUS
red_stdout2="$SCRATCH/determinism/red.run2.stdout"
red_stderr2="$SCRATCH/determinism/red.run2.stderr"
run_tool "$RED/naf-literal.pl" "$red_stdout2" "$red_stderr2" validate
red_status2=$RUN_STATUS
if [ "$red_status1" -ne 1 ] || [ "$red_status2" -ne 1 ]; then
    fail_case "determinism/red/status" "expected two status-1 runs, got $red_status1 and $red_status2"
fi
if [ -s "$red_stdout1" ] || [ -s "$red_stdout2" ]; then
    fail_case "determinism/red/stdout" "expected zero bytes"
fi
if ! cmp "$red_stderr1" "$red_stderr2"; then
    fail_case "determinism/red/stderr" "fresh runs differ"
fi
if ! command grep -Eq '^ir_tool_error\(validate,naf,.*\)\.$' "$red_stderr1"; then
    fail_case "determinism/red/class" "expected naf rejection"
fi
pass_case "determinism/red"

green_stdout1="$SCRATCH/determinism/green.run1.stdout"
green_stderr1="$SCRATCH/determinism/green.run1.stderr"
run_tool "$GREEN/slice.pl" "$green_stdout1" "$green_stderr1" validate
green_status1=$RUN_STATUS
green_stdout2="$SCRATCH/determinism/green.run2.stdout"
green_stderr2="$SCRATCH/determinism/green.run2.stderr"
run_tool "$GREEN/slice.pl" "$green_stdout2" "$green_stderr2" validate
green_status2=$RUN_STATUS
if [ "$green_status1" -ne 0 ] || [ "$green_status2" -ne 0 ]; then
    fail_case "determinism/green/status" "expected two status-0 runs, got $green_status1 and $green_status2"
fi
if ! cmp "$green_stdout1" "$green_stdout2" || ! cmp "$green_stderr1" "$green_stderr2"; then
    fail_case "determinism/green/streams" "fresh runs differ"
fi
if [ -s "$green_stdout1" ] || [ -s "$green_stderr1" ]; then
    fail_case "determinism/green/streams" "expected zero bytes"
fi
pass_case "determinism/green"

if [ "$PASS_COUNT" -ne "$EXPECTED_PASS_COUNT" ]; then
    fail_case "harness/pass-count" "expected $EXPECTED_PASS_COUNT, got $PASS_COUNT"
fi
printf 'SUMMARY: %s passed, 0 failed\n' "$PASS_COUNT"
