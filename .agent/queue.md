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

- cdc-2022-opioid | https://www.cdc.gov/mmwr/volumes/71/rr/pdfs/rr7103a1-H.pdf | in-progress (extraction-census round, `.agent/archive/extraction-census/`: 214 new payload regions + the nine conditions of S37-14 = all 234 in-scope census rows (`map.tsv`); 110 regions + S37-14 pending ACE; pending: S9-05 class names + front-matter scope (pp4–10), recommendations 1–2 (pp19–30), recommendations 3–5 + S37-14 (pp31–40), recommendations 6–9 (pp40–52), recommendations 10–12 + the DSM-5 criteria + conclusion (pp52–62), README coverage statement + waiver caveat)
