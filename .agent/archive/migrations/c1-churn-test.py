#!/usr/bin/env python3
"""D13 oracle tests; run with python3 -I. No production imports or commands."""

import contextlib
import hashlib
import io
import importlib.util
import os
import random
import re
import subprocess
import sys
import tempfile
import time
import unittest
from pathlib import Path
from unittest.mock import patch

sys.dont_write_bytecode = True
spec = importlib.util.spec_from_file_location("c1_churn", Path(__file__).with_name("c1-churn.py"))
c1 = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = c1
spec.loader.exec_module(c1)

OLD_INDICATORS = (
    ("schema_version", 1), ("document", 3), ("entity", 4),
    ("cardinality", 5), ("event", 3), ("arg", 4), ("pp", 4),
    ("property", 4), ("operator", 3),
)
EXTRA = (("interval", 6), ("recurrence", 4), ("frequency", 5),
         ("order", 4), ("recurrence_window", 7), ("range", 3))
OLD_TABLE = (b"# format: kind\tlemma\tvalue\n"
             b"# kind: unit (value: second|minute|hour|day|week|month|year) | relation (value: duration|within|after|before) | spacing (value: frame noun lemma)\n")


def digest(data):
    return hashlib.sha256(data).hexdigest().encode()


def term(value):
    return b"'" + value + b"'" if value[:1].isdigit() else value


def declarations(version):
    names = list(OLD_INDICATORS)
    if version != 1:
        names[1] = ("document", 4)
        names += EXTRA[:2] if version == 2 else EXTRA
    return b"".join((f":- multifile(guideline_{name}/{arity}).\n"
                     f":- discontiguous(guideline_{name}/{arity}).\n").encode() for name, arity in names)


def doc(version=1, table=b"a" * 64, body=b"guideline_event(actual,e,wait).\n"):
    wrapper = b",temporal(sha256(" + term(table) + b"))" if version > 1 else b""
    return (b"% d.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.\n" +
            declarations(version) + f"guideline_schema_version({version}).\n".encode() +
            b"guideline_document(d,ace_sha256(" + b"a" * 64 + b"),ulex(none)" + wrapper +
            b").\n% S1: x\n" + body)


def query(version=1, table=b"a" * 64):
    wrapper = b",temporal(sha256(" + term(table) + b"))" if version > 1 else b""
    return (b"% q compiled from ACE question by ace_to_pl question mode; do not edit.\n" +
            f"'$guideline_query'(v{version},q,ace_sha256(".encode() + b"b" * 64 +
            b"),ulex(none)" + wrapper + b").\n% Q1: x?\n" +
            b"'$guideline_query_projection'(goal(guideline_event(actual,e,wait)),answers([])).\n")


def answer(q):
    return (b"% q answered against the loaded composition by ckc queries; do not edit.\n" +
            b"'$guideline_answers'(v1,q,query_sha256(" + term(digest(q)) + b"),result(yes)).\n")


def trace(q, a):
    return (b"% q traced against the loaded composition by ckc queries; do not edit.\n" +
            b"'$guideline_traces'(v1,q,query_sha256(" + term(digest(q)) + b"),answers_sha256(" + term(digest(a)) +
            b"),result(yes(proved([clause(sentence(d,1),clause_sha256(" + b"c" * 64 + b"),[])])))).\n")


def row(family, item, status):
    fields = (family, item, "", "", "", "", status, "test law", "test evidence")
    return c1.Disposition(family, item, status, fields)


def edit_bytes(old, new):
    return {"old": old, "new": new, "count": 1}


def review_digest(fields):
    return digest(b"bundle v2 " + fields[0] + b"\nace " + fields[1] + b"\ncoverage " + fields[2] +
                  b"\npayload " + fields[3] + b"\nclauses " + fields[4] + b"\n")


class OracleContract(unittest.TestCase):
    def test_document_header_domain_and_semantic_property(self):
        rng = random.Random(0xC1)
        for version in (1, 2, 3):
            for _ in range(24):
                noun = bytes(rng.choices(b"abcdefghijklmnopqrstuvwxyz", k=rng.randrange(1, 25)))
                body = b"guideline_entity(actual,r," + noun + b",countable).\n"
                old = doc(version, body=body)
                expected_head = old.split(b"\n", 1)[0] + b"\n" + declarations(3) + b"guideline_schema_version(1).\n"
                transformed = c1.document_transform(old)
                self.assertTrue(transformed.startswith(expected_head))
                self.assertEqual(transformed[transformed.index(b"% S1:"):], b"% S1: x\n" + body)
                self.assertIn(b",vocabulary(none)).\n" if version == 1 else b",vocabulary(sha256(" + b"a" * 64 + b"))).\n", transformed)
                self.assertEqual(transformed.count(b":- multifile("), 15)
                self.assertEqual(transformed.count(b":- discontiguous("), 15)

    def test_query_envelope_and_projection_property(self):
        for version in (1, 2, 3):
            for h in (b"a" * 64, b"1" * 64):
                old = query(version, h)
                actual = c1.query_transform(old)
                self.assertEqual(actual.split(b"\n")[2:], old.split(b"\n")[2:])
                self.assertIn(b"'$guideline_query'(v1,q,", actual)
                self.assertIn(b"vocabulary(none)" if version == 1 else b"vocabulary(sha256(" + term(h) + b"))", actual)

    def test_table_rows_and_header(self):
        for rows in (b"unit\tday\tday\n", b"range\tto\tminimum\n#bad\n", b"unit\tday\tday", b"\xff\n", b""):
            old = OLD_TABLE + rows
            self.assertEqual(c1.table_transform(old), b"# format: kind\tlemma\tvalue\n" + rows)
        self.assertEqual(c1.renamed("tests/red/temporal_load--x.temporal.tsv"), "tests/red/vocabulary_load--x.vocabulary.tsv")
        self.assertEqual(c1.renamed("tests/check-mutants/foo.temporal.tsv.in"), "tests/check-mutants/foo.vocabulary.tsv.in")
        self.assertEqual(c1.table_transform(b"bad header\nunit\tday\tday\n"), b"bad header\nunit\tday\tday\n")

    def test_custody_dependency_chain(self):
        table = OLD_TABLE + b"unit\tday\tday\n"
        q = query(3, digest(table))
        a, t = answer(q), trace(q, answer(q))
        base = {"guidelines/g/temporal.tsv": c1.Blob(table),
                "guidelines/g/queries/pl/q.pl": c1.Blob(q),
                "guidelines/g/queries/answers/q.pl": c1.Blob(a),
                "guidelines/g/queries/traces/q.pl": c1.Blob(t)}
        expected = c1.Oracle(base, {}).build().expected
        new_table = b"# format: kind\tlemma\tvalue\nunit\tday\tday\n"
        nq = c1.query_transform(q).replace(term(digest(table)), term(digest(new_table)))
        self.assertEqual(expected["guidelines/g/queries/pl/q.pl"].data, nq)
        self.assertEqual(expected["guidelines/g/queries/answers/q.pl"].data, answer(nq))
        self.assertEqual(expected["guidelines/g/queries/traces/q.pl"].data, trace(nq, answer(nq)))

    def test_clause_hash_and_result_are_never_rebound(self):
        q = query()
        original = trace(q, answer(q))
        mapping = {b"c" * 64: b"d" * 64}
        self.assertEqual(c1.rebind_artifact(original, mapping), original)
        semantic = doc(body=b"guideline_event(actual,e,'" + b"c" * 64 + b"').\n")
        self.assertEqual(c1.rebind_artifact(semantic, mapping), semantic)
        nested = answer(q).replace(b"result(yes)", b"result(solutions([sol([query_sha256(" + b"c" * 64 + b")])]))")
        self.assertEqual(c1.rebind_artifact(nested, mapping), nested)
        named = doc().replace(b"guideline_document(d,", b"guideline_document(" + b"c" * 64 + b",")
        self.assertEqual(c1.rebind_artifact(named, mapping), named)
        malformed_quote = query().replace(b"ace_sha256(" + b"b" * 64 + b")", b"ace_sha256('" + b"b" * 64 + b"')")
        self.assertEqual(c1.rebind_artifact(malformed_quote, {}), malformed_quote)

    def test_malformed_metadata_rebinds_without_healing_its_defect(self):
        q = query()
        nq = c1.query_transform(q)
        a, na = answer(q), answer(nq)
        cases = {
            "two-records.answers.pl.in": (a + a, na + na),
            "payload.answers.pl.in": (a.replace(b",result(", b",payload("), na.replace(b",result(", b",payload(")),
            "layout.answers.pl.in": (a.replace(b",result(", b", result("), na.replace(b",result(", b", result(")),
            "hash-wrapper.traces.pl.in": (trace(q, a).replace(b"answers_sha256(", b"answer_hash("),
                                           trace(nq, na).replace(b"answers_sha256(", b"answer_hash(")),
            "payload.traces.pl.in": (trace(q, a).replace(b",result(", b",payload("),
                                      trace(nq, na).replace(b",result(", b",payload(")),
        }
        base = {"tests/v1/x/query.pl.in": c1.Blob(q), "tests/v1/x/answers.pl.in": c1.Blob(a)}
        base.update({"tests/v1/x/" + name: c1.Blob(old) for name, (old, _) in cases.items()})
        result = c1.Oracle(base, {}).build().expected
        for name, (_, wanted) in cases.items():
            with self.subTest(name=name):
                self.assertEqual(result["tests/v1/x/" + name].data, wanted)

    def test_noncanonical_semantic_line_delta(self):
        for version, old_line, new_line in ((1, 23, 35), (2, 27, 35), (3, 35, 35)):
            base = {
                "tests/v1/cases.tsv": c1.Blob(b"# case\tprobe\tmode\trc\tstderr_lines\targs\nx\ttarget\tcheck\t2\t1\tinput.pl\n"),
                "tests/v1/x/input.pl.in": c1.Blob(doc(version)),
                "tests/v1/x/expect/target.stderr": c1.Blob(f"ace_to_pl_error(check_load,noncanonical({old_line},4)).\n".encode()),
            }
            new = c1.Oracle(base, {}).build().expected
            self.assertEqual(new["tests/v1/x/expect/target.stderr"].data, f"ace_to_pl_error(check_load,noncanonical({new_line},4)).\n".encode())

    def test_retired_probe_and_case_deletion(self):
        base = {"tests/v1/cases.tsv": c1.Blob(b"# table\nx\ttarget\tcheck\t2\t1\tinput.pl\ny\ttarget\tcheck\t0\t-\tinput.pl\n"),
                "tests/v1/x/input.pl.in": c1.Blob(doc()),
                "tests/v1/x/expect/target.stderr": c1.Blob(b"reject\n"),
                "tests/v1/y/input.pl.in": c1.Blob(doc())}
        d = row("tests/v1", "x/target", "retired")
        o = c1.Oracle(base, {d.key: d}).build()
        self.assertNotIn("tests/v1/x/input.pl.in", o.expected)
        self.assertNotIn(b"x\ttarget", o.expected["tests/v1/cases.tsv"].data)
        bad = dict(o.expected)
        bad["tests/v1/x/input.pl.in"] = base["tests/v1/x/input.pl.in"]
        self.assertIn("retired item retained", o.compare(bad)[0])

    def test_rule_needs_matching_base_and_disposition(self):
        p = "tests/v1/x/expect/target.stderr"
        base = {p: c1.Blob(b"old\n")}
        d = row("tests/v1", "x/target", "re-targeted")
        rule = {"family": d.family, "item": d.item, "path": p, "base_sha256": digest(b"old\n").decode(),
                "edits": [edit_bytes(b"old", b"new")], "reason": "retained reject retargeted"}
        self.assertEqual(c1.Oracle(base, {d.key: d}, [rule]).build().expected[p].data, b"new\n")
        with self.assertRaises(c1.InputError):
            c1.Oracle(base, {}, [rule]).build()
        rule["base_sha256"] = "0" * 64
        with self.assertRaises(c1.InputError):
            c1.Oracle(base, {d.key: d}, [rule]).build()

    def test_template_expansion_and_pin(self):
        body = doc() + b"@@run 1 3 guideline_event(a,e@i@,v@j@).\n@@tree 3\ta\tb\tp(\t).\n"
        template = b"@@sha256 " + b"0" * 64 + b"\n" + body
        expanded = doc() + b"guideline_event(a,e1,v2).\nguideline_event(a,e2,v3).\nguideline_event(a,e3,v4).\np(','(','(a,b),b)).\n"
        self.assertEqual(c1.expand_template(template), expanded)
        template = b"@@sha256 " + digest(expanded) + b"\n" + body
        output = c1.Oracle({"tests/v1/x/input.pl.tpl.in": c1.Blob(template)}, {}).build().expected
        result = output["tests/v1/x/input.pl.tpl.in"].data
        self.assertEqual(result.split(b"\n", 1)[0], b"@@sha256 " + digest(c1.expand_template(result)))
        self.assertTrue(c1.expand_template(result).endswith(expanded[len(doc()):]))

    def test_ui_pl_text_and_time_words_only(self):
        case = "tests/ui/green/example/"
        p = case + "tree/guidelines/g/pl/d.pl"
        page = case + "golden/g/g/doc/d.html"
        index = case + "golden/g/g/index.html"
        old = doc(2)
        caption = f"<summary>Compiled Prolog ({len(old.splitlines())} lines)</summary>".encode()
        word = b"<tr><td>after</td><td>how long after a reference point the action occurs</td></tr>"
        assertions = case + "assertions.tsv"
        base = {p: c1.Blob(old), page: c1.Blob(caption + b"<pre>" + c1.ui_escape(old) + b"</pre><p>Keep me.</p>\n"),
                case + "tree/guidelines/g/temporal.tsv": c1.Blob(OLD_TABLE + b"relation\tafter\tafter\n"),
                index: c1.Blob(word + b"\n"), assertions: c1.Blob(b"g/g/index.html\tcontains\t" + word.hex().encode() + b"\n")}
        o = c1.Oracle(base, {}).build()
        new_caption = f"<summary>Compiled Prolog ({len(o.expected[p].data.splitlines())} lines)</summary>".encode()
        self.assertEqual(o.expected[page].data, new_caption + b"<pre>" + c1.ui_escape(o.expected[p].data) + b"</pre><p>Keep me.</p>\n")
        self.assertEqual(bytes.fromhex(o.expected[assertions].data.split(b"\t")[2].decode()), o.expected[index].data.rstrip(b"\n"))
        self.assertIn(b"or only that it occurs after it</td>", o.expected[index].data)
        tree = dict(o.expected)
        tree[page] = c1.Blob(tree[page].data.replace(b"Keep me.", b"Lose me."))
        self.assertTrue(o.compare(tree))

    def test_review_bundle_recomputes_exact_hash_and_hidden_field(self):
        case = "tests/ui/green/example/"
        prefix = case + "tree/guidelines/g/"
        old = doc(3)
        fields = [b"d", b"a" * 64, b"b" * 64, b"c" * 64, digest(c1.semantic_stream(old))]
        review = review_digest(fields)
        fields.append(review)
        manifest = prefix + "audit/review-manifest.tsv"
        page = case + "golden/g/g/doc/d.html"
        base = {prefix + "pl/d.pl": c1.Blob(old), manifest: c1.Blob(b"\t".join(fields) + b"\n"),
                page: c1.Blob(b'<input type="hidden" name="review_sha256" value="' + review + b'">\n<p>Visible text.</p>\n')}
        o = c1.Oracle(base, {}).build()
        sem = digest(c1.semantic_stream(o.expected[prefix + "pl/d.pl"].data))
        migrated = fields[:]
        migrated[4] = sem
        wanted = review_digest(migrated)
        self.assertEqual(o.expected[manifest].data.split(b"\t")[4:], [sem, wanted + b"\n"])
        self.assertIn(wanted, o.expected[page].data)
        self.assertEqual(o.compare(o.expected), [])
        bad = dict(o.expected)
        bad[manifest] = c1.Blob(bad[manifest].data.replace(wanted, b"e" * 64))
        bad[page] = c1.Blob(bad[page].data.replace(wanted, b"e" * 64))
        self.assertTrue(o.compare(bad), "matching wrong manifest/form digests must still fail")

    def test_review_rebind_does_not_touch_visible_digest_text(self):
        case = "tests/ui/green/example/"
        prefix = case + "tree/guidelines/g/"
        old = doc(3)
        fields = [b"d", b"a" * 64, b"b" * 64, b"c" * 64, digest(c1.semantic_stream(old))]
        review = review_digest(fields)
        fields.append(review)
        manifest = prefix + "audit/review-manifest.tsv"
        page = case + "golden/g/g/doc/d.html"
        for literal in (review, b'name="review_sha256" value="' + review + b'"', b"review_sha256=" + review + b" is text"):
            with self.subTest(literal=literal):
                base = {prefix + "pl/d.pl": c1.Blob(old), manifest: c1.Blob(b"\t".join(fields) + b"\n"),
                        page: c1.Blob(b'<input type="hidden" name="review_sha256" value="' + review + b'">\n<p>' + literal + b'</p>\n')}
                o = c1.Oracle(base, {}).build()
                self.assertIn(b"<p>" + literal + b"</p>", o.expected[page].data)
                self.assertEqual(o.compare(o.expected), [])
                actual = dict(o.expected)
                changed = literal.replace(review, b"e" * 64)
                actual[page] = c1.Blob(actual[page].data.replace(b"<p>" + literal, b"<p>" + changed))
                self.assertTrue(o.compare(actual))

    def test_review_rebind_preserves_current_outdated_relation(self):
        prefix = "tests/ui/green/example/tree/guidelines/g/"
        old = doc(3)
        fields = [b"d", b"a" * 64, b"b" * 64, b"c" * 64, digest(c1.semantic_stream(old))]
        fields.append(review_digest(fields))
        future = fields[:]
        future[4] = digest(c1.semantic_stream(c1.document_transform(old)))
        future_review = review_digest(future)
        manifest = prefix + "audit/review-manifest.tsv"
        ledger = prefix + "audit/adjudication.tsv"
        base = {prefix + "pl/d.pl": c1.Blob(old), manifest: c1.Blob(b"\t".join(fields) + b"\n"),
                ledger: c1.Blob(b"d\t" + future_review + b"\t\tapproved\tReviewer\t2026-01-01T00:00:00Z\t\n")}
        o = c1.Oracle(base, {}).build()
        self.assertTrue(o.compare(o.expected), "an outdated row becoming current requires a ruled re-pin")

    def test_loose_ui_payload_is_byte_identical(self):
        path = "tests/ui/green/basic/tree/guidelines/g/pl/d.pl"
        loose = b"guideline_schema_version(3).\nguideline_document(d, ace_sha256(fixture), ulex(none)).\nfixture_fact(d).\n"
        page = "tests/ui/green/basic/golden/g/g/doc/d.html"
        o = c1.Oracle({path: c1.Blob(loose), page: c1.Blob(b"<pre>" + loose + b"</pre>\n")}, {}).build()
        self.assertEqual(o.expected[path].data, loose)
        self.assertEqual(o.expected[page].data, b"<pre>" + loose + b"</pre>\n")

    def test_git_commit_recipe_matches_git(self):
        root = Path(__file__).resolve().parents[3]
        scratch = root / ".scratch"
        scratch.mkdir(exist_ok=True)
        files = {"a.txt": c1.Blob(b"alpha\n"), "nested/b.txt": c1.Blob(b"beta\n"),
                 "z.sh": c1.Blob(b"#!/bin/sh\nexit 0\n", "100755")}
        env = dict(os.environ, GIT_CONFIG_GLOBAL="/dev/null", GIT_CONFIG_SYSTEM="/dev/null", GIT_DEFAULT_HASH="sha1",
                   GIT_CEILING_DIRECTORIES=str(scratch),
                   GIT_AUTHOR_NAME="fixture", GIT_AUTHOR_EMAIL="fixture@localhost", GIT_COMMITTER_NAME="fixture",
                   GIT_COMMITTER_EMAIL="fixture@localhost", GIT_AUTHOR_DATE="2026-01-01T00:00:00+00:00",
                   GIT_COMMITTER_DATE="2026-01-01T00:00:00+00:00")
        with tempfile.TemporaryDirectory(prefix="churn-git-", dir=scratch) as temporary:
            for name, blob in files.items():
                p = Path(temporary) / name
                p.parent.mkdir(parents=True, exist_ok=True)
                p.write_bytes(blob.data)
                p.chmod(0o755 if blob.mode == "100755" else 0o644)
            for argv in (["init", "-q", "-b", "main"], ["add", "-A"], ["commit", "-q", "-m", "fixture corpus"]):
                subprocess.run(["git", "-C", temporary, *argv], env=env, check=True)
            actual = subprocess.check_output(["git", "-C", temporary, "rev-parse", "HEAD"], env=env).strip()
        self.assertEqual(c1.fixture_commit(files), actual)

    def test_kani_constants_close_over_hashes_offsets_and_bounds(self):
        root = Path(__file__).resolve().parents[3]
        source = (root / c1.KANI).read_bytes()
        o = c1.Oracle({c1.KANI: c1.Blob(source)}, {}).build()
        result = o.expected[c1.KANI].data
        payloads = c1.embedded_artifacts(result)
        document = next(x for x in payloads if c1.artifact_kind(x) == "doc")
        queries = [x for x in payloads if c1.artifact_kind(x) == "query"]
        answers = next(x for x in payloads if c1.artifact_kind(x) == "answers")
        self.assertEqual(document.count(b":- multifile("), 15)
        self.assertIn(f"#[kani::unwind({len(document) + 1})]".encode(), result)
        index = int(re.search(rb"query\[([0-9]+)\] =", result)[1])
        self.assertEqual([i for i, (a, b) in enumerate(zip(*queries)) if a != b], [index])
        self.assertIn(digest(queries[0]), answers)
        self.assertIn(digest(queries[1]), result)
        self.assertIn(digest(answers), result)

    def test_release_members_rebind_hash_and_size_and_preserve_labels(self):
        old = doc(3)
        manifest = (b"meta\tschema\tv3\nmember\tdata/guidelines/g/pl/d.pl\t" + digest(old) + b"\t" +
                    str(len(old)).encode() + b"\nlabel\td\tunreviewed\n")
        o = c1.Oracle({"release-manifest.tsv": c1.Blob(manifest), "guidelines/g/pl/d.pl": c1.Blob(old)}, {}).build()
        new = o.expected["guidelines/g/pl/d.pl"].data
        expected = b"meta\tschema\tv1\nmember\tdata/guidelines/g/pl/d.pl\t" + digest(new) + b"\t" + str(len(new)).encode() + b"\nlabel\td\tunreviewed\n"
        self.assertEqual(o.expected["release-manifest.tsv"].data, expected)
        bad = dict(o.expected)
        bad["release-manifest.tsv"] = c1.Blob(expected.replace(b"unreviewed", b"approved"))
        self.assertTrue(o.compare(bad))

    def test_rule_cannot_name_an_unrelated_case(self):
        p = "tests/v1/y/input.pl.in"
        base = {p: c1.Blob(doc())}
        d = row("tests/v1", "x/target", "re-targeted")
        rule = {"family": d.family, "item": d.item, "path": p, "base_sha256": digest(doc()).decode(),
                "edits": [edit_bytes(b"% S1: x", b"% S1: y")], "reason": "wrong case"}
        with self.assertRaises(c1.InputError):
            c1.Oracle(base, {d.key: d}, [rule]).build()

    def test_rule_cannot_erase_retained_reject_or_other_case(self):
        d = row("tests/v1", "x/target", "re-targeted")
        p = "tests/v1/x/expect/target.stderr"
        original = b"ace_to_pl_error(check_load,noncanonical(1,1)).\n"
        rule = {"family": d.family, "item": d.item, "path": p, "base_sha256": digest(original).decode(),
                "edits": [edit_bytes(original, b"")], "reason": "unapproved erased reject"}
        with self.assertRaises(c1.InputError):
            c1.Oracle({p: c1.Blob(original)}, {d.key: d}, [rule]).build()
        positive = row(d.family, d.item, "positive")
        self.assertEqual(c1.Oracle({p: c1.Blob(original)}, {d.key: positive}, [rule]).build().expected[p].data, b"")
        rows = b"x\ttarget\tcheck\t2\t1\tinput.pl\ny\ttarget\tcheck\t2\t1\tinput.pl\n"
        p = "tests/v1/cases.tsv"
        for old, new in ((rows.splitlines(keepends=True)[1], b""),
                         (rows.splitlines(keepends=True)[0], b""),
                         (rows.splitlines(keepends=True)[0], rows.splitlines(keepends=True)[0] * 2)):
            rule = {"family": d.family, "item": d.item, "path": p, "base_sha256": digest(rows).decode(),
                    "edits": [edit_bytes(old, new)], "reason": "unapproved case loss or duplication"}
            with self.assertRaises(c1.InputError):
                c1.Oracle({p: c1.Blob(rows)}, {d.key: d}, [rule]).build()

    def test_order_ruling_is_insert_only_and_names_terms(self):
        p = "tests/queries/green/v2-annotation/doc.pl"
        base = {p: c1.Blob(doc(2))}
        d = row("tests/queries", "green/v2-annotation", "migrated")
        addition = b"guideline_order(actual,e,after,r).\n"
        rule = {"family": d.family, "item": d.item, "path": p, "base_sha256": digest(doc(2)).decode(),
                "edits": [edit_bytes(b"guideline_event(actual,e,wait).\n", b"guideline_event(actual,e,wait).\n" + addition)],
                "reason": "D6 order over former v2 table"}
        with self.assertRaises(c1.InputError):
            c1.Oracle(base, {d.key: d}, [rule]).build()
        rule["reason"] = "order: heads=guideline_order(actual,e,after,r); goals=none; bodies=none"
        self.assertTrue(c1.Oracle(base, {d.key: d}, [rule]).build().expected[p].data.endswith(addition))
        rule["edits"][0]["new"] = b"guideline_event(actual,e,stop).\n" + addition
        with self.assertRaises(c1.InputError):
            c1.Oracle(base, {d.key: d}, [rule]).build()
        qpath = "tests/queries/green/v2-annotation/query.pl"
        q = query(2)
        goal = b"guideline_event(actual,e,wait)"
        order = b"guideline_order(actual,e,after,r)"
        edit = edit_bytes(b"goal(" + goal + b")", b"goal(','(" + goal + b"," + order + b"))")
        qr = {"family": d.family, "item": d.item, "path": qpath, "edits": [edit],
              "reason": "order: heads=none; goals=guideline_order(actual,e,after,r); bodies=none"}
        self.assertIn(order, c1.Oracle({qpath: c1.Blob(q)}, {d.key: d}, [qr]).build().expected[qpath].data)
        edit["new"] = edit["new"].replace(b"wait", b"stop")
        with self.assertRaises(c1.InputError):
            c1.Oracle({qpath: c1.Blob(q)}, {d.key: d}, [qr]).build()

    def test_tsv_literal_round_trip_and_parser_errors(self):
        data = bytes(range(256))
        self.assertEqual(c1.literal_bytes(c1.escaped_bytes(data)), data)
        header = b"# family\titem\tpath\told\tnew\tcount\treason\n"
        line = b"tests/v1\tx/target\ttests/v1/x/input.pl.in\t@whole\t" + c1.escaped_bytes(data).encode() + b"\t1\texact byte fixture\n"
        parsed = c1.load_rulings(header + line)
        self.assertEqual(parsed[0]["mode"], "whole")
        self.assertEqual(parsed[0]["edits"][0]["new"], data)
        for malformed in (line, header + line.rstrip(b"\n"), header + line.replace(b"\t1\t", b"\t0\t"),
                          header + line.replace(b"@whole", b"@new").replace(b"\t1\t", b"\t2\t"),
                          header + line.replace(b"@whole", b"\\q"),
                          header + line.replace(b"@whole", b"anchor").replace(b"\t1\t", b"\t" + b"1" * 5000 + b"\t")):
            with self.subTest(malformed=malformed[:90]), self.assertRaises(c1.InputError):
                c1.load_rulings(malformed)

    def test_tsv_rows_apply_in_order_and_bind_original_hash(self):
        p = "tests/v1/x/expect/target.stdout"
        d = row("tests/v1", "x/target", "positive")
        header = "# family\titem\tpath\told\tnew\tcount\treason\tbase_sha256\n"
        rows = (f"tests/v1\tx/target\t{p}\ta\tb\t1\tfirst\t{digest(b'a').decode()}\n" +
                f"tests/v1\tx/target\t{p}\tb\tc\t1\tsecond\t{digest(b'a').decode()}\n")
        rules = c1.load_rulings((header + rows).encode())
        self.assertEqual(c1.Oracle({p: c1.Blob(b"a")}, {d.key: d}, rules).build().expected[p].data, b"c")
        bad = c1.load_rulings((header + rows.replace(digest(b'a').decode(), "0" * 64)).encode())
        with self.assertRaises(c1.InputError):
            c1.Oracle({p: c1.Blob(b"a")}, {d.key: d}, bad).build()

    def test_tsv_whole_new_and_post_rename_path(self):
        p = "tests/red/temporal_load--x.expect"
        target = "tests/red/vocabulary_load--x.expect"
        d = row("tests/red", "temporal_load--x", "re-targeted")
        old = b"ace_to_pl_error(temporal_load,old).\n"
        header = "# family\titem\tpath\told\tnew\tcount\treason\tbase_sha256\n"
        text = header + f"{d.family}\t{d.item}\t{target}\t@whole\tace_to_pl_error(vocabulary_load,new).\\n\t1\tretarget\t{digest(old).decode()}\n"
        result = c1.Oracle({p: c1.Blob(old)}, {d.key: d}, c1.load_rulings(text.encode())).build()
        self.assertEqual(result.expected[target].data, b"ace_to_pl_error(vocabulary_load,new).\n")
        added = "tests/red/vocabulary_load--x.argv"
        text = header + f"{d.family}\t{d.item}\t{added}\t@new\t@TREE@ @ID@ - -\\n\t1\tnew vector\t\n"
        result = c1.Oracle({}, {d.key: d}, c1.load_rulings(text.encode())).build()
        self.assertEqual(result.expected[added].data, b"@TREE@ @ID@ - -\n")

    def test_tsv_deleted_support_file(self):
        p = "tests/certify-one/v3-header.txt.in"
        d = row("tests/certify-one", "tests/certify-one/v3/temporal.tsv.in", "migrated")
        header = "# family\titem\tpath\told\tnew\tcount\treason\n"
        text = header + f"{d.family}\t{d.item}\t{p}\t@whole\t@deleted\t1\theader now emitted by generator\n"
        o = c1.Oracle({p: c1.Blob(OLD_TABLE)}, {d.key: d}, c1.load_rulings(text.encode())).build()
        self.assertNotIn(p, o.expected)
        self.assertEqual(o.compare({}), [])
        self.assertTrue(o.compare({p: c1.Blob(OLD_TABLE)}))
        live = "tests/v1/x/expect/target.stderr"
        d = row("tests/v1", "x/target", "re-targeted")
        text = header + f"{d.family}\t{d.item}\t{live}\t@whole\t@deleted\t1\tunapproved reject deletion\n"
        with self.assertRaises(c1.InputError):
            c1.Oracle({live: c1.Blob(b"reject\n")}, {d.key: d}, c1.load_rulings(text.encode())).build()
        with self.assertRaises(c1.InputError):
            c1.load_rulings(text.replace("@whole", "@new").encode())
        seed = "tests/v1/q18-k1-001/document-v1-none.pl.in"
        d = row("q18 census", "document-v1-none/S6", "re-targeted")
        text = header + f"{d.family}\t{d.item}\t{seed}\t@whole\t@deleted\t1\tdomain rebuilt under consolidated axes\n"
        o = c1.Oracle({seed: c1.Blob(doc())}, {d.key: d}, c1.load_rulings(text.encode())).build()
        self.assertNotIn(seed, o.expected)
        other = row("q18 census", "query-v1-none/S1", "re-targeted")
        text = text.replace(d.item, other.item)
        with self.assertRaises(c1.InputError):
            c1.Oracle({seed: c1.Blob(doc())}, {other.key: other}, c1.load_rulings(text.encode())).build()

    def test_new_coverage_without_disposition_and_its_limits(self):
        manifest = "tests/check-mutants/cases.tsv"
        old_row = b"old\tcorpus_vocabulary\t1\n"
        new_row = b"fresh\tcorpus_vocabulary\t1\n"
        base = {manifest: c1.Blob(old_row), "tests/check-mutants/old.expect": c1.Blob(b"reject\n")}
        add = {"family": "tests/check-mutants", "item": "fresh", "path": "tests/check-mutants/fresh.expect",
               "mode": "new", "edits": [edit_bytes(b"", b"new reject\n")], "reason": "new: new vocabulary coverage"}
        insert = {"family": "tests/check-mutants", "item": "fresh", "path": manifest,
                  "mode": "edit", "edits": [edit_bytes(old_row, old_row + new_row)], "reason": "new: add the same item"}
        o = c1.Oracle(base, {}, [add, insert]).build()
        self.assertEqual(o.expected[manifest].data, old_row + new_row)
        self.assertEqual(o.expected[add["path"]].data, b"new reject\n")
        for bad in (
            dict(add, reason="no new prefix"),
            dict(add, item="old", path="tests/check-mutants/old.extra.in"),
            dict(insert, edits=[edit_bytes(old_row, old_row.replace(b"vocabulary", b"changed"))]),
            dict(insert, mode="whole", edits=[edit_bytes(b"", old_row + new_row)]),
        ):
            with self.subTest(bad=bad), self.assertRaises(c1.InputError):
                c1.Oracle(base, {}, [bad]).build()
        known = row("tests/check-mutants", "old", "migrated")
        bad = dict(add, item="old", path="tests/check-mutants/old.extra.in")
        with self.assertRaises(c1.InputError):
            c1.Oracle(base, {known.key: known}, [bad]).build()
        red_base = {"tests/red/temporal_load--old.ace": c1.Blob(b"x\n")}
        bad = dict(add, family="tests/red", item="vocabulary_load--old", path="tests/red/vocabulary_load--old.extra.in")
        with self.assertRaises(c1.InputError):
            c1.Oracle(red_base, {}, [bad]).build()

    def test_document_as_answers_term_count_follows_declarations(self):
        for version, delta in ((1, 12), (2, 8), (3, 0)):
            document = doc(version)
            n = sum(line.endswith(b".") and not line.startswith(b"%") for line in document.splitlines())
            pin = f"ace_to_pl_error(check_load,answers_file(term_count({n}))).\n".encode()
            base = {"tests/v1/cases.tsv": c1.Blob(b"x\tdoc-class\ttrace\t2\t1\tm.tsv q.pl a.pl\n"),
                    "tests/v1/x/a.pl.in": c1.Blob(document),
                    "tests/v1/x/expect/doc-class.stderr": c1.Blob(pin)}
            o = c1.Oracle(base, {}).build()
            self.assertEqual(o.expected["tests/v1/x/expect/doc-class.stderr"].data,
                             f"ace_to_pl_error(check_load,answers_file(term_count({n + delta}))).\n".encode())

    def test_case_rulings_do_not_create_global_digest_back_edges(self):
        q = query()
        good = "tests/v1/x/query.pl.in"
        bad = "tests/v1/y/query.pl.in"
        answer_path = "tests/v1/x/answers.pl.in"
        d = row("tests/v1", "y/target", "re-targeted")
        ruling = {"family": d.family, "item": d.item, "path": bad, "mode": "whole",
                  "edits": [edit_bytes(b"", q)], "reason": "keep missing mandatory vocabulary wrapper"}
        old_reject = q.replace(b"'(v1,q,", b"'(v3,q,")
        o = c1.Oracle({good: c1.Blob(q), bad: c1.Blob(old_reject), answer_path: c1.Blob(answer(q))}, {d.key: d}, [ruling]).build()
        self.assertEqual(o.expected[good].data, c1.query_transform(q))
        self.assertEqual(o.expected[bad].data, q)
        self.assertEqual(o.expected[answer_path].data, answer(c1.query_transform(q)))
        d = row("tests/v1", "y/target", "positive")
        path = "tests/v1/y/expect/target.stdout"
        ruling = {"family": d.family, "item": d.item, "path": path, "mode": "whole",
                  "edits": [edit_bytes(b"", b"accepted output\n")], "reason": "former rejection now accepts"}
        base = {path: c1.Blob(b""), "tests/v1/x/empty.in": c1.Blob(b""), "tests/v1/x/hash.in": c1.Blob(digest(b""))}
        o = c1.Oracle(base, {d.key: d}, [ruling]).build()
        self.assertEqual(o.expected["tests/v1/x/hash.in"].data, digest(b""))

    def test_broken_self_test_returns_two(self):
        root = Path(__file__).resolve().parents[3]
        scratch = root / ".scratch"
        scratch.mkdir(exist_ok=True)
        base = {"tests/v1/cases.tsv": c1.Blob(b"x\ttarget\tcheck\t0\t-\tinput.pl\n"),
                "tests/v1/x/input.pl.in": c1.Blob(doc())}
        with tempfile.TemporaryDirectory(prefix="churn-status-", dir=scratch) as temporary:
            p = Path(temporary) / c1.DISPOSITIONS
            p.parent.mkdir(parents=True)
            p.write_bytes(b"tests/v1\tx/target\t\t\t\t\tmigrated\tlaw\tevidence\n")
            output = io.StringIO()
            with patch.object(c1, "base_blobs", return_value=base), patch.object(c1, "self_test", side_effect=AssertionError("planted checker failure")), contextlib.redirect_stdout(output):
                rc = c1.main(["c1-churn.py", "base", temporary])
        self.assertEqual(rc, 2)
        self.assertEqual(output.getvalue(), "c1-churn: self-test failed: planted checker failure\n")

    def test_self_test_all_eight_plants(self):
        base = {"tests/v1/cases.tsv": c1.Blob(b"x\ttarget\tcheck\t2\t1\tinput.pl\n"),
                "tests/v1/x/input.pl.in": c1.Blob(doc()),
                "tests/v1/x/expect/target.stderr": c1.Blob(b"ace_to_pl_error(check_load,noncanonical(23,1)).\n")}
        o = c1.Oracle(base, {}).build()
        evidence = c1.self_test(o)
        self.assertEqual({name for name, _ in evidence}, {"semantic-clause", "wrong-digest", "wrong-offset", "erased-reject", "unlisted-file", "omitted-case", "new-base-item", "new-existing-row"})
        original_compare = o.compare
        o.compare = lambda actual: []
        with self.assertRaises(AssertionError):
            c1.self_test(o)
        o.compare = original_compare


def integration(base_commit, root):
    root = Path(root).resolve()
    script = Path(__file__).with_name("c1-churn.py").resolve()
    start = time.monotonic()
    base = c1.base_blobs(root, base_commit)
    dispositions = c1.load_dispositions((root / c1.DISPOSITIONS).read_bytes())
    rules = root / c1.RULINGS
    rulings = c1.load_rulings(rules.read_bytes()) if rules.exists() else []
    oracle = c1.Oracle(base, dispositions, rulings)
    oracle.validate_dispositions()
    oracle.build()
    print(f"model: {len(base)} base artifacts; {len(oracle.expected)} predicted artifacts; {dict(oracle.counts)}", flush=True)
    print(f"derived: {len(oracle.hashes)} digest bindings; {len(oracle.offsets)} offset pins; "
          f"{sum(map(len, oracle.review_changes.values()))} exact review re-pins; {len(oracle.commit_ids)} fixture commits", flush=True)
    if oracle.compare(oracle.expected):
        raise AssertionError("synthetic model rejected")
    c1.self_test(oracle)
    scratch = Path(__file__).resolve().parents[3] / ".scratch"
    scratch.mkdir(exist_ok=True)
    with tempfile.TemporaryDirectory(prefix="c1-churn-perfect-", dir=scratch) as temporary:
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

        for path, blob in oracle.expected.items():
            materialize(path, blob)
        for path in (c1.DISPOSITIONS, c1.RULINGS):
            if (root / path).exists():
                materialize(path, c1.Blob((root / path).read_bytes()))
        env = dict(os.environ, GIT_CONFIG_GLOBAL="/dev/null", GIT_CONFIG_SYSTEM="/dev/null", GIT_CEILING_DIRECTORIES=str(scratch))
        env["GIT_ALTERNATE_OBJECT_DIRECTORIES"] = c1.git(root, "rev-parse", "--path-format=absolute", "--git-path", "objects").strip().decode()
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

        run("synthetic CLI", 0)
        for name, path, blob in c1.self_test_cases(oracle):
            materialize(path, blob)
            lines = run("plant " + name, 1)
            if not any(line.startswith("c1-churn: " + path + ":") for line in lines):
                raise AssertionError(f"{name}: failure did not identify planted path")
            materialize(path, oracle.expected.get(path))
        policy = (root / c1.RULINGS).read_bytes() if (root / c1.RULINGS).exists() else None
        prefix = policy if policy is not None else b"# family\titem\tpath\told\tnew\tcount\treason\n"
        for name, ruling, expected_error in c1.self_test_rulings(oracle):
            edit = ruling["edits"][0]
            old = "@new" if ruling["mode"] == "new" else c1.escaped_bytes(edit["old"])
            fields = [ruling["family"], ruling["item"], ruling["path"], old, c1.escaped_bytes(edit["new"]),
                      str(edit["count"]), ruling["reason"]]
            materialize(c1.RULINGS, c1.Blob(prefix + ("\t".join(fields) + "\n").encode()))
            path = ruling["path"]
            if ruling["mode"] == "new":
                planted = edit["new"]
            else:
                planted = oracle.expected[path].data.replace(edit["old"], edit["new"], edit["count"])
            materialize(path, c1.Blob(planted))
            lines = run("plant " + name, 1)
            if not any(expected_error in line for line in lines):
                raise AssertionError(f"{name}: wrong refusal: {lines[:1]}")
            materialize(path, oracle.expected.get(path))
            materialize(c1.RULINGS, c1.Blob(policy) if policy is not None else None)
    print(f"integration: ok; eight CLI plants rejected; {time.monotonic() - start:.3f} s", flush=True)


if __name__ == "__main__":
    if len(sys.argv) == 4 and sys.argv[1] == "--integration":
        integration(sys.argv[2], sys.argv[3])
    else:
        unittest.main(verbosity=2)
