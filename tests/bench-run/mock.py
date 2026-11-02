#!/usr/bin/env python3
import mmap
import os
from pathlib import Path
import subprocess
import sys
import time


def stamp(out, label):
    with (out / "controls.tsv").open("a") as stream:
        stream.write(f"{label}\t{os.getpid()}\t{time.monotonic_ns()}\n")


def allocation(out, delay, hold, size):
    time.sleep(delay)
    memory = mmap.mmap(-1, size)
    for offset in range(0, size, 4096):
        memory[offset] = 1
    stamp(out, "allocated")
    time.sleep(hold)
    memory.close()
    stamp(out, "freed")


def phases(out, role, start, cpu, load):
    elapsed = time.monotonic() - start
    used = time.process_time() - cpu
    if role == "producer":
        rows = [("load", load, 0), ("eval", elapsed - load, used),
                ("certificate", 0, 0), ("write", 0, 0)]
    else:
        rows = [("load", load, 0), ("decode", 0, 0),
                ("admission", 0, 0), ("index", 0, 0),
                ("support", elapsed - load, used), ("closure", 0, 0),
                ("answers", 0, 0)]
    with (out / f"{role}-phases.tsv").open("w") as stream:
        stream.write("phase\tseconds\tcpu\n")
        for name, wall, used in rows:
            stream.write(f"{name}\t{wall:.9f}\t{used:.9f}\n")


def main():
    role, mode = sys.argv[1:3]
    if role == "worker":
        allocation(Path(sys.argv[3]), float(sys.argv[4]),
                   float(sys.argv[5]), int(sys.argv[6]))
        return
    if role == "producer":
        assert sys.argv[3] == "--root"
        composition, out = map(Path, sys.argv[5:7])
        assert composition.is_file()
    else:
        assert sys.argv[3] == "--root"
        composition, model, certificate, query = map(Path, sys.argv[5:9])
        assert all(path.is_file() for path in (composition, model, certificate, query))
        out = model.parent
    start, cpu = time.monotonic(), time.process_time()
    load = 0.015
    time.sleep(load)
    if role == "producer":
        (out / "model.txt").write_text("fact.\n")
        (out / "certificate.txt").write_text("certificate\n")
    if mode in ("timeout", "memory", "size") and role == "checker":
        print("PARTIAL-ANSWER", flush=True)
        if mode == "timeout":
            child = subprocess.Popen([sys.executable, "-I", __file__, "worker", "unused",
                                      str(out), "0", "5", str(4 * 1024 * 1024)])
            (out.parent / "child.pid").write_text(str(child.pid))
            time.sleep(5)
        elif mode == "memory":
            allocation(out, 0, 2, 40 * 1024 * 1024)
        else:
            print("x" * (2 * 1024 * 1024), flush=True)
    elif mode == "allocations":
        workers = [subprocess.Popen([sys.executable, "-I", __file__, "worker", "unused",
                                     str(out), str(delay), str(hold), str(size)])
                   for delay, hold, size in [(0.025, 0.14, 12 * 1024 * 1024),
                                              (0.095, 0.18, 18 * 1024 * 1024)]]
        allocation(out, 0.01, 0.30, 8 * 1024 * 1024)
        for worker in workers:
            worker.wait()
    elif mode == "short-spike":
        worker = subprocess.Popen([sys.executable, "-I", __file__, "worker", "unused",
                                   str(out), "0.03", "0.004", str(16 * 1024 * 1024)])
        worker.wait()
        time.sleep(0.35)
    elif mode == "parallel":
        workers = [subprocess.Popen([sys.executable, "-I", "-c", "import time; time.sleep(.08)"])
                   for _ in range(2)]
        for worker in workers:
            worker.wait()
    else:
        time.sleep(0.025)
    if mode == "bad-phases":
        (out / f"{role}-phases.tsv").write_text("phase\tseconds\tcpu\nload\t9999\t0\n")
    else:
        phases(out, role, start, cpu, load)
    if role == "checker":
        (out / "checker-counters.tsv").write_text(
            "kind\tname\tvalue\nlogical\tmodel_facts\t1\nruntime\tcandidate_joins\t1\n")
        if mode == "refused":
            print("refused closure planted", file=sys.stderr)
            sys.exit(1)
        if mode == "producer-failure":
            sys.exit(2)
        if mode != "empty":
            print("COMPLETE-ANSWER")
    elif mode == "producer-failure":
        print("resource planted", file=sys.stderr)
        sys.exit(9)


if __name__ == "__main__":
    main()
