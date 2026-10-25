#!/usr/bin/env python3
"""Schema revision migration: the declaration block every compiled document opens with gains
the indicators `--add` names, right after the `--after` indicator (generalizes s1-revise.py +
s2-revise.py, whose S2 run it replays: `--add guideline_qualifier/4 --after
guideline_preference/4 --frozen tests/s1-compiler/ --frozen tests/s1-codec/ --frozen
tests/s2-compiler/ --frozen tests/s2-codec/ --label s2`).

Usage: python3 -I <this> <repo-root> <ckc-binary> --add NAME/ARITY [--add …] --after NAME/ARITY
       [--frozen PREFIX …] [--label TEXT] [--dry] [--semantic [--recompile QUERY-CASE …]]
Toolchain PATH = the reference swipl (10.0.2); repo-root carries the revised compiler, whose `v1_indicators`
list = the old block with the added indicators after the anchor (checked). --dry computes
Phase A without writing. Scope = tracked guidelines/, tests/, the Kani harness, minus each
--frozen prefix (suite-owned byte snapshots).

Phase A: canonical old block → new block (+2 lines per added indicator). A block malformed
before the anchor stays unchanged; records + semantic clauses stay unchanged. C1 machinery
retained: digest rebinding to a fixed point, .tpl.in expansion pins, tests/ui fixture commit
ids, Kani DOC + its byte-sized bounds, certify swap-lines operands past the insertion point.
Phase B: regenerate tests/v1 streams, red/green pins, changed UI manifests + goldens,
certify-one fixtures through their generators; recompile the corpus + its queries and
review manifests, requiring byte equality with Phase A's output after each command.
The caller amends release-manifest.tsv after committing the migrated release inputs
(`just amend-manifest`, ops.md).

Predicted churn: R9 offsets past the insertion point +2 lines per indicator; UI
compiled-line counts likewise; a document read as answers has term_count(N+2k); raw-pl
custody hashes rebound. Question records, answer artifacts + review bundle v2 digests stay
unchanged. Two fresh base copies must give one scope sha256; rerunning must change no bytes.

--semantic (a revision that changes clause bytes too, nt1): Phase B records each corpus
command's drift instead of requiring Phase A identity, the --recompile query cases regenerate,
`guidelines/schema.pl` may join the output census, and the predictions above give way to the
unit's churn oracle (nt1: nt1-churn.py; certify mutants then re-anchor via
nt1-certify-repin.py, since Phase A shifts their operands by the declaration block alone).
"""
import argparse
import hashlib
import os
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path


COMPILER = "vendor/ape/prolog/ace_to_pl.pl"
KANI = "rust/ckc-kani-harness/src/lib.rs"
SCOPE = ("guidelines", "tests", KANI)
DOC_LINE1 = re.compile(rb"\A% [^\n]*\.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit\.\n")
HEX = re.compile(rb"(?<![0-9a-f])[0-9a-f]{64}(?![0-9a-f])")
TERM = re.compile(rb"(?<=sha256\()('?)([0-9a-f]{64})\1(?=\))")
SHA1 = re.compile(rb"(?<![0-9a-f])[0-9a-f]{40}(?![0-9a-f])")
LIT = re.compile(rb'^(\s*)"((?:[^"\\]|\\.)*)",\n', re.M)


def block(indicators):
    return b"".join(b":- multifile(%s/%d).\n:- discontiguous(%s/%d).\n" %
                    (name.encode(), arity, name.encode(), arity) for name, arity in indicators)


# Set by configure(): the old + new declaration blocks, the added pairs, the anchor's end
# inside the old block, the frozen prefixes, the report label.
OLD_BLOCK = NEW_BLOCK = ADDED = b""
CUT = b""
FROZEN = ()
LABEL = "revise"
# --semantic (nt1): the revision also changes clause bytes, so the corpus recompile
# may drift from Phase A (the unit's churn oracle grades it) and the schema file
# `guidelines/schema.pl` may join the census. --recompile names the tests/queries cases
# whose trees are compiler output (the old compiler reproduces them; the other cases are
# hand-shaped and keep Phase A bytes): they recompile like the corpus.
SEMANTIC = False
SCHEMA = "guidelines/schema.pl"
RECOMPILE = ()


def indicator(text):
    name, _, arity = text.partition("/")
    assert re.fullmatch(r"[a-z][a-z0-9_]*", name) and arity.isdigit(), f"not NAME/ARITY: {text}"
    return name, int(arity)


def configure(root, added, after, frozen, label):
    """The compiler's v1_indicators list = old list + `added` right after `after`."""
    global OLD_BLOCK, NEW_BLOCK, ADDED, CUT, FROZEN, LABEL
    src = (root / COMPILER).read_text()
    body = re.search(r"^v1_indicators\(\[\n(.*?)\n\]\)\.$", src, re.M | re.S)
    assert body, f"{COMPILER}: no v1_indicators list"
    new = [indicator(t.strip()) for t in body.group(1).split(",")]
    assert after in new and new[new.index(after) + 1:new.index(after) + 1 + len(added)] == added, \
        f"{COMPILER}: {added} do not follow {after} in v1_indicators"
    old = [i for i in new if i not in added]
    k = old.index(after) + 1
    OLD_BLOCK, ADDED, NEW_BLOCK = block(old), block(added), block(new)
    CUT = block(old[:k])
    assert NEW_BLOCK == CUT + ADDED + OLD_BLOCK[len(CUT):]
    FROZEN, LABEL = tuple(frozen), label


def doc_revise(blob):
    """Keep line-1 defects; require the complete canonical declaration prefix."""
    offsets = [0]
    if blob.startswith(b"\xef\xbb\xbf"):
        offsets.append(3)
    offsets.append(blob.find(b"\n") + 1)
    for start in dict.fromkeys(offsets):
        match = DOC_LINE1.match(blob[start:])
        if match is None:
            continue
        at = start + match.end()
        if blob[at:].startswith(NEW_BLOCK):
            return blob
        if blob[at:].startswith(OLD_BLOCK):
            at += len(CUT)
            return blob[:at] + ADDED + blob[at:]
    return blob


def sha(blob):
    return hashlib.sha256(blob).hexdigest().encode()


def atom(digest):
    return b"'" + digest + b"'" if digest[:1].isdigit() else digest


def tree(n, first, leaf):
    layer = [first] + [leaf] * (n - 1)
    while len(layer) > 1:
        layer = [b"','(" + layer[i] + b"," + layer[i + 1] + b")" if i + 1 < len(layer) else layer[i]
                 for i in range(0, len(layer), 2)]
    return layer[0]


def expand(template):
    """v1_suite.rs expansion; the @@sha256 line is not part of the input."""
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


def template_pin(blob):
    _, body = blob.split(b"\n", 1)
    return b"@@sha256 " + sha(expand(blob)) + b"\n" + body


def tracked(root, *prefixes, others=False):
    args = ["git", "ls-files", "-z", "--cached"]
    if others:
        args += ["--others", "--exclude-standard"]
    names = subprocess.check_output([*args, "--", *prefixes], cwd=root).decode().split("\0")
    # Suite-owned byte snapshots (--frozen) keep the bytes their suites froze.
    return sorted({p for p in names if p and (root / p).is_file() and not p.startswith(FROZEN)})


GIT_ENV = dict(GIT_CONFIG_GLOBAL="/dev/null", GIT_CONFIG_SYSTEM="/dev/null", GIT_DEFAULT_HASH="sha1",
               GIT_AUTHOR_NAME="fixture", GIT_AUTHOR_EMAIL="fixture@localhost", GIT_COMMITTER_NAME="fixture",
               GIT_COMMITTER_EMAIL="fixture@localhost", GIT_AUTHOR_DATE="2026-01-01T00:00:00+00:00",
               GIT_COMMITTER_DATE="2026-01-01T00:00:00+00:00")


def fixture_commit(files):
    """ui_fixtures.rs Fixture::new: deterministic HEAD before its worktree overlay."""
    with tempfile.TemporaryDirectory() as tmp:
        for rel, blob in files.items():
            target = Path(tmp) / rel
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(blob)
        env = dict(os.environ, **GIT_ENV)
        for args in (["init", "-q", "-b", "main"], ["add", "-A"], ["commit", "-q", "-m", "fixture corpus"]):
            subprocess.run(["git", *args], cwd=tmp, env=env, check=True)
        return subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=tmp, env=env).strip()


def unrust(blob):
    return blob.replace(b"\\n", b"\n").replace(b'\\"', b'"').replace(b"\\\\", b"\\")


def rust_lines(blob, indent):
    return b"".join(indent + b'"' + line.replace(b"\\", b"\\\\").replace(b'"', b'\\"').replace(b"\n", b"\\n")
                    + b'",\n' for line in blob.splitlines(keepends=True))


def concat_block(src, opener):
    at = src.index(opener)
    match = re.search(rb"concat!\(\n((?:\s*\"(?:[^\"\\]|\\.)*\",\n)+)\s*\)", src[at:])
    assert match is not None, opener
    literals = LIT.findall(match.group(1))
    return at + match.start(1), at + match.end(1), b"".join(unrust(s) for _, s in literals), literals[0][0]


def kani_revise(src):
    start, end, old, indent = concat_block(src, b"const DOC")
    new = doc_revise(old)
    if new == old:
        return src
    src = src[:start] + rust_lines(new, indent) + src[end:]
    n = len(new)
    src = re.sub(rb"#\[kani::unwind\(\d+\)\](\n    fn (?:reader_trace_check_small|engine_answer_tiny)\b)",
                 lambda m: b"#[kani::unwind(%d)]" % (n + 1) + m.group(1), src)
    src = re.sub(rb"DOC has \d+ bytes", b"DOC has %d bytes" % n, src)
    return re.sub(rb"DOC\.len\(\) = \d+; bound = \d+", b"DOC.len() = %d; bound = %d" % (n, n), src)


def phase_a(root):
    data = {p: (root / p).read_bytes() for p in tracked(root, *SCOPE)}
    out = {}
    malformed = []
    documents = []
    for p, blob in data.items():
        if p == KANI:
            new = kani_revise(blob)
        else:
            template = p.endswith(".tpl.in")
            body = blob.split(b"\n", 1)[1] if template else blob
            new = doc_revise(body)
            if new != body:
                documents.append(p)
            elif b":- multifile(guideline_" in body and OLD_BLOCK not in body:
                malformed.append(p)
            if template:
                assert blob == template_pin(blob), f"stale template pin: {p}"
                new = template_pin(blob.split(b"\n", 1)[0] + b"\n" + new)
        if new != blob:
            out[p] = new

    def cur(p):
        return out.get(p, data[p])

    # A certify mutant's swap-lines operands index lines of its target document: a
    # migrated target gains the added lines after the anchor, so operands past it shift.
    certify = "tests/certify/cases.tsv"
    if certify in data:
        rows = cur(certify).split(b"\n")
        for i, row in enumerate(rows):
            f = row.split(b"\t")
            if len(f) > 5 and f[3] == b"swap-lines" and f[2].decode() in documents:
                body = data[f[2].decode()]
                cut = body[:body.find(OLD_BLOCK) + len(CUT)].count(b"\n")
                f[4], f[5] = (b"%d" % (int(x) + ADDED.count(b"\n")) if int(x) >= cut else x
                              for x in (f[4], f[5]))
                rows[i] = b"\t".join(f)
        if b"\n".join(rows) != data[certify]:
            out[certify] = b"\n".join(rows)

    def bind(mapping, old, new):
        if old != new:
            previous = mapping.get(old)
            mapping[old] = new
            if previous is not None and previous != new:
                mapping[previous] = new

    fixtures = {}
    for case in sorted((root / "tests/ui").glob("*/*/worktree")):
        prefix = case.parent.relative_to(root).as_posix() + "/tree/"
        fixtures[prefix] = [p for p in data if p.startswith(prefix)]
    old_commits = {}
    current_commits = {}
    rebind = {}
    commits = {}
    while True:
        for p in data:
            if p == KANI:
                continue
            bind(rebind, sha(data[p]), sha(cur(p)))
            if p.endswith(".tpl.in"):
                bind(rebind, sha(expand(data[p])), sha(expand(cur(p))))
        for prefix, paths in fixtures.items():
            if not any(cur(p) != data[p] for p in paths):
                continue
            before = {p[len(prefix):]: data[p] for p in paths}
            after = {p[len(prefix):]: cur(p) for p in paths}
            if prefix not in old_commits:
                old_commits[prefix] = fixture_commit(before)
            if current_commits.get(prefix, (None,))[0] != after:
                current_commits[prefix] = (after, fixture_commit(after))
            bind(commits, old_commits[prefix], current_commits[prefix][1])
        changed = False
        for p in data:
            if p == KANI:
                continue
            blob = cur(p)
            new = TERM.sub(lambda m: atom(rebind[m.group(2)]) if m.group(2) in rebind else m.group(), blob)
            new = HEX.sub(lambda m: rebind.get(m.group(), m.group()), new)
            new = SHA1.sub(lambda m: commits.get(m.group(), m.group()), new)
            if p.endswith(".tpl.in"):
                new = template_pin(new)
            if new != blob:
                out[p] = new
                changed = True
        if not changed:
            break
    return data, out, malformed, documents, rebind, commits


def run(argv, cwd, stdin=b"", env=None, timeout=600):
    result = subprocess.run(argv, cwd=cwd, input=stdin, capture_output=True, env=env, timeout=timeout)
    return result.returncode, result.stdout, result.stderr


def materialize(src, dst, top=True):
    for path in sorted(src.iterdir()):
        if path.is_dir():
            if not (top and path.name == "expect"):
                materialize(path, dst / path.name, False)
            continue
        dst.mkdir(parents=True, exist_ok=True)
        blob = path.read_bytes()
        assert path.name.endswith(".in"), path
        if path.name.endswith(".tpl.in"):
            assert blob == template_pin(blob), path
            (dst / path.name[:-len(".tpl.in")]).write_bytes(expand(blob))
        else:
            (dst / path.name[:-len(".in")]).write_bytes(blob)


def phase_b(root, ckc):
    before = {p: (root / p).read_bytes() for p in tracked(root, *SCOPE)}

    def put(rel, blob):
        assert rel in before or (SEMANTIC and rel == SCHEMA), ("outside tracked output census", rel)
        target = root / rel
        if not target.is_file() or target.read_bytes() != blob:
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(blob)

    # v1_suite.rs records relative operands in its diagnostics.
    layout = ".scratch/m5u2/suite/cases"
    v1 = root / "tests/v1"
    with tempfile.TemporaryDirectory() as tmp:
        private = Path(tmp)
        lines = (v1 / "cases.tsv").read_text().splitlines(keepends=True)
        for case in dict.fromkeys(r.split("\t")[0] for r in lines if not r.startswith("#")):
            materialize(v1 / case, private / layout / case)
        new_lines = []
        for row in lines:
            if row.startswith("#"):
                new_lines.append(row)
                continue
            case, probe, mode, expected_rc, expected_n, args = row.rstrip("\n").split("\t")
            argv = [str(ckc), "v1", mode] + [f"{layout}/{case}/{a}" for a in args.split(" ") if a]
            rc, so, se = run(argv, private)
            assert rc == int(expected_rc), (case, probe, "changed verdict", expected_rc, rc, se)
            put(f"tests/v1/{case}/expect/{probe}.stdout", so)
            put(f"tests/v1/{case}/expect/{probe}.stderr", se)
            n = se.count(b"\n") + (1 if se and not se.endswith(b"\n") else 0)
            new_lines.append("\t".join([case, probe, mode, expected_rc,
                                        expected_n if expected_n == "-" else str(n), args]) + "\n")
        put("tests/v1/cases.tsv", "".join(new_lines).encode())

    rc, _, se = run([sys.executable, "-I", str(root / ".agent/archive/generators/probe-pins"), str(root)], root)
    assert rc == 0, se

    ui_env = dict(os.environ, GIT_OPTIONAL_LOCKS="0", GIT_CONFIG_GLOBAL="/dev/null", GIT_CONFIG_SYSTEM="/dev/null")
    ui_cases = sorted({"/".join(p.split("/")[:4]) for p, blob in before.items()
                       if p.startswith("tests/ui/") and p.split("/")[4:5] == ["tree"] and NEW_BLOCK in blob})
    for case in ui_cases:
        source = root / case
        assert not any((source / name).exists() for name in ("tree-order.txt", "empty-dirs.txt", "worktree")), case
        with tempfile.TemporaryDirectory() as tmp:
            scratch = Path(tmp)
            fixture = scratch / case / "tree"
            shutil.copytree(source / "tree", fixture)
            bare = scratch / "bare"
            shutil.copytree(source / "tree", bare)
            for guideline in sorted((bare / "guidelines").iterdir()):
                if (guideline / "audit/review-manifest.tsv").is_file():
                    # Review bundles exclude the table; preserve a red table's defect.
                    (guideline / "vocabulary.tsv").unlink(missing_ok=True)
                    rc, _, se = run([str(ckc), "review-manifest", guideline.name], bare)
                    assert rc == 0, (case, se)
                    manifest = (guideline / "audit/review-manifest.tsv").read_bytes()
                    put(f"{case}/tree/guidelines/{guideline.name}/audit/review-manifest.tsv", manifest)
                    (fixture / "guidelines" / guideline.name / "audit/review-manifest.tsv").write_bytes(manifest)
            for n, row in enumerate((source / "case.tsv").read_text().splitlines()):
                if not row or row.startswith("#"):
                    continue
                argv, expected_rc, err_pin, out_pin = row.split("\t")
                dest = scratch / f"out-{n}"
                args = [{"@TREE@": f"{case}/tree", "@OUT@": str(dest)}.get(a, a) for a in argv.split(" ")]
                rc, so, se = run([str(ckc), "ui", *args], scratch, env=dict(ui_env, TMPDIR=tmp))
                assert rc == int(expected_rc), (case, row, rc, se)
                for pin, blob in ((out_pin, so), (err_pin, se)):
                    if pin != "-":
                        put(f"{case}/{pin}", blob)
                if args[0] == "render" and case.startswith("tests/ui/green/"):
                    fresh = {p.relative_to(dest).as_posix() for p in dest.rglob("*") if p.is_file()}
                    for p in sorted((source / "golden").rglob("*")):
                        if p.is_file() and p.relative_to(source / "golden").as_posix() not in fresh:
                            p.unlink()
                    for rel in sorted(fresh):
                        put(f"{case}/golden/{rel}", (dest / rel).read_bytes())

    # A recompiled case's answer + trace goldens follow its tree where they mirrored it.
    for case in RECOMPILE:
        source = root / "tests/queries" / case
        with tempfile.TemporaryDirectory() as tmp:
            stage = Path(tmp)
            (stage / "vendor").symlink_to(root / "vendor")
            (stage / "rust/ckc").mkdir(parents=True)
            (stage / "rust/ckc/prolog").symlink_to(root / "rust/ckc/prolog")
            shutil.copytree(source / "tree/guidelines", stage / "guidelines")
            (gid,) = [p.name for p in (stage / "guidelines").iterdir()]
            for command in ("compile", "queries"):
                rc, _, se = run([str(ckc), command, gid], stage)
                assert rc == 0, (case, command, se)
            tree = f"tests/queries/{case}/tree/guidelines/{gid}"
            fresh = stage / "guidelines" / gid
            for path in sorted(p for p in fresh.rglob("*") if p.is_file()):
                put(f"{tree}/{path.relative_to(fresh).as_posix()}", path.read_bytes())
            for lane, kind in (("answers-golden", "answers"), ("traces-golden", "traces")):
                for golden in sorted((source / lane).glob("*.pl")):
                    pin, mirrored = f"tests/queries/{case}/{lane}/{golden.name}", f"{tree}/queries/{kind}/{golden.name}"
                    if before[pin] == before[mirrored]:
                        put(pin, (root / mirrored).read_bytes())
        print(f"{LABEL} queries fixture: {case} recompiled", flush=True)

    corpus = {p: (root / p).read_bytes() for p in tracked(root, "guidelines", others=True)}
    for guideline in sorted((root / "guidelines").iterdir()):
        if guideline.is_dir():
            for command in ("compile", "queries", "review-manifest"):
                rc, _, se = run([str(ckc), command, guideline.name], root, timeout=7200)
                assert rc == 0, (command, guideline.name, se)
                fresh = {p: (root / p).read_bytes() for p in tracked(root, "guidelines", others=True)}
                drift = sorted(p for p in corpus.keys() | fresh.keys() if corpus.get(p) != fresh.get(p))
                if SEMANTIC:
                    print(f"{LABEL} corpus: {command} {guideline.name} {len(drift)} files drift (oracle grades)", flush=True)
                    corpus = fresh
                    continue
                assert not drift, (command, guideline.name, "changed Phase A corpus bytes", drift)
                print(f"{LABEL} corpus: {command} {guideline.name} byte-identical ({len(corpus)} files)", flush=True)
    rc, _, se = run(["bash", str(root / ".agent/archive/generators/certify-one-fixtures"), str(root), str(ckc)], root)
    assert rc == 0, se

    after = {p: (root / p).read_bytes() for p in tracked(root, *SCOPE, others=True)}
    added = after.keys() - before.keys()
    assert after.keys() >= before.keys() and added <= ({SCHEMA} if SEMANTIC else set()), (
        "changed output census", sorted(after.keys() ^ before.keys()))
    regenerated = sorted(p for p in before if before[p] != after[p])
    digest = hashlib.sha256()
    for p, blob in sorted(after.items()):
        digest.update(p.encode() + b"\0" + blob + b"\0")
    print(f"{LABEL} phase B: {len(regenerated)} files regenerated; scope tree sha256 {digest.hexdigest()}")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("repo_root", type=Path)
    parser.add_argument("ckc_binary", type=Path)
    parser.add_argument("--add", action="append", required=True, type=indicator, metavar="NAME/ARITY")
    parser.add_argument("--after", required=True, type=indicator, metavar="NAME/ARITY")
    parser.add_argument("--frozen", action="append", default=[], metavar="PREFIX")
    parser.add_argument("--label", default="revise")
    parser.add_argument("--dry", action="store_true")
    parser.add_argument("--semantic", action="store_true")
    parser.add_argument("--recompile", action="append", default=[], metavar="QUERY-CASE")
    args = parser.parse_args()
    global SEMANTIC, RECOMPILE
    SEMANTIC, RECOMPILE = args.semantic, tuple(args.recompile)
    if RECOMPILE and not SEMANTIC:
        parser.error("--recompile needs --semantic")
    root, ckc = args.repo_root.resolve(), args.ckc_binary.resolve()
    configure(root, args.add, args.after, args.frozen, args.label)
    if not args.dry and (not ckc.is_file() or not os.access(ckc, os.X_OK)):
        parser.error(f"ckc binary is not executable: {ckc}")
    data, out, malformed, documents, rebind, commits = phase_a(root)
    for p in malformed:
        print(f"kept malformed block {p}")
    for p, blob in sorted(out.items()):
        if args.dry:
            print(f"would change {p}")
        else:
            (root / p).write_bytes(blob)
    label = " (dry)" if args.dry else ""
    print(f"{LABEL} phase A{label}: {len(data)} files scanned, {len(documents)} declaration blocks extended, "
          f"{len(out)} {'would write' if args.dry else 'written'}, {len(rebind)} digests rebound, "
          f"{len(commits)} fixture commits rebound, {len(malformed)} malformed blocks kept")
    if not args.dry:
        phase_b(root, ckc)


if __name__ == "__main__":
    main()
