"""Apply Taxlane internal uncertainty, accounting, and ranking to the grid."""

from __future__ import annotations

import argparse
import json
from pathlib import Path


ADMINISTRATION_CEILING_BILLIONS = 0.077
MACRO_STRESS_PERCENTS = [-5.0, -2.5, 0.0]
FY2026_AVERAGE_INTEREST_RATE_PERCENT = 3.404
CENTRAL_ELASTICITY = 0.25


def case_for(candidate: dict, elasticity: float) -> dict:
    return next(
        case
        for case in candidate["elasticity_cases"]
        if case["substitution_elasticity"] == elasticity
    )


def analyze(grid: dict) -> dict:
    target = grid["fy2026_revenue_target_billions"]
    candidate_rows = []
    for candidate in grid["candidates"]:
        scenarios = []
        for case in candidate["elasticity_cases"]:
            for macro_percent in MACRO_STRESS_PERCENTS:
                cash = case["first_year_cash_proxy_billions"]
                after_macro = cash * (1.0 + macro_percent / 100.0)
                after_admin = after_macro - ADMINISTRATION_CEILING_BILLIONS
                pre_interest_gap = after_admin - target
                debt_service_upper = (
                    max(-pre_interest_gap, 0.0)
                    * FY2026_AVERAGE_INTEREST_RATE_PERCENT
                    / 100.0
                )
                final_gap = pre_interest_gap - debt_service_upper
                scenarios.append(
                    {
                        "substitution_elasticity": case["substitution_elasticity"],
                        "macro_revenue_stress_percent": macro_percent,
                        "cash_proxy_billions": cash,
                        "administration_ceiling_billions": ADMINISTRATION_CEILING_BILLIONS,
                        "first_order_debt_service_upper_billions": round(
                            debt_service_upper, 3
                        ),
                        "final_target_difference_billions": round(final_gap, 3),
                        "target_met": final_gap >= 0.0,
                    }
                )
        central = case_for(candidate, CENTRAL_ELASTICITY)
        central_no_macro = next(
            scenario
            for scenario in scenarios
            if scenario["substitution_elasticity"] == CENTRAL_ELASTICITY
            and scenario["macro_revenue_stress_percent"] == 0.0
        )
        no_macro_cases = [
            scenario
            for scenario in scenarios
            if scenario["macro_revenue_stress_percent"] == 0.0
        ]
        candidate_rows.append(
            {
                "uniform_uplift_points": candidate["uniform_uplift_points"],
                "schedule_percent": candidate["schedule_percent"],
                "central_cash_proxy_billions": central["first_year_cash_proxy_billions"],
                "central_gap_after_administration_and_debt_billions": central_no_macro[
                    "final_target_difference_billions"
                ],
                "behavior_cases_meeting_target_without_macro_stress": sum(
                    scenario["target_met"] for scenario in no_macro_cases
                ),
                "all_behavior_cases_meet_target_without_macro_stress": all(
                    scenario["target_met"] for scenario in no_macro_cases
                ),
                "stress_cases_meeting_target": sum(
                    scenario["target_met"] for scenario in scenarios
                ),
                "stress_case_count": len(scenarios),
                "mean_tax_change_dollars_central": central["mean_tax_change_dollars"],
                "after_tax_income_change_percent_central": central[
                    "after_tax_income_change_percent"
                ],
                "top_decile_share_percent_central": central[
                    "top_decile_share_percent"
                ],
                "scenarios": scenarios,
            }
        )

    central_fit = next(
        row
        for row in candidate_rows
        if row["central_gap_after_administration_and_debt_billions"] >= 0.0
    )
    behavior_robust = next(
        (
            row
            for row in candidate_rows
            if row["all_behavior_cases_meet_target_without_macro_stress"]
        ),
        None,
    )
    stress_robust = next(
        (
            row
            for row in candidate_rows
            if row["stress_cases_meeting_target"] == row["stress_case_count"]
        ),
        None,
    )
    ranked = sorted(
        candidate_rows,
        key=lambda row: (
            not row["central_gap_after_administration_and_debt_billions"] >= 0.0,
            abs(row["central_gap_after_administration_and_debt_billions"]),
            row["uniform_uplift_points"],
        ),
    )
    return {
        "record_family": "rev_internal_rate_candidate_analysis",
        "version": "v1.generated",
        "as_of_date": "2026-07-27",
        "input_grid_path": "data/derived/breadth_benchmark_matrix/rev_internal_rate_sensitivity_grid_run.v1.generated.json",
        "analysis_scope": "independent_taxlane_analysis_only",
        "uncertainty_contract": {
            "administration_ceiling_billions": ADMINISTRATION_CEILING_BILLIONS,
            "macro_revenue_stress_percents": MACRO_STRESS_PERCENTS,
            "macro_values_are_internal_stresses_not_estimates": True,
            "fy2026_average_interest_rate_percent": FY2026_AVERAGE_INTEREST_RATE_PERCENT,
            "debt_service_is_first_order_full_year_upper_sensitivity": True,
            "pay_additive_contribution_billions": 0.0,
            "net_direct_cut_billions": 0.0,
        },
        "candidate_rows": candidate_rows,
        "ranking_by_central_fit": [row["uniform_uplift_points"] for row in ranked],
        "selection": {
            "lowest_central_case_uplift_meeting_target": central_fit[
                "uniform_uplift_points"
            ],
            "lowest_all_behavior_case_uplift_meeting_target_without_macro_stress": (
                behavior_robust["uniform_uplift_points"]
                if behavior_robust is not None
                else None
            ),
            "lowest_all_stress_case_uplift_meeting_target": (
                stress_robust["uniform_uplift_points"]
                if stress_robust is not None
                else None
            ),
            "strict_stress_grid_extension_required": stress_robust is None,
        },
        "boundary": {
            "official_request_planned": False,
            "official_score": False,
            "taxlane_internal_analysis": True,
            "macro_stresses_are_not_forecasts": True,
            "balanced_budget_proven": False,
        },
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--input", type=Path, nargs="+", required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    grids = [json.loads(path.read_text()) for path in args.input]
    grid = grids[0]
    if any(
        candidate["uniform_uplift_points"]
        in {
            existing["uniform_uplift_points"]
            for existing in grid["candidates"]
        }
        for extra in grids[1:]
        for candidate in extra["candidates"]
    ):
        raise ValueError("candidate grids must not contain duplicate uplifts")
    grid["candidates"] = sorted(
        [candidate for item in grids for candidate in item["candidates"]],
        key=lambda candidate: candidate["uniform_uplift_points"],
    )
    result = analyze(grid)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2, allow_nan=False) + "\n")
    print(json.dumps(result["selection"]))


if __name__ == "__main__":
    main()
