#!/usr/bin/env python3
"""Small producer controls; --consumer adds the four committed CDC patient worlds."""

import argparse
import hashlib
import math
from pathlib import Path
import re
import subprocess
import tempfile


def invoke(args, expected=0):
    proc = subprocess.run(args, text=True, capture_output=True, timeout=180)
    if proc.returncode != expected:
        raise AssertionError(f"rc {proc.returncode}, expected {expected}: {args}\n{proc.stdout}\n{proc.stderr}")
    return proc


def identity(root, composition):
    schema = (root / "guidelines/schema.pl").read_bytes()
    digest = hashlib.sha256()
    digest.update(f"schema\0{len(schema)}\0{hashlib.sha256(schema).hexdigest()}\n".encode())
    for listed in composition.read_text().splitlines():
        if not listed or listed.startswith("#"):
            continue
        data = (root / listed).read_bytes()
        digest.update(f"file\0{listed}\0{len(data)}\0{hashlib.sha256(data).hexdigest()}\n".encode())
    return digest.hexdigest()


def producer(root, composition, out, expected=0):
    return invoke([
        "swipl", "-q", "-f", "none", "-F", "none", "-s",
        str(root / "prototype/sb1/producer.prolog"), "-g", "sb1_main", "-t", "halt(9)",
        "--", "--root", str(root), str(composition), str(out),
    ], expected)


def reference(root, composition, recipe, out):
    result = invoke([
        "swipl", "-q", "-f", "none", "-F", "none", "-s",
        str(root / "prototype/sb1/producer-tests/reference.pl.in"),
        "-g", "sb1_test_main", "-t", "halt(9)", "--", str(root), str(composition),
        str(recipe), str(out / "model.txt"), str(out / "certificate.txt"),
    ])
    assert not result.stderr, result.stderr
    return result.stdout.strip()


def check_outputs(root, composition, out):
    cert = (out / "certificate.txt").read_text().splitlines()
    assert cert[:2] == ["sb1-certificate 0", f"composition {identity(root, composition)}"]
    phases = (out / "producer-phases.tsv").read_text().splitlines()
    assert phases[0] == "phase\tseconds\tcpu"
    assert [row.split("\t")[0] for row in phases[1:]] == ["load", "eval", "certificate", "write"]
    assert all(math.isfinite(float(v)) and float(v) >= 0
               for row in phases[1:] for v in row.split("\t")[1:])
    model = (out / "model.txt").read_text().splitlines()
    assert len(cert) - 2 == len(model)
    assert all(line.endswith(".") for line in model)
    return cert, model


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("root", type=Path)
    parser.add_argument("--consumer", action="store_true")
    args = parser.parse_args()
    root = args.root.resolve()
    scratch = root / ".scratch/sb1-producer-tests"
    scratch.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(dir=scratch) as temp:
        temp = Path(temp)
        section = (root / "docs/REFERENCE.md").read_text().split("### Loading a whole knowledge base\n", 1)[1].split("\n### ", 1)[0]
        blocks = re.findall(r"```prolog\n(.*?)```", section, re.S)
        assert len(blocks) == 3
        recipe = temp / "recipe.pl"
        recipe.write_text(blocks[0] + blocks[1])
        tiny = temp / "tiny.tsv"
        tiny.write_text("# identity excludes this comment\nprototype/sb1/producer-tests/tiny.pl.in\n")
        out = temp / "tiny-out"
        result = producer(root, tiny, out)
        assert not result.stdout and not result.stderr, (result.stdout, result.stderr)
        cert, model = check_outputs(root, tiny, out)
        print("tiny:", reference(root, tiny, recipe, out))
        rows = [row.split("\t") for row in cert[2:]]
        levels = {row[0]: int(row[3]) for row in rows}
        assert levels["guideline_property(actual,a,p,pos)"] == 1
        assert levels["guideline_property(actual,a,q,pos)"] == 2
        assert levels["guideline_property(actual,a,r,pos)"] == 3
        assert levels["guideline_property(actual,z,ratio,pos)"] == 2
        assert levels["guideline_property(actual,a,time,pos)"] == 1
        assert levels["guideline_exclusion(tiny)"] == 1
        assert levels["guideline_cardinality(actual,z,na,exactly,rational(3,2))"] == 1
        assert "guideline_property(actual,a,clear,pos)." in model
        assert "guideline_property(actual,c,clear,pos)." not in model
        assert all(int(row[3]) == 0 for row in rows if row[0].startswith(("guideline_document(", "guideline_supersession(")))
        reordered = [r for r in rows if r[0] == "guideline_arg(actual,e,3,a)"]
        assert len(reordered) == 1 and reordered[0][2] == "[=(v1,e),=(v2,a)]", reordered
        again = temp / "tiny-again"
        producer(root, tiny, again)
        for name in ["model.txt", "certificate.txt"]:
            assert (out / name).read_bytes() == (again / name).read_bytes(), name
        immutable = producer(root, tiny, out, 1)
        assert immutable.stderr.startswith("sb1-producer write ") and immutable.stderr.count("\n") == 1, immutable.stderr
        assert (out / "certificate.txt").read_text().splitlines() == cert
        bad = temp / "bad.tsv"
        bad.write_text("missing-producer-input.pl\n")
        missing = producer(root, bad, temp / "missing-out", 1)
        assert missing.stderr.startswith("sb1-producer load ") and missing.stderr.count("\n") == 1, missing.stderr
        foreign = temp / "foreign.pl"
        foreign.write_text("guideline_schema_version(2).\nguideline_exclusion(x) :- halt.\n")
        bad.write_text(f"{foreign}\n")
        denied = producer(root, bad, temp / "foreign-out", 1)
        assert denied.stderr.startswith("sb1-producer load "), denied.stderr
        old = temp / "old.pl"
        old.write_text("guideline_schema_version(1).\n")
        bad.write_text(f"{old}\n")
        mixed = producer(root, bad, temp / "identifier-out", 1)
        assert mixed.stderr.startswith("sb1-producer load "), mixed.stderr
        for tag, extra in [("variable-clause", "_.\n"), ("variable-directive", ":- multifile(_).\n")]:
            malformed = temp / f"{tag}.pl"
            malformed.write_text("guideline_schema_version(2).\n" + extra)
            bad.write_text(f"{malformed}\n")
            rejected = producer(root, bad, temp / f"{tag}-out", 1)
            assert rejected.stderr.startswith("sb1-producer load "), rejected.stderr
        negative = temp / "negative.pl"
        negative.write_text("guideline_schema_version(2).\nguideline_exclusion(loop) :- \\+ guideline_exclusion(loop).\n")
        bad.write_text(f"{negative}\n")
        refused = producer(root, bad, temp / "negative-out", 1)
        assert refused.stderr.startswith("sb1-producer certificate "), refused.stderr
        agenda = invoke([
            "swipl", "-q", "-f", "none", "-F", "none", "-s", str(root / "prototype/sb1/producer.prolog"),
            "-s", str(root / "prototype/sb1/producer-tests/agenda.pl.in"),
            "-g", "sb1_agenda_tests", "-t", "halt(9)",
        ])
        assert agenda.stdout == "agenda: founded base + unfounded residual passed\n" and not agenda.stderr, agenda
        scaling = invoke([
            "swipl", "-q", "-f", "none", "-F", "none", "-s", str(root / "prototype/sb1/producer.prolog"),
            "-s", str(root / "prototype/sb1/producer-tests/scaling.pl.in"),
            "-g", "sb1_scaling_tests", "-t", "halt(9)", "--", str(root),
        ])
        assert scaling.stdout == "scaling: 36 keyed query pairs; 136 FIFO/heap cases passed\n" and not scaling.stderr, scaling
        print(scaling.stdout.strip())
        for stage, fault in [("eval", "sb1_test_eval_fault"),
                             ("resource", "error(resource_error(table_space),context(sb1_control,allocation))")]:
            goal = f"catch(sb1_producer:sb1_stage(eval,throw({fault}),_),E,sb1_producer:sb1_failure(E))"
            failure = invoke([
                "swipl", "-q", "-f", "none", "-F", "none", "-s", str(root / "prototype/sb1/producer.prolog"),
                "-g", goal, "-t", "halt(9)",
            ], 1)
            assert not failure.stdout and failure.stderr.startswith(f"sb1-producer {stage} "), failure
            assert failure.stderr.count("\n") == 1, failure.stderr
        print("controls: identity, canonical writer, correlated NAF, founded alternative, unfounded residual, minimal levels, valuation order, immutable output, five failure stages passed (eval/resource = injected exceptions)")
        if args.consumer:
            documents = sorted((root / "guidelines/cdc-2022-opioid/pl").glob("*.pl"))
            assert documents
            for name in ["adult", "minor", "cancer", "dose"]:
                world = root / ("tests/queries/green/dose-arithmetic/tree/guidelines/fx/pl/world.pl" if name == "dose" else f"tests/consumer/{name}.pl.in")
                composition = temp / f"{name}.tsv"
                composition.write_text("".join(f"{path.relative_to(root)}\n" for path in [*documents, world]))
                world_out = temp / f"{name}-out"
                result = producer(root, composition, world_out)
                assert not result.stdout and not result.stderr, (name, result.stdout, result.stderr)
                check_outputs(root, composition, world_out)
                print(f"{name}:", reference(root, composition, recipe, world_out))
    scratch.rmdir()


if __name__ == "__main__":
    main()
