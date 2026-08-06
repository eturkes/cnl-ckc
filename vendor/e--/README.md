# E-- (English--) — cnl-ckc fork

Modified from upstream e-- in the cnl-ckc fork; see PROVENANCE.

Pruned in-place fork of the E-- English-to-Python compiler, reduced to the
strict deterministic pipeline. The upstream README, packaging, examples, and
legacy transpiler pipeline are available in git history at the fork base
recorded in `PROVENANCE`.

## Layout

- `src/e_minus_minus/` — the complete package; module list and TCB roles in
  `PROVENANCE`.
- `docs/spec.md` — the E-- language specification; the strict entry point
  adds the fork's validation profile on top.

## Use

Compile one canonical `.emm` source to Python on stdout:

    PYTHONPATH=vendor/e--/src PYTHONDONTWRITEBYTECODE=1 \
      python3 -P -m e_minus_minus.strict <file.emm>

`tools/regen.py` drives this for every tracked `.emm` and enforces
source/generated identity. License: Apache-2.0 (`LICENSE`).
