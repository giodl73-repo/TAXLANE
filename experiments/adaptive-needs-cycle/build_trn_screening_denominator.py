"""Build the TRN rumble-strip screening denominator from custodied FHWA tables."""

from __future__ import annotations

import argparse
import hashlib
import json
import re
from html import unescape
from pathlib import Path


RURAL_COLUMNS = [
    "interstate",
    "other_freeways_and_expressways",
    "other_principal_arterial",
    "minor_arterial",
    "major_collector",
    "minor_collector",
    "local",
    "total",
]
SCREENING_COLUMNS = [
    "other_principal_arterial",
    "minor_arterial",
    "major_collector",
    "minor_collector",
]


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def us_total_values(path: Path) -> list[int]:
    html = path.read_text(encoding="utf-8", errors="replace")
    match = re.search(r"<tr[^>]*>\s*<th[^>]*>\s*U\.S\. Total\s*</th>(.*?)</tr>", html, re.I | re.S)
    if not match:
        raise ValueError(f"U.S. Total row not found in {path}")
    cells = re.findall(r"<td[^>]*>(.*?)</td>", match.group(1), re.I | re.S)
    values = []
    for cell in cells:
        text = re.sub(r"<[^>]+>", "", unescape(cell)).strip().replace(",", "")
        values.append(0 if text in {"", "-"} else int(text))
    return values


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--hm20", type=Path, required=True)
    parser.add_argument("--fi20", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()

    hm = us_total_values(args.hm20)
    fi = us_total_values(args.fi20)
    if len(hm) != 17:
        raise ValueError(f"expected 17 HM-20 values, found {len(hm)}")
    if len(fi) != 18:
        raise ValueError(f"expected 18 FI-20 values, found {len(fi)}")

    rural_miles = dict(zip(RURAL_COLUMNS, hm[:8]))
    rural_fatalities = dict(zip(RURAL_COLUMNS, fi[:8]))
    screening_miles = sum(rural_miles[name] for name in SCREENING_COLUMNS)
    screening_fatalities = sum(rural_fatalities[name] for name in SCREENING_COLUMNS)
    historical_cost_per_mile = 3_800

    output = {
        "record_id": "trn-rumble-strip-screening-denominator:2024:v1",
        "record_family": "trn_rumble_strip_screening_denominator",
        "version": "v1.draft",
        "status": "broad_screening_frame_ready_segment_eligibility_blocked",
        "as_of_date": "2026-07-28",
        "source_custody": [
            {"source_id": "SRC-FHWA-HIGHWAY-STATISTICS-2024-HM20", "path": str(args.hm20).replace("\\", "/"), "sha256": sha256(args.hm20)},
            {"source_id": "SRC-FHWA-HIGHWAY-STATISTICS-2024-FI20", "path": str(args.fi20).replace("\\", "/"), "sha256": sha256(args.fi20)},
        ],
        "screening_definition": {
            "geography": "United States excluding Puerto Rico",
            "year": 2024,
            "included_rural_functional_systems": SCREENING_COLUMNS,
            "excluded_rural_functional_systems": ["interstate", "other_freeways_and_expressways", "local"],
            "reason": "A broad nonfreeway arterial-and-collector screen for a rural two-lane candidate; it is not a two-lane or design-eligible inventory.",
        },
        "rural_source_values": {"route_miles": rural_miles, "fatalities": rural_fatalities},
        "screening_results": {
            "route_miles": screening_miles,
            "fatalities_all_crash_types": screening_fatalities,
            "share_of_rural_route_miles_percent": round(screening_miles / rural_miles["total"] * 100, 6),
            "share_of_rural_fatalities_percent": round(screening_fatalities / rural_fatalities["total"] * 100, 6),
            "mechanical_historical_cost_product_billions": round(screening_miles * historical_cost_per_mile / 1_000_000_000, 6),
            "mechanical_product_status": "not_a_candidate_cost_or_upper_bound",
        },
        "eligibility_reduction_sequence": [
            "obtain HPMS or road-owner section records and retain rural two-way undivided sections with exactly two through lanes",
            "remove sections already treated or programmed for treatment",
            "remove or redesign sections failing pavement condition width bicycle motorcycle noise or environmental criteria",
            "join the remaining sections to target head-on opposite-direction sideswipe and single-vehicle run-off-road fatal-and-injury crash exposure",
            "apply current jurisdiction-specific bid maintenance accommodation administration and delivery-capacity costs",
        ],
        "blocked_values": {
            "two_lane_route_miles": None,
            "design_eligible_untreated_route_miles": None,
            "target_crash_fatalities_and_injuries": None,
            "current_weighted_cost_per_mile": None,
            "candidate_cost_billions": None,
            "candidate_outcome": None,
        },
        "claim_boundaries": {
            "screening_frame_ready": True,
            "eligible_denominator_ready": False,
            "affected_crash_numerator_ready": False,
            "candidate_cost_ready": False,
            "candidate_outcome_ready": False,
            "allocation_ready": False,
            "funding_recommended": False,
            "public_release_authorized": False,
        },
    }
    args.output.write_text(json.dumps(output, indent=2) + "\n")
    print(json.dumps({"output": str(args.output), "screening_miles": screening_miles, "screening_fatalities": screening_fatalities}))


if __name__ == "__main__":
    main()
