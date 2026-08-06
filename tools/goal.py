import os
import pathlib
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
def compile_doc(scratch_path, swipl_executable, stage_path, docid, ace_path, lexicon_path):
    ace_bytes = ace_path.read_bytes()
    tail_args = [str(stage_path), docid]
    if lexicon_path != None:
        tail_args.append(str(lexicon_path))
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
def check_doc_queries(scratch_path, swipl_executable, stage_path, pl_path):
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
        compiled = compiled_pl_path(tracked)
        if not vendored:
            if not compiled:
                violation("prolog-inventory", "unauthorized tracked prolog: " + tracked)
def check_documents(scratch_path, swipl_executable, stage_path, guideline_path, ace_paths, docids, lexicon_path):
    pl_dir = guideline_path.joinpath("pl")
    for docid in docids:
        ace_path = ace_paths.get(docid)
        first_bytes = compile_doc(scratch_path, swipl_executable, stage_path, docid, ace_path, lexicon_path)
        second_bytes = compile_doc(scratch_path, swipl_executable, stage_path, docid, ace_path, lexicon_path)
        if first_bytes != second_bytes:
            cleanup_violation(scratch_path, "determinism", "two compiles differ for document: " + docid)
        committed_path = pl_dir.joinpath(docid + ".pl")
        committed_bytes = committed_path.read_bytes()
        if first_bytes != committed_bytes:
            cleanup_violation(scratch_path, "stale", "committed pl differs from fresh compile: " + str(committed_path))
        check_doc_queries(scratch_path, swipl_executable, stage_path, committed_path)
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
        pl_bytes = compile_doc(scratch_path, swipl_executable, stage_path, docid, ace_path, lexicon_path)
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
    exit_one = ["input_utf8", "ape_messages", "empty_drs", "sentence_lines", "unsupported", "safety", "query_failed"]
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
                violation("red-entry", "unsupported entry: " + entry_name)
    for ulex_stem in ulex_stems:
        paired = ulex_stem in ace_stems
        if not paired:
            violation("red-entry", "orphan ulex without ace probe: " + ulex_stem)
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
    stderr_text = result.stderr.decode("utf-8", errors="replace")
    expected_prefix = "ace_to_pl_error(" + class_name + ","
    if not stderr_text.startswith(expected_prefix):
        cleanup_violation(scratch_path, "red-class", "stderr class mismatch for probe: " + probe_name)
    if not stderr_text.endswith(").\n"):
        cleanup_violation(scratch_path, "red-stderr", "stderr lacks canonical term suffix for probe: " + probe_name)
def check_command():
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
