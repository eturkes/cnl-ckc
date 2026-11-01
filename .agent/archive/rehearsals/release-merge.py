#!/usr/bin/env python3
"""Release-merge rehearsal (contract `mrg`): a schema-1 release line merged into the schema-2 main
inside a disposable clone, by the merge runbook of docs/REFERENCE.md § Releasing (step 14), then
the D5 asserts on the real merge + one planted wrong resolution per assert.

Usage: python3 -I .agent/archive/rehearsals/release-merge.py <repo-root>
rc 0 = every assert holds on the merge and fires on its plant; rc 1 = a named assert failed (or a
plant slipped through); rc 2 = usage or setup failure (a step exited nonzero, a conflict outside
the runbook's classes).

Scope: a schema + custody rehearsal over frozen refs, not a replay of the real line tip (its
publication + tooling commits stay out). L0 = the cut e4c26978 (identifier 1), built by the
line's own schema-1 `ckc` (the L0 source tree via `git archive`, cached under
.scratch/mrg/l0-<sha>/); M0 = the repo root's HEAD (identifier 2, clean tree) + its release
build. The line gains a ledger batch (document A approved at the line commit the reviewer
viewed) and a correction round (documents B + D corrected; C corrected differently from main's
own edit of C); main gains its edit of C. Heavy steps (both compiles, certify, check) run once;
each plant reruns the cheap asserts on a clone of the merge.
"""
import hashlib
import os
import shutil
import subprocess
import sys
from pathlib import Path

USAGE = "usage: release-merge.py <repo-root>"
L0 = "e4c26978"
GID = "cdc-2022-opioid"
A = "cdc2022-opioid-rec01"
B, C, D = "cdc2022-opioid-rec11-imp07", "cdc2022-opioid-s53-12", "cdc2022-opioid-rec05-imp14"
RULING_C = "line"  # the scripted stand-in for the user's semantic ruling on a conflicting ACE edit
LEDGER = f"guidelines/{GID}/audit/adjudication.tsv"
MANIFEST = f"guidelines/{GID}/audit/review-manifest.tsv"
HEADER = "# format: docid<TAB>review_sha256<TAB>ace_commit<TAB>verdict<TAB>reviewer<TAB>date<TAB>comment\n"
PLANTS = {  # plant → the assert it must trip
    "dropped-correction": "corrections-present",
    "overturned-ruling": "conflict-ruling",
    "product-at-1": "identifiers-2",
    "rewritten-ledger-row": "ledger-immutable",
    "one-parent-merge": "two-parents",
    "moved-line-ref": "line-unchanged",
}


class Setup(Exception):
    pass


def run(args, cwd, env, ok=(0,)):
    try:
        r = subprocess.run([str(a) for a in args], cwd=cwd, env=env, capture_output=True, text=True)
    except OSError as e:
        raise Setup(f"{' '.join(map(str, args))} (cwd {cwd}): {e}") from e
    if r.returncode not in ok:
        raise Setup(f"{' '.join(map(str, args))} (cwd {cwd}) rc {r.returncode}: {r.stderr.strip()[-400:]}")
    return r


def git(cwd, env, *args, ok=(0,)):
    return run(["git", *args], cwd, env, ok).stdout.strip()


def show(repo, env, rev, path):
    """A file's committed bytes, unstripped: a ledger row ends in its (empty) comment's tab."""
    return run(["git", "show", f"{rev}:{path}"], repo, env).stdout


def environment(root):
    t = Path(os.environ.get("CKC_TOOLCHAIN") or root / ".toolchain")
    env = dict(os.environ, CKC_TOOLCHAIN=str(t), RUSTUP_HOME=str(t / "rustup"), CARGO_HOME=str(t / "cargo"),
               GIT_CONFIG_GLOBAL="/dev/null", GIT_CONFIG_SYSTEM="/dev/null",
               GIT_CEILING_DIRECTORIES=str(root / ".scratch"))
    env["PATH"] = f"{t}/verus-x86-linux:{t}/cargo/bin:{t}/bin:{env.get('PATH', '')}"
    env.setdefault("CKC_JOBS", "4")  # two corpus compiles + certify + check: half the cores keeps memory modest
    return env


def line_binary(root, env):
    """The line's own compiler: L0's source tree built once per L0 commit."""
    sha = git(root, env, "rev-parse", f"{L0}^{{commit}}")
    cache = root / ".scratch/mrg" / f"l0-{sha}"
    binary = cache / "tree/rust/target/release/ckc"
    if not (cache / "built").is_file():
        shutil.rmtree(cache, ignore_errors=True)
        (cache / "tree").mkdir(parents=True)
        archive = subprocess.run(["git", "archive", sha], cwd=root, env=env, capture_output=True, check=True).stdout
        subprocess.run(["tar", "-x", "-C", str(cache / "tree")], input=archive, check=True)
        run(["cargo", "build", "--release", "--locked", "--offline", "-p", "ckc"], cache / "tree/rust", env)
        (cache / "built").write_text(sha + "\n")
    return binary


def main_binary(root, env):
    if git(root, env, "status", "--porcelain", "--untracked-files=no"):
        raise Setup("the repo root has uncommitted changes: M0 = a committed HEAD")
    run(["cargo", "build", "--release", "--locked", "--offline", "-p", "ckc"], root / "rust", env)
    return root / "rust/target/release/ckc"


def ace(doc):
    return f"guidelines/{GID}/ace/{doc}.ace"


def correct(repo, doc, spaces):
    """A correction that keeps every alignment anchor: whitespace before the final period."""
    path = repo / ace(doc)
    text = path.read_text(encoding="utf-8")
    rows = [r.split("\t") for r in (repo / f"guidelines/{GID}/align/{doc}.tsv").read_text(encoding="utf-8").splitlines() if r]
    end = max((int(r[2]) + len(r[3]) for r in rows if r[1] == "ace"), default=0)
    if not text.endswith(".\n") or end > len(text) - 2:
        raise Setup(f"{doc}: the final period does not follow every ACE anchor")
    path.write_text(text[:-2] + " " * spaces + ".\n", encoding="utf-8")


def review_digest(manifest_text, doc):
    for row in manifest_text.splitlines():
        f = row.split("\t")
        if f[0] == doc and len(f) == 6:
            return f[5]
    raise Setup(f"{doc}: no review-manifest row")


def refresh(repo, env, ckc):
    """Products of every guideline the merge or the correction touches, then the commit."""
    for command in ("compile", "queries", "review-manifest"):
        run([ckc, command, GID], repo, env)


def amend_manifest(repo, env, ckc):
    run([ckc, "release-manifest"], repo, env)
    git(repo, env, "add", "release-manifest.tsv")
    git(repo, env, "commit", "-q", "--amend", "--no-edit")


def archive_hash(repo, env, ckc, ref, scratch):
    """`ckc dist build` at `ref` with the line's binary: the sha256 of the archive bytes."""
    tree = scratch / f"dist-{ref[:12]}-{len(list(scratch.glob('dist-*')))}"
    git(repo, env, "worktree", "add", "-q", "--detach", str(tree), ref)
    try:
        dest = tree / ".dist-out"
        run([ckc, "dist", "build", dest], tree, env)
        (archive,) = sorted(dest.glob("*.tar.gz"))
        return hashlib.sha256(archive.read_bytes()).hexdigest()
    finally:
        git(repo, env, "worktree", "remove", "--force", str(tree))


def asserts(repo, env, line_ckc, refs, scratch):
    """D5's cheap asserts on HEAD; returns the names of the failed ones."""
    failed = []
    head = git(repo, env, "rev-parse", "HEAD")
    parents = git(repo, env, "rev-list", "--parents", "-n", "1", head).split()[1:]
    if parents != [refs["main"], refs["line"]]:
        failed.append("two-parents")
    if git(repo, env, "rev-parse", "line") != refs["line"] or \
            archive_hash(repo, env, line_ckc, git(repo, env, "rev-parse", "line"), scratch) != refs["archive"]:
        failed.append("line-unchanged")
    bad = []
    for p in git(repo, env, "ls-files", f"guidelines/{GID}/pl", f"guidelines/{GID}/queries/pl").splitlines():
        if not p.endswith(".pl"):
            continue
        lines = (repo / p).read_text(encoding="utf-8").splitlines()
        if "/queries/pl/" in p:
            tags = [l for l in lines if l.startswith("'$guideline_query'(")]
            good = len(tags) == 1 and tags[0].startswith("'$guideline_query'(v2,")
        else:
            good = [l for l in lines if l.startswith("guideline_schema_version(")] == ["guideline_schema_version(2)."]
        if not good:
            bad.append(p)
    if bad:
        failed.append("identifiers-2")
    if any((repo / ace(doc)).read_text(encoding="utf-8") != show(repo, env, refs["line"], ace(doc)) for doc in (B, D)):
        failed.append("corrections-present")
    ruled = show(repo, env, refs["line" if RULING_C == "line" else "main"], ace(C))
    recorded = f"{ace(C)} keeps the {RULING_C} text." in git(repo, env, "log", "-1", "--format=%B", head)
    if (repo / ace(C)).read_text(encoding="utf-8") != ruled or not recorded:
        failed.append("conflict-ruling")
    row = refs["ledger_row"]
    ledger = (repo / LEDGER).read_text(encoding="utf-8") if (repo / LEDGER).is_file() else ""
    viewed = row.split("\t")[2]
    attests = review_digest(show(repo, env, viewed, MANIFEST), A) == row.split("\t")[1]
    ancestor = subprocess.run(["git", "merge-base", "--is-ancestor", viewed, head], cwd=repo, env=env).returncode == 0
    stale = review_digest((repo / MANIFEST).read_text(encoding="utf-8"), A) != row.split("\t")[1]
    if ledger != show(repo, env, refs["line"], LEDGER) or row not in ledger.splitlines() or not (attests and ancestor and stale):
        failed.append("ledger-immutable")
    return failed


def plant(repo, env, name, refs):
    if name == "dropped-correction":
        (repo / ace(D)).write_text(show(repo, env, refs["main"], ace(D)), encoding="utf-8")
    elif name == "overturned-ruling":
        other = "main" if RULING_C == "line" else "line"
        (repo / ace(C)).write_text(show(repo, env, refs[other], ace(C)), encoding="utf-8")
    elif name == "product-at-1":
        p = repo / f"guidelines/{GID}/pl/{B}.pl"
        p.write_text(p.read_text(encoding="utf-8").replace("guideline_schema_version(2).", "guideline_schema_version(1).", 1), encoding="utf-8")
    elif name == "rewritten-ledger-row":
        p = repo / LEDGER
        p.write_text(p.read_text(encoding="utf-8").replace("\trehearsal\t", "\trewritten\t", 1), encoding="utf-8")
    elif name == "moved-line-ref":
        git(repo, env, "update-ref", "refs/heads/line", refs["main"])
        return
    if name == "one-parent-merge":
        tree = git(repo, env, "rev-parse", "HEAD^{tree}")
        message = git(repo, env, "log", "-1", "--format=%B", "HEAD")  # the recorded ruling stays
        squash = git(repo, env, "commit-tree", tree, "-p", refs["main"], "-m", message)
        git(repo, env, "reset", "-q", "--hard", squash)
        return
    git(repo, env, "commit", "-q", "-a", "--amend", "--no-edit")  # the merge keeps both parents


def rehearse(root):
    env = environment(root)
    scratch = root / ".scratch/mrg" / f"run-{os.getpid()}"
    shutil.rmtree(scratch, ignore_errors=True)
    scratch.mkdir(parents=True)
    line_ckc, main_ckc = line_binary(root, env), main_binary(root, env)
    m0 = git(root, env, "rev-parse", "HEAD")
    repo = scratch / "repo"
    run(["git", "clone", "-q", "--no-hardlinks", root, repo], scratch, env)
    git(repo, env, "config", "user.name", "rehearsal")
    git(repo, env, "config", "user.email", "rehearsal@localhost")
    refs = {}
    # The line: a ledger batch, then a correction round, each with the line's binary.
    git(repo, env, "checkout", "-q", "-b", "line", L0)
    viewed = git(repo, env, "rev-parse", "HEAD")
    refs["ledger_row"] = f"{A}\t{review_digest((repo / MANIFEST).read_text(encoding='utf-8'), A)}\t{viewed}\tapproved\trehearsal\t2026-11-01T00:00:00Z\t"
    (repo / LEDGER).write_text(HEADER + refs["ledger_row"] + "\n", encoding="utf-8")
    git(repo, env, "add", LEDGER)
    git(repo, env, "commit", "-q", "-m", "line: ledger batch")
    amend_manifest(repo, env, line_ckc)
    correct(repo, B, 1)
    correct(repo, C, 1)
    correct(repo, D, 1)
    refresh(repo, env, line_ckc)
    git(repo, env, "add", "-A")
    git(repo, env, "commit", "-q", "-m", "line: correction round (B, C, D)")
    amend_manifest(repo, env, line_ckc)
    refs["line"] = git(repo, env, "rev-parse", "HEAD")
    refs["archive"] = archive_hash(repo, env, line_ckc, refs["line"], scratch)
    # Main: its own edit of C.
    git(repo, env, "checkout", "-q", "-b", "mainline", m0)
    correct(repo, C, 2)
    git(repo, env, "commit", "-q", "-am", "main: edit C")
    refs["main"] = git(repo, env, "rev-parse", "HEAD")
    # The runbook (REFERENCE § Releasing step 14), with main's binary.
    run(["git", "merge", "--no-ff", "--no-edit", "line"], repo, env, ok=(0, 1))
    rulings = []
    for p in git(repo, env, "diff", "--name-only", "--diff-filter=U").splitlines():
        if p.startswith(f"guidelines/{GID}/ace/"):
            git(repo, env, "checkout", "--theirs" if RULING_C == "line" else "--ours", "--", p)
            rulings.append(f"ACE ruling (scripted stand-in for the user's): {p} keeps the {RULING_C} text.")
        elif p == "release-manifest.tsv" or p.startswith((f"guidelines/{GID}/pl/", f"guidelines/{GID}/queries/")) \
                or p == MANIFEST:
            git(repo, env, "checkout", "--ours", "--", p)  # regenerated below: neither side's bytes survive
        else:
            raise Setup(f"conflict outside the runbook's classes: {p}")
    git(repo, env, "add", "-A")
    refresh(repo, env, main_ckc)
    git(repo, env, "add", "-A")
    git(repo, env, "commit", "-q", "-m", "Merge release line into main", *[a for r in rulings for a in ("-m", r)])
    amend_manifest(repo, env, main_ckc)
    run([main_ckc, "certify", GID], repo, env)
    run([main_ckc, "check"], repo, env)
    report, rc = [], 0
    failed = asserts(repo, env, line_ckc, refs, scratch)
    report.append(f"merge {git(repo, env, 'rev-parse', '--short', 'HEAD')}: certify + check green; "
                  + (f"FAILED {', '.join(failed)}" if failed else "every assert holds"))
    rc |= bool(failed)
    for name, expected in PLANTS.items():
        copy = scratch / f"plant-{name}"
        run(["git", "clone", "-q", "--no-hardlinks", repo, copy], scratch, env)
        git(copy, env, "config", "user.name", "rehearsal")
        git(copy, env, "config", "user.email", "rehearsal@localhost")
        git(copy, env, "branch", "-q", "line", f"origin/line")
        git(copy, env, "checkout", "-q", "-B", "mainline", "origin/mainline")
        plant(copy, env, name, refs)
        got = asserts(copy, env, line_ckc, refs, scratch)
        fired = expected in got
        rc |= not fired
        report.append(f"plant {name}: {'fires ' + expected if fired else 'SLIPPED (' + expected + ' held)'}; failed = {got}")
    if rc == 0:
        shutil.rmtree(scratch, ignore_errors=True)
    else:
        report.append(f"scratch kept for inspection: {scratch}")
    return rc, report


def main():
    if len(sys.argv) != 2:
        print(USAGE, file=sys.stderr)
        return 2
    try:
        rc, report = rehearse(Path(sys.argv[1]).resolve())
    except Setup as e:
        print(f"release-merge: setup: {e}", file=sys.stderr)
        return 2
    print("\n".join(report))
    print(f"release-merge: {'ok' if rc == 0 else 'FAILED'}")
    return 1 if rc else 0


if __name__ == "__main__":
    sys.exit(main())
