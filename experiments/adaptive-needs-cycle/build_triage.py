"""Build the first evidence-bounded all-lane needs triage.

The output classifies the problem and next intervention work. It deliberately
does not manufacture a cross-lane need score or allocate profile reserves.
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path


TRACKS = {
    "health-medicare": ("HLT", "cost_pressure_with_access_and_provider_floors", "cost_down_reform_plus_targeted_capacity_only", "structural_reform_and_targeted_temporary_repair"),
    "social-security": ("OAS", "solvency_and_retirement_adequacy_pressure", "dedicated_solvency_and_adequacy_package", "structural_dedicated_overlay"),
    "national-defense": ("DEF", "strategy_efficiency_and_personnel_safety", "strategy_led_reallocation_with_readiness_floor", "structural_strategy_change_plus_targeted_repair"),
    "income-security-family": ("ISF", "child_poverty_hardship_and_family_support_gap", "targeted_household_support_and_access_interventions", "temporary_pain_response_or_structural_support_after_evaluation"),
    "education-workforce": ("EDU", "targeted_completion_access_and_workforce_gaps_without_topline_expansion_case", "targeted_or_pilot_interventions_not_blanket_expansion", "pilot_or_time_limited_targeted_response"),
    "veterans": ("VET", "claims_backlog_and_service_timeliness", "backlog_repair_and_service_delivery_intervention", "temporary_pain_response_with_milestones"),
    "disaster-resilience": ("DIS", "hazard_exposure_life_safety_and_recovery_pressure", "exposure_normalized_mitigation_and_contingent_reserve", "stabilization_reserve_plus_multiyear_mitigation"),
    "justice-courts-public-safety": ("JUS", "victimization_rights_access_and_case_timeliness", "targeted_safety_rights_and_process_interventions", "targeted_temporary_or_structural_after_component_review"),
    "science-energy-environment": ("SEE", "distinct_science_energy_and_environment_capacity_risks", "separate_component_interventions_no_composite_award", "pilot_or_structural_by_component"),
    "agriculture": ("AGR", "farm_resilience_support_design_and_risk_distribution", "targeted_risk_research_or_support_reform", "cyclical_reserve_or_structural_by_instrument"),
    "international-affairs": ("INT", "mission_specific_diplomacy_humanitarian_security_and_development_needs", "component_and_region_specific_interventions", "temporary_crisis_response_or_structural_mission_base"),
    "transportation-infrastructure": ("TRN", "road_safety_asset_condition_and_trust_fund_pressure", "safety_maintenance_delivery_and_financing_interventions", "temporary_repair_plus_structural_maintenance"),
    "revenue-solvency": ("REV", "persistent_financing_and_borrowing_pressure", "financing_constraint_and_distributionally_scored_revenue_design", "structural_financing_constraint"),
    "payment-integrity": ("PAY", "control_quality_error_and_service_risk", "owner_attributed_controls_with_due_process", "non_additive_control_overlay"),
    "net-interest": ("NET", "debt_service_and_maturity_pressure", "recompute_from_primary_balance_debt_and_rate_paths", "endogenous_result_not_direct_allocation"),
}

PERIOD_STATUS = {
    "health-medicare": "standard_reporting_lag_acceptable_for_diagnostic",
    "social-security": "current_annual_measure",
    "national-defense": "current_annual_measure",
    "income-security-family": "current_annual_measure",
    "education-workforce": "refresh_required_2021_22_anchor",
    "veterans": "current_point_in_time_measure",
    "disaster-resilience": "current_but_requires_multiyear_exposure_normalization",
    "justice-courts-public-safety": "current_but_instrument_lineage_review_required",
    "science-energy-environment": "standard_reporting_lag_but_incomplete_composite_lane",
    "agriculture": "current_annual_measure",
    "international-affairs": "current_annual_measure_but_reporting_proxy_only",
    "transportation-infrastructure": "current_annual_measure",
    "revenue-solvency": "current_fiscal_measure",
    "payment-integrity": "measurement_window_lag_and_component_scope_review_required",
    "net-interest": "current_monthly_measure_but_fiscal_path_incomplete",
}


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--floor-readiness", type=Path, required=True)
    parser.add_argument("--breadth", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()

    floors = json.loads(args.floor_readiness.read_text())
    breadth_rows = [json.loads(line) for line in args.breadth.read_text().splitlines() if line.strip()]
    breadth_by_lane: dict[str, list[dict]] = {}
    for row in breadth_rows:
        breadth_by_lane.setdefault(row["lane_id"], []).append(row)

    lane_rows = []
    for floor in floors["lane_rows"]:
        lane_id = floor["lane_id"]
        track, diagnosis, response, funding_type = TRACKS[lane_id]
        packet_path = Path(floor["baseline_context_evidence_paths"][-1])
        packet = json.loads(packet_path.read_text())
        baseline_values = packet["baseline_values"]
        primary = baseline_values["primary_baseline"]
        period_key = next(
            (key for key in ("reporting_period", "reference_year", "reference_school_year", "observed_date", "measurement_window") if key in baseline_values),
            None,
        )
        benchmarks = []
        for row in breadth_by_lane.get(lane_id, []):
            benchmarks.append(
                {
                    "metric": row["metric_label"],
                    "current_value": row["current_value"],
                    "unit": row["current_unit"],
                    "gap_direction": row["gap_direction"],
                    "comparability_grade": row["comparability_grade"],
                    "interpretation": row["efficiency_gap_status"],
                }
            )
        lane_rows.append(
            {
                "track": track,
                "lane_id": lane_id,
                "public_label": floor["public_label"],
                "anchor": {
                    "rationale": floor["threshold_rationale"],
                    "measure": primary["measure"],
                    "unit": primary["unit"],
                    "observation_period": baseline_values.get(period_key) if period_key else None,
                    "period_alignment_status": PERIOD_STATUS[lane_id],
                    "affected_population": {key: value for key, value in primary.items() if "population" in key or "poverty" in key},
                    "threshold_value": floor["threshold_value"],
                    "baseline_value": floor["baseline_value"],
                    "source_packet_path": str(packet_path).replace("\\", "/"),
                    "status": "one_source_custodied_anchor_not_complete_lane_need_measure",
                },
                "benchmark_context": benchmarks,
                "provisional_diagnosis": diagnosis,
                "candidate_response_class": response,
                "candidate_funding_type": funding_type,
                "floor_risk_status": "not_scored",
                "pain_severity_score": None,
                "affected_population_score": None,
                "urgency_and_reversibility_score": None,
                "marginal_outcome_gain_score": None,
                "equity_score": None,
                "delivery_capacity_score": None,
                "whole_system_net_value_score": None,
                "intervention_evidence_multiplier": None,
                "delivery_confidence_multiplier": None,
                "candidate_cap_billions": None,
                "allocation_admission_gates_passed": False,
                "overall_need_score": None,
                "reserve_allocation_billions": None,
                "next_action": "define a current common-year pain threshold and a specific intervention; estimate marginal outcome, delivery, equity, duration, and whole-system cost before cross-lane ranking",
            }
        )

    output = {
        "record_id": "adaptive-needs-all-lane-triage:v1",
        "record_family": "adaptive_needs_all_lane_triage",
        "version": "v1.draft",
        "status": "fifteen_lane_problem_and_response_classes_populated_scores_and_allocations_blocked",
        "as_of_date": "2026-07-28",
        "framework_path": "data/derived/breadth_benchmark_matrix/adaptive_needs_and_balance_cycle.v1.draft.json",
        "floor_readiness_path": str(args.floor_readiness).replace("\\", "/"),
        "breadth_matrix_path": str(args.breadth).replace("\\", "/"),
        "method": "Classify each lane from source-custodied floor anchors and benchmark context. Do not normalize incomparable indicators, infer causality, rank needs, or allocate dollars until common-year measures and intervention evidence exist.",
        "lane_rows": lane_rows,
        "rollup": {
            "lanes_present": len(lane_rows),
            "problem_classes_populated": sum(bool(row["provisional_diagnosis"]) for row in lane_rows),
            "response_classes_populated": sum(bool(row["candidate_response_class"]) for row in lane_rows),
            "overall_need_scores_ready": 0,
            "reserve_allocations_ready": 0,
            "education_automatic_increase": False,
        },
        "next_pass": [
            "select one current pain measure per lane on a common observation window",
            "set reviewed absolute or distributional severity thresholds",
            "attach at least one specific intervention with causal or bounded evidence",
            "estimate marginal outcome, affected population, urgency, equity, delivery capacity, duration, and whole-system cost",
            "normalize only comparable dimensions and publish sensitivity rather than one opaque score",
            "allocate each profile reserve provisionally with sunsets and an unallocated contingency",
            "recalculate the common-perimeter spending mix and repeat after observed results",
        ],
        "claim_boundaries": {
            "pain_measured_completely": False,
            "cross_lane_rank_ready": False,
            "funding_recommendation_ready": False,
            "reserve_allocated": False,
            "savings_admitted": False,
            "official_score": False,
            "public_release_authorized": False,
        },
    }
    if len(lane_rows) != 15 or set(TRACKS) != {row["lane_id"] for row in lane_rows}:
        raise ValueError("triage must contain every lane exactly once")
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(output, indent=2) + "\n")
    print(json.dumps({"output": str(args.output), "lanes": len(lane_rows), "allocations": 0}))


if __name__ == "__main__":
    main()
