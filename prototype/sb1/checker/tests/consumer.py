#!/usr/bin/env python3
"""Producer → checker controls on all four committed CDC patient worlds; no timing grade."""

import argparse
import hashlib
from pathlib import Path
import subprocess


def invoke(argv):
    result = subprocess.run([str(arg) for arg in argv], capture_output=True, timeout=180)
    assert result.returncode == 0, (argv, result.returncode, result.stdout, result.stderr)
    assert not result.stderr, (argv, result.stderr)
    return result.stdout


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", required=True, type=Path)
    parser.add_argument("--producer", required=True, type=Path)
    parser.add_argument("--checker", required=True, type=Path)
    parser.add_argument("--out", required=True, type=Path)
    args = parser.parse_args()
    root = args.root.resolve()
    args.out.mkdir(parents=True, exist_ok=False)
    assert b"10.0.2" in invoke(["swipl", "--version"])
    documents = sorted((root / "guidelines/cdc-2022-opioid/pl").glob("*.pl"))
    assert len(documents) == 492, len(documents)
    reference = Path(__file__).with_name("model-answers.prolog")
    query = root / "guidelines/cdc-2022-opioid/queries/pl/category-a-recommendations.pl"
    summary = ["world\tfacts\tq_all_rows\tmodel_sha256\tq_all_sha256\n"]
    for name in ["adult", "minor", "cancer", "dose"]:
        case = args.out / name
        case.mkdir()
        world = root / ("tests/queries/green/dose-arithmetic/tree/guidelines/fx/pl/world.pl" if name == "dose" else f"tests/consumer/{name}.pl.in")
        composition = case / "composition.tsv"
        composition.write_text("".join(f"{path.relative_to(root)}\n" for path in [*documents, world]))
        generated = case / "producer"
        assert not invoke(["swipl", "-q", "-f", "none", "-F", "none", "-s", args.producer,
                           "-g", "sb1_main", "-t", "halt(9)", "--", "--root", root, composition, generated])
        model = generated / "model.txt"
        certificate = generated / "certificate.txt"
        facts = len(model.read_bytes().splitlines())
        cmd = [args.checker, "--root", root, composition, model, certificate]
        assert invoke(cmd) == f"accepted {facts} facts\n".encode()
        q_all = case / "q-all.txt"
        q_all.write_text("sb1-q-all 0\n")
        answer = invoke([*cmd, q_all])
        for label, query_arg, native in [("q-all", "q-all", answer), ("q-one", query, invoke([*cmd, query]))]:
            expected = invoke(["swipl", "-q", "-f", "none", "-F", "none", "-s", reference,
                               "-g", "main", "-t", "halt(9)", "--", root / "guidelines/schema.pl", model, query_arg])
            (case / f"{label}.actual").write_bytes(native)
            (case / f"{label}.expected").write_bytes(expected)
            assert native == expected, (name, label, "answer bytes differ")
        summary.append(f"{name}\t{facts}\t{len(answer.splitlines())}\t{hashlib.sha256(model.read_bytes()).hexdigest()}\t{hashlib.sha256(answer).hexdigest()}\n")
        print(f"{name}: accepted {facts} facts; q-all {len(answer.splitlines())} rows + q-one = independent model query", flush=True)
    (args.out / "results.tsv").write_text("".join(summary))


if __name__ == "__main__":
    main()
