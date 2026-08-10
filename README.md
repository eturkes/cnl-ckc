# cnl-ckc

A minimal, fully auditable pipeline that turns clinical guidelines into
executable Prolog through controlled natural language:

1. **Fetch** — a guideline source document is acquired and recorded under
   `guidelines/<id>/source/` with digests, rights, and attribution in that
   guideline's `README.md`.
2. **Normalize** — recommendations are authored as Attempto Controlled
   English (ACE), one document per recommendation, under
   `guidelines/<id>/ace/`, with domain vocabulary in
   `guidelines/<id>/lexicon.ulex`.
3. **Compile** — `tools/goal.py` stages the vendored APE parser and compiles
   every ACE document to plain Prolog under `guidelines/<id>/pl/` via
   `vendor/ape/prolog/ace_to_pl.pl`.

Claude Code's built-in `/goal` command drives all three steps ("Operating"
below).

## Audit story

Every artifact a human needs to read is controlled natural language, a direct
compilation of it, or part of one small named compiler base:

- **ACE → Prolog.** All guideline Prolog (`guidelines/*/pl/`) is compiled
  from ACE. The compiled files quote each source sentence beside its clause,
  and each carries the SHA-256 of the exact ACE and lexicon bytes it was
  compiled from.
- **E-- → Python.** All first-party Python (`tools/goal.py`,
  `tools/regen.py`) is compiled from E-- (`tools/*.emm`), an English-like
  language. `tools/regen.py --check` proves every committed `.py` is
  byte-identical to a fresh compile of its `.emm` and flags any tracked
  Python outside `vendor/e--/src/` that has no `.emm` source.
- **The compiler base is closed and named.** Two vendored forks perform
  those compilations and are the trusted computing base a human must read
  directly: `vendor/ape/` — the ACE parser plus the hand-authored
  `prolog/ace_to_pl.pl` compiler, the one first-party Prolog artifact not
  itself compiled from ACE — and `vendor/e--/` — the hand-authored Python
  that compiles E--. Both trees are pruned to their load closures;
  `vendor/*/PROVENANCE` records upstream, fork base, inventory, and trust
  boundary. Git history is the change record.
- **Tests are data.** `tests/red/` holds rejection probes named
  `<expected-error-class>--<name>.ace`. One `tools/goal.py check` invocation
  is the full acceptance gate: it validates layout, source records, and
  Prolog/lexicon inventory closure, recompiles every guideline twice
  (byte-determinism), compares against the committed Prolog (freshness),
  proves every compiled query, and asserts each red probe is rejected with
  its named error class and exit status.

## Running

Requires SWI-Prolog 9.2.9 and Python ≥ 3.11 (the supported invocations use
`python3 -P`; CI pins the SWI 9.2.9 container, whose Debian Python is 3.11).

```sh
python3 -P tools/goal.py compile <guideline-id>   # ACE → Prolog
python3 -P tools/goal.py check                    # full acceptance gate
python3 -P tools/regen.py --check                 # E-- → Python identity
```

## Operating

Guideline work runs as goal rounds in a Claude Code session opened at the
repository root, driven by the built-in `/goal` stop-condition command:

```
/goal Process American clinical guidelines through the pipeline as described in README.md "Operating", fanning bulk work out to teammates per .agent/rounds.md: work the in-progress source document to full coverage before fetching the next; done only when every fetched guideline is complete, every remaining .agent/queue.md entry is a recorded blocker, and the compendium exhaustion clause in .agent/compendium.md "Protocol" holds: every guideline row of .agent/compendium.tsv done, blocked, or excluded, and every organization row terminal.
```

The goal re-arms each time Claude tries to stop and survives session resume,
so halting at any moment is safe: the repository is the only persistence,
and every round starts by deriving state from it — `.agent/queue.md`,
`git status`, `tools/goal.py check`, and the in-progress guideline README's
coverage statement — then finishes or discards incomplete work before
taking on anything new. Bulk work (source reading, extraction drafting, ACE
drafting, adversarial review) fans out to subagent teammates per
`.agent/rounds.md`; the session lead alone writes the repository and
commits.

Exactly one source document is in progress at a time and is worked to
completion — across as many rounds as it takes — before the next one is
fetched. A source document is complete when every normative statement in it
has been extracted verbatim into `source/` evidence and either authored as
ACE and compiled, or recorded in the guideline README as uncovered with a
reason, and `python3 -P tools/goal.py check` is green.

While a document is in progress, a round advances it one increment:

1. **Extract** — the next batch of normative statements, verbatim, into
   `guidelines/<id>/source/` evidence files.
2. **Normalize** — one ACE document per statement under `ace/` plus the
   shared `lexicon.ulex` (supported constructs: the header of
   `vendor/ape/prolog/ace_to_pl.pl`). Each ACE document is a minimal
   faithful projection of its statement; the guideline README states honest
   coverage.
3. **Compile** — `python3 -P tools/goal.py compile <id>`. A rejection means
   the ACE or lexicon is adjusted first; only a genuinely new construct
   extends the `ace_to_pl.pl` translation, minimally, keeping totality
   (every sentence compiles to exactly one clause; unrecognized shapes
   reject) and adding `tests/red/` probes for the new rejection boundary.
4. **Close** — `python3 -P tools/goal.py check` green, plus
   `python3 -P tools/regen.py --check` when E-- sources changed; a scoped
   commit; `.agent/queue.md` and the guideline README's coverage statement
   updated.

When no document is in progress, the round fetches the next source — the
queue's next entry, the queue refilling from `.agent/compendium.tsv` per
`.agent/compendium.md` § Queue promotion when it runs dry: acquire the
document and
persist it immutably under `guidelines/<id>/source/` before any generative
processing; record URL, retrieval date, SHA-256, byte length, and a rights
quote in `guidelines/<id>/README.md` (model:
`guidelines/cdc-2022-opioid/`); commit the source record on its own, so it
survives any halt. Ids follow `[a-z0-9-]+`, chosen once per source; an
already-fetched URL keeps its recorded id, and changed remote content
becomes a new versioned id while every recorded source stays immutable. A
paywall or rights gate is recorded as a queue blocker and the round moves
on.

## Licensing

First-party work outside `vendor/`: Apache-2.0 WITH LLVM-exception
(`LICENSE`). First-party additions inside a vendored tree adopt that tree's
license — `vendor/ape/prolog/ace_to_pl.pl` is LGPL-3.0-or-later. Vendored
trees keep their own licenses: `vendor/e--` Apache-2.0, `vendor/ape`
LGPL-3.0-or-later. See `NOTICE`.
