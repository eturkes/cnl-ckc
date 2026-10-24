"""Rebuild + verify the extraction census from its committed parts.

usage: python3 -I .agent/archive/extraction-census/check.py <repo-root>
1. candidates.py regenerates the candidate rows (pdftotext of the source PDF) from the guideline at BASE (the census
   reads the 713-region extraction; later corpus rounds extract more) → sha256 must equal CANDIDATES_SHA256;
2. census.tsv = codings.tsv main columns patched by adjudication.tsv, row for row;
3. validate.py grammar over census.tsv;
4. prints blind agreement (scope, Cohen's kappa, ace overlap) of MAIN vs each coder;
5. map.tsv (the CR round): every in-scope row → a region of the live coverage.tsv with a ruling (not pending).
rc 0 iff 1–3 + 5 hold.
"""
import hashlib
import subprocess
import sys
import tempfile
from pathlib import Path

CANDIDATES_SHA256 = "2cb3fdbb58500500d34cc28e569e4366fca1628ffda786298900864f1cdfe47e"
BASE = "6646052a"
IN = {"directive", "action-claim", "definition", "scope", "reported", "aim"}


def rows(path):
    return [l.split("\t") for l in path.read_text(encoding="utf-8").splitlines()[1:] if l]


def main():
    root = Path(sys.argv[1]).resolve()
    here = root / ".agent/archive/extraction-census"
    ok = True
    with tempfile.TemporaryDirectory() as tmp:
        base = Path(tmp) / "base"
        base.mkdir()
        tree = subprocess.run(["git", "-C", str(root), "archive", BASE, "guidelines/cdc-2022-opioid"], check=True, capture_output=True).stdout
        subprocess.run(["tar", "-x", "-C", str(base)], input=tree, check=True)
        subprocess.run(["python3", "-I", str(here / "candidates.py"), str(base), tmp], check=True, capture_output=True)
        cand = Path(tmp) / "candidates.tsv"
        digest = hashlib.sha256(cand.read_bytes()).hexdigest()
        print(f"candidates: sha256 {digest} {'ok' if digest == CANDIDATES_SHA256 else 'DIFFERS'}")
        ok &= digest == CANDIDATES_SHA256
        v = subprocess.run(["python3", "-I", str(here / "validate.py"), str(base), str(cand), str(here / "census.tsv"), "1", "100"],
                           capture_output=True, text=True)
        print(v.stdout.strip().splitlines()[-1])
        ok &= v.returncode == 0
    codings = {r[0]: r for r in rows(here / "codings.tsv")}
    patch = {r[0]: r for r in rows(here / "adjudication.tsv")}
    mismatch = 0
    for r in rows(here / "census.tsv"):
        want = patch[r[0]][1:4] if r[0] in patch else codings[r[0]][1:4]
        basis = patch[r[0]][4] if r[0] in patch else "M"
        if r[3:6] != want or r[6] != basis:
            mismatch += 1
    print(f"rebuild: {'ok' if not mismatch else f'{mismatch} rows differ'}")
    ok &= not mismatch
    for coder in ("1", "2"):
        sel = [r for r in codings.values() if r[4] == coder]
        n = len(sel)
        a = [r[1] in IN for r in sel]
        b = [r[5] in IN for r in sel]
        po = sum(x == y for x, y in zip(a, b)) / n
        pa, pb = sum(a) / n, sum(b) / n
        pe = pa * pb + (1 - pa) * (1 - pb)
        ace_a = {r[0] for r in sel if r[2] == "ace"}
        ace_b = {r[0] for r in sel if r[6] == "ace"}
        print(f"coder {coder}: rows={n} scope_agree={po:.3f} kappa={(po - pe) / (1 - pe):.3f} "
              f"in main={sum(a)} coder={sum(b)} ace main={len(ace_a)} coder={len(ace_b)} both={len(ace_a & ace_b)}")
    # The corpus round's map: every in-scope row → a region that holds a coverage ruling.
    in_scope = [r[0] for r in rows(here / "census.tsv") if r[4] != "-"]
    mapped = {r[0]: r[1] for r in rows(here / "map.tsv")}
    cov = {r[0]: r[4] for r in rows(root / "guidelines/cdc-2022-opioid/coverage.tsv") if not r[0].startswith("#")}
    unmapped = [x for x in in_scope if x not in mapped]
    extra = [x for x in mapped if x not in in_scope]
    unruled = [x for x in mapped if not cov.get(mapped[x]) or cov[mapped[x]] == "pending"]
    print(f"map: rows={len(mapped)} in_scope={len(in_scope)} unmapped={len(unmapped)} extra={len(extra)} "
          f"unruled={len(unruled)} regions={len(set(mapped.values()))}")
    ok &= not (unmapped or extra or unruled)
    sys.exit(0 if ok else 1)


main()
