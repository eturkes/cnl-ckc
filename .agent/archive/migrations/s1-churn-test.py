#!/usr/bin/env python3
"""Independent S1 D10 oracle tests; Python 3.13 stdlib, no compiler calls."""
import contextlib
import hashlib
import importlib.util
import io
import os
from pathlib import Path
import random
import re
import subprocess
import sys
import tempfile
import time
import unittest
from unittest.mock import patch

sys.dont_write_bytecode = True
spec = importlib.util.spec_from_file_location("s1_churn", Path(__file__).with_name("s1-churn.py"))
s1 = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = s1
spec.loader.exec_module(s1)
BASE = "3639f733"
INDICATORS = (
    ("schema_version", 1), ("document", 4), ("entity", 4), ("cardinality", 5),
    ("event", 3), ("arg", 4), ("pp", 4), ("property", 4), ("operator", 3),
    ("interval", 6), ("recurrence", 4), ("frequency", 5), ("order", 4),
    ("recurrence_window", 7), ("range", 3),
)


def declarations(indicators):
    return b"".join((f":- multifile(guideline_{name}/{arity}).\n"
                     f":- discontiguous(guideline_{name}/{arity}).\n").encode() for name, arity in indicators)


OLD = declarations(INDICATORS)
ADDED = (b":- multifile(guideline_grade/5).\n"
         b":- discontiguous(guideline_grade/5).\n"
         b":- multifile(guideline_preference/4).\n"
         b":- discontiguous(guideline_preference/4).\n")
TABLE = b"# format: kind\tlemma\tvalue\nunit\tday\tday\n"


def sha(data):
    return hashlib.sha256(data).hexdigest().encode()


def doc(body=b"guideline_event(actual,e,wait).\n", header=OLD):
    return (b"% d.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.\n" + header +
            b"guideline_schema_version(1).\nguideline_document(d,ace_sha256(" + b"a" * 64 +
            b"),ulex(none),vocabulary(none)).\n% S1: x\n" + body)


def query():
    return (b"% q compiled from ACE question by ace_to_pl question mode; do not edit.\n"
            b"'$guideline_query'(v1,q,ace_sha256(" + b"a" * 64 + b"),ulex(none),vocabulary(none)).\n"
            b"% Q1: x?\n'$guideline_query_projection'(goal(guideline_event(actual,e,wait)),answers([])).\n")


def atom(data):
    return b"'" + data + b"'" if data[:1].isdigit() else data


def answer(q):
    return (b"% q answered against the loaded composition by ckc queries; do not edit.\n"
            b"'$guideline_answers'(v1,q,query_sha256(" + atom(sha(q)) + b"),result(yes)).\n")


def trace(q, a):
    return (b"% q traced against the loaded composition by ckc queries; do not edit.\n"
            b"'$guideline_traces'(v1,q,query_sha256(" + atom(sha(q)) + b"),answers_sha256(" + atom(sha(a)) +
            b"),result(yes(proved([clause(sentence(d,1),clause_sha256(" + b"c" * 64 + b"),[])])))).\n")


def bundle(fields):
    return sha(b"bundle v2 " + fields[0] + b"\nace " + fields[1] + b"\ncoverage " + fields[2] +
               b"\npayload " + fields[3] + b"\nclauses " + fields[4] + b"\n")


def rows_blob():
    return b"# case\tprobe\tmode\trc\tstderr_lines\targs\nx\ttarget\tcheck\t2\t1\tinput.pl\n"


class S1OracleContract(unittest.TestCase):
    def test_exact_four_lines_and_150_bytes(self):
        self.assertEqual(len(ADDED), 150)
        actual = s1.document_transform(doc())
        self.assertEqual(actual, doc(header=OLD + ADDED))
        self.assertEqual(actual.count(b":- multifile("), 17)
        self.assertEqual(actual.count(b":- discontiguous("), 17)
        self.assertEqual(s1.document_transform(actual), actual)

    def test_semantic_tail_property(self):
        rng = random.Random(0x51)
        for n in (0, 1, 2, 31, 128, 1024):
            for _ in range(8):
                payload = bytes(rng.choices(range(256), k=n))
                old = doc(payload)
                new = s1.document_transform(old)
                self.assertEqual(new[new.index(b"% S1:"):], old[old.index(b"% S1:"):])
                self.assertEqual(new.replace(OLD + ADDED, OLD, 1), old)

    def test_every_prefix_byte_mutation_preserves_bad_block(self):
        for at in range(len(OLD)):
            broken = OLD[:at] + b"~" + OLD[at + 1:]
            original = doc(header=broken)
            self.assertEqual(s1.document_transform(original), original, f"byte {at}")
        for count in range(15):
            original = doc(header=declarations(INDICATORS[:count]))
            self.assertEqual(s1.document_transform(original), original, f"pairs {count}")
        pair_only = declarations((("range", 3),))
        self.assertEqual(s1.document_transform(pair_only), pair_only)

    def test_first_line_defect_does_not_hide_canonical_block(self):
        old = doc().replace(b"% d.pl", b"broken d.pl", 1)
        self.assertEqual(s1.document_transform(old), old.replace(OLD, OLD + ADDED, 1))
        eof = b"% d.pl\n" + OLD
        self.assertEqual(s1.document_transform(eof), eof + ADDED)

    def test_r9_offsets_before_at_and_after_block(self):
        for line, wanted in ((1, 1), (31, 31), (32, 36), (33, 37), (34, 38), (35, 39)):
            base = {"tests/v1/cases.tsv": s1.Blob(rows_blob()), "tests/v1/x/input.pl.in": s1.Blob(doc()),
                    "tests/v1/x/expect/target.stderr": s1.Blob(f"ace_to_pl_error(check_load,noncanonical({line},7)).\n".encode())}
            o = s1.Oracle(base, {}).build()
            self.assertEqual(o.expected["tests/v1/x/expect/target.stderr"].data,
                             f"ace_to_pl_error(check_load,noncanonical({wanted},7)).\n".encode())
        broken = doc(header=OLD.replace(b"guideline_event/3", b"guideline_event/4", 1))
        base["tests/v1/x/input.pl.in"] = s1.Blob(broken)
        self.assertEqual(s1.Oracle(base, {}).build().expected["tests/v1/x/expect/target.stderr"], base["tests/v1/x/expect/target.stderr"])

    def test_document_as_answers_term_count_plus_four(self):
        old = doc()
        n = sum(line.endswith(b".") and not line.startswith(b"%") for line in old.splitlines())
        base = {"tests/v1/cases.tsv": s1.Blob(b"x\tcount\ttrace\t2\t1\tm q a.pl\n"),
                "tests/v1/x/a.pl.in": s1.Blob(old),
                "tests/v1/x/expect/count.stderr": s1.Blob(f"ace_to_pl_error(check_load,answers_file(term_count({n}))).\n".encode())}
        result = s1.Oracle(base, {}).build().expected
        self.assertEqual(result["tests/v1/x/expect/count.stderr"].data,
                         f"ace_to_pl_error(check_load,answers_file(term_count({n + 4}))).\n".encode())
        base["tests/v1/x/a.pl.in"] = s1.Blob(answer(query()))
        self.assertEqual(s1.Oracle(base, {}).build().expected["tests/v1/x/expect/count.stderr"], base["tests/v1/x/expect/count.stderr"])

    def test_certify_mutation_indices_follow_target(self):
        p = "guidelines/g/pl/d.pl"
        row = b"reorder\tdoc\t" + p.encode() + b"\tswap-lines\t34\t35\t1\tclauses(1)\\n\n"
        base = {p: s1.Blob(doc()), "tests/certify/cases.tsv": s1.Blob(row)}
        result = s1.Oracle(base, {}).build().expected
        self.assertEqual(result["tests/certify/cases.tsv"].data, row.replace(b"\t34\t35\t", b"\t38\t39\t"))

    def test_query_answers_traces_proofs_and_align_stay_exact(self):
        q = query()
        a = answer(q)
        base = {"guidelines/g/pl/d.pl": s1.Blob(doc()), "guidelines/g/queries/pl/q.pl": s1.Blob(q),
                "guidelines/g/queries/answers/q.pl": s1.Blob(a), "guidelines/g/queries/traces/q.pl": s1.Blob(trace(q, a)),
                "tests/green/p.proof.expect": s1.Blob(b"'$guideline_proof'(d,1,variant(1),witness([]),prove([guideline_event(actual,e,wait)])).\n"),
                "guidelines/g/align/d.tsv": s1.Blob(b"group\tside\tstart\tspan\n0\tace\t0\t1\n"),
                "tests/align/x/table.tsv.in": s1.Blob(b"bad\t" + sha(doc()) + b"\n"),
                "tests/adjudication/x/manifest.tsv": s1.Blob(b"bad\t" + sha(doc()) + b"\n")}
        out = s1.Oracle(base, {}).build().expected
        for path, blob in base.items():
            if path != "guidelines/g/pl/d.pl":
                self.assertEqual(out[path], blob, path)

    def test_review_bundle_unchanged_and_ui_header_count_only(self):
        old = doc()
        case = "tests/ui/green/example/"
        p = case + "tree/guidelines/g/pl/d.pl"
        fields = [b"d", b"a" * 64, b"b" * 64, b"c" * 64, sha(s1.semantic_stream(old))]
        fields.append(bundle(fields))
        manifest = case + "tree/guidelines/g/audit/review-manifest.tsv"
        page = case + "golden/g/g/doc/d.html"
        caption = f"<summary>Compiled Prolog ({len(old.splitlines())} lines)</summary>".encode()
        new_caption = f"<summary>Compiled Prolog ({len(old.splitlines()) + 4} lines)</summary>".encode()
        page_bytes = caption + b"<pre>" + s1.ui_escape(old) + b'</pre><input type="hidden" name="review_sha256" value="' + fields[5] + b'">\n'
        base = {p: s1.Blob(old), manifest: s1.Blob(b"\t".join(fields) + b"\n"), page: s1.Blob(page_bytes),
                case + "assertions.tsv": s1.Blob(b"g/g/doc/d.html\tcontains\t" + caption.hex().encode() + b"\n")}
        o = s1.Oracle(base, {}).build()
        self.assertEqual(o.expected[manifest], base[manifest])
        self.assertEqual(s1.semantic_stream(o.expected[p].data), s1.semantic_stream(old))
        self.assertEqual(o.expected[page].data, page_bytes.replace(caption, new_caption).replace(s1.ui_escape(old), s1.ui_escape(doc(header=OLD + ADDED))))
        self.assertEqual(bytes.fromhex(o.expected[case + "assertions.tsv"].data.split(b"\t")[2].decode()), new_caption)

    def test_raw_hash_pin_rebound(self):
        old = doc()
        p = "tests/v1/x/input.pl.in"
        pin = "tests/check/raw-pl.expect"
        o = s1.Oracle({p: s1.Blob(old), pin: s1.Blob(sha(old) + b"\n")}, {}).build()
        self.assertEqual(o.expected[pin].data, sha(doc(header=OLD + ADDED)) + b"\n")

    def test_template_expansion_pin_and_offset(self):
        body = doc() + b"@@run 1 3 guideline_event(a,e@i@,v@j@).\n"
        raw = b"@@sha256 " + b"0" * 64 + b"\n" + body
        expanded = s1.expand_template(raw)
        raw = b"@@sha256 " + sha(expanded) + b"\n" + body
        path = "tests/v1/x/input.pl.tpl.in"
        base = {path: s1.Blob(raw), "tests/v1/cases.tsv": s1.Blob(rows_blob()),
                "tests/v1/x/expect/target.stderr": s1.Blob(b"ace_to_pl_error(check_load,noncanonical(35,1)).\n")}
        result = s1.Oracle(base, {}).build().expected
        new = result[path].data
        self.assertEqual(new.split(b"\n", 1)[0], b"@@sha256 " + sha(s1.expand_template(new)))
        self.assertEqual(s1.expand_template(new), expanded.replace(OLD, OLD + ADDED, 1))
        self.assertIn(b"noncanonical(39,1)", result["tests/v1/x/expect/target.stderr"].data)

    def test_no_s1_rows_or_literals_in_pure_base(self):
        for kind in (b"strength", b"evidence", b"preference", b"alternative"):
            bad = TABLE + kind + b"\tx\tx\n"
            with self.subTest(kind=kind), self.assertRaises(s1.InputError):
                s1.Oracle({"guidelines/g/vocabulary.tsv": s1.Blob(bad)}, {}).build()
        for body in (b"guideline_grade(actual,r,strength,strong,l).\n",
                     b"guideline_event(actual,e,wait) :- guideline_preference(actual,e,p,a).\n",
                     b"guideline_event(actual,e,wait) :- \\+ (guideline_grade(actual,r,evidence,high,l)).\n"):
            with self.subTest(body=body), self.assertRaises(s1.InputError):
                s1.Oracle({"guidelines/g/pl/d.pl": s1.Blob(doc(body))}, {}).build()
        safe = doc(b"guideline_property(actual,p,'guideline_grade(a,b,c,d,e)',pos).\n")
        self.assertIsNone(s1.new_semantic_call(safe))
        self.assertIsNone(s1.new_semantic_call(OLD + ADDED))

    def test_loose_ui_and_malformed_fixture_bytes_preserved(self):
        loose = b"guideline_schema_version(1).\nguideline_document(d,ace_sha256(fixture),ulex(none)).\nfixture_fact(d).\n"
        p = "tests/ui/green/basic/tree/guidelines/g/pl/d.pl"
        bad = doc(header=OLD.replace(b"guideline_range/3", b"guideline_range/4", 1))
        base = {p: s1.Blob(loose), "tests/v1/x/input.pl.in": s1.Blob(bad)}
        self.assertEqual(s1.Oracle(base, {}).build().expected, base)

    def test_archive_uses_oracle_history_when_base_object_is_absent(self):
        owner = Path(__file__).resolve().parents[3]
        archive = owner / ".scratch" / "archive-without-history"
        commit, oid, payload = b"a" * 40, b"b" * 40, b"A test.\n"
        calls = []
        def fake_git(root, *args, data=None):
            calls.append((root, args))
            if root == archive:
                raise s1.InputError("base object absent")
            self.assertEqual(root, owner)
            if args[0] == "rev-parse":
                return commit + b"\n"
            if args[0] == "ls-tree":
                return b"100644 blob " + oid + b"\ttests/red/a.ace\0"
            self.assertEqual(args, ("cat-file", "--batch"))
            self.assertEqual(data, oid + b"\n")
            return oid + b" blob " + str(len(payload)).encode() + b"\n" + payload + b"\n"
        with patch.object(s1, "git", side_effect=fake_git):
            result = s1.base_blobs(archive, BASE)
        self.assertEqual(result, {"tests/red/a.ace": s1.Blob(payload)})
        self.assertEqual(len(calls), 4)

    def test_scope_excludes_release_and_code(self):
        self.assertFalse(s1.scope_path("release-manifest.tsv"))
        self.assertFalse(s1.scope_path("rust/ckc/tests/s1_compiler.rs"))
        self.assertFalse(s1.scope_path(".agent/archive/migrations/s1-revise.py"))
        self.assertTrue(s1.scope_path("tests/green/a.expect"))
        self.assertTrue(s1.scope_path(s1.KANI))

    def test_kani_doc_and_bounds_add_150_only(self):
        root = Path(__file__).resolve().parents[3]
        source = s1.git(root, "show", BASE + ":" + s1.KANI)
        o = s1.Oracle({s1.KANI: s1.Blob(source)}, {}).build()
        result = o.expected[s1.KANI].data
        before, after = s1.embedded_artifacts(source), s1.embedded_artifacts(result)
        for old, new in zip(before, after, strict=True):
            if s1.artifact_kind(old) == "doc":
                self.assertEqual(len(new), len(old) + 150)
                self.assertEqual(new, old.replace(OLD, OLD + ADDED, 1))
                self.assertIn(f"#[kani::unwind({len(new) + 1})]".encode(), result)
            else:
                self.assertEqual(new, old)
        self.assertIn(b"query[264] =", result)

    def test_new_coverage_requires_fresh_item_and_path(self):
        manifest = "tests/check-mutants/cases.tsv"
        old = b"old\tsection\t1\n"
        new = b"fresh\tsection\t1\n"
        add = {"family": "tests/check-mutants", "item": "fresh", "path": "tests/check-mutants/fresh.expect", "mode": "new",
               "edits": [{"old": b"", "new": b"new error\n", "count": 1}], "reason": "new: S1 coverage"}
        row = dict(add, path=manifest, mode="edit", edits=[{"old": old, "new": old + new, "count": 1}])
        o = s1.Oracle({manifest: s1.Blob(old)}, {}, [add, row]).build()
        self.assertEqual(o.expected[manifest].data, old + new)
        bad = dict(add, item="old", path="tests/check-mutants/old.extra")
        with self.assertRaises(s1.InputError):
            s1.Oracle({manifest: s1.Blob(old)}, {}, [bad]).build()

    def test_tsv_bytes_and_count_guard(self):
        raw = bytes(range(256))
        self.assertEqual(s1.literal_bytes(s1.escaped_bytes(raw)), raw)
        text = b"# header\ntests/green\tx\ttests/green/x.expect\t@new\tvalue\\n\t1\tnew: case\n"
        rules = s1.load_rulings(text)
        self.assertEqual(rules[0]["edits"][0]["new"], b"value\n")
        with self.assertRaises(s1.InputError):
            s1.load_rulings(text.replace(b"\t1\t", b"\t2\t"))

    def test_self_test_eight_plants(self):
        base = {"tests/v1/cases.tsv": s1.Blob(rows_blob()), "tests/v1/x/input.pl.in": s1.Blob(doc()),
                "tests/v1/x/expect/target.stderr": s1.Blob(b"ace_to_pl_error(check_load,noncanonical(35,1)).\n")}
        o = s1.Oracle(base, {}).build()
        evidence = s1.self_test(o)
        self.assertEqual(len(evidence), 8)
        with patch.object(o, "compare", return_value=[]), self.assertRaises(AssertionError):
            s1.self_test(o)

    def test_self_test_failure_returns_two(self):
        base = {"tests/v1/x/input.pl.in": s1.Blob(doc())}
        root = Path(__file__).resolve().parents[3]
        scratch = root / ".scratch"
        scratch.mkdir(exist_ok=True)
        with tempfile.TemporaryDirectory(prefix="s1-status-", dir=scratch) as temporary:
            output = io.StringIO()
            with patch.object(s1, "base_blobs", return_value=base), patch.object(s1, "self_test", side_effect=AssertionError("plant")), contextlib.redirect_stdout(output):
                rc = s1.main(["s1-churn.py", BASE, temporary])
        self.assertEqual(rc, 2)
        self.assertEqual(output.getvalue(), "s1-churn: self-test failed: plant\n")


def integration(base_commit, root):
    root = Path(root).resolve()
    script = Path(__file__).with_name("s1-churn.py").resolve()
    start = time.monotonic()
    base = s1.base_blobs(root, base_commit)
    policy = root / s1.DISPOSITIONS
    dispositions = s1.load_dispositions(policy.read_bytes()) if policy.exists() else {}
    rules = root / s1.RULINGS
    rulings = s1.load_rulings(rules.read_bytes()) if rules.exists() else []
    oracle = s1.Oracle(base, dispositions, rulings)
    oracle.validate_dispositions()
    oracle.build()
    print(f"model: {len(base)} base artifacts; {len(oracle.expected)} predicted artifacts; {dict(oracle.counts)}", flush=True)
    print(f"derived: {len(oracle.hashes)} digest bindings; {len(oracle.offsets)} offset pins; "
          f"review bundles unchanged; {len(oracle.commit_ids)} fixture identities", flush=True)
    if oracle.compare(oracle.expected):
        raise AssertionError("synthetic model rejected")
    s1.self_test(oracle)
    scratch = Path(__file__).resolve().parents[3] / ".scratch"
    scratch.mkdir(exist_ok=True)
    with tempfile.TemporaryDirectory(prefix="s1-churn-perfect-", dir=scratch) as temporary:
        target = Path(temporary)

        def materialize(path, blob):
            file = target / path
            if file.exists() or file.is_symlink():
                file.unlink()
            if blob is not None:
                file.parent.mkdir(parents=True, exist_ok=True)
                if blob.mode == "120000":
                    file.symlink_to(os.fsdecode(blob.data))
                else:
                    file.write_bytes(blob.data)
                    file.chmod(0o755 if blob.mode == "100755" else 0o644)

        for path, blob in oracle.base.items():
            materialize(path, blob)
        for path in (s1.DISPOSITIONS, s1.RULINGS):
            if (root / path).exists():
                materialize(path, s1.Blob((root / path).read_bytes()))
        env = dict(os.environ, GIT_CONFIG_GLOBAL="/dev/null", GIT_CONFIG_SYSTEM="/dev/null", GIT_CEILING_DIRECTORIES=str(scratch))
        env["GIT_ALTERNATE_OBJECT_DIRECTORIES"] = s1.git(root, "rev-parse", "--path-format=absolute", "--git-path", "objects").strip().decode()
        subprocess.run(["git", "-C", temporary, "init", "-q", "-b", "oracle"], env=env, check=True)

        def run(name, expected_rc):
            subprocess.run(["git", "-C", temporary, "add", "-A"], env=env, check=True)
            proc = subprocess.run([sys.executable, "-I", str(script), base_commit, temporary], env=env, capture_output=True)
            lines = proc.stdout.decode().splitlines()
            first = lines[0] if lines else proc.stderr.decode().strip()
            print(f"{name}: rc={proc.returncode}; {first}", flush=True)
            if proc.returncode != expected_rc:
                raise AssertionError(f"{name}: rc {proc.returncode} != {expected_rc}; {proc.stderr!r}")
            return lines

        run("unmigrated base CLI", 1)
        for path in oracle.base.keys() - oracle.expected.keys():
            materialize(path, None)
        for path, blob in oracle.expected.items():
            materialize(path, blob)
        run("synthetic CLI", 0)
        for name, path, blob in s1.self_test_cases(oracle):
            materialize(path, blob)
            lines = run("plant " + name, 1)
            if not any(line.startswith("s1-churn: " + path + ":") for line in lines):
                raise AssertionError(f"{name}: failure did not identify planted path")
            materialize(path, oracle.expected.get(path))
        policy = (root / s1.RULINGS).read_bytes() if (root / s1.RULINGS).exists() else None
        prefix = policy if policy is not None else b"# family\titem\tpath\told\tnew\tcount\treason\n"
        for name, ruling, expected_error in s1.self_test_rulings(oracle):
            edit = ruling["edits"][0]
            old = "@new" if ruling["mode"] == "new" else s1.escaped_bytes(edit["old"])
            fields = [ruling["family"], ruling["item"], ruling["path"], old, s1.escaped_bytes(edit["new"]),
                      str(edit["count"]), ruling["reason"]]
            materialize(s1.RULINGS, s1.Blob(prefix + ("\t".join(fields) + "\n").encode()))
            path = ruling["path"]
            if ruling["mode"] == "new":
                planted = edit["new"]
            else:
                planted = oracle.expected[path].data.replace(edit["old"], edit["new"], edit["count"])
            materialize(path, s1.Blob(planted))
            lines = run("plant " + name, 1)
            if not any(expected_error in line for line in lines):
                raise AssertionError(f"{name}: wrong refusal: {lines[:1]}")
            materialize(path, oracle.expected.get(path))
            materialize(s1.RULINGS, s1.Blob(policy) if policy is not None else None)
    print(f"integration: ok; eight CLI plants rejected; {time.monotonic() - start:.3f} s", flush=True)


if __name__ == "__main__":
    if len(sys.argv) == 4 and sys.argv[1] == "--integration":
        integration(sys.argv[2], sys.argv[3])
    else:
        unittest.main(verbosity=2)
