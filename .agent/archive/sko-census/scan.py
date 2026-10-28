"""Derived + root actual-context entities whose noun another document's condition reads.
usage: scan.py <pl-dir> → TSV docid S kind noun readers"""
import re, sys
from pathlib import Path
d = Path(sys.argv[1])
head = re.compile(r"^guideline_entity\(actual,'\$guideline_id'\(product,'([^']+)',(\d+),ref\(\d+\),\[([^\]]*)\]\),'?([^',]+)'?,\w+\)( :- |\.$)")
body = re.compile(r"guideline_entity\(actual,[A-Z_][A-Za-z0-9_]*,'?([^',)]+)'?,\w+\)")
readers, made = {}, set()
for p in sorted(d.glob('*.pl')):
    for l in p.read_text(encoding='utf-8').split('\n'):
        if ':-' in l:
            for n in body.findall(l.split(':-', 1)[1]): readers.setdefault(n, set()).add(p.stem)
        m = head.match(l)
        if m: made.add((m.group(1), int(m.group(2)), 'rule' if m.group(5) == ' :- ' else 'root', m.group(4)))
for doc, s, kind, n in sorted(made):
    r = readers.get(n, set()) - {doc}
    if r: print(f'{doc}\t{s}\t{kind}\t{n}\t{len(r)}')
