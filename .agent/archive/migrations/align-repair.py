#!/usr/bin/env python3
"""Alignment repair after realign2 (REV-P + AV): restore dropped multi-word ACE spans, prune cross-concept partial links.
usage: python3 -I align-repair.py <root> <docid> | ckc align cdc-2022-opioid <docid>   (report on stderr)
Replay: on the parent of the repair commit, every document whose align table changed in it → byte-identical tables.
Current rows = the working tree's align/<docid>.tsv. Bases = de3b9e25 + 0972d02a align of the document.
- Restore: each base ACE row whose span holds a space returns in each new sentence whose most similar base sentence
  (difflib ratio >= 0.6) is the row's own and that still holds the span (the occurrence nearest the base position), in the
  current group holding the same src spans, else a new group with the base src rows; current ACE rows inside it leave.
- Ruled: align-repair-keep.tsv keeps reviewed partial links; align-repair-drop.tsv drops ruled wrong links.
- Prune: a current ACE row whose span is a strict part of its ACE token, new against de3b9e25 (span + src set), stays only
  when more than half of the token's content words occur in the group's src text (plural/prefix tolerant)."""
import re, subprocess, sys
from pathlib import Path
root, doc = Path(sys.argv[1]), sys.argv[2]
G = root / 'guidelines/cdc-2022-opioid'
MANUAL = {('cdc2022-opioid-s22-13', 'term-opioid-use', 'short-term-opioid-use')}  # short-term linked to a long-term phrase
KEEP = {('cdc2022-opioid-rec06-imp03', 'precautionary', 'precautionary-additional-opioid')}  # AV link: just in case pain continues longer than expected
KEEP |= {tuple(l.split('\t')[1:4]) for l in (Path(__file__).parent / 'align-repair-keep.tsv').read_text(encoding='utf-8').splitlines()[1:] if l}  # reviewed partial links (AV re-review RP-2)
STOP = {'example', 'or', 'of', 'and', 'for', 'to', 'with', 'in', 'the', 'a', 'an', 'on', 'at', 'by'}
TOK = re.compile(r'[A-Za-z0-9][A-Za-z0-9-]*[A-Za-z0-9]|[A-Za-z0-9]')
def show(rev, rel):
    r = subprocess.run(['git', '-C', str(root), 'show', f'{rev}:{rel}'], capture_output=True, text=True)
    return r.stdout if r.returncode == 0 else None
def parse(text): return [l.split('\t') for l in (text or '').splitlines() if l and not l.startswith('#')]
ace = (G / f'ace/{doc}.ace').read_text(encoding='utf-8')
cur = parse((G / f'align/{doc}.tsv').read_text(encoding='utf-8'))
bases = {rev: (show(rev, f'guidelines/cdc-2022-opioid/ace/{doc}.ace'), parse(show(rev, f'guidelines/cdc-2022-opioid/align/{doc}.tsv')))
         for rev in ('de3b9e25', '0972d02a')}
# rows as (group, side, offset, span); src offsets are stable (the passage did not change)
rows = [[g, s, int(o), sp] for g, s, o, sp in cur]
def srcset(rs, g): return frozenset((o, sp) for gg, s, o, sp in rs if gg == g and s == 'src')
def overlaps(a0, a1, b0, b1): return a0 < b1 and b0 < a1
report = []
# --- prune (relative to de3b9e25)
b_ace, b_rows = bases['de3b9e25']
b_rows = [[g, s, int(o), sp] for g, s, o, sp in b_rows]
base_pairs = {(sp, srcset(b_rows, g)) for g, s, o, sp in b_rows if s == 'ace'}
def words(t): return [w for w in re.split(r'[\s\-–—,.;:()]+', t.lower()) if w and w not in STOP]
def has(word, text):
    tw = words(text)
    return any(x == word or x == word + 's' or x + 's' == word or x == word + 'es' or
               (len(word) >= 5 and (x.startswith(word[:5]) and len(x) >= 5)) for x in tw)
keep = []
for r in rows:
    g, s, o, sp = r
    if s == 'ace':
        tok = next((m for m in TOK.finditer(ace) if m.start() <= o < m.end()), None)
        if tok and tok.group() != sp and (sp, srcset(rows, g)) not in base_pairs:
            srcs = [x[3] for x in rows if x[0] == g and x[1] == 'src']
            src_text = ' '.join(srcs)
            norm = lambda z: ' '.join(words(z))
            same = any(norm(sp) in (norm(z), norm(z).rstrip('s')) or norm(sp) + 's' == norm(z) for z in srcs)
            tw = words(tok.group())
            frac = sum(has(w, src_text) for w in tw) / max(len(tw), 1)
            if ((doc, sp, tok.group()) in MANUAL or (not same and frac <= 0.5)) and (doc, sp, tok.group()) not in KEEP:
                report.append(f'prune g{g} {sp!r} ⊂ {tok.group()!r} → {src_text[:80]!r} ({frac:.2f})'); continue
    keep.append(r)
rows = keep
# --- restore multi-word spans
nxt = max([int(r[0]) for r in rows] + [-1]) + 1
def whole_occ(text, span):
    out, i = [], 0
    while (j := text.find(span, i)) >= 0:
        if (j == 0 or not re.match(r'[A-Za-z0-9-]', text[j - 1])) and (j + len(span) == len(text) or not re.match(r'[A-Za-z0-9-]', text[j + len(span)])):
            out.append(j)
        i = j + 1
    return out
import difflib
def sentences(text):
    out, pos = [], 0
    for s in text.split('\n'):
        out.append((pos, s)); pos += len(s) + 1
    return out
new_sents = [(p0, s) for p0, s in sentences(ace) if s]
done = set()
for rev, (bace, brows) in bases.items():
    if not bace: continue
    brows = [[g, s, int(o), sp] for g, s, o, sp in brows]
    base_sents = [(p0, s) for p0, s in sentences(bace) if s]
    # each new sentence maps to its most similar base sentence (ratio >= 0.6)
    best = {}
    for np0, ns in new_sents:
        r, bi = max((difflib.SequenceMatcher(None, bs, ns, autojunk=False).ratio(), i) for i, (bp0, bs) in enumerate(base_sents))
        if r >= 0.6: best[np0] = bi
    for g, s, o, sp in brows:
        if s != 'ace' or ' ' not in sp: continue
        bi = next(i for i, (bp0, bs) in enumerate(base_sents) if bp0 <= o < bp0 + len(bs) + 1)
        bp0, bs = base_sents[bi]; rel = (o - bp0) / max(len(bs), 1)
        bsrc = srcset(brows, g)
        for np0, ns in new_sents:
            if best.get(np0) != bi: continue
            occs = [np0 + j for j in whole_occ(ns, sp)]
            if not occs: continue
            occ = min(occs, key=lambda x: abs((x - np0) / max(len(ns), 1) - rel))
            if (sp, occ) in done or any(r[1] == 'ace' and r[3] == sp and r[2] == occ for r in rows): continue
            done.add((sp, occ))
            tgt = next((gg for gg in sorted({r[0] for r in rows}, key=int) if srcset(rows, gg) == bsrc), None)
            if tgt is None:
                clash = [r for r in rows if r[1] == 'src' and any(overlaps(r[2], r[2] + len(r[3]), so, so + len(ss)) for so, ss in bsrc)]
                if clash: report.append(f'skip {sp!r}@{occ}: src clash with g{clash[0][0]} {clash[0][3]!r}'); continue
                tgt = str(nxt); nxt += 1
                rows += [[tgt, 'src', so, ss] for so, ss in sorted(bsrc)]
            gone = [r for r in rows if r[1] == 'ace' and overlaps(r[2], r[2] + len(r[3]), occ, occ + len(sp))]
            rows = [r for r in rows if r not in gone]
            rows.append([tgt, 'ace', occ, sp])
            report.append(f'restore g{tgt} {sp!r}@{occ}' + (f' (drops {[x[3] for x in gone]})' if gone else ''))
# --- ruled drops (link classification of the links in newly written sentences + AV re-review RP-3)
DROP = {(f[1], int(f[2]), f[3]) for f in (l.split('\t') for l in (Path(__file__).parent / 'align-repair-drop.tsv').read_text(encoding='utf-8').splitlines()[1:] if l)}
gone = [r for r in rows if r[1] == 'ace' and (doc, r[2], r[3]) in DROP]
if gone: report.append(f'ruled drops {[(r[3], r[2]) for r in gone]}')
rows = [r for r in rows if r not in gone]
# --- a group needs both sides
sides = {}
for g, s, o, sp in rows: sides.setdefault(g, set()).add(s)
lone = {g for g, ss in sides.items() if ss != {'ace', 'src'}}
if lone: report.append(f'drop one-sided groups {sorted(lone, key=int)}')
rows = [r for r in rows if r[0] not in lone]
# --- to `ckc align` input (raw-substring occurrences)
src_text = None
def occ_of(text, span, off):
    n, i = 0, 0
    while (j := text.find(span, i)) >= 0:
        n += 1
        if j == off: return n
        i = j + len(span)
    return None
def payload(G, doc):
    cov = [l.split('\t') for l in (G / 'coverage.tsv').read_text(encoding='utf-8').splitlines() if l and not l.startswith('#')]
    regs = [r for r in cov if len(r) >= 5 and r[4] == f'ace({doc})']
    reg, f, sect = regs[0][0], regs[0][1], regs[0][3]
    lines = (G / f).read_text(encoding='utf-8').split('\n')
    if reg.startswith('B3-'):
        m = re.search(r'Recommendation (\d+)', sect); n = f'{m.group(1)}. ' if m else ''
        return next(l[len(n):] for l in lines if l.startswith(n))
    k = next(i for i, l in enumerate(lines) if l.startswith(f'[{reg} |'))
    body = []
    for l in lines[k + 1:]:
        if l.startswith('['): break
        body.append(l)
    while body and not body[-1].strip(): body.pop()
    return '\n'.join(body)

src = payload(G, doc)
out = []
for g, s, o, sp in sorted(rows, key=lambda r: (int(r[0]), r[1] != 'ace', r[2])):
    text = ace if s == 'ace' else src
    if text[o:o + len(sp)] != sp: report.append(f'OFFSET MISMATCH g{g} {s} {o} {sp!r}'); continue
    k = occ_of(text, sp, o)
    if k is None: report.append(f'NO OCC g{g} {s} {o} {sp!r} (overlapping raw match)'); continue
    out.append(f'{g}\t{s}\t{k}\t{sp}')
print('\n'.join(out))
print(f'{doc}: {len(cur)} → {len(out)} rows; ' + '; '.join(report), file=sys.stderr)
