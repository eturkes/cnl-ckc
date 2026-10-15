#!/usr/bin/env python3
# N23 fixture re-pin (contract n23 D7), idempotent, derived from the contract alone. Scope =
# n22's: tests/ui golden/expect files (a `golden/` tree, a `.golden` or `.expect` file) outside
# fixture input trees, holding a page (`<head>` + charset line). Precondition: no `·` or `—`
# in any tests/ui `tree/` | `worktree/` input, so every one in a golden page is generated chrome.
# Per page, in order: the provenance line (the line after a document heading `<h2>` holding a
# status chip) → `<p class="provenance">`, `<span>Passage R</span>`, one space, the link; the
# nav's ` · ` joins → one space; every other ` · ` (section-path joins) → ` &gt; `; the
# `<title>` suffix ` — cnl-ckc reviewer` dropped; `Not covered — ` → `Not covered: `; the two
# stylesheet rules after `nav.docnav { … }`. No other byte moves.
# Usage: python3 .agent/archive/migrations/n23-separators.py <repo-root> → `n23: <changed> of <pages> page files`
import pathlib
import re
import sys

assert len(sys.argv) == 2, "usage: n23-separators.py <repo-root>"
ROOT = pathlib.Path(sys.argv[1]).resolve()
UI = ROOT / "tests/ui"
assert UI.is_dir(), f"no tests/ui under {ROOT}"
HEAD = '<head>\n<meta charset="utf-8">\n'
DOCNAV = "nav.docnav { padding: 1rem 0; border-top: 1px solid #e5e7eb; }\n"
RULES = "nav.docnav a + a { margin-left: 1.5rem; }\np.provenance span + a { margin-left: 0.75rem; }\n"
CHIP_H2 = re.compile(r'<h2>.*<span class="chip chip-[a-z]+">[A-Za-z]+</span></h2>')


def provenance(line):
    content = line[len("<p>") : -len("</p>")]
    if content.startswith('<a href="../source/'):
        return f'<p class="provenance">{content}</p>'
    region, sep, link = content.partition(' · <a href="../source/')
    item = f"<span>Passage {region}</span>"
    return f'<p class="provenance">{item} <a href="../source/{link}</p>' if sep else f'<p class="provenance">{item}</p>'


def migrate(text):
    lines = text.split("\n")
    for i in range(1, len(lines)):
        prev, line = lines[i - 1], lines[i]
        if CHIP_H2.fullmatch(prev) and line.startswith("<p>") and line.endswith("</p>") and "All decision records</a>" not in line:
            lines[i] = provenance(line)
        elif line.startswith('<nav class="docnav">'):
            lines[i] = line.replace(" · ", " ")
    text = "\n".join(lines).replace(" · ", " &gt; ")
    text = re.sub(r"(<title>[^<\n]*) — cnl-ckc reviewer</title>", r"\1</title>", text)
    text = text.replace("Not covered — ", "Not covered: ")
    if RULES not in text:
        assert text.count(DOCNAV) == 1
        text = text.replace(DOCNAV, DOCNAV + RULES)
    return text


for path in sorted(UI.rglob("*")):
    rel = path.relative_to(UI)
    if path.is_file() and {"tree", "worktree"} & set(rel.parts[:-1]):
        data = path.read_bytes()
        assert "·".encode() not in data and "—".encode() not in data, f"precondition: {rel}"

pages = changed = 0
for path in sorted(UI.rglob("*")):
    rel = path.relative_to(UI)
    dirs = rel.parts[:-1]
    if not path.is_file() or {"tree", "worktree"} & set(dirs):
        continue
    if not ("golden" in dirs or rel.suffix in (".golden", ".expect")):
        continue
    data = path.read_bytes()
    if HEAD.encode() not in data:
        continue
    pages += 1
    new = migrate(data.decode("utf-8")).encode("utf-8")
    assert "·".encode() not in new and "—".encode() not in new, f"old glyph left: {rel}"
    if new != data:
        path.write_bytes(new)
        changed += 1
print(f"n23: {changed} of {pages} page files")
