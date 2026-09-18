"""Build Pennsylvania's same-basis state-owned untreated rumble-strip screen."""

from __future__ import annotations

import argparse
import hashlib
import json
from collections import defaultdict
from pathlib import Path

import openpyxl


CLASSES = {
    "3": "other_principal_arterial",
    "4": "minor_arterial",
    "5": "major_collector",
    "6": "minor_collector",
}


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def captured(path: Path) -> dict:
    value = json.loads(path.read_text(encoding="utf-8-sig"))
    if int(value["feature_count"]) != len(value["features"]):
        raise ValueError(f"capture count mismatch: {path}")
    return value


def merge(intervals: list[tuple[float, float]]) -> list[tuple[float, float]]:
    if not intervals:
        return []
    result: list[tuple[float, float]] = []
    for begin, end in sorted(intervals):
        if end <= begin:
            continue
        if result and begin <= result[-1][1]:
            result[-1] = (result[-1][0], max(result[-1][1], end))
        else:
            result.append((begin, end))
    return result


def feet(intervals: list[tuple[float, float]]) -> float:
    return sum(end - begin for begin, end in merge(intervals))


def subtract(base: list[tuple[float, float]], removal: list[tuple[float, float]]) -> list[tuple[float, float]]:
    remaining = merge(base)
    for remove_begin, remove_end in merge(removal):
        next_remaining = []
        for begin, end in remaining:
            if remove_end <= begin or remove_begin >= end:
                next_remaining.append((begin, end))
                continue
            if remove_begin > begin:
                next_remaining.append((begin, min(remove_begin, end)))
            if remove_end < end:
                next_remaining.append((max(remove_end, begin), end))
        remaining = next_remaining
    return remaining


def shoulder_availability(
    base: list[tuple[float, float]],
    side_rows: dict[str, list[tuple[float, float, float | None]]],
) -> dict[str, float]:
    totals = {
        "both_current_widths_ge_4_feet": 0.0,
        "both_current_widths_reported_one_or_both_lt_4_feet": 0.0,
        "missing_either_side_record": 0.0,
        "ambiguous_or_missing_current_width": 0.0,
    }
    for base_begin, base_end in merge(base):
        points = {base_begin, base_end}
        for rows in side_rows.values():
            for begin, end, _ in rows:
                if end > base_begin and begin < base_end:
                    points.add(max(begin, base_begin))
                    points.add(min(end, base_end))
        ordered = sorted(points)
        for begin, end in zip(ordered, ordered[1:]):
            if end <= begin:
                continue
            midpoint = (begin + end) / 2
            hits = {
                side: [width for row_begin, row_end, width in side_rows.get(side, []) if row_begin <= midpoint < row_end]
                for side in ("L", "R")
            }
            length = end - begin
            if not hits["L"] or not hits["R"]:
                totals["missing_either_side_record"] += length
                continue
            widths = {side: set(hits[side]) for side in ("L", "R")}
            if any(None in widths[side] or len(widths[side]) != 1 for side in ("L", "R")):
                totals["ambiguous_or_missing_current_width"] += length
                continue
            left = next(iter(widths["L"]))
            right = next(iter(widths["R"]))
            band = (
                "both_current_widths_ge_4_feet"
                if left >= 4 and right >= 4
                else "both_current_widths_reported_one_or_both_lt_4_feet"
            )
            totals[band] += length
    return totals


def rms_bounds(row: dict) -> tuple[tuple[int, int], tuple[int, int]]:
    first = (int(row["SEG_BGN"]), int(row["OFFSET_BGN"]))
    second = (int(row["SEG_END"]), int(row["OFFSET_END"]))
    return min(first, second), max(first, second)


def segment_admin_interval(segment: dict, admin: dict) -> tuple[float, float] | None:
    segment_number = int(segment["SEG_NO"])
    segment_length = float(segment["SEG_LNGTH_FEET"])
    lower, upper = rms_bounds(admin)
    if segment_number < lower[0] or segment_number > upper[0]:
        return None
    begin = float(lower[1]) if segment_number == lower[0] else 0.0
    end = float(upper[1]) if segment_number == upper[0] else segment_length
    begin = max(0.0, min(begin, segment_length))
    end = max(0.0, min(end, segment_length))
    return (begin, end) if end > begin else None


def miles(value: float) -> float:
    return round(value / 5280, 6)


def year_band(value: object, current_year: int = 2026) -> str:
    try:
        year = int(str(value)[:4])
    except (TypeError, ValueError):
        return "missing_or_invalid"
    if year <= 0:
        return "missing_or_invalid"
    if year > current_year:
        return "future_after_2026"
    if year >= 2021:
        return "2021_to_2026"
    if year >= 2016:
        return "2016_to_2020"
    return "through_2015"


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--workbook", type=Path, required=True)
    parser.add_argument("--segments", type=Path, required=True)
    parser.add_argument("--admin", type=Path, required=True)
    parser.add_argument("--shoulder", type=Path, required=True)
    parser.add_argument("--prior-join", type=Path, required=True)
    parser.add_argument("--hpms-state-totals", type=Path, required=True)
    parser.add_argument("--hpms-screen", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()

    segment_capture = captured(args.segments)
    admin_capture = captured(args.admin)
    shoulder_capture = captured(args.shoulder)
    segments = segment_capture["features"]
    admin_rows = admin_capture["features"]
    shoulder_rows = shoulder_capture["features"]
    admin_index: dict[tuple[str, str], list[dict]] = defaultdict(list)
    for row in admin_rows:
        admin_index[(row["CTY_CODE"], row["ST_RT_NO"])].append(row)
    shoulder_index: dict[tuple[str, str], list[dict]] = defaultdict(list)
    for row in shoulder_rows:
        shoulder_index[(row["CTY_CODE"], row["ST_RT_NO"])].append(row)

    segment_index: dict[tuple[str, str, str, str], dict] = {}
    denominator: dict[tuple[tuple[str, str, str, str], str], list[tuple[float, float]]] = defaultdict(list)
    segments_with_selected_interval = 0
    fully_covered_segments = 0
    partially_covered_segments = 0
    cross_class_overlap_segments = 0
    for segment in segments:
        key = (segment["CTY_CODE"], segment["ST_RT_NO"], segment["SEG_NO"], "B")
        if key in segment_index:
            raise ValueError(f"duplicate segment key: {key}")
        segment_index[key] = segment
        by_class: dict[str, list[tuple[float, float]]] = defaultdict(list)
        for admin in admin_index[(segment["CTY_CODE"], segment["ST_RT_NO"])]:
            code = str(admin["FHWA_FUNC_CLS"])
            if code not in CLASSES:
                continue
            value = segment_admin_interval(segment, admin)
            if value is not None:
                by_class[code].append(value)
        for code, values in by_class.items():
            denominator[(key, code)].extend(values)
        class_feet = sum(feet(values) for values in by_class.values())
        all_feet = feet([item for values in by_class.values() for item in values])
        if class_feet > all_feet + 0.01:
            cross_class_overlap_segments += 1
        segment_length = float(segment["SEG_LNGTH_FEET"])
        if all_feet > 0:
            segments_with_selected_interval += 1
            if abs(all_feet - segment_length) <= 0.01:
                fully_covered_segments += 1
            else:
                partially_covered_segments += 1
    if cross_class_overlap_segments:
        raise ValueError(f"cross-class interval overlap in {cross_class_overlap_segments} segments")
    denominator = {key: merge(values) for key, values in denominator.items()}

    sheet = openpyxl.load_workbook(args.workbook, read_only=True, data_only=True)["tblRumbleStrips"]
    values = sheet.iter_rows(values_only=True)
    headers = list(next(values))
    treatment_rows = [dict(zip(headers, row)) for row in values]
    candidate_rows = [
        row for row in treatment_rows
        if row["Direction"] == "B" and int(row["# of Lanes"]) == 2 and row["Type"] in {"CL", "SH", "EL"}
    ]
    treated: dict[tuple[tuple[str, str, str, str], str], list[tuple[float, float]]] = defaultdict(list)
    candidate_rows_intersecting_denominator = 0
    for treatment in candidate_rows:
        key = (
            f"{int(treatment['County #']):02d}", f"{int(treatment['State Route #']):04d}",
            f"{int(treatment['Segment #']):04d}", "B",
        )
        begin, end = sorted((float(treatment["Begin Offset (ft)"]), float(treatment["End Offset (ft)"])))
        row_intersects = False
        for code in CLASSES:
            for denominator_begin, denominator_end in denominator.get((key, code), []):
                overlap_begin = max(begin, denominator_begin)
                overlap_end = min(end, denominator_end)
                if overlap_end > overlap_begin:
                    treated[(key, code)].append((overlap_begin, overlap_end))
                    row_intersects = True
                elif begin == end and denominator_begin <= begin <= denominator_end:
                    row_intersects = True
        candidate_rows_intersecting_denominator += row_intersects
    treated = {key: merge(values) for key, values in treated.items()}

    totals = {code: {"denominator_feet": 0.0, "treated_feet": 0.0, "untreated_feet": 0.0} for code in CLASSES}
    untreated_by_iri = {"iri_le_170": 0.0, "iri_gt_170": 0.0, "missing": 0.0}
    untreated_by_surface_year = {"2021_to_2026": 0.0, "2016_to_2020": 0.0, "through_2015": 0.0, "future_after_2026": 0.0, "missing_or_invalid": 0.0}
    untreated_by_condition_date = {"2021_to_2026": 0.0, "2016_to_2020": 0.0, "through_2015": 0.0, "future_after_2026": 0.0, "missing_or_invalid": 0.0}
    untreated_by_opi_rating: dict[str, float] = defaultdict(float)
    pavement_field_coverage = {
        "surface_type_reported": 0.0,
        "surface_year_reported": 0.0,
        "legacy_year_resurfaced_reported": 0.0,
        "condition_date_reported": 0.0,
        "overall_pavement_index_reported": 0.0,
        "pavement_condition_rating_reported": 0.0,
    }
    untreated_by_shoulder = {
        "both_current_widths_ge_4_feet": 0.0,
        "both_current_widths_reported_one_or_both_lt_4_feet": 0.0,
        "missing_either_side_record": 0.0,
        "ambiguous_or_missing_current_width": 0.0,
    }
    untreated_shoulder_by_opi: dict[str, dict[str, float]] = defaultdict(lambda: defaultdict(float))
    untreated_shoulder_by_surface_year: dict[str, dict[str, float]] = defaultdict(lambda: defaultdict(float))
    untreated_segment_keys: set[tuple[str, str, str, str]] = set()
    treated_segment_keys: set[tuple[str, str, str, str]] = set()
    for (key, code), base_intervals in denominator.items():
        treatment_intervals = treated.get((key, code), [])
        remaining = subtract(base_intervals, treatment_intervals)
        denominator_feet = feet(base_intervals)
        treated_feet = feet(treatment_intervals)
        untreated_feet = feet(remaining)
        totals[code]["denominator_feet"] += denominator_feet
        totals[code]["treated_feet"] += treated_feet
        totals[code]["untreated_feet"] += untreated_feet
        if treated_feet:
            treated_segment_keys.add(key)
        if untreated_feet:
            untreated_segment_keys.add(key)
            iri = segment_index[key]["ROUGH_INDX"]
            band = "missing" if iri is None else ("iri_le_170" if float(iri) <= 170 else "iri_gt_170")
            untreated_by_iri[band] += untreated_feet
            segment = segment_index[key]
            untreated_by_surface_year[year_band(segment["SURFACE_YEAR"])] += untreated_feet
            untreated_by_condition_date[year_band(segment["COND_DATE"])] += untreated_feet
            opi_rating = str(segment["OPI_RATING_TEXT"] or "MISSING").upper()
            untreated_by_opi_rating[opi_rating] += untreated_feet
            for output_field, source_field in {
                "surface_type_reported": "SURF_TYPE",
                "surface_year_reported": "SURFACE_YEAR",
                "legacy_year_resurfaced_reported": "YR_RESURF",
                "condition_date_reported": "COND_DATE",
                "overall_pavement_index_reported": "OVERALL_PVMNT_IDX",
                "pavement_condition_rating_reported": "PVMNT_COND_RATE",
            }.items():
                if segment[source_field] not in {None, ""}:
                    pavement_field_coverage[output_field] += untreated_feet
            side_rows: dict[str, list[tuple[float, float, float | None]]] = defaultdict(list)
            for shoulder in shoulder_index[(segment["CTY_CODE"], segment["ST_RT_NO"])]:
                side = shoulder["SHLD_SIDE_IND"]
                if side not in {"L", "R"}:
                    continue
                interval = segment_admin_interval(segment, shoulder)
                if interval is None:
                    continue
                width_value = shoulder["CURRENT_PAVE_WIDTH"]
                width = None if width_value is None else float(width_value)
                side_rows[side].append((*interval, width))
            shoulder_totals = shoulder_availability(remaining, side_rows)
            for shoulder_band, value in shoulder_totals.items():
                untreated_by_shoulder[shoulder_band] += value
                untreated_shoulder_by_opi[shoulder_band][opi_rating] += value
                untreated_shoulder_by_surface_year[shoulder_band][year_band(segment["SURFACE_YEAR"])] += value

    denominator_feet = sum(value["denominator_feet"] for value in totals.values())
    treated_feet = sum(value["treated_feet"] for value in totals.values())
    untreated_feet = sum(value["untreated_feet"] for value in totals.values())
    if abs(denominator_feet - treated_feet - untreated_feet) > 0.01:
        raise ValueError("state denominator identity does not reconcile")
    if abs(sum(untreated_by_shoulder.values()) - untreated_feet) > 0.01:
        raise ValueError("untreated shoulder availability bands do not reconcile")

    hpms_rows = json.loads(args.hpms_state_totals.read_text(encoding="utf-8-sig"))
    hpms_pa_miles = float(next(row for row in hpms_rows if int(row["stateid"]) == 42)["miles"])
    prior_join = json.loads(args.prior_join.read_text(encoding="utf-8-sig"))
    prior_treated_miles = float(prior_join["matched_comparable_treated_footprint"]["union_interval_footprint_miles"])
    hpms_screen = json.loads(args.hpms_screen.read_text(encoding="utf-8-sig"))
    historical_cost_per_mile = float(hpms_screen["historical_cost_products"]["cost_per_mile"])
    untreated_miles = miles(untreated_feet)
    shoulder_miles = {key: miles(value) for key, value in untreated_by_shoulder.items()}
    surface_year_miles = {key: miles(value) for key, value in untreated_by_surface_year.items()}
    condition_date_miles = {key: miles(value) for key, value in untreated_by_condition_date.items()}

    class_rows = []
    for code, name in CLASSES.items():
        value = totals[code]
        class_rows.append({
            "fhwa_functional_class": int(code),
            "functional_system": name,
            "state_owned_screen_miles": miles(value["denominator_feet"]),
            "matched_existing_treated_miles": miles(value["treated_feet"]),
            "untreated_screening_miles": miles(value["untreated_feet"]),
        })

    source_paths = [args.workbook, args.segments, args.admin, args.shoulder, args.prior_join, args.hpms_state_totals, args.hpms_screen]
    output = {
        "record_id": "trn-penndot-state-owned-untreated-screen:2026-capture:v1",
        "record_family": "trn_penndot_state_owned_untreated_screen",
        "version": "v1.draft",
        "status": "same_basis_state_untreated_and_shoulder_availability_screen_ready_design_eligibility_and_target_crashes_blocked",
        "as_of_date": "2026-07-28",
        "source_custody": [
            {"path": str(path).replace("\\", "/"), "sha256": sha256(path)} for path in source_paths
        ],
        "filter_contract": {
            "roadwaysegments": segment_capture["where"],
            "roadwayadmin": admin_capture["where"],
            "roadwayshoulder": shoulder_capture["where"],
            "coordinate_basis": "PennDOT county, state route, segment, direction, and within-segment offsets",
            "scope": "PennDOT state-jurisdiction rural two-way two-lane roadway intervals in FHWA functional classes 3 through 6",
        },
        "state_owned_screening_denominator": {
            "captured_segment_rows": segment_capture["feature_count"],
            "captured_admin_rows": admin_capture["feature_count"],
            "captured_shoulder_rows": shoulder_capture["feature_count"],
            "segments_with_selected_class_interval": segments_with_selected_interval,
            "fully_covered_segments": fully_covered_segments,
            "partially_covered_segments": partially_covered_segments,
            "cross_class_overlap_segments": cross_class_overlap_segments,
            "miles": miles(denominator_feet),
        },
        "existing_treatment_subtraction": {
            "candidate_workbook_rows": len(candidate_rows),
            "candidate_rows_intersecting_same_basis_denominator": candidate_rows_intersecting_denominator,
            "treated_segment_direction_keys": len(treated_segment_keys),
            "matched_existing_treated_miles": miles(treated_feet),
            "prior_full_containment_join_miles": prior_treated_miles,
            "method_change": "The full-denominator method intersects and splits treatment intervals at administration-class boundaries; the prior join retained only rows wholly contained by one administration interval.",
        },
        "untreated_screen": {
            "untreated_segment_direction_keys": len(untreated_segment_keys),
            "miles": untreated_miles,
            "status": "same_basis_state_owned_untreated_screen_not_design_eligible_mileage",
        },
        "by_functional_class": class_rows,
        "untreated_iri_availability": {
            "iri_le_170_miles": miles(untreated_by_iri["iri_le_170"]),
            "iri_gt_170_miles": miles(untreated_by_iri["iri_gt_170"]),
            "missing_iri_miles": miles(untreated_by_iri["missing"]),
            "boundary": "IRI is a segment-level ride-quality proxy, not pavement structural adequacy or design eligibility.",
        },
        "untreated_shoulder_availability": {
            "both_current_paved_widths_ge_4_feet_miles": shoulder_miles["both_current_widths_ge_4_feet"],
            "both_current_paved_widths_ge_4_feet_share_percent": round(untreated_by_shoulder["both_current_widths_ge_4_feet"] / untreated_feet * 100, 6),
            "both_current_paved_widths_reported_one_or_both_lt_4_feet_miles": shoulder_miles["both_current_widths_reported_one_or_both_lt_4_feet"],
            "both_current_paved_widths_reported_one_or_both_lt_4_feet_share_percent": round(untreated_by_shoulder["both_current_widths_reported_one_or_both_lt_4_feet"] / untreated_feet * 100, 6),
            "missing_either_side_record_miles": shoulder_miles["missing_either_side_record"],
            "ambiguous_or_missing_current_paved_width_miles": shoulder_miles["ambiguous_or_missing_current_width"],
            "display_rounding_difference_miles": round(untreated_miles - sum(shoulder_miles.values()), 6),
            "boundary": "PennDOT CURRENT_PAVE_WIDTH records are interval-level availability inputs. Four reported paved feet are not verified usable clear width beyond a proposed strip; narrower, missing, or conflicting values are not automatic ineligibility findings.",
        },
        "untreated_pavement_availability": {
            "surface_year_vintage_miles": surface_year_miles,
            "surface_year_display_rounding_difference_miles": round(untreated_miles - sum(surface_year_miles.values()), 6),
            "condition_date_vintage_miles": condition_date_miles,
            "condition_date_display_rounding_difference_miles": round(untreated_miles - sum(condition_date_miles.values()), 6),
            "reported_opi_rating_miles": {key.lower(): miles(value) for key, value in sorted(untreated_by_opi_rating.items())},
            "reported_opi_display_rounding_difference_miles": round(
                untreated_miles - sum(miles(value) for value in untreated_by_opi_rating.values()), 6
            ),
            "field_reported_miles": {key: miles(value) for key, value in pavement_field_coverage.items()},
            "field_missing_miles": {key.removesuffix("_reported"): miles(untreated_feet - value) for key, value in pavement_field_coverage.items()},
            "shoulder_band_by_opi_rating_miles": {
                band: {rating.lower(): miles(value) for rating, value in sorted(ratings.items())}
                for band, ratings in sorted(untreated_shoulder_by_opi.items())
            },
            "shoulder_band_by_surface_year_vintage_miles": {
                band: {vintage: miles(value) for vintage, value in sorted(vintages.items())}
                for band, vintages in sorted(untreated_shoulder_by_surface_year.items())
            },
            "cross_field_review_queue": {
                "both_paved_widths_ge_4_and_opi_good_or_excellent_miles": miles(
                    untreated_shoulder_by_opi["both_current_widths_ge_4_feet"]["GOOD"]
                    + untreated_shoulder_by_opi["both_current_widths_ge_4_feet"]["EXCELLENT"]
                ),
                "both_paved_widths_ge_4_and_surface_year_2021_to_2026_miles": miles(
                    untreated_shoulder_by_surface_year["both_current_widths_ge_4_feet"]["2021_to_2026"]
                ),
                "status": "field_intersection_for_engineering_record_priority_not_design_eligibility",
            },
            "boundary": "SURFACE_YEAR, COND_DATE, SURF_TYPE, OVERALL_PVMNT_IDX, OPI_RATING_TEXT, and legacy YR_RESURF are segment-level inventory or condition fields. They do not reveal pavement thickness, cracking, base structure, remaining structural life, resurfacing commitments, or engineering approval. PVMNT_COND_RATE is retained as missing rather than inferred from other fields.",
        },
        "hpms_reconciliation": {
            "pennsylvania_hpms_all_public_selected_system_section_miles": hpms_pa_miles,
            "pennsylvania_rms_state_owned_same_basis_screen_miles": miles(denominator_feet),
            "rms_to_hpms_ratio": round(miles(denominator_feet) / hpms_pa_miles, 6),
            "difference_miles": round(hpms_pa_miles - miles(denominator_feet), 6),
            "status": "different_ownership_and_dataset_universes_not_forced_to_equal",
        },
        "historical_cost_product": {
            "untreated_screening_miles": untreated_miles,
            "historical_cost_per_mile": historical_cost_per_mile,
            "mechanical_product_billions": round(untreated_miles * historical_cost_per_mile / 1_000_000_000, 6),
            "status": "not_candidate_cost_or_upper_bound",
            "boundary": "The 2015 PennDOT-derived unit input excludes current prices, pavement work, accommodations, maintenance, administration, and delivery. The untreated screen is not design-eligible mileage.",
        },
        "remaining_reductions": [
            "current pavement structure, cracking, resurfacing schedule, and remaining life",
            "side-specific usable shoulder geometry and bicycle accommodation",
            "noise, residential, environmental, intersection, and driveway constraints",
            "already-programmed projects and treatment inventory vintage changes",
            "target head-on, opposite-direction sideswipe, and run-off-road crash exposure",
            "current road-owner unit, maintenance, administration, and delivery costs",
        ],
        "blocked_values": {
            "untreated_design_eligible_miles": None,
            "target_crash_fatalities_and_injuries": None,
            "current_weighted_cost_per_mile": None,
            "candidate_cost_billions": None,
            "candidate_outcome": None,
        },
        "claim_boundaries": {
            "full_state_owned_screening_denominator_ready": True,
            "same_basis_existing_treatment_subtraction_ready": True,
            "untreated_screening_mileage_ready": True,
            "side_specific_current_paved_width_availability_ready": True,
            "segment_level_pavement_field_availability_ready": True,
            "pavement_structural_adequacy_ready": False,
            "programmed_resurfacing_schedule_ready": False,
            "hpms_official_mileage_reconciled": False,
            "untreated_design_eligible_mileage_ready": False,
            "target_crash_numerator_ready": False,
            "current_cost_ready": False,
            "candidate_cost_ready": False,
            "national_extrapolation_allowed": False,
            "allocation_ready": False,
            "funding_recommended": False,
            "public_release_authorized": False,
        },
    }
    args.output.write_text(json.dumps(output, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({
        "output": str(args.output), "denominator_miles": miles(denominator_feet),
        "treated_miles": miles(treated_feet), "untreated_screening_miles": untreated_miles,
    }))


if __name__ == "__main__":
    main()
