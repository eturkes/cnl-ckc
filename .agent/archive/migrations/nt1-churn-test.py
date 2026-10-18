#!/usr/bin/env python3
"""NT1 D12 oracle contract tests; independent expected bytes + planted changes."""
import contextlib
import importlib.util
import io
from pathlib import Path
import random
import sys
import types
import unittest
from unittest.mock import patch

sys.dont_write_bytecode = True
spec = importlib.util.spec_from_file_location("nt1_churn", Path(__file__).with_name("nt1-churn.py"))
nt = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = nt
spec.loader.exec_module(nt)

INDICATORS = (("schema_version", 1), ("document", 4), ("entity", 4), ("cardinality", 5),
              ("event", 3), ("arg", 4), ("pp", 4), ("property", 4), ("operator", 3),
              ("interval", 6), ("recurrence", 4), ("frequency", 5), ("order", 4),
              ("recurrence_window", 7), ("range", 3), ("grade", 5), ("preference", 4), ("qualifier", 4))
OLD = b"".join(f":- {k}(guideline_{n}/{a}).\n".encode()
               for n, a in INDICATORS for k in ("multifile", "discontiguous"))
ADDED = b":- multifile(guideline_unit/3).\n:- discontiguous(guideline_unit/3).\n"
TABLE = b"# format: kind\tlemma\tvalue\nunit\tday\tday\nunit\thour\thour\nunit\td\tday\n"
UNITS = {b"day": b"day", b"hour": b"hour", b"d": b"day"}


def doc(body, header=OLD):
    return (b"% d.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.\n" + header +
            b"guideline_schema_version(1).\nguideline_document(d,ace_sha256(" + b"a" * 64 +
            b"),ulex(none),vocabulary(none)).\n% S1: A statement.\n" + body)


def question(goals):
    return (b"% q compiled from ACE question by ace_to_pl question mode; do not edit.\n"
            b"'$guideline_query'(v1,q,ace_sha256(" + b"a" * 64 + b"),ulex(none),vocabulary(none)).\n"
            b"% Q1: A question?\n'$guideline_query_projection'(goal(" + goals + b"),answers([])).\n")


def bundle(f):
    return nt.sha(b"bundle v2 " + f[0] + b"\nace " + f[1] + b"\ncoverage " + f[2] +
                  b"\npayload " + f[3] + b"\nclauses " + f[4] + b"\n")


class NT1OracleContract(unittest.TestCase):
    def test_o2_declarations_exact_and_idempotent(self):
        original = doc(b"guideline_event(actual,e,wait).\n")
        self.assertEqual(nt.document_transform(original), original.replace(OLD, OLD + ADDED, 1))
        self.assertEqual(nt.document_transform(nt.document_transform(original)), nt.document_transform(original))
        self.assertEqual(nt.document_transform(original).count(b":- multifile("), 19)

    def test_o2_every_malformed_block_byte_is_preserved(self):
        for i in range(len(OLD)):
            broken = OLD[:i] + b"~" + OLD[i + 1:]
            self.assertEqual(nt.document_transform(broken), broken, i)

    def test_o2_offsets_and_term_counts(self):
        original = doc(b"guideline_event(actual,e,wait).\n")
        base = {"tests/v1/cases.tsv": nt.Blob(b"x\tp\tcheck\t2\t1\tin.pl\n"),
                "tests/v1/x/in.pl.in": nt.Blob(original),
                "tests/v1/x/expect/p.stderr": nt.Blob(b"ace_to_pl_error(check_load,noncanonical(40,7)).\n")}
        o = nt.Oracle(base, {}).build()
        self.assertIn(b"noncanonical(42,7)", o.expected["tests/v1/x/expect/p.stderr"].data)

    def test_o2_template_pin(self):
        body = doc(b"guideline_event(actual,e,wait).\n") + b"@@run 1 4 guideline_event(a,e@i@,v@j@).\n"
        raw = nt.template_pin(b"@@sha256 " + b"0" * 64 + b"\n" + body)
        p = "tests/v1/x/in.pl.tpl.in"
        predicted = nt.Oracle({p: nt.Blob(raw)}, {}).build().expected[p].data
        self.assertEqual(predicted.split(b"\n", 1)[0], b"@@sha256 " + nt.sha(nt.expand_template(predicted)))
        self.assertEqual(nt.expand_template(predicted), nt.expand_template(raw).replace(OLD, OLD + ADDED))

    def test_o3_non_time_conversion(self):
        old = b"guideline_event(actual,A,wait) :- guideline_entity(actual,B,tablet,countable), guideline_cardinality(actual,B,na,geq,50).\n"
        new = b"guideline_event(actual,A,wait) :- guideline_entity(actual,B,tablet,countable), guideline_cardinality(actual,B,na,C,D), guideline_satisfies(C,D,geq,50).\n"
        self.assertEqual(nt.semantic_document(doc(old), {}).data, doc(new, OLD + ADDED))

    def test_o3_time_root_and_head_units(self):
        for suffix in (b".\n", b" :- guideline_event(actual,A,wait).\n"):
            old = b"guideline_entity(actual,q,day,countable)" + suffix + b"guideline_cardinality(actual,q,na,geq,3)" + suffix
            new = old + b"guideline_unit(actual,q,day)" + suffix
            self.assertEqual(nt.semantic_document(doc(old), UNITS).data, doc(new, OLD + ADDED))

    def test_o3_time_conversion_annotation_before_quantity(self):
        old = (b"guideline_event(actual,A,wait) :- guideline_recurrence(actual,A,B,hour), guideline_entity(actual,B,hour,countable), "
               b"guideline_cardinality(actual,B,na,geq,4).\n")
        new = (b"guideline_event(actual,A,wait) :- guideline_recurrence(actual,A,B,C), guideline_cardinality(actual,B,na,D,E), "
               b"guideline_unit(actual,B,C), guideline_satisfies(D,E,C,geq,4,hour).\n")
        self.assertEqual(nt.semantic_document(doc(old), UNITS).data, doc(new, OLD + ADDED))

    def test_o3_all_annotation_unit_links(self):
        old = (b"guideline_event(actual,A,wait) :- guideline_recurrence_window(actual,A,B,day,C,D,hour), "
               b"guideline_interval(actual,A,duration,B,day,none), guideline_entity(actual,B,day,countable), "
               b"guideline_cardinality(actual,B,na,geq,2), guideline_entity(actual,D,hour,countable), guideline_cardinality(actual,D,na,less,6).\n")
        result = nt.semantic_document(doc(old), UNITS)
        self.assertIn(b"guideline_recurrence_window(actual,A,B,C,D,E,F)", result.data)
        self.assertIn(b"guideline_interval(actual,A,duration,B,C,none)", result.data)
        self.assertIn(b"guideline_satisfies(G,H,C,geq,2,day)", result.data)
        self.assertIn(b"guideline_satisfies(I,J,F,less,6,hour)", result.data)
        self.assertEqual(result.reverse(), doc(old, OLD + ADDED))

    def test_o3_naf_local_and_horn_variants(self):
        old = (b"guideline_event(actual,A,wait) :- guideline_entity(actual,A,patient,countable), \\+ (guideline_entity(actual,B,day,countable), guideline_cardinality(actual,B,na,less,3)).\n")
        new = (b"guideline_event(actual,A,wait) :- guideline_entity(actual,A,patient,countable), \\+ (guideline_cardinality(actual,B,na,C,D), guideline_unit(actual,B,E), guideline_satisfies(C,D,E,less,3,day)).\n")
        self.assertEqual(nt.semantic_document(doc(old + old), UNITS).data, doc(new + new, OLD + ADDED))

    def test_o3_convertible_domain_and_inverse_property(self):
        rng = random.Random(0xD12)
        for cmp in (b"eq", b"about", b"na", b"exactly", b"geq", b"greater", b"leq", b"less"):
            for unit in (b"day", b"tablet"):
                for _ in range(10):
                    n = str(rng.randrange(-10**40, 10**40)).encode()
                    old = b"guideline_event(actual,A,wait) :- guideline_entity(actual,B," + unit + b",countable), guideline_cardinality(actual,B,na," + cmp + b"," + n + b").\n"
                    result = nt.semantic_document(doc(old), UNITS)
                    self.assertEqual(result.reverse(), doc(old, OLD + ADDED))
                    self.assertEqual(b"guideline_satisfies(" in result.data, cmp in (b"exactly", b"geq", b"greater", b"leq", b"less"))
                    self.assertEqual(result.data.count(b"guideline_unit(actual,"), int(unit == b"day"))

    def test_o3_whole_document_unit_classification(self):
        old = (b"guideline_entity(actual,q,d,countable).\nguideline_cardinality(actual,q,na,eq,2).\n"
               b"% S2: That quantity is used.\nguideline_event(actual,A,wait) :- guideline_cardinality(actual,q,na,geq,2).\n")
        result = nt.semantic_document(doc(old), UNITS)
        self.assertIn(b"guideline_unit(actual,q,day).", result.data)
        self.assertIn(b"guideline_satisfies(B,C,D,geq,2,day)", result.data)
        self.assertEqual(result.reverse(), doc(old, OLD + ADDED))

    def test_o4_witness_time_drops_entity_keeps_reflexive_bound(self):
        payload = (b"'$guideline_proof'(d,1,variant(1),witness([guideline_entity(actual,q,day,countable),guideline_cardinality(actual,q,na,geq,3)]),"
                   b"prove([guideline_entity(actual,r,day,countable),guideline_cardinality(actual,r,na,less,6)])).\n")
        expected = (b"'$guideline_proof'(d,1,variant(1),witness([guideline_cardinality(actual,q,na,geq,3),guideline_unit(actual,q,day)]),"
                    b"prove([guideline_entity(actual,r,day,countable),guideline_cardinality(actual,r,na,less,6),guideline_unit(actual,r,day)])).\n")
        self.assertEqual(nt.proof_transform(payload, UNITS), expected)
        self.assertEqual(nt.proof_transform(payload, {}), payload)

    def test_o5_question_record_unchanged_and_goals_converted(self):
        old = question(b"','(guideline_entity(actual,A,hour,countable),guideline_cardinality(actual,A,na,geq,4))")
        new = question(b"','(guideline_cardinality(actual,A,na,B,C),','(guideline_unit(actual,A,D),guideline_satisfies(B,C,D,geq,4,hour)))")
        self.assertEqual(nt.semantic_query(old, UNITS).data, new)

    def test_o6_answer_set_unchanged_query_hash_rebound(self):
        q = question(b"','(guideline_entity(actual,A,tablet,countable),guideline_cardinality(actual,A,na,geq,4))")
        answer = b"'$guideline_answers'(v1,q,query_sha256(" + nt.atom(nt.sha(q)) + b"),result(solutions([sol([a]),sol([b])]))).\n"
        qp, ap = "guidelines/g/queries/pl/q.pl", "guidelines/g/queries/answers/q.pl"
        o = nt.Oracle({qp: nt.Blob(q), ap: nt.Blob(answer)}, {}).build()
        self.assertEqual(o.expected[ap].data, answer.replace(nt.atom(nt.sha(q)), nt.atom(nt.sha(o.expected[qp].data))))
        self.assertNotEqual(o.expected[ap].data, answer)

    def test_o7_trace_insertion_digest_and_inverse(self):
        old = doc(b"guideline_entity(actual,q,hour,countable).\nguideline_cardinality(actual,q,na,geq,4).\n")
        q = question(b"','(guideline_entity(actual,A,hour,countable),guideline_cardinality(actual,A,na,geq,4))")
        a = b"'$guideline_answers'(v1,q,query_sha256(" + nt.atom(nt.sha(q)) + b"),result(yes)).\n"
        nodes = b",".join(b"clause(sentence(d,1),clause_sha256(" + nt.atom(nt.sha(l + b"\n")) + b"),[])" for l in old.splitlines()[-2:])
        t = b"'$guideline_traces'(v1,q,query_sha256(" + nt.atom(nt.sha(q)) + b"),answers_sha256(" + nt.atom(nt.sha(a)) + b"),result(yes(proved([" + nodes + b"])))).\n"
        base = {"guidelines/g/vocabulary.tsv": nt.Blob(TABLE), "guidelines/g/pl/d.pl": nt.Blob(old),
                "guidelines/g/queries/pl/q.pl": nt.Blob(q), "guidelines/g/queries/answers/q.pl": nt.Blob(a), "guidelines/g/queries/traces/q.pl": nt.Blob(t)}
        o = nt.Oracle(base, {}).build()
        new = o.expected["guidelines/g/queries/traces/q.pl"].data
        self.assertIn(b"builtin(guideline_satisfies(geq,4,hour,geq,4,hour))", new)
        self.assertIn(nt.sha(b"guideline_unit(actual,q,hour).\n"), new)
        self.assertNotIn(nt.sha(b"guideline_entity(actual,q,hour,countable).\n"), new)
        self.assertFalse(o.trace_problems(o.expected))

    def test_o8_relation_exhaustive_interval_oracle(self):
        # Dense intervals → independently compare rational half-step witnesses.
        def values(c, n):
            universe = range(-12, 13)
            return {x for x in universe if {"eq": x >= 2*n, "geq": x >= 2*n, "greater": x > 2*n,
                                            "exactly": x == 2*n, "leq": x <= 2*n, "less": x < 2*n}[c]}
        for s in ("eq", "geq", "greater", "exactly", "leq", "less"):
            for b in ("geq", "greater", "exactly", "leq", "less"):
                for x in range(-3, 4):
                    for y in range(-3, 4):
                        self.assertEqual(nt.within(s, x, b, y), values(s, x) <= values(b, y), (s, x, b, y))
        self.assertFalse(nt.within("greater", 49, "geq", 50))

    def test_o8_units_bigints_and_types(self):
        n = 10**100 + 1
        for args, want in [(("eq", 1, "week", "greater", 3, "day"), True),
                           (("exactly", 18, "month", "geq", 1, "year"), True),
                           (("eq", 45, "day", "geq", 1, "month"), False),
                           (("exactly", n, "week", "exactly", n*7, "day"), True),
                           (("about", 50, "geq", 1), False), (("na", 1, "geq", 1), False),
                           (("eq", True, "geq", 1), False), (("eq", "1", "geq", 1), False)]:
            self.assertEqual(nt.satisfies(args), want, args)

    def test_o8_schema_structure_and_semantic_grid(self):
        schema = nt.schema_bytes()
        self.assertEqual(schema.count(b"guideline_within(") - 2, 15)
        self.assertEqual(schema.count(b"guideline_scale(") - 2, 7)
        self.assertIn(b"Apache-2.0 WITH LLVM-exception", schema)
        self.assertGreater(len(nt.schema_cases()), 10000)
        self.assertFalse(nt.schema_structure(schema))
        self.assertTrue(nt.schema_structure(schema.replace(b"atom(S), ", b"", 1)))

    def test_o9_bundle_changes_exactly_with_semantics(self):
        p = "guidelines/g/pl/d.pl"
        old = doc(b"guideline_entity(actual,q,day,countable).\nguideline_cardinality(actual,q,na,eq,2).\n")
        f = [b"d", b"a"*64, b"b"*64, b"c"*64, nt.sha(nt.semantic_stream(old))]
        f.append(bundle(f))
        m = "guidelines/g/audit/review-manifest.tsv"
        base = {p: nt.Blob(old), m: nt.Blob(b"\t".join(f)+b"\n"), "guidelines/g/vocabulary.tsv": nt.Blob(TABLE)}
        o = nt.Oracle(base, {}).build()
        g = list(f)
        g[4] = nt.sha(nt.semantic_stream(o.expected[p].data))
        g[5] = bundle(g)
        self.assertEqual(o.expected[m].data, b"\t".join(g)+b"\n")
        self.assertNotEqual(f[4:], g[4:])
        base.pop("guidelines/g/vocabulary.tsv")
        self.assertEqual(nt.Oracle(base, {}).build().expected[m], base[m])

    def test_o10_block_only_semantics_and_frozen_inputs(self):
        body = b"guideline_event(actual,A,wait) :- guideline_entity(actual,B,day,countable), guideline_cardinality(actual,B,na,geq,4).\n"
        base = {p: nt.Blob(doc(body)) for p in ("tests/v1/x/in.pl.in", "tests/ui/green/x/tree/guidelines/g/pl/d.pl",
                                              "tests/s1-compiler/old.pl", "tests/s2-codec/old.pl", "tests/nt1/old.pl")}
        out = nt.Oracle(base, {}).build().expected
        for p in base:
            expected = base[p].data if any(f"/{k}/" in p for k in ("s1-compiler", "s2-codec", "nt1")) else doc(body, OLD+ADDED)
            self.assertEqual(out[p].data, expected)

    def test_o1_no_removal_addition_or_mode_changes(self):
        p = "tests/red/x.expect"
        o = nt.Oracle({p: nt.Blob(b"reject\n")}, {}).build()
        for path, value in [(p, None), (p, nt.Blob(b"reject\n", "100755")), ("tests/new", nt.Blob(b"x"))]:
            tree = dict(o.expected)
            if value is None:
                del tree[path]
            else:
                tree[path] = value
            problems = o.compare(tree)
            self.assertTrue(problems)
            self.assertTrue(any(path in x for x in problems))

    def test_o11_all_family_plants_fire(self):
        o = nt.synthetic_oracle()
        evidence = nt.self_test(o)
        self.assertTrue({f"O{i}" for i in range(1, 11)} <= {x[0].split(":")[0] for x in evidence})
        self.assertFalse(o.compare(o.expected))

    def test_o12_cli_clean_bad_and_broken_selftest(self):
        o = nt.synthetic_oracle()
        with patch.object(nt, "base_blobs", return_value=o.base), patch.object(nt, "rulings_for", return_value=[]), \
             patch.object(nt, "working_blobs", return_value=(o.expected, set())), patch.object(nt, "schema_differential", return_value=(0, 123)), \
             contextlib.redirect_stdout(io.StringIO()) as out:
            self.assertEqual(nt.main(["nt1-churn.py", "base", str(Path(__file__).resolve().parents[3])]), 0)
            self.assertIn("0 unpredicted changes", out.getvalue())
            with patch.object(nt, "self_test", side_effect=AssertionError("plant accepted")):
                self.assertEqual(nt.main(["nt1-churn.py", "base", str(Path(__file__).resolve().parents[3])]), 2)


class NT1OracleBoundaries(unittest.TestCase):
    def test_quoted_atoms_delimiters_do_not_split_clause(self):
        old = b"guideline_entity(actual,q,'a :- b, [x]',countable).\n"
        self.assertEqual(nt.semantic_document(doc(old), {}).data, doc(old, OLD + ADDED))
        line = b"guideline_entity(actual,A,'can\\'t, split',countable) :- guideline_cardinality(actual,A,na,eq,1).\n"
        self.assertEqual(nt.render_clause(nt.parse_clause(line)), line)

    def test_consequent_skolem_dependencies_do_not_gain_comparison_variables(self):
        q = b"'$guideline_id'(product,d,1,ref(3),[A,B])"
        body = b" :- guideline_entity(actual,A,tablet,countable), guideline_cardinality(actual,A,na,geq,2), guideline_entity(actual,B,patient,countable).\n"
        old = b"guideline_entity(actual," + q + b",d,countable)" + body + b"guideline_cardinality(actual," + q + b",na,geq,4)" + body
        p = nt.semantic_document(doc(old), UNITS)
        self.assertEqual(p.reverse(), doc(old, OLD+ADDED))
        self.assertIn(b"guideline_unit(actual," + q + b",day) :- ", p.data)
        for c in p.changes:
            self.assertEqual(c.new.head, c.old.head)

    def test_inverse_rejects_changed_prediction(self):
        old = b"guideline_event(actual,A,wait) :- guideline_entity(actual,B,day,countable), guideline_cardinality(actual,B,na,geq,4).\n"
        p = nt.semantic_document(doc(old), UNITS)
        p.data = p.data.replace(b"geq,4,day", b"geq,5,day")
        with self.assertRaises(nt.InputError):
            p.reverse()

    def test_repeated_annotation_instances_inverse_by_quantity(self):
        old = (b"guideline_event(actual,A,wait) :- guideline_interval(actual,A,duration,B,day,none), guideline_interval(actual,A,duration,C,hour,none), "
               b"guideline_entity(actual,B,day,countable), guideline_cardinality(actual,B,na,geq,2), guideline_entity(actual,C,hour,countable), guideline_cardinality(actual,C,na,greater,1).\n")
        p = nt.semantic_document(doc(old), UNITS)
        self.assertEqual(p.reverse(), doc(old, OLD+ADDED))
        self.assertEqual(p.data.count(b"guideline_satisfies("), 2)

    def test_query_answers_share_renumbered_variables(self):
        q = question(b"','(guideline_entity(actual,A,hour,countable),','(guideline_cardinality(actual,A,na,geq,4),guideline_event(actual,B,wait)))")
        q = q.replace(b"answers([])", b"answers([answer(B,wh(what))])")
        p = nt.semantic_query(q, UNITS)
        self.assertIn(b"guideline_event(actual,E,wait)", p.data)
        self.assertIn(b"answers([answer(E,wh(what))])", p.data)
        self.assertEqual(p.data.splitlines()[:3], q.splitlines()[:3])

    def test_certify_swap_tracks_source_lines_past_unit_heads(self):
        body = (b"guideline_entity(actual,q,day,countable).\n"
                b"guideline_cardinality(actual,q,na,eq,2).\n"
                b"guideline_event(actual,e,wait).\n"
                b"guideline_arg(actual,e,1,q).\n")
        old = doc(body)
        lines = old.splitlines()
        target = "guidelines/g/pl/d.pl"
        for first in range(lines.index(body.splitlines()[0]), len(lines)-1):
            second = first + 1
            row = (f"move\tdoc\t{target}\tswap-lines\t{first}\t{second}\t1\tclauses(1)\\n\n").encode()
            base = {target: nt.Blob(old), "guidelines/g/vocabulary.tsv": nt.Blob(TABLE),
                    "tests/certify/cases.tsv": nt.Blob(row)}
            oracle = nt.Oracle(base).build()
            new_lines = oracle.expected[target].data.splitlines()
            # Select the same two original clause occurrences, not a fixed offset.
            expected = [new_lines.index(lines[i]) for i in (first,second)]
            result = oracle.expected["tests/certify/cases.tsv"].data.split(b"\t")
            self.assertEqual([int(result[4]),int(result[5])], expected)

    def test_certify_replace_first_keeps_its_source_head(self):
        body = (b"guideline_entity(actual,q,day,countable).\n"
                b"guideline_cardinality(actual,q,na,eq,2).\n"
                b"guideline_recurrence(actual,e,q,day).\n")
        target = "guidelines/g/pl/d.pl"
        row = (f"unit\tdoc\t{target}\treplace-first\tq,day).\tq,hour).\t1\tclauses(1)\\n\n").encode()
        base = {target: nt.Blob(doc(body)), "guidelines/g/vocabulary.tsv": nt.Blob(TABLE),
                "tests/certify/cases.tsv": nt.Blob(row)}
        oracle = nt.Oracle(base).build()
        fields = oracle.expected["tests/certify/cases.tsv"].data.split(b"\t")
        self.assertEqual(fields[4:6], [b"e,q,day).", b"e,q,hour)."])
        mutated = oracle.expected[target].data.replace(fields[4],fields[5],1)
        self.assertIn(b"guideline_unit(actual,q,day).",mutated)
        self.assertIn(b"guideline_recurrence(actual,e,q,hour).",mutated)
        base.pop("guidelines/g/vocabulary.tsv")
        self.assertEqual(nt.Oracle(base).build().expected["tests/certify/cases.tsv"].data,row)

    def test_certify_query_edits_follow_converted_projection_links(self):
        pp = b"guideline_pp(actual,B,at,A)"
        rec = b"guideline_recurrence(actual,B,A,hour)"
        pair = b"','("+pp+b","+rec+b")"
        q = question(b"','(guideline_entity(actual,A,hour,countable),','(guideline_cardinality(actual,A,na,geq,4),"+pair+b"))")
        target = "guidelines/g/queries/pl/q.pl"
        rows = []
        for name,old,new in (("query-v2-annotation-altered",rec,rec.replace(b"hour",b"day")),
                             ("query-v2-annotation-dropped",pair,pp)):
            rows.append(name.encode()+b"\tquery\t"+target.encode()+b"\treplace-first\t"+old+b"\t"+new+b"\t1\tprojection\\n\n")
        base = {target: nt.Blob(q), "guidelines/g/vocabulary.tsv": nt.Blob(TABLE),
                "tests/certify/cases.tsv": nt.Blob(b"".join(rows))}
        oracle = nt.Oracle(base).build()
        fields = [r.split(b"\t") for r in oracle.expected["tests/certify/cases.tsv"].data.splitlines()]
        new_pp = b"guideline_pp(actual,E,at,A)"
        new_rec = b"guideline_recurrence(actual,E,A,D)"
        self.assertEqual(fields[0][4:6],[new_rec,b"guideline_recurrence(actual,E,A,day)"])
        self.assertEqual(fields[1][4:6],[b"','("+new_pp+b","+new_rec+b")",new_pp])

    def test_o2_arbitrary_payload_and_multiple_blocks_property(self):
        rng = random.Random(0xD2)
        for count in range(1, 5):
            for n in (0, 1, 27, 250):
                payload = bytes(rng.choices(range(256), k=n))
                old = (b"% d\n" + OLD + b"% opaque\n" + payload + b"\n") * count
                new = nt.document_transform(old)
                self.assertEqual(new, old.replace(OLD, OLD+ADDED))
                self.assertEqual(new.replace(OLD+ADDED,OLD), old)
                self.assertEqual(len(new)-len(old), 68*count)

    def test_trace_dangling_clause_false_builtin_and_true_tamper(self):
        o = nt.synthetic_oracle()
        p = "guidelines/g/queries/traces/q.pl"
        original = o.expected[p]
        for before, after, semantic_error in [
                (b"geq,4,hour,geq,4,hour",b"geq,3,hour,geq,4,hour",True),
                (b"geq,4,hour,geq,4,hour",b"geq,5,hour,geq,4,hour",False)]:
            tree = dict(o.expected)
            self.assertIn(before, original.data)
            tree[p] = nt.Blob(original.data.replace(before,after,1))
            self.assertTrue(o.compare(tree))
            self.assertEqual(bool(o.trace_problems(tree)), semantic_error)
        tree = dict(o.expected)
        pos = original.data.index(b"clause_sha256(")
        head, tail = original.data[:pos], original.data[pos:]
        tail = nt.re.sub(rb"[0-9a-f]{64}", b"0"*64, tail, count=1)
        tree[p] = nt.Blob(head+tail)
        self.assertTrue(o.trace_problems(tree))

    def test_trace_golden_digest_checked_when_tree_trace_absent(self):
        o = nt.synthetic_oracle()
        tree = dict(o.expected)
        prefix = "tests/queries/red/missing-trace/"
        golden = prefix + "traces-golden/q.pl"
        tree[prefix + "tree/guidelines/g/pl/d.pl"] = tree["guidelines/g/pl/d.pl"]
        original = tree["guidelines/g/queries/traces/q.pl"]
        tree[golden] = original
        self.assertFalse(o.trace_problems(tree))
        at = original.data.index(b"clause_sha256(")
        changed = original.data[:at] + nt.re.sub(rb"[0-9a-f]{64}", b"0"*64, original.data[at:], count=1)
        tree[golden] = nt.Blob(changed)
        self.assertTrue(o.trace_problems(tree))

    def test_rulings_cannot_add_remove_or_touch_frozen(self):
        raw = bytes(range(256))
        self.assertEqual(nt.literal_bytes(nt.escaped_bytes(raw)), raw)
        o = nt.synthetic_oracle()
        p = "tests/red/r.expect"
        r = {"family":"O4", "item":"r", "path":p, "mode":"edit", "base_sha256":nt.sha(o.base[p].data).decode(),
             "reason":"MAIN explicit change", "edits":[{"old":b"unsupported", "new":b"input", "count":1}]}
        for changed in (dict(r,base_sha256="0"*64),dict(r,mode="new"),dict(r,mode="delete"),dict(r,path="tests/s1-compiler/frozen")):
            with self.assertRaises(nt.InputError):
                nt.Oracle(o.base,rulings=[changed]).build()
        with self.assertRaises(nt.InputError):
            nt.safe_path("../outside")
        with self.assertRaises(nt.InputError):
            nt.load_rulings(b"# header\nO4\tr\ttests/red/r.expect\tx\ty\t0\treason\n")

    def test_cli_bad_tree_input_error_and_family_names(self):
        o = nt.synthetic_oracle()
        bad = dict(o.expected)
        bad["guidelines/schema.pl"] = nt.Blob(b"wrong schema\n")
        with patch.object(nt,"base_blobs",return_value=o.base),patch.object(nt,"rulings_for",return_value=[]), \
             patch.object(nt,"working_blobs",return_value=(bad,set())),contextlib.redirect_stdout(io.StringIO()) as out:
            self.assertEqual(nt.main(["nt1-churn.py","base",str(Path(__file__).resolve().parents[3])]),1)
            self.assertIn("O8",out.getvalue())
            self.assertIn("unpredicted",out.getvalue())
        with contextlib.redirect_stdout(io.StringIO()):
            self.assertEqual(nt.main(["nt1-churn.py"]),1)
            with patch.object(nt,"base_blobs",side_effect=nt.InputError("no base")):
                self.assertEqual(nt.main(["nt1-churn.py","no-base",str(Path(__file__).resolve().parents[3])]),1)

    def test_arbitrary_precision_exceeds_python_default_decimal_limit(self):
        n = 10**5000 + 37
        g = nt.read_term(b"guideline_satisfies(exactly," + str(n).encode() + b",year,exactly," + str(n*12).encode() + b",month)")
        self.assertTrue(nt.builtin_holds(g))
        self.assertFalse(nt.satisfies(("exactly", n, "year", "exactly", n*12+1, "month")))

    def test_semantic_stream_uses_only_schema_and_clauses(self):
        c = b"guideline_event(actual,e,wait).\n"
        self.assertEqual(nt.semantic_stream(doc(c)),b"guideline_schema_version(1).\n"+c)
        self.assertEqual(nt.semantic_stream(doc(c,OLD+ADDED)),nt.semantic_stream(doc(c)))

    def test_schema_grid_detects_exception_duplicate_and_binding(self):
        cases = nt.schema_cases()
        self.assertTrue(any(a[0] is None for a in cases))
        self.assertTrue(any(a[1] == 1.5 for a in cases))
        for text in (b"bad 1 [ok,ok]\n", b"bad 1 error(type_error(integer,a))\n", b"bad 1 [bound]\n"):
            good = nt.subprocess.CompletedProcess([],0,b"swi(9,2,9,[])\n",b"")
            bad = nt.subprocess.CompletedProcess([],0,text,b"")
            with patch.object(nt.subprocess,"run",side_effect=[good,bad]),self.assertRaises(nt.InputError):
                nt.schema_differential(Path("schema.pl"))


MODEL_FAULTS = [
('within-inclusive','return s in ("eq", "exactly", "geq", "greater") and x >= y','return s in ("eq", "exactly", "geq", "greater") and x > y'),
('greater-reflexive','(s == "greater" and x >= y)','(s == "greater" and x > y)'),
('integer-type','if type(x) is not int or type(y) is not int:', 'if False:'),
('scale-day','"day": ("time", 86400)','"day": ("time", 86401)'),
('scale-crossgroup','if u not in SCALES or v not in SCALES or SCALES[u][0] != SCALES[v][0]:','if u not in SCALES or v not in SCALES:'),
('convert-less','CONVERTIBLE = {b"exactly", b"geq", b"greater", b"leq", b"less"}','CONVERTIBLE = {b"exactly", b"geq", b"greater", b"leq"}'),
('annotation-link','args[uslot] = converted[args[qslot]][2]','args[uslot] = args[uslot]'),
('unit-head','if unit:\n            out.append(render_clause(unit))','if False:\n            out.append(render_clause(unit))'),
('entity-kept','if is_term(g, b"guideline_entity", 4) and g.args[1] in converted and g.args[1] in times:\n            continue','if is_term(g, b"guideline_entity", 4) and g.args[1] in converted and g.args[1] in times:\n            out.append(Planned(g, i))\n            continue'),
('witness-entity-kept','if witness and unit and is_term(g, b"guideline_entity", 4)','if False and witness and unit and is_term(g, b"guideline_entity", 4)'),
('stream-document-included','(b"%", b":- ", b"guideline_document(")','(b"%", b":- ")'),
('frozen-disabled','self.frozen = {p for p in base if p.startswith(FROZEN)}','self.frozen = set()'),
('mode-ignored','elif self.expected[path].mode != actual[path].mode:','elif False:'),
('inverse-bypassed','def reverse(self):\n        # Inverse uses','def reverse(self):\n        return document_transform(self.original)\n        # Inverse uses'),
('swap-unit-offset','shift += int(change.unit is not None)','shift += 0'),
('anchor-shadow','def reanchor_document(self, path, needle, replacement):\n        old, new =','def reanchor_document(self, path, needle, replacement):\n        return needle, replacement\n        old, new ='),
('query-edit-links','def reanchor_query(self, path, needle, replacement):\n        projection =','def reanchor_query(self, path, needle, replacement):\n        return needle, replacement\n        projection ='),
]


class NT1ModelFaultPlants(unittest.TestCase):
    def test_contract_suite_kills_model_faults(self):
        global nt
        baseline = nt
        path = Path(__file__).with_name("nt1-churn.py")
        source = path.read_text()
        killed = []
        for i, (name, old, new) in enumerate(MODEL_FAULTS):
            with self.subTest(fault=name):
                self.assertEqual(source.count(old), 1, "mutation anchor")
                module = types.ModuleType("nt1_model_fault_" + str(i))
                module.__file__ = str(path)
                sys.modules[module.__name__] = module
                try:
                    exec(compile(source.replace(old, new), str(path), "exec"), module.__dict__)
                    nt = module
                    suite = unittest.TestSuite(unittest.defaultTestLoader.loadTestsFromTestCase(c)
                                               for c in (NT1OracleContract, NT1OracleBoundaries))
                    log = io.StringIO()
                    result = unittest.TextTestRunner(stream=log, verbosity=0).run(suite)
                    self.assertFalse(result.wasSuccessful(), "planted oracle fault survived")
                    killed.append(name)
                finally:
                    nt = baseline
                    del sys.modules[module.__name__]
        print("oracle-model-plants: " + str(len(killed)) + "/" + str(len(MODEL_FAULTS)) + " killed; " + ",".join(killed))


if __name__ == "__main__":
    unittest.main(verbosity=2)
