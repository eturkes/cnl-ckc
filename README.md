# cnl-ckc

cnl-ckc (Controlled Natural Language - Clinical Knowledge Compiler)
turns published clinical guidelines into a computable knowledge base
with a complete audit trail. Every guideline statement
is written in Attempto Controlled English (ACE): sentences that read
as plain English and also parse as formal logic. A compiler turns them
into plain Prolog that any standard engine can load and query.
Clinicians review each statement beside the exact guideline passage
that it came from, and every decision stays on record.

## Design

- **One readable source of truth.** Each guideline passage becomes a
  short ACE document. The sentences that a clinician reviews are the
  same source that the compiler consumes — there is no second,
  hidden encoding to trust.
- **Plain Prolog as the product.** The compiled knowledge base is
  ordinary Prolog clauses over a small, fixed vocabulary. Modality
  ("should", "must") and negation are recorded as data, not buried in
  code, and any conforming engine can load the result.
- **Proof-checked compilation.** The compiler derives a proof
  obligation for every sentence and discharges it before it emits
  anything: a shipped clause is one that the compiler has proved
  follows from the sentence it quotes. Committed questions ship with
  answers and proof traces that name the exact clause lines used.
- **Review against exact versions.** The reviewer site shows each ACE
  document beside its original passage and its compiled Prolog. A
  decision binds to the precise version judged; a later change marks
  it outdated, and no decision is ever deleted.
- **A small trusted base.** Everything a human must read is controlled
  English, a direct compilation of it, or part of two small named
  compiler forks. Even the project's own tools are compiled from an
  English-like language.

## What the repository holds

The original guideline files, stored unchanged with their retrieval
records; the ACE documents, one per guideline passage; the compiled
Prolog; and every review decision. The first guideline in the
collection is the CDC Clinical Practice Guideline for Prescribing
Opioids for Pain (2022).

## Scope and limits

- The knowledge base reports what the loaded guidelines state. It is
  not clinical advice, and it does not interpret a guideline for a
  patient.
- Each ACE document is a deliberately simplified projection of its
  passage, not a complete copy. The review pages put the original text
  beside it so that the simplification is always visible.
- The stored questions and answers are prepared demonstrations, not a
  clinical search service.

## Reviewing

Start the site with `python3 -P tools/ui.py serve` and open the
address that it prints. The site runs only on the local computer, with
no accounts and no JavaScript. Each document page asks one question:
does the ACE representation appropriately reflect the original
passage? Record approve or reject, with your name and an optional
comment. The site records the name as typed and does not verify it.

## Technical reference

`docs/REFERENCE.md` states the pipeline, the checks, the compiled
schema, the operating procedure, and the export format. Each guideline folder
records its source, retrieval date, and rights in its own `README.md`.
`LICENSE` and `NOTICE` state the license terms.
