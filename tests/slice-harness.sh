#!/usr/bin/env bash
set -eu

ROOT=$PWD
repo_root=$(git rev-parse --show-toplevel 2>/dev/null || :)
if [ "$repo_root" != "$ROOT" ] || \
        ! [ -f vendor/ape/Makefile ] || \
        ! [ -f tools/ace_front_end.py ] || \
        ! [ -f src/prolog/adapter.pl ] || \
        ! [ -f src/prolog/ir_tool.pl ] || \
        ! [ -f src/prolog/drs_to_ir.pl ] || \
        ! [ -f src/prolog/ir_to_prolog.pl ] || \
        ! [ -f src/prolog/inference_kernel.pl ] || \
        ! [ -f src/prolog/explanation.pl ] || \
        ! [ -d tests/fixtures/slice/docs ] || \
        ! [ -d tests/fixtures/slice/golden ] || \
        ! [ -d tests/fixtures/slice/ir ] || \
        ! [ -d tests/fixtures/slice/program ] || \
        ! [ -d tests/fixtures/slice/result ]; then
    printf 'FAIL repo-root: run from cnl-ckc repository root\n'
    printf 'SUMMARY: 0 passed, 1 failed\n'
    exit 1
fi

SWIPL=${SWIPL:-swipl}
DOCS="$ROOT/tests/fixtures/slice/docs"
GOLDEN="$ROOT/tests/fixtures/slice/golden"
IR="$ROOT/tests/fixtures/slice/ir"
PROGRAM="$ROOT/tests/fixtures/slice/program"
RESULT="$ROOT/tests/fixtures/slice/result"
SCRATCH="$ROOT/.scratch/slice-harness.$$"
TREE="$SCRATCH/tree"
OUT1="$SCRATCH/out1"
OUT2="$SCRATCH/out2"
PASS_COUNT=0
RUN_STATUS=0
CHAIN_STAGE=
CHAIN_STATUS=0
CHAIN_REASON=
EXPECTED_PASS_COUNT=29

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
    local out_dir stdout_path stderr_path
    out_dir=$1
    stdout_path=$2
    stderr_path=$3

    if SWIPL="$SWIPL" PYTHONDONTWRITEBYTECODE=1 \
        python3 -P tools/ace_front_end.py \
        "$TREE" tests/fixtures/slice/docs "$out_dir" \
        >"$stdout_path" 2>"$stderr_path"; then
        RUN_STATUS=0
    else
        RUN_STATUS=$?
    fi
}

run_tool() {
    local input stdout_path stderr_path command_name
    input=$1
    stdout_path=$2
    stderr_path=$3
    command_name=$4

    if "$SWIPL" -q -f none -F none -s "$ROOT/src/prolog/ir_tool.pl" \
        -g main -t 'halt(9)' -- "$command_name" \
        <"$input" >"$stdout_path" 2>"$stderr_path"; then
        RUN_STATUS=0
    else
        RUN_STATUS=$?
    fi
}

write_file_set() {
    local directory output_path
    directory=$1
    output_path=$2
    find "$directory" -mindepth 1 -maxdepth 1 -printf '%f\n' | \
        LC_ALL=C sort >"$output_path"
}

write_chain_expected() {
    local output_path stem
    output_path=$1
    shift
    for stem in "$@"; do
        printf '%s\n' \
            "$stem.ir.pl" \
            "$stem.lower.stderr" \
            "$stem.validate.stdout" \
            "$stem.validate.stderr" \
            "$stem.program.pl" \
            "$stem.compile.stderr" \
            "$stem.result.pl" \
            "$stem.run.stderr"
    done | LC_ALL=C sort >"$output_path"
}

tree_content_hash() {
    find "$TREE" -type f -print0 | \
        LC_ALL=C sort -z | \
        xargs -0 sha256sum | \
        sha256sum
}

write_front_end_expected() {
    local out_dir output_path
    out_dir=$1
    output_path=$2
    printf '%s\n' \
        "ace-front-end: wrote $out_dir/manifest.pl" \
        "ace-front-end: wrote $out_dir/slice-naf.drs.pl" \
        "ace-front-end: wrote $out_dir/slice-unknown.drs.pl" \
        "ace-front-end: wrote $out_dir/slice-wh.drs.pl" \
        "ace-front-end: wrote $out_dir/slice.drs.pl" \
        'ace-front-end: ok 4 documents' \
        >"$output_path"
}

run_chain() {
    local input stem directory ir_path validate_stdout validate_stderr
    local lower_stderr program_path compile_stderr result_path run_stderr
    input=$1
    stem=$2
    directory=$3
    ir_path="$directory/$stem.ir.pl"
    lower_stderr="$directory/$stem.lower.stderr"
    validate_stdout="$directory/$stem.validate.stdout"
    validate_stderr="$directory/$stem.validate.stderr"
    program_path="$directory/$stem.program.pl"
    compile_stderr="$directory/$stem.compile.stderr"
    result_path="$directory/$stem.result.pl"
    run_stderr="$directory/$stem.run.stderr"
    CHAIN_STAGE=lower
    CHAIN_STATUS=0
    CHAIN_REASON=

    run_tool "$input" "$ir_path" "$lower_stderr" lower
    CHAIN_STATUS=$RUN_STATUS
    if [ "$RUN_STATUS" -ne 0 ]; then
        CHAIN_REASON=status
        return 1
    fi
    if [ -s "$lower_stderr" ]; then
        CHAIN_REASON=stderr
        return 1
    fi
    if ! cmp "$ir_path" "$IR/$stem.ir.pl"; then
        CHAIN_REASON=golden-bytes
        return 1
    fi

    CHAIN_STAGE=validate
    run_tool "$ir_path" "$validate_stdout" "$validate_stderr" validate
    CHAIN_STATUS=$RUN_STATUS
    if [ "$RUN_STATUS" -ne 0 ]; then
        CHAIN_REASON=status
        return 1
    fi
    if [ -s "$validate_stdout" ] || [ -s "$validate_stderr" ]; then
        CHAIN_REASON=streams
        return 1
    fi

    CHAIN_STAGE=compile
    run_tool "$ir_path" "$program_path" "$compile_stderr" compile
    CHAIN_STATUS=$RUN_STATUS
    if [ "$RUN_STATUS" -ne 0 ]; then
        CHAIN_REASON=status
        return 1
    fi
    if [ -s "$compile_stderr" ]; then
        CHAIN_REASON=stderr
        return 1
    fi
    if ! cmp "$program_path" "$PROGRAM/$stem.program.pl"; then
        CHAIN_REASON=golden-bytes
        return 1
    fi

    CHAIN_STAGE=run
    run_tool "$program_path" "$result_path" "$run_stderr" run
    CHAIN_STATUS=$RUN_STATUS
    if [ "$RUN_STATUS" -ne 0 ]; then
        CHAIN_REASON=status
        return 1
    fi
    if [ -s "$run_stderr" ]; then
        CHAIN_REASON=stderr
        return 1
    fi
    if ! cmp "$result_path" "$RESULT/$stem.result.pl"; then
        CHAIN_REASON=golden-bytes
        return 1
    fi

    CHAIN_STAGE=complete
    CHAIN_STATUS=0
    CHAIN_REASON=
    return 0
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
if [ "$#" -ne 4 ]; then
    fail_case "fixtures/count" "expected 4 ACE fixtures, got $#"
fi
set -- "$DOCS"/*.ulex
if [ "$#" -ne 4 ]; then
    fail_case "fixtures/count" "expected 4 Ulex fixtures, got $#"
fi
set -- "$GOLDEN"/*.drs.pl
if [ "$#" -ne 4 ]; then
    fail_case "fixtures/count" "expected 4 DRS goldens, got $#"
fi
set -- "$GOLDEN"/*.pl
if [ "$#" -ne 5 ]; then
    fail_case "fixtures/count" "expected manifest plus 4 DRS goldens, got $# files"
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
pass_case "fixtures/count"

rm -rf "$SCRATCH"
mkdir -p "$SCRATCH/expected" "$TREE" "$SCRATCH/run1-chain" \
    "$SCRATCH/run2-chain" "$SCRATCH/logs" "$SCRATCH/red-chain"
trap 'rm -rf "$SCRATCH"' EXIT

printf '%s\n' \
    slice-naf.ace \
    slice-naf.ulex \
    slice-unknown.ace \
    slice-unknown.ulex \
    slice-wh.ace \
    slice-wh.ulex \
    slice.ace \
    slice.ulex | LC_ALL=C sort >"$SCRATCH/expected/docs"
printf '%s\n' \
    manifest.pl \
    slice-naf.drs.pl \
    slice-unknown.drs.pl \
    slice-wh.drs.pl \
    slice.drs.pl | LC_ALL=C sort >"$SCRATCH/expected/golden"
printf '%s\n' \
    slice-naf.ir.pl \
    slice-unknown.ir.pl \
    slice-wh.ir.pl \
    slice.ir.pl | LC_ALL=C sort >"$SCRATCH/expected/ir"
printf '%s\n' \
    slice-naf.program.pl \
    slice-unknown.program.pl \
    slice-wh.program.pl \
    slice.program.pl | LC_ALL=C sort >"$SCRATCH/expected/program"
printf '%s\n' \
    slice-naf.result.pl \
    slice-unknown.result.pl \
    slice-wh.result.pl \
    slice.result.pl | LC_ALL=C sort >"$SCRATCH/expected/result"
write_chain_expected "$SCRATCH/expected/chain" \
    slice-unknown slice slice-naf slice-wh
printf '%s\n' \
    slice-invalid.ir.pl \
    slice-invalid.lower.stderr | LC_ALL=C sort >"$SCRATCH/expected/red-chain"

write_file_set "$DOCS" "$SCRATCH/expected/docs.actual"
write_file_set "$GOLDEN" "$SCRATCH/expected/golden.actual"
write_file_set "$IR" "$SCRATCH/expected/ir.actual"
write_file_set "$PROGRAM" "$SCRATCH/expected/program.actual"
write_file_set "$RESULT" "$SCRATCH/expected/result.actual"
if ! cmp "$SCRATCH/expected/docs" "$SCRATCH/expected/docs.actual" || \
        ! cmp "$SCRATCH/expected/golden" "$SCRATCH/expected/golden.actual" || \
        ! cmp "$SCRATCH/expected/ir" "$SCRATCH/expected/ir.actual" || \
        ! cmp "$SCRATCH/expected/program" "$SCRATCH/expected/program.actual" || \
        ! cmp "$SCRATCH/expected/result" "$SCRATCH/expected/result.actual"; then
    fail_case "fixtures/set" "slice fixture names differ from the pinned set"
fi
for fixture in \
    "$DOCS/slice-naf.ace" \
    "$DOCS/slice-naf.ulex" \
    "$DOCS/slice-unknown.ace" \
    "$DOCS/slice-unknown.ulex" \
    "$DOCS/slice-wh.ace" \
    "$DOCS/slice-wh.ulex" \
    "$DOCS/slice.ace" \
    "$DOCS/slice.ulex" \
    "$GOLDEN/manifest.pl" \
    "$GOLDEN/slice-naf.drs.pl" \
    "$GOLDEN/slice-unknown.drs.pl" \
    "$GOLDEN/slice-wh.drs.pl" \
    "$GOLDEN/slice.drs.pl" \
    "$IR/slice-naf.ir.pl" \
    "$IR/slice-unknown.ir.pl" \
    "$IR/slice-wh.ir.pl" \
    "$IR/slice.ir.pl" \
    "$PROGRAM/slice-naf.program.pl" \
    "$PROGRAM/slice-unknown.program.pl" \
    "$PROGRAM/slice-wh.program.pl" \
    "$PROGRAM/slice.program.pl" \
    "$RESULT/slice-naf.result.pl" \
    "$RESULT/slice-unknown.result.pl" \
    "$RESULT/slice-wh.result.pl" \
    "$RESULT/slice.result.pl"; do
    if ! [ -f "$fixture" ] || [ -L "$fixture" ]; then
        fail_case "fixtures/set" "expected regular non-symlink file: $fixture"
    fi
done
pass_case "fixtures/set"

if ! vendor_copy_status=$(git status --porcelain --ignored -- vendor/); then
    fail_case "vendor/precopy-clean" "git status failed"
fi
if [ -n "$vendor_copy_status" ]; then
    printf '%s\n' "$vendor_copy_status"
    fail_case "vendor/precopy-clean" "vendor tree has tracked or ignored artifacts"
fi
pass_case "vendor/precopy-clean"

cp -a "$ROOT/vendor/ape/." "$TREE/"
pass_case "vendor/copy"

if ! make -C "$TREE" plp "swipl=$SWIPL -f none"; then
    fail_case "vendor/plp" "make plp failed"
fi
if ! [ -f "$TREE/prolog/parser/grammar.plp" ]; then
    fail_case "vendor/plp" "grammar.plp is missing"
fi
pass_case "vendor/plp"

if ! tree_hash_before=$(tree_content_hash); then
    fail_case "tree/immutable" "could not hash staged APE tree"
fi

run1_stdout="$SCRATCH/logs/run1.stdout"
run1_stderr="$SCRATCH/logs/run1.stderr"
run_front_end "$OUT1" "$run1_stdout" "$run1_stderr"
if [ "$RUN_STATUS" -ne 0 ]; then
    fail_case "front-end/run1/status" "expected 0, got $RUN_STATUS"
fi
if [ -s "$run1_stderr" ]; then
    fail_case "front-end/run1/stderr" "expected zero bytes"
fi
pass_case "front-end/run1"

write_front_end_expected "$OUT1" "$SCRATCH/expected/run1.stdout"
if ! cmp "$run1_stdout" "$SCRATCH/expected/run1.stdout"; then
    fail_case "front-end/run1-stdout" "stdout differs from pinned bytes"
fi
pass_case "front-end/run1-stdout"

write_file_set "$OUT1" "$SCRATCH/logs/out1.files"
if ! cmp "$SCRATCH/logs/out1.files" "$SCRATCH/expected/golden"; then
    fail_case "front-end/run1-files" "output file set differs from goldens"
fi
pass_case "front-end/run1-files"

for name in manifest.pl slice-naf.drs.pl slice-unknown.drs.pl \
        slice-wh.drs.pl slice.drs.pl; do
    if ! cmp "$OUT1/$name" "$GOLDEN/$name"; then
        fail_case "front-end/run1-goldens" "mismatch: $name"
    fi
done
pass_case "front-end/run1-goldens"

for stem in slice-unknown slice slice-naf slice-wh; do
    if ! run_chain "$OUT1/$stem.drs.pl" "$stem" "$SCRATCH/run1-chain"; then
        fail_case "chain/run1/$stem" \
            "$CHAIN_STAGE/$CHAIN_REASON, status $CHAIN_STATUS"
    fi
    pass_case "chain/run1/$stem"
done
write_file_set "$SCRATCH/run1-chain" "$SCRATCH/logs/run1-chain.files"
if ! cmp "$SCRATCH/logs/run1-chain.files" "$SCRATCH/expected/chain"; then
    fail_case "chain/run1-files" "chain file set differs from the pinned set"
fi
pass_case "chain/run1-files"

run2_stdout="$SCRATCH/logs/run2.stdout"
run2_stderr="$SCRATCH/logs/run2.stderr"
run_front_end "$OUT2" "$run2_stdout" "$run2_stderr"
if [ "$RUN_STATUS" -ne 0 ]; then
    fail_case "front-end/run2/status" "expected 0, got $RUN_STATUS"
fi
if [ -s "$run2_stderr" ]; then
    fail_case "front-end/run2/stderr" "expected zero bytes"
fi
pass_case "front-end/run2"

write_front_end_expected "$OUT2" "$SCRATCH/expected/run2.stdout"
if ! cmp "$run2_stdout" "$SCRATCH/expected/run2.stdout"; then
    fail_case "front-end/run2-stdout" "stdout differs from pinned bytes"
fi
pass_case "front-end/run2-stdout"

write_file_set "$OUT2" "$SCRATCH/logs/out2.files"
if ! cmp "$SCRATCH/logs/out2.files" "$SCRATCH/expected/golden"; then
    fail_case "front-end/run2-files" "output file set differs from goldens"
fi
pass_case "front-end/run2-files"

for name in manifest.pl slice-naf.drs.pl slice-unknown.drs.pl \
        slice-wh.drs.pl slice.drs.pl; do
    if ! cmp "$OUT2/$name" "$GOLDEN/$name"; then
        fail_case "front-end/run2-goldens" "mismatch: $name"
    fi
done
pass_case "front-end/run2-goldens"

for stem in slice-unknown slice slice-naf slice-wh; do
    if ! run_chain "$OUT2/$stem.drs.pl" "$stem" "$SCRATCH/run2-chain"; then
        fail_case "chain/run2/$stem" \
            "$CHAIN_STAGE/$CHAIN_REASON, status $CHAIN_STATUS"
    fi
    pass_case "chain/run2/$stem"
done
write_file_set "$SCRATCH/run2-chain" "$SCRATCH/logs/run2-chain.files"
if ! cmp "$SCRATCH/logs/run2-chain.files" "$SCRATCH/expected/chain"; then
    fail_case "chain/run2-files" "chain file set differs from the pinned set"
fi
pass_case "chain/run2-files"

if ! tree_hash_after=$(tree_content_hash); then
    fail_case "tree/immutable" "could not re-hash staged APE tree"
fi
if [ "$tree_hash_before" != "$tree_hash_after" ]; then
    fail_case "tree/immutable" "staged APE tree changed across the two passes"
fi
pass_case "tree/immutable"

for name in manifest.pl slice-naf.drs.pl slice-unknown.drs.pl \
        slice-wh.drs.pl slice.drs.pl; do
    if ! cmp "$OUT1/$name" "$OUT2/$name"; then
        fail_case "determinism/front-end" "fresh runs differ: $name"
    fi
done
pass_case "determinism/front-end"

while IFS= read -r name; do
    if ! cmp "$SCRATCH/run1-chain/$name" "$SCRATCH/run2-chain/$name"; then
        fail_case "determinism/downstream" "fresh chains differ: $name"
    fi
done <"$SCRATCH/expected/chain"
pass_case "determinism/downstream"

bad_input="$SCRATCH/slice-invalid.drs.pl"
cp "$OUT1/slice.drs.pl" "$bad_input"
printf '%s\n' 'unexpected_term.' >>"$bad_input"
if run_chain "$bad_input" slice-invalid "$SCRATCH/red-chain"; then
    fail_case "red/no-nonempty-downstream" "invalid DRS unexpectedly completed"
fi
if [ "$CHAIN_STAGE" != lower ] || [ "$CHAIN_REASON" != status ] || \
        [ "$CHAIN_STATUS" -ne 1 ]; then
    fail_case "red/no-nonempty-downstream" \
        "expected lower/status 1, got $CHAIN_STAGE/$CHAIN_REASON $CHAIN_STATUS"
fi
red_stderr="$SCRATCH/red-chain/slice-invalid.lower.stderr"
red_line_count=$(wc -l <"$red_stderr")
if [ "$red_line_count" -ne 1 ]; then
    fail_case "red/no-nonempty-downstream" \
        "expected one stderr line, got $red_line_count"
fi
if ! printf '%s\n' "$(<"$red_stderr")" | cmp - "$red_stderr"; then
    fail_case "red/no-nonempty-downstream" \
        "expected exactly one LF-terminated stderr line"
fi
if ! command grep -Eq '^ir_tool_error\(lower,envelope,.*\)\.$' "$red_stderr"; then
    fail_case "red/no-nonempty-downstream" "expected lower envelope rejection"
fi
write_file_set "$SCRATCH/red-chain" "$SCRATCH/logs/red-chain.files"
if ! cmp "$SCRATCH/logs/red-chain.files" "$SCRATCH/expected/red-chain"; then
    fail_case "red/no-nonempty-downstream" \
        "red chain file set differs from the pinned lower-failure set"
fi
for artifact in \
    "$SCRATCH/red-chain/slice-invalid.ir.pl" \
    "$SCRATCH/red-chain/slice-invalid.program.pl" \
    "$SCRATCH/red-chain/slice-invalid.result.pl"; do
    if [ -s "$artifact" ]; then
        fail_case "red/no-nonempty-downstream" "non-empty artifact: $artifact"
    fi
done
pass_case "red/no-nonempty-downstream"

if ! git diff --exit-code -- vendor/; then
    fail_case "vendor/clean" "tracked vendor bytes changed"
fi
vendor_status=$(git status --porcelain -- vendor/)
if [ -n "$vendor_status" ]; then
    printf '%s\n' "$vendor_status"
    fail_case "vendor/clean" "vendor tree changed"
fi
pass_case "vendor/clean"

if [ "$PASS_COUNT" -ne "$EXPECTED_PASS_COUNT" ]; then
    fail_case "harness/pass-count" \
        "expected $EXPECTED_PASS_COUNT, got $PASS_COUNT"
fi
printf 'SUMMARY: %s passed, 0 failed\n' "$PASS_COUNT"
