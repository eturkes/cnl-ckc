import hashlib
import html
import os
import pathlib
import re
import shutil
import subprocess
import sys
import tempfile
import urllib.parse
import wsgiref.simple_server
usage_text = "ui: usage: expected: ui serve [<port>] [<root>] | ui render <outdir> [<root>] | ui check [<root>] | ui request <method> <path> [<root>]"
census_rx = re.compile("identify the ([0-9]+) payloads below")
chip_states = ["approved", "rejected", "stale", "unreviewed"]
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
    value_bytes = value.encode("utf-8")
    if len(value_bytes) > 250:
        return False
    return subset
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
        census_text = census_hit.group(1)
        census_count = int(census_text)
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
def build_doc_states(guideline_path, gid, docids, review_by_docid, manifest_text):
    ledger_path = guideline_path.joinpath("audit", "adjudication.tsv")
    states = {}
    for docid in docids:
        states.update({docid: "unreviewed"})
    ledger_digests = {}
    if not ledger_path.is_file():
        return ok([states, ledger_digests])
    try:
        ledger_bytes = ledger_path.read_bytes()
    except OSError:
        return err("ui: viewmodel: " + gid + " missing audit/adjudication.tsv")
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
    rows_result = parse_tsv_rows(ledger_text, 6, gid, "adjudication")
    if result_kind(rows_result) == "err":
        return rows_result
    for row_pair in result_value(rows_result):
        pair_copy = list(row_pair)
        fields = pair_copy.pop(0)
        docid = fields.pop(0)
        row_digest = fields.pop(0)
        verdict = fields.pop(0)
        current_digest = review_by_docid.get(docid, "")
        ledger_digests.update({docid: row_digest})
        if row_digest == current_digest:
            states.update({docid: verdict})
        else:
            states.update({docid: "stale"})
    return ok([states, ledger_digests])
def build_guideline_model(root_path, gid):
    guideline_path = root_path.joinpath("guidelines", gid)
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
                return err("ui: viewmodel: " + gid + " duplicate docid " + docid)
            docids.append(docid)
            region_by_docid.update({docid: region_id})
            section_by_docid.update({docid: section_text})
            page_by_docid.update({docid: page_text})
            file_by_docid.update({docid: file_text})
            file_docid_ordinal.update({docid: ordinal})
            ace_counter = ace_counter + 1
        else:
            region_rows.append([region_id, status_text, section_text])
            if status_text.startswith("restates("):
                restates_counter = restates_counter + 1
            else:
                if status_text.startswith("uncovered("):
                    uncovered_counter = uncovered_counter + 1
                else:
                    pending_counter = pending_counter + 1
    sorted_docids = sorted(docids)
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
    manifest_rows_result = parse_tsv_rows(manifest_text, 7, gid, "review-manifest")
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
        manifest_docids.append(docid)
        ace_digest_by_docid.update({docid: ace_digest})
        payload_digest_by_docid.update({docid: payload_digest})
        notes_digest = fields.pop(0)
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
    notes_path = guideline_path.joinpath("audit", "projection-notes.tsv")
    if not notes_path.is_file():
        return err("ui: viewmodel: " + gid + " missing audit/projection-notes.tsv")
    notes_result = load_text(notes_path, gid, "audit/projection-notes.tsv")
    if result_kind(notes_result) == "err":
        return notes_result
    notes_rows_result = parse_tsv_rows(result_value(notes_result), 4, gid, "projection-notes")
    if result_kind(notes_rows_result) == "err":
        return notes_rows_result
    kept_by_docid = {}
    dropped_by_docid = {}
    notes_region_by_docid = {}
    notes_docids = []
    for row_pair in result_value(notes_rows_result):
        pair_copy = list(row_pair)
        fields = pair_copy.pop(0)
        docid = fields.pop(0)
        region_id = fields.pop(0)
        kept_text = fields.pop(0)
        dropped_text = fields.pop(0)
        notes_docids.append(docid)
        notes_region_by_docid.update({docid: region_id})
        kept_by_docid.update({docid: kept_text})
        dropped_by_docid.update({docid: dropped_text})
    for docid in sorted_docids:
        present = notes_region_by_docid.get(docid, "")
        if not present:
            return err("ui: viewmodel: " + gid + " doc " + docid + " missing projection-notes row")
    for docid in sorted(notes_docids):
        claimed = region_by_docid.get(docid, "")
        if not claimed:
            return err("ui: viewmodel: " + gid + " orphan projection-notes doc " + docid)
    for docid in sorted_docids:
        notes_region = notes_region_by_docid.get(docid, "")
        coverage_region = region_by_docid.get(docid, "")
        if notes_region != coverage_region:
            return err("ui: viewmodel: " + gid + " doc " + docid + " notes region " + notes_region + " differs from coverage " + coverage_region)
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
            if census_count != payload_total:
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
    ledger_digests = states_copy.pop(0)
    model = {}
    model.update({"gid": gid})
    model.update({"path": guideline_path})
    title_result = readme_title(guideline_path, gid)
    if result_kind(title_result) == "err":
        return title_result
    model.update({"title": result_value(title_result)})
    model.update({"docids": sorted_docids})
    model.update({"region_by_docid": region_by_docid})
    model.update({"section_by_docid": section_by_docid})
    model.update({"page_by_docid": page_by_docid})
    model.update({"file_by_docid": file_by_docid})
    model.update({"payload_by_docid": payload_by_docid})
    model.update({"kept_by_docid": kept_by_docid})
    model.update({"dropped_by_docid": dropped_by_docid})
    model.update({"ace_digest_by_docid": ace_digest_by_docid})
    model.update({"payload_digest_by_docid": payload_digest_by_docid})
    model.update({"review_by_docid": review_by_docid})
    model.update({"doc_states": doc_states})
    model.update({"ledger_digests": ledger_digests})
    model.update({"region_rows": region_rows})
    counts = {}
    counts.update({"regions": len(coverage_rows)})
    counts.update({"ace": ace_counter})
    counts.update({"restates": restates_counter})
    counts.update({"uncovered": uncovered_counter})
    counts.update({"pending": pending_counter})
    approved_count = 0
    rejected_count = 0
    stale_count = 0
    unreviewed_count = 0
    for docid in sorted_docids:
        state = doc_states.get(docid, "unreviewed")
        if state == "approved":
            approved_count = approved_count + 1
        else:
            if state == "rejected":
                rejected_count = rejected_count + 1
            else:
                if state == "stale":
                    stale_count = stale_count + 1
                else:
                    unreviewed_count = unreviewed_count + 1
    counts.update({"approved": approved_count})
    counts.update({"rejected": rejected_count})
    counts.update({"stale": stale_count})
    counts.update({"unreviewed": unreviewed_count})
    model.update({"counts": counts})
    return ok(model)
def build_viewmodel(root_path):
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
        model_result = build_guideline_model(root_path, gid)
        if result_kind(model_result) == "err":
            return model_result
        models.append(result_value(model_result))
    return ok(models)
palette = {"body": ["#111827", "#ffffff"], "link": ["#1d4ed8", "#ffffff"], "chip-approved": ["#14532d", "#dcfce7"], "chip-rejected": ["#7f1d1d", "#fee2e2"], "chip-stale": ["#78350f", "#fef3c7"], "chip-unreviewed": ["#1f2937", "#e5e7eb"]}
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
    lines.append("h1 { font-size: 1.6rem; }")
    lines.append("table { border-collapse: collapse; width: 100%; margin: 1rem 0; }")
    lines.append("th, td { text-align: left; padding: 0.4rem 0.6rem; border-bottom: 1px solid " + line_color + "; vertical-align: top; }")
    lines.append("th { border-bottom: 2px solid " + body_fg + "; }")
    lines.append(".chip { display: inline-block; padding: 0.1rem 0.6rem; border-radius: 999px; font-size: 0.85rem; font-weight: 600; }")
    lines.append(".chip-approved { color: " + pal_fg("chip-approved") + "; background: " + pal_bg("chip-approved") + "; }")
    lines.append(".chip-rejected { color: " + pal_fg("chip-rejected") + "; background: " + pal_bg("chip-rejected") + "; }")
    lines.append(".chip-stale { color: " + pal_fg("chip-stale") + "; background: " + pal_bg("chip-stale") + "; }")
    lines.append(".chip-unreviewed { color: " + pal_fg("chip-unreviewed") + "; background: " + pal_bg("chip-unreviewed") + "; }")
    lines.append("pre { padding: 0.75rem 1rem; border: 1px solid " + line_color + "; overflow-x: auto; }")
    lines.append("pre, code { font-family: ui-monospace, Menlo, Consolas, monospace; font-size: 0.95rem; }")
    lines.append("pre.src { font-family: Georgia, serif; font-size: 1.05rem; white-space: pre-wrap; overflow-wrap: anywhere; }")
    lines.append("dt { font-weight: 600; margin-top: 0.6rem; }")
    lines.append("dd { margin-left: 0; }")
    lines.append("summary { cursor: pointer; }")
    lines.append("section { margin: 1.5rem 0; }")
    lines.append("nav.docnav { padding: 1rem 0; border-top: 1px solid " + line_color + "; }")
    joiner = "\n"
    return joiner.join(lines)
css_text = build_css()
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
    parts.append("</body>")
    parts.append("</html>")
    joiner = "\n"
    return joiner.join(parts) + "\n"
def chip_html(state):
    return "<span class=\"chip chip-" + state + "\">" + state + "</span>"
def build_index_page(models):
    rows = []
    for model in models:
        gid = model.get("gid", "")
        counts = model.get("counts", {})
        docids = model.get("docids", [])
        cells = []
        cells.append("<td><a href=\"g/" + url_seg(gid) + "/index.html\">" + esc_text(gid) + "</a></td>")
        cells.append("<td>" + esc_text(model.get("title", gid)) + "</td>")
        cells.append("<td>" + str(len(docids)) + "</td>")
        cells.append("<td>" + str(counts.get("regions", 0)) + "</td>")
        cells.append("<td>" + str(counts.get("approved", 0)) + "</td>")
        cells.append("<td>" + str(counts.get("rejected", 0)) + "</td>")
        cells.append("<td>" + str(counts.get("stale", 0)) + "</td>")
        cells.append("<td>" + str(counts.get("unreviewed", 0)) + "</td>")
        empty_text = ""
        rows.append("<tr>" + empty_text.join(cells) + "</tr>")
    parts = []
    parts.append("<h1>Guidelines</h1>")
    parts.append("<section>")
    parts.append("<table>")
    parts.append("<thead><tr><th>guideline</th><th>title</th><th>docs</th><th>regions</th><th>approved</th><th>rejected</th><th>stale</th><th>unreviewed</th></tr></thead>")
    joiner = "\n"
    parts.append("<tbody>" + joiner.join(rows) + "</tbody>")
    parts.append("</table>")
    parts.append("</section>")
    body_html = joiner.join(parts)
    return page_html("guidelines", "cnl-ckc reviewer", body_html)
def build_guideline_page(model):
    gid = model.get("gid", "")
    counts = model.get("counts", {})
    docids = model.get("docids", [])
    doc_states = model.get("doc_states", {})
    region_by_docid = model.get("region_by_docid", {})
    section_by_docid = model.get("section_by_docid", {})
    region_rows = model.get("region_rows", [])
    joiner = "\n"
    empty_text = ""
    parts = []
    parts.append("<h1>" + esc_text(model.get("title", gid)) + "</h1>")
    parts.append("<section>")
    parts.append("<h2>Meters</h2>")
    meter_one = "regions=" + str(counts.get("regions", 0)) + " ace=" + str(counts.get("ace", 0)) + " restates=" + str(counts.get("restates", 0)) + " uncovered=" + str(counts.get("uncovered", 0)) + " pending=" + str(counts.get("pending", 0))
    meter_two = "approved=" + str(counts.get("approved", 0)) + " rejected=" + str(counts.get("rejected", 0)) + " stale=" + str(counts.get("stale", 0)) + " unreviewed=" + str(counts.get("unreviewed", 0))
    parts.append("<p>" + meter_one + "</p>")
    parts.append("<p>" + meter_two + "</p>")
    parts.append("</section>")
    parts.append("<section>")
    parts.append("<h2>Documents</h2>")
    parts.append("<table>")
    parts.append("<thead><tr><th>document</th><th>status</th><th>region</th><th>section</th></tr></thead>")
    doc_rows = []
    for docid in docids:
        state = doc_states.get(docid, "unreviewed")
        cells = []
        cells.append("<td><a href=\"doc/" + url_seg(docid) + ".html\">" + esc_text(docid) + "</a></td>")
        cells.append("<td>" + chip_html(state) + "</td>")
        cells.append("<td>" + esc_text(region_by_docid.get(docid, "")) + "</td>")
        cells.append("<td>" + esc_text(section_by_docid.get(docid, "")) + "</td>")
        doc_rows.append("<tr>" + empty_text.join(cells) + "</tr>")
    parts.append("<tbody>" + joiner.join(doc_rows) + "</tbody>")
    parts.append("</table>")
    parts.append("</section>")
    parts.append("<section>")
    parts.append("<h2>Regions without ACE</h2>")
    parts.append("<table>")
    parts.append("<thead><tr><th>region</th><th>status</th><th>section</th></tr></thead>")
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
    crumb_html = "<a href=\"../../index.html\">guidelines</a> / " + esc_text(gid)
    return page_html(gid, crumb_html, body_html)
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
    data = {}
    data.update({"ace_text": ace_text})
    data.update({"pl_text": pl_text})
    data.update({"pl_lines": len(pl_text.splitlines())})
    return ok(data)
def build_doc_page(model, docid, doc_data, prev_id, next_id):
    gid = model.get("gid", "")
    doc_states = model.get("doc_states", {})
    state = doc_states.get(docid, "unreviewed")
    region_by_docid = model.get("region_by_docid", {})
    section_by_docid = model.get("section_by_docid", {})
    page_by_docid = model.get("page_by_docid", {})
    file_by_docid = model.get("file_by_docid", {})
    payload_by_docid = model.get("payload_by_docid", {})
    kept_by_docid = model.get("kept_by_docid", {})
    dropped_by_docid = model.get("dropped_by_docid", {})
    review_by_docid = model.get("review_by_docid", {})
    ledger_digests = model.get("ledger_digests", {})
    joiner = "\n"
    parts = []
    parts.append("<h1>" + esc_text(docid) + " " + chip_html(state) + "</h1>")
    if state == "stale":
        parts.append("<section class=\"stale\">")
        parts.append("<h2>Adjudication stale</h2>")
        parts.append("<p>bundle differs</p>")
        parts.append("<dl>")
        parts.append("<dt>pinned review_sha256</dt><dd><code>" + esc_text(ledger_digests.get(docid, "")) + "</code></dd>")
        parts.append("<dt>current review_sha256</dt><dd><code>" + esc_text(review_by_docid.get(docid, "")) + "</code></dd>")
        parts.append("</dl>")
        parts.append("</section>")
    parts.append("<section>")
    parts.append("<h2>Source region " + esc_text(region_by_docid.get(docid, "")) + "</h2>")
    parts.append("<p>" + esc_text(section_by_docid.get(docid, "")) + " · " + esc_text(file_by_docid.get(docid, "")) + " · " + esc_text(page_by_docid.get(docid, "")) + "</p>")
    parts.append("<pre class=\"src\">" + esc_text(payload_by_docid.get(docid, "")) + "</pre>")
    parts.append("</section>")
    parts.append("<section>")
    parts.append("<h2>ACE</h2>")
    parts.append("<pre>" + esc_text(doc_data.get("ace_text", "")) + "</pre>")
    parts.append("</section>")
    parts.append("<section>")
    parts.append("<h2>Projection notes</h2>")
    parts.append("<dl>")
    parts.append("<dt>kept</dt><dd>" + esc_text(kept_by_docid.get(docid, "")) + "</dd>")
    parts.append("<dt>dropped</dt><dd>" + esc_text(dropped_by_docid.get(docid, "")) + "</dd>")
    parts.append("</dl>")
    parts.append("</section>")
    parts.append("<section>")
    parts.append("<h2>Compiled Prolog</h2>")
    parts.append("<details>")
    parts.append("<summary>compiled Prolog (" + str(doc_data.get("pl_lines", 0)) + " lines)</summary>")
    parts.append("<pre>" + esc_text(doc_data.get("pl_text", "")) + "</pre>")
    parts.append("</details>")
    parts.append("</section>")
    nav_parts = []
    if prev_id:
        nav_parts.append("<a href=\"" + url_seg(prev_id) + ".html\">prev: " + esc_text(prev_id) + "</a>")
    nav_parts.append("<a href=\"../index.html\">up: " + esc_text(gid) + "</a>")
    if next_id:
        nav_parts.append("<a href=\"" + url_seg(next_id) + ".html\">next: " + esc_text(next_id) + "</a>")
    nav_joiner = " · "
    parts.append("<nav class=\"docnav\">" + nav_joiner.join(nav_parts) + "</nav>")
    body_html = joiner.join(parts)
    crumb_html = "<a href=\"../../../index.html\">guidelines</a> / <a href=\"../index.html\">" + esc_text(gid) + "</a> / " + esc_text(docid)
    return page_html(docid, crumb_html, body_html)
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
    page_order.append("index.html")
    pages.update({"index.html": build_index_page(models)})
    meter_lines = []
    for model in models:
        gid = model.get("gid", "")
        docids = model.get("docids", [])
        counts = model.get("counts", {})
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
        page_count = 1 + len(docids)
        meter_lines.append("ui: " + gid + " docs=" + str(len(docids)) + " regions=" + str(counts.get("regions", 0)) + " pages=" + str(page_count))
    for page_path in page_order:
        page_text = pages.get(page_path, "")
        for href_value in extract_hrefs(page_text):
            resolved = resolve_href(page_path, href_value)
            if resolved != "#":
                known = resolved in pages
                if not known:
                    return err("ui: viewmodel: dangling href " + page_path + " " + href_value)
    bundle = {}
    bundle.update({"page_order": page_order})
    bundle.update({"pages": pages})
    bundle.update({"meter_lines": meter_lines})
    return ok(bundle)
def render_tree(root_path, out_path):
    model_result = build_viewmodel(root_path)
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
    result = {}
    result.update({"guidelines": len(models)})
    result.update({"pages_total": len(page_order)})
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
    if page_text.count("href=\"http") != 0:
        return "external-href"
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
    return ""
h5_content_type = tuple(["Content-Type", "text/html; charset=utf-8"])
h5_csp = tuple(["Content-Security-Policy", "default-src 'none'; style-src 'unsafe-inline'"])
h5_nosniff = tuple(["X-Content-Type-Options", "nosniff"])
h5_referrer = tuple(["Referrer-Policy", "no-referrer"])
h5_cache = tuple(["Cache-Control", "no-store"])
def base_headers():
    return [h5_content_type, h5_csp, h5_nosniff, h5_referrer, h5_cache]
def not_found_response():
    body_html = "<p>The requested page does not exist.</p>"
    page_text = build_error_page("not found", "Not found", body_html)
    body_bytes = page_text.encode("utf-8")
    return ["404 Not Found", base_headers(), body_bytes]
def method_response():
    body_html = "<p>Only GET is supported.</p>"
    page_text = build_error_page("method not allowed", "Method not allowed", body_html)
    body_bytes = page_text.encode("utf-8")
    headers = base_headers()
    headers.append(tuple(["Allow", "GET"]))
    return ["405 Method Not Allowed", headers, body_bytes]
def error_response(detail):
    body_html = "<pre>" + esc_text(detail) + "</pre>"
    page_text = build_error_page("server error", "Server error", body_html)
    body_bytes = page_text.encode("utf-8")
    return ["500 Internal Server Error", base_headers(), body_bytes]
def page_response(page_text):
    body_bytes = page_text.encode("utf-8")
    return ["200 OK", base_headers(), body_bytes]
def respond(root_path, method_text, path_text):
    if method_text != "GET":
        return method_response()
    model_result = build_viewmodel(root_path)
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
        return not_found_response()
    if seg_count == 3:
        gid_seg = segs.pop(0)
        mid_seg = segs.pop(0)
        leaf_seg = segs.pop(0)
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
    triple = respond(root_path, method_text, path_text)
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
    result = render_tree(root_path, out_path)
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
    result_one = render_tree(root_path, path_one)
    if result_kind(result_one) == "err":
        detail = result_value(result_one)
    else:
        summary = result_value(result_one)
        result_two = render_tree(root_path, path_two)
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
    if len(args) > 3:
        usage_fail()
    method_text = args.pop(0)
    path_raw = args.pop(0)
    root_text = "."
    if args:
        root_text = args.pop(0)
    query_parts = path_raw.split("?", 1)
    no_query = query_parts.pop(0)
    path_text = urllib.parse.unquote(no_query)
    root_path = pathlib.Path(root_text)
    triple = respond(root_path, method_text, path_text)
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
