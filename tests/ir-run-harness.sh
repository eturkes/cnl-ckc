#!/usr/bin/env bash
set -eu

ROOT=$PWD
if ! [ -f src/prolog/ir_tool.pl ] || \
        ! [ -f src/prolog/ir_to_prolog.pl ] || \
        ! [ -f src/prolog/inference_kernel.pl ] || \
        ! [ -f src/prolog/explanation.pl ] || \
        ! [ -d tests/fixtures/slice/golden ] || \
        ! [ -d tests/fixtures/slice/ir ] || \
        ! [ -d tests/fixtures/slice/program ] || \
        ! [ -d tests/fixtures/slice/result ] || \
        ! [ -d tests/fixtures/run/green ] || \
        ! [ -d tests/fixtures/run/red ] || \
        ! [ -f tests/fixtures/ir/red/naf-literal.pl ]; then
    printf 'FAIL repo-root: run from cnl-ckc repository root\n'
    printf 'SUMMARY: 0 passed, 1 failed\n'
    exit 1
fi

SWIPL=${SWIPL:-swipl}
GOLDEN="$ROOT/tests/fixtures/slice/golden"
IR="$ROOT/tests/fixtures/slice/ir"
PROGRAM="$ROOT/tests/fixtures/slice/program"
RESULT="$ROOT/tests/fixtures/slice/result"
GREEN="$ROOT/tests/fixtures/run/green"
RED="$ROOT/tests/fixtures/run/red"
IR_RED="$ROOT/tests/fixtures/ir/red"
SCRATCH="$ROOT/.scratch/ir-run-harness"
PASS_COUNT=0
RUN_STATUS=0
EXPECTED_PASS_COUNT=31

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

set -- "$GOLDEN"/*.drs.pl
if [ "$#" -ne 2 ]; then
    fail_case "fixtures/count" "expected 2 DRS goldens, got $#"
fi
set -- "$IR"/*.ir.pl
if [ "$#" -ne 2 ]; then
    fail_case "fixtures/count" "expected 2 IR goldens, got $#"
fi
set -- "$PROGRAM"/*.program.pl
if [ "$#" -ne 2 ]; then
    fail_case "fixtures/count" "expected 2 program goldens, got $#"
fi
set -- "$RESULT"/*.result.pl
if [ "$#" -ne 2 ]; then
    fail_case "fixtures/count" "expected 2 result goldens, got $#"
fi
set -- "$GREEN"/*.program.pl
if [ "$#" -ne 2 ]; then
    fail_case "fixtures/count" "expected 2 hand-green programs, got $#"
fi
set -- "$GREEN"/*.result.pl
if [ "$#" -ne 2 ]; then
    fail_case "fixtures/count" "expected 2 hand-green results, got $#"
fi
set -- "$RED"/*.program.pl
if [ "$#" -ne 14 ]; then
    fail_case "fixtures/count" "expected 14 red programs, got $#"
fi
pass_case "fixtures/count"

rm -rf "$SCRATCH"
mkdir -p "$SCRATCH/chain" "$SCRATCH/green" "$SCRATCH/red" \
    "$SCRATCH/stage-pin" "$SCRATCH/scratch" "$SCRATCH/usage" \
    "$SCRATCH/determinism"
trap 'rm -rf "$SCRATCH"' EXIT

for input in "$GOLDEN"/*.drs.pl; do
    name=${input##*/}
    stem=${name%.drs.pl}

    lower_stdout="$SCRATCH/chain/$stem.lower.stdout"
    lower_stderr="$SCRATCH/chain/$stem.lower.stderr"
    run_tool "$input" "$lower_stdout" "$lower_stderr" lower
    if [ "$RUN_STATUS" -ne 0 ]; then
        fail_case "chain/$stem/lower-status" "expected 0, got $RUN_STATUS"
    fi
    if [ -s "$lower_stderr" ]; then
        fail_case "chain/$stem/lower-stderr" "expected zero bytes"
    fi
    if ! cmp "$lower_stdout" "$IR/$stem.ir.pl"; then
        fail_case "chain/$stem/lower-bytes" "output differs from IR golden"
    fi

    compile_stdout="$SCRATCH/chain/$stem.compile.stdout"
    compile_stderr="$SCRATCH/chain/$stem.compile.stderr"
    run_tool "$lower_stdout" "$compile_stdout" "$compile_stderr" compile
    if [ "$RUN_STATUS" -ne 0 ]; then
        fail_case "chain/$stem/compile-status" "expected 0, got $RUN_STATUS"
    fi
    if [ -s "$compile_stderr" ]; then
        fail_case "chain/$stem/compile-stderr" "expected zero bytes"
    fi
    if ! cmp "$compile_stdout" "$PROGRAM/$stem.program.pl"; then
        fail_case "chain/$stem/compile-bytes" \
            "output differs from program golden"
    fi

    run_stdout="$SCRATCH/chain/$stem.run.stdout"
    run_stderr="$SCRATCH/chain/$stem.run.stderr"
    run_tool "$compile_stdout" "$run_stdout" "$run_stderr" run
    if [ "$RUN_STATUS" -ne 0 ]; then
        fail_case "chain/$stem/run-status" "expected 0, got $RUN_STATUS"
    fi
    if [ -s "$run_stderr" ]; then
        fail_case "chain/$stem/run-stderr" "expected zero bytes"
    fi
    if ! cmp "$run_stdout" "$RESULT/$stem.result.pl"; then
        fail_case "chain/$stem/run-bytes" "output differs from result golden"
    fi
    pass_case "chain/$stem"
done

if ! command grep -Eq '^answer\(.*,proved\)\.$' \
        "$RESULT/slice.result.pl"; then
    fail_case "answer/slice-proved" "expected proved answer line"
fi
pass_case "answer/slice-proved"

if ! command grep -Eq '^answer\(.*,not_proved\)\.$' \
        "$RESULT/slice-unknown.result.pl"; then
    fail_case "answer/slice-unknown" "expected not_proved answer line"
fi
pass_case "answer/slice-unknown"

if command grep -q '^proof(' "$RESULT/slice-unknown.result.pl"; then
    fail_case "answer/slice-unknown-no-proof" "unexpected proof term"
fi
pass_case "answer/slice-unknown-no-proof"

for input in "$GREEN"/*.program.pl; do
    name=${input##*/}
    stem=${name%.program.pl}
    expected="$GREEN/$stem.result.pl"
    stdout_path="$SCRATCH/green/$stem.stdout"
    stderr_path="$SCRATCH/green/$stem.stderr"
    if ! [ -f "$expected" ]; then
        fail_case "green/$stem/pair" "missing result golden"
    fi
    run_tool "$input" "$stdout_path" "$stderr_path" run
    if [ "$RUN_STATUS" -ne 0 ]; then
        fail_case "green/$stem/status" "expected 0, got $RUN_STATUS"
    fi
    if [ -s "$stderr_path" ]; then
        fail_case "green/$stem/stderr" "expected zero bytes"
    fi
    if ! cmp "$stdout_path" "$expected"; then
        fail_case "green/$stem/bytes" "run output differs from golden"
    fi
    pass_case "green/$stem"
done

run_committed_red() {
    local name expected_class stdout_path stderr_path
    name=$1
    expected_class=$2
    stdout_path="$SCRATCH/red/$name.stdout"
    stderr_path="$SCRATCH/red/$name.stderr"
    run_tool "$RED/$name.program.pl" "$stdout_path" "$stderr_path" run
    check_rejection "red/$name" 1 run "$expected_class" \
        "$stdout_path" "$stderr_path"
}

run_committed_red cycle-self-loop cycle
run_committed_red envelope-missing-document envelope
run_committed_red envelope-trailing-after-goal envelope
run_committed_red envelope-wrong-header envelope
run_committed_red identity-fact-with-body identity
run_committed_red naf-literal naf
run_committed_red ordering-duplicate-id ordering
run_committed_red query-count-two query_count
run_committed_red query-count-zero query_count
run_committed_red safety-head-uncovered safety
run_committed_red scope-non-dense scope
run_committed_red section-order-fact-after-rule section_order
run_committed_red shape-native-variable shape
run_committed_red shape-unknown-constructor shape

pin_stdout="$SCRATCH/stage-pin/compile-naf.stdout"
pin_stderr="$SCRATCH/stage-pin/compile-naf.stderr"
run_tool "$IR_RED/naf-literal.pl" "$pin_stdout" "$pin_stderr" compile
check_rejection "stage-pin/compile-naf" 1 compile naf \
    "$pin_stdout" "$pin_stderr"

base_program="$PROGRAM/slice.program.pl"
if ! command sed '3s/,/, /' "$base_program" \
        >"$SCRATCH/scratch/noncanonical.program.pl"; then
    fail_case "scratch/generate" "could not create noncanonical input"
fi
printf '\xff' >"$SCRATCH/scratch/bad-utf8.program.pl"

run_scratch_red() {
    local name expected_class stdout_path stderr_path
    name=$1
    expected_class=$2
    stdout_path="$SCRATCH/scratch/$name.stdout"
    stderr_path="$SCRATCH/scratch/$name.stderr"
    run_tool "$SCRATCH/scratch/$name.program.pl" \
        "$stdout_path" "$stderr_path" run
    check_rejection "scratch/$name" 1 run "$expected_class" \
        "$stdout_path" "$stderr_path"
}

run_scratch_red noncanonical canonical
run_scratch_red bad-utf8 input_utf8

usage_stdout="$SCRATCH/usage/compile-extra.stdout"
usage_stderr="$SCRATCH/usage/compile-extra.stderr"
run_tool "$IR/slice.ir.pl" "$usage_stdout" "$usage_stderr" compile extra-arg
check_rejection "usage/compile-extra" 2 cli usage \
    "$usage_stdout" "$usage_stderr"

usage_stdout="$SCRATCH/usage/run-extra.stdout"
usage_stderr="$SCRATCH/usage/run-extra.stderr"
run_tool "$base_program" "$usage_stdout" "$usage_stderr" run extra-arg
check_rejection "usage/run-extra" 2 cli usage \
    "$usage_stdout" "$usage_stderr"

compile_stdout1="$SCRATCH/determinism/compile.run1.stdout"
compile_stderr1="$SCRATCH/determinism/compile.run1.stderr"
run_tool "$IR/slice.ir.pl" "$compile_stdout1" "$compile_stderr1" compile
compile_status1=$RUN_STATUS
compile_stdout2="$SCRATCH/determinism/compile.run2.stdout"
compile_stderr2="$SCRATCH/determinism/compile.run2.stderr"
run_tool "$IR/slice.ir.pl" "$compile_stdout2" "$compile_stderr2" compile
compile_status2=$RUN_STATUS
if [ "$compile_status1" -ne 0 ] || [ "$compile_status2" -ne 0 ]; then
    fail_case "determinism/compile/status" \
        "expected two status-0 runs, got $compile_status1 and $compile_status2"
fi
if [ -s "$compile_stderr1" ] || [ -s "$compile_stderr2" ]; then
    fail_case "determinism/compile/stderr" "expected zero bytes"
fi
if ! cmp "$compile_stdout1" "$compile_stdout2" || \
        ! cmp "$compile_stdout1" "$PROGRAM/slice.program.pl"; then
    fail_case "determinism/compile/stdout" "fresh runs differ"
fi
pass_case "determinism/compile"

run_stdout1="$SCRATCH/determinism/run.run1.stdout"
run_stderr1="$SCRATCH/determinism/run.run1.stderr"
run_tool "$base_program" "$run_stdout1" "$run_stderr1" run
run_status1=$RUN_STATUS
run_stdout2="$SCRATCH/determinism/run.run2.stdout"
run_stderr2="$SCRATCH/determinism/run.run2.stderr"
run_tool "$base_program" "$run_stdout2" "$run_stderr2" run
run_status2=$RUN_STATUS
if [ "$run_status1" -ne 0 ] || [ "$run_status2" -ne 0 ]; then
    fail_case "determinism/run/status" \
        "expected two status-0 runs, got $run_status1 and $run_status2"
fi
if [ -s "$run_stderr1" ] || [ -s "$run_stderr2" ]; then
    fail_case "determinism/run/stderr" "expected zero bytes"
fi
if ! cmp "$run_stdout1" "$run_stdout2" || \
        ! cmp "$run_stdout1" "$RESULT/slice.result.pl"; then
    fail_case "determinism/run/stdout" "fresh runs differ"
fi
pass_case "determinism/run"

red_input="$RED/cycle-self-loop.program.pl"
red_stdout1="$SCRATCH/determinism/red.run1.stdout"
red_stderr1="$SCRATCH/determinism/red.run1.stderr"
run_tool "$red_input" "$red_stdout1" "$red_stderr1" run
red_status1=$RUN_STATUS
red_stdout2="$SCRATCH/determinism/red.run2.stdout"
red_stderr2="$SCRATCH/determinism/red.run2.stderr"
run_tool "$red_input" "$red_stdout2" "$red_stderr2" run
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
if ! command grep -Eq '^ir_tool_error\(run,cycle,.*\)\.$' \
        "$red_stderr1"; then
    fail_case "determinism/red/class" "expected cycle rejection"
fi
pass_case "determinism/red"

if [ "$PASS_COUNT" -ne "$EXPECTED_PASS_COUNT" ]; then
    fail_case "harness/pass-count" \
        "expected $EXPECTED_PASS_COUNT, got $PASS_COUNT"
fi
printf 'SUMMARY: %s passed, 0 failed\n' "$PASS_COUNT"
