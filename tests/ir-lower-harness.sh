#!/usr/bin/env bash
set -eu

ROOT=$PWD
if ! [ -f src/prolog/ir_tool.pl ] || ! [ -f src/prolog/drs_to_ir.pl ] || \
        ! [ -d tests/fixtures/slice/golden ] || \
        ! [ -d tests/fixtures/slice/ir ] || \
        ! [ -d tests/fixtures/lower/red ]; then
    printf 'FAIL repo-root: run from cnl-ckc repository root\n'
    printf 'SUMMARY: 0 passed, 1 failed\n'
    exit 1
fi

SWIPL=${SWIPL:-swipl}
DOCS="$ROOT/tests/fixtures/slice/docs"
GOLDEN="$ROOT/tests/fixtures/slice/golden"
IR="$ROOT/tests/fixtures/slice/ir"
RED="$ROOT/tests/fixtures/lower/red"
SCRATCH="$ROOT/.scratch/ir-lower-harness"
PASS_COUNT=0
RUN_STATUS=0
EXPECTED_PASS_COUNT=27

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

    if "$SWIPL" -q -f none -F none -s "$ROOT/src/prolog/ir_tool.pl" \
        -g main -t 'halt(9)' -- "$@" \
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
    if ! command grep -Eq \
        "^ir_tool_error\\(${expected_stage},${expected_class},.*\\)\\.$" \
        "$stderr_path"; then
        fail_case "$label/class" \
            "expected ir_tool_error($expected_stage,$expected_class,...)"
    fi
    pass_case "$label"
}

if swipl_version=$("$SWIPL" --version 2>&1); then
    case $swipl_version in
        *'SWI-Prolog version 9.2.9 '*)
            pass_case "swipl/version: $swipl_version"
            ;;
        *)
            fail_case "swipl/version" \
                "expected SWI-Prolog version 9.2.9, got: $swipl_version"
            ;;
    esac
else
    fail_case "swipl/version" "could not run $SWIPL"
fi

set -- "$DOCS"/*.ace
if [ "$#" -ne 2 ]; then
    fail_case "fixtures/count" "expected 2 ACE fixtures, got $#"
fi
set -- "$DOCS"/*.ulex
if [ "$#" -ne 2 ]; then
    fail_case "fixtures/count" "expected 2 Ulex fixtures, got $#"
fi
set -- "$GOLDEN"/*.drs.pl
if [ "$#" -ne 2 ]; then
    fail_case "fixtures/count" "expected 2 DRS goldens, got $#"
fi
set -- "$GOLDEN"/*.pl
if [ "$#" -ne 3 ]; then
    fail_case "fixtures/count" "expected 3 front-end goldens, got $#"
fi
set -- "$IR"/*.ir.pl
if [ "$#" -ne 2 ]; then
    fail_case "fixtures/count" "expected 2 IR goldens, got $#"
fi
set -- "$RED"/*.pl
if [ "$#" -ne 18 ]; then
    fail_case "fixtures/count" "expected 18 red fixtures, got $#"
fi
pass_case "fixtures/count"

rm -rf "$SCRATCH"
mkdir -p "$SCRATCH/green" "$SCRATCH/red" "$SCRATCH/scratch" \
    "$SCRATCH/usage" "$SCRATCH/determinism"
trap 'rm -rf "$SCRATCH"' EXIT

for input in "$GOLDEN"/*.drs.pl; do
    name=${input##*/}
    stem=${name%.drs.pl}
    expected="$IR/$stem.ir.pl"
    stdout_path="$SCRATCH/green/$stem.lower.stdout"
    stderr_path="$SCRATCH/green/$stem.lower.stderr"
    run_tool "$input" "$stdout_path" "$stderr_path" lower
    if [ "$RUN_STATUS" -ne 0 ]; then
        fail_case "green/$stem/lower-status" "expected 0, got $RUN_STATUS"
    fi
    if [ -s "$stderr_path" ]; then
        fail_case "green/$stem/lower-stderr" "expected zero bytes"
    fi
    if ! cmp "$stdout_path" "$expected"; then
        fail_case "green/$stem/lower-bytes" "lower output differs from golden"
    fi

    validate_stdout="$SCRATCH/green/$stem.validate.stdout"
    validate_stderr="$SCRATCH/green/$stem.validate.stderr"
    run_tool "$expected" "$validate_stdout" "$validate_stderr" validate
    if [ "$RUN_STATUS" -ne 0 ]; then
        fail_case "green/$stem/validate-status" "expected 0, got $RUN_STATUS"
    fi
    if [ -s "$validate_stdout" ] || [ -s "$validate_stderr" ]; then
        fail_case "green/$stem/validate-streams" "expected zero bytes"
    fi
    pass_case "green/$stem"
done

run_committed_red() {
    local name expected_class stdout_path stderr_path
    name=$1
    expected_class=$2
    stdout_path="$SCRATCH/red/$name.stdout"
    stderr_path="$SCRATCH/red/$name.stderr"
    run_tool "$RED/$name.pl" "$stdout_path" "$stderr_path" lower
    check_rejection "red/$name" 1 lower "$expected_class" \
        "$stdout_path" "$stderr_path"
}

run_committed_red wh-query wh_query
run_committed_red zero-question question_count
run_committed_red two-questions question_count
run_committed_red non-final-question question_count
run_committed_red unpaired-object copula
run_committed_red unpaired-be copula
run_committed_red be-non-named copula
run_committed_red object-wrong-fields copula
run_committed_red object-field-alias-event copula
run_committed_red rule-object-field-alias-event unsupported
run_committed_red event-reuse referent
run_committed_red unconsumed-domain referent
run_committed_red undeclared-referent referent
run_committed_red unknown-condition unsupported
run_committed_red nested-implication unsupported
run_committed_red envelope-wrong-header envelope
run_committed_red envelope-missing-drs envelope
run_committed_red envelope-trailing-term envelope

base="$GOLDEN/slice.drs.pl"
if ! command sed '3s/,/, /' "$base" >"$SCRATCH/scratch/noncanonical.pl"; then
    fail_case "scratch/generate" "could not create noncanonical input"
fi
printf '\xff' >"$SCRATCH/scratch/bad-utf8.pl"

run_scratch_red() {
    local name expected_class stdout_path stderr_path
    name=$1
    expected_class=$2
    stdout_path="$SCRATCH/scratch/$name.stdout"
    stderr_path="$SCRATCH/scratch/$name.stderr"
    run_tool "$SCRATCH/scratch/$name.pl" "$stdout_path" "$stderr_path" lower
    check_rejection "scratch/$name" 1 lower "$expected_class" \
        "$stdout_path" "$stderr_path"
}

run_scratch_red noncanonical canonical
run_scratch_red bad-utf8 input_utf8

usage_stdout="$SCRATCH/usage/lower-extra.stdout"
usage_stderr="$SCRATCH/usage/lower-extra.stderr"
run_tool "$base" "$usage_stdout" "$usage_stderr" lower extra-arg
check_rejection "usage/lower-extra" 2 cli usage \
    "$usage_stdout" "$usage_stderr"

green_stdout1="$SCRATCH/determinism/green.run1.stdout"
green_stderr1="$SCRATCH/determinism/green.run1.stderr"
run_tool "$base" "$green_stdout1" "$green_stderr1" lower
green_status1=$RUN_STATUS
green_stdout2="$SCRATCH/determinism/green.run2.stdout"
green_stderr2="$SCRATCH/determinism/green.run2.stderr"
run_tool "$base" "$green_stdout2" "$green_stderr2" lower
green_status2=$RUN_STATUS
if [ "$green_status1" -ne 0 ] || [ "$green_status2" -ne 0 ]; then
    fail_case "determinism/green/status" \
        "expected two status-0 runs, got $green_status1 and $green_status2"
fi
if [ -s "$green_stderr1" ] || [ -s "$green_stderr2" ]; then
    fail_case "determinism/green/stderr" "expected zero bytes"
fi
if ! cmp "$green_stdout1" "$green_stdout2" || \
        ! cmp "$green_stdout1" "$IR/slice.ir.pl"; then
    fail_case "determinism/green/stdout" "fresh runs differ"
fi
pass_case "determinism/green"

red_input="$RED/event-reuse.pl"
red_stdout1="$SCRATCH/determinism/red.run1.stdout"
red_stderr1="$SCRATCH/determinism/red.run1.stderr"
run_tool "$red_input" "$red_stdout1" "$red_stderr1" lower
red_status1=$RUN_STATUS
red_stdout2="$SCRATCH/determinism/red.run2.stdout"
red_stderr2="$SCRATCH/determinism/red.run2.stderr"
run_tool "$red_input" "$red_stdout2" "$red_stderr2" lower
red_status2=$RUN_STATUS
if [ "$red_status1" -ne 1 ] || [ "$red_status2" -ne 1 ]; then
    fail_case "determinism/red/status" \
        "expected two status-1 runs, got $red_status1 and $red_status2"
fi
if [ -s "$red_stdout1" ] || [ -s "$red_stdout2" ]; then
    fail_case "determinism/red/stdout" "expected zero bytes"
fi
if ! cmp "$red_stderr1" "$red_stderr2"; then
    fail_case "determinism/red/stderr" "fresh runs differ"
fi
if ! command grep -Eq '^ir_tool_error\(lower,referent,.*\)\.$' \
        "$red_stderr1"; then
    fail_case "determinism/red/class" "expected referent rejection"
fi
pass_case "determinism/red"

if [ "$PASS_COUNT" -ne "$EXPECTED_PASS_COUNT" ]; then
    fail_case "harness/pass-count" \
        "expected $EXPECTED_PASS_COUNT, got $PASS_COUNT"
fi
printf 'SUMMARY: %s passed, 0 failed\n' "$PASS_COUNT"
