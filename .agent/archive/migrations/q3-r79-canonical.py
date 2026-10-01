#!/usr/bin/env python3
"""Queue row Q3 (m5u3 R79): the ten tests/queries fixtures that `ckc check` pinned as
`noncanonical` rejections → canonical v1 documents exercising the same law, so the
fixture lane grades their expect/golden pins again (tests/check/r79-nonv1.tsv → 0 rows).
Each document keeps the case's facts; helper predicates, builtins, zero-bundle files,
quoted bare atoms and `_X` variables become semantic-vocabulary equivalents:
- depth limit (limit-depth, trace-indeterminate-mirror): 115-level property chain;
- inner-inference limit (limit-inner-inference): doubling chain of 2^20 branches;
- yes/no limit before proof (yesno-limit-before-proof): the doubling chain ahead of
  the patient fact;
- empty result (empty-solutions, no-finite-failure, trace-no-mirror): one bundle
  without a patient;
- NAF inference cut (trace-naf-inference-cut): canonical variables in the burn rule;
- bigint (numeric-bigint): canonical bare `ace_sha256` atom;
- trace join (trace-digest-join): the event + arg clauses cite S1 referents from the
  `% S2:` block, so their trace nodes name sentence(doc,1) and join no S1 line.
Then trace-digest-join's committed trace + golden are re-derived through `ckc v1 trace`
(only clause digests move). Usage: python3 -P <this> <repo-root> <ckc>; idempotent.
"""
import subprocess
import sys
import tempfile
from pathlib import Path

root, ckc = Path(sys.argv[1]).resolve(), Path(sys.argv[2]).resolve()
Q = root / "tests/queries"
HEAD = """% doc.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
""" + "".join(
    f":- multifile({p}).\n:- discontiguous({p}).\n"
    for p in [
        "guideline_schema_version/1",
        "guideline_document/3",
        "guideline_entity/4",
        "guideline_cardinality/5",
        "guideline_event/3",
        "guideline_arg/4",
        "guideline_pp/4",
        "guideline_property/4",
        "guideline_operator/3",
    ]
) + """guideline_schema_version(1).
guideline_document(doc,ace_sha256(f2cbd2e45cf351ebdc1c338853a7d245488c07c9153399f9bfccfe476b65d5df),ulex(none)).
"""
R1 = "'$guideline_id'(product,doc,1,ref(1),[])"
R2 = "'$guideline_id'(product,doc,1,ref(2),[])"
S1 = "% S1: Synthetic attributed fixture sentence.\n"
PATIENT = f"guideline_entity(actual,{R1},patient,countable).\n"
WAITS = (
    f"guideline_cardinality(actual,{R1},na,eq,1).\n"
    f"guideline_event(actual,{R2},wait).\n"
    f"guideline_arg(actual,{R2},1,{R1}).\n"
)
NURSE = S1 + f"guideline_entity(actual,{R1},nurse,countable).\n"


def prop(tag):
    return f"guideline_property(actual,{R1},{tag},pos)"


def chain(n):  # patient :- level(1); level(i) :- level(i+1); level(n) fact
    rules = [f"guideline_entity(actual,{R1},patient,countable) :- {prop('level(1)')}.\n"]
    rules += [f"{prop(f'level({i})')} :- {prop(f'level({i + 1})')}.\n" for i in range(1, n)]
    return "".join(rules) + f"{prop(f'level({n})')}.\n"


def doubling(n):  # branch(i) :- branch(i+1), twice per level; branch(n) underivable
    return "".join(f"{prop(f'branch({i})')} :- {prop(f'branch({i + 1})')}.\n" * 2 for i in range(n))


DEPTH = S1 + chain(115) + WAITS
INNER = S1 + f"guideline_entity(actual,{R1},patient,countable) :- {prop('branch(0)')}.\n" + doubling(20) + WAITS
YESNO = S1 + f"guideline_entity(actual,{R1},patient,countable) :- {prop('branch(0)')}.\n" + doubling(20) + PATIENT + WAITS
JOIN = (
    "% S1: A patient waits.\n" + PATIENT + f"guideline_cardinality(actual,{R1},na,eq,1).\n"
    "% S2: The patient waits.\n" + f"guideline_event(actual,{R2},wait).\n" + f"guideline_arg(actual,{R2},1,{R1}).\n"
)


def after_record(old):
    return old.split("guideline_document(", 1)[1].split("\n", 1)[1]


CASES = {
    "red/empty-solutions": NURSE,
    "red/no-finite-failure": NURSE,
    "red/trace-no-mirror": NURSE,
    "red/limit-depth": DEPTH,
    "red/trace-indeterminate-mirror": DEPTH,
    "red/limit-inner-inference": INNER,
    "red/yesno-limit-before-proof": YESNO,
    "red/trace-digest-join": JOIN,
    "red/trace-naf-inference-cut": None,
    "green/numeric-bigint": None,
}
for case, body in CASES.items():
    path = Q / case / "tree/guidelines/fx/pl/doc.pl"
    old = path.read_text()
    if case.endswith("trace-naf-inference-cut"):
        body = after_record(old).replace(",_L,left,", ",A,left,").replace(",_R,right,", ",B,right,")
    elif case.endswith("numeric-bigint"):
        body = after_record(old)
    path.write_text(HEAD + body)

gid = Q / "red/trace-digest-join/tree/guidelines/fx"
with tempfile.TemporaryDirectory() as t:
    manifest = Path(t) / "manifest.tsv"
    pl = gid / "pl/doc.pl"
    manifest.write_text(f"{pl}\t{pl}\n")
    out = subprocess.run(
        [ckc, "v1", "trace", manifest, gid / "queries/pl/q-digest.pl", gid / "queries/answers/q-digest.pl"],
        capture_output=True, check=True,
    ).stdout
for target in [gid / "queries/traces/q-digest.pl", Q / "red/trace-digest-join/traces-golden/q-digest.pl"]:
    target.write_bytes(out)
