# roadmap

Charter: `.agent/initial-prompt.md`. Shipped: fetch→ACE→Prolog guideline
pipeline (`tools/goal.py compile <id> | check`, `tools/regen.py --check`);
guideline coverage runs as built-in `/goal` rounds — procedure: root
`README.md` § Operating; queue: `.agent/queue.md`. This file plans further
project development; new tasks are planned in a new session on demand.

## Standing direction

- Product = the KB artifacts (`guidelines/*/{ace,pl}` + lexicons +
  provenance/coverage ledgers); consumers load them into their own
  engines. Compiled Prolog = a public interface once M3 lands:
  engine-portable plain clauses, schema documented + versioned. No
  query/serving layer in this repo — APIs, CDS integrations,
  FHIR/CDS-Hooks adapters = downstream projects; the check's derived
  probes are the sole in-repo query machinery (testing-grade by design).
- Multilingual (Japanese first): a port = a new source-language CNL
  frontend + jurisdiction-native corpus (Japanese guidelines, e.g. the
  Minds clearinghouse), never corpus translation — verification requires
  source-language comparison. DRS = the language-neutral waist
  (`ace_to_pl` consumes DRS, not English); `vendor/ape` grammar + ulex
  morphology = the English-specific components. Repo split (per-language
  repos vs agnostic core) is decided when Japanese work is planned; until
  then nothing English-specific enters `tools/`, `vendor/e--`, schemas or
  ledger formats, and all machinery treats lemma symbols as opaque atoms.
- Domain generalization: the pipeline is normative-document
  formalization, not clinical-only; domain rules (eligibility, actor
  classes, corpus protocol) stay in corpus data (compendium header,
  per-guideline files), never in code.

## M1 — American clinical guideline source compendium (easy tier) — REVIEWED

Goal: `.agent/compendium.md` (rules + org table) + `.agent/compendium.tsv`
(guideline rows) = the master list of eligible American clinical guidelines,
discovery-complete per the header's audited protocol. Fetch stays deferred
until a row is promoted into `.agent/queue.md`. This gives the standing
`/goal` a finite terminal condition = compendium exhaustion.

Terminal state: 475 organizations (127 yes / 309 unverified / 39 no), 1,118
guideline rows; every org row terminal (`CPGs=no` | dated manifest |
`blocked(<why>)`); `goal.py check` meter `475 organizations 1118 rows;
terminal remaining: orgs=0 rows=847 provisional=54`. Row-status residue = the
intended `/goal` worklist, not a defect.

Scope rulings (compendium header owns the durable form):

- Easy tier = index enumerates in <=2 anonymous fetches (`static-list`,
  `search-api`, `pdf-index`, or a scripted-UA 403 the `r.jina.ai` reader
  clears) and artifacts classify without an authenticated session. Bounded
  fan-out qualifies (<=30 anonymous static subindexes, enumerable from the
  index in one fetch). Everything else defers as `blocked(<why>)` -> M2.
- M1 closes at ORG-UNIVERSE completeness, not harvest exhaustion: the
  guideline table freezes at M1.2's 1,118 rows while every org row ends
  terminal. An unswept `yes` org records `blocked(easy-tier harvest
  deferred; M2)` — a named gap rather than a hidden one.
- Per-org determination defers to M2 whole: measured 3.2-3.4 tool calls +
  1.2-1.5 WebSearches/org over 346 orgs = 415-519 searches against the
  200/session ceiling, buying `/goal` nothing it can consume now; M1.3b's
  crosswalk is its costed worklist. Bulk prefilter measured: GC u NGC u
  AAFP-PG u PubMed corporate-author flags 9 of 11 true-yes orgs, every hit
  positive evidence only (an omission is silence).
- Harvest shape: `prod` teammates sweep assigned indexes, MAIN merges with
  `.scratch/merge_rows.py` (the only row inserter; reports = source of truth,
  so a repair edits a report and re-merges), each swept org recording
  `<date> <method>; <n> index entries -> <e> eligible + <x> excluded` |
  `blocked(<why>)`. Fan-out method + budgets = `.agent/rounds.md`.

Units — narratives, per-unit rulings, gauges + SHAs:
`git log --grep "(M1[. ]" -p -- .agent/roadmap.md`; durable rulings =
compendium header; technique + tooling contracts = `.agent/memory.md`.

- M1.1 DONE — schema + protocol + 447-org seed roster + 88 seed rows.
- M1.2a DONE — federal easy tier, 13 orgs -> 395 rows; USPSTF + NIOSH
  blocked on exhausted anonymous enumeration.
- M1.2c DONE — M1.2a review remediation + ACIP re-swept at artifact
  granularity (27 -> 68) -> 435 rows; issuance-vs-index custody rulings
  (`indexed-by=`, `off-index(`, MMWR label vs publication year).
- M1.2b DONE — society/other easy tier, 10 orgs -> 1,118 rows / 474 orgs;
  access re-probe promoted 32 rows; ACOG deferred under contract R1.
- M1.3a DONE — 5 gate defects (each red-first) + 2 org-name collision merges
  -> 473 orgs; identity, independence + canonical sorts centralized in
  `compendium_io.py`.
- M1.3a2 DONE (data) — 32 HICPAC re-attributions + 58 R6 close calls + 2
  validated org appends -> 475 orgs.
- M1.3b DONE (data) — org-universe terminalization (427 edits, idempotent),
  669-candidate cross-check over 16 orgs, `/goal` integration.
- M1.5 DONE (kernel) — `/goal`-consumed predicates ported to E--
  `goal.py check`; deep predicates stay `.scratch/` with regeneration paths
  in memory. 48-fixture dual gate green, 35/35 mutation kills.
- M1 REVIEW DONE (data) — 13/13 claims replayed, gates 4/4 green; F-01 AASM
  advisory exclusion reason corrected, F-02 provenance narration pruned.

## M2 — deferred hard-tier harvest — UNPLANNED

The register of what M1 consciously skips. Every entry here is an org
whose `swept` cell reads `blocked(<why>)`, so the compendium stays
terminal-consistent while naming its own gaps. Plan this milestone only
after M1 is REVIEWED and `/goal` has consumed a useful run of easy-tier
rows — the real per-document cost of `/goal` is what should size it.

Deferred classes, from the 46-org measurement
(`.scratch/agents/scout-m1u2.md`):

- Easy-tier orgs determined but never harvested — the terminal-scope
  ruling's direct output, and M2's cheapest class: every org ending
  `CPGs=yes` + `blocked(easy-tier harvest deferred; M2)`, each carrying
  the index URL and tier M1 recorded — ~94 known at the re-cut (81
  pending-since-M1.1 + the scouts' easy-tier `yes`), growing as the
  determination pass runs. The only class whose per-org sweep cost M1
  has already measured.
- Per-org determination pass — the re-cut's wholesale deferral: every
  row `blocked(determination deferred; M2 — <crosswalk signal>)`, the
  crosswalk its costed, prioritized worklist. Protocol-v2 friction
  rulings stay banked in the scout reports: D2's two-fetch cap covers
  own-site index discovery only while D5 buys one bibliographic fetch;
  transport classification runs direct→reader before any artifact
  fetch; a `CPGs=no` needs affirmative evidence (function + exhaustive
  own-site output taxonomy + a co-issuance check), never search
  silence; `WPSI-program` counts nothing toward the ≥2. Sized by the
  measured 3.2–3.4 calls + 1.2–1.5 searches/org against the 200/session
  WebSearch ceiling.
- Cross-check gap adjudication — M1.3b's gap list over the 23
  swept/blocked orgs: eligibility, version + access ruled per gap, rows
  entering through the off-index mechanism. Delivered: 669 candidate
  gaps/16 orgs (`.scratch/m1u3b/gapcheck/gaps.tsv`; record in
  compendium § Protocol layer 3). Determination-pass leads banked here:
  historical-issuer PMIDs ACRO 14585480 (2003) + SCPC 23892939 (2013);
  HVS + SCPC co-issuer attribution audit (eligible shipped AUC rows
  name both while both sit `unverified` + unsettled-blocked); APTA
  academy renames (APTA Neurology → Academy of Neurologic Physical
  Therapy; scout-1's alias-map shortcut: parent-directory crosswalk →
  active academy name/domain → own CPG page).
- ACCM artifact-attribution audit — Q3b/Q3c unresolved
  (`.scratch/agents/res-m1u3a-1.md`); relationship settled (SCCM special
  body; `CONTROLS` recorded); its 3 guideline rows' issuer cells stand
  unverified against artifact text.
- Corpora whose breadth outruns the easy tier — enumeration is one
  anonymous call but per-artifact version and access adjudication is not,
  so the org defers whole rather than being swept against a narrow facet
  (M1.2b ruling R1): ACOG (~2,500 clinical records across 11 guidance
  types, of which the compendium's recorded index sees 61).
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
- Storage format — guideline rows already live in `.agent/compendium.tsv`
  (rules + org table in the `.md`). At ≈4,400 rows revisit whether the
  org table follows.
- Row granularity — the M1.1 header rules appropriate-use criteria and
  committee opinions each into their own row, which is what puts ACR at
  279 and drives the ≈4,400 total. A coarser rule (one row per guideline
  series) would cut the compendium to ≈500 rows; revisiting it is a
  header-level decision, not a harvest decision.

## M3 — projection redesign (clinician-verifiable ACE) — REVIEWED

Goal: replace fixture-bearing legacy projections with source-anchored,
knowledge-only ACE; freeze an engine-portable public Prolog ABI; derive and
replay proof obligations per document + aggregate; migrate the complete
`cdc-2022-opioid` corpus without changing coverage/census/docid custody.

Terminal state:

- Corpus = 79 ACE = 79 compiled Prolog documents, one document per ruled
  source region, sentences per region free (21 single-sentence, rest 2-25);
  zero authored questions, zero witness fixtures.
- Schema v1 = frozen nine-indicator ABI documented in root README + compiler
  header; every corpus document compiles explicitly on v1. Legacy fact/rule
  emission stays byte-frozen; authored questions reject on both paths.
- Proof = 257 generated obligations, discharged per document and forward +
  reverse aggregate under bounded search (depth 4000 inside 10^6 inferences;
  either bound exceeded = underivable).
- Lexicon = 487 live entries, zero `pn_sg`, zero 77-roster mega lemmas.
- Custody = `audit/projection-notes.tsv` total, duplicate-free, set-equal to
  the ACE inventory; `coverage.tsv` + census keys unchanged through M3.
- Queue gate lifted in this review-close commit; further ACE authoring runs
  under the frozen v1 knowledge-only contract.

Governing authoring rulings:

- Preserve source conditions, objects, numbers + modality; record every
  approximation/drop in projection notes; introduce no unanchored structure.
- Hoist universal restrictors out of modal complements into antecedents; split
  compound "..., and if A then B" statements into separate ACE sentences.
- Render "only ... should ..." restrictions as conditional prohibitions, one
  rule per unmet conjunct; classical negation for unmet facts, NAF only for
  evidentiary absence ("unless there are indications ...").
- Map `might`->`may`, ability/option `can`->`can`, responsibility->`must`;
  modality stays reified data with no deontic inference. Causal rationale never
  becomes a temporal PP. Source `for` takes `prep(for,for)` + a determiner.
- Noun modification = relative clause or compound noun; `of` rejects
  (`condition_shape(relation(A,of,B))`).
- Source upper bounds (`at most n`, `exactly n`, `less than n`) reject at the
  root of an asserted sentence (`root_condition([...])`, ACE scoping them over
  a grouped condition list) and compile inside a rule or operator box; a
  root-position bound is hoisted under its modality or recorded as an
  approximation in the notes. Lower bounds (`at least n`, `more than n`) carry
  no group and compile anywhere.
- Source "or" folds only where the notes name the approximation; disjunctive
  consequents stay unsupported. Defeasibility/backend changes need a named
  consumer question and recompile the unchanged ACE corpus.

Assurance + evidence:

- Durable acceptance = `python3 -P tools/goal.py check` (terminal corpus,
  aggregate replay, 21 committed reds each pinning class + rc + byte-exact
  stderr) + `python3 -P tools/regen.py --check`.
- Closing M3 chain = durable gate -> `.scratch/m3u1/gate_m3u1.py` ->
  `.scratch/m3u2/gate_m3u2.py` -> `.scratch/m3u3/gate_m3u3.py` ->
  `.scratch/m3u7/diff_m3u7.py` -> fresh guideline compile ->
  `git diff --quiet -- guidelines/`.
- Question boundary = `.scratch/m3u7/suite/runner.py` 96 cells; contracts of
  record = `.scratch/contracts/m3u1.md`...`m3u7.md`; regeneration + port
  ownership = `.agent/memory.md` M3 evidence bullet + `.agent/polish.md`.
- Unit gauges: M3.1 main=84% 202K, mate=80% 192K - M3.2 main=93% 224K,
  mate=100% 239K - M3.3 main=102% 245K pre-compaction / 59% 143K close,
  mate=100% 240K - M3.4 main=94% 225K pre-compaction / 49% 117K close,
  mate=58% 139K - M3.5 main=92% 220K, mate=39% 95K - M3.6 main=87% 208K,
  mate=28% 67K - M3.7 main=93% 224K, mate=95% 227K.

Review (9 teammates: 7 unit lenses + cross-cutting + claim-replay audit;
26/28 audited claims replayed clean, the two misses being a stale brief claim
and the README bounds numerals fixed here):

- HIGH, fixed: aggregate payloads were self-certifying — an emptied payload
  beside a real product reported `aggregate ok 1 documents 0 obligations`
  rc0. Replay now checks the obligation set against the loaded composition
  before proving any head (coverage equality with the sentence identities the
  products carry, unique `(doc, sentence, variant)` keys, `1..K` variant
  runs, nonempty head lists, final-LF payload bytes). Battery
  `.scratch/m3rev/payload_battery.py` 9/9. Open residue: a dropped LAST
  variant stays undetected — rule-head `Deps` are variables, so per-sentence
  variant counts are not derivable under the frozen ABI (polish row).
- MEDIUM, fixed: committed reds pinned class/rc/framing but not Detail — a
  mutant rewriting both question details kept `goal.py check` green. Every
  probe now carries a required `.expect` sidecar compared byte-exact;
  orphan/missing pins are violations. 20 probes at review close (added: v1
  GROUP root detail, `of` noun modifier).
- MEDIUM, fixed: the ulex slot silently accepted the reserved basename
  `schema=v1`; `reserved_argv_token/1` now rejects it like `proof`.
- MEDIUM, fixed: the legacy witness covered only copula facts + one rule, so
  deleting the ground-fact branch killed nothing. Widened to every documented
  legacy family (copula, ground fact arity 1 + 2, positive rule, NAF rule
  with its dynamic declaration); `.scratch/m3rev/test_legacy_families.py`
  kills both branch mutants.
- MEDIUM -> docs: compiled documents are a definite-clause program, so an
  authored consequent entity feeding its own antecedent can diverge under
  naive SLD in one load order and succeed in another (verified: reverse order
  exhausts a 64 MiB stack, forward order proves). Our own replay is
  fail-closed — the same query under the compiler's bounds fails finitely —
  and no clause in the shipped corpus is left-recursive (scan 0/79, positive
  control 1). README carries the consumer note; `goal.py check` runs the scan
  over the shipped corpus through the compiler's `recursion-check` mode.
- Docs corrected: schema label candidate -> frozen; fork notice reduced to one
  §5(a)-relevant date plus git history; README gained the search-bound
  numerals, the aggregate-integrity guarantees, the `of` rewrite recipe and
  one-document-per-region wording; the guideline README's one-rule-per-impl
  claim replaced with the measured 21 single-sentence / 2-25 range.
- Deferred to `.agent/polish.md` with acceptance checks: last-variant residue,
  committed legacy-family oracle, object-operator closed-set kill, root-NAF
  precedence, vendor fork-notice gate.
- Review rerun (decisive chain, this commit): check ok 475 organizations /
  1,118 rows, terminal remaining orgs=0 rows=847 provisional=54, 1 guideline
  79 documents 20 red probes; regen check ok; gate_m3u1 24/10/0; gate_m3u2
  7/5/73; gate_m3u3 64/64; diff_m3u7 100 cases accept=79 reject=21
  divergences=0; question suite 96/96; fresh compile leaves `pl/` + `ace/`
  byte-identical. main=84% 200K/240K (review close; the wave spans 2
  windows), mate=86% 206K/240K (mrev-m3u7 peak).

Out of scope remains: serving/query API; probabilities, thresholds, arithmetic
or unit conversion absent from source; defeasibility/s(CASP) before a named
consumer question; rec6-12 authoring and pending-region/coverage closure,
which return to `/goal` rounds after review.

Sizing (analogs: M1.5 main=166K narrow kernel; M1.3a main=196K; M1.2b
main=203K oversized harvest; M1.3b main=179K): kernel units project
~150-175K MAIN with a hard checkpoint at contract+manifest before
implementation begins; oracle teammates may span planned successors
(each dispatch <=~170K). Production batches budget by source words
(~1,000/batch), not doc count; MAIN ~150-170K each. Off-spine items live
in `.agent/polish.md`, never as units. Out of scope: defeasibility/
s(CASP) (named consumer question first; backend swap = one backend +
recompile), serving layer, recs 6-12 authoring (post-M3 `/goal` work),
queue items 1-2 (pending-region rulings + coverage closure = `/goal`
rounds), new coverage rulings, probabilities/thresholds/conversions
absent from source.
