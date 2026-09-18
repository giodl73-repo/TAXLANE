"""Join PennDOT rumble-strip inventory rows to public RMS attribute layers."""

from __future__ import annotations

import argparse
import hashlib
import json
from collections import Counter, defaultdict
from pathlib import Path

import openpyxl


KEY_FIELDS = ("District", "County #", "State Route #", "Segment #", "Direction")
SELECTED_CLASSES = {"3", "4", "5", "6"}
CLASS_NAMES = {
    "1": "interstate", "2": "other_freeway_expressway",
    "3": "other_principal_arterial", "4": "minor_arterial",
    "5": "major_collector", "6": "minor_collector", "7": "local",
}


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def envelope(path: Path) -> dict:
    value = json.loads(path.read_text(encoding="utf-8-sig"))
    if int(value["feature_count"]) != len(value["features"]):
        raise ValueError(f"capture count mismatch: {path}")
    if len(value["county_queries"]) != 67:
        raise ValueError(f"expected 67 county query records: {path}")
    return value


def cty(value: object) -> str:
    return f"{int(value):02d}"


def route_or_segment(value: object) -> str:
    return f"{int(value):04d}"


def interval(row: dict) -> tuple[tuple[int, int], tuple[int, int]]:
    first = (int(row["Segment #"]), int(row["Begin Offset (ft)"]))
    second = (int(row["Segment #"]), int(row["End Offset (ft)"]))
    return min(first, second), max(first, second)


def rms_interval(row: dict) -> tuple[tuple[int, int], tuple[int, int]]:
    first = (int(row["SEG_BGN"]), int(row["OFFSET_BGN"]))
    second = (int(row["SEG_END"]), int(row["OFFSET_END"]))
    return min(first, second), max(first, second)


def contains(container: dict, treatment: dict) -> bool:
    container_start, container_end = rms_interval(container)
    treatment_start, treatment_end = interval(treatment)
    return container_start <= treatment_start and treatment_end <= container_end


def overlaps(container: dict, treatment: dict) -> bool:
    container_start, container_end = rms_interval(container)
    treatment_start, treatment_end = interval(treatment)
    if treatment_start == treatment_end:
        return container_start <= treatment_start <= container_end
    return max(container_start, treatment_start) < min(container_end, treatment_end)


def union_miles(rows: list[dict]) -> tuple[int, float]:
    groups: dict[tuple, list[tuple[float, float]]] = defaultdict(list)
    for row in rows:
        begin, end = sorted((float(row["Begin Offset (ft)"]), float(row["End Offset (ft)"])))
        groups[tuple(row[field] for field in KEY_FIELDS)].append((begin, end))
    total = 0.0
    for intervals in groups.values():
        intervals.sort()
        current_begin, current_end = intervals[0]
        for begin, end in intervals[1:]:
            if begin <= current_end:
                current_end = max(current_end, end)
            else:
                total += current_end - current_begin
                current_begin, current_end = begin, end
        total += current_end - current_begin
    return len(groups), round(total / 5280, 6)


def pct(part: int | float, whole: int | float) -> float:
    return round(part / whole * 100, 6)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--workbook", type=Path, required=True)
    parser.add_argument("--segments", type=Path, required=True)
    parser.add_argument("--admin", type=Path, required=True)
    parser.add_argument("--shoulder", type=Path, required=True)
    parser.add_argument("--hpms-state-totals", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()

    sheet = openpyxl.load_workbook(args.workbook, read_only=True, data_only=True)["tblRumbleStrips"]
    values = sheet.iter_rows(values_only=True)
    headers = list(next(values))
    treatment_rows = [dict(zip(headers, row)) for row in values]
    candidate_rows = [
        row for row in treatment_rows
        if row["Direction"] == "B" and int(row["# of Lanes"]) == 2 and row["Type"] in {"CL", "SH", "EL"}
    ]

    segments_capture = envelope(args.segments)
    admin_capture = envelope(args.admin)
    shoulder_capture = envelope(args.shoulder)
    segments = segments_capture["features"]
    admin = admin_capture["features"]
    shoulders = shoulder_capture["features"]

    segment_index: dict[tuple[str, str, str, str], list[dict]] = defaultdict(list)
    for row in segments:
        segment_index[(row["CTY_CODE"], row["ST_RT_NO"], row["SEG_NO"], row["DIR_IND"])].append(row)
    if any(len(rows) != 1 for rows in segment_index.values()):
        raise ValueError("roadwaysegments exact keys are not unique")

    admin_index: dict[tuple[str, str], list[dict]] = defaultdict(list)
    for row in admin:
        if str(row["JURIS"]) == "1":
            admin_index[(row["CTY_CODE"], row["ST_RT_NO"])].append(row)
    shoulder_index: dict[tuple[str, str], list[dict]] = defaultdict(list)
    for row in shoulders:
        if str(row["JURIS"]) == "1":
            shoulder_index[(row["CTY_CODE"], row["ST_RT_NO"])].append(row)

    all_exact_segment_matches = 0
    candidate_exact_segment_matches = 0
    candidate_unique_admin_matches = 0
    candidate_both_matches = 0
    admin_class_counts: Counter[str] = Counter()
    comparable: list[tuple[dict, dict, dict]] = []
    for treatment in treatment_rows:
        key = (
            cty(treatment["County #"]), route_or_segment(treatment["State Route #"]),
            route_or_segment(treatment["Segment #"]), str(treatment["Direction"]),
        )
        if key in segment_index:
            all_exact_segment_matches += 1
    for treatment in candidate_rows:
        county_route = (cty(treatment["County #"]), route_or_segment(treatment["State Route #"]))
        segment_key = (*county_route, route_or_segment(treatment["Segment #"]), "B")
        segment_matches = segment_index.get(segment_key, [])
        admin_matches = [row for row in admin_index[county_route] if contains(row, treatment)]
        if len(segment_matches) == 1:
            candidate_exact_segment_matches += 1
        if len(admin_matches) == 1:
            candidate_unique_admin_matches += 1
            admin_class_counts[str(admin_matches[0]["FHWA_FUNC_CLS"])] += 1
        if len(segment_matches) == 1 and len(admin_matches) == 1:
            candidate_both_matches += 1
            segment = segment_matches[0]
            admin_row = admin_matches[0]
            if (
                str(segment["URBAN_RURAL"]) == "1"
                and str(segment["FAC_TYPE"]) == "2"
                and int(segment["LANE_CNT"]) == 2
                and str(admin_row["FHWA_FUNC_CLS"]) in SELECTED_CLASSES
                and str(segment["JURIS"]) == "1"
            ):
                comparable.append((treatment, segment, admin_row))

    comparable_rows = [value[0] for value in comparable]
    comparable_keys, comparable_miles = union_miles(comparable_rows)
    class_footprints = []
    for code in sorted(SELECTED_CLASSES):
        selected = [treatment for treatment, _segment, admin_row in comparable if str(admin_row["FHWA_FUNC_CLS"]) == code]
        keys, miles = union_miles(selected)
        class_footprints.append({
            "fhwa_functional_class": int(code),
            "functional_system": CLASS_NAMES[code],
            "treatment_rows": len(selected),
            "segment_direction_keys": keys,
            "union_interval_footprint_miles": miles,
        })

    iri_reported = [value for value in comparable if value[1]["ROUGH_INDX"] is not None]
    iri_good_fair = [value for value in iri_reported if float(value[1]["ROUGH_INDX"]) <= 170]
    iri_poor = [value for value in iri_reported if float(value[1]["ROUGH_INDX"]) > 170]
    both_shoulder_rows = 0
    any_shoulder_rows = 0
    for treatment, _segment, _admin in comparable:
        county_route = (cty(treatment["County #"]), route_or_segment(treatment["State Route #"]))
        hits = [row for row in shoulder_index[county_route] if overlaps(row, treatment)]
        sides = {str(row["SHLD_SIDE_IND"]) for row in hits}
        any_shoulder_rows += bool(hits)
        both_shoulder_rows += "L" in sides and "R" in sides

    hpms_rows = json.loads(args.hpms_state_totals.read_text(encoding="utf-8-sig"))
    hpms_pa = next(row for row in hpms_rows if int(row["stateid"]) == 42)
    source_paths = [args.workbook, args.segments, args.admin, args.shoulder, args.hpms_state_totals]
    output = {
        "record_id": "trn-penndot-rms-existing-treatment-join:2024:v1",
        "record_family": "trn_penndot_rms_existing_treatment_join",
        "version": "v1.draft",
        "status": "state_comparable_treated_footprint_ready_full_denominator_and_untreated_eligibility_blocked",
        "as_of_date": "2026-07-28",
        "source_custody": [
            {"path": str(path).replace("\\", "/"), "sha256": sha256(path)} for path in source_paths
        ],
        "capture_scope": {
            "county_route_pairs_from_workbook": sum(len(row["state_routes"]) for row in segments_capture["county_queries"]),
            "roadwaysegment_features": segments_capture["feature_count"],
            "roadwayadmin_features": admin_capture["feature_count"],
            "roadwayshoulder_features": shoulder_capture["feature_count"],
            "selection_boundary": "The capture includes only county-route pairs present in the treatment workbook. It cannot represent routes with no recorded treatment and is not a statewide denominator.",
        },
        "join_coverage": {
            "all_workbook_treatment_rows": len(treatment_rows),
            "all_rows_exact_segment_matches": all_exact_segment_matches,
            "all_rows_exact_segment_match_percent": pct(all_exact_segment_matches, len(treatment_rows)),
            "candidate_setting_treatment_rows": len(candidate_rows),
            "candidate_rows_exact_segment_matches": candidate_exact_segment_matches,
            "candidate_exact_segment_match_percent": pct(candidate_exact_segment_matches, len(candidate_rows)),
            "candidate_rows_unique_admin_interval_matches": candidate_unique_admin_matches,
            "candidate_unique_admin_match_percent": pct(candidate_unique_admin_matches, len(candidate_rows)),
            "candidate_rows_with_both_matches": candidate_both_matches,
            "candidate_both_match_percent": pct(candidate_both_matches, len(candidate_rows)),
            "admin_class_counts_for_unique_matches": [
                {"fhwa_functional_class": int(code), "functional_system": CLASS_NAMES.get(code, "unknown"), "treatment_rows": count}
                for code, count in sorted(admin_class_counts.items())
            ],
        },
        "matched_comparable_treated_footprint": {
            "criteria": "unique segment and admin matches; RMS jurisdiction=1, urban_rural=1, facility_type=2, lane_count=2; FHWA functional class in 3,4,5,6",
            "treatment_rows": len(comparable_rows),
            "share_of_candidate_setting_rows_percent": pct(len(comparable_rows), len(candidate_rows)),
            "segment_direction_keys": comparable_keys,
            "union_interval_footprint_miles": comparable_miles,
            "by_functional_class": class_footprints,
            "measurement_boundary": "This is a matched state-owned existing-treatment interval footprint, not an official route-mile total. Treatment types can overlap and the capture excludes county-route pairs with no treatment record.",
        },
        "matched_treated_condition_availability": {
            "iri_reported_treatment_rows": len(iri_reported),
            "iri_le_170_treatment_rows": len(iri_good_fair),
            "iri_gt_170_treatment_rows": len(iri_poor),
            "any_shoulder_interval_match_treatment_rows": any_shoulder_rows,
            "both_shoulder_sides_interval_match_treatment_rows": both_shoulder_rows,
            "boundary": "These counts describe already-treated records and field availability. IRI is not structural adequacy; shoulder records do not establish usable width beyond a proposed strip or suitability of untreated roads.",
        },
        "hpms_reconciliation": {
            "pennsylvania_rural_two_way_two_lane_filtered_section_miles": float(hpms_pa["miles"]),
            "matched_treated_footprint_subtracted": False,
            "reason": "HPMS covers the selected public-road systems, while the matched footprint is state-owned and comes from a treatment-selected county-route universe. A full state-owned RMS denominator and crosswalk to HPMS are required before subtraction.",
        },
        "next_reduction_contract": {
            "step": "capture the complete Pennsylvania state-owned RMS segment and admin universes, apply the same rural/two-way/two-lane/FHWA-class filter, and reconcile that denominator to HPMS before subtracting the matched treated footprint",
            "then": "join full shoulder and pavement intervals, existing/programmed treatments, and observed target crashes; report every nonmatch and coverage rate",
        },
        "blocked_values": {
            "pennsylvania_state_owned_comparable_denominator_miles": None,
            "pennsylvania_untreated_miles": None,
            "pennsylvania_untreated_design_eligible_miles": None,
            "target_crash_fatalities_and_injuries": None,
            "national_untreated_design_eligible_miles": None,
            "candidate_cost_billions": None,
            "candidate_outcome": None,
        },
        "claim_boundaries": {
            "public_rms_attributes_raw_custodied": True,
            "inventory_segment_join_audited": True,
            "comparable_state_owned_treated_footprint_ready": True,
            "full_state_denominator_ready": False,
            "hpms_subtraction_ready": False,
            "untreated_mileage_ready": False,
            "design_eligible_mileage_ready": False,
            "target_crash_numerator_ready": False,
            "national_extrapolation_allowed": False,
            "candidate_cost_ready": False,
            "allocation_ready": False,
            "funding_recommended": False,
            "public_release_authorized": False,
        },
    }
    args.output.write_text(json.dumps(output, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({
        "output": str(args.output), "comparable_treated_miles": comparable_miles,
        "full_state_denominator_ready": False,
    }))


if __name__ == "__main__":
    main()
