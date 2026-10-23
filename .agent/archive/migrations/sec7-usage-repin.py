#!/usr/bin/env python3
# SEC7 (security review): `ckc ui request` gains `--session <64 hex>`; re-pins
# every pinned usage line under tests/ by inserting the flag after `--token`.
# Idempotent: a re-run changes nothing.
# usage: python3 -I sec7-usage-repin.py <repo-root>
import pathlib
import sys

root = pathlib.Path(sys.argv[1])
old = b"[--token <text>] [--now <utc-iso>]"
new = b"[--token <text>] [--session <64 hex>] [--now <utc-iso>]"
files = lines = 0
for path in sorted(root.joinpath("tests").rglob("*")):
    if not path.is_file() or path.is_symlink():
        continue
    data = path.read_bytes()
    n = data.count(old)
    if n:
        path.write_bytes(data.replace(old, new))
        files += 1
        lines += n
print(f"sec7-usage-repin: {lines} lines in {files} files")
