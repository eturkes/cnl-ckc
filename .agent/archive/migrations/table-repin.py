#!/usr/bin/env python3
"""Re-pin the two fixture families anchored on the CDC vocabulary table (corpus.md).

Usage: python3 -I table-repin.py <repo-root> <old-table-sha256> <old-body-row-count>

A table byte change moves (1) the `tests/certify/cases.tsv` digest mutants: the table
sha256 inside `vocabulary(sha256(...))` and its corrupted variant (first hex digit + 1
mod 16), and (2) the `tests/check-mutants/*.expect` pins of mutants that append a row to
the CDC table (`at row <rows + 1>`). Rewrites both to the current table; rerun = no change.
"""
import hashlib
import pathlib
import sys

TABLE = "guidelines/cdc-2022-opioid/vocabulary.tsv"


def quoted(digest):
    return f"'{digest}'" if digest[0].isdigit() else digest


def corrupt(digest):
    return "%x" % ((int(digest[0], 16) + 1) % 16) + digest[1:]


def main():
    root, old, old_rows = pathlib.Path(sys.argv[1]), sys.argv[2], int(sys.argv[3])
    if len(old) != 64 or any(c not in "0123456789abcdef" for c in old):
        sys.exit("table-repin: old sha256 must be 64 lowercase hex digits")
    raw = (root / TABLE).read_bytes()
    new = hashlib.sha256(raw).hexdigest()
    rows = sum(1 for line in raw.decode().splitlines() if line and not line.startswith("#"))
    changed = []
    cases = root / "tests/certify/cases.tsv"
    text = cases.read_text()
    out = text
    for a, b in ((corrupt(old), corrupt(new)), (old, new)):
        for form in (f"sha256('{a}')", f"sha256({a})"):  # a Prolog atom opening with a digit is quoted
            out = out.replace(form, f"sha256({quoted(b)})")
    if out != text:
        cases.write_text(out)
        changed.append(str(cases.relative_to(root)))
    for expect in sorted((root / "tests/check-mutants").glob("*.expect")):
        text = expect.read_text()
        out = text.replace(f"at row {old_rows + 1}\n", f"at row {rows + 1}\n") if TABLE in text else text
        if out != text:
            expect.write_text(out)
            changed.append(str(expect.relative_to(root)))
    print(f"table-repin: sha256 {old[:8]}… → {new[:8]}…, rows {old_rows} → {rows}; {len(changed)} files")
    for path in changed:
        print(f"  {path}")


if __name__ == "__main__":
    main()
