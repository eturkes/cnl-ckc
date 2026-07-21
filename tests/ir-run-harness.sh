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
IR_RED="$ROOT/tests/fixtures/ir/red"
SCRATCH="$ROOT/.scratch/ir-run-harness.$$"
PASS_COUNT=0
RUN_STATUS=0
EXPECTED_PASS_COUNT=81

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
if [ "$#" -ne 10 ]; then
    fail_case "fixtures/count" "expected 10 hand-green programs, got $#"
fi
set -- "$GREEN"/*.result.pl
if [ "$#" -ne 10 ]; then
    fail_case "fixtures/count" "expected 10 hand-green results, got $#"
fi
set -- "$RED"/*.program.pl
if [ "$#" -ne 20 ]; then
    fail_case "fixtures/count" "expected 20 red programs, got $#"
fi
set -- "$LOWER_GREEN"/*.ir.pl
if [ "$#" -ne 4 ]; then
    fail_case "fixtures/count" "expected 4 lower IR goldens, got $#"
fi
set -- "$LOWER_GREEN"/*.program.pl
if [ "$#" -ne 4 ]; then
    fail_case "fixtures/count" "expected 4 lower program goldens, got $#"
fi
set -- "$LOWER_GREEN"/*.result.pl
if [ "$#" -ne 4 ]; then
    fail_case "fixtures/count" "expected 4 lower result goldens, got $#"
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

# M4.2 chain pin: the lower fixture directory owns CLI-generated IR, program,
# and result goldens. Compiling and running all four proves both admitted NAF
# profiles and both admitted who-question records reach the shipped v2 kernel.
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
    check_result_digest "digest/lower/$stem" \
        "$expected_program" "$expected_result"
    pass_case "chain/lower/$stem"
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
run_committed_red cycle-signed-transitive cycle
run_committed_red document-float shape
run_committed_red envelope-missing-document envelope
run_committed_red envelope-trailing-after-goal envelope
run_committed_red envelope-v1-record envelope
run_committed_red envelope-wrong-header envelope
run_committed_red identity-fact-with-body identity
run_committed_red ordering-duplicate-id ordering
run_committed_red query-count-two query_count
run_committed_red query-count-zero query_count
run_committed_red safety-head-uncovered safety
run_committed_red safety-naf-order-interleave safety
run_committed_red safety-naf-var-uncovered safety
run_committed_red scope-non-dense scope
run_committed_red section-order-fact-after-rule section_order
run_committed_red shape-naf-goal-position shape
run_committed_red shape-native-variable shape
run_committed_red shape-unknown-constructor shape
run_committed_red resource-cap resource

pin_stdout="$SCRATCH/stage-pin/compile-v1-envelope.stdout"
pin_stderr="$SCRATCH/stage-pin/compile-v1-envelope.stderr"
run_tool "$IR_RED/envelope-v1-record.pl" "$pin_stdout" "$pin_stderr" compile
check_rejection "stage-pin/compile-v1-envelope" 1 compile envelope \
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
        -g '(assertz(cnl_program_db:program_clause(1,rule_id(sentence(1),clause(1)),pred(q,[named(a)]),[pred(p,[named(a)])])),assertz(cnl_program_db:program_clause(2,fact_id(sentence(2),clause(1)),pred(p,[named(a)]),[])),Child=proof(pred(p,[named(a)]),fact_id(sentence(2),clause(1)),[]),Proof=proof(pred(q,[named(a)]),rule_id(sentence(1),clause(1)),weird(Child,[],x)),catch(explanation:replay_certificate(pred(q,[named(a)]),Proof,[]),Error,true),\+explanation:replay_children(weird(Child,[],x),[]),retractall(cnl_program_db:program_clause(_,_,_,_)),Error==explanation_invariant(replay_failed)->halt(0);halt(1))' \
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
        -g '(assertz(cnl_program_db:program_clause(1,rule_id(sentence(1),clause(1)),pred(recover,[var(1)]),[pred(patient,[var(1)]),naf(pred(smoke,[var(1)]))])),assertz(cnl_program_db:program_clause(2,fact_id(sentence(2),clause(1)),pred(patient,[named(a)]),[])),Proof=proof(pred(recover,[named(a)]),rule_id(sentence(1),clause(1)),[proof(pred(patient,[named(a)]),fact_id(sentence(2),clause(1)),[]),naf(pred(smoke,[named(a)]))]),explanation:replay_certificate(pred(recover,[named(a)]),Proof,[]),Store=[entry(pred(smoke,[named(a)]),by(fact_id(sentence(3),clause(1)),[]))],catch(explanation:replay_certificate(pred(recover,[named(a)]),Proof,Store),Error,true),retractall(cnl_program_db:program_clause(_,_,_,_)),Error==explanation_invariant(replay_failed)->halt(0);halt(1))' \
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
