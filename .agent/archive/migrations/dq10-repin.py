#!/usr/bin/env python3
# DQ10 fixture re-pin (deferred row `Superseded notice overflow`), idempotent: every tests/ui
# golden/expect page file (a `golden/` tree, a `.golden` or `.expect` file; fixture inputs never;
# a page = `<!doctype html>` alone or as the body of an `HTTP ` response) gets the stylesheet rule
# RULE right after ANCHOR, inside the screen rules before `@media print {`. No other byte moves.
# Usage: python3 .agent/archive/migrations/dq10-repin.py <repo-root> → `dq10: <changed> of <pages> page files`.
import pathlib
import sys

assert len(sys.argv) == 2, "usage: dq10-repin.py <repo-root>"
ROOT = pathlib.Path(sys.argv[1]).resolve()
assert (ROOT / "tests/ui").is_dir(), f"no tests/ui under {ROOT}"
PRINT = b"@media print {"
ANCHOR = b"p.provenance span + a { margin-left: 0.75rem; }\n"
RULE = b"section.superseded p { overflow-wrap: anywhere; }\n"

pages = changed = 0
for path in sorted((ROOT / "tests/ui").rglob("*")):
    rel = path.relative_to(ROOT / "tests/ui")
    if not path.is_file() or (len(rel.parts) > 3 and rel.parts[2] in ("tree", "worktree")):
        continue
    if not ("golden" in rel.parts[:-1] or rel.suffix in (".golden", ".expect")):
        continue
    data = path.read_bytes()
    body = data.split(b"\n\n", 1)[-1] if data.startswith(b"HTTP ") else data
    if not body.startswith(b"<!doctype html>"):
        continue
    pages += 1
    head, cut, tail = data.partition(PRINT)
    assert cut and head.count(ANCHOR) == 1 and tail.count(RULE) == 0, path
    if head.count(ANCHOR + RULE) == 1:
        continue
    assert head.count(RULE) == 0, path
    path.write_bytes(head.replace(ANCHOR, ANCHOR + RULE) + cut + tail)
    changed += 1
print(f"dq10: {changed} of {pages} page files")
