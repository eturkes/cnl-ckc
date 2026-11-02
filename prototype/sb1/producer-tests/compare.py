#!/usr/bin/env python3
"""Compare the producer with its frozen pre-repair revision; all outputs are local."""

import argparse
import csv
import hashlib
from pathlib import Path
import shutil
import subprocess

BASE = "d8f92c21bb04f49b2cd79908dc4954ffd00c8b9b"
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


def compare(root, before, after, composition, out, writer, label):
    producer(root, before, composition, out / "before", out / "before.log")
    producer(root, after, composition, out / "after", out / "after.log")
    for name in ("model.txt", "certificate.txt"):
        a, b = out / "before" / name, out / "after" / name
        rc = subprocess.run(["cmp", "--silent", str(a), str(b)]).returncode
        writer.writerow((label, name, rc, a.stat().st_size, digest(a), digest(b)))
        assert rc == 0, (label, name)
    model = out / "after/model.txt"
    print(f"{label}: model+certificate identical; {model.read_bytes().count(b'\n')} facts", flush=True)
    for arm in ("before", "after"):
        for name in ("model.txt", "certificate.txt"):
            (out / arm / name).unlink()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("root", type=Path)
    parser.add_argument("--ckc", required=True, type=Path)
    parser.add_argument("--out", required=True, type=Path)
    args = parser.parse_args()
    root, ckc, out = args.root.resolve(), args.ckc.resolve(), args.out.resolve()
    out.mkdir(parents=True, exist_ok=False)
    work = root / "rust/target/bench" / f"builder-4-{out.name}"
    work.mkdir(parents=True, exist_ok=False)
    before = out / "pre-repair.prolog"
    before.write_bytes(subprocess.check_output(["git", "-C", str(root), "show", f"{BASE}:prototype/sb1/producer.prolog"]))
    after = root / "prototype/sb1/producer.prolog"
    (out / "identities.tsv").write_text(
        f"base\t{BASE}\npre_repair_sha256\t{digest(before)}\n"
        f"producer_sha256\t{digest(after)}\nckc_sha256\t{digest(ckc)}\n")
    with (out / "equivalence.tsv").open("w") as stream:
        writer = csv.writer(stream, delimiter="\t", lineterminator="\n")
        writer.writerow(("case", "file", "cmp_rc", "bytes", "before_sha256", "after_sha256"))
        documents = sorted((root / "guidelines/cdc-2022-opioid/pl").glob("*.pl"))
        assert documents
        for name in ("adult", "minor", "cancer", "dose"):
            label = f"consumer-{name}"
            case, report = work / label, out / label
            case.mkdir()
            report.mkdir()
            world = root / ("tests/queries/green/dose-arithmetic/tree/guidelines/fx/pl/world.pl"
                            if name == "dose" else f"tests/consumer/{name}.pl.in")
            composition = case / "composition.tsv"
            composition.write_text("".join(f"{path.relative_to(root)}\n" for path in (*documents, world)))
            compare(root, before, after, composition, report, writer, label)
            stream.flush()
            shutil.rmtree(case)
        for g in (1, 2, 4):
            for family in FAMILIES:
                label = f"{family}-{g}"
                case, report = work / label, out / label
                case.mkdir()
                report.mkdir()
                profile = "p-adult" if family in ("F1a", "F1b") else f"p-{family.lower()}"
                run([str(ckc), "bench", "gen", family, str(g), "--root", str(root),
                     "--patient", f"bench/sb/fixtures/patients/{profile}.pl.in", "--out", str(case / "composition")],
                    report / "generate.log")
                composition = case / "composition/composition.tsv"
                shutil.copy2(case / "composition/manifest.tsv", report / "manifest.tsv")
                shutil.copy2(composition, report / "composition.tsv")
                compare(root, before, after, composition, report, writer, label)
                stream.flush()
                shutil.rmtree(case)
    before.unlink()
    work.rmdir()
    with (out / "equivalence.tsv").open() as stream:
        rows = list(csv.DictReader(stream, delimiter="\t"))
    assert len(rows) == 62 and all(row["cmp_rc"] == "0" for row in rows)
    assert digest(after) == dict(line.split("\t", 1) for line in (out / "identities.tsv").read_text().splitlines())["producer_sha256"]
    print("comparison: 31 compositions; 62 exact model/certificate comparisons passed")


if __name__ == "__main__":
    main()
