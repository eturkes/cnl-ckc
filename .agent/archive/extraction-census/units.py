"""Group ace rows of census.tsv into content units: rows whose texts share ≥70% of 30-char windows.

usage: python3 -I .agent/archive/extraction-census/units.py .agent/archive/extraction-census/census.tsv
"""
import re
import sys
import unicodedata


def key(s):
    return re.sub(r"[^0-9a-z]", "", unicodedata.normalize("NFKC", s).lower())


def windows(k, w=30):
    return {k[i : i + w] for i in range(0, max(len(k) - w, 0) + 1, 5)}


rows = [l.split("\t") for l in open(sys.argv[1], encoding="utf-8").read().splitlines()[1:]]
ace = [r for r in rows if r[4] == "ace"]
keys = {r[0]: windows(key(r[2])) for r in ace}
parent = {r[0]: r[0] for r in ace}


def find(x):
    while parent[x] != x:
        x = parent[x]
    return x


ids = [r[0] for r in ace]
for i, a in enumerate(ids):
    for b in ids[i + 1 :]:
        wa, wb = keys[a], keys[b]
        if wa and wb and len(wa & wb) / min(len(wa), len(wb)) >= 0.7:
            parent[find(b)] = find(a)
groups = {}
for r in ace:
    groups.setdefault(find(r[0]), []).append(r)
print("unit\trows\tgroup\tclass\treason")
for n, (root, members) in enumerate(sorted(groups.items(), key=lambda kv: ids.index(kv[0])), 1):
    first = members[0]
    tag = re.search(r"\[g:([a-z-]+)\]", first[5])
    print(f"U{n:03d}\t{' '.join(m[0] for m in members)}\t{tag.group(1) if tag else '-'}\t{first[3]}\t{first[5][:160]}")
