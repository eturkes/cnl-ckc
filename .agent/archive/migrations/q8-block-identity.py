#!/usr/bin/env python3
"""Queue row Q8 (review F-SPEC-11): trace nodes name the clause's own `% S<n>:` block,
so a derived node always joins its own line. Fixture consequences, re-derived through
`ckc v1` (only pins + the inputs named below change):
- join-zero rows (t-c407, t-c409, t-c811/identity-name, t-c819/zero) can no longer be
  derived; their committed traces now differ from the derivation → `trace_check(stale)`;
- t-c813 identity-count rows (target, multiple, pristine-variable) now trace (rc 0);
- ordering + join rows keep their law through a duplicate line in one block (join count 2):
  t-c821 non-demo-before-later-join + join-before-later-non-demo, t-c822/join,
  tests/queries red/trace-digest-join (pinned verdict: multiple committed clause lines);
- new t-c931: an S2 property clause citing only an S1 referent traces as sentence(doc,2)
  and trace-checks green (target = trace-check, `trace` = the trace bytes).
Usage: python3 -P <this> <repo-root> <ckc>; idempotent.
"""
import os, shutil, subprocess, sys, tempfile
from pathlib import Path

root, ckc = Path(sys.argv[1]).resolve(), Path(sys.argv[2]).resolve()
V = root / "tests/v1"
LAY = ".scratch/m5u2/suite/cases"
LINE = "guideline_entity(actual,'$guideline_id'(product,doc,1,ref(1),[]),patient,countable).\n"
DUP = "% S1: The duplicate line joins twice.\n" + LINE + LINE


def rows():
    return [l.rstrip("\n").split("\t") for l in open(V / "cases.tsv") if not l.startswith("#")]


def materialize(case, t):
    src, dst = V / case, Path(t) / LAY / case
    for dp, _, fs in os.walk(src):
        rel = os.path.relpath(dp, src)
        if rel.startswith("expect"):
            continue
        for f in fs:
            out = dst / rel / (f[:-3] if f.endswith(".in") else f)
            out.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(Path(dp) / f, out)


def run(case, mode, args):
    with tempfile.TemporaryDirectory() as t:
        materialize(case, t)
        p = subprocess.run([ckc, "v1", mode, *[f"{LAY}/{case}/{a}" for a in args]], cwd=t, capture_output=True)
        return p.returncode, p.stdout, p.stderr


def header(doc):
    lines = doc.read_text().split("\n")
    keep = []
    for l in lines:
        if l.startswith("% S"):
            break
        keep.append(l)
    return "\n".join(keep) + "\n"


# Inputs: duplicate-line blocks for the ordering + join rows.
for v in ["t-c821/variants/non-demo-before-later-join", "t-c821/variants/join-before-later-non-demo",
          "t-c822/variants/join"]:
    doc = V / v / "doc.pl.in"
    doc.write_text(header(doc) + DUP)
    case, sub = v.split("/", 1)
    rc, out, err = run(case, "trace", [f"{sub}/manifest.tsv", f"{sub}/query.pl", f"{sub}/answers.pl"])
    assert rc == 0, (v, err)
    (V / v / "trace.pl.in").write_bytes(out)

# New anaphora probe t-c931.
c = V / "t-c931"
(c / "expect").mkdir(parents=True, exist_ok=True)
base_doc = header(V / "t-c828/doc.pl.in")
(c / "doc.pl.in").write_text(base_doc + "% S1: A patient waits.\n" + LINE
                             + "% S2: The patient is old.\n"
                             + "guideline_property(actual,'$guideline_id'(product,doc,1,ref(1),[]),old,pos).\n")
(c / "payload.pl.in").write_text("")
(c / "manifest.tsv.in").write_text(f"{LAY}/t-c931/doc.pl\t{LAY}/t-c931/payload.pl\n")
(c / "input.txt.in").write_text("S2 property clause citing only an S1 referent traces as its own block sentence(doc,2)\n")
(c / "query.pl.in").write_text(
    "% q-wh compiled from ACE question by ace_to_pl question mode; do not edit.\n"
    "'$guideline_query'(v1,'q-wh',ace_sha256(" + "a" * 64 + "),ulex(none)).\n"
    "% Q1: Which patient is old?\n"
    "'$guideline_query_projection'(goal(','(guideline_entity(actual,A,patient,countable),"
    "guideline_property(actual,A,old,pos))),answers([answer(A,noun(patient,countable))])).\n")
rc, out, err = run("t-c931", "answer", ["manifest.tsv", "query.pl"])
assert rc == 0, err
(c / "answers.pl.in").write_bytes(out)
rc, out, err = run("t-c931", "trace", ["manifest.tsv", "query.pl", "answers.pl"])
assert rc == 0, err
(c / "trace.pl.in").write_bytes(out)
table = (V / "cases.tsv").read_text()
for line in ["t-c931\ttarget\ttrace-check\t0\t0\tmanifest.tsv query.pl answers.pl trace.pl\n",
             "t-c931\ttrace\ttrace\t0\t0\tmanifest.tsv query.pl answers.pl\n"]:
    if line not in table:
        table += line
(V / "cases.tsv").write_text(table)

# Pins: re-derive every changed row; rc + stderr-line columns follow.
REPIN = {("t-c407", "target"), ("t-c409", "target"), ("t-c811", "identity-name"), ("t-c819", "zero"),
         ("t-c813", "target"), ("t-c813", "multiple"), ("t-c813", "pristine-variable"),
         ("t-c821", "non-demo-before-later-join"), ("t-c821", "join-before-later-non-demo"),
         ("t-c822", "join"), ("t-c931", "target"), ("t-c931", "trace")}
out_rows = []
for r in rows():
    if (r[0], r[1]) in REPIN:
        rc, out, err = run(r[0], r[2], [a for a in r[5].split(" ") if a])
        (V / r[0] / "expect" / f"{r[1]}.stdout").write_bytes(out)
        (V / r[0] / "expect" / f"{r[1]}.stderr").write_bytes(err)
        lines = err.count(b"\n") + (1 if err and not err.endswith(b"\n") else 0)
        r = [r[0], r[1], r[2], str(rc), str(lines) if r[4] != "-" else "-", r[5]]
    out_rows.append("\t".join(r) + "\n")
head = [l for l in open(V / "cases.tsv") if l.startswith("#")]
(V / "cases.tsv").write_text("".join(head + out_rows))

# Queries lane: the join fixture keeps its law through a duplicate S1 line.
q = root / "tests/queries/red/trace-digest-join"
doc = q / "tree/guidelines/fx/pl/doc.pl"
body = ("% S1: A patient waits.\n" + LINE.replace("ref(1),[]),patient", "ref(1),[]),patient") + LINE
        + "guideline_cardinality(actual,'$guideline_id'(product,doc,1,ref(1),[]),na,eq,1).\n"
        + "guideline_event(actual,'$guideline_id'(product,doc,1,ref(2),[]),wait).\n"
        + "guideline_arg(actual,'$guideline_id'(product,doc,1,ref(2),[]),1,'$guideline_id'(product,doc,1,ref(1),[])).\n")
doc.write_text(header(doc) + body)
(q / "expect").write_text("ckc: traces: trace node resolves to multiple committed clause lines: q-digest doc S1\n")
