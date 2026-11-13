#!/usr/bin/env python3
# Mock warm server for `ckc bench run --serve` (contract sb § Carried rulings): the
# checker's `serve` protocol — load, `ready`, then one call per stdin line
# `<id>\t<query>\t<new dir>` answered by `done\t<id>\t<rc>` once the call's files exist. Each call
# records its request's arrival (`request.tsv`) and, except in mode `unreported`, a producer report
# in `checker-counters.tsv` (contract sb P8): mode `producer-self` names the server's own pid with
# a 1 GiB peak (`producer-self-start` adds its own start ticks, `producer-self-stale` that start + 1:
# TAS-P2's (pid, start_ticks) binding; `producer-self-small` = its own start with a 1 B peak: the samples'
# larger HWM must win); otherwise a pid no process holds with a 1,000,000,000 B peak;
# `producer-cpu` adds `producer_cpu_us` 2,000,000 (a launcher-reaped producer's CPU). Mode `grow` touches
# 48 MiB per call before answering (the server's VmHWM rises after the last periodic sample); mode
# `transient` runs a 0.3 s child in the first call, gone before the answer (scientist-8 S7).
# Load modes (the direct load-memory bound): `census` reaps a child touching 48 MiB with wait4 and
# reports it as an integrated load's census (`arm/load.tsv`); `census-unreported` omits that report;
# `load-transient` runs a 0.3 s child before ready, reported nowhere.
import hashlib
import mmap
import os
from pathlib import Path
import subprocess
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
    if mode in ("census", "census-unreported"):
        touch = ("import mmap, time\nm = mmap.mmap(-1, 48 * 1024 * 1024)\n"
                 "for o in range(0, len(m), 4096):\n    m[o] = 1\ntime.sleep(0.05)\n")
        child = subprocess.Popen([sys.executable, "-I", "-c", touch])
        _, status, usage = os.wait4(child.pid, 0)
        assert os.waitstatus_to_exitcode(status) == 0
        rows = "knowledge_documents\t1\nreceipt\tintegrated\n"
        if mode == "census":
            rows += f"census_pid\t{child.pid}\ncensus_maxrss_bytes\t{usage.ru_maxrss * 1024}\n"
        (arm / "load.tsv").write_text(rows)
    if mode == "load-transient":
        subprocess.run(["sleep", "0.3"], check=True)
    table(arm / "load-phases.tsv", LOAD, "load", 0.005)
    print("ready", flush=True)
    calls = 0
    for line in sys.stdin:
        ident, query, directory = line.rstrip("\n").split("\t")
        out = Path(directory)
        out.mkdir()
        (out / "request.tsv").write_text(f"received_unix_ns\t{time.time_ns()}\n")
        calls += 1
        assert Path(query).is_file()
        if mode == "slow" and calls == 1:
            time.sleep(5)
        if mode == "memory" and calls == 1:
            memory = mmap.mmap(-1, 48 * 1024 * 1024)
            for offset in range(0, len(memory), 4096):
                memory[offset] = 1
            time.sleep(1)
        if mode == "grow":
            grown = mmap.mmap(-1, 48 * 1024 * 1024)
            for offset in range(0, len(grown), 4096):
                grown[offset] = 1
            grown.close()
        if mode == "transient" and calls == 1:
            subprocess.run(["sleep", "0.3"], check=True)
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
        if mode != "unreported":
            own = mode.startswith("producer-self")
            pid, peak = (os.getpid(), 1 << 30) if own else (4294967295, 10**9)
            if mode == "producer-self-small":
                peak = 1
            rows = (f"kind\tcounter\tvalue\nruntime\tproducer_pid\t{pid}\n"
                    f"runtime\tproducer_maxrss_bytes\t{peak}\n")
            if mode in ("producer-self-start", "producer-self-stale", "producer-self-small"):
                start = int(Path("/proc/self/stat").read_text().rsplit(")", 1)[1].split()[19])
                rows += f"runtime\tproducer_start_ticks\t{start + (mode == 'producer-self-stale')}\n"
            if mode == "producer-cpu":
                rows += "runtime\tproducer_cpu_us\t2000000\n"
            (out / "checker-counters.tsv").write_text(rows)
        (out / "call-done.tsv").write_text(f"rc\t{rc}\n")
        print(f"done\t{ident}\t{rc}", flush=True)
        print(os.getpid(), file=sys.stderr, flush=True)


if __name__ == "__main__":
    main()
