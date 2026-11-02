#!/usr/bin/env python3
"""Exact model + expanded-support comparison with the frozen transport-0 producer."""

import argparse
import csv
import hashlib
from pathlib import Path
import shutil
import subprocess

BASE = "4980bbb20e21925d4f7d62cf420f22a2fd2bf00d"
FAMILIES = ("F1a", "F1b", "F2", "F3", "F4", "F5a", "F5b", "F5c", "MX")


def digest(path):
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def run(argv, log):
    with log.open("wb") as stream:
        proc = subprocess.run(argv, stdout=stream, stderr=subprocess.STDOUT, timeout=900)
    if proc.returncode:
        raise AssertionError(f"rc={proc.returncode}: {argv}; log={log}")


def producer(root, source, composition, out, log):
    run(["swipl", "-q", "-f", "none", "-F", "none", "-s", str(source),
         "-g", "sb1_main", "-t", "halt(9)", "--", "--root", str(root),
         str(composition), str(out)], log)
    assert log.stat().st_size == 0, log


def expand(directory):
    model = (directory / "model.txt").read_text().splitlines()
    certificate = (directory / "certificate.txt").read_text().splitlines()
    version = certificate[0]
    assert version in ("sb1-certificate 0", "sb1-certificate 1")
    claims = []
    indices = []
    for line in certificate[2:]:
        fields = line.split("\t")
        assert len(fields) == 4
        if version == "sb1-certificate 1":
            index = int(fields[0])
            indices.append(index)
            fields[0] = model[index - 1][:-1]
        claims.append("\t".join(fields))
    assert len(claims) == len(model)
    if indices:
        assert indices == list(range(1, len(model) + 1))
    path = directory / "claims.txt"
    path.write_text("\n".join(["sb1-certificate 0", certificate[1], *claims]) + "\n")
    return path


def compare(root, before, after, composition, out, writer, label):
    producer(root, before, composition, out / "before", out / "before.log")
    producer(root, after, composition, out / "after", out / "after.log")
    for arm in ("before", "after"):
        expand(out / arm)
    for name in ("model.txt", "claims.txt"):
        a, b = out / "before" / name, out / "after" / name
        rc = subprocess.run(["cmp", "--silent", str(a), str(b)]).returncode
        writer.writerow((label, name, rc, a.stat().st_size, digest(a), digest(b)))
        assert rc == 0, (label, name)
    model = out / "after/model.txt"
    print(f"{label}: model+support identical; {model.read_bytes().count(b'\n')} facts", flush=True)
    for arm in ("before", "after"):
        for name in ("model.txt", "certificate.txt", "claims.txt"):
            (out / arm / name).unlink()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("root", type=Path)
    parser.add_argument("--ckc", required=True, type=Path)
    parser.add_argument("--out", required=True, type=Path)
    args = parser.parse_args()
    root, ckc, out = args.root.resolve(), args.ckc.resolve(), args.out.resolve()
    out.mkdir(parents=True, exist_ok=False)
    work = root / "prototype/sb1/checker/target" / f"producer-compare-{out.name}"
    work.mkdir(parents=True, exist_ok=False)
    (work / "guidelines").mkdir()
    shutil.copy2(root / "guidelines/schema.pl", work / "guidelines/schema.pl")
    shutil.copytree(root / "guidelines/cdc-2022-opioid/pl", work / "guidelines/cdc-2022-opioid/pl")
    shutil.copytree(root / "bench/sb", work / "bench/sb")
    for listed in [*(f"tests/consumer/{n}.pl.in" for n in ("adult", "minor", "cancer")),
                   "tests/queries/green/dose-arithmetic/tree/guidelines/fx/pl/world.pl"]:
        destination = work / listed
        destination.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(root / listed, destination)
    cases = work / "rust/target/bench"
    cases.mkdir(parents=True)
    before = out / "transport-0.prolog"
    before.write_bytes(subprocess.check_output(["git", "-C", str(root), "show", f"{BASE}:prototype/sb1/producer.prolog"]))
    after = root / "prototype/sb1/producer.prolog"
    (out / "identities.tsv").write_text(
        f"base\t{BASE}\ntransport_0_sha256\t{digest(before)}\n"
        f"producer_sha256\t{digest(after)}\nckc_sha256\t{digest(ckc)}\n")
    with (out / "equivalence.tsv").open("w") as stream:
        writer = csv.writer(stream, delimiter="\t", lineterminator="\n")
        writer.writerow(("case", "file", "cmp_rc", "bytes", "before_sha256", "after_sha256"))
        documents = sorted((root / "guidelines/cdc-2022-opioid/pl").glob("*.pl"))
        assert documents
        for name in ("adult", "minor", "cancer", "dose"):
            label = f"consumer-{name}"
            case, report = cases / label, out / label
            case.mkdir()
            report.mkdir()
            world = root / ("tests/queries/green/dose-arithmetic/tree/guidelines/fx/pl/world.pl"
                            if name == "dose" else f"tests/consumer/{name}.pl.in")
            composition = case / "composition.tsv"
            composition.write_text("".join(f"{path.relative_to(root)}\n" for path in (*documents, world)))
            compare(work, before, after, composition, report, writer, label)
            stream.flush()
            shutil.rmtree(case)
        for g in (1, 2, 4):
            for family in FAMILIES:
                label = f"{family}-{g}"
                case, report = cases / label, out / label
                case.mkdir()
                report.mkdir()
                profile = "p-adult" if family in ("F1a", "F1b") else f"p-{family.lower()}"
                run([str(ckc), "bench", "gen", family, str(g), "--root", str(work),
                     "--patient", f"bench/sb/fixtures/patients/{profile}.pl.in", "--out", str(case / "composition")],
                    report / "generate.log")
                composition = case / "composition/composition.tsv"
                shutil.copy2(case / "composition/manifest.tsv", report / "manifest.tsv")
                shutil.copy2(composition, report / "composition.tsv")
                compare(work, before, after, composition, report, writer, label)
                stream.flush()
                shutil.rmtree(case)
    before.unlink()
    shutil.rmtree(work)
    with (out / "equivalence.tsv").open() as stream:
        rows = list(csv.DictReader(stream, delimiter="\t"))
    assert len(rows) == 62 and all(row["cmp_rc"] == "0" for row in rows)
    assert digest(after) == dict(line.split("\t", 1) for line in (out / "identities.tsv").read_text().splitlines())["producer_sha256"]
    print("comparison: 31 compositions; 62 exact model/support comparisons passed")


if __name__ == "__main__":
    main()
