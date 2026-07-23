#!/usr/bin/env bash
set -eu

export LC_ALL=C

ROOT=$PWD
repo_root=$(git rev-parse --show-toplevel 2>/dev/null || :)
if [ "$repo_root" != "$ROOT" ] || \
   ! [ -f "$ROOT/vendor/ape/Makefile" ] || \
   ! [ -f "$ROOT/tools/pipeline.py" ] || \
   ! [ -f "$ROOT/tools/ace_front_end.py" ] || \
   ! [ -f "$ROOT/src/prolog/registry_tool.pl" ] || \
   ! [ -f "$ROOT/src/prolog/adapter.pl" ] || \
   ! [ -f "$ROOT/src/prolog/ir_tool.pl" ] || \
   ! [ -f "$ROOT/guidelines/registry.pl" ] || \
   ! [ -f "$ROOT/guidelines/cdc-2022-opioid/terminology.pl" ] || \
   ! [ -f "$ROOT/guidelines/cdc-2022-opioid/mapping.pl" ] || \
   ! [ -d "$ROOT/guidelines/cdc-2022-opioid/ace" ] || \
   ! [ -d "$ROOT/guidelines/cdc-2022-opioid/golden/front" ] || \
   ! [ -d "$ROOT/guidelines/cdc-2022-opioid/golden/chain" ]; then
    printf 'FAIL repo-root: run from cnl-ckc repository root\n'
    printf 'SUMMARY: 0 passed, 1 failed\n'
    exit 1
fi

SWIPL=${SWIPL:-swipl}
REGISTRY="$ROOT/guidelines/registry.pl"
TERMINOLOGY="$ROOT/guidelines/cdc-2022-opioid/terminology.pl"
MAPPING="$ROOT/guidelines/cdc-2022-opioid/mapping.pl"
DOCS="$ROOT/guidelines/cdc-2022-opioid/ace"
GOLDEN="$ROOT/guidelines/cdc-2022-opioid/golden"
SCRATCH="$ROOT/.scratch/guideline-harness.$$"
TREE="$SCRATCH/tree"
OUT1="$SCRATCH/out1"
OUT2="$SCRATCH/out2"
OUT3="$SCRATCH/out3"
OUT4="$SCRATCH/out4"
PASS_COUNT=0
RUN_STATUS=0
EXPECTED_PASS_COUNT=45

pass_case() {
    PASS_COUNT=$((PASS_COUNT + 1))
    printf 'PASS %s\n' "$1"
}

fail_case() {
    printf 'FAIL %s: %s\n' "$1" "$2"
    printf 'SUMMARY: %s passed, 1 failed\n' "$PASS_COUNT"
    exit 1
}

run_registry_tool() {
    local input stdout_path stderr_path command_name
    input=$1
    stdout_path=$2
    stderr_path=$3
    command_name=$4

    if "$SWIPL" -q -f none -F none \
        -s "$ROOT/src/prolog/registry_tool.pl" -g main -t 'halt(9)' -- \
        "$command_name" <"$input" >"$stdout_path" 2>"$stderr_path"; then
        RUN_STATUS=0
    else
        RUN_STATUS=$?
    fi
}

run_pipeline() {
    local swipl_value docs_dir out_dir stdout_path stderr_path
    swipl_value=$1
    docs_dir=$2
    out_dir=$3
    stdout_path=$4
    stderr_path=$5

    if bash -c '
        set +e
        SWIPL=$1 PYTHONDONTWRITEBYTECODE=1 \
            python3 -P tools/pipeline.py "$2" "$3" "$4"
        status=$?
        exit "$status"
    ' bash "$swipl_value" "$TREE" "$docs_dir" "$out_dir" \
        >"$stdout_path" 2>"$stderr_path"; then
        RUN_STATUS=0
    else
        RUN_STATUS=$?
    fi
}

sha256_file() {
    local output
    output=$(sha256sum "$1")
    printf '%s\n' "${output%% *}"
}

tree_content_hash() {
    local directory output_path
    directory=$1
    output_path=$2

    (
        cd "$directory"
        find . -type f -print0 | sort -z | xargs -0 -r sha256sum
    ) | sha256sum >"$output_path"
}

write_pipeline_stdout() {
    local out_dir output_path relpath
    out_dir=$1
    output_path=$2

    : >"$output_path"
    while IFS= read -r relpath; do
        printf 'pipeline: wrote %s/%s\n' "$out_dir" "$relpath" >>"$output_path"
    done <"$SCRATCH/expected/golden-files"
    printf 'pipeline: ok 12 documents\n' >>"$output_path"
}

write_staging_entries() {
    local out_dir output_path parent base
    out_dir=$1
    output_path=$2
    parent=${out_dir%/*}
    base=${out_dir##*/}

    find "$parent" -mindepth 1 -maxdepth 1 \
        -name "$base.tmp.*" -printf '%p\n' | sort >"$output_path"
}

rm -rf "$SCRATCH"
mkdir -p "$SCRATCH/expected" "$SCRATCH/logs" "$SCRATCH/mapping" \
    "$SCRATCH/recovery" "$SCRATCH/clex"
trap 'rm -rf "$SCRATCH"' EXIT

if ! git status --porcelain >"$SCRATCH/status.before"; then
    fail_case "repository/status-before" "git status failed"
fi

if swipl_version=$("$SWIPL" --version 2>&1); then
    case $swipl_version in
        *'SWI-Prolog version 9.2.9 '*) ;;
        *)
            fail_case "swipl/version" \
                "expected SWI-Prolog version 9.2.9, got: $swipl_version"
            ;;
    esac
else
    fail_case "swipl/version" "could not run $SWIPL"
fi
pass_case "swipl/version"

ace_count=$(find "$DOCS" -mindepth 1 -maxdepth 1 -type f -name '*.ace' -printf '.\n' | wc -l)
ulex_count=$(find "$DOCS" -mindepth 1 -maxdepth 1 -type f -name '*.ulex' -printf '.\n' | wc -l)
golden_count=$(find "$GOLDEN" -type f -printf '.\n' | wc -l)
if [ "$ace_count" -ne 12 ] || [ "$ulex_count" -ne 12 ] || [ "$golden_count" -ne 50 ]; then
    fail_case "inventory/counts" \
        "expected 12 ACE, 12 Ulex, 50 golden files; got $ace_count, $ulex_count, $golden_count"
fi
pass_case "inventory/counts"

printf '%s\n' \
    cdc2022-opioid-rec01 \
    cdc2022-opioid-rec02 \
    cdc2022-opioid-rec03 \
    cdc2022-opioid-rec04 \
    cdc2022-opioid-rec05 \
    cdc2022-opioid-rec06 \
    cdc2022-opioid-rec07 \
    cdc2022-opioid-rec08 \
    cdc2022-opioid-rec09 \
    cdc2022-opioid-rec10 \
    cdc2022-opioid-rec11 \
    cdc2022-opioid-rec12 \
    >"$SCRATCH/expected/docids"

: >"$SCRATCH/expected/docs-files.unsorted"
: >"$SCRATCH/expected/golden-files.unsorted"
while IFS= read -r docid; do
    printf '%s\n' "$docid.ace" "$docid.ulex" \
        >>"$SCRATCH/expected/docs-files.unsorted"
    printf '%s\n' \
        "front/$docid.drs.pl" \
        "chain/$docid.ir.pl" \
        "chain/$docid.program.pl" \
        "chain/$docid.result.pl" \
        >>"$SCRATCH/expected/golden-files.unsorted"
done <"$SCRATCH/expected/docids"
printf '%s\n' manifest.pl front/manifest.pl \
    >>"$SCRATCH/expected/golden-files.unsorted"
sort "$SCRATCH/expected/docs-files.unsorted" >"$SCRATCH/expected/docs-files"
sort "$SCRATCH/expected/golden-files.unsorted" >"$SCRATCH/expected/golden-files"

find "$DOCS" -mindepth 1 -maxdepth 1 ! -type d -printf '%f\n' | sort \
    >"$SCRATCH/logs/docs-files.actual"
find "$GOLDEN" -mindepth 1 ! -type d -printf '%P\n' | sort \
    >"$SCRATCH/logs/golden-files.actual"
find "$DOCS" -mindepth 1 -maxdepth 1 -type f -name '*.ace' -printf '%f\n' | \
    command sed 's/\.ace$//' | sort >"$SCRATCH/logs/ace-docids.actual"
find "$DOCS" -mindepth 1 -maxdepth 1 -type f -name '*.ulex' -printf '%f\n' | \
    command sed 's/\.ulex$//' | sort >"$SCRATCH/logs/ulex-docids.actual"

if ! cmp -s "$SCRATCH/expected/docs-files" "$SCRATCH/logs/docs-files.actual" || \
   ! cmp -s "$SCRATCH/expected/golden-files" "$SCRATCH/logs/golden-files.actual" || \
   ! cmp -s "$SCRATCH/expected/docids" "$SCRATCH/logs/ace-docids.actual" || \
   ! cmp -s "$SCRATCH/expected/docids" "$SCRATCH/logs/ulex-docids.actual"; then
    fail_case "inventory/sets" "document or golden filename inventory differs"
fi
if [ -L "$GOLDEN/front" ] || [ -L "$GOLDEN/chain" ] || \
   ! [ -d "$GOLDEN/front" ] || ! [ -d "$GOLDEN/chain" ]; then
    fail_case "inventory/sets" "golden front/chain must be real directories"
fi
while IFS= read -r relpath; do
    if ! [ -f "$DOCS/$relpath" ] || [ -L "$DOCS/$relpath" ]; then
        fail_case "inventory/sets" "not a regular non-symlink file: $relpath"
    fi
done <"$SCRATCH/expected/docs-files"
while IFS= read -r relpath; do
    if ! [ -f "$GOLDEN/$relpath" ] || [ -L "$GOLDEN/$relpath" ]; then
        fail_case "inventory/sets" "not a regular non-symlink golden: $relpath"
    fi
done <"$SCRATCH/expected/golden-files"
pass_case "inventory/sets"

run_registry_tool "$REGISTRY" "$SCRATCH/logs/registry.stdout" \
    "$SCRATCH/logs/registry.stderr" registry
if [ "$RUN_STATUS" -ne 0 ] || [ -s "$SCRATCH/logs/registry.stdout" ] || \
   [ -s "$SCRATCH/logs/registry.stderr" ]; then
    fail_case "registry/registry" "expected rc 0 and zero streams, got rc $RUN_STATUS"
fi
pass_case "registry/registry"

run_registry_tool "$TERMINOLOGY" "$SCRATCH/logs/terminology.stdout" \
    "$SCRATCH/logs/terminology.stderr" terminology
if [ "$RUN_STATUS" -ne 0 ] || [ -s "$SCRATCH/logs/terminology.stdout" ] || \
   [ -s "$SCRATCH/logs/terminology.stderr" ]; then
    fail_case "registry/terminology" "expected rc 0 and zero streams, got rc $RUN_STATUS"
fi
pass_case "registry/terminology"

run_registry_tool "$MAPPING" "$SCRATCH/logs/mapping.stdout" \
    "$SCRATCH/logs/mapping.stderr" mapping
if [ "$RUN_STATUS" -ne 0 ] || [ -s "$SCRATCH/logs/mapping.stdout" ] || \
   [ -s "$SCRATCH/logs/mapping.stderr" ]; then
    fail_case "registry/mapping" "expected rc 0 and zero streams, got rc $RUN_STATUS"
fi
pass_case "registry/mapping"

run_registry_tool "$TERMINOLOGY" "$SCRATCH/logs/terminology.ulex.1" \
    "$SCRATCH/logs/terminology.ulex.1.stderr" ulex
if [ "$RUN_STATUS" -ne 0 ] || [ -s "$SCRATCH/logs/terminology.ulex.1.stderr" ] || \
   ! [ -s "$SCRATCH/logs/terminology.ulex.1" ]; then
    fail_case "ulex/emission" "expected rc 0, empty stderr, and nonempty output"
fi
pass_case "ulex/emission"

run_registry_tool "$TERMINOLOGY" "$SCRATCH/logs/terminology.ulex.2" \
    "$SCRATCH/logs/terminology.ulex.2.stderr" ulex
if [ "$RUN_STATUS" -ne 0 ] || [ -s "$SCRATCH/logs/terminology.ulex.2.stderr" ] || \
   ! cmp -s "$SCRATCH/logs/terminology.ulex.1" "$SCRATCH/logs/terminology.ulex.2"; then
    fail_case "ulex/determinism" "fresh emissions differ or second emission failed"
fi
pass_case "ulex/determinism"

while IFS= read -r docid; do
    if ! cmp -s "$SCRATCH/logs/terminology.ulex.1" "$DOCS/$docid.ulex"; then
        fail_case "ulex/sidecars" "emission differs from $docid.ulex"
    fi
done <"$SCRATCH/expected/docids"
pass_case "ulex/sidecars"

command sed -nE \
    "s|^mapping_document\\('cdc-2022-opioid','([^']+)',ace\\(relpath\\('([^']+)'\\),ace_sha256\\('?([0-9a-f]{64})'?\\)\\),ulex\\(relpath\\('([^']+)'\\),ulex_sha256\\('?([0-9a-f]{64})'?\\)\\)\\)\\.$|\\1\t\\2\t\\3\t\\4\t\\5|p" \
    "$MAPPING" >"$SCRATCH/mapping/documents"
mapping_document_count=$(wc -l <"$SCRATCH/mapping/documents")
command sed -E $'s/\t.*$//' "$SCRATCH/mapping/documents" \
    >"$SCRATCH/mapping/document-docids"
if [ "$mapping_document_count" -ne 12 ] || \
   ! cmp -s "$SCRATCH/expected/docids" "$SCRATCH/mapping/document-docids"; then
    fail_case "mapping/documents" "expected the exact 12 mapping_document rows"
fi
while IFS=$'\t' read -r docid ace_rel ace_hash ulex_rel ulex_hash; do
    if [ -z "$docid" ] || [ -z "$ace_hash" ] || [ -z "$ulex_hash" ] || \
       ! [ -f "$ROOT/$ace_rel" ] || [ -L "$ROOT/$ace_rel" ] || \
       ! [ -f "$ROOT/$ulex_rel" ] || [ -L "$ROOT/$ulex_rel" ]; then
        fail_case "mapping/documents" "missing regular non-symlink path for $docid"
    fi
done <"$SCRATCH/mapping/documents"
pass_case "mapping/documents"

while IFS=$'\t' read -r docid ace_rel ace_hash ulex_rel ulex_hash; do
    actual_ace_hash=$(sha256_file "$ROOT/$ace_rel")
    actual_ulex_hash=$(sha256_file "$ROOT/$ulex_rel")
    if [ "$actual_ace_hash" != "$ace_hash" ] || \
       [ "$actual_ulex_hash" != "$ulex_hash" ]; then
        fail_case "mapping/digests" "recorded digest differs for $docid"
    fi
done <"$SCRATCH/mapping/documents"
pass_case "mapping/digests"

if ! git status --porcelain --ignored -- vendor/ \
    >"$SCRATCH/logs/vendor.precopy.status"; then
    fail_case "vendor/precopy-clean" "git status failed"
fi
if [ -s "$SCRATCH/logs/vendor.precopy.status" ]; then
    fail_case "vendor/precopy-clean" "vendor tree is not clean"
fi
pass_case "vendor/precopy-clean"

mkdir -p "$TREE"
if ! cp -a "$ROOT/vendor/ape/." "$TREE/" \
    >"$SCRATCH/logs/vendor-copy.stdout" 2>"$SCRATCH/logs/vendor-copy.stderr"; then
    fail_case "vendor/copy" "cp -a failed"
fi
pass_case "vendor/copy"

if make -C "$TREE" plp "swipl=$SWIPL -f none" \
    >"$SCRATCH/logs/make.stdout" 2>"$SCRATCH/logs/make.stderr"; then
    :
else
    make_status=$?
    fail_case "vendor/plp" "make failed with rc $make_status"
fi
if ! [ -f "$TREE/prolog/parser/grammar.plp" ] || \
   [ -L "$TREE/prolog/parser/grammar.plp" ]; then
    fail_case "vendor/plp" "grammar.plp missing or not a regular file"
fi
pass_case "vendor/plp"

tree_content_hash "$TREE" "$SCRATCH/tree.hash.before"

run_pipeline "$SWIPL" "$DOCS" "$OUT1" \
    "$SCRATCH/logs/run1.stdout" "$SCRATCH/logs/run1.stderr"
if [ "$RUN_STATUS" -ne 0 ] || [ -s "$SCRATCH/logs/run1.stderr" ]; then
    fail_case "pipeline/run1" "expected rc 0 and empty stderr, got rc $RUN_STATUS"
fi
pass_case "pipeline/run1"

write_pipeline_stdout "$OUT1" "$SCRATCH/expected/run1.stdout"
if ! cmp -s "$SCRATCH/expected/run1.stdout" "$SCRATCH/logs/run1.stdout"; then
    fail_case "pipeline/run1-stdout" "stdout differs from the pinned 50-file report"
fi
pass_case "pipeline/run1-stdout"

if diff -r "$OUT1" "$GOLDEN" \
    >"$SCRATCH/logs/run1-golden.diff" 2>&1; then
    :
else
    diff_status=$?
    fail_case "pipeline/run1-golden" "diff -r failed with rc $diff_status"
fi
pass_case "pipeline/run1-golden"

run_pipeline "$SWIPL" "$DOCS" "$OUT2" \
    "$SCRATCH/logs/run2.stdout" "$SCRATCH/logs/run2.stderr"
if [ "$RUN_STATUS" -ne 0 ] || [ -s "$SCRATCH/logs/run2.stderr" ]; then
    fail_case "pipeline/run2" "expected rc 0 and empty stderr, got rc $RUN_STATUS"
fi
pass_case "pipeline/run2"

write_pipeline_stdout "$OUT2" "$SCRATCH/expected/run2.stdout"
if ! cmp -s "$SCRATCH/expected/run2.stdout" "$SCRATCH/logs/run2.stdout"; then
    fail_case "pipeline/run2-stdout" "stdout differs from the pinned 50-file report"
fi
pass_case "pipeline/run2-stdout"

if diff -r "$OUT2" "$GOLDEN" \
    >"$SCRATCH/logs/run2-golden.diff" 2>&1; then
    :
else
    diff_status=$?
    fail_case "pipeline/run2-golden" "diff -r failed with rc $diff_status"
fi
pass_case "pipeline/run2-golden"

if diff -r "$OUT1" "$OUT2" \
    >"$SCRATCH/logs/determinism.diff" 2>&1; then
    :
else
    diff_status=$?
    fail_case "pipeline/determinism" "fresh outputs differ, diff rc $diff_status"
fi
pass_case "pipeline/determinism"

command sed -nE \
    "s|^document\\(docid\\('([^']+)'\\),drs_sha256\\('([0-9a-f]{64})'\\),ir_sha256\\('([0-9a-f]{64})'\\),program_sha256\\('([0-9a-f]{64})'\\),result_sha256\\('([0-9a-f]{64})'\\),front_manifest_sha256\\('([0-9a-f]{64})'\\)\\)\\.$|\\1\t\\2\t\\3\t\\4\t\\5\t\\6|p" \
    "$OUT1/manifest.pl" >"$SCRATCH/mapping/manifest"
manifest_count=$(wc -l <"$SCRATCH/mapping/manifest")
command sed -E $'s/\t.*$//' "$SCRATCH/mapping/manifest" \
    >"$SCRATCH/mapping/manifest-docids"
if [ "$manifest_count" -ne 12 ] || \
   ! cmp -s "$SCRATCH/expected/docids" "$SCRATCH/mapping/manifest-docids"; then
    fail_case "pipeline/manifest" "expected the exact 12 manifest rows"
fi
while IFS=$'\t' read -r docid drs_hash ir_hash program_hash result_hash front_hash; do
    actual_drs_hash=$(sha256_file "$OUT1/front/$docid.drs.pl")
    actual_ir_hash=$(sha256_file "$OUT1/chain/$docid.ir.pl")
    actual_program_hash=$(sha256_file "$OUT1/chain/$docid.program.pl")
    actual_result_hash=$(sha256_file "$OUT1/chain/$docid.result.pl")
    actual_front_hash=$(sha256_file "$OUT1/front/manifest.pl")
    if [ "$actual_drs_hash" != "$drs_hash" ] || \
       [ "$actual_ir_hash" != "$ir_hash" ] || \
       [ "$actual_program_hash" != "$program_hash" ] || \
       [ "$actual_result_hash" != "$result_hash" ] || \
       [ "$actual_front_hash" != "$front_hash" ]; then
        fail_case "pipeline/manifest" "digest mismatch for $docid"
    fi
done <"$SCRATCH/mapping/manifest"
pass_case "pipeline/manifest"

command sed -nE \
    "s|^mapping_claim\\('cdc-2022-opioid','([^']+)','([^']+)',projection\\(([^)]+)\\),docid\\('([^']+)'\\),items\\((\\[.*\\])\\),expected_answer\\((.*)\\)\\)\\.$|\\1\t\\2\t\\3\t\\4\t\\5\t\\6|p" \
    "$MAPPING" >"$SCRATCH/mapping/claims"

check_mapping_claim() {
    local suffix expected_claim expected_projection label prefix row_path row_count
    local region claim_id projection docid items expected_answer result program
    local query_count rule_count query_id actual_answer expected_answer_path rule_id
    suffix=$1
    expected_claim=$2
    expected_projection=$3
    label="mapping/claim/rec$suffix"
    prefix="box3.rec.$suffix"$'\t'
    row_path="$SCRATCH/mapping/claim-rec$suffix"

    command grep -F "$prefix" "$SCRATCH/mapping/claims" >"$row_path" || :
    row_count=$(wc -l <"$row_path")
    if [ "$row_count" -ne 1 ]; then
        fail_case "$label" "expected exactly one extracted claim row"
    fi
    IFS=$'\t' read -r region claim_id projection docid items expected_answer <"$row_path"
    if [ "$region" != "box3.rec.$suffix" ] || \
       [ "$claim_id" != "$expected_claim" ] || \
       [ "$projection" != "$expected_projection" ] || \
       [ "$docid" != "cdc2022-opioid-rec$suffix" ]; then
        fail_case "$label" "claim identity or projection differs"
    fi

    result="$OUT1/chain/$docid.result.pl"
    program="$OUT1/chain/$docid.program.pl"
    command grep -oE 'query_id\(sentence\([0-9]+\),clause\([0-9]+\)\)' \
        <<<"$items" >"$row_path.queries" || :
    command grep -oE 'rule_id\(sentence\([0-9]+\),clause\([0-9]+\)\)' \
        <<<"$items" >"$row_path.rules" || :
    query_count=$(wc -l <"$row_path.queries")
    rule_count=$(wc -l <"$row_path.rules")
    if [ "$query_count" -ne 1 ] || [ "$rule_count" -lt 1 ]; then
        fail_case "$label" "items must contain one query ID and at least one rule ID"
    fi
    query_id=$(<"$row_path.queries")

    command grep '^answer(' "$result" >"$row_path.answer.actual" || :
    actual_answer=$(wc -l <"$row_path.answer.actual")
    if [ "$actual_answer" -ne 1 ]; then
        fail_case "$label" "fresh result must contain exactly one answer line"
    fi
    expected_answer_path="$row_path.answer.expected"
    printf '%s.\n' "$expected_answer" >"$expected_answer_path"
    if ! cmp -s "$expected_answer_path" "$row_path.answer.actual"; then
        fail_case "$label" "expected_answer differs from fresh result bytes"
    fi
    if ! command grep -Fq "$query_id" "$result"; then
        fail_case "$label" "query ID absent from fresh result"
    fi
    while IFS= read -r rule_id; do
        if ! command grep -Fq "$rule_id" "$program"; then
            fail_case "$label" "rule ID absent from fresh program: $rule_id"
        fi
    done <"$row_path.rules"
    pass_case "$label"
}

check_mapping_claim 01 claim.rec01.action-kind action_kind
check_mapping_claim 02 claim.rec02.action-kind action_kind
check_mapping_claim 03 claim.rec03.action-kind action_kind
check_mapping_claim 04 claim.rec04.action-kind action_kind
check_mapping_claim 05 claim.rec05.applicability applicability
check_mapping_claim 06 claim.rec06.action-kind action_kind
check_mapping_claim 07 claim.rec07.action-kind action_kind
check_mapping_claim 08 claim.rec08.action-kind action_kind
check_mapping_claim 09 claim.rec09.action-kind action_kind
check_mapping_claim 10 claim.rec10.applicability applicability
check_mapping_claim 11 claim.rec11.action-kind action_kind
check_mapping_claim 12 claim.rec12.action-kind action_kind

printf '%s\n' \
    box3.rec.01 box3.rec.02 box3.rec.03 box3.rec.04 \
    box3.rec.05 box3.rec.06 box3.rec.07 box3.rec.08 \
    box3.rec.09 box3.rec.10 box3.rec.11 box3.rec.12 \
    >"$SCRATCH/expected/claim-regions"
printf '%s\n' \
    claim.rec01.action-kind \
    claim.rec02.action-kind \
    claim.rec03.action-kind \
    claim.rec04.action-kind \
    claim.rec05.applicability \
    claim.rec06.action-kind \
    claim.rec07.action-kind \
    claim.rec08.action-kind \
    claim.rec09.action-kind \
    claim.rec10.applicability \
    claim.rec11.action-kind \
    claim.rec12.action-kind \
    >"$SCRATCH/expected/claim-ids"
command sed -E $'s/\t.*$//' "$SCRATCH/mapping/claims" \
    >"$SCRATCH/mapping/claim-regions"
command sed -E $'s/^[^\t]*\t//; s/\t.*$//' "$SCRATCH/mapping/claims" \
    >"$SCRATCH/mapping/claim-ids"
command sed -E $'s/^[^\t]*\t[^\t]*\t[^\t]*\t//; s/\t.*$//' \
    "$SCRATCH/mapping/claims" >"$SCRATCH/mapping/claim-docids"
claim_count=$(wc -l <"$SCRATCH/mapping/claims")
if [ "$claim_count" -ne 12 ] || \
   ! cmp -s "$SCRATCH/expected/claim-regions" "$SCRATCH/mapping/claim-regions" || \
   ! cmp -s "$SCRATCH/expected/claim-ids" "$SCRATCH/mapping/claim-ids" || \
   ! cmp -s "$SCRATCH/expected/docids" "$SCRATCH/mapping/claim-docids"; then
    fail_case "mapping/claims-inventory" "claim region, ID, or docid inventory differs"
fi
pass_case "mapping/claims-inventory"

printf '%s\n' \
    box3.grp.01 box3.grp.02 box3.grp.03 box3.grp.04 \
    box3.rec.01 box3.rec.02 box3.rec.03 box3.rec.04 \
    box3.rec.05 box3.rec.06 box3.rec.07 box3.rec.08 \
    box3.rec.09 box3.rec.10 box3.rec.11 box3.rec.12 \
    >"$SCRATCH/expected/regions"
command sed -nE \
    "s|^mapping_region\\('cdc-2022-opioid','([^']+)'\\)\\.$|\\1|p" \
    "$MAPPING" >"$SCRATCH/mapping/regions"
command sed -nE \
    "s|^guideline_region\\('cdc-2022-opioid','([^']+)',.*$|\\1|p" \
    "$REGISTRY" >"$SCRATCH/mapping/registry-regions"
if ! cmp -s "$SCRATCH/expected/regions" "$SCRATCH/mapping/regions" || \
   ! cmp -s "$SCRATCH/expected/regions" "$SCRATCH/mapping/registry-regions"; then
    fail_case "mapping/regions-inventory" "mapping and registry region inventories differ"
fi
pass_case "mapping/regions-inventory"

command sed -nE \
    "s|^mapping_claim\\('cdc-2022-opioid','([^']+)'.*$|\\1|p" \
    "$MAPPING" >"$SCRATCH/mapping/coverage.unsorted"
command sed -nE \
    "s|^mapping_residual\\('cdc-2022-opioid','([^']+)'.*$|\\1|p" \
    "$MAPPING" >>"$SCRATCH/mapping/coverage.unsorted"
sort -u "$SCRATCH/mapping/coverage.unsorted" >"$SCRATCH/mapping/coverage"
sort "$SCRATCH/expected/regions" >"$SCRATCH/expected/regions.sorted"
if ! cmp -s "$SCRATCH/expected/regions.sorted" "$SCRATCH/mapping/coverage"; then
    fail_case "mapping/coverage" "at least one region lacks a claim or residual"
fi
command sed -nE \
    "s|^mapping_residual\\('cdc-2022-opioid','([^']+)'.*$|\\1|p" \
    "$MAPPING" >"$SCRATCH/mapping/residual-regions"
while IFS= read -r claimed_region; do
    claim_residual_count=$(command grep -Fxc "$claimed_region" \
        "$SCRATCH/mapping/residual-regions" || :)
    if [ "$claim_residual_count" -lt 1 ]; then
        fail_case "mapping/coverage" "claimed region lacks a residual: $claimed_region"
    fi
done <"$SCRATCH/mapping/claim-regions"
pass_case "mapping/coverage"

for group_region in box3.grp.01 box3.grp.02 box3.grp.03 box3.grp.04; do
    group_residual_count=$(command grep -Fxc "$group_region" \
        "$SCRATCH/mapping/residual-regions" || :)
    if [ "$group_residual_count" -lt 1 ]; then
        fail_case "mapping/group-residuals" "no residual for $group_region"
    fi
done
pass_case "mapping/group-residuals"

real_swipl=$(command -v "$SWIPL" 2>/dev/null || :)
if [ -z "$real_swipl" ]; then
    fail_case "recovery/interrupted-rc-137" "cannot resolve SWIPL executable"
fi
interrupt_swipl="$SCRATCH/recovery/swipl-interrupt"
{
    printf '%s\n' '#!/usr/bin/env bash' 'set -eu'
    printf 'REAL_SWIPL=%q\n' "$real_swipl"
    printf '%s\n' \
        "last=" \
        "for arg in \"\$@\"; do" \
        "    last=\$arg" \
        "done" \
        "if [ \"\$last\" = run ]; then" \
        "    kill -9 \"\$PPID\"" \
        "    exit 1" \
        "fi" \
        "exec \"\$REAL_SWIPL\" \"\$@\""
} >"$interrupt_swipl"
chmod 755 "$interrupt_swipl"

run_pipeline "$interrupt_swipl" "$DOCS" "$OUT3" \
    "$SCRATCH/recovery/interrupted.stdout" \
    "$SCRATCH/recovery/interrupted.stderr"
write_staging_entries "$OUT3" "$SCRATCH/recovery/staging.entries"
staging_count=$(wc -l <"$SCRATCH/recovery/staging.entries")
if [ "$RUN_STATUS" -ne 137 ] || [ -e "$OUT3" ] || [ -L "$OUT3" ] || \
   [ "$staging_count" -ne 1 ]; then
    fail_case "recovery/interrupted-rc-137" \
        "expected rc 137, absent output, and one staging entry; got rc $RUN_STATUS and $staging_count entries"
fi
stale_entry=$(<"$SCRATCH/recovery/staging.entries")
stale_basename=${stale_entry##*/}
if ! [ -d "$stale_entry" ] || [ -L "$stale_entry" ] || \
   ! [ -f "$stale_entry/front/manifest.pl" ] || \
   [ -L "$stale_entry/front/manifest.pl" ] || \
   ! [ -f "$stale_entry/chain/cdc2022-opioid-rec01.ir.pl" ] || \
   [ -L "$stale_entry/chain/cdc2022-opioid-rec01.ir.pl" ] || \
   ! [ -f "$stale_entry/chain/cdc2022-opioid-rec01.program.pl" ] || \
   [ -L "$stale_entry/chain/cdc2022-opioid-rec01.program.pl" ]; then
    fail_case "recovery/interrupted-rc-137" \
        "partial front manifest and rec01 IR/program are not visible"
fi
pass_case "recovery/interrupted-rc-137"

run_pipeline "$SWIPL" "$DOCS" "$OUT3" \
    "$SCRATCH/recovery/stale.stdout" "$SCRATCH/recovery/stale.stderr"
printf 'pipeline: staging: stale staging: %s\n' "$stale_basename" \
    >"$SCRATCH/recovery/stale.expected"
if [ "$RUN_STATUS" -ne 2 ] || [ -s "$SCRATCH/recovery/stale.stdout" ] || \
   ! cmp -s "$SCRATCH/recovery/stale.expected" "$SCRATCH/recovery/stale.stderr" || \
   [ -e "$OUT3" ] || [ -L "$OUT3" ]; then
    fail_case "recovery/stale-refusal" "stale staging refusal differs"
fi
pass_case "recovery/stale-refusal"

rm -rf -- "$stale_entry"
run_pipeline "$SWIPL" "$DOCS" "$OUT3" \
    "$SCRATCH/recovery/rerun.stdout" "$SCRATCH/recovery/rerun.stderr"
write_pipeline_stdout "$OUT3" "$SCRATCH/expected/rerun.stdout"
if [ "$RUN_STATUS" -ne 0 ] || [ -s "$SCRATCH/recovery/rerun.stderr" ] || \
   ! cmp -s "$SCRATCH/expected/rerun.stdout" "$SCRATCH/recovery/rerun.stdout"; then
    fail_case "recovery/rerun" "clean rerun status or streams differ"
fi
if ! diff -r "$OUT3" "$GOLDEN" \
    >"$SCRATCH/recovery/rerun-golden.diff" 2>&1 || \
   ! diff -r "$OUT3" "$OUT1" \
    >"$SCRATCH/recovery/rerun-run1.diff" 2>&1; then
    fail_case "recovery/rerun" "recovered output differs from golden or run1"
fi
write_staging_entries "$OUT3" "$SCRATCH/recovery/rerun-staging.entries"
if [ -s "$SCRATCH/recovery/rerun-staging.entries" ]; then
    fail_case "recovery/rerun" "staging residue remains after successful rerun"
fi
pass_case "recovery/rerun"

removed_entry_count=$(command grep -Fc \
    "lex.acute-pain-clinician.noun-sg" "$TERMINOLOGY" || :)
if [ "$removed_entry_count" -ne 1 ]; then
    fail_case "clex-independence" "expected exactly one terminology row to remove"
fi
command grep -Fv "lex.acute-pain-clinician.noun-sg" "$TERMINOLOGY" \
    >"$SCRATCH/clex/terminology.pl"
if command grep -Fq "lex.acute-pain-clinician.noun-sg" \
    "$SCRATCH/clex/terminology.pl"; then
    fail_case "clex-independence" "target terminology row remains"
fi
mkdir -p "$SCRATCH/clex/docs"
cp "$DOCS/cdc2022-opioid-rec01.ace" \
    "$SCRATCH/clex/docs/cdc2022-opioid-rec01.ace"
run_registry_tool "$SCRATCH/clex/terminology.pl" \
    "$SCRATCH/clex/terminology.stdout" \
    "$SCRATCH/clex/terminology.stderr" terminology
if [ "$RUN_STATUS" -ne 0 ] || [ -s "$SCRATCH/clex/terminology.stdout" ] || \
   [ -s "$SCRATCH/clex/terminology.stderr" ]; then
    fail_case "clex-independence" "mutated terminology validation failed"
fi
run_registry_tool "$SCRATCH/clex/terminology.pl" \
    "$SCRATCH/clex/docs/cdc2022-opioid-rec01.ulex" \
    "$SCRATCH/clex/ulex.stderr" ulex
if [ "$RUN_STATUS" -ne 0 ] || [ -s "$SCRATCH/clex/ulex.stderr" ] || \
   ! [ -s "$SCRATCH/clex/docs/cdc2022-opioid-rec01.ulex" ]; then
    fail_case "clex-independence" "mutated Ulex emission failed"
fi
run_pipeline "$SWIPL" "$SCRATCH/clex/docs" "$OUT4" \
    "$SCRATCH/clex/pipeline.stdout" "$SCRATCH/clex/pipeline.stderr"
write_staging_entries "$OUT4" "$SCRATCH/clex/staging.entries"
clex_stderr_lines=$(wc -l <"$SCRATCH/clex/pipeline.stderr")
if [ "$RUN_STATUS" -ne 1 ] || [ -s "$SCRATCH/clex/pipeline.stdout" ] || \
   [ -e "$OUT4" ] || [ -L "$OUT4" ] || \
   [ -s "$SCRATCH/clex/staging.entries" ] || [ "$clex_stderr_lines" -ne 1 ]; then
    fail_case "clex-independence" "expected clean rc 1 OOV rejection"
fi
if ! printf '%s\n' "$(<"$SCRATCH/clex/pipeline.stderr")" | \
        cmp -s - "$SCRATCH/clex/pipeline.stderr" || \
   ! command grep -Eq '^adapter_error\(ape_messages,.*\)\.$' \
        "$SCRATCH/clex/pipeline.stderr" || \
   ! command grep -Fq 'acute-pain-clinician' \
        "$SCRATCH/clex/pipeline.stderr"; then
    fail_case "clex-independence" "stderr is not the exact one-line OOV adapter error"
fi
pass_case "clex-independence"

tree_content_hash "$TREE" "$SCRATCH/tree.hash.after"
if ! cmp -s "$SCRATCH/tree.hash.before" "$SCRATCH/tree.hash.after"; then
    fail_case "tree/immutable" "APE tree content changed after pipeline runs"
fi
pass_case "tree/immutable"

if git diff --exit-code -- vendor/ \
    >"$SCRATCH/logs/vendor.diff" 2>&1; then
    :
else
    vendor_diff_status=$?
    fail_case "vendor/clean" "git diff failed with rc $vendor_diff_status"
fi
if ! git status --porcelain -- vendor/ >"$SCRATCH/logs/vendor.status"; then
    fail_case "vendor/clean" "git status failed"
fi
if [ -s "$SCRATCH/logs/vendor.status" ]; then
    fail_case "vendor/clean" "vendor status is not clean"
fi
pass_case "vendor/clean"

if ! git status --porcelain >"$SCRATCH/status.after"; then
    fail_case "repository/cleanliness" "git status failed"
fi
if ! cmp -s "$SCRATCH/status.before" "$SCRATCH/status.after"; then
    fail_case "repository/cleanliness" "repository status changed during harness"
fi
pass_case "repository/cleanliness"

if [ "$PASS_COUNT" -ne "$EXPECTED_PASS_COUNT" ]; then
    fail_case "harness/pass-count" \
        "expected $EXPECTED_PASS_COUNT passes, got $PASS_COUNT"
fi
printf 'SUMMARY: %s passed, 0 failed\n' "$PASS_COUNT"
