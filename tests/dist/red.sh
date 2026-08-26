#!/usr/bin/env bash
set -u
if [ "$#" -gt 1 ]; then
    printf '%s\n' 'usage: tests/dist/red.sh [<repo-root>]' >&2
    exit 2
fi
self_dir=$(CDPATH='' cd -- "$(dirname -- "$0")" && pwd -P) || exit 2
root=${1:-$(git -C "$self_dir" rev-parse --show-toplevel)}
export PYTHONDONTWRITEBYTECODE=1
export LC_ALL=C.UTF-8
export TZ=UTC
exec python3 -P - "$root" "$self_dir/cases.tsv" <<'PY'
import csv
import gzip
import hashlib
import io
import os
import pathlib
import re
import shutil
import struct
import subprocess
import sys
import tarfile
import tempfile
import zlib

TARGET = pathlib.Path(sys.argv[1]).resolve()
CASES_PATH = pathlib.Path(sys.argv[2]).resolve()
PYTHON = "python3"
DIST = TARGET / "tools" / "dist.py"
GOAL = TARGET / "tools" / "goal.py"
RIGHTS_HEADER = "profile\tstatement\turl\tretrieved\tnote\n"
REVIEW_HEADER_1 = "# format: docid<TAB>ace_sha256<TAB>coverage_row_sha256<TAB>region_payload_sha256<TAB>semantic_clause_sha256<TAB>review_sha256\n"
REVIEW_HEADER_2 = "# bundle v2; review_sha256 = sha256 of the labeled component-digest block; regenerate: python3 -P tools/goal.py review-manifest <id>; do not edit.\n"
LEDGER_HEADER = "# format: docid<TAB>review_sha256<TAB>ace_commit<TAB>verdict<TAB>reviewer<TAB>date<TAB>comment\n"
SCHEMA_SECTION = "## Compiled Prolog schema (v1)\n\nFixture schema bytes stay verbatim.\n\n### Load\n\nRun `swipl -q -s data/guidelines/g-red/pl/doc-a.pl`.\n\n"
ROOT_README = "# Fixture KB\n\nFixture repository.\n\n" + SCHEMA_SECTION + "## Operating\n\nRun the checks.\n\n## Licensing\n\nFixture licensing.\n"
ROOT_NOTICE = "Fixture KB notice.\nFirst-party fixture text.\n"
GIT_ENV = {
    "GIT_CONFIG_GLOBAL": "/dev/null",
    "GIT_CONFIG_SYSTEM": "/dev/null",
    "GIT_DEFAULT_HASH": "sha1",
    "GIT_AUTHOR_NAME": "fixture",
    "GIT_AUTHOR_EMAIL": "fixture@localhost",
    "GIT_AUTHOR_DATE": "2026-01-01T00:00:00+00:00",
    "GIT_COMMITTER_NAME": "fixture",
    "GIT_COMMITTER_EMAIL": "fixture@localhost",
    "GIT_COMMITTER_DATE": "2026-01-01T00:00:00+00:00",
    "LC_ALL": "C.UTF-8",
    "TZ": "UTC",
}


def digest(data):
    return hashlib.sha256(data).hexdigest()


def command(argv, cwd, env=None, timeout=180):
    merged = os.environ.copy()
    if env:
        merged.update(env)
    try:
        done = subprocess.run(argv, cwd=str(cwd), env=merged, stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=timeout)
        return Result(done.returncode, done.stdout, done.stderr)
    except subprocess.TimeoutExpired as exc:
        stdout = exc.stdout or b""
        stderr = (exc.stderr or b"") + b"harness: timeout\n"
        return Result(124, stdout, stderr)
    except OSError as exc:
        return Result(127, b"", ("harness: exec " + str(exc) + "\n").encode("utf-8", errors="replace"))


class Result:
    def __init__(self, rc, stdout=b"", stderr=b"", issues=None):
        self.rc = rc
        self.stdout = stdout
        self.stderr = stderr
        self.issues = list(issues or [])

    def issue(self, text):
        self.issues.append(text)
        return self


class SetupFailure(Exception):
    def __init__(self, result, phase):
        super().__init__(phase)
        self.result = result
        self.phase = phase


class Repo:
    def __init__(self, path):
        self.path = pathlib.Path(path)
        self.path.mkdir(parents=True)
        init = self.git("init", "-q", "-b", "main")
        if init.rc != 0:
            raise RuntimeError(init.stderr.decode("utf-8", errors="replace"))

    def env(self, extra=None):
        env = dict(GIT_ENV)
        if extra:
            env.update(extra)
        return env

    def git(self, *args):
        return command(["git", *args], self.path, self.env())

    def write(self, rel, data, mode=None):
        target = self.path / rel
        target.parent.mkdir(parents=True, exist_ok=True)
        if isinstance(data, str):
            data = data.encode("utf-8")
        target.write_bytes(data)
        if mode is not None:
            target.chmod(mode)
        return target

    def remove(self, rel):
        target = self.path / rel
        if target.is_dir() and not target.is_symlink():
            shutil.rmtree(target)
        else:
            target.unlink()

    def commit(self, message):
        added = self.git("add", "-A")
        if added.rc != 0:
            raise RuntimeError(added.stderr.decode("utf-8", errors="replace"))
        made = self.git("commit", "-q", "-m", message)
        if made.rc != 0:
            raise RuntimeError(made.stderr.decode("utf-8", errors="replace"))
        return self.head()

    def commit_index(self, message):
        made = self.git("commit", "-q", "-m", message)
        if made.rc != 0:
            raise RuntimeError(made.stderr.decode("utf-8", errors="replace"))
        return self.head()

    def head(self):
        got = self.git("rev-parse", "HEAD")
        if got.rc != 0:
            raise RuntimeError(got.stderr.decode("utf-8", errors="replace"))
        return got.stdout.decode("ascii").strip()

    def input_head(self):
        got = self.git("log", "-1", "--format=%H", "HEAD", "--", "guidelines", "vendor/ape/prolog/ace_to_pl.pl", "vendor/clex/clex_lexicon.pl")
        if got.rc != 0:
            raise RuntimeError(got.stderr.decode("utf-8", errors="replace"))
        return got.stdout.decode("ascii").strip()

    def show(self, rel, rev="HEAD"):
        got = self.git("show", rev + ":" + rel)
        if got.rc != 0:
            raise RuntimeError(got.stderr.decode("utf-8", errors="replace"))
        return got.stdout

    def clean(self):
        got = self.git("status", "--porcelain")
        return got.rc == 0 and got.stdout == b""


def review_record(gid, docid):
    parts = [digest((name + ":" + gid + ":" + docid).encode("utf-8")) for name in ("ace", "coverage", "payload", "clauses")]
    block = "bundle v2 " + docid + "\n"
    for name, value in zip(("ace", "coverage", "payload", "clauses"), parts):
        block += name + " " + value + "\n"
    review = digest(block.encode("utf-8"))
    return parts + [review]


def rights_text(rows):
    text = RIGHTS_HEADER
    for row in rows:
        text += "\t".join(row) + "\n"
    return text


def write_guideline(repo, gid="g-red", profile="redistributable", docs=None, rights_rows=None, rights_only=False):
    if rights_rows is None:
        rights_rows = [(profile, "Fixture rights statement for " + gid + ".", "https://example.invalid/" + gid, "2026-01-01", "Fixture note.")]
    repo.write("guidelines/" + gid + "/rights.tsv", rights_text(rights_rows))
    if rights_only:
        return
    if docs is None:
        docs = {"doc-a": "unreviewed"}
    repo.write("guidelines/" + gid + "/README.md", "# " + gid + "\n\n## Rights and attribution\n\nFixture custody text.\n")
    repo.write("guidelines/" + gid + "/source/original.txt", ("source bytes for " + gid + "\n").encode("utf-8"))
    manifest = REVIEW_HEADER_1 + REVIEW_HEADER_2
    ledger_rows = []
    for docid in sorted(docs):
        values = review_record(gid, docid)
        manifest += docid + "\t" + "\t".join(values) + "\n"
        repo.write("guidelines/" + gid + "/ace/" + docid + ".ace", "Every fixture is a record.\n")
        repo.write("guidelines/" + gid + "/pl/" + docid + ".pl", "guideline_document('" + gid + "','" + docid + "',[],x).\n")
        state = docs[docid]
        review = values[-1]
        if state == "approved":
            ledger_rows.append((docid, "2026-01-01T00:00:00Z", review, "approved"))
        elif state == "rejected":
            ledger_rows.append((docid, "2026-01-01T00:00:00Z", review, "rejected"))
        elif state == "stale":
            ledger_rows.append((docid, "2026-01-01T00:00:00Z", digest((gid + docid + "old").encode()), "approved"))
        elif state == "contested":
            ledger_rows.append((docid, "2026-01-01T00:00:00Z", review, "approved"))
            ledger_rows.append((docid, "2026-01-01T00:00:01Z", review, "rejected"))
        elif state != "unreviewed":
            raise ValueError("bad state " + state)
    repo.write("guidelines/" + gid + "/audit/review-manifest.tsv", manifest)
    if ledger_rows:
        ledger = LEDGER_HEADER
        for docid, when, review, verdict in sorted(ledger_rows):
            ledger += docid + "\t" + review + "\t\t" + verdict + "\tfixture\t" + when + "\tfixture decision\n"
        repo.write("guidelines/" + gid + "/audit/adjudication.tsv", ledger)
    repo.write("guidelines/" + gid + "/queries/query-a.ace", "Is every fixture a record?\n")
    repo.write("guidelines/" + gid + "/queries/query-a.answers.pl", "answer_manifest(v1,query_a,yes,[]).\n")
    repo.write("guidelines/" + gid + "/queries/query-a.trace.pl", "trace_manifest(v1,query_a,yes,[]).\n")


def base_repo(path, profile="redistributable", docs=None, rights_rows=None, rights_only=False):
    repo = Repo(path)
    repo.write("README.md", ROOT_README)
    repo.write("NOTICE", ROOT_NOTICE)
    repo.write("vendor/ape/prolog/ace_to_pl.pl", "% fixture compiler\n")
    repo.write("vendor/clex/clex_lexicon.pl", "% fixture base lexicon\n")
    repo.write("tools/excluded.txt", "must not ship\n")
    repo.write("tests/excluded.txt", "must not ship\n")
    repo.write(".agent/excluded.txt", "must not ship\n")
    repo.write(".github/excluded.txt", "must not ship\n")
    write_guideline(repo, profile=profile, docs=docs, rights_rows=rights_rows, rights_only=rights_only)
    repo.commit("fixture corpus")
    return repo


def goal(repo, *args, env=None, timeout=180):
    merged = repo.env({"PYTHONDONTWRITEBYTECODE": "1"})
    if env:
        merged.update(env)
    return command([PYTHON, "-P", str(GOAL), *args], repo.path, merged, timeout)


def dist(repo, *args, env=None, timeout=180):
    merged = repo.env({"PYTHONDONTWRITEBYTECODE": "1"})
    if env:
        merged.update(env)
    return command([PYTHON, "-P", str(DIST), *args], repo.path, merged, timeout)


def require_ok(result, phase):
    if result.rc != 0:
        result.issue("setup " + phase)
        raise SetupFailure(result, phase)


def prepare_manifest(repo):
    made = goal(repo, "release-manifest")
    require_ok(made, "release-manifest")
    path = repo.path / "release-manifest.tsv"
    if not path.is_file():
        made.issue("release-manifest command wrote no file")
        raise SetupFailure(made, "release-manifest-output")
    data = path.read_bytes()
    repo.commit("fixture release manifest")
    return data


def build(repo, dest_name="out", env=None):
    dest = repo.path / dest_name
    tmp_root = repo.path.parent / (repo.path.name + "-process-tmp")
    tmp_root.mkdir(exist_ok=True)
    merged = {"TMPDIR": str(tmp_root)}
    if env:
        merged.update(env)
    result = dist(repo, "build", str(dest), env=merged)
    return result, dest, tmp_root


class Bag:
    def __init__(self, dest):
        archives = sorted(dest.glob("*.tar.gz")) if dest.is_dir() else []
        if len(archives) != 1:
            raise AssertionError("archive count=" + str(len(archives)))
        self.archive = archives[0]
        self.sidecar = pathlib.Path(str(self.archive) + ".sha256")
        if not self.sidecar.is_file():
            raise AssertionError("sidecar missing")
        self.raw = self.archive.read_bytes()
        self.tar_bytes = gzip.decompress(self.raw)
        with tarfile.open(fileobj=io.BytesIO(self.tar_bytes), mode="r:") as opened:
            self.members = opened.getmembers()
            self.files = {}
            for member in self.members:
                if member.isfile():
                    stream = opened.extractfile(member)
                    self.files[member.name] = stream.read()
        roots = {name.split("/", 1)[0] for name in self.files}
        if len(roots) != 1:
            raise AssertionError("bag root count=" + str(len(roots)))
        self.root = roots.pop()
        prefix = self.root + "/"
        self.rel = {name.removeprefix(prefix): data for name, data in self.files.items()}

    def release_rows(self):
        data = self.rel["release-manifest.tsv"]
        text = data.decode("utf-8")
        if not text.endswith("\n") or "\r" in text:
            raise AssertionError("release-manifest newline law")
        return [line.split("\t") for line in text[:-1].split("\n")]


def add_issue(result, condition, detail):
    if not condition:
        result.issue(detail)


def bag_after(result, dest):
    if result.rc != 0:
        return None
    try:
        return Bag(dest)
    except Exception as exc:
        result.issue("bag parse: " + str(exc))
        return None


def meta_value(rows, key):
    hits = [row[2] for row in rows if len(row) == 3 and row[0] == "meta" and row[1] == key]
    if len(hits) != 1:
        raise AssertionError("meta " + key + " count=" + str(len(hits)))
    return hits[0]


def mutate_after_manifest(case_dir, rel, data=None, remove=False):
    repo = base_repo(case_dir / "repo")
    prepare_manifest(repo)
    if remove:
        repo.remove(rel)
    else:
        repo.write(rel, data)
    repo.commit("fixture mutation")
    result, _, _ = build(repo)
    return result


def scenario_input_head(case_dir):
    repo = base_repo(case_dir / "repo")
    input_head_1 = repo.input_head()
    writer_1 = goal(repo, "release-manifest")
    require_ok(writer_1, "release-manifest initial")
    manifest_1 = (repo.path / "release-manifest.tsv").read_bytes()
    repo.commit("fixture release manifest")
    writer_2 = goal(repo, "release-manifest")
    require_ok(writer_2, "release-manifest after commit")
    add_issue(writer_2, (repo.path / "release-manifest.tsv").read_bytes() == manifest_1, "manifest changed after manifest-only commit")
    add_issue(writer_2, repo.clean(), "writer dirt after manifest-only commit")
    repo.write("docs/unrelated.txt", "unrelated\n")
    repo.commit("unrelated docs")
    add_issue(writer_2, repo.input_head() == input_head_1, "unrelated commit changed input head")
    writer_3 = goal(repo, "release-manifest")
    require_ok(writer_3, "release-manifest after unrelated commit")
    add_issue(writer_3, (repo.path / "release-manifest.tsv").read_bytes() == manifest_1, "unrelated commit changed manifest")
    first, first_dest, _ = build(repo, "out-one")
    require_ok(first, "first input-head build")
    bag_1 = bag_after(first, first_dest)
    if bag_1:
        rows = bag_1.release_rows()
        add_issue(first, meta_value(rows, "head") == input_head_1, "meta head does not bind input head")
        add_issue(first, bag_1.archive.name == "cnl-ckc-kb-g" + input_head_1[:12] + ".tar.gz", "archive name does not bind input head")
        add_issue(first, bag_1.root == "cnl-ckc-kb-g" + input_head_1[:12], "bag root does not bind input head")
    repo.write("vendor/clex/clex_lexicon.pl", "% changed fixture base lexicon\n")
    input_head_2 = repo.commit("compiler input change")
    add_issue(first, repo.input_head() == input_head_2, "compiler input commit not selected")
    writer_4 = goal(repo, "release-manifest")
    require_ok(writer_4, "release-manifest after input change")
    repo.commit("updated release manifest")
    second, second_dest, _ = build(repo, "out-two")
    if second.rc == 0:
        bag_2 = bag_after(second, second_dest)
        if bag_2:
            add_issue(second, bag_2.archive.name == "cnl-ckc-kb-g" + input_head_2[:12] + ".tar.gz", "changed input head not in archive name")
            add_issue(second, bag_2.archive.name != bag_1.archive.name if bag_1 else False, "input change retained archive name")
    return second


def scenario_reconstructable_source(case_dir):
    repo = base_repo(case_dir / "repo", profile="reconstructable")
    source = repo.show("guidelines/g-red/source/original.txt")
    url = "https://example.invalid/g-red"
    prepare_manifest(repo)
    result, dest, _ = build(repo)
    bag = bag_after(result, dest)
    if bag:
        source_path = "data/guidelines/g-red/source/original.txt"
        add_issue(result, source_path not in bag.rel, "reconstructable source shipped")
        rows = bag.release_rows()
        wanted = ["source", source_path, digest(source), str(len(source)), url]
        add_issue(result, wanted in rows, "reconstructable source row mismatch")
    return result


def scenario_reconstructable_empty_url(case_dir):
    repo = base_repo(case_dir / "repo")
    prepare_manifest(repo)
    repo.write("guidelines/g-red/rights.tsv", rights_text([("reconstructable", "Fixture rights.", "", "2026-01-01", "note")]))
    repo.commit("empty reconstructable url")
    result, _, _ = build(repo)
    return result


def rights_mutation(case_dir, data=None, remove=False):
    return mutate_after_manifest(case_dir, "guidelines/g-red/rights.tsv", data=data, remove=remove)


def scenario_rights_missing(case_dir):
    return rights_mutation(case_dir, remove=True)


def scenario_rights_header(case_dir):
    return rights_mutation(case_dir, "bad\theader\nredistributable\tstatement\thttps://example.invalid\t2026-01-01\tnote\n")


def scenario_rights_rows(case_dir):
    return rights_mutation(case_dir, RIGHTS_HEADER)


def scenario_rights_fields(case_dir):
    return rights_mutation(case_dir, RIGHTS_HEADER + "redistributable\tstatement\thttps://example.invalid\t2026-01-01\tnote\textra\n")


def scenario_rights_profile(case_dir):
    return rights_mutation(case_dir, rights_text([("other", "statement", "https://example.invalid", "2026-01-01", "note")]))


def scenario_rights_statement(case_dir):
    return rights_mutation(case_dir, rights_text([("redistributable", "", "https://example.invalid", "2026-01-01", "note")]))


def scenario_rights_url(case_dir):
    return rights_mutation(case_dir, rights_text([("redistributable", "statement", "", "2026-01-01", "note")]))


def scenario_rights_retrieved(case_dir):
    return rights_mutation(case_dir, rights_text([("redistributable", "statement", "https://example.invalid", "2026-02-30", "note")]))


def scenario_rights_control(case_dir):
    data = RIGHTS_HEADER.encode() + b"redistributable\tstatement\thttps://example.invalid\t2026-01-01\tnote\x7f\n"
    return rights_mutation(case_dir, data)


def scenario_rights_utf8(case_dir):
    data = RIGHTS_HEADER.encode() + b"redistributable\tbad\xff\thttps://example.invalid\t2026-01-01\tnote\n"
    return rights_mutation(case_dir, data)


def scenario_rights_second_row_invalid(case_dir):
    rows = [
        ("redistributable", "operative", "https://example.invalid/one", "2026-01-01", "one"),
        ("invalid", "second", "https://example.invalid/two", "2026-01-02", "two"),
    ]
    return rights_mutation(case_dir, rights_text(rows))


def scenario_rights_first_row_operative(case_dir):
    rows = [
        ("redistributable", "operative row statement", "https://example.invalid/one", "2026-01-01", "one"),
        ("restricted", "secondary row statement", "https://example.invalid/two", "2026-01-02", "two"),
    ]
    repo = base_repo(case_dir / "repo", rights_rows=rows)
    source = repo.show("guidelines/g-red/source/original.txt")
    prepare_manifest(repo)
    result, dest, _ = build(repo)
    bag = bag_after(result, dest)
    if bag:
        add_issue(result, bag.rel.get("data/guidelines/g-red/source/original.txt") == source, "row 2 changed operative profile")
    return result


def scenario_stray_root(case_dir):
    repo = base_repo(case_dir / "repo")
    prepare_manifest(repo)
    repo.write("guidelines/stray.txt", "stray\n")
    repo.commit("stray guideline root member")
    result, _, _ = build(repo)
    return result


def scenario_no_guidelines(case_dir):
    repo = base_repo(case_dir / "repo")
    prepare_manifest(repo)
    repo.remove("guidelines")
    repo.commit("remove guidelines")
    result, _, _ = build(repo)
    return result


def scenario_rights_only(case_dir):
    repo = base_repo(case_dir / "repo", rights_only=True)
    prepare_manifest(repo)
    result, dest, _ = build(repo)
    bag = bag_after(result, dest)
    if bag:
        add_issue(result, "data/guidelines/g-red/rights.tsv" in bag.rel, "rights-only guideline absent")
    return result


def scenario_committed_state(case_dir):
    repo = base_repo(case_dir / "repo")
    committed_source = repo.show("guidelines/g-red/source/original.txt")
    committed_readme = repo.show("README.md")
    committed_notice = repo.show("NOTICE")
    committed_compiler = repo.show("vendor/ape/prolog/ace_to_pl.pl")
    input_head = repo.input_head()
    committed_manifest = prepare_manifest(repo)
    repo.write("guidelines/g-red/source/original.txt", "dirty source\n")
    repo.write("guidelines/g-red/rights.tsv", b"bad\xff")
    repo.write("README.md", "dirty readme\n")
    repo.write("NOTICE", "dirty notice\n")
    repo.write("vendor/ape/prolog/ace_to_pl.pl", "% dirty compiler\n")
    repo.write("release-manifest.tsv", "dirty manifest\n")
    result, dest, _ = build(repo)
    bag = bag_after(result, dest)
    if bag:
        add_issue(result, bag.rel.get("data/guidelines/g-red/source/original.txt") == committed_source, "working source leaked")
        add_issue(result, SCHEMA_SECTION.encode() in bag.rel.get("README-dist.md", b""), "committed schema section absent")
        add_issue(result, b"dirty readme" not in bag.rel.get("README-dist.md", b""), "working README leaked")
        add_issue(result, committed_notice in bag.rel.get("NOTICE", b""), "committed NOTICE absent")
        add_issue(result, b"dirty notice" not in bag.rel.get("NOTICE", b""), "working NOTICE leaked")
        add_issue(result, bag.rel.get("release-manifest.tsv") == committed_manifest, "working release manifest leaked")
        rows = bag.release_rows()
        add_issue(result, meta_value(rows, "compiler") == digest(committed_compiler), "working compiler leaked")
        add_issue(result, meta_value(rows, "head") == input_head, "working input changed head")
        add_issue(result, committed_readme.startswith(b"# Fixture"), "fixture precondition")
    return result


def scenario_manifest_order(case_dir):
    repo = base_repo(case_dir / "repo")
    prepare_manifest(repo)
    result, dest, _ = build(repo)
    bag = bag_after(result, dest)
    if not bag:
        return result
    rows = bag.release_rows()
    kinds = [row[0] for row in rows]
    rank = {"meta": 0, "member": 1, "source": 2, "label": 3}
    add_issue(result, all(kinds[i] in rank for i in range(len(kinds))), "manifest row kind")
    add_issue(result, [rank[k] for k in kinds] == sorted(rank[k] for k in kinds), "manifest row group order")
    fixed = [row[1] for row in rows if row[0] == "meta" and row[1] not in ("replay", "generated")]
    add_issue(result, fixed == ["schema", "head", "compiler", "base-lexicon", "python", "swipl", "verify"], "fixed meta order")
    replay = [row[2] for row in rows if row[:2] == ["meta", "replay"]]
    add_issue(result, len(replay) == 3, "replay row count")
    if len(replay) == 3:
        add_issue(result, "compile" in replay[0] and "check" in replay[1] and "load" in replay[2], "replay order")
    generated = [row[2] for row in rows if row[:2] == ["meta", "generated"]]
    add_issue(result, generated == ["release-manifest.tsv", "manifest-sha256.txt", "tagmanifest-sha256.txt"], "generated row order")
    for kind, path_index in (("member", 1), ("source", 1), ("label", 1)):
        paths = [row[path_index] for row in rows if row[0] == kind]
        add_issue(result, paths == sorted(paths, key=lambda text: text.encode("utf-8")), kind + " sort order")
    data = bag.rel["release-manifest.tsv"]
    add_issue(result, data.endswith(b"\n") and b"\r" not in data, "manifest LF law")
    return result


def scenario_runtime_meta(case_dir):
    repo = base_repo(case_dir / "repo")
    compiler = repo.show("vendor/ape/prolog/ace_to_pl.pl")
    lexicon = repo.show("vendor/clex/clex_lexicon.pl")
    input_head = repo.input_head()
    prepare_manifest(repo)
    result, dest, _ = build(repo)
    bag = bag_after(result, dest)
    if bag:
        rows = bag.release_rows()
        expected = {
            "schema": "v1",
            "head": input_head,
            "compiler": digest(compiler),
            "base-lexicon": digest(lexicon),
            "python": "3.11",
            "swipl": "9.2.9",
            "verify": "sha256sum -c manifest-sha256.txt tagmanifest-sha256.txt",
        }
        for key, value in expected.items():
            add_issue(result, meta_value(rows, key) == value, "meta " + key + " mismatch")
    return result


def scenario_manifest_source_drift(case_dir):
    repo = base_repo(case_dir / "repo")
    prepare_manifest(repo)
    repo.write("guidelines/g-red/source/original.txt", "changed committed source\n")
    repo.commit("source drift")
    result, _, _ = build(repo)
    return result


def scenario_manifest_tamper(case_dir):
    repo = base_repo(case_dir / "repo")
    prepare_manifest(repo)
    path = repo.path / "release-manifest.tsv"
    lines = path.read_text(encoding="utf-8").splitlines()
    prefix = "member\tdata/guidelines/g-red/source/original.txt\t"
    changed = False
    for index, line in enumerate(lines):
        if line.startswith(prefix):
            fields = line.split("\t")
            fields[2] = "0" * 64
            lines[index] = "\t".join(fields)
            changed = True
            break
    if not changed:
        return Result(125, b"", b"harness: source member row absent\n")
    path.write_text("\n".join(lines) + "\n", encoding="utf-8")
    repo.commit("tamper release manifest")
    result, _, _ = build(repo)
    return result


def scenario_writer_idempotent(case_dir):
    repo = base_repo(case_dir / "repo")
    first = goal(repo, "release-manifest")
    require_ok(first, "release-manifest first")
    path = repo.path / "release-manifest.tsv"
    before = path.read_bytes()
    repo.commit("fixture release manifest")
    second = goal(repo, "release-manifest")
    if second.rc == 0:
        add_issue(second, path.read_bytes() == before, "writer changed bytes after commit")
        add_issue(second, repo.clean(), "writer left worktree dirty")
    return second


def profile_repo(path):
    repo = base_repo(path, docs={"doc-approved": "approved", "doc-stale": "stale", "doc-unreviewed": "unreviewed"})
    write_guideline(repo, gid="g-fetch", profile="reconstructable", docs={"fetch-doc": "unreviewed"})
    write_guideline(repo, gid="g-hold", profile="restricted", docs={"held-a": "approved", "held-b": "unreviewed"})
    repo.commit("profile fixtures")
    return repo


def scenario_restricted_labels(case_dir):
    repo = profile_repo(case_dir / "repo")
    prepare_manifest(repo)
    result, dest, _ = build(repo)
    bag = bag_after(result, dest)
    if not bag:
        return result
    rows = bag.release_rows()
    labels = {row[1]: row[2] for row in rows if row[0] == "label"}
    expected = {
        "doc-approved": "approved",
        "doc-stale": "stale",
        "doc-unreviewed": "unreviewed",
        "fetch-doc": "unreviewed",
    }
    add_issue(result, labels == expected, "restricted/shipped label set mismatch")
    add_issue(result, not any(path.startswith("data/guidelines/g-hold/") for path in bag.rel), "restricted payload shipped")
    readme = bag.rel.get("README-dist.md", b"").decode("utf-8", errors="replace")
    notice = bag.rel.get("NOTICE", b"").decode("utf-8", errors="replace")
    add_issue(result, "g-hold" in readme and "2" in readme, "held-back id/count absent")
    add_issue(result, "held-a" not in readme and "held-b" not in readme, "held-back docid disclosed")
    add_issue(result, "Fixture rights statement for g-hold." in readme, "restricted rights absent from README")
    add_issue(result, "Fixture rights statement for g-hold." in notice, "restricted rights absent from NOTICE")
    return result


def scenario_rejected_order(case_dir):
    repo = base_repo(case_dir / "repo", docs={"z-rejected": "rejected", "a-rejected": "rejected", "m-approved": "approved"})
    prepare_manifest(repo)
    result, dest, tmp_root = build(repo)
    add_issue(result, not dest.exists() or not any(dest.iterdir()), "rejected build left finals")
    add_issue(result, not any(tmp_root.iterdir()), "rejected build leaked process temp")
    return result


def scenario_contested(case_dir):
    repo = base_repo(case_dir / "repo", docs={"doc-contested": "contested"})
    prepare_manifest(repo)
    result, dest, _ = build(repo)
    add_issue(result, not dest.exists() or not any(dest.iterdir()), "contested build left finals")
    return result


def scenario_label_classes(case_dir):
    repo = base_repo(case_dir / "repo", docs={"doc-approved": "approved", "doc-stale": "stale", "doc-unreviewed": "unreviewed"})
    prepare_manifest(repo)
    result, dest, _ = build(repo)
    bag = bag_after(result, dest)
    if bag:
        rows = bag.release_rows()
        labels = [row for row in rows if row[0] == "label"]
        add_issue(result, labels == [
            ["label", "doc-approved", "approved"],
            ["label", "doc-stale", "stale"],
            ["label", "doc-unreviewed", "unreviewed"],
        ], "adjudication label classes/order")
    return result


def scenario_exec_mode(case_dir):
    repo = base_repo(case_dir / "repo")
    target = repo.path / "guidelines/g-red/ace/doc-a.ace"
    target.chmod(0o755)
    repo.commit("executable corpus blob")
    payload = repo.show("guidelines/g-red/ace/doc-a.ace")
    prepare_manifest(repo)
    result, dest, _ = build(repo)
    bag = bag_after(result, dest)
    if bag:
        full = bag.root + "/data/guidelines/g-red/ace/doc-a.ace"
        hits = [member for member in bag.members if member.name == full]
        add_issue(result, len(hits) == 1 and hits[0].mode == 0o644, "executable mode not normalized")
        rows = bag.release_rows()
        wanted = [row for row in rows if row[:2] == ["member", "data/guidelines/g-red/ace/doc-a.ace"]]
        add_issue(result, len(wanted) == 1 and wanted[0][2] == digest(payload), "exec member digest mismatch")
    return result


def scenario_output_collision(case_dir):
    repo = base_repo(case_dir / "repo")
    prepare_manifest(repo)
    first, dest, tmp_root = build(repo)
    require_ok(first, "collision seed build")
    bag = bag_after(first, dest)
    if not bag:
        return first
    bag.archive.write_bytes(bag.archive.read_bytes() + b"tamper")
    archive_after_tamper = bag.archive.read_bytes()
    sidecar_before = bag.sidecar.read_bytes()
    second = dist(repo, "build", str(dest), env={"TMPDIR": str(tmp_root)})
    add_issue(second, bag.archive.read_bytes() == archive_after_tamper, "collision archive replaced")
    add_issue(second, bag.sidecar.read_bytes() == sidecar_before, "collision sidecar replaced")
    add_issue(second, not any(tmp_root.iterdir()), "collision leaked process temp")
    return second


def scenario_gzip_header(case_dir):
    repo = base_repo(case_dir / "repo")
    prepare_manifest(repo)
    result, dest, _ = build(repo)
    bag = bag_after(result, dest)
    if bag:
        add_issue(result, bag.raw[:10] == bytes.fromhex("1f8b08000000000002ff"), "gzip header=" + bag.raw[:10].hex())
        if len(bag.raw) >= 18:
            stored_crc, stored_size = struct.unpack("<II", bag.raw[-8:])
            add_issue(result, stored_crc == zlib.crc32(bag.tar_bytes) & 0xFFFFFFFF, "gzip trailer CRC")
            add_issue(result, stored_size == len(bag.tar_bytes) & 0xFFFFFFFF, "gzip trailer ISIZE")
    return result


def scenario_sidecar(case_dir):
    repo = base_repo(case_dir / "repo")
    prepare_manifest(repo)
    result, dest, _ = build(repo)
    bag = bag_after(result, dest)
    if bag:
        expected = digest(bag.raw) + "  " + bag.archive.name + "\n"
        add_issue(result, bag.sidecar.read_text(encoding="utf-8") == expected, "sidecar grammar/digest")
        add_issue(result, sorted(path.name for path in dest.iterdir()) == sorted([bag.archive.name, bag.sidecar.name]), "destination final set")
    return result


def scenario_symlink_member(case_dir):
    repo = base_repo(case_dir / "repo")
    prepare_manifest(repo)
    os.symlink("doc-a.ace", repo.path / "guidelines/g-red/ace/link.ace")
    repo.commit("symlink member")
    result, _, _ = build(repo)
    return result


def scenario_gitlink_member(case_dir):
    repo = base_repo(case_dir / "repo")
    prepare_manifest(repo)
    oid = repo.head()
    staged = repo.git("update-index", "--add", "--cacheinfo", "160000," + oid + ",guidelines/g-red/submodule")
    if staged.rc != 0:
        staged.issue("gitlink setup")
        return staged
    repo.commit_index("gitlink member")
    result, _, _ = build(repo)
    return result


def scenario_member_precedence(case_dir):
    repo = base_repo(case_dir / "repo")
    prepare_manifest(repo)
    os.symlink("doc-a.ace", repo.path / "guidelines/g-red/ace/link.ace")
    repo.remove("guidelines/g-red/rights.tsv")
    repo.commit("symlink plus rights defect")
    result, _, _ = build(repo)
    return result


def scenario_success_meter(case_dir):
    repo = base_repo(case_dir / "repo")
    input_head = repo.input_head()
    prepare_manifest(repo)
    result, dest, _ = build(repo)
    bag = bag_after(result, dest)
    if result.rc == 0 and bag:
        lines = result.stdout.decode("utf-8", errors="replace").splitlines(keepends=True)
        add_issue(result, len(lines) == 2 and all(line.endswith("\n") for line in lines), "success stdout line count/LF")
        if len(lines) == 2:
            add_issue(result, re.fullmatch(r"dist: ok [0-9]+ guidelines [0-9]+ members [0-9]+ bytes\n", lines[0]) is not None, "success meter line")
            expected_sha = "dist: sha256=" + digest(bag.raw) + " cnl-ckc-kb-g" + input_head[:12] + ".tar.gz\n"
            add_issue(result, lines[1] == expected_sha, "sha meter line")
    return result


def scenario_byte_determinism(case_dir):
    repo = base_repo(case_dir / "repo")
    prepare_manifest(repo)
    first, dest_one, _ = build(repo, "out-one", {"PYTHONHASHSEED": "1", "TZ": "Pacific/Honolulu", "LC_ALL": "C"})
    require_ok(first, "determinism build one")
    second, dest_two, _ = build(repo, "out-two", {"PYTHONHASHSEED": "99991", "TZ": "UTC", "LC_ALL": "C.UTF-8"})
    if second.rc == 0:
        bag_one = bag_after(first, dest_one)
        bag_two = bag_after(second, dest_two)
        if bag_one and bag_two:
            add_issue(second, bag_one.raw == bag_two.raw, "archive bytes differ across environment/dest")
            add_issue(second, bag_one.sidecar.read_bytes() == bag_two.sidecar.read_bytes(), "sidecar bytes differ")
    return second


def raw_tar_headers(data):
    headers = []
    offset = 0
    while offset + 512 <= len(data):
        block = data[offset:offset + 512]
        if block == b"\0" * 512:
            break
        size_field = block[124:136].rstrip(b"\0 ").lstrip(b" ")
        size = int(size_field or b"0", 8)
        stored_field = block[148:156].rstrip(b"\0 ").lstrip(b" ")
        stored = int(stored_field or b"0", 8)
        check_block = block[:148] + b" " * 8 + block[156:]
        prefix = block[345:500].split(b"\0", 1)[0]
        name = block[:100].split(b"\0", 1)[0]
        full = prefix + (b"/" if prefix and name else b"") + name
        headers.append({
            "name": full,
            "mode": block[100:108],
            "uid": block[108:116],
            "gid": block[116:124],
            "mtime": block[136:148],
            "type": block[156:157],
            "magic": block[257:263],
            "version": block[263:265],
            "uname": block[265:297],
            "gname": block[297:329],
            "checksum_ok": stored == sum(check_block),
        })
        offset += 512 + ((size + 511) // 512) * 512
    return headers


def scenario_tar_fields(case_dir):
    repo = base_repo(case_dir / "repo")
    input_head = repo.input_head()
    epoch_result = repo.git("show", "-s", "--format=%ct", input_head)
    epoch = int(epoch_result.stdout.decode("ascii").strip())
    prepare_manifest(repo)
    result, dest, _ = build(repo)
    bag = bag_after(result, dest)
    if not bag:
        return result
    names = [member.name for member in bag.members]
    add_issue(result, names == sorted(names, key=lambda text: text.encode("utf-8")), "tar member order")
    add_issue(result, all(member.isfile() for member in bag.members), "tar contains directory/non-file member")
    for member in bag.members:
        add_issue(result, member.uid == 0 and member.gid == 0, "tar uid/gid " + member.name)
        add_issue(result, member.uname == "" and member.gname == "", "tar owner names " + member.name)
        add_issue(result, member.mode == 0o644, "tar mode " + member.name)
        add_issue(result, member.mtime == epoch, "tar mtime " + member.name)
        add_issue(result, not member.pax_headers, "tar pax headers " + member.name)
    headers = raw_tar_headers(bag.tar_bytes)
    add_issue(result, len(headers) == len(bag.members), "raw header/member count")
    for header in headers:
        add_issue(result, header["type"] in (b"0", b"\0"), "raw non-file type")
        add_issue(result, header["magic"] == b"ustar\0" and header["version"] == b"00", "raw USTAR pin")
        add_issue(result, header["checksum_ok"], "raw tar checksum")
        add_issue(result, header["uname"].rstrip(b"\0") == b"" and header["gname"].rstrip(b"\0") == b"", "raw owner name")
    return result


def scenario_longname(case_dir):
    repo = base_repo(case_dir / "repo")
    long_leaf = "x" * 150 + ".txt"
    rel = "guidelines/g-red/source/" + long_leaf
    repo.write(rel, "long path bytes\n")
    repo.commit("long member path")
    prepare_manifest(repo)
    result, dest, _ = build(repo)
    bag = bag_after(result, dest)
    if bag:
        add_issue(result, "data/" + rel in bag.rel, "long member absent")
        headers = raw_tar_headers(bag.tar_bytes)
        add_issue(result, any(header["type"] == b"L" for header in headers), "GNU longname fallback absent")
        add_issue(result, not any(header["type"] in (b"x", b"g") for header in headers), "PAX fallback used")
    return result


def parse_digest_lines(data):
    if not data.endswith(b"\n") or b"\r" in data or data.startswith(b"\xef\xbb\xbf"):
        raise AssertionError("digest file encoding/newline")
    rows = []
    for raw_line in data[:-1].split(b"\n"):
        match = re.fullmatch(rb"([0-9a-f]{64})  (.+)", raw_line)
        if match is None:
            raise AssertionError("digest line grammar " + ascii(raw_line))
        rows.append((match.group(2).decode("utf-8"), match.group(1).decode("ascii")))
    paths = [path for path, _ in rows]
    if paths != sorted(paths, key=lambda text: text.encode("utf-8")):
        raise AssertionError("digest path order")
    return rows


def scenario_bagit_closure(case_dir):
    repo = profile_repo(case_dir / "repo")
    prepare_manifest(repo)
    result, dest, _ = build(repo)
    bag = bag_after(result, dest)
    if not bag:
        return result
    rows = bag.release_rows()
    member_rows = [row for row in rows if row[0] == "member"]
    member_paths = [row[1] for row in member_rows]
    expected_members = sorted(
        [path for path in bag.rel if path.startswith("data/guidelines/")] + ["bagit.txt", "README-dist.md", "NOTICE"],
        key=lambda text: text.encode("utf-8"),
    )
    add_issue(result, member_paths == expected_members, "release member closure")
    generated = [row for row in rows if row[:2] == ["meta", "generated"]]
    add_issue(result, generated == [
        ["meta", "generated", "release-manifest.tsv"],
        ["meta", "generated", "manifest-sha256.txt"],
        ["meta", "generated", "tagmanifest-sha256.txt"],
    ], "generated digestless closure")
    try:
        payload_rows = parse_digest_lines(bag.rel["manifest-sha256.txt"])
        tag_rows = parse_digest_lines(bag.rel["tagmanifest-sha256.txt"])
        add_issue(result, [path for path, _ in payload_rows] == sorted(path for path in bag.rel if path.startswith("data/")), "payload manifest closure")
        expected_tags = sorted(path for path in bag.rel if not path.startswith("data/") and path != "tagmanifest-sha256.txt")
        add_issue(result, [path for path, _ in tag_rows] == expected_tags, "tagmanifest closure")
    except Exception as exc:
        result.issue("checksum closure: " + str(exc))
    return result


def scenario_profile_member_set(case_dir):
    repo = profile_repo(case_dir / "repo")
    red_source = repo.show("guidelines/g-red/source/original.txt")
    fetch_source = repo.show("guidelines/g-fetch/source/original.txt")
    prepare_manifest(repo)
    result, dest, _ = build(repo)
    bag = bag_after(result, dest)
    if not bag:
        return result
    add_issue(result, bag.rel.get("data/guidelines/g-red/source/original.txt") == red_source, "redistributable source absent")
    add_issue(result, "data/guidelines/g-fetch/source/original.txt" not in bag.rel, "reconstructable source present")
    add_issue(result, not any(path.startswith("data/guidelines/g-hold/") for path in bag.rel), "restricted guideline present")
    for rel in ("rights.tsv", "ace/fetch-doc.ace", "pl/fetch-doc.pl", "audit/review-manifest.tsv"):
        add_issue(result, "data/guidelines/g-fetch/" + rel in bag.rel, "reconstructable non-source absent: " + rel)
    rows = bag.release_rows()
    source_row = ["source", "data/guidelines/g-fetch/source/original.txt", digest(fetch_source), str(len(fetch_source)), "https://example.invalid/g-fetch"]
    add_issue(result, source_row in rows, "reconstructable source authority row")
    return result


def scenario_bag_layout(case_dir):
    repo = base_repo(case_dir / "repo")
    input_head = repo.input_head()
    prepare_manifest(repo)
    result, dest, _ = build(repo)
    bag = bag_after(result, dest)
    if not bag:
        return result
    required = {"bagit.txt", "manifest-sha256.txt", "tagmanifest-sha256.txt", "README-dist.md", "NOTICE", "release-manifest.tsv"}
    add_issue(result, required.issubset(bag.rel), "required BagIt tags")
    add_issue(result, bag.rel.get("bagit.txt") == b"BagIt-Version: 1.0\nTag-File-Character-Encoding: UTF-8\n", "bagit.txt bytes")
    add_issue(result, "bag-info.txt" not in bag.rel, "bag-info.txt present")
    add_issue(result, all(path in required or path.startswith("data/guidelines/") for path in bag.rel), "bag path outside layout")
    add_issue(result, bag.root == "cnl-ckc-kb-g" + input_head[:12], "single bag root name")
    add_issue(result, all(member.isfile() for member in bag.members), "serialized directory member")
    return result


def scenario_exclusion(case_dir):
    repo = base_repo(case_dir / "repo")
    prepare_manifest(repo)
    result, dest, _ = build(repo)
    bag = bag_after(result, dest)
    if bag:
        forbidden = ("tools/", "vendor/", "tests/", ".agent/", ".github/", ".git/")
        add_issue(result, not any(path.startswith(forbidden) for path in bag.rel), "non-guideline repository tree shipped")
        add_issue(result, not any(b"must not ship" in data for data in bag.rel.values()), "excluded marker bytes shipped")
    return result


def scenario_consumer_copy(case_dir):
    repo = profile_repo(case_dir / "repo")
    prepare_manifest(repo)
    result, dest, _ = build(repo)
    bag = bag_after(result, dest)
    if not bag:
        return result
    readme = bag.rel.get("README-dist.md", b"")
    notice = bag.rel.get("NOTICE", b"")
    add_issue(result, SCHEMA_SECTION.encode("utf-8") in readme, "schema section not embedded verbatim")
    add_issue(result, ROOT_NOTICE.encode("utf-8") in notice, "root NOTICE not embedded")
    for gid in ("g-red", "g-fetch", "g-hold"):
        statement = ("Fixture rights statement for " + gid + ".").encode("utf-8")
        add_issue(result, statement in readme, "README rights missing " + gid)
        add_issue(result, statement in notice, "NOTICE rights missing " + gid)
    add_issue(result, b"sha256sum -c manifest-sha256.txt tagmanifest-sha256.txt" in readme, "verification command absent")
    add_issue(result, b"compile" in readme and b"check" in readme and b"load" in readme, "replay commands absent")
    lower_notice = notice.lower()
    add_issue(result, b"compiler" in lower_notice and b"derived" in lower_notice, "compiler-derived output statement absent")
    return result


def unsafe_path_repo(case_dir):
    repo = base_repo(case_dir / "repo")
    prepare_manifest(repo)
    return repo


def scenario_newline_path(case_dir):
    repo = unsafe_path_repo(case_dir)
    repo.write("guidelines/g-red/ace/bad\nname.ace", "bad path\n")
    repo.commit("newline path")
    result, _, _ = build(repo)
    add_issue(result, b"\\n" in result.stderr, "newline path rendering not escaped")
    return result


def scenario_invalid_utf8_path(case_dir):
    repo = unsafe_path_repo(case_dir)
    parent = os.fsencode(repo.path / "guidelines/g-red/ace")
    raw_path = parent + b"/bad-\xff.ace"
    descriptor = os.open(raw_path, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o644)
    os.write(descriptor, b"bad path\n")
    os.close(descriptor)
    repo.commit("invalid utf8 path")
    result, _, _ = build(repo)
    add_issue(result, b"\\xff" in result.stderr, "non-UTF8 rendering not ascii escaped")
    return result


def scenario_backslash_path(case_dir):
    repo = unsafe_path_repo(case_dir)
    repo.write("guidelines/g-red/ace/bad\\name.ace", "bad path\n")
    repo.commit("backslash path")
    result, _, _ = build(repo)
    add_issue(result, b"\\\\" in result.stderr, "backslash path rendering not escaped")
    return result


def scenario_space_path(case_dir):
    repo = base_repo(case_dir / "repo")
    rel = "guidelines/g-red/source/space name.txt"
    repo.write(rel, "space path\n")
    repo.commit("space path")
    prepare_manifest(repo)
    result, dest, _ = build(repo)
    bag = bag_after(result, dest)
    if bag:
        path = "data/" + rel
        add_issue(result, bag.rel.get(path) == b"space path\n", "space path payload absent")
        try:
            payload = dict(parse_digest_lines(bag.rel["manifest-sha256.txt"]))
            add_issue(result, path in payload, "space path checksum absent")
        except Exception as exc:
            result.issue("space checksum parse: " + str(exc))
    return result


def extract_bag(bag, dest):
    root = dest / bag.root
    for rel, data in bag.rel.items():
        target = root / pathlib.PurePosixPath(rel)
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(data)
    return root


def scenario_sha256_command(case_dir):
    repo = profile_repo(case_dir / "repo")
    repo.write("guidelines/g-red/source/space name.txt", "space path\n")
    repo.commit("verification space path")
    prepare_manifest(repo)
    result, dest, _ = build(repo)
    bag = bag_after(result, dest)
    if bag:
        bag_root = extract_bag(bag, case_dir / "extract")
        verified = command(["sha256sum", "-c", "manifest-sha256.txt", "tagmanifest-sha256.txt"], bag_root, timeout=60)
        add_issue(result, verified.rc == 0, "sha256sum verify rc=" + str(verified.rc))
        add_issue(result, verified.stderr == b"", "sha256sum verify stderr")
    return result


def scenario_checksum_tamper(case_dir):
    repo = base_repo(case_dir / "repo")
    prepare_manifest(repo)
    result, dest, _ = build(repo)
    bag = bag_after(result, dest)
    if bag:
        bag_root = extract_bag(bag, case_dir / "extract")
        payloads = sorted(path for path in bag.rel if path.startswith("data/"))
        if not payloads:
            result.issue("tamper probe has no payload")
            return result
        target = bag_root / pathlib.PurePosixPath(payloads[0])
        target.write_bytes(target.read_bytes() + b"tamper")
        verified = command(["sha256sum", "-c", "manifest-sha256.txt", "tagmanifest-sha256.txt"], bag_root, timeout=60)
        add_issue(result, verified.rc == 1, "tampered sha256sum rc=" + str(verified.rc))
        add_issue(result, b"FAILED" in verified.stdout or b"FAILED" in verified.stderr, "tampered checksum not reported")
    return result


def scenario_tag_self_exclusion(case_dir):
    repo = base_repo(case_dir / "repo")
    prepare_manifest(repo)
    result, dest, _ = build(repo)
    bag = bag_after(result, dest)
    if bag:
        try:
            rows = parse_digest_lines(bag.rel["tagmanifest-sha256.txt"])
            paths = [path for path, _ in rows]
            add_issue(result, "tagmanifest-sha256.txt" not in paths, "tagmanifest hashes itself")
            expected = sorted(path for path in bag.rel if not path.startswith("data/") and path != "tagmanifest-sha256.txt")
            add_issue(result, paths == expected, "tagmanifest root tag set")
        except Exception as exc:
            result.issue("tagmanifest parse: " + str(exc))
    return result


def scenario_release_member_digests(case_dir):
    repo = profile_repo(case_dir / "repo")
    prepare_manifest(repo)
    result, dest, _ = build(repo)
    bag = bag_after(result, dest)
    if bag:
        rows = bag.release_rows()
        for row in [row for row in rows if row[0] == "member"]:
            path, expected_digest, expected_bytes = row[1:]
            data = bag.rel.get(path)
            add_issue(result, data is not None, "member row path absent " + path)
            if data is not None:
                add_issue(result, digest(data) == expected_digest, "member digest " + path)
                add_issue(result, str(len(data)) == expected_bytes, "member bytes " + path)
        add_issue(result, not any(row[0] == "member" and row[1] in ("release-manifest.tsv", "manifest-sha256.txt", "tagmanifest-sha256.txt") for row in rows), "circular member digest row")
    return result


def process_tmp(repo):
    path = repo.path.parent / (repo.path.name + "-process-tmp")
    path.mkdir(exist_ok=True)
    return path


def scenario_usage_missing(case_dir):
    repo = base_repo(case_dir / "repo")
    return dist(repo)


def scenario_usage_mode(case_dir):
    repo = base_repo(case_dir / "repo")
    return dist(repo, "inspect")


def scenario_default_dest(case_dir):
    repo = base_repo(case_dir / "repo")
    prepare_manifest(repo)
    tmp_root = process_tmp(repo)
    result = dist(repo, "build", env={"TMPDIR": str(tmp_root)})
    dest = repo.path / "dist"
    bag = bag_after(result, dest)
    if bag:
        add_issue(result, dest.is_dir(), "default dist directory absent")
        add_issue(result, sorted(path.name for path in dest.iterdir()) == sorted([bag.archive.name, bag.sidecar.name]), "default destination final set")
    return result


def scenario_symlink_dest(case_dir):
    repo = base_repo(case_dir / "repo")
    prepare_manifest(repo)
    outside = case_dir / "outside"
    outside.mkdir()
    dest = repo.path / "out"
    os.symlink(outside, dest)
    tmp_root = process_tmp(repo)
    result = dist(repo, "build", str(dest), env={"TMPDIR": str(tmp_root)})
    add_issue(result, not any(outside.iterdir()), "symlinked destination received finals")
    add_issue(result, not any(tmp_root.iterdir()), "symlinked destination leaked process temp")
    return result


def scenario_archive_symlink(case_dir):
    repo = base_repo(case_dir / "repo")
    input_head = repo.input_head()
    prepare_manifest(repo)
    dest = repo.path / "out"
    dest.mkdir()
    sentinel = case_dir / "sentinel"
    sentinel.write_bytes(b"sentinel\n")
    archive = dest / ("cnl-ckc-kb-g" + input_head[:12] + ".tar.gz")
    os.symlink(sentinel, archive)
    tmp_root = process_tmp(repo)
    result = dist(repo, "build", str(dest), env={"TMPDIR": str(tmp_root)})
    add_issue(result, sentinel.read_bytes() == b"sentinel\n", "archive symlink target changed")
    add_issue(result, archive.is_symlink(), "archive symlink replaced")
    add_issue(result, not any(tmp_root.iterdir()), "archive symlink leaked process temp")
    return result


def scenario_sidecar_directory(case_dir):
    repo = base_repo(case_dir / "repo")
    input_head = repo.input_head()
    prepare_manifest(repo)
    dest = repo.path / "out"
    dest.mkdir()
    archive_name = "cnl-ckc-kb-g" + input_head[:12] + ".tar.gz"
    sidecar = dest / (archive_name + ".sha256")
    sidecar.mkdir()
    tmp_root = process_tmp(repo)
    result = dist(repo, "build", str(dest), env={"TMPDIR": str(tmp_root)})
    add_issue(result, sidecar.is_dir(), "sidecar collision directory replaced")
    add_issue(result, not (dest / archive_name).exists(), "archive published before sidecar refusal")
    add_issue(result, not any(tmp_root.iterdir()), "sidecar collision leaked process temp")
    return result


def scenario_stage_clean_success(case_dir):
    repo = base_repo(case_dir / "repo")
    prepare_manifest(repo)
    result, dest, tmp_root = build(repo)
    bag = bag_after(result, dest)
    add_issue(result, not any(tmp_root.iterdir()), "success leaked process temp")
    if bag:
        add_issue(result, sorted(path.name for path in dest.iterdir()) == sorted([bag.archive.name, bag.sidecar.name]), "destination contains stage residue")
    return result


def scenario_stage_clean_refusal(case_dir):
    repo = base_repo(case_dir / "repo")
    prepare_manifest(repo)
    repo.remove("guidelines/g-red/rights.tsv")
    repo.commit("rights refusal")
    result, dest, tmp_root = build(repo)
    add_issue(result, not dest.exists() or not any(dest.iterdir()), "refusal left destination finals/stage")
    add_issue(result, not any(tmp_root.iterdir()), "refusal leaked process temp")
    return result


def attach_repo(path):
    repo = object.__new__(Repo)
    repo.path = pathlib.Path(path)
    return repo


def dist_meter_lines(stdout):
    return [line for line in stdout.decode("utf-8", errors="replace").splitlines() if line.startswith("goal: dist ")]


def scenario_goal_check_live(case_dir):
    repo = attach_repo(TARGET)
    result = goal(repo, "check", env={"SWIPL": "__dist_red_missing_swipl__"}, timeout=420)
    meters = dist_meter_lines(result.stdout)
    add_issue(result, len(meters) == 1, "goal check dist meter count=" + str(len(meters)))
    if meters:
        add_issue(result, meters[0].startswith("goal: dist ok ") or re.fullmatch(r"goal: dist blocked rejected=[0-9]+ contested=[0-9]+", meters[0]) is not None, "goal check dist meter grammar")
    return result


def scenario_goal_check_blocked(case_dir):
    clone_path = case_dir / "blocked-clone"
    cloned = command(["git", "clone", "-q", "--shared", str(TARGET), str(clone_path)], case_dir, GIT_ENV, timeout=180)
    if cloned.rc != 0:
        cloned.issue("blocked clone setup")
        return cloned
    repo = attach_repo(clone_path)
    guideline_dirs = sorted(path for path in (repo.path / "guidelines").iterdir() if path.is_dir())
    if not guideline_dirs:
        return Result(125, b"", b"harness: blocked clone has no guidelines\n")
    for guideline in guideline_dirs:
        rights = guideline / "rights.tsv"
        if not rights.exists():
            gid = guideline.name
            rights.write_text(rights_text([("redistributable", "Fixture blocked-check rights.", "https://example.invalid/" + gid, "2026-01-01", "Fixture note.")]), encoding="utf-8")
    guideline = guideline_dirs[0]
    manifest_path = guideline / "audit/review-manifest.tsv"
    body = manifest_path.read_text(encoding="utf-8").splitlines()[2:]
    parsed = [line.split("\t") for line in body if line]
    if len(parsed) < 2:
        return Result(125, b"", b"harness: blocked clone needs two review rows\n")
    parsed.sort(key=lambda row: row[0])
    rejected_doc, rejected_digest = parsed[0][0], parsed[0][-1]
    contested_doc, contested_digest = parsed[1][0], parsed[1][-1]
    ledger_rows = [
        (rejected_doc, "2026-01-01T00:00:00Z", rejected_digest, "rejected"),
        (contested_doc, "2026-01-01T00:00:00Z", contested_digest, "approved"),
        (contested_doc, "2026-01-01T00:00:01Z", contested_digest, "rejected"),
    ]
    ledger = LEDGER_HEADER
    for docid, when, review, verdict in sorted(ledger_rows):
        ledger += docid + "\t" + review + "\t\t" + verdict + "\tfixture\t" + when + "\tfixture blocked decision\n"
    (guideline / "audit/adjudication.tsv").write_text(ledger, encoding="utf-8")
    repo.commit("blocked distribution corpus")
    writer = goal(repo, "release-manifest", timeout=180)
    writer_ok = writer.rc == 0 and (repo.path / "release-manifest.tsv").is_file()
    if writer_ok:
        repo.commit("blocked release manifest")
    result = goal(repo, "check", env={"SWIPL": "__dist_red_missing_swipl__"}, timeout=420)
    if not writer_ok:
        result.issue("blocked setup release-manifest rc=" + str(writer.rc))
    meters = dist_meter_lines(result.stdout)
    add_issue(result, meters == ["goal: dist blocked rejected=1 contested=1"], "blocked meter=" + repr(meters))
    return result


SCENARIOS = {
    "input_head": scenario_input_head,
    "reconstructable_source": scenario_reconstructable_source,
    "reconstructable_empty_url": scenario_reconstructable_empty_url,
    "rights_missing": scenario_rights_missing,
    "rights_header": scenario_rights_header,
    "rights_rows": scenario_rights_rows,
    "rights_fields": scenario_rights_fields,
    "rights_profile": scenario_rights_profile,
    "rights_statement": scenario_rights_statement,
    "rights_url": scenario_rights_url,
    "rights_retrieved": scenario_rights_retrieved,
    "rights_control": scenario_rights_control,
    "rights_utf8": scenario_rights_utf8,
    "rights_second_row_invalid": scenario_rights_second_row_invalid,
    "rights_first_row_operative": scenario_rights_first_row_operative,
    "stray_root": scenario_stray_root,
    "no_guidelines": scenario_no_guidelines,
    "rights_only": scenario_rights_only,
    "committed_state": scenario_committed_state,
    "manifest_order": scenario_manifest_order,
    "runtime_meta": scenario_runtime_meta,
    "manifest_source_drift": scenario_manifest_source_drift,
    "manifest_tamper": scenario_manifest_tamper,
    "writer_idempotent": scenario_writer_idempotent,
    "restricted_labels": scenario_restricted_labels,
    "rejected_order": scenario_rejected_order,
    "contested": scenario_contested,
    "label_classes": scenario_label_classes,
    "exec_mode": scenario_exec_mode,
    "output_collision": scenario_output_collision,
    "gzip_header": scenario_gzip_header,
    "sidecar": scenario_sidecar,
    "symlink_member": scenario_symlink_member,
    "gitlink_member": scenario_gitlink_member,
    "member_precedence": scenario_member_precedence,
    "success_meter": scenario_success_meter,
    "byte_determinism": scenario_byte_determinism,
    "tar_fields": scenario_tar_fields,
    "longname": scenario_longname,
    "bagit_closure": scenario_bagit_closure,
    "profile_member_set": scenario_profile_member_set,
    "bag_layout": scenario_bag_layout,
    "exclusion": scenario_exclusion,
    "consumer_copy": scenario_consumer_copy,
    "newline_path": scenario_newline_path,
    "invalid_utf8_path": scenario_invalid_utf8_path,
    "backslash_path": scenario_backslash_path,
    "space_path": scenario_space_path,
    "sha256_command": scenario_sha256_command,
    "checksum_tamper": scenario_checksum_tamper,
    "tag_self_exclusion": scenario_tag_self_exclusion,
    "release_member_digests": scenario_release_member_digests,
    "usage_missing": scenario_usage_missing,
    "usage_mode": scenario_usage_mode,
    "default_dest": scenario_default_dest,
    "symlink_dest": scenario_symlink_dest,
    "archive_symlink": scenario_archive_symlink,
    "sidecar_directory": scenario_sidecar_directory,
    "stage_clean_success": scenario_stage_clean_success,
    "stage_clean_refusal": scenario_stage_clean_refusal,
    "goal_check_live": scenario_goal_check_live,
    "goal_check_blocked": scenario_goal_check_blocked,
}


def read_cases():
    with CASES_PATH.open("r", encoding="utf-8", newline="") as stream:
        rows = list(csv.DictReader(stream, delimiter="\t"))
    required = {"id", "family", "scenario", "expected_rc", "expected_stderr_prefix"}
    if not rows or set(rows[0]) != required:
        raise SystemExit("dist-red: invalid cases.tsv header or empty case set")
    ids = [row["id"] for row in rows]
    if len(ids) != len(set(ids)):
        raise SystemExit("dist-red: duplicate case id")
    return rows


def detail_text(result):
    parts = []
    if result.stderr:
        parts.append(result.stderr.splitlines()[0].decode("utf-8", errors="backslashreplace"))
    if result.issues:
        parts.append("issue=" + result.issues[0])
    if not parts and result.stdout:
        parts.append(result.stdout.splitlines()[0].decode("utf-8", errors="backslashreplace"))
    if not parts:
        parts.append("-")
    return " | ".join(parts).replace("\t", "\\t")[:240]


def evaluate(row, result):
    expected_rc = int(row["expected_rc"])
    prefix = row["expected_stderr_prefix"]
    ok = result.rc == expected_rc
    if prefix == "-":
        ok = ok and result.stderr == b""
    else:
        ok = ok and result.stderr.startswith(prefix.encode("utf-8"))
    if expected_rc == 1:
        ok = ok and result.stdout == b""
        ok = ok and result.stderr.endswith(b"\n") and result.stderr.count(b"\n") == 1
    ok = ok and not result.issues
    return ok


def main():
    rows = read_cases()
    passed = 0
    print("case\tfamily\texpected\tactual\tverdict\tdetail")
    for row in rows:
        scenario = SCENARIOS.get(row["scenario"])
        if scenario is None:
            result = Result(125, b"", ("harness: missing scenario " + row["scenario"] + "\n").encode())
        else:
            with tempfile.TemporaryDirectory(prefix="cnl-ckc-dist-red-") as scratch:
                try:
                    result = scenario(pathlib.Path(scratch))
                except SetupFailure as exc:
                    result = exc.result
                except Exception as exc:
                    result = Result(125, b"", ("harness: " + type(exc).__name__ + ": " + str(exc) + "\n").encode("utf-8", errors="replace"))
        ok = evaluate(row, result)
        if ok:
            passed += 1
        expected = "rc=" + row["expected_rc"] + " stderr^=" + row["expected_stderr_prefix"]
        actual = "rc=" + str(result.rc)
        print("\t".join((row["id"], row["family"], expected, actual, "PASS" if ok else "RED", detail_text(result))))
    total = len(rows)
    red = total - passed
    print("dist-red: pass=" + str(passed) + " red=" + str(red) + " total=" + str(total))
    return 0 if red == 0 else 1


raise SystemExit(main())
PY
