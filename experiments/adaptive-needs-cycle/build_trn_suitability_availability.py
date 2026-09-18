"""Audit HPMS field availability for the TRN rumble-strip suitability screen."""

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


def rows(path: Path) -> list[dict]:
    value = json.loads(path.read_text(encoding="utf-8-sig"))
    if not isinstance(value, list):
        raise ValueError(f"expected a JSON row array: {path}")
    return value


def decimal(row: dict, key: str) -> Decimal:
    return Decimal(str(row[key]))


def count(row: dict, key: str) -> int:
    return int(row[key])


def percent(part: Decimal, whole: Decimal) -> float:
    return round(float(part / whole * 100), 6)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--metadata", type=Path, required=True)
    parser.add_argument("--hpms-screen", type=Path, required=True)
    parser.add_argument("--field-completeness", type=Path, required=True)
    parser.add_argument("--shoulder-bands", type=Path, required=True)
    parser.add_argument("--iri-bands", type=Path, required=True)
    parser.add_argument("--by-functional-system", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()

    metadata = json.loads(args.metadata.read_text(encoding="utf-8-sig"))
    field_names = {column["fieldName"] for column in metadata["columns"]}
    required_fields = {
        "sectionlength", "shoulder_width_l", "shoulder_width_r", "iri",
        "psr", "cracking_percent", "surface_type", "lane_width",
    }
    if not required_fields.issubset(field_names):
        raise ValueError(f"missing HPMS fields: {sorted(required_fields - field_names)}")
    rumble_fields = sorted(name for name in field_names if "rumble" in name.lower())

    hpms_screen = json.loads(args.hpms_screen.read_text(encoding="utf-8-sig"))
    completeness_rows = rows(args.field_completeness)
    if len(completeness_rows) != 1:
        raise ValueError("field completeness input must have exactly one row")
    completeness = completeness_rows[0]
    total_rows = count(completeness, "rows")
    total_miles = decimal(completeness, "miles")
    if total_rows != hpms_screen["raw_hpms_filter"]["section_rows"]:
        raise ValueError("field-completeness row total does not match the HPMS screen")
    if total_miles != Decimal(str(hpms_screen["raw_hpms_filter"]["section_miles"])):
        raise ValueError("field-completeness mileage does not match the HPMS screen")

    shoulder_source = {row["shoulder_band"]: row for row in rows(args.shoulder_bands)}
    iri_source = {row["iri_band"]: row for row in rows(args.iri_bands)}
    expected_shoulders = {"both_ge_4", "one_or_both_lt_4", "missing"}
    expected_iri = {"iri_le_170", "iri_gt_170", "missing"}
    if set(shoulder_source) != expected_shoulders or set(iri_source) != expected_iri:
        raise ValueError("unexpected shoulder or IRI band labels")
    if sum(count(row, "rows") for row in shoulder_source.values()) != total_rows:
        raise ValueError("shoulder row bands do not reconcile")
    if sum(decimal(row, "miles") for row in shoulder_source.values()) != total_miles:
        raise ValueError("shoulder mileage bands do not reconcile")
    if sum(count(row, "rows") for row in iri_source.values()) != total_rows:
        raise ValueError("IRI row bands do not reconcile")
    if sum(decimal(row, "miles") for row in iri_source.values()) != total_miles:
        raise ValueError("IRI mileage bands do not reconcile")

    functional_rows = rows(args.by_functional_system)
    if {int(row["f_system"]) for row in functional_rows} != set(SYSTEMS):
        raise ValueError("expected functional systems 3, 4, 5, and 6")
    if sum(count(row, "rows") for row in functional_rows) != total_rows:
        raise ValueError("functional-system row totals do not reconcile")
    if sum(decimal(row, "miles") for row in functional_rows) != total_miles:
        raise ValueError("functional-system mileage totals do not reconcile")

    def band(row: dict) -> dict:
        miles = decimal(row, "miles")
        return {
            "section_rows": count(row, "rows"),
            "section_miles": round(float(miles), 4),
            "share_of_filtered_section_miles_percent": percent(miles, total_miles),
        }

    def field_coverage(key: str) -> dict:
        reported = count(completeness, key)
        return {
            "reported_section_rows": reported,
            "row_coverage_percent": round(reported / total_rows * 100, 6),
        }

    source_paths = [
        args.metadata, args.hpms_screen, args.field_completeness,
        args.shoulder_bands, args.iri_bands, args.by_functional_system,
    ]
    output = {
        "record_id": "trn-hpms-rumble-suitability-availability:2024:v1",
        "record_family": "trn_hpms_rumble_suitability_availability",
        "version": "v1.draft",
        "status": "suitability_data_availability_audited_national_eligibility_blocked",
        "as_of_date": "2026-07-28",
        "dataset": {
            "id": metadata["id"],
            "name": metadata["name"],
            "filter_contract_source": hpms_screen["record_id"],
            "filtered_section_rows": total_rows,
            "filtered_section_miles": round(float(total_miles), 4),
        },
        "source_custody": [
            {"path": str(path).replace("\\", "/"), "sha256": sha256(path)}
            for path in source_paths
        ],
        "official_guidance_sources": [
            {
                "title": "Technical Advisory: Shoulder and Edge Line Rumble Strips",
                "url": "https://highways.dot.gov/safety/rwd/keep-vehicles-road/rumble-strips/technical-advisory-shoulder-and-edge-line-rumble-strips",
                "use": "requires context-specific consideration of roadway condition, environment, and all users",
            },
            {
                "title": "Decision Support Guide for the Installation of Shoulder and Center Line Rumble Strips",
                "url": "https://highways.dot.gov/safety/rwd/keep-vehicles-road/rumble-strips/decision-support-guide-installation-shoulder-and-4",
                "use": "pavement, bicycle, noise, and modified-design decision boundary",
            },
            {
                "title": "Rumble Strips Frequently Asked Questions",
                "url": "https://highways.dot.gov/safety/rwd/keep-vehicles-road/rumble-strips/frequently-asked-questions",
                "use": "usable-width reference and warning against a single shoulder-width rule",
            },
            {
                "title": "Fact Sheet: Rumble Strips and Pavement",
                "url": "https://highways.dot.gov/safety/rwd/keep-vehicles-road/rumble-strips/fact-sheet-rumble-strips-and-pavement",
                "use": "pavement structure and condition boundary",
            },
            {
                "title": "Using LTPP Data to Understand Trends in Pavement Condition",
                "url": "https://www.fhwa.dot.gov/publications/research/infrastructure/pavements/ltpp/17090/001.cfm",
                "use": "IRI good, fair, and poor reporting thresholds",
            },
        ],
        "field_availability": {
            "left_shoulder_width": field_coverage("left_shoulder_rows"),
            "right_shoulder_width": field_coverage("right_shoulder_rows"),
            "iri": field_coverage("iri_rows"),
            "psr": field_coverage("psr_rows"),
            "cracking_percent": field_coverage("cracking_rows"),
            "surface_type": field_coverage("surface_rows"),
            "lane_width": field_coverage("lane_width_rows"),
        },
        "shoulder_width_triage": {
            "guidance_reference": "FHWA's FAQ cites four feet of usable shoulder width beyond the rumble strip as the AASHTO minimum for bicycle accommodation, with an additional foot by a barrier; FHWA also describes modified designs for constrained settings.",
            "measurement_boundary": "HPMS shoulder_width_l and shoulder_width_r are reported shoulder-width fields, not verified usable clear width beyond a proposed strip. The bands audit availability and identify records for local review; they do not classify design eligibility.",
            "reported_both_ge_4_feet": band(shoulder_source["both_ge_4"]),
            "reported_one_or_both_lt_4_feet": band(shoulder_source["one_or_both_lt_4"]),
            "missing_either_side": band(shoulder_source["missing"]),
        },
        "iri_triage": {
            "guidance_reference": "FHWA condition reporting labels IRI at or below 170 inches per mile good or fair and values above 170 poor.",
            "measurement_boundary": "IRI is a ride-quality proxy, not structural adequacy. Rumble-strip review still requires cracking, pavement type and depth, joints, remaining service life, and road-owner engineering judgment.",
            "reported_good_or_fair_proxy_iri_le_170": band(iri_source["iri_le_170"]),
            "reported_poor_proxy_iri_gt_170": band(iri_source["iri_gt_170"]),
            "missing": band(iri_source["missing"]),
        },
        "functional_system_availability": [
            {
                "f_system": int(row["f_system"]),
                "functional_system": SYSTEMS[int(row["f_system"])],
                "section_rows": count(row, "rows"),
                "section_miles": round(float(decimal(row, "miles")), 4),
                "both_shoulders_reported_miles": round(float(decimal(row, "both_shoulder_reported_miles")), 4),
                "iri_reported_miles": round(float(decimal(row, "iri_reported_miles")), 4),
                "cracking_reported_miles": round(float(decimal(row, "cracking_reported_miles")), 4),
                "surface_type_reported_miles": round(float(decimal(row, "surface_reported_miles")), 4),
            }
            for row in sorted(functional_rows, key=lambda value: int(value["f_system"]))
        ],
        "national_existing_treatment_inventory": {
            "installed_untreated_miles": None,
            "hpms_rumble_named_fields": rumble_fields,
            "status": "not_present_in_hpms_schema_and_not_identified_in_this_review",
            "boundary": "No rumble-named field appears in this HPMS dataset metadata, and this review did not identify a national installed-mile inventory. That is not proof that no inventory exists elsewhere; road-owner and program records remain required.",
        },
        "required_local_reduction": [
            "road-owner inventory of existing and already-programmed treatments",
            "section-level pavement structure, cracking, remaining service life, and resurfacing schedule",
            "usable shoulder and lane geometry plus bicycle and motorcycle accommodation",
            "residential noise, environmental, intersection, driveway, and other design constraints",
            "target-crash exposure and corridor-level safety diagnosis",
        ],
        "blocked_values": {
            "design_eligible_miles": None,
            "untreated_design_eligible_miles": None,
            "target_crash_fatalities_and_injuries": None,
            "candidate_cost_billions": None,
            "candidate_outcome": None,
        },
        "claim_boundaries": {
            "data_availability_audited": True,
            "national_design_eligibility_estimated": False,
            "missing_values_treated_as_ineligible": False,
            "reported_four_foot_shoulders_treated_as_eligible": False,
            "iri_treated_as_structural_adequacy": False,
            "existing_treatments_reconciled": False,
            "candidate_cost_ready": False,
            "allocation_ready": False,
            "funding_recommended": False,
            "public_release_authorized": False,
        },
    }
    args.output.write_text(json.dumps(output, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({
        "output": str(args.output),
        "filtered_miles": float(total_miles),
        "shoulder_missing_percent": output["shoulder_width_triage"]["missing_either_side"]["share_of_filtered_section_miles_percent"],
        "iri_missing_percent": output["iri_triage"]["missing"]["share_of_filtered_section_miles_percent"],
        "eligibility_estimated": False,
    }))


if __name__ == "__main__":
    main()
