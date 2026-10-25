# SUP1 mutation credit — already-passing shell-facing tests

Law: `CLAUDE.md` Engineering (verification integrity) + `.claude/rules/rust.md` Mutation credit. Run on 86b7c845 (wt/sup1; the shell files + both suites are byte-identical in main 9c1647ab), pool host, `just mutants` (cargo-mutants, default operators, `--no-config`, in place in a disposable worktree of HEAD).

| run | tests | scope | command | result |
|---|---|---|---|---|
| A | `sup1_surface` `table::p1_inherited_error_order_characterization`, `table::p1_ui_file_preflight_characterization` | ckc/src/main.rs, ckc/src/ui/{mod,pages,intake,common}.rs | `just mutants '--release --test sup1_surface -- table::p1_inherited_error_order_characterization table::p1_ui_file_preflight_characterization' ckc/src/main.rs ckc/src/ui/mod.rs ckc/src/ui/pages.rs ckc/src/ui/intake.rs ckc/src/ui/common.rs` | 280 mutants, 81 caught, 177 missed, 22 unviable, 0 timeout (82 min) |
| B | `sup1_acceptance` `composition::p4_independent_control`, `composition::p4_caller_model_characterization`, `certification::p5_model_baselines` | ckc/src/main.rs, ckc/src/v1_cli.rs, ckc/src/certify_cli.rs | `just mutants '--release --test sup1_acceptance -- composition::p4_independent_control composition::p4_caller_model_characterization certification::p5_model_baselines' ckc/src/main.rs ckc/src/v1_cli.rs ckc/src/certify_cli.rs` | 136 mutants, 57 caught, 74 missed, 5 unviable, 0 timeout (2 h) |

Lists = `{a,b}-{caught,missed,unviable}.txt`. Kills cover the pinned behavior: A kills every mutant of the vocabulary preflight (`common.rs` `checked_text` + `load_text`) and of the vocabulary refusal path (`intake.rs` `vocabulary` except its retired-name guard); B kills the `v1 answer` + `certify-one` success paths the tests call.

Survivor rulings (MAIN; the tests' authors = tester-1, tester-2): every survivor lies outside the five tests' calls or assertions, so the SUP1 contract leaves it to the named suites — recorded, no test added.

| run | file: functions (survivors) | ruling |
|---|---|---|
| A | main.rs `main` (26) | arms of other commands + usage text; the tests call `ckc ui check` alone; pinned by cli.rs, v1_suite, check_mutants, dist_cases |
| A | ui/mod.rs `run`, `finish_code`, `usage` (15) | the render, copy-check, request + serve arms and the argument-count guards past two arguments; pinned by ui_fixtures (render, request, copy-check cases) + ui_socket (serve) |
| A | ui/intake.rs `load`, `inventory`, `manifest`, `ledger`, `document`, `evidence`, `payloads`, `census`, `assets`, `valid_asset`, `media_type`, `vocabulary` retired-name guard (49) | the other intake laws (directory shape, ledger + manifest custody, document digests, payloads, assets) + the retired `temporal.tsv` refusal; the fixture trees pass them; pinned by the 85 `tests/ui/red` cases (`temporal-invalid` included) |
| A | ui/pages.rs `render`, `files`, `Tree::write`, `destination_blocked`, `strip_between`, `visible`, `word`, `has_functor`, `visible_violation`, `resource_tag`, `resource_reference`, `invariant`, `check` (74) | page rendering + page invariants on the success path, which these tests assert only as rc 0; pinned by ui_fixtures (golden byte compare + invariant cases), n22_viewport, n23_separators, ur_ui, rd_disclosure |
| A | ui/common.rs `digest`, `valid_id`, `valid_hex`, `Scratch::drop` (13) | helpers outside the vocabulary preflight; pinned by ui_commit + ui_fixtures |
| B | main.rs `main` (20) | arms of other commands; pinned as in A |
| B | v1_cli.rs `run`, `run_mode`, `run_file`, `reject`, `line_col` (25) | modes other than `answer` (aggregate-check, recursion-check, trace, trace-check, check, render) + reject-position formatting; pinned by v1_suite + cli.rs (the DA1 credit ruled the same guards) |
| B | certify_cli.rs `run`, `run_one` argument guards, `ace_paths`, `optional_ulex` + `optional_table` NotFound guards, `name_ok`, `order_artifacts`, `certify_parallel`, `emit`, `reject`, `Stage::drop` (29) | `ckc certify <id>` (guideline mode) + certify-one's error paths + absent-file guards, which the baselines never reach (valid inputs, no table); pinned by certify_one.rs (every reject's rc + stderr, the table-backed fixture) + `just certify` |
