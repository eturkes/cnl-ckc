# Projection redesign — clinician-verifiable ACE (M3)

REVIEWED milestone record; stub = `.agent/roadmap.md`. Live law = README
schema section + the governing authoring rulings below.

Goal: replace fixture-bearing legacy projections with source-anchored,
knowledge-only ACE; freeze an engine-portable public Prolog ABI; derive and
replay proof obligations per document + aggregate; migrate the complete
`cdc-2022-opioid` corpus without changing coverage/census/docid custody.

Terminal state (at review close 18ab5a1; live law = README schema section +
the rulings below): 79 ACE = 79 compiled documents on frozen nine-indicator
schema v1, 257 obligations discharged per document + aggregate under bounded
search, lexicon 487 live entries, custody ledgers total + set-equal to the
ACE inventory, queue gate lifted — corpus growth since = `/goal` rounds
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
  aggregate replay, 19 committed reds each pinning class + rc + byte-exact
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

Review DONE (9 lenses; 26/28 audited claims replayed clean): fixed HIGH
self-certifying aggregate payloads (composition-derived coverage checks;
battery `.scratch/m3rev/payload_battery.py`) + MED byte-exact `.expect`
pins on every red probe, reserved `schema=v1` ulex basename + widened
legacy witness (both retired with the pre-v1 path deletion); SLD-divergence
risk documented in README + left-recursion scan in
`goal.py check`; residues + deferred hardening = `.agent/polish.md` rows;
decisive chain rerun green at close — narratives, rulings + rerun numbers =
`git log --grep "(M3 review" -p -- .agent/roadmap.md` (close = 18ab5a1);
gauges main=84% 200K close (wave spans 2 windows), mate=86% 206K peak.

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
