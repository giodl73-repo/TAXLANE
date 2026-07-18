# Deterministic Annual Update Simulator Contract Schema

Canonical artifact:
`deterministic_annual_update_simulator_contract.v1.draft.json`.

Required top-level fields:

- `record_id = deterministic-annual-update-simulator-contract:v1`
- `record_family = deterministic_annual_update_simulator_contract`
- `version`
- `status`
- `pulse = 87`
- `pilot_lane_selection_gate_path`
- `adaptive_rate_system_contract_path`
- `technology_transition_operating_model_path`
- `public_rate_card_v2_contract_path`
- `phase_plan_path`
- `source_custody_status`
- `non_claim_boundary`
- `simulator_scope`
- `required_paths`
- `deterministic_equations`
- `fund_treatment`
- `floor_gate`
- `blocking_conditions`
- `output_placeholders`
- `claim_booleans`

Validation rules:

- Source custody must show no new external request.
- The simulator must cover ten years plus the baseline year.
- Optimization must be false.
- Baseline, modernization, and stress paths must exist as required paths but
  remain null until a pilot is selected.
- Net interest must be endogenous and recomputed after primary-balance changes.
- OASDI, Medicare HI, transportation trust, general fund, and reserves must be
  separate.
- Lower-rate recognition remains blocked unless floors pass.
- Every output placeholder remains `null`.
- Every claim boolean remains `false`.
