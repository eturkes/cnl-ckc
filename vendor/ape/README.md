# APE (Attempto Parsing Engine) — cnl-ckc fork

Modified 2026-08-18 from upstream APE in the cnl-ckc fork; see `PROVENANCE`.

This tree is a pruned in-place fork of the Attempto Parsing Engine. It is
reduced to the load and build closure of `prolog/ace_to_pl.pl`, the ACE →
plain-Prolog guideline compiler whose contract lives in its file header. The
upstream README and all removed components are available in git history at
the fork base recorded in `PROVENANCE`.

## Layout

- `prolog/ace_to_pl.pl` — first-party compiler entry point (stdin ACE →
  stdout Prolog; `check` mode loads one compiled document).
- `prolog/parser/` — ACE parser (`ace_to_drs.pl` and dependencies) plus the
  ProFIT grammar sources (`*.fit`) and their translator (`fit_to_plp.pl`,
  `prologfeatures.pl`), which build the untracked `*.plp` grammar files.
- `prolog/lexicon/` — lexicon machinery. `clex_lexicon.pl` is trimmed to the
  minimal entry set used by `tests/red/`; guideline vocabulary is supplied
  per guideline via `lexicon.ulex` files.
- `prolog/logger/`, `prolog/utils/` — the error logger and the three DRS
  utility modules the parser closure loads.

## Use

Always run through `tools/goal.py`: it stages a scratch copy
(`shutil.copytree`), builds the grammar (`swipl -g "[fit_to_plp], halt."`
inside `prolog/parser/`), and invokes the compiler there. License:
LGPL-3.0-or-later (`LICENSE.txt`).
