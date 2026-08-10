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

- cdc-2022-opioid | https://www.cdc.gov/mmwr/volumes/71/rr/pdfs/rr7103a1-H.pdf | in-progress (compendium row: Centers for Disease Control and Prevention, `id=cdc-2022-opioid`; 718 regions census-reconciled; counts = guideline README Coverage; pending, ordered: (1) pending-region rulings, (2) E-- coverage-closure extension of goal.py check, (3) ACE batches rec 6–12 impl — 107 pending impl regions, ~3 teammates — GATED on roadmap M3 projection redesign: no new ACE documents under the pre-M3 contract)
