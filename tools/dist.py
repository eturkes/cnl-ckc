import datetime
import gzip
import hashlib
import io
import os
import pathlib
import subprocess
import sys
import tarfile
def refuse(detail):
    safe_detail = detail.replace("\n", "\\n")
    safe_detail = safe_detail.replace("\r", "\\r")
    print("dist: " + safe_detail, file=sys.stderr)
    raise SystemExit(1)
def usage_exit():
    print("dist: usage: python3 -P tools/dist.py build [<dest>]", file=sys.stderr)
    raise SystemExit(2)
def sha_hex(data):
    digest_object = hashlib.sha256(data)
    return digest_object.hexdigest()
def prefix_chars(text, count):
    kept = ""
    taken = 0
    for char_text in text:
        if taken < count:
            kept = kept + char_text
            taken = taken + 1
    return kept
def prefix_byte_list(data, count):
    kept = []
    for byte_value in data:
        if len(kept) < count:
            kept.append(byte_value)
    return bytes(kept)
def render_path(path_bytes):
    rendered = ""
    for byte_value in path_bytes:
        if byte_value == 92:
            rendered = rendered + "\\\\"
        elif byte_value == 10:
            rendered = rendered + "\\n"
        elif byte_value == 13:
            rendered = rendered + "\\r"
        elif byte_value == 9:
            rendered = rendered + "\\t"
        elif (byte_value >= 32) and (byte_value <= 126):
            rendered = rendered + chr(byte_value)
        else:
            rendered = rendered + "\\x" + format(byte_value, "02x")
    return rendered
def valid_rights_date(text):
    if len(text) != 10:
        return False
    digit_chars = set("0123456789")
    position = 0
    for char_text in text:
        if (position == 4) or (position == 7):
            if char_text != "-":
                return False
        else:
            if char_text not in digit_chars:
                return False
        position = position + 1
    try:
        datetime.date.fromisoformat(text)
    except ValueError:
        return False
    return True
def cell_control_clean(text):
    for char_text in text:
        code_point = ord(char_text)
        if code_point < 32:
            return False
        if code_point == 127:
            return False
    return True
def git_run(repo_dir, arg_list):
    command_list = ["git"]
    for arg_text in arg_list:
        command_list.append(arg_text)
    return subprocess.run(command_list, capture_output=True, cwd=repo_dir)
def git_bytes(repo_dir, arg_list):
    result = git_run(repo_dir, arg_list)
    if result.returncode != 0:
        empty_bytes = bytes([])
        return [False, empty_bytes]
    return [True, result.stdout]
def path_grammar_detail(path_bytes):
    path_text = ""
    decode_ok = True
    try:
        path_text = path_bytes.decode("utf-8")
    except UnicodeDecodeError:
        decode_ok = False
    if not decode_ok:
        return "member-path " + render_path(path_bytes)
    clean = True
    for char_text in path_text:
        code_point = ord(char_text)
        if code_point < 32:
            clean = False
        if code_point == 127:
            clean = False
        if char_text == "\\":
            clean = False
    if path_text.startswith("/"):
        clean = False
    segments = path_text.split("/")
    if ".." in segments:
        clean = False
    if "." in segments:
        clean = False
    if "" in segments:
        clean = False
    if not clean:
        return "member-path " + render_path(path_bytes)
    return ""
def corpus_entries(repo_dir):
    empty_list = []
    pair = git_bytes(repo_dir, ["ls-tree", "-r", "-z", "HEAD", "--", "guidelines"])
    ok_flag = pair.pop(0)
    out_bytes = pair.pop(0)
    if not ok_flag:
        return ["no-guidelines", empty_list]
    entries = []
    raw_entries = out_bytes.split(bytes([0]))
    for raw_entry in raw_entries:
        if raw_entry:
            meta_parts = list(raw_entry.partition(bytes([9])))
            meta_bytes = meta_parts.pop(0)
            tab_bytes = meta_parts.pop(0)
            path_bytes = meta_parts.pop(0)
            head_fields = meta_bytes.split(bytes([32]))
            mode_bytes = head_fields.pop(0)
            type_bytes = head_fields.pop(0)
            mode_text = mode_bytes.decode("ascii")
            type_text = type_bytes.decode("ascii")
            entries.append([mode_text, type_text, path_bytes])
    return ["", entries]
def stage_corpus(repo_dir):
    empty_map = {}
    pair = git_bytes(repo_dir, ["archive", "--format=tar", "HEAD", "guidelines"])
    ok_flag = pair.pop(0)
    out_bytes = pair.pop(0)
    if not ok_flag:
        return [False, empty_map]
    buffer = io.BytesIO(out_bytes)
    opened = tarfile.open(fileobj=buffer, mode="r:")
    staged = {}
    for member in opened.getmembers():
        if member.isfile():
            stream = opened.extractfile(member)
            data = stream.read()
            staged.update({member.name: data})
    opened.close()
    return [True, staged]
def validate_rights(gid, staged):
    empty_rows = []
    data = staged.get("guidelines/" + gid + "/rights.tsv", None)
    if data == None:
        return ["rights " + gid + " missing", "", "", empty_rows]
    text = ""
    decode_ok = True
    try:
        text = data.decode("utf-8")
    except UnicodeDecodeError:
        decode_ok = False
    if not decode_ok:
        return ["rights " + gid + " utf8", "", "", empty_rows]
    header_parts = list(text.partition("\n"))
    header_line = header_parts.pop(0)
    header_sep = header_parts.pop(0)
    body = header_parts.pop(0)
    if header_line != "profile\tstatement\turl\tretrieved\tnote":
        return ["rights " + gid + " header", "", "", empty_rows]
    if header_sep != "\n":
        return ["rights " + gid + " rows", "", "", empty_rows]
    if body != "":
        if not body.endswith("\n"):
            return ["rights " + gid + " rows", "", "", empty_rows]
    row_lines = body.split("\n")
    row_lines.pop()
    row_list = []
    row_number = 0
    profile_names = {"redistributable": True, "reconstructable": True, "restricted": True}
    for row_line in row_lines:
        row_number = row_number + 1
        row_label = str(row_number)
        fields = row_line.split("\t")
        if len(fields) != 5:
            return ["rights " + gid + " fields:" + row_label, "", "", empty_rows]
        profile_field = fields.pop(0)
        statement_field = fields.pop(0)
        url_field = fields.pop(0)
        retrieved_field = fields.pop(0)
        note_field = fields.pop(0)
        if profile_field not in profile_names:
            return ["rights " + gid + " profile:" + row_label, "", "", empty_rows]
        if statement_field == "":
            return ["rights " + gid + " statement:" + row_label, "", "", empty_rows]
        if url_field == "":
            return ["rights " + gid + " url:" + row_label, "", "", empty_rows]
        if not valid_rights_date(retrieved_field):
            return ["rights " + gid + " retrieved:" + row_label, "", "", empty_rows]
        cells_clean = True
        if not cell_control_clean(profile_field):
            cells_clean = False
        if not cell_control_clean(statement_field):
            cells_clean = False
        if not cell_control_clean(url_field):
            cells_clean = False
        if not cell_control_clean(retrieved_field):
            cells_clean = False
        if not cell_control_clean(note_field):
            cells_clean = False
        if not cells_clean:
            return ["rights " + gid + " control:" + row_label, "", "", empty_rows]
        row_list.append([profile_field, statement_field, url_field, retrieved_field, note_field])
    if len(row_list) == 0:
        return ["rights " + gid + " rows", "", "", empty_rows]
    rows_copy = list(row_list)
    first_row = rows_copy.pop(0)
    first_fields = list(first_row)
    operative_profile = first_fields.pop(0)
    first_fields.pop(0)
    operative_url = first_fields.pop(0)
    return ["", operative_profile, operative_url, row_list]
def review_docs(gid, staged):
    digest_map = {}
    data = staged.get("guidelines/" + gid + "/audit/review-manifest.tsv", None)
    if data == None:
        return digest_map
    text = ""
    decode_ok = True
    try:
        text = data.decode("utf-8")
    except UnicodeDecodeError:
        decode_ok = False
    if not decode_ok:
        return digest_map
    for line_text in text.split("\n"):
        if line_text:
            if not line_text.startswith("#"):
                fields = line_text.split("\t")
                if len(fields) >= 2:
                    docid = fields.pop(0)
                    review_digest = fields.pop()
                    digest_map.update({docid: review_digest})
    return digest_map
def ledger_classes(gid, staged, digest_map):
    seen_map = {}
    approved_map = {}
    rejected_map = {}
    data = staged.get("guidelines/" + gid + "/audit/adjudication.tsv", None)
    text = ""
    if data != None:
        try:
            text = data.decode("utf-8")
        except UnicodeDecodeError:
            text = ""
    for line_text in text.split("\n"):
        if line_text:
            if not line_text.startswith("#"):
                fields = line_text.split("\t")
                if len(fields) == 7:
                    docid = fields.pop(0)
                    digest_field = fields.pop(0)
                    commit_field = fields.pop(0)
                    verdict_field = fields.pop(0)
                    if docid in digest_map:
                        seen_map.update({docid: True})
                        if digest_field == digest_map.get(docid, ""):
                            if verdict_field == "approved":
                                approved_map.update({docid: True})
                            if verdict_field == "rejected":
                                rejected_map.update({docid: True})
    class_map = {}
    for docid in sorted(digest_map):
        doc_class = "unreviewed"
        if docid in seen_map:
            doc_class = "stale"
            has_approved = docid in approved_map
            has_rejected = docid in rejected_map
            if has_approved:
                doc_class = "approved"
                if has_rejected:
                    doc_class = "contested"
            else:
                if has_rejected:
                    doc_class = "rejected"
        class_map.update({docid: doc_class})
    return class_map
def schema_section(readme_text):
    anchor = "## Compiled Prolog schema"
    head_parts = list(readme_text.partition("\n" + anchor))
    head_parts.pop(0)
    anchor_sep = head_parts.pop(0)
    after_text = head_parts.pop(0)
    if anchor_sep == "":
        if not readme_text.startswith(anchor):
            return [False, ""]
        after_text = readme_text.removeprefix(anchor)
    tail_parts = list(after_text.partition("\n## "))
    section_body = tail_parts.pop(0)
    tail_sep = tail_parts.pop(0)
    if tail_sep == "":
        return [True, anchor + section_body]
    return [True, anchor + section_body + "\n"]
def rights_row_line(gid, row, open_mark, close_mark):
    fields = list(row)
    profile_field = fields.pop(0)
    statement_field = fields.pop(0)
    url_field = fields.pop(0)
    retrieved_field = fields.pop(0)
    note_field = fields.pop(0)
    line_text = "- " + gid + " (" + profile_field + "): \"" + statement_field + "\" "
    line_text = line_text + open_mark + url_field + ", retrieved " + retrieved_field + close_mark
    if note_field != "":
        line_text = line_text + " " + note_field
    return line_text + "\n"
def class_count_line(class_counts, class_name):
    count_value = class_counts.get(class_name, 0)
    return "- " + class_name + ": " + str(count_value) + " documents.\n"
def build_readme_dist(gid_list, profile_map, rights_map, doc_count_map, class_counts, schema_text):
    text = "# cnl-ckc knowledge base export\n\n"
    text = text + "This archive is a BagIt 1.0 bag. It holds the compiled clinical-guideline knowledge base from the cnl-ckc repository. The archive name and the `meta head` row in `release-manifest.tsv` give the source commit.\n\n"
    text = text + "## Verification\n\nRun `sha256sum -c manifest-sha256.txt tagmanifest-sha256.txt` from this directory. Each line must report OK.\n\n"
    text = text + "## Contents\n\n"
    for gid in gid_list:
        profile_text = profile_map.get(gid, "")
        doc_count = doc_count_map.get(gid, 0)
        count_text = str(doc_count)
        if profile_text == "restricted":
            text = text + "- " + gid + ": " + count_text + " documents held back (restricted rights).\n"
        elif profile_text == "reconstructable":
            text = text + "- " + gid + " (reconstructable): " + count_text + " documents. Source files are not included. Fetch each source URL in release-manifest.tsv and verify its digest.\n"
        else:
            text = text + "- " + gid + " (redistributable): " + count_text + " documents.\n"
    text = text + "\n## Review status\n\nEach shipped document has a label row in release-manifest.tsv.\n\n"
    text = text + class_count_line(class_counts, "approved")
    text = text + class_count_line(class_counts, "rejected")
    text = text + class_count_line(class_counts, "contested")
    text = text + class_count_line(class_counts, "stale")
    text = text + class_count_line(class_counts, "unreviewed")
    text = text + "\n## Rights\n\n"
    for gid in gid_list:
        row_list = rights_map.get(gid)
        for row in row_list:
            text = text + rights_row_line(gid, row, "Source: ", ".")
    text = text + "\n## Replay\n\nThese commands run in the source repository at the commit that this archive names.\n\n"
    text = text + "- compile: python3 -P tools/goal.py compile <guideline-id>\n"
    text = text + "- check: python3 -P tools/goal.py check\n"
    text = text + "- load: swipl -q -s data/guidelines/<guideline-id>/pl/<docid>.pl\n\n"
    return text + schema_text
def build_notice(notice_text, gid_list, profile_map, rights_map):
    text = notice_text
    if not text.endswith("\n"):
        text = text + "\n"
    text = text + "\nRights records for distributed sources:\n"
    for gid in gid_list:
        row_list = rights_map.get(gid)
        for row in row_list:
            text = text + rights_row_line(gid, row, "(", ")")
    text = text + "\nThe pl/ Prolog files in the payload are outputs that the vendored ACE compiler derived from the ace/ source documents.\n"
    return text
def head_input_bytes(repo_dir, spec, missing_detail, plan_out):
    pair = git_bytes(repo_dir, ["show", spec])
    ok_flag = pair.pop(0)
    out_bytes = pair.pop(0)
    if not ok_flag:
        return [missing_detail, out_bytes]
    return ["", out_bytes]
def derive_release(repo_dir):
    empty_plan = {}
    head_args = ["log", "-1", "--format=%H", "HEAD", "--", "guidelines", "vendor/ape/prolog/ace_to_pl.pl", "vendor/clex/clex_lexicon.pl"]
    pair = git_bytes(repo_dir, head_args)
    ok_flag = pair.pop(0)
    out_bytes = pair.pop(0)
    if not ok_flag:
        return ["no-input head", empty_plan]
    head_raw = out_bytes.decode("ascii", errors="replace")
    head_text = head_raw.strip()
    if head_text == "":
        return ["no-guidelines", empty_plan]
    pair = git_bytes(repo_dir, ["show", "-s", "--format=%ct", head_text])
    ok_flag = pair.pop(0)
    out_bytes = pair.pop(0)
    if not ok_flag:
        return ["no-input head", empty_plan]
    epoch_raw = out_bytes.decode("ascii", errors="replace")
    epoch_value = int(epoch_raw.strip())
    pair = corpus_entries(repo_dir)
    entries_detail = pair.pop(0)
    entries = pair.pop(0)
    if entries_detail != "":
        return [entries_detail, empty_plan]
    if len(entries) == 0:
        return ["no-guidelines", empty_plan]
    for entry in entries:
        entry_copy = list(entry)
        entry_copy.pop(0)
        entry_copy.pop(0)
        path_bytes = entry_copy.pop(0)
        grammar_detail = path_grammar_detail(path_bytes)
        if grammar_detail != "":
            return [grammar_detail, empty_plan]
    for entry in entries:
        entry_copy = list(entry)
        mode_text = entry_copy.pop(0)
        type_text = entry_copy.pop(0)
        path_bytes = entry_copy.pop(0)
        path_text = path_bytes.decode("utf-8")
        regular = False
        if type_text == "blob":
            if (mode_text == "100644") or (mode_text == "100755"):
                regular = True
        if not regular:
            return ["member-not-regular " + path_text, empty_plan]
    gid_map = {}
    for entry in entries:
        entry_copy = list(entry)
        entry_copy.pop(0)
        entry_copy.pop(0)
        path_bytes = entry_copy.pop(0)
        path_text = path_bytes.decode("utf-8")
        segments = path_text.split("/")
        segments.pop(0)
        if len(segments) == 1:
            return ["stray-root-member " + path_text, empty_plan]
        gid = segments.pop(0)
        gid_map.update({gid: True})
    if len(gid_map) == 0:
        return ["no-guidelines", empty_plan]
    pair = stage_corpus(repo_dir)
    ok_flag = pair.pop(0)
    staged = pair.pop(0)
    if not ok_flag:
        return ["no-input archive", empty_plan]
    pair = head_input_bytes(repo_dir, "HEAD:vendor/ape/prolog/ace_to_pl.pl", "no-input vendor/ape/prolog/ace_to_pl.pl", empty_plan)
    input_detail = pair.pop(0)
    compiler_bytes = pair.pop(0)
    if input_detail != "":
        return [input_detail, empty_plan]
    pair = head_input_bytes(repo_dir, "HEAD:vendor/clex/clex_lexicon.pl", "no-input vendor/clex/clex_lexicon.pl", empty_plan)
    input_detail = pair.pop(0)
    lexicon_bytes = pair.pop(0)
    if input_detail != "":
        return [input_detail, empty_plan]
    pair = head_input_bytes(repo_dir, "HEAD:README.md", "no-input README.md", empty_plan)
    input_detail = pair.pop(0)
    readme_bytes = pair.pop(0)
    if input_detail != "":
        return [input_detail, empty_plan]
    pair = head_input_bytes(repo_dir, "HEAD:NOTICE", "no-input NOTICE", empty_plan)
    input_detail = pair.pop(0)
    notice_bytes = pair.pop(0)
    if input_detail != "":
        return [input_detail, empty_plan]
    readme_text = ""
    notice_text = ""
    decode_ok = True
    try:
        readme_text = readme_bytes.decode("utf-8")
        notice_text = notice_bytes.decode("utf-8")
    except UnicodeDecodeError:
        decode_ok = False
    if not decode_ok:
        return ["no-input README.md", empty_plan]
    pair = schema_section(readme_text)
    schema_ok = pair.pop(0)
    schema_text = pair.pop(0)
    if not schema_ok:
        return ["no-input README.md schema section", empty_plan]
    gid_list = sorted(gid_map)
    profile_map = {}
    url_map = {}
    rights_map = {}
    for gid in gid_list:
        rights_result = validate_rights(gid, staged)
        rights_detail = rights_result.pop(0)
        operative_profile = rights_result.pop(0)
        operative_url = rights_result.pop(0)
        row_list = rights_result.pop(0)
        if rights_detail != "":
            return [rights_detail, empty_plan]
        profile_map.update({gid: operative_profile})
        url_map.update({gid: operative_url})
        rights_map.update({gid: row_list})
    doc_count_map = {}
    label_map = {}
    shipped_count = 0
    for gid in gid_list:
        digest_map = review_docs(gid, staged)
        doc_count_map.update({gid: len(digest_map)})
        profile_text = profile_map.get(gid)
        if profile_text != "restricted":
            shipped_count = shipped_count + 1
            class_map = ledger_classes(gid, staged, digest_map)
            for docid in sorted(class_map):
                label_map.update({docid: class_map.get(docid)})
    rejected_list = []
    contested_list = []
    class_counts = {}
    for docid in sorted(label_map):
        doc_class = label_map.get(docid)
        class_count = class_counts.get(doc_class, 0)
        class_counts.update({doc_class: class_count + 1})
        if doc_class == "rejected":
            rejected_list.append(docid)
        if doc_class == "contested":
            contested_list.append(docid)
    payload_map = {}
    source_rows = []
    for path_text in sorted(staged):
        segments = path_text.split("/")
        segments.pop(0)
        gid = segments.pop(0)
        profile_text = profile_map.get(gid, "")
        if profile_text != "restricted":
            data = staged.get(path_text)
            source_prefix = "guidelines/" + gid + "/source/"
            is_source = path_text.startswith(source_prefix)
            if (profile_text == "reconstructable") and is_source:
                url_text = url_map.get(gid)
                source_rows.append(["data/" + path_text, sha_hex(data), str(len(data)), url_text])
            else:
                payload_map.update({"data/" + path_text: data})
    readme_dist_text = build_readme_dist(gid_list, profile_map, rights_map, doc_count_map, class_counts, schema_text)
    notice_dist_text = build_notice(notice_text, gid_list, profile_map, rights_map)
    bagit_text = "BagIt-Version: 1.0\nTag-File-Character-Encoding: UTF-8\n"
    tags_map = {}
    tags_map.update({"bagit.txt": bagit_text.encode("utf-8")})
    tags_map.update({"README-dist.md": readme_dist_text.encode("utf-8")})
    tags_map.update({"NOTICE": notice_dist_text.encode("utf-8")})
    member_map = {}
    for path_text in payload_map:
        member_map.update({path_text: payload_map.get(path_text)})
    for path_text in tags_map:
        member_map.update({path_text: tags_map.get(path_text)})
    manifest_text = "meta\tschema\tv1\n"
    manifest_text = manifest_text + "meta\thead\t" + head_text + "\n"
    manifest_text = manifest_text + "meta\tcompiler\t" + sha_hex(compiler_bytes) + "\n"
    manifest_text = manifest_text + "meta\tbase-lexicon\t" + sha_hex(lexicon_bytes) + "\n"
    manifest_text = manifest_text + "meta\tpython\t3.11\n"
    manifest_text = manifest_text + "meta\tswipl\t9.2.9\n"
    manifest_text = manifest_text + "meta\tverify\tsha256sum -c manifest-sha256.txt tagmanifest-sha256.txt\n"
    manifest_text = manifest_text + "meta\treplay\tcompile: python3 -P tools/goal.py compile <guideline-id>\n"
    manifest_text = manifest_text + "meta\treplay\tcheck: python3 -P tools/goal.py check\n"
    manifest_text = manifest_text + "meta\treplay\tload: swipl -q -s data/guidelines/<guideline-id>/pl/<docid>.pl\n"
    manifest_text = manifest_text + "meta\tgenerated\trelease-manifest.tsv\n"
    manifest_text = manifest_text + "meta\tgenerated\tmanifest-sha256.txt\n"
    manifest_text = manifest_text + "meta\tgenerated\ttagmanifest-sha256.txt\n"
    for path_text in sorted(member_map):
        data = member_map.get(path_text)
        size_text = str(len(data))
        manifest_text = manifest_text + "member\t" + path_text + "\t" + sha_hex(data) + "\t" + size_text + "\n"
    for source_row in source_rows:
        row_copy = list(source_row)
        source_path = row_copy.pop(0)
        source_sha = row_copy.pop(0)
        source_size = row_copy.pop(0)
        source_url = row_copy.pop(0)
        manifest_text = manifest_text + "source\t" + source_path + "\t" + source_sha + "\t" + source_size + "\t" + source_url + "\n"
    for docid in sorted(label_map):
        manifest_text = manifest_text + "label\t" + docid + "\t" + label_map.get(docid) + "\n"
    plan = {}
    plan.update({"manifest": manifest_text})
    plan.update({"head": head_text})
    plan.update({"epoch": epoch_value})
    plan.update({"payload": payload_map})
    plan.update({"tags": tags_map})
    plan.update({"rejected": rejected_list})
    plan.update({"contested": contested_list})
    plan.update({"shipped": shipped_count})
    return ["", plan]
def manifest_row_map(manifest_text, kind_text):
    row_map = {}
    for line_text in manifest_text.split("\n"):
        if line_text:
            fields = line_text.split("\t")
            kind_field = fields.pop(0)
            if kind_field == kind_text:
                if len(fields) >= 1:
                    key_field = fields.pop(0)
                    row_map.update({key_field: line_text})
    return row_map
def drift_path(committed_text, derived_text):
    for kind_text in ["member", "source", "label", "meta"]:
        committed_map = manifest_row_map(committed_text, kind_text)
        derived_map = manifest_row_map(derived_text, kind_text)
        union_map = {}
        for key_text in committed_map:
            union_map.update({key_text: True})
        for key_text in derived_map:
            union_map.update({key_text: True})
        for key_text in sorted(union_map):
            if committed_map.get(key_text, "") != derived_map.get(key_text, ""):
                return key_text
    return "release-manifest.tsv"
def octal_field(value, width):
    digits = format(value, "o")
    digit_width = width - 1
    padded = digits.rjust(digit_width, "0")
    return padded + "\0"
def tar_header(name_bytes, size_value, mtime_value, type_text):
    name_pad = 100 - len(name_bytes)
    zero_byte = bytes([0])
    name_field = name_bytes + (zero_byte * name_pad)
    front_text = octal_field(420, 8)
    front_text = front_text + octal_field(0, 8)
    front_text = front_text + octal_field(0, 8)
    front_text = front_text + octal_field(size_value, 12)
    front_text = front_text + octal_field(mtime_value, 12)
    part_a = name_field + front_text.encode("ascii")
    back_text = type_text
    back_text = back_text + ("\0" * 100)
    back_text = back_text + "ustar\0" + "00"
    back_text = back_text + ("\0" * 64)
    back_text = back_text + octal_field(0, 8)
    back_text = back_text + octal_field(0, 8)
    back_text = back_text + ("\0" * 167)
    part_b = back_text.encode("ascii")
    space_text = "        "
    space_field = space_text.encode("ascii")
    checksum_value = sum(part_a + space_field + part_b)
    checksum_text = format(checksum_value, "06o") + "\0" + " "
    return part_a + checksum_text.encode("ascii") + part_b
def block_pad(chunk_len):
    rem = chunk_len % 512
    if rem == 0:
        return bytes([])
    pad_count = 512 - rem
    return bytes([0]) * pad_count
def tar_member_bytes(name_text, data, mtime_value):
    name_bytes = name_text.encode("utf-8")
    chunks = []
    if len(name_bytes) > 100:
        longlink_text = "././@LongLink"
        longlink_bytes = longlink_text.encode("ascii")
        link_data = name_bytes + bytes([0])
        chunks.append(tar_header(longlink_bytes, len(link_data), 0, "L"))
        chunks.append(link_data)
        chunks.append(block_pad(len(link_data)))
        header_name = prefix_byte_list(name_bytes, 100)
    else:
        header_name = name_bytes
    chunks.append(tar_header(header_name, len(data), mtime_value, "0"))
    chunks.append(data)
    chunks.append(block_pad(len(data)))
    empty_bytes = bytes([])
    return empty_bytes.join(chunks)
def build_archive(files_map, mtime_value):
    chunks = []
    for name_text in sorted(files_map):
        chunks.append(tar_member_bytes(name_text, files_map.get(name_text), mtime_value))
    chunks.append(bytes([0]) * 1024)
    empty_bytes = bytes([])
    tar_bytes = empty_bytes.join(chunks)
    rem = len(tar_bytes) % 10240
    if rem != 0:
        pad_count = 10240 - rem
        tar_bytes = tar_bytes + (bytes([0]) * pad_count)
    buffer = io.BytesIO()
    gz_file = gzip.GzipFile(fileobj=buffer, mode="wb", compresslevel=9, mtime=0)
    gz_file.write(tar_bytes)
    gz_file.close()
    return buffer.getvalue()
def digest_lines(path_list, data_map):
    text = ""
    for path_text in sorted(path_list):
        data = data_map.get(path_text)
        text = text + sha_hex(data) + "  " + path_text + "\n"
    return text
def publish_archive(dest_arg, archive_name, raw):
    dest_path = pathlib.Path(dest_arg)
    if dest_path.is_symlink():
        return "dest destination is a symlink: " + dest_arg
    if dest_path.exists():
        if not dest_path.is_dir():
            return "dest destination is not a directory: " + dest_arg
    else:
        dest_path.mkdir(parents=True)
    archive_path = dest_path.joinpath(archive_name)
    sidecar_name = archive_name + ".sha256"
    sidecar_path = dest_path.joinpath(sidecar_name)
    if archive_path.is_symlink():
        return "dest archive path is a symlink: " + archive_name
    if archive_path.exists():
        if not archive_path.is_file():
            return "dest archive path is not a regular file: " + archive_name
        existing_bytes = archive_path.read_bytes()
        if existing_bytes != raw:
            return "output-collision " + archive_name
    if sidecar_path.is_symlink():
        return "dest sidecar path is a symlink: " + sidecar_name
    if sidecar_path.exists():
        if not sidecar_path.is_file():
            return "dest sidecar path is not a regular file: " + sidecar_name
    pid_text = str(os.getpid())
    sidecar_text = sha_hex(raw) + "  " + archive_name + "\n"
    archive_temp = dest_path.joinpath(archive_name + ".tmp." + pid_text)
    archive_temp.write_bytes(raw)
    os.replace(archive_temp, archive_path)
    sidecar_temp = dest_path.joinpath(sidecar_name + ".tmp." + pid_text)
    sidecar_temp.write_bytes(sidecar_text.encode("utf-8"))
    os.replace(sidecar_temp, sidecar_path)
    return ""
def build_command(dest_arg):
    pair = derive_release(".")
    detail = pair.pop(0)
    plan = pair.pop(0)
    if detail != "":
        refuse(detail)
    pair = git_bytes(".", ["show", "HEAD:release-manifest.tsv"])
    ok_flag = pair.pop(0)
    committed_bytes = pair.pop(0)
    if not ok_flag:
        refuse("manifest-drift release-manifest.tsv")
    committed_text = ""
    decode_ok = True
    try:
        committed_text = committed_bytes.decode("utf-8")
    except UnicodeDecodeError:
        decode_ok = False
    if not decode_ok:
        refuse("manifest-drift release-manifest.tsv")
    derived_manifest = plan.get("manifest")
    if committed_text != derived_manifest:
        refuse("manifest-drift " + drift_path(committed_text, derived_manifest))
    rejected_list = list(plan.get("rejected"))
    if rejected_list:
        refuse("rejected-verdict " + rejected_list.pop(0))
    contested_list = list(plan.get("contested"))
    if contested_list:
        refuse("contested-verdict " + contested_list.pop(0))
    head_text = plan.get("head")
    bag_root = "cnl-ckc-kb-g" + prefix_chars(head_text, 12)
    archive_name = bag_root + ".tar.gz"
    rel_map = {}
    payload_map = plan.get("payload")
    for path_text in payload_map:
        rel_map.update({path_text: payload_map.get(path_text)})
    tags_map = plan.get("tags")
    for path_text in tags_map:
        rel_map.update({path_text: tags_map.get(path_text)})
    rel_map.update({"release-manifest.tsv": derived_manifest.encode("utf-8")})
    payload_paths = sorted(payload_map)
    payload_manifest = digest_lines(payload_paths, rel_map)
    rel_map.update({"manifest-sha256.txt": payload_manifest.encode("utf-8")})
    tag_paths = ["NOTICE", "README-dist.md", "bagit.txt", "manifest-sha256.txt", "release-manifest.tsv"]
    tag_manifest = digest_lines(tag_paths, rel_map)
    rel_map.update({"tagmanifest-sha256.txt": tag_manifest.encode("utf-8")})
    files_map = {}
    for path_text in rel_map:
        files_map.update({bag_root + "/" + path_text: rel_map.get(path_text)})
    raw = build_archive(files_map, plan.get("epoch"))
    publish_detail = publish_archive(dest_arg, archive_name, raw)
    if publish_detail != "":
        refuse(publish_detail)
    shipped_text = str(plan.get("shipped"))
    member_text = str(len(files_map))
    byte_text = str(len(raw))
    print("dist: ok " + shipped_text + " guidelines " + member_text + " members " + byte_text + " bytes")
    print("dist: sha256=" + sha_hex(raw) + " " + archive_name)
argv = list(sys.argv)
argv.pop(0)
if len(argv) == 0:
    usage_exit()
mode_text = argv.pop(0)
if mode_text != "build":
    usage_exit()
if len(argv) > 1:
    usage_exit()
dest_arg = "dist"
if len(argv) == 1:
    dest_arg = argv.pop(0)
build_command(dest_arg)
