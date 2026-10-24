#!/usr/bin/env python3
# RD fixture re-pin (contract rd D4), idempotent, derived from the contract alone:
# every tests/ui golden/expect page file (a `golden/` tree, a `.golden` or `.expect` file;
# fixture inputs never; a page = `<!doctype html>` alone or as the body of an `HTTP ` response)
# holding a decision form (`</fieldset>` line, then the `Reviewer name` label line) gains the
# publication sentence line between the two; no other byte moves.
# Usage: python3 .agent/archive/migrations/rd-publication-note.py <repo-root>
#   → `rd: <changed> of <forms> form files`.
import pathlib
import sys

assert len(sys.argv) == 2, "usage: rd-publication-note.py <repo-root>"
ROOT = pathlib.Path(sys.argv[1]).resolve()
assert (ROOT / "tests/ui").is_dir(), f"no tests/ui under {ROOT}"
FIELDSET = b"</fieldset>\n"
LABEL = b'<label for="reviewer">Reviewer name</label>\n'
NOTE = (
    b"<p>Your name and comment are published with your decision in the knowledge base"
    b" and its public repository.</p>\n"
)

forms = changed = 0
for path in sorted((ROOT / "tests/ui").rglob("*")):
    rel = path.relative_to(ROOT / "tests/ui")
    dirs = rel.parts[:-1]
    # Fixture inputs (a case's `tree/` + `worktree/`) never move, whatever they hold; a
    # guideline named `tree` inside golden output is output.
    if not path.is_file() or (len(rel.parts) > 3 and rel.parts[2] in ("tree", "worktree")):
        continue
    if not ("golden" in dirs or rel.suffix in (".golden", ".expect")):
        continue
    data = path.read_bytes()
    body = data.split(b"\n\n", 1)[-1] if data.startswith(b"HTTP ") else data
    if not body.startswith(b"<!doctype html>") or FIELDSET not in data:
        continue
    forms += 1
    assert data.count(FIELDSET) == 1, path
    new = data
    if FIELDSET + NOTE + LABEL not in new:
        assert new.count(FIELDSET + LABEL) == 1 and NOTE not in new, path
        new = new.replace(FIELDSET + LABEL, FIELDSET + NOTE + LABEL)
    assert new.count(NOTE) == 1 and new.count(FIELDSET + NOTE + LABEL) == 1, path
    if new != data:
        path.write_bytes(new)
        changed += 1
print(f"rd: {changed} of {forms} form files")
