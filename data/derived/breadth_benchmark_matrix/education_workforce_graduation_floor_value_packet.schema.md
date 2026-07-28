# Education Workforce Graduation Floor Value Packet Schema

Machine record:
`data/derived/breadth_benchmark_matrix/education_workforce_graduation_floor_value_packet.v1.draft.json`

Required fields:

- `record_id`, `record_family`, `version`, `pulse`, `as_of_date`
- `lane_id`: must be `education-workforce`
- `floor_id`: must be `completion_persistence`
- `floor_definition_packet_path`
- `k12_outcome_baseline_path`
- `outcome_floor_wave_d_value_readiness_path`
- `threshold_rationale`
- `baseline_values`
- `policy_values`, `stress_values`, `pass_fail_evidence`
- `readiness_status`
- `blocked_outputs`
- `claim_booleans`
- `public_warning`

The packet may publish a draft threshold rationale, threshold value, and sourced
baseline value. It must keep policy values, stress values, pass/fail evidence,
lower-cost scenario admissibility, target costs, solver inputs, public rates,
savings, technology-savings claims, and balanced-budget claims blocked until
reviewed scenario evidence exists.
