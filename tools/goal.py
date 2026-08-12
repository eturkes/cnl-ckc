import hashlib
import os
import pathlib
import re
import shutil
import subprocess
import sys
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
    return subset
def resolve_swipl():
    swipl = os.environ.get("SWIPL", "swipl")
    swipl_executable = shutil.which(swipl)
    if swipl_executable == None:
        fail("swipl-exec", "not executable: " + swipl)
    return swipl_executable
def make_scratch():
    pid_text = str(os.getpid())
    scratch_path = pathlib.Path(".goal.tmp." + pid_text)
    if scratch_path.exists():
        fail("scratch", "already exists: " + str(scratch_path))
    scratch_path.mkdir()
    return scratch_path
def stage_ape(scratch_path, swipl_executable):
    stage_path = scratch_path.joinpath("ape-stage")
    shutil.copytree("vendor/ape", stage_path)
    stage_clex = stage_path.joinpath("prolog", "lexicon", "clex_lexicon.pl")
    shutil.copy(clex_path_text, stage_clex)
    parser_dir = stage_path.joinpath("prolog", "parser")
    build_goal = "working_directory(_, '" + parser_dir.as_posix() + "'), [fit_to_plp], halt."
    command = [swipl_executable, "-O", "-f", "none", "-F", "none", "-g", build_goal, "-t", "halt"]
    result = subprocess.run(command, capture_output=True)
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
    result = subprocess.run(command, input=ace_bytes, capture_output=True)
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
    result = subprocess.run(command, capture_output=True)
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
def check_prolog_inventory():
    git_res = subprocess.run(["git", "ls-files", "--", "*.pl"], capture_output=True, check=True)
    tracked_text = git_res.stdout.decode("utf-8")
    for tracked in sorted(tracked_text.splitlines()):
        vendored = tracked.startswith("vendor/ape/")
        clex_base = tracked == "vendor/clex/clex_lexicon.pl"
        compiled = compiled_pl_path(tracked)
        if not vendored:
            if not clex_base:
                if not compiled:
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
    result = subprocess.run(command, capture_output=True)
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
    result = subprocess.run(command, capture_output=True)
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
    result = subprocess.run(command, input=probe_bytes, capture_output=True)
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
coverage_header_text = "# format: id<TAB>file<TAB>page<TAB>section<TAB>status\n# status: ace(<docid>) | restates(<id>) | uncovered(<class>: <one-clause reason>) | pending\n# uncovered classes: heading | process | external | aim | descriptive | notice"
uncovered_class_names = ["heading", "process", "external", "aim", "descriptive", "notice"]
census_rx = re.compile("identify the ([0-9]+) payloads below")
v1_functors = ["guideline_schema_version", "guideline_document", "guideline_entity", "guideline_cardinality", "guideline_event", "guideline_arg", "guideline_pp", "guideline_property", "guideline_operator"]
v1_directives = ["guideline_schema_version/1", "guideline_document/3", "guideline_entity/4", "guideline_cardinality/5", "guideline_event/3", "guideline_arg/4", "guideline_pp/4", "guideline_property/4", "guideline_operator/3"]
v1_decl_kinds = ["multifile", "discontiguous"]
token_strip_chars = ".,;:?!\"()"
clex_path_text = "vendor/clex/clex_lexicon.pl"
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
    row_docids = []
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
        row_docids.append(docid)
    return row_docids
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
def evidence_regions(evidence_path):
    data = read_corpus_file(evidence_path, "coverage")
    text = data.decode("utf-8")
    census_hit = census_rx.search(text)
    if census_hit == None:
        violation("coverage", "evidence lacks region-authority census: " + str(evidence_path))
    census_text = census_hit.group(1)
    census_count = int(census_text)
    locator_ids = []
    for raw_line in text.split("\n"):
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
                            locator_ids.append(region_id)
    return [census_count, locator_ids]
def check_coverage(guideline_path, docids):
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
    meter = "goal: coverage ok " + guideline_path.name + " " + str(row_count) + " regions; ace=" + str(ace_count) + " restates=" + str(restates_count) + " uncovered=" + str(uncovered_count) + " pending=" + str(pending_count)
    print(meter)
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
def check_corpus(guideline_path, ace_paths, docids, lexicon_path):
    ledger_docids = check_projection_ledger(guideline_path)
    for ledger_docid in ledger_docids:
        known = ledger_docid in docids
        if not known:
            violation("projection-ledger", "row for unknown docid: " + ledger_docid)
    for docid in docids:
        covered = docid in ledger_docids
        if not covered:
            violation("projection-ledger", "docid missing projection row: " + docid)
        check_product_vocabulary(guideline_path, docid)
    check_coverage(guideline_path, docids)
    if lexicon_path != None:
        check_lexicon(guideline_path, ace_paths, docids)
def check_command():
    check_fork_notices()
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
    probe_count = 0
    for probe_record in red_plan:
        probe_path = probe_record.pop(0)
        class_name = probe_record.pop(0)
        expected_exit = probe_record.pop(0)
        run_red_probe(scratch_path, swipl_executable, stage_path, probe_path, class_name, expected_exit)
        probe_count = probe_count + 1
    shutil.rmtree(scratch_path)
    print("goal: check ok " + str(guideline_count) + " guidelines " + str(document_count) + " documents " + str(probe_count) + " red probes")
compiler_source = pathlib.Path("vendor/ape/prolog/ace_to_pl.pl")
if not (compiler_source.is_file()):
    raise AssertionError("requirement failed")
argv = list(sys.argv)
argv.pop(0)
if len(argv) == 0:
    fail("usage", "expected: goal compile <guideline-id> | goal check")
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
        fail("usage", "unknown subcommand: " + subcommand)
