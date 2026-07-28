# Income Security Family Child Poverty Floor Value Packet Schema

Machine record:
`data/derived/breadth_benchmark_matrix/income_security_family_child_poverty_floor_value_packet.v1.draft.json`

Required fields:

- `record_id`, `record_family`, `version`, `pulse`, `as_of_date`
- `lane_id`: must be `income-security-family`
- `floor_id`: must be `child_poverty`
- `floor_definition_packet_path`
- `census_child_poverty_income_capture_gap_path`
- `outcome_floor_wave_d_value_readiness_path`
- `threshold_rationale`
- `baseline_values`
- `policy_values`, `stress_values`, `pass_fail_evidence`
- `readiness_status`
- `blocked_outputs`
- `claim_booleans`
- `public_warning`

The packet may publish a draft threshold rationale, threshold value, and sourced
baseline value. It must keep benefit-package modeling, take-up, childcare,
nutrition handoff, policy values, stress values, pass/fail evidence, lower-cost
scenario admissibility, target costs, solver inputs, public rates, savings,
technology-savings claims, and balanced-budget claims blocked until reviewed
scenario evidence exists.
