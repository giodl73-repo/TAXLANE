"""Run the independent Taxlane TY2026 rate-sensitivity grid."""

from __future__ import annotations

import argparse
import json
import math
from pathlib import Path

from behresp import response
from taxcalc import DIST_VARIABLES, Policy
from taxcalc.utils import create_difference_table

from run import RATE_PARAMETERS, YEAR, make_calculator, weighted_billions


DEFAULT_UPLIFTS = [9.5, 10.0, 10.159, 10.5, 10.922, 11.0, 11.5, 11.854, 12.0]
DEFAULT_ELASTICITIES = [0.15, 0.25, 0.35]
DEFAULT_FIRST_YEAR_RATIO = 0.774223895
DEFAULT_TARGET_BILLIONS = 813.727


def rounded_or_none(value: float, digits: int) -> float | None:
    numeric = float(value)
    return round(numeric, digits) if math.isfinite(numeric) else None


def evaluate_grid(
    uplifts: list[float],
    elasticities: list[float],
    first_year_ratio: float,
    target_billions: float,
) -> dict:
    baseline = make_calculator()
    baseline_frame = baseline.dataframe(DIST_VARIABLES)
    baseline_iitax = weighted_billions(baseline_frame, "iitax")
    policy = Policy()
    policy.set_year(YEAR)
    current_rates = [
        round(float(getattr(policy, name)[0]) * 100.0, 6)
        for name in RATE_PARAMETERS
    ]
    candidates = []

    for uplift in uplifts:
        reform = make_calculator(uplift)
        reform_frame = reform.dataframe(DIST_VARIABLES)
        static_table = create_difference_table(
            baseline_frame, reform_frame, "weighted_deciles", "iitax"
        )
        elasticity_cases = []
        central_distribution = None
        for elasticity in elasticities:
            _, behavioral_reform = response(
                baseline,
                reform,
                {"sub": elasticity, "inc": 0.0, "cg": 0.0},
            )
            behavioral_table = create_difference_table(
                baseline_frame,
                behavioral_reform,
                "weighted_deciles",
                "iitax",
            )
            liability_change = float(behavioral_table.loc["ALL", "tot_change"])
            cash_proxy = liability_change * first_year_ratio
            case = {
                "substitution_elasticity": elasticity,
                "full_year_liability_change_billions": round(liability_change, 3),
                "first_year_cash_proxy_billions": round(cash_proxy, 3),
                "target_difference_billions": round(cash_proxy - target_billions, 3),
                "target_coverage_percent": round(cash_proxy / target_billions * 100.0, 3),
                "mean_tax_change_dollars": round(
                    float(behavioral_table.loc["ALL", "mean"]), 2
                ),
                "after_tax_income_change_percent": round(
                    float(behavioral_table.loc["ALL", "pc_aftertaxinc"]), 3
                ),
                "top_decile_share_percent": round(
                    float(behavioral_table.loc["90-100", "share_of_change"]), 2
                ),
            }
            elasticity_cases.append(case)
            if elasticity == 0.25:
                central_distribution = [
                    {
                        "group": str(index),
                        "change_billions": rounded_or_none(row["tot_change"], 3),
                        "share_of_change_percent": rounded_or_none(
                            row["share_of_change"], 2
                        ),
                        "mean_tax_change_dollars": rounded_or_none(row["mean"], 2),
                        "after_tax_income_change_percent": rounded_or_none(
                            row["pc_aftertaxinc"], 3
                        ),
                    }
                    for index, row in behavioral_table.iterrows()
                ]

        candidates.append(
            {
                "uniform_uplift_points": uplift,
                "schedule_percent": [round(rate + uplift, 6) for rate in current_rates],
                "static_full_year_change_billions": round(
                    float(static_table.loc["ALL", "tot_change"]), 3
                ),
                "elasticity_cases": elasticity_cases,
                "central_distribution": central_distribution,
            }
        )

    return {
        "record_family": "rev_internal_rate_sensitivity_grid_run",
        "version": "v1.generated",
        "as_of_date": "2026-07-27",
        "analysis_scope": "independent_taxlane_analysis_only",
        "model": "Tax-Calculator 6.5.1",
        "data": "bundled CPS tax-unit file",
        "tax_year": YEAR,
        "baseline_iitax_billions": round(baseline_iitax, 3),
        "baseline_schedule_percent": current_rates,
        "first_year_ratio": first_year_ratio,
        "fy2026_revenue_target_billions": target_billions,
        "elasticities": elasticities,
        "candidates": candidates,
        "boundary": {
            "official_request_planned": False,
            "official_score": False,
            "taxlane_internal_analysis": True,
            "administration_macro_and_debt_applied": False,
        },
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--uplifts", type=float, nargs="*", default=DEFAULT_UPLIFTS)
    parser.add_argument(
        "--elasticities", type=float, nargs="*", default=DEFAULT_ELASTICITIES
    )
    parser.add_argument("--first-year-ratio", type=float, default=DEFAULT_FIRST_YEAR_RATIO)
    parser.add_argument("--target-billions", type=float, default=DEFAULT_TARGET_BILLIONS)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    result = evaluate_grid(
        args.uplifts,
        args.elasticities,
        args.first_year_ratio,
        args.target_billions,
    )
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2, allow_nan=False) + "\n")
    print(json.dumps({
        "output": str(args.output),
        "candidate_count": len(result["candidates"]),
        "elasticity_count": len(result["elasticities"]),
        "official_request_planned": False,
    }))


if __name__ == "__main__":
    main()
