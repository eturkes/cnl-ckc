#!/usr/bin/env python3
# REV-K fixture re-pin (contract rev-k P1/P2), idempotent, derived from the contract alone:
# every tests/ui golden/expect page file (a `golden/` tree, a `.golden` or `.expect` file;
# fixture inputs never; a page = `<!doctype html>` alone or as the body of an `HTTP ` response)
# P1: the stylesheet before `@media print {` gets the three highlight-click dimming rules and
#     the text-field border at #4b5563 (the print block keeps its bytes);
# P2: each count noun of the review tally + the compiled-code line count takes its number.
# Page bytes outside these sites never move; corpus text renders escaped, so the `<p>` and
# `<summary>` openings anchor fixed copy alone.
# Usage: python3 .agent/archive/migrations/rev-k-repin.py <repo-root>
#   → `rev-k: <changed> of <pages> page files, <nouns> count nouns singular`.
import pathlib
import re
import sys

assert len(sys.argv) == 2, "usage: rev-k-repin.py <repo-root>"
ROOT = pathlib.Path(sys.argv[1]).resolve()
assert (ROOT / "tests/ui").is_dir(), f"no tests/ui under {ROOT}"
PRINT = b"@media print {"
RULES = [
    (b"main.hl-click pre.prose { color: %s; }\n", None),
    (b"main.hl-click pre.prose .kw { color: %s; }\n", None),
    (b"main.hl-click pre.prose mark:not(.hl-pick) { background: none; text-decoration-color: %s; }\n", None),
    (
        b'input[type="text"], textarea { display: block; box-sizing: border-box; width: 100%%;'
        b" max-width: 28rem; margin-top: 0.3rem; padding: 0.45rem 0.6rem; border: 1px solid %s;"
        b" font-family: inherit; font-size: 1rem; color: #111827; background: #ffffff; }\n",
        b"#e5e7eb",
    ),
]
NEW = b"#4b5563"
# (opening, [(singular, plural) after each number]); a pair of equal forms = no noun.
SENTENCES = [
    (b"<p>Reviewers recorded ", [(b" decision on ", b" decisions on "), (b" of ", b" of "), (b" document.", b" documents.")]),
    (b"<p>No decisions are recorded for the ", [(b" document in this guideline.", b" documents in this guideline.")]),
    (b"<summary>Compiled Prolog (", [(b" line)</summary>", b" lines)</summary>")]),
]


def stylesheet(data):
    head, cut, tail = data.partition(PRINT)
    assert cut, "page without its print block"
    for rule, old in RULES:
        before, after = rule % (old or b"#9ca3af"), rule % NEW
        assert head.count(before) + head.count(after) == 1, rule
        head = head.replace(before, after)
    return head + cut + tail


def nouns(data):
    singular = 0
    for opening, slots in SENTENCES:
        out, rest = b"", data
        while opening in rest:
            pre, _, rest = rest.partition(opening)
            out += pre + opening
            for one, many in slots:
                n = re.match(rb"\d+", rest)
                assert n, opening
                count, rest = n.group(), rest[n.end() :]
                form = next(f for f in (one, many) if rest.startswith(f))
                want = one if count == b"1" else many
                singular += want != many
                out += count + want
                rest = rest[len(form) :]
        data = out + rest
    return data, singular


pages = changed = singular = 0
for path in sorted((ROOT / "tests/ui").rglob("*")):
    rel = path.relative_to(ROOT / "tests/ui")
    # Fixture inputs (a case's `tree/` + `worktree/`) never move, whatever they hold.
    if not path.is_file() or (len(rel.parts) > 3 and rel.parts[2] in ("tree", "worktree")):
        continue
    if not ("golden" in rel.parts[:-1] or rel.suffix in (".golden", ".expect")):
        continue
    data = path.read_bytes()
    body = data.split(b"\n\n", 1)[-1] if data.startswith(b"HTTP ") else data
    if not body.startswith(b"<!doctype html>"):
        continue
    pages += 1
    new, n = nouns(stylesheet(data))
    singular += n
    # The old dim color is gone from the stylesheet; corpus text may spell it and stays.
    assert b"#9ca3af" not in new.split(b"<style>", 1)[1].split(b"</style>", 1)[0], path
    if new != data:
        path.write_bytes(new)
        changed += 1
print(f"rev-k: {changed} of {pages} page files, {singular} count nouns singular")
