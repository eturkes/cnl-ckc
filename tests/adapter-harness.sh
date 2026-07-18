#!/usr/bin/env bash
set -eu

ROOT=$PWD
if ! [ -d vendor/ape ] || ! [ -f src/prolog/adapter.pl ]; then
    printf 'FAIL repo-root: run from cnl-ckc repository root\n'
    printf 'SUMMARY: 0 passed, 1 failed\n'
    exit 1
fi

SWIPL=${SWIPL:-swipl}
GREEN="$ROOT/tests/fixtures/adapter/green"
RED="$ROOT/tests/fixtures/adapter/red"
SCRATCH="$ROOT/.scratch/adapter-harness"
TREE="$SCRATCH/tree"
FAKE_SHAPE="$SCRATCH/fake-shape"
FAKE_ATTVAR="$SCRATCH/fake-attvar"
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

run_adapter() {
    local input stdout_path stderr_path
    input=$1
    stdout_path=$2
    stderr_path=$3
    shift 3

    if "$SWIPL" -q -f none -F none -s "$ROOT/src/prolog/adapter.pl" -g main -t 'halt(9)' -- "$@" \
        <"$input" >"$stdout_path" 2>"$stderr_path"; then
        RUN_STATUS=0
    else
        RUN_STATUS=$?
    fi
}

check_rejection() {
    local label expected_status expected_class expected_detail stdout_path stderr_path
    local line_count
    label=$1
    expected_status=$2
    expected_class=$3
    expected_detail=$4
    stdout_path=$5
    stderr_path=$6

    if [ "$RUN_STATUS" -ne "$expected_status" ]; then
        fail_case "$label/status" "expected $expected_status, got $RUN_STATUS"
    fi
    if [ -s "$stdout_path" ]; then
        fail_case "$label/stdout" "expected zero bytes"
    fi
    line_count=$(grep -c '^' "$stderr_path" || :)
    if [ "$line_count" -ne 1 ]; then
        fail_case "$label/stderr" "expected one line, got $line_count"
    fi
    if ! printf '%s\n' "$(<"$stderr_path")" | cmp - "$stderr_path"; then
        fail_case "$label/stderr" "expected exactly one LF-terminated line"
    fi
    if ! grep -Eq "^adapter_error\\(${expected_class},.*\\)\\.$" "$stderr_path"; then
        fail_case "$label/class" "expected adapter_error($expected_class, ...)"
    fi
    if [ -n "$expected_detail" ] && ! grep -Fq "$expected_detail" "$stderr_path"; then
        fail_case "$label/detail" "missing: $expected_detail"
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
mkdir -p "$TREE" "$SCRATCH/green" "$SCRATCH/red" "$SCRATCH/env"
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

for input in "$GREEN"/*.ace; do
    name=${input##*/}
    stem=${name%.ace}
    golden="$GREEN/$stem.golden"
    stdout_path="$SCRATCH/green/$stem.run1.stdout"
    stderr_path="$SCRATCH/green/$stem.run1.stderr"

    if ! [ -f "$golden" ]; then
        fail_case "green/$stem/golden" "missing $golden"
    fi
    run_adapter "$input" "$stdout_path" "$stderr_path" "$TREE"
    if [ "$RUN_STATUS" -ne 0 ]; then
        fail_case "green/$stem/status" "expected 0, got $RUN_STATUS"
    fi
    if [ -s "$stderr_path" ]; then
        fail_case "green/$stem/stderr" "expected zero bytes"
    fi
    if ! cmp "$stdout_path" "$golden"; then
        fail_case "green/$stem/stdout" "golden mismatch"
    fi
    pass_case "green/$stem"
done

anchor_expected="$SCRATCH/anchor.expected"
printf '%s\n' "drs([A],[-(predicate(A,wait,named('John')),/(1,2))])." >"$anchor_expected"
if ! cmp "$GREEN/anchor.golden" "$anchor_expected"; then
    fail_case "anchor/golden" "hard-anchor bytes changed"
fi
pass_case "anchor/golden"

for input in "$GREEN"/*.ace; do
    name=${input##*/}
    stem=${name%.ace}
    first_stdout="$SCRATCH/green/$stem.run1.stdout"
    second_stdout="$SCRATCH/green/$stem.run2.stdout"
    second_stderr="$SCRATCH/green/$stem.run2.stderr"

    run_adapter "$input" "$second_stdout" "$second_stderr" "$TREE"
    if [ "$RUN_STATUS" -ne 0 ]; then
        fail_case "determinism/$stem/status" "second run exited $RUN_STATUS"
    fi
    if [ -s "$second_stderr" ]; then
        fail_case "determinism/$stem/stderr" "second run wrote stderr"
    fi
    if ! cmp "$first_stdout" "$second_stdout"; then
        fail_case "determinism/$stem/bytes" "fresh runs differ"
    fi
    pass_case "determinism/$stem"
done

stdout_path="$SCRATCH/red/noperiod.stdout"
stderr_path="$SCRATCH/red/noperiod.stderr"
run_adapter "$RED/noperiod.ace" "$stdout_path" "$stderr_path" "$TREE"
check_rejection "red/noperiod" 1 ape_messages 'message(error,sentence,' "$stdout_path" "$stderr_path"

stdout_path="$SCRATCH/red/oov.stdout"
stderr_path="$SCRATCH/red/oov.stderr"
run_adapter "$RED/oov.ace" "$stdout_path" "$stderr_path" "$TREE"
check_rejection "red/oov" 1 ape_messages 'message(error,word,' "$stdout_path" "$stderr_path"

stdout_path="$SCRATCH/red/warnonly.stdout"
stderr_path="$SCRATCH/red/warnonly.stderr"
run_adapter "$RED/warnonly.ace" "$stdout_path" "$stderr_path" "$TREE"
check_rejection "red/warnonly" 1 ape_messages 'message(warning,' "$stdout_path" "$stderr_path"

stdout_path="$SCRATCH/red/empty.stdout"
stderr_path="$SCRATCH/red/empty.stderr"
run_adapter "$RED/empty.ace" "$stdout_path" "$stderr_path" "$TREE"
check_rejection "red/empty" 1 empty_drs 'drs([],[])' "$stdout_path" "$stderr_path"

stdout_path="$SCRATCH/env/usage.stdout"
stderr_path="$SCRATCH/env/usage.stderr"
run_adapter "$RED/empty.ace" "$stdout_path" "$stderr_path"
check_rejection "env/usage" 2 usage 'argv([])' "$stdout_path" "$stderr_path"

stdout_path="$SCRATCH/env/ape-load.stdout"
stderr_path="$SCRATCH/env/ape-load.stderr"
run_adapter "$RED/empty.ace" "$stdout_path" "$stderr_path" "$SCRATCH/no-such-tree"
check_rejection "env/ape-load" 2 ape_load 'existence_error(source_sink,' "$stdout_path" "$stderr_path"

# Fake parser modules exercise adapter boundaries without touching vendor.
mkdir -p "$FAKE_SHAPE/prolog/parser" "$FAKE_ATTVAR/prolog/parser"
printf '%s\n\n%s\n' \
    ':- module(ace_to_drs, [acetext_to_drs/8]).' \
    'acetext_to_drs(_, off, off, [], [], not_a_drs, [], 0).' \
    >"$FAKE_SHAPE/prolog/parser/ace_to_drs.pl"
printf '%s\n\n%s\n%s\n' \
    ':- module(ace_to_drs, [acetext_to_drs/8]).' \
    'acetext_to_drs(_, off, off, [], [], drs([x], []), [poison(Var)], 0) :-' \
    '    put_attr(Var, ace_to_drs, poison).' \
    >"$FAKE_ATTVAR/prolog/parser/ace_to_drs.pl"

stdout_path="$SCRATCH/env/fake-shape.stdout"
stderr_path="$SCRATCH/env/fake-shape.stderr"
run_adapter "$RED/empty.ace" "$stdout_path" "$stderr_path" "$FAKE_SHAPE"
check_rejection "fake/shape" 2 uncaught 'invalid_drs(not_a_drs)' "$stdout_path" "$stderr_path"

stdout_path="$SCRATCH/env/fake-attvar.stdout"
stderr_path="$SCRATCH/env/fake-attvar.stderr"
run_adapter "$RED/empty.ace" "$stdout_path" "$stderr_path" "$FAKE_ATTVAR"
check_rejection "fake/attvar" 1 ape_messages 'adapter_error(ape_messages,unserializable).' "$stdout_path" "$stderr_path"

if ! vendor_status=$(git status --porcelain -- vendor/); then
    fail_case "vendor/clean" "git status failed"
fi
if [ -n "$vendor_status" ]; then
    printf '%s\n' "$vendor_status"
    fail_case "vendor/clean" "vendor tree changed"
fi
pass_case "vendor/clean"

printf 'SUMMARY: %s passed, 0 failed\n' "$PASS_COUNT"
