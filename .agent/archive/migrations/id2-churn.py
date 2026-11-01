#!/usr/bin/env python3
"""id2 churn oracle (contract id2 D7), independent of revise.py: predicts the exact tip bytes of
every in-scope path from its base bytes, then compares.

Usage: python3 -I <this> <base-dir> <tip-dir>   (base = the seed; tip = a migrated pre-migration copy)
rc 0 = every in-scope path matches its prediction; rc 1 = defects, one line each; rc 2 = usage.

Census (both trees, read from disk: tracked + untracked files, ignored ones excluded) = guidelines/,
tests/ (minus `*.rs` code + the manifest's `code` rows), the Kani constants, release-manifest.tsv.
Prediction per path, classified by the committed transform manifest
(.agent/archive/migrations/id2-transforms.tsv, read from the tip):
- frozen: the base bytes.
- retarget: the C1 negative keeps an invalid token (version 2 → 1, tag v2 → v1, quoted 'v1' → 'v2').
- positive: the D1 token edits (the line `guideline_schema_version(1).` → `(2).`; the tag
  `'$guideline_query'(v1,` → `(v2,`; the row `meta\tschema\tv1` → `v2`), then every digest that
  covers changed bytes, to a fixed point: a file digest anywhere (a `sha256(…)` term requotes:
  an atom opening with a digit takes quotes); a v1-suite template pin over its expansion; a
  review bundle's clause + review digests from the retained clause stream.
- the Kani constants: the token edits inside DOC + QUERIES, their QUERY_SHA256 literals, ANSWERS'
  citation + every copy of the answers digest.
- release-manifest.tsv: the base bytes. The migration leaves it to P-d (`just amend-manifest`),
  whose output `ckc check` grades against its own derivation from HEAD.
Anything else that differs is a defect, as is a path missing from, or added to, the tip's census.
"""
import hashlib
import os
import re
import subprocess
import sys
from pathlib import Path

MANIFEST = ".agent/archive/migrations/id2-transforms.tsv"
KANI = "rust/ckc-kani-harness/src/lib.rs"
RELEASE = "release-manifest.tsv"
PREFIXES = ("guidelines/", "tests/")
VERSION = re.compile(rb"^guideline_schema_version\(1\)\.$", re.M)
HEX64 = re.compile(rb"(?<![0-9a-f])[0-9a-f]{64}(?![0-9a-f])")
SHA_TERM = re.compile(rb"(?<=sha256\()'?([0-9a-f]{64})'?(?=\))")
CONCAT = re.compile(rb"concat!\(\n((?:\s*\"(?:[^\"\\]|\\.)*\",\n)+)\s*\)")
USAGE = "usage: id2-churn.py <base-dir> <tip-dir>"


def digest(blob):
    return hashlib.sha256(blob).hexdigest().encode()


def quoted(hexdigest):
    return b"'" + hexdigest + b"'" if hexdigest[:1].isdigit() else hexdigest


def census(root, code):
    names = subprocess.check_output(
        ["git", "ls-files", "-z", "--cached", "--others", "--exclude-standard"], cwd=root
    ).decode().split("\0")
    return sorted({p for p in names if p and p not in code and (root / p).is_file()
                   and (p.startswith(PREFIXES) and not p.endswith(".rs") or p in (KANI, RELEASE))})


def classes(tip):
    frozen, retarget, code = [], set(), set()
    for line in (tip / MANIFEST).read_text().splitlines():
        if line and not line.startswith("#"):
            path, kind, _ = line.split("\t")
            if kind == "frozen":
                frozen.append(path)
            elif kind == "retarget":
                retarget.add(path)
            elif kind == "code":
                code.add(path)
    return tuple(frozen), retarget, code


def tokens(blob):
    return (VERSION.sub(b"guideline_schema_version(2).", blob)
            .replace(b"'$guideline_query'(v1,", b"'$guideline_query'(v2,")
            .replace(b"meta\tschema\tv1\n", b"meta\tschema\tv2\n"))


def negative(blob):
    return (blob.replace(b"guideline_schema_version(2).", b"guideline_schema_version(1).")
            .replace(b"'$guideline_query'(v2,", b"'$guideline_query'(v1,")
            .replace(b"'$guideline_query'('v1',", b"'$guideline_query'('v2',"))


# v1-suite template grammar (rust/ckc/tests/v1_suite.rs `expand`), restated.
def comma_tree(n, first, leaf):
    layer = [first] + [leaf] * (n - 1)
    while len(layer) > 1:
        layer = [b"','(" + a + b"," + b + b")" for a, b in zip(layer[::2], layer[1::2])] + \
                ([layer[-1]] if len(layer) % 2 else [])
    return layer[0]


def expand(template):
    out = []
    for line in template.rstrip(b"\n").split(b"\n")[1:]:
        if line.startswith(b"@@run "):
            lo, hi, body = line[6:].split(b" ", 2)
            for i in range(int(lo), int(hi) + 1):
                out.append(body.replace(b"@i@", str(i).encode()).replace(b"@j@", str(i + 1).encode()))
        elif line.startswith(b"@@tree "):
            n, first, leaf, prefix, suffix = line[7:].split(b"\t")
            out.append(prefix + comma_tree(int(n), first, leaf) + suffix)
        else:
            out.append(line)
    return b"".join(x + b"\n" for x in out)


def repin(template):
    head, body = template.split(b"\n", 1)
    assert head.startswith(b"@@sha256 ")
    return b"@@sha256 " + digest(expand(template)) + b"\n" + body


def retained(pl):
    keep = [line for line in pl.split(b"\n")[:-1]
            if not (line.startswith(b"guideline_document(") or line.startswith(b"%") or line.startswith(b":- "))]
    return b"".join(line + b"\n" for line in keep)


def review_rows(blob, docs):
    """Recompute each bundle row's clause + review digests from the predicted documents."""
    out = []
    for line in blob.split(b"\n"):
        f = line.split(b"\t")
        if line.startswith(b"#") or len(f) != 6 or f[0] not in docs:
            out.append(line)
            continue
        cl = digest(retained(docs[f[0]]))
        block = b"bundle v2 %s\nace %s\ncoverage %s\npayload %s\nclauses %s\n" % (f[0], f[1], f[2], f[3], cl)
        out.append(b"\t".join(f[:4] + [cl, digest(block)]))
    return b"\n".join(out)


def literals(src, opener):
    """(start, end, bytes) of the string literals of the first `concat!(` after `opener`."""
    at = src.index(opener)
    m = CONCAT.search(src[at:])
    assert m is not None, opener
    body = m.group(1)
    text = b"".join(x.replace(b"\\n", b"\n").replace(b'\\"', b'"').replace(b"\\\\", b"\\")
                    for x in re.findall(rb'"((?:[^"\\]|\\.)*)"', body))
    return at + m.start(1), at + m.end(1), text


def kani_predict(src):
    """Token edits inside DOC + both QUERIES; QUERY_SHA256, ANSWERS' citation + its digest follow."""
    out = src
    old_q, new_q = [], []
    # DOC
    s, e, _ = literals(out, b"const DOC")
    out = out[:s] + out[s:e].replace(b"guideline_schema_version(1).\\n", b"guideline_schema_version(2).\\n") + out[e:]
    # QUERIES: two concat! blocks after `const QUERIES`
    at = out.index(b"const QUERIES")
    for _ in range(2):
        m = CONCAT.search(out[at:])
        assert m is not None, "QUERIES"
        s, e = at + m.start(1), at + m.end(1)
        _, _, text = literals(out[at:], b"concat!(")
        old_q.append(digest(text))
        new_body = out[s:e].replace(b"'$guideline_query'(v1,", b"'$guideline_query'(v2,")
        out = out[:s] + new_body + out[s + (e - s):]
        _, _, ntext = literals(out[at:], b"concat!(")
        new_q.append(digest(ntext))
        at = s + len(new_body)
    s, e, answers = literals(out, b"const ANSWERS")
    new_body = out[s:e].replace(b"query_sha256(" + quoted(old_q[0]) + b")", b"query_sha256(" + quoted(new_q[0]) + b")")
    out = out[:s] + new_body + out[e:]
    _, _, new_answers = literals(out, b"const ANSWERS")
    for o, n in zip(old_q + [digest(answers)], new_q + [digest(new_answers)]):
        out = out.replace(o, n)
    return out


NONCANON = re.compile(rb"ace_to_pl_error\(check_load,noncanonical\((\d+),(\d+)\)\)\.\n")


def substituted(blob, rebind):
    """The fixed-point substitutions: rebound file digests, a `sha256(…)` term requoted."""
    new = SHA_TERM.sub(lambda m: quoted(rebind[m.group(1)]) if m.group(1) in rebind else m.group(0), blob)
    return HEX64.sub(lambda m: rebind.get(m.group(0), m.group(0)), new)


def materialized(path, blob):
    """The v1 suite's input file: `.in` dropped, a `.tpl.in` template expanded."""
    return expand(blob) if path.endswith(".tpl.in") else blob


def remapped_rejects(old, pred, rebind):
    """A v1 check/render reject `noncanonical(L,C)` moves with the bytes before it on line L:
    a digest requoted there shifts C by the quotes it gains or loses. Defects for a reject whose
    line the prediction does not reproduce."""
    out, defects = {}, []
    for row in old["tests/v1/cases.tsv"].decode().splitlines():
        if not row or row.startswith("#"):
            continue
        case, probe, mode, _rc, _n, args = row.split("\t")
        err = f"tests/v1/{case}/expect/{probe}.stderr"
        m = NONCANON.fullmatch(old.get(err, b""))
        if mode not in ("check", "render") or m is None:
            continue
        arg = os.path.normpath(f"tests/v1/{case}/{args.split(' ')[0]}")
        src = next((p for p in (arg + ".in", arg + ".tpl.in") if p in old), None)
        if src is None:
            defects.append(f"{err}: no input for {arg}")
            continue
        line, col = int(m.group(1)), int(m.group(2))
        before = materialized(src, old[src]).split(b"\n")[line - 1][: col - 1]
        moved = substituted(tokens(before), rebind)
        if not materialized(src, pred[src]).split(b"\n")[line - 1].startswith(moved):
            defects.append(f"{err}: the predicted input line {line} does not keep the reject's prefix")
            continue
        out[err] = b"ace_to_pl_error(check_load,noncanonical(%d,%d)).\n" % (line, len(moved) + 1)
    return out, defects


def main():
    if len(sys.argv) != 3:
        print(USAGE, file=sys.stderr)
        return 2
    base, tip = Path(sys.argv[1]).resolve(), Path(sys.argv[2]).resolve()
    frozen, retarget, code = classes(tip)
    paths = census(base, code)
    defects = [f"{p}: missing from the tip" for p in sorted(set(paths) - set(census(tip, code)))]
    defects += [f"{p}: added to the tip" for p in sorted(set(census(tip, code)) - set(paths))]
    old = {p: (base / p).read_bytes() for p in paths}
    pred = {}
    for p, blob in old.items():
        if p.startswith(frozen) or p == RELEASE:
            pred[p] = blob
        elif p in retarget:
            pred[p] = negative(blob)
        elif p == KANI:
            pred[p] = kani_predict(blob)
        elif p.endswith(".tpl.in"):
            head, body = blob.split(b"\n", 1)
            pred[p] = head + b"\n" + tokens(body)
        else:
            pred[p] = tokens(blob)
    # digests covering changed bytes, to a fixed point
    while True:
        rebind = {}
        for p, blob in old.items():
            if p in (KANI, RELEASE) or p.startswith(frozen):
                continue
            if pred[p] != blob:
                rebind[digest(blob)] = digest(pred[p])
                if p.endswith(".tpl.in"):
                    rebind[digest(expand(blob))] = digest(expand(repin(pred[p])))
        changed = False
        for p in old:
            if p in (KANI, RELEASE) or p.startswith(frozen):
                continue
            blob = pred[p]
            new = substituted(blob, rebind)
            if p.endswith(".tpl.in"):
                new = repin(new)
            if new != blob:
                pred[p], changed = new, True
        if not changed:
            break
    rejects, remap_defects = remapped_rejects(old, pred, rebind)
    pred.update(rejects)
    defects += remap_defects
    # review bundles: clause + review digests of every row whose document the census holds
    for p in old:
        if p.endswith("/audit/review-manifest.tsv") and not p.startswith(frozen):
            pl_dir = p[: -len("audit/review-manifest.tsv")] + "pl/"
            docs = {q[len(pl_dir):-3].encode(): pred[q] for q in pred if q.startswith(pl_dir) and q.endswith(".pl")}
            pred[p] = review_rows(pred[p], docs)
    present = set(census(tip, code))
    for p in paths:
        if p not in present:
            continue
        got = (tip / p).read_bytes()
        if got != pred[p]:
            a, b = pred[p].split(b"\n"), got.split(b"\n")
            k = next((i for i, (x, y) in enumerate(zip(a, b)) if x != y), min(len(a), len(b)))
            defects.append(f"{p}: line {k + 1} differs from the prediction")
    for d in defects:
        print(d)
    print(f"id2-churn: {len(paths)} paths, {len(defects)} defects")
    return 1 if defects else 0


if __name__ == "__main__":
    sys.exit(main())
