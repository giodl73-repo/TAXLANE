"""Export chart-ready CSV views from income-tax outlay model JSONL."""

from __future__ import annotations

import argparse
import csv
import json
import math
from collections import defaultdict
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[3]
MODEL_DIR = ROOT / "data" / "derived" / "income_tax_outlay_model"
ANNUAL_JSONL = (
    MODEL_DIR / "income_tax_outlay_model.omb-fy2027.2026-06-21.draft.jsonl"
)
DECADE_JSONL = (
    MODEL_DIR
    / "income_tax_outlay_model.omb-fy2027.2026-06-21.decade-summary.jsonl"
)
ANNUAL_CSV = (
    MODEL_DIR / "income_tax_outlay_model.omb-fy2027.2026-06-21.annual-wide.csv"
)
DECADE_CSV = (
    MODEL_DIR / "income_tax_outlay_model.omb-fy2027.2026-06-21.decade-wide.csv"
)

CATEGORIES = [
    ("national-defense", "national_defense_percent"),
    ("human-resources", "human_resources_percent"),
    ("physical-resources", "physical_resources_percent"),
    ("net-interest", "net_interest_percent"),
    ("other-functions", "other_functions_percent"),
    ("undistributed-offsetting-receipts", "offsetting_receipts_percent"),
]


def read_jsonl(path: Path) -> list[dict[str, Any]]:
    return [json.loads(line) for line in path.read_text().splitlines()]


def annual_rows() -> list[dict[str, Any]]:
    grouped: dict[int, dict[str, dict[str, Any]]] = defaultdict(dict)
    for row in read_jsonl(ANNUAL_JSONL):
        grouped[row["fiscal_year"]][row["category_key"]] = row

    rows: list[dict[str, Any]] = []
    for year in sorted(grouped):
        categories = grouped[year]
        anchor = categories["national-defense"]
        out: dict[str, Any] = {
            "fiscal_year": year,
            "coverage_note": "full_year",
            "individual_income_tax_receipts_millions": anchor[
                "individual_income_tax_receipts_amount"
            ],
            "total_outlays_millions": anchor["total_outlays_amount"],
            "total_receipts_millions": anchor["total_receipts_amount"],
            "deficit_gap_millions": anchor["deficit_gap_amount"],
            "borrowed_share_percent_of_outlays": anchor[
                "borrowed_share_percent_of_outlays"
            ],
            "income_tax_coverage_percent_of_outlays": anchor[
                "income_tax_coverage_percent_of_outlays"
            ],
            "allocation_method": anchor["allocation_method"],
            "legal_allocation_status": anchor["legal_allocation_status"],
            "actual_or_projection": anchor["actual_or_projection"],
        }
        percent_sum = 0.0
        for key, field in CATEGORIES:
            percent = categories[key]["allocation_share_percent"]
            out[field] = percent
            percent_sum += percent
        out["category_percent_sum"] = round(percent_sum, 6)
        rows.append(out)
    return rows


def decade_rows() -> list[dict[str, Any]]:
    grouped: dict[str, dict[str, dict[str, Any]]] = defaultdict(dict)
    for row in read_jsonl(DECADE_JSONL):
        grouped[row["decade"]][row["category_key"]] = row

    rows: list[dict[str, Any]] = []
    for decade in sorted(grouped):
        categories = grouped[decade]
        anchor = categories["national-defense"]
        out: dict[str, Any] = {
            "decade": decade,
            "start_fiscal_year": anchor["start_fiscal_year"],
            "end_fiscal_year": anchor["end_fiscal_year"],
            "year_count": anchor["year_count"],
            "coverage_note": anchor["coverage_note"],
            "cumulative_individual_income_tax_receipts_millions": anchor[
                "cumulative_individual_income_tax_receipts_amount"
            ],
            "cumulative_total_outlays_millions": anchor[
                "cumulative_total_outlays_amount"
            ],
            "cumulative_total_receipts_millions": anchor[
                "cumulative_total_receipts_amount"
            ],
            "cumulative_deficit_gap_millions": anchor[
                "cumulative_deficit_gap_amount"
            ],
            "borrowed_share_percent_of_outlays": anchor[
                "borrowed_share_percent_of_outlays"
            ],
            "income_tax_coverage_percent_of_outlays": anchor[
                "income_tax_coverage_percent_of_outlays"
            ],
            "allocation_method": anchor["allocation_method"],
            "legal_allocation_status": anchor["legal_allocation_status"],
            "actual_or_projection": anchor["actual_or_projection"],
        }
        percent_sum = 0.0
        for key, field in CATEGORIES:
            percent = categories[key]["category_percent_of_decade_income_tax"]
            out[field] = percent
            percent_sum += percent
        out["category_percent_sum"] = round(percent_sum, 6)
        rows.append(out)
    return rows


def validate(rows: list[dict[str, Any]], label: str, expected_count: int) -> list[str]:
    errors: list[str] = []
    if len(rows) != expected_count:
        errors.append(f"{label}: expected {expected_count} rows, found {len(rows)}")
    for row in rows:
        total = row["category_percent_sum"]
        if not math.isclose(total, 100.0, abs_tol=0.00001):
            errors.append(f"{label}: percent sum {total} for {row}")
        if row["legal_allocation_status"] != "modeled_not_legal_dedication":
            errors.append(f"{label}: missing modeled legal status for {row}")
        if row["actual_or_projection"] != "actual":
            errors.append(f"{label}: unexpected projection status for {row}")
    return errors


def write_csv(path: Path, rows: list[dict[str, Any]]) -> None:
    if not rows:
        raise ValueError(f"no rows for {path}")
    with path.open("w", encoding="utf-8", newline="") as f:
        writer = csv.DictWriter(f, fieldnames=list(rows[0].keys()))
        writer.writeheader()
        writer.writerows(rows)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()

    annual = annual_rows()
    decade = decade_rows()
    errors = validate(annual, "annual", 86)
    errors.extend(validate(decade, "decade", 9))
    if errors:
        for error in errors:
            print(error)
        return 1
    if not args.check:
        write_csv(ANNUAL_CSV, annual)
        write_csv(DECADE_CSV, decade)
    print(f"validated {len(annual)} annual rows and {len(decade)} decade rows")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
