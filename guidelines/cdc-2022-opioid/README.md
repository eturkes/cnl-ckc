# CDC 2022 opioid guideline

Dowell D, Ragan KR, Jones CM, Baldwin GT, Chou R. CDC Clinical Practice
Guideline for Prescribing Opioids for Pain — United States, 2022. MMWR Recomm
Rep 2022;71(No. RR-3):1–95. DOI `http://dx.doi.org/10.15585/mmwr.rr7103a1`.

## Source of record

- `source/rr7103a1-H.pdf` — official CDC PDF, retrieved 2026-07-22 from
  `https://www.cdc.gov/mmwr/volumes/71/rr/pdfs/rr7103a1-H.pdf` (two fetches,
  byte-identical). SHA-256
  `f4e5098d13e9b3dc5cc27bb90137df57d3667350b2add885fd367f279402d18d`,
  1,418,584 bytes. HTML manifestation:
  `https://www.cdc.gov/mmwr/volumes/71/rr/rr7103a1.htm`.
- `source/box3-extraction.txt` — verbatim extraction of Box 3 (the twelve
  recommendations; PDF pages 13–14). SHA-256
  `6776a52087f893b462c1f1092c1ed8799353438809368e0cfd123ca4244d0ac5`. Number
  labels in the extraction are presentation scaffolding, not CDC text edits.

## Rights and attribution

CDC states at `https://www.cdc.gov/mmwr/about.html`: "All material in the MMWR
Series is in the public domain and may be used and reprinted without special
permission; citation as to source, however, is appreciated." Source: CDC. CDC
does not endorse this project. The project adds no CDC/HHS marks or
endorsement branding; the unmodified source PDF retains the official marks it
was published with.

## Formal derivatives (project-authored)

- `ace/cdc2022-opioid-recNN.ace` (NN = 01–12) — one ACE document per Box 3
  recommendation NN. Each document is a deliberately minimal formal projection
  of its recommendation — an actor-class fact, one universal rule (rec05 adds
  a negation-as-failure guard), and a probe query — not a complete
  formalization of the clinical semantics, conditions, or qualifiers of the
  CDC text. Read the ACE next to the extraction to judge coverage.
- `lexicon.ulex` — the guideline's APE user lexicon (domain compounds and
  proper names shared by all twelve documents).
- `pl/cdc2022-opioid-recNN.pl` — Prolog compiled from the ACE by
  `vendor/ape/prolog/ace_to_pl.pl`; regenerate via
  `python3 -P tools/goal.py compile cdc-2022-opioid`; never edit by hand.

These derivatives are project-authored, not altered CDC text, and must not be
presented as CDC-authored formalizations.
