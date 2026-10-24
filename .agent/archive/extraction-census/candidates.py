"""Extraction census candidates: every PDF text span outside the extracted payloads.

usage: python3 -I .agent/archive/extraction-census/candidates.py <repo-root> <out-dir>
writes <out-dir>/candidates.tsv + <out-dir>/mapping.tsv (payload → PDF match per extracted payload); prints a summary.
Deterministic for one pdftotext build (poppler 26.10.0: sha256 in check.py).
"""
import re
import subprocess
import sys
import unicodedata
from pathlib import Path

GID = "guidelines/cdc-2022-opioid"
PDF = f"{GID}/source/rr7103a1-H.pdf"
EXTRACTIONS = [
    "box3-extraction.txt",
    "front-matter-extraction.txt",
    "rec01-05-body-extraction.txt",
    "rec06-12-body-extraction.txt",
]
RUNNING = re.compile(
    r"^(Recommendations and Reports|MMWR\s*/.*|"
    r"US Department of Health and Human Services/Centers for Disease Control and Prevention|"
    r"\d{1,3})$"
)
# Back matter by section: main reference list = p64 "References" heading .. end of p77; rosters
# (workgroup, peer reviewers, BSC) = p78; appendix reference list = p90 "References" .. end of p99.
WINDOW = 40
DIRECTIVE = re.compile(
    r"\b(should|must|shall|recommend\w*|advis\w*|consider\w*|avoid\w*|ensure\w*|encourag\w*|"
    r"prefer\w*|required?|requires|need(?:s|ed)? to|important|essential|critical|necessary|"
    r"appropriate\w*|caution\w*|careful\w*|warrant\w*|urge\w*|discourag\w*|contraindicat\w*)\b",
    re.I,
)
WEAK = re.compile(r"\b(can|could|might|may)\b", re.I)
REPORTED = re.compile(
    r"\b(experts?|guidelines?|polic\w*|laws?|legislation|regulat\w*|FDA|CMS|ASAM|ACOG|ACP|ASH|"
    r"NCCN|AHRQ|SAMHSA|HHS|states? have)\b"
)
IMPERATIVE = re.compile(
    r"^(?:[•ï]\s*)?(Ask|Assess|Avoid|Check|Confirm|Consider|Discuss|Document|Engage|Ensure|"
    r"Evaluate|Explain|Identify|Inform|Offer|Prescribe|Provide|Reassess|Refer|Review|Screen|"
    r"Start|Taper|Use|Work)\b"
)


# Sentence ends: [.?!] + space + an opening capital, quote or bracket, except after an initial
# or a fixed abbreviation; bullets (• and the sub-bullet glyph ï) also end a row.
SENTENCE = re.compile(
    r"(?<![A-Z]\.)(?<!e\.g\.)(?<!i\.e\.)(?<! al\.)(?<! No\.)(?<! Vol\.)(?<! vs\.)"
    r"(?<=[.?!])\s+(?=[A-Z“\"(\[])|\s*[•ï]\s*"
)


def key_map(text):
    """Lowercased NFKC alphanumerics of text + the source offset of each."""
    keys, offs = [], []
    for i, ch in enumerate(text):
        for c in unicodedata.normalize("NFKC", ch).lower():
            if "0" <= c <= "9" or "a" <= c <= "z":
                keys.append(c)
                offs.append(i)
    return "".join(keys), offs


def key(text):
    return key_map(text)[0]


def document(root):
    raw = subprocess.run(
        ["pdftotext", "-enc", "UTF-8", str(root / PDF), "-"],
        check=True, capture_output=True, text=True,
    ).stdout
    pages = raw.split("\f")
    text, starts = [], []
    pos = 0
    for n, page in enumerate(pages, 1):
        lines = [l.strip() for l in page.split("\n")]
        body = " ".join(l for l in lines if l and not RUNNING.match(l))
        starts.append((pos, n))
        text.append(body + " ")
        pos += len(body) + 1
    return "".join(text), starts


def page_of(offset, starts):
    page = 1
    for pos, n in starts:
        if pos <= offset:
            page = n
    return page


def payloads(root):
    for name in EXTRACTIONS:
        lines = (root / GID / "source" / name).read_text(encoding="utf-8").split("\n")
        body = lines[lines.index("") + 1 :]
        for line in body:
            if line and not line.startswith("["):
                yield name, re.sub(r"^\d{1,2}\. ", "", line)


def occurrences(k, doc_key):
    found, i = [], doc_key.find(k)
    while i >= 0:
        found.append(i)
        i = doc_key.find(k, i + 1)
    return found


def main():
    root, out = Path(sys.argv[1]), Path(sys.argv[2])
    doc, starts = document(root)
    doc_key, offs = key_map(doc)
    covered = bytearray(len(doc_key))
    mapping = []
    for name, text in payloads(root):
        k = key(text)
        hits = occurrences(k, doc_key) if k else []
        how = "whole"
        pieces = [(k, hits)]
        if not hits:
            how = "pieces"
            frags = [key(f) for f in re.split(r"(?<=[.?!;:])\s+", text)]
            pieces = [(f, occurrences(f, doc_key)) for f in frags if len(f) >= 20]
        missing = 0
        for f, fh in pieces:
            if not fh:
                # Column-crossing or interleaved print: cover the windows that do match.
                how = "windows"
                for w in range(0, max(len(f) - WINDOW, 0) + 1, WINDOW // 4):
                    piece = f[w : w + WINDOW]
                    wh = occurrences(piece, doc_key)
                    if not wh:
                        missing += WINDOW // 4
                    for h in wh:
                        covered[h : h + len(piece)] = b"\x01" * len(piece)
            for h in fh:
                covered[h : h + len(f)] = b"\x01" * len(f)
        mapping.append((name, how, len(k), missing, text[:120]))
    first = {n: pos for pos, n in starts}
    sections = [
        (doc.find("References", first[64]), first[78], "reference-list"),
        (first[78], first[79], "roster"),
        (doc.find("References", first[90]), first[100], "reference-list"),
    ]
    runs, a = [], None
    for i, c in enumerate(covered):
        if not c and a is None:
            a = i
        if c and a is not None:
            runs.append((a, i))
            a = None
    if a is not None:
        runs.append((a, len(covered)))
    rows = []
    for a, b in runs:
        start = offs[a]
        end = offs[b] if b < len(offs) else len(doc)
        span = doc[start:end]
        for s in SENTENCE.split(span):
            s = s.strip()
            if not key(s):
                continue
            at = doc.find(s, start, end)
            p = page_of(at, starts)
            k = key(s)
            section = [m for lo, hi, m in sections if lo <= at < hi]
            if section:
                mech = section[0]
            elif k.isdigit():
                mech = "digits"
            else:
                mech = ""
            marks = "".join(
                m for m, rx in (("D", DIRECTIVE), ("W", WEAK), ("R", REPORTED), ("I", IMPERATIVE))
                if rx.search(s)
            )
            rows.append((p, len(k), marks, mech, s.replace("\t", " ")))
    out.mkdir(parents=True, exist_ok=True)
    seq = {}
    with open(out / "candidates.tsv", "w", encoding="utf-8") as f:
        f.write("id\tpage\tkeylen\tmarks\tmech\ttext\n")
        for p, n, marks, mech, s in rows:
            seq[p] = seq.get(p, 0) + 1
            f.write(f"X{p}-{seq[p]:03d}\t{p}\t{n}\t{marks}\t{mech}\t{s}\n")
    with open(out / "mapping.tsv", "w", encoding="utf-8") as f:
        f.write("file\thow\tkeylen\tunmapped_keylen\tpayload_head\n")
        for row in mapping:
            f.write("\t".join(map(str, row)) + "\n")
    whole = sum(1 for m in mapping if m[1] == "whole")
    print("mechanical", {m: sum(1 for r in rows if r[3] == m) for m in ("reference-list", "roster", "digits")})
    lost = sum(1 for m in mapping if m[3])
    judged = sum(1 for r in rows if not r[3])
    print(f"payloads={len(mapping)} whole={whole} pieces={len(mapping) - whole} with_unmapped={lost}")
    print(f"doc_key={len(doc_key)} covered={sum(covered)} rows={len(rows)} judged={judged}")


main()
