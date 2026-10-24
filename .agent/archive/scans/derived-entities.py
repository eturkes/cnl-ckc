"""Derived actual-context entity nouns of the given documents that some rule body reads (Skolem pollution of condition searches; corpus.md Authoring law).
usage: python3 -I .agent/archive/scans/derived-entities.py <pl-dir> <ids-file>  (pl-dir = compiled documents; ids-file = docids to scan, one per line)"""
import re, sys
from pathlib import Path
d = Path(sys.argv[1]); new = set(Path(sys.argv[2]).read_text().split())
head = re.compile(r"^guideline_entity\(actual,'\$guideline_id'\(product,'([^']+)',(\d+),ref\(\d+\),\[[^\]]*\]\),'?([^',]+)'?,\w+\) :- ")
body = re.compile(r"guideline_entity\(actual,[A-Z][A-Za-z0-9_]*,'?([^',)]+)'?,\w+\)")
readers, derived = {}, {}
for p in d.glob('*.pl'):
    doc = p.stem
    for l in p.read_text(encoding='utf-8').split('\n'):
        if ':-' not in l:
            continue
        h, b = l.split(':-', 1)
        for n in body.findall(b):
            readers.setdefault(n, set()).add(doc)
        m = head.match(l)
        if m and doc in new:
            derived.setdefault(m.group(3), set()).add(f"{doc.replace('cdc2022-opioid-','')}:S{m.group(2)}")
for n in sorted(derived):
    r = readers.get(n, set()) - {x for x in new if any(x.endswith(s.split(':')[0]) for s in derived[n])}
    if r:
        print(f"{n}\tderived by {' '.join(sorted(derived[n]))}\tread by {len(r)} docs")
