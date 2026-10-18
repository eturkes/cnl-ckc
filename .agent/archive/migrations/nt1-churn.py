#!/usr/bin/env python3
"""NT1 D12 independent migration oracle; Python 3.13 stdlib, no compiler calls.

Usage: python3 -I nt1-churn.py <base-commit> <repo-root>
Exit 0 = predicted tree, 1 = unpredicted changes/input errors, 2 = broken self-test.

The base's tracked fixture/corpus blobs are the inputs, not regenerated pins.
Inputs = immutable base blobs + contract; working bytes only enter compare.
Production source/docs + release-manifest remain outside this fixture oracle.
Rulings: .agent/archive/migrations/nt1-churn-rulings.tsv (see load_rulings).
"""

from __future__ import annotations

import collections
import dataclasses
import hashlib
import json
import os
import re
import stat
import subprocess
import sys
from pathlib import Path, PurePosixPath

sys.set_int_max_str_digits(0)  # D1 integers have no decimal-width bound.

RULINGS = ".agent/archive/migrations/nt1-churn-rulings.tsv"
TABLE_HEADER = b"# format: kind\tlemma\tvalue\n"
INDICATORS = (
    ("schema_version", 1), ("document", 4), ("entity", 4),
    ("cardinality", 5), ("event", 3), ("arg", 4), ("pp", 4),
    ("property", 4), ("operator", 3), ("interval", 6),
    ("recurrence", 4), ("frequency", 5), ("order", 4),
    ("recurrence_window", 7), ("range", 3), ("grade", 5), ("preference", 4), ("qualifier", 4),
)
COUPLED = frozenset({
    "rust/ckc-kani-harness/src/lib.rs",
})
HEX64 = re.compile(rb"(?<![0-9a-f])[0-9a-f]{64}(?![0-9a-f])")
SHA_TERM = re.compile(rb"(?<=sha256\()('?)([0-9a-f]{64})\1(?=\))")
MARKER = re.compile(rb"(?m)^% S[0-9]+:")
OFFSET = re.compile(rb"noncanonical\(([0-9]+),([0-9]+)\)")
DOC_RECORD = re.compile(rb"(?m)^guideline_document\(([^\n]*)\)\.\n")
QUERY_RECORD = re.compile(rb"(?m)^'\$guideline_query'\(([^\n]*)\)\.\n")
SCHEMA = re.compile(rb"(?m)^guideline_schema_version\(([123])\)\.\n")


class InputError(Exception):
    pass


@dataclasses.dataclass(frozen=True)
class Blob:
    data: bytes
    mode: str = "100644"


def sha(data: bytes) -> bytes:
    return hashlib.sha256(data).hexdigest().encode("ascii")


def atom(digest: bytes) -> bytes:
    return b"'" + digest + b"'" if digest[:1].isdigit() else digest


def safe_path(path: str) -> str:
    p = PurePosixPath(path)
    if not path or p.is_absolute() or ".." in p.parts or str(p) != path or "\0" in path:
        raise InputError(f"invalid relative path {path!r}")
    return path


def git(root: Path, *args: str, data: bytes | None = None) -> bytes:
    proc = subprocess.run(["git", "-C", str(root), *args], input=data,
                          stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    if proc.returncode:
        raise InputError(f"git {' '.join(args[:2])}: {proc.stderr.decode(errors='replace').strip()}")
    return proc.stdout


def scope_path(path: str) -> bool:
    # Fixture README files are input bytes. Corpus README files are reviewed docs.
    return (path.startswith("tests/") or
            (path.startswith("guidelines/") and not path.endswith("/README.md")) or
            path in COUPLED)


def scope_paths(paths) -> set[str]:
    return {safe_path(p) for p in paths if scope_path(p)}


def base_blobs(root: Path, revision: str) -> dict[str, Blob]:
    history = root
    try:
        commit = git(history, "rev-parse", "--verify", f"{revision}^{{commit}}").strip().decode()
    except InputError:
        # Migration archives have their own fresh Git index, not the base history.
        history = Path(__file__).resolve().parents[3]
        commit = git(history, "rev-parse", "--verify", f"{revision}^{{commit}}").strip().decode()
    entries = {}
    for row in git(history, "ls-tree", "-rz", "--full-tree", commit).split(b"\0"):
        if not row:
            continue
        metadata, name = row.split(b"\t", 1)
        mode, kind, oid = metadata.decode().split()
        path = name.decode("utf-8")
        if scope_path(path):
            if kind != "blob":
                raise InputError(f"{path}: scoped non-blob {kind}")
            entries[safe_path(path)] = mode, oid
    ordered = sorted(entries)
    wire = b"".join(entries[p][1].encode() + b"\n" for p in ordered)
    payload = git(history, "cat-file", "--batch", data=wire)
    result, at = {}, 0
    for path in ordered:
        end = payload.index(b"\n", at)
        oid, kind, size = payload[at:end].split()
        if kind != b"blob" or oid.decode() != entries[path][1]:
            raise InputError(f"{path}: unexpected cat-file response")
        at = end + 1
        size = int(size)
        result[path] = Blob(payload[at:at + size], entries[path][0])
        at += size
        if payload[at:at + 1] != b"\n":
            raise InputError(f"{path}: truncated cat-file response")
        at += 1
    if at != len(payload) or scope_paths(entries) != set(result):
        raise InputError("base scope enumeration incomplete")
    return result


def working_blobs(root: Path) -> tuple[dict[str, Blob], set[str]]:
    tracked = {p.decode("utf-8") for p in git(root, "ls-files", "-z").split(b"\0") if p}
    untracked = {p.decode("utf-8") for p in git(root, "ls-files", "-z", "--others", "--exclude-standard").split(b"\0") if p}
    result = {}
    for path in sorted(scope_paths(tracked | untracked)):
        file = root / path
        try:
            st = file.lstat()
        except FileNotFoundError:
            continue
        if stat.S_ISLNK(st.st_mode):
            result[path] = Blob(os.fsencode(os.readlink(file)), "120000")
        elif stat.S_ISREG(st.st_mode):
            mode = "100755" if st.st_mode & stat.S_IXUSR else "100644"
            result[path] = Blob(file.read_bytes(), mode)
        else:
            raise InputError(f"{path}: scoped path is not a file")
    return result, scope_paths(untracked)


def escaped_bytes(data: bytes) -> str:
    parts = []
    for byte in data:
        if byte == 9:
            parts.append(r"\t")
        elif byte == 10:
            parts.append(r"\n")
        elif byte == 92:
            parts.append(r"\\")
        elif 32 <= byte <= 126:
            parts.append(chr(byte))
        else:
            parts.append(f"\\x{byte:02x}")
    return "".join(parts)


def literal_bytes(text: str) -> bytes:
    result = bytearray()
    at = 0
    while at < len(text):
        char = text[at]
        at += 1
        if char != "\\":
            if not 32 <= ord(char) <= 126:
                raise InputError("literal bytes require printable ASCII or escapes")
            result.append(ord(char))
            continue
        if at == len(text):
            raise InputError("trailing backslash in byte literal")
        escape = text[at]
        at += 1
        if escape in {"t", "n", "\\"}:
            result.append({"t": 9, "n": 10, "\\": 92}[escape])
        elif escape == "x" and re.fullmatch(r"[0-9a-fA-F]{2}", text[at:at + 2]):
            result.append(int(text[at:at + 2], 16))
            at += 2
        else:
            raise InputError(f"unsupported byte-literal escape \\{escape}")
    return bytes(result)


def load_rulings(data: bytes) -> list[dict]:
    r"""TSV: family, item, post-migration path, old, new, count, reason[, base_sha256].

    One # header; literal-byte fields use \t, \n, \\, \xHH (no quotes).
    @whole replaces an existing file; @law records an algorithm ruling without
    granting byte changes. Other rows replace their counted literal anchor.
    Byte allowances require the immutable base digest. Frozen paths are fixed.
    """
    if not data.endswith(b"\n"):
        raise InputError(f"{RULINGS}: final newline")
    lines = data.decode("utf-8").split("\n")[:-1]
    if not lines or not lines[0].startswith("#"):
        raise InputError(f"{RULINGS}: one # header line required")
    rows = []
    for number, line in enumerate(lines[1:], 2):
        fields = line.split("\t")
        if len(fields) not in (7, 8) or line.startswith("#"):
            raise InputError(f"{RULINGS}:{number}: expected seven or eight fields")
        family, item, path, old, new, count, reason = fields[:7]
        if not all((family, item, path, reason)) or not re.fullmatch(r"[1-9][0-9]*", count):
            raise InputError(f"{RULINGS}:{number}: empty field or invalid count")
        safe_path(path)
        digest = fields[7] if len(fields) == 8 and fields[7] else None
        if digest is not None and not re.fullmatch(r"[0-9a-f]{64}", digest):
            raise InputError(f"{RULINGS}:{number}: base_sha256")
        mode = {"@whole": "whole", "@law": "law"}.get(old, "edit")
        if old in {"@new", "@freeze"} or new == "@deleted":
            raise InputError(f"{RULINGS}:{number}: scope/addition/removal ruling forbidden")
        try:
            before = literal_bytes(old) if mode == "edit" else b""
            after = literal_bytes(new)
        except InputError as error:
            raise InputError(f"{RULINGS}:{number}: {error}") from error
        if mode != "edit" and count != "1":
            raise InputError(f"{RULINGS}:{number}: @{mode} count must be 1")
        if mode == "edit" and not before:
            raise InputError(f"{RULINGS}:{number}: empty old field; use @whole")
        try:
            occurrences = int(count)
        except ValueError as error:
            raise InputError(f"{RULINGS}:{number}: count is outside the parser's integer domain") from error
        row = {"family": family, "item": item, "path": path, "base_sha256": digest,
               "edits": [{"old": before, "new": after, "count": occurrences}],
               "reason": reason, "mode": mode}
        rows.append(row)
    return rows


def rulings_for(root: Path) -> list[dict]:
    policy = Path(__file__).resolve().with_name(PurePosixPath(RULINGS).name)
    return load_rulings(policy.read_bytes()) if policy.exists() else []


def declaration_block(indicators) -> bytes:
    return b"".join(f":- {decl}(guideline_{name}/{arity}).\n".encode()
                    for name, arity in indicators for decl in ("multifile", "discontiguous"))


BASE_DECLS = declaration_block(INDICATORS)
ADDED_DECLS = declaration_block((("unit", 3),))
NT1_DECLS = BASE_DECLS + ADDED_DECLS
assert len(ADDED_DECLS) == 68 and ADDED_DECLS.count(b"\n") == 2
BLOCK = re.compile(rb"(?m)^" + re.escape(BASE_DECLS))


def insertion_points(data: bytes) -> list[int]:
    return [m.end() for m in BLOCK.finditer(data) if not data.startswith(ADDED_DECLS, m.end())]


def split_args(data: bytes) -> list[bytes]:
    # Only split the envelope, never parse/re-render a semantic term.
    out, start, depth, quote, escape = [], 0, 0, False, False
    for i, byte in enumerate(data):
        if quote:
            if escape:
                escape = False
            elif byte == 92:
                escape = True
            elif byte == 39:
                quote = False
        elif byte == 39:
            quote = True
        elif byte in (40, 91):
            depth += 1
        elif byte in (41, 93):
            depth -= 1
            if depth < 0:
                raise ValueError("unbalanced envelope")
        elif byte == 44 and depth == 0:
            out.append(data[start:i])
            start = i + 1
    if depth or quote:
        raise ValueError("unterminated envelope")
    return out + [data[start:]]




def document_transform(data: bytes) -> bytes:
    parts, previous = [], 0
    for at in insertion_points(data):
        parts.extend((data[previous:at], ADDED_DECLS))
        previous = at
    return b"".join(parts) + data[previous:]


def expand_template(data: bytes) -> bytes:
    if not data.startswith(b"@@sha256 ") or not data.endswith(b"\n"):
        raise InputError("malformed .tpl.in template")
    body = []
    for line in data.split(b"\n")[1:-1]:
        if line.startswith(b"@@run "):
            _, lo, hi, text = line.split(b" ", 3)
            body.extend(text.replace(b"@i@", str(i).encode()).replace(b"@j@", str(i + 1).encode())
                        for i in range(int(lo), int(hi) + 1))
        elif line.startswith(b"@@tree "):
            n, first, leaf, prefix, suffix = line[7:].split(b"\t")
            layer = [first] + [leaf] * (int(n) - 1)
            while len(layer) > 1:
                layer = [b"','(" + layer[i] + b"," + layer[i + 1] + b")" if i + 1 < len(layer) else layer[i]
                         for i in range(0, len(layer), 2)]
            body.append(prefix + layer[0] + suffix)
        else:
            body.append(line)
    return b"\n".join(body) + b"\n"


def template_pin(data: bytes) -> bytes:
    return b"@@sha256 " + sha(expand_template(data)) + b"\n" + data.split(b"\n", 1)[1]


def table_file(path: str) -> bool:
    return PurePosixPath(path).name.endswith(("temporal.tsv", "temporal.tsv.in", "vocabulary.tsv", "vocabulary.tsv.in")) or path == "tests/certify-one/v3-header.txt.in"


def artifact_kind(data: bytes) -> str | None:
    if data.startswith(b"@@sha256 "):
        return artifact_kind(data.split(b"\n", 1)[1])
    if DOC_RECORD.search(data) and (SCHEMA.search(data) or b":- multifile(guideline_" in data):
        return "doc"
    if QUERY_RECORD.search(data):
        return "query"
    if re.search(rb"(?m)^'\$guideline_answers'\(", data):
        return "answers"
    if re.search(rb"(?m)^'\$guideline_traces'\(", data):
        return "traces"
    return None


def rebind_bytes(data: bytes, mapping: dict[bytes, bytes]) -> bytes:
    data = SHA_TERM.sub(lambda m: atom(mapping[m[2]]) if m[2] in mapping else m.group(), data)
    return HEX64.sub(lambda m: mapping.get(m.group(), m.group()), data)


def protected_payload(path: str, data: bytes) -> bool:
    return (artifact_kind(data) is not None or path.endswith((".pl", ".pl.in", ".pl.tpl.in", ".proof.expect")) or
            b"'$guideline_proof'(" in data or path.endswith(("/audit/review-manifest.tsv", "/audit/adjudication.tsv")) or
            (path.endswith((".tsv", ".tsv.in")) and "/align/" in path) or
            path.startswith(("tests/align/", "tests/adjudication/")) or
            table_file(path) or data.startswith(TABLE_HEADER))


def header_line_shift(data: bytes, line: int) -> int:
    return 2 * sum(data.count(b"\n", 0, at) < line for at in insertion_points(data))


def first_difference(expected: bytes, actual: bytes) -> str:
    common = min(len(expected), len(actual))
    at = next((i for i in range(common) if expected[i] != actual[i]), common)
    line = expected.count(b"\n", 0, at) + 1
    column = at - expected.rfind(b"\n", 0, at)
    return f"byte {at}, line {line}:{column}; expected={expected[at:at + 48]!r}, actual={actual[at:at + 48]!r}"


def semantic_stream(data: bytes) -> bytes:
    lines = data.removesuffix(b"\n").split(b"\n")
    return b"".join(line + b"\n" for line in lines
                    if not line.startswith((b"%", b":- ", b"guideline_document(")))






def ui_escape(data: bytes) -> bytes:
    return data.replace(b"&", b"&amp;").replace(b"<", b"&lt;").replace(b">", b"&gt;")


def git_object(kind: bytes, data: bytes) -> bytes:
    return hashlib.sha1(kind + b" " + str(len(data)).encode() + b"\0" + data).digest()


def fixture_commit(files: dict[str, Blob]) -> bytes:
    """Git SHA-1 tree/commit bytes; same fixed fixture identity/date as the base."""
    nodes = {}
    for path, blob in files.items():
        at = nodes
        parts = PurePosixPath(path).parts
        for part in parts[:-1]:
            at = at.setdefault(part, {})
        if parts[-1] in at:
            raise InputError(f"fixture tree path collision: {path}")
        at[parts[-1]] = blob

    def tree(node):
        result = []
        for name, child in sorted(node.items(), key=lambda pair: pair[0].encode() + (b"/" if isinstance(pair[1], dict) else b"")):
            if isinstance(child, dict):
                mode, oid = b"40000", tree(child)
            else:
                mode, oid = child.mode.encode(), git_object(b"blob", child.data)
            result.append(mode + b" " + name.encode() + b"\0" + oid)
        return git_object(b"tree", b"".join(result))

    identity = b"fixture <fixture@localhost> 1767225600 +0000\n"
    commit = b"tree " + tree(nodes).hex().encode() + b"\nauthor " + identity + b"committer " + identity + b"\nfixture corpus\n"
    return git_object(b"commit", commit).hex().encode()


RUST_CONCAT = re.compile(rb'concat!\((\s*(?:"(?:[^"\\]|\\.)*"\s*,?\s*)+)\)\s*\.as_bytes\(\)')
RUST_STRING = re.compile(rb'"(?:[^"\\]|\\.)*"')
KANI = "rust/ckc-kani-harness/src/lib.rs"


def concat_payload(match: re.Match) -> bytes:
    try:
        return b"".join(json.loads(literal.group().decode()).encode("utf-8") for literal in RUST_STRING.finditer(match[1]))
    except (ValueError, UnicodeError) as error:
        raise InputError("unsupported Rust string escape in Kani fixture constant") from error


def embedded_artifacts(data: bytes) -> list[bytes]:
    return [concat_payload(m) for m in RUST_CONCAT.finditer(data)]


def render_concat(match: re.Match, data: bytes) -> bytes:
    prefix = match[1][:match[1].index(b'"')]
    indent = prefix.rsplit(b"\n", 1)[-1]
    encoded = [json.dumps(line.decode(), ensure_ascii=False).encode() for line in data.splitlines(keepends=True)]
    suffix = match[1][match[1].rfind(b'"') + 1:]
    body = prefix + (b",\n" + indent).join(encoded) + suffix
    return match.group().replace(match[1], body, 1)




def kani_transform(data: bytes) -> tuple[bytes, list[tuple[bytes, bytes]]]:
    pairs = []

    def transform(match):
        old = concat_payload(match)
        kind = artifact_kind(old)
        new = document_transform(old) if kind == "doc" else old
        pairs.append((old, new))
        if old == new:
            return match.group()
        return render_concat(match, new)

    result = RUST_CONCAT.sub(transform, data)
    docs = [(old, new) for old, new in pairs if artifact_kind(old) == "doc"]
    queries = [(old, new) for old, new in pairs if artifact_kind(old) == "query"]
    if len(docs) != 1 or len(queries) != 2:
        raise InputError("Kani fixture constant census: expected one document and two queries")
    _, new_doc = docs[0]
    for prefix, suffix in ((b"DOC has ", b" bytes."), (b"DOC.len() = ", b";"), (b"bound = ", b" + one")):
        result = re.sub(re.escape(prefix) + rb"[0-9]+" + re.escape(suffix),
                        lambda _: prefix + str(len(new_doc)).encode() + suffix, result)
    result = re.sub(rb"#\[kani::unwind\([0-9]+\)\](\s*fn (?:reader_trace_check_small|engine_answer_tiny)\(\))",
                    lambda m: f"#[kani::unwind({len(new_doc) + 1})]".encode() + m[1], result)
    old_diffs = [i for i, (a, b) in enumerate(zip(queries[0][0], queries[1][0])) if a != b]
    new_diffs = [i for i, (a, b) in enumerate(zip(queries[0][1], queries[1][1])) if a != b]
    if len(old_diffs) != 1 or len(new_diffs) != 1:
        raise InputError("Kani query variant must differ at exactly one goal byte")
    result = result.replace(f"query[{old_diffs[0]}] =".encode(), f"query[{new_diffs[0]}] =".encode())
    return result, pairs


# The term reader is deliberately independent of the compiler and native codec.
# Atoms retain their source spelling; only variables are canonically numbered.
@dataclasses.dataclass(frozen=True)
class Var:
    name: object


@dataclasses.dataclass(frozen=True)
class Term:
    name: bytes
    args: tuple = ()


@dataclasses.dataclass(frozen=True)
class Naf:
    goals: tuple


def compound(name: bytes, *args):
    return Term(name, tuple(args))


def is_term(t, name: bytes, arity: int | None = None):
    return isinstance(t, Term) and unquote(t.name) == name and (arity is None or len(t.args) == arity)


def unquote(raw: bytes) -> bytes:
    if raw.startswith(b"'") and raw.endswith(b"'"):
        return raw[1:-1].replace(b"\\'", b"'").replace(b"\\\\", b"\\")
    return raw


class Reader:
    def __init__(self, data: bytes):
        self.data, self.at = data, 0

    def space(self):
        while self.at < len(self.data) and self.data[self.at] in b" \t\r\n":
            self.at += 1

    def take(self, text):
        self.space()
        if self.data.startswith(text, self.at):
            self.at += len(text)
            return True
        return False

    def need(self, text):
        if not self.take(text):
            raise InputError(f"term byte {self.at}: expected {text!r}, got {self.data[self.at:self.at+30]!r}")

    def term(self):
        self.space()
        if self.take(b"["):
            items = []
            if self.take(b"]"):
                return Term(b"[]")
            while True:
                items.append(self.term())
                if self.take(b"]"):
                    tail = Term(b"[]")
                    break
                if self.take(b"|"):
                    tail = self.term()
                    self.need(b"]")
                    break
                self.need(b",")
            for item in reversed(items):
                tail = compound(b"[|]", item, tail)
            return tail
        start = self.at
        if self.at >= len(self.data):
            raise InputError("term truncated")
        if self.data[self.at] == 39:
            self.at += 1
            while self.at < len(self.data):
                if self.data[self.at] == 92:
                    self.at += 2
                elif self.data[self.at] == 39:
                    self.at += 1
                    break
                else:
                    self.at += 1
            else:
                raise InputError("unterminated quoted atom")
        else:
            while self.at < len(self.data) and self.data[self.at] not in b" \t\r\n(),[]|":
                self.at += 1
        token = self.data[start:self.at]
        if not token:
            raise InputError(f"empty token at {self.at}")
        if re.fullmatch(rb"[A-Z_][A-Za-z_0-9]*", token):
            return Var(token)
        if self.take(b"("):
            args = [self.term()]
            while self.take(b","):
                args.append(self.term())
            self.need(b")")
            return Term(token, tuple(args))
        return Term(token)


def read_term(data: bytes):
    r = Reader(data)
    t = r.term()
    r.space()
    if r.at != len(data):
        raise InputError(f"term trailing bytes {data[r.at:r.at+40]!r}")
    return t


def list_term(items):
    out = Term(b"[]")
    for t in reversed(items):
        out = compound(b"[|]", t, out)
    return out


def list_items(t):
    result = []
    while is_term(t, b"[|]", 2):
        result.append(t.args[0])
        t = t.args[1]
    if t != Term(b"[]"):
        raise InputError("proper list required")
    return result


def render(t, variables=None):
    if isinstance(t, Var):
        if variables is None:
            if not isinstance(t.name, bytes):
                raise InputError("fresh variable needs canonical numbering")
            return t.name
        if t not in variables:
            k = len(variables)
            variables[t] = bytes([65 + k % 26]) + (str(k // 26).encode() if k >= 26 else b"")
        return variables[t]
    if is_term(t, b"[|]", 2):
        parts = []
        while is_term(t, b"[|]", 2):
            parts.append(render(t.args[0], variables))
            t = t.args[1]
        return b"[" + b",".join(parts) + (b"" if t == Term(b"[]") else b"|" + render(t, variables)) + b"]"
    return t.name + (b"(" + b",".join(render(a, variables) for a in t.args) + b")" if t.args else b"")


def flatten(t):
    if is_term(t, b",", 2):
        return flatten(t.args[0]) + flatten(t.args[1])
    return [t]


def conjunction(goals):
    if not goals:
        return Term(b"true")
    result = goals[-1]
    for goal in reversed(goals[:-1]):
        result = compound(b"','", goal, result)
    return result


@dataclasses.dataclass(frozen=True)
class Clause:
    head: Term
    body: tuple


def parse_clause(line):
    reader = Reader(line.rstrip(b"\n").removesuffix(b"."))
    head = reader.term()
    sep = reader.take(b":-")
    body = reader.data[reader.at:]
    if not sep and body.strip():
        raise InputError("clause suffix")
    goals = []
    for part in split_args(body) if sep else []:
        part = part.strip()
        if part.startswith(b"\\+ "):
            inner = part[3:]
            if inner.startswith(b"("):
                if not inner.endswith(b")"):
                    raise InputError("NAF envelope")
                inner = inner[1:-1]
            goals.append(Naf(tuple(read_term(x.strip()) for x in split_args(inner))))
        else:
            goals.append(read_term(part))
    return Clause(head, tuple(goals))


def render_clause(c):
    variables = {}
    head = render(c.head, variables)
    goals = []
    for g in c.body:
        if isinstance(g, Naf):
            inner = b", ".join(render(x, variables) for x in g.goals)
            goals.append(b"\\+ " + (b"(" + inner + b")" if len(g.goals) != 1 else inner))
        else:
            goals.append(render(g, variables))
    return head + (b" :- " + b", ".join(goals) if goals else b"") + b".\n"


def map_vars(t, f):
    if isinstance(t, Var):
        return f(t)
    if isinstance(t, Naf):
        return Naf(tuple(map_vars(x, f) for x in t.goals))
    return Term(t.name, tuple(map_vars(x, f) for x in t.args))


def ground(t):
    return not isinstance(t, Var) and all(ground(x) for x in t.args)


def entity_map(goals):
    return {g.args[1]: (g.args[2], g.args[3]) for g in goals if is_term(g, b"guideline_entity", 4)}


def unit_table(data):
    units = {}
    for row in data.splitlines():
        fields = row.split(b"\t")
        if fields[:1] == [b"unit"] and len(fields) == 3:
            units[fields[1]] = fields[2]
    return units


CONVERTIBLE = {b"exactly", b"geq", b"greater", b"leq", b"less"}
ANNOTATIONS = {b"guideline_interval": ((3, 4),), b"guideline_recurrence": ((2, 3),),
               b"guideline_recurrence_window": ((2, 3), (5, 6))}


@dataclasses.dataclass(frozen=True)
class Planned:
    goal: object
    source: int
    kind: str = "old"


# No target bytes inform these transformations. Referent identity is the base
# term; scoped variables have a distinct namespace per clause / NAF payload.
def plan_goals(goals, units, known=None, namespace=()):
    known = {} if known is None else known
    nouns = dict(known)
    nouns.update(entity_map(goals))
    converted, times = {}, {}
    for i, g in enumerate(goals):
        if is_term(g, b"guideline_cardinality", 5):
            q = g.args[1]
            noun = nouns.get(q)
            if noun and unquote(noun[0].name) in units:
                times[q] = Term(units[unquote(noun[0].name)])
            if g.args[3].name in CONVERTIBLE:
                converted[q] = tuple(Var((namespace, i, k)) for k in range(3))
    out = []
    for i, g in enumerate(goals):
        if isinstance(g, Naf):
            inner = plan_goals(g.goals, units, known, namespace + (i,))
            out.append(Planned(Naf(tuple(p.goal for p in inner)), i))
            continue
        if is_term(g, b"guideline_entity", 4) and g.args[1] in converted and g.args[1] in times:
            continue
        if is_term(g, b"guideline_cardinality", 5):
            ctx, q, unit, cmp, count = g.args
            cv = converted.get(q)
            if cv:
                s, n, v = cv
                out.append(Planned(compound(g.name, ctx, q, unit, s, n), i))
                if q in times:
                    out.append(Planned(compound(b"guideline_unit", ctx, q, v), i, "unit"))
                    built = compound(b"guideline_satisfies", s, n, v, cmp, count, times[q])
                else:
                    built = compound(b"guideline_satisfies", s, n, cmp, count)
                out.append(Planned(built, i, "builtin"))
            else:
                out.append(Planned(g, i))
                if q in times:
                    out.append(Planned(compound(b"guideline_unit", ctx, q, times[q]), i, "unit"))
            continue
        args = list(g.args)
        for qslot, uslot in ANNOTATIONS.get(unquote(g.name), ()):
            if len(args) > uslot and args[qslot] in converted and args[qslot] in times:
                args[uslot] = converted[args[qslot]][2]
        out.append(Planned(Term(g.name, tuple(args)), i))
    return tuple(out)


@dataclasses.dataclass
class ClauseChange:
    old: Clause
    new: Clause
    plan: tuple
    sentence: int
    docid: Term
    unit: Clause | None = None


@dataclasses.dataclass
class Projection:
    data: bytes
    original: bytes
    changes: list
    query_plan: tuple = ()
    query_term: Term | None = None

    def reverse(self):
        # Inverse uses source occurrence noun/class, not a unit→noun guess:
        # multiple table aliases may name the same unit.
        if self.query_term is not None:
            return self.original
        output = self.data.splitlines(keepends=True)
        changes = iter(self.changes)
        pending = next(changes, None)
        result, skip_unit = [], None
        for line in output:
            if skip_unit is not None:
                if line != skip_unit:
                    raise InputError("reverse unit clause position")
                skip_unit = None
                continue
            if pending is not None and line == render_clause(pending.new):
                old = reverse_clause(pending.new, pending.old)
                result.append(render_clause(old))
                skip_unit = render_clause(pending.unit) if pending.unit is not None else None
                pending = next(changes, None)
            else:
                result.append(line)
        if pending is not None or skip_unit is not None:
            raise InputError("reverse clause enumeration")
        return b"".join(result)


def reverse_clause(new, old):
    def reverse_goals(new_goals, old_goals):
        nouns = entity_map(old_goals)
        bounds = {}
        for g in new_goals:
            if is_term(g, b"guideline_satisfies") and len(g.args) in (4, 6):
                bounds[(g.args[0], g.args[1])] = (g.args[2:4] if len(g.args) == 4 else g.args[3:5])
        out = []
        oi = 0
        for g in new_goals:
            if is_term(g, b"guideline_unit") or is_term(g, b"guideline_satisfies"):
                continue
            if isinstance(g, Naf):
                while oi < len(old_goals) and not isinstance(old_goals[oi], Naf):
                    oi += 1
                if oi == len(old_goals):
                    raise InputError("inverse NAF scope")
                out.append(Naf(reverse_goals(g.goals, old_goals[oi].goals)))
                oi += 1
                continue
            if is_term(g, b"guideline_cardinality", 5) and (g.args[3], g.args[4]) in bounds:
                q = g.args[1]
                if q in nouns and not any(is_term(x, b"guideline_entity", 4) and x.args[1] == q for x in out):
                    out.append(compound(b"guideline_entity", g.args[0], q, *nouns[q]))
                cmp, count = bounds[(g.args[3], g.args[4])]
                g = compound(g.name, *g.args[:3], cmp, count)
            if unquote(g.name) in ANNOTATIONS:
                unit_slots = {u for _, u in ANNOTATIONS[unquote(g.name)]}
                candidates = [x for x in old_goals if isinstance(x, Term) and x.name == g.name and len(x.args) == len(g.args)
                              and all(a == b for j, (a, b) in enumerate(zip(x.args, g.args, strict=True)) if j not in unit_slots)]
                if len(candidates) == 1:
                    args = list(g.args)
                    for _, uslot in ANNOTATIONS[unquote(g.name)]:
                        args[uslot] = candidates[0].args[uslot]
                    g = Term(g.name, tuple(args))
            out.append(g)
        return tuple(out)
    result = Clause(new.head, reverse_goals(new.body, old.body))
    if render_clause(result) != render_clause(old):
        raise InputError("inverse clause differs from immutable base: " + first_difference(render_clause(old), render_clause(result)))
    return result


def semantic_document(data, units):
    lines = data.splitlines(keepends=True)
    parsed, known, sentence, docid = {}, {}, 0, Term(b"none")
    for i, line in enumerate(lines):
        if DOC_RECORD.fullmatch(line):
            docid = read_term(line.rstrip(b"\n").removesuffix(b".")).args[0]
        m = re.match(rb"% S([0-9]+):", line)
        if m:
            sentence = int(m[1])
        if sentence and line.startswith(b"guideline_"):
            c = parse_clause(line)
            parsed[i] = c, sentence
            if is_term(c.head, b"guideline_entity", 4) and not isinstance(c.head.args[1], Var):
                known[c.head.args[1]] = c.head.args[2:4]
    out, changes = [], []
    for i, line in enumerate(lines):
        if i not in parsed:
            out.append(line)
            continue
        c, sentence = parsed[i]
        plan = plan_goals(c.body, units, known, (i,))
        new = Clause(c.head, tuple(p.goal for p in plan))
        unit = None
        if is_term(c.head, b"guideline_cardinality", 5):
            q = c.head.args[1]
            noun = known.get(q)
            # A head may name an antecedent variable whose entity is in its body.
            noun = entity_map(c.body).get(q, noun)
            if noun and unquote(noun[0].name) in units:
                unit = Clause(compound(b"guideline_unit", c.head.args[0], q, Term(units[unquote(noun[0].name)])), new.body)
        rendered = render_clause(new)
        if render_clause(c) != line:
            raise InputError(f"compiler-output clause not canonical: {first_difference(render_clause(c), line)}")
        out.append(rendered)
        if unit:
            out.append(render_clause(unit))
        changes.append(ClauseChange(c, new, plan, sentence, docid, unit))
    result = Projection(document_transform(b"".join(out)), data, changes)
    if result.reverse() != document_transform(data):
        raise InputError("document reverse projection differs")
    return result


def semantic_query(data, units):
    out, result = [], None
    for line in data.splitlines(keepends=True):
        if not line.startswith(b"'$guideline_query_projection'("):
            out.append(line)
            continue
        term = read_term(line.rstrip(b"\n").removesuffix(b"."))
        goals = flatten(term.args[0].args[0])
        plan = (tuple(Planned(g, i) for i, g in enumerate(goals)) if units is None
                else plan_goals(goals, units, namespace=("query",)))
        new = compound(term.name, compound(b"goal", conjunction([p.goal for p in plan])), term.args[1])
        out.append(render(new, {}) + b".\n")
        result = Projection(b"", data, [], plan, new)
    if result is None:
        return Projection(data, data, [])
    result.data = b"".join(out)
    return result


def proof_transform(data, units):
    def facts(ts, witness):
        nouns = entity_map(ts)
        cards = {g.args[1]: g for g in ts if is_term(g, b"guideline_cardinality", 5)}
        out = []
        for g in ts:
            q = g.args[1] if len(g.args) > 1 else None
            noun = nouns.get(q)
            unit = units.get(unquote(noun[0].name)) if noun else None
            if witness and unit and is_term(g, b"guideline_entity", 4) and q in cards and cards[q].args[3].name in CONVERTIBLE:
                continue
            out.append(g)
            if unit and is_term(g, b"guideline_cardinality", 5):
                out.append(compound(b"guideline_unit", g.args[0], q, Term(unit)))
        return out
    out = []
    for line in data.splitlines(keepends=True):
        if not line.startswith(b"'$guideline_proof'("):
            out.append(line)
            continue
        t = read_term(line.rstrip(b"\n").removesuffix(b"."))
        args = list(t.args)
        for index, witness in ((3, True), (4, False)):
            args[index] = compound(args[index].name, list_term(facts(list_items(args[index].args[0]), witness)))
        out.append(render(Term(t.name, tuple(args))) + b".\n")
    return b"".join(out)


SCALES = {"second": ("time", 1), "minute": ("time", 60), "hour": ("time", 3600),
          "day": ("time", 86400), "week": ("time", 604800), "month": ("calendar", 1), "year": ("calendar", 12)}
STATED = ("eq", "exactly", "geq", "greater", "leq", "less")
BOUNDS = ("geq", "greater", "leq", "less", "exactly")


def within(s, x, b, y):
    if type(x) is not int or type(y) is not int:
        return False
    if b == "geq":
        return s in ("eq", "exactly", "geq", "greater") and x >= y
    if b == "greater":
        return (s in ("eq", "exactly", "geq") and x > y) or (s == "greater" and x >= y)
    if b == "leq":
        return s in ("exactly", "leq", "less") and x <= y
    if b == "less":
        return (s in ("exactly", "leq") and x < y) or (s == "less" and x <= y)
    return b == s == "exactly" and x == y


def satisfies(args):
    if len(args) == 4:
        s, n, b, m = args
        return type(s) is str and type(b) is str and within(s, n, b, m)
    if len(args) == 6:
        s, n, u, b, m, v = args
        if not all(type(x) is str for x in (s, u, b, v)) or type(n) is not int or type(m) is not int:
            return False
        if u not in SCALES or v not in SCALES or SCALES[u][0] != SCALES[v][0]:
            return False
        return within(s, n * SCALES[u][1], b, m * SCALES[v][1])
    return False


def builtin_holds(t):
    if not is_term(t, b"guideline_satisfies"):
        return False
    args = []
    for a in t.args:
        if not isinstance(a, Term) or a.args:
            return False
        args.append(int(a.name) if re.fullmatch(rb"-?[0-9]+", a.name) else unquote(a.name).decode())
    return satisfies(args)


def schema_bytes():
    # Spec printer spelling; relation is independently evaluated above.
    out = [b"% schema.pl: the comparison built-ins of the compiled Prolog schema (docs/REFERENCE.md); generated by ckc from the formal specification; do not edit; first-party code, Apache-2.0 WITH LLVM-exception.\n",
           b"guideline_satisfies(S, N, B, M) :- atom(S), integer(N), atom(B), integer(M), guideline_within(S, N, B, M).\n",
           b"guideline_satisfies(S, N, U, B, M, V) :- atom(S), integer(N), atom(U), atom(B), integer(M), atom(V), guideline_scale(U, G, R), guideline_scale(V, G, Q), X is N * R, Y is M * Q, guideline_within(S, X, B, Y).\n"]
    for b in BOUNDS:
        for s in STATED:
            outcomes = tuple(within(s, x, b, 0) for x in (-1, 0, 1))
            op = {(False, True, True): ">=", (False, False, True): ">", (True, True, False): "=<",
                  (True, False, False): "<", (False, True, False): "=:="}.get(outcomes)
            if op:
                out.append(f"guideline_within({s}, X, {b}, Y) :- X {op} Y.\n".encode())
    for u, (g, n) in SCALES.items():
        out.append(f"guideline_scale({u}, {g}, {n}).\n".encode())
    return b"".join(out)


def schema_structure(data):
    # Exact spec bytes imply the required guards, ISO-only helpers and census.
    return [] if data == schema_bytes() else ["O8 schema structure: " + first_difference(schema_bytes(), data)]


def schema_cases():
    cases = []
    for s in STATED:
        for b in BOUNDS:
            for y in (-10**100, -2, 0, 2, 10**100):
                for x in (y-1, y, y+1):
                    cases.append((s, x, b, y))
                    for u in SCALES:
                        for v in SCALES:
                            cases.append((s, x, u, b, y, v))
            for u in SCALES:
                for v in SCALES:
                    for n in (-10**100, -1, 0, 1, 10**100):
                        for delta in (-1, 0, 1):
                            cases.append((s, n*SCALES[v][1]+delta, u, b, n*SCALES[u][1], v))
    for args in (("exactly", 1, "geq", 1), ("exactly", 1, "day", "geq", 1, "day")):
        for i in range(len(args)):
            for bad in (None, "about", "na", "bogus", "eq", "day", "1", 0, ("compound",), 1.5, []):
                row = list(args)
                row[i] = bad
                cases.append(tuple(row))
    return cases


def schema_differential(path):
    version = subprocess.run(["swipl", "-q", "-f", "none", "-F", "none", "-g", "current_prolog_flag(version_data,V),write_canonical(V),nl,halt"], capture_output=True)
    if version.returncode or version.stdout != b"swi(9,2,9,[])\n" or version.stderr:
        raise InputError("O8 requires SWI-Prolog 9.2.9: " + version.stdout.decode(errors="replace").strip())
    def arg(x, i):
        if x is None:
            return f"V{i}"
        if isinstance(x, tuple):
            return "compound(x)"
        if isinstance(x, list):
            return "[]"
        if isinstance(x, str):
            return "'" + x.replace("\\", "\\\\").replace("'", "\\'") + "'"
        return str(x)
    lines = []
    cases = schema_cases()
    for i, args in enumerate(cases):
        goal = "guideline_satisfies(" + ",".join(arg(x, j) for j, x in enumerate(args)) + ")"
        lines.append(f"probe({i},{goal},{int(satisfies(args))}).\n")
    # Each successful solution must leave the exact incoming term unchanged;
    # equality is tested inside findall before its bindings are undone.
    driver = ("probe(I,G,N) :- copy_term(G,B), catch(findall(S,(call(G),(G==B->S=ok;S=bound)),L),E,L=error(E)), "
              "(N=0->Want=[];Want=[ok]), (L==Want->true;format('bad ~w ~q~n',[I,L])).\n"
              "run :- read_term(user_input,T,[]), (T==end_of_file->halt;call(T),run).\n")
    # Load only the graded schema. The neutral driver enters through -g, with
    # cases on stdin; no files are created beside the read-only target.
    setup = "assertz((" + driver.split("\n")[0].removesuffix(".") + ")),assertz((" + driver.split("\n")[1].removesuffix(".") + ")),run"
    proc = subprocess.run(["swipl", "-q", "-f", "none", "-F", "none", "-s", str(path), "-g", setup],
                          input="".join(lines).encode(), capture_output=True, timeout=120)
    if proc.returncode or proc.stderr or proc.stdout:
        raise InputError(f"O8 schema differential rc={proc.returncode}; " + (proc.stdout+proc.stderr)[:4000].decode(errors="replace"))
    return proc.returncode, len(cases)

def deref(t, env):
    while isinstance(t, Var) and t in env:
        t = env[t]
    return t


def substitute(t, env):
    t = deref(t, env)
    if isinstance(t, Var):
        return t
    if isinstance(t, Naf):
        return Naf(tuple(substitute(g, env) for g in t.goals))
    return Term(t.name, tuple(substitute(a, env) for a in t.args))


def unify(a, b, env):
    a, b = deref(a, env), deref(b, env)
    if a == b:
        return True
    if isinstance(a, Var):
        env[a] = b
        return True
    if isinstance(b, Var):
        env[b] = a
        return True
    return (unquote(a.name) == unquote(b.name) and len(a.args) == len(b.args) and
            all(unify(x, y, env) for x, y in zip(a.args, b.args, strict=True)))


class TraceTransport:
    def __init__(self, changes):
        self.by_hash, self.serial = {}, 0
        for change in changes:
            key = (change.docid, change.sentence, sha(render_clause(change.old)))
            if key in self.by_hash and self.by_hash[key].new != change.new:
                raise InputError("trace clause mapping ambiguous")
            self.by_hash[key] = change

    def clause_node(self, goal, old, env, unit=False):
        if not is_term(old, b"clause", 3):
            raise InputError("trace transport expected clause node")
        sent, digest, kids = old.args
        if not is_term(sent, b"sentence", 2) or not is_term(digest, b"clause_sha256", 1):
            raise InputError("trace node envelope")
        key = (sent.args[0], int(sent.args[1].name), unquote(digest.args[0].name))
        change = self.by_hash.get(key)
        if change is None:
            raise InputError(f"trace base clause missing {render(sent)!r} {key[2].decode()}")
        c = change.unit if unit else change.new
        if c is None:
            raise InputError(f"trace requires unit clause absent from source {render(sent)!r}")
        self.serial += 1
        serial = self.serial
        fresh = lambda v: Var((serial, v.name))
        head = map_vars(c.head, fresh)
        if not unify(goal, head, env):
            raise InputError(f"trace migrated derivation cannot unify {render(substitute(goal, env), {})!r} with {render(head, {})!r}")
        plan = [Planned(map_vars(p.goal, fresh), p.source, p.kind) for p in change.plan]
        children = self.goals(plan, list_items(kids), env)
        return compound(b"clause", sent, compound(b"clause_sha256", Term(atom(sha(render_clause(c))))), list_term(children))

    def goals(self, plan, old_nodes, env):
        result = []
        for p in plan:
            if p.source >= len(old_nodes):
                raise InputError("trace source node count")
            if p.kind == "builtin":
                result.append(compound(b"builtin", p.goal))
            elif isinstance(p.goal, Naf):
                if not is_term(old_nodes[p.source], b"naf", 1):
                    raise InputError("trace source NAF node missing")
                result.append(compound(b"naf", conjunction(list(p.goal.goals))))
            else:
                result.append(self.clause_node(p.goal, old_nodes[p.source], env, p.kind == "unit"))
        return result

    def transform(self, data, projection):
        if not projection.query_plan:
            return data
        numbering = [0]
        def freeze_nafs(t):
            if is_term(t, b"naf", 1):
                local = {}
                def number(v):
                    if v not in local:
                        local[v] = compound(b"'$VAR'", Term(str(numbering[0]).encode()))
                        numbering[0] += 1
                    return local[v]
                return compound(t.name, map_vars(t.args[0], number))
            return Term(t.name, tuple(freeze_nafs(a) for a in t.args)) if isinstance(t, Term) else t
        def walk(t, values=None):
            if is_term(t, b"sol", 2):
                return compound(t.name, t.args[0], walk(t.args[1], list_items(t.args[0])))
            if is_term(t, b"proved", 1):
                env = {}
                if values is not None:
                    answers = list_items(projection.query_term.args[1].args[0])
                    if len(values) != len(answers):
                        raise InputError("trace solution arity")
                    for answer, value in zip(answers, values, strict=True):
                        if not unify(answer.args[0], value, env):
                            raise InputError("trace solution substitution")
                result = compound(t.name, list_term(self.goals(projection.query_plan, list_items(t.args[0]), env)))
                result = freeze_nafs(substitute(result, env))
                if not ground(result):
                    raise InputError("transported trace nonground: " + repr(render(result, {})[:1200]))
                for leaf in terms(result):
                    if is_term(leaf, b"builtin", 1) and not builtin_holds(leaf.args[0]):
                        raise InputError("transported builtin does not hold")
                return result
            return Term(t.name, tuple(walk(a, values) for a in t.args))
        out = []
        for line in data.splitlines(keepends=True):
            if line.startswith(b"'$guideline_traces'("):
                out.append(render(walk(read_term(line.rstrip(b"\n").removesuffix(b".")))) + b".\n")
            else:
                out.append(line)
        return b"".join(out)

    def reverse(self, data, original, projection):
        def node(new, old, unit=False):
            if not is_term(new, b"clause", 3) or not is_term(old, b"clause", 3):
                raise InputError("trace reverse clause shape")
            sent, digest, kids = old.args
            key = (sent.args[0], int(sent.args[1].name), unquote(digest.args[0].name))
            change = self.by_hash.get(key)
            if change is None:
                raise InputError("trace reverse base clause absent")
            clause = change.unit if unit else change.new
            if clause is None or new.args[:2] != (sent, compound(b"clause_sha256", Term(atom(sha(render_clause(clause)))))):
                raise InputError("trace reverse clause identity")
            children = forest(list_items(new.args[2]), list_items(kids), change.plan, change.old.body)
            return compound(old.name, sent, digest, list_term(children))

        def forest(new, old, plan, old_goals):
            if len(new) != len(plan) or len(old) != len(old_goals):
                raise InputError("trace reverse goal/node count")
            restored = {}
            for t, p in zip(new, plan, strict=True):
                if p.kind == "builtin":
                    if not is_term(t, b"builtin", 1) or not builtin_holds(t.args[0]):
                        raise InputError("trace reverse builtin leaf")
                elif p.kind == "unit":
                    if node(t, old[p.source], True) != old[p.source]:
                        raise InputError("trace reverse unit derivation")
                elif isinstance(p.goal, Naf):
                    if not is_term(t, b"naf", 1) or not is_term(old[p.source], b"naf", 1):
                        raise InputError("trace reverse NAF shape")
                    # D3 also changes the opaque NAF payload and its numbervars
                    # counter. Its source occurrence restores that frozen leaf;
                    # forward byte equality checks the complete new payload.
                    restored[p.source] = old[p.source]
                else:
                    restored[p.source] = node(t, old[p.source])
            for i, g in enumerate(old_goals):
                if i not in restored:
                    if not is_term(g, b"guideline_entity", 4):
                        raise InputError("trace reverse lost non-entity goal")
                    restored[i] = old[i]
            return [restored[i] for i in range(len(old))]

        old_query = next(read_term(line.removesuffix(b".")) for line in projection.original.splitlines()
                         if line.startswith(b"'$guideline_query_projection'("))
        old_goals = flatten(old_query.args[0].args[0])
        def walk(new, old):
            if is_term(old, b"proved", 1):
                if not is_term(new, b"proved", 1):
                    raise InputError("trace reverse proof envelope")
                return compound(old.name, list_term(forest(list_items(new.args[0]), list_items(old.args[0]), projection.query_plan, old_goals)))
            if is_term(old, b"query_sha256", 1) or is_term(old, b"answers_sha256", 1):
                if new.name != old.name or len(new.args) != 1:
                    raise InputError("trace reverse custody envelope")
                return old
            if new.name != old.name or len(new.args) != len(old.args):
                raise InputError("trace reverse non-proof payload changed")
            return Term(old.name, tuple(walk(a,b) for a,b in zip(new.args, old.args, strict=True)))
        new_lines, old_lines = data.splitlines(keepends=True), original.splitlines(keepends=True)
        if len(new_lines) != len(old_lines):
            raise InputError("trace reverse line count")
        out = []
        for new, old in zip(new_lines, old_lines, strict=True):
            if old.startswith(b"'$guideline_traces'("):
                out.append(render(walk(read_term(new.rstrip(b"\n").removesuffix(b".")), read_term(old.rstrip(b"\n").removesuffix(b".")))) + b".\n")
            elif new == old:
                out.append(old)
            else:
                raise InputError("trace reverse header changed")
        result = b"".join(out)
        if result != original:
            raise InputError("trace reverse differs from immutable base")
        return result


def terms(t):
    yield t
    if isinstance(t, Term):
        for a in t.args:
            yield from terms(a)


def parse_tsv(data):
    return [line.split("\t") for line in data.decode().splitlines() if line and not line.startswith("#")]


FROZEN = ("tests/s1-compiler/", "tests/s1-codec/", "tests/s2-compiler/", "tests/s2-codec/", "tests/nt1/", "tests/nt1-surface/")


def semantic_path(path, data):
    if re.fullmatch(r"guidelines/[^/]+/pl/[^/]+\.pl", path):
        return "doc"
    if re.fullmatch(r"guidelines/[^/]+/queries/pl/[^/]+\.pl", path):
        return "query"
    if path.startswith("tests/green/") and path.endswith(".expect"):
        if path.endswith(".proof.expect"):
            return "proof"
        return artifact_kind(data)
    if path.startswith("tests/certify-one/"):
        if path.endswith("pl.txt.in"):
            return artifact_kind(data)
        if path.endswith("stdout.expect") and b"'$guideline_proof'(" in data:
            return "proof"
    if path.startswith("tests/queries/green/v2-annotation/") and path.endswith(".pl"):
        if "/queries/pl/" in path:
            return "query"
        if re.search(r"/tree/guidelines/[^/]+/pl/[^/]+\.pl$", path):
            return "doc"
    return None


def family(path, data=b""):
    if path == "guidelines/schema.pl":
        return "O8"
    if path.startswith(FROZEN):
        return "O1"
    if path == KANI or path.endswith(".tpl.in"):
        return "O2"
    if path.endswith("/audit/review-manifest.tsv"):
        return "O9"
    kind = artifact_kind(data)
    if kind == "answers" and not path.startswith("tests/v1/"):
        return "O6"
    if kind == "traces" and not path.startswith("tests/v1/"):
        return "O7"
    if semantic_path(path, data) == "query":
        return "O5"
    if re.fullmatch(r"guidelines/[^/]+/pl/[^/]+\.pl", path):
        return "O3"
    if path.startswith(("tests/green/", "tests/red/", "tests/certify-one/")) or semantic_path(path, data) == "doc":
        return "O4"
    if path.startswith("tests/"):
        return "O10"
    return "O1"


class Oracle:
    def __init__(self, base, dispositions=None, rulings=()):
        self.base = base
        self.rulings = list(rulings)
        self.expected, self.projections, self.hashes = {}, {}, {}
        self.ambiguous = set()
        self.frozen = {p for p in base if p.startswith(FROZEN)}
        self.offsets, self.term_counts, self.commit_ids = set(), set(), {}
        self.counts = collections.Counter()
        self.rule_paths = set()

    def table(self, path):
        candidates = []
        if path.startswith("tests/green/"):
            stem = path.removesuffix(".proof.expect").removesuffix(".expect")
            candidates = [stem + ".vocabulary.tsv"]
        elif path.startswith("tests/certify-one/"):
            candidates = [str(PurePosixPath(path).parent / "vocabulary.tsv.in")]
        elif match := re.search(r"(^|/)guidelines/[^/]+/", path):
            candidates = [path[:match.end()] + "vocabulary.tsv"]
        for p in candidates:
            if p in self.base:
                return unit_table(self.base[p].data)
        return {}

    def initial_transform(self, path, blob):
        if path in self.frozen or blob.mode == "120000":
            return blob.data
        data = blob.data
        if path == KANI:
            data, pairs = kani_transform(data)
            for old, new in pairs:
                self.remember_hash(old, new)
            return data
        kind = semantic_path(path, data)
        if kind is None and path.startswith("tests/queries/") and "/queries/pl/" in path and artifact_kind(data) == "query":
            # These fixture inputs are hand-shaped, not regenerated compiler
            # output (O4 ruling). Identity transport still checks their traces.
            projection = semantic_query(data, None)
            if projection.data != data:
                raise InputError("hand-shaped question re-render differs")
            self.projections[path] = projection
            return data
        if kind == "proof":
            return proof_transform(data, self.table(path))
        if kind in {"doc", "query"}:
            projection = (semantic_document if kind == "doc" else semantic_query)(data, self.table(path))
            self.projections[path] = projection
            return projection.data
        if path.endswith((".html", ".golden", "assertions.tsv")) or table_file(path) or data.startswith(TABLE_HEADER):
            return data
        data = document_transform(data)
        return template_pin(data) if path.endswith(".tpl.in") else data

    def remember_hash(self, old, new):
        if old != new:
            a, b = sha(old), sha(new)
            if a in self.ambiguous:
                return
            if a in self.hashes and self.hashes[a] != b:
                self.ambiguous.add(a)
                del self.hashes[a]
            else:
                self.hashes[a] = b

    def update(self, path, data):
        blob = self.expected[path]
        if blob.data != data:
            self.expected[path] = Blob(data, blob.mode)
            self.hashes[sha(blob.data)] = sha(data)
            if path in self.base and sha(self.base[path].data) not in self.ambiguous:
                self.hashes[sha(self.base[path].data)] = sha(data)

    def digest_closure(self):
        for _ in range(len(self.expected) + 1):
            pending = {}
            for path, blob in self.expected.items():
                if path in self.frozen:
                    continue
                kind = artifact_kind(blob.data)
                if kind in {"answers", "traces"}:
                    # Custody wrappers alone: no arbitrary solution value or
                    # clause digest is rewritten by a file-hash coincidence.
                    data = re.sub(rb"((?:query|answers)_sha256\()('?)([0-9a-f]{64})\2(\))",
                                  lambda m: m[1] + atom(self.hashes[m[3]]) + m[4] if m[3] in self.hashes else m.group(), blob.data)
                elif path.startswith("tests/ui/") and path.endswith((".html", ".golden")):
                    data = re.sub(rb'(<input (?=[^>]*type="hidden")(?=[^>]*name="(?:ledger|review)_sha256")[^>]*value=")([0-9a-f]{64})(")',
                                  lambda m: m[1] + self.hashes.get(m[2], m[2]) + m[3], blob.data)
                else:
                    data = blob.data if protected_payload(path, blob.data) else rebind_bytes(blob.data, self.hashes)
                if path.endswith(".tpl.in"):
                    data = template_pin(data)
                if data != blob.data:
                    pending[path] = data
            if not pending:
                return
            for path, data in pending.items():
                before = self.expected[path].data
                self.update(path, data)
                if path == KANI:
                    for old, new in zip(embedded_artifacts(before), embedded_artifacts(data), strict=True):
                        self.hashes[sha(old)] = sha(new)
                if path.endswith(".tpl.in"):
                    self.hashes[sha(expand_template(before))] = sha(expand_template(data))
                    self.hashes[sha(expand_template(self.base[path].data))] = sha(expand_template(data))
        raise InputError("digest dependency cycle")

    def review_manifests(self):
        for path, blob in self.base.items():
            if path in self.frozen or not path.endswith("/audit/review-manifest.tsv"):
                continue
            prefix = path.removesuffix("audit/review-manifest.tsv")
            out = []
            for line in blob.data.splitlines(keepends=True):
                fields = line.rstrip(b"\n").split(b"\t")
                if len(fields) == 6 and not line.startswith(b"#"):
                    document = prefix + "pl/" + fields[0].decode() + ".pl"
                    if document in self.base:
                        old_clauses = sha(semantic_stream(self.base[document].data))
                        new_clauses = sha(semantic_stream(self.expected[document].data))
                        old_bundle = sha(bundle_bytes(fields))
                        # Malformed/stale red rows stay stale. Valid rows are
                        # recomputed independently; never a blind digest swap.
                        if fields[4] == old_clauses:
                            fields[4] = new_clauses
                            if fields[5] == old_bundle:
                                fields[5] = sha(bundle_bytes(fields))
                            self.hashes[old_clauses] = new_clauses
                            self.hashes[old_bundle] = sha(bundle_bytes(fields))
                            line = b"\t".join(fields) + (b"\n" if line.endswith(b"\n") else b"")
                out.append(line)
            self.update(path, b"".join(out))

    def corpus_changes(self, query_path):
        prefix = query_path.split("/queries/pl/", 1)[0] + "/pl/"
        result = []
        for path, blob in self.base.items():
            if path.startswith(prefix) and path.endswith(".pl"):
                if path in self.projections:
                    result.extend(self.projections[path].changes)
                else:
                    # Hand-authored query fixtures keep their clause stream.
                    d = semantic_document(blob.data, {})
                    for c in d.changes:
                        c.new, c.plan, c.unit = c.old, tuple(Planned(g, i) for i, g in enumerate(c.old.body)), None
                    result.extend(d.changes)
        return result

    def traces(self):
        for qp, projection in self.projections.items():
            if projection.query_term is None or "/queries/pl/" not in qp:
                continue
            tp = qp.replace("/queries/pl/", "/queries/traces/")
            if tp not in self.base:
                continue
            transport = TraceTransport(self.corpus_changes(qp))
            old = self.base[tp].data
            try:
                new = transport.transform(old, projection)
            except InputError as error:
                raise InputError(f"{tp}: {error}") from error
            if transport.reverse(new, old, projection) != old:
                raise InputError(f"{tp}: trace reverse differs")
            self.update(tp, new)
            # Query fixture goldens carry the very same trace bytes.
            if qp.startswith("tests/queries/"):
                case = "/".join(qp.split("/")[:4])
                golden = case + "/traces-golden/" + PurePosixPath(tp).name
                if golden in self.base and self.base[golden].data == old:
                    self.update(golden, new)

    def trace_problems(self, actual):
        problems, cache = [], {}
        for tp, blob in sorted(actual.items()):
            if family(tp, self.expected.get(tp, blob).data) != "O7":
                continue
            if "/queries/traces/" in tp:
                prefix = tp.split("/queries/traces/", 1)[0] + "/pl/"
                pattern = re.escape(prefix) + r"[^/]+\.pl$"
            elif "/traces-golden/" in tp:
                prefix = tp.split("/traces-golden/", 1)[0] + "/tree/guidelines/"
                pattern = re.escape(prefix) + r"[^/]+/pl/[^/]+\.pl$"
            else:
                problems.append(f"{tp}: O7 composition path absent")
                continue
            if pattern not in cache:
                hashes = set()
                for path, source in actual.items():
                    if not re.fullmatch(pattern, path):
                        continue
                    sentence, docid = 0, None
                    for line in source.data.splitlines(keepends=True):
                        if DOC_RECORD.fullmatch(line):
                            docid = read_term(line.rstrip(b"\n").removesuffix(b".")).args[0]
                        if m := re.match(rb"% S([0-9]+):", line):
                            sentence = int(m[1])
                        if sentence and line.startswith(b"guideline_"):
                            hashes.add((docid, sentence, sha(render_clause(parse_clause(line)))))
                cache[pattern] = hashes
            try:
                records = 0
                for line in blob.data.splitlines():
                    if not line.startswith(b"'$guideline_traces'("):
                        continue
                    records += 1
                    for t in terms(read_term(line.removesuffix(b"."))):
                        if is_term(t, b"builtin", 1) and not builtin_holds(t.args[0]):
                            problems.append(f"{tp}: O7 builtin false or ill-typed")
                        if is_term(t, b"clause", 3):
                            sent, digest, _ = t.args
                            key = (sent.args[0], int(sent.args[1].name), unquote(digest.args[0].name))
                            if key not in cache[pattern]:
                                problems.append(f"{tp}: O7 clause digest absent from migrated composition {key[2].decode()}")
                if records != 1:
                    problems.append(f"{tp}: O7 trace record count {records}")
            except (InputError, ValueError, IndexError) as error:
                problems.append(f"{tp}: O7 malformed trace {error}")
        return problems

    def document_answers_diagnostic(self, row: list[str]):
        case, probe, mode, _, _, arguments = row
        args = arguments.split()
        if mode not in {"trace", "trace-check"} or len(args) < 3:
            return
        source = f"tests/v1/{case}/{args[2]}.in"
        if source not in self.base or source not in self.expected:
            return
        old, new = self.base[source].data, self.expected[source].data
        if artifact_kind(old) != "doc":
            return
        old_terms = sum(line.endswith(b".") and not line.startswith(b"%") for line in old.split(b"\n"))
        new_terms = sum(line.endswith(b".") and not line.startswith(b"%") for line in new.split(b"\n"))
        if old_terms == new_terms:
            return
        for stream in ("stdout", "stderr"):
            path = f"tests/v1/{case}/expect/{probe}.{stream}"
            if path in self.expected:
                blob = self.expected[path]
                data = re.sub(rb"answers_file\(term_count\(([0-9]+)\)\)",
                              lambda m: f"answers_file(term_count({new_terms}))".encode() if int(m[1]) == old_terms else m.group(), blob.data)
                if data != blob.data:
                    self.expected[path] = Blob(data, blob.mode)
                    self.term_counts.add(path)

    def migrated_line(self, path, index):
        data = self.base[path].data
        lines = data.splitlines(keepends=True)
        if not 0 <= index < len(lines):
            raise InputError(f"{path}: swap operand outside base line domain: {index}")
        shift = header_line_shift(data, index + 1)
        projection = self.projections.get(path)
        if projection is not None and projection.query_term is None:
            changes = iter(projection.changes)
            change = next(changes, None)
            for line in lines[:index]:
                if change is not None and line == render_clause(change.old):
                    shift += int(change.unit is not None)
                    change = next(changes, None)
        return index + shift

    def reanchor_document(self, path, needle, replacement):
        old, new = self.base[path].data, self.expected[path].data
        at = old.find(needle)
        if at < 0:
            raise InputError(f"{path}: mutation anchor absent from base")
        def head(data, at):
            start = data.rfind(b"\n", 0, at) + 1
            end = data.find(b"\n", start)
            line = data[start:len(data) if end < 0 else end]
            return render(parse_clause(line).head, {}) if line.startswith(b"guideline_") else line
        original_head = head(old, at)
        first = new.find(needle)
        if first >= 0 and head(new, first) == original_head:
            return needle, replacement
        line = self.migrated_line(path, old.count(b"\n", 0, at))
        lines = new.splitlines(keepends=True)
        start = sum(map(len, lines[:line]))
        end = start + len(lines[line])
        at = new.find(needle, start)
        if at < start or at >= end:
            raise InputError(f"{path}: mutation anchor absent from its projected source clause")
        # The least left extension preserves the old occurrence's clause head;
        # extending the replacement by the same bytes keeps the mutation itself.
        for begin in range(at, start - 1, -1):
            prefix = new[begin:at]
            anchor = prefix + needle
            first = new.find(anchor)
            if first >= 0 and head(new, first) == original_head:
                return anchor, prefix + replacement
        raise InputError(f"{path}: mutation anchor cannot retain its source head")

    def reanchor_query(self, path, needle, replacement):
        projection = self.projections.get(path)
        if projection is None or projection.query_term is None:
            raise InputError(f"{path}: query mutation projection absent")
        old_projection = next(read_term(line.removesuffix(b".")) for line in projection.original.splitlines()
                              if line.startswith(b"'$guideline_query_projection'("))
        goals = flatten(old_projection.args[0].args[0])
        def projected(t):
            if t in goals:
                source = goals.index(t)
                matches = [p.goal for p in projection.query_plan if p.source == source and p.kind == "old"]
                if len(matches) != 1:
                    raise InputError(f"{path}: query edit has no retained source goal")
                return matches[0]
            if is_term(t, b",", 2):
                children = [projected(a) for a in t.args]
                if all(c is not None for c in children):
                    return Term(t.name, tuple(children))
            return None
        old, changed = read_term(needle), read_term(replacement)
        new = projected(old)
        if new is None:
            raise InputError(f"{path}: query anchor is not a projected source fragment")
        def edit(a, b, target):
            if a == b:
                return target
            selected = projected(b)
            if selected is not None:
                return selected
            if all(isinstance(x, Term) for x in (a,b,target)) and a.name == b.name == target.name and len(a.args) == len(b.args) == len(target.args):
                return Term(target.name, tuple(edit(x,y,z) for x,y,z in zip(a.args,b.args,target.args,strict=True)))
            return b
        variables = {}
        render(projection.query_term, variables)
        count = len(variables)
        result = render(new, variables), render(edit(old, changed, new), variables)
        if len(variables) != count:
            raise InputError(f"{path}: mutation introduced a variable outside the projection")
        return result

    def diagnostic_transform(self):
        for row in parse_tsv(self.base.get("tests/v1/cases.tsv", Blob(b"")).data):
            if len(row) == 6:
                self.document_answers_diagnostic(row)
            if len(row) != 6 or row[2] not in {"check", "render"} or not row[5]:
                continue
            case, probe, _, _, _, args = row
            source = f"tests/v1/{case}/{args.split()[0]}.in"
            if source not in self.base:
                source = f"tests/v1/{case}/{args.split()[0]}.tpl.in"
            if source not in self.base:
                continue
            old = self.base[source].data
            if source.endswith(".tpl.in"):
                old = expand_template(old)
            if not insertion_points(old):
                continue
            for stream in ("stdout", "stderr"):
                path = f"tests/v1/{case}/expect/{probe}.{stream}"
                if path not in self.expected:
                    continue
                blob = self.expected[path]
                data = OFFSET.sub(lambda m: b"noncanonical(%d,%d)" % (int(m[1]) + header_line_shift(old, int(m[1])), int(m[2])), blob.data)
                if data != blob.data:
                    self.expected[path] = Blob(data, blob.mode)
                    self.offsets.add(path)
        path = "tests/certify/cases.tsv"
        if path in self.base:
            output = []
            for line in self.base[path].data.splitlines(keepends=True):
                fields = line.removesuffix(b"\n").split(b"\t")
                if len(fields) == 8 and fields[3] in {b"swap-lines", b"replace-first"}:
                    target = fields[2].decode()
                    if target in self.base:
                        if fields[3] == b"swap-lines":
                            for i in (4, 5):
                                fields[i] = str(self.migrated_line(target, int(fields[i]))).encode()
                        elif fields[1] == b"doc":
                            before = literal_bytes(fields[4].decode())
                            after = literal_bytes(fields[5].decode())
                            before, after = self.reanchor_document(target, before, after)
                            fields[4:6] = [escaped_bytes(x).encode() for x in (before,after)]
                        elif fields[0] in {b"query-v2-annotation-altered", b"query-v2-annotation-dropped"}:
                            before, after = self.reanchor_query(target, fields[4], fields[5])
                            fields[4:6] = [before, after]
                        line = b"\t".join(fields) + (b"\n" if line.endswith(b"\n") else b"")
                output.append(line)
            self.expected[path] = Blob(b"".join(output), self.base[path].mode)

    def ui_transform(self):
        changes = collections.defaultdict(list)
        for path, original in self.base.items():
            if path.startswith("tests/ui/") and path.endswith(".pl") and ("/tree/" in path or "/worktree/" in path):
                if path in self.expected and self.expected[path].data != original.data:
                    case = "/".join(path.split("/")[:4]) + "/"
                    changes[case].append((ui_escape(original.data), ui_escape(self.expected[path].data)))
        for path, blob in list(self.expected.items()):
            if not path.startswith("tests/ui/") or not path.endswith((".html", ".golden", "assertions.tsv")):
                continue
            case = "/".join(path.split("/")[:4]) + "/"

            def transform(data, context):
                for old, new in changes.get(case, []):
                    if old in context:
                        data = data.replace(old, new)
                        old_caption = f"<summary>Compiled Prolog ({len(old.splitlines())} lines)</summary>".encode()
                        new_caption = f"<summary>Compiled Prolog ({len(new.splitlines())} lines)</summary>".encode()
                        data = data.replace(old_caption, new_caption)
                return data

            if path.endswith("assertions.tsv"):
                output = []
                for line in blob.data.splitlines(keepends=True):
                    fields = line.removesuffix(b"\n").split(b"\t")
                    if len(fields) == 3 and fields[1] in {b"contains", b"absent"} and re.fullmatch(rb"(?:[0-9a-f]{2})*", fields[2]):
                        fragment = bytes.fromhex(fields[2].decode())
                        context = self.base.get(case + "golden/" + fields[0].decode(), Blob(fragment)).data
                        fields[2] = transform(fragment, context).hex().encode()
                        line = b"\t".join(fields) + (b"\n" if line.endswith(b"\n") else b"")
                    output.append(line)
                data = b"".join(output)
            else:
                data = transform(blob.data, blob.data)
            self.update(path, data)

    def fixture_commits(self):
        cases = {"/".join(path.split("/")[:4]) for path in self.base
                 if path.startswith("tests/ui/") and "/worktree/" in path}
        for case in sorted(cases):
            prefix = case + "/tree/"
            old_files = {p[len(prefix):]: b for p, b in self.base.items() if p.startswith(prefix)}
            new_files = {p[len(prefix):]: b for p, b in self.expected.items() if p.startswith(prefix)}
            old_id, new_id = fixture_commit(old_files), fixture_commit(new_files)
            previous = self.commit_ids.get(case, old_id)
            self.commit_ids[case] = new_id
            if previous == new_id:
                continue
            ids = {old_id, previous}
            for path, blob in list(self.expected.items()):
                if path.startswith(case + "/"):
                    data = re.sub(rb"(?<![0-9a-f])[0-9a-f]{40}(?![0-9a-f])",
                                  lambda m: new_id if m.group() in ids else m.group(), blob.data)
                    self.update(path, data)




    def apply_rulings(self):
        for row in self.rulings:
            path = row["path"]
            if row["mode"] == "law":
                laws = {("O7", "trace-transport", "guidelines"),
                        ("O4", "faithful-query-fixture", "tests/queries/green/v2-annotation"),
                        ("O2", "certify-source-occurrences", "tests/certify/cases.tsv"),
                        ("O2", "query-annotation-edits", "tests/certify/cases.tsv")}
                if (row["family"], row["item"], path) not in laws:
                    raise InputError("unrecognized algorithm ruling")
                continue
            if path in self.frozen or path not in self.base or row["mode"] not in {"edit", "whole"}:
                raise InputError(f"ruling {path}: only existing unfrozen paths may change")
            if row.get("base_sha256") != sha(self.base[path].data).decode():
                raise InputError(f"ruling {path}: immutable base digest required")
            if not row["reason"]:
                raise InputError(f"ruling {path}: reason required")
            data = self.expected[path].data
            for edit in row["edits"]:
                if row["mode"] == "whole":
                    data = edit["new"]
                elif data.count(edit["old"]) != edit["count"]:
                    raise InputError(f"ruling {path}: anchor count differs")
                else:
                    data = data.replace(edit["old"], edit["new"])
            self.update(path, data)
            self.rule_paths.add(path)

    def build(self):
        for path, blob in sorted(self.base.items()):
            try:
                data = self.initial_transform(path, blob)
            except InputError as error:
                raise InputError(f"{path}: {error}") from error
            self.expected[path] = Blob(data, blob.mode)
            if path not in self.frozen:
                self.remember_hash(blob.data, data)
            if path.endswith(".tpl.in"):
                old = expand_template(blob.data)
                if blob.data.split(b"\n", 1)[0] != b"@@sha256 " + sha(old):
                    raise InputError(f"{path}: base template pin differs")
                self.remember_hash(old, expand_template(data))
            self.counts[family(path, blob.data)] += 1
        if "guidelines/schema.pl" in self.base:
            raise InputError("base already holds guidelines/schema.pl")
        self.expected["guidelines/schema.pl"] = Blob(schema_bytes())
        self.counts["O8"] += 1
        self.diagnostic_transform()
        self.review_manifests()
        self.traces()
        self.digest_closure()
        self.ui_transform()
        for _ in range(len(self.expected) + 1):
            before = dict(self.expected)
            self.fixture_commits()
            self.digest_closure()
            if self.expected == before:
                break
        else:
            raise InputError("fixture-commit custody cycle")
        self.apply_rulings()
        return self

    def compare(self, actual):
        problems = []
        for path in sorted(self.expected.keys() | actual.keys()):
            label = family(path, self.expected.get(path, actual.get(path, Blob(b""))).data)
            if path not in self.expected:
                why, label = "unlisted file", "O1"
            elif path not in actual:
                why = "omitted file/case"
            elif self.expected[path].mode != actual[path].mode:
                why = f"mode {actual[path].mode} != predicted {self.expected[path].mode}"
            elif self.expected[path].data != actual[path].data:
                why = first_difference(self.expected[path].data, actual[path].data)
            else:
                continue
            problems.append(f"{path}: {label} {why}")
        return problems


def bundle_bytes(fields):
    return (b"bundle v2 " + fields[0] + b"\nace " + fields[1] + b"\ncoverage " + fields[2] +
            b"\npayload " + fields[3] + b"\nclauses " + fields[4] + b"\n")


def synthetic_oracle():
    d = (b"% d.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.\n" + BASE_DECLS +
         b"guideline_schema_version(1).\nguideline_document(d,ace_sha256(" + b"a"*64 +
         b"),ulex(none),vocabulary(none)).\n% S1: Four hours.\nguideline_entity(actual,q,hour,countable).\n"
         b"guideline_cardinality(actual,q,na,geq,4).\n")
    q = (b"% q compiled from ACE question by ace_to_pl question mode; do not edit.\n"
         b"'$guideline_query'(v1,q,ace_sha256(" + b"a"*64 + b"),ulex(none),vocabulary(none)).\n% Q1: Four hours?\n"
         b"'$guideline_query_projection'(goal(','(guideline_entity(actual,A,hour,countable),guideline_cardinality(actual,A,na,geq,4))),answers([])).\n")
    a = b"'$guideline_answers'(v1,q,query_sha256(" + atom(sha(q)) + b"),result(yes)).\n"
    nodes = b",".join(b"clause(sentence(d,1),clause_sha256(" + atom(sha(line + b"\n")) + b"),[])" for line in d.splitlines()[-2:])
    t = b"'$guideline_traces'(v1,q,query_sha256(" + atom(sha(q)) + b"),answers_sha256(" + atom(sha(a)) + b"),result(yes(proved([" + nodes + b"])))).\n"
    f = [b"d", b"a"*64, b"b"*64, b"c"*64, sha(semantic_stream(d))]
    f.append(sha(bundle_bytes(f)))
    base = {"guidelines/g/vocabulary.tsv": Blob(TABLE_HEADER + b"unit\thour\thour\n"), "guidelines/g/pl/d.pl": Blob(d),
            "guidelines/g/queries/pl/q.pl": Blob(q), "guidelines/g/queries/answers/q.pl": Blob(a),
            "guidelines/g/queries/traces/q.pl": Blob(t), "guidelines/g/audit/review-manifest.tsv": Blob(b"\t".join(f)+b"\n"),
            "tests/green/d.expect": Blob(d), "tests/red/r.expect": Blob(b"ace_to_pl_error(unsupported,x).\n"),
            "tests/v1/cases.tsv": Blob(b"x\tp\tcheck\t2\t1\tin.pl\n"), "tests/v1/x/in.pl.in": Blob(d),
            "tests/v1/x/expect/p.stderr": Blob(b"ace_to_pl_error(check_load,noncanonical(40,1)).\n"),
            "tests/s1-compiler/frozen": Blob(b"frozen\n"), "tests/check-mutants/cases.tsv": Blob(b"x\tcheck\t1\n")}
    return Oracle(base).build()


def self_test_cases(oracle):
    cases = []
    for label in (f"O{i}" for i in range(1, 11)):
        matches = [(p, b) for p, b in sorted(oracle.expected.items()) if family(p, b.data) == label]
        if label == "O2" and not matches:
            matches = [(p, b) for p, b in sorted(oracle.expected.items()) if p in oracle.offsets]
        if label in {"O3", "O4"}:
            matches = [(p,b) for p,b in matches if artifact_kind(b.data) == "doc"]
        if label in {"O5", "O7"}:
            matches.sort(key=lambda item: b"guideline_satisfies(" not in item[1].data)
        if label == "O10":
            matches = [(p,b) for p,b in matches if b.data.startswith(b"ace_to_pl_error(")]
        if not matches:
            raise AssertionError(f"{label} plant anchor missing")
        p, blob = matches[0]
        if label == "O2":
            if b"guideline_unit/3" in blob.data:
                changed = blob.data.replace(b"guideline_unit/3", b"guideline_unit/4", 1)
            else:
                changed = OFFSET.sub(lambda m: b"noncanonical(%d,%d)" % (int(m[1])+1,int(m[2])),blob.data,count=1)
        elif label in {"O3", "O4"}:
            changed = blob.data.replace(b"guideline_cardinality(", b"guideline_wrong(", 1)
        elif label in {"O5", "O7"}:
            changed = blob.data.replace(b"guideline_satisfies(", b"guideline_satisfies_wrong(", 1)
        elif label == "O8":
            changed = blob.data.replace(b"atom(S), ", b"", 1)
        elif label in {"O6", "O9"}:
            changed = HEX64.sub(lambda m: (b"1" if m[0][:1] == b"0" else b"0") + m[0][1:],blob.data,count=1)
        elif label == "O10":
            changed = b""
        else:
            changed = blob.data + b"nt1-oracle-unpredicted\n"
        if changed == blob.data:
            raise AssertionError(f"{label} mutation anchor missing")
        cases.append((label + ":changed", p, Blob(changed, blob.mode)))
    p = "tests/nt1-oracle-unlisted"
    if p in oracle.expected:
        raise AssertionError("unlisted plant collides")
    cases.append(("O1:added", p, Blob(b"plant\n")))
    p = next(iter(sorted(oracle.expected)))
    cases.append(("O1:removed", p, None))
    blob = oracle.expected[p]
    cases.append(("O1:mode", p, Blob(blob.data, "100755" if blob.mode != "100755" else "100644")))
    if oracle.frozen:
        p = sorted(oracle.frozen)[0]
        blob = oracle.expected[p]
        cases.append(("O1:frozen", p, Blob(blob.data + b"drift\n", blob.mode)))
    return cases


def self_test(oracle):
    if comparison_result(oracle.compare(oracle.expected))[0] != 0:
        raise AssertionError("clean predicted tree rejected")
    evidence = []
    for name, path, blob in self_test_cases(oracle):
        actual = dict(oracle.expected)
        if blob is None:
            del actual[path]
        else:
            actual[path] = blob
        problems = oracle.compare(actual)
        rc, lines = comparison_result(problems)
        if rc != 1 or not any(p.startswith(path + ":") for p in problems):
            raise AssertionError(f"{name} planted unpredicted change accepted")
        evidence.append((name, f"rc={rc} {lines[0]}"))
    return evidence


def comparison_result(problems):
    return (1 if problems else 0, ["nt1-churn: unpredicted " + p.replace("\n", "\\n").replace("\r", "\\r") for p in problems])


def main(argv):
    if len(argv) != 3:
        print("nt1-churn: usage: nt1-churn.py <base-commit> <repo-root>")
        return 1
    try:
        root = Path(argv[2]).resolve(strict=True)
        oracle = Oracle(base_blobs(root, argv[1]), rulings=rulings_for(root)).build()
        try:
            tests = self_test(oracle)
        except Exception as error:
            print("nt1-churn: self-test failed: " + str(error).replace("\n", "\\n"))
            return 2
        actual, _ = working_blobs(root)
        problems = oracle.compare(actual)
        # A byte mismatch is decisive. Matching trees additionally validate all
        # trace joins and builtin truth, not just predicted trace bytes.
        if not problems:
            problems.extend(oracle.trace_problems(actual))
        grid = 0
        if actual.get("guidelines/schema.pl") == oracle.expected["guidelines/schema.pl"]:
            _, grid = schema_differential(root / "guidelines/schema.pl")
        rc, lines = comparison_result(problems)
        if rc:
            for line in lines:
                print(line)
            print(f"nt1-churn: {len(problems)} unpredicted changes; self-test {len(tests)}/{len(tests)}; schema-grid {grid}")
            return rc
        changed = sum(oracle.base.get(p) != b for p, b in actual.items()) + len(oracle.base.keys() - actual.keys())
        counts = ",".join(f"{k}={v}" for k, v in sorted(oracle.counts.items()))
        print(f"nt1-churn: ok {len(actual)} artifacts; {changed} changed paths; families {counts}; self-test {len(tests)}/{len(tests)}; schema-grid {grid}; 0 unpredicted changes")
        return 0
    except (InputError, OSError, UnicodeError, ValueError, subprocess.TimeoutExpired) as error:
        print("nt1-churn: " + str(error).replace("\n", "\\n").replace("\r", "\\r"))
        return 1


if __name__ == "__main__":
    sys.exit(main(sys.argv))
