# Pilot Lane Selection Gate Schema

Canonical artifact:
`pilot_lane_selection_gate.v1.draft.json`.

Required top-level fields:

- `record_id = pilot-lane-selection-gate:v1`
- `record_family = pilot_lane_selection_gate`
- `version`
- `status`
- `pulse = 86`
- `adaptive_rate_system_contract_path`
- `public_rate_card_v2_contract_path`
- `role_review_path`
- `phase_plan_path`
- `source_custody_status`
- `non_claim_boundary`
- `selection_criteria`
- `recommended_initial_candidates`
- `excluded_first_pilots`
- `role_review_gate`
- `final_selection`
- `output_placeholders`
- `claim_booleans`

Validation rules:

- Source custody must show no new external request.
- Selection criteria must include narrow scope, low normative-distribution risk,
  official-source feasibility, floor observability, technology-transition fit,
  solver containment, and public-language safety.
- Recommended candidates may be listed but must stay `candidate_not_selected`.
- Excluded first pilots must include Social Security, Medicare, broad health,
  veterans, and immediate normative distribution choices.
- Role review must be required and incomplete.
- Final selection fields remain `null` and simulator/public-claim flags remain
  `false`.
- Every output placeholder remains `null`.
- Every claim boolean remains `false`.
