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
  code, and any conforming engine can load the result. The file
  `guidelines/schema.pl` defines the numeric comparisons of the
  conditions. Load it with the documents.
- **Checked compilation.** For every sentence, the compiler makes
  sample facts that satisfy the conditions of the sentence. Before it
  emits anything, it checks that the emitted clauses derive the
  conclusion of the sentence from these facts, for the document alone
  and with the whole guideline loaded. This check finds clauses that do
  not carry their sentence. It does not show that a sentence states
  its passage correctly: the clinician review checks that. Committed
  questions ship with answers and proof traces that name the exact
  clause lines used.
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
- A condition with a number, such as "at least 50 MME/day", matches a
  stated value that meets it: 60 meets it, and "at most 70" does not. A
  plain number in a statement means at least that many. To state a
  measured value, use `exactly`. Seconds, minutes, hours, days and weeks
  convert, and months convert to years. Days and months do not convert.
- The knowledge base converts the daily dose of each opioid in the CDC
  conversion table to MME per day, with the exact factor of the table. To
  use it, state the dose as an opioid dosage that contains an amount, for
  example `exactly 40 oxycodone-milligrams-per-day`. The conversion does
  not apply to a dosage that serves the management of opioid use
  disorder. The knowledge base does not add the doses of several opioids:
  a total dosage must come from the patient data.
- A guideline can state where its recommendations do not apply, for
  example for patients with cancer-related pain. The compiler turns
  such a statement into a scope exclusion. When the condition of an
  exclusion holds for the patient, the knowledge base derives no
  "should", "must" or "may" statement of that guideline. If no
  exclusion holds, this does not prove that the guideline applies to
  the patient. One set of loaded facts must describe one patient. A
  condition with "does not provably" holds when the loaded facts and
  rules do not establish its content, so a missing record counts as
  absence. When the search of the `ckc` program stops at its limit, it
  establishes nothing, and the answer is `indeterminate(limit)`.
- The `ckc` program searches for answers in its own order. It checks
  an operator box, such as "should" or a negation, after the goal that
  names the content of the box. A standard Prolog engine searches the
  same clauses from left to right. For the documents in this
  repository, both orders give the same answers when both searches
  end.
- To query the knowledge base with the facts of a patient in a
  standard Prolog engine, declare the schema predicates as tabled
  before you load the documents. `docs/REFERENCE.md` states the
  declaration in "Loading a whole knowledge base". Without it, a
  broad question, such as every "should" statement for the patient,
  does not end. The tested question whether a dosage reaches 50 MME
  per day ends without it. The `ckc` program answers the stored
  questions and that tested question. A broad question about a patient
  can stop at its search limit with `indeterminate(limit)`.

## Install

The project supports Linux on x86-64.

1. Install Git, curl, tar, unzip, and a C linker (`cc`).
2. Install SWI-Prolog 10.0.2. The project pins that exact version.
   `docs/REFERENCE.md` states the build recipe.
3. Install `just`. Use your package manager, or the release that
   `rust/tools.lock` records.
4. In the repository root, run `just tools`. This command downloads
   the pinned Rust toolchain, the verifier, the audit tools, and a
   headless browser with Node.js for the phone-layout check into
   `.toolchain/`. It checks every download against its recorded digest.
   The headless browser needs the shared libraries of Google Chrome. On
   Debian or Ubuntu, the file
   `.toolchain/chrome-headless-shell/chrome-headless-shell-linux64/deb.deps`
   lists the packages.
5. Run `just build`. The program is `rust/target/release/ckc`.

To review a release candidate, you do not need these tools. Each
candidate and each release publishes the reviewer program
`ckc-x86_64-linux`, which needs Git and tar only:

1. On the release page of the candidate, download `ckc-x86_64-linux`.
2. Clone the branch of the candidate, for example
   `git clone --branch release/kb-YYYY-MM-DD https://github.com/eturkes/cnl-ckc`.
3. In the clone, run
   `git -c gpg.ssh.allowedSignersFile=docs/allowed_signers verify-tag kb-YYYY-MM-DD-rc.1`
   with the tag of the candidate. The output must report a good
   signature.
4. In the download directory, run
   `git -C <clone> tag -l --format='%(contents:body)' kb-YYYY-MM-DD-rc.1 | sha256sum -c --ignore-missing`.
   The line of `ckc-x86_64-linux` must report OK.
5. Run `chmod +x ckc-x86_64-linux`.
6. In the clone, run `<download directory>/ckc-x86_64-linux ui serve`.
   "Reviewing" tells how to review. The site writes your decisions to
   `guidelines/<guideline-id>/audit/adjudication.tsv` in the clone.

## Run

Run each command in the repository root. The examples call the program
`ckc`. Put `rust/target/release` on your `PATH`, or type the full path.

- `ckc check` validates the repository: sources, coverage, lexicon,
  review records, fixtures, proofs, and the release manifest. It
  prints one line per section.
- `ckc compile <guideline-id>` compiles the ACE documents again and
  writes `guidelines/schema.pl`.
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
  scan, phone layout, repository checks, and certification.

## Configure

- `SWIPL` names the SWI-Prolog program. By default, `ckc` uses
  `swipl` from your `PATH`.
- `CKC_TOOLCHAIN` names the directory of the pinned tools. The default
  is `.toolchain/` in the repository root.
- `CKC_BIN` names a prebuilt `ckc` program for `just mobile`,
  `just check`, and `just certify`. If you do not set it, these recipes
  build `ckc` first.
- The reviewer site listens on `127.0.0.1` only. To choose the port,
  give a port number to `ckc ui serve`.
- `BROWSER` names the program that opens the reviewer site. If you do
  not set it, `ckc ui serve` uses `xdg-open` on a desktop, or `wslview`
  in WSL.

## Reviewing

Start the site with `ckc ui serve` in a terminal window. The site opens
in your browser. Keep the window open while you review. To open the site
again, press Enter in that window. To stop the site, press Ctrl+C. If no
browser opens, open the address that the window shows.

The site runs only on the local computer, with no accounts. Only a
browser that opened the site from that window can use it. Each document
page asks one question:
does the ACE representation appropriately reflect the original
passage? Record approve or reject, with your name and an optional
comment. The site records the name as typed and does not verify it.
A word with a solid underline in the ACE text has a fixed meaning in
the vocabulary table of the guideline. Select the word to see how the
compiler reads it. The guideline page lists all of these meanings under
"Word meanings".

## Releases

A release of the knowledge base is a git tag with the name
`kb-YYYY-MM-DD`. Its release notes give the archive name, the SHA-256
of the archive, and the SHA-256 of the reviewer program. A tag that
ends in `-rc.1`, `-rc.2`, and so on is a release candidate. The review
of a release candidate is not complete. To use the knowledge base in
other software, pin a release tag.

Each tag carries the SSH signature of the maintainer. To check a tag,
run `git -c gpg.ssh.allowedSignersFile=docs/allowed_signers verify-tag kb-YYYY-MM-DD`
in a clone. The output must report a good signature. "Releasing" in
`docs/REFERENCE.md` states how to check the downloaded files against
the signed notes.

Each release has its own branch, `release/kb-YYYY-MM-DD`. To review a
release candidate, start the reviewer site in a checkout of that
branch. Development continues on `main`. `docs/REFERENCE.md` states the
release procedure in "Releasing".

## Technical reference

`docs/REFERENCE.md` states the pipeline, the checks, the compiled
schema, the operating procedure, and the export format. Each guideline folder
records its source, retrieval date, and rights in its own `README.md`.
`LICENSE` and `NOTICE` state the license terms.
