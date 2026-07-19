#!/usr/bin/env bash
set -eu

ROOT=$PWD
if ! [ -d vendor/e-- ]; then
    printf 'FAIL repo-root: run from the cnl-ckc repository root\n'
    printf 'SUMMARY: 0 passed, 1 failed\n'
    exit 1
fi

export PYTHONPATH="$ROOT/vendor/e--/src"
export PYTHONDONTWRITEBYTECODE=1
export RG_FFF_NO_FUZZY_FALLBACK=1

GREEN="$ROOT/tests/fixtures/strict/green"
RED="$ROOT/tests/fixtures/strict/red"
BEHAVIOR_RED="$ROOT/tests/fixtures/strict/behavior-red"
SCRATCH="$ROOT/.scratch/strict-harness.$$"
rm -rf "$SCRATCH"
mkdir -p "$SCRATCH/alternate-cwd" "$SCRATCH/atomic"
trap 'rm -rf "$SCRATCH"' EXIT

PASS_COUNT=0
FAIL_COUNT=0
RUN_STATUS=0
EXPECTED_PASS_COUNT=57

pass_case() {
    PASS_COUNT=$((PASS_COUNT + 1))
    printf 'PASS %s\n' "$1"
}

fail_case() {
    FAIL_COUNT=$((FAIL_COUNT + 1))
    printf 'FAIL %s: %s\n' "$1" "$2"
}

run_strict() {
    stdout_path=$1
    stderr_path=$2
    shift 2
    if python3 -P -m e_minus_minus.strict "$@" >"$stdout_path" 2>"$stderr_path"; then
        RUN_STATUS=0
    else
        RUN_STATUS=$?
    fi
}

check_green() {
    input=$1
    name=${input##*/}
    stem=${name%.emm}
    golden="$GREEN/$stem.golden"
    stdout_path="$SCRATCH/green.stdout"
    stderr_path="$SCRATCH/green.stderr"

    run_strict "$stdout_path" "$stderr_path" "$input"
    if [ "$RUN_STATUS" -ne 0 ]; then
        fail_case "green/$name" "exit $RUN_STATUS, expected 0"
    elif [ -s "$stderr_path" ]; then
        fail_case "green/$name" "stderr is not empty"
    elif ! cmp -s "$stdout_path" "$golden"; then
        fail_case "green/$name" "stdout differs from golden"
    else
        pass_case "green/$name"
    fi
}

check_failure() {
    name=$1
    expected_status=$2
    category=$3
    shift 3
    stdout_path="$SCRATCH/red.stdout"
    stderr_path="$SCRATCH/red.stderr"

    run_strict "$stdout_path" "$stderr_path" "$@"
    if [ "$RUN_STATUS" -ne "$expected_status" ]; then
        fail_case "$name" "exit $RUN_STATUS, expected $expected_status"
    elif [ -s "$stdout_path" ]; then
        fail_case "$name" "stdout is not empty"
    elif [ "$(wc -l < "$stderr_path")" -ne 1 ]; then
        fail_case "$name" "stderr is not exactly one line"
    elif ! grep -q "^strict:$category:" "$stderr_path"; then
        fail_case "$name" "stderr class is not strict:$category"
    else
        pass_case "$name"
    fi
}

check_determinism() {
    input=$1
    name=${input##*/}
    first="$SCRATCH/determinism-first.out"
    second="$SCRATCH/determinism-second.out"
    first_err="$SCRATCH/determinism-first.err"
    second_err="$SCRATCH/determinism-second.err"

    if (
        cd "$ROOT"
        LC_ALL=C TZ=UTC PYTHONHASHSEED=0 \
            python3 -P -m e_minus_minus.strict "$input"
    ) >"$first" 2>"$first_err"; then
        first_status=0
    else
        first_status=$?
    fi
    if (
        cd "$SCRATCH/alternate-cwd"
        LC_ALL=en_US.UTF-8 TZ=Asia/Tokyo PYTHONHASHSEED=12345 \
            python3 -P -m e_minus_minus.strict "$input"
    ) >"$second" 2>"$second_err"; then
        second_status=0
    else
        second_status=$?
    fi

    if [ "$first_status" -ne 0 ] || [ "$second_status" -ne 0 ]; then
        fail_case "determinism/$name" \
            "exits $first_status/$second_status, expected 0/0"
    elif [ -s "$first_err" ] || [ -s "$second_err" ]; then
        fail_case "determinism/$name" "stderr is not empty"
    elif ! cmp -s "$first" "$second"; then
        fail_case "determinism/$name" "environment or cwd changed output bytes"
    else
        pass_case "determinism/$name"
    fi
}

check_atomic_output() {
    input="$GREEN/01-values.emm"
    stdout_result="$SCRATCH/atomic/stdout.py"
    file_result="$SCRATCH/atomic/output.py"
    stdout_err="$SCRATCH/atomic/stdout.err"
    file_stdout="$SCRATCH/atomic/file.stdout"
    file_err="$SCRATCH/atomic/file.err"

    if python3 -P -m e_minus_minus.strict "$input" \
        >"$stdout_result" 2>"$stdout_err"; then
        stdout_status=0
    else
        stdout_status=$?
    fi
    if python3 -P -m e_minus_minus.strict -o "$file_result" "$input" \
        >"$file_stdout" 2>"$file_err"; then
        file_status=0
    else
        file_status=$?
    fi
    shopt -s nullglob
    leftovers=("$file_result".tmp.*)
    shopt -u nullglob

    if [ "$stdout_status" -ne 0 ] || [ "$file_status" -ne 0 ]; then
        fail_case "output/atomic" \
            "exits $stdout_status/$file_status, expected 0/0"
    elif [ -s "$stdout_err" ] || [ -s "$file_err" ]; then
        fail_case "output/atomic" "stderr is not empty"
    elif [ -s "$file_stdout" ]; then
        fail_case "output/atomic" "-o wrote to stdout"
    elif ! cmp -s "$stdout_result" "$file_result"; then
        fail_case "output/atomic" "-o bytes differ from stdout mode"
    elif [ "${#leftovers[@]}" -ne 0 ]; then
        fail_case "output/atomic" "temporary output remains"
    else
        pass_case "output/atomic"
    fi
}

check_import_isolation() {
    input="$GREEN/01-values.emm"
    stdout_path="$SCRATCH/import.stdout"
    stderr_path="$SCRATCH/import.stderr"

    if python3 -P -X importtime -m e_minus_minus.strict "$input" \
        >"$stdout_path" 2>"$stderr_path"; then
        status=0
    else
        status=$?
    fi
    hits=$(grep -E -c \
        'e_minus_minus\.(transpiler|normalizer|resolver|cli)|anthropic' \
        "$stderr_path" || true)

    if [ "$status" -ne 0 ]; then
        fail_case "imports/quarantine" "exit $status, expected 0"
    elif ! cmp -s "$stdout_path" "$GREEN/01-values.golden"; then
        fail_case "imports/quarantine" "stdout differs from golden"
    elif [ "$hits" -ne 0 ]; then
        fail_case "imports/quarantine" "$hits quarantined imports observed"
    else
        pass_case "imports/quarantine"
    fi
}

check_runtime() {
    local name=$1
    local input=$2
    local expected_status=$3
    local expected_stdout=$4
    local expected_stderr=$5
    local python_path="$SCRATCH/runtime.py"
    local compile_stdout="$SCRATCH/runtime-compile.stdout"
    local compile_stderr="$SCRATCH/runtime-compile.stderr"
    local runtime_stdout="$SCRATCH/runtime.stdout"
    local runtime_stderr="$SCRATCH/runtime.stderr"
    local expected_stdout_path="$SCRATCH/runtime-expected.stdout"
    local expected_stderr_path="$SCRATCH/runtime-expected.stderr"
    local status

    rm -f "$python_path"
    : > "$expected_stdout_path"
    : > "$expected_stderr_path"
    if [ -n "$expected_stdout" ]; then
        printf '%s\n' "$expected_stdout" > "$expected_stdout_path"
    fi
    if [ -n "$expected_stderr" ]; then
        printf '%s\n' "$expected_stderr" > "$expected_stderr_path"
    fi

    run_strict "$compile_stdout" "$compile_stderr" -o "$python_path" "$input"
    if [ "$RUN_STATUS" -ne 0 ]; then
        fail_case "$name" "compile exit $RUN_STATUS, expected 0"
    elif [ -s "$compile_stdout" ]; then
        fail_case "$name" "compiler wrote stdout in -o mode"
    elif [ -s "$compile_stderr" ]; then
        fail_case "$name" "compiler stderr is not empty"
    else
        if python3 -P "$python_path" >"$runtime_stdout" 2>"$runtime_stderr"; then
            status=0
        else
            status=$?
        fi
        if [ "$status" -ne "$expected_status" ]; then
            fail_case "$name" "runtime exit $status, expected $expected_status"
        elif ! cmp -s "$runtime_stdout" "$expected_stdout_path"; then
            fail_case "$name" "runtime stdout differs"
        elif ! cmp -s "$runtime_stderr" "$expected_stderr_path"; then
            fail_case "$name" "runtime stderr differs"
        else
            pass_case "$name"
        fi
    fi
}

check_require_failure() {
    local input="$BEHAVIOR_RED/01-require-fail.emm"
    local python_path="$SCRATCH/require-fail.py"
    local compile_stdout="$SCRATCH/require-compile.stdout"
    local compile_stderr="$SCRATCH/require-compile.stderr"
    local normal_stdout="$SCRATCH/require-normal.stdout"
    local normal_stderr="$SCRATCH/require-normal.stderr"
    local optimized_stdout="$SCRATCH/require-optimized.stdout"
    local optimized_stderr="$SCRATCH/require-optimized.stderr"
    local normal_status
    local optimized_status

    rm -f "$python_path"
    run_strict "$compile_stdout" "$compile_stderr" -o "$python_path" "$input"
    if [ "$RUN_STATUS" -ne 0 ]; then
        fail_case "behavior/require-fail" \
            "compile exit $RUN_STATUS, expected 0"
    elif [ -s "$compile_stdout" ] || [ -s "$compile_stderr" ]; then
        fail_case "behavior/require-fail" "compiler produced output"
    else
        if python3 -P "$python_path" >"$normal_stdout" 2>"$normal_stderr"; then
            normal_status=0
        else
            normal_status=$?
        fi
        if python3 -P -O "$python_path" \
            >"$optimized_stdout" 2>"$optimized_stderr"; then
            optimized_status=0
        else
            optimized_status=$?
        fi

        if [ "$normal_status" -eq 0 ] || [ "$optimized_status" -eq 0 ]; then
            fail_case "behavior/require-fail" \
                "runtime exits $normal_status/$optimized_status, expected nonzero/nonzero"
        elif [ -s "$normal_stdout" ] || [ -s "$optimized_stdout" ]; then
            fail_case "behavior/require-fail" "runtime stdout is not empty"
        elif ! command grep -F -q 'AssertionError: requirement failed' \
            "$normal_stderr"; then
            fail_case "behavior/require-fail" \
                "normal stderr lacks requirement failure"
        elif ! command grep -F -q 'AssertionError: requirement failed' \
            "$optimized_stderr"; then
            fail_case "behavior/require-fail" \
                "-O stderr lacks requirement failure"
        else
            pass_case "behavior/require-fail"
        fi
    fi
}

for input in "$GREEN"/*.emm; do
    check_green "$input"
done

check_failure "red/slot" 1 slot "$RED/01-slot.emm"
check_failure "red/bad-syntax" 1 syntax "$RED/02-bad-syntax.emm"
check_failure "red/non-ascii-ident" 1 non-ascii "$RED/03-non-ascii-ident.emm"
check_failure "red/keyword-target" 1 python-invalid "$RED/04-keyword-target.emm"
check_failure "red/leading-zero" 1 python-invalid "$RED/05-leading-zero.emm"
check_failure "red/truncated-hex-escape" 1 python-invalid \
    "$RED/06-truncated-hex-escape.emm"
check_failure "red/invalid-escape" 1 python-invalid "$RED/07-invalid-escape.emm"
check_failure "red/duplicate-params" 1 python-invalid "$RED/08-duplicate-params.emm"
check_failure "red/top-level-return" 1 python-invalid "$RED/09-top-level-return.emm"
check_failure "red/nested-use" 1 syntax "$RED/10-nested-use.emm"
check_failure "red/dotted-assignment" 1 syntax "$RED/11-dotted-assignment.emm"

printf 'Set x to 1.\015\012' > "$SCRATCH/crlf.emm"
printf 'Set\011x to 1.\012' > "$SCRATCH/tab.emm"
printf 'Set\013x to 1.\012' > "$SCRATCH/vertical-tab.emm"
printf 'Set\014x to 1.\012' > "$SCRATCH/form-feed.emm"
printf 'Set\034x to 1.\012' > "$SCRATCH/file-separator.emm"
printf 'Set\035x to 1.\012' > "$SCRATCH/group-separator.emm"
printf 'Set\036x to 1.\012' > "$SCRATCH/record-separator.emm"
printf 'Set x to 1.\000\012' > "$SCRATCH/nul.emm"
printf 'Set x to 1.\302\205\012' > "$SCRATCH/nel.emm"
printf 'Set x to 1.\342\200\250\012' > "$SCRATCH/line-separator.emm"
printf 'Set x to 1.\342\200\251\012' > "$SCRATCH/paragraph-separator.emm"
printf '\357\273\277Set x to 1.\012' > "$SCRATCH/bom.emm"
printf 'Set x to \200.\012' > "$SCRATCH/invalid-utf8.emm"
printf 'Set x to "left\011right".\012' > "$SCRATCH/control-in-string.emm"

check_failure "bytes/crlf" 1 control-char "$SCRATCH/crlf.emm"
check_failure "bytes/tab" 1 control-char "$SCRATCH/tab.emm"
check_failure "bytes/vertical-tab" 1 control-char "$SCRATCH/vertical-tab.emm"
check_failure "bytes/form-feed" 1 control-char "$SCRATCH/form-feed.emm"
check_failure "bytes/file-separator" 1 control-char "$SCRATCH/file-separator.emm"
check_failure "bytes/group-separator" 1 control-char "$SCRATCH/group-separator.emm"
check_failure "bytes/record-separator" 1 control-char "$SCRATCH/record-separator.emm"
check_failure "bytes/nul" 1 control-char "$SCRATCH/nul.emm"
check_failure "bytes/nel" 1 control-char "$SCRATCH/nel.emm"
check_failure "bytes/line-separator" 1 control-char "$SCRATCH/line-separator.emm"
check_failure "bytes/paragraph-separator" 1 control-char \
    "$SCRATCH/paragraph-separator.emm"
check_failure "bytes/bom" 1 bom "$SCRATCH/bom.emm"
check_failure "bytes/invalid-utf8" 1 encoding "$SCRATCH/invalid-utf8.emm"
check_failure "bytes/control-in-string" 1 control-char \
    "$SCRATCH/control-in-string.emm"

check_failure "cli/no-args" 2 usage
check_failure "cli/unknown-flag" 2 usage --unknown
check_failure "cli/extra-arg" 2 usage "$GREEN/01-values.emm" extra
check_failure "cli/missing-input" 2 io "$SCRATCH/missing.emm"

check_runtime "behavior/use-os" "$GREEN/06-use-os.emm" 0 "single" ""
check_runtime "behavior/use-os-path" "$GREEN/07-use-os-path.emm" 0 "multi.txt" ""
check_runtime "behavior/pathlib-path" "$GREEN/08-pathlib-path.emm" 0 "file.txt" ""
check_runtime "behavior/method-call" "$GREEN/09-method-call.emm" 0 "method" ""
check_runtime "behavior/dotted-ref-chain" \
    "$GREEN/10-dotted-ref-chain.emm" 0 "chain" ""
check_runtime "behavior/require-pass" "$GREEN/11-require-pass.emm" 0 "required" ""
check_runtime "behavior/exit-int" "$GREEN/12-exit-int.emm" 7 "" ""
check_runtime "behavior/exit-string" "$GREEN/13-exit-string.emm" 1 "" "goodbye"
check_runtime "behavior/terminator-dotted-ref" \
    "$GREEN/14-terminator-dotted-ref.emm" 0 "terminator" ""
check_require_failure

check_determinism "$GREEN/01-values.emm"
check_determinism "$GREEN/04-functions.emm"
check_atomic_output
check_import_isolation

if [ "$PASS_COUNT" -ne "$EXPECTED_PASS_COUNT" ]; then
    fail_case "harness/pass-count" \
        "expected $EXPECTED_PASS_COUNT, got $PASS_COUNT"
fi
printf 'SUMMARY: %s passed, %s failed\n' "$PASS_COUNT" "$FAIL_COUNT"
if [ "$FAIL_COUNT" -eq 0 ]; then
    exit 0
fi
exit 1
