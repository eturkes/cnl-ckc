import argparse
import hashlib
import os
import pathlib
import shutil
import subprocess
import sys
def fail(category, detail):
    safe_detail = detail.replace("\n", "\\n")
    safe_detail = safe_detail.replace("\r", "\\r")
    print("pipeline: " + category + ": " + safe_detail, file=sys.stderr)
    raise SystemExit(2)
def sha256_hex(data):
    digest = hashlib.sha256(data)
    hex_text = digest.hexdigest()
    return hex_text
def cleanup_and_fail(staging_path, category, detail):
    shutil.rmtree(staging_path)
    fail(category, detail)
def relay_failure(staging_path, result):
    shutil.rmtree(staging_path)
    sys.stderr.buffer.write(result.stderr)
    raise SystemExit(result.returncode)
def preflight(ape_path, docs_path, out_path):
    if not ape_path.is_dir():
        fail("ape-tree", "not a directory: " + str(ape_path))
    if not docs_path.is_dir():
        fail("docs-dir", "not a directory: " + str(docs_path))
    out_exists = out_path.exists()
    out_symlink = out_path.is_symlink()
    if out_exists:
        fail("out-dir", "already exists: " + str(out_path))
    if out_symlink:
        fail("out-dir", "already exists: " + str(out_path))
    parent = out_path.parent
    if not parent.is_dir():
        fail("out-dir", "parent is not a directory: " + str(parent))
    staging_prefix = out_path.name + ".tmp."
    parent_entries = sorted(parent.iterdir())
    for entry in parent_entries:
        entry_name = entry.name
        if entry_name.startswith(staging_prefix):
            fail("staging", "stale staging: " + entry_name)
    swipl = os.environ.get("SWIPL", "swipl")
    swipl_executable = shutil.which(swipl)
    if swipl_executable == None:
        fail("swipl-exec", "not executable: " + swipl)
    return swipl_executable
def run_front_end(staging_path, ape_path, docs_path, front_path):
    python_executable = sys.executable
    ape_text = str(ape_path)
    docs_text = str(docs_path)
    front_text = str(front_path)
    command = [python_executable, "-P", "tools/ace_front_end.py", ape_text, docs_text, front_text]
    result = subprocess.run(command, capture_output=True)
    if result.returncode != 0:
        relay_failure(staging_path, result)
    if result.stderr:
        cleanup_and_fail(staging_path, "child-stderr", "non-empty stderr from ace front end")
def front_docids(staging_path, front_path):
    front_regular = front_path.is_dir()
    front_symlink = front_path.is_symlink()
    if not front_regular:
        cleanup_and_fail(staging_path, "front-out", "front output is not a directory")
    if front_symlink:
        cleanup_and_fail(staging_path, "front-out", "front output is not a directory")
    docids = []
    found_manifest = False
    entries = sorted(front_path.iterdir())
    for entry in entries:
        entry_name = entry.name
        regular = entry.is_file()
        symlink = entry.is_symlink()
        if not regular:
            cleanup_and_fail(staging_path, "front-out", "entry is not a regular file: " + entry_name)
        if symlink:
            cleanup_and_fail(staging_path, "front-out", "entry is not a regular file: " + entry_name)
        if entry_name == "manifest.pl":
            found_manifest = True
        else:
            if not entry_name.endswith(".drs.pl"):
                cleanup_and_fail(staging_path, "front-out", "unsupported entry: " + entry_name)
            docid = entry_name.removesuffix(".drs.pl")
            docids.append(docid)
    if not found_manifest:
        cleanup_and_fail(staging_path, "front-out", "missing manifest.pl")
    if not docids:
        cleanup_and_fail(staging_path, "front-out", "no .drs.pl documents")
    return sorted(docids)
def stage_command(swipl_executable, stage):
    command = [swipl_executable, "-q", "-f", "none", "-F", "none", "-s", "src/prolog/ir_tool.pl", "-g", "main", "-t", "halt(9)", "--", stage]
    return command
def run_transform(staging_path, swipl_executable, stage, docid, input_path, output_path):
    input_bytes = input_path.read_bytes()
    command = stage_command(swipl_executable, stage)
    result = subprocess.run(command, input=input_bytes, capture_output=True)
    if result.returncode != 0:
        relay_failure(staging_path, result)
    if result.stderr:
        cleanup_and_fail(staging_path, "stage-stderr", "non-empty stderr for stage: " + stage + " document: " + docid)
    if not result.stdout:
        cleanup_and_fail(staging_path, "stage-stdout", "empty stdout for stage: " + stage + " document: " + docid)
    output_path.write_bytes(result.stdout)
def run_validate(staging_path, swipl_executable, docid, input_path):
    input_bytes = input_path.read_bytes()
    stage = "validate"
    command = stage_command(swipl_executable, stage)
    result = subprocess.run(command, input=input_bytes, capture_output=True)
    if result.returncode != 0:
        relay_failure(staging_path, result)
    if result.stdout:
        cleanup_and_fail(staging_path, "stage-stdout", "non-empty stdout for stage: validate document: " + docid)
    if result.stderr:
        cleanup_and_fail(staging_path, "stage-stderr", "non-empty stderr for stage: validate document: " + docid)
def run_chain(staging_path, swipl_executable, front_path, chain_path, docids):
    for docid in docids:
        drs_path = front_path.joinpath(docid + ".drs.pl")
        ir_path = chain_path.joinpath(docid + ".ir.pl")
        program_path = chain_path.joinpath(docid + ".program.pl")
        result_path = chain_path.joinpath(docid + ".result.pl")
        run_transform(staging_path, swipl_executable, "lower", docid, drs_path, ir_path)
        run_validate(staging_path, swipl_executable, docid, ir_path)
        run_transform(staging_path, swipl_executable, "compile", docid, ir_path, program_path)
        run_transform(staging_path, swipl_executable, "run", docid, program_path, result_path)
def write_manifest(staging_path, front_path, chain_path, docids):
    manifest_text = "cnl_pipeline_manifest(1).\n"
    front_manifest_path = front_path.joinpath("manifest.pl")
    for docid in docids:
        drs_path = front_path.joinpath(docid + ".drs.pl")
        ir_path = chain_path.joinpath(docid + ".ir.pl")
        program_path = chain_path.joinpath(docid + ".program.pl")
        result_path = chain_path.joinpath(docid + ".result.pl")
        drs_bytes = drs_path.read_bytes()
        ir_bytes = ir_path.read_bytes()
        program_bytes = program_path.read_bytes()
        result_bytes = result_path.read_bytes()
        front_manifest_bytes = front_manifest_path.read_bytes()
        drs_hex = sha256_hex(drs_bytes)
        ir_hex = sha256_hex(ir_bytes)
        program_hex = sha256_hex(program_bytes)
        result_hex = sha256_hex(result_bytes)
        front_manifest_hex = sha256_hex(front_manifest_bytes)
        manifest_line = "document(docid('" + docid + "'),drs_sha256('" + drs_hex + "'),ir_sha256('" + ir_hex + "'),program_sha256('" + program_hex + "'),result_sha256('" + result_hex + "'),front_manifest_sha256('" + front_manifest_hex + "')).\n"
        manifest_text = manifest_text + manifest_line
    manifest_bytes = manifest_text.encode("utf-8")
    manifest_path = staging_path.joinpath("manifest.pl")
    manifest_path.write_bytes(manifest_bytes)
def published_relpaths(docids):
    relpaths = ["front/manifest.pl", "manifest.pl"]
    for docid in docids:
        relpaths.append("front/" + docid + ".drs.pl")
        relpaths.append("chain/" + docid + ".ir.pl")
        relpaths.append("chain/" + docid + ".program.pl")
        relpaths.append("chain/" + docid + ".result.pl")
    return sorted(relpaths)
def publish(staging_path, out_path, docids):
    relpaths = published_relpaths(docids)
    os.replace(staging_path, out_path)
    for relpath in relpaths:
        target = out_path.joinpath(relpath)
        print("pipeline: wrote " + str(target))
    print("pipeline: ok " + str(len(docids)) + " documents")
ir_tool_path = pathlib.Path("src/prolog/ir_tool.pl")
if not (ir_tool_path.is_file()):
    raise AssertionError("requirement failed")
front_end_path = pathlib.Path("tools/ace_front_end.py")
if not (front_end_path.is_file()):
    raise AssertionError("requirement failed")
parser = argparse.ArgumentParser(prog="pipeline")
parser.add_argument("ape_tree_dir")
parser.add_argument("docs_dir")
parser.add_argument("out_dir")
args = parser.parse_args()
ape_path = pathlib.Path(args.ape_tree_dir)
docs_path = pathlib.Path(args.docs_dir)
out_path = pathlib.Path(args.out_dir)
swipl_executable = preflight(ape_path, docs_path, out_path)
pid_text = str(os.getpid())
staging_name = out_path.name + ".tmp." + pid_text
parent = out_path.parent
staging_path = parent.joinpath(staging_name)
staging_path.mkdir()
front_path = staging_path.joinpath("front")
run_front_end(staging_path, ape_path, docs_path, front_path)
docids = front_docids(staging_path, front_path)
chain_path = staging_path.joinpath("chain")
chain_path.mkdir()
run_chain(staging_path, swipl_executable, front_path, chain_path, docids)
write_manifest(staging_path, front_path, chain_path, docids)
publish(staging_path, out_path, docids)
