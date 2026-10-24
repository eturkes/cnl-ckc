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
