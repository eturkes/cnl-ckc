#!/usr/bin/env python3
"""C1 consolidation (contract .agent/contracts/c1.md D12): every pre-C1 artifact of the
corpus + fixture trees → the one schema (identifier 1, `vocabulary.tsv`, record
`vocabulary(none | sha256(T))`, question record `'$guideline_query'(v1, …)` /5).

Usage: python3 -I <this> <repo-root> <ckc-binary> [--dry]   (toolchain PATH: swipl 9.2.9;
--dry = phase A only, nothing written)

Phase A, pure transforms over the tracked scope (guidelines/, tests/, the Kani harness):
- retired items (c1-dispositions.tsv, user-approved) → deleted;
- table renames: temporal.tsv | *.temporal.tsv | *.temporal.tsv.in → the D4 name; the
  pre-C1 two-line header → the one D4 line;
- canonical compiled documents (v1 | v2 | v3) → D2 header (15 declaration pairs,
  identifier 1, record /4 ending vocabulary(none) | vocabulary(sha256(T))); questions →
  D3 record; bundles + projections byte-identical; `.tpl.in` templates transformed in
  their literal header lines;
- re-targeted items: the hand edits of RETARGET (defect kept inside a D2/D3 envelope);
- digests of changed files rebound to a fixed point (atom quoting follows the digest);
  `@@sha256` template pins = their expansion; tests/ui fixture commit ids recomputed.
Phase B, regeneration through the C1 binary + compiler: tests/v1 expectations, red +
green pins, check-mutant pins, tests/ui audit manifests + goldens, the corpus review
manifest, certify-one fixtures (their generator).
A second run on its own output changes nothing.
"""
import hashlib
import os
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

root = Path(sys.argv[1]).resolve()
ckc = Path(sys.argv[2]).resolve()

# ---------- formats ----------

KINDS2 = b"unit (value: second|minute|hour|day|week|month|year) | relation (value: duration|within|after|before) | spacing (value: frame noun lemma)"
KINDS3 = KINDS2 + b" | window (value: frame noun lemma) | frequency (value: period) | approximation (value: about) | range (value: minimum)"
HEADER = b"# format: kind\tlemma\tvalue\n"
OLD_HEADERS = [HEADER + b"# kind: " + KINDS3 + b"\n", HEADER + b"# kind: " + KINDS2 + b"\n"]

BASE = [("guideline_schema_version", 1), ("guideline_document", 3), ("guideline_entity", 4),
        ("guideline_cardinality", 5), ("guideline_event", 3), ("guideline_arg", 4),
        ("guideline_pp", 4), ("guideline_property", 4), ("guideline_operator", 3)]
ANN = [("guideline_interval", 6), ("guideline_recurrence", 4)]
Q12 = [("guideline_frequency", 5), ("guideline_order", 4), ("guideline_recurrence_window", 7),
       ("guideline_range", 3)]


def indicators(v):
    base = list(BASE)
    if v >= 2:
        base[1] = ("guideline_document", 4)
    return base + (ANN if v >= 2 else []) + (Q12 if v >= 3 else [])


def block(inds):
    return b"".join(b":- multifile(%s/%d).\n:- discontiguous(%s/%d).\n" % (n.encode(), a, n.encode(), a)
                    for n, a in inds)


OLD_BLOCK = {v: block(indicators(v)) for v in (3, 2, 1)}
NEW_BLOCK = block(indicators(3))
DOC_LINE1 = re.compile(rb"\A% [^\n]*\.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit\.\n")
QUERY_LINE1 = re.compile(rb"\A% [^\n]* compiled from ACE question by ace_to_pl question mode; do not edit\.\n")
QREC = re.compile(rb"'\$guideline_query'\((v[123]),")


def record_c1(record, v):
    """A pre-C1 record line (`….`) → its D2/D3 form; None = not the canonical shape."""
    if v == 1:
        if b"temporal(" in record or b"vocabulary(" in record or not record.endswith(b"))."):
            return None
        return record[:-2] + b",vocabulary(none))."
    i = record.rfind(b",temporal(sha256(")
    if i < 0 or not record.endswith(b")))."):
        return None
    return record[:i] + b",vocabulary(" + record[i + len(b",temporal("):]


def doc_c1(data):
    m = DOC_LINE1.match(data)
    if not m:
        return None
    head, rest = data[:m.end()], data[m.end():]
    for v in (3, 2, 1):
        if rest.startswith(OLD_BLOCK[v]):
            rest = rest[len(OLD_BLOCK[v]):]
            break
    else:
        return None
    sv = b"guideline_schema_version(%d).\n" % v
    if not rest.startswith(sv):
        return None
    rest = rest[len(sv):]
    nl = rest.find(b"\n")
    if nl < 0 or not rest.startswith(b"guideline_document("):
        return None
    record = record_c1(rest[:nl], v)
    if record is None:
        return None
    return head + NEW_BLOCK + b"guideline_schema_version(1).\n" + record + rest[nl:]


def query_c1(data):
    m = QUERY_LINE1.match(data)
    if not m:
        return None
    head, rest = data[:m.end()], data[m.end():]
    nl = rest.find(b"\n")
    r = QREC.match(rest)
    if nl < 0 or not r:
        return None
    v = int(r.group(1)[1:])
    record = record_c1(rest[:nl], v)
    if record is None:
        return None
    return head + b"'$guideline_query'(v1," + record[r.end():] + rest[nl:]


def is_c1(data):
    """Already in the consolidated form (idempotence)."""
    m = DOC_LINE1.match(data)
    if m:
        return data[m.end():].startswith(NEW_BLOCK + b"guideline_schema_version(1).\n")
    m = QUERY_LINE1.match(data)
    return bool(m) and data[m.end():].startswith(b"'$guideline_query'(v1,") and \
        b",vocabulary(" in data[m.end():].split(b"\n", 1)[0]


def compiled_c1(data):
    """A compiled document | question (plain bytes) → C1 bytes; None = not canonical pre-C1."""
    if is_c1(data):
        return data
    return doc_c1(data) if DOC_LINE1.match(data) else query_c1(data) if QUERY_LINE1.match(data) else None


def line1_defect(data):
    """A fixture whose line-1 defect (a BOM, an extra first line) precedes a canonical
    pre-C1 document: the offset where that document starts, else None."""
    for k in (3, data.find(b"\n") + 1):
        if 0 < k < len(data) and (k != 3 or data.startswith(b"\xef\xbb\xbf")) \
                and DOC_LINE1.match(data[k:]) and doc_c1(data[k:]) is not None:
            return k
    return None


# ---------- helpers ----------

def sha(data):
    return hashlib.sha256(data).hexdigest().encode()


def atom(hexdigest):
    return b"'" + hexdigest + b"'" if hexdigest[:1].isdigit() else hexdigest


HEX = re.compile(rb"(?<![0-9a-f])[0-9a-f]{64}(?![0-9a-f])")
TERM = re.compile(rb"(?<=sha256\()('?)([0-9a-f]{64})\1(?=\))")
SHA1 = re.compile(rb"(?<![0-9a-f])[0-9a-f]{40}(?![0-9a-f])")


def tree(n, first, leaf):
    layer = [first] + [leaf] * (n - 1)
    while len(layer) > 1:
        layer = [b"','(" + layer[i] + b"," + layer[i + 1] + b")" if i + 1 < len(layer) else layer[i]
                 for i in range(0, len(layer), 2)]
    return layer[0]


def expand(template):
    """tests/v1 `.tpl.in` expansion (v1_suite.rs `expand`; the pin line excluded)."""
    body = []
    for line in template.split(b"\n")[1:-1]:
        if line.startswith(b"@@run "):
            _, lo, hi, text = line.split(b" ", 3)
            body += [text.replace(b"@i@", str(i).encode()).replace(b"@j@", str(i + 1).encode())
                     for i in range(int(lo), int(hi) + 1)]
        elif line.startswith(b"@@tree "):
            n, first, leaf, prefix, suffix = line[len(b"@@tree "):].split(b"\t")
            body.append(prefix + tree(int(n), first, leaf) + suffix)
        else:
            body.append(line)
    return b"\n".join(body) + b"\n"


GIT_ENV = dict(GIT_CONFIG_GLOBAL="/dev/null", GIT_CONFIG_SYSTEM="/dev/null", GIT_DEFAULT_HASH="sha1",
               GIT_AUTHOR_NAME="fixture", GIT_AUTHOR_EMAIL="fixture@localhost", GIT_COMMITTER_NAME="fixture",
               GIT_COMMITTER_EMAIL="fixture@localhost", GIT_AUTHOR_DATE="2026-01-01T00:00:00+00:00",
               GIT_COMMITTER_DATE="2026-01-01T00:00:00+00:00")


def fixture_commit(files):
    """HEAD of `git init; add -A; commit` over {relpath: bytes} (ui_fixtures.rs `Fixture::new`)."""
    with tempfile.TemporaryDirectory() as tmp:
        for rel, blob in files.items():
            (Path(tmp) / rel).parent.mkdir(parents=True, exist_ok=True)
            (Path(tmp) / rel).write_bytes(blob)
        env = dict(os.environ, **GIT_ENV)
        for argv in (["init", "-q", "-b", "main"], ["add", "-A"], ["commit", "-q", "-m", "fixture corpus"]):
            subprocess.run(["git", *argv], cwd=tmp, env=env, check=True)
        return subprocess.run(["git", "rev-parse", "HEAD"], cwd=tmp, env=env, capture_output=True,
                              check=True).stdout.strip()


def tracked(*prefixes):
    out = subprocess.run(["git", "ls-files", "-z", "--", *prefixes], cwd=root, capture_output=True,
                         check=True).stdout.decode()
    return [p for p in out.split("\0") if p and (root / p).is_file()]


# ---------- dispositions ----------

DISPOSITIONS = root / ".agent/archive/migrations/c1-dispositions.tsv"
rows = [r.split("\t") for r in DISPOSITIONS.read_text().splitlines() if r and not r.startswith("#")]
retired = {(r[0], r[1]) for r in rows if r[6] == "retired"}


def dispose(family, item):
    return (family, item) in retired


# ---------- re-targets: hand edits keeping each defect inside a D2/D3 envelope ----------
# path → function(bytes) → bytes, applied in place of the mechanical transform (the
# c1-dispositions.tsv row of each item names the defect it keeps).
FULL = NEW_BLOCK.split(b"\n")[:-1]          # the 30 D2 declaration lines


def env(decls=None, version=None, record=None):
    """A document header rebuilt around its kept defect: decls(old lines) → lines,
    version = the identifier digit, record(old line) → line; bundles untouched."""
    def edit(blob):
        lines = blob.split(b"\n")
        i = 1
        while lines[i].startswith(b":- "):
            i += 1
        head = lines[1:i] if decls is None else decls(lines[1:i])
        ver = lines[i] if version is None else b"guideline_schema_version(%s)." % version
        rec = lines[i + 1] if record is None else record(lines[i + 1])
        return b"\n".join([lines[0], *head, ver, rec, *lines[i + 2:]])
    return edit


def qenv(record):
    def edit(blob):
        lines = blob.split(b"\n")
        return b"\n".join([lines[0], record(lines[1]), *lines[2:]])
    return edit


def sub(old, new):
    """Replace the one `old`; a line already holding `new` and no other `old` = applied."""
    def edit(line):
        if new in line and old not in line.replace(new, b""):
            return line
        assert line.count(old) == 1, (old, line)
        return line.replace(old, new)
    return edit


def chain(*fs):
    def edit(x):
        for f in fs:
            x = f(x)
        return x
    return edit


def certify_cases(blob):
    """The digest mutants corrupt the vocabulary digest of the D4 table; the former
    schema-version mutant states vocabulary(none) over the table (c1 D8)."""
    corpus = "guidelines/cdc-2022-opioid/"
    if corpus + "temporal.tsv" not in data:                 # a migrated tree
        return blob
    old_t = sha(data[corpus + "temporal.tsv"])
    new_t = sha(out[corpus + "vocabulary.tsv"])
    bad = (b"%x" % ((int(new_t[:1], 16) + 1) % 16)) + new_t[1:]
    rows = []
    for row in blob.split(b"\n"):
        f = row.split(b"\t")
        if f[0] in (b"doc-v2-digest", b"query-v2-digest") and f[4].startswith(b"temporal("):
            f[4] = b"vocabulary(sha256(" + atom(old_t) + b"))"
            f[5] = b"vocabulary(sha256(" + atom(bad) + b"))"
            f[7] = f[7].replace(b": temporal.\\n", b": vocabulary.\\n")
        elif f[0] == b"doc-v3-schema-version" and f[7].endswith(b": schema_version.\\n"):
            f[4] = b"vocabulary(sha256(" + atom(old_t) + b"))"
            f[5] = b"vocabulary(none)"
            f[7] = f[7].replace(b": schema_version.\\n", b": vocabulary.\\n")
        rows.append(b"\t".join(f))
    return b"\n".join(rows)


full = lambda _: FULL
add_none = sub(b")).", b"),vocabulary(none)).")
vocab = sub(b",temporal(", b",vocabulary(")
swap_ann = lambda _: FULL[:18] + FULL[20:22] + FULL[18:20] + FULL[22:]

RETARGET = {
    "tests/v1/m7t-k1-011/input.pl.in": env(full, b"1"),                        # keeps the /3 record
    "tests/v1/m7t-k1-012/input.pl.in": env(full, b"1"),                        # keeps temporal(...)
    "tests/v1/m7t-k1-013/input.pl.in": env(lambda _: FULL[:18], b"1", add_none),  # omits the annotation pairs
    "tests/v1/m7t-k1-014/input.pl.in": env(swap_ann, b"1", vocab),             # recurrence before interval
    "tests/v1/m7t-k1-017/input.pl.in": lambda b: b,                            # v1 /5 ending temporal(...)
    "tests/v1/m7t-k1-018/input.pl.in": qenv(sub(b"'$guideline_query'(v2,", b"'$guideline_query'(v1,")),  # /4
    "tests/v1/m7t-k1-020/input.pl.in": env(full, None, vocab),                 # keeps identifier 3
    "tests/v1/q12-k1-decls-arity/input.pl.in": env(None, b"1", vocab),         # keeps range/4
    "tests/v1/q12-k1-decls-missing/input.pl.in": env(None, b"1", vocab),       # keeps the missing pair
    "tests/v1/q12-k1-decls-short/input.pl.in": env(None, b"1", vocab),         # keeps eleven pairs
    "tests/v1/q12-k1-document-record/input.pl.in": env(None, b"1"),            # keeps the /3 record
    "tests/v1/q12-k1-header-over-v3/input.pl.in": env(None, None, vocab),      # keeps identifier 2
    "tests/v1/q12-k1-header-v4/input.pl.in": env(None, None, vocab),           # keeps identifier 4
    "tests/v1/q12-k1-question-record/input.pl.in": qenv(sub(b"'$guideline_query'(v3,", b"'$guideline_query'(v1,")),
    "tests/v1/q12-k1-question-v4/input.pl.in": qenv(vocab),                    # keeps v4
    "tests/v1/q17-k1-003/input.pl.in": qenv(add_none),                         # keeps 'v1'
    "tests/v1/q18-k1-003/input.pl.in": qenv(add_none),                         # keeps $guideline_querz
    "tests/v1/q18-k1-013/input.pl.in": qenv(chain(sub(b"'$guideline_query'(v2,", b"'$guideline_query'(v1,"),
                                                  sub(b",temporaz(", b",vocabularz("))),
    "tests/v1/q18-k1-014/input.pl.in": qenv(chain(sub(b"'$guideline_query'(v2,", b"'$guideline_query'(v1,"),
                                                  vocab)),                     # keeps 'sha256'
    "tests/v1/q18-k1-018/input.pl.in": env(full, b"1", add_none),              # keeps guideline_documenz
    "tests/v1/q18-k1-022/input.pl.in": env(full, b"1", sub(b",'temporal'(", b",'vocabulary'(")),
    "tests/v1/q18-k1-023/input.pl.in": env(full, b"1", vocab),                 # keeps sha25z
    "tests/v1/q18-k1-025/input.pl.in": env(None, b"1", vocab),                 # keeps 'guideline_document'
    "tests/v1/t-c703/query.pl.in": qenv(add_none),                             # keeps v0 + the bad digests
    "tests/v1/t-c908/doc.pl.in": env(full, None, add_none),                    # keeps identifier 2
    "tests/certify/cases.tsv": lambda b: certify_cases(b),
    # interval/6 is legal in every document now: a foreign name keeps the R9 offset law
    # (the active head, never the comment or quoted-atom occurrence before it).
    "tests/v1/dq4-k1-005/input.pl.in": lambda b: compiled_c1(b).replace(b"guideline_interval(", b"guideline_intervaz("),
    "tests/v1/dq4-k1-006/input.pl.in": lambda b: compiled_c1(b).replace(b"guideline_interval(", b"guideline_intervaz("),
    # c1 D9/P6: a table's before/after rows also explain the unquantified order.
    "tests/ui/green/temporal-words/assertions.tsv": lambda b: b.replace(
        b"<tr><td>after</td><td>how long after a reference point the action occurs</td></tr>".hex().encode(),
        b"<tr><td>after</td><td>how long after a reference point the action occurs, or only that it occurs "
        b"after it</td></tr>".hex().encode()).replace(
        b"<tr><td>before</td><td>how long before a reference point the action occurs</td></tr>".hex().encode(),
        b"<tr><td>before</td><td>how long before a reference point the action occurs, or only that it occurs "
        b"before it</td></tr>".hex().encode()),

    # The pre-C1 v2 header line alone is the one D4 header: drop it so the probe keeps
    # its malformed-header law (vocabulary_load, header).
    "tests/red/temporal_load--v2-header.temporal.tsv": lambda b: b[len(HEADER):] if b.startswith(HEADER) else b,
}


# ---------- Phase A ----------

scope = tracked("guidelines", "tests") + ["rust/ckc-kani-harness/src/lib.rs"]
data = {p: (root / p).read_bytes() for p in scope}
out = {}            # post-migration relpath → bytes (changed or moved files)
moved = {}          # pre-migration relpath → post-migration relpath
gone = set()        # deleted pre-migration relpaths

# Retired items (contract D14; user-approved deletion set of 70).
v1_rows = (root / "tests/v1/cases.tsv").read_text().splitlines(keepends=True)
v1_cases = {}
for row in v1_rows:
    if not row.startswith("#"):
        case, probe = row.split("\t")[:2]
        v1_cases.setdefault(case, []).append(probe)
for case, probes in v1_cases.items():
    hits = [dispose("tests/v1", f"{case}/{p}") for p in probes]
    if any(hits):
        assert all(hits), f"partly retired case {case}"
        gone |= {p for p in scope if p.startswith(f"tests/v1/{case}/")}
kept = "".join(r for r in v1_rows if r.startswith("#") or r.split("\t")[0] not in
               {c for c, ps in v1_cases.items() if all(dispose("tests/v1", f"{c}/{p}") for p in ps)})
if kept.encode() != data["tests/v1/cases.tsv"]:
    out["tests/v1/cases.tsv"] = kept.encode()
for family, item in sorted(retired):
    if family == "tests/red":
        gone |= {p for p in scope if p.startswith(f"tests/red/{item}.")}
    elif family == "tests/check-mutants":
        gone |= {p for p in scope if p.startswith(f"tests/check-mutants/{item}.")}
        rows_cm = out.get("tests/check-mutants/cases.tsv", data["tests/check-mutants/cases.tsv"])
        kept_cm = b"".join(r for r in rows_cm.splitlines(keepends=True) if r.split(b"\t")[0] != item.encode())
        if kept_cm != rows_cm:
            out["tests/check-mutants/cases.tsv"] = kept_cm

# tests/certify-one: its generator writes the one D4 header itself (B5).
if "tests/certify-one/v3-header.txt.in" in data:
    gone.add("tests/certify-one/v3-header.txt.in")

# Renames: red probes of the load class take its D5 name (contract D11); tables take
# the D4 name and the one-line header.
TABLE = re.compile(r"(^|/)temporal\.tsv(\.in)?$|\.temporal\.tsv(\.in)?$")
for p in scope:
    if p in gone:
        continue
    new_path = p.replace("tests/red/temporal_load--", "tests/red/vocabulary_load--", 1)
    if TABLE.search(p):
        head, _, base = new_path.rpartition("/")
        base = base.replace("temporal.tsv", "vocabulary.tsv", 1)
        new_path = f"{head}/{base}" if head else base
        blob = data[p]
        old = next((h for h in OLD_HEADERS if blob.startswith(h)), None)
        if p in RETARGET:
            blob = RETARGET[p](blob)
        elif old is not None:
            blob = HEADER + blob[len(old):]
        else:
            raise SystemExit(f"table without a pre-C1 header and no ruling: {p}")
        moved[p] = new_path
        out[new_path] = blob
    elif new_path != p:
        moved[p] = new_path
        out[new_path] = data[p]

# Compiled documents + questions (plain, `.in`, `.tpl.in`, green pins), the canonical envelope.
malformed = []
for p in scope:
    if p in gone or p in moved:
        continue
    blob = out.get(p, data[p])
    if p.endswith(".tpl.in"):
        pin, body = blob.split(b"\n", 1)
        new = RETARGET[p](body) if p in RETARGET else compiled_c1(body)
        if new is None:
            malformed.append(p)
            continue
        if new != body:
            out[p] = b"@@sha256 " + sha(expand(b"@@sha256 x\n" + new)) + b"\n" + new
        continue
    if p in RETARGET:
        new = RETARGET[p](blob)
        if new != blob:
            out[p] = new
        continue
    if DOC_LINE1.match(blob) or QUERY_LINE1.match(blob):
        new = compiled_c1(blob)
        if new is None:
            malformed.append(p)
        elif new != blob:
            out[p] = new
    elif (k := line1_defect(blob)) is not None:
        out[p] = blob[:k] + doc_c1(blob[k:])
if malformed:
    raise SystemExit(f"{len(malformed)} pre-C1 envelopes without a ruling:\n" + "\n".join(malformed))

# check-mutant pins + the section column: the table section names the D4 file.
cm = "tests/check-mutants/cases.tsv"
blob = out.get(cm, data[cm])
if b"\tcorpus_temporal\t" in blob:
    out[cm] = blob.replace(b"\tcorpus_temporal\t", b"\tcorpus_vocabulary\t")
for p in scope:
    if p.startswith("tests/check-mutants/") and p.endswith(".expect") and p not in gone:
        blob = out.get(p, data[p])
        new = blob.replace(b"ckc: temporal: guidelines/cdc-2022-opioid/temporal.tsv: ",
                           b"ckc: vocabulary: guidelines/cdc-2022-opioid/vocabulary.tsv: ")
        if new != blob:
            out[p] = new

# Kani harness constants (contract P10): DOC + QUERIES in D2/D3, their digests, the
# answers custody digest, the goal byte offset + the DOC-sized unwind bounds.
KANI = "rust/ckc-kani-harness/src/lib.rs"
LIT = re.compile(rb'^(\s*)"((?:[^"\\]|\\.)*)",\n', re.M)


def unrust(s):
    return s.replace(b"\\n", b"\n").replace(b'\\"', b'"').replace(b"\\\\", b"\\")


def rust_lines(blob, indent):
    return b"".join(indent + b'"' + line.replace(b"\\", b"\\\\").replace(b'"', b'\\"').replace(b"\n", b"\\n")
                    + b'",\n' for line in blob.splitlines(keepends=True))


def concat_blocks(src, opener):
    """(start, end, bytes, indent) of each `concat!(` literal run after `opener`."""
    blocks = []
    at = src.index(opener)
    for m in re.finditer(rb"concat!\(\n((?:\s*\"(?:[^\"\\]|\\.)*\",\n)+)\s*\)", src[at:]):
        lits = LIT.findall(m.group(1))
        blocks.append((at + m.start(1), at + m.end(1), b"".join(unrust(s) for _, s in lits), lits[0][0]))
        if opener != b"const QUERIES" or len(blocks) == 2:
            break
    return blocks


src = data[KANI]
if b"guideline_document/3" in src:
    doc_blk = concat_blocks(src, b"const DOC")[0]
    q_blks = concat_blocks(src, b"const QUERIES")
    ans_blk = concat_blocks(src, b"const ANSWERS")[0]
    doc = doc_c1(doc_blk[2])
    queries = [query_c1(b[2]) for b in q_blks]
    old_qsha = [sha(b[2]) for b in q_blks]
    new_qsha = [sha(q) for q in queries]
    answers = ans_blk[2].replace(b"query_sha256(" + atom(old_qsha[0]) + b")",
                                 b"query_sha256(" + atom(new_qsha[0]) + b")")
    goal = next(i for i in range(len(queries[0])) if queries[0][i] != queries[1][i])
    old_goal = next(i for i in range(len(q_blks[0][2])) if q_blks[0][2][i] != q_blks[1][2][i])
    edits = sorted([(doc_blk, doc), (q_blks[0], queries[0]), (q_blks[1], queries[1]), (ans_blk, answers)],
                   key=lambda e: -e[0][0])
    for (start, end, _, indent), new in edits:
        src = src[:start] + rust_lines(new, indent) + src[end:]
    for old, new in zip(old_qsha, new_qsha):
        src = src.replace(b'b"' + old + b'"', b'b"' + new + b'"')
    # The answers digest beside `&answers` = sha256(ANSWERS) (base: a stale value), the
    # DOC-bounded unwinds = DOC.len() + 1 and their comments = DOC.len() (base: 948,
    # stale; DOC held 946 bytes) — checked natively by rust/ckc/tests/kani_constants.rs.
    src = re.sub(rb'(&answers,\n\s*b")[0-9a-f]{64}(")', lambda m: m.group(1) + sha(answers) + m.group(2), src)
    src = src.replace(b"query[%d] = " % old_goal, b"query[%d] = " % goal)
    n = len(doc)
    src = re.sub(rb"#\[kani::unwind\(\d+\)\](\n    fn (?:reader_trace_check_small|engine_answer_tiny)\b)",
                 lambda m: b"#[kani::unwind(%d)]" % (n + 1) + m.group(1), src)
    src = re.sub(rb"DOC has \d+ bytes", b"DOC has %d bytes" % n, src)
    src = re.sub(rb"DOC\.len\(\) = \d+; bound = \d+", b"DOC.len() = %d; bound = %d" % (n, n), src)
    out[KANI] = src

# Digests of changed files rebound to a fixed point (r99-repin.py mechanics).
def cur(p):
    return out.get(moved.get(p, p), data[p])


rebind = {}
for p in scope:
    if p in gone or p == KANI:
        continue
    old, new = data[p], cur(p)
    if p.endswith(".tpl.in"):
        old_exp, new_exp = expand(old), expand(new)
        if old_exp != new_exp:
            rebind[sha(old_exp)] = sha(new_exp)
        continue
    if new != old:
        rebind[sha(old)] = sha(new)
while True:
    grew = False
    for p in scope:
        if p in gone or p == KANI:
            continue
        blob = cur(p)
        new = TERM.sub(lambda m: atom(rebind[m.group(2)]) if m.group(2) in rebind else m.group(), blob)
        new = HEX.sub(lambda m: rebind.get(m.group(), m.group()), new)
        if p.endswith(".tpl.in"):
            if new.split(b"\n", 1)[1] != blob.split(b"\n", 1)[1]:
                raise SystemExit(f"template binds a rebound digest: {p}")
            continue
        if new != blob:
            prev = sha(blob)
            out[moved.get(p, p)] = new
            rebind[sha(data[p])] = sha(new)
            if prev != sha(data[p]):
                rebind[prev] = sha(new)
            grew = True
    if not grew:
        break

if "--dry" in sys.argv:
    for p in sorted(set(out) - set(moved.values())):
        if (root / p).is_file() and (root / p).read_bytes() != out[p]:
            print(f"would change {p}")
    print(f"c1 phase A (dry): {len(gone)} deleted, {len(moved)} renamed, {len(out)} changed, "
          f"{len(rebind)} digests rebound")
    sys.exit(0)

# Write phase A.
for p in sorted(gone):
    (root / p).unlink()
for p in sorted(moved):
    (root / p).unlink()
for p, blob in sorted(out.items()):
    (root / p).parent.mkdir(parents=True, exist_ok=True)
    (root / p).write_bytes(blob)
for d in sorted({(root / p).parent for p in gone | set(moved)}, key=lambda d: -len(d.parts)):
    while d != root and d.is_dir() and not any(d.iterdir()):
        d.rmdir()
        d = d.parent
print(f"c1 phase A: {len(scope)} files scanned, {len(gone)} deleted, {len(moved)} renamed, "
      f"{len(out)} written, {len(rebind)} digests rebound")


# ---------- Phase B: regeneration through the C1 binary + compiler ----------

def run(argv, cwd, stdin=b"", env=None):
    r = subprocess.run(argv, cwd=cwd, input=stdin, capture_output=True, env=env, timeout=600)
    return r.returncode, r.stdout, r.stderr


def put(rel, blob):
    path = root / rel
    if not path.is_file() or path.read_bytes() != blob:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(blob)
        regenerated.append(rel)


regenerated = []

# B1 tests/v1: every row runs at its recording layout inside a private root (v1_suite.rs).
LAYOUT = ".scratch/m5u2/suite/cases"
v1 = root / "tests/v1"


def materialize(src, dst, top):
    for path in sorted(src.iterdir()):
        if path.is_dir():
            if not (top and path.name == "expect"):
                materialize(path, dst / path.name, False)
            continue
        dst.mkdir(parents=True, exist_ok=True)
        blob = path.read_bytes()
        if path.name.endswith(".tpl.in"):
            pin = blob.split(b"\n", 1)[0][len(b"@@sha256 "):]
            exp = expand(blob)
            assert sha(exp) == pin, path
            (dst / path.name[:-len(".tpl.in")]).write_bytes(exp)
        else:
            (dst / path.name[:-len(".in")]).write_bytes(blob)


case_rows = []
with tempfile.TemporaryDirectory() as private:
    private = Path(private)
    lines = (v1 / "cases.tsv").read_text().splitlines(keepends=True)
    for case in dict.fromkeys(r.split("\t")[0] for r in lines if not r.startswith("#")):
        materialize(v1 / case, private / LAYOUT / case, True)
    new_lines = []
    for row in lines:
        if row.startswith("#"):
            new_lines.append(row)
            continue
        case, probe, mode, _rc, _n, args = row.rstrip("\n").split("\t")
        argv = [str(ckc), "v1", mode] + [f"{LAYOUT}/{case}/{a}" for a in args.split(" ") if a]
        rc, so, se = run(argv, private)
        put(f"tests/v1/{case}/expect/{probe}.stdout", so)
        put(f"tests/v1/{case}/expect/{probe}.stderr", se)
        n = se.count(b"\n") + (1 if se and not se.endswith(b"\n") else 0)
        new_lines.append("\t".join([case, probe, mode, str(rc), _n if _n == "-" else str(n), args]) + "\n")
    put("tests/v1/cases.tsv", "".join(new_lines).encode())

# B2 tests/red + tests/green pins through their generator (the `ckc check` argv).
rc, so, se = run([str(root / ".agent/archive/generators/probe-pins"), str(root)], root)
assert rc == 0, se
regenerated += [line.split(" ", 1)[1] for line in so.decode().splitlines() if line.startswith("wrote ")]

# B3 tests/ui: each case whose tree changed → its audit manifests, stream pins + goldens
# (ui_fixtures.rs `Fixture` + `ui_case`; these cases carry no `worktree`, so no commit).
UI_ENV = dict(os.environ, GIT_OPTIONAL_LOCKS="0", GIT_CONFIG_GLOBAL="/dev/null", GIT_CONFIG_SYSTEM="/dev/null")
ui_changed = sorted({"/".join(p.split("/")[:4]) for p in list(out) + list(moved) + sorted(gone)
                     if p.startswith("tests/ui/") and p.split("/")[4:5] == ["tree"]})
for case in ui_changed:
    c = root / case
    assert not any((c / m).exists() for m in ("tree-order.txt", "empty-dirs.txt", "worktree")), case
    with tempfile.TemporaryDirectory() as tmp:
        scratch = Path(tmp)
        tree = scratch / case / "tree"
        shutil.copytree(c / "tree", tree)
        # Review bundles never read the table, so a red case's broken table stays out
        # of its manifest's derivation.
        bare = scratch / "bare"
        shutil.copytree(c / "tree", bare)
        for g in sorted((bare / "guidelines").iterdir()):
            if (g / "audit/review-manifest.tsv").is_file():
                (g / "vocabulary.tsv").unlink(missing_ok=True)
                rc, _, se = run([str(ckc), "review-manifest", g.name], bare)
                assert rc == 0, (case, se)
                manifest = (g / "audit/review-manifest.tsv").read_bytes()
                put(f"{case}/tree/guidelines/{g.name}/audit/review-manifest.tsv", manifest)
                (tree / "guidelines" / g.name / "audit/review-manifest.tsv").write_bytes(manifest)
        for n, row in enumerate((c / "case.tsv").read_text().splitlines()):
            if not row or row.startswith("#"):
                continue
            argv, _rc, err_pin, out_pin = row.split("\t")
            dest = scratch / f"out-{n}"
            args = [{"@TREE@": f"{case}/tree", "@OUT@": str(dest)}.get(a, a) for a in argv.split(" ")]
            rc, so, se = run([str(ckc), "ui", *args], scratch, env=dict(UI_ENV, TMPDIR=tmp))
            assert rc == int(_rc), (case, row, se)
            for pin, blob in ((out_pin, so), (err_pin, se)):
                if pin != "-":
                    put(f"{case}/{pin}", blob)
            if args[0] == "render" and case.startswith("tests/ui/green/"):
                fresh = {p.relative_to(dest).as_posix() for p in dest.rglob("*") if p.is_file()}
                for p in sorted((c / "golden").rglob("*")):
                    if p.is_file() and p.relative_to(c / "golden").as_posix() not in fresh:
                        p.unlink()
                        regenerated.append(str(p.relative_to(root)))
                for rel in sorted(fresh):
                    put(f"{case}/golden/{rel}", (dest / rel).read_bytes())

# B4 the corpus review manifest (review bundle v2 digests over the new headers).
for g in sorted((root / "guidelines").iterdir()):
    before = (g / "audit/review-manifest.tsv").read_bytes()
    rc, _, se = run([str(ckc), "review-manifest", g.name], root)
    assert rc == 0, (g, se)
    if (g / "audit/review-manifest.tsv").read_bytes() != before:
        regenerated.append(str((g / "audit/review-manifest.tsv").relative_to(root)))

# B5 certify-one fixtures through their generator.
rc, _, se = run([str(root / ".agent/archive/generators/certify-one-fixtures"), str(root), str(ckc)], root)
assert rc == 0, se

# Scope digest = sha256 over sorted (relpath NUL bytes NUL) of the tracked + new scope files.
digest = hashlib.sha256()
listed = subprocess.run(["git", "ls-files", "-z", "--cached", "--others", "--exclude-standard", "--",
                         "guidelines", "tests", KANI], cwd=root, capture_output=True, check=True).stdout
for p in sorted({p for p in listed.decode().split("\0") if p and (root / p).is_file()}):
    digest.update(p.encode() + b"\0" + (root / p).read_bytes() + b"\0")
print(f"c1 phase B: {len(regenerated)} files regenerated; scope tree sha256 {digest.hexdigest()}")
