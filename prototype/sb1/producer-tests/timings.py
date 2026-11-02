#!/usr/bin/env python3
"""Nine development observations through ckc bench run; invoke under exclusive heavy."""

import argparse
import csv
import hashlib
from pathlib import Path
import shlex
import shutil
import subprocess


def digest(path):
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def rows(path):
    with path.open() as stream:
        return list(csv.DictReader(stream, delimiter="\t"))


def run(argv, log):
    with log.open("wb") as stream:
        result = subprocess.run(argv, stdout=stream, stderr=subprocess.STDOUT, timeout=3700)
    assert result.returncode == 0, (result.returncode, argv, log)


def exclusive():
    status = subprocess.check_output(["heavy", "--status"], text=True)
    holders = [line for line in status.splitlines() if line.startswith("slot ")]
    assert len(holders) == 1, f"expected this process's heavy holder alone; got {len(holders)}: {status}"


def phases(path):
    return {row["phase"]: row["seconds"] for row in rows(path)}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("root", type=Path)
    parser.add_argument("--ckc", required=True, type=Path)
    parser.add_argument("--checker", required=True, type=Path)
    parser.add_argument("--out", required=True, type=Path)
    args = parser.parse_args()
    root, ckc, checker, out = (args.root.resolve(), args.ckc.resolve(),
                               args.checker.resolve(), args.out.resolve())
    exclusive()
    out.mkdir(parents=True, exist_ok=False)
    work = root / "rust/target/bench" / f"builder-4-{out.name}"
    work.mkdir(parents=True, exist_ok=False)
    source = root / "prototype/sb1/producer.prolog"
    pinned = {"producer": (source, digest(source)), "ckc": (ckc, digest(ckc)),
              "checker": (checker, digest(checker))}
    (out / "identities.tsv").write_text("".join(f"{name}\t{value}\t{path}\n" for name, (path, value) in pinned.items()))
    (out / "protocol.tsv").write_text(
        "claim\tdevelopment observations only; not graded\n"
        "cases\tG=2,4,10; F1a,F1b,MX; fixed order; one block each\n"
        "deadline_ms\t3600000\nmemory_bytes\t48000000000\nsample_ms\t50\n"
        "exclusions\tnone; every invocation retained\n")
    producer = shlex.join(["swipl", "-q", "-f", "none", "-F", "none", "-s", str(source),
                           "-g", "sb1_main", "-t", "halt(9)", "--"])
    columns = ["family", "G", "outcome", "producer_load", "eval", "certificate", "write",
               "checker_load", "decode", "admission", "index", "support", "closure", "answers",
               "scored_seconds", "peak_rss_bytes", "model_facts", "model_sha256", "certificate_sha256", "answer_sha256"]
    with (out / "timings.tsv").open("w") as stream:
        writer = csv.DictWriter(stream, fieldnames=columns, delimiter="\t", lineterminator="\n")
        writer.writeheader()
        for g in (2, 4, 10):
            for family in ("F1a", "F1b", "MX"):
                exclusive()
                for path, value in pinned.values():
                    assert digest(path) == value, path
                label = f"{family}-{g}"
                case, report = work / label, out / label
                case.mkdir()
                report.mkdir()
                profile = "p-mx" if family == "MX" else "p-adult"
                generate = [str(ckc), "bench", "gen", family, str(g), "--root", str(root),
                            "--patient", f"bench/sb/fixtures/patients/{profile}.pl.in", "--out", str(case / "composition")]
                run(generate, report / "generate.log")
                shutil.copy2(case / "composition/manifest.tsv", report / "manifest.tsv")
                command = [str(ckc), "bench", "run", str(case / "composition"), "--root", str(root),
                           "--producer", producer, "--checker", shlex.join([str(checker)]),
                           "--query", str(root / "bench/sb/queries/q-all.txt"), "--blocks", "1", "--seed", "1",
                           "--deadline-ms", "3600000", "--memory-bytes", "48000000000", "--sample-ms", "50",
                           "--identity-file", str(source), "--identity-file", str(checker),
                           "--out", str(report / "run")]
                (report / "command.txt").write_text(shlex.join(command) + "\n")
                exclusive()
                run(command, report / "run.log")
                exclusive()
                observed = rows(report / "run/runs.tsv")
                assert len(observed) == 1 and observed[0]["outcome"] == "verified", observed
                result = observed[0]
                stage = report / "run/run-000001"
                p, c = phases(stage / "producer-phases.tsv"), phases(stage / "checker-phases.tsv")
                model, cert = stage / "model.txt", stage / "certificate.txt"
                row = {"family": family, "G": g, "outcome": result["outcome"],
                       "producer_load": p["load"], "checker_load": c["load"],
                       "scored_seconds": f"{int(result['scored_ns']) / 1e9:.9f}",
                       "peak_rss_bytes": result["peak_rss_bytes"],
                       "model_facts": model.read_bytes().count(b"\n"),
                       "model_sha256": digest(model), "certificate_sha256": digest(cert),
                       "answer_sha256": result["output_sha256"]}
                row.update({name: p[name] for name in ("eval", "certificate", "write")})
                row.update({name: c[name] for name in ("decode", "admission", "index", "support", "closure", "answers")})
                writer.writerow(row)
                stream.flush()
                print(f"{label}: eval={p['eval']} certificate={p['certificate']} scored={row['scored_seconds']} s; verified", flush=True)
                model.unlink()
                cert.unlink()
                shutil.rmtree(case)
    work.rmdir()
    assert len(rows(out / "timings.tsv")) == 9
    for path, value in pinned.values():
        assert digest(path) == value, path
    print("timings: nine development observations complete; no feasibility claim")


if __name__ == "__main__":
    main()
