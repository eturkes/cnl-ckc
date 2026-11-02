#!/usr/bin/env python3
"""Full-checker development profiles; reframe frozen certificates only after identity equality."""

import argparse
import csv
import hashlib
import os
from pathlib import Path
import random
import shutil
import statistics
import subprocess


def sha(data):
    return hashlib.sha256(data).hexdigest()


def framed(root, comp, old=False):
    schema = (root / "guidelines/schema.pl").read_bytes()
    data = f"schema\0{len(schema)}\0{sha(schema)}\n"
    for listed in comp.read_text().splitlines():
        if not listed or listed.startswith("#"):
            continue
        payload = (root / listed).read_bytes()
        path = listed.replace("rust/target/bench/admission/", "rust/target/bench/scientist-4/", 1) if old else listed
        data += f"file\0{path}\0{len(payload)}\0{sha(payload)}\n"
    return sha(data.encode())


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--inputs", required=True, type=Path)
    source = parser.add_mutually_exclusive_group(required=True)
    source.add_argument("--reference", type=Path)
    source.add_argument("--producer", type=Path)
    parser.add_argument("--checker", required=True, type=Path)
    parser.add_argument("--out", required=True, type=Path)
    args = parser.parse_args()
    assert args.out.resolve().is_relative_to(Path(__file__).resolve().parents[1] / "target")
    args.out.mkdir(parents=True)
    root = args.inputs / "root"
    datasets = {}
    for g in [2, 4, 10]:
        comp = root / f"rust/target/bench/admission/F1b-{g}/composition.tsv"
        case = args.out / f"G{g}"
        case.mkdir()
        if args.reference:
            reference = args.reference / f"F1b-{g}-baseline"
            certificate = (reference / "certificate.txt").read_text().splitlines()
            assert certificate[:2] == ["sb1-certificate 0", f"composition {framed(root, comp, old=True)}"], (g, "source identity does not match the frozen reference")
            certificate[1] = f"composition {framed(root, comp)}"
            (case / "certificate.txt").write_text("\n".join(certificate) + "\n")
            shutil.copyfile(reference / "model.txt", case / "model.txt")
            expected = (args.reference / f"F1b-{g}-baseline-check.log").read_bytes()
        else:
            generated = case / "producer"
            result = subprocess.run(["swipl", "-q", "-f", "none", "-F", "none", "-s", str(args.producer), "-g", "sb1_main", "-t", "halt(9)", "--", "--root", str(root), str(comp), str(generated)], capture_output=True, timeout=240)
            assert result.returncode == 0 and not result.stderr, result.stderr
            for name in ["model.txt", "certificate.txt"]:
                shutil.copyfile(generated / name, case / name)
            result = subprocess.run([str(args.checker), "--root", str(root), str(comp), str(case / "model.txt"), str(case / "certificate.txt"), str(root / "bench/sb/queries/q-all.txt")], env=dict(os.environ, SB1_SOURCE_CODEC="kernel"), capture_output=True, timeout=180)
            assert result.returncode == 0 and not result.stderr, result.stderr
            expected = result.stdout
        (case / "expected.answers").write_bytes(expected)
        datasets[g] = comp, case, expected
    rng = random.Random(5103)
    plan = []
    for block in range(1, 4):
        arms = [(g, mode) for g in [2, 4, 10] for mode in ["kernel", "parsed"]]
        rng.shuffle(arms)
        plan.extend((block, order, g, mode) for order, (g, mode) in enumerate(arms, 1))
    with (args.out / "plan.tsv").open("w") as stream:
        writer = csv.writer(stream, delimiter="\t", lineterminator="\n")
        writer.writerow(["block", "order", "G", "mode"])
        writer.writerows(plan)
    rows = []
    for block, order, g, mode in plan:
        comp, case, expected = datasets[g]
        env = dict(os.environ, SB1_SOURCE_CODEC=mode)
        cmd = [str(args.checker), "--root", str(root), str(comp), str(case / "model.txt"), str(case / "certificate.txt"), str(root / "bench/sb/queries/q-all.txt")]
        result = subprocess.run(cmd, env=env, capture_output=True, timeout=180)
        assert result.returncode == 0 and not result.stderr, (g, mode, result.returncode, result.stderr)
        assert result.stdout == expected, (g, mode, "answers differ")
        stem = f"block-{block}-order-{order}-G{g}-{mode}"
        (args.out / f"{stem}.answers").write_bytes(result.stdout)
        phases = {}
        for name in ["checker-phases.tsv", "checker-source-profile.tsv", "checker-counters.tsv"]:
            shutil.copyfile(case / name, args.out / f"{stem}-{name}")
        for row in csv.DictReader((case / "checker-phases.tsv").read_text().splitlines(), delimiter="\t"):
            phases[row["phase"]] = float(row["seconds"])
        phases["postload"] = sum(value for key, value in phases.items() if key != "load")
        rows.extend([block, order, g, mode, step, value] for step, value in phases.items())
        print(f"full block={block} G={g} mode={mode} admission={phases['admission']:.6f} postload={phases['postload']:.6f}", flush=True)
    with (args.out / "samples.tsv").open("w") as stream:
        writer = csv.writer(stream, delimiter="\t", lineterminator="\n")
        writer.writerow(["block", "order", "G", "mode", "step", "seconds"])
        writer.writerows(rows)
    with (args.out / "summary.tsv").open("w") as stream:
        writer = csv.writer(stream, delimiter="\t", lineterminator="\n")
        writer.writerow(["mode", "step", "G2_median", "G4_median", "G10_median", "slope_s_per_G", "intercept_s", "G100_projection_s", "max_sample_s"])
        for mode in ["kernel", "parsed"]:
            for step in ["load", "decode", "admission", "index", "support", "closure", "answers", "postload"]:
                medians = [statistics.median(row[5] for row in rows if row[2:5] == [g, mode, step]) for g in [2, 4, 10]]
                xs = [2, 4, 10]
                xm, ym = statistics.mean(xs), statistics.mean(medians)
                slope = sum((x-xm)*(y-ym) for x, y in zip(xs, medians)) / sum((x-xm)**2 for x in xs)
                intercept = ym-slope*xm
                writer.writerow([mode, step, *medians, slope, intercept, intercept+100*slope, max(row[5] for row in rows if row[3:5] == [mode, step])])
    (args.out / "checker.sha256").write_text(f"{sha(args.checker.read_bytes())}  {args.checker}\n")


if __name__ == "__main__":
    main()
