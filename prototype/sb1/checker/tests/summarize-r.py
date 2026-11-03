#!/usr/bin/env python3
"""Summarize every locked development R sample; never discard a slow or failed run."""

import argparse
import csv
from pathlib import Path
import statistics


def rows(path):
    with path.open() as stream:
        return list(csv.DictReader(stream, delimiter="\t"))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("directory", type=Path)
    args = parser.parse_args()
    output = args.directory.resolve()
    phases = []
    counts = []
    for family in ("F1b", "F1a"):
        directory = output / family
        runs = rows(directory / "runs.tsv")
        assert runs and all(row["outcome"] == "verified" for row in runs), (family, "incomplete campaign")
        blocks = {row["block"] for row in runs}
        assert len(runs) == 2 * len(blocks)
        for block in blocks:
            assert sorted(row["arm"] for row in runs if row["block"] == block) == ["baseline", "target"], (family, block, "unpaired block")
        medians = {}
        for arm in ("baseline", "target"):
            selected = [row for row in runs if row["arm"] == arm]
            assert len(selected) * 2 == len(runs), (family, arm, "unpaired campaign")
            by_phase = {}
            cardinalities = []
            for row in selected:
                stage = directory / f"run-{int(row['run']):06}"
                scored = 0
                for phase in rows(stage / "phases.tsv"):
                    duration = int(phase["wall_ns"])
                    by_phase.setdefault((phase["phase"], phase["kind"]), []).append(duration)
                    if phase["kind"] == "scored":
                        scored += duration
                gap = int(row["scored_ns"]) - scored
                assert gap >= 0, (family, arm, "overlapping charged phases")
                by_phase.setdefault(("scored_gaps", "scored"), []).append(gap)
                by_phase.setdefault(("scored_total", "total"), []).append(int(row["scored_ns"]))
                by_phase.setdefault(("load_total", "load"), []).append(int(row["load_ns"]))
                if "teardown_ns" in row:
                    by_phase.setdefault(("teardown", "load-lifecycle"), []).append(int(row["teardown_ns"]))
                    assert int(row["scored_ns"]) + int(row["load_ns"]) + int(row["teardown_ns"]) == int(row["wall_ns"])
                counters = {item["counter"]: item["value"] for item in rows(stage / "checker-counters.tsv")}
                cardinalities.append((counters["selected_clauses"], counters["model_facts"]))
            for (name, kind), values in by_phase.items():
                assert len(values) == len(selected), (family, arm, name, "missing phase")
                phases.append((family, arm, name, kind, len(values),
                               f"{statistics.median(values) / 1e9:.9f}", f"{max(values) / 1e9:.9f}"))
            median = statistics.median(int(row["scored_ns"]) for row in selected) / 1e9
            medians[arm] = median
            peak = max(int(row["peak_rss_bytes"]) for row in selected)
            counts.append((family, arm, len(selected),
                           ",".join(sorted({pair[0] for pair in cardinalities})),
                           ",".join(sorted({pair[1] for pair in cardinalities})), peak))
            print(f"{family} {arm}: n={len(selected)} median_scored_s={median:.9f}; max_sampled_RSS_GB={peak / 1e9:.9f}")
        ratio = rows(directory / "ratio.tsv")
        assert len(ratio) == 1 and ratio[0]["status"] == "complete"
        calculated = medians["target"] / medians["baseline"]
        assert abs(calculated - float(ratio[0]["ratio_of_medians"])) < 1e-8
        print(f"{family}: R={calculated:.9f}; one_sided_95_upper={ratio[0]['one_sided_95_upper']}; development only")
    with (output / "phase-summary.tsv").open("w") as stream:
        writer = csv.writer(stream, delimiter="\t", lineterminator="\n")
        writer.writerow(("family", "arm", "phase", "kind", "samples", "median_seconds", "max_seconds"))
        writer.writerows(phases)
    with (output / "cardinalities.tsv").open("w") as stream:
        writer = csv.writer(stream, delimiter="\t", lineterminator="\n")
        writer.writerow(("family", "arm", "samples", "selected_clauses", "model_facts", "max_sampled_rss_bytes"))
        writer.writerows(counts)


if __name__ == "__main__":
    main()
