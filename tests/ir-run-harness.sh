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
        ! [ -d tests/fixtures/lower/green ] || \
        ! [ -d tests/fixtures/ir/green ] || \
        ! [ -f tests/fixtures/ir/red/envelope-v1-record.pl ]; then
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
LOWER_GREEN="$ROOT/tests/fixtures/lower/green"
IR_GREEN="$ROOT/tests/fixtures/ir/green"
IR_RED="$ROOT/tests/fixtures/ir/red"
SCRATCH="$ROOT/.scratch/ir-run-harness.$$"
PASS_COUNT=0
RUN_STATUS=0
EXPECTED_PASS_COUNT=175

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

run_tool_with_limits() {
    local seconds max_kib input stdout_path stderr_path
    seconds=$1
    max_kib=$2
    input=$3
    stdout_path=$4
    stderr_path=$5
    shift 5

    if (
        ulimit -v "$max_kib" || exit 125
        timeout "$seconds" "$SWIPL" -q -f none -F none \
            -s "$ROOT/src/prolog/ir_tool.pl" -g main -t 'halt(9)' -- "$@" \
            <"$input" >"$stdout_path" 2>"$stderr_path"
    ); then
        RUN_STATUS=0
    else
        RUN_STATUS=$?
    fi
}

check_rejection() {
    local label expected_status expected_stage expected_class stdout_path stderr_path
    local expected_line line_count
    label=$1
    expected_status=$2
    expected_stage=$3
    expected_class=$4
    stdout_path=$5
    stderr_path=$6
    expected_line=${7-}

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
    if [ -n "$expected_line" ] && \
            ! printf '%s\n' "$expected_line" | cmp - "$stderr_path"; then
        fail_case "$label/detail" "stderr differs from exact expected line"
    fi
    pass_case "$label"
}

extract_result_digest() {
    local label result_path line_count line
    label=$1
    result_path=$2

    line_count=$(command grep -Ec \
        "^program\\(sha256\\('[0-9a-f]{64}'\\)\\)\\.$" \
        "$result_path" || :)
    if [ "$line_count" -ne 1 ]; then
        fail_case "$label/digest-line" \
            "expected one forced-quoted lowercase SHA-256 line, got $line_count"
    fi
    line=$(command grep -E \
        "^program\\(sha256\\('[0-9a-f]{64}'\\)\\)\\.$" \
        "$result_path" || :)
    printf '%s\n' "${line:16:64}"
}

check_result_digest() {
    local label program_path result_path sha_output expected actual
    label=$1
    program_path=$2
    result_path=$3

    if ! sha_output=$(sha256sum "$program_path"); then
        fail_case "$label/sha256sum" "could not hash program bytes"
    fi
    expected=${sha_output%% *}
    actual=$(extract_result_digest "$label" "$result_path")
    if [ "$actual" != "$expected" ]; then
        fail_case "$label/digest" \
            "expected $expected from program bytes, got $actual"
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
if [ "$#" -ne 4 ]; then
    fail_case "fixtures/count" "expected 4 DRS goldens, got $#"
fi
set -- "$IR"/*.ir.pl
if [ "$#" -ne 4 ]; then
    fail_case "fixtures/count" "expected 4 IR goldens, got $#"
fi
set -- "$PROGRAM"/*.program.pl
if [ "$#" -ne 4 ]; then
    fail_case "fixtures/count" "expected 4 program goldens, got $#"
fi
set -- "$RESULT"/*.result.pl
if [ "$#" -ne 4 ]; then
    fail_case "fixtures/count" "expected 4 result goldens, got $#"
fi
set -- "$GREEN"/*.program.pl
if [ "$#" -ne 13 ]; then
    fail_case "fixtures/count" "expected 13 hand-green programs, got $#"
fi
set -- "$GREEN"/*.result.pl
if [ "$#" -ne 13 ]; then
    fail_case "fixtures/count" "expected 13 hand-green results, got $#"
fi
set -- "$RED"/*.program.pl
if [ "$#" -ne 52 ]; then
    fail_case "fixtures/count" "expected 52 red programs, got $#"
fi
set -- "$LOWER_GREEN"/*.ir.pl
if [ "$#" -ne 21 ]; then
    fail_case "fixtures/count" "expected 21 lower IR goldens, got $#"
fi
set -- "$LOWER_GREEN"/*.program.pl
if [ "$#" -ne 21 ]; then
    fail_case "fixtures/count" "expected 21 lower program goldens, got $#"
fi
set -- "$LOWER_GREEN"/*.result.pl
if [ "$#" -ne 21 ]; then
    fail_case "fixtures/count" "expected 21 lower result goldens, got $#"
fi
pass_case "fixtures/count"

rm -rf "$SCRATCH"
mkdir -p "$SCRATCH/chain" "$SCRATCH/green" "$SCRATCH/red" \
    "$SCRATCH/stage-pin" "$SCRATCH/scratch" "$SCRATCH/usage" \
    "$SCRATCH/probes" "$SCRATCH/determinism"
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
    check_result_digest "digest/chain/$stem" \
        "$PROGRAM/$stem.program.pl" "$RESULT/$stem.result.pl"
    pass_case "chain/$stem"
done

# Lower-chain pin: the lower fixture directory owns CLI-generated IR, program,
# and result goldens. Compiling and running every committed chain proves the
# covered NAF, wh, transitive, relation, property, branch, exception, and
# alternative records reach the shipped v3 kernel.
for ir in "$LOWER_GREEN"/*.ir.pl; do
    name=${ir##*/}
    stem=${name%.ir.pl}
    expected_program="$LOWER_GREEN/$stem.program.pl"
    expected_result="$LOWER_GREEN/$stem.result.pl"

    compile_stdout="$SCRATCH/chain/lower-$stem.compile.stdout"
    compile_stderr="$SCRATCH/chain/lower-$stem.compile.stderr"
    run_tool "$ir" "$compile_stdout" "$compile_stderr" compile
    if [ "$RUN_STATUS" -ne 0 ]; then
        fail_case "chain/lower/$stem/compile-status" \
            "expected 0, got $RUN_STATUS"
    fi
    if [ -s "$compile_stderr" ]; then
        fail_case "chain/lower/$stem/compile-stderr" "expected zero bytes"
    fi
    if ! cmp "$compile_stdout" "$expected_program"; then
        fail_case "chain/lower/$stem/compile-bytes" \
            "output differs from program golden"
    fi

    run_stdout="$SCRATCH/chain/lower-$stem.run.stdout"
    run_stderr="$SCRATCH/chain/lower-$stem.run.stderr"
    run_tool "$compile_stdout" "$run_stdout" "$run_stderr" run
    if [ "$RUN_STATUS" -ne 0 ]; then
        fail_case "chain/lower/$stem/run-status" \
            "expected 0, got $RUN_STATUS"
    fi
    if [ -s "$run_stderr" ]; then
        fail_case "chain/lower/$stem/run-stderr" "expected zero bytes"
    fi
    if ! cmp "$run_stdout" "$expected_result"; then
        fail_case "chain/lower/$stem/run-bytes" \
            "output differs from result golden"
    fi

    rerun_compile_stdout="$SCRATCH/determinism/lower-$stem.compile.stdout"
    rerun_compile_stderr="$SCRATCH/determinism/lower-$stem.compile.stderr"
    run_tool "$ir" "$rerun_compile_stdout" "$rerun_compile_stderr" compile
    if [ "$RUN_STATUS" -ne 0 ]; then
        fail_case "chain/lower/$stem/compile-determinism-status" \
            "expected 0, got $RUN_STATUS"
    fi
    if [ -s "$rerun_compile_stderr" ]; then
        fail_case "chain/lower/$stem/compile-determinism-stderr" \
            "expected zero bytes"
    fi
    if ! cmp "$compile_stdout" "$rerun_compile_stdout"; then
        fail_case "chain/lower/$stem/compile-determinism-bytes" \
            "fresh runs differ"
    fi

    rerun_run_stdout="$SCRATCH/determinism/lower-$stem.run.stdout"
    rerun_run_stderr="$SCRATCH/determinism/lower-$stem.run.stderr"
    run_tool "$rerun_compile_stdout" "$rerun_run_stdout" \
        "$rerun_run_stderr" run
    if [ "$RUN_STATUS" -ne 0 ]; then
        fail_case "chain/lower/$stem/run-determinism-status" \
            "expected 0, got $RUN_STATUS"
    fi
    if [ -s "$rerun_run_stderr" ]; then
        fail_case "chain/lower/$stem/run-determinism-stderr" \
            "expected zero bytes"
    fi
    if ! cmp "$run_stdout" "$rerun_run_stdout"; then
        fail_case "chain/lower/$stem/run-determinism-bytes" \
            "fresh runs differ"
    fi

    check_result_digest "digest/lower/$stem" \
        "$expected_program" "$expected_result"
    pass_case "chain/lower/$stem"
done

if ! command grep -Fq \
        "rule_id(sentence(3),clause(1),branch(1))" \
        "$LOWER_GREEN/m6-disjunction.result.pl"; then
    fail_case "answer/lower-m6-disjunction-branch" \
        "expected generated branch(1) certificate rule ID"
fi
pass_case "answer/lower-m6-disjunction-branch"

mixed_ir="$LOWER_GREEN/m6-disjunction-mixed.ir.pl"
if ! command grep -Fq \
        "branch(1)),pred(recover,[var(1)]),body([pred(patient,[var(1)]),pred(wait,[var(1)])])" \
        "$mixed_ir" || \
        ! command grep -Fq \
        "branch(2)),pred(recover,[var(1)]),body([pred(patient,[var(1)]),pred(sleep,[var(1)]),pred(cough,[var(1)])])" \
        "$mixed_ir" || \
        ! command grep -Fq \
        "branch(3)),pred(recover,[var(1)]),body([pred(patient,[var(1)]),pred(smoke,[var(1)])])" \
        "$mixed_ir"; then
    fail_case "answer/lower-m6-disjunction-mixed" \
        "expected left-to-right DNF branches without conjunction flattening"
fi
pass_case "answer/lower-m6-disjunction-mixed"

labeled_leaf="naf(exception_id(rule(rule_id(sentence(3),clause(1))),literal(2)),pred(smoker,[named('John')]))"
if ! command grep -Fq "$labeled_leaf" \
        "$LOWER_GREEN/m6-labeled-exception.result.pl"; then
    fail_case "answer/lower-m6-labeled-exception" \
        "expected labeled absence certificate leaf"
fi
pass_case "answer/lower-m6-labeled-exception"

composition_program="$LOWER_GREEN/m6-disjunction-exception.program.pl"
if ! command grep -Fq "closed_world(exception_id(" \
        "$composition_program" || \
        ! command grep -Fq \
        "rule_id(sentence(2),clause(1),branch(1))" \
        "$composition_program" || \
        ! command grep -Fq \
        "rule_id(sentence(2),clause(1),branch(2))" \
        "$composition_program" || \
        ! command grep -Fq "$labeled_leaf" \
        "$LOWER_GREEN/m6-disjunction-exception.result.pl"; then
    fail_case "answer/lower-m6-disjunction-exception" \
        "expected split exception definitions and labeled absence leaf"
fi
pass_case "answer/lower-m6-disjunction-exception"

alternative_result="$LOWER_GREEN/m6-alternative-set.result.pl"
if ! command grep -Fq \
        "members([pred(offer,[var(1),named('Mary')]),pred(arrange,[var(1),named('Mary')])])" \
        "$alternative_result" || \
        ! command grep -Fq "satisfaction(any_member)" \
        "$alternative_result" || \
        ! command grep -Fq "exclusivity(not_asserted)" \
        "$alternative_result" || \
        ! command grep -Fq "exhaustiveness(not_asserted)" \
        "$alternative_result"; then
    fail_case "answer/lower-m6-alternative-set" \
        "expected record-visible any-member alternative metadata"
fi
pass_case "answer/lower-m6-alternative-set"

if ! command grep -Fq \
        "rule_id(sentence(3),clause(1),branch(64))" \
        "$LOWER_GREEN/m6-disjunction-cap.result.pl"; then
    fail_case "answer/lower-m6-disjunction-cap" \
        "expected accepted cap boundary branch(64) proof"
fi
pass_case "answer/lower-m6-disjunction-cap"

root_alternative_line="alternative_set(alternative_set_id(sentence(1),clause(1)),members([pred(offer,[named('John'),named('Mary')]),pred(arrange,[named('John'),named('Mary')])]),body([]),satisfaction(any_member),exclusivity(not_asserted),exhaustiveness(not_asserted),source(sentence(1),tokens([2,5])))."
if ! command grep -Fxq "$root_alternative_line" \
        "$LOWER_GREEN/m6-root-alternative-order.ir.pl"; then
    fail_case "answer/lower-m6-root-alternative-order" \
        "expected exact left-then-right root member order"
fi
pass_case "answer/lower-m6-root-alternative-order"

first_declaration="closed_world(exception_id(rule(rule_id(sentence(4),clause(1))),literal(2)),affects(rule_id(sentence(4),clause(1))),predicate_key(smoker,arity(1)))."
second_declaration="closed_world(exception_id(rule(rule_id(sentence(5),clause(1))),literal(2)),affects(rule_id(sentence(5),clause(1))),predicate_key(sleeper,arity(1)))."
two_declaration_ir="$LOWER_GREEN/m6-two-generated-declarations.ir.pl"
if [ "$(command grep -n -Fx "$first_declaration" "$two_declaration_ir" || :)" != \
        "4:$first_declaration" ] || \
        [ "$(command grep -n -Fx "$second_declaration" "$two_declaration_ir" || :)" != \
        "5:$second_declaration" ]; then
    fail_case "answer/lower-m6-two-generated-declarations" \
        "expected exact ordered closed-world rows"
fi
pass_case "answer/lower-m6-two-generated-declarations"

composition_ir="$LOWER_GREEN/m6-branch-alternative-composition.ir.pl"
composition_first="alternative_set(alternative_set_id(sentence(3),clause(1),branch(1)),members([pred(offer,[var(1),named('Mary')]),pred(arrange,[var(1),named('Mary')])]),body([pred(patient,[var(1)]),pred(wait,[var(1)])]),satisfaction(any_member),exclusivity(not_asserted),exhaustiveness(not_asserted),source(sentence(3),tokens([2,4,8,11])))."
composition_second="alternative_set(alternative_set_id(sentence(3),clause(1),branch(2)),members([pred(offer,[var(1),named('Mary')]),pred(arrange,[var(1),named('Mary')])]),body([pred(patient,[var(1)]),pred(sleep,[var(1)])]),satisfaction(any_member),exclusivity(not_asserted),exhaustiveness(not_asserted),source(sentence(3),tokens([2,7,8,11])))."
composition_count=$(command grep -c '^alternative_set(' "$composition_ir" || :)
if [ "$composition_count" -ne 2 ] || \
        ! command grep -Fxq "$composition_first" "$composition_ir" || \
        ! command grep -Fxq "$composition_second" "$composition_ir"; then
    fail_case "answer/lower-m6-branch-alternative-composition" \
        "expected exactly two ordered branched alternative rows"
fi
pass_case "answer/lower-m6-branch-alternative-composition"

nary_ir="$IR_GREEN/nary-binary.pl"
nary_validate_stdout="$SCRATCH/chain/nary-binary.validate.stdout"
nary_validate_stderr="$SCRATCH/chain/nary-binary.validate.stderr"
run_tool "$nary_ir" "$nary_validate_stdout" "$nary_validate_stderr" validate
if [ "$RUN_STATUS" -ne 0 ]; then
    fail_case "chain/nary-binary/validate-status" \
        "expected 0, got $RUN_STATUS"
fi
if [ -s "$nary_validate_stdout" ] || [ -s "$nary_validate_stderr" ]; then
    fail_case "chain/nary-binary/validate-streams" "expected zero bytes"
fi
nary_compile_stdout="$SCRATCH/chain/nary-binary.compile.stdout"
nary_compile_stderr="$SCRATCH/chain/nary-binary.compile.stderr"
run_tool "$nary_ir" "$nary_compile_stdout" "$nary_compile_stderr" compile
if [ "$RUN_STATUS" -ne 0 ]; then
    fail_case "chain/nary-binary/compile-status" \
        "expected 0, got $RUN_STATUS"
fi
if [ -s "$nary_compile_stderr" ]; then
    fail_case "chain/nary-binary/compile-stderr" "expected zero bytes"
fi
if ! cmp "$nary_compile_stdout" "$GREEN/nary-binary.program.pl"; then
    fail_case "chain/nary-binary/compile-bytes" \
        "output differs from program golden"
fi
nary_run_stdout="$SCRATCH/chain/nary-binary.run.stdout"
nary_run_stderr="$SCRATCH/chain/nary-binary.run.stderr"
run_tool "$nary_compile_stdout" "$nary_run_stdout" "$nary_run_stderr" run
if [ "$RUN_STATUS" -ne 0 ]; then
    fail_case "chain/nary-binary/run-status" "expected 0, got $RUN_STATUS"
fi
if [ -s "$nary_run_stderr" ]; then
    fail_case "chain/nary-binary/run-stderr" "expected zero bytes"
fi
if ! cmp "$nary_run_stdout" "$GREEN/nary-binary.result.pl"; then
    fail_case "chain/nary-binary/run-bytes" "output differs from result golden"
fi
pass_case "chain/nary-binary"

ternary_ir="$IR_GREEN/nary-ternary.pl"
ternary_validate_stdout="$SCRATCH/chain/nary-ternary.validate.stdout"
ternary_validate_stderr="$SCRATCH/chain/nary-ternary.validate.stderr"
run_tool "$ternary_ir" "$ternary_validate_stdout" "$ternary_validate_stderr" validate
if [ "$RUN_STATUS" -ne 0 ]; then
    fail_case "chain/nary-ternary/validate-status" \
        "expected 0, got $RUN_STATUS"
fi
if [ -s "$ternary_validate_stdout" ] || [ -s "$ternary_validate_stderr" ]; then
    fail_case "chain/nary-ternary/validate-streams" "expected zero bytes"
fi
ternary_compile_stdout="$SCRATCH/chain/nary-ternary.compile.stdout"
ternary_compile_stderr="$SCRATCH/chain/nary-ternary.compile.stderr"
run_tool "$ternary_ir" "$ternary_compile_stdout" "$ternary_compile_stderr" compile
if [ "$RUN_STATUS" -ne 0 ]; then
    fail_case "chain/nary-ternary/compile-status" \
        "expected 0, got $RUN_STATUS"
fi
if [ -s "$ternary_compile_stderr" ]; then
    fail_case "chain/nary-ternary/compile-stderr" "expected zero bytes"
fi
if ! cmp "$ternary_compile_stdout" "$GREEN/nary-ternary.program.pl"; then
    fail_case "chain/nary-ternary/compile-bytes" \
        "output differs from program golden"
fi
ternary_run_stdout="$SCRATCH/chain/nary-ternary.run.stdout"
ternary_run_stderr="$SCRATCH/chain/nary-ternary.run.stderr"
run_tool "$ternary_compile_stdout" "$ternary_run_stdout" "$ternary_run_stderr" run
if [ "$RUN_STATUS" -ne 0 ]; then
    fail_case "chain/nary-ternary/run-status" "expected 0, got $RUN_STATUS"
fi
if [ -s "$ternary_run_stderr" ]; then
    fail_case "chain/nary-ternary/run-stderr" "expected zero bytes"
fi
if ! cmp "$ternary_run_stdout" "$GREEN/nary-ternary.result.pl"; then
    fail_case "chain/nary-ternary/run-bytes" "output differs from result golden"
fi
pass_case "chain/nary-ternary"

cross_arity_ir="$IR_GREEN/nary-same-name-cross-arity.pl"
cross_arity_validate_stdout="$SCRATCH/chain/nary-same-name-cross-arity.validate.stdout"
cross_arity_validate_stderr="$SCRATCH/chain/nary-same-name-cross-arity.validate.stderr"
run_tool "$cross_arity_ir" "$cross_arity_validate_stdout" \
    "$cross_arity_validate_stderr" validate
if [ "$RUN_STATUS" -ne 0 ]; then
    fail_case "chain/nary-same-name-cross-arity/validate-status" \
        "expected 0, got $RUN_STATUS"
fi
if [ -s "$cross_arity_validate_stdout" ] || \
        [ -s "$cross_arity_validate_stderr" ]; then
    fail_case "chain/nary-same-name-cross-arity/validate-streams" \
        "expected zero bytes"
fi
cross_arity_compile_stdout="$SCRATCH/chain/nary-same-name-cross-arity.compile.stdout"
cross_arity_compile_stderr="$SCRATCH/chain/nary-same-name-cross-arity.compile.stderr"
run_tool "$cross_arity_ir" "$cross_arity_compile_stdout" \
    "$cross_arity_compile_stderr" compile
if [ "$RUN_STATUS" -ne 0 ]; then
    fail_case "chain/nary-same-name-cross-arity/compile-status" \
        "expected 0, got $RUN_STATUS"
fi
if [ -s "$cross_arity_compile_stderr" ]; then
    fail_case "chain/nary-same-name-cross-arity/compile-stderr" \
        "expected zero bytes"
fi
if ! cmp "$cross_arity_compile_stdout" \
        "$GREEN/nary-same-name-cross-arity.program.pl"; then
    fail_case "chain/nary-same-name-cross-arity/compile-bytes" \
        "output differs from program golden"
fi
cross_arity_run_stdout="$SCRATCH/chain/nary-same-name-cross-arity.run.stdout"
cross_arity_run_stderr="$SCRATCH/chain/nary-same-name-cross-arity.run.stderr"
run_tool "$cross_arity_compile_stdout" "$cross_arity_run_stdout" \
    "$cross_arity_run_stderr" run
if [ "$RUN_STATUS" -ne 0 ]; then
    fail_case "chain/nary-same-name-cross-arity/run-status" \
        "expected 0, got $RUN_STATUS"
fi
if [ -s "$cross_arity_run_stderr" ]; then
    fail_case "chain/nary-same-name-cross-arity/run-stderr" \
        "expected zero bytes"
fi
if ! cmp "$cross_arity_run_stdout" \
        "$GREEN/nary-same-name-cross-arity.result.pl"; then
    fail_case "chain/nary-same-name-cross-arity/run-bytes" \
        "output differs from result golden"
fi
pass_case "chain/nary-same-name-cross-arity"

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
    if [ "$stem" = naf-open ] && \
            ! command grep -Fq "named('café patient')" "$input"; then
        fail_case "green/$stem/multibyte" \
            "expected committed quoted multibyte constant"
    fi
    if [ "$stem" = wide-join ]; then
        run_tool_with_limits 30 98304 \
            "$input" "$stdout_path" "$stderr_path" run
    else
        run_tool "$input" "$stdout_path" "$stderr_path" run
    fi
    if [ "$RUN_STATUS" -ne 0 ]; then
        fail_case "green/$stem/status" "expected 0, got $RUN_STATUS"
    fi
    if [ -s "$stderr_path" ]; then
        fail_case "green/$stem/stderr" "expected zero bytes"
    fi
    if ! cmp "$stdout_path" "$expected"; then
        fail_case "green/$stem/bytes" "run output differs from golden"
    fi

    rerun_stdout="$SCRATCH/determinism/$stem.stdout"
    rerun_stderr="$SCRATCH/determinism/$stem.stderr"
    if [ "$stem" = wide-join ]; then
        run_tool_with_limits 30 98304 \
            "$input" "$rerun_stdout" "$rerun_stderr" run
    else
        run_tool "$input" "$rerun_stdout" "$rerun_stderr" run
    fi
    if [ "$RUN_STATUS" -ne 0 ]; then
        fail_case "green/$stem/determinism-status" \
            "expected 0, got $RUN_STATUS"
    fi
    if [ -s "$rerun_stderr" ]; then
        fail_case "green/$stem/determinism-stderr" "expected zero bytes"
    fi
    if ! cmp "$stdout_path" "$rerun_stdout"; then
        fail_case "green/$stem/determinism-bytes" "fresh runs differ"
    fi

    check_result_digest "digest/green/$stem" "$input" "$expected"
    pass_case "green/$stem"
done

if ! command grep -q 'naf(pred(' "$GREEN/naf-open.result.pl"; then
    fail_case "answer/naf-open-leaf" "expected a positional NAF proof leaf"
fi
pass_case "answer/naf-open-leaf"

if ! command grep -Fq "naf(pred(smoke,[named('John')]))" \
        "$LOWER_GREEN/p13-naf-intransitive.result.pl"; then
    fail_case "answer/lower-p13-naf-leaf" \
        "expected the lowered P13 rule's positional NAF leaf"
fi
pass_case "answer/lower-p13-naf-leaf"

p10_answer_line="answer(query_id(sentence(4),clause(1)),wh(who),pred(recover,[var(1)]),answers([pred(recover,[named('John')])]))."
if ! command grep -Fxq "$p10_answer_line" \
        "$LOWER_GREEN/p10-wh-prefix.result.pl"; then
    fail_case "answer/lower-p10-wh" \
        "expected the lowered who query's complete John answer"
fi
pass_case "answer/lower-p10-wh"

wh_order_line="answer(query_id(sentence(4),clause(1)),wh(who),pred(recover,[var(1)]),answers([pred(recover,[named('z z')]),pred(recover,[named(a)])]))."
if ! command grep -Fxq "$wh_order_line" \
        "$GREEN/wh-multi-order.result.pl"; then
    fail_case "answer/wh-byte-order" \
        "expected quoted z-space-z answer before unquoted a"
fi
pass_case "answer/wh-byte-order"

competing_input="$GREEN/competing-witness.program.pl"
competing_expected="$GREEN/competing-witness.result.pl"
competing_first="$SCRATCH/green/competing-witness.stdout"
competing_rerun_stdout="$SCRATCH/determinism/competing-witness.stdout"
competing_rerun_stderr="$SCRATCH/determinism/competing-witness.stderr"
run_tool "$competing_input" "$competing_rerun_stdout" \
    "$competing_rerun_stderr" run
if [ "$RUN_STATUS" -ne 0 ]; then
    fail_case "determinism/competing-witness/status" \
        "expected 0, got $RUN_STATUS"
fi
if [ -s "$competing_rerun_stderr" ]; then
    fail_case "determinism/competing-witness/stderr" "expected zero bytes"
fi
if ! cmp "$competing_first" "$competing_rerun_stdout" || \
        ! cmp "$competing_rerun_stdout" "$competing_expected"; then
    fail_case "determinism/competing-witness/stdout" \
        "fresh runs or golden differ"
fi
pass_case "determinism/competing-witness"

run_committed_red() {
    local name expected_class expected_line stdout_path stderr_path
    local first_status rerun_stdout rerun_stderr
    name=$1
    expected_class=$2
    expected_line=${3-}
    stdout_path="$SCRATCH/red/$name.stdout"
    stderr_path="$SCRATCH/red/$name.stderr"
    run_tool "$RED/$name.program.pl" "$stdout_path" "$stderr_path" run
    first_status=$RUN_STATUS
    check_rejection "red/$name" 1 run "$expected_class" \
        "$stdout_path" "$stderr_path" "$expected_line"

    rerun_stdout="$SCRATCH/determinism/$name.red.stdout"
    rerun_stderr="$SCRATCH/determinism/$name.red.stderr"
    run_tool "$RED/$name.program.pl" "$rerun_stdout" "$rerun_stderr" run
    if [ "$RUN_STATUS" -ne "$first_status" ]; then
        fail_case "red/$name/determinism-status" \
            "fresh runs differ: $first_status and $RUN_STATUS"
    fi
    if ! cmp "$stdout_path" "$rerun_stdout" || \
            ! cmp "$stderr_path" "$rerun_stderr"; then
        fail_case "red/$name/determinism-bytes" "fresh runs differ"
    fi
}

run_committed_red ordering-branch-gap ordering \
    'ir_tool_error(run,ordering,term(6,branch_sequence(expected(2),found(3)))).'
run_committed_red ordering-branch-singleton ordering \
    'ir_tool_error(run,ordering,term(5,branch_group_singleton(rule,pair(3,1)))).'
run_committed_red ordering-branch-head-mismatch ordering \
    'ir_tool_error(run,ordering,term(5,branch_payload_mismatch(rule,pair(3,1)))).'
run_committed_red ordering-alternative-branch-members-mismatch ordering \
    'ir_tool_error(run,ordering,term(5,branch_payload_mismatch(alternative_set,pair(3,1)))).'
run_committed_red exception-undeclared exception \
    'ir_tool_error(run,exception,term(5,undeclared_labeled_target(exception_id(rule(rule_id(sentence(3),clause(1))),literal(2))))).'
run_committed_red shape-closed-world shape \
    'ir_tool_error(run,shape,term(4,closed_world)).'
run_committed_red exception-affected-mismatch exception \
    'ir_tool_error(run,exception,term(4,affected_rule_mismatch(rule_id(sentence(3),clause(1)),rule_id(sentence(2),clause(1))))).'
run_committed_red exception-target-not-defined exception \
    'ir_tool_error(run,exception,term(4,target_not_defined(predicate_key(cough,arity(1))))).'
run_committed_red exception-target-mismatch exception \
    'ir_tool_error(run,exception,term(6,undeclared_labeled_target(exception_id(rule(rule_id(sentence(3),clause(1))),literal(2))))).'
run_committed_red exception-unused-declaration exception \
    'ir_tool_error(run,exception,term(4,unused_declaration(exception_id(rule(rule_id(sentence(3),clause(1))),literal(2))))).'
run_committed_red shape-alternative-policy shape \
    'ir_tool_error(run,shape,term(5,alternative_set)).'
run_committed_red safety-alternative-member safety \
    'ir_tool_error(run,safety,term(5,alternative_member_var_not_in_body(1))).'
run_committed_red shape-alternative-members shape \
    'ir_tool_error(run,shape,term(5,alternative_set)).'
run_committed_red alternative-set-duplicate-members shape \
    'ir_tool_error(run,shape,term(4,alternative_set)).'
run_committed_red alternative-set-member-scope-order scope \
    'ir_tool_error(run,scope,term(3,variable_sequence(expected(1),found(2),occurrence(1)))).'
run_committed_red alternative-set-body-naf safety \
    'ir_tool_error(run,safety,term(3,alternative_set_naf(2))).'
run_committed_red branch-wrapper-float shape \
    'ir_tool_error(run,shape,term(5,branch_id)).'
run_committed_red branch-wrapper-atom shape \
    'ir_tool_error(run,shape,term(5,branch_id)).'
run_committed_red exception-label-position exception \
    'ir_tool_error(run,exception,term(6,label_position(expected(exception_id(rule(rule_id(sentence(3),clause(1))),literal(2))),found(exception_id(rule(rule_id(sentence(3),clause(1))),literal(1)))))).'
run_committed_red exception-label-target-swap exception \
    'ir_tool_error(run,exception,term(8,undeclared_labeled_target(exception_id(rule(rule_id(sentence(4),clause(1))),literal(2))))).'
run_committed_red exception-declaration-order exception \
    'ir_tool_error(run,exception,term(5,declaration_order_after(key(4,1,0,3)))).'
run_committed_red cycle-labeled-mixed cycle \
    'ir_tool_error(run,cycle,term(6,body_literal(2,signed_dependency(naf,pred(r,1),pred(p,1))))).'
run_committed_red cycle-self-loop cycle
run_committed_red cycle-signed-transitive cycle \
    'ir_tool_error(run,cycle,term(5,body_literal(2,signed_dependency(naf,pred(r,1),pred(p,1))))).'
run_committed_red document-float shape
run_committed_red envelope-missing-document envelope
run_committed_red envelope-trailing-after-goal envelope
run_committed_red envelope-v1-record envelope \
    'ir_tool_error(run,envelope,term(1,expected(cnl_program_record(3)))).'
run_committed_red envelope-wrong-header envelope \
    'ir_tool_error(run,envelope,term(1,expected(cnl_program_record(3)))).'
run_committed_red identity-fact-with-body identity
run_committed_red identity-rule-empty-body identity \
    'ir_tool_error(run,identity,term(3,id_kind(fact,rule))).'
run_committed_red ordering-duplicate-id ordering
run_committed_red query-count-two query_count
run_committed_red query-count-two-mixed query_count \
    'ir_tool_error(run,query_count,count(2,second_term(4))).'
run_committed_red query-count-zero query_count
run_committed_red safety-head-uncovered safety
run_committed_red safety-naf-order-interleave safety \
    'ir_tool_error(run,safety,term(3,positive_after_naf(2))).'
run_committed_red safety-naf-var-uncovered safety \
    'ir_tool_error(run,safety,term(3,naf_var_not_in_positive_body(2))).'
run_committed_red scope-naf-non-dense scope \
    'ir_tool_error(run,scope,term(3,variable_sequence(expected(2),found(3),occurrence(3)))).'
run_committed_red scope-non-dense scope
run_committed_red section-order-fact-after-rule section_order
run_committed_red shape-naf-argument-position shape \
    'ir_tool_error(run,shape,term(3,clause)).'
run_committed_red shape-naf-goal-position shape \
    'ir_tool_error(run,shape,term(3,goal)).'
run_committed_red shape-naf-head-position shape \
    'ir_tool_error(run,shape,term(3,clause)).'
run_committed_red shape-native-variable shape
run_committed_red shape-empty-args shape \
    'ir_tool_error(run,shape,term(3,clause)).'
run_committed_red shape-improper-args shape \
    'ir_tool_error(run,shape,term(3,clause)).'
run_committed_red shape-unknown-constructor shape
run_committed_red shape-wh-marker shape \
    'ir_tool_error(run,shape,term(3,goal)).'
run_committed_red shape-wh-pattern-arg shape \
    'ir_tool_error(run,shape,term(3,goal)).'
run_committed_red shape-wh-pattern-arity shape \
    'ir_tool_error(run,shape,term(3,goal)).'
run_committed_red resource-cap resource

for stem in \
        alternative-set-duplicate-members \
        alternative-set-member-scope-order \
        alternative-set-body-naf \
        exception-label-position \
        exception-label-target-swap \
        exception-declaration-order \
        branch-wrapper-float \
        branch-wrapper-atom \
        cycle-labeled-mixed; do
    pair_validate_stdout="$SCRATCH/stage-pin/$stem.pair.validate.stdout"
    pair_validate_stderr="$SCRATCH/stage-pin/$stem.pair.validate.stderr"
    pair_compile_stdout="$SCRATCH/stage-pin/$stem.pair.compile.stdout"
    pair_compile_stderr="$SCRATCH/stage-pin/$stem.pair.compile.stderr"
    pair_run_stdout="$SCRATCH/stage-pin/$stem.pair.run.stdout"
    pair_run_stderr="$SCRATCH/stage-pin/$stem.pair.run.stderr"
    pair_expected="$SCRATCH/stage-pin/$stem.pair.expected.stderr"
    pair_actual="$SCRATCH/stage-pin/$stem.pair.actual.stderr"

    run_tool "$IR_RED/$stem.pl" "$pair_validate_stdout" \
        "$pair_validate_stderr" validate
    pair_validate_status=$RUN_STATUS
    run_tool "$IR_RED/$stem.pl" "$pair_compile_stdout" \
        "$pair_compile_stderr" compile
    pair_compile_status=$RUN_STATUS
    run_tool "$RED/$stem.program.pl" "$pair_run_stdout" \
        "$pair_run_stderr" run
    pair_run_status=$RUN_STATUS
    if [ "$pair_validate_status" -ne 1 ] || \
            [ "$pair_compile_status" -ne 1 ] || \
            [ "$pair_run_status" -ne 1 ]; then
        fail_case "stage-pin/paired-errors/$stem/status" \
            "expected validate/compile/run status 1"
    fi
    if [ -s "$pair_validate_stdout" ] || \
            [ -s "$pair_compile_stdout" ] || \
            [ -s "$pair_run_stdout" ]; then
        fail_case "stage-pin/paired-errors/$stem/stdout" \
            "expected zero bytes"
    fi
    command sed \
        's/^ir_tool_error(validate,/ir_tool_error(STAGE,/' \
        "$pair_validate_stderr" >"$pair_expected"
    command sed \
        's/^ir_tool_error(compile,/ir_tool_error(STAGE,/' \
        "$pair_compile_stderr" >"$pair_actual"
    if ! cmp "$pair_expected" "$pair_actual"; then
        fail_case "stage-pin/paired-errors/$stem/compile" \
            "stderr differs after stage-only rewrite"
    fi
    command sed \
        's/^ir_tool_error(run,/ir_tool_error(STAGE,/' \
        "$pair_run_stderr" >"$pair_actual"
    if ! cmp "$pair_expected" "$pair_actual"; then
        fail_case "stage-pin/paired-errors/$stem/run" \
            "stderr differs after stage-only rewrite"
    fi
done
pass_case "stage-pin/paired-errors"

pin_stdout="$SCRATCH/stage-pin/compile-v1-envelope.stdout"
pin_stderr="$SCRATCH/stage-pin/compile-v1-envelope.stderr"
run_tool "$IR_RED/envelope-v1-record.pl" "$pin_stdout" "$pin_stderr" compile
check_rejection "stage-pin/compile-v1-envelope" 1 compile envelope \
    "$pin_stdout" "$pin_stderr" \
    'ir_tool_error(compile,envelope,term(1,expected(cnl_ir_record(3)))).'

for input in "$IR_RED"/*.pl; do
    name=${input##*/}
    stem=${name%.pl}
    validate_stdout="$SCRATCH/stage-pin/$stem.validate.stdout"
    validate_stderr="$SCRATCH/stage-pin/$stem.validate.stderr"
    compile_stdout="$SCRATCH/stage-pin/$stem.compile.stdout"
    compile_stderr="$SCRATCH/stage-pin/$stem.compile.stderr"
    expected_stderr="$SCRATCH/stage-pin/$stem.expected.stderr"

    run_tool "$input" "$validate_stdout" "$validate_stderr" validate
    validate_status=$RUN_STATUS
    if [ "$validate_status" -ne 1 ]; then
        fail_case "stage-pin/compile-ir-red-ownership/$stem" \
            "validate expected status 1, got $validate_status"
    fi
    if [ -s "$validate_stdout" ]; then
        fail_case "stage-pin/compile-ir-red-ownership/$stem" \
            "validate wrote stdout"
    fi
    validate_line_count=$(command grep -c '^' "$validate_stderr" || :)
    if [ "$validate_line_count" -ne 1 ] || \
            ! printf '%s\n' "$(<"$validate_stderr")" | \
                cmp - "$validate_stderr"; then
        fail_case "stage-pin/compile-ir-red-ownership/$stem" \
            "validate expected one LF-terminated stderr line"
    fi
    if ! command grep -q '^ir_tool_error(validate,' "$validate_stderr"; then
        fail_case "stage-pin/compile-ir-red-ownership/$stem" \
            "validate stderr has wrong stage"
    fi
    validate_rerun_stdout="$SCRATCH/determinism/$stem.stage-pin.validate.stdout"
    validate_rerun_stderr="$SCRATCH/determinism/$stem.stage-pin.validate.stderr"
    run_tool "$input" "$validate_rerun_stdout" \
        "$validate_rerun_stderr" validate
    if [ "$RUN_STATUS" -ne "$validate_status" ] || \
            ! cmp "$validate_stdout" "$validate_rerun_stdout" || \
            ! cmp "$validate_stderr" "$validate_rerun_stderr"; then
        fail_case "stage-pin/compile-ir-red-ownership/$stem" \
            "validate fresh runs differ"
    fi

    run_tool "$input" "$compile_stdout" "$compile_stderr" compile
    compile_status=$RUN_STATUS
    if [ "$compile_status" -ne 1 ]; then
        fail_case "stage-pin/compile-ir-red-ownership/$stem" \
            "compile expected status 1, got $compile_status"
    fi
    if [ -s "$compile_stdout" ]; then
        fail_case "stage-pin/compile-ir-red-ownership/$stem" \
            "compile wrote stdout"
    fi
    compile_line_count=$(command grep -c '^' "$compile_stderr" || :)
    if [ "$compile_line_count" -ne 1 ] || \
            ! printf '%s\n' "$(<"$compile_stderr")" | \
                cmp - "$compile_stderr"; then
        fail_case "stage-pin/compile-ir-red-ownership/$stem" \
            "compile expected one LF-terminated stderr line"
    fi
    if ! command sed \
            's/^ir_tool_error(validate,/ir_tool_error(compile,/' \
            "$validate_stderr" >"$expected_stderr"; then
        fail_case "stage-pin/compile-ir-red-ownership/$stem" \
            "could not derive expected compile stderr"
    fi
    if ! cmp "$expected_stderr" "$compile_stderr"; then
        fail_case "stage-pin/compile-ir-red-ownership/$stem" \
            "compile stderr differs from validate stderr after stage rewrite"
    fi
    compile_rerun_stdout="$SCRATCH/determinism/$stem.stage-pin.compile.stdout"
    compile_rerun_stderr="$SCRATCH/determinism/$stem.stage-pin.compile.stderr"
    run_tool "$input" "$compile_rerun_stdout" \
        "$compile_rerun_stderr" compile
    if [ "$RUN_STATUS" -ne "$compile_status" ] || \
            ! cmp "$compile_stdout" "$compile_rerun_stdout" || \
            ! cmp "$compile_stderr" "$compile_rerun_stderr"; then
        fail_case "stage-pin/compile-ir-red-ownership/$stem" \
            "compile fresh runs differ"
    fi
done
pass_case "stage-pin/compile-ir-red-ownership"

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

tampered_program="$SCRATCH/scratch/digest-tampered.program.pl"
tampered_result="$SCRATCH/scratch/digest-tampered.result.pl"
tampered_stderr="$SCRATCH/scratch/digest-tampered.stderr"
if ! command sed "s/docid('slice')/docid('sljce')/" \
        "$base_program" >"$tampered_program"; then
    fail_case "digest/tamper-binding/generate" \
        "could not create canonical byte tamper"
fi
if cmp -s "$tampered_program" "$base_program"; then
    fail_case "digest/tamper-binding/generate" "tamper changed no bytes"
fi
run_tool "$tampered_program" "$tampered_result" "$tampered_stderr" run
if [ "$RUN_STATUS" -ne 0 ]; then
    fail_case "digest/tamper-binding/status" \
        "expected 0, got $RUN_STATUS"
fi
if [ -s "$tampered_stderr" ]; then
    fail_case "digest/tamper-binding/stderr" "expected zero bytes"
fi
if ! tampered_sha_output=$(sha256sum "$tampered_program"); then
    fail_case "digest/tamper-binding/sha256sum" \
        "could not hash tampered bytes"
fi
tampered_expected=${tampered_sha_output%% *}
tampered_actual=$(extract_result_digest \
    "digest/tamper-binding" "$tampered_result")
committed_actual=$(extract_result_digest \
    "digest/tamper-binding/committed" "$RESULT/slice.result.pl")
if [ "$tampered_actual" != "$tampered_expected" ]; then
    fail_case "digest/tamper-binding/value" \
        "tampered result does not bind tampered program bytes"
fi
if [ "$tampered_actual" = "$committed_actual" ]; then
    fail_case "digest/tamper-binding/change" \
        "tampered and committed digests unexpectedly match"
fi
pass_case "digest/tamper-binding"

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

replay_stdout="$SCRATCH/probes/replay-list.stdout"
replay_stderr="$SCRATCH/probes/replay-list.stderr"
if "$SWIPL" -q -f none -F none -s "$ROOT/src/prolog/explanation.pl" \
        -g '(assertz(cnl_program_db:program_clause(1,rule_id(sentence(1),clause(1)),pred(q,[named(a)]),[pred(p,[named(a)])])),assertz(cnl_program_db:program_clause(2,fact_id(sentence(2),clause(1)),pred(p,[named(a)]),[])),Child=proof(pred(p,[named(a)]),fact_id(sentence(2),clause(1)),[]),Proof=proof(pred(q,[named(a)]),rule_id(sentence(1),clause(1)),weird(Child,[],x)),Store=[entry(pred(q,[named(a)]),by(rule_id(sentence(1),clause(1)),[pred(p,[named(a)])])),entry(pred(p,[named(a)]),by(fact_id(sentence(2),clause(1)),[]))],catch(explanation:replay_certificate(pred(q,[named(a)]),Proof,Store),Error,true),\+explanation:replay_children(weird(Child,[],x),Store),retractall(cnl_program_db:program_clause(_,_,_,_)),Error==explanation_invariant(replay_failed)->halt(0);halt(1))' \
        -t 'halt(9)' >"$replay_stdout" 2>"$replay_stderr"; then
    replay_status=0
else
    replay_status=$?
fi
if [ "$replay_status" -ne 0 ]; then
    fail_case "probe/replay-list-shape" "expected invariant rejection"
fi
if [ -s "$replay_stdout" ] || [ -s "$replay_stderr" ]; then
    fail_case "probe/replay-list-shape" "expected zero bytes"
fi
pass_case "probe/replay-list-shape"

replay_naf_stdout="$SCRATCH/probes/replay-naf-absence.stdout"
replay_naf_stderr="$SCRATCH/probes/replay-naf-absence.stderr"
if "$SWIPL" -q -f none -F none -s "$ROOT/src/prolog/explanation.pl" \
        -g '(assertz(cnl_program_db:program_clause(1,rule_id(sentence(1),clause(1)),pred(recover,[var(1)]),[pred(patient,[var(1)]),naf(pred(smoke,[var(1)]))])),assertz(cnl_program_db:program_clause(2,fact_id(sentence(2),clause(1)),pred(patient,[named(a)]),[])),Proof=proof(pred(recover,[named(a)]),rule_id(sentence(1),clause(1)),[proof(pred(patient,[named(a)]),fact_id(sentence(2),clause(1)),[]),naf(pred(smoke,[named(a)]))]),Store=[entry(pred(recover,[named(a)]),by(rule_id(sentence(1),clause(1)),[pred(patient,[named(a)]),naf(pred(smoke,[named(a)]))])),entry(pred(patient,[named(a)]),by(fact_id(sentence(2),clause(1)),[]))],explanation:replay_certificate(pred(recover,[named(a)]),Proof,Store),PresentStore=[entry(pred(recover,[named(a)]),by(rule_id(sentence(1),clause(1)),[pred(patient,[named(a)]),naf(pred(smoke,[named(a)]))])),entry(pred(patient,[named(a)]),by(fact_id(sentence(2),clause(1)),[])),entry(pred(smoke,[named(a)]),by(fact_id(sentence(3),clause(1)),[]))],catch(explanation:replay_certificate(pred(recover,[named(a)]),Proof,PresentStore),Error,true),retractall(cnl_program_db:program_clause(_,_,_,_)),Error==explanation_invariant(replay_failed)->halt(0);halt(1))' \
        -t 'halt(9)' >"$replay_naf_stdout" 2>"$replay_naf_stderr"; then
    replay_naf_status=0
else
    replay_naf_status=$?
fi
if [ "$replay_naf_status" -ne 0 ]; then
    fail_case "probe/replay-naf-absence" \
        "expected absent-store success and present-store rejection"
fi
if [ -s "$replay_naf_stdout" ] || [ -s "$replay_naf_stderr" ]; then
    fail_case "probe/replay-naf-absence" "expected zero bytes"
fi
pass_case "probe/replay-naf-absence"

replay_witness_stdout="$SCRATCH/probes/replay-identical-witness.stdout"
replay_witness_stderr="$SCRATCH/probes/replay-identical-witness.stderr"
if "$SWIPL" -q -f none -F none \
        -s "$ROOT/src/prolog/inference_kernel.pl" \
        -g '(Terms=[cnl_program_record(3),document(docid(d16),source_sha256('\''aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa'\''),ulex(none)),clause(fact_id(sentence(1),clause(1)),pred(patient,[named(a)]),body([])),clause(fact_id(sentence(2),clause(1)),pred(wait,[named(a)]),body([])),clause(rule_id(sentence(3),clause(1),branch(1)),pred(recover,[var(1)]),body([pred(patient,[var(1)]),pred(wait,[var(1)])])),clause(rule_id(sentence(3),clause(1),branch(2)),pred(recover,[var(1)]),body([pred(patient,[var(1)]),pred(wait,[var(1)])])),goal(query_id(sentence(4),clause(1)),pred(recover,[named(a)]))],inference_kernel:validate_program_terms(Terms,_,Clauses,_),length(Clauses,Count),inference_kernel:max_clause_stratum(Clauses,Max),setup_call_cleanup(inference_kernel:install_program(Clauses),(inference_kernel:stratified_model(Max,Count,Store),Goal=pred(recover,[named(a)]),explanation:build_certificate(Goal,Store,Proof),arg(2,Proof,FirstId),FirstId==rule_id(sentence(3),clause(1),branch(1)),explanation:replay_certificate(Goal,Proof,Store),copy_term(Proof,Swapped),setarg(2,Swapped,rule_id(sentence(3),clause(1),branch(2))),catch(explanation:replay_certificate(Goal,Swapped,Store),Error,true),Error==explanation_invariant(replay_failed)),inference_kernel:teardown_program)->halt(0);halt(1))' \
        -t 'halt(9)' >"$replay_witness_stdout" \
        2>"$replay_witness_stderr"; then
    replay_witness_status=0
else
    replay_witness_status=$?
fi
if [ "$replay_witness_status" -ne 0 ]; then
    fail_case "probe/replay-identical-witness" \
        "expected retained branch(1) replay and branch(2) rejection"
fi
if [ -s "$replay_witness_stdout" ] || \
        [ -s "$replay_witness_stderr" ]; then
    fail_case "probe/replay-identical-witness" "expected zero bytes"
fi
pass_case "probe/replay-identical-witness-valid"
pass_case "probe/replay-identical-witness-swap"

answer_yes_no_stdout="$SCRATCH/probes/answer-v3-yes-no.stdout"
answer_yes_no_stderr="$SCRATCH/probes/answer-v3-yes-no.stderr"
if "$SWIPL" -q -f none -F none -s "$ROOT/src/prolog/explanation.pl" \
        -g '(Doc=document(docid(d),source_sha256(s),ulex([])),Program=program(sha256(h)),Id=query_id(sentence(1),clause(1)),Atom=pred(p,[named(a)]),Proof=proof(Atom,fact_id(sentence(1),clause(1)),[]),explanation:validate_answer_terms([cnl_answer_record(3),Doc,Program,answer(Id,Atom,proved),Proof]),explanation:validate_answer_terms([cnl_answer_record(3),Doc,Program,answer(Id,Atom,not_proved)])->halt(0);halt(1))' \
        -t 'halt(9)' >"$answer_yes_no_stdout" \
        2>"$answer_yes_no_stderr"; then
    answer_yes_no_status=0
else
    answer_yes_no_status=$?
fi
if [ "$answer_yes_no_status" -ne 0 ]; then
    fail_case "probe/answer-v3-yes-no" \
        "expected proved and not_proved v3 records to pass"
fi
if [ -s "$answer_yes_no_stdout" ] || [ -s "$answer_yes_no_stderr" ]; then
    fail_case "probe/answer-v3-yes-no" "expected zero bytes"
fi
pass_case "probe/answer-v3-yes-no"

run_answer_alternative_tamper() {
    local name alternative stdout_path stderr_path goal status
    name=$1
    alternative=$2
    stdout_path="$SCRATCH/probes/$name.stdout"
    stderr_path="$SCRATCH/probes/$name.stderr"
    goal="(Doc=document(docid(d),source_sha256(s),ulex(none)),Program=program(sha256(h)),Id=query_id(sentence(2),clause(1)),Atom=pred(q,[named(x)]),Alternative=$alternative,catch(explanation:validate_answer_terms([cnl_answer_record(3),Doc,Program,Alternative,answer(Id,Atom,not_proved)]),Error,true),Error==explanation_invariant(generated_answer_envelope)->halt(0);halt(1))"
    if "$SWIPL" -q -f none -F none \
            -s "$ROOT/src/prolog/explanation.pl" -g "$goal" \
            -t 'halt(9)' >"$stdout_path" 2>"$stderr_path"; then
        status=0
    else
        status=$?
    fi
    if [ "$status" -ne 0 ]; then
        fail_case "probe/$name" \
            "expected generated_answer_envelope rejection"
    fi
    if [ -s "$stdout_path" ] || [ -s "$stderr_path" ]; then
        fail_case "probe/$name" "expected zero bytes"
    fi
    pass_case "probe/$name"
}

run_answer_alternative_tamper answer-alternative-duplicate-members \
    'alternative_set(alternative_set_id(sentence(1),clause(1)),members([pred(a,[named(x)]),pred(a,[named(x)])]),body([]),satisfaction(any_member),exclusivity(not_asserted),exhaustiveness(not_asserted))'
run_answer_alternative_tamper answer-alternative-policy \
    'alternative_set(alternative_set_id(sentence(1),clause(1)),members([pred(a,[named(x)]),pred(b,[named(x)])]),body([]),satisfaction(all_members),exclusivity(not_asserted),exhaustiveness(not_asserted))'
run_answer_alternative_tamper answer-alternative-member-count \
    'alternative_set(alternative_set_id(sentence(1),clause(1)),members([pred(a,[named(x)])]),body([]),satisfaction(any_member),exclusivity(not_asserted),exhaustiveness(not_asserted))'

answer_version_stdout="$SCRATCH/probes/answer-version-rejection.stdout"
answer_version_stderr="$SCRATCH/probes/answer-version-rejection.stderr"
if "$SWIPL" -q -f none -F none -s "$ROOT/src/prolog/explanation.pl" \
        -g '(Doc=document(docid(d),source_sha256(s),ulex([])),Program=program(sha256(h)),Id=query_id(sentence(1),clause(1)),Atom=pred(p,[named(a)]),Proof=proof(Atom,fact_id(sentence(1),clause(1)),[]),catch(explanation:validate_answer_terms([cnl_answer_record(2),Doc,Program,answer(Id,Atom,proved),Proof]),Error2,true),catch(explanation:validate_answer_terms([cnl_answer_record(4),Doc,Program,answer(Id,Atom,not_proved)]),Error4,true),Error2==explanation_invariant(generated_answer_envelope),Error4==explanation_invariant(generated_answer_envelope)->halt(0);halt(1))' \
        -t 'halt(9)' >"$answer_version_stdout" \
        2>"$answer_version_stderr"; then
    answer_version_status=0
else
    answer_version_status=$?
fi
if [ "$answer_version_status" -ne 0 ]; then
    fail_case "probe/answer-version-rejection" \
        "expected yes/no v2 and v4 records to reject"
fi
if [ -s "$answer_version_stdout" ] || [ -s "$answer_version_stderr" ]; then
    fail_case "probe/answer-version-rejection" "expected zero bytes"
fi
pass_case "probe/answer-version-rejection"

answer_wh_v2_stdout="$SCRATCH/probes/answer-wh-v2-rejection.stdout"
answer_wh_v2_stderr="$SCRATCH/probes/answer-wh-v2-rejection.stderr"
if "$SWIPL" -q -f none -F none -s "$ROOT/src/prolog/explanation.pl" \
        -g '(Doc=document(docid(d),source_sha256(s),ulex([])),Program=program(sha256(h)),Id=query_id(sentence(1),clause(1)),catch(explanation:validate_answer_terms([cnl_answer_record(2),Doc,Program,answer(Id,wh(who),pred(p,[var(1)]),answers([]))]),Error,true),Error==explanation_invariant(generated_answer_envelope)->halt(0);halt(1))' \
        -t 'halt(9)' >"$answer_wh_v2_stdout" \
        2>"$answer_wh_v2_stderr"; then
    answer_wh_v2_status=0
else
    answer_wh_v2_status=$?
fi
if [ "$answer_wh_v2_status" -ne 0 ]; then
    fail_case "probe/answer-wh-v2-rejection" "expected wh v2 record to reject"
fi
if [ -s "$answer_wh_v2_stdout" ] || [ -s "$answer_wh_v2_stderr" ]; then
    fail_case "probe/answer-wh-v2-rejection" "expected zero bytes"
fi
pass_case "probe/answer-wh-v2-rejection"

generated_ir_stdout="$SCRATCH/probes/generated-ir.stdout"
generated_ir_stderr="$SCRATCH/probes/generated-ir.stderr"
if "$SWIPL" -q -f none -F none -s "$ROOT/src/prolog/drs_to_ir.pl" \
        -g '(catch(drs_to_ir:validate_generated_ir([bad]),Error,true),nonvar(Error),Error=error(generated_record_invalid(envelope,_),context(drs_to_ir,ir_validation))->halt(0);halt(1))' \
        -t 'halt(9)' >"$generated_ir_stdout" 2>"$generated_ir_stderr"; then
    generated_ir_status=0
else
    generated_ir_status=$?
fi
if [ "$generated_ir_status" -ne 0 ]; then
    fail_case "probe/generated-ir" "expected wrapped internal exception"
fi
if [ -s "$generated_ir_stdout" ] || [ -s "$generated_ir_stderr" ]; then
    fail_case "probe/generated-ir" "expected zero bytes"
fi
pass_case "probe/generated-ir"

generated_program_stdout="$SCRATCH/probes/generated-program.stdout"
generated_program_stderr="$SCRATCH/probes/generated-program.stderr"
if "$SWIPL" -q -f none -F none -s "$ROOT/src/prolog/ir_to_prolog.pl" \
        -g '(catch(ir_to_prolog:validate_generated_program([bad]),Error,true),nonvar(Error),Error=error(generated_record_invalid(envelope,_),context(ir_to_prolog,program_validation))->halt(0);halt(1))' \
        -t 'halt(9)' >"$generated_program_stdout" \
        2>"$generated_program_stderr"; then
    generated_program_status=0
else
    generated_program_status=$?
fi
if [ "$generated_program_status" -ne 0 ]; then
    fail_case "probe/generated-program" "expected wrapped internal exception"
fi
if [ -s "$generated_program_stdout" ] || \
        [ -s "$generated_program_stderr" ]; then
    fail_case "probe/generated-program" "expected zero bytes"
fi
pass_case "probe/generated-program"

generated_stdout="$SCRATCH/probes/generated-record.stdout"
generated_stderr="$SCRATCH/probes/generated-record.stderr"
if "$SWIPL" -q -f none -F none -s "$ROOT/src/prolog/ir_tool.pl" \
        -g '(ir_tool:pin_flags,catch(ir_tool:self_checked_canonical_codes([bad("x")],_),Error,true),nonvar(Error),Error=error(generated_record_invalid(canonical,term(1,unserializable)),_)->halt(0);halt(1))' \
        -t 'halt(9)' >"$generated_stdout" 2>"$generated_stderr"; then
    generated_status=0
else
    generated_status=$?
fi
if [ "$generated_status" -ne 0 ]; then
    fail_case "probe/generated-record" "expected wrapped internal exception"
fi
if [ -s "$generated_stdout" ] || [ -s "$generated_stderr" ]; then
    fail_case "probe/generated-record" "expected zero bytes"
fi
pass_case "probe/generated-record"

digest_probe_stdout="$SCRATCH/probes/program-digest-shape.stdout"
digest_probe_stderr="$SCRATCH/probes/program-digest-shape.stderr"
if "$SWIPL" -q -f none -F none -s "$ROOT/src/prolog/ir_tool.pl" \
        -g '(ir_tool:pin_flags,ir_tool:canonical_record_line(3,program(sha256(123)),Generic),Generic=="program(sha256(123)).\n",ir_tool:canonical_record_line(3,program(sha256(abc)),Quoted),string_codes(Quoted,Codes),nth0(15,Codes,39)->halt(0);halt(1))' \
        -t 'halt(9)' >"$digest_probe_stdout" \
        2>"$digest_probe_stderr"; then
    digest_probe_status=0
else
    digest_probe_status=$?
fi
if [ "$digest_probe_status" -ne 0 ]; then
    fail_case "probe/program-digest-shape" \
        "expected malformed digest fallback and atom forced quoting"
fi
if [ -s "$digest_probe_stdout" ] || [ -s "$digest_probe_stderr" ]; then
    fail_case "probe/program-digest-shape" "expected zero bytes"
fi
pass_case "probe/program-digest-shape"

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
