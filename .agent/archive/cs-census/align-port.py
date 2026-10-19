#!/usr/bin/env python3
"""Occurrence-form `ckc align` input for a re-authored ACE document.

usage: align-port.py <old-ace> <new-ace> <old-align.tsv> <src-payload> <extra.tsv> [<drop-groups>]
- src rows + ace rows on unchanged lines (byte-equal line text) carry over: their offsets map to
  the new text and become occurrence counts (1-based, non-overlapping left-to-right, as `ckc align`).
- extra.tsv rows `group<TAB>src|<line><TAB>k<TAB>span`: `src` = the k-th occurrence in the payload,
  <line> = 1-based ACE line, k-th occurrence inside that line.
- drop-groups: comma list of groups whose old rows are dropped (re-authored in extra.tsv).
"""
import sys


def scan(text, span):
    out, i = [], 0
    while True:
        j = text.find(span, i)
        if j < 0:
            return out
        out.append(j)
        i = j + len(span)


def occurrence(text, span, at):
    hits = scan(text, span)
    assert at in hits, (span, at)
    return hits.index(at) + 1


old_ace, new_ace = open(sys.argv[1], encoding='utf-8').read(), open(sys.argv[2], encoding='utf-8').read()
src = open(sys.argv[4], encoding='utf-8').read().rstrip('\n')
drop = set(sys.argv[6].split(',')) if len(sys.argv) > 6 and sys.argv[6] else set()


def starts(text):
    lines, at = [], 0
    for line in text.split('\n'):
        lines.append((at, line))
        at += len(line) + 1
    return lines


old_lines, new_lines = starts(old_ace), starts(new_ace)
rows = []
for row in open(sys.argv[3], encoding='utf-8').read().splitlines():
    group, side, start, span = row.split('\t')
    if group in drop:
        continue
    start = int(start)
    if side == 'src':
        rows.append((group, 'src', occurrence(src, span, start), span))
        continue
    (n, (base, line)), = [(n, l) for n, l in enumerate(old_lines) if l[0] <= start < l[0] + len(l[1]) + 1]
    if n < len(new_lines) and new_lines[n][1] == line:
        rows.append((group, 'ace', occurrence(new_ace, span, new_lines[n][0] + start - base), span))
for row in open(sys.argv[5], encoding='utf-8').read().splitlines():
    if not row or row.startswith('#'):
        continue
    group, where, k, span = row.split('\t')
    if where == 'src':
        rows.append((group, 'src', int(k), span))
        continue
    base, line = new_lines[int(where) - 1]
    at = base + scan(line, span)[int(k) - 1]
    rows.append((group, 'ace', occurrence(new_ace, span, at), span))
for r in rows:
    print('\t'.join(str(x) for x in r))
