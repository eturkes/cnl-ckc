#!/usr/bin/env python3
"""Fork shrink (m5u7 P3, R95/R100): drop the check, aggregate-check,
recursion-check, answer and trace modes of ace_to_pl.pl and every predicate
only they reach. Reachability = prolog_xref call edges from main/0 + the
user:message_hook/3 clause, recomputed after the dispatch clauses leave.
A removed term takes the gap before it (its comments) along; directives stay
unless they name a removed predicate. Then the header + three dead-mode
remnants are rewritten (EDITS below).
Record: input = vendor/ape/prolog/ace_to_pl.pl at the U5a-1 commit (3598
lines); output = the U5a-2 file (2259 lines): 7 dispatch clauses + 90
predicates removed; input sha256 7dc668dd26861f839858312fad3c4bc6d28e3d5a08ab158fa2f3e750fa1d7a19, output sha256 6844f559e5d5f797bcf8748710d8dd37d7857558b49abff0ae2309d94d2741bd; replay
= byte-identical. Helpers = fork-shrink-{terms,xref}.pl.txt beside this file.
Usage: python3 -P <this> <in.pl> <out.pl> <swipl>"""
import re
import subprocess
import sys
from pathlib import Path

src, dst, swipl = sys.argv[1:4]
here = Path(__file__).resolve().parent
MODES = ("check", "'aggregate-check'", "'recursion-check'", "answer", "trace")


def terms(path):
    out = subprocess.run([swipl, "-q", "-f", "none", "-F", "none", "-s", str(here / "fork-shrink-terms.pl.txt"),
                          "-g", "probe_main", "-t", "halt", "--", path], capture_output=True, text=True, check=True)
    return [(k, pi, int(a), int(b)) for k, pi, a, b in (l.split("\t") for l in out.stdout.splitlines())]


def edges(path):
    out = subprocess.run([swipl, "-q", "-f", "none", "-F", "none", "-s", str(here / "fork-shrink-xref.pl.txt"),
                          "-g", "probe_main", "-t", "halt", "--", path], capture_output=True, text=True, check=True)
    calls, defs = {}, set()
    for line in out.stdout.splitlines():
        f = line.split("\t")
        if f[0] == "def":
            defs.add(f"{f[1]}/{f[2]}")
        elif f[0] == "call":
            calls.setdefault(f[1], set()).add(f[2])
    return defs, calls


def spans(text, items):
    """Each term → (gap_start, term_end_after_dot_line); gap = text after the previous term's line."""
    out, prev = [], 0
    for k, pi, a, b in items:
        dot = text.index(".", b)
        end = text.find("\n", dot)
        end = len(text) if end < 0 else end + 1
        out.append((k, pi, prev, a, end))
        prev = end
    return out


def cut(text, items, drop):
    keep, pos = [], 0
    for idx, (k, pi, gap, a, end) in enumerate(spans(text, items)):
        if idx in drop:
            keep.append(text[pos:gap])
            pos = end
    keep.append(text[pos:])
    return re.sub(r"\n{3,}", "\n\n", "".join(keep))


text = Path(src).read_text(encoding="utf-8")
items = terms(src)
disp = {i for i, (k, pi, a, b) in enumerate(items)
        if pi == "dispatch/4" and re.match(r"dispatch\(\[(%s)[,|\]]" % "|".join(re.escape(m) for m in MODES), text[a:b])}
assert len(disp) == 7, len(disp)
stage = text
stage = cut(stage, items, disp)
Path(dst).write_text(stage, encoding="utf-8")
defs, calls = edges(dst)
live, todo = set(), ["main/0", "run/3"]
while todo:
    p = todo.pop()
    if p in live:
        continue
    live.add(p)
    todo += [c for c in calls.get(p, ()) if c in defs]
# hook clause (user:message_hook/3) + its callees
todo = [c for c in calls.get("message_hook/3", ()) if c in defs]
while todo:
    p = todo.pop()
    if p not in live:
        live.add(p); todo += [c for c in calls.get(p, ()) if c in defs]
dead = sorted(defs - live)
items2 = terms(dst)
drop = set()
for i, (k, pi, a, b) in enumerate(items2):
    if k == "clause" and pi in dead:
        drop.add(i)
    elif k == "directive" and any(re.search(r"\b%s\(" % re.escape(d.split("/")[0]), stage[a:b]) for d in dead):
        drop.add(i)
final = cut(stage, items2, drop)
i = final.index("%   check <file.pl>                   load compiled file into user")
j = final.index("%\n% Compile contract:")
final = final[:i] + ("% The composition consumption modes (check, aggregate-check, recursion-check,\n"
                     "% answer, trace) run in the verified Rust kernel as `ckc v1 <mode>`.\n") + final[j:]
EDITS = [
    ("%       2=usage|ape_load|ulex_load|check_load|uncaught.\n", "%       2=usage|ape_load|ulex_load|uncaught.\n"),
    ('file and is documented in README.md "Compiled Prolog schema (v1)".',
     'file and is documented in docs/REFERENCE.md "Compiled Prolog schema (v1)".'),
    ("/* Bounded obligation call, shared by per-document and aggregate\n   replay: the depth-limited search",
     "/* Bounded obligation call of the per-document replay: the depth-limited search"),
    ('fallback_error_line(check_load, "ace_to_pl_error(check_load,unserializable).\\n") :- !.\n', ""),
]
for old, new in EDITS:
    assert final.count(old) == 1, old[:50]
    final = final.replace(old, new)
Path(dst).write_text(final, encoding="utf-8")
print(f"shrink: dispatch clauses removed {len(disp)}; predicates removed {len(dead)}; "
      f"lines {text.count(chr(10))} -> {final.count(chr(10))}")
for d in dead:
    print("  dead", d)
