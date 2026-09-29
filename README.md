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
- **Certified compilation.** `ckc certify` parses every committed
  document again with the upstream ACE parser. It then checks that the
  committed clauses are exactly what the formal specification states
  for that parse. A human therefore does not have to trust the
  compiler program itself.
- **Review against exact versions.** The reviewer site shows each ACE
  document beside its original passage and its compiled Prolog. A
  decision binds to the precise version judged; a later change marks
  it outdated, and no decision is ever deleted.
- **A small trusted base.** Everything a human must read is controlled
  English, a direct compilation of it, or the short formal
  specification in `rust/ckc-spec/`. The program that checks, answers,
  traces, and renders the knowledge base is machine-verified against
  the committed specification under a pinned verifier. The verifier
  and its toolchain are part of the trusted base.

## What the repository holds

The original guideline files, stored unchanged with their retrieval
records; the ACE documents, one per guideline passage; the compiled
Prolog; and every review decision. The first guideline in the
collection is the CDC Clinical Practice Guideline for Prescribing
Opioids for Pain (2022). The `rust/` directory holds the `ckc`
program, its formal specification, and its proofs.

## Scope and limits

- The knowledge base reports what the loaded guidelines state. It is
  not clinical advice, and it does not interpret a guideline for a
  patient.
- Each ACE document is a deliberately simplified projection of its
  passage, not a complete copy. The review pages put the original text
  beside it so that the simplification is always visible.
- The stored questions and answers are prepared demonstrations, not a
  clinical search service.

## Install

The project supports Linux on x86-64.

1. Install Git, curl, tar, unzip, and a C linker (`cc`).
2. Install SWI-Prolog 9.2.9. The project pins that exact version.
   `docs/REFERENCE.md` states the build recipe.
3. Install `just`. Use your package manager, or the release that
   `rust/tools.lock` records.
4. In the repository root, run `just tools`. This command downloads
   the pinned Rust toolchain, the verifier, and the audit tools into
   `.toolchain/`. It checks every download against its recorded digest.
5. Run `just build`. The program is `rust/target/release/ckc`.

## Run

Run each command in the repository root. The examples call the program
`ckc`. Put `rust/target/release` on your `PATH`, or type the full path.

- `ckc check` validates the repository: sources, coverage, lexicon,
  review records, fixtures, proofs, and the release manifest. It
  prints one line per section.
- `ckc compile <guideline-id>` compiles the ACE documents again.
  `ckc queries <guideline-id>` derives the answers and the proof
  traces again. The results must equal the committed files.
- `ckc certify <guideline-id>` certifies every committed document and
  question against the formal specification.
- `ckc ui serve` starts the reviewer site on the local computer.
  `ckc ui render <directory>` writes the same pages to a directory.
- `ckc release-manifest` refreshes `release-manifest.tsv`, and
  `ckc dist build` writes the release archive to `dist/`.
- `just gate` runs every check that CI runs on each push. CI also runs `just deny`, `just kani`, and `just outdated` once a week. The checks are format,
  lint, verification, tests, dependency audit, secret scan, workflow
  scan, repository checks, and certification.

## Configure

- `SWIPL` names the SWI-Prolog program. By default, `ckc` uses
  `swipl` from your `PATH`.
- `CKC_TOOLCHAIN` names the directory of the pinned tools. The default
  is `.toolchain/` in the repository root.
- `CKC_BIN` names a prebuilt `ckc` program for `just check` and
  `just certify`. If you do not set it, these recipes build `ckc`
  first.
- The reviewer site listens on `127.0.0.1` only. To choose the port,
  give a port number to `ckc ui serve`.

## Reviewing

Start the site with `ckc ui serve` and open the
address that it prints. The site runs only on the local computer, with
no accounts. Each document page asks one question:
does the ACE representation appropriately reflect the original
passage? Record approve or reject, with your name and an optional
comment. The site records the name as typed and does not verify it.

## Technical reference

`docs/REFERENCE.md` states the pipeline, the checks, the compiled
schema, the operating procedure, and the export format. Each guideline folder
records its source, retrieval date, and rights in its own `README.md`.
`LICENSE` and `NOTICE` state the license terms.
