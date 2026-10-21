#!/usr/bin/env python3
"""da1 D12: re-pin the nt1-surface schema model to the regenerated schema file.

usage: python3 -I da1-schema-repin.py <repo-root>

The model's concat! block = the header + rule lines of guidelines/schema.pl
(every line before the first guideline_within row, as `ckc compile` writes
them); the test still generates the within + scale rows itself. Idempotent:
a second run leaves the file byte-identical.
"""
import json
import pathlib
import sys

root = pathlib.Path(sys.argv[1])
lines = (root / 'guidelines/schema.pl').read_text(encoding='ascii').splitlines(keepends=True)
head = []
for line in lines:
    if line.startswith('guideline_within('):
        break
    head.append(line)
assert head and head[0].startswith('% schema.pl: '), 'unexpected schema header'
path = root / 'tests/nt1-surface/release.rs'
text = path.read_text(encoding='utf-8')
start = text.index('    let mut out=concat!(\n')
end = text.index('    ).to_string();\n', start)
block = '    let mut out=concat!(\n' + ',\n'.join('        ' + json.dumps(l) for l in head) + '\n'
path.write_text(text[:start] + block + text[end:], encoding='utf-8')
