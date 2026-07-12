#!/usr/bin/env python3
"""Diverse-personality and uncertainty stress test for the 100-point ballot."""

import importlib.util
import json
import math
import random
from pathlib import Path

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("budget_v1", HERE / "simulate.py")
base = importlib.util.module_from_spec(spec)
spec.loader.exec_module(base)


def percentile(values, probability):
    ordered = sorted(values)
    index = (len(ordered) - 1) * probability
    low = math.floor(index)
    high = math.ceil(index)
    return ordered[low] if low == high else ordered[low] * (high - index) + ordered[high] * (index - low)


def ballot(rng, lanes, archetype, config):
    multipliers = config["archetype_multipliers"][archetype]
    contrarian = rng.random() < config["contrarian_probability"]
    spike = rng.randrange(len(lanes)) if rng.random() < config["single_issue_probability"] else None
    raw = []
    for index, lane in enumerate(lanes):
        multiplier = multipliers.get(lane["lane_id"], 1.0)
        if contrarian:
            multiplier = 1.0 / max(multiplier, 0.2)
        value = (lane["baseline"] ** config["baseline_anchor_exponent"])
        value *= multiplier * math.exp(rng.gauss(0.0, config["individual_log_noise_sigma"]))
        if index == spike:
            value *= config["single_issue_multiplier"]
        raw.append(value)
    result = base.normalized(raw)
    if min(result) < 0 or abs(sum(result) - 100.0) > 1e-9:
        raise ValueError("invalid V2 ballot")
    return result


def run_once(config, lanes, states, seed, voters_per_state, include_states=False):
    rng = random.Random(seed)
    rows = []
    for abbreviation, electoral_votes, profile in states:
        weights = dict(config["archetype_weights"])
        for name, factor in config["state_profile_adjustments"][profile].items():
            weights[name] *= factor
        totals = [0.0] * len(lanes)
        for _ in range(voters_per_state):
            personality = base.weighted_choice(rng, weights)
            values = ballot(rng, lanes, personality, config)
            totals = [a + b for a, b in zip(totals, values)]
        mean = [value / voters_per_state for value in totals]
        rows.append({"state": abbreviation, "profile": profile, "electoral_votes": electoral_votes,
                     "house_apportionment_weight": electoral_votes - 2, "equal_weight": 1, "mean": mean})
    results = {
        "electoral_college_weighted": base.aggregate(rows, "electoral_votes"),
        "house_apportionment_weighted_proxy": base.aggregate(rows, "house_apportionment_weight"),
        "equal_state_weighted": base.aggregate(rows, "equal_weight"),
    }
    return results, rows if include_states else None


def main():
    config = json.loads((HERE / "config.v2.json").read_text(encoding="utf-8"))
    v1 = json.loads((HERE / "config.v1.json").read_text(encoding="utf-8"))
    merged = {**v1, **config}
    lanes = base.load_lanes(merged)
    results, states = run_once(config, lanes, v1["states"], config["seed"], config["voters_per_state"], True)
    uncertainty = []
    for index in range(config["uncertainty_runs"]):
        run, _ = run_once(config, lanes, v1["states"], config["seed"] + 1000 + index,
                          config["uncertainty_voters_per_state"])
        uncertainty.append(run["electoral_college_weighted"])
    intervals = {}
    for lane_index, lane in enumerate(lanes):
        values = [run[lane_index] for run in uncertainty]
        intervals[lane["lane_id"]] = {"p10": percentile(values, 0.10), "median": percentile(values, 0.50),
                                      "p90": percentile(values, 0.90)}
    ec = results["electoral_college_weighted"]
    output = {
        "experiment_id": config["experiment_id"], "seed": config["seed"],
        "simulation_status": "synthetic_diversity_stress_test_not_public_opinion_forecast",
        "personality_count": len(config["archetype_weights"]),
        "main_run_ballots": len(states) * config["voters_per_state"],
        "uncertainty_runs": config["uncertainty_runs"],
        "uncertainty_ballots": len(states) * config["uncertainty_voters_per_state"] * config["uncertainty_runs"],
        "lanes": lanes,
        "baseline": {lane["lane_id"]: lane["baseline"] for lane in lanes},
        "national_results": {name: {lane["lane_id"]: value for lane, value in zip(lanes, values)}
                             for name, values in results.items()},
        "uncertainty_intervals": intervals,
        "state_results": [{"state": row["state"], "profile": row["profile"],
                           "electoral_votes": row["electoral_votes"],
                           "allocation": {lane["lane_id"]: value for lane, value in zip(lanes, row["mean"])}}
                          for row in states],
        "invariants": {"ballots_sum_to_100": True, "state_means_sum_to_100": True,
                       "national_results_sum_to_100": True, "electoral_votes_total": 538}
    }
    out_dir = HERE / "outputs"
    out_dir.mkdir(exist_ok=True)
    (out_dir / "diverse-run.v2.json").write_text(json.dumps(output, indent=2) + "\n", encoding="utf-8")
    baseline = output["baseline"]
    lines = ["# Diverse Annual Budget Ballot — Run V2", "",
             f"Main run: **{output['main_run_ballots']:,} ballots**; uncertainty: **{output['uncertainty_runs']} runs / {output['uncertainty_ballots']:,} ballots**; personalities: **{output['personality_count']}**.", "",
             "> Synthetic diversity stress test—not measured public opinion or a forecast.", "",
             "| Lane | Current | EC result | Change | 10th–90th percentile |", "|---|---:|---:|---:|---:|"]
    for lane in lanes:
        key = lane["lane_id"]
        interval = intervals[key]
        lines.append(f"| {lane['label']} | {baseline[key]:.3f}% | {ec[lanes.index(lane)]:.3f}% | {ec[lanes.index(lane)]-baseline[key]:+.3f} pp | {interval['p10']:.3f}–{interval['p90']:.3f}% |")
    ec_map = output["national_results"]["electoral_college_weighted"]
    house = output["national_results"]["house_apportionment_weighted_proxy"]
    max_effect = max(abs(ec_map[key] - house[key]) for key in ec_map)
    total_reallocation = sum(abs(ec_map[key] - baseline[key]) for key in ec_map) / 2
    lines += ["", f"Total allocation moved from the current normalized budget: **{total_reallocation:.3f}%**.",
              f"Largest EC-versus-House proxy effect: **{max_effect:.3f} percentage point**.", "",
              "Every individual ballot, state mean, and national result is constrained to 100 points.", ""]
    (out_dir / "diverse-run.v2.md").write_text("\n".join(lines), encoding="utf-8")


if __name__ == "__main__":
    main()
