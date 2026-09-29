#!/usr/bin/env python3
"""R99 re-pin (contract .agent/contracts/m5u7.md P7, ruling R99): every retired
tool or mode name and the `goal: ` violation/meter prefix in the corpus and the
fixture trees → its `ckc` form. Values derived from transformed bytes follow:
- sha256 digests of a transformed file (or tests/v1 template expansion), to a
  fixed point; a digest that bound nothing before (a red fixture's deliberate
  stale value) stays unbound; digest term arguments keep canonical atom quoting;
- the commit id of a git-materialized tests/ui fixture (ui_fixtures.rs commits
  tree/ with fixed identities + dates), inside that case;
- a tests/v1 `noncanonical(1,C)` pin whose column lies past the first changed
  byte of its checked file's line 1 shifts by that line's length delta.
Every changed file = those classes alone (asserted). Usage: python3 -P <this> <repo-root>
Record: base = tag `legacy` (39968a58); scope = tracked guidelines/ + tests/
minus tests/strict/ + red.sh (5784 files); input scope sha256 8770d9dc…3de14;
output scope sha256 e6f8f55bec75bb1a85a8c50baa1b95cea0e8584717d68e3f5f397a1eaed221a4
(1597 files changed, 1053 digests rebound, 2 fixture commit ids, 1 column pin);
replay in a fresh `legacy` worktree = the same output digest; rerun on its own
output = 0 changed. Scope digest = sha256 over sorted (relpath NUL bytes NUL).
"""
import hashlib
import os
import re
import subprocess
import sys
import tempfile
from pathlib import Path

root = Path(sys.argv[1]).resolve()
LITERALS = [
    (b"regenerate via tools/goal.py", b"regenerate via ckc compile"),
    (b"by ace_to_pl answer mode", b"by ckc queries"),
    (b"by ace_to_pl trace mode", b"by ckc queries"),
    (b"python3 -P tools/goal.py ", b"ckc "),
    (b"tools/goal.py review-manifest", b"ckc review-manifest"),
    (b"validated by goal.py check", b"validated by ckc check"),
    (b"expected: goal ", b"expected: ckc "),
]
PREFIX = re.compile(rb"(?<![A-Za-z0-9_])goal: ")
HEX = re.compile(rb"(?<![0-9a-f])[0-9a-f]{64}(?![0-9a-f])")
# A digest term argument is a Prolog atom: canonical writers quote it iff it
# starts with a digit, so a rebound digest may gain or lose its quotes.
TERM = re.compile(rb"(?<=sha256\()('?)([0-9a-f]{64})\1(?=\))")
SKIP = re.compile(r"^tests/strict/|(^|/)red\.sh$")


def sha(data):
    return hashlib.sha256(data).hexdigest().encode()


def atom(hexdigest):
    return b"'" + hexdigest + b"'" if hexdigest[:1].isdigit() else hexdigest


def mapped(data):
    for old, new in LITERALS:
        data = data.replace(old, new)
    return PREFIX.sub(b"ckc: ", data)


def tree(n, first, leaf):
    layer = [first] + [leaf] * (n - 1)
    while len(layer) > 1:
        layer = [b"','(" + layer[i] + b"," + layer[i + 1] + b")" if i + 1 < len(layer) else layer[i]
                 for i in range(0, len(layer), 2)]
    return layer[0]


def expand(template):
    """tests/v1 `.tpl.in` expansion (u4-h4-v1 `expand`, pin line not checked)."""
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


SHA1 = re.compile(rb"(?<![0-9a-f])[0-9a-f]{40}(?![0-9a-f])")
COLUMN = re.compile(rb"noncanonical\(1,([0-9]+)\)")
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


paths = [root / p for p in subprocess.run(["git", "ls-files", "-z", "--", "guidelines", "tests"], cwd=root,
                                          capture_output=True, check=True).stdout.decode().split("\0")
         if p and not SKIP.search(p)]
data = {p: p.read_bytes() for p in paths}
rebind = {}                      # old sha256 hex → new sha256 hex
out = {}
for p, old in data.items():
    new = mapped(old)
    if p.name.endswith(".tpl.in"):
        head, rest = new.split(b"\n", 1)
        old_exp, new_exp = expand(old), expand(new)
        assert head == b"@@sha256 " + sha(old_exp), p
        if new_exp != old_exp:
            rebind[sha(old_exp)] = sha(new_exp)
            new = b"@@sha256 " + sha(new_exp) + b"\n" + rest
    if new != old:
        out[p] = new
        rebind[sha(old)] = sha(new)
commits = {}                     # case dir → (old commit id, new commit id)
for case in sorted({p.parents[len(p.relative_to(root / "tests/ui").parts) - 3] for p in paths
                    if p.is_relative_to(root / "tests/ui") and "worktree" in p.relative_to(root / "tests/ui").parts}):
    tree_files = {p: b for p, b in data.items() if p.is_relative_to(case / "tree")}
    old_id = fixture_commit({p.relative_to(case / "tree"): b for p, b in tree_files.items()})
    new_id = fixture_commit({p.relative_to(case / "tree"): out.get(p, b) for p, b in tree_files.items()})
    if old_id != new_id:
        commits[case] = (old_id, new_id)
        for p in paths:
            if p.is_relative_to(case):
                cur = out.get(p, data[p])
                new = SHA1.sub(lambda m: new_id if m.group() == old_id else m.group(), cur)
                if new != cur:
                    out[p] = new
                    rebind[sha(data[p])] = sha(new)
shifted = []
v1 = root / "tests/v1"
for row in (v1 / "cases.tsv").read_bytes().decode().splitlines():
    if row.startswith("#"):
        continue
    case, probe, mode, _rc, _lines, args = row.split("\t")
    if mode not in ("check", "render") or not args:
        continue
    checked = v1 / case / (args.split(" ")[0] + ".in")
    if checked not in data:
        continue
    old_line = data[checked].split(b"\n", 1)[0]
    new_line = out.get(checked, data[checked]).split(b"\n", 1)[0]
    if old_line == new_line:
        continue
    first = next(i for i in range(min(len(old_line), len(new_line)) + 1)
                 if i == min(len(old_line), len(new_line)) or old_line[i] != new_line[i]) + 1
    delta = len(new_line) - len(old_line)
    for stream in ("stdout", "stderr"):
        pin = v1 / case / "expect" / f"{probe}.{stream}"
        cur = out.get(pin, data[pin])
        new = COLUMN.sub(lambda m: b"noncanonical(1,%d)" % (int(m.group(1)) + delta)
                         if int(m.group(1)) > first else m.group(), cur)
        if new != cur:
            out[pin] = new
            rebind[sha(data[pin])] = sha(new)
            shifted.append(pin)
while True:
    grew = False
    for p in paths:
        cur = out.get(p, data[p])
        new = TERM.sub(lambda m: atom(rebind[m.group(2)]) if m.group(2) in rebind else m.group(), cur)
        new = HEX.sub(lambda m: rebind.get(m.group(), m.group()), new)
        if new != cur:
            if p.name.endswith(".tpl.in"):
                raise SystemExit(f"template binds a rebound digest: {p}")
            prev = sha(cur)
            out[p] = new
            rebind[sha(data[p])] = sha(new)
            if prev != sha(data[p]):
                rebind[prev] = sha(new)
            grew = True
    if not grew:
        break
# Every changed file = mapped substitutions + rebound digests alone.
for p, new in out.items():
    masked = lambda b: COLUMN.sub(b"noncanonical(1,#)", SHA1.sub(b"#", HEX.sub(b"#", TERM.sub(b"#", b)))) \
        if p in shifted or any(p.is_relative_to(c) for c in commits) else HEX.sub(b"#", TERM.sub(b"#", b))
    assert masked(mapped(data[p]).split(b"\n", 1)[1] if p.name.endswith(".tpl.in") else mapped(data[p])) == \
        masked(new.split(b"\n", 1)[1] if p.name.endswith(".tpl.in") else new), p
for p, new in out.items():
    p.write_bytes(new)
digest = hashlib.sha256()
for p in sorted(paths):
    digest.update(p.relative_to(root).as_posix().encode() + b"\0" + p.read_bytes() + b"\0")
print(f"r99: {len(paths)} files scanned, {len(out)} changed, {len(rebind)} digests rebound, "
      f"{len(commits)} fixture commit ids, {len(shifted)} column pins; "
      f"scope tree sha256 {digest.hexdigest()}")
