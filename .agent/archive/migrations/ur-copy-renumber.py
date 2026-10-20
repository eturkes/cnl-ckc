#!/usr/bin/env python3
"""ur D9 (contract .agent/contracts/ur.md): the copy-table rows that only the removed review tables
used leave rust/ckc-spec/src/ui.rs, and every hand-written K5 proof index past a removed row shifts
down by the number of removed rows below it.

Usage: python3 -I <this> <repo-root>
Dead rows = `copy_table!` rows with no named use (`name(`) in ui.rs outside the table, comments
stripped; asserted equal to DEAD (names, base indices). Index forms in the kept proof files:
`l::fNNN` + `l::lNNN` (3 digits), `b::f(NNN)`, bare `f(NNN)` in k5_sound_build.rs (its own `f` = the registry
fragment). States = the files' exact bytes (sha256): BASE → every file rewritten at once, checked
against FINAL before any write; FINAL → no change; any other state refuses before writing.
Generated proofs (k5-sound-* generators) regenerate separately (contract D9).
"""
import hashlib
import re
import sys
from pathlib import Path

UI = "rust/ckc-spec/src/ui.rs"
PROOFS = ["build", "document", "guideline", "highlight", "meanings", "pages", "post", "records"]
DEAD = [  # (base index, row name)
    (243, "h3_open_timing_as_compiled_h3_close"),
    (244, "p_open_each_row_is_one_time_limit"),
    (245, "thead_open_tr_open_th_open_sentence_th_close_th_open"),
    (246, "statement_2"),
    (247, "condition_2"),
    (248, "excluded_condition"),
    (250, "exactly_sp"),
    (251, "at_least_sp"),
    (252, "more_than_sp"),
    (253, "at_most_sp"),
    (254, "less_than_sp"),
    (255, "lasts_sp"),
    (256, "within_sp"),
    (257, "repeats_sp"),
    (258, "sp_after"),
    (259, "sp_before"),
    (260, "sp_of"),
    (261, "sp_apart"),
    (262, "sp_second"),
    (263, "sp_seconds"),
    (264, "sp_minute"),
    (265, "sp_minutes"),
    (266, "sp_hour"),
    (267, "sp_hours"),
    (268, "sp_day"),
    (269, "sp_days"),
    (270, "sp_week"),
    (271, "sp_weeks"),
    (272, "sp_month"),
    (273, "sp_months"),
    (274, "sp_year"),
    (275, "sp_years"),
    (276, "h2_open_time_words_h2_close"),
    (277, "p_open_the_compiler_reads_a_time_limit"),
    (278, "thead_open_tr_open_th_open_word_th_close_th_open"),
    (279, "unit_of_time"),
    (280, "how_long_the_action_lasts"),
    (281, "how_far_the_action_lies_from_a"),
    (282, "how_far_apart_repeats_of_the_action_lie"),
    (283, "with_sp"),
    (284, "how_long_after_or_only_after"),
    (285, "how_long_before_or_only_before"),
    (286, "the_time_window_in_which_the_spacing"),
    (287, "how_many_of_an_item_the_action"),
    (288, "before"),
    (289, "after"),
    (290, "sp_per_sp"),
    (291, "counted_item_sep"),
    (292, "sp_apart_during_sp"),
    (293, "sp_from"),
    (294, "about_sp"),
    (295, "a_minimum_of_sp"),
    (296, "sp_to_sp"),
    (297, "marks_a_time_limit_as_approximate"),
    (298, "joins_the_upper_end_of_a_minimum"),
    (303, "h3_open_grade_as_compiled_h3_close"),
    (304, "p_open_each_row_is_one_grade"),
    (305, "thead_open_tr_open_th_open_measure"),
    (306, "strength_of_the_recommendation"),
    (307, "certainty_of_the_evidence"),
    (308, "grade_strong"),
    (309, "grade_conditional"),
    (310, "grade_high"),
    (311, "grade_moderate"),
    (312, "grade_low"),
    (313, "grade_very_low"),
    (314, "h3_open_preferences_as_compiled_h3_close"),
    (315, "p_open_each_row_is_one_preference"),
    (316, "thead_open_tr_open_th_open_sentence_stated"),
    (317, "op_should"),
    (318, "op_must"),
    (319, "op_can"),
    (320, "op_may"),
    (321, "op_not"),
    (322, "as_a_fact"),
    (323, "none_named"),
    (324, "word_sep"),
    (325, "h2_open_grade_words_h2_close"),
    (326, "p_open_the_compiler_reads_a_grade"),
    (327, "strength_of_the_recommendation_sep"),
    (328, "certainty_of_the_evidence_sep"),
    (329, "h2_open_preference_words_h2_close"),
    (330, "p_open_the_compiler_reads_a_preference"),
    (331, "marks_the_preferred_option"),
    (332, "joins_the_option_that_the_preferred"),
    (333, "h3_open_qualifiers_as_compiled_h3_close"),
    (334, "p_open_each_row_is_one_qualifier"),
    (335, "thead_open_tr_open_th_open_sentence_kind"),
    (343, "h2_open_qualifier_words_h2_close"),
    (344, "p_open_the_compiler_reads_a_qualifier"),
]
BASE = {  # ur checkpoint A1 (36d07477)
    "rust/ckc-spec/src/ui.rs": "3249c6c0bd8b78390ee874a679185ab058f808aecfaaeeb3a0e8d46d4ccaba87",
    "rust/ckc-kernel/src/k5_sound_build.rs": "b1f36dbd5da4a8d4892be62c44024e60bf0fd046e1a97c46f680c3dc2f223212",
    "rust/ckc-kernel/src/k5_sound_document.rs": "452cfc705aa8f9a974e15917c15b4852b3ac3889762abd8880ed79829f2a8cb6",
    "rust/ckc-kernel/src/k5_sound_guideline.rs": "44eb3e7678ef60d4e3540317ea5e7ba5c80df1a80ddb0c50602ad59792e8e544",
    "rust/ckc-kernel/src/k5_sound_highlight.rs": "f76427c7151c0f3a525835e13aa90831a25ce6bd737fdf7c5e0d9631bc983916",
    "rust/ckc-kernel/src/k5_sound_meanings.rs": "aeaf6724a8b0b474a1df1fcbe2df882592e7b548553f75768b7c65c64156037c",
    "rust/ckc-kernel/src/k5_sound_pages.rs": "765decb107ffb14d63daadd4031743aacad481c1c97c14b87c3a98f083e7dd6b",
    "rust/ckc-kernel/src/k5_sound_post.rs": "0561afc4e83d236321283095f798ee331869ac9af39a54fada7a6183021ad19b",
    "rust/ckc-kernel/src/k5_sound_records.rs": "4a83d1e592bfaf03138959bda9020575232561f4262265b57eaee4f7085b45f0",
}
FINAL = {  # the rewrite
    "rust/ckc-spec/src/ui.rs": "6baf36baad380ca5f7e5e91050617cf6709abd34869fab1f6946972386e85b2e",
    "rust/ckc-kernel/src/k5_sound_build.rs": "d9d223e28ecb5fd5e81ee065f828cdad9f4b5fc5e0d8a716dfa39dbedd3f9b3e",
    "rust/ckc-kernel/src/k5_sound_document.rs": "82c78f43bf772209690c0c5f124a6bacc05a7e2e25fe513c90aab95195cbba2e",
    "rust/ckc-kernel/src/k5_sound_guideline.rs": "44eb3e7678ef60d4e3540317ea5e7ba5c80df1a80ddb0c50602ad59792e8e544",
    "rust/ckc-kernel/src/k5_sound_highlight.rs": "f76427c7151c0f3a525835e13aa90831a25ce6bd737fdf7c5e0d9631bc983916",
    "rust/ckc-kernel/src/k5_sound_meanings.rs": "6c0cd04caf3c86f3db2ce6fac4f12d0a41601f7a41c9f6a2a910753f887e9335",
    "rust/ckc-kernel/src/k5_sound_pages.rs": "765decb107ffb14d63daadd4031743aacad481c1c97c14b87c3a98f083e7dd6b",
    "rust/ckc-kernel/src/k5_sound_post.rs": "0561afc4e83d236321283095f798ee331869ac9af39a54fada7a6183021ad19b",
    "rust/ckc-kernel/src/k5_sound_records.rs": "4a83d1e592bfaf03138959bda9020575232561f4262265b57eaee4f7085b45f0",
}
FORMS = [r"\bl::[fl](\d{3})\b", r"\bb::f\((\d+)\)"]
BARE = r"(?<![A-Za-z0-9_:])f\((\d+)\)"


def table(ui):
    block = ui.split("copy_table! {", 1)[1].split("\n}\n", 1)[0]
    return re.findall(r'^(\s+([a-z0-9_]+)(?: = ("(?:[^"\\]|\\.)*"))?;\n)', block + "\n", re.M)


def dead_rows(ui):
    head, rest = ui.split("copy_table! {", 1)
    outside = re.sub(r"//[^\n]*", "", head + rest.split("\n}\n", 1)[1])
    return [(i, name) for i, (_, name, _) in enumerate(table(ui)) if not re.search(r"\b" + name + r"\(", outside)]


def digests(texts):
    return {rel: hashlib.sha256(text.encode()).hexdigest() for rel, text in texts.items()}


def transform(current):
    ui = current[UI]
    dead = dead_rows(ui)
    assert dead == DEAD, f"dead rows differ from DEAD: {dead}"
    gone = {i for i, _ in dead}
    shift = lambda i: i - sum(1 for d in gone if d < i)
    out = {UI: ui}
    for line, name, _ in table(ui):
        if name in {n for _, n in dead}:
            out[UI] = out[UI].replace(line, "", 1)
    total = 0
    for name in PROOFS:
        rel = f"rust/ckc-kernel/src/k5_sound_{name}.rs"
        text = current[rel]
        forms = FORMS + ([BARE] if name == "build" else [])
        found = {int(m.group(1)) for pat in forms for m in re.finditer(pat, text)}
        assert not found & gone, f"{rel}: references a removed row {sorted(found & gone)}"
        n = 0

        def move(m):
            nonlocal n
            i = int(m.group(1))
            if shift(i) == i:
                return m.group(0)
            n += 1
            digits = f"{shift(i):03d}" if m.group(0).startswith("l::") else str(shift(i))
            return m.group(0)[: m.start(1) - m.start(0)] + digits + m.group(0)[m.end(1) - m.start(0):]

        for pat in forms:
            text = re.sub(pat, move, text)
        out[rel] = text
        print(f"ur-copy: {Path(rel).name} {n} index references renumbered")
        total += n
    print(f"ur-copy: {len(dead)} copy-table rows removed, {total} index references renumbered")
    return out


def main(root):
    rels = [UI] + [f"rust/ckc-kernel/src/k5_sound_{n}.rs" for n in PROOFS]
    # Bytes, never read_text(): universal newlines would hash a CRLF copy as the LF state.
    current = {rel: (root / rel).read_bytes().decode() for rel in rels}
    state = digests(current)
    if FINAL and state == FINAL:
        print("ur-copy: already applied")
        return
    if BASE:
        off = sorted(rel for rel in rels if state[rel] != BASE[rel])
        assert not off, f"neither the base nor the final state: {off}"
    out = transform(current)
    result = digests(out)
    if FINAL:
        assert result == FINAL, f"rewrite misses the final state: {sorted(r for r in FINAL if result[r] != FINAL[r])}"
    else:
        print("ur-copy: BASE =", state)
        print("ur-copy: FINAL =", result)
    for rel, text in out.items():
        (root / rel).write_bytes(text.encode())


if __name__ == "__main__":
    main(Path(sys.argv[1]).resolve())
