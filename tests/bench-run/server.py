#!/usr/bin/env python3
# Mock warm server for `ckc bench run --serve` (contract sb § Carried rulings): the
# checker's `serve` protocol — load, `ready`, then one call per stdin line
# `<id>\t<query>\t<new dir>` answered by `done\t<id>\t<rc>` once the call's files exist.
import hashlib
import mmap
import os
from pathlib import Path
import sys
import time

SCORED = ["patient", "slice", "materialize", "producer", "decode", "index", "support",
          "closure", "answers"]
LOAD = ["load_parse", "load_admission", "load_analysis", "load_index", "load_receipt"]


def table(path, names, kind, step):
    rows = "".join(f"{n}\t{step:.9f}\t0.000000000\t{i * step:.9f}\t{kind}\n"
                   for i, n in enumerate(names))
    path.write_text("phase\tseconds\tcpu\tstart_seconds\tkind\n" + rows)


def ticks():
    fields = Path("/proc/self/stat").read_text().rsplit(")", 1)[1].split()
    return sum(max(0, int(fields[i])) for i in (11, 12, 13, 14))


def main():
    mode = sys.argv[1]
    assert sys.argv[2] == "--root" and len(sys.argv) == 6
    arm = Path(sys.argv[5])
    arm.mkdir(parents=True)
    if mode == "load-exit":
        sys.exit(4)
    time.sleep(0.05)
    table(arm / "load-phases.tsv", LOAD, "load", 0.005)
    print("ready", flush=True)
    calls = 0
    for line in sys.stdin:
        ident, query, directory = line.rstrip("\n").split("\t")
        out = Path(directory)
        out.mkdir()
        calls += 1
        assert Path(query).is_file()
        if mode == "slow" and calls == 1:
            time.sleep(5)
        if mode == "memory" and calls == 1:
            memory = mmap.mmap(-1, 48 * 1024 * 1024)
            for offset in range(0, len(memory), 4096):
                memory[offset] = 1
            time.sleep(1)
        if mode == "protocol":
            print(f"done\t{int(ident) + 1}\t0", flush=True)
            continue
        rc = 1 if mode == "refuse-first" and calls == 1 else 0
        time.sleep(0.03)
        table(out / "call-phases.tsv", SCORED, "scored", 0.001)
        if rc == 0:
            (out / "model.txt").write_text("fact.\n")
            (out / "certificate.txt").write_text("certificate\n")
            answer = f"ANSWER {Path(query).name}\n".encode()
            (out / "answers.txt").write_bytes(answer)
            closed = time.time_ns()
            tmp = out / "sink-complete.tmp"
            tmp.write_text(f"sb1-sink 1\nclosed_unix_ns\t{closed}\nbytes\t{len(answer)}\n"
                           f"sha256\t{hashlib.sha256(answer).hexdigest()}\ncpu_ticks\t{ticks()}\n")
            tmp.rename(out / "sink-complete.tsv")
        (out / "call-done.tsv").write_text(f"rc\t{rc}\n")
        print(f"done\t{ident}\t{rc}", flush=True)
        print(os.getpid(), file=sys.stderr, flush=True)


if __name__ == "__main__":
    main()
