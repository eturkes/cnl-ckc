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

- cdc-2022-opioid | https://www.cdc.gov/mmwr/volumes/71/rr/pdfs/rr7103a1-H.pdf | in-progress
  (next: ACE wave 2 — 151 pending rec06-12 regions, coverage meter = the count;
  kit + docid rules + worklist = `.agent/rounds.md` § Wave kit)
