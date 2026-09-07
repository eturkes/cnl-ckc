---
paths:
  - "tools/**"
  - "vendor/e--/**"
  - "tests/strict/**"
  - "tests/adjudication/**"
  - "tests/copy/**"
  - "tests/dist/**"
---

# Legacy E-- toolchain law (retires at M5.7 cutover)

Until the M5.7 cutover: `regen.py --check` + `goal.py check` stay CI-authoritative and green at every commit; the E-- → generated-Python identity gate binds the legacy tree.

- Strict E-- authoring: the language rejects comments, indexing + postfix chains; bind intermediate call results; `[[x.y]](args)` call marker; retain a loop's final value explicitly. `Use <dotted>.` emits `import <dotted>`; tuple values via `[[tuple]](<a, b>)`; `Try:`/`Catch <name>:` = class-name handlers (dotted ok), no binding var, no blanket-except, one-or-more Catch, source-order first-match, `Exit with`/SystemExit bypasses `Catch Exception` while `Catch SystemExit`/`Catch BaseException` catch it. `python3 -P` omits the script dir from `sys.path` ⇒ one `.emm` per entry point. Atomic writes = exclusive temp + flush + fsync + `os.replace` + CAS.
- Strict-fixture gate (`check_strict_fixtures`): root+family closed inventory, class→rc map syntax|slot|non-ascii|control-char→1 usage|io→2, red byte-`.expect` + `strict:<class>:` prefix, green byte-`.golden`, inline canonical-detector subprocess check, meter `goal: strict fixtures ok <r> red <g> green`.
- Adjudication gates: `check_adjudication` re-derives `audit/review-manifest.tsv` in memory + byte-compares vs committed (stale/missing = violation + regen hint), validates `audit/adjudication.tsv` against the derived manifest; meter `goal: adjudication <id> approved=<a> rejected=<r> stale=<s> unreviewed=<u>`; absent ledger = green all-unreviewed. `check_adjudication_fixtures` = tests/adjudication differential. CLI writers: `review-manifest <id>`, `ledger-validate <ledger> <manifest> <label>`. `semantic_clause_sha256` hashes retained clause lines only (drops `%` comments, `:- ` directives + the document record; exactly-one-record check) → pure ulex append stales nothing.
- KB export: `tools/dist.emm`→`dist.py` + goal.emm `check_dist` (loads dist.py as the sys.argv[0] sibling via exec of the source prefix before the `\nargv = list(sys.argv)` marker) + `goal.py release-manifest` writer. ALL derivation reads HEAD (git show/ls-tree), never the working tree; input-head = last commit touching `guidelines/` + the vendored compiler + lexicon; archive name + bag root = `cnl-ckc-kb-g<head12>`. `release-manifest.tsv` stales on commits touching guidelines/, the vendored compiler/lexicon, `NOTICE`, or the docs/REFERENCE.md schema section → regen per REFERENCE Operating § Close. check_dist meter `goal: dist ok <g> guidelines <m> members <b> bytes` | `goal: dist blocked rejected=<r> contested=<c>` (blocked skips the live build only). Red suite = `tests/dist/red.sh` (d07 cases clone HEAD ⇒ run post-commit).
- goal.py = module-level dispatch, never import (batteries exec the source with argv pinned). Full `goal.py check` >10 min + block-buffered → background to a log.
