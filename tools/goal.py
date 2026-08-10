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
def check_command():
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
