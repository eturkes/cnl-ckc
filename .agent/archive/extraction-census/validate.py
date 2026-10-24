"""Validate one census coding file against the candidate rows it must cover.

usage: python3 -I .agent/archive/extraction-census/validate.py <repo-root> <candidates.tsv> <coding.tsv> <first-page> <last-page>
coding.tsv columns: id, page, text, class, disposition, reason[, basis] (header row first).
rc 0 = every judged candidate row on pages first..last coded once, in order, by the rubric's grammar.
"""
import re
import sys
from pathlib import Path

IN_SCOPE = {"directive", "action-claim", "definition", "scope", "reported", "aim"}
OUT_SCOPE = {"evidence", "background", "method", "scaffolding", "fragment"}
UNCOVERED = {"aim", "descriptive", "external", "process", "heading", "notice"}


def main():
    root, cand, coding = Path(sys.argv[1]), sys.argv[2], sys.argv[3]
    lo, hi = int(sys.argv[4]), int(sys.argv[5])
    regions = {}
    for line in (root / "guidelines/cdc-2022-opioid/coverage.tsv").read_text(encoding="utf-8").splitlines():
        if line and not line.startswith("#"):
            cols = line.split("\t")
            regions[cols[0]] = cols[4]
    want = []
    for line in open(cand, encoding="utf-8").read().splitlines()[1:]:
        c = line.split("\t")
        if not c[4] and lo <= int(c[1]) <= hi:
            want.append(c[0])
    got = [l.split("\t") for l in open(coding, encoding="utf-8").read().splitlines()[1:] if l]
    errors, unknown = [], 0
    if [g[0] for g in got] != want:
        errors.append(f"row ids differ from the {len(want)} candidate rows of pages {lo}-{hi} (got {len(got)})")
    for g in got:
        if len(g) not in (6, 7):
            errors.append(f"{g[0]}: {len(g)} columns, want 6 or 7")
            continue
        rid, _, _, cls, disp, reason = g[:6]
        if "unknown" in (cls, disp, reason):
            unknown += 1
            continue
        if cls in IN_SCOPE:
            m = re.fullmatch(r"restates\((B3-\d\d|S\d+-\d\d)\)", disp)
            u = re.fullmatch(r"uncovered\(([a-z]+)\)", disp)
            if m:
                target = regions.get(m.group(1))
                if target is None:
                    errors.append(f"{rid}: restates names no region: {m.group(1)}")
                elif target.startswith("restates("):
                    errors.append(f"{rid}: restates target {m.group(1)} is itself {target}")
            elif u:
                if u.group(1) not in UNCOVERED:
                    errors.append(f"{rid}: uncovered class {u.group(1)}")
            elif disp != "ace":
                errors.append(f"{rid}: disposition {disp!r} for in-scope class {cls}")
        elif cls in OUT_SCOPE:
            if disp != "-":
                errors.append(f"{rid}: out-of-scope class {cls} needs disposition '-', got {disp!r}")
        else:
            errors.append(f"{rid}: class {cls!r}")
        if not reason.strip():
            errors.append(f"{rid}: empty reason")
    for e in errors[:40]:
        print(e)
    print(f"census-validate: rows={len(got)} want={len(want)} unknown={unknown} errors={len(errors)}")
    sys.exit(0 if not errors and not unknown else 1)


main()
