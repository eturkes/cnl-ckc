#!/usr/bin/env python3
"""nt1 re-anchor of tests/certify/cases.tsv: each mutant keeps the source occurrence it
targeted at the base, after revise.py --semantic added unit literals + converted conditions.

Usage: python3 -I <this> <repo-root> <base-commit>   (run after revise.py; rerun = no change)
A clause is identified by its head (the line before ` :- `): D2 adds whole unit lines and
D3 rewrites bodies, while every head keeps its bytes.
- swap-lines: the operands index the target lines whose bytes the base operands indexed
  (each such line occurs exactly once in the migrated target).
- replace-first: an anchor whose first hit no longer lies in the clause the base hit lay in
  grows leftward inside that clause line until its first hit does; the replacement grows alike.
- QUERY rows: a converted question renames its variables and relinks the recurrence unit
  slot, so the base anchor is gone; the edit keeps its kind on the migrated bytes.
"""
import subprocess
import sys
from pathlib import Path

TABLE = "tests/certify/cases.tsv"
QUERY = {
    "query-v2-annotation-altered": ("guideline_recurrence(actual,G,C,F)", "guideline_recurrence(actual,G,C,day)"),
    "query-v2-annotation-dropped": ("','(guideline_pp(actual,G,at,B),guideline_recurrence(actual,G,C,F))",
                                    "guideline_pp(actual,G,at,B)"),
}


def unescape(s):
    return s.replace("\\\\", "\0").replace("\\n", "\n").replace("\\t", "\t").replace("\0", "\\")


def escape(s):
    return s.replace("\\", "\\\\").replace("\t", "\\t").replace("\n", "\\n")


def hit(text, anchor):
    """(line index, offset in that line) of the anchor's first hit."""
    k = text.index(anchor)
    return text.count("\n", 0, k), k - (text.rfind("\n", 0, k) + 1)


def head(line):
    return line.split(" :- ", 1)[0]


def main(root, base):
    root = Path(root)

    def at_base(path):
        return subprocess.run(["git", "-C", str(root), "show", f"{base}:{path}"],
                              check=True, capture_output=True).stdout.decode()

    old_rows, rows = at_base(TABLE).split("\n"), (root / TABLE).read_text().split("\n")
    assert len(old_rows) == len(rows), "row count changed"
    changed = 0
    for n, (old, row) in enumerate(zip(old_rows, rows)):
        if n == 0 or not row:
            continue
        f, g = old.split("\t"), row.split("\t")
        cid, target, op = f[0], f[2], f[3]
        assert g[:4] == f[:4], (cid, "row identity changed")
        before, text = at_base(target), (root / target).read_text()
        lines = text.split("\n")
        if op == "swap-lines":
            base_lines = before.split("\n")
            new = []
            for i in (int(f[4]), int(f[5])):
                hits = [k for k, line in enumerate(lines) if line == base_lines[i]]
                assert len(hits) == 1, (cid, "base line not unique in target", i, hits)
                new.append(str(hits[0]))
            g[4], g[5] = new
        elif op == "replace-first" and cid in QUERY:
            a1, a2 = QUERY[cid]
            assert unescape(g[4]) not in text or unescape(g[4]) == a1, (cid, "query anchor still present")
            assert a1 in text, (cid, "query anchor")
            g[4], g[5] = escape(a1), escape(a2)
        elif op == "replace-first":
            a1, a2 = unescape(g[4]), unescape(g[5])
            bl, off = hit(before, unescape(f[4]))
            want = [k for k, line in enumerate(lines) if head(line) == head(before.split("\n")[bl])]
            assert len(want) == 1, (cid, "base clause not unique in target", want)
            # A grown anchor keeps the base hit's end; its start is where it begins now.
            first = sum(len(t) + 1 for t in lines[:want[0]])
            end = first + off + len(unescape(f[4]))
            start = end - len(a1)
            assert first <= start and text[start:end] == a1, (cid, "anchor not at its base occurrence")
            grow = 0
            while text.index(text[start - grow:start] + a1) != start - grow:
                grow += 1
                assert grow <= start - first, (cid, "anchor cannot reach its base occurrence")
            g[4], g[5] = escape(text[start - grow:start] + a1), escape(text[start - grow:start] + a2)
        new_row = "\t".join(g)
        if new_row != row:
            rows[n] = new_row
            changed += 1
    (root / TABLE).write_text("\n".join(rows))
    print(f"nt1-certify-repin: {changed} rows re-anchored")


if __name__ == "__main__":
    main(*sys.argv[1:])
