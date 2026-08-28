import ast
import datetime
import gzip
import hashlib
import io
import os
import pathlib
import re
import shutil
import signal
import subprocess
import sys
import tarfile
import tempfile
import time
def fail(category, detail):
    safe_detail = detail.replace("\n", "\\n")
    safe_detail = safe_detail.replace("\r", "\\r")
    print("goal: " + category + ": " + safe_detail, file=sys.stderr)
    raise SystemExit(2)
def violation(category, detail):
    safe_detail = detail.replace("\n", "\\n")
    safe_detail = safe_detail.replace("\r", "\\r")
    print("goal: " + category + ": " + safe_detail)
    raise SystemExit(1)
def cleanup_and_fail(scratch_path, category, detail):
    shutil.rmtree(scratch_path)
    fail(category, detail)
def cleanup_violation(scratch_path, category, detail):
    shutil.rmtree(scratch_path)
    violation(category, detail)
def relay_failure(scratch_path, result):
    shutil.rmtree(scratch_path)
    sys.stderr.buffer.write(result.stderr)
    raise SystemExit(result.returncode)
def valid_docid(docid):
    allowed = set("abcdefghijklmnopqrstuvwxyz0123456789-")
    chars = set(docid)
    subset = chars.issubset(allowed)
    leading_dash = docid.startswith("-")
    if not docid:
        return False
    if leading_dash:
        return False
    if len(docid) > 250:
        return False
    return subset
def resolve_swipl():
    swipl = os.environ.get("SWIPL", "swipl")
    swipl_executable = shutil.which(swipl)
    if swipl_executable == None:
        fail("swipl-exec", "not executable: " + swipl)
    version_result = subprocess.run([swipl_executable, "--version"], capture_output=True)
    if version_result.returncode != 0:
        fail("swipl-version", "version probe failed: " + swipl_executable)
    version_text = version_result.stdout.decode("utf-8", errors="replace")
    version_tokens = version_text.split()
    version_value = ""
    if len(version_tokens) > 2:
        product_token = version_tokens.pop(0)
        label_token = version_tokens.pop(0)
        value_token = version_tokens.pop(0)
        if product_token == "SWI-Prolog":
            if label_token == "version":
                version_value = value_token
    if version_value != swipl_version_required:
        fail("swipl-version", "expected " + swipl_version_required + ", found: " + version_text.strip())
    return swipl_executable
def make_scratch():
    pid_text = str(os.getpid())
    scratch_path = pathlib.Path(".goal.tmp." + pid_text)
    if scratch_path.exists():
        fail("scratch", "already exists: " + str(scratch_path))
    scratch_path.mkdir()
    return scratch_path
swipl_wall_seconds = 300
def swipl_run_walled(command, input_bytes, wall_seconds):
    proc = subprocess.Popen(command, stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, start_new_session=True)
    timed_out = False
    out_bytes = bytes([])
    err_bytes = bytes([])
    try:
        comm = proc.communicate(input=input_bytes, timeout=wall_seconds)
        comm_copy = list(comm)
        out_bytes = comm_copy.pop(0)
        err_bytes = comm_copy.pop(0)
    except subprocess.TimeoutExpired:
        timed_out = True
        try:
            os.killpg(os.getpgid(proc.pid), signal.SIGKILL)
        except ProcessLookupError:
            timed_out = True
        comm = proc.communicate()
        comm_copy = list(comm)
        out_bytes = comm_copy.pop(0)
        err_bytes = comm_copy.pop(0)
    result = subprocess.CompletedProcess(command, proc.returncode, out_bytes, err_bytes)
    return [timed_out, result]
def bounded_swipl_run(scratch_path, label, command, input_bytes):
    pair = swipl_run_walled(command, input_bytes, swipl_wall_seconds)
    pair_copy = list(pair)
    timed_out = pair_copy.pop(0)
    result = pair_copy.pop(0)
    if timed_out:
        cleanup_and_fail(scratch_path, "swipl-timeout", label + " exceeded " + str(swipl_wall_seconds) + "s wall clock")
    return result
def stage_ape(scratch_path, swipl_executable):
    compiler_source = pathlib.Path("vendor/ape/prolog/ace_to_pl.pl")
    if not compiler_source.is_file():
        cleanup_and_fail(scratch_path, "compiler-source", "missing: " + str(compiler_source))
    stage_path = scratch_path.joinpath("ape-stage")
    shutil.copytree("vendor/ape", stage_path)
    stage_clex = stage_path.joinpath("prolog", "lexicon", "clex_lexicon.pl")
    shutil.copy(clex_path_text, stage_clex)
    parser_dir = stage_path.joinpath("prolog", "parser")
    build_goal = "working_directory(_, '" + parser_dir.as_posix() + "'), [fit_to_plp], halt."
    command = [swipl_executable, "-O", "-f", "none", "-F", "none", "-g", build_goal, "-t", "halt"]
    result = bounded_swipl_run(scratch_path, "ape-stage build", command, None)
    if result.returncode != 0:
        relay_failure(scratch_path, result)
    grammar_path = parser_dir.joinpath("grammar.plp")
    if not grammar_path.is_file():
        cleanup_and_fail(scratch_path, "ape-stage", "missing grammar.plp after build")
    return stage_path
def compiler_command(swipl_executable, stage_path, tail_args):
    compiler_path = stage_path.joinpath("prolog", "ace_to_pl.pl")
    command = [swipl_executable, "-q", "-f", "none", "-F", "none", "-s", str(compiler_path), "-g", "main", "-t", "halt(9)", "--"]
    for tail_arg in tail_args:
        command.append(tail_arg)
    return command
def compile_doc(scratch_path, swipl_executable, stage_path, docid, ace_path, lexicon_path, extra_args):
    ace_bytes = ace_path.read_bytes()
    tail_args = [str(stage_path), docid]
    if lexicon_path != None:
        tail_args.append(str(lexicon_path))
    for extra_arg in extra_args:
        tail_args.append(extra_arg)
    command = compiler_command(swipl_executable, stage_path, tail_args)
    result = bounded_swipl_run(scratch_path, "compile " + docid, command, ace_bytes)
    if result.returncode != 0:
        relay_failure(scratch_path, result)
    if result.stderr:
        cleanup_and_fail(scratch_path, "compiler-stderr", "non-empty stderr for document: " + docid)
    if not result.stdout:
        cleanup_and_fail(scratch_path, "compiler-stdout", "empty stdout for document: " + docid)
    newline_bytes = bytes([10])
    final_newline = result.stdout.endswith(newline_bytes)
    if not final_newline:
        cleanup_and_fail(scratch_path, "compiler-stdout", "missing final newline for document: " + docid)
    return result.stdout
def check_doc_load(scratch_path, swipl_executable, stage_path, pl_path):
    tail_args = ["check", str(pl_path)]
    command = compiler_command(swipl_executable, stage_path, tail_args)
    result = bounded_swipl_run(scratch_path, "check-load " + str(pl_path), command, None)
    if result.returncode != 0:
        relay_failure(scratch_path, result)
    if result.stdout:
        cleanup_and_fail(scratch_path, "check-stdout", "non-empty stdout for: " + str(pl_path))
    if result.stderr:
        cleanup_and_fail(scratch_path, "check-stderr", "non-empty stderr for: " + str(pl_path))
def collect_guideline(guideline_path):
    ace_dir = guideline_path.joinpath("ace")
    ace_symlink = ace_dir.is_symlink()
    if ace_symlink:
        fail("guideline", "ace directory is a symlink: " + str(ace_dir))
    if not ace_dir.is_dir():
        fail("guideline", "missing ace directory: " + str(ace_dir))
    ace_paths = {}
    docids = []
    for entry in sorted(ace_dir.iterdir()):
        entry_name = entry.name
        regular = entry.is_file()
        symlink = entry.is_symlink()
        if not regular:
            fail("guideline", "entry is not a regular file: " + entry_name)
        if symlink:
            fail("guideline", "entry is not a regular file: " + entry_name)
        if not entry_name.endswith(".ace"):
            fail("guideline", "unsupported ace entry: " + entry_name)
        docid = entry_name.removesuffix(".ace")
        if not valid_docid(docid):
            fail("docid", "invalid document id: " + docid)
        ace_paths.update({docid: entry})
        docids.append(docid)
    if not docids:
        fail("guideline", "no .ace documents in: " + str(ace_dir))
    lexicon_path = guideline_path.joinpath("lexicon.ulex")
    lexicon_symlink = lexicon_path.is_symlink()
    if lexicon_symlink:
        fail("guideline", "lexicon is a symlink: " + str(lexicon_path))
    lexicon_present = lexicon_path.is_file()
    if not lexicon_present:
        lexicon_path = None
    return [ace_paths, sorted(docids), lexicon_path]
def check_source_record(guideline_path):
    readme_path = guideline_path.joinpath("README.md")
    readme_symlink = readme_path.is_symlink()
    if readme_symlink:
        violation("source-record", "README.md is a symlink under: " + str(guideline_path))
    if not readme_path.is_file():
        violation("source-record", "missing README.md under: " + str(guideline_path))
    source_dir = guideline_path.joinpath("source")
    source_symlink = source_dir.is_symlink()
    if source_symlink:
        violation("source-record", "source is a symlink under: " + str(guideline_path))
    if not source_dir.is_dir():
        violation("source-record", "missing source directory under: " + str(guideline_path))
    record_count = 0
    for entry in sorted(source_dir.iterdir()):
        symlink = entry.is_symlink()
        if symlink:
            violation("source-record", "symlink under source: " + entry.name)
        regular = entry.is_file()
        if regular:
            record_count = record_count + 1
    if record_count == 0:
        violation("source-record", "no source files under: " + str(guideline_path))
def check_pl_inventory(guideline_path, docids):
    pl_dir = guideline_path.joinpath("pl")
    pl_symlink = pl_dir.is_symlink()
    if pl_symlink:
        violation("pl-dir", "is a symlink: " + str(pl_dir))
    if not pl_dir.is_dir():
        violation("missing-pl", str(pl_dir))
    expected_names = []
    for docid in docids:
        expected_names.append(docid + ".pl")
    actual_names = []
    for entry in sorted(pl_dir.iterdir()):
        entry_name = entry.name
        regular = entry.is_file()
        symlink = entry.is_symlink()
        if not regular:
            violation("pl-entry", "not a regular file: " + entry_name)
        if symlink:
            violation("pl-entry", "not a regular file: " + entry_name)
        actual_names.append(entry_name)
    if sorted(expected_names) != actual_names:
        violation("pl-inventory", "committed pl/ does not match ace/ document set")
def compiled_pl_path(tracked):
    parts = tracked.split("/")
    if len(parts) != 4:
        return False
    root_part = parts.pop(0)
    id_part = parts.pop(0)
    dir_part = parts.pop(0)
    file_part = parts.pop(0)
    if root_part != "guidelines":
        return False
    if not valid_docid(id_part):
        return False
    if dir_part != "pl":
        return False
    if not file_part.endswith(".pl"):
        return False
    stem = file_part.removesuffix(".pl")
    return valid_docid(stem)
def fixture_pl_path(tracked):
    parts = tracked.split("/")
    if len(parts) != 9:
        return False
    root_part = parts.pop(0)
    suite_part = parts.pop(0)
    color_part = parts.pop(0)
    case_part = parts.pop(0)
    tree_part = parts.pop(0)
    guidelines_part = parts.pop(0)
    id_part = parts.pop(0)
    dir_part = parts.pop(0)
    file_part = parts.pop(0)
    if root_part != "tests":
        return False
    if suite_part != "ui":
        return False
    color_ok = False
    if color_part == "red":
        color_ok = True
    if color_part == "green":
        color_ok = True
    if not color_ok:
        return False
    corpus_ok = False
    if tree_part == "tree":
        corpus_ok = True
    if tree_part == "worktree":
        corpus_ok = True
    if not corpus_ok:
        return False
    if guidelines_part != "guidelines":
        return False
    if dir_part != "pl":
        return False
    return file_part.endswith(".pl")
def query_artifact_pl_path(tracked):
    parts = tracked.split("/")
    if len(parts) != 5:
        return False
    root_part = parts.pop(0)
    id_part = parts.pop(0)
    queries_part = parts.pop(0)
    dir_part = parts.pop(0)
    file_part = parts.pop(0)
    if root_part != "guidelines":
        return False
    if not valid_docid(id_part):
        return False
    if queries_part != "queries":
        return False
    dir_ok = False
    if dir_part == "pl":
        dir_ok = True
    if dir_part == "answers":
        dir_ok = True
    if dir_part == "traces":
        dir_ok = True
    if not dir_ok:
        return False
    if not file_part.endswith(".pl"):
        return False
    stem = file_part.removesuffix(".pl")
    return valid_docid(stem)
def queries_fixture_pl_path(tracked):
    parts = tracked.split("/")
    part_count = len(parts)
    count_ok = False
    if part_count == 6:
        count_ok = True
    if part_count == 9:
        count_ok = True
    if part_count == 10:
        count_ok = True
    if not count_ok:
        return False
    root_part = parts.pop(0)
    suite_part = parts.pop(0)
    color_part = parts.pop(0)
    case_part = parts.pop(0)
    if root_part != "tests":
        return False
    if suite_part != "queries":
        return False
    color_ok = False
    if color_part == "red":
        color_ok = True
    if color_part == "green":
        color_ok = True
    if not color_ok:
        return False
    if not valid_docid(case_part):
        return False
    if part_count == 6:
        pin_part = parts.pop(0)
        file_part = parts.pop(0)
        pin_ok = False
        if pin_part == "answers-golden":
            pin_ok = True
        if pin_part == "traces-golden":
            pin_ok = True
        if not pin_ok:
            return False
        if not file_part.endswith(".pl"):
            return False
        stem = file_part.removesuffix(".pl")
        return valid_docid(stem)
    tree_part = parts.pop(0)
    guidelines_part = parts.pop(0)
    id_part = parts.pop(0)
    if tree_part != "tree":
        return False
    if guidelines_part != "guidelines":
        return False
    if not valid_docid(id_part):
        return False
    if part_count == 9:
        dir_part = parts.pop(0)
        file_part = parts.pop(0)
        if dir_part != "pl":
            return False
        if not file_part.endswith(".pl"):
            return False
        stem = file_part.removesuffix(".pl")
        return valid_docid(stem)
    queries_part = parts.pop(0)
    dir_part = parts.pop(0)
    file_part = parts.pop(0)
    if queries_part != "queries":
        return False
    dir_ok = False
    if dir_part == "pl":
        dir_ok = True
    if dir_part == "answers":
        dir_ok = True
    if dir_part == "traces":
        dir_ok = True
    if not dir_ok:
        return False
    if not file_part.endswith(".pl"):
        return False
    stem = file_part.removesuffix(".pl")
    return valid_docid(stem)
def check_prolog_inventory():
    git_res = subprocess.run(["git", "ls-files", "--", "*.pl"], capture_output=True, check=True)
    tracked_text = git_res.stdout.decode("utf-8")
    for tracked in sorted(tracked_text.splitlines()):
        vendored = tracked.startswith("vendor/ape/")
        clex_base = tracked == "vendor/clex/clex_lexicon.pl"
        compiled = compiled_pl_path(tracked)
        fixture_pl = fixture_pl_path(tracked)
        query_artifact = query_artifact_pl_path(tracked)
        queries_fixture = queries_fixture_pl_path(tracked)
        if not vendored:
            if not clex_base:
                if not compiled:
                    if not fixture_pl:
                        if not query_artifact:
                            if not queries_fixture:
                                violation("prolog-inventory", "unauthorized tracked prolog: " + tracked)
def provenance_field(prov_lines, key):
    prefix = key + ": "
    value = ""
    for prov_line in prov_lines:
        starts = prov_line.startswith(prefix)
        if starts:
            rest = prov_line.removeprefix(prefix)
            value = rest.strip()
    return value
def provenance_list_field(prov_lines, key):
    value = provenance_field(prov_lines, key)
    items = []
    for part in value.split(","):
        item = part.strip()
        if item != "":
            items.append(item)
    return items
def license_requires_date(tree_prefix, license_id):
    gpl = license_id.startswith("GPL-")
    lgpl = license_id.startswith("LGPL-")
    agpl = license_id.startswith("AGPL-")
    if gpl:
        return True
    if lgpl:
        return True
    if agpl:
        return True
    if license_id == "Apache-2.0":
        return False
    violation("fork-notice", "unrecognized license in " + tree_prefix + ": " + license_id)
    return False
def fork_notice_scan(file_path):
    raw = file_path.read_bytes()
    text = raw.decode("utf-8", "replace")
    prominent = ""
    buried = ""
    seen = 0
    for line in text.splitlines():
        seen = seen + 1
        marks = line.count("Modified")
        forks = line.count("fork")
        hit = 0
        if marks > 0:
            if forks > 0:
                hit = 1
        if seen > 40:
            if hit > 0:
                if buried == "":
                    buried = line
        else:
            if hit > 0:
                if prominent == "":
                    prominent = line
    return [prominent, buried]
def notice_has_date(notice):
    hits = re.findall("[0-9]{4}-[0-9]{2}-[0-9]{2}", notice)
    count = len(hits)
    if count > 0:
        return True
    return False
def tracked_under(prefix):
    git_res = subprocess.run(["git", "ls-files", "--", prefix], capture_output=True, check=True)
    tracked_text = git_res.stdout.decode("utf-8")
    items = tracked_text.splitlines()
    return sorted(items)
def touched_since(import_commit, prefix):
    spec = import_commit + "..HEAD"
    log_res = subprocess.run(["git", "log", "--name-only", "--pretty=format:", spec, "--", prefix], capture_output=True, check=True)
    log_text = log_res.stdout.decode("utf-8")
    diff_res = subprocess.run(["git", "diff", "--name-only", "HEAD", "--", prefix], capture_output=True, check=True)
    diff_text = diff_res.stdout.decode("utf-8")
    combined = log_text + "\n" + diff_text
    touched = []
    for log_line in combined.splitlines():
        item = log_line.strip()
        if item != "":
            present = item in touched
            if not present:
                touched.append(item)
    return touched
def check_pristine_tree(tree_path, tree_prefix, tracked, first_party):
    manifest_path = tree_path.joinpath("MANIFEST.sha256")
    if not manifest_path.is_file():
        violation("fork-notice", "pristine tree lacks MANIFEST.sha256: " + tree_prefix)
    manifest_bytes = manifest_path.read_bytes()
    manifest_text = manifest_bytes.decode("utf-8")
    manifest_map = {}
    for manifest_line in manifest_text.splitlines():
        if manifest_line:
            line_parts = manifest_line.split("  ", 1)
            if len(line_parts) != 2:
                violation("fork-notice", "malformed manifest row in " + tree_prefix + ": " + manifest_line)
            digest_hex = line_parts.pop(0)
            rel_name = line_parts.pop(0)
            rel_clean = rel_name.removeprefix("./")
            full_name = tree_prefix + "/" + rel_clean
            manifest_map.update({full_name: digest_hex})
    for tracked_path in tracked:
        exempt = tracked_path in first_party
        if not exempt:
            file_path = pathlib.Path(tracked_path)
            scan = fork_notice_scan(file_path)
            prominent = scan.pop(0)
            buried = scan.pop(0)
            if prominent != "":
                violation("fork-notice", "pristine vendored file carries a change notice: " + tracked_path)
            if buried != "":
                violation("fork-notice", "pristine vendored file carries a change notice: " + tracked_path)
            listed = tracked_path in manifest_map
            if not listed:
                violation("fork-notice", "pristine file missing from manifest: " + tracked_path)
            expected_digest = manifest_map.get(tracked_path)
            file_bytes = file_path.read_bytes()
            digest_value = hashlib.sha256(file_bytes)
            actual_digest = digest_value.hexdigest()
            if actual_digest != expected_digest:
                violation("fork-notice", "pristine file differs from manifest digest: " + tracked_path)
    for full_name in manifest_map:
        manifest_tracked = full_name in tracked
        if not manifest_tracked:
            violation("fork-notice", "manifest row without tracked file: " + full_name)
        manifest_first_party = full_name in first_party
        if manifest_first_party:
            violation("fork-notice", "manifest row names first-party file: " + full_name)
    return 0
def check_vendor_tree(tree_path):
    tree_prefix = tree_path.as_posix()
    prov_path = tree_path.joinpath("PROVENANCE")
    if not prov_path.is_file():
        violation("fork-notice", "missing PROVENANCE: " + tree_prefix)
    prov_bytes = prov_path.read_bytes()
    prov_text = prov_bytes.decode("utf-8")
    prov_lines = prov_text.splitlines()
    license_id = provenance_field(prov_lines, "License")
    if license_id == "":
        violation("fork-notice", "PROVENANCE states no License: " + tree_prefix)
    import_commit = provenance_field(prov_lines, "Import commit")
    if import_commit == "":
        violation("fork-notice", "PROVENANCE states no Import commit: " + tree_prefix)
    needs_date = license_requires_date(tree_prefix, license_id)
    tracked = tracked_under(tree_prefix)
    touched = touched_since(import_commit, tree_prefix)
    declared = provenance_list_field(prov_lines, "First-party files")
    first_party = []
    for relative in declared:
        declared_path = tree_prefix + "/" + relative
        declared_tracked = declared_path in tracked
        if not declared_tracked:
            violation("fork-notice", "declared first-party file is untracked: " + declared_path)
        first_party.append(declared_path)
    pristine_flag = provenance_field(prov_lines, "Pristine")
    if pristine_flag == "yes":
        pristine_count = check_pristine_tree(tree_path, tree_prefix, tracked, first_party)
        return pristine_count
    modified_count = 0
    for tracked_path in tracked:
        exempt = tracked_path in first_party
        if not exempt:
            modified = tracked_path in touched
            if modified:
                modified_count = modified_count + 1
            file_path = pathlib.Path(tracked_path)
            scan = fork_notice_scan(file_path)
            prominent = scan.pop(0)
            buried = scan.pop(0)
            if modified:
                if prominent == "":
                    if buried == "":
                        violation("fork-notice", "modified vendored file carries no change notice: " + tracked_path)
                    else:
                        violation("fork-notice", "change notice is not prominent: " + tracked_path)
                if needs_date:
                    dated = notice_has_date(prominent)
                    if not dated:
                        violation("fork-notice", "change notice states no date: " + tracked_path)
            else:
                if prominent != "":
                    violation("fork-notice", "unmodified vendored file carries a change notice: " + tracked_path)
    return modified_count
def check_fork_notices():
    vendor_root = pathlib.Path("vendor")
    if not vendor_root.is_dir():
        violation("fork-notice", "missing vendor directory")
    tree_count = 0
    modified_total = 0
    for entry in sorted(vendor_root.iterdir()):
        entry_name = entry.as_posix()
        if entry.is_symlink():
            violation("fork-notice", "vendor entry is a symlink: " + entry_name)
        if not entry.is_dir():
            violation("fork-notice", "vendor entry is not a directory: " + entry_name)
        tree_modified = check_vendor_tree(entry)
        tree_count = tree_count + 1
        modified_total = modified_total + tree_modified
    if tree_count == 0:
        violation("fork-notice", "no vendor trees found")
    if modified_total == 0:
        violation("fork-notice", "no modified vendored files found")
    print("goal: fork notices ok " + str(tree_count) + " trees " + str(modified_total) + " modified files")
def check_documents(scratch_path, swipl_executable, stage_path, guideline_path, ace_paths, docids, lexicon_path):
    pl_dir = guideline_path.joinpath("pl")
    payload_dir = scratch_path.joinpath("payloads")
    if not payload_dir.is_dir():
        payload_dir.mkdir()
    manifest_pairs = []
    for docid in docids:
        ace_path = ace_paths.get(docid)
        extra_args = []
        first_bytes = compile_doc(scratch_path, swipl_executable, stage_path, docid, ace_path, lexicon_path, extra_args)
        second_bytes = compile_doc(scratch_path, swipl_executable, stage_path, docid, ace_path, lexicon_path, extra_args)
        if first_bytes != second_bytes:
            cleanup_violation(scratch_path, "determinism", "two compiles differ for document: " + docid)
        committed_path = pl_dir.joinpath(docid + ".pl")
        committed_bytes = committed_path.read_bytes()
        if first_bytes != committed_bytes:
            cleanup_violation(scratch_path, "stale", "committed pl differs from fresh compile: " + str(committed_path))
        proof_args = ["proof"]
        first_payload = compile_doc(scratch_path, swipl_executable, stage_path, docid, ace_path, lexicon_path, proof_args)
        second_payload = compile_doc(scratch_path, swipl_executable, stage_path, docid, ace_path, lexicon_path, proof_args)
        if first_payload != second_payload:
            cleanup_violation(scratch_path, "determinism", "two proof runs differ for document: " + docid)
        payload_path = payload_dir.joinpath(docid + ".proof")
        payload_path.write_bytes(first_payload)
        manifest_pairs.append([committed_path, payload_path])
        check_doc_load(scratch_path, swipl_executable, stage_path, committed_path)
    manifest_ids = manifest_document_ids(manifest_pairs)
    if manifest_ids != docids:
        cleanup_violation(scratch_path, "aggregate-totality", "manifest documents differ from corpus documents: " + str(guideline_path))
    check_aggregate(scratch_path, swipl_executable, stage_path, manifest_pairs)
    query_counts = validate_queries(scratch_path, swipl_executable, stage_path, guideline_path, lexicon_path)
    queries_meter(guideline_path.name, query_counts)
    traces_meter(guideline_path.name, query_counts)
def manifest_document_ids(manifest_pairs):
    docid_list = []
    for manifest_pair in manifest_pairs:
        pair_copy = list(manifest_pair)
        pl_path = pair_copy.pop(0)
        pl_name = pl_path.name
        docid_list.append(pl_name.removesuffix(".pl"))
    return docid_list
def write_manifest(manifest_path, manifest_pairs):
    manifest_text = ""
    for manifest_pair in manifest_pairs:
        pair_copy = list(manifest_pair)
        pl_path = pair_copy.pop(0)
        payload_path = pair_copy.pop(0)
        manifest_text = manifest_text + str(pl_path) + "\t" + str(payload_path) + "\n"
    manifest_path.write_text(manifest_text, encoding="utf-8")
def run_aggregate(scratch_path, swipl_executable, stage_path, manifest_path, doc_count):
    tail_args = ["aggregate-check", str(manifest_path)]
    command = compiler_command(swipl_executable, stage_path, tail_args)
    result = bounded_swipl_run(scratch_path, "aggregate-check", command, None)
    if result.returncode != 0:
        relay_failure(scratch_path, result)
    if result.stderr:
        cleanup_and_fail(scratch_path, "aggregate-stderr", "non-empty stderr for manifest: " + str(manifest_path))
    stdout_text = result.stdout.decode("utf-8", errors="replace")
    expected_prefix = "ace_to_pl aggregate ok " + str(doc_count) + " documents "
    if not stdout_text.startswith(expected_prefix):
        cleanup_violation(scratch_path, "aggregate-report", "unexpected report: " + stdout_text.strip())
    if not stdout_text.endswith(" obligations\n"):
        cleanup_violation(scratch_path, "aggregate-report", "unexpected report: " + stdout_text.strip())
    return stdout_text
def run_recursion(scratch_path, swipl_executable, stage_path, manifest_path, doc_count):
    tail_args = ["recursion-check", str(manifest_path)]
    command = compiler_command(swipl_executable, stage_path, tail_args)
    result = bounded_swipl_run(scratch_path, "recursion-check", command, None)
    if result.returncode != 0:
        relay_failure(scratch_path, result)
    if result.stderr:
        cleanup_and_fail(scratch_path, "recursion-stderr", "non-empty stderr for manifest: " + str(manifest_path))
    stdout_text = result.stdout.decode("utf-8", errors="replace")
    expected_prefix = "ace_to_pl recursion ok " + str(doc_count) + " documents "
    if not stdout_text.startswith(expected_prefix):
        cleanup_violation(scratch_path, "recursion-report", "unexpected report: " + stdout_text.strip())
    if not stdout_text.endswith(" rule clauses\n"):
        cleanup_violation(scratch_path, "recursion-report", "unexpected report: " + stdout_text.strip())
    return stdout_text
def check_aggregate(scratch_path, swipl_executable, stage_path, manifest_pairs):
    doc_count = len(manifest_pairs)
    forward_path = scratch_path.joinpath("manifest-forward")
    write_manifest(forward_path, manifest_pairs)
    forward_report = run_aggregate(scratch_path, swipl_executable, stage_path, forward_path, doc_count)
    reverse_pairs = list(manifest_pairs)
    reverse_pairs.reverse()
    reverse_path = scratch_path.joinpath("manifest-reverse")
    write_manifest(reverse_path, reverse_pairs)
    reverse_report = run_aggregate(scratch_path, swipl_executable, stage_path, reverse_path, doc_count)
    if forward_report != reverse_report:
        cleanup_violation(scratch_path, "aggregate-order", "forward and reverse manifests disagree")
    recursion_report = run_recursion(scratch_path, swipl_executable, stage_path, forward_path, doc_count)
    print("goal: " + recursion_report.strip())
def queries_violation(scratch_path, category, detail):
    if scratch_path == None:
        violation(category, detail)
    cleanup_violation(scratch_path, category, detail)
def collect_query_aces(scratch_path, guideline_path):
    queries_dir = guideline_path.joinpath("queries")
    if queries_dir.is_symlink():
        queries_violation(scratch_path, "queries", "is a symlink: " + str(queries_dir))
    if not queries_dir.exists():
        return []
    if not queries_dir.is_dir():
        queries_violation(scratch_path, "queries", "not a directory: " + str(queries_dir))
    qids = []
    for entry in sorted(queries_dir.iterdir()):
        entry_name = entry.name
        subdir_name = False
        if entry_name == "pl":
            subdir_name = True
        if entry_name == "answers":
            subdir_name = True
        if entry_name == "traces":
            subdir_name = True
        if subdir_name:
            if entry.is_symlink():
                queries_violation(scratch_path, "queries", "unsupported entry: " + str(entry))
            if not entry.is_dir():
                queries_violation(scratch_path, "queries", "unsupported entry: " + str(entry))
        else:
            if not entry_name.endswith(".ace"):
                queries_violation(scratch_path, "queries", "unsupported entry: " + str(entry))
            if entry.is_symlink():
                queries_violation(scratch_path, "queries", "not a regular file: " + str(entry))
            if not entry.is_file():
                queries_violation(scratch_path, "queries", "not a regular file: " + str(entry))
            stem = entry_name.removesuffix(".ace")
            if not valid_docid(stem):
                queries_violation(scratch_path, "queries", "invalid qid filename: " + entry_name)
            qids.append(stem)
    return qids
def collect_query_dir(scratch_path, queries_dir, dir_name):
    sub_dir = queries_dir.joinpath(dir_name)
    if not sub_dir.is_dir():
        return []
    stems = []
    for entry in sorted(sub_dir.iterdir()):
        entry_name = entry.name
        if not entry_name.endswith(".pl"):
            queries_violation(scratch_path, "queries", "unsupported entry: " + str(entry))
        if entry.is_symlink():
            queries_violation(scratch_path, "queries", "not a regular file: " + str(entry))
        if not entry.is_file():
            queries_violation(scratch_path, "queries", "not a regular file: " + str(entry))
        stem = entry_name.removesuffix(".pl")
        if not valid_docid(stem):
            queries_violation(scratch_path, "queries", "invalid qid filename: " + entry_name)
        stems.append(stem)
    return stems
def build_query_manifest(manifest_path, pl_dir):
    manifest_text = ""
    if pl_dir.is_dir():
        for entry in sorted(pl_dir.iterdir()):
            entry_name = entry.name
            if entry_name.endswith(".pl"):
                manifest_text = manifest_text + str(entry) + "\t" + str(entry) + "\n"
    manifest_path.write_text(manifest_text, encoding="utf-8")
def run_question_compile(scratch_path, swipl_executable, stage_path, qid, ace_path, lexicon_path):
    ace_bytes = ace_path.read_bytes()
    tail_args = ["question", str(stage_path), qid]
    if lexicon_path != None:
        tail_args.append(str(lexicon_path))
    command = compiler_command(swipl_executable, stage_path, tail_args)
    try:
        result = subprocess.run(command, input=ace_bytes, capture_output=True, timeout=30)
    except subprocess.TimeoutExpired:
        queries_violation(scratch_path, "queries", "wall_clock for qid: " + qid)
    if result.returncode != 0:
        relay_failure(scratch_path, result)
    if result.stderr:
        cleanup_and_fail(scratch_path, "compiler-stderr", "non-empty stderr for question: " + qid)
    if not result.stdout:
        cleanup_and_fail(scratch_path, "compiler-stdout", "empty stdout for question: " + qid)
    newline_bytes = bytes([10])
    if not result.stdout.endswith(newline_bytes):
        cleanup_and_fail(scratch_path, "compiler-stdout", "missing final newline for question: " + qid)
    return result.stdout
def run_answer(scratch_path, swipl_executable, stage_path, qid, manifest_path, query_pl_path):
    tail_args = ["answer", str(manifest_path), str(query_pl_path)]
    command = compiler_command(swipl_executable, stage_path, tail_args)
    try:
        result = subprocess.run(command, capture_output=True, timeout=30)
    except subprocess.TimeoutExpired:
        queries_violation(scratch_path, "queries", "wall_clock for qid: " + qid)
    if result.returncode != 0:
        relay_failure(scratch_path, result)
    if result.stderr:
        cleanup_and_fail(scratch_path, "answer-stderr", "non-empty stderr for question: " + qid)
    if not result.stdout:
        cleanup_and_fail(scratch_path, "answer-stdout", "empty stdout for question: " + qid)
    newline_bytes = bytes([10])
    if not result.stdout.endswith(newline_bytes):
        cleanup_and_fail(scratch_path, "answer-stdout", "missing final newline for question: " + qid)
    return result.stdout
def run_trace(scratch_path, swipl_executable, stage_path, qid, manifest_path, query_pl_path, answers_pl_path):
    tail_args = ["trace", str(manifest_path), str(query_pl_path), str(answers_pl_path)]
    command = compiler_command(swipl_executable, stage_path, tail_args)
    try:
        result = subprocess.run(command, capture_output=True, timeout=30)
    except subprocess.TimeoutExpired:
        queries_violation(scratch_path, "queries", "wall_clock for qid: " + qid)
    if result.returncode != 0:
        relay_failure(scratch_path, result)
    if result.stderr:
        cleanup_and_fail(scratch_path, "trace-stderr", "non-empty stderr for question: " + qid)
    if not result.stdout:
        cleanup_and_fail(scratch_path, "trace-stdout", "empty stdout for question: " + qid)
    newline_bytes = bytes([10])
    if not result.stdout.endswith(newline_bytes):
        cleanup_and_fail(scratch_path, "trace-stdout", "missing final newline for question: " + qid)
    return result.stdout
def answer_result_text(answer_bytes):
    answer_text = answer_bytes.decode("utf-8", errors="replace")
    answer_lines = answer_text.splitlines()
    if len(answer_lines) != 2:
        return ""
    term_line = answer_lines.pop(1)
    parts = list(term_line.partition("result("))
    head_text = parts.pop(0)
    sep_text = parts.pop(0)
    tail_text = parts.pop(0)
    if sep_text == "":
        return ""
    if not tail_text.endswith("))."):
        return ""
    return tail_text.removesuffix(")).")
def trace_ascii_digit(ch):
    ascii_digits = "0123456789"
    ok = ch in ascii_digits
    return ok
def trace_scan_number(chars, num_text):
    is_float = False
    if chars:
        c2 = chars.pop()
        if c2 == ".":
            if chars:
                c3 = chars.pop()
                if trace_ascii_digit(c3):
                    is_float = True
                    num_text = num_text + "." + c3
                    frac_done = False
                    while not frac_done:
                        if not chars:
                            frac_done = True
                        else:
                            c4 = chars.pop()
                            if trace_ascii_digit(c4):
                                num_text = num_text + c4
                            else:
                                chars.append(c4)
                                frac_done = True
                else:
                    chars.append(c3)
                    chars.append(c2)
            else:
                chars.append(c2)
        else:
            chars.append(c2)
    if is_float:
        if chars:
            e1 = chars.pop()
            is_e = False
            if e1 == "e":
                is_e = True
            if e1 == "E":
                is_e = True
            if not is_e:
                chars.append(e1)
            else:
                exp_text = ""
                exp_sign = ""
                exp_ok = False
                if chars:
                    e2 = chars.pop()
                    if trace_ascii_digit(e2):
                        exp_text = e2
                        exp_ok = True
                    elif e2 == "-":
                        exp_sign = e2
                    elif e2 == "+":
                        exp_sign = e2
                    else:
                        chars.append(e2)
                if exp_sign:
                    if chars:
                        e3 = chars.pop()
                        if trace_ascii_digit(e3):
                            exp_text = e3
                            exp_ok = True
                        else:
                            chars.append(e3)
                if exp_ok:
                    exp_done = False
                    while not exp_done:
                        if not chars:
                            exp_done = True
                        else:
                            e4 = chars.pop()
                            if trace_ascii_digit(e4):
                                exp_text = exp_text + e4
                            else:
                                chars.append(e4)
                                exp_done = True
                    num_text = num_text + "e" + exp_sign + exp_text
                else:
                    if exp_sign:
                        chars.append(exp_sign)
                    chars.append(e1)
        return ["float", num_text]
    return ["int", num_text]
def trace_scan_tokens(term_text):
    chars = list(term_text)
    chars.reverse()
    tokens = []
    punct_chars = "()[],."
    ascii_lower = "abcdefghijklmnopqrstuvwxyz"
    ascii_alnum = ascii_lower + "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
    while chars:
        c = chars.pop()
        if c == "'":
            inner = ""
            closed = False
            while not closed:
                if not chars:
                    return None
                c2 = chars.pop()
                if c2 == "'":
                    closed = True
                elif c2 == "\\":
                    if not chars:
                        return None
                    c3 = chars.pop()
                    numeric_escape = False
                    if c3 == "x":
                        numeric_escape = True
                    if trace_ascii_digit(c3):
                        numeric_escape = True
                    if numeric_escape:
                        esc_closed = False
                        while not esc_closed:
                            if not chars:
                                return None
                            c4 = chars.pop()
                            if c4 == "\\":
                                esc_closed = True
                            else:
                                inner = inner + "\\" + c4
                    else:
                        inner = inner + "\\" + c3
                else:
                    inner = inner + c2
            tokens.append(["atom", inner])
        elif trace_ascii_digit(c):
            num_text = c
            num_done = False
            while not num_done:
                if not chars:
                    num_done = True
                else:
                    c2 = chars.pop()
                    if trace_ascii_digit(c2):
                        num_text = num_text + c2
                    else:
                        chars.append(c2)
                        num_done = True
            tokens.append(trace_scan_number(chars, num_text))
        elif c == "-":
            if not chars:
                return None
            c2 = chars.pop()
            if not trace_ascii_digit(c2):
                return None
            num_text = c + c2
            num_done = False
            while not num_done:
                if not chars:
                    num_done = True
                else:
                    c3 = chars.pop()
                    if trace_ascii_digit(c3):
                        num_text = num_text + c3
                    else:
                        chars.append(c3)
                        num_done = True
            tokens.append(trace_scan_number(chars, num_text))
        elif c in punct_chars:
            tokens.append(["punct", c])
        elif c in ascii_lower:
            atom_text = c
            atom_done = False
            while not atom_done:
                if not chars:
                    atom_done = True
                else:
                    c2 = chars.pop()
                    atom_char = c2 in ascii_alnum
                    if c2 == "_":
                        atom_char = True
                    if atom_char:
                        atom_text = atom_text + c2
                    else:
                        chars.append(c2)
                        atom_done = True
            tokens.append(["atom", atom_text])
        else:
            return None
    return tokens
def trace_parse_term(tokens):
    if not tokens:
        return None
    tok = tokens.pop()
    tok_copy = list(tok)
    kind = tok_copy.pop(0)
    value = tok_copy.pop(0)
    if kind == "int":
        return ["i", value]
    if kind == "float":
        return ["f", value]
    if kind == "atom":
        open_paren = False
        if tokens:
            peek = tokens.pop()
            if peek == ["punct", "("]:
                open_paren = True
            else:
                tokens.append(peek)
        if not open_paren:
            return ["a", value]
        args = []
        arg_node = trace_parse_term(tokens)
        if arg_node == None:
            return None
        args.append(arg_node)
        args_done = False
        while not args_done:
            if not tokens:
                return None
            sep = tokens.pop()
            if sep == ["punct", ","]:
                arg_node = trace_parse_term(tokens)
                if arg_node == None:
                    return None
                args.append(arg_node)
            elif sep == ["punct", ")"]:
                args_done = True
            else:
                return None
        return ["c", value, args]
    if kind == "punct":
        if value != "[":
            return None
        items = []
        if tokens:
            peek = tokens.pop()
            if peek == ["punct", "]"]:
                return ["l", items]
            tokens.append(peek)
        item_node = trace_parse_term(tokens)
        if item_node == None:
            return None
        items.append(item_node)
        items_done = False
        while not items_done:
            if not tokens:
                return None
            sep = tokens.pop()
            if sep == ["punct", ","]:
                item_node = trace_parse_term(tokens)
                if item_node == None:
                    return None
                items.append(item_node)
            elif sep == ["punct", "]"]:
                items_done = True
            else:
                return None
        return ["l", items]
    return None
def trace_parse_line(line_text):
    tokens = trace_scan_tokens(line_text)
    if tokens == None:
        return None
    tokens.reverse()
    term = trace_parse_term(tokens)
    if term == None:
        return None
    if not tokens:
        return None
    stop = tokens.pop()
    if stop != ["punct", "."]:
        return None
    if tokens:
        return None
    return term
def trace_walk_node(node, triples, node_marks):
    node_copy = list(node)
    kind = node_copy.pop(0)
    if kind != "c":
        return False
    name = node_copy.pop(0)
    args = node_copy.pop(0)
    if name == "naf":
        if len(args) != 1:
            return False
        return True
    if name != "clause":
        return False
    if len(args) != 3:
        return False
    args_copy = list(args)
    sentence_node = args_copy.pop(0)
    digest_node = args_copy.pop(0)
    children_node = args_copy.pop(0)
    sentence_copy = list(sentence_node)
    sentence_kind = sentence_copy.pop(0)
    if sentence_kind != "c":
        return False
    sentence_name = sentence_copy.pop(0)
    if sentence_name != "sentence":
        return False
    sentence_args = sentence_copy.pop(0)
    if len(sentence_args) != 2:
        return False
    sentence_args_copy = list(sentence_args)
    docid_node = sentence_args_copy.pop(0)
    s_node = sentence_args_copy.pop(0)
    docid_copy = list(docid_node)
    docid_kind = docid_copy.pop(0)
    if docid_kind != "a":
        return False
    docid_value = docid_copy.pop(0)
    if not valid_docid(docid_value):
        return False
    s_copy = list(s_node)
    s_kind = s_copy.pop(0)
    if s_kind != "i":
        return False
    s_text = s_copy.pop(0)
    if not s_text.isdigit():
        return False
    if s_text.startswith("0"):
        return False
    if len(s_text) > 9:
        return False
    s_value = int(s_text)
    digest_copy = list(digest_node)
    digest_kind = digest_copy.pop(0)
    if digest_kind != "c":
        return False
    digest_name = digest_copy.pop(0)
    if digest_name != "clause_sha256":
        return False
    digest_args = digest_copy.pop(0)
    if len(digest_args) != 1:
        return False
    digest_args_copy = list(digest_args)
    hex_node = digest_args_copy.pop(0)
    hex_copy = list(hex_node)
    hex_kind = hex_copy.pop(0)
    if hex_kind != "a":
        return False
    hex_value = hex_copy.pop(0)
    if not valid_digest(hex_value):
        return False
    children_copy = list(children_node)
    children_kind = children_copy.pop(0)
    if children_kind != "l":
        return False
    children_items = children_copy.pop(0)
    for child in children_items:
        child_ok = trace_walk_node(child, triples, node_marks)
        if not child_ok:
            return False
    triples.append([docid_value, s_value, hex_value])
    node_marks.append(1)
    return True
def trace_walk_proof(proof_node, triples, node_marks):
    proof_copy = list(proof_node)
    kind = proof_copy.pop(0)
    if kind != "c":
        return "invalid"
    name = proof_copy.pop(0)
    args = proof_copy.pop(0)
    if name == "proved":
        if len(args) != 1:
            return "invalid"
        args_copy = list(args)
        nodes_node = args_copy.pop(0)
        nodes_copy = list(nodes_node)
        nodes_kind = nodes_copy.pop(0)
        if nodes_kind != "l":
            return "invalid"
        node_items = nodes_copy.pop(0)
        if not node_items:
            return "invalid"
        for item in node_items:
            root_copy = list(item)
            root_kind = root_copy.pop(0)
            if root_kind != "c":
                return "invalid"
            root_name = root_copy.pop(0)
            if root_name != "clause":
                return "invalid"
            item_ok = trace_walk_node(item, triples, node_marks)
            if not item_ok:
                return "invalid"
        return "proved"
    if name == "unproved":
        if len(args) != 1:
            return "invalid"
        args_copy = list(args)
        why_node = args_copy.pop(0)
        if why_node == ["a", "finite_failure"]:
            return "unproved"
        if why_node == ["a", "limit"]:
            return "unproved"
        return "invalid"
    return "invalid"
def parse_trace_artifact(trace_bytes, qid):
    malformed = ["malformed", False, [], 0]
    text = ""
    try:
        text = trace_bytes.decode("utf-8")
    except UnicodeDecodeError:
        return malformed
    if not text.endswith("\n"):
        return malformed
    line_list = text.split("\n")
    line_list.pop()
    if len(line_list) != 2:
        return malformed
    comment_line = line_list.pop(0)
    term_line = line_list.pop(0)
    expected_comment = "% " + qid + " traced against the loaded composition by ace_to_pl trace mode; do not edit."
    if comment_line != expected_comment:
        return malformed
    term = trace_parse_line(term_line)
    if term == None:
        return malformed
    term_copy = list(term)
    term_kind = term_copy.pop(0)
    if term_kind != "c":
        return malformed
    term_name = term_copy.pop(0)
    if term_name != "$guideline_traces":
        return malformed
    term_args = term_copy.pop(0)
    if len(term_args) != 5:
        return malformed
    args_copy = list(term_args)
    version_node = args_copy.pop(0)
    qid_node = args_copy.pop(0)
    query_wrap = args_copy.pop(0)
    answers_wrap = args_copy.pop(0)
    result_wrap = args_copy.pop(0)
    if version_node != ["a", "v1"]:
        return malformed
    if qid_node != ["a", qid]:
        return malformed
    query_copy = list(query_wrap)
    query_kind = query_copy.pop(0)
    if query_kind != "c":
        return malformed
    query_name = query_copy.pop(0)
    if query_name != "query_sha256":
        return malformed
    query_args = query_copy.pop(0)
    if len(query_args) != 1:
        return malformed
    query_args_copy = list(query_args)
    query_hex_node = query_args_copy.pop(0)
    query_hex_copy = list(query_hex_node)
    query_hex_kind = query_hex_copy.pop(0)
    if query_hex_kind != "a":
        return malformed
    query_hex = query_hex_copy.pop(0)
    if not valid_digest(query_hex):
        return malformed
    answers_copy = list(answers_wrap)
    answers_kind = answers_copy.pop(0)
    if answers_kind != "c":
        return malformed
    answers_name = answers_copy.pop(0)
    if answers_name != "answers_sha256":
        return malformed
    answers_args = answers_copy.pop(0)
    if len(answers_args) != 1:
        return malformed
    answers_args_copy = list(answers_args)
    answers_hex_node = answers_args_copy.pop(0)
    answers_hex_copy = list(answers_hex_node)
    answers_hex_kind = answers_hex_copy.pop(0)
    if answers_hex_kind != "a":
        return malformed
    answers_hex = answers_hex_copy.pop(0)
    if not valid_digest(answers_hex):
        return malformed
    result_copy = list(result_wrap)
    result_kind = result_copy.pop(0)
    if result_kind != "c":
        return malformed
    result_name = result_copy.pop(0)
    if result_name != "result":
        return malformed
    result_args = result_copy.pop(0)
    if len(result_args) != 1:
        return malformed
    result_args_copy = list(result_args)
    result_node = result_args_copy.pop(0)
    triples = []
    node_marks = []
    demo_ok = False
    result_node_copy = list(result_node)
    result_node_kind = result_node_copy.pop(0)
    if result_node_kind != "c":
        return malformed
    result_node_name = result_node_copy.pop(0)
    result_node_args = result_node_copy.pop(0)
    if result_node_name == "yes":
        if len(result_node_args) != 1:
            return malformed
        yes_args_copy = list(result_node_args)
        proof_node = yes_args_copy.pop(0)
        proof_status = trace_walk_proof(proof_node, triples, node_marks)
        if proof_status == "invalid":
            return malformed
        if proof_status == "proved":
            demo_ok = True
    elif result_node_name == "no":
        if result_node_args != [["a", "finite_failure"]]:
            return malformed
    elif result_node_name == "indeterminate":
        if result_node_args != [["a", "limit"]]:
            return malformed
    elif result_node_name == "solutions":
        if len(result_node_args) != 1:
            return malformed
        sols_args_copy = list(result_node_args)
        rows_node = sols_args_copy.pop(0)
        rows_copy = list(rows_node)
        rows_kind = rows_copy.pop(0)
        if rows_kind != "l":
            return malformed
        row_items = rows_copy.pop(0)
        all_proved = True
        for row in row_items:
            row_copy = list(row)
            row_kind = row_copy.pop(0)
            if row_kind != "c":
                return malformed
            row_name = row_copy.pop(0)
            if row_name != "sol":
                return malformed
            row_args = row_copy.pop(0)
            if len(row_args) != 2:
                return malformed
            row_args_copy = list(row_args)
            values_node = row_args_copy.pop(0)
            proof_node = row_args_copy.pop(0)
            values_copy = list(values_node)
            values_kind = values_copy.pop(0)
            if values_kind != "l":
                return malformed
            proof_status = trace_walk_proof(proof_node, triples, node_marks)
            if proof_status == "invalid":
                return malformed
            if proof_status != "proved":
                all_proved = False
        if row_items:
            if all_proved:
                demo_ok = True
    else:
        return malformed
    return ["", demo_ok, triples, len(node_marks)]
def trace_block_table(pl_path):
    table = {}
    if pl_path.is_symlink():
        return table
    if not pl_path.is_file():
        return table
    raw = pl_path.read_bytes()
    text = ""
    try:
        text = raw.decode("utf-8")
    except UnicodeDecodeError:
        return table
    current_block = 0
    for line_text in text.split("\n"):
        is_marker = False
        if line_text.startswith("% S"):
            marker_rest = line_text.removeprefix("% S")
            marker_parts = list(marker_rest.partition(": "))
            marker_head = marker_parts.pop(0)
            marker_sep = marker_parts.pop(0)
            if marker_sep == ": ":
                head_ok = False
                if marker_head:
                    head_ok = True
                ascii_digits = "0123456789"
                for head_char in marker_head:
                    char_ok = head_char in ascii_digits
                    if not char_ok:
                        head_ok = False
                if marker_head.startswith("0"):
                    head_ok = False
                if len(marker_head) > 9:
                    head_ok = False
                if head_ok:
                    current_block = int(marker_head)
                    is_marker = True
        if not is_marker:
            skip_line = False
            if line_text.startswith("%"):
                skip_line = True
            if line_text.startswith(":- "):
                skip_line = True
            if line_text.startswith("guideline_schema_version("):
                skip_line = True
            if line_text.startswith("guideline_document("):
                skip_line = True
            if not line_text.endswith("."):
                skip_line = True
            if not skip_line:
                line_full = line_text + "\n"
                line_hash = sha256_hex(line_full.encode("utf-8"))
                table_key = str(current_block) + ":" + line_hash
                prior = table.get(table_key, 0)
                table.update({table_key: prior + 1})
    return table
def trace_join_nodes(scratch_path, guideline_path, qid, triples, doc_cache):
    for triple in triples:
        triple_copy = list(triple)
        docid_value = triple_copy.pop(0)
        s_value = triple_copy.pop(0)
        hex_value = triple_copy.pop(0)
        cached = docid_value in doc_cache
        if not cached:
            pl_path = guideline_path.joinpath("pl", docid_value + ".pl")
            doc_cache.update({docid_value: trace_block_table(pl_path)})
        table = doc_cache.get(docid_value)
        table_key = str(s_value) + ":" + hex_value
        match_count = table.get(table_key, 0)
        if match_count == 0:
            queries_violation(scratch_path, "traces", "trace node resolves to no committed clause line: " + qid + " " + docid_value + " S" + str(s_value))
        if match_count > 1:
            queries_violation(scratch_path, "traces", "trace node resolves to multiple committed clause lines: " + qid + " " + docid_value + " S" + str(s_value))
def validate_queries(scratch_path, swipl_executable, stage_path, guideline_path, lexicon_path):
    queries_dir = guideline_path.joinpath("queries")
    qids = collect_query_aces(scratch_path, guideline_path)
    pl_stems = collect_query_dir(scratch_path, queries_dir, "pl")
    answers_stems = collect_query_dir(scratch_path, queries_dir, "answers")
    traces_stems = collect_query_dir(scratch_path, queries_dir, "traces")
    if pl_stems != qids:
        queries_violation(scratch_path, "queries", "pl inventory differs from ace query set: " + str(queries_dir.joinpath("pl")))
    if answers_stems != qids:
        queries_violation(scratch_path, "queries", "answers inventory differs from ace query set: " + str(queries_dir.joinpath("answers")))
    if traces_stems != qids:
        queries_violation(scratch_path, "queries", "traces inventory differs from ace query set: " + str(queries_dir.joinpath("traces")))
    wh_count = 0
    yesno_count = 0
    trace_nodes = 0
    trace_doc_cache = {}
    if qids:
        manifest_path = scratch_path.joinpath("queries-manifest-" + guideline_path.name)
        build_query_manifest(manifest_path, guideline_path.joinpath("pl"))
        for qid in qids:
            ace_path = queries_dir.joinpath(qid + ".ace")
            first_bytes = run_question_compile(scratch_path, swipl_executable, stage_path, qid, ace_path, lexicon_path)
            second_bytes = run_question_compile(scratch_path, swipl_executable, stage_path, qid, ace_path, lexicon_path)
            if first_bytes != second_bytes:
                queries_violation(scratch_path, "determinism", "two query compiles differ for question: " + qid)
            committed_pl = queries_dir.joinpath("pl", qid + ".pl")
            committed_bytes = committed_pl.read_bytes()
            if first_bytes != committed_bytes:
                queries_violation(scratch_path, "stale", "committed query pl differs from fresh compile: " + str(committed_pl))
            first_answer = run_answer(scratch_path, swipl_executable, stage_path, qid, manifest_path, committed_pl)
            second_answer = run_answer(scratch_path, swipl_executable, stage_path, qid, manifest_path, committed_pl)
            if first_answer != second_answer:
                queries_violation(scratch_path, "determinism", "two answer runs differ for question: " + qid)
            committed_answers = queries_dir.joinpath("answers", qid + ".pl")
            committed_answer_bytes = committed_answers.read_bytes()
            if first_answer != committed_answer_bytes:
                queries_violation(scratch_path, "stale", "committed query answers differ from fresh answer: " + str(committed_answers))
            result_text = answer_result_text(first_answer)
            if result_text == "":
                queries_violation(scratch_path, "queries", "unparsable answer artifact for qid: " + qid)
            demo_ok = False
            if result_text == "yes":
                demo_ok = True
            if result_text.startswith("solutions("):
                if result_text != "solutions([])":
                    demo_ok = True
            if not demo_ok:
                queries_violation(scratch_path, "queries", "non-demo result for qid " + qid + ": " + result_text)
            fresh_text = first_bytes.decode("utf-8", errors="replace")
            fresh_lines = fresh_text.splitlines()
            if len(fresh_lines) != 4:
                queries_violation(scratch_path, "queries", "unexpected query pl shape for qid: " + qid)
            projection_line = fresh_lines.pop(3)
            if projection_line.endswith("answers([]))."):
                yesno_count = yesno_count + 1
            else:
                wh_count = wh_count + 1
            first_trace = run_trace(scratch_path, swipl_executable, stage_path, qid, manifest_path, committed_pl, committed_answers)
            second_trace = run_trace(scratch_path, swipl_executable, stage_path, qid, manifest_path, committed_pl, committed_answers)
            if first_trace != second_trace:
                queries_violation(scratch_path, "determinism", "two trace runs differ for question: " + qid)
            committed_trace = queries_dir.joinpath("traces", qid + ".pl")
            committed_trace_bytes = committed_trace.read_bytes()
            if first_trace != committed_trace_bytes:
                queries_violation(scratch_path, "stale", "committed query trace differs from fresh trace: " + str(committed_trace))
            parse_result = parse_trace_artifact(committed_trace_bytes, qid)
            parse_copy = list(parse_result)
            parse_detail = parse_copy.pop(0)
            demo_ok = parse_copy.pop(0)
            trace_triples = parse_copy.pop(0)
            qid_nodes = parse_copy.pop(0)
            if parse_detail != "":
                queries_violation(scratch_path, "traces", "malformed trace artifact: " + str(committed_trace))
            trace_join_nodes(scratch_path, guideline_path, qid, trace_triples, trace_doc_cache)
            if not demo_ok:
                queries_violation(scratch_path, "traces", "non-demo proof for qid: " + qid)
            trace_nodes = trace_nodes + qid_nodes
    return [len(qids), wh_count, yesno_count, trace_nodes]
def queries_meter(guideline_name, query_counts):
    counts_copy = list(query_counts)
    query_count = counts_copy.pop(0)
    wh_count = counts_copy.pop(0)
    yesno_count = counts_copy.pop(0)
    print("goal: queries " + guideline_name + " " + str(query_count) + " queries; wh=" + str(wh_count) + " yesno=" + str(yesno_count))
def traces_meter(guideline_name, query_counts):
    counts_copy = list(query_counts)
    query_count = counts_copy.pop(0)
    wh_count = counts_copy.pop(0)
    yesno_count = counts_copy.pop(0)
    node_count = counts_copy.pop(0)
    print("goal: traces " + guideline_name + " " + str(query_count) + " traces; nodes=" + str(node_count))
def queries_check_command(guideline_dir, stage_arg):
    guideline_path = pathlib.Path(guideline_dir)
    if guideline_path.is_symlink():
        fail("guideline", "is a symlink: " + str(guideline_path))
    if not guideline_path.is_dir():
        fail("guideline", "not a directory: " + str(guideline_path))
    lexicon_path = guideline_path.joinpath("lexicon.ulex")
    if lexicon_path.is_symlink():
        fail("guideline", "lexicon is a symlink: " + str(lexicon_path))
    if not lexicon_path.is_file():
        lexicon_path = None
    swipl_executable = resolve_swipl()
    scratch_path = make_scratch()
    if stage_arg == None:
        stage_path = stage_ape(scratch_path, swipl_executable)
    else:
        stage_path = pathlib.Path(stage_arg)
        if not stage_path.is_dir():
            cleanup_and_fail(scratch_path, "ape-stage", "missing stage: " + stage_arg)
    query_counts = validate_queries(scratch_path, swipl_executable, stage_path, guideline_path, lexicon_path)
    shutil.rmtree(scratch_path)
    queries_meter(guideline_path.name, query_counts)
def queries_command(guideline_id):
    if not valid_docid(guideline_id):
        fail("guideline", "invalid guideline id: " + guideline_id)
    guidelines_root = pathlib.Path("guidelines")
    guideline_path = guidelines_root.joinpath(guideline_id)
    if guideline_path.is_symlink():
        fail("guideline", "is a symlink: " + str(guideline_path))
    if not guideline_path.is_dir():
        fail("guideline", "not a directory: " + str(guideline_path))
    queries_dir = guideline_path.joinpath("queries")
    qids = collect_query_aces(None, guideline_path)
    if not qids:
        for dir_name in ["pl", "answers", "traces"]:
            sub_dir = queries_dir.joinpath(dir_name)
            if sub_dir.is_dir():
                shutil.rmtree(sub_dir)
        print("goal: queries ok 0 queries")
    else:
        lexicon_path = guideline_path.joinpath("lexicon.ulex")
        if lexicon_path.is_symlink():
            fail("guideline", "lexicon is a symlink: " + str(lexicon_path))
        if not lexicon_path.is_file():
            lexicon_path = None
        swipl_executable = resolve_swipl()
        scratch_path = make_scratch()
        stage_path = stage_ape(scratch_path, swipl_executable)
        manifest_path = scratch_path.joinpath("queries-manifest")
        build_query_manifest(manifest_path, guideline_path.joinpath("pl"))
        pl_by_qid = {}
        answers_by_qid = {}
        traces_by_qid = {}
        for qid in qids:
            ace_path = queries_dir.joinpath(qid + ".ace")
            pl_bytes = run_question_compile(scratch_path, swipl_executable, stage_path, qid, ace_path, lexicon_path)
            fresh_path = scratch_path.joinpath("query-" + qid + ".pl")
            fresh_path.write_bytes(pl_bytes)
            answer_bytes = run_answer(scratch_path, swipl_executable, stage_path, qid, manifest_path, fresh_path)
            fresh_answers_path = scratch_path.joinpath("answers-" + qid + ".pl")
            fresh_answers_path.write_bytes(answer_bytes)
            trace_bytes = run_trace(scratch_path, swipl_executable, stage_path, qid, manifest_path, fresh_path, fresh_answers_path)
            pl_by_qid.update({qid: pl_bytes})
            answers_by_qid.update({qid: answer_bytes})
            traces_by_qid.update({qid: trace_bytes})
        pl_dir = queries_dir.joinpath("pl")
        answers_dir = queries_dir.joinpath("answers")
        traces_dir = queries_dir.joinpath("traces")
        if pl_dir.is_dir():
            shutil.rmtree(pl_dir)
        if answers_dir.is_dir():
            shutil.rmtree(answers_dir)
        if traces_dir.is_dir():
            shutil.rmtree(traces_dir)
        pl_dir.mkdir()
        answers_dir.mkdir()
        traces_dir.mkdir()
        for qid in sorted(pl_by_qid):
            pl_target = pl_dir.joinpath(qid + ".pl")
            pl_target.write_bytes(pl_by_qid.get(qid))
            print("goal: wrote " + str(pl_target))
            answers_target = answers_dir.joinpath(qid + ".pl")
            answers_target.write_bytes(answers_by_qid.get(qid))
            print("goal: wrote " + str(answers_target))
            trace_target = traces_dir.joinpath(qid + ".pl")
            trace_target.write_bytes(traces_by_qid.get(qid))
            print("goal: wrote " + str(trace_target))
        shutil.rmtree(scratch_path)
        print("goal: queries ok " + str(len(qids)) + " queries")
def compile_command(guideline_id):
    if not valid_docid(guideline_id):
        fail("guideline", "invalid guideline id: " + guideline_id)
    guidelines_root = pathlib.Path("guidelines")
    guideline_path = guidelines_root.joinpath(guideline_id)
    guideline_symlink = guideline_path.is_symlink()
    if guideline_symlink:
        fail("guideline", "is a symlink: " + str(guideline_path))
    if not guideline_path.is_dir():
        fail("guideline", "not a directory: " + str(guideline_path))
    collected = collect_guideline(guideline_path)
    ace_paths = collected.pop(0)
    docids = collected.pop(0)
    lexicon_path = collected.pop(0)
    pl_dir = guideline_path.joinpath("pl")
    pl_symlink = pl_dir.is_symlink()
    if pl_symlink:
        fail("pl-dir", "is a symlink: " + str(pl_dir))
    pl_exists = pl_dir.exists()
    if pl_exists:
        if not pl_dir.is_dir():
            fail("pl-dir", "not a directory: " + str(pl_dir))
    swipl_executable = resolve_swipl()
    scratch_path = make_scratch()
    stage_path = stage_ape(scratch_path, swipl_executable)
    outputs = {}
    for docid in docids:
        ace_path = ace_paths.get(docid)
        extra_args = []
        pl_bytes = compile_doc(scratch_path, swipl_executable, stage_path, docid, ace_path, lexicon_path, extra_args)
        outputs.update({docid: pl_bytes})
    new_dir = scratch_path.joinpath("pl-new")
    new_dir.mkdir()
    for docid in sorted(outputs):
        payload = outputs.get(docid)
        target = new_dir.joinpath(docid + ".pl")
        target.write_bytes(payload)
    if pl_exists:
        backup_dir = scratch_path.joinpath("pl-old")
        os.rename(pl_dir, backup_dir)
    os.rename(new_dir, pl_dir)
    shutil.rmtree(scratch_path)
    for docid in sorted(outputs):
        print("goal: wrote " + str(pl_dir.joinpath(docid + ".pl")))
    print("goal: compile ok " + str(len(outputs)) + " documents")
def strict_expected_exit(class_name):
    exit_one = ["encoding", "bom", "control-char", "slot", "non-ascii", "syntax", "python-invalid"]
    exit_two = ["usage", "io"]
    is_one = class_name in exit_one
    is_two = class_name in exit_two
    if is_one:
        return 1
    if is_two:
        return 2
    violation("strict-class", "unknown strict error class: " + class_name)
def strict_fixture_class(fixture_name):
    stem = fixture_name.removesuffix(".emm")
    parts = stem.split("--")
    if len(parts) > 1:
        class_name = parts.pop(0)
        case_head = parts.pop(0)
        if class_name and case_head:
            return class_name
    violation("strict-fixture", "fixture name lacks <class>--<case> shape: " + fixture_name)
def collect_strict_dir(family_dir, sidecar_suffix, red_family):
    dir_symlink = family_dir.is_symlink()
    if dir_symlink:
        violation("strict-dir", "is a symlink: " + str(family_dir))
    dir_present = family_dir.is_dir()
    if not dir_present:
        violation("strict-dir", "missing: " + str(family_dir))
    fixtures = []
    fixture_stems = []
    sidecar_stems = []
    for entry in sorted(family_dir.iterdir()):
        entry_name = entry.name
        regular = entry.is_file()
        symlink = entry.is_symlink()
        if not regular:
            violation("strict-entry", "not a regular file: " + entry_name)
        if symlink:
            violation("strict-entry", "not a regular file: " + entry_name)
        is_fixture = entry_name.endswith(".emm")
        is_sidecar = entry_name.endswith(sidecar_suffix)
        if is_fixture:
            expected_exit = 0
            class_name = ""
            if red_family:
                class_name = strict_fixture_class(entry_name)
                expected_exit = strict_expected_exit(class_name)
            fixture_record = [entry, class_name, expected_exit]
            fixtures.append(fixture_record)
            fixture_stems.append(entry_name.removesuffix(".emm"))
        else:
            if is_sidecar:
                sidecar_stems.append(entry_name.removesuffix(sidecar_suffix))
            else:
                violation("strict-entry", "unsupported entry: " + entry_name)
    for sidecar_stem in sidecar_stems:
        paired = sidecar_stem in fixture_stems
        if not paired:
            violation("strict-entry", "orphan sidecar without fixture: " + sidecar_stem)
    for fixture_stem in fixture_stems:
        pinned = fixture_stem in sidecar_stems
        if not pinned:
            violation("strict-entry", "fixture lacks sidecar pin: " + fixture_stem)
    if not fixtures:
        violation("strict-dir", "no fixtures found: " + str(family_dir))
    return fixtures
def strict_compiler_env():
    env = os.environ.copy()
    env.update({"PYTHONPATH": "vendor/e--/src", "PYTHONDONTWRITEBYTECODE": "1"})
    return env
def run_strict_red(env, fixture_path, class_name, expected_exit):
    fixture_name = fixture_path.name
    command = [sys.executable, "-P", "-m", "e_minus_minus.strict", str(fixture_path)]
    result = subprocess.run(command, capture_output=True, env=env)
    if result.returncode != expected_exit:
        violation("strict-exit", "status " + str(result.returncode) + " for fixture: " + fixture_name)
    if result.stdout:
        violation("strict-stdout", "non-empty stdout for fixture: " + fixture_name)
    newline_bytes = bytes([10])
    newline_count = result.stderr.count(newline_bytes)
    final_newline = result.stderr.endswith(newline_bytes)
    if newline_count != 1:
        violation("strict-stderr", "stderr is not one LF line for fixture: " + fixture_name)
    if not final_newline:
        violation("strict-stderr", "stderr is not one LF line for fixture: " + fixture_name)
    stem = fixture_name.removesuffix(".emm")
    expect_path = fixture_path.parent.joinpath(stem + ".expect")
    expect_bytes = expect_path.read_bytes()
    if result.stderr != expect_bytes:
        violation("strict-expect", "stderr differs from expect pin for fixture: " + fixture_name)
    stderr_text = result.stderr.decode("utf-8", errors="replace")
    expected_prefix = "strict:" + class_name + ":"
    prefix_ok = stderr_text.startswith(expected_prefix)
    if not prefix_ok:
        violation("strict-classline", "stderr class mismatch for fixture: " + fixture_name)
def run_strict_green(env, fixture_path):
    fixture_name = fixture_path.name
    command = [sys.executable, "-P", "-m", "e_minus_minus.strict", str(fixture_path)]
    result = subprocess.run(command, capture_output=True, env=env)
    if result.returncode != 0:
        violation("strict-exit", "status " + str(result.returncode) + " for fixture: " + fixture_name)
    if result.stderr:
        violation("strict-stderr", "non-empty stderr for fixture: " + fixture_name)
    stem = fixture_name.removesuffix(".emm")
    golden_path = fixture_path.parent.joinpath(stem + ".golden")
    golden_bytes = golden_path.read_bytes()
    if result.stdout != golden_bytes:
        violation("strict-golden", "stdout differs from golden for fixture: " + fixture_name)
def check_strict_detector(env):
    detector_code = "import sys\nfrom e_minus_minus.parser import is_canonical_statement_line as f\nok = f('Try:') and f('Catch OSError:') and f('Catch urllib.error.HTTPError:') and not f('Try x:') and not f('Catch:') and not f('Catch a..b:')\nsys.exit(0 if ok else 1)"
    command = [sys.executable, "-P", "-c", detector_code]
    result = subprocess.run(command, capture_output=True, env=env)
    if result.returncode != 0:
        violation("strict-detector", "canonical detector disagrees on Try/Catch headers")
def check_strict_cli_probes(env):
    usage_command = [sys.executable, "-P", "-m", "e_minus_minus.strict"]
    usage_result = subprocess.run(usage_command, capture_output=True, env=env)
    if usage_result.returncode != strict_expected_exit("usage"):
        violation("strict-usage-probe", "bad-argv probe status " + str(usage_result.returncode))
    usage_text = usage_result.stderr.decode("utf-8", errors="replace")
    if not usage_text.startswith("strict:usage:"):
        violation("strict-usage-probe", "bad-argv probe stderr class mismatch")
    io_scratch = tempfile.mkdtemp()
    io_command = [sys.executable, "-P", "-m", "e_minus_minus.strict", io_scratch + "/absent.emm"]
    io_result = subprocess.run(io_command, capture_output=True, env=env)
    shutil.rmtree(io_scratch, ignore_errors=True)
    if io_result.returncode != strict_expected_exit("io"):
        violation("strict-io-probe", "unreadable-input probe status " + str(io_result.returncode))
    io_text = io_result.stderr.decode("utf-8", errors="replace")
    if not io_text.startswith("strict:io:"):
        violation("strict-io-probe", "unreadable-input probe stderr class mismatch")
def check_strict_fixtures():
    strict_root = pathlib.Path("tests/strict")
    root_symlink = strict_root.is_symlink()
    if root_symlink:
        violation("strict-dir", "is a symlink: " + str(strict_root))
    root_present = strict_root.is_dir()
    if not root_present:
        violation("strict-dir", "missing: " + str(strict_root))
    family_names = ["red", "green"]
    for root_entry in sorted(strict_root.iterdir()):
        root_name = root_entry.name
        known = root_name in family_names
        if not known:
            violation("strict-dir", "unsupported entry: " + root_name)
    red_records = collect_strict_dir(strict_root.joinpath("red"), ".expect", True)
    green_records = collect_strict_dir(strict_root.joinpath("green"), ".golden", False)
    env = strict_compiler_env()
    check_strict_detector(env)
    check_strict_cli_probes(env)
    red_count = 0
    for red_record in red_records:
        record_copy = list(red_record)
        fixture_path = record_copy.pop(0)
        class_name = record_copy.pop(0)
        expected_exit = record_copy.pop(0)
        run_strict_red(env, fixture_path, class_name, expected_exit)
        red_count = red_count + 1
    green_count = 0
    for green_record in green_records:
        green_copy = list(green_record)
        fixture_path = green_copy.pop(0)
        green_class = green_copy.pop(0)
        green_exit = green_copy.pop(0)
        run_strict_green(env, fixture_path)
        green_count = green_count + 1
    if red_count != 36:
        violation("strict-fixtures", "red case count drift: expected 36 got " + str(red_count))
    if green_count != 14:
        violation("strict-fixtures", "green case count drift: expected 14 got " + str(green_count))
    print("goal: strict fixtures ok " + str(red_count) + " red " + str(green_count) + " green")
def red_probe_class(probe_name):
    stem = probe_name.removesuffix(".ace")
    parts = stem.split("--")
    if len(parts) > 1:
        class_name = parts.pop(0)
        if class_name:
            return class_name
    violation("red-probe", "probe name lacks <class>-- prefix: " + probe_name)
def red_expected_exit(class_name):
    exit_one = ["input_utf8", "ape_messages", "empty_drs", "sentence_lines", "unsupported", "safety", "proof"]
    exit_two = ["usage", "ape_load", "ulex_load", "check_load", "uncaught"]
    is_one = class_name in exit_one
    is_two = class_name in exit_two
    if is_one:
        return 1
    if is_two:
        return 2
    violation("red-class", "unknown error class: " + class_name)
def collect_red_probes():
    red_dir = pathlib.Path("tests/red")
    red_symlink = red_dir.is_symlink()
    if red_symlink:
        violation("red-dir", "is a symlink: " + str(red_dir))
    if not red_dir.is_dir():
        violation("red-dir", "missing: " + str(red_dir))
    probes = []
    ace_stems = []
    ulex_stems = []
    expect_stems = []
    for entry in sorted(red_dir.iterdir()):
        entry_name = entry.name
        regular = entry.is_file()
        symlink = entry.is_symlink()
        if not regular:
            violation("red-entry", "not a regular file: " + entry_name)
        if symlink:
            violation("red-entry", "not a regular file: " + entry_name)
        is_ace = entry_name.endswith(".ace")
        is_ulex = entry_name.endswith(".ulex")
        is_expect = entry_name.endswith(".expect")
        if is_ace:
            class_name = red_probe_class(entry_name)
            expected_exit = red_expected_exit(class_name)
            probe_record = [entry, class_name, expected_exit]
            probes.append(probe_record)
            ace_stems.append(entry_name.removesuffix(".ace"))
        else:
            if is_ulex:
                ulex_stems.append(entry_name.removesuffix(".ulex"))
            else:
                if is_expect:
                    expect_stems.append(entry_name.removesuffix(".expect"))
                else:
                    violation("red-entry", "unsupported entry: " + entry_name)
    for ulex_stem in ulex_stems:
        paired = ulex_stem in ace_stems
        if not paired:
            violation("red-entry", "orphan ulex without ace probe: " + ulex_stem)
    for expect_stem in expect_stems:
        expect_paired = expect_stem in ace_stems
        if not expect_paired:
            violation("red-entry", "orphan expect without ace probe: " + expect_stem)
    for ace_stem in ace_stems:
        pinned = ace_stem in expect_stems
        if not pinned:
            violation("red-entry", "probe lacks expect pin: " + ace_stem)
    if not probes:
        violation("red-dir", "no red probes found")
    return probes
def run_red_probe(scratch_path, swipl_executable, stage_path, probe_path, class_name, expected_exit):
    probe_name = probe_path.name
    probe_bytes = probe_path.read_bytes()
    lexicon_path = probe_path.with_suffix(".ulex")
    lexicon_present = lexicon_path.is_file()
    tail_args = [str(stage_path), "red-probe"]
    if lexicon_present:
        tail_args.append(str(lexicon_path))
    command = compiler_command(swipl_executable, stage_path, tail_args)
    result = bounded_swipl_run(scratch_path, "red-probe " + probe_name, command, probe_bytes)
    if result.returncode != expected_exit:
        cleanup_violation(scratch_path, "red-exit", "status " + str(result.returncode) + " for probe: " + probe_name)
    if result.stdout:
        cleanup_violation(scratch_path, "red-stdout", "non-empty stdout for probe: " + probe_name)
    newline_bytes = bytes([10])
    newline_count = result.stderr.count(newline_bytes)
    final_newline = result.stderr.endswith(newline_bytes)
    if newline_count != 1:
        cleanup_violation(scratch_path, "red-stderr", "stderr is not one LF line for probe: " + probe_name)
    if not final_newline:
        cleanup_violation(scratch_path, "red-stderr", "stderr is not one LF line for probe: " + probe_name)
    expect_path = probe_path.with_suffix(".expect")
    expect_present = expect_path.is_file()
    if not expect_present:
        cleanup_violation(scratch_path, "red-expect", "missing expect pin for probe: " + probe_name)
    expect_bytes = expect_path.read_bytes()
    if result.stderr != expect_bytes:
        cleanup_violation(scratch_path, "red-expect", "stderr differs from expect pin for probe: " + probe_name)
    stderr_text = result.stderr.decode("utf-8", errors="replace")
    expected_prefix = "ace_to_pl_error(" + class_name + ","
    if not stderr_text.startswith(expected_prefix):
        cleanup_violation(scratch_path, "red-class", "stderr class mismatch for probe: " + probe_name)
    if not stderr_text.endswith(").\n"):
        cleanup_violation(scratch_path, "red-stderr", "stderr lacks canonical term suffix for probe: " + probe_name)
bal_pattern = "(?:[^()]|\\([^()]*\\))*"
access_rx = re.compile("^(open|unverified|paywalled\\([^)]+\\)|login\\([^)]+\\))$")
status_rx = re.compile("^(unqueued|provisional\\(" + bal_pattern + "\\)|queued|in-progress|done|blocked\\(" + bal_pattern + "\\)|excluded\\(" + bal_pattern + "\\))$")
swept_rx = re.compile("^(pending|-|\\d{4}-\\d{2}-\\d{2} .+|blocked\\(" + bal_pattern + "\\))$")
year_rx = re.compile("\\((\\d{4})(;\\s*pub\\s*(\\d{4}))?\\)\\s*$")
token_rx = re.compile("guestAccessKey|accessKey=|token=")
org_header = ["org", "abbrev", "class", "CPGs", "index URL", "enum sources", "swept"]
row_header = "org\ttitle (year)\tURL\taccess\tstatus\tnotes"
class_names = ["federal", "society", "other"]
cpgs_names = ["yes", "unverified", "no"]
def read_compendium_file(path_text):
    file_path = pathlib.Path(path_text)
    if file_path.is_symlink():
        fail("compendium", "is a symlink: " + path_text)
    if not file_path.is_file():
        fail("compendium", "missing: " + path_text)
    return file_path.read_text(encoding="utf-8")
def org_section_text(comp_text):
    head_parts = comp_text.split("\n## Organizations\n", 1)
    if len(head_parts) != 2:
        violation("compendium-org-table", "missing `## Organizations` section in .agent/compendium.md")
    head_parts.pop(0)
    tail_text = head_parts.pop(0)
    tail_parts = tail_text.split("\n## Guidelines\n", 1)
    section_text = tail_parts.pop(0)
    return section_text
def org_table_rows(section_text):
    rows = []
    seen_header = False
    separator_chars = set("- ")
    for raw_line in section_text.split("\n"):
        line_text = raw_line.strip()
        starts_pipe = line_text.startswith("|")
        ends_pipe = line_text.endswith("|")
        if starts_pipe:
            if ends_pipe:
                inner_text = line_text.removeprefix("|")
                inner_text = inner_text.removesuffix("|")
                cell_list = []
                for cell_text in inner_text.split("|"):
                    cell_list.append(cell_text.strip())
                if not seen_header:
                    if cell_list != org_header:
                        violation("compendium-org-table", "organization table header mismatch")
                    seen_header = True
                else:
                    joined_text = ""
                    for cell_text in cell_list:
                        joined_text = joined_text + cell_text
                    joined_chars = set(joined_text)
                    is_separator = joined_chars.issubset(separator_chars)
                    if not is_separator:
                        if len(cell_list) != 7:
                            violation("compendium-org-table", "organization row without 7 cells: " + line_text)
                        rows.append(cell_list)
    if not seen_header:
        violation("compendium-org-table", "organization table absent - no header row found")
    if not rows:
        violation("compendium-org-table", "organization table holds no rows")
    return rows
def check_compendium_orgs(orows):
    org_class = {}
    seen_names = {}
    remaining = 0
    order_keys = []
    for row_cells in orows:
        cells_copy = list(row_cells)
        org_name = cells_copy.pop(0)
        abbrev_cell = cells_copy.pop(0)
        class_cell = cells_copy.pop(0)
        cpgs_cell = cells_copy.pop(0)
        index_cell = cells_copy.pop(0)
        sources_cell = cells_copy.pop(0)
        swept_cell = cells_copy.pop(0)
        if not org_name:
            violation("compendium-org", "empty org cell in organization row")
        if not class_cell:
            violation("compendium-org", "empty class cell for: " + org_name)
        if not cpgs_cell:
            violation("compendium-org", "empty CPGs cell for: " + org_name)
        class_known = class_cell in class_names
        if not class_known:
            violation("compendium-org", "bad class `" + class_cell + "` for: " + org_name)
        cpgs_known = cpgs_cell in cpgs_names
        if not cpgs_known:
            violation("compendium-org", "bad CPGs `" + cpgs_cell + "` for: " + org_name)
        dup_org = org_name in seen_names
        if dup_org:
            violation("compendium-org", "duplicate organization row: " + org_name)
        seen_names.update({org_name: True})
        if not swept_rx.match(swept_cell):
            violation("compendium-org", "bad swept `" + swept_cell + "` for: " + org_name)
        org_class.update({org_name: class_cell})
        class_rank = class_names.index(class_cell)
        cpgs_rank = cpgs_names.index(cpgs_cell)
        folded_name = org_name.casefold()
        order_key = [class_rank, cpgs_rank, folded_name]
        order_keys.append([order_key, org_name])
        is_blocked = swept_cell.startswith("blocked(")
        is_dated = True
        if swept_cell == "pending":
            is_dated = False
        if swept_cell == "-":
            is_dated = False
        if is_blocked:
            is_dated = False
        terminal = False
        if cpgs_cell == "no":
            terminal = True
        if is_blocked:
            terminal = True
        if is_dated:
            terminal = True
        if not terminal:
            remaining = remaining + 1
    prev_key = None
    prev_bound = False
    for key_record in order_keys:
        record_copy = list(key_record)
        current_key = record_copy.pop(0)
        current_name = record_copy.pop(0)
        if prev_bound:
            out_of_order = current_key < prev_key
            if out_of_order:
                violation("compendium-org-order", "organization order violates class -> CPGs -> alpha at: " + current_name)
        prev_key = current_key
        prev_bound = True
    return [org_class, remaining]
def load_compendium_rows(tsv_text):
    line_list = tsv_text.split("\n")
    header_line = line_list.pop(0)
    if header_line != row_header:
        violation("compendium-tsv", "first line of .agent/compendium.tsv is not the 6-column header")
    rows = []
    for line_text in line_list:
        if line_text:
            cell_list = line_text.split("\t")
            if len(cell_list) != 6:
                first_cell = cell_list.pop(0)
                violation("compendium-tsv", "row without 6 cells starting: " + first_cell)
            rows.append(cell_list)
    return rows
def check_compendium_rows(grows, org_class):
    active_count = 0
    remaining = 0
    provisional_count = 0
    seen_rows = {}
    url_titles = {}
    order_keys = []
    for row_cells in grows:
        cells_copy = list(row_cells)
        org_cell = cells_copy.pop(0)
        title_cell = cells_copy.pop(0)
        url_cell = cells_copy.pop(0)
        access_cell = cells_copy.pop(0)
        status_cell = cells_copy.pop(0)
        notes_cell = cells_copy.pop(0)
        if not access_rx.match(access_cell):
            violation("compendium-row", "bad access `" + access_cell + "` for: " + title_cell)
        if not status_rx.match(status_cell):
            violation("compendium-row", "bad status `" + status_cell + "` for: " + title_cell)
        year_match = year_rx.search(title_cell)
        if not year_match:
            violation("compendium-row", "title lacks terminal (year): " + title_cell)
        if not org_cell:
            violation("compendium-row", "empty org cell for: " + title_cell)
        if not title_cell:
            violation("compendium-row", "empty title cell")
        if not url_cell:
            violation("compendium-row", "empty URL cell for: " + title_cell)
        is_http = url_cell.startswith("http://")
        is_https = url_cell.startswith("https://")
        if not (is_http or is_https):
            violation("compendium-row", "URL lacks http(s) scheme: " + url_cell)
        if token_rx.search(url_cell):
            violation("compendium-row", "capability-token URL: " + url_cell)
        is_queued = status_cell == "queued"
        is_progress = status_cell == "in-progress"
        if is_queued or is_progress:
            active_count = active_count + 1
        is_provisional = status_cell.startswith("provisional(")
        is_excluded = status_cell.startswith("excluded(")
        if access_cell == "unverified":
            if not (is_provisional or is_excluded):
                violation("compendium-row", "unverified access must be provisional(...) or excluded(...): " + title_cell)
        if status_cell == "unqueued":
            has_unresolved = "unresolved" in notes_cell
            if has_unresolved:
                violation("compendium-row", "unresolved row must not be unqueued: " + title_cell)
        is_done = status_cell == "done"
        is_blocked_row = status_cell.startswith("blocked(")
        row_terminal = False
        if is_done:
            row_terminal = True
        if is_blocked_row:
            row_terminal = True
        if is_excluded:
            row_terminal = True
        if not row_terminal:
            remaining = remaining + 1
        if is_provisional:
            provisional_count = provisional_count + 1
        folded_title = title_cell.casefold()
        row_key = folded_title + "\t" + url_cell
        dup_row = row_key in seen_rows
        if dup_row:
            violation("compendium-dup", "duplicate guideline row: " + title_cell)
        seen_rows.update({row_key: True})
        url_seen = url_cell in url_titles
        if url_seen:
            prior_title = url_titles.get(url_cell)
            if prior_title != folded_title:
                violation("compendium-dup", "URL shared by two rows: " + url_cell)
        url_titles.update({url_cell: folded_title})
        band = 1
        if access_cell == "open":
            band = 0
        org_parts = org_cell.split("+")
        first_part = org_parts.pop(0)
        first_org = first_part.strip()
        first_class = org_class.get(first_org, "other")
        class_rank = class_names.index(first_class)
        year_text = year_match.group(1)
        year_value = int(year_text)
        year_key = 0 - year_value
        folded_org = org_cell.casefold()
        order_key = [band, class_rank, folded_org, year_key, folded_title]
        order_keys.append([order_key, title_cell])
    if active_count > 1:
        violation("compendium-active", str(active_count) + " rows queued|in-progress (max 1)")
    prev_key = None
    prev_bound = False
    for key_record in order_keys:
        record_copy = list(key_record)
        current_key = record_copy.pop(0)
        current_title = record_copy.pop(0)
        if prev_bound:
            out_of_order = current_key < prev_key
            if out_of_order:
                violation("compendium-row-order", "guideline order violates access -> class -> org -> year desc -> title at: " + current_title)
        prev_key = current_key
        prev_bound = True
    return [remaining, provisional_count]
def check_compendium():
    comp_text = read_compendium_file(".agent/compendium.md")
    tsv_text = read_compendium_file(".agent/compendium.tsv")
    section_text = org_section_text(comp_text)
    orows = org_table_rows(section_text)
    org_count = len(orows)
    org_result = check_compendium_orgs(orows)
    org_class = org_result.pop(0)
    orgs_remaining = org_result.pop(0)
    grows = load_compendium_rows(tsv_text)
    row_count = len(grows)
    row_result = check_compendium_rows(grows, org_class)
    rows_remaining = row_result.pop(0)
    provisional_count = row_result.pop(0)
    summary = "goal: compendium ok " + str(org_count) + " organizations " + str(row_count) + " rows; terminal remaining: orgs=" + str(orgs_remaining) + " rows=" + str(rows_remaining) + " provisional=" + str(provisional_count)
    print(summary)
projection_header_text = "# format: docid<TAB>region<TAB>kept<TAB>dropped\n# per-document projection loss record: what each minimal rule keeps from its verbatim source\n# region and what it drops or interprets. Header bytes, row shape and per-document row\n# coverage are validated by goal.py check; kept/dropped prose stays document-owned."
census_row_rx = re.compile("p[0-9]{3}[.]C[0-9]{2}")
coverage_header_text = "# format: id<TAB>file<TAB>page<TAB>section<TAB>status\n# status: ace(<docid>) | restates(<id>) | uncovered(<class>: <one-clause reason>) | pending\n# uncovered classes: heading | process | external | aim | descriptive | notice"
uncovered_class_names = ["heading", "process", "external", "aim", "descriptive", "notice"]
census_rx = re.compile("identify the ([0-9]+) payloads below")
v1_functors = ["guideline_schema_version", "guideline_document", "guideline_entity", "guideline_cardinality", "guideline_event", "guideline_arg", "guideline_pp", "guideline_property", "guideline_operator"]
v1_directives = ["guideline_schema_version/1", "guideline_document/3", "guideline_entity/4", "guideline_cardinality/5", "guideline_event/3", "guideline_arg/4", "guideline_pp/4", "guideline_property/4", "guideline_operator/3"]
v1_decl_kinds = ["multifile", "discontiguous"]
token_strip_chars = ".,;:?!\"()"
clex_path_text = "vendor/clex/clex_lexicon.pl"
swipl_version_required = "9.2.9"
def read_corpus_file(file_path, category):
    if file_path.is_symlink():
        violation(category, "is a symlink: " + str(file_path))
    if not file_path.is_file():
        violation(category, "missing: " + str(file_path))
    return file_path.read_bytes()
def check_projection_ledger(guideline_path):
    ledger_path = guideline_path.joinpath("audit", "projection-notes.tsv")
    data = read_corpus_file(ledger_path, "projection-ledger")
    text = data.decode("utf-8")
    if "\r" in text:
        violation("projection-ledger", "carriage return byte in ledger")
    if not text.endswith("\n"):
        violation("projection-ledger", "ledger lacks final newline")
    expected_header = projection_header_text + "\n"
    if not text.startswith(expected_header):
        violation("projection-ledger", "header bytes drift")
    body = text.removeprefix(expected_header)
    row_lines = body.split("\n")
    row_lines.pop()
    if not row_lines:
        violation("projection-ledger", "ledger holds no rows")
    seen_docids = {}
    row_pairs = []
    for row_line in row_lines:
        fields = row_line.split("\t")
        if len(fields) != 4:
            violation("projection-ledger", "row without 4 columns: " + row_line)
        docid = fields.pop(0)
        region = fields.pop(0)
        kept = fields.pop(0)
        dropped = fields.pop(0)
        if not valid_docid(docid):
            violation("projection-ledger", "invalid docid: " + docid)
        if not region:
            violation("projection-ledger", "empty region for: " + docid)
        if not kept:
            violation("projection-ledger", "empty kept column for: " + docid)
        if not dropped:
            violation("projection-ledger", "empty dropped column for: " + docid)
        duplicate = docid in seen_docids
        if duplicate:
            violation("projection-ledger", "duplicate row docid: " + docid)
        seen_docids.update({docid: True})
        row_pairs.append([docid, region])
    return row_pairs
def check_census_map(guideline_path, status_by_id):
    map_path = guideline_path.joinpath("audit", "census-map.tsv")
    data = read_corpus_file(map_path, "census-map")
    text = ""
    try:
        text = data.decode("utf-8")
    except UnicodeDecodeError:
        violation("census-map", "not UTF-8: " + str(map_path))
    if "\r" in text:
        violation("census-map", "carriage return byte in map")
    if not text.endswith("\n"):
        violation("census-map", "map lacks final newline")
    if not text.startswith("# format: census<TAB>region<TAB>disposition\n"):
        violation("census-map", "header bytes drift")
    row_count = 0
    seen_census = {}
    line_list = text.split("\n")
    line_list.pop()
    for line_text in line_list:
        if not line_text.startswith("#"):
            fields = line_text.split("\t")
            if len(fields) != 3:
                violation("census-map", "row without 3 columns: " + line_text)
            census_key = fields.pop(0)
            region_field = fields.pop(0)
            disposition = fields.pop(0)
            if census_row_rx.fullmatch(census_key) == None:
                violation("census-map", "census key grammar: " + census_key)
            duplicate = census_key in seen_census
            if duplicate:
                violation("census-map", "duplicate census key: " + census_key)
            seen_census.update({census_key: True})
            if region_field != "-":
                status = status_by_id.get(region_field, "")
                if status == "":
                    violation("census-map", "row names no coverage region: " + census_key + " " + region_field)
            if not disposition:
                violation("census-map", "empty disposition: " + census_key)
            row_count = row_count + 1
    if row_count == 0:
        violation("census-map", "map holds no rows")
def coverage_status_kind(row_id, status_text):
    if status_text == "pending":
        return ["pending", ""]
    if status_text.startswith("ace("):
        closed = status_text.endswith(")")
        if not closed:
            violation("coverage", "malformed status for " + row_id + ": " + status_text)
        inner = status_text.removeprefix("ace(")
        inner = inner.removesuffix(")")
        if not valid_docid(inner):
            violation("coverage", "ace names invalid docid for " + row_id + ": " + inner)
        return ["ace", inner]
    if status_text.startswith("restates("):
        closed = status_text.endswith(")")
        if not closed:
            violation("coverage", "malformed status for " + row_id + ": " + status_text)
        inner = status_text.removeprefix("restates(")
        inner = inner.removesuffix(")")
        if not inner:
            violation("coverage", "empty restates target for " + row_id)
        return ["restates", inner]
    if status_text.startswith("uncovered("):
        closed = status_text.endswith(")")
        if not closed:
            violation("coverage", "malformed status for " + row_id + ": " + status_text)
        inner = status_text.removeprefix("uncovered(")
        inner = inner.removesuffix(")")
        parts = inner.split(": ", 1)
        if len(parts) != 2:
            violation("coverage", "uncovered without class and reason for " + row_id + ": " + status_text)
        class_name = parts.pop(0)
        reason_text = parts.pop(0)
        known_class = class_name in uncovered_class_names
        if not known_class:
            violation("coverage", "unknown uncovered class for " + row_id + ": " + class_name)
        if not reason_text:
            violation("coverage", "empty uncovered reason for " + row_id)
        return ["uncovered", class_name]
    violation("coverage", "unknown status for " + row_id + ": " + status_text)
def bind_locator_payload(evidence_path, region_id, payload_lines, payload_by_locator):
    line_count = len(payload_lines)
    if line_count != 1:
        violation("coverage", "evidence region " + region_id + " carries " + str(line_count) + " content lines in: " + str(evidence_path))
    payload_line = payload_lines.pop(0)
    payload_by_locator.update({region_id: payload_line})
def strip_list_label(payload_line):
    label_parts = payload_line.split(". ", 1)
    if len(label_parts) != 2:
        return payload_line
    head_text = label_parts.pop(0)
    tail_text = label_parts.pop(0)
    if not head_text:
        return payload_line
    digit_chars = set("0123456789")
    head_chars = set(head_text)
    all_digits = head_chars.issubset(digit_chars)
    if not all_digits:
        return payload_line
    return tail_text
def evidence_regions(evidence_path):
    data = read_corpus_file(evidence_path, "coverage")
    text = data.decode("utf-8")
    census_hit = census_rx.search(text)
    if census_hit == None:
        violation("coverage", "evidence lacks region-authority census: " + str(evidence_path))
    census_text = census_hit.group(1)
    census_count = int(census_text)
    locator_ids = []
    payload_by_locator = {}
    current_locator = ""
    current_payloads = []
    past_first_blank = False
    post_blank_lines = []
    for raw_line in text.split("\n"):
        is_locator = False
        bracketed = raw_line.startswith("[")
        if bracketed:
            closed = raw_line.endswith("]")
            delimited = " | " in raw_line
            if closed:
                if delimited:
                    body_text = raw_line.removeprefix("[")
                    parts = body_text.split(" | ")
                    region_id = parts.pop(0)
                    spaced = " " in region_id
                    if region_id:
                        if not spaced:
                            is_locator = True
                            if current_locator:
                                bind_locator_payload(evidence_path, current_locator, current_payloads, payload_by_locator)
                            locator_ids.append(region_id)
                            current_locator = region_id
                            current_payloads = []
        if not is_locator:
            if raw_line == "":
                past_first_blank = True
            else:
                if current_locator:
                    current_payloads.append(raw_line)
                if past_first_blank:
                    post_blank_lines.append(raw_line)
    if current_locator:
        bind_locator_payload(evidence_path, current_locator, current_payloads, payload_by_locator)
    ordered_payloads = []
    if not locator_ids:
        payload_count = len(post_blank_lines)
        if payload_count != census_count:
            violation("coverage", "payload lines " + str(payload_count) + " differ from census " + str(census_count) + " for: " + str(evidence_path))
        for post_blank_line in post_blank_lines:
            ordered_payloads.append(strip_list_label(post_blank_line))
    return [census_count, locator_ids, payload_by_locator, ordered_payloads]
def check_coverage(guideline_path, docids, emit_meter):
    coverage_path = guideline_path.joinpath("coverage.tsv")
    data = read_corpus_file(coverage_path, "coverage")
    text = data.decode("utf-8")
    if "\r" in text:
        violation("coverage", "carriage return byte in ledger")
    if not text.endswith("\n"):
        violation("coverage", "ledger lacks final newline")
    expected_header = coverage_header_text + "\n"
    if not text.startswith(expected_header):
        violation("coverage", "header bytes drift")
    body = text.removeprefix(expected_header)
    row_lines = body.split("\n")
    row_lines.pop()
    docid_known = {}
    for docid in docids:
        docid_known.update({docid: True})
    comment_zone = True
    row_count = 0
    ace_count = 0
    restates_count = 0
    uncovered_count = 0
    pending_count = 0
    status_by_id = {}
    ace_claims = {}
    restates_rows = []
    restates_targets = []
    file_order = []
    file_ids = {}
    ace_row_line_by_docid = {}
    ace_file_by_docid = {}
    ace_region_by_docid = {}
    ace_ordinal_by_docid = {}
    evidence_locator_payloads = {}
    evidence_ordinal_payloads = {}
    for row_line in row_lines:
        is_comment = row_line.startswith("#")
        if is_comment:
            if not comment_zone:
                violation("coverage", "comment line after rows: " + row_line)
        else:
            comment_zone = False
            fields = row_line.split("\t")
            if len(fields) != 5:
                violation("coverage", "row without 5 columns: " + row_line)
            row_id = fields.pop(0)
            file_field = fields.pop(0)
            page_field = fields.pop(0)
            section_field = fields.pop(0)
            status_field = fields.pop(0)
            if not row_id:
                violation("coverage", "empty region id: " + row_line)
            spaced_id = " " in row_id
            if spaced_id:
                violation("coverage", "region id holds a space: " + row_id)
            under_source = file_field.startswith("source/")
            if not under_source:
                violation("coverage", "file outside source/ for " + row_id + ": " + file_field)
            dotted = ".." in file_field
            if dotted:
                violation("coverage", "file path traversal for " + row_id + ": " + file_field)
            if not page_field:
                violation("coverage", "empty page for: " + row_id)
            if not section_field:
                violation("coverage", "empty section for: " + row_id)
            duplicate = row_id in status_by_id
            if duplicate:
                violation("coverage", "duplicate region id: " + row_id)
            status_by_id.update({row_id: status_field})
            known_file = file_field in file_ids
            if not known_file:
                file_order.append(file_field)
                empty_ids = []
                file_ids.update({file_field: empty_ids})
            claimed_ids = file_ids.get(file_field)
            claimed_ids.append(row_id)
            parsed = coverage_status_kind(row_id, status_field)
            kind = parsed.pop(0)
            payload = parsed.pop(0)
            row_count = row_count + 1
            if kind == "pending":
                pending_count = pending_count + 1
            if kind == "ace":
                ace_count = ace_count + 1
                claimed_before = payload in ace_claims
                if claimed_before:
                    violation("coverage", "docid claimed by two rows: " + payload)
                known_docid = payload in docid_known
                if not known_docid:
                    violation("coverage", "ace names unknown docid for " + row_id + ": " + payload)
                ace_claims.update({payload: True})
                ace_row_line_by_docid.update({payload: row_line + "\n"})
                ace_file_by_docid.update({payload: file_field})
                ace_region_by_docid.update({payload: row_id})
                citing_count = len(claimed_ids)
                ace_ordinal_by_docid.update({payload: citing_count - 1})
            if kind == "restates":
                restates_count = restates_count + 1
                restates_rows.append(row_id)
                restates_targets.append(payload)
            if kind == "uncovered":
                uncovered_count = uncovered_count + 1
    if row_count == 0:
        violation("coverage", "ledger holds no rows")
    for target_id in restates_targets:
        row_id = restates_rows.pop(0)
        if target_id == row_id:
            violation("coverage", "restates itself: " + row_id)
        target_status = status_by_id.get(target_id, "")
        if not target_status:
            violation("coverage", "restates unknown region for " + row_id + ": " + target_id)
        chained = target_status.startswith("restates(")
        if chained:
            violation("coverage", "restates a restatement for " + row_id + ": " + target_id)
    for docid in docids:
        claimed = docid in ace_claims
        if not claimed:
            violation("coverage", "docid without a coverage row: " + docid)
    for file_field in file_order:
        evidence_path = guideline_path.joinpath(file_field)
        region_info = evidence_regions(evidence_path)
        census_count = region_info.pop(0)
        locator_ids = region_info.pop(0)
        payload_by_locator = region_info.pop(0)
        ordered_payloads = region_info.pop(0)
        claimed_ids = file_ids.get(file_field)
        claimed_count = len(claimed_ids)
        if claimed_count != census_count:
            violation("coverage", "rows " + str(claimed_count) + " differ from census " + str(census_count) + " for: " + file_field)
        if locator_ids:
            locator_count = len(locator_ids)
            if locator_count != census_count:
                violation("coverage", "locators " + str(locator_count) + " differ from census " + str(census_count) + " for: " + file_field)
            locator_known = {}
            for region_id in locator_ids:
                seen_before = region_id in locator_known
                if seen_before:
                    violation("coverage", "duplicate evidence locator: " + region_id)
                locator_known.update({region_id: True})
            for claimed_id in claimed_ids:
                anchored = claimed_id in locator_known
                if not anchored:
                    violation("coverage", "coverage row without evidence region: " + claimed_id)
            evidence_locator_payloads.update({file_field: payload_by_locator})
        else:
            payload_by_ordinal = {}
            ordinal_counter = 0
            for ordered_payload in ordered_payloads:
                payload_by_ordinal.update({ordinal_counter: ordered_payload})
                ordinal_counter = ordinal_counter + 1
            evidence_ordinal_payloads.update({file_field: payload_by_ordinal})
    payload_text_by_docid = {}
    for docid in docids:
        ace_file = ace_file_by_docid.get(docid, "")
        locator_map = evidence_locator_payloads.get(ace_file, None)
        if locator_map != None:
            region_key = ace_region_by_docid.get(docid)
            payload_text = locator_map.get(region_key, None)
            if payload_text != None:
                payload_text_by_docid.update({docid: payload_text})
        else:
            ordinal_map = evidence_ordinal_payloads.get(ace_file, None)
            if ordinal_map != None:
                ordinal_key = ace_ordinal_by_docid.get(docid)
                payload_text = ordinal_map.get(ordinal_key, None)
                if payload_text != None:
                    payload_text_by_docid.update({docid: payload_text})
    meter = "goal: coverage ok " + guideline_path.name + " " + str(row_count) + " regions; ace=" + str(ace_count) + " restates=" + str(restates_count) + " uncovered=" + str(uncovered_count) + " pending=" + str(pending_count)
    if emit_meter:
        print(meter)
    return [status_by_id, ace_row_line_by_docid, payload_text_by_docid]
def lexicon_entry(line_text):
    head_parts = line_text.split("(", 1)
    kind = head_parts.pop(0)
    if len(head_parts) != 1:
        violation("lexicon-entry", "malformed entry: " + line_text)
    rest = head_parts.pop(0)
    quoted = rest.split("'")
    surface = ""
    lemma = ""
    if len(quoted) > 3:
        quoted.pop(0)
        surface = quoted.pop(0)
        quoted.pop(0)
        lemma = quoted.pop(0)
    else:
        plain = rest.split(",")
        if len(plain) < 2:
            violation("lexicon-entry", "malformed entry: " + line_text)
        surface = plain.pop(0)
        lemma = plain.pop(0)
    return [kind, surface, lemma]
def ace_token_set(ace_paths, docids):
    tokens = {}
    for docid in docids:
        ace_path = ace_paths.get(docid)
        text = ace_path.read_text(encoding="utf-8")
        for raw_token in text.split():
            token = raw_token.strip(token_strip_chars)
            tokens.update({token: True})
    return tokens
def normalized_lexicon_entry(line_text):
    text = line_text.strip()
    text = text.removesuffix(".")
    text = text.replace("'", "")
    text = text.replace(" ", "")
    text = text.replace("\t", "")
    return text
def clex_entry_maps():
    clex_path = pathlib.Path(clex_path_text)
    data = read_corpus_file(clex_path, "clex-file")
    text = data.decode("utf-8")
    entries = {}
    surfaces = {}
    for raw_line in text.split("\n"):
        line_text = raw_line.strip()
        if line_text:
            is_comment = line_text.startswith("%")
            is_directive = line_text.startswith(":-")
            if not is_comment:
                if not is_directive:
                    normalized = normalized_lexicon_entry(line_text)
                    entries.update({normalized: True})
                    fact = lexicon_entry(line_text)
                    fact_copy = list(fact)
                    fact_kind = fact_copy.pop(0)
                    fact_surface = fact_copy.pop(0)
                    surface_known = fact_surface in surfaces
                    if not surface_known:
                        surfaces.update({fact_surface: []})
                    surface_list = surfaces.get(fact_surface)
                    fact_present = normalized in surface_list
                    if not fact_present:
                        surface_list.append(normalized)
    return [entries, surfaces]
shadow_ruling_header = "ulex_entry\tclex_entries\truling"
def lexicon_shadow_rulings(guideline_path):
    rulings_path = guideline_path.joinpath("audit", "lexicon-shadow.tsv")
    rulings = {}
    if rulings_path.is_symlink():
        violation("lexicon-shadow-file", "is a symlink: " + str(rulings_path))
    if not rulings_path.is_file():
        return rulings
    data = read_corpus_file(rulings_path, "lexicon-shadow-file")
    text = data.decode("utf-8")
    line_list = text.split("\n")
    header_line = line_list.pop(0)
    if header_line != shadow_ruling_header:
        violation("lexicon-shadow-file", "first line is not the 3-column header: " + str(rulings_path))
    for line_text in line_list:
        if line_text:
            cell_list = line_text.split("\t")
            if len(cell_list) != 3:
                violation("lexicon-shadow-file", "row without 3 cells: " + line_text)
            ulex_cell = cell_list.pop(0)
            clex_cell = cell_list.pop(0)
            ruling_cell = cell_list.pop(0)
            if not ulex_cell:
                violation("lexicon-shadow-file", "empty ulex_entry cell: " + line_text)
            if not clex_cell:
                violation("lexicon-shadow-file", "empty clex_entries cell: " + line_text)
            if not ruling_cell:
                violation("lexicon-shadow-file", "empty ruling for: " + ulex_cell)
            row_key = ulex_cell + "\t" + clex_cell
            dup_row = row_key in rulings
            if dup_row:
                violation("lexicon-shadow-file", "duplicate ruling row: " + ulex_cell)
            rulings.update({row_key: False})
    return rulings
def check_lexicon(guideline_path, ace_paths, docids):
    lexicon_path = guideline_path.joinpath("lexicon.ulex")
    data = read_corpus_file(lexicon_path, "lexicon-file")
    text = data.decode("utf-8")
    all_tokens = ace_token_set(ace_paths, docids)
    clex_maps = clex_entry_maps()
    clex_entries = clex_maps.pop(0)
    clex_surfaces = clex_maps.pop(0)
    shadow_rulings = lexicon_shadow_rulings(guideline_path)
    ruled_count = 0
    entries = []
    seen_entries = {}
    live_lexemes = {}
    for raw_line in text.split("\n"):
        line_text = raw_line.strip()
        if line_text:
            normalized = normalized_lexicon_entry(line_text)
            redundant = normalized in clex_entries
            if redundant:
                violation("lexicon-redundant", "clex already provides: " + line_text)
            repeated = normalized in seen_entries
            if repeated:
                violation("lexicon-duplicate", "entry repeated in lexicon: " + line_text)
            seen_entries.update({normalized: True})
            entry = lexicon_entry(line_text)
            entries.append(entry)
            entry_copy = list(entry)
            entry_kind = entry_copy.pop(0)
            entry_surface = entry_copy.pop(0)
            entry_lemma = entry_copy.pop(0)
            covered = entry_surface in clex_surfaces
            if covered:
                clex_lines = clex_surfaces.get(entry_surface)
                clex_key = ""
                for clex_line in sorted(clex_lines):
                    if clex_key:
                        clex_key = clex_key + "+" + clex_line
                    else:
                        clex_key = clex_line
                shadow_key = normalized + "\t" + clex_key
                ruled = shadow_key in shadow_rulings
                if ruled:
                    shadow_rulings.update({shadow_key: True})
                    ruled_count = ruled_count + 1
                else:
                    violation("lexicon-shadow", "clex shares surface without ruling: " + line_text + " vs " + clex_key)
            referenced = entry_surface in all_tokens
            if referenced:
                live_lexemes.update({entry_lemma: True})
    for row_key in shadow_rulings:
        row_used = shadow_rulings.get(row_key)
        if not row_used:
            key_parts = row_key.split("\t")
            ulex_part = key_parts.pop(0)
            violation("lexicon-shadow", "stale ruling matches no live shadow: " + ulex_part)
    for entry in entries:
        entry_copy = list(entry)
        kind = entry_copy.pop(0)
        surface = entry_copy.pop(0)
        lemma = entry_copy.pop(0)
        live = lemma in live_lexemes
        if not live:
            violation("lexicon-dead-lexeme", "no ace document references: " + surface)
    print("goal: lexicon ok " + str(lexicon_path) + " " + str(len(entries)) + " entries " + str(len(clex_entries)) + " clex facts " + str(ruled_count) + " ruled shadows")
def check_product_vocabulary(guideline_path, docid):
    pl_path = guideline_path.joinpath("pl", docid + ".pl")
    data = read_corpus_file(pl_path, "product-vocabulary")
    text = data.decode("utf-8")
    for raw_line in text.split("\n"):
        line_text = raw_line.strip()
        if line_text:
            is_comment = line_text.startswith("%")
            if not is_comment:
                is_directive = line_text.startswith(":- ")
                if is_directive:
                    inner = line_text.removeprefix(":- ")
                    inner = inner.removesuffix(").")
                    decl_parts = inner.split("(", 1)
                    decl_kind = decl_parts.pop(0)
                    if len(decl_parts) != 1:
                        violation("product-vocabulary", "unauthorized directive in " + docid + ": " + line_text)
                    decl_spec = decl_parts.pop(0)
                    kind_known = decl_kind in v1_decl_kinds
                    if not kind_known:
                        violation("product-vocabulary", "unauthorized directive in " + docid + ": " + line_text)
                    spec_known = decl_spec in v1_directives
                    if not spec_known:
                        violation("product-vocabulary", "undeclared indicator in " + docid + ": " + decl_spec)
                else:
                    functor_parts = line_text.split("(", 1)
                    functor = functor_parts.pop(0)
                    functor_known = functor in v1_functors
                    if not functor_known:
                        violation("product-vocabulary", "unauthorized clause functor in " + docid + ": " + functor)
adjudication_manifest_header_1 = "# format: docid<TAB>ace_sha256<TAB>coverage_row_sha256<TAB>region_payload_sha256<TAB>semantic_clause_sha256<TAB>review_sha256"
adjudication_manifest_header_2 = "# bundle v2; review_sha256 = sha256 of the labeled component-digest block; regenerate: python3 -P tools/goal.py review-manifest <id>; do not edit."
adjudication_ledger_header = "# format: docid<TAB>review_sha256<TAB>ace_commit<TAB>verdict<TAB>reviewer<TAB>date<TAB>comment"
manifest_component_names = ["ace_sha256", "coverage_row_sha256", "region_payload_sha256", "semantic_clause_sha256"]
def sha256_hex(data):
    digest_value = hashlib.sha256(data)
    return digest_value.hexdigest()
def valid_digest(digest_text):
    if len(digest_text) != 64:
        return False
    allowed = set("0123456789abcdef")
    chars = set(digest_text)
    return chars.issubset(allowed)
def valid_commit_field(commit_text):
    if commit_text == "":
        return True
    if len(commit_text) != 40:
        return False
    allowed = set("0123456789abcdef")
    chars = set(commit_text)
    return chars.issubset(allowed)
def semantic_clause_digest(pl_path, docid):
    data = read_corpus_file(pl_path, "adjudication")
    text = ""
    try:
        text = data.decode("utf-8")
    except UnicodeDecodeError:
        violation("adjudication", "compiled document encoding: " + docid)
    if not text.endswith("\n"):
        violation("adjudication", "compiled document lacks final newline: " + docid)
    line_list = text.split("\n")
    line_list.pop()
    record_count = 0
    retained_text = ""
    for line_text in line_list:
        is_record = line_text.startswith("guideline_document(")
        if is_record:
            record_count = record_count + 1
            if not line_text.endswith("."):
                violation("adjudication", "noncanonical document record in: " + docid)
        else:
            is_comment = line_text.startswith("%")
            is_directive = line_text.startswith(":- ")
            if not is_comment:
                if not is_directive:
                    if not line_text.endswith("."):
                        violation("adjudication", "noncanonical clause line in: " + docid)
                    retained_text = retained_text + line_text + "\n"
    if record_count != 1:
        violation("adjudication", "document record count " + str(record_count) + " for: " + docid)
    return sha256_hex(retained_text.encode("utf-8"))
def bundle_digest(docid, components):
    component_copy = list(components)
    ace_digest = component_copy.pop(0)
    coverage_digest = component_copy.pop(0)
    payload_digest = component_copy.pop(0)
    clause_digest = component_copy.pop(0)
    block_text = "bundle v2 " + docid + "\n"
    block_text = block_text + "ace " + ace_digest + "\n"
    block_text = block_text + "coverage " + coverage_digest + "\n"
    block_text = block_text + "payload " + payload_digest + "\n"
    block_text = block_text + "clauses " + clause_digest + "\n"
    return sha256_hex(block_text.encode("utf-8"))
def derive_review_manifest(guideline_path, docids, ace_row_line_by_docid, payload_text_by_docid):
    sorted_docids = sorted(docids)
    manifest_text = adjudication_manifest_header_1 + "\n" + adjudication_manifest_header_2 + "\n"
    bundle_by_docid = {}
    for docid in sorted_docids:
        ace_path = guideline_path.joinpath("ace", docid + ".ace")
        ace_bytes = read_corpus_file(ace_path, "adjudication")
        ace_digest = sha256_hex(ace_bytes)
        coverage_row = ace_row_line_by_docid.get(docid, None)
        if coverage_row == None:
            violation("adjudication", "docid without coverage row bytes: " + docid)
        coverage_digest = sha256_hex(coverage_row.encode("utf-8"))
        payload_text = payload_text_by_docid.get(docid, None)
        if payload_text == None:
            violation("adjudication", "docid without region payload: " + docid)
        payload_digest = sha256_hex(payload_text.encode("utf-8"))
        pl_path = guideline_path.joinpath("pl", docid + ".pl")
        clause_digest = semantic_clause_digest(pl_path, docid)
        components = [ace_digest, coverage_digest, payload_digest, clause_digest]
        review_digest = bundle_digest(docid, components)
        row_text = docid + "\t" + ace_digest + "\t" + coverage_digest + "\t" + payload_digest + "\t" + clause_digest + "\t" + review_digest
        manifest_text = manifest_text + row_text + "\n"
        bundle_by_docid.update({docid: review_digest})
    return [manifest_text, bundle_by_docid]
def parse_review_manifest(data, manifest_path):
    text = ""
    try:
        text = data.decode("utf-8")
    except UnicodeDecodeError:
        violation("adjudication", "manifest encoding")
    if "\r" in text:
        violation("adjudication", "manifest carriage-return")
    if not text.endswith("\n"):
        violation("adjudication", "manifest final-newline")
    header_1 = adjudication_manifest_header_1 + "\n"
    if not text.startswith(header_1):
        violation("adjudication", "manifest header line 1")
    remainder = text.removeprefix(header_1)
    header_2 = adjudication_manifest_header_2 + "\n"
    if not remainder.startswith(header_2):
        violation("adjudication", "manifest header line 2")
    body = remainder.removeprefix(header_2)
    row_lines = body.split("\n")
    row_lines.pop()
    if not row_lines:
        violation("adjudication", "manifest holds no rows: " + str(manifest_path))
    bundle_by_docid = {}
    row_number = 2
    prev_docid = ""
    for row_line in row_lines:
        row_number = row_number + 1
        fields = row_line.split("\t")
        field_count = len(fields)
        if field_count != 6:
            violation("adjudication", "manifest row " + str(row_number) + " field-count " + str(field_count))
        docid = fields.pop(0)
        if not valid_docid(docid):
            violation("adjudication", "manifest row " + str(row_number) + " docid-grammar")
        duplicate = docid in bundle_by_docid
        if duplicate:
            violation("adjudication", "manifest row " + str(row_number) + " duplicate-docid " + docid)
        if docid < prev_docid:
            violation("adjudication", "manifest row " + str(row_number) + " sort-order " + docid + " after " + prev_docid)
        prev_docid = docid
        component_values = []
        for component_name in manifest_component_names:
            component_value = fields.pop(0)
            if not valid_digest(component_value):
                violation("adjudication", "manifest row " + str(row_number) + " " + component_name)
            component_values.append(component_value)
        review_value = fields.pop(0)
        if not valid_digest(review_value):
            violation("adjudication", "manifest row " + str(row_number) + " review_sha256")
        recomputed = bundle_digest(docid, component_values)
        if recomputed != review_value:
            violation("adjudication", "manifest row " + str(row_number) + " review_sha256 self-consistency")
        bundle_by_docid.update({docid: review_value})
    return bundle_by_docid
def reviewer_text_ok(field_text):
    for char_text in field_text:
        code_point = ord(char_text)
        if code_point < 32:
            return False
        if code_point == 127:
            return False
    return True
def valid_review_date(date_text):
    if len(date_text) != 20:
        return False
    digit_chars = set("0123456789")
    position = 0
    for char_text in date_text:
        expected = ""
        if position == 4:
            expected = "-"
        if position == 7:
            expected = "-"
        if position == 10:
            expected = "T"
        if position == 13:
            expected = ":"
        if position == 16:
            expected = ":"
        if position == 19:
            expected = "Z"
        if expected:
            if char_text != expected:
                return False
        else:
            is_digit = char_text in digit_chars
            if not is_digit:
                return False
        position = position + 1
    try:
        datetime.datetime.strptime(date_text, "%Y-%m-%dT%H:%M:%SZ")
    except ValueError:
        return False
    return True
def derive_bundles_at_commit(gid, commit_field):
    scratch_text = tempfile.mkdtemp()
    archive_result = subprocess.run(["git", "archive", commit_field, "--", "guidelines/" + gid], capture_output=True)
    if archive_result.returncode != 0:
        shutil.rmtree(scratch_text, ignore_errors=True)
        violation("adjudication", "ledger commit lacks guideline tree: " + gid + " " + commit_field)
    tar_result = subprocess.run(["tar", "-x", "-C", scratch_text], input=archive_result.stdout, capture_output=True)
    if tar_result.returncode != 0:
        shutil.rmtree(scratch_text, ignore_errors=True)
        violation("adjudication", "ledger commit tree extraction failed: " + gid + " " + commit_field)
    scratch_root = pathlib.Path(scratch_text)
    commit_guideline_path = scratch_root.joinpath("guidelines", gid)
    collected = collect_guideline(commit_guideline_path)
    ace_paths = collected.pop(0)
    docids = collected.pop(0)
    lexicon_path = collected.pop(0)
    coverage_result = check_coverage(commit_guideline_path, docids, False)
    status_by_id = coverage_result.pop(0)
    ace_row_line_by_docid = coverage_result.pop(0)
    payload_text_by_docid = coverage_result.pop(0)
    derived = derive_review_manifest(commit_guideline_path, docids, ace_row_line_by_docid, payload_text_by_docid)
    manifest_text = derived.pop(0)
    bundle_by_docid = derived.pop(0)
    shutil.rmtree(scratch_text, ignore_errors=True)
    return bundle_by_docid
def check_ledger_commit_row(gid, row_number, docid, commit_field, digest_field, current_digest, commit_bundle_cache):
    exists_result = subprocess.run(["git", "cat-file", "-e", commit_field + "^{commit}"], capture_output=True)
    if exists_result.returncode != 0:
        violation("adjudication", "ledger row " + str(row_number) + " commit absent from repository: " + commit_field)
    if digest_field == current_digest:
        return 0
    bundle_at_commit = commit_bundle_cache.get(commit_field, None)
    if bundle_at_commit == None:
        bundle_at_commit = derive_bundles_at_commit(gid, commit_field)
        commit_bundle_cache.update({commit_field: bundle_at_commit})
    historical = bundle_at_commit.get(docid, "")
    if not historical:
        violation("adjudication", "ledger row " + str(row_number) + " docid absent at recorded commit: " + docid + " " + commit_field)
    if historical != digest_field:
        violation("adjudication", "ledger row " + str(row_number) + " digest mismatch at recorded commit: " + docid + " " + commit_field)
def validate_ledger(ledger_path, bundle_by_docid, label, odb_check):
    manifest_total = len(bundle_by_docid)
    decision_count = 0
    seen_row_docids = {}
    commit_bundle_cache = {}
    current_approved = {}
    current_rejected = {}
    if ledger_path.is_symlink():
        violation("adjudication", "ledger is a symlink: " + str(ledger_path))
    ledger_exists = ledger_path.exists()
    if ledger_exists:
        if not ledger_path.is_file():
            violation("adjudication", "ledger is not a regular file: " + str(ledger_path))
        data = ledger_path.read_bytes()
        text = ""
        try:
            text = data.decode("utf-8")
        except UnicodeDecodeError:
            violation("adjudication", "ledger encoding")
        if "\r" in text:
            violation("adjudication", "ledger carriage-return")
        if not text.endswith("\n"):
            violation("adjudication", "ledger final-newline")
        header_line = adjudication_ledger_header + "\n"
        if not text.startswith(header_line):
            violation("adjudication", "ledger header")
        body = text.removeprefix(header_line)
        row_lines = body.split("\n")
        row_lines.pop()
        row_number = 1
        prev_docid = ""
        prev_date = ""
        prev_key = ""
        for row_line in row_lines:
            row_number = row_number + 1
            if row_line.startswith("#"):
                violation("adjudication", "ledger header")
            fields = row_line.split("\t")
            field_count = len(fields)
            if field_count != 7:
                violation("adjudication", "ledger row " + str(row_number) + " field-count " + str(field_count))
            docid = fields.pop(0)
            digest_field = fields.pop(0)
            commit_field = fields.pop(0)
            verdict_field = fields.pop(0)
            reviewer_field = fields.pop(0)
            date_field = fields.pop(0)
            comment_field = fields.pop(0)
            if not valid_docid(docid):
                violation("adjudication", "ledger row " + str(row_number) + " docid-grammar")
            known = docid in bundle_by_docid
            if not known:
                violation("adjudication", "ledger row " + str(row_number) + " unknown-docid " + docid)
            order_key = docid + "\t" + date_field
            if order_key < prev_key:
                violation("adjudication", "ledger row " + str(row_number) + " sort-order " + docid + " " + date_field + " after " + prev_docid + " " + prev_date)
            prev_docid = docid
            prev_date = date_field
            prev_key = order_key
            seen_row_docids.update({docid: True})
            if not valid_digest(digest_field):
                violation("adjudication", "ledger row " + str(row_number) + " hex")
            if not valid_commit_field(commit_field):
                violation("adjudication", "ledger row " + str(row_number) + " ace-commit")
            verdict_ok = False
            if verdict_field == "approved":
                verdict_ok = True
            if verdict_field == "rejected":
                verdict_ok = True
            if not verdict_ok:
                violation("adjudication", "ledger row " + str(row_number) + " verdict")
            if not reviewer_field:
                violation("adjudication", "ledger row " + str(row_number) + " reviewer")
            if not reviewer_text_ok(reviewer_field):
                violation("adjudication", "ledger row " + str(row_number) + " reviewer")
            if not valid_review_date(date_field):
                violation("adjudication", "ledger row " + str(row_number) + " date")
            if not reviewer_text_ok(comment_field):
                violation("adjudication", "ledger row " + str(row_number) + " comment")
            current_digest = bundle_by_docid.get(docid)
            if odb_check:
                if commit_field:
                    check_ledger_commit_row(label, row_number, docid, commit_field, digest_field, current_digest, commit_bundle_cache)
            decision_count = decision_count + 1
            if digest_field == current_digest:
                if verdict_field == "approved":
                    current_approved.update({docid: True})
                else:
                    current_rejected.update({docid: True})
    approved_count = 0
    rejected_count = 0
    contested_count = 0
    stale_count = 0
    for row_docid in seen_row_docids:
        has_approved = row_docid in current_approved
        has_rejected = row_docid in current_rejected
        if has_approved:
            if has_rejected:
                contested_count = contested_count + 1
            else:
                approved_count = approved_count + 1
        else:
            if has_rejected:
                rejected_count = rejected_count + 1
            else:
                stale_count = stale_count + 1
    unreviewed_count = manifest_total - len(seen_row_docids)
    meter = "goal: adjudication " + label + " approved=" + str(approved_count) + " rejected=" + str(rejected_count) + " contested=" + str(contested_count) + " stale=" + str(stale_count) + " unreviewed=" + str(unreviewed_count) + " decisions=" + str(decision_count)
    print(meter)
def check_adjudication(guideline_path, docids, ace_row_line_by_docid, payload_text_by_docid):
    derived = derive_review_manifest(guideline_path, docids, ace_row_line_by_docid, payload_text_by_docid)
    manifest_text = derived.pop(0)
    bundle_by_docid = derived.pop(0)
    manifest_path = guideline_path.joinpath("audit", "review-manifest.tsv")
    regen_hint = "; regenerate: python3 -P tools/goal.py review-manifest " + guideline_path.name
    if manifest_path.is_symlink():
        violation("adjudication", "manifest is a symlink: " + str(manifest_path))
    if not manifest_path.exists():
        violation("adjudication", "manifest missing: " + str(manifest_path) + regen_hint)
    if not manifest_path.is_file():
        violation("adjudication", "manifest is not a regular file: " + str(manifest_path))
    committed_bytes = manifest_path.read_bytes()
    derived_bytes = manifest_text.encode("utf-8")
    if committed_bytes != derived_bytes:
        violation("adjudication", "manifest stale: " + str(manifest_path) + regen_hint)
    ledger_path = guideline_path.joinpath("audit", "adjudication.tsv")
    validate_ledger(ledger_path, bundle_by_docid, guideline_path.name, True)
def review_manifest_command(guideline_id):
    if not valid_docid(guideline_id):
        fail("guideline", "invalid guideline id: " + guideline_id)
    guidelines_root = pathlib.Path("guidelines")
    guideline_path = guidelines_root.joinpath(guideline_id)
    guideline_symlink = guideline_path.is_symlink()
    if guideline_symlink:
        fail("guideline", "is a symlink: " + str(guideline_path))
    if not guideline_path.is_dir():
        fail("guideline", "not a directory: " + str(guideline_path))
    collected = collect_guideline(guideline_path)
    ace_paths = collected.pop(0)
    docids = collected.pop(0)
    lexicon_path = collected.pop(0)
    coverage_result = check_coverage(guideline_path, docids, False)
    status_by_id = coverage_result.pop(0)
    ace_row_line_by_docid = coverage_result.pop(0)
    payload_text_by_docid = coverage_result.pop(0)
    derived = derive_review_manifest(guideline_path, docids, ace_row_line_by_docid, payload_text_by_docid)
    manifest_text = derived.pop(0)
    bundle_by_docid = derived.pop(0)
    manifest_path = guideline_path.joinpath("audit", "review-manifest.tsv")
    if manifest_path.is_symlink():
        violation("adjudication", "manifest is a symlink: " + str(manifest_path))
    target_present = manifest_path.exists()
    if target_present:
        if not manifest_path.is_file():
            violation("adjudication", "manifest is not a regular file: " + str(manifest_path))
    manifest_path.write_bytes(manifest_text.encode("utf-8"))
    print("goal: review-manifest " + guideline_id + " " + str(len(docids)) + " documents")
def derive_review_manifest_command(guideline_dir):
    guideline_path = pathlib.Path(guideline_dir)
    guideline_symlink = guideline_path.is_symlink()
    if guideline_symlink:
        fail("guideline", "is a symlink: " + str(guideline_path))
    if not guideline_path.is_dir():
        fail("guideline", "not a directory: " + str(guideline_path))
    collected = collect_guideline(guideline_path)
    ace_paths = collected.pop(0)
    docids = collected.pop(0)
    lexicon_path = collected.pop(0)
    coverage_result = check_coverage(guideline_path, docids, False)
    status_by_id = coverage_result.pop(0)
    ace_row_line_by_docid = coverage_result.pop(0)
    payload_text_by_docid = coverage_result.pop(0)
    derived = derive_review_manifest(guideline_path, docids, ace_row_line_by_docid, payload_text_by_docid)
    manifest_text = derived.pop(0)
    bundle_by_docid = derived.pop(0)
    manifest_bytes = manifest_text.encode("utf-8")
    sys.stdout.buffer.write(manifest_bytes)
    sys.stdout.buffer.flush()
def ledger_validate_command(ledger_arg, manifest_arg, label):
    if not valid_docid(label):
        fail("usage", "label must match [a-z0-9-]+: " + label)
    manifest_path = pathlib.Path(manifest_arg)
    if manifest_path.is_symlink():
        violation("adjudication", "manifest is a symlink: " + str(manifest_path))
    if not manifest_path.exists():
        violation("adjudication", "manifest missing: " + str(manifest_path))
    if not manifest_path.is_file():
        violation("adjudication", "manifest is not a regular file: " + str(manifest_path))
    data = manifest_path.read_bytes()
    bundle_by_docid = parse_review_manifest(data, manifest_path)
    ledger_path = pathlib.Path(ledger_arg)
    validate_ledger(ledger_path, bundle_by_docid, label, False)
def check_adjudication_fixtures():
    fixtures_root = pathlib.Path("tests/adjudication")
    if fixtures_root.is_symlink():
        violation("adjudication-fixtures", "is a symlink: " + str(fixtures_root))
    if not fixtures_root.is_dir():
        violation("adjudication-fixtures", "missing: " + str(fixtures_root))
    case_names = []
    for entry in sorted(fixtures_root.iterdir()):
        entry_name = entry.name
        if entry.is_symlink():
            violation("adjudication-fixtures", "not a case directory: " + entry_name)
        if not entry.is_dir():
            violation("adjudication-fixtures", "not a case directory: " + entry_name)
        if not valid_docid(entry_name):
            violation("adjudication-fixtures", "invalid case name: " + entry_name)
        case_names.append(entry_name)
    if not case_names:
        violation("adjudication-fixtures", "no fixture cases found: " + str(fixtures_root))
    member_names = ["expect", "golden", "ledger.tsv", "manifest.tsv"]
    red_count = 0
    green_count = 0
    for case_name in case_names:
        case_path = fixtures_root.joinpath(case_name)
        has_manifest = False
        has_expect = False
        has_golden = False
        for member in sorted(case_path.iterdir()):
            member_name = member.name
            member_known = member_name in member_names
            if not member_known:
                violation("adjudication-fixtures", "unsupported entry: " + case_name + "/" + member_name)
            if member.is_symlink():
                violation("adjudication-fixtures", "not a regular file: " + case_name + "/" + member_name)
            if not member.is_file():
                violation("adjudication-fixtures", "not a regular file: " + case_name + "/" + member_name)
            if member_name == "manifest.tsv":
                has_manifest = True
            if member_name == "expect":
                has_expect = True
            if member_name == "golden":
                has_golden = True
        if not has_manifest:
            violation("adjudication-fixtures", "case without manifest.tsv: " + case_name)
        if has_expect:
            if has_golden:
                violation("adjudication-fixtures", "case pins both expect and golden: " + case_name)
        if not has_expect:
            if not has_golden:
                violation("adjudication-fixtures", "case without expect or golden pin: " + case_name)
        ledger_arg = str(case_path.joinpath("ledger.tsv"))
        manifest_arg = str(case_path.joinpath("manifest.tsv"))
        command = [sys.executable, "-P", "tools/goal.py", "ledger-validate", ledger_arg, manifest_arg, "fx"]
        result = subprocess.run(command, capture_output=True)
        if has_expect:
            if result.returncode != 1:
                violation("adjudication-fixtures", "status " + str(result.returncode) + " for case: " + case_name)
            if result.stderr:
                violation("adjudication-fixtures", "non-empty stderr for case: " + case_name)
            expect_path = case_path.joinpath("expect")
            expect_bytes = expect_path.read_bytes()
            if result.stdout != expect_bytes:
                violation("adjudication-fixtures", "stdout differs from expect pin for case: " + case_name)
            red_count = red_count + 1
        else:
            if result.returncode != 0:
                violation("adjudication-fixtures", "status " + str(result.returncode) + " for case: " + case_name)
            if result.stderr:
                violation("adjudication-fixtures", "non-empty stderr for case: " + case_name)
            golden_path = case_path.joinpath("golden")
            golden_bytes = golden_path.read_bytes()
            if result.stdout != golden_bytes:
                violation("adjudication-fixtures", "stdout differs from golden for case: " + case_name)
            green_count = green_count + 1
    if red_count != 42:
        violation("adjudication-fixtures", "red case count drift: expected 42 got " + str(red_count))
    if green_count != 9:
        violation("adjudication-fixtures", "green case count drift: expected 9 got " + str(green_count))
    print("goal: adjudication fixtures ok " + str(red_count) + " red " + str(green_count) + " green")
def check_queries_fixtures(scratch_path, swipl_executable, stage_path):
    fixtures_root = pathlib.Path("tests/queries")
    if fixtures_root.is_symlink():
        cleanup_violation(scratch_path, "queries-fixtures", "is a symlink: " + str(fixtures_root))
    if not fixtures_root.is_dir():
        cleanup_violation(scratch_path, "queries-fixtures", "missing: " + str(fixtures_root))
    color_names = ["red", "green"]
    for entry in sorted(fixtures_root.iterdir()):
        entry_known = entry.name in color_names
        if not entry_known:
            cleanup_violation(scratch_path, "queries-fixtures", "unsupported entry: " + entry.name)
    red_count = 0
    green_count = 0
    red_names = []
    green_names = []
    pin_inventory = []
    red_required = ["bad-qid", "empty-solutions", "limit-depth", "limit-inner-inference", "malformed-query-file", "missing-trace", "no-finite-failure", "orphan-answers", "orphan-pl", "orphan-trace", "stale-answers", "stale-pl", "stale-trace", "trace-digest-join", "trace-indeterminate-mirror", "trace-naf-depth-cut", "trace-naf-inference-cut", "trace-naf-proved", "trace-no-mirror", "trace-row-failure-after-cut", "trace-unproved-finite", "trace-unproved-limit", "uncompiled-ace", "yesno-limit-before-proof"]
    green_required = ["absent-queries", "canonical-sort", "depth-backtrack", "empty-queries", "numeric-bigint", "positive", "trace-direct-rejects", "trace-multi-proof", "trace-naf", "trace-positive-rule", "trace-serializer"]
    for color in color_names:
        color_path = fixtures_root.joinpath(color)
        if color_path.is_symlink():
            cleanup_violation(scratch_path, "queries-fixtures", "is a symlink: " + str(color_path))
        if not color_path.is_dir():
            cleanup_violation(scratch_path, "queries-fixtures", "missing: " + str(color_path))
        for case_entry in sorted(color_path.iterdir()):
            case_name = case_entry.name
            if case_entry.is_symlink():
                cleanup_violation(scratch_path, "queries-fixtures", "not a case directory: " + case_name)
            if not case_entry.is_dir():
                cleanup_violation(scratch_path, "queries-fixtures", "not a case directory: " + case_name)
            if not valid_docid(case_name):
                cleanup_violation(scratch_path, "queries-fixtures", "invalid case name: " + case_name)
            if color == "red":
                red_names.append(case_name)
            else:
                green_names.append(case_name)
            has_tree = False
            has_expect = False
            has_golden = False
            has_answers_golden = False
            has_traces_golden = False
            has_trace_reject = False
            for member in sorted(case_entry.iterdir()):
                member_name = member.name
                if member.is_symlink():
                    cleanup_violation(scratch_path, "queries-fixtures", "unsupported entry: " + case_name + "/" + member_name)
                if member_name == "tree":
                    if not member.is_dir():
                        cleanup_violation(scratch_path, "queries-fixtures", "unsupported entry: " + case_name + "/" + member_name)
                    has_tree = True
                elif member_name == "answers-golden":
                    if not member.is_dir():
                        cleanup_violation(scratch_path, "queries-fixtures", "unsupported entry: " + case_name + "/" + member_name)
                    has_answers_golden = True
                elif member_name == "traces-golden":
                    if not member.is_dir():
                        cleanup_violation(scratch_path, "queries-fixtures", "unsupported entry: " + case_name + "/" + member_name)
                    has_traces_golden = True
                elif member_name == "trace-reject":
                    if not member.is_dir():
                        cleanup_violation(scratch_path, "queries-fixtures", "unsupported entry: " + case_name + "/" + member_name)
                    has_trace_reject = True
                elif member_name == "expect":
                    if not member.is_file():
                        cleanup_violation(scratch_path, "queries-fixtures", "unsupported entry: " + case_name + "/" + member_name)
                    has_expect = True
                elif member_name == "golden":
                    if not member.is_file():
                        cleanup_violation(scratch_path, "queries-fixtures", "unsupported entry: " + case_name + "/" + member_name)
                    has_golden = True
                else:
                    cleanup_violation(scratch_path, "queries-fixtures", "unsupported entry: " + case_name + "/" + member_name)
            if not has_tree:
                cleanup_violation(scratch_path, "queries-fixtures", "case without tree: " + case_name)
            if has_expect:
                if has_golden:
                    cleanup_violation(scratch_path, "queries-fixtures", "case pins both expect and golden: " + case_name)
            if not has_expect:
                if not has_golden:
                    cleanup_violation(scratch_path, "queries-fixtures", "case without expect or golden pin: " + case_name)
            if color == "red":
                if has_golden:
                    cleanup_violation(scratch_path, "queries-fixtures", "red case pins golden: " + case_name)
            if color == "green":
                if has_expect:
                    cleanup_violation(scratch_path, "queries-fixtures", "green case pins expect: " + case_name)
            tree_guidelines = case_entry.joinpath("tree", "guidelines")
            if not tree_guidelines.is_dir():
                cleanup_violation(scratch_path, "queries-fixtures", "case tree without guidelines: " + case_name)
            tree_ids = []
            for tree_entry in sorted(tree_guidelines.iterdir()):
                tree_ids.append(tree_entry)
            if len(tree_ids) != 1:
                cleanup_violation(scratch_path, "queries-fixtures", "case tree without one guideline: " + case_name)
            gid_path = tree_ids.pop(0)
            if not gid_path.is_dir():
                cleanup_violation(scratch_path, "queries-fixtures", "case tree without one guideline: " + case_name)
            if not valid_docid(gid_path.name):
                cleanup_violation(scratch_path, "queries-fixtures", "case tree without one guideline: " + case_name)
            command = [sys.executable, "-P", "tools/goal.py", "queries-check", str(gid_path), str(stage_path)]
            result = subprocess.run(command, capture_output=True)
            if has_expect:
                if result.returncode != 1:
                    cleanup_violation(scratch_path, "queries-fixtures", "status " + str(result.returncode) + " for case: " + case_name)
                if result.stderr:
                    cleanup_violation(scratch_path, "queries-fixtures", "non-empty stderr for case: " + case_name)
                expect_path = case_entry.joinpath("expect")
                expect_bytes = expect_path.read_bytes()
                if result.stdout != expect_bytes:
                    cleanup_violation(scratch_path, "queries-fixtures", "stdout differs from expect pin for case: " + case_name)
                red_count = red_count + 1
            else:
                if result.returncode != 0:
                    cleanup_violation(scratch_path, "queries-fixtures", "status " + str(result.returncode) + " for case: " + case_name)
                if result.stderr:
                    cleanup_violation(scratch_path, "queries-fixtures", "non-empty stderr for case: " + case_name)
                golden_path = case_entry.joinpath("golden")
                golden_bytes = golden_path.read_bytes()
                if result.stdout != golden_bytes:
                    cleanup_violation(scratch_path, "queries-fixtures", "stdout differs from golden for case: " + case_name)
                green_count = green_count + 1
            any_pin_lane = False
            if has_answers_golden:
                any_pin_lane = True
            if has_traces_golden:
                any_pin_lane = True
            if has_trace_reject:
                any_pin_lane = True
            fixture_manifest = scratch_path.joinpath("queries-fixture-manifest-" + color + "-" + case_name)
            if any_pin_lane:
                build_query_manifest(fixture_manifest, gid_path.joinpath("pl"))
            if has_answers_golden:
                pins_dir = case_entry.joinpath("answers-golden")
                for pin in sorted(pins_dir.iterdir()):
                    pin_name = pin.name
                    if pin.is_symlink():
                        cleanup_violation(scratch_path, "queries-fixtures", "unsupported entry: " + case_name + "/answers-golden/" + pin_name)
                    if not pin.is_file():
                        cleanup_violation(scratch_path, "queries-fixtures", "unsupported entry: " + case_name + "/answers-golden/" + pin_name)
                    if not pin_name.endswith(".pl"):
                        cleanup_violation(scratch_path, "queries-fixtures", "unsupported entry: " + case_name + "/answers-golden/" + pin_name)
                    pin_qid = pin_name.removesuffix(".pl")
                    if not valid_docid(pin_qid):
                        cleanup_violation(scratch_path, "queries-fixtures", "unsupported entry: " + case_name + "/answers-golden/" + pin_name)
                    pin_inventory.append(color + "/" + case_name + "/answers-golden/" + pin_qid)
                    query_ace = gid_path.joinpath("queries", pin_qid + ".ace")
                    if not query_ace.is_file():
                        cleanup_violation(scratch_path, "queries-fixtures", "answers-golden qid has no query: " + case_name + "/" + pin_qid)
                    query_pl = gid_path.joinpath("queries", "pl", pin_qid + ".pl")
                    if not query_pl.is_file():
                        cleanup_violation(scratch_path, "queries-fixtures", "answers-golden qid has no query: " + case_name + "/" + pin_qid)
                    answer_bytes = run_answer(scratch_path, swipl_executable, stage_path, pin_qid, fixture_manifest, query_pl)
                    pin_bytes = pin.read_bytes()
                    if answer_bytes != pin_bytes:
                        cleanup_violation(scratch_path, "queries-fixtures", "answer bytes differ from answers-golden: " + case_name + "/" + pin_qid)
            if has_traces_golden:
                pins_dir = case_entry.joinpath("traces-golden")
                for pin in sorted(pins_dir.iterdir()):
                    pin_name = pin.name
                    if pin.is_symlink():
                        cleanup_violation(scratch_path, "queries-fixtures", "unsupported entry: " + case_name + "/traces-golden/" + pin_name)
                    if not pin.is_file():
                        cleanup_violation(scratch_path, "queries-fixtures", "unsupported entry: " + case_name + "/traces-golden/" + pin_name)
                    if not pin_name.endswith(".pl"):
                        cleanup_violation(scratch_path, "queries-fixtures", "unsupported entry: " + case_name + "/traces-golden/" + pin_name)
                    pin_qid = pin_name.removesuffix(".pl")
                    if not valid_docid(pin_qid):
                        cleanup_violation(scratch_path, "queries-fixtures", "unsupported entry: " + case_name + "/traces-golden/" + pin_name)
                    pin_inventory.append(color + "/" + case_name + "/traces-golden/" + pin_qid)
                    query_ace = gid_path.joinpath("queries", pin_qid + ".ace")
                    if not query_ace.is_file():
                        cleanup_violation(scratch_path, "queries-fixtures", "traces-golden qid has no query: " + case_name + "/" + pin_qid)
                    query_pl = gid_path.joinpath("queries", "pl", pin_qid + ".pl")
                    if not query_pl.is_file():
                        cleanup_violation(scratch_path, "queries-fixtures", "traces-golden qid has no query: " + case_name + "/" + pin_qid)
                    answers_pl = gid_path.joinpath("queries", "answers", pin_qid + ".pl")
                    if not answers_pl.is_file():
                        cleanup_violation(scratch_path, "queries-fixtures", "traces-golden qid has no query: " + case_name + "/" + pin_qid)
                    trace_bytes = run_trace(scratch_path, swipl_executable, stage_path, pin_qid, fixture_manifest, query_pl, answers_pl)
                    pin_bytes = pin.read_bytes()
                    if trace_bytes != pin_bytes:
                        cleanup_violation(scratch_path, "queries-fixtures", "trace bytes differ from traces-golden: " + case_name + "/" + pin_qid)
            if has_trace_reject:
                reject_dir = case_entry.joinpath("trace-reject")
                answers_qids = []
                expect_qids = []
                for reject_member in sorted(reject_dir.iterdir()):
                    reject_name = reject_member.name
                    if reject_member.is_symlink():
                        cleanup_violation(scratch_path, "queries-fixtures", "unsupported entry: " + case_name + "/trace-reject/" + reject_name)
                    if not reject_member.is_file():
                        cleanup_violation(scratch_path, "queries-fixtures", "unsupported entry: " + case_name + "/trace-reject/" + reject_name)
                    is_answers = reject_name.endswith(".answers")
                    is_expect = reject_name.endswith(".expect")
                    if not is_answers:
                        if not is_expect:
                            cleanup_violation(scratch_path, "queries-fixtures", "unsupported entry: " + case_name + "/trace-reject/" + reject_name)
                    if is_answers:
                        reject_qid = reject_name.removesuffix(".answers")
                        if not valid_docid(reject_qid):
                            cleanup_violation(scratch_path, "queries-fixtures", "unsupported entry: " + case_name + "/trace-reject/" + reject_name)
                        answers_qids.append(reject_qid)
                    else:
                        reject_qid = reject_name.removesuffix(".expect")
                        if not valid_docid(reject_qid):
                            cleanup_violation(scratch_path, "queries-fixtures", "unsupported entry: " + case_name + "/trace-reject/" + reject_name)
                        expect_qids.append(reject_qid)
                for reject_qid in answers_qids:
                    has_partner = reject_qid in expect_qids
                    if not has_partner:
                        cleanup_violation(scratch_path, "queries-fixtures", "trace-reject member without partner: " + case_name + "/" + reject_qid + ".answers")
                for reject_qid in expect_qids:
                    has_partner = reject_qid in answers_qids
                    if not has_partner:
                        cleanup_violation(scratch_path, "queries-fixtures", "trace-reject member without partner: " + case_name + "/" + reject_qid + ".expect")
                for reject_qid in answers_qids:
                    pin_inventory.append(color + "/" + case_name + "/trace-reject/" + reject_qid)
                    query_ace = gid_path.joinpath("queries", reject_qid + ".ace")
                    if not query_ace.is_file():
                        cleanup_violation(scratch_path, "queries-fixtures", "trace-reject qid has no query: " + case_name + "/" + reject_qid)
                    query_pl = gid_path.joinpath("queries", "pl", reject_qid + ".pl")
                    if not query_pl.is_file():
                        cleanup_violation(scratch_path, "queries-fixtures", "trace-reject qid has no query: " + case_name + "/" + reject_qid)
                    hostile_answers = reject_dir.joinpath(reject_qid + ".answers")
                    reject_tail = ["trace", str(fixture_manifest), str(query_pl), str(hostile_answers)]
                    reject_command = compiler_command(swipl_executable, stage_path, reject_tail)
                    try:
                        reject_result = subprocess.run(reject_command, capture_output=True, timeout=30)
                    except subprocess.TimeoutExpired:
                        cleanup_violation(scratch_path, "queries-fixtures", "wall_clock for trace-reject: " + case_name + "/" + reject_qid)
                    if reject_result.returncode != 2:
                        cleanup_violation(scratch_path, "queries-fixtures", "trace-reject status " + str(reject_result.returncode) + " for case: " + case_name + "/" + reject_qid)
                    if reject_result.stdout:
                        cleanup_violation(scratch_path, "queries-fixtures", "trace-reject stdout not empty for case: " + case_name + "/" + reject_qid)
                    expect_path = reject_dir.joinpath(reject_qid + ".expect")
                    expect_bytes = expect_path.read_bytes()
                    if reject_result.stderr != expect_bytes:
                        cleanup_violation(scratch_path, "queries-fixtures", "trace-reject stderr differs from expect: " + case_name + "/" + reject_qid)
    for required_name in red_required:
        present = required_name in red_names
        if not present:
            cleanup_violation(scratch_path, "queries-fixtures", "missing required case: red/" + required_name)
    for required_name in green_required:
        present = required_name in green_names
        if not present:
            cleanup_violation(scratch_path, "queries-fixtures", "missing required case: green/" + required_name)
    if red_count == 0:
        cleanup_violation(scratch_path, "queries-fixtures", "no red fixture cases found: " + str(fixtures_root))
    if green_count == 0:
        cleanup_violation(scratch_path, "queries-fixtures", "no green fixture cases found: " + str(fixtures_root))
    pin_expected = ["green/canonical-sort/answers-golden/q-sort", "green/depth-backtrack/answers-golden/q-back", "green/numeric-bigint/answers-golden/q-big", "green/numeric-bigint/traces-golden/q-big", "green/positive/answers-golden/q-wh", "green/positive/answers-golden/q-yes", "green/trace-direct-rejects/trace-reject/qid-mismatch", "green/trace-direct-rejects/trace-reject/query-sha256-mismatch", "green/trace-direct-rejects/trace-reject/result-mode-mismatch", "green/trace-direct-rejects/trace-reject/result-shape", "green/trace-direct-rejects/traces-golden/qid-mismatch", "green/trace-direct-rejects/traces-golden/query-sha256-mismatch", "green/trace-direct-rejects/traces-golden/result-mode-mismatch", "green/trace-direct-rejects/traces-golden/result-shape", "green/trace-multi-proof/traces-golden/q-multi", "green/trace-naf/traces-golden/q-naf", "green/trace-positive-rule/traces-golden/q-rule", "green/trace-serializer/traces-golden/q-serializer", "red/empty-solutions/answers-golden/q-empty", "red/empty-solutions/traces-golden/q-empty", "red/limit-depth/answers-golden/q-depth", "red/limit-depth/traces-golden/q-depth", "red/limit-inner-inference/answers-golden/q-inner", "red/limit-inner-inference/traces-golden/q-inner", "red/missing-trace/traces-golden/q-missing-trace", "red/no-finite-failure/answers-golden/q-no", "red/no-finite-failure/traces-golden/q-no", "red/orphan-trace/traces-golden/q-orphan", "red/stale-trace/traces-golden/q-stale-trace", "red/trace-digest-join/traces-golden/q-digest", "red/trace-indeterminate-mirror/traces-golden/q-indeterminate", "red/trace-naf-depth-cut/traces-golden/q-naf-depth", "red/trace-naf-inference-cut/traces-golden/q-naf-inference", "red/trace-naf-proved/traces-golden/q-naf-proved", "red/trace-no-mirror/traces-golden/q-no-trace", "red/trace-row-failure-after-cut/traces-golden/q-row-cut", "red/trace-unproved-finite/traces-golden/q-unproved-finite", "red/trace-unproved-limit/traces-golden/q-unproved-limit", "red/yesno-limit-before-proof/answers-golden/q-yn-limit", "red/yesno-limit-before-proof/traces-golden/q-yn-limit"]
    if sorted(pin_inventory) != pin_expected:
        cleanup_violation(scratch_path, "queries-fixtures", "golden-pin inventory drifted from pinned 40-entry map: " + str(len(pin_inventory)) + " pins on disk")
    if red_count != 24:
        cleanup_violation(scratch_path, "queries-fixtures", "red case count drift: expected 24 got " + str(red_count))
    if green_count != 11:
        cleanup_violation(scratch_path, "queries-fixtures", "green case count drift: expected 11 got " + str(green_count))
    check_trace_nonfinite_probe(scratch_path, swipl_executable, stage_path)
    print("goal: queries fixtures ok " + str(red_count) + " red " + str(green_count) + " green")
def check_trace_nonfinite_probe(scratch_path, swipl_executable, stage_path):
    case_gid = pathlib.Path("tests/queries/green/trace-direct-rejects/tree/guidelines/fx")
    manifest_path = scratch_path.joinpath("nonfinite-manifest")
    build_query_manifest(manifest_path, case_gid.joinpath("pl"))
    template_path = pathlib.Path("tests/queries/green/trace-direct-rejects/trace-reject/result-shape.answers")
    template_text = template_path.read_text(encoding="utf-8")
    hostile_text = template_text.replace("result(bogus)", "result(solutions([sol([1.0Inf])]))")
    if hostile_text == template_text:
        cleanup_violation(scratch_path, "trace-nonfinite", "probe template lost its result(bogus) anchor")
    hostile_path = scratch_path.joinpath("nonfinite-answers.pl")
    hostile_path.write_text(hostile_text, encoding="utf-8")
    query_pl = case_gid.joinpath("queries", "pl", "result-shape.pl")
    probe_tail = ["trace", str(manifest_path), str(query_pl), str(hostile_path)]
    probe_command = compiler_command(swipl_executable, stage_path, probe_tail)
    try:
        probe_result = subprocess.run(probe_command, capture_output=True, timeout=30)
    except subprocess.TimeoutExpired:
        cleanup_violation(scratch_path, "trace-nonfinite", "wall_clock for non-finite float probe")
    if probe_result.returncode != 1:
        cleanup_violation(scratch_path, "trace-nonfinite", "status " + str(probe_result.returncode) + " for non-finite float probe")
    if probe_result.stdout:
        cleanup_violation(scratch_path, "trace-nonfinite", "non-empty stdout for non-finite float probe")
    expect_text = "ace_to_pl_error(proof,trace_unserializable).\n"
    expect_bytes = expect_text.encode("utf-8")
    if probe_result.stderr != expect_bytes:
        cleanup_violation(scratch_path, "trace-nonfinite", "stderr differs from pinned trace_unserializable line")
def ui_walk_files(base_path):
    found = []
    pending = [base_path]
    while pending:
        current = pending.pop(0)
        for entry in sorted(current.iterdir()):
            if entry.is_dir():
                pending.append(entry)
            else:
                rel_path = entry.relative_to(base_path)
                found.append(str(rel_path))
    return sorted(found)
def ui_fixture_stream(case_path, case_name, stream_name):
    if stream_name == "-":
        empty_text = ""
        return empty_text.encode("utf-8")
    stream_path = case_path.joinpath(stream_name)
    if not stream_path.is_file():
        violation("ui-fixtures", "missing stream fixture " + stream_name + " for case: " + case_name)
    return stream_path.read_bytes()
def ui_compare_golden(case_path, case_name, out_text):
    golden_path = case_path.joinpath("golden")
    if not golden_path.is_dir():
        violation("ui-fixtures", "missing golden directory for case: " + case_name)
    out_path = pathlib.Path(out_text)
    golden_rel = ui_walk_files(golden_path)
    out_rel = ui_walk_files(out_path)
    if golden_rel != out_rel:
        violation("ui-fixtures", "golden tree differs for case: " + case_name)
    for rel_text in golden_rel:
        golden_file = golden_path.joinpath(rel_text)
        out_file = out_path.joinpath(rel_text)
        golden_bytes = golden_file.read_bytes()
        out_bytes = out_file.read_bytes()
        if golden_bytes != out_bytes:
            violation("ui-fixtures", "golden bytes differ for case: " + case_name + " " + rel_text)
def ui_check_assertions(case_path, case_name):
    assertions_path = case_path.joinpath("assertions.tsv")
    if not assertions_path.is_file():
        return 0
    data = assertions_path.read_bytes()
    text = ""
    try:
        text = data.decode("utf-8")
    except UnicodeDecodeError:
        violation("ui-fixtures", "assertions not UTF-8 for case: " + case_name)
    applied = 0
    for line_text in text.split("\n"):
        is_comment = line_text.startswith("#")
        if not is_comment:
            if line_text:
                fields = line_text.split("\t")
                if len(fields) != 3:
                    violation("ui-fixtures", "malformed assertion row for case: " + case_name)
                page_rel = fields.pop(0)
                op_name = fields.pop(0)
                hex_text = fields.pop(0)
                needle = ""
                try:
                    needle = bytes.fromhex(hex_text)
                except ValueError:
                    violation("ui-fixtures", "invalid assertion hex for case: " + case_name)
                page_path = case_path.joinpath("golden", page_rel)
                if not page_path.is_file():
                    violation("ui-fixtures", "assertion page missing for case: " + case_name + " " + page_rel)
                haystack = page_path.read_bytes()
                found = needle in haystack
                if op_name == "contains":
                    if not found:
                        violation("ui-fixtures", "assertion contains failed for case: " + case_name + " " + page_rel)
                else:
                    if op_name == "absent":
                        if found:
                            violation("ui-fixtures", "assertion absent failed for case: " + case_name + " " + page_rel)
                    else:
                        violation("ui-fixtures", "unknown assertion op for case: " + case_name)
                applied = applied + 1
    return applied
def ui_safe_rel(rel_text, case_name):
    backslash = chr(92)
    bad = False
    if rel_text == "":
        bad = True
    if rel_text.startswith("/"):
        bad = True
    if backslash in rel_text:
        bad = True
    parts = rel_text.split("/")
    for part in parts:
        if part == "..":
            bad = True
        if part == "":
            bad = True
    if bad:
        violation("ui-fixtures", "unsafe sidecar path for case: " + case_name)
def ui_read_sidecar_lines(sidecar_path, case_name):
    data = sidecar_path.read_bytes()
    text = ""
    try:
        text = data.decode("utf-8")
    except UnicodeDecodeError:
        violation("ui-fixtures", "sidecar not UTF-8 for case: " + case_name)
    lines = []
    for line_text in text.split("\n"):
        is_comment = line_text.startswith("#")
        if not is_comment:
            if line_text:
                ui_safe_rel(line_text, case_name)
                lines.append(line_text)
    return lines
def ui_git_env():
    env = os.environ.copy()
    env.update({"GIT_DEFAULT_HASH": "sha1"})
    env.update({"GIT_CONFIG_GLOBAL": "/dev/null"})
    env.update({"GIT_CONFIG_SYSTEM": "/dev/null"})
    env.update({"GIT_AUTHOR_NAME": "fixture"})
    env.update({"GIT_AUTHOR_EMAIL": "fixture@localhost"})
    env.update({"GIT_AUTHOR_DATE": "2026-01-01T00:00:00+00:00"})
    env.update({"GIT_COMMITTER_NAME": "fixture"})
    env.update({"GIT_COMMITTER_EMAIL": "fixture@localhost"})
    env.update({"GIT_COMMITTER_DATE": "2026-01-01T00:00:00+00:00"})
    return env
def ui_git_run(work_dir, arg_list, label, case_name):
    command = ["git"]
    for arg_text in arg_list:
        command.append(arg_text)
    result = None
    try:
        result = subprocess.run(command, capture_output=True, cwd=work_dir, env=ui_git_env())
    except OSError:
        violation("ui-fixtures", "git " + label + " unavailable for case: " + case_name)
    if result.returncode != 0:
        violation("ui-fixtures", "git " + label + " failed for case: " + case_name)
def ui_commit_worktree(case_path, case_name, dest_root):
    worktree_path = case_path.joinpath("worktree")
    if not worktree_path.is_dir():
        return False
    work_dir = str(dest_root)
    ui_git_run(work_dir, ["init", "-q", "-b", "main"], "init", case_name)
    ui_git_run(work_dir, ["add", "-A"], "add", case_name)
    ui_git_run(work_dir, ["commit", "-q", "-m", "fixture corpus"], "commit", case_name)
    for rel_text in ui_walk_files(worktree_path):
        ui_safe_rel(rel_text, case_name)
        src_file = worktree_path.joinpath(rel_text)
        dest_file = dest_root.joinpath(rel_text)
        dest_parent = dest_file.parent
        dest_parent.mkdir(parents=True, exist_ok=True)
        payload = src_file.read_bytes()
        dest_file.write_bytes(payload)
    return True
def ui_materialize_tree(case_path, case_name, tree_rel):
    order_path = case_path.joinpath("tree-order.txt")
    empties_path = case_path.joinpath("empty-dirs.txt")
    worktree_path = case_path.joinpath("worktree")
    has_order = order_path.is_file()
    has_empties = empties_path.is_file()
    has_worktree = worktree_path.is_dir()
    if not has_order:
        if not has_empties:
            if not has_worktree:
                return [tree_rel, ""]
    tree_path = case_path.joinpath("tree")
    walk_list = ui_walk_files(tree_path)
    listed = walk_list
    if has_order:
        listed = ui_read_sidecar_lines(order_path, case_name)
        listed_sorted = sorted(listed)
        if listed_sorted != walk_list:
            violation("ui-fixtures", "tree-order mismatch: " + case_name)
    scratch_text = tempfile.mkdtemp()
    dest_root = pathlib.Path(scratch_text + "/tree")
    for rel_text in listed:
        src_file = tree_path.joinpath(rel_text)
        dest_file = dest_root.joinpath(rel_text)
        dest_parent = dest_file.parent
        dest_parent.mkdir(parents=True, exist_ok=True)
        payload = src_file.read_bytes()
        dest_file.write_bytes(payload)
    if has_empties:
        empty_lines = ui_read_sidecar_lines(empties_path, case_name)
        for rel_text in empty_lines:
            empty_dir = dest_root.joinpath(rel_text)
            empty_dir.mkdir(parents=True, exist_ok=True)
    ui_commit_worktree(case_path, case_name, dest_root)
    return [str(dest_root), scratch_text]
def run_ui_fixture_case(color, case_name, case_path):
    case_file = case_path.joinpath("case.tsv")
    if not case_file.is_file():
        violation("ui-fixtures", "missing case.tsv for case: " + case_name)
    data = case_file.read_bytes()
    text = ""
    try:
        text = data.decode("utf-8")
    except UnicodeDecodeError:
        violation("ui-fixtures", "case.tsv not UTF-8 for case: " + case_name)
    tree_rel = "tests/ui/" + color + "/" + case_name + "/tree"
    mat_pair = ui_materialize_tree(case_path, case_name, tree_rel)
    mat_copy = list(mat_pair)
    tree_text = mat_copy.pop(0)
    mat_scratch = mat_copy.pop(0)
    row_count = 0
    for line_text in text.split("\n"):
        is_comment = line_text.startswith("#")
        if not is_comment:
            if line_text:
                fields = line_text.split("\t")
                if len(fields) != 4:
                    violation("ui-fixtures", "malformed case row for case: " + case_name)
                argv_text = fields.pop(0)
                rc_text = fields.pop(0)
                stderr_name = fields.pop(0)
                stdout_name = fields.pop(0)
                expected_rc = 0
                try:
                    expected_rc = int(rc_text)
                except ValueError:
                    violation("ui-fixtures", "invalid rc for case: " + case_name)
                scratch_text = tempfile.mkdtemp()
                out_text = scratch_text + "/out"
                tokens = argv_text.split(" ")
                first_token = ""
                second_token = ""
                token_index = 0
                built_tokens = [sys.executable, "-P", "tools/ui.py"]
                for token in tokens:
                    built = token
                    if token == "@TREE@":
                        built = tree_text
                    if token == "@OUT@":
                        built = out_text
                    if token_index == 0:
                        first_token = token
                    if token_index == 1:
                        second_token = token
                    token_index = token_index + 1
                    built_tokens.append(built)
                if first_token == "serve":
                    violation("ui-fixtures", "serve row for case: " + case_name)
                if first_token == "request":
                    if second_token == "POST":
                        if mat_scratch == "":
                            violation("ui-fixtures", "post row without materialized tree for case: " + case_name)
                result = subprocess.run(built_tokens, capture_output=True)
                if result.returncode != expected_rc:
                    violation("ui-fixtures", "status " + str(result.returncode) + " for case: " + case_name)
                expected_stderr = ui_fixture_stream(case_path, case_name, stderr_name)
                if result.stderr != expected_stderr:
                    violation("ui-fixtures", "stderr differs for case: " + case_name)
                expected_stdout = ui_fixture_stream(case_path, case_name, stdout_name)
                if result.stdout != expected_stdout:
                    violation("ui-fixtures", "stdout differs for case: " + case_name)
                if first_token == "render":
                    if color == "green":
                        ui_compare_golden(case_path, case_name, out_text)
                shutil.rmtree(scratch_text, ignore_errors=True)
                row_count = row_count + 1
    if row_count == 0:
        violation("ui-fixtures", "no case rows for case: " + case_name)
    tree_root = pathlib.Path(tree_text)
    after_path = case_path.joinpath("after")
    if after_path.is_dir():
        if mat_scratch == "":
            violation("ui-fixtures", "after sidecar without materialized tree for case: " + case_name)
        for rel_text in ui_walk_files(after_path):
            expect_file = after_path.joinpath(rel_text)
            actual_file = tree_root.joinpath(rel_text)
            if not actual_file.is_file():
                violation("ui-fixtures", "after file missing for case: " + case_name + " " + rel_text)
            expect_bytes = expect_file.read_bytes()
            actual_bytes = actual_file.read_bytes()
            if actual_bytes != expect_bytes:
                violation("ui-fixtures", "after bytes differ for case: " + case_name + " " + rel_text)
    globs_path = case_path.joinpath("absent-globs.txt")
    if globs_path.is_file():
        if mat_scratch == "":
            violation("ui-fixtures", "absent-globs sidecar without materialized tree for case: " + case_name)
        for glob_line in ui_read_sidecar_lines(globs_path, case_name):
            matches = list(tree_root.glob(glob_line))
            if matches:
                violation("ui-fixtures", "absent glob matched for case: " + case_name + " " + glob_line)
    if color == "green":
        check_tokens = [sys.executable, "-P", "tools/ui.py", "check", tree_text]
        check_result = subprocess.run(check_tokens, capture_output=True)
        if check_result.returncode != 0:
            violation("ui-fixtures", "ui check failed for case: " + case_name)
        if check_result.stderr:
            violation("ui-fixtures", "ui check stderr for case: " + case_name)
        ui_check_assertions(case_path, case_name)
        committed_tree = case_path.joinpath("tree")
        fixture_guidelines = committed_tree.joinpath("guidelines")
        if fixture_guidelines.is_dir():
            for gid_entry in sorted(fixture_guidelines.iterdir()):
                manifest_file = gid_entry.joinpath("audit", "review-manifest.tsv")
                if manifest_file.is_file():
                    derive_tokens = [sys.executable, "-P", "tools/goal.py", "derive-review-manifest", str(gid_entry)]
                    derive_result = subprocess.run(derive_tokens, capture_output=True)
                    if derive_result.returncode == 0:
                        committed_manifest_bytes = manifest_file.read_bytes()
                        if derive_result.stdout != committed_manifest_bytes:
                            violation("ui-fixtures", "manifest stale for case: " + case_name + " " + gid_entry.name)
    if mat_scratch:
        shutil.rmtree(mat_scratch, ignore_errors=True)
copy_marketing_pattern = "(?i)\\b(simply|seamless|seamlessly|powerful|robust|robustly|leverage|leverages|leveraged|leveraging|effortless|effortlessly|intuitive|streamline|streamlined|unlock|empower|empowering|cutting-edge|state-of-the-art|world-class|blazing|stunning|delightful|revolutionize|game-changing|supercharge|best-in-class|next-generation)\\b"
copy_relative_time_pattern = "(?i)\\bago\\b|\\bjust now\\b|\\byesterday\\b|\\btomorrow\\b|\\brecently\\b|\\blast (week|month|year)\\b"
copy_css_tokens = ["linear-gradient", "radial-gradient", "conic-gradient", "@keyframes", "animation", "transition", "backdrop-filter", "box-shadow"]
def copy_emoji_hit(text):
    for ch in text:
        code_point = ord(ch)
        banned = False
        if code_point > 126975:
            if code_point < 129792:
                banned = True
        if code_point > 9727:
            if code_point < 10176:
                banned = True
        if code_point > 11007:
            if code_point < 11264:
                banned = True
        if code_point == 65039:
            banned = True
        if code_point == 8205:
            banned = True
        if banned:
            return "emoji: U+" + format(code_point, "04X")
    return ""
def copy_register_hit(text):
    emoji_hit = copy_emoji_hit(text)
    if emoji_hit:
        return emoji_hit
    lower_text = text.lower()
    for css_token in copy_css_tokens:
        found = css_token in lower_text
        if found:
            return "css: " + css_token
    marketing_match = re.search(copy_marketing_pattern, text)
    if marketing_match:
        return "marketing: " + marketing_match.group(0)
    relative_match = re.search(copy_relative_time_pattern, text)
    if relative_match:
        return "relative-time: " + relative_match.group(0)
    exclaim_match = re.search("[A-Za-z0-9]!", text)
    if exclaim_match:
        return "exclamatory: " + exclaim_match.group(0)
    return ""
def copy_scan_source(source_text):
    hits = []
    literal_count = 0
    tree = ast.parse(source_text)
    constant_type = getattr(ast, "Constant")
    for node in ast.walk(tree):
        is_const = isinstance(node, constant_type)
        if is_const:
            value_obj = getattr(node, "value", None)
            is_str = isinstance(value_obj, str)
            if is_str:
                literal_count = literal_count + 1
                hit_text = copy_register_hit(value_obj)
                if hit_text:
                    line_no = getattr(node, "lineno", 0)
                    hits.append(str(line_no) + " " + hit_text)
    return [hits, literal_count]
def copy_fixture_case(color, case_name, case_path):
    source_path = case_path.joinpath("source.txt")
    if source_path.is_symlink():
        violation("copy-fixtures", "source is a symlink: " + color + "/" + case_name)
    if not source_path.is_file():
        violation("copy-fixtures", "missing source.txt: " + color + "/" + case_name)
    read_ok = True
    source_text = ""
    try:
        source_text = source_path.read_text(encoding="utf-8")
    except OSError:
        read_ok = False
    except UnicodeDecodeError:
        read_ok = False
    if not read_ok:
        violation("copy-fixtures", "unreadable source.txt: " + color + "/" + case_name)
    scan_ok = True
    scan_pair = []
    try:
        scan_pair = copy_scan_source(source_text)
    except SyntaxError:
        scan_ok = False
    except ValueError:
        scan_ok = False
    if not scan_ok:
        violation("copy-fixtures", "unparseable source.txt: " + color + "/" + case_name)
    hits = scan_pair.pop(0)
    expect_path = case_path.joinpath("expect.txt")
    if color == "red":
        if not expect_path.is_file():
            violation("copy-fixtures", "missing expect.txt: red/" + case_name)
        expect_text = expect_path.read_text(encoding="utf-8")
        if not hits:
            violation("copy-fixtures", "red case produced no hits: " + case_name)
        joiner = "\n"
        actual_text = joiner.join(hits) + "\n"
        if actual_text != expect_text:
            violation("copy-fixtures", "hit mismatch: red/" + case_name)
    else:
        if expect_path.exists():
            violation("copy-fixtures", "unexpected expect.txt: green/" + case_name)
        if hits:
            first_hit = hits.pop(0)
            violation("copy-fixtures", "green case produced a hit: " + case_name + " " + first_hit)
def check_copy_register():
    ui_path = pathlib.Path("tools/ui.py")
    read_ok = True
    ui_source = ""
    try:
        ui_source = ui_path.read_text(encoding="utf-8")
    except OSError:
        read_ok = False
    except UnicodeDecodeError:
        read_ok = False
    if not read_ok:
        violation("copy", "cannot read tools/ui.py")
    ui_marker = "def page_invariant_name("
    marker_present = ui_marker in ui_source
    if not marker_present:
        violation("copy", "scan target is not the generated UI module")
    scan_ok = True
    scan_pair = []
    try:
        scan_pair = copy_scan_source(ui_source)
    except SyntaxError:
        scan_ok = False
    except ValueError:
        scan_ok = False
    if not scan_ok:
        violation("copy", "tools/ui.py does not parse")
    ui_hits = scan_pair.pop(0)
    literal_count = scan_pair.pop(0)
    for hit_line in ui_hits:
        violation("copy", "tools/ui.py:" + hit_line)
    print("goal: copy ok " + str(literal_count) + " literals")
    fixtures_root = pathlib.Path("tests/copy")
    if fixtures_root.is_symlink():
        violation("copy-fixtures", "is a symlink: " + str(fixtures_root))
    if not fixtures_root.is_dir():
        violation("copy-fixtures", "missing: " + str(fixtures_root))
    red_required = ["copy-css-animation", "copy-css-backdrop", "copy-css-boxshadow", "copy-css-gradient", "copy-css-keyframes", "copy-css-transition", "copy-emoji", "copy-exclamatory", "copy-marketing", "copy-relative-time"]
    green_required = ["copy-clean", "copy-lookalike"]
    red_count = 0
    green_count = 0
    red_names = []
    green_names = []
    colors = ["green", "red"]
    for color in colors:
        color_path = fixtures_root.joinpath(color)
        if color_path.is_symlink():
            violation("copy-fixtures", "is a symlink: " + color)
        if not color_path.is_dir():
            violation("copy-fixtures", "missing directory: " + color)
        for entry in sorted(color_path.iterdir()):
            entry_name = entry.name
            if entry.is_symlink():
                violation("copy-fixtures", "not a case directory: " + entry_name)
            if not entry.is_dir():
                violation("copy-fixtures", "not a case directory: " + entry_name)
            if not valid_docid(entry_name):
                violation("copy-fixtures", "invalid case name: " + entry_name)
            copy_fixture_case(color, entry_name, entry)
            if color == "red":
                red_count = red_count + 1
                red_names.append(entry_name)
            else:
                green_count = green_count + 1
                green_names.append(entry_name)
    for required_name in red_required:
        present = required_name in red_names
        if not present:
            violation("copy-fixtures", "missing required case: red/" + required_name)
    for required_name in green_required:
        present = required_name in green_names
        if not present:
            violation("copy-fixtures", "missing required case: green/" + required_name)
    if red_count != 10:
        violation("copy-fixtures", "red case count drift: expected 10 got " + str(red_count))
    if green_count != 2:
        violation("copy-fixtures", "green case count drift: expected 2 got " + str(green_count))
    print("goal: copy fixtures ok " + str(red_count) + " red " + str(green_count) + " green")
def chain_call_name(chain_stmt):
    is_expr = isinstance(chain_stmt, getattr(ast, "Expr"))
    if not is_expr:
        return ""
    value_obj = getattr(chain_stmt, "value", None)
    is_call = isinstance(value_obj, getattr(ast, "Call"))
    if not is_call:
        return ""
    func_obj = getattr(value_obj, "func", None)
    is_name = isinstance(func_obj, getattr(ast, "Name"))
    if not is_name:
        return ""
    return getattr(func_obj, "id", "")
def check_copy_chain_slot():
    goal_path = pathlib.Path("tools/goal.py")
    read_ok = True
    goal_source = ""
    try:
        goal_source = goal_path.read_text(encoding="utf-8")
    except OSError:
        read_ok = False
    except UnicodeDecodeError:
        read_ok = False
    if not read_ok:
        violation("copy-chain", "cannot read tools/goal.py")
    parse_ok = True
    tree = None
    try:
        tree = ast.parse(goal_source)
    except SyntaxError:
        parse_ok = False
    except ValueError:
        parse_ok = False
    if not parse_ok:
        violation("copy-chain", "tools/goal.py does not parse")
    def_type = getattr(ast, "FunctionDef")
    slot_ok = False
    expect_next = False
    for node in ast.walk(tree):
        is_def = isinstance(node, def_type)
        if is_def:
            if getattr(node, "name", "") == "check_command":
                for chain_stmt in getattr(node, "body", []):
                    call_name = chain_call_name(chain_stmt)
                    if expect_next:
                        if call_name == "check_copy_register":
                            slot_ok = True
                        expect_next = False
                    if call_name == "check_ui":
                        expect_next = True
    if not slot_ok:
        violation("copy-chain", "check_command must call check_copy_register directly after check_ui")
def check_ui():
    check_copy_chain_slot()
    command = [sys.executable, "-P", "tools/ui.py", "check"]
    result = subprocess.run(command, capture_output=True)
    if result.returncode != 0:
        stderr_text = result.stderr.decode("utf-8", errors="replace")
        relay = "ui check failed"
        found_relay = False
        for line_text in stderr_text.splitlines():
            stripped = line_text.strip()
            if stripped:
                if not found_relay:
                    relay = stripped
                    found_relay = True
        violation("ui", relay)
    if result.stderr:
        violation("ui", "non-empty stderr from ui check")
    stdout_text = result.stdout.decode("utf-8", errors="replace")
    ok_suffix = ""
    check_ok_seen = False
    for line_text in stdout_text.splitlines():
        if line_text.startswith("ui: ok "):
            ok_suffix = line_text.removeprefix("ui: ok ")
        if line_text == "ui: check ok":
            check_ok_seen = True
    if ok_suffix == "":
        violation("ui", "missing ui ok meter")
    if not check_ok_seen:
        violation("ui", "missing ui check ok line")
    print("goal: ui ok " + ok_suffix)
    fixtures_root = pathlib.Path("tests/ui")
    if fixtures_root.is_symlink():
        violation("ui-fixtures", "is a symlink: " + str(fixtures_root))
    if not fixtures_root.is_dir():
        violation("ui-fixtures", "missing: " + str(fixtures_root))
    red_required = ["copy-visible-hex", "digest-mismatch-ace", "digest-mismatch-payload", "doc-missing-manifest", "duplicate-docid", "http-404", "http-405", "ledger-invalid", "manifest-missing-doc", "missing-coverage", "orphan-pl", "region-resolve-failure", "unknown-ledger-docid", "verdict-405", "verdict-artifact-drift", "verdict-artifact-drift-payload", "verdict-cas-conflict", "verdict-crash", "verdict-csrf", "verdict-field-grammar", "verdict-get-form", "verdict-host", "verdict-ledger-invalid", "verdict-manifest-derivation", "verdict-ok-append", "verdict-ok-create", "verdict-origin", "verdict-path-decode", "verdict-request-cli", "verdict-subject-drift"]
    green_required = ["basic", "git-uncommitted-edit", "git-untracked-document", "highlight", "hostile", "multi-guideline-order", "payload-selection", "verdicts"]
    red_count = 0
    green_count = 0
    red_names = []
    green_names = []
    colors = ["green", "red"]
    for color in colors:
        color_path = fixtures_root.joinpath(color)
        if color_path.is_symlink():
            violation("ui-fixtures", "is a symlink: " + color)
        if not color_path.is_dir():
            violation("ui-fixtures", "missing directory: " + color)
        for entry in sorted(color_path.iterdir()):
            entry_name = entry.name
            if entry.is_symlink():
                violation("ui-fixtures", "not a case directory: " + entry_name)
            if not entry.is_dir():
                violation("ui-fixtures", "not a case directory: " + entry_name)
            if not valid_docid(entry_name):
                violation("ui-fixtures", "invalid case name: " + entry_name)
            run_ui_fixture_case(color, entry_name, entry)
            if color == "red":
                red_count = red_count + 1
                red_names.append(entry_name)
            else:
                green_count = green_count + 1
                green_names.append(entry_name)
    for required_name in red_required:
        present = required_name in red_names
        if not present:
            violation("ui-fixtures", "missing required case: red/" + required_name)
    for required_name in green_required:
        present = required_name in green_names
        if not present:
            violation("ui-fixtures", "missing required case: green/" + required_name)
    if red_count != 81:
        violation("ui-fixtures", "red case count drift: expected 81 got " + str(red_count))
    if green_count != 15:
        violation("ui-fixtures", "green case count drift: expected 15 got " + str(green_count))
    print("goal: ui fixtures ok " + str(red_count) + " red " + str(green_count) + " green")
def check_corpus(guideline_path, ace_paths, docids, lexicon_path):
    ledger_pairs = check_projection_ledger(guideline_path)
    ledger_docids = []
    for ledger_pair in ledger_pairs:
        pair_copy = list(ledger_pair)
        ledger_docid = pair_copy.pop(0)
        ledger_docids.append(ledger_docid)
    for ledger_docid in ledger_docids:
        known = ledger_docid in docids
        if not known:
            violation("projection-ledger", "row for unknown docid: " + ledger_docid)
    for docid in docids:
        covered = docid in ledger_docids
        if not covered:
            violation("projection-ledger", "docid missing projection row: " + docid)
        check_product_vocabulary(guideline_path, docid)
    coverage_result = check_coverage(guideline_path, docids, True)
    status_by_id = coverage_result.pop(0)
    ace_row_line_by_docid = coverage_result.pop(0)
    payload_text_by_docid = coverage_result.pop(0)
    for ledger_pair in ledger_pairs:
        pair_copy = list(ledger_pair)
        ledger_docid = pair_copy.pop(0)
        ledger_region = pair_copy.pop(0)
        expected_status = "ace(" + ledger_docid + ")"
        actual_status = status_by_id.get(ledger_region, "")
        if actual_status == "":
            violation("projection-coverage", "projection row names no coverage region: " + ledger_docid + " " + ledger_region)
        if actual_status != expected_status:
            violation("projection-coverage", "coverage region " + ledger_region + " does not carry ace(" + ledger_docid + "): " + actual_status)
    check_census_map(guideline_path, status_by_id)
    check_adjudication(guideline_path, docids, ace_row_line_by_docid, payload_text_by_docid)
    if lexicon_path != None:
        check_lexicon(guideline_path, ace_paths, docids)
def check_docid_grammar_probe():
    long_ok = ""
    pad_done = False
    while not pad_done:
        long_ok = long_ok + "aaaaaaaaaa"
        if len(long_ok) == 250:
            pad_done = True
    if not valid_docid(long_ok):
        violation("docid-grammar", "250-byte docid rejected")
    if valid_docid(long_ok + "a"):
        violation("docid-grammar", "251-byte docid accepted")
def check_trace_numeric_probe():
    big_text = ""
    pad_done = False
    while not pad_done:
        big_text = big_text + "9999999999"
        if len(big_text) == 5000:
            pad_done = True
    hex_pad = ""
    hex_done = False
    while not hex_done:
        hex_pad = hex_pad + "aaaaaaaa"
        if len(hex_pad) == 64:
            hex_done = True
    comment_line = "% probe traced against the loaded composition by ace_to_pl trace mode; do not edit."
    term_head = "'$guideline_traces'(v1,probe,query_sha256('" + hex_pad + "'),answers_sha256('" + hex_pad + "'),result(solutions([sol(["
    term_tail = "],proved([clause(sentence(doc,1),clause_sha256('" + hex_pad + "'),[])]))])))."
    artifact = comment_line + "\n" + term_head + big_text + term_tail + "\n"
    parse_result = parse_trace_artifact(artifact.encode("utf-8"), "probe")
    parse_copy = list(parse_result)
    parse_err = parse_copy.pop(0)
    if parse_err != "":
        violation("trace-numeric", "5000-digit integer payload rejected by trace parser")
    wide_artifact = comment_line + "\n" + term_head + "1],proved([clause(sentence(doc,9999999999),clause_sha256('" + hex_pad + "'),[])]))]))." + "\n"
    wide_result = parse_trace_artifact(wide_artifact.encode("utf-8"), "probe")
    wide_copy = list(wide_result)
    wide_err = wide_copy.pop(0)
    if wide_err != "malformed":
        violation("trace-numeric", "10-digit sentence ordinal accepted by trace parser")
    zero_artifact = comment_line + "\n" + term_head + "1],proved([clause(sentence(doc,01),clause_sha256('" + hex_pad + "'),[])]))]))." + "\n"
    zero_result = parse_trace_artifact(zero_artifact.encode("utf-8"), "probe")
    zero_copy = list(zero_result)
    zero_err = zero_copy.pop(0)
    if zero_err != "malformed":
        violation("trace-numeric", "zero-padded sentence ordinal accepted by trace parser")
def check_swipl_wall_probe():
    start_stamp = time.monotonic()
    sleeper_pair = swipl_run_walled(["/bin/sh", "-c", "sleep 600"], None, 1)
    sleeper_copy = list(sleeper_pair)
    sleeper_timed_out = sleeper_copy.pop(0)
    if not sleeper_timed_out:
        violation("swipl-timeout-probe", "sleeper exited under the wall clock")
    pipe_pair = swipl_run_walled(["/bin/sh", "-c", "sleep 600 & exec sleep 600"], None, 1)
    pipe_copy = list(pipe_pair)
    pipe_timed_out = pipe_copy.pop(0)
    if not pipe_timed_out:
        violation("swipl-timeout-probe", "descendant-held-pipe sleeper exited under the wall clock")
    elapsed = time.monotonic() - start_stamp
    if elapsed > 30:
        violation("swipl-timeout-probe", "wall-clock probes overran 30s: descendant pipes survive the kill")
def load_dist_module():
    argv_copy = list(sys.argv)
    script_arg = argv_copy.pop(0)
    script_path = pathlib.Path(script_arg)
    resolved_path = script_path.resolve()
    dist_path = resolved_path.with_name("dist.py")
    if not dist_path.is_file():
        fail("dist", "missing dist runner beside goal.py: " + str(dist_path))
    source_text = dist_path.read_text(encoding="utf-8")
    marker = "\nargv = list(sys.argv)"
    parts = list(source_text.partition(marker))
    prefix_text = parts.pop(0)
    marker_sep = parts.pop(0)
    if marker_sep == "":
        fail("dist", "dist runner lacks argv marker: " + str(dist_path))
    namespace = {}
    exec(prefix_text, namespace)
    return [namespace, dist_path]
def dist_git_run(work_dir, arg_list):
    command_list = ["git"]
    for arg_text in arg_list:
        command_list.append(arg_text)
    env_map = ui_git_env()
    return subprocess.run(command_list, capture_output=True, cwd=str(work_dir), env=env_map)
def dist_probe_write(repo_path, rel_text, text):
    target_path = repo_path.joinpath(rel_text)
    parent_path = target_path.parent
    parent_path.mkdir(parents=True, exist_ok=True)
    target_path.write_text(text, encoding="utf-8")
def dist_probe_commit(scratch_path, repo_path, message_text):
    add_result = dist_git_run(repo_path, ["add", "-A"])
    if add_result.returncode != 0:
        cleanup_violation(scratch_path, "dist", "probe git add failed")
    commit_result = dist_git_run(repo_path, ["commit", "-q", "-m", message_text])
    if commit_result.returncode != 0:
        cleanup_violation(scratch_path, "dist", "probe git commit failed")
def dist_probe_hex(text):
    text_bytes = text.encode("utf-8")
    digest_object = hashlib.sha256(text_bytes)
    return digest_object.hexdigest()
def dist_probe_review_text(docid_list, digest_map_out):
    text = "# probe review manifest\n"
    for docid in docid_list:
        review_digest = dist_probe_hex("review:" + docid)
        filler_digest = dist_probe_hex("component:" + docid)
        row_text = docid
        column = 0
        while column < 4:
            row_text = row_text + "\t" + filler_digest
            column = column + 1
        text = text + row_text + "\t" + review_digest + "\n"
        digest_map_out.update({docid: review_digest})
    return text
def dist_probe_ledger_row(docid, digest_value, verdict_text):
    return docid + "\t" + digest_value + "\t\t" + verdict_text + "\tprobe\t2026-01-01T00:00:00Z\tprobe decision\n"
def dist_probe_repo(scratch_path, name_text):
    repo_path = scratch_path.joinpath(name_text)
    repo_path.mkdir()
    init_result = dist_git_run(repo_path, ["init", "-q", "-b", "main"])
    if init_result.returncode != 0:
        cleanup_violation(scratch_path, "dist", "probe git init failed")
    dist_probe_write(repo_path, "docs/REFERENCE.md", "# Probe KB\n\n## Compiled Prolog schema (v1)\n\nProbe schema bytes.\n\n## Operating\n\nRun the checks.\n")
    dist_probe_write(repo_path, "NOTICE", "Probe notice.\n")
    dist_probe_write(repo_path, "vendor/ape/prolog/ace_to_pl.pl", "% probe compiler\n")
    dist_probe_write(repo_path, "vendor/clex/clex_lexicon.pl", "% probe lexicon\n")
    dist_probe_write(repo_path, "guidelines/g-probe/rights.tsv", "profile\tstatement\turl\tretrieved\tnote\nredistributable\tProbe rights statement.\thttps://example.invalid/g-probe\t2026-01-01\tProbe note.\n")
    dist_probe_write(repo_path, "guidelines/g-probe/source/original.txt", "probe source bytes\n")
    dist_probe_write(repo_path, "guidelines/g-probe/ace/doc-a.ace", "Every probe is a record.\n")
    dist_probe_write(repo_path, "guidelines/g-probe/pl/doc-a.pl", "guideline_document('g-probe','doc-a',[],x).\n")
    return repo_path
def dist_probe_manifest(scratch_path, namespace, repo_path):
    derive_function = namespace.get("derive_release")
    pair = derive_function(str(repo_path))
    detail = pair.pop(0)
    plan = pair.pop(0)
    if detail != "":
        cleanup_violation(scratch_path, "dist", "probe manifest derive failed: " + detail)
    manifest_text = plan.get("manifest")
    manifest_path = repo_path.joinpath("release-manifest.tsv")
    manifest_path.write_text(manifest_text, encoding="utf-8")
    dist_probe_commit(scratch_path, repo_path, "probe release manifest")
def dist_probe_build(dist_path, repo_path):
    command_list = [sys.executable, "-P", str(dist_path), "build", "out"]
    return subprocess.run(command_list, capture_output=True, cwd=str(repo_path))
def dist_probe_expect_refusal(scratch_path, result, probe_name, expected_line):
    stderr_text = result.stderr.decode("utf-8", errors="replace")
    stdout_text = result.stdout.decode("utf-8", errors="replace")
    ok_flag = True
    if result.returncode != 1:
        ok_flag = False
    if stdout_text != "":
        ok_flag = False
    if stderr_text != expected_line:
        ok_flag = False
    if not ok_flag:
        cleanup_violation(scratch_path, "dist", "probe " + probe_name + " expected " + expected_line.strip() + "; got rc " + str(result.returncode) + " stderr " + stderr_text.strip())
def check_dist_probes(scratch_path, namespace, dist_path):
    repo_path = dist_probe_repo(scratch_path, "probe-tamper")
    digest_map = {}
    review_text = dist_probe_review_text(["doc-a"], digest_map)
    dist_probe_write(repo_path, "guidelines/g-probe/audit/review-manifest.tsv", review_text)
    dist_probe_commit(scratch_path, repo_path, "probe corpus")
    dist_probe_manifest(scratch_path, namespace, repo_path)
    dist_probe_write(repo_path, "guidelines/g-probe/source/original.txt", "tampered probe source\n")
    dist_probe_commit(scratch_path, repo_path, "probe tamper")
    result = dist_probe_build(dist_path, repo_path)
    dist_probe_expect_refusal(scratch_path, result, "tamper", "dist: manifest-drift data/guidelines/g-probe/source/original.txt\n")
    repo_path = dist_probe_repo(scratch_path, "probe-rejected")
    digest_map = {}
    review_text = dist_probe_review_text(["doc-a"], digest_map)
    dist_probe_write(repo_path, "guidelines/g-probe/audit/review-manifest.tsv", review_text)
    ledger_text = "# probe ledger\n" + dist_probe_ledger_row("doc-a", digest_map.get("doc-a"), "rejected")
    dist_probe_write(repo_path, "guidelines/g-probe/audit/adjudication.tsv", ledger_text)
    dist_probe_commit(scratch_path, repo_path, "probe corpus")
    dist_probe_manifest(scratch_path, namespace, repo_path)
    result = dist_probe_build(dist_path, repo_path)
    dist_probe_expect_refusal(scratch_path, result, "rejected", "dist: rejected-verdict doc-a\n")
    repo_path = dist_probe_repo(scratch_path, "probe-rights-missing")
    dist_probe_commit(scratch_path, repo_path, "probe corpus")
    dist_probe_manifest(scratch_path, namespace, repo_path)
    rights_path = repo_path.joinpath("guidelines/g-probe/rights.tsv")
    rights_path.unlink()
    dist_probe_commit(scratch_path, repo_path, "probe rights removed")
    result = dist_probe_build(dist_path, repo_path)
    dist_probe_expect_refusal(scratch_path, result, "rights-missing", "dist: rights g-probe missing\n")
    repo_path = dist_probe_repo(scratch_path, "probe-rights-profile")
    dist_probe_commit(scratch_path, repo_path, "probe corpus")
    dist_probe_manifest(scratch_path, namespace, repo_path)
    dist_probe_write(repo_path, "guidelines/g-probe/rights.tsv", "profile\tstatement\turl\tretrieved\tnote\nother\tProbe rights statement.\thttps://example.invalid/g-probe\t2026-01-01\tProbe note.\n")
    dist_probe_commit(scratch_path, repo_path, "probe rights profile")
    result = dist_probe_build(dist_path, repo_path)
    dist_probe_expect_refusal(scratch_path, result, "rights-profile", "dist: rights g-probe profile:1\n")
    repo_path = dist_probe_repo(scratch_path, "probe-rights-statement")
    dist_probe_commit(scratch_path, repo_path, "probe corpus")
    dist_probe_manifest(scratch_path, namespace, repo_path)
    dist_probe_write(repo_path, "guidelines/g-probe/rights.tsv", "profile\tstatement\turl\tretrieved\tnote\nredistributable\t\thttps://example.invalid/g-probe\t2026-01-01\tProbe note.\n")
    dist_probe_commit(scratch_path, repo_path, "probe rights statement")
    result = dist_probe_build(dist_path, repo_path)
    dist_probe_expect_refusal(scratch_path, result, "rights-statement", "dist: rights g-probe statement:1\n")
    repo_path = dist_probe_repo(scratch_path, "probe-label")
    digest_map = {}
    review_text = dist_probe_review_text(["doc-a", "doc-b"], digest_map)
    dist_probe_write(repo_path, "guidelines/g-probe/audit/review-manifest.tsv", review_text)
    dist_probe_write(repo_path, "guidelines/g-probe/pl/doc-b.pl", "guideline_document('g-probe','doc-b',[],x).\n")
    stale_digest = dist_probe_hex("stale:doc-b")
    ledger_text = "# probe ledger\n" + dist_probe_ledger_row("doc-b", stale_digest, "approved")
    dist_probe_write(repo_path, "guidelines/g-probe/audit/adjudication.tsv", ledger_text)
    dist_probe_commit(scratch_path, repo_path, "probe corpus")
    dist_probe_manifest(scratch_path, namespace, repo_path)
    manifest_path = repo_path.joinpath("release-manifest.tsv")
    manifest_text = manifest_path.read_text(encoding="utf-8")
    if "label\tdoc-a\tunreviewed\n" not in manifest_text:
        cleanup_violation(scratch_path, "dist", "probe label missing unreviewed row for doc-a")
    if "label\tdoc-b\tstale\n" not in manifest_text:
        cleanup_violation(scratch_path, "dist", "probe label missing stale row for doc-b")
    result = dist_probe_build(dist_path, repo_path)
    stdout_text = result.stdout.decode("utf-8", errors="replace")
    label_ok = True
    if result.returncode != 0:
        label_ok = False
    if not stdout_text.startswith("dist: ok "):
        label_ok = False
    if not label_ok:
        cleanup_violation(scratch_path, "dist", "probe label expected green build; got rc " + str(result.returncode))
def check_dist():
    load_pair = load_dist_module()
    namespace = load_pair.pop(0)
    dist_path = load_pair.pop(0)
    derive_function = namespace.get("derive_release")
    pair = derive_function(".")
    detail = pair.pop(0)
    plan = pair.pop(0)
    if detail != "":
        violation("dist", detail)
    regen_hint = "; regenerate: python3 -P tools/goal.py release-manifest"
    manifest_path = pathlib.Path("release-manifest.tsv")
    if manifest_path.is_symlink():
        violation("dist", "release manifest is a symlink: release-manifest.tsv")
    if not manifest_path.exists():
        violation("dist", "release manifest missing: release-manifest.tsv" + regen_hint)
    if not manifest_path.is_file():
        violation("dist", "release manifest is not a regular file: release-manifest.tsv")
    committed_bytes = manifest_path.read_bytes()
    derived_manifest = plan.get("manifest")
    derived_bytes = derived_manifest.encode("utf-8")
    if committed_bytes != derived_bytes:
        violation("dist", "release manifest stale: release-manifest.tsv" + regen_hint)
    scratch_path = pathlib.Path(tempfile.mkdtemp(prefix="goal-dist-"))
    check_dist_probes(scratch_path, namespace, dist_path)
    rejected_list = plan.get("rejected")
    contested_list = plan.get("contested")
    rejected_count = len(rejected_list)
    contested_count = len(contested_list)
    if (rejected_count > 0) or (contested_count > 0):
        shutil.rmtree(scratch_path)
        print("goal: dist blocked rejected=" + str(rejected_count) + " contested=" + str(contested_count))
        return None
    dest_one = scratch_path.joinpath("live-one")
    dest_two = scratch_path.joinpath("live-two")
    command_one = [sys.executable, "-P", str(dist_path), "build", str(dest_one)]
    result_one = subprocess.run(command_one, capture_output=True)
    if result_one.returncode != 0:
        stderr_one = result_one.stderr.decode("utf-8", errors="replace")
        cleanup_violation(scratch_path, "dist", "live build failed: " + stderr_one.strip())
    stdout_one = result_one.stdout.decode("utf-8", errors="replace")
    if not stdout_one.startswith("dist: ok "):
        cleanup_violation(scratch_path, "dist", "live build meter grammar: " + stdout_one.strip())
    command_two = [sys.executable, "-P", str(dist_path), "build", str(dest_two)]
    result_two = subprocess.run(command_two, capture_output=True)
    if result_two.returncode != 0:
        cleanup_violation(scratch_path, "dist", "second live build failed")
    archives_one = sorted(dest_one.glob("*.tar.gz"))
    archives_two = sorted(dest_two.glob("*.tar.gz"))
    if len(archives_one) != 1:
        cleanup_violation(scratch_path, "dist", "live build archive count " + str(len(archives_one)))
    if len(archives_two) != 1:
        cleanup_violation(scratch_path, "dist", "second live build archive count " + str(len(archives_two)))
    archive_one = archives_one.pop(0)
    archive_two = archives_two.pop(0)
    raw_one = archive_one.read_bytes()
    raw_two = archive_two.read_bytes()
    if raw_one != raw_two:
        cleanup_violation(scratch_path, "dist", "live builds are not byte-identical")
    prefix_function = namespace.get("prefix_chars")
    head_text = plan.get("head")
    bag_root = "cnl-ckc-kb-g" + prefix_function(head_text, 12)
    tag_map = {"bagit.txt": True, "manifest-sha256.txt": True, "tagmanifest-sha256.txt": True, "README-dist.md": True, "NOTICE": True, "release-manifest.tsv": True}
    raw_tar = gzip.decompress(raw_one)
    tar_buffer = io.BytesIO(raw_tar)
    opened = tarfile.open(fileobj=tar_buffer, mode="r:")
    extract_root = scratch_path.joinpath("extract")
    member_count = 0
    root_prefix = bag_root + "/"
    for member in opened.getmembers():
        if not member.isfile():
            cleanup_violation(scratch_path, "dist", "archive member is not a regular file: " + member.name)
        member_name = member.name
        if not member_name.startswith(root_prefix):
            cleanup_violation(scratch_path, "dist", "archive member outside bag root: " + member_name)
        rel_name = member_name.removeprefix(root_prefix)
        rel_ok = False
        if rel_name in tag_map:
            rel_ok = True
        if rel_name.startswith("data/guidelines/"):
            rel_ok = True
        if not rel_ok:
            cleanup_violation(scratch_path, "dist", "archive member outside layout: " + rel_name)
        stream = opened.extractfile(member)
        member_data = stream.read()
        target_path = extract_root.joinpath(member_name)
        parent_path = target_path.parent
        parent_path.mkdir(parents=True, exist_ok=True)
        target_path.write_bytes(member_data)
        member_count = member_count + 1
    opened.close()
    bag_root_path = extract_root.joinpath(bag_root)
    verify_command = ["sha256sum", "-c", "manifest-sha256.txt", "tagmanifest-sha256.txt"]
    verify_result = subprocess.run(verify_command, capture_output=True, cwd=str(bag_root_path))
    if verify_result.returncode != 0:
        cleanup_violation(scratch_path, "dist", "sha256sum verification failed rc " + str(verify_result.returncode))
    if len(verify_result.stderr) != 0:
        cleanup_violation(scratch_path, "dist", "sha256sum verification stderr not empty")
    shipped_text = str(plan.get("shipped"))
    byte_count = len(raw_one)
    shutil.rmtree(scratch_path)
    print("goal: dist ok " + shipped_text + " guidelines " + str(member_count) + " members " + str(byte_count) + " bytes")
def release_manifest_command():
    load_pair = load_dist_module()
    namespace = load_pair.pop(0)
    load_pair.pop(0)
    derive_function = namespace.get("derive_release")
    pair = derive_function(".")
    detail = pair.pop(0)
    plan = pair.pop(0)
    if detail != "":
        violation("dist", detail)
    manifest_path = pathlib.Path("release-manifest.tsv")
    if manifest_path.is_symlink():
        violation("dist", "release manifest is a symlink: release-manifest.tsv")
    manifest_text = plan.get("manifest")
    manifest_bytes = manifest_text.encode("utf-8")
    manifest_path.write_bytes(manifest_bytes)
    payload_map = plan.get("payload")
    tags_map = plan.get("tags")
    member_count = len(payload_map) + len(tags_map)
    shipped_text = str(plan.get("shipped"))
    print("goal: release-manifest " + shipped_text + " guidelines " + str(member_count) + " members")
def check_command():
    check_fork_notices()
    check_docid_grammar_probe()
    check_trace_numeric_probe()
    check_swipl_wall_probe()
    check_strict_fixtures()
    check_adjudication_fixtures()
    check_compendium()
    guidelines_root = pathlib.Path("guidelines")
    root_symlink = guidelines_root.is_symlink()
    if root_symlink:
        fail("guidelines", "guidelines directory is a symlink")
    if not guidelines_root.is_dir():
        fail("guidelines", "missing guidelines directory")
    plans = []
    for entry in sorted(guidelines_root.iterdir()):
        entry_name = entry.name
        if entry.is_symlink():
            violation("guideline-entry", "symlink: " + entry_name)
        if not entry.is_dir():
            violation("guideline-entry", "not a directory: " + entry_name)
        if not valid_docid(entry_name):
            violation("guideline-entry", "invalid guideline id: " + entry_name)
        check_source_record(entry)
        collected = collect_guideline(entry)
        ace_paths = collected.pop(0)
        docids = collected.pop(0)
        lexicon_path = collected.pop(0)
        check_pl_inventory(entry, docids)
        plan_record = [entry, ace_paths, docids, lexicon_path]
        plans.append(plan_record)
    if not plans:
        violation("guidelines", "no guideline directories")
    red_plan = collect_red_probes()
    check_prolog_inventory()
    for plan_record in plans:
        record_copy = list(plan_record)
        corpus_path = record_copy.pop(0)
        corpus_ace_paths = record_copy.pop(0)
        corpus_docids = record_copy.pop(0)
        corpus_lexicon_path = record_copy.pop(0)
        check_corpus(corpus_path, corpus_ace_paths, corpus_docids, corpus_lexicon_path)
    check_ui()
    check_copy_register()
    check_dist()
    swipl_executable = resolve_swipl()
    scratch_path = make_scratch()
    stage_path = stage_ape(scratch_path, swipl_executable)
    guideline_count = 0
    document_count = 0
    for plan_record in plans:
        guideline_path = plan_record.pop(0)
        ace_paths = plan_record.pop(0)
        docids = plan_record.pop(0)
        lexicon_path = plan_record.pop(0)
        check_documents(scratch_path, swipl_executable, stage_path, guideline_path, ace_paths, docids, lexicon_path)
        guideline_count = guideline_count + 1
        document_count = document_count + len(docids)
    check_queries_fixtures(scratch_path, swipl_executable, stage_path)
    probe_count = 0
    for probe_record in red_plan:
        probe_path = probe_record.pop(0)
        class_name = probe_record.pop(0)
        expected_exit = probe_record.pop(0)
        run_red_probe(scratch_path, swipl_executable, stage_path, probe_path, class_name, expected_exit)
        probe_count = probe_count + 1
    shutil.rmtree(scratch_path)
    print("goal: check ok " + str(guideline_count) + " guidelines " + str(document_count) + " documents " + str(probe_count) + " red probes")
argv = list(sys.argv)
argv.pop(0)
if len(argv) == 0:
    fail("usage", "expected: goal compile <guideline-id> | goal check | goal queries <guideline-id> | goal queries-check <guideline-dir> [<stage-dir>] | goal review-manifest <guideline-id> | goal derive-review-manifest <guideline-dir> | goal ledger-validate <ledger-path> <manifest-path> <label> | goal release-manifest")
subcommand = argv.pop(0)
if subcommand == "compile":
    if len(argv) != 1:
        fail("usage", "expected: goal compile <guideline-id>")
    guideline_id = argv.pop(0)
    compile_command(guideline_id)
else:
    if subcommand == "check":
        if len(argv) != 0:
            fail("usage", "expected: goal check")
        check_command()
    else:
        if subcommand == "queries":
            if len(argv) != 1:
                fail("usage", "expected: goal queries <guideline-id>")
            queries_guideline_id = argv.pop(0)
            queries_command(queries_guideline_id)
        elif subcommand == "queries-check":
            queries_check_argc = len(argv)
            queries_check_ok = False
            if queries_check_argc == 1:
                queries_check_ok = True
            if queries_check_argc == 2:
                queries_check_ok = True
            if not queries_check_ok:
                fail("usage", "expected: goal queries-check <guideline-dir> [<stage-dir>]")
            queries_check_dir = argv.pop(0)
            queries_check_stage = None
            if queries_check_argc == 2:
                queries_check_stage = argv.pop(0)
            queries_check_command(queries_check_dir, queries_check_stage)
        elif subcommand == "review-manifest":
            if len(argv) != 1:
                fail("usage", "expected: goal review-manifest <guideline-id>")
            review_guideline_id = argv.pop(0)
            review_manifest_command(review_guideline_id)
        elif subcommand == "release-manifest":
            if len(argv) != 0:
                fail("usage", "expected: goal release-manifest")
            release_manifest_command()
        else:
            if subcommand == "derive-review-manifest":
                if len(argv) != 1:
                    fail("usage", "expected: goal derive-review-manifest <guideline-dir>")
                derive_guideline_dir = argv.pop(0)
                derive_review_manifest_command(derive_guideline_dir)
            elif subcommand == "ledger-validate":
                if len(argv) != 3:
                    fail("usage", "expected: goal ledger-validate <ledger-path> <manifest-path> <label>")
                ledger_arg = argv.pop(0)
                manifest_arg = argv.pop(0)
                label_arg = argv.pop(0)
                ledger_validate_command(ledger_arg, manifest_arg, label_arg)
            else:
                fail("usage", "unknown subcommand: " + subcommand)
