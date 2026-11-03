#!/usr/bin/env python3
"""Exact orchestrator answers vs the frozen whole-composition producer + model queries."""

import argparse
import csv
import hashlib
from pathlib import Path
import shutil
import subprocess

BASE = "6c9320a7"
FAMILIES = ("F1a", "F1b", "F2", "F3", "F4", "F5a", "F5b", "F5c", "MX")


def digest(path):
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def run(argv, stdout, stderr, timeout=900):
    with stdout.open("wb") as output, stderr.open("wb") as error:
        result = subprocess.run([str(arg) for arg in argv], stdout=output, stderr=error, timeout=timeout)
    assert result.returncode == 0, (result.returncode, argv, stderr.read_text()[:2000])
    assert stderr.stat().st_size == 0, (argv, stderr.read_text()[:2000])


def compare(root, producer, producer_after, checker, reference, composition, queries, out, writer, label):
    baseline = out / "baseline"
    run(["swipl", "--stack-limit=32g", "--table-space=32g", "-q", "-f", "none", "-F", "none",
         "-s", producer, "-g", "sb1_main", "-t", "halt(9)", "--", "--root", root,
         composition, baseline], out / "baseline.stdout", out / "baseline.stderr")
    assert (out / "baseline.stdout").stat().st_size == 0
    for name, query in queries.items():
        expected = out / f"{name}.expected"
        run(["swipl", "-q", "-f", "none", "-F", "none", "-s", reference, "-g", "main", "-t", "halt(9)",
             "--", root / "guidelines/schema.pl", baseline / "model.txt", "q-all" if name == "q-all" else query],
            expected, out / f"{name}.reference.stderr")
        for provider in ("all", "static"):
            directory = out / f"{name}-{provider}"
            actual = out / f"{name}-{provider}.actual"
            run([checker, "orchestrate", "--provider", provider, "--producer", producer_after,
                 "--root", root, composition, directory, query], actual, out / f"{name}-{provider}.stderr")
            rc = subprocess.run(["cmp", "--silent", actual, expected]).returncode
            with (directory / "selected-clauses.tsv").open() as selected:
                selected_count = sum(1 for _ in selected)
            writer.writerow((label, name, provider, rc, actual.stat().st_size, digest(expected), digest(actual),
                             selected_count, digest(directory / "model.txt"), digest(directory / "certificate.txt")))
            assert rc == 0, (label, name, provider, "answer bytes differ")
            if provider == "all":
                model_rc = subprocess.run(["cmp", "--silent", directory / "model.txt", baseline / "model.txt"]).returncode
                assert model_rc == 0, (label, name, "all-provider model differs")
            for name_to_remove in ("model.txt", "certificate.txt", "slice.pl.in"):
                (directory / name_to_remove).unlink()
    for name in ("model.txt", "certificate.txt"):
        (baseline / name).unlink()
    print(f"{label}: 6 exact answer comparisons; all-provider models byte-identical", flush=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", required=True, type=Path)
    parser.add_argument("--ckc", required=True, type=Path)
    parser.add_argument("--checker", required=True, type=Path)
    parser.add_argument("--out", required=True, type=Path)
    parser.add_argument("--only", action="append")
    args = parser.parse_args()
    root, ckc, checker, out = (p.resolve() for p in (args.root, args.ckc, args.checker, args.out))
    out.mkdir(parents=True, exist_ok=False)
    fixture = out / "fixture-root"
    (fixture / "guidelines").mkdir(parents=True)
    shutil.copy2(root / "guidelines/schema.pl", fixture / "guidelines/schema.pl")
    shutil.copytree(root / "guidelines/cdc-2022-opioid/pl", fixture / "guidelines/cdc-2022-opioid/pl")
    shutil.copytree(root / "bench/sb", fixture / "bench/sb")
    for listed in [*(f"tests/consumer/{name}.pl.in" for name in ("adult", "minor", "cancer")),
                   "tests/queries/green/dose-arithmetic/tree/guidelines/fx/pl/world.pl"]:
        destination = fixture / listed
        destination.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(root / listed, destination)
    frozen = out / "producer-before.prolog"
    frozen.write_bytes(subprocess.check_output(["git", "-C", root, "show", f"{BASE}:prototype/sb1/producer.prolog"]))
    reference = root / "prototype/sb1/checker/tests/model-answers.prolog"
    (out / "identities.tsv").write_text(
        f"base\t{BASE}\nproducer_before_sha256\t{digest(frozen)}\n"
        f"producer_after_sha256\t{digest(root / 'prototype/sb1/producer.prolog')}\n"
        f"checker_sha256\t{digest(checker)}\nckc_sha256\t{digest(ckc)}\nreference_sha256\t{digest(reference)}\n")
    producer_after = root / "prototype/sb1/producer.prolog"
    documents = sorted((fixture / "guidelines/cdc-2022-opioid/pl").glob("*.pl"))
    assert len(documents) == 492
    selected = set(args.only or ())
    compared = 0
    with (out / "equivalence.tsv").open("w") as stream:
        writer = csv.writer(stream, delimiter="\t", lineterminator="\n")
        writer.writerow(("case", "query", "provider", "cmp_rc", "bytes", "expected_sha256", "actual_sha256",
                         "selected_clauses", "model_sha256", "certificate_sha256"))
        for name in ("adult", "minor", "cancer", "dose"):
            label = f"consumer-{name}"
            if selected and label not in selected:
                continue
            case = out / label
            case.mkdir()
            world = fixture / ("tests/queries/green/dose-arithmetic/tree/guidelines/fx/pl/world.pl" if name == "dose"
                               else f"tests/consumer/{name}.pl.in")
            composition = case / "composition.tsv"
            composition.write_text("".join(f"{path.relative_to(fixture)}\n" for path in (*documents, world)))
            queries = {"q-all": fixture / "bench/sb/queries/q-all.txt",
                       "q-one": fixture / "bench/sb/queries/q-one.pl.in",
                       "q-patient": fixture / "bench/sb/queries/q-patient.pl.in"}
            compare(fixture, frozen, producer_after, checker, reference, composition, queries, case, writer, label)
            stream.flush()
            compared += 1
        for g in (1, 2, 4):
            for family in FAMILIES:
                label = f"{family}-{g}"
                if selected and label not in selected:
                    continue
                case = out / label
                case.mkdir()
                composition_dir = fixture / "rust/target/bench" / label
                composition_dir.parent.mkdir(parents=True, exist_ok=True)
                profile = "p-adult" if family in ("F1a", "F1b") else f"p-{family.lower()}"
                with (case / "generate.log").open("wb") as log:
                    result = subprocess.run([ckc, "bench", "gen", family, str(g), "--root", fixture,
                                             "--patient", f"bench/sb/fixtures/patients/{profile}.pl.in",
                                             "--out", composition_dir], stdout=log, stderr=subprocess.STDOUT, timeout=900)
                assert result.returncode == 0, (label, result.returncode, (case / "generate.log").read_text()[:2000])
                queries = {name: composition_dir / "queries" / ("q-all.txt" if name == "q-all" else f"{name}.pl")
                           for name in ("q-all", "q-one", "q-patient")}
                shutil.copy2(composition_dir / "manifest.tsv", case / "manifest.tsv")
                compare(fixture, frozen, producer_after, checker, reference, composition_dir / "composition.tsv", queries, case, writer, label)
                stream.flush()
                compared += 1
                shutil.rmtree(composition_dir)
    with (out / "equivalence.tsv").open() as stream:
        rows = list(csv.DictReader(stream, delimiter="\t"))
    assert len(rows) == compared * 6 and all(row["cmp_rc"] == "0" for row in rows)
    if not selected:
        assert compared == 31 and len(rows) == 186
    shutil.rmtree(fixture)
    print(f"comparison: compositions={compared}; exact_answers={len(rows)}; all-provider exact models={compared * 3}")


if __name__ == "__main__":
    main()
