#!/usr/bin/env python3
"""C1 D13 independent migration oracle; Python 3.13 stdlib, no compiler calls.

Usage: python3 -I c1-churn.py <base-commit> <repo-root>
Exit 0 = predicted tree, 1 = unpredicted changes/input errors, 2 = broken self-test.

The base's tracked fixture/corpus blobs are the inputs, not regenerated pins.
Normal review owns production source/docs. Coupled fixture sources are included
and fail closed unless an exact D14-linked ruling predicts their changes.
Rulings: .agent/archive/migrations/c1-churn-rulings.tsv (see load_rulings).
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

DISPOSITIONS = ".agent/archive/migrations/c1-dispositions.tsv"
RULINGS = ".agent/archive/migrations/c1-churn-rulings.tsv"
TABLE_HEADER = b"# format: kind\tlemma\tvalue\n"
OLD_KIND_V2 = b"# kind: unit (value: second|minute|hour|day|week|month|year) | relation (value: duration|within|after|before) | spacing (value: frame noun lemma)"
OLD_KIND_V3 = OLD_KIND_V2 + b" | window (value: frame noun lemma) | frequency (value: period) | approximation (value: about) | range (value: minimum)"
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
            path == "release-manifest.tsv" or path in COUPLED)


def scope_paths(paths) -> set[str]:
    return {safe_path(p) for p in paths if scope_path(p)}


def base_blobs(root: Path, revision: str) -> dict[str, Blob]:
    commit = git(root, "rev-parse", "--verify", f"{revision}^{{commit}}").strip().decode()
    entries = {}
    for row in git(root, "ls-tree", "-rz", "--full-tree", commit).split(b"\0"):
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
    payload = git(root, "cat-file", "--batch", data=wire)
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
    p = PurePosixPath(path)
    name = p.name
    if name.startswith("temporal_load--"):
        name = "vocabulary_load--" + name.removeprefix("temporal_load--")
    for suffix in ("temporal.tsv.in", "temporal.tsv"):
        if name == suffix or name.endswith("." + suffix):
            name = name[:-len(suffix)] + suffix.replace("temporal", "vocabulary", 1)
            break
    return str(p.with_name(name))


def declaration_block(version: int) -> bytes:
    indicators = list(INDICATORS[:{1: 9, 2: 11, 3: 15}[version]])
    if version == 1:
        indicators[1] = ("document", 3)
    return b"".join(f":- {decl}(guideline_{name}/{arity}).\n".encode()
                    for name, arity in indicators for decl in ("multifile", "discontiguous"))


DECLS = declaration_block(3)


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


def envelope_record(match: re.Match, query: bool) -> bytes:
    try:
        fields = split_args(match.group(1))
    except ValueError:
        return match.group()
    old_count = 4 if query else 3
    if query:
        if fields[0] not in (b"v1", b"v2", b"v3"):
            return match.group()
        fields[0] = b"v1"
    if len(fields) == old_count:
        fields.append(b"vocabulary(none)")
    elif len(fields) == old_count + 1 and fields[-1].lstrip().startswith(b"temporal("):
        fields[-1] = fields[-1].replace(b"temporal(", b"vocabulary(", 1)
    else:
        return match.group()
    name = b"'$guideline_query'" if query else b"guideline_document"
    return name + b"(" + b",".join(fields) + b").\n"


def document_transform(data: bytes) -> bytes:
    marker = MARKER.search(data)
    end = marker.start() if marker else len(data)
    head, tail = data[:end], data[end:]
    # Longest first: v3 shares its leading declarations with v2.
    for version in (3, 2, 1):
        old = declaration_block(version)
        if old in head:
            head = head.replace(old, DECLS, 1)
            break
    head = SCHEMA.sub(b"guideline_schema_version(1).\n", head)
    head = DOC_RECORD.sub(lambda m: envelope_record(m, False), head)
    return head + tail


def query_transform(data: bytes) -> bytes:
    return QUERY_RECORD.sub(lambda m: envelope_record(m, True), data)


def table_transform(data: bytes) -> bytes:
    # Invalid headers remain invalid; their re-targeting needs an exact ruling.
    for kinds in (OLD_KIND_V3, OLD_KIND_V2):
        header = TABLE_HEADER + kinds + b"\n"
        if data.startswith(header):
            return TABLE_HEADER + data[len(header):]
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


def rebind_artifact(data: bytes, mapping: dict[bytes, bytes]) -> bytes:
    kind = artifact_kind(data)
    if kind in ("doc", "query"):
        def record(match):
            try:
                fields = split_args(match[1])
            except ValueError:
                return match.group()
            start = 1 if kind == "doc" else 2
            fields[start:] = [SHA_TERM.sub(lambda m: atom(mapping[m[2]]) if m[2] in mapping else m.group(), field)
                              for field in fields[start:]]
            return match.group().replace(match[1], b",".join(fields), 1)
        if kind == "doc":
            marker = MARKER.search(data)
            end = marker.start() if marker else len(data)
            return DOC_RECORD.sub(record, data[:end]) + data[end:]
        return QUERY_RECORD.sub(record, data)
    if kind in ("answers", "traces"):
        def custody(match):
            try:
                fields = split_args(match[2])
            except ValueError:
                return match.group()
            indices = (2,) if match[1] == b"'$guideline_answers'" else (2, 3)
            if len(fields) != len(indices) + 3:
                return match.group()
            for i in indices:
                fields[i] = re.sub(rb"(?<![0-9a-f])('?)([0-9a-f]{64})\1(?![0-9a-f])",
                                   lambda m: atom(mapping[m[2]]) if m[2] in mapping else m.group(), fields[i])
            return match.group().replace(match[2], b",".join(fields), 1)
        return re.sub(rb"(?m)^('\$guideline_(?:answers|traces)')\(([^\n]*)\)\.(?=\n|$)", custody, data)
    return rebind_bytes(data, mapping)


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


def semantic_input_valid(data: bytes) -> bool:
    lines = data.removesuffix(b"\n").split(b"\n")
    return (data.endswith(b"\n") and sum(line.startswith(b"guideline_document(") for line in lines) == 1 and
            all(line.startswith((b"%", b":- ")) or line.endswith(b".") for line in lines))


def bundle_digest(fields: list[bytes]) -> bytes:
    docid, ace, coverage, payload, clauses = fields[:5]
    return sha(b"bundle v2 " + docid + b"\nace " + ace + b"\ncoverage " + coverage +
               b"\npayload " + payload + b"\nclauses " + clauses + b"\n")


def loose_ui_document(path: str, data: bytes) -> bool:
    return (path.startswith("tests/ui/") and path.endswith(".pl") and
            not data.startswith(b"%") and b":- multifile(" not in data and
            b":- discontiguous(" not in data and MARKER.search(data) is None)


def review_row_digests(data: bytes) -> dict[bytes, bytes]:
    return {f[0]: f[5] for line in data.splitlines()
            if not line.startswith(b"#") and len(f := line.split(b"\t")) == 6}


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


def kani_custody(data: bytes) -> bytes:
    queries = [b for b in embedded_artifacts(data) if artifact_kind(b) == "query"]
    if len(queries) != 2:
        raise InputError("Kani query constant census")
    qshas = list(map(sha, queries))

    def answer_record(match):
        old = concat_payload(match)
        if artifact_kind(old) != "answers":
            return match.group()
        new = re.sub(rb"query_sha256\('?([0-9a-f]{64})'?\)",
                     lambda _: b"query_sha256(" + atom(qshas[0]) + b")", old)
        return render_concat(match, new) if old != new else match.group()

    data = RUST_CONCAT.sub(answer_record, data)
    data = re.sub(rb'(const QUERY_SHA256:[^=]*=\s*\[\s*b")[0-9a-f]{64}("\s*,\s*b")[0-9a-f]{64}("\s*,?\s*\])',
                  lambda m: m[1] + qshas[0] + m[2] + qshas[1] + m[3], data)
    answers = [b for b in embedded_artifacts(data) if artifact_kind(b) == "answers"]
    if len(answers) != 1:
        raise InputError("Kani answer constant census")
    return re.sub(rb'(&answers,\s*b")[0-9a-f]{64}(")',
                  lambda m: m[1] + sha(answers[0]) + m[2], data)


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
        self.review_changes: dict[str, dict[bytes, tuple[bytes, bytes]]] = {}
        self.review_semantics: dict[tuple[str, bytes], bytes] = {}
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

    def retirements(self):
        v1_rows = parse_tsv(self.base.get("tests/v1/cases.tsv", Blob(b"")).data)
        remaining = collections.Counter(row[0] for row in v1_rows
                                        if self.dispositions.get(("tests/v1", "/".join(row[:2])), None) is None or
                                        self.dispositions[("tests/v1", "/".join(row[:2]))].status != "retired")
        for disposition in self.dispositions.values():
            if disposition.status != "retired":
                continue
            family, item = disposition.key
            if family == "tests/v1":
                case, probe = item.split("/", 1)
                if not remaining[case]:
                    self.retired.update(p for p in self.base if p.startswith(f"tests/v1/{case}/"))
                else:
                    self.retired.update({f"tests/v1/{case}/expect/{probe}.stdout", f"tests/v1/{case}/expect/{probe}.stderr"})
            elif family in {"tests/red", "tests/green", "tests/check-mutants"}:
                self.retired.update(p for p in self.base if p.startswith(f"{family}/{item}."))

    def manifest_transform(self, path: str, data: bytes) -> bytes:
        if path not in {"tests/v1/cases.tsv", "tests/check-mutants/cases.tsv", "tests/certify/cases.tsv"}:
            return data
        family = str(PurePosixPath(path).parent)
        result = []
        for line in data.splitlines(keepends=True):
            fields = line.rstrip(b"\n").split(b"\t")
            if line.startswith(b"#") or not line.strip():
                result.append(line)
                continue
            item = b"/".join(fields[:2]).decode() if family == "tests/v1" else fields[0].decode()
            d = self.dispositions.get((family, item))
            if d is None or d.status != "retired":
                result.append(line)
        return b"".join(result)

    def initial_transform(self, path: str, blob: Blob) -> bytes:
        data = blob.data
        if blob.mode == "120000" or loose_ui_document(path, data):
            return data
        if path == KANI:
            data, pairs = kani_transform(data)
            for old, new in pairs:
                self.remember_hash(old, new)
            return data
        if table_file(path):
            return table_transform(data)
        if path in COUPLED:
            return data
        kind = None if path.endswith((".html", ".golden")) else artifact_kind(data)
        if kind == "doc":
            data = document_transform(data)
        elif kind == "query":
            data = query_transform(data)
        elif path.endswith((".expect", ".stderr", ".stdout")):
            data = data.replace(b"ckc: temporal:", b"ckc: vocabulary:")
            data = data.replace(b"temporal_load", b"vocabulary_load")
            data = data.replace(b" temporal.tsv:", b" vocabulary.tsv:")
        # Exact path spellings, not the semantic temporal_shape error name.
        if kind is None and path not in COUPLED and not path.endswith(".rs"):
            data = data.replace(b".temporal.tsv", b".vocabulary.tsv")
            data = data.replace(b"/temporal.tsv", b"/vocabulary.tsv")
        data = self.manifest_transform(path, data)
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
                if loose_ui_document(path, blob.data):
                    continue
                if path.startswith("tests/ui/") and path.endswith((".html", ".golden")):
                    data = re.sub(rb'(<input (?=[^>]*type="hidden")(?=[^>]*name="(?:review_sha256|ledger_sha256)")[^>]*value=")([0-9a-f]{64})(")',
                                  lambda m: m[1] + self.hashes.get(m[2], m[2]) + m[3], blob.data)
                else:
                    data = rebind_artifact(blob.data, self.hashes)
                if path == KANI:
                    data = kani_custody(data)
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
            if len(row) != 6 or row[2] not in ("check", "render") or not row[5]:
                continue
            case, probe, _, _, _, args = row
            source = f"tests/v1/{case}/{args.split()[0]}.in"
            if source not in self.base or renamed(source) not in self.expected:
                continue
            old, new = self.base[source].data, self.expected[renamed(source)].data
            old_marker, new_marker = MARKER.search(old), MARKER.search(new)
            if old_marker is None or new_marker is None:
                continue
            old_line = old.count(b"\n", 0, old_marker.start()) + 1
            new_line = new.count(b"\n", 0, new_marker.start()) + 1
            delta = new_line - old_line
            if not delta:
                continue
            for stream in ("stdout", "stderr"):
                path = f"tests/v1/{case}/expect/{probe}.{stream}"
                if path not in self.expected:
                    continue
                blob = self.expected[path]
                data = OFFSET.sub(lambda m: b"noncanonical(%d,%d)" % (int(m[1]) + delta, int(m[2]))
                                  if int(m[1]) >= old_line else m.group(), blob.data)
                if data != blob.data:
                    self.expected[path] = Blob(data, blob.mode)
                    self.offsets.add(path)

    def update(self, path: str, data: bytes):
        blob = self.expected[path]
        if blob.data != data:
            self.expected[path] = Blob(data, blob.mode)
            self.hashes[sha(blob.data)] = sha(data)
            if path in self.origin:
                self.hashes[sha(self.base[self.origin[path]].data)] = sha(data)

    def review_transform(self):
        for path, original in self.base.items():
            if not path.endswith("/audit/review-manifest.tsv") or path not in self.expected:
                continue
            prefix = path.removesuffix("audit/review-manifest.tsv")
            current = self.expected[path].data.splitlines(keepends=True)
            old_lines = original.data.splitlines(keepends=True)
            if len(current) != len(old_lines):
                continue
            for i, old_line in enumerate(old_lines):
                fields = old_line.rstrip(b"\n").split(b"\t")
                if len(fields) != 6 or not all(re.fullmatch(rb"[0-9a-f]{64}", v) for v in fields[1:]):
                    continue
                document = prefix + "pl/" + fields[0].decode() + ".pl"
                if document not in self.base or renamed(document) not in self.expected:
                    continue
                old_doc, new_doc = self.base[document].data, self.expected[renamed(document)].data
                if not semantic_input_valid(old_doc) or not semantic_input_valid(new_doc):
                    continue
                old_sem, new_sem = sha(semantic_stream(old_doc)), sha(semantic_stream(new_doc))
                migrated = current[i].rstrip(b"\n").split(b"\t")
                if len(migrated) != 6 or migrated[0] != fields[0]:
                    continue
                last_sem = self.review_semantics.get((path, fields[0]), old_sem)
                if fields[4] == old_sem and migrated[4] in {old_sem, last_sem}:
                    migrated[4] = new_sem
                    self.review_semantics[path, fields[0]] = new_sem
                last_review = self.review_changes.get(path, {}).get(fields[0], (fields[5], fields[5]))[1]
                if fields[5] == bundle_digest(fields) and migrated[5] in {fields[5], last_review}:
                    new_review = bundle_digest(migrated)
                    migrated[5] = new_review
                    if new_review != fields[5]:
                        self.review_changes.setdefault(path, {})[fields[0]] = (fields[5], new_review)
                current[i] = b"\t".join(migrated) + b"\n"
            self.update(path, b"".join(current))
        bindings = collections.defaultdict(dict)
        for path, rows in self.review_changes.items():
            prefix = "/".join(path.split("/")[:4]) + "/" if path.startswith("tests/ui/") else path.removesuffix("audit/review-manifest.tsv")
            for old, new in rows.values():
                if old in bindings[prefix] and bindings[prefix][old] != new:
                    raise InputError(f"{path}: inconsistent independently derived review digest")
                bindings[prefix][old] = new
        for path, blob in list(self.expected.items()):
            permitted = (path.endswith("/audit/adjudication.tsv") or
                         (path.startswith("tests/ui/") and path.endswith((".html", ".golden", "case.tsv", "assertions.tsv"))))
            if permitted:
                for prefix, mapping in bindings.items():
                    if path.startswith(prefix):
                        self.update(path, self.rebind_review_refs(path, self.expected[path].data, mapping))

    def ui_transform(self):
        changes = collections.defaultdict(list)
        word_cases = set()
        for path, original in self.base.items():
            if not path.startswith("tests/ui/"):
                continue
            parts = path.split("/")
            if len(parts) < 5:
                continue
            case = "/".join(parts[:4]) + "/"
            if path.endswith(".pl") and ("/tree/" in path or "/worktree/" in path):
                target = renamed(path)
                if target in self.expected:
                    old = self.ui_inputs.get(path, original.data)
                    new = self.expected[target].data
                    if old != new:
                        changes[case].append((ui_escape(old), ui_escape(new)))
                        self.ui_inputs[path] = new
            if table_file(path) and original.data.startswith(TABLE_HEADER + b"# kind: "):
                header = original.data.split(b"\n")[1]
                if b" | window " not in header:
                    word_cases.add(case)
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
                if case in word_cases:
                    for direction in (b"before", b"after"):
                        old = b"how long " + direction + b" a reference point the action occurs</td>"
                        new = b"how long " + direction + b" a reference point the action occurs, or only that it occurs " + direction + b" it</td>"
                        data = data.replace(old, new)
                return data

            if path.endswith("assertions.tsv"):
                output = []
                for line in blob.data.splitlines(keepends=True):
                    fields = line.removesuffix(b"\n").split(b"\t")
                    if len(fields) == 3 and fields[1] in {b"contains", b"absent"} and re.fullmatch(rb"(?:[0-9a-f]{2})*", fields[2]):
                        fragment = bytes.fromhex(fields[2].decode())
                        context = self.expected.get(case + "golden/" + fields[0].decode(), Blob(fragment)).data
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

    def release_transform(self):
        path = "release-manifest.tsv"
        if path not in self.expected:
            return
        result, schema_seen = [], False
        for line in self.expected[path].data.splitlines(keepends=True):
            fields = line.rstrip(b"\n").split(b"\t")
            if fields[:2] == [b"meta", b"schema"]:
                if not schema_seen:
                    result.append(b"meta\tschema\tv1\n")
                    schema_seen = True
                continue
            if len(fields) == 4 and fields[0] == b"member":
                member = fields[1].decode()
                target = renamed(member.removeprefix("data/"))
                if target in self.expected:
                    data = self.expected[target].data
                    fields[1] = ("data/" + target if member.startswith("data/") else target).encode()
                    fields[2], fields[3] = sha(data), str(len(data)).encode()
                    line = b"\t".join(fields) + b"\n"
            result.append(line)
        self.update(path, b"".join(result))

    def rebind_review_refs(self, path: str, data: bytes, mapping: dict[bytes, bytes]) -> bytes:
        if path.endswith(("/audit/review-manifest.tsv", "/audit/adjudication.tsv")):
            is_manifest = path.endswith("review-manifest.tsv")
            manifest = path if is_manifest else path.removesuffix("adjudication.tsv") + "review-manifest.tsv"
            if manifest not in self.base and "/after/" in manifest:
                manifest = manifest.replace("/after/", "/tree/", 1)
            permitted = review_row_digests(self.base.get(manifest, Blob(b"")).data)
            column, width = (5, 6) if is_manifest else (1, 7)
            output = []
            for line in data.splitlines(keepends=True):
                fields = line.removesuffix(b"\n").split(b"\t")
                if len(fields) == width and fields[column] == permitted.get(fields[0]):
                    fields[column] = mapping.get(fields[column], fields[column])
                    line = b"\t".join(fields) + (b"\n" if line.endswith(b"\n") else b"")
                output.append(line)
            return b"".join(output)
        if path.endswith((".html", ".golden")):
            return re.sub(rb'(<input (?=[^>]*type="hidden")(?=[^>]*name="review_sha256")[^>]*value=")([0-9a-f]{64})(")',
                          lambda m: m[1] + mapping.get(m[2], m[2]) + m[3], data)
        data = re.sub(rb'(name="review_sha256"[^>\n]*value=")([0-9a-f]{64})(")',
                      lambda m: m[1] + mapping.get(m[2], m[2]) + m[3], data)
        return re.sub(rb"(review_sha256=)([0-9a-f]{64})(?=&|[ \t\n]|$)",
                      lambda m: m[1] + mapping.get(m[2], m[2]), data)

    def release_view(self, actual: dict[str, Blob]) -> dict[str, Blob]:
        expected = dict(self.expected)
        path = "release-manifest.tsv"
        if path in expected and path in actual:
            wanted = expected[path].data.splitlines(keepends=True)
            got = actual[path].data.splitlines(keepends=True)
            if len(wanted) == len(got):
                for i, (old_line, new_line) in enumerate(zip(wanted, got)):
                    old = old_line.rstrip(b"\n").split(b"\t")
                    new = new_line.rstrip(b"\n").split(b"\t")
                    if len(old) == len(new) == 3 and old[:2] == new[:2] == [b"meta", b"compiler"]:
                        if re.fullmatch(rb"[0-9a-f]{64}", new[2]):
                            wanted[i] = new_line
                    if (len(old) == len(new) == 4 and old[:2] == new[:2] and old[0] == b"member" and
                            (old[1] == b"README-dist.md" or old[1].endswith(b"/README.md")) and
                            re.fullmatch(rb"[0-9a-f]{64}", new[2]) and re.fullmatch(rb"[1-9][0-9]*", new[3])):
                        wanted[i] = new_line
                expected[path] = Blob(b"".join(wanted), expected[path].mode)
        return expected

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
                support = source == "tests/certify-one/v3-header.txt.in" and disposition.family == "tests/certify-one"
                seed = disposition.item.split("/", 1)[0]
                rebuilt_seed = (disposition.family == "q18 census" and
                                re.fullmatch(r"(?:document|query)-v[123]-(?:none|sha256)", seed) is not None and
                                source == f"tests/v1/q18-k1-001/{seed}.pl.in")
                if original is None or not (support or rebuilt_seed):
                    raise InputError(f"ruling {key}: {path}: deletion is not a named retired/rebuilt support file")
                del self.expected[path]
                self.origin.pop(path, None)
                self.retired.add(source)
                self.rule_paths.add(path)
                continue
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
                if row["reason"].startswith("order:"):
                    check_order_edit(prior, data, row["reason"], path)
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
            semantic_changed = semantic_payload(before) != semantic_payload(data)
            if original is not None and disposition.status == "migrated" and semantic_changed:
                if not row["reason"].startswith("order:"):
                    raise InputError(f"ruling {key}: {path}: semantic change requires an order: ruling")
            if row["reason"].startswith("order:"):
                if original is None or not (b"guideline_schema_version(2)." in original.data or b"'$guideline_query'(v2," in original.data):
                    raise InputError(f"ruling {key}: {path}: order exception requires former-v2 input")
            self.expected[path] = Blob(data, blob.mode)
            if original is not None:
                self.origin[path] = source
            self.rule_paths.add(path)
            if new_coverage:
                self.new_items.add(key)

    def build(self):
        self.retirements()
        for path, blob in sorted(self.base.items()):
            if path in self.retired:
                continue
            target = renamed(path)
            if target in self.expected:
                raise InputError(f"rename collision: {path} -> {target}")
            new = self.initial_transform(path, blob)
            self.expected[target] = Blob(new, blob.mode)
            self.origin[target] = path
            self.remember_hash(blob.data, new)
            if path.endswith(".tpl.in"):
                old_exp = expand_template(blob.data)
                if blob.data.split(b"\n", 1)[0] != b"@@sha256 " + sha(old_exp):
                    raise InputError(f"{path}: base template pin does not match expansion")
                self.remember_hash(old_exp, expand_template(new))
            self.counts[artifact_kind(blob.data) or ("table" if table_file(path) else "other")] += 1
        self.diagnostic_transform()
        self.digest_closure()
        self.review_transform()
        self.ui_transform()
        self.fixture_commits()
        self.digest_closure()
        self.release_transform()
        # Per-case exceptions are final bytes. A negative fixture may restore
        # another case's old valid bytes; feeding that edge back globally cycles.
        self.apply_rulings()
        return self

    def review_relations(self, actual: dict[str, Blob]) -> list[str]:
        problems = []
        for path, old in self.base.items():
            if not path.endswith("/audit/adjudication.tsv") or path not in actual:
                continue
            manifest = path.removesuffix("adjudication.tsv") + "review-manifest.tsv"
            if manifest not in self.base and "/after/" in manifest:
                manifest = manifest.replace("/after/", "/tree/", 1)
            if manifest not in self.base or manifest not in actual:
                continue
            old_manifest = review_row_digests(self.base[manifest].data)
            new_manifest = review_row_digests(actual[manifest].data)
            old_rows = [line.split(b"\t") for line in old.data.splitlines() if not line.startswith(b"#")]
            new_rows = [line.split(b"\t") for line in actual[path].data.splitlines() if not line.startswith(b"#")]
            for old_row, new_row in zip(old_rows, new_rows):
                if len(old_row) != 7 or len(new_row) != 7 or old_row[0] != new_row[0]:
                    continue
                docid = old_row[0]
                if docid in old_manifest and docid in new_manifest:
                    was_current = old_row[1] == old_manifest[docid]
                    is_current = new_row[1] == new_manifest[docid]
                    if was_current != is_current:
                        problems.append(f"{path}: review current/outdated relation changed for {docid!r}")
        return problems

    def compare(self, actual: dict[str, Blob]) -> list[str]:
        expected = self.release_view(actual)
        problems = self.review_relations(actual)
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


def order_goal(data: bytes) -> bool:
    if not data.startswith(b"guideline_order(") or not data.endswith(b")"):
        return False
    try:
        return len(split_args(data[len(b"guideline_order("):-1])) == 4
    except ValueError:
        return False


def strip_order_goals(data: bytes) -> tuple[bytes | None, list[bytes]]:
    if order_goal(data):
        return None, [data]
    if data.startswith(b"','(") and data.endswith(b")"):
        fields = split_args(data[4:-1])
        if len(fields) != 2:
            raise InputError("order ruling: malformed conjunction")
        left, lg = strip_order_goals(fields[0])
        right, rg = strip_order_goals(fields[1])
        if left is None:
            return right, lg + rg
        if right is None:
            return left, lg + rg
        return b"','(" + left + b"," + right + b")", lg + rg
    return data, []


def check_order_edit(before: bytes, after: bytes, reason: str, path: str):
    heads, goals, bodies = [], [], []
    kind = artifact_kind(before)
    if kind == "doc":
        old = before.splitlines(keepends=True)
        at = 0
        for line in after.splitlines(keepends=True):
            if at < len(old) and line == old[at]:
                at += 1
                continue
            if not line.endswith(b".\n"):
                raise InputError(f"{path}: order ruling alters a non-order byte")
            head, separator, body = line[:-2].partition(b" :- ")
            if not order_goal(head):
                raise InputError(f"{path}: order ruling must insert complete guideline_order clause lines")
            heads.append(head)
            if separator:
                bodies.append(body)
        if at != len(old):
            raise InputError(f"{path}: order ruling removes or alters existing bytes")
    elif kind == "query":
        pattern = re.compile(rb"(?m)^'\$guideline_query_projection'\(([^\n]*)\)\.\n")
        old, new = pattern.search(before), pattern.search(after)
        if old is None or new is None or before[:old.start()] != after[:new.start()] or before[old.end():] != after[new.end():]:
            raise InputError(f"{path}: order ruling alters query envelope or projection count")
        old_fields, new_fields = split_args(old[1]), split_args(new[1])
        if len(old_fields) != 2 or len(new_fields) != 2 or old_fields[1] != new_fields[1]:
            raise InputError(f"{path}: order ruling alters answer manifest")
        if not all(f.startswith(b"goal(") and f.endswith(b")") for f in (old_fields[0], new_fields[0])):
            raise InputError(f"{path}: order ruling goal wrapper")
        stripped, goals = strip_order_goals(new_fields[0][5:-1])
        if stripped != old_fields[0][5:-1]:
            raise InputError(f"{path}: order ruling alters a non-order goal byte")
    else:
        raise InputError(f"{path}: order ruling requires a document or projection")
    if not (heads or goals) or not all(label in reason for label in ("heads=", "goals=", "bodies=")):
        raise InputError(f"{path}: order reason must name heads=, goals=, bodies= (none for empty)")
    for term in heads + goals + bodies:
        if term.decode("utf-8") not in reason:
            raise InputError(f"{path}: order reason omits term {term!r}")


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
    extra = "tests/v1/c1-oracle-unlisted/input.pl.in"
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
    while (path := f"tests/v1/{case}/c1-oracle-authority-{index}.in") in oracle.expected or path in oracle.base_origin:
        index += 1
    base_item = {"family": "tests/v1", "item": case + "/" + probe, "path": path, "mode": "new",
                 "edits": [{"old": b"", "new": b"plant\n", "count": 1}], "reason": "new: invalid base-item redeclaration"}
    items = {"/".join(row[:2]) for row in parse_tsv(oracle.expected[manifest].data)}
    index = 0
    while (item := f"c1-oracle-new-{index}/target") in items or oracle.base_item_exists("tests/v1", item) or ("tests/v1", item) in oracle.dispositions:
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
        print("c1-churn: usage: c1-churn.py <base-commit> <repo-root>")
        return 1
    try:
        root = Path(argv[2]).resolve(strict=True)
        dispositions = load_dispositions((root / DISPOSITIONS).read_bytes())
        rules_file = root / RULINGS
        rulings = load_rulings(rules_file.read_bytes()) if rules_file.exists() else []
        oracle = Oracle(base_blobs(root, argv[1]), dispositions, rulings)
        oracle.validate_dispositions()
        oracle.build()
        try:
            tests = self_test(oracle)
        except Exception as error:
            print("c1-churn: self-test failed: " + str(error).replace("\n", "\\n").replace("\r", "\\r"))
            return 2
        actual, _ = working_blobs(root)
        problems = oracle.compare(actual)
        if problems:
            for problem in problems:
                print("c1-churn: " + problem.replace("\n", "\\n").replace("\r", "\\r"))
            return 1
        changed = sum(oracle.base.get(p) != b for p, b in actual.items()) + len(oracle.base.keys() - actual.keys())
        print(f"c1-churn: ok {len(actual)} artifacts; {changed} changed paths; {len(dispositions)} dispositions; {len(oracle.new_items)} new coverage items; self-test {len(tests)}/8; 0 unpredicted changes")
        return 0
    except (InputError, OSError, UnicodeError) as error:
        print("c1-churn: " + str(error).replace("\n", "\\n").replace("\r", "\\r"))
        return 1


if __name__ == "__main__":
    sys.exit(main(sys.argv))
