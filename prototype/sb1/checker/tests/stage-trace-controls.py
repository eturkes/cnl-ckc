#!/usr/bin/env python3
"""Synthetic positive + refusal controls for cross-origin stage/CPU analysis."""

import csv
from pathlib import Path
import subprocess
import sys
import tempfile

OWNER_EVENTS = [
    "call_begin", "patient_decode_begin", "patient_decode_end", "patient_admission_begin",
    "patient_admission_end", "identity_begin", "identity_end", "query_decode_begin",
    "query_decode_end", "slice_select_begin", "slice_select_end", "slice_clone_begin",
    "slice_clone_end", "slice_write_begin", "slice_write_end", "producer_spawn_begin",
    "producer_exited", "model_certificate_decode_begin", "model_certificate_decode_end",
    "check_begin", "check_end", "answers_compute_begin", "answers_compute_end",
    "call_inner_done", "patient_documents_dropped", "patient_clauses_dropped", "arena_rewound",
    "metrics_write_begin", "metrics_write_end", "result_return_ready", "stdout_write_begin",
    "stdout_flushed", "stdout_guard_dropped", "pre_process_exit",
]


def write(path, header, data):
    with path.open("w") as stream:
        writer = csv.writer(stream, delimiter="\t", lineterminator="\n")
        writer.writerow(header)
        writer.writerows(data)


def fixture(root):
    write(root / "settings.tsv", ("key", "value"), [("clock_ticks_per_second", "100")])
    runs = []
    for i in range(1, 7):
        arm = "baseline" if i % 2 else "target"
        load = 1_000_000_000 if arm == "baseline" else 10_000_000_000
        exit_ns = 20_000_000 if arm == "baseline" else 400_000_000
        pid = 100 + i
        pre_exit = load + 100_000_000
        nonlive = pre_exit + 2_000_000 + exit_ns
        final_ticks = (pre_exit + exit_ns) // 10_000_000
        stage = root / f"run-{i:06}"
        stage.mkdir()
        owner = [(event, load + j * 1_000_000, load + j * 1_000_000, pid)
                 for j, event in enumerate(OWNER_EVENTS)]
        owner[-1] = ("pre_process_exit", pre_exit, pre_exit, pid)
        write(stage / "orchestrator-trace.tsv", ("event", "wall_ns", "cpu_ns", "pid"), owner)
        events = [("orchestrator", "spawn_begin", 0), ("orchestrator", "spawned", 1_000_000),
                  ("orchestrator", "registered", 2_000_000),
                  ("orchestrator", "root_nonlive_or_limit", nonlive),
                  ("orchestrator", "exit_boundary_done", nonlive + 1_000_000),
                  ("orchestrator", "wait_begin", nonlive + 2_000_000),
                  ("orchestrator", "wait_done", nonlive + 3_000_000),
                  ("orchestrator", "post_wait_boundary_done", nonlive + 4_000_000),
                  ("controller", "answer_sink_digest_begin", nonlive + 5_000_000),
                  ("controller", "answer_sink_renamed", nonlive + 6_000_000),
                  ("controller", "sampler_finish_begin", nonlive + 7_000_000),
                  ("controller", "sampler_finish_end", nonlive + 8_000_000),
                  ("controller", "publish_begin", nonlive + 9_000_000),
                  ("controller", "publish_end", nonlive + 10_000_000),
                  ("controller", "scored_wall_end", nonlive + 11_000_000)]
        parent = [(role, event, wall, 0, pid, 0, "Z", final_ticks, 0) for role, event, wall in events]
        write(stage / "harness-stage-trace.tsv", ("role", "event", "wall_ns", "parent_own_ticks",
              "child_pid", "child_start_ticks", "child_state", "child_own_ticks", "child_waited_ticks"), parent)
        write(stage / "rss-processes.tsv", ("pid", "state", "sample_end_ns", "rss_bytes"),
              [(pid, "R", nonlive - 1, 40_000_000 if arm == "baseline" else 4_000_000_000)])
        runs.append((i, (i + 1) // 2, 1 if i % 2 else 2, arm, "verified", nonlive + 11_000_000 - load))
    write(root / "runs.tsv", ("run", "block", "order", "arm", "outcome", "scored_ns"), runs)


def main():
    source = Path(__file__).with_name("analyze-stage-trace.py")
    parent = Path(__file__).parents[1] / "target/stage-trace-controls"
    parent.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(dir=parent) as name:
        root = Path(name)
        fixture(root)
        result = subprocess.run([sys.executable, "-I", source, root], capture_output=True, text=True)
        assert result.returncode == 0, result.stderr
        with (root / "stage-summary.tsv").open() as stream:
            values = {(r["arm"], r["stage"]): float(r["median"]) for r in csv.DictReader(stream, delimiter="\t")}
        for arm, wall, cpu in [("baseline", 0.022, 0.020), ("target", 0.402, 0.400)]:
            assert abs(values[arm, "startup_plus_post_pre_exit"] - wall) < 1e-8
            assert abs(values[arm, "post_pre_exit_CPU_tick_quantized"] - cpu) < 1e-8
            assert abs(values[arm, "wait_reap"] - 0.001) < 1e-8
        print("stage trace control: separate wall origins + absolute CPU remainder = exact expected values")
        trace = root / "run-000002/harness-stage-trace.tsv"
        original = trace.read_text()
        trace.write_text(original.replace("\t102\t", "\t999\t"))
        result = subprocess.run([sys.executable, "-I", source, root], capture_output=True, text=True)
        assert result.returncode != 0 and "AssertionError" in result.stderr
        print("stage trace plant: cross-process identity mismatch refused")
        trace.write_text(original)
        run_file = root / "runs.tsv"
        run_file.write_text(run_file.read_text().replace("\tverified\t", "\tindeterminate\t", 1))
        result = subprocess.run([sys.executable, "-I", source, root], capture_output=True, text=True)
        assert result.returncode != 0 and "AssertionError" in result.stderr
        print("stage trace plant: incomplete campaign refused")


if __name__ == "__main__":
    main()
