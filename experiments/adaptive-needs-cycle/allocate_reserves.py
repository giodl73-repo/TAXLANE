"""Apply the adaptive reserve gates without inventing missing scores."""

from __future__ import annotations

import argparse
import json
from pathlib import Path


SCORE_FIELDS = [
    "pain_severity_score", "affected_population_score", "urgency_and_reversibility_score",
    "marginal_outcome_gain_score", "equity_score", "whole_system_net_value_score",
    "intervention_evidence_multiplier", "delivery_confidence_multiplier", "candidate_cap_billions",
]


def score(row: dict, weights: dict) -> float:
    value = sum(row[f"{name}_score"] * weight for name, weight in weights.items())
    return value * row["intervention_evidence_multiplier"] * row["delivery_confidence_multiplier"]


def allocate(total: float, candidates: list[dict], max_share: float) -> list[dict]:
    allocations={row["track"]:0.0 for row in candidates}
    caps={row["track"]:min(row["candidate_cap_billions"], total*max_share) for row in candidates}
    active={row["track"]:row["priority_score"] for row in candidates if row["priority_score"] > 0}
    remaining=total
    while active and remaining > 1e-9:
        weight_total=sum(active.values())
        if weight_total <= 0: break
        capped=[]
        for track,weight in active.items():
            proposed=remaining*weight/weight_total
            room=caps[track]-allocations[track]
            if proposed >= room-1e-9:
                allocations[track]+=max(room,0)
                remaining-=max(room,0)
                capped.append(track)
        if not capped:
            for track,weight in active.items():
                allocations[track]+=remaining*weight/weight_total
            remaining=0
        else:
            for track in capped: active.pop(track)
    return [{"track":track,"allocation_billions":round(value,3)} for track,value in allocations.items() if value > 0]


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--contract", type=Path, required=True)
    parser.add_argument("--triage", type=Path, required=True)
    parser.add_argument("--profiles", type=Path, required=True)
    parser.add_argument("--dossiers", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    contract=json.loads(args.contract.read_text())
    triage=json.loads(args.triage.read_text())
    profiles=json.loads(args.profiles.read_text())
    dossiers=json.loads(args.dossiers.read_text())
    dossier_by_track={row["track"]:row for row in dossiers["dossiers"]}
    eligible=set(contract["eligible_reserve_tracks"])
    audits=[]
    for row in triage["lane_rows"]:
        if row["track"] not in eligible:
            audits.append({"track":row["track"],"lane_id":row["lane_id"],"reserve_role":"separate_rail","allocation_ready":False,"missing_inputs":[],"priority_score":None})
            continue
        dossier=dossier_by_track.get(row["track"])
        source_inputs=dossier["scoring_inputs"] if dossier else row
        missing=[field for field in SCORE_FIELDS if source_inputs.get(field) is None]
        if dossier is None: missing.append("pain_aligned_intervention_dossier")
        if not (dossier or row).get("allocation_admission_gates_passed",False): missing.append("allocation_admission_gates_passed")
        audits.append({"track":row["track"],"lane_id":row["lane_id"],"reserve_role":"eligible_service_lane","dossier_id":dossier["dossier_id"] if dossier else None,"allocation_ready":not missing,"missing_inputs":missing,"priority_score":None})
    ready=[row for row in audits if row["allocation_ready"]]
    source={row["track"]:({**row,**(dossier_by_track[row["track"]]["scoring_inputs"] if row["track"] in dossier_by_track else {})}) for row in triage["lane_rows"]}
    for audit in ready:
        audit["priority_score"]=round(score(source[audit["track"]],contract["primary_score"]["weighted_dimensions"]),6)
    results=[]
    for profile in profiles["profiles"]:
        reserve=profile["reinvestment_goal_billions"]
        allocatable=reserve*contract["allocation_rules"]["allocatable_share_percent"]/100
        candidates=[{**source[row["track"]],"priority_score":row["priority_score"]} for row in ready]
        lane_allocations=allocate(allocatable,candidates,contract["allocation_rules"]["maximum_single_lane_share_of_allocatable_percent"]/100) if candidates else []
        allocated=round(sum(row["allocation_billions"] for row in lane_allocations),3)
        results.append({
            "profile_id":profile["profile_id"],"label":profile["label"],"reserve_billions":reserve,
            "contingency_billions":round(reserve*contract["allocation_rules"]["contingency_share_percent"]/100,3),
            "provisionally_allocated_billions":allocated,"unallocated_pending_evidence_billions":round(reserve-allocated,3),
            "lane_allocations":lane_allocations,"status":"provisional_allocation_available" if lane_allocations else "no_lane_clears_allocation_admission_gates"
        })
    output={
        "record_id":"adaptive-profile-reserve-allocation-readiness:v1","record_family":"adaptive_profile_reserve_allocation_readiness","version":"v1.draft",
        "status":"four_profiles_audited_zero_provisional_allocation","as_of_date":"2026-07-28",
        "contract_path":str(args.contract).replace('\\','/'),"triage_path":str(args.triage).replace('\\','/'),"profiles_path":str(args.profiles).replace('\\','/'),"dossiers_path":str(args.dossiers).replace('\\','/'),
        "lane_admission_audit":audits,"profile_results":results,
        "rollup":{"eligible_service_lanes":len(eligible),"allocation_ready_lanes":len(ready),"profiles_audited":len(results),"total_provisionally_allocated_billions":round(sum(row["provisionally_allocated_billions"] for row in results),3)},
        "claim_boundaries":{"provisional_allocation_ready":bool(ready),"funding_recommended":False,"savings_admitted":False,"official_score":False,"public_release_authorized":False}
    }
    args.output.write_text(json.dumps(output,indent=2)+"\n")
    print(json.dumps({"output":str(args.output),"profiles":len(results),"ready_lanes":len(ready)}))


if __name__ == "__main__":
    main()
