#!/usr/bin/env python3
"""Occurrence-form `ckc align` rows for an ACE document whose lines changed in place.

usage: align-port2.py <old-ace> <new-ace> <old-align.tsv> <src-payload> [<extra.tsv>]
Each old ace span moves with its line: an unchanged line keeps it; a changed line keeps it when
difflib maps the whole span into one equal block (an inserted `provably ` shifts the spans
after it); a span the edit touched drops. src rows carry over. extra.tsv = `group<TAB>src|<line><TAB>k<TAB>span`.
Prints the rows plus a `# dropped` line per lost span (stderr).
"""
import difflib
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


def starts(text):
    lines, at = [], 0
    for line in text.split('\n'):
        lines.append((at, line))
        at += len(line) + 1
    return lines


old_ace = open(sys.argv[1], encoding='utf-8').read()
new_ace = open(sys.argv[2], encoding='utf-8').read()
src = open(sys.argv[4], encoding='utf-8').read().rstrip('\n')
old_lines, new_lines = starts(old_ace), starts(new_ace)
old_ne = [i for i, (_, l) in enumerate(old_lines) if l]
new_ne = [i for i, (_, l) in enumerate(new_lines) if l]
rows = []
for row in open(sys.argv[3], encoding='utf-8').read().splitlines():
    group, side, start, span = row.split('\t')
    start = int(start)
    if side == 'src':
        rows.append((group, 'src', occurrence(src, span, start), span))
        continue
    (n, (base, line)), = [(n, l) for n, l in enumerate(old_lines) if l[0] <= start < l[0] + len(l[1]) + 1]
    k = old_ne.index(n)  # the k-th sentence keeps its ordinal
    if k >= len(new_ne):
        print(f'# dropped {group} {span} (sentence gone)', file=sys.stderr)
        continue
    nbase, nline = new_lines[new_ne[k]]
    off = start - base
    target = None
    if nline == line:
        target = off
    else:
        for tag, i1, i2, j1, j2 in difflib.SequenceMatcher(None, line, nline, autojunk=False).get_opcodes():
            if tag == 'equal' and i1 <= off and off + len(span) <= i2:
                target = j1 + off - i1
                break
    if target is None or nline[target:target + len(span)] != span:
        print(f'# dropped {group} {span} (line {k + 1} edited)', file=sys.stderr)
        continue
    rows.append((group, 'ace', occurrence(new_ace, span, nbase + target), span))
if len(sys.argv) > 5:
    for row in open(sys.argv[5], encoding='utf-8').read().splitlines():
        if not row or row.startswith('#'):
            continue
        group, where, k, span = row.split('\t')
        if where == 'src':
            rows.append((group, 'src', int(k), span))
            continue
        base, line = new_lines[new_ne[int(where) - 1]]
        rows.append((group, 'ace', occurrence(new_ace, span, base + scan(line, span)[int(k) - 1]), span))
for r in rows:
    print('\t'.join(str(x) for x in r))
