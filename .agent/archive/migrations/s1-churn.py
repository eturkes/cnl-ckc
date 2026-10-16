#!/usr/bin/env python3
"""S1 D10 independent migration oracle; Python 3.13 stdlib, no compiler calls.

Usage: python3 -I s1-churn.py <base-commit> <repo-root>
Exit 0 = predicted tree, 1 = unpredicted changes/input errors, 2 = broken self-test.

The base's tracked fixture/corpus blobs are the inputs, not regenerated pins.
Normal review owns production source/docs and generator/harness code. The Kani
fixture carrier remains in scope; release-manifest is outside this oracle.
Rulings: .agent/archive/migrations/s1-churn-rulings.tsv (see load_rulings).
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

DISPOSITIONS = ".agent/archive/migrations/s1-dispositions.tsv"
RULINGS = ".agent/archive/migrations/s1-churn-rulings.tsv"
TABLE_HEADER = b"# format: kind\tlemma\tvalue\n"
INDICATORS = (
    ("schema_version", 1), ("document", 4), ("entity", 4),
    ("cardinality", 5), ("event", 3), ("arg", 4), ("pp", 4),
    ("property", 4), ("operator", 3), ("interval", 6),
    ("recurrence", 4), ("frequency", 5), ("order", 4),
    ("recurrence_window", 7), ("range", 3),
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


@dataclasses.dataclass(frozen=True)
class Disposition:
    family: str
    item: str
    status: str
    fields: tuple[str, ...]

    @property
    def key(self):
        return self.family, self.item


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


def load_dispositions(data: bytes) -> dict[tuple[str, str], Disposition]:
    rows = {}
    for number, line in enumerate(data.decode("utf-8").splitlines(), 1):
        if not line or line.startswith("#"):
            continue
        fields = tuple(line.split("\t"))
        if len(fields) != 9 or fields[6] not in {"migrated", "re-targeted", "positive", "retired"}:
            raise InputError(f"{DISPOSITIONS}:{number}: invalid row")
        row = Disposition(fields[0], fields[1], fields[6], fields)
        if row.key in rows:
            raise InputError(f"{DISPOSITIONS}:{number}: duplicate {row.key}")
        rows[row.key] = row
    if not rows:
        raise InputError(f"{DISPOSITIONS}: no disposition rows")
    return rows


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
    @whole replaces an existing file; @new adds one; @whole + @deleted removes
    an explicitly retired support file. These operations all require count=1.
    The optional digest binds the immutable base blob, before any automatic rename.
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
        mode = {"@whole": "whole", "@new": "new"}.get(old, "edit")
        if new == "@deleted":
            if mode != "whole":
                raise InputError(f"{RULINGS}:{number}: @deleted requires old=@whole")
            mode = "delete"
        try:
            before = literal_bytes(old) if mode == "edit" else b""
            after = b"" if mode == "delete" else literal_bytes(new)
        except InputError as error:
            raise InputError(f"{RULINGS}:{number}: {error}") from error
        if mode != "edit" and count != "1":
            raise InputError(f"{RULINGS}:{number}: @{mode} count must be 1")
        if mode == "edit" and not before:
            raise InputError(f"{RULINGS}:{number}: empty old field; use @new or @whole")
        try:
            occurrences = int(count)
        except ValueError as error:
            raise InputError(f"{RULINGS}:{number}: count is outside the parser's integer domain") from error
        rows.append({"family": family, "item": item, "path": path, "base_sha256": digest,
                     "edits": [{"old": before, "new": after, "count": occurrences}],
                     "reason": reason, "mode": mode})
    return rows


def renamed(path: str) -> str:
    return path


def declaration_block(indicators) -> bytes:
    return b"".join(f":- {decl}(guideline_{name}/{arity}).\n".encode()
                    for name, arity in indicators for decl in ("multifile", "discontiguous"))


C1_DECLS = declaration_block(INDICATORS)
ADDED_DECLS = declaration_block((("grade", 5), ("preference", 4)))
S1_DECLS = C1_DECLS + ADDED_DECLS
assert len(ADDED_DECLS) == 150 and ADDED_DECLS.count(b"\n") == 4
BLOCK = re.compile(rb"(?m)^" + re.escape(C1_DECLS))


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


def query_transform(data: bytes) -> bytes:
    return data


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


def semantic_payload(data: bytes) -> bytes | None:
    kind = artifact_kind(data)
    if kind == "doc":
        marker = MARKER.search(data)
        return data[marker.start():] if marker else semantic_stream(data)
    if kind == "query":
        projection = re.search(rb"(?m)^'\$guideline_query_projection'[^\n]*\n?", data)
        return projection.group() if projection else None
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


def new_semantic_call(data: bytes) -> bytes | None:
    names = {b"guideline_grade", b"guideline_preference"}
    at = 0
    while at < len(data):
        if data[at:at + 1] == b"%":
            end = data.find(b"\n", at)
            at = len(data) if end < 0 else end + 1
            continue
        start = at
        if data[at] in (34, 39):
            quote = data[at]
            at += 1
            while at < len(data):
                if data[at] == 92:
                    at += 2
                elif data[at] == quote:
                    at += 1
                    break
                else:
                    at += 1
            token = data[start + 1:at - 1] if quote == 39 else b""
        elif 65 <= data[at] <= 90 or 97 <= data[at] <= 122 or data[at] in (36, 95):
            at += 1
            while at < len(data) and (data[at:at + 1].isalnum() or data[at] in (36, 95)):
                at += 1
            token = data[start:at]
        else:
            at += 1
            continue
        end = at
        while end < len(data) and data[end] in (9, 10, 13, 32):
            end += 1
        if token in names and data[end:end + 1] == b"(":
            return token
    return None


def header_line_shift(data: bytes, line: int) -> int:
    return 4 * sum(data.count(b"\n", 0, at) < line for at in insertion_points(data))


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






def loose_ui_document(path: str, data: bytes) -> bool:
    return (path.startswith("tests/ui/") and path.endswith(".pl") and
            not data.startswith(b"%") and b":- multifile(" not in data and
            b":- discontiguous(" not in data and MARKER.search(data) is None)




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
        new = document_transform(old) if kind == "doc" else query_transform(old) if kind == "query" else old
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


def disposition_covers(row: Disposition, path: str) -> bool:
    family, item = row.key
    if family == "tests/v1":
        case = item.split("/", 1)[0]
        return path.startswith(f"tests/v1/{case}/") or path in {"tests/v1/cases.tsv", "rust/ckc/tests/v1_suite.rs"}
    if family in {"tests/red", "tests/green", "tests/check-mutants"}:
        return (path.startswith(f"{family}/{item}.") or path.startswith(renamed(f"{family}/{item}") + ".") or path == f"{family}/cases.tsv" or
                (family == "tests/check-mutants" and path == "rust/ckc/tests/check_mutants.rs"))
    if family == "tests/certify":
        return path == "tests/certify/cases.tsv"
    if family == "tests/certify-one":
        return path.startswith("tests/certify-one/") or path in {"rust/ckc/tests/certify_one.rs", ".agent/archive/generators/certify-one-fixtures"}
    if family in {"tests/ui", "tests/queries"}:
        return path.startswith(f"{family}/{item}/") or (family == "tests/ui" and path == "rust/ckc/tests/ui_fixtures.rs")
    if family == "q18 census":
        return path == "rust/ckc/tests/q18_fixed_name_census.rs" or path.startswith("tests/v1/q18-k1-001/")
    return family == path == "rust/ckc/tests/q13_temporal_properties.rs"


class Oracle:
    def __init__(self, base: dict[str, Blob], dispositions: dict[tuple[str, str], Disposition],
                 rulings: list[dict] = ()):
        self.base = base
        self.base_origin = {renamed(path): path for path in base}
        self.dispositions = dispositions
        self.rulings = list(rulings)
        self.expected: dict[str, Blob] = {}
        self.origin: dict[str, str] = {}
        self.hashes: dict[bytes, bytes] = {}
        self.retired: set[str] = set()
        self.offsets: set[str] = set()
        self.rule_paths: set[str] = set()
        self.counts = collections.Counter()
        self.commit_ids: dict[str, bytes] = {}
        self.ui_inputs: dict[str, bytes] = {}
        self.new_items: set[tuple[str, str]] = set()
        self.term_counts: set[str] = set()
        self.item_inventory: dict[str, set[str]] = {}

    def base_item_exists(self, family: str, item: str) -> bool:
        if family not in self.item_inventory:
            if family in {"tests/v1", "tests/check-mutants", "tests/certify"}:
                rows = parse_tsv(self.base.get(f"{family}/cases.tsv", Blob(b"")).data)
                if family == "tests/v1":
                    items = {"/".join(row[:2]) for row in rows}
                else:
                    items = {row[0] for row in rows if row[:2] != ["id", "kind"]}
                if family == "tests/check-mutants":
                    items.update(row[0] for row in parse_tsv(self.base.get(f"{family}/late.tsv", Blob(b"")).data)
                                 if row != ["name", "rc"])
                self.item_inventory[family] = items
            elif family in {"tests/red", "tests/green"}:
                items = {p[len(family) + 1:-4] for p in self.base if p.startswith(family + "/") and p.endswith(".ace")}
                self.item_inventory[family] = items | {renamed(f"{family}/{i}")[len(family) + 1:] for i in items}
            elif family in {"tests/ui", "tests/queries"}:
                self.item_inventory[family] = {"/".join(p.split("/")[2:4]) for p in self.base if p.startswith(family + "/")}
            elif family == "tests/certify-one":
                self.item_inventory[family] = {p for p in self.base if p.startswith(family + "/")}
            else:
                self.item_inventory[family] = {i for f, i in self.dispositions if f == family}
        return item in self.item_inventory[family]

    def validate_dispositions(self):
        manifests = {
            "tests/v1": {"/".join(r[:2]) for r in parse_tsv(self.base.get("tests/v1/cases.tsv", Blob(b"")).data)},
            "tests/check-mutants": {r[0] for r in parse_tsv(self.base.get("tests/check-mutants/cases.tsv", Blob(b"")).data)},
            "tests/certify": {r[0] for r in parse_tsv(self.base.get("tests/certify/cases.tsv", Blob(b"")).data)},
        }
        for row in self.dispositions.values():
            family, item = row.key
            if family in manifests:
                valid = item in manifests[family]
            elif family in {"tests/red", "tests/green"}:
                valid = f"{family}/{item}.ace" in self.base
            elif family == "tests/certify-one":
                valid = item in self.base
            elif family in {"tests/ui", "tests/queries"}:
                valid = any(p.startswith(f"{family}/{item}/") for p in self.base)
            elif family == "q18 census":
                seed, separator, slot = item.partition("/")
                valid = separator and slot in {f"S{i}" for i in range(1, 9)} and f"tests/v1/q18-k1-001/{seed}.pl.in" in self.base
            else:
                valid = family == "rust/ckc/tests/q13_temporal_properties.rs"  # Native harness review owns generated domains.
            if not valid and row.status != "retired":
                valid = any((r["family"], r["item"]) == row.key and r.get("mode") == "new" and
                            disposition_covers(row, r["path"]) for r in self.rulings)
            if not valid:
                raise InputError(f"disposition {family}/{item}: item absent from base enumeration and no ruled addition")



    def initial_transform(self, path: str, blob: Blob) -> bytes:
        data = blob.data
        if blob.mode == "120000":
            return data
        if path == KANI:
            data, pairs = kani_transform(data)
            for old, new in pairs:
                self.remember_hash(old, new)
            return data
        if path.endswith((".html", ".golden", "assertions.tsv")) or table_file(path) or data.startswith(TABLE_HEADER):
            return data
        if artifact_kind(data) in {"query", "answers", "traces"} or b"'$guideline_proof'(" in data:
            return data
        data = document_transform(data)
        if path.endswith(".tpl.in"):
            data = template_pin(data)
        return data

    def remember_hash(self, old: bytes, new: bytes):
        if old == new:
            return
        a, b = sha(old), sha(new)
        previous = self.hashes.get(a)
        if previous is not None and previous != b:
            raise InputError(f"ambiguous digest rebinding for {a.decode()}")
        self.hashes[a] = b

    def digest_closure(self):
        # Derive from immutable bases each round; stale red digests stay stale.
        for _ in range(len(self.expected) + 1):
            changed = False
            pending = {}
            for path, blob in self.expected.items():
                if path.startswith("tests/ui/") and path.endswith((".html", ".golden")):
                    data = re.sub(rb'(<input (?=[^>]*type="hidden")(?=[^>]*name="ledger_sha256")[^>]*value=")([0-9a-f]{64})(")',
                                  lambda m: m[1] + self.hashes.get(m[2], m[2]) + m[3], blob.data)
                else:
                    data = blob.data if protected_payload(path, blob.data) else rebind_bytes(blob.data, self.hashes)
                if path.endswith(".tpl.in"):
                    data = template_pin(data)
                if data != blob.data:
                    pending[path] = Blob(data, blob.mode)
                    changed = True
            if not changed:
                return
            for path, blob in pending.items():
                previous = self.expected[path].data
                self.hashes[sha(previous)] = sha(blob.data)
                if path == KANI:
                    for old, new in zip(embedded_artifacts(previous), embedded_artifacts(blob.data), strict=True):
                        if old != new:
                            self.hashes[sha(old)] = sha(new)
                if path.endswith(".tpl.in"):
                    self.hashes[sha(expand_template(previous))] = sha(expand_template(blob.data))
                original = self.base[self.origin[path]].data if path in self.origin else b""
                # A digest may advance as its dependencies stabilize.
                if original:
                    self.hashes[sha(original)] = sha(blob.data)
                    if path.endswith(".tpl.in"):
                        self.hashes[sha(expand_template(original))] = sha(expand_template(blob.data))
            self.expected.update(pending)
        raise InputError("digest dependency cycle")

    def document_answers_diagnostic(self, row: list[str]):
        case, probe, mode, _, _, arguments = row
        args = arguments.split()
        if mode not in {"trace", "trace-check"} or len(args) < 3:
            return
        source = f"tests/v1/{case}/{args[2]}.in"
        if source not in self.base or renamed(source) not in self.expected:
            return
        old, new = self.base[source].data, self.expected[renamed(source)].data
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
                if len(fields) == 8 and fields[3] == b"swap-lines":
                    target = fields[2].decode()
                    if target in self.base:
                        for i in (4, 5):
                            index = int(fields[i])
                            fields[i] = str(index + header_line_shift(self.base[target].data, index + 1)).encode()
                        line = b"\t".join(fields) + (b"\n" if line.endswith(b"\n") else b"")
                output.append(line)
            self.expected[path] = Blob(b"".join(output), self.base[path].mode)

    def update(self, path: str, data: bytes):
        blob = self.expected[path]
        if blob.data != data:
            self.expected[path] = Blob(data, blob.mode)
            self.hashes[sha(blob.data)] = sha(data)
            if path in self.origin:
                self.hashes[sha(self.base[self.origin[path]].data)] = sha(data)


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
            key = row["family"], row["item"]
            mode = row.get("mode", "edit")
            new_reason = row["reason"].startswith("new:")
            if new_reason and (key in self.dispositions or self.base_item_exists(*key)):
                raise InputError(f"ruling {key}: new: item exists in base/D14")
            new_coverage = key not in self.dispositions
            if new_coverage:
                if not new_reason:
                    raise InputError(f"ruling {key}: no disposition row or new: declaration")
                shared = row["path"] == f"{row['family']}/cases.tsv" and row["family"] in {"tests/v1", "tests/check-mutants", "tests/certify"}
                if mode != "new" and not (mode == "edit" and shared):
                    raise InputError(f"ruling {key}: new: requires @new or its own shared-manifest insertion")
                disposition = Disposition(key[0], key[1], "new", ())
            else:
                disposition = self.dispositions[key]
            if disposition.status == "retired" and mode != "delete":
                raise InputError(f"ruling {key}: retired item cannot gain files")
            path = row["path"]
            if not scope_path(path):
                raise InputError(f"ruling {key}: {path}: outside migration scope")
            source = self.origin.get(path, self.base_origin.get(path, path))
            if not (disposition_covers(disposition, source) or disposition_covers(disposition, path)):
                raise InputError(f"ruling {key}: {path}: unrelated disposition")
            original = self.base.get(source)
            digest = None if original is None else sha(original.data).decode()
            if row.get("base_sha256") is not None and digest != row["base_sha256"]:
                raise InputError(f"ruling {key}: {path}: base_sha256 mismatch")
            if mode == "new":
                if original is not None or path in self.expected:
                    raise InputError(f"ruling {key}: {path}: @new path exists")
            elif path not in self.expected:
                raise InputError(f"ruling {key}: {path}: absent file; use @new for additions")
            if mode == "delete":
                raise InputError(f"ruling {key}: S1 header migration retains every base fixture")
            blob = self.expected.get(path, Blob(b""))
            before = data = blob.data
            for edit in row["edits"]:
                old, new, count = edit["old"], edit["new"], edit["count"]
                prior = data
                if mode in {"new", "whole"}:
                    if count != 1:
                        raise InputError(f"ruling {key}: {path}: whole-file count must be 1")
                    data = new
                elif not old or data.count(old) != count:
                    raise InputError(f"ruling {key}: {path}: edit anchor count {data.count(old)} != {count}")
                else:
                    data = data.replace(old, new)
            if original is not None and loose_ui_document(source, original.data) and data != original.data:
                raise InputError(f"ruling {key}: {path}: G2 opaque UI pseudo-document must stay byte-identical")
            if before and not data and path.endswith((".stderr", ".expect")) and disposition.status != "positive":
                raise InputError(f"ruling {key}: {path}: retained reject erased")
            if path in {"tests/v1/cases.tsv", "tests/check-mutants/cases.tsv", "tests/certify/cases.tsv"}:
                columns = 2 if path == "tests/v1/cases.tsv" else 1
                def manifest_rows(blob):
                    result = {}
                    for line in blob.splitlines():
                        if line and not line.startswith(b"#"):
                            name = b"/".join(line.split(b"\t")[:columns]).decode()
                            if name in result:
                                raise InputError(f"ruling {key}: {path}: duplicate case row {name}")
                            result[name] = line
                    return result
                old_rows, new_rows = manifest_rows(before), manifest_rows(data)
                changed = {name for name in old_rows.keys() | new_rows.keys() if old_rows.get(name) != new_rows.get(name)}
                if new_coverage and (changed != {row["item"]} or row["item"] in old_rows or old_rows.keys() - new_rows.keys()):
                    raise InputError(f"ruling {key}: new: shared-manifest edit must insert its own row only")
                if changed - {row["item"]} or old_rows.keys() - new_rows.keys():
                    raise InputError(f"ruling {key}: {path}: unlisted or omitted case row")
            if original is not None and disposition.status == "migrated" and semantic_payload(before) != semantic_payload(data):
                raise InputError(f"ruling {key}: {path}: S1 migration cannot alter semantic payloads")
            self.expected[path] = Blob(data, blob.mode)
            if original is not None:
                self.origin[path] = source
            self.rule_paths.add(path)
            if new_coverage:
                self.new_items.add(key)

    def validate_pure_inputs(self):
        for path, blob in self.base.items():
            data = blob.data
            if table_file(path) or data.startswith(TABLE_HEADER):
                if re.search(rb"(?m)^(?:strength|evidence|preference|alternative)\t", data):
                    raise InputError(f"{path}: base table already has an S1 row")
            payloads = embedded_artifacts(data) if path == KANI else [data] if protected_payload(path, data) else []
            for payload in payloads:
                if (name := new_semantic_call(payload)) is not None:
                    raise InputError(f"{path}: base already has S1 literal {name.decode()}")

    def build(self):
        self.validate_pure_inputs()
        for path, blob in sorted(self.base.items()):
            new = self.initial_transform(path, blob)
            self.expected[path] = Blob(new, blob.mode)
            self.origin[path] = path
            self.remember_hash(blob.data, new)
            if path.endswith(".tpl.in"):
                old_exp = expand_template(blob.data)
                if blob.data.split(b"\n", 1)[0] != b"@@sha256 " + sha(old_exp):
                    raise InputError(f"{path}: base template pin does not match expansion")
                self.remember_hash(old_exp, expand_template(new))
            self.counts[artifact_kind(blob.data) or ("table" if table_file(path) else "other")] += 1
        self.diagnostic_transform()
        self.digest_closure()
        self.ui_transform()
        self.fixture_commits()
        self.digest_closure()
        for path, old in self.base.items():
            if path.endswith("/audit/review-manifest.tsv") and self.expected[path].data != old.data:
                raise InputError(f"{path}: header-only migration changed a review bundle")
            before, after = old.data, self.expected[path].data
            if path.endswith(".tpl.in"):
                before, after = expand_template(before), expand_template(after)
            if insertion_points(before) and not path.endswith((".html", ".golden")) and semantic_stream(before) != semantic_stream(after):
                raise InputError(f"{path}: header insertion changed the semantic stream")
        self.apply_rulings()
        return self

    def compare(self, actual: dict[str, Blob]) -> list[str]:
        expected = self.expected
        problems = []
        for path in sorted(expected.keys() | actual.keys()):
            if path not in expected:
                why = "retired item retained" if path in self.retired else "unlisted file"
                problems.append(f"{path}: {why}")
            elif path not in actual:
                problems.append(f"{path}: omitted file/case")
            elif expected[path].mode != actual[path].mode:
                problems.append(f"{path}: mode {actual[path].mode} != predicted {expected[path].mode}")
            elif expected[path].data != actual[path].data:
                problems.append(f"{path}: {first_difference(expected[path].data, actual[path].data)}")
        return problems


def parse_tsv(data: bytes) -> list[list[str]]:
    return [line.split("\t") for line in data.decode("utf-8").splitlines()
            if line and not line.startswith("#")]








def self_test_cases(oracle: Oracle):
    pristine = oracle.expected
    mutations = []
    for path, blob in sorted(pristine.items()):
        marker = MARKER.search(blob.data)
        if marker and b"guideline_event(" in blob.data[marker.start():]:
            at = blob.data.index(b"guideline_event(", marker.start())
            end = blob.data.index(b"\n", at)
            line = blob.data[at:end]
            head, separator, body = line.removesuffix(b".").partition(b" :- ")
            fields = split_args(head[len(b"guideline_event("):-1])
            if len(fields) != 3:
                continue
            fields[-1] = b"c1_oracle_mutant"
            altered = b"guideline_event(" + b",".join(fields) + b")" + separator + body + b"."
            changed = blob.data[:at] + altered + blob.data[end:]
            mutations.append(("semantic-clause", path, Blob(changed, blob.mode)))
            break
    for path, blob in sorted(pristine.items()):
        match = re.search(rb"(?:query_sha256|ace_sha256)\('?([0-9a-f]{64})", blob.data)
        if match and artifact_kind(blob.data):
            at = match.end(1) - 1
            replacement = b"0" if blob.data[at:at + 1] != b"0" else b"1"
            mutations.append(("wrong-digest", path, Blob(blob.data[:at] + replacement + blob.data[at + 1:], blob.mode)))
            break
    for path, blob in sorted(pristine.items(), key=lambda item: (item[0] not in oracle.offsets, item[0])):
        match = OFFSET.search(blob.data)
        if match and path.startswith("tests/v1/"):
            data = blob.data[:match.start()] + b"noncanonical(%d,%d)" % (int(match[1]) + 1, int(match[2])) + blob.data[match.end():]
            mutations.append(("wrong-offset", path, Blob(data, blob.mode)))
            break
    for path, blob in sorted(pristine.items(), key=lambda item: (not item[0].startswith("tests/red/"), item[0])):
        if path.endswith((".stderr", ".expect")) and blob.data.startswith(b"ace_to_pl_error("):
            mutations.append(("erased-reject", path, Blob(b"", blob.mode)))
            break
    extra = "tests/v1/s1-oracle-unlisted/input.pl.in"
    if extra in pristine:
        raise AssertionError("unlisted-file plant collides")
    mutations.append(("unlisted-file", extra, Blob(b"plant\n")))
    manifest = "tests/v1/cases.tsv"
    if manifest in pristine:
        blob = pristine[manifest]
        omitted = next((line for line in blob.data.splitlines(keepends=True) if line and not line.startswith(b"#")), None)
        if omitted is not None:
            mutations.append(("omitted-case", manifest, Blob(blob.data.replace(omitted, b"", 1), blob.mode)))
    else:
        for path in sorted(pristine):
            if path.endswith("/input.pl.in"):
                mutations.append(("omitted-case", path, None))
                break
    required = {"semantic-clause", "wrong-digest", "wrong-offset", "erased-reject", "unlisted-file", "omitted-case"}
    if {name for name, _, _ in mutations} != required:
        raise AssertionError("self-test plant anchor missing")
    return mutations


def self_test_rulings(oracle: Oracle):
    manifest = "tests/v1/cases.tsv"
    lines = oracle.expected.get(manifest, Blob(b"")).data.splitlines(keepends=True)
    existing = next((line for line in lines if not line.startswith(b"#") and
                     len(fields := line.rstrip(b"\n").split(b"\t")) >= 2 and
                     oracle.base_item_exists("tests/v1", b"/".join(fields[:2]).decode())), None)
    if existing is None:
        raise AssertionError("new-coverage self-test base-item anchor missing")
    fields = existing.rstrip(b"\n").split(b"\t")
    case, probe = fields[0].decode(), fields[1].decode()
    index = 0
    while (path := f"tests/v1/{case}/s1-oracle-authority-{index}.in") in oracle.expected or path in oracle.base_origin:
        index += 1
    base_item = {"family": "tests/v1", "item": case + "/" + probe, "path": path, "mode": "new",
                 "edits": [{"old": b"", "new": b"plant\n", "count": 1}], "reason": "new: invalid base-item redeclaration"}
    items = {"/".join(row[:2]) for row in parse_tsv(oracle.expected[manifest].data)}
    index = 0
    while (item := f"s1-oracle-new-{index}/target") in items or oracle.base_item_exists("tests/v1", item) or ("tests/v1", item) in oracle.dispositions:
        index += 1
    old_row = {"family": "tests/v1", "item": item, "path": manifest, "mode": "edit",
               "edits": [{"old": existing, "new": existing.rstrip(b"\n") + b" c1_authority_mutation\n", "count": 1}],
               "reason": "new: invalid existing-row change"}
    return [("new-base-item", base_item, "new: item exists in base/D14"),
            ("new-existing-row", old_row, "new: shared-manifest edit must insert its own row only")]


def self_test(oracle: Oracle) -> list[tuple[str, str]]:
    pristine = oracle.expected
    if oracle.compare(pristine):
        raise AssertionError("synthetic perfect migration rejected")
    evidence = []
    for name, path, blob in self_test_cases(oracle):
        tree = dict(pristine)
        if blob is None:
            del tree[path]
        else:
            tree[path] = blob
        problems = oracle.compare(tree)
        if not problems or not any(line.startswith(path + ":") for line in problems):
            raise AssertionError(f"self-test {name}: planted deviation accepted")
        evidence.append((name, problems[0]))
    for name, ruling, expected_error in self_test_rulings(oracle):
        probe = Oracle(oracle.base, oracle.dispositions, [ruling])
        probe.expected = dict(oracle.expected)
        probe.origin = dict(oracle.origin)
        probe.hashes = dict(oracle.hashes)
        probe.retired = set(oracle.retired)
        try:
            probe.apply_rulings()
        except InputError as error:
            if expected_error not in str(error):
                raise AssertionError(f"self-test {name}: wrong refusal: {error}") from error
            evidence.append((name, str(error)))
        else:
            raise AssertionError(f"self-test {name}: invalid new: authority accepted")
    return evidence


def main(argv: list[str]) -> int:
    if len(argv) != 3:
        print("s1-churn: usage: s1-churn.py <base-commit> <repo-root>")
        return 1
    try:
        root = Path(argv[2]).resolve(strict=True)
        policy = root / DISPOSITIONS
        dispositions = load_dispositions(policy.read_bytes()) if policy.exists() else {}
        rules_file = root / RULINGS
        rulings = load_rulings(rules_file.read_bytes()) if rules_file.exists() else []
        oracle = Oracle(base_blobs(root, argv[1]), dispositions, rulings)
        oracle.validate_dispositions()
        oracle.build()
        try:
            tests = self_test(oracle)
        except Exception as error:
            print("s1-churn: self-test failed: " + str(error).replace("\n", "\\n").replace("\r", "\\r"))
            return 2
        actual, _ = working_blobs(root)
        problems = oracle.compare(actual)
        if problems:
            for problem in problems:
                print("s1-churn: " + problem.replace("\n", "\\n").replace("\r", "\\r"))
            return 1
        changed = sum(oracle.base.get(p) != b for p, b in actual.items()) + len(oracle.base.keys() - actual.keys())
        print(f"s1-churn: ok {len(actual)} artifacts; {changed} changed paths; {len(dispositions)} dispositions; {len(oracle.new_items)} new coverage items; self-test {len(tests)}/8; 0 unpredicted changes")
        return 0
    except (InputError, OSError, UnicodeError) as error:
        print("s1-churn: " + str(error).replace("\n", "\\n").replace("\r", "\\r"))
        return 1


if __name__ == "__main__":
    sys.exit(main(sys.argv))
