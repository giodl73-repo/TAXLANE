"""Build goal-seeking peer-informed savings and reinvestment profiles.

The spending amounts in these profiles are policy envelopes, not estimates or
admitted savings.  The rate implications are calculated from a Tax-Calculator
grid produced by ``run_grid.py``.
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path


BASE_TARGET_BILLIONS = 813.727
ADMINISTRATION_CEILING_BILLIONS = 0.077
FY2026_AVERAGE_INTEREST_RATE_PERCENT = 3.404
CENTRAL_ELASTICITY = 0.25


PROFILE_DEFINITIONS = [
    {
        "profile_id": "focused_efficiency_and_reinvestment",
        "label": "Focused",
        "summary": "Pursue concentrated health, defense, and owner-attributed operating reforms while reserving funds for the strongest current all-lane needs.",
        "gross_savings_goal_billions": 225.0,
        "savings_initiatives": [
            {"tracks": ["HLT"], "initiative": "provider, pharmaceutical, and administrative cost convergence", "goal_billions": 100.0},
            {"tracks": ["DEF"], "initiative": "strategy-led procurement and force-posture reform", "goal_billions": 75.0},
            {"tracks": ["PAY", "AGR", "JUS", "INT"], "initiative": "owner-attributed payment, procurement, subsidy, and operating reforms", "goal_billions": 50.0},
        ],
        "reinvestment_goal_billions": 75.0,
        "reinvestment_priorities": [
            {"tracks": ["TRN", "HLT", "EDU", "ISF", "VET", "AGR", "DEF", "DIS", "JUS", "SEE", "INT"], "priority": "adaptive needs reserve allocated only after current pain, outcome, urgency, and delivery review", "goal_billions": 75.0},
        ],
        "net_savings_goal_billions": 150.0,
        "lower_tested_uplift_points": 8.8,
        "selected_uplift_points": 8.9,
    },
    {
        "profile_id": "balanced_peer_reallocation",
        "label": "Balanced",
        "summary": "Move farther toward peer health and defense composition while returning one-third of gross savings to the service lanes with the strongest current evidence of remediable pain.",
        "gross_savings_goal_billions": 450.0,
        "savings_initiatives": [
            {"tracks": ["HLT"], "initiative": "provider, pharmaceutical, and administrative cost convergence", "goal_billions": 225.0},
            {"tracks": ["DEF"], "initiative": "strategy-led procurement and force-posture reform", "goal_billions": 150.0},
            {"tracks": ["PAY", "AGR", "JUS", "INT"], "initiative": "owner-attributed payment, procurement, subsidy, and operating reforms", "goal_billions": 75.0},
        ],
        "reinvestment_goal_billions": 150.0,
        "reinvestment_priorities": [
            {"tracks": ["TRN", "HLT", "EDU", "ISF", "VET", "AGR", "DEF", "DIS", "JUS", "SEE", "INT"], "priority": "adaptive needs reserve allocated only after current pain, outcome, urgency, and delivery review", "goal_billions": 150.0},
        ],
        "net_savings_goal_billions": 300.0,
        "lower_tested_uplift_points": 6.7,
        "selected_uplift_points": 6.8,
    },
    {
        "profile_id": "ambitious_peer_reallocation",
        "label": "Ambitious",
        "summary": "Test a large health-price and defense-posture reset while reserving more than one-third of gross savings for the strongest current all-lane needs.",
        "gross_savings_goal_billions": 700.0,
        "savings_initiatives": [
            {"tracks": ["HLT"], "initiative": "provider, pharmaceutical, and administrative cost convergence", "goal_billions": 350.0},
            {"tracks": ["DEF"], "initiative": "strategy-led procurement and force-posture reform", "goal_billions": 225.0},
            {"tracks": ["PAY", "AGR", "JUS", "INT"], "initiative": "owner-attributed payment, procurement, subsidy, and operating reforms", "goal_billions": 125.0},
        ],
        "reinvestment_goal_billions": 250.0,
        "reinvestment_priorities": [
            {"tracks": ["TRN", "HLT", "EDU", "ISF", "VET", "AGR", "DEF", "DIS", "JUS", "SEE", "INT"], "priority": "adaptive needs reserve allocated only after current pain, outcome, urgency, and delivery review", "goal_billions": 250.0},
        ],
        "net_savings_goal_billions": 450.0,
        "lower_tested_uplift_points": 4.7,
        "selected_uplift_points": 4.8,
    },
    {
        "profile_id": "transformative_peer_reallocation",
        "label": "Transformative",
        "summary": "Explore the outer policy frontier: deep health-cost and defense-strategy changes, substantial reinvestment, and a remaining ordinary-income schedule much closer to current law.",
        "gross_savings_goal_billions": 950.0,
        "savings_initiatives": [
            {"tracks": ["HLT"], "initiative": "provider, pharmaceutical, and administrative cost convergence", "goal_billions": 500.0},
            {"tracks": ["DEF"], "initiative": "strategy-led procurement and force-posture reform", "goal_billions": 300.0},
            {"tracks": ["PAY", "AGR", "JUS", "INT"], "initiative": "owner-attributed payment, procurement, subsidy, and operating reforms", "goal_billions": 150.0},
        ],
        "reinvestment_goal_billions": 350.0,
        "reinvestment_priorities": [
            {"tracks": ["TRN", "HLT", "EDU", "ISF", "VET", "AGR", "DEF", "DIS", "JUS", "SEE", "INT"], "priority": "adaptive needs reserve allocated only after current pain, outcome, urgency, and delivery review", "goal_billions": 350.0},
        ],
        "net_savings_goal_billions": 600.0,
        "lower_tested_uplift_points": 2.7,
        "selected_uplift_points": 2.8,
    },
]


def candidate_for(grid: dict, uplift: float) -> dict:
    return next(
        candidate
        for candidate in grid["candidates"]
        if candidate["uniform_uplift_points"] == uplift
    )


def central_case(candidate: dict) -> dict:
    return next(
        case
        for case in candidate["elasticity_cases"]
        if case["substitution_elasticity"] == CENTRAL_ELASTICITY
    )


def final_gap(cash_proxy: float, target: float) -> float:
    pre_interest_gap = cash_proxy - ADMINISTRATION_CEILING_BILLIONS - target
    debt_service_upper = (
        max(-pre_interest_gap, 0.0)
        * FY2026_AVERAGE_INTEREST_RATE_PERCENT
        / 100.0
    )
    return round(pre_interest_gap - debt_service_upper, 3)


def build(grid: dict, taxpayer_profiles: dict) -> dict:
    profiles = []
    for definition in PROFILE_DEFINITIONS:
        target = round(
            BASE_TARGET_BILLIONS - definition["net_savings_goal_billions"], 3
        )
        selected_candidate = candidate_for(grid, definition["selected_uplift_points"])
        lower_candidate = candidate_for(grid, definition["lower_tested_uplift_points"])
        selected_central = central_case(selected_candidate)
        lower_central = central_case(lower_candidate)
        selected_gap = final_gap(
            selected_central["first_year_cash_proxy_billions"], target
        )
        lower_gap = final_gap(lower_central["first_year_cash_proxy_billions"], target)
        if selected_gap < 0 or lower_gap >= 0:
            raise ValueError(
                f"{definition['profile_id']} does not bracket the central target"
            )
        if sum(item["goal_billions"] for item in definition["savings_initiatives"]) != definition["gross_savings_goal_billions"]:
            raise ValueError(f"{definition['profile_id']} gross savings do not add")
        if sum(item["goal_billions"] for item in definition["reinvestment_priorities"]) != definition["reinvestment_goal_billions"]:
            raise ValueError(f"{definition['profile_id']} reinvestment does not add")
        if definition["gross_savings_goal_billions"] - definition["reinvestment_goal_billions"] != definition["net_savings_goal_billions"]:
            raise ValueError(f"{definition['profile_id']} net savings do not reconcile")

        taxpayer_examples = []
        for taxpayer in taxpayer_profiles["profiles"]:
            scenario_tax = round(
                taxpayer["current_law_ordinary_bracket_tax_dollars"]
                + taxpayer["taxable_ordinary_income_dollars"]
                * definition["selected_uplift_points"]
                / 100.0,
                2,
            )
            taxpayer_examples.append(
                {
                    "profile_id": taxpayer["profile_id"],
                    "taxable_ordinary_income_dollars": taxpayer[
                        "taxable_ordinary_income_dollars"
                    ],
                    "scenario_ordinary_bracket_tax_dollars": scenario_tax,
                    "difference_from_current_law_dollars": round(
                        scenario_tax
                        - taxpayer["current_law_ordinary_bracket_tax_dollars"],
                        2,
                    ),
                }
            )

        profiles.append(
            {
                **definition,
                "initiative_status": "goal_seeking_policy_envelopes_not_savings_estimates",
                "admitted_savings_billions": 0.0,
                "remaining_fy2026_revenue_target_billions": target,
                "rate_result_status": "central_rate_implication_model_scored_conditional_on_full_net_savings_goal",
                "selected_central_rate": {
                    "uniform_uplift_points": definition["selected_uplift_points"],
                    "schedule_percent": selected_candidate["schedule_percent"],
                    "first_year_cash_proxy_billions": selected_central[
                        "first_year_cash_proxy_billions"
                    ],
                    "final_target_difference_billions": selected_gap,
                    "mean_tax_change_dollars": selected_central[
                        "mean_tax_change_dollars"
                    ],
                    "after_tax_income_change_percent": selected_central[
                        "after_tax_income_change_percent"
                    ],
                },
                "lower_tested_rate": {
                    "uniform_uplift_points": definition[
                        "lower_tested_uplift_points"
                    ],
                    "schedule_percent": lower_candidate["schedule_percent"],
                    "first_year_cash_proxy_billions": lower_central[
                        "first_year_cash_proxy_billions"
                    ],
                    "final_target_difference_billions": lower_gap,
                    "target_met": False,
                },
                "taxpayer_examples": taxpayer_examples,
                "required_gates": [
                    "same-year federal-to-general-government crosswalk",
                    "concrete statutory and administrative policy instruments",
                    "current-law annual and ten-year score",
                    "coverage, readiness, access, quality, and beneficiary floors",
                    "federal, state, local, and private cost-shift analysis",
                    "transition, implementation, distribution, and compliance effects",
                    "owner attribution and PAY overlap removal",
                    "NET debt-service recomputation and fresh REV microsimulation",
                ],
            }
        )

    return {
        "record_id": "peer-informed-savings-and-reinvestment-profiles:ty2026:v1",
        "record_family": "peer_informed_savings_and_reinvestment_profiles",
        "version": "v1.draft",
        "status": "four_goal_seeking_profiles_with_model_scored_central_rate_implications",
        "as_of_date": "2026-07-28",
        "analysis_scope": "repository_local_independent_taxlane_scenario_analysis",
        "model": {
            "engine": grid["model"],
            "data": grid["data"],
            "tax_year": grid["tax_year"],
            "first_year_ratio": grid["first_year_ratio"],
            "central_substitution_elasticity": CENTRAL_ELASTICITY,
            "administration_ceiling_billions": ADMINISTRATION_CEILING_BILLIONS,
            "fy2026_average_interest_rate_percent": FY2026_AVERAGE_INTEREST_RATE_PERCENT,
            "baseline_current_law_schedule_percent": grid[
                "baseline_schedule_percent"
            ],
            "baseline_fy2026_revenue_target_billions": BASE_TARGET_BILLIONS,
        },
        "peer_direction": {
            "source_basis": "captured OECD COFOG general-government composition; policy envelopes remain federal FY2026 hypotheses",
            "health": "downward cost-convergence research direction",
            "defense": "downward share and strategy research direction",
            "social_protection": "upward or restructuring research direction; eligible for but not guaranteed adaptive reserve funding",
            "direct_tax_rate_comparability": False,
        },
        "allocation_rule": {
            "method": "recurring_needs_and_balance_cycle",
            "framework_path": "data/derived/breadth_benchmark_matrix/adaptive_needs_and_balance_cycle.v1.draft.json",
            "education_automatic_increase": False,
            "peer_gap_automatic_allocation": False,
            "reinvestment_amounts_preallocated_to_service_lanes": False,
            "rule": "Reinvestment remains a profile-level reserve until current pain, outcome-floor risk, urgency, intervention evidence, and delivery capacity are compared across eligible service lanes.",
        },
        "profiles": profiles,
        "current_taxlane_reference": {
            "net_savings_billions": 0.0,
            "remaining_target_billions": BASE_TARGET_BILLIONS,
            "uniform_uplift_points": 11.0,
            "schedule_percent": [21.0, 23.0, 33.0, 35.0, 43.0, 46.0, 48.0],
        },
        "full_current_law_rate_reference": {
            "net_savings_required_to_remove_scoped_target_billions": BASE_TARGET_BILLIONS,
            "uniform_uplift_points": 0.0,
            "schedule_percent": grid["baseline_schedule_percent"],
            "warning": "This arithmetic endpoint removes the scoped ordinary-income target; it is not evidence that such savings exist or that the overall federal budget is balanced.",
        },
        "decision": {
            "scenario_profiles_ready": True,
            "central_rate_implications_model_scored": True,
            "initiative_savings_estimates_ready": False,
            "initiative_savings_admitted": False,
            "official_score_ready": False,
            "public_release_authorized": False,
        },
        "public_warning": "Every spending and reinvestment amount is a goal-seeking policy envelope, not an estimate, recommendation, admitted saving, or claim that peer composition can be copied. Rate schedules are conditional central-model implications only. No profile is enacted law, an official score, a complete household tax calculation, proof of balance, or public-release authority.",
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--grid", type=Path, nargs="+", required=True)
    parser.add_argument("--taxpayer-profiles", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    grids = [json.loads(path.read_text()) for path in args.grid]
    grid = grids[0]
    grid["candidates"] = sorted(
        [candidate for item in grids for candidate in item["candidates"]],
        key=lambda candidate: candidate["uniform_uplift_points"],
    )
    result = build(grid, json.loads(args.taxpayer_profiles.read_text()))
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2) + "\n")
    print(
        json.dumps(
            {
                "output": str(args.output),
                "profile_count": len(result["profiles"]),
                "initiative_savings_admitted": False,
            }
        )
    )


if __name__ == "__main__":
    main()
