#!/usr/bin/env python3
"""Re-pin the check mutants whose pins hold a corpus position (corpus.md).

Usage: python3 -I check-mutant-repin.py <repo-root> <check_mutants test log>

The formula-cycle plants append to `cdc2022-opioid-rec02.pl` (pins name its lines from
line count + 1 on), `derived-entity-stale` appends a row to `audit/derived-entities.tsv`
(pin = that row). A corpus change that moves either count fails these mutants with the
same message at moved positions. For each such failure in the log (`cargo test --test
check_mutants` output), the new first violation must equal the old pin with every position
shifted by the count's change; then it replaces the pin. Any other failure is refused.
"""
import pathlib
import re
import sys

G = "guidelines/cdc-2022-opioid"
POS = re.compile(r"(?<=:)\d+|(?<=row )\d+")


def main():
    root, log = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2]).read_text(encoding="utf-8")
    base = {
        "formula-cycle-": len((root / G / "pl/cdc2022-opioid-rec02.pl").read_bytes().splitlines()) + 1,
        "derived-entity-stale": len((root / G / "audit/derived-entities.tsv").read_bytes().splitlines()) + 1,
    }
    bad = 0
    for name, line in re.findall(r'^([a-z0-9-]+): rc Some\(1\) expected 1, first violation "(.*)\\n"$', log, re.M):
        family = next((k for k in base if name.startswith(k)), None)
        pin = root / f"tests/check-mutants/{name}.expect"
        old = pin.read_text(encoding="utf-8").rstrip("\n")
        first = POS.search(old)
        if family is None or first is None:
            print(f"check-mutant-repin: {name}: not a positional pin", file=sys.stderr); bad += 1; continue
        delta = base[family] - int(first.group())
        if POS.sub(lambda m: str(int(m.group()) + delta), old) != line:
            print(f"check-mutant-repin: {name}: {line!r} is not the pin shifted by {delta}", file=sys.stderr); bad += 1
            continue
        pin.write_text(line + "\n", encoding="utf-8")
        print(f"check-mutant-repin: {name} shifted by {delta}")
    sys.exit(1 if bad else 0)


if __name__ == "__main__":
    main()
