"""Build and calibrate the TRN HPMS rural two-way, two-lane section screen."""

from __future__ import annotations

import argparse
import hashlib
import json
from decimal import Decimal
from pathlib import Path


SYSTEMS = {
    3: "other_principal_arterial",
    4: "minor_arterial",
    5: "major_collector",
    6: "minor_collector",
}


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def keyed_rows(path: Path, key: str) -> dict[int, dict]:
    return {int(row[key]): row for row in json.loads(path.read_text(encoding="utf-8"))}


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--metadata", type=Path, required=True)
    parser.add_argument("--functional-totals", type=Path, required=True)
    parser.add_argument("--two-lane-functional-totals", type=Path, required=True)
    parser.add_argument("--two-lane-state-totals", type=Path, required=True)
    parser.add_argument("--official-screen", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()

    metadata = json.loads(args.metadata.read_text(encoding="utf-8"))
    columns = {column["fieldName"] for column in metadata["columns"]}
    required = {"urban_id", "stateid", "f_system", "facility_type", "through_lanes", "sectionlength"}
    if not required.issubset(columns):
        raise ValueError(f"missing HPMS columns: {sorted(required - columns)}")

    totals = keyed_rows(args.functional_totals, "f_system")
    filtered = keyed_rows(args.two_lane_functional_totals, "f_system")
    states = keyed_rows(args.two_lane_state_totals, "stateid")
    if set(totals) != set(SYSTEMS) or set(filtered) != set(SYSTEMS):
        raise ValueError("expected HPMS functional systems 3, 4, 5, and 6")
    if len(states) != 50:
        raise ValueError(f"expected 50 state rows, found {len(states)}")

    official = json.loads(args.official_screen.read_text(encoding="utf-8"))
    official_miles = official["rural_source_values"]["route_miles"]
    calibration = []
    raw_total = Decimal("0")
    raw_rows = 0
    spatial_total = Decimal("0")
    official_total = Decimal("0")
    calibrated_total = Decimal("0")
    for code, name in SYSTEMS.items():
        spatial = Decimal(totals[code]["miles"])
        raw = Decimal(filtered[code]["miles"])
        official_value = Decimal(str(official_miles[name]))
        share = raw / spatial
        calibrated = official_value * share
        spatial_total += spatial
        raw_total += raw
        official_total += official_value
        calibrated_total += calibrated
        raw_rows += int(filtered[code]["rows"])
        calibration.append({
            "f_system": code,
            "functional_system": name,
            "official_hm20_route_miles": int(official_value),
            "hpms_spatial_all_section_miles": round(float(spatial), 4),
            "hpms_two_way_two_lane_section_miles": round(float(raw), 4),
            "within_hpms_two_way_two_lane_share_percent": round(float(share * 100), 6),
            "calibrated_screen_miles": round(float(calibrated), 4),
        })

    historical_cost_per_mile = Decimal("3800")
    output = {
        "record_id": "trn-hpms-two-way-two-lane-screen:2024:v1",
        "record_family": "trn_hpms_two_way_two_lane_screen",
        "version": "v1.draft",
        "status": "section_filter_and_calibration_ready_design_eligibility_blocked",
        "as_of_date": "2026-07-28",
        "dataset": {"id": metadata["id"], "name": metadata["name"], "issued": "2025-11-10"},
        "query_contract": {
            "rural_rule": "urban_id=99999",
            "geography_rule": "stateid != 72; 50 states represented; District of Columbia has no rural row",
            "functional_system_rule": "f_system in (3,4,5,6)",
            "two_way_rule": "facility_type=2",
            "two_lane_rule": "through_lanes=2; for two-way roads HPMS reports both directions",
        },
        "source_custody": [
            {"path": str(path).replace("\\", "/"), "sha256": sha256(path)}
            for path in [args.metadata, args.functional_totals, args.two_lane_functional_totals, args.two_lane_state_totals, args.official_screen]
        ],
        "raw_hpms_filter": {
            "section_rows": raw_rows,
            "section_miles": round(float(raw_total), 4),
            "states_represented": len(states),
        },
        "official_table_reconciliation": {
            "hpms_spatial_all_section_miles_in_four_systems": round(float(spatial_total), 4),
            "official_hm20_route_miles_in_four_systems": int(official_total),
            "spatial_to_official_ratio": round(float(spatial_total / official_total), 6),
            "status": "not_directly_reconciled",
            "reason": "FHWA warns that spatial full-join aggregation may differ from official Highway Statistics enterprise totals.",
        },
        "functional_system_calibration": calibration,
        "calibrated_screen": {
            "method": "apply each HPMS within-system two-way two-lane share to the corresponding official HM-20 route miles, then sum",
            "miles": round(float(calibrated_total), 4),
            "status": "analytical_screen_not_official_mileage_or_design_eligible_inventory",
        },
        "historical_cost_products": {
            "cost_per_mile": int(historical_cost_per_mile),
            "raw_hpms_product_billions": round(float(raw_total * historical_cost_per_mile / Decimal(1_000_000_000)), 6),
            "calibrated_product_billions": round(float(calibrated_total * historical_cost_per_mile / Decimal(1_000_000_000)), 6),
            "status": "neither_value_is_a_candidate_cost_or_upper_bound",
        },
        "remaining_filters": [
            "existing or already-programmed rumble strips",
            "pavement condition and remaining service life",
            "shoulder and lane width plus bicycle and motorcycle accommodation",
            "residential noise and environmental constraints",
            "target head-on opposite-direction sideswipe and single-vehicle run-off-road crash exposure",
            "current road-owner installation maintenance administration and delivery costs",
        ],
        "blocked_values": {
            "design_eligible_untreated_miles": None,
            "target_crash_fatalities_and_injuries": None,
            "current_weighted_cost_per_mile": None,
            "candidate_cost_billions": None,
            "candidate_outcome": None,
        },
        "claim_boundaries": {
            "section_filter_ready": True,
            "official_mileage_reconciled": False,
            "design_eligible_denominator_ready": False,
            "affected_crash_numerator_ready": False,
            "candidate_cost_ready": False,
            "allocation_ready": False,
            "funding_recommended": False,
            "public_release_authorized": False,
        },
    }
    args.output.write_text(json.dumps(output, indent=2) + "\n")
    print(json.dumps({"output": str(args.output), "raw_miles": float(raw_total), "calibrated_miles": round(float(calibrated_total), 4)}))


if __name__ == "__main__":
    main()
