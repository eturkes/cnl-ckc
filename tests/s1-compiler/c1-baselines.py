#!/usr/bin/env python3
"""C1 fixture regeneration: <pre-S1 repo root> [--check]; writes C1 bytes only."""

import concurrent.futures
import csv
import hashlib
import pathlib
import shutil
import subprocess
import sys
import tempfile

ROOT = pathlib.Path(sys.argv[1]).resolve()
SUITE = ROOT / "tests/s1-compiler"
CHECK = sys.argv[2:] == ["--check"]
KINDS = {"strength", "evidence", "preference", "alternative"}
HEADER = "# format: kind\tlemma\tvalue\n"
INERT = "unit\ts1-unused-c1-unit\tsecond\n"


def git(*args):
    return subprocess.check_output(["git", "-C", str(ROOT), *args], text=True).strip()


def run(args, **kwargs):
    return subprocess.run(args, capture_output=True, timeout=60, **kwargs)


def filtered(table):
    if table == "-":
        return None
    raw = (SUITE / f"{table}.tsv").read_text()
    rows = [line for line in raw.splitlines()[1:] if line.split("\t")[0] not in KINDS]
    return HEADER + ("\n".join(rows) + "\n" if rows else INERT)


def atom(value):
    return "'" + str(value).replace("\\", "\\\\").replace("'", "\\'") + "'"


def main():
    compiler = "vendor/ape/prolog/ace_to_pl.pl"
    assert git("hash-object", compiler) == git("rev-parse", f"d972a813:{compiler}"), "C1 compiler required"
    version = subprocess.check_output(["swipl", "--version"], text=True)
    assert "version 9.2.9 " in version, version
    rows = list(csv.reader((SUITE / "cases.tsv").read_text().splitlines()[1:], delimiter="\t"))
    parent = ROOT / "rust/target/s1-baselines"
    parent.mkdir(parents=True, exist_ok=True)
    failures = []
    digests = []
    with tempfile.TemporaryDirectory(dir=parent) as tmp:
        scratch = pathlib.Path(tmp)
        stage = scratch / "ape"
        shutil.copytree(ROOT / "vendor/ape", stage)
        shutil.copyfile(ROOT / "vendor/clex/clex_lexicon.pl", stage / "prolog/lexicon/clex_lexicon.pl")
        goal = f"working_directory(_, {atom(stage / 'prolog/parser')}), [fit_to_plp], halt."
        built = run(["swipl", "-O", "-f", "none", "-F", "none", "-g", goal, "-t", "halt"], cwd=scratch)
        assert built.returncode == 0, built.stderr.decode()

        def compile_row(row):
            _, name, mode, table, _, _, expect, ace = row
            case = scratch / name
            case.mkdir()
            data = filtered(table)
            table_arg = "-"
            if data is not None:
                path = case / "vocabulary.tsv"
                path.write_text(data)
                table_arg = str(path)
            args = (["question"] if mode == "query" else []) + [str(stage), name, table_arg, str(SUITE / "lexicon.ulex")]
            if mode == "proof":
                args.append("proof")
            out = run(["swipl", "-q", "-f", "none", "-F", "none", "-s", str(stage / "prolog/ace_to_pl.pl"), "-g", "main", "-t", "halt(9)", "--", *args], input=(ace.replace("\\n", "\n") + "\n").encode(), cwd=case)
            if expect != "ok":
                if expect.startswith("temporal_shape("):
                    wanted = f"ace_to_pl_error(unsupported,{expect}).\n".encode()
                    error = None if out.returncode == 1 and not out.stdout and out.stderr == wanted else "C1 temporal control differs"
                else:
                    error = None if out.returncode == 0 and not out.stderr else "C1 must accept an S1-only reject probe"
                return name, error, out
            if out.returncode != 0 or out.stderr:
                return name, f"rc={out.returncode}, stderr={out.stderr.decode().strip()}", out
            target = SUITE / "c1" / f"{name}.pl.in"
            if CHECK:
                if not target.exists() or target.read_bytes() != out.stdout:
                    return name, "C1 bytes differ", out
            else:
                target.parent.mkdir(parents=True, exist_ok=True)
                target.write_bytes(out.stdout)
            return name, hashlib.sha256(out.stdout).hexdigest(), out

        with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:
            for row, (name, result, out) in zip(rows, pool.map(compile_row, rows)):
                if row[6] != "ok":
                    print(f"probe {name}: rc={out.returncode} {out.stderr.decode().strip()}")
                    if result is not None:
                        failures.append(f"{name}: {result}; {out.stderr.decode().strip()}")
                elif result is None or len(result) != 64:
                    failures.append(f"{name}: {result}")
                else:
                    digests.append(f"{result}  {name}.pl.in")
        for failure in failures:
            print(f"FAIL {failure}", file=sys.stderr)
        if not failures:
            manifest = "\n".join(sorted(digests)) + "\n"
            target = SUITE / "c1/SHA256SUMS"
            if CHECK:
                assert target.read_text() == manifest, "C1 digest census differs"
            else:
                target.write_text(manifest)
        print(f"C1 baselines: {len(digests)} green, {len(failures)} failed; {'checked' if CHECK else 'written'}")
    return bool(failures)


if __name__ == "__main__":
    sys.exit(main())
