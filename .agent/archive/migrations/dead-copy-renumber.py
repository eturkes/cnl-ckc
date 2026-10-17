#!/usr/bin/env python3
"""dead-copy D1 + D2 (contract .agent/contracts/dead-copy.md): the two dead copy-table rows
(registry 282 `how_long_after_a_reference_point`, 283 `how_long_before_a_reference_point`) leave
rust/ckc-spec/src/ui.rs, and every hand-written K5 proof index above them shifts down by 2.

Usage: python3 -I <this> <repo-root>
Index forms: `l::fNNN` (lemma names, 3 digits), `b::f(NNN)`, bare `f(NNN)` in k5_sound_build.rs
(its own `f` = the registry fragment), and the six `lemma_word` index constants in
k5_sound_words.rs (`k == 299 || k == 300`, its two conditional postconditions, its two callers).
States = the nine files' exact bytes (sha256 below): base → every file rewritten at once, the
result checked against the final digests before any write; final → no byte changes; any other
state (the table edited alone, one proof file already shifted) refuses before writing.
Generated proofs (k5-sound-* generators) regenerate separately (contract D3).
"""
import hashlib
import re
import sys
from pathlib import Path

DEAD = [
    '    how_long_after_a_reference_point = "how long after a reference point the action occurs";\n',
    '    how_long_before_a_reference_point = "how long before a reference point the action occurs";\n',
]
FILES = ["build", "document", "qualifier", "qualifier_words", "strength", "strength_words", "timing", "words"]
BASE = {  # 03d20b7f
    "rust/ckc-spec/src/ui.rs": "4c0ef214fce6b84d1642626d1c2b9e5c92462cabf6d9ec0c1e369490146219aa",
    "rust/ckc-kernel/src/k5_sound_build.rs": "d39951b1c7a9e8746de4ba98ddf719002306fa9320d26210dda30b50359e1ca3",
    "rust/ckc-kernel/src/k5_sound_document.rs": "80a0dbd2e7cb6264762f01bc038a5c6f6e39356d52a16fab66937e6480bcc670",
    "rust/ckc-kernel/src/k5_sound_qualifier.rs": "61186fa6be891745def0bc9fa0581ba707816433b46c2b3cb716deb94109dedd",
    "rust/ckc-kernel/src/k5_sound_qualifier_words.rs": "dffbbef86b5237721ab4ca23ff636d79637f2c7acc3860923960ae7542557fa0",
    "rust/ckc-kernel/src/k5_sound_strength.rs": "1bbb8415e27d932cb2e4082b7c23fb8e314f3699060315fd1a67ba8bf71a7818",
    "rust/ckc-kernel/src/k5_sound_strength_words.rs": "98983e3203f53638bfbc81c40849138f7b4fe673833f8be8e5c0ff22f7aabdb2",
    "rust/ckc-kernel/src/k5_sound_timing.rs": "a32ee9d69524bc93c509f08d6b0326628353c512efa51c20c37c2cae196937cf",
    "rust/ckc-kernel/src/k5_sound_words.rs": "949be5a6b2172f4bd5bac53f2803e8b300f8a29cf06d0257a93cff84f8053392",
}
FINAL = {
    "rust/ckc-spec/src/ui.rs": "80cc7b1a9ea916ff3e898b238fac9be9fd5cf3acd39a349ea1d0dab0689013f5",
    "rust/ckc-kernel/src/k5_sound_build.rs": "b1f36dbd5da4a8d4892be62c44024e60bf0fd046e1a97c46f680c3dc2f223212",
    "rust/ckc-kernel/src/k5_sound_document.rs": "782b91b7dd21a704a6117bdfcf7dbf0d6ac32ded93ac54d02962ac21e6334c13",
    "rust/ckc-kernel/src/k5_sound_qualifier.rs": "ab76a8c02452003a6030d00d6b709f8195acf02b689426d77d61c67c9b5596a7",
    "rust/ckc-kernel/src/k5_sound_qualifier_words.rs": "6274d6e11ea6a476f5f3bacb2cdd243dc992f6d10ba4d52c8a25c8a0da90fda2",
    "rust/ckc-kernel/src/k5_sound_strength.rs": "27f9d499581237a02bddcd1f0370a17d644e39169fc6ef902db6d9fdec58e33f",
    "rust/ckc-kernel/src/k5_sound_strength_words.rs": "6637fb000b9ce3e61ee9a05759782434abab42eb8b7853253098d87fb0a1f2ae",
    "rust/ckc-kernel/src/k5_sound_timing.rs": "0071a76543dbc18c4251aae9f3b4f99880d2127afe79bbec354e1bbfa0eb2f47",
    "rust/ckc-kernel/src/k5_sound_words.rs": "6a1de8b6f61efcd45c8eae7e482b9705cc176d2d9bae39468bd10223b58250e7",
}
FORMS = [r"\bl::f(\d{3})\b", r"\bb::f\((\d+)\)"]
BARE = r"(?<![A-Za-z0-9_:])f\((\d+)\)"
WORDS = [  # lemma_word's registry index k: 299 = approximation, 300 = range
    ("k == 299 || k == 300,", "k == 297 || k == 298,"),
    ("k == 299 ==> h::sound(w::approximation_row", "k == 297 ==> h::sound(w::approximation_row"),
    ("k == 300 ==> h::sound(w::range_row", "k == 298 ==> h::sound(w::range_row"),
    ("lemma_word(raw, tm.approximations[i], 299, inputs);", "lemma_word(raw, tm.approximations[i], 297, inputs);"),
    ("lemma_word(raw, tm.ranges[i], 300, inputs);", "lemma_word(raw, tm.ranges[i], 298, inputs);"),
]


def table_rows(ui):
    block = ui.split("copy_table! {", 1)[1].split("\n}\n", 1)[0]
    return re.findall(r'^\s+([a-z0-9_]+)(?: = ("(?:[^"\\]|\\.)*"))?;$', block, re.M)


def digests(texts):
    return {rel: hashlib.sha256(text.encode()).hexdigest() for rel, text in texts.items()}


def main(root):
    # Bytes, never read_text(): universal newlines would hash a CRLF copy as the LF state.
    current = {rel: (root / rel).read_bytes().decode() for rel in BASE}
    state = digests(current)
    if state == FINAL:
        print("dead-copy: already applied")
        return
    off = sorted(rel for rel in BASE if state[rel] != BASE[rel])
    assert not off, f"neither the base nor the final state: {off}"
    ui_path = root / "rust/ckc-spec/src/ui.rs"
    ui = current["rust/ckc-spec/src/ui.rs"]
    rows = [n for n, _ in table_rows(ui)]
    proofs = {n: root / f"rust/ckc-kernel/src/k5_sound_{n}.rs" for n in FILES}
    words = current["rust/ckc-kernel/src/k5_sound_words.rs"]
    assert len(rows) == 347 and rows[282:284] == [d.split()[0] for d in DEAD], "ui.rs: dead rows at 282 + 283"
    assert all(words.count(old) == 1 for old, _ in WORDS), "k5_sound_words.rs: lemma_word index anchors"
    out, total = {ui_path: ui.replace(DEAD[0], "").replace(DEAD[1], "")}, 0
    for name, path in proofs.items():
        text = current[path.relative_to(root).as_posix()]
        forms = FORMS + ([BARE] if name == "build" else [])
        found = [int(m.group(1)) for pat in forms for m in re.finditer(pat, text)]
        assert not {282, 283} & set(found), f"{path}: references a removed row"
        n = 0

        def shift(m):
            nonlocal n
            i = int(m.group(1))
            if i < 284:
                return m.group(0)
            n += 1
            digits = f"{i - 2:03d}" if m.group(0).startswith("l::f") else str(i - 2)
            return m.group(0)[: m.start(1) - m.start(0)] + digits + m.group(0)[m.end(1) - m.start(0):]

        for pat in forms:
            text = re.sub(pat, shift, text)
        if name == "words":
            for old, new in WORDS:
                text = text.replace(old, new)
                n += len(re.findall(r"\b(?:299|300)\b", old))
        out[path] = text
        print(f"dead-copy: {path.name} {n} index references renumbered")
        total += n
    result = digests({path.relative_to(root).as_posix(): text for path, text in out.items()})
    assert result == FINAL, f"rewrite misses the final state: {sorted(r for r in FINAL if result.get(r) != FINAL[r])}"
    for path, text in out.items():
        path.write_bytes(text.encode())
    print(f"dead-copy: 2 copy-table rows removed, {total} index references renumbered")


if __name__ == "__main__":
    main(Path(sys.argv[1]).resolve())
