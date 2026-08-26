# standing instructions

User scope directive; every session holds the project to these rules.

- Scope stays minimal: the project is minimally sufficient for fetching
  clinical guidelines, normalizing them into ACE, and compiling them into
  Prolog, all driven by the single built-in `/goal` command in Claude Code.
- In-repo consumers stay reviewer-facing: a local UI adjudicates each ACE
  document against its source, and preloaded traced queries demonstrate
  the compiled base as committed artifacts — no runtime LLM, no remote
  serving. The knowledge base ships portable through a deterministic
  distribution build that excludes the UI and tooling. Committed answers
  and traces are derived deterministically by the named compiler closure
  from ACE-compiled products and carry no hand-authored knowledge;
  hand-authored Prolog stays confined to that named closure.
- Extend/fork APE, Clex, and others where necessary; build upon the
  existing work of other people as much as possible.
- Keep the compiler/lexicon the smallest that supports the currently
  analyzed guidelines; adapt both as new guidelines demand.
- All Prolog must be compiled from ACE. All Python must be compiled from
  canonical E--. Extend/fork E-- liberally; the upstream is an abandoned
  proof-of-concept by a solo developer.
- Avoid libraries in Python so that all aspects of the program stay
  transparent. Except only libraries that are easily reasoned about,
  mature, and dangerous to implement incorrectly — file-system writes are
  the example meeting all three.
- As much as possible, the codebase contains only Prolog and Python (and
  their ACE/E-- counterparts).
- Keep the codebase, tests included, the smallest possible, so that a
  human can audit everything through reading the ACE and E--.
- Prolog and Python code itself stays agent-oriented — comments, Git
  messages, etc.
