# Strict E-- compilation profile

Status: normative for the trusted E-- bootstrap path. `vendor/e--/src/e_minus_minus/strict.py` implements this profile with Python standard-library code plus allowlisted CORE compiler modules.

## Input and output contract

- Input is exactly one filesystem path; stdin is outside the interface.
- The input file is read as bytes, then decoded with strict UTF-8.
- Accepted source contains LF (`U+000A`) line endings, no leading BOM, and no forbidden control code points.
- Default output is emitted Python written to `sys.stdout.buffer` as UTF-8 with exactly one trailing LF.
- `-o OUTPUT` writes binary bytes to same-directory `OUTPUT.tmp.<pid>`, closes the temporary file, then atomically installs it with `os.replace`.
- Validation always completes before output begins. Every language, strictness, or Python-validity failure leaves stdout empty.
- Success writes no stderr. Failure writes exactly one stderr line and no traceback: `strict:<class>: <detail>`.

Canonical invocation:

```sh
PYTHONPATH=vendor/e--/src PYTHONDONTWRITEBYTECODE=1 python3 -P -m e_minus_minus.strict INPUT
```

`-P` keeps the current working directory off Python's import path; the canonical `PYTHONPATH` selects the fork-pinned compiler.

## Trusted language extensions

- `Use IDENT (DOT IDENT)*.` imports that module and is accepted only at top level. One-segment and multi-segment module names are valid; a nested `Use` is a syntax failure.
- Expression operands and `[[...]]` call targets accept `IDENT (DOT IDENT)*`. A dot followed by an identifier extends the name; the final dot terminates the statement. Assignment targets remain a single identifier, so dotted assignment is rejected.
- `Require that EXPR.` emits `if not (EXPR):` followed by a four-space-indented `raise AssertionError("requirement failed")`. The expression is evaluated once, and failure remains active under `python3 -O`.
- `Exit with EXPR.` emits `raise SystemExit(EXPR)`.

## Ordered pipeline

The implementation applies every stage in this order:

1. **Binary read** — open and read `INPUT` as bytes. Open/read failure emits `io`, exit 2.
2. **Strict decode** — decode UTF-8 with error handling set to `strict`. Decode failure emits `encoding`, exit 1.
3. **BOM gate** — leading `U+FEFF` emits `bom`, exit 1.
4. **Whole-text control gate** — scan every decoded character, including string contents, with `ord()`. Reject `U+0000`–`U+001F` except `U+000A`, all `U+007F`–`U+009F`, `U+2028`, and `U+2029`; emit `control-char`, exit 1.
5. **Poison quarantine** — assign `None` in `sys.modules` for `e_minus_minus.transpiler`, `e_minus_minus.normalizer`, `e_minus_minus.resolver`, `e_minus_minus.cli`, and `anthropic`. Any accidental import becomes an unexpected `ImportError`, handled as `internal`, exit 70.
6. **Tokenize** — call the fork-pinned lexer. `EmmSyntaxError` emits `syntax`, exit 1.
7. **Token strictness** — any `SLOT` token emits `slot`, exit 1. Every token value except `STRING` must satisfy `str.isascii()`; violation emits `non-ascii`, exit 1.
8. **Parse** — call the fork-pinned parser. `EmmSyntaxError` emits `syntax`, exit 1.
9. **Emit** — call the fork-pinned emitter. Slots have already failed; the injected resolver is an unreachable backstop.
10. **Python gate** — call `compile(py_src, "<strict>", "exec", dont_inherit=True, optimize=0)` inside `warnings.catch_warnings()` with `warnings.simplefilter("error")`. `SyntaxError`, `ValueError`, or any warning promoted to an exception emits `python-invalid`, exit 1. The code object is discarded; this stage never executes emitted Python.
11. **Binary output** — encode emitted source as UTF-8, normalize its suffix to exactly one LF, then write stdout or atomically replace `OUTPUT`. Write failure emits `io`, exit 2.

A top-level catch converts every other `Exception` into one `internal` diagnostic and exit 70.

## CLI

| Form | Result |
|---|---|
| `INPUT` | Compile one file; write Python bytes to stdout. |
| `-o OUTPUT INPUT` | Compile one file; atomically replace `OUTPUT`; stdout stays empty. |
| Missing, extra, or unknown arguments | Emit `usage`; exit 2. |

Stable usage detail: `usage: e_minus_minus.strict [-o OUTPUT] INPUT`.

## Exit codes

| Code | Meaning |
|---:|---|
| 0 | Compilation and output succeeded. |
| 1 | Source failed UTF-8, BOM, control, token, syntax, slot, or emitted-Python validation. |
| 2 | CLI usage or filesystem I/O failed. |
| 70 | An unexpected internal exception reached the wrapper. |

## Diagnostic classes

| Class | Trigger | Exit |
|---|---|---:|
| `usage` | Argument vector differs from `INPUT` or `-o OUTPUT INPUT`. | 2 |
| `io` | Input read or output write/replace fails. | 2 |
| `encoding` | Input bytes are not strict UTF-8. | 1 |
| `bom` | Decoded text starts with `U+FEFF`. | 1 |
| `control-char` | Whole-text scan finds a forbidden code point. | 1 |
| `non-ascii` | A non-`STRING` token value contains a non-ASCII code point. | 1 |
| `slot` | The lexer emits a `SLOT` token for `{{ ... }}`. | 1 |
| `syntax` | The fork-pinned lexer or parser raises `EmmSyntaxError`. | 1 |
| `python-invalid` | CPython rejects or warns about emitted source during `compile()`. | 1 |
| `internal` | Any other `Exception` escapes a pipeline stage. | 70 |

Diagnostic class and prefix are stable. `python-invalid` detail may include the running CPython message and is intentionally unpinned.

## Profile decisions

- **Warnings are errors.** Invalid escapes such as `"\q"` already warn on some CPython releases and may become syntax errors later. Promotion makes the strict result immediate, deterministic for a given pinned runtime, and forward-compatible.
- **Escape spelling passes through as data.** The lexer preserves a source escape as its two ASCII characters inside `STRING`; strict token checks do not decode it. The Python gate accepts valid spellings such as `\n` and rejects invalid spellings. This keeps source bytes authoritative.
- **The control set is code-point-defined.** `ord()` range checks avoid `unicodedata` category tables and Unicode-version drift; the fixed set includes CR, tab, vertical tab, form feed, file/group/record separators, NUL, NEL, and Unicode line/paragraph separators.
- **ASCII is required outside strings.** Keywords, identifiers, operators, numbers, and structural token values are pure ASCII. Unicode is data only when carried by `STRING` content.
- **Slots always fail closed.** Token rejection is the primary gate; poisoned module entries and the emitter resolver are independent backstops against the quarantined legacy path.
