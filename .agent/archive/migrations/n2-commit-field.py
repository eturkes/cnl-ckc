#!/usr/bin/env python3
# N2 fixture re-pin (contract n2 P7), idempotent, derived from the contract alone:
# - every form-shaped `ui request POST` body (5..=32 pairs, no `commit` pair) gains `&commit=<v>`
#   (empty, single-pair and >32-pair bodies keep their first refusal without the field);
# - every document page golden gains `<input type="hidden" name="commit" value="<v>">`
#   right after its ledger_sha256 input.
# v = the row's `--commit` value | the git carrier's fixture HEAD | empty (filesystem mode).
# Carrier HEAD = ui_fixtures.rs Fixture::new replayed (tree/ committed with fixed identities).
# Usage (repo root): python3 .agent/archive/migrations/n2-commit-field.py
import os
import pathlib
import re
import subprocess
import tempfile

ROOT = pathlib.Path(
    subprocess.check_output(["git", "rev-parse", "--show-toplevel"], text=True).strip()
)
UI = ROOT / "tests/ui"
LEDGER = b'<input type="hidden" name="ledger_sha256" value="'
COMMIT = b'<input type="hidden" name="commit" value="'
HEX40 = re.compile(r"[0-9a-f]{40}")
GIT_ENV = {
    **os.environ,
    "GIT_CONFIG_GLOBAL": "/dev/null",
    "GIT_CONFIG_SYSTEM": "/dev/null",
    "GIT_DEFAULT_HASH": "sha1",
    "GIT_AUTHOR_NAME": "fixture",
    "GIT_AUTHOR_EMAIL": "fixture@localhost",
    "GIT_COMMITTER_NAME": "fixture",
    "GIT_COMMITTER_EMAIL": "fixture@localhost",
    "GIT_AUTHOR_DATE": "2026-01-01T00:00:00+00:00",
    "GIT_COMMITTER_DATE": "2026-01-01T00:00:00+00:00",
}


def carrier_head(case):
    with tempfile.TemporaryDirectory() as tmp:
        tree = pathlib.Path(tmp) / "tree"
        for src in sorted((case / "tree").rglob("*")):
            if src.is_file():
                dst = tree / src.relative_to(case / "tree")
                dst.parent.mkdir(parents=True, exist_ok=True)
                dst.write_bytes(src.read_bytes())
        for args in (["init", "-q", "-b", "main"], ["add", "-A"],
                     ["commit", "-q", "-m", "fixture corpus"]):
            subprocess.run(["git", *args], cwd=tree, env=GIT_ENV, check=True)
        return subprocess.check_output(
            ["git", "rev-parse", "HEAD"], cwd=tree, env=GIT_ENV, text=True
        ).strip()


def flag(tokens, name):
    if name in tokens and tokens.index(name) + 1 < len(tokens):
        return tokens[tokens.index(name) + 1]
    return None


def page(path, value):
    data = path.read_bytes()
    lines = data.split(b"\n")
    at = [i for i, line in enumerate(lines) if line.startswith(LEDGER)]
    if not at:
        return False
    assert len(at) == 1, path
    i = at[0]
    if i + 1 < len(lines) and lines[i + 1].startswith(COMMIT):
        return False
    lines.insert(i + 1, COMMIT + value.encode() + b'">')
    path.write_bytes(b"\n".join(lines))
    return True


bodies = pages = 0
for case in sorted(p.parent for p in UI.glob("*/*/case.tsv")):
    head = carrier_head(case) if (case / "worktree").is_dir() else ""
    for ledger in (case / "after").glob("guidelines/*/audit/adjudication.tsv") if head else ():
        recorded = {r.split("\t")[2] for r in ledger.read_text().split("\n")[1:] if r}
        assert recorded <= {"", head}, (case, recorded, head)
    out = []
    for row in (case / "case.tsv").read_bytes().decode().split("\n"):
        cols = row.split("\t")
        if row.startswith("#") or len(cols) != 4:
            out.append(row)
            continue
        tokens = cols[0].split(" ")
        commit = flag(tokens, "--commit")
        value = commit if commit and HEX40.fullmatch(commit) else head
        if tokens[:2] == ["request", "POST"] and "--body" in tokens:
            i = tokens.index("--body") + 1
            body = tokens[i] if i < len(tokens) else ""
            pairs = body.split("&")
            if 5 <= len(pairs) <= 32 and not any(p.split("=", 1)[0] == "commit" for p in pairs):
                tokens[i] = body + "&commit=" + value
                bodies += 1
        cols[0] = " ".join(tokens)
        if tokens[:2] == ["request", "GET"] and len(tokens) > 2 and "/doc/" in tokens[2] \
                and cols[3] != "-":
            pages += page(case / cols[3], value)
        if tokens[0] == "render":
            for doc in sorted((case / "golden").glob("g/*/doc/*.html")):
                pages += page(doc, head)
        out.append("\t".join(cols))
    (case / "case.tsv").write_bytes("\n".join(out).encode())
print(f"n2-commit-field: bodies={bodies} pages={pages}")
