"""Build decade summaries from annual income-tax outlay model rows."""

from __future__ import annotations

import argparse
import json
import math
from collections import defaultdict
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[3]
INPUT_PATH = (
    ROOT
    / "data"
    / "derived"
    / "income_tax_outlay_model"
    / "income_tax_outlay_model.omb-fy2027.2026-06-21.draft.jsonl"
)
JSONL_OUTPUT_PATH = (
    ROOT
    / "data"
    / "derived"
    / "income_tax_outlay_model"
    / "income_tax_outlay_model.omb-fy2027.2026-06-21.decade-summary.jsonl"
)
MD_OUTPUT_PATH = (
    ROOT
    / "data"
    / "derived"
    / "income_tax_outlay_model"
    / "decade-summary.md"
)

CATEGORY_ORDER = [
    "national-defense",
    "human-resources",
    "physical-resources",
    "net-interest",
    "other-functions",
    "undistributed-offsetting-receipts",
]


def decade_label(year: int) -> str:
    start = year - year % 10
    return f"{start}s"


def read_rows() -> list[dict[str, Any]]:
    return [json.loads(line) for line in INPUT_PATH.read_text().splitlines()]


def build_summary() -> tuple[list[dict[str, Any]], list[str]]:
    rows = read_rows()
    errors: list[str] = []
    by_decade: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for row in rows:
        by_decade[decade_label(row["fiscal_year"])].append(row)

    output: list[dict[str, Any]] = []
    for decade in sorted(by_decade):
        decade_rows = by_decade[decade]
        years = sorted({row["fiscal_year"] for row in decade_rows})
        if any(len([r for r in decade_rows if r["fiscal_year"] == year]) != 6 for year in years):
            errors.append(f"{decade}: expected six category rows for each fiscal year")
            continue

        annual_anchor = {
            row["fiscal_year"]: row
            for row in decade_rows
            if row["category_key"] == "national-defense"
        }
        income_tax_total = sum(
            row["individual_income_tax_receipts_amount"]
            for row in annual_anchor.values()
        )
        total_outlays = sum(row["total_outlays_amount"] for row in annual_anchor.values())
        total_receipts = sum(row["total_receipts_amount"] for row in annual_anchor.values())
        deficit_gap = sum(row["deficit_gap_amount"] for row in annual_anchor.values())
        borrowed_share = deficit_gap / total_outlays * 100 if total_outlays else 0
        income_tax_coverage = income_tax_total / total_outlays * 100 if total_outlays else 0

        category_sum = 0.0
        for category in CATEGORY_ORDER:
            category_rows = [row for row in decade_rows if row["category_key"] == category]
            if len(category_rows) != len(years):
                errors.append(f"{decade}: missing {category} rows")
                continue
            modeled_total = sum(
                row["modeled_income_tax_allocation_amount"] for row in category_rows
            )
            category_percent = modeled_total / income_tax_total * 100
            category_sum += category_percent
            output.append(
                {
                    "record_id": f"income-tax-outlay-model:{decade}:{category}:decade-summary",
                    "record_family": "income_tax_outlay_model_decade_summary",
                    "source_record_family": "income_tax_outlay_model",
                    "model_id": "individual-income-tax-proportional-outlays-v1",
                    "decade": decade,
                    "start_fiscal_year": min(years),
                    "end_fiscal_year": max(years),
                    "year_count": len(years),
                    "coverage_note": "partial_decade" if len(years) < 10 else "full_decade",
                    "category_key": category,
                    "category_label": category_rows[0]["category_label"],
                    "cumulative_modeled_income_tax_allocation_amount": round(
                        modeled_total, 6
                    ),
                    "cumulative_individual_income_tax_receipts_amount": round(
                        income_tax_total, 6
                    ),
                    "category_percent_of_decade_income_tax": round(
                        category_percent, 9
                    ),
                    "cumulative_total_outlays_amount": round(total_outlays, 6),
                    "cumulative_total_receipts_amount": round(total_receipts, 6),
                    "cumulative_deficit_gap_amount": round(deficit_gap, 6),
                    "borrowed_share_percent_of_outlays": round(borrowed_share, 9),
                    "income_tax_coverage_percent_of_outlays": round(
                        income_tax_coverage, 9
                    ),
                    "allocation_method": "proportional_outlay_share",
                    "legal_allocation_status": "modeled_not_legal_dedication",
                    "actual_or_projection": "actual",
                    "status": "draft",
                    "notes": (
                        "Decade summary derived from annual modeled allocation rows; "
                        "not legal dedication or program tracing."
                    ),
                }
            )
        if not math.isclose(category_sum, 100.0, abs_tol=0.00001):
            errors.append(f"{decade}: category percentages sum to {category_sum}")

    return output, errors


def write_outputs(rows: list[dict[str, Any]]) -> None:
    with JSONL_OUTPUT_PATH.open("w", encoding="utf-8", newline="\n") as f:
        for row in rows:
            f.write(json.dumps(row, separators=(",", ":")) + "\n")

    by_decade: dict[str, dict[str, dict[str, Any]]] = defaultdict(dict)
    for row in rows:
        by_decade[row["decade"]][row["category_key"]] = row

    lines = [
        "# Decade Summary: Modeled Income-Tax Outlay Allocation",
        "",
        "This table summarizes the annual draft model by decade. Category",
        "percentages equal cumulative modeled category allocations divided by",
        "cumulative individual income-tax receipts for the years in that decade.",
        "The 2020s are partial because the current actual-year model ends in 2025.",
        "",
        "These are modeled allocations, not legal destinations for income-tax",
        "receipts.",
        "",
        "| Decade | Years | National defense | Human resources | Physical resources | Net interest | Other functions | Offsetting receipts | Borrowed share of outlays | Income-tax coverage of outlays |",
        "|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|",
    ]
    for decade in sorted(by_decade):
        first = by_decade[decade][CATEGORY_ORDER[0]]
        years = f"{first['start_fiscal_year']}-{first['end_fiscal_year']}"
        values = [
            by_decade[decade][category]["category_percent_of_decade_income_tax"]
            for category in CATEGORY_ORDER
        ]
        lines.append(
            "| {decade} | {years} | {vals[0]:.1f}% | {vals[1]:.1f}% | "
            "{vals[2]:.1f}% | {vals[3]:.1f}% | {vals[4]:.1f}% | "
            "{vals[5]:.1f}% | {borrowed:.1f}% | {coverage:.1f}% |".format(
                decade=decade,
                years=years,
                vals=values,
                borrowed=first["borrowed_share_percent_of_outlays"],
                coverage=first["income_tax_coverage_percent_of_outlays"],
            )
        )
    lines.append("")
    MD_OUTPUT_PATH.write_text("\n".join(lines), encoding="utf-8", newline="\n")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    rows, errors = build_summary()
    if errors:
        for error in errors:
            print(error)
        return 1
    if not args.check:
        write_outputs(rows)
    print(f"validated {len(rows)} decade summary rows")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
