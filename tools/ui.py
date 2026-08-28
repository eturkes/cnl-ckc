import datetime
import fcntl
import hashlib
import html
import os
import pathlib
import re
import shutil
import subprocess
import sys
import tarfile
import tempfile
import urllib.parse
import wsgiref.simple_server
usage_text = "ui: usage: expected: ui serve [<port>] [<root>] | ui render <outdir> [<root>] | ui check [<root>] | ui request <method> <path> [<root>] [--header <name:value>]* [--body <text>] [--body-hex <hex>] [--token <text>] [--now <utc-iso>] [--commit <40hex>] [--fault after-tmp-write]"
census_rx = re.compile("identify the ([0-9]+) payloads below")
ledger_header_text = "# format: docid<TAB>review_sha256<TAB>ace_commit<TAB>verdict<TAB>reviewer<TAB>date<TAB>comment"
commit_url_base = "https://github.com/eturkes/cnl-ckc/commit/"
verdict_field_names = ["verdict", "reviewer", "comment", "review_sha256", "ledger_sha256", "csrf"]
serve_config = {}
def out_line(text):
    line_text = text + "\n"
    data = line_text.encode("utf-8")
    sys.stdout.buffer.write(data)
    sys.stdout.buffer.flush()
def err_line(text):
    line_text = text + "\n"
    data = line_text.encode("utf-8")
    sys.stderr.buffer.write(data)
    sys.stderr.buffer.flush()
def usage_fail():
    err_line(usage_text)
    raise SystemExit(2)
def esc_text(value):
    return html.escape(value, quote=False)
def esc_attr(value):
    return html.escape(value, quote=True)
def url_seg(value):
    return urllib.parse.quote(value, safe="")
def sha256_hex(data):
    digest_value = hashlib.sha256(data)
    return digest_value.hexdigest()
def valid_ui_id(value):
    allowed = set("abcdefghijklmnopqrstuvwxyz0123456789-")
    chars = set(value)
    subset = chars.issubset(allowed)
    if not value:
        return False
    if value.startswith("-"):
        return False
    value_bytes = bytes()
    try:
        value_bytes = value.encode("utf-8")
    except UnicodeEncodeError:
        return False
    if len(value_bytes) > 250:
        return False
    return subset
def valid_post_date(date_text):
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
def valid_hex64(value):
    if len(value) != 64:
        return False
    allowed = set("0123456789abcdef")
    chars = set(value)
    return chars.issubset(allowed)
def valid_hex40(value):
    if len(value) != 40:
        return False
    allowed = set("0123456789abcdef")
    chars = set(value)
    return chars.issubset(allowed)
def commit_field_ok(value):
    return valid_hex40(value)
def field_text_ok(value):
    for char_text in value:
        code_point = ord(char_text)
        if code_point < 32:
            return False
        if code_point == 127:
            return False
    return True
def err(detail):
    return ["err", detail]
def ok(value):
    return ["ok", value]
def result_kind(result):
    copy_list = list(result)
    return copy_list.pop(0)
def result_value(result):
    copy_list = list(result)
    kind = copy_list.pop(0)
    return copy_list.pop(0)
def control_violation(text, gid, relpath):
    bad_point = None
    for ch in text:
        code_point = ord(ch)
        bad = False
        if code_point < 32:
            if code_point != 9:
                if code_point != 10:
                    bad = True
        if code_point == 127:
            bad = True
        if code_point > 8233:
            if code_point < 8239:
                bad = True
        if code_point > 8293:
            if code_point < 8298:
                bad = True
        if bad:
            if bad_point == None:
                bad_point = code_point
    if bad_point != None:
        hex_text = format(bad_point, "04X")
        return "ui: viewmodel: " + gid + " unsupported control U+" + hex_text + " in " + relpath
    return ""
def load_text(path, gid, relpath):
    try:
        data = path.read_bytes()
    except OSError:
        return err("ui: viewmodel: " + gid + " missing " + relpath)
    text = ""
    try:
        text = data.decode("utf-8")
    except UnicodeDecodeError:
        return err("ui: viewmodel: " + gid + " file not UTF-8: " + relpath)
    control_detail = control_violation(text, gid, relpath)
    if control_detail:
        return err(control_detail)
    return ok(text)
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
def parse_evidence(text):
    census_count = None
    census_hit = census_rx.search(text)
    if census_hit != None:
        census_count = census_hit.group(1)
    locator_ids = []
    locator_lines = {}
    locator_counts = {}
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
                                line_count = len(current_payloads)
                                locator_counts.update({current_locator: line_count})
                                if line_count == 1:
                                    payload_line = current_payloads.pop(0)
                                    locator_lines.update({current_locator: payload_line})
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
        line_count = len(current_payloads)
        locator_counts.update({current_locator: line_count})
        if line_count == 1:
            payload_line = current_payloads.pop(0)
            locator_lines.update({current_locator: payload_line})
    ordered_payloads = []
    if not locator_ids:
        for post_blank_line in post_blank_lines:
            ordered_payloads.append(strip_list_label(post_blank_line))
    return [census_count, locator_ids, locator_lines, locator_counts, ordered_payloads]
def parse_tsv_rows(text, field_count, gid, label):
    rows = []
    line_list = text.split("\n")
    for line_text in line_list:
        is_comment = line_text.startswith("#")
        if not is_comment:
            if line_text:
                fields = line_text.split("\t")
                if len(fields) != field_count:
                    return err("ui: viewmodel: " + gid + " malformed " + label + " row: " + line_text)
                rows.append([fields, line_text])
    return ok(rows)
def list_stems(dir_path, suffix):
    stems = []
    if dir_path.is_dir():
        for entry in sorted(dir_path.iterdir()):
            symlink = entry.is_symlink()
            if not symlink:
                if entry.is_file():
                    name_text = entry.name
                    if name_text.endswith(suffix):
                        stems.append(name_text.removesuffix(suffix))
    return sorted(stems)
def valid_asset_name(name_text):
    if not name_text:
        return False
    if name_text.startswith("."):
        return False
    for ch in name_text:
        allowed = False
        if ch.isascii():
            if ch.isalnum():
                allowed = True
        if ch == ".":
            allowed = True
        if ch == "_":
            allowed = True
        if ch == "-":
            allowed = True
        if not allowed:
            return False
    return True
def asset_media_type(name_text):
    if name_text.endswith(".txt"):
        return "text/plain; charset=utf-8"
    if name_text.endswith(".pdf"):
        return "application/pdf"
    return ""
def source_assets(guideline_path):
    names = []
    source_dir = guideline_path.joinpath("source")
    if source_dir.is_symlink():
        return names
    if not source_dir.is_dir():
        return names
    for entry in sorted(source_dir.iterdir()):
        usable = entry.is_file()
        if entry.is_symlink():
            usable = False
        if usable:
            name_text = entry.name
            if valid_asset_name(name_text):
                if asset_media_type(name_text):
                    names.append(name_text)
    return names
def readme_title(guideline_path, gid):
    readme_path = guideline_path.joinpath("README.md")
    try:
        data = readme_path.read_bytes()
    except OSError:
        return ok(gid)
    text = ""
    try:
        text = data.decode("utf-8")
    except UnicodeDecodeError:
        return err("ui: viewmodel: " + gid + " file not UTF-8: README.md")
    control_detail = control_violation(text, gid, "README.md")
    if control_detail:
        return err(control_detail)
    for line_text in text.split("\n"):
        if line_text.startswith("# "):
            heading = line_text.removeprefix("# ")
            heading = heading.strip()
            if heading:
                return ok(heading)
    return ok(gid)
def humanize_section(section_text):
    segs = []
    for raw_seg in section_text.split(">"):
        seg = raw_seg.strip()
        if seg:
            segs.append(seg)
    if len(segs) == 0:
        return ""
    head = segs.pop(0)
    head_parts = head.split(" ")
    head_word = ""
    head_number = ""
    if len(head_parts) == 2:
        head_word = head_parts.pop(0)
        head_number = head_parts.pop(0)
        if not head_number.isascii():
            head_word = ""
        if not head_number.isdigit():
            head_word = ""
    out_segs = []
    if head_word == "Rec":
        out_segs.append("Recommendation " + head_number)
    else:
        drop_head = False
        if head_word == "BOX":
            if len(segs) > 0:
                drop_head = True
        if not drop_head:
            out_segs.append(head)
    for seg in segs:
        out_segs.append(seg)
    title_joiner = " · "
    return title_joiner.join(out_segs)
def human_date(date_text):
    if not date_text.endswith("Z"):
        return date_text
    parts = date_text.removesuffix("Z")
    parts = parts.split("T")
    if len(parts) != 2:
        return date_text
    day_text = parts.pop(0)
    time_text = parts.pop(0)
    return day_text + " " + time_text + " UTC"
def document_title(docid, section_text, page_text, region_id, shared):
    base = humanize_section(section_text)
    if not base:
        return docid
    if not shared:
        return base
    page_number = page_text.strip()
    if page_number.startswith("p"):
        page_number = page_number.removeprefix("p")
    if not page_number.isascii():
        page_number = ""
    if not page_number.isdigit():
        page_number = ""
    last_part = ""
    for part in region_id.split("-"):
        last_part = part
    passage_number = ""
    if last_part.isascii():
        if last_part.isdigit():
            passage_number = str(int(last_part))
    if page_number:
        if passage_number:
            return base + ", page " + page_number + ", passage " + passage_number
    return base + " (" + region_id + ")"
def first_output_line(result):
    stderr_text = result.stderr.decode("utf-8", errors="replace")
    for line_text in stderr_text.splitlines():
        stripped = line_text.strip()
        if stripped:
            return stripped
    stdout_text = result.stdout.decode("utf-8", errors="replace")
    for line_text in stdout_text.splitlines():
        stripped = line_text.strip()
        if stripped:
            return stripped
    return "validator produced no diagnostic"
def goal_py_path():
    argv_copy = list(sys.argv)
    script_text = argv_copy.pop(0)
    script_path = pathlib.Path(script_text)
    resolved = script_path.resolve()
    tool_dir = resolved.parent
    return tool_dir.joinpath("goal.py")
def git_output(work_dir, arg_list):
    command = ["git"]
    for arg_text in arg_list:
        command.append(arg_text)
    result = None
    try:
        result = subprocess.run(command, capture_output=True, cwd=work_dir)
    except OSError:
        return None
    if result.returncode != 0:
        return None
    return result.stdout.decode("utf-8", errors="replace")
def corpus_head_hex(root_path):
    work_dir = str(root_path.resolve())
    top_command = ["git", "rev-parse", "--show-toplevel"]
    top_result = None
    try:
        top_result = subprocess.run(top_command, capture_output=True, cwd=work_dir)
    except OSError:
        return "unavailable"
    if top_result.returncode != 0:
        return ""
    top_text = top_result.stdout.decode("utf-8", errors="replace")
    top_path = pathlib.Path(top_text.strip())
    top_resolved = str(top_path.resolve())
    if top_resolved != work_dir:
        return ""
    head_text = git_output(work_dir, ["rev-parse", "HEAD"])
    if head_text == None:
        return ""
    head_hex = head_text.strip()
    if not valid_hex40(head_hex):
        return ""
    return head_hex
def extract_committed(archive_path, dest_root):
    extract_ok = True
    try:
        archive = tarfile.open(str(archive_path))
        for member in archive.getmembers():
            name_text = member.name
            safe = True
            if name_text.startswith("/"):
                safe = False
            segs = name_text.split("/")
            has_dotdot = ".." in segs
            if has_dotdot:
                safe = False
            if not safe:
                extract_ok = False
            else:
                target = dest_root.joinpath(name_text)
                is_dir = member.isdir()
                is_file = member.isfile()
                if is_dir:
                    target.mkdir(parents=True, exist_ok=True)
                elif is_file:
                    parent_dir = target.parent
                    parent_dir.mkdir(parents=True, exist_ok=True)
                    stream = archive.extractfile(member)
                    payload = stream.read()
                    target.write_bytes(payload)
                else:
                    extract_ok = False
        archive.close()
    except tarfile.TarError:
        extract_ok = False
    except OSError:
        extract_ok = False
    return extract_ok
def materialize_committed(root_path, commit_hex):
    work_dir = str(root_path.resolve())
    scratch_text = tempfile.mkdtemp()
    scratch_dir = pathlib.Path(scratch_text)
    archive_path = scratch_dir.joinpath("committed.tar")
    command = ["git", "archive", "--format=tar", "-o", str(archive_path), commit_hex, "guidelines"]
    result = None
    launched = True
    try:
        result = subprocess.run(command, capture_output=True, cwd=work_dir)
    except OSError:
        launched = False
    built = launched
    if launched:
        if result.returncode != 0:
            built = False
    dest_root = scratch_dir.joinpath("corpus")
    if built:
        dest_root.mkdir(parents=True, exist_ok=True)
        built = extract_committed(archive_path, dest_root)
    if not built:
        shutil.rmtree(scratch_text, ignore_errors=True)
        return ["", None]
    return [scratch_text, dest_root]
def overlay_ledgers(real_root, corpus_root):
    real_guidelines = real_root.joinpath("guidelines")
    corpus_guidelines = corpus_root.joinpath("guidelines")
    if not corpus_guidelines.is_dir():
        return True
    for entry in sorted(corpus_guidelines.iterdir()):
        if entry.is_dir():
            committed_ledger = entry.joinpath("audit", "adjudication.tsv")
            live_ledger = real_guidelines.joinpath(entry.name, "audit", "adjudication.tsv")
            try:
                if live_ledger.is_file():
                    audit_dir = committed_ledger.parent
                    audit_dir.mkdir(parents=True, exist_ok=True)
                    payload = live_ledger.read_bytes()
                    committed_ledger.write_bytes(payload)
                elif committed_ledger.is_file():
                    committed_ledger.unlink()
            except OSError:
                return False
    return True
def worktree_corpus(root_path):
    corpus = {}
    corpus.update({"root": root_path})
    corpus.update({"real_root": root_path})
    corpus.update({"scratch": ""})
    corpus.update({"commit": ""})
    return corpus
def resolve_corpus(root_path):
    corpus = worktree_corpus(root_path)
    head_hex = corpus_head_hex(root_path)
    if head_hex == "unavailable":
        return err("ui: corpus: cannot read the committed guideline files")
    if head_hex == "":
        return ok(corpus)
    pair = materialize_committed(root_path, head_hex)
    pair_copy = list(pair)
    scratch_text = pair_copy.pop(0)
    dest_root = pair_copy.pop(0)
    if scratch_text == "":
        return err("ui: corpus: cannot read the committed guideline files")
    overlay_ok = overlay_ledgers(root_path, dest_root)
    if not overlay_ok:
        shutil.rmtree(scratch_text, ignore_errors=True)
        return err("ui: corpus: cannot read the committed guideline files")
    corpus.update({"root": dest_root})
    corpus.update({"scratch": scratch_text})
    corpus.update({"commit": head_hex})
    return ok(corpus)
def release_corpus(corpus):
    scratch_text = corpus.get("scratch", "")
    if scratch_text:
        shutil.rmtree(scratch_text, ignore_errors=True)
def ace_commit_hex(guideline_path, docid, corpus_commit):
    if not valid_hex40(corpus_commit):
        return ""
    ace_path = guideline_path.joinpath("ace", docid + ".ace")
    work_dir = str(guideline_path.resolve())
    ace_text = str(ace_path.resolve())
    log_command = ["git", "log", "-1", "--format=%H", corpus_commit, "--", ace_text]
    log_result = None
    try:
        log_result = subprocess.run(log_command, capture_output=True, cwd=work_dir)
    except OSError:
        return ""
    if log_result.returncode != 0:
        return ""
    log_text = log_result.stdout.decode("utf-8", errors="replace")
    commit_text = log_text.strip()
    if not valid_hex40(commit_text):
        return ""
    return commit_text
def build_doc_states(guideline_path, gid, docids, review_by_docid, manifest_text):
    ledger_path = guideline_path.joinpath("audit", "adjudication.tsv")
    states = {}
    for docid in docids:
        states.update({docid: "unreviewed"})
    ledger_by_docid = {}
    ledger_order = []
    ledger_file_digest = "absent"
    if not ledger_path.is_file():
        return ok([states, ledger_by_docid, ledger_order, ledger_file_digest])
    try:
        ledger_bytes = ledger_path.read_bytes()
    except OSError:
        return err("ui: viewmodel: " + gid + " missing audit/adjudication.tsv")
    ledger_file_digest = sha256_hex(ledger_bytes)
    goal_py = goal_py_path()
    scratch_text = tempfile.mkdtemp()
    scratch_dir = pathlib.Path(scratch_text)
    ledger_copy = scratch_dir.joinpath("ledger.tsv")
    manifest_copy = scratch_dir.joinpath("manifest.tsv")
    ledger_copy.write_bytes(ledger_bytes)
    manifest_bytes = manifest_text.encode("utf-8")
    manifest_copy.write_bytes(manifest_bytes)
    command = [sys.executable, "-P", str(goal_py), "ledger-validate", str(ledger_copy), str(manifest_copy), "ui"]
    tool_dir = goal_py.parent
    repo_root = tool_dir.parent
    result = subprocess.run(command, capture_output=True, cwd=str(repo_root))
    shutil.rmtree(scratch_text, ignore_errors=True)
    if result.returncode != 0:
        relay = first_output_line(result)
        return err("ui: adjudication ledger invalid: " + relay)
    ledger_text = ""
    try:
        ledger_text = ledger_bytes.decode("utf-8")
    except UnicodeDecodeError:
        return err("ui: viewmodel: " + gid + " file not UTF-8: audit/adjudication.tsv")
    rows_result = parse_tsv_rows(ledger_text, 7, gid, "adjudication")
    if result_kind(rows_result) == "err":
        return rows_result
    current_approved = {}
    current_rejected = {}
    for row_pair in result_value(rows_result):
        pair_copy = list(row_pair)
        fields = pair_copy.pop(0)
        docid = fields.pop(0)
        row_digest = fields.pop(0)
        commit_field = fields.pop(0)
        verdict = fields.pop(0)
        reviewer_field = fields.pop(0)
        date_field = fields.pop(0)
        comment_field = fields.pop(0)
        current_digest = review_by_docid.get(docid, "")
        current = row_digest == current_digest
        row = [row_digest, commit_field, verdict, reviewer_field, date_field, comment_field, current]
        known = docid in ledger_by_docid
        if not known:
            ledger_by_docid.update({docid: []})
        doc_history = ledger_by_docid.get(docid)
        doc_history.append(row)
        ledger_order.append([docid, row_digest, commit_field, verdict, reviewer_field, date_field, comment_field, current])
        if current:
            if verdict == "approved":
                current_approved.update({docid: True})
            else:
                current_rejected.update({docid: True})
    for row_docid in ledger_by_docid:
        has_approved = row_docid in current_approved
        has_rejected = row_docid in current_rejected
        row_state = "stale"
        if has_approved:
            row_state = "approved"
            if has_rejected:
                row_state = "contested"
        elif has_rejected:
            row_state = "rejected"
        states.update({row_docid: row_state})
    return ok([states, ledger_by_docid, ledger_order, ledger_file_digest])
def build_guideline_model(corpus, gid):
    root_path = corpus.get("root", None)
    real_root = corpus.get("real_root", None)
    corpus_commit = corpus.get("commit", "")
    guideline_path = root_path.joinpath("guidelines", gid)
    real_guideline_path = real_root.joinpath("guidelines", gid)
    if not valid_ui_id(gid):
        return err("ui: viewmodel: " + gid + " invalid guideline id")
    coverage_path = guideline_path.joinpath("coverage.tsv")
    if not coverage_path.is_file():
        return err("ui: viewmodel: " + gid + " missing coverage.tsv")
    coverage_result = load_text(coverage_path, gid, "coverage.tsv")
    if result_kind(coverage_result) == "err":
        return coverage_result
    coverage_text = result_value(coverage_result)
    rows_result = parse_tsv_rows(coverage_text, 5, gid, "coverage")
    if result_kind(rows_result) == "err":
        return rows_result
    coverage_rows = result_value(rows_result)
    docids = []
    region_by_docid = {}
    section_by_docid = {}
    page_by_docid = {}
    file_by_docid = {}
    region_rows = []
    file_row_regions = {}
    file_docid_ordinal = {}
    duplicate_docids = []
    ace_counter = 0
    restates_counter = 0
    uncovered_counter = 0
    pending_counter = 0
    for row_pair in coverage_rows:
        pair_copy = list(row_pair)
        fields = pair_copy.pop(0)
        region_id = fields.pop(0)
        file_text = fields.pop(0)
        page_text = fields.pop(0)
        section_text = fields.pop(0)
        status_text = fields.pop(0)
        known_file = file_row_regions.get(file_text, None)
        if known_file == None:
            known_file = []
            file_row_regions.update({file_text: known_file})
        ordinal = len(known_file)
        known_file.append(region_id)
        is_ace = status_text.startswith("ace(")
        if is_ace:
            closed_status = status_text.endswith(")")
            if not closed_status:
                return err("ui: viewmodel: " + gid + " malformed coverage row: " + region_id)
            docid = status_text.removeprefix("ace(")
            docid = docid.removesuffix(")")
            if not valid_ui_id(docid):
                return err("ui: viewmodel: " + gid + " invalid docid " + docid)
            seen = region_by_docid.get(docid, "")
            if seen:
                duplicate_docids.append(docid)
            else:
                docids.append(docid)
                region_by_docid.update({docid: region_id})
                section_by_docid.update({docid: section_text})
                page_by_docid.update({docid: page_text})
                file_by_docid.update({docid: file_text})
                file_docid_ordinal.update({docid: ordinal})
            ace_counter = ace_counter + 1
        else:
            display_text = ""
            if status_text.startswith("restates("):
                inner = status_text.removeprefix("restates(")
                if not inner.endswith(")"):
                    return err("ui: viewmodel: " + gid + " malformed coverage row: " + region_id)
                inner = inner.removesuffix(")")
                if not inner:
                    return err("ui: viewmodel: " + gid + " malformed coverage row: " + region_id)
                display_text = "Restates " + inner
                restates_counter = restates_counter + 1
            elif status_text.startswith("uncovered("):
                inner = status_text.removeprefix("uncovered(")
                if not inner.endswith(")"):
                    return err("ui: viewmodel: " + gid + " malformed coverage row: " + region_id)
                inner = inner.removesuffix(")")
                reason_parts = inner.split(": ", 1)
                reason_text = reason_parts.pop()
                if len(reason_parts) == 0:
                    return err("ui: viewmodel: " + gid + " malformed coverage row: " + region_id)
                if not reason_text:
                    return err("ui: viewmodel: " + gid + " malformed coverage row: " + region_id)
                display_text = "Not covered — " + reason_text
                uncovered_counter = uncovered_counter + 1
            elif status_text == "pending":
                display_text = "Pending"
                pending_counter = pending_counter + 1
            else:
                return err("ui: viewmodel: " + gid + " malformed coverage row: " + region_id)
            region_rows.append([region_id, display_text, section_text])
    if duplicate_docids:
        return err("ui: viewmodel: " + gid + " duplicate docid " + min(duplicate_docids))
    sorted_docids = sorted(docids)
    section_doc_counts = {}
    for docid in sorted_docids:
        section_text = section_by_docid.get(docid, "")
        seen_count = section_doc_counts.get(section_text, 0)
        section_doc_counts.update({section_text: seen_count + 1})
    title_by_docid = {}
    for docid in sorted_docids:
        section_text = section_by_docid.get(docid, "")
        shared = False
        if section_doc_counts.get(section_text, 0) > 1:
            shared = True
        page_text = page_by_docid.get(docid, "")
        region_id = region_by_docid.get(docid, "")
        title_by_docid.update({docid: document_title(docid, section_text, page_text, region_id, shared)})
    ace_stems = list_stems(guideline_path.joinpath("ace"), ".ace")
    for docid in sorted_docids:
        present = docid in ace_stems
        if not present:
            return err("ui: viewmodel: " + gid + " doc " + docid + " missing ace file")
    for stem in ace_stems:
        claimed = region_by_docid.get(stem, "")
        if not claimed:
            return err("ui: viewmodel: " + gid + " orphan ace file " + stem + ".ace")
    manifest_path = guideline_path.joinpath("audit", "review-manifest.tsv")
    if not manifest_path.is_file():
        return err("ui: viewmodel: " + gid + " missing audit/review-manifest.tsv")
    manifest_result = load_text(manifest_path, gid, "audit/review-manifest.tsv")
    if result_kind(manifest_result) == "err":
        return manifest_result
    manifest_text = result_value(manifest_result)
    manifest_rows_result = parse_tsv_rows(manifest_text, 6, gid, "review-manifest")
    if result_kind(manifest_rows_result) == "err":
        return manifest_rows_result
    ace_digest_by_docid = {}
    payload_digest_by_docid = {}
    review_by_docid = {}
    manifest_docids = []
    for row_pair in result_value(manifest_rows_result):
        pair_copy = list(row_pair)
        fields = pair_copy.pop(0)
        docid = fields.pop(0)
        ace_digest = fields.pop(0)
        coverage_digest = fields.pop(0)
        payload_digest = fields.pop(0)
        dup_row = docid in manifest_docids
        if dup_row:
            return err("ui: viewmodel: " + gid + " duplicate review-manifest doc " + docid)
        manifest_docids.append(docid)
        ace_digest_by_docid.update({docid: ace_digest})
        payload_digest_by_docid.update({docid: payload_digest})
        clause_digest = fields.pop(0)
        review_digest = fields.pop(0)
        review_by_docid.update({docid: review_digest})
    for docid in sorted_docids:
        present = review_by_docid.get(docid, "")
        if not present:
            return err("ui: viewmodel: " + gid + " doc " + docid + " missing review-manifest row")
    for docid in sorted(manifest_docids):
        claimed = region_by_docid.get(docid, "")
        if not claimed:
            return err("ui: viewmodel: " + gid + " orphan review-manifest doc " + docid)
    pl_stems = list_stems(guideline_path.joinpath("pl"), ".pl")
    for docid in sorted_docids:
        present = docid in pl_stems
        if not present:
            return err("ui: viewmodel: " + gid + " doc " + docid + " missing Prolog file")
    for stem in pl_stems:
        claimed = region_by_docid.get(stem, "")
        if not claimed:
            return err("ui: viewmodel: " + gid + " orphan Prolog file " + stem + ".pl")
    evidence_cache = {}
    payload_by_docid = {}
    for docid in sorted_docids:
        file_text = file_by_docid.get(docid, "")
        region_id = region_by_docid.get(docid, "")
        file_hostile = False
        if file_text.startswith("/"):
            file_hostile = True
        has_backslash = "\\" in file_text
        if has_backslash:
            file_hostile = True
        file_segs = file_text.split("/")
        has_dotdot = ".." in file_segs
        if has_dotdot:
            file_hostile = True
        if file_hostile:
            return err("ui: viewmodel: " + gid + " doc " + docid + " region " + region_id + " unresolved in " + file_text)
        parsed = evidence_cache.get(file_text, None)
        if parsed == None:
            evidence_path = guideline_path.joinpath(file_text)
            evidence_result = load_text(evidence_path, gid, file_text)
            if result_kind(evidence_result) == "err":
                return evidence_result
            parsed = parse_evidence(result_value(evidence_result))
            evidence_cache.update({file_text: parsed})
        parsed_copy = list(parsed)
        census_count = parsed_copy.pop(0)
        locator_ids = parsed_copy.pop(0)
        locator_lines = parsed_copy.pop(0)
        locator_counts = parsed_copy.pop(0)
        ordered_payloads = parsed_copy.pop(0)
        payload_line = ""
        if locator_ids:
            seen_count = 0
            for locator_id in locator_ids:
                if locator_id == region_id:
                    seen_count = seen_count + 1
            if seen_count == 0:
                return err("ui: viewmodel: " + gid + " doc " + docid + " region " + region_id + " unresolved in " + file_text)
            if seen_count > 1:
                return err("ui: viewmodel: " + gid + " duplicate region locator " + region_id + " in " + file_text)
            line_count = locator_counts.get(region_id, 0)
            if line_count == 0:
                return err("ui: viewmodel: " + gid + " doc " + docid + " region " + region_id + " has empty payload")
            if line_count != 1:
                return err("ui: viewmodel: " + gid + " doc " + docid + " region " + region_id + " unresolved in " + file_text)
            payload_line = locator_lines.get(region_id, "")
        else:
            file_regions = file_row_regions.get(file_text, [])
            row_total = len(file_regions)
            payload_total = len(ordered_payloads)
            if census_count != str(payload_total):
                return err("ui: viewmodel: " + gid + " region census mismatch " + file_text + " coverage=" + str(row_total) + " payloads=" + str(payload_total))
            if row_total != payload_total:
                return err("ui: viewmodel: " + gid + " region census mismatch " + file_text + " coverage=" + str(row_total) + " payloads=" + str(payload_total))
            ordinal = file_docid_ordinal.get(docid, 0)
            scan_index = 0
            for candidate in ordered_payloads:
                if scan_index == ordinal:
                    payload_line = candidate
                scan_index = scan_index + 1
            if not payload_line:
                return err("ui: viewmodel: " + gid + " doc " + docid + " region " + region_id + " has empty payload")
        payload_by_docid.update({docid: payload_line})
    states_result = build_doc_states(guideline_path, gid, sorted_docids, review_by_docid, manifest_text)
    if result_kind(states_result) == "err":
        return states_result
    states_pair = result_value(states_result)
    states_copy = list(states_pair)
    doc_states = states_copy.pop(0)
    ledger_by_docid = states_copy.pop(0)
    ledger_order = states_copy.pop(0)
    ledger_file_digest = states_copy.pop(0)
    model = {}
    model.update({"gid": gid})
    model.update({"path": guideline_path})
    model.update({"real_path": real_guideline_path})
    model.update({"corpus_commit": corpus_commit})
    title_result = readme_title(guideline_path, gid)
    if result_kind(title_result) == "err":
        return title_result
    model.update({"title": result_value(title_result)})
    model.update({"docids": sorted_docids})
    model.update({"region_by_docid": region_by_docid})
    model.update({"section_by_docid": section_by_docid})
    model.update({"title_by_docid": title_by_docid})
    model.update({"page_by_docid": page_by_docid})
    model.update({"file_by_docid": file_by_docid})
    source_names = source_assets(guideline_path)
    pdf_names = []
    for name_text in source_names:
        if name_text.endswith(".pdf"):
            pdf_names.append(name_text)
    pdf_name = ""
    if len(pdf_names) == 1:
        pdf_name = pdf_names.pop(0)
    model.update({"source_names": source_names})
    model.update({"pdf_name": pdf_name})
    model.update({"payload_by_docid": payload_by_docid})
    model.update({"ace_digest_by_docid": ace_digest_by_docid})
    model.update({"payload_digest_by_docid": payload_digest_by_docid})
    model.update({"review_by_docid": review_by_docid})
    model.update({"doc_states": doc_states})
    model.update({"ledger_by_docid": ledger_by_docid})
    model.update({"ledger_order": ledger_order})
    model.update({"ledger_file_digest": ledger_file_digest})
    model.update({"region_rows": region_rows})
    counts = {}
    counts.update({"regions": len(coverage_rows)})
    counts.update({"ace": ace_counter})
    counts.update({"restates": restates_counter})
    counts.update({"uncovered": uncovered_counter})
    counts.update({"pending": pending_counter})
    approved_count = 0
    rejected_count = 0
    contested_count = 0
    stale_count = 0
    unreviewed_count = 0
    for docid in sorted_docids:
        state = doc_states.get(docid, "unreviewed")
        if state == "approved":
            approved_count = approved_count + 1
        elif state == "rejected":
            rejected_count = rejected_count + 1
        elif state == "contested":
            contested_count = contested_count + 1
        elif state == "stale":
            stale_count = stale_count + 1
        else:
            unreviewed_count = unreviewed_count + 1
    counts.update({"approved": approved_count})
    counts.update({"rejected": rejected_count})
    counts.update({"contested": contested_count})
    counts.update({"stale": stale_count})
    counts.update({"unreviewed": unreviewed_count})
    counts.update({"decisions": len(ledger_order)})
    counts.update({"reviewed": len(ledger_by_docid)})
    model.update({"counts": counts})
    return ok(model)
def build_viewmodel(corpus):
    root_path = corpus.get("root", None)
    guidelines_root = root_path.joinpath("guidelines")
    root_missing = False
    symlink = guidelines_root.is_symlink()
    if symlink:
        root_missing = True
    if not guidelines_root.is_dir():
        root_missing = True
    if root_missing:
        return err("ui: viewmodel: missing guidelines directory")
    gids = []
    for entry in sorted(guidelines_root.iterdir()):
        symlink = entry.is_symlink()
        if not symlink:
            if entry.is_dir():
                gids.append(entry.name)
    models = []
    for gid in gids:
        model_result = build_guideline_model(corpus, gid)
        if result_kind(model_result) == "err":
            return model_result
        models.append(result_value(model_result))
    return ok(models)
hl_stop_words = set(["a", "an", "the", "every", "each", "no", "all", "some", "any", "this", "that", "these", "those", "such", "is", "are", "was", "were", "be", "been", "being", "has", "have", "had", "does", "do", "did", "should", "must", "may", "can", "cannot", "might", "will", "would", "shall", "could", "if", "then", "and", "or", "nor", "but", "not", "it", "its", "itself", "they", "them", "their", "he", "she", "who", "whom", "whose", "which", "what", "where", "when", "there", "something", "somebody", "someone", "everything", "everybody", "everyone", "nothing", "nobody", "of", "for", "with", "without", "during", "to", "at", "in", "on", "by", "from", "as", "against", "about", "after", "before", "through", "under", "over", "above", "below", "into", "onto", "per", "within", "between", "among", "around", "near", "than", "least", "most", "more", "less", "fewer", "greater"])
hl_token_rx = re.compile("([0-9A-Za-z]+(?:-[0-9A-Za-z]+)*)")
hl_hover_classes = 48
hl_hue_count = 12
def hl_slice(text, span_start, span_end):
    return text.__getitem__(slice(span_start, span_end))
def hl_int_field(field_text):
    bad = 0 - 1
    value = bad
    try:
        value = int(field_text)
    except ValueError:
        return bad
    if value < 0:
        return bad
    if str(value) != field_text:
        return bad
    return value
def hl_spans_out(side_rows, index_by_group):
    spans = []
    for span_row in sorted(side_rows):
        row_copy = list(span_row)
        span_start = row_copy.pop(0)
        span_end = row_copy.pop(0)
        group_id = row_copy.pop(0)
        spans.append(tuple([span_start, span_end, index_by_group.get(group_id, 0)]))
    return spans
def hl_parse_align(align_text, src_text, ace_text):
    if not align_text.endswith("\n"):
        return err("missing trailing newline")
    body = align_text.removesuffix("\n")
    if not body:
        return err("empty file")
    src_rows = []
    ace_rows = []
    src_groups = set([])
    ace_groups = set([])
    line_no = 0
    for row in body.split("\n"):
        line_no = line_no + 1
        where = "row " + str(line_no) + ": "
        fields = row.split("\t")
        if len(fields) != 4:
            return err(where + "expected 4 tab-separated fields")
        group_text = fields.pop(0)
        side = fields.pop(0)
        start_text = fields.pop(0)
        span = fields.pop(0)
        group_id = hl_int_field(group_text)
        if group_id < 0:
            return err(where + "group must be a canonical decimal")
        span_start = hl_int_field(start_text)
        if span_start < 0:
            return err(where + "start must be a canonical decimal")
        if not span:
            return err(where + "empty span")
        side_text = ""
        if side == "src":
            side_text = src_text
        else:
            if side == "ace":
                side_text = ace_text
            else:
                return err(where + "side must be src or ace")
        span_end = span_start + len(span)
        if span_end > len(side_text):
            return err(where + "span out of range")
        if hl_slice(side_text, span_start, span_end) != span:
            return err(where + "span does not match the text at start")
        if side == "src":
            src_rows.append(tuple([span_start, span_end, group_id]))
            src_groups.add(group_id)
        else:
            ace_rows.append(tuple([span_start, span_end, group_id]))
            ace_groups.add(group_id)
    if src_groups != ace_groups:
        return err("every group needs both a src span and an ace span")
    side_packs = []
    side_packs.append(tuple(["src", src_rows]))
    side_packs.append(tuple(["ace", ace_rows]))
    for side_pack in side_packs:
        pack_copy = list(side_pack)
        side_name = pack_copy.pop(0)
        rows_for_side = pack_copy.pop(0)
        prev_end = 0
        for span_row in sorted(rows_for_side):
            row_copy = list(span_row)
            span_start = row_copy.pop(0)
            span_end = row_copy.pop(0)
            if span_start < prev_end:
                return err("overlapping " + side_name + " spans")
            prev_end = span_end
    index_by_group = {}
    for span_row in sorted(ace_rows):
        row_copy = list(span_row)
        row_copy.pop(0)
        row_copy.pop(0)
        group_id = row_copy.pop(0)
        seen = group_id in index_by_group
        if not seen:
            index_by_group.update({group_id: len(index_by_group)})
    data = {}
    data.update({"src": hl_spans_out(src_rows, index_by_group)})
    data.update({"ace": hl_spans_out(ace_rows, index_by_group)})
    data.update({"count": len(index_by_group)})
    return ok(data)
def hl_mark_html(text, index):
    class_attr = ""
    if index < hl_hover_classes:
        class_attr = " class=\"t" + str(index) + "\""
    return "<mark" + class_attr + ">" + esc_text(text) + "</mark>"
def hl_kw_html(text):
    html_parts = []
    is_token = False
    for piece in hl_token_rx.split(text):
        muted = False
        if is_token:
            folded = piece.casefold()
            if folded in hl_stop_words:
                muted = True
        if muted:
            html_parts.append("<span class=\"kw\">" + esc_text(piece) + "</span>")
        else:
            html_parts.append(esc_text(piece))
        is_token = not is_token
    empty_text = ""
    return empty_text.join(html_parts)
def hl_marked_html(text, spans, kw_gaps):
    html_parts = []
    cursor = 0
    for span_row in spans:
        row_copy = list(span_row)
        span_start = row_copy.pop(0)
        span_end = row_copy.pop(0)
        tindex = row_copy.pop(0)
        gap = hl_slice(text, cursor, span_start)
        if gap:
            if kw_gaps:
                html_parts.append(hl_kw_html(gap))
            else:
                html_parts.append(esc_text(gap))
        html_parts.append(hl_mark_html(hl_slice(text, span_start, span_end), tindex))
        cursor = span_end
    tail = hl_slice(text, cursor, len(text))
    if tail:
        if kw_gaps:
            html_parts.append(hl_kw_html(tail))
        else:
            html_parts.append(esc_text(tail))
    empty_text = ""
    return empty_text.join(html_parts)
def hl_link_data(ace_text, payload_line, align_data):
    data = {}
    if align_data == None:
        data.update({"matched": 0})
        data.update({"payload_html": esc_text(payload_line)})
        data.update({"ace_html": hl_kw_html(ace_text)})
        return data
    data.update({"matched": align_data.get("count", 0)})
    data.update({"payload_html": hl_marked_html(payload_line, align_data.get("src", []), False)})
    data.update({"ace_html": hl_marked_html(ace_text, align_data.get("ace", []), True)})
    return data
palette = {"body": ["#111827", "#ffffff"], "link": ["#1d4ed8", "#ffffff"], "muted": ["#4b5563", "#ffffff"], "chip-approved": ["#14532d", "#dcfce7"], "chip-rejected": ["#7f1d1d", "#fee2e2"], "chip-contested": ["#4c1d95", "#ede9fe"], "chip-stale": ["#78350f", "#fef3c7"], "chip-unreviewed": ["#1f2937", "#e5e7eb"], "mark-0": ["#111827", "#dbeafe"], "mark-active-0": ["#111827", "#bfdbfe"], "mark-1": ["#111827", "#fef9c3"], "mark-active-1": ["#111827", "#fef08a"], "mark-2": ["#111827", "#f3e8ff"], "mark-active-2": ["#111827", "#e9d5ff"], "mark-3": ["#111827", "#ffedd5"], "mark-active-3": ["#111827", "#fed7aa"], "mark-4": ["#111827", "#ccfbf1"], "mark-active-4": ["#111827", "#99f6e4"], "mark-5": ["#111827", "#ffe4e6"], "mark-active-5": ["#111827", "#fecdd3"], "mark-6": ["#111827", "#dcfce7"], "mark-active-6": ["#111827", "#bbf7d0"], "mark-7": ["#111827", "#fae8ff"], "mark-active-7": ["#111827", "#f5d0fe"], "mark-8": ["#111827", "#cffafe"], "mark-active-8": ["#111827", "#a5f3fc"], "mark-9": ["#111827", "#ecfccb"], "mark-active-9": ["#111827", "#d9f99d"], "mark-10": ["#111827", "#e0e7ff"], "mark-active-10": ["#111827", "#c7d2fe"], "mark-11": ["#111827", "#e7e5e4"], "mark-active-11": ["#111827", "#d6d3d1"], "mark-line-0": ["#2563eb", "#ffffff"], "mark-line-1": ["#a16207", "#ffffff"], "mark-line-2": ["#7c3aed", "#ffffff"], "mark-line-3": ["#c2410c", "#ffffff"], "mark-line-4": ["#0f766e", "#ffffff"], "mark-line-5": ["#be123c", "#ffffff"], "mark-line-6": ["#15803d", "#ffffff"], "mark-line-7": ["#a21caf", "#ffffff"], "mark-line-8": ["#0e7490", "#ffffff"], "mark-line-9": ["#4d7c0f", "#ffffff"], "mark-line-10": ["#4f46e5", "#ffffff"], "mark-line-11": ["#57534e", "#ffffff"]}
def pal_fg(role):
    pair = palette.get(role, [])
    pair_copy = list(pair)
    return pair_copy.pop(0)
def pal_bg(role):
    pair = palette.get(role, [])
    pair_copy = list(pair)
    pair_copy.pop(0)
    return pair_copy.pop(0)
def build_css():
    body_fg = pal_fg("body")
    body_bg = pal_bg("body")
    link_fg = pal_fg("link")
    line_color = pal_bg("chip-unreviewed")
    lines = []
    lines.append("body { margin: 0 auto; max-width: 72rem; padding: 0 1.5rem 4rem; font-family: system-ui, sans-serif; line-height: 1.55; color: " + body_fg + "; background: " + body_bg + "; }")
    lines.append("a { color: " + link_fg + "; }")
    lines.append("a:focus-visible, summary:focus-visible { outline: 3px solid " + link_fg + "; outline-offset: 2px; }")
    lines.append(".skip { position: absolute; left: -999px; top: 0; padding: 0.5rem 1rem; background: " + body_bg + "; color: " + link_fg + "; }")
    lines.append(".skip:focus { left: 0; z-index: 1; }")
    lines.append("nav.crumbs { padding: 1rem 0; border-bottom: 1px solid " + line_color + "; }")
    lines.append("h1 { font-size: 1.5rem; }")
    lines.append("h2 { font-size: 1.25rem; }")
    lines.append("h3 { font-size: 1.05rem; }")
    lines.append("h1 a.source { font-size: 1rem; font-weight: 400; margin-left: 0.5rem; }")
    lines.append("table { border-collapse: collapse; width: 100%; margin: 1rem 0; }")
    lines.append("th, td { text-align: left; padding: 0.4rem 0.6rem; border-bottom: 1px solid " + line_color + "; vertical-align: top; }")
    lines.append("th { border-bottom: 2px solid " + body_fg + "; }")
    lines.append("table.compact { width: auto; }")
    lines.append("table.compact th, table.compact td { padding-right: 2rem; }")
    lines.append("table.records { table-layout: fixed; }")
    lines.append("table.records th { box-sizing: border-box; }")
    lines.append("table.records th:nth-child(1) { width: 12%; }")
    lines.append("table.records th:nth-child(2) { width: 18%; }")
    lines.append("table.records th:nth-child(3) { width: 22%; }")
    lines.append("table.records th:nth-child(4) { width: 10%; }")
    lines.append(".chip { display: inline-block; padding: 0.1rem 0.6rem; border-radius: 999px; font-size: 0.85rem; font-weight: 600; }")
    lines.append(".chip-approved { color: " + pal_fg("chip-approved") + "; background: " + pal_bg("chip-approved") + "; }")
    lines.append(".chip-rejected { color: " + pal_fg("chip-rejected") + "; background: " + pal_bg("chip-rejected") + "; }")
    lines.append(".chip-contested { color: " + pal_fg("chip-contested") + "; background: " + pal_bg("chip-contested") + "; }")
    lines.append(".chip-stale { color: " + pal_fg("chip-stale") + "; background: " + pal_bg("chip-stale") + "; }")
    lines.append(".chip-unreviewed { color: " + pal_fg("chip-unreviewed") + "; background: " + pal_bg("chip-unreviewed") + "; }")
    lines.append("pre { padding: 0.75rem 1rem; border: 1px solid " + line_color + "; white-space: pre-wrap; overflow-x: auto; }")
    lines.append("pre, code { font-family: ui-monospace, Menlo, Consolas, monospace; font-size: 0.95rem; }")
    lines.append("pre.prose { font-family: Georgia, serif; font-size: 1.05rem; overflow-wrap: anywhere; }")
    lines.append("dt { font-weight: 600; margin-top: 0.6rem; }")
    lines.append("dd { margin-left: 0; }")
    lines.append("summary { cursor: pointer; }")
    lines.append("section { margin: 1.5rem 0; }")
    lines.append("nav.docnav { padding: 1rem 0; border-top: 1px solid " + line_color + "; }")
    lines.append("footer.scope { margin-top: 2rem; padding: 1rem 0; border-top: 1px solid " + line_color + "; font-size: 0.9rem; }")
    lines.append("form label { display: block; margin-top: 1rem; font-weight: 600; }")
    lines.append("fieldset { border: 0; margin: 1rem 0 0; padding: 0; max-width: 28rem; }")
    lines.append("legend { font-weight: 600; padding: 0; }")
    lines.append("fieldset label { margin-top: 0.5rem; font-weight: 400; }")
    lines.append("input[type=\"text\"], textarea { display: block; box-sizing: border-box; width: 100%; max-width: 28rem; margin-top: 0.3rem; padding: 0.45rem 0.6rem; border: 1px solid " + line_color + "; font-family: inherit; font-size: 1rem; color: " + body_fg + "; background: " + body_bg + "; }")
    lines.append("textarea { min-height: 6rem; }")
    lines.append("input[type=\"radio\"] { accent-color: " + body_fg + "; }")
    lines.append("input[type=\"text\"]:focus-visible, input[type=\"radio\"]:focus-visible, textarea:focus-visible, button:focus-visible { outline: 3px solid " + link_fg + "; outline-offset: 2px; }")
    lines.append("button { margin-top: 1.25rem; padding: 0.5rem 1.2rem; border: 1px solid " + body_fg + "; font-family: inherit; font-size: 1rem; font-weight: 600; color: " + body_bg + "; background: " + body_fg + "; }")
    muted_fg = pal_fg("muted")
    mark_bg = pal_bg("mark-0")
    lines.append("mark { background: " + mark_bg + "; color: inherit; text-decoration: underline dotted " + muted_fg + "; text-underline-offset: 0.15em; }")
    lines.append(".kw { color: " + muted_fg + "; }")
    lines.append(".hl-note { color: " + muted_fg + "; font-size: 0.9rem; }")
    hue = 0
    for hue_ignored in range(hl_hue_count):
        if hue > 0:
            selector_parts = []
            class_index = hue
            for step_ignored in range(hl_hover_classes):
                if class_index < hl_hover_classes:
                    selector_parts.append("mark.t" + str(class_index))
                class_index = class_index + hl_hue_count
            comma = ", "
            lines.append(comma.join(selector_parts) + " { background: " + pal_bg("mark-" + str(hue)) + "; }")
        hue = hue + 1
    hue = 0
    for hue_ignored in range(hl_hue_count):
        selector_parts = []
        class_index = hue
        for step_ignored in range(hl_hover_classes):
            if class_index < hl_hover_classes:
                selector_parts.append("mark.t" + str(class_index))
            class_index = class_index + hl_hue_count
        comma = ", "
        lines.append(comma.join(selector_parts) + " { text-decoration-color: " + pal_fg("mark-line-" + str(hue)) + "; }")
        hue = hue + 1
    hue = 0
    for hover_index in range(hl_hover_classes):
        class_name = "t" + str(hover_index)
        lines.append("main:has(mark." + class_name + ":hover) mark." + class_name + " { background: " + pal_bg("mark-active-" + str(hue)) + "; text-decoration-style: solid; }")
        hue = hue + 1
        if hue == hl_hue_count:
            hue = 0
    lines.append("@media print {")
    lines.append("body { max-width: none; padding: 0; }")
    lines.append("nav.crumbs, nav.docnav, .skip, form, .verdict-entry, .hl-note { display: none; }")
    lines.append("mark, mark[class] { background: none; text-decoration-color: " + muted_fg + "; }")
    lines.append("details::details-content { content-visibility: visible; }")
    lines.append("pre { border: none; padding: 0; white-space: pre-wrap; overflow-x: visible; }")
    lines.append("a { color: inherit; text-decoration: none; }")
    lines.append(".chip { border: 1px solid " + body_fg + "; background: none; color: inherit; }")
    lines.append("}")
    joiner = "\n"
    return joiner.join(lines)
css_text = build_css()
scope_line_text = "This page reports what the loaded guideline documents state. It does not give clinical advice."
hl_note_text = "Highlights link matching parts of the passage and the ACE text. Linked parts share one color. Colors repeat when a page has many links. Point at a highlight to emphasize its linked parts."
refusal_refused_text = "The request was refused. Open the document page again from this site and submit the decision again."
refusal_invalid_form_text = "The submitted form was not valid. Go back to the document page, reload it, and submit the decision again."
refusal_subject_text = "The document or its source changed after this page was loaded. The decision was not recorded. Open the document page again and check the current version."
refusal_ledger_text = "Another decision was recorded for this guideline before this one. The decision was not recorded. Open the document page again and check the current state."
def comment_safe(detail):
    safe_chars = []
    for ch in detail:
        keep = False
        if ch.isascii():
            if ch.isalnum():
                keep = True
        if ch == " ":
            keep = True
        if ch == ":":
            keep = True
        if ch == "_":
            keep = True
        if ch == ".":
            keep = True
        if keep:
            safe_chars.append(ch)
        else:
            safe_chars.append("_")
    empty_text = ""
    return empty_text.join(safe_chars)
def page_html(title_text, crumb_html, body_html):
    parts = []
    parts.append("<!doctype html>")
    parts.append("<html lang=\"en\">")
    parts.append("<head>")
    parts.append("<meta charset=\"utf-8\">")
    parts.append("<title>" + esc_text(title_text) + " — cnl-ckc reviewer</title>")
    parts.append("<style>")
    parts.append(css_text)
    parts.append("</style>")
    parts.append("</head>")
    parts.append("<body>")
    parts.append("<a class=\"skip\" href=\"#main\">Skip to content</a>")
    parts.append("<nav class=\"crumbs\">" + crumb_html + "</nav>")
    parts.append("<main id=\"main\">")
    parts.append(body_html)
    parts.append("</main>")
    parts.append("<footer class=\"scope\"><p>" + esc_text(scope_line_text) + "</p></footer>")
    parts.append("</body>")
    parts.append("</html>")
    joiner = "\n"
    return joiner.join(parts) + "\n"
def state_label(state):
    if state == "stale":
        return "Outdated"
    return state.capitalize()
def chip_html(state):
    return "<span class=\"chip chip-" + state + "\">" + state_label(state) + "</span>"
def reviewer_roster(ledger_order):
    names = []
    latest_date = ""
    latest_name = ""
    for row in ledger_order:
        fields_copy = list(row)
        fields_copy.pop(0)
        fields_copy.pop(0)
        fields_copy.pop(0)
        fields_copy.pop(0)
        name_text = fields_copy.pop(0)
        date_text = fields_copy.pop(0)
        if name_text:
            known = name_text in names
            if not known:
                names.append(name_text)
            older = date_text < latest_date
            if not older:
                latest_date = date_text
                latest_name = name_text
    return [sorted(names), latest_name]
def datalist_html(names):
    option_parts = []
    for name_text in names:
        option_parts.append("<option value=\"" + esc_attr(name_text) + "\"></option>")
    option_joiner = ""
    return "<datalist id=\"reviewer-names\">" + option_joiner.join(option_parts) + "</datalist>"
def doc_tally(ledger_by_docid, docid):
    approved_count = 0
    rejected_count = 0
    earlier_count = 0
    for row in ledger_by_docid.get(docid, []):
        row_copy = list(row)
        row_copy.pop(0)
        row_copy.pop(0)
        verdict = row_copy.pop(0)
        row_copy.pop(0)
        row_copy.pop(0)
        row_copy.pop(0)
        current = row_copy.pop(0)
        if current:
            if verdict == "approved":
                approved_count = approved_count + 1
            else:
                rejected_count = rejected_count + 1
        else:
            earlier_count = earlier_count + 1
    return [approved_count, rejected_count, earlier_count]
def tally_text(tally):
    tally_copy = list(tally)
    approved_count = tally_copy.pop(0)
    rejected_count = tally_copy.pop(0)
    earlier_count = tally_copy.pop(0)
    current_parts = []
    if approved_count > 0:
        current_parts.append(str(approved_count) + " approved")
    if rejected_count > 0:
        current_parts.append(str(rejected_count) + " rejected")
    sentences = []
    if current_parts:
        part_joiner = " and "
        sentences.append("Decisions on this version: " + part_joiner.join(current_parts) + ".")
    elif earlier_count > 0:
        sentences.append("No decision is recorded on this version.")
    else:
        sentences.append("No decision is recorded.")
    if earlier_count > 0:
        sentences.append("Decisions on earlier versions: " + str(earlier_count) + ".")
    joiner = " "
    return joiner.join(sentences)
def tally_cell(tally):
    tally_copy = list(tally)
    approved_count = tally_copy.pop(0)
    rejected_count = tally_copy.pop(0)
    earlier_count = tally_copy.pop(0)
    parts = []
    if approved_count > 0:
        parts.append(str(approved_count) + " approved")
    if rejected_count > 0:
        parts.append(str(rejected_count) + " rejected")
    if earlier_count > 0:
        parts.append(str(earlier_count) + " earlier")
    if not parts:
        return "None"
    joiner = ", "
    return joiner.join(parts)
def review_summary_text(counts, doc_total):
    decision_count = counts.get("decisions", 0)
    if decision_count == 0:
        return "No decisions are recorded for the " + str(doc_total) + " documents in this guideline."
    reviewed_count = counts.get("reviewed", 0)
    return "Reviewers recorded " + str(decision_count) + " decisions on " + str(reviewed_count) + " of " + str(doc_total) + " documents."
def build_index_page(models):
    rows = []
    for model in models:
        gid = model.get("gid", "")
        counts = model.get("counts", {})
        docids = model.get("docids", [])
        cells = []
        cells.append("<td><a href=\"g/" + url_seg(gid) + "/index.html\">" + esc_text(model.get("title", gid)) + "</a></td>")
        cells.append("<td>" + str(len(docids)) + "</td>")
        cells.append("<td>" + str(counts.get("regions", 0)) + "</td>")
        cells.append("<td>" + str(counts.get("approved", 0)) + "</td>")
        cells.append("<td>" + str(counts.get("rejected", 0)) + "</td>")
        cells.append("<td>" + str(counts.get("contested", 0)) + "</td>")
        cells.append("<td>" + str(counts.get("stale", 0)) + "</td>")
        cells.append("<td>" + str(counts.get("unreviewed", 0)) + "</td>")
        empty_text = ""
        rows.append("<tr>" + empty_text.join(cells) + "</tr>")
    parts = []
    parts.append("<h1>Guidelines</h1>")
    parts.append("<section>")
    parts.append("<table>")
    parts.append("<thead><tr><th>Guideline</th><th>Documents</th><th>Passages</th><th>Approved</th><th>Rejected</th><th>Contested</th><th>Outdated</th><th>Unreviewed</th></tr></thead>")
    joiner = "\n"
    parts.append("<tbody>" + joiner.join(rows) + "</tbody>")
    parts.append("</table>")
    parts.append("</section>")
    body_html = joiner.join(parts)
    return page_html("Guidelines", "cnl-ckc reviewer", body_html)
def build_guideline_page(model):
    gid = model.get("gid", "")
    counts = model.get("counts", {})
    docids = model.get("docids", [])
    doc_states = model.get("doc_states", {})
    region_by_docid = model.get("region_by_docid", {})
    title_by_docid = model.get("title_by_docid", {})
    region_rows = model.get("region_rows", [])
    joiner = "\n"
    empty_text = ""
    parts = []
    ledger_by_docid = model.get("ledger_by_docid", {})
    parts.append("<h1>" + esc_text(model.get("title", gid)) + "</h1>")
    parts.append("<p>" + esc_text(review_summary_text(counts, len(docids))) + " <a href=\"records.html\">All decision records</a></p>")
    parts.append("<section>")
    parts.append("<h2>Status</h2>")
    parts.append("<table class=\"compact\">")
    parts.append("<thead><tr><th>Status</th><th>Count</th></tr></thead>")
    status_rows = []
    status_labels = [["Passages", "regions"], ["With ACE", "ace"], ["Pending", "pending"], ["Approved", "approved"], ["Rejected", "rejected"], ["Contested", "contested"], ["Outdated", "stale"], ["Unreviewed", "unreviewed"]]
    for label_pair in status_labels:
        pair_copy = list(label_pair)
        label_text = pair_copy.pop(0)
        count_key = pair_copy.pop(0)
        status_rows.append("<tr><td>" + label_text + "</td><td>" + str(counts.get(count_key, 0)) + "</td></tr>")
    parts.append("<tbody>" + joiner.join(status_rows) + "</tbody>")
    parts.append("</table>")
    parts.append("</section>")
    parts.append("<section>")
    parts.append("<h2>Documents</h2>")
    parts.append("<table class=\"compact\">")
    parts.append("<thead><tr><th>Document</th><th>Status</th><th>Decisions</th><th>Passage</th></tr></thead>")
    doc_rows = []
    for docid in docids:
        state = doc_states.get(docid, "unreviewed")
        cells = []
        cells.append("<td><a href=\"doc/" + url_seg(docid) + ".html\">" + esc_text(title_by_docid.get(docid, docid)) + "</a></td>")
        cells.append("<td>" + chip_html(state) + "</td>")
        cells.append("<td>" + esc_text(tally_cell(doc_tally(ledger_by_docid, docid))) + "</td>")
        cells.append("<td>" + esc_text(region_by_docid.get(docid, "")) + "</td>")
        doc_rows.append("<tr>" + empty_text.join(cells) + "</tr>")
    parts.append("<tbody>" + joiner.join(doc_rows) + "</tbody>")
    parts.append("</table>")
    parts.append("</section>")
    parts.append("<section>")
    parts.append("<h2>Passages without ACE</h2>")
    parts.append("<table class=\"compact\">")
    parts.append("<thead><tr><th>Passage</th><th>Status</th><th>Section</th></tr></thead>")
    other_rows = []
    for region_row in region_rows:
        row_copy = list(region_row)
        region_id = row_copy.pop(0)
        status_text = row_copy.pop(0)
        section_text = row_copy.pop(0)
        cells = []
        cells.append("<td>" + esc_text(region_id) + "</td>")
        cells.append("<td>" + esc_text(status_text) + "</td>")
        cells.append("<td>" + esc_text(section_text) + "</td>")
        other_rows.append("<tr>" + empty_text.join(cells) + "</tr>")
    parts.append("<tbody>" + joiner.join(other_rows) + "</tbody>")
    parts.append("</table>")
    parts.append("</section>")
    body_html = joiner.join(parts)
    title_text = model.get("title", gid)
    crumb_html = "<a href=\"../../index.html\">guidelines</a> / " + esc_text(title_text)
    return page_html(title_text, crumb_html, body_html)
def doc_render_data(model, docid):
    gid = model.get("gid", "")
    guideline_path = model.get("path", None)
    ace_digest_by_docid = model.get("ace_digest_by_docid", {})
    payload_digest_by_docid = model.get("payload_digest_by_docid", {})
    payload_by_docid = model.get("payload_by_docid", {})
    ace_path = guideline_path.joinpath("ace", docid + ".ace")
    try:
        ace_bytes = ace_path.read_bytes()
    except OSError:
        return err("ui: viewmodel: " + gid + " doc " + docid + " missing ace file")
    ace_text = ""
    try:
        ace_text = ace_bytes.decode("utf-8")
    except UnicodeDecodeError:
        return err("ui: viewmodel: " + gid + " file not UTF-8: ace/" + docid + ".ace")
    control_detail = control_violation(ace_text, gid, "ace/" + docid + ".ace")
    if control_detail:
        return err(control_detail)
    ace_actual = sha256_hex(ace_bytes)
    if ace_actual != ace_digest_by_docid.get(docid, ""):
        return err("ui: digest mismatch: " + docid + " ace")
    payload_line = payload_by_docid.get(docid, "")
    payload_bytes = payload_line.encode("utf-8")
    payload_actual = sha256_hex(payload_bytes)
    if payload_actual != payload_digest_by_docid.get(docid, ""):
        return err("ui: digest mismatch: " + docid + " payload")
    pl_path = guideline_path.joinpath("pl", docid + ".pl")
    pl_result = load_text(pl_path, gid, "pl/" + docid + ".pl")
    if result_kind(pl_result) == "err":
        return pl_result
    pl_text = result_value(pl_result)
    align_path = guideline_path.joinpath("align", docid + ".tsv")
    align_data = None
    if align_path.is_file():
        align_result = load_text(align_path, gid, "align/" + docid + ".tsv")
        if result_kind(align_result) == "err":
            return align_result
        parse_result = hl_parse_align(result_value(align_result), payload_line, ace_text)
        if result_kind(parse_result) == "err":
            return err("ui: viewmodel: " + gid + " doc " + docid + " align: " + result_value(parse_result))
        align_data = result_value(parse_result)
    data = {}
    data.update({"ace_text": ace_text})
    data.update({"pl_text": pl_text})
    data.update({"pl_lines": len(pl_text.splitlines())})
    data.update({"align": align_data})
    return ok(data)
def build_doc_page(model, docid, doc_data, prev_id, next_id):
    gid = model.get("gid", "")
    doc_states = model.get("doc_states", {})
    state = doc_states.get(docid, "unreviewed")
    region_by_docid = model.get("region_by_docid", {})
    title_by_docid = model.get("title_by_docid", {})
    file_by_docid = model.get("file_by_docid", {})
    payload_by_docid = model.get("payload_by_docid", {})
    source_names = model.get("source_names", [])
    pdf_name = model.get("pdf_name", "")
    guideline_title = model.get("title", gid)
    doc_title = title_by_docid.get(docid, docid)
    joiner = "\n"
    parts = []
    head_html = esc_text(guideline_title)
    if pdf_name:
        head_html = head_html + " <a class=\"source\" href=\"../source/" + url_seg(pdf_name) + "\">PDF</a>"
    parts.append("<h1>" + head_html + "</h1>")
    parts.append("<h2>" + esc_text(doc_title) + " " + chip_html(state) + "</h2>")
    file_name = file_by_docid.get(docid, "")
    base_name = file_name.removeprefix("source/")
    published = base_name in source_names
    region_text = region_by_docid.get(docid, "")
    prov_parts = []
    if region_text:
        prov_parts.append(esc_text(region_text))
    if published:
        prov_parts.append("<a href=\"../source/" + url_seg(base_name) + "\">Source text</a>")
    if prov_parts:
        prov_joiner = " · "
        parts.append("<p>" + prov_joiner.join(prov_parts) + "</p>")
    ledger_by_docid = model.get("ledger_by_docid", {})
    records_href = "../records.html"
    has_history = docid in ledger_by_docid
    if has_history:
        records_href = records_href + "#" + url_seg(docid)
    tally = doc_tally(ledger_by_docid, docid)
    parts.append("<p>" + esc_text(tally_text(tally)) + " <a href=\"" + records_href + "\">All decision records</a></p>")
    if state == "stale":
        parts.append("<section class=\"stale\">")
        parts.append("<p>The document or its source changed after the last decision. No recorded decision applies to the version shown here.</p>")
        parts.append("</section>")
    link_data = hl_link_data(doc_data.get("ace_text", ""), payload_by_docid.get(docid, ""), doc_data.get("align", None))
    if link_data.get("matched", 0) > 0:
        parts.append("<p class=\"hl-note\">" + esc_text(hl_note_text) + "</p>")
    parts.append("<section>")
    parts.append("<h3>Original passage</h3>")
    parts.append("<pre class=\"prose\">" + link_data.get("payload_html", "") + "</pre>")
    parts.append("</section>")
    parts.append("<section>")
    parts.append("<h3>Attempto Controlled English (ACE)</h3>")
    parts.append("<pre class=\"prose\">" + link_data.get("ace_html", "") + "</pre>")
    parts.append("</section>")
    review_by_docid = model.get("review_by_docid", {})
    current_digest = review_by_docid.get(docid, "")
    ledger_file_digest = model.get("ledger_file_digest", "absent")
    token_text = serve_config.get("token", "")
    roster = reviewer_roster(model.get("ledger_order", []))
    roster_copy = list(roster)
    reviewer_names = roster_copy.pop(0)
    default_reviewer = roster_copy.pop(0)
    parts.append("<section class=\"verdict-entry\">")
    parts.append("<h3>Record a decision</h3>")
    parts.append("<p>Does the ACE representation appropriately reflect the original passage?</p>")
    parts.append("<form method=\"post\">")
    parts.append("<fieldset>")
    parts.append("<legend>Decision</legend>")
    parts.append("<label><input type=\"radio\" name=\"verdict\" value=\"approved\" required> Approved</label>")
    parts.append("<label><input type=\"radio\" name=\"verdict\" value=\"rejected\" required> Rejected</label>")
    parts.append("</fieldset>")
    parts.append("<label for=\"reviewer\">Reviewer name</label>")
    parts.append("<input type=\"text\" id=\"reviewer\" name=\"reviewer\" list=\"reviewer-names\" value=\"" + esc_attr(default_reviewer) + "\" required>")
    parts.append(datalist_html(reviewer_names))
    parts.append("<label for=\"comment\">Comment (optional)</label>")
    parts.append("<textarea id=\"comment\" name=\"comment\"></textarea>")
    parts.append("<input type=\"hidden\" name=\"review_sha256\" value=\"" + esc_attr(current_digest) + "\">")
    parts.append("<input type=\"hidden\" name=\"ledger_sha256\" value=\"" + esc_attr(ledger_file_digest) + "\">")
    parts.append("<input type=\"hidden\" name=\"csrf\" value=\"" + esc_attr(token_text) + "\">")
    parts.append("<button>Record decision</button>")
    parts.append("</form>")
    parts.append("</section>")
    parts.append("<section>")
    parts.append("<details>")
    parts.append("<summary>Compiled Prolog (" + str(doc_data.get("pl_lines", 0)) + " lines)</summary>")
    parts.append("<pre>" + esc_text(doc_data.get("pl_text", "")) + "</pre>")
    parts.append("</details>")
    parts.append("</section>")
    nav_parts = []
    if prev_id:
        nav_parts.append("<a href=\"" + url_seg(prev_id) + ".html\">Previous document</a>")
    nav_parts.append("<a href=\"../index.html\">Guideline index</a>")
    if next_id:
        nav_parts.append("<a href=\"" + url_seg(next_id) + ".html\">Next document</a>")
    nav_joiner = " · "
    parts.append("<nav class=\"docnav\">" + nav_joiner.join(nav_parts) + "</nav>")
    body_html = joiner.join(parts)
    crumb_html = "<a href=\"../../../index.html\">guidelines</a> / <a href=\"../index.html\">" + esc_text(guideline_title) + "</a> / " + esc_text(region_by_docid.get(docid, docid))
    return page_html(doc_title, crumb_html, body_html)
def build_records_page(model):
    gid = model.get("gid", "")
    docids = model.get("docids", [])
    counts = model.get("counts", {})
    title_by_docid = model.get("title_by_docid", {})
    ledger_by_docid = model.get("ledger_by_docid", {})
    joiner = "\n"
    empty_text = ""
    parts = []
    parts.append("<h1>Decision records</h1>")
    summary_text = review_summary_text(counts, len(docids))
    if counts.get("decisions", 0) > 0:
        summary_text = summary_text + " The newest decision for each document is first."
    parts.append("<p>" + esc_text(summary_text) + "</p>")
    section_count = 0
    link_count = 0
    for docid in docids:
        history = ledger_by_docid.get(docid, [])
        if history:
            section_count = section_count + 1
            newest_first = list(history)
            newest_first.reverse()
            parts.append("<section id=\"" + esc_attr(docid) + "\">")
            parts.append("<h2><a href=\"doc/" + url_seg(docid) + ".html\">" + esc_text(title_by_docid.get(docid, docid)) + "</a></h2>")
            parts.append("<table class=\"records\">")
            parts.append("<thead><tr><th>Decision</th><th>Reviewer</th><th>Date</th><th>Version</th><th>Comment</th></tr></thead>")
            row_parts = []
            for row in newest_first:
                row_copy = list(row)
                row_copy.pop(0)
                commit_text = row_copy.pop(0)
                verdict = row_copy.pop(0)
                reviewer_text = row_copy.pop(0)
                date_text = row_copy.pop(0)
                comment_text = row_copy.pop(0)
                current = row_copy.pop(0)
                version_text = "Earlier"
                if current:
                    version_text = "Current"
                if commit_text:
                    link_count = link_count + 1
                    version_text = "<a href=\"" + commit_url_base + esc_attr(commit_text) + "\">" + esc_text(version_text) + "</a>"
                shown_comment = comment_text
                if not shown_comment:
                    shown_comment = "Not given"
                cells = []
                cells.append("<td>" + esc_text(state_label(verdict)) + "</td>")
                cells.append("<td>" + esc_text(reviewer_text) + "</td>")
                cells.append("<td>" + esc_text(human_date(date_text)) + "</td>")
                cells.append("<td>" + version_text + "</td>")
                cells.append("<td>" + esc_text(shown_comment) + "</td>")
                row_parts.append("<tr>" + empty_text.join(cells) + "</tr>")
            parts.append("<tbody>" + joiner.join(row_parts) + "</tbody>")
            parts.append("</table>")
            parts.append("</section>")
    if section_count == 0:
        parts.append("<p>Open a document and record a decision to start this list.</p>")
    else:
        parts.append("<p>Each reviewer name is recorded as entered and is not verified.</p>")
        if link_count > 0:
            parts.append("<p>Each version links to the stored version of the text that the reviewer read.</p>")
    parts.append("<nav class=\"docnav\"><a href=\"index.html\">Guideline index</a></nav>")
    body_html = joiner.join(parts)
    crumb_html = "<a href=\"../../index.html\">guidelines</a> / <a href=\"index.html\">" + esc_text(model.get("title", gid)) + "</a> / records"
    return page_html("Decision records", crumb_html, body_html)
def build_error_page(title_text, heading_text, body_html):
    parts = []
    parts.append("<h1>" + esc_text(heading_text) + "</h1>")
    parts.append(body_html)
    joiner = "\n"
    return page_html(title_text, "cnl-ckc reviewer", joiner.join(parts))
def extract_hrefs(page_text):
    found = []
    parts = page_text.split("<a ")
    parts.pop(0)
    for part in parts:
        tag_parts = part.split(">", 1)
        tag_text = tag_parts.pop(0)
        attr_parts = tag_text.split("href=\"", 1)
        if len(attr_parts) == 2:
            attr_parts.pop(0)
            attr_tail = attr_parts.pop(0)
            value_parts = attr_tail.split("\"", 1)
            href_value = value_parts.pop(0)
            found.append(href_value)
    return found
def resolve_href(page_path, href_value):
    if href_value.startswith("#"):
        return "#"
    if href_value.startswith("https://"):
        return "#"
    fragment_parts = href_value.split("#", 1)
    href_value = fragment_parts.pop(0)
    base_segs = page_path.split("/")
    base_segs.pop()
    target_segs = href_value.split("/")
    failed = False
    for seg in target_segs:
        if seg == "..":
            if base_segs:
                base_segs.pop()
            else:
                failed = True
        elif seg != ".":
            base_segs.append(seg)
    if failed:
        return ""
    joiner = "/"
    return joiner.join(base_segs)
def render_pages(models):
    page_order = []
    pages = {}
    asset_paths = {}
    page_order.append("index.html")
    pages.update({"index.html": build_index_page(models)})
    meter_lines = []
    for model in models:
        gid = model.get("gid", "")
        docids = model.get("docids", [])
        counts = model.get("counts", {})
        guideline_path = model.get("path", None)
        for name_text in model.get("source_names", []):
            asset_rel = "g/" + url_seg(gid) + "/source/" + url_seg(name_text)
            asset_paths.update({asset_rel: guideline_path.joinpath("source", name_text)})
        gid_page = "g/" + url_seg(gid) + "/index.html"
        page_order.append(gid_page)
        pages.update({gid_page: build_guideline_page(model)})
        prev_map = {}
        next_map = {}
        prev_id = ""
        for docid in docids:
            if prev_id:
                prev_map.update({docid: prev_id})
                next_map.update({prev_id: docid})
            prev_id = docid
        for docid in docids:
            data_result = doc_render_data(model, docid)
            if result_kind(data_result) == "err":
                return data_result
            doc_data = result_value(data_result)
            doc_page = "g/" + url_seg(gid) + "/doc/" + url_seg(docid) + ".html"
            page_order.append(doc_page)
            pages.update({doc_page: build_doc_page(model, docid, doc_data, prev_map.get(docid, ""), next_map.get(docid, ""))})
        records_page = "g/" + url_seg(gid) + "/records.html"
        page_order.append(records_page)
        pages.update({records_page: build_records_page(model)})
        page_count = 2 + len(docids)
        meter_lines.append("ui: " + gid + " docs=" + str(len(docids)) + " regions=" + str(counts.get("regions", 0)) + " pages=" + str(page_count))
    for page_path in page_order:
        page_text = pages.get(page_path, "")
        for href_value in extract_hrefs(page_text):
            resolved = resolve_href(page_path, href_value)
            if resolved != "#":
                known = resolved in pages
                if not known:
                    known = resolved in asset_paths
                if not known:
                    return err("ui: viewmodel: dangling href " + page_path + " " + href_value)
    bundle = {}
    bundle.update({"page_order": page_order})
    bundle.update({"pages": pages})
    bundle.update({"asset_paths": asset_paths})
    bundle.update({"meter_lines": meter_lines})
    return ok(bundle)
def render_tree(corpus, out_path):
    model_result = build_viewmodel(corpus)
    if result_kind(model_result) == "err":
        return model_result
    models = result_value(model_result)
    bundle_result = render_pages(models)
    if result_kind(bundle_result) == "err":
        return bundle_result
    bundle = result_value(bundle_result)
    page_order = bundle.get("page_order", [])
    pages = bundle.get("pages", {})
    for page_path in page_order:
        full_path = out_path.joinpath(page_path)
        parent_dir = full_path.parent
        parent_dir.mkdir(parents=True, exist_ok=True)
        page_text = pages.get(page_path, "")
        page_bytes = page_text.encode("utf-8")
        full_path.write_bytes(page_bytes)
    asset_paths = bundle.get("asset_paths", {})
    asset_rels = sorted(asset_paths)
    for asset_rel in asset_rels:
        full_path = out_path.joinpath(asset_rel)
        parent_dir = full_path.parent
        parent_dir.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(asset_paths.get(asset_rel), full_path)
    result = {}
    result.update({"guidelines": len(models)})
    result.update({"pages_total": len(page_order)})
    result.update({"asset_rels": asset_rels})
    result.update({"meter_lines": bundle.get("meter_lines", [])})
    return ok(result)
def walk_files(base_path):
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
def strip_between(source_text, open_mark, close_mark):
    pieces = source_text.split(open_mark)
    kept = []
    first = True
    for piece in pieces:
        if first:
            kept.append(piece)
            first = False
        else:
            tail_parts = piece.split(close_mark)
            if len(tail_parts) > 1:
                tail_parts.pop(0)
                close_joiner = close_mark
                kept.append(close_joiner.join(tail_parts))
    empty_text = ""
    return empty_text.join(kept)
def visible_text(page_text):
    no_style = strip_between(page_text, "<style>", "</style>")
    no_pre = strip_between(no_style, "<pre", "</pre>")
    no_comments = strip_between(no_pre, "<!--", "-->")
    fragments = no_comments.split("<")
    texts = []
    first = True
    for fragment in fragments:
        if first:
            texts.append(fragment)
            first = False
        else:
            tag_parts = fragment.split(">")
            if len(tag_parts) > 1:
                tag_parts.pop(0)
                gt_joiner = ">"
                texts.append(gt_joiner.join(tag_parts))
    space_joiner = " "
    return html.unescape(space_joiner.join(texts))
def page_invariant_name(page_text):
    if page_text.count("<!doctype html>") != 1:
        return "doctype"
    if page_text.count("<html lang=\"en\">") != 1:
        return "html-lang"
    if page_text.count("<h1") != 1:
        return "h1"
    if page_text.count("<main id=\"main\">") != 1:
        return "main"
    if page_text.count("<nav") < 1:
        return "nav"
    if page_text.count("<a class=\"skip\" href=\"#main\">Skip to content</a>") != 1:
        return "skip"
    if page_text.count("<style>") != 1:
        return "style"
    if page_text.count("<script") != 0:
        return "script"
    if page_text.count("tabindex") != 0:
        return "tabindex"
    if page_text.count("style=") != 0:
        return "inline-style"
    external_total = page_text.count("href=\"http")
    commit_links = page_text.count("href=\"" + commit_url_base)
    if external_total != commit_links:
        return "external-href"
    scope_html = "<footer class=\"scope\"><p>" + esc_text(scope_line_text) + "</p></footer>"
    if page_text.count(scope_html) != 1:
        return "scope-line"
    page_visible = visible_text(page_text)
    visible_lower = page_visible.lower()
    if "sha256" in visible_lower:
        return "copy-sha256"
    if "bundle differs" in visible_lower:
        return "copy-bundle-differs"
    hex_runs = re.findall("[0-9a-fA-F]{16,}", page_visible)
    for hex_run in hex_runs:
        if re.search("[0-9]", hex_run):
            return "copy-hex"
    if re.search("\\b(ace|restates|uncovered)\\(", page_visible):
        return "copy-functor"
    short_runs = re.findall("\\b[0-9a-f]{7,15}\\b", page_visible)
    for short_run in short_runs:
        if re.search("[0-9]", short_run):
            if re.search("[a-f]", short_run):
                return "copy-short-hex"
    return ""
def selftest_violation():
    if url_seg("a b/%") != "a%20b%2F%25":
        return "ui: selftest failed: url_seg"
    if url_seg("/") != "%2F":
        return "ui: selftest failed: url_seg"
    if url_seg("漢") != "%E6%BC%A2":
        return "ui: selftest failed: url_seg"
    if esc_text("<&>\"'") != "&lt;&amp;&gt;\"'":
        return "ui: selftest failed: esc_text"
    if esc_text("plain") != "plain":
        return "ui: selftest failed: esc_text"
    if esc_attr("<&>\"'") != "&lt;&amp;&gt;&quot;&#x27;":
        return "ui: selftest failed: esc_attr"
    if esc_attr("plain") != "plain":
        return "ui: selftest failed: esc_attr"
    if hl_slice("abcdef", 2, 4) != "cd":
        return "ui: selftest failed: hl slice"
    if hl_int_field("7") != 7:
        return "ui: selftest failed: hl int field"
    bad_field = 0 - 1
    if hl_int_field("07") != bad_field:
        return "ui: selftest failed: hl int canonical"
    if hl_int_field("-1") != bad_field:
        return "ui: selftest failed: hl int negative"
    if hl_int_field("1x") != bad_field:
        return "ui: selftest failed: hl int junk"
    align_case = hl_parse_align("0\tace\t6\tclinician\n1\tace\t23\toffer\n2\tace\t29\tnaloxone\n0\tsrc\t0\tClinicians\n1\tsrc\t27\toffering\n2\tsrc\t36\tnaloxone\n", "Clinicians should consider offering naloxone.", "Every clinician should offer-naloxone.\n")
    if result_kind(align_case) != "ok":
        return "ui: selftest failed: hl parse ok"
    hl_case = hl_link_data("Every clinician should offer-naloxone.\n", "Clinicians should consider offering naloxone.", result_value(align_case))
    if hl_case.get("matched", 0) != 3:
        return "ui: selftest failed: hl match count"
    if hl_case.get("payload_html", "") != "<mark class=\"t0\">Clinicians</mark> should consider <mark class=\"t1\">offering</mark> <mark class=\"t2\">naloxone</mark>.":
        return "ui: selftest failed: hl payload html"
    if hl_case.get("ace_html", "") != "<span class=\"kw\">Every</span> <mark class=\"t0\">clinician</mark> <span class=\"kw\">should</span> <mark class=\"t1\">offer</mark>-<mark class=\"t2\">naloxone</mark>.\n":
        return "ui: selftest failed: hl ace html"
    align_case = hl_parse_align("2\tace\t31\trecommendation\n5\tace\t20\tcategory-B\n2\tsrc\t8\trecommendation\n5\tsrc\t24\tcategory: B\n", "It is a recommendation (category: B).", "Every patient has a category-B-recommendation.\n")
    hl_case = hl_link_data("Every patient has a category-B-recommendation.\n", "It is a recommendation (category: B).", result_value(align_case))
    if hl_case.get("matched", 0) != 2:
        return "ui: selftest failed: hl compound count"
    if hl_case.get("payload_html", "") != "It is a <mark class=\"t1\">recommendation</mark> (<mark class=\"t0\">category: B</mark>).":
        return "ui: selftest failed: hl compound payload"
    if hl_case.get("ace_html", "") != "<span class=\"kw\">Every</span> patient <span class=\"kw\">has</span> <span class=\"kw\">a</span> <mark class=\"t0\">category-B</mark>-<mark class=\"t1\">recommendation</mark>.\n":
        return "ui: selftest failed: hl compound ace"
    align_case = hl_parse_align("0\tace\t18\ttherapy\n0\tsrc\t16\ttherapy\n", "A therapy and a therapy < now.", "A patient needs a therapy.\n")
    hl_case = hl_link_data("A patient needs a therapy.\n", "A therapy and a therapy < now.", result_value(align_case))
    if hl_case.get("matched", 0) != 1:
        return "ui: selftest failed: hl occurrence count"
    if hl_case.get("payload_html", "") != "A therapy and a <mark class=\"t0\">therapy</mark> &lt; now.":
        return "ui: selftest failed: hl occurrence payload"
    hl_case = hl_link_data("No opioids today.\n", "Avoid opioids.", None)
    if hl_case.get("matched", 0) != 0:
        return "ui: selftest failed: hl missing align"
    if hl_case.get("payload_html", "") != "Avoid opioids.":
        return "ui: selftest failed: hl missing align payload"
    if hl_case.get("ace_html", "") != "<span class=\"kw\">No</span> opioids today.\n":
        return "ui: selftest failed: hl missing align ace"
    if result_value(hl_parse_align("0\tsrc\t0\tA", "A x", "B y\n")) != "missing trailing newline":
        return "ui: selftest failed: hl align newline"
    if result_value(hl_parse_align("\n", "A x", "B y\n")) != "empty file":
        return "ui: selftest failed: hl align empty"
    if result_value(hl_parse_align("0\tsrc\t0\n", "A x", "B y\n")) != "row 1: expected 4 tab-separated fields":
        return "ui: selftest failed: hl align fields"
    if result_value(hl_parse_align("01\tsrc\t0\tA\n", "A x", "B y\n")) != "row 1: group must be a canonical decimal":
        return "ui: selftest failed: hl align group"
    if result_value(hl_parse_align("0\tsrc\t00\tA\n", "A x", "B y\n")) != "row 1: start must be a canonical decimal":
        return "ui: selftest failed: hl align start"
    if result_value(hl_parse_align("0\tmid\t0\tA\n", "A x", "B y\n")) != "row 1: side must be src or ace":
        return "ui: selftest failed: hl align side"
    if result_value(hl_parse_align("0\tsrc\t0\t\n", "A x", "B y\n")) != "row 1: empty span":
        return "ui: selftest failed: hl align empty span"
    if result_value(hl_parse_align("0\tsrc\t9\tA\n", "A x", "B y\n")) != "row 1: span out of range":
        return "ui: selftest failed: hl align range"
    if result_value(hl_parse_align("0\tsrc\t0\tB\n", "A x", "B y\n")) != "row 1: span does not match the text at start":
        return "ui: selftest failed: hl align mismatch"
    if result_value(hl_parse_align("0\tsrc\t0\tA\n0\tsrc\t0\tA\n0\tace\t0\tB\n", "A x", "B y\n")) != "overlapping src spans":
        return "ui: selftest failed: hl align overlap"
    if result_value(hl_parse_align("0\tsrc\t0\tA\n", "A x", "B y\n")) != "every group needs both a src span and an ace span":
        return "ui: selftest failed: hl align one-sided"
    if "mark.t1, mark.t13, mark.t25, mark.t37 { background: #fef9c3; }" not in css_text:
        return "ui: selftest failed: hue group css"
    if "mark.t6, mark.t18, mark.t30, mark.t42 { background: #dcfce7; }" not in css_text:
        return "ui: selftest failed: hue extension css"
    if "main:has(mark.t13:hover) mark.t13 { background: #fef08a; text-decoration-style: solid; }" not in css_text:
        return "ui: selftest failed: hue hover cycle css"
    if "main:has(mark.t0:hover) mark.t0 { background: #bfdbfe; text-decoration-style: solid; }" not in css_text:
        return "ui: selftest failed: hue hover base css"
    if "mark.t0, mark.t12, mark.t24, mark.t36 { text-decoration-color: #2563eb; }" not in css_text:
        return "ui: selftest failed: hue underline css"
    if "mark.p" in css_text:
        return "ui: selftest failed: partial class retired"
    if "mark, mark[class] { background: none; text-decoration-color: #4b5563; }" not in css_text:
        return "ui: selftest failed: print mark strip"
    unique_values = []
    for role in sorted(palette):
        pair = palette.get(role, [])
        pair_copy = list(pair)
        fg_value = pair_copy.pop(0)
        bg_value = pair_copy.pop(0)
        fg_known = fg_value in unique_values
        if not fg_known:
            unique_values.append(fg_value)
        bg_known = bg_value in unique_values
        if not bg_known:
            unique_values.append(bg_value)
    allowed_total = 0
    for color_value in unique_values:
        allowed_total = allowed_total + css_text.count(color_value)
    if css_text.count("#") != allowed_total:
        return "ui: selftest failed: palette"
    case_head = "<h1>Selftest</h1>"
    clean_page = page_html("Selftest", "selftest", case_head + "<p>Clean page.</p>")
    if page_invariant_name(clean_page) != "":
        return "ui: selftest failed: invariant clean"
    scope_html = "<footer class=\"scope\"><p>" + esc_text(scope_line_text) + "</p></footer>"
    scope_missing = clean_page.replace(scope_html, "")
    if page_invariant_name(scope_missing) != "scope-line":
        return "ui: selftest failed: invariant scope-missing"
    scope_doubled = clean_page.replace(scope_html, scope_html + scope_html)
    if page_invariant_name(scope_doubled) != "scope-line":
        return "ui: selftest failed: invariant scope-duplicate"
    invariant_cases = []
    case_row = ["<p><code>SHA256</code></p>", "copy-sha256", "hex-label-case"]
    invariant_cases.append(case_row)
    case_row = ["<p>sha&#50;56</p>", "copy-sha256", "character-reference"]
    invariant_cases.append(case_row)
    case_row = ["<p>The bundle differs from the recorded one.</p>", "copy-bundle-differs", "bundle-phrase"]
    invariant_cases.append(case_row)
    case_row = ["<p><code>0123456789abcdef</code></p>", "copy-hex", "hex-run-16"]
    invariant_cases.append(case_row)
    case_row = ["<p><code>0123456789abcde</code></p>", "copy-short-hex", "hex-run-15"]
    invariant_cases.append(case_row)
    case_row = ["<p>abcdefabcdefabcdef</p>", "", "letters-only-run"]
    invariant_cases.append(case_row)
    case_row = ["<!-- ui: verdict: a > b sha256 bundle differs 0123456789abcdef --><p>Clean page.</p>", "", "comment-exempt"]
    invariant_cases.append(case_row)
    case_row = ["<input type=\"hidden\" name=\"review_sha256\" value=\"0123456789abcdef0123456789abcdef\">", "", "attribute-exempt"]
    invariant_cases.append(case_row)
    case_row = ["<pre>sha256 0123456789abcdef</pre>", "", "quoted-artifact-exempt"]
    invariant_cases.append(case_row)
    case_row = ["<p>sha<span>256</span></p>", "", "split-across-tags"]
    invariant_cases.append(case_row)
    case_row = ["<p>ace(a-2)</p>", "copy-functor", "functor-ace"]
    invariant_cases.append(case_row)
    case_row = ["<p>restates(b3-01)</p>", "copy-functor", "functor-restates"]
    invariant_cases.append(case_row)
    case_row = ["<p>uncovered(heading: x)</p>", "copy-functor", "functor-uncovered"]
    invariant_cases.append(case_row)
    case_row = ["<p>space(1) Restates b3-01</p>", "", "functor-embedded-word-exempt"]
    invariant_cases.append(case_row)
    case_row = ["<pre>restates(b3-01) 0123456789ab</pre>", "", "functor-pre-exempt"]
    invariant_cases.append(case_row)
    case_row = ["<p><code>0abcdef</code></p>", "copy-short-hex", "short-hex-7"]
    invariant_cases.append(case_row)
    case_row = ["<p>202211041022</p>", "", "short-hex-digits-only"]
    invariant_cases.append(case_row)
    case_row = ["<p>abcdefabcdef</p>", "", "short-hex-letters-only"]
    invariant_cases.append(case_row)
    for invariant_case in invariant_cases:
        case_copy = list(invariant_case)
        case_body = case_copy.pop(0)
        case_want = case_copy.pop(0)
        case_label = case_copy.pop(0)
        case_page = page_html("Selftest", "selftest", case_head + case_body)
        if page_invariant_name(case_page) != case_want:
            return "ui: selftest failed: invariant " + case_label
    return ""
h5_content_type = tuple(["Content-Type", "text/html; charset=utf-8"])
h5_csp = tuple(["Content-Security-Policy", "default-src 'none'; style-src 'unsafe-inline'"])
h5_nosniff = tuple(["X-Content-Type-Options", "nosniff"])
h5_referrer = tuple(["Referrer-Policy", "no-referrer"])
h5_cache = tuple(["Cache-Control", "no-store"])
def base_headers():
    return [h5_content_type, h5_csp, h5_nosniff, h5_referrer, h5_cache]
def asset_headers(media_type):
    content_type = tuple(["Content-Type", media_type])
    return [content_type, h5_csp, h5_nosniff, h5_referrer, h5_cache]
def not_found_response():
    body_html = "<p>The requested page does not exist.</p>"
    page_text = build_error_page("Not found", "Not found", body_html)
    body_bytes = page_text.encode("utf-8")
    return ["404 Not Found", base_headers(), body_bytes]
def doc_shaped(path_text):
    if not path_text.startswith("/g/"):
        return False
    rest_text = path_text.removeprefix("/g/")
    segs = rest_text.split("/")
    if len(segs) != 3:
        return False
    gid_seg = segs.pop(0)
    mid_seg = segs.pop(0)
    leaf_seg = segs.pop(0)
    if gid_seg == "":
        return False
    if mid_seg != "doc":
        return False
    if not leaf_seg.endswith(".html"):
        return False
    stem_text = leaf_seg.removesuffix(".html")
    if stem_text == "":
        return False
    return True
def method_response(shaped):
    allow_value = "GET"
    body_html = "<p>Only GET is supported on this page.</p>"
    if shaped:
        allow_value = "GET, POST"
        body_html = "<p>Only GET and POST are supported on this page.</p>"
    page_text = build_error_page("Method not allowed", "Method not allowed", body_html)
    body_bytes = page_text.encode("utf-8")
    headers = base_headers()
    headers.append(tuple(["Allow", allow_value]))
    return ["405 Method Not Allowed", headers, body_bytes]
def verdict_refusal(status_text, heading_text, detail, plain_text):
    body_html = "<p>" + esc_text(plain_text) + "</p>\n<!-- " + comment_safe(detail) + " -->"
    page_text = build_error_page(heading_text, heading_text, body_html)
    body_bytes = page_text.encode("utf-8")
    return [status_text, base_headers(), body_bytes]
def redirect_response(gid, docid):
    location_value = "/g/" + url_seg(gid) + "/doc/" + url_seg(docid) + ".html"
    body_html = "<p>The decision was recorded.</p>"
    page_text = build_error_page("Decision recorded", "Decision recorded", body_html)
    body_bytes = page_text.encode("utf-8")
    headers = base_headers()
    headers.append(tuple(["Location", location_value]))
    return ["303 See Other", headers, body_bytes]
def error_response(detail):
    body_html = "<p>The server could not complete the request. Reload the page and try again.</p>\n<!-- " + comment_safe(detail) + " -->"
    page_text = build_error_page("Server error", "Server error", body_html)
    body_bytes = page_text.encode("utf-8")
    return ["500 Internal Server Error", base_headers(), body_bytes]
def page_response(page_text):
    body_bytes = page_text.encode("utf-8")
    return ["200 OK", base_headers(), body_bytes]
def parse_form_fields(body_bytes):
    body_text = ""
    try:
        body_text = body_bytes.decode("utf-8")
    except UnicodeDecodeError:
        return err("ui: verdict: body not decodable")
    parsed = {}
    try:
        parsed = urllib.parse.parse_qs(body_text, keep_blank_values=True, strict_parsing=True, errors="strict", max_num_fields=32)
    except ValueError:
        return err("ui: verdict: body not parseable")
    except UnicodeDecodeError:
        return err("ui: verdict: body not parseable")
    for name in verdict_field_names:
        present = name in parsed
        if not present:
            return err("ui: verdict: missing field " + name)
    for name in verdict_field_names:
        value_list = parsed.get(name)
        if len(value_list) > 1:
            return err("ui: verdict: duplicate field " + name)
    for key_name in parsed:
        known = key_name in verdict_field_names
        if not known:
            return err("ui: verdict: unknown field " + key_name)
    fields = {}
    for name in verdict_field_names:
        value_list = list(parsed.get(name))
        fields.update({name: value_list.pop(0)})
    verdict_value = fields.get("verdict")
    verdict_ok = False
    if verdict_value == "approved":
        verdict_ok = True
    if verdict_value == "rejected":
        verdict_ok = True
    if not verdict_ok:
        return err("ui: verdict: invalid verdict")
    reviewer_value = fields.get("reviewer")
    if not reviewer_value:
        return err("ui: verdict: invalid reviewer")
    if not field_text_ok(reviewer_value):
        return err("ui: verdict: invalid reviewer")
    if not field_text_ok(fields.get("comment")):
        return err("ui: verdict: invalid comment")
    if not valid_hex64(fields.get("review_sha256")):
        return err("ui: verdict: invalid review_sha256")
    ledger_value = fields.get("ledger_sha256")
    ledger_ok = valid_hex64(ledger_value)
    if ledger_value == "absent":
        ledger_ok = True
    if not ledger_ok:
        return err("ui: verdict: invalid ledger_sha256")
    return ok(fields)
def discard_tmp(tmp_text):
    try:
        os.unlink(tmp_text)
    except OSError:
        discard_failed = True
def release_ledger_lock(lock_fd, audit_dir):
    try:
        os.unlink(str(audit_dir.joinpath(".adjudication.lock")))
    except OSError:
        unlink_failed = True
    try:
        os.close(lock_fd)
    except OSError:
        close_failed = True
def handle_verdict_post(model, gid, docid, meta):
    port_value = serve_config.get("port", 8377)
    expected_host = "127.0.0.1:" + str(port_value)
    origin_value = meta.get("origin", None)
    expected_origin = "http://" + expected_host
    if origin_value != None:
        if origin_value != expected_origin:
            return verdict_refusal("403 Forbidden", "Forbidden", "ui: verdict: origin not allowed", refusal_refused_text)
    if meta.get("content_type", "") != "application/x-www-form-urlencoded":
        return verdict_refusal("400 Bad Request", "Bad request", "ui: verdict: unsupported content type", refusal_invalid_form_text)
    body_bytes = meta.get("body", None)
    if body_bytes == None:
        stream = meta.get("stream", None)
        length_text = meta.get("length_text", "")
        length_ok = True
        length_num = 0
        try:
            length_num = int(length_text)
        except ValueError:
            length_ok = False
        if length_num < 0:
            length_ok = False
        if length_ok:
            if stream != None:
                read_ok = True
                read_bytes = bytes()
                try:
                    read_bytes = stream.read(length_num)
                except OSError:
                    read_ok = False
                if read_ok:
                    if len(read_bytes) == length_num:
                        body_bytes = read_bytes
    if body_bytes == None:
        return verdict_refusal("400 Bad Request", "Bad request", "ui: verdict: missing body", refusal_invalid_form_text)
    fields_result = parse_form_fields(body_bytes)
    if result_kind(fields_result) == "err":
        return verdict_refusal("400 Bad Request", "Bad request", result_value(fields_result), refusal_invalid_form_text)
    fields = result_value(fields_result)
    token_value = serve_config.get("token", "")
    token_ok = True
    if token_value == "":
        token_ok = False
    if fields.get("csrf") != token_value:
        token_ok = False
    if not token_ok:
        return verdict_refusal("403 Forbidden", "Forbidden", "ui: verdict: invalid csrf token", refusal_refused_text)
    data_result = doc_render_data(model, docid)
    if result_kind(data_result) == "err":
        return error_response(result_value(data_result))
    goal_py = goal_py_path()
    tool_dir = goal_py.parent
    repo_root = tool_dir.parent
    guideline_path = model.get("path", None)
    derive_command = [sys.executable, "-P", str(goal_py), "derive-review-manifest", str(guideline_path.resolve())]
    derive_ok = True
    derive_result = None
    try:
        derive_result = subprocess.run(derive_command, capture_output=True, cwd=str(repo_root))
    except OSError:
        derive_ok = False
    if not derive_ok:
        return error_response("ui: verdict: manifest derivation failed: validator launch failed")
    if derive_result.returncode != 0:
        relay = first_output_line(derive_result)
        return error_response("ui: verdict: manifest derivation failed: " + relay)
    manifest_bytes = derive_result.stdout
    manifest_text = ""
    try:
        manifest_text = manifest_bytes.decode("utf-8")
    except UnicodeDecodeError:
        return error_response("ui: verdict: manifest derivation failed: undecodable output")
    fresh_digest = ""
    for line_text in manifest_text.split("\n"):
        line_fields = line_text.split("\t")
        if len(line_fields) == 6:
            row_docid = line_fields.pop(0)
            if row_docid == docid:
                fresh_digest = line_fields.pop()
    if fresh_digest == "":
        return error_response("ui: verdict: manifest derivation failed: docid row missing")
    if fields.get("review_sha256") != fresh_digest:
        return verdict_refusal("409 Conflict", "Conflict", "ui: verdict: subject changed", refusal_subject_text)
    if fields.get("ledger_sha256") != model.get("ledger_file_digest", "absent"):
        return verdict_refusal("409 Conflict", "Conflict", "ui: verdict: ledger changed", refusal_ledger_text)
    real_guideline_path = model.get("real_path", None)
    audit_dir = real_guideline_path.joinpath("audit")
    ledger_path = audit_dir.joinpath("adjudication.tsv")
    if ledger_path.is_symlink():
        return error_response("ui: verdict: ledger not a regular file")
    if ledger_path.exists():
        if not ledger_path.is_file():
            return error_response("ui: verdict: ledger not a regular file")
    now_text = serve_config.get("now", "")
    if now_text == "":
        now_value = datetime.datetime.now(datetime.timezone.utc)
        now_text = now_value.strftime("%Y-%m-%dT%H:%M:%SZ")
    commit_text = serve_config.get("commit_pin", None)
    if commit_text == None:
        commit_text = ace_commit_hex(real_guideline_path, docid, model.get("corpus_commit", ""))
    tab_text = "\t"
    new_line = docid + tab_text + fields.get("review_sha256") + tab_text + commit_text + tab_text + fields.get("verdict") + tab_text + fields.get("reviewer") + tab_text + now_text + tab_text + fields.get("comment")
    new_key = docid + tab_text + now_text
    kept_lines = []
    insert_at = 0
    scan_index = 0
    for row in model.get("ledger_order", []):
        row_copy = list(row)
        row_docid = row_copy.pop(0)
        row_digest = row_copy.pop(0)
        row_commit = row_copy.pop(0)
        row_verdict = row_copy.pop(0)
        row_reviewer = row_copy.pop(0)
        row_date = row_copy.pop(0)
        row_comment = row_copy.pop(0)
        kept_lines.append(row_docid + tab_text + row_digest + tab_text + row_commit + tab_text + row_verdict + tab_text + row_reviewer + tab_text + row_date + tab_text + row_comment)
        scan_index = scan_index + 1
        row_key = row_docid + tab_text + row_date
        later = new_key < row_key
        if not later:
            insert_at = scan_index
    out_lines = []
    out_lines.append(ledger_header_text)
    emit_index = 0
    for kept_line in kept_lines:
        if emit_index == insert_at:
            out_lines.append(new_line)
        out_lines.append(kept_line)
        emit_index = emit_index + 1
    if insert_at == emit_index:
        out_lines.append(new_line)
    joiner = "\n"
    candidate_text = joiner.join(out_lines) + "\n"
    candidate_bytes = candidate_text.encode("utf-8")
    tmp_ok = True
    tmp_pair = None
    try:
        tmp_pair = tempfile.mkstemp(dir=str(audit_dir), prefix=".adjudication.tsv.")
    except OSError:
        tmp_ok = False
    if not tmp_ok:
        return error_response("ui: verdict: ledger write failed")
    pair_list = list(tmp_pair)
    tmp_fd = pair_list.pop(0)
    tmp_text = pair_list.pop(0)
    chmod_mode = 420
    write_ok = True
    written_count = 0
    try:
        written_count = os.write(tmp_fd, candidate_bytes)
        os.fsync(tmp_fd)
        os.close(tmp_fd)
        os.chmod(tmp_text, chmod_mode)
    except OSError:
        write_ok = False
    if written_count != len(candidate_bytes):
        write_ok = False
    if not write_ok:
        try:
            os.close(tmp_fd)
        except OSError:
            write_ok = False
        discard_tmp(tmp_text)
        return error_response("ui: verdict: ledger write failed")
    snapshot_ok = True
    snap_text = ""
    manifest_copy = None
    try:
        snap_text = tempfile.mkdtemp()
        snap_dir = pathlib.Path(snap_text)
        manifest_copy = snap_dir.joinpath("manifest.tsv")
        manifest_copy.write_bytes(manifest_bytes)
    except OSError:
        snapshot_ok = False
    if not snapshot_ok:
        if snap_text:
            shutil.rmtree(snap_text, ignore_errors=True)
        discard_tmp(tmp_text)
        return error_response("ui: adjudication ledger invalid: validator launch failed")
    validate_command = [sys.executable, "-P", str(goal_py), "ledger-validate", tmp_text, str(manifest_copy), "ui"]
    validate_ok = True
    validate_result = None
    try:
        validate_result = subprocess.run(validate_command, capture_output=True, cwd=str(repo_root))
    except OSError:
        validate_ok = False
    shutil.rmtree(snap_text, ignore_errors=True)
    if not validate_ok:
        discard_tmp(tmp_text)
        return error_response("ui: adjudication ledger invalid: validator launch failed")
    if validate_result.returncode != 0:
        relay = first_output_line(validate_result)
        discard_tmp(tmp_text)
        return error_response("ui: adjudication ledger invalid: " + relay)
    if serve_config.get("fault", "") == "after-tmp-write":
        raise SystemExit(3)
    lock_fd = -1
    try:
        lock_fd = os.open(str(audit_dir.joinpath(".adjudication.lock")), 66, 384)
        fcntl.flock(lock_fd, fcntl.LOCK_EX)
    except OSError:
        if lock_fd > -1:
            try:
                os.close(lock_fd)
            except OSError:
                close_failed = True
        lock_fd = -1
    if lock_fd == -1:
        discard_tmp(tmp_text)
        return error_response("ui: verdict: ledger write failed")
    current_digest = "absent"
    dest_ok = True
    if ledger_path.exists():
        dest_bytes = bytes()
        try:
            dest_bytes = ledger_path.read_bytes()
        except OSError:
            dest_ok = False
        if dest_ok:
            current_digest = sha256_hex(dest_bytes)
    race_hit = False
    if not dest_ok:
        race_hit = True
    if current_digest != model.get("ledger_file_digest", "absent"):
        race_hit = True
    if race_hit:
        release_ledger_lock(lock_fd, audit_dir)
        discard_tmp(tmp_text)
        return verdict_refusal("409 Conflict", "Conflict", "ui: verdict: ledger changed", refusal_ledger_text)
    replace_ok = True
    try:
        os.replace(tmp_text, str(ledger_path))
    except OSError:
        replace_ok = False
    release_ledger_lock(lock_fd, audit_dir)
    if not replace_ok:
        discard_tmp(tmp_text)
        return error_response("ui: verdict: ledger write failed")
    return redirect_response(gid, docid)
def respond(corpus, method_text, path_text, meta):
    port_value = serve_config.get("port", 8377)
    expected_host = "127.0.0.1:" + str(port_value)
    if meta.get("host", "") != expected_host:
        return verdict_refusal("403 Forbidden", "Forbidden", "ui: request: host not allowed", refusal_refused_text)
    shaped = doc_shaped(path_text)
    if method_text != "GET":
        if method_text != "POST":
            return method_response(shaped)
        if not shaped:
            return method_response(shaped)
    model_result = build_viewmodel(corpus)
    if result_kind(model_result) == "err":
        return error_response(result_value(model_result))
    models = result_value(model_result)
    if path_text == "/":
        return page_response(build_index_page(models))
    if path_text == "/index.html":
        return page_response(build_index_page(models))
    if not path_text.startswith("/g/"):
        return not_found_response()
    model_by_gid = {}
    for model in models:
        gid = model.get("gid", "")
        model_by_gid.update({gid: model})
    rest_text = path_text.removeprefix("/g/")
    segs = rest_text.split("/")
    seg_count = len(segs)
    if seg_count == 2:
        gid_seg = segs.pop(0)
        tail_seg = segs.pop(0)
        model = model_by_gid.get(gid_seg, None)
        if model == None:
            return not_found_response()
        if tail_seg == "":
            return page_response(build_guideline_page(model))
        if tail_seg == "index.html":
            return page_response(build_guideline_page(model))
        if tail_seg == "records.html":
            return page_response(build_records_page(model))
        return not_found_response()
    if seg_count == 3:
        gid_seg = segs.pop(0)
        mid_seg = segs.pop(0)
        leaf_seg = segs.pop(0)
        if mid_seg == "source":
            model = model_by_gid.get(gid_seg, None)
            if model == None:
                return not_found_response()
            source_names = model.get("source_names", [])
            published = leaf_seg in source_names
            if not published:
                return not_found_response()
            asset_path = model.get("path", None)
            asset_path = asset_path.joinpath("source", leaf_seg)
            asset_bytes = bytes()
            try:
                asset_bytes = asset_path.read_bytes()
            except OSError:
                return not_found_response()
            return ["200 OK", asset_headers(asset_media_type(leaf_seg)), asset_bytes]
        if mid_seg != "doc":
            return not_found_response()
        if not leaf_seg.endswith(".html"):
            return not_found_response()
        model = model_by_gid.get(gid_seg, None)
        if model == None:
            return not_found_response()
        docid = leaf_seg.removesuffix(".html")
        docids = model.get("docids", [])
        known = docid in docids
        if not known:
            return not_found_response()
        if method_text == "POST":
            return handle_verdict_post(model, gid_seg, docid, meta)
        data_result = doc_render_data(model, docid)
        if result_kind(data_result) == "err":
            return error_response(result_value(data_result))
        doc_data = result_value(data_result)
        prev_id = ""
        next_id = ""
        seen = False
        walker = ""
        for candidate in docids:
            if candidate == docid:
                prev_id = walker
                seen = True
            elif seen:
                if next_id == "":
                    next_id = candidate
            walker = candidate
        return page_response(build_doc_page(model, docid, doc_data, prev_id, next_id))
    return not_found_response()
def wsgi_app(environ, start_response):
    root_text = serve_config.get("root", ".")
    root_path = pathlib.Path(root_text)
    method_text = environ.get("REQUEST_METHOD", "")
    path_text = environ.get("PATH_INFO", "")
    meta = {}
    meta.update({"host": environ.get("HTTP_HOST", "")})
    meta.update({"origin": environ.get("HTTP_ORIGIN", None)})
    meta.update({"content_type": environ.get("CONTENT_TYPE", "")})
    meta.update({"length_text": environ.get("CONTENT_LENGTH", "")})
    meta.update({"stream": environ.get("wsgi.input", None)})
    corpus_result = resolve_corpus(root_path)
    triple = None
    if result_kind(corpus_result) == "err":
        triple = error_response(result_value(corpus_result))
    else:
        corpus = result_value(corpus_result)
        triple = respond(corpus, method_text, path_text, meta)
        release_corpus(corpus)
    triple_copy = list(triple)
    status_text = triple_copy.pop(0)
    headers = triple_copy.pop(0)
    body_bytes = triple_copy.pop(0)
    start_response(status_text, headers)
    return [body_bytes]
def serve_command(args):
    port_num = 8377
    root_text = "."
    if len(args) > 2:
        usage_fail()
    if args:
        port_text = args.pop(0)
        try:
            port_num = int(port_text)
        except ValueError:
            usage_fail()
        if port_num < 1024:
            usage_fail()
        if port_num > 65535:
            usage_fail()
    if args:
        root_text = args.pop(0)
    serve_config.update({"root": root_text})
    serve_config.update({"port": port_num})
    token_bytes = os.urandom(32)
    serve_config.update({"token": token_bytes.hex()})
    serve_config.update({"now": ""})
    serve_config.update({"fault": ""})
    server = wsgiref.simple_server.make_server("127.0.0.1", port_num, wsgi_app)
    out_line("ui: serving http://127.0.0.1:" + str(port_num) + "/")
    server.serve_forever()
def render_command(args):
    if not args:
        usage_fail()
    if len(args) > 2:
        usage_fail()
    outdir_text = args.pop(0)
    root_text = "."
    if args:
        root_text = args.pop(0)
    out_path = pathlib.Path(outdir_text)
    blocked = False
    if out_path.is_symlink():
        blocked = True
    elif out_path.is_dir():
        entries = list(out_path.iterdir())
        if entries:
            blocked = True
    elif out_path.exists():
        blocked = True
    if blocked:
        err_line("ui: render: destination not empty: " + outdir_text)
        raise SystemExit(2)
    root_path = pathlib.Path(root_text)
    corpus_result = resolve_corpus(root_path)
    if result_kind(corpus_result) == "err":
        err_line(result_value(corpus_result))
        raise SystemExit(1)
    corpus = result_value(corpus_result)
    result = render_tree(corpus, out_path)
    release_corpus(corpus)
    if result_kind(result) == "err":
        err_line(result_value(result))
        raise SystemExit(1)
    summary = result_value(result)
    for meter_line in summary.get("meter_lines", []):
        out_line(meter_line)
    out_line("ui: ok " + str(summary.get("guidelines", 0)) + " guidelines " + str(summary.get("pages_total", 0)) + " pages")
def check_ui_command(args):
    if len(args) > 1:
        usage_fail()
    root_text = "."
    if args:
        root_text = args.pop(0)
    selftest_detail = selftest_violation()
    if selftest_detail:
        err_line(selftest_detail)
        raise SystemExit(1)
    root_path = pathlib.Path(root_text)
    dir_one = tempfile.mkdtemp()
    dir_two = tempfile.mkdtemp()
    path_one = pathlib.Path(dir_one)
    path_two = pathlib.Path(dir_two)
    detail = ""
    summary = {}
    asset_rels = []
    corpus = worktree_corpus(root_path)
    result_one = render_tree(corpus, path_one)
    if result_kind(result_one) == "err":
        detail = result_value(result_one)
    else:
        summary = result_value(result_one)
        asset_rels = summary.get("asset_rels", [])
        result_two = render_tree(corpus, path_two)
        if result_kind(result_two) == "err":
            detail = result_value(result_two)
    if detail == "":
        rel_one = walk_files(path_one)
        rel_two = walk_files(path_two)
        if rel_one != rel_two:
            detail = "ui: render not byte-stable: tree"
    if detail == "":
        for rel_text in rel_one:
            if detail == "":
                file_one = path_one.joinpath(rel_text)
                file_two = path_two.joinpath(rel_text)
                bytes_one = file_one.read_bytes()
                bytes_two = file_two.read_bytes()
                if bytes_one != bytes_two:
                    detail = "ui: render not byte-stable: " + rel_text
                else:
                    copied = rel_text in asset_rels
                    if not copied:
                        page_text = bytes_one.decode("utf-8")
                        invariant_name = page_invariant_name(page_text)
                        if invariant_name:
                            detail = "ui: page invariant failed: " + rel_text + " " + invariant_name
    shutil.rmtree(dir_one, ignore_errors=True)
    shutil.rmtree(dir_two, ignore_errors=True)
    if detail:
        err_line(detail)
        raise SystemExit(1)
    for meter_line in summary.get("meter_lines", []):
        out_line(meter_line)
    out_line("ui: ok " + str(summary.get("guidelines", 0)) + " guidelines " + str(summary.get("pages_total", 0)) + " pages")
    out_line("ui: check ok")
def request_command(args):
    if len(args) < 2:
        usage_fail()
    method_text = args.pop(0)
    path_raw = args.pop(0)
    root_text = "."
    root_seen = False
    headers_map = {}
    body_value = None
    body_seen = False
    token_seen = False
    now_seen = False
    commit_seen = False
    fault_seen = False
    token_text = ""
    now_text = ""
    commit_text = ""
    fault_text = ""
    flag_names = ["--header", "--body", "--body-hex", "--token", "--now", "--commit", "--fault"]
    pending = ""
    flags_started = False
    for arg_text in args:
        if pending != "":
            if pending == "--header":
                colon_found = ":" in arg_text
                if not colon_found:
                    usage_fail()
                header_parts = arg_text.split(":", 1)
                header_name = header_parts.pop(0)
                header_value = header_parts.pop(0)
                if header_name == "":
                    usage_fail()
                headers_map.update({header_name.lower(): header_value})
            elif pending == "--body":
                if body_seen:
                    usage_fail()
                body_value = arg_text.encode("utf-8")
                body_seen = True
            elif pending == "--body-hex":
                if body_seen:
                    usage_fail()
                try:
                    body_value = bytes.fromhex(arg_text)
                except ValueError:
                    usage_fail()
                body_seen = True
            elif pending == "--token":
                if token_seen:
                    usage_fail()
                token_text = arg_text
                token_seen = True
            elif pending == "--now":
                if now_seen:
                    usage_fail()
                if not valid_post_date(arg_text):
                    usage_fail()
                now_text = arg_text
                now_seen = True
            elif pending == "--commit":
                if commit_seen:
                    usage_fail()
                if not commit_field_ok(arg_text):
                    usage_fail()
                commit_text = arg_text
                commit_seen = True
            else:
                if fault_seen:
                    usage_fail()
                if arg_text != "after-tmp-write":
                    usage_fail()
                fault_text = arg_text
                fault_seen = True
            pending = ""
        else:
            is_flag = arg_text in flag_names
            if is_flag:
                pending = arg_text
                flags_started = True
            elif arg_text.startswith("--"):
                usage_fail()
            else:
                if root_seen:
                    usage_fail()
                if flags_started:
                    usage_fail()
                root_text = arg_text
                root_seen = True
    if pending != "":
        usage_fail()
    host_text = headers_map.get("host", "127.0.0.1:8377")
    origin_value = headers_map.get("origin", None)
    ct_fallback = ""
    if body_seen:
        ct_fallback = "application/x-www-form-urlencoded"
    ct_text = headers_map.get("content-type", ct_fallback)
    serve_config.update({"port": 8377})
    serve_config.update({"token": token_text})
    serve_config.update({"now": now_text})
    if commit_seen:
        serve_config.update({"commit_pin": commit_text})
    serve_config.update({"fault": fault_text})
    meta = {}
    meta.update({"host": host_text})
    meta.update({"origin": origin_value})
    meta.update({"content_type": ct_text})
    meta.update({"body": body_value})
    query_parts = path_raw.split("?", 1)
    no_query = query_parts.pop(0)
    path_text = urllib.parse.unquote(no_query)
    root_path = pathlib.Path(root_text)
    corpus_result = resolve_corpus(root_path)
    triple = None
    if result_kind(corpus_result) == "err":
        triple = error_response(result_value(corpus_result))
    else:
        corpus = result_value(corpus_result)
        triple = respond(corpus, method_text, path_text, meta)
        release_corpus(corpus)
    triple_copy = list(triple)
    status_text = triple_copy.pop(0)
    headers = triple_copy.pop(0)
    body_bytes = triple_copy.pop(0)
    status_parts = status_text.split(" ", 1)
    status_code = status_parts.pop(0)
    out_line("HTTP " + status_code)
    for header_pair in headers:
        pair_copy = list(header_pair)
        header_name = pair_copy.pop(0)
        header_value = pair_copy.pop(0)
        out_line(header_name + ": " + header_value)
    out_line("")
    sys.stdout.buffer.write(body_bytes)
    sys.stdout.buffer.flush()
argv_list = list(sys.argv)
argv_list.pop(0)
if not argv_list:
    usage_fail()
command_name = argv_list.pop(0)
if command_name == "serve":
    serve_command(argv_list)
elif command_name == "render":
    render_command(argv_list)
elif command_name == "check":
    check_ui_command(argv_list)
elif command_name == "request":
    request_command(argv_list)
else:
    usage_fail()
