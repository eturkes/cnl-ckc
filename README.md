# cnl-ckc

This project turns published clinical guidelines into a knowledge base
that software can search. It rewrites each guideline passage as short
sentences in Attempto Controlled English (ACE). ACE is a form of
English that a computer can also read. A compiler turns those
sentences into a formal record of what the guideline states.
Clinicians review each rewritten document against the exact passage
that it came from.

## What the repository holds

- The original guideline files, stored unchanged, with a record of
  where each file came from and when.
- The rewritten documents. Each document covers one passage of the
  guideline.
- The compiled knowledge base that software loads and queries.
- The decision records: every review decision, kept with the exact
  version of the document that it judged.

The first guideline in the collection is the CDC Clinical Practice
Guideline for Prescribing Opioids for Pain (2022).

## Scope and limits

- The knowledge base reports what the loaded guidelines state. It is
  not clinical advice, and it does not interpret a guideline for a
  patient.
- Each rewritten document is a simplified version of its passage, not
  a complete copy. The review pages show the original text beside the
  rewritten text, so that you can judge the difference.
- The stored questions and answers are prepared demonstrations, not a
  clinical search service.

## How review works

The review site runs only on the local computer. It is not on the
internet, and it has no accounts.

1. Start the site with `python3 -P tools/ui.py serve`. If you do not
   run commands yourself, ask a technical colleague to start it.
2. Open the address that the command prints.
3. Choose a guideline, then choose a document.
4. Read the rewritten sentences beside the original passage.
5. Answer one question: does the ACE representation appropriately
   reflect the original passage? Record your decision as approved or
   rejected, with your name and an optional comment.

Every decision stays on record. When a document changes later, the
site marks the old decision as outdated; no decision is deleted.
The site records the name that you type. It does not verify names.

## Technical reference

`TECHNICAL.md` states the pipeline, the checks, the compiled schema,
the operating procedure, and the export format. Each guideline folder
records its source, retrieval date, and rights in its own `README.md`.
`LICENSE` and `NOTICE` state the license terms.
