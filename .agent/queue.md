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

- cdc-2022-opioid | https://www.cdc.gov/mmwr/volumes/71/rr/pdfs/rr7103a1-H.pdf | in-progress (compendium row: Centers for Disease Control and Prevention, `id=cdc-2022-opioid`; 718 regions census-reconciled; counts = guideline README Coverage; coverage-closure gate live — check's coverage meter reads pending directly; pending, ordered: (1) ACE batches — 258 ace-slated regions (151 rec06-12, 85 rec01-05, 22 front matter), `.scratch/goalrounds/ace-worklist.tsv` carries per-region evidence + flags; impl docids = `cdc2022-opioid-recNN-impKK` reserved-ordinal convention; decide the docid convention for front-matter/rationale/special-population regions before the first batch; author under the frozen schema-v1 knowledge-only contract; generated obligations must discharge per document and in aggregate)
