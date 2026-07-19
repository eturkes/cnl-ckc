# User lexicon contract

The ACE adapter accepts either an APE tree alone or an APE tree plus one user
lexicon (Ulex) file:

```text
swipl -q -f none -F none -s src/prolog/adapter.pl -g main -t 'halt(9)' -- <ape-tree-dir>
swipl -q -f none -F none -s src/prolog/adapter.pl -g main -t 'halt(9)' -- <ape-tree-dir> <ulex-file>
```

## Load and lookup semantics

The optional file is loaded after APE and before ACE input is read or parsed. The
adapter pre-reads the whole Ulex file as binary and applies strict UTF-8 validation
before handing its in-memory text to APE; `ulex:read_ulex/1` must consume through
physical EOF. It calls `ulex:discard_ulex/0` and `ulex:read_ulex/1` with
cleanup-guaranteed stream closure. With no Ulex argument, it does not call the
Ulex module.

Run a fresh SWI-Prolog process for each document. Thus Ulex state is per document
and cannot carry into another parse. Within a run, Ulex clauses have lookup
priority over Clex clauses because `lexicon_interface.pl` tries `ulex:` before
`clex:`.

## Entry forms

APE accepts exactly the 27 templates declared by `lexicon_template/1` in
`vendor/ape/prolog/lexicon/ulex.pl`:

```prolog
adv(WordForm, Lemma).
adv_comp(WordForm, Lemma).
adv_sup(WordForm, Lemma).

adj_itr(WordForm, Lemma).
adj_itr_comp(WordForm, Lemma).
adj_itr_sup(WordForm, Lemma).
adj_tr(WordForm, Lemma, Preposition).
adj_tr_comp(WordForm, Lemma, Preposition).
adj_tr_sup(WordForm, Lemma, Preposition).

noun_sg(WordForm, Lemma, Gender).
noun_pl(WordForm, Lemma, Gender).
noun_mass(WordForm, Lemma, Gender).
mn_sg(WordForm, Lemma).
mn_pl(WordForm, Lemma).

pn_sg(WordForm, Lemma, Gender).
pn_pl(WordForm, Lemma, Gender).
pndef_sg(WordForm, Lemma, Gender).
pndef_pl(WordForm, Lemma, Gender).

iv_finsg(WordForm, Lemma).
iv_infpl(WordForm, Lemma).
tv_finsg(WordForm, Lemma).
tv_infpl(WordForm, Lemma).
tv_pp(WordForm, Lemma).
dv_finsg(WordForm, Lemma, Preposition).
dv_infpl(WordForm, Lemma, Preposition).
dv_pp(WordForm, Lemma, Preposition).

prep(WordForm, Lemma).
```

`WordForm` is the ACE surface atom. The predicate name identifies its inflection:
`sg`/`pl` are singular/plural, `finsg` is finite third-person singular,
`infpl` is infinitive or finite plural, `pp` is past participle, and `comp`/`sup`
are comparative/superlative. `pndef` is a proper name used with *the*.

`Lemma` is the logical symbol emitted into the DRS; inflections and approved
aliases may share it. `Gender` controls anaphoric compatibility and is one of
`undef`, `neutr`, `human`, `masc`, or `fem`. For `adj_tr`, `Preposition` is the
adjective's governed preposition. For `dv_*`, it is the indirect-object marker;
the empty atom `''` denotes the double-object frame. See the
[ACE Lexicon Specification](https://attempto.ifi.uzh.ch/site/docs/ace_lexicon.html)
for lexical syntax and category details.

## Fail-closed behavior

| Condition | Adapter result |
|---|---|
| Missing or unreadable Ulex file | Exit 2, `ulex_load`; zero stdout. |
| Missing `ulex` module in the parser tree or plain `ulex:read_ulex/1` failure | Exit 2, `ulex_load`; zero stdout. |
| Malformed Prolog file or malformed entry | APE records a `lexicon` error; exit 1, `ape_messages`; zero stdout. |
| Function-word redefinition, duplicate entry, or forbidden category intersection | APE records a `lexicon` warning (including “defined twice” or “Bad intersection”); exit 1, `ape_messages`; zero stdout. |

APE's parse-time `clear_ape_messages/0` clears only `character`, `word`,
`sentence`, `anaphor`, and `pronoun` messages, so Ulex `lexicon` messages survive
to the adapter. The adapter rejects on any message, warning or error. APE also
forces `drs([], [])` for error messages, but the any-message rule is sufficient
to reject.

## Deterministic production

Ulex producers must emit UTF-8 with LF line endings, one canonical fact per line
ending in `.`, sorted and unique by `(kind, WordForm)`. This is necessary but
not sufficient: producers must also avoid every `ulex:check_intersections/1`
conflict, including degree-form duplicates and forbidden cross-category reuse.
Those conflicts produce `lexicon` warnings and are rejected. Identical Ulex bytes
and ACE input must produce identical canonical DRS bytes in fresh processes.

For M5, production Ulex files will be generated from the versioned terminology
registry, with a stable lemma-to-registry-ID mapping and the generated Ulex hash
recorded in provenance, following `docs/research/cnl-ace.md` §5. Hand-written
Ulex files are test fixtures only.
