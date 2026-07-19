# Deterministic ACE front end

`tools/ace_front_end.py` orchestrates one deterministic ACE-to-DRS run per document. Its sole authored source is `tools/ace_front_end.emm`; the Python file is a byte-accepted regeneration artifact. The glue validates the document set, starts processes, hashes and moves bytes, and writes provenance envelopes. It does not parse, transform, or interpret a DRS. All DRS parsing, acceptance, canonicalization, and semantic decisions remain in the hand-authored Prolog adapter.

## Invocation

Run from the repository root:

```sh
PYTHONDONTWRITEBYTECODE=1 python3 -P tools/ace_front_end.py APE_TREE_DIR DOCS_DIR OUT_DIR
```

`APE_TREE_DIR` is a scratch copy of `vendor/ape/` after `make plp` has produced `prolog/parser/grammar.plp`. `DOCS_DIR` is the input directory, and `OUT_DIR` is a new output directory. The `SWIPL` environment variable selects the SWI-Prolog executable; its default is `swipl`. Before any adapter run, the glue resolves that value with `shutil.which` and requires an executable.

The generated program has a strict `Require` guard for `src/prolog/adapter.pl`, so repository-root execution is a precondition. Each document uses this adapter command, with a final Ulex argument only when the matching file exists:

```text
SWIPL -q -f none -F none -s src/prolog/adapter.pl -g main -t halt(9) -- APE_TREE_DIR [ULEX_FILE]
```

The argument-vector invocation does not involve a shell. The ACE file's raw bytes are supplied on stdin.

## Input directory and document IDs

`DOCS_DIR` must satisfy all of these rules before any adapter process starts:

- It exists and is a directory.
- It is flat. Every entry is a regular, non-symlink file.
- Every filename ends in `.ace` or `.ulex`.
- At least one `.ace` file exists.
- Every `.ulex` file has a same-stem `.ace` sibling.

A document ID is the filename with its final `.ace` suffix removed. It is an interim filename-stem ID until the M5 guideline registry assigns IDs. It must be non-empty, contain only `[a-z0-9-]`, and not begin with `-`. The same rule applies to a Ulex filename stem. Filesystem filenames make the ACE stems unique.

`OUT_DIR` must not exist, including as a symlink, and its parent must already be a directory. This condition is also checked before any adapter process starts.

Documents are processed in sorted document-ID order. Each document gets a fresh SWI-Prolog process with ambient init files disabled by `-f none -F none`.

## Identity and sentence provenance

A document's durable identity is the pair of its `docid` and the SHA-256 digest of its raw ACE source bytes. The digest is not computed from decoded text.

APE's `-SentId/TokId` annotations restart at 1 because every document is parsed in a fresh process. Within one record, `/(SentId,TokId)` therefore identifies `(sentence ordinal, token ordinal)` under that record's `source_sha256`. A sentence's durable provenance is the document digest plus its sentence ordinal. Raw sentence or token ordinals must never be cited without the containing record's document identity.

For a document with a sibling Ulex file, its SHA-256 is computed from one upfront read of the raw `.ulex` bytes. After the adapter run, the path is read again and must match that upfront byte snapshot. This re-verification enforces the record's exact-per-run Ulex binding.

## Record format

The file name is `<docid>.drs.pl`. Its bytes are exactly two UTF-8 header lines followed by the adapter stdout bytes verbatim:

```prolog
ace_front_end_record(1).
document(docid('<docid>'),source_sha256('<source-hex>'),ulex(<ulex-term>)).
<canonical-drs-line>
```

All lines end in LF. There are no spaces after commas, and atoms in the identity fields use single quotes. `<source-hex>` is lowercase SHA-256 over the raw `.ace` bytes. `<ulex-term>` is `none` when there is no sibling Ulex, or `sha256('<ulex-hex>')` where `<ulex-hex>` is lowercase SHA-256 over the raw `.ulex` bytes. `<canonical-drs-line>` is the adapter's already-canonical, LF-terminated stdout line; the glue does not inspect it.

The leading `ace_front_end_record(1).` term versions the envelope independently of later IR formats.

## Manifest format

`manifest.pl` begins with its version term, followed by one line per document in sorted document-ID order:

```prolog
ace_front_end_manifest(1).
document(docid('<docid>'),source_sha256('<source-hex>'),record_sha256('<record-hex>')).
```

`<record-hex>` is lowercase SHA-256 over the complete corresponding record file, including both header lines and the adapter's final LF. The manifest has no timestamps, host paths, process IDs, or durations.

## User lexicon wiring

A same-stem `.ulex` file is passed as the adapter's optional final argument. The adapter loads it after APE and before reading or parsing ACE input. Fresh per-document processes prevent Ulex state from carrying into another document. Entry forms, priority over Clex, rejection rules, and producer requirements are defined in [the Ulex contract](ulex.md); the glue adds no lexicon semantics.

## Fail-closed behavior

All validation and every adapter run complete before `OUT_DIR` is created. Processing stops at the first failing document, which is deterministic because document IDs are sorted. The four glue integrity failures below all exit 2, emit zero stdout, leave `OUT_DIR` absent, and emit one sanitized `ace-front-end: <class>: <detail>` line.

| Condition | Exit | Stdout | Stderr |
|---|---:|---|---|
| Wrong positional argument count | 2 | Empty | `argparse` usage diagnostic. |
| Invalid or empty input set, unsupported entry, non-regular entry, or orphan Ulex | 2 | Empty | Exactly one `ace-front-end: docs-dir: <detail>` line. |
| Invalid ACE or Ulex filename stem | 2 | Empty | Exactly one `ace-front-end: docid: <detail>` line. |
| Existing `OUT_DIR` or missing/non-directory parent | 2 | Empty | Exactly one `ace-front-end: out-dir: <detail>` line. |
| Selected `SWIPL` is not found or is not executable (`shutil.which` preflight) | 2 | Empty | Exactly one `ace-front-end: adapter-exec: <detail>` line. |
| Adapter exits 0 after emitting any stderr bytes | 2 | Empty | Exactly one `ace-front-end: adapter-stderr: <detail>` line. |
| Adapter exits 0 with empty stdout or stdout that is not exactly one LF-terminated line | 2 | Empty | Exactly one `ace-front-end: adapter-stdout: <detail>` line. |
| Ulex bytes differ from the upfront snapshot after the adapter run | 2 | Empty | Exactly one `ace-front-end: ulex-changed: <detail>` line. |
| Adapter rejects ACE or Ulex | 1 | Empty | Captured adapter stderr bytes relayed verbatim. |
| Adapter setup or APE/Ulex load fails | 2 | Empty | Captured adapter stderr bytes relayed verbatim. |
| Any other nonzero adapter status | Adapter status | Empty | Captured adapter stderr bytes relayed verbatim. |
| Success | 0 | One `ace-front-end: wrote OUT_DIR/<filename>` line per file in sorted filename order, then `ace-front-end: ok <N> documents`. | Empty. |

The adapter's own exit surface is 0 = accepted; 1 = `input_utf8` (stdin fails strict RFC 3629 validation, including overlong, surrogate, out-of-range, or truncated sequences), `ape_messages`, or `empty_drs`; 2 = `usage`, `ape_load`, `ulex_load`, or `uncaught`. APE load-time warnings as well as errors fail as `ape_load`. The exact Prolog-side stream contract remains normative in `src/prolog/adapter.pl`.

## Writes and crash recovery

Validation failures, adapter failures, and successful-adapter output guards leave `OUT_DIR` absent: records, the manifest, and even the directory are buffered until every document succeeds. On success, the glue creates `OUT_DIR` with a plain `mkdir`, then writes each buffered file with an ordinary file write. These post-`mkdir` writes are intentionally not an atomic directory transaction. If the process or filesystem fails during that write phase, remove the partial `OUT_DIR` before rerunning.

The absent-output precondition prevents accidental overwrite and makes a completed run immutable by convention.

## Determinism and acceptance gates

Byte stability follows from the following constraints:

- input files and document IDs are validated, then document IDs are sorted;
- source and Ulex identities hash raw bytes;
- each document runs in a fresh SWI-Prolog process with ambient init files disabled;
- the Prolog adapter owns canonical DRS serialization;
- record headers and manifest terms use fixed, versioned byte templates;
- manifest rows use sorted document-ID order, and files are written in sorted filename order;
- accepted artifacts contain no timings, banners, host paths, or other run-specific data.

`tests/pipeline-harness.sh` builds a fresh scratch APE tree, runs the three-document front-end chain twice, compares both file sets and every output byte, and compares the first run to committed goldens. Its two-sentence fixture also requires both `/(1,` and `/(2,` annotations. Rejection gates cover an OOV document, invalid ID, orphan Ulex, existing output directory, CLI usage, successful-adapter stderr, empty or malformed successful stdout, missing executable, Ulex mutation, missing APE tree, zero writes, and vendor cleanliness.

`tests/slice-harness.sh` independently stages a fresh APE tree and runs both committed slice documents twice from ACE + Ulex through DRS, IR, program, and answer records. It pins front-end stdout and file sets, byte-compares each artifact-producing stage to its authoritative golden before continuing, treats validation as a separate zero-stream gate, proves the two passes' complete artifact sets are byte-identical, checks lower-stage failure leaves no non-empty downstream artifact, and verifies vendor cleanliness.

The CI `test` job checks regeneration identity and lints all nine shell harnesses. The pinned SWI 9.2.9 `ape` job runs the adapter and pipeline harnesses, then the slice harness immediately after the pipeline harness and before its final repository-cleanliness check.
