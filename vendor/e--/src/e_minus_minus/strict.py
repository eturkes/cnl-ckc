"""Strict, deterministic E-- compilation entry point."""

from __future__ import annotations

import os
import sys
import warnings
from typing import NoReturn, cast

from .emitter import emit
from .errors import EmmSyntaxError
from .lexer import tokenize
from .parser import parse


_USAGE = "usage: e_minus_minus.strict [-o OUTPUT] INPUT"
_QUARANTINED_MODULES = (
    "e_minus_minus.transpiler",
    "e_minus_minus.normalizer",
    "e_minus_minus.resolver",
    "e_minus_minus.cli",
    "anthropic",
)


class _StrictFailure(Exception):
    def __init__(self, category: str, detail: str, exit_code: int):
        super().__init__(detail)
        self.category = category
        self.detail = detail
        self.exit_code = exit_code


def _fail(category: str, detail: str, exit_code: int) -> NoReturn:
    raise _StrictFailure(category, detail, exit_code)


def _one_line(value: object) -> str:
    return str(value).replace("\r", "\\r").replace("\n", "\\n")


def _diagnose(category: str, detail: str) -> None:
    sys.stderr.write(f"strict:{category}: {_one_line(detail)}\n")


def _parse_argv(argv):
    if len(argv) == 1 and not argv[0].startswith("-"):
        return None, argv[0]
    if len(argv) == 3 and argv[0] == "-o" and not argv[2].startswith("-"):
        return argv[1], argv[2]
    _fail("usage", _USAGE, 2)


def _read_input(path: str) -> bytes:
    try:
        with open(path, "rb") as source:
            return source.read()
    except OSError:
        _fail("io", f"cannot read input {path!r}", 2)


def _decode_input(data: bytes) -> str:
    try:
        text = data.decode("utf-8", errors="strict")
    except UnicodeDecodeError as error:
        _fail("encoding", f"invalid UTF-8 at byte offset {error.start}", 1)
    if text and ord(text[0]) == 0xFEFF:
        _fail("bom", "leading U+FEFF is forbidden", 1)
    return text


def _check_controls(text: str) -> None:
    for offset, char in enumerate(text):
        codepoint = ord(char)
        forbidden = (
            (codepoint < 0x20 and codepoint != 0x0A)
            or 0x7F <= codepoint <= 0x9F
            or codepoint in (0x2028, 0x2029)
        )
        if forbidden:
            _fail(
                "control-char",
                f"U+{codepoint:04X} at character offset {offset}",
                1,
            )


def _quarantine_modules() -> None:
    modules = cast(dict[str, object], sys.modules)
    for name in _QUARANTINED_MODULES:
        modules[name] = None


def _tokenize(text: str):
    try:
        tokens = tokenize(text)
    except EmmSyntaxError as error:
        _fail("syntax", str(error), 1)

    for token in tokens:
        if token.kind == "SLOT":
            _fail("slot", f"line {token.line}: slots are forbidden", 1)
    for token in tokens:
        if token.kind != "STRING" and not token.value.isascii():
            _fail(
                "non-ascii",
                f"line {token.line}: {token.kind} token contains non-ASCII text",
                1,
            )
    return tokens


def _parse(tokens):
    try:
        return parse(tokens)
    except EmmSyntaxError as error:
        _fail("syntax", str(error), 1)


def _slot_backstop(*_args, **_kwargs):
    raise RuntimeError("slot reached strict emitter")


def _emit_and_validate(program) -> str:
    python_source = emit(program, _slot_backstop)
    try:
        with warnings.catch_warnings():
            warnings.simplefilter("error")
            compile(
                python_source,
                "<strict>",
                "exec",
                dont_inherit=True,
                optimize=0,
            )
    except (SyntaxError, ValueError, Warning) as error:
        _fail("python-invalid", str(error), 1)
    return python_source


def _write_file(path: str, payload: bytes) -> None:
    temporary = f"{path}.tmp.{os.getpid()}"
    try:
        with open(temporary, "wb") as destination:
            destination.write(payload)
        os.replace(temporary, path)
    except OSError:
        try:
            os.unlink(temporary)
        except OSError:
            pass
        _fail("io", f"cannot write output {path!r}", 2)


def _write_stdout(payload: bytes) -> None:
    try:
        sys.stdout.buffer.write(payload)
        sys.stdout.buffer.flush()
    except OSError:
        _fail("io", "cannot write standard output", 2)


def _run(argv) -> int:
    output_path, input_path = _parse_argv(argv)
    data = _read_input(input_path)
    text = _decode_input(data)
    _check_controls(text)
    _quarantine_modules()
    tokens = _tokenize(text)
    program = _parse(tokens)
    python_source = _emit_and_validate(program)
    payload = python_source.rstrip("\n").encode("utf-8") + b"\n"
    if output_path is None:
        _write_stdout(payload)
    else:
        _write_file(output_path, payload)
    return 0


def main(argv) -> int:
    try:
        return _run(argv)
    except _StrictFailure as error:
        _diagnose(error.category, error.detail)
        return error.exit_code
    except Exception as error:
        _diagnose("internal", f"unexpected {type(error).__name__}")
        return 70


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
