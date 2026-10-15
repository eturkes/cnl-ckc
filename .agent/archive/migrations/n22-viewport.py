#!/usr/bin/env python3
# N22 fixture re-pin (contract n22 D5), idempotent, derived from the contract alone:
# every tests/ui golden/expect file (a `golden/` tree, a `.golden` or `.expect` file; fixture
# inputs never) holding a rendered page (`<head>` followed by its charset meta line;
# one `@media print {` line inside its `<style>` block) gains the viewport meta line right
# after the charset line and the narrow-screen table rule line right before the print
# block; no other byte moves.
# Usage: python3 .agent/archive/migrations/n22-viewport.py <repo-root> → `n22: <changed> of <pages> page files`.
# The root is an explicit argument: a cwd-derived root (git toplevel) once rewrote the
# enclosing checkout when a copy ran from a scratch directory inside it.
import pathlib
import sys

assert len(sys.argv) == 2, "usage: n22-viewport.py <repo-root>"
ROOT = pathlib.Path(sys.argv[1]).resolve()
assert (ROOT / "tests/ui").is_dir(), f"no tests/ui under {ROOT}"
CHARSET = b'<meta charset="utf-8">\n'
HEAD = b"<head>\n" + CHARSET
VIEWPORT = b'<meta name="viewport" content="width=device-width, initial-scale=1">\n'
PRINT = b"@media print {\n"
RULE = b"@media screen and (max-width: 40rem) { table { display: block; overflow-x: auto; } }\n"

pages = changed = 0
for path in sorted((ROOT / "tests/ui").rglob("*")):
    rel = path.relative_to(ROOT / "tests/ui")
    dirs = rel.parts[:-1]
    # Fixture inputs (`tree/`, `worktree/`) never move, whatever they hold.
    if not path.is_file() or {"tree", "worktree"} & set(dirs):
        continue
    if not ("golden" in dirs or rel.suffix in (".golden", ".expect")):
        continue
    data = path.read_bytes()
    if HEAD not in data:
        continue
    pages += 1
    style = data.find(b"<style>\n")
    assert data.count(HEAD) == 1 and data.count(PRINT) == 1, path
    assert 0 <= style < data.find(PRINT) < data.find(b"</style>\n", style), path
    new = data
    if VIEWPORT not in new:
        new = new.replace(HEAD, HEAD + VIEWPORT)
    if RULE not in new:
        new = new.replace(PRINT, RULE + PRINT)
    assert new.count(VIEWPORT) == 1 and new.count(RULE) == 1, path
    if new != data:
        path.write_bytes(new)
        changed += 1
print(f"n22: {changed} of {pages} page files")
