#!/usr/bin/env python3
"""SUP4 P2/P3: pin-baseline.py <pre-SUP4 ckc> <repo-root>.

Writes baseline.sha256. Each fixture renders with/without SUP rows; PL,
manifest, ledger, token and commit stay fixed. A SUP-only table is removed.
"""

import hashlib
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile


def main():
    if len(sys.argv) != 3:
        raise SystemExit("usage: pin-baseline.py <pre-SUP4 ckc> <repo-root>")
    binary, root = (Path(p).resolve(strict=True) for p in sys.argv[1:])
    suite = root / "tests/sup4-ui"
    fixtures = sorted((suite / "fixtures").iterdir())
    assert fixtures, "fixture census"
    pins = []
    with tempfile.TemporaryDirectory(prefix="pin-", dir=suite) as tmp:
        tmp = Path(tmp)
        env = dict(os.environ, GIT_CEILING_DIRECTORIES=str(tmp), TMPDIR=str(tmp))
        for case in fixtures:
            tree = tmp / case.name / "tree"
            shutil.copytree(case / "tree", tree)
            for source in tree.rglob("*.pl.in"):
                source.rename(source.with_suffix(""))
            table = tree / "guidelines/alpha/vocabulary.tsv"
            with_rows = None
            for mode in ("rows", "removed"):
                if mode == "removed":
                    lines = table.read_bytes().splitlines(keepends=True)
                    kept = [line for line in lines if not line.startswith((b"supersession\t", b"superseded\t"))]
                    assert len(kept) < len(lines), "row-removal positive control"
                    if len(kept) == 1:
                        table.unlink()
                    else:
                        table.write_bytes(b"".join(kept))
                outdir = tmp / case.name / mode
                out = subprocess.run(
                    [str(binary), "ui", "render", str(outdir), str(tree)],
                    cwd=tmp, env=env, capture_output=True, check=True,
                )
                assert not out.stderr, out.stderr
                pages = {
                    p.relative_to(outdir).as_posix(): p.read_bytes()
                    for p in sorted(outdir.rglob("*")) if p.is_file()
                }
                assert "index.html" in pages and len(pages) == 7, pages.keys()
                assert sum(name.endswith(".html") for name in pages) == 6
                for name, data in pages.items():
                    if name.endswith(".html"):
                        assert b'<section class="superseded">' not in data
                        assert b'<h2>Superseded passages</h2>' not in data
                    pins.append((f"{case.name}/{mode}/{name}", hashlib.sha256(data).hexdigest()))
                if with_rows is None:
                    with_rows = pages
                else:
                    assert pages == with_rows, "pre-SUP4 row-removal identity"
    lines = [
        "# SUP4 P2/P3 pre-unit page and source-byte pins; rows/removed inputs.",
        f"# Binary sha256: {hashlib.sha256(binary.read_bytes()).hexdigest()}",
    ]
    lines.extend(f"{digest}\t{name}" for name, digest in sorted(pins))
    (suite / "baseline.sha256").write_text("\n".join(lines) + "\n")
    print(f"SUP4 baseline: {len(pins)} pins ({len(fixtures)} fixtures × 2 controls × 7 files)")


if __name__ == "__main__":
    main()
