#!/usr/bin/env bash
# usage: align-all.sh <base-worktree> <new-guideline-dir> <out-dir> <doc>… — align-port2 rows per doc (old = base HEAD)
set -uo pipefail
B=$1; N=$2; O=$3; shift 3
G=guidelines/cdc-2022-opioid
mkdir -p "$O"
for d in "$@"; do
  doc=cdc2022-opioid-$d
  row=$(awk -F'\t' -v s="ace($doc)" '$5 == s' "$B/$G/coverage.tsv")
  region=$(cut -f1 <<< "$row"); file=$(cut -f2 <<< "$row")
  n=$(grep -n "^\[$region |" "$B/$G/$file" | cut -d: -f1)
  sed -n "$((n + 1))p" "$B/$G/$file" > "$O/$d.src"
  git -C "$B" show "HEAD:$G/ace/$doc.ace" > "$O/$d.old.ace"
  git -C "$B" show "HEAD:$G/align/$doc.tsv" > "$O/$d.old.tsv"
  extra=; [ -f "$O/$d.extra" ] && extra="$O/$d.extra"
  python3 -I "$(dirname "$0")/align-port2.py" "$O/$d.old.ace" "$N/ace/$doc.ace" "$O/$d.old.tsv" "$O/$d.src" $extra > "$O/$d.rows" 2> "$O/$d.dropped"
  echo "$d region=$region rows=$(wc -l < "$O/$d.rows") dropped=$(wc -l < "$O/$d.dropped")"
done
