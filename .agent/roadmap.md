# roadmap

Charter: `.agent/initial-prompt.md`. Shipped: fetch→ACE→Prolog guideline
pipeline (`tools/goal.py compile <id> | check`, `tools/regen.py --check`);
guideline coverage runs as built-in `/goal` rounds — procedure: root
`README.md` § Operating; queue: `.agent/queue.md`. This file plans further
project development; new tasks are planned in a new session on demand.

## M1 — American clinical guideline source compendium (easy tier) — IN-PROGRESS

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

Tiering (user ruling, measured by `.scratch/agents/scout-m1u2.md`): the
420-org sweep population carries ≈4,400 guideline rows for the 97 known
issuers alone and runs ≈24 teammate waves, so M1 harvests the EASY TIER
and defers the rest to M2 rather than stalling. An org is easy when its
index enumerates in ≤2 anonymous fetches — `static-list`, `search-api`,
`pdf-index`, or a scripted-UA 403 that the `r.jina.ai` reader clears —
and its artifacts classify without an authenticated session. Everything
else is deferred by recording the org's `swept` cell as
`blocked(<why>)`, which the compendium's terminal condition already
accepts, so M1 closes honestly with the deferral set enumerated rather
than hidden. Deferred classes, each with named orgs, live in M2.

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

- M1.1 DONE — schema + protocol + seed roster. `.agent/compendium.md`
  ships the header (eligibility + rulings, three-layer discovery
  protocol, independence definition, row formats, enumeration-source
  legend, queue promotion + ordering) with 447 organization rows
  (federal 15/3/12, society 75/283/6, other 7/37/9 as yes/unverified/no)
  and 88 seed guideline rows. Gate green: every `CPGs=yes` row carries ≥2
  independent enumeration sources under the owner rule, every such org
  holds ≥1 seed, and all structural/ordering/vocabulary predicates pass
  (`.scratch/check_compendium.py`; M1.5 ports them into `goal.py check`).
  main=83% 198K/240K, mate=86% 207K/240K.

Harvest units run one shape: `prod` teammates sweep their assigned
indexes behind `.scratch/check_compendium.py`, MAIN merges with
`.scratch/merge_rows.py` (the only row inserter; reports are the source
of truth, so every repair edits a report and re-merges), re-verifies a
sample per wave against live artifacts, and records each swept org as
`<date> <method>; <n> index entries -> <e> eligible + <x> excluded` or
`blocked(<why>)`. Cap = ~200 emitted rows or 4 ordinary orgs per
teammate.

- M1.2a DONE — federal easy tier, 13 orgs, 318 rows; compendium 88 → 395
  guideline rows. Swept with dated manifests: DHA JTS 103 · BOP 47 ·
  HICPAC 47 · SAMHSA 45 · ACIP 27 · VA/DoD 26 · HRSA 14 · ClinicalInfo 6
  · IHS 1 · NAEPP 1 · PHS tobacco 1. Stale ACIP `index URL` repaired.
  Gates green: `check_compendium.py --require-swept` over all 13 assigned
  orgs, and the 25-test `gate_m1u2a.py`. MAIN re-verified 12 rows across
  6 orgs against live artifacts (10 clean on title+year+access; 2 thin
  reads verify nothing and are reported as such).
  Scope corrections against the plan, each evidence-backed:
  - USPSTF 90 → `blocked(...)`. Anonymous enumeration is exhausted: the
    JSON API is key-gated behind a 202, the Drupal view carries no
    `views` key in `drupal-settings-json`, there is no sitemap, and the
    reader proxy returns the shell only. Seed rows preserved.
  - NIOSH → `blocked(...)`, NOT `CPGs=no`: the digital radiography
    guideline (DHHS Pub 2011-198) does qualify, so the org stays
    `unverified` and joins M1.3 rather than being ruled out.
  - VA/DoD 38 → 26: the live index carries 27 topics, one of them a
    superseded 2012 external edition that collapses per the versions rule.
  - SAMHSA ≈32 → 45, from 74 index entries: the TIP facet folds 48
    numbered products into 26 concepts, the Advisory facet adds 14.
  - JTS 106 → 103 rows, of which 20 are excluded: 15
    `excluded(veterinary; not human patient care)` — the index carries
    military-working-dog CPGs, and patient means human patient — plus 3
    parentless iCOVER derivatives and 2 process/author documents.
  Rows left deliberately unresolved, both non-promoting by construction:
  - 36 SAMHSA rows `unverified` + `provisional(...)`. `library.samhsa.gov`
    meters anonymous requests per-IP and answers past the budget with 202
    + zero bytes, its own site root included. 9 artifacts earned an
    individual `open` verdict; the rest were never reached, and access is
    classified from the artifact, so they cannot read `open`.
    `.scratch/fix_m1u2a_samhsa_access.py` is convergent — rerun it after
    `.scratch/probe_until_settled.py` collects more evidence and exactly
    the newly proven rows promote.
  - 13 HRSA/WPSI rows `provisional(year unresolved: ...)`. No version
    year appears on a WPSI recommendation page under either a reader
    proxy or a real browser, beside a service in the HRSA index, or on
    the WPSI recommendations table; the sweep had reported those pages
    establish version years. The cervical-cancer row keeps its year,
    which the index states as a dated adoption.
  main=87% 210K/240K, mate=100% 240K/240K.
- M1.2c OPEN — M1.2a review remediation. Adversarial review raised 8
  findings; MAIN re-derived each against live artifacts before ruling.
  Five are closed in the M1.2a commit range, each with its acceptance
  check in `.scratch/gate_m1u2a.py` (26 tests) or the merge gate:
  - F2 BOP — 5 rows reclassified `excluded(administrative; ...)`:
    care-level classification states its purpose as classifying patients
    for institution assignment, compassionate release supplies criteria
    for a sentence determination, two are directories of external
    programs, and pandemic module 4 handles the deceased. Manifest 44+3
    → 39+8, recomputed from rows by
    `.scratch/fix_m1u2c_eligibility.py`.
  - F3 SAMHSA TIP 51 — row repointed from the KAP Keys derivative to the
    parent protocol (NCBI Bookshelf NBK83252, 2009), access `open`
    earned by its own probe.
  - F5 access evidence — the fixer keyed idempotence on the exact
    `provisional(...)` string, so rows an earlier version wrote against
    another host were frozen beyond promotion; guard now matches any
    `provisional(access unverified:`.
  - F6 `probe_urls.py` — the per-host lock covered only the sleep, so
    slow same-host requests overlapped while the docstring claimed one
    at a time; the lock now spans the request and the gap runs from
    completion. Measured: 4 concurrent before, 1 after.
  - F7 manifest provenance — `sole <= claimed <= counting_co_issued`
    accepted both an org counting a co-issued row its index omitted and
    the co-issuer dropping the row its index carried. The exact check
    lives in `.scratch/merge_rows.py`, where the reports still hold the
    provenance the merged table loses: manifest `e/x` must equal the
    rows that org's own report block emits. Exact only while co-issuers
    are swept by separate reports.
  Open, and the reason this unit stays open:
  - F1 ACIP (HIGH, confirmed) — the 27 swept entries are family
    subindexes, not guidelines. The live Rabies page carries a `CURRENT
    Rabies Vaccine Recommendations` section listing the 2022 PrEP
    update, the 2010 4-dose PEP update and the 2008 baseline, with
    `ARCHIVED` explicitly empty; only the 2022 row exists. Reviewer
    triage found 40 further current MMWR artifacts across 17 of the 27
    pages (`.scratch/m1u2c/acip-current-missing.tsv`), none present in
    the compendium. Re-sweep at artifact granularity, adjudicating each
    of the 40 rather than assuming eligibility from a title.
    Route ruling required first: enumerating the child pages costs 27
    fetches, so ACIP does not meet the easy-tier `≤2 anonymous fetches`
    test. Pick one and record it — an aggregate ≤2-fetch endpoint if one
    exists, an explicit easy-tier exception with `swept` reading
    `per-topic-pages`, or `blocked(...)` deferring ACIP to M2.
  - F4 HICPAC rr6007 — MAIN rules the artifact ELIGIBLE against the
    review: it presents vaccination recommendations per disease in two
    graded categories, and consolidating prior ACIP output is what a
    summary guideline does, so it is not a review without
    recommendations. Its `does not contain any new recommendations or
    policies` sentence describes novelty, not content. The real defect
    is the issuer cell: the title reads `Recommendations of the Advisory
    Committee on Immunization Practices (ACIP)` while the row names
    HICPAC alone. Fix issuer attribution with the ACIP re-sweep, since
    both manifests move together.
  - F8 IHS (LOW) — manifest counts a navigation link to an external
    clearinghouse as an index entry (`n=2`, one row emitted). Either
    count artifact entries only, or admit navigation dispositions into
    the header's reconciliation vocabulary and its gate.
  Gate: each finding lands as a red test a fix turns green, or is
  recorded as ruled-out with the probe that dismissed it.
- M1.2b OPEN — society/other easy tier, 11 orgs ≈ 320 rows. ACOG 12 ·
  ACC ≈88 · AARC 37 · ASA ≥28 · AES 7 · ACG 62 · AASM 27 · CFF 38 ·
  APTA Orthopedics 20 · NPIAP 1 · AOCD → `CPGs=no`. The six
  `unverified` orgs here (ACG, AASM, CFF, APTA Orthopedics, NPIAP, AOCD)
  need their second owner-distinct enumeration source before `CPGs=yes`
  lands. Gate: as M1.2a.
- M1.3 OPEN — determination pass over the remaining 314 `unverified`
  orgs. Cheap per org (≤2 anonymous fetches): decide `CPGs=yes|no`, record
  `swept` date+method, and sweep inline when the org is both easy and
  small (`<15` rows); everything else takes `blocked(<why>)` and joins
  the M2 register. This is what fixes the org universe, so it precedes
  the cross-check. Gate: zero `unverified` org rows remain; every row
  swept or blocked with a named reason.
- M1.4 OPEN — completeness cross-check, scoped to the easy tier.
  Aggregator sweeps diffed against the org-derived list; gaps feed back as
  rows; MAIN dedupes to one row per guideline (latest current version) and
  rules eligibility edge cases. A gap landing on a deferred org joins the
  M2 register instead of the gap list. Gate: cross-check documented in the
  compendium header with the easy-tier gap list driven to zero; dedupe
  pass recorded.
- M1.5 OPEN — integration. Root `README.md` § Operating + the canonical
  `/goal` argument rewritten → terminal condition = compendium
  exhaustion; `.agent/queue.md` feeds from the compendium (promotion
  rule); cdc-2022-opioid linked to its compendium row; the M1.1 + M1.2
  gate predicates ported from `.scratch/check_compendium.py` into E-- →
  `tools/goal.py check`; `memory.md` delta. Gate: `tools/goal.py check`
  green incl. compendium predicates; `regen.py --check` green; docs
  mutually consistent; scoped commits.

Unit/milestone close per session-prompt WORK-UNIT/MILESTONE-REVIEW
protocol (gauges recorded, commits `compendium (M1.<u>): …`).

## M2 — deferred hard-tier harvest — UNPLANNED

The register of what M1 consciously skips. Every entry here is an org
whose `swept` cell reads `blocked(<why>)`, so the compendium stays
terminal-consistent while naming its own gaps. Plan this milestone only
after M1 is REVIEWED and `/goal` has consumed a useful run of easy-tier
rows — the real per-document cost of `/goal` is what should size it.

Deferred classes, from the 46-org measurement
(`.scratch/agents/scout-m1u2.md`):

- Browser-gated indexes — the authenticated browser is the only route:
  AAP, NKF, HRS, ASCO, ACCP (Cloudflare), CFF index (Akamai), OPA
  (Cloudflare), NIAID (JS CAPTCHA), CPIC (JS-only shell).
- Login corpora — one gate covers a whole corpus, so each is a single
  holder pass: NCCN (free account, ≥91 rows), AORN (eGuidelines
  subscription, 36 rows).
- Heavy tail — enumeration is cheap but per-artifact resolution is not,
  so each wants a solo teammate split by topic or pagination: ACR 279,
  VA PBM ≈582, ASCO ≥139, IDSA ≈105 (109 fetches), AAN 65 (7 AJAX
  pages), ACR-scale others surfacing during M1.3.
- Version-ambiguous collections — the index mixes current with
  superseded, so a version ruling must precede row emission: CDC Stacks
  (786 records over 40 pages).
- Missing or stale enumeration surfaces — a replacement index must be
  found first: AAFP (methodology overview, no artifact index),
  ADA-Dental (404), AUA + ATS + BTF (indexes expose only part of the
  corpus), Endocrine Society (count needs the listing API).
- Access classification deferred wholesale — M1 records `access` from
  the artifact only where an anonymous fetch settles it; the remaining
  rows keep `unverified` + `provisional(…)` and never promote, which is
  the intended M2 worklist rather than a defect.
- Storage format — at ≈4,400 rows the guideline table outgrows a
  markdown file every agent reads whole. Moving guideline rows to
  `.agent/compendium.tsv` (rules + org table staying in the `.md`,
  matching the `coverage.tsv` precedent) is cheap now and expensive
  later; M1 stays on markdown per the easy-first ruling.
- Row granularity — the M1.1 header rules appropriate-use criteria and
  committee opinions each into their own row, which is what puts ACR at
  279 and drives the ≈4,400 total. A coarser rule (one row per guideline
  series) would cut the compendium to ≈500 rows; revisiting it is a
  header-level decision, not a harvest decision.
