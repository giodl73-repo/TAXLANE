# Transportation Pilot Baseline Path Contract Schema

Canonical artifact:
`transportation_pilot_baseline_path_contract.v1.draft.json`.

Required top-level fields:

- `record_id = transportation-pilot-baseline-path-contract:v1`
- `record_family = transportation_pilot_baseline_path_contract`
- `version`
- `status`
- `pulse = 91`
- `selected_pilot_decision_path`
- `source_plan_path`
- `transportation_depth_card_path`
- `deterministic_annual_update_simulator_contract_path`
- `program_lane_target_cost_contract_path`
- `phase_plan_path`
- `source_custody_status`
- `selected_pilot`
- `non_claim_boundary`
- `baseline_horizon`
- `fy2025_anchor`
- `required_annual_fields`
- `accounting_identities`
- `fund_treatment`
- `baseline_rows`
- `blocked_gates`
- `blocking_conditions`
- `output_placeholders`
- `claim_booleans`

Validation rules:

- The selected pilot must remain transportation asset maintenance and safety
  under `transportation-infrastructure`.
- The horizon must cover FY2025-FY2035, include the baseline year, require 11
  annual rows, require zero reform deltas, and remain incomplete.
- The FY2025 anchor must match the existing transportation depth card:
  total outlays of 145,320 million and component sum of 145,320 million.
- Required annual fields remain present and initially `null`.
- Accounting identities must include primary outlays, net cash requirement,
  fund balance change, zero-reform-delta treatment, and explicit rounding-line
  treatment.
- Transportation trust fund and general fund remain separate, with explicit
  interfund transfers and credited offsetting collections.
- `baseline_rows` remains empty until source custody is captured.
- Every blocked gate remains `false`.
- Every output placeholder remains `null`.
- Only `baseline_contract_published` may be true; completed baseline,
  source-custody, simulator, target-cost, rate, public-card, savings, waste,
  fraud, department-cut, technology-savings, and balanced-budget claims remain
  false.
