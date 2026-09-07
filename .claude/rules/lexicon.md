---
paths:
  - "vendor/clex/**"
  - "guidelines/*/lexicon.ulex"
  - "guidelines/*/audit/lexicon-shadow.tsv"
---

# Lexicon law

- Clex = `vendor/clex/clex_lexicon.pl` (97,527 facts, verbatim upstream pin 20960a5c; `Pristine: yes` PROVENANCE class → fork-notice gate verifies MANIFEST.sha256 digests + zero notices instead of git log). `stage_ape` overlays it onto the staged demo `prolog/lexicon/clex_lexicon.pl` → every parse (corpus + tests/red) reads full Clex; vendor/ape's 8-fact demo file = dead at runtime, kept to avoid churning its fork notice.
- Clex is authoritative for the words it provides: a ulex entry duplicating one byte-for-byte rejects (`lexicon-redundant`; normalization = strip quotes/whitespace/final dot). The same normalization pins ulex-internal duplicates (`lexicon-duplicate`). Ulex lookup MERGES with clex per category (lexicon_interface.pl falls through; no suppression) → a ulex entry sharing a surface with any non-byte-identical clex fact rejects (`lexicon-shadow`) unless `guidelines/<id>/audit/lexicon-shadow.tsv` rules that exact ulex-entry × sorted `+`-joined clex-reading-set pair (pinned header `ulex_entry	clex_entries	ruling`; stale/dup/malformed rows reject `lexicon-shadow(-file)`; re-pinning Clex upstream re-opens rulings by design). Rulings regenerate via `.scratch/clex-restore/emit_rulings.py`; redundancy prune replay = `.scratch/clex-restore/prune_ulex.py`; shadow/redundancy enumerator = `.scratch/polish-lexshadow/dryrun.py`, battery = `battery.py` beside it.
- Ulex `pn_sg/3` WordForm is case-sensitive to the ACE surface.
- Any `lexicon.ulex` byte change rewrites the ulex digest in every `guideline_document/3` record → recompile the whole corpus.
