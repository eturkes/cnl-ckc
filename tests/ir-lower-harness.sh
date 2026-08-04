#!/usr/bin/env bash
set -eu

ROOT=$PWD
if ! [ -f src/prolog/ir_tool.pl ] || ! [ -f src/prolog/drs_to_ir.pl ] || \
        ! [ -d tests/fixtures/slice/golden ] || \
        ! [ -d tests/fixtures/slice/ir ] || \
        ! [ -d tests/fixtures/lower/green ] || \
        ! [ -d tests/fixtures/lower/red ]; then
    printf 'FAIL repo-root: run from cnl-ckc repository root\n'
    printf 'SUMMARY: 0 passed, 1 failed\n'
    exit 1
fi

SWIPL=${SWIPL:-swipl}
DOCS="$ROOT/tests/fixtures/slice/docs"
GOLDEN="$ROOT/tests/fixtures/slice/golden"
IR="$ROOT/tests/fixtures/slice/ir"
LOWER_GREEN="$ROOT/tests/fixtures/lower/green"
RED="$ROOT/tests/fixtures/lower/red"
SCRATCH="$ROOT/.scratch/ir-lower-harness.$$"
PASS_COUNT=0
RUN_STATUS=0
EXPECTED_PASS_COUNT=151

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
    fail_case "fixtures/count" "expected 5 front-end goldens, got $#"
fi
set -- "$IR"/*.ir.pl
if [ "$#" -ne 4 ]; then
    fail_case "fixtures/count" "expected 4 IR goldens, got $#"
fi
set -- "$LOWER_GREEN"/*.drs.pl
if [ "$#" -ne 38 ]; then
    fail_case "fixtures/count" "expected 38 lower DRS greens, got $#"
fi
set -- "$LOWER_GREEN"/*.ir.pl
if [ "$#" -ne 38 ]; then
    fail_case "fixtures/count" "expected 38 lower IR goldens, got $#"
fi
set -- "$LOWER_GREEN"/*.program.pl
if [ "$#" -ne 38 ]; then
    fail_case "fixtures/count" "expected 38 lower program goldens, got $#"
fi
set -- "$LOWER_GREEN"/*.result.pl
if [ "$#" -ne 38 ]; then
    fail_case "fixtures/count" "expected 38 lower result goldens, got $#"
fi
set -- "$RED"/*.pl
if [ "$#" -ne 100 ]; then
    fail_case "fixtures/count" "expected 100 red fixtures, got $#"
fi
pass_case "fixtures/count"

rm -rf "$SCRATCH"
mkdir -p "$SCRATCH/green" "$SCRATCH/red" "$SCRATCH/scratch" \
    "$SCRATCH/usage" "$SCRATCH/determinism"
trap 'rm -rf "$SCRATCH"' EXIT
: >"$SCRATCH/dispatched-green-stems"
: >"$SCRATCH/dispatched-red-stems"

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

# Hand-authored probe-pinned M2 inputs carry CLI-generated IR goldens. Each
# committed green is lowered twice so every covered NAF/wh/binary path is byte-stable.
for input in "$LOWER_GREEN"/*.drs.pl; do
    name=${input##*/}
    stem=${name%.drs.pl}
    printf '%s\n' "$stem" >>"$SCRATCH/dispatched-green-stems"
    expected="$LOWER_GREEN/$stem.ir.pl"
    stdout_path="$SCRATCH/green/$stem.lower.stdout"
    stderr_path="$SCRATCH/green/$stem.lower.stderr"
    run_tool "$input" "$stdout_path" "$stderr_path" lower
    if [ "$RUN_STATUS" -ne 0 ]; then
        fail_case "green/lower/$stem/status" "expected 0, got $RUN_STATUS"
    fi
    if [ -s "$stderr_path" ]; then
        fail_case "green/lower/$stem/stderr" "expected zero bytes"
    fi
    if ! cmp "$stdout_path" "$expected"; then
        fail_case "green/lower/$stem/bytes" "lower output differs from golden"
    fi

    rerun_stdout="$SCRATCH/determinism/$stem.lower.stdout"
    rerun_stderr="$SCRATCH/determinism/$stem.lower.stderr"
    run_tool "$input" "$rerun_stdout" "$rerun_stderr" lower
    if [ "$RUN_STATUS" -ne 0 ]; then
        fail_case "green/lower/$stem/determinism-status" \
            "expected 0, got $RUN_STATUS"
    fi
    if [ -s "$rerun_stderr" ]; then
        fail_case "green/lower/$stem/determinism-stderr" \
            "expected zero bytes"
    fi
    if ! cmp "$stdout_path" "$rerun_stdout"; then
        fail_case "green/lower/$stem/determinism-bytes" "fresh runs differ"
    fi

    validate_stdout="$SCRATCH/green/$stem.validate.stdout"
    validate_stderr="$SCRATCH/green/$stem.validate.stderr"
    run_tool "$expected" "$validate_stdout" "$validate_stderr" validate
    if [ "$RUN_STATUS" -ne 0 ]; then
        fail_case "green/lower/$stem/validate-status" \
            "expected 0, got $RUN_STATUS"
    fi
    if [ -s "$validate_stdout" ] || [ -s "$validate_stderr" ]; then
        fail_case "green/lower/$stem/validate-streams" "expected zero bytes"
    fi

    rerun_validate_stdout="$SCRATCH/determinism/$stem.validate.stdout"
    rerun_validate_stderr="$SCRATCH/determinism/$stem.validate.stderr"
    run_tool "$expected" "$rerun_validate_stdout" \
        "$rerun_validate_stderr" validate
    if [ "$RUN_STATUS" -ne 0 ]; then
        fail_case "green/lower/$stem/validate-determinism-status" \
            "expected 0, got $RUN_STATUS"
    fi
    if [ -s "$rerun_validate_stdout" ] || \
            [ -s "$rerun_validate_stderr" ]; then
        fail_case "green/lower/$stem/validate-determinism-streams" \
            "expected zero bytes"
    fi
    pass_case "green/lower/$stem"
done

for input in "$LOWER_GREEN"/*.drs.pl; do
    name=${input##*/}
    printf '%s\n' "${name%.drs.pl}"
done | LC_ALL=C sort >"$SCRATCH/green-fixture-stems"
LC_ALL=C sort "$SCRATCH/dispatched-green-stems" \
    >"$SCRATCH/green-dispatched-stems"
if ! cmp "$SCRATCH/green-fixture-stems" \
        "$SCRATCH/green-dispatched-stems"; then
    fail_case "inventory/green-dispatch" \
        "fixture and dispatched stems differ"
fi
pass_case "inventory/green-dispatch"

run_committed_red() {
    local name expected_class expected_line stdout_path stderr_path
    local first_status rerun_stdout rerun_stderr
    name=$1
    expected_class=$2
    expected_line=${3-}
    printf '%s\n' "$name" >>"$SCRATCH/dispatched-red-stems"
    stdout_path="$SCRATCH/red/$name.stdout"
    stderr_path="$SCRATCH/red/$name.stderr"
    run_tool "$RED/$name.pl" "$stdout_path" "$stderr_path" lower
    first_status=$RUN_STATUS
    check_rejection "red/$name" 1 lower "$expected_class" \
        "$stdout_path" "$stderr_path" "$expected_line"

    rerun_stdout="$SCRATCH/determinism/$name.red.stdout"
    rerun_stderr="$SCRATCH/determinism/$name.red.stderr"
    run_tool "$RED/$name.pl" "$rerun_stdout" "$rerun_stderr" lower
    if [ "$RUN_STATUS" -ne "$first_status" ]; then
        fail_case "red/$name/determinism-status" \
            "fresh runs differ: $first_status and $RUN_STATUS"
    fi
    if ! cmp "$stdout_path" "$rerun_stdout" || \
            ! cmp "$stderr_path" "$rerun_stderr"; then
        fail_case "red/$name/determinism-bytes" "fresh runs differ"
    fi
}

run_committed_red disjunction-root-outside alternative_set \
    'ir_tool_error(lower,alternative_set,root_condition(1,branch(left,profile))).'
run_committed_red disjunction-consequent-outside alternative_set \
    'ir_tool_error(lower,alternative_set,rule(1,consequent_branch(left,profile))).'
run_committed_red disjunction-question disjunction \
    'ir_tool_error(lower,disjunction,question).'
run_committed_red disjunction-v-under-naf disjunction \
    'ir_tool_error(lower,disjunction,rule(1,v_under_naf)).'
run_committed_red disjunction-naf-under-v disjunction \
    'ir_tool_error(lower,disjunction,rule(1,naf_inside_disjunct(left))).'
run_committed_red disjunction-malformed disjunction \
    'ir_tool_error(lower,disjunction,rule(1,disjunct_shape(left))).'
run_committed_red disjunction-cap-exceeded disjunction \
    'ir_tool_error(lower,disjunction,rule(4,antecedent_branch_cap_exceeded(64))).'
run_committed_red alternative-set-root-argument alternative_set \
    'ir_tool_error(lower,alternative_set,root_condition(1,branch(left,argument(2)))).'
run_committed_red alternative-set-duplicate-member alternative_set \
    'ir_tool_error(lower,alternative_set,-(root_condition(1),duplicate_member)).'
run_committed_red alternative-set-consequent-domain alternative_set \
    'ir_tool_error(lower,alternative_set,rule(1,consequent_outer_domain)).'
run_committed_red alternative-set-antecedent-naf alternative_set \
    'ir_tool_error(lower,alternative_set,rule(1,alternative_body_naf)).'
run_committed_red property-degree-comp unsupported \
    'ir_tool_error(lower,unsupported,-(root_condition(1),property_degree(comp))).'
run_committed_red property-degree-sup unsupported \
    'ir_tool_error(lower,unsupported,-(root_condition(1),property_degree(sup))).'
run_committed_red property-arity-six unsupported \
    'ir_tool_error(lower,unsupported,-(root_condition(1),property_arity(6))).'
run_committed_red property-naf negation \
    'ir_tool_error(lower,negation,rule(1,antecedent_condition(2,profile))).'
run_committed_red property-attributive-consequent unsupported \
    'ir_tool_error(lower,unsupported,rule(1,consequent_count(3))).'
run_committed_red property-wh-query wh_query \
    'ir_tool_error(lower,wh_query,question).'
run_committed_red property-comparative-unbound referent \
    'ir_tool_error(lower,referent,root_condition(1,property_subject(unbound))).'
run_committed_red property-comparative-ambiguous referent \
    'ir_tool_error(lower,referent,root_condition(1,property_subject(ambiguous))).'
run_committed_red property-antecedent-ambiguous referent \
    'ir_tool_error(lower,referent,rule(1,antecedent_condition(2,property_subject(ambiguous)))).'
run_committed_red property-carrier-head referent \
    'ir_tool_error(lower,referent,rule(1,unbound_head_referent)).'
run_committed_red property-root-event-in-use referent \
    'ir_tool_error(lower,referent,-(root_condition(1),event_in_use)).'
run_committed_red property-rule-body-event-in-use referent \
    'ir_tool_error(lower,referent,-(rule(1,antecedent),event_in_use)).'
run_committed_red property-rule-head-event-in-use referent \
    'ir_tool_error(lower,referent,-(rule(1,consequent),event_in_use)).'
run_committed_red property-question-event-in-use referent \
    'ir_tool_error(lower,referent,-(question,event_in_use)).'
run_committed_red separator-naf-verb unsupported \
    'ir_tool_error(lower,unsupported,-(rule_naf,lemma_space(verb))).'
run_committed_red separator-naf-object-class unsupported \
    'ir_tool_error(lower,unsupported,-(rule_naf,lemma_space(object_class))).'
run_committed_red separator-question-adjective unsupported \
    'ir_tool_error(lower,unsupported,-(question,lemma_space(adjective))).'
run_committed_red separator-rule-body-object-class unsupported \
    'ir_tool_error(lower,unsupported,-(rule(1,antecedent),lemma_space(object_class))).'
run_committed_red property-question-unpaired unsupported \
    'ir_tool_error(lower,unsupported,question(copula_profile)).'
run_committed_red property-rule-head-unpaired unsupported \
    'ir_tool_error(lower,unsupported,rule(1,consequent_copula)).'
run_committed_red property-separator-adjective unsupported \
    'ir_tool_error(lower,unsupported,-(root_condition(1),lemma_space(adjective))).'
run_committed_red separator-verb unsupported \
    'ir_tool_error(lower,unsupported,-(root_condition(1),lemma_space(verb))).'
run_committed_red separator-object-class unsupported \
    'ir_tool_error(lower,unsupported,-(root_condition(1),lemma_space(object_class))).'
run_committed_red separator-relation unsupported \
    'ir_tool_error(lower,unsupported,-(root_condition(1),lemma_space(relation))).'
run_committed_red wh-query wh_query
run_committed_red p07-which-query wh_query
run_committed_red p08-wh-copula wh_query
run_committed_red p02-antecedent-classical-negation negation
run_committed_red p03-root-classical-negation negation
run_committed_red p04-negated-consequent negation
run_committed_red p06-negated-antecedent-copula negation
run_committed_red p09-negated-question negation
run_committed_red p14-root-naf negation
run_committed_red p16-nested-negation negation
run_committed_red naf-positive-interleave negation
run_committed_red naf-inner-unrecognized negation
run_committed_red zero-question question_count
run_committed_red two-questions question_count
run_committed_red non-final-question question_count
run_committed_red unpaired-object copula
run_committed_red relation-unpaired-object-precedence copula \
    'ir_tool_error(lower,copula,root_condition(1,unpaired_object)).'
run_committed_red unpaired-be copula
run_committed_red be-non-named copula
run_committed_red object-wrong-fields copula
run_committed_red object-field-alias-event copula
run_committed_red reversed-copula-orientation copula
run_committed_red rule-object-field-alias-event unsupported
run_committed_red event-reuse referent
run_committed_red role-reuse referent
run_committed_red cross-drs-redeclaration referent
run_committed_red unbound-head-variable referent
run_committed_red unconsumed-domain referent
run_committed_red undeclared-referent referent
run_committed_red relation-unbound-referent referent \
    'ir_tool_error(lower,referent,root_condition(1,relation_argument(1,unbound))).'
run_committed_red relation-ambiguous-referent referent \
    'ir_tool_error(lower,referent,root_condition(1,relation_argument(1,ambiguous))).'
run_committed_red transitive-event-in-use referent \
    'ir_tool_error(lower,referent,-(root_condition(1),event_in_use)).'
run_committed_red transitive-event-in-question referent \
    'ir_tool_error(lower,referent,-(root_condition(1),event_in_use)).'
run_committed_red transitive-rule-body-event-in-use referent \
    'ir_tool_error(lower,referent,-(rule(1,antecedent),event_in_use)).'
run_committed_red transitive-rule-head-event-in-use referent \
    'ir_tool_error(lower,referent,-(rule(1,consequent),event_in_use)).'
run_committed_red transitive-question-event-in-use referent \
    'ir_tool_error(lower,referent,-(question,event_in_use)).'
run_committed_red transitive-cycle-self-loop cycle \
    'ir_tool_error(lower,cycle,term(3,body_literal(1,signed_dependency(positive,pred(like,2),pred(like,2))))).'
run_committed_red relation-cycle-self-loop cycle \
    'ir_tool_error(lower,cycle,term(3,body_literal(1,signed_dependency(positive,pred(of,2),pred(of,2))))).'
run_committed_red transitive-nonground-fact unsupported \
    'ir_tool_error(lower,unsupported,-(root_condition(1),nonground_argument(1))).'
run_committed_red body-only-variable-fact unsupported
run_committed_red mixed-sentence-anchors unsupported
run_committed_red unknown-condition unsupported
run_committed_red relation-non-of unsupported \
    'ir_tool_error(lower,unsupported,-(root_condition(1),relation_name)).'
run_committed_red nested-implication unsupported
run_committed_red m6u3-real-quantity quantity \
    'ir_tool_error(lower,quantity,real_not_integer).'
run_committed_red m6u3-scalar-cross-unit quantity \
    "ir_tool_error(lower,quantity,scalar_cross_unit('morphine-milligram-equivalent',day))."
run_committed_red m6u3-false-friend-window temporal \
    'ir_tool_error(lower,temporal,false_friend_within_to_range).'
run_committed_red m6u3-malformed-bound-wrapper quantity \
    'ir_tool_error(lower,quantity,-(root,condition_wrapper)).'
run_committed_red m6u3-unknown-bound quantity \
    'ir_tool_error(lower,quantity,object_bound_relation(gte)).'
run_committed_red m6u3-scalar-no-be quantity \
    'ir_tool_error(lower,quantity,-(root_condition(1),scalar_comparison_be(missing))).'
run_committed_red m6u3-scalar-named-be quantity \
    'ir_tool_error(lower,quantity,-(root_condition(1),scalar_comparison_actual_operand)).'
run_committed_red m6u3-scalar-question-actual quantity \
    'ir_tool_error(lower,quantity,-(question,scalar_comparison_actual_operand)).'
run_committed_red m6u3-window-mixed-anchor temporal \
    'ir_tool_error(lower,temporal,window_condition(1,mixed_sentence_anchors)).'
run_committed_red m6u3-scalar-before-fact unsupported \
    'ir_tool_error(lower,unsupported,root_section_order).'
run_committed_red m6u3-quantity-object-fields-owner copula \
    'ir_tool_error(lower,copula,root_condition(1,object_fields)).'
run_committed_red m6u3-window-six-of-seven-owner unsupported \
    'ir_tool_error(lower,unsupported,-(root_condition(1),constructor(/(has_part,2)))).'
run_committed_red m6u3-temporal-reused-event temporal \
    'ir_tool_error(lower,temporal,-(root_condition(1),event_in_use)).'
run_committed_red m6u3-temporal-anchor-variable temporal \
    'ir_tool_error(lower,temporal,root_condition(1,modifier_profile)).'
run_committed_red m6u3-temporal-anchor-ambiguous temporal \
    'ir_tool_error(lower,temporal,root_condition(1,modifier_profile)).'
run_committed_red m6u3-temporal-relation-unknown temporal \
    'ir_tool_error(lower,temporal,root_condition(1,modifier_profile)).'
run_committed_red m6u3-count-one-root copula \
    'ir_tool_error(lower,copula,root_condition(1,unpaired_object)).'
run_committed_red envelope-wrong-header envelope
run_committed_red malformed-document envelope
run_committed_red envelope-missing-drs envelope
run_committed_red envelope-trailing-term envelope

for input in "$RED"/*.pl; do
    name=${input##*/}
    printf '%s\n' "${name%.pl}"
done | LC_ALL=C sort >"$SCRATCH/red-fixture-stems"
LC_ALL=C sort "$SCRATCH/dispatched-red-stems" \
    >"$SCRATCH/red-dispatched-stems"
if ! cmp "$SCRATCH/red-fixture-stems" \
        "$SCRATCH/red-dispatched-stems"; then
    fail_case "inventory/red-dispatch" \
        "fixture and dispatched stems differ"
fi
pass_case "inventory/red-dispatch"

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
