# roadmap

Charter: `.agent/initial-prompt.md`. Shipped: fetch→ACE→Prolog guideline
pipeline (`tools/goal.py compile <id> | check`, `tools/regen.py --check`);
guideline coverage runs as built-in `/goal` rounds — procedure: root
`README.md` § Operating; queue: `.agent/queue.md`. This file plans further
project development; new tasks are planned in a new session on demand.

## M1 — American clinical guideline source compendium — IN-PROGRESS

Goal: `.agent/compendium.md` = the complete master list of eligible
American clinical guidelines available online — one row per guideline:
issuing org | title + year | canonical source URL | access
(`open|paywalled(<gate>)|login(<gate>)`) | status
(`unqueued|queued|in-progress|done|blocked(<why>)|excluded(<why>)`) —
discovery-complete per a documented, auditable search protocol. Downloads
stay deferred: an entry is fetched only when promoted into
`.agent/queue.md`. This gives the standing `/goal` a finite terminal
condition — done = compendium exhaustion (every row
`done|blocked|excluded`) — replacing the open-ended web-search clause.
Planned directly with full session context at user direction; planning
waves intentionally skipped.

Method (all units): research fan-out per `.agent/rounds.md` `res` role +
the session-prompt brief spec. WebSearch budget = 200/session shared →
explicit per-teammate allowances (~30–40); bulk index crawling goes
through BrowserOS/WebFetch instead. BrowserOS (the authenticated
day-to-day browser incl. university journal access) = the route for
paywalled/member/login-gated listings — it is one live browser profile =
single-holder-per-wave resource: one designated holder (MAIN or one named
teammate) per wave; every other teammate stays on WebSearch/WebFetch and
queues BrowserOS verifications to the holder's worklist. Auth, captcha,
2FA, or any human gate → teammate flags it in its report; MAIN batches
the gates and asks the user immediately while other waves continue.

- M1.1 OPEN — schema + protocol + seed roster. MAIN authors
  `.agent/compendium.md` header: eligibility criteria (American issuing
  body; clinical practice guideline bearing patient-care recommendations;
  current version available online; edge rules recorded as rulings —
  US-led multi-society/international collaborations, federal panels,
  retired/superseded versions), row format, queue-promotion + ordering
  rule, per-org `swept` provenance line. `res` wave enumerates the
  issuing-org universe: federal (CDC, USPSTF, NIH panels, AHRQ, VA/DoD,
  SAMHSA, …), specialty societies (AMA specialty-society frame), and
  aggregators (ECRI Guidelines Trust, PubMed practice-guideline filter,
  Guideline Central). Gate: header committed; org roster confirmed
  against ≥2 independent enumeration sources; one seed row per org.
- M1.2 OPEN — per-org index harvest. `res` waves sweep each org's
  guideline index → one compendium row per eligible guideline, URL
  verified live, access class recorded; the BrowserOS holder clears the
  paywalled/login verification worklist. Gate: every M1.1 org swept or
  `blocked(<why>)`; MAIN re-verifies a sample of rows per wave.
- M1.3 OPEN — completeness cross-check. Aggregator sweeps diffed against
  the org-derived list; gaps feed back as rows; MAIN dedupes to one row
  per guideline (latest current version) and rules eligibility edge
  cases. Gate: cross-check documented in the compendium header with the
  gap list driven to zero; dedupe pass recorded.
- M1.4 OPEN — integration. Root `README.md` § Operating + the canonical
  `/goal` argument rewritten → terminal condition = compendium
  exhaustion; `.agent/queue.md` feeds from the compendium (promotion
  rule); cdc-2022-opioid linked to its compendium row; `memory.md`
  delta. Gate: `tools/goal.py check` green; docs mutually consistent;
  scoped commits.

Unit/milestone close per session-prompt WORK-UNIT/MILESTONE-REVIEW
protocol (gauges recorded, commits `compendium (M1.<u>): …`).
