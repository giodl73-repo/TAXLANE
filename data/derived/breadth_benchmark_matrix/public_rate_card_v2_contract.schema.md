# Public Rate-Card V2 Contract Schema

Canonical artifact:
`public_rate_card_v2_contract.v1.draft.json`.

Required top-level fields:

- `record_id = public-rate-card-v2-contract:v1`
- `record_family = public_rate_card_v2_contract`
- `version`
- `status`
- `pulse = 85`
- `adaptive_rate_system_contract_path`
- `overspending_risk_taxonomy_path`
- `technology_transition_operating_model_path`
- `phase_plan_path`
- `source_custody_status`
- `non_claim_boundary`
- `card_status_values`
- `required_card_fields`
- `publication_gates`
- `statutory_language_rule`
- `display_rules`
- `output_placeholders`
- `claim_booleans`

Validation rules:

- Source custody must show no new external request.
- Status values must include `not_calculated` and `blocked` as displayable
  public outcomes.
- Required card fields must include current cost, target cost, assigned base,
  rate, burden, distribution, floors, technology status, risk signals, evidence
  grade, blockers, and public-claim status.
- All initial field values remain `null`.
- Publication gates remain `false`.
- Statutory-rate language remains blocked unless publication gates pass.
- Every output placeholder remains `null`.
- Every claim boolean remains `false`.
