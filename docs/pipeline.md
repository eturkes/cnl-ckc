# Deterministic document pipeline

`tools/pipeline.py` orchestrates the complete project-owned document chain from validated ACE input through DRS, IR, program, and result records. Its sole authored source is `tools/pipeline.emm`; the Python file is a byte-accepted regeneration artifact. The pipeline starts processes, moves and hashes raw bytes, validates filesystem shape, and publishes one directory transaction. It never parses Prolog. Record parsing, validation, canonicalization, compilation, and inference remain in the project-owned Prolog tools.

## Invocation

Run from the repository root:

```sh
PYTHONDONTWRITEBYTECODE=1 python3 -P tools/pipeline.py APE_TREE_DIR DOCS_DIR OUT_DIR
```

`APE_TREE_DIR` is the APE tree consumed by the [deterministic ACE front end](ace-front-end.md). `DOCS_DIR` is its flat ACE and optional Ulex input directory. `OUT_DIR` names a new published tree. The `SWIPL` environment variable selects the SWI-Prolog executable; its default is `swipl`.

The generated program has strict `Require` guards for both `src/prolog/ir_tool.pl` and `tools/ace_front_end.py`, so repository-root execution is a precondition. Child commands use argument vectors and do not involve a shell.

## Pre-flight contract

After `argparse` accepts exactly three positional arguments, every pre-flight check completes in this fixed order before any child process starts and before any directory or file is created:

| Order | Condition | Failure |
|---:|---|---|
| 1 | `APE_TREE_DIR` is a directory. | Exit 2; `pipeline: ape-tree: not a directory: <path>`. |
| 2 | `DOCS_DIR` is a directory. | Exit 2; `pipeline: docs-dir: not a directory: <path>`. |
| 3 | `OUT_DIR` does not exist, including as a symlink. | Exit 2; `pipeline: out-dir: already exists: <path>`. |
| 4 | The parent of `OUT_DIR` is a directory. | Exit 2; `pipeline: out-dir: parent is not a directory: <path>`. |
| 5 | No sorted parent entry begins with the exact filename prefix `<OUT_DIR-basename>.tmp.`. | Exit 2; `pipeline: staging: stale staging: <entry-name>`. |
| 6 | `shutil.which` resolves `os.environ.get("SWIPL", "swipl")`. | Exit 2; `pipeline: swipl-exec: not executable: <value>`. |

The stale-staging check uses sorted `Path.iterdir()` entries and `str.startswith`, not a glob. Metacharacters in an output basename therefore remain literal bytes rather than becoming pattern syntax. The first sorted stale entry is the deterministic failure detail.

A wrong positional count exits 2 with the normal `argparse` usage diagnostic, zero stdout, and no writes. Every pipeline-owned failure emits exactly one sanitized stderr line of the form `pipeline: <class>: <detail>`; embedded LF and CR characters in the detail become the visible two-character spellings `\n` and `\r`. Pipeline-owned failures emit zero stdout.

## Staging and front-end child

After pre-flight, the pipeline creates the sibling staging directory `<OUT_DIR>.tmp.<pid>` with a plain `mkdir`. It then invokes the front end as:

```text
PYTHON -P tools/ace_front_end.py APE_TREE_DIR DOCS_DIR <OUT_DIR>.tmp.<pid>/front
```

`PYTHON` is `sys.executable`. The environment is inherited, so the already-resolved `SWIPL` selection reaches the child. No stdin is supplied. Successful child stdout is captured and discarded.

A nonzero front-end exit removes the complete staging tree, relays the child's captured stderr bytes verbatim, and exits with the child's exact status. The pipeline adds no prefix, newline, or interpretation. A front-end exit of 0 with any stderr bytes is instead a pipeline integrity failure: staging is removed and the pipeline exits 2 with `pipeline: child-stderr: non-empty stderr from ace front end`.

After a successful child, `front/` must be a real directory. Every sorted entry must be a regular non-symlink file named either `manifest.pl` or with the suffix `.drs.pl`; `manifest.pl` must be present and at least one `.drs.pl` file must exist. Any violation removes staging and exits 2 with a single `pipeline: front-out: <detail>` line. Document IDs are the sorted `.drs.pl` filename stems. This is a defensive byte-and-name inventory only; the front end remains responsible for its document-ID contract.

## Per-document stage chain

The pipeline creates `chain/` inside staging and processes document IDs in sorted order. Every stage runs in a fresh ambient-init-free SWI-Prolog process with this exact argument vector:

```text
SWIPL -q -f none -F none -s src/prolog/ir_tool.pl -g main -t halt(9) -- STAGE
```

For each document, the stage order and files are:

| Stage | Fresh stdin read | Successful output |
|---|---|---|
| `lower` | `front/<docid>.drs.pl` | Non-empty stdout written to `chain/<docid>.ir.pl`. |
| `validate` | The just-written `chain/<docid>.ir.pl` | Zero stdout and zero stderr; no artifact. |
| `compile` | A new read of `chain/<docid>.ir.pl` | Non-empty stdout written to `chain/<docid>.program.pl`. |
| `run` | A new read of `chain/<docid>.program.pl` | Non-empty stdout written to `chain/<docid>.result.pl`. |

Each invocation obtains stdin from a fresh `Path.read_bytes()` call on the prior on-disk artifact. The pipeline never carries a transforming stage's in-memory stdout directly into the next process. The bytes installed on disk are therefore the bytes that continue the chain.

For `lower`, `compile`, and `run`, exit 0 requires empty stderr and non-empty stdout. Non-empty stderr fails as `pipeline: stage-stderr: non-empty stderr for stage: <stage> document: <docid>`; empty stdout fails as `pipeline: stage-stdout: empty stdout for stage: <stage> document: <docid>`. For `validate`, exit 0 requires the explicit zero-stream gate: any stdout first fails as `pipeline: stage-stdout: non-empty stdout for stage: validate document: <docid>`, and otherwise any stderr fails as `pipeline: stage-stderr: non-empty stderr for stage: validate document: <docid>`. Every such integrity failure removes staging, exits 2, emits zero stdout, and leaves `OUT_DIR` absent.

A nonzero IR-tool exit at any stage removes staging, relays stderr verbatim, and preserves the child status exactly. The pipeline does not parse or restate an `ir_tool_error(...)` record.

## Pipeline manifest

After every document succeeds, the pipeline writes UTF-8 `manifest.pl` with LF line endings. It begins:

```prolog
cnl_pipeline_manifest(1).
```

One row follows per document in sorted document-ID order, with no spaces after commas:

```prolog
document(docid('<docid>'),drs_sha256('<drs-hex>'),ir_sha256('<ir-hex>'),program_sha256('<program-hex>'),result_sha256('<result-hex>'),front_manifest_sha256('<front-manifest-hex>')).
```

Every digest is lowercase SHA-256 over a fresh read of the complete raw on-disk artifact bytes: `front/<docid>.drs.pl`, `chain/<docid>.ir.pl`, `chain/<docid>.program.pl`, `chain/<docid>.result.pl`, and `front/manifest.pl`. The front-manifest digest is deliberately repeated on every row. Each row therefore binds one document's full chain to the exact validated document-set context, while every published artifact kind is digest-bound.

## Atomic publication and success streams

Publication is one `os.replace(staging, OUT_DIR)` operation. No file is copied from staging after that rename. The output tree is:

```text
OUT_DIR/
  manifest.pl
  front/
    manifest.pl
    <docid>.drs.pl
  chain/
    <docid>.ir.pl
    <docid>.program.pl
    <docid>.result.pl
```

On success, stdout contains one line per known published file in sorted POSIX-relative-path order:

```text
pipeline: wrote OUT_DIR/<relpath>
```

The final line is `pipeline: ok <N> documents`. Success exits 0 with empty stderr. The inventory is built from the known front and chain files rather than a recursive filesystem scan. No timestamp, process ID, hostname, duration, or staging path appears in any published byte or success line.

Pipeline-owned failures after staging begins remove staging before reporting. Child rejections remove staging before verbatim relay. Filesystem failures during reads, writes, removal, or replacement remain uncaught fail-loud environment errors, matching the regeneration tooling's posture.

## Crash recovery and immutability

A killed process may leave `<OUT_DIR>.tmp.<pid>`. The next run refuses to proceed because the stale-staging pre-flight makes that leak visible. The operator removes the stale entry and reruns. A rerun regenerates the front output and every downstream stage from scratch; it never adopts a partial prior tree, so recovery neither duplicates completed work nor loses an unreported partial result.

The absent-output precondition and single directory replacement make a completed `OUT_DIR` immutable by convention. Rebuilding means choosing a new output path or deliberately removing the old published tree before a fresh run.

## Determinism and acceptance gates

Byte stability follows from fixed pre-flight order, sorted document IDs, fresh child processes with ambient init files disabled, fresh on-disk byte reads between stages, canonical Prolog-owned serialization, fixed manifest templates, raw-byte SHA-256 identities, sorted success reporting, and a single atomic publish. The process-specific staging suffix disappears during publication and is never included in an artifact.

`tests/pipeline-cli-harness.sh` is an offline 17-gate harness. It supplies a deterministic scratch `SWIPL` stub, compares two complete fresh publications to each other and to committed goldens, independently verifies every manifest digest tie, pins usage and pre-flight failures, proves front-end and IR-tool rejection relay, exercises transforming-stage stream guards and the validate zero-stream gate, checks zero residue, and preserves the repository status it observed at start. The CI `test` job lints this harness and runs it after the strict compiler and regeneration harnesses. Its runtime dependencies are Bash, Python, Git, GNU findutils, GNU grep, GNU diffutils, and GNU coreutils; it performs no APE build, network access, or real SWI-Prolog invocation.
