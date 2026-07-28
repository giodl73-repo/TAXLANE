"""Run a reproducible TY2026 uniform ordinary-rate microsimulation."""

from __future__ import annotations

import argparse
import json
import math

import taxcalc
from behresp import response
from taxcalc import Calculator, DIST_VARIABLES, Policy, Records
from taxcalc.utils import create_difference_table


YEAR = 2026
RATE_PARAMETERS = [f"II_rt{index}" for index in range(1, 8)]


def make_calculator(uplift_points: float = 0.0) -> Calculator:
    policy = Policy()
    if uplift_points:
        policy.set_year(YEAR)
        current_rates = [float(getattr(policy, name)[0]) for name in RATE_PARAMETERS]
        reform = {
            name: {YEAR: rate + uplift_points / 100.0}
            for name, rate in zip(RATE_PARAMETERS, current_rates)
        }
        policy.implement_reform(reform)
    calculator = Calculator(policy=policy, records=Records.cps_constructor())
    calculator.advance_to_year(YEAR)
    calculator.calc_all()
    return calculator


def weighted_billions(frame, variable: str) -> float:
    return float((frame[variable] * frame["s006"]).sum() / 1_000_000_000)


def rounded_or_none(value: float, digits: int) -> float | None:
    numeric = float(value)
    return round(numeric, digits) if math.isfinite(numeric) else None


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--uplift", type=float, required=True)
    parser.add_argument("--sub-elasticity", type=float, default=0.25)
    parser.add_argument(
        "--sensitivity-elasticities", type=float, nargs="*", default=[]
    )
    parser.add_argument("--first-year-ratio", type=float)
    args = parser.parse_args()

    baseline = make_calculator()
    reform = make_calculator(args.uplift)
    baseline_frame = baseline.dataframe(DIST_VARIABLES)
    reform_frame = reform.dataframe(DIST_VARIABLES)
    behavioral_baseline, behavioral_reform = response(
        baseline,
        reform,
        {"sub": args.sub_elasticity, "inc": 0.0, "cg": 0.0},
    )

    static_table = create_difference_table(
        baseline_frame, reform_frame, "weighted_deciles", "iitax"
    )
    behavioral_table = create_difference_table(
        behavioral_baseline, behavioral_reform, "weighted_deciles", "iitax"
    )
    elasticity_sensitivity = []
    for elasticity in args.sensitivity_elasticities:
        _, sensitivity_reform = response(
            baseline,
            reform,
            {"sub": elasticity, "inc": 0.0, "cg": 0.0},
        )
        elasticity_sensitivity.append(
            {
                "substitution_elasticity": elasticity,
                "reform_iitax_billions": round(
                    weighted_billions(sensitivity_reform, "iitax"), 3
                ),
                "change_billions": round(
                    weighted_billions(sensitivity_reform, "iitax")
                    - weighted_billions(baseline_frame, "iitax"),
                    3,
                ),
            }
        )
    decile_names = [
        "negative",
        "zero",
        "positive_to_10",
        "10_to_20",
        "20_to_30",
        "30_to_40",
        "40_to_50",
        "50_to_60",
        "60_to_70",
        "70_to_80",
        "80_to_90",
        "90_to_100",
        "all",
        "90_to_95",
        "95_to_99",
        "top_1",
    ]

    result = {
        "model": "Tax-Calculator",
        "model_version": taxcalc.__version__,
        "data": "bundled CPS tax-unit file",
        "tax_year": YEAR,
        "uniform_uplift_points": args.uplift,
        "substitution_elasticity": args.sub_elasticity,
        "income_elasticity": 0.0,
        "capital_gains_semi_elasticity": 0.0,
        "baseline_iitax_billions": round(weighted_billions(baseline_frame, "iitax"), 3),
        "static_reform_iitax_billions": round(weighted_billions(reform_frame, "iitax"), 3),
        "static_change_billions": round(float(static_table.loc["ALL", "tot_change"]), 3),
        "behavioral_reform_iitax_billions": round(
            weighted_billions(behavioral_reform, "iitax"), 3
        ),
        "behavioral_change_billions": round(
            float(behavioral_table.loc["ALL", "tot_change"]), 3
        ),
        "first_year_cash_proxy_billions": (
            round(
                float(behavioral_table.loc["ALL", "tot_change"])
                * args.first_year_ratio,
                3,
            )
            if args.first_year_ratio is not None
            else None
        ),
        "first_year_ratio": args.first_year_ratio,
        "elasticity_sensitivity": elasticity_sensitivity,
        "behavioral_distribution": [
            {
                "group": name,
                "change_billions": rounded_or_none(row["tot_change"], 3),
                "share_of_change_percent": rounded_or_none(row["share_of_change"], 2),
                "mean_tax_change_dollars": rounded_or_none(row["mean"], 2),
                "after_tax_income_change_percent": rounded_or_none(
                    row["pc_aftertaxinc"], 3
                ),
            }
            for name, (_, row) in zip(decile_names, behavioral_table.iterrows())
        ],
        "boundary": {
            "official_jct_or_cbo_score": False,
            "tax_year_liability_not_fiscal_year_cash": True,
            "administration_cost_modeled": False,
            "macroeconomic_feedback_modeled": False,
        },
    }
    print(json.dumps(result, indent=2, allow_nan=False))


if __name__ == "__main__":
    main()
