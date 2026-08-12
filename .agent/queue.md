# queue

Guideline source queue and status; procedure = root `README.md` § Operating.
Entries promote from `.agent/compendium.tsv` per `.agent/compendium.md`
§ Queue promotion; the compendium row mirrors this queue's lifecycle
(`in-progress` → `done` | `blocked(<why>)`).
One entry in progress at a time; complete it before the next fetch. Format:
`- <id> | <source url> | in-progress|queued|blocked(<why>)`; completed
entries are dropped (the guideline directory and its README are the record).
The in-progress entry's pending list is the ordered round queue — its first
item is the next round; coverage counts live solely in the guideline README
Coverage section.

- cdc-2022-opioid | https://www.cdc.gov/mmwr/volumes/71/rr/pdfs/rr7103a1-H.pdf | in-progress (compendium row: Centers for Disease Control and Prevention, `id=cdc-2022-opioid`; 718 regions census-reconciled; counts = guideline README Coverage; coverage-closure gate live — check's coverage meter reads pending directly; pending, ordered: (1) ACE wave 2 — 151 ace-slated rec06-12 regions (90 impl + 61 rationale/special-pop/end-matter; wave 1's 107 front-matter + rec01-05 regions authored, adversarially reviewed, and repaired — rulings in guideline README § Coverage, fix script `.scratch/goalrounds/ace-wave1/apply_rev_fixes.py`; wave-2 dispatch kit READY at `.scratch/goalrounds/ace-wave2/` — brief-common.md (carries the projection rulings + pl-as-compile-output allowlist fix), 6 batch tables with precomputed docids, roster.md with dispatch/harvest/close procedure and a name-burn note), `.scratch/goalrounds/ace-worklist.tsv` carries per-region evidence + flags; docids: impl regions = `cdc2022-opioid-recNN-impKK` reserved-ordinal convention (KK from worklist rec/ord evidence); all other regions = `cdc2022-opioid-<region-id lowercased>` (S47-05 → `cdc2022-opioid-s47-05`; self-locating, no ordinal bookkeeping); author under the frozen schema-v1 knowledge-only contract; generated obligations must discharge per document and in aggregate)
