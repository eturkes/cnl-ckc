#!/usr/bin/env python3
"""REV-K byte-preservation oracle; run with the pre-fix binary + repository root."""
import hashlib
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile

binary = Path(sys.argv[1]).resolve()
root = Path(sys.argv[2]).resolve()
assert binary.is_file()
pins = {}
parent = root / ".scratch/rev-k"
parent.mkdir(parents=True, exist_ok=True)


def run(scratch, *args):
    result = subprocess.run(
        [str(binary), *map(str, args)],
        cwd=scratch,
        env={**os.environ, "GIT_CEILING_DIRECTORIES": str(scratch),
             "GIT_OPTIONAL_LOCKS": "0", "TMPDIR": str(scratch)},
        capture_output=True,
    )
    assert result.returncode == 0 and not result.stderr, (args, result.returncode, result.stderr)
    return result.stdout


def copy_tree(scratch, case):
    source = root / "tests/ui/green" / case / "tree"
    tree = scratch / "tree"
    shutil.copytree(source, tree)
    empty = source.parent / "empty-dirs.txt"
    if empty.exists():
        for name in empty.read_text().splitlines():
            (tree / name).mkdir(parents=True, exist_ok=True)
    return tree


def pin(name, body):
    assert name not in pins
    style = body.split(b"<style>\n", 1)[1].split(b"</style>", 1)[0]
    assert style == (root / "tests/rev-k/style.before.in").read_bytes(), "pre-fix stylesheet required"
    pins[name] = hashlib.sha256(body).hexdigest()


def get(scratch, tree, route):
    raw = run(scratch, "ui", "request", "GET", route, tree)
    headers, body = raw.split(b"\n\n", 1)
    assert headers.splitlines()[0] == b"HTTP 200", (route, body.split(b"<main", 1)[-1])
    return body


def tally_tree(scratch, n, k, m, pl):
    assert m <= min(n, k) and (m == 0) == (n == 0)
    tree = copy_tree(scratch, "header-only-ledger")
    guideline = tree / "guidelines/alpha"
    ace = (guideline / "ace/a.ace").read_text()
    for directory in ["ace", "pl"]:
        shutil.rmtree(guideline / directory)
        (guideline / directory).mkdir()
    coverage = "".join(line + "\n" for line in (guideline / "coverage.tsv").read_text().splitlines()
                       if line.startswith("#"))
    evidence = f"Fixture evidence\nRegion authority: identify the {k} payloads below\n"
    old_manifest = (guideline / "audit/review-manifest.tsv").read_text().splitlines()
    manifest = "".join(line + "\n" for line in old_manifest if line.startswith("#"))
    digest_fields = next(line.split("\t", 1)[1] for line in old_manifest if not line.startswith("#"))
    for i in range(1, k + 1):
        docid = f"d{i}"
        (guideline / f"ace/{docid}.ace").write_text(ace)
        (guideline / f"pl/{docid}.pl").write_text(pl)
        coverage += f"r{i}\tsource/evidence.txt\tp{i}\tFixture section\tace({docid})\n"
        evidence += f"\n[r{i} | p{i} | Fixture section]\nSource payload for {docid}.\n"
        parts = digest_fields.split("\t")[:4]
        block = f"bundle v2 {docid}\n" + "".join(f"{key} {value}\n" for key, value in
                                                    zip(["ace", "coverage", "payload", "clauses"], parts))
        manifest += f"{docid}\t" + "\t".join(parts) + "\t" + hashlib.sha256(block.encode()).hexdigest() + "\n"
    manifest = ("".join(line + "\n" for line in old_manifest if line.startswith("#"))
                + "".join(line + "\n" for line in sorted(manifest.splitlines()) if not line.startswith("#")))
    ledger = "# format: docid<TAB>review_sha256<TAB>ace_commit<TAB>verdict<TAB>reviewer<TAB>date<TAB>comment\n"
    for i in range(n):
        ledger += f"d{i % m + 1}\t{'0' * 64}\t\tapproved\tReader {i + 1}\t2026-01-01T00:00:00Z\tCount control.\n"
    ledger = "\n".join(sorted(ledger.splitlines())) + "\n"
    for path, content in [("coverage.tsv", coverage), ("source/evidence.txt", evidence),
                          ("audit/review-manifest.tsv", manifest), ("audit/adjudication.tsv", ledger)]:
        (guideline / path).write_text(content)
    return tree


for case in sorted((root / "tests/ui/green").iterdir()):
    if not case.is_dir():
        continue
    with tempfile.TemporaryDirectory(dir=parent) as temp:
        scratch = Path(temp)
        tree = copy_tree(scratch, case.name)
        output = scratch / "pages"
        run(scratch, "ui", "render", output, tree)
        pages = sorted(output.rglob("*.html"))
        assert pages
        for page in pages:
            pin(f"fixture/{case.name}/{page.relative_to(output)}", page.read_bytes())

for n in [0, 1, 2, 3, 10, 11, 21]:
    for k in [1, 2, 3, 11]:
        for m in [0] if n == 0 else range(1, min(n, k) + 1):
            with tempfile.TemporaryDirectory(dir=parent) as temp:
                scratch = Path(temp)
                tree = tally_tree(scratch, n, k, m, "fixture_fact('d1').\n")
                for route in ["index", "records"]:
                    pin(f"tally/n{n}-k{k}-m{m}/{route}",
                        get(scratch, tree, f"/g/alpha/{route}.html"))

for label, pl in [
    ("empty", ""),
    ("one", "fixture_fact('d1').\n"),
    ("two", "fixture_fact('d1').\nfixture_fact('d2').\n"),
    ("one-unterminated", "fixture_fact('d1')."),
    ("two-unterminated", "fixture_fact('d1').\nfixture_fact('d2')."),
    ("one-blank", "\n"),
    ("two-blank", "\n\n"),
]:
    with tempfile.TemporaryDirectory(dir=parent) as temp:
        scratch = Path(temp)
        tree = copy_tree(scratch, "header-only-ledger")
        (tree / "guidelines/alpha/pl/a.pl").write_text(pl)
        pin(f"line/{label}", get(scratch, tree, "/g/alpha/doc/a.html"))

output = root / "tests/rev-k/baseline.sha256"
output.write_text("# Pre-REV-K UI bodies; regenerate with pin-baseline.py <pre-fix-ckc> <root>.\n"
                  + "".join(f"{digest}  {name}\n" for name, digest in sorted(pins.items())))
print(f"REV-K baseline: {len(pins)} bodies")
