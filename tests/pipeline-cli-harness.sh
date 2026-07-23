#!/usr/bin/env bash
set -eu
set -o pipefail

ROOT=$PWD
if ! [ -f tools/pipeline.py ] || \
 ! [ -f tools/ace_front_end.py ] || \
 ! [ -f src/prolog/ir_tool.pl ] || \
 ! [ -d tests/fixtures/pipeline-cli/docs ] || \
 ! [ -f tests/fixtures/pipeline-cli/golden/manifest.pl ]; then
    printf 'FAIL repo-root: run from cnl-ckc repository root\n'
    printf 'SUMMARY: 0 passed, 1 failed\n'
    exit 1
fi

DOCS="$ROOT/tests/fixtures/pipeline-cli/docs"
GOLDEN="$ROOT/tests/fixtures/pipeline-cli/golden"
SCRATCH="$ROOT/.scratch/pipeline-cli-harness.$$"
APE_TREE="$SCRATCH/ape-tree"
STUB="$SCRATCH/swipl-stub"
MISSING_SWIPL="$SCRATCH/no-such-swipl"
OUT1="$SCRATCH/out1"
OUT2="$SCRATCH/out2"
PASS_COUNT=0
RUN_STATUS=0
EXPECTED_PASS_COUNT=17

pass_case() {
    PASS_COUNT=$((PASS_COUNT + 1))
    printf 'PASS %s\n' "$1"
}

fail_case() {
    printf 'FAIL %s: %s\n' "$1" "$2"
    printf 'SUMMARY: %s passed, 1 failed\n' "$PASS_COUNT"
    exit 1
}

run_pipeline() {
    local stdout_path stderr_path swipl_value stub_log_path
    stdout_path=$1
    stderr_path=$2
    swipl_value=$3
    shift 3
    stub_log_path=${STUB_LOG_PATH:-}

    if SWIPL="$swipl_value" \
        PIPELINE_CLI_STUB_LOG="$stub_log_path" \
        PIPELINE_CLI_EXPECTED_APE_TREE="$APE_TREE" \
        PIPELINE_CLI_EXPECTED_ULEX="$DOCS/doc-b.ulex" \
        PYTHONDONTWRITEBYTECODE=1 \
        python3 -P tools/pipeline.py "$@" >"$stdout_path" 2>"$stderr_path"; then
        RUN_STATUS=0
    else
        RUN_STATUS=$?
    fi
}

assert_out_absent() {
    local label out_dir
    label=$1
    out_dir=$2
    if [ -e "$out_dir" ] || [ -L "$out_dir" ]; then
        fail_case "$label/zero-write" "output path exists"
    fi
}

assert_no_staging() {
    local label out_dir parent base entry
    label=$1
    out_dir=$2
    parent=${out_dir%/*}
    base=${out_dir##*/}
    for entry in "$parent"/"$base".tmp.*; do
        if [ -e "$entry" ] || [ -L "$entry" ]; then
            fail_case "$label/staging" "staging residue exists: ${entry##*/}"
        fi
    done
}

check_exact_failure() {
    local label expected_status expected_line out_dir stdout_path stderr_path expected_path
    label=$1
    expected_status=$2
    expected_line=$3
    out_dir=$4
    stdout_path=$5
    stderr_path=$6

    if [ "$RUN_STATUS" -ne "$expected_status" ]; then
        fail_case "$label/status" "expected $expected_status, got $RUN_STATUS"
    fi
    if [ -s "$stdout_path" ]; then
        fail_case "$label/stdout" "expected zero bytes"
    fi
    expected_path="$SCRATCH/expected-${label//\//-}.stderr"
    printf '%s\n' "$expected_line" >"$expected_path"
    if ! cmp "$stderr_path" "$expected_path"; then
        fail_case "$label/stderr" "stderr differs"
    fi
    assert_out_absent "$label" "$out_dir"
    assert_no_staging "$label" "$out_dir"
    pass_case "$label"
}

write_tree_inventory() {
    local directory output_path
    directory=$1
    output_path=$2
    (
        cd "$directory"
        find . -mindepth 1 -printf '%P\t%y\n' | LC_ALL=C sort
    ) >"$output_path"
}

compare_trees() {
    local label first second first_inventory second_inventory file_list relpath
    label=$1
    first=$2
    second=$3
    first_inventory="$SCRATCH/${label//\//-}.first-inventory"
    second_inventory="$SCRATCH/${label//\//-}.second-inventory"
    file_list="$SCRATCH/${label//\//-}.files"
    write_tree_inventory "$first" "$first_inventory"
    write_tree_inventory "$second" "$second_inventory"
    if ! cmp "$first_inventory" "$second_inventory"; then
        fail_case "$label/inventory" "tree inventories differ"
    fi
    find "$second" -type f -printf '%P\n' | LC_ALL=C sort >"$file_list"
    while IFS= read -r relpath; do
        if ! cmp "$first/$relpath" "$second/$relpath"; then
            fail_case "$label/bytes" "file differs: $relpath"
        fi
    done <"$file_list"
}

write_expected_stdout() {
    local out_dir output_path
    out_dir=$1
    output_path=$2
    printf '%s\n' \
        "pipeline: wrote $out_dir/chain/doc-a.ir.pl" \
        "pipeline: wrote $out_dir/chain/doc-a.program.pl" \
        "pipeline: wrote $out_dir/chain/doc-a.result.pl" \
        "pipeline: wrote $out_dir/chain/doc-b.ir.pl" \
        "pipeline: wrote $out_dir/chain/doc-b.program.pl" \
        "pipeline: wrote $out_dir/chain/doc-b.result.pl" \
        "pipeline: wrote $out_dir/front/doc-a.drs.pl" \
        "pipeline: wrote $out_dir/front/doc-b.drs.pl" \
        "pipeline: wrote $out_dir/front/manifest.pl" \
        "pipeline: wrote $out_dir/manifest.pl" \
        "pipeline: ok 2 documents" \
        >"$output_path"
}

append_argv_record() {
    local output_path argument
    output_path=$1
    shift

    {
        printf 'argc=%s' "$#"
        for argument in "$@"; do
            printf '\t%q' "$argument"
        done
        printf '\n'
    } >>"$output_path"
}

write_expected_swipl_argv() {
    local output_path document_index stage
    output_path=$1

    : >"$output_path"
    append_argv_record "$output_path" \
        -q -f none -F none -s src/prolog/adapter.pl \
        -g main -t 'halt(9)' -- "$APE_TREE"
    append_argv_record "$output_path" \
        -q -f none -F none -s src/prolog/adapter.pl \
        -g main -t 'halt(9)' -- "$APE_TREE" "$DOCS/doc-b.ulex"
    for document_index in 1 2; do
        : "$document_index"
        for stage in lower validate compile run; do
            append_argv_record "$output_path" \
                -q -f none -F none -s src/prolog/ir_tool.pl \
                -g main -t 'halt(9)' -- "$stage"
        done
    done
}

sha256_file() {
    local output
    output=$(sha256sum "$1")
    printf '%s' "${output%% *}"
}

make_poison_docs() {
    local directory docid token
    directory=$1
    docid=$2
    token=$3
    mkdir "$directory"
    printf '%s\n' "$token" >"$directory/$docid.ace"
}

rm -rf "$SCRATCH"
mkdir -p "$SCRATCH/red" "$SCRATCH/run1" "$SCRATCH/run2" "$APE_TREE"
trap 'rm -rf "$SCRATCH"' EXIT

if ! git status --porcelain >"$SCRATCH/status.before"; then
    fail_case "repository/status-before" "git status failed"
fi

cat >"$STUB" <<'STUB'
#!/usr/bin/env bash
set -eu

invalid_argv() {
    printf '%s\n' 'stub: invalid argv' >&2
    exit 2
}

if [ "$#" -ne 13 ] && [ "$#" -ne 14 ]; then
    invalid_argv
fi
if [ "$1" != -q ] || \
    [ "$2" != -f ] || \
    [ "$3" != none ] || \
    [ "$4" != -F ] || \
    [ "$5" != none ] || \
    [ "$6" != -s ] || \
    [ "$8" != -g ] || \
    [ "$9" != main ] || \
    [ "${10}" != -t ] || \
    [ "${11}" != 'halt(9)' ] || \
    [ "${12}" != -- ]; then
    invalid_argv
fi

expected_ape=${PIPELINE_CLI_EXPECTED_APE_TREE:-}
expected_ulex=${PIPELINE_CLI_EXPECTED_ULEX:-}
if [ -z "$expected_ape" ] || [ -z "$expected_ulex" ]; then
    invalid_argv
fi

mode=
stage=
case $7 in
    src/prolog/adapter.pl)
        if [ "${13}" != "$expected_ape" ]; then
            invalid_argv
        fi
        if [ "$#" -eq 14 ] && [ "${14}" != "$expected_ulex" ]; then
            invalid_argv
        fi
        mode=adapter
        ;;
    src/prolog/ir_tool.pl)
        if [ "$#" -ne 13 ]; then
            invalid_argv
        fi
        mode=ir-tool
        stage=${13}
        case $stage in
            lower|validate|compile|run) ;;
            *) invalid_argv ;;
        esac
        ;;
    *)
        invalid_argv
        ;;
esac

if [ -n "${PIPELINE_CLI_STUB_LOG:-}" ]; then
    {
        printf 'argc=%s' "$#"
        for argument in "$@"; do
            printf '\t%q' "$argument"
        done
        printf '\n'
    } >>"$PIPELINE_CLI_STUB_LOG"
fi

input_file=$(mktemp "${TMPDIR:-/tmp}/pipeline-cli-stub.XXXXXX")
trap 'rm -f "$input_file"' EXIT
command cat >"$input_file"
sha_output=$(sha256sum "$input_file")
sha_hex=${sha_output%% *}
token=
for candidate in \
    poisonlowerfail \
    poisonlowerempty \
    poisoncompilestderr \
    poisonvalidatestdout; do
    if command grep -Fq "$candidate" "$input_file"; then
        token=$candidate
        break
    fi
done

emit_transform() {
    if [ -n "$token" ]; then
        printf "stage('%s','%s','%s').\n" "$stage" "$sha_hex" "$token"
    else
        printf "stage('%s','%s').\n" "$stage" "$sha_hex"
    fi
}

case $mode in
    adapter)
        if [ -n "$token" ]; then
            printf "drs(stub,'%s','%s').\n" "$sha_hex" "$token"
        else
            printf "drs(stub,'%s').\n" "$sha_hex"
        fi
        ;;
    ir-tool)
        case "$stage:$token" in
            lower:poisonlowerfail)
                printf '%s\n' 'ir_tool_error(lower,stub,poison).' >&2
                exit 1
                ;;
            lower:poisonlowerempty)
                exit 0
                ;;
            compile:poisoncompilestderr)
                emit_transform
                printf '%s\n' 'ir_tool_error(compile,stub,stderr).' >&2
                ;;
            validate:poisonvalidatestdout)
                printf "validate(stub,'%s','%s').\n" "$sha_hex" "$token"
                ;;
            validate:*)
                ;;
            lower:*|compile:*|run:*)
                emit_transform
                ;;
            *)
                printf '%s\n' 'ir_tool_error(cli,usage,stub).' >&2
                exit 2
                ;;
        esac
        ;;
    *)
        printf '%s\n' 'stub: unknown invocation' >&2
        exit 2
        ;;
esac
STUB
chmod 755 "$STUB"

expected_argv="$SCRATCH/expected-swipl.argv"
write_expected_swipl_argv "$expected_argv"
run1_stdout="$SCRATCH/run1/stdout"
run1_stderr="$SCRATCH/run1/stderr"
run1_argv="$SCRATCH/run1/swipl.argv"
: >"$run1_argv"
STUB_LOG_PATH=$run1_argv
run_pipeline "$run1_stdout" "$run1_stderr" "$STUB" "$APE_TREE" "$DOCS" "$OUT1"
STUB_LOG_PATH=
if [ "$RUN_STATUS" -ne 0 ]; then
    fail_case "green/publish/status" "expected 0, got $RUN_STATUS"
fi
if [ -s "$run1_stderr" ]; then
    fail_case "green/publish/stderr" "expected zero bytes"
fi
run1_expected="$SCRATCH/run1/expected-stdout"
write_expected_stdout "$OUT1" "$run1_expected"
if ! cmp "$run1_stdout" "$run1_expected"; then
    fail_case "green/publish/stdout" "success stdout differs"
fi
compare_trees "green/golden" "$OUT1" "$GOLDEN"
if ! cmp "$run1_argv" "$expected_argv"; then
    fail_case "green/publish/argv" "SWIPL invocation sequence differs"
fi
assert_no_staging "green/publish" "$OUT1"
pass_case "green/publish"

run2_stdout="$SCRATCH/run2/stdout"
run2_stderr="$SCRATCH/run2/stderr"
run2_argv="$SCRATCH/run2/swipl.argv"
: >"$run2_argv"
STUB_LOG_PATH=$run2_argv
run_pipeline "$run2_stdout" "$run2_stderr" "$STUB" "$APE_TREE" "$DOCS" "$OUT2"
STUB_LOG_PATH=
if [ "$RUN_STATUS" -ne 0 ]; then
    fail_case "green/determinism/status" "expected 0, got $RUN_STATUS"
fi
if [ -s "$run2_stderr" ]; then
    fail_case "green/determinism/stderr" "expected zero bytes"
fi
run2_expected="$SCRATCH/run2/expected-stdout"
write_expected_stdout "$OUT2" "$run2_expected"
if ! cmp "$run2_stdout" "$run2_expected"; then
    fail_case "green/determinism/stdout" "success stdout differs"
fi
compare_trees "green/determinism" "$OUT1" "$OUT2"
if ! cmp "$run2_argv" "$expected_argv" || \
    ! cmp "$run1_argv" "$run2_argv"; then
    fail_case "green/determinism/argv" \
        "SWIPL invocation sequence differs from expected or run 1"
fi
assert_no_staging "green/determinism" "$OUT2"
pass_case "green/determinism"

manifest_expected="$SCRATCH/manifest.expected"
front_manifest_hex=$(sha256_file "$OUT1/front/manifest.pl")
printf '%s\n' 'cnl_pipeline_manifest(1).' >"$manifest_expected"
for docid in doc-a doc-b; do
    drs_hex=$(sha256_file "$OUT1/front/$docid.drs.pl")
    ir_hex=$(sha256_file "$OUT1/chain/$docid.ir.pl")
    program_hex=$(sha256_file "$OUT1/chain/$docid.program.pl")
    result_hex=$(sha256_file "$OUT1/chain/$docid.result.pl")
    printf "document(docid('%s'),drs_sha256('%s'),ir_sha256('%s'),program_sha256('%s'),result_sha256('%s'),front_manifest_sha256('%s')).\n" \
        "$docid" "$drs_hex" "$ir_hex" "$program_hex" "$result_hex" "$front_manifest_hex" \
        >>"$manifest_expected"
done
if ! cmp "$OUT1/manifest.pl" "$manifest_expected"; then
    fail_case "green/manifest" "manifest digest ties differ"
fi
pass_case "green/manifest"

usage_out="$SCRATCH/red/usage-out"
usage_stdout="$SCRATCH/red/usage.stdout"
usage_stderr="$SCRATCH/red/usage.stderr"
run_pipeline "$usage_stdout" "$usage_stderr" "$STUB" \
    "$APE_TREE" "$DOCS" "$usage_out" extra
if [ "$RUN_STATUS" -ne 2 ]; then
    fail_case "red/usage/status" "expected 2, got $RUN_STATUS"
fi
if [ -s "$usage_stdout" ]; then
    fail_case "red/usage/stdout" "expected zero bytes"
fi
usage_expected="$SCRATCH/red/usage.expected"
printf '%s\n' \
    'usage: pipeline [-h] ape_tree_dir docs_dir out_dir' \
    'pipeline: error: unrecognized arguments: extra' \
    >"$usage_expected"
if ! cmp "$usage_stderr" "$usage_expected"; then
    fail_case "red/usage/stderr" "stderr differs"
fi
assert_out_absent "red/usage" "$usage_out"
assert_no_staging "red/usage" "$usage_out"
pass_case "red/usage"

ape_missing_out="$SCRATCH/red/ape-missing-out"
ape_missing_stdout="$SCRATCH/red/ape-missing.stdout"
ape_missing_stderr="$SCRATCH/red/ape-missing.stderr"
ape_missing="$SCRATCH/no-such-ape-tree"
ape_missing_docs="$SCRATCH/no-such-docs-with-ape"
run_pipeline "$ape_missing_stdout" "$ape_missing_stderr" "$MISSING_SWIPL" \
    "$ape_missing" "$ape_missing_docs" "$ape_missing_out"
check_exact_failure "red/ape-tree" 2 \
    "pipeline: ape-tree: not a directory: $ape_missing" \
    "$ape_missing_out" "$ape_missing_stdout" "$ape_missing_stderr"

docs_missing_out="$SCRATCH/red/no-such-docs-parent/out"
docs_missing_stdout="$SCRATCH/red/docs-missing.stdout"
docs_missing_stderr="$SCRATCH/red/docs-missing.stderr"
docs_missing="$SCRATCH/no-such-docs"
run_pipeline "$docs_missing_stdout" "$docs_missing_stderr" "$MISSING_SWIPL" \
    "$APE_TREE" "$docs_missing" "$docs_missing_out"
check_exact_failure "red/docs-dir" 2 \
    "pipeline: docs-dir: not a directory: $docs_missing" \
    "$docs_missing_out" "$docs_missing_stdout" "$docs_missing_stderr"

exists_out="$SCRATCH/red/exists-out"
exists_stdout="$SCRATCH/red/exists.stdout"
exists_stderr="$SCRATCH/red/exists.stderr"
mkdir "$exists_out"
printf '%s\n' 'preserve-directory' >"$exists_out/sentinel"
cp "$exists_out/sentinel" "$SCRATCH/red/exists-sentinel.expected"
write_tree_inventory "$exists_out" "$SCRATCH/red/exists.before"
run_pipeline "$exists_stdout" "$exists_stderr" "$MISSING_SWIPL" \
    "$APE_TREE" "$DOCS" "$exists_out"
if [ "$RUN_STATUS" -ne 2 ]; then
    fail_case "red/out-dir-directory/status" "expected 2, got $RUN_STATUS"
fi
if [ -s "$exists_stdout" ]; then
    fail_case "red/out-dir-directory/stdout" "expected zero bytes"
fi
printf '%s\n' "pipeline: out-dir: already exists: $exists_out" \
    >"$SCRATCH/red/exists.expected"
if ! cmp "$exists_stderr" "$SCRATCH/red/exists.expected"; then
    fail_case "red/out-dir-directory/stderr" "stderr differs"
fi
write_tree_inventory "$exists_out" "$SCRATCH/red/exists.after"
if ! cmp "$SCRATCH/red/exists.before" "$SCRATCH/red/exists.after"; then
    fail_case "red/out-dir-directory/zero-write" "pre-existing directory changed"
fi
if ! cmp "$exists_out/sentinel" "$SCRATCH/red/exists-sentinel.expected"; then
    fail_case "red/out-dir-directory/sentinel" "sentinel bytes changed"
fi
assert_no_staging "red/out-dir-directory" "$exists_out"
pass_case "red/out-dir-directory"

symlink_target="$SCRATCH/red/no-such-symlink-target"
symlink_out="$SCRATCH/red/symlink-out"
symlink_stdout="$SCRATCH/red/symlink.stdout"
symlink_stderr="$SCRATCH/red/symlink.stderr"
ln -s "$symlink_target" "$symlink_out"
symlink_before=$(readlink "$symlink_out")
run_pipeline "$symlink_stdout" "$symlink_stderr" "$MISSING_SWIPL" \
    "$APE_TREE" "$DOCS" "$symlink_out"
if [ "$RUN_STATUS" -ne 2 ]; then
    fail_case "red/out-dir-symlink/status" "expected 2, got $RUN_STATUS"
fi
if [ -s "$symlink_stdout" ]; then
    fail_case "red/out-dir-symlink/stdout" "expected zero bytes"
fi
printf '%s\n' "pipeline: out-dir: already exists: $symlink_out" \
    >"$SCRATCH/red/symlink.expected"
if ! cmp "$symlink_stderr" "$SCRATCH/red/symlink.expected"; then
    fail_case "red/out-dir-symlink/stderr" "stderr differs"
fi
if ! [ -L "$symlink_out" ]; then
    fail_case "red/out-dir-symlink/zero-write" "symlink was replaced"
fi
symlink_after=$(readlink "$symlink_out")
if [ "$symlink_after" != "$symlink_before" ]; then
    fail_case "red/out-dir-symlink/zero-write" "symlink target changed"
fi
if [ -e "$symlink_target" ] || [ -L "$symlink_target" ]; then
    fail_case "red/out-dir-symlink/zero-write" "dangling target was created"
fi
assert_no_staging "red/out-dir-symlink" "$symlink_out"
pass_case "red/out-dir-symlink"

parent_missing="$SCRATCH/red/no-such-parent"
parent_missing_out="$parent_missing/out"
parent_missing_stdout="$SCRATCH/red/parent-missing.stdout"
parent_missing_stderr="$SCRATCH/red/parent-missing.stderr"
run_pipeline "$parent_missing_stdout" "$parent_missing_stderr" "$MISSING_SWIPL" \
    "$APE_TREE" "$DOCS" "$parent_missing_out"
check_exact_failure "red/out-parent" 2 \
    "pipeline: out-dir: parent is not a directory: $parent_missing" \
    "$parent_missing_out" "$parent_missing_stdout" "$parent_missing_stderr"

stale_out="$SCRATCH/red/stale[glob]-out"
stale_entry_first="$stale_out.tmp.12345"
stale_entry_second="$stale_out.tmp.99999"
stale_stdout="$SCRATCH/red/stale.stdout"
stale_stderr="$SCRATCH/red/stale.stderr"
mkdir "$stale_entry_second" "$stale_entry_first"
printf '%s\n' 'preserve-first-stale-evidence' >"$stale_entry_first/sentinel"
printf '%s\n' 'preserve-second-stale-evidence' >"$stale_entry_second/sentinel"
cp "$stale_entry_first/sentinel" "$SCRATCH/red/stale-first.expected"
cp "$stale_entry_second/sentinel" "$SCRATCH/red/stale-second.expected"
write_tree_inventory "$stale_entry_first" "$SCRATCH/red/stale-first.before"
write_tree_inventory "$stale_entry_second" "$SCRATCH/red/stale-second.before"
run_pipeline "$stale_stdout" "$stale_stderr" "$MISSING_SWIPL" \
    "$APE_TREE" "$DOCS" "$stale_out"
if [ "$RUN_STATUS" -ne 2 ]; then
    fail_case "red/stale-staging/status" "expected 2, got $RUN_STATUS"
fi
if [ -s "$stale_stdout" ]; then
    fail_case "red/stale-staging/stdout" "expected zero bytes"
fi
printf '%s\n' 'pipeline: staging: stale staging: stale[glob]-out.tmp.12345' \
    >"$SCRATCH/red/stale.expected"
if ! cmp "$stale_stderr" "$SCRATCH/red/stale.expected"; then
    fail_case "red/stale-staging/stderr" "stderr differs"
fi
assert_out_absent "red/stale-staging" "$stale_out"
if ! [ -d "$stale_entry_first" ] || ! [ -d "$stale_entry_second" ]; then
    fail_case "red/stale-staging/evidence" "stale directories were changed"
fi
write_tree_inventory "$stale_entry_first" "$SCRATCH/red/stale-first.after"
write_tree_inventory "$stale_entry_second" "$SCRATCH/red/stale-second.after"
if ! cmp "$SCRATCH/red/stale-first.before" "$SCRATCH/red/stale-first.after" || \
    ! cmp "$SCRATCH/red/stale-second.before" "$SCRATCH/red/stale-second.after"; then
    fail_case "red/stale-staging/evidence" "stale inventories changed"
fi
if ! cmp "$stale_entry_first/sentinel" "$SCRATCH/red/stale-first.expected" || \
    ! cmp "$stale_entry_second/sentinel" "$SCRATCH/red/stale-second.expected"; then
    fail_case "red/stale-staging/evidence" "stale sentinel bytes changed"
fi
stale_count=0
for entry in "$SCRATCH/red"/"stale[glob]-out".tmp.*; do
    if [ -e "$entry" ] || [ -L "$entry" ]; then
        stale_count=$((stale_count + 1))
        if [ "$entry" != "$stale_entry_first" ] && \
            [ "$entry" != "$stale_entry_second" ]; then
            fail_case "red/stale-staging/residue" "unexpected staging entry: ${entry##*/}"
        fi
    fi
done
if [ "$stale_count" -ne 2 ]; then
    fail_case "red/stale-staging/evidence" "expected two stale entries, got $stale_count"
fi
pass_case "red/stale-staging"

missing_swipl_out="$SCRATCH/red/missing-swipl-out"
missing_swipl_stdout="$SCRATCH/red/missing-swipl.stdout"
missing_swipl_stderr="$SCRATCH/red/missing-swipl.stderr"
run_pipeline "$missing_swipl_stdout" "$missing_swipl_stderr" "$MISSING_SWIPL" \
    "$APE_TREE" "$DOCS" "$missing_swipl_out"
check_exact_failure "red/swipl-exec" 2 \
    "pipeline: swipl-exec: not executable: $MISSING_SWIPL" \
    "$missing_swipl_out" "$missing_swipl_stdout" "$missing_swipl_stderr"

front_docs="$SCRATCH/red/front-docs"
front_out="$SCRATCH/red/front-out"
front_stdout="$SCRATCH/red/front.stdout"
front_stderr="$SCRATCH/red/front.stderr"
mkdir "$front_docs"
printf '%s\n' 'Invalid document identifier.' >"$front_docs/BAD.ace"
run_pipeline "$front_stdout" "$front_stderr" "$STUB" \
    "$APE_TREE" "$front_docs" "$front_out"
check_exact_failure "red/front-relay" 2 \
    'ace-front-end: docid: invalid document id: BAD' \
    "$front_out" "$front_stdout" "$front_stderr"

lower_fail_docs="$SCRATCH/red/lower-fail-docs"
lower_fail_out="$SCRATCH/red/lower-fail-out"
lower_fail_stdout="$SCRATCH/red/lower-fail.stdout"
lower_fail_stderr="$SCRATCH/red/lower-fail.stderr"
make_poison_docs "$lower_fail_docs" poisonlowerfail poisonlowerfail
run_pipeline "$lower_fail_stdout" "$lower_fail_stderr" "$STUB" \
    "$APE_TREE" "$lower_fail_docs" "$lower_fail_out"
check_exact_failure "red/lower-failure-relay" 1 \
    'ir_tool_error(lower,stub,poison).' \
    "$lower_fail_out" "$lower_fail_stdout" "$lower_fail_stderr"

lower_empty_docs="$SCRATCH/red/lower-empty-docs"
lower_empty_out="$SCRATCH/red/lower-empty-out"
lower_empty_stdout="$SCRATCH/red/lower-empty.stdout"
lower_empty_stderr="$SCRATCH/red/lower-empty.stderr"
make_poison_docs "$lower_empty_docs" poisonlowerempty poisonlowerempty
run_pipeline "$lower_empty_stdout" "$lower_empty_stderr" "$STUB" \
    "$APE_TREE" "$lower_empty_docs" "$lower_empty_out"
check_exact_failure "red/lower-empty" 2 \
    'pipeline: stage-stdout: empty stdout for stage: lower document: poisonlowerempty' \
    "$lower_empty_out" "$lower_empty_stdout" "$lower_empty_stderr"

compile_stderr_docs="$SCRATCH/red/compile-stderr-docs"
compile_stderr_out="$SCRATCH/red/compile-stderr-out"
compile_stderr_stdout="$SCRATCH/red/compile-stderr.stdout"
compile_stderr_stderr="$SCRATCH/red/compile-stderr.stderr"
make_poison_docs "$compile_stderr_docs" poisoncompilestderr poisoncompilestderr
run_pipeline "$compile_stderr_stdout" "$compile_stderr_stderr" "$STUB" \
    "$APE_TREE" "$compile_stderr_docs" "$compile_stderr_out"
check_exact_failure "red/compile-stderr" 2 \
    'pipeline: stage-stderr: non-empty stderr for stage: compile document: poisoncompilestderr' \
    "$compile_stderr_out" "$compile_stderr_stdout" "$compile_stderr_stderr"

validate_stdout_docs="$SCRATCH/red/validate-stdout-docs"
validate_stdout_out="$SCRATCH/red/validate-stdout-out"
validate_stdout_stdout="$SCRATCH/red/validate-stdout.stdout"
validate_stdout_stderr="$SCRATCH/red/validate-stdout.stderr"
make_poison_docs "$validate_stdout_docs" poisonvalidatestdout poisonvalidatestdout
run_pipeline "$validate_stdout_stdout" "$validate_stdout_stderr" "$STUB" \
    "$APE_TREE" "$validate_stdout_docs" "$validate_stdout_out"
check_exact_failure "red/validate-stdout" 2 \
    'pipeline: stage-stdout: non-empty stdout for stage: validate document: poisonvalidatestdout' \
    "$validate_stdout_out" "$validate_stdout_stdout" "$validate_stdout_stderr"

if ! git status --porcelain >"$SCRATCH/status.after"; then
    fail_case "repository/status-after" "git status failed"
fi
if ! cmp "$SCRATCH/status.before" "$SCRATCH/status.after"; then
    printf '%s\n' '--- status before ---'
    command cat "$SCRATCH/status.before"
    printf '%s\n' '--- status after ---'
    command cat "$SCRATCH/status.after"
    fail_case "repository/cleanliness" "harness changed the working tree"
fi
pass_case "repository/cleanliness"

if [ "$PASS_COUNT" -ne "$EXPECTED_PASS_COUNT" ]; then
    fail_case "harness/pass-count" \
        "expected $EXPECTED_PASS_COUNT, got $PASS_COUNT"
fi
printf 'SUMMARY: %s passed, 0 failed\n' "$PASS_COUNT"
