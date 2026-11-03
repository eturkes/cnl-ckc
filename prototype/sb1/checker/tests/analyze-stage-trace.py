#!/usr/bin/env python3
"""Separate owner work, controller work and the quantized post-pre-exit CPU remainder."""

import argparse
import csv
from pathlib import Path
import statistics


def rows(path):
    with path.open() as stream:
        return list(csv.DictReader(stream, delimiter="\t"))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("campaign", type=Path)
    args = parser.parse_args()
    root = args.campaign.resolve()
    runs = rows(root / "runs.tsv")
    assert len(runs) == 6 and all(row["outcome"] == "verified" for row in runs)
    assert all(sorted(row["arm"] for row in runs if row["block"] == str(block)) == ["baseline", "target"]
               for block in (1, 2, 3))
    settings = {row["key"]: row["value"] for row in rows(root / "settings.tsv")}
    ticks = float(settings["clock_ticks_per_second"])
    observations = []
    for run in runs:
        stage = root / f"run-{int(run['run']):06}"
        owner_rows = rows(stage / "orchestrator-trace.tsv")
        owner = {row["event"]: row for row in owner_rows}
        assert len(owner) == len(owner_rows)
        parent_rows = rows(stage / "harness-stage-trace.tsv")
        parent = {(row["role"], row["event"]): row for row in parent_rows}
        assert len(parent) == len(parent_rows)

        def add(name, value, units="seconds"):
            observations.append((run["run"], run["block"], run["order"], run["arm"], name, units, value))

        def owner_span(name, begin, end):
            add(name, (int(owner[end]["wall_ns"]) - int(owner[begin]["wall_ns"])) / 1e9)

        def parent_span(name, begin, end, role="controller"):
            add(name, (int(parent[role, end]["wall_ns"]) - int(parent[role, begin]["wall_ns"])) / 1e9)

        for name, begin, end in [
            ("patient_decode", "patient_decode_begin", "patient_decode_end"),
            ("patient_admission", "patient_admission_begin", "patient_admission_end"),
            ("identity", "identity_begin", "identity_end"),
            ("query_decode", "query_decode_begin", "query_decode_end"),
            ("slice_select", "slice_select_begin", "slice_select_end"),
            ("slice_clone", "slice_clone_begin", "slice_clone_end"),
            ("slice_write", "slice_write_begin", "slice_write_end"),
            ("SWI_spawn_wait", "producer_spawn_begin", "producer_exited"),
            ("model_certificate_decode", "model_certificate_decode_begin", "model_certificate_decode_end"),
            ("check", "check_begin", "check_end"),
            ("answers_compute", "answers_compute_begin", "answers_compute_end"),
            ("model_local_drop", "answers_compute_end", "call_inner_done"),
            ("patient_document_drop", "call_inner_done", "patient_documents_dropped"),
            ("patient_clause_drop", "patient_documents_dropped", "patient_clauses_dropped"),
            ("arena_rewind", "patient_clauses_dropped", "arena_rewound"),
            ("metrics_write", "metrics_write_begin", "metrics_write_end"),
            ("owner_call_to_pre_exit", "call_begin", "pre_process_exit"),
        ]:
            owner_span(name, begin, end)
        if "answer_sink_closed" in owner:
            owner_span("answer_sink_write_close", "answer_sink_write_begin", "answer_sink_closed")
            owner_span("sink_marker_publish", "answer_sink_closed", "sink_marker_published")
        else:
            owner_span("stdout_write_flush", "stdout_write_begin", "stdout_flushed")
            owner_span("stdout_guard_drop", "stdout_flushed", "stdout_guard_dropped")
        for name, begin, end, role in [
            ("controller_spawn", "spawn_begin", "spawned", "orchestrator"),
            ("controller_register", "spawned", "registered", "orchestrator"),
            ("exit_boundary_scan", "root_nonlive_or_limit", "exit_boundary_done", "orchestrator"),
            ("wait_reap", "wait_begin", "wait_done", "orchestrator"),
            ("post_wait_scan", "wait_done", "post_wait_boundary_done", "orchestrator"),
            ("answer_sink_digest_rename", "answer_sink_digest_begin", "answer_sink_renamed", "controller"),
            ("sampler_finish", "sampler_finish_begin", "sampler_finish_end", "controller"),
            ("publish", "publish_begin", "publish_end", "controller"),
        ]:
            parent_span(name, begin, end, role)
        final = parent["orchestrator", "root_nonlive_or_limit"]
        assert final["child_state"] in ("Z", "X") and final["child_pid"] == owner["pre_process_exit"]["pid"]
        lifetime = int(final["wall_ns"]) - int(parent["orchestrator", "spawn_begin"]["wall_ns"])
        add("startup_plus_post_pre_exit", (lifetime - int(owner["pre_process_exit"]["wall_ns"])) / 1e9)
        # Separate origins cancel in CPU: both counters cover the owner's full life.
        # utime/stime are tick-quantized; negative values within two ticks are noise.
        cpu_delta = int(final["child_own_ticks"]) / ticks - int(owner["pre_process_exit"]["cpu_ns"]) / 1e9
        add("post_pre_exit_CPU_tick_quantized", cpu_delta)
        own_pid = final["child_pid"]
        live_samples = [row for row in rows(stage / "rss-processes.tsv")
                        if row["pid"] == own_pid and row["state"] not in ("Z", "X")]
        assert live_samples
        last = max(live_samples, key=lambda row: int(row["sample_end_ns"]))
        add("last_live_owner_RSS", int(last["rss_bytes"]), "bytes")
        add("whole_scored", int(run["scored_ns"]) / 1e9)
    with (root / "stage-deltas.tsv").open("w") as stream:
        writer = csv.writer(stream, delimiter="\t", lineterminator="\n")
        writer.writerow(("run", "block", "order", "arm", "stage", "units", "value"))
        writer.writerows(observations)
    groups = {}
    for _, _, _, arm, name, units, value in observations:
        groups.setdefault((arm, name, units), []).append(value)
    with (root / "stage-summary.tsv").open("w") as stream:
        writer = csv.writer(stream, delimiter="\t", lineterminator="\n")
        writer.writerow(("arm", "stage", "units", "samples", "median", "max"))
        for (arm, name, units), values in sorted(groups.items()):
            assert len(values) == 3
            writer.writerow((arm, name, units, 3, f"{statistics.median(values):.9f}", f"{max(values):.9f}"))
    changes = []
    for _, name, units in groups:
        if units != "seconds":
            continue
        change = statistics.median(groups["target", name, units]) - statistics.median(groups["baseline", name, units])
        changes.append((change, name))
    print("D3 stage deltas: 6 complete paired runs; separate wall origins; CPU uncertainty <=2/CLK_TCK seconds")
    for change, name in sorted(set(changes), reverse=True)[:10]:
        print(name, f"target-minus-baseline median={change:.9f} s")


if __name__ == "__main__":
    main()
