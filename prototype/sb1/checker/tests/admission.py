#!/usr/bin/env python3
"""Development-only admission profiles and kernel/parsed-codec equivalence controls."""

import argparse
import csv
import hashlib
import json
import os
from pathlib import Path
import random
import shutil
import statistics
import subprocess

FAMILIES = ["F1a", "F1b", "F2", "F3", "F4", "F5a", "F5b", "F5c", "MX"]
PATIENTS = {"F1a": "p-adult", "F1b": "p-adult", "F2": "p-f2", "F3": "p-f3", "F4": "p-f4", "F5a": "p-f5a", "F5b": "p-f5b", "F5c": "p-f5c", "MX": "p-mx"}


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def run(argv, mode=None):
    env = dict(os.environ)
    if mode:
        env["SB1_SOURCE_CODEC"] = mode
    proc = subprocess.run([str(a) for a in argv], env=env, capture_output=True, timeout=240)
    assert proc.returncode == 0 and not proc.stderr, (argv, proc.returncode, proc.stderr.decode())
    return proc.stdout


def composition(out, family, g):
    return out / f"root/rust/target/bench/admission/{family}-{g}/composition.tsv"


def prepare(args):
    root = args.out / "root"
    (root / "guidelines/cdc-2022-opioid").mkdir(parents=True)
    shutil.copytree(args.root / "guidelines/cdc-2022-opioid/pl", root / "guidelines/cdc-2022-opioid/pl")
    shutil.copyfile(args.root / "guidelines/schema.pl", root / "guidelines/schema.pl")
    shutil.copytree(args.root / "bench/sb", root / "bench/sb")
    (root / "docs").mkdir()
    shutil.copyfile(args.root / "docs/REFERENCE.md", root / "docs/REFERENCE.md")
    for family, g in [(f, g) for f in FAMILIES for g in [1, 2]] + [("F1b", 4), ("F1b", 10)]:
        comp = composition(args.out, family, g)
        run([args.ckc, "bench", "gen", family, g, "--root", root, "--patient", f"bench/sb/fixtures/patients/{PATIENTS[family]}.pl.in", "--out", comp.parent])
        print(f"prepared {family}-{g}", flush=True)
    identities = {"ckc": digest(args.ckc), "checker": digest(args.checker), "producer": digest(args.producer), "base": run(["git", "-C", args.root, "rev-parse", "HEAD"]).decode().strip()}
    (args.out / "identities.json").write_text(json.dumps(identities, indent=2) + "\n")


def compare(args):
    root = args.out / "root"
    results = []
    for family in FAMILIES:
        for g in [1, 2]:
            case = args.out / f"compare/{family}-{g}"
            case.mkdir(parents=True)
            comp = composition(args.out, family, g)
            out = case / "producer"
            run(["swipl", "-q", "-f", "none", "-F", "none", "-s", args.producer, "-g", "sb1_main", "-t", "halt(9)", "--", "--root", root, comp, out])
            answers = []
            for mode in ["kernel", "parsed"]:
                answer = run([args.checker, "--root", root, comp, out / "model.txt", out / "certificate.txt", root / "bench/sb/queries/q-all.txt"], mode)
                (case / f"{mode}.answers").write_bytes(answer)
                for name in ["checker-phases.tsv", "checker-counters.tsv", "checker-source-profile.tsv"]:
                    shutil.copyfile(out / name, case / f"{mode}-{name}")
                answers.append(answer)
            assert answers[0] == answers[1], (family, g, "answer mismatch")
            assert (case / "kernel-checker-counters.tsv").read_bytes() == (case / "parsed-checker-counters.tsv").read_bytes(), (family, g, "work-count mismatch")
            results.append([family, g, len((out / "model.txt").read_bytes().splitlines()), len(answers[0].splitlines()), digest(out / "model.txt"), digest(case / "parsed.answers")])
            print(f"equal {family}-{g}: {results[-1][2]} facts, {results[-1][3]} q-all rows", flush=True)
    with (args.out / "equivalence.tsv").open("w") as stream:
        writer = csv.writer(stream, delimiter="\t", lineterminator="\n")
        writer.writerow(["family", "G", "facts", "answer_rows", "model_sha256", "answers_sha256"])
        writer.writerows(results)


def profile(args):
    root = args.out / "root"
    rng = random.Random(5102)
    rows = []
    plan = []
    for block in range(1, 4):
        arms = [(g, mode) for g in [2, 4, 10] for mode in ["kernel", "parsed"]]
        rng.shuffle(arms)
        plan.extend((block, order, g, mode) for order, (g, mode) in enumerate(arms, 1))
    with (args.out / "plan.tsv").open("w") as stream:
        writer = csv.writer(stream, delimiter="\t", lineterminator="\n")
        writer.writerow(["block", "order", "G", "mode"])
        writer.writerows(plan)
    for block, order, g, mode in plan:
        comp = composition(args.out, "F1b", g)
        raw = run([args.checker, "source-profile", "--root", root, comp], mode)
        path = args.out / f"profile-{block}-{order}-G{g}-{mode}.tsv"
        path.write_bytes(raw)
        steps = {row["step"]: float(row["seconds"]) for row in csv.DictReader(raw.decode().splitlines(), delimiter="\t")}
        steps["admission"] = steps["canonical"] + steps["shape_safety"]
        rows.extend([block, order, g, mode, step, value] for step, value in steps.items())
        print(f"profile block={block} G={g} mode={mode} admission={steps['admission']:.6f}", flush=True)
    with (args.out / "samples.tsv").open("w") as stream:
        writer = csv.writer(stream, delimiter="\t", lineterminator="\n")
        writer.writerow(["block", "order", "G", "mode", "step", "seconds"])
        writer.writerows(rows)
    with (args.out / "summary.tsv").open("w") as stream:
        writer = csv.writer(stream, delimiter="\t", lineterminator="\n")
        writer.writerow(["mode", "step", "G2_median", "G4_median", "G10_median", "slope_s_per_G", "intercept_s", "G100_projection_s", "max_sample_s"])
        for mode in ["kernel", "parsed"]:
            for step in ["normalization", "decoder", "canonical", "shape_safety", "admission"]:
                medians = [statistics.median(row[5] for row in rows if row[2:5] == [g, mode, step]) for g in [2, 4, 10]]
                xs = [2, 4, 10]
                xm = statistics.mean(xs)
                ym = statistics.mean(medians)
                slope = sum((x-xm)*(y-ym) for x, y in zip(xs, medians)) / sum((x-xm)**2 for x in xs)
                intercept = ym-slope*xm
                maximum = max(row[5] for row in rows if row[3:5] == [mode, step])
                writer.writerow([mode, step, *medians, slope, intercept, intercept+100*slope, maximum])


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("operation", choices=["prepare", "compare", "profile"])
    parser.add_argument("--root", required=True, type=Path)
    parser.add_argument("--ckc", required=True, type=Path)
    parser.add_argument("--checker", required=True, type=Path)
    parser.add_argument("--producer", required=True, type=Path)
    parser.add_argument("--out", required=True, type=Path)
    args = parser.parse_args()
    args.root = args.root.resolve()
    args.out = args.out.resolve()
    target = Path(__file__).resolve().parents[1] / "target"
    assert args.out.is_relative_to(target), "outputs must remain in this checker's target"
    args.out.mkdir(parents=True, exist_ok=True)
    {"prepare": prepare, "compare": compare, "profile": profile}[args.operation](args)


if __name__ == "__main__":
    main()
