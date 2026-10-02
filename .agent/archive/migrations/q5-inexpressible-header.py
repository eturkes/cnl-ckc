#!/usr/bin/env python3
"""Queue row Q5 (M7, user ruling R5a): the coverage grammar gains the uncovered class
`inexpressible`, so the pinned third header line of every tracked coverage ledger
(`coverage.tsv`, fixture `coverage.tsv.in`) appends ` | inexpressible`. Only that line
changes. A git-backed UI fixture commits its `tree/` (fixed identity + date, see
rust/ckc/tests/ui_fixtures.rs), so its commit id follows its bytes: the one pin that
embeds it (git-uncommitted-edit `after/` ledger, POST row ace_commit) is re-derived.
Usage: python3 -P <this> <repo-root>; idempotent.
"""
import subprocess, sys
from pathlib import Path
root = Path(sys.argv[1]).resolve()
OLD = b"# uncovered classes: heading | process | external | aim | descriptive | notice\n"
NEW = b"# uncovered classes: heading | process | external | aim | descriptive | notice | inexpressible\n"
files = subprocess.run(["git", "-C", str(root), "ls-files", "-z"], capture_output=True, check=True).stdout.split(b"\0")
changed = 0
for f in files:
    if not (f.endswith(b"coverage.tsv") or f.endswith(b"coverage.tsv.in")):
        continue
    p = root / f.decode()
    data = p.read_bytes()
    if OLD in data:
        assert data.count(OLD) == 1, p
        p.write_bytes(data.replace(OLD, NEW))
        changed += 1
print(f"q5: {changed} coverage ledgers re-headed")

import os, shutil, tempfile
ENV = dict(os.environ, GIT_CONFIG_GLOBAL="/dev/null", GIT_CONFIG_SYSTEM="/dev/null", GIT_DEFAULT_HASH="sha1",
           GIT_AUTHOR_NAME="fixture", GIT_AUTHOR_EMAIL="fixture@localhost", GIT_COMMITTER_NAME="fixture",
           GIT_COMMITTER_EMAIL="fixture@localhost", GIT_AUTHOR_DATE="2026-01-01T00:00:00+00:00",
           GIT_COMMITTER_DATE="2026-01-01T00:00:00+00:00")


def fixture_commit(tree):
    with tempfile.TemporaryDirectory() as t:
        dst = Path(t) / "tree"
        shutil.copytree(tree, dst, symlinks=True)
        for args in (["init", "-q", "-b", "main"], ["add", "-A"], ["commit", "-q", "-m", "fixture corpus"]):
            subprocess.run(["git", *args], cwd=dst, env=ENV, check=True, capture_output=True)
        return subprocess.run(["git", "rev-parse", "HEAD"], cwd=dst, env=ENV, check=True, capture_output=True).stdout.strip()


case = root / "tests/ui/green/git-uncommitted-edit"
ledger = case / "after/guidelines/alpha/audit/adjudication.tsv"
rows = ledger.read_bytes().split(b"\n")
new = fixture_commit(case / "tree")
for i, row in enumerate(rows):
    f = row.split(b"\t")
    if len(f) == 7 and len(f[2]) == 40:
        f[2] = new
        rows[i] = b"\t".join(f)
ledger.write_bytes(b"\n".join(rows))
print(f"q5: git-uncommitted-edit fixture commit = {new.decode()}")
