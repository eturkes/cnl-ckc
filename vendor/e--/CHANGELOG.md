# Changelog

All notable changes to E-- are documented here.

Semver: MAJOR.MINOR.PATCH. Pre-1.0 releases may change any behavior between MINOR versions; we aim to keep expression-slot semantics stable across the 0.x line.

## [0.2.0] — 2026-07-09

### Added
- **Code slots** (statement-position `{{ ... }}`). A slot placed as its own line at a block's indentation level resolves to one or more Python statements. Author writes structure; delegates whole regions to the LLM per line.
- Resolver contract extended: `resolve_slot(text: str, position: str = "expression") -> str`. Existing single-argument resolvers continue to work — the emitter tries the new kwarg and falls back to the v0.1.0 signature on `TypeError`.
- Anthropic-backed resolver: dedicated system prompt for statement mode; `max_tokens=1024` for statement slots (vs. 256 for expression); statement output validated with `ast.parse(mode="exec")` (vs. `mode="eval"`).
- Cache keys now include a position prefix (`expression:<text>` / `statement:<text>`). v0.1.0 bare-text entries still HIT for expression-position slots (back-compat). A cache miss always writes the new prefixed key.
- `examples/code_slot_example.emm` — a small program using a code slot.

### Grammar
- Parser: `{{ ... }}` may now appear at statement position (own line, block indentation) in addition to expression position. Position discriminator lives on the reused `LlmSlot` AST node (`position: "expression" | "statement"`).
- Emitter: statement slots splice at the surrounding block's indentation. Blank lines within the resolved output are preserved; each non-empty resolved line is indented uniformly.

### Design decision
- Wikilinks or callable references inside a code-slot's resolved Python are OPAQUE. E-- transpiler emits Python; it doesn't build a dependency graph. Author knowingly accepts DAG invisibility inside code-slot regions in exchange for region-level delegation.

### Docs
- `docs/spec.md` — grammar section extended; new "Slot positions" subsection.
- `README.md` — "Code slots" example added under `{{ }}` slot documentation.

## [0.1.0] — 2026-07-09

### Added
- Initial PyPI release. `pip install e-minus-minus` installs the deterministic transpiler with zero base dependencies.
- Console script `emm-transpile`.
- Programmatic API `from e_minus_minus import transpile`.
- Optional `[llm]` extra installs `anthropic` for `{{ }}` value-slot resolution.
- Deterministic transpile pipeline: tokenize → parse → emit. LLM invocation happens only for slots on a cache miss.

[0.2.0]: https://github.com/frmoded/e--/releases/tag/v0.2.0
[0.1.0]: https://github.com/frmoded/e--/releases/tag/v0.1.0
