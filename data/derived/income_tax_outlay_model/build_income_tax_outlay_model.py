"""Build the draft yearly individual income-tax outlay allocation model."""

from __future__ import annotations

import argparse
import json
import math
import re
from pathlib import Path
from typing import Any
from xml.etree import ElementTree as ET
from zipfile import ZipFile


ROOT = Path(__file__).resolve().parents[3]
OBSERVED_DATE = "2026-06-21"
MODEL_ID = "individual-income-tax-proportional-outlays-v1"
SOURCE_IDS = [
    "SRC-OMB-HIST-1-1-FY2027",
    "SRC-OMB-HIST-2-1-FY2027",
    "SRC-OMB-HIST-3-1-FY2027",
]
OUTPUT_PATH = (
    ROOT
    / "data"
    / "derived"
    / "income_tax_outlay_model"
    / "income_tax_outlay_model.omb-fy2027.2026-06-21.draft.jsonl"
)
REPORT_PATH = (
    ROOT
    / "data"
    / "derived"
    / "income_tax_outlay_model"
    / "source-profile.md"
)

TABLE_1_1 = (
    ROOT
    / "data"
    / "raw"
    / "omb"
    / "SRC-OMB-HIST-1-1-FY2027"
    / OBSERVED_DATE
    / "hist01z1_fy2027.xlsx"
)
TABLE_2_1 = (
    ROOT
    / "data"
    / "raw"
    / "omb"
    / "SRC-OMB-HIST-2-1-FY2027"
    / OBSERVED_DATE
    / "hist02z1_fy2027.xlsx"
)
TABLE_3_1 = (
    ROOT
    / "data"
    / "raw"
    / "omb"
    / "SRC-OMB-HIST-3-1-FY2027"
    / OBSERVED_DATE
    / "hist03z1_fy2027.xlsx"
)

NS = {"a": "http://schemas.openxmlformats.org/spreadsheetml/2006/main"}

BROAD_CATEGORIES = [
    ("national-defense", "National Defense", 4),
    ("human-resources", "Human resources", 5),
    ("physical-resources", "Physical resources", 14),
    ("net-interest", "Net interest", 22),
    ("other-functions", "Other functions", 25),
    (
        "undistributed-offsetting-receipts",
        "Undistributed offsetting receipts",
        32,
    ),
]


def col_to_num(col: str) -> int:
    num = 0
    for char in col:
        num = num * 26 + ord(char) - ord("A") + 1
    return num


def cell_value(raw: str | None) -> Any:
    if raw is None:
        return None
    value = raw.strip()
    if value in {"", ".........."}:
        return None
    if value == "-*":
        return 0
    try:
        number = float(value)
    except ValueError:
        return value
    if number.is_integer():
        return int(number)
    return number


def read_sheet(path: Path) -> dict[int, dict[str, Any]]:
    with ZipFile(path) as zf:
        shared: list[str] = []
        if "xl/sharedStrings.xml" in zf.namelist():
            root = ET.fromstring(zf.read("xl/sharedStrings.xml"))
            for si in root.findall("a:si", NS):
                shared.append("".join(t.text or "" for t in si.findall(".//a:t", NS)))

        root = ET.fromstring(zf.read("xl/worksheets/sheet1.xml"))
        rows: dict[int, dict[str, Any]] = {}
        for row in root.findall(".//a:sheetData/a:row", NS):
            row_num = int(row.attrib["r"])
            cells: dict[str, Any] = {}
            for cell in row.findall("a:c", NS):
                ref = cell.attrib.get("r", "")
                match = re.match(r"([A-Z]+)", ref)
                if not match:
                    continue
                col = match.group(1)
                cell_type = cell.attrib.get("t")
                value_node = cell.find("a:v", NS)
                raw = None if value_node is None else value_node.text
                if cell_type == "s" and raw is not None:
                    value = shared[int(raw)]
                elif cell_type == "inlineStr":
                    value = "".join(
                        t.text or "" for t in cell.findall(".//a:t", NS)
                    )
                else:
                    value = raw
                cells[col] = cell_value(value)
            rows[row_num] = cells
        return rows


def parse_table_1_1(rows: dict[int, dict[str, Any]]) -> dict[int, dict[str, float]]:
    out: dict[int, dict[str, float]] = {}
    for row_num, cells in rows.items():
        year = cells.get("A")
        if not isinstance(year, int):
            continue
        receipts = cells.get("B")
        outlays = cells.get("C")
        surplus_or_deficit = cells.get("D")
        if not all(isinstance(v, (int, float)) for v in [receipts, outlays, surplus_or_deficit]):
            continue
        out[year] = {
            "row": row_num,
            "total_receipts": float(receipts),
            "total_outlays": float(outlays),
            "surplus_or_deficit": float(surplus_or_deficit),
        }
    return out


def parse_table_2_1(rows: dict[int, dict[str, Any]]) -> dict[int, dict[str, float]]:
    out: dict[int, dict[str, float]] = {}
    for row_num, cells in rows.items():
        year = cells.get("A")
        amount = cells.get("B")
        if isinstance(year, int) and isinstance(amount, (int, float)):
            out[year] = {"row": row_num, "individual_income_tax": float(amount)}
    return out


def parse_table_3_1(rows: dict[int, dict[str, Any]]) -> tuple[list[int], dict[str, dict[int, float]]]:
    header = rows[2]
    years_by_col: dict[str, int] = {}
    for col, value in header.items():
        if isinstance(value, int):
            years_by_col[col] = value

    categories: dict[str, dict[int, float]] = {}
    for key, label, row_num in BROAD_CATEGORIES + [
        ("total-federal-outlays", "Total, Federal outlays", 35)
    ]:
        cells = rows[row_num]
        if cells.get("A") != label:
            raise ValueError(f"Unexpected Table 3.1 row {row_num}: {cells.get('A')!r}")
        categories[key] = {}
        for col, year in years_by_col.items():
            value = cells.get(col)
            if isinstance(value, (int, float)):
                categories[key][year] = float(value)

    return sorted(years_by_col.values()), categories


def round_amount(value: float) -> float:
    return round(value, 6)


def build_records() -> tuple[list[dict[str, Any]], list[str], dict[str, Any]]:
    t11 = parse_table_1_1(read_sheet(TABLE_1_1))
    t21 = parse_table_2_1(read_sheet(TABLE_2_1))
    years_31, t31 = parse_table_3_1(read_sheet(TABLE_3_1))

    years = [year for year in years_31 if 1940 <= year <= 2025]
    records: list[dict[str, Any]] = []
    errors: list[str] = []
    annual_checks: list[dict[str, Any]] = []

    for year in years:
        if year not in t11:
            errors.append(f"{year}: missing Table 1.1 row")
            continue
        if year not in t21:
            errors.append(f"{year}: missing Table 2.1 row")
            continue
        if year not in t31["total-federal-outlays"]:
            errors.append(f"{year}: missing Table 3.1 total outlays")
            continue

        total_outlays_11 = t11[year]["total_outlays"]
        total_outlays_31 = t31["total-federal-outlays"][year]
        if not math.isclose(total_outlays_11, total_outlays_31, abs_tol=0.5):
            errors.append(
                f"{year}: Table 3.1 total {total_outlays_31} does not reconcile "
                f"to Table 1.1 total {total_outlays_11}"
            )

        category_total = sum(t31[key][year] for key, _, _ in BROAD_CATEGORIES)
        category_total_difference = category_total - total_outlays_31
        if not math.isclose(category_total, total_outlays_31, abs_tol=2.0):
            errors.append(
                f"{year}: category total {category_total} does not reconcile "
                f"to Table 3.1 total {total_outlays_31}"
            )

        income_tax = t21[year]["individual_income_tax"]
        total_receipts = t11[year]["total_receipts"]
        surplus_or_deficit = t11[year]["surplus_or_deficit"]
        deficit_gap = max(total_outlays_31 - total_receipts, 0)
        borrowed_share = deficit_gap / total_outlays_31 * 100
        income_tax_coverage = income_tax / total_outlays_31 * 100
        modeled_sum = 0.0

        for key, label, table_row in BROAD_CATEGORIES:
            category_outlays = t31[key][year]
            outlay_share = category_outlays / total_outlays_31 * 100
            allocation_share = category_outlays / category_total * 100
            modeled_amount = income_tax * category_outlays / category_total
            modeled_sum += modeled_amount
            records.append(
                {
                    "record_id": f"income-tax-outlay-model:{year}:{key}",
                    "record_family": "income_tax_outlay_model",
                    "model_id": MODEL_ID,
                    "fiscal_year": year,
                    "year_basis": "fiscal_year",
                    "source_ids": SOURCE_IDS,
                    "source_table_refs": {
                        "fiscal_spine": f"OMB Historical Table 1.1 FY2027 row {t11[year]['row']}",
                        "tax_receipts": f"OMB Historical Table 2.1 FY2027 row {t21[year]['row']}, column B",
                        "outlay_category": f"OMB Historical Table 3.1 FY2027 row {table_row}",
                        "outlay_total": "OMB Historical Table 3.1 FY2027 row 35",
                    },
                    "tax_source": "individual-income-taxes",
                    "allocation_method": "proportional_outlay_share",
                    "legal_allocation_status": "modeled_not_legal_dedication",
                    "category_key": key,
                    "category_label": label,
                    "category_outlays_amount": round_amount(category_outlays),
                    "total_outlays_amount": round_amount(total_outlays_31),
                    "category_total_outlays_amount": round_amount(category_total),
                    "individual_income_tax_receipts_amount": round_amount(income_tax),
                    "outlay_share_percent": round(outlay_share, 9),
                    "allocation_share_percent": round(allocation_share, 9),
                    "modeled_income_tax_allocation_amount": round_amount(modeled_amount),
                    "total_receipts_amount": round_amount(total_receipts),
                    "surplus_or_deficit_amount": round_amount(surplus_or_deficit),
                    "deficit_gap_amount": round_amount(deficit_gap),
                    "borrowed_share_percent_of_outlays": round(borrowed_share, 9),
                    "income_tax_coverage_percent_of_outlays": round(
                        income_tax_coverage, 9
                    ),
                    "category_total_reconciliation_difference_amount": round_amount(
                        category_total_difference
                    ),
                    "actual_or_projection": "actual",
                    "status": "draft",
                    "observed_date": OBSERVED_DATE,
                    "notes": (
                        "Modeled allocation of ordinary individual income-tax "
                        "receipts by broad Table 3.1 outlay share, normalized "
                        "over displayed broad-category rows to handle source "
                        "rounding; not legal dedication or program tracing."
                    ),
                }
            )

        if not math.isclose(modeled_sum, income_tax, abs_tol=0.0005):
            errors.append(
                f"{year}: modeled allocation sum {modeled_sum} does not "
                f"match individual income-tax receipts {income_tax}"
            )
        annual_checks.append(
            {
                "year": year,
                "table_1_1_outlays": total_outlays_11,
                "table_3_1_outlays": total_outlays_31,
                "category_total": category_total,
                "income_tax": income_tax,
                "modeled_sum": modeled_sum,
                "deficit_gap": deficit_gap,
            }
        )

    profile = {
        "year_count": len(years),
        "first_year": min(years),
        "last_year": max(years),
        "record_count": len(records),
        "annual_checks": annual_checks,
    }
    return records, errors, profile


def write_outputs(records: list[dict[str, Any]], profile: dict[str, Any]) -> None:
    OUTPUT_PATH.parent.mkdir(parents=True, exist_ok=True)
    with OUTPUT_PATH.open("w", encoding="utf-8", newline="\n") as f:
        for record in records:
            f.write(json.dumps(record, separators=(",", ":")) + "\n")

    checks = profile["annual_checks"]
    sample_years = [1940, 1950, 1960, 1970, 1980, 1990, 2000, 2010, 2020, 2025]
    sample = [row for row in checks if row["year"] in sample_years]
    lines = [
        "# Income-Tax Outlay Model Source Profile",
        "",
        "## Source Coverage",
        "",
        f"- Model ID: `{MODEL_ID}`",
        f"- Fiscal years emitted: {profile['first_year']}-{profile['last_year']}",
        f"- Year count: {profile['year_count']}",
        f"- Record count: {profile['record_count']}",
        "- Actual/projection treatment: actual years only; FY2026-FY2031 are excluded.",
        "",
        "## Source Roles",
        "",
        "| Source ID | Use |",
        "|---|---|",
        "| `SRC-OMB-HIST-1-1-FY2027` | Total receipts, total outlays, and surplus/deficit. |",
        "| `SRC-OMB-HIST-2-1-FY2027` | Individual income-tax receipts. |",
        "| `SRC-OMB-HIST-3-1-FY2027` | Broad outlay categories and total federal outlays. |",
        "",
        "## Broad Categories",
        "",
        "| Category key | OMB label | Table 3.1 row |",
        "|---|---|---:|",
    ]
    for key, label, row_num in BROAD_CATEGORIES:
        lines.append(f"| `{key}` | {label} | {row_num} |")
    lines.extend(
        [
            "",
            "## Reconciliation Sample",
            "",
            "All amounts are in millions of dollars. `Modeled sum` is the sum of",
            "the six category allocation rows for the fiscal year.",
            "",
        "| Fiscal year | Table 1.1 outlays | Table 3.1 outlays | Category total | Income tax receipts | Modeled sum | Deficit gap |",
            "|---:|---:|---:|---:|---:|---:|---:|",
        ]
    )
    for row in sample:
        lines.append(
            "| {year} | {table_1_1_outlays:,.0f} | {table_3_1_outlays:,.0f} | "
            "{category_total:,.0f} | {income_tax:,.0f} | {modeled_sum:,.3f} | "
            "{deficit_gap:,.0f} |".format(**row)
        )
    lines.extend(
        [
            "",
            "## Model Caveat",
            "",
            "These rows allocate individual income-tax receipts by reported outlay",
            "share, normalized over the displayed broad-category rows when source",
            "rounding creates a small difference from the displayed total. They do",
            "not claim that income-tax dollars were legally dedicated to the listed",
            "outlay categories.",
            "",
        ]
    )
    REPORT_PATH.write_text("\n".join(lines), encoding="utf-8", newline="\n")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--check",
        action="store_true",
        help="Build in memory and fail if validation errors are found.",
    )
    args = parser.parse_args()

    records, errors, profile = build_records()
    if errors:
        for error in errors:
            print(error)
        return 1
    if not args.check:
        write_outputs(records, profile)
    print(
        f"validated {profile['record_count']} rows for "
        f"{profile['first_year']}-{profile['last_year']}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
