#!/usr/bin/env python3
# SEC5 (security review F1): the reviewer CSP gains base-uri, form-action and
# frame-ancestors. Re-pins every golden CSP header line under tests/ by appending
# the three directives; other bytes stay. Idempotent: a re-run changes nothing.
# usage: python3 -I sec5-csp-repin.py <repo-root>
import pathlib
import re
import sys

root = pathlib.Path(sys.argv[1])
line = re.compile(
    rb"^(Content-Security-Policy: default-src 'none'; style-src 'unsafe-inline'; "
    rb"script-src 'sha256-[A-Za-z0-9+/=]+')$",
    re.M,
)
suffix = b"; base-uri 'none'; form-action 'self'; frame-ancestors 'none'"
files = lines = 0
for path in sorted(root.joinpath("tests").rglob("*")):
    if not path.is_file() or path.is_symlink():
        continue
    data = path.read_bytes()
    new, n = line.subn(lambda m: m.group(1) + suffix, data)
    if n:
        path.write_bytes(new)
        files += 1
        lines += n
print(f"sec5-csp-repin: {lines} lines in {files} files")
