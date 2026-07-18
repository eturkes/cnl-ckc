import argparse
import hashlib
import os
import pathlib
import subprocess
import sys
def fail(category, detail):
    safe_detail = detail.replace("\n", "\\n")
    safe_detail = safe_detail.replace("\r", "\\r")
    print("ace-front-end: " + category + ": " + safe_detail, file=sys.stderr)
    raise SystemExit(2)
def sha256_hex(data):
    digest = hashlib.sha256(data)
    hex_text = digest.hexdigest()
    return hex_text
def valid_docid(docid):
    allowed = set("abcdefghijklmnopqrstuvwxyz0123456789-")
    chars = set(docid)
    subset = chars.issubset(allowed)
    leading_dash = docid.startswith("-")
    if not docid:
        return False
    if leading_dash:
        return False
    return subset
def collect_documents(docs_path):
    if not docs_path.is_dir():
        fail("docs-dir", "not a directory: " + str(docs_path))
    ace_paths = {}
    ulex_paths = {}
    ace_docids = []
    ulex_docids = []
    entries = sorted(docs_path.iterdir())
    for entry in entries:
        entry_name = entry.name
        regular = entry.is_file()
        symlink = entry.is_symlink()
        if not regular:
            fail("docs-dir", "entry is not a regular file: " + entry_name)
        if symlink:
            fail("docs-dir", "entry is not a regular file: " + entry_name)
        is_ace = entry_name.endswith(".ace")
        is_ulex = entry_name.endswith(".ulex")
        if not is_ace:
            if not is_ulex:
                fail("docs-dir", "unsupported entry: " + entry_name)
        if is_ace:
            docid = entry_name.removesuffix(".ace")
            if not valid_docid(docid):
                fail("docid", "invalid document id: " + docid)
            ace_paths.update({docid: entry})
            ace_docids.append(docid)
        else:
            docid = entry_name.removesuffix(".ulex")
            if not valid_docid(docid):
                fail("docid", "invalid document id: " + docid)
            ulex_paths.update({docid: entry})
            ulex_docids.append(docid)
    if not ace_docids:
        fail("docs-dir", "no .ace documents")
    for ulex_docid in sorted(ulex_docids):
        if not (ulex_docid in ace_docids):
            fail("docs-dir", "orphan user lexicon: " + ulex_docid + ".ulex")
    return [ace_paths, ulex_paths, sorted(ace_docids)]
def check_out_dir(out_path):
    out_exists = out_path.exists()
    out_symlink = out_path.is_symlink()
    if out_exists:
        fail("out-dir", "already exists: " + str(out_path))
    if out_symlink:
        fail("out-dir", "already exists: " + str(out_path))
    parent = out_path.parent
    if not parent.is_dir():
        fail("out-dir", "parent is not a directory: " + str(parent))
def adapter_command(swipl, ape_tree, ulex_path):
    command = [swipl, "-q", "-f", "none", "-F", "none", "-s", "src/prolog/adapter.pl", "-g", "main", "-t", "halt(9)", "--", ape_tree]
    if ulex_path != None:
        command.append(str(ulex_path))
    return command
def run_document(swipl, ape_tree, docid, source_bytes, ulex_path):
    command = adapter_command(swipl, ape_tree, ulex_path)
    result = subprocess.run(command, input=source_bytes, capture_output=True)
    if result.returncode != 0:
        sys.stderr.buffer.write(result.stderr)
        raise SystemExit(result.returncode)
    if not result.stdout:
        fail("adapter-stdout", "empty stdout for document: " + docid)
    return result.stdout
def build_record(docid, source_bytes, ulex_path, drs_bytes):
    source_hex = sha256_hex(source_bytes)
    ulex_term = "none"
    if ulex_path != None:
        ulex_bytes = ulex_path.read_bytes()
        ulex_hex = sha256_hex(ulex_bytes)
        ulex_term = "sha256('" + ulex_hex + "')"
    header = "ace_front_end_record(1).\ndocument(docid('" + docid + "'),source_sha256('" + source_hex + "'),ulex(" + ulex_term + ")).\n"
    header_bytes = header.encode("utf-8")
    return header_bytes + drs_bytes
def compile_documents(swipl, ape_tree, ace_paths, ulex_paths, docids):
    records = {}
    manifest_lines = []
    for docid in docids:
        ace_path = ace_paths.get(docid)
        ulex_path = ulex_paths.get(docid)
        source_bytes = ace_path.read_bytes()
        drs_bytes = run_document(swipl, ape_tree, docid, source_bytes, ulex_path)
        record_bytes = build_record(docid, source_bytes, ulex_path, drs_bytes)
        record_name = docid + ".drs.pl"
        records.update({record_name: record_bytes})
        source_hex = sha256_hex(source_bytes)
        record_hex = sha256_hex(record_bytes)
        manifest_line = "document(docid('" + docid + "'),source_sha256('" + source_hex + "'),record_sha256('" + record_hex + "')).\n"
        manifest_lines.append(manifest_line)
    manifest_text = "ace_front_end_manifest(1).\n"
    for manifest_line in manifest_lines:
        manifest_text = manifest_text + manifest_line
    manifest_bytes = manifest_text.encode("utf-8")
    records.update({"manifest.pl": manifest_bytes})
    return records
def write_outputs(out_path, records, document_count):
    out_path.mkdir()
    for filename in sorted(records):
        payload = records.get(filename)
        target = out_path.joinpath(filename)
        target.write_bytes(payload)
        print("ace-front-end: wrote " + str(target))
    print("ace-front-end: ok " + str(document_count) + " documents")
adapter_path = pathlib.Path("src/prolog/adapter.pl")
if not (adapter_path.is_file()):
    raise AssertionError("requirement failed")
parser = argparse.ArgumentParser(prog="ace_front_end")
parser.add_argument("ape_tree_dir")
parser.add_argument("docs_dir")
parser.add_argument("out_dir")
args = parser.parse_args()
docs_path = pathlib.Path(args.docs_dir)
out_path = pathlib.Path(args.out_dir)
collected = collect_documents(docs_path)
ace_paths = collected.pop(0)
ulex_paths = collected.pop(0)
docids = collected.pop(0)
check_out_dir(out_path)
swipl = os.environ.get("SWIPL", "swipl")
records = compile_documents(swipl, args.ape_tree_dir, ace_paths, ulex_paths, docids)
write_outputs(out_path, records, len(docids))
