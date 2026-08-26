# queue

Guideline source queue and status; procedure = `docs/REFERENCE.md` § Operating.
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
  (next: wave-2 adversarial review round — `rev-<id>` teammates over the 151
  ace-wave2 documents per `.agent/rounds.md`: ACE↔extraction fidelity, notes
  honesty, knowledge-only status, README claims; teammate reports + flags =
  `.scratch/goalrounds/ace-wave2/ace-*.md`. Wave 2 landed at coverage
  ace=337 pending=0; completion ruling after the review round closes)
