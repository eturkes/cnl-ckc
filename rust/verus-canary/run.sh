#!/usr/bin/env bash
set -uo pipefail
export LC_ALL=C

usage() {
    printf 'usage: %s <verus-path>\n' "$0" >&2
    printf 'CKC_VERUS_CANARY_TIMEOUT sets the per-case timeout (default: 60s).\n' >&2
    exit 2
}

[[ $# == 1 ]] || usage
verus=$1
[[ -f "$verus" && -x "$verus" ]] || {
    printf 'canary: error: verifier is not executable: %s\n' "$verus" >&2
    exit 2
}
verus=$(readlink -f -- "$verus") || exit 2
case_timeout=${CKC_VERUS_CANARY_TIMEOUT:-60s}
[[ "$case_timeout" =~ ^[0-9]+([.][0-9]+)?[smhd]?$ && "$case_timeout" =~ [1-9] ]] || usage
command -v timeout >/dev/null || exit 2
dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd) || exit 2

names=() expects=() diagnostics=()
declare -A seen=()
manifest_error() {
    printf 'canary: error: invalid cases.tsv row %s\n' "$1" >&2
    exit 2
}
{
    IFS= read -r header || manifest_error 1
    [[ "$header" == $'case\texpect\tdiagnostic\tupstream_pr' ]] || manifest_error 1
    line=1
    while IFS= read -r row || [[ -n "$row" ]]; do
        ((line += 1))
        # Count separators before read: Bash collapses adjacent tab delimiters.
        tabs=${row//[^$'\t']/}
        [[ ${#tabs} == 3 && "$row" != *$'\t\t'* && "$row" != $'\t'* && "$row" != *$'\t' ]] || manifest_error "$line"
        IFS=$'\t' read -r name expect diagnostic upstream <<< "$row"
        [[ "$name" =~ ^[a-z][a-z0-9_]*$ && -z ${seen[$name]+set} ]] || manifest_error "$line"
        [[ "$upstream" =~ ^([0-9]+|-)$ ]] || manifest_error "$line"
        case "$expect" in
            verify | partial | run) [[ "$diagnostic" == - ]] || manifest_error "$line" ;;
            reject) [[ "$diagnostic" != - && "$diagnostic" != *$'\r'* ]] || manifest_error "$line" ;;
            *) manifest_error "$line" ;;
        esac
        [[ -f "$dir/$name.rs" && -r "$dir/$name.rs" ]] || manifest_error "$line"
        seen[$name]=1
        names+=("$name") expects+=("$expect") diagnostics+=("$diagnostic")
    done
} < "$dir/cases.tsv"
[[ ${#names[@]} -gt 0 ]] || manifest_error 2

logs=$(mktemp -d "${TMPDIR:-/tmp}/ckc-verus-canary.XXXXXXXX") || exit 2
trap 'rm -rf -- "$logs"' EXIT
trap 'exit 2' HUP INT TERM
mismatched=0
launch_failed=0
for i in "${!names[@]}"; do
    name=${names[$i]}
    expect=${expects[$i]}
    diagnostic=${diagnostics[$i]}
    log=$logs/$name.log
    # The upstream cases use uninterpreted functions; --no-cheating rejects those declarations,
    # so verifier flags come per case from a first line `// canary-args: <flag>...`.
    read -ra args <<< "$(sed -n '1s#^// canary-args: ##p' "$dir/$name.rs")"
    # A fix that only changes the erased program (the binary) shows when the binary runs.
    [[ "$expect" == run ]] && args+=(--compile -o "$logs/$name.bin")
    timeout --kill-after=5s "$case_timeout" "$verus" --color never "${args[@]}" "$dir/$name.rs" > "$log" 2>&1
    rc=$?
    got=error
    if [[ $rc == 0 ]] && awk '
        /^verification results:: [1-9][0-9]* verified, 0 errors$/ { summary++ }
        /^error(\[|:)/ { bad++ }
        END { exit !(summary == 1 && bad == 0) }
    ' "$log"; then
        got=verified
    elif [[ $rc == 0 ]] && awk '
        /^verification results:: [1-9][0-9]* verified, 0 errors \(partial verification with `--verify-\*`\)$/ { summary++ }
        /^error(\[|:)/ { bad++ }
        END { exit !(summary == 1 && bad == 0) }
    ' "$log"; then
        got=partial
    elif [[ $rc == 1 ]] && awk -v diagnostic="$diagnostic" '
        /^error(\[|:)/ {
            message = $0
            sub(/^error(\[E[0-9]+\])?: /, "", message)
            if (diagnostic != "-" && message != $0 && index(message, diagnostic) == 1) expected++
            else if (diagnostic == "-" && $0 ~ /^error: (`assert_by_compute_only` failed to simplify down to true|assertion failed)/) expected++
            else if ($0 !~ /^error: aborting due to [0-9]+ previous errors?([; ].*)?$/) unexpected++
        }
        /panicked at|internal compiler error|fatal runtime error/ { unexpected++ }
        END { exit !(expected > 0 && unexpected == 0) }
    ' "$log"; then
        got=rejected-with-diagnostic
    fi
    case $rc in
        125|126|127) launch_failed=1 ;;
    esac
    if [[ "$expect:$got" == run:verified ]]; then
        timeout --kill-after=5s "$case_timeout" "$logs/$name.bin" >> "$log" 2>&1
        run_rc=$?
        if [[ $run_rc == 0 ]]; then got=ran; else got="run-failed($run_rc)"; fi
    fi
    status=MISMATCH
    if [[ "$expect:$got" == verify:verified || "$expect:$got" == partial:partial || "$expect:$got" == reject:rejected-with-diagnostic || "$expect:$got" == run:ran ]]; then
        status=ok
    else
        ((mismatched += 1))
        printf 'canary: %s: verifier rc=%s\n' "$name" "$rc" >&2
        cat -- "$log" >&2
    fi
    printf '%s\t%s\t%s\t%s\n' "$name" "$expect" "$got" "$status"
done
if [[ $mismatched == 0 ]]; then
    printf 'canary: ok %s cases\n' "${#names[@]}"
    exit 0
fi
printf 'canary: %s of %s mismatched\n' "$mismatched" "${#names[@]}"
[[ $launch_failed == 0 ]] || exit 2
exit 1
