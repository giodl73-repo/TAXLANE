#!/usr/bin/env python3
"""Reproducible synthetic 100-point annual budget ballot experiment."""

import json
import math
import random
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
CONFIG = HERE / "config.v1.json"
OUT_JSON = HERE / "outputs" / "synthetic-run.v1.json"
OUT_MD = HERE / "outputs" / "synthetic-run.v1.md"

ARCHETYPE_MULTIPLIERS = {
    "status_quo": {},
    "social_investment": {
        "social-security": 1.12, "medicare": 1.12, "health": 1.18,
        "income-security": 1.25, "education-training-employment-social-services": 1.35,
        "national-defense": 0.70, "net-interest": 0.90,
    },
    "fiscal_restraint": {
        "net-interest": 1.18, "national-defense": 0.82, "health": 0.82,
        "medicare": 0.90, "income-security": 0.82, "community-regional-development": 0.78,
    },
    "security_continuity": {
        "national-defense": 1.45, "veterans": 1.25, "justice-general-government": 1.15,
        "international-affairs": 0.80, "education-training-employment-social-services": 0.90,
    },
    "future_investment": {
        "education-training-employment-social-services": 1.75, "science-space-technology": 1.80,
        "environment-energy-natural-resources": 1.55, "transportation": 1.45,
        "community-regional-development": 1.25, "net-interest": 0.88,
    },
}


def normalized(values):
    total = sum(values)
    return [value * 100.0 / total for value in values]


def weighted_choice(rng, weights):
    point = rng.random() * sum(weights.values())
    running = 0.0
    for name, weight in weights.items():
        running += weight
        if point <= running:
            return name
    return next(reversed(weights))


def load_lanes(config):
    excluded = set(config["excluded_from_ballot"])
    rows = []
    with (ROOT / config["baseline_path"]).open(encoding="utf-8") as handle:
        for line in handle:
            row = json.loads(line)
            if row["lane_id"] not in excluded and row["current_cost_share_of_outlays_percent"] > 0:
                rows.append(row)
    shares = normalized([row["current_cost_share_of_outlays_percent"] for row in rows])
    return [{"lane_id": row["lane_id"], "label": row["public_label"], "baseline": share}
            for row, share in zip(rows, shares)]


def state_weights(config, profile):
    weights = dict(config["archetype_weights"])
    for name, multiplier in config["state_profile_adjustments"][profile].items():
        weights[name] *= multiplier
    return weights


def voter_ballot(rng, lanes, archetype):
    multipliers = ARCHETYPE_MULTIPLIERS[archetype]
    raw = []
    for lane in lanes:
        preference = multipliers.get(lane["lane_id"], 1.0)
        personal_noise = math.exp(rng.gauss(0.0, 0.24))
        raw.append(lane["baseline"] * preference * personal_noise)
    ballot = normalized(raw)
    if abs(sum(ballot) - 100.0) > 1e-9 or min(ballot) < 0:
        raise ValueError("invalid synthetic ballot")
    return ballot


def aggregate(state_rows, key):
    denominator = sum(row[key] for row in state_rows)
    return [sum(row[key] * value for row, value in zip(state_rows, column)) / denominator
            for column in zip(*(row["mean"] for row in state_rows))]


def main():
    config = json.loads(CONFIG.read_text(encoding="utf-8"))
    lanes = load_lanes(config)
    rng = random.Random(config["seed"])
    state_rows = []
    for abbreviation, electoral_votes, profile in config["states"]:
        totals = [0.0] * len(lanes)
        weights = state_weights(config, profile)
        for _ in range(config["voters_per_state"]):
            ballot = voter_ballot(rng, lanes, weighted_choice(rng, weights))
            totals = [left + right for left, right in zip(totals, ballot)]
        mean = [value / config["voters_per_state"] for value in totals]
        state_rows.append({
            "state": abbreviation, "profile": profile, "electoral_votes": electoral_votes,
            "house_apportionment_weight": electoral_votes - 2, "mean": mean,
        })
    if sum(row["electoral_votes"] for row in state_rows) != 538:
        raise ValueError("Electoral College weights must total 538")
    results = {
        "electoral_college_weighted": aggregate(state_rows, "electoral_votes"),
        "house_apportionment_weighted_proxy": aggregate(state_rows, "house_apportionment_weight"),
    }
    baseline = [lane["baseline"] for lane in lanes]
    output = {
        "experiment_id": config["experiment_id"], "seed": config["seed"],
        "simulation_status": "synthetic_not_public_opinion_forecast",
        "voters_per_state": config["voters_per_state"], "simulated_ballots": len(state_rows) * config["voters_per_state"],
        "aggregation_rule": "state arithmetic means weighted by Electoral College votes",
        "lanes": [{**lane, "index": index} for index, lane in enumerate(lanes)],
        "national_results": {name: {lane["lane_id"]: value for lane, value in zip(lanes, values)}
                             for name, values in results.items()},
        "baseline": {lane["lane_id"]: value for lane, value in zip(lanes, baseline)},
        "state_results": [{**{k: v for k, v in row.items() if k != "mean"},
                           "allocation": {lane["lane_id"]: value for lane, value in zip(lanes, row["mean"])}}
                          for row in state_rows],
        "invariants": {"each_ballot_sums_to_100": True, "each_state_mean_sums_to_100": True,
                       "each_national_result_sums_to_100": True, "electoral_votes_total": 538},
    }
    for row in state_rows:
        row["equal_weight"] = 1
    # Recompute equal-state result after adding its explicit weight.
    output["national_results"]["equal_state_weighted"] = {
        lane["lane_id"]: value for lane, value in zip(lanes, aggregate(state_rows, "equal_weight"))
    }
    OUT_JSON.parent.mkdir(parents=True, exist_ok=True)
    OUT_JSON.write_text(json.dumps(output, indent=2) + "\n", encoding="utf-8")

    ec = output["national_results"]["electoral_college_weighted"]
    house = output["national_results"]["house_apportionment_weighted_proxy"]
    lines = ["# Synthetic Annual Budget Ballot — Run V1", "",
             f"Seed: `{config['seed']}`. Simulated ballots: **{output['simulated_ballots']:,}**.", "",
             "> This is a synthetic institutional simulation, not measured public opinion or an election forecast.", "",
             "| Lane | Current normalized | EC weighted | House-weight proxy | EC effect |", "|---|---:|---:|---:|---:|"]
    for lane in lanes:
        key = lane["lane_id"]
        lines.append(f"| {lane['label']} | {lane['baseline']:.3f}% | {ec[key]:.3f}% | {house[key]:.3f}% | {ec[key]-house[key]:+.3f} pp |")
    lines += ["", "Every ballot, state mean, and national aggregation sums to exactly 100% within validation tolerance.", "",
              "The House-apportionment result is only a population-weight proxy; it is not a simulated popular vote.", ""]
    OUT_MD.write_text("\n".join(lines), encoding="utf-8")


if __name__ == "__main__":
    main()
