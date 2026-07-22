#!/usr/bin/env bash
set -eu

ROOT=$PWD
if ! [ -f src/prolog/registry_tool.pl ] || \
        ! [ -f guidelines/registry.pl ] || \
        ! [ -f guidelines/cdc-2022-opioid/source/rr7103a1-H.pdf ] || \
        ! [ -f guidelines/cdc-2022-opioid/source/box3-extraction.txt ] || \
        ! [ -d tests/fixtures/registry/green ] || \
        ! [ -d tests/fixtures/registry/red ] || \
        ! [ -d tests/fixtures/registry/golden ]; then
    printf 'FAIL repo-root: run from cnl-ckc repository root\n'
    printf 'SUMMARY: 0 passed, 1 failed\n'
    exit 1
fi

SWIPL=${SWIPL:-swipl}
GREEN="$ROOT/tests/fixtures/registry/green"
RED="$ROOT/tests/fixtures/registry/red"
GOLDEN="$ROOT/tests/fixtures/registry/golden"
REGISTRY="$ROOT/guidelines/registry.pl"
PDF="$ROOT/guidelines/cdc-2022-opioid/source/rr7103a1-H.pdf"
EXTRACTION="$ROOT/guidelines/cdc-2022-opioid/source/box3-extraction.txt"
SCRATCH="${TMPDIR:-/tmp}/cnl-ckc-registry-harness.$$"
PASS_COUNT=0
RUN_STATUS=0
EXPECTED_PASS_COUNT=62

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

    if "$SWIPL" -q -f none -F none \
        -s "$ROOT/src/prolog/registry_tool.pl" -g main -t 'halt(9)' -- "$@" \
        <"$input" >"$stdout_path" 2>"$stderr_path"; then
        RUN_STATUS=0
    else
        RUN_STATUS=$?
    fi
}

check_success() {
    local label stdout_path stderr_path
    label=$1
    stdout_path=$2
    stderr_path=$3

    if [ "$RUN_STATUS" -ne 0 ]; then
        fail_case "$label/status" "expected 0, got $RUN_STATUS"
    fi
    if [ -s "$stdout_path" ]; then
        fail_case "$label/stdout" "expected zero bytes"
    fi
    if [ -s "$stderr_path" ]; then
        fail_case "$label/stderr" "expected zero bytes"
    fi
    pass_case "$label"
}

check_rejection() {
    local label expected_status stdout_path stderr_path expected_line line_count
    label=$1
    expected_status=$2
    stdout_path=$3
    stderr_path=$4
    expected_line=$5

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
    if ! printf '%s\n' "$expected_line" | cmp - "$stderr_path"; then
        fail_case "$label/stderr" "stderr differs from exact expected line"
    fi
    pass_case "$label"
}

rm -rf "$SCRATCH"
mkdir -p "$SCRATCH/green" "$SCRATCH/red" "$SCRATCH/usage" \
    "$SCRATCH/real" "$SCRATCH/regions"
trap 'rm -rf "$SCRATCH"' EXIT

if ! git status --porcelain >"$SCRATCH/status.before"; then
    fail_case "repository/status-before" "git status failed"
fi

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

set -- "$GREEN"/*.pl
if [ "$#" -ne 2 ]; then
    fail_case "fixtures/count" "expected 2 green Prolog fixtures, got $#"
fi
set -- "$RED"/*.pl
if [ "$#" -ne 15 ]; then
    fail_case "fixtures/count" "expected 15 red Prolog fixtures, got $#"
fi
set -- "$RED"/*.bin
if [ "$#" -ne 8 ]; then
    fail_case "fixtures/count" "expected 8 red binary fixtures, got $#"
fi
if ! [ -f "$GOLDEN/terminology.ulex" ]; then
    fail_case "fixtures/count" "missing golden Ulex fixture"
fi
pass_case "fixtures/count"

template_count=$(command grep -c '^terminology_entry' \
    "$GREEN/terminology.pl" || :)
golden_template_count=$(command grep -c '^' "$GOLDEN/terminology.ulex" || :)
if [ "$template_count" -ne 27 ] || [ "$golden_template_count" -ne 27 ]; then
    fail_case "fixtures/templates" \
        "expected 27 terminology and golden rows, got $template_count and $golden_template_count"
fi
pass_case "fixtures/templates"

registry_stdout="$SCRATCH/green/registry.stdout"
registry_stderr="$SCRATCH/green/registry.stderr"
run_tool "$GREEN/registry.pl" "$registry_stdout" "$registry_stderr" registry
check_success "green/registry" "$registry_stdout" "$registry_stderr"

terminology_stdout="$SCRATCH/green/terminology.stdout"
terminology_stderr="$SCRATCH/green/terminology.stderr"
run_tool "$GREEN/terminology.pl" "$terminology_stdout" \
    "$terminology_stderr" terminology
check_success "green/terminology" "$terminology_stdout" "$terminology_stderr"

ulex_stdout1="$SCRATCH/green/terminology.ulex.1"
ulex_stderr1="$SCRATCH/green/terminology.ulex.1.stderr"
run_tool "$GREEN/terminology.pl" "$ulex_stdout1" "$ulex_stderr1" ulex
if [ "$RUN_STATUS" -ne 0 ]; then
    fail_case "green/ulex-golden/status" "expected 0, got $RUN_STATUS"
fi
if [ -s "$ulex_stderr1" ]; then
    fail_case "green/ulex-golden/stderr" "expected zero bytes"
fi
if ! cmp "$GOLDEN/terminology.ulex" "$ulex_stdout1"; then
    fail_case "green/ulex-golden/bytes" "emission differs from golden"
fi
pass_case "green/ulex-golden"

ulex_stdout2="$SCRATCH/green/terminology.ulex.2"
ulex_stderr2="$SCRATCH/green/terminology.ulex.2.stderr"
run_tool "$GREEN/terminology.pl" "$ulex_stdout2" "$ulex_stderr2" ulex
if [ "$RUN_STATUS" -ne 0 ]; then
    fail_case "determinism/ulex/status" "expected 0, got $RUN_STATUS"
fi
if [ -s "$ulex_stderr2" ]; then
    fail_case "determinism/ulex/stderr" "expected zero bytes"
fi
if ! cmp "$ulex_stdout1" "$ulex_stdout2"; then
    fail_case "determinism/ulex/bytes" "fresh-process emissions differ"
fi
pass_case "determinism/ulex"

run_red() {
    local name command_name expected_line stdout_path stderr_path
    name=$1
    command_name=$2
    expected_line=$3
    stdout_path="$SCRATCH/red/$name.stdout"
    stderr_path="$SCRATCH/red/$name.stderr"
    run_tool "$RED/$name" "$stdout_path" "$stderr_path" "$command_name"
    check_rejection "red/$name" 1 "$stdout_path" "$stderr_path" \
        "$expected_line"
}

run_red invalid-utf8.bin registry \
    'registry_tool_error(registry,input_utf8,byte_offset(0)).'
run_red utf8-bare-continuation.bin registry \
    'registry_tool_error(registry,input_utf8,byte_offset(0)).'
run_red utf8-overlong.bin registry \
    'registry_tool_error(registry,input_utf8,byte_offset(0)).'
run_red utf8-overlong-3byte.bin registry \
    'registry_tool_error(registry,input_utf8,byte_offset(0)).'
run_red utf8-surrogate.bin registry \
    'registry_tool_error(registry,input_utf8,byte_offset(0)).'
run_red utf8-out-of-range.bin registry \
    'registry_tool_error(registry,input_utf8,byte_offset(0)).'
run_red utf8-truncated.bin registry \
    'registry_tool_error(registry,input_utf8,byte_offset(0)).'
run_red utf8-nonzero-offset.bin registry \
    'registry_tool_error(registry,input_utf8,byte_offset(1)).'
run_red registry-syntax.pl registry \
    'registry_tool_error(registry,syntax,term(2)).'
run_red registry-canonical-spacing.pl registry \
    'registry_tool_error(registry,canonical,codepoint_offset(26)).'
run_red registry-missing-version.pl registry \
    'registry_tool_error(registry,version,term(1,missing(cnl_guideline_registry(1)))).'
run_red registry-wrong-version.pl registry \
    'registry_tool_error(registry,version,term(1,expected(cnl_guideline_registry(1),found(cnl_guideline_registry(2))))).'
run_red registry-unknown-row.pl registry \
    'registry_tool_error(registry,row,term(2,unknown_constructor(mystery,1))).'
run_red registry-bad-digest-length.pl registry \
    'registry_tool_error(registry,digest,term(2,digest(artifact_sha256,length(expected(64),found(63))))).'
run_red registry-bad-digest-uppercase.pl registry \
    'registry_tool_error(registry,digest,term(2,digest(artifact_sha256,lower_hex))).'
run_red registry-bad-range.pl registry \
    'registry_tool_error(registry,range,term(4,byte_range(1,1))).'
run_red registry-duplicate-region.pl registry \
    "registry_tool_error(registry,duplicate,term(7,duplicate(region_id,'box.rec.01',first_term(4))))."
run_red registry-native-variable.pl registry \
    'registry_tool_error(registry,shape,term(2,field(may_commit))).'
run_red registry-native-boolean-variable.pl registry \
    'registry_tool_error(registry,shape,term(2,field(byte_identical))).'
run_red terminology-duplicate-key.pl terminology \
    'registry_tool_error(terminology,duplicate,term(3,duplicate(terminology_key,[adv,quickly],first_term(2)))).'
run_red terminology-unknown-template.pl terminology \
    'registry_tool_error(terminology,template,kind(unknown_template)).'
run_red terminology-bad-gender.pl terminology \
    'registry_tool_error(terminology,gender,term(2,kind(noun_sg,value(robot)))).'
run_red terminology-unsorted.pl terminology \
    'registry_tool_error(terminology,ordering,term(3,terminology_key(adv,quickly,after(noun_sg,patient,previous_term(2))))).'

printf 'cnl_guideline_registry(1).\r\n' >"$SCRATCH/red/crlf.pl"
printf 'cnl_guideline_registry(1).' >"$SCRATCH/red/missing-final-lf.pl"
for scratch_name in crlf missing-final-lf; do
    scratch_stdout="$SCRATCH/red/$scratch_name.stdout"
    scratch_stderr="$SCRATCH/red/$scratch_name.stderr"
    run_tool "$SCRATCH/red/$scratch_name.pl" "$scratch_stdout" \
        "$scratch_stderr" registry
    check_rejection "scratch/$scratch_name" 1 \
        "$scratch_stdout" "$scratch_stderr" \
        'registry_tool_error(registry,canonical,codepoint_offset(26)).'
done

swap_stdout="$SCRATCH/red/swap-registry-to-terminology.stdout"
swap_stderr="$SCRATCH/red/swap-registry-to-terminology.stderr"
run_tool "$GREEN/registry.pl" "$swap_stdout" "$swap_stderr" terminology
check_rejection "stage-swap/registry-to-terminology" 1 \
    "$swap_stdout" "$swap_stderr" \
    'registry_tool_error(terminology,version,term(1,expected(cnl_guideline_terminology(1),found(cnl_guideline_registry(1))))).'

swap_stdout="$SCRATCH/red/swap-terminology-to-registry.stdout"
swap_stderr="$SCRATCH/red/swap-terminology-to-registry.stderr"
run_tool "$GREEN/terminology.pl" "$swap_stdout" "$swap_stderr" registry
check_rejection "stage-swap/terminology-to-registry" 1 \
    "$swap_stdout" "$swap_stderr" \
    'registry_tool_error(registry,version,term(1,expected(cnl_guideline_registry(1),found(cnl_guideline_terminology(1))))).'

swap_stdout="$SCRATCH/red/swap-registry-to-ulex.stdout"
swap_stderr="$SCRATCH/red/swap-registry-to-ulex.stderr"
run_tool "$GREEN/registry.pl" "$swap_stdout" "$swap_stderr" ulex
check_rejection "stage-swap/registry-to-ulex" 1 \
    "$swap_stdout" "$swap_stderr" \
    'registry_tool_error(ulex,version,term(1,expected(cnl_guideline_terminology(1),found(cnl_guideline_registry(1))))).'

usage_stdout="$SCRATCH/usage/no-args.stdout"
usage_stderr="$SCRATCH/usage/no-args.stderr"
run_tool "$GREEN/registry.pl" "$usage_stdout" "$usage_stderr"
check_rejection "usage/no-args" 2 "$usage_stdout" "$usage_stderr" \
    'registry_tool_error(cli,usage,argv([])).'

usage_stdout="$SCRATCH/usage/unknown.stdout"
usage_stderr="$SCRATCH/usage/unknown.stderr"
run_tool "$GREEN/registry.pl" "$usage_stdout" "$usage_stderr" unknown
check_rejection "usage/unknown" 2 "$usage_stdout" "$usage_stderr" \
    'registry_tool_error(cli,usage,argv([unknown])).'

real_stdout="$SCRATCH/real/registry.stdout"
real_stderr="$SCRATCH/real/registry.stderr"
run_tool "$REGISTRY" "$real_stdout" "$real_stderr" registry
check_success "real/registry" "$real_stdout" "$real_stderr"

artifact_fields=$(command grep -oE \
    "artifact\\(relpath\\('[^']+'\\),artifact_sha256\\('?([0-9a-f]{64})'?\\),byte_length\\([0-9]+\\),media_type\\('[^']+'\\)\\)" \
    "$REGISTRY" || :)
artifact_field_count=$(printf '%s\n' "$artifact_fields" | \
    command grep -c '^artifact(' || :)
if [ "$artifact_field_count" -ne 1 ]; then
    fail_case "real/artifact-metadata" \
        "expected one artifact record, got $artifact_field_count"
fi
artifact_metadata=$(printf '%s\n' "$artifact_fields" | command sed -E \
    "s/^artifact\\(relpath\\('([^']+)'\\),artifact_sha256\\('?([0-9a-f]{64})'?\\),byte_length\\(([0-9]+)\\),media_type\\('([^']+)'\\)\\)$/\\1 \\2 \\3 \\4/")
IFS=' ' read -r registered_pdf_path registered_pdf_hash \
    registered_pdf_bytes registered_pdf_media <<EOF
$artifact_metadata
EOF
if [ "$registered_pdf_path" != "${PDF#"$ROOT"/}" ] || \
        [ "$registered_pdf_media" != application/pdf ] || \
        [ "$registered_pdf_hash" != \
            f4e5098d13e9b3dc5cc27bb90137df57d3667350b2add885fd367f279402d18d ] || \
        [ "$registered_pdf_bytes" -ne 1418584 ]; then
    fail_case "real/artifact-metadata" "registered PDF metadata differs"
fi
pass_case "real/artifact-metadata"

pdf_output=$(sha256sum "$PDF")
pdf_hash=${pdf_output%% *}
if [ "$pdf_hash" != "$registered_pdf_hash" ]; then
    fail_case "real/pdf-sha256" \
        "registered $registered_pdf_hash, got $pdf_hash"
fi
pass_case "real/pdf-sha256"

pdf_bytes=$(wc -c <"$PDF")
if [ "$pdf_bytes" -ne "$registered_pdf_bytes" ]; then
    fail_case "real/pdf-bytes" \
        "registered $registered_pdf_bytes, got $pdf_bytes"
fi
pass_case "real/pdf-bytes"

extraction_rows=$(command grep -E \
    "^extraction_evidence\\('cdc-2022-opioid',box3," "$REGISTRY" || :)
extraction_row_count=$(printf '%s\n' "$extraction_rows" | \
    command grep -c '^extraction_evidence' || :)
if [ "$extraction_row_count" -ne 1 ]; then
    fail_case "real/extraction-metadata" \
        "expected one Box 3 extraction row, got $extraction_row_count"
fi
extraction_metadata=$(printf '%s\n' "$extraction_rows" | command sed -nE \
    "s/^extraction_evidence\\('cdc-2022-opioid',box3,relpath\\('([^']+)'\\),extraction_sha256\\('?([0-9a-f]{64})'?\\),artifact_relpath\\('([^']+)'\\)\\)\\.$/\\1 \\2 \\3/p")
IFS=' ' read -r registered_extraction_path registered_extraction_hash \
    extraction_artifact_path <<EOF
$extraction_metadata
EOF
if [ "$registered_extraction_path" != "${EXTRACTION#"$ROOT"/}" ] || \
        [ "$extraction_artifact_path" != "$registered_pdf_path" ]; then
    fail_case "real/extraction-metadata" \
        "registered extraction paths do not bind to the PDF"
fi
pass_case "real/extraction-metadata"

extraction_output=$(sha256sum "$EXTRACTION")
actual_extraction_hash=${extraction_output%% *}
if [ "$actual_extraction_hash" != "$registered_extraction_hash" ]; then
    fail_case "real/extraction-sha256" \
        "registered $registered_extraction_hash, got $actual_extraction_hash"
fi
pass_case "real/extraction-sha256"

region_manifest="$SCRATCH/regions/manifest"
command sed -nE \
    "s/^guideline_region\\('cdc-2022-opioid','([^']+)',([^,]+),pdf_pages\\(([0-9]+),([0-9]+),([0-9]+),([0-9]+)\\),byte_range\\(([0-9]+),([0-9]+)\\),region_sha256\\('?([0-9a-f]{64})'?\\)\\)\\.$/\\1 \\2 \\3 \\4 \\5 \\6 \\7 \\8 \\9/p" \
    "$REGISTRY" >"$region_manifest"
region_count=$(wc -l <"$region_manifest")
if [ "$region_count" -ne 16 ]; then
    fail_case "real/regions/count" "expected 16, got $region_count"
fi
pass_case "real/regions/count"

expected_region_ids="$SCRATCH/regions/expected-ids"
printf '%s\n' \
    box3.grp.01 box3.grp.02 box3.grp.03 box3.grp.04 \
    box3.rec.01 box3.rec.02 box3.rec.03 box3.rec.04 \
    box3.rec.05 box3.rec.06 box3.rec.07 box3.rec.08 \
    box3.rec.09 box3.rec.10 box3.rec.11 box3.rec.12 \
    >"$expected_region_ids"
command cut -d' ' -f1 "$region_manifest" >"$SCRATCH/regions/actual-ids"
if ! cmp "$expected_region_ids" "$SCRATCH/regions/actual-ids"; then
    fail_case "real/regions/inventory" "region ID inventory differs"
fi
pass_case "real/regions/inventory"

while IFS=' ' read -r region_id extraction_id physical_first physical_last \
        printed_first printed_last start end expected_hash; do
    if [ "$extraction_id" != box3 ]; then
        fail_case "real/region/$region_id" \
            "expected extraction ID box3, got $extraction_id"
    fi
    case $region_id in
        box3.grp.01|box3.grp.02|box3.grp.03|box3.rec.01|box3.rec.02|\
        box3.rec.03|box3.rec.04|box3.rec.05|box3.rec.06|box3.rec.07)
            expected_pages='13 13 11 11'
            ;;
        *)
            expected_pages='14 14 12 12'
            ;;
    esac
    actual_pages="$physical_first $physical_last $printed_first $printed_last"
    if [ "$actual_pages" != "$expected_pages" ]; then
        fail_case "real/region/$region_id" \
            "expected pages $expected_pages, got $actual_pages"
    fi
    length=$((end - start))
    dd_slice="$SCRATCH/regions/$region_id.dd"
    tail_slice="$SCRATCH/regions/$region_id.tail"
    if ! dd if="$EXTRACTION" of="$dd_slice" bs=1 skip="$start" \
            count="$length" status=none; then
        fail_case "real/region/$region_id" "dd extraction failed"
    fi
    if ! tail -c "+$((start + 1))" "$EXTRACTION" | \
            head -c "$length" >"$tail_slice"; then
        fail_case "real/region/$region_id" "tail/head extraction failed"
    fi
    if ! cmp "$dd_slice" "$tail_slice"; then
        fail_case "real/region/$region_id" \
            "independent byte extraction methods differ"
    fi
    actual_length=$(wc -c <"$dd_slice")
    if [ "$actual_length" -ne "$length" ]; then
        fail_case "real/region/$region_id" \
            "expected $length bytes, got $actual_length"
    fi
    hash_output=$(sha256sum "$dd_slice")
    actual_hash=${hash_output%% *}
    if [ "$actual_hash" != "$expected_hash" ]; then
        fail_case "real/region/$region_id" \
            "registered $expected_hash, got $actual_hash"
    fi
    pass_case "real/region/$region_id"
done <"$region_manifest"

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
