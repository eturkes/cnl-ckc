# standing instructions

User scope directive; every session holds the project to these rules.

- Scope stays minimal: the project is minimally sufficient for fetching
  clinical guidelines, normalizing them into ACE, and compiling them into
  Prolog, all driven by the single built-in `/goal` command in Claude Code.
- In-repo consumers stay reviewer-facing: a local UI adjudicates each ACE
  document against its source, and preloaded traced queries demonstrate
  the compiled base as committed artifacts — no runtime LLM, no remote
  serving. The knowledge base ships portable through a deterministic
  distribution build that excludes the UI and tooling.
- Extend/fork APE, Clex, and others where necessary; build upon the
  existing work of other people as much as possible.
- Keep the compiler/lexicon the smallest that supports the currently
  analyzed guidelines; adapt both as new guidelines demand.
- Language law, Prolog: all knowledge Prolog is emitted through the
  vendored APE fork from ACE; hand-authored Prolog stays confined to that
  fork's emission closure (parse → DRS → v1 document + question emission).
  Every ACE document faces clinician review through the UI, so committed
  Prolog encodes only what that review chain touches: guideline knowledge
  and its custody records. Committed answers and traces are machine-derived
  demonstrations over ACE-compiled products and carry no hand-authored
  knowledge.
- Language law, Rust: all other first-party code is Rust. No human reads
  the Rust: a small human-read formal specification plus a machine-checked
  proof gate (pinned verifier, escape-hatch audit) certify the verified
  kernel with zero trust in its author; a thin enumerated shell stays
  outside the proofs and inside the fixture gates. E-- and its generated
  Python are retired; git history is their record. Migration = roadmap M5;
  until its cutover unit lands, the legacy E-- → Python identity gate
  remains binding for the legacy tree.
- Dependencies stay minimal and enumerated: a trusted dependency must be
  easily reasoned about, mature, and dangerous to implement incorrectly —
  cryptographic digests are the example meeting all three. The verifier
  toolchain is pinned and named in the trusted surface.
- As much as possible, the codebase contains only Prolog and Rust (plus
  ACE, the formal specification, and fixture data).
- Keep the codebase, tests included, the smallest possible: a human audits
  the project through the ACE, the formal specification, and the named
  Prolog closure; the proof checker audits everything else.
- Prolog and Rust code itself stays agent-oriented — comments, Git
  messages, etc.
