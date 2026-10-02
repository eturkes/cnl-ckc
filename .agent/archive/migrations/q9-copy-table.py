#!/usr/bin/env python3
"""Queue row Q9 (review F-SPEC-05, user go-ahead): every fixed page string of
rust/ckc-spec/src/ui.rs had two spellings — the renderer's `fixed("…"@)`/`lit("…"@)`
and the `copy_registry` list. The copy table (`copy_table!` in ui.rs) now defines each
string once as a named spec fn and builds `copy_registry()` from the same table in the
same order; renderer call sites name the fn. Values, order and count (239) are
unchanged. Usage: python3 -P <this> <repo-root>; refuses a tree already converted.
"""
import ast, re, sys
from pathlib import Path

root = Path(sys.argv[1]).resolve()
P = root / "rust/ckc-spec/src/ui.rs"
s = P.read_text()
assert "copy_table!" not in s, "already converted"
head, rest = s.split("pub open spec fn copy_registry() -> Seq<Bytes> {", 1)
body, tail = rest.split("// --- Copy gate:", 1)
LIT = r'lit\(\s*("(?:[^"\\]|\\.)*")\s*@\s*,?\s*\)'
ents = list(re.finditer(LIT + r'|\b(css_text|script_html)\(\)', body, re.S))
assert len(ents) == 239
SYM = {"&amp;": "amp", "&lt;": "lt", "&gt;": "gt", "&quot;": "quot", "&#x27;": "apos", "\n": "newline", "g/": "guideline_dir", "doc/": "document_dir", ".html": "html_ext",
       "/index.html": "index_page_path", "source/": "source_dir", ".pdf": "pdf_ext",
       " · ": "middot_sep", ", ": "comma_sep", " (": "paren_open_sep", ")": "paren_close", "Z": "zulu",
       " ": "space", ".": "period", ": ": "colon_sep", "p": "page_prefix", "# ": "hash_space",
       " UTC": "utc_suffix", "\">": "attr_end", "</a>": "a_close", "</td>": "td_close",
       "<td>": "td_open", "<tr>": "tr_open", "</tr>": "tr_close", "<a href=\"": "a_href_open"}


KEYWORDS = {"box", "type", "match", "loop", "move", "ref", "use", "mod", "fn", "impl", "self", "main"}


def name_of(text, used):
    if text in SYM:
        base = SYM[text]
    else:
        tags = re.sub(r"<(/?)([a-z0-9]+)[^>]*>", lambda m: f" {m.group(2)}_{'close' if m.group(1) else 'open'} ", text)
        words = re.findall(r"[a-z0-9_]+", tags.lower())
        base = "_".join(w.strip("_") for w in words[:6] if w.strip("_")) or "copy"
        alpha = [c for c in text if c.isalpha()]
        if alpha and alpha[0].isupper() and not text.lstrip().startswith("<"):
            base += "_cap"
        if text.startswith(" "):
            base = "sp_" + base
        if text.endswith(" ") and not text.endswith(": ") and text.strip():
            base += "_sp"
        if base[0].isdigit() or base in KEYWORDS:
            base = "copy_" + base
    name, k = base, 2
    while name in used:
        name, k = f"{base}_{k}", k + 1
    used.add(name)
    return name


used = {"css_text", "script_html"} | set(re.findall(r"\b[a-z_][a-z0-9_]*\b", s))  # never shadow or be shadowed
table, names, order = [], {}, []
for m in ents:
    if m.group(2):
        order.append(m.group(2) + "()")
        table.append(f"    {m.group(2)};")
        continue
    text = ast.literal_eval(m.group(1))
    n = name_of(text, used)
    names[text] = n
    order.append(n + "()")
    table.append(f"    {n} = {m.group(1)};")
macro = '''// One definition per fixed page string: each entry becomes a named spec fn,
// and `copy_registry()` lists the entries in table order.
macro_rules! copy_table {
    ($($name:ident $(= $text:literal)?;)*) => {
        verus! {
            $( $( pub open spec fn $name() -> Bytes { lit($text@) } )? )*

            // --- Declared static copy/chrome: each literal that reaches a page. ---
            pub open spec fn copy_registry() -> Seq<Bytes> {
                seq![$($name()),*]
            }
        }
    };
}

copy_table! {
''' + "\n".join(table) + "\n}\n"
head = head.rstrip()
assert head.endswith("// --- Declared static copy/chrome: each literal that reaches a page. ---")
head = head[: -len("// --- Declared static copy/chrome: each literal that reaches a page. ---")].rstrip() + "\n\n"
# renderer + other spec code: name the registry strings
def rename(src):
    def fx(m):
        t = ast.literal_eval(m.group(1))
        return f"fixed_bytes({names[t]}())" if t in names else m.group(0)
    src = re.sub(r'\bfixed\(\s*("(?:[^"\\]|\\.)*")\s*@\s*,?\s*\)', fx, src, flags=re.S)
    def li(m):
        t = ast.literal_eval(m.group(1))
        return f"{names[t]}()" if t in names else m.group(0)
    return re.sub(r'\b' + LIT, li, src, flags=re.S)
# `lit` itself must stay defined before the table; split the verus! block around the registry.
out = rename(head) + "} // verus!\n\n" + macro + "\nverus! {\n\n// --- Copy gate:" + rename(tail)
P.write_text(out)
print(f"q9: {len(names)} named copy strings")

# Kernel mirrors (hand-written K5 modules; generated files stay generator output):
# the same names, so kernel spec terms match the spec renderer syntactically.
GENERATED = {"k5_sound_chrome.rs", "k5_sound_encoding.rs", "k5_sound_literals.rs", "k5_sound_patterns.rs",
             "k5_sound_registry.rs"}
for k in sorted((root / "rust/ckc-kernel/src").glob("k5_*.rs")):
    if k.name in GENERATED:
        continue
    src = k.read_text()

    def kfx(m):
        t = ast.literal_eval(m.group(1))
        return f"u::fixed_bytes(u::{names[t]}())" if t in names else m.group(0)

    def kli(m):
        t = ast.literal_eval(m.group(1))
        return f"u::{names[t]}()" if t in names else m.group(0)
    new = re.sub(r'\bu::fixed\(\s*' + LIT[5:], kfx, src, flags=re.S)
    new = re.sub(r'\bu::' + LIT, kli, new, flags=re.S)
    if new != src:
        k.write_text(new)
        print(f"q9: kernel mirror {k.name}")
