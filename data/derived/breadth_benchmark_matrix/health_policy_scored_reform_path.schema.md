# Health Policy-Scored Reform Path Schema

Schema for `health_policy_scored_reform_path.v1.draft.json`.

The record gates health `central_reform` and `stress` scenarios before any
federal cash-flow, solver, target-cost, or balanced-rate use.

Required top-level fields:

- `record_id`, `record_family`, `schema_version`, `as_of_date`, and `lane_id`.
- `contract_path`, `rubric_path`, and `evidence_boundary_paths`.
- `purpose`, `non_claim_boundary`, and `stress_definition`.
- `required_policy_score_inputs`: one row for each required policy-score input.
- `scenarios`: exactly `central_reform` and `stress`.
- `claim_booleans`, `explicit_blockers`, and `readiness`.

Each scenario must keep federal cash-flow fields null until a policy-specific
score exists:

- `gross_program_outlay_delta_musd`
- `implementation_admin_outlay_delta_musd`
- `credited_offset_delta_musd`
- `receipt_delta_musd`
- `net_interest_delta_musd`
- `net_federal_budget_effect_musd`

`central_reform` requires a specific federal policy instrument, segmentation,
phase-in, behavior, transition/administration cost, incidence, provenance, and
passed outcome floors before any federal effect can be scored.

`stress` must be the same selected policy under adverse realization. It is not
the aggressive private-insurance Medicare-relative payment sensitivity.
