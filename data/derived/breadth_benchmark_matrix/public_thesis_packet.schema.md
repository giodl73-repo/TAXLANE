# Public Thesis Packet Schema

Canonical artifact:
`public_thesis_packet.v1.draft.json`.

Required top-level fields:

- `record_id = public-thesis-packet:v1`
- `record_family = public_thesis_packet`
- `version`
- `status`
- `pulse = 88`
- `adaptive_rate_system_contract_path`
- `overspending_risk_taxonomy_path`
- `technology_transition_operating_model_path`
- `public_rate_card_v2_contract_path`
- `pilot_lane_selection_gate_path`
- `deterministic_annual_update_simulator_contract_path`
- `role_review_path`
- `phase_plan_path`
- `source_custody_status`
- `public_thesis`
- `non_claim_boundary`
- `public_language_rules`
- `public_packet_sections`
- `role_review_summary`
- `blocking_conditions`
- `output_placeholders`
- `claim_booleans`

Validation rules:

- Source custody must show no new external request.
- The non-claim boundary must block statutory rates, effective rates, public
  rate cards, tax proposals, savings, waste, fraud, department cuts,
  technology-savings claims, solver results, pilot selection, and
  balanced-budget claims.
- Public language must say `overspending risk`, not unsupported waste.
- Public language must say technology is a transition path, not an automatic
  cut.
- Blocked rates and not-calculated outputs must be described as valid public
  outcomes.
- Fairness must require burden and distribution analysis, not arithmetic balance
  alone.
- The role review summary must cover all eight roles.
- Every output placeholder remains `null`.
- Every claim boolean remains `false`.
